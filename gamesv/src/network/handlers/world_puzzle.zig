const std = @import("std");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const common = @import("common");
const FileSystem = common.FileSystem;
const Assets = @import("../../data/Assets.zig");
const Scene = @import("../../logic/Scene.zig");
const PlayerID = @import("../../logic/PlayerID.zig");
const EventQueue = @import("../../logic/EventQueue.zig");
const Transaction = @import("../handlers.zig").Transaction;
const PlayerBasicComponent = @import("../../logic/component/player/PlayerBasicComponent.zig");
const PlayerInventoryComponent = @import("../../logic/component/player/PlayerInventoryComponent.zig");
const PlayerWeaponComponent = @import("../../logic/component/player/PlayerWeaponComponent.zig");
const PlayerEchoComponent = @import("../../logic/component/player/PlayerEchoComponent.zig");
const PlayerRoleComponent = @import("../../logic/component/player/PlayerRoleComponent.zig");
const PlayerMapComponent = @import("../../logic/component/player/PlayerMapComponent.zig");
const PlayerQuestComponent = @import("../../logic/component/player/PlayerQuestComponent.zig");
const PlayerProgressComponent = @import("../../logic/component/player/PlayerProgressComponent.zig");
const ConsumeHelper = @import("../../logic/helpers/consume.zig");
const RewardService = @import("../../logic/helpers/reward.zig");
const RoleHelper = @import("../../logic/helpers/role.zig");
const QuestEngine = @import("../../logic/helpers/quest_engine.zig");
const ConditionEvaluator = @import("../../logic/helpers/condition.zig");
const LevelPlayHelper = @import("../../logic/helpers/level_play.zig");
const gameplay_tags = @import("../../logic/helpers/gameplay_tags.zig");
const CoopService = @import("../../logic/CoopService.zig");
const map_handler = @import("map.zig");
const QuestInfo = @import("../../fs/QuestInfo.zig");
const BuffTimerScheduler = @import("../../logic/schedulers/BuffTimerScheduler.zig");
const entity_proto = @import("../../logic/helpers/entity_proto.zig");
const EnergyHelper = @import("../../logic/helpers/energy.zig");
const pb_kv = @import("pb_kv.zig");
const spline_state = @import("../../logic/helpers/spline_state.zig");

const log = std.log.scoped(.world_puzzle);

fn persistIntKey(
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
    comptime prefix: []const u8,
    id: i64,
    val: i64,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "{s}:{d}", .{ prefix, id });
    try pc.info.setKeyInt(alloc.gpa, key, val);
    try pc.save(fs, alloc.arena);
}

fn persistClaimKey(
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
    comptime prefix: []const u8,
    id: i64,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "{s}:{d}", .{ prefix, id });
    _ = try pc.info.markClaim(alloc.gpa, key);
    try pc.save(fs, alloc.arena);
}

pub fn buildLevelPlayVars(
    arena: std.mem.Allocator,
    quest_comp: *const PlayerQuestComponent,
) !std.ArrayList(pb.MapEntry([]const u8, pb.VarDefinePb)) {
    var vars: std.ArrayList(pb.MapEntry([]const u8, pb.VarDefinePb)) = .empty;
    for (quest_comp.info.vars) |qv| {
        const vdef: pb.VarDefinePb = if (qv.str_val.len > 0) .{
            .VarType = 3,
            .Value = .{ .String = qv.str_val },
        } else if (qv.bool_val) .{
            .VarType = 1,
            .Value = .{ .Boolean = true },
        } else .{
            .VarType = 2,
            .Value = .{ .Int = qv.int_val },
        };
        try vars.append(arena, .{ .key = qv.key, .value = vdef });
    }
    return vars;
}

// =========================================================================
// Entity / Puzzle / Spline / HookLock / Gravity / BeControlled / Combine
// =========================================================================

fn safeFloatToI64(f: f32) i64 {
    if (!std.math.isFinite(f)) return 0;
    const clamped = std.math.clamp(f, -9_000_000_000_000_000_000.0, 9_000_000_000_000_000_000.0);
    return @intFromFloat(clamped);
}

fn stateNameFromTagId(
    assets: *const Assets,
    map_id: i32,
    entity_cfg_id: i64,
    tag_id: i32,
) ?[]const u8 {
    if (tag_id == 0) return null;
    const remaps = &assets.tables.gameplay_tag_remap;
    const candidates = [_][]const u8{
        "关卡.Common.状态.常态",
        "关卡.Common.状态.激活",
        "关卡.Common.状态.完成",
        "关卡.Common.状态.销毁",
        "关卡.Common.状态.解锁",
        "关卡.Common.状态.锁定",
        "关卡.Common.状态.封锁",
        "关卡.Common.状态.失败",
        "关卡.Common.状态.交互中",
        "关卡.Common.状态.准备中",
        "关卡.Common.状态.低压",
        "关卡.Common.状态.高压",
        "关卡.Common.状态.死眼命中",
        "关卡.Common.状态.场景物件交互中",
        "关卡.Common.a79721bf.常态",
        "关卡.Common.a79721bf.激活",
        "关卡.Common.表现.7a21323a.常态",
        "关卡.Common.表现.娃娃机.常态",
        "关卡.Common.表现.娃娃机.进行中",
    };
    for (candidates) |name| {
        const id = gameplay_tags.idFromName(remaps, name) catch continue;
        if (id == tag_id) return name;
    }
    if (assets.findLevelEntity(map_id, entity_cfg_id)) |lcfg| {
        if (lcfg.ComponentsData.EntityStateComponent) |esc| {
            if (esc.State) |st| {
                const id = gameplay_tags.idFromName(remaps, st) catch 0;
                if (id == tag_id) return st;
            }
        }
    }
    return null;
}

