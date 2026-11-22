const std = @import("std");
const common = @import("common");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const Assets = @import("../../data/Assets.zig");
const RoleInfo = @import("../../fs/RoleInfo.zig");
const PlayerBasicComponent = @import("../../logic/component/player/PlayerBasicComponent.zig");
const PlayerEchoComponent = @import("../../logic/component/player/PlayerEchoComponent.zig");
const PlayerInventoryComponent = @import("../../logic/component/player/PlayerInventoryComponent.zig");
const PlayerMapComponent = @import("../../logic/component/player/PlayerMapComponent.zig");
const PlayerProgressComponent = @import("../../logic/component/player/PlayerProgressComponent.zig");
const PlayerQuestComponent = @import("../../logic/component/player/PlayerQuestComponent.zig");
const ConditionEvaluator = @import("../../logic/helpers/condition.zig");
const QuestEngine = @import("../../logic/helpers/quest_engine.zig");
const RewardService = @import("../../logic/helpers/reward.zig");
const Transaction = @import("../handlers.zig").Transaction;
const pb_kv = @import("pb_kv.zig");

const FileSystem = common.FileSystem;

fn claimIds(
    reward_ctx: RewardService.RewardContext,
    progress_comp: *PlayerProgressComponent,
    fs: *FileSystem,
    gpa: std.mem.Allocator,
    arena: std.mem.Allocator,
    prefix: []const u8,
    table_name: []const u8,
    ids: []const i32,
) !std.ArrayList(i32) {
    var claimed: std.ArrayList(i32) = .empty;
    var dirty = false;
    var kbuf: [64]u8 = undefined;
    for (ids) |id| {
        const k = try std.fmt.bufPrint(&kbuf, "{s}:{d}", .{ prefix, id });
        if (!(try progress_comp.info.markClaim(gpa, k))) continue;
        dirty = true;
        try claimed.append(arena, id);
        _ = try RewardService.grantTableReward(
            reward_ctx,
            table_name,
            id,
            false,
            true,
            1,
            &.{ .{ .item_id = 3, .count = 40 }, .{ .item_id = 2, .count = 10000 } },
            null,
        );
    }
    if (dirty) try progress_comp.save(fs, arena);
    return claimed;
}

