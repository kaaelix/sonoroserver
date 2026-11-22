const std = @import("std");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const common = @import("common");
const FileSystem = common.FileSystem;
const Transaction = @import("../handlers.zig").Transaction;
const PlayerProgressComponent = @import("../../logic/component/player/PlayerProgressComponent.zig");
const RewardService = @import("../../logic/helpers/reward.zig");
const Assets = @import("../../data/Assets.zig");
const EnergyHelper = @import("../../logic/helpers/energy.zig");
const pb_kv = @import("pb_kv.zig");

fn rewardToMapEntries(
    reward_ctx: RewardService.RewardContext,
    table_name: []const u8,
    key: i64,
    fallback: []const RewardService.RewardItem,
    arena: std.mem.Allocator,
) !std.ArrayList(pb.MapEntry(i32, i32)) {
    var granted: std.ArrayList(RewardService.RewardItem) = .empty;
    _ = try RewardService.grantTableReward(reward_ctx, table_name, key, false, true, 19500, fallback, &granted);
    var map: std.ArrayList(pb.MapEntry(i32, i32)) = .empty;
    for (granted.items) |it| {
        try map.append(arena, .{ .key = it.item_id, .value = it.count });
    }
    return map;
}

// =========================================================================
// Classic Roguelike (1.x / Somnoire) & Popular Entries / Archive
// =========================================================================

pub fn onRoguelikeStartRequest(
    txn: *Transaction(pb.RoguelikeStartRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "rogue:season", txn.message.SeasonId);
    try progress_comp.info.setKeyInt(alloc.gpa, "rogue:inst", txn.message.InstId);
    try progress_comp.info.setKeyInt(alloc.gpa, "rogue:hot_group", txn.message.SelectHotEntryGroupId);
    if (!txn.message.ContinueLastProgress) {
        try progress_comp.info.setKeyInt(alloc.gpa, "rogue:layer", 1);
        try progress_comp.info.setKeyInt(alloc.gpa, "rogue:gain_refresh", 0);
        try progress_comp.info.setKeyInt(alloc.gpa, "rogue:gain_count", 0);
    }
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRoguelikeLastInfoRequest(
    txn: *Transaction(pb.RoguelikeLastInfoRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    const inst_id: i32 = @intCast(progress_comp.info.getKeyInt("rogue:inst"));
    const cur_layer: i32 = @intCast(@max(1, progress_comp.info.getKeyInt("rogue:layer")));
    var infos: std.ArrayList(pb.RoguelikeLastInfo) = .empty;
    if (inst_id > 0) {
        try infos.append(alloc.arena, .{
            .InstId = inst_id,
            .CurLayer = cur_layer,
            .MaxLayer = 10,
        });
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .RoguelikeLastInfos = infos,
        .HasData = inst_id > 0,
    });
}

pub fn onRoguelikeChooseDataRequest(
    txn: *Transaction(pb.RoguelikeChooseDataRequest),
    progress_comp: *const PlayerProgressComponent,
) !void {
    const layer = if (txn.message.Layer > 0) txn.message.Layer else @as(i32, @intCast(@max(1, progress_comp.info.getKeyInt("rogue:layer"))));
    const used: i32 = @intCast(std.math.clamp(progress_comp.info.getKeyInt("rogue:gain_refresh"), 0, 3));
    txn.respond(.{
        .ErrorCode = .Success,
        .RoguelikeChooseData = .{
            .Index = txn.message.Index,
            .Layer = layer,
            .MaxTime = 3,
            .UseTime = used,
            .IsSelect = false,
        },
    });
}

pub fn onRoguelikeChooseDataResultRequest(
    txn: *Transaction(pb.RoguelikeChooseDataResultRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "rogue:gain:{d}:{d}", .{ txn.message.Layer, txn.message.BindId });
    try progress_comp.info.setKeyInt(alloc.gpa, key, txn.message.Index);
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "rogue:gain_count", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .ErrorCode = .Success,
        .IsSelect = true,
        .RoguelikeChooseDataNotify = .{ .RoguelikeChooseDataList = .empty },
    });
}

