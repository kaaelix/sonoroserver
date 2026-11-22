const AssetsDb = @This();
const std = @import("std");
const common = @import("common");
const sqlite = common.sqlite;
const DataTables = @import("DataTables.zig");

const Io = std.Io;
const Allocator = std.mem.Allocator;
const ArenaAllocator = std.heap.ArenaAllocator;

const log = std.log.scoped(.assets_db);

pub fn RingCache(comptime K: type, comptime V: type, comptime bucket_cap: usize) type {
    const Map = if (K == []const u8)
        std.StringHashMapUnmanaged(*const V)
    else
        std.AutoHashMapUnmanaged(K, *const V);

    return struct {
        const Self = @This();

        const Bucket = struct {
            arena: ArenaAllocator,
            map: Map = .empty,

            fn init(gpa: Allocator) Bucket {
                return .{ .arena = ArenaAllocator.init(gpa) };
            }

            fn deinit(self: *Bucket, gpa: Allocator) void {
                self.map.deinit(gpa);
                self.arena.deinit();
            }

            fn reset(self: *Bucket, gpa: Allocator) void {
                self.map.clearRetainingCapacity();
                _ = self.arena.reset(.free_all);
                _ = gpa;
            }
        };

        gpa: Allocator,
        buckets: [4]Bucket,
        cur: usize = 0,

        pub fn init(gpa: Allocator) Self {
            return .{
                .gpa = gpa,
                .buckets = .{
                    Bucket.init(gpa),
                    Bucket.init(gpa),
                    Bucket.init(gpa),
                    Bucket.init(gpa),
                },
            };
        }

        pub fn deinit(self: *Self) void {
            for (&self.buckets) |*b| b.deinit(self.gpa);
        }

        pub fn getPtr(self: *const Self, key: K) ?*const V {
            var step: usize = 0;
            while (step < 4) : (step += 1) {
                const idx = (self.cur + 4 - step) % 4;
                if (self.buckets[idx].map.get(key)) |ptr| return ptr;
            }
            return null;
        }

        pub fn putJson(self: *Self, key: K, json_bytes: []const u8) !*const V {
            if (self.buckets[self.cur].map.count() >= bucket_cap) {
                self.cur = (self.cur + 1) % 4;
                self.buckets[self.cur].reset(self.gpa);
            }
            const bucket = &self.buckets[self.cur];
            const arena = bucket.arena.allocator();
            const parsed = try std.json.parseFromSliceLeaky(
                V,
                arena,
                json_bytes,
                .{ .ignore_unknown_fields = true, .allocate = .alloc_always },
            );
            const ptr = try arena.create(V);
            ptr.* = parsed;

            const stored_key: K = if (K == []const u8) try arena.dupe(u8, key) else key;
            try bucket.map.put(self.gpa, stored_key, ptr);
            return ptr;
        }

        pub fn putValue(self: *Self, key: K, val: V) !*const V {
            if (self.buckets[self.cur].map.count() >= bucket_cap) {
                self.cur = (self.cur + 1) % 4;
                self.buckets[self.cur].reset(self.gpa);
            }
            const bucket = &self.buckets[self.cur];
            const arena = bucket.arena.allocator();
            const ptr = try arena.create(V);
            ptr.* = val;

            const stored_key: K = if (K == []const u8) try arena.dupe(u8, key) else key;
            try bucket.map.put(self.gpa, stored_key, ptr);
            return ptr;
        }
    };
}

gpa: Allocator,
io: Io,
mutex: Io.Mutex = .init,
db: sqlite.Db,

stmt_kv_int: sqlite.Stmt,
stmt_kv_str: sqlite.Stmt,
stmt_buff: sqlite.Stmt,
stmt_autobuff: sqlite.Stmt,
stmt_le_idx: sqlite.Stmt,
stmt_le_area_exp: sqlite.Stmt,
stmt_le_exp_only: sqlite.Stmt,
stmt_le_bp: sqlite.Stmt,
stmt_le_sun: sqlite.Stmt,
stmt_tpl_id: sqlite.Stmt,
stmt_tpl_bp: sqlite.Stmt,
stmt_tpl_passenger: sqlite.Stmt,
stmt_esp_bp: sqlite.Stmt,
stmt_esp_pref: sqlite.Stmt,
stmt_node_tree: sqlite.Stmt,
stmt_node_has: sqlite.Stmt,
stmt_raw_file: sqlite.Stmt,

