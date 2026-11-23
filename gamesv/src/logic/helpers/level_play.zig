const std = @import("std");
const pb = @import("proto").pb;
const Assets = @import("../../data/Assets.zig");
const Components = @import("../../data/tables/entity_components/Components.zig");
const LazyNodeStore = @import("../../data/LazyNodeStore.zig");
const FileSystem = @import("common").FileSystem;
const Connection = @import("../../network/Connection.zig");
const Scene = @import("../Scene.zig");
const EventQueue = @import("../EventQueue.zig");
const mem = @import("../../mem.zig");
const PlayerBasicComponent = @import("../component/player/PlayerBasicComponent.zig");
const PlayerInventoryComponent = @import("../component/player/PlayerInventoryComponent.zig");
const PlayerWeaponComponent = @import("../component/player/PlayerWeaponComponent.zig");
const PlayerEchoComponent = @import("../component/player/PlayerEchoComponent.zig");
const PlayerMapComponent = @import("../component/player/PlayerMapComponent.zig");
const PlayerQuestComponent = @import("../component/player/PlayerQuestComponent.zig");
const PlayerRoleComponent = @import("../component/player/PlayerRoleComponent.zig");
const PlayerGuideComponent = @import("../component/player/PlayerGuideComponent.zig");
const PlayerMailComponent = @import("../component/player/PlayerMailComponent.zig");
const PlayerProgressComponent = @import("../component/player/PlayerProgressComponent.zig");
const PlayerSceneComponent = @import("../component/player/PlayerSceneComponent.zig");
const RewardService = @import("reward.zig");
const QuestEngine = @import("quest_engine.zig");
const quest_types = @import("quest_types.zig");
const ConditionEvaluator = @import("condition.zig");
const QuestInfo = @import("../../fs/QuestInfo.zig");
const prologue_quest = @import("prologue_quest.zig");

const log = std.log.scoped(.level_play);

pub const LevelPlayContext = struct {
    assets: *const Assets,
    fs: *FileSystem,
    conn: *Connection,
    scene: *Scene,
    events: *EventQueue,
    alloc: mem.Alloc,
    io: std.Io,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    map_comp: *PlayerMapComponent,
    quest_comp: *PlayerQuestComponent,
    role_comp: *PlayerRoleComponent,
    scene_comp: ?*PlayerSceneComponent = null,
    guide_comp: ?*PlayerGuideComponent = null,
    mail_comp: ?*PlayerMailComponent = null,
    progress_comp: ?*PlayerProgressComponent = null,
};

pub const ResolvedEntityConfig = struct {
    blueprint_type: []const u8,
    components: Components,
};

fn toRewardCtx(ctx: LevelPlayContext) RewardService.RewardContext {
    return .{
        .alloc = ctx.alloc,
        .io = ctx.io,
        .fs = ctx.fs,
        .assets = ctx.assets,
        .conn = ctx.conn,
        .basic_comp = ctx.basic_comp,
        .inventory_comp = ctx.inventory_comp,
        .weapon_comp = ctx.weapon_comp,
        .echo_comp = ctx.echo_comp,
    };
}

fn toQuestCtx(ctx: LevelPlayContext) QuestEngine.QuestContext {
    return .{
        .assets = ctx.assets,
        .fs = ctx.fs,
        .conn = ctx.conn,
        .scene = ctx.scene,
        .scene_comp = ctx.scene_comp,
        .events = ctx.events,
        .alloc = ctx.alloc,
        .io = ctx.io,
        .basic_comp = ctx.basic_comp,
        .inventory_comp = ctx.inventory_comp,
        .weapon_comp = ctx.weapon_comp,
        .echo_comp = ctx.echo_comp,
        .map_comp = ctx.map_comp,
        .quest_comp = ctx.quest_comp,
        .role_comp = ctx.role_comp,
        .guide_comp = ctx.guide_comp,
        .mail_comp = ctx.mail_comp,
        .progress_comp = ctx.progress_comp,
    };
}

pub fn resolveEntityConfig(assets: *const Assets, scene_map_id: i32, config_id: i64) ?ResolvedEntityConfig {
    if (config_id <= 0) return null;

    var matched: ?*const Assets.DataTables.LevelEntityConfig = assets.findLevelEntity(scene_map_id, config_id);
    if (matched == null) {
        const indices = assets.indexes.levelEntityIndicesById(config_id);
        if (indices.len > 0) {
            matched = assets.getLevelEntityByIdx(indices[0]);
        }
    }

    if (matched) |cfg| {
        var comps = cfg.ComponentsData;
        if (assets.findTemplateConfig(cfg.BlueprintType)) |tpl| {
            tpl.ComponentsData.mergeInto(&comps);
        }
        return .{
            .blueprint_type = cfg.BlueprintType,
            .components = comps,
        };
    }

    if (std.math.cast(i32, config_id)) |tid| {
        if (assets.tables.template_config.getDataById(tid)) |tpl| {
            return .{
                .blueprint_type = tpl.BlueprintType,
                .components = tpl.ComponentsData,
            };
        }
    }

    return null;
}

fn matchesSceneInstance(assets: *const Assets, lp: Assets.DataTables.LevelPlayData, scene_instance_id: i32) bool {
    const expected_inst: i32 = if (lp.Data.InstanceId != 0) lp.Data.InstanceId else lp.Data.LevelId;
    if (expected_inst == 0 or expected_inst == scene_instance_id) return true;
    return assets.resolveMapConfigId(scene_instance_id) == expected_inst;
}

fn jsonFloat(v_opt: ?std.json.Value) ?f32 {
    const v = v_opt orelse return null;
    return switch (v) {
        .integer => |n| @floatFromInt(n),
        .float => |f| if (std.math.isFinite(f)) @floatCast(f) else null,
        else => null,
    };
}

fn isPlayerWithinReachArea(ctx: LevelPlayContext, node_data: std.json.Value, cond_obj: std.json.ObjectMap, pos: [3]f32) bool {
    return QuestEngine.matchesReachAreaPub(toQuestCtx(ctx), 0, node_data, cond_obj, pos);
}

fn jsonInt(v_opt: ?std.json.Value) ?i32 {
    const v = v_opt orelse return null;
    return switch (v) {
        .integer => |n| std.math.cast(i32, n),
        .float => |f| {
            if (!std.math.isFinite(f)) return null;
            const min_f: f64 = @floatFromInt(std.math.minInt(i32));
            const max_f: f64 = @floatFromInt(std.math.maxInt(i32));
            if (f < min_f or f > max_f) return null;
            return @intFromFloat(f);
        },
        else => null,
    };
}

fn jsonStr(v_opt: ?std.json.Value) ?[]const u8 {
    const v = v_opt orelse return null;
    return switch (v) {
        .string => |s| s,
        else => null,
    };
}

fn jsonArrayContainsInt(arr_opt: ?std.json.Value, needle: i32) bool {
    const v = arr_opt orelse return false;
    if (v != .array) return false;
    for (v.array.items) |item| {
        if (jsonInt(item)) |id| {
            if (id == needle) return true;
        }
    }
    return false;
}

fn isTargetStillAliveInScene(scene: *Scene, cfg_id: i32, exclude_net_id: i64) bool {
    const slice = scene.entities.slice();
    const configs = slice.items(.config);
    const net_ids = slice.items(.entity_id);
    for (configs, net_ids) |cfg, nid| {
        if (nid.net_id == exclude_net_id) continue;
        if (scene.rewarded_kills.contains(nid.net_id)) continue;
        if (cfg.config_id == cfg_id) return true;
    }
    return false;
}

fn ensureEntitySpawned(ctx: LevelPlayContext, config_id: i32) void {
    if (config_id <= 0) return;
    const slice = ctx.scene.entities.slice();
    for (slice.items(.config)) |cfg| {
        if (cfg.config_id == config_id) return;
    }
    if (ctx.quest_comp.info.hasOpenedChest(config_id)) return;
    prologue_quest.spawnQuestEntity(
        ctx.events,
        ctx.scene,
        ctx.fs,
        ctx.assets,
        ctx.conn,
        ctx.basic_comp,
        ctx.alloc,
        ctx.io,
        config_id,
    );
}

var lp_step_depth: u32 = 0;
var lp_recheck_pending: bool = false;
var lp_deferred_bt_vars: ?*std.ArrayList(QuestEngine.DeferredBtVar) = null;

const LpStepResult = enum {
    running,
    completed,
    failed,
    tree_ended,
};

fn lpKeyBuf(buf: *[48]u8, prefix: []const u8, a: i32, b: i32) []const u8 {
    return std.fmt.bufPrint(buf, "{s}:{d}:{d}", .{ prefix, a, b }) catch "";
}

fn isLpNodeActive(progress_comp: ?*const PlayerProgressComponent, lp_id: i32, node_id: i32) bool {
    const pc = progress_comp orelse return false;
    var buf: [48]u8 = undefined;
    const key = lpKeyBuf(&buf, "lp_act", lp_id, node_id);
    return key.len > 0 and pc.info.getKeyInt(key) != 0;
}

fn setLpNodeActive(ctx: LevelPlayContext, lp_id: i32, node_id: i32, active: bool, dirty_pc: *bool) void {
    const pc = ctx.progress_comp orelse return;
    var buf: [48]u8 = undefined;
    const key = lpKeyBuf(&buf, "lp_act", lp_id, node_id);
    if (key.len == 0) return;
    const target: i64 = if (active) 1 else 0;
    if (pc.info.getKeyInt(key) != target) {
        pc.info.setKeyInt(ctx.alloc.gpa, key, target) catch {};
        dirty_pc.* = true;
    }
}

fn markLpNodeFinished(ctx: LevelPlayContext, lp_id: i32, node_id: i32, dirty_pc: *bool) void {
    setLpNodeActive(ctx, lp_id, node_id, false, dirty_pc);
    const pc = ctx.progress_comp orelse return;
    var buf: [48]u8 = undefined;
    const key = lpKeyBuf(&buf, "lp_node", lp_id, node_id);
    if (key.len == 0) return;
    if (pc.info.markClaim(ctx.alloc.gpa, key) catch false) {
        dirty_pc.* = true;
    }
    if (pc.info.getKeyInt(key) == 0) {
        pc.info.setKeyInt(ctx.alloc.gpa, key, 1) catch {};
        dirty_pc.* = true;
    }
}

fn isLpBtBooted(progress_comp: ?*const PlayerProgressComponent, lp_id: i32) bool {
    const pc = progress_comp orelse return false;
    var buf: [48]u8 = undefined;
    const key = std.fmt.bufPrint(&buf, "lp_bt:{d}", .{lp_id}) catch return false;
    return pc.info.getKeyInt(key) != 0;
}

fn markLpBtBooted(ctx: LevelPlayContext, lp_id: i32, dirty_pc: *bool) void {
    const pc = ctx.progress_comp orelse return;
    var buf: [48]u8 = undefined;
    const key = std.fmt.bufPrint(&buf, "lp_bt:{d}", .{lp_id}) catch return;
    if (pc.info.getKeyInt(key) == 0) {
        pc.info.setKeyInt(ctx.alloc.gpa, key, 1) catch {};
        dirty_pc.* = true;
    }
}

fn findLpNodeById(nodes: []const LazyNodeStore.NodeEntry, node_id: i32) ?*const LazyNodeStore.NodeEntry {
    for (nodes) |*node| {
        if (node.id == node_id) return node;
    }
    return null;
}

fn findLpRootNode(nodes: []const LazyNodeStore.NodeEntry) ?LazyNodeStore.NodeEntry {
    for (nodes) |node| {
        if (node.parent_id == 0) return node;
    }
    return if (nodes.len > 0) nodes[0] else null;
}

fn resolveLevelPlayAnchorPos(ctx: LevelPlayContext, lp: Assets.DataTables.LevelPlayData) ?[3]f32 {
    if (lp.Data.LevelPlayEntityId <= 0) return null;
    const slice = ctx.scene.entities.slice();
    for (slice.items(.config), slice.items(.position)) |cfg, pos_opt| {
        if (cfg.config_id != lp.Data.LevelPlayEntityId) continue;
        if (pos_opt) |pos| return pos.location;
    }
    if (ctx.assets.findLevelEntity(ctx.scene.instance_id, lp.Data.LevelPlayEntityId)) |cfg| {
        return Assets.AssetIndexes.entityWorldPos(cfg);
    }
    const indices = ctx.assets.indexes.levelEntityIndicesById(lp.Data.LevelPlayEntityId);
    if (indices.len > 0) {
        if (ctx.assets.getLevelEntityByIdx(indices[0])) |cfg| {
            return Assets.AssetIndexes.entityWorldPos(cfg);
        }
    }
    return null;
}

fn currentPlayerPos(ctx: LevelPlayContext) ?[3]f32 {
    for (ctx.scene.instance.players) |player| {
        if (player.id == ctx.scene.player_id) {
            return player.location;
        }
    }
    if (ctx.scene.instance.players.len > 0) {
        return ctx.scene.instance.players[0].location;
    }
    return null;
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
    for (quest_info.progress.entered_level_plays) |lp_id| {
        const nodes = assets.level_play_nodes.getOwnerNodes(io, lp_id) catch continue;
        for (nodes) |node| {
            if (!isLpNodeActive(progress_comp, lp_id, node.id)) continue;
            try QuestEngine.pushDoInteractOptionsForBtNode(scene, conn, alloc, io, .BtTypeLevelPlay, lp_id, node, cfg_id);
        }
    }
}

fn conditionReferencesLevelPlayId(cond: std.json.ObjectMap, lp_id: i32) bool {
    if ((jsonInt(cond.get("LevelPlayId")) orelse 0) == lp_id) return true;
    if (cond.get("LevelPlayIds")) |ids_val| {
        if (ids_val == .array) {
            for (ids_val.array.items) |v| {
                if ((jsonInt(v) orelse 0) == lp_id) return true;
            }
        }
    }
    return false;
}

fn appendConditionLevelPlayIds(arena: std.mem.Allocator, out: *std.ArrayList(i32), cond: std.json.ObjectMap) !void {
    if (jsonInt(cond.get("LevelPlayId"))) |lpid| {
        if (lpid > 0 and !QuestInfo.containsInt(out.items, lpid)) {
            try out.append(arena, lpid);
        }
    }
    if (cond.get("LevelPlayIds")) |ids_val| {
        if (ids_val == .array) {
            for (ids_val.array.items) |v| {
                const lpid = jsonInt(v) orelse continue;
                if (lpid > 0 and !QuestInfo.containsInt(out.items, lpid)) {
                    try out.append(arena, lpid);
                }
            }
        }
    }
}

