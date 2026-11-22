const Scene = @This();
const std = @import("std");
const pb = @import("proto").pb;
const Assets = @import("../data/Assets.zig");
const FileSystem = @import("common").FileSystem;
const SceneInstance = @import("../fs/SceneInstance.zig");
const FormationInfo = @import("../fs/FormationInfo.zig");
const ExploreToolsInfo = @import("../fs/ExploreToolsInfo.zig");
const file_util = @import("../fs/file_util.zig");
const comp_util = @import("component/comp_util.zig");
const EntityComponentStorage = @import("component/entity/EntityComponentStorage.zig");
const PlayerSceneComponent = @import("component/player/PlayerSceneComponent.zig");
const FsmWakeScheduler = @import("schedulers/FsmWakeScheduler.zig");
pub const QuestTimerQueue = @import("schedulers/QuestTimerQueue.zig");
const incr = @import("../fs/incr.zig");
const gameplay_tags = @import("helpers/gameplay_tags.zig");

const Allocator = std.mem.Allocator;

player_id: i32,
instance_id: i32,
/// Set only during the quest recheck triggered by a PassPortalRequest.
last_passed_portal_id: i32 = 0,
instance: SceneInstance,
formation_info: FormationInfo,
explore_tools_info: ExploreToolsInfo,
entities: std.MultiArrayList(EntityComponentStorage) = .empty,
net_id_map: std.array_hash_map.Auto(i64, usize) = .empty,
scene_time: TimeInfo = .{},
active_battle_entities: std.AutoHashMapUnmanaged(i64, void) = .empty,
rewarded_kills: std.AutoHashMapUnmanaged(i64, void) = .empty,
fsm_hate_states: std.AutoHashMapUnmanaged(i64, bool) = .empty,
combine_relations: std.AutoHashMapUnmanaged(i64, CombineRelation) = .empty,
pending_combine_detaches: std.ArrayListUnmanaged(CombineDetach) = .empty,
debug_spawn_routes: std.AutoHashMapUnmanaged(i64, DebugSpawnRoute) = .empty,
debug_spawn_route_owners: std.AutoHashMapUnmanaged(i64, i64) = .empty,
fsm_wakes: FsmWakeScheduler = .{},
player_in_battle: bool = false,
battle_state_notified: bool = false,
battle_state_dirty: bool = false,
last_spatial_check_loc: [3]f32 = .{ 0, 0, 0 },
last_quest_move_loc: [3]f32 = .{ 0, 0, 0 },
has_quest_move_loc: bool = false,
player_movement_mode: i32 = 1,
has_player_movement_mode: bool = false,
player_motion_type: pb.MotionType = .Spurt,
has_player_motion: bool = false,
current_quest_node: i32 = 19,
current_quest_id: i32 = 139000025,
prologue_flow_started: bool = false,
active_flow_inc_id: i64 = 0,
active_flow_quest_id: i32 = 0,
active_flow_node_id: i32 = 0,
active_flow_bt_type: pb.BtType = .BtTypeQuest,
active_flow_list_name: []u8 = &.{},
active_flow_id: i32 = 0,
active_flow_state_id: i32 = 0,
active_flow_owner_entity_id: i64 = 0,
flow_next_state_id: i32 = 0,
executed_flow_action_ids: std.ArrayListUnmanaged(i32) = .empty,
pending_transition_instance_id: ?i32 = null,
next_action_inc_id: i32 = 1,
client_black_screen_open: bool = false,
next_entity_id: i64 = 1,
plot_added_rover_slot: ?usize = null,
plot_saved_cur_role: i32 = 0,
plot_saved_slot_role_id: i32 = 0,
plot_saved_slot_entity_id: i64 = -1,
pending_flows: std.ArrayListUnmanaged(PendingFlow) = .empty,
quest_timers: QuestTimerQueue = .{},
lp_timers: QuestTimerQueue = .{},
fail_timers: QuestTimerQueue = .{},
lp_fail_timers: QuestTimerQueue = .{},
bt_timers: std.ArrayListUnmanaged(ActiveBtTimer) = .empty,
bt_occupations: std.ArrayListUnmanaged(BtOccupation) = .empty,
save_node_keys: std.ArrayListUnmanaged(u64) = .empty,
trial_role_ids: std.ArrayListUnmanaged(i32) = .empty,
pending_killed_removals: std.ArrayListUnmanaged(i64) = .empty,
dynamic_interact_options: std.ArrayListUnmanaged(DynamicInteractOption) = .empty,
play_bubbles: std.ArrayListUnmanaged(PlayBubbleRecord) = .empty,
player_range_states: std.AutoHashMapUnmanaged(i64, RangeTriggerState) = .empty,

pub const RangeTriggerState = struct {
    in_range: bool = false,
    enter_fired: bool = false,
};

pub const DynamicInteractOption = struct {
    option_guid: []u8,
    entity_net_id: i64,
    entity_config_id: i32,
    bt_type: pb.BtType = .BtTypeQuest,
    quest_id: i32 = 0,
    node_id: i32 = 0,
    kind: Kind = .none,
    flow_list_name: []u8 = &.{},
    flow_id: i32 = 0,
    state_id: i32 = 1,
    actions_json: []u8 = &.{},

    pub const Kind = enum { none, flow, actions };

    pub fn deinit(opt: *DynamicInteractOption, gpa: Allocator) void {
        if (opt.option_guid.len != 0) gpa.free(opt.option_guid);
        if (opt.flow_list_name.len != 0) gpa.free(opt.flow_list_name);
        if (opt.actions_json.len != 0) gpa.free(opt.actions_json);
    }
};

pub const PlayBubbleRecord = struct {
    entity_config_id: i32,
    action_guid: []u8,

    pub fn deinit(rec: *PlayBubbleRecord, gpa: Allocator) void {
        if (rec.action_guid.len != 0) gpa.free(rec.action_guid);
    }
};

pub const ActiveBtTimer = struct {
    owner_id: i32,
    inc_id: i64,
    node_id: i32,
    timer_type: []u8,
    end_time_ms: i64,
    pause_time_ms: i64 = 0,
    paused: bool = false,

    pub fn deinit(timer: *ActiveBtTimer, gpa: Allocator) void {
        if (timer.timer_type.len != 0) gpa.free(timer.timer_type);
    }

    pub fn toProto(timer: ActiveBtTimer) pb.TimerInfoPb {
        return .{
            .TimerType = timer.timer_type,
            .NodeId = timer.node_id,
            .EndTime = timer.end_time_ms,
            .PauseTime = timer.pause_time_ms,
        };
    }
};

pub const StateTagUpdate = struct {
    old_tag_id: i32,
    new_tag_id: i32,
};

pub const PendingFlow = struct {
    quest_id: i32,
    node_id: i32,
    flow_list_name: []u8,
    flow_id: i32,
    state_id: i32,

    pub fn deinit(flow: *PendingFlow, gpa: Allocator) void {
        gpa.free(flow.flow_list_name);
    }
};

pub const BtOccupation = struct {
    bt_type: pb.BtType = .BtTypeQuest,
    owner_id: i32 = 0,
    inc_id: i64 = 0,
    node_id: i32 = 0,
    resource_name: []u8,

    pub fn deinit(occ: *BtOccupation, gpa: Allocator) void {
        if (occ.resource_name.len != 0) gpa.free(occ.resource_name);
    }
};

pub const DebugSpawnRoute = struct {
    source_map_id: i32,
    repeat_boss: bool,
    owner_count: usize,

    pub fn matches(route: DebugSpawnRoute, source_map_id: i32, repeat_boss: bool) bool {
        return route.source_map_id == source_map_id and route.repeat_boss == repeat_boss;
    }
};

pub const CombineRelation = struct {
    target_entity_id: i64,
    part_index: i32,
    position: pb.Vector,
    rotation: pb.Rotator,

    pub fn matches(relation: CombineRelation, other: CombineRelation) bool {
        return relation.target_entity_id == other.target_entity_id and
            relation.part_index == other.part_index and
            std.meta.eql(relation.position, other.position) and
            std.meta.eql(relation.rotation, other.rotation);
    }
};

pub const CombineDetach = struct {
    combine_entity_id: i64,
    target_entity_id: i64,
};

pub const CombineRegisterResult = enum {
    added,
    existing,
};

pub const TimeInfo = @import("SceneTime.zig");

