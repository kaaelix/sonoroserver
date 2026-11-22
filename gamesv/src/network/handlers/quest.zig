const std = @import("std");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const common = @import("common");
const FileSystem = common.FileSystem;
const PlayerQuestComponent = @import("../../logic/component/player/PlayerQuestComponent.zig");
const PlayerProgressComponent = @import("../../logic/component/player/PlayerProgressComponent.zig");
const QuestEngine = @import("../../logic/helpers/quest_engine.zig");
const LevelPlayHelper = @import("../../logic/helpers/level_play.zig");
const Transaction = @import("../handlers.zig").Transaction;

pub fn onQuestReviewDataRequest(
    txn: *Transaction(pb.QuestReviewDataRequest),
    quest_comp: *const PlayerQuestComponent,
    progress_comp: *PlayerProgressComponent,
    fs: *FileSystem,
    alloc: mem.Alloc,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "quest:finished_cnt", @intCast(quest_comp.info.finished_quests.len));
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSetFocusModeDeterCondRequest(
    txn: *Transaction(pb.SetFocusModeDeterCondRequest),
    qctx: QuestEngine.QuestContext,
) !void {
    try QuestEngine.setFocusDeterCond(qctx, txn.message.DeterCond);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSetQuestFocusModeRequest(
    txn: *Transaction(pb.SetQuestFocusModeRequest),
    fs: *FileSystem,
    quest_comp: *PlayerQuestComponent,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    quest_comp.info.focus_quest_id = txn.message.QuestId;
    try quest_comp.save(fs, alloc.arena);
    try txn.conn.push(alloc.gpa, io, pb.FocusQuestSetNotify{
        .FocusQuestId = txn.message.QuestId,
        .Reason = .Request,
    });
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onCancelQuestFocusModeRequest(
    txn: *Transaction(pb.CancelQuestFocusModeRequest),
    fs: *FileSystem,
    quest_comp: *PlayerQuestComponent,
    alloc: mem.Alloc,
) !void {
    quest_comp.info.focus_quest_id = 0;
    try quest_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onAcceptFocusWaitQuestRequest(
    txn: *Transaction(pb.AcceptFocusWaitQuestRequest),
    qctx: QuestEngine.QuestContext,
) !void {
    try QuestEngine.acceptFocusWaitQuest(qctx, txn.message.QuestId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onConfirmQuestResourceRequest(
    txn: *Transaction(pb.ConfirmQuestResourceRequest),
    qctx: QuestEngine.QuestContext,
) !void {
    try QuestEngine.syncQuestResourceAvailability(qctx, &.{}, txn.message.QuestIds.items, false);
    txn.respond(.{ .ErrorId = .Success });
}

pub fn onQuestBranchRequest(
    txn: *Transaction(pb.QuestBranchRequest),
    qctx: QuestEngine.QuestContext,
) !void {
    txn.respond(.{
        .errorCode = pb.ErrorCode.Success,
        .QuestBranchInfo = try QuestEngine.buildQuestBranchInfo(qctx),
    });
}

pub fn onQuestBranchSelectRequest(
    txn: *Transaction(pb.QuestBranchSelectRequest),
    qctx: QuestEngine.QuestContext,
) !void {
    const info = try QuestEngine.selectQuestBranch(qctx, txn.message.BranchPageId, txn.message.BranchId);
    txn.respond(.{
        .ErrorCode = .Success,
        .OneQuestBranchPageInfo = info,
    });
}

pub fn onConditionGroupStateRequest(
    txn: *Transaction(pb.ConditionGroupStateRequest),
    qctx: QuestEngine.QuestContext,
) !void {
    const infos = try QuestEngine.evalConditionGroups(qctx, txn.message.ConditionGroupIds.items);
    txn.respond(.{
        .ConditionGroupFinishInfos = infos,
        .ErrorCode = .Success,
    });
}

pub fn onDailyQuestGetRewardRequest(
    txn: *Transaction(pb.DailyQuestGetRewardRequest),
    qctx: QuestEngine.QuestContext,
) !void {
    const rewards = try QuestEngine.claimDailyRewards(qctx, txn.message.QuestId.items);
    var list: std.ArrayList(pb.DailyQuestReward) = .empty;
    try list.appendSlice(qctx.alloc.arena, rewards);
    txn.respond(.{
        .Rewards = list,
        .ErrorId = .Success,
    });
}

pub fn onDailyQuestChangePreferRequest(
    txn: *Transaction(pb.DailyQuestChangePreferRequest),
    qctx: QuestEngine.QuestContext,
) !void {
    try QuestEngine.changeDailyPrefer(qctx, txn.message.TargetArea, txn.message.TargetInfluence);
    txn.respond(.{ .ErrorId = .Success });
}

pub fn onSubmitPhotoNodeRequest(
    txn: *Transaction(pb.SubmitPhotoNodeRequest),
    qctx: QuestEngine.QuestContext,
) !void {
    const owner_id = std.math.cast(i32, txn.message.TreeOwnerId) orelse 0;
    try QuestEngine.onSubmitPhotoNode(qctx, owner_id, txn.message.NodeId);
    if (LevelPlayHelper.fromQuestContext(qctx)) |lp_ctx| {
        _ = try LevelPlayHelper.onSubmitPhotoLevelPlayNode(lp_ctx, qctx.progress_comp, owner_id, txn.message.NodeId);
    }
    txn.respond(.{ .ErrorId = .Success });
}

pub fn onSubmitNodeRequest(
    txn: *Transaction(pb.SubmitNodeRequest),
    qctx: QuestEngine.QuestContext,
) !void {
    const err_id = try QuestEngine.onSubmitNode(
        qctx,
        txn.message.TreeOwnerId,
        txn.message.TreeIncId,
        txn.message.NodeId,
    );
    if (LevelPlayHelper.fromQuestContext(qctx)) |lp_ctx| {
        _ = try LevelPlayHelper.onSubmitLevelPlayNode(
            lp_ctx,
            qctx.progress_comp,
            txn.message.TreeOwnerId,
            txn.message.TreeIncId,
            txn.message.NodeId,
        );
    }
    txn.respond(.{
        .TreeOwnerId = txn.message.TreeOwnerId,
        .TreeIncId = txn.message.TreeIncId,
        .NodeId = txn.message.NodeId,
        .ErrorId = err_id,
    });
}

pub fn onTimerEndRequest(
    txn: *Transaction(pb.TimerEndRequest),
    qctx: QuestEngine.QuestContext,
) !void {
    const owner_id = std.math.cast(i32, txn.message.TreeOwnerId) orelse 0;
    try QuestEngine.onBtTimerEnd(qctx, owner_id, txn.message.TreeIncId, txn.message.TimerType);
    txn.respond(.{ .ErrorId = .Success });
}

pub fn onBtSetTimerInfoRequest(
    txn: *Transaction(pb.BtSetTimerInfoRequest),
    qctx: QuestEngine.QuestContext,
) !void {
    const owner_id = std.math.cast(i32, txn.message.TreeOwnerId) orelse 0;
    try QuestEngine.onBtSetTimerInfo(
        qctx,
        owner_id,
        txn.message.TreeIncId,
        txn.message.NodeId,
        txn.message.TimerType,
        txn.message.Second,
        txn.message.SetType orelse pb.TimerSetType.default,
    );
    txn.respond(.{ .ErrorId = .Success });
}

pub fn onTraceQuestRequest(
    txn: *Transaction(pb.TraceQuestRequest),
    qctx: QuestEngine.QuestContext,
) !void {
    const qid: i32 = if (txn.message.Operate == 0) 0 else txn.message.QuestId;
    try QuestEngine.setTrackedQuest(qctx, qid);
    try txn.conn.push(qctx.alloc.gpa, qctx.io, pb.TraceQuestNotify{
        .QuestId = qctx.quest_comp.info.tracked_quest_id,
    });
    txn.respond(.{ .ErrorId = .Success });
}

pub fn onQuestRedDotRequest(
    txn: *Transaction(pb.QuestRedDotRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "quest:red_dot:{d}", .{txn.message.QuestId});
    try progress_comp.info.setKeyInt(alloc.gpa, k, txn.message.Operate);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorId = .Success });
}

pub fn onActionFinishRequest(
    txn: *Transaction(pb.ActionFinishRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [64]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "quest:act_fin:{d}:{d}", .{ txn.message.IncId, txn.message.StartIndex });
    _ = try progress_comp.info.markClaim(alloc.gpa, k);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .Code = .Success });
}

pub fn onBtGiveUpRequest(
    txn: *Transaction(pb.BtGiveUpRequest),
    qctx: QuestEngine.QuestContext,
) !void {
    try QuestEngine.onBtGiveUp(qctx, txn.message.TreeOwnerId, txn.message.TreeIncId, txn.message.NodeId);
    txn.respond(.{ .ErrorId = .Success });
}

pub fn onBtRollbackRequest(
    txn: *Transaction(pb.BtRollbackRequest),
    qctx: QuestEngine.QuestContext,
) !void {
    try QuestEngine.onBtRollback(qctx, txn.message.TreeOwnerId, txn.message.TreeIncId, txn.message.Select);
    txn.respond(.{ .ErrorId = .Success });
}

pub fn onQuestActionRequest(
    txn: *Transaction(pb.QuestActionRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    if (txn.message.QuestId > 0) {
        var kbuf: [64]u8 = undefined;
        const k = try std.fmt.bufPrint(&kbuf, "quest:act:{d}:{d}:{d}", .{ txn.message.QuestId, txn.message.NodeId, txn.message.ActionId });
        if (try progress_comp.info.markClaim(alloc.gpa, k)) {
            try progress_comp.save(fs, alloc.arena);
        }
    }
    try QuestEngine.onConditionStateChanged(qctx);
    txn.respond(.{ .ErrorId = .Success });
}

pub fn onQuestFinishActionRequest(
    txn: *Transaction(pb.QuestFinishActionRequest),
    qctx: QuestEngine.QuestContext,
) !void {
    try QuestEngine.onConditionStateChanged(qctx);
    txn.respond(.{ .ErrorId = .Success });
}

pub fn onQuestFinishActionEndRequest(
    txn: *Transaction(pb.QuestFinishActionEndRequest),
    qctx: QuestEngine.QuestContext,
) !void {
    try QuestEngine.onConditionStateChanged(qctx);
    txn.respond(.{ .ErrorId = .Success });
}

pub fn onQuestTreeInfoRequest(
    txn: *Transaction(pb.QuestTreeInfoRequest),
    qctx: QuestEngine.QuestContext,
) !void {
    try QuestEngine.resyncOnReconnect(qctx);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRoleQuestAcceptRequest(
    txn: *Transaction(pb.RoleQuestAcceptRequest),
    qctx: QuestEngine.QuestContext,
) !void {
    try QuestEngine.acceptQuestById(qctx, txn.message.QuestId);
    txn.respond(.{ .ErrorId = .Success });
}

pub fn onRoleFavorAcceptQuestRequest(
    txn: *Transaction(pb.RoleFavorAcceptQuestRequest),
) !void {
    txn.respond(.{
        .QuestType = txn.message.QuestType,
        .RoleId = txn.message.RoleId,
        .ErrCode = .Success,
    });
}

pub fn onSubmitAwakeAndLoadEntityNodeRequest(
    txn: *Transaction(pb.SubmitAwakeAndLoadEntityNodeRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    var dirty = false;
    var kbuf: [48]u8 = undefined;
    if (txn.message.TreeOwnerId > 0) {
        const k = try std.fmt.bufPrint(&kbuf, "awake_node:{d}", .{txn.message.TreeOwnerId});
        try pc.info.setKeyInt(alloc.gpa, k, txn.message.NodeId);
        dirty = true;
    }
    if (txn.message.TreeIncId > 0) {
        const k = try std.fmt.bufPrint(&kbuf, "awake_node:{d}", .{txn.message.TreeIncId});
        try pc.info.setKeyInt(alloc.gpa, k, txn.message.NodeId);
        dirty = true;
    }
    if (dirty) try pc.save(fs, alloc.arena);
    _ = try QuestEngine.onSubmitNode(qctx, txn.message.TreeOwnerId, txn.message.TreeIncId, txn.message.NodeId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onQuestReviewInfoRequest(
    txn: *Transaction(pb.QuestReviewInfoRequest),
    quest_comp: *const PlayerQuestComponent,
    progress_comp: *PlayerProgressComponent,
    fs: *FileSystem,
    alloc: mem.Alloc,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "quest:finished_cnt", @intCast(quest_comp.info.finished_quests.len));
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}