fn isQuestCheckingLevelPlay(ctx: LevelPlayContext, lp_id: i32) bool {
    for (ctx.quest_comp.info.active_quests) |aq| {
        if (aq.active_nodes.len == 0) continue;
        const nodes = ctx.assets.quest_nodes.getOwnerNodes(ctx.io, aq.quest_id) catch continue;
        for (aq.active_nodes) |nid| {
            const node = findLpNodeById(nodes, nid) orelse continue;
            if (!std.mem.eql(u8, node.node_type, "ChildQuest")) continue;
            const cond = QuestEngine.getChildConditionPub(node.data) orelse continue;
            const ctype = jsonStr(cond.get("Type")) orelse continue;
            if (!std.mem.eql(u8, ctype, "CheckLevelPlay")) continue;
            if (conditionReferencesLevelPlayId(cond, lp_id)) return true;
        }
    }
    return false;
}

fn canEnterLevelPlay(ctx: LevelPlayContext, lp: Assets.DataTables.LevelPlayData) bool {
    if (!matchesSceneInstance(ctx.assets, lp, ctx.scene.instance_id)) return false;
    if (QuestInfo.containsInt(ctx.quest_comp.info.completed_level_plays, lp.LevelPlayId)) return false;
    if (!std.mem.eql(u8, lp.Data.Type, "Quest")) return true;

    if (ctx.progress_comp) |pc| {
        var buf: [48]u8 = undefined;
        const key = std.fmt.bufPrint(&buf, "lp_state:{d}", .{lp.LevelPlayId}) catch "";
        if (key.len > 0 and pc.info.getKeyInt(key) == 2) return true;
    }
    return isQuestCheckingLevelPlay(ctx, lp.LevelPlayId);
}

fn isLpChildQuestSatisfiedNow(
    ctx: LevelPlayContext,
    lp_id: i32,
    node: LazyNodeStore.NodeEntry,
    player_pos_opt: ?[3]f32,
) bool {
    const cond = QuestEngine.getChildConditionPub(node.data) orelse return true;
    const ctype = jsonStr(cond.get("Type")) orelse return true;
    if (std.mem.eql(u8, ctype, "ReachArea")) {
        const pos = player_pos_opt orelse currentPlayerPos(ctx) orelse return false;
        return isPlayerWithinReachArea(ctx, node.data, cond, pos);
    }
    return QuestEngine.evalChildConditionAuto(toQuestCtx(ctx), lp_id, node);
}

fn pushLevelPlayBehaviorTreeInfo(
    ctx: LevelPlayContext,
    lp_id: i32,
    nodes: []const LazyNodeStore.NodeEntry,
) !void {
    var tree_nodes: std.ArrayList(pb.MapEntry(i32, pb.NodeInfo)) = .empty;
    var timer_infos: std.ArrayList(pb.TimerInfoPb) = .empty;
    for (nodes) |node| {
        const finished = isLevelPlayNodeMarked(ctx.progress_comp, lp_id, node.id);
        const active = !finished and isLpNodeActive(ctx.progress_comp, lp_id, node.id);
        if (!finished and !active) continue;

        const status: pb.NodeStatus = if (finished) .CompletedSuccess else .Activate;
        var node_info: pb.NodeInfo = .{ .Status = status };
        if (std.mem.eql(u8, node.node_type, "ChildQuest")) {
            node_info.ExtraInfo = .{ .ChildQuestNodeInfo = .{
                .Status = if (finished) .Finished else .Progress,
            } };
            if (active) {
                if (ctx.scene.lp_timers.getDueMs(lp_id, node.id)) |due_ms| {
                    try timer_infos.append(ctx.alloc.arena, .{
                        .TimerType = "WaitTime",
                        .NodeId = node.id,
                        .EndTime = @divTrunc(due_ms, 1000),
                    });
                }
            }
        }
        try tree_nodes.append(ctx.alloc.arena, .{
            .key = node.id,
            .value = node_info,
        });
    }

    var tree_infos: std.ArrayList(pb.TreeInfo) = .empty;
    try tree_infos.append(ctx.alloc.arena, .{
        .TreeOwnerId = lp_id,
        .TreeIncId = lp_id,
        .BtType = .BtTypeLevelPlay,
        .BlackboardId = lp_id,
        .Nodes = tree_nodes,
        .TimerInfos = timer_infos,
    });
    try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.BehaviorTreeInfoNotify{
        .TreeInfos = tree_infos,
    });
    try ctx.scene.sent_lp_bt_trees.put(ctx.alloc.gpa, lp_id, {});
}

fn finishLpNonChildNode(
    ctx: LevelPlayContext,
    lp_id: i32,
    node: LazyNodeStore.NodeEntry,
    push_updates: bool,
    dirty_pc: *bool,
) void {
    ctx.scene.releaseOccupationsForNode(ctx.alloc.gpa, lp_id, node.id);
    markLpNodeFinished(ctx, lp_id, node.id, dirty_pc);
    if (push_updates) {
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.UpdateNodeStatusNotify{
            .TreeOwnerId = lp_id,
            .TreeIncId = lp_id,
            .NodeId = node.id,
            .Status = .CompletedSuccess,
        }) catch {};
    }
}

fn clearLpNodeFinished(ctx: LevelPlayContext, lp_id: i32, node_id: i32, dirty_pc: *bool) void {
    setLpNodeActive(ctx, lp_id, node_id, false, dirty_pc);
    const pc = ctx.progress_comp orelse return;
    var buf: [48]u8 = undefined;
    const key = lpKeyBuf(&buf, "lp_node", lp_id, node_id);
    if (key.len == 0) return;
    if (pc.info.removeClaim(ctx.alloc.gpa, key)) {
        dirty_pc.* = true;
    }
    if (pc.info.getKeyInt(key) != 0) {
        pc.info.setKeyInt(ctx.alloc.gpa, key, 0) catch {};
        dirty_pc.* = true;
    }
}

fn resetLpSubtreeFinishedNodes(
    ctx: LevelPlayContext,
    lp_id: i32,
    nodes: []const LazyNodeStore.NodeEntry,
    parent_id: i32,
    dirty_pc: *bool,
) void {
    for (nodes) |child| {
        if (child.parent_id != parent_id) continue;
        clearLpNodeFinished(ctx, lp_id, child.id, dirty_pc);
        resetLpSubtreeFinishedNodes(ctx, lp_id, nodes, child.id, dirty_pc);
    }
}

fn deactivateLpSubtree(
    ctx: LevelPlayContext,
    lp_id: i32,
    nodes: []const LazyNodeStore.NodeEntry,
    parent_id: i32,
    push_updates: bool,
    dirty_pc: *bool,
) void {
    const qctx = toQuestCtx(ctx);
    for (nodes) |child| {
        if (child.parent_id != parent_id) continue;
        ctx.scene.lp_timers.cancel(lp_id, child.id);
        ctx.scene.releaseOccupationsForNode(ctx.alloc.gpa, lp_id, child.id);
        if (isLpNodeActive(ctx.progress_comp, lp_id, child.id)) {
            setLpNodeActive(ctx, lp_id, child.id, false, dirty_pc);
            QuestEngine.removeDoInteractOptionsForBtNode(qctx, .BtTypeLevelPlay, lp_id, child.id);
            if (push_updates) {
                ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.UpdateNodeStatusNotify{
                    .TreeOwnerId = lp_id,
                    .TreeIncId = lp_id,
                    .NodeId = child.id,
                    .Status = .Destroy,
                }) catch {};
            }
        }
        deactivateLpSubtree(ctx, lp_id, nodes, child.id, push_updates, dirty_pc);
    }
}

fn completeLpChildQuest(
    ctx: LevelPlayContext,
    lp_id: i32,
    nodes: []const LazyNodeStore.NodeEntry,
    node: LazyNodeStore.NodeEntry,
    push_updates: bool,
    dirty_pc: *bool,
) !void {
    _ = nodes;
    ctx.scene.lp_timers.cancel(lp_id, node.id);
    if (isLevelPlayNodeMarked(ctx.progress_comp, lp_id, node.id)) return;
    markLpNodeFinished(ctx, lp_id, node.id, dirty_pc);
    if (dirty_pc.*) {
        if (ctx.progress_comp) |pc| {
            pc.save(ctx.fs, ctx.alloc.arena) catch {};
            dirty_pc.* = false;
        }
    }

    const qctx = toQuestCtx(ctx);
    QuestEngine.removeDoInteractOptionsForBtNode(qctx, .BtTypeLevelPlay, lp_id, node.id);

    if (push_updates) {
        QuestEngine.pushBtChildNodeStatus(qctx, .BtTypeLevelPlay, lp_id, &node, .CompletedSuccess, .Finished) catch {};
    }
    if (node.data == .object) {
        runLevelPlayActions(ctx, lp_id, node.id, node.data.object.get("FinishActions"));
    }
}

