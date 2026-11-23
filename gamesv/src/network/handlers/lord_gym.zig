const std = @import("std");
const common = @import("common");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const Assets = @import("../../data/Assets.zig");
const ProgressInfo = @import("../../fs/ProgressInfo.zig");
const PlayerProgressComponent = @import("../../logic/component/player/PlayerProgressComponent.zig");
const Scene = @import("../../logic/Scene.zig");
const QuestEngine = @import("../../logic/helpers/quest_engine.zig");
const Transaction = @import("../handlers.zig").Transaction;

const FileSystem = common.FileSystem;

pub fn onLordGymInfoRequest(
    txn: *Transaction(pb.LordGymInfoRequest),
    assets: *const Assets,
    progress_comp: *const PlayerProgressComponent,
    alloc: mem.Alloc,
) !void {
    var unlock_ids: std.ArrayList(i32) = .empty;
    for (assets.tables.lord_gym.items) |gym| {
        try unlock_ids.append(alloc.arena, gym.Id);
    }

    var read_ids: std.ArrayList(i32) = .empty;
    try read_ids.appendSlice(alloc.arena, progress_comp.info.challenge.read_lord_gyms);

    var pass_records: std.ArrayList(pb.LordGymPassRecord) = .empty;
    for (progress_comp.info.challenge.passed_lord_gyms) |rec| {
        try pass_records.append(alloc.arena, .{
            .LoadGymId = rec.gym_id,
            .PassTime = rec.pass_time,
        });
    }

    var entrances: std.ArrayList(pb.LordGymEntranceInfo) = .empty;
    var group_infos: std.ArrayList(pb.LordGymGroupInfo) = .empty;
    for (assets.tables.lord_gym_entrance.items) |entrance| {
        try entrances.append(alloc.arena, .{ .ConfigId = entrance.Id });
        var max_diff: i32 = 0;
        for (progress_comp.info.challenge.passed_lord_gyms) |pass_rec| {
            if (assets.tables.lord_gym.getDataById(pass_rec.gym_id)) |g| {
                // Match difficulty IDs belonging to the same encounter group
                if (@divTrunc(g.Id, 100) == @divTrunc(entrance.Id, 10) and g.Difficulty > max_diff) {
                    max_diff = g.Difficulty;
                }
            }
        }
        if (max_diff > 0) {
            try group_infos.append(alloc.arena, .{
                .GroupId = entrance.Id,
                .PassDiff = max_diff,
            });
        }
    }

    txn.respond(.{
        .UnlockLoadGymIds = unlock_ids,
        .ReadLoadGymIds = read_ids,
        .LordGymPassRecords = pass_records,
        .LordGymEntranceInfos = entrances,
        .LordGymGroupInfos = group_infos,
    });
}

pub fn onLordGymReadRequest(
    txn: *Transaction(pb.LordGymReadRequest),
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    alloc: mem.Alloc,
) !void {
    const id = txn.message.LoadGymId;
    if (assets.tables.lord_gym.getDataById(id) == null) {
        txn.respond(.{ .ErrorCode = .ErrLordGymConfigNotFound });
        return;
    }
    if (try ProgressInfo.appendUniqueId(&progress_comp.info.challenge.read_lord_gyms, alloc.gpa, id)) {
        try progress_comp.save(fs, alloc.arena);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onLordGymBeginRequest(
    txn: *Transaction(pb.LordGymBeginRequest),
    assets: *const Assets,
    qctx: QuestEngine.QuestContext,
    scene: *Scene,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    const gym = assets.tables.lord_gym.getDataById(txn.message.LoadGymId) orelse {
        txn.respond(.{ .ErrorCode = .ErrLordGymConfigNotFound });
        return;
    };
    if (gym.PlayId > 0) {
        if (assets.tables.level_play_data.getDataById(gym.PlayId)) |lp| {
            const target_inst = if (lp.Data.InstanceId > 0) lp.Data.InstanceId else lp.Data.LevelId;
            const in_lord_gym_inst = if (assets.tables.instance_dungeon.getDataById(scene.instance_id)) |cur_inst|
                cur_inst.InstSubType == 48
            else
                false;
            if (target_inst > 0 and target_inst != scene.instance_id and !in_lord_gym_inst and assets.tables.instance_dungeon.getDataById(target_inst) != null) {
                try QuestEngine.enterInst(qctx, target_inst, 0);
            }
        }
        const LevelPlayHelper = @import("../../logic/helpers/level_play.zig");
        if (LevelPlayHelper.fromQuestContext(qctx)) |lp_ctx| {
            try LevelPlayHelper.onLevelPlayEnabled(lp_ctx, gym.PlayId, true);
        } else {
            try txn.conn.push(alloc.gpa, io, pb.EnterLevelPlayNotify{
                .Id = gym.PlayId,
                .State = 1,
                .CanGetReward = false,
            });
            try txn.conn.push(alloc.gpa, io, pb.LevelPlayStateNotify{
                .Id = gym.PlayId,
                .State = 1,
                .CompleteNumber = 0,
            });
        }
    }
    txn.respond(.{ .ErrorCode = .Success });
}