le_cache: RingCache(u32, DataTables.LevelEntityConfig, 512),
tpl_cache: RingCache(i32, DataTables.TemplateConfig, 256),
tpl_bp_cache: RingCache([]const u8, DataTables.TemplateConfig, 256),
buff_cache: RingCache(i64, DataTables.Buff, 512),
damage_cache: RingCache(i64, DataTables.Damage, 256),
base_prop_cache: RingCache(i32, DataTables.BaseProperty, 256),
trail_phantom_cache: RingCache(i32, DataTables.TrailPhantomProp, 128),
model_preload_cache: RingCache(i32, DataTables.ModelConfigPreload, 256),
cond_cache: RingCache(i32, DataTables.Condition, 256),
cond_grp_cache: RingCache(i32, DataTables.ConditionGroup, 256),
passive_cache: RingCache(i64, DataTables.PassiveSkill, 256),
drop_pkg_cache: RingCache(i32, DataTables.DropPackage, 256),
drop_grp_cache: RingCache(i32, DataTables.DropGroup, 128),
flow_cache: RingCache([]const u8, DataTables.Flow, 128),
ai_fsm_cache: RingCache([]const u8, DataTables.AiStateMachineConfig, 32),

pub const SpawnScanRow = struct {
    idx: u32,
    entity_id: i64,
    map_id: i32,
    cell_x: i32,
    cell_y: i32,
    spawnable: bool,
};

pub fn ensureBuilt(io: Io) void {
    var child = std.process.spawn(io, .{
        .argv = &.{ "python3", "scripts/build_assets_db.py", "--if-stale" },
    }) catch |err| {
        log.warn("auto-build assets.db spawn failed ({t}); using existing assets.db if present", .{err});
        return;
    };
    const term = child.wait(io) catch |err| {
        log.warn("auto-build assets.db wait failed ({t})", .{err});
        return;
    };
    switch (term) {
        .exited => |code| if (code != 0) {
            log.warn("build_assets_db.py exited with code {d}", .{code});
        },
        else => log.warn("build_assets_db.py terminated abnormally", .{}),
    }
}