fn stepLevelPlayNode(
    ctx: LevelPlayContext,
    lp_id: i32,
    nodes: []const LazyNodeStore.NodeEntry,
    node: LazyNodeStore.NodeEntry,
    initial_boot: bool,
    push_updates: bool,
    player_pos_opt: ?[3]f32,
    newly_activated: *std.ArrayList(LazyNodeStore.NodeEntry),
    newly_actions: *std.ArrayList(LazyNodeStore.NodeEntry),
    dirty_pc: *bool,
    steps: *usize,
) anyerror!LpStepResult {
    if (steps.* >= 96) return .running;
    steps.* += 1;

    if (QuestInfo.containsInt(ctx.quest_comp.info.completed_level_plays, lp_id)) return .tree_ended;
    if (isLevelPlayNodeMarked(ctx.progress_comp, lp_id, node.id)) return .completed;

    const qctx = toQuestCtx(ctx);
    if (try QuestEngine.checkNodeOccupationAndSavePub(qctx, .BtTypeLevelPlay, lp_id, node, push_updates and !initial_boot)) {
        return .running;
    }

    if (std.mem.eql(u8, node.node_type, "Sequence")) {
        for (nodes) |child| {
            if (child.parent_id != node.id) continue;
            const res = try stepLevelPlayNode(ctx, lp_id, nodes, child, initial_boot, push_updates, player_pos_opt, newly_activated, newly_actions, dirty_pc, steps);
            switch (res) {
                .running => return .running,
                .failed => return .failed,
                .tree_ended => return .tree_ended,
                .completed => continue,
            }
        }
        finishLpNonChildNode(ctx, lp_id, node, push_updates and !initial_boot, dirty_pc);
        return .completed;
    } else if (std.mem.eql(u8, node.node_type, "Select")) {
        var any_child = false;
        for (nodes) |child| {
            if (child.parent_id != node.id) continue;
            any_child = true;
            const res = try stepLevelPlayNode(ctx, lp_id, nodes, child, initial_boot, push_updates, player_pos_opt, newly_activated, newly_actions, dirty_pc, steps);
            switch (res) {
                .tree_ended => return .tree_ended,
                .completed => {
                    finishLpNonChildNode(ctx, lp_id, node, push_updates and !initial_boot, dirty_pc);
                    return .completed;
                },
                .running => return .running,
                .failed => continue,
            }
        }
        if (any_child and node.parent_id != 0) return .failed;
        finishLpNonChildNode(ctx, lp_id, node, push_updates and !initial_boot, dirty_pc);
        return .completed;
    } else if (std.mem.eql(u8, node.node_type, "ParallelSelect") or std.mem.eql(u8, node.node_type, "Parallel")) {
        const is_parallel_all = std.mem.eql(u8, node.node_type, "Parallel");
        const needed_sel: i32 = @max(1, node.count);
        var finished_count: i32 = 0;
        var failed_count: i32 = 0;
        var total_children: i32 = 0;

        for (nodes) |child| {
            if (child.parent_id != node.id) continue;
            total_children += 1;
            if (isLevelPlayNodeMarked(ctx.progress_comp, lp_id, child.id)) {
                finished_count += 1;
                continue;
            }
            const res = try stepLevelPlayNode(ctx, lp_id, nodes, child, initial_boot, push_updates, player_pos_opt, newly_activated, newly_actions, dirty_pc, steps);
            switch (res) {
                .tree_ended => return .tree_ended,
                .completed => {
                    finished_count += 1;
                    if (!is_parallel_all and finished_count >= needed_sel) break;
                },
                .failed => failed_count += 1,
                .running => {},
            }
        }
        const needed: i32 = if (is_parallel_all) total_children else needed_sel;
        if (is_parallel_all and failed_count > 0) return .failed;
        if (finished_count >= needed) {
            if (!is_parallel_all) {
                deactivateLpSubtree(ctx, lp_id, nodes, node.id, push_updates and !initial_boot, dirty_pc);
            }
            finishLpNonChildNode(ctx, lp_id, node, push_updates and !initial_boot, dirty_pc);
            return .completed;
        }
        if (!is_parallel_all and node.parent_id != 0 and total_children > 0 and (total_children - failed_count) < needed) {
            return .failed;
        }
        return .running;
    } else if (std.mem.eql(u8, node.node_type, "AlwaysPending")) {
        for (nodes) |child| {
            if (child.parent_id != node.id) continue;
            const res = try stepLevelPlayNode(ctx, lp_id, nodes, child, initial_boot, push_updates, player_pos_opt, newly_activated, newly_actions, dirty_pc, steps);
            if (res == .tree_ended) return .tree_ended;
        }
        return .running;
    } else if (std.mem.eql(u8, node.node_type, "AlwaysFalse")) {
        for (nodes) |child| {
            if (child.parent_id != node.id) continue;
            const res = try stepLevelPlayNode(ctx, lp_id, nodes, child, initial_boot, push_updates, player_pos_opt, newly_activated, newly_actions, dirty_pc, steps);
            if (res == .tree_ended) return .tree_ended;
        }
        return .running;
    } else if (std.mem.eql(u8, node.node_type, "AlwaysTrue")) {
        for (nodes) |child| {
            if (child.parent_id != node.id) continue;
            const res = try stepLevelPlayNode(ctx, lp_id, nodes, child, initial_boot, push_updates, player_pos_opt, newly_activated, newly_actions, dirty_pc, steps);
            if (res == .tree_ended) return .tree_ended;
            if (res == .running) return .running;
        }
        finishLpNonChildNode(ctx, lp_id, node, push_updates and !initial_boot, dirty_pc);
        return .completed;
    } else if (std.mem.eql(u8, node.node_type, "QuestSucceed")) {
        markLpNodeFinished(ctx, lp_id, node.id, dirty_pc);
        if (dirty_pc.*) {
            if (ctx.progress_comp) |pc| {
                pc.save(ctx.fs, ctx.alloc.arena) catch {};
                dirty_pc.* = false;
            }
        }
        if (!initial_boot) {
            try completeLevelPlay(ctx, lp_id);
        }
        return .tree_ended;
    } else if (std.mem.eql(u8, node.node_type, "QuestFailed")) {
        if (try QuestEngine.evalQuestFailedConditionPub(qctx, lp_id, node, player_pos_opt, &ctx.scene.lp_fail_timers)) |reason| {
            if (node.data == .object) {
                runLevelPlayActions(ctx, lp_id, node.id, node.data.object.get("FinishActions"));
            }
            try onBtRollback(ctx, lp_id, node.id, reason);
            return .tree_ended;
        }
        return .running;
    } else if (std.mem.eql(u8, node.node_type, "Action") or std.mem.eql(u8, node.node_type, "ActionWithResult")) {
        if (node.data == .object) {
            var single_buf: [1]std.json.Value = undefined;
            const action_items: ?[]const std.json.Value = if (node.data.object.get("Actions")) |acts_val|
                (if (acts_val == .array) acts_val.array.items else null)
            else if (node.data.object.get("Action")) |single_act| blk: {
                if (single_act != .object) break :blk null;
                single_buf[0] = single_act;
                break :blk single_buf[0..1];
            } else null;
            if (action_items) |items| {
                if (items.len > 0) {
                    if (initial_boot) {
                        try newly_actions.append(ctx.alloc.arena, node);
                    } else if (push_updates) {
                        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.UpdateNodeStatusNotify{
                            .TreeOwnerId = lp_id,
                            .TreeIncId = lp_id,
                            .NodeId = node.id,
                            .Status = .Activate,
                        }) catch {};
                    }
                    var lp_cbs = QuestEngine.actionCallbacks();
                    lp_cbs.bt_type = .BtTypeLevelPlay;
                    lp_cbs.deferred_bt_vars = lp_deferred_bt_vars;
                    QuestEngine.executeActionList(qctx, lp_cbs, lp_id, node.id, items) catch {};
                    if (!initial_boot and push_updates) {
                        QuestEngine.pushBtLevelEventsForType(qctx, .BtTypeLevelPlay, lp_id, node.id, .ActionGroupNodeAction, items) catch {};
                    }
                    const is_async_flow = QuestEngine.hasAsyncPlayFlowAction(items);
                    if (!is_async_flow and
                        ((ctx.scene.active_flow_inc_id != 0 and
                        ctx.scene.active_flow_quest_id == lp_id and
                        ctx.scene.active_flow_node_id == node.id) or
                        ctx.scene.hasPendingFlowForNode(lp_id, node.id)))
                    {
                        setLpNodeActive(ctx, lp_id, node.id, true, dirty_pc);
                        return .running;
                    }
                }
            }
        }
        finishLpNonChildNode(ctx, lp_id, node, push_updates and !initial_boot, dirty_pc);
        return .completed;
    } else if (std.mem.eql(u8, node.node_type, "Condition")) {
        const wait_true = if (node.data == .object)
            if (node.data.object.get("WaitUntilTrue")) |w| w == .bool and w.bool else false
        else
            false;
        const ok = if (node.data == .object)
            if (node.data.object.get("Condition")) |cb|
                QuestEngine.evalInlineCondition(qctx, lp_id, cb)
            else
                true
        else
            true;
        if (ok) {
            finishLpNonChildNode(ctx, lp_id, node, push_updates and !initial_boot, dirty_pc);
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
                            QuestEngine.evalInlineCondition(qctx, lp_id, sc)
                        else
                            true;
                        if (!pass) continue;
                        const sn = slot.object.get("Node") orelse continue;
                        if (sn != .object) continue;
                        const cid = jsonInt(sn.object.get("Id")) orelse continue;
                        const cnode = findLpNodeById(nodes, cid) orelse continue;
                        const res = try stepLevelPlayNode(ctx, lp_id, nodes, cnode.*, initial_boot, push_updates, player_pos_opt, newly_activated, newly_actions, dirty_pc, steps);
                        switch (res) {
                            .completed => {
                                finishLpNonChildNode(ctx, lp_id, node, push_updates and !initial_boot, dirty_pc);
                                return .completed;
                            },
                            .running, .tree_ended => return res,
                            .failed => continue,
                        }
                    }
                }
            }
        }
        return .running;
    } else if (std.mem.eql(u8, node.node_type, "Repeater")) {
        const cd_key = quest_types.repeaterCooldownKey(lp_id, node.id);
        while (steps.* < 96) {
            if (node.data == .object) {
                if (node.data.object.get("ExitCondition")) |ec| {
                    if (QuestEngine.evalInlineCondition(qctx, lp_id, ec)) {
                        deactivateLpSubtree(ctx, lp_id, nodes, node.id, push_updates and !initial_boot, dirty_pc);
                        finishLpNonChildNode(ctx, lp_id, node, push_updates and !initial_boot, dirty_pc);
                        return .completed;
                    }
                }
            }
            if (ctx.scene.lp_repeater_cooldowns.get(cd_key)) |ready_ms| {
                if (QuestEngine.nowMsPub(ctx.io) < ready_ms) return .running;
            }
            var any_child = false;
            var child_completed = false;
            for (nodes) |child| {
                if (child.parent_id != node.id) continue;
                any_child = true;
                const res = try stepLevelPlayNode(ctx, lp_id, nodes, child, initial_boot, push_updates, player_pos_opt, newly_activated, newly_actions, dirty_pc, steps);
                if (res == .tree_ended) return .tree_ended;
                if (res == .completed) {
                    child_completed = true;
                    break;
                }
                return .running;
            }
            if (!any_child) {
                finishLpNonChildNode(ctx, lp_id, node, push_updates and !initial_boot, dirty_pc);
                return .completed;
            }
            if (!child_completed) return .running;
            resetLpSubtreeFinishedNodes(ctx, lp_id, nodes, node.id, dirty_pc);
            const wait_ms = quest_types.subtreeWaitDelayMs(nodes, node.id);
            if (wait_ms > 0) {
                ctx.scene.lp_repeater_cooldowns.put(ctx.alloc.gpa, cd_key, QuestEngine.nowMsPub(ctx.io) + wait_ms) catch {};
                return .running;
            }
        }
        return .running;
    } else if (std.mem.eql(u8, node.node_type, "ChildQuest")) {
        const was_active = isLpNodeActive(ctx.progress_comp, lp_id, node.id);
        if (!was_active) {
            if (node.data == .object) {
                runLevelPlayActions(ctx, lp_id, node.id, node.data.object.get("EnterActions"));
            }
            if (QuestEngine.getChildConditionPub(node.data)) |cond| {
                if (QuestEngine.parseTimerDelayMsPub(cond)) |delay_ms| {
                    if (delay_ms > 0 and !ctx.scene.lp_timers.contains(lp_id, node.id)) {
                        try ctx.scene.lp_timers.upsert(ctx.alloc.gpa, .{
                            .quest_id = lp_id,
                            .node_id = node.id,
                            .due_ms = QuestEngine.nowMsPub(ctx.io) + delay_ms,
                        });
                    }
                }
            }
        }
        if (isLpChildQuestSatisfiedNow(ctx, lp_id, node, player_pos_opt)) {
            if (!was_active and !initial_boot and push_updates) {
                QuestEngine.pushBtChildNodeStatus(qctx, .BtTypeLevelPlay, lp_id, &node, .Activate, .Progress) catch {};
            }
            try completeLpChildQuest(ctx, lp_id, nodes, node, push_updates and !initial_boot, dirty_pc);
            return .completed;
        }
        if (!was_active) {
            setLpNodeActive(ctx, lp_id, node.id, true, dirty_pc);
            if (initial_boot) {
                try newly_activated.append(ctx.alloc.arena, node);
            } else if (push_updates) {
                QuestEngine.pushBtChildNodeStatus(qctx, .BtTypeLevelPlay, lp_id, &node, .Activate, .Progress) catch {};
                QuestEngine.pushDoInteractOptionsForBtNode(ctx.scene, ctx.conn, ctx.alloc, ctx.io, .BtTypeLevelPlay, lp_id, node, null) catch {};
            }
        }
        return .running;
    }

    finishLpNonChildNode(ctx, lp_id, node, push_updates and !initial_boot, dirty_pc);
    return .completed;
}

fn stepLevelPlayTree(
    ctx: LevelPlayContext,
    lp_id: i32,
    nodes: []const LazyNodeStore.NodeEntry,
    push_updates: bool,
    player_pos_opt: ?[3]f32,
) !void {
    if (nodes.len == 0) return;
    if (QuestInfo.containsInt(ctx.quest_comp.info.completed_level_plays, lp_id)) return;
    if (push_updates and !ctx.scene.sent_lp_bt_trees.contains(lp_id)) {
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.LevelPlayStateNotify{
            .Id = lp_id,
            .State = 2,
            .CompleteNumber = 0,
        }) catch {};
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.EnterLevelPlayNotify{
            .Id = lp_id,
            .State = 2,
            .CanGetReward = true,
        }) catch {};
        try pushLevelPlayBehaviorTreeInfo(ctx, lp_id, nodes);
    }
    if (lp_step_depth > 0) {
        lp_recheck_pending = true;
        return;
    }

    lp_step_depth += 1;
    defer {
        lp_step_depth -= 1;
        if (lp_step_depth == 0) lp_recheck_pending = false;
    }

    const root = findLpRootNode(nodes) orelse return;
    var dirty_pc = false;
    var pass: usize = 0;
    while (pass < 4) : (pass += 1) {
        lp_recheck_pending = false;
        var newly_activated: std.ArrayList(LazyNodeStore.NodeEntry) = .empty;
        var newly_actions: std.ArrayList(LazyNodeStore.NodeEntry) = .empty;
        var steps: usize = 0;
        const res = try stepLevelPlayNode(
            ctx,
            lp_id,
            nodes,
            root,
            false,
            push_updates,
            player_pos_opt,
            &newly_activated,
            &newly_actions,
            &dirty_pc,
            &steps,
        );
        if (dirty_pc) {
            if (ctx.progress_comp) |pc| {
                pc.save(ctx.fs, ctx.alloc.arena) catch {};
                dirty_pc = false;
            }
        }
        if (res == .completed and !QuestInfo.containsInt(ctx.quest_comp.info.completed_level_plays, lp_id)) {
            try completeLevelPlay(ctx, lp_id);
            break;
        }
        if (!lp_recheck_pending or res == .tree_ended) break;
    }
}

fn enterAndStartLevelPlay(
    ctx: LevelPlayContext,
    lp: Assets.DataTables.LevelPlayData,
    player_pos_opt: ?[3]f32,
    dirty_entered: *bool,
) !void {
    const lp_id = lp.LevelPlayId;
    if (QuestInfo.containsInt(ctx.quest_comp.info.completed_level_plays, lp_id)) return;
    if (QuestInfo.containsInt(ctx.quest_comp.info.progress.entered_level_plays, lp_id)) return;

    if (try QuestInfo.appendUniqueInt(&ctx.quest_comp.info.progress.entered_level_plays, ctx.alloc.gpa, lp_id)) {
        dirty_entered.* = true;
    }
    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.LevelPlayStateNotify{
        .Id = lp_id,
        .State = 2,
        .CompleteNumber = 0,
    }) catch {};
    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.EnterLevelPlayNotify{
        .Id = lp_id,
        .State = 2,
        .CanGetReward = true,
    }) catch {};

    const nodes = ctx.assets.level_play_nodes.getOwnerNodes(ctx.io, lp_id) catch &[_]LazyNodeStore.NodeEntry{};
    if (nodes.len == 0) return;

    const qctx = toQuestCtx(ctx);
    const root = findLpRootNode(nodes) orelse nodes[0];
    var dirty_pc = false;

    if (!isLpBtBooted(ctx.progress_comp, lp_id)) {
        markLpBtBooted(ctx, lp_id, &dirty_pc);
        var newly_activated: std.ArrayList(LazyNodeStore.NodeEntry) = .empty;
        var newly_actions: std.ArrayList(LazyNodeStore.NodeEntry) = .empty;
        var deferred_vars: std.ArrayList(QuestEngine.DeferredBtVar) = .empty;
        var steps: usize = 0;

        lp_step_depth += 1;
        const prev_deferred = lp_deferred_bt_vars;
        lp_deferred_bt_vars = &deferred_vars;
        const res = stepLevelPlayNode(
            ctx,
            lp_id,
            nodes,
            root,
            true,
            false,
            player_pos_opt,
            &newly_activated,
            &newly_actions,
            &dirty_pc,
            &steps,
        ) catch |err| {
            lp_deferred_bt_vars = prev_deferred;
            lp_step_depth -= 1;
            return err;
        };
        lp_deferred_bt_vars = prev_deferred;
        lp_step_depth -= 1;

        if (dirty_pc) {
            if (ctx.progress_comp) |pc| {
                pc.save(ctx.fs, ctx.alloc.arena) catch {};
                dirty_pc = false;
            }
        }

        // Always create the client BehaviorTree BEFORE any BtVarUpdateNotify, ActionNotify, or LeaveLevelPlayNotify.
        try pushLevelPlayBehaviorTreeInfo(ctx, lp_id, nodes);

        for (deferred_vars.items) |dv| {
            ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.BtVarUpdateNotify{
                .TreeOwnerId = dv.owner_id,
                .TreeIncId = dv.owner_id,
                .VarName = dv.var_name,
                .VarDefine = dv.var_define,
            }) catch {};
        }

        for (newly_activated.items) |act_node| {
            QuestEngine.pushBtLevelEventsForType(
                qctx,
                .BtTypeLevelPlay,
                lp_id,
                act_node.id,
                .ChildQuestNodeEnterAction,
                QuestEngine.nodeActionItemsPub(&act_node, "EnterActions"),
            ) catch {};
            QuestEngine.pushDoInteractOptionsForBtNode(ctx.scene, ctx.conn, ctx.alloc, ctx.io, .BtTypeLevelPlay, lp_id, act_node, null) catch {};
        }
        for (newly_actions.items) |act_node| {
            var single_buf: [1]std.json.Value = undefined;
            const action_items: ?[]const std.json.Value = if (act_node.data == .object) blk: {
                if (act_node.data.object.get("Actions")) |av| {
                    break :blk if (av == .array) av.array.items else null;
                }
                if (act_node.data.object.get("Action")) |sv| {
                    if (sv != .object) break :blk null;
                    single_buf[0] = sv;
                    break :blk single_buf[0..1];
                }
                break :blk null;
            } else null;
            if (action_items) |items| {
                QuestEngine.pushBtLevelEventsForType(qctx, .BtTypeLevelPlay, lp_id, act_node.id, .ActionGroupNodeAction, items) catch {};
            }
        }

        if (res == .tree_ended) {
            try completeLevelPlay(ctx, lp_id);
        } else if (res == .completed or lp_recheck_pending) {
            lp_recheck_pending = false;
            try stepLevelPlayTree(ctx, lp_id, nodes, true, player_pos_opt);
        }
    } else {
        try pushLevelPlayBehaviorTreeInfo(ctx, lp_id, nodes);
        for (nodes) |node| {
            if (isLpNodeActive(ctx.progress_comp, lp_id, node.id)) {
                QuestEngine.pushDoInteractOptionsForBtNode(ctx.scene, ctx.conn, ctx.alloc, ctx.io, .BtTypeLevelPlay, lp_id, node, null) catch {};
            }
        }
        try stepLevelPlayTree(ctx, lp_id, nodes, true, player_pos_opt);
    }
}

