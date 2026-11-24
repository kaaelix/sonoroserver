const AssetIndexes = @This();
const std = @import("std");
const Allocator = std.mem.Allocator;
const DataTables = @import("DataTables.zig");
const AssetsDb = @import("AssetsDb.zig");

pub const MapEntityKey = struct {
    map_id: i32,
    entity_id: i64,
};

pub const SpatialCellKey = struct {
    map_id: i32,
    cell_x: i32,
    cell_y: i32,
};

pub const Span = struct {
    start: u32,
    len: u32,
};

pub const CellRange = struct {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
};

pub const spatial_cell_size: f32 = 5000.0;
pub const max_spatial_query_cells: usize = 169;

pub const Inputs = struct {
    level_entity_configs: []const DataTables.LevelEntityConfig = &.{},
    blueprint_configs: []const DataTables.BlueprintConfig = &.{},
    template_configs: []const DataTables.TemplateConfig = &.{},
    teleporters: []const DataTables.Teleporter = &.{},
    level_play_data: []const DataTables.LevelPlayData = &.{},
};

fn SpanTable(comptime K: type) type {
    return struct {
        map: std.AutoHashMapUnmanaged(K, Span) = .empty,
        values: []u32 = &.{},

        const Self = @This();

        fn deinit(self: *Self, gpa: Allocator) void {
            self.map.deinit(gpa);
            if (self.values.len > 0) gpa.free(self.values);
            self.* = .{};
        }

        fn get(self: *const Self, key: K) []const u32 {
            const span = self.map.get(key) orelse return &.{};
            return self.values[span.start .. span.start + span.len];
        }

        const Builder = struct {
            counts: std.AutoHashMapUnmanaged(K, u32) = .empty,
            total: usize = 0,

            fn deinit(self: *Builder, gpa: Allocator) void {
                self.counts.deinit(gpa);
            }

            fn countKey(self: *Builder, gpa: Allocator, key: K) !void {
                const gop = try self.counts.getOrPut(gpa, key);
                if (!gop.found_existing) {
                    gop.value_ptr.* = 0;
                }
                gop.value_ptr.* += 1;
                self.total += 1;
            }

            fn allocateResult(self: *Builder, gpa: Allocator) !Self {
                var result: Self = .{};
                errdefer result.deinit(gpa);

                if (self.total == 0) return result;
                if (self.total > std.math.maxInt(u32)) return error.IndexOverflow;

                result.values = try gpa.alloc(u32, self.total);
                try result.map.ensureTotalCapacity(gpa, self.counts.count());

                var offset: u32 = 0;
                var it = self.counts.iterator();
                while (it.next()) |entry| {
                    const n = entry.value_ptr.*;
                    result.map.putAssumeCapacity(entry.key_ptr.*, .{ .start = offset, .len = 0 });
                    offset += n;
                }
                return result;
            }
        };

        fn appendValue(self: *Self, key: K, val: u32) void {
            const span = self.map.getPtr(key).?;
            self.values[span.start + span.len] = val;
            span.len += 1;
        }
    };
}

entity_by_map_pair: std.AutoHashMapUnmanaged(MapEntityKey, u32) = .empty,
entity_by_id: SpanTable(i64) = .{},
spawnables_by_map_cell: SpanTable(SpatialCellKey) = .{},
spawnables_by_map: SpanTable(i32) = .{},
teleporters_by_map: SpanTable(i32) = .{},
level_plays_by_entity: SpanTable(i32) = .{},
level_plays_by_ref_entity: SpanTable(i32) = .{},
level_plays_by_reward_entity: SpanTable(i32) = .{},
blueprint_by_type: std.StringHashMapUnmanaged(u32) = .empty,
template_by_type: std.StringHashMapUnmanaged(u32) = .empty,

pub fn init(gpa: Allocator, tables: *const DataTables) !AssetIndexes {
    return initFromSlices(gpa, .{
        .level_entity_configs = tables.level_entity_config.items,
        .blueprint_configs = tables.blueprint_config.items,
        .template_configs = tables.template_config.items,
        .teleporters = tables.teleporter.items,
        .level_play_data = tables.level_play_data.items,
    });
}

pub fn initWithDb(gpa: Allocator, db: *AssetsDb, tables: *const DataTables) !AssetIndexes {
    var result: AssetIndexes = .{};
    errdefer result.deinit(gpa);

    try buildStringIndex(DataTables.BlueprintConfig, gpa, tables.blueprint_config.items, &result.blueprint_by_type);
    try buildLevelEntityIndexesFromDb(gpa, db, &result);
    try buildTeleporterIndex(gpa, tables.teleporter.items, &result);
    try buildLevelPlayIndexes(gpa, tables.level_play_data.items, &result);

    return result;
}