// === Infr & InfrV2 ===
pub fn onInfrArchiveReadRequest(
    txn: *Transaction(pb.InfrArchiveReadRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    for (txn.message.ArchiveId.items) |id| {
        const k = try std.fmt.bufPrint(&kbuf, "infr:arch:{d}", .{id});
        _ = try progress_comp.info.markClaim(alloc.gpa, k);
    }
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onInfrArchiveTaskRewardRequest(
    txn: *Transaction(pb.InfrArchiveTaskRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimIds(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "infr:arch_rwd", "InfrArchiveTask", txn.message.TargetIds.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onInfrFireNoticeRequest(
    txn: *Transaction(pb.InfrFireNoticeRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "infr:fire_notice_ack", 1);
    try progress_comp.save(fs, alloc.arena);
    var notices: std.ArrayList(pb.InfrNotice) = .empty;
    if (progress_comp.info.getKeyInt("infr:lv") > 0) {
        try notices.append(alloc.arena, .{});
    }
    txn.respond(.{ .Notices = notices });
}

pub fn onInfrLevelUpRequest(
    txn: *Transaction(pb.InfrLevelUpRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "infr:lvl", 1);
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "infr:exp", 100);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onInfrManualCancelTraceRoadRequest(
    txn: *Transaction(pb.InfrManualCancelTraceRoadRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "infr:trace_road", 0);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onInfrManualSwitchTraceRoadRequest(
    txn: *Transaction(pb.InfrManualSwitchTraceRoadRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "infr:trace_road", txn.message.RoadId);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onInfrPhoneTaskRewardRequest(
    txn: *Transaction(pb.InfrPhoneTaskRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimIds(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "infr:phone", "InfrPhoneTask", txn.message.TargetIds.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onInfrRoadBuildRequest(
    txn: *Transaction(pb.InfrRoadBuildRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "infr:road:{d}", .{txn.message.RoadId});
    _ = try progress_comp.info.markClaim(alloc.gpa, k);
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "infr:exp", 50);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onInfrThemeActivityAwardRequest(
    txn: *Transaction(pb.InfrThemeActivityAwardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimIds(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "infr:theme", "InfrActivityTask", txn.message.TaskId.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onInfrV2LevelUpRequest(
    txn: *Transaction(pb.InfrV2LevelUpRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "infr2:lvl", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onInfrV2ManualCancelTraceTreeRequest(
    txn: *Transaction(pb.InfrV2ManualCancelTraceTreeRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "infr2:trace_tree", 0);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onInfrV2ManualSwitchTraceTreeRequest(
    txn: *Transaction(pb.InfrV2ManualSwitchTraceTreeRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "infr2:trace_tree", txn.message.TreeId);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onInfrV2ScoreRewardRequest(
    txn: *Transaction(pb.InfrV2ScoreRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimIds(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "infr2:score", "InfrV2ScoreReward", txn.message.ScoreIds.items);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onInfrV2TaskRewardRequest(
    txn: *Transaction(pb.InfrV2TaskRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimIds(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "infr2:task", "InfrV2Reward", txn.message.TaskIds.items);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onInfrV2TreeBuildRequest(
    txn: *Transaction(pb.InfrV2TreeBuildRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "infr2:tree:{d}", .{txn.message.Id});
    _ = try progress_comp.info.markClaim(alloc.gpa, k);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

// === SolarSchool ===
pub fn onSolarSchoolBatchSyncRequest(
    txn: *Transaction(pb.SolarSchoolBatchSyncRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [64]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "solar:day:{d}", .{txn.message.ActivityId});
    try progress_comp.info.setKeyInt(alloc.gpa, k, txn.message.GameDayNow);
    for (txn.message.SyncData.items) |entry| {
        if (entry.AttrChange) |ac| {
            const ak = try std.fmt.bufPrint(&kbuf, "solar:attr:{d}:{d}", .{
                txn.message.ActivityId,
                if (ac.AttrType) |t| @intFromEnum(t) else 0,
            });
            _ = try progress_comp.info.addKeyInt(alloc.gpa, ak, ac.Delta);
        }
        if (entry.UnlockNpc) |un| {
            for (un.NpcId.items) |nid| {
                const nk = try std.fmt.bufPrint(&kbuf, "solar:npc:{d}:{d}", .{ txn.message.ActivityId, nid });
                _ = try progress_comp.info.markClaim(alloc.gpa, nk);
            }
        }
        if (entry.NpcFavorChange) |nfc| {
            const fk = try std.fmt.bufPrint(&kbuf, "solar:favor:{d}:{d}", .{ txn.message.ActivityId, nfc.NpcId });
            try progress_comp.info.setKeyInt(alloc.gpa, fk, nfc.FavorAbsolute);
        }
        if (entry.JoinInfluence) |ji| {
            for (ji.InfluenceId.items) |iid| {
                const ik = try std.fmt.bufPrint(&kbuf, "solar:inf:{d}:{d}", .{ txn.message.ActivityId, iid });
                _ = try progress_comp.info.markClaim(alloc.gpa, ik);
            }
        }
        if (entry.ExamFinish) |ef| {
            const ek = try std.fmt.bufPrint(&kbuf, "solar:exam:{d}:{d}", .{ txn.message.ActivityId, ef.ExamId });
            try progress_comp.info.setKeyInt(alloc.gpa, ek, ef.Score);
        }
        if (entry.ShopBuy) |sb| {
            const sk = try std.fmt.bufPrint(&kbuf, "solar:shop:{d}:{d}:{d}", .{ txn.message.ActivityId, sb.ShopId, sb.GoodsId });
            _ = try progress_comp.info.addKeyInt(alloc.gpa, sk, sb.Count);
        }
        if (entry.CompleteQuest) |cq| {
            for (cq.QuestId.items) |qid| {
                const qk = try std.fmt.bufPrint(&kbuf, "solar:quest:{d}:{d}", .{ txn.message.ActivityId, qid });
                _ = try progress_comp.info.markClaim(alloc.gpa, qk);
            }
        }
        if (entry.ItemChange) |ic| {
            if (ic.Item) |it| {
                const itk = try std.fmt.bufPrint(&kbuf, "solar:item:{d}:{d}", .{ txn.message.ActivityId, it.ItemId });
                _ = try progress_comp.info.addKeyInt(alloc.gpa, itk, @as(i64, it.ItemAdd) - @as(i64, it.ItemCost));
            }
        }
        if (entry.Behave) |bh| {
            const bk = try std.fmt.bufPrint(&kbuf, "solar:behave:{d}:{d}", .{ txn.message.ActivityId, bh.BehaviorId });
            try progress_comp.info.setKeyInt(alloc.gpa, bk, if (bh.IsSuccess) 1 else 0);
        }
    }
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success, .FailedIndex = -1 });
}

pub fn onSolarSchoolLoadRequest(
    txn: *Transaction(pb.SolarSchoolLoadRequest),
    alloc: mem.Alloc,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const slot_key = try std.fmt.bufPrint(&kbuf, "solar:save:{d}:{d}", .{ txn.message.ActivityId, txn.message.SlotId });
    var loaded = pb_kv.loadMsg(alloc.arena, progress_comp, slot_key, pb.SolarSchoolSavePb);
    if (loaded == null) {
        const latest_key = try std.fmt.bufPrint(&kbuf, "solar:save:{d}", .{txn.message.ActivityId});
        loaded = pb_kv.loadMsg(alloc.arena, progress_comp, latest_key, pb.SolarSchoolSavePb);
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .SavePb = loaded orelse .{ .ActivityId = txn.message.ActivityId },
    });
}

pub fn onSolarSchoolOneKeyGalleryRewardRequest(
    txn: *Transaction(pb.SolarSchoolOneKeyGalleryRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimIds(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "solar:gallery", "SolarSchoolEnding", txn.message.Id.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSolarSchoolOneKeyRewardRequest(
    txn: *Transaction(pb.SolarSchoolOneKeyRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimIds(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "solar:task", "SolarSchoolTask", txn.message.TaskId.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSolarSchoolReachEndingRequest(
    txn: *Transaction(pb.SolarSchoolReachEndingRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    for (txn.message.Id.items) |id| {
        const k = try std.fmt.bufPrint(&kbuf, "solar:ending:{d}", .{id});
        _ = try progress_comp.info.markClaim(alloc.gpa, k);
    }
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSolarSchoolSaveRequest(
    txn: *Transaction(pb.SolarSchoolSaveRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "solar:slot:{d}", .{txn.message.ActivityId});
    try progress_comp.info.setKeyInt(alloc.gpa, k, txn.message.LoadSourceSlotId);
    if (txn.message.SavePb) |save_pb| {
        const latest_key = try std.fmt.bufPrint(&kbuf, "solar:save:{d}", .{txn.message.ActivityId});
        try pb_kv.saveMsg(alloc, fs, progress_comp, latest_key, save_pb);
        if (txn.message.LoadSourceSlotId >= 0) {
            const slot_key = try std.fmt.bufPrint(&kbuf, "solar:save:{d}:{d}", .{ txn.message.ActivityId, txn.message.LoadSourceSlotId });
            try pb_kv.saveMsg(alloc, fs, progress_comp, slot_key, save_pb);
        }
    } else {
        try progress_comp.save(fs, alloc.arena);
    }
    var flows: std.ArrayList(pb.SolarSchoolFlowPb) = .empty;
    for (progress_comp.info.dynamic.claims) |c| {
        if (std.mem.startsWith(u8, c, "solar:flow:")) {
            if (std.fmt.parseInt(i32, c["solar:flow:".len..], 10)) |fid| {
                const fpk = try std.fmt.bufPrint(&kbuf, "solar:flow_pb:{d}", .{fid});
                const fpb = pb_kv.loadMsg(alloc.arena, progress_comp, fpk, pb.SolarSchoolFlowPb) orelse pb.SolarSchoolFlowPb{ .SolarSchoolFlowId = fid };
                try flows.append(alloc.arena, fpb);
            } else |_| {}
        }
    }
    txn.respond(.{ .ErrorCode = .Success, .Flow = flows });
}

pub fn onSolarSchoolSetFlowRequest(
    txn: *Transaction(pb.SolarSchoolSetFlowRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "solar:flow:{d}", .{txn.message.SolarSchoolFlowId});
    _ = try progress_comp.info.markClaim(alloc.gpa, k);
    const fpk = try std.fmt.bufPrint(&kbuf, "solar:flow_pb:{d}", .{txn.message.SolarSchoolFlowId});
    var flow_pb = pb_kv.loadMsg(alloc.arena, progress_comp, fpk, pb.SolarSchoolFlowPb) orelse pb.SolarSchoolFlowPb{
        .SolarSchoolFlowId = txn.message.SolarSchoolFlowId,
    };
    flow_pb.SolarSchoolFlowId = txn.message.SolarSchoolFlowId;
    for (txn.message.ReadPlot.items) |pid| {
        if (pid < 0) continue;
        const word_idx: usize = @intCast(@divTrunc(pid, 64));
        const bit_idx: u6 = @intCast(@mod(pid, 64));
        while (flow_pb.ReadPlotFlatBit.items.len <= word_idx) {
            try flow_pb.ReadPlotFlatBit.append(alloc.arena, 0);
        }
        const cur: u64 = @bitCast(flow_pb.ReadPlotFlatBit.items[word_idx]);
        flow_pb.ReadPlotFlatBit.items[word_idx] = @bitCast(cur | (@as(u64, 1) << bit_idx));
    }
    try pb_kv.saveMsg(alloc, fs, progress_comp, fpk, flow_pb);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSolarSchoolTaskProgressRewardRequest(
    txn: *Transaction(pb.SolarSchoolTaskProgressRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const single = [_]i32{txn.message.ActivityId};
    _ = try claimIds(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "solar:prog", "SolarSchoolActivity", &single);
    txn.respond(.{
        .ErrorCode = .Success,
        .Activity = .{ .LimitTaskProgressRewarded = true },
    });
}

// === TrackMoon ===
pub fn onTrackMoonActivityRewardRequest(
    txn: *Transaction(pb.TrackMoonActivityRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const single = [_]i32{txn.message.Id};
    _ = try claimIds(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "trackmoon:act", "TrackMoonActivityReward", &single);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onTrackMoonBuildFlowRequest(
    txn: *Transaction(pb.TrackMoonBuildFlowRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "trackmoon:bld:{d}", .{txn.message.BuildingId});
    _ = try progress_comp.info.addKeyInt(alloc.gpa, k, 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onTrackMoonBuildingDataRequest(
    txn: *Transaction(pb.TrackMoonBuildingDataRequest),
    alloc: mem.Alloc,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    var infos: std.ArrayList(pb.BuildingInfo) = .empty;
    var kbuf: [48]u8 = undefined;
    var id: i32 = 1;
    while (id <= 10) : (id += 1) {
        const k = try std.fmt.bufPrint(&kbuf, "trackmoon:bld:{d}", .{id});
        const raw = progress_comp.info.getKeyInt(k);
        const lvl: i32 = @intCast(@max(@as(i64, if (id == 1) 1 else 0), raw));
        try infos.append(alloc.arena, .{
            .BuildingId = id,
            .Level = lvl,
            .IsUnlock = (raw > 0 or id == 1),
        });
    }
    txn.respond(.{ .BuildingInfos = infos });
}
pub fn onTrackMoonEntrustDataRequest(
    txn: *Transaction(pb.TrackMoonEntrustDataRequest),
    alloc: mem.Alloc,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    var infos: std.ArrayList(pb.EntrustInfo) = .empty;
    var kbuf: [48]u8 = undefined;
    var id: i32 = 1;
    while (id <= 18) : (id += 1) {
        const k = try std.fmt.bufPrint(&kbuf, "trackmoon:entrust:{d}", .{id});
        const lvl: i32 = @intCast(@max(@as(i64, 0), progress_comp.info.getKeyInt(k)));
        try infos.append(alloc.arena, .{
            .EntrustId = id,
            .IsVisiable = true,
            .HistoryLevel = lvl,
        });
    }
    txn.respond(.{ .EntrustInfos = infos });
}
pub fn onTrackMoonIllustratedRequest(
    txn: *Transaction(pb.TrackMoonIllustratedRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    var dirty = false;
    for (txn.message.Ids.items) |id| {
        const k = try std.fmt.bufPrint(&kbuf, "trackmoon:illus:{d}", .{id});
        if (try progress_comp.info.markClaim(alloc.gpa, k)) dirty = true;
    }
    if (dirty) try progress_comp.save(fs, alloc.arena);
    var out_ids: std.ArrayList(i32) = .empty;
    for (progress_comp.info.dynamic.claims) |c| {
        if (std.mem.startsWith(u8, c, "trackmoon:illus:")) {
            if (std.fmt.parseInt(i32, c["trackmoon:illus:".len..], 10)) |id| {
                try out_ids.append(alloc.arena, id);
            } else |_| {}
        }
    }
    txn.respond(.{ .Ids = if (out_ids.items.len > 0) out_ids else txn.message.Ids });
}
pub fn onTrackMoonRoleDataRequest(
    txn: *Transaction(pb.TrackMoonRoleDataRequest),
    alloc: mem.Alloc,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    var roles: std.ArrayList(pb.EntrustRoleInfo) = .empty;
    var kbuf: [48]u8 = undefined;
    var id: i32 = 1;
    while (id <= 14) : (id += 1) {
        const k = try std.fmt.bufPrint(&kbuf, "trackmoon:role_lvl:{d}", .{id});
        const lvl: i32 = @intCast(@max(@as(i64, 1), progress_comp.info.getKeyInt(k)));
        try roles.append(alloc.arena, .{
            .Level = lvl,
            .RoleAttr = .{
                .RoleId = id,
                .Level = lvl,
                .PropertyA = @intCast(@max(@as(i64, 100), progress_comp.info.getKeyInt(try std.fmt.bufPrint(&kbuf, "trackmoon:role_a:{d}", .{id})))),
                .PropertyB = @intCast(@max(@as(i64, 100), progress_comp.info.getKeyInt(try std.fmt.bufPrint(&kbuf, "trackmoon:role_b:{d}", .{id})))),
                .PropertyC = @intCast(@max(@as(i64, 100), progress_comp.info.getKeyInt(try std.fmt.bufPrint(&kbuf, "trackmoon:role_c:{d}", .{id})))),
            },
        });
    }
    txn.respond(.{ .EntrustRoleInfos = roles });
}
pub fn onTrackMoonSettleRequest(
    txn: *Transaction(pb.TrackMoonSettleRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    const cnt: i32 = @intCast(try progress_comp.info.addKeyInt(alloc.gpa, "trackmoon:settle_cnt", 1));
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "trackmoon:popular", 10);
    try progress_comp.save(fs, alloc.arena);
    const entrust_do: i32 = @intCast(@max(@as(i64, cnt), progress_comp.info.getKeyInt("trackmoon:entrust_done")));
    const wish_total: i32 = @intCast(@max(@as(i64, cnt * 100), progress_comp.info.getKeyInt("trackmoon:wish")));
    const gold_total: i32 = @intCast(@max(@as(i64, cnt * 1000), progress_comp.info.getKeyInt("trackmoon:gold")));
    const pop_total: i32 = @intCast(@max(@as(i64, cnt * 500), progress_comp.info.getKeyInt("trackmoon:popular")));
    const fav_id: i32 = @intCast(@max(@as(i64, 1), progress_comp.info.getKeyInt("trackmoon:fav_role")));
    txn.respond(.{
        .ErrorCode = .Success,
        .MostLikeRole = .{ .Id = fav_id, .Num = cnt },
        .EntrustDoMost = .{ .Id = 1, .Num = entrust_do },
        .EntrustDo = entrust_do,
        .AlwaysRoleId = .{ .Id = fav_id, .Num = entrust_do },
        .WishTotal = wish_total,
        .GoldTotal = gold_total,
        .PopularTotal = pop_total,
    });
}
pub fn onTrackMoonTargetRewardRequest(
    txn: *Transaction(pb.TrackMoonTargetRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const single = [_]i32{txn.message.Id};
    _ = try claimIds(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "trackmoon:target", "TrackMoonTarget", &single);
    txn.respond(.{ .ErrorCode = .Success });
}

// === Recall ===
pub fn onStartRecallRequest(
    txn: *Transaction(pb.StartRecallRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    try progress_comp.info.setKeyInt(alloc.gpa, "recall:cur", txn.message.RecallId);
    const k = try std.fmt.bufPrint(&kbuf, "recall:started:{d}", .{txn.message.RecallId});
    _ = try progress_comp.info.markClaim(alloc.gpa, k);
    try progress_comp.save(fs, alloc.arena);
    const inst_k = try std.fmt.bufPrint(&kbuf, "recall:inst:{d}", .{txn.message.RecallId});
    const saved_inst: i32 = @intCast(@max(0, progress_comp.info.getKeyInt(inst_k)));
    const default_inst: i32 = if (assets.tables.recall_config.getDataById(txn.message.RecallId)) |rc|
        (if (rc.DungeonIds.len > 0) rc.DungeonIds[0] else 0)
    else
        0;
    const enter_inst_id: i32 = if (txn.message.Continue and saved_inst > 0) saved_inst else default_inst;
    txn.respond(.{
        .ErrorCode = .Success,
        .CanContinue = txn.message.Continue,
        .EnterInstId = enter_inst_id,
    });
}
pub fn onQueryRecallItemBagRequest(
    txn: *Transaction(pb.QueryRecallItemBagRequest),
    alloc: mem.Alloc,
    progress_comp: *PlayerProgressComponent,
) !void {
    var items: std.ArrayList(pb.RecallItemBagItem) = .empty;
    var pbuf: [48]u8 = undefined;
    const prefix = try std.fmt.bufPrint(&pbuf, "recall:item:{d}:", .{txn.message.RecallId});
    for (progress_comp.info.dynamic.ints) |entry| {
        if (std.mem.startsWith(u8, entry.key, prefix) and entry.val > 0) {
            if (std.fmt.parseInt(i32, entry.key[prefix.len..], 10)) |item_id| {
                try items.append(alloc.arena, .{ .ItemId = item_id, .Count = @intCast(entry.val) });
            } else |_| {}
        }
    }
    txn.respond(.{ .Items = items, .ErrorCode = .Success });
}
pub fn onTraceRecallQuestRequest(
    txn: *Transaction(pb.TraceRecallQuestRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "recall:trace:{d}", .{txn.message.RecallQuestId});
    try progress_comp.info.setKeyInt(alloc.gpa, k, txn.message.Operate);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorId = .Success });
}
pub fn onRecallExploreSkillRouletteSetRequest(
    txn: *Transaction(pb.RecallExploreSkillRouletteSetRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "recall:roulette", txn.message.ExtraItemId);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorId = .Success });
}
pub fn onRecallItemUseRequest(
    txn: *Transaction(pb.RecallItemUseRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "recall:item_used:{d}", .{txn.message.ItemId});
    _ = try progress_comp.info.addKeyInt(alloc.gpa, k, @max(1, txn.message.Count));
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrCode = .Success });
}
pub fn onRecallQuestActionRequest(
    txn: *Transaction(pb.RecallQuestActionRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [64]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "recall:qact:{d}:{d}:{d}", .{
        txn.message.RecallQuestId,
        txn.message.NodeId,
        txn.message.ActionId,
    });
    _ = try progress_comp.info.markClaim(alloc.gpa, k);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorId = .Success });
}
pub fn onRecallQuestFinishActionRequest(
    txn: *Transaction(pb.RecallQuestFinishActionRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [64]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "recall:qfin:{d}:{d}:{d}", .{
        txn.message.RecallQuestId,
        txn.message.NodeId,
        txn.message.ActionId,
    });
    _ = try progress_comp.info.markClaim(alloc.gpa, k);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorId = .Success });
}
pub fn onRecallQuestFinishActionEndRequest(
    txn: *Transaction(pb.RecallQuestFinishActionEndRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    var dirty = false;
    for (txn.message.RecallQuestId.items) |qid| {
        const k = try std.fmt.bufPrint(&kbuf, "recall:qend:{d}", .{qid});
        if (try progress_comp.info.markClaim(alloc.gpa, k)) dirty = true;
    }
    if (dirty) try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorId = .Success });
}
pub fn onRecallQuestNpcMoveOverRequest(
    txn: *Transaction(pb.RecallQuestNpcMoveOverRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "recall:npcmove:{d}", .{txn.message.EntityId});
    _ = try progress_comp.info.markClaim(alloc.gpa, k);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorId = .Success });
}
pub fn onRecallQuestReadyForGiveUpRequest(
    txn: *Transaction(pb.RecallQuestReadyForGiveUpRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "recall:giveup:{d}", .{txn.message.RecallQuestId});
    _ = try progress_comp.info.markClaim(alloc.gpa, k);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorId = .Success });
}
pub fn onRecallTeleportToMarkIdRequest(
    txn: *Transaction(pb.RecallTeleportToMarkIdRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "recall:tp_mark", txn.message.MarkId);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorId = .Success });
}

// === RoadBook & RealmBetween ===
pub fn onRoadBookFullRewardRequest(
    txn: *Transaction(pb.RoadBookFullRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    if (try progress_comp.info.markClaim(alloc.gpa, "roadbook:full")) {
        try progress_comp.save(fs, alloc.arena);
        _ = try RewardService.grantItems(reward_ctx, 0, true, &.{.{ .item_id = 3, .count = 160 }}, 1);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRoadBookLevelUpRequest(
    txn: *Transaction(pb.RoadBookLevelUpRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "roadbook:lvl", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRoadBookMotorRewardRequest(
    txn: *Transaction(pb.RoadBookMotorRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const claimed = try claimIds(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "roadbook:motor", "RoadBookTask", txn.message.TaskIds.items);
    txn.respond(.{ .ErrorCode = .Success, .TaskIds = claimed });
}

pub fn onRoadBookTaskRewardRequest(
    txn: *Transaction(pb.RoadBookTaskRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const claimed = try claimIds(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "roadbook:task", "RoadBookTask", txn.message.TaskIds.items);
    txn.respond(.{ .ErrorCode = .Success, .TaskIds = claimed });
}

pub fn onRealmBetweenFullRewardRequest(
    txn: *Transaction(pb.RealmBetweenFullRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    if (try progress_comp.info.markClaim(alloc.gpa, "realmbetween:full")) {
        try progress_comp.save(fs, alloc.arena);
        _ = try RewardService.grantItems(reward_ctx, 0, true, &.{.{ .item_id = 3, .count = 160 }}, 1);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRealmBetweenLevelUpRequest(
    txn: *Transaction(pb.RealmBetweenLevelUpRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "realmbetween:lvl", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRealmBetweenMotorRewardRequest(
    txn: *Transaction(pb.RealmBetweenMotorRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const claimed = try claimIds(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "realmbetween:motor", "RoadBookTask", txn.message.TaskIds.items);
    txn.respond(.{ .ErrorCode = .Success, .TaskIds = claimed });
}

pub fn onRealmBetweenTaskRewardRequest(
    txn: *Transaction(pb.RealmBetweenTaskRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const claimed = try claimIds(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "realmbetween:task", "RoadBookTask", txn.message.TaskIds.items);
    txn.respond(.{ .ErrorCode = .Success, .TaskIds = claimed });
}

// === Newbie / NewJourney / NewPlayerSupport / ThroughTrain ===
pub fn onNewBieCourseRewardRequest(
    txn: *Transaction(pb.NewBieCourseRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const single = [_]i32{txn.message.Level};
    _ = try claimIds(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "newbie:course", "NewBieCourse", &single);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onNewJourneyRequest(
    txn: *Transaction(pb.NewJourneyRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
    basic_comp: *const PlayerBasicComponent,
    echo_comp: *const PlayerEchoComponent,
    inventory_comp: *const PlayerInventoryComponent,
    map_comp: *const PlayerMapComponent,
    quest_comp: *const PlayerQuestComponent,
    progress_comp: *const PlayerProgressComponent,
) !void {
    const cond_ctx: ConditionEvaluator.EvalContext = .{
        .assets = assets,
        .basic_comp = basic_comp,
        .echo_comp = echo_comp,
        .inventory_comp = inventory_comp,
        .map_comp = map_comp,
        .quest_comp = quest_comp,
        .progress_comp = progress_comp,
    };
    var tasks: std.ArrayList(pb.NewJourneyTask) = .empty;
    var kbuf: [48]u8 = undefined;
    if (txn.message.TaskIds.items.len > 0) {
        for (txn.message.TaskIds.items) |id| {
            const cfg = assets.tables.world_new_journey.getDataById(id);
            const cond_ok = if (cfg) |c| (c.ConditionId <= 0 or ConditionEvaluator.evalConditionOrGroup(cond_ctx, c.ConditionId)) else true;
            const k = try std.fmt.bufPrint(&kbuf, "newbie:journey:{d}", .{id});
            const claimed = progress_comp.info.hasClaim(k);
            try tasks.append(alloc.arena, .{
                .TaskIdx = id,
                .CanJump = cond_ok,
                .Complete = claimed or (cfg != null and cfg.?.ConditionId > 0 and cond_ok),
            });
        }
    } else {
        for (assets.tables.world_new_journey.items) |cfg| {
            const cond_ok = cfg.ConditionId <= 0 or ConditionEvaluator.evalConditionOrGroup(cond_ctx, cfg.ConditionId);
            const k = try std.fmt.bufPrint(&kbuf, "newbie:journey:{d}", .{cfg.Id});
            const claimed = progress_comp.info.hasClaim(k);
            try tasks.append(alloc.arena, .{
                .TaskIdx = cfg.Id,
                .CanJump = cond_ok,
                .Complete = claimed or (cfg.ConditionId > 0 and cond_ok),
            });
        }
    }
    txn.respond(.{ .NewJourneyTask = tasks });
}

pub fn onNewPlayerSupportRewardTaskRequest(
    txn: *Transaction(pb.NewPlayerSupportRewardTaskRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimIds(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "newbie:support_task", "NewPlayerSupportTask", txn.message.TaskIds.items);
    txn.respond(.{ .Code = .Success });
}

fn buildSupportTrialRoleInfo(
    arena: std.mem.Allocator,
    assets: *const Assets,
    progress_comp: *const PlayerProgressComponent,
    lv_prefix: []const u8,
    trial_role_id: i32,
) !pb.roleInfo {
    var kbuf: [64]u8 = undefined;
    const lv_key = try std.fmt.bufPrint(&kbuf, "{s}:{d}", .{ lv_prefix, trial_role_id });
    const stored_lv = progress_comp.info.getKeyInt(lv_key);
    const trial_opt = assets.tables.trial_role_info.getDataById(trial_role_id);
    const lv: i32 = if (stored_lv > 0)
        @intCast(@min(@as(i64, 90), stored_lv))
    else if (trial_opt) |t|
        @max(1, t.Level)
    else
        80;

    var base_props: std.ArrayList(pb.ArrayIntInt) = .empty;
    var add_props: std.ArrayList(pb.ArrayIntInt) = .empty;
    var breach: i32 = 0;
    var star: i32 = 4;
    var skin_id: i32 = 0;
    if (assets.tables.resolveBaseRoleInfo(trial_role_id)) |base_cfg| {
        star = @max(4, base_cfg.QualityId);
        skin_id = if (trial_opt) |t| (if (t.RoleSkin != 0) t.RoleSkin else base_cfg.SkinId) else base_cfg.SkinId;
        for (assets.tables.role_breach.items) |entry| {
            if (entry.BreachGroupId != base_cfg.BreachId) continue;
            if (lv > entry.MaxLevel and entry.BreachLevel + 1 > breach) {
                breach = entry.BreachLevel + 1;
            }
        }
        var prop_buf: [@intFromEnum(pb.EAttributeType.MAX)]i32 = undefined;
        if (RoleInfo.computeScaledBaseProps(assets, base_cfg.Id, lv, breach, &prop_buf)) {
            for (prop_buf, 0..) |val, idx| {
                try base_props.append(arena, .{ .Key = @intCast(idx), .Value = val });
                try add_props.append(arena, .{ .Key = @intCast(idx), .Value = 0 });
            }
        }
    }
    return .{
        .RoleId = trial_role_id,
        .Level = lv,
        .Breakthrough = breach,
        .Star = star,
        .SkinId = skin_id,
        .BaseProp = base_props,
        .AddProp = add_props,
    };
}

pub fn onNewPlayerSupportSetCurUseTrialRoleRequest(
    txn: *Transaction(pb.NewPlayerSupportSetCurUseTrialRoleRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
) !void {
    const prev: i32 = @intCast(progress_comp.info.getKeyInt("newbie:support_role"));
    try progress_comp.info.setKeyInt(alloc.gpa, "newbie:support_role", txn.message.TrialRoleId);
    try progress_comp.save(fs, alloc.arena);
    const info = try buildSupportTrialRoleInfo(alloc.arena, assets, progress_comp, "newbie:support_role_lv", txn.message.TrialRoleId);
    txn.respond(.{
        .Code = .Success,
        .RoleId = txn.message.TrialRoleId,
        .ReplacedRoleId = prev,
        .ReplacedCurRole = prev != 0 and prev != txn.message.TrialRoleId,
        .CurUseRoleInfo = info,
    });
}

pub fn onNewPlayerSupportSetCurUseTrialRoleV2Request(
    txn: *Transaction(pb.NewPlayerSupportSetCurUseTrialRoleV2Request),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
) !void {
    const prev: i32 = @intCast(progress_comp.info.getKeyInt("newbie:support_role_v2"));
    try progress_comp.info.setKeyInt(alloc.gpa, "newbie:support_role_v2", txn.message.TrialRoleId);
    try progress_comp.save(fs, alloc.arena);
    const info = try buildSupportTrialRoleInfo(alloc.arena, assets, progress_comp, "newbie:support_role_v2_lv", txn.message.TrialRoleId);
    txn.respond(.{
        .Code = .Success,
        .RoleId = txn.message.TrialRoleId,
        .ReplacedRoleId = prev,
        .ReplacedCurRole = prev != 0 and prev != txn.message.TrialRoleId,
        .CurUseRoleInfo = info,
    });
}

pub fn onNewPlayerSupportTrialRoleLvUpRequest(
    txn: *Transaction(pb.NewPlayerSupportTrialRoleLvUpRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "newbie:support_role_lv:{d}", .{txn.message.TrialRoleId});
    const prev = progress_comp.info.getKeyInt(k);
    const next_lv: i32 = @intCast(@min(@as(i64, 90), @max(@as(i64, 1), prev) + 10));
    try progress_comp.info.setKeyInt(alloc.gpa, k, next_lv);
    try progress_comp.save(fs, alloc.arena);
    const info = try buildSupportTrialRoleInfo(alloc.arena, assets, progress_comp, "newbie:support_role_lv", txn.message.TrialRoleId);
    txn.respond(.{
        .Code = .Success,
        .RoleId = txn.message.TrialRoleId,
        .CurUseRoleInfo = info,
    });
}

pub fn onNewPlayerSupportTrialRoleLvUpV2Request(
    txn: *Transaction(pb.NewPlayerSupportTrialRoleLvUpV2Request),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "newbie:support_role_v2_lv:{d}", .{txn.message.TrialRoleId});
    const prev = progress_comp.info.getKeyInt(k);
    const next_lv: i32 = @intCast(@min(@as(i64, 90), @max(@as(i64, 1), prev) + 10));
    try progress_comp.info.setKeyInt(alloc.gpa, k, next_lv);
    try progress_comp.save(fs, alloc.arena);
    const info = try buildSupportTrialRoleInfo(alloc.arena, assets, progress_comp, "newbie:support_role_v2_lv", txn.message.TrialRoleId);
    txn.respond(.{
        .RoleId = txn.message.TrialRoleId,
        .Code = .Success,
        .CurUseRoleInfo = info,
    });
}

pub fn onNewTrialRoleGetNightmarePhantomInstInfoRequest(
    txn: *Transaction(pb.NewTrialRoleGetNightmarePhantomInstInfoRequest),
    alloc: mem.Alloc,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    var ins_list: std.ArrayList(pb.NightmarePhantomInstInfo) = .empty;
    var save_ids: std.ArrayList(i32) = .empty;
    for (progress_comp.info.dynamic.ints) |entry| {
        if (std.mem.startsWith(u8, entry.key, "newbie:nightmare_kill:")) {
            if (std.fmt.parseInt(i32, entry.key["newbie:nightmare_kill:".len..], 10)) |ins_id| {
                try ins_list.append(alloc.arena, .{ .InsId = ins_id, .KillNum = entry.val });
            } else |_| {}
        } else if (std.mem.startsWith(u8, entry.key, "newbie:nightmare_save:") and entry.val > 0) {
            if (std.fmt.parseInt(i32, entry.key["newbie:nightmare_save:".len..], 10)) |ins_id| {
                try save_ids.append(alloc.arena, ins_id);
            } else |_| {}
        }
    }
    txn.respond(.{
        .Code = .Success,
        .InsInfoList = ins_list,
        .SaveInsIdList = save_ids,
    });
}

pub fn onNewbieAdventureChapterDropRequest(
    txn: *Transaction(pb.NewbieAdventureChapterDropRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const single = [_]i32{txn.message.ChapterId};
    _ = try claimIds(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "newbie:adv_chap", "AdventureTaskChapter", &single);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onNewbieAdventureRoleRewardRequest(
    txn: *Transaction(pb.NewbieAdventureRoleRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const single = [_]i32{txn.message.RoleId};
    _ = try claimIds(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "newbie:adv_role", "AdventureRole", &single);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onNewbieAdventureTaskRewardRequest(
    txn: *Transaction(pb.NewbieAdventureTaskRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimIds(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "newbie:adv_task", "AdventureTask", txn.message.TaskIds.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onNewbieCarnivalAwardRequest(
    txn: *Transaction(pb.NewbieCarnivalAwardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const single = [_]i32{txn.message.Config};
    _ = try claimIds(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "newbie:carnival", "NewbieCarnival", &single);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onNewbieCarnivalSwitchRoleRequest(
    txn: *Transaction(pb.NewbieCarnivalSwitchRoleRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "newbie:carnival_role", txn.message.RoleId);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onNewbieCourseV2RewardRequest(
    txn: *Transaction(pb.NewbieCourseV2RewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimIds(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "newbie:course2", "NewbieCourseV2", txn.message.Level.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onNewbieFetchScoreRewardRequest(
    txn: *Transaction(pb.NewbieFetchScoreRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimIds(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "newbie:score", "NewbieScore", txn.message.ScoreRewardIds.items);
    txn.respond(.{ .Code = .Success });
}

pub fn onThroughTrainRequest(
    txn: *Transaction(pb.ThroughTrainRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "throughtrain:entered:{d}", .{txn.message.ActivityId});
    _ = try progress_comp.info.markClaim(alloc.gpa, k);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onThroughTrainFinishViewRequest(
    txn: *Transaction(pb.ThroughTrainFinishViewRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "throughtrain:{d}", .{txn.message.ActivityId});
    _ = try progress_comp.info.markClaim(alloc.gpa, k);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}