fn runLevelPlayActions(ctx: LevelPlayContext, owner_id: i32, node_id: i32, actions_opt: ?std.json.Value) void {
    const actions_val = actions_opt orelse return;
    if (actions_val != .array) return;

    var lp_cbs = QuestEngine.actionCallbacks();
    lp_cbs.bt_type = .BtTypeLevelPlay;
    lp_cbs.deferred_bt_vars = lp_deferred_bt_vars;
    QuestEngine.executeActionList(
        toQuestCtx(ctx),
        lp_cbs,
        owner_id,
        node_id,
        actions_val.array.items,
    ) catch {};

    for (actions_val.array.items) |act_val| {
        if (act_val != .object) continue;
        const name = jsonStr(act_val.object.get("Name")) orelse continue;
        const params_val = act_val.object.get("Params") orelse continue;
        if (params_val != .object) continue;
        const p = params_val.object;

        if (std.mem.eql(u8, name, "AwakeEntity") or std.mem.eql(u8, name, "UnlockEntity")) {
            if (jsonInt(p.get("EntityId"))) |eid| ensureEntitySpawned(ctx, eid);
            if (p.get("EntityIds")) |eids| {
                if (eids == .array) {
                    for (eids.array.items) |v| {
                        if (jsonInt(v)) |eid| ensureEntitySpawned(ctx, eid);
                    }
                }
            }
        }
    }
}

pub fn completeLevelPlay(ctx: LevelPlayContext, level_play_id: i32) !void {
    if (level_play_id <= 0) return;
    ctx.scene.lp_timers.cancelQuest(level_play_id);
    ctx.scene.fail_timers.cancelQuest(level_play_id);
    ctx.scene.lp_fail_timers.cancelQuest(level_play_id);
    ctx.scene.releaseOccupationsForTree(ctx.alloc.gpa, level_play_id, level_play_id);
    ctx.scene.clearSaveNodesForOwner(level_play_id);
    if (QuestInfo.containsInt(ctx.quest_comp.info.completed_level_plays, level_play_id)) return;

    const qctx = toQuestCtx(ctx);
    const lp_nodes = ctx.assets.level_play_nodes.getOwnerNodes(ctx.io, level_play_id) catch &[_]LazyNodeStore.NodeEntry{};

    for (lp_nodes) |node| {
        if (isLpNodeActive(ctx.progress_comp, level_play_id, node.id)) {
            if (ctx.progress_comp) |pc| {
                var dirty = false;
                setLpNodeActive(ctx, level_play_id, node.id, false, &dirty);
                if (dirty) pc.save(ctx.fs, ctx.alloc.arena) catch {};
            }
        }
        QuestEngine.removeDoInteractOptionsForBtNode(qctx, .BtTypeLevelPlay, level_play_id, node.id);

        if (!std.mem.eql(u8, node.node_type, "QuestSucceed")) continue;
        if (isLpBtBooted(ctx.progress_comp, level_play_id)) {
            ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.UpdateNodeStatusNotify{
                .TreeOwnerId = level_play_id,
                .TreeIncId = level_play_id,
                .NodeId = node.id,
                .Status = .CompletedSuccess,
            }) catch {};
        }
        if (node.data == .object) {
            if (node.data.object.get("FinishActions")) |acts_val| {
                if (acts_val == .array and acts_val.array.items.len > 0) {
                    if (isLpBtBooted(ctx.progress_comp, level_play_id)) {
                        QuestEngine.pushBtLevelEventsForType(
                            qctx,
                            .BtTypeLevelPlay,
                            level_play_id,
                            node.id,
                            .SuccessNodeAction,
                            acts_val.array.items,
                        ) catch {};
                    }
                    QuestEngine.executeActionList(qctx, QuestEngine.actionCallbacks(), level_play_id, node.id, acts_val.array.items) catch {};
                }
            }
        }
    }

    const was_entered = try QuestInfo.removeIntValue(&ctx.quest_comp.info.progress.entered_level_plays, ctx.alloc.gpa, level_play_id);
    try ctx.quest_comp.info.markLevelPlayCompleted(ctx.alloc.gpa, level_play_id);
    try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);

    if (ctx.progress_comp) |pc| {
        var kbuf: [48]u8 = undefined;
        const k = std.fmt.bufPrint(&kbuf, "lp_state:{d}", .{level_play_id}) catch "";
        if (k.len > 0) {
            pc.info.setKeyInt(ctx.alloc.gpa, k, 3) catch {};
            pc.save(ctx.fs, ctx.alloc.arena) catch {};
        }
    }

    if (was_entered) {
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.LevelPlayFirstNotify{
            .Id = level_play_id,
        });
    }
    try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.LevelPlayStateNotify{
        .Id = level_play_id,
        .State = 3,
        .CompleteNumber = 1,
    });
    if (was_entered) {
        _ = ctx.scene.sent_lp_bt_trees.remove(level_play_id);
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.LeaveLevelPlayNotify{
            .Id = level_play_id,
        });
    }

    var infos: std.ArrayList(pb.LevelPlayInfo) = .empty;
    try infos.append(ctx.alloc.arena, .{
        .Id = level_play_id,
        .IsFirst = true,
        .State = 3,
    });
    try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.LevelPlayInfoNotify{
        .LevelPlayInfo = infos,
    });

    try QuestEngine.onLevelPlayCompleted(qctx, level_play_id);
    if (ctx.scene.instance_id != 8) {
        try QuestEngine.onDungeonEvent(qctx, ctx.scene.instance_id, true);
    }
    log.info("Completed LevelPlay {d}", .{level_play_id});
}

fn removeEntityFromSceneAndNotify(ctx: LevelPlayContext, net_id: i64) void {
    var remove_infos: std.ArrayList(pb.EntityRemoveInfo) = .empty;
    remove_infos.append(ctx.alloc.arena, .{ .EntityId = net_id }) catch {};
    ctx.scene.remove(ctx.alloc.gpa, ctx.fs, net_id) catch {};
    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.EntityRemoveNotify{
        .IsRemove = true,
        .RemoveInfos = remove_infos,
    }) catch {};
}