fn buildLevelEntityIndexesFromDb(gpa: Allocator, db: *AssetsDb, out: *AssetIndexes) !void {
    try db.mutex.lock(db.io);
    defer db.mutex.unlock(db.io);

    var stmt = try db.db.prepare("SELECT idx, entity_id, map_id, cell_x, cell_y, spawnable FROM level_entities ORDER BY idx");
    defer stmt.deinit();

    const Row = struct {
        idx: u32,
        entity_id: i64,
        map_id: i32,
        cell_x: i32,
        cell_y: i32,
        spawnable: bool,
    };
    var rows: std.ArrayList(Row) = .empty;
    defer rows.deinit(gpa);
    try rows.ensureTotalCapacity(gpa, 220_000);

    while (try stmt.step()) {
        try rows.append(gpa, .{
            .idx = @intCast(stmt.colInt(0)),
            .entity_id = stmt.colInt(1),
            .map_id = @intCast(stmt.colInt(2)),
            .cell_x = @intCast(stmt.colInt(3)),
            .cell_y = @intCast(stmt.colInt(4)),
            .spawnable = stmt.colInt(5) != 0,
        });
    }

    const cap = std.math.cast(u32, rows.items.len) orelse return error.IndexOverflow;
    if (cap == 0) return;
    try out.entity_by_map_pair.ensureTotalCapacity(gpa, cap);

    var id_builder: SpanTable(i64).Builder = .{};
    defer id_builder.deinit(gpa);
    var cell_builder: SpanTable(SpatialCellKey).Builder = .{};
    defer cell_builder.deinit(gpa);
    var map_builder: SpanTable(i32).Builder = .{};
    defer map_builder.deinit(gpa);

    for (rows.items) |r| {
        const pair_gop = out.entity_by_map_pair.getOrPutAssumeCapacity(.{
            .map_id = r.map_id,
            .entity_id = r.entity_id,
        });
        if (!pair_gop.found_existing) pair_gop.value_ptr.* = r.idx;

        try id_builder.countKey(gpa, r.entity_id);
        if (r.spawnable) {
            try cell_builder.countKey(gpa, .{ .map_id = r.map_id, .cell_x = r.cell_x, .cell_y = r.cell_y });
            try map_builder.countKey(gpa, r.map_id);
        }
    }

    out.entity_by_id = try id_builder.allocateResult(gpa);
    out.spawnables_by_map_cell = try cell_builder.allocateResult(gpa);
    out.spawnables_by_map = try map_builder.allocateResult(gpa);

    for (rows.items) |r| {
        out.entity_by_id.appendValue(r.entity_id, r.idx);
        if (r.spawnable) {
            out.spawnables_by_map_cell.appendValue(.{ .map_id = r.map_id, .cell_x = r.cell_x, .cell_y = r.cell_y }, r.idx);
            out.spawnables_by_map.appendValue(r.map_id, r.idx);
        }
    }
}

pub fn initFromSlices(gpa: Allocator, inputs: Inputs) !AssetIndexes {
    var result: AssetIndexes = .{};
    errdefer result.deinit(gpa);

    try buildStringIndex(DataTables.BlueprintConfig, gpa, inputs.blueprint_configs, &result.blueprint_by_type);
    try buildStringIndex(DataTables.TemplateConfig, gpa, inputs.template_configs, &result.template_by_type);
    try buildLevelEntityIndexes(gpa, inputs.level_entity_configs, inputs.blueprint_configs, inputs.template_configs, &result);
    try buildTeleporterIndex(gpa, inputs.teleporters, &result);
    try buildLevelPlayIndexes(gpa, inputs.level_play_data, &result);

    return result;
}

pub fn deinit(self: *AssetIndexes, gpa: Allocator) void {
    self.entity_by_map_pair.deinit(gpa);
    self.entity_by_id.deinit(gpa);
    self.spawnables_by_map_cell.deinit(gpa);
    self.spawnables_by_map.deinit(gpa);
    self.teleporters_by_map.deinit(gpa);
    self.level_plays_by_entity.deinit(gpa);
    self.level_plays_by_ref_entity.deinit(gpa);
    self.level_plays_by_reward_entity.deinit(gpa);
    self.blueprint_by_type.deinit(gpa);
    self.template_by_type.deinit(gpa);
    self.* = .{};
}

pub fn findLevelEntity(
    self: *const AssetIndexes,
    tables: *const DataTables,
    map_id: i32,
    entity_id: i64,
) ?*const DataTables.LevelEntityConfig {
    const idx = self.entity_by_map_pair.get(.{
        .map_id = map_id,
        .entity_id = entity_id,
    }) orelse return null;
    if (tables.level_entity_config.lazy_ctx) |ptr| {
        const db: *AssetsDb = @ptrCast(@alignCast(ptr));
        return db.getLevelEntityByIdx(idx);
    }
    return &tables.level_entity_config.items[idx];
}

pub fn levelEntityIndicesById(self: *const AssetIndexes, entity_id: i64) []const u32 {
    return self.entity_by_id.get(entity_id);
}

pub fn teleporterIndicesForMap(self: *const AssetIndexes, map_id: i32) []const u32 {
    return self.teleporters_by_map.get(map_id);
}

pub fn spawnableIndicesForMapCell(
    self: *const AssetIndexes,
    map_id: i32,
    cell_x: i32,
    cell_y: i32,
) []const u32 {
    return self.spawnables_by_map_cell.get(.{
        .map_id = map_id,
        .cell_x = cell_x,
        .cell_y = cell_y,
    });
}

pub fn spawnableIndicesForMap(self: *const AssetIndexes, map_id: i32) []const u32 {
    return self.spawnables_by_map.get(map_id);
}