pub fn onRoguelikeRefreshGainRequest(
    txn: *Transaction(pb.RoguelikeRefreshGainRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const cur = @max(0, progress_comp.info.getKeyInt("rogue:gain_refresh"));
    if (cur >= 3) {
        txn.respond(.{
            .ErrorCode = .RequestParamError,
            .RoguelikeChooseData = .{
                .Index = txn.message.BindId,
                .Layer = txn.message.Layer,
                .MaxTime = 3,
                .UseTime = 3,
            },
        });
        return;
    }
    const cnt = try progress_comp.info.addKeyInt(alloc.gpa, "rogue:gain_refresh", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .ErrorCode = .Success,
        .RoguelikeChooseData = .{
            .Index = txn.message.BindId,
            .Layer = txn.message.Layer,
            .MaxTime = 3,
            .UseTime = @intCast(std.math.clamp(cnt, 0, 3)),
        },
    });
}

pub fn onRoguelikeGiveUpGainRequest(
    txn: *Transaction(pb.RoguelikeGiveUpGainRequest),
    alloc: mem.Alloc,
    reward_ctx: RewardService.RewardContext,
) !void {
    const fallback = [_]RewardService.RewardItem{.{ .item_id = 2, .count = 5000 }};
    const item_map = try rewardToMapEntries(reward_ctx, "RogueSeasonReward", txn.message.BindId, &fallback, alloc.arena);
    txn.respond(.{
        .ErrorCode = .Success,
        .BindId = txn.message.BindId,
        .ItemMap = item_map,
    });
}