pub fn onChangeEntityStateRequest(
    txn: *Transaction(pb.ChangeEntityStateRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    assets: *const Assets,
    scene_opt: *?Scene,
    map_comp: *PlayerMapComponent,
    quest_comp: *PlayerQuestComponent,
    progress_comp: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    const map_id: i32 = if (scene_opt.*) |*scene| assets.resolveMapConfigId(scene.instance_id) else 8;
    const resolved_cfg_id: i64 = if (scene_opt.*) |*scene|
        (scene.findConfigIdByNetId(txn.message.EntityId) orelse txn.message.EntityId)
    else
        txn.message.EntityId;
    const state_name_opt = stateNameFromTagId(assets, map_id, resolved_cfg_id, txn.message.StateTagId);
    if (scene_opt.*) |*scene| {
        if (txn.message.EntityId != 0) {
            const net_id_opt: ?i64 = scene.findEntityNetIdByConfigId(txn.message.EntityId) orelse
                if (scene.net_id_map.contains(@as(i64, txn.message.EntityId))) @as(i64, txn.message.EntityId) else null;
            if (net_id_opt) |net_id| {
                if (try scene.updateEntityStateTagId(alloc.gpa, net_id, txn.message.StateTagId)) |upd| {
                    try QuestEngine.emitEntityStateTagUpdate(txn.conn, alloc.gpa, alloc.arena, io, net_id, upd);
                }
            }

            if (state_name_opt) |sname| {
                if (std.mem.eql(u8, sname, "关卡.Common.状态.激活")) {
                    var tp_dirty = false;
                    var newly_unlocked_tps: std.ArrayList(i32) = .empty;
                    for (assets.indexes.teleporterIndicesForMap(map_id)) |tp_idx| {
                        const tp = assets.tables.teleporter.items[tp_idx];
                        if (tp.TeleportEntityConfigId != resolved_cfg_id and tp.TeleportEntityConfigId != txn.message.EntityId) continue;
                        if (try map_comp.info.unlockTeleporter(alloc.gpa, tp.Id)) {
                            tp_dirty = true;
                            try newly_unlocked_tps.append(alloc.arena, tp.Id);
                        }
                        if (tp.Type == 1 and tp.FogId > 0 and try map_handler.unlockFogAndSiblings(txn.conn, assets, map_comp, alloc.gpa, io, tp.FogId)) {
                            tp_dirty = true;
                        }
                    }
                    if (tp_dirty) {
                        try map_comp.save(fs, alloc.arena);
                        if (newly_unlocked_tps.items.len > 0) {
                            txn.conn.pushBestEffort(alloc.gpa, io, pb.TeleportUpdateNotify{ .Ids = newly_unlocked_tps });
                        }
                    }
                }
            }
        }
    }
    if (resolved_cfg_id != 0) {
        if (state_name_opt) |sname| {
            try quest_comp.info.setEntityState(alloc.gpa, resolved_cfg_id, sname);
            try quest_comp.save(fs, alloc.arena);
            if (resolved_cfg_id > 0 and resolved_cfg_id <= std.math.maxInt(i32)) {
                QuestEngine.onEntityStateChanged(qctx, @intCast(resolved_cfg_id), sname) catch |err| QuestEngine.logHookErr("onEntityStateChanged", err);
            }
        }
    }
    const key = try std.fmt.allocPrint(alloc.arena, "ent_state:{d}", .{resolved_cfg_id});
    try progress_comp.info.setKeyInt(alloc.gpa, key, txn.message.StateTagId);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onChangeEntityRoleRequest(txn: *Transaction(pb.ChangeEntityRoleRequest), player_id: PlayerID) !void {
    txn.respond(.{
        .Success = true,
        .PlayerId = if (txn.message.PlayerId != 0) txn.message.PlayerId else player_id.id,
        .Id = txn.message.Id,
    });
}

pub fn onEntityBlackboardRequest(
    txn: *Transaction(pb.EntityBlackboardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    scene_opt: *?Scene,
    progress_comp: *PlayerProgressComponent,
) !void {
    var entity_key_id = txn.message.EntityId;
    if (scene_opt.*) |*scene| {
        entity_key_id = scene.findConfigIdByNetId(txn.message.EntityId) orelse entity_key_id;
    }
    for (txn.message.Params.items) |p| {
        if (p.Key.len == 0) continue;
        const key = try std.fmt.allocPrint(alloc.arena, "ent_bb:{d}:{s}", .{ entity_key_id, p.Key });
        if (p.Value) |val| {
            switch (val) {
                .IntValue => |v| try progress_comp.info.setKeyInt(alloc.gpa, key, v),
                .LongValue => |v| try progress_comp.info.setKeyInt(alloc.gpa, key, v),
                .BooleanValue => |v| try progress_comp.info.setKeyInt(alloc.gpa, key, if (v) 1 else 0),
                .StringValue => |s| try progress_comp.info.setKeyStr(alloc.gpa, key, s),
                .FloatValue => |f| try progress_comp.info.setKeyInt(alloc.gpa, key, safeFloatToI64(f * 1000.0)),
                else => _ = try progress_comp.info.markClaim(alloc.gpa, key),
            }
        } else {
            _ = try progress_comp.info.markClaim(alloc.gpa, key);
        }
    }
    if (scene_opt.*) |*scene| {
        if (scene.net_id_map.get(txn.message.EntityId)) |idx| {
            if (scene.entities.items(.fsm)[idx] != null) {
                try scene.fsm_wakes.markDirty(alloc.gpa, txn.message.EntityId);
            }
            if (scene.entities.items(.config)[idx].entity_type == .scene_item) {
                const config_id = scene.entities.items(.config)[idx].config_id;
                var scene_item = scene.entities.items(.world_item)[idx].scene_item orelse Scene.Entity.SceneItemComponent{ .pos_sender = scene.player_id };
                for (txn.message.Params.items) |p| {
                    if (!std.mem.eql(u8, p.Key, "ManipulatableState")) continue;
                    const item_state = if (p.Value) |value| switch (value) {
                        .IntValue => |v| std.math.cast(i32, v),
                        .LongValue => |v| std.math.cast(i32, v),
                        .BooleanValue => |v| @as(?i32, @intFromBool(v)),
                        else => null,
                    } else @as(?i32, 1);
                    if (item_state) |state| {
                        scene_item.manipulatable_state = state;
                        if (config_id > 0) {
                            const item_key = try std.fmt.allocPrint(alloc.arena, "item_bb:{d}:0", .{config_id});
                            try progress_comp.info.setKeyInt(alloc.gpa, item_key, state);
                        }
                    }
                }
                scene.entities.items(.world_item)[idx].scene_item = scene_item;
            }
        }
    }
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onUpdateSceneItemBlackboardRequest(
    txn: *Transaction(pb.UpdateSceneItemBlackboardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    const scene = qctx.scene orelse {
        txn.respond(.{ .ErrorCode = .ErrSceneEntityNotExist });
        return;
    };
    const entity_index = scene.net_id_map.get(txn.message.EntityId) orelse {
        txn.respond(.{ .ErrorCode = .ErrSceneEntityNotExist });
        return;
    };
    const entity_config = scene.entities.items(.config)[entity_index];
    if (entity_config.entity_type != .scene_item or entity_config.config_id <= 0) {
        txn.respond(.{ .ErrorCode = .ErrSceneEntityNotExist });
        return;
    }
    for (txn.message.Params.items) |p| {
        const bb_k = @intFromEnum(p.Key orelse .ManipulatableState);
        const key = try std.fmt.allocPrint(alloc.arena, "item_bb:{d}:{d}", .{ entity_config.config_id, bb_k });
        if (p.Value) |val| {
            switch (val) {
                .IntValue => |v| try progress_comp.info.setKeyInt(alloc.gpa, key, v),
                .LongValue => |v| try progress_comp.info.setKeyInt(alloc.gpa, key, v),
                .BooleanValue => |v| try progress_comp.info.setKeyInt(alloc.gpa, key, if (v) 1 else 0),
                .StringValue => |s| try progress_comp.info.setKeyStr(alloc.gpa, key, s),
                .FloatValue => |f| try progress_comp.info.setKeyInt(alloc.gpa, key, safeFloatToI64(f)),
                else => try progress_comp.info.setKeyInt(alloc.gpa, key, if (bb_k == 0) 1 else bb_k),
            }
        } else {
            try progress_comp.info.setKeyInt(alloc.gpa, key, if (bb_k == 0) 1 else bb_k);
        }
    }
    {
        var scene_item = scene.entities.items(.world_item)[entity_index].scene_item orelse Scene.Entity.SceneItemComponent{ .pos_sender = scene.player_id };
        for (txn.message.Params.items) |p| {
            if ((p.Key orelse .ManipulatableState) != .ManipulatableState) continue;
            scene_item.manipulatable_state = if (p.Value) |value| switch (value) {
                .IntValue => |v| v,
                .LongValue => |v| std.math.cast(i32, v),
                .BooleanValue => |v| if (v) 1 else 0,
                else => scene_item.manipulatable_state,
            } else @as(?i32, 1);
        }
        scene.entities.items(.world_item)[entity_index].scene_item = scene_item;
    }
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
    try QuestEngine.onConditionStateChanged(qctx);
}

pub fn onEntityAdsorbRequest(
    txn: *Transaction(pb.EntityAdsorbRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistClaimKey(alloc, fs, pc, "wp:adsorb", txn.message.EntityId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onEntityBattleInfoRequest(
    txn: *Transaction(pb.EntityBattleInfoRequest),
    scene_opt: *?Scene,
    alloc: mem.Alloc,
) !void {
    var attrs: std.ArrayList(pb.GameplayAttributeData) = .empty;
    var buffs_pb: ?pb.FightBuffComponentPb = null;
    if (scene_opt.*) |*scene| {
        if (scene.net_id_map.get(txn.message.EntityId)) |idx| {
            if (scene.entities.items(.attribute)[idx]) |attr| {
                for (attr.attributes, 0..) |a, ai| {
                    if (a.current == 0 and a.base == 0) continue;
                    const atype = std.enums.fromInt(pb.EAttributeType, @as(i32, @intCast(ai))) orelse continue;
                    try attrs.append(alloc.arena, .{
                        .AttributeType = atype,
                        .BaseValue = a.base,
                        .CurrentValue = a.current,
                    });
                }
            }
            if (scene.entities.items(.buffs)[idx]) |buff_comp| {
                buffs_pb = buff_comp.toProto();
            }
        }
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .EntityId = txn.message.EntityId,
        .Attributes = attrs,
        .FightBuffComponentPb = buffs_pb,
    });
}

pub fn onEntityBeamReceiveRequest(
    txn: *Transaction(pb.EntityBeamReceiveRequest),
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    const is_satisfied = if (txn.message.ReceiveType) |rt| rt != .StopAction else true;
    txn.conn.pushBestEffort(alloc.gpa, io, pb.BeamReceiveNotify{
        .EntityId = txn.message.EntityId,
        .IsSatisfied = is_satisfied,
    });
    txn.respond(.{
        .ReceiveType = txn.message.ReceiveType,
        .ErrorCode = .Success,
        .EntityId = txn.message.EntityId,
    });
}

pub fn onEntityCameraAlertRequest(
    txn: *Transaction(pb.EntityCameraAlertRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    scene_opt: *?Scene,
    progress_comp: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    if (scene_opt.*) |scene| {
        const config_id = scene.findConfigIdByNetId(txn.message.EntityId) orelse
            std.math.cast(i32, txn.message.EntityId) orelse 0;
        if (config_id > 0) {
            if (assets.findLevelEntity(scene.instance_id, config_id)) |config| {
                if (config.AreaId > 0) {
                    const key = try std.fmt.allocPrint(alloc.arena, "alert:area:{d}", .{config.AreaId});
                    try progress_comp.info.setKeyInt(alloc.gpa, key, @intFromBool(txn.message.IsAlert));
                    try progress_comp.save(fs, alloc.arena);
                }
            }
        }
    }
    txn.respond(.{
        .EntityId = txn.message.EntityId,
        .ErrorCode = .Success,
        .IsAlert = txn.message.IsAlert,
    });
    try QuestEngine.onConditionStateChanged(qctx);
}

pub fn onEntityPatrolAddTagRequest(
    txn: *Transaction(pb.EntityPatrolAddTagRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistIntKey(alloc, fs, pc, "patrol:tag", txn.message.EntityId, txn.message.TagId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onEntityPatrolChangeDirRequest(
    txn: *Transaction(pb.EntityPatrolChangeDirRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistIntKey(alloc, fs, pc, "patrol:dir", txn.message.EntityId, if (txn.message.Dir) 1 else 0);
    txn.respond(.{ .Code = .Success });
}

pub fn onEntityRandomInteractRequest(
    txn: *Transaction(pb.EntityRandomInteractRequest),
    qctx: QuestEngine.QuestContext,
) !void {
    const scene = qctx.scene orelse {
        txn.respond(.{ .ErrorCode = .ErrSceneEntityNotExist, .Interacting = false });
        return;
    };
    if (scene.active_flow_inc_id != 0) {
        txn.respond(.{ .ErrorCode = .ErrInteracTreeSuspend, .Interacting = false });
        return;
    }
    if (!try QuestEngine.onRandomInteract(qctx, txn.message.EntityId, txn.message.OptionIndex)) {
        txn.respond(.{ .ErrorCode = .ErrInteractOptionGuidInvalid, .Interacting = false });
        return;
    }
    txn.respond(.{ .ErrorCode = .Success, .Interacting = false });
}

pub fn onEntitySendEventRequest(
    txn: *Transaction(pb.EntitySendEventRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    scene_opt: *?Scene,
    progress_comp: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "ent_evt:{d}:{s}", .{ txn.message.EntityId, txn.message.EventKey });
    _ = try progress_comp.info.markClaim(alloc.gpa, key);
    try progress_comp.save(fs, alloc.arena);

    var cfg_id: i64 = txn.message.EntityId;
    if (scene_opt.*) |*scene| {
        if (scene.findConfigIdByNetId(txn.message.EntityId)) |cid| {
            cfg_id = cid;
        }
        if (scene.net_id_map.get(txn.message.EntityId)) |idx| {
            if (scene.entities.items(.fsm)[idx] != null) {
                try scene.fsm_wakes.markDirty(alloc.gpa, txn.message.EntityId);
            }
        }
    }
    if (cfg_id > 0 and cfg_id <= std.math.maxInt(i32)) {
        try QuestEngine.onEntityStateChanged(qctx, @intCast(cfg_id), txn.message.EventKey);
    }
    try QuestEngine.onConditionStateChanged(qctx);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onEntityWalkingPatternFinishRequest(
    txn: *Transaction(pb.EntityWalkingPatternFinishRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    try persistIntKey(alloc, fs, pc, "walk_pat:fin", txn.message.EntityId, txn.message.Point);
    if (LevelPlayHelper.fromQuestContext(qctx)) |lp_ctx| {
        const cfg_id = lp_ctx.scene.findConfigIdByNetId(txn.message.EntityId) orelse txn.message.EntityId;
        try LevelPlayHelper.onConditionEvent(lp_ctx, pc, .walking_pattern, cfg_id, txn.message.Point);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSubmitWalkingPatternPointRequest(
    txn: *Transaction(pb.SubmitWalkingPatternPointRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    try persistIntKey(alloc, fs, pc, "walk_pat:pt", txn.message.TreeIncId, txn.message.Point);
    if (txn.message.NodeId > 0) {
        if (LevelPlayHelper.fromQuestContext(qctx)) |lp_ctx| {
            _ = try LevelPlayHelper.onSubmitLevelPlayNode(lp_ctx, pc, txn.message.TreeOwnerId, txn.message.TreeIncId, txn.message.NodeId);
        }
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onMonsterPatrolPointReachedRequest(
    txn: *Transaction(pb.MonsterPatrolPointReachedRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistIntKey(alloc, fs, pc, "patrol:pt", txn.message.EntityId, txn.message.Index);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onVfxNpcPatrolFinishRequest(
    txn: *Transaction(pb.VfxNpcPatrolFinishRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistClaimKey(alloc, fs, pc, "vfx_patrol:fin", txn.message.EntityId);
    txn.respond(.{ .Code = .Success });
}

pub fn onNpcTraceFailedRequest(
    txn: *Transaction(pb.NpcTraceFailedRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistIntKey(alloc, fs, pc, "npc_trace:fail", txn.message.TreeIncId, txn.message.NodeId);
    txn.respond(.{ .ErrorId = .Success });
}

pub fn onQuestNpcMoveOverRequest(
    txn: *Transaction(pb.QuestNpcMoveOverRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistClaimKey(alloc, fs, pc, "quest_npc:over", txn.message.EntityId);
    txn.respond(.{ .ErrorId = .Success });
}

pub fn onProgressBarFinishRequest(
    txn: *Transaction(pb.ProgressBarFinishRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "prog_bar:{d}", .{txn.message.EntityId});
    _ = try progress_comp.info.markClaim(alloc.gpa, key);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSetInitTagRequest(
    txn: *Transaction(pb.SetInitTagRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistClaimKey(alloc, fs, pc, "init_tag", txn.message.EntityId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSetBaoziStateRequest(
    txn: *Transaction(pb.SetBaoziStateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "baozi:{d}", .{txn.message.BaoziEntityId});
    try progress_comp.info.setKeyInt(alloc.gpa, key, txn.message.IsActive);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onSetFanNumberOfTurnsRequest(
    txn: *Transaction(pb.SetFanNumberOfTurnsRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    scene_opt: *?Scene,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "fan_turns:{d}", .{txn.message.EntityId});
    try progress_comp.info.setKeyInt(alloc.gpa, key, txn.message.NumberOfTurns);
    try progress_comp.save(fs, alloc.arena);
    if (scene_opt.*) |*scene| {
        if (scene.net_id_map.get(txn.message.EntityId) orelse blk: {
            const cfg32 = std.math.cast(i32, txn.message.EntityId) orelse break :blk null;
            const nid = scene.findEntityNetIdByConfigId(cfg32) orelse break :blk null;
            break :blk scene.net_id_map.get(nid);
        }) |idx| {
            scene.entities.items(.world_item)[idx].fan_turns = txn.message.NumberOfTurns;
        }
    }
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onSetFanStateRequest(
    txn: *Transaction(pb.SetFanStateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "fan_state:{d}", .{txn.message.FanEntityId});
    try progress_comp.info.setKeyInt(alloc.gpa, key, txn.message.IsActive);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onElevatorStateChangeRequest(
    txn: *Transaction(pb.ElevatorStateChangeRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    scene_opt: *?Scene,
    progress_comp: *PlayerProgressComponent,
) !void {
    const scene = scene_opt.* orelse {
        txn.respond(.{ .ErrorCode = .ErrElevatorEntityNotExit });
        return;
    };
    const config_id = scene.findConfigIdByNetId(txn.message.EntityId) orelse {
        txn.respond(.{ .ErrorCode = .ErrElevatorEntityNotExit });
        return;
    };
    if (config_id <= 0) {
        txn.respond(.{ .ErrorCode = .ErrElevatorConfigNotExit });
        return;
    }
    const key = try std.fmt.allocPrint(alloc.arena, "elevator:{d}", .{config_id});
    try progress_comp.info.setKeyInt(alloc.gpa, key, txn.message.TagState);
    try progress_comp.save(fs, alloc.arena);
    txn.conn.pushBestEffort(alloc.gpa, io, pb.ElevatorMoveNotify{
        .EntityId = txn.message.EntityId,
        .Location = txn.message.TagState,
    });
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onHardnessModeChangedRequest(
    txn: *Transaction(pb.HardnessModeChangedRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistIntKey(alloc, fs, pc, "hardness", txn.message.EntityId, txn.message.HardnessModeId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSceneItemStateRequest(
    txn: *Transaction(pb.SceneItemStateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistIntKey(alloc, fs, pc, "scene_item_state", txn.message.EntityId, txn.message.OperateType);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSceneItemMoveSplineRequest(
    txn: *Transaction(pb.SceneItemMoveSplineRequest),
    scene_opt: *?Scene,
    assets: *const Assets,
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const scene = if (scene_opt.*) |*s| s else {
        txn.respond(.{ .ErrorCode = .ErrSceneEntityNotExist });
        return;
    };
    const index = scene.net_id_map.get(txn.message.EntityId) orelse {
        txn.respond(.{ .ErrorCode = .ErrSceneEntityNotExist });
        return;
    };
    const config_id = scene.entities.items(.config)[index].config_id;
    if (config_id <= 0) {
        txn.respond(.{ .ErrorCode = .ErrSceneEntityNotExist });
        return;
    }
    if (sceneItemMovementDisabled(assets, scene, config_id)) {
        txn.respond(.{ .ErrorCode = .ErrSceneItemOperate });
        return;
    }

    const status = txn.message.MoveStatus orelse .Moving;
    const spline_active = status == .Moving or status == .Switch;
    if (spline_active and txn.message.SplineEntityId <= 0) {
        txn.respond(.{ .ErrorCode = .ErrSplineEntityIdNotExist });
        return;
    }
    if (txn.message.RuntimeData) |runtime| {
        if (runtime.CurPos) |pos| {
            if (!finiteVec3(.{ pos.X, pos.Y, pos.Z })) {
                txn.respond(.{ .ErrorCode = .ErrSceneItemState });
                return;
            }
        }
        if (runtime.Distance) |distance| switch (distance) {
            .DistanceAlongPath => |value| if (!std.math.isFinite(value)) {
                txn.respond(.{ .ErrorCode = .ErrSceneItemState });
                return;
            },
        };
        if (runtime.Rot) |rotation| switch (rotation) {
            .CurRot => |value_opt| {
                if (value_opt) |value| {
                    if (!finiteVec3(.{ value.Roll, value.Pitch, value.Yaw })) {
                        txn.respond(.{ .ErrorCode = .ErrSceneItemState });
                        return;
                    }
                }
            },
        };
    }

    scene.entities.items(.world_item)[index].move_spline = if (spline_active) .{
        .move_status = status,
        .spline_entity_id = txn.message.SplineEntityId,
        .runtime_data = txn.message.RuntimeData,
        .move_spline_config = txn.message.MoveSplineConfig,
    } else null;

    if (txn.message.RuntimeData) |runtime| {
        if (runtime.CurPos) |position| {
            var rotation: ?pb.Rotator = null;
            if (runtime.Rot) |rot| switch (rot) {
                .CurRot => |value| rotation = value,
            };
            _ = try persistWorldEntityTransform(
                scene,
                fs,
                alloc,
                progress_comp,
                txn.message.EntityId,
                .{ position.X, position.Y, position.Z },
                rotation,
            );
        }
    }

    const spline_key = try std.fmt.allocPrint(alloc.arena, "scene_item_spline_cfg:{d}", .{config_id});
    const status_key = try std.fmt.allocPrint(alloc.arena, "scene_item_spline_status:{d}", .{config_id});
    const active_key = try std.fmt.allocPrint(alloc.arena, "scene_item_spline_active:{d}", .{config_id});
    const quest_active_key = try std.fmt.allocPrint(alloc.arena, "quest_spline_active:{d}", .{config_id});
    const vehicle_active_key = try std.fmt.allocPrint(alloc.arena, "vehicle:spline_active:{d}", .{config_id});
    const target_active_key = try std.fmt.allocPrint(alloc.arena, "scene_item_move_target_active:{d}", .{config_id});
    try progress_comp.info.setKeyInt(alloc.gpa, spline_key, txn.message.SplineEntityId);
    try progress_comp.info.setKeyInt(alloc.gpa, status_key, @intFromEnum(status));
    try progress_comp.info.setKeyInt(alloc.gpa, active_key, @intFromBool(spline_active));
    if (txn.message.MoveSplineConfig) |move_spline_config| {
        try spline_state.persistMoveSplineConfig(
            alloc.gpa,
            alloc.arena,
            &progress_comp.info,
            config_id,
            move_spline_config,
        );
    }
    try progress_comp.info.setKeyInt(alloc.gpa, quest_active_key, 0);
    try progress_comp.info.setKeyInt(alloc.gpa, vehicle_active_key, 0);
    try progress_comp.info.setKeyInt(alloc.gpa, target_active_key, 0);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSceneItemMoveTargetRequest(
    txn: *Transaction(pb.SceneItemMoveTargetRequest),
    scene_opt: *?Scene,
    assets: *const Assets,
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const scene = if (scene_opt.*) |*s| s else {
        txn.respond(.{ .ErrorCode = .ErrSceneEntityNotExist });
        return;
    };
    const mi = txn.message.MoveInfo orelse {
        txn.respond(.{ .ErrorCode = .ErrSceneItemState });
        return;
    };
    const loc = mi.Location orelse {
        txn.respond(.{ .ErrorCode = .ErrSceneItemState });
        return;
    };
    const target_pos = [3]f32{ loc.X, loc.Y, loc.Z };
    if (!finiteVec3(target_pos) or !std.math.isFinite(mi.MoveTime) or !std.math.isFinite(mi.StayTime) or
        !std.math.isFinite(mi.MaxSpeed) or !std.math.isFinite(mi.Acceleration))
    {
        txn.respond(.{ .ErrorCode = .ErrSceneItemState });
        return;
    }
    const index = scene.net_id_map.get(mi.EntityId) orelse {
        txn.respond(.{ .ErrorCode = .ErrSceneEntityNotExist });
        return;
    };
    const config_id = scene.entities.items(.config)[index].config_id;
    if (config_id <= 0) {
        txn.respond(.{ .ErrorCode = .ErrSceneEntityNotExist });
        return;
    }
    if (sceneItemMovementDisabled(assets, scene, config_id)) {
        txn.respond(.{ .ErrorCode = .ErrSceneItemOperate });
        return;
    }

    scene.entities.items(.world_item)[index].move_to_point = .{
        .target_pos = target_pos,
        .move_time = mi.MoveTime,
        .stay_time = mi.StayTime,
        .max_speed = mi.MaxSpeed,
        .acceleration = mi.Acceleration,
    };
    const fields = [_][]const u8{ "x", "y", "z", "move_time", "stay_time", "max_speed", "acceleration" };
    const values = [_]f32{ target_pos[0], target_pos[1], target_pos[2], mi.MoveTime, mi.StayTime, mi.MaxSpeed, mi.Acceleration };
    for (fields, values) |field, value| {
        const key = try std.fmt.allocPrint(alloc.arena, "scene_item_move_target:{d}:{s}", .{ config_id, field });
        try progress_comp.info.setKeyInt(alloc.gpa, key, @intCast(@as(u32, @bitCast(value))));
    }
    const active_key = try std.fmt.allocPrint(alloc.arena, "scene_item_move_target_active:{d}", .{config_id});
    try progress_comp.info.setKeyInt(alloc.gpa, active_key, 1);
    try progress_comp.save(fs, alloc.arena);
    // The client queues this target before sending the request. Echoing the
    // notify to the same client would enqueue the same waypoint a second time.
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSceneItemSequenceFrameStartRequest(
    txn: *Transaction(pb.SceneItemSequenceFrameStartRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistClaimKey(alloc, fs, pc, "seq_frame:start", txn.message.EntityId);
    txn.respond(.{ .Code = .Success });
}

pub fn onSceneItemSequenceFrameEndRequest(
    txn: *Transaction(pb.SceneItemSequenceFrameEndRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistClaimKey(alloc, fs, pc, "seq_frame:end", txn.message.EntityId);
    txn.respond(.{ .Code = .Success });
}

pub fn onSceneItemStopRequest(
    txn: *Transaction(pb.SceneItemStopRequest),
    scene_opt: *?Scene,
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var changed = false;
    if (scene_opt.*) |*scene| {
        for (txn.message.StopInfos.items) |si| {
            if (scene.net_id_map.get(si.EntityId)) |index| {
                const config_id = scene.entities.items(.config)[index].config_id;
                scene.entities.items(.world_item)[index].move_spline = null;
                scene.entities.items(.world_item)[index].move_to_point = null;
                if (config_id > 0) {
                    const spline_active_key = try std.fmt.allocPrint(alloc.arena, "scene_item_spline_active:{d}", .{config_id});
                    const spline_status_key = try std.fmt.allocPrint(alloc.arena, "scene_item_spline_status:{d}", .{config_id});
                    const target_active_key = try std.fmt.allocPrint(alloc.arena, "scene_item_move_target_active:{d}", .{config_id});
                    const quest_active_key = try std.fmt.allocPrint(alloc.arena, "quest_spline_active:{d}", .{config_id});
                    const vehicle_active_key = try std.fmt.allocPrint(alloc.arena, "vehicle:spline_active:{d}", .{config_id});
                    try progress_comp.info.setKeyInt(alloc.gpa, spline_active_key, 0);
                    try progress_comp.info.setKeyInt(alloc.gpa, spline_status_key, @intFromEnum(pb.MoveStatus.Stop));
                    try progress_comp.info.setKeyInt(alloc.gpa, target_active_key, 0);
                    try progress_comp.info.setKeyInt(alloc.gpa, quest_active_key, 0);
                    try progress_comp.info.setKeyInt(alloc.gpa, vehicle_active_key, 0);
                    changed = true;
                }
            }
            if (si.Location) |loc| {
                changed = try persistWorldEntityTransform(
                    scene,
                    fs,
                    alloc,
                    progress_comp,
                    si.EntityId,
                    .{ loc.X, loc.Y, loc.Z },
                    null,
                ) or changed;
            }
        }
    }
    if (changed) try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onMoveSceneItemResetPositionRequest(
    txn: *Transaction(pb.MoveSceneItemResetPositionRequest),
    scene_opt: *?Scene,
    assets: *const Assets,
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    if (scene_opt.*) |*scene| {
        if (scene.net_id_map.get(txn.message.EntityId)) |idx| {
            const cfg_id = scene.entities.items(.config)[idx].config_id;
            scene.entities.items(.world_item)[idx].move_spline = null;
            scene.entities.items(.world_item)[idx].move_to_point = null;
            if (cfg_id > 0) {
                const spline_active_key = try std.fmt.allocPrint(alloc.arena, "scene_item_spline_active:{d}", .{cfg_id});
                const spline_status_key = try std.fmt.allocPrint(alloc.arena, "scene_item_spline_status:{d}", .{cfg_id});
                const target_active_key = try std.fmt.allocPrint(alloc.arena, "scene_item_move_target_active:{d}", .{cfg_id});
                const quest_active_key = try std.fmt.allocPrint(alloc.arena, "quest_spline_active:{d}", .{cfg_id});
                const vehicle_active_key = try std.fmt.allocPrint(alloc.arena, "vehicle:spline_active:{d}", .{cfg_id});
                try pc.info.setKeyInt(alloc.gpa, spline_active_key, 0);
                try pc.info.setKeyInt(alloc.gpa, spline_status_key, @intFromEnum(pb.MoveStatus.Stop));
                try pc.info.setKeyInt(alloc.gpa, target_active_key, 0);
                try pc.info.setKeyInt(alloc.gpa, quest_active_key, 0);
                try pc.info.setKeyInt(alloc.gpa, vehicle_active_key, 0);
            }
            if (assets.findLevelEntity(scene.instance_id, cfg_id)) |ecfg| {
                if (Assets.AssetIndexes.entityWorldPos(ecfg)) |pos| {
                    if (try persistWorldEntityTransform(scene, fs, alloc, pc, txn.message.EntityId, pos, null)) {
                        try pc.save(fs, alloc.arena);
                    }
                }
            }
        }
    }
    try persistClaimKey(alloc, fs, pc, "item_reset", txn.message.EntityId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onMovePlacementRequest(
    txn: *Transaction(pb.MovePlacementRequest),
    scene_opt: *?Scene,
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    if (scene_opt.*) |*scene| {
        if (txn.message.PosRot) |pr| {
            if (pr.Pos) |loc| {
                const changed = try persistWorldEntityTransform(
                    scene,
                    fs,
                    alloc,
                    pc,
                    txn.message.PlacementEntityId,
                    .{ loc.X, loc.Y, loc.Z },
                    pr.Rot,
                );
                if (changed) try pc.save(fs, alloc.arena);
            }
        }
    }
    try persistIntKey(alloc, fs, pc, "placement", txn.message.PlacementEntityId, txn.message.TargetBoardEntityId);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onSplineMoveRequest(
    txn: *Transaction(pb.SplineMoveRequest),
    scene_opt: *?Scene,
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    if (scene_opt.*) |*scene| {
        if (txn.message.Pos) |loc| {
            if (try persistWorldEntityTransform(
                scene,
                fs,
                alloc,
                pc,
                txn.message.EntityId,
                .{ loc.X, loc.Y, loc.Z },
                null,
            )) try pc.save(fs, alloc.arena);
        }
    }
    try persistIntKey(alloc, fs, pc, "spline:path", txn.message.EntityId, txn.message.PathId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSplinePositionUpdateRequest(
    txn: *Transaction(pb.SplinePositionUpdateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistIntKey(alloc, fs, pc, "spline:idx", txn.message.EntityId, txn.message.SplineIdx);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onStopMoveSplineRequest(
    txn: *Transaction(pb.StopMoveSplineRequest),
    scene_opt: *?Scene,
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    const stop_key = try std.fmt.allocPrint(alloc.arena, "spline:stop:{d}", .{txn.message.EntityId});
    try pc.info.setKeyInt(alloc.gpa, stop_key, txn.message.SplineEntityId);

    if (scene_opt.*) |*scene| {
        if (scene.net_id_map.get(txn.message.EntityId)) |index| {
            const config_id = scene.entities.items(.config)[index].config_id;
            if (config_id > 0) {
                const vehicle_spline_key = try std.fmt.allocPrint(alloc.arena, "vehicle:spline:{d}", .{config_id});
                const quest_spline_key = try std.fmt.allocPrint(alloc.arena, "quest_spline:{d}", .{config_id});
                const item_spline_key = try std.fmt.allocPrint(alloc.arena, "scene_item_spline_cfg:{d}", .{config_id});
                const active_matches =
                    ((pc.info.getKeyInt(try std.fmt.allocPrint(alloc.arena, "vehicle:spline_active:{d}", .{config_id})) == 1) and pc.info.getKeyInt(vehicle_spline_key) == txn.message.SplineEntityId) or
                    ((pc.info.getKeyInt(try std.fmt.allocPrint(alloc.arena, "quest_spline_active:{d}", .{config_id})) == 1) and pc.info.getKeyInt(quest_spline_key) == txn.message.SplineEntityId) or
                    ((pc.info.getKeyInt(try std.fmt.allocPrint(alloc.arena, "scene_item_spline_active:{d}", .{config_id})) == 1) and pc.info.getKeyInt(item_spline_key) == txn.message.SplineEntityId);
                if (active_matches) {
                    const keys = [_][]const u8{
                        try std.fmt.allocPrint(alloc.arena, "vehicle:spline_active:{d}", .{config_id}),
                        try std.fmt.allocPrint(alloc.arena, "quest_spline_active:{d}", .{config_id}),
                        try std.fmt.allocPrint(alloc.arena, "scene_item_spline_active:{d}", .{config_id}),
                        try std.fmt.allocPrint(alloc.arena, "scene_item_move_target_active:{d}", .{config_id}),
                    };
                    for (keys) |key| try pc.info.setKeyInt(alloc.gpa, key, 0);
                    const status_key = try std.fmt.allocPrint(alloc.arena, "scene_item_spline_status:{d}", .{config_id});
                    try pc.info.setKeyInt(alloc.gpa, status_key, @intFromEnum(pb.MoveStatus.Stop));
                    scene.entities.items(.world_item)[index].move_spline = null;
                    scene.entities.items(.world_item)[index].move_to_point = null;
                }
            }
        }
    }
    try pc.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onGravityFlipInteractRequest(
    txn: *Transaction(pb.GravityFlipInteractRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    scene_opt: *?Scene,
    io: std.Io,
) !void {
    const target = txn.message.TargetGravityFlipType orelse {
        txn.respond(.{ .ErrorCode = .GravityFlipTypeNotExist });
        return;
    };
    const config_id = std.math.cast(i32, txn.message.GravityFlipEntityId) orelse {
        txn.respond(.{ .ErrorCode = .ErrSceneEntityNotExist });
        return;
    };
    const scene = if (scene_opt.*) |*s| s else {
        txn.respond(.{ .ErrorCode = .ErrSceneEntityNotExist });
        return;
    };
    if (config_id <= 0) {
        txn.respond(.{ .ErrorCode = .ErrSceneEntityNotExist });
        return;
    }
    const net_id = scene.findEntityNetIdByConfigId(config_id) orelse {
        txn.respond(.{ .ErrorCode = .ErrSceneEntityNotExist });
        return;
    };

    const cur_val: i64 = @intFromEnum(target);
    if (scene.net_id_map.get(net_id)) |idx| {
        scene.entities.items(.world_item)[idx].gravity_flip = @intCast(cur_val);
    }
    const key = try std.fmt.allocPrint(alloc.arena, "gravity:{d}", .{config_id});
    try progress_comp.info.setKeyInt(alloc.gpa, key, cur_val);
    try progress_comp.save(fs, alloc.arena);
    txn.conn.pushBestEffort(alloc.gpa, io, pb.GravityFlipUpdateTypeNotify{
        .NewGravityFlipType = target,
        .EntityId = config_id,
    });
    txn.respond(.{
        .CurGravityFlipType = target,
        .ErrorCode = .Success,
    });
}

pub fn onGravityHookLockPointChangeGravityRequest(
    txn: *Transaction(pb.GravityHookLockPointChangeGravityRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistClaimKey(alloc, fs, pc, "gravity_hook:chg", txn.message.EntityId);
    txn.respond(.{ .ErrorCode = .Success });
}

const HookInteractConfig = @import("../../data/tables/entity_components/Extra.zig").HookInteractConfig;

fn resolveHookInteractConfig(
    assets: *const Assets,
    map_id: i32,
    cfg_id: i64,
) ?HookInteractConfig {
    const level_cfg = assets.findLevelEntity(map_id, cfg_id) orelse return null;
    for (level_cfg.ComponentsData.extra) |entry| {
        if (entry == .HookLockPoint) {
            if (entry.HookLockPoint.HookInteractConfig) |cfg| return cfg;
        }
    }
    if (assets.findTemplateConfig(level_cfg.BlueprintType)) |tpl_cfg| {
        for (tpl_cfg.ComponentsData.extra) |entry| {
            if (entry == .HookLockPoint) {
                if (entry.HookLockPoint.HookInteractConfig) |cfg| return cfg;
            }
        }
    }
    return null;
}

fn handleHookLockPointAction(
    qctx: QuestEngine.QuestContext,
    entity_id: i64,
    action_type: pb.HookInteractActionType,
) !void {
    const scene = qctx.scene orelse return;
    const idx = scene.net_id_map.get(entity_id) orelse return;
    const cfg_id = scene.entities.items(.config)[idx].config_id;
    if (cfg_id <= 0) return;

    const hook_cfg = resolveHookInteractConfig(qctx.assets, scene.instance_id, cfg_id) orelse return;
    const actions = switch (action_type) {
        .Hooked => hook_cfg.HookActions,
        .ExitEndpoint => hook_cfg.FinishActions,
        .ExitMidway => hook_cfg.ExitHookActions,
    } orelse return;
    if (actions.len == 0) return;

    try QuestEngine.executeEntityActions(qctx, entity_id, cfg_id, actions);
    const end_idx: i32 = @intCast(actions.len - 1);
    const total_cnt: i32 = @intCast(actions.len);
    try qctx.conn.push(qctx.alloc.gpa, qctx.io, pb.ActionNotify{
        .GameCtx = .{
            .CtxType = .HookLockPointActionCtx,
            .CtxInfo = .{ .HookLockPointAction = .{
                .EntityCtx = .{ .ConfigId = cfg_id, .IncId = entity_id },
                .InteractionType = action_type,
            } },
        },
        .TotalCount = total_cnt,
        .PlayerId = scene.player_id,
        .StartIndex = 0,
        .EndIndex = end_idx,
        .NeedFinishReq = false,
        .IncId = scene.allocActionIncId(),
    });
}

pub fn onHookLockPointRequest(
    txn: *Transaction(pb.HookLockPointRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    try persistIntKey(alloc, fs, pc, "hook_point:active", txn.message.EntityId, 1);
    try handleHookLockPointAction(qctx, txn.message.EntityId, .Hooked);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onHookLockPointFinishRequest(
    txn: *Transaction(pb.HookLockPointFinishRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    try persistClaimKey(alloc, fs, pc, "hook_point:fin", txn.message.EntityId);
    try handleHookLockPointAction(qctx, txn.message.EntityId, .ExitEndpoint);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onHookLockPointExitRequest(
    txn: *Transaction(pb.HookLockPointExitRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    try persistIntKey(alloc, fs, pc, "hook_point:active", txn.message.EntityId, 0);
    try handleHookLockPointAction(qctx, txn.message.EntityId, .ExitMidway);
    txn.respond(.{ .ErrorCode = .Success });
}

fn resolveAndGrantEntityCollectOrCapture(
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    scene_opt: *?Scene,
    reward_ctx: RewardService.RewardContext,
    is_capture: bool,
    net_id: i64,
    out_removes: *std.ArrayList(pb.EntityRemoveInfo),
) !i32 {
    var cfg_id: i64 = @mod(net_id, 2_000_000_000);
    var map_id: i32 = 8;
    var vision_id: i32 = 0;
    if (scene_opt.*) |*scene| {
        map_id = scene.instance_id;
        if (scene.net_id_map.get(net_id)) |idx| {
            if (scene.entities.items(.world_item)[idx].monster_capture) |mc| {
                if (mc.entity_id > 0) cfg_id = mc.entity_id;
                if (mc.monster_id > 0) vision_id = mc.monster_id;
            } else {
                cfg_id = scene.entities.items(.config)[idx].config_id;
            }
        } else if (scene.findConfigIdByNetId(net_id)) |cid| {
            cfg_id = cid;
        }
    }

    var reward_id: i32 = 0;
    var tb_type: i32 = 0;
    if (LevelPlayHelper.resolveEntityConfig(assets, map_id, cfg_id)) |resolved| {
        if (resolved.components.RewardComponent) |rc| reward_id = rc.RewardId orelse 0;
        if (vision_id <= 0) {
            if (resolved.components.VisionCaptureComponent) |vc| vision_id = vc.VisionCaptureId orelse 0;
        }
        if (resolved.components.TreasureBoxComponent) |tb| tb_type = tb.TypeId orelse 0;
    }

    if (is_capture) {
        const calabash_lvl = @import("../../logic/helpers/condition.zig").currentCalabashLevel(
            assets,
            reward_ctx.basic_comp,
            reward_ctx.echo_comp,
        );
        _ = try RewardService.grantMonsterDropWithEchoOption(reward_ctx, 0, vision_id, true, calabash_lvl, true);
    } else if (tb_type > 0) {
        _ = try RewardService.grantTreasureBoxReward(reward_ctx, tb_type, reward_id, 1);
    } else if (reward_id > 0) {
        const res = try RewardService.grantDropPackage(reward_ctx, reward_id, 1);
        if (res.granted_count == 0) {
            _ = try RewardService.grantItems(reward_ctx, 0, true, &.{.{ .item_id = 2, .count = 150 }}, 1);
        }
    } else {
        _ = try RewardService.grantItems(reward_ctx, 0, true, &.{.{ .item_id = 2, .count = 150 }}, 1);
    }

    if (scene_opt.*) |*scene| {
        if (scene.net_id_map.contains(net_id)) {
            try scene.remove(alloc.gpa, fs, net_id);
            try out_removes.append(alloc.arena, .{ .EntityId = net_id });
        }
    }
    return @intCast(std.math.clamp(cfg_id, 0, std.math.maxInt(i32)));
}

pub fn onHookLockBatchCollectRequest(
    txn: *Transaction(pb.HookLockBatchCollectRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    assets: *const Assets,
    scene_opt: *?Scene,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var success_cfgs: std.ArrayList(i32) = .empty;
    var remove_infos: std.ArrayList(pb.EntityRemoveInfo) = .empty;
    var dirty = false;

    if (txn.message.BatchIds.items.len > 0) {
        for (txn.message.BatchIds.items) |bid| {
            const key = try std.fmt.allocPrint(alloc.arena, "hook_collect:{d}", .{bid});
            if (progress_comp.info.hasClaim(key)) continue;
            const cfg32 = try resolveAndGrantEntityCollectOrCapture(alloc, fs, assets, scene_opt, reward_ctx, false, bid, &remove_infos);
            _ = try progress_comp.info.markClaim(alloc.gpa, key);
            dirty = true;
            try success_cfgs.append(alloc.arena, cfg32);
        }
    } else if (txn.message.EntityId > 0) {
        const key = try std.fmt.allocPrint(alloc.arena, "hook_collect:{d}", .{txn.message.EntityId});
        if (!progress_comp.info.hasClaim(key)) {
            const cfg32 = try resolveAndGrantEntityCollectOrCapture(alloc, fs, assets, scene_opt, reward_ctx, false, txn.message.EntityId, &remove_infos);
            _ = try progress_comp.info.markClaim(alloc.gpa, key);
            dirty = true;
            try success_cfgs.append(alloc.arena, cfg32);
        }
    }
    if (dirty) try progress_comp.save(fs, alloc.arena);

    if (remove_infos.items.len > 0) {
        txn.conn.pushBestEffort(alloc.gpa, io, pb.EntityRemoveNotify{
            .IsRemove = true,
            .RemoveInfos = remove_infos,
        });
    }

    const main_cfg: i32 = if (scene_opt.*) |*scene| blk: {
        if (scene.findConfigIdByNetId(txn.message.EntityId)) |cid| {
            break :blk @intCast(std.math.clamp(cid, 0, std.math.maxInt(i32)));
        }
        break :blk @intCast(@mod(txn.message.EntityId, 2_000_000_000));
    } else @intCast(@mod(txn.message.EntityId, 2_000_000_000));

    txn.respond(.{
        .ErrorCode = .Success,
        .EntityConfigId = main_cfg,
        .SuccessConfigIds = success_cfgs,
    });
}

pub fn onConnectorCanConnectRequest(
    txn: *Transaction(pb.ConnectorCanConnectRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistIntKey(alloc, fs, pc, "connector:can", txn.message.EntityId, if (txn.message.IsEnter) 1 else 0);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onConnectorConnectEntityRequest(
    txn: *Transaction(pb.ConnectorConnectEntityRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    if (txn.message.ConnectEntityInfo) |ci| {
        try persistClaimKey(alloc, fs, pc, "connector:link", ci.SrcEntityId);
        var infos: std.ArrayList(pb.ConnectEntityInfo) = .empty;
        try infos.append(alloc.arena, ci);
        txn.conn.pushBestEffort(alloc.gpa, io, pb.ConnectorConnectEntityNotify{
            .ConnectEntityInfos = infos,
        });
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onBeControlledRequest(
    txn: *Transaction(pb.BeControlledRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistIntKey(alloc, fs, pc, "be_ctrl:active", txn.message.EntityId, if (txn.message.IsControl) 1 else 0);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onBeControlledShowRequest(
    txn: *Transaction(pb.BeControlledShowRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistIntKey(alloc, fs, pc, "be_ctrl:show", txn.message.EntityId, if (txn.message.IsShow) 1 else 0);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onBeControlledEntityHoverRequest(
    txn: *Transaction(pb.BeControlledEntityHoverRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistClaimKey(alloc, fs, pc, "be_ctrl:hover", txn.message.EntityId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onEntityTimeFlowPush(_: *Transaction(pb.EntityTimeFlowPush)) !void {}

pub fn onSceneModeChangeFinishRequest(
    txn: *Transaction(pb.SceneModeChangeFinishRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistClaimKey(alloc, fs, pc, "scene:mode_fin", 1);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onBeControlledThrowRequest(
    txn: *Transaction(pb.BeControlledThrowRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistIntKey(alloc, fs, pc, "be_ctrl:throw", txn.message.EntityId, txn.message.ThrowType);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onBeControlledDestroyRequest(
    txn: *Transaction(pb.BeControlledDestroyRequest),
    scene_opt: *?Scene,
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    if (scene_opt.*) |*scene| {
        var cpack: std.ArrayList(pb.CombatReceiveData) = .empty;
        if (try scene.setEntityVisibility(txn.message.EntityId, false, &cpack, alloc.arena)) {
            if (cpack.items.len > 0) {
                try txn.conn.push(alloc.gpa, io, pb.CombatReceivePackNotify{ .Data = cpack });
            }
        }
    }
    try persistClaimKey(alloc, fs, pc, "be_ctrl:destroy", txn.message.EntityId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onCombineEntitiesRequest(
    txn: *Transaction(pb.CombineEntitiesRequest),
    scene_opt: *?Scene,
    alloc: mem.Alloc,
) !void {
    if (scene_opt.*) |*scene| {
        if (txn.message.CombinePartInfo) |info| {
            if (info.OffsetPos) |pos| {
                if (info.OffsetRotate) |rot| {
                    _ = try scene.registerCombineRelation(alloc.gpa, info.CombinerEntityId, .{
                        .target_entity_id = txn.message.TargetEntity,
                        .part_index = info.PartId,
                        .position = pos,
                        .rotation = rot,
                    });
                }
            }
        }
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onDissolveCombineRelationRequest(
    txn: *Transaction(pb.DissolveCombineRelationRequest),
    alloc: mem.Alloc,
    scene_opt: *?Scene,
) !void {
    if (scene_opt.*) |*scene| {
        if (scene.detachCombineRelation(txn.message.EntityA, txn.message.EntityB) != null) {
            try scene.signalFsmDissolveCombine(alloc.gpa, txn.message.EntityA);
        } else if (scene.detachCombineRelation(txn.message.EntityB, txn.message.EntityA) != null) {
            try scene.signalFsmDissolveCombine(alloc.gpa, txn.message.EntityB);
        }
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onAttachEntityRequest(
    txn: *Transaction(pb.AttachEntityRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistClaimKey(alloc, fs, pc, "attach_ent", txn.message.EntityId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onHoldHandRequest(
    txn: *Transaction(pb.HoldHandRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistIntKey(alloc, fs, pc, "hold_hand", txn.message.EntityId, if (txn.message.IsHold) 1 else 0);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSitChairRequest(
    txn: *Transaction(pb.SitChairRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistIntKey(alloc, fs, pc, "sit_chair", txn.message.EntityId, if (txn.message.IsSitDown) 1 else 0);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSneakRequest(
    txn: *Transaction(pb.SneakRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistIntKey(alloc, fs, pc, "sneak:node", txn.message.NodeId, if (txn.message.IsStart) 1 else 0);
    txn.respond(.{ .ErrorId = .Success });
}

pub fn onSneakTimeRequest(
    txn: *Transaction(pb.SneakTimeRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistClaimKey(alloc, fs, pc, "sneak:time", txn.message.TreeIncId);
    txn.respond(.{ .ErrorId = .Success });
}

pub fn onTimelineTraceStartRequest(
    txn: *Transaction(pb.TimelineTraceStartRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    scene_opt: *?Scene,
    progress_comp: *PlayerProgressComponent,
) !void {
    var entity_ids: std.ArrayList(i64) = .empty;
    var point_datas: std.ArrayList(pb.ControlPointData) = .empty;
    var cp_count: usize = 0;

    var cfg_id: i64 = txn.message.EntityId;
    var map_id: i32 = 8;
    if (scene_opt.*) |*scene| {
        map_id = scene.instance_id;
        if (scene.findConfigIdByNetId(txn.message.EntityId)) |cid| {
            cfg_id = cid;
        }
    }

    if (LevelPlayHelper.resolveEntityConfig(assets, map_id, cfg_id)) |resolved| {
        const maybe_ttc: ?*const @import("../../data/tables/entity_components/Extra.zig").TimelineTrackControlComponent = for (resolved.components.extra) |entry| {
            switch (entry) {
                .TimelineTrackControlComponent => |ttc| break ttc,
                else => {},
            }
        } else null;
        if (maybe_ttc) |ttc| {
            const idx: usize = @intCast(@max(0, txn.message.Index));
            if (idx < ttc.ControlGroups.len) {
                const group = ttc.ControlGroups[idx];
                for (group.ControlConfigs) |cfg| {
                    if (cfg.ControlPoints.len > cp_count) {
                        cp_count = cfg.ControlPoints.len;
                    }
                    if (cfg.EntityId) |target_cfg_id| {
                        if (target_cfg_id > 0) {
                            const target_net_id: i64 = if (scene_opt.*) |*scene| blk: {
                                if (std.math.cast(i32, target_cfg_id)) |cid32| {
                                    break :blk scene.findEntityNetIdByConfigId(cid32) orelse target_cfg_id;
                                }
                                break :blk target_cfg_id;
                            } else target_cfg_id;
                            try entity_ids.append(alloc.arena, target_net_id);
                        }
                    }
                }
            }
        }
    }

    if (entity_ids.items.len == 0 and txn.message.EntityId > 0) {
        try entity_ids.append(alloc.arena, txn.message.EntityId);
    }
    if (cp_count == 0) cp_count = 2;

    for (0..cp_count) |i| {
        try point_datas.append(alloc.arena, .{
            .ControlPointIndex = @intCast(i),
            .LeftEnable = i > 0,
            .RightEnable = i + 1 < cp_count,
        });
    }

    const cp_key = try std.fmt.allocPrint(alloc.arena, "timeline_cp:{d}", .{txn.message.EntityId});
    const max_key = try std.fmt.allocPrint(alloc.arena, "timeline_cp_max:{d}", .{txn.message.EntityId});
    const max_cp: i64 = @intCast(cp_count - 1);
    try progress_comp.info.setKeyInt(alloc.gpa, max_key, max_cp);
    const cur_cp: i64 = std.math.clamp(progress_comp.info.getKeyInt(cp_key), 0, max_cp);
    try progress_comp.info.setKeyInt(alloc.gpa, cp_key, cur_cp);
    try progress_comp.save(fs, alloc.arena);

    txn.respond(.{
        .Index = txn.message.Index,
        .ControlPoint = @intCast(cur_cp),
        .EntityIds = entity_ids,
        .PointDatas = point_datas,
        .Code = .Success,
    });
}

pub fn onTimelineTraceControlRequest(
    txn: *Transaction(pb.TimelineTraceControlRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "timeline_cp:{d}", .{txn.message.EntityId});
    const max_key = try std.fmt.allocPrint(alloc.arena, "timeline_cp_max:{d}", .{txn.message.EntityId});
    const saved_max = progress_comp.info.getKeyInt(max_key);
    const max_cp: i64 = if (saved_max > 0) saved_max else 1;
    const cur: i64 = std.math.clamp(progress_comp.info.getKeyInt(key), 0, max_cp);
    const at_bound = (txn.message.Forward and cur >= max_cp) or (!txn.message.Forward and cur <= 0);
    if (at_bound) {
        txn.respond(.{
            .ControlPoint = @intCast(cur),
            .Code = .ErrTimelineMove,
        });
        return;
    }
    const next_cp: i64 = if (txn.message.Forward) cur + 1 else cur - 1;
    try progress_comp.info.setKeyInt(alloc.gpa, key, next_cp);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .ControlPoint = @intCast(next_cp),
        .Code = .Success,
    });
}

pub fn onTimelineTraceExitRequest(
    txn: *Transaction(pb.TimelineTraceExitRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistClaimKey(alloc, fs, pc, "timeline_exit", txn.message.EntityId);
    txn.respond(.{ .Code = .Success });
}

pub fn onRemoteBatchCollectRequest(
    txn: *Transaction(pb.RemoteBatchCollectRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    assets: *const Assets,
    scene_opt: *?Scene,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var collected: std.ArrayList(i64) = .empty;
    var remove_infos: std.ArrayList(pb.EntityRemoveInfo) = .empty;
    var dirty = false;
    for (txn.message.EntityIds.items) |eid| {
        const key = try std.fmt.allocPrint(alloc.arena, "remote_collect:{d}", .{eid});
        if (!progress_comp.info.hasClaim(key)) {
            _ = try resolveAndGrantEntityCollectOrCapture(alloc, fs, assets, scene_opt, reward_ctx, false, eid, &remove_infos);
            _ = try progress_comp.info.markClaim(alloc.gpa, key);
            dirty = true;
        }
        try collected.append(alloc.arena, eid);
    }
    if (dirty) try progress_comp.save(fs, alloc.arena);
    if (remove_infos.items.len > 0) {
        txn.conn.pushBestEffort(alloc.gpa, io, pb.EntityRemoveNotify{
            .IsRemove = true,
            .RemoveInfos = remove_infos,
        });
    }
    txn.respond(.{
        .EntityIds = collected,
        .ErrCode = .Success,
    });
}

pub fn onCaptureEntityRequest(
    txn: *Transaction(pb.CaptureEntityRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    assets: *const Assets,
    scene_opt: *?Scene,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "capture:{d}", .{txn.message.Id});
    var remove_infos: std.ArrayList(pb.EntityRemoveInfo) = .empty;
    if (!progress_comp.info.hasClaim(key)) {
        _ = try resolveAndGrantEntityCollectOrCapture(alloc, fs, assets, scene_opt, reward_ctx, true, txn.message.Id, &remove_infos);
        _ = try progress_comp.info.markClaim(alloc.gpa, key);
        try progress_comp.save(fs, alloc.arena);
    }
    if (remove_infos.items.len > 0) {
        txn.conn.pushBestEffort(alloc.gpa, io, pb.EntityRemoveNotify{
            .IsRemove = true,
            .RemoveInfos = remove_infos,
        });
    }
    txn.respond(.{
        .ErrCode = .Success,
        .Id = txn.message.Id,
    });
}

pub fn onCaptureBatchEntityRequest(
    txn: *Transaction(pb.CaptureBatchEntityRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    assets: *const Assets,
    scene_opt: *?Scene,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var ids: std.ArrayList(i64) = .empty;
    var remove_infos: std.ArrayList(pb.EntityRemoveInfo) = .empty;
    var dirty = false;
    for (txn.message.Ids.items) |eid| {
        const key = try std.fmt.allocPrint(alloc.arena, "capture:{d}", .{eid});
        if (!progress_comp.info.hasClaim(key)) {
            _ = try resolveAndGrantEntityCollectOrCapture(alloc, fs, assets, scene_opt, reward_ctx, true, eid, &remove_infos);
            _ = try progress_comp.info.markClaim(alloc.gpa, key);
            dirty = true;
        }
        try ids.append(alloc.arena, eid);
    }
    if (dirty) try progress_comp.save(fs, alloc.arena);
    if (remove_infos.items.len > 0) {
        txn.conn.pushBestEffort(alloc.gpa, io, pb.EntityRemoveNotify{
            .IsRemove = true,
            .RemoveInfos = remove_infos,
        });
    }
    txn.respond(.{
        .ErrCode = .Success,
        .Ids = ids,
    });
}

pub fn onPickUpDropItemRequest(
    txn: *Transaction(pb.PickUpDropItemRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    assets: *const Assets,
    scene_opt: *?Scene,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "drop_pickup:{d}", .{txn.message.DropEntityId});
    if (!progress_comp.info.hasClaim(key)) {
        var granted_any = false;
        if (txn.message.ItemGetFlow) |flow| {
            if (flow.ItemID > 0 and flow.ItemCount > 0) {
                const items = [_]RewardService.RewardItem{.{ .item_id = flow.ItemID, .count = flow.ItemCount }};
                const res = try RewardService.grantItems(reward_ctx, 0, true, &items, 1);
                if (res.granted_count > 0) granted_any = true;
            }
        }
        if (!granted_any) {
            if (scene_opt.*) |*scene| {
                const cfg_id = scene.findConfigIdByNetId(txn.message.DropEntityId) orelse txn.message.DropEntityId;
                if (LevelPlayHelper.resolveEntityConfig(assets, scene.instance_id, cfg_id)) |resolved| {
                    const reward_id = if (resolved.components.RewardComponent) |rc| (rc.RewardId orelse 0) else 0;
                    const tb_type = if (resolved.components.TreasureBoxComponent) |tb| (tb.TypeId orelse 0) else 0;
                    if (tb_type > 0) {
                        const res = try RewardService.grantTreasureBoxReward(reward_ctx, tb_type, reward_id, 1);
                        if (res.granted_count > 0) granted_any = true;
                    } else if (reward_id > 0) {
                        const res = try RewardService.grantDropPackage(reward_ctx, reward_id, 1);
                        if (res.granted_count > 0) granted_any = true;
                    }
                }
            }
        }
        if (!granted_any) {
            _ = try RewardService.grantItems(reward_ctx, 0, true, &.{.{ .item_id = 2, .count = 150 }}, 1);
        }

        if (scene_opt.*) |*scene| {
            if (scene.net_id_map.contains(txn.message.DropEntityId)) {
                var remove_infos: std.ArrayList(pb.EntityRemoveInfo) = .empty;
                try scene.remove(alloc.gpa, fs, txn.message.DropEntityId);
                try remove_infos.append(alloc.arena, .{ .EntityId = txn.message.DropEntityId });
                txn.conn.pushBestEffort(alloc.gpa, io, pb.EntityRemoveNotify{
                    .IsRemove = true,
                    .RemoveInfos = remove_infos,
                });
            }
        }
        _ = try progress_comp.info.markClaim(alloc.gpa, key);
        try progress_comp.save(fs, alloc.arena);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onPilotThrowOperationRequest(
    txn: *Transaction(pb.PilotThrowOperationRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    const op_val: i64 = if (txn.message.OpType) |t| @intFromEnum(t) else 0;
    try persistIntKey(alloc, fs, pc, "pilot_throw:op", 1, op_val);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onCommonOrganResetTimeRequest(
    txn: *Transaction(pb.CommonOrganResetTimeRequest),
    progress_comp: *const PlayerProgressComponent,
    alloc: mem.Alloc,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "organ_reset:{d}", .{txn.message.EntityId});
    txn.respond(.{
        .EntityId = txn.message.EntityId,
        .ErrorCode = .Success,
        .ResetTime = @intCast(@max(0, progress_comp.info.getKeyInt(key))),
    });
}

pub fn onSummon2Request(
    txn: *Transaction(pb.Summon2Request),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try persistIntKey(alloc, fs, progress_comp, "wp:summon2", txn.message.SummonerEntityId, txn.message.Version);
    txn.respond(.{
        .Version = txn.message.Version,
        .ErrorCode = .Success,
    });
}

pub fn onSummonRandomRequest(
    txn: *Transaction(pb.SummonRandomRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistClaimKey(alloc, fs, pc, "summon_rand", txn.message.SummonerId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onPlayerEnterPollutionRequest(
    txn: *Transaction(pb.PlayerEnterPollutionRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    const rty: i64 = if (txn.message.RangeType) |t| @intFromEnum(t) else 0;
    try persistIntKey(alloc, fs, pc, "pollution:mgr", txn.message.ManagerEntityId, rty);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onEnrichmentAreaRequest(
    txn: *Transaction(pb.EnrichmentAreaRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "enrich:{d}", .{txn.message.ItemId});
    _ = try progress_comp.info.markClaim(alloc.gpa, key);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onAddDecalRequest(
    txn: *Transaction(pb.AddDecalRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistIntKey(alloc, fs, pc, "decal:scene", txn.message.SceneId, txn.message.EffectId);
    txn.respond(.{ .ErrCode = .Success });
}

fn materialReplaceExchangeRatio(arena: std.mem.Allocator, io: std.Io, item_id: i32) i32 {
    const raw = std.Io.Dir.readFileAlloc(std.Io.Dir.cwd(), io, "assets/BinData/MaterialReplace.json", arena, std.Io.Limit.unlimited) catch return 1;
    const parsed = std.json.parseFromSlice(std.json.Value, arena, raw, .{}) catch return 1;
    const arr = switch (parsed.value) {
        .array => |a| a.items,
        else => return 1,
    };
    for (arr) |entry| {
        const obj = switch (entry) {
            .object => |o| o,
            else => continue,
        };
        const iid: i64 = switch (obj.get("ItemId") orelse continue) {
            .integer => |v| v,
            else => continue,
        };
        if (iid != item_id) continue;
        const param: i64 = switch (obj.get("ExchangeParam") orelse return 1) {
            .integer => |v| v,
            else => return 1,
        };
        return @intCast(@max(1, std.math.clamp(param, 1, std.math.maxInt(i32))));
    }
    return 1;
}

pub fn onBatchMaterialReplaceRequest(
    txn: *Transaction(pb.BatchMaterialReplaceRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    io: std.Io,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    assets: *const Assets,
) !void {
    const rctx: RewardService.RewardContext = .{
        .alloc = alloc,
        .io = io,
        .fs = fs,
        .assets = assets,
        .conn = txn.conn,
        .basic_comp = basic_comp,
        .inventory_comp = inventory_comp,
        .weapon_comp = weapon_comp,
        .echo_comp = echo_comp,
    };
    for (txn.message.ReplaceItems.items) |ri| {
        if (ri.TargetReplaceItemId <= 0 or ri.TargetNum <= 0) {
            txn.respond(.{ .Code = .RequestParamError });
            return;
        }
        var costs: std.ArrayList(ConsumeHelper.CostItem) = .empty;
        var total_consume: i64 = 0;
        for (ri.ConsumeItems.items) |ci| {
            if (ci.key > 0 and ci.value > 0) {
                try costs.append(alloc.arena, .{ .id = ci.key, .count = ci.value });
                total_consume += ci.value;
            }
        }
        if (costs.items.len == 0 or total_consume <= 0) {
            txn.respond(.{ .Code = .RequestParamError });
            return;
        }
        const ratio = materialReplaceExchangeRatio(alloc.arena, io, ri.TargetReplaceItemId);
        const max_grant: i32 = @intCast(std.math.clamp(@divTrunc(total_consume, ratio), 0, std.math.maxInt(i32)));
        const grant_num = @min(ri.TargetNum, max_grant);
        if (grant_num <= 0) {
            txn.respond(.{ .Code = .RequestParamError });
            return;
        }
        if (!(try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, costs.items, io))) {
            txn.respond(.{ .Code = .ErrItemNotEnough });
            return;
        }
        _ = try RewardService.grantItems(rctx, 0, true, &.{.{ .item_id = ri.TargetReplaceItemId, .count = grant_num }}, 1);
    }
    txn.respond(.{ .Code = .Success });
}

pub fn onMaterialReplaceRequest(
    txn: *Transaction(pb.MaterialReplaceRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    io: std.Io,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    assets: *const Assets,
) !void {
    if (txn.message.ConsumeItemId > 0 and txn.message.ConsumeItemNum > 0 and txn.message.TargetReplaceItemId > 0) {
        const costs = [_]ConsumeHelper.CostItem{.{ .id = txn.message.ConsumeItemId, .count = txn.message.ConsumeItemNum }};
        if (!(try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, &costs, io))) {
            txn.respond(.{ .Code = .ErrItemNotEnough });
            return;
        }
        const rctx: RewardService.RewardContext = .{
            .alloc = alloc,
            .io = io,
            .fs = fs,
            .assets = assets,
            .conn = txn.conn,
            .basic_comp = basic_comp,
            .inventory_comp = inventory_comp,
            .weapon_comp = weapon_comp,
            .echo_comp = echo_comp,
        };
        _ = try RewardService.grantItems(rctx, 0, true, &.{.{ .item_id = txn.message.TargetReplaceItemId, .count = txn.message.ConsumeItemNum }}, 1);
    }
    txn.respond(.{ .Code = .Success });
}

pub fn onFurnitureDiySaveRequest(
    txn: *Transaction(pb.FurnitureDiySaveRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "diy_atm:{d}:{d}", .{ txn.message.ActivityId, txn.message.AreaId });
    const score: i64 = @intCast(txn.message.FurnitureDiySlotInfos.items.len * 100);
    try progress_comp.info.setKeyInt(alloc.gpa, key, score);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .Atmosphere = @intCast(score),
        .ErrCode = .Success,
    });
}

pub fn onAcceptEntrustRequest(
    txn: *Transaction(pb.AcceptEntrustRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "entrust:{d}", .{txn.message.EntrustId});
    _ = try progress_comp.info.markClaim(alloc.gpa, key);
    var kbuf: [48]u8 = undefined;
    const tk_ent = try std.fmt.bufPrint(&kbuf, "trackmoon:entrust:{d}", .{txn.message.EntrustId});
    try progress_comp.info.setKeyInt(alloc.gpa, tk_ent, 1);

    const cfg = assets.tables.track_moon_entrust.getDataById(txn.message.EntrustId);
    const star: i32 = if (cfg) |c| @max(1, c.Star) else 1;
    const gold: i32 = star * 1000;
    const wish: i32 = star * 100;
    const heat: i32 = star * 10;

    _ = try progress_comp.info.addKeyInt(alloc.gpa, "trackmoon:entrust_done", 1);
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "trackmoon:gold", gold);
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "trackmoon:wish", wish);
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "trackmoon:popular", heat);

    var role_settle: std.ArrayList(pb.RoleProperSettle) = .empty;
    var role_goods: std.ArrayList(pb.RoleGood) = .empty;
    var first_role: i32 = 0;
    for (txn.message.RoleIds.items) |rid| {
        if (first_role == 0) first_role = rid;
        const lv_k = try std.fmt.bufPrint(&kbuf, "trackmoon:role_lvl:{d}", .{rid});
        const lvl: i32 = @intCast(@max(@as(i64, 1), progress_comp.info.getKeyInt(lv_k)));
        const a_k = try std.fmt.bufPrint(&kbuf, "trackmoon:role_a:{d}", .{rid});
        const b_k = try std.fmt.bufPrint(&kbuf, "trackmoon:role_b:{d}", .{rid});
        const c_k = try std.fmt.bufPrint(&kbuf, "trackmoon:role_c:{d}", .{rid});
        const bonus: i32 = star * 20;
        const pa: i32 = @as(i32, @intCast(@min(@as(i64, std.math.maxInt(i32) - bonus), @max(@as(i64, 100), progress_comp.info.getKeyInt(a_k))))) + bonus;
        const pb_v: i32 = @as(i32, @intCast(@min(@as(i64, std.math.maxInt(i32) - bonus), @max(@as(i64, 100), progress_comp.info.getKeyInt(b_k))))) + bonus;
        const pc_v: i32 = @as(i32, @intCast(@min(@as(i64, std.math.maxInt(i32) - bonus), @max(@as(i64, 100), progress_comp.info.getKeyInt(c_k))))) + bonus;
        try role_settle.append(alloc.arena, .{
            .RoleId = rid,
            .PropertyA = pa,
            .PropertyB = pb_v,
            .PropertyC = pc_v,
            .Level = lvl,
        });
        try role_goods.append(alloc.arena, .{
            .RoleId = rid,
            .Result = .Good,
        });
    }
    if (first_role > 0) {
        try progress_comp.info.setKeyInt(alloc.gpa, "trackmoon:fav_role", first_role);
    }
    try progress_comp.save(fs, alloc.arena);

    txn.respond(.{
        .AddEntrust = .{
            .EntrustId = txn.message.EntrustId,
            .IsVisiable = true,
            .HistoryLevel = 1,
        },
        .ErrorCode = .Success,
        .RoleProperSettle = role_settle,
        .TriggerIdeaRole = first_role,
        .Wish = wish,
        .Gold = gold,
        .Heat = heat,
        .TriggerIdea = first_role > 0,
        .RoleGoods = role_goods,
        .EvaluationLevel = 3,
    });
}

pub fn onUiGamePlayRequest(
    txn: *Transaction(pb.UiGamePlayRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "ui_gp:{s}", .{txn.message.GamePlayKey});
    _ = try progress_comp.info.markClaim(alloc.gpa, key);
    try progress_comp.save(fs, alloc.arena);
    QuestEngine.onUiGamePlayFinish(qctx, txn.message.GamePlayKey) catch |err| QuestEngine.logHookErr("onUiGamePlayFinish", err);
    txn.respond(.{ .ErrorId = .Success });
}

pub fn onWellStreamingFinisRequest(
    txn: *Transaction(pb.WellStreamingFinisRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistClaimKey(alloc, fs, pc, "well_stream:fin", txn.message.EntityIncId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onReadDisplayInfoRequest(
    txn: *Transaction(pb.ReadDisplayInfoRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "display_read:{d}", .{txn.message.DisplayId});
    _ = try progress_comp.info.markClaim(alloc.gpa, key);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onLoopTeleportVolumeActionRequest(
    txn: *Transaction(pb.LoopTeleportVolumeActionRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    pc: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    try persistClaimKey(alloc, fs, pc, "loop_tp:vol", txn.message.VolumeId);
    var met_ids: std.ArrayList(i32) = .empty;
    const cond_ctx = QuestEngine.makeEvalContext(qctx);
    const callbacks = QuestEngine.actionCallbacks();
    for (assets.tables.loop_teleport_volume_function.items) |row| {
        if (row.VolumeId != txn.message.VolumeId) continue;
        const ok = if (row.Conditions) |cond| ConditionEvaluator.evalInlineConditionBlock(cond_ctx, cond) else true;
        if (!ok) continue;
        try met_ids.append(alloc.arena, row.Id);
        if (row.Actions) |act_val| {
            if (act_val == .array) {
                QuestEngine.executeActionList(qctx, callbacks, 0, 0, act_val.array.items) catch |err| QuestEngine.logHookErr("executeActionList", err);
            }
        }
    }
    txn.respond(.{ .ErrorCode = .Success, .ConditionMetIds = met_ids });
}

pub fn onBtForcedOccupationRequest(
    txn: *Transaction(pb.BtForcedOccupationRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistClaimKey(alloc, fs, pc, "bt:forced_occ", txn.message.TreeIncId);
    txn.respond(.{ .ErrorId = .Success });
}

pub fn onActionOperationScreenSuccessRequest(
    txn: *Transaction(pb.ActionOperationScreenSuccessRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistClaimKey(alloc, fs, pc, "act:op_screen", txn.message.IncId);
    txn.respond(.{ .Code = .Success });
}

pub fn onActionResetPosPrePerformanceEndRequest(
    txn: *Transaction(pb.ActionResetPosPrePerformanceEndRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistIntKey(alloc, fs, pc, "act:reset_pos_pre", txn.message.IncId, txn.message.Index);
    txn.respond(.{ .Code = .Success });
}

pub fn onAccessPasserbyNpcSpawnerRequest(
    txn: *Transaction(pb.AccessPasserbyNpcSpawnerRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistIntKey(alloc, fs, pc, "passerby_spawner", txn.message.EntityId, txn.message.SplineConfigId);
    txn.respond(.{ .ErrorCode = .Success });
}

// =========================================================================
// Vehicles, Weather, Teleport, Dungeon, LevelPlay & Quest
// =========================================================================

pub fn onVehiclePlatformGetOnRequest(
    txn: *Transaction(pb.VehiclePlatformGetOnRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistIntKey(alloc, fs, pc, "vehicle:platform", txn.message.EntityId, if (txn.message.IsGetOn) 1 else 0);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onWeatherControlInfoRequest(
    txn: *Transaction(pb.WeatherControlInfoRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    var ids: std.ArrayList(i32) = .empty;
    const cur = progress_comp.info.getKeyInt("weather:switch_id");
    if (cur > 0) try ids.append(alloc.arena, @intCast(cur));
    txn.respond(.{
        .UnlockedSwitchIds = ids,
        .IncId = 0,
        .ErrorCode = .Success,
    });
}

pub fn onWeatherControlRequest(
    txn: *Transaction(pb.WeatherControlRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "weather:switch_id", txn.message.WeatherSwitchId);
    try progress_comp.save(fs, alloc.arena);
    const weather_id = if (txn.message.WeatherSwitchId > 0) txn.message.WeatherSwitchId else 1;
    try QuestEngine.changeWeather(qctx, weather_id);
    txn.respond(.{
        .ErrorCode = .Success,
        .WeatherId = weather_id,
    });
}

pub fn onWeatherSwitchInfoRequest(
    txn: *Transaction(pb.WeatherSwitchInfoRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
    quest_comp: *const PlayerQuestComponent,
) !void {
    var ids: std.ArrayList(i32) = .empty;
    for (quest_comp.info.env.unlocked_weather_switches) |wid| {
        try ids.append(alloc.arena, wid);
    }
    const cur = progress_comp.info.getKeyInt("weather:switch_id");
    if (cur > 0) {
        const cur_i32: i32 = @intCast(cur);
        if (!QuestInfo.containsInt(ids.items, cur_i32)) try ids.append(alloc.arena, cur_i32);
    }
    txn.respond(.{ .UnlockedSwitchIds = ids });
}

pub fn onSeamlessTeleportRequest(
    txn: *Transaction(pb.SeamlessTeleportRequest),
    qctx: QuestEngine.QuestContext,
) !void {
    if (txn.message.Location) |loc| {
        if (!finiteVec3(.{ loc.X, loc.Y, loc.Z })) {
            txn.respond(.{ .ErrorCode = .ErrTeleportPositionIllegal });
            return;
        }
        const rotation = if (txn.message.Rotation) |rot| [3]f32{ rot.Roll, rot.Pitch, rot.Yaw } else null;
        if (rotation) |value| {
            if (!finiteVec3(value)) {
                txn.respond(.{ .ErrorCode = .ErrTeleportPositionIllegal });
                return;
            }
        }
        try QuestEngine.updatePlayerSceneState(qctx, .{ loc.X, loc.Y, loc.Z }, rotation);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSeamlessTeleportVehicleRequest(
    txn: *Transaction(pb.SeamlessTeleportVehicleRequest),
    qctx: QuestEngine.QuestContext,
    progress_comp: *PlayerProgressComponent,
) !void {
    if (txn.message.Location) |loc| {
        const location = [3]f32{ loc.X, loc.Y, loc.Z };
        const rotation = txn.message.Rotation;
        const player_rotation = if (rotation) |rot| [3]f32{ rot.Roll, rot.Pitch, rot.Yaw } else null;
        if (!finiteVec3(location)) {
            txn.respond(.{ .ErrorCode = .ErrTeleportPositionIllegal });
            return;
        }
        if (player_rotation) |value| {
            if (!finiteVec3(value)) {
                txn.respond(.{ .ErrorCode = .ErrTeleportPositionIllegal });
                return;
            }
        }
        const scene = qctx.scene orelse {
            txn.respond(.{ .ErrorCode = .ErrSceneEntityNotExist });
            return;
        };
        const vehicle_net_id = if (scene.net_id_map.contains(txn.message.VehicleEntityId))
            txn.message.VehicleEntityId
        else if (std.math.cast(i32, txn.message.VehicleEntityId)) |config_id|
            scene.findEntityNetIdByConfigId(config_id) orelse 0
        else
            0;
        const vehicle_index = scene.net_id_map.get(vehicle_net_id) orelse {
            txn.respond(.{ .ErrorCode = .ErrSceneEntityNotExist });
            return;
        };
        if (scene.entities.items(.vehicle)[vehicle_index] == null) {
            txn.respond(.{ .ErrorCode = .ErrVehicleEntityTypeError });
            return;
        }
        if (!try persistWorldEntityTransform(
            scene,
            qctx.fs,
            qctx.alloc,
            progress_comp,
            vehicle_net_id,
            location,
            rotation,
        )) {
            txn.respond(.{ .ErrorCode = .ErrSceneEntityNotExist });
            return;
        }
        try progress_comp.save(qctx.fs, qctx.alloc.arena);
        try QuestEngine.updatePlayerSceneState(qctx, location, player_rotation);
        var samples: std.ArrayList(pb.MoveReplaySample) = .empty;
        try samples.append(qctx.alloc.arena, .{
            .Location = .{ .X = location[0], .Y = location[1], .Z = location[2] },
            .Rotation = rotation,
        });
        var entities: std.ArrayList(pb.MovingEntityData) = .empty;
        try entities.append(qctx.alloc.arena, .{
            .EntityId = vehicle_net_id,
            .Originator = vehicle_net_id,
            .MoveInfos = samples,
            .ForcePush = true,
        });
        qctx.conn.pushBestEffort(qctx.alloc.gpa, qctx.io, pb.MovePackageNotify{ .MovingEntities = entities });
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onPassPortalRequest(
    txn: *Transaction(pb.PassPortalRequest),
    scene_opt: *?Scene,
    qctx: QuestEngine.QuestContext,
) !void {
    if (txn.message.Location) |loc| {
        if (!finiteVec3(.{ loc.X, loc.Y, loc.Z })) {
            txn.respond(.{ .ErrorCode = .ErrTeleportPositionIllegal });
            return;
        }
        try QuestEngine.updatePlayerSceneState(qctx, .{ loc.X, loc.Y, loc.Z }, null);
    }
    txn.respond(.{ .ErrorCode = .Success });
    if (scene_opt.*) |scene| {
        if (scene.findConfigIdByNetId(txn.message.EntityId)) |config_id| {
            try QuestEngine.onPortalPassed(qctx, config_id);
        }
    }
}

pub fn onAwakeDynamicPortalRequest(
    txn: *Transaction(pb.AwakeDynamicPortalRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistIntKey(alloc, fs, pc, "dyn_portal:awake", txn.message.EntityId, if (txn.message.IsAwake) 1 else 0);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onInstEntityTeleportRequest(
    txn: *Transaction(pb.InstEntityTeleportRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistIntKey(alloc, fs, pc, "inst:ent_tp", txn.message.InstId, txn.message.EntityConfigId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onFlowTeleportToPlotPosRequest(
    txn: *Transaction(pb.FlowTeleportToPlotPosRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistClaimKey(alloc, fs, pc, "flow:tp_plot", txn.message.FlowIncId);
    txn.respond(.{ .Code = .Success });
}

pub fn onQuestFastReturnPosRequest(
    txn: *Transaction(pb.QuestFastReturnPosRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    if (txn.message.NodeId > 0) {
        _ = try QuestEngine.onSubmitNode(qctx, @intCast(@max(0, txn.message.TreeOwnerId)), txn.message.TreeIncId, txn.message.NodeId);
    }
    try persistIntKey(alloc, fs, pc, "quest:fast_ret", txn.message.TreeIncId, txn.message.NodeId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onFastReturnDungeonRequest(
    txn: *Transaction(pb.FastReturnDungeonRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try persistIntKey(alloc, fs, progress_comp, "wp:fast_ret_node", txn.message.TreeIncId, txn.message.NodeId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onUnOpenedAreaPullbackRequest(
    txn: *Transaction(pb.UnOpenedAreaPullbackRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try persistIntKey(alloc, fs, progress_comp, "wp:unopen_pullback", txn.message.TransferEffectId, 1);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onReviveRequest(
    txn: *Transaction(pb.ReviveRequest),
    scene_opt: *?Scene,
    role_comp: *PlayerRoleComponent,
    events: *EventQueue,
    fs: *FileSystem,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    if (scene_opt.*) |*scene| {
        try RoleHelper.reviveAndHealFormation(txn.conn, scene, role_comp, events, fs, alloc, io);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onLowMemoryChangeSceneFinishRequest(
    txn: *Transaction(pb.LowMemoryChangeSceneFinishRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    if (qctx.scene != null) QuestEngine.onSceneLoadingFinish(qctx) catch |err| QuestEngine.logHookErr("onSceneLoadingFinish", err);
    try persistClaimKey(alloc, fs, pc, "scene:low_mem_fin", 1);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onInstEntranceDetailRequest(
    txn: *Transaction(pb.InstEntranceDetailRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
    quest_comp: *const PlayerQuestComponent,
    progress_comp: *const PlayerProgressComponent,
) !void {
    var list: std.ArrayList(pb.InstDetailEntry) = .empty;
    var kbuf: [48]u8 = undefined;
    if (assets.tables.instance_dungeon_entrance.getDataById(txn.message.EntranceId)) |entrance| {
        for (entrance.InstanceDungeonList) |inst_id| {
            const done = QuestInfo.containsInt(quest_comp.info.completed_dungeons, inst_id);
            const reset_k = try std.fmt.bufPrint(&kbuf, "inst:reset:{d}", .{inst_id});
            const arch_k = try std.fmt.bufPrint(&kbuf, "inst:archive:{d}", .{inst_id});
            const arch_val = progress_comp.info.getKeyInt(arch_k);
            const arch: pb.ArchiveType = switch (arch_val) {
                1 => .NormalArchive,
                2 => .OverdueVersion,
                3 => .OverdueTime,
                4 => .OverdueChangeSex,
                else => .NoneArchive,
            };
            try list.append(alloc.arena, .{
                .InstId = inst_id,
                .ResetTime = @max(0, progress_comp.info.getKeyInt(reset_k)),
                .Count = if (done) 1 else 0,
                .ArchiveType = arch,
            });
            if (list.items.len >= 32) break;
        }
    } else {
        for (assets.tables.instance_dungeon.items) |inst| {
            const done = QuestInfo.containsInt(quest_comp.info.completed_dungeons, inst.Id);
            if (done or inst.Id == txn.message.EntranceId) {
                try list.append(alloc.arena, .{
                    .InstId = inst.Id,
                    .ResetTime = 0,
                    .Count = if (done) 1 else 0,
                    .ArchiveType = .NoneArchive,
                });
                if (list.items.len >= 32) break;
            }
        }
    }
    const lim_k = try std.fmt.bufPrint(&kbuf, "inst:limit_end:{d}", .{txn.message.EntranceId});
    txn.respond(.{
        .ErrorCode = .Success,
        .LimitEndTime = @max(0, progress_comp.info.getKeyInt(lim_k)),
        .EntranceId = txn.message.EntranceId,
        .InstDetailEntryList = list,
    });
}

pub fn onInviteRechallengeRequest(
    txn: *Transaction(pb.InviteRechallengeRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistIntKey(alloc, fs, pc, "inst:rechallenge_invite", 1, 1);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onReceiveRechallengeRequest(
    txn: *Transaction(pb.ReceiveRechallengeRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    const res_val: i64 = if (txn.message.Result) |r| @intFromEnum(r) else 0;
    try persistIntKey(alloc, fs, pc, "inst:rechallenge_res", 1, res_val);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onLevelSequenceFrameEventActionRequest(
    txn: *Transaction(pb.LevelSequenceFrameEventActionRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    try persistClaimKey(alloc, fs, pc, "level_seq:evt", txn.message.EntityId);
    txn.respond(.{ .ErrorCode = .Success });
    const entity_config_id = if (qctx.scene) |scene|
        scene.findConfigIdByNetId(txn.message.EntityId) orelse std.math.cast(i32, txn.message.EntityId) orelse 0
    else
        std.math.cast(i32, txn.message.EntityId) orelse 0;
    try QuestEngine.onLevelSequenceMarkReached(qctx, entity_config_id, txn.message.Key);
}

pub fn onLoadingLevelAreaRequest(
    txn: *Transaction(pb.LoadingLevelAreaRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
) !void {
    _ = txn.message;
    var configs: std.ArrayList(pb.LoadingAreaConfig) = .empty;
    for (assets.tables.loading_level_area.items) |row| {
        if (row.BeginTime > 0 or row.EndTime > 0 or row.IsLimitShow) {
            try configs.append(alloc.arena, .{
                .Id = row.Id,
                .BeginTime = row.BeginTime,
                .EndTime = row.EndTime,
            });
        }
    }
    txn.respond(.{ .LimitConfigs = configs });
}

pub fn onLstBattleRecordRequest(
    txn: *Transaction(pb.LstBattleRecordRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    _ = try pc.info.addKeyInt(alloc.gpa, "battle:record_req", 1);
    try pc.save(fs, alloc.arena);
    _ = txn.message;
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onQuestReadyForGiveUpRequest(
    txn: *Transaction(pb.QuestReadyForGiveUpRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try persistClaimKey(alloc, fs, pc, "quest:ready_giveup", txn.message.QuestId);
    txn.respond(.{ .ErrorCode = .Success });
}

// =========================================================================
// Formation, Roles, Shop, Cooking & Inventory
// =========================================================================

pub fn onUpdateFightRoleRequest(
    txn: *Transaction(pb.UpdateFightRoleRequest),
    scene: *Scene,
    role_comp: *const PlayerRoleComponent,
    fs: *FileSystem,
    events: *EventQueue,
    alloc: mem.Alloc,
) !void {
    if (scene.formation_info.cur_formation >= 0 and
        @as(usize, @intCast(scene.formation_info.cur_formation)) < scene.formation_info.formations.len)
    {
        const idx: usize = @intCast(scene.formation_info.cur_formation);
        var roles: [3]?@import("../../fs/FormationInfo.zig").Formation.Role = .{ null, null, null };
        var count: usize = 0;
        for (txn.message.RoleIds.items) |rid| {
            if (rid <= 0 or !role_comp.role_map.contains(rid) or count >= 3) continue;
            roles[count] = .{ .role_id = rid };
            count += 1;
        }
        if (count > 0) {
            scene.formation_info.formations[idx].roles = roles;
            scene.formation_info.formations[idx].cur_role = if (txn.message.CurRoleId != 0)
                txn.message.CurRoleId
            else if (roles[0]) |first|
                first.role_id
            else
                0;
            try scene.save(fs, alloc.gpa);
            try events.enqueue(.update_formations, .{});
        }
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onReorderFormationRequest(
    txn: *Transaction(pb.ReorderFormationRequest),
    scene: *Scene,
    fs: *FileSystem,
    events: *EventQueue,
    alloc: mem.Alloc,
) !void {
    if (scene.formation_info.formations.len == 0) {
        txn.respond(.{ .ErrorCode = .Success });
        return;
    }
    const max_idx: i32 = @intCast(scene.formation_info.formations.len - 1);
    const src_idx: usize = @intCast(std.math.clamp(txn.message.FormationId - 1, 0, max_idx));
    const dst_idx: usize = @intCast(std.math.clamp(txn.message.TargetId - 1, 0, max_idx));
    if (src_idx != dst_idx) {
        const tmp = scene.formation_info.formations[src_idx];
        scene.formation_info.formations[src_idx] = scene.formation_info.formations[dst_idx];
        scene.formation_info.formations[dst_idx] = tmp;
        try scene.save(fs, alloc.gpa);
        try events.enqueue(.update_formations, .{});
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSceneGroupDataAndCurFormationEntityRequest(
    txn: *Transaction(pb.SceneGroupDataAndCurFormationEntityRequest),
    scene: *Scene,
    assets: *const Assets,
    buff_timers: *BuffTimerScheduler,
    player_id: PlayerID,
    progress_comp: *const PlayerProgressComponent,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    var groups: std.ArrayList(pb.PlayerSceneGroupDataList) = .empty;
    var cur_role_entities: std.ArrayList(pb.EntityPb) = .empty;
    const now_ms = EnergyHelper.nowSeconds(io) * 1000;
    const saved_group = progress_comp.info.getKeyInt("scene_group:active");
    const cur_group: i32 = if (saved_group > 0) @intCast(saved_group) else 1;
    if (scene.formation_info.cur_formation >= 0 and
        @as(usize, @intCast(scene.formation_info.cur_formation)) < scene.formation_info.formations.len)
    {
        const form = scene.formation_info.formations[@intCast(scene.formation_info.cur_formation)];
        var role_infos: std.ArrayList(pb.SceneRoleInformation) = .empty;
        for (form.roles) |maybe_r| {
            const r = maybe_r orelse continue;
            try role_infos.append(alloc.arena, .{
                .RoleId = r.role_id,
                .EntityId = r.entity_id,
                .OnStageOutOfControl = r.on_stage_without_control,
            });
            if (r.entity_id > 0) {
                if (entity_proto.buildOptional(alloc, assets, scene, buff_timers, r.entity_id, now_ms)) |epb| {
                    try cur_role_entities.append(alloc.arena, epb);
                } else |_| {}
            }
        }
        var sg_list: std.ArrayList(pb.PlayerSceneGroupData) = .empty;
        try sg_list.append(alloc.arena, .{
            .Group = cur_group,
            .FightRoleInfos = role_infos,
            .CurRole = form.cur_role,
            .LivingStatus = RoleHelper.formationLivingStatus(scene, form),
        });
        try groups.append(alloc.arena, .{
            .PlayerId = player_id.id,
            .PlayerSceneGroupData = sg_list,
            .CurrentGroup = cur_group,
        });
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .PlayerSceneGroupDataList = groups,
        .CurFormationRoleEntitys = cur_role_entities,
    });
}

pub fn onUpdateSceneGroupDataRequest(
    txn: *Transaction(pb.UpdateSceneGroupDataRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    const grp: i64 = if (txn.message.TargetGroup) |g| @intFromEnum(g) else 0;
    try pc.info.setKeyInt(alloc.gpa, "scene_group:active", grp);
    for (txn.message.EntityIds.items, 0..) |eid, idx| {
        const k = try std.fmt.allocPrint(alloc.arena, "scene_group:{d}:{d}", .{ grp, idx });
        try pc.info.setKeyInt(alloc.gpa, k, eid);
    }
    try pc.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSwitchBattleModeRequest(
    txn: *Transaction(pb.SwitchBattleModeRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    try pc.info.setKeyInt(alloc.gpa, "battle:client_mode", if (txn.message.Client) 1 else 0);
    try pc.save(fs, alloc.arena);
    txn.respond(.{ .Client = txn.message.Client });
}

pub fn onResonantChainOptionalLimitRequest(txn: *Transaction(pb.ResonantChainOptionalLimitRequest)) !void {
    txn.respond(.{ .Limit = 10 });
}

pub fn onFoodProcessRequest(
    txn: *Transaction(pb.FoodProcessRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    reward_ctx: RewardService.RewardContext,
    io: std.Io,
) !void {
    const count = txn.message.CookCount;
    if (count <= 0) {
        txn.respond(.{ .Code = .ErrCookingCount, .Id = txn.message.Id });
        return;
    }
    const cp = assets.tables.cook_processed.getDataById(txn.message.Id) orelse {
        txn.respond(.{ .Code = .ErrCookingProcessNotFound, .Id = txn.message.Id });
        return;
    };
    var costs: std.ArrayList(ConsumeHelper.CostItem) = .empty;
    if (cp.ConsumeItemsId.len > 0) {
        for (cp.ConsumeItemsId) |entry| {
            if (entry.ItemId <= 0 or entry.Count <= 0) continue;
            const scaled = std.math.mul(i32, entry.Count, count) catch {
                txn.respond(.{ .Code = .ErrCookingCount, .Id = txn.message.Id });
                return;
            };
            try costs.append(alloc.arena, .{ .id = entry.ItemId, .count = scaled });
        }
    } else {
        for (txn.message.PrimaryFood.items) |it| {
            if (it.ItemId <= 0 or it.ItemNum <= 0) continue;
            const scaled = std.math.mul(i32, it.ItemNum, count) catch {
                txn.respond(.{ .Code = .ErrCookingCount, .Id = txn.message.Id });
                return;
            };
            try costs.append(alloc.arena, .{ .id = it.ItemId, .count = scaled });
        }
    }
    if (costs.items.len > 0) {
        if (!(try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, costs.items, io))) {
            txn.respond(.{ .Code = .ErrItemNotEnough, .Id = txn.message.Id });
            return;
        }
    }
    var final_food: std.ArrayList(pb.SingleItemInfo) = .empty;
    if (cp.FinalItemId > 0) {
        const out_items = [_]RewardService.RewardItem{.{ .item_id = cp.FinalItemId, .count = count }};
        _ = try RewardService.grantItems(reward_ctx, txn.message.Id, false, &out_items, 2);
        try final_food.append(alloc.arena, .{ .ItemId = cp.FinalItemId, .ItemNum = count });
    }
    txn.respond(.{
        .Code = .Success,
        .FinalFood = final_food,
        .Id = txn.message.Id,
    });
}

fn hideDiscardedWeapon(
    conn: anytype,
    alloc: mem.Alloc,
    io: std.Io,
    scene_opt: *?Scene,
    entity_id: i64,
) !void {
    if (scene_opt.*) |*scene| {
        var cpack: std.ArrayList(pb.CombatReceiveData) = .empty;
        if (try scene.setEntityVisibility(entity_id, false, &cpack, alloc.arena)) {
            if (cpack.items.len > 0) {
                try conn.push(alloc.gpa, io, pb.CombatReceivePackNotify{ .Data = cpack });
            }
        }
    }
}

pub fn onDiscardWeaponRequest(
    txn: *Transaction(pb.DiscardWeaponRequest),
    alloc: mem.Alloc,
    io: std.Io,
    scene_opt: *?Scene,
) !void {
    try hideDiscardedWeapon(txn.conn, alloc, io, scene_opt, txn.message.EntityId);
    txn.respond(.{ .DiscardSucc = true });
}

pub fn onDiscardWeaponPush(
    txn: *Transaction(pb.DiscardWeaponPush),
    alloc: mem.Alloc,
    io: std.Io,
    scene_opt: *?Scene,
) !void {
    try hideDiscardedWeapon(txn.conn, alloc, io, scene_opt, txn.message.EntityId);
}

// =========================================================================
// Map, Explore, Skins, Music, Profile, Settings & Misc Sync
// =========================================================================

pub fn onLastBigSceneMiniMapInfoRequest(
    txn: *Transaction(pb.LastBigSceneMiniMapInfoRequest),
    scene_opt: *?Scene,
    player_id: PlayerID,
) !void {
    var px: f32 = 0;
    var py: f32 = 0;
    var pz: f32 = 0;
    if (scene_opt.*) |*scene| {
        for (scene.instance.players) |player| {
            if (player.id == player_id.id) {
                px = player.location[0];
                py = player.location[1];
                pz = player.location[2];
                break;
            }
        }
    }
    txn.respond(.{
        .Code = .Success,
        .PosX = px,
        .PosY = py,
        .PosZ = pz,
        .PosA = 0,
    });
}

pub fn onNavigationFindPathRequest(txn: *Transaction(pb.NavigationFindPathRequest), alloc: mem.Alloc) !void {
    var path: std.ArrayList(pb.Vector) = .empty;
    if (txn.message.StartPos) |s| try path.append(alloc.arena, s);
    if (txn.message.EndPos) |e| try path.append(alloc.arena, e);
    txn.respond(.{
        .ErrorCode = .Success,
        .PosList = path,
    });
}

fn saveAndPushTreasureSlots(
    conn: anytype,
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    slots: []const i64,
) !void {
    var str_buf: std.ArrayList(u8) = .empty;
    var pb_slots: std.ArrayList(pb.TreasureBoxSlotInfo) = .empty;
    for (slots, 0..) |id, idx| {
        if (idx > 0) try str_buf.append(alloc.arena, ',');
        const id_str = try std.fmt.allocPrint(alloc.arena, "{d}", .{id});
        try str_buf.appendSlice(alloc.arena, id_str);
        try pb_slots.append(alloc.arena, .{ .DetectionSlotId = id, .MarkId = 0 });
    }
    try progress_comp.info.setKeyStr(alloc.gpa, "treasure_slots", str_buf.items);
    try progress_comp.save(fs, alloc.arena);
    try conn.push(alloc.gpa, io, pb.TreasureBoxSlotAllInfoNotify{
        .AllTreasureBoxSlotInfo = pb_slots,
    });
}

fn readTreasureSlots(arena: std.mem.Allocator, progress_comp: *const PlayerProgressComponent, skip_id: ?i64) !std.ArrayList(i64) {
    var slots: std.ArrayList(i64) = .empty;
    const raw = progress_comp.info.getKeyStr("treasure_slots");
    if (raw.len == 0) return slots;
    var it = std.mem.splitScalar(u8, raw, ',');
    while (it.next()) |tok| {
        if (tok.len == 0) continue;
        const parsed = std.fmt.parseInt(i64, tok, 10) catch continue;
        if (parsed <= 0) continue;
        if (skip_id) |sid| {
            if (parsed == sid) continue;
        }
        try slots.append(arena, parsed);
    }
    return slots;
}

pub fn onAddTreasureBoxSlotRequest(
    txn: *Transaction(pb.AddTreasureBoxSlotRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var slots = try readTreasureSlots(alloc.arena, progress_comp, null);
    const slot_id = try progress_comp.info.addKeyInt(alloc.gpa, "treasure_slot:next", 1);
    try slots.append(alloc.arena, slot_id);
    try saveAndPushTreasureSlots(txn.conn, alloc, io, fs, progress_comp, slots.items);
    txn.respond(.{
        .Code = .Success,
        .TreasureBoxSlotInfo = .{ .DetectionSlotId = slot_id, .MarkId = 0 },
    });
}

pub fn onRemoveTreasureBoxSlotRequest(
    txn: *Transaction(pb.RemoveTreasureBoxSlotRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const slots = try readTreasureSlots(alloc.arena, progress_comp, txn.message.DetectionSlotId);
    try saveAndPushTreasureSlots(txn.conn, alloc, io, fs, progress_comp, slots.items);
    txn.respond(.{ .Code = .Success });
}

pub fn onExploreActivityTaskRewardRequest(
    txn: *Transaction(pb.ExploreActivityTaskRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "explore_task:{d}", .{txn.message.TaskId});
    if (try progress_comp.info.markClaim(alloc.gpa, key)) {
        const fallback = [_]RewardService.RewardItem{ .{ .item_id = 3, .count = 20 }, .{ .item_id = 2, .count = 10000 } };
        _ = try RewardService.grantTableReward(reward_ctx, "ExploreActivityTask", txn.message.TaskId, false, true, 1, &fallback, null);
        try progress_comp.save(fs, alloc.arena);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onMulFishingIllustratedRewardRequest(
    txn: *Transaction(pb.MulFishingIllustratedRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var ids: std.ArrayList(i32) = .empty;
    for (txn.message.RewardId.items) |rid| {
        const key = try std.fmt.allocPrint(alloc.arena, "fish_ill:{d}", .{rid});
        if (try progress_comp.info.markClaim(alloc.gpa, key)) {
            const fallback = [_]RewardService.RewardItem{.{ .item_id = 3, .count = 20 }};
            _ = try RewardService.grantTableReward(reward_ctx, "FishingHandBook", rid, false, true, 1, &fallback, null);
        }
        try ids.append(alloc.arena, rid);
    }
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .RewardId = ids, .ErrorCode = .Success });
}

pub fn onOnlineMotorActivityLevelRequest(
    txn: *Transaction(pb.OnlineMotorActivityLevelRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const rtype: i32 = if (txn.message.RewardType) |t| @intFromEnum(t) else 0;
    for (txn.message.RewardIds.items) |rid| {
        const key = try std.fmt.allocPrint(alloc.arena, "online_motor:{d}:{d}", .{ rtype, rid });
        if (try progress_comp.info.markClaim(alloc.gpa, key)) {
            const fallback = [_]RewardService.RewardItem{.{ .item_id = 3, .count = 40 }};
            _ = try RewardService.grantTableReward(reward_ctx, "OnlineMotorActivity", rid, false, true, 1, &fallback, null);
        }
    }
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onPlayerCreatePsnSessionRequest(
    txn: *Transaction(pb.PlayerCreatePsnSessionRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    if (txn.message.PsnSessionId.len > 0) {
        try pc.info.setKeyStr(alloc.gpa, "psn:session_id", txn.message.PsnSessionId);
        try pc.save(fs, alloc.arena);
    }
    txn.respond(.{});
}

pub fn onPlayerPsnInfoGetRequest(
    txn: *Transaction(pb.PlayerPsnInfoGetRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    if (txn.message.PsnOnlineId.len > 0) {
        try pc.info.setKeyStr(alloc.gpa, "psn:online_id", txn.message.PsnOnlineId);
        try pc.save(fs, alloc.arena);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onPlayerXboxInfoGetRequest(
    txn: *Transaction(pb.PlayerXboxInfoGetRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    if (txn.message.XboxOnlineId.len > 0) {
        try pc.info.setKeyStr(alloc.gpa, "xbox:online_id", txn.message.XboxOnlineId);
        try pc.save(fs, alloc.arena);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onPsnSettingUpdateRequest(
    txn: *Transaction(pb.PsnSettingUpdateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "psn:only_play", if (txn.message.MatchPsnUser) 1 else 0);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onXboxSettingUpdateRequest(
    txn: *Transaction(pb.XboxSettingUpdateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "setting:xbox_match", if (txn.message.MatchXboxUser) 1 else 0);
    try progress_comp.info.setKeyInt(alloc.gpa, "setting:xbox_block_ugc", if (txn.message.XboxBlockUserCreatedContentViewing) 1 else 0);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onCommonTouchUiSettingUpdateRequest(
    txn: *Transaction(pb.CommonTouchUiSettingUpdateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    for (txn.message.MobileButtonSettings.items) |btn| {
        const k = try std.fmt.allocPrint(alloc.arena, "ui:touch_btn:{d}", .{btn.Id});
        try pb_kv.saveMsg(alloc, fs, pc, k, btn);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onMobileButtonSettingUpdateRequest(
    txn: *Transaction(pb.MobileButtonSettingUpdateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
) !void {
    for (txn.message.MobileButtonSettings.items) |btn| {
        const k = try std.fmt.allocPrint(alloc.arena, "ui:mobile_btn:{d}", .{btn.Id});
        try pb_kv.saveMsg(alloc, fs, pc, k, btn);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onChatMutePlayerRequest(
    txn: *Transaction(pb.ChatMutePlayerRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "chat_mute:{d}", .{txn.message.TargetUID});
    try progress_comp.info.setKeyInt(alloc.gpa, key, if (txn.message.Mute) 1 else 0);
    try progress_comp.save(fs, alloc.arena);
    var unmutes: std.ArrayList(i32) = .empty;
    if (!txn.message.Mute) try unmutes.append(alloc.arena, txn.message.TargetUID);
    txn.respond(.{ .RemoveMutes = unmutes, .ErrorCode = .Success });
}

// =========================================================================
// Client Pushes & UDP Movement Broadcast
// =========================================================================

pub fn onEntitySimplyMoveInfoPackagePush(
    txn: *Transaction(pb.EntitySimplyMoveInfoPackagePush),
    scene_opt: *?Scene,
    fs: *FileSystem,
    alloc: mem.Alloc,
    progress_comp: *PlayerProgressComponent,
) !void {
    const scene = if (scene_opt.*) |*s| s else return;
    var changed = false;
    for (txn.message.MoveInfos.items) |move| {
        const loc = move.Location orelse continue;
        const transform_changed = try persistWorldEntityTransform(
            scene,
            fs,
            alloc,
            progress_comp,
            move.EntityId,
            .{ loc.X, loc.Y, loc.Z },
            move.Rotation,
        );
        changed = transform_changed or changed;
        changed = try clearReachedSceneItemMoveTarget(
            scene,
            alloc,
            progress_comp,
            move.EntityId,
            .{ loc.X, loc.Y, loc.Z },
        ) or changed;
    }
    if (changed) try progress_comp.save(fs, alloc.arena);
}

pub fn onMoveSceneItemPush(
    txn: *Transaction(pb.MoveSceneItemPush),
    scene_opt: *?Scene,
    fs: *FileSystem,
    alloc: mem.Alloc,
    progress_comp: *PlayerProgressComponent,
) !void {
    const scene = if (scene_opt.*) |*s| s else return;
    const combat_common = txn.message.CombatCommon orelse return;
    if (txn.message.MoveInfos.items.len == 0) return;
    const move = txn.message.MoveInfos.items[txn.message.MoveInfos.items.len - 1];
    const loc = move.Location orelse return;
    var changed = try persistWorldEntityTransform(
        scene,
        fs,
        alloc,
        progress_comp,
        combat_common.EntityId,
        .{ loc.X, loc.Y, loc.Z },
        move.Rotation,
    );
    changed = try clearReachedSceneItemMoveTarget(
        scene,
        alloc,
        progress_comp,
        combat_common.EntityId,
        .{ loc.X, loc.Y, loc.Z },
    ) or changed;
    if (changed) try progress_comp.save(fs, alloc.arena);
}

fn clearReachedSceneItemMoveTarget(
    scene: *Scene,
    alloc: mem.Alloc,
    progress_comp: *PlayerProgressComponent,
    net_id: i64,
    location: [3]f32,
) !bool {
    if (!finiteVec3(location)) return false;
    const index = scene.net_id_map.get(net_id) orelse return false;
    const config_id = scene.entities.items(.config)[index].config_id;
    if (config_id <= 0) return false;

    const active_key = try std.fmt.allocPrint(alloc.arena, "scene_item_move_target_active:{d}", .{config_id});
    if (progress_comp.info.getKeyInt(active_key) != 1) return false;

    const fields = [_][]const u8{ "x", "y", "z" };
    var target: [3]f32 = undefined;
    for (fields, 0..) |field, i| {
        const key = try std.fmt.allocPrint(alloc.arena, "scene_item_move_target:{d}:{s}", .{ config_id, field });
        const raw = std.math.cast(u32, progress_comp.info.getKeyInt(key)) orelse return false;
        target[i] = @bitCast(raw);
        if (!std.math.isFinite(target[i])) return false;
    }

    const dx = location[0] - target[0];
    const dy = location[1] - target[1];
    const dz = location[2] - target[2];
    if (dx * dx + dy * dy + dz * dz > 1) return false;

    try progress_comp.info.setKeyInt(alloc.gpa, active_key, 0);
    return true;
}

fn persistWorldEntityTransform(
    scene: *Scene,
    fs: *FileSystem,
    alloc: mem.Alloc,
    progress_comp: *PlayerProgressComponent,
    net_id: i64,
    location: [3]f32,
    rotation_opt: ?pb.Rotator,
) !bool {
    if (!finiteVec3(location)) return false;
    const index = scene.net_id_map.get(net_id) orelse return false;
    const config = scene.entities.items(.config)[index];
    if (config.config_id <= 0 or config.entity_type == .player or config.entity_type == .player_entity) return false;
    if (scene.entities.items(.player_id)[index] != null) return false;
    if (rotation_opt) |rot| if (!finiteVec3(.{ rot.Roll, rot.Pitch, rot.Yaw })) return false;

    if (!scene.setEntityPosition(net_id, location, if (rotation_opt) |rot| rot.Yaw else null)) return false;
    if (rotation_opt) |rot| {
        if (scene.entities.items(.position)[index]) |*position| {
            position.rotation = .{ rot.Roll, rot.Pitch, rot.Yaw };
        }
    }
    try scene.saveComponents(fs, alloc.gpa, .{ .index = index, .net_id = net_id }, &.{Scene.Entity.PositionComponent});

    const position = scene.entities.items(.position)[index] orelse return true;
    const parts = [_][]const u8{ "x", "y", "z", "roll", "pitch", "yaw" };
    const values = [_]f32{
        position.location[0], position.location[1], position.location[2],
        position.rotation[0], position.rotation[1], position.rotation[2],
    };
    for (parts, values) |part, value| {
        const key = try std.fmt.allocPrint(alloc.arena, "world_pos:{d}:{s}", .{ config.config_id, part });
        try progress_comp.info.setKeyInt(alloc.gpa, key, @intCast(@as(u32, @bitCast(value))));
    }
    return true;
}

fn finiteVec3(value: [3]f32) bool {
    return std.math.isFinite(value[0]) and std.math.isFinite(value[1]) and std.math.isFinite(value[2]);
}

fn sceneItemMovementDisabled(assets: *const Assets, scene: *Scene, config_id: i32) bool {
    const config = assets.findLevelEntity(scene.instance_id, config_id) orelse return false;
    var components = config.ComponentsData;
    if (assets.findTemplateConfig(config.BlueprintType)) |template| {
        template.ComponentsData.mergeInto(&components);
    }
    return if (components.SceneItemMovementComponent) |movement| movement.Disabled orelse false else false;
}
pub fn onCharacterMoveToPointEndPush(
    txn: *Transaction(pb.CharacterMoveToPointEndPush),
    scene_opt: *?Scene,
    fs: *FileSystem,
    alloc: mem.Alloc,
    progress_comp: *PlayerProgressComponent,
) !void {
    const scene = if (scene_opt.*) |*s| s else return;
    const index = scene.net_id_map.get(txn.message.EntityId) orelse return;
    const position = scene.entities.items(.position)[index] orelse return;
    if (try persistWorldEntityTransform(
        scene,
        fs,
        alloc,
        progress_comp,
        txn.message.EntityId,
        position.location,
        .{ .Roll = position.rotation[0], .Pitch = position.rotation[1], .Yaw = position.rotation[2] },
    )) try progress_comp.save(fs, alloc.arena);
}
pub fn onFollowerChairPresentationPush(_: *Transaction(pb.FollowerChairPresentationPush)) !void {}
pub fn onHoldWeaponPush(_: *Transaction(pb.HoldWeaponPush)) !void {}
pub fn onPhotographPush(
    _: *Transaction(pb.PhotographPush),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    if (LevelPlayHelper.fromQuestContext(qctx)) |lp_ctx| {
        _ = LevelPlayHelper.onSubmitPhotoLevelPlayNode(lp_ctx, pc, 0, 0) catch {};
    } else {
        _ = pc.info.addKeyInt(alloc.gpa, "explore:photo_cnt", 1) catch {};
    }
    pc.save(fs, alloc.arena) catch {};
    QuestEngine.onConditionStateChanged(qctx) catch {};
}
pub fn onPlayEntityMontagePush(_: *Transaction(pb.PlayEntityMontagePush)) !void {}
pub fn onRoadNetworkEntityUpdateCurRoadPush(_: *Transaction(pb.RoadNetworkEntityUpdateCurRoadPush)) !void {}
pub fn onTestSpawnTemplateEntityPush(_: *Transaction(pb.TestSpawnTemplateEntityPush)) !void {}
pub fn onPlayerNetStateBackServerPush(_: *Transaction(pb.PlayerNetStateBackServerPush)) !void {}
pub fn onExitGamePush(_: *Transaction(pb.ExitGamePush)) !void {
    log.debug("client sent ExitGamePush", .{});
}

pub fn onUDPMovePackagePush(
    txn: *Transaction(pb.UDPMovePackagePush),
    scene_opt: *?Scene,
    sender: PlayerID,
    events: *EventQueue,
    assets: *const Assets,
    fs: *FileSystem,
    alloc: mem.Alloc,
    io: std.Io,
    role_comp: *PlayerRoleComponent,
    quest_comp: *PlayerQuestComponent,
    qctx: QuestEngine.QuestContext,
    maybe_coop: ?*CoopService,
) !void {
    if (scene_opt.*) |*scene| {
        var movable_q = Scene.Query(&.{ Scene.Entity, *Scene.Entity.PositionComponent }){
            .iterator = .{ .entities = &scene.entities, .net_id_map = &scene.net_id_map },
        };
        var controlled_q = Scene.Query(&.{ Scene.Entity, *Scene.Entity.DirectControlMarker }){
            .iterator = .{ .entities = &scene.entities, .net_id_map = &scene.net_id_map },
        };
        for (txn.message.MovingEntities.items) |data| {
            const entity, const position_comp = movable_q.byNetId(data.EntityId) orelse continue;
            const is_occupied_player_vehicle = if (scene.net_id_map.get(data.EntityId)) |eidx| blk: {
                const slice = scene.entities.slice();
                const veh = slice.items(.vehicle)[eidx] orelse break :blk false;
                if (!veh.occupied) break :blk false;
                const pid = slice.items(.player_id)[eidx] orelse break :blk true;
                break :blk pid.id == sender.id;
            } else false;
            const controlled_by_player = controlled_q.byNetId(data.EntityId) != null or is_occupied_player_vehicle;
            if (data.MoveInfos.items.len == 0) continue;
            const last_move_replay = data.MoveInfos.items[data.MoveInfos.items.len - 1];
            if (last_move_replay.Location) |location| {
                const next_location = [3]f32{ location.X, location.Y, location.Z };
                if (!finiteVec3(next_location)) continue;
                if (last_move_replay.Rotation) |rotation| {
                    if (!finiteVec3(.{ rotation.Roll, rotation.Pitch, rotation.Yaw })) continue;
                }
                var move_quest_pos: ?[3]f32 = null;
                if (controlled_by_player) {
                    scene.player_movement_mode = last_move_replay.MovementMode;
                    for (scene.instance.players) |*player| {
                        if (player.id == sender.id) {
                            player.location = next_location;
                            const dx = player.location[0] - scene.last_spatial_check_loc[0];
                            const dy = player.location[1] - scene.last_spatial_check_loc[1];
                            if (dx * dx + dy * dy >= 2500.0 * 2500.0) {
                                scene.last_spatial_check_loc = player.location;
                                events.enqueueBestEffort(.spatial_spawns_check, .{ .location = player.location });
                            }
                            if (quest_comp.info.active_quests.len > 0) {
                                const mdx = player.location[0] - position_comp.location[0];
                                const mdy = player.location[1] - position_comp.location[1];
                                const mdz = player.location[2] - position_comp.location[2];
                                if (mdx * mdx + mdy * mdy + mdz * mdz >= 100.0 * 100.0) {
                                    move_quest_pos = player.location;
                                }
                            }
                        }
                    }
                }
                position_comp.location = next_location;
                if (last_move_replay.Rotation) |rotation| {
                    if (controlled_by_player) {
                        for (scene.instance.players) |*player| {
                            if (player.id == sender.id) {
                                player.rotation = .{ rotation.Roll, rotation.Pitch, rotation.Yaw };
                            }
                        }
                    }
                    position_comp.rotation = .{ rotation.Roll, rotation.Pitch, rotation.Yaw };
                }
                try events.enqueue(.entity_movement, .{ .entity = entity });
                if (controlled_by_player) {
                    RoleHelper.healFormationIfNearNexus(txn.conn, scene, assets, role_comp, events, fs, alloc, io, next_location) catch |err| {
                        std.log.scoped(.role).warn("healFormationIfNearNexus failed: {t}", .{err});
                    };
                }
                if (move_quest_pos) |pos| {
                    try QuestEngine.onMove(qctx, pos);
                }
            }
        }
    }
    if (maybe_coop) |coop| {
        coop.broadcastMoveExcept(alloc.gpa, io, sender.id, txn.message.MovingEntities);
    }
}

pub fn onInteractiveUpdateRequest(
    txn: *Transaction(pb.InteractiveUpdateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    try persistClaimKey(alloc, fs, progress_comp, "interact:upd", txn.message.InteractiveId);
    QuestEngine.onConditionStateChanged(qctx) catch |err| QuestEngine.logHookErr("onConditionStateChanged", err);
    txn.respond(.{
        .Code = .Success,
        .InteractiveId = txn.message.InteractiveId,
    });
}

pub fn onSceneItemSetPosSenderRequest(
    txn: *Transaction(pb.SceneItemSetPosSenderRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try persistIntKey(alloc, fs, progress_comp, "scene_item:pos_sender", txn.message.EntityId, if (txn.message.IsControl) 1 else 0);
    txn.respond(.{ .ErrorCode = .Success });
}