pub fn findBlueprintConfig(
    self: *const AssetIndexes,
    tables: *const DataTables,
    blueprint_type: []const u8,
) ?*const DataTables.BlueprintConfig {
    const idx = self.blueprint_by_type.get(blueprint_type) orelse return null;
    return &tables.blueprint_config.items[idx];
}

pub fn findTemplateConfig(
    self: *const AssetIndexes,
    tables: *const DataTables,
    blueprint_type: []const u8,
) ?*const DataTables.TemplateConfig {
    if (tables.template_config.lazy_ctx) |ptr| {
        const db: *AssetsDb = @ptrCast(@alignCast(ptr));
        return db.findTemplateByBlueprint(blueprint_type);
    }
    const idx = self.template_by_type.get(blueprint_type) orelse return null;
    return &tables.template_config.items[idx];
}

pub fn levelPlayIndicesForEntity(self: *const AssetIndexes, entity_id: i32) []const u32 {
    return self.level_plays_by_entity.get(entity_id);
}

pub fn levelPlayIndicesForRefEntity(self: *const AssetIndexes, entity_id: i32) []const u32 {
    return self.level_plays_by_ref_entity.get(entity_id);
}

pub fn levelPlayIndicesForRewardEntity(self: *const AssetIndexes, entity_id: i32) []const u32 {
    return self.level_plays_by_reward_entity.get(entity_id);
}

pub fn entityWorldPos(cfg: *const DataTables.LevelEntityConfig) ?[3]f32 {
    if (cfg.Transform.len < 1) return null;
    const raw = cfg.Transform[0];
    // LevelEntityConfig Transform[0..1] are fixed-point x100 (cm * 100, deg * 100), not scaled by Transform[2].
    const x = raw.X / 100.0;
    const y = raw.Y / 100.0;
    const z = raw.Z / 100.0;
    if (!std.math.isFinite(x) or !std.math.isFinite(y) or !std.math.isFinite(z)) return null;
    return .{ x, y, z };
}

pub fn entityWorldRot(cfg: *const DataTables.LevelEntityConfig) [3]f32 {
    if (cfg.Transform.len < 2) return .{ 0, 0, 0 };
    const raw = cfg.Transform[1];
    const r = raw.X / 100.0;
    const p = raw.Y / 100.0;
    const y = raw.Z / 100.0;
    if (!std.math.isFinite(r) or !std.math.isFinite(p) or !std.math.isFinite(y)) return .{ 0, 0, 0 };
    return .{ r, p, y };
}

pub fn entityWorldScale(cfg: *const DataTables.LevelEntityConfig) [3]f32 {
    if (!cfg.IsScaleEnabled or cfg.Transform.len < 3) return .{ 1, 1, 1 };
    const raw = cfg.Transform[2];
    const sx = if (std.math.isFinite(raw.X) and raw.X > 0) raw.X / 100.0 else 1.0;
    const sy = if (std.math.isFinite(raw.Y) and raw.Y > 0) raw.Y / 100.0 else 1.0;
    const sz = if (std.math.isFinite(raw.Z) and raw.Z > 0) raw.Z / 100.0 else 1.0;
    return .{ sx, sy, sz };
}

pub fn isSpawnableBlueprint(bp: []const u8) bool {
    return std.mem.indexOf(u8, bp, "Monster") != null or
        std.mem.indexOf(u8, bp, "Animal") != null or
        std.ascii.indexOfIgnoreCase(bp, "Npc") != null or
        std.mem.indexOf(u8, bp, "Treasure") != null or
        std.mem.indexOf(u8, bp, "Teleport") != null or
        std.mem.indexOf(u8, bp, "AirWall") != null;
}

fn isSpawnableItemConfig(
    cfg: *const DataTables.LevelEntityConfig,
    entity_type: @FieldType(DataTables.BlueprintConfig, "EntityType"),
) bool {
    _ = cfg;
    return switch (entity_type) {
        .AdviseItem,
        .Audio,
        .AudioBox,
        .Chair,
        .Chair2,
        .CustomAoiEditor,
        .DaemonHack,
        .DollGrabShowcase,
        .DynamicSpawnMonster,
        .DynamicSpawnMonsterPoint,
        .EntityBatchRefresh,
        .EntityList,
        .EntityPackage,
        .ExhibitBase,
        .FollowDecorationChair,
        .Furniture,
        .FurnitureSlot,
        .GroupAi,
        .LevelPlay,
        .LevelPlay2,
        .LevelPlayReward,
        .Monitor,
        .PerformanceOptimizer,
        .Position,
        .RefreshGroup,
        .RenderSpecifiedRange,
        .RenderSpecifiedRange2,
        .SceneItemStateHint,
        .SceneText,
        .Slots,
        .Spline,
        .SplineFX,
        .SplineRange,
        .StreamingSourcePoint,
        .TimeScheduleManagerEntity,
        .VarManager,
        .WuWaGoEntity,
        => false,
        else => true,
    };
}

pub fn spatialCellRange(px: f32, py: f32, radius: f32) ?CellRange {
    if (!std.math.isFinite(radius) or radius < 0) return null;
    const pad = radius + 1.0;
    const min_x = cellCoord(px - pad) orelse return null;
    const max_x = cellCoord(px + pad) orelse return null;
    const min_y = cellCoord(py - pad) orelse return null;
    const max_y = cellCoord(py + pad) orelse return null;
    if (max_x < min_x or max_y < min_y) return null;

    const nx: i64 = @as(i64, max_x) - @as(i64, min_x) + 1;
    const ny: i64 = @as(i64, max_y) - @as(i64, min_y) + 1;
    if (nx > max_spatial_query_cells or ny > max_spatial_query_cells) return null;
    if (nx * ny > max_spatial_query_cells) return null;

    return .{
        .min_x = min_x,
        .max_x = max_x,
        .min_y = min_y,
        .max_y = max_y,
    };
}