pub const Entity = struct {
    pub const ConfigComponent = @import("component/entity/ConfigComponent.zig");
    pub const PositionComponent = @import("component/entity/PositionComponent.zig");
    pub const GravityComponent = @import("component/entity/GravityComponent.zig");
    pub const LiftComponent = @import("component/entity/LiftComponent.zig");
    pub const MoveSplineComponent = @import("component/entity/MoveSplineComponent.zig");
    pub const MoveToPointComponent = @import("component/entity/MoveToPointComponent.zig");
    pub const SceneItemComponent = @import("component/entity/SceneItemComponent.zig");
    pub const InteractStateComponent = @import("component/entity/InteractStateComponent.zig");
    pub const MonsterCaptureComponent = @import("component/entity/MonsterCaptureComponent.zig");
    pub const PlayerIDComponent = @import("component/entity/PlayerIDComponent.zig");
    pub const VisibleMarker = @import("component/entity/VisibleMarker.zig");
    pub const ActorVisibleMarker = @import("component/entity/ActorVisibleMarker.zig");
    pub const DirectControlMarker = @import("component/entity/DirectControlMarker.zig");
    pub const AttributeComponent = @import("component/entity/AttributeComponent.zig");
    pub const TagComponent = @import("component/entity/TagComponent.zig");
    pub const PartComponent = @import("component/entity/PartComponent.zig");
    pub const FightBuffComponent = @import("component/entity/FightBuffComponent.zig");
    pub const CharacterAttachComponent = @import("component/entity/CharacterAttachComponent.zig");
    pub const ConcomitantComponent = @import("component/entity/ConcomitantComponent.zig");
    pub const EquipComponent = @import("component/entity/EquipComponent.zig");
    pub const FollowerComponent = @import("component/entity/FollowerComponent.zig");
    pub const VehicleComponent = @import("component/entity/VehicleComponent.zig");
    pub const NpcDriveVehicleComponent = @import("component/entity/NpcDriveVehicleComponent.zig");
    pub const MotorOutlookComponent = @import("component/entity/MotorOutlookComponent.zig");
    pub const MotorDaCtxComponent = @import("component/entity/MotorDaCtxComponent.zig");
    pub const LogicStateComponent = @import("component/entity/LogicStateComponent.zig");
    pub const MonsterAiComponent = @import("component/entity/MonsterAiComponent.zig");
    pub const PassiveGaSkillComponent = @import("component/entity/PassiveGaSkillComponent.zig");
    pub const PlayerBattleBinder = @import("component/entity/PlayerBattleBinder.zig");
    pub const SummonerComponent = @import("component/entity/SummonerComponent.zig");
    pub const FollowShooterComponent = @import("component/entity/FollowShooterComponent.zig");
    pub const FollowEntityComponent = @import("component/entity/FollowEntityComponent.zig");
    pub const FsmComponent = @import("component/entity/FsmComponent.zig");
    pub const VisionSkillComponent = @import("component/entity/VisionSkillComponent.zig");
    pub const BaseSkinComponent = @import("component/entity/BaseSkinComponent.zig");
    pub const WeaponSkinComponent = @import("component/entity/WeaponSkinComponent.zig");
    pub const OrnamentComponent = @import("component/entity/OrnamentComponent.zig");
    pub const CalabashSkinComponent = @import("component/entity/CalabashSkinComponent.zig");
    pub const StateTagComponent = @import("component/entity/StateTagComponent.zig");

    index: usize,
    net_id: i64,
};

pub fn Query(comptime types: []const type) type {
    return struct {
        const Q = @This();
        pub const scene_query_types = types;
        pub const Item = @Tuple(types);

        iterator: Iterator,

        pub fn byNetId(query: Q, net_id: i64) ?Item {
            inline for (types) |ty| {
                if (ty == Entity) break;
            } else @compileError("can't extract components by net_id when the identifier is not queried");

            if (query.iterator.net_id_map) |map| {
                const index = map.get(net_id) orelse return null;
                if (index >= query.iterator.entities.len) return null;
                const slice = query.iterator.entities.slice();
                if (slice.items(.entity_id)[index].net_id != net_id) return null;
                var it = query.iterator;
                it.index = index;
                return it.peekCurrent(slice);
            }

            var it = query.iterator;
            while (it.next()) |item| {
                inline for (types, 0..) |ty, i| {
                    if (ty == Entity and item[i].net_id == net_id) return item;
                }
            } else return null;
        }

        pub fn byEntityHandle(query: Q, entity: Entity) ?Item {
            const slice = query.iterator.entities.slice();
            const len = query.iterator.entities.len;
            var index = entity.index;
            if (index >= len or slice.items(.entity_id)[index].net_id != entity.net_id) {
                index = if (query.iterator.net_id_map) |map| blk: {
                    const mapped = map.get(entity.net_id) orelse return null;
                    if (mapped >= len or slice.items(.entity_id)[mapped].net_id != entity.net_id) return null;
                    break :blk mapped;
                } else blk: {
                    for (slice.items(.entity_id), 0..) |eid, i| {
                        if (eid.net_id == entity.net_id) break :blk i;
                    }
                    return null;
                };
            }

            var it = query.iterator;
            it.index = index;
            return it.peekCurrent(slice);
        }

        pub const Iterator = struct {
            entities: *std.MultiArrayList(EntityComponentStorage),
            net_id_map: ?*const std.array_hash_map.Auto(i64, usize) = null,
            index: usize = 0,

            pub fn next(it: *Iterator) ?Item {
                const slice = it.entities.slice();
                while (it.index < it.entities.len) : (it.index += 1) {
                    if (it.peekCurrent(slice)) |item| {
                        it.index += 1;
                        return item;
                    }
                }

                return null;
            }

            fn peekCurrent(it: *Iterator, slice: std.MultiArrayList(EntityComponentStorage).Slice) ?Item {
                @setEvalBranchQuota(2_000);
                var item: Item = undefined;

                inline for (types, 0..) |Component, i| {
                    if (Component == Entity) {
                        item[i] = .{
                            .index = it.index,
                            .net_id = slice.items(.entity_id)[it.index].net_id,
                        };

                        continue;
                    }

                    const index = comptime EntityComponentStorage.fieldIndexForType(Component);
                    const items = slice.items(@enumFromInt(index));
                    const requested_optional = comptime std.meta.activeTag(@typeInfo(Component)) == .optional;
                    const is_component_optional = comptime EntityComponentStorage.isComponentOptional(Component);

                    if (requested_optional and is_component_optional) {
                        item[i] = if (items[it.index]) |*comp| comp else null;
                    } else if (is_component_optional) {
                        // if the request is non-optional and component is null, skip this entity right away
                        item[i] = if (items[it.index]) |*comp| comp else return null;
                    } else {
                        item[i] = &items[it.index];
                    }
                }

                return item;
            }
        };
    };
}

pub fn clear_entities(scene: *Scene, arena: *std.heap.ArenaAllocator, fs: *FileSystem) !void {
    const entity_dir = try std.fmt.allocPrint(
        arena.allocator(),
        "player/{d}/scene/{d}/entity",
        .{ scene.player_id, scene.instance_id },
    );

    try fs.deleteTree(entity_dir);
}

pub fn init(
    gpa: Allocator,
    fs: *FileSystem,
    player_id: i32,
    instance_id: i32,
) !Scene {
    var arena = std.heap.ArenaAllocator.init(gpa);
    defer arena.deinit();

    var instance = try file_util.loadOrCreateZon(
        SceneInstance,
        gpa,
        arena.allocator(),
        fs,
        "player/{d}/scene/{d}/instance",
        .{ player_id, instance_id },
    );
    instance.buff_handle = 0;

    const formation_info = try file_util.loadOrCreateZon(
        FormationInfo,
        gpa,
        arena.allocator(),
        fs,
        "player/{d}/formation_info",
        .{player_id},
    );

    const explore_tools_info = try file_util.loadOrCreateZon(
        ExploreToolsInfo,
        gpa,
        arena.allocator(),
        fs,
        "player/{d}/explore_tools_info",
        .{player_id},
    );

    var scene: Scene = .{
        .player_id = player_id,
        .instance_id = instance_id,
        .instance = instance,
        .formation_info = formation_info,
        .explore_tools_info = explore_tools_info,
    };

    try scene.clear_entities(&arena, fs);

    errdefer scene.deinit(gpa, fs);

    return scene;
}

