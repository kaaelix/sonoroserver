const std = @import("std");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const common = @import("common");
const FileSystem = common.FileSystem;
const Transaction = @import("../handlers.zig").Transaction;
const Assets = @import("../../data/Assets.zig");
const Scene = @import("../../logic/Scene.zig");
const PlayerBasicComponent = @import("../../logic/component/player/PlayerBasicComponent.zig");
const PlayerInventoryComponent = @import("../../logic/component/player/PlayerInventoryComponent.zig");
const PlayerProgressComponent = @import("../../logic/component/player/PlayerProgressComponent.zig");
const ConsumeHelper = @import("../../logic/helpers/consume.zig");
const QuestEngine = @import("../../logic/helpers/quest_engine.zig");
const RewardService = @import("../../logic/helpers/reward.zig");
const EnergyHelper = @import("../../logic/helpers/energy.zig");
const pb_kv = @import("pb_kv.zig");

fn claimIdList(
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
    prefix: []const u8,
    table: []const u8,
    ids: []const i32,
) !std.ArrayList(pb.MapEntry(i32, i32)) {
    const fallback = [_]RewardService.RewardItem{
        .{ .item_id = 3, .count = 40 },
        .{ .item_id = 2, .count = 15000 },
    };
    var granted_map: std.ArrayList(pb.MapEntry(i32, i32)) = .empty;
    for (ids) |id| {
        const key = try std.fmt.allocPrint(alloc.arena, "{s}:{d}", .{ prefix, id });
        if (try progress_comp.info.markClaim(alloc.gpa, key)) {
            var tmp: std.ArrayList(RewardService.RewardItem) = .empty;
            _ = try RewardService.grantTableReward(reward_ctx, table, id, false, true, 19500, &fallback, &tmp);
            for (tmp.items) |it| {
                try granted_map.append(alloc.arena, .{ .key = it.item_id, .value = it.count });
            }
        }
    }
    try progress_comp.save(fs, alloc.arena);
    return granted_map;
}

fn claimOne(
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
    prefix: []const u8,
    table: []const u8,
    id: i32,
) !std.ArrayList(pb.MapEntry(i32, i32)) {
    const arr = [_]i32{id};
    return try claimIdList(alloc, fs, progress_comp, reward_ctx, prefix, table, &arr);
}

// =========================================================================
// General Activity, PayShop, Sign, Daily, Scratch, Turntable, WuWu
// =========================================================================