pub fn cellCoord(coord: f32) ?i32 {
    if (!std.math.isFinite(coord)) return null;
    const scaled = @floor(coord / spatial_cell_size);
    if (scaled < -1_000_000.0 or scaled > 1_000_000.0) return null;
    return @intFromFloat(scaled);
}

fn isSpawnableConfig(
    cfg: *const DataTables.LevelEntityConfig,
    blueprint_configs: []const DataTables.BlueprintConfig,
    blueprint_by_type: *const std.StringHashMapUnmanaged(u32),
    template_configs: []const DataTables.TemplateConfig,
    template_by_type: *const std.StringHashMapUnmanaged(u32),
) bool {
    if (std.mem.startsWith(u8, cfg.BlueprintType, "Editor")) return false;
    if (blueprint_by_type.get(cfg.BlueprintType)) |bp_idx| {
        const bp = blueprint_configs[bp_idx];
        return switch (bp.EntityLogic) {
            .Monster, .Animal, .Npc, .Vision, .Vehicle, .SimpleCombat => true,
            .Custom => switch (bp.EntityType) {
                .ClientTrigger, .EnrichmentArea, .LocationSafety => true,
                else => false,
            },
            .Item => blk: {
                if (isSpawnableItemConfig(cfg, bp.EntityType)) break :blk true;
                if (template_by_type.get(cfg.BlueprintType)) |tpl_idx| {
                    const tpl = template_configs[tpl_idx];
                    if (bp.EntityType == .TreasureBox or
                        bp.EntityType == .HookTreasureBox or
                        bp.EntityType == .VisionTreasureBox or
                        bp.EntityType == .Destructible or
                        bp.EntityType == .DestructibleWithHook or
                        bp.EntityType == .DestructibleExploreInteractor)
                    {
                        break :blk tpl.ComponentsData.RewardComponent != null or tpl.ComponentsData.TreasureBoxComponent != null;
                    }
                    if (tpl.ComponentsData.RewardComponent != null and std.mem.indexOf(u8, cfg.BlueprintType, "Treasure") != null) {
                        break :blk true;
                    }
                }
                break :blk false;
            },
            else => false,
        };
    }
    return isSpawnableBlueprint(cfg.BlueprintType);
}

fn spawnableCellForEntity(
    cfg: *const DataTables.LevelEntityConfig,
    blueprint_configs: []const DataTables.BlueprintConfig,
    blueprint_by_type: *const std.StringHashMapUnmanaged(u32),
    template_configs: []const DataTables.TemplateConfig,
    template_by_type: *const std.StringHashMapUnmanaged(u32),
) ?SpatialCellKey {
    if (!isSpawnableConfig(cfg, blueprint_configs, blueprint_by_type, template_configs, template_by_type)) return null;
    const pos = entityWorldPos(cfg) orelse return null;
    const cell_x = cellCoord(pos[0]) orelse return null;
    const cell_y = cellCoord(pos[1]) orelse return null;
    return .{
        .map_id = cfg.MapId,
        .cell_x = cell_x,
        .cell_y = cell_y,
    };
}

fn buildStringIndex(
    comptime T: type,
    gpa: Allocator,
    items: []const T,
    map: *std.StringHashMapUnmanaged(u32),
) !void {
    const cap = std.math.cast(u32, items.len) orelse return error.IndexOverflow;
    if (cap == 0) return;
    try map.ensureTotalCapacity(gpa, cap);
    for (items, 0..) |entry, idx| {
        if (entry.BlueprintType.len == 0) continue;
        const gop = map.getOrPutAssumeCapacity(entry.BlueprintType);
        if (!gop.found_existing) {
            gop.value_ptr.* = @intCast(idx);
        }
    }
}

fn buildLevelEntityIndexes(
    gpa: Allocator,
    items: []const DataTables.LevelEntityConfig,
    blueprint_configs: []const DataTables.BlueprintConfig,
    template_configs: []const DataTables.TemplateConfig,
    out: *AssetIndexes,
) !void {
    const cap = std.math.cast(u32, items.len) orelse return error.IndexOverflow;
    if (cap == 0) return;

    try out.entity_by_map_pair.ensureTotalCapacity(gpa, cap);

    var id_builder: SpanTable(i64).Builder = .{};
    defer id_builder.deinit(gpa);
    var cell_builder: SpanTable(SpatialCellKey).Builder = .{};
    defer cell_builder.deinit(gpa);
    var map_builder: SpanTable(i32).Builder = .{};
    defer map_builder.deinit(gpa);

    for (items, 0..) |*cfg, idx| {
        const pair_gop = out.entity_by_map_pair.getOrPutAssumeCapacity(.{
            .map_id = cfg.MapId,
            .entity_id = cfg.EntityId,
        });
        if (!pair_gop.found_existing) {
            pair_gop.value_ptr.* = @intCast(idx);
        }

        try id_builder.countKey(gpa, cfg.EntityId);
        if (spawnableCellForEntity(cfg, blueprint_configs, &out.blueprint_by_type, template_configs, &out.template_by_type)) |cell_key| {
            try cell_builder.countKey(gpa, cell_key);
            try map_builder.countKey(gpa, cfg.MapId);
        }
    }

    out.entity_by_id = try id_builder.allocateResult(gpa);
    out.spawnables_by_map_cell = try cell_builder.allocateResult(gpa);
    out.spawnables_by_map = try map_builder.allocateResult(gpa);

    for (items, 0..) |*cfg, idx| {
        const idx_u32: u32 = @intCast(idx);
        out.entity_by_id.appendValue(cfg.EntityId, idx_u32);
        if (spawnableCellForEntity(cfg, blueprint_configs, &out.blueprint_by_type, template_configs, &out.template_by_type)) |cell_key| {
            out.spawnables_by_map_cell.appendValue(cell_key, idx_u32);
            out.spawnables_by_map.appendValue(cfg.MapId, idx_u32);
        }
    }
}