pub fn deinit(scene: *Scene, gpa: Allocator, fs: *FileSystem) void {
    scene.instance.deinit(gpa);
    scene.formation_info.deinit(gpa);
    scene.explore_tools_info.deinit(gpa);

    for (0..scene.entities.len) |i| {
        var entity = scene.entities.get(i);
        entity.deinit(gpa);
    }

    scene.entities.deinit(gpa);
    scene.net_id_map.deinit(gpa);
    scene.active_battle_entities.deinit(gpa);
    scene.rewarded_kills.deinit(gpa);
    scene.fsm_hate_states.deinit(gpa);
    scene.combine_relations.deinit(gpa);
    scene.pending_combine_detaches.deinit(gpa);
    scene.debug_spawn_routes.deinit(gpa);
    scene.debug_spawn_route_owners.deinit(gpa);
    scene.fsm_wakes.deinit(gpa);
    scene.clearActiveFlow(gpa);
    scene.executed_flow_action_ids.deinit(gpa);
    scene.clearPendingFlows(gpa);
    scene.pending_flows.deinit(gpa);
    scene.quest_timers.deinit(gpa);
    scene.lp_timers.deinit(gpa);
    scene.fail_timers.deinit(gpa);
    scene.lp_fail_timers.deinit(gpa);
    scene.clearBtTimers(gpa);
    scene.bt_timers.deinit(gpa);
    scene.clearBtOccupations(gpa);
    scene.bt_occupations.deinit(gpa);
    scene.save_node_keys.deinit(gpa);
    scene.trial_role_ids.deinit(gpa);
    scene.pending_killed_removals.deinit(gpa);
    for (scene.dynamic_interact_options.items) |*opt| opt.deinit(gpa);
    scene.dynamic_interact_options.deinit(gpa);
    for (scene.play_bubbles.items) |*rec| rec.deinit(gpa);
    scene.play_bubbles.deinit(gpa);
    scene.player_range_states.deinit(gpa);
    _ = fs;
}

/// Records the currently playing flow and owns a copy of its `flow_list_name`.
pub fn setActiveFlow(
    scene: *Scene,
    gpa: Allocator,
    inc_id: i64,
    quest_id: i32,
    node_id: i32,
    flow_list_name: []const u8,
    flow_id: i32,
    state_id: i32,
) !void {
    const dup_name = try gpa.dupe(u8, flow_list_name);
    scene.clearActiveFlow(gpa);
    scene.active_flow_inc_id = inc_id;
    scene.active_flow_quest_id = quest_id;
    scene.active_flow_node_id = node_id;
    scene.active_flow_list_name = dup_name;
    scene.active_flow_id = flow_id;
    scene.active_flow_state_id = state_id;
}

/// Clears active flow tracking state and releases any owned flow name buffer.
pub fn clearActiveFlow(scene: *Scene, gpa: Allocator) void {
    if (scene.active_flow_list_name.len > 0) {
        gpa.free(scene.active_flow_list_name);
        scene.active_flow_list_name = &.{};
    }
    scene.active_flow_inc_id = 0;
    scene.active_flow_quest_id = 0;
    scene.active_flow_node_id = 0;
    scene.active_flow_bt_type = .BtTypeQuest;
    scene.active_flow_id = 0;
    scene.active_flow_state_id = 0;
    scene.active_flow_owner_entity_id = 0;
    scene.flow_next_state_id = 0;
    scene.executed_flow_action_ids.clearRetainingCapacity();
}

pub fn markFlowActionExecuted(scene: *Scene, gpa: Allocator, action_id: i32) !void {
    if (action_id <= 0 or scene.hasFlowActionExecuted(action_id)) return;
    try scene.executed_flow_action_ids.append(gpa, action_id);
}

pub fn hasFlowActionExecuted(scene: *const Scene, action_id: i32) bool {
    if (action_id <= 0) return false;
    for (scene.executed_flow_action_ids.items) |id| {
        if (id == action_id) return true;
    }
    return false;
}

pub fn hasPendingFlowForNode(scene: *const Scene, quest_id: i32, node_id: i32) bool {
    for (scene.pending_flows.items) |queued| {
        if (queued.quest_id == quest_id and queued.node_id == node_id) return true;
    }
    return false;
}

/// Enqueues a flow to start once `active_flow_inc_id` finishes, skipping exact duplicates.
pub fn enqueuePendingFlow(
    scene: *Scene,
    gpa: Allocator,
    quest_id: i32,
    node_id: i32,
    flow_list_name: []const u8,
    flow_id: i32,
    state_id: i32,
) !void {
    if (scene.active_flow_inc_id != 0 and
        scene.active_flow_quest_id == quest_id and
        scene.active_flow_node_id == node_id and
        quest_id != 0)
    {
        return;
    }
    for (scene.pending_flows.items) |queued| {
        if (queued.quest_id == quest_id and
            queued.node_id == node_id and
            queued.flow_id == flow_id and
            queued.state_id == state_id and
            std.mem.eql(u8, queued.flow_list_name, flow_list_name))
        {
            return;
        }
    }

    const dup_name = try gpa.dupe(u8, flow_list_name);
    errdefer gpa.free(dup_name);

    try scene.pending_flows.append(gpa, .{
        .quest_id = quest_id,
        .node_id = node_id,
        .flow_list_name = dup_name,
        .flow_id = flow_id,
        .state_id = state_id,
    });
}

/// Pops the oldest queued flow (FIFO); caller owns the returned strings via `PendingFlow.deinit`.
pub fn popPendingFlow(scene: *Scene) ?PendingFlow {
    if (scene.pending_flows.items.len == 0) return null;
    return scene.pending_flows.orderedRemove(0);
}

/// Frees all queued flows without starting them.
pub fn clearPendingFlows(scene: *Scene, gpa: Allocator) void {
    for (scene.pending_flows.items) |*flow| flow.deinit(gpa);
    scene.pending_flows.clearRetainingCapacity();
}

pub fn spawn(scene: *Scene, gpa: Allocator, fs: *FileSystem, components: anytype) !Entity {
    return try spawnWithOptionalNetId(scene, gpa, fs, components, null);
}

pub fn spawnWithNetId(scene: *Scene, gpa: Allocator, fs: *FileSystem, net_id: i64, components: anytype) !Entity {
    if (net_id <= 0) return error.InvalidEntityId;
    if (scene.net_id_map.contains(net_id)) return error.EntityIdAlreadyExists;
    return try spawnWithOptionalNetId(scene, gpa, fs, components, net_id);
}

pub fn initFsmRuntimes(scene: *Scene, gpa: Allocator, now_ms: i64) !void {
    const slice = scene.entities.slice();
    for (slice.items(.entity_id), slice.items(.fsm)) |entity_id, *optional_fsm| {
        if (optional_fsm.*) |*fsm| {
            try fsm.initRuntime(gpa, now_ms);
            try scene.fsm_wakes.markDirty(gpa, entity_id.net_id);
        }
    }
}

pub fn setFsmEncounterTarget(scene: *Scene, gpa: Allocator, fsm_entity_id: i64, target_entity_id: ?i64) !bool {
    const fsm_index = scene.net_id_map.get(fsm_entity_id) orelse return error.EntityNotFound;
    const fsm_slot = &scene.entities.items(.fsm)[fsm_index];
    const fsm = if (fsm_slot.*) |*component| component else return error.EntityFsmNotFound;

    const target = if (target_entity_id) |target_id| blk: {
        if (!scene.net_id_map.contains(target_id)) return error.TargetEntityNotFound;
        break :blk std.math.cast(i32, target_id) orelse return error.FsmBlackboardTargetOutOfRange;
    } else null;

    const changed = fsm.setEncounterTarget(target);
    if (changed) {
        _ = fsm.markDirty(Entity.FsmComponent.WakeReason.blackboard);
        try scene.fsm_wakes.markDirty(gpa, fsm_entity_id);
    }
    return changed;
}

pub fn markFsmDirty(scene: *Scene, gpa: Allocator, entity_id: i64, reason: Entity.FsmComponent.WakeMask) !void {
    const index = scene.net_id_map.get(entity_id) orelse return;
    const fsm_slot = &scene.entities.items(.fsm)[index];
    const fsm = if (fsm_slot.*) |*component| component else return;
    if (fsm.markDirty(reason)) try scene.fsm_wakes.markDirty(gpa, entity_id);
}

pub fn updateFsmHateTarget(scene: *Scene, gpa: Allocator, entity_id: i64, has_target: bool) !bool {
    const index = scene.net_id_map.get(entity_id) orelse return false;
    const previous = scene.fsm_hate_states.get(entity_id);
    try scene.fsm_hate_states.put(gpa, entity_id, has_target);
    if (previous != null and previous.? == has_target) return false;

    const entity_changed = try scene.refreshFsmHateAtIndex(gpa, index, false);
    try scene.refreshSummonedFsmHate(gpa, entity_id);
    return entity_changed;
}

