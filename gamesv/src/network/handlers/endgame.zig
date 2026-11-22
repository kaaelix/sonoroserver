const std = @import("std");
const common = @import("common");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const Assets = @import("../../data/Assets.zig");
const PlayerBasicComponent = @import("../../logic/component/player/PlayerBasicComponent.zig");
const PlayerProgressComponent = @import("../../logic/component/player/PlayerProgressComponent.zig");
const PlayerRoleComponent = @import("../../logic/component/player/PlayerRoleComponent.zig");
const RewardService = @import("../../logic/helpers/reward.zig");
const EnergyHelper = @import("../../logic/helpers/energy.zig");
const QuestEngine = @import("../../logic/helpers/quest_engine.zig");
const LevelPlayHelper = @import("../../logic/helpers/level_play.zig");
const Transaction = @import("../handlers.zig").Transaction;
const PlayerID = @import("../../logic/PlayerID.zig");

const FileSystem = common.FileSystem;

fn claimRewardList(
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
            19500,
            &.{ .{ .item_id = 3, .count = 50 }, .{ .item_id = 2, .count = 15000 } },
            null,
        );
    }
    if (dirty) try progress_comp.save(fs, arena);
    return claimed;
}

// === SlashAndTower (Whimpering Wastes) ===
pub fn onSlashAndTowerInfoRequest(
    txn: *Transaction(pb.SlashAndTowerInfoRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
    progress_comp: *const PlayerProgressComponent,
    io: std.Io,
) !void {
    const now_s = EnergyHelper.nowSeconds(io);
    const rows = assets.reward_index.getRows(io, "SlashAndTowerCfg");
    var plays: std.ArrayList(pb.SlashLevelPlayInfo) = .empty;
    var kbuf: [48]u8 = undefined;
    for (rows) |row| {
        const id: i32 = @intCast(row.key);
        if (id <= 0) continue;
        const s1_k = try std.fmt.bufPrint(&kbuf, "slash:s1:{d}", .{id});
        const s2_k = try std.fmt.bufPrint(&kbuf, "slash:s2:{d}", .{id});
        const s1 = @as(i32, @intCast(progress_comp.info.getKeyInt(s1_k)));
        const s2 = @as(i32, @intCast(progress_comp.info.getKeyInt(s2_k)));
        try plays.append(alloc.arena, .{
            .Id = id,
            .IsLocked = false,
            .FirstScore = s1,
            .SecondScore = s2,
            .IsPassed = (s1 + s2) > 0,
        });
    }
    var recvd: std.ArrayList(i32) = .empty;
    for (progress_comp.info.dynamic.claims) |k| {
        if (std.mem.startsWith(u8, k, "slash:clm:")) {
            const rid = std.fmt.parseInt(i32, k["slash:clm:".len..], 10) catch continue;
            try recvd.append(alloc.arena, rid);
        }
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .SlashLevelPlayInfo = plays,
        .RewardsReceived = recvd,
        .CurSeasonEndTime = now_s + 2_592_000,
    });
}

pub fn onSlashAndTowerRecommendRequest(
    txn: *Transaction(pb.SlashAndTowerRecommendRequest),
    alloc: mem.Alloc,
) !void {
    _ = txn.message;
    var infos: std.ArrayList(pb.SlashLevelRecommend) = .empty;
    try infos.append(alloc.arena, .{ .Usage = 1 });
    txn.respond(.{ .ErrorCode = .Success, .RecommendInfos = infos });
}