fn buildTeleporterIndex(
    gpa: Allocator,
    items: []const DataTables.Teleporter,
    out: *AssetIndexes,
) !void {
    if (items.len > std.math.maxInt(u32)) return error.IndexOverflow;
    if (items.len == 0) return;

    var builder: SpanTable(i32).Builder = .{};
    defer builder.deinit(gpa);

    for (items) |tp| {
        try builder.countKey(gpa, tp.MapId);
    }
    out.teleporters_by_map = try builder.allocateResult(gpa);
    for (items, 0..) |tp, idx| {
        out.teleporters_by_map.appendValue(tp.MapId, @intCast(idx));
    }
}

pub fn parseRefSuffixId(ref: []const u8) ?i32 {
    const idx = std.mem.lastIndexOfScalar(u8, ref, '_') orelse return null;
    const digits = ref[idx + 1 ..];
    if (digits.len == 0 or digits[0] == '0') return null;
    for (digits) |ch| {
        if (ch < '0' or ch > '9') return null;
    }
    const parsed = std.fmt.parseInt(i32, digits, 10) catch return null;
    return if (parsed > 0) parsed else null;
}

fn appendUniqueRefId(refs: *std.ArrayListUnmanaged(i32), gpa: Allocator, id: i32) !void {
    if (id <= 0) return;
    for (refs.items) |existing| {
        if (existing == id) return;
    }
    try refs.append(gpa, id);
}

fn collectLevelPlayRefIds(
    lp: DataTables.LevelPlayData,
    gpa: Allocator,
    refs: *std.ArrayListUnmanaged(i32),
) !void {
    refs.clearRetainingCapacity();
    if (lp.Data.LevelPlayEntityId > 0) {
        try appendUniqueRefId(refs, gpa, lp.Data.LevelPlayEntityId);
    }
    for (lp.Data.Children) |ref| {
        if (parseRefSuffixId(ref)) |id| {
            try appendUniqueRefId(refs, gpa, id);
        }
    }
    for (lp.Data.Reference) |ref| {
        if (parseRefSuffixId(ref)) |id| {
            try appendUniqueRefId(refs, gpa, id);
        }
    }
}

fn buildLevelPlayIndexes(
    gpa: Allocator,
    items: []const DataTables.LevelPlayData,
    out: *AssetIndexes,
) !void {
    if (items.len > std.math.maxInt(u32)) return error.IndexOverflow;
    if (items.len == 0) return;

    var entity_builder: SpanTable(i32).Builder = .{};
    defer entity_builder.deinit(gpa);
    var ref_builder: SpanTable(i32).Builder = .{};
    defer ref_builder.deinit(gpa);
    var reward_builder: SpanTable(i32).Builder = .{};
    defer reward_builder.deinit(gpa);

    var row_refs: std.ArrayListUnmanaged(i32) = .empty;
    defer row_refs.deinit(gpa);

    for (items) |lp| {
        if (lp.Data.LevelPlayEntityId > 0) {
            try entity_builder.countKey(gpa, lp.Data.LevelPlayEntityId);
        }
        try collectLevelPlayRefIds(lp, gpa, &row_refs);
        for (row_refs.items) |ref_id| {
            try ref_builder.countKey(gpa, ref_id);
        }
        if (lp.Data.LevelPlayRewardConfig) |rc| {
            if (rc.RewardEntityId > 0) {
                try reward_builder.countKey(gpa, rc.RewardEntityId);
            }
        }
    }

    out.level_plays_by_entity = try entity_builder.allocateResult(gpa);
    out.level_plays_by_ref_entity = try ref_builder.allocateResult(gpa);
    out.level_plays_by_reward_entity = try reward_builder.allocateResult(gpa);

    for (items, 0..) |lp, idx| {
        const idx_u32: u32 = @intCast(idx);
        if (lp.Data.LevelPlayEntityId > 0) {
            out.level_plays_by_entity.appendValue(lp.Data.LevelPlayEntityId, idx_u32);
        }
        try collectLevelPlayRefIds(lp, gpa, &row_refs);
        for (row_refs.items) |ref_id| {
            out.level_plays_by_ref_entity.appendValue(ref_id, idx_u32);
        }
        if (lp.Data.LevelPlayRewardConfig) |rc| {
            if (rc.RewardEntityId > 0) {
                out.level_plays_by_reward_entity.appendValue(rc.RewardEntityId, idx_u32);
            }
        }
    }
}