pub fn init(gpa: Allocator, io: Io) !*AssetsDb {
    ensureBuilt(io);

    var db = try sqlite.Db.openReadOnly("assets/assets.db");
    errdefer db.close();

    const self = try gpa.create(AssetsDb);
    errdefer gpa.destroy(self);

    self.* = .{
        .gpa = gpa,
        .io = io,
        .db = db,
        .stmt_kv_int = try db.prepare("SELECT data FROM kv_int WHERE tbl=? AND key_id=?"),
        .stmt_kv_str = try db.prepare("SELECT data FROM kv_str WHERE tbl=? AND key_str=?"),
        .stmt_buff = try db.prepare("SELECT data FROM buffs WHERE id=?"),
        .stmt_autobuff = try db.prepare("SELECT data FROM buffs WHERE id_prefix=? AND is_infinite=1"),
        .stmt_le_idx = try db.prepare("SELECT data FROM level_entities WHERE idx=?"),
        .stmt_le_area_exp = try db.prepare("SELECT entity_id FROM level_entities WHERE area_id=? AND exploratory_degree=? LIMIT 32"),
        .stmt_le_exp_only = try db.prepare("SELECT entity_id FROM level_entities WHERE exploratory_degree=? LIMIT 32"),
        .stmt_le_bp = try db.prepare("SELECT entity_id FROM level_entities WHERE map_id=? AND blueprint_type=? LIMIT 32"),
        .stmt_le_sun = try db.prepare("SELECT entity_id, map_id, area_id FROM level_entities WHERE is_sun_spirit=1"),
        .stmt_tpl_id = try db.prepare("SELECT data FROM template_configs WHERE id=?"),
        .stmt_tpl_bp = try db.prepare("SELECT data FROM template_configs WHERE blueprint_type=? LIMIT 1"),
        .stmt_tpl_passenger = try db.prepare("SELECT data FROM template_configs WHERE is_passenger=1"),
        .stmt_esp_bp = try db.prepare("SELECT skill_id FROM entity_skill_preload WHERE actor_blueprint=?"),
        .stmt_esp_pref = try db.prepare("SELECT skill_id FROM entity_skill_preload WHERE skill_prefix=?"),
        .stmt_node_tree = try db.prepare("SELECT data FROM node_trees WHERE kind=? AND owner_id=? ORDER BY seq"),
        .stmt_node_has = try db.prepare("SELECT 1 FROM node_trees WHERE kind=? AND owner_id=? LIMIT 1"),
        .stmt_raw_file = try db.prepare("SELECT data FROM raw_files WHERE name=?"),
        .le_cache = RingCache(u32, DataTables.LevelEntityConfig, 512).init(gpa),
        .tpl_cache = RingCache(i32, DataTables.TemplateConfig, 256).init(gpa),
        .tpl_bp_cache = RingCache([]const u8, DataTables.TemplateConfig, 256).init(gpa),
        .buff_cache = RingCache(i64, DataTables.Buff, 512).init(gpa),
        .damage_cache = RingCache(i64, DataTables.Damage, 256).init(gpa),
        .base_prop_cache = RingCache(i32, DataTables.BaseProperty, 256).init(gpa),
        .trail_phantom_cache = RingCache(i32, DataTables.TrailPhantomProp, 128).init(gpa),
        .model_preload_cache = RingCache(i32, DataTables.ModelConfigPreload, 256).init(gpa),
        .cond_cache = RingCache(i32, DataTables.Condition, 256).init(gpa),
        .cond_grp_cache = RingCache(i32, DataTables.ConditionGroup, 256).init(gpa),
        .passive_cache = RingCache(i64, DataTables.PassiveSkill, 256).init(gpa),
        .drop_pkg_cache = RingCache(i32, DataTables.DropPackage, 256).init(gpa),
        .drop_grp_cache = RingCache(i32, DataTables.DropGroup, 128).init(gpa),
        .flow_cache = RingCache([]const u8, DataTables.Flow, 128).init(gpa),
        .ai_fsm_cache = RingCache([]const u8, DataTables.AiStateMachineConfig, 32).init(gpa),
    };
    log.info("AssetsDb initialized read-only at assets/assets.db", .{});
    return self;
}

pub fn deinit(self: *AssetsDb) void {
    self.le_cache.deinit();
    self.tpl_cache.deinit();
    self.tpl_bp_cache.deinit();
    self.buff_cache.deinit();
    self.damage_cache.deinit();
    self.base_prop_cache.deinit();
    self.trail_phantom_cache.deinit();
    self.model_preload_cache.deinit();
    self.cond_cache.deinit();
    self.cond_grp_cache.deinit();
    self.passive_cache.deinit();
    self.drop_pkg_cache.deinit();
    self.drop_grp_cache.deinit();
    self.flow_cache.deinit();
    self.ai_fsm_cache.deinit();

    self.stmt_kv_int.deinit();
    self.stmt_kv_str.deinit();
    self.stmt_buff.deinit();
    self.stmt_autobuff.deinit();
    self.stmt_le_idx.deinit();
    self.stmt_le_area_exp.deinit();
    self.stmt_le_exp_only.deinit();
    self.stmt_le_bp.deinit();
    self.stmt_le_sun.deinit();
    self.stmt_tpl_id.deinit();
    self.stmt_tpl_bp.deinit();
    self.stmt_tpl_passenger.deinit();
    self.stmt_esp_bp.deinit();
    self.stmt_esp_pref.deinit();
    self.stmt_node_tree.deinit();
    self.stmt_node_has.deinit();
    self.stmt_raw_file.deinit();

    self.db.close();
    self.gpa.destroy(self);
}