fn refreshFsmHateAtIndex(scene: *Scene, gpa: Allocator, index: usize, enqueue: bool) !bool {
    const fsm_slot = &scene.entities.items(.fsm)[index];
    const fsm = if (fsm_slot.*) |*component| component else return false;
    const entity_id = scene.entities.items(.entity_id)[index].net_id;
    var in_hate = scene.fsm_hate_states.get(entity_id) orelse false;

    if (in_hate) {
        if (scene.entities.items(.summoner)[index]) |summoner| {
            if (scene.net_id_map.contains(summoner.summoner_id)) {
                in_hate = scene.fsm_hate_states.get(summoner.summoner_id) orelse false;
            }
        }
    }

    if (!fsm.setInHate(in_hate)) return false;
    const should_wake = fsm.markDirty(Entity.FsmComponent.WakeReason.hate);
    if (enqueue and should_wake) try scene.fsm_wakes.markDirty(gpa, entity_id);
    return true;
}

fn refreshSummonedFsmHate(scene: *Scene, gpa: Allocator, summoner_id: i64) !void {
    const slice = scene.entities.slice();
    for (slice.items(.summoner), 0..) |optional_summoner, index| {
        const summoner = optional_summoner orelse continue;
        if (summoner.summoner_id != summoner_id) continue;
        _ = try scene.refreshFsmHateAtIndex(gpa, index, true);
    }
}

pub fn markFsmRootDirty(
    scene: *Scene,
    gpa: Allocator,
    entity_id: i64,
    fsm_id: i32,
    reason: Entity.FsmComponent.WakeMask,
) !void {
    const index = scene.net_id_map.get(entity_id) orelse return;
    const fsm_slot = &scene.entities.items(.fsm)[index];
    const fsm = if (fsm_slot.*) |*component| component else return;
    if (fsm.markRootDirty(fsm_id, reason)) try scene.fsm_wakes.markDirty(gpa, entity_id);
}

pub fn queueFsmSync(scene: *Scene, gpa: Allocator, entity_id: i64) !void {
    const index = scene.net_id_map.get(entity_id) orelse return;
    if (scene.entities.items(.fsm)[index] == null) return;
    try scene.fsm_wakes.markDirty(gpa, entity_id);
}

pub fn syncFsmDeadlines(
    scene: *Scene,
    gpa: Allocator,
    entity_id: i64,
    fsm: *Entity.FsmComponent,
) !void {
    for (fsm.runtime_nodes) |runtime| {
        if (runtime.next_timer_due_ms) |due_ms| {
            try scene.fsm_wakes.upsert(gpa, .{
                .entity_id = entity_id,
                .fsm_id = runtime.fsm_id,
                .kind = .root_timer,
                .due_ms = due_ms,
            });
        } else {
            scene.fsm_wakes.cancel(entity_id, runtime.fsm_id, .root_timer);
        }

        if (runtime.delayed_suicide_due_ms) |due_ms| {
            try scene.fsm_wakes.upsert(gpa, .{
                .entity_id = entity_id,
                .fsm_id = runtime.fsm_id,
                .kind = .delayed_suicide,
                .due_ms = due_ms,
            });
        } else {
            scene.fsm_wakes.cancel(entity_id, runtime.fsm_id, .delayed_suicide);
        }

        if (runtime.delayed_destroy_due_ms) |due_ms| {
            try scene.fsm_wakes.upsert(gpa, .{
                .entity_id = entity_id,
                .fsm_id = runtime.fsm_id,
                .kind = .delayed_destroy,
                .due_ms = due_ms,
            });
        } else {
            scene.fsm_wakes.cancel(entity_id, runtime.fsm_id, .delayed_destroy);
        }
    }

    if (fsm.pendingDeadline()) |due_ms| {
        try scene.fsm_wakes.upsert(gpa, .{
            .entity_id = entity_id,
            .kind = .pending_timeout,
            .due_ms = due_ms,
        });
    } else {
        scene.fsm_wakes.cancel(entity_id, 0, .pending_timeout);
    }

    if (fsm.paralysis_active and fsm.paralysis_next_ms != null) {
        try scene.fsm_wakes.upsert(gpa, .{
            .entity_id = entity_id,
            .kind = .paralysis,
            .due_ms = fsm.paralysis_next_ms.?,
        });
    } else {
        scene.fsm_wakes.cancel(entity_id, 0, .paralysis);
    }
}

pub fn hateTargetsFormation(scene: *const Scene, hate_list: []const pb.AiHateEntity) bool {
    const formation_index = std.math.cast(usize, scene.formation_info.cur_formation) orelse return false;
    if (formation_index >= scene.formation_info.formations.len) return false;
    const formation = scene.formation_info.formations[formation_index];

    for (hate_list) |hate| {
        if (hate.HatredValue < 1) continue;
        for (formation.roles) |maybe_role| {
            const role = maybe_role orelse continue;
            if (role.entity_id == hate.EntityId) return true;
        }
    }
    return false;
}

pub fn setBattleEntityActive(scene: *Scene, gpa: Allocator, entity_id: i64, active: bool) !void {
    if (active) {
        try scene.active_battle_entities.put(gpa, entity_id, {});
    } else {
        _ = scene.active_battle_entities.remove(entity_id);
    }
    scene.updateBattleState();
}

pub fn updateBattleEntityFromHate(
    scene: *Scene,
    gpa: Allocator,
    entity_id: i64,
    hate_list: []const pb.AiHateEntity,
) !void {
    try scene.setBattleEntityActive(gpa, entity_id, scene.hateTargetsFormation(hate_list));
}

pub fn forceBattleState(scene: *Scene, mode: bool) void {
    scene.active_battle_entities.clearRetainingCapacity();
    scene.player_in_battle = mode;
    scene.battle_state_dirty = scene.player_in_battle != scene.battle_state_notified;
}

pub fn combineRelation(scene: *const Scene, combine_entity_id: i64) ?CombineRelation {
    return scene.combine_relations.get(combine_entity_id);
}

pub fn registerCombineRelation(
    scene: *Scene,
    gpa: Allocator,
    combine_entity_id: i64,
    relation: CombineRelation,
) !CombineRegisterResult {
    if (scene.combine_relations.get(combine_entity_id)) |existing| {
        if (existing.matches(relation)) return .existing;
        return error.CombineRelationConflict;
    }

    try scene.combine_relations.put(gpa, combine_entity_id, relation);
    return .added;
}

pub fn detachCombineRelation(
    scene: *Scene,
    combine_entity_id: i64,
    target_entity_id: i64,
) ?CombineRelation {
    const relation = scene.combine_relations.get(combine_entity_id) orelse return null;
    if (relation.target_entity_id != target_entity_id) return null;
    _ = scene.combine_relations.remove(combine_entity_id);
    return relation;
}

pub fn removeCombineRelationsForEntity(scene: *Scene, gpa: Allocator, entity_id: i64) !void {
    var affected: std.ArrayList(i64) = .empty;
    defer affected.deinit(gpa);

    var detach_count: usize = 0;
    var iterator = scene.combine_relations.iterator();
    while (iterator.next()) |entry| {
        if (entry.key_ptr.* != entity_id and entry.value_ptr.target_entity_id != entity_id) continue;
        try affected.append(gpa, entry.key_ptr.*);
        if (entry.value_ptr.target_entity_id == entity_id and entry.key_ptr.* != entity_id) detach_count += 1;
    }

    try scene.pending_combine_detaches.ensureUnusedCapacity(gpa, detach_count);
    for (affected.items) |combine_entity_id| {
        const relation = scene.combine_relations.get(combine_entity_id) orelse continue;
        _ = scene.combine_relations.remove(combine_entity_id);
        if (relation.target_entity_id == entity_id and combine_entity_id != entity_id) {
            scene.pending_combine_detaches.appendAssumeCapacity(.{
                .combine_entity_id = combine_entity_id,
                .target_entity_id = relation.target_entity_id,
            });
        }
    }
}

pub fn pendingCombineDetaches(scene: *const Scene) []const CombineDetach {
    return scene.pending_combine_detaches.items;
}

pub fn clearPendingCombineDetaches(scene: *Scene) void {
    scene.pending_combine_detaches.clearRetainingCapacity();
}

pub fn pendingUpdateWakeDelayMs(scene: *const Scene) ?i64 {
    return pendingUpdateWakeDelay(scene.battle_state_dirty, scene.pending_combine_detaches.items.len);
}

fn pendingUpdateWakeDelay(battle_state_dirty: bool, pending_detach_count: usize) ?i64 {
    return if (battle_state_dirty or pending_detach_count != 0) 0 else null;
}