const testing = std.testing;

fn parseTestLevelEntities(gpa: Allocator, json_bytes: []const u8) !std.json.Parsed([]DataTables.LevelEntityConfig) {
    return std.json.parseFromSlice([]DataTables.LevelEntityConfig, gpa, json_bytes, .{
        .ignore_unknown_fields = true,
        .allocate = .alloc_always,
    });
}

test "indexes level entities by map pair, id spans, and spatial grid" {
    const json_payload =
        \\[
        \\  {"Id": 1, "EntityId": 1001, "MapId": 8, "BlueprintType": "Monster_01", "Name": "m1", "InSleep": false, "IsHidden": false, "AreaId": 1, "IsScaleEnabled": false, "Transform": [{"X": 200.0, "Y": 400.0, "Z": 50.0}, {"X": 0.0, "Y": 0.0, "Z": 0.0}, {"X": 2.0, "Y": 2.0, "Z": 1.0}], "ComponentsData": {}},
        \\  {"Id": 2, "EntityId": 1001, "MapId": 9, "BlueprintType": "Npc_01", "Name": "n1", "InSleep": false, "IsHidden": false, "AreaId": 1, "IsScaleEnabled": false, "Transform": [{"X": 9000.0, "Y": 0.0, "Z": 0.0}, {"X": 0.0, "Y": 0.0, "Z": 0.0}, {"X": 1.0, "Y": 1.0, "Z": 1.0}], "ComponentsData": {}},
        \\  {"Id": 3, "EntityId": 1002, "MapId": 8, "BlueprintType": "Monster_ZeroScale", "Name": "m2", "InSleep": false, "IsHidden": false, "AreaId": 1, "IsScaleEnabled": false, "Transform": [{"X": 0.0, "Y": 0.0, "Z": 0.0}, {"X": 0.0, "Y": 0.0, "Z": 0.0}, {"X": 0.0, "Y": 1.0, "Z": 1.0}], "ComponentsData": {}},
        \\  {"Id": 4, "EntityId": 1003, "MapId": 8, "BlueprintType": "Item_Chest", "Name": "c1", "InSleep": false, "IsHidden": false, "AreaId": 1, "IsScaleEnabled": false, "Transform": [{"X": 100.0, "Y": 200.0, "Z": 0.0}, {"X": 0.0, "Y": 0.0, "Z": 0.0}, {"X": 1.0, "Y": 1.0, "Z": 1.0}], "ComponentsData": {}},
        \\  {"Id": 5, "EntityId": 1004, "MapId": 8, "BlueprintType": "Animal_Boundary", "Name": "a1", "InSleep": false, "IsHidden": false, "AreaId": 1, "IsScaleEnabled": false, "Transform": [{"X": -100.0, "Y": 4999.0, "Z": 0.0}, {"X": 0.0, "Y": 0.0, "Z": 0.0}, {"X": 1.0, "Y": 1.0, "Z": 1.0}], "ComponentsData": {}}
        \\]
    ;

    var parsed = try parseTestLevelEntities(testing.allocator, json_payload);
    defer parsed.deinit();

    const teleporters = [_]DataTables.Teleporter{
        .{ .Id = 1, .MapId = 8, .TeleportEntityConfigId = 1003 },
        .{ .Id = 2, .MapId = 9, .TeleportEntityConfigId = 1001 },
        .{ .Id = 3, .MapId = 8, .TeleportEntityConfigId = 1001 },
    };

    const children_a = [_][]const u8{ "e_8_1001", "e_8_1003", "e_8_1001" };
    const refs_a = [_][]const u8{ "e_8_1003", "prefix_0100", "bad_ref" };
    const level_plays = [_]DataTables.LevelPlayData{
        .{
            .LevelPlayId = 500,
            .Data = .{
                .LevelPlayEntityId = 1001,
                .Children = &children_a,
                .Reference = &refs_a,
                .LevelPlayRewardConfig = .{ .RewardEntityId = 777, .RewardId = 88 },
            },
        },
        .{
            .LevelPlayId = 501,
            .Data = .{
                .LevelPlayEntityId = 2002,
            },
        },
    };

    const blueprints = [_]DataTables.BlueprintConfig{
        .{ .Id = 1, .BlueprintType = "Monster_01", .EntityType = .Monster, .EntityLogic = .Monster, .ModelId = 1, .HalfHeight = 1, .TrackHeight = 1 },
    };
    const templates = [_]DataTables.TemplateConfig{
        .{ .Id = 10, .BlueprintType = "Monster_01", .ComponentsData = .{} },
    };

    var indexes = try AssetIndexes.initFromSlices(testing.allocator, .{
        .level_entity_configs = parsed.value,
        .blueprint_configs = &blueprints,
        .template_configs = &templates,
        .teleporters = &teleporters,
        .level_play_data = &level_plays,
    });
    defer indexes.deinit(testing.allocator);

    try testing.expectEqual(@as(?u32, 0), indexes.entity_by_map_pair.get(.{ .map_id = 8, .entity_id = 1001 }));
    try testing.expectEqual(@as(?u32, 1), indexes.entity_by_map_pair.get(.{ .map_id = 9, .entity_id = 1001 }));
    try testing.expectEqual(@as(?u32, null), indexes.entity_by_map_pair.get(.{ .map_id = 10, .entity_id = 1001 }));

    try testing.expectEqualSlices(u32, &.{ 0, 1 }, indexes.levelEntityIndicesById(1001));
    try testing.expectEqualSlices(u32, &.{2}, indexes.levelEntityIndicesById(1002));
    try testing.expectEqualSlices(u32, &.{}, indexes.levelEntityIndicesById(9999));

    const pos0 = entityWorldPos(&parsed.value[0]).?;
    try testing.expectApproxEqAbs(@as(f32, 2.0), pos0[0], 0.001);
    try testing.expectApproxEqAbs(@as(f32, 4.0), pos0[1], 0.001);
    try testing.expectApproxEqAbs(@as(f32, 0.5), pos0[2], 0.001);
    try testing.expect(entityWorldPos(&parsed.value[2]) != null);

    // Non-spawnable Item_Chest (idx 3) is excluded from spawnables.
    try testing.expectEqualSlices(u32, &.{ 0, 2, 4 }, indexes.spawnableIndicesForMap(8));
    try testing.expectEqualSlices(u32, &.{1}, indexes.spawnableIndicesForMap(9));
    try testing.expectEqualSlices(u32, &.{ 0, 2 }, indexes.spawnableIndicesForMapCell(8, 0, 0));
    try testing.expectEqualSlices(u32, &.{4}, indexes.spawnableIndicesForMapCell(8, -1, 0));

    const range = spatialCellRange(0.0, 5000.0, 200.0).?;
    var found_boundary = false;
    var cx = range.min_x;
    while (cx <= range.max_x) : (cx += 1) {
        var cy = range.min_y;
        while (cy <= range.max_y) : (cy += 1) {
            for (indexes.spawnableIndicesForMapCell(8, cx, cy)) |idx| {
                if (idx == 4) found_boundary = true;
            }
        }
    }
    try testing.expect(found_boundary);
    try testing.expect(spatialCellRange(std.math.nan(f32), 0.0, 1000.0) == null);
    try testing.expect(spatialCellRange(0.0, 0.0, 10_000_000.0) == null);

    try testing.expectEqualSlices(u32, &.{ 0, 2 }, indexes.teleporterIndicesForMap(8));
    try testing.expectEqualSlices(u32, &.{1}, indexes.teleporterIndicesForMap(9));

    try testing.expectEqualSlices(u32, &.{0}, indexes.levelPlayIndicesForEntity(1001));
    try testing.expectEqualSlices(u32, &.{0}, indexes.levelPlayIndicesForRefEntity(1001));
    try testing.expectEqualSlices(u32, &.{0}, indexes.levelPlayIndicesForRefEntity(1003));
    try testing.expectEqualSlices(u32, &.{}, indexes.levelPlayIndicesForRefEntity(100));
    try testing.expectEqualSlices(u32, &.{0}, indexes.levelPlayIndicesForRewardEntity(777));

    try testing.expectEqual(@as(?u32, 0), indexes.blueprint_by_type.get("Monster_01"));
    try testing.expectEqual(@as(?u32, 0), indexes.template_by_type.get("Monster_01"));
}

