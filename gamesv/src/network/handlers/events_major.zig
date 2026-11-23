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
const RewardService = @import("../../logic/helpers/reward.zig");
const EnergyHelper = @import("../../logic/helpers/energy.zig");
const QuestEngine = @import("../../logic/helpers/quest_engine.zig");
const LevelPlayHelper = @import("../../logic/helpers/level_play.zig");
const pb_kv = @import("pb_kv.zig");

fn claimIdList(
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
    prefix: []const u8,
    table: []const u8,
    ids: []const i32,
) !void {
    const fallback = [_]RewardService.RewardItem{
        .{ .item_id = 3, .count = 40 },
        .{ .item_id = 2, .count = 15000 },
    };
    for (ids) |id| {
        const key = try std.fmt.allocPrint(alloc.arena, "{s}:{d}", .{ prefix, id });
        if (try progress_comp.info.markClaim(alloc.gpa, key)) {
            _ = try RewardService.grantTableReward(reward_ctx, table, id, false, true, 19500, &fallback, null);
        }
    }
    try progress_comp.save(fs, alloc.arena);
}

fn claimOne(
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
    prefix: []const u8,
    table: []const u8,
    id: i32,
) !void {
    const arr = [_]i32{id};
    try claimIdList(alloc, fs, progress_comp, reward_ctx, prefix, table, &arr);
}

// =========================================================================
// Dango Brawl & Dango Monopoly
// =========================================================================

