const Assets = @This();
const std = @import("std");
const Io = std.Io;
const Allocator = std.mem.Allocator;

pub const DataTables = @import("DataTables.zig");
pub const AssetIndexes = @import("AssetIndexes.zig");
pub const FsmGraphRegistry = @import("FsmGraphRegistry.zig");
pub const LazyNodeStore = @import("LazyNodeStore.zig");
pub const LazyFlowStore = @import("LazyFlowStore.zig");
pub const RewardIndex = @import("RewardIndex.zig");

tables: DataTables,
indexes: AssetIndexes,
fsm_graphs: FsmGraphRegistry,
quest_nodes: LazyNodeStore,
level_play_nodes: LazyNodeStore,
flow_states: LazyFlowStore,
reward_index: RewardIndex,

pub fn init(gpa: Allocator, io: Io) !Assets {
    var tables = try DataTables.load(gpa, io);
    errdefer tables.deinit();
    var indexes = try AssetIndexes.init(gpa, &tables);
    errdefer indexes.deinit(gpa);
    var fsm_graphs = try FsmGraphRegistry.init(gpa, &tables);
    errdefer fsm_graphs.deinit();
    var quest_nodes = try LazyNodeStore.init(gpa, io, "assets/BinData/QuestNodeData.json");
    errdefer quest_nodes.deinit();
    var level_play_nodes = try LazyNodeStore.init(gpa, io, "assets/BinData/LevelPlayNodeData.json");
    errdefer level_play_nodes.deinit();
    var flow_states = try LazyFlowStore.init(gpa, io, "assets/BinData/FlowState.json");
    errdefer flow_states.deinit();
    var reward_index = try RewardIndex.init(gpa);
    errdefer reward_index.deinit();
    return .{
        .fsm_graphs = fsm_graphs,
        .tables = tables,
        .indexes = indexes,
        .quest_nodes = quest_nodes,
        .level_play_nodes = level_play_nodes,
        .flow_states = flow_states,
        .reward_index = reward_index,
    };
}

/// Resolves a scene/instance ID or raw map ID to its underlying `MapConfigId`.
pub fn resolveMapConfigId(assets: *const Assets, scene_or_map_id: i32) i32 {
    if (assets.tables.instance_dungeon.getDataById(scene_or_map_id)) |inst| {
        return inst.MapConfigId;
    }
    return scene_or_map_id;
}

/// Resolves a `MapConfigId` to the preferred overworld/open-map `InstanceDungeon.Id`.
pub fn resolveInstanceIdForMap(assets: *const Assets, map_config_id: i32) i32 {
    if (assets.tables.instance_dungeon.getDataById(map_config_id)) |inst| {
        if (inst.MapConfigId == map_config_id) return inst.Id;
    }
    for (assets.tables.instance_dungeon.items) |inst| {
        if (inst.MapConfigId == map_config_id and inst.InstType == 1) {
            return inst.Id;
        }
    }
    return map_config_id;
}

pub fn findLevelEntity(assets: *const Assets, map_id: i32, entity_id: i64) ?*const DataTables.LevelEntityConfig {
    if (assets.indexes.findLevelEntity(&assets.tables, map_id, entity_id)) |cfg| {
        return cfg;
    }
    const resolved_map_id = assets.resolveMapConfigId(map_id);
    if (resolved_map_id != map_id) {
        if (assets.indexes.findLevelEntity(&assets.tables, resolved_map_id, entity_id)) |cfg| {
            return cfg;
        }
    }
    return null;
}

/// Explicit cross-map entity lookup for non-scene-scoped callers; never use for scene/teleport resolution.
pub fn findLevelEntityAnyMap(assets: *const Assets, entity_id: i64) ?*const DataTables.LevelEntityConfig {
    const indices = assets.indexes.levelEntityIndicesById(entity_id);
    if (indices.len > 0) {
        return &assets.tables.level_entity_config.items[indices[0]];
    }
    return null;
}

pub fn findBlueprintConfig(assets: *const Assets, blueprint_type: []const u8) ?*const DataTables.BlueprintConfig {
    return assets.indexes.findBlueprintConfig(&assets.tables, blueprint_type);
}

pub fn findTemplateConfig(assets: *const Assets, blueprint_type: []const u8) ?*const DataTables.TemplateConfig {
    return assets.indexes.findTemplateConfig(&assets.tables, blueprint_type);
}

pub fn questRequiresResource(assets: *const Assets, quest_id: i32) bool {
    if (quest_id <= 0) return false;
    if (assets.tables.quest_resource_white.getDataById(quest_id) != null) return false;
    if (assets.tables.quest_ref_map_block_config.getDataById(quest_id) != null) return true;
    for (assets.tables.quest_ref_video_config.items) |row| {
        if (row.QuestId == quest_id) return true;
    }
    return false;
}

pub fn getMotorcycleConfigId(assets: *const Assets) ?i32 {
    for (assets.tables.explore_tools.items) |tool| {
        if (tool.SummonConfigId == 0) continue;
        const summon = assets.tables.summon_cfg.getDataById(tool.SummonConfigId) orelse continue;
        const blueprint = assets.findBlueprintConfig(summon.BlueprintType) orelse continue;
        if (blueprint.EntityLogic != .Vehicle) continue;
        if (assets.findTemplateConfig(summon.BlueprintType)) |template| {
            return template.Id;
        }
    }
    return null;
}

pub fn deinit(assets: *Assets) void {
    assets.reward_index.deinit();
    assets.flow_states.deinit();
    assets.level_play_nodes.deinit();
    assets.quest_nodes.deinit();
    assets.fsm_graphs.deinit();
    assets.tables.deinit();
}