test "includes interactive item blueprints and excludes non-interactive item props even with Monster in name" {
    const json_payload =
        \\[
        \\  {"Id": 1, "EntityId": 136000479, "MapId": 8, "BlueprintType": "Animal016", "Name": "CollectButterfly", "InSleep": false, "IsHidden": false, "AreaId": 1, "IsScaleEnabled": false, "Transform": [{"X": 100.0, "Y": 100.0, "Z": 0.0}, {"X": 0.0, "Y": 0.0, "Z": 0.0}, {"X": 1.0, "Y": 1.0, "Z": 1.0}], "ComponentsData": {}},
        \\  {"Id": 2, "EntityId": 136000472, "MapId": 8, "BlueprintType": "Animal006", "Name": "RealAnimal", "InSleep": false, "IsHidden": false, "AreaId": 1, "IsScaleEnabled": false, "Transform": [{"X": 100.0, "Y": 100.0, "Z": 0.0}, {"X": 0.0, "Y": 0.0, "Z": 0.0}, {"X": 1.0, "Y": 1.0, "Z": 1.0}], "ComponentsData": {}},
        \\  {"Id": 3, "EntityId": 139000395, "MapId": 8, "BlueprintType": "MonsterChair", "Name": "DecoChair", "InSleep": false, "IsHidden": false, "AreaId": 1, "IsScaleEnabled": false, "Transform": [{"X": 100.0, "Y": 100.0, "Z": 0.0}, {"X": 0.0, "Y": 0.0, "Z": 0.0}, {"X": 1.0, "Y": 1.0, "Z": 1.0}], "ComponentsData": {}},
        \\  {"Id": 4, "EntityId": 139000396, "MapId": 8, "BlueprintType": "Treasure004", "Name": "SupplyChest", "InSleep": false, "IsHidden": false, "AreaId": 1, "IsScaleEnabled": false, "Transform": [{"X": 100.0, "Y": 100.0, "Z": 0.0}, {"X": 0.0, "Y": 0.0, "Z": 0.0}, {"X": 1.0, "Y": 1.0, "Z": 1.0}], "ComponentsData": {}}
        \\]
    ;

    var parsed = try parseTestLevelEntities(testing.allocator, json_payload);
    defer parsed.deinit();

    const blueprints = [_]DataTables.BlueprintConfig{
        .{ .Id = 35, .BlueprintType = "Animal016", .EntityType = .Collect, .EntityLogic = .Item, .ModelId = 630057, .HalfHeight = 0, .TrackHeight = 0 },
        .{ .Id = 36, .BlueprintType = "Animal006", .EntityType = .Animal, .EntityLogic = .Animal, .ModelId = 630006, .HalfHeight = 50, .TrackHeight = 0 },
        .{ .Id = 2659, .BlueprintType = "MonsterChair", .EntityType = .Chair, .EntityLogic = .Item, .ModelId = 709994, .HalfHeight = 0, .TrackHeight = 0 },
        .{ .Id = 4001, .BlueprintType = "Treasure004", .EntityType = .TreasureBox, .EntityLogic = .Item, .ModelId = 700001, .HalfHeight = 0, .TrackHeight = 0 },
    };

    var indexes = try AssetIndexes.initFromSlices(testing.allocator, .{
        .level_entity_configs = parsed.value,
        .blueprint_configs = &blueprints,
    });
    defer indexes.deinit(testing.allocator);

    try testing.expectEqualSlices(u32, &.{ 0, 1, 3 }, indexes.spawnableIndicesForMap(8));
    try testing.expectEqualSlices(u32, &.{ 0, 1, 3 }, indexes.spawnableIndicesForMapCell(8, 0, 0));
}