pub fn onDangoBrawlBagOperateRequest(
    txn: *Transaction(pb.DangoBrawlBagOperateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try pb_kv.saveMsg(alloc, fs, progress_comp, "db:bag_ops", txn.message);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onDangoBrawlEventRefreshRequest(
    txn: *Transaction(pb.DangoBrawlEventRefreshRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "db_evt_ref:{d}", .{txn.message.SlotIndex});
    const cnt = try progress_comp.info.addKeyInt(alloc.gpa, key, 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .Slot = .{ .EventConfigId = 1, .RefreshedCount = @intCast(cnt) },
        .SlotIndex = txn.message.SlotIndex,
        .ErrorCode = .Success,
    });
}

pub fn onDangoBrawlEventSelectRequest(
    txn: *Transaction(pb.DangoBrawlEventSelectRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "db_evt_slot", txn.message.SlotIndex);
    try progress_comp.save(fs, alloc.arena);
    var active: std.ArrayList(i32) = .empty;
    try active.append(alloc.arena, txn.message.SlotIndex);
    txn.respond(.{ .ErrorCode = .Success, .ActiveEventIds = active });
}

pub fn onDangoBrawlStartFightRequest(
    txn: *Transaction(pb.DangoBrawlStartFightRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "db_fight_cnt", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onDangoBrawlFightOverRequest(
    txn: *Transaction(pb.DangoBrawlFightOverRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "db_round", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onDangoBrawlSettleRequest(
    txn: *Transaction(pb.DangoBrawlSettleRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "db_round", 0);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onDangoBrawlScoreRewardRequest(
    txn: *Transaction(pb.DangoBrawlScoreRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimIdList(alloc, fs, progress_comp, reward_ctx, "db_score", "DangoBrawlChallenge", txn.message.ScoreIds.items);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onDangoBrawlTaskRewardRequest(
    txn: *Transaction(pb.DangoBrawlTaskRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimIdList(alloc, fs, progress_comp, reward_ctx, "db_task", "DangoBrawlChallenge", txn.message.TaskIds.items);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onDangoBrawlShopBuyRequest(
    txn: *Transaction(pb.DangoBrawlShopBuyRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "db:buy:{d}", .{txn.message.SlotUid});
    _ = try progress_comp.info.addKeyInt(alloc.gpa, key, 1);
    try progress_comp.info.setKeyInt(alloc.gpa, "db:buy_bench", if (txn.message.ToBench) 1 else 0);
    if (txn.message.Slot) |slot| {
        const skey = try std.fmt.allocPrint(alloc.arena, "db:slot:{d}", .{txn.message.SlotUid});
        try pb_kv.saveMsg(alloc, fs, progress_comp, skey, slot);
    } else {
        try progress_comp.save(fs, alloc.arena);
    }
    txn.respond(.{ .ErrorCode = .Success, .UseServerBagUpdate = true });
}

pub fn onDangoBrawlShopComposeRequest(
    txn: *Transaction(pb.DangoBrawlShopComposeRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const uid = try progress_comp.info.addKeyInt(alloc.gpa, "db_uid", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success, .NewItemUid = @intCast(uid) });
}

pub fn onDangoBrawlShopLockRequest(
    txn: *Transaction(pb.DangoBrawlShopLockRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "db:lock:{d}", .{txn.message.SlotUid});
    try pb_kv.persistInt(alloc, fs, progress_comp, key, if (txn.message.Locked) 1 else 0);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onDangoBrawlShopRefreshRequest(
    txn: *Transaction(pb.DangoBrawlShopRefreshRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const cnt = try progress_comp.info.addKeyInt(alloc.gpa, "db_shop_ref", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .Shop = .{ .NextRefreshCost = 1, .ManualRefreshCount = @intCast(cnt) },
        .ErrorCode = .Success,
    });
}

pub fn onDangoBrawlShopSellRequest(
    txn: *Transaction(pb.DangoBrawlShopSellRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "db:sold:{d}", .{txn.message.ItemUid});
    try pb_kv.persistClaim(alloc, fs, progress_comp, key);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onDangoMonopolyDiceRequest(
    txn: *Transaction(pb.DangoMonopolyDiceRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    io: std.Io,
) !void {
    const now_ns: u64 = @intCast(@max(1, EnergyHelper.nowSeconds(io)));
    const roll: i32 = @intCast((now_ns % 6) + 1);
    const key = try std.fmt.allocPrint(alloc.arena, "dmono_grid:{d}", .{txn.message.ActivityId});
    _ = try progress_comp.info.addKeyInt(alloc.gpa, key, roll);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .DiceNum = roll,
        .MoveGridNum = roll,
        .ErrorCode = .Success,
    });
}

pub fn onDangoMonopolyGridRewardRequest(
    txn: *Transaction(pb.DangoMonopolyGridRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const cur_grid: i32 = @intCast(progress_comp.info.getKeyInt(try std.fmt.allocPrint(alloc.arena, "dmono_grid:{d}", .{txn.message.ActivityId})));
    try claimOne(alloc, fs, progress_comp, reward_ctx, "dmono_grid_rw", "Dango", cur_grid);
    txn.respond(.{ .ErrorCode = .Success, .HasReward = true });
}

pub fn onDangoMonopolyNextBoardRequest(
    txn: *Transaction(pb.DangoMonopolyNextBoardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "dmono_board:{d}", .{txn.message.ActivityId});
    const next_board = try progress_comp.info.addKeyInt(alloc.gpa, key, 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .ErrorCode = .Success,
        .BoardId = @intCast(next_board),
    });
}

pub fn onDangoMonopolyBoardRewardRequest(
    txn: *Transaction(pb.DangoMonopolyBoardRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimIdList(alloc, fs, progress_comp, reward_ctx, "dmono_board_rw", "Dango", txn.message.BoardIds.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onDangoMonopolyTaskRewardRequest(
    txn: *Transaction(pb.DangoMonopolyTaskRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimIdList(alloc, fs, progress_comp, reward_ctx, "dmono_task_rw", "Dango", txn.message.TaskIds.items);
    txn.respond(.{ .ErrorCode = .Success });
}

// =========================================================================
// Doll Grabber & Delivery
// =========================================================================

pub fn onDollDeliveryRequest(
    txn: *Transaction(pb.DollDeliveryRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    scene_opt: *?Scene,
    inventory_comp: *const PlayerInventoryComponent,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "doll:delivery:{d}", .{txn.message.EntityCfgId});
    try pb_kv.persistClaim(alloc, fs, progress_comp, key);

    var belong_type: ?[]const u8 = null;
    if (scene_opt.*) |*scene| {
        if (assets.findLevelEntity(scene.instance_id, txn.message.EntityCfgId)) |le| {
            for (le.ComponentsData.extra) |entry| {
                switch (entry) {
                    .DollGrabShowcaseComponent => |sc| {
                        if (sc.BelongType) |bt| {
                            if (bt.len > 0) belong_type = bt;
                        }
                    },
                    else => {},
                }
            }
        }
    }

    var items: std.ArrayList(i32) = .empty;
    var first_match: ?i32 = null;
    for (assets.tables.doll_grab_items_csv.items) |di| {
        if (di.BindingItemId <= 0) continue;
        if (belong_type) |bt| {
            if (!std.mem.eql(u8, di.ShowcaseBelongType, bt)) continue;
        }
        if (first_match == null) first_match = di.Id;
        if (inventory_comp.info.normalItemCount(di.BindingItemId) > 0) {
            const dk = try std.fmt.allocPrint(alloc.arena, "doll:item:{d}:{d}", .{ txn.message.EntityCfgId, di.Id });
            _ = try progress_comp.info.markClaim(alloc.gpa, dk);
            try items.append(alloc.arena, di.Id);
        }
    }
    if (items.items.len == 0) {
        if (first_match) |fid| {
            const dk = try std.fmt.allocPrint(alloc.arena, "doll:item:{d}:{d}", .{ txn.message.EntityCfgId, fid });
            _ = try progress_comp.info.markClaim(alloc.gpa, dk);
            try items.append(alloc.arena, fid);
        }
    }
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ItemIds = items, .ErrorCode = .Success });
}

pub fn onDollGarbInsertCoinRequest(
    txn: *Transaction(pb.DollGarbInsertCoinRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "doll:coins:{d}", .{txn.message.EntityIncId});
    _ = try progress_comp.info.addKeyInt(alloc.gpa, key, 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onDollGarbStartRequest(
    txn: *Transaction(pb.DollGarbStartRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "doll_grab:cur", 0);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onDollGrabPauseStartRequest(
    txn: *Transaction(pb.DollGrabPauseStartRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "doll:paused:{d}", .{txn.message.EntityIncId});
    try pb_kv.persistInt(alloc, fs, progress_comp, key, 1);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onDollGrabPauseEndRequest(
    txn: *Transaction(pb.DollGrabPauseEndRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "doll:paused:{d}", .{txn.message.EntityIncId});
    try pb_kv.persistInt(alloc, fs, progress_comp, key, 0);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onDollGrabRequest(
    txn: *Transaction(pb.DollGrabRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const sess = try progress_comp.info.addKeyInt(alloc.gpa, "doll_grab:cur", 10);
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "doll_grab:total", 10);
    try progress_comp.save(fs, alloc.arena);
    _ = sess;
    var items: std.ArrayList(pb.DollGrabItemInfo) = .empty;
    try items.append(alloc.arena, .{ .ItemId = txn.message.ItemUniId, .ItemCount = 1 });
    txn.respond(.{ .ErrorCode = .Success, .GrabItemInfos = items });
}

pub fn onDollGrabTimeBallRequest(
    txn: *Transaction(pb.DollGrabTimeBallRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "doll_grab:cur", 15);
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "doll_grab:total", 15);
    try progress_comp.save(fs, alloc.arena);
    var items: std.ArrayList(pb.DollGrabItemInfo) = .empty;
    try items.append(alloc.arena, .{ .ItemId = txn.message.ItemUniId, .ItemCount = 1 });
    txn.respond(.{ .ErrorCode = .Success, .GrabItemInfos = items });
}

pub fn onDollGrabStopRequest(
    txn: *Transaction(pb.DollGrabStopRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const sess: i32 = @intCast(progress_comp.info.getKeyInt("doll_grab:cur"));
    const total: i32 = @intCast(progress_comp.info.getKeyInt("doll_grab:total"));
    const high: i32 = @intCast(@max(sess, progress_comp.info.getKeyInt("doll_grab:high")));
    try progress_comp.info.setKeyInt(alloc.gpa, "doll_grab:high", high);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .HighScore = high,
        .AccumulatedScore = total,
        .SessionScore = sess,
        .ErrorCode = .Success,
    });
}

// =========================================================================
// Floro Ranch
// =========================================================================

pub fn onFloroRanchFirstEnterRequest(
    txn: *Transaction(pb.FloroRanchFirstEnterRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "fr_first_enter", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onFloroRanchAnimationRequest(
    txn: *Transaction(pb.FloroRanchAnimationRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "fr:anim:{d}", .{txn.message.ActivityId});
    try pb_kv.persistClaim(alloc, fs, progress_comp, key);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onFloroRanchStartPlayRequest(
    txn: *Transaction(pb.FloroRanchStartPlayRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "fr:day", 1);
    try progress_comp.info.setKeyInt(alloc.gpa, "fr:stage", 1);
    try progress_comp.info.setKeyInt(alloc.gpa, "fr:coin", 100);
    try progress_comp.info.setKeyInt(alloc.gpa, "fr:diamond", 10);
    try progress_comp.save(fs, alloc.arena);

    var races: std.ArrayList(i32) = .empty;
    try races.appendSlice(alloc.arena, txn.message.Races.items);
    txn.respond(.{
        .ErrorCode = .Success,
        .PlayInfo = .{
            .SubInsId = txn.message.SubInsId,
            .CurCoin = 100,
            .CurDiamond = 10,
            .SkillId = txn.message.SkillId,
            .Stage = 1,
            .Day = 1,
            .CurStageDayNum = 1,
            .TotalDay = 1,
            .RemainDay = 5,
            .Races = races,
        },
    });
}

fn floroSettleTasks(arena: std.mem.Allocator) !std.ArrayList(pb.FloroRanchPlayTask) {
    var tasks: std.ArrayList(pb.FloroRanchPlayTask) = .empty;
    try tasks.append(arena, .{
        .TaskType = .DaySettleActions,
        .UnitActions = .{ .UnitActions = .empty },
    });
    return tasks;
}

pub fn onFloroRanchPlayNextDayRequest(
    txn: *Transaction(pb.FloroRanchPlayNextDayRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const cur_day: i32 = @intCast(try progress_comp.info.addKeyInt(alloc.gpa, "fr:day", 1));
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "fr:coin", 50);
    try progress_comp.save(fs, alloc.arena);
    const cur_stage: i32 = @intCast(@max(1, progress_comp.info.getKeyInt("fr:stage")));
    const is_unlim = progress_comp.info.getKeyInt("fr:unlimited") > 0;
    var tasks: std.ArrayList(pb.FloroRanchPlayTask) = .empty;
    try tasks.append(alloc.arena, .{
        .TaskType = .DayStart,
        .DayStart = .{
            .CurDay = cur_day,
            .CurStage = cur_stage,
            .IsUnlimited = is_unlim,
            .TotalDay = cur_day,
            .RemainDay = if (is_unlim) 99 else @max(0, 6 - cur_day),
        },
    });
    txn.respond(.{ .ErrorCode = .Success, .Task = tasks });
}

pub fn onFloroRanchPlayTributeRequest(
    txn: *Transaction(pb.FloroRanchPlayTributeRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const stage: i32 = @intCast(try progress_comp.info.addKeyInt(alloc.gpa, "fr:stage", 1));
    try progress_comp.save(fs, alloc.arena);
    var tasks: std.ArrayList(pb.FloroRanchPlayTask) = .empty;
    try tasks.append(alloc.arena, .{
        .TaskType = .StageStart,
        .StageStart = .{
            .Target = 100 * @as(i64, stage),
            .DayCount = 5,
            .Reward = 50,
        },
    });
    txn.respond(.{
        .Settle = false,
        .Task = tasks,
        .CurCoin = progress_comp.info.getKeyInt("fr:coin"),
        .CurDiamond = progress_comp.info.getKeyInt("fr:diamond"),
        .RemainDay = 5,
        .CurStage = stage,
        .ErrorCode = .Success,
    });
}

pub fn onFloroRanchPlayUnlimitedModeRequest(
    txn: *Transaction(pb.FloroRanchPlayUnlimitedModeRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "fr:unlimited", 1);
    try progress_comp.save(fs, alloc.arena);
    const cur_day: i32 = @intCast(@max(1, progress_comp.info.getKeyInt("fr:day")));
    const cur_stage: i32 = @intCast(@max(1, progress_comp.info.getKeyInt("fr:stage")));
    var tasks: std.ArrayList(pb.FloroRanchPlayTask) = .empty;
    try tasks.append(alloc.arena, .{
        .TaskType = .DayStart,
        .DayStart = .{
            .CurDay = cur_day,
            .CurStage = cur_stage,
            .IsUnlimited = true,
            .TotalDay = cur_day,
            .RemainDay = 99,
        },
    });
    txn.respond(.{
        .Task = tasks,
        .ErrorCode = .Success,
        .CurStage = cur_stage,
    });
}

pub fn onFloroRanchPlayGachaRequest(
    txn: *Transaction(pb.FloroRanchPlayGachaRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const unit_inc = try progress_comp.info.addKeyInt(alloc.gpa, "fr:unit_inc", 1);
    try progress_comp.save(fs, alloc.arena);
    const unit: pb.FloroRanchPlayUnit = .{
        .PlayIncId = @intCast(unit_inc),
        .Id = txn.message.CardId,
        .StartStage = 1,
        .StartDay = 1,
    };
    var res: std.ArrayList(pb.FloroRanchPlayUnit) = .empty;
    try res.append(alloc.arena, unit);
    txn.respond(.{
        .ErrorCode = .Success,
        .GachaResult = res,
        .Unit = unit,
    });
}

pub fn onFloroRanchPlayGachaRefreshRequest(
    txn: *Transaction(pb.FloroRanchPlayGachaRefreshRequest),
    progress_comp: *const PlayerProgressComponent,
) !void {
    txn.respond(.{
        .CurDiamond = progress_comp.info.getKeyInt("fr:diamond"),
        .CurCoin = progress_comp.info.getKeyInt("fr:coin"),
        .ErrorCode = .Success,
        .Task = .{ .AllowedRefresh = true, .IncId = txn.message.IncId, .FreeTimes = 1 },
    });
}

pub fn onFloroRanchPlayRefreshShopRequest(
    txn: *Transaction(pb.FloroRanchPlayRefreshShopRequest),
    progress_comp: *const PlayerProgressComponent,
) !void {
    txn.respond(.{
        .Shop = .{ .AllowedRefresh = true, .IncId = txn.message.IncId, .FreeTimes = 1 },
        .CurDiamond = progress_comp.info.getKeyInt("fr:diamond"),
        .CurCoin = progress_comp.info.getKeyInt("fr:coin"),
        .ErrorCode = .Success,
    });
}

pub fn onFloroRanchPlayShopBuyRequest(
    txn: *Transaction(pb.FloroRanchPlayShopBuyRequest),
    progress_comp: *const PlayerProgressComponent,
) !void {
    txn.respond(.{
        .Shop = .{ .AllowedRefresh = true, .IncId = txn.message.IncId, .FreeTimes = 1 },
        .CurCoin = progress_comp.info.getKeyInt("fr:coin"),
        .CurDiamond = progress_comp.info.getKeyInt("fr:diamond"),
        .ErrorCode = .Success,
    });
}

pub fn onFloroRanchPlaySelectCardGroupRequest(
    txn: *Transaction(pb.FloroRanchPlaySelectCardGroupRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try pb_kv.saveMsg(alloc, fs, progress_comp, "fr:card_group", txn.message);
    var units: std.ArrayList(pb.FloroRanchPlayUnit) = .empty;
    for (txn.message.CardId.items, 1..) |cid, idx| {
        try units.append(alloc.arena, .{ .Id = cid, .PlayIncId = @intCast(idx) });
    }
    txn.respond(.{ .NewPhantoms = units, .ErrorCode = .Success });
}

pub fn onFloroRanchPlayRemoveUnitRequest(
    txn: *Transaction(pb.FloroRanchPlayRemoveUnitRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const coin = try progress_comp.info.addKeyInt(alloc.gpa, "fr:coin", 5);
    const k = try std.fmt.allocPrint(alloc.arena, "fr:rm_unit:{d}:{d}", .{ txn.message.SubInsId, txn.message.UnitIncId });
    try pb_kv.persistClaim(alloc, fs, progress_comp, k);
    txn.respond(.{
        .Task = try floroSettleTasks(alloc.arena),
        .ErrorCode = .Success,
        .CurDiamond = progress_comp.info.getKeyInt("fr:diamond"),
        .CurCoin = coin,
    });
}

pub fn onFloroRanchCloseTaskRequest(
    txn: *Transaction(pb.FloroRanchCloseTaskRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "fr:close_task:{d}:{d}", .{ txn.message.SubInsId, txn.message.TaskIncId });
    try pb_kv.persistClaim(alloc, fs, progress_comp, key);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onFloroRanchEventChoiceRequest(
    txn: *Transaction(pb.FloroRanchEventChoiceRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "fr:evt_choice:{d}:{d}", .{ txn.message.SubInsId, txn.message.IncId });
    try pb_kv.persistInt(alloc, fs, progress_comp, key, txn.message.Choice);
    txn.respond(.{
        .ErrorCode = .Success,
        .Task = try floroSettleTasks(alloc.arena),
    });
}

pub fn onFloroRanchExecuteSkillPlayRequest(
    txn: *Transaction(pb.FloroRanchExecuteSkillPlayRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "fr:skill:{d}:{d}", .{ txn.message.SubInsId, txn.message.IncId });
    _ = try progress_comp.info.addKeyInt(alloc.gpa, key, 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .ErrorCode = .Success,
        .Task = try floroSettleTasks(alloc.arena),
    });
}

pub fn onFloroRanchOutRequest(
    txn: *Transaction(pb.FloroRanchOutRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try pb_kv.persistInt(alloc, fs, progress_comp, "fr:cur_sub_ins", 0);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onFloroRanchUnlockTechPointRequest(
    txn: *Transaction(pb.FloroRanchUnlockTechPointRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "fr:tech:{d}", .{txn.message.TechPoint});
    _ = try progress_comp.info.markClaim(alloc.gpa, key);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onFloroRanchRewardMilestoneRequest(
    txn: *Transaction(pb.FloroRanchRewardMilestoneRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimIdList(alloc, fs, progress_comp, reward_ctx, "fr_ms", "FloroRanchReward", txn.message.MilestoneIds.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onFloroRanchRewardTaskRequest(
    txn: *Transaction(pb.FloroRanchRewardTaskRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimIdList(alloc, fs, progress_comp, reward_ctx, "fr_task", "FloroRanchReward", txn.message.TaskIds.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onFloroRanchSettleDataRequest(
    txn: *Transaction(pb.FloroRanchSettleDataRequest),
    progress_comp: *const PlayerProgressComponent,
) !void {
    const coin = progress_comp.info.getKeyInt("fr:coin");
    const days: i32 = @intCast(@max(1, progress_comp.info.getKeyInt("fr:day")));
    txn.respond(.{
        .SettleData = .{ .TotalDays = days, .TotalCoin = coin, .Win = true },
        .ErrorCode = .Success,
        .ShowMvp = true,
    });
}

pub fn onFloroRanchSettleRequest(
    txn: *Transaction(pb.FloroRanchSettleRequest),
    progress_comp: *const PlayerProgressComponent,
) !void {
    const coin = progress_comp.info.getKeyInt("fr:coin");
    const days: i32 = @intCast(@max(1, progress_comp.info.getKeyInt("fr:day")));
    txn.respond(.{
        .TotalCoin = coin,
        .MaxDayCoin = coin,
        .TotalDays = days,
        .Win = true,
        .ErrorCode = .Success,
        .SettleData = .{ .TotalDays = days, .TotalCoin = coin, .Win = true },
    });
}

// =========================================================================
// Honami Story
// =========================================================================

pub fn onHonamiStoryActivityInfoRequest(
    txn: *Transaction(pb.HonamiStoryActivityInfoRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    const ls_lv: i32 = @intCast(@max(1, progress_comp.info.getKeyInt("honami:ls_lv")));
    var activated_talents: std.ArrayList(i32) = .empty;
    var talent_infos: std.ArrayList(pb.HonamiStoryTalentInfo) = .empty;
    var unlocked_weapons: std.ArrayList(i32) = .empty;
    var role_equip_list: std.ArrayList(pb.HonamiStoryRoleData) = .empty;

    for (progress_comp.info.dynamic.claims) |c| {
        if (std.mem.startsWith(u8, c, "honami:talent:")) {
            if (std.fmt.parseInt(i32, c["honami:talent:".len..], 10)) |tid| {
                try activated_talents.append(alloc.arena, tid);
                try talent_infos.append(alloc.arena, .{ .TalentId = tid, .Status = 2 });
            } else |_| {}
        }
    }

    var kbuf: [48]u8 = undefined;
    var rack_idx: i32 = 0;
    while (rack_idx < 3) : (rack_idx += 1) {
        const wk = try std.fmt.bufPrint(&kbuf, "honami:weapon_dress:{d}:{d}", .{ txn.message.ActivityId, rack_idx });
        const dressed_weapon: i32 = @intCast(@max(0, progress_comp.info.getKeyInt(wk)));
        if (dressed_weapon > 0) {
            try unlocked_weapons.append(alloc.arena, dressed_weapon);
        }
        try role_equip_list.append(alloc.arena, .{
            .RoleId = @intCast(rack_idx + 1),
            .DressWeapon = dressed_weapon,
        });
    }

    txn.respond(.{
        .HonamiStoryActivityInfo = .{
            .ActivatedTalentId = activated_talents,
            .HonamiStoryTalentInfos = talent_infos,
            .LifeSupportLevel = ls_lv,
            .HonamiStoryPlayerBagInfo = .{
                .Warehouse = .{
                    .BackpackId = 1,
                    .Width = 7,
                    .Capacity = 385,
                },
                .EquipRack = .{
                    .BackpackId = 2,
                    .Width = 7,
                    .Capacity = 63,
                },
                .RoleEquipList = role_equip_list,
                .UnlockedWeaponIds = unlocked_weapons,
            },
        },
        .ErrorCode = .Success,
    });
}

pub fn onHonamiStoryActivateTalentRequest(
    txn: *Transaction(pb.HonamiStoryActivateTalentRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "honami:talent:{d}", .{txn.message.TalentId});
    _ = try progress_comp.info.markClaim(alloc.gpa, key);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success, .TalentId = txn.message.TalentId });
}

pub fn onHonamiStoryAreaSecretRewardRequest(
    txn: *Transaction(pb.HonamiStoryAreaSecretRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimOne(alloc, fs, progress_comp, reward_ctx, "honami_sec", "HonamiStoryArea", txn.message.AreaSecretId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onHonamiStoryBagOperateRequest(
    txn: *Transaction(pb.HonamiStoryBagOperateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try pb_kv.saveMsg(alloc, fs, progress_comp, "honami:bag_op", txn.message);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onHonamiStoryDiscardItemRequest(
    txn: *Transaction(pb.HonamiStoryDiscardItemRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try pb_kv.persistOptMsg(alloc, fs, progress_comp, "honami:discard", txn.message.HonamiStoryBagUpdateContext);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onHonamiStoryPickUpItemRequest(
    txn: *Transaction(pb.HonamiStoryPickUpItemRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try pb_kv.persistOptMsg(alloc, fs, progress_comp, "honami:pickup", txn.message.HonamiStoryBagUpdateInfo);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onHonamiStoryLockItemRequest(
    txn: *Transaction(pb.HonamiStoryLockItemRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "honami:lock:{d}:{d}", .{ txn.message.BackPackId, txn.message.ItemUid });
    try pb_kv.persistInt(alloc, fs, progress_comp, key, if (txn.message.IsLock) 1 else 0);
    txn.respond(.{
        .ItemUid = txn.message.ItemUid,
        .IsLock = txn.message.IsLock,
        .BackPackId = txn.message.BackPackId,
        .ErrorCode = .Success,
    });
}

pub fn onHonamiStorySellItemRequest(
    txn: *Transaction(pb.HonamiStorySellItemRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "honami:sell:{d}", .{txn.message.ActivityId});
    try pb_kv.saveMsg(alloc, fs, progress_comp, key, txn.message);
    txn.respond(.{
        .ActivityId = txn.message.ActivityId,
        .HonamiStoryItemUidInfos = txn.message.HonamiStoryItemUidInfos,
        .ErrorCode = .Success,
    });
}

pub fn onHonamiStoryEquipRoleRequest(
    txn: *Transaction(pb.HonamiStoryEquipRoleRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "honami:equip:{d}", .{txn.message.ActivityId});
    try pb_kv.saveMsg(alloc, fs, progress_comp, key, txn.message);
    var out: std.ArrayList(i32) = .empty;
    try out.appendSlice(alloc.arena, txn.message.RoleId.items);
    txn.respond(.{
        .RoleId = out,
        .ActivityId = txn.message.ActivityId,
        .ErrorCode = .Success,
    });
}

pub fn onHonamiStoryInstLeaveRequest(
    txn: *Transaction(pb.HonamiStoryInstLeaveRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    try pb_kv.persistInt(alloc, fs, progress_comp, "honami:in_inst", 0);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onHonamiStoryItemCollectionRequest(
    txn: *Transaction(pb.HonamiStoryItemCollectionRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimIdList(alloc, fs, progress_comp, reward_ctx, "honami_col", "HonamiStoryItemCollection", txn.message.HonamiStoryItemCollectionId.items);
    var out: std.ArrayList(i32) = .empty;
    try out.appendSlice(alloc.arena, txn.message.HonamiStoryItemCollectionId.items);
    txn.respond(.{
        .ActivityId = txn.message.ActivityId,
        .HonamiStoryItemCollectionId = out,
        .ErrorCode = .Success,
    });
}

pub fn onHonamiStoryLifeSupportUpRequest(
    txn: *Transaction(pb.HonamiStoryLifeSupportUpRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const lv = try progress_comp.info.addKeyInt(alloc.gpa, "honami:ls_lv", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .ActivityId = txn.message.ActivityId,
        .LifeSupportLevel = @intCast(lv),
        .ErrorCode = .Success,
    });
}

pub fn onHonamiStoryLimitTaskRewardRequest(
    txn: *Transaction(pb.HonamiStoryLimitTaskRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimIdList(alloc, fs, progress_comp, reward_ctx, "honami_lt", "HonamiStoryLimitTask", txn.message.LimitTaskIds.items);
    if (txn.message.LimitTaskId != 0) try claimOne(alloc, fs, progress_comp, reward_ctx, "honami_lt", "HonamiStoryLimitTask", txn.message.LimitTaskId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onHonamiStoryResidentTaskRewardRequest(
    txn: *Transaction(pb.HonamiStoryResidentTaskRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimIdList(alloc, fs, progress_comp, reward_ctx, "honami_rt", "HonamiStoryResidentTask", txn.message.ResidentTaskIds.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onHonamiStoryMascotRewardRequest(
    txn: *Transaction(pb.HonamiStoryMascotRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimOne(alloc, fs, progress_comp, reward_ctx, "honami_mascot", "HonamiStoryMascot", txn.message.HonamiStoryMascotId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onHonamiStoryScoreRewardRequest(
    txn: *Transaction(pb.HonamiStoryScoreRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimIdList(alloc, fs, progress_comp, reward_ctx, "honami_score", "HonamiStoryScoreReward", txn.message.ScoreRewardId.items);
    var out: std.ArrayList(i32) = .empty;
    try out.appendSlice(alloc.arena, txn.message.ScoreRewardId.items);
    txn.respond(.{
        .ScoreRewardId = out,
        .ActivityId = txn.message.ActivityId,
        .ErrorCode = .Success,
    });
}

pub fn onHonamiStoryUnlockSlotRequest(
    txn: *Transaction(pb.HonamiStoryUnlockSlotRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "honami:slot:{d}:{d}", .{ txn.message.RackIndex, txn.message.SlotId });
    _ = try progress_comp.info.markClaim(alloc.gpa, key);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .ErrorCode = .Success,
        .RackIndex = txn.message.RackIndex,
        .SlotId = txn.message.SlotId,
    });
}

pub fn onHonamiStoryWeaponDressRequest(
    txn: *Transaction(pb.HonamiStoryWeaponDressRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "honami:weapon_dress:{d}:{d}", .{ txn.message.ActivityId, txn.message.RackIndex });
    try pb_kv.persistInt(alloc, fs, progress_comp, key, txn.message.WeaponItemId);
    txn.respond(.{
        .ErrorCode = .Success,
        .RackIndex = txn.message.RackIndex,
        .WeaponItemId = txn.message.WeaponItemId,
    });
}

// =========================================================================
// JokerGuess, Kurotato & Pinball
// =========================================================================

pub fn onJokerGuessStartNewRequest(
    txn: *Transaction(pb.JokerGuessStartNewRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try pb_kv.persistInt(alloc, fs, progress_comp, "joker:cur_level", txn.message.LevelId);
    var cmd: std.ArrayList(pb.JokerGuessOp) = .empty;
    try cmd.append(alloc.arena, .{
        .JokerGuessLevelId = txn.message.LevelId,
        .OpType = .EnterNewRound,
        .NewRoundAction = .{ .RoundNum = 1, .ChangeResult = 0 },
    });
    txn.respond(.{ .ErrorCode = .Success, .InitData = .{ .FirstActor = .Who }, .Cmd = cmd });
}

pub fn onJokerGuessDrawCardRequest(
    txn: *Transaction(pb.JokerGuessDrawCardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try pb_kv.persistInt(alloc, fs, progress_comp, "joker:drawn_card", txn.message.TargetCardId);
    const lv: i32 = @intCast(@max(0, progress_comp.info.getKeyInt("joker:cur_level")));
    var cmd: std.ArrayList(pb.JokerGuessOp) = .empty;
    try cmd.append(alloc.arena, .{
        .JokerGuessLevelId = lv,
        .OpType = .DrawCard,
        .DrawAction = .{ .Drawer = .Who, .DrawCard = txn.message.TargetCardId },
    });
    txn.respond(.{ .DrawCardResult = txn.message.TargetCardId, .ErrorCode = .Success, .Cmd = cmd });
}

pub fn onJokerGuessPlayCardRequest(
    txn: *Transaction(pb.JokerGuessPlayCardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try pb_kv.persistOptMsg(alloc, fs, progress_comp, "joker:last_play", txn.message.PlayCard);
    const lv: i32 = @intCast(@max(0, progress_comp.info.getKeyInt("joker:cur_level")));
    var cmd: std.ArrayList(pb.JokerGuessOp) = .empty;
    try cmd.append(alloc.arena, .{
        .JokerGuessLevelId = lv,
        .OpType = .PlayCard,
        .PlayAction = .{ .PlayActor = .Who, .PlayCard = txn.message.PlayCard },
    });
    txn.respond(.{ .ErrorCode = .Success, .Cmd = cmd });
}

pub fn onJokerGuessUseSkillRequest(
    txn: *Transaction(pb.JokerGuessUseSkillRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try pb_kv.persistInt(alloc, fs, progress_comp, "joker:last_skill", txn.message.SkillId);
    const lv: i32 = @intCast(@max(0, progress_comp.info.getKeyInt("joker:cur_level")));
    var cmd: std.ArrayList(pb.JokerGuessOp) = .empty;
    try cmd.append(alloc.arena, .{
        .JokerGuessLevelId = lv,
        .OpType = .ExecSkill,
        .SkillExecAction = .{
            .JokerSkillId = txn.message.SkillId,
            .Owner = .Who,
            .Result = txn.message.Op orelse .Use,
        },
    });
    txn.respond(.{ .ErrorCode = .Success, .Cmd = cmd });
}

pub fn onJokerGuessRewardRequest(
    txn: *Transaction(pb.JokerGuessRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimOne(alloc, fs, progress_comp, reward_ctx, "joker_rw", "JokerGuessLevel", txn.message.LevelId);
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "guess_joker:done:{d}", .{txn.message.LevelId});
    _ = try progress_comp.info.markClaim(alloc.gpa, k);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onKurotatoSyncEntityDealDamagePush(
    txn: *Transaction(pb.KurotatoSyncEntityDealDamagePush),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var dirty = false;
    for (txn.message.EntityDealDamages.items) |entry| {
        const key = try std.fmt.allocPrint(alloc.arena, "kurotato:dmg:{d}", .{entry.key});
        _ = try progress_comp.info.addKeyInt(alloc.gpa, key, entry.value);
        _ = try progress_comp.info.addKeyInt(alloc.gpa, "kurotato:dmg_total", entry.value);
        dirty = true;
    }
    if (dirty) try progress_comp.save(fs, alloc.arena);
}

pub fn onKurotatoChestRewardSelectRequest(
    txn: *Transaction(pb.KurotatoChestRewardSelectRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try pb_kv.persistInt(alloc, fs, progress_comp, "kurotato:chest_sold", if (txn.message.WillSold) 1 else 0);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onKurotatoEndShopStepRequest(
    txn: *Transaction(pb.KurotatoEndShopStepRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try pb_kv.persistInt(alloc, fs, progress_comp, "kurotato:endless", if (txn.message.IsEndLess) 1 else 0);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onKurotatoEndSpecialWaveCombatRequest(
    txn: *Transaction(pb.KurotatoEndSpecialWaveCombatRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    try pb_kv.persistClaim(alloc, fs, progress_comp, "kurotato:spec_wave");
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onKurotatoGoldAddRequest(
    txn: *Transaction(pb.KurotatoGoldAddRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "kurotato:gold", txn.message.GoldCount);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onKurotatoLevelRecordDeleteRequest(
    txn: *Transaction(pb.KurotatoLevelRecordDeleteRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "kurotato:lvl:{d}", .{txn.message.LevelId});
    try progress_comp.info.setKeyInt(alloc.gpa, key, 0);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onKurotatoLimitTaskReceiveRequest(
    txn: *Transaction(pb.KurotatoLimitTaskReceiveRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimIdList(alloc, fs, progress_comp, reward_ctx, "kurotato_lt", "KurotatoLimitAward", txn.message.TaskIds.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onKurotatoResTaskReceiveRequest(
    txn: *Transaction(pb.KurotatoResTaskReceiveRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimIdList(alloc, fs, progress_comp, reward_ctx, "kurotato_rt", "KurotatoResAward", txn.message.TaskIds.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onKurotatoScoreTaskReceiveRequest(
    txn: *Transaction(pb.KurotatoScoreTaskReceiveRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimIdList(alloc, fs, progress_comp, reward_ctx, "kurotato_st", "KurotatoScoreAward", txn.message.TaskIds.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onKurotatoReChallengeRequest(
    txn: *Transaction(pb.KurotatoReChallengeRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try pb_kv.persistInt(alloc, fs, progress_comp, "kurotato:rechallenge_cur", if (txn.message.IsFromCurWave) 1 else 0);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onKurotatoRefineWeaponRequest(
    txn: *Transaction(pb.KurotatoRefineWeaponRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "kurotato:refine_wp:{d}", .{txn.message.WeaponIncId});
    _ = try progress_comp.info.addKeyInt(alloc.gpa, key, 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success, .PanelPbData = .{} });
}

pub fn onKurotatoSellWeaponRequest(
    txn: *Transaction(pb.KurotatoSellWeaponRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "kurotato:sell_wp:{d}", .{txn.message.WeaponIncId});
    _ = try progress_comp.info.markClaim(alloc.gpa, key);
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "kurotato:gold", 10);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success, .PanelPbData = .{} });
}

pub fn onKurotatoRoleRecordRequest(
    txn: *Transaction(pb.KurotatoRoleRecordRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    try pb_kv.persistClaim(alloc, fs, progress_comp, "kurotato:role_rec");
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onKurotatoSettlementRequest(
    txn: *Transaction(pb.KurotatoSettlementRequest),
    progress_comp: *const PlayerProgressComponent,
) !void {
    txn.respond(.{
        .ErrorCode = .Success,
        .KurotatoSettlementInfo = .{
            .IsPass = true,
            .CumulativeKills = @intCast(progress_comp.info.getKeyInt("simple_combat:kills")),
        },
    });
}

pub fn onKurotatoShopRewardLockRequest(
    txn: *Transaction(pb.KurotatoShopRewardLockRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "kurotato:shop_lock:{d}", .{txn.message.SelectionId});
    const prev = progress_comp.info.getKeyInt(key);
    try pb_kv.persistInt(alloc, fs, progress_comp, key, if (prev == 0) 1 else 0);
    txn.respond(.{ .ErrorCode = .Success, .PanelPbData = .{} });
}

pub fn onKurotatoShopRewardRefreshRequest(
    txn: *Transaction(pb.KurotatoShopRewardRefreshRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    const cnt = try progress_comp.info.addKeyInt(alloc.gpa, "kurotato:shop_ref", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success, .PanelPbData = .{ .RefreshCost = @intCast(cnt + 1) } });
}

pub fn onKurotatoShopRewardSelectRequest(
    txn: *Transaction(pb.KurotatoShopRewardSelectRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "kurotato:shop_sel:{d}", .{txn.message.SelectionId});
    try pb_kv.persistClaim(alloc, fs, progress_comp, key);
    txn.respond(.{ .ErrorCode = .Success, .PanelPbData = .{} });
}

pub fn onKurotatoStepAdvanceRequest(
    txn: *Transaction(pb.KurotatoStepAdvanceRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    _ = txn.message;
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "kurotato:step", 1);
    try progress_comp.save(fs, alloc.arena);
    if (LevelPlayHelper.fromQuestContext(qctx)) |lp_ctx| {
        LevelPlayHelper.onConditionEvent(lp_ctx, progress_comp, .finish_kurotato, 0, 0) catch {};
    }
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onKurotatoUpgradeRewardRefreshRequest(
    txn: *Transaction(pb.KurotatoUpgradeRewardRefreshRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "kurotato:upg_ref", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success, .PanelPbData = .{} });
}

pub fn onKurotatoUpgradeRewardSelectRequest(
    txn: *Transaction(pb.KurotatoUpgradeRewardSelectRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "kurotato:upg_sel:{d}", .{txn.message.SelectionId});
    try pb_kv.persistClaim(alloc, fs, progress_comp, key);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onPinballBonusFullWinRequest(
    txn: *Transaction(pb.PinballBonusFullWinRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    try pb_kv.persistClaim(alloc, fs, progress_comp, "pinball:full_win");
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onPinballFriendRankRequest(
    txn: *Transaction(pb.PinballFriendRankRequest),
    basic_comp: *const PlayerBasicComponent,
) !void {
    txn.respond(.{
        .ErrorCode = .Success,
        .PlayerSelf = .{ .Name = basic_comp.info.attributes.name, .TowerLevel = 1 },
    });
}

pub fn onPinballInGameKickBallRequest(
    txn: *Transaction(pb.PinballInGameKickBallRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "pinball:kicks", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onPinballInGameSyncPayloadRequest(
    txn: *Transaction(pb.PinballInGameSyncPayloadRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try pb_kv.persistOptMsg(alloc, fs, progress_comp, "pinball:payload", txn.message.Payload);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onPinballQuitRequest(
    txn: *Transaction(pb.PinballQuitRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try pb_kv.persistOptMsg(alloc, fs, progress_comp, "pinball:last_quit", txn.message.Payload);
    try pb_kv.persistInt(alloc, fs, progress_comp, "pinball:in_game", 0);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onPinballRoleLevelUpRequest(
    txn: *Transaction(pb.PinballRoleLevelUpRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "pinball:role_lv:{d}", .{txn.message.RoleId});
    _ = try progress_comp.info.addKeyInt(alloc.gpa, key, txn.message.AddLv);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onPinballSetPayloadRequest(
    txn: *Transaction(pb.PinballSetPayloadRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try pb_kv.persistOptMsg(alloc, fs, progress_comp, "pinball:payload", txn.message.Payload);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onPinballTaskRewardRequest(
    txn: *Transaction(pb.PinballTaskRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimIdList(alloc, fs, progress_comp, reward_ctx, "pinball_task", "PinballTask", txn.message.TaskIds.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onPinballTeamDieRequest(
    txn: *Transaction(pb.PinballTeamDieRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "pinball:deaths", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onPinballTeamReviveRequest(
    txn: *Transaction(pb.PinballTeamReviveRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "pinball:revives", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onPinballWeaponDegradeRequest(
    txn: *Transaction(pb.PinballWeaponDegradeRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var dirty = false;
    for (txn.message.WeaponIncIds.items) |wid| {
        const key = try std.fmt.allocPrint(alloc.arena, "pinball:wp_deg:{d}", .{wid});
        _ = try progress_comp.info.addKeyInt(alloc.gpa, key, 1);
        dirty = true;
    }
    if (dirty) try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onPinballWeaponWearRequest(
    txn: *Transaction(pb.PinballWeaponWearRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "pinball:role_wp:{d}", .{txn.message.RoleId});
    try progress_comp.info.setKeyInt(alloc.gpa, key, txn.message.WeaponInc);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

// =========================================================================
// RacingBets, Regress, Rhythm & RollBlock
// =========================================================================

pub fn onRacingBetMatchInfoRequest(
    txn: *Transaction(pb.RacingBetMatchInfoRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    var list: std.ArrayList(pb.RacingBetsInstanceDangoInfo) = .empty;
    var did: i32 = 1;
    while (did <= 6) : (did += 1) {
        const key = try std.fmt.allocPrint(alloc.arena, "rb:bet:{d}:{d}", .{ txn.message.LegMatchId, did });
        const pt: i32 = @intCast(progress_comp.info.getKeyInt(key));
        try list.append(alloc.arena, .{ .DangoId = did, .CurPoint = pt });
    }
    txn.respond(.{ .DangoInfo = list, .ErrorCode = .Success });
}

pub fn onRacingBetMatchActionRequest(txn: *Transaction(pb.RacingBetMatchActionRequest)) !void {
    txn.respond(.{
        .ActivityId = txn.message.ActivityId,
        .LegMatchId = txn.message.LegMatchId,
        .ErrorCode = .Success,
    });
}

pub fn onRacingBetMatchRoundActionRequest(txn: *Transaction(pb.RacingBetMatchRoundActionRequest)) !void {
    txn.respond(.{
        .ErrorCode = .Success,
        .Rounds = .{ .RoundRank = txn.message.Order, .IsFinalRound = txn.message.Order >= 3 },
        .IsResultRound = txn.message.Order >= 3,
    });
}

pub fn onRacingBetsBulletScreenRequest(
    txn: *Transaction(pb.RacingBetsBulletScreenRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "rb:bullet:{d}:{d}", .{ txn.message.LegMatchId, txn.message.BulletScreenId });
    _ = try progress_comp.info.addKeyInt(alloc.gpa, key, 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRacingBetsCloseSettleMenuRequest(
    txn: *Transaction(pb.RacingBetsCloseSettleMenuRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "rb:close_settle:{d}:{d}", .{ txn.message.ActivityId, txn.message.LegMatchId });
    try pb_kv.persistClaim(alloc, fs, progress_comp, key);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRacingBetsGearRequest(
    txn: *Transaction(pb.RacingBetsGearRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const cash: i32 = @intCast(@max(1000, progress_comp.info.getKeyInt("rb:cash")));
    const key = try std.fmt.allocPrint(alloc.arena, "rb:bet:{d}:{d}", .{ txn.message.LegMatchId, txn.message.DangoId });
    try progress_comp.info.setKeyInt(alloc.gpa, key, txn.message.Cash);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .ErrorCode = .Success,
        .PlayerInfo = .{ .CurCash = cash, .TotalCash = cash },
    });
}

pub fn onRacingBetsGearRefundRequest(
    txn: *Transaction(pb.RacingBetsGearRefundRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "rb:bet:{d}:{d}", .{ txn.message.LegMatchId, txn.message.DangoId });
    try progress_comp.info.setKeyInt(alloc.gpa, key, 0);
    try progress_comp.save(fs, alloc.arena);
    const cash: i32 = @intCast(@max(1000, progress_comp.info.getKeyInt("rb:cash")));
    txn.respond(.{
        .ErrorCode = .Success,
        .PlayerInfo = .{ .CurCash = cash, .TotalCash = cash },
    });
}

pub fn onRacingBetsMatchInfoRequest(
    txn: *Transaction(pb.RacingBetsMatchInfoRequest),
    alloc: mem.Alloc,
) !void {
    var dangos: std.ArrayList(i32) = .empty;
    var did: i32 = 1;
    while (did <= 6) : (did += 1) {
        try dangos.append(alloc.arena, did);
    }
    var groups: std.ArrayList(pb.RacingBetsGroupMatchesInfo) = .empty;
    try groups.append(alloc.arena, .{ .GroupMatchId = 1, .Dangos = dangos });
    txn.respond(.{
        .MatchInfos = groups,
        .ErrorCode = .Success,
        .ActivityId = txn.message.ActivityId,
    });
}

pub fn onRacingBetsRankRequest(
    txn: *Transaction(pb.RacingBetsRankRequest),
    basic_comp: *const PlayerBasicComponent,
    progress_comp: *const PlayerProgressComponent,
) !void {
    const cash: i32 = @intCast(@max(1000, progress_comp.info.getKeyInt("rb:cash")));
    txn.respond(.{
        .ErrorCode = .Success,
        .Status = .Rank,
        .SelfData = .{
            .HeadIcon = basic_comp.info.attributes.head_photo,
            .Name = basic_comp.info.attributes.name,
            .Cash = cash,
        },
    });
}

pub fn onRacingBetsUpdateOddsRequest(
    txn: *Transaction(pb.RacingBetsUpdateOddsRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    var dango_infos: std.ArrayList(pb.RacingBetsDangoInfo) = .empty;
    var did: i32 = 1;
    while (did <= 6) : (did += 1) {
        const key = try std.fmt.allocPrint(alloc.arena, "rb:bet:{d}:{d}", .{ txn.message.LegMatchId, did });
        const pt: i32 = @intCast(progress_comp.info.getKeyInt(key));
        try dango_infos.append(alloc.arena, .{
            .Id = did,
            .CurPoint = pt,
            .Odds = 200 + did * 20,
        });
    }
    txn.respond(.{
        .DangoInfo = dango_infos,
        .OddsVersion = "1",
        .ActivityId = txn.message.ActivityId,
        .LegMatchId = txn.message.LegMatchId,
        .ErrorCode = .Success,
    });
}

pub fn onRacingBetsTaskRewardRequest(
    txn: *Transaction(pb.RacingBetsTaskRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimOne(alloc, fs, progress_comp, reward_ctx, "rb_task", "RacingBetsTask", txn.message.TaskId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRacingBetsRepeatedTaskRewardRequest(
    txn: *Transaction(pb.RacingBetsRepeatedTaskRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimIdList(alloc, fs, progress_comp, reward_ctx, "rb_rtask", "RacingBetsTask", txn.message.TaskId.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRegressBonusRequest(
    txn: *Transaction(pb.RegressBonusRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimIdList(alloc, fs, progress_comp, reward_ctx, "reg_bonus", "RegressBonus", txn.message.Ids.items);
    var out: std.ArrayList(i32) = .empty;
    try out.appendSlice(alloc.arena, txn.message.Ids.items);
    txn.respond(.{ .ErrorCode = .Success, .Ids = out });
}

pub fn onRegressBuyBonusLvRequest(
    txn: *Transaction(pb.RegressBuyBonusLvRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const lv = try progress_comp.info.addKeyInt(alloc.gpa, "regress:bonus_lv", txn.message.BuyLv);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success, .BonusNum = @intCast(lv) });
}

pub fn onRegressDisposableRewardRequest(
    txn: *Transaction(pb.RegressDisposableRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimOne(alloc, fs, progress_comp, reward_ctx, "reg_disp", "RegressReward", 1);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRegressQuestionOpenRequest(
    txn: *Transaction(pb.RegressQuestionOpenRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "reg_q:{d}", .{txn.message.QuestionId});
    _ = try progress_comp.info.markClaim(alloc.gpa, key);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRegressTaskRequest(
    txn: *Transaction(pb.RegressTaskRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimOne(alloc, fs, progress_comp, reward_ctx, "reg_task", "RegressTask", txn.message.TaskId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRegressRepeatedTaskRequest(
    txn: *Transaction(pb.RegressRepeatedTaskRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimIdList(alloc, fs, progress_comp, reward_ctx, "reg_rtask", "RegressTask", txn.message.TaskId.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRegressSetCurUseTrialRoleRequest(
    txn: *Transaction(pb.RegressSetCurUseTrialRoleRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const prev: i32 = @intCast(progress_comp.info.getKeyInt("reg:trial_role"));
    try progress_comp.info.setKeyInt(alloc.gpa, "reg:trial_role", txn.message.TrialRoleId);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .Code = .Success,
        .ReplacedRoleId = prev,
        .RoleId = txn.message.TrialRoleId,
        .ReplacedCurRole = prev != 0,
    });
}

pub fn onRegressTrialRoleLvUpRequest(
    txn: *Transaction(pb.RegressTrialRoleLvUpRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "reg_role_lv:{d}", .{txn.message.TrialRoleId});
    const lv = try progress_comp.info.addKeyInt(alloc.gpa, key, 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .RoleId = txn.message.TrialRoleId,
        .CurUseRoleInfo = .{ .RoleId = txn.message.TrialRoleId, .Level = @intCast(lv) },
        .Code = .Success,
    });
}

pub fn onRhythmChangePartnerRoleRequest(
    txn: *Transaction(pb.RhythmChangePartnerRoleRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "rhythm:role", txn.message.RhythmRoleId);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRhythmMainLineInfoRequest(
    txn: *Transaction(pb.RhythmMainLineInfoRequest),
    progress_comp: *const PlayerProgressComponent,
) !void {
    txn.respond(.{
        .RhythmMainLinePb = .{ .RhythmRoleId = @intCast(progress_comp.info.getKeyInt("rhythm:role")) },
        .ErrorCode = .Success,
    });
}

pub fn onRhythmRankListRequest(
    txn: *Transaction(pb.RhythmRankListRequest),
    basic_comp: *const PlayerBasicComponent,
    player_id: @import("../../logic/PlayerID.zig"),
) !void {
    txn.respond(.{
        .SelfData = .{ .PlayerId = player_id.id, .Name = basic_comp.info.attributes.name },
        .ErrorCode = .Success,
    });
}

pub fn onRhythmRankListSelfDataRequest(
    txn: *Transaction(pb.RhythmRankListSelfDataRequest),
    basic_comp: *const PlayerBasicComponent,
    player_id: @import("../../logic/PlayerID.zig"),
) !void {
    txn.respond(.{
        .SelfData = .{ .PlayerId = player_id.id, .Name = basic_comp.info.attributes.name },
        .ErrorCode = .Success,
    });
}

pub fn onRhythmSetRedDotRequest(
    txn: *Transaction(pb.RhythmSetRedDotRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "rhythm:reddot:{d}", .{txn.message.ActivityId});
    try pb_kv.persistOptMsg(alloc, fs, progress_comp, key, txn.message.RedDot);
    txn.respond(.{ .ErrorCode = .Success, .RedDot = txn.message.RedDot });
}

pub fn onRhythmSettleRequest(
    txn: *Transaction(pb.RhythmSettleRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const score = if (txn.message.Payload) |p| p.Score else 0;
    const key = try std.fmt.allocPrint(alloc.arena, "rhythm:best:{d}", .{txn.message.SubLevelId});
    const prev: i32 = @intCast(progress_comp.info.getKeyInt(key));
    const best_flag = score > prev;
    const best = @max(score, prev);
    var dirty = false;
    if (best_flag) {
        try progress_comp.info.setKeyInt(alloc.gpa, key, best);
        dirty = true;
    }
    if (txn.message.Payload) |p| {
        if (p.Rank) |r| {
            const rank_val: i64 = @intFromEnum(r);
            if (rank_val > 0) {
                const rkey = try std.fmt.allocPrint(alloc.arena, "rhythm:rank:{d}", .{txn.message.SubLevelId});
                const prev_rank = progress_comp.info.getKeyInt(rkey);
                if (prev_rank <= 0 or rank_val < prev_rank) {
                    try progress_comp.info.setKeyInt(alloc.gpa, rkey, rank_val);
                    dirty = true;
                }
            }
        }
    }
    if (dirty) {
        try progress_comp.save(fs, alloc.arena);
    }
    txn.respond(.{
        .BestScore = best,
        .BestScoreFlag = best_flag,
        .ErrCode = .Success,
        .Payload = txn.message.Payload,
    });
}

pub fn onRhythmTaskOneKeyRewardRequest(
    txn: *Transaction(pb.RhythmTaskOneKeyRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimIdList(alloc, fs, progress_comp, reward_ctx, "rhythm_task", "RhythmTask", txn.message.TaskId.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRollBlockExitGamePlayRequest(
    txn: *Transaction(pb.RollBlockExitGamePlayRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "rb_blk:active:{d}", .{txn.message.IncId});
    try pb_kv.persistInt(alloc, fs, progress_comp, key, 0);
    txn.respond(.{ .Code = .Success });
}

pub fn onRollBlockGamePlayReadyRequest(
    txn: *Transaction(pb.RollBlockGamePlayReadyRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "rb_blk:active:{d}", .{txn.message.IncId});
    try pb_kv.persistInt(alloc, fs, progress_comp, key, 1);
    txn.respond(.{ .Code = .Success });
}

pub fn onRollBlockInputRequest(
    txn: *Transaction(pb.RollBlockInputRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "rb_blk:moves:{d}", .{txn.message.IncId});
    _ = try progress_comp.info.addKeyInt(alloc.gpa, key, 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .Code = .Success });
}

pub fn onRollBlockMovementFinishRequest(
    txn: *Transaction(pb.RollBlockMovementFinishRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "rb_blk:entity:{d}", .{txn.message.IncId});
    try pb_kv.persistInt(alloc, fs, progress_comp, key, txn.message.EntityId);
    if (LevelPlayHelper.fromQuestContext(qctx)) |lp_ctx| {
        const cfg_id = lp_ctx.scene.findConfigIdByNetId(txn.message.EntityId) orelse txn.message.EntityId;
        LevelPlayHelper.onConditionEvent(lp_ctx, progress_comp, .finish_roll_block, cfg_id, txn.message.IncId) catch {};
    }
    txn.respond(.{ .Code = .Success });
}

pub fn onRollBlockResetGamePlayRequest(
    txn: *Transaction(pb.RollBlockResetGamePlayRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "rb_blk:moves:{d}", .{txn.message.IncId});
    try pb_kv.persistInt(alloc, fs, progress_comp, key, 0);
    txn.respond(.{ .Code = .Success });
}

pub fn onRollBlockSwitchControlBlockRequest(
    txn: *Transaction(pb.RollBlockSwitchControlBlockRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "rb_blk:ctrl:{d}", .{txn.message.IncId});
    const prev = progress_comp.info.getKeyInt(key);
    const next = prev + 1;
    try pb_kv.persistInt(alloc, fs, progress_comp, key, next);
    txn.respond(.{ .PreMainControlBlockEntityId = prev, .CurMainControlBlockEntityId = next, .Code = .Success });
}

pub fn onRollBlockToggleHintRequest(
    txn: *Transaction(pb.RollBlockToggleHintRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "rb_blk:hint:{d}", .{txn.message.IncId});
    try pb_kv.persistInt(alloc, fs, progress_comp, key, if (txn.message.IsOpen) 1 else 0);
    txn.respond(.{ .Code = .Success });
}

// =========================================================================
// Spring Festival, Survivors, TowerDefence & TrapDefense
// =========================================================================

pub fn onSpringFestivalAtmosphereRewardRequest(
    txn: *Transaction(pb.SpringFestivalAtmosphereRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimIdList(alloc, fs, progress_comp, reward_ctx, "sf_atm", "SpringFestivalReward", txn.message.LevelIds.items);
    if (txn.message.LevelIds.items.len > 0) {
        _ = try progress_comp.info.addKeyInt(alloc.gpa, "spring:atmosphere", @intCast(txn.message.LevelIds.items.len));
        try progress_comp.save(fs, alloc.arena);
    }
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onSpringFestivalScoreRewardRequest(
    txn: *Transaction(pb.SpringFestivalScoreRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimIdList(alloc, fs, progress_comp, reward_ctx, "sf_score", "SpringFestivalReward", txn.message.ScoreIds.items);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onSpringFestivalTaskRewardRequest(
    txn: *Transaction(pb.SpringFestivalTaskRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimIdList(alloc, fs, progress_comp, reward_ctx, "sf_task", "SpringFestivalReward", txn.message.TaskIds.items);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onSpringSignDrawRewardRequest(
    txn: *Transaction(pb.SpringSignDrawRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimOne(alloc, fs, progress_comp, reward_ctx, "ssign_rw", "SpringReward", txn.message.Id);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSpringSignDrawRoleRequest(
    txn: *Transaction(pb.SpringSignDrawRoleRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const draw_id = try progress_comp.info.addKeyInt(alloc.gpa, "ssign:draw_idx", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .DrawId = @intCast(draw_id), .ErrorCode = .Success });
}

pub fn onSpringSignPhotoRewardRequest(
    txn: *Transaction(pb.SpringSignPhotoRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimOne(alloc, fs, progress_comp, reward_ctx, "ssign_photo", "SpringReward", 1);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSpringSignSkinRewardRequest(
    txn: *Transaction(pb.SpringSignSkinRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimOne(alloc, fs, progress_comp, reward_ctx, "ssign_skin", "SpringReward", 1);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSurvivorsLastInfoRequest(
    txn: *Transaction(pb.SurvivorsLastInfoRequest),
    progress_comp: *const PlayerProgressComponent,
) !void {
    txn.respond(.{
        .SurvivorLastInfo = .{
            .LevelId = @intCast(progress_comp.info.getKeyInt("surv:level")),
            .WaveId = @intCast(@max(1, progress_comp.info.getKeyInt("surv:wave"))),
            .MaxWaveId = @intCast(@max(1, progress_comp.info.getKeyInt("surv:max_wave"))),
        },
    });
}

pub fn onSurvivorsLockRequest(
    txn: *Transaction(pb.SurvivorsLockRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "surv:lock:{d}", .{txn.message.ClientSelectId});
    try progress_comp.info.setKeyInt(alloc.gpa, key, if (txn.message.IsLock) 1 else 0);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onSurvivorsNormalAwardRequest(
    txn: *Transaction(pb.SurvivorsNormalAwardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimIdList(alloc, fs, progress_comp, reward_ctx, "surv_norm", "SurvivorsTask", txn.message.Configs.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSurvivorsScoreAwardRequest(
    txn: *Transaction(pb.SurvivorsScoreAwardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimIdList(alloc, fs, progress_comp, reward_ctx, "surv_score", "SurvivorsScoreReward", txn.message.Configs.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSurvivorsOpDoneRequest(
    txn: *Transaction(pb.SurvivorsOpDoneRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "surv:op:{d}", .{txn.message.IncId});
    try pb_kv.saveMsg(alloc, fs, progress_comp, key, txn.message);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onSurvivorsStepAdvanceRequest(
    txn: *Transaction(pb.SurvivorsStepAdvanceRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    const wave = try progress_comp.info.addKeyInt(alloc.gpa, "surv:wave", 1);
    const max_w = @max(wave, progress_comp.info.getKeyInt("surv:max_wave"));
    try progress_comp.info.setKeyInt(alloc.gpa, "surv:max_wave", max_w);
    try progress_comp.save(fs, alloc.arena);
    if (LevelPlayHelper.fromQuestContext(qctx)) |lp_ctx| {
        LevelPlayHelper.onConditionEvent(lp_ctx, progress_comp, .finish_survivors_rouge, 0, @intCast(wave)) catch {};
    }
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onSurvivorsResultRequest(
    txn: *Transaction(pb.SurvivorsResultRequest),
    progress_comp: *const PlayerProgressComponent,
) !void {
    txn.respond(.{
        .ErrCode = .Success,
        .ResultView = .{
            .LevelId = @intCast(progress_comp.info.getKeyInt("surv:level")),
            .WaveId = @intCast(@max(1, progress_comp.info.getKeyInt("surv:wave"))),
            .IsFinishCond = true,
            .KillMonsterCount = @intCast(progress_comp.info.getKeyInt("simple_combat:kills")),
        },
    });
}

pub fn onSurvivorsTalentLevelUpRequest(
    txn: *Transaction(pb.SurvivorsTalentLevelUpRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "surv:talent:{d}", .{txn.message.SkillId});
    const lv = try progress_comp.info.addKeyInt(alloc.gpa, key, 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success, .Level = @intCast(lv) });
}

pub fn onSurvivorsWeaponInfoRequest(
    txn: *Transaction(pb.SurvivorsWeaponInfoRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    _ = txn.message;
    var kills: std.ArrayList(pb.MapEntry(i32, i32)) = .empty;
    for (progress_comp.info.dynamic.ints) |entry| {
        if (std.mem.startsWith(u8, entry.key, "surv:wp_kill:")) {
            if (std.fmt.parseInt(i32, entry.key["surv:wp_kill:".len..], 10)) |wid| {
                try kills.append(alloc.arena, .{ .key = wid, .value = @intCast(entry.val) });
            } else |_| {}
        }
    }
    txn.respond(.{ .WeaponKillDict = kills });
}

pub fn onTowerDefenceChallengeQuitRequest(
    txn: *Transaction(pb.TowerDefenceChallengeQuitRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    try pb_kv.persistInt(alloc, fs, progress_comp, "td:in_challenge", 0);
    const score: i32 = @intCast(@max(1000, progress_comp.info.getKeyInt("td:last_score")));
    txn.respond(.{ .ErrorCode = .Success, .Score = score });
}

pub fn onTowerDefenceInstanceRewardRequest(
    txn: *Transaction(pb.TowerDefenceInstanceRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimIdList(alloc, fs, progress_comp, reward_ctx, "td_inst", "TowerDefenceReward", txn.message.Ids.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onTowerDefenceScoreRewardRequest(
    txn: *Transaction(pb.TowerDefenceScoreRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimIdList(alloc, fs, progress_comp, reward_ctx, "td_score", "TowerDefenceReward", txn.message.Ids.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onTowerDefenceRandListRequest(
    txn: *Transaction(pb.TowerDefenceRandListRequest),
    progress_comp: *const PlayerProgressComponent,
) !void {
    txn.respond(.{
        .ShowName = progress_comp.info.getKeyInt("td:show_name") != 0,
        .ErrorCode = .Success,
    });
}

pub fn onTowerDefenceRankShowNameRequest(
    txn: *Transaction(pb.TowerDefenceRankShowNameRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "td:show_name", if (txn.message.ShowName) 1 else 0);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onTowerDefenceSelfPassRequest(
    txn: *Transaction(pb.TowerDefenceSelfPassRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    var list: std.ArrayList(pb.OneTowerDefencePassInfo) = .empty;
    const score: i32 = @intCast(@max(0, progress_comp.info.getKeyInt("td:last_score")));
    const pass_time: i32 = @intCast(@max(0, progress_comp.info.getKeyInt("td:best_time")));
    if (score > 0 or txn.message.Id != 0) {
        try list.append(alloc.arena, .{
            .ChallengeId = txn.message.Id,
            .IsSingle = true,
            .PassInfo = .{ .Score = score, .PassTime = pass_time },
        });
    }
    txn.respond(.{ .SelfPassData = list });
}

pub fn onTrapDefenseAuxiliaryBranchRequest(
    txn: *Transaction(pb.TrapDefenseAuxiliaryBranchRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "trap:aux_br:{d}", .{txn.message.Id});
    try progress_comp.info.setKeyInt(alloc.gpa, key, txn.message.Branch);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onTrapDefenseAuxiliaryLevelUpRequest(
    txn: *Transaction(pb.TrapDefenseAuxiliaryLevelUpRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "trap:aux_lv:{d}", .{txn.message.Id});
    _ = try progress_comp.info.addKeyInt(alloc.gpa, key, 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onTrapDefenseAuxiliaryResetRequest(
    txn: *Transaction(pb.TrapDefenseAuxiliaryResetRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "trap:aux_lv:{d}", .{txn.message.Id});
    try progress_comp.info.setKeyInt(alloc.gpa, key, 0);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onTrapDefenseBuildingBranchRequest(
    txn: *Transaction(pb.TrapDefenseBuildingBranchRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "trap:bld_br:{d}", .{txn.message.Id});
    try progress_comp.info.setKeyInt(alloc.gpa, key, txn.message.Branch);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onTrapDefenseBuildingLevelUpRequest(
    txn: *Transaction(pb.TrapDefenseBuildingLevelUpRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "trap:bld_lv:{d}", .{txn.message.Id});
    _ = try progress_comp.info.addKeyInt(alloc.gpa, key, 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onTrapDefenseBuildingResetRequest(
    txn: *Transaction(pb.TrapDefenseBuildingResetRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "trap:bld_lv:{d}", .{txn.message.Id});
    try progress_comp.info.setKeyInt(alloc.gpa, key, 0);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onTrapDefenseBuyShopProductRequest(
    txn: *Transaction(pb.TrapDefenseBuyShopProductRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "trap:shop_buy:{d}", .{txn.message.ItemConfigId});
    _ = try progress_comp.info.addKeyInt(alloc.gpa, key, @max(1, txn.message.BuyCount));
    try progress_comp.save(fs, alloc.arena);
    const rem: i32 = @intCast(@max(0, 5 - progress_comp.info.getKeyInt("trap:shop_ref")));
    txn.respond(.{ .ErrorCode = .Success, .ShopInfo = .{ .TotalRefreshTimes = 5, .RemainRefreshTimes = rem } });
}

pub fn onTrapDefenseChallengeQuitRequest(
    txn: *Transaction(pb.TrapDefenseChallengeQuitRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try pb_kv.persistInt(alloc, fs, progress_comp, "trap:in_challenge", if (txn.message.Settle) 0 else 2);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onTrapDefenseCurChallengeInfoRequest(
    txn: *Transaction(pb.TrapDefenseCurChallengeInfoRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    _ = txn.message;
    var targets: std.ArrayList(i32) = .empty;
    for (progress_comp.info.dynamic.claims) |c| {
        if (std.mem.startsWith(u8, c, "trap:target:")) {
            if (std.fmt.parseInt(i32, c["trap:target:".len..], 10)) |tid| {
                try targets.append(alloc.arena, tid);
            } else |_| {}
        }
    }
    txn.respond(.{ .ErrorCode = .Success, .Target = targets });
}

pub fn onTrapDefenseDevelopResetRequest(
    txn: *Transaction(pb.TrapDefenseDevelopResetRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "trap:reset_cnt:{d}", .{txn.message.ActivityId});
    _ = try progress_comp.info.addKeyInt(alloc.gpa, key, 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onTrapDefenseGetRewardRequest(
    txn: *Transaction(pb.TrapDefenseGetRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimIdList(alloc, fs, progress_comp, reward_ctx, "trap_rw", "TrapDefenseReward", txn.message.Ids.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onTrapDefenseGetSpecialRewardRequest(
    txn: *Transaction(pb.TrapDefenseGetSpecialRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimIdList(alloc, fs, progress_comp, reward_ctx, "trap_sp_rw", "TrapDefenseSpecialReward", txn.message.Ids.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onTrapDefensePreviewFinishRequest(
    txn: *Transaction(pb.TrapDefensePreviewFinishRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    _ = txn.message;
    try pb_kv.persistClaim(alloc, fs, progress_comp, "trap:preview_done");
    if (LevelPlayHelper.fromQuestContext(qctx)) |lp_ctx| {
        LevelPlayHelper.onConditionEvent(lp_ctx, progress_comp, .finish_trap_defense, 0, 0) catch {};
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onTrapDefenseRefreshGainingGroupRequest(
    txn: *Transaction(pb.TrapDefenseRefreshGainingGroupRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    const ref_cnt = try progress_comp.info.addKeyInt(alloc.gpa, "trap:gain_ref", 1);
    try progress_comp.save(fs, alloc.arena);
    const rem: i32 = @intCast(@max(0, 3 - ref_cnt));
    txn.respond(.{ .NewRewardInfo = .{ .TotalRefreshTimes = 3, .RemainRefreshTimes = rem }, .ErrorCode = .Success });
}

pub fn onTrapDefenseRefreshShopRequest(
    txn: *Transaction(pb.TrapDefenseRefreshShopRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    const ref_cnt = try progress_comp.info.addKeyInt(alloc.gpa, "trap:shop_ref", 1);
    try progress_comp.save(fs, alloc.arena);
    const rem: i32 = @intCast(@max(0, 5 - ref_cnt));
    txn.respond(.{ .NewShopInfo = .{ .TotalRefreshTimes = 5, .RemainRefreshTimes = rem }, .ErrorCode = .Success });
}

pub fn onTrapDefenseSelectGainingGroupRequest(
    txn: *Transaction(pb.TrapDefenseSelectGainingGroupRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "trap:gain_grp", txn.message.GainingGroupId);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onTrapDefenseSlotUpdateRequest(
    txn: *Transaction(pb.TrapDefenseSlotUpdateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try pb_kv.saveMsg(alloc, fs, progress_comp, "trap:slots", txn.message);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onTrapDefenseTechUnlockRequest(
    txn: *Transaction(pb.TrapDefenseTechUnlockRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "trap:tech:{d}", .{txn.message.Id});
    _ = try progress_comp.info.markClaim(alloc.gpa, key);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onTrapDefenseUseItemRequest(
    txn: *Transaction(pb.TrapDefenseUseItemRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "trap:use_item:{d}", .{txn.message.ItemConfigId});
    _ = try progress_comp.info.addKeyInt(alloc.gpa, key, 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}
