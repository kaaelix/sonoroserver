const std = @import("std");
const common = @import("common");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const Assets = @import("../../data/Assets.zig");
const Scene = @import("../../logic/Scene.zig");
const QuestEngine = @import("../../logic/helpers/quest_engine.zig");
const RewardService = @import("../../logic/helpers/reward.zig");
const PlayerProgressComponent = @import("../../logic/component/player/PlayerProgressComponent.zig");
const PlayerQuestComponent = @import("../../logic/component/player/PlayerQuestComponent.zig");
const PlayerRoleComponent = @import("../../logic/component/player/PlayerRoleComponent.zig");
const RoleHelper = @import("../../logic/helpers/role.zig");
const EventQueue = @import("../../logic/EventQueue.zig");
const QuestInfo = @import("../../fs/QuestInfo.zig");
const Transaction = @import("../handlers.zig").Transaction;

const FileSystem = common.FileSystem;

pub fn onEnterInstRequest(
    txn: *Transaction(pb.EnterInstRequest),
    assets: *const Assets,
    qctx: QuestEngine.QuestContext,
) !void {
    if (assets.tables.instance_dungeon.getDataById(txn.message.InstanceId) == null) {
        txn.respond(.{ .ErrorCode = .ErrInstanceConfigNotFound });
        return;
    }
    try QuestEngine.enterInst(qctx, txn.message.InstanceId, txn.message.PosEntityId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onEnterBigWorldInstRequest(
    txn: *Transaction(pb.EnterBigWorldInstRequest),
    assets: *const Assets,
    qctx: QuestEngine.QuestContext,
) !void {
    if (qctx.scene) |scene| {
        if (assets.tables.instance_dungeon.getDataById(scene.instance_id)) |inst| {
            if (inst.InstType != 1) {
                try QuestEngine.leaveInst(qctx, 0);
            }
        }
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onLeaveInstRequest(
    txn: *Transaction(pb.LeaveInstRequest),
    qctx: QuestEngine.QuestContext,
) !void {
    try QuestEngine.leaveInst(qctx, txn.message.PosEntityId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onTeleportDungeonActionRequest(
    txn: *Transaction(pb.TeleportDungeonActionRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "inst:tp_action:{d}", .{txn.message.IncId});
    try progress_comp.info.setKeyInt(alloc.gpa, key, if (txn.message.IsConfirm) 1 else 0);
    try progress_comp.save(fs, alloc.arena);
    if (txn.message.IsConfirm and qctx.scene != null) {
        QuestEngine.onSceneLoadingFinish(qctx) catch |err| QuestEngine.logHookErr("onSceneLoadingFinish", err);
    }
    txn.respond(.{ .Code = .Success });
}

pub fn onTeleportFlowEndRequest(
    txn: *Transaction(pb.TeleportFlowEndRequest),
    qctx: QuestEngine.QuestContext,
) !void {
    if (qctx.scene != null) {
        QuestEngine.onSceneLoadingFinish(qctx) catch |err| QuestEngine.logHookErr("onSceneLoadingFinish", err);
    }
    txn.respond(.{ .Code = .Success });
}

pub fn onSceneSubLevelsRequest(
    txn: *Transaction(pb.SceneSubLevelsRequest),
    qctx: QuestEngine.QuestContext,
) !void {
    try QuestEngine.changeSubLevels(
        qctx,
        txn.message.ShowSubLevels.items,
        &.{},
        txn.message.HideSubLevels.items,
        0,
    );
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onChangeDataLayerFinishRequest(
    txn: *Transaction(pb.ChangeDataLayerFinishRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "inst:datalayer:{d}", .{txn.message.InstId});
    try progress_comp.info.setKeyInt(alloc.gpa, key, 1);
    try progress_comp.save(fs, alloc.arena);
    if (qctx.scene != null) {
        QuestEngine.onSceneLoadingFinish(qctx) catch |err| QuestEngine.logHookErr("onSceneLoadingFinish", err);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onFlowRestartRequest(
    txn: *Transaction(pb.FlowRestartRequest),
    qctx: QuestEngine.QuestContext,
) !void {
    try QuestEngine.restartFlow(qctx, txn.message.FlowIncId);
    txn.respond(.{ .Code = .Success });
}

pub fn onLevelPlayOpenRequest(
    txn: *Transaction(pb.LevelPlayOpenRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "lp:open:{d}", .{txn.message.LevelPlayId});
    _ = try progress_comp.info.markClaim(alloc.gpa, k);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .ErrorCode = .Success,
        .LevelPlayId = txn.message.LevelPlayId,
    });
}

pub fn onLevelPlayRewardRequest(
    txn: *Transaction(pb.LevelPlayRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const cnt = @max(1, txn.message.ExchangeCount);
    var reward_id: i32 = 0;
    if (std.math.cast(i32, txn.message.EntityId)) |entity_id32| {
        const reward_indices = assets.indexes.levelPlayIndicesForRewardEntity(entity_id32);
        const lp_indices = if (reward_indices.len > 0) reward_indices else assets.indexes.levelPlayIndicesForEntity(entity_id32);
        if (lp_indices.len > 0) {
            const lp = assets.tables.level_play_data.items[lp_indices[0]];
            if (lp.Data.LevelPlayRewardConfig) |rc| {
                reward_id = rc.RewardId;
            }
        }
    }

    var kbuf: [64]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "lp:reward_cnt:{d}", .{txn.message.EntityId});
    _ = try progress_comp.info.addKeyInt(alloc.gpa, k, cnt);
    try progress_comp.save(fs, alloc.arena);

    var granted_any = false;
    if (reward_id > 0) {
        var i: i32 = 0;
        while (i < cnt) : (i += 1) {
            const res = try RewardService.grantTreasureBoxReward(reward_ctx, 2, reward_id, 18500);
            if (res.granted_count > 0) granted_any = true;
        }
    }
    if (!granted_any) {
        _ = try RewardService.grantItems(reward_ctx, 0, true, &.{
            .{ .item_id = 3, .count = 20 * cnt },
            .{ .item_id = 2, .count = 5000 * cnt },
            .{ .item_id = 1, .count = 100 * cnt },
        }, 18500);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onGetLevelPlayRewardsRequest(
    txn: *Transaction(pb.GetLevelPlayRewardsRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
    quest_comp: *const PlayerQuestComponent,
) !void {
    var reward_nodes: std.ArrayList(i32) = .empty;
    var treasure_boxes: std.ArrayList(i32) = .empty;
    const completed = QuestInfo.containsInt(quest_comp.info.completed_level_plays, txn.message.LevelPlayId);
    if (assets.tables.level_play_data.getDataById(txn.message.LevelPlayId)) |lp| {
        if (lp.Data.LevelPlayRewardConfig) |rc| {
            if (rc.RewardEntityId > 0 and quest_comp.info.hasOpenedChest(rc.RewardEntityId)) {
                try treasure_boxes.append(alloc.arena, rc.RewardEntityId);
            }
        }
        for (lp.Data.Children) |child_str| {
            const child_id = std.fmt.parseInt(i32, child_str, 10) catch continue;
            if (quest_comp.info.hasOpenedChest(child_id)) {
                var dup = false;
                for (treasure_boxes.items) |b| if (b == child_id) {
                    dup = true;
                    break;
                };
                if (!dup) try treasure_boxes.append(alloc.arena, child_id);
            }
        }
    }
    if (completed or treasure_boxes.items.len > 0) {
        try reward_nodes.append(alloc.arena, txn.message.LevelPlayId);
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .TreasureBoxIds = treasure_boxes,
        .RewardNodeIds = reward_nodes,
    });
}

pub fn onLevelPlayFastReturnPosRequest(
    txn: *Transaction(pb.LevelPlayFastReturnPosRequest),
    qctx: QuestEngine.QuestContext,
) !void {
    if (txn.message.NodeId > 0) {
        const owner_id = std.math.cast(i32, txn.message.TreeOwnerId) orelse 0;
        _ = try QuestEngine.onSubmitNode(qctx, @max(0, owner_id), txn.message.TreeIncId, txn.message.NodeId);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onLevelPlaySwitchSportModeRequest(
    txn: *Transaction(pb.LevelPlaySwitchSportModeRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const mode: i32 = if (txn.message.NewSportMode) |m| @intFromEnum(m) else 0;
    try progress_comp.info.setKeyInt(alloc.gpa, "lp:sport_mode", mode);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSingleInstRechallengeRequest(
    txn: *Transaction(pb.SingleInstRechallengeRequest),
    scene: *Scene,
    assets: *const Assets,
    role_comp: *PlayerRoleComponent,
    events: *EventQueue,
    fs: *FileSystem,
    alloc: mem.Alloc,
    io: std.Io,
    qctx: QuestEngine.QuestContext,
) !void {
    try RoleHelper.reviveAndHealFormation(txn.conn, scene, role_comp, events, fs, alloc, io);
    const inst = assets.tables.instance_dungeon.getDataById(scene.instance_id);
    if (txn.message.RechallengeInInst) {
        if (inst) |cfg| {
            const x: f32 = @floatFromInt(cfg.BornPosition[0]);
            const y: f32 = @floatFromInt(cfg.BornPosition[1]);
            const z: f32 = @floatFromInt(cfg.BornPosition[2]);
            const rot_x: f32 = @floatFromInt(cfg.BornRotation[0]);
            const rot_y: f32 = @floatFromInt(cfg.BornRotation[1]);
            const rot_z: f32 = @floatFromInt(cfg.BornRotation[2]);
            try QuestEngine.updatePlayerSceneState(qctx, .{ x, y, z }, .{ rot_x, rot_y, rot_z });
            try txn.conn.push(alloc.gpa, io, pb.TeleportNotify{
                .MapId = cfg.MapConfigId,
                .Pos = .{ .X = x, .Y = y, .Z = z },
                .Rot = .{ .X = rot_x, .Y = rot_y, .Z = rot_z },
            });
        }
    } else {
        try txn.conn.push(alloc.gpa, io, pb.LeaveSceneNotify{
            .PlayerId = scene.player_id,
            .SceneId = "",
            .TransitionOption = .{},
        });
        try events.enqueue(.initial_scene_join, .{});
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onApplyRechallengeRequest(
    txn: *Transaction(pb.ApplyRechallengeRequest),
    scene: *Scene,
    role_comp: *PlayerRoleComponent,
    events: *EventQueue,
    fs: *FileSystem,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    _ = txn.message;
    try RoleHelper.reviveAndHealFormation(txn.conn, scene, role_comp, events, fs, alloc, io);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onInstFightEndPush(
    txn: *Transaction(pb.InstFightEndPush),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    if (txn.message.InstId > 0) {
        var kbuf: [48]u8 = undefined;
        const key = try std.fmt.bufPrint(&kbuf, "inst:fight_end:{d}", .{txn.message.InstId});
        _ = try progress_comp.info.markClaim(alloc.gpa, key);
        try progress_comp.save(fs, alloc.arena);
    }
    QuestEngine.onConditionStateChanged(qctx) catch |err| QuestEngine.logHookErr("onConditionStateChanged", err);
}

pub fn onInstTeleportToResetPointRequest(
    txn: *Transaction(pb.InstTeleportToResetPointRequest),
    assets: *const Assets,
    qctx: QuestEngine.QuestContext,
) !void {
    if (qctx.scene) |scene| {
        if (assets.tables.instance_dungeon.getDataById(scene.instance_id)) |inst| {
            const pos: [3]f32 = .{
                @floatFromInt(inst.BornPosition[0]),
                @floatFromInt(inst.BornPosition[1]),
                @floatFromInt(inst.BornPosition[2]),
            };
            const rot: [3]f32 = .{
                @floatFromInt(inst.BornRotation[0]),
                @floatFromInt(inst.BornRotation[1]),
                @floatFromInt(inst.BornRotation[2]),
            };
            try QuestEngine.updatePlayerSceneState(qctx, pos, rot);
            try txn.conn.push(qctx.alloc.gpa, qctx.io, pb.TeleportNotify{
                .MapId = scene.instance_id,
                .Pos = .{ .X = pos[0], .Y = pos[1], .Z = pos[2] },
                .Rot = .{ .X = rot[0], .Y = rot[1], .Z = rot[2] },
                .Reason = .Action,
            });
        }
    }
    QuestEngine.onConditionStateChanged(qctx) catch |err| QuestEngine.logHookErr("onConditionStateChanged", err);
    txn.respond(.{ .ErrorCode = .Success });
}