test "indexes reward and interactive non-reward destructibles" {
    const json_payload =
        \\[
        \\  {"Id": 1, "EntityId": 500001, "MapId": 8, "BlueprintType": "Destructible_Reward", "Name": "BreakableBox", "InSleep": false, "IsHidden": false, "AreaId": 1, "IsScaleEnabled": false, "Transform": [{"X": 100.0, "Y": 100.0, "Z": 0.0}, {"X": 0.0, "Y": 0.0, "Z": 0.0}, {"X": 1.0, "Y": 1.0, "Z": 1.0}], "ComponentsData": {}},
        \\  {"Id": 2, "EntityId": 500002, "MapId": 8, "BlueprintType": "Destructible_Plain", "Name": "PlainProp", "InSleep": false, "IsHidden": false, "AreaId": 1, "IsScaleEnabled": false, "Transform": [{"X": 100.0, "Y": 100.0, "Z": 0.0}, {"X": 0.0, "Y": 0.0, "Z": 0.0}, {"X": 1.0, "Y": 1.0, "Z": 1.0}], "ComponentsData": {}}
        \\]
    ;

    var parsed = try parseTestLevelEntities(testing.allocator, json_payload);
    defer parsed.deinit();

    const blueprints = [_]DataTables.BlueprintConfig{
        .{ .Id = 5001, .BlueprintType = "Destructible_Reward", .EntityType = .Destructible, .EntityLogic = .Item, .ModelId = 700010, .HalfHeight = 0, .TrackHeight = 0 },
        .{ .Id = 5002, .BlueprintType = "Destructible_Plain", .EntityType = .DestructibleExploreInteractor, .EntityLogic = .Item, .ModelId = 700011, .HalfHeight = 0, .TrackHeight = 0 },
    };
    const templates = [_]DataTables.TemplateConfig{
        .{ .Id = 5001, .BlueprintType = "Destructible_Reward", .ComponentsData = .{ .RewardComponent = .{} } },
        .{ .Id = 5002, .BlueprintType = "Destructible_Plain", .ComponentsData = .{} },
    };

    var indexes = try AssetIndexes.initFromSlices(testing.allocator, .{
        .level_entity_configs = parsed.value,
        .blueprint_configs = &blueprints,
        .template_configs = &templates,
    });
    defer indexes.deinit(testing.allocator);

    try testing.expectEqualSlices(u32, &.{ 0, 1 }, indexes.spawnableIndicesForMap(8));
}

test "entityWorldPos and entityWorldRot divide by fixed 100.0 even with non-100 scale" {
    var tr = [_]DataTables.LevelEntityConfig.Vector{
        .{ .X = -13502878, .Y = 8241805, .Z = 383788 },
        .{ .X = 0, .Y = 0, .Z = 9000 },
        .{ .X = 100, .Y = 47, .Z = 150 },
    };
    const cfg: DataTables.LevelEntityConfig = .{
        .Id = 1,
        .EntityId = 105000233,
        .BlueprintType = "Gameplay292",
        .InSleep = false,
        .IsHidden = false,
        .MapId = 8,
        .AreaId = 1,
        .Transform = &tr,
        .ComponentsData = .{},
    };
    const pos = entityWorldPos(&cfg).?;
    const rot = entityWorldRot(&cfg);
    try testing.expectApproxEqAbs(@as(f32, -135028.78), pos[0], 0.01);
    try testing.expectApproxEqAbs(@as(f32, 82418.05), pos[1], 0.01);
    try testing.expectApproxEqAbs(@as(f32, 3837.88), pos[2], 0.01);
    try testing.expectApproxEqAbs(@as(f32, 90.0), rot[2], 0.01);
}