pub fn signalFsmDissolveCombine(scene: *Scene, gpa: Allocator, entity_id: i64) !void {
    const index = scene.net_id_map.get(entity_id) orelse return;
    const fsm_slot = &scene.entities.items(.fsm)[index];
    if (fsm_slot.*) |*fsm| {
        fsm.signalDissolveCombine();
        _ = fsm.markDirty(Entity.FsmComponent.WakeReason.dissolve);
        try scene.fsm_wakes.markDirty(gpa, entity_id);
    }
}

pub fn noteFsmSkillStart(scene: *Scene, gpa: Allocator, entity_id: i64, skill_id: i32, now_ms: i64) !void {
    const index = scene.net_id_map.get(entity_id) orelse return;
    const fsm_slot = &scene.entities.items(.fsm)[index];
    if (fsm_slot.*) |*fsm| {
        try fsm.initRuntime(gpa, now_ms);
        fsm.noteSkillStart(skill_id);
    }
}

pub fn signalFsmSkillEnd(scene: *Scene, gpa: Allocator, entity_id: i64, skill_id: i32, now_ms: i64) !void {
    const index = scene.net_id_map.get(entity_id) orelse return;
    const fsm_slot = &scene.entities.items(.fsm)[index];
    if (fsm_slot.*) |*fsm| {
        try fsm.initRuntime(gpa, now_ms);
        if (fsm.signalSkillEnd(skill_id)) try scene.fsm_wakes.markDirty(gpa, entity_id);
    }
}

pub fn removeCombineNotify(detach: CombineDetach) pb.CombatReceiveData {
    return .{ .Message = .{ .CombatNotifyData = .{
        .CombatCommon = .{ .EntityId = detach.combine_entity_id },
        .Message = .{ .RemoveCombineRelationNotify = .{
            .CombineEntity = detach.combine_entity_id,
            .TargetEntity = detach.target_entity_id,
        } },
    } } };
}

pub fn findEntityNetIdByConfigId(scene: *const Scene, config_id: i32) ?i64 {
    const slice = scene.entities.slice();
    for (slice.items(.entity_id), slice.items(.config)) |id, cfg| {
        if (cfg.config_id == config_id and cfg.entity_type != .player) return id.net_id;
    }
    for (slice.items(.entity_id), slice.items(.config)) |id, cfg| {
        if (cfg.config_id == config_id) return id.net_id;
    }
    return null;
}

pub fn findConfigIdByNetId(scene: *const Scene, net_id: i64) ?i32 {
    const idx = scene.net_id_map.get(net_id) orelse return null;
    return scene.entities.items(.config)[idx].config_id;
}

pub fn collectEntitiesInRadius(
    scene: *const Scene,
    gpa: Allocator,
    center: [3]f32,
    radius: f32,
    only_monsters_and_chests: bool,
    out_net_ids: *std.ArrayListUnmanaged(i64),
) !void {
    const r2 = radius * radius;
    const slice = scene.entities.slice();
    for (slice.items(.entity_id), slice.items(.config), slice.items(.position)) |id, cfg, opt_pos| {
        if (cfg.entity_type == .player) continue;
        if (only_monsters_and_chests and cfg.entity_type != .monster and cfg.entity_type != .scene_item) continue;
        const pos = opt_pos orelse continue;
        const dx = pos.location[0] - center[0];
        const dy = pos.location[1] - center[1];
        const dz = pos.location[2] - center[2];
        if (dx * dx + dy * dy + dz * dz <= r2) {
            try out_net_ids.append(gpa, id.net_id);
        }
    }
}

pub fn allocActionIncId(scene: *Scene) i32 {
    const id = scene.next_action_inc_id;
    scene.next_action_inc_id = if (id == std.math.maxInt(i32)) 1 else id + 1;
    return id;
}

pub fn setEntityVisibility(
    scene: *Scene,
    net_id: i64,
    visible: bool,
    combat_pack: *std.ArrayList(pb.CombatReceiveData),
    arena: Allocator,
) !bool {
    const idx = scene.net_id_map.get(net_id) orelse return false;
    scene.entities.items(.visible)[idx] = if (visible) Entity.VisibleMarker{} else null;
    scene.entities.items(.actor_visible)[idx] = if (visible) Entity.ActorVisibleMarker{} else null;
    try combat_pack.append(arena, .{ .Message = .{ .CombatNotifyData = .{
        .CombatCommon = .{ .EntityId = net_id, .MessageId = -1, .PreMessageId = -1 },
        .Message = .{ .EntityIsVisibleNotify = .{
            .Id = net_id,
            .IsVisible = visible,
        } },
    } } });
    try combat_pack.append(arena, .{ .Message = .{ .CombatNotifyData = .{
        .CombatCommon = .{ .EntityId = net_id, .MessageId = -1, .PreMessageId = -1 },
        .Message = .{ .ActorVisibleNotify = .{
            .Id = net_id,
            .IsActorVisible = visible,
        } },
    } } });
    return true;
}

pub fn updateEntityStateTag(
    scene: *Scene,
    gpa: Allocator,
    remaps: *const Assets.DataTables.GameplayTagRemapTable,
    net_id: i64,
    state_name: []const u8,
) !?StateTagUpdate {
    const new_tag_id: i32 = if (state_name.len == 0) 0 else try gameplay_tags.idFromName(remaps, state_name);
    return try scene.updateEntityStateTagId(gpa, net_id, new_tag_id);
}

pub fn updateEntityStateTagId(
    scene: *Scene,
    gpa: Allocator,
    net_id: i64,
    new_tag_id: i32,
) !?StateTagUpdate {
    const idx = scene.net_id_map.get(net_id) orelse return null;

    const state_slot = &scene.entities.items(.state_tag)[idx];
    const old_tag_id: i32 = if (state_slot.*) |st| st.state_tag_id else 0;
    state_slot.* = if (new_tag_id != 0) Entity.StateTagComponent{ .state_tag_id = new_tag_id } else null;

    const tag_slot = &scene.entities.items(.tag)[idx];
    if (old_tag_id != 0 and old_tag_id != new_tag_id) {
        if (tag_slot.*) |*tags| {
            _ = tags.removeCommonTag(gpa, old_tag_id);
            try tags.removeGameplayTag(gpa, old_tag_id);
        }
    }
    if (new_tag_id != 0) {
        if (tag_slot.*) |*tags| {
            try tags.addCommonTag(gpa, new_tag_id);
            try tags.setGameplayTagCount(gpa, new_tag_id, 1);
        } else {
            var new_tags = Entity.TagComponent{};
            try new_tags.addCommonTag(gpa, new_tag_id);
            try new_tags.setGameplayTagCount(gpa, new_tag_id, 1);
            tag_slot.* = new_tags;
        }
    }
    return .{ .old_tag_id = old_tag_id, .new_tag_id = new_tag_id };
}

pub fn setTrialRoles(scene: *Scene, gpa: Allocator, role_ids: []const i32) !void {
    scene.trial_role_ids.clearRetainingCapacity();
    try scene.trial_role_ids.appendSlice(gpa, role_ids);
}

pub fn upsertBtTimer(
    scene: *Scene,
    gpa: Allocator,
    owner_id: i32,
    inc_id: i64,
    node_id: i32,
    timer_type: []const u8,
    end_time_ms: i64,
) !ActiveBtTimer {
    for (scene.bt_timers.items) |*t| {
        if (t.owner_id == owner_id and t.inc_id == inc_id and std.mem.eql(u8, t.timer_type, timer_type)) {
            t.node_id = node_id;
            t.end_time_ms = end_time_ms;
            t.paused = false;
            t.pause_time_ms = 0;
            return t.*;
        }
    }
    const owned_type = try gpa.dupe(u8, timer_type);
    errdefer if (owned_type.len != 0) gpa.free(owned_type);
    try scene.bt_timers.append(gpa, .{
        .owner_id = owner_id,
        .inc_id = inc_id,
        .node_id = node_id,
        .timer_type = owned_type,
        .end_time_ms = end_time_ms,
    });
    return scene.bt_timers.items[scene.bt_timers.items.len - 1];
}

pub fn adjustBtTimer(
    scene: *Scene,
    gpa: Allocator,
    owner_id: i32,
    inc_id: i64,
    node_id: i32,
    timer_type: []const u8,
    now_ms: i64,
    seconds: i32,
    mode: pb.TimerSetType,
) !ActiveBtTimer {
    const delta_ms: i64 = @as(i64, seconds) * std.time.ms_per_s;
    for (scene.bt_timers.items) |*t| {
        if (t.owner_id == owner_id and t.inc_id == inc_id and std.mem.eql(u8, t.timer_type, timer_type)) {
            t.node_id = node_id;
            t.end_time_ms = switch (mode) {
                .Add => t.end_time_ms + delta_ms,
                .Sub => @max(now_ms, t.end_time_ms - delta_ms),
                .Set => now_ms + delta_ms,
            };
            return t.*;
        }
    }
    return try scene.upsertBtTimer(gpa, owner_id, inc_id, node_id, timer_type, now_ms + @max(0, delta_ms));
}