pub fn onRoguelikeGotoNextRoomRequest(
    txn: *Transaction(pb.RoguelikeGotoNextRoomRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "rogue:layer", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRoguelikeQuitRequest(
    txn: *Transaction(pb.RoguelikeQuitRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "rogue:inst", 0);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRoguelikeResultRequest(
    txn: *Transaction(pb.RoguelikeResultRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
    io: std.Io,
) !void {
    const cur_layer: i32 = @intCast(@max(1, progress_comp.info.getKeyInt("rogue:layer")));
    const gain_count: i32 = @intCast(@max(0, progress_comp.info.getKeyInt("rogue:gain_count")));
    const fallback = [_]RewardService.RewardItem{.{ .item_id = 3, .count = 60 }, .{ .item_id = 2, .count = 20000 }};
    const rewards = try rewardToMapEntries(reward_ctx, "RogueSeasonReward", txn.message.InstId, &fallback, alloc.arena);
    try progress_comp.info.setKeyInt(alloc.gpa, "rogue:inst", 0);
    try progress_comp.save(fs, alloc.arena);

    txn.respond(.{
        .ErrorCode = .Success,
        .RoguelikeResultInfo = .{
            .InstId = txn.message.InstId,
            .CurLayer = cur_layer,
            .MaxLayer = 10,
            .GetGainCount = gain_count,
            .Rewards = rewards,
            .Time = @intCast(std.math.clamp(EnergyHelper.nowSeconds(io), 0, std.math.maxInt(u32))),
        },
    });
}

pub fn onRoguelikePopularEntriesInfoRequest(
    txn: *Transaction(pb.RoguelikePopularEntriesInfoRequest),
    progress_comp: *const PlayerProgressComponent,
    alloc: mem.Alloc,
) !void {
    var entries: std.ArrayList(i32) = .empty;
    const saved = progress_comp.info.getKeyInt("rogue:pop_entry");
    if (saved > 0) try entries.append(alloc.arena, @intCast(saved));
    txn.respond(.{
        .ErrorCode = .Success,
        .PopularEntrie = .{
            .InstId = txn.message.InstId,
            .SeasonId = txn.message.SeasonId,
            .Entries = entries,
        },
    });
}

pub fn onRoguelikePopularEntriesChangeRequest(
    txn: *Transaction(pb.RoguelikePopularEntriesChangeRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    if (txn.message.Entries.items.len > 0) {
        try progress_comp.info.setKeyInt(alloc.gpa, "rogue:pop_entry", txn.message.Entries.items[0]);
        try progress_comp.save(fs, alloc.arena);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRoguelikeSeasonRewardReceiveRequest(
    txn: *Transaction(pb.RoguelikeSeasonRewardReceiveRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var item_map: std.ArrayList(pb.MapEntry(i32, i32)) = .empty;
    for (txn.message.Index.items) |idx| {
        const key = try std.fmt.allocPrint(alloc.arena, "rogue:season_rw:{d}:{d}", .{ txn.message.SeasonId, idx });
        if (try progress_comp.info.markClaim(alloc.gpa, key)) {
            const fallback = [_]RewardService.RewardItem{.{ .item_id = 3, .count = 50 }, .{ .item_id = 2, .count = 15000 }};
            const m = try rewardToMapEntries(reward_ctx, "RogueSeasonReward", idx, &fallback, alloc.arena);
            try item_map.appendSlice(alloc.arena, m.items);
        }
    }
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .ErrorCode = .Success,
        .ItemMap = item_map,
    });
}

pub fn onRoguelikeTokenReceiveRequest(
    txn: *Transaction(pb.RoguelikeTokenReceiveRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "rogue:token:{d}:{d}", .{ txn.message.SeasonId, txn.message.Id });
    var item_map: std.ArrayList(pb.MapEntry(i32, i32)) = .empty;
    if (try progress_comp.info.markClaim(alloc.gpa, key)) {
        const fallback = [_]RewardService.RewardItem{.{ .item_id = 3, .count = 40 }};
        item_map = try rewardToMapEntries(reward_ctx, "RogueToken", txn.message.Id, &fallback, alloc.arena);
        try progress_comp.save(fs, alloc.arena);
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .ItemMap = item_map,
    });
}

pub fn onRoguelikeTalentInfoRequest(
    txn: *Transaction(pb.RoguelikeTalentInfoRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    var dict: std.ArrayList(pb.MapEntry(i32, i32)) = .empty;
    for (progress_comp.info.dynamic.ints) |entry| {
        if (std.mem.startsWith(u8, entry.key, "rogue:talent:")) {
            if (std.fmt.parseInt(i32, entry.key["rogue:talent:".len..], 10)) |sid| {
                try dict.append(alloc.arena, .{ .key = sid, .value = @intCast(entry.val) });
            } else |_| {}
        }
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .TalentSkillDict = dict,
    });
}

pub fn onRoguelikeTalentLevelUpRequest(
    txn: *Transaction(pb.RoguelikeTalentLevelUpRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "rogue:talent:{d}", .{txn.message.SkillId});
    const lv = try progress_comp.info.addKeyInt(alloc.gpa, key, 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .ErrorCode = .Success,
        .Level = @intCast(lv),
    });
}

pub fn onRoguelikeTrialRoleInfoRequest(
    txn: *Transaction(pb.RoguelikeTrialRoleInfoRequest),
    alloc: mem.Alloc,
) !void {
    var valid: std.ArrayList(i32) = .empty;
    try valid.appendSlice(alloc.arena, txn.message.TrialRoleIds.items);
    txn.respond(.{
        .ErrorCode = .Success,
        .ValidTrialRoleIds = valid,
        .RoleLevel = 90,
        .MaxLevel = 90,
        .WeaponLevel = 90,
    });
}

pub fn onRoguelikeRoleInstStartRequest(
    txn: *Transaction(pb.RoguelikeRoleInstStartRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "rogue:role_inst_idx", txn.message.Index);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRoguelikeRoleRoomSelectRequest(
    txn: *Transaction(pb.RoguelikeRoleRoomSelectRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "rogue:role_room:{d}", .{txn.message.RoleRoomId});
    try progress_comp.info.setKeyInt(alloc.gpa, key, txn.message.Index);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRogueInstExtraInfoRequest(
    txn: *Transaction(pb.RogueInstExtraInfoRequest),
    progress_comp: *const PlayerProgressComponent,
) !void {
    _ = progress_comp;
    txn.respond(.{
        .ErrorCode = .Success,
        .RogueInstExtraInfo = .{ .SeasonId = txn.message.SeasonId },
    });
}

pub fn onRogueArchiveSaveRequest(
    txn: *Transaction(pb.RogueArchiveSaveRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    io: std.Io,
) !void {
    const now_s = EnergyHelper.nowSeconds(io);
    const tkey = try std.fmt.allocPrint(alloc.arena, "rogue:arch_time:{d}", .{txn.message.SlotId});
    const ikey = try std.fmt.allocPrint(alloc.arena, "rogue:arch_inst:{d}", .{txn.message.SlotId});
    try progress_comp.info.setKeyInt(alloc.gpa, tkey, now_s);
    try progress_comp.info.setKeyInt(alloc.gpa, ikey, progress_comp.info.getKeyInt("rogue:inst"));
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .SaveTime = now_s,
        .ErrorCode = .Success,
    });
}

pub fn onRogueArchiveListRequest(
    txn: *Transaction(pb.RogueArchiveListRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    var list: std.ArrayList(pb.RogueArchiveInfo) = .empty;
    var slot: i32 = 1;
    while (slot <= 4) : (slot += 1) {
        const tkey = try std.fmt.allocPrint(alloc.arena, "rogue:arch_time:{d}", .{slot});
        const ikey = try std.fmt.allocPrint(alloc.arena, "rogue:arch_inst:{d}", .{slot});
        const stime = progress_comp.info.getKeyInt(tkey);
        const inst = progress_comp.info.getKeyInt(ikey);
        try list.append(alloc.arena, .{
            .SlotId = slot,
            .HasData = stime > 0,
            .SaveTime = stime,
            .InstId = @intCast(inst),
            .SeasonId = txn.message.SeasonId,
        });
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .ArchiveInfos = list,
    });
}

pub fn onRogueBlackFlowerRequest(
    txn: *Transaction(pb.RogueBlackFlowerRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "rogue:bf:{d}", .{txn.message.EntityConfigId});
    _ = try progress_comp.info.markClaim(alloc.gpa, key);
    if (txn.message.Ret) {
        const fallback = [_]RewardService.RewardItem{.{ .item_id = 3, .count = 20 }, .{ .item_id = 2, .count = 10000 }};
        _ = try RewardService.grantTableReward(reward_ctx, "RogueSeasonReward", txn.message.EntityConfigId, false, true, 1, &fallback, null);
    }
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

// =========================================================================
// Permanent RogueRes & RogueTowerTrial & WhiteCat & Weekly Rogue
// =========================================================================

pub fn onRogueResInstStartRequest(
    txn: *Transaction(pb.RogueResInstStartRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "rogue_res:inst", txn.message.InstId);
    try progress_comp.info.setKeyInt(alloc.gpa, "rogue_res:room", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRogueResLastInstInfoRequest(
    txn: *Transaction(pb.RogueResLastInstInfoRequest),
    progress_comp: *const PlayerProgressComponent,
) !void {
    txn.respond(.{
        .InstId = @intCast(progress_comp.info.getKeyInt("rogue_res:inst")),
        .ErrorCode = .Success,
    });
}

pub fn onRogueResGotoNextRoomRequest(
    txn: *Transaction(pb.RogueResGotoNextRoomRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "rogue_res:room", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRogueResMoveRequest(
    txn: *Transaction(pb.RogueResMoveRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    if (txn.message.MoveOp) |op| {
        if (op.Paths.items.len > 0) {
            try progress_comp.info.setKeyInt(alloc.gpa, "rogue_res:grid", op.Paths.items[op.Paths.items.len - 1]);
        }
    }
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "rogue_res:mood_cost", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onRogueResGridEventRequest(
    txn: *Transaction(pb.RogueResGridEventRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const k = try std.fmt.allocPrint(alloc.arena, "rogue_res:evt:{d}", .{txn.message.GridIndex});
    _ = try progress_comp.info.markClaim(alloc.gpa, k);
    const sk = try std.fmt.allocPrint(alloc.arena, "rogue_res:evt_step:{d}", .{txn.message.GridIndex});
    try progress_comp.info.setKeyInt(alloc.gpa, sk, txn.message.OptionIndex);
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "rogue_res:evt", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onRogueResOpDoneRequest(
    txn: *Transaction(pb.RogueResOpDoneRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const k = try std.fmt.allocPrint(alloc.arena, "rogue_res:op:{d}", .{txn.message.IncId});
    try progress_comp.info.setKeyInt(alloc.gpa, k, txn.message.ClientSelectId);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onRogueResChangeFormationRequest(
    txn: *Transaction(pb.RogueResChangeFormationRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    if (txn.message.RogueResFormation) |form| {
        const k = try std.fmt.allocPrint(alloc.arena, "rogue_res:form:{d}", .{txn.message.Index});
        try pb_kv.saveMsg(alloc, fs, progress_comp, k, form);
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .RogueResFormation = txn.message.RogueResFormation,
    });
}

pub fn onRogueResSwitchFormationRequest(
    txn: *Transaction(pb.RogueResSwitchFormationRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "rogue_res:form_idx", txn.message.Index);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRogueResTalentSkillLevelRequest(
    txn: *Transaction(pb.RogueResTalentSkillLevelRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "rogue_res:skill:{d}", .{txn.message.SkillId});
    const lv = try progress_comp.info.addKeyInt(alloc.gpa, key, 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .Level = @intCast(lv),
        .ErrorCode = .Success,
    });
}

pub fn onRogueResInstResultRequest(
    txn: *Transaction(pb.RogueResInstResultRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const inst_id: i32 = @intCast(progress_comp.info.getKeyInt("rogue_res:inst"));
    try progress_comp.info.setKeyInt(alloc.gpa, "rogue_res:inst", 0);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .ErrorCode = .Success,
        .InstResultView = .{
            .InstId = inst_id,
            .IsSucc = true,
            .Score = 1000,
        },
    });
}

pub fn onRogueResEndingRewardRequest(
    txn: *Transaction(pb.RogueResEndingRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "rogue_res:end:{d}:{d}", .{ txn.message.SeasonId, txn.message.Index });
    if (try progress_comp.info.markClaim(alloc.gpa, key)) {
        const fallback = [_]RewardService.RewardItem{.{ .item_id = 3, .count = 50 }};
        _ = try RewardService.grantTableReward(reward_ctx, "RogueResEnding", txn.message.Index, false, true, 1, &fallback, null);
        try progress_comp.save(fs, alloc.arena);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRogueResIllustrationAwardRequest(
    txn: *Transaction(pb.RogueResIllustrationAwardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    for (txn.message.ConfigIds.items) |cid| {
        const key = try std.fmt.allocPrint(alloc.arena, "rogue_res:ill:{d}", .{cid});
        if (try progress_comp.info.markClaim(alloc.gpa, key)) {
            const fallback = [_]RewardService.RewardItem{.{ .item_id = 3, .count = 20 }};
            _ = try RewardService.grantTableReward(reward_ctx, "RogueResIllustration", cid, false, true, 1, &fallback, null);
        }
    }
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success, .IsFull = false });
}

pub fn onRogueResTaskAwardRequest(
    txn: *Transaction(pb.RogueResTaskAwardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "rogue_res:task:{d}", .{txn.message.ConfigId});
    if (try progress_comp.info.markClaim(alloc.gpa, key)) {
        const fallback = [_]RewardService.RewardItem{.{ .item_id = 3, .count = 40 }};
        _ = try RewardService.grantTableReward(reward_ctx, "RogueResTask", txn.message.ConfigId, false, true, 1, &fallback, null);
        try progress_comp.save(fs, alloc.arena);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRogueTowerTrialStartRequest(
    txn: *Transaction(pb.RogueTowerTrialStartRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "rogue_trial:inst", txn.message.InstId);
    try progress_comp.info.setKeyInt(alloc.gpa, "rogue_trial:slot", txn.message.SlotId);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRogueTowerTrialSelectBossRequest(
    txn: *Transaction(pb.RogueTowerTrialSelectBossRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "rogue_trial:boss", txn.message.Id);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRogueTowerTrialGiveUpTempArchiveRequest(
    txn: *Transaction(pb.RogueTowerTrialGiveUpTempArchiveRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "rogue_trial:slot", 0);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRogueTowerTrialResultRequest(
    txn: *Transaction(pb.RogueTowerTrialResultRequest),
    progress_comp: *const PlayerProgressComponent,
) !void {
    txn.respond(.{
        .ErrorCode = .Success,
        .RogueTowerTrialResultInfo = .{
            .SlotId = @intCast(progress_comp.info.getKeyInt("rogue_trial:slot")),
            .ChallengingBossId = @intCast(progress_comp.info.getKeyInt("rogue_trial:boss")),
        },
    });
}

pub fn onRogueWeeklyStartRequest(
    txn: *Transaction(pb.RogueWeeklyStartRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "rogue_weekly:cycle", txn.message.CycleId);
    try progress_comp.info.setKeyInt(alloc.gpa, "rogue_weekly:layer", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRogueWeeklyLastInfoRequest(
    txn: *Transaction(pb.RogueWeeklyLastInfoRequest),
    progress_comp: *const PlayerProgressComponent,
) !void {
    txn.respond(.{
        .ErrorCode = .Success,
        .RogueWeeklyLastInfo = .{
            .InstId = @intCast(progress_comp.info.getKeyInt("rogue_weekly:cycle")),
            .CurLayer = @intCast(@max(1, progress_comp.info.getKeyInt("rogue_weekly:layer"))),
            .MaxLayer = 4,
            .WorldLevel = 1,
        },
    });
}

pub fn onRogueWeeklyArtifactInfoRequest(
    txn: *Transaction(pb.RogueWeeklyArtifactInfoRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
    io: std.Io,
) !void {
    _ = txn.message;
    var entries: std.ArrayList(pb.RogueWeeklyEntry) = .empty;
    for (assets.reward_index.getRows(io, "RogueWeeklyBF")) |row| {
        const cid: i32 = @intCast(row.key);
        try entries.append(alloc.arena, .{ .Index = @intCast(entries.items.len), .ConfigId = cid });
        if (entries.items.len >= 6) break;
    }
    txn.respond(.{ .ErrorCode = .Success, .RogueWeeklyEntrys = entries });
}

pub fn onRogueWeeklyArtifactSelectRequest(
    txn: *Transaction(pb.RogueWeeklyArtifactSelectRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "rogue_weekly:artifact", txn.message.Index);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRogueWeeklyOptionSelectRequest(
    txn: *Transaction(pb.RogueWeeklyOptionSelectRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "rogue_weekly:opt:{d}", .{txn.message.BindId});
    try progress_comp.info.setKeyInt(alloc.gpa, key, txn.message.Index);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRogueWeeklyGotoNextRoomRequest(
    txn: *Transaction(pb.RogueWeeklyGotoNextRoomRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    if (txn.message.Confirm) {
        _ = try progress_comp.info.addKeyInt(alloc.gpa, "rogue_weekly:layer", 1);
        try progress_comp.save(fs, alloc.arena);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRogueWeeklyBlackFlowerRequest(
    txn: *Transaction(pb.RogueWeeklyBlackFlowerRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const cur = @max(0, progress_comp.info.getKeyInt("rogue_weekly:bf_used"));
    if (cur >= 3) {
        txn.respond(.{
            .ErrorCode = .RequestParamError,
            .MaxFreeCount = 3,
            .UseFreeCount = 3,
        });
        return;
    }
    const used = try progress_comp.info.addKeyInt(alloc.gpa, "rogue_weekly:bf_used", 1);
    const fallback = [_]RewardService.RewardItem{.{ .item_id = 3, .count = 40 }, .{ .item_id = 2, .count = 20000 }};
    _ = try RewardService.grantTableReward(reward_ctx, "RogueWeeklyReward", txn.message.SilentAreaId, false, true, 1, &fallback, null);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .ErrorCode = .Success,
        .MaxFreeCount = 3,
        .UseFreeCount = @intCast(std.math.clamp(used, 0, 3)),
    });
}

pub fn onRogueWeeklyResultRequest(
    txn: *Transaction(pb.RogueWeeklyResultRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "rogue_weekly:score", 500);
    try progress_comp.info.setKeyInt(alloc.gpa, "rogue_weekly:layer", 0);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onMulRogueWeeklyRewardRequest(
    txn: *Transaction(pb.MulRogueWeeklyRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var out_ids: std.ArrayList(i32) = .empty;
    for (txn.message.ConfigId.items) |cid| {
        const key = try std.fmt.allocPrint(alloc.arena, "rogue_weekly:rw:{d}:{d}", .{ txn.message.ActivityId, cid });
        if (try progress_comp.info.markClaim(alloc.gpa, key)) {
            const fallback = [_]RewardService.RewardItem{.{ .item_id = 3, .count = 40 }};
            _ = try RewardService.grantTableReward(reward_ctx, "RogueWeeklyReward", cid, false, true, 1, &fallback, null);
        }
        try out_ids.append(alloc.arena, cid);
    }
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ConfigId = out_ids, .ErrorCode = .Success });
}

pub fn onRogueWhiteCatLimitedRewardRequest(
    txn: *Transaction(pb.RogueWhiteCatLimitedRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "rogue_cat:lim:{d}", .{txn.message.Index});
    if (try progress_comp.info.markClaim(alloc.gpa, key)) {
        const fallback = [_]RewardService.RewardItem{.{ .item_id = 3, .count = 40 }};
        _ = try RewardService.grantTableReward(reward_ctx, "RogueLimitTimeReward", txn.message.Index, false, true, 1, &fallback, null);
        try progress_comp.save(fs, alloc.arena);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRogueWhiteCatRecordRequest(
    txn: *Transaction(pb.RogueWhiteCatRecordRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "rogue_cat:rec:{d}", .{txn.message.ActivityId});
    try progress_comp.info.setKeyInt(alloc.gpa, key, txn.message.Value);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onMulRoguelikeWriteCatRewardRequest(
    txn: *Transaction(pb.MulRoguelikeWriteCatRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var indices: std.ArrayList(i32) = .empty;
    for (txn.message.Index.items) |idx| {
        const key = try std.fmt.allocPrint(alloc.arena, "rogue_cat:rw:{d}:{d}", .{ txn.message.ActivityId, idx });
        if (try progress_comp.info.markClaim(alloc.gpa, key)) {
            const fallback = [_]RewardService.RewardItem{.{ .item_id = 3, .count = 40 }};
            _ = try RewardService.grantTableReward(reward_ctx, "RogueWhiteCatReward", idx, false, true, 1, &fallback, null);
        }
        try indices.append(alloc.arena, idx);
    }
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .Index = indices, .ErrorCode = .Success });
}

pub fn onRoguelikeWriteCatBossRewardRequest(
    txn: *Transaction(pb.RoguelikeWriteCatBossRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "rogue_cat:boss:{d}:{d}", .{ txn.message.ActivityId, txn.message.Index });
    if (try progress_comp.info.markClaim(alloc.gpa, key)) {
        const fallback = [_]RewardService.RewardItem{.{ .item_id = 3, .count = 40 }};
        _ = try RewardService.grantTableReward(reward_ctx, "RogueWhiteCatBossReward", txn.message.Index, false, true, 1, &fallback, null);
        try progress_comp.save(fs, alloc.arena);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRoguelikeWriteCatLevelPlayRewardRequest(
    txn: *Transaction(pb.RoguelikeWriteCatLevelPlayRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "rogue_cat:lp:{d}:{d}", .{ txn.message.ActivityId, txn.message.Index });
    if (try progress_comp.info.markClaim(alloc.gpa, key)) {
        const fallback = [_]RewardService.RewardItem{.{ .item_id = 3, .count = 40 }};
        _ = try RewardService.grantTableReward(reward_ctx, "RogueWhiteCatReward", txn.message.Index, false, true, 1, &fallback, null);
        try progress_comp.save(fs, alloc.arena);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

// =========================================================================
// 3.x RoverRogue
// =========================================================================

pub fn onRoverRogueInsListRequest(
    txn: *Transaction(pb.RoverRogueInsListRequest),
    reward_ctx: RewardService.RewardContext,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    var list: std.ArrayList(pb.RoverRogueInsEntry) = .empty;
    const rows = reward_ctx.assets.reward_index.getRows(io, "RoverRogueIns");
    for (rows) |row| {
        try list.append(alloc.arena, .{
            .instId = @intCast(row.key),
            .Level = 1,
            .Unlocked = true,
            .BestGrade = .RoverRogueGradeS,
        });
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .List = list,
    });
}

pub fn onRoverRogueBlessGroupSelectRequest(
    txn: *Transaction(pb.RoverRogueBlessGroupSelectRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "rover_rogue:bless_group", txn.message.BlessGroupId);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRoverRogueChooseDataResultRequest(
    txn: *Transaction(pb.RoverRogueChooseDataResultRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "rover_rogue:gain:{d}", .{txn.message.BindId});
    try progress_comp.info.setKeyInt(alloc.gpa, key, txn.message.IncId);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .IsSelect = true, .ErrorCode = .Success });
}

pub fn onRoverRogueEventSelectRequest(
    txn: *Transaction(pb.RoverRogueEventSelectRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "rover_rogue:evt:{d}", .{txn.message.EventIncId});
    try progress_comp.info.setKeyInt(alloc.gpa, key, txn.message.ChoiceId);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRoverRogueGiveUpGainRequest(
    txn: *Transaction(pb.RoverRogueGiveUpGainRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "rover_rogue:giveup_cnt", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRoverRogueGotoNextRoomCommitRequest(
    txn: *Transaction(pb.RoverRogueGotoNextRoomCommitRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "rover_rogue:layer", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRoverRogueLootInfoRequest(
    txn: *Transaction(pb.RoverRogueLootInfoRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    _ = txn.message;
    var loots: std.ArrayList(pb.RoverRogueGainEntry) = .empty;
    for (progress_comp.info.dynamic.claims) |c| {
        if (std.mem.startsWith(u8, c, "rover_rogue:loot:")) {
            const lid = std.fmt.parseInt(i32, c["rover_rogue:loot:".len..], 10) catch continue;
            try loots.append(alloc.arena, .{ .ConfigId = lid, .IncId = lid });
        }
    }
    txn.respond(.{ .ErrorCode = .Success, .Loots = loots });
}

pub fn onRoverRogueLootChangeRequest(
    txn: *Transaction(pb.RoverRogueLootChangeRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "rover_rogue:loot:{d}", .{txn.message.IncId});
    try progress_comp.info.setKeyInt(alloc.gpa, key, txn.message.LootId);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRoverRogueRefreshGainRequest(
    txn: *Transaction(pb.RoverRogueRefreshGainRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const cnt = try progress_comp.info.addKeyInt(alloc.gpa, "rover_rogue:gain_refresh", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .ErrorCode = .Success,
        .ChooseData = .{
            .BindId = txn.message.BindId,
            .Layer = @intCast(@max(1, progress_comp.info.getKeyInt("rover_rogue:layer"))),
            .MaxTime = 3,
            .UseTime = @intCast(cnt),
        },
    });
}

pub fn onRoverRogueShopRefreshRequest(
    txn: *Transaction(pb.RoverRogueShopRefreshRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const cnt = try progress_comp.info.addKeyInt(alloc.gpa, "rover_rogue:shop_refresh", 1);
    try progress_comp.save(fs, alloc.arena);
    _ = txn.message;
    txn.respond(.{
        .ErrorCode = .Success,
        .Snapshot = .{ .RefreshCount = @intCast(cnt), .MaxRefreshCount = 5 },
    });
}

pub fn onRoverRogueShopBuyRequest(
    txn: *Transaction(pb.RoverRogueShopBuyRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "rover_rogue:buy:{d}", .{txn.message.GoodIncId});
    _ = try progress_comp.info.markClaim(alloc.gpa, key);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRoverRogueTalentUnlockRequest(
    txn: *Transaction(pb.RoverRogueTalentUnlockRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "rover_rogue:talent:{d}", .{txn.message.TalentId});
    _ = try progress_comp.info.markClaim(alloc.gpa, key);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRoverRogueTaskRewardTakeRequest(
    txn: *Transaction(pb.RoverRogueTaskRewardTakeRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var out_ids: std.ArrayList(i32) = .empty;
    for (txn.message.RewardIds.items) |rid| {
        const key = try std.fmt.allocPrint(alloc.arena, "rover_rogue:task:{d}:{d}", .{ txn.message.ActivityId, rid });
        if (try progress_comp.info.markClaim(alloc.gpa, key)) {
            const fallback = [_]RewardService.RewardItem{.{ .item_id = 3, .count = 40 }};
            _ = try RewardService.grantTableReward(reward_ctx, "RoverRogueReward", rid, false, true, 1, &fallback, null);
        }
        try out_ids.append(alloc.arena, rid);
    }
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .ErrorCode = .Success,
        .SuccessRewardIds = out_ids,
    });
}

pub fn onRoverRogueResultRequest(
    txn: *Transaction(pb.RoverRogueResultRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const layer: i32 = @intCast(@max(1, progress_comp.info.getKeyInt("rover_rogue:layer")));
    try progress_comp.info.setKeyInt(alloc.gpa, "rover_rogue:layer", 0);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .ErrorCode = .Success,
        .ResultInfo = .{
            .InstId = txn.message.InstId,
            .Grade = .RoverRogueGradeS,
            .CurLayer = layer,
            .MaxLayer = 5,
            .RoomPassedCount = layer,
        },
    });
}

pub fn onRoverRogueQuitRequest(
    txn: *Transaction(pb.RoverRogueQuitRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "rover_rogue:layer", 0);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}