fn queryKvInt(self: *AssetsDb, comptime V: type, cache: anytype, tbl: []const u8, key_id: i64, typed_key: anytype) ?V {
    self.mutex.lock(self.io) catch return null;
    defer self.mutex.unlock(self.io);

    if (cache.getPtr(typed_key)) |ptr| return ptr.*;

    self.stmt_kv_int.reset();
    self.stmt_kv_int.bindText(1, tbl) catch return null;
    self.stmt_kv_int.bindInt(2, key_id) catch return null;
    if (!(self.stmt_kv_int.step() catch return null)) return null;
    const raw = self.stmt_kv_int.colText(0) orelse return null;
    const ptr = cache.putJson(typed_key, raw) catch return null;
    return ptr.*;
}

fn queryKvStr(self: *AssetsDb, comptime V: type, cache: anytype, tbl: []const u8, key_str: []const u8) ?V {
    self.mutex.lock(self.io) catch return null;
    defer self.mutex.unlock(self.io);

    if (cache.getPtr(key_str)) |ptr| return ptr.*;

    self.stmt_kv_str.reset();
    self.stmt_kv_str.bindText(1, tbl) catch return null;
    self.stmt_kv_str.bindText(2, key_str) catch return null;
    if (!(self.stmt_kv_str.step() catch return null)) return null;
    const raw = self.stmt_kv_str.colText(0) orelse return null;
    const ptr = cache.putJson(key_str, raw) catch return null;
    return ptr.*;
}

pub fn getBuff(self: *AssetsDb, id: i64) ?DataTables.Buff {
    self.mutex.lock(self.io) catch return null;
    defer self.mutex.unlock(self.io);

    if (self.buff_cache.getPtr(id)) |ptr| return ptr.*;
    self.stmt_buff.reset();
    self.stmt_buff.bindInt(1, id) catch return null;
    if (!(self.stmt_buff.step() catch return null)) return null;
    const raw = self.stmt_buff.colText(0) orelse return null;
    const ptr = self.buff_cache.putJson(id, raw) catch return null;
    return ptr.*;
}

pub fn getAutoBuffCandidates(self: *AssetsDb, role_id: i32, arena: Allocator) ![]const DataTables.Buff {
    try self.mutex.lock(self.io);
    defer self.mutex.unlock(self.io);

    self.stmt_autobuff.reset();
    try self.stmt_autobuff.bindInt(1, role_id);
    var list: std.ArrayList(DataTables.Buff) = .empty;
    while (try self.stmt_autobuff.step()) {
        const raw = self.stmt_autobuff.colText(0) orelse continue;
        const parsed = try std.json.parseFromSliceLeaky(
            DataTables.Buff,
            arena,
            raw,
            .{ .ignore_unknown_fields = true, .allocate = .alloc_always },
        );
        try list.append(arena, parsed);
    }
    return list.items;
}

pub fn getLevelEntityByIdx(self: *AssetsDb, idx: u32) ?*const DataTables.LevelEntityConfig {
    self.mutex.lock(self.io) catch return null;
    defer self.mutex.unlock(self.io);

    if (self.le_cache.getPtr(idx)) |ptr| return ptr;
    self.stmt_le_idx.reset();
    self.stmt_le_idx.bindInt(1, idx) catch return null;
    if (!(self.stmt_le_idx.step() catch return null)) return null;
    const raw = self.stmt_le_idx.colText(0) orelse return null;
    return self.le_cache.putJson(idx, raw) catch null;
}

pub fn getTemplateById(self: *AssetsDb, id: i32) ?DataTables.TemplateConfig {
    self.mutex.lock(self.io) catch return null;
    defer self.mutex.unlock(self.io);

    if (self.tpl_cache.getPtr(id)) |ptr| return ptr.*;
    self.stmt_tpl_id.reset();
    self.stmt_tpl_id.bindInt(1, id) catch return null;
    if (!(self.stmt_tpl_id.step() catch return null)) return null;
    const raw = self.stmt_tpl_id.colText(0) orelse return null;
    const ptr = self.tpl_cache.putJson(id, raw) catch return null;
    return ptr.*;
}