pub fn removeBtTimer(scene: *Scene, gpa: Allocator, owner_id: i32, inc_id: i64, timer_type: []const u8) bool {
    for (scene.bt_timers.items, 0..) |*t, i| {
        if (t.owner_id == owner_id and t.inc_id == inc_id and (timer_type.len == 0 or std.mem.eql(u8, t.timer_type, timer_type))) {
            var removed = scene.bt_timers.swapRemove(i);
            removed.deinit(gpa);
            return true;
        }
    }
    return false;
}

pub fn clearBtTimers(scene: *Scene, gpa: Allocator) void {
    for (scene.bt_timers.items) |*t| t.deinit(gpa);
    scene.bt_timers.clearRetainingCapacity();
}

pub fn nextQuestWakeDelayMs(scene: *const Scene, now_ms: i64) ?i64 {
    var best: ?i64 = scene.quest_timers.nextWakeDelayMs(now_ms);
    for ([_]?i64{
        scene.lp_timers.nextWakeDelayMs(now_ms),
        scene.fail_timers.nextWakeDelayMs(now_ms),
        scene.lp_fail_timers.nextWakeDelayMs(now_ms),
    }) |cand_opt| {
        if (cand_opt) |cand| {
            best = if (best) |cur| @min(cur, cand) else cand;
        }
    }
    return best;
}

pub fn clearBtOccupations(scene: *Scene, gpa: Allocator) void {
    for (scene.bt_occupations.items) |*o| o.deinit(gpa);
    scene.bt_occupations.clearRetainingCapacity();
}

pub fn findOccupationConflict(
    scene: *const Scene,
    resource_name: []const u8,
    owner_id: i32,
    inc_id: i64,
) ?BtOccupation {
    for (scene.bt_occupations.items) |occ| {
        if (std.mem.eql(u8, occ.resource_name, resource_name) and
            !(occ.owner_id == owner_id and occ.inc_id == inc_id))
        {
            return occ;
        }
    }
    return null;
}

pub fn upsertOccupation(
    scene: *Scene,
    gpa: Allocator,
    bt_type: pb.BtType,
    owner_id: i32,
    inc_id: i64,
    node_id: i32,
    resource_name: []const u8,
) !void {
    for (scene.bt_occupations.items) |*occ| {
        if (std.mem.eql(u8, occ.resource_name, resource_name)) {
            occ.bt_type = bt_type;
            occ.owner_id = owner_id;
            occ.inc_id = inc_id;
            occ.node_id = node_id;
            return;
        }
    }
    const owned = try gpa.dupe(u8, resource_name);
    errdefer gpa.free(owned);
    try scene.bt_occupations.append(gpa, .{
        .bt_type = bt_type,
        .owner_id = owner_id,
        .inc_id = inc_id,
        .node_id = node_id,
        .resource_name = owned,
    });
}

pub fn releaseOccupationsForTree(scene: *Scene, gpa: Allocator, owner_id: i32, inc_id: i64) void {
    var i: usize = 0;
    while (i < scene.bt_occupations.items.len) {
        if (scene.bt_occupations.items[i].owner_id == owner_id and scene.bt_occupations.items[i].inc_id == inc_id) {
            var removed = scene.bt_occupations.swapRemove(i);
            removed.deinit(gpa);
        } else {
            i += 1;
        }
    }
}

pub fn releaseOccupationsForNode(scene: *Scene, gpa: Allocator, owner_id: i32, node_id: i32) void {
    var i: usize = 0;
    while (i < scene.bt_occupations.items.len) {
        if (scene.bt_occupations.items[i].owner_id == owner_id and scene.bt_occupations.items[i].node_id == node_id) {
            var removed = scene.bt_occupations.swapRemove(i);
            removed.deinit(gpa);
        } else {
            i += 1;
        }
    }
}

fn saveNodeKey(owner_id: i32, node_id: i32) u64 {
    return (@as(u64, @as(u32, @bitCast(owner_id))) << 32) | @as(u64, @as(u32, @bitCast(node_id)));
}

pub fn hasSaveNodeEntered(scene: *const Scene, owner_id: i32, node_id: i32) bool {
    const k = saveNodeKey(owner_id, node_id);
    for (scene.save_node_keys.items) |item| {
        if (item == k) return true;
    }
    return false;
}

pub fn markSaveNodeEntered(scene: *Scene, gpa: Allocator, owner_id: i32, node_id: i32) !bool {
    if (scene.hasSaveNodeEntered(owner_id, node_id)) return false;
    try scene.save_node_keys.append(gpa, saveNodeKey(owner_id, node_id));
    return true;
}

pub fn clearSaveNodesForOwner(scene: *Scene, owner_id: i32) void {
    const prefix = @as(u64, @as(u32, @bitCast(owner_id))) << 32;
    const mask: u64 = 0xFFFF_FFFF_0000_0000;
    var i: usize = 0;
    while (i < scene.save_node_keys.items.len) {
        if ((scene.save_node_keys.items[i] & mask) == prefix) {
            _ = scene.save_node_keys.swapRemove(i);
        } else {
            i += 1;
        }
    }
}

pub fn setEntityPosition(scene: *Scene, net_id: i64, pos: [3]f32, yaw_opt: ?f32) bool {
    const idx = scene.net_id_map.get(net_id) orelse return false;
    const slot = &scene.entities.items(.position)[idx];
    if (slot.*) |*p| {
        p.location = pos;
        if (yaw_opt) |yaw| p.rotation[2] = yaw;
    } else {
        slot.* = .{
            .location = pos,
            .rotation = .{ 0, 0, yaw_opt orelse 0 },
        };
    }
    return true;
}

pub fn setCurrentRoleGravity(scene: *Scene, gravity_dir: [3]f32) void {
    const role_eid = scene.currentRoleEntityId();
    if (role_eid <= 0) return;
    const idx = scene.net_id_map.get(role_eid) orelse return;
    if (idx >= scene.entities.len) return;
    scene.entities.items(.world_item)[idx].gravity = .{ .location = gravity_dir };
}

pub fn upsertDynamicInteractOption(
    scene: *Scene,
    gpa: Allocator,
    guid: []const u8,
    entity_net_id: i64,
    entity_config_id: i32,
    quest_id: i32,
    node_id: i32,
    kind: DynamicInteractOption.Kind,
    flow_list_name: []const u8,
    flow_id: i32,
    state_id: i32,
    actions_json: []const u8,
) !void {
    return upsertDynamicInteractOptionWithBt(
        scene,
        gpa,
        guid,
        entity_net_id,
        entity_config_id,
        .BtTypeQuest,
        quest_id,
        node_id,
        kind,
        flow_list_name,
        flow_id,
        state_id,
        actions_json,
    );
}

pub fn upsertDynamicInteractOptionWithBt(
    scene: *Scene,
    gpa: Allocator,
    guid: []const u8,
    entity_net_id: i64,
    entity_config_id: i32,
    bt_type: pb.BtType,
    quest_id: i32,
    node_id: i32,
    kind: DynamicInteractOption.Kind,
    flow_list_name: []const u8,
    flow_id: i32,
    state_id: i32,
    actions_json: []const u8,
) !void {
    for (scene.dynamic_interact_options.items, 0..) |*existing, i| {
        if (std.mem.eql(u8, existing.option_guid, guid)) {
            var old = scene.dynamic_interact_options.swapRemove(i);
            old.deinit(gpa);
            break;
        }
    }
    const owned_guid = try gpa.dupe(u8, guid);
    errdefer if (owned_guid.len != 0) gpa.free(owned_guid);
    const owned_flow = try gpa.dupe(u8, flow_list_name);
    errdefer if (owned_flow.len != 0) gpa.free(owned_flow);
    const owned_actions = try gpa.dupe(u8, actions_json);
    errdefer if (owned_actions.len != 0) gpa.free(owned_actions);

    try scene.dynamic_interact_options.append(gpa, .{
        .option_guid = owned_guid,
        .entity_net_id = entity_net_id,
        .entity_config_id = entity_config_id,
        .bt_type = bt_type,
        .quest_id = quest_id,
        .node_id = node_id,
        .kind = kind,
        .flow_list_name = owned_flow,
        .flow_id = flow_id,
        .state_id = state_id,
        .actions_json = owned_actions,
    });
}

