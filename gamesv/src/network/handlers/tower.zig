const std = @import("std");
const common = @import("common");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const Assets = @import("../../data/Assets.zig");
const ProgressInfo = @import("../../fs/ProgressInfo.zig");
const PlayerProgressComponent = @import("../../logic/component/player/PlayerProgressComponent.zig");
const PlayerBasicComponent = @import("../../logic/component/player/PlayerBasicComponent.zig");
const PlayerInventoryComponent = @import("../../logic/component/player/PlayerInventoryComponent.zig");
const PlayerWeaponComponent = @import("../../logic/component/player/PlayerWeaponComponent.zig");
const PlayerEchoComponent = @import("../../logic/component/player/PlayerEchoComponent.zig");
const CraftHelper = @import("../../logic/helpers/craft.zig");
const RewardService = @import("../../logic/helpers/reward.zig");
const QuestEngine = @import("../../logic/helpers/quest_engine.zig");
const EnergyHelper = @import("../../logic/helpers/energy.zig");
const Transaction = @import("../handlers.zig").Transaction;

const FileSystem = common.FileSystem;

fn buildDifficultyPb(
    arena: std.mem.Allocator,
    diff: i32,
    assets: *const Assets,
    progress: ProgressInfo,
) !pb.TowerDifficultyPb {
    var claimed: std.ArrayList(i32) = .empty;
    for (progress.challenge.tower_claimed_rewards) |rec| {
        if (rec.difficulty == diff) try claimed.append(arena, rec.reward_index);
    }

    var area_nums: [8]i32 = undefined;
    var area_count: usize = 0;
    for (assets.tables.tower_config.items) |cfg| {
        if (cfg.Difficulty != diff) continue;
        if (std.mem.indexOfScalar(i32, area_nums[0..area_count], cfg.AreaNum) == null and area_count < area_nums.len) {
            area_nums[area_count] = cfg.AreaNum;
            area_count += 1;
        }
    }

    var areas: std.ArrayList(pb.TowerAreaPb) = .empty;
    var max_star: i32 = 0;
    for (area_nums[0..area_count]) |anum| {
        var floors: std.ArrayList(pb.TowerFloorPb) = .empty;
        for (progress.challenge.tower_floors) |fl| {
            if (fl.difficulty != diff or fl.area_num != anum) continue;
            var star_idx: std.ArrayList(i32) = .empty;
            for ([_]i32{ 1, 2, 3 }) |bit_i| {
                const shift: u5 = @intCast(bit_i - 1);
                if ((fl.star_mask & (@as(i32, 1) << shift)) != 0) {
                    try star_idx.append(arena, bit_i);
                }
            }
            max_star += fl.star;
            try floors.append(arena, .{
                .TowerConfigId = fl.config_id,
                .Star = fl.star,
                .StarIndex = star_idx,
            });
        }
        try areas.append(arena, .{
            .AreaNum = anum,
            .TowerFloors = floors,
        });
    }

    return .{
        .Difficulty = diff,
        .RewardIndex = claimed,
        .TowerAreas = areas,
        .MaxStar = max_star,
    };
}

pub fn buildTowerInfo(
    arena: std.mem.Allocator,
    assets: *const Assets,
    progress: ProgressInfo,
    now_sec: i64,
) !pb.TowerInfo {
    var diffs: std.ArrayList(pb.TowerDifficultyPb) = .empty;
    for (assets.tables.tower_difficulty.items) |d| {
        try diffs.append(arena, try buildDifficultyPb(arena, d.Difficulty, assets, progress));
    }
    return .{
        .CurrentSeason = progress.challenge.tower_season,
        .DataSeason = progress.challenge.tower_season,
        .TowerDifficulties = diffs,
        .BeginTime = now_sec - 86_400,
        .EndTime = now_sec + 2_592_000,
        .MaxUnlockDifficulty = 4,
    };
}

pub fn onTowerRequest(
    txn: *Transaction(pb.TowerRequest),
    assets: *const Assets,
    progress_comp: *const PlayerProgressComponent,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    txn.respond(.{
        .TowerInfo = try buildTowerInfo(alloc.arena, assets, progress_comp.info, EnergyHelper.nowSeconds(io)),
    });
}