fn checkLevelPlayKillProgress(ctx: LevelPlayContext, killed_cfg_id: i32, killed_net_id: i64) !void {
    if (killed_cfg_id <= 0) return;

    var candidate_lp_indices: std.ArrayList(usize) = .empty;
    for (ctx.assets.indexes.levelPlayIndicesForRefEntity(killed_cfg_id)) |lp_idx| {
        try candidate_lp_indices.append(ctx.alloc.arena, lp_idx);
    }
    if (ctx.scene.instance_id != 8) {
        for (ctx.assets.tables.level_play_data.items, 0..) |lp, lp_idx| {
            if ((lp.Data.InstanceId != 0 and lp.Data.InstanceId == ctx.scene.instance_id) or
                (lp.Data.LevelId != 0 and lp.Data.LevelId == ctx.scene.instance_id))
            {
                if (std.mem.indexOfScalar(usize, candidate_lp_indices.items, lp_idx) == null) {
                    try candidate_lp_indices.append(ctx.alloc.arena, lp_idx);
                }
            }
        }
    }

    for (candidate_lp_indices.items) |lp_idx| {
        const lp = ctx.assets.tables.level_play_data.items[lp_idx];
        if (!matchesSceneInstance(ctx.assets, lp, ctx.scene.instance_id)) continue;
        if (QuestInfo.containsInt(ctx.quest_comp.info.completed_level_plays, lp.LevelPlayId)) continue;

        const nodes = ctx.assets.level_play_nodes.getOwnerNodes(ctx.io, lp.LevelPlayId) catch continue;
        for (nodes) |node| {
            if (node.data != .object) continue;
            const ntype = jsonStr(node.data.object.get("Type")) orelse continue;
            if (!std.mem.eql(u8, ntype, "ChildQuest")) continue;
            const cond_val = node.data.object.get("Condition") orelse continue;
            if (cond_val != .object) continue;
            const ctype = jsonStr(cond_val.object.get("Type")) orelse continue;
            if (!std.mem.eql(u8, ctype, "Kill") and !std.mem.eql(u8, ctype, "MonsterCreator")) continue;

            const in_exist = jsonArrayContainsInt(cond_val.object.get("ExistTargets"), killed_cfg_id);
            const in_awake = jsonArrayContainsInt(cond_val.object.get("TargetsToAwake"), killed_cfg_id);
            const in_mc = jsonArrayContainsInt(cond_val.object.get("MonsterCreatorEntityIds"), killed_cfg_id);
            if (!in_exist and !in_awake and !in_mc) continue;

            // Check whether all targets in this Kill/MonsterCreator node are now defeated.
            var any_alive = false;
            inline for (.{ "ExistTargets", "TargetsToAwake", "MonsterCreatorEntityIds" }) |field| {
                if (!any_alive) {
                    if (cond_val.object.get(field)) |et| {
                        if (et == .array) {
                            for (et.array.items) |item| {
                                const tid = jsonInt(item) orelse continue;
                                if (isTargetStillAliveInScene(ctx.scene, tid, killed_net_id)) {
                                    any_alive = true;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            if (any_alive) break;

            // Kill objective cleared!
            var dirty_pc = false;
            try completeLpChildQuest(ctx, lp.LevelPlayId, nodes, node, true, &dirty_pc);
            if (dirty_pc) {
                if (ctx.progress_comp) |pc| pc.save(ctx.fs, ctx.alloc.arena) catch {};
            }

            const has_reward_chest = if (lp.Data.LevelPlayRewardConfig) |rc|
                rc.RewardEntityId > 0 and !ctx.quest_comp.info.hasOpenedChest(rc.RewardEntityId)
            else
                false;

            if (lp.Data.LevelPlayRewardConfig) |rc| {
                if (rc.RewardEntityId > 0) {
                    ensureEntitySpawned(ctx, rc.RewardEntityId);
                }
            }

            if (!has_reward_chest) {
                if (lp.Data.LevelPlayRewardConfig) |rc| {
                    if (rc.RewardId > 0) {
                        _ = try RewardService.grantTreasureBoxReward(toRewardCtx(ctx), 2, rc.RewardId, 7000);
                    }
                }
                if (isLpBtBooted(ctx.progress_comp, lp.LevelPlayId)) {
                    try stepLevelPlayTree(ctx, lp.LevelPlayId, nodes, true, null);
                } else {
                    try completeLevelPlay(ctx, lp.LevelPlayId);
                }
            }
            break;
        }
    }
}

const normal_monster_respawn_ms: i64 = 60_000;
const boss_monster_respawn_ms: i64 = 180_000;
const collectible_respawn_ms: i64 = 300_000;
const animal_respawn_ms: i64 = 60_000;

fn recordWorldRespawnCooldown(ctx: LevelPlayContext, cfg_id: i32, duration_ms: i64) !void {
    if (cfg_id <= 0 or duration_ms <= 0) return;
    const now_ms = std.Io.Clock.awake.now(ctx.io).toMilliseconds();
    try ctx.scene.instance.setRespawnCooldown(ctx.alloc.gpa, cfg_id, now_ms, now_ms + duration_ms);
    try ctx.scene.savePersistent(ctx.fs, ctx.alloc.gpa);
}

fn spawnVisionCapturePhantom(
    ctx: LevelPlayContext,
    killed_net_id: i64,
    killed_cfg_id: i32,
    blueprint_type: []const u8,
    vision_id: i32,
) void {
    if (vision_id <= 0) return;
    const tpl = ctx.assets.findTemplateConfig(blueprint_type) orelse return;
    if (tpl.Id <= 0) return;

    var pos: [3]f32 = if (ctx.scene.instance.players.len > 0) ctx.scene.instance.players[0].location else .{ 0, 0, 0 };
    var rot: [3]f32 = .{ 0, 0, 0 };
    if (ctx.scene.net_id_map.get(killed_net_id)) |idx| {
        if (ctx.scene.entities.items(.position)[idx]) |p| {
            pos = p.location;
            rot = p.rotation;
        }
    } else if (ctx.assets.findLevelEntity(ctx.scene.instance_id, killed_cfg_id)) |lcfg| {
        if (Assets.AssetIndexes.entityWorldPos(lcfg)) |wpos| pos = wpos;
        rot = Assets.AssetIndexes.entityWorldRot(lcfg);
    }

    const entity = ctx.scene.spawn(ctx.alloc.gpa, ctx.fs, .{
        Scene.Entity.ConfigComponent{
            .camp = 2,
            .config_id = 14000056,
            .config_type = .template,
            .entity_type = .scene_item,
            .state = .default,
        },
        Scene.Entity.ActorVisibleMarker{},
        Scene.Entity.VisibleMarker{},
        Scene.Entity.PositionComponent{
            .location = pos,
            .rotation = rot,
        },
    }) catch return;

    ctx.scene.entities.items(.world_item)[entity.index].monster_capture = .{
        .template_id = tpl.Id,
        .entity_id = killed_cfg_id,
        .monster_id = vision_id,
    };
    ctx.scene.saveEntity(ctx.fs, ctx.alloc.gpa, entity) catch {};

    const storage = ctx.scene.entities.get(entity.index);
    const proto_pb = storage.entityToProto(entity.net_id, ctx.alloc, ctx.assets) catch return;
    var entity_pbs: std.ArrayList(pb.EntityPb) = .empty;
    entity_pbs.append(ctx.alloc.arena, proto_pb) catch return;
    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.EntityAddNotify{ .EntityPbs = entity_pbs }) catch {};
}

pub fn onEntityKilled(ctx: LevelPlayContext, target_net_id: i64, cfg_id_opt: ?i32) !void {
    if (ctx.scene.rewarded_kills.contains(target_net_id)) return;
    try ctx.scene.rewarded_kills.put(ctx.alloc.gpa, target_net_id, {});
    errdefer _ = ctx.scene.rewarded_kills.remove(target_net_id);

    var cfg_id: i32 = cfg_id_opt orelse 0;
    if (cfg_id <= 0) {
        if (ctx.scene.net_id_map.get(target_net_id)) |idx| {
            cfg_id = ctx.scene.entities.items(.config)[idx].config_id;
        }
    }

    if (cfg_id > 0) {
        if (resolveEntityConfig(ctx.assets, ctx.scene.instance_id, cfg_id)) |resolved| {
            const reward_id = if (resolved.components.RewardComponent) |rc| (rc.RewardId orelse 0) else 0;
            const vision_id = if (resolved.components.VisionCaptureComponent) |vc| (vc.VisionCaptureId orelse 0) else 0;
            const is_monster = std.mem.indexOf(u8, resolved.blueprint_type, "Monster") != null;
            const is_boss = std.mem.indexOf(u8, resolved.blueprint_type, "Boss") != null or
                std.mem.indexOf(u8, resolved.blueprint_type, "Elite") != null;

            if (is_monster or reward_id > 0 or vision_id > 0) {
                const calabash_lvl = ConditionEvaluator.currentCalabashLevel(ctx.assets, ctx.basic_comp, ctx.echo_comp);
                _ = try RewardService.grantMonsterDropWithEchoOption(toRewardCtx(ctx), reward_id, vision_id, is_boss, calabash_lvl, false);
                if (vision_id > 0) {
                    spawnVisionCapturePhantom(ctx, target_net_id, cfg_id, resolved.blueprint_type, vision_id);
                }
            }
            try recordWorldRespawnCooldown(ctx, cfg_id, if (is_boss) boss_monster_respawn_ms else normal_monster_respawn_ms);
            try checkLevelPlayKillProgress(ctx, cfg_id, target_net_id);
            if ((is_monster or reward_id > 0 or vision_id > 0) and is_boss and ctx.scene.instance_id != 8) {
                try QuestEngine.onDungeonEvent(toQuestCtx(ctx), ctx.scene.instance_id, true);
            }
        } else {
            try checkLevelPlayKillProgress(ctx, cfg_id, target_net_id);
        }
    }
}

pub fn onGetRewardTreasureBox(ctx: LevelPlayContext, net_id: i64) !void {
    const idx = ctx.scene.net_id_map.get(net_id) orelse return;
    const cfg_id = ctx.scene.entities.items(.config)[idx].config_id;
    if (cfg_id > 0 and ctx.quest_comp.info.hasOpenedChest(cfg_id)) {
        return;
    }

    var chest_type_id: i32 = 1;
    var reward_id: i32 = 0;

    if (resolveEntityConfig(ctx.assets, ctx.scene.instance_id, cfg_id)) |resolved| {
        if (resolved.components.TreasureBoxComponent) |tb| {
            if (tb.TypeId) |tid| chest_type_id = tid;
        }
        if (resolved.components.RewardComponent) |rc| {
            if (rc.RewardId) |rid| reward_id = rid;
        }
    }

    var completed_lp_id: ?i32 = null;
    if (cfg_id > 0) {
        if (std.math.cast(i32, cfg_id)) |cfg_id32| {
            const reward_indices = ctx.assets.indexes.levelPlayIndicesForRewardEntity(cfg_id32);
            for (reward_indices) |lp_idx| {
                const lp = ctx.assets.tables.level_play_data.items[lp_idx];
                if (!matchesSceneInstance(ctx.assets, lp, ctx.scene.instance_id)) continue;
                if (lp.Data.LevelPlayRewardConfig) |rc| {
                    if (reward_id == 0 and rc.RewardId > 0) reward_id = rc.RewardId;
                }
                completed_lp_id = lp.LevelPlayId;
                break;
            }
        }
    }

    // Grant before marking chest opened so a failed grant can be retried cleanly.
    _ = try RewardService.grantTreasureBoxReward(toRewardCtx(ctx), chest_type_id, reward_id, 7000);

    if (cfg_id > 0) {
        _ = try ctx.quest_comp.info.markChestOpened(ctx.alloc.gpa, cfg_id);
        _ = try ctx.quest_comp.info.setEntityState(ctx.alloc.gpa, cfg_id, "关卡.Common.状态.激活");
        try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);
    }

    if (try ctx.scene.updateEntityStateTag(ctx.alloc.gpa, &ctx.assets.tables.gameplay_tag_remap, net_id, "关卡.Common.状态.激活")) |upd| {
        try QuestEngine.emitEntityStateTagUpdateChecked(ctx.scene, ctx.assets, ctx.conn, ctx.alloc.gpa, ctx.alloc.arena, ctx.io, net_id, upd);
    }

    if (completed_lp_id) |lp_id| {
        try completeLevelPlay(ctx, lp_id);
    }
}

pub fn onEntityInteractFallback(ctx: LevelPlayContext, net_id: i64) !bool {
    const idx = ctx.scene.net_id_map.get(net_id) orelse return false;
    const cfg_id = ctx.scene.entities.items(.config)[idx].config_id;
    if (cfg_id <= 0) return false;

    const resolved = resolveEntityConfig(ctx.assets, ctx.scene.instance_id, cfg_id) orelse return false;

    if (resolved.components.TreasureBoxComponent != null or
        std.mem.indexOf(u8, resolved.blueprint_type, "TreasureBox") != null)
    {
        try onGetRewardTreasureBox(ctx, net_id);
        return true;
    }

    const is_collectible = resolved.components.RewardComponent != null and
        (std.mem.indexOf(u8, resolved.blueprint_type, "Collect") != null or
            std.mem.indexOf(u8, resolved.blueprint_type, "Pick") != null);

    if (is_collectible) {
        const now_ms = std.Io.Clock.awake.now(ctx.io).toMilliseconds();
        if (!ctx.quest_comp.info.hasOpenedChest(cfg_id) and !ctx.scene.instance.isOnRespawnCooldown(cfg_id, now_ms)) {
            const reward_id = if (resolved.components.RewardComponent) |rc| (rc.RewardId orelse 0) else 0;
            if (reward_id > 0) {
                const res = try RewardService.grantDropPackage(toRewardCtx(ctx), reward_id, 1);
                if (res.granted_count == 0) {
                    const items = [_]RewardService.RewardItem{.{ .item_id = 2, .count = 200 }};
                    _ = try RewardService.grantItems(toRewardCtx(ctx), 0, false, &items, 1);
                }
            }
            try recordWorldRespawnCooldown(ctx, cfg_id, collectible_respawn_ms);
        }
        removeEntityFromSceneAndNotify(ctx, net_id);
        return true;
    }

    // Also advance any nearby LevelPlay waiting on DoInteract for this entity.
    const cfg_id32 = std.math.cast(i32, cfg_id) orelse return false;
    for (ctx.assets.indexes.levelPlayIndicesForRefEntity(cfg_id32)) |lp_idx| {
        const lp = ctx.assets.tables.level_play_data.items[lp_idx];
        if (!canEnterLevelPlay(ctx, lp)) continue;

        const nodes = ctx.assets.level_play_nodes.getOwnerNodes(ctx.io, lp.LevelPlayId) catch continue;
        var has_tree_active = false;
        for (nodes) |n| {
            if (isLpNodeActive(ctx.progress_comp, lp.LevelPlayId, n.id)) {
                has_tree_active = true;
                break;
            }
        }
        var matched_node: ?LazyNodeStore.NodeEntry = null;
        for (nodes) |node| {
            if (node.data != .object) continue;
            if (!std.mem.eql(u8, node.node_type, "ChildQuest")) continue;
            if (has_tree_active and !isLpNodeActive(ctx.progress_comp, lp.LevelPlayId, node.id)) continue;
            const cond_val = node.data.object.get("Condition") orelse continue;
            if (cond_val != .object) continue;
            const ctype = jsonStr(cond_val.object.get("Type")) orelse continue;
            if (!std.mem.eql(u8, ctype, "DoInteract")) continue;
            if ((jsonInt(cond_val.object.get("EntityId")) orelse 0) == cfg_id32) {
                matched_node = node;
                break;
            }
            if (cond_val.object.get("AddOptions")) |opts_val| {
                if (opts_val == .array) {
                    for (opts_val.array.items) |opt_v| {
                        if (opt_v == .object and (jsonInt(opt_v.object.get("EntityId")) orelse 0) == cfg_id32) {
                            matched_node = node;
                            break;
                        }
                    }
                    if (matched_node != null) break;
                }
            }
        }
        if (matched_node) |mnode| {
            if (isLpBtBooted(ctx.progress_comp, lp.LevelPlayId) or has_tree_active) {
                var dirty_pc = false;
                try completeLpChildQuest(ctx, lp.LevelPlayId, nodes, mnode, true, &dirty_pc);
                try stepLevelPlayTree(ctx, lp.LevelPlayId, nodes, true, null);
            } else {
                runLevelPlayActions(ctx, lp.LevelPlayId, mnode.id, mnode.data.object.get("FinishActions"));
                try completeLevelPlay(ctx, lp.LevelPlayId);
            }
            return true;
        }
    }

    return false;
}

pub fn onAnimalDrop(ctx: LevelPlayContext, net_id: i64) !void {
    const idx = ctx.scene.net_id_map.get(net_id) orelse return;
    const cfg_id = ctx.scene.entities.items(.config)[idx].config_id;
    var reward_id: i32 = 0;
    if (resolveEntityConfig(ctx.assets, ctx.scene.instance_id, cfg_id)) |resolved| {
        if (resolved.components.RewardComponent) |rc| {
            reward_id = rc.RewardId orelse 0;
        }
    }
    var res: RewardService.GrantResult = .{};
    if (reward_id > 0) {
        res = try RewardService.grantDropPackage(toRewardCtx(ctx), reward_id, 1);
    }
    if (res.granted_count == 0) {
        const items = [_]RewardService.RewardItem{
            .{ .item_id = 42300010, .count = 1 },
            .{ .item_id = 2, .count = 100 },
        };
        _ = try RewardService.grantItems(toRewardCtx(ctx), 0, false, &items, 1);
    }
    try recordWorldRespawnCooldown(ctx, cfg_id, animal_respawn_ms);
    removeEntityFromSceneAndNotify(ctx, net_id);
}

/// Removes a hunted or destructible animal entity cleanly on client `AnimalDestroyRequest`.
pub fn onAnimalDestroy(ctx: LevelPlayContext, net_id: i64) !void {
    const idx = ctx.scene.net_id_map.get(net_id) orelse return;
    const cfg_id = ctx.scene.entities.items(.config)[idx].config_id;
    try recordWorldRespawnCooldown(ctx, cfg_id, animal_respawn_ms);
    removeEntityFromSceneAndNotify(ctx, net_id);
}

pub fn ensureActiveTimersScheduled(ctx: LevelPlayContext) !void {
    const now_ms = QuestEngine.nowMsPub(ctx.io);
    for (ctx.quest_comp.info.progress.entered_level_plays) |lpid| {
        if (QuestInfo.containsInt(ctx.quest_comp.info.completed_level_plays, lpid)) continue;
        const nodes = ctx.assets.level_play_nodes.getOwnerNodes(ctx.io, lpid) catch continue;
        for (nodes) |node| {
            if (!isLpNodeActive(ctx.progress_comp, lpid, node.id)) continue;
            if (!std.mem.eql(u8, node.node_type, "ChildQuest")) continue;
            if (ctx.scene.lp_timers.contains(lpid, node.id)) continue;
            const cond = QuestEngine.getChildConditionPub(node.data) orelse continue;
            const delay_ms = QuestEngine.parseTimerDelayMsPub(cond) orelse continue;
            if (delay_ms > 0) {
                try ctx.scene.lp_timers.upsert(ctx.alloc.gpa, .{
                    .quest_id = lpid,
                    .node_id = node.id,
                    .due_ms = now_ms + delay_ms,
                });
            }
        }
    }
}

pub fn onTimerDue(ctx: LevelPlayContext, lp_id: i32, node_id: i32) !bool {
    if (lp_id <= 0 or node_id <= 0) return false;
    if (QuestInfo.containsInt(ctx.quest_comp.info.completed_level_plays, lp_id)) return false;
    if (!QuestInfo.containsInt(ctx.quest_comp.info.progress.entered_level_plays, lp_id)) return false;
    if (!isLpNodeActive(ctx.progress_comp, lp_id, node_id)) return false;
    const nodes = ctx.assets.level_play_nodes.getOwnerNodes(ctx.io, lp_id) catch return false;
    const node = findLpNodeById(nodes, node_id) orelse return false;
    var dirty_pc = false;
    try completeLpChildQuest(ctx, lp_id, nodes, node.*, true, &dirty_pc);
    try stepLevelPlayTree(ctx, lp_id, nodes, true, currentPlayerPos(ctx));
    return true;
}

pub fn onFailTimerDue(ctx: LevelPlayContext, lp_id: i32, node_id: i32) !bool {
    if (lp_id <= 0 or node_id <= 0) return false;
    if (QuestInfo.containsInt(ctx.quest_comp.info.completed_level_plays, lp_id)) return false;
    if (!QuestInfo.containsInt(ctx.quest_comp.info.progress.entered_level_plays, lp_id)) return false;
    const nodes = ctx.assets.level_play_nodes.getOwnerNodes(ctx.io, lp_id) catch return false;
    if (findLpNodeById(nodes, node_id)) |node| {
        if (node.data == .object) {
            runLevelPlayActions(ctx, lp_id, node.id, node.data.object.get("FinishActions"));
        }
    }
    try onLevelPlayEnabled(ctx, lp_id, false);
    return true;
}

pub fn syncOnLogin(ctx: LevelPlayContext) !void {
    try ensureActiveTimersScheduled(ctx);
    const pos = currentPlayerPos(ctx);
    const entered_snap = try ctx.alloc.arena.dupe(i32, ctx.quest_comp.info.progress.entered_level_plays);
    for (entered_snap) |lpid| {
        const lp = ctx.assets.tables.level_play_data.getDataById(lpid) orelse continue;
        if (!matchesSceneInstance(ctx.assets, lp, ctx.scene.instance_id)) continue;
        if (QuestInfo.containsInt(ctx.quest_comp.info.completed_level_plays, lpid)) continue;
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.LevelPlayStateNotify{
            .Id = lpid,
            .State = 2,
            .CompleteNumber = 0,
        }) catch {};
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.EnterLevelPlayNotify{
            .Id = lpid,
            .State = 2,
            .CanGetReward = true,
        }) catch {};
        const nodes = ctx.assets.level_play_nodes.getOwnerNodes(ctx.io, lpid) catch continue;
        if (nodes.len == 0) continue;
        try pushLevelPlayBehaviorTreeInfo(ctx, lpid, nodes);
        for (nodes) |node| {
            if (isLpNodeActive(ctx.progress_comp, lpid, node.id)) {
                QuestEngine.pushDoInteractOptionsForBtNode(ctx.scene, ctx.conn, ctx.alloc, ctx.io, .BtTypeLevelPlay, lpid, node, null) catch {};
            }
        }
        try stepLevelPlayTree(ctx, lpid, nodes, true, pos);
    }
}

pub fn onPlayerMove(ctx: LevelPlayContext, x: f32, y: f32, z: f32) !void {
    var dirty_entered = false;
    const player_pos: [3]f32 = .{ x, y, z };

    // 1) Check currently entered LevelPlays for leaving LeaveRadius (3D spherical distance)
    if (ctx.quest_comp.info.progress.entered_level_plays.len > 0) {
        var idx: usize = 0;
        while (idx < ctx.quest_comp.info.progress.entered_level_plays.len) {
            const lpid = ctx.quest_comp.info.progress.entered_level_plays[idx];
            const lp_opt = ctx.assets.tables.level_play_data.getDataById(lpid);
            var still_inside = false;
            if (lp_opt) |lp| {
                if (matchesSceneInstance(ctx.assets, lp, ctx.scene.instance_id)) {
                    const enter_r: f32 = if (lp.Data.EnterRadius > 0) @floatFromInt(lp.Data.EnterRadius) else 3000.0;
                    const leave_r: f32 = if (lp.Data.LeaveRadius > 0) @floatFromInt(lp.Data.LeaveRadius) else enter_r + 1500.0;
                    if (resolveLevelPlayAnchorPos(ctx, lp)) |anchor| {
                        const dx = anchor[0] - x;
                        const dy = anchor[1] - y;
                        const dz = anchor[2] - z;
                        if (dx * dx + dy * dy + dz * dz <= leave_r * leave_r) {
                            still_inside = true;
                        }
                    } else {
                        still_inside = true;
                    }
                }
            }
            if (!still_inside) {
                if (try QuestInfo.removeIntValue(&ctx.quest_comp.info.progress.entered_level_plays, ctx.alloc.gpa, lpid)) {
                    dirty_entered = true;
                    _ = ctx.scene.sent_lp_bt_trees.remove(lpid);
                    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.LeaveLevelPlayNotify{ .Id = lpid }) catch {};
                    continue;
                }
            }
            idx += 1;
        }
    }

    // 2) Step currently entered LevelPlays (e.g. ReachArea child quests while inside the play)
    const entered_snap = try ctx.alloc.arena.dupe(i32, ctx.quest_comp.info.progress.entered_level_plays);
    for (entered_snap) |lpid| {
        const lp = ctx.assets.tables.level_play_data.getDataById(lpid) orelse continue;
        if (!matchesSceneInstance(ctx.assets, lp, ctx.scene.instance_id)) continue;
        const nodes = ctx.assets.level_play_nodes.getOwnerNodes(ctx.io, lpid) catch continue;
        try stepLevelPlayTree(ctx, lpid, nodes, true, player_pos);
    }

    // 3) Collect nearby LevelPlay anchors + enabled/quest-referenced LevelPlays
    const configs = ctx.scene.entities.slice().items(.config);
    var candidate_lps: std.ArrayList(i32) = .empty;
    for (configs) |cfg| {
        if (cfg.config_id <= 0) continue;
        const cfg_id32 = std.math.cast(i32, cfg.config_id) orelse continue;
        for (ctx.assets.indexes.levelPlayIndicesForEntity(cfg_id32)) |lp_idx| {
            const lp = ctx.assets.tables.level_play_data.items[lp_idx];
            if (!QuestInfo.containsInt(candidate_lps.items, lp.LevelPlayId)) {
                try candidate_lps.append(ctx.alloc.arena, lp.LevelPlayId);
            }
        }
    }
    if (ctx.progress_comp) |pc| {
        for (pc.info.dynamic.ints) |entry| {
            if (!std.mem.startsWith(u8, entry.key, "lp_state:") or entry.val != 2) continue;
            const lpid = std.fmt.parseInt(i32, entry.key["lp_state:".len..], 10) catch continue;
            if (!QuestInfo.containsInt(candidate_lps.items, lpid)) {
                try candidate_lps.append(ctx.alloc.arena, lpid);
            }
        }
    }
    for (ctx.quest_comp.info.active_quests) |aq| {
        if (aq.active_nodes.len == 0) continue;
        const qnodes = ctx.assets.quest_nodes.getOwnerNodes(ctx.io, aq.quest_id) catch continue;
        for (aq.active_nodes) |nid| {
            const qnode = findLpNodeById(qnodes, nid) orelse continue;
            if (!std.mem.eql(u8, qnode.node_type, "ChildQuest")) continue;
            const cond = QuestEngine.getChildConditionPub(qnode.data) orelse continue;
            if (!std.mem.eql(u8, jsonStr(cond.get("Type")) orelse "", "CheckLevelPlay")) continue;
            try appendConditionLevelPlayIds(ctx.alloc.arena, &candidate_lps, cond);
        }
    }

    for (candidate_lps.items) |lpid| {
        const lp = ctx.assets.tables.level_play_data.getDataById(lpid) orelse continue;
        if (!canEnterLevelPlay(ctx, lp)) continue;
        if (QuestInfo.containsInt(ctx.quest_comp.info.progress.entered_level_plays, lpid)) continue;
        if (lp.Data.EnterRadius > 0) {
            if (resolveLevelPlayAnchorPos(ctx, lp)) |anchor| {
                const enter_r: f32 = @floatFromInt(lp.Data.EnterRadius);
                const dx = anchor[0] - x;
                const dy = anchor[1] - y;
                const dz = anchor[2] - z;
                if (dx * dx + dy * dy + dz * dz > enter_r * enter_r) continue;
            }
        }
        try enterAndStartLevelPlay(ctx, lp, player_pos, &dirty_entered);
    }

    // Also evaluate non-spawnable map LevelPlay anchors (`LevelPlayEntityId` with spawnable=0) within 3D EnterRadius
    for (ctx.assets.tables.level_play_data.items) |lp| {
        if (lp.Data.EnterRadius <= 0 or lp.Data.LevelPlayEntityId <= 0) continue;
        if (std.mem.eql(u8, lp.Data.Type, "Quest") or std.mem.eql(u8, lp.Data.Type, "Dungeon") or std.mem.eql(u8, lp.Data.Type, "LordGym")) continue;
        if (QuestInfo.containsInt(candidate_lps.items, lp.LevelPlayId)) continue;
        if (!canEnterLevelPlay(ctx, lp)) continue;
        if (QuestInfo.containsInt(ctx.quest_comp.info.progress.entered_level_plays, lp.LevelPlayId)) continue;
        const lcfg = ctx.assets.findLevelEntity(ctx.scene.instance_id, lp.Data.LevelPlayEntityId) orelse continue;
        const anchor = Assets.AssetIndexes.entityWorldPos(lcfg) orelse continue;
        const enter_r: f32 = @floatFromInt(lp.Data.EnterRadius);
        const dx = anchor[0] - x;
        const dy = anchor[1] - y;
        const dz = anchor[2] - z;
        if (dx * dx + dy * dy + dz * dz > enter_r * enter_r) continue;
        try enterAndStartLevelPlay(ctx, lp, player_pos, &dirty_entered);
    }

    if (dirty_entered) {
        ctx.quest_comp.save(ctx.fs, ctx.alloc.arena) catch {};
    }
}

pub fn onLevelPlayEnabled(ctx: LevelPlayContext, level_play_id: i32, enabled: bool) !void {
    if (level_play_id <= 0) return;
    if (!enabled) {
        ctx.scene.lp_timers.cancelQuest(level_play_id);
        ctx.scene.fail_timers.cancelQuest(level_play_id);
        ctx.scene.lp_fail_timers.cancelQuest(level_play_id);
        ctx.scene.releaseOccupationsForTree(ctx.alloc.gpa, level_play_id, level_play_id);
        ctx.scene.clearSaveNodesForOwner(level_play_id);
        if (try QuestInfo.removeIntValue(&ctx.quest_comp.info.progress.entered_level_plays, ctx.alloc.gpa, level_play_id)) {
            ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.LevelPlayStateNotify{
                .Id = level_play_id,
                .State = 0,
                .CompleteNumber = 0,
            }) catch {};
            _ = ctx.scene.sent_lp_bt_trees.remove(level_play_id);
            ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.LeaveLevelPlayNotify{ .Id = level_play_id }) catch {};
            ctx.quest_comp.save(ctx.fs, ctx.alloc.arena) catch {};
        }
        return;
    }

    const lp = ctx.assets.tables.level_play_data.getDataById(level_play_id) orelse return;
    if (!canEnterLevelPlay(ctx, lp)) return;
    const pos = currentPlayerPos(ctx) orelse return;
    if (lp.Data.EnterRadius > 0) {
        if (resolveLevelPlayAnchorPos(ctx, lp)) |anchor| {
            const enter_r: f32 = @floatFromInt(lp.Data.EnterRadius);
            const dx = anchor[0] - pos[0];
            const dy = anchor[1] - pos[1];
            const dz = anchor[2] - pos[2];
            if (dx * dx + dy * dy + dz * dz > enter_r * enter_r) return;
        }
    }
    var dirty_entered = false;
    try enterAndStartLevelPlay(ctx, lp, pos, &dirty_entered);
    if (dirty_entered) {
        ctx.quest_comp.save(ctx.fs, ctx.alloc.arena) catch {};
    }
}