pub fn findDynamicInteractOption(scene: *const Scene, guid: []const u8) ?DynamicInteractOption {
    for (scene.dynamic_interact_options.items) |opt| {
        if (std.mem.eql(u8, opt.option_guid, guid)) return opt;
    }
    return null;
}

pub fn takeMatchingDynamicInteractOptions(
    scene: *Scene,
    arena: Allocator,
    entity_config_id: i32,
    flow_list_name_opt: ?[]const u8,
    flow_id_opt: ?i32,
) ![]DynamicInteractOption {
    var removed: std.ArrayList(DynamicInteractOption) = .empty;
    var i: usize = scene.dynamic_interact_options.items.len;
    while (i > 0) {
        i -= 1;
        const opt = scene.dynamic_interact_options.items[i];
        if (entity_config_id > 0 and opt.entity_config_id != entity_config_id) continue;
        if (flow_list_name_opt) |fname| {
            if (!std.mem.eql(u8, opt.flow_list_name, fname)) continue;
        }
        if (flow_id_opt) |fid| {
            if (fid > 0 and opt.flow_id != fid) continue;
        }
        try removed.append(arena, scene.dynamic_interact_options.swapRemove(i));
    }
    return removed.items;
}

pub fn takeDynamicInteractOptionsForQuestNode(
    scene: *Scene,
    arena: Allocator,
    quest_id: i32,
    node_id: i32,
) ![]DynamicInteractOption {
    return takeDynamicInteractOptionsForBtNode(scene, arena, .BtTypeQuest, quest_id, node_id);
}

pub fn takeDynamicInteractOptionsForBtNode(
    scene: *Scene,
    arena: Allocator,
    bt_type: pb.BtType,
    owner_id: i32,
    node_id: i32,
) ![]DynamicInteractOption {
    var removed: std.ArrayList(DynamicInteractOption) = .empty;
    var i: usize = scene.dynamic_interact_options.items.len;
    while (i > 0) {
        i -= 1;
        const opt = scene.dynamic_interact_options.items[i];
        if (opt.bt_type != bt_type or opt.quest_id != owner_id or opt.node_id != node_id) continue;
        try removed.append(arena, scene.dynamic_interact_options.swapRemove(i));
    }
    return removed.items;
}

pub fn addPlayBubble(scene: *Scene, gpa: Allocator, entity_config_id: i32, action_guid: []const u8) !void {
    if (entity_config_id <= 0 or action_guid.len == 0) return;
    for (scene.play_bubbles.items) |rec| {
        if (rec.entity_config_id == entity_config_id and std.mem.eql(u8, rec.action_guid, action_guid)) return;
    }
    const owned = try gpa.dupe(u8, action_guid);
    errdefer gpa.free(owned);
    try scene.play_bubbles.append(gpa, .{
        .entity_config_id = entity_config_id,
        .action_guid = owned,
    });
}

pub fn takePlayBubblesForEntity(scene: *Scene, arena: Allocator, entity_config_id: i32) ![]PlayBubbleRecord {
    var removed: std.ArrayList(PlayBubbleRecord) = .empty;
    var i: usize = scene.play_bubbles.items.len;
    while (i > 0) {
        i -= 1;
        if (scene.play_bubbles.items[i].entity_config_id != entity_config_id) continue;
        try removed.append(arena, scene.play_bubbles.swapRemove(i));
    }
    return removed.items;
}

pub fn debugSpawnRoute(scene: *const Scene, config_id: i64) ?DebugSpawnRoute {
    return scene.debug_spawn_routes.get(config_id);
}

pub fn registerDebugSpawnRoute(
    scene: *Scene,
    gpa: Allocator,
    net_id: i64,
    config_id: i64,
    source_map_id: i32,
    repeat_boss: bool,
) !void {
    if (scene.debug_spawn_route_owners.contains(net_id)) return error.DebugSpawnRouteOwnerExists;

    if (scene.debug_spawn_routes.getPtr(config_id)) |route| {
        if (!route.matches(source_map_id, repeat_boss)) return error.DebugSpawnRouteConflict;
        try scene.debug_spawn_route_owners.put(gpa, net_id, config_id);
        route.owner_count += 1;
        return;
    }

    try scene.debug_spawn_routes.put(gpa, config_id, .{
        .source_map_id = source_map_id,
        .repeat_boss = repeat_boss,
        .owner_count = 1,
    });
    errdefer _ = scene.debug_spawn_routes.remove(config_id);
    try scene.debug_spawn_route_owners.put(gpa, net_id, config_id);
}

pub fn unregisterDebugSpawnRoute(scene: *Scene, net_id: i64) void {
    const config_id = scene.debug_spawn_route_owners.get(net_id) orelse return;
    _ = scene.debug_spawn_route_owners.remove(net_id);

    const route = scene.debug_spawn_routes.getPtr(config_id) orelse return;
    if (route.owner_count > 1) {
        route.owner_count -= 1;
    } else {
        _ = scene.debug_spawn_routes.remove(config_id);
    }
}

pub fn appendBattleStateNotify(
    scene: *Scene,
    arena: Allocator,
    data: *std.ArrayList(pb.CombatReceiveData),
) !bool {
    if (!scene.battle_state_dirty) return false;

    try data.append(arena, .{ .Message = .{
        .CombatNotifyData = .{
            .CombatCommon = .{ .EntityId = scene.currentRoleEntityId() },
            .Message = .{
                .PlayerBattleStateChangeNotify = .{
                    .PlayerId = scene.player_id,
                    .InBattle = scene.player_in_battle,
                },
            },
        },
    } });
    scene.battle_state_notified = scene.player_in_battle;
    scene.battle_state_dirty = false;
    return true;
}

pub fn currentRoleEntityId(scene: *const Scene) i64 {
    const formation_index = std.math.cast(usize, scene.formation_info.cur_formation) orelse return -1;
    if (formation_index >= scene.formation_info.formations.len) return -1;
    const formation = scene.formation_info.formations[formation_index];
    for (formation.roles) |maybe_role| {
        const role = maybe_role orelse continue;
        if (role.role_id == formation.cur_role) return role.entity_id;
    }
    return -1;
}

fn updateBattleState(scene: *Scene) void {
    scene.player_in_battle = scene.active_battle_entities.count() != 0;
    scene.battle_state_dirty = scene.player_in_battle != scene.battle_state_notified;
}

test "scene pending updates wake only for pending work" {
    try std.testing.expect(pendingUpdateWakeDelay(false, 0) == null);
    try std.testing.expectEqual(@as(?i64, 0), pendingUpdateWakeDelay(true, 0));
    try std.testing.expectEqual(@as(?i64, 0), pendingUpdateWakeDelay(false, 1));
    try std.testing.expectEqual(@as(?i64, 0), pendingUpdateWakeDelay(true, 1));
}

test "scene active flow replaces and clears owned flow name" {
    const testing = std.testing;
    var scene = Scene{
        .player_id = 1,
        .instance_id = 8,
        .instance = undefined,
        .formation_info = undefined,
        .explore_tools_info = undefined,
    };
    defer scene.clearActiveFlow(testing.allocator);

    try scene.setActiveFlow(testing.allocator, 10, 139000025, 21, "剧情_新主线_CE版_序章", 1, 12);
    try testing.expectEqual(@as(i64, 10), scene.active_flow_inc_id);
    try testing.expectEqual(@as(i32, 1), scene.active_flow_id);
    try testing.expectEqual(@as(i32, 12), scene.active_flow_state_id);
    try testing.expectEqualStrings("剧情_新主线_CE版_序章", scene.active_flow_list_name);

    try scene.setActiveFlow(testing.allocator, 11, 139000025, 22, "next_flow", 2, 14);
    try testing.expectEqual(@as(i64, 11), scene.active_flow_inc_id);
    try testing.expectEqualStrings("next_flow", scene.active_flow_list_name);

    scene.clearActiveFlow(testing.allocator);
    try testing.expectEqual(@as(i64, 0), scene.active_flow_inc_id);
    try testing.expectEqual(@as(usize, 0), scene.active_flow_list_name.len);
}