pub fn findTemplateByBlueprint(self: *AssetsDb, bp: []const u8) ?*const DataTables.TemplateConfig {
    self.mutex.lock(self.io) catch return null;
    defer self.mutex.unlock(self.io);

    if (self.tpl_bp_cache.getPtr(bp)) |ptr| return ptr;
    self.stmt_tpl_bp.reset();
    self.stmt_tpl_bp.bindText(1, bp) catch return null;
    if (!(self.stmt_tpl_bp.step() catch return null)) return null;
    const raw = self.stmt_tpl_bp.colText(0) orelse return null;
    return self.tpl_bp_cache.putJson(bp, raw) catch null;
}

pub fn queryPassengerTemplates(self: *AssetsDb, arena: Allocator) ![]const DataTables.TemplateConfig {
    try self.mutex.lock(self.io);
    defer self.mutex.unlock(self.io);

    self.stmt_tpl_passenger.reset();
    var list: std.ArrayList(DataTables.TemplateConfig) = .empty;
    while (try self.stmt_tpl_passenger.step()) {
        const raw = self.stmt_tpl_passenger.colText(0) orelse continue;
        const parsed = try std.json.parseFromSliceLeaky(
            DataTables.TemplateConfig,
            arena,
            raw,
            .{ .ignore_unknown_fields = true, .allocate = .alloc_always },
        );
        try list.append(arena, parsed);
    }
    return list.items;
}

pub fn querySkillIdsByBlueprint(self: *AssetsDb, bp: []const u8, arena: Allocator) ![]const i32 {
    try self.mutex.lock(self.io);
    defer self.mutex.unlock(self.io);

    self.stmt_esp_bp.reset();
    try self.stmt_esp_bp.bindText(1, bp);
    var list: std.ArrayList(i32) = .empty;
    while (try self.stmt_esp_bp.step()) {
        const sid: i32 = @intCast(self.stmt_esp_bp.colInt(0));
        if (sid > 0) try list.append(arena, sid);
    }
    return list.items;
}

pub fn querySkillIdsByRolePrefix(self: *AssetsDb, role_id: i32, arena: Allocator) ![]const i32 {
    try self.mutex.lock(self.io);
    defer self.mutex.unlock(self.io);

    self.stmt_esp_pref.reset();
    try self.stmt_esp_pref.bindInt(1, role_id);
    var list: std.ArrayList(i32) = .empty;
    while (try self.stmt_esp_pref.step()) {
        const sid: i32 = @intCast(self.stmt_esp_pref.colInt(0));
        if (sid > 0) try list.append(arena, sid);
    }
    return list.items;
}

pub fn queryEntityIdsByAreaExplore(self: *AssetsDb, area_id: i32, exp_degree: i32, arena: Allocator) ![]const i32 {
    try self.mutex.lock(self.io);
    defer self.mutex.unlock(self.io);

    var list: std.ArrayList(i32) = .empty;
    if (area_id != 0) {
        self.stmt_le_area_exp.reset();
        try self.stmt_le_area_exp.bindInt(1, area_id);
        try self.stmt_le_area_exp.bindInt(2, exp_degree);
        while (try self.stmt_le_area_exp.step()) {
            const eid = self.stmt_le_area_exp.colInt(0);
            if (eid > 0 and eid <= std.math.maxInt(i32)) try list.append(arena, @intCast(eid));
        }
    } else {
        self.stmt_le_exp_only.reset();
        try self.stmt_le_exp_only.bindInt(1, exp_degree);
        while (try self.stmt_le_exp_only.step()) {
            const eid = self.stmt_le_exp_only.colInt(0);
            if (eid > 0 and eid <= std.math.maxInt(i32)) try list.append(arena, @intCast(eid));
        }
    }
    return list.items;
}