pub fn onTowerSeasonUpdateRequest(
    txn: *Transaction(pb.TowerSeasonUpdateRequest),
    assets: *const Assets,
    progress_comp: *const PlayerProgressComponent,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    txn.respond(.{
        .Towers = .{ .TowerInfo = try buildTowerInfo(alloc.arena, assets, progress_comp.info, EnergyHelper.nowSeconds(io)) },
        .MaxUnlockDifficulty = 4,
    });
}

pub fn onTowerStartRequest(
    txn: *Transaction(pb.TowerStartRequest),
    assets: *const Assets,
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    const cfg = assets.tables.tower_config.getDataById(txn.message.TowerConfigId) orelse {
        txn.respond(.{ .ErrorCode = .ErrTowerConfigNotFound, .NeedUpdateSeason = false });
        return;
    };
    try progress_comp.info.setKeyInt(alloc.gpa, "tower:start_ms", EnergyHelper.nowSeconds(io) * 1000);
    try progress_comp.save(fs, alloc.arena);
    if (cfg.InstanceId > 0) {
        try QuestEngine.enterInst(qctx, cfg.InstanceId, 0);
    }
    txn.respond(.{ .ErrorCode = .Success, .NeedUpdateSeason = false });
}

pub fn onTowerResetRequest(
    txn: *Transaction(pb.TowerResetRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
) !void {
    const id = txn.message.TowerConfigId;
    const cfg = assets.tables.tower_config.getDataById(id) orelse {
        txn.respond(.{ .TowerConfigId = id, .ErrorCode = .ErrTowerConfigNotFound });
        return;
    };

    const old_floors = progress_comp.info.challenge.tower_floors;
    var keep_count: usize = 0;
    for (old_floors) |fl| {
        if (fl.config_id != id) keep_count += 1;
    }
    if (keep_count < old_floors.len) {
        var next = try alloc.gpa.alloc(ProgressInfo.TowerFloor, keep_count);
        var idx: usize = 0;
        for (old_floors) |fl| {
            if (fl.config_id != id) {
                next[idx] = fl;
                idx += 1;
            }
        }
        alloc.gpa.free(old_floors);
        progress_comp.info.challenge.tower_floors = next;
        try progress_comp.save(fs, alloc.arena);
    }

    var reset_floors: std.ArrayList(pb.TowerFloorPb) = .empty;
    try reset_floors.append(alloc.arena, .{
        .TowerConfigId = id,
        .Star = 0,
        .IsQuickPass = false,
    });
    txn.conn.pushBestEffort(alloc.gpa, io, pb.TowerFloorUpdateNotify{
        .TowerFloors = reset_floors,
    });

    var diffs: std.ArrayList(pb.TowerDifficultyPb) = .empty;
    try diffs.append(alloc.arena, try buildDifficultyPb(alloc.arena, cfg.Difficulty, assets, progress_comp.info));
    txn.conn.pushBestEffort(alloc.gpa, io, pb.TowerDifficultyUpdateNotify{
        .TowerDifficulties = diffs,
    });

    txn.respond(.{
        .NeedUpdateSeason = false,
        .TowerConfigId = id,
        .ErrorCode = .Success,
    });
}

pub fn onTowerRewardRequest(
    txn: *Transaction(pb.TowerRewardRequest),
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    const diff = txn.message.Difficulty;
    const diff_cfg = assets.tables.tower_difficulty.getDataById(diff) orelse {
        txn.respond(.{ .ErrorCode = .ErrTowerRewardNotFound });
        return;
    };

    var indices: std.ArrayList(i32) = .empty;
    if (txn.message.RewardIndexList.items.len > 0) {
        try indices.appendSlice(alloc.arena, txn.message.RewardIndexList.items);
    } else if (txn.message.RewardIndex >= 0) {
        try indices.append(alloc.arena, txn.message.RewardIndex);
    }

    var stars: i32 = 0;
    for (progress_comp.info.challenge.tower_floors) |fl| {
        if (fl.difficulty == diff) stars += fl.star;
    }

    var claimed: std.ArrayList(i32) = .empty;
    var item_map: std.AutoHashMapUnmanaged(i32, i32) = .empty;
    var changed = false;

    for (indices.items) |ridx| {
        if (ridx < 0 or @as(usize, @intCast(ridx)) >= diff_cfg.Reward.len) continue;
        const entry = diff_cfg.Reward[@intCast(ridx)];
        if (stars < entry.Item1) continue;
        if (!(try progress_comp.info.claimTowerReward(alloc.gpa, diff, ridx))) continue;

        try CraftHelper.grantDropAndCollectItems(.{
            .conn = txn.conn,
            .alloc = alloc,
            .io = io,
            .fs = fs,
            .assets = assets,
            .basic_comp = basic_comp,
            .inventory_comp = inventory_comp,
            .weapon_comp = weapon_comp,
            .echo_comp = echo_comp,
        }, entry.Item2, 3, 50, 19500, &item_map);
        try claimed.append(alloc.arena, ridx);
        changed = true;
    }

    if (changed) {
        try progress_comp.save(fs, alloc.arena);
        var diffs: std.ArrayList(pb.TowerDifficultyPb) = .empty;
        try diffs.append(alloc.arena, try buildDifficultyPb(alloc.arena, diff, assets, progress_comp.info));
        txn.conn.pushBestEffort(alloc.gpa, io, pb.TowerDifficultyUpdateNotify{
            .TowerDifficulties = diffs,
        });
    }

    txn.respond(.{
        .NeedUpdateSeason = false,
        .ErrorCode = .Success,
        .RewardIndexList = claimed,
    });
}

pub fn onTowerApplyFloorDataRequest(
    txn: *Transaction(pb.TowerApplyFloorDataRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
) !void {
    if (txn.message.Apply) {
        try progress_comp.info.setKeyInt(alloc.gpa, "tower:apply_floor", 1);
        try progress_comp.save(fs, alloc.arena);
        const last_diff = progress_comp.info.getKeyInt("tower:last_diff");
        if (last_diff > 0) {
            var diffs: std.ArrayList(pb.TowerDifficultyPb) = .empty;
            try diffs.append(alloc.arena, try buildDifficultyPb(alloc.arena, @intCast(last_diff), assets, progress_comp.info));
            txn.conn.pushBestEffort(alloc.gpa, io, pb.TowerDifficultyUpdateNotify{
                .TowerDifficulties = diffs,
            });
        }
    }
    txn.respond(.{ .ErrorCode = .Success, .NeedUpdateSeason = false });
}

pub fn onTowerFormationRecommendRequest(
    txn: *Transaction(pb.TowerFormationRecommendRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
) !void {
    var recs: std.ArrayList(pb.TowerRecommendFormation) = .empty;
    var roles: std.ArrayList(pb.TowerRecommendRole) = .empty;
    for (assets.tables.role_info.items) |r| {
        if (r.Id >= 1000 and r.Id < 2000) {
            try roles.append(alloc.arena, .{ .RoleId = r.Id, .Level = 90 });
            if (roles.items.len >= 3) break;
        }
    }
    if (roles.items.len > 0) {
        try recs.append(alloc.arena, .{ .Formation = roles, .Usage = 100 });
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .TowerConfigId = txn.message.TowerConfigId,
        .Formations = recs,
        .NeedUpdateSeason = false,
    });
}

pub fn onTowerGuideActivityInfoRequest(
    txn: *Transaction(pb.TowerGuideActivityInfoRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    var ids: std.ArrayList(i32) = .empty;
    for (progress_comp.info.dynamic.claims) |k| {
        if (std.mem.startsWith(u8, k, "tower_guide:")) {
            const id = std.fmt.parseInt(i32, k["tower_guide:".len..], 10) catch continue;
            try ids.append(alloc.arena, id);
        }
    }
    txn.respond(.{ .TowerGuideId = ids });
}

pub fn onTowerGuideActivityRewardRequest(
    txn: *Transaction(pb.TowerGuideActivityRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "tower_guide:{d}", .{txn.message.TowerGuideId});
    if (try progress_comp.info.markClaim(alloc.gpa, k)) {
        try progress_comp.save(fs, alloc.arena);
        _ = try RewardService.grantItems(reward_ctx, 0, true, &.{
            .{ .item_id = 3, .count = 100 },
            .{ .item_id = 2, .count = 20000 },
        }, 19500);
    }
    txn.respond(.{ .ErrorCode = .Success });
}