fn spawnWithOptionalNetId(scene: *Scene, gpa: Allocator, fs: *FileSystem, components: anytype, requested_net_id: ?i64) !Entity {
    const log = std.log.scoped(.entity_spawn);
    var storage: EntityComponentStorage = undefined;

    inline for (comptime std.meta.fields(EntityComponentStorage)) |storage_field| {
        if (comptime std.mem.eql(u8, storage_field.name, "entity_id")) continue;
        if (comptime std.mem.eql(u8, storage_field.name, "world_item")) {
            storage.world_item = .{};
            continue;
        }

        const is_optional = comptime std.meta.activeTag(@typeInfo(storage_field.type)) == .optional;
        inline for (comptime std.meta.fields(@TypeOf(components)), 0..) |param_field, i| {
            if ((!is_optional and param_field.type == storage_field.type) or
                (is_optional and (param_field.type == storage_field.type or ?param_field.type == storage_field.type)))
            {
                @field(storage, storage_field.name) = components[i];
                break;
            }
        } else if (!is_optional) {
            log.err("missing required component '" ++ @typeName(storage_field.type) ++ "'", .{});
            return error.MissingRequiredComponent;
        } else {
            @field(storage, storage_field.name) = null;
        }
    }

    const id = requested_net_id orelse try scene.nextEntityId(fs, gpa);
    if (requested_net_id != null) try scene.ensureNextEntityIdAtLeast(fs, gpa, id + 1);

    storage.entity_id = .{ .net_id = id };
    if (storage.motor_da_ctx) |*comp| {
        if (comp.context_id == 0) comp.context_id = id;
    }
    if (storage.fsm) |*fsm| {
        try fsm.initRuntime(gpa, std.Io.Clock.awake.now(fs.io).toMilliseconds());
    }

    try scene.net_id_map.put(gpa, id, scene.entities.len);
    try scene.entities.append(gpa, storage);
    _ = try scene.refreshFsmHateAtIndex(gpa, scene.entities.len - 1, false);
    try scene.refreshSummonedFsmHate(gpa, id);

    if (storage.fsm != null) try scene.fsm_wakes.markDirty(gpa, id);

    return .{ .index = scene.entities.len - 1, .net_id = id };
}

pub fn delete(
    fs: *FileSystem,
    player_id: i32,
    scene_id: i32,
    net_id: i64,
) !void {
    _ = fs;
    _ = player_id;
    _ = scene_id;
    _ = net_id;
}

pub fn remove(scene: *Scene, gpa: Allocator, fs: *FileSystem, net_id: i64) !void {
    _ = fs;
    const index = scene.net_id_map.get(net_id) orelse return;
    try scene.removeEntityState(gpa, index, net_id);
}

pub fn rollbackSpawn(scene: *Scene, gpa: Allocator, fs: *FileSystem, net_id: i64) !void {
    _ = fs;
    const index = scene.net_id_map.get(net_id) orelse return;
    try scene.removeEntityState(gpa, index, net_id);
}

fn removeEntityState(scene: *Scene, gpa: Allocator, index: usize, net_id: i64) !void {
    try scene.clearFsmEncounterTargetReferences(gpa, net_id);
    try scene.removeCombineRelationsForEntity(gpa, net_id);
    scene.fsm_wakes.cancelEntity(net_id);
    scene.unregisterDebugSpawnRoute(net_id);
    _ = scene.active_battle_entities.remove(net_id);
    _ = scene.rewarded_kills.remove(net_id);
    scene.updateBattleState();
    var storage = scene.entities.get(index);
    storage.deinit(gpa);
    scene.entities.swapRemove(index);
    _ = scene.net_id_map.swapRemove(net_id);
    _ = scene.fsm_hate_states.remove(net_id);
    if (index < scene.entities.len) {
        const swapped_id = scene.entities.items(.entity_id)[index].net_id;
        try scene.net_id_map.put(gpa, swapped_id, index);
    }
    try scene.refreshSummonedFsmHate(gpa, net_id);
}

fn clearFsmEncounterTargetReferences(scene: *Scene, gpa: Allocator, target_entity_id: i64) !void {
    const target_id = std.math.cast(i32, target_entity_id) orelse return;
    const slice = scene.entities.slice();
    for (slice.items(.entity_id), slice.items(.fsm)) |entity_id, *optional_fsm| {
        if (optional_fsm.*) |*fsm| {
            if (fsm.clearEncounterTargetReference(target_id)) {
                _ = fsm.markDirty(Entity.FsmComponent.WakeReason.blackboard);
                try scene.fsm_wakes.markDirty(gpa, entity_id.net_id);
            }
        }
    }
}

pub fn savePersistent(scene: *Scene, fs: *FileSystem, gpa: Allocator) !void {
    var arena = std.heap.ArenaAllocator.init(gpa);
    defer arena.deinit();

    const instance_path = try std.fmt.allocPrint(arena.allocator(), "player/{d}/scene/{d}/instance", .{ scene.player_id, scene.instance_id });
    try comp_util.saveStruct(fs, scene.instance, instance_path, arena.allocator());
    const formation_path = try std.fmt.allocPrint(arena.allocator(), "player/{d}/formation_info", .{scene.player_id});
    try comp_util.saveStruct(fs, scene.formation_info, formation_path, arena.allocator());
    const explore_tools_path = try std.fmt.allocPrint(arena.allocator(), "player/{d}/explore_tools_info", .{scene.player_id});
    try comp_util.saveStruct(fs, scene.explore_tools_info, explore_tools_path, arena.allocator());
}

pub fn save(scene: *Scene, fs: *FileSystem, gpa: Allocator) !void {
    var arena = std.heap.ArenaAllocator.init(gpa);
    defer arena.deinit();

    try scene.savePersistent(fs, gpa);

    for (0..scene.entities.len) |i| {
        const entity = scene.entities.get(i);
        try entity.save(arena.allocator(), fs, scene.player_id, scene.instance_id);
    }
}

pub fn saveLastScene(scene: *Scene, scene_comp: *PlayerSceneComponent, fs: *FileSystem, gpa: Allocator) !void {
    var arena = std.heap.ArenaAllocator.init(gpa);
    defer arena.deinit();

    const last_scene_info = try std.fmt.allocPrint(arena.allocator(), "player/{}/last_scene", .{scene.player_id});
    try comp_util.saveStruct(fs, scene_comp.last_scene_info, last_scene_info, arena.allocator());
}

pub fn saveEntity(
    scene: *Scene,
    fs: *FileSystem,
    gpa: Allocator,
    entity: Entity,
) !void {
    const storage = scene.entities.get(entity.index);

    var arena = std.heap.ArenaAllocator.init(gpa);
    defer arena.deinit();

    try storage.save(arena.allocator(), fs, scene.player_id, scene.instance_id);
}

pub fn saveComponents(
    scene: *Scene,
    fs: *FileSystem,
    gpa: Allocator,
    entity: Entity,
    comptime components: []const type,
) !void {
    _ = scene;
    _ = fs;
    _ = gpa;
    _ = entity;
    _ = components;
}

pub fn saveInstance(scene: *Scene, fs: *FileSystem, gpa: Allocator) !void {
    var arena = std.heap.ArenaAllocator.init(gpa);
    defer arena.deinit();

    const path = try std.fmt.allocPrint(
        arena.allocator(),
        "player/{d}/scene/{d}/instance",
        .{ scene.player_id, scene.instance_id },
    );
    try comp_util.saveStruct(fs, scene.instance, path, arena.allocator());
}

pub fn nextEntityId(scene: *Scene, fs: *FileSystem, gpa: Allocator) !i64 {
    _ = fs;
    _ = gpa;
    const id = scene.next_entity_id;
    scene.next_entity_id += 1;
    return id;
}

pub fn ensureNextEntityIdAtLeast(scene: *Scene, fs: *FileSystem, gpa: Allocator, next_id: i64) !void {
    _ = fs;
    _ = gpa;
    if (next_id > scene.next_entity_id) {
        scene.next_entity_id = next_id;
    }
}

fn entityCounterPath(scene: *Scene, gpa: Allocator) ![]u8 {
    return try std.fmt.allocPrint(
        gpa,
        "player/{d}/scene/{d}/entity/next",
        .{ scene.player_id, scene.instance_id },
    );
}

const BuffAdditionEntry = @import("../logic/events.zig").BuffAdditionEntry;
pub fn buildFightBuffInfos(
    scene: *Scene,
    role_buffs: std.ArrayListUnmanaged(BuffAdditionEntry),
    net_id: i64,
    gpa: Allocator,
) ![]pb.FightBuffInformation {
    var infos: std.ArrayList(pb.FightBuffInformation) = .empty;
    errdefer infos.deinit(gpa);
    for (role_buffs.items) |entry| {
        scene.*.instance.buff_handle += 1;
        try infos.append(gpa, Assets.DataTables.createBuffInformation(
            scene.instance.buff_handle,
            entry.id,
            net_id,
            net_id,
            entry.is_active,
        ));
    }
    return infos.toOwnedSlice(gpa);
}
