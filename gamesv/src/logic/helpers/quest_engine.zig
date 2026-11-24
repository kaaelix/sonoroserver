const std = @import("std");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const common = @import("common");
const FileSystem = common.FileSystem;
const Connection = @import("../../network/Connection.zig");
const Assets = @import("../../data/Assets.zig");
const LazyNodeStore = @import("../../data/LazyNodeStore.zig");
const QuestData = @import("../../data/tables/QuestData.zig");
const GuideInfo = @import("../../fs/GuideInfo.zig");
const InventoryInfo = @import("../../fs/InventoryInfo.zig");
const QuestInfo = @import("../../fs/QuestInfo.zig");
const ProgressInfo = @import("../../fs/ProgressInfo.zig");
const PlayerProgressComponent = @import("../component/player/PlayerProgressComponent.zig");
const comp_util = @import("../component/comp_util.zig");
const Scene = @import("../Scene.zig");
const ConditionEvaluator = @import("condition.zig");
const gameplay_tags = @import("gameplay_tags.zig");
const RewardService = @import("reward.zig");
const quest_types = @import("quest_types.zig");
const MoraleHelper = @import("morale.zig");
const QuestActions = @import("quest_actions.zig");
const LevelPlayHelper = @import("level_play.zig");
const RoleHelper = @import("role.zig");
const RoleEntityTemplates = @import("../templates/RoleEntityTemplates.zig");
const entity_proto = @import("entity_proto.zig");
const TriggerConfig = @import("../../data/tables/entity_components/Trigger.zig");

const log = std.log.scoped(.quest_engine);

pub const prologue_quest_id: i32 = 139000025;
const max_steps_per_advance: usize = 96;

pub const QuestContext = quest_types.QuestContext;
pub const DeferredBtVar = quest_types.DeferredBtVar;
pub const makeEvalContext = quest_types.makeEvalContext;
const makeEvalContextForQuest = quest_types.makeEvalContextForQuest;
pub const makeRewardContext = quest_types.makeRewardContext;
pub const unlockTeleporter = QuestActions.unlockTeleporter;

/// Logs non-fatal quest hook errors where the primary RPC/tick must continue.
pub fn logHookErr(comptime hook: []const u8, err: anyerror) void {
    log.warn("quest hook {s} failed: {t}", .{ hook, err });
}
pub const syncWorldStateOnJoin = QuestActions.syncWorldStateOnJoin;
pub const emitEntityStateTagUpdate = QuestActions.emitEntityStateTagUpdate;
pub const emitEntityStateTagUpdateChecked = QuestActions.emitEntityStateTagUpdateChecked;
pub const hasClientLevelTagComponent = @import("../handlers/tags.zig").hasClientLevelTagComponent;
pub const entityDataLayersActive = QuestActions.entityDataLayersActive;
pub const syncPersistedVehicleNpcLinks = QuestActions.syncPersistedVehicleNpcLinks;
pub const updatePlayerSceneState = QuestActions.updatePlayerSceneState;
pub const leaveCurrentDungeon = QuestActions.leaveCurrentDungeon;
pub const executeActionList = QuestActions.executeActionList;

const StepResult = enum {
    running,
    completed,
    failed,
    quest_ended,
};

pub fn actionCallbacks() quest_types.ActionCallbacks {
    return .{
        .start_flow = startFlowWithMode,
        .start_flow_state = startFlowParams,
        .complete_guide = completeGuideById,
        .on_entity_state_changed = onEntityStateChanged,
        .on_phone_message = onPhoneMessageEvent,
        .on_npc_mail = onNpcMailSent,
        .on_time_changed = onTimeChanged,
        .on_weather_changed = onWeatherChanged,
        .on_item_gained = onItemGained,
        .on_level_play_enabled = onLevelPlayEnabled,
        .on_accept_focus_wait = acceptFocusWaitQuestAction,
        .on_finish_flow_state = finishFlowStateFromAction,
        .on_destroy_quest = destroyQuest,
        .on_finish_condition = finishConditionFromAction,
        .on_trace_current_quest = traceCurrentQuestFromAction,
        .on_var_changed = onVarChanged,
    };
}

pub fn onVarChanged(ctx: QuestContext, owner_id: i32, var_name: []const u8) !void {
    const VarEvt = struct { owner_id: i32, name: []const u8 };
    const Check = struct {
        fn run(
            c: QuestContext,
            ev: VarEvt,
            qid: i32,
            _: QuestInfo.ActiveQuest,
            nodes: []const LazyNodeStore.NodeEntry,
            node: LazyNodeStore.NodeEntry,
            cond: std.json.ObjectMap,
            ctype: []const u8,
        ) !bool {
            if (!std.mem.eql(u8, ctype, "ListenToEvent")) return false;
            const et = cond.get("EventType") orelse return false;
            if (et != .object) return false;
            if (!std.mem.eql(u8, jsonStr(et.object.get("Type")) orelse "", "VarValueChange")) return false;
            const want_name = jsonStr(et.object.get("VarName")) orelse jsonStr(et.object.get("Name")) orelse "";
            if (want_name.len > 0 and ev.name.len > 0 and !std.mem.eql(u8, want_name, ev.name)) return false;
            if (et.object.get("Var")) |vobj| {
                if (vobj == .object) {
                    if (quest_types.resolveVarName(vobj.object)) |vn| {
                        if (vn.len > 0 and ev.name.len > 0 and !std.mem.eql(u8, vn, ev.name)) return false;
                    }
                }
            }
            const _oid = ev.owner_id;
            _ = _oid;
            try completeChildQuest(c, qid, nodes, node);
            return true;
        }
    };
    _ = try advanceMatchingChildNodes(ctx, VarEvt{ .owner_id = owner_id, .name = var_name }, Check.run);
    if (refresh_loop_depth == 0) {
        if (try recheckPendingConditions(ctx)) {
            try finalizeProgression(ctx);
        }
    }
    if (LevelPlayHelper.fromQuestContext(ctx)) |lp_ctx| {
        try LevelPlayHelper.onVarChanged(lp_ctx, owner_id, var_name);
    }
}

fn traceCurrentQuestFromAction(ctx: QuestContext, quest_id: i32) !void {
    try onSceneTrace(ctx, quest_id);
}

const jsonStr = quest_types.jsonStr;
const jsonInt = quest_types.jsonInt;
const jsonFloat = quest_types.jsonFloat;
const jsonBool = quest_types.jsonBool;
const resolveVarInt = quest_types.resolveVarInt;

pub fn syncOnLogin(ctx: QuestContext) !void {
    // Ensure Synthesis (10035) is marked unlocked so synthesis menu RPCs are not gated.
    _ = try ctx.quest_comp.info.unlockSystem(ctx.alloc.gpa, 10035);
    _ = try refreshAvailableQuests(ctx, false);
    _ = try refreshDailyQuests(ctx, false);

    // Ensure every active quest has at least one active node initialized
    const active_snapshot = try snapshotActiveQuestIds(ctx);
    for (active_snapshot) |qid| {
        const aq = ctx.quest_comp.info.findActiveQuest(qid) orelse continue;
        if (aq.active_nodes.len == 0) {
            try advanceQuestTree(ctx, qid, false);
        }
    }

    ensureTrackedFocusValid(ctx);
    if (ctx.scene) |scene| scene.current_quest_id = ctx.quest_comp.info.tracked_quest_id;
    syncSceneCurrentNode(ctx);

    try pushQuestStateNotifiesWithEnter(ctx, false, true);
    if (LevelPlayHelper.fromQuestContext(ctx)) |lp_ctx| {
        try LevelPlayHelper.syncOnLogin(lp_ctx);
    }
    try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);
}

/// Re-pushes tracked quest state and active dynamic interact options after silent reconnect without flooding QuestListNotify.
pub fn resyncOnReconnect(ctx: QuestContext) !void {
    ensureTrackedFocusValid(ctx);
    if (ctx.scene) |scene| scene.current_quest_id = ctx.quest_comp.info.tracked_quest_id;
    syncSceneCurrentNode(ctx);

    ensureActiveQuestTargetsSpawned(ctx);
    if (ctx.scene) |scene| {
        for (ctx.quest_comp.info.active_quests) |aq| {
            const nodes = ctx.assets.quest_nodes.getOwnerNodes(ctx.io, aq.quest_id) catch continue;
            for (aq.active_nodes) |nid| {
                const node = findNodeById(nodes, nid) orelse continue;
                try pushDoInteractOptionsForNode(scene, ctx.conn, ctx.alloc, ctx.io, aq.quest_id, node.*, null);
            }
        }
    }
    if (LevelPlayHelper.fromQuestContext(ctx)) |lp_ctx| {
        try LevelPlayHelper.syncOnLogin(lp_ctx);
    }
    try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.TraceQuestNotify{ .QuestId = ctx.quest_comp.info.tracked_quest_id });
    if (ctx.quest_comp.info.focus_quest_id > 0) {
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.FocusQuestSetNotify{ .FocusQuestId = ctx.quest_comp.info.focus_quest_id, .Reason = .Inherit });
    }
}

/// Handles `SceneTraceRequest`, switching/resyncing quest tracking and spawning trace targets.
pub fn onSceneTrace(ctx: QuestContext, trace_id: i64) !void {
    if (trace_id > 0) {
        if (std.math.cast(i32, trace_id)) |id32| {
            if (ctx.quest_comp.info.hasActiveQuest(id32)) {
                ctx.quest_comp.info.tracked_quest_id = id32;
                try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);
            } else {
                QuestActions.ensureQuestEntitySpawned(ctx, id32);
            }
        }
    }
    try resyncOnReconnect(ctx);
}

pub fn onSceneLoadingFinish(ctx: QuestContext) !void {
    const scene = ctx.scene orelse return;
    ensureActiveQuestTargetsSpawned(ctx);
    try ensureActiveQuestTimersScheduled(ctx);

    // Always re-sync quest tree & tracker state after scene loading finishes (including reconnects
    // or cutscenes in progress) so HUD objectives and map markers never disappear.
    try pushQuestStateNotifies(ctx);
    try syncMoraleOnSceneLoad(ctx);

    if (scene.active_flow_inc_id != 0) return;

    const tracked = ctx.quest_comp.info.tracked_quest_id;
    if (try tryStartPendingPlayFlowForQuest(ctx, tracked)) return;

    const active_snapshot = try snapshotActiveQuestIds(ctx);
    for (active_snapshot) |qid| {
        if (qid == tracked) continue;
        if (try tryStartPendingPlayFlowForQuest(ctx, qid)) return;
    }
}

fn ensureActiveQuestTargetsSpawned(ctx: QuestContext) void {
    for (ctx.quest_comp.info.active_quests) |aq| {
        const nodes = ctx.assets.quest_nodes.getOwnerNodes(ctx.io, aq.quest_id) catch continue;
        for (aq.active_nodes) |nid| {
            const node = findNodeById(nodes, nid) orelse continue;
            spawnNodeTargetEntities(ctx, aq.quest_id, node.*);
        }
    }
}

fn visitActiveQuestTargetIds(
    quest_info: QuestInfo,
    io: std.Io,
    assets: *const Assets,
    ctx: anytype,
    comptime visitor: fn (@TypeOf(ctx), i32) anyerror!bool,
) !bool {
    var buf: [32]i32 = undefined;
    for (quest_info.active_quests) |aq| {
        const nodes = assets.quest_nodes.getOwnerNodes(io, aq.quest_id) catch continue;
        for (aq.active_nodes) |nid| {
            const node = findNodeById(nodes, nid) orelse continue;
            for (extractNodeTargetEntityIds(&buf, node.*)) |target_id| {
                if (try visitor(ctx, target_id)) return true;
            }
        }
    }
    return false;
}

/// Returns whether `cfg_id` is referenced by any active quest node (condition targets or `TrackTarget`).
pub fn isEntityActiveQuestTarget(
    quest_info: QuestInfo,
    io: std.Io,
    assets: *const Assets,
    cfg_id: i64,
) bool {
    const id32 = std.math.cast(i32, cfg_id) orelse return false;
    if (id32 <= 0) return false;
    const MatchCtx = struct {
        needle: i32,
        fn check(c: @This(), target_id: i32) !bool {
            return target_id == c.needle;
        }
    };
    return visitActiveQuestTargetIds(quest_info, io, assets, MatchCtx{ .needle = id32 }, MatchCtx.check) catch false;
}

/// Collects all positive entity config IDs referenced by active quest nodes.
pub fn collectActiveQuestTargetIds(
    quest_info: QuestInfo,
    io: std.Io,
    assets: *const Assets,
    gpa: std.mem.Allocator,
) !std.AutoHashMapUnmanaged(i32, void) {
    var ids: std.AutoHashMapUnmanaged(i32, void) = .empty;
    errdefer ids.deinit(gpa);

    const CollectCtx = struct {
        map: *std.AutoHashMapUnmanaged(i32, void),
        gpa: std.mem.Allocator,
        fn add(c: @This(), target_id: i32) !bool {
            try c.map.put(c.gpa, target_id, {});
            return false;
        }
    };
    _ = try visitActiveQuestTargetIds(quest_info, io, assets, CollectCtx{ .map = &ids, .gpa = gpa }, CollectCtx.add);
    return ids;
}

pub fn isTargetIdInSet(
    target_ids: *const std.AutoHashMapUnmanaged(i32, void),
    cfg_id: i64,
) bool {
    const id32 = std.math.cast(i32, cfg_id) orelse return false;
    return id32 > 0 and target_ids.contains(id32);
}

pub fn shouldHideWorldEntity(
    info: QuestInfo,
    targets: *const std.AutoHashMapUnmanaged(i32, void),
    cfg_id: i64,
) bool {
    const id32 = std.math.cast(i32, cfg_id) orelse return false;
    if (id32 <= 0) return false;
    if (QuestInfo.containsInt(info.world.shown_entities, id32)) return false;
    if (QuestInfo.containsInt(info.world.hidden_entities, id32)) return true;
    if (isTargetIdInSet(targets, id32)) return false;
    for (info.world.hidden_groups) |grp| {
        if (QuestInfo.containsInt(grp.entity_ids, id32)) return true;
    }
    return false;
}

/// Collects unique positive entity config IDs from a `ChildQuest` node's `Condition` and `TrackTarget`.
pub fn extractNodeTargetEntityIds(out: []i32, node: LazyNodeStore.NodeEntry) []const i32 {
    var count: usize = 0;
    if (getChildCondition(node.data)) |cond| {
        const ctype = jsonStr(cond.get("Type")) orelse "";
        if (std.mem.eql(u8, ctype, "ReachArea")) {
            appendJsonIntArray(out, &count, cond.get("RangeEntities"));
            appendUniqueTargetId(out, &count, jsonInt(cond.get("RangeEntityId")) orelse 0);
            appendUniqueTargetId(out, &count, jsonInt(cond.get("EntityId")) orelse 0);
        } else if (std.mem.eql(u8, ctype, "Kill")) {
            appendJsonIntArray(out, &count, cond.get("TargetsToAwake"));
            appendJsonIntArray(out, &count, cond.get("ExistTargets"));
        } else if (std.mem.eql(u8, ctype, "MonsterCreator")) {
            appendJsonIntArray(out, &count, cond.get("MonsterCreatorEntityIds"));
        } else if (std.mem.eql(u8, ctype, "AwakeAndLoadEntity")) {
            appendJsonIntArray(out, &count, cond.get("EntityIds"));
        } else if (std.mem.eql(u8, ctype, "DetectCombatState") or std.mem.eql(u8, ctype, "DetectCombatState2")) {
            appendUniqueTargetId(out, &count, jsonInt(cond.get("EntityId")) orelse 0);
            if (cond.get("Conditions")) |cs| {
                if (cs == .array) {
                    for (cs.array.items) |c| {
                        if (c == .object) appendUniqueTargetId(out, &count, jsonInt(c.object.get("EntityId")) orelse 0);
                    }
                }
            }
        } else if (std.mem.eql(u8, ctype, "DoInteract")) {
            appendUniqueTargetId(out, &count, jsonInt(cond.get("EntityId")) orelse 0);
            if (cond.get("AddOptions")) |opts_val| {
                if (opts_val == .array) {
                    for (opts_val.array.items) |opt_v| {
                        if (opt_v == .object) {
                            appendUniqueTargetId(out, &count, jsonInt(opt_v.object.get("EntityId")) orelse 0);
                        }
                    }
                }
            }
        } else if (std.mem.eql(u8, ctype, "HandInItems")) {
            if (cond.get("AddOption")) |ao| {
                if (ao == .object) appendUniqueTargetId(out, &count, jsonInt(ao.object.get("EntityId")) orelse 0);
            } else if (cond.get("HandInItems")) |hi| {
                if (hi == .object) {
                    if (hi.object.get("AddOption")) |ao| {
                        if (ao == .object) appendUniqueTargetId(out, &count, jsonInt(ao.object.get("EntityId")) orelse 0);
                    }
                }
            }
        } else if (std.mem.eql(u8, ctype, "TakePhoto") or std.mem.eql(u8, ctype, "TakePhoto2")) {
            if (cond.get("PhotoTargets")) |pts| {
                if (pts == .array) {
                    for (pts.array.items) |pt| {
                        if (pt == .object) appendUniqueTargetId(out, &count, jsonInt(pt.object.get("EntityId")) orelse 0);
                    }
                }
            }
            if (cond.get("PosCondition")) |pc| {
                if (pc == .object) appendUniqueTargetId(out, &count, jsonInt(pc.object.get("RangeEntity")) orelse 0);
            }
        } else if (std.mem.eql(u8, ctype, "CheckEntityState")) {
            if (cond.get("Conditions")) |cs| {
                if (cs == .array) {
                    for (cs.array.items) |c| {
                        if (c == .object) appendUniqueTargetId(out, &count, jsonInt(c.object.get("EntityId")) orelse 0);
                    }
                }
            }
        } else if (std.mem.eql(u8, ctype, "ListenToEvent")) {
            if (cond.get("EventType")) |et| {
                if (et == .object) appendUniqueTargetId(out, &count, jsonInt(et.object.get("EntityId")) orelse 0);
            }
        } else if (std.mem.eql(u8, ctype, "WaitBattleCondition")) {
            if (cond.get("Option")) |opts| {
                if (opts == .array) {
                    for (opts.array.items) |o| {
                        if (o == .object) appendUniqueTargetId(out, &count, jsonInt(o.object.get("EntityId")) orelse 0);
                    }
                }
            }
        } else if (std.mem.eql(u8, ctype, "CheckTargetBattleAttribute")) {
            if (cond.get("Option")) |opts| {
                if (opts == .array) {
                    for (opts.array.items) |o| {
                        if (o == .object) appendJsonIntArray(out, &count, o.object.get("EntityIds"));
                    }
                }
            }
        }
    }
    if (node.data == .object) {
        if (node.data.object.get("TrackTarget")) |tt_val| {
            if (tt_val == .object) {
                if (tt_val.object.get("TrackType")) |track_type| {
                    if (track_type == .object) {
                        appendJsonIntArray(out, &count, track_type.object.get("EntityIds"));
                        if (track_type.object.get("Entities")) |ents_val| {
                            if (ents_val == .array) {
                                for (ents_val.array.items) |item| {
                                    const eid = switch (item) {
                                        .object => |obj| jsonInt(obj.get("EntityId")) orelse jsonInt(obj.get("Id")) orelse 0,
                                        else => jsonInt(item) orelse 0,
                                    };
                                    appendUniqueTargetId(out, &count, eid);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return out[0..count];
}

fn spawnNodeTargetEntities(ctx: QuestContext, quest_id: i32, node: LazyNodeStore.NodeEntry) void {
    const aq_opt = ctx.quest_comp.info.findActiveQuest(quest_id);
    var buf: [32]i32 = undefined;
    for (extractNodeTargetEntityIds(&buf, node)) |eid| {
        if (aq_opt) |aq| {
            if (QuestInfo.hasKilledEntity(aq, eid)) continue;
        }
        if (QuestInfo.containsInt(ctx.quest_comp.info.world.hidden_entities, eid)) continue;
        QuestActions.ensureQuestEntitySpawned(ctx, eid);
    }
}

fn appendJsonIntArray(out: []i32, count: *usize, arr_opt: ?std.json.Value) void {
    const arr = arr_opt orelse return;
    if (arr != .array) return;
    for (arr.array.items) |v| {
        appendUniqueTargetId(out, count, jsonInt(v) orelse 0);
    }
}

fn appendUniqueTargetId(out: []i32, count: *usize, id: i32) void {
    if (id <= 0 or count.* >= out.len) return;
    for (out[0..count.*]) |existing| {
        if (existing == id) return;
    }
    out[count.*] = id;
    count.* += 1;
}

fn tryStartPendingPlayFlowForQuest(ctx: QuestContext, quest_id: i32) !bool {
    const scene = ctx.scene orelse return false;
    const aq = ctx.quest_comp.info.findActiveQuest(quest_id) orelse return false;
    if (aq.active_nodes.len == 0) return false;
    const nodes = ctx.assets.quest_nodes.getOwnerNodes(ctx.io, quest_id) catch return false;

    for (aq.active_nodes) |nid| {
        const node = findNodeById(nodes, nid) orelse continue;
        if (!std.mem.eql(u8, node.node_type, "ChildQuest")) continue;
        const cond = getChildCondition(node.data) orelse continue;
        const ctype = jsonStr(cond.get("Type")) orelse continue;
        if (!std.mem.eql(u8, ctype, "PlayFlow")) continue;

        if (cond.get("Flow")) |flow_val| {
            if (flow_val == .object) {
                try startFlowFromObj(ctx, quest_id, nid, flow_val.object);
                if (quest_id == prologue_quest_id) scene.prologue_flow_started = true;
                return true;
            }
        }
    }
    return false;
}

/// Evaluates `ChildQuest` nodes across all active quests using a shared callback filter.
fn advanceMatchingChildNodes(
    ctx: QuestContext,
    matcher: anytype,
    comptime check_fn: anytype,
) !bool {
    var progressed = false;
    const active_snapshot = try snapshotActiveQuestIds(ctx);

    for (active_snapshot) |qid| {
        const aq = ctx.quest_comp.info.findActiveQuest(qid) orelse continue;
        if (aq.active_nodes.len == 0) continue;
        const nodes = ctx.assets.quest_nodes.getOwnerNodes(ctx.io, qid) catch continue;

        const node_ids = try ctx.alloc.arena.dupe(i32, aq.active_nodes);
        for (node_ids) |nid| {
            const node = findNodeById(nodes, nid) orelse continue;
            if (!std.mem.eql(u8, node.node_type, "ChildQuest")) continue;
            const cond = getChildCondition(node.data) orelse continue;
            const ctype = jsonStr(cond.get("Type")) orelse continue;

            const cur_aq = ctx.quest_comp.info.findActiveQuest(qid) orelse break;
            if (try check_fn(ctx, matcher, qid, cur_aq, nodes, node.*, cond, ctype)) {
                progressed = true;
            }
        }
    }
    return progressed;
}

fn matchMoveNode(
    ctx: QuestContext,
    pos: [3]f32,
    qid: i32,
    _: QuestInfo.ActiveQuest,
    nodes: []const LazyNodeStore.NodeEntry,
    node: LazyNodeStore.NodeEntry,
    cond: std.json.ObjectMap,
    ctype: []const u8,
) !bool {
    if (!std.mem.eql(u8, ctype, "ReachArea")) return false;
    if (!matchesReachArea(ctx, qid, node.data, cond, pos)) return false;
    try completeChildQuest(ctx, qid, nodes, node);
    return true;
}

pub fn onMove(ctx: QuestContext, pos: [3]f32) !void {
    var progressed = try advanceMatchingChildNodes(ctx, pos, matchMoveNode);
    if (try checkActiveQuestFailedMonitors(ctx, pos)) {
        progressed = true;
    }
    if (progressed) {
        try finalizeProgression(ctx);
    }
    if (LevelPlayHelper.fromQuestContext(ctx)) |lp_ctx| {
        try LevelPlayHelper.onPlayerMove(lp_ctx, pos[0], pos[1], pos[2]);
    }
}

const FlowEndReplayOutcome = struct {
    next_state_id: i32 = 0,
    has_finish_do_interact: bool = false,
    finish_do_interact_executed: bool = false,
};

fn isFlowReplayServerAction(name: []const u8) bool {
    const actions = [_][]const u8{
        "SetVar",
        "CalculateVar",
        "RandomVar",
        "AccpetCurrentQuest",
        "OpenSystemFunction",
        "EnableSystem",
        "EnableFunction",
        "UnlockSystemItem",
        "GetItem",
        "DestroyQuestItem",
        "UnlockTeleportTrigger",
        "EnableLevelPlay",
        "ChangeEntityState",
        "ChangeSelfEntityState",
        "AwakeEntity",
        "DestroyEntity",
        "SetEntityVisible",
        "HideGroup",
        "ShowHidedGroup",
        "HideSpecificEntities",
        "ShowSpecificEntities",
        "HideTargetRange",
        "ShowTargetRange",
        "HideByRangeInFlow",
        "ShowAllHidedGroupInFlow",
        "ShowHidedGroupInFlow",
        "LockEntity",
        "UnlockEntity",
        "SetEntityPos",
        "ResetEntityPos",
        "SetPlayerPos",
        "ServerSetPlayerPos",
        "ChangeEntityCamp",
        "MoveSceneItem",
        "MoveWithSpline",
        "NewMoveWithSpline",
        "StopNewMoveWithSpline",
        "NpcMoveToPoint",
        "AddFlowInteractOption",
        "RemoveFlowInteractOption",
        "ModifySceneItemAttributeTag",
        "SetElementalBoxToTarget",
        "ChangeElementalBoxState",
        "ChangeLiftTarget",
        "AddPlayBubble",
        "ClearPlayBubble",
        "RunActions",
        "FinishState",
        "AddTrialCharacter",
        "RemoveTrialCharacter",
        "UpdateTrialCharacter",
        "RestoreTrialCharacter",
        "SheriffUnlockClue",
        "SheriffUnlockProgress",
        "SheriffChangeCriminalState",
        "SheriffChangeCriminalIdentity",
        "SheriffUnlockEnding",
        "SetTime",
        "SetTimeLockState",
        "SetWeather",
        "SetWeatherLockState",
        "SwitchDataLayers",
        "SwitchSubLevels",
        "CompleteGuide",
        "GuideTrigger",
        "SendNpcMail",
        "PhoneSystemInteract",
        "TraceCurrentQuest",
        "DestroyQuest",
        "FinishCondition",
        "ChangeState",
        "ChangeOtherState",
        "SetAudioState",
        "VehicleEnter",
        "VehicleExitPlayer",
        "VehicleExitNpc",
        "TeleportToAndEnterVehicle",
        "AddGuestCharacter",
        "RemoveGuestCharacter",
    };
    for (actions) |n| {
        if (std.mem.eql(u8, n, name)) return true;
    }
    return false;
}

fn scanFlowActionsForFinishDoInteract(items: []const std.json.Value) bool {
    for (items) |act| {
        if (act != .object) continue;
        const name = jsonStr(act.object.get("Name")) orelse continue;
        if (std.mem.eql(u8, name, "FinishDoInteract")) return true;
        if (std.mem.eql(u8, name, "RunActions")) {
            if (act.object.get("Params")) |p| {
                if (p == .object) {
                    if (p.object.get("ActionList")) |al| {
                        if (al == .array and scanFlowActionsForFinishDoInteract(al.array.items)) return true;
                    }
                }
            }
        }
        if (std.mem.eql(u8, name, "ShowTalk")) {
            const params = act.object.get("Params") orelse continue;
            if (params != .object) continue;
            const tis = params.object.get("TalkItems") orelse continue;
            if (tis != .array) continue;
            for (tis.array.items) |ti| {
                if (ti != .object) continue;
                if (ti.object.get("IntroActions")) |iacts| {
                    if (iacts == .array and scanFlowActionsForFinishDoInteract(iacts.array.items)) return true;
                }
                if (ti.object.get("Actions")) |tacts| {
                    if (tacts == .array and scanFlowActionsForFinishDoInteract(tacts.array.items)) return true;
                }
                const opts = ti.object.get("Options") orelse continue;
                if (opts != .array) continue;
                for (opts.array.items) |opt| {
                    if (opt != .object) continue;
                    const oacts = opt.object.get("Actions") orelse continue;
                    if (oacts == .array and scanFlowActionsForFinishDoInteract(oacts.array.items)) return true;
                }
            }
        }
    }
    return false;
}

fn executeSingleFlowReplayAction(
    ctx: QuestContext,
    scene: *Scene,
    qid: i32,
    nid: i32,
    act: std.json.Value,
    outcome: *FlowEndReplayOutcome,
) anyerror!void {
    if (act != .object) return;
    const name = jsonStr(act.object.get("Name")) orelse return;
    const action_id = jsonInt(act.object.get("ActionId")) orelse 0;
    if (action_id > 0 and scene.hasFlowActionExecuted(action_id)) return;
    if (std.mem.eql(u8, name, "FinishDoInteract")) {
        outcome.finish_do_interact_executed = true;
        if (action_id > 0) scene.markFlowActionExecuted(ctx.alloc.gpa, action_id) catch {};
        return;
    }
    if (std.mem.eql(u8, name, "RunActions")) {
        if (act.object.get("Params")) |p| {
            if (p == .object) {
                if (p.object.get("ActionList")) |al| {
                    if (al == .array) {
                        for (al.array.items) |sub_act| {
                            try executeSingleFlowReplayAction(ctx, scene, qid, nid, sub_act, outcome);
                        }
                    }
                }
            }
        }
        if (action_id > 0) scene.markFlowActionExecuted(ctx.alloc.gpa, action_id) catch {};
        return;
    }
    if (!isFlowReplayServerAction(name)) return;
    var cbs = actionCallbacks();
    cbs.is_flow_action = true;
    cbs.defer_flow_transition = true;
    try QuestActions.executeAction(ctx, cbs, qid, nid, act);
    if (action_id > 0) scene.markFlowActionExecuted(ctx.alloc.gpa, action_id) catch {};
}

fn replayFlowStateOnEnd(
    ctx: QuestContext,
    scene: *Scene,
    flow_name: []const u8,
    flow_id: i32,
    state_id: i32,
    qid: i32,
    nid: i32,
    option_infos: []const pb.MapEntry(i32, pb.FlowOptionInfoList),
    is_skip: bool,
) !FlowEndReplayOutcome {
    var outcome: FlowEndReplayOutcome = .{};
    const state_val = (try ctx.assets.flow_states.getFlowState(
        ctx.io,
        ctx.alloc.arena,
        flow_name,
        flow_id,
        state_id,
    )) orelse return outcome;
    if (state_val != .object) return outcome;
    const acts_val = state_val.object.get("Actions") orelse return outcome;
    if (acts_val != .array) return outcome;

    outcome.has_finish_do_interact = scanFlowActionsForFinishDoInteract(acts_val.array.items);
    scene.flow_next_state_id = 0;

    for (acts_val.array.items) |act| {
        if (act != .object) continue;
        const name = jsonStr(act.object.get("Name")) orelse continue;
        if (std.mem.eql(u8, name, "ShowTalk")) {
            const st_aid = jsonInt(act.object.get("ActionId")) orelse 0;
            const params = act.object.get("Params") orelse continue;
            if (params != .object) continue;
            const tis_val = params.object.get("TalkItems") orelse continue;
            if (tis_val != .array) continue;

            for (tis_val.array.items) |ti| {
                if (ti != .object) continue;
                const tid = jsonInt(ti.object.get("Id")) orelse 0;
                const opts_val_opt = ti.object.get("Options");
                const has_options = if (opts_val_opt) |ov| (ov == .array and ov.array.items.len > 0) else false;

                var selected_indices: std.ArrayList(usize) = .empty;
                for (option_infos) |entry| {
                    if (st_aid > 0 and entry.key != 0 and entry.key != st_aid) continue;
                    for (entry.value.OptionIndexList.items) |opt_sel| {
                        if (opt_sel.TalkId == tid) {
                            if (std.math.cast(usize, opt_sel.OptionIndex)) |idx| {
                                selected_indices.append(ctx.alloc.arena, idx) catch {};
                            }
                        }
                    }
                }

                // Linear TalkItems (or selected branched TalkItems, or skipped flows) execute IntroActions + Actions.
                if (!has_options or selected_indices.items.len > 0 or is_skip) {
                    if (ti.object.get("IntroActions")) |iacts| {
                        if (iacts == .array) {
                            for (iacts.array.items) |iact| {
                                try executeSingleFlowReplayAction(ctx, scene, qid, nid, iact, &outcome);
                            }
                        }
                    }
                    if (ti.object.get("Actions")) |tacts| {
                        if (tacts == .array) {
                            for (tacts.array.items) |tact| {
                                try executeSingleFlowReplayAction(ctx, scene, qid, nid, tact, &outcome);
                            }
                        }
                    }
                }

                if (has_options) {
                    const opts_val = opts_val_opt.?;
                    for (selected_indices.items) |idx| {
                        if (idx >= opts_val.array.items.len) continue;
                        const opt_obj = opts_val.array.items[idx];
                        if (opt_obj != .object) continue;
                        if (opt_obj.object.get("HiddenOption")) |ho| {
                            if (ho == .bool and ho.bool) continue;
                        }
                        if (opt_obj.object.get("PreCondition")) |pc_val| {
                            if (!ConditionEvaluator.evalInlineConditionBlock(makeEvalContextForQuest(ctx, qid), pc_val)) {
                                continue;
                            }
                        }
                        const oacts = opt_obj.object.get("Actions") orelse continue;
                        if (oacts == .array) {
                            for (oacts.array.items) |oact| {
                                try executeSingleFlowReplayAction(ctx, scene, qid, nid, oact, &outcome);
                            }
                        }
                    }
                }
            }
            continue;
        }
        try executeSingleFlowReplayAction(ctx, scene, qid, nid, act, &outcome);
    }

    outcome.next_state_id = scene.flow_next_state_id;
    return outcome;
}

pub fn onFlowEnd(
    ctx: QuestContext,
    flow_inc_id: i64,
    option_infos: []const pb.MapEntry(i32, pb.FlowOptionInfoList),
    is_skip: bool,
) !void {
    const scene = ctx.scene orelse return;
    const matched_qid = scene.active_flow_quest_id;
    const matched_nid = scene.active_flow_node_id;
    const matched_bt_type = scene.active_flow_bt_type;
    const matched_inc = scene.active_flow_inc_id;
    const owner_entity_id = scene.active_flow_owner_entity_id;

    if (matched_inc != 0 and (matched_inc == flow_inc_id or flow_inc_id == 0)) {
        const prev_flow_name = if (scene.active_flow_list_name.len > 0)
            try ctx.alloc.arena.dupe(u8, scene.active_flow_list_name)
        else
            "";
        const prev_flow_id = scene.active_flow_id;
        const prev_state_id = scene.active_flow_state_id;
        var replay: FlowEndReplayOutcome = .{};
        if (prev_flow_name.len > 0 and prev_flow_id > 0 and prev_state_id > 0) {
            replay = replayFlowStateOnEnd(
                ctx,
                scene,
                prev_flow_name,
                prev_flow_id,
                prev_state_id,
                matched_qid,
                matched_nid,
                option_infos,
                is_skip,
            ) catch |err| blk: {
                logHookErr("replayFlowStateOnEnd", err);
                break :blk .{};
            };
        }

        scene.clearActiveFlow(ctx.alloc.gpa);
        scene.prologue_flow_started = false;

        if (replay.next_state_id > 0 and prev_flow_name.len > 0 and prev_flow_id > 0) {
            try startFlowParamsBt(
                ctx,
                matched_qid,
                matched_nid,
                matched_bt_type,
                prev_flow_name,
                prev_flow_id,
                replay.next_state_id,
                false,
                false,
            );
            scene.active_flow_owner_entity_id = owner_entity_id;
            return;
        }

        restorePlotMainRole(ctx);

        const interact_blocked_cancel = !is_skip and replay.has_finish_do_interact and !replay.finish_do_interact_executed;
        if (matched_qid != 0 and !interact_blocked_cancel) {
            if (ctx.quest_comp.info.findActiveQuest(matched_qid)) |aq| {
                const nodes = try ctx.assets.quest_nodes.getOwnerNodes(ctx.io, matched_qid);
                if (QuestInfo.isNodeActive(aq, matched_nid)) {
                    if (findNodeById(nodes, matched_nid)) |node| {
                        var ready_to_complete = true;
                        if (getChildCondition(node.data)) |cond| {
                            const ctype = jsonStr(cond.get("Type")) orelse "";
                            if (std.mem.eql(u8, ctype, "DoInteract")) {
                                if (matchDoInteract(cond, owner_entity_id, resolveConfigId(scene, owner_entity_id), 0)) |matched_opt| {
                                    ready_to_complete = try markAndCheckDoInteractComplete(ctx, matched_qid, node.*, cond, owner_entity_id, matched_opt);
                                }
                            }
                        }
                        if (ready_to_complete) {
                            try completeChildQuest(ctx, matched_qid, nodes, node.*);
                        }
                        try finalizeProgression(ctx);
                    }
                } else if (findNodeById(nodes, matched_nid)) |node| {
                    if (std.mem.eql(u8, node.node_type, "Action") or std.mem.eql(u8, node.node_type, "ActionWithResult")) {
                        const akey = actionNodeProgressKey(matched_qid, matched_nid);
                        const total_actions = countNodeActions(node.*);
                        const next_idx = scene.action_node_next_idx.get(akey) orelse total_actions;
                        if (next_idx < total_actions) {
                            try dispatchPostFlowFollowups(ctx, scene);
                            if (scene.active_flow_inc_id == 0 and !scene.hasPendingFlowForNode(matched_qid, matched_nid) and ctx.quest_comp.info.hasActiveQuest(matched_qid)) {
                                try advanceQuestTree(ctx, matched_qid, true);
                                try finalizeProgression(ctx);
                            }
                            return;
                        } else if (scene.hasPendingFlowForNode(matched_qid, matched_nid)) {
                            try dispatchPostFlowFollowups(ctx, scene);
                            return;
                        }
                        _ = scene.action_node_next_idx.remove(akey);
                        try finishNonChildNode(ctx, matched_qid, node.*, true);
                        try advanceQuestTree(ctx, matched_qid, true);
                        try finalizeProgression(ctx);
                    }
                }
            } else if (LevelPlayHelper.fromQuestContext(ctx)) |lp_ctx| {
                try LevelPlayHelper.onFlowEnd(lp_ctx, matched_qid, matched_nid);
            }
        }
        try dispatchPostFlowFollowups(ctx, scene);
        return;
    }

    if (flow_inc_id != 0) {
        if (scene.takeAsyncFlow(flow_inc_id)) |af_val| {
            var af = af_val;
            defer af.deinit(ctx.alloc.gpa);
            if (af.flow_list_name.len > 0 and af.flow_id > 0 and af.state_id > 0) {
                _ = replayFlowStateOnEnd(
                    ctx,
                    scene,
                    af.flow_list_name,
                    af.flow_id,
                    af.state_id,
                    af.quest_id,
                    af.node_id,
                    option_infos,
                    is_skip,
                ) catch |err| logHookErr("replayAsyncFlowStateOnEnd", err);
            }
            return;
        }
    }

    // Ignore stale/out-of-order FlowEndRequest when a different flow is active,
    // when the client finished a non-quest/ambient flow (`flow_inc_id != 0`),
    // or when a normal non-quest NPC interact flow was active.
    if (matched_inc != 0 or flow_inc_id != 0 or (owner_entity_id != 0 and matched_qid == 0)) return;

    // Fallback: complete the active PlayFlow node on tracked quest, or any active quest
    if (try completeFirstActivePlayFlow(ctx, ctx.quest_comp.info.tracked_quest_id)) {
        scene.clearActiveFlow(ctx.alloc.gpa);
        scene.prologue_flow_started = false;
        restorePlotMainRole(ctx);
        try finalizeProgression(ctx);
        try dispatchPostFlowFollowups(ctx, scene);
        return;
    }

    const active_snapshot = try snapshotActiveQuestIds(ctx);
    for (active_snapshot) |qid| {
        if (try completeFirstActivePlayFlow(ctx, qid)) {
            scene.clearActiveFlow(ctx.alloc.gpa);
            scene.prologue_flow_started = false;
            restorePlotMainRole(ctx);
            try finalizeProgression(ctx);
            try dispatchPostFlowFollowups(ctx, scene);
            return;
        }
    }
}

/// Executes a single server-driven action within the scene's active or async flow state.
pub fn onFlowAction(ctx: QuestContext, flow_inc_id: i64, action_id: i32) !void {
    const scene = ctx.scene orelse return;
    if (action_id <= 0) return;

    var target_list_name: []const u8 = scene.active_flow_list_name;
    var target_flow_id: i32 = scene.active_flow_id;
    var target_state_id: i32 = scene.active_flow_state_id;
    var target_quest_id: i32 = scene.active_flow_quest_id;
    var target_node_id: i32 = scene.active_flow_node_id;
    var is_active = true;

    if (scene.active_flow_inc_id == 0 or (flow_inc_id != 0 and scene.active_flow_inc_id != flow_inc_id)) {
        const af = scene.findAsyncFlow(flow_inc_id) orelse return;
        target_list_name = try ctx.alloc.arena.dupe(u8, af.flow_list_name);
        target_flow_id = af.flow_id;
        target_state_id = af.state_id;
        target_quest_id = af.quest_id;
        target_node_id = af.node_id;
        is_active = false;
    } else if (target_list_name.len == 0) {
        return;
    }

    const state_val = (try ctx.assets.flow_states.getFlowState(
        ctx.io,
        ctx.alloc.arena,
        target_list_name,
        target_flow_id,
        target_state_id,
    )) orelse return;

    const action_val = Assets.LazyFlowStore.findActionById(state_val, action_id) orelse return;
    if (is_active) try scene.markFlowActionExecuted(ctx.alloc.gpa, action_id);
    const saved_owner = scene.active_flow_owner_entity_id;
    const prev_state_id = scene.active_flow_state_id;
    const actions = [_]std.json.Value{action_val};
    var cbs = actionCallbacks();
    cbs.is_flow_action = true;
    try QuestActions.executeActionList(ctx, cbs, target_quest_id, target_node_id, &actions);
    if (is_active and scene.active_flow_state_id != prev_state_id and scene.active_flow_owner_entity_id == 0 and saved_owner != 0) {
        scene.active_flow_owner_entity_id = saved_owner;
    }
}

fn dispatchPostFlowFollowups(ctx: QuestContext, scene: *Scene) !void {
    if (scene.active_flow_inc_id != 0) return;

    if (scene.popPendingFlow()) |*pending_val| {
        var pending = pending_val.*;
        defer pending.deinit(ctx.alloc.gpa);
        try startFlowParams(
            ctx,
            pending.quest_id,
            pending.node_id,
            pending.flow_list_name,
            pending.flow_id,
            pending.state_id,
            false,
        );
        if (scene.active_flow_inc_id != 0) return;
    }

    const tracked = ctx.quest_comp.info.tracked_quest_id;
    if (try tryStartPendingPlayFlowForQuest(ctx, tracked)) return;

    const active_snapshot = try snapshotActiveQuestIds(ctx);
    for (active_snapshot) |qid| {
        if (qid == tracked) continue;
        if (try tryStartPendingPlayFlowForQuest(ctx, qid)) return;
    }

    if (scene.active_flow_inc_id != 0) return;
    try reevaluateInRangeTriggers(ctx);
    for (scene.instance.players) |player| {
        if (player.id == scene.player_id) {
            try onMove(ctx, player.location);
            break;
        }
    }
}

fn completeFirstActivePlayFlow(ctx: QuestContext, quest_id: i32) !bool {
    const aq = ctx.quest_comp.info.findActiveQuest(quest_id) orelse return false;
    const nodes = ctx.assets.quest_nodes.getOwnerNodes(ctx.io, quest_id) catch return false;
    const node_ids = try ctx.alloc.arena.dupe(i32, aq.active_nodes);

    for (node_ids) |nid| {
        const node = findNodeById(nodes, nid) orelse continue;
        if (!std.mem.eql(u8, node.node_type, "ChildQuest")) continue;
        const cond = getChildCondition(node.data) orelse continue;
        const ctype = jsonStr(cond.get("Type")) orelse continue;
        if (!std.mem.eql(u8, ctype, "PlayFlow")) continue;

        try completeChildQuest(ctx, quest_id, nodes, node.*);
        return true;
    }
    return false;
}

pub fn onInteract(ctx: QuestContext, entity_id: i64, option_index: i32) !void {
    _ = try onInteractHandled(ctx, entity_id, option_index);
}

pub fn onInteractHandled(ctx: QuestContext, entity_id: i64, option_index: i32) !bool {
    const cfg_id = resolveConfigId(ctx.scene, entity_id);
    var progressed = false;

    // Determine normal InteractComponent option count so normal options at low indices aren't hijacked by DoInteract fallbacks.
    var normal_opts_len: usize = 0;
    if (cfg_id > 0) {
        const inst_id = if (ctx.scene) |s| s.instance_id else 8;
        if (ctx.assets.findLevelEntity(inst_id, cfg_id)) |lcfg| {
            var components = lcfg.ComponentsData;
            if (ctx.assets.findTemplateConfig(lcfg.BlueprintType)) |template| {
                template.ComponentsData.mergeInto(&components);
            }
            if (components.InteractComponent) |ic| {
                if (!(ic.Disabled orelse false)) normal_opts_len = ic.Options.len;
            }
        }
    }

    // 1) Activate on-demand "Interact" quests bound to this entity
    if (ctx.quest_comp.info.hasFinishedQuest(prologue_quest_id)) {
        for (ctx.assets.tables.quest_data.items) |q| {
            const d = q.Data;
            if (!isEligibleQuest(ctx, d, false)) continue;
            if (!std.mem.eql(u8, d.DistributeType, "Interact")) continue;
            if (!matchesQuestInteractStarter(ctx, d, entity_id, cfg_id)) continue;
            if (!ConditionEvaluator.evalQuestProvideConditions(makeEvalContext(ctx), d.ProvideType)) continue;
            if (try activateQuest(ctx, d.Id, true)) {
                progressed = true;
            }
        }
    }

    // 2) Advance active DoInteract / HandInItems ChildQuest nodes matching this entity
    const prefer_normal_opt = option_index >= 0 and @as(usize, @intCast(option_index)) < normal_opts_len;
    const InteractMatcher = struct { entity_id: i64, cfg_id: i32, option_index: i32, allow_fallback: bool };
    const Check = struct {
        fn run(
            c: QuestContext,
            m: InteractMatcher,
            qid: i32,
            _: QuestInfo.ActiveQuest,
            nodes: []const LazyNodeStore.NodeEntry,
            node: LazyNodeStore.NodeEntry,
            cond: std.json.ObjectMap,
            ctype: []const u8,
        ) !bool {
            if (std.mem.eql(u8, ctype, "DoInteract")) {
                if (m.allow_fallback) {
                    if (matchDoInteract(cond, m.entity_id, m.cfg_id, m.option_index)) |matched_opt| {
                        return try handleInteractOption(c, qid, nodes, node, cond, m.entity_id, matched_opt);
                    }
                }
            } else if (std.mem.eql(u8, ctype, "HandInItems")) {
                if (matchesHandInEntity(cond, m.entity_id, m.cfg_id)) {
                    if (try consumeHandInItems(c, cond)) {
                        try completeChildQuest(c, qid, nodes, node);
                        return true;
                    }
                }
            }
            return false;
        }
    };
    if (try advanceMatchingChildNodes(ctx, InteractMatcher{
        .entity_id = entity_id,
        .cfg_id = cfg_id,
        .option_index = option_index,
        .allow_fallback = !prefer_normal_opt,
    }, Check.run)) {
        progressed = true;
    }

    if (progressed) {
        try finalizeProgression(ctx);
        return true;
    } else if (cfg_id > 0 and option_index >= 0) {
        const inst_id = if (ctx.scene) |s| s.instance_id else 8;
        const no_active_flow = if (ctx.scene) |s| s.active_flow_inc_id == 0 else true;
        if (ctx.assets.findLevelEntity(inst_id, cfg_id)) |lcfg| {
            var components = lcfg.ComponentsData;
            if (ctx.assets.findTemplateConfig(lcfg.BlueprintType)) |template| {
                template.ComponentsData.mergeInto(&components);
            }
            if (components.InteractComponent) |ic| {
                const opt_idx: usize = @intCast(option_index);
                if (opt_idx < ic.Options.len) {
                    if (ic.Options[opt_idx].Condition) |opt_cond| {
                        var eval_ctx = makeEvalContext(ctx);
                        eval_ctx.self_entity_config_id = cfg_id;
                        if (!ConditionEvaluator.evalInlineConditionBlock(eval_ctx, opt_cond)) {
                            return false;
                        }
                    }
                    if (ic.Options[opt_idx].Type) |ot| {
                        if (std.mem.eql(u8, ot.Type, "Flow")) {
                            if (no_active_flow) {
                                if (ot.Flow) |fl| {
                                    if (fl.FlowListName.len > 0 and fl.FlowId > 0) {
                                        try startNormalInteractFlow(
                                            ctx,
                                            entity_id,
                                            cfg_id,
                                            option_index,
                                            fl.FlowListName,
                                            fl.FlowId,
                                            if (fl.StateId > 0) fl.StateId else 1,
                                        );
                                        return true;
                                    }
                                }
                            }
                        } else if (std.mem.eql(u8, ot.Type, "Actions")) {
                            try executeNormalInteractActions(ctx, entity_id, cfg_id, option_index, ot.Actions, no_active_flow);
                            return true;
                        }
                    }
                }
            }
        }
    }
    return false;
}

fn executeNormalInteractActions(
    ctx: QuestContext,
    entity_id: i64,
    cfg_id: i32,
    option_index: i32,
    actions: []const @import("../../data/tables/entity_components/Interact.zig").InteractAction,
    no_active_flow: bool,
) !void {
    var callbacks = actionCallbacks();
    callbacks.is_flow_action = false;
    var state_changed = false;
    var started_flow = false;
    for (actions) |act| {
        if (std.mem.eql(u8, act.Name, "PlayFlow")) {
            if (!started_flow and no_active_flow) {
                if (act.Params) |p| {
                    if (p.FlowListName.len > 0 and p.FlowId > 0) {
                        try startNormalInteractFlow(
                            ctx,
                            entity_id,
                            cfg_id,
                            option_index,
                            p.FlowListName,
                            p.FlowId,
                            if (p.StateId > 0) p.StateId else 1,
                        );
                        started_flow = true;
                    }
                }
            }
        } else if (std.mem.eql(u8, act.Name, "ChangeSelfEntityState")) {
            const p = act.Params orelse continue;
            const state_str = if (p.EntityState.len > 0) p.EntityState else p.State;
            if (state_str.len == 0) continue;
            try QuestActions.applyEntityStateChange(ctx, callbacks, cfg_id, state_str);
            state_changed = true;
        } else if (act.Raw) |raw_act| {
            const prev_owner = if (ctx.scene) |scene| blk: {
                const prev = scene.active_flow_owner_entity_id;
                scene.active_flow_owner_entity_id = entity_id;
                break :blk prev;
            } else @as(i64, 0);
            defer if (ctx.scene) |scene| {
                scene.active_flow_owner_entity_id = prev_owner;
            };
            try QuestActions.executeAction(ctx, callbacks, cfg_id, option_index, raw_act);
            state_changed = true;
        } else if (std.mem.eql(u8, act.Name, "ChangeEntityState")) {
            const p = act.Params orelse continue;
            const action_type = if (p.Type.len > 0) p.Type else "Directly";
            if (std.mem.eql(u8, action_type, "BatchDirectly")) {
                if (p.State.len == 0) continue;
                for (p.EntityIds) |eid| {
                    try QuestActions.applyEntityStateChange(ctx, callbacks, eid, p.State);
                    state_changed = true;
                }
            } else if (p.EntityId > 0) {
                if (std.mem.eql(u8, action_type, "Loop")) {
                    const cur = ctx.quest_comp.info.getEntityState(p.EntityId) orelse "";
                    const next_state: []const u8 = if (std.mem.indexOf(u8, cur, "激活") != null)
                        "关卡.Common.状态.常态"
                    else
                        "关卡.Common.状态.激活";
                    try QuestActions.applyEntityStateChange(ctx, callbacks, p.EntityId, next_state);
                    state_changed = true;
                } else if (p.State.len > 0) {
                    try QuestActions.applyEntityStateChange(ctx, callbacks, p.EntityId, p.State);
                    state_changed = true;
                }
            }
        } else if (std.mem.eql(u8, act.Name, "TeleportDungeon")) {
            const p = act.Params orelse continue;
            if (p.DungeonId > 0) {
                try QuestActions.resolveTeleportDungeonAction(
                    ctx,
                    p.DungeonId,
                    if (p.LocationEntityId > 0) p.LocationEntityId else null,
                );
            }
        } else if (std.mem.eql(u8, act.Name, "ExitDungeon")) {
            const pos_eid: i32 = if (act.Params) |p|
                if (p.LocationEntityId > 0) std.math.cast(i32, p.LocationEntityId) orelse 0 else 0
            else
                0;
            try QuestActions.leaveCurrentDungeon(ctx, pos_eid);
        }
    }
    if (state_changed) {
        try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);
    }
}

pub fn onRandomInteract(ctx: QuestContext, entity_id: i64, option_index: i32) !bool {
    const scene = ctx.scene orelse return false;
    const entity_index = scene.net_id_map.get(entity_id) orelse return false;
    const config_id = scene.entities.items(.config)[entity_index].config_id;
    if (config_id <= 0) return false;
    const interact_state = scene.entities.items(.world_item)[entity_index].interact orelse return false;
    if (!containsNodeId(interact_state.random_option_indices, option_index)) return false;

    const level_entity = ctx.assets.findLevelEntity(scene.instance_id, config_id) orelse return false;
    var components = level_entity.ComponentsData;
    if (ctx.assets.findTemplateConfig(level_entity.BlueprintType)) |template| {
        template.ComponentsData.mergeInto(&components);
    }
    const interact = components.InteractComponent orelse return false;
    if (interact.Disabled orelse false) return false;
    const random = interact.RandomInteract orelse return false;
    const index = std.math.cast(usize, option_index) orelse return false;
    if (index >= random.Options.len) return false;
    const option = random.Options[index].Option orelse return false;
    const option_type = option.Type orelse return false;

    if (std.mem.eql(u8, option_type.Type, "Flow")) {
        const flow = option_type.Flow orelse return false;
        if (flow.FlowListName.len == 0 or flow.FlowId <= 0) return false;
        return try startRandomInteractFlow(
            ctx,
            entity_id,
            config_id,
            option_index,
            flow.FlowListName,
            flow.FlowId,
            if (flow.StateId > 0) flow.StateId else 1,
        );
    }

    if (std.mem.eql(u8, option_type.Type, "Actions")) {
        for (option_type.Actions) |action| {
            if (!std.mem.eql(u8, action.Name, "PlayFlow")) continue;
            const params = action.Params orelse continue;
            if (params.FlowListName.len == 0 or params.FlowId <= 0) continue;
            return try startRandomInteractFlow(
                ctx,
                entity_id,
                config_id,
                option_index,
                params.FlowListName,
                params.FlowId,
                if (params.StateId > 0) params.StateId else 1,
            );
        }
    }
    return false;
}

fn startRandomInteractFlow(
    ctx: QuestContext,
    entity_id: i64,
    config_id: i32,
    option_index: i32,
    flow_name: []const u8,
    flow_id: i32,
    state_id: i32,
) !bool {
    const scene = ctx.scene orelse return false;
    if (scene.active_flow_inc_id != 0) return false;

    const flow_inc_id = ctx.quest_comp.info.allocFlowIncId();
    try scene.setActiveFlow(ctx.alloc.gpa, flow_inc_id, 0, 0, flow_name, flow_id, state_id);
    scene.active_flow_owner_entity_id = entity_id;
    prepareFlowStateEntities(ctx, flow_name, flow_id, state_id);
    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.FlowStartNotify{
        .FlowIncId = flow_inc_id,
        .FlowListName = flow_name,
        .FlowId = flow_id,
        .StateId = state_id,
        .GameCtx = .{
            .CtxType = .RandomInteract,
            .CtxInfo = .{ .RandomInteract = .{
                .EntityCtx = .{ .ConfigId = config_id, .IncId = entity_id },
                .OptionIndex = option_index,
            } },
        },
    }) catch |err| {
        scene.clearActiveFlow(ctx.alloc.gpa);
        return err;
    };
    return true;
}

fn startNormalInteractFlow(
    ctx: QuestContext,
    entity_id: i64,
    config_id: i32,
    option_index: i32,
    flow_name: []const u8,
    flow_id: i32,
    state_id: i32,
) !void {
    const scene = ctx.scene orelse return;
    if (scene.active_flow_inc_id != 0) return;

    const flow_inc_id = ctx.quest_comp.info.allocFlowIncId();
    try scene.setActiveFlow(ctx.alloc.gpa, flow_inc_id, 0, 0, flow_name, flow_id, state_id);
    scene.active_flow_owner_entity_id = entity_id;
    prepareFlowStateEntities(ctx, flow_name, flow_id, state_id);
    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.FlowStartNotify{
        .FlowIncId = flow_inc_id,
        .FlowListName = flow_name,
        .FlowId = flow_id,
        .StateId = state_id,
        .GameCtx = .{
            .CtxType = .NormalInteract,
            .CtxInfo = .{ .NormalInteract = .{
                .EntityCtx = .{ .ConfigId = config_id, .IncId = entity_id },
                .OptionIndex = option_index,
            } },
        },
    }) catch |err| {
        scene.clearActiveFlow(ctx.alloc.gpa);
        return err;
    };
}

fn jsonArrayContainsInt(arr_val_opt: ?std.json.Value, needle: i32) bool {
    const arr_val = arr_val_opt orelse return false;
    if (arr_val != .array) return false;
    for (arr_val.array.items) |v| {
        if ((jsonInt(v) orelse 0) == needle) return true;
    }
    return false;
}

fn isEntityKillRelevantToQuest(
    aq: QuestInfo.ActiveQuest,
    nodes: []const LazyNodeStore.NodeEntry,
    cfg_id: i32,
) bool {
    for (aq.active_nodes) |nid| {
        const node = findNodeById(nodes, nid) orelse continue;
        if (!std.mem.eql(u8, node.node_type, "ChildQuest")) continue;
        const cond = getChildCondition(node.data) orelse continue;
        const ctype = jsonStr(cond.get("Type")) orelse continue;
        if (std.mem.eql(u8, ctype, "MonsterCreator")) {
            const list = cond.get("MonsterCreatorEntityIds");
            if (list == null or (list.? == .array and list.?.array.items.len == 0)) return true;
            if (jsonArrayContainsInt(list, cfg_id)) return true;
        } else if (std.mem.eql(u8, ctype, "Kill")) {
            var has_any_list = false;
            inline for (.{ "TargetsToAwake", "ExistTargets" }) |field| {
                if (cond.get(field)) |arr| {
                    if (arr == .array and arr.array.items.len > 0) has_any_list = true;
                }
                if (jsonArrayContainsInt(cond.get(field), cfg_id)) return true;
            }
            if (!has_any_list) return true;
        } else if (std.mem.eql(u8, ctype, "DetectCombatState") or std.mem.eql(u8, ctype, "DetectCombatState2")) {
            if ((jsonInt(cond.get("EntityId")) orelse 0) == cfg_id) return true;
            if (cond.get("Conditions")) |cs| {
                if (cs == .array) {
                    for (cs.array.items) |c| {
                        if (c == .object and (jsonInt(c.object.get("EntityId")) orelse 0) == cfg_id) return true;
                    }
                }
            }
        }
    }
    for (nodes) |node| {
        if (!std.mem.eql(u8, node.node_type, "QuestFailed")) continue;
        if (!isSubtreeActiveForMonitor(nodes, aq.active_nodes, node.parent_id)) continue;
        if (node.data != .object) continue;
        const fc_val = node.data.object.get("FailedCondition") orelse continue;
        if (fc_val != .object) continue;
        const edc_val = fc_val.object.get("EntityDeathCondition") orelse continue;
        if (edc_val != .object) continue;
        if (jsonArrayContainsInt(edc_val.object.get("EntityIds"), cfg_id)) return true;
    }
    return false;
}

pub fn onEntityKilled(ctx: QuestContext, target_net_id: i64, config_id_opt: ?i32) !void {
    const cfg_id = config_id_opt orelse resolveConfigId(ctx.scene, target_net_id);
    if (cfg_id <= 0) return;

    var progressed = false;
    const active_snapshot = try snapshotActiveQuestIds(ctx);

    for (active_snapshot) |qid| {
        const nodes = ctx.assets.quest_nodes.getOwnerNodes(ctx.io, qid) catch continue;
        if (ctx.quest_comp.info.findActiveQuestPtr(qid)) |aq_ptr| {
            if (isEntityKillRelevantToQuest(aq_ptr.*, nodes, cfg_id)) {
                _ = try QuestInfo.recordKilledEntity(aq_ptr, ctx.alloc.gpa, cfg_id);
            }
        }

        if (try checkCombatStateForQuest(ctx, qid, cfg_id, 0.0)) {
            progressed = true;
        }

        const aq_ptr = ctx.quest_comp.info.findActiveQuestPtr(qid) orelse continue;
        const node_ids = try ctx.alloc.arena.dupe(i32, aq_ptr.active_nodes);
        for (node_ids) |nid| {
            const node = findNodeById(nodes, nid) orelse continue;
            if (!std.mem.eql(u8, node.node_type, "ChildQuest")) continue;
            const cond = getChildCondition(node.data) orelse continue;
            const ctype = jsonStr(cond.get("Type")) orelse continue;

            if (std.mem.eql(u8, ctype, "Kill") or std.mem.eql(u8, ctype, "MonsterCreator")) {
                const updated_aq = ctx.quest_comp.info.findActiveQuest(qid) orelse continue;
                if (isKillSatisfied(updated_aq, cond, ctype)) {
                    try completeChildQuest(ctx, qid, nodes, node.*);
                    progressed = true;
                } else {
                    try pushNodeProgressNotify(ctx, updated_aq, node.*);
                }
            }
        }
    }

    if (try checkActiveQuestFailedMonitors(ctx, null)) {
        progressed = true;
    }
    if (try extendFailTimersOnKill(ctx)) {
        progressed = true;
    }

    if (progressed) {
        try finalizeProgression(ctx);
    }
}

pub fn onEntityDamaged(ctx: QuestContext, cfg_id: i32, hp_ratio: f32) !void {
    if (cfg_id <= 0) return;
    var progressed = false;
    const active_snapshot = try snapshotActiveQuestIds(ctx);

    for (active_snapshot) |qid| {
        if (try checkCombatStateForQuest(ctx, qid, cfg_id, hp_ratio)) {
            progressed = true;
        }
    }

    if (try checkActiveQuestFailedMonitors(ctx, null)) {
        progressed = true;
    }

    if (progressed) {
        try finalizeProgression(ctx);
    }
}

fn entityHasStateTag(ctx: QuestContext, target_cfg: i32, state_name: []const u8) bool {
    if (target_cfg <= 0 or state_name.len == 0) return false;
    if (ctx.quest_comp.info.getEntityState(target_cfg)) |saved_state| {
        if (std.mem.eql(u8, saved_state, state_name)) return true;
    }
    const scene = ctx.scene orelse return false;
    const tag_id = gameplay_tags.idFromName(&ctx.assets.tables.gameplay_tag_remap, state_name) catch return false;

    const slice = scene.entities.slice();
    const configs = slice.items(.config);
    const tags = slice.items(.tag);
    for (configs, tags) |cfg, tag_opt| {
        if (cfg.config_id != target_cfg) continue;
        const tag_comp = tag_opt orelse continue;
        if (tag_comp.hasTag(&ctx.assets.tables.gameplay_tag_parent, tag_id)) return true;
    }
    return false;
}

fn combatStateAllowsHpFallback(state_name: []const u8) bool {
    return std.mem.indexOf(u8, state_name, "阶段") != null or
        std.mem.indexOf(u8, state_name, "濒死") != null or
        std.mem.indexOf(u8, state_name, "处决") != null or
        std.mem.indexOf(u8, state_name, "信号") != null or
        std.mem.indexOf(u8, state_name, "瘫痪") != null or
        std.mem.indexOf(u8, state_name, "对策") != null or
        std.mem.indexOf(u8, state_name, "被控") != null or
        std.mem.indexOf(u8, state_name, "破防") != null or
        std.mem.indexOf(u8, state_name, "虚弱") != null or
        std.mem.indexOf(u8, state_name, "狂暴") != null;
}

fn combatStateThreshold(state_name: []const u8, hp_ratio: f32) f32 {
    if (hp_ratio <= 0.0) return 1.0;
    if (std.mem.indexOf(u8, state_name, "三阶段") != null or
        std.mem.indexOf(u8, state_name, "处决") != null)
    {
        return 0.40;
    }
    return 0.75;
}

fn checkCombatStateForQuest(ctx: QuestContext, quest_id: i32, cfg_id: i32, hp_ratio: f32) !bool {
    const aq = ctx.quest_comp.info.findActiveQuest(quest_id) orelse return false;
    if (aq.active_nodes.len == 0) return false;
    const nodes = ctx.assets.quest_nodes.getOwnerNodes(ctx.io, quest_id) catch return false;
    const node_ids = try ctx.alloc.arena.dupe(i32, aq.active_nodes);

    var changed = false;
    for (node_ids) |nid| {
        const node = findNodeById(nodes, nid) orelse continue;
        if (!std.mem.eql(u8, node.node_type, "ChildQuest")) continue;
        const cond = getChildCondition(node.data) orelse continue;
        const ctype = jsonStr(cond.get("Type")) orelse continue;

        if (std.mem.eql(u8, ctype, "DetectCombatState") or std.mem.eql(u8, ctype, "DetectCombatState2")) {
            var matched = false;
            if (cond.get("Conditions")) |cs| {
                if (cs == .array and cs.array.items.len > 0) {
                    const req_cnt = jsonInt(cond.get("Count")) orelse 0;
                    const needed: usize = if (req_cnt <= 0) cs.array.items.len else @intCast(req_cnt);
                    var hit_cnt: usize = 0;
                    for (cs.array.items) |item| {
                        if (item != .object) continue;
                        const te = jsonInt(item.object.get("EntityId")) orelse 0;
                        const st = jsonStr(item.object.get("State")) orelse "";
                        const check_eid = if (te != 0) te else cfg_id;
                        const has_tag = entityHasStateTag(ctx, check_eid, st);
                        const hp_hit = (te == 0 or te == cfg_id) and (hp_ratio <= 0.0 or (combatStateAllowsHpFallback(st) and hp_ratio <= combatStateThreshold(st, hp_ratio)));
                        if (has_tag or hp_hit) {
                            hit_cnt += 1;
                        }
                    }
                    matched = hit_cnt >= needed;
                }
            } else {
                const target_entity = jsonInt(cond.get("EntityId")) orelse 0;
                if (target_entity == cfg_id or target_entity == 0) {
                    const state_name = jsonStr(cond.get("State")) orelse "";
                    const entity_to_check = if (target_entity != 0) target_entity else cfg_id;
                    matched = entityHasStateTag(ctx, entity_to_check, state_name) or
                        hp_ratio <= 0.0 or
                        (combatStateAllowsHpFallback(state_name) and hp_ratio <= combatStateThreshold(state_name, hp_ratio));
                }
            }
            if (matched) {
                try completeChildQuest(ctx, quest_id, nodes, node.*);
                changed = true;
            }
        } else if (std.mem.eql(u8, ctype, "CheckTargetBattleAttribute") or std.mem.eql(u8, ctype, "WaitBattleCondition")) {
            if (isBattleAttrOrWaitSatisfied(ctx, cond, ctype, cfg_id, hp_ratio)) {
                try completeChildQuest(ctx, quest_id, nodes, node.*);
                changed = true;
            }
        }
    }
    return changed;
}

pub fn onGuideFinish(ctx: QuestContext, guide_group_id: i32) !void {
    if (guide_group_id <= 0) return;
    if (try completeMatchingChildQuests(ctx, "Guide", "GuideGroupId", guide_group_id)) {
        try finalizeProgression(ctx);
    }
}

fn skillMatchesGenre(assets: *const Assets, skill_id: i32, genre: i32) bool {
    if (genre == 9 or genre == 10) {
        if (assets.tables.phantom_skill.getDataById(skill_id) != null) return true;
        if (assets.tables.explore_tools.getDataById(skill_id) != null) return true;
        for (assets.tables.phantom_skill.items) |ps| {
            if (ps.Id == skill_id or ps.PhantomSkillId == skill_id) return true;
        }
        return false;
    }
    const sk = assets.tables.skill.getDataById(skill_id) orelse return false;
    return switch (genre) {
        0 => sk.SkillType == 1, // Normal attack
        1 => sk.SkillType == 2, // Heavy / Charge
        2 => sk.SkillType == 3, // Resonance Skill (E)
        3 => sk.SkillType == 4, // Resonance Liberation (Ult)
        4 => sk.SkillType == 5, // QTE / Intro
        5 => sk.SkillType == 7, // Dodge Counter
        6, 7, 11 => sk.SkillType == 6, // Dodge / Extreme Dodge / Air Dodge
        12 => sk.SkillType == 12, // Outro
        else => sk.SkillType == genre,
    };
}

fn matchesUseSkillCondition(assets: *const Assets, cond: std.json.ObjectMap, skill_id: i32) bool {
    const chk_val = cond.get("Check") orelse return false;
    if (chk_val != .object) return false;
    const chk = chk_val.object;
    const chk_type = jsonStr(chk.get("Type")) orelse "";
    if (std.mem.eql(u8, chk_type, "CheckBySkillId")) {
        const wanted = jsonInt(chk.get("SkillId")) orelse return false;
        return wanted > 0 and wanted == skill_id;
    }
    if (std.mem.eql(u8, chk_type, "CheckBySkillGenre")) {
        const genre = jsonInt(chk.get("SkillGenre")) orelse return false;
        return skillMatchesGenre(assets, skill_id, genre);
    }
    if (std.mem.eql(u8, chk_type, "CheckVisionShowId") or std.mem.eql(u8, chk_type, "CheckVisionSummonId")) {
        const vid = jsonInt(chk.get("VisionId")) orelse return false;
        if (vid <= 0) return false;
        if (skill_id == vid) return true;
        if (assets.tables.phantom_skill.getDataById(skill_id)) |ps| {
            return ps.PhantomSkillId == vid or ps.Id == vid;
        }
        return false;
    }
    return false;
}

pub fn onSkillUse(ctx: QuestContext, skill_id: i32) !void {
    if (skill_id <= 0) return;
    const Check = struct {
        fn run(
            c: QuestContext,
            sid: i32,
            qid: i32,
            _: QuestInfo.ActiveQuest,
            nodes: []const LazyNodeStore.NodeEntry,
            node: LazyNodeStore.NodeEntry,
            cond: std.json.ObjectMap,
            ctype: []const u8,
        ) !bool {
            if (!std.mem.eql(u8, ctype, "UseSkill")) return false;
            if (!matchesUseSkillCondition(c.assets, cond, sid)) return false;
            try completeChildQuest(c, qid, nodes, node);
            return true;
        }
    };
    if (try advanceMatchingChildNodes(ctx, skill_id, Check.run)) {
        try finalizeProgression(ctx);
    }
}

pub fn onSkillUnlock(ctx: QuestContext, skill_id: i32) !void {
    if (skill_id <= 0) return;
    if (try completeMatchingChildQuests(ctx, "GetSkill", "SkillId", skill_id)) {
        try finalizeProgression(ctx);
    }
}

pub fn onLevelSequenceMarkReached(ctx: QuestContext, entity_config_id: i32, mark: []const u8) !void {
    if (entity_config_id <= 0 or mark.len == 0) return;
    const Match = struct { entity_id: i32, mark: []const u8 };
    const Check = struct {
        fn run(
            c: QuestContext,
            m: Match,
            qid: i32,
            _: QuestInfo.ActiveQuest,
            nodes: []const LazyNodeStore.NodeEntry,
            node: LazyNodeStore.NodeEntry,
            cond: std.json.ObjectMap,
            ctype: []const u8,
        ) !bool {
            if (!std.mem.eql(u8, ctype, "WaitUntilLevelSequenceReachMark")) return false;
            if ((jsonInt(cond.get("EntityId")) orelse 0) != m.entity_id) return false;
            const required_mark = jsonStr(cond.get("Mark")) orelse return false;
            if (!std.mem.eql(u8, required_mark, m.mark)) return false;
            try completeChildQuest(c, qid, nodes, node);
            return true;
        }
    };
    if (try advanceMatchingChildNodes(ctx, Match{ .entity_id = entity_config_id, .mark = mark }, Check.run)) {
        try finalizeProgression(ctx);
    }
    if (LevelPlayHelper.fromQuestContext(ctx)) |lp_ctx| {
        try LevelPlayHelper.onLevelSequenceMarkReached(lp_ctx, entity_config_id, mark);
    }
}

pub fn onLevelPlayCompleted(ctx: QuestContext, level_play_id: i32) !void {
    if (level_play_id > 0) {
        try ctx.quest_comp.info.markLevelPlayCompleted(ctx.alloc.gpa, level_play_id);
    }

    var progressed = try refreshAvailableQuests(ctx, true);
    const Check = struct {
        fn run(
            c: QuestContext,
            _: void,
            qid: i32,
            _: QuestInfo.ActiveQuest,
            nodes: []const LazyNodeStore.NodeEntry,
            node: LazyNodeStore.NodeEntry,
            cond: std.json.ObjectMap,
            ctype: []const u8,
        ) !bool {
            if (!std.mem.eql(u8, ctype, "CheckLevelPlay")) return false;
            if (!isLevelPlaySatisfied(c, cond)) return false;
            try completeChildQuest(c, qid, nodes, node);
            return true;
        }
    };
    if (try advanceMatchingChildNodes(ctx, {}, Check.run)) {
        progressed = true;
    }

    if (progressed) {
        try finalizeProgression(ctx);
    } else if (level_play_id > 0) {
        try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);
    }
}

pub fn onLevelPlayEnabled(ctx: QuestContext, level_play_id: i32, enabled: bool) !void {
    if (LevelPlayHelper.fromQuestContext(ctx)) |lp_ctx| {
        try LevelPlayHelper.onLevelPlayEnabled(lp_ctx, level_play_id, enabled);
    }
    if (!enabled) return;
    if (try recheckPendingConditions(ctx)) {
        try finalizeProgression(ctx);
    }
}

pub fn onDungeonEvent(ctx: QuestContext, dungeon_id: i32, is_finish: bool) !void {
    if (is_finish and dungeon_id > 0) {
        try ctx.quest_comp.info.markDungeonCompleted(ctx.alloc.gpa, dungeon_id);
        try recordChallengeDungeonFinish(ctx, dungeon_id);
    }

    var progressed = if (is_finish) try refreshAvailableQuests(ctx, true) else false;
    const DungeonEvt = struct { did: i32, finish: bool };
    const Check = struct {
        fn run(
            c: QuestContext,
            params: DungeonEvt,
            qid: i32,
            _: QuestInfo.ActiveQuest,
            nodes: []const LazyNodeStore.NodeEntry,
            node: LazyNodeStore.NodeEntry,
            cond: std.json.ObjectMap,
            ctype: []const u8,
        ) !bool {
            if (!params.finish and std.mem.eql(u8, ctype, "EnterDungeon")) {
                const target = nestedDungeonId(cond, "EnterType");
                if (target == 0 or target == params.did) {
                    try completeChildQuest(c, qid, nodes, node);
                    return true;
                }
            } else if (params.finish and std.mem.eql(u8, ctype, "FinishDungeon")) {
                const target = nestedDungeonId(cond, "Condition");
                if (target == 0 or target == params.did) {
                    try completeChildQuest(c, qid, nodes, node);
                    return true;
                }
            } else if (!params.finish and params.did == 8 and std.mem.eql(u8, ctype, "LeaveDungeon")) {
                try completeChildQuest(c, qid, nodes, node);
                return true;
            }
            return false;
        }
    };
    if (try advanceMatchingChildNodes(ctx, DungeonEvt{ .did = dungeon_id, .finish = is_finish }, Check.run)) {
        progressed = true;
    }

    if (progressed) {
        try finalizeProgression(ctx);
    } else if (is_finish and dungeon_id > 0) {
        try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);
    }
}

fn recordChallengeDungeonFinish(ctx: QuestContext, dungeon_id: i32) !void {
    const pc = ctx.progress_comp orelse return;
    var changed = false;

    for (ctx.assets.tables.lord_gym.items) |gym| {
        if (gym.PlayId != dungeon_id) continue;
        _ = try pc.info.recordLordGymPass(ctx.alloc.gpa, gym.Id, 60);
        changed = true;
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.LordGymLevelPlayResultNotify{
            .IsSuccess = true,
            .PassTime = 60,
            .LordGymPassRecord = .{ .LoadGymId = gym.Id, .PassTime = 60 },
        }) catch {};
    }

    const preferred_tower_id = pc.info.getKeyInt("tower:last_floor_id");
    var matched_tc: ?Assets.DataTables.TowerConfig = null;
    for (ctx.assets.tables.tower_config.items) |tc| {
        if (tc.InstanceId != dungeon_id) continue;
        if (preferred_tower_id > 0 and tc.Id == preferred_tower_id) {
            matched_tc = tc;
            break;
        }
        if (matched_tc == null) matched_tc = tc;
    }
    if (matched_tc) |tc| {
        const limit_s: i64 = if (ctx.assets.tables.instance_dungeon.getDataById(dungeon_id)) |inst|
            (if (inst.AutoLeaveTime > 0) inst.AutoLeaveTime else 300)
        else
            300;
        const start_ms = pc.info.getKeyInt("tower:start_ms");
        const now_ms = std.Io.Clock.real.now(ctx.io).toMilliseconds();
        const elapsed_s: i64 = if (start_ms > 0 and now_ms >= start_ms) @divTrunc(now_ms - start_ms, 1000) else 0;
        const remaining_s: i64 = @max(0, limit_s - elapsed_s);

        var star_count: i32 = 0;
        var star_mask: i32 = 0;
        var star_idx: std.ArrayList(i32) = .empty;
        if (tc.Target.len > 0) {
            for (tc.Target, 0..) |target_s, ti| {
                if (ti >= 3) break;
                if (start_ms <= 0 or remaining_s >= target_s) {
                    star_count += 1;
                    star_mask |= (@as(i32, 1) << @intCast(ti));
                    try star_idx.append(ctx.alloc.arena, @as(i32, @intCast(ti + 1)));
                }
            }
        }
        if (star_count == 0) {
            star_count = 1;
            star_mask = 1;
            try star_idx.append(ctx.alloc.arena, 1);
        }

        try pc.info.recordTowerFloor(ctx.alloc.gpa, tc.Id, tc.Difficulty, tc.AreaNum, star_count, star_mask);
        try pc.info.setKeyInt(ctx.alloc.gpa, "tower:last_floor_id", tc.Id);
        try pc.info.setKeyInt(ctx.alloc.gpa, "tower:last_diff", tc.Difficulty);
        changed = true;
        var formation_roles: std.ArrayList(pb.TowerRolePb) = .empty;
        if (ctx.scene) |scene| {
            if (scene.formation_info.cur_formation >= 0 and
                @as(usize, @intCast(scene.formation_info.cur_formation)) < scene.formation_info.formations.len)
            {
                const form = scene.formation_info.formations[@intCast(scene.formation_info.cur_formation)];
                for (form.roles) |maybe_r| {
                    const r = maybe_r orelse continue;
                    try formation_roles.append(ctx.alloc.arena, .{ .RoleId = r.role_id });
                }
            }
        }
        const floor_pb: pb.TowerFloorPb = .{
            .TowerConfigId = tc.Id,
            .Star = star_count,
            .StarIndex = star_idx,
            .Formation = formation_roles,
        };
        var floors_list: std.ArrayList(pb.TowerFloorPb) = .empty;
        try floors_list.append(ctx.alloc.arena, floor_pb);
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.TowerFloorUpdateNotify{
            .TowerFloors = floors_list,
        }) catch {};
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.TowerEndNotify{
            .Success = true,
            .NeedUpdateSeason = false,
            .CurrentFloorData = floor_pb,
        }) catch {};
    }

    if (changed) {
        try pc.save(ctx.fs, ctx.alloc.arena);
    }
}

pub fn onEntityStateChanged(ctx: QuestContext, entity_config_id: i32, state_name: []const u8) !void {
    const StateEvt = struct { eid: i32, state: []const u8 };
    const Check = struct {
        fn run(
            c: QuestContext,
            ev: StateEvt,
            qid: i32,
            _: QuestInfo.ActiveQuest,
            nodes: []const LazyNodeStore.NodeEntry,
            node: LazyNodeStore.NodeEntry,
            cond: std.json.ObjectMap,
            ctype: []const u8,
        ) !bool {
            if (std.mem.eql(u8, ctype, "CheckEntityState")) {
                if (isCheckEntityStateSatisfied(c, cond)) {
                    try completeChildQuest(c, qid, nodes, node);
                    return true;
                }
            } else if (std.mem.eql(u8, ctype, "ListenToEvent")) {
                if (cond.get("EventType")) |et| {
                    if (et == .object and std.mem.eql(u8, jsonStr(et.object.get("Type")) orelse "", "EntityStateChange")) {
                        const target_id = jsonInt(et.object.get("EntityId")) orelse 0;
                        const want_state = jsonStr(et.object.get("State")) orelse "";
                        if ((target_id == 0 or target_id == ev.eid) and
                            (want_state.len == 0 or std.mem.eql(u8, want_state, ev.state)))
                        {
                            try completeChildQuest(c, qid, nodes, node);
                            return true;
                        }
                    }
                }
            } else if (std.mem.eql(u8, ctype, "DetectCombatState") or
                std.mem.eql(u8, ctype, "DetectCombatState2") or
                std.mem.eql(u8, ctype, "WaitBattleCondition"))
            {
                if (isBattleAttrOrWaitSatisfied(c, cond, ctype, ev.eid, 1.0)) {
                    try completeChildQuest(c, qid, nodes, node);
                    return true;
                }
                const target_st = jsonStr(cond.get("State")) orelse "";
                if (target_st.len > 0 and std.mem.eql(u8, target_st, ev.state)) {
                    const target_id = jsonInt(cond.get("EntityId")) orelse 0;
                    if (target_id == 0 or target_id == ev.eid) {
                        try completeChildQuest(c, qid, nodes, node);
                        return true;
                    }
                }
            }
            return false;
        }
    };

    var progressed = try advanceMatchingChildNodes(ctx, StateEvt{ .eid = entity_config_id, .state = state_name }, Check.run);
    if (try refreshAvailableQuests(ctx, true)) progressed = true;
    if (try recheckPendingConditions(ctx)) progressed = true;
    if (try extendFailTimersOnEntityState(ctx, entity_config_id, state_name)) progressed = true;
    if (try checkActiveQuestFailedMonitors(ctx, null)) progressed = true;
    if (progressed) {
        try finalizeProgression(ctx);
    }
    if (LevelPlayHelper.fromQuestContext(ctx)) |lp_ctx| {
        try LevelPlayHelper.onEntityStateChanged(lp_ctx, entity_config_id, state_name);
    }
}

pub fn onSubmitPhotoNode(ctx: QuestContext, owner_id: i32, node_id: i32) !void {
    if (owner_id > 0 and node_id > 0) {
        _ = try ctx.quest_comp.info.markPhotoNodeCompleted(ctx.alloc.gpa, owner_id, node_id);
    }
    const PhotoEvt = struct { owner: i32, nid: i32 };
    const Check = struct {
        fn run(
            c: QuestContext,
            ev: PhotoEvt,
            qid: i32,
            _: QuestInfo.ActiveQuest,
            nodes: []const LazyNodeStore.NodeEntry,
            node: LazyNodeStore.NodeEntry,
            _: std.json.ObjectMap,
            ctype: []const u8,
        ) !bool {
            if (!std.mem.eql(u8, ctype, "TakePhoto") and !std.mem.eql(u8, ctype, "TakePhoto2")) return false;
            if (ev.owner != 0 and ev.owner != qid) return false;
            if (ev.nid != 0 and ev.nid != node.id) return false;
            try completeChildQuest(c, qid, nodes, node);
            return true;
        }
    };
    if (try advanceMatchingChildNodes(ctx, PhotoEvt{ .owner = owner_id, .nid = node_id }, Check.run)) {
        try finalizeProgression(ctx);
    } else if (owner_id > 0 and node_id > 0) {
        try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);
    }
}

fn allowsClientSubmitNode(data: std.json.Value) bool {
    const cond = getChildCondition(data) orelse return true;
    const ctype = jsonStr(cond.get("Type")) orelse return true;
    return !(std.mem.eql(u8, ctype, "DoInteract") or
        std.mem.eql(u8, ctype, "PlayFlow") or
        std.mem.eql(u8, ctype, "HandInItems") or
        std.mem.eql(u8, ctype, "Kill") or
        std.mem.eql(u8, ctype, "MonsterCreator") or
        std.mem.eql(u8, ctype, "Timer") or
        std.mem.eql(u8, ctype, "WaitTime"));
}

pub fn onSubmitNode(ctx: QuestContext, owner_id: i32, tree_inc_id: i64, node_id: i32) !pb.ErrorCode {
    const quest_id: i32 = if (owner_id > 0)
        owner_id
    else
        std.math.cast(i32, tree_inc_id) orelse return .Success;

    if (quest_id <= 0 or node_id <= 0) return .Success;

    const aq = ctx.quest_comp.info.findActiveQuest(quest_id) orelse return .Success;
    if (!QuestInfo.containsInt(aq.active_nodes, node_id)) return .Success;

    const nodes = ctx.assets.quest_nodes.getOwnerNodes(ctx.io, quest_id) catch return .Success;
    const node = findNodeById(nodes, node_id) orelse return .Success;
    if (!std.mem.eql(u8, node.node_type, "ChildQuest")) return .Success;
    if (!allowsClientSubmitNode(node.data)) return .Success;

    try completeChildQuest(ctx, quest_id, nodes, node.*);
    try finalizeProgression(ctx);
    return .Success;
}

pub fn onUiGamePlayFinish(ctx: QuestContext, gameplay_key: []const u8) !void {
    if (gameplay_key.len == 0) return;
    const Check = struct {
        fn run(
            c: QuestContext,
            key: []const u8,
            qid: i32,
            _: QuestInfo.ActiveQuest,
            nodes: []const LazyNodeStore.NodeEntry,
            node: LazyNodeStore.NodeEntry,
            cond: std.json.ObjectMap,
            ctype: []const u8,
        ) !bool {
            if (!std.mem.eql(u8, ctype, "CheckUiGame")) return false;
            if (!matchesCheckUiGameKey(cond, key)) return false;
            try completeChildQuest(c, qid, nodes, node);
            return true;
        }
    };
    if (try advanceMatchingChildNodes(ctx, gameplay_key, Check.run)) {
        try finalizeProgression(ctx);
    }
}

fn finishConditionFromAction(ctx: QuestContext, quest_id: i32, node_id: i32) !void {
    if (quest_id > 0 and node_id > 0) {
        _ = try onSubmitNode(ctx, quest_id, quest_id, node_id);
    }
}

fn interruptActiveFlowForQuest(ctx: QuestContext, quest_id: i32) ?i64 {
    const scene = ctx.scene orelse return null;
    if (scene.active_flow_inc_id == 0 or scene.active_flow_quest_id != quest_id) return null;
    const ended_inc = scene.active_flow_inc_id;
    scene.clearActiveFlow(ctx.alloc.gpa);
    scene.prologue_flow_started = false;
    restorePlotMainRole(ctx);
    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.FlowEndNotify{
        .FlowIncId = ended_inc,
        .Async = false,
        .IsFinished = false,
    }) catch {};
    return ended_inc;
}

pub fn onBtGiveUp(ctx: QuestContext, owner_id: i32, tree_inc_id: i64, node_id: i32) !void {
    const quest_id = if (owner_id > 0) owner_id else (std.math.cast(i32, tree_inc_id) orelse return);
    var flow_ids: std.ArrayList(i64) = .empty;
    if (ctx.scene) |scene| {
        _ = scene.removeBtTimer(ctx.alloc.gpa, owner_id, tree_inc_id, "");
        if (interruptActiveFlowForQuest(ctx, quest_id)) |ended_inc| {
            try flow_ids.append(ctx.alloc.arena, ended_inc);
        }
    }
    if (quest_id > 0 and node_id > 0) {
        if (ctx.quest_comp.info.findActiveQuest(quest_id)) |aq| {
            if (QuestInfo.isNodeActive(aq, node_id)) {
                ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.UpdateChildQuestNodeStatusNotify{
                    .TreeOwnerId = quest_id,
                    .TreeIncId = quest_id,
                    .NodeId = node_id,
                    .Status = .Fail,
                }) catch {};
            }
        }
    }
    if (quest_id > 0 and ctx.quest_comp.info.hasActiveQuest(quest_id)) {
        const eff_inc_id = if (tree_inc_id != 0) tree_inc_id else @as(i64, quest_id);
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.QuestGiveUpNotify{
            .FlowIncIds = flow_ids,
            .QuestId = quest_id,
            .TreeIncId = eff_inc_id,
        }) catch {};
        try rollbackQuestToNode(ctx, quest_id, node_id, .GiveUpFail, false);
        try finalizeProgression(ctx);
    } else if (quest_id > 0 and QuestInfo.containsInt(ctx.quest_comp.info.progress.entered_level_plays, quest_id)) {
        if (LevelPlayHelper.fromQuestContext(ctx)) |lp_ctx| {
            try LevelPlayHelper.onBtRollback(lp_ctx, quest_id, node_id, .GiveUpFail);
        }
    }
}

pub fn onBtRollback(ctx: QuestContext, owner_id: i32, tree_inc_id: i64, _: i32) !void {
    if (ctx.scene) |scene| {
        _ = scene.removeBtTimer(ctx.alloc.gpa, owner_id, tree_inc_id, "");
    }
    const quest_id = if (owner_id > 0) owner_id else (std.math.cast(i32, tree_inc_id) orelse return);
    if (quest_id > 0 and ctx.quest_comp.info.hasActiveQuest(quest_id)) {
        if (ctx.scene) |scene| {
            if (scene.last_server_rollback_quest_id == quest_id and (nowMs(ctx.io) - scene.last_server_rollback_ms) < 5000) {
                scene.last_server_rollback_quest_id = 0;
                return;
            }
        }
        try rollbackQuestToNode(ctx, quest_id, 0, .DefaultFail, true);
        try finalizeProgression(ctx);
    } else if (quest_id > 0 and QuestInfo.containsInt(ctx.quest_comp.info.progress.entered_level_plays, quest_id)) {
        if (LevelPlayHelper.fromQuestContext(ctx)) |lp_ctx| {
            try LevelPlayHelper.onBtRollback(lp_ctx, quest_id, 0, .DefaultFail);
        }
    }
}

pub fn onForcedOccupation(ctx: QuestContext, tree_inc_id: i64) !void {
    const target_owner = std.math.cast(i32, tree_inc_id) orelse 0;
    if (ctx.scene) |scene| {
        var i: usize = 0;
        while (i < scene.bt_occupations.items.len) {
            const occ = scene.bt_occupations.items[i];
            if (occ.inc_id != tree_inc_id and occ.owner_id != target_owner) {
                var removed = scene.bt_occupations.swapRemove(i);
                removed.deinit(ctx.alloc.gpa);
            } else {
                i += 1;
            }
        }
    }
    if (target_owner > 0 and ctx.quest_comp.info.hasActiveQuest(target_owner)) {
        try advanceQuestTree(ctx, target_owner, true);
        try finalizeProgression(ctx);
    } else if (target_owner > 0 and QuestInfo.containsInt(ctx.quest_comp.info.progress.entered_level_plays, target_owner)) {
        if (LevelPlayHelper.fromQuestContext(ctx)) |lp_ctx| {
            try LevelPlayHelper.stepEnteredTreeById(lp_ctx, target_owner);
        }
    }
}

pub fn executeDynamicInteractOption(ctx: QuestContext, entity_id: i64, option_guid: []const u8) !void {
    _ = try executeDynamicInteractOptionHandled(ctx, entity_id, option_guid);
}

pub fn executeDynamicInteractOptionHandled(ctx: QuestContext, entity_id: i64, option_guid: []const u8) !bool {
    if (option_guid.len == 0) return false;
    const scene = ctx.scene orelse return false;
    const opt = scene.findDynamicInteractOption(option_guid) orelse return false;
    const target_entity_id = if (entity_id != 0) entity_id else opt.entity_net_id;
    switch (opt.kind) {
        .flow => {
            if (opt.flow_list_name.len > 0 and opt.flow_id > 0) {
                try startFlowParams(ctx, opt.quest_id, opt.node_id, opt.flow_list_name, opt.flow_id, opt.state_id, true);
                if (scene.active_flow_inc_id != 0 and scene.active_flow_quest_id == opt.quest_id and scene.active_flow_node_id == opt.node_id) {
                    scene.active_flow_owner_entity_id = target_entity_id;
                }
                return true;
            } else if (opt.bt_type == .BtTypeLevelPlay and opt.quest_id > 0 and opt.node_id > 0) {
                if (LevelPlayHelper.fromQuestContext(ctx)) |lp_ctx| {
                    if (try LevelPlayHelper.onDynamicInteract(lp_ctx, opt.quest_id, opt.node_id, target_entity_id)) return true;
                }
            }
            return try onInteractHandled(ctx, target_entity_id, 0);
        },
        .actions => {
            if (opt.actions_json.len > 0) {
                if (std.json.parseFromSlice(std.json.Value, ctx.alloc.arena, opt.actions_json, .{})) |parsed| {
                    if (parsed.value == .array and parsed.value.array.items.len > 0) {
                        const prev_owner = scene.active_flow_owner_entity_id;
                        scene.active_flow_owner_entity_id = target_entity_id;
                        defer scene.active_flow_owner_entity_id = prev_owner;
                        try executeActions(ctx, opt.quest_id, opt.node_id, parsed.value.array.items);
                    }
                } else |_| {}
            }
            if (opt.bt_type == .BtTypeLevelPlay and opt.quest_id > 0 and opt.node_id > 0) {
                if (LevelPlayHelper.fromQuestContext(ctx)) |lp_ctx| {
                    _ = try LevelPlayHelper.onDynamicInteract(lp_ctx, opt.quest_id, opt.node_id, target_entity_id);
                }
                return true;
            } else if (opt.quest_id > 0 and opt.node_id > 0) {
                _ = try onSubmitNode(ctx, opt.quest_id, opt.quest_id, opt.node_id);
                return true;
            }
            return try onInteractHandled(ctx, target_entity_id, 0);
        },
        .none => {
            if (opt.bt_type == .BtTypeLevelPlay and opt.quest_id > 0 and opt.node_id > 0) {
                if (LevelPlayHelper.fromQuestContext(ctx)) |lp_ctx| {
                    _ = try LevelPlayHelper.onDynamicInteract(lp_ctx, opt.quest_id, opt.node_id, target_entity_id);
                }
                return true;
            } else if (opt.quest_id > 0 and opt.node_id > 0) {
                _ = try onSubmitNode(ctx, opt.quest_id, opt.quest_id, opt.node_id);
                return true;
            }
            return try onInteractHandled(ctx, target_entity_id, 0);
        },
    }
}

pub fn onUiAction(ctx: QuestContext, ui_type: []const u8) !void {
    if (ui_type.len == 0) return;
    const Check = struct {
        fn run(
            c: QuestContext,
            ut_want: []const u8,
            qid: i32,
            _: QuestInfo.ActiveQuest,
            nodes: []const LazyNodeStore.NodeEntry,
            node: LazyNodeStore.NodeEntry,
            cond: std.json.ObjectMap,
            ctype: []const u8,
        ) !bool {
            if (!std.mem.eql(u8, ctype, "ShowUi")) return false;
            const ut = cond.get("UiType") orelse return false;
            if (ut != .object) return false;
            const utype = jsonStr(ut.object.get("Type")) orelse return false;
            if (!std.mem.eql(u8, utype, ut_want)) return false;
            try completeChildQuest(c, qid, nodes, node);
            return true;
        }
    };
    if (try advanceMatchingChildNodes(ctx, ui_type, Check.run)) {
        try finalizeProgression(ctx);
    }
}

pub fn onVisionSystemEvent(ctx: QuestContext, quest_type: []const u8) !void {
    if (quest_type.len == 0) return;
    if (ctx.progress_comp) |pc| {
        if (std.mem.eql(u8, quest_type, "VisionSetActivated")) {
            _ = try pc.info.markClaim(ctx.alloc.gpa, "vision:activated");
        } else if (std.mem.eql(u8, quest_type, "DoVisionIdentify")) {
            _ = try pc.info.markClaim(ctx.alloc.gpa, "vision:identified");
        }
        try pc.save(ctx.fs, ctx.alloc.arena);
    }
    const Check = struct {
        fn run(
            c: QuestContext,
            q_want: []const u8,
            qid: i32,
            _: QuestInfo.ActiveQuest,
            nodes: []const LazyNodeStore.NodeEntry,
            node: LazyNodeStore.NodeEntry,
            cond: std.json.ObjectMap,
            ctype: []const u8,
        ) !bool {
            if (!std.mem.eql(u8, ctype, "VisionSystem")) return false;
            const qv = cond.get("Quest") orelse return false;
            if (qv != .object) return false;
            const qt = jsonStr(qv.object.get("Type")) orelse return false;
            if (!std.mem.eql(u8, qt, q_want)) return false;
            try completeChildQuest(c, qid, nodes, node);
            return true;
        }
    };
    if (try advanceMatchingChildNodes(ctx, quest_type, Check.run)) {
        try finalizeProgression(ctx);
    }
}

pub fn onItemUse(ctx: QuestContext, item_id: i32, count: i32) !void {
    if (item_id <= 0) return;
    const use_delta: i32 = @max(1, count);
    const UseMatch = struct { iid: i32, delta: i32 };
    const Check = struct {
        fn run(
            c: QuestContext,
            m: UseMatch,
            qid: i32,
            aq: QuestInfo.ActiveQuest,
            nodes: []const LazyNodeStore.NodeEntry,
            node: LazyNodeStore.NodeEntry,
            cond: std.json.ObjectMap,
            ctype: []const u8,
        ) !bool {
            if (!std.mem.eql(u8, ctype, "UseItem")) return false;
            const wanted = jsonInt(cond.get("ItemId")) orelse 0;
            if (wanted != 0 and wanted != m.iid) return false;
            const req_count: i32 = @max(1, jsonInt(cond.get("Count")) orelse 1);
            var total_used: i32 = m.delta;
            if (c.progress_comp) |pc| {
                var kbuf: [64]u8 = undefined;
                const key = std.fmt.bufPrint(&kbuf, "use_item:{d}:{d}:{d}", .{ qid, node.id, m.iid }) catch return false;
                const updated = try pc.info.addKeyInt(c.alloc.gpa, key, m.delta);
                total_used = @intCast(std.math.clamp(updated, 0, std.math.maxInt(i32)));
                try pc.save(c.fs, c.alloc.arena);
            }
            if (total_used >= req_count) {
                try completeChildQuest(c, qid, nodes, node);
                return true;
            }
            try pushNodeProgressNotify(c, aq, node);
            return false;
        }
    };

    var progressed = try advanceMatchingChildNodes(ctx, UseMatch{ .iid = item_id, .delta = use_delta }, Check.run);
    if (ctx.quest_comp.info.hasFinishedQuest(prologue_quest_id)) {
        for (ctx.assets.tables.quest_data.items) |q| {
            const d = q.Data;
            if (!isEligibleQuest(ctx, d, false)) continue;
            if (!std.mem.eql(u8, d.DistributeType, "UseItem")) continue;
            if (ConditionEvaluator.evalQuestProvideConditions(makeEvalContext(ctx), d.ProvideType)) {
                if (try activateQuest(ctx, d.Id, true)) progressed = true;
            }
        }
    }
    if (try checkActiveQuestFailedMonitors(ctx, null)) {
        progressed = true;
    }

    if (progressed) {
        try finalizeProgression(ctx);
    }
}

fn syncMoraleOnSceneLoad(ctx: QuestContext) !void {
    const active_play: i64 = if (ctx.progress_comp) |pc| pc.info.getKeyInt("morale:active_play") else 0;
    const play_1_on: i64 = if (ctx.progress_comp) |pc| pc.info.getKeyInt("morale:play:1") else 0;
    const cfg = MoraleHelper.resolvePlayConfig(ctx.assets, if (active_play > 0 and active_play <= std.math.maxInt(i32)) @intCast(active_play) else 1);
    const keep_exp = ctx.inventory_comp.info.normalItemCount(cfg.keep_item_id);
    const temp_exp = ctx.inventory_comp.info.normalItemCount(cfg.temp_item_id);
    if (active_play <= 0 and play_1_on <= 0 and keep_exp <= 0 and temp_exp <= 0) return;

    _ = try MoraleHelper.syncFromInventory(
        ctx.alloc,
        ctx.fs,
        ctx.io,
        ctx.conn,
        ctx.assets,
        ctx.basic_comp,
        ctx.inventory_comp,
        ctx.progress_comp,
        cfg.play_id,
        null,
        true,
    );
}

pub fn onItemGained(ctx: QuestContext, item_id: i32) !void {
    const Check = struct {
        fn run(
            c: QuestContext,
            _: void,
            qid: i32,
            aq: QuestInfo.ActiveQuest,
            nodes: []const LazyNodeStore.NodeEntry,
            node: LazyNodeStore.NodeEntry,
            cond: std.json.ObjectMap,
            ctype: []const u8,
        ) !bool {
            if (!std.mem.eql(u8, ctype, "GetItem")) return false;
            if (hasAllItems(c, cond.get("Items"))) {
                try completeChildQuest(c, qid, nodes, node);
                return true;
            }
            try pushNodeProgressNotify(c, aq, node);
            return false;
        }
    };
    var progressed = try advanceMatchingChildNodes(ctx, {}, Check.run);
    const cfg = MoraleHelper.resolvePlayConfig(ctx.assets, 1);
    if (item_id == cfg.keep_item_id or item_id == cfg.temp_item_id or
        item_id == MoraleHelper.default_keep_item_id or item_id == MoraleHelper.default_temp_item_id)
    {
        _ = try MoraleHelper.syncFromInventory(
            ctx.alloc,
            ctx.fs,
            ctx.io,
            ctx.conn,
            ctx.assets,
            ctx.basic_comp,
            ctx.inventory_comp,
            ctx.progress_comp,
            null,
            null,
            true,
        );
        try onConditionStateChanged(ctx);
    }
    if (try refreshAvailableQuests(ctx, true)) progressed = true;
    if (try checkActiveQuestFailedMonitors(ctx, null)) {
        progressed = true;
    }

    if (progressed) {
        try finalizeProgression(ctx);
    }
}

pub fn onMailRead(ctx: QuestContext, mail_id_str: []const u8) !void {
    const mid = std.fmt.parseInt(i32, mail_id_str, 10) catch 0;
    if (mid <= 0) return;
    _ = try QuestInfo.appendUniqueInt(&ctx.quest_comp.info.progress.read_mails, ctx.alloc.gpa, mid);
    if (try completeMatchingChildQuests(ctx, "ReadMail", "MailId", mid)) {
        try finalizeProgression(ctx);
    } else {
        try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);
    }
}

pub fn onPhoneMessageEvent(ctx: QuestContext, phone_msg_id: i32, telecom_id: i32) !void {
    if (phone_msg_id > 0) {
        _ = try QuestInfo.appendUniqueInt(&ctx.quest_comp.info.progress.read_phone_messages, ctx.alloc.gpa, phone_msg_id);
    }
    if (telecom_id > 0) {
        _ = try QuestInfo.appendUniqueInt(&ctx.quest_comp.info.progress.received_telecoms, ctx.alloc.gpa, telecom_id);
    }

    var progressed = false;
    if (phone_msg_id > 0 and try completeMatchingChildQuests(ctx, "ReadPhoneMessage", "PhoneMessageId", phone_msg_id)) {
        progressed = true;
    }
    if (telecom_id > 0 and try completeMatchingChildQuests(ctx, "ReceiveTelecom", "TelecomId", telecom_id)) {
        progressed = true;
    }
    if (ctx.quest_comp.info.hasFinishedQuest(prologue_quest_id)) {
        for (ctx.assets.tables.quest_data.items) |q| {
            const d = q.Data;
            if (!isEligibleQuest(ctx, d, false)) continue;
            if (!std.mem.eql(u8, d.DistributeType, "PhoneMessage")) continue;
            if (ConditionEvaluator.evalQuestProvideConditions(makeEvalContext(ctx), d.ProvideType)) {
                if (try activateQuest(ctx, d.Id, true)) progressed = true;
            }
        }
    }
    if (progressed) {
        try finalizeProgression(ctx);
    } else {
        try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);
    }
}

pub fn onNpcMailSent(ctx: QuestContext, mail_id: i32) !void {
    _ = mail_id;
    if (try refreshAvailableQuests(ctx, true)) {
        try finalizeProgression(ctx);
    }
}

threadlocal var time_change_depth: u32 = 0;
pub fn onTimeChanged(ctx: QuestContext, hour: i32, minute: i32) !void {
    if (time_change_depth >= 2) return;
    time_change_depth += 1;
    defer time_change_depth -= 1;

    ctx.quest_comp.info.env.time_lock_hour = hour;
    ctx.quest_comp.info.env.time_lock_minute = minute;

    const TimeEvt = struct { hour: i32, minute: i32, day: u32 };
    const Check = struct {
        fn run(
            c: QuestContext,
            ev: TimeEvt,
            qid: i32,
            _: QuestInfo.ActiveQuest,
            nodes: []const LazyNodeStore.NodeEntry,
            node: LazyNodeStore.NodeEntry,
            cond: std.json.ObjectMap,
            ctype: []const u8,
        ) !bool {
            if (std.mem.eql(u8, ctype, "ScheduleTime")) {
                if (isScheduleTimeWindowMatch(cond, ev.hour, ev.minute, ev.day)) {
                    try completeChildQuest(c, qid, nodes, node);
                    return true;
                }
            }
            return false;
        }
    };
    const cur_day: u32 = if (ctx.scene) |s| s.instance.map_time.day else 1;
    var progressed = try advanceMatchingChildNodes(ctx, TimeEvt{ .hour = hour, .minute = minute, .day = cur_day }, Check.run);
    if (try recheckPendingConditions(ctx)) progressed = true;
    if (progressed) {
        try finalizeProgression(ctx);
    } else {
        try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);
    }
}

pub fn onWeatherChanged(ctx: QuestContext, weather_id: i32) !void {
    _ = weather_id;
    const Check = struct {
        fn run(
            c: QuestContext,
            _: void,
            qid: i32,
            _: QuestInfo.ActiveQuest,
            nodes: []const LazyNodeStore.NodeEntry,
            node: LazyNodeStore.NodeEntry,
            _: std.json.ObjectMap,
            ctype: []const u8,
        ) !bool {
            if (!std.mem.eql(u8, ctype, "UseWeatherSwitch")) return false;
            try completeChildQuest(c, qid, nodes, node);
            return true;
        }
    };
    var progressed = try advanceMatchingChildNodes(ctx, {}, Check.run);
    if (try recheckPendingConditions(ctx)) progressed = true;
    if (try checkActiveQuestFailedMonitors(ctx, null)) {
        progressed = true;
    }

    if (progressed) {
        try finalizeProgression(ctx);
    }
}

pub fn onBvbFinish(ctx: QuestContext, bvb_id: i32) !void {
    if (bvb_id > 0) {
        _ = try QuestInfo.appendUniqueInt(&ctx.quest_comp.info.progress.completed_bvb, ctx.alloc.gpa, bvb_id);
    }
    if (try completeMatchingChildQuests(ctx, "FinishBvbChallenge", "BvbChallengeId", bvb_id)) {
        try finalizeProgression(ctx);
    } else {
        try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);
    }
}

pub fn onBtSetTimerInfo(
    ctx: QuestContext,
    owner_id: i32,
    inc_id: i64,
    node_id: i32,
    timer_type: []const u8,
    second: i32,
    set_type: pb.TimerSetType,
) !void {
    const scene = ctx.scene orelse return;
    const now_ms = nowMs(ctx.io);
    const bt = try scene.adjustBtTimer(ctx.alloc.gpa, owner_id, inc_id, node_id, timer_type, now_ms, second, set_type);
    try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.UpdateTimerInfoNotify{
        .TreeOwnerId = owner_id,
        .TreeIncId = inc_id,
        .TimerInfo = .{
            .TimerType = bt.timer_type,
            .NodeId = bt.node_id,
            .EndTime = @divTrunc(bt.end_time_ms, std.time.ms_per_s),
            .PauseTime = bt.pause_time_ms,
        },
    });
}

pub fn onBtTimerEnd(ctx: QuestContext, owner_id: i32, inc_id: i64, timer_type: []const u8) !void {
    if (ctx.scene) |scene| {
        _ = scene.removeBtTimer(ctx.alloc.gpa, owner_id, inc_id, timer_type);
    }
    if (owner_id > 0) {
        if (ctx.quest_comp.info.findActiveQuest(owner_id)) |aq| {
            const nodes = ctx.assets.quest_nodes.getOwnerNodes(ctx.io, owner_id) catch return;
            const node_ids = try ctx.alloc.arena.dupe(i32, aq.active_nodes);
            var progressed = false;
            for (node_ids) |nid| {
                const node = findNodeById(nodes, nid) orelse continue;
                if (!std.mem.eql(u8, node.node_type, "ChildQuest")) continue;
                const cond = getChildCondition(node.data) orelse continue;
                const ctype = jsonStr(cond.get("Type")) orelse continue;
                if (!std.mem.eql(u8, ctype, "Timer") and !std.mem.eql(u8, ctype, "WaitTime")) continue;
                try completeChildQuest(ctx, owner_id, nodes, node.*);
                progressed = true;
                break;
            }
            if (progressed) try finalizeProgression(ctx);
        } else if (QuestInfo.containsInt(ctx.quest_comp.info.progress.entered_level_plays, owner_id)) {
            if (LevelPlayHelper.fromQuestContext(ctx)) |lp_ctx| {
                _ = try LevelPlayHelper.onBtTimerEnd(lp_ctx, owner_id);
            }
        }
    }
}

pub fn buildQuestBranchInfo(ctx: QuestContext) !pb.QuestBranchInfo {
    var pages: std.ArrayList(pb.OneQuestBranchPageInfo) = .empty;
    for (ctx.quest_comp.info.progress.quest_branches) |rec| {
        var completed: std.ArrayList(i32) = .empty;
        for (rec.complete_branches) |bid| {
            try completed.append(ctx.alloc.arena, bid);
        }
        try pages.append(ctx.alloc.arena, .{
            .id = rec.page_id,
            .CurBranch = rec.cur_branch,
            .CompleteBranches = completed,
        });
    }
    return .{ .QuestBranchInfos = pages };
}

pub fn selectQuestBranch(ctx: QuestContext, page_id: i32, branch_id: i32) !pb.OneQuestBranchPageInfo {
    const rec = try ctx.quest_comp.info.selectQuestBranch(ctx.alloc.gpa, page_id, branch_id);
    try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);
    var completed: std.ArrayList(i32) = .empty;
    for (rec.complete_branches) |bid| {
        try completed.append(ctx.alloc.arena, bid);
    }
    const page_pb: pb.OneQuestBranchPageInfo = .{
        .id = rec.page_id,
        .CurBranch = rec.cur_branch,
        .CompleteBranches = completed,
    };
    const branch_info = try buildQuestBranchInfo(ctx);
    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.QuestBranchUpdateNotify{
        .QuestBranchInfo = branch_info,
        .ErrorCode = .Success,
    }) catch {};
    return page_pb;
}

pub fn setFocusDeterCond(ctx: QuestContext, deter: bool) !void {
    ctx.quest_comp.info.progress.focus_deter_cond = deter;
    try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);
}

pub fn acceptQuestById(ctx: QuestContext, quest_id: i32) !void {
    if (quest_id <= 0) return;
    if (ctx.quest_comp.info.hasActiveQuest(quest_id) or ctx.quest_comp.info.hasFinishedQuest(quest_id)) return;
    if (ctx.assets.tables.quest_data.getDataById(quest_id)) |q| {
        if (isEligibleQuest(ctx, q.Data, false)) {
            _ = try activateQuest(ctx, quest_id, true);
            try finalizeProgression(ctx);
        }
    }
}

pub fn acceptFocusWaitQuest(ctx: QuestContext, quest_id: i32) !void {
    if (quest_id <= 0) return;
    if (ctx.assets.tables.quest_data.getDataById(quest_id)) |q| {
        if (isEligibleQuest(ctx, q.Data, false)) {
            _ = try activateQuest(ctx, quest_id, true);
        }
    }
    if (ctx.quest_comp.info.hasActiveQuest(quest_id)) {
        ctx.quest_comp.info.focus_quest_id = quest_id;
        ctx.quest_comp.info.tracked_quest_id = quest_id;
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.FocusQuestSetNotify{
            .FocusQuestId = quest_id,
            .Reason = .Request,
        }) catch {};
        try finalizeProgression(ctx);
    }
}

fn acceptFocusWaitQuestAction(ctx: QuestContext) !void {
    if (ctx.quest_comp.info.focus_quest_id > 0) {
        try acceptFocusWaitQuest(ctx, ctx.quest_comp.info.focus_quest_id);
    }
}

fn finishFlowStateFromAction(ctx: QuestContext) !void {
    if (try recheckPendingConditions(ctx)) {
        try finalizeProgression(ctx);
    }
}

pub fn destroyQuest(ctx: QuestContext, quest_id: i32) !void {
    if (quest_id <= 0) return;
    if (ctx.scene) |scene| {
        scene.quest_timers.cancelQuest(quest_id);
        scene.fail_timers.cancelQuest(quest_id);
        scene.releaseOccupationsForTree(ctx.alloc.gpa, quest_id, quest_id);
        scene.clearSaveNodesForOwner(quest_id);
        scene.clearActionNodeProgressForOwner(quest_id);
        _ = interruptActiveFlowForQuest(ctx, quest_id);
    }
    if (!ctx.quest_comp.info.hasActiveQuest(quest_id)) return;
    try ctx.quest_comp.info.removeActiveQuest(ctx.alloc.gpa, quest_id);

    var dq_list: std.ArrayList(i32) = .empty;
    try dq_list.append(ctx.alloc.arena, quest_id);
    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.DestroyQuestsNotify{ .QuestIds = dq_list }) catch {};

    var ql: pb.QuestListNotify = .default;
    try ql.Quests.append(ctx.alloc.arena, .{ .QuestId = quest_id, .Status = .Delete });
    ctx.conn.push(ctx.alloc.gpa, ctx.io, ql) catch {};

    try finalizeProgression(ctx);
}

fn resolveGmTargetQuestId(ctx: QuestContext, quest_id_opt: ?i32) i32 {
    if (quest_id_opt) |qid| {
        if (qid > 0 and ctx.quest_comp.info.hasActiveQuest(qid)) return qid;
    }
    const tracked = ctx.quest_comp.info.tracked_quest_id;
    if (tracked > 0 and ctx.quest_comp.info.hasActiveQuest(tracked)) return tracked;
    if (ctx.quest_comp.info.active_quests.len > 0) return ctx.quest_comp.info.active_quests[0].quest_id;
    return 0;
}

pub fn gmPickQuest(ctx: QuestContext, quest_id: i32, force_replay: bool) !bool {
    if (quest_id <= 0) return false;
    const exists = ctx.assets.tables.quest_data.getDataById(quest_id) != null or ctx.assets.quest_nodes.hasOwner(quest_id);
    if (!exists) return false;

    if (force_replay) {
        _ = try QuestInfo.removeIntValue(&ctx.quest_comp.info.finished_quests, ctx.alloc.gpa, quest_id);
        if (ctx.quest_comp.info.hasActiveQuest(quest_id)) {
            try destroyQuest(ctx, quest_id);
        }
    }

    if (!ctx.quest_comp.info.hasActiveQuest(quest_id) and !ctx.quest_comp.info.hasFinishedQuest(quest_id)) {
        _ = try activateQuest(ctx, quest_id, true);
    }
    if (ctx.quest_comp.info.hasActiveQuest(quest_id)) {
        ctx.quest_comp.info.tracked_quest_id = quest_id;
        if (ctx.scene) |scene| scene.current_quest_id = quest_id;
    }

    try pushQuestStateNotifies(ctx);
    ensureActiveQuestTargetsSpawned(ctx);
    try finalizeProgression(ctx);
    return true;
}

pub fn gmSkipQuestNode(ctx: QuestContext, quest_id_opt: ?i32) !i32 {
    const quest_id = resolveGmTargetQuestId(ctx, quest_id_opt);
    if (quest_id <= 0) return 0;

    const aq = ctx.quest_comp.info.findActiveQuest(quest_id) orelse return 0;
    const nodes = ctx.assets.quest_nodes.getOwnerNodes(ctx.io, quest_id) catch &[_]LazyNodeStore.NodeEntry{};
    if (nodes.len == 0 or aq.active_nodes.len == 0) {
        try succeedQuest(ctx, quest_id);
        try pushQuestStateNotifies(ctx);
        ensureActiveQuestTargetsSpawned(ctx);
        try finalizeProgression(ctx);
        return quest_id;
    }

    const active_snap = try ctx.alloc.arena.dupe(i32, aq.active_nodes);
    for (active_snap) |nid| {
        const node = findNodeById(nodes, nid) orelse continue;
        if (std.mem.eql(u8, node.node_type, "ChildQuest")) {
            try completeChildQuestWithoutAdvance(ctx, quest_id, node.*, true);
        } else {
            try finishNonChildNode(ctx, quest_id, node.*, true);
        }
    }

    if (ctx.quest_comp.info.hasActiveQuest(quest_id)) {
        try advanceQuestTree(ctx, quest_id, true);
    }

    try pushQuestStateNotifies(ctx);
    ensureActiveQuestTargetsSpawned(ctx);
    try finalizeProgression(ctx);
    return quest_id;
}

pub fn gmSkipQuest(ctx: QuestContext, quest_id_opt: ?i32) !i32 {
    const quest_id = if (quest_id_opt) |qid|
        if (qid > 0) qid else resolveGmTargetQuestId(ctx, null)
    else
        resolveGmTargetQuestId(ctx, null);
    if (quest_id <= 0) return 0;

    const exists = ctx.assets.tables.quest_data.getDataById(quest_id) != null or
        ctx.assets.quest_nodes.hasOwner(quest_id) or
        ctx.quest_comp.info.hasActiveQuest(quest_id) or
        ctx.quest_comp.info.hasFinishedQuest(quest_id);
    if (!exists) return 0;

    if (!ctx.quest_comp.info.hasActiveQuest(quest_id) and !ctx.quest_comp.info.hasFinishedQuest(quest_id)) {
        _ = try ctx.quest_comp.info.getOrAddActiveQuest(ctx.alloc.gpa, quest_id);
        if (ctx.assets.tables.quest_data.getDataById(quest_id)) |q| {
            try executeActions(ctx, quest_id, 0, q.Data.ActiveActions);
            try executeActions(ctx, quest_id, 0, q.Data.AcceptActions);
        }
    }

    if (ctx.quest_comp.info.hasActiveQuest(quest_id)) {
        if (ctx.quest_comp.info.findActiveQuest(quest_id)) |aq| {
            const active_snap = try ctx.alloc.arena.dupe(i32, aq.active_nodes);
            for (active_snap) |nid| {
                if (ctx.quest_comp.info.findActiveQuestPtr(quest_id)) |aq_ptr| {
                    _ = try ctx.quest_comp.info.markNodeFinished(aq_ptr, ctx.alloc.gpa, nid);
                    try pushNodeStatus(ctx, quest_id, nid, .CompletedSuccess, .Finished);
                }
            }
        }
        try succeedQuest(ctx, quest_id);
    }

    try pushQuestStateNotifies(ctx);
    ensureActiveQuestTargetsSpawned(ctx);
    try finalizeProgression(ctx);
    return quest_id;
}

pub fn claimDailyRewards(ctx: QuestContext, quest_ids: []const i32) ![]pb.DailyQuestReward {
    var rewards: std.ArrayList(pb.DailyQuestReward) = .empty;
    var any_claimed = false;

    for (quest_ids) |qid| {
        if (qid <= 0) continue;
        if (!QuestInfo.containsInt(ctx.quest_comp.info.daily.daily_quests, qid)) continue;
        if (QuestInfo.containsInt(ctx.quest_comp.info.daily.claimed_daily_rewards, qid)) continue;
        if (!ctx.quest_comp.info.hasFinishedQuest(qid)) continue;

        if (ctx.assets.tables.quest_data.getDataById(qid)) |q| {
            if (q.Data.RewardId > 0) {
                _ = try RewardService.grantDropPackagePopup(makeRewardContext(ctx), q.Data.RewardId, 6003);
                if (ctx.assets.tables.drop_package.getDataById(q.Data.RewardId)) |pkg| {
                    var it = pkg.DropPreview.map.iterator();
                    while (it.next()) |kv| {
                        const iid = kv.key_ptr.*;
                        if (iid <= 0) continue;
                        const cnt = if (kv.value_ptr.* > 0) kv.value_ptr.* else 1;
                        try rewards.append(ctx.alloc.arena, .{ .ItemId = iid, .ItemCount = cnt });
                    }
                }
            }
        }

        _ = try QuestInfo.appendUniqueInt(&ctx.quest_comp.info.daily.claimed_daily_rewards, ctx.alloc.gpa, qid);
        any_claimed = true;
    }

    if (any_claimed) {
        try pushDailyAndBranchNotifies(ctx);
        try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);
    }
    return rewards.items;
}

pub fn changeDailyPrefer(ctx: QuestContext, target_area: i32, target_influence: i32) !void {
    if (target_area > 0) ctx.quest_comp.info.daily.preferred_daily_area = target_area;
    ctx.quest_comp.info.daily.preferred_daily_influence = target_influence;
    try pushDailyAndBranchNotifies(ctx);
    try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);
}

pub fn syncDailyAndBranchOnLogin(ctx: QuestContext) !void {
    try pushDailyAndBranchNotifies(ctx);
}

fn pushDailyAndBranchNotifies(ctx: QuestContext) !void {
    var all_quest: std.ArrayList(pb.MapEntry(i32, i32)) = .empty;
    for (ctx.quest_comp.info.daily.daily_quests) |dq| {
        const status: i32 = if (QuestInfo.containsInt(ctx.quest_comp.info.daily.claimed_daily_rewards, dq))
            2
        else if (ctx.quest_comp.info.hasFinishedQuest(dq))
            1
        else
            0;
        try all_quest.append(ctx.alloc.arena, .{ .key = dq, .value = status });
    }
    var got_rewards: std.ArrayList(i32) = .empty;
    for (ctx.quest_comp.info.daily.claimed_daily_rewards) |cq| {
        try got_rewards.append(ctx.alloc.arena, cq);
    }
    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.DailyQuestStateNotify{
        .AllQuest = all_quest,
        .GetReward = got_rewards,
    }) catch {};

    var unlock_areas: std.ArrayList(i32) = .empty;
    try unlock_areas.append(ctx.alloc.arena, 1);
    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.DailyQuestUnlockAreaNotify{
        .CurrChoosedArea = ctx.quest_comp.info.daily.preferred_daily_area,
        .UnlockAreaIds = unlock_areas,
        .IsInit = true,
        .CurrChoosedInfluence = ctx.quest_comp.info.daily.preferred_daily_influence,
    }) catch {};

    if (ctx.quest_comp.info.progress.quest_branches.len > 0) {
        const branch_info = try buildQuestBranchInfo(ctx);
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.QuestBranchUpdateNotify{
            .QuestBranchInfo = branch_info,
            .ErrorCode = .Success,
        }) catch {};
    }
}

pub fn setTrackedQuest(ctx: QuestContext, quest_id: i32) !void {
    if (quest_id == 0 or ctx.quest_comp.info.hasActiveQuest(quest_id)) {
        ctx.quest_comp.info.tracked_quest_id = quest_id;
        if (ctx.scene) |scene| scene.current_quest_id = quest_id;
        syncSceneCurrentNode(ctx);
        try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);
    }
}

pub fn setSceneTime(ctx: QuestContext, hour: i32, minute: i32, show_ui: bool) !void {
    try QuestActions.applySetTime(ctx, actionCallbacks(), hour, minute, show_ui);
    try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);
}

pub fn changeWeather(ctx: QuestContext, weather_id: i32) !void {
    try QuestActions.applyWeatherChange(ctx, actionCallbacks(), weather_id);
    try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);
}

pub fn changeDataLayer(ctx: QuestContext, load_layers: []const i32, unload_layers: []const i32) !void {
    try QuestActions.applyDataLayerChange(ctx, load_layers, unload_layers);
    if (try recheckPendingConditions(ctx)) {
        try finalizeProgression(ctx);
    } else {
        try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);
    }
}

pub fn changeSubLevels(
    ctx: QuestContext,
    load_levels: []const []const u8,
    unload_levels: []const []const u8,
    hide_levels: []const []const u8,
    tp_entity_id: i32,
) !void {
    try QuestActions.applySubLevelChange(ctx, load_levels, unload_levels, hide_levels, tp_entity_id);
    if (try recheckPendingConditions(ctx)) {
        try finalizeProgression(ctx);
    } else {
        try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);
    }
}

pub fn enterInst(ctx: QuestContext, inst_id: i32, pos_entity_id: i32) !void {
    if (inst_id <= 0) return;
    if (ctx.scene) |scene| scene.clearFailedDungeon(inst_id);
    try QuestActions.applyInstanceTrialRoles(ctx, inst_id);
    var target_pos: ?[3]f32 = null;
    if (pos_entity_id > 0) {
        if (ctx.assets.findLevelEntity(inst_id, pos_entity_id)) |cfg| {
            target_pos = Assets.AssetIndexes.entityWorldPos(cfg);
        }
    }
    try QuestActions.transitionToScene(ctx, inst_id, target_pos, null);
    try onDungeonEvent(ctx, inst_id, false);
}

pub fn leaveInst(ctx: QuestContext, pos_entity_id: i32) !void {
    const prev_inst = if (ctx.scene) |s| s.instance_id else 0;
    try QuestActions.leaveCurrentDungeon(ctx, pos_entity_id);
    if (prev_inst > 0 and prev_inst != 8) {
        if (ctx.scene) |scene| {
            if (!QuestInfo.containsInt(ctx.quest_comp.info.completed_dungeons, prev_inst)) {
                scene.recordFailedDungeon(ctx.alloc.gpa, prev_inst) catch {};
            }
        }
    }
}

pub fn restartFlow(ctx: QuestContext, flow_inc_id: i64) !void {
    const scene = ctx.scene orelse return;
    if (scene.active_flow_list_name.len == 0) return;
    if (flow_inc_id != 0 and scene.active_flow_inc_id != 0 and scene.active_flow_inc_id != flow_inc_id) return;

    prepareFlowStateEntities(ctx, scene.active_flow_list_name, scene.active_flow_id, scene.active_flow_state_id);
    const inc_id = if (scene.active_flow_inc_id != 0) scene.active_flow_inc_id else ctx.quest_comp.info.allocFlowIncId();
    const qid = scene.active_flow_quest_id;
    const bt_inc_id: i64 = if (qid != 0) qid else inc_id;
    try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.FlowStartNotify{
        .FlowIncId = inc_id,
        .FlowListName = scene.active_flow_list_name,
        .FlowId = scene.active_flow_id,
        .StateId = scene.active_flow_state_id,
        .GameCtx = .{
            .CtxType = .BehaviorTree,
            .CtxInfo = .{ .BehaviorTree = .{
                .IncId = bt_inc_id,
                .BtType = .BtTypeQuest,
                .BtId = qid,
                .NodeId = scene.active_flow_node_id,
            } },
        },
    });
}

pub fn evalConditionGroups(ctx: QuestContext, group_ids: []const i32) !std.ArrayList(pb.ConditionGroupFinishInfo) {
    var result: std.ArrayList(pb.ConditionGroupFinishInfo) = .empty;
    const eval_ctx = makeEvalContext(ctx);
    for (group_ids) |gid| {
        var fin: std.ArrayList(i32) = .empty;
        if (ctx.assets.tables.condition_group.getDataById(gid)) |grp| {
            for (grp.GroupId) |cid| {
                if (ConditionEvaluator.evalConditionId(eval_ctx, cid)) {
                    try fin.append(ctx.alloc.arena, cid);
                }
            }
        }
        try result.append(ctx.alloc.arena, .{
            .ConditionGroupId = gid,
            .FinishConditions = fin,
        });
    }
    return result;
}

fn completeMatchingChildQuests(ctx: QuestContext, cond_type: []const u8, id_field: []const u8, target_id: i32) !bool {
    const Match = struct { cond_type: []const u8, id_field: []const u8, target_id: i32 };
    const Check = struct {
        fn run(
            c: QuestContext,
            m: Match,
            qid: i32,
            _: QuestInfo.ActiveQuest,
            nodes: []const LazyNodeStore.NodeEntry,
            node: LazyNodeStore.NodeEntry,
            cond: std.json.ObjectMap,
            ctype: []const u8,
        ) !bool {
            if (!std.mem.eql(u8, ctype, m.cond_type)) return false;
            const wanted = jsonInt(cond.get(m.id_field)) orelse 0;
            if (wanted != 0 and wanted != m.target_id) return false;
            try completeChildQuest(c, qid, nodes, node);
            return true;
        }
    };
    return try advanceMatchingChildNodes(ctx, Match{
        .cond_type = cond_type,
        .id_field = id_field,
        .target_id = target_id,
    }, Check.run);
}

threadlocal var refresh_loop_depth: u8 = 0;
threadlocal var pending_refresh_quests: bool = false;
threadlocal var pending_refresh_push: bool = false;
threadlocal var pending_recheck_conditions: bool = false;
threadlocal var quest_transition_depth: u8 = 0;
const max_refresh_passes: usize = 8;
const max_transition_depth: u8 = 16;

fn recheckPendingConditions(ctx: QuestContext) !bool {
    if (refresh_loop_depth > 0) {
        pending_recheck_conditions = true;
        return false;
    }
    return try runRefreshAndRecheckLoop(ctx, false, false, true);
}

/// Re-evaluates server-backed condition-group child quests after a handler
/// updates a value used by ConditionEvaluator.
pub fn onConditionStateChanged(ctx: QuestContext) !void {
    var cond_prog = try recheckPendingConditions(ctx);
    const quest_added = try refreshAvailableQuests(ctx, true);
    if (try checkActiveQuestFailedMonitors(ctx, null)) {
        cond_prog = true;
    }
    if (refresh_loop_depth == 0) {
        if (try stepPendingControlNodes(ctx)) {
            cond_prog = true;
        }
    }
    if (cond_prog or quest_added) {
        try finalizeProgression(ctx);
    }
    try reevaluateInRangeTriggers(ctx);
    if (LevelPlayHelper.fromQuestContext(ctx)) |lp_ctx| {
        try LevelPlayHelper.onConditionStateChanged(lp_ctx);
    }
}

pub fn onPlayerTransferOrLeaveInst(ctx: QuestContext, is_leave_dungeon: bool) !void {
    const aq_snapshot = try ctx.alloc.arena.dupe(QuestInfo.ActiveQuest, ctx.quest_comp.info.active_quests);
    var changed = false;
    for (aq_snapshot) |aq| {
        const nodes = ctx.assets.quest_nodes.getOwnerNodes(ctx.io, aq.quest_id) catch continue;
        for (nodes) |node| {
            if (!std.mem.eql(u8, node.node_type, "QuestFailed")) continue;
            if (QuestInfo.isNodeFinished(aq, node.id)) continue;
            if (!isSubtreeActiveForMonitor(nodes, aq.active_nodes, node.parent_id)) continue;
            if (node.data != .object) continue;
            const fc_val = node.data.object.get("FailedCondition") orelse continue;
            if (fc_val != .object) continue;
            const is_transfer_fail = jsonBool(fc_val.object.get("IsTransferFailure")) orelse false;
            const is_leave_fail = jsonBool(fc_val.object.get("IsLeaveDungeonFailure")) orelse false;
            if (is_leave_dungeon and is_leave_fail) {
                try rollbackQuestToNode(ctx, aq.quest_id, node.id, .LeaveInstFail, false);
                changed = true;
                break;
            }
            if (is_transfer_fail) {
                try rollbackQuestToNode(ctx, aq.quest_id, node.id, .TransferFail, false);
                changed = true;
                break;
            }
        }
    }
    if (LevelPlayHelper.fromQuestContext(ctx)) |lp_ctx| {
        const entered_snap = try ctx.alloc.arena.dupe(i32, ctx.quest_comp.info.progress.entered_level_plays);
        for (entered_snap) |lpid| {
            if (QuestInfo.containsInt(ctx.quest_comp.info.completed_level_plays, lpid)) continue;
            const nodes = ctx.assets.level_play_nodes.getOwnerNodes(ctx.io, lpid) catch continue;
            for (nodes) |node| {
                if (!std.mem.eql(u8, node.node_type, "QuestFailed")) continue;
                if (node.data != .object) continue;
                const fc_val = node.data.object.get("FailedCondition") orelse continue;
                if (fc_val != .object) continue;
                const is_transfer_fail = jsonBool(fc_val.object.get("IsTransferFailure")) orelse false;
                const is_leave_fail = jsonBool(fc_val.object.get("IsLeaveDungeonFailure")) orelse false;
                if ((is_leave_dungeon and is_leave_fail) or is_transfer_fail) {
                    if (try LevelPlayHelper.onFailTimerDue(lp_ctx, lpid, node.id)) {
                        changed = true;
                    }
                    break;
                }
            }
        }
    }
    if (changed) {
        try finalizeProgression(ctx);
    }
}

pub fn onDifficultyChanged(ctx: QuestContext) !void {
    const aq_snapshot = try ctx.alloc.arena.dupe(QuestInfo.ActiveQuest, ctx.quest_comp.info.active_quests);
    var changed = false;
    for (aq_snapshot) |aq| {
        const nodes = ctx.assets.quest_nodes.getOwnerNodes(ctx.io, aq.quest_id) catch continue;
        for (nodes) |node| {
            if (!std.mem.eql(u8, node.node_type, "QuestFailed")) continue;
            if (QuestInfo.isNodeFinished(aq, node.id)) continue;
            if (!isSubtreeActiveForMonitor(nodes, aq.active_nodes, node.parent_id)) continue;
            if (node.data != .object) continue;
            const fc_val = node.data.object.get("FailedCondition") orelse continue;
            if (fc_val == .object and (jsonBool(fc_val.object.get("SwitchGameDifficulty")) orelse false)) {
                try rollbackQuestToNode(ctx, aq.quest_id, node.id, .DefaultFail, false);
                changed = true;
                break;
            }
        }
    }
    if (LevelPlayHelper.fromQuestContext(ctx)) |lp_ctx| {
        const entered_snap = try ctx.alloc.arena.dupe(i32, ctx.quest_comp.info.progress.entered_level_plays);
        for (entered_snap) |lpid| {
            if (QuestInfo.containsInt(ctx.quest_comp.info.completed_level_plays, lpid)) continue;
            const nodes = ctx.assets.level_play_nodes.getOwnerNodes(ctx.io, lpid) catch continue;
            for (nodes) |node| {
                if (!std.mem.eql(u8, node.node_type, "QuestFailed")) continue;
                if (node.data != .object) continue;
                const fc_val = node.data.object.get("FailedCondition") orelse continue;
                if (fc_val == .object and (jsonBool(fc_val.object.get("SwitchGameDifficulty")) orelse false)) {
                    if (try LevelPlayHelper.onFailTimerDue(lp_ctx, lpid, node.id)) changed = true;
                    break;
                }
            }
        }
    }
    if (changed) {
        try finalizeProgression(ctx);
    }
}

pub fn onSneakFailed(ctx: QuestContext, owner_id: i32, tree_inc_id: i64) !void {
    const target_qid = if (owner_id > 0) owner_id else (std.math.cast(i32, tree_inc_id) orelse 0);
    var changed = false;
    if (target_qid > 0 and ctx.quest_comp.info.hasActiveQuest(target_qid)) {
        try rollbackQuestToNode(ctx, target_qid, 0, .SneakTimeoutFail, false);
        changed = true;
    } else {
        const aq_snapshot = try ctx.alloc.arena.dupe(QuestInfo.ActiveQuest, ctx.quest_comp.info.active_quests);
        for (aq_snapshot) |aq| {
            const nodes = ctx.assets.quest_nodes.getOwnerNodes(ctx.io, aq.quest_id) catch continue;
            for (nodes) |node| {
                if (!std.mem.eql(u8, node.node_type, "QuestFailed")) continue;
                if (QuestInfo.isNodeFinished(aq, node.id)) continue;
                if (!isSubtreeActiveForMonitor(nodes, aq.active_nodes, node.parent_id)) continue;
                if (node.data != .object) continue;
                const fc_val = node.data.object.get("FailedCondition") orelse continue;
                if (fc_val == .object and fc_val.object.get("SneakPlayCondition") != null) {
                    try rollbackQuestToNode(ctx, aq.quest_id, node.id, .SneakTimeoutFail, false);
                    changed = true;
                    break;
                }
            }
        }
    }
    if (LevelPlayHelper.fromQuestContext(ctx)) |lp_ctx| {
        const entered_snap = try ctx.alloc.arena.dupe(i32, ctx.quest_comp.info.progress.entered_level_plays);
        for (entered_snap) |lpid| {
            if (QuestInfo.containsInt(ctx.quest_comp.info.completed_level_plays, lpid)) continue;
            const nodes = ctx.assets.level_play_nodes.getOwnerNodes(ctx.io, lpid) catch continue;
            for (nodes) |node| {
                if (!std.mem.eql(u8, node.node_type, "QuestFailed")) continue;
                if (node.data != .object) continue;
                const fc_val = node.data.object.get("FailedCondition") orelse continue;
                if (fc_val == .object and fc_val.object.get("SneakPlayCondition") != null) {
                    if (try LevelPlayHelper.onFailTimerDue(lp_ctx, lpid, node.id)) changed = true;
                    break;
                }
            }
        }
    }
    if (changed) {
        try finalizeProgression(ctx);
    }
}

pub fn onProgressionUnlockChanged(ctx: QuestContext) !void {
    try onConditionStateChanged(ctx);
    try QuestActions.pushFuncOpen(ctx);
    const show_list = try buildPreShowQuestList(ctx);
    try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.QuestShowListNotify{ .QuestId = show_list });
    try pushReadyQuestList(ctx);
}

/// Evaluates which active quests require block/video subpackage resources that the client
/// has not yet reported as downloaded or confirmed.
pub fn collectMissingResourceQuests(ctx: QuestContext) ![]const i32 {
    const pc = ctx.progress_comp orelse return &.{};
    var missing: std.ArrayList(i32) = .empty;
    var kbuf: [48]u8 = undefined;
    for (ctx.quest_comp.info.active_quests) |aq| {
        if (!ctx.assets.questRequiresResource(aq.quest_id)) continue;
        const dl_k = try std.fmt.bufPrint(&kbuf, "quest:dl:{d}", .{aq.quest_id});
        if (pc.info.hasClaim(dl_k)) continue;
        const ok_k = try std.fmt.bufPrint(&kbuf, "quest:res_ok:{d}", .{aq.quest_id});
        if (pc.info.hasClaim(ok_k)) continue;
        try missing.append(ctx.alloc.arena, aq.quest_id);
    }
    return missing.items;
}

/// Records downloaded/confirmed quest resources from the client and notifies missing dependencies
/// so lite-package clients detect un-downloaded quests before hitting missing cutscenes/blocks.
pub fn syncQuestResourceAvailability(
    ctx: QuestContext,
    downloaded_quest_ids: []const i32,
    confirmed_quest_ids: []const i32,
    is_full_report: bool,
) !void {
    const pc = ctx.progress_comp orelse return;
    var dirty = false;
    var kbuf: [48]u8 = undefined;

    if (is_full_report and pc.info.getKeyInt("quest:res_reported") == 0) {
        try pc.info.setKeyInt(ctx.alloc.gpa, "quest:res_reported", 1);
        dirty = true;
    }
    for (downloaded_quest_ids) |qid| {
        if (qid <= 0) continue;
        const k = try std.fmt.bufPrint(&kbuf, "quest:dl:{d}", .{qid});
        if (try pc.info.markClaim(ctx.alloc.gpa, k)) dirty = true;
    }
    for (confirmed_quest_ids) |qid| {
        if (qid <= 0) continue;
        const ok_k = try std.fmt.bufPrint(&kbuf, "quest:res_ok:{d}", .{qid});
        if (try pc.info.markClaim(ctx.alloc.gpa, ok_k)) dirty = true;
        const dl_k = try std.fmt.bufPrint(&kbuf, "quest:dl:{d}", .{qid});
        if (try pc.info.markClaim(ctx.alloc.gpa, dl_k)) dirty = true;
    }

    if (pc.info.getKeyInt("quest:res_reported") == 0 and confirmed_quest_ids.len == 0) {
        if (dirty) try pc.save(ctx.fs, ctx.alloc.arena);
        return;
    }

    const missing = try collectMissingResourceQuests(ctx);
    var csv: std.ArrayList(u8) = .empty;
    for (missing, 0..) |qid, idx| {
        if (idx > 0) try csv.append(ctx.alloc.arena, ',');
        const id_txt = try std.fmt.allocPrint(ctx.alloc.arena, "{d}", .{qid});
        try csv.appendSlice(ctx.alloc.arena, id_txt);
    }
    if (!std.mem.eql(u8, pc.info.getKeyStr("quest:missing_res"), csv.items)) {
        try pc.info.setKeyStr(ctx.alloc.gpa, "quest:missing_res", csv.items);
        dirty = true;
    }
    if (dirty) try pc.save(ctx.fs, ctx.alloc.arena);

    var lack_ids: std.ArrayList(i32) = .empty;
    var lock_infos: std.ArrayList(pb.QuestLockInfo) = .empty;
    for (missing) |qid| {
        try lack_ids.append(ctx.alloc.arena, qid);
        try lock_infos.append(ctx.alloc.arena, .{
            .QuestId = qid,
            .MultiModeLock = false,
            .LockType = .QuestResource,
        });
    }

    if (missing.len > 0) {
        log.warn(
            "player({d}) active quests missing downloaded resources ({d}): {any}",
            .{ ctx.basic_comp.player_id, missing.len, missing },
        );
    } else if (is_full_report or confirmed_quest_ids.len > 0) {
        log.info(
            "player({d}) all resource-dependent active quests are ready (reported={d}, confirmed={d})",
            .{ ctx.basic_comp.player_id, downloaded_quest_ids.len, confirmed_quest_ids.len },
        );
    }

    try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.WaitQuestConfirmResourceNotify{
        .LackResourceQuestIds = lack_ids,
    });
    try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.QuestLockInfoNotify{
        .QuestLockInfos = lock_infos,
    });
}

/// PassPortalRequest is the client's authoritative portal traversal event.
pub fn onPortalPassed(ctx: QuestContext, portal_config_id: i32) !void {
    if (portal_config_id <= 0) return;
    const scene = ctx.scene orelse return;
    scene.last_passed_portal_id = portal_config_id;
    defer scene.last_passed_portal_id = 0;
    try onConditionStateChanged(ctx);
}

fn recheckPendingConditionsPass(ctx: QuestContext) !bool {
    const Check = struct {
        fn run(
            c: QuestContext,
            _: void,
            qid: i32,
            _: QuestInfo.ActiveQuest,
            nodes: []const LazyNodeStore.NodeEntry,
            node: LazyNodeStore.NodeEntry,
            cond: std.json.ObjectMap,
            ctype: []const u8,
        ) !bool {
            if (std.mem.eql(u8, ctype, "CheckConditionGroup") or
                std.mem.eql(u8, ctype, "CheckClientConditionGroup") or
                std.mem.eql(u8, ctype, "CompareVar") or
                std.mem.eql(u8, ctype, "CheckEntityState") or
                std.mem.eql(u8, ctype, "CheckLevelPlay") or
                std.mem.eql(u8, ctype, "CheckActivityState") or
                std.mem.eql(u8, ctype, "GetItem") or
                std.mem.eql(u8, ctype, "VisionSystem") or
                std.mem.eql(u8, ctype, "ScheduleTime") or
                std.mem.eql(u8, ctype, "EnterDungeon") or
                std.mem.eql(u8, ctype, "ReadMail") or
                std.mem.eql(u8, ctype, "CheckTargetBattleAttribute") or
                std.mem.eql(u8, ctype, "WaitBattleCondition") or
                std.mem.eql(u8, ctype, "AwakeAndLoadEntity") or
                std.mem.eql(u8, ctype, "AtomicProcess") or
                std.mem.eql(u8, ctype, "CheckSpringFestivalGameplayCompleted") or
                std.mem.eql(u8, ctype, "CheckUiGame"))
            {
                if (shouldAutoCompleteChildQuest(c, qid, node)) {
                    try completeChildQuest(c, qid, nodes, node);
                    return true;
                }
            } else if (std.mem.eql(u8, ctype, "LeaveDungeon")) {
                if (if (c.scene) |s| s.instance_id == 8 else false) {
                    try completeChildQuest(c, qid, nodes, node);
                    return true;
                }
            }
            _ = cond;
            return false;
        }
    };
    return try advanceMatchingChildNodes(ctx, {}, Check.run);
}

fn stepPendingControlNodes(ctx: QuestContext) !bool {
    const active_snapshot = try snapshotActiveQuestIds(ctx);
    var progressed = false;
    for (active_snapshot) |qid| {
        const aq = ctx.quest_comp.info.findActiveQuest(qid) orelse continue;
        const nodes = ctx.assets.quest_nodes.getOwnerNodes(ctx.io, qid) catch continue;
        var has_control = false;
        for (nodes) |n| {
            if (QuestInfo.isNodeFinished(aq, n.id)) continue;
            if (std.mem.eql(u8, n.node_type, "Condition") or
                std.mem.eql(u8, n.node_type, "ConditionSelector") or
                std.mem.eql(u8, n.node_type, "Repeater") or
                std.mem.eql(u8, n.node_type, "Parallel") or
                std.mem.eql(u8, n.node_type, "ParallelSelect"))
            {
                has_control = true;
                break;
            }
        }
        if (has_control or aq.active_nodes.len == 0) {
            try advanceQuestTree(ctx, qid, true);
            progressed = true;
        }
    }
    return progressed;
}

pub fn recheckAndStepActiveTrees(ctx: QuestContext) !void {
    try onConditionStateChanged(ctx);
}

fn refreshDailyQuests(ctx: QuestContext, push_updates: bool) !bool {
    if (!ctx.quest_comp.info.hasFinishedQuest(prologue_quest_id)) return false;

    var active_or_unclaimed = false;
    for (ctx.quest_comp.info.daily.daily_quests) |dq| {
        if (!QuestInfo.containsInt(ctx.quest_comp.info.daily.claimed_daily_rewards, dq)) {
            active_or_unclaimed = true;
            break;
        }
    }
    if (ctx.quest_comp.info.daily.daily_quests.len > 0 and active_or_unclaimed) {
        var added_missing = false;
        for (ctx.quest_comp.info.daily.daily_quests) |dq| {
            if (!ctx.quest_comp.info.hasFinishedQuest(dq) and !ctx.quest_comp.info.hasActiveQuest(dq)) {
                if (try activateQuest(ctx, dq, push_updates)) added_missing = true;
            }
        }
        return added_missing;
    }

    var seen_types: [4]i32 = .{ 0, 0, 0, 0 };
    var picked_count: usize = 0;
    var any_added = false;
    const pref_country = if (ctx.quest_comp.info.daily.preferred_daily_area > 0)
        ctx.quest_comp.info.daily.preferred_daily_area
    else
        1;

    for (ctx.assets.tables.daily_task_group.items) |grp| {
        if (picked_count >= 4) break;
        if (grp.CountryId != pref_country or grp.TaskIds.len == 0) continue;

        var type_used = false;
        for (seen_types[0..picked_count]) |st| {
            if (st == grp.TypeId) {
                type_used = true;
                break;
            }
        }
        if (type_used) continue;

        for (grp.TaskIds) |tid| {
            if (ctx.quest_comp.info.hasFinishedQuest(tid) or ctx.quest_comp.info.hasActiveQuest(tid)) continue;
            if (!ctx.assets.quest_nodes.hasOwner(tid)) continue;
            if (ctx.assets.tables.quest_data.getDataById(tid)) |q| {
                if (!ConditionEvaluator.evalQuestProvideConditions(makeEvalContext(ctx), q.Data.ProvideType)) continue;
            } else continue;

            _ = try QuestInfo.appendUniqueInt(&ctx.quest_comp.info.daily.daily_quests, ctx.alloc.gpa, tid);
            seen_types[picked_count] = grp.TypeId;
            picked_count += 1;
            if (try activateQuest(ctx, tid, push_updates)) any_added = true;
            break;
        }
    }
    return any_added;
}

fn refreshAvailableQuests(ctx: QuestContext, push_updates: bool) anyerror!bool {
    if (refresh_loop_depth > 0) {
        pending_refresh_quests = true;
        if (push_updates) pending_refresh_push = true;
        return false;
    }
    return try runRefreshAndRecheckLoop(ctx, true, push_updates, false);
}

fn runRefreshAndRecheckLoop(
    ctx: QuestContext,
    initial_refresh: bool,
    initial_push: bool,
    initial_recheck: bool,
) anyerror!bool {
    refresh_loop_depth += 1;
    defer {
        refresh_loop_depth -= 1;
        if (refresh_loop_depth == 0) {
            pending_refresh_quests = false;
            pending_refresh_push = false;
            pending_recheck_conditions = false;
        }
    }

    pending_refresh_quests = initial_refresh;
    pending_refresh_push = initial_push;
    pending_recheck_conditions = initial_recheck;

    var any_progressed = false;
    var pass: usize = 0;
    while ((pending_refresh_quests or pending_recheck_conditions) and pass < max_refresh_passes) : (pass += 1) {
        if (pending_refresh_quests) {
            const push = pending_refresh_push;
            pending_refresh_quests = false;
            pending_refresh_push = false;
            if (try refreshAvailableQuestsPass(ctx, push)) {
                any_progressed = true;
            }
        }
        if (pending_recheck_conditions) {
            pending_recheck_conditions = false;
            if (try recheckPendingConditionsPass(ctx)) {
                any_progressed = true;
            }
        }
    }
    return any_progressed;
}

fn refreshAvailableQuestsPass(ctx: QuestContext, push_updates: bool) anyerror!bool {
    const prologue_done = ctx.quest_comp.info.hasFinishedQuest(prologue_quest_id);
    var any_added = false;

    for (ctx.assets.tables.quest_data.items) |q| {
        const d = q.Data;
        if (!prologue_done and d.Id != prologue_quest_id) continue;
        if (!isEligibleQuest(ctx, d, true)) continue;
        if (!ConditionEvaluator.evalQuestProvideConditions(makeEvalContext(ctx), d.ProvideType)) continue;
        if (try activateQuest(ctx, d.Id, push_updates)) {
            any_added = true;
        }
    }
    return any_added;
}

fn isEligibleQuest(ctx: QuestContext, d: QuestData.DataEntry, auto_only: bool) bool {
    if (d.Id <= 0) return false;
    // Skip dev/test (100); skip time-limited festival events (15) unless explicitly opened
    if (d.Type == 100) return false;
    if (d.Type == 15) {
        if (d.ActivityId <= 0 or !QuestInfo.containsInt(ctx.quest_comp.info.progress.opened_activities, d.ActivityId)) {
            return false;
        }
    }
    if (ctx.quest_comp.info.hasFinishedQuest(d.Id)) return false;
    if (ctx.quest_comp.info.hasActiveQuest(d.Id)) return false;
    if (!ctx.assets.quest_nodes.hasOwner(d.Id)) return false;

    if (auto_only) {
        const is_sys = std.mem.eql(u8, d.DistributeType, "System");
        const is_passive = std.mem.eql(u8, d.DistributeType, "Passive");
        const is_activity = std.mem.eql(u8, d.DistributeType, "Activity") and
            (d.ActivityId == 0 or ConditionEvaluator.isActivityOpen(makeEvalContext(ctx), d.ActivityId));
        if (!is_sys and !is_passive and !is_activity) return false;

        // Avoid flooding level-1 open world with empty-condition or bare-ExploreLevel festival/passive quests
        const explicitly_opened = is_activity and d.ActivityId > 0 and
            QuestInfo.containsInt(ctx.quest_comp.info.progress.opened_activities, d.ActivityId);
        const is_standard_quest = (d.Type >= 1 and d.Type <= 4) or d.Type == 7 or d.Type == 9;
        if (!is_standard_quest and !explicitly_opened) {
            const has_meaningful_cond = blk: {
                const pt = d.ProvideType orelse break :blk false;
                if (pt != .object) break :blk false;
                const cv = pt.object.get("Conditions") orelse break :blk false;
                if (cv != .array or cv.array.items.len == 0) break :blk false;
                for (cv.array.items) |item| {
                    if (item != .object) break :blk true;
                    const ctype = jsonStr(item.object.get("Type")) orelse break :blk true;
                    if (std.mem.eql(u8, ctype, "OpenDate") or std.mem.eql(u8, ctype, "Gender")) continue;
                    if (std.mem.eql(u8, ctype, "ExploreLevel")) {
                        const req_lv = jsonInt(item.object.get("ExploreLevel")) orelse 1;
                        if (d.Type == 11 and req_lv <= 5) continue;
                    }
                    break :blk true;
                }
                break :blk false;
            };
            if (!has_meaningful_cond) return false;
        }
    }
    return true;
}

fn activateQuest(ctx: QuestContext, quest_id: i32, push_updates: bool) anyerror!bool {
    if (quest_transition_depth >= max_transition_depth) return false;
    if (ctx.quest_comp.info.hasActiveQuest(quest_id) or ctx.quest_comp.info.hasFinishedQuest(quest_id)) {
        return false;
    }

    quest_transition_depth += 1;
    defer quest_transition_depth -= 1;

    _ = try ctx.quest_comp.info.getOrAddActiveQuest(ctx.alloc.gpa, quest_id);

    if (push_updates) {
        var ql: pb.QuestListNotify = .default;
        try ql.Quests.append(ctx.alloc.arena, .{ .QuestId = quest_id, .Status = .InProgress });
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, ql);

        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.QuestShowListNotify{ .QuestId = try buildPreShowQuestList(ctx) });
        try pushReadyQuestList(ctx);

        var tree_infos: std.ArrayList(pb.TreeInfo) = .empty;
        try tree_infos.append(ctx.alloc.arena, initTreeInfo(quest_id, .empty, .empty));
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.BehaviorTreeInfoNotify{ .TreeInfos = tree_infos });
    }

    if (ctx.assets.tables.quest_data.getDataById(quest_id)) |q| {
        try executeActions(ctx, quest_id, 0, q.Data.ActiveActions);
        try executeActions(ctx, quest_id, 0, q.Data.AcceptActions);
    }

    try advanceQuestTree(ctx, quest_id, push_updates);
    return true;
}

fn advanceQuestTree(ctx: QuestContext, quest_id: i32, push_updates: bool) anyerror!void {
    const nodes = ctx.assets.quest_nodes.getOwnerNodes(ctx.io, quest_id) catch return;
    if (nodes.len == 0) return;

    const root = findRootNode(nodes) orelse nodes[0];
    var steps: usize = 0;
    const res = try stepNode(ctx, quest_id, nodes, root, push_updates, &steps);
    if (res == .completed and ctx.quest_comp.info.hasActiveQuest(quest_id)) {
        try succeedQuest(ctx, quest_id);
    }
}

fn stepNode(
    ctx: QuestContext,
    quest_id: i32,
    nodes: []const LazyNodeStore.NodeEntry,
    node: LazyNodeStore.NodeEntry,
    push_updates: bool,
    steps: *usize,
) anyerror!StepResult {
    if (steps.* >= max_steps_per_advance) return .running;
    steps.* += 1;

    if (!ctx.quest_comp.info.hasActiveQuest(quest_id)) return .quest_ended;
    const aq = ctx.quest_comp.info.findActiveQuest(quest_id) orelse return .quest_ended;
    if (QuestInfo.isNodeFinished(aq, node.id)) return .completed;

    if (try checkNodeOccupationAndSave(ctx, .BtTypeQuest, quest_id, node, push_updates)) |suspend_res| {
        return suspend_res;
    }

    if (std.mem.eql(u8, node.node_type, "Sequence")) {
        for (nodes) |child| {
            if (child.parent_id != node.id) continue;
            const child_res = try stepNode(ctx, quest_id, nodes, child, push_updates, steps);
            switch (child_res) {
                .running => return .running,
                .failed => return .failed,
                .quest_ended => return .quest_ended,
                .completed => continue,
            }
        }
        try finishNonChildNode(ctx, quest_id, node, push_updates);
        return .completed;
    } else if (std.mem.eql(u8, node.node_type, "Select")) {
        var any_child = false;
        for (nodes) |child| {
            if (child.parent_id != node.id) continue;
            any_child = true;
            const child_res = try stepNode(ctx, quest_id, nodes, child, push_updates, steps);
            switch (child_res) {
                .quest_ended => return .quest_ended,
                .completed => {
                    try finishNonChildNode(ctx, quest_id, node, push_updates);
                    return .completed;
                },
                .running => return .running,
                .failed => continue,
            }
        }
        if (any_child and node.parent_id != 0) return .failed;
        try finishNonChildNode(ctx, quest_id, node, push_updates);
        return .completed;
    } else if (std.mem.eql(u8, node.node_type, "ParallelSelect") or std.mem.eql(u8, node.node_type, "Parallel")) {
        const is_parallel_all = std.mem.eql(u8, node.node_type, "Parallel");
        var eligible_children: i32 = 0;
        for (nodes) |child| {
            if (child.parent_id != node.id) continue;
            if (!std.mem.eql(u8, child.node_type, "QuestFailed")) {
                eligible_children += 1;
            }
        }
        const needed_sel: i32 = if (node.count > 0) node.count else @max(@as(i32, 1), eligible_children);
        var finished_count: i32 = 0;
        var failed_count: i32 = 0;
        var total_children: i32 = 0;
        var any_ended = false;

        for (nodes) |child| {
            if (child.parent_id != node.id) continue;
            total_children += 1;
            const cur_aq = ctx.quest_comp.info.findActiveQuest(quest_id) orelse return .quest_ended;
            if (QuestInfo.isNodeFinished(cur_aq, child.id)) {
                finished_count += 1;
                continue;
            }

            const child_res = try stepNode(ctx, quest_id, nodes, child, push_updates, steps);
            switch (child_res) {
                .quest_ended => {
                    any_ended = true;
                    break;
                },
                .completed => {
                    finished_count += 1;
                    if (!is_parallel_all and finished_count >= needed_sel) break;
                },
                .failed => {
                    failed_count += 1;
                },
                .running => {},
            }
        }
        if (any_ended or !ctx.quest_comp.info.hasActiveQuest(quest_id)) return .quest_ended;

        const needed: i32 = if (is_parallel_all) total_children else needed_sel;
        if (is_parallel_all and failed_count > 0) return .failed;
        if (finished_count >= needed) {
            if (!is_parallel_all) {
                try deactivateSubtree(ctx, quest_id, nodes, node.id, push_updates);
            }
            try finishNonChildNode(ctx, quest_id, node, push_updates);
            return .completed;
        }
        if (!is_parallel_all and node.parent_id != 0 and total_children > 0 and (total_children - failed_count) < needed) {
            return .failed;
        }
        return .running;
    } else if (std.mem.eql(u8, node.node_type, "AlwaysPending")) {
        for (nodes) |child| {
            if (child.parent_id != node.id) continue;
            const res = try stepNode(ctx, quest_id, nodes, child, push_updates, steps);
            if (res == .quest_ended) return .quest_ended;
        }
        return .running;
    } else if (std.mem.eql(u8, node.node_type, "AlwaysFalse")) {
        for (nodes) |child| {
            if (child.parent_id != node.id) continue;
            const res = try stepNode(ctx, quest_id, nodes, child, push_updates, steps);
            if (res == .quest_ended) return .quest_ended;
        }
        return .running;
    } else if (std.mem.eql(u8, node.node_type, "AlwaysTrue")) {
        for (nodes) |child| {
            if (child.parent_id != node.id) continue;
            const res = try stepNode(ctx, quest_id, nodes, child, push_updates, steps);
            if (res == .quest_ended) return .quest_ended;
            if (res == .running) return .running;
        }
        try finishNonChildNode(ctx, quest_id, node, push_updates);
        return .completed;
    } else if (std.mem.eql(u8, node.node_type, "QuestSucceed")) {
        if (push_updates) {
            ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.UpdateNodeStatusNotify{
                .TreeOwnerId = quest_id,
                .TreeIncId = quest_id,
                .NodeId = node.id,
                .Status = .CompletedSuccess,
            }) catch {};
            if (node.data == .object) {
                if (node.data.object.get("FinishActions")) |acts_val| {
                    if (acts_val == .array and acts_val.array.items.len > 0) {
                        pushBtLevelEvents(ctx, quest_id, node.id, .SuccessNodeAction, acts_val.array.items) catch {};
                    }
                }
            }
        }
        try runNodeFinishEffects(ctx, quest_id, node);
        try succeedQuest(ctx, quest_id);
        return .quest_ended;
    } else if (std.mem.eql(u8, node.node_type, "QuestFailed")) {
        return try stepQuestFailedNode(ctx, quest_id, nodes, node, push_updates);
    } else if (std.mem.eql(u8, node.node_type, "Action") or std.mem.eql(u8, node.node_type, "ActionWithResult")) {
        if (node.data == .object) {
            var single_buf: [1]std.json.Value = undefined;
            if (extractNodeActionItems(node.data, &single_buf)) |items| {
                const akey = actionNodeProgressKey(quest_id, node.id);
                const start_idx: usize = if (ctx.scene) |sc| (sc.action_node_next_idx.get(akey) orelse 0) else 0;
                if (start_idx == 0 and push_updates and items.len > 0) {
                    try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.UpdateNodeStatusNotify{
                        .TreeOwnerId = quest_id,
                        .TreeIncId = quest_id,
                        .NodeId = node.id,
                        .Status = .Activate,
                    });
                    try pushBtLevelEvents(
                        ctx,
                        quest_id,
                        node.id,
                        .ActionGroupNodeAction,
                        items,
                    );
                }
                if (start_idx < items.len) {
                    for (items[start_idx..], start_idx..) |act_item, idx| {
                        try executeActions(ctx, quest_id, node.id, &.{act_item});
                        if (ctx.scene) |scene| {
                            if (isSyncPlayFlowAction(act_item) and
                                ((scene.active_flow_inc_id != 0 and
                                scene.active_flow_quest_id == quest_id and
                                scene.active_flow_node_id == node.id) or
                                scene.hasPendingFlowForNode(quest_id, node.id)))
                            {
                                try scene.action_node_next_idx.put(ctx.alloc.gpa, akey, idx + 1);
                                return .running;
                            }
                        }
                    }
                }
                if (ctx.scene) |scene| {
                    if ((scene.active_flow_inc_id != 0 and
                        scene.active_flow_quest_id == quest_id and
                        scene.active_flow_node_id == node.id) or
                        scene.hasPendingFlowForNode(quest_id, node.id))
                    {
                        try scene.action_node_next_idx.put(ctx.alloc.gpa, akey, items.len);
                        return .running;
                    }
                    _ = scene.action_node_next_idx.remove(akey);
                }
            }
        }
        try finishNonChildNode(ctx, quest_id, node, push_updates);
        return .completed;
    } else if (std.mem.eql(u8, node.node_type, "Condition")) {
        const wait_true = if (node.data == .object)
            if (node.data.object.get("WaitUntilTrue")) |w| w == .bool and w.bool else false
        else
            false;
        const ok = if (node.data == .object)
            if (node.data.object.get("Condition")) |cb|
                ConditionEvaluator.evalInlineConditionBlock(makeEvalContextForQuest(ctx, quest_id), cb)
            else
                true
        else
            true;
        if (ok) {
            try finishNonChildNode(ctx, quest_id, node, push_updates);
            return .completed;
        }
        return if (wait_true) .running else .failed;
    } else if (std.mem.eql(u8, node.node_type, "ConditionSelector")) {
        if (node.data == .object) {
            if (node.data.object.get("Slots")) |slots_val| {
                if (slots_val == .array) {
                    for (slots_val.array.items) |slot| {
                        if (slot != .object) continue;
                        const pass = if (slot.object.get("Condition")) |sc|
                            ConditionEvaluator.evalInlineConditionBlock(makeEvalContextForQuest(ctx, quest_id), sc)
                        else
                            true;
                        if (!pass) continue;
                        const sn = slot.object.get("Node") orelse continue;
                        if (sn != .object) continue;
                        const cid = jsonInt(sn.object.get("Id")) orelse continue;
                        const cnode = findNodeById(nodes, cid) orelse continue;
                        const res = try stepNode(ctx, quest_id, nodes, cnode.*, push_updates, steps);
                        switch (res) {
                            .completed => {
                                try finishNonChildNode(ctx, quest_id, node, push_updates);
                                return .completed;
                            },
                            .running, .quest_ended => return res,
                            .failed => continue,
                        }
                    }
                }
            }
        }
        return .running;
    } else if (std.mem.eql(u8, node.node_type, "Repeater")) {
        const cd_key = quest_types.repeaterCooldownKey(-quest_id, node.id);
        while (steps.* < max_steps_per_advance) {
            if (node.data == .object) {
                if (node.data.object.get("ExitCondition")) |ec| {
                    if (ConditionEvaluator.evalInlineConditionBlock(makeEvalContextForQuest(ctx, quest_id), ec)) {
                        try deactivateSubtree(ctx, quest_id, nodes, node.id, push_updates);
                        try finishNonChildNode(ctx, quest_id, node, push_updates);
                        return .completed;
                    }
                }
            }
            if (ctx.scene) |scene| {
                if (scene.lp_repeater_cooldowns.get(cd_key)) |ready_ms| {
                    if (nowMs(ctx.io) < ready_ms) return .running;
                }
            }
            var any_child = false;
            var child_completed = false;
            for (nodes) |child| {
                if (child.parent_id != node.id) continue;
                any_child = true;
                const res = try stepNode(ctx, quest_id, nodes, child, push_updates, steps);
                if (res == .quest_ended) return .quest_ended;
                if (res == .completed) {
                    child_completed = true;
                    break;
                }
                return .running;
            }
            if (!any_child) {
                try finishNonChildNode(ctx, quest_id, node, push_updates);
                return .completed;
            }
            if (!child_completed) return .running;
            try resetQuestSubtreeFinishedNodes(ctx, quest_id, nodes, node.id);
            if (ctx.scene) |scene| {
                const wait_ms = quest_types.subtreeWaitDelayMs(nodes, node.id);
                if (wait_ms > 0) {
                    scene.lp_repeater_cooldowns.put(ctx.alloc.gpa, cd_key, nowMs(ctx.io) + wait_ms) catch {};
                    return .running;
                }
            }
        }
        return .running;
    } else if (std.mem.eql(u8, node.node_type, "ChildQuest")) {
        return try stepChildQuest(ctx, quest_id, nodes, node, push_updates);
    }

    try finishNonChildNode(ctx, quest_id, node, push_updates);
    return .completed;
}

fn stepChildQuest(
    ctx: QuestContext,
    quest_id: i32,
    nodes: []const LazyNodeStore.NodeEntry,
    node: LazyNodeStore.NodeEntry,
    push_updates: bool,
) !StepResult {
    const aq_ptr = ctx.quest_comp.info.findActiveQuestPtr(quest_id) orelse return .quest_ended;
    const newly_entered = try QuestInfo.markNodeActive(aq_ptr, ctx.alloc.gpa, node.id);

    if (newly_entered) {
        if (push_updates) {
            try pushAncestorActivations(ctx, quest_id, nodes, node.parent_id);
            try pushNodeStatus(ctx, quest_id, node.id, .Activate, .Progress);
        }
        if (node.data == .object) {
            if (node.data.object.get("EnterActions")) |acts_val| {
                if (acts_val == .array) {
                    try executeActions(ctx, quest_id, node.id, acts_val.array.items);
                }
            }
        }
        try onEnterChildQuestSpecial(ctx, quest_id, node);
    }

    if (shouldAutoCompleteChildQuest(ctx, quest_id, node)) {
        try completeChildQuestWithoutAdvance(ctx, quest_id, node, push_updates);
        return .completed;
    }

    return .running;
}

pub fn nowMsPub(io: std.Io) i64 {
    return nowMs(io);
}

pub fn parseTimerDelayMsPub(cond: std.json.ObjectMap) ?i64 {
    return parseTimerDelayMs(cond);
}

fn nowMs(io: std.Io) i64 {
    return std.Io.Clock.awake.now(io).toMilliseconds();
}

fn parseTimerDelayMs(cond: std.json.ObjectMap) ?i64 {
    const ctype = jsonStr(cond.get("Type")) orelse return null;
    if (std.mem.eql(u8, ctype, "Timer")) {
        const seconds = jsonFloat(cond.get("Time")) orelse return 0;
        if (!std.math.isFinite(seconds) or seconds <= 0.0) return 0;
        const ms_f = @round(@as(f64, seconds) * 1000.0);
        if (ms_f >= @as(f64, @floatFromInt(std.math.maxInt(i64)))) return std.math.maxInt(i64);
        return @intFromFloat(ms_f);
    }
    if (std.mem.eql(u8, ctype, "WaitTime")) {
        if (jsonFloat(cond.get("Time"))) |seconds| {
            if (!std.math.isFinite(seconds) or seconds <= 0.0) return 0;
            const ms_f = @round(@as(f64, seconds) * 1000.0);
            if (ms_f >= @as(f64, @floatFromInt(std.math.maxInt(i64)))) return std.math.maxInt(i64);
            return @intFromFloat(ms_f);
        }
        const days = jsonInt(cond.get("Days")) orelse 0;
        const hours = jsonInt(cond.get("Hours")) orelse 0;
        const mins = jsonInt(cond.get("Minutes")) orelse 0;
        const secs = jsonInt(cond.get("Seconds")) orelse 0;
        const total_game_sec: i64 = @as(i64, days) * 86400 + @as(i64, hours) * 3600 + @as(i64, mins) * 60 + secs;
        if (total_game_sec <= 0) return 0;
        // Game time runs ~60x faster than real time; cap fallback real wait so quests never stall if clock isn't adjusted.
        const scaled_ms = @divTrunc(total_game_sec * 1000, 60);
        return std.math.clamp(scaled_ms, 1500, 8000);
    }
    return null;
}

fn ensureActiveQuestTimersScheduled(ctx: QuestContext) !void {
    const scene = ctx.scene orelse return;
    const now_ms = nowMs(ctx.io);
    for (ctx.quest_comp.info.active_quests) |aq| {
        const nodes = ctx.assets.quest_nodes.getOwnerNodes(ctx.io, aq.quest_id) catch continue;
        for (aq.active_nodes) |nid| {
            if (scene.quest_timers.contains(aq.quest_id, nid)) continue;
            const node = findNodeById(nodes, nid) orelse continue;
            if (!std.mem.eql(u8, node.node_type, "ChildQuest")) continue;
            const cond = getChildCondition(node.data) orelse continue;
            const delay_ms = parseTimerDelayMs(cond) orelse continue;
            if (delay_ms > 0) {
                try scene.quest_timers.upsert(ctx.alloc.gpa, .{
                    .quest_id = aq.quest_id,
                    .node_id = nid,
                    .due_ms = now_ms + delay_ms,
                });
            }
        }
    }
    if (LevelPlayHelper.fromQuestContext(ctx)) |lp_ctx| {
        try LevelPlayHelper.ensureActiveTimersScheduled(lp_ctx);
    }
}

/// Fires due `Timer`/`WaitTime` child quest, LevelPlay timer, and `QuestFailed` deadline nodes.
pub fn onTimersDue(ctx: QuestContext, now_ms: i64) !void {
    const scene = ctx.scene orelse return;
    var progressed = false;

    while (scene.quest_timers.popDue(now_ms)) |due| {
        const aq = ctx.quest_comp.info.findActiveQuest(due.quest_id) orelse continue;
        if (!QuestInfo.isNodeActive(aq, due.node_id)) continue;
        const nodes = ctx.assets.quest_nodes.getOwnerNodes(ctx.io, due.quest_id) catch continue;
        const node = findNodeById(nodes, due.node_id) orelse continue;
        try completeChildQuest(ctx, due.quest_id, nodes, node.*);
        progressed = true;
    }

    while (scene.lp_timers.popDue(now_ms)) |due| {
        if (LevelPlayHelper.fromQuestContext(ctx)) |lp_ctx| {
            if (try LevelPlayHelper.onTimerDue(lp_ctx, due.quest_id, due.node_id)) {
                progressed = true;
            }
        }
    }

    while (scene.fail_timers.popDue(now_ms)) |due| {
        if (ctx.quest_comp.info.hasActiveQuest(due.quest_id)) {
            try rollbackQuestToNode(ctx, due.quest_id, due.node_id, .TimeoutFail, false);
            progressed = true;
        }
    }

    while (scene.lp_fail_timers.popDue(now_ms)) |due| {
        if (LevelPlayHelper.fromQuestContext(ctx)) |lp_ctx| {
            if (try LevelPlayHelper.onFailTimerDue(lp_ctx, due.quest_id, due.node_id)) {
                progressed = true;
            }
        }
    }

    if (progressed) {
        try finalizeProgression(ctx);
    }
}

fn onEnterChildQuestSpecial(ctx: QuestContext, quest_id: i32, node: LazyNodeStore.NodeEntry) !void {
    if (!shouldAutoCompleteChildQuest(ctx, quest_id, node)) {
        spawnNodeTargetEntities(ctx, quest_id, node);
        if (ctx.scene) |scene| {
            try pushDoInteractOptionsForNode(scene, ctx.conn, ctx.alloc, ctx.io, quest_id, node, null);
        }
    }

    const cond = getChildCondition(node.data) orelse return;
    const ctype = jsonStr(cond.get("Type")) orelse return;

    if (parseTimerDelayMs(cond)) |delay_ms| {
        if (delay_ms > 0) {
            if (ctx.scene) |scene| {
                if (!scene.quest_timers.contains(quest_id, node.id)) {
                    try scene.quest_timers.upsert(ctx.alloc.gpa, .{
                        .quest_id = quest_id,
                        .node_id = node.id,
                        .due_ms = nowMs(ctx.io) + delay_ms,
                    });
                }
            }
        }
        return;
    }

    if (std.mem.eql(u8, ctype, "PlayFlow")) {
        // First prologue cutscene (139000025:19) starts on SceneLoadingFinishRequest
        if (!(quest_id == prologue_quest_id and node.id == 19)) {
            if (cond.get("Flow")) |flow_val| {
                if (flow_val == .object) {
                    try startFlowFromObj(ctx, quest_id, node.id, flow_val.object);
                }
            }
        }
    }
}

fn shouldAutoCompleteChildQuest(ctx: QuestContext, quest_id: i32, node: LazyNodeStore.NodeEntry) bool {
    const cond = getChildCondition(node.data) orelse return true;
    const ctype = jsonStr(cond.get("Type")) orelse return true;

    if (std.mem.eql(u8, ctype, "ReachArea") or
        std.mem.eql(u8, ctype, "PlayFlow") or
        std.mem.eql(u8, ctype, "DoInteract") or
        std.mem.eql(u8, ctype, "HandInItems") or
        std.mem.eql(u8, ctype, "UseSkill") or
        std.mem.eql(u8, ctype, "FinishDungeon") or
        std.mem.eql(u8, ctype, "DetectCombatState") or
        std.mem.eql(u8, ctype, "DetectCombatState2") or
        std.mem.eql(u8, ctype, "UseItem") or
        std.mem.eql(u8, ctype, "Parkour") or
        std.mem.eql(u8, ctype, "RhythmGame") or
        std.mem.eql(u8, ctype, "SlidingBlocks") or
        std.mem.eql(u8, ctype, "Deliver") or
        std.mem.eql(u8, ctype, "Communicate") or
        std.mem.eql(u8, ctype, "CheckPlayerInput") or
        std.mem.eql(u8, ctype, "FinishArcadeGameplay") or
        std.mem.eql(u8, ctype, "SpringManorGameplayClose") or
        std.mem.eql(u8, ctype, "CompareActorVar") or
        std.mem.eql(u8, ctype, "CrossSceneSyncKillMonster") or
        std.mem.eql(u8, ctype, "FinishFlipper") or
        std.mem.eql(u8, ctype, "FinishKuroSimpleCombatKillMonster") or
        std.mem.eql(u8, ctype, "FinishKurotato") or
        std.mem.eql(u8, ctype, "FinishRoleTower") or
        std.mem.eql(u8, ctype, "FinishRollBlock") or
        std.mem.eql(u8, ctype, "FinishSurvivorsRouge") or
        std.mem.eql(u8, ctype, "FinishTetris") or
        std.mem.eql(u8, ctype, "FinishTrapDefense") or
        std.mem.eql(u8, ctype, "NewHandInItems") or
        std.mem.eql(u8, ctype, "ParallaxAlign") or
        std.mem.eql(u8, ctype, "PlaybackShowTimeline") or
        std.mem.eql(u8, ctype, "TakePicturesWithTimeScale") or
        std.mem.eql(u8, ctype, "WalkingPattern"))
    {
        return false;
    }

    if (parseTimerDelayMs(cond)) |delay_ms| {
        return ctx.scene == null or delay_ms <= 0;
    }

    if (std.mem.eql(u8, ctype, "ScheduleTime")) {
        return isScheduleTimeSatisfied(ctx, cond);
    }

    if (std.mem.eql(u8, ctype, "EnterDungeon")) {
        const target = nestedDungeonId(cond, "EnterType");
        return if (ctx.scene) |s| (target > 0 and s.instance_id == target) else false;
    }

    if (std.mem.eql(u8, ctype, "LeaveDungeon")) {
        return if (ctx.scene) |s| s.instance_id == 8 else true;
    }

    if (std.mem.eql(u8, ctype, "Kill") or std.mem.eql(u8, ctype, "MonsterCreator")) {
        const aq = ctx.quest_comp.info.findActiveQuest(quest_id) orelse return false;
        return isKillSatisfied(aq, cond, ctype);
    }

    if (std.mem.eql(u8, ctype, "Guide")) {
        if (Assets.DataTables.Config.autocomplete_guides) return true;
        if (ctx.guide_comp) |gc| {
            const gid = jsonInt(cond.get("GuideGroupId")) orelse 0;
            if (gid > 0 and gc.info.hasFinished(gid)) return true;
        }
        return false;
    }

    if (std.mem.eql(u8, ctype, "CheckConditionGroup") or std.mem.eql(u8, ctype, "CheckClientConditionGroup")) {
        const eval_ctx = makeEvalContextForQuest(ctx, quest_id);
        if (cond.get("Condition")) |sub| {
            return ConditionEvaluator.evalInlineConditionBlock(eval_ctx, sub);
        }
        if (jsonInt(cond.get("GroupId"))) |gid| {
            return ConditionEvaluator.evalConditionGroup(eval_ctx, gid);
        }
        return false;
    }

    if (std.mem.eql(u8, ctype, "CompareVar")) {
        return evalCompareVarList(ctx, cond);
    }

    if (std.mem.eql(u8, ctype, "CheckLevelPlay")) {
        return isLevelPlaySatisfied(ctx, cond);
    }

    if (std.mem.eql(u8, ctype, "GetItem")) {
        return hasAllItems(ctx, cond.get("Items"));
    }

    if (std.mem.eql(u8, ctype, "CheckEntityState")) {
        return isCheckEntityStateSatisfied(ctx, cond);
    }

    if (std.mem.eql(u8, ctype, "TakePhoto") or std.mem.eql(u8, ctype, "TakePhoto2")) {
        return ctx.quest_comp.info.hasPhotoNodeCompleted(quest_id, node.id);
    }

    if (std.mem.eql(u8, ctype, "ReadMail")) {
        if (jsonInt(cond.get("MailId"))) |mid| {
            if (mid > 0) return QuestInfo.containsInt(ctx.quest_comp.info.progress.read_mails, mid);
        }
        return true;
    }

    if (std.mem.eql(u8, ctype, "ListenToEvent")) {
        if (cond.get("EventType")) |et| {
            if (et == .object) {
                const et_type = jsonStr(et.object.get("Type")) orelse "";
                if (std.mem.eql(u8, et_type, "EntityStateChange")) {
                    const eid = jsonInt(et.object.get("EntityId")) orelse 0;
                    const want_state = jsonStr(et.object.get("State")) orelse "";
                    if (eid > 0 and ctx.scene != null) {
                        if (ctx.quest_comp.info.getEntityState(eid)) |st| {
                            return want_state.len == 0 or std.mem.eql(u8, want_state, st);
                        }
                    }
                } else if (std.mem.eql(u8, et_type, "VarValueChange")) {
                    if (et.object.get("Var")) |vobj| {
                        if (vobj == .object) {
                            if (et.object.get("Value")) |want_v| {
                                const cur = quest_types.resolveVarIntForOwner(ctx, quest_id, vobj);
                                return cur == (jsonInt(want_v) orelse cur + 1);
                            }
                        }
                    }
                }
            }
        }
        return false;
    }

    if (std.mem.eql(u8, ctype, "AwakeAndLoadEntity")) {
        if (ctx.progress_comp) |pc| {
            var buf: [48]u8 = undefined;
            const key = std.fmt.bufPrint(&buf, "awake_node:{d}", .{quest_id}) catch return false;
            return pc.info.getKeyInt(key) == node.id;
        }
        return false;
    }

    if (std.mem.eql(u8, ctype, "CheckUiGame")) {
        if (ctx.progress_comp) |pc| {
            return hasCompletedUiGameplay(pc, cond);
        }
        return false;
    }

    if (std.mem.eql(u8, ctype, "AtomicProcess") or std.mem.eql(u8, ctype, "CheckSpringFestivalGameplayCompleted")) {
        if (ctx.progress_comp) |pc| {
            var buf: [48]u8 = undefined;
            const key = std.fmt.bufPrint(&buf, "quest_sub:{d}:{d}", .{ quest_id, node.id }) catch return false;
            return pc.info.hasClaim(key);
        }
        return false;
    }

    if (std.mem.eql(u8, ctype, "CheckActivityState")) {
        const aid = jsonInt(cond.get("ActivityId")) orelse return true;
        const want = jsonInt(cond.get("ActivityState")) orelse 2;
        const cmp = jsonStr(cond.get("Compare")) orelse "Eq";
        const cur: i32 = if (ConditionEvaluator.isActivityOpen(makeEvalContext(ctx), aid)) 2 else 0;
        if (std.mem.eql(u8, cmp, "Ne") or std.mem.eql(u8, cmp, "NotEq")) return cur != want;
        return cur == want;
    }

    if (std.mem.eql(u8, ctype, "CheckTargetBattleAttribute") or std.mem.eql(u8, ctype, "WaitBattleCondition")) {
        return isBattleAttrOrWaitSatisfied(ctx, cond, ctype, 0, 1.0);
    }

    if (std.mem.eql(u8, ctype, "VisionSystem")) {
        if (cond.get("Quest")) |qv| {
            if (qv == .object) {
                const qt = jsonStr(qv.object.get("Type")) orelse "";
                if (std.mem.eql(u8, qt, "HasVisionGetTargetLevel")) {
                    const req_lv = jsonInt(qv.object.get("TargetLevel")) orelse 1;
                    return ConditionEvaluator.currentCalabashLevel(ctx.assets, ctx.basic_comp, ctx.echo_comp) >= req_lv;
                }
                if (std.mem.eql(u8, qt, "VisionSetActivated")) {
                    if (ctx.progress_comp) |pc| {
                        if (pc.info.hasClaim("vision:activated")) return true;
                    }
                    return ctx.echo_comp.echo_map.count() > 0;
                }
                if (std.mem.eql(u8, qt, "DoVisionIdentify")) {
                    if (ctx.progress_comp) |pc| {
                        if (pc.info.hasClaim("vision:identified")) return true;
                    }
                    return ctx.echo_comp.echo_map.count() > 0;
                }
            }
        }
        return false;
    }

    // ShowUi All / CiacconaAvgBoard / ItemObtain call SubmitNode() from client; Cook/Synthetic/Forging complete via craft handlers.
    if (std.mem.eql(u8, ctype, "ShowUi")) {
        if (cond.get("UiType")) |ut| {
            if (ut == .object) {
                const utype = jsonStr(ut.object.get("Type")) orelse "";
                if (std.mem.eql(u8, utype, "All") or
                    std.mem.eql(u8, utype, "CiacconaAvgBoard") or
                    std.mem.eql(u8, utype, "ItemObtain") or
                    std.mem.eql(u8, utype, "Cook") or
                    std.mem.eql(u8, utype, "Synthetic") or
                    std.mem.eql(u8, utype, "Forging"))
                {
                    return false;
                }
            }
        }
        return true;
    }
    if (std.mem.eql(u8, ctype, "ProgramSpecialProcess") or
        std.mem.eql(u8, ctype, "InformationViewCheck") or
        std.mem.eql(u8, ctype, "GetSkill"))
    {
        return true;
    }

    if (std.mem.eql(u8, ctype, "UseWeatherSwitch")) {
        return false;
    }

    if (std.mem.eql(u8, ctype, "FinishBvbChallenge")) {
        if (jsonInt(cond.get("BvbChallengeId"))) |bvb_id| {
            if (bvb_id > 0) return QuestInfo.containsInt(ctx.quest_comp.info.progress.completed_bvb, bvb_id);
        }
        return true;
    }

    if (std.mem.eql(u8, ctype, "ReadPhoneMessage")) {
        if (jsonInt(cond.get("PhoneMessageId"))) |pmid| {
            if (pmid > 0) return QuestInfo.containsInt(ctx.quest_comp.info.progress.read_phone_messages, pmid);
        }
        return false;
    }

    if (std.mem.eql(u8, ctype, "ReceiveTelecom")) {
        if (jsonInt(cond.get("TelecomId"))) |tid| {
            if (tid > 0) return QuestInfo.containsInt(ctx.quest_comp.info.progress.received_telecoms, tid);
        }
        return false;
    }

    if (std.mem.eql(u8, ctype, "WaitUntilLevelSequenceReachMark")) {
        return false;
    }

    return false;
}

fn completeChildQuest(
    ctx: QuestContext,
    quest_id: i32,
    nodes: []const LazyNodeStore.NodeEntry,
    node: LazyNodeStore.NodeEntry,
) !void {
    try completeChildQuestWithoutAdvance(ctx, quest_id, node, true);
    _ = nodes;
    try advanceQuestTree(ctx, quest_id, true);
}

fn completeChildQuestWithoutAdvance(
    ctx: QuestContext,
    quest_id: i32,
    node: LazyNodeStore.NodeEntry,
    push_updates: bool,
) !void {
    if (ctx.scene) |scene| scene.quest_timers.cancel(quest_id, node.id);
    removeDoInteractOptionsForNode(ctx, quest_id, node.id);
    const aq_ptr = ctx.quest_comp.info.findActiveQuestPtr(quest_id) orelse return;
    if (!try ctx.quest_comp.info.markNodeFinished(aq_ptr, ctx.alloc.gpa, node.id)) return;

    if (push_updates) {
        try pushNodeStatus(ctx, quest_id, node.id, .CompletedSuccess, .Finished);
    }

    try runNodeFinishEffects(ctx, quest_id, node);
}

fn finishNonChildNode(
    ctx: QuestContext,
    quest_id: i32,
    node: LazyNodeStore.NodeEntry,
    push_updates: bool,
) !void {
    if (ctx.scene) |scene| {
        scene.releaseOccupationsForNode(ctx.alloc.gpa, quest_id, node.id);
    }
    if (node.reward_id > 0) {
        _ = try RewardService.grantDropPackage(makeRewardContext(ctx), node.reward_id, 6003);
    }

    const aq_ptr = ctx.quest_comp.info.findActiveQuestPtr(quest_id) orelse return;
    _ = try ctx.quest_comp.info.markNodeFinished(aq_ptr, ctx.alloc.gpa, node.id);

    if (push_updates) {
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.UpdateNodeStatusNotify{
            .TreeOwnerId = quest_id,
            .TreeIncId = quest_id,
            .NodeId = node.id,
            .Status = .CompletedSuccess,
        }) catch {};
    }

    if (node.data == .object) {
        if (node.data.object.get("OccupationConfig")) |occ_val| {
            if (occ_val == .object) {
                if (occ_val.object.get("ExitActions")) |exit_acts| {
                    if (exit_acts == .array and exit_acts.array.items.len > 0) {
                        try executeActions(ctx, quest_id, node.id, exit_acts.array.items);
                    }
                }
            }
        }
    }

    try runNodeFinishEffects(ctx, quest_id, node);
}

fn runNodeFinishEffects(ctx: QuestContext, quest_id: i32, node: LazyNodeStore.NodeEntry) !void {
    if (node.data == .object) {
        if (node.data.object.get("FinishActions")) |acts_val| {
            if (acts_val == .array) {
                try executeActions(ctx, quest_id, node.id, acts_val.array.items);
            }
        }
    }
}

pub fn checkNodeOccupationAndSavePub(
    ctx: QuestContext,
    bt_type: pb.BtType,
    owner_id: i32,
    node: LazyNodeStore.NodeEntry,
    push_updates: bool,
) !bool {
    return (try checkNodeOccupationAndSave(ctx, bt_type, owner_id, node, push_updates)) != null;
}

pub fn evalQuestFailedConditionPub(
    ctx: QuestContext,
    owner_id: i32,
    node: LazyNodeStore.NodeEntry,
    player_pos_opt: ?[3]f32,
    fail_queue: *Scene.QuestTimerQueue,
) !?pb.FailReason {
    return evalQuestFailedCondition(ctx, owner_id, node, player_pos_opt, fail_queue);
}

fn checkNodeOccupationAndSave(
    ctx: QuestContext,
    bt_type: pb.BtType,
    quest_id: i32,
    node: LazyNodeStore.NodeEntry,
    push_updates: bool,
) !?StepResult {
    const scene = ctx.scene orelse return null;
    if (node.data != .object) return null;

    if (node.data.object.get("OccupationConfig")) |occ_val| {
        if (occ_val == .object) {
            const reclaim = jsonBool(occ_val.object.get("ReclaimOccupationWhenEnter")) orelse false;
            if (occ_val.object.get("Occupations")) |occ_list_val| {
                if (occ_list_val == .array and occ_list_val.array.items.len > 0) {
                    var conflicts: std.ArrayList(pb.OccupationPbInfo) = .empty;
                    for (occ_list_val.array.items) |item| {
                        const res_name = jsonStr(item) orelse continue;
                        if (!reclaim) {
                            if (scene.findOccupationConflict(res_name, quest_id, quest_id)) |conf| {
                                try conflicts.append(ctx.alloc.arena, .{
                                    .ResourceName = conf.resource_name,
                                    .NodeId = conf.node_id,
                                    .IncId = conf.inc_id,
                                });
                            }
                        }
                    }
                    if (conflicts.items.len > 0) {
                        if (push_updates) {
                            ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.BtSuspendNotify{
                                .NodeId = node.id,
                                .SuspendType = @intFromEnum(pb.SuspendType.Occupation),
                                .OccupationInfo = conflicts,
                                .TreeOwnerId = quest_id,
                                .TreeIncId = quest_id,
                                .IsScreenOccupy = false,
                            }) catch {};
                        }
                        return .running;
                    }
                    for (occ_list_val.array.items) |item| {
                        const res_name = jsonStr(item) orelse continue;
                        try scene.upsertOccupation(ctx.alloc.gpa, bt_type, quest_id, quest_id, node.id, res_name);
                    }
                }
            }
        }
    }

    if (node.data.object.get("SaveConfig")) |save_val| {
        if (save_val == .object) {
            if (try scene.markSaveNodeEntered(ctx.alloc.gpa, quest_id, node.id)) {
                var cbs = actionCallbacks();
                cbs.bt_type = bt_type;
                if (save_val.object.get("InitConditionActions")) |init_acts| {
                    if (init_acts == .array and init_acts.array.items.len > 0) {
                        try QuestActions.executeActionList(ctx, cbs, quest_id, node.id, init_acts.array.items);
                    }
                }
                if (save_val.object.get("EnterActions")) |enter_acts| {
                    if (enter_acts == .array and enter_acts.array.items.len > 0) {
                        try QuestActions.executeActionList(ctx, cbs, quest_id, node.id, enter_acts.array.items);
                    }
                }
            }
        }
    }

    return null;
}

fn evalQuestFailedCondition(
    ctx: QuestContext,
    quest_id: i32,
    node: LazyNodeStore.NodeEntry,
    player_pos_opt: ?[3]f32,
    fail_queue: *Scene.QuestTimerQueue,
) !?pb.FailReason {
    if (node.data != .object) return null;
    const fc_val = node.data.object.get("FailedCondition") orelse return null;
    if (fc_val != .object) return null;
    const fc = fc_val.object;

    if (fc.get("Timer")) |tval| {
        if (tval == .object) {
            const secs = jsonFloat(tval.object.get("Time")) orelse 0;
            if (std.math.isFinite(secs) and secs > 0) {
                if (!fail_queue.contains(quest_id, node.id)) {
                    const delay_ms: i64 = @intFromFloat(@round(@as(f64, secs) * 1000.0));
                    try fail_queue.upsert(ctx.alloc.gpa, .{
                        .quest_id = quest_id,
                        .node_id = node.id,
                        .due_ms = nowMs(ctx.io) + delay_ms,
                    });
                }
            }
        }
    }

    if (fc.get("RangeLimiting")) |rl_val| {
        if (rl_val == .object) {
            const pt_obj = if (rl_val.object.get("Point")) |p| (if (p == .object) p.object else null) else null;
            const range = jsonFloat(rl_val.object.get("Range")) orelse 0;
            if (pt_obj != null and range > 0) {
                const px = jsonFloat(pt_obj.?.get("X")) orelse 0;
                const py = jsonFloat(pt_obj.?.get("Y")) orelse 0;
                const pz = jsonFloat(pt_obj.?.get("Z")) orelse 0;
                // Skip placeholder (0,0,0) anchors that are only populated when bound to an entity
                if (px != 0 or py != 0 or pz != 0) {
                    const ppos_opt = player_pos_opt orelse ConditionEvaluator.currentPlayerPos3D(makeEvalContextForQuest(ctx, quest_id));
                    if (ppos_opt) |ppos| {
                        const dx = ppos[0] - px;
                        const dy = ppos[1] - py;
                        const dz = ppos[2] - pz;
                        const inside = (dx * dx + dy * dy + dz * dz) <= (range * range);
                        if (ctx.scene) |scene| {
                            const entered_marker_id = node.id + 1_000_000_000;
                            if (inside) {
                                _ = try scene.markSaveNodeEntered(ctx.alloc.gpa, quest_id, entered_marker_id);
                                if (rl_val.object.get("OverRangeCountdown") != null) {
                                    fail_queue.cancel(quest_id, node.id);
                                }
                            } else if (scene.hasSaveNodeEntered(quest_id, entered_marker_id)) {
                                const countdown = jsonFloat(rl_val.object.get("OverRangeCountdown")) orelse 0;
                                if (countdown > 0) {
                                    if (!fail_queue.contains(quest_id, node.id)) {
                                        const delay_ms: i64 = @intFromFloat(@round(@as(f64, countdown) * 1000.0));
                                        try fail_queue.upsert(ctx.alloc.gpa, .{
                                            .quest_id = quest_id,
                                            .node_id = node.id,
                                            .due_ms = nowMs(ctx.io) + delay_ms,
                                        });
                                    }
                                } else {
                                    return .OutRangeFail;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if (fc.get("CompareVar")) |cv_val| {
        if (cv_val == .object and evalCompareVarList(ctx, cv_val.object)) {
            return .CompareVarFail;
        }
    }

    if (fc.get("EntityDistanceLimiting")) |edl_val| {
        if (edl_val == .object) {
            const target_eid = jsonInt(edl_val.object.get("EntityId")) orelse 0;
            const max_dist = jsonFloat(edl_val.object.get("MaxDistance")) orelse 0;
            if (target_eid > 0 and max_dist > 0) {
                const ppos_opt = player_pos_opt orelse ConditionEvaluator.currentPlayerPos3D(makeEvalContextForQuest(ctx, quest_id));
                const epos_opt = resolveSceneOrAssetEntityPos3D(ctx, target_eid);
                if (ppos_opt != null and epos_opt != null) {
                    const ppos = ppos_opt.?;
                    const epos = epos_opt.?;
                    const dx = ppos[0] - epos[0];
                    const dy = ppos[1] - epos[1];
                    const dz = ppos[2] - epos[2];
                    const inside = (dx * dx + dy * dy + dz * dz) <= (max_dist * max_dist);
                    if (ctx.scene) |scene| {
                        const entered_marker_id = node.id + 1_500_000_000;
                        if (inside) {
                            _ = try scene.markSaveNodeEntered(ctx.alloc.gpa, quest_id, entered_marker_id);
                            if (edl_val.object.get("OverMaxCountdown") != null) {
                                fail_queue.cancel(quest_id, node.id);
                            }
                        } else if (scene.hasSaveNodeEntered(quest_id, entered_marker_id)) {
                            const countdown = jsonFloat(edl_val.object.get("OverMaxCountdown")) orelse 0;
                            if (countdown > 0) {
                                if (!fail_queue.contains(quest_id, node.id)) {
                                    const delay_ms: i64 = @intFromFloat(@round(@as(f64, countdown) * 1000.0));
                                    try fail_queue.upsert(ctx.alloc.gpa, .{
                                        .quest_id = quest_id,
                                        .node_id = node.id,
                                        .due_ms = nowMs(ctx.io) + delay_ms,
                                    });
                                }
                            } else {
                                return .OutRangeFail;
                            }
                        }
                    }
                }
            }
        }
    }

    if (fc.get("EntityDeathCondition")) |edc_val| {
        if (edc_val == .object) {
            if (edc_val.object.get("EntityIds")) |eids_val| {
                if (eids_val == .array) {
                    const aq_opt = ctx.quest_comp.info.findActiveQuest(quest_id);
                    for (eids_val.array.items) |v| {
                        const eid = jsonInt(v) orelse continue;
                        if (eid <= 0) continue;
                        if (aq_opt) |aq| {
                            if (QuestInfo.hasKilledEntity(aq, eid)) return .EntityDieFail;
                        }
                        if (entityHpPercent(ctx, eid)) |hp_pct| {
                            if (hp_pct <= 0.0) return .EntityDieFail;
                        }
                    }
                }
            }
        }
    }

    if (fc.get("EntityStateCondition")) |esc_val| {
        if (esc_val == .object) {
            if (esc_val.object.get("Conditions")) |conds_val| {
                if (conds_val == .array) {
                    for (conds_val.array.items) |item| {
                        if (item != .object) continue;
                        const eid = jsonInt(item.object.get("EntityId")) orelse continue;
                        const want_state = jsonStr(item.object.get("State")) orelse continue;
                        if (eid > 0 and entityHasStateTag(ctx, eid, want_state)) {
                            return .EntityStateFail;
                        }
                    }
                }
            }
        }
    }

    if (fc.get("CheckDataLayer")) |dl_val| {
        if (dl_val == .object) {
            const dl_id = jsonInt(dl_val.object.get("DataLayerId")) orelse 0;
            if (dl_id > 0) {
                const want_load = jsonBool(dl_val.object.get("IsLoad")) orelse true;
                const is_loaded = QuestInfo.containsInt(ctx.quest_comp.info.world.loaded_datalayers, dl_id) and
                    !QuestInfo.containsInt(ctx.quest_comp.info.world.unloaded_datalayers, dl_id);
                if (is_loaded == want_load) return .DataLayerFail;
            }
        }
    }

    if (fc.get("TimeRange")) |tr_val| {
        if (tr_val == .object) {
            const st_obj = if (tr_val.object.get("StartTime")) |v| (if (v == .object) v.object else null) else null;
            const et_obj = if (tr_val.object.get("EndTime")) |v| (if (v == .object) v.object else null) else null;
            if (st_obj != null and et_obj != null) {
                const sh = jsonInt(st_obj.?.get("Hours")) orelse 0;
                const sm = jsonInt(st_obj.?.get("Minutes")) orelse 0;
                const eh = jsonInt(et_obj.?.get("Hours")) orelse 0;
                const em = jsonInt(et_obj.?.get("Minutes")) orelse 0;
                const start_m = sh * 60 + sm;
                const end_m = eh * 60 + em;
                if (start_m != end_m) {
                    const ch: i32 = if (ctx.quest_comp.info.env.time_locked)
                        ctx.quest_comp.info.env.time_lock_hour
                    else if (ctx.scene) |s|
                        s.instance.map_time.hour
                    else
                        ctx.quest_comp.info.env.time_lock_hour;
                    const cm: i32 = if (ctx.quest_comp.info.env.time_locked)
                        ctx.quest_comp.info.env.time_lock_minute
                    else if (ctx.scene) |s|
                        s.instance.map_time.minute
                    else
                        ctx.quest_comp.info.env.time_lock_minute;
                    const cur_m = ch * 60 + cm;
                    const in_window = if (start_m < end_m)
                        (cur_m >= start_m and cur_m <= end_m)
                    else
                        (cur_m >= start_m or cur_m <= end_m);
                    if (!in_window) return .TimeRange;
                }
            }
        }
    }

    if (fc.get("PlayerMotionStateCondition")) |pm_val| {
        if (pm_val == .object) {
            if (ctx.scene) |scene| {
                if (scene.has_player_motion) {
                    const motion = jsonStr(pm_val.object.get("MotionState")) orelse "";
                    const cmp = jsonStr(pm_val.object.get("Compare")) orelse "Eq";
                    if (ConditionEvaluator.evalPlayerMotionState(makeEvalContextForQuest(ctx, quest_id), motion)) |matched| {
                        const fail = if (std.mem.eql(u8, cmp, "Ne") or std.mem.eql(u8, cmp, "NotEq")) !matched else matched;
                        if (fail) return .PlayerMotionState;
                    }
                }
            }
        }
    }

    if (fc.get("PlayerMovementModeCondition")) |pmm_val| {
        if (pmm_val == .object) {
            if (ctx.scene) |scene| {
                if (scene.has_player_movement_mode) {
                    const mode = jsonStr(pmm_val.object.get("Mode")) orelse "";
                    const cmp = jsonStr(pmm_val.object.get("Compare")) orelse "Eq";
                    // Special modes (Rail, Ski, WingSuitFly) use UE custom movement modes (>= 5)
                    const in_mode = if (std.mem.eql(u8, mode, "WingSuitFly"))
                        (scene.player_movement_mode == 5 or scene.player_movement_mode >= 6)
                    else if (std.mem.eql(u8, mode, "Rail") or std.mem.eql(u8, mode, "Ski"))
                        (scene.player_movement_mode >= 6)
                    else
                        false;
                    const fail = if (std.mem.eql(u8, cmp, "Ne") or std.mem.eql(u8, cmp, "NotEq")) !in_mode else in_mode;
                    if (fail) return .PlayerLevelPlayMoveStateFail;
                }
            }
        }
    }

    if ((jsonBool(fc.get("IsTeamDeathFailure")) orelse false) or
        (jsonBool(fc.get("IsNonAiTeamDeathFailure")) orelse false))
    {
        if (ctx.scene) |scene| {
            const form_idx = std.math.cast(usize, scene.formation_info.cur_formation);
            if (form_idx) |fi| {
                if (fi < scene.formation_info.formations.len) {
                    const form = scene.formation_info.formations[fi];
                    const cur_eid = scene.currentRoleEntityId();
                    if (cur_eid > 0 and RoleHelper.formationLivingStatus(scene, form) == .Dead) {
                        return .CharacterDieFail;
                    }
                }
            }
        }
    }

    if (fc.get("CheckDungeonFailure")) |cdf_val| {
        if (cdf_val == .array) {
            if (ctx.scene) |scene| {
                for (cdf_val.array.items) |v| {
                    const did = jsonInt(v) orelse continue;
                    if (did > 0 and scene.hasFailedDungeon(did)) return .InstFail;
                }
            }
        }
    }

    if (fc.get("EntityAlert")) |ea_val| {
        if (ea_val == .object) {
            if (ea_val.object.get("EntityIds")) |eids_val| {
                if (eids_val == .array) {
                    if (ctx.scene) |scene| {
                        const slice = scene.entities.slice();
                        const configs = slice.items(.config);
                        const net_ids = slice.items(.entity_id);
                        for (eids_val.array.items) |v| {
                            const target_cfg = jsonInt(v) orelse continue;
                            if (target_cfg <= 0) continue;
                            for (configs, net_ids) |cfg, nid| {
                                if (cfg.config_id == target_cfg) {
                                    if (scene.fsm_hate_states.get(nid.net_id) orelse false) {
                                        return .DefaultFail;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if (fc.get("SneakPlayCondition")) |sp_val| {
        if (sp_val == .object) {
            if (ctx.scene) |scene| {
                if (scene.active_sneak_id > 0) {
                    const secs = jsonFloat(sp_val.object.get("Time")) orelse 0;
                    if (std.math.isFinite(secs) and secs > 0 and !fail_queue.contains(quest_id, node.id)) {
                        const delay_ms: i64 = @intFromFloat(@round(@as(f64, secs) * 1000.0));
                        try fail_queue.upsert(ctx.alloc.gpa, .{
                            .quest_id = quest_id,
                            .node_id = node.id,
                            .due_ms = nowMs(ctx.io) + delay_ms,
                        });
                    }
                }
            }
        }
    }

    return null;
}

fn resolveSceneOrAssetEntityPos3D(ctx: QuestContext, cfg_id: i32) ?[3]f32 {
    if (cfg_id <= 0) return null;
    if (ctx.scene) |scene| {
        const slice = scene.entities.slice();
        for (slice.items(.config), slice.items(.position)) |cfg, pos_opt| {
            if (cfg.config_id == cfg_id) {
                if (pos_opt) |p| return p.location;
            }
        }
        if (ctx.assets.findLevelEntity(scene.instance_id, cfg_id)) |lcfg| {
            if (Assets.AssetIndexes.entityWorldPos(lcfg)) |wp| return wp;
        }
    }
    return null;
}

fn extendFailTimersOnKillInQueue(
    ctx: QuestContext,
    timers: *Scene.QuestTimerQueue,
    store: LazyNodeStore,
) !bool {
    var extended = false;
    const items = try ctx.alloc.arena.dupe(@TypeOf(timers.heap.items[0]), timers.heap.items);
    for (items) |entry| {
        const nodes = store.getOwnerNodes(ctx.io, entry.quest_id) catch continue;
        const node = findNodeById(nodes, entry.node_id) orelse continue;
        if (node.data != .object) continue;
        const fc_val = node.data.object.get("FailedCondition") orelse continue;
        if (fc_val != .object) continue;
        const atc = fc_val.object.get("AddTimeConfig") orelse continue;
        if (atc != .object) continue;
        const mc = atc.object.get("MonsterConfig") orelse continue;
        if (mc != .object) continue;
        const add_secs = jsonFloat(mc.object.get("NormalAddTime")) orelse 0;
        if (add_secs <= 0) continue;
        const new_due = entry.due_ms + @as(i64, @intFromFloat(@round(@as(f64, add_secs) * 1000.0)));
        try timers.upsert(ctx.alloc.gpa, .{
            .quest_id = entry.quest_id,
            .node_id = entry.node_id,
            .due_ms = new_due,
        });
        extended = true;
    }
    return extended;
}

fn extendFailTimersOnKill(ctx: QuestContext) !bool {
    const scene = ctx.scene orelse return false;
    var extended = try extendFailTimersOnKillInQueue(ctx, &scene.fail_timers, ctx.assets.quest_nodes);
    if (try extendFailTimersOnKillInQueue(ctx, &scene.lp_fail_timers, ctx.assets.level_play_nodes)) {
        extended = true;
    }
    return extended;
}

fn extendFailTimersOnEntityStateInQueue(
    ctx: QuestContext,
    timers: *Scene.QuestTimerQueue,
    store: LazyNodeStore,
    entity_config_id: i32,
    state_name: []const u8,
) !bool {
    var extended = false;
    const items = try ctx.alloc.arena.dupe(@TypeOf(timers.heap.items[0]), timers.heap.items);
    for (items) |entry| {
        const nodes = store.getOwnerNodes(ctx.io, entry.quest_id) catch continue;
        const node = findNodeById(nodes, entry.node_id) orelse continue;
        if (node.data != .object) continue;
        const fc_val = node.data.object.get("FailedCondition") orelse continue;
        if (fc_val != .object) continue;
        const atc = fc_val.object.get("AddTimeConfig") orelse continue;
        if (atc != .object) continue;
        const sc_list = atc.object.get("StateConfig") orelse continue;
        if (sc_list != .array) continue;
        var add_secs: f32 = 0;
        for (sc_list.array.items) |sc_item| {
            if (sc_item != .object) continue;
            const cond = sc_item.object.get("Condition") orelse continue;
            if (cond != .object) continue;
            const eid = jsonInt(cond.object.get("EntityId")) orelse 0;
            const want_st = jsonStr(cond.object.get("State")) orelse "";
            if ((eid == 0 or eid == entity_config_id) and (want_st.len == 0 or std.mem.eql(u8, want_st, state_name))) {
                add_secs += jsonFloat(sc_item.object.get("AddTime")) orelse 0;
            }
        }
        if (add_secs <= 0) continue;
        const new_due = entry.due_ms + @as(i64, @intFromFloat(@round(@as(f64, add_secs) * 1000.0)));
        try timers.upsert(ctx.alloc.gpa, .{
            .quest_id = entry.quest_id,
            .node_id = entry.node_id,
            .due_ms = new_due,
        });
        extended = true;
    }
    return extended;
}

fn extendFailTimersOnEntityState(ctx: QuestContext, entity_config_id: i32, state_name: []const u8) !bool {
    const scene = ctx.scene orelse return false;
    var extended = try extendFailTimersOnEntityStateInQueue(ctx, &scene.fail_timers, ctx.assets.quest_nodes, entity_config_id, state_name);
    if (try extendFailTimersOnEntityStateInQueue(ctx, &scene.lp_fail_timers, ctx.assets.level_play_nodes, entity_config_id, state_name)) {
        extended = true;
    }
    return extended;
}

fn stepQuestFailedNode(
    ctx: QuestContext,
    quest_id: i32,
    nodes: []const LazyNodeStore.NodeEntry,
    node: LazyNodeStore.NodeEntry,
    push_updates: bool,
) !StepResult {
    _ = nodes;
    const scene = ctx.scene orelse return .running;
    if (try evalQuestFailedCondition(ctx, quest_id, node, null, &scene.fail_timers)) |reason| {
        try rollbackQuestToNode(ctx, quest_id, node.id, reason, !push_updates);
        return if (ctx.quest_comp.info.hasActiveQuest(quest_id)) .running else .quest_ended;
    }
    return .running;
}

fn isSubtreeActiveForMonitor(
    nodes: []const LazyNodeStore.NodeEntry,
    active_nodes: []const i32,
    parent_id: i32,
) bool {
    if (parent_id <= 0) return true;
    for (active_nodes) |nid| {
        var cur = nid;
        var hops: usize = 0;
        while (cur > 0 and hops < 32) : (hops += 1) {
            if (cur == parent_id) return true;
            const n = findNodeById(nodes, cur) orelse break;
            cur = n.parent_id;
        }
    }
    return false;
}

fn checkActiveQuestFailedMonitors(ctx: QuestContext, pos_opt: ?[3]f32) !bool {
    const aq_snapshot = try ctx.alloc.arena.dupe(QuestInfo.ActiveQuest, ctx.quest_comp.info.active_quests);
    var changed = false;
    for (aq_snapshot) |aq| {
        const nodes = ctx.assets.quest_nodes.getOwnerNodes(ctx.io, aq.quest_id) catch continue;
        for (nodes) |node| {
            if (!std.mem.eql(u8, node.node_type, "QuestFailed")) continue;
            if (QuestInfo.isNodeFinished(aq, node.id)) continue;
            if (!isSubtreeActiveForMonitor(nodes, aq.active_nodes, node.parent_id)) continue;
            if (try evalQuestFailedCondition(ctx, aq.quest_id, node, pos_opt, if (ctx.scene) |s| &s.fail_timers else return false)) |reason| {
                try rollbackQuestToNode(ctx, aq.quest_id, node.id, reason, false);
                changed = true;
                break;
            }
        }
    }
    return changed;
}

fn rollbackQuestToNode(
    ctx: QuestContext,
    quest_id: i32,
    fail_node_id: i32,
    reason: pb.FailReason,
    client_initiated: bool,
) anyerror!void {
    const aq_ptr = ctx.quest_comp.info.findActiveQuestPtr(quest_id) orelse return;
    const nodes = ctx.assets.quest_nodes.getOwnerNodes(ctx.io, quest_id) catch return;

    if (ctx.scene) |scene| {
        scene.quest_timers.cancelQuest(quest_id);
        scene.fail_timers.cancelQuest(quest_id);
        scene.releaseOccupationsForTree(ctx.alloc.gpa, quest_id, quest_id);
        scene.clearSaveNodesForOwner(quest_id);
        scene.clearActionNodeProgressForOwner(quest_id);
        if (!client_initiated and reason != .GiveUpFail) {
            scene.last_server_rollback_quest_id = quest_id;
            scene.last_server_rollback_ms = nowMs(ctx.io);
        }
        _ = interruptActiveFlowForQuest(ctx, quest_id);
    }

    var has_pos_rollback = false;
    var fail_node_opt: ?*const LazyNodeStore.NodeEntry = null;
    if (fail_node_id > 0) {
        fail_node_opt = findNodeById(nodes, fail_node_id);
        if (fail_node_opt) |fn_entry| {
            if (fn_entry.data == .object) {
                if (fn_entry.data.object.get("FailedCondition")) |fc_val| {
                    if (fc_val == .object and fc_val.object.get("FailedTeleport") != null) {
                        has_pos_rollback = true;
                    }
                }
            }
        }
    }

    if (!client_initiated) {
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.BtRollbackStartNotify{
            .TreeOwnerId = quest_id,
            .TreeIncId = quest_id,
            .IsRollbackSubLevel = false,
            .IsRollbackPos = has_pos_rollback,
        }) catch {};

        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.BtRollbackNotify{
            .TreeOwnerId = quest_id,
            .TreeIncId = quest_id,
            .FailNodeId = fail_node_id,
            .FailReason = reason,
            .BtRollbackInfo = .{
                .FailRollbackData = .{
                    .FailNodeId = fail_node_id,
                    .FailReason = reason,
                },
            },
        }) catch {};
    }

    if (fail_node_opt) |fn_entry| {
        try runNodeFinishEffects(ctx, quest_id, fn_entry.*);
    }

    // Find latest finished SaveConfig node to preserve prior progress; clear post-checkpoint state
    var latest_ckpt_id: i32 = 0;
    for (aq_ptr.finished_nodes) |fid| {
        const fnode = findNodeById(nodes, fid) orelse continue;
        if (fnode.data == .object and fnode.data.object.get("SaveConfig") != null) {
            if (fid > latest_ckpt_id) latest_ckpt_id = fid;
        }
    }

    const active_copy = try ctx.alloc.arena.dupe(i32, aq_ptr.active_nodes);
    for (active_copy) |aid| {
        removeDoInteractOptionsForNode(ctx, quest_id, aid);
        _ = try QuestInfo.markNodeInactive(aq_ptr, ctx.alloc.gpa, aid);
    }
    if (aq_ptr.killed_entities.len != 0) {
        ctx.alloc.gpa.free(aq_ptr.killed_entities);
        aq_ptr.killed_entities = &.{};
    }
    if (latest_ckpt_id > 0) {
        const fin_copy = try ctx.alloc.arena.dupe(i32, aq_ptr.finished_nodes);
        for (fin_copy) |fid| {
            if (fid >= latest_ckpt_id) {
                _ = try QuestInfo.removeIntValue(&aq_ptr.finished_nodes, ctx.alloc.gpa, fid);
            }
        }
    } else {
        while (aq_ptr.finished_nodes.len > 0) {
            _ = try QuestInfo.removeIntValue(&aq_ptr.finished_nodes, ctx.alloc.gpa, aq_ptr.finished_nodes[aq_ptr.finished_nodes.len - 1]);
        }
    }

    const tree_info = try buildTreeInfo(ctx, aq_ptr.*);
    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.BtRollbackInfoNotify{
        .TreeOwnerId = quest_id,
        .TreeIncId = quest_id,
        .TreeInfo = tree_info,
    }) catch {};

    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.QuestRollBackEndNotify{
        .TreeIncId = quest_id,
        .QuestId = quest_id,
    }) catch {};

    try advanceQuestTree(ctx, quest_id, true);
}

fn succeedQuest(ctx: QuestContext, quest_id: i32) anyerror!void {
    if (quest_transition_depth >= max_transition_depth) return;
    if (!ctx.quest_comp.info.hasActiveQuest(quest_id)) return;
    if (ctx.scene) |scene| {
        scene.quest_timers.cancelQuest(quest_id);
        scene.fail_timers.cancelQuest(quest_id);
        scene.releaseOccupationsForTree(ctx.alloc.gpa, quest_id, quest_id);
        scene.clearSaveNodesForOwner(quest_id);
        scene.clearActionNodeProgressForOwner(quest_id);
    }

    quest_transition_depth += 1;
    defer quest_transition_depth -= 1;

    const quest_opt = ctx.assets.tables.quest_data.getDataById(quest_id);
    if (quest_opt) |q| {
        if (q.Data.RewardId > 0) {
            _ = try RewardService.grantDropPackage(makeRewardContext(ctx), q.Data.RewardId, 6003);
        }
    }

    try ctx.quest_comp.info.finishQuest(ctx.alloc.gpa, quest_id);

    if (quest_opt) |q| {
        try executeActions(ctx, quest_id, 0, q.Data.FinishActions);
    }

    var fl: pb.QuestFinishListNotify = .default;
    try fl.QuestId.append(ctx.alloc.arena, quest_id);
    ctx.conn.push(ctx.alloc.gpa, ctx.io, fl) catch {};

    var ql: pb.QuestListNotify = .default;
    try ql.Quests.append(ctx.alloc.arena, .{ .QuestId = quest_id, .Status = .Finish });
    ctx.conn.push(ctx.alloc.gpa, ctx.io, ql) catch {};

    if (QuestInfo.containsInt(ctx.quest_comp.info.daily.daily_quests, quest_id)) {
        try pushDailyAndBranchNotifies(ctx);
    }

    // Re-evaluate newly unlocked quests and dynamic FuncOpen flags
    _ = try refreshAvailableQuests(ctx, true);
    _ = try recheckPendingConditions(ctx);
    if (quest_transition_depth <= 1 and refresh_loop_depth == 0) {
        try QuestActions.pushFuncOpen(ctx);
    }
}

fn resetQuestSubtreeFinishedNodes(
    ctx: QuestContext,
    quest_id: i32,
    nodes: []const LazyNodeStore.NodeEntry,
    parent_id: i32,
) !void {
    const aq_ptr = ctx.quest_comp.info.findActiveQuestPtr(quest_id) orelse return;
    for (nodes) |n| {
        if (n.parent_id != parent_id) continue;
        _ = try QuestInfo.removeIntValue(&aq_ptr.finished_nodes, ctx.alloc.gpa, n.id);
        _ = try QuestInfo.markNodeInactive(aq_ptr, ctx.alloc.gpa, n.id);
        try resetQuestSubtreeFinishedNodes(ctx, quest_id, nodes, n.id);
    }
}

fn deactivateSubtree(
    ctx: QuestContext,
    quest_id: i32,
    nodes: []const LazyNodeStore.NodeEntry,
    parent_id: i32,
    push_updates: bool,
) !void {
    const aq_ptr = ctx.quest_comp.info.findActiveQuestPtr(quest_id) orelse return;
    for (nodes) |n| {
        if (n.parent_id != parent_id) continue;
        if (ctx.scene) |scene| {
            scene.quest_timers.cancel(quest_id, n.id);
            scene.fail_timers.cancel(quest_id, n.id);
            scene.releaseOccupationsForNode(ctx.alloc.gpa, quest_id, n.id);
        }
        removeDoInteractOptionsForNode(ctx, quest_id, n.id);
        const was_active = try QuestInfo.markNodeInactive(aq_ptr, ctx.alloc.gpa, n.id);
        const is_child_quest = std.mem.eql(u8, n.node_type, "ChildQuest");
        if (push_updates) {
            if (was_active and is_child_quest) {
                try pushNodeStatus(ctx, quest_id, n.id, .Destroy, .NotActive);
            } else if (was_active or !is_child_quest) {
                try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.UpdateNodeStatusNotify{
                    .TreeOwnerId = quest_id,
                    .TreeIncId = quest_id,
                    .NodeId = n.id,
                    .Status = .Destroy,
                });
            }
        }
        try deactivateSubtree(ctx, quest_id, nodes, n.id, push_updates);
    }
}

fn executeActions(
    ctx: QuestContext,
    quest_id: i32,
    node_id: i32,
    actions: []const std.json.Value,
) anyerror!void {
    try QuestActions.executeActionList(ctx, actionCallbacks(), quest_id, node_id, actions);
}

/// Temporarily binds an entity as the action owner, runs `actions`, and persists quest state.
pub fn executeEntityActions(
    ctx: QuestContext,
    entity_id: i64,
    cfg_id: i32,
    actions: []const std.json.Value,
) !void {
    if (actions.len == 0) return;

    var owner_nid: i64 = entity_id;
    const prev_owner: i64 = if (ctx.scene) |scene| blk: {
        const prev = scene.active_flow_owner_entity_id;
        if (!scene.net_id_map.contains(owner_nid)) {
            const slice = scene.entities.slice();
            for (slice.items(.config), slice.items(.entity_id)) |cfg, nid| {
                if (cfg.config_id == cfg_id) {
                    owner_nid = nid.net_id;
                    break;
                }
            }
        }
        scene.active_flow_owner_entity_id = owner_nid;
        break :blk prev;
    } else 0;
    defer if (ctx.scene) |scene| {
        scene.active_flow_owner_entity_id = prev_owner;
    };

    try executeActions(ctx, 0, 0, actions);
    try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);
}

threadlocal var reevaluating_range_triggers: bool = false;

/// Tracks runtime player overlap with a `TriggerComponent` entity and runs
/// enter/exit trigger actions once its condition is satisfied.
pub fn handleEntityRangeTrigger(
    ctx: QuestContext,
    entity_id: i64,
    cfg_id: i32,
    trig: TriggerConfig,
    is_enter: bool,
) !bool {
    const scene = ctx.scene orelse return false;
    const gop = try scene.player_range_states.getOrPut(ctx.alloc.gpa, entity_id);
    if (!gop.found_existing) gop.value_ptr.* = .{};

    var progress_dirty = false;
    if (is_enter) {
        gop.value_ptr.in_range = true;
        if (!gop.value_ptr.enter_fired) {
            const max_times = trig.MaxTriggerTimes orelse 0;
            var can_fire = true;
            var cnt_key: []const u8 = "";
            var cur_cnt: i64 = 0;
            if (max_times > 0 and cfg_id > 0) {
                if (ctx.progress_comp) |pc| {
                    cnt_key = try std.fmt.allocPrint(ctx.alloc.arena, "trig:cnt:{d}", .{cfg_id});
                    cur_cnt = pc.info.getKeyInt(cnt_key);
                    if (cur_cnt >= max_times) can_fire = false;
                }
            }
            if (can_fire and try executeTriggerActions(ctx, entity_id, cfg_id, trig.Condition, trig.Actions)) {
                gop.value_ptr.enter_fired = true;
                if (max_times > 0 and cfg_id > 0) {
                    if (ctx.progress_comp) |pc| {
                        try pc.info.setKeyInt(ctx.alloc.gpa, cnt_key, cur_cnt + 1);
                        progress_dirty = true;
                    }
                }
            }
        }
    } else {
        const was_in_range = gop.value_ptr.in_range;
        gop.value_ptr.in_range = false;
        gop.value_ptr.enter_fired = false;
        if (was_in_range and trig.ExitConfig == .object) {
            const exit_obj = trig.ExitConfig.object;
            const exit_cond = exit_obj.get("Condition") orelse .null;
            const exit_acts = exit_obj.get("Actions") orelse .null;
            _ = try executeTriggerActions(ctx, entity_id, cfg_id, exit_cond, exit_acts);
        }
    }
    return progress_dirty;
}

/// Re-evaluates `TriggerComponent` entities the player is currently standing inside
/// whose enter condition was not yet met (e.g. door unlocked after dialogue/quest update).
pub fn reevaluateInRangeTriggers(ctx: QuestContext) !void {
    if (reevaluating_range_triggers) return;
    const scene = ctx.scene orelse return;
    if (scene.player_range_states.count() == 0) return;

    reevaluating_range_triggers = true;
    defer reevaluating_range_triggers = false;

    var pending_ids: std.ArrayList(i64) = .empty;
    var it = scene.player_range_states.iterator();
    while (it.next()) |entry| {
        if (entry.value_ptr.in_range and !entry.value_ptr.enter_fired) {
            try pending_ids.append(ctx.alloc.arena, entry.key_ptr.*);
        }
    }
    if (pending_ids.items.len == 0) return;

    var dirty = false;
    for (pending_ids.items) |eid| {
        const resolved = scene.findConfigIdByNetId(eid) orelse eid;
        const cfg_id = std.math.cast(i32, resolved) orelse continue;
        const lcfg = ctx.assets.findLevelEntity(scene.instance_id, resolved) orelse continue;
        var components = lcfg.ComponentsData;
        if (ctx.assets.findTemplateConfig(lcfg.BlueprintType)) |tpl| {
            tpl.ComponentsData.mergeInto(&components);
        }
        const trig = components.TriggerComponent orelse continue;
        if (trig.Disabled orelse false) continue;
        if (try handleEntityRangeTrigger(ctx, eid, cfg_id, trig, true)) {
            dirty = true;
        }
    }
    if (dirty) {
        if (ctx.progress_comp) |pc| {
            try pc.save(ctx.fs, ctx.alloc.arena);
        }
    }
}

pub fn executeTriggerActions(
    ctx: QuestContext,
    entity_id: i64,
    cfg_id: i32,
    cond_val: std.json.Value,
    actions_val: std.json.Value,
) !bool {
    if (actions_val != .array or actions_val.array.items.len == 0) return false;
    if (cond_val == .object) {
        var eval_ctx = makeEvalContext(ctx);
        eval_ctx.self_entity_config_id = cfg_id;
        if (!ConditionEvaluator.evalInlineConditionBlock(eval_ctx, cond_val)) return false;
    }

    try executeEntityActions(ctx, entity_id, cfg_id, actions_val.array.items);
    return true;
}

fn completeGuideById(ctx: QuestContext, guide_id: i32) anyerror!void {
    if (guide_id <= 0) return;
    if (ctx.guide_comp) |gc| {
        try gc.info.addFinished(ctx.alloc.gpa, guide_id);
        const path = try std.fmt.allocPrint(ctx.alloc.arena, "player/{}/{s}", .{
            gc.player_id,
            GuideInfo.data_path,
        });
        try comp_util.saveStruct(ctx.fs, gc.info, path, ctx.alloc.arena);
    }
    _ = try completeMatchingChildQuests(ctx, "Guide", "GuideGroupId", guide_id);
}

fn actionNodeProgressKey(quest_id: i32, node_id: i32) u64 {
    return (@as(u64, @as(u32, @bitCast(quest_id))) << 32) | @as(u64, @as(u32, @bitCast(node_id)));
}

fn extractNodeActionItems(data: std.json.Value, single_buf: *[1]std.json.Value) ?[]const std.json.Value {
    if (data != .object) return null;
    if (data.object.get("Actions")) |acts_val| {
        return if (acts_val == .array) acts_val.array.items else null;
    }
    if (data.object.get("Action")) |single_act| {
        if (single_act != .object) return null;
        single_buf[0] = single_act;
        return single_buf[0..1];
    }
    return null;
}

fn countNodeActions(node: LazyNodeStore.NodeEntry) usize {
    var single_buf: [1]std.json.Value = undefined;
    const items = extractNodeActionItems(node.data, &single_buf) orelse return 0;
    return items.len;
}

fn isPlayFlowActionAsync(act: std.json.Value) bool {
    if (act != .object) return false;
    if (jsonBool(act.object.get("Async"))) |b| return b;
    if (act.object.get("Params")) |pv| {
        if (pv == .object) {
            if (jsonBool(pv.object.get("Async"))) |b| return b;
        }
    }
    return false;
}

fn isSyncPlayFlowAction(act: std.json.Value) bool {
    if (act != .object) return false;
    const name = jsonStr(act.object.get("Name")) orelse return false;
    if (!std.mem.eql(u8, name, "PlayFlow")) return false;
    return !isPlayFlowActionAsync(act);
}

pub fn hasAsyncPlayFlowAction(items: []const std.json.Value) bool {
    var saw_play_flow = false;
    for (items) |act| {
        if (act != .object) continue;
        const name = jsonStr(act.object.get("Name")) orelse continue;
        if (!std.mem.eql(u8, name, "PlayFlow")) continue;
        saw_play_flow = true;
        if (!isPlayFlowActionAsync(act)) return false;
    }
    return saw_play_flow;
}

fn startFlowFromObj(
    ctx: QuestContext,
    quest_id: i32,
    node_id: i32,
    obj: std.json.ObjectMap,
) !void {
    try startFlowWithMode(ctx, quest_id, node_id, obj, true, null);
}

fn startFlowWithMode(
    ctx: QuestContext,
    quest_id: i32,
    node_id: i32,
    obj: std.json.ObjectMap,
    enqueue_if_busy: bool,
    is_async_override: ?bool,
) !void {
    const flow_name = jsonStr(obj.get("FlowListName")) orelse return;
    if (flow_name.len == 0) return;
    const flow_id = jsonInt(obj.get("FlowId")) orelse 1;
    const state_id = jsonInt(obj.get("StateId")) orelse 1;
    const is_async = is_async_override orelse (jsonBool(obj.get("Async")) orelse false);
    const bt_type: pb.BtType = if (quest_id > 0 and
        ctx.quest_comp.info.findActiveQuest(quest_id) == null and
        ctx.assets.tables.level_play_data.getDataById(quest_id) != null)
        .BtTypeLevelPlay
    else
        .BtTypeQuest;
    try startFlowParamsBt(ctx, quest_id, node_id, bt_type, flow_name, flow_id, state_id, enqueue_if_busy, is_async);
}

fn startFlowParams(
    ctx: QuestContext,
    quest_id: i32,
    node_id: i32,
    flow_name: []const u8,
    flow_id: i32,
    state_id: i32,
    enqueue_if_busy: bool,
) !void {
    const bt_type: pb.BtType = if (quest_id > 0 and
        ctx.quest_comp.info.findActiveQuest(quest_id) == null and
        ctx.assets.tables.level_play_data.getDataById(quest_id) != null)
        .BtTypeLevelPlay
    else
        .BtTypeQuest;
    try startFlowParamsBt(ctx, quest_id, node_id, bt_type, flow_name, flow_id, state_id, enqueue_if_busy, false);
}

fn startFlowParamsBt(
    ctx: QuestContext,
    quest_id: i32,
    node_id: i32,
    bt_type: pb.BtType,
    flow_name: []const u8,
    flow_id: i32,
    state_id: i32,
    enqueue_if_busy: bool,
    is_async: bool,
) !void {
    if (ctx.scene) |scene| {
        if (!is_async and scene.active_flow_inc_id != 0) {
            if (enqueue_if_busy) {
                try scene.enqueuePendingFlow(ctx.alloc.gpa, quest_id, node_id, flow_name, flow_id, state_id);
            }
            return;
        }
    }

    const inc_id = ctx.quest_comp.info.allocFlowIncId();
    if (!is_async) {
        if (ctx.scene) |scene| {
            try scene.setActiveFlow(ctx.alloc.gpa, inc_id, quest_id, node_id, flow_name, flow_id, state_id);
            scene.active_flow_bt_type = bt_type;
            prepareFlowStateEntities(ctx, flow_name, flow_id, state_id);
        }
    } else if (ctx.scene) |scene| {
        try scene.recordAsyncFlow(ctx.alloc.gpa, inc_id, quest_id, node_id, bt_type, flow_name, flow_id, state_id, 0);
        prepareFlowStateEntities(ctx, flow_name, flow_id, state_id);
    }

    const bt_inc_id: i64 = if (quest_id != 0) quest_id else inc_id;
    try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.FlowStartNotify{
        .FlowIncId = inc_id,
        .FlowListName = flow_name,
        .FlowId = flow_id,
        .StateId = state_id,
        .aAsync = is_async,
        .GameCtx = .{
            .CtxType = .BehaviorTree,
            .CtxInfo = .{ .BehaviorTree = .{
                .IncId = bt_inc_id,
                .BtType = bt_type,
                .BtId = quest_id,
                .NodeId = node_id,
            } },
        },
    });
}

fn prepareFlowStateEntities(ctx: QuestContext, flow_name: []const u8, flow_id: i32, state_id: i32) void {
    const state_val = (ctx.assets.flow_states.getFlowState(
        ctx.io,
        ctx.alloc.arena,
        flow_name,
        flow_id,
        state_id,
    ) catch return) orelse return;

    var awake_ids: std.ArrayList(i32) = .empty;
    Assets.LazyFlowStore.collectAwakeEntityIds(ctx.alloc.arena, state_val, &awake_ids) catch {};
    for (awake_ids.items) |eid| {
        QuestActions.ensureQuestEntityAwakened(ctx, eid);
    }
    if (awake_ids.items.len > 0) {
        QuestActions.sendQuestEntityVisibility(ctx, awake_ids.items, true) catch {};
    }

    var entity_ids: std.ArrayList(i32) = .empty;
    Assets.LazyFlowStore.collectSetEntityVisibleIds(ctx.alloc.arena, state_val, &entity_ids) catch return;
    for (entity_ids.items) |eid| {
        QuestActions.ensureQuestEntitySpawned(ctx, eid);
    }

    if (Assets.LazyFlowStore.doesFlowSwitchMainRole(state_val)) {
        ensurePlotMainRole(ctx);
    }
}

fn pushFormationGroupUpdate(ctx: QuestContext, scene: *Scene, cur_form: anytype) void {
    var fight_role_infos: std.ArrayList(pb.FightRoleInfo) = .empty;
    for (cur_form.roles) |maybe_slot| {
        const slot = maybe_slot orelse continue;
        fight_role_infos.append(ctx.alloc.arena, .{
            .RoleId = slot.role_id,
            .EntityId = slot.entity_id,
            .OnStageWithoutControl = false,
        }) catch return;
    }

    var fight_role_infos_wrapper: std.ArrayList(pb.FightRoleInfos) = .empty;
    fight_role_infos_wrapper.append(ctx.alloc.arena, .{
        .GroupType = 1,
        .FightRoleInfos = fight_role_infos,
        .CurRole = cur_form.cur_role,
        .LivingStatus = RoleHelper.formationLivingStatus(scene, cur_form.*),
        .IsFixedLocation = false,
    }) catch return;

    var group_formations: std.ArrayList(pb.GroupFormation) = .empty;
    group_formations.append(ctx.alloc.arena, .{
        .PlayerId = scene.player_id,
        .FightRoleInfos = fight_role_infos_wrapper,
        .CurrentGroupType = 1,
    }) catch return;

    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.UpdateGroupFormationNotify{
        .GroupFormation = group_formations,
    }) catch {};
}

fn setFormationVisibility(scene: *Scene, desired_role_id: i32) void {
    const slice = scene.entities.slice();
    for (slice.items(.config), 0..) |config, i| {
        if (config.entity_type != .player or config.config_type != .character) continue;
        if (config.config_id == desired_role_id) {
            slice.items(.visible)[i] = .{};
        } else {
            slice.items(.visible)[i] = null;
        }
    }
}

fn removeRoleEntityAndConcomitants(ctx: QuestContext, scene: *Scene, entity_id: i64) void {
    if (entity_id <= 0) return;
    const idx = scene.net_id_map.get(entity_id) orelse return;
    var remove_infos: std.ArrayList(pb.EntityRemoveInfo) = .empty;
    const storage = scene.entities.get(idx);
    if (storage.concomitant) |concomitant| {
        for (concomitant.vision_entity_id) |cid| {
            scene.remove(ctx.alloc.gpa, ctx.fs, cid) catch {};
            remove_infos.append(ctx.alloc.arena, .{ .EntityId = cid }) catch {};
        }
        for (concomitant.custom_entity_ids) |cid| {
            scene.remove(ctx.alloc.gpa, ctx.fs, cid) catch {};
            remove_infos.append(ctx.alloc.arena, .{ .EntityId = cid }) catch {};
        }
    }
    scene.remove(ctx.alloc.gpa, ctx.fs, entity_id) catch {};
    remove_infos.append(ctx.alloc.arena, .{ .EntityId = entity_id }) catch {};
    if (remove_infos.items.len > 0) {
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.EntityRemoveNotify{
            .RemoveInfos = remove_infos,
            .IsRemove = true,
        }) catch {};
    }
}

fn ensurePlotMainRole(ctx: QuestContext) void {
    const scene = ctx.scene orelse return;
    if (scene.plot_added_rover_slot != null) return;
    const role_comp = ctx.role_comp orelse return;
    const gender = ctx.basic_comp.info.attributes.sex;
    if (RoleHelper.currentFormationMainRoleId(ctx.assets, scene, gender) != null) return;

    const main_role_id = RoleHelper.selectedMainRoleId(ctx.assets, role_comp, gender, 0) orelse
        RoleHelper.mainRoleIdForElement(ctx.assets, gender, 1) orelse return;
    if (!role_comp.role_map.contains(main_role_id)) return;
    if (scene.formation_info.cur_formation < 0) return;
    const cur_idx: usize = @intCast(scene.formation_info.cur_formation);
    if (cur_idx >= scene.formation_info.formations.len) return;
    const cur_form = &scene.formation_info.formations[cur_idx];

    const inst_dungeon = ctx.assets.tables.instance_dungeon.getDataById(scene.instance_id) orelse
        ctx.assets.tables.instance_dungeon.getDataById(8) orelse return;

    var target_slot: usize = 0;
    var found_empty = false;
    for (cur_form.roles, 0..) |maybe_role, i| {
        if (maybe_role == null) {
            target_slot = i;
            found_empty = true;
            break;
        }
    }

    const saved_cur = cur_form.cur_role;
    const saved_slot_role = if (!found_empty and cur_form.roles[target_slot] != null) cur_form.roles[target_slot].?.role_id else 0;
    const saved_slot_eid = if (!found_empty and cur_form.roles[target_slot] != null) cur_form.roles[target_slot].?.entity_id else -1;

    const rover_entity = RoleEntityTemplates.createRoleEntity(
        ctx.fs,
        scene,
        ctx.alloc,
        scene.player_id,
        ctx.assets,
        role_comp,
        ctx.weapon_comp,
        ctx.echo_comp,
        inst_dungeon,
        main_role_id,
    ) catch return;

    scene.plot_added_rover_slot = target_slot;
    scene.plot_saved_cur_role = saved_cur;
    scene.plot_saved_slot_role_id = saved_slot_role;
    scene.plot_saved_slot_entity_id = saved_slot_eid;

    cur_form.roles[target_slot] = .{
        .role_id = main_role_id,
        .entity_id = rover_entity.net_id,
        .on_stage_without_control = false,
    };
    cur_form.cur_role = main_role_id;
    setFormationVisibility(scene, main_role_id);

    const now_ms = std.Io.Clock.awake.now(ctx.io).toMilliseconds();
    var add_role_pbs: std.ArrayList(pb.EntityPb) = .empty;
    var add_concom_pbs: std.ArrayList(pb.EntityPb) = .empty;
    const role_pb = entity_proto.buildOptional(ctx.alloc, ctx.assets, scene, null, rover_entity.net_id, now_ms) catch return;
    add_role_pbs.append(ctx.alloc.arena, role_pb) catch return;
    if (scene.entities.slice().items(.concomitant)[rover_entity.index]) |concomitant| {
        for (concomitant.vision_entity_id) |cid| {
            if (!scene.net_id_map.contains(cid)) continue;
            const cpb = entity_proto.buildOptional(ctx.alloc, ctx.assets, scene, null, cid, now_ms) catch continue;
            add_concom_pbs.append(ctx.alloc.arena, cpb) catch continue;
        }
        for (concomitant.custom_entity_ids) |cid| {
            if (!scene.net_id_map.contains(cid)) continue;
            const cpb = entity_proto.buildOptional(ctx.alloc, ctx.assets, scene, null, cid, now_ms) catch continue;
            add_concom_pbs.append(ctx.alloc.arena, cpb) catch continue;
        }
    }
    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.EntityAddNotify{
        .EntityPbs = add_role_pbs,
        .RemoveTagIds = false,
    }) catch {};
    if (add_concom_pbs.items.len > 0) {
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.EntityAddNotify{
            .EntityPbs = add_concom_pbs,
            .RemoveTagIds = false,
        }) catch {};
    }
    pushFormationGroupUpdate(ctx, scene, cur_form);
}

fn restorePlotMainRole(ctx: QuestContext) void {
    const scene = ctx.scene orelse return;
    const slot_idx = scene.plot_added_rover_slot orelse return;
    const saved_cur = scene.plot_saved_cur_role;
    const saved_slot_role = scene.plot_saved_slot_role_id;
    const saved_slot_eid = scene.plot_saved_slot_entity_id;
    scene.plot_added_rover_slot = null;
    scene.plot_saved_cur_role = 0;
    scene.plot_saved_slot_role_id = 0;
    scene.plot_saved_slot_entity_id = -1;

    if (scene.formation_info.cur_formation < 0) return;
    const cur_idx: usize = @intCast(scene.formation_info.cur_formation);
    if (cur_idx >= scene.formation_info.formations.len) return;
    const cur_form = &scene.formation_info.formations[cur_idx];

    if (slot_idx < cur_form.roles.len) {
        if (cur_form.roles[slot_idx]) |temp_slot| {
            removeRoleEntityAndConcomitants(ctx, scene, temp_slot.entity_id);
        }
        cur_form.roles[slot_idx] = if (saved_slot_role != 0) .{
            .role_id = saved_slot_role,
            .entity_id = saved_slot_eid,
            .on_stage_without_control = false,
        } else null;
    }
    if (saved_cur != 0) {
        cur_form.cur_role = saved_cur;
    }
    setFormationVisibility(scene, cur_form.cur_role);
    pushFormationGroupUpdate(ctx, scene, cur_form);
}

fn containsNodeId(ids: []const i32, target: i32) bool {
    for (ids) |id| {
        if (id == target) return true;
    }
    return false;
}

/// Collects active ancestor node IDs (top-down from root) for a quest's active child nodes.
fn collectActiveAncestors(
    allocator: std.mem.Allocator,
    nodes: []const LazyNodeStore.NodeEntry,
    active_nodes: []const i32,
    finished_nodes: []const i32,
) ![]const i32 {
    var result: std.ArrayList(i32) = .empty;
    for (active_nodes) |nid| {
        var chain: std.ArrayList(i32) = .empty;
        var cur = findNodeById(nodes, nid);
        while (cur) |entry| {
            if (entry.parent_id <= 0) break;
            try chain.append(allocator, entry.parent_id);
            cur = findNodeById(nodes, entry.parent_id);
        }
        var i = chain.items.len;
        while (i > 0) {
            i -= 1;
            const pid = chain.items[i];
            if (containsNodeId(active_nodes, pid) or containsNodeId(finished_nodes, pid)) continue;
            if (!containsNodeId(result.items, pid)) {
                try result.append(allocator, pid);
            }
        }
    }
    return result.items;
}

/// Builds a BehaviorTreeInfo entry with finished, active ancestor, and active child node states.
fn buildChildNodeProgress(
    ctx: QuestContext,
    aq: QuestInfo.ActiveQuest,
    node: LazyNodeStore.NodeEntry,
) !?pb.ChildQuestNodeProgress {
    const cond = getChildCondition(node.data) orelse return null;
    const ctype = jsonStr(cond.get("Type")) orelse return null;

    if (std.mem.eql(u8, ctype, "Kill") or std.mem.eql(u8, ctype, "MonsterCreator")) {
        var ids_buf: [32]i32 = undefined;
        const target_ids = extractNodeTargetEntityIds(&ids_buf, node);
        var mon_ids: std.ArrayList(i32) = .empty;
        var killed_count: i32 = 0;
        for (target_ids) |tid| {
            try mon_ids.append(ctx.alloc.arena, tid);
            if (QuestInfo.hasKilledEntity(aq, tid)) killed_count += 1;
        }
        const total_count: i32 = if (target_ids.len > 0) @intCast(target_ids.len) else 1;
        return .{ .Progress = .{ .Kill = .{
            .MonId = mon_ids,
            .PrefabNum = total_count,
            .CurrNum = killed_count,
            .TotalNum = total_count,
        } } };
    }

    if (std.mem.eql(u8, ctype, "DoInteract")) {
        var ids_buf: [32]i32 = undefined;
        const target_ids = extractNodeTargetEntityIds(&ids_buf, node);
        var npc_ids: std.ArrayList(i32) = .empty;
        if (ctx.progress_comp) |pc| {
            for (target_ids) |tid| {
                var kbuf: [64]u8 = undefined;
                const key = std.fmt.bufPrint(&kbuf, "interact_done:{d}:{d}:{d}", .{ aq.quest_id, node.id, tid }) catch continue;
                if (pc.info.hasClaim(key)) {
                    try npc_ids.append(ctx.alloc.arena, tid);
                }
            }
        }
        return .{ .Progress = .{ .Interact = .{ .NpcId = npc_ids } } };
    }

    if (std.mem.eql(u8, ctype, "GetItem")) {
        var list: std.ArrayList(pb.GetItemCount) = .empty;
        if (cond.get("Items")) |items_v| {
            if (items_v == .array) {
                for (items_v.array.items) |it| {
                    if (it != .object) continue;
                    const iid = jsonInt(it.object.get("ItemId")) orelse continue;
                    const req = jsonInt(it.object.get("Count")) orelse 1;
                    const owned = ctx.inventory_comp.info.normalItemCount(iid);
                    try list.append(ctx.alloc.arena, .{ .ItemId = iid, .Count = @min(req, owned) });
                }
            }
        }
        return .{ .Progress = .{ .GetItem = .{ .Info = list } } };
    }

    if (std.mem.eql(u8, ctype, "UseItem")) {
        const iid = jsonInt(cond.get("ItemId")) orelse 0;
        if (iid > 0) {
            var used: i32 = 0;
            if (ctx.progress_comp) |pc| {
                var kbuf: [64]u8 = undefined;
                if (std.fmt.bufPrint(&kbuf, "use_item:{d}:{d}:{d}", .{ aq.quest_id, node.id, iid })) |key| {
                    used = @intCast(std.math.clamp(pc.info.getKeyInt(key), 0, std.math.maxInt(i32)));
                } else |_| {}
            }
            return .{ .Progress = .{ .UseItem = .{ .ItemId = iid, .Count = used } } };
        }
    }

    if (std.mem.eql(u8, ctype, "CheckEntityState")) {
        var eids: std.ArrayList(i32) = .empty;
        if (cond.get("Conditions")) |cs| {
            if (cs == .array) {
                for (cs.array.items) |c| {
                    if (c != .object) continue;
                    const tid = jsonInt(c.object.get("EntityId")) orelse continue;
                    if (tid <= 0) continue;
                    const want_state = jsonStr(c.object.get("State")) orelse "";
                    if (want_state.len == 0 or entityHasStateTag(ctx, tid, want_state)) {
                        try eids.append(ctx.alloc.arena, tid);
                    }
                }
            }
        } else {
            var ids_buf: [32]i32 = undefined;
            const target_ids = extractNodeTargetEntityIds(&ids_buf, node);
            for (target_ids) |tid| {
                try eids.append(ctx.alloc.arena, tid);
            }
        }
        return .{ .Progress = .{ .EntityStateList = .{ .EntityId = eids } } };
    }

    if (std.mem.eql(u8, ctype, "CheckLevelPlay")) {
        var done: i32 = 0;
        if (cond.get("LevelPlayIds")) |ids_val| {
            if (ids_val == .array) {
                for (ids_val.array.items) |v| {
                    const lpid = jsonInt(v) orelse continue;
                    if (QuestInfo.containsInt(ctx.quest_comp.info.completed_level_plays, lpid)) done += 1;
                }
            }
        }
        return .{ .Progress = .{ .levelPlayCount = done } };
    }

    if (std.mem.eql(u8, ctype, "FinishDungeon")) {
        const target = nestedDungeonId(cond, "Condition");
        const count: i32 = if (target > 0 and QuestInfo.containsInt(ctx.quest_comp.info.completed_dungeons, target)) 1 else 0;
        return .{ .Progress = .{ .CompleteInst = .{ .InstId = target, .Count = count } } };
    }

    return null;
}

fn pushNodeProgressNotify(
    ctx: QuestContext,
    aq: QuestInfo.ActiveQuest,
    node: LazyNodeStore.NodeEntry,
) !void {
    const progress = (try buildChildNodeProgress(ctx, aq, node)) orelse return;
    try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.UpdateNodeProgressNotify{
        .TreeOwnerId = aq.quest_id,
        .TreeIncId = aq.quest_id,
        .NodeId = node.id,
        .Progress = progress,
    });
}

fn buildTreeInfo(ctx: QuestContext, aq: QuestInfo.ActiveQuest) !pb.TreeInfo {
    const nodes = ctx.assets.quest_nodes.getOwnerNodes(ctx.io, aq.quest_id) catch &[_]LazyNodeStore.NodeEntry{};
    const ancestors = try collectActiveAncestors(ctx.alloc.arena, nodes, aq.active_nodes, aq.finished_nodes);

    var tree_nodes: std.ArrayList(pb.MapEntry(i32, pb.NodeInfo)) = .empty;
    var timer_infos: std.ArrayList(pb.TimerInfoPb) = .empty;
    var seen: std.ArrayList(i32) = .empty;

    for (aq.finished_nodes) |fid| {
        if (containsNodeId(seen.items, fid)) continue;
        try seen.append(ctx.alloc.arena, fid);
        const is_child = if (findNodeById(nodes, fid)) |n| std.mem.eql(u8, n.node_type, "ChildQuest") else false;
        try tree_nodes.append(ctx.alloc.arena, .{
            .key = fid,
            .value = .{
                .Status = .CompletedSuccess,
                .ExtraInfo = if (is_child) .{ .ChildQuestNodeInfo = .{ .Status = .Finished } } else null,
            },
        });
    }

    for (ancestors) |aid| {
        if (containsNodeId(seen.items, aid)) continue;
        try seen.append(ctx.alloc.arena, aid);
        try tree_nodes.append(ctx.alloc.arena, .{
            .key = aid,
            .value = .{ .Status = .Activate },
        });
    }

    for (aq.active_nodes) |nid| {
        if (containsNodeId(seen.items, nid)) continue;
        try seen.append(ctx.alloc.arena, nid);
        const progress = if (findNodeById(nodes, nid)) |n|
            try buildChildNodeProgress(ctx, aq, n.*)
        else
            null;

        if (ctx.scene) |scene| {
            if (scene.quest_timers.getDueMs(aq.quest_id, nid)) |due_ms| {
                try timer_infos.append(ctx.alloc.arena, .{
                    .TimerType = "WaitTime",
                    .NodeId = nid,
                    .EndTime = @divTrunc(due_ms, 1000),
                });
            }
        }

        try tree_nodes.append(ctx.alloc.arena, .{
            .key = nid,
            .value = .{
                .Status = .Activate,
                .ExtraInfo = .{ .ChildQuestNodeInfo = .{
                    .Status = .Progress,
                    .Progress = progress,
                } },
            },
        });
    }

    return initTreeInfo(aq.quest_id, tree_nodes, timer_infos);
}

fn initTreeInfo(
    quest_id: i32,
    nodes: @TypeOf(@as(pb.TreeInfo, undefined).Nodes),
    timer_infos: @TypeOf(@as(pb.TreeInfo, undefined).TimerInfos),
) pb.TreeInfo {
    return .{
        .TreeOwnerId = quest_id,
        .TreeIncId = quest_id,
        .BtType = .BtTypeQuest,
        .BlackboardId = quest_id,
        .Nodes = nodes,
        .TimerInfos = timer_infos,
    };
}

test "initTreeInfo binds quest BlackboardId" {
    const info = initTreeInfo(139000025, .empty, .empty);
    try std.testing.expectEqual(@as(i64, 139000025), info.TreeOwnerId);
    try std.testing.expectEqual(@as(i64, 139000025), info.TreeIncId);
    try std.testing.expectEqual(pb.BtType.BtTypeQuest, info.BtType);
    try std.testing.expectEqual(@as(i32, 139000025), info.BlackboardId);
}

fn pushAncestorActivations(
    ctx: QuestContext,
    quest_id: i32,
    nodes: []const LazyNodeStore.NodeEntry,
    parent_id: i32,
) !void {
    if (parent_id <= 0) return;
    var chain: std.ArrayList(i32) = .empty;
    var pid = parent_id;
    while (pid > 0) {
        try chain.append(ctx.alloc.arena, pid);
        const parent = findNodeById(nodes, pid) orelse break;
        pid = parent.parent_id;
    }
    var i = chain.items.len;
    while (i > 0) {
        i -= 1;
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.UpdateNodeStatusNotify{
            .TreeOwnerId = quest_id,
            .TreeIncId = quest_id,
            .NodeId = chain.items[i],
            .Status = .Activate,
        });
    }
}

fn buildPreShowQuestList(ctx: QuestContext) !std.ArrayList(i32) {
    var list: std.ArrayList(i32) = .empty;
    if (!ctx.quest_comp.info.hasFinishedQuest(prologue_quest_id)) return list;

    const eval_ctx = makeEvalContext(ctx);
    for (ctx.assets.tables.quest_data.items) |q| {
        const d = q.Data;
        if (!isEligibleQuest(ctx, d, false)) continue;
        if (ConditionEvaluator.evalQuestProvideConditions(eval_ctx, d.ProvideType)) continue;
        const psi = d.PreShowInfo orelse continue;
        if (psi != .object) continue;
        const cond = psi.object.get("PreShowCondition") orelse continue;
        if (ConditionEvaluator.evalQuestProvideConditions(eval_ctx, cond)) {
            try list.append(ctx.alloc.arena, d.Id);
        }
    }
    return list;
}

fn buildReadyQuestList(ctx: QuestContext) !std.ArrayList(i32) {
    var list: std.ArrayList(i32) = .empty;
    if (!ctx.quest_comp.info.hasFinishedQuest(prologue_quest_id)) return list;

    const cur_map: i32 = if (ctx.scene) |s| s.instance_id else if (ctx.scene_comp) |sc| sc.last_scene_info.instance_id else 8;
    const eval_ctx = makeEvalContext(ctx);
    for (ctx.assets.tables.quest_data.items) |q| {
        const d = q.Data;
        if (!std.mem.eql(u8, d.DistributeType, "Interact")) continue;
        if (d.DungeonId > 0 and d.DungeonId != cur_map) continue;
        if (d.AddInteractOption) |aio| {
            if (aio == .object) {
                if (jsonInt(aio.object.get("EntityId"))) |eid| {
                    if (eid > 0 and ctx.assets.findLevelEntity(cur_map, eid) == null) continue;
                }
            }
        }
        if (!isEligibleQuest(ctx, d, false)) continue;
        if (ConditionEvaluator.evalQuestProvideConditions(eval_ctx, d.ProvideType)) {
            try list.append(ctx.alloc.arena, d.Id);
        }
    }
    return list;
}

pub fn pushReadyQuestList(ctx: QuestContext) !void {
    const scene = ctx.scene orelse return;
    if (!scene.join_scene_pushed) return;
    const ready_list = try buildReadyQuestList(ctx);
    if (ready_list.items.len > 0) {
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.QuestReadyListNotify{ .QuestId = ready_list });
    }
}

fn pushQuestStateNotifies(ctx: QuestContext) !void {
    try pushQuestStateNotifiesWithEnter(ctx, false, false);
}

fn pushQuestStateNotifiesWithEnter(ctx: QuestContext, send_enter: bool, send_trees: bool) !void {
    var quest_list: std.ArrayList(pb.QuestInfo) = .empty;
    var finish_list: std.ArrayList(i32) = .empty;
    var tree_infos: std.ArrayList(pb.TreeInfo) = .empty;

    for (ctx.quest_comp.info.active_quests) |aq| {
        try quest_list.append(ctx.alloc.arena, .{ .QuestId = aq.quest_id, .Status = .InProgress });
        if (send_trees) {
            try tree_infos.append(ctx.alloc.arena, try buildTreeInfo(ctx, aq));
        }
    }

    for (ctx.quest_comp.info.finished_quests) |fid| {
        try finish_list.append(ctx.alloc.arena, fid);
    }

    const show_list = try buildPreShowQuestList(ctx);

    try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.QuestListNotify{ .Quests = quest_list });
    try pushReadyQuestList(ctx);
    try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.QuestShowListNotify{ .QuestId = show_list });
    try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.QuestFinishListNotify{ .QuestId = finish_list });
    if (send_trees and tree_infos.items.len > 0) {
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.BehaviorTreeInfoNotify{ .TreeInfos = tree_infos });
    }

    if (!send_trees) {
        for (ctx.quest_comp.info.active_quests) |aq| {
            const nodes = ctx.assets.quest_nodes.getOwnerNodes(ctx.io, aq.quest_id) catch &[_]LazyNodeStore.NodeEntry{};
            const ancestors = try collectActiveAncestors(ctx.alloc.arena, nodes, aq.active_nodes, aq.finished_nodes);
            for (ancestors) |aid| {
                try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.UpdateNodeStatusNotify{
                    .TreeOwnerId = aq.quest_id,
                    .TreeIncId = aq.quest_id,
                    .NodeId = aid,
                    .Status = .Activate,
                });
            }
            // Resync active nodes state to client (without re-triggering Progress on login-synced trees).
            for (aq.active_nodes) |nid| {
                if (isNodeHideUi(findNodeById(nodes, nid))) try resyncActiveNode(ctx, aq.quest_id, nid, findNodeById(nodes, nid), send_enter);
            }
            for (aq.active_nodes) |nid| {
                if (!isNodeHideUi(findNodeById(nodes, nid))) try resyncActiveNode(ctx, aq.quest_id, nid, findNodeById(nodes, nid), send_enter);
            }
        }
    }

    try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.TraceQuestNotify{ .QuestId = ctx.quest_comp.info.tracked_quest_id });
    if (ctx.quest_comp.info.focus_quest_id > 0) {
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.FocusQuestSetNotify{ .FocusQuestId = ctx.quest_comp.info.focus_quest_id, .Reason = .Inherit });
    }
    try pushDailyAndBranchNotifies(ctx);
}

fn isNodeHideUi(node: ?*const LazyNodeStore.NodeEntry) bool {
    const entry = node orelse return false;
    const hide_val = entry.data.object.get("HideUi") orelse return false;
    return hide_val == .bool and hide_val.bool;
}

fn nodeActionItems(node_opt: ?*const LazyNodeStore.NodeEntry, field: []const u8) []const std.json.Value {
    const entry = node_opt orelse return &.{};
    if (entry.data != .object) return &.{};
    const acts = entry.data.object.get(field) orelse return &.{};
    if (acts != .array) return &.{};
    return acts.array.items;
}

fn isPlayFlowClientFlow(ctx: QuestContext, act: std.json.Value) bool {
    const params_val = act.object.get("Params") orelse return false;
    if (params_val != .object) return false;
    const flow_name = jsonStr(params_val.object.get("FlowListName")) orelse return false;
    const flow_id = jsonInt(params_val.object.get("FlowId")) orelse 1;
    const state_id = jsonInt(params_val.object.get("StateId")) orelse 1;
    const state_val = (ctx.assets.flow_states.getFlowState(
        ctx.io,
        ctx.alloc.arena,
        flow_name,
        flow_id,
        state_id,
    ) catch null) orelse return false;
    if (state_val != .object) return false;
    return jsonBool(state_val.object.get("IsClientFlow")) orelse false;
}

// Verified against 3.8.11 Aki.Game.dll LevelEventCenter.RegistEvents intersect QuestNodeData.json.
// Excludes server-only actions (not registered in LevelEventCenter) and unsafe/conflicting handlers
// (LeisureInteract is option-whitelisted below because some options cast BT context;
// AddBuffToPlayer needs ClientTriggerComponent; AddBuffToEntity and SetPlayerOperationRestriction
// are empty no-ops; OpenSystemFunction/RemoveBuffFromEntity run server-side).
const client_executable_bt_actions = std.StaticStringMap(void).initComptime(.{
    .{ "AddTrialCharacter", {} },
    .{ "AdjustPlayerCamera", {} },
    .{ "BvbPlayDialog", {} },
    .{ "BvbPlayerOperationConstraint", {} },
    .{ "CameraLookAt", {} },
    .{ "ChangeEntityState", {} },
    .{ "ChangeNpcPerformState", {} },
    .{ "CharacterLookAt", {} },
    .{ "CharacterMoveToPoint", {} },
    .{ "ClientChangeTeamPosition", {} },
    .{ "ClientModifyTargetTag", {} },
    .{ "ClientPreEnableSubLevels", {} },
    .{ "ClientSetPlayerPos", {} },
    .{ "ClientSetSeasonState", {} },
    .{ "ClientUnlockAchievementSystemItem", {} },
    .{ "CommonTip", {} },
    .{ "CommonTip2", {} },
    .{ "CompleteGuide", {} },
    .{ "DisableEntityLookAt", {} },
    .{ "DrinksRollRoleRequirement", {} },
    .{ "DriveMotorInSpecConfig", {} },
    .{ "EnableAI", {} },
    .{ "EnableActor", {} },
    .{ "EnableClientFunction", {} },
    .{ "EnableEntityLookAt", {} },
    .{ "EnableHostility", {} },
    .{ "EnableKey4Func", {} },
    .{ "EnableNearbyTracking", {} },
    .{ "EnableSplineMoveModel", {} },
    .{ "EndCommonTip", {} },
    .{ "EnterMovieCamera", {} },
    .{ "EnterMovieMode", {} },
    .{ "EnterOrbitalCamera", {} },
    .{ "EntityLookAt", {} },
    .{ "EntityTurnTo", {} },
    .{ "ExecClientBattleAction", {} },
    .{ "ExitDungeon", {} },
    .{ "ExitMovieCamera", {} },
    .{ "ExitMovieMode", {} },
    .{ "ExitOrbitalCamera", {} },
    .{ "FadeInScreen", {} },
    .{ "FadeOutScreen", {} },
    .{ "FocusOnMapMark", {} },
    .{ "GiantTriggerResponse", {} },
    .{ "GuestOperateUiAnimation", {} },
    .{ "GuideTrigger", {} },
    .{ "HideTargetRange", {} },
    .{ "LockEntity", {} },
    .{ "Log", {} },
    .{ "MoveSceneItem", {} },
    .{ "MoveWithSpline", {} },
    .{ "NewMoveWithSpline", {} },
    .{ "NpcLeisureInteract", {} },
    .{ "OpenQte", {} },
    .{ "OpenQuickHackSystem", {} },
    .{ "OpenSimpleGameplay", {} },
    .{ "OpenSystemBoard", {} },
    .{ "OpenSystemBoardWithReturn", {} },
    .{ "OverrideAiBehaviorTree", {} },
    .{ "PhoneSystemInteract", {} },
    .{ "PlayBubble", {} },
    .{ "PlayDynamicSettlement", {} },
    .{ "PlayEffect", {} },
    .{ "PlayEffect2", {} },
    .{ "PlayMontage", {} },
    .{ "PlayRegisteredMontage", {} },
    .{ "PlayerInput", {} },
    .{ "PlayerLookAt", {} },
    .{ "PostAkEvent", {} },
    .{ "Preload", {} },
    .{ "Prompt", {} },
    .{ "PromptQuestChapterUI", {} },
    .{ "RemoveNpcGroupPerform", {} },
    .{ "ResetPlayerCameraFocus", {} },
    .{ "RestoreAiBehaviorTree", {} },
    .{ "RestorePlayerCameraAdjustment", {} },
    .{ "RunActions", {} },
    .{ "SetBattleState", {} },
    .{ "SetEntityClientVisibleSave", {} },
    .{ "SetNpcGroupPerform", {} },
    .{ "SetRegionConfig", {} },
    .{ "SetSportsState", {} },
    .{ "SetSubScreenMode", {} },
    .{ "SetTimeScale", {} },
    .{ "SetWuYinQuState", {} },
    .{ "ShowTargetRange", {} },
    .{ "SlideRailEnd", {} },
    .{ "StartRoadNetworkNavigation", {} },
    .{ "StopCameraLookAt", {} },
    .{ "StopNewMoveWithSpline", {} },
    .{ "StopUiScreenEffect", {} },
    .{ "SwitchDataLayers", {} },
    .{ "SwitchSubLevels", {} },
    .{ "TakePlotPhoto", {} },
    .{ "TeleportDungeon", {} },
    .{ "ToggleAirWall", {} },
    .{ "ToggleDragActorPlay", {} },
    .{ "ToggleHighlightExploreUi", {} },
    .{ "ToggleMapMarkState", {} },
    .{ "ToggleScanSplineEffect", {} },
    .{ "TrapDefenseChangeMiniMap", {} },
    .{ "TrapDefensePlayerOperationConstraint", {} },
    .{ "TriggerCameraShake", {} },
    .{ "TriggerGamepadShake", {} },
    .{ "TriggerSpecificScanEffect", {} },
    .{ "UnlockEntity", {} },
    .{ "UsePhantomSkill", {} },
    .{ "VehicleMoveWithPathLine", {} },
    .{ "Wait", {} },
    .{ "WaitBattleCondition", {} },
    .{ "WaitUntilLevelSequenceReachMark", {} },
});

fn isClientActionNameRegistered(name: []const u8) bool {
    return client_executable_bt_actions.has(name);
}

fn isClientExecutableBtAction(ctx: QuestContext, act: std.json.Value) bool {
    if (act != .object) return false;
    const name = jsonStr(act.object.get("Name")) orelse return false;
    if (std.mem.eql(u8, name, "PlayFlow")) return isPlayFlowClientFlow(ctx, act);
    if (std.mem.eql(u8, name, "LeisureInteract")) return isSafeBtLeisureInteract(act);
    return isClientActionNameRegistered(name);
}

fn isSafeBtLeisureInteract(act: std.json.Value) bool {
    const params = act.object.get("Params") orelse return false;
    if (params != .object) return false;
    const option = params.object.get("Option") orelse return false;
    if (option != .object) return false;
    const option_type = jsonStr(option.object.get("Type")) orelse return false;

    // These options do not cast the BT GameCtx to TriggerContext/EntityContext in
    // LevelEventLeisureInteract. Sit actions need an explicit scene entity because
    // BT context cannot resolve their target; slide entry likewise needs its rail ID.
    if (std.mem.eql(u8, option_type, "SitDown") or std.mem.eql(u8, option_type, "SitOnGround")) {
        return (jsonInt(params.object.get("SceneEntity")) orelse 0) > 0;
    }
    if (std.mem.eql(u8, option_type, "MotorEnterSlide")) {
        return (jsonInt(option.object.get("EntityId")) orelse 0) > 0;
    }

    const safe_options = [_][]const u8{
        "CustomSkill",
        "GetUp",
        "FailurePose",
        "Soar",
        "StandControl2",
        "MoneHackingDoor",
        "LandSlideEnd",
        "Glide",
    };
    for (safe_options) |safe_type| {
        if (std.mem.eql(u8, option_type, safe_type)) return true;
    }
    return false;
}

pub fn evalInlineCondition(ctx: QuestContext, owner_id: i32, cb: std.json.Value) bool {
    return ConditionEvaluator.evalInlineConditionBlock(makeEvalContextForQuest(ctx, owner_id), cb);
}

pub fn evalChildConditionAuto(ctx: QuestContext, owner_id: i32, node: LazyNodeStore.NodeEntry) bool {
    return shouldAutoCompleteChildQuest(ctx, owner_id, node);
}

pub fn getChildConditionPub(data: std.json.Value) ?std.json.ObjectMap {
    return getChildCondition(data);
}

pub fn nodeActionItemsPub(node_opt: ?*const LazyNodeStore.NodeEntry, field: []const u8) []const std.json.Value {
    return nodeActionItems(node_opt, field);
}

pub fn pushBtLevelEventsForType(
    ctx: QuestContext,
    bt_type: pb.BtType,
    owner_id: i32,
    node_id: i32,
    comptime ctx_type: pb.GameCtxType,
    actions: []const std.json.Value,
) !void {
    if (actions.len == 0) return;
    const bt_ctx: pb.BehaviorTreeCtxPb = .{
        .IncId = owner_id,
        .BtType = bt_type,
        .BtId = owner_id,
        .NodeId = node_id,
    };
    const game_ctx: pb.GameCtxPb = switch (ctx_type) {
        .ChildQuestNodeEnterAction => .{
            .CtxType = .ChildQuestNodeEnterAction,
            .CtxInfo = .{ .ChildQuestNodeEnterAction = .{ .BehaviorTreeCtx = bt_ctx } },
        },
        .ChildQuestNodeFinishAction => .{
            .CtxType = .ChildQuestNodeFinishAction,
            .CtxInfo = .{ .ChildQuestNodeFinishAction = .{ .BehaviorTreeCtx = bt_ctx } },
        },
        .ActionGroupNodeAction => .{
            .CtxType = .ActionGroupNodeAction,
            .CtxInfo = .{ .ActionGroupNodeAction = .{ .BehaviorTreeCtx = bt_ctx } },
        },
        .SuccessNodeAction => .{
            .CtxType = .SuccessNodeAction,
            .CtxInfo = .{ .SuccessNodeAction = .{ .BehaviorTreeCtx = bt_ctx } },
        },
        else => @compileError("unsupported BehaviorTree LevelEvent context"),
    };
    const total_count: i32 = @intCast(actions.len);
    var i: usize = 0;
    while (i < actions.len) {
        if (!isClientExecutableBtAction(ctx, actions[i])) {
            i += 1;
            continue;
        }
        const start_idx = i;
        while (i < actions.len and isClientExecutableBtAction(ctx, actions[i])) : (i += 1) {}
        const end_idx = i - 1;
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.LevelEventNotify{
            .PlayerId = ctx.basic_comp.player_id,
            .IncId = owner_id,
            .GameCtx = game_ctx,
            .TotalCount = total_count,
            .StartIndex = @intCast(start_idx),
            .EndIndex = @intCast(end_idx),
        });
    }
}

fn pushBtLevelEvents(
    ctx: QuestContext,
    quest_id: i32,
    node_id: i32,
    comptime ctx_type: pb.GameCtxType,
    actions: []const std.json.Value,
) !void {
    try pushBtLevelEventsForType(ctx, .BtTypeQuest, quest_id, node_id, ctx_type, actions);
}

fn resyncActiveNode(
    ctx: QuestContext,
    quest_id: i32,
    node_id: i32,
    node_opt: ?*const LazyNodeStore.NodeEntry,
    send_enter: bool,
) !void {
    if (send_enter) {
        try pushNodeStatus(ctx, quest_id, node_id, .Activate, .Progress);
        if (node_opt) |node| {
            if (ctx.scene) |scene| {
                try pushDoInteractOptionsForNode(scene, ctx.conn, ctx.alloc, ctx.io, quest_id, node.*, null);
            }
        }
        return;
    }
    try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.UpdateNodeStatusNotify{
        .TreeOwnerId = quest_id,
        .TreeIncId = quest_id,
        .NodeId = node_id,
        .Status = .Activate,
    });
    if (ctx.quest_comp.info.findActiveQuest(quest_id)) |aq| {
        if (node_opt) |node| {
            try pushNodeProgressNotify(ctx, aq, node.*);
        }
    }
    if (node_opt) |node| {
        if (ctx.scene) |scene| {
            try pushDoInteractOptionsForNode(scene, ctx.conn, ctx.alloc, ctx.io, quest_id, node.*, null);
        }
    }
}

pub fn syncDynamicInteractOptionsForEntity(
    scene: *Scene,
    assets: *const Assets,
    conn: *Connection,
    quest_info: QuestInfo,
    progress_comp: ?*const PlayerProgressComponent,
    alloc: mem.Alloc,
    io: std.Io,
    cfg_id: i64,
) !void {
    if (cfg_id <= 0) return;
    for (quest_info.active_quests) |aq| {
        const nodes = assets.quest_nodes.getOwnerNodes(io, aq.quest_id) catch continue;
        for (aq.active_nodes) |nid| {
            const node = findNodeById(nodes, nid) orelse continue;
            try pushDoInteractOptionsForNode(scene, conn, alloc, io, aq.quest_id, node.*, cfg_id);
        }
    }
    try LevelPlayHelper.syncDynamicInteractOptionsForEntity(scene, assets, conn, quest_info, progress_comp, alloc, io, cfg_id);
}

fn pushDoInteractOptionsForNode(
    scene: *Scene,
    conn: *Connection,
    alloc: mem.Alloc,
    io: std.Io,
    quest_id: i32,
    node: LazyNodeStore.NodeEntry,
    filter_cfg_id: ?i64,
) !void {
    try pushDoInteractOptionsForBtNode(scene, conn, alloc, io, .BtTypeQuest, quest_id, node, filter_cfg_id);
}

pub fn pushDoInteractOptionsForBtNode(
    scene: *Scene,
    conn: *Connection,
    alloc: mem.Alloc,
    io: std.Io,
    bt_type: pb.BtType,
    owner_id: i32,
    node: LazyNodeStore.NodeEntry,
    filter_cfg_id: ?i64,
) !void {
    if (!std.mem.eql(u8, node.node_type, "ChildQuest")) return;
    const cond = getChildCondition(node.data) orelse return;
    const ctype = jsonStr(cond.get("Type")) orelse return;
    if (std.mem.eql(u8, ctype, "DoInteract")) {
        const add_opts = cond.get("AddOptions") orelse return;
        if (add_opts != .array) return;
        for (add_opts.array.items) |entry| {
            try pushDynamicInteractEntry(scene, conn, alloc, io, bt_type, owner_id, node.id, entry, filter_cfg_id);
        }
    } else if (std.mem.eql(u8, ctype, "HandInItems")) {
        if (cond.get("AddOption")) |ao| {
            try pushDynamicInteractEntry(scene, conn, alloc, io, bt_type, owner_id, node.id, ao, filter_cfg_id);
        } else if (cond.get("HandInItems")) |hi| {
            if (hi == .object) {
                if (hi.object.get("AddOption")) |ao| {
                    try pushDynamicInteractEntry(scene, conn, alloc, io, bt_type, owner_id, node.id, ao, filter_cfg_id);
                }
            }
        }
    }
}

fn pushDynamicInteractEntry(
    scene: *Scene,
    conn: *Connection,
    alloc: mem.Alloc,
    io: std.Io,
    bt_type: pb.BtType,
    quest_id: i32,
    node_id: i32,
    entry: std.json.Value,
    filter_cfg_id: ?i64,
) !void {
    if (entry != .object) return;
    const cfg_id = jsonInt(entry.object.get("EntityId")) orelse return;
    if (cfg_id <= 0) return;
    if (filter_cfg_id) |want_id| {
        if (cfg_id != want_id) return;
    }
    const net_id = scene.findEntityNetIdByConfigId(cfg_id) orelse return;
    const opt_val = entry.object.get("Option") orelse return;
    if (opt_val != .object) return;
    const guid = jsonStr(opt_val.object.get("Guid")) orelse return;
    if (guid.len == 0) return;
    const text = jsonStr(opt_val.object.get("TidContent")) orelse "";

    var kind: Scene.DynamicInteractOption.Kind = .none;
    var flow_name: []const u8 = "";
    var flow_id: i32 = 0;
    var state_id: i32 = 0;
    var actions_json: []const u8 = "";

    if (opt_val.object.get("Type")) |tv| {
        if (tv == .object) {
            const tname = jsonStr(tv.object.get("Type")) orelse "";
            if (std.mem.eql(u8, tname, "Flow")) {
                if (tv.object.get("Flow")) |fv| {
                    if (fv == .object) {
                        flow_name = jsonStr(fv.object.get("FlowListName")) orelse "";
                        flow_id = jsonInt(fv.object.get("FlowId")) orelse 0;
                        state_id = jsonInt(fv.object.get("StateId")) orelse 0;
                        if (flow_name.len > 0 and flow_id > 0) kind = .flow;
                    }
                }
            } else if (std.mem.eql(u8, tname, "Actions")) {
                kind = .actions;
                if (tv.object.get("Actions")) |av| {
                    if (av == .array and av.array.items.len > 0) {
                        actions_json = std.json.Stringify.valueAlloc(alloc.arena, av, .{}) catch "[]";
                    } else {
                        actions_json = "[]";
                    }
                } else {
                    actions_json = "[]";
                }
            }
        }
    }

    if (kind == .none) {
        kind = .actions;
        actions_json = "[]";
    }

    try scene.upsertDynamicInteractOptionWithBt(
        alloc.gpa,
        guid,
        net_id,
        @intCast(cfg_id),
        bt_type,
        quest_id,
        node_id,
        kind,
        flow_name,
        flow_id,
        state_id,
        actions_json,
    );

    conn.push(alloc.gpa, io, pb.EntityAddDynamicInteractNotify{
        .GameCtx = .{
            .CtxType = .BehaviorTree,
            .CtxInfo = .{ .BehaviorTree = .{
                .IncId = quest_id,
                .BtType = bt_type,
                .BtId = quest_id,
                .NodeId = node_id,
            } },
        },
        .EntityId = net_id,
        .DelayRemove = false,
        .Text = text,
        .OptionGuid = guid,
    }) catch {};
}

fn removeDoInteractOptionsForNode(ctx: QuestContext, quest_id: i32, node_id: i32) void {
    removeDoInteractOptionsForBtNode(ctx, .BtTypeQuest, quest_id, node_id);
}

pub fn removeDoInteractOptionsForBtNode(ctx: QuestContext, bt_type: pb.BtType, owner_id: i32, node_id: i32) void {
    const scene = ctx.scene orelse return;
    const removed = scene.takeDynamicInteractOptionsForBtNode(ctx.alloc.arena, bt_type, owner_id, node_id) catch return;
    for (removed) |*opt| {
        defer opt.deinit(ctx.alloc.gpa);
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.EntityRemoveDynamicInteractNotify{
            .OptionGuid = opt.option_guid,
            .EntityId = opt.entity_net_id,
        }) catch {};
    }
}

pub fn pushBtChildNodeStatus(
    ctx: QuestContext,
    bt_type: pb.BtType,
    owner_id: i32,
    node: *const LazyNodeStore.NodeEntry,
    node_status: pb.NodeStatus,
    child_status: pb.ChildQuestNodeStatus,
) !void {
    if (node_status == .Activate and child_status == .Progress) {
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.UpdateNodeStatusNotify{
            .TreeOwnerId = owner_id,
            .TreeIncId = owner_id,
            .NodeId = node.id,
            .Status = node_status,
        });
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.UpdateChildQuestNodeStatusNotify{
            .TreeOwnerId = owner_id,
            .TreeIncId = owner_id,
            .NodeId = node.id,
            .Status = .Enter,
        });
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.UpdateChildQuestNodeStatusNotify{
            .TreeOwnerId = owner_id,
            .TreeIncId = owner_id,
            .NodeId = node.id,
            .Status = .EnterAction,
        });
        try pushBtLevelEventsForType(ctx, bt_type, owner_id, node.id, .ChildQuestNodeEnterAction, nodeActionItems(node, "EnterActions"));
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.UpdateChildQuestNodeStatusNotify{
            .TreeOwnerId = owner_id,
            .TreeIncId = owner_id,
            .NodeId = node.id,
            .Status = child_status,
        });
    } else if (child_status == .Finished) {
        // Client ServerNotifyAssistant crashes with KeyNotFoundException if UpdateChildQuestNodeStatusNotify sends CqnsFinishAction.
        try pushBtLevelEventsForType(ctx, bt_type, owner_id, node.id, .ChildQuestNodeFinishAction, nodeActionItems(node, "FinishActions"));
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.UpdateChildQuestNodeStatusNotify{
            .TreeOwnerId = owner_id,
            .TreeIncId = owner_id,
            .NodeId = node.id,
            .Status = child_status,
        });
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.UpdateNodeStatusNotify{
            .TreeOwnerId = owner_id,
            .TreeIncId = owner_id,
            .NodeId = node.id,
            .Status = node_status,
        });
    } else {
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.UpdateNodeStatusNotify{
            .TreeOwnerId = owner_id,
            .TreeIncId = owner_id,
            .NodeId = node.id,
            .Status = node_status,
        });
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.UpdateChildQuestNodeStatusNotify{
            .TreeOwnerId = owner_id,
            .TreeIncId = owner_id,
            .NodeId = node.id,
            .Status = child_status,
        });
    }
}

fn pushNodeStatus(
    ctx: QuestContext,
    quest_id: i32,
    node_id: i32,
    node_status: pb.NodeStatus,
    child_status: pb.ChildQuestNodeStatus,
) !void {
    const nodes = ctx.assets.quest_nodes.getOwnerNodes(ctx.io, quest_id) catch &[_]LazyNodeStore.NodeEntry{};
    const node_opt = findNodeById(nodes, node_id);

    if (node_status == .Activate and child_status == .Progress) {
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.UpdateNodeStatusNotify{
            .TreeOwnerId = quest_id,
            .TreeIncId = quest_id,
            .NodeId = node_id,
            .Status = node_status,
        });
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.UpdateChildQuestNodeStatusNotify{
            .TreeOwnerId = quest_id,
            .TreeIncId = quest_id,
            .NodeId = node_id,
            .Status = .Enter,
        });
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.UpdateChildQuestNodeStatusNotify{
            .TreeOwnerId = quest_id,
            .TreeIncId = quest_id,
            .NodeId = node_id,
            .Status = .EnterAction,
        });
        try pushBtLevelEvents(ctx, quest_id, node_id, .ChildQuestNodeEnterAction, nodeActionItems(node_opt, "EnterActions"));
        if (ctx.quest_comp.info.findActiveQuest(quest_id)) |aq| {
            if (node_opt) |node| {
                try pushNodeProgressNotify(ctx, aq, node.*);
            }
        }
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.UpdateChildQuestNodeStatusNotify{
            .TreeOwnerId = quest_id,
            .TreeIncId = quest_id,
            .NodeId = node_id,
            .Status = child_status,
        });
    } else if (child_status == .Finished) {
        // Client ServerNotifyAssistant crashes with KeyNotFoundException if UpdateChildQuestNodeStatusNotify sends CqnsFinishAction.
        try pushBtLevelEvents(ctx, quest_id, node_id, .ChildQuestNodeFinishAction, nodeActionItems(node_opt, "FinishActions"));
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.UpdateChildQuestNodeStatusNotify{
            .TreeOwnerId = quest_id,
            .TreeIncId = quest_id,
            .NodeId = node_id,
            .Status = child_status,
        });
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.UpdateNodeStatusNotify{
            .TreeOwnerId = quest_id,
            .TreeIncId = quest_id,
            .NodeId = node_id,
            .Status = node_status,
        });
    } else {
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.UpdateNodeStatusNotify{
            .TreeOwnerId = quest_id,
            .TreeIncId = quest_id,
            .NodeId = node_id,
            .Status = node_status,
        });
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.UpdateChildQuestNodeStatusNotify{
            .TreeOwnerId = quest_id,
            .TreeIncId = quest_id,
            .NodeId = node_id,
            .Status = child_status,
        });
    }
}

var finalizing_depth: u32 = 0;

fn finalizeProgression(ctx: QuestContext) !void {
    if (finalizing_depth == 0) {
        finalizing_depth += 1;
        defer finalizing_depth -= 1;
        _ = stepPendingControlNodes(ctx) catch false;
    }
    ensureTrackedFocusValid(ctx);
    if (ctx.scene) |scene| scene.current_quest_id = ctx.quest_comp.info.tracked_quest_id;
    syncSceneCurrentNode(ctx);

    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.TraceQuestNotify{ .QuestId = ctx.quest_comp.info.tracked_quest_id }) catch {};
    try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);
}

fn ensureTrackedFocusValid(ctx: QuestContext) void {
    const focus = ctx.quest_comp.info.focus_quest_id;
    if (focus > 0 and !ctx.quest_comp.info.hasActiveQuest(focus)) {
        ctx.quest_comp.info.focus_quest_id = 0;
    }

    const cur = ctx.quest_comp.info.tracked_quest_id;
    if (cur <= 0) {
        ctx.quest_comp.info.tracked_quest_id = 0;
        return;
    }
    if (ctx.quest_comp.info.hasActiveQuest(cur)) return;

    var best: i32 = 0;
    for (ctx.quest_comp.info.active_quests) |aq| {
        if (best == 0) best = aq.quest_id;
        if (ctx.assets.tables.quest_data.getDataById(aq.quest_id)) |q| {
            if (q.Data.Type == 1) {
                best = aq.quest_id;
                break;
            }
        }
    }
    ctx.quest_comp.info.tracked_quest_id = best;
}

fn syncSceneCurrentNode(ctx: QuestContext) void {
    const scene = ctx.scene orelse return;
    scene.has_quest_move_loc = false;
    if (ctx.quest_comp.info.findActiveQuest(ctx.quest_comp.info.tracked_quest_id)) |aq| {
        if (aq.active_nodes.len > 0) {
            scene.current_quest_node = aq.active_nodes[0];
            return;
        }
    }
    scene.current_quest_node = 0;
}

fn markAndCheckDoInteractComplete(
    ctx: QuestContext,
    quest_id: i32,
    node: LazyNodeStore.NodeEntry,
    cond: std.json.ObjectMap,
    entity_id: i64,
    opt_obj: std.json.ObjectMap,
) !bool {
    const pc = ctx.progress_comp orelse return true;
    const target_eid: i32 = jsonInt(opt_obj.get("EntityId")) orelse resolveConfigId(ctx.scene, entity_id);
    if (target_eid > 0) {
        var kbuf: [64]u8 = undefined;
        const key = std.fmt.bufPrint(&kbuf, "interact_done:{d}:{d}:{d}", .{ quest_id, node.id, target_eid }) catch return true;
        _ = try pc.info.markClaim(ctx.alloc.gpa, key);
        try pc.save(ctx.fs, ctx.alloc.arena);
    }

    var all_done = true;
    if (cond.get("AddOptions")) |opts_v| {
        if (opts_v == .array and opts_v.array.items.len > 1) {
            for (opts_v.array.items) |o| {
                if (o != .object) continue;
                const eid = jsonInt(o.object.get("EntityId")) orelse continue;
                var kbuf: [64]u8 = undefined;
                const key = std.fmt.bufPrint(&kbuf, "interact_done:{d}:{d}:{d}", .{ quest_id, node.id, eid }) catch continue;
                if (!pc.info.hasClaim(key)) {
                    all_done = false;
                    break;
                }
            }
        }
    }

    if (!all_done) {
        if (ctx.quest_comp.info.findActiveQuest(quest_id)) |aq| {
            try pushNodeProgressNotify(ctx, aq, node);
        }
    }
    return all_done;
}

fn handleInteractOption(
    ctx: QuestContext,
    quest_id: i32,
    nodes: []const LazyNodeStore.NodeEntry,
    node: LazyNodeStore.NodeEntry,
    cond: std.json.ObjectMap,
    entity_id: i64,
    opt_obj: std.json.ObjectMap,
) !bool {
    const inner = opt_obj.get("Option") orelse return blk: {
        if (try markAndCheckDoInteractComplete(ctx, quest_id, node, cond, entity_id, opt_obj)) {
            try completeChildQuest(ctx, quest_id, nodes, node);
            break :blk true;
        }
        break :blk false;
    };
    if (inner != .object) {
        if (try markAndCheckDoInteractComplete(ctx, quest_id, node, cond, entity_id, opt_obj)) {
            try completeChildQuest(ctx, quest_id, nodes, node);
            return true;
        }
        return false;
    }

    if (inner.object.get("Type")) |tval| {
        if (tval == .object) {
            const tstr = jsonStr(tval.object.get("Type")) orelse "";
            if (std.mem.eql(u8, tstr, "Flow")) {
                if (tval.object.get("Flow")) |fv| {
                    if (fv == .object) {
                        try startFlowFromObj(ctx, quest_id, node.id, fv.object);
                        if (ctx.scene) |scene| {
                            if (scene.active_flow_inc_id != 0 and scene.active_flow_quest_id == quest_id and scene.active_flow_node_id == node.id) {
                                scene.active_flow_owner_entity_id = entity_id;
                            }
                        }
                        return false;
                    }
                }
            } else if (std.mem.eql(u8, tstr, "Actions")) {
                if (tval.object.get("Actions")) |av| {
                    if (av == .array) {
                        const prev_owner = if (ctx.scene) |scene| blk: {
                            const prev = scene.active_flow_owner_entity_id;
                            scene.active_flow_owner_entity_id = entity_id;
                            break :blk prev;
                        } else @as(i64, 0);
                        defer if (ctx.scene) |scene| {
                            scene.active_flow_owner_entity_id = prev_owner;
                        };
                        try executeActions(ctx, quest_id, node.id, av.array.items);
                    }
                }
            }
        }
    }

    if (try markAndCheckDoInteractComplete(ctx, quest_id, node, cond, entity_id, opt_obj)) {
        try completeChildQuest(ctx, quest_id, nodes, node);
        return true;
    }
    return false;
}

fn consumeHandInItems(ctx: QuestContext, cond: std.json.ObjectMap) !bool {
    const hi = cond.get("HandInItems") orelse return false;
    if (hi != .object) return false;
    const items_val = hi.object.get("Items") orelse return true;
    if (items_val != .array) return true;

    // Verify full required quantity is owned before consuming anything.
    for (items_val.array.items) |it| {
        if (it != .object) continue;
        const count = jsonInt(it.object.get("Count")) orelse 1;
        if (count <= 0) continue;
        const ids_v = it.object.get("ItemIds") orelse return false;
        if (ids_v != .array) return false;
        var avail: i64 = 0;
        for (ids_v.array.items) |id_v| {
            const iid = jsonInt(id_v) orelse continue;
            avail += ctx.inventory_comp.info.normalItemCount(iid);
        }
        if (avail < count) return false;
    }

    var removed_any = false;
    for (items_val.array.items) |it| {
        if (it != .object) continue;
        const count = jsonInt(it.object.get("Count")) orelse 1;
        if (count <= 0) continue;

        if (it.object.get("ItemIds")) |ids_v| {
            if (ids_v == .array) {
                var rem = count;
                for (ids_v.array.items) |id_v| {
                    if (rem <= 0) break;
                    const iid = jsonInt(id_v) orelse continue;
                    const owned = ctx.inventory_comp.info.normalItemCount(iid);
                    if (owned > 0) {
                        const take = @min(owned, rem);
                        if (ctx.inventory_comp.info.consumeNormalItem(ctx.alloc.gpa, iid, take) catch false) {
                            rem -= take;
                            removed_any = true;
                        }
                    }
                }
            }
        }
    }

    if (removed_any) {
        const i_path = try std.fmt.allocPrint(ctx.alloc.arena, "player/{d}/{s}", .{ ctx.basic_comp.player_id, InventoryInfo.data_path });
        try comp_util.saveStruct(ctx.fs, ctx.inventory_comp.info, i_path, ctx.alloc.arena);
    }
    return true;
}

fn extractUiGameplayKey(cfg: std.json.ObjectMap, buf: []u8) ?[]const u8 {
    const keys = [_][]const u8{ "MorseCodeId", "CipherId", "ReignsId", "Id" };
    for (keys) |k| {
        const val = cfg.get(k) orelse continue;
        switch (val) {
            .string => |s| if (s.len > 0) return s,
            .integer => |iv| return std.fmt.bufPrint(buf, "{d}", .{iv}) catch null,
            else => {},
        }
    }
    return null;
}

fn matchesCheckUiGameKey(cond: std.json.ObjectMap, gameplay_key: []const u8) bool {
    const cfgs = cond.get("GameplayConfigs") orelse return false;
    if (cfgs != .array) return false;
    for (cfgs.array.items) |item| {
        if (item != .object) continue;
        var buf: [32]u8 = undefined;
        const k = extractUiGameplayKey(item.object, &buf) orelse continue;
        if (std.mem.eql(u8, k, gameplay_key)) return true;
    }
    return false;
}

fn hasCompletedUiGameplay(pc: *const PlayerProgressComponent, cond: std.json.ObjectMap) bool {
    const cfgs = cond.get("GameplayConfigs") orelse return false;
    if (cfgs != .array or cfgs.array.items.len == 0) return false;
    for (cfgs.array.items) |item| {
        if (item != .object) continue;
        var buf: [32]u8 = undefined;
        const k = extractUiGameplayKey(item.object, &buf) orelse return false;
        var cbuf: [64]u8 = undefined;
        const claim_key = std.fmt.bufPrint(&cbuf, "ui_gp:{s}", .{k}) catch return false;
        if (!pc.info.hasClaim(claim_key)) return false;
    }
    return true;
}

fn snapshotActiveQuestIds(ctx: QuestContext) ![]const i32 {
    const ids = try ctx.alloc.arena.alloc(i32, ctx.quest_comp.info.active_quests.len);
    for (ctx.quest_comp.info.active_quests, 0..) |aq, i| {
        ids[i] = aq.quest_id;
    }
    return ids;
}

fn findRootNode(nodes: []const LazyNodeStore.NodeEntry) ?LazyNodeStore.NodeEntry {
    for (nodes) |n| {
        if (n.parent_id == 0) return n;
    }
    return null;
}

fn findNodeById(nodes: []const LazyNodeStore.NodeEntry, node_id: i32) ?*const LazyNodeStore.NodeEntry {
    for (nodes) |*n| {
        if (n.id == node_id) return n;
    }
    return null;
}

fn getChildCondition(data: std.json.Value) ?std.json.ObjectMap {
    if (data != .object) return null;
    const c = data.object.get("Condition") orelse return null;
    return switch (c) {
        .object => |obj| obj,
        else => null,
    };
}

fn resolveConfigId(scene_opt: ?*const Scene, net_or_cfg: i64) i32 {
    if (scene_opt) |scene| {
        if (scene.net_id_map.get(net_or_cfg)) |idx| {
            return scene.entities.items(.config)[idx].config_id;
        }
    }
    if (net_or_cfg <= 0 or net_or_cfg > std.math.maxInt(i32)) return 0;
    return @intCast(net_or_cfg);
}

fn matchesReachRoleOption(ctx: QuestContext, cond: std.json.ObjectMap) bool {
    const opts = cond.get("MatchRoleOption") orelse return true;
    if (opts != .array or opts.array.items.len == 0) return true;
    const scene = ctx.scene orelse return false;
    const formation_idx = std.math.cast(usize, scene.formation_info.cur_formation) orelse return false;
    if (formation_idx >= scene.formation_info.formations.len) return false;
    const formation = scene.formation_info.formations[formation_idx];
    for (opts.array.items) |opt| {
        if (opt != .object) continue;
        const otype = jsonStr(opt.object.get("Type")) orelse "";
        if (std.mem.eql(u8, otype, "Player")) {
            if (formation.cur_role > 0) return true;
        } else if (std.mem.eql(u8, otype, "Phantom")) {
            const pid = jsonInt(opt.object.get("Id")) orelse 0;
            if (pid <= 0 or scene.trial_role_ids.items.len > 0) return true;
            for (formation.roles) |mr| {
                const r = mr orelse continue;
                if (r.role_id == pid) return true;
            }
            if (ctx.echo_comp.echo_map.count() > 0) return true;
        } else {
            return true;
        }
    }
    return false;
}

fn checkReachEntityRange(ctx: QuestContext, eid: i32, check_pos: [3]f32, default_range: f32) bool {
    var ent_pos: ?[3]f32 = null;
    var ent_rot: [3]f32 = .{ 0, 0, 0 };
    if (ctx.scene) |scene| {
        const slice = scene.entities.slice();
        for (slice.items(.config), slice.items(.position)) |cfg, pos_opt| {
            if (cfg.config_id == eid) {
                if (pos_opt) |p| {
                    ent_pos = p.location;
                    ent_rot = p.rotation;
                    break;
                }
            }
        }
    }
    var shape: @import("../../data/tables/entity_components/Range.zig").ShapeConfig = .{};
    if (ctx.scene) |scene| {
        if (ctx.assets.findLevelEntity(scene.instance_id, eid)) |lcfg| {
            if (ent_pos == null) {
                ent_pos = Assets.AssetIndexes.entityWorldPos(lcfg);
                ent_rot = Assets.AssetIndexes.entityWorldRot(lcfg);
            }
            const maybe_rc = lcfg.ComponentsData.RangeComponent orelse
                if (ctx.assets.findTemplateConfig(lcfg.BlueprintType)) |tpl| tpl.ComponentsData.RangeComponent else null;
            if (maybe_rc) |rc| {
                if (rc.Shape) |sh| shape = sh;
            }
        }
    }
    const base = ent_pos orelse return false;
    return ConditionEvaluator.isPosInsideRangeShape(check_pos, base, ent_rot, shape, default_range, 600.0);
}

pub fn matchesReachAreaPub(
    ctx: QuestContext,
    quest_id: i32,
    node_data: std.json.Value,
    cond: std.json.ObjectMap,
    pos: [3]f32,
) bool {
    return matchesReachArea(ctx, quest_id, node_data, cond, pos);
}

fn matchesReachArea(
    ctx: QuestContext,
    quest_id: i32,
    node_data: std.json.Value,
    cond: std.json.ObjectMap,
    pos: [3]f32,
) bool {
    if (cond.get("PreConditions")) |pre_val| {
        if (!ConditionEvaluator.evalInlineConditionBlock(makeEvalContextForQuest(ctx, quest_id), pre_val)) {
            return false;
        }
    }
    if (!matchesReachRoleOption(ctx, cond)) return false;

    var check_pos = pos;
    if (jsonInt(cond.get("EntityId"))) |target_eid| {
        if (target_eid > 0) {
            const scene = ctx.scene orelse return false;
            var found_pos: ?[3]f32 = null;
            const slice = scene.entities.slice();
            for (slice.items(.config), slice.items(.position)) |cfg, p_opt| {
                if (cfg.config_id == target_eid) {
                    if (p_opt) |p| {
                        found_pos = p.location;
                        break;
                    }
                }
            }
            check_pos = found_pos orelse return false;
        }
    }

    const raw_range = jsonFloat(cond.get("Range")) orelse 1000.0;
    const range: f32 = @max(raw_range, 600.0);

    if (cond.get("RangeEntities")) |re_val| {
        if (re_val == .array and re_val.array.items.len > 0) {
            for (re_val.array.items) |ev| {
                const eid = jsonInt(ev) orelse continue;
                if (checkReachEntityRange(ctx, eid, check_pos, range)) return true;
            }
            return false;
        }
    }
    if (jsonInt(cond.get("RangeEntityId"))) |reid| {
        if (reid > 0) {
            return checkReachEntityRange(ctx, reid, check_pos, range);
        }
    }

    if (cond.get("Pos")) |pv| {
        if (pv == .object and checkPoint(pv.object, check_pos, range)) return true;
    }
    if (cond.get("Center")) |cv| {
        if (cv == .object and checkPoint(cv.object, check_pos, range)) return true;
    }
    if (cond.get("Locations")) |lv| {
        if (checkPointArray(lv, check_pos, range)) return true;
    }
    if (node_data == .object) {
        if (node_data.object.get("TrackTarget")) |tt| {
            if (tt == .object) {
                if (tt.object.get("TrackType")) |track_type| {
                    if (track_type == .object) {
                        if (checkPointArray(track_type.object.get("Locations"), check_pos, range)) return true;
                    }
                }
            }
        }
    }
    return false;
}

fn checkPointArray(arr_opt: ?std.json.Value, pos: [3]f32, range: f32) bool {
    const lv = arr_opt orelse return false;
    if (lv != .array) return false;
    for (lv.array.items) |pt| {
        if (pt == .object and checkPoint(pt.object, pos, range)) return true;
    }
    return false;
}

fn checkPoint(pt: std.json.ObjectMap, pos: [3]f32, range: f32) bool {
    const tx = jsonFloat(pt.get("X")) orelse return false;
    const ty = jsonFloat(pt.get("Y")) orelse return false;
    const tz = jsonFloat(pt.get("Z")) orelse pos[2];

    const dx = pos[0] - tx;
    const dy = pos[1] - ty;
    const dz = @abs(pos[2] - tz);
    return (dx * dx + dy * dy <= range * range) and (dz <= @max(range, 3000.0));
}

fn matchesQuestInteractStarter(ctx: QuestContext, d: QuestData.DataEntry, entity_id: i64, cfg_id: i32) bool {
    if (d.AddInteractOption) |aio| {
        if (aio == .object) {
            if (jsonInt(aio.object.get("EntityId"))) |eid| {
                if (eid == cfg_id or @as(i64, eid) == entity_id) return true;
            }
        }
    }
    const nodes = ctx.assets.quest_nodes.getOwnerNodes(ctx.io, d.Id) catch return false;
    for (nodes) |n| {
        if (!std.mem.eql(u8, n.node_type, "ChildQuest")) continue;
        const cond = getChildCondition(n.data) orelse continue;
        const ct = jsonStr(cond.get("Type")) orelse continue;
        if (std.mem.eql(u8, ct, "DoInteract")) {
            if (matchDoInteract(cond, entity_id, cfg_id, 0) != null) return true;
        }
    }
    return false;
}

fn matchDoInteract(cond: std.json.ObjectMap, entity_id: i64, cfg_id: i32, option_index: i32) ?std.json.ObjectMap {
    const opts = cond.get("AddOptions") orelse return null;
    if (opts != .array) return null;

    var matched_any: ?std.json.ObjectMap = null;
    var match_i: i32 = 0;
    for (opts.array.items) |opt| {
        if (opt != .object) continue;
        const eid = jsonInt(opt.object.get("EntityId")) orelse continue;
        if (eid == cfg_id or @as(i64, eid) == entity_id) {
            if (match_i == option_index) return opt.object;
            if (matched_any == null) matched_any = opt.object;
            match_i += 1;
        }
    }
    return matched_any;
}

fn matchesHandInEntity(cond: std.json.ObjectMap, entity_id: i64, cfg_id: i32) bool {
    const add_opt = cond.get("AddOption") orelse blk: {
        const hi = cond.get("HandInItems") orelse return false;
        if (hi != .object) return false;
        break :blk hi.object.get("AddOption") orelse return false;
    };
    if (add_opt != .object) return false;
    const eid = jsonInt(add_opt.object.get("EntityId")) orelse return false;
    return eid == cfg_id or @as(i64, eid) == entity_id;
}

fn isKillSatisfied(aq: QuestInfo.ActiveQuest, cond: std.json.ObjectMap, ctype: []const u8) bool {
    if (std.mem.eql(u8, ctype, "MonsterCreator")) {
        const list = cond.get("MonsterCreatorEntityIds") orelse return aq.killed_entities.len > 0;
        if (list != .array or list.array.items.len == 0) return aq.killed_entities.len > 0;
        for (list.array.items) |v| {
            const eid = jsonInt(v) orelse continue;
            if (!QuestInfo.hasKilledEntity(aq, eid)) return false;
        }
        return true;
    }

    var total: usize = 0;
    var killed: usize = 0;
    inline for (.{ "TargetsToAwake", "ExistTargets" }) |field| {
        if (cond.get(field)) |arr| {
            if (arr == .array) {
                for (arr.array.items) |v| {
                    const eid = jsonInt(v) orelse continue;
                    total += 1;
                    if (QuestInfo.hasKilledEntity(aq, eid)) killed += 1;
                }
            }
        }
    }
    if (total == 0) return aq.killed_entities.len > 0;
    return killed >= total;
}

fn evalCompareVarList(ctx: QuestContext, cond: std.json.ObjectMap) bool {
    const conds = cond.get("Conditions") orelse return true;
    if (conds != .array or conds.array.items.len == 0) return true;
    const needed = jsonInt(cond.get("Count")) orelse 0;
    const eval_ctx = makeEvalContext(ctx);

    var passed: i32 = 0;
    for (conds.array.items) |item| {
        if (item != .object) continue;
        if (ConditionEvaluator.evalCompareVarCondition(eval_ctx, item.object)) {
            passed += 1;
        }
    }
    return if (needed <= 0) passed == @as(i32, @intCast(conds.array.items.len)) else passed >= needed;
}

fn isLevelPlaySatisfied(ctx: QuestContext, cond: std.json.ObjectMap) bool {
    const ids_val = cond.get("LevelPlayIds") orelse return false;
    if (ids_val != .array or ids_val.array.items.len == 0) return false;
    const req = @max(1, jsonInt(cond.get("CompleteNumber")) orelse 1);

    var done: i32 = 0;
    for (ids_val.array.items) |v| {
        const lpid = jsonInt(v) orelse continue;
        if (QuestInfo.containsInt(ctx.quest_comp.info.completed_level_plays, lpid)) {
            done += 1;
        }
    }
    return done >= req;
}

fn hasAllItems(ctx: QuestContext, items_opt: ?std.json.Value) bool {
    const arr = items_opt orelse return true;
    if (arr != .array) return true;
    for (arr.array.items) |it| {
        if (it != .object) continue;
        const iid = jsonInt(it.object.get("ItemId")) orelse continue;
        const cnt = jsonInt(it.object.get("Count")) orelse 1;
        const owned: i32 = if (iid == 2)
            ctx.basic_comp.info.attributes.coin
        else if (iid == 3)
            @max(ctx.basic_comp.info.attributes.rare_coin, ctx.inventory_comp.info.normalItemCount(3))
        else
            ctx.inventory_comp.info.normalItemCount(iid);
        if (owned < cnt) return false;
    }
    return true;
}

fn nestedDungeonId(cond: std.json.ObjectMap, field: []const u8) i32 {
    const sub = cond.get(field) orelse return 0;
    if (sub != .object) return 0;
    return jsonInt(sub.object.get("DungeonId")) orelse 0;
}

fn isCheckEntityStateSatisfied(ctx: QuestContext, cond: std.json.ObjectMap) bool {
    const cs = cond.get("Conditions") orelse return true;
    if (cs != .array or cs.array.items.len == 0) return true;
    for (cs.array.items) |item| {
        if (item != .object) continue;
        const eid = jsonInt(item.object.get("EntityId")) orelse continue;
        const want_state = jsonStr(item.object.get("State")) orelse continue;
        if (!entityHasStateTag(ctx, eid, want_state)) return false;
    }
    return true;
}

fn entityHasTagId(ctx: QuestContext, target_cfg: i32, tag_id: i32) bool {
    if (target_cfg <= 0 or tag_id == 0) return false;
    const scene = ctx.scene orelse return false;
    const slice = scene.entities.slice();
    const configs = slice.items(.config);
    const tags = slice.items(.tag);
    for (configs, tags) |cfg, tag_opt| {
        if (cfg.config_id != target_cfg) continue;
        const tag_comp = tag_opt orelse continue;
        if (tag_comp.hasTag(&ctx.assets.tables.gameplay_tag_parent, tag_id)) return true;
    }
    return false;
}

fn entityHpPercent(ctx: QuestContext, target_cfg: i32) ?f32 {
    if (target_cfg <= 0) return null;
    const scene = ctx.scene orelse return null;
    const slice = scene.entities.slice();
    const configs = slice.items(.config);
    const attrs = slice.items(.attribute);
    const life_idx: usize = @intCast(@intFromEnum(pb.EAttributeType.Life));
    const max_idx: usize = @intCast(@intFromEnum(pb.EAttributeType.LifeMax));
    for (configs, attrs) |cfg, attr_opt| {
        if (cfg.config_id != target_cfg) continue;
        const attr = attr_opt orelse continue;
        const max_hp = attr.attributes[max_idx].current;
        if (max_hp <= 0) continue;
        const cur_hp = attr.attributes[life_idx].current;
        return @as(f32, @floatFromInt(cur_hp)) / @as(f32, @floatFromInt(max_hp)) * 100.0;
    }
    return null;
}

fn isBattleAttrOrWaitSatisfied(
    ctx: QuestContext,
    cond: std.json.ObjectMap,
    ctype: []const u8,
    hit_cfg_id: i32,
    hp_ratio: f32,
) bool {
    const opts = cond.get("Option") orelse return true;
    if (opts != .array or opts.array.items.len == 0) return true;
    const req_count = jsonInt(cond.get("Count")) orelse 0;
    var passed: i32 = 0;

    for (opts.array.items) |o| {
        if (o != .object) continue;
        if (std.mem.eql(u8, ctype, "CheckTargetBattleAttribute")) {
            const inner = o.object.get("Option");
            const min_pct: f32 = if (inner) |iv|
                if (iv == .object) jsonFloat(iv.object.get("Min")) orelse 0 else 0
            else
                0;
            const max_pct: f32 = if (inner) |iv|
                if (iv == .object) jsonFloat(iv.object.get("Max")) orelse 100 else 100
            else
                100;
            if (o.object.get("EntityIds")) |eids| {
                if (eids == .array) {
                    for (eids.array.items) |v| {
                        const eid = jsonInt(v) orelse continue;
                        const pct_opt: ?f32 = if (hit_cfg_id != 0 and eid == hit_cfg_id)
                            hp_ratio * 100.0
                        else
                            entityHpPercent(ctx, eid);
                        if (pct_opt) |pct| {
                            const in_range = pct >= min_pct and pct <= max_pct;
                            if (in_range) {
                                passed += 1;
                                break;
                            }
                        }
                    }
                }
            } else if (std.mem.eql(u8, jsonStr(o.object.get("Type")) orelse "", "Player")) {
                passed += 1;
            }
        } else {
            const eid = jsonInt(o.object.get("EntityId")) orelse 0;
            const state_str = jsonStr(o.object.get("State")) orelse "";
            const tag_cfg_id = jsonInt(o.object.get("TagConfigId")) orelse 0;
            const has_tag_or_state = state_str.len > 0 or tag_cfg_id != 0;
            if ((state_str.len > 0 and entityHasStateTag(ctx, eid, state_str)) or
                (tag_cfg_id != 0 and entityHasTagId(ctx, eid, tag_cfg_id)) or
                (!has_tag_or_state and hit_cfg_id != 0 and (eid == 0 or eid == hit_cfg_id) and hp_ratio <= 0.75))
            {
                passed += 1;
            }
        }
    }

    const needed: i32 = if (req_count <= 0) @intCast(opts.array.items.len) else req_count;
    return passed >= needed;
}

fn isScheduleTimeSatisfied(ctx: QuestContext, cond: std.json.ObjectMap) bool {
    const h: i32 = if (ctx.quest_comp.info.env.time_locked)
        ctx.quest_comp.info.env.time_lock_hour
    else if (ctx.scene) |s|
        s.instance.map_time.hour
    else
        ctx.quest_comp.info.env.time_lock_hour;
    const m: i32 = if (ctx.quest_comp.info.env.time_locked)
        ctx.quest_comp.info.env.time_lock_minute
    else if (ctx.scene) |s|
        s.instance.map_time.minute
    else
        ctx.quest_comp.info.env.time_lock_minute;
    const d: u32 = if (ctx.scene) |s| s.instance.map_time.day else 1;
    return isScheduleTimeWindowMatch(cond, h, m, d);
}

fn isScheduleTimeWindowMatch(cond: std.json.ObjectMap, cur_hour: i32, cur_min: i32, cur_day: u32) bool {
    if (jsonInt(cond.get("Day"))) |req_day| {
        if (req_day > 0 and cur_day < @as(u32, @intCast(req_day))) return false;
    }
    const start_obj = cond.get("StartTime") orelse return true;
    if (start_obj != .object) return true;
    const sh = jsonInt(start_obj.object.get("Hour")) orelse 0;
    const sm = jsonInt(start_obj.object.get("Minute")) orelse 0;
    const cur_total = @mod(cur_hour, 24) * 60 + @mod(cur_min, 60);
    const start_total = @mod(sh, 24) * 60 + @mod(sm, 60);

    if (cond.get("EndTime")) |end_obj| {
        if (end_obj == .object) {
            const eh = jsonInt(end_obj.object.get("Hour")) orelse sh;
            const em = jsonInt(end_obj.object.get("Minute")) orelse sm;
            const end_total = @mod(eh, 24) * 60 + @mod(em, 60);
            if (start_total <= end_total) {
                return cur_total >= start_total and cur_total <= end_total;
            }
            return cur_total >= start_total or cur_total <= end_total;
        }
    }
    return cur_total >= start_total;
}

test "parseTimerDelayMs supports WaitTime Days/Hours/Minutes/Seconds and Timer" {
    const alloc = std.testing.allocator;
    const wt = try std.json.parseFromSlice(std.json.Value, alloc,
        \\{"Type":"WaitTime","Days":0,"Hours":1,"Minutes":0,"Seconds":0}
    , .{});
    defer wt.deinit();
    try std.testing.expectEqual(@as(?i64, 8000), parseTimerDelayMs(wt.value.object));

    const tm = try std.json.parseFromSlice(std.json.Value, alloc,
        \\{"Type":"Timer","Time":5}
    , .{});
    defer tm.deinit();
    try std.testing.expectEqual(@as(?i64, 5000), parseTimerDelayMs(tm.value.object));
}

test "extractNodeTargetEntityIds captures TakePhoto2 and CheckEntityState targets" {
    const alloc = std.testing.allocator;
    const parsed = try std.json.parseFromSlice(std.json.Value, alloc,
        \\{"Condition":{"Type":"TakePhoto2","PhotoTargets":[{"EntityId":181850024}],"PosCondition":{"RangeEntity":181850026}}}
    , .{});
    defer parsed.deinit();
    var buf: [16]i32 = undefined;
    const ids = extractNodeTargetEntityIds(&buf, .{
        .id = 1,
        .parent_id = 0,
        .node_type = "ChildQuest",
        .count = 0,
        .reward_id = 0,
        .data = parsed.value,
    });
    try std.testing.expectEqualSlices(i32, &.{ 181850024, 181850026 }, ids);
}

test "isScheduleTimeWindowMatch handles daytime and wraparound windows" {
    const alloc = std.testing.allocator;
    const day = try std.json.parseFromSlice(std.json.Value, alloc,
        \\{"Type":"ScheduleTime","StartTime":{"Hour":15,"Minute":0},"EndTime":{"Hour":17,"Minute":0}}
    , .{});
    defer day.deinit();
    try std.testing.expect(isScheduleTimeWindowMatch(day.value.object, 16, 0, 1));
    try std.testing.expect(!isScheduleTimeWindowMatch(day.value.object, 12, 0, 1));

    const night = try std.json.parseFromSlice(std.json.Value, alloc,
        \\{"Type":"ScheduleTime","StartTime":{"Hour":22,"Minute":0},"EndTime":{"Hour":4,"Minute":0}}
    , .{});
    defer night.deinit();
    try std.testing.expect(isScheduleTimeWindowMatch(night.value.object, 23, 30, 1));
    try std.testing.expect(isScheduleTimeWindowMatch(night.value.object, 2, 0, 1));
    try std.testing.expect(!isScheduleTimeWindowMatch(night.value.object, 10, 0, 1));
}

test "isClientActionNameRegistered allows safe client LevelEvents and filters server/unsafe actions" {
    try std.testing.expect(isClientActionNameRegistered("FadeInScreen"));
    try std.testing.expect(isClientActionNameRegistered("ChangeEntityState"));
    try std.testing.expect(isClientActionNameRegistered("PlayMontage"));
    try std.testing.expect(isClientActionNameRegistered("Prompt"));

    // Server-only actions absent from LevelEventCenter must stay server-side
    try std.testing.expect(!isClientActionNameRegistered("AwakeEntity"));
    try std.testing.expect(!isClientActionNameRegistered("HideGroup"));
    try std.testing.expect(!isClientActionNameRegistered("RemoveBuffFromPlayer"));
    try std.testing.expect(!isClientActionNameRegistered("SetVar"));
    try std.testing.expect(!isClientActionNameRegistered("UnlockTeleportTrigger"));

    // Unsafe cast or TriggerContext-only handlers in GeneralLogicTreeContext
    try std.testing.expect(!isClientActionNameRegistered("LeisureInteract"));
    try std.testing.expect(!isClientActionNameRegistered("AddBuffToPlayer"));
    try std.testing.expect(!isClientActionNameRegistered("AddBuffToEntity"));
    try std.testing.expect(!isClientActionNameRegistered("OpenSystemFunction"));
}

test "isSafeBtLeisureInteract allows context-independent options and explicit sit targets" {
    const allocator = std.testing.allocator;
    const custom_skill = try std.json.parseFromSlice(std.json.Value, allocator,
        \\{"Name":"LeisureInteract","Params":{"Option":{"Type":"CustomSkill","SkillId":800022}}}
    , .{});
    defer custom_skill.deinit();
    try std.testing.expect(isSafeBtLeisureInteract(custom_skill.value));

    const sit_with_target = try std.json.parseFromSlice(std.json.Value, allocator,
        \\{"Name":"LeisureInteract","Params":{"Option":{"Type":"SitDown"},"SceneEntity":38750004}}
    , .{});
    defer sit_with_target.deinit();
    try std.testing.expect(isSafeBtLeisureInteract(sit_with_target.value));

    const sit_without_target = try std.json.parseFromSlice(std.json.Value, allocator,
        \\{"Name":"LeisureInteract","Params":{"Option":{"Type":"SitDown"}}}
    , .{});
    defer sit_without_target.deinit();
    try std.testing.expect(!isSafeBtLeisureInteract(sit_without_target.value));

    const context_cast = try std.json.parseFromSlice(std.json.Value, allocator,
        \\{"Name":"LeisureInteract","Params":{"Option":{"Type":"Catapult"}}}
    , .{});
    defer context_cast.deinit();
    try std.testing.expect(!isSafeBtLeisureInteract(context_cast.value));
}

test "reentrant refreshAvailableQuests and shouldHideWorldEntity prevent recursion and ghost spawns" {
    const gpa = std.testing.allocator;

    refresh_loop_depth = 1;
    pending_refresh_quests = false;
    pending_refresh_push = false;
    pending_recheck_conditions = false;
    defer {
        refresh_loop_depth = 0;
        pending_refresh_quests = false;
        pending_refresh_push = false;
        pending_recheck_conditions = false;
    }

    const dummy_ctx: QuestContext = undefined;
    try std.testing.expect(!(try refreshAvailableQuests(dummy_ctx, true)));
    try std.testing.expect(pending_refresh_quests);
    try std.testing.expect(pending_refresh_push);

    try std.testing.expect(!(try recheckPendingConditions(dummy_ctx)));
    try std.testing.expect(pending_recheck_conditions);

    var targets: std.AutoHashMapUnmanaged(i32, void) = .empty;
    defer targets.deinit(gpa);
    try targets.put(gpa, 310000014, {});

    var hidden_slice = [_]i32{310000014};
    var shown_slice = [_]i32{880700164};
    var info: QuestInfo = .default;
    info.world.hidden_entities = &hidden_slice;
    info.world.shown_entities = &shown_slice;

    // Explicitly hidden/destroyed entity stays hidden even if still in active quest targets
    try std.testing.expect(shouldHideWorldEntity(info, &targets, 310000014));
    // Explicitly shown entity is not hidden
    try std.testing.expect(!shouldHideWorldEntity(info, &targets, 880700164));
}

test "allowsClientSubmitNode blocks DoInteract and PlayFlow while allowing ReachArea" {
    const gpa = std.testing.allocator;

    const interact_node = try std.json.parseFromSlice(std.json.Value, gpa,
        \\{"Type":"ChildQuest","Condition":{"Type":"DoInteract"}}
    , .{});
    defer interact_node.deinit();
    try std.testing.expect(!allowsClientSubmitNode(interact_node.value));

    const playflow_node = try std.json.parseFromSlice(std.json.Value, gpa,
        \\{"Type":"ChildQuest","Condition":{"Type":"PlayFlow"}}
    , .{});
    defer playflow_node.deinit();
    try std.testing.expect(!allowsClientSubmitNode(playflow_node.value));

    const reach_node = try std.json.parseFromSlice(std.json.Value, gpa,
        \\{"Type":"ChildQuest","Condition":{"Type":"ReachArea"}}
    , .{});
    defer reach_node.deinit();
    try std.testing.expect(allowsClientSubmitNode(reach_node.value));
}