pub fn onActivityFirstReadRequest(
    txn: *Transaction(pb.ActivityFirstReadRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "act_read:{d}", .{txn.message.ActivityId});
    _ = try progress_comp.info.markClaim(alloc.gpa, key);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onActivityPreOpenRequest(
    txn: *Transaction(pb.ActivityPreOpenRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "act:preopen:{d}", .{txn.message.ActivityId});
    try pb_kv.persistClaim(alloc, fs, progress_comp, key);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onActivityPayShopBuyRequest(
    txn: *Transaction(pb.ActivityPayShopBuyRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var out: std.ArrayList(pb.ActivityBuyItem) = .empty;
    for (txn.message.BuyItems.items) |bi| {
        const key = try std.fmt.allocPrint(alloc.arena, "act_shop:{d}", .{bi.Id});
        _ = try progress_comp.info.addKeyInt(alloc.gpa, key, bi.Count);
        try out.append(alloc.arena, bi);
    }
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success, .BuyItems = out });
}

pub fn onActivityResultRewardRequest(
    txn: *Transaction(pb.ActivityResultRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimOne(alloc, fs, progress_comp, reward_ctx, "act_res_rw", "ActivityReward", txn.message.ResultId);
    var ids: std.ArrayList(i32) = .empty;
    try ids.append(alloc.arena, txn.message.ResultId);
    txn.respond(.{ .ErrorCode = .Success, .TakeRewardResultIds = ids });
}

pub fn onSignActivityRequest(
    txn: *Transaction(pb.SignActivityRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "sign_act:{d}:{d}", .{ txn.message.ActivityId, txn.message.Index });
    var item_map: std.ArrayList(pb.MapEntry(i32, i32)) = .empty;
    if (try progress_comp.info.markClaim(alloc.gpa, key)) {
        const fallback = [_]RewardService.RewardItem{
            .{ .item_id = 3, .count = 40 },
            .{ .item_id = 2, .count = 15000 },
        };
        var tmp: std.ArrayList(RewardService.RewardItem) = .empty;
        _ = try RewardService.grantTableReward(reward_ctx, "ActivitySign", txn.message.Index, false, true, 19500, &fallback, &tmp);
        for (tmp.items) |it| {
            try item_map.append(alloc.arena, .{ .key = it.item_id, .value = it.count });
        }
        try progress_comp.save(fs, alloc.arena);
    }
    txn.respond(.{ .ErrorCode = .Success, .ItemMap = item_map });
}

pub fn onSignActivityFreeDropRequest(
    txn: *Transaction(pb.SignActivityFreeDropRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimOne(alloc, fs, progress_comp, reward_ctx, "sign_free", "ActivitySignGrandReward", txn.message.ActivityId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onDailyAdventurePtRewardRequest(
    txn: *Transaction(pb.DailyAdventurePtRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimOne(alloc, fs, progress_comp, reward_ctx, "dadv_pt", "DailyAdventurePoint", txn.message.Id);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onDailyAdventureTaskRewardRequest(
    txn: *Transaction(pb.DailyAdventureTaskRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimOne(alloc, fs, progress_comp, reward_ctx, "dadv_task", "DailyAdventureTask", txn.message.Id);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRunTurntableActivityRequest(
    txn: *Transaction(pb.RunTurntableActivityRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const spin_key = try std.fmt.allocPrint(alloc.arena, "turntable:spin:{d}", .{txn.message.ActivityId});
    const roll = try progress_comp.info.addKeyInt(alloc.gpa, spin_key, 1);
    const round = @divFloor(roll - 1, 8) + 1;
    const rw_id: i32 = @intCast(@mod(roll - 1, 8) + 1);
    const rw_prefix = try std.fmt.allocPrint(alloc.arena, "turntable_rw:{d}:{d}", .{ txn.message.ActivityId, round });
    const item_map = try claimOne(alloc, fs, progress_comp, reward_ctx, rw_prefix, "TurntableAwards", rw_id);
    txn.respond(.{
        .ErrorCode = .Success,
        .IsFinishCurRound = (@mod(roll, 8) == 0),
        .RewardId = rw_id,
        .ItemMap = item_map,
    });
}

pub fn onTurntableCompleteRequest(
    txn: *Transaction(pb.TurntableCompleteRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "turntable:done:{d}", .{txn.message.EntityId});
    try pb_kv.persistClaim(alloc, fs, progress_comp, key);
    txn.respond(.{ .ErrCode = .Success, .EntityId = txn.message.EntityId });
}

pub fn onScratchCardActivityInfoRequest(
    txn: *Transaction(pb.ScratchCardActivityInfoRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "scratch:info:{d}", .{txn.message.ActivityId});
    const info: pb.ScratchCardActivityInfo = pb_kv.loadMsg(alloc.arena, progress_comp, key, pb.ScratchCardActivityInfo) orelse .{};
    txn.respond(.{ .Error = .Success, .Info = info });
}

pub fn onScratchCardRewardRequest(
    txn: *Transaction(pb.ScratchCardRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "scratch:{d}:{d}", .{ txn.message.RoundId, txn.message.Index });
    const item_map = try claimOne(alloc, fs, progress_comp, reward_ctx, key, "ScratchCard", txn.message.Index);
    txn.respond(.{
        .ItemMap = item_map,
        .OpenType = 1,
        .Error = .Success,
    });
}

pub fn onWuWuKujiGachaRequest(
    txn: *Transaction(pb.WuWuKujiGachaRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const spin = try progress_comp.info.addKeyInt(alloc.gpa, "wuwu:spin", 1);
    const item_map = try claimOne(alloc, fs, progress_comp, reward_ctx, "wuwu_gacha", "WuWuKuji", @intCast(spin));
    var awards: std.ArrayList(pb.SingleGachaAwardPb) = .empty;
    try awards.append(alloc.arena, .{ .GroupId = 1, .Times = 1, .ItemMap = item_map });
    txn.respond(.{
        .TrickType = 1,
        .ErrorCode = .Success,
        .Awards = awards,
    });
}

pub fn onWuWuPackageRewardRequest(
    txn: *Transaction(pb.WuWuPackageRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimOne(alloc, fs, progress_comp, reward_ctx, "wuwu_pkg", "WuWuTaskPackage", txn.message.WrapId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onWuWuTaskRewardRequest(
    txn: *Transaction(pb.WuWuTaskRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimIdList(alloc, fs, progress_comp, reward_ctx, "wuwu_task", "WuWuWeekTask", txn.message.Ids.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onConsumptiveActivityInfoRequest(
    txn: *Transaction(pb.ConsumptiveActivityInfoRequest),
    progress_comp: *const PlayerProgressComponent,
) !void {
    txn.respond(.{
        .ConsumptiveActivityInfo = .{
            .TotalScore = @intCast(progress_comp.info.getKeyInt("cons:score")),
        },
        .ErrorCode = .Success,
    });
}

pub fn onConsumptiveActivityScoreRequest(
    txn: *Transaction(pb.ConsumptiveActivityScoreRequest),
    progress_comp: *const PlayerProgressComponent,
) !void {
    txn.respond(.{
        .ErrorCode = .Success,
        .TotalScore = @intCast(progress_comp.info.getKeyInt("cons:score")),
    });
}

pub fn onConsumptiveRewardRequest(
    txn: *Transaction(pb.ConsumptiveRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimIdList(alloc, fs, progress_comp, reward_ctx, "cons_task", "ConsumptiveTask", txn.message.TaskIds.items);
    var task_infos: std.ArrayList(pb.ConsumptiveTaskInfo) = .empty;
    for (txn.message.TaskIds.items) |tid| {
        const cfg = assets.tables.consumptive_task.getDataById(tid);
        const target: i32 = if (cfg) |c| @max(1, c.MaxFinishCount) else 1;
        const ttype: pb.ConsumptiveTaskType = if (cfg) |c| switch (c.TaskTab) {
            2 => .Cycle,
            else => .Single,
        } else .Single;
        try task_infos.append(alloc.arena, .{
            .Id = tid,
            .Type = ttype,
            .Process = .{ .Current = target, .Target = target },
            .Reward = .{ .Rewarded = 1, .WaitReward = 0, .MaxReward = target },
        });
    }
    txn.respond(.{ .ErrorCode = .Success, .TaskInfos = task_infos });
}

pub fn onPreheatSignSurveyInfoRequest(
    txn: *Transaction(pb.PreheatSignSurveyInfoRequest),
    progress_comp: *const PlayerProgressComponent,
    alloc: mem.Alloc,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "preheat:{d}", .{txn.message.PreheatNodeId});
    txn.respond(.{
        .Error = .Success,
        .PreheatSignSurveyInfo = .{
            .SupportNum = 100,
            .OpposeNum = 10,
            .AnswerStatus = progress_comp.info.hasClaim(key),
        },
    });
}

pub fn onPreheatSignRewardRequest(
    txn: *Transaction(pb.PreheatSignRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "preheat:{d}", .{txn.message.PreheatNodeId});
    _ = try claimOne(alloc, fs, progress_comp, reward_ctx, key, "PreheatSign", txn.message.PreheatNodeId);
    txn.respond(.{
        .Error = .Success,
        .PreheatSignSurveyInfo = .{ .SupportNum = 101, .OpposeNum = 10, .AnswerStatus = txn.message.AnswerStatus },
    });
}

pub fn onH5CircumFluenceActivityReadRequest(
    txn: *Transaction(pb.H5CircumFluenceActivityReadRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    try pb_kv.persistClaim(alloc, fs, progress_comp, "h5:cf_read");
    txn.respond(.{});
}

pub fn onH5ViewActivityReadRequest(
    txn: *Transaction(pb.H5ViewActivityReadRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "h5:view_read:{d}", .{txn.message.ActivityId});
    try pb_kv.persistClaim(alloc, fs, progress_comp, key);
    txn.respond(.{});
}

// =========================================================================
// Theme, Linkage, Coast, Festival, Building & Collection Events
// =========================================================================

pub fn onArtemisNodeUpdateRequest(
    txn: *Transaction(pb.ArtemisNodeUpdateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "artemis:{d}:{d}", .{ txn.message.ActivityId, txn.message.Index });
    _ = try progress_comp.info.markClaim(alloc.gpa, key);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .Error = .Success });
}

pub fn onAvignonRewardRequest(
    txn: *Transaction(pb.AvignonRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimOne(alloc, fs, progress_comp, reward_ctx, "avignon_rw", "Avignon", txn.message.TaskId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onBlackCoastThemeActiveRewardRequest(
    txn: *Transaction(pb.BlackCoastThemeActiveRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimIdList(alloc, fs, progress_comp, reward_ctx, "bc_act_rw", "BlackCoastThemeRewardRe", txn.message.ActiveRewardIds.items);
    var out: std.ArrayList(i32) = .empty;
    try out.appendSlice(alloc.arena, txn.message.ActiveRewardIds.items);
    txn.respond(.{ .Error = .Success, .ActiveRewardedIds = out });
}

pub fn onBlackCoastThemeTaskRewardRequest(
    txn: *Transaction(pb.BlackCoastThemeTaskRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimOne(alloc, fs, progress_comp, reward_ctx, "bc_task_rw", "BlackCoastThemeTaskRe", txn.message.TaskId);
    txn.respond(.{ .Error = .Success });
}

pub fn onBlackSwordChallengeInfoRequest(
    txn: *Transaction(pb.BlackSwordChallengeInfoRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    var list: std.ArrayList(pb.BlackSwordChallengeInfo) = .empty;
    for (progress_comp.info.dynamic.ints) |entry| {
        if (std.mem.startsWith(u8, entry.key, "bsword:lp:")) {
            if (std.fmt.parseInt(i32, entry.key["bsword:lp:".len..], 10)) |lp_id| {
                try list.append(alloc.arena, .{ .LevelPlayId = lp_id, .IsUnLocked = true, .CompleteNumber = @intCast(entry.val) });
            } else |_| {}
        }
    }
    txn.respond(.{ .LevelPlayUiInfoList = list, .ErrorCode = .Success, .ChallengeId = txn.message.ChallengeId });
}

pub fn onBlackSwordChallengeMarkItemPanelInfoRequest(
    txn: *Transaction(pb.BlackSwordChallengeMarkItemPanelInfoRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    var list: std.ArrayList(pb.BlackSwordChallengeInfo) = .empty;
    for (progress_comp.info.dynamic.ints) |entry| {
        if (std.mem.startsWith(u8, entry.key, "bsword:lp:")) {
            if (std.fmt.parseInt(i32, entry.key["bsword:lp:".len..], 10)) |lp_id| {
                try list.append(alloc.arena, .{ .LevelPlayId = lp_id, .IsUnLocked = true, .CompleteNumber = @intCast(entry.val) });
            } else |_| {}
        }
    }
    txn.respond(.{ .LevelPlayUiInfoList = list, .ErrorCode = .Success, .ChallengeId = txn.message.ChallengeId });
}

pub fn onBrochureRewardRequest(
    txn: *Transaction(pb.BrochureRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimIdList(alloc, fs, progress_comp, reward_ctx, "brochure", "Brochure", txn.message.BookItems.items);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onBuildingUnLockRequest(
    txn: *Transaction(pb.BuildingUnLockRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "bld:lv:{d}", .{txn.message.BuildingId});
    if (progress_comp.info.getKeyInt(key) == 0) {
        try progress_comp.info.setKeyInt(alloc.gpa, key, 1);
    }
    const like = try progress_comp.info.addKeyInt(alloc.gpa, "bld:like", 10);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success, .PeopleLike = @intCast(like) });
}

pub fn onBuildingLevelUpRequest(
    txn: *Transaction(pb.BuildingLevelUpRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "bld:lv:{d}", .{txn.message.BuildingId});
    const lv = try progress_comp.info.addKeyInt(alloc.gpa, key, 1);
    const like = try progress_comp.info.addKeyInt(alloc.gpa, "bld:like", 15);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .BuildingInfo = .{ .BuildingId = txn.message.BuildingId, .Level = @intCast(lv), .IsUnlock = true },
        .ErrorCode = .Success,
        .PeopleLike = @intCast(like),
    });
}

pub fn onCertificateLevelRewardRequest(
    txn: *Transaction(pb.CertificateLevelRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimOne(alloc, fs, progress_comp, reward_ctx, "cert_lv_rw", "CertificateReward", 1);
    txn.respond(.{ .Code = .Success });
}

pub fn onCorniceChallengeTransRequest(
    txn: *Transaction(pb.CorniceChallengeTransRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try pb_kv.persistInt(alloc, fs, progress_comp, "cornice:cur_lp", txn.message.LevelPlayId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onMulCorniceMeetingRewardRequest(
    txn: *Transaction(pb.MulCorniceMeetingRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    for (txn.message.CorniceMeetingRewardScoreLists.items) |entry| {
        _ = try claimIdList(alloc, fs, progress_comp, reward_ctx, "cornice_rw", "CorniceMeeting", entry.value.ScoreIndexes.items);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onDarkCoastInfoRequest(
    txn: *Transaction(pb.DarkCoastInfoRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
    progress_comp: *const PlayerProgressComponent,
) !void {
    const pool_id = if (txn.message.DragonPoolId > 0) txn.message.DragonPoolId else 3;
    var drop_ids: std.ArrayList(i32) = .empty;
    var guard_ids: std.ArrayList(i32) = .empty;
    var box_done: std.ArrayList(i32) = .empty;

    var level_gains: i32 = @intCast(@max(0, progress_comp.info.getKeyInt(
        try std.fmt.allocPrint(alloc.arena, "mingsu:lvl:{d}", .{pool_id}),
    )));
    if (assets.tables.dragon_pool.getDataById(pool_id)) |pool| {
        for (pool.DropIds) |did| {
            try drop_ids.append(alloc.arena, did);
        }
        var lvl: i32 = 1;
        while (lvl <= @as(i32, @intCast(pool.DropIds.len))) : (lvl += 1) {
            const clm_k = try std.fmt.allocPrint(alloc.arena, "mingsu:clm:{d}:{d}", .{ pool_id, lvl });
            if (progress_comp.info.hasClaim(clm_k) and lvl > level_gains) {
                level_gains = lvl;
            }
        }
    }

    for (assets.tables.dark_coast_delivery.items) |dc| {
        const g_key = try std.fmt.allocPrint(alloc.arena, "darkcoast:guard:{d}", .{dc.Id});
        const lp_key = try std.fmt.allocPrint(alloc.arena, "lp_state:{d}", .{dc.LevelPlayId});
        if (progress_comp.info.hasClaim(g_key) or progress_comp.info.getKeyInt(lp_key) >= 3) {
            try guard_ids.append(alloc.arena, dc.Id);
        }
        const b_key = try std.fmt.allocPrint(alloc.arena, "darkcoast:box:{d}", .{dc.Id});
        if (progress_comp.info.hasClaim(b_key)) {
            try box_done.append(alloc.arena, dc.Id);
        }
    }

    txn.respond(.{
        .DragonPoolConf = .{
            .DragonPoolId = pool_id,
            .DropIds = drop_ids,
        },
        .GuardIds = guard_ids,
        .TreasureBoxDone = box_done,
        .LevelGains = level_gains,
        .ErrorCode = .Success,
    });
}

pub fn onDragonPoolConfRequest(txn: *Transaction(pb.DragonPoolConfRequest)) !void {
    txn.respond(.{
        .DragonPoolConf = .{ .DragonPoolId = txn.message.DragonPoolId },
        .ErrorCode = .Success,
    });
}

pub fn onEdgeRunnerFunctionPreUnlockRequest(
    txn: *Transaction(pb.EdgeRunnerFunctionPreUnlockRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "edgerunner:fn:{d}", .{txn.message.FuncId});
    _ = try progress_comp.info.markClaim(alloc.gpa, key);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onEdgeRunnerScoreRewardRequest(
    txn: *Transaction(pb.EdgeRunnerScoreRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimOne(alloc, fs, progress_comp, reward_ctx, "er_score", "EdgeRunnerReward", txn.message.ScoreId);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onEdgeRunnerTaskRewardRequest(
    txn: *Transaction(pb.EdgeRunnerTaskRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimIdList(alloc, fs, progress_comp, reward_ctx, "er_task", "EdgeRunnerReward", txn.message.TaskIds.items);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onFeiXuePreheatInfoRewardRequest(
    txn: *Transaction(pb.FeiXuePreheatInfoRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimIdList(alloc, fs, progress_comp, reward_ctx, "feixue", "FeiXuePreheat", txn.message.FeiXuePreheatInfoIds.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onFlagChallengeRewardRequest(
    txn: *Transaction(pb.FlagChallengeRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimIdList(alloc, fs, progress_comp, reward_ctx, "flag_rw", "FlagChallenge", txn.message.FlagChallengeTaskIds.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onFunPlayChallengeAwardRequest(
    txn: *Transaction(pb.FunPlayChallengeAwardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimOne(alloc, fs, progress_comp, reward_ctx, "funplay_rw", "FunPlay", txn.message.ChallengeId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onFuncShowRequest(
    txn: *Transaction(pb.FuncShowRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var dirty = false;
    for (txn.message.FuncId.items) |fid| {
        const key = try std.fmt.allocPrint(alloc.arena, "func:shown:{d}", .{fid});
        if (try progress_comp.info.markClaim(alloc.gpa, key)) dirty = true;
    }
    if (dirty) try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onGatherActivityRewardRequest(
    txn: *Transaction(pb.GatherActivityRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimOne(alloc, fs, progress_comp, reward_ctx, "gather_rw", "GatherActivity", txn.message.GatherTaskId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onGiantDeliveryItemRequest(
    txn: *Transaction(pb.GiantDeliveryItemRequest),
    qctx: QuestEngine.QuestContext,
    reward_ctx: RewardService.RewardContext,
) !void {
    const eco_type = txn.message.GiantEcoType orelse {
        txn.respond(.{ .ErrorCode = .ErrGiantItemInvaild });
        return;
    };
    const result = try deliverGiantItems(qctx, reward_ctx, @intFromEnum(eco_type), txn.message.ItemInfos.items, false);
    if (result.err) |err| {
        txn.respond(.{ .ErrorCode = err });
        return;
    }
    txn.respond(.{
        .HasReward = result.has_reward,
        .ErrorCode = .Success,
        .TopQuality = result.top_quality,
        .GiantEcoInfo = .{
            .GiantEcoType = eco_type,
            .Level = result.level,
            .StableValue = result.stable_value,
            .Status = result.status,
        },
    });
    try QuestEngine.onConditionStateChanged(qctx);
}

pub fn onGiantMaxLevelDeliveryItemRequest(
    txn: *Transaction(pb.GiantMaxLevelDeliveryItemRequest),
    qctx: QuestEngine.QuestContext,
    reward_ctx: RewardService.RewardContext,
) !void {
    const eco_type = txn.message.GiantEcoType orelse {
        txn.respond(.{ .ErrorCode = .ErrGiantItemInvaild });
        return;
    };
    const result = try deliverGiantItems(qctx, reward_ctx, @intFromEnum(eco_type), txn.message.ItemInfos.items, true);
    if (result.err) |err| {
        txn.respond(.{ .ErrorCode = err });
        return;
    }
    txn.respond(.{
        .GiantEcoInfo = .{ .GiantEcoType = eco_type, .Level = result.level, .StableValue = result.stable_value, .Status = result.status },
        .TopQuality = result.top_quality,
        .ErrorCode = .Success,
        .HasReward = result.has_reward,
    });
    try QuestEngine.onConditionStateChanged(qctx);
}

const GiantDeliveryResult = struct {
    err: ?pb.ErrorCode = null,
    level: i32 = 1,
    stable_value: i32 = 0,
    status: pb.GiantEcoNetStatus = .Noraml,
    top_quality: i32 = 0,
    has_reward: bool = false,
};

fn giantLevel(reward: anytype, stable_value: i32) i32 {
    for (reward.Goal, 0..) |goal, i| {
        if (stable_value < goal) return @intCast(i + 1);
    }
    return @intCast(reward.Goal.len + 1);
}

fn giantStatus(reward: anytype, level: i32, stable_value: i32) pb.GiantEcoNetStatus {
    if (stable_value >= reward.MaxStableValue) return .MaxStableValue;
    if (level > reward.Goal.len) return .MaxLevel;
    return .Noraml;
}

fn deliverGiantItems(
    qctx: QuestEngine.QuestContext,
    reward_ctx: RewardService.RewardContext,
    eco_type: i32,
    items: []const pb.MapEntry(i32, i32),
    max_level_delivery: bool,
) !GiantDeliveryResult {
    const table = qctx.assets.tables.giant_delivery_reward.getDataById(eco_type) orelse return .{ .err = .ErrGiantItemInvaild };
    const progress = qctx.progress_comp orelse return .{ .err = .ErrGiantItemInvaild };
    const key = try std.fmt.allocPrint(qctx.alloc.arena, "giant:stable:{d}", .{eco_type});
    const old_stable = @as(i32, @intCast(std.math.clamp(progress.info.getKeyInt(key), 0, table.MaxStableValue)));
    const old_level = giantLevel(table, old_stable);
    if (max_level_delivery and old_level <= table.Goal.len) return .{ .err = .ErrGiantNotMaxLevel };
    if (!max_level_delivery and old_level > table.Goal.len) return .{ .err = .ErrGiantMaxLevel };

    var costs: std.ArrayList(ConsumeHelper.CostItem) = .empty;
    var quality_counts: [8]i32 = @splat(0);
    var total_add: i64 = 0;
    var top_quality: i32 = 0;
    for (items) |entry| {
        const item_id = entry.key;
        const count = entry.value;
        if (item_id <= 0 or count <= 0) return .{ .err = .ErrGiantItemInvaild };
        const configured_for_eco = for (table.ItemIds) |allowed_id| {
            if (allowed_id == item_id) break true;
        } else false;
        if (!configured_for_eco) return .{ .err = .ErrGiantItemNotMatch };
        const item = qctx.assets.tables.giant_item.getDataById(item_id) orelse return .{ .err = .ErrGiantItemInvaild };
        if (item.GiantEcoType != eco_type) return .{ .err = .ErrGiantItemNotMatch };
        const owned = qctx.inventory_comp.info.normalItemCount(item_id);
        if (owned < count) return .{ .err = .ErrGiantItemNotEnough };
        if (!max_level_delivery) {
            const quality_idx = std.math.cast(usize, item.Quality) orelse return .{ .err = .ErrGiantItemInvaild };
            if (quality_idx >= quality_counts.len) return .{ .err = .ErrGiantItemInvaild };
            quality_counts[quality_idx] += count;
            if (quality_counts[quality_idx] > table.MaxDeliveryCount) return .{ .err = .ErrGiantItemInvaild };
        }
        top_quality = @max(top_quality, item.Quality);
        total_add += @as(i64, item.AddStableValue) * count;
        try costs.append(qctx.alloc.arena, .{ .id = item_id, .count = count });
    }
    if (costs.items.len == 0) return .{ .err = .ErrGiantItemInvaild };

    const new_stable: i32 = @intCast(@min(@as(i64, table.MaxStableValue), @as(i64, old_stable) + total_add));
    if (new_stable <= old_stable) return .{ .err = .ErrGiantMaxLevel };
    if (!(try ConsumeHelper.consumeCosts(
        qctx.conn,
        qctx.alloc,
        qctx.fs,
        qctx.basic_comp,
        qctx.inventory_comp,
        costs.items,
        qctx.io,
    ))) return .{ .err = .ErrGiantItemNotEnough };

    try progress.info.setKeyInt(qctx.alloc.gpa, key, new_stable);
    try progress.save(qctx.fs, qctx.alloc.arena);

    var has_reward = false;
    if (max_level_delivery) {
        if (table.MaxLevelDropId > 0) {
            const grant = try RewardService.grantDropPackagePopup(reward_ctx, table.MaxLevelDropId, 19500);
            has_reward = grant.granted_count > 0;
        }
    } else {
        for (table.Goal, 0..) |goal, i| {
            if (old_stable < goal and new_stable >= goal and i < table.DropIds.len and table.DropIds[i] > 0) {
                const grant = try RewardService.grantDropPackagePopup(reward_ctx, table.DropIds[i], 19500);
                has_reward = has_reward or grant.granted_count > 0;
            }
        }
    }

    const level = giantLevel(table, new_stable);
    return .{
        .level = level,
        .stable_value = new_stable,
        .status = giantStatus(table, level, new_stable),
        .top_quality = top_quality,
        .has_reward = has_reward,
    };
}

pub fn onGiantStableValueActivityRequest(
    txn: *Transaction(pb.GiantStableValueActivityRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimIdList(alloc, fs, progress_comp, reward_ctx, "giant_sv", "GiantActivity", txn.message.RewardIds.items);
    var out: std.ArrayList(i32) = .empty;
    try out.appendSlice(alloc.arena, txn.message.RewardIds.items);
    txn.respond(.{ .ErrorCode = .Success, .GotRewardIds = out });
}

pub fn onGivebackRewardRequest(
    txn: *Transaction(pb.GivebackRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    if (txn.message.RewardId != 0) {
        _ = try claimOne(alloc, fs, progress_comp, reward_ctx, "giveback_rw", "Giveback", txn.message.RewardId);
    }
    _ = try claimIdList(alloc, fs, progress_comp, reward_ctx, "giveback_rw", "Giveback", txn.message.RwdId.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onLinkageCheckInRewardRequest(
    txn: *Transaction(pb.LinkageCheckInRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimIdList(alloc, fs, progress_comp, reward_ctx, "link_ci", "LinkageCheckIn", txn.message.TaskIds.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onLinkageRewardRequest(
    txn: *Transaction(pb.LinkageRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimOne(alloc, fs, progress_comp, reward_ctx, "link_rw", "ActivityLinkage", txn.message.ActivityPageId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onLongShanScoreRewardRequest(
    txn: *Transaction(pb.LongShanScoreRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimIdList(alloc, fs, progress_comp, reward_ctx, "longshan_sc", "LongShanScoreReward", txn.message.Ids.items);
    var out: std.ArrayList(i32) = .empty;
    try out.appendSlice(alloc.arena, txn.message.Ids.items);
    txn.respond(.{ .Error = .Success, .ScoreRewardedIds = out });
}

pub fn onLongShanTaskRewardRequest(
    txn: *Transaction(pb.LongShanTaskRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimIdList(alloc, fs, progress_comp, reward_ctx, "longshan_task", "LongShanTask", txn.message.TaskIds.items);
    txn.respond(.{ .Error = .Success });
}

pub fn onMoonPhaseRandomRequest(txn: *Transaction(pb.MoonPhaseRandomRequest), io: std.Io) !void {
    const now_s: u64 = @intCast(@max(1, EnergyHelper.nowSeconds(io)));
    txn.respond(.{
        .Result = .{
            .MoonPhaseId = @intCast((now_s % 6) + 1),
            .Label1 = 1,
            .Label2 = 2,
        },
        .ErrorCode = .Success,
    });
}

pub fn onMoonPhaseRewardRequest(
    txn: *Transaction(pb.MoonPhaseRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimOne(alloc, fs, progress_comp, reward_ctx, "moon_phase_rw", "MoonPhase", 1);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onTraceMoonPhaseRewardRequest(
    txn: *Transaction(pb.TraceMoonPhaseRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimOne(alloc, fs, progress_comp, reward_ctx, "trace_mp_rw", "TrackMoonPhase", txn.message.ActivityId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onOccupyRewardRequest(
    txn: *Transaction(pb.OccupyRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimIdList(alloc, fs, progress_comp, reward_ctx, "occupy_rw", "OccupyReward", txn.message.Ids.items);
    var out: std.ArrayList(i32) = .empty;
    try out.appendSlice(alloc.arena, txn.message.Ids.items);
    txn.respond(.{ .Ids = out, .ErrorCode = .Success });
}

pub fn onQingXiaoTaskRewardRequest(
    txn: *Transaction(pb.QingXiaoTaskRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimIdList(alloc, fs, progress_comp, reward_ctx, "qingxiao_rw", "QingXiao", txn.message.TaskIds.items);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onScheduleRewardRequest(
    txn: *Transaction(pb.ScheduleRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimOne(alloc, fs, progress_comp, reward_ctx, "sched_rw", "ActivityReward", txn.message.RewardId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onThemePersonalScoreRequest(
    txn: *Transaction(pb.ThemePersonalScoreRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimIdList(alloc, fs, progress_comp, reward_ctx, "theme_ps", "ThemeCelebration", txn.message.Ids.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onThemeWorldScoreRequest(
    txn: *Transaction(pb.ThemeWorldScoreRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimIdList(alloc, fs, progress_comp, reward_ctx, "theme_ws", "ThemeCelebration", txn.message.Ids.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onTimePointActivityRewardRequest(
    txn: *Transaction(pb.TimePointActivityRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimOne(alloc, fs, progress_comp, reward_ctx, "tpa_rw", "TimePointReward", txn.message.Id);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onTimePointActivityBatchRewardRequest(
    txn: *Transaction(pb.TimePointActivityBatchRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimIdList(alloc, fs, progress_comp, reward_ctx, "tpa_brw", "TimePointReward", txn.message.Ids.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onTrialRoleRewardRequest(
    txn: *Transaction(pb.TrialRoleRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimOne(alloc, fs, progress_comp, reward_ctx, "trial_role_rw", "RoleTrial", txn.message.RoleId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onTrialRoleSkinRewardRequest(
    txn: *Transaction(pb.TrialRoleSkinRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimOne(alloc, fs, progress_comp, reward_ctx, "trial_skin_rw", "RoleSkinTrial", txn.message.RoleSkinTrialInfoId);
    txn.respond(.{ .ErrorCode = .Success });
}

// =========================================================================
// Mini-Games: Bath, Board, DropCatch, Encircle, FarmGold, Harvest, Match3,
// MixDrink, MowTower, Parkour, PhotoFight, Renju, RiskHarvest, Tetris, Zither
// =========================================================================

pub fn onAreaTerminalPinOperationRequest(
    txn: *Transaction(pb.AreaTerminalPinOperationRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "misc:area_pin:{d}", .{txn.message.Id});
    try pb_kv.persistInt(alloc, fs, progress_comp, key, if (txn.message.OpType) 1 else 0);
    txn.respond(.{ .Code = .Success });
}

pub fn onBathChangeCameraRequest(
    txn: *Transaction(pb.BathChangeCameraRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const cam: i64 = if (txn.message.Camera) |c| @intFromEnum(c) else 0;
    try pb_kv.persistInt(alloc, fs, progress_comp, "misc:bath_cam", cam);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onBathPhotoRequest(
    txn: *Transaction(pb.BathPhotoRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "misc:bath_photos", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onPlaceItemOnBoardRequest(
    txn: *Transaction(pb.PlaceItemOnBoardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "misc:board_item:{d}", .{txn.message.ToPlace});
    try pb_kv.saveMsg(alloc, fs, progress_comp, key, txn.message);
    txn.respond(.{ .ToPlace = txn.message.ToPlace, .ErrCode = .Success });
}

pub fn onCommitBoardRequest(
    txn: *Transaction(pb.CommitBoardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "misc:board_commit:{d}", .{txn.message.BoardEntityId});
    try pb_kv.persistClaim(alloc, fs, progress_comp, key);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onConstructTrapRequest(
    txn: *Transaction(pb.ConstructTrapRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const nid = try progress_comp.info.addKeyInt(alloc.gpa, "trap_ent:next", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success, .Id = nid });
}

pub fn onDeconstructTrapRequest(
    txn: *Transaction(pb.DeconstructTrapRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "misc:trap_decon:{d}", .{txn.message.Id});
    try pb_kv.persistClaim(alloc, fs, progress_comp, key);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onDropCatchInfoRequest(
    txn: *Transaction(pb.DropCatchInfoRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    _ = txn.message;
    var infos: std.ArrayList(pb.DropCatchLevelInfo) = .empty;
    for (progress_comp.info.dynamic.ints) |entry| {
        if (std.mem.startsWith(u8, entry.key, "dc:score:")) {
            if (std.fmt.parseInt(i32, entry.key["dc:score:".len..], 10)) |dc_id| {
                var states: std.ArrayList(i32) = .empty;
                const pfx = try std.fmt.allocPrint(alloc.arena, "dc_rw:{d}:", .{dc_id});
                for (progress_comp.info.dynamic.claims) |c| {
                    if (std.mem.startsWith(u8, c, pfx)) {
                        if (std.fmt.parseInt(i32, c[pfx.len..], 10)) |idx| {
                            try states.append(alloc.arena, idx);
                        } else |_| {}
                    }
                }
                try infos.append(alloc.arena, .{
                    .DropCatchId = dc_id,
                    .Score = @intCast(entry.val),
                    .RewardStates = states,
                });
            } else |_| {}
        }
    }
    txn.respond(.{ .ErrorCode = .Success, .Infos = infos });
}

pub fn onDropCatchLevelFinishRequest(
    txn: *Transaction(pb.DropCatchLevelFinishRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "dc:score:{d}", .{txn.message.DropCatchId});
    if (txn.message.Score > progress_comp.info.getKeyInt(key)) {
        try progress_comp.info.setKeyInt(alloc.gpa, key, txn.message.Score);
        try progress_comp.save(fs, alloc.arena);
    }
    txn.respond(.{ .Error = .Success });
}

pub fn onDropCatchLevelRewardRequest(
    txn: *Transaction(pb.DropCatchLevelRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const pfx = try std.fmt.allocPrint(alloc.arena, "dc_rw:{d}", .{txn.message.DropCatchId});
    _ = try claimIdList(alloc, fs, progress_comp, reward_ctx, pfx, "DropCatch", txn.message.RewardIdxs.items);
    txn.respond(.{ .Error = .Success });
}

pub fn onEncircleChallengeStartRequest(
    txn: *Transaction(pb.EncircleChallengeStartRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try pb_kv.persistInt(alloc, fs, progress_comp, "encircle:cur", txn.message.ChallengeId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onEncircleChallengeCompleteRequest(
    txn: *Transaction(pb.EncircleChallengeCompleteRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "encircle:done:{d}", .{txn.message.ChallengeId});
    _ = try progress_comp.info.markClaim(alloc.gpa, key);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onExhibitionItemSaveRequest(
    txn: *Transaction(pb.ExhibitionItemSaveRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try pb_kv.saveMsg(alloc, fs, progress_comp, "misc:exhibition", txn.message);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onFarmGoldLevelDiffRequest(
    txn: *Transaction(pb.FarmGoldLevelDiffRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "fg:diff:{d}", .{txn.message.InstId});
    try pb_kv.persistInt(alloc, fs, progress_comp, key, txn.message.Diff);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onFarmGoldLevelPlayRepeatedRequest(
    txn: *Transaction(pb.FarmGoldLevelPlayRepeatedRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimIdList(alloc, fs, progress_comp, reward_ctx, "fg_lp", "FarmGold", txn.message.InstIdList.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onFarmGoldPointRepeatedRequest(
    txn: *Transaction(pb.FarmGoldPointRepeatedRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimIdList(alloc, fs, progress_comp, reward_ctx, "fg_pt", "FarmGold", txn.message.Ids.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onFarmGoldResultRequest(
    txn: *Transaction(pb.FarmGoldResultRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "fg:runs", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onFixToolRequest(
    txn: *Transaction(pb.FixToolRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "misc:fix_tool:{d}", .{txn.message.EntityId});
    try pb_kv.persistInt(alloc, fs, progress_comp, key, txn.message.FixTool);
    txn.respond(.{ .Code = .Success, .EntityId = txn.message.EntityId });
}

pub fn onFlowerPollutionRequest(
    txn: *Transaction(pb.FlowerPollutionRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "misc:flower:{d}", .{txn.message.TargetEntityId});
    try pb_kv.saveMsg(alloc, fs, progress_comp, key, txn.message);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onFlowerPollutionSplinePush(_: *Transaction(pb.FlowerPollutionSplinePush)) !void {}

pub fn onFoundationRequest(
    txn: *Transaction(pb.FoundationRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "misc:foundation:{d}", .{txn.message.EntityId});
    try pb_kv.persistInt(alloc, fs, progress_comp, key, txn.message.ActiveOperate);
    txn.respond(.{ .ErrorCode = .Success, .ActiveOperate = txn.message.ActiveOperate });
}

pub fn onFoundationCoreOperateRequest(
    txn: *Transaction(pb.FoundationCoreOperateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    scene: *Scene,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "misc:foundation_core:{d}", .{txn.message.FoundationEntityIncId});
    try pb_kv.persistInt(alloc, fs, progress_comp, key, if (txn.message.IsTakeOut) 0 else 1);
    if (!txn.message.IsTakeOut) {
        try pb_kv.persistInt(alloc, fs, progress_comp, "misc:hand_core_box", 0);
        try pb_kv.persistInt(alloc, fs, progress_comp, "misc:hand_core_src", 0);
        txn.respond(.{ .ErrorCode = .Success, .FromFoundationEntityId = 0 });
        return;
    }

    const cfg_id: i32 = if (scene.net_id_map.get(txn.message.FoundationEntityIncId)) |idx|
        scene.entities.items(.config)[idx].config_id
    else
        @intCast(@max(1, @min(txn.message.FoundationEntityIncId, std.math.maxInt(i32))));

    var box_id: i32 = 850001;
    var elem_type_opt: ?[]const u8 = null;
    if (assets.findLevelEntity(scene.instance_id, cfg_id)) |le| {
        for (le.ComponentsData.extra) |entry| {
            switch (entry) {
                .ElementalBoxFoundationComponent => |ef| {
                    if (ef.InitialActiveElementalBoxId) |eid| {
                        if (eid > 0) box_id = @intCast(eid);
                    }
                    if (ef.ElementalType) |et| {
                        if (et.len > 0) elem_type_opt = et;
                    }
                },
                else => {},
            }
        }
    }

    const box_cfg = assets.tables.elemental_box_define.getDataById(box_id) orelse
        assets.tables.elemental_box_define.getDataById(850001) orelse
        (if (assets.tables.elemental_box_define.items.len > 0) assets.tables.elemental_box_define.items[0] else null);

    const resolved_box_id: i32 = if (box_cfg) |bc| bc.Id else box_id;
    const elem_type: []const u8 = elem_type_opt orelse
        (if (box_cfg) |bc| if (bc.ElementalType.len > 0) bc.ElementalType else "关卡.Common.匣子元素.火" else "关卡.Common.匣子元素.火");
    const state_str: []const u8 = if (box_cfg) |bc|
        if (bc.InitState.len > 0) bc.InitState else "关卡.Common.状态.激活"
    else
        "关卡.Common.状态.激活";
    const priority: i32 = if (box_cfg) |bc| bc.Priority else 1;

    try pb_kv.persistInt(alloc, fs, progress_comp, "misc:hand_core_box", resolved_box_id);
    try pb_kv.persistInt(alloc, fs, progress_comp, "misc:hand_core_src", cfg_id);
    txn.respond(.{
        .ErrorCode = .Success,
        .FromFoundationEntityId = cfg_id,
        .FoundationCoreInfo = .{
            .SourceEntityId = cfg_id,
            .ElementalBoxId = resolved_box_id,
            .State = state_str,
            .ElementalType = elem_type,
            .Priority = priority,
            .EntityId = cfg_id,
        },
    });
}

pub fn onPullingFoundationRequest(
    txn: *Transaction(pb.PullingFoundationRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "misc:pull_foundation:{d}", .{txn.message.EntityId});
    try pb_kv.saveMsg(alloc, fs, progress_comp, key, txn.message);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onGolemCrackUpdateRequest(
    txn: *Transaction(pb.GolemCrackUpdateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    if (txn.message.GolemCrackLevelInfo) |info| {
        const key = try std.fmt.allocPrint(alloc.arena, "misc:golem:{d}:{d}", .{ txn.message.ActivityId, info.id });
        try pb_kv.saveMsg(alloc, fs, progress_comp, key, info);
    }
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onHarvestActivityLevelDiffRequest(
    txn: *Transaction(pb.HarvestActivityLevelDiffRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "harv:diff:{d}", .{txn.message.InstId});
    try pb_kv.persistInt(alloc, fs, progress_comp, key, txn.message.Diff);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onHarvestActivityLevelRequest(
    txn: *Transaction(pb.HarvestActivityLevelRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const item_map = try claimOne(alloc, fs, progress_comp, reward_ctx, "harv_lv", "RiskHarvestInst", txn.message.InstId);
    txn.respond(.{ .ErrorCode = .Success, .ItemMap = item_map });
}

pub fn onHarvestActivityPointRequest(
    txn: *Transaction(pb.HarvestActivityPointRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const item_map = try claimOne(alloc, fs, progress_comp, reward_ctx, "harv_pt", "RiskHarvestScoreReward", txn.message.Id);
    txn.respond(.{ .ErrorCode = .Success, .ItemMap = item_map });
}

pub fn onHarvestActivityResultRequest(
    txn: *Transaction(pb.HarvestActivityResultRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "harv:runs", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{});
}

pub fn onInspirationUnlockChoiceRequest(
    txn: *Transaction(pb.InspirationUnlockChoiceRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "insp:{d}:{d}", .{ txn.message.ChapterId, txn.message.ChoiceId });
    _ = try progress_comp.info.markClaim(alloc.gpa, key);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onInvestRequest(
    txn: *Transaction(pb.InvestRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    progress_comp: *PlayerProgressComponent,
    io: std.Io,
) !void {
    if (txn.message.Amount > 0) {
        const costs = [_]ConsumeHelper.CostItem{.{ .id = 15, .count = txn.message.Amount }};
        if (!(try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, &costs, io))) {
            txn.respond(.{ .ErrorCode = .ErrItemNotEnough, .Success = false });
            return;
        }
    }
    const gain: i32 = txn.message.Amount * 2;
    const key = try std.fmt.allocPrint(alloc.arena, "invest:{d}", .{txn.message.EntrustId});
    _ = try progress_comp.info.addKeyInt(alloc.gpa, key, gain);
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "trackmoon:gold", gain);
    try progress_comp.save(fs, alloc.arena);

    const delta: i32 = @max(10, @divTrunc(txn.message.Amount, 10));
    var role_settle: std.ArrayList(pb.RoleProperSettle) = .empty;
    var rid: i32 = 1;
    while (rid <= 3) : (rid += 1) {
        try role_settle.append(alloc.arena, .{
            .RoleId = rid,
            .PropertyA = delta,
            .PropertyB = delta,
            .PropertyC = delta,
            .Level = 1,
        });
    }
    txn.respond(.{
        .Heat = 10,
        .RoleProperSettle = role_settle,
        .Gold = gain,
        .BaseGold = txn.message.Amount,
        .EvaluationLevel = 3,
        .ErrorCode = .Success,
        .Success = true,
    });
}

pub fn onLifePointChallengeStartRequest(
    txn: *Transaction(pb.LifePointChallengeStartRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try pb_kv.persistInt(alloc, fs, progress_comp, "misc:lifepoint_cur", txn.message.Id);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onLineCrossChallengeStartRequest(
    txn: *Transaction(pb.LineCrossChallengeStartRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try pb_kv.persistInt(alloc, fs, progress_comp, "misc:linecross_cur", txn.message.Id);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onMatchThreeStarRequest(
    txn: *Transaction(pb.MatchThreeStarRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "misc:m3_role:{d}", .{txn.message.LevelId});
    try pb_kv.persistInt(alloc, fs, progress_comp, key, txn.message.JoinRole);
    txn.respond(.{ .TraceId = "m3-1", .ErrorCode = .Success });
}

pub fn onMatchThreeEndRequest(
    txn: *Transaction(pb.MatchThreeEndRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "misc:m3_end:{d}", .{txn.message.LevelId});
    try pb_kv.saveMsg(alloc, fs, progress_comp, key, txn.message);
    txn.respond(.{ .ErrorCode = .Success, .Success = true });
}

pub fn onMiniWriteLetterSaveContentRequest(
    txn: *Transaction(pb.MiniWriteLetterSaveContentRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try pb_kv.saveMsg(alloc, fs, progress_comp, "misc:letter", txn.message);
    txn.respond(.{ .ErrorId = .Success });
}

pub fn onMixDrinkRoleInviteRequest(
    txn: *Transaction(pb.MixDrinkRoleInviteRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "mix_inv:{d}", .{txn.message.RoleId});
    _ = try progress_comp.info.markClaim(alloc.gpa, key);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onMixDrinkRoleRewardRequest(
    txn: *Transaction(pb.MixDrinkRoleRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimOne(alloc, fs, progress_comp, reward_ctx, "mix_role_rw", "MixDrink", txn.message.RoleInviteId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onMixDrinkSettleRequest(
    txn: *Transaction(pb.MixDrinkSettleRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "misc:mix_settle:{d}", .{txn.message.RequireId});
    try pb_kv.saveMsg(alloc, fs, progress_comp, key, txn.message);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onMowTowerLevelsRewardRequest(
    txn: *Transaction(pb.MowTowerLevelsRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimOne(alloc, fs, progress_comp, reward_ctx, "mow_rw", "MowTower", txn.message.RewardId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onMowTowerSettlementRequest(
    txn: *Transaction(pb.MowTowerSettlementRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "mow:settle_cnt", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.conn.pushBestEffort(alloc.gpa, io, pb.MowTowerResultNotify{
        .LevelsId = 1,
        .FirstKilMonsterScore = 1000,
        .FirstTimerScore = 500,
        .SecondKilMonsterScore = 1000,
        .SecondTimerScore = 500,
        .HighScore = 3000,
    });
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onOpenSystemBoardResultRequest(
    txn: *Transaction(pb.OpenSystemBoardResultRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "misc:sys_board:{d}", .{txn.message.IncId});
    try pb_kv.persistInt(alloc, fs, progress_comp, key, txn.message.Result);
    txn.respond(.{ .Code = .Success });
}

pub fn onParkourChallengeRequest(
    txn: *Transaction(pb.ParkourChallengeRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    _ = txn.message;
    var open_ids: std.ArrayList(i32) = .empty;
    var challenges: std.ArrayList(pb.ParkourChallenge) = .empty;
    var cid: i32 = 1;
    while (cid <= 6) : (cid += 1) {
        try open_ids.append(alloc.arena, cid);
        const key = try std.fmt.allocPrint(alloc.arena, "parkour:best:{d}", .{cid});
        const best: i32 = @intCast(progress_comp.info.getKeyInt(key));
        try challenges.append(alloc.arena, .{ .ChallengeId = cid, .MaxScore = best });
    }
    txn.respond(.{ .OpenIds = open_ids, .Challenges = challenges });
}

pub fn onParkourChallengeTakeRequest(
    txn: *Transaction(pb.ParkourChallengeTakeRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const pfx = try std.fmt.allocPrint(alloc.arena, "parkour:{d}", .{txn.message.ChallengeId});
    const rws = try claimOne(alloc, fs, progress_comp, reward_ctx, pfx, "Parkour", txn.message.ScoreIndex);
    txn.respond(.{ .Rewards = rws, .ErrorCode = .Success });
}

pub fn onParkourChallengeTransRequest(
    txn: *Transaction(pb.ParkourChallengeTransRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try pb_kv.persistInt(alloc, fs, progress_comp, "parkour:cur", txn.message.ChallengeId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onPhotoFightSettleRequest(
    txn: *Transaction(pb.PhotoFightSettleRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "pf:settle_cnt", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onPhotoFightTaskRewardRequest(
    txn: *Transaction(pb.PhotoFightTaskRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimIdList(alloc, fs, progress_comp, reward_ctx, "pf_task", "PhotoFight", txn.message.TargetIds.items);
    var targets: std.ArrayList(pb.PhotoFightTargetPb) = .empty;
    for (txn.message.TargetIds.items) |tid| {
        try targets.append(alloc.arena, .{ .Id = tid, .Status = .ActivityTaskTaken });
    }
    txn.respond(.{ .Targets = targets, .ErrorCode = .Success });
}

pub fn onPhotoMemoryRewardRequest(
    txn: *Transaction(pb.PhotoMemoryRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimIdList(alloc, fs, progress_comp, reward_ctx, "pmem_rw", "PhotoMemory", txn.message.CollectIds.items);
    txn.respond(.{ .Error = .Success });
}

pub fn onRenjuExitMatchedActionRequest(
    txn: *Transaction(pb.RenjuExitMatchedActionRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "misc:renju_exit:{d}", .{txn.message.EntityId});
    try pb_kv.persistClaim(alloc, fs, progress_comp, key);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onRenjuResetRequest(
    txn: *Transaction(pb.RenjuResetRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "misc:renju_reset:{d}", .{txn.message.EntityId});
    _ = try progress_comp.info.addKeyInt(alloc.gpa, key, 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onRiskHarvestInstRewardRequest(
    txn: *Transaction(pb.RiskHarvestInstRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimOne(alloc, fs, progress_comp, reward_ctx, "rh_inst", "RiskHarvestInst", txn.message.Id);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRiskHarvestScoreRewardRequest(
    txn: *Transaction(pb.RiskHarvestScoreRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimOne(alloc, fs, progress_comp, reward_ctx, "rh_score", "RiskHarvestScoreReward", txn.message.Id);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRiskHarvestStarRewardRequest(
    txn: *Transaction(pb.RiskHarvestStarRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const pfx = try std.fmt.allocPrint(alloc.arena, "rh_star:{d}", .{txn.message.Id});
    _ = try claimOne(alloc, fs, progress_comp, reward_ctx, pfx, "RiskHarvestInst", txn.message.Index);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRiskHarvestSettleRequest(
    txn: *Transaction(pb.RiskHarvestSettleRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "harv:settle_cnt", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSunSpiritActionOperationEndPush(_: *Transaction(pb.SunSpiritActionOperationEndPush)) !void {}

pub fn onTetrisFinishRequest(
    txn: *Transaction(pb.TetrisFinishRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    if (txn.message.Score > progress_comp.info.getKeyInt("tetris:best")) {
        try progress_comp.info.setKeyInt(alloc.gpa, "tetris:best", txn.message.Score);
        try progress_comp.save(fs, alloc.arena);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onTetrisUpdateRequest(
    txn: *Transaction(pb.TetrisUpdateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    if (txn.message.TetrisLevelInfo) |info| {
        const key = try std.fmt.allocPrint(alloc.arena, "misc:tetris:{d}", .{txn.message.ActivityId});
        try pb_kv.saveMsg(alloc, fs, progress_comp, key, info);
    }
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onVacuumCleanerRequest(
    txn: *Transaction(pb.VacuumCleanerRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "misc:vacuum:{d}", .{txn.message.AdsorbEntityIncId});
    _ = try progress_comp.info.addKeyInt(alloc.gpa, key, 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onZitherPlayNotePush(_: *Transaction(pb.ZitherPlayNotePush)) !void {}

pub fn onZitherPlayStateRequest(
    txn: *Transaction(pb.ZitherPlayStateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const st: i64 = if (txn.message.State) |s| @intFromEnum(s) else 0;
    try pb_kv.persistInt(alloc, fs, progress_comp, "misc:zither_state", st);
    txn.respond(.{ .ErrCode = .Success });
}