pub fn onDynamicInteract(ctx: LevelPlayContext, lp_id: i32, node_id: i32, entity_id: i64) !bool {
    if (lp_id <= 0 or node_id <= 0) return false;
    const lp = ctx.assets.tables.level_play_data.getDataById(lp_id) orelse return false;
    if (!matchesSceneInstance(ctx.assets, lp, ctx.scene.instance_id)) return false;
    if (QuestInfo.containsInt(ctx.quest_comp.info.completed_level_plays, lp_id)) return false;

    const nodes = ctx.assets.level_play_nodes.getOwnerNodes(ctx.io, lp_id) catch return false;
    const node = findLpNodeById(nodes, node_id) orelse return false;
    if (!std.mem.eql(u8, node.node_type, "ChildQuest")) return false;

    if (entity_id != 0) {
        if (QuestEngine.getChildConditionPub(node.data)) |cond| {
            const cfg_id: i32 = if (ctx.scene.net_id_map.get(entity_id)) |idx|
                std.math.cast(i32, ctx.scene.entities.items(.config)[idx].config_id) orelse 0
            else
                std.math.cast(i32, entity_id) orelse 0;
            var matched = false;
            var has_targets = false;
            if (jsonInt(cond.get("EntityId"))) |eid| {
                if (eid > 0) {
                    has_targets = true;
                    if (eid == cfg_id or @as(i64, eid) == entity_id) matched = true;
                }
            }
            if (cond.get("AddOptions")) |opts_v| {
                if (opts_v == .array and opts_v.array.items.len > 0) {
                    for (opts_v.array.items) |o| {
                        if (o != .object) continue;
                        const eid = jsonInt(o.object.get("EntityId")) orelse continue;
                        if (eid > 0) {
                            has_targets = true;
                            if (eid == cfg_id or @as(i64, eid) == entity_id) matched = true;
                        }
                    }
                }
            }
            if (cond.get("HandInItems")) |hi| {
                if (hi == .object) {
                    if (hi.object.get("AddOption")) |ao| {
                        if (ao == .object) {
                            if (jsonInt(ao.object.get("EntityId"))) |eid| {
                                if (eid > 0) {
                                    has_targets = true;
                                    if (eid == cfg_id or @as(i64, eid) == entity_id) matched = true;
                                }
                            }
                        }
                    }
                }
            }
            if (has_targets and !matched) return false;
        }
    }

    var dirty_pc = false;
    try completeLpChildQuest(ctx, lp_id, nodes, node.*, true, &dirty_pc);
    try stepLevelPlayTree(ctx, lp_id, nodes, true, currentPlayerPos(ctx));
    return true;
}

pub fn onVarChanged(ctx: LevelPlayContext, owner_id: i32, _: []const u8) !void {
    try evaluateEnteredLevelPlayTrees(ctx, owner_id);
}

pub fn onEntityStateChanged(ctx: LevelPlayContext, _: i32, _: []const u8) !void {
    try evaluateEnteredLevelPlayTrees(ctx, 0);
}

pub fn onConditionStateChanged(ctx: LevelPlayContext) !void {
    try evaluateEnteredLevelPlayTrees(ctx, 0);
}

pub fn stepEnteredTreeById(ctx: LevelPlayContext, lp_id: i32) !void {
    if (lp_id <= 0) return;
    try evaluateEnteredLevelPlayTrees(ctx, lp_id);
}

pub fn onBtTimerEnd(ctx: LevelPlayContext, lp_id: i32) !bool {
    if (lp_id <= 0) return false;
    if (QuestInfo.containsInt(ctx.quest_comp.info.completed_level_plays, lp_id)) return false;
    if (!QuestInfo.containsInt(ctx.quest_comp.info.progress.entered_level_plays, lp_id)) return false;
    const nodes = ctx.assets.level_play_nodes.getOwnerNodes(ctx.io, lp_id) catch return false;
    var progressed = false;
    var dirty_pc = false;
    for (nodes) |node| {
        if (!isLpNodeActive(ctx.progress_comp, lp_id, node.id)) continue;
        if (!std.mem.eql(u8, node.node_type, "ChildQuest")) continue;
        const cond = QuestEngine.getChildConditionPub(node.data) orelse continue;
        const ctype = jsonStr(cond.get("Type")) orelse continue;
        if (!std.mem.eql(u8, ctype, "Timer") and !std.mem.eql(u8, ctype, "WaitTime")) continue;
        try completeLpChildQuest(ctx, lp_id, nodes, node, true, &dirty_pc);
        progressed = true;
        break;
    }
    if (progressed) {
        try stepLevelPlayTree(ctx, lp_id, nodes, true, currentPlayerPos(ctx));
    }
    return progressed;
}

pub fn onLevelSequenceMarkReached(ctx: LevelPlayContext, entity_config_id: i32, mark: []const u8) !void {
    if (entity_config_id <= 0 or mark.len == 0) return;
    const entered_snap = try ctx.alloc.arena.dupe(i32, ctx.quest_comp.info.progress.entered_level_plays);
    for (entered_snap) |lpid| {
        if (QuestInfo.containsInt(ctx.quest_comp.info.completed_level_plays, lpid)) continue;
        const nodes = ctx.assets.level_play_nodes.getOwnerNodes(ctx.io, lpid) catch continue;
        var progressed = false;
        var dirty_pc = false;
        for (nodes) |node| {
            if (!isLpNodeActive(ctx.progress_comp, lpid, node.id)) continue;
            if (!std.mem.eql(u8, node.node_type, "ChildQuest")) continue;
            const cond = QuestEngine.getChildConditionPub(node.data) orelse continue;
            const ctype = jsonStr(cond.get("Type")) orelse continue;
            if (!std.mem.eql(u8, ctype, "WaitUntilLevelSequenceReachMark")) continue;
            if ((jsonInt(cond.get("EntityId")) orelse 0) != entity_config_id) continue;
            const required_mark = jsonStr(cond.get("Mark")) orelse continue;
            if (!std.mem.eql(u8, required_mark, mark)) continue;
            try completeLpChildQuest(ctx, lpid, nodes, node, true, &dirty_pc);
            progressed = true;
        }
        if (progressed) {
            try stepLevelPlayTree(ctx, lpid, nodes, true, currentPlayerPos(ctx));
        }
    }
}