pub fn onSlashAndTowerResetRequest(
    txn: *Transaction(pb.SlashAndTowerResetRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const s1_k = try std.fmt.bufPrint(&kbuf, "slash:s1:{d}", .{txn.message.Id});
    const s2_k = try std.fmt.bufPrint(&kbuf, "slash:s2:{d}", .{txn.message.Id});
    try progress_comp.info.setKeyInt(alloc.gpa, s1_k, 0);
    try progress_comp.info.setKeyInt(alloc.gpa, s2_k, 0);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSlashAndTowerSaveRecordRequest(
    txn: *Transaction(pb.SlashAndTowerSaveRecordRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "slash:saved:{d}", .{txn.message.Id});
    try progress_comp.info.setKeyInt(alloc.gpa, k, 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSlashAndTowerScoreRewardRequest(
    txn: *Transaction(pb.SlashAndTowerScoreRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const claimed = try claimRewardList(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "slash:clm", "SlashAndTowerReward", txn.message.RewardIds.items);
    txn.respond(.{ .ErrorCode = .Success, .RewardIds = claimed });
}

// === NewTower ===
pub fn onNewTowerClearRecordRequest(
    txn: *Transaction(pb.NewTowerClearRecordRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "newtower:score:{d}", .{txn.message.LevelId});
    try progress_comp.info.setKeyInt(alloc.gpa, k, 0);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onNewTowerCycleStartRequest(
    txn: *Transaction(pb.NewTowerCycleStartRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "newtower:cycle", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onNewTowerLastCycleInfoRequest(
    txn: *Transaction(pb.NewTowerLastCycleInfoRequest),
    progress_comp: *const PlayerProgressComponent,
) !void {
    _ = txn.message;
    const cycle_id: i32 = @intCast(@max(1, progress_comp.info.getKeyInt("newtower:cycle")));
    txn.respond(.{ .ErrorCode = .Success, .NewTowerCycleRecordPb = .{ .CycleId = cycle_id } });
}

pub fn onNewTowerMedalInfoRequest(
    txn: *Transaction(pb.NewTowerMedalInfoRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    var medals: std.ArrayList(pb.NewTowerMedalInfo) = .empty;
    const score: i32 = @intCast(@max(0, progress_comp.info.getKeyInt("new_tower:all_score")));
    try medals.append(alloc.arena, .{
        .MedalGroupId = 1,
        .SeasonId = 1,
        .CycleId = @intCast(@max(1, progress_comp.info.getKeyInt("newtower:cycle"))),
        .Progress = score,
        .CompleteTime = if (score > 0) 1 else 0,
    });
    txn.respond(.{ .NewTowerMedalInfos = medals });
}

pub fn onNewTowerRecommendRequest(txn: *Transaction(pb.NewTowerRecommendRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success, .NewTowerRecommendData = .{} });
}

pub fn onNewTowerRewardRequest(
    txn: *Transaction(pb.NewTowerRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimRewardList(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "newtower:clm", "NewTowerScoreReward", txn.message.Indexs.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onNewTowerRoleEnergyRequest(
    txn: *Transaction(pb.NewTowerRoleEnergyRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    _ = txn.message;
    var energies: std.ArrayList(pb.RecordRoleEnergy) = .empty;
    const lv: i32 = @intCast(@max(1, progress_comp.info.getKeyInt("newtower:cycle")));
    try energies.append(alloc.arena, .{ .LevelId = lv });
    txn.respond(.{ .ErrorCode = .Success, .RecordRoleEnergys = energies });
}

pub fn onNewTowerSeasonRewardRequest(
    txn: *Transaction(pb.NewTowerSeasonRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimRewardList(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "newtower:sclm", "NewTowerSeasonAward", txn.message.Indexs.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onNewTowerSeasonScoreRewardRequest(
    txn: *Transaction(pb.NewTowerSeasonScoreRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimRewardList(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "newtower:ssclm", "NewTowerSeasonSAward", txn.message.Indexs.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onNewTowerSeasonTaskDataUpdateRequest(
    txn: *Transaction(pb.NewTowerSeasonTaskDataUpdateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "new_tower:task_sync", 1);
    try progress_comp.save(fs, alloc.arena);
    var tasks: std.ArrayList(pb.ActivityTask) = .empty;
    const score: i32 = @intCast(@max(0, progress_comp.info.getKeyInt("new_tower:all_score")));
    var kbuf: [48]u8 = undefined;
    for (assets.tables.new_tower_season_award.items) |row| {
        const k = try std.fmt.bufPrint(&kbuf, "newtower:sclm:{d}", .{row.Id});
        const claimed = progress_comp.info.hasClaim(k);
        const status: pb.ActivityTaskState = if (claimed) .ActivityTaskTaken else if (score > 0) .ActivityTaskFinish else .ActivityTaskRunning;
        try tasks.append(alloc.arena, .{
            .Id = row.Id,
            .Current = if (claimed or score > 0) 1 else 0,
            .Target = 1,
            .Status = status,
        });
    }
    txn.respond(.{ .SeasonTasks = tasks });
}

pub fn onNewTowerUpdateRecordRequest(
    txn: *Transaction(pb.NewTowerUpdateRecordRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "newtower:score:{d}", .{txn.message.LevelId});
    const score = @as(i32, @intCast(progress_comp.info.getKeyInt(k)));
    _ = alloc;
    txn.respond(.{
        .ErrorCode = .Success,
        .Record = .{ .LevelId = txn.message.LevelId, .Score = score, .HistoryScore = score, .IsUnlock = true },
    });
}

// === RoleTower ===
pub fn onRoleTowerAttributeQueryRequest(
    txn: *Transaction(pb.RoleTowerAttributeQueryRequest),
    alloc: mem.Alloc,
    role_comp: *const PlayerRoleComponent,
) !void {
    var infos: std.ArrayList(pb.RoleTowerRoleAttributeInfo) = .empty;
    for (txn.message.RoleIds.items) |rid| {
        var base_prop: std.ArrayList(pb.ArrayIntInt) = .empty;
        const level = if (role_comp.role_map.get(rid)) |r| r.level else 90;
        try base_prop.append(alloc.arena, .{ .Key = 1, .Value = level });
        try infos.append(alloc.arena, .{ .RoleProp = .{ .RoleId = rid, .BaseProp = base_prop } });
    }
    txn.respond(.{ .ErrorCode = .Success, .RoleInfos = infos, .PhantomLevel = 25 });
}

pub fn onRoleTowerChallengeSettleRequest(
    txn: *Transaction(pb.RoleTowerChallengeSettleRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    const score = try progress_comp.info.addKeyInt(alloc.gpa, "role_tower:score", 1000);
    const floor = try progress_comp.info.addKeyInt(alloc.gpa, "role_tower:stage", 1);
    try progress_comp.save(fs, alloc.arena);
    if (LevelPlayHelper.fromQuestContext(qctx)) |lp_ctx| {
        LevelPlayHelper.onConditionEvent(lp_ctx, progress_comp, .finish_role_tower, 0, @intCast(floor)) catch {};
    }
    const rec: pb.RoleTowerBestRecord = .{
        .ChallengeId = txn.message.ChallengeId,
        .PassedFloorCount = @intCast(@max(0, floor)),
        .IsAllPassed = true,
        .IsNewRecord = true,
    };
    _ = score;
    txn.respond(.{
        .ErrorCode = .Success,
        .ChallengeSettleInfo = .{
            .ChallengeId = txn.message.ChallengeId,
            .CurrentRecord = rec,
            .BestRecord = rec,
            .IsBestRecordUpdated = true,
        },
    });
}

pub fn onRoleTowerConsumeItemRequest(
    txn: *Transaction(pb.RoleTowerConsumeItemRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "roletower:item:{d}", .{txn.message.ItemId});
    _ = try progress_comp.info.addKeyInt(alloc.gpa, k, 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRoleTowerPhantomAddAttrsRequest(
    txn: *Transaction(pb.RoleTowerPhantomAddAttrsRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "role_tower:attr_cnt:{d}", .{txn.message.ChallengeId});
    try progress_comp.info.setKeyInt(alloc.gpa, k, @intCast(txn.message.RoleIds.items.len));
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRoleTowerRankRequest(
    txn: *Transaction(pb.RoleTowerRankRequest),
    alloc: mem.Alloc,
    player_id: PlayerID,
    basic_comp: *const PlayerBasicComponent,
    progress_comp: *const PlayerProgressComponent,
) !void {
    _ = txn.message;
    var ranks: std.ArrayList(pb.RoleTowerRankInfo) = .empty;
    const floor: i32 = @intCast(@max(0, progress_comp.info.getKeyInt("role_tower:stage")));
    const self_entry: pb.RoleTowerRankEntry = .{
        .PlayerId = player_id.id,
        .Name = basic_comp.info.attributes.name,
        .Rank = 1,
        .RankPercent = 1,
        .Record = .{ .ChallengeId = 1, .PassedFloorCount = floor, .IsAllPassed = floor > 0 },
    };
    try ranks.append(alloc.arena, .{
        .ChallengeId = 1,
        .SelfServerEntry = self_entry,
        .CanGetServerRankReward = floor > 0,
    });
    txn.respond(.{ .ErrorCode = .Success, .RankInfo = ranks });
}

pub fn onRoleTowerRankRewardReceiveRequest(
    txn: *Transaction(pb.RoleTowerRankRewardReceiveRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimRewardList(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "roletower:rank", "RoleTowerRankReward", txn.message.ChallengeIds.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRoleTowerRewardTaskReceiveRequest(
    txn: *Transaction(pb.RoleTowerRewardTaskReceiveRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimRewardList(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "roletower:task", "RoleTowerTask", txn.message.TaskIds.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRoleTowerSaveDeleteRequest(
    txn: *Transaction(pb.RoleTowerSaveDeleteRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "roletower:step:{d}", .{txn.message.ChallengeId});
    try progress_comp.info.setKeyInt(alloc.gpa, k, 0);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRoleTowerScoreRewardReceiveRequest(
    txn: *Transaction(pb.RoleTowerScoreRewardReceiveRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimRewardList(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "roletower:score", "RoleTowerScoreReward", txn.message.RewardIds.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRoleTowerStageRetryRequest(
    txn: *Transaction(pb.RoleTowerStageRetryRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "role_tower:retry:{d}", .{txn.message.ChallengeId});
    _ = try progress_comp.info.addKeyInt(alloc.gpa, k, 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRoleTowerStageSettleRequest(
    txn: *Transaction(pb.RoleTowerStageSettleRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    const floor = try progress_comp.info.addKeyInt(alloc.gpa, "role_tower:stage", 1);
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "role_tower:stage:{d}", .{floor});
    _ = try progress_comp.info.markClaim(alloc.gpa, k);
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "role_tower:score", 500);
    try progress_comp.save(fs, alloc.arena);
    _ = txn.message;
    if (LevelPlayHelper.fromQuestContext(qctx)) |lp_ctx| {
        LevelPlayHelper.onConditionEvent(lp_ctx, progress_comp, .finish_role_tower, 0, @intCast(floor)) catch {};
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRoleTowerStepAdvanceRequest(
    txn: *Transaction(pb.RoleTowerStepAdvanceRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    _ = txn.message;
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "roletower:steps", 1);
    try progress_comp.save(fs, alloc.arena);
    if (LevelPlayHelper.fromQuestContext(qctx)) |lp_ctx| {
        LevelPlayHelper.onConditionEvent(lp_ctx, progress_comp, .finish_role_tower, 0, 0) catch {};
    }
    txn.respond(.{ .ErrorCode = .Success });
}

// === Abyss ===
pub fn onAbyssChallengeGotoNextRoomRequest(
    txn: *Transaction(pb.AbyssChallengeGotoNextRoomRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "abyss:room", txn.message.RoomId);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onAbyssChallengeQuitRequest(
    txn: *Transaction(pb.AbyssChallengeQuitRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "abyss:room", 0);
    try progress_comp.save(fs, alloc.arena);
    _ = txn.message;
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onAbyssChallengeUnlockRequest(
    txn: *Transaction(pb.AbyssChallengeUnlockRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "abyss:unl:{d}", .{txn.message.ChallengeId});
    _ = try progress_comp.info.markClaim(alloc.gpa, k);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onAbyssLikeRequest(
    txn: *Transaction(pb.AbyssLikeRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "abyss:like:{d}", .{txn.message.PlayerId});
    _ = try progress_comp.info.markClaim(alloc.gpa, k);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onAbyssLittleRoleLevelUpRequest(
    txn: *Transaction(pb.AbyssLittleRoleLevelUpRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    if (txn.message.Id <= 0 or txn.message.Level <= 0 or assets.tables.abyss_little_role.getDataById(txn.message.Id) == null) {
        txn.respond(.{ .ErrorCode = .ErrAbyssRoleLevelConfig });
        return;
    }
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "abyss:lrole:{d}", .{txn.message.Id});
    try progress_comp.info.setKeyInt(alloc.gpa, k, @max(1, txn.message.Level));
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
    try QuestEngine.onConditionStateChanged(qctx);
}

pub fn onAbyssPluginPutRequest(
    txn: *Transaction(pb.AbyssPluginPutRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "abyss:plugin_cnt:{d}", .{txn.message.LittleRoleId});
    try progress_comp.info.setKeyInt(alloc.gpa, k, @intCast(txn.message.EquipItems.items.len));
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onAbyssPluginSynthesisRequest(
    txn: *Transaction(pb.AbyssPluginSynthesisRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "abyss:plugin_synth", 1);
    try progress_comp.save(fs, alloc.arena);
    var out: std.ArrayList(pb.AddCountItemInfo) = .empty;
    if (txn.message.PluginItemIncrId.items.len > 0) {
        try out.append(alloc.arena, .{ .Id = 1, .Count = 1, .IncrId = txn.message.PluginItemIncrId.items[0] });
    }
    txn.respond(.{ .ErrorCode = .Success, .OutputItems = out });
}

pub fn onAbyssRankListRequest(
    txn: *Transaction(pb.AbyssRankListRequest),
    alloc: mem.Alloc,
    player_id: PlayerID,
    basic_comp: *const PlayerBasicComponent,
    progress_comp: *const PlayerProgressComponent,
) !void {
    _ = txn.message;
    var self_list: std.ArrayList(pb.AbyssChallengeInfo) = .empty;
    var pinfos: std.ArrayList(pb.AbyssChallengePassPlayerInfo) = .empty;
    try pinfos.append(alloc.arena, .{ .PlayerId = player_id.id, .Name = basic_comp.info.attributes.name });
    try self_list.append(alloc.arena, .{
        .PlayerInfos = pinfos,
        .ChallengeId = 1,
        .PassWave = @intCast(@max(0, progress_comp.info.getKeyInt("abyss:room"))),
        .IsSingle = true,
        .OwnerId = player_id.id,
    });
    txn.respond(.{ .ErrorCode = .Success, .SelfPassData = self_list });
}

pub fn onAbyssReviveCharacterRequest(
    txn: *Transaction(pb.AbyssReviveCharacterRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    io: std.Io,
    progress_comp: *PlayerProgressComponent,
) !void {
    const rev_cnt = try progress_comp.info.addKeyInt(alloc.gpa, "abyss:revive_cnt", 1);
    try progress_comp.save(fs, alloc.arena);
    try txn.conn.push(alloc.gpa, io, pb.AbyssReviveNotify{
        .MaxReviveTimes = 3,
        .CurReviveTimes = @intCast(rev_cnt),
    });
    _ = txn.message;
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onAbyssRewardRequest(
    txn: *Transaction(pb.AbyssRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    if (txn.message.Ids.items.len > 0) {
        _ = try claimRewardList(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "abyss:clm", "AbyssReward", txn.message.Ids.items);
    } else if (txn.message.Id > 0) {
        const single = [_]i32{txn.message.Id};
        _ = try claimRewardList(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "abyss:clm", "AbyssReward", &single);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onAbyssSelfPassRequest(
    txn: *Transaction(pb.AbyssSelfPassRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "abyss:unl:{d}", .{txn.message.Id});
    var list: std.ArrayList(pb.AbyssChallengeInfo) = .empty;
    if (progress_comp.info.hasClaim(k)) {
        try list.append(alloc.arena, .{ .ChallengeId = txn.message.Id, .IsSingle = true });
    }
    txn.respond(.{ .SelfPassData = list });
}

pub fn onAbyssShowNameRequest(
    txn: *Transaction(pb.AbyssShowNameRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "abyss:showname", if (txn.message.ShowName) 1 else 0);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

// === BossRush & BossPiling ===
pub fn onBossRushBatchClaimRewardRequest(
    txn: *Transaction(pb.BossRushBatchClaimRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimRewardList(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "bossrush:task", "BossRushTaskConfig", txn.message.TaskRewardIds.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onBossRushBuffChooseResultRequest(
    txn: *Transaction(pb.BossRushBuffChooseResultRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "bossrush:buff_idx", txn.message.Index);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onBossRushChooseBuffRequest(
    txn: *Transaction(pb.BossRushChooseBuffRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
) !void {
    const pick_cnt = try progress_comp.info.addKeyInt(alloc.gpa, "bossrush:buff_pick", 1);
    var buff_ids: std.ArrayList(i32) = .empty;
    const all_buffs = assets.tables.boss_rush_buff.items;
    if (all_buffs.len > 0) {
        const count = @min(@as(usize, 3), all_buffs.len);
        const start: usize = @intCast(@mod(@max(@as(i64, 0), pick_cnt - 1) * 3, @as(i64, @intCast(all_buffs.len))));
        for (0..count) |i| {
            const bid = all_buffs[(start + i) % all_buffs.len].Id;
            try buff_ids.append(alloc.arena, bid);
            var kbuf: [48]u8 = undefined;
            const ok = try std.fmt.bufPrint(&kbuf, "bossrush:offer:{d}", .{i});
            try progress_comp.info.setKeyInt(alloc.gpa, ok, bid);
        }
    }
    try progress_comp.save(fs, alloc.arena);
    _ = txn.message;
    txn.respond(.{ .ErrorCode = .Success, .BuffIds = buff_ids });
}

pub fn onBossRushClaimRewardRequest(
    txn: *Transaction(pb.BossRushClaimRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const single = [_]i32{txn.message.RewardId};
    _ = try claimRewardList(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "bossrush:clm", "BossRushScore", &single);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onBossRushLevelScoreRewardRequest(
    txn: *Transaction(pb.BossRushLevelScoreRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const single = [_]i32{txn.message.Index};
    _ = try claimRewardList(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "bossrush:lscore", "BossRushScore", &single);
    txn.respond(.{ .Error = .Success });
}

pub fn onBossRushSettlementRequest(
    txn: *Transaction(pb.BossRushSettlementRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const score = try progress_comp.info.addKeyInt(alloc.gpa, "bossrush:score", 2000);
    try progress_comp.save(fs, alloc.arena);
    _ = txn.message;
    txn.conn.pushBestEffort(alloc.gpa, io, pb.BossRushResultNotify{
        .ActivityId = 1,
        .InstId = 1,
        .KilMonsterScore = 1000,
        .TimerScore = 500,
        .TechnicalScore = 500,
        .HighScore = @intCast(@max(2000, score)),
        .LeftTime = 60,
    });
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onBossRushTaskRewardRequest(
    txn: *Transaction(pb.BossRushTaskRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const single = [_]i32{txn.message.TaskId};
    _ = try claimRewardList(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "bossrush:task", "BossRushTaskConfig", &single);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onBossPilingRewardRequest(
    txn: *Transaction(pb.BossPilingRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = try claimRewardList(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "bosspiling:task", "BossPilingTask", txn.message.BossPilingTaskIds.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onBossPilingSettleRequest(
    txn: *Transaction(pb.BossPilingSettleRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "bp:hp:1001", 100);
    try progress_comp.info.setKeyInt(alloc.gpa, "bp:max_hp", 100);
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "bp:score", 1000);
    try progress_comp.save(fs, alloc.arena);
    _ = txn.message;
    txn.respond(.{ .ErrorCode = .Success });
}

// === BabelTower & EndLessHistory ===
pub fn onBabelTowerDailyTaskRewardRequest(
    txn: *Transaction(pb.BabelTowerDailyTaskRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const single = [_]i32{txn.message.TaskId};
    _ = try claimRewardList(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "babel:daily", "BabelTowerDailyTask", &single);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onBabelTowerRankListRequest(
    txn: *Transaction(pb.BabelTowerRankListRequest),
    player_id: PlayerID,
    basic_comp: *const PlayerBasicComponent,
) !void {
    _ = txn.message;
    txn.respond(.{
        .ErrorCode = .Success,
        .SelfPassData = .{
            .PlayerId = player_id.id,
            .Name = basic_comp.info.attributes.name,
            .HeadId = basic_comp.info.attributes.head_photo,
        },
    });
}

pub fn onBabelTowerSelfPassRequest(
    txn: *Transaction(pb.BabelTowerSelfPassRequest),
    player_id: PlayerID,
    basic_comp: *const PlayerBasicComponent,
) !void {
    _ = txn.message;
    txn.respond(.{
        .ErrorCode = .Success,
        .SelfPassData = .{
            .PlayerId = player_id.id,
            .Name = basic_comp.info.attributes.name,
            .HeadId = basic_comp.info.attributes.head_photo,
        },
    });
}

pub fn onBabelTowerSettlementRequest(
    txn: *Transaction(pb.BabelTowerSettlementRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "babel:finish", 1);
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "babel:star", 3);
    try progress_comp.save(fs, alloc.arena);
    _ = txn.message;
    txn.conn.pushBestEffort(alloc.gpa, io, pb.BabelActivitySettleInfoNotify{
        .IsPass = true,
        .Star = 3,
        .PassTime = 60,
        .UseTime = 60,
        .IsActivityClose = false,
    });
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onBabelTowerShowNameRequest(
    txn: *Transaction(pb.BabelTowerShowNameRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "babel:showname", if (txn.message.ShowName) 1 else 0);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onBabelTowerTaskRewardRequest(
    txn: *Transaction(pb.BabelTowerTaskRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    if (txn.message.TaskIds.items.len > 0) {
        _ = try claimRewardList(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "babel:task", "BabelTowerTask", txn.message.TaskIds.items);
    } else if (txn.message.TaskId > 0) {
        const single = [_]i32{txn.message.TaskId};
        _ = try claimRewardList(reward_ctx, progress_comp, fs, alloc.gpa, alloc.arena, "babel:task", "BabelTowerTask", &single);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onResetBabelTowerLevelRequest(
    txn: *Transaction(pb.ResetBabelTowerLevelRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "babel:cur_level", 0);
    try progress_comp.save(fs, alloc.arena);
    _ = txn.message;
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSelectBabelActivityDeTermRequest(
    txn: *Transaction(pb.SelectBabelActivityDeTermRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "babel:determ_cnt", 1);
    try progress_comp.save(fs, alloc.arena);
    _ = txn.message;
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onEndLessHistoryRequest(
    txn: *Transaction(pb.EndLessHistoryRequest),
    io: std.Io,
    progress_comp: *const PlayerProgressComponent,
) !void {
    const rtc: std.Io.Clock = .real;
    const now_s = rtc.now(io).toSeconds();
    var best = progress_comp.info.getKeyInt("slash_tower:score");
    var best_id: i32 = 1;
    var best_s1: i32 = 0;
    var best_s2: i32 = 0;
    var kbuf: [48]u8 = undefined;
    for (progress_comp.info.dynamic.ints) |entry| {
        if (!std.mem.startsWith(u8, entry.key, "slash:s1:")) continue;
        const id = std.fmt.parseInt(i32, entry.key["slash:s1:".len..], 10) catch continue;
        const s2_k = std.fmt.bufPrint(&kbuf, "slash:s2:{d}", .{id}) catch continue;
        const s1: i64 = @max(0, entry.val);
        const s2: i64 = @max(0, progress_comp.info.getKeyInt(s2_k));
        if (s1 + s2 > best) {
            best = s1 + s2;
            best_id = id;
            best_s1 = @intCast(s1);
            best_s2 = @intCast(s2);
        }
    }
    _ = txn.message;
    const rec: ?pb.EndlessBattleRecord = if (best > 0) .{
        .Id = best_id,
        .FirstRecord = if (best_s1 > 0) .{ .MaxScore = best_s1 } else null,
        .SecondRecord = if (best_s2 > 0) .{ .MaxScore = best_s2 } else null,
        .SaveTime = now_s,
    } else null;
    txn.respond(.{
        .ErrorCode = .Success,
        .NowSeasonRecord = rec,
        .HistoryRecord = rec,
    });
}