pub fn queryEntityIdsByMapBlueprint(self: *AssetsDb, map_id: i32, bp: []const u8, arena: Allocator) ![]const i32 {
    try self.mutex.lock(self.io);
    defer self.mutex.unlock(self.io);

    self.stmt_le_bp.reset();
    try self.stmt_le_bp.bindInt(1, map_id);
    try self.stmt_le_bp.bindText(2, bp);
    var list: std.ArrayList(i32) = .empty;
    while (try self.stmt_le_bp.step()) {
        const eid = self.stmt_le_bp.colInt(0);
        if (eid > 0 and eid <= std.math.maxInt(i32)) try list.append(arena, @intCast(eid));
    }
    return list.items;
}

pub const SunSpiritRow = struct {
    entity_id: i64,
    map_id: i32,
    area_id: i32,
};

pub fn querySunSpiritEntities(self: *AssetsDb, arena: Allocator) ![]const SunSpiritRow {
    try self.mutex.lock(self.io);
    defer self.mutex.unlock(self.io);

    self.stmt_le_sun.reset();
    var list: std.ArrayList(SunSpiritRow) = .empty;
    while (try self.stmt_le_sun.step()) {
        try list.append(arena, .{
            .entity_id = self.stmt_le_sun.colInt(0),
            .map_id = @intCast(self.stmt_le_sun.colInt(1)),
            .area_id = @intCast(self.stmt_le_sun.colInt(2)),
        });
    }
    return list.items;
}

pub fn getDamage(self: *AssetsDb, id: i64) ?DataTables.Damage {
    return self.queryKvInt(DataTables.Damage, &self.damage_cache, "Damage", id, id);
}
pub fn getBaseProperty(self: *AssetsDb, id: i32) ?DataTables.BaseProperty {
    return self.queryKvInt(DataTables.BaseProperty, &self.base_prop_cache, "BaseProperty", id, id);
}
pub fn getTrailPhantomProp(self: *AssetsDb, id: i32) ?DataTables.TrailPhantomProp {
    return self.queryKvInt(DataTables.TrailPhantomProp, &self.trail_phantom_cache, "TrailPhantomProp", id, id);
}
pub fn getModelConfigPreload(self: *AssetsDb, id: i32) ?DataTables.ModelConfigPreload {
    return self.queryKvInt(DataTables.ModelConfigPreload, &self.model_preload_cache, "ModelConfigPreload", id, id);
}
pub fn getCondition(self: *AssetsDb, id: i32) ?DataTables.Condition {
    return self.queryKvInt(DataTables.Condition, &self.cond_cache, "Condition", id, id);
}
pub fn getConditionGroup(self: *AssetsDb, id: i32) ?DataTables.ConditionGroup {
    return self.queryKvInt(DataTables.ConditionGroup, &self.cond_grp_cache, "ConditionGroup", id, id);
}
pub fn getPassiveSkill(self: *AssetsDb, id: i64) ?DataTables.PassiveSkill {
    return self.queryKvInt(DataTables.PassiveSkill, &self.passive_cache, "PassiveSkill", id, id);
}
pub fn getDropPackage(self: *AssetsDb, id: i32) ?DataTables.DropPackage {
    return self.queryKvInt(DataTables.DropPackage, &self.drop_pkg_cache, "DropPackage", id, id);
}
pub fn getDropGroup(self: *AssetsDb, id: i32) ?DataTables.DropGroup {
    return self.queryKvInt(DataTables.DropGroup, &self.drop_grp_cache, "DropGroup", id, id);
}
pub fn getFlow(self: *AssetsDb, id: []const u8) ?DataTables.Flow {
    return self.queryKvStr(DataTables.Flow, &self.flow_cache, "Flow", id);
}
pub fn getAiStateMachineConfig(self: *AssetsDb, id: []const u8) ?DataTables.AiStateMachineConfig {
    return self.queryKvStr(DataTables.AiStateMachineConfig, &self.ai_fsm_cache, "AiStateMachineConfig", id);
}

pub fn readRawFileAlloc(self: *AssetsDb, arena: Allocator, name: []const u8) !?[]u8 {
    try self.mutex.lock(self.io);
    defer self.mutex.unlock(self.io);

    self.stmt_raw_file.reset();
    try self.stmt_raw_file.bindText(1, name);
    if (!(try self.stmt_raw_file.step())) return null;
    const txt = self.stmt_raw_file.colText(0) orelse return null;
    return try arena.dupe(u8, txt);
}