pub fn onBtRollback(ctx: LevelPlayContext, lp_id: i32, fail_node_id: i32, reason: pb.FailReason) !void {
    if (lp_id <= 0) return;
    if (QuestInfo.containsInt(ctx.quest_comp.info.completed_level_plays, lp_id)) return;
    ctx.scene.lp_timers.cancelQuest(lp_id);
    ctx.scene.lp_fail_timers.cancelQuest(lp_id);
    ctx.scene.releaseOccupationsForTree(ctx.alloc.gpa, lp_id, lp_id);

    const nodes = ctx.assets.level_play_nodes.getOwnerNodes(ctx.io, lp_id) catch return;
    var latest_ckpt_id: i32 = 0;
    for (nodes) |n| {
        if (!isLevelPlayNodeMarked(ctx.progress_comp, lp_id, n.id)) continue;
        if (n.data == .object and n.data.object.get("SaveConfig") != null and n.id > latest_ckpt_id) {
            latest_ckpt_id = n.id;
        }
    }
    if (latest_ckpt_id == 0 and reason != .DefaultFail) {
        try onLevelPlayEnabled(ctx, lp_id, false);
        return;
    }

    var dirty_pc = false;
    const qctx = toQuestCtx(ctx);
    for (nodes) |n| {
        if (isLpNodeActive(ctx.progress_comp, lp_id, n.id)) {
            QuestEngine.removeDoInteractOptionsForBtNode(qctx, .BtTypeLevelPlay, lp_id, n.id);
            setLpNodeActive(ctx, lp_id, n.id, false, &dirty_pc);
        }
        if (n.id >= latest_ckpt_id) {
            clearLpNodeFinished(ctx, lp_id, n.id, &dirty_pc);
        }
    }
    if (dirty_pc) {
        if (ctx.progress_comp) |pc| {
            pc.save(ctx.fs, ctx.alloc.arena) catch {};
        }
    }
    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.BtRollbackNotify{
        .TreeOwnerId = lp_id,
        .TreeIncId = lp_id,
        .FailNodeId = fail_node_id,
        .FailReason = reason,
    }) catch {};
    try stepLevelPlayTree(ctx, lp_id, nodes, true, currentPlayerPos(ctx));
}

pub fn onFlowEnd(ctx: LevelPlayContext, lp_id: i32, node_id: i32) !void {
    if (lp_id <= 0 or node_id <= 0) return;
    if (ctx.assets.tables.level_play_data.getDataById(lp_id) == null) return;
    if (QuestInfo.containsInt(ctx.quest_comp.info.completed_level_plays, lp_id)) return;
    const nodes = ctx.assets.level_play_nodes.getOwnerNodes(ctx.io, lp_id) catch return;
    const node = findLpNodeById(nodes, node_id) orelse return;
    var dirty_pc = false;
    if (std.mem.eql(u8, node.node_type, "ChildQuest")) {
        try completeLpChildQuest(ctx, lp_id, nodes, node.*, true, &dirty_pc);
    } else if (std.mem.eql(u8, node.node_type, "Action") or std.mem.eql(u8, node.node_type, "ActionWithResult")) {
        finishLpNonChildNode(ctx, lp_id, node.*, true, &dirty_pc);
        if (dirty_pc) {
            if (ctx.progress_comp) |pc| {
                pc.save(ctx.fs, ctx.alloc.arena) catch {};
                dirty_pc = false;
            }
        }
    }
    try stepLevelPlayTree(ctx, lp_id, nodes, true, currentPlayerPos(ctx));
}

fn evaluateEnteredLevelPlayTrees(ctx: LevelPlayContext, preferred_lp_id: i32) !void {
    if (lp_step_depth > 0) {
        lp_recheck_pending = true;
        return;
    }
    const pos = currentPlayerPos(ctx);
    const entered_snap = try ctx.alloc.arena.dupe(i32, ctx.quest_comp.info.progress.entered_level_plays);
    for (entered_snap) |lpid| {
        if (preferred_lp_id > 0 and lpid != preferred_lp_id) continue;
        const lp = ctx.assets.tables.level_play_data.getDataById(lpid) orelse continue;
        if (!matchesSceneInstance(ctx.assets, lp, ctx.scene.instance_id)) continue;
        const nodes = ctx.assets.level_play_nodes.getOwnerNodes(ctx.io, lpid) catch continue;
        try stepLevelPlayTree(ctx, lpid, nodes, true, pos);
    }
    if (preferred_lp_id > 0 and !QuestInfo.containsInt(entered_snap, preferred_lp_id)) {
        if (ctx.assets.tables.level_play_data.getDataById(preferred_lp_id)) |lp| {
            if (matchesSceneInstance(ctx.assets, lp, ctx.scene.instance_id)) {
                const nodes = ctx.assets.level_play_nodes.getOwnerNodes(ctx.io, preferred_lp_id) catch return;
                try stepLevelPlayTree(ctx, preferred_lp_id, nodes, true, pos);
            }
        }
    }
}

pub const LevelPlayEventKind = enum {
    compare_actor_var,
    cross_scene_sync_kill,
    finish_flipper,
    finish_kuro_simple_combat_kill,
    finish_kurotato,
    finish_role_tower,
    finish_roll_block,
    finish_survivors_rouge,
    finish_tetris,
    finish_trap_defense,
    new_hand_in_items,
    parallax_align,
    playback_show_timeline,
    take_pictures_with_time_scale,
    walking_pattern,

    pub fn conditionType(self: LevelPlayEventKind) []const u8 {
        return switch (self) {
            .compare_actor_var => "CompareActorVar",
            .cross_scene_sync_kill => "CrossSceneSyncKillMonster",
            .finish_flipper => "FinishFlipper",
            .finish_kuro_simple_combat_kill => "FinishKuroSimpleCombatKillMonster",
            .finish_kurotato => "FinishKurotato",
            .finish_role_tower => "FinishRoleTower",
            .finish_roll_block => "FinishRollBlock",
            .finish_survivors_rouge => "FinishSurvivorsRouge",
            .finish_tetris => "FinishTetris",
            .finish_trap_defense => "FinishTrapDefense",
            .new_hand_in_items => "NewHandInItems",
            .parallax_align => "ParallaxAlign",
            .playback_show_timeline => "PlaybackShowTimeline",
            .take_pictures_with_time_scale => "TakePicturesWithTimeScale",
            .walking_pattern => "WalkingPattern",
        };
    }

    pub fn matchesConditionType(self: LevelPlayEventKind, ctype: []const u8) bool {
        if (std.mem.eql(u8, ctype, self.conditionType())) return true;
        return switch (self) {
            .take_pictures_with_time_scale => std.mem.eql(u8, ctype, "TakePhoto") or std.mem.eql(u8, ctype, "TakePhoto2"),
            else => false,
        };
    }
};

pub fn fromQuestContext(qctx: QuestEngine.QuestContext) ?LevelPlayContext {
    const scene = qctx.scene orelse return null;
    const role_comp = qctx.role_comp orelse return null;
    return .{
        .assets = qctx.assets,
        .fs = qctx.fs,
        .conn = qctx.conn,
        .scene = scene,
        .scene_comp = qctx.scene_comp,
        .events = qctx.events,
        .alloc = qctx.alloc,
        .io = qctx.io,
        .basic_comp = qctx.basic_comp,
        .inventory_comp = qctx.inventory_comp,
        .weapon_comp = qctx.weapon_comp,
        .echo_comp = qctx.echo_comp,
        .map_comp = qctx.map_comp,
        .quest_comp = qctx.quest_comp,
        .role_comp = role_comp,
        .guide_comp = qctx.guide_comp,
        .mail_comp = qctx.mail_comp,
        .progress_comp = qctx.progress_comp,
    };
}

pub fn allowsLevelPlayClientSubmit(ctype: []const u8) bool {
    const allowed = [_][]const u8{
        "CompareActorVar",
        "FinishFlipper",
        "FinishKurotato",
        "FinishRoleTower",
        "FinishRollBlock",
        "FinishSurvivorsRouge",
        "FinishTetris",
        "FinishTrapDefense",
        "NewHandInItems",
        "ParallaxAlign",
        "PlaybackShowTimeline",
        "TakePicturesWithTimeScale",
        "TakePhoto",
        "TakePhoto2",
        "WalkingPattern",
        "ReachArea",
        "CheckUiGame",
        "CheckEntityState",
        "CompareVar",
        "UseSkill",
        "UseItem",
        "Deliver",
        "Communicate",
        "Parkour",
        "RhythmGame",
        "SlidingBlocks",
        "CrossSceneSyncKillMonster",
        "FinishKuroSimpleCombatKillMonster",
        "ShowUi",
        "AwakeAndLoadEntity",
        "GetItem",
        "CheckPlayerInput",
        "ReadPhoneMessage",
        "FinishBvbChallenge",
        "FinishArcadeGameplay",
    };
    for (allowed) |name| {
        if (std.mem.eql(u8, ctype, name)) return true;
    }
    return false;
}

fn isLevelPlayNodeMarked(progress_comp: ?*const PlayerProgressComponent, lp_id: i32, node_id: i32) bool {
    const pc = progress_comp orelse return false;
    var kbuf: [48]u8 = undefined;
    const key = std.fmt.bufPrint(&kbuf, "lp_node:{d}:{d}", .{ lp_id, node_id }) catch return false;
    return pc.info.hasClaim(key) or pc.info.getKeyInt(key) != 0;
}

fn jsonIntVal(v_opt: ?std.json.Value) i64 {
    const v = v_opt orelse return 0;
    return switch (v) {
        .integer => |i| i,
        .float => |f| @intFromFloat(f),
        .string => |s| std.fmt.parseInt(i64, s, 10) catch 0,
        else => 0,
    };
}

fn hasPendingSiblingChildQuests(
    nodes: []const LazyNodeStore.NodeEntry,
    finished_node: LazyNodeStore.NodeEntry,
    progress_comp: ?*const PlayerProgressComponent,
    lp_id: i32,
) bool {
    const parent_id: i64 = if (finished_node.data == .object)
        jsonIntVal(finished_node.data.object.get("ParentNodeId"))
    else
        0;
    if (parent_id <= 0) return false;
    for (nodes) |other| {
        if (other.id == finished_node.id) continue;
        if (other.data != .object) continue;
        const ntype = jsonStr(other.data.object.get("Type")) orelse continue;
        if (!std.mem.eql(u8, ntype, "ChildQuest")) continue;
        if (jsonIntVal(other.data.object.get("ParentNodeId")) != parent_id) continue;
        const cond_v = other.data.object.get("Condition") orelse continue;
        if (cond_v != .object) continue;
        const ctype = jsonStr(cond_v.object.get("Type")) orelse continue;
        // Do not block ReachArea completion on another ReachArea or CompareVar stub.
        if (std.mem.eql(u8, ctype, "ReachArea") or std.mem.eql(u8, ctype, "CompareVar")) continue;
        if (!isLevelPlayNodeMarked(progress_comp, lp_id, other.id)) return true;
    }
    return false;
}

fn finishLevelPlayNodeAndMaybeComplete(
    ctx: LevelPlayContext,
    progress_comp: ?*PlayerProgressComponent,
    lp_id: i32,
    nodes: []const LazyNodeStore.NodeEntry,
    node: LazyNodeStore.NodeEntry,
) !void {
    var eff_ctx = ctx;
    if (eff_ctx.progress_comp == null and progress_comp != null) {
        eff_ctx.progress_comp = progress_comp;
    }
    var dirty_pc = false;
    if (isLpBtBooted(eff_ctx.progress_comp, lp_id) or isLpNodeActive(eff_ctx.progress_comp, lp_id, node.id)) {
        try completeLpChildQuest(eff_ctx, lp_id, nodes, node, true, &dirty_pc);
        try stepLevelPlayTree(eff_ctx, lp_id, nodes, true, currentPlayerPos(eff_ctx));
        return;
    }
    if (node.data == .object) {
        runLevelPlayActions(eff_ctx, lp_id, node.id, node.data.object.get("FinishActions"));
    }
    if (eff_ctx.progress_comp) |pc| {
        var kbuf: [48]u8 = undefined;
        const key = std.fmt.bufPrint(&kbuf, "lp_node:{d}:{d}", .{ lp_id, node.id }) catch "";
        if (key.len > 0) {
            if (try pc.info.markClaim(eff_ctx.alloc.gpa, key)) {
                try pc.save(eff_ctx.fs, eff_ctx.alloc.arena);
            }
        }
    }
    if (!hasPendingSiblingChildQuests(nodes, node, eff_ctx.progress_comp, lp_id)) {
        try completeLevelPlay(eff_ctx, lp_id);
    } else {
        eff_ctx.conn.push(eff_ctx.alloc.gpa, eff_ctx.io, pb.LevelPlayStateNotify{
            .Id = lp_id,
            .State = 2,
            .CompleteNumber = 0,
        }) catch {};
    }
}

fn findActiveLevelPlayId(ctx: LevelPlayContext, owner_id: i32, tree_inc_id: i64) ?i32 {
    if (owner_id > 0) {
        if (ctx.assets.tables.level_play_data.getDataById(owner_id) != null) return owner_id;
    }
    if (tree_inc_id > 0) {
        if (std.math.cast(i32, tree_inc_id)) |inc32| {
            if (ctx.assets.tables.level_play_data.getDataById(inc32) != null) return inc32;
        }
    }
    return null;
}

pub fn onSubmitLevelPlayNode(
    ctx: LevelPlayContext,
    progress_comp: ?*PlayerProgressComponent,
    owner_id: i32,
    tree_inc_id: i64,
    node_id: i32,
) !bool {
    if (node_id <= 0) return false;
    const lp_id = findActiveLevelPlayId(ctx, owner_id, tree_inc_id) orelse return false;
    const lp = ctx.assets.tables.level_play_data.getDataById(lp_id) orelse return false;
    if (!matchesSceneInstance(ctx.assets, lp, ctx.scene.instance_id)) return false;
    if (QuestInfo.containsInt(ctx.quest_comp.info.completed_level_plays, lp_id)) return false;

    const nodes = ctx.assets.level_play_nodes.getOwnerNodes(ctx.io, lp_id) catch return false;
    for (nodes) |node| {
        if (node.id != node_id or node.data != .object) continue;
        const ntype = jsonStr(node.data.object.get("Type")) orelse return false;
        if (!std.mem.eql(u8, ntype, "ChildQuest")) return false;
        const cond_val = node.data.object.get("Condition") orelse return false;
        if (cond_val != .object) return false;
        const ctype = jsonStr(cond_val.object.get("Type")) orelse return false;
        if (!allowsLevelPlayClientSubmit(ctype)) return false;

        try finishLevelPlayNodeAndMaybeComplete(ctx, progress_comp, lp_id, nodes, node);
        return true;
    }
    return false;
}