pub fn hasNodeTreeOwner(self: *AssetsDb, kind: i32, owner_id: i32) bool {
    self.mutex.lock(self.io) catch return false;
    defer self.mutex.unlock(self.io);

    self.stmt_node_has.reset();
    self.stmt_node_has.bindInt(1, kind) catch return false;
    self.stmt_node_has.bindInt(2, owner_id) catch return false;
    return self.stmt_node_has.step() catch false;
}

pub fn queryNodeTreeDataSlices(self: *AssetsDb, kind: i32, owner_id: i32, arena: Allocator) ![][]const u8 {
    try self.mutex.lock(self.io);
    defer self.mutex.unlock(self.io);

    self.stmt_node_tree.reset();
    try self.stmt_node_tree.bindInt(1, kind);
    try self.stmt_node_tree.bindInt(2, owner_id);
    var list: std.ArrayList([]const u8) = .empty;
    while (try self.stmt_node_tree.step()) {
        const raw = self.stmt_node_tree.colText(0) orelse continue;
        try list.append(arena, try arena.dupe(u8, raw));
    }
    return list.items;
}

pub fn queryFlowStateJson(self: *AssetsDb, state_key: []const u8, arena: Allocator) !?[]const u8 {
    try self.mutex.lock(self.io);
    defer self.mutex.unlock(self.io);

    self.stmt_kv_str.reset();
    try self.stmt_kv_str.bindText(1, "FlowState");
    try self.stmt_kv_str.bindText(2, state_key);
    if (!(try self.stmt_kv_str.step())) return null;
    const raw = self.stmt_kv_str.colText(0) orelse return null;
    return try arena.dupe(u8, raw);
}

// Type-erased adapters for DataTables.Table(T, key)
pub fn makeTableLookup(comptime T: type, comptime K: type, comptime method: []const u8) *const fn (*anyopaque, K) ?T {
    return struct {
        fn call(ptr: *anyopaque, key: K) ?T {
            const self: *AssetsDb = @ptrCast(@alignCast(ptr));
            return @field(AssetsDb, method)(self, key);
        }
    }.call;
}

test "RingCache rotates 4 buckets while preserving recent pointers" {
    const V = struct { a: i32 };
    var cache = RingCache(i32, V, 2).init(std.testing.allocator);
    defer cache.deinit();

    const p1 = try cache.putJson(1, "{\"a\":10}");
    const p2 = try cache.putJson(2, "{\"a\":20}");
    try std.testing.expectEqual(@as(i32, 10), p1.a);
    try std.testing.expectEqual(@as(i32, 20), p2.a);

    // Bucket 0 holds [1,2]. Putting 3..8 fills buckets 1..3; putting 9 wraps onto bucket 0.
    for (3..10) |i| {
        const key: i32 = @intCast(i);
        var buf: [32]u8 = undefined;
        const json_s = try std.fmt.bufPrint(&buf, "{{\"a\":{d}}}", .{i * 10});
        _ = try cache.putJson(key, json_s);
        if (i == 8) {
            try std.testing.expectEqual(@as(i32, 10), cache.getPtr(1).?.a);
        }
    }
    try std.testing.expect(cache.getPtr(1) == null);
    try std.testing.expectEqual(@as(i32, 90), cache.getPtr(9).?.a);
}

test "AssetsDb queries assets.db when built" {
    const io = std.testing.io;
    Io.Dir.cwd().access(io, "assets/assets.db", .{}) catch return;

    const db = try AssetsDb.init(std.testing.allocator, io);
    defer db.deinit();

    const first = db.getLevelEntityByIdx(0);
    try std.testing.expect(first != null);
    try std.testing.expect(db.getLevelEntityByIdx(0) == first);

    var arena = ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const suns = try db.querySunSpiritEntities(arena.allocator());
    try std.testing.expect(suns.len > 0);
}