pub fn onSubmitPhotoLevelPlayNode(
    ctx: LevelPlayContext,
    progress_comp: ?*PlayerProgressComponent,
    owner_id: i32,
    node_id: i32,
) !bool {
    if (progress_comp) |pc| {
        _ = pc.info.addKeyInt(ctx.alloc.gpa, "explore:photo_cnt", 1) catch {};
    }
    if (owner_id > 0 and node_id > 0) {
        if (try onSubmitLevelPlayNode(ctx, progress_comp, owner_id, owner_id, node_id)) return true;
    }
    try onConditionEvent(ctx, progress_comp, .take_pictures_with_time_scale, 0, 0);
    return true;
}

fn conditionMatchesTargetEntity(kind: LevelPlayEventKind, cond: std.json.ObjectMap, entity_cfg_id: i64, extra_id: i32) bool {
    return switch (kind) {
        .new_hand_in_items => blk: {
            const hi = cond.get("HandIn") orelse break :blk entity_cfg_id == 0;
            if (hi != .object) break :blk false;
            const eid = jsonIntVal(hi.object.get("EntityId"));
            const iid = jsonIntVal(hi.object.get("ItemId"));
            if (entity_cfg_id > 0 and eid > 0 and eid != entity_cfg_id) break :blk false;
            if (extra_id > 0 and iid > 0 and iid != extra_id) break :blk false;
            break :blk true;
        },
        .walking_pattern => blk: {
            if (entity_cfg_id <= 0) break :blk true;
            const spline_id = jsonIntVal(cond.get("SplineEntityId"));
            const end_id = jsonIntVal(cond.get("EndEntityId"));
            break :blk spline_id == entity_cfg_id or end_id == entity_cfg_id or (spline_id == 0 and end_id == 0);
        },
        .finish_roll_block => blk: {
            if (entity_cfg_id <= 0) break :blk true;
            const center_id = jsonIntVal(cond.get("CenterEntityId"));
            const interact_id = jsonIntVal(cond.get("InteractEntityId"));
            const grp_id = jsonIntVal(cond.get("RollBlockGroupId"));
            break :blk center_id == entity_cfg_id or interact_id == entity_cfg_id or (extra_id > 0 and grp_id == extra_id);
        },
        .finish_tetris => blk: {
            if (entity_cfg_id <= 0) break :blk true;
            const origin_id = jsonIntVal(cond.get("OriginPosEntityId"));
            break :blk origin_id == 0 or origin_id == entity_cfg_id;
        },
        else => true,
    };
}

pub fn onConditionEvent(
    ctx: LevelPlayContext,
    progress_comp: ?*PlayerProgressComponent,
    kind: LevelPlayEventKind,
    entity_cfg_id: i64,
    extra_id: i32,
) !void {
    if (progress_comp) |pc| {
        const stat_key: ?[]const u8 = switch (kind) {
            .parallax_align => "parallax:done",
            .playback_show_timeline => "timeline:done",
            .finish_flipper => "flipper:fin",
            .finish_roll_block => "rb_blk:fin",
            .walking_pattern => "walk_pat:done",
            else => null,
        };
        if (stat_key) |k| {
            _ = pc.info.addKeyInt(ctx.alloc.gpa, k, 1) catch {};
            pc.save(ctx.fs, ctx.alloc.arena) catch {};
        }
    }

    var candidate_ids: std.ArrayList(i32) = .empty;
    for (ctx.quest_comp.info.progress.entered_level_plays) |lpid| {
        if (!QuestInfo.containsInt(candidate_ids.items, lpid)) {
            try candidate_ids.append(ctx.alloc.arena, lpid);
        }
    }
    if (std.math.cast(i32, entity_cfg_id)) |cfg32| {
        if (cfg32 > 0) {
            for (ctx.assets.indexes.levelPlayIndicesForRefEntity(cfg32)) |lp_idx| {
                const lpid = ctx.assets.tables.level_play_data.items[lp_idx].LevelPlayId;
                if (!QuestInfo.containsInt(candidate_ids.items, lpid)) {
                    try candidate_ids.append(ctx.alloc.arena, lpid);
                }
            }
            for (ctx.assets.indexes.levelPlayIndicesForEntity(cfg32)) |lp_idx| {
                const lpid = ctx.assets.tables.level_play_data.items[lp_idx].LevelPlayId;
                if (!QuestInfo.containsInt(candidate_ids.items, lpid)) {
                    try candidate_ids.append(ctx.alloc.arena, lpid);
                }
            }
        }
    }

    for (candidate_ids.items) |lp_id| {
        const lp = ctx.assets.tables.level_play_data.getDataById(lp_id) orelse continue;
        if (!matchesSceneInstance(ctx.assets, lp, ctx.scene.instance_id)) continue;
        if (QuestInfo.containsInt(ctx.quest_comp.info.completed_level_plays, lp_id)) continue;

        const nodes = ctx.assets.level_play_nodes.getOwnerNodes(ctx.io, lp_id) catch continue;
        for (nodes) |node| {
            if (node.data != .object) continue;
            const ntype = jsonStr(node.data.object.get("Type")) orelse continue;
            if (!std.mem.eql(u8, ntype, "ChildQuest")) continue;
            const cond_val = node.data.object.get("Condition") orelse continue;
            if (cond_val != .object) continue;
            const ctype = jsonStr(cond_val.object.get("Type")) orelse continue;
            if (!kind.matchesConditionType(ctype)) continue;
            if (!conditionMatchesTargetEntity(kind, cond_val.object, entity_cfg_id, extra_id)) continue;

            try finishLevelPlayNodeAndMaybeComplete(ctx, progress_comp, lp_id, nodes, node);
            break;
        }
    }
}

pub fn onEntityKilledWithConditions(
    ctx: LevelPlayContext,
    progress_comp: ?*PlayerProgressComponent,
    target_net_id: i64,
    cfg_id_opt: ?i32,
) !void {
    const killed_cfg: i32 = cfg_id_opt orelse blk: {
        const idx = ctx.scene.net_id_map.get(target_net_id) orelse break :blk 0;
        break :blk std.math.cast(i32, ctx.scene.entities.items(.config)[idx].config_id) orelse 0;
    };
    try onEntityKilled(ctx, target_net_id, cfg_id_opt);
    if (killed_cfg <= 0) return;

    var candidate_ids: std.ArrayList(i32) = .empty;
    for (ctx.quest_comp.info.progress.entered_level_plays) |lpid| {
        if (!QuestInfo.containsInt(candidate_ids.items, lpid)) {
            try candidate_ids.append(ctx.alloc.arena, lpid);
        }
    }
    for (ctx.assets.indexes.levelPlayIndicesForRefEntity(killed_cfg)) |lp_idx| {
        const lpid = ctx.assets.tables.level_play_data.items[lp_idx].LevelPlayId;
        if (!QuestInfo.containsInt(candidate_ids.items, lpid)) {
            try candidate_ids.append(ctx.alloc.arena, lpid);
        }
    }

    for (candidate_ids.items) |lp_id| {
        const lp = ctx.assets.tables.level_play_data.getDataById(lp_id) orelse continue;
        if (!matchesSceneInstance(ctx.assets, lp, ctx.scene.instance_id)) continue;
        if (QuestInfo.containsInt(ctx.quest_comp.info.completed_level_plays, lp_id)) continue;

        const nodes = ctx.assets.level_play_nodes.getOwnerNodes(ctx.io, lp_id) catch continue;
        for (nodes) |node| {
            if (node.data != .object) continue;
            const ntype = jsonStr(node.data.object.get("Type")) orelse continue;
            if (!std.mem.eql(u8, ntype, "ChildQuest")) continue;
            const cond_val = node.data.object.get("Condition") orelse continue;
            if (cond_val != .object) continue;
            const ctype = jsonStr(cond_val.object.get("Type")) orelse continue;
            if (!std.mem.eql(u8, ctype, "CrossSceneSyncKillMonster") and
                !std.mem.eql(u8, ctype, "FinishKuroSimpleCombatKillMonster")) continue;
            if (!isLpNodeActive(progress_comp, lp_id, node.id) and
                !QuestInfo.containsInt(ctx.quest_comp.info.progress.entered_level_plays, lp_id)) continue;

            var has_filter = false;
            var matches_filter = false;
            if (jsonInt(cond_val.object.get("EntityId"))) |eid| {
                if (eid > 0) {
                    has_filter = true;
                    if (eid == killed_cfg) matches_filter = true;
                }
            }
            inline for (.{ "EntityIds", "ExistTargets", "TargetsToAwake", "MonsterCreatorEntityIds" }) |field| {
                if (cond_val.object.get(field)) |arr| {
                    if (arr == .array and arr.array.items.len > 0) {
                        has_filter = true;
                        if (jsonArrayContainsInt(arr, killed_cfg)) matches_filter = true;
                    }
                }
            }
            if (has_filter and !matches_filter) continue;

            try finishLevelPlayNodeAndMaybeComplete(ctx, progress_comp, lp_id, nodes, node);
            break;
        }
    }
}

fn makeTestInstanceDungeon(id: i32, map_config_id: i32) Assets.DataTables.InstanceDungeon {
    return .{
        .Id = id,
        .MapConfigId = map_config_id,
        .MapName = "",
        .InstType = 0,
        .InstSubType = 0,
        .WorldDungeonSubType = 0,
        .OnlineType = 0,
        .CustomTypes = &.{},
        .MiniMapId = 0,
        .FightFormationId = 0,
        .RoleTypeList = .{ 0, 0, 0 },
        .TrialRoleInfo = .{ 0, 0, 0 },
        .ReviveId = 0,
        .BornPosition = .{ 0, 0, 0 },
        .BornRotation = .{ 0, 0, 0 },
        .RecoverWorldLocation = &.{},
        .ExitDungeonConfirmId = &.{},
        .EntranceEntities = &.{},
        .ExitEntities = &.{},
        .FirstRewardId = 0,
        .RewardId = 0,
        .RepeatRewardId = 0,
        .ExchangeRewardId = 0,
        .DropVisionLimit = 0,
        .SharedTreasureGroup = &.{},
        .EnterControlId = 0,
        .EnterCondition = &.{},
        .EntityLevel = 0,
        .AutoLeaveTime = 0,
        .VerifyCreatureGen = false,
    };
}

test "matchesSceneInstance respects InstanceId override and rejects cross-map LevelId" {
    const testing = std.testing;
    const dungeons = [_]Assets.DataTables.InstanceDungeon{
        makeTestInstanceDungeon(8, 8),
        makeTestInstanceDungeon(904, 904),
        makeTestInstanceDungeon(909, 909),
        makeTestInstanceDungeon(1500, 8),
    };
    var inst_map: std.array_hash_map.Auto(i32, usize) = .empty;
    defer inst_map.deinit(testing.allocator);
    for (dungeons, 0..) |d, i| try inst_map.put(testing.allocator, d.Id, i);

    var assets: Assets = undefined;
    assets.tables.instance_dungeon = .{ .items = &dungeons, .index = inst_map };

    const lp_909: Assets.DataTables.LevelPlayData = .{
        .LevelPlayId = 109000078,
        .Data = .{ .LevelId = 909, .InstanceId = 0, .Type = "Quest" },
    };
    try testing.expect(!matchesSceneInstance(&assets, lp_909, 904));
    try testing.expect(!matchesSceneInstance(&assets, lp_909, 8));
    try testing.expect(matchesSceneInstance(&assets, lp_909, 909));

    // InstanceId overrides LevelId when both are set.
    const lp_inst_override: Assets.DataTables.LevelPlayData = .{
        .LevelPlayId = 16000004,
        .Data = .{ .LevelId = 100, .InstanceId = 8, .Type = "Challenge" },
    };
    try testing.expect(matchesSceneInstance(&assets, lp_inst_override, 8));
    try testing.expect(!matchesSceneInstance(&assets, lp_inst_override, 100));
    try testing.expect(matchesSceneInstance(&assets, lp_inst_override, 1500));
}

test "all 15 LevelPlayNodeData Condition.Types map cleanly and allow valid submit or event triggers" {
    const testing = std.testing;
    const all_15 = [_][]const u8{
        "CompareActorVar",
        "CrossSceneSyncKillMonster",
        "FinishFlipper",
        "FinishKuroSimpleCombatKillMonster",
        "FinishKurotato",
        "FinishRoleTower",
        "FinishRollBlock",
        "FinishSurvivorsRouge",
        "FinishTetris",
        "FinishTrapDefense",
        "NewHandInItems",
        "ParallaxAlign",
        "PlaybackShowTimeline",
        "TakePicturesWithTimeScale",
        "WalkingPattern",
    };
    for (all_15) |name| {
        try testing.expect(allowsLevelPlayClientSubmit(name));
    }
    try testing.expect(!allowsLevelPlayClientSubmit("Kill"));
    try testing.expect(!allowsLevelPlayClientSubmit("PlayFlow"));

    // Verify LevelPlayEventKind covers all 15 types 1:1
    inline for (@typeInfo(LevelPlayEventKind).@"enum".fields) |f| {
        const kind: LevelPlayEventKind = @enumFromInt(f.value);
        try testing.expect(kind.matchesConditionType(kind.conditionType()));
    }
    try testing.expect(LevelPlayEventKind.take_pictures_with_time_scale.matchesConditionType("TakePhoto2"));

    // Verify conditionMatchesTargetEntity validates NewHandInItems, WalkingPattern, FinishRollBlock, FinishTetris
    const handin = try std.json.parseFromSlice(std.json.Value, testing.allocator,
        \\{"Type":"NewHandInItems","HandIn":{"Type":"ProjectionMachine","EntityId":39000113,"ItemId":70140004}}
    , .{});
    defer handin.deinit();
    try testing.expect(conditionMatchesTargetEntity(.new_hand_in_items, handin.value.object, 39000113, 70140004));
    try testing.expect(!conditionMatchesTargetEntity(.new_hand_in_items, handin.value.object, 999, 70140004));

    const wp = try std.json.parseFromSlice(std.json.Value, testing.allocator,
        \\{"Type":"WalkingPattern","SplineEntityId":122000345,"EndEntityId":122000342}
    , .{});
    defer wp.deinit();
    try testing.expect(conditionMatchesTargetEntity(.walking_pattern, wp.value.object, 122000342, 0));
    try testing.expect(!conditionMatchesTargetEntity(.walking_pattern, wp.value.object, 123, 0));
}
