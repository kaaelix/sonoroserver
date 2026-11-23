const std = @import("std");
const common = @import("common");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const Assets = @import("../../data/Assets.zig");
const RewardIndex = @import("../../data/RewardIndex.zig");
const Transaction = @import("../handlers.zig").Transaction;
const PlayerBasicComponent = @import("../../logic/component/player/PlayerBasicComponent.zig");
const PlayerInventoryComponent = @import("../../logic/component/player/PlayerInventoryComponent.zig");
const PlayerProgressComponent = @import("../../logic/component/player/PlayerProgressComponent.zig");
const ConsumeHelper = @import("../../logic/helpers/consume.zig");
const EnergyHelper = @import("../../logic/helpers/energy.zig");
const RewardService = @import("../../logic/helpers/reward.zig");
const pb_kv = @import("pb_kv.zig");

const FileSystem = common.FileSystem;

pub fn onRoleFavorUnLockRequest(
    txn: *Transaction(pb.RoleFavorUnLockRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const itype: i32 = if (txn.message.ItemType) |t| @intFromEnum(t) else 0;
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "favor_unlock:{d}:{d}:{d}", .{ txn.message.RoleId, itype, txn.message.UnLockId });
    _ = try progress_comp.info.markClaim(alloc.gpa, k);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .RoleId = txn.message.RoleId,
        .ItemType = txn.message.ItemType,
        .UnLockId = txn.message.UnLockId,
        .ErrCode = .Success,
    });
}

pub fn onRoleGiftRewardRequest(
    txn: *Transaction(pb.RoleGiftRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "role_gift_clm:{d}", .{txn.message.TaskId});
    if (try progress_comp.info.markClaim(alloc.gpa, k)) {
        try progress_comp.save(fs, alloc.arena);
        _ = try RewardService.grantItems(reward_ctx, 0, true, &.{
            .{ .item_id = 3, .count = 40 },
            .{ .item_id = 2, .count = 10000 },
        }, 19500);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRoleMotionUnLockRequest(
    txn: *Transaction(pb.RoleMotionUnLockRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "role_motion:{d}:{d}", .{ txn.message.RoleId, txn.message.UnLockId });
    _ = try progress_comp.info.markClaim(alloc.gpa, k);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .RoleId = txn.message.RoleId,
        .UnLockId = txn.message.UnLockId,
        .ErrCode = .Success,
    });
}

pub fn onRoleTrainRequest(
    txn: *Transaction(pb.RoleTrainRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "role_train:{d}:{d}", .{ txn.message.RoleId, txn.message.TrainType });
    _ = try progress_comp.info.addKeyInt(alloc.gpa, k, 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .GreatSucc = true, .ErrorCode = .Success });
}

pub fn onRoleUpdateDevelopTargetRequest(
    txn: *Transaction(pb.RoleUpdateDevelopTargetRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "role_dev_target:{d}", .{txn.message.RoleId});
    try progress_comp.info.setKeyInt(alloc.gpa, k, txn.message.PlanId);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn buildShortMessageInfos(
    arena: std.mem.Allocator,
    assets: *const Assets,
    progress_comp: *const PlayerProgressComponent,
    io: std.Io,
) !std.ArrayList(pb.ShortMessageInfo) {
    var infos: std.ArrayList(pb.ShortMessageInfo) = .empty;
    const rows = assets.reward_index.getRows(io, "ShortMessage");
    var kbuf: [48]u8 = undefined;
    for (rows) |row| {
        const cid: i32 = @intCast(row.key);
        // Exclude development test scripts that don't exist in client's SQLite DB (avoids CommonDbConnect Error)
        if (cid >= 38000 and cid <= 38999) continue;

        const read_k = try std.fmt.bufPrint(&kbuf, "sm:read:{d}", .{cid});
        const recv_k = try std.fmt.bufPrint(&kbuf, "sm:recv:{d}", .{cid});
        const fin_k = try std.fmt.bufPrint(&kbuf, "sm:fin:{d}", .{cid});
        const last_k = try std.fmt.bufPrint(&kbuf, "sm:last:{d}", .{cid});
        const saved_last = progress_comp.info.getKeyInt(last_k);
        const last_cid: i32 = if (saved_last > 0) @intCast(saved_last) else cid;

        try infos.append(arena, .{
            .ConfigId = cid,
            .LastConfigId = last_cid,
            .IsRead = progress_comp.info.hasClaim(read_k),
            .IsReceived = progress_comp.info.hasClaim(recv_k),
            .IsFinish = progress_comp.info.hasClaim(fin_k),
            .UnlockTime = 1,
        });
    }
    return infos;
}

pub fn onShortMessageInfoRequest(
    txn: *Transaction(pb.ShortMessageInfoRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
    progress_comp: *const PlayerProgressComponent,
    io: std.Io,
) !void {
    const infos = try buildShortMessageInfos(alloc.arena, assets, progress_comp, io);
    const charm_saved = progress_comp.info.getKeyInt("sm:charm");
    const theme_saved = progress_comp.info.getKeyInt("sm:theme");
    const bg_saved = progress_comp.info.getKeyInt("sm:bg");
    const bubble_saved = progress_comp.info.getKeyInt("sm:bubble");

    const charm: i32 = if (charm_saved > 0) @intCast(charm_saved) else 80892001;
    const theme: i32 = if (theme_saved > 0) @intCast(theme_saved) else 80891001;
    const bg: i32 = if (bg_saved > 0) @intCast(bg_saved) else 80820001;
    const bubble: i32 = if (bubble_saved > 0) @intCast(bubble_saved) else 80810001;

    var bg_ids: std.ArrayList(i32) = .empty;
    try bg_ids.appendSlice(alloc.arena, &.{ 80820001, 80820002, 80820004 });

    var theme_ids: std.ArrayList(i32) = .empty;
    try theme_ids.appendSlice(alloc.arena, &.{ 80891001, 80891002 });

    var bubble_ids: std.ArrayList(i32) = .empty;
    try bubble_ids.appendSlice(alloc.arena, &.{ 80810001, 80810002, 80810003 });

    var charm_ids: std.ArrayList(i32) = .empty;
    try charm_ids.appendSlice(alloc.arena, &.{ 80892001, 80892002 });

    txn.respond(.{
        .ShortMessageInfos = infos,
        .ChatCharmId = charm,
        .ChatCharmIds = charm_ids,
        .ChatThemeId = theme,
        .ChatThemeIds = theme_ids,
        .ChatBgId = bg,
        .ChatBgIds = bg_ids,
        .BubbleId = bubble,
        .BubbleIds = bubble_ids,
        .ErrCode = .Success,
    });
}

pub fn onShortMessageReadRequest(
    txn: *Transaction(pb.ShortMessageReadRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "sm:read:{d}", .{txn.message.ConfigId});
    _ = try progress_comp.info.markClaim(alloc.gpa, k);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onShortMessageReceivedRequest(
    txn: *Transaction(pb.ShortMessageReceivedRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "sm:recv:{d}", .{txn.message.ConfigId});
    if (try progress_comp.info.markClaim(alloc.gpa, k)) {
        try progress_comp.save(fs, alloc.arena);
        _ = try RewardService.grantTableReward(reward_ctx, "ShortMessage", txn.message.ConfigId, false, true, 19500, &.{.{ .item_id = 3, .count = 20 }}, null);
    }
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onShortMessageReplyRequest(
    txn: *Transaction(pb.ShortMessageReplyRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "sm:opt:{d}:{d}", .{ txn.message.ConfigId, txn.message.CurOptionId });
    try progress_comp.info.setKeyInt(alloc.gpa, k, txn.message.SelectId);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onShortMessageUpdateRequest(
    txn: *Transaction(pb.ShortMessageUpdateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    if (txn.message.IsFinish) {
        const k = try std.fmt.bufPrint(&kbuf, "sm:fin:{d}", .{txn.message.ConfigId});
        _ = try progress_comp.info.markClaim(alloc.gpa, k);
    }
    const last_k = try std.fmt.bufPrint(&kbuf, "sm:last:{d}", .{txn.message.ConfigId});
    try progress_comp.info.setKeyInt(alloc.gpa, last_k, txn.message.LastConfigId);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onShortMessageChatCharmChangeRequest(
    txn: *Transaction(pb.ShortMessageChatCharmChangeRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "sm:charm", txn.message.ChatCharmId);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onShortMessageChatThemeChangeRequest(
    txn: *Transaction(pb.ShortMessageChatThemeChangeRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "sm:theme", txn.message.ChatThemeId);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onShortMessageShowChangeRequest(
    txn: *Transaction(pb.ShortMessageShowChangeRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "sm:bg", txn.message.ChatBgId);
    try progress_comp.info.setKeyInt(alloc.gpa, "sm:bubble", txn.message.BubbleId);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onBirthDayCardRoleRequest(
    txn: *Transaction(pb.BirthDayCardRoleRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "birthday:role:{d}", .{txn.message.YearId});
    try progress_comp.info.setKeyInt(alloc.gpa, k, txn.message.RoleID);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onBirthDayRewardRequest(
    txn: *Transaction(pb.BirthDayRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "birthday:clm:{d}", .{txn.message.YearId});
    if (try progress_comp.info.markClaim(alloc.gpa, k)) {
        try progress_comp.info.setKeyInt(alloc.gpa, "birthday:reward_year", txn.message.YearId);
        try progress_comp.save(fs, alloc.arena);
        _ = try RewardService.grantItems(reward_ctx, 0, true, &.{
            .{ .item_id = 3, .count = 160 },
            .{ .item_id = 2, .count = 20000 },
        }, 19500);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

fn currentBattlePassId(assets: *const Assets, io: std.Io) i32 {
    const rows = assets.reward_index.getRows(io, "BattlePass");
    return if (rows.len > 0) @intCast(rows[rows.len - 1].key) else 2;
}

fn getBattlePassItem(assets: *const Assets, io: std.Io, bp_id: i32, level: i32, is_pay: bool) i32 {
    const keys = [_]i64{
        RewardIndex.compositeKey(bp_id, level),
        RewardIndex.compositeKey(13, level),
        RewardIndex.compositeKey(0, level),
        RewardIndex.compositeKey(1, level),
    };
    for (keys) |k| {
        if (assets.reward_index.getRow(io, "BattlePassReward", k)) |row| {
            if (is_pay) {
                if (row.pay_items.len > 0) return row.pay_items[0].item_id;
            } else {
                if (row.items.len > 0) return row.items[0].item_id;
            }
        }
    }
    return 0;
}

pub fn onBattlePassRequest(
    txn: *Transaction(pb.BattlePassRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
    progress_comp: *const PlayerProgressComponent,
    io: std.Io,
) !void {
    const now_s = EnergyHelper.nowSeconds(io);
    const bp_id = currentBattlePassId(assets, io);
    const lvl = @max(1, @as(i32, @intCast(progress_comp.info.getKeyInt("bp:level"))));
    const exp = @as(i32, @intCast(@max(0, progress_comp.info.getKeyInt("bp:exp"))));
    const paid_raw = progress_comp.info.getKeyInt("bp:paid");
    const pay_status: pb.BattlePassPayStatus = if (paid_raw >= 2) .Advanced else if (paid_raw >= 1) .Paid else .NoPaid;

    var taken: std.ArrayList(pb.PbBattlePassReward) = .empty;
    var kbuf: [48]u8 = undefined;
    var l: i32 = 1;
    while (l <= 70) : (l += 1) {
        const fk = try std.fmt.bufPrint(&kbuf, "bp:clm:0:{d}", .{l});
        if (progress_comp.info.hasClaim(fk)) {
            const f_item = getBattlePassItem(assets, io, bp_id, l, false);
            try taken.append(alloc.arena, .{ .Level = l, .Type = .Free, .ItemId = f_item });
        }
        const pk = try std.fmt.bufPrint(&kbuf, "bp:clm:1:{d}", .{l});
        if (progress_comp.info.hasClaim(pk)) {
            const p_item = getBattlePassItem(assets, io, bp_id, l, true);
            try taken.append(alloc.arena, .{ .Level = l, .Type = .Pay, .ItemId = p_item });
        }
    }

    txn.respond(.{
        .ErrorCode = .Success,
        .BattlePass = .{
            .InTimeRange = true,
            .Id = bp_id,
            .Level = lvl,
            .Exp = exp,
            .WeeklyTotalExp = exp,
            .PayStatus = pay_status,
            .TakenRewards = taken,
            .BeginTime = now_s - 86_400,
            .EndTime = now_s + 30 * 86_400,
            .HadEnter = progress_comp.info.hasClaim("bp:entered"),
        },
    });
}

pub fn onBattlePassEnterPush(
    txn: *Transaction(pb.BattlePassEnterPush),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn;
    _ = try progress_comp.info.markClaim(alloc.gpa, "bp:entered");
    try progress_comp.save(fs, alloc.arena);
}

pub fn onBattlePassLevelUpRequest(
    txn: *Transaction(pb.BattlePassLevelUpRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    progress_comp: *PlayerProgressComponent,
    io: std.Io,
) !void {
    const cur = @max(1, @as(i32, @intCast(progress_comp.info.getKeyInt("bp:level"))));
    const requested = if (txn.message.Level > 0) txn.message.Level else cur + 1;
    const next = std.math.clamp(requested, cur, 70);
    const levels_gained = next - cur;
    if (levels_gained <= 0) {
        txn.respond(.{ .ErrorCode = .Success });
        return;
    }

    const costs = [_]ConsumeHelper.CostItem{.{ .id = 3, .count = levels_gained * 100 }};
    if (!(try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, &costs, io))) {
        txn.respond(.{ .ErrorCode = .ErrItemNotEnough });
        return;
    }

    try progress_comp.info.setKeyInt(alloc.gpa, "bp:level", next);
    try progress_comp.save(fs, alloc.arena);
    try txn.conn.push(alloc.gpa, io, pb.BattlePassExpUpdateNotify{
        .Level = next,
        .Exp = 0,
        .WeeklyTotalExp = next * 1000,
    });
    txn.respond(.{ .ErrorCode = .Success });
}

fn grantBattlePassLevelReward(
    reward_ctx: RewardService.RewardContext,
    assets: *const Assets,
    io: std.Io,
    bp_id: i32,
    level: i32,
    is_pay: bool,
) !bool {
    const keys = [_]i64{
        RewardIndex.compositeKey(bp_id, level),
        RewardIndex.compositeKey(0, level),
        RewardIndex.compositeKey(1, level),
    };
    for (keys) |k| {
        if (assets.reward_index.getRow(io, "BattlePassReward", k) != null) {
            _ = try RewardService.grantTableReward(
                reward_ctx,
                "BattlePassReward",
                k,
                is_pay,
                false,
                18500,
                &.{.{ .item_id = 2, .count = 10000 }},
                null,
            );
            return true;
        }
    }
    return false;
}

pub fn onBattlePassTakeRewardRequest(
    txn: *Transaction(pb.BattlePassTakeRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
    io: std.Io,
) !void {
    const t_val: i32 = if (txn.message.Type) |t| @intFromEnum(t) else 0;
    const is_pay = t_val == 1;
    const cur_lvl = @min(70, @max(1, @as(i32, @intCast(progress_comp.info.getKeyInt("bp:level")))));
    const paid = progress_comp.info.getKeyInt("bp:paid") != 0;
    if (txn.message.Level < 1 or txn.message.Level > cur_lvl or (is_pay and !paid)) {
        txn.respond(.{ .ErrorCode = .RequestParamError });
        return;
    }
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "bp:clm:{d}:{d}", .{ t_val, txn.message.Level });
    if (try progress_comp.info.markClaim(alloc.gpa, k)) {
        try progress_comp.save(fs, alloc.arena);
        const bp_id = currentBattlePassId(assets, io);
        if (!(try grantBattlePassLevelReward(reward_ctx, assets, io, bp_id, txn.message.Level, is_pay))) {
            _ = try RewardService.grantItems(reward_ctx, 0, true, &.{.{ .item_id = 2, .count = 10000 }}, 18500);
        }
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onBattlePassTakeAllRewardRequest(
    txn: *Transaction(pb.BattlePassTakeAllRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
    io: std.Io,
) !void {
    const cur_lvl = @min(70, @max(1, @as(i32, @intCast(progress_comp.info.getKeyInt("bp:level")))));
    const paid = progress_comp.info.getKeyInt("bp:paid") != 0;
    const bp_id = currentBattlePassId(assets, io);
    var taken: std.ArrayList(pb.PbBattlePassReward) = .empty;
    var dirty = false;
    var kbuf: [48]u8 = undefined;

    var l: i32 = 1;
    while (l <= cur_lvl) : (l += 1) {
        for ([_]i32{ 0, 1 }) |tp| {
            if (tp == 1 and !paid) continue;
            const k = try std.fmt.bufPrint(&kbuf, "bp:clm:{d}:{d}", .{ tp, l });
            if (try progress_comp.info.markClaim(alloc.gpa, k)) {
                dirty = true;
                _ = try grantBattlePassLevelReward(reward_ctx, assets, io, bp_id, l, tp == 1);
                const it_id = getBattlePassItem(assets, io, bp_id, l, tp == 1);
                try taken.append(alloc.arena, .{ .Level = l, .Type = if (tp == 1) .Pay else .Free, .ItemId = it_id });
            }
        }
    }
    if (dirty) try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success, .TakenRewards = taken });
}

pub fn onBattlePassTaskRequest(
    txn: *Transaction(pb.BattlePassTaskRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
    progress_comp: *const PlayerProgressComponent,
    io: std.Io,
) !void {
    const now_s = EnergyHelper.nowSeconds(io);
    const rows = assets.reward_index.getRows(io, "BattlePassTask");
    var tasks: std.ArrayList(pb.PbBattlePassTask) = .empty;
    var kbuf: [48]u8 = undefined;
    for (rows) |row| {
        const tid: i32 = @intCast(row.key);
        if (tid <= 0) continue;
        const k = try std.fmt.bufPrint(&kbuf, "bp:task:{d}", .{tid});
        try tasks.append(alloc.arena, .{
            .Id = tid,
            .Current = 1,
            .Target = 1,
            .IsFinished = true,
            .IsTaken = progress_comp.info.hasClaim(k),
        });
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .Tasks = tasks,
        .DayEnd = now_s + 86_400,
        .WeekEnd = now_s + 7 * 86_400,
    });
}

pub fn onBattlePassTaskTakeRequest(
    txn: *Transaction(pb.BattlePassTaskTakeRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    io: std.Io,
) !void {
    var taken_ids: std.ArrayList(i32) = .empty;
    var exp_gain: i32 = 0;
    var kbuf: [48]u8 = undefined;
    for (txn.message.Ids.items) |tid| {
        const k = try std.fmt.bufPrint(&kbuf, "bp:task:{d}", .{tid});
        if (!(try progress_comp.info.markClaim(alloc.gpa, k))) continue;
        try taken_ids.append(alloc.arena, tid);
        if (assets.reward_index.getRow(io, "BattlePassTask", tid)) |row| {
            for (row.items) |it| exp_gain += it.count;
        } else {
            exp_gain += 250;
        }
    }

    if (taken_ids.items.len > 0) {
        var cur_lvl = @max(1, @as(i32, @intCast(progress_comp.info.getKeyInt("bp:level"))));
        var cur_exp = @as(i32, @intCast(@max(0, progress_comp.info.getKeyInt("bp:exp")))) + exp_gain;
        while (cur_exp >= 1000 and cur_lvl < 70) {
            cur_exp -= 1000;
            cur_lvl += 1;
        }
        try progress_comp.info.setKeyInt(alloc.gpa, "bp:level", cur_lvl);
        try progress_comp.info.setKeyInt(alloc.gpa, "bp:exp", cur_exp);
        try progress_comp.save(fs, alloc.arena);
        try txn.conn.push(alloc.gpa, io, pb.BattlePassExpUpdateNotify{
            .Level = cur_lvl,
            .Exp = cur_exp,
            .WeeklyTotalExp = cur_lvl * 1000 + cur_exp,
        });
    }

    txn.respond(.{ .ErrorCode = .Success, .Ids = taken_ids });
}

pub fn onMonthCardRequest(
    txn: *Transaction(pb.MonthCardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
    io: std.Io,
) !void {
    var days = @as(i32, @intCast(@max(0, progress_comp.info.getKeyInt("month_card:days"))));
    var dirty = false;
    if (days <= 0 and (try progress_comp.info.markClaim(alloc.gpa, "month_card:init"))) {
        days = 30;
        try progress_comp.info.setKeyInt(alloc.gpa, "month_card:days", days);
        dirty = true;
    }
    const day_bucket = @divFloor(EnergyHelper.nowSeconds(io), 86_400);
    const last_got_day = progress_comp.info.getKeyInt("month_card:last_day");
    const is_got_today = (last_got_day == day_bucket);
    var granted_today = false;
    if (!is_got_today and days > 0) {
        days -= 1;
        try progress_comp.info.setKeyInt(alloc.gpa, "month_card:days", days);
        try progress_comp.info.setKeyInt(alloc.gpa, "month_card:last_day", day_bucket);
        dirty = true;
        granted_today = true;
    }
    if (dirty) try progress_comp.save(fs, alloc.arena);
    if (granted_today) {
        _ = try RewardService.grantItems(reward_ctx, 0, true, &.{.{ .item_id = 3, .count = 90 }}, 19500);
    }
    txn.respond(.{
        .Days = days,
        .IsDailyGot = is_got_today or granted_today,
        .ErrorCode = .Success,
    });
}

pub fn onWeekCardInfoRequest(
    txn: *Transaction(pb.WeekCardInfoRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
    io: std.Io,
) !void {
    const now_s = EnergyHelper.nowSeconds(io);
    var infos: std.ArrayList(pb.WeekCardInfo) = .empty;
    var kbuf: [48]u8 = undefined;

    if (txn.message.WeekCardIds.items.len > 0) {
        for (txn.message.WeekCardIds.items) |wid| {
            var contents: std.ArrayList(pb.WeekCardContentInfo) = .empty;
            const k = try std.fmt.bufPrint(&kbuf, "week_card:{d}", .{wid});
            try contents.append(alloc.arena, .{
                .WeekCardContentId = wid,
                .UnlockDay = 1,
                .Status = if (progress_comp.info.hasClaim(k)) 2 else 1,
            });
            try infos.append(alloc.arena, .{
                .WeekCardId = wid,
                .Days = 7,
                .BuyTimeStamp = now_s,
                .EndTimeStamp = now_s + 7 * 86_400,
                .WeekCardContentInfos = contents,
            });
        }
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .WeekCardInfos = infos,
        .Infos = if (infos.items.len > 0) infos.items[0] else .{
            .WeekCardId = txn.message.WeekCardId,
            .Days = 7,
            .BuyTimeStamp = now_s,
            .EndTimeStamp = now_s + 7 * 86_400,
        },
    });
}

pub fn onWeekCardRewardRequest(
    txn: *Transaction(pb.WeekCardRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "week_card:{d}", .{txn.message.WeekCardContentId});
    if (try progress_comp.info.markClaim(alloc.gpa, k)) {
        try progress_comp.save(fs, alloc.arena);
        _ = try RewardService.grantItems(reward_ctx, 0, true, &.{
            .{ .item_id = 3, .count = 100 },
            .{ .item_id = 2, .count = 15000 },
        }, 1);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onPayGiftInfoRequest(
    txn: *Transaction(pb.PayGiftInfoRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
    progress_comp: *const PlayerProgressComponent,
    io: std.Io,
) !void {
    var infos: std.ArrayList(pb.PayGiftInfo) = .empty;
    var kbuf: [48]u8 = undefined;
    for (assets.reward_index.getRows(io, "PayGift")) |row| {
        const gid: i32 = @intCast(row.key);
        const k = try std.fmt.bufPrint(&kbuf, "paygift:{d}", .{gid});
        const bought: i32 = @intCast(@max(0, progress_comp.info.getKeyInt(k)));
        try infos.append(alloc.arena, .{
            .Id = gid,
            .PayId = gid,
            .BuyLimit = 99,
            .BoughtCount = bought,
            .BeginTime = 1,
            .EndTime = 2_000_000_000,
            .IsCanBuy = true,
        });
    }
    txn.respond(.{
        .Infos = infos,
        .Version = if (txn.message.Version.len > 0) txn.message.Version else "1",
        .ErrorCode = .Success,
    });
}

pub fn onPayGiftRequest(
    txn: *Transaction(pb.PayGiftRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "paygift:{d}", .{txn.message.Id});
    _ = try progress_comp.info.addKeyInt(alloc.gpa, k, 1);
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "topup:score", 300);
    try progress_comp.save(fs, alloc.arena);
    _ = try RewardService.grantItems(reward_ctx, 0, true, &.{
        .{ .item_id = 4, .count = 300 },
        .{ .item_id = 2, .count = 50000 },
    }, 8001);
    txn.respond(.{ .ErrorCode = .Success, .ReceiptId = "local_receipt" });
}

pub fn onPayItemRequest(
    txn: *Transaction(pb.PayItemRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "payitem:{d}", .{txn.message.Id});
    _ = try progress_comp.info.addKeyInt(alloc.gpa, k, 1);
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "topup:score", 648);
    try progress_comp.save(fs, alloc.arena);
    _ = try RewardService.grantItems(reward_ctx, 0, true, &.{.{ .item_id = 4, .count = 648 }}, 8001);
    txn.respond(.{ .ErrorCode = .Success, .ReceiptId = "local_receipt" });
}

pub fn onGachaAccumulateInfoRequest(
    txn: *Transaction(pb.GachaAccumulateInfoRequest),
) !void {
    txn.respond(.{
        .ErrorCode = .Success,
        .GachaAccumulateInfo = .{
            .GachaAccumulateId = txn.message.GachaAccumulateId,
            .GachaAccumulateGroupInfo = .{ .GroupId = 1, .CurGachaNum = 0 },
        },
    });
}

pub fn onGachaAccumulateRewardRequest(
    txn: *Transaction(pb.GachaAccumulateRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var dirty = false;
    var kbuf: [48]u8 = undefined;
    for (txn.message.RewardId.items) |rid| {
        const k = try std.fmt.bufPrint(&kbuf, "gacha_acc:{d}:{d}", .{ txn.message.GachaAccumulateId, rid });
        if (try progress_comp.info.markClaim(alloc.gpa, k)) {
            dirty = true;
            _ = try RewardService.grantItems(reward_ctx, 0, true, &.{.{ .item_id = 3, .count = 160 }}, 19500);
        }
    }
    if (dirty) try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onTotalTopUpScoreInfoRequest(
    txn: *Transaction(pb.TotalTopUpScoreInfoRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    const score = @as(i32, @intCast(@max(0, progress_comp.info.getKeyInt("topup:score"))));
    var infos: std.ArrayList(pb.TotalTopUpScoreInfo) = .empty;
    try infos.append(alloc.arena, .{ .Id = 1, .Score = score });
    txn.respond(.{ .ErrorCode = .Success, .Infos = infos });
}

pub fn onTotalTopUpRewardRequest(
    txn: *Transaction(pb.TotalTopUpRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "topup:clm:{d}", .{txn.message.TotalTopUpRewardId});
    if (try progress_comp.info.markClaim(alloc.gpa, k)) {
        try progress_comp.save(fs, alloc.arena);
        _ = try RewardService.grantItems(reward_ctx, 0, true, &.{.{ .item_id = 3, .count = 300 }}, 19500);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onInfluenceInfoRequest(
    txn: *Transaction(pb.InfluenceInfoRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    var infos: std.ArrayList(pb.InfluenceInfo) = .empty;
    for (progress_comp.info.dynamic.ints) |entry| {
        if (!std.mem.startsWith(u8, entry.key, "inf:idx:")) continue;
        const inf_id = std.fmt.parseInt(i32, entry.key["inf:idx:".len..], 10) catch continue;
        try infos.append(alloc.arena, .{
            .InfluenceId = inf_id,
            .RewardIndex = @intCast(@max(0, entry.val)),
            .Relation = @intCast(@max(0, entry.val)),
        });
    }
    txn.respond(.{ .InfluenceInfos = infos });
}

pub fn onInfluenceRewardRequest(
    txn: *Transaction(pb.InfluenceRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var kbuf: [48]u8 = undefined;
    const idx_k = try std.fmt.bufPrint(&kbuf, "inf:idx:{d}", .{txn.message.InfluenceId});
    const next_idx = @as(i32, @intCast(try progress_comp.info.addKeyInt(alloc.gpa, idx_k, 1)));
    try progress_comp.save(fs, alloc.arena);
    _ = try RewardService.grantItems(reward_ctx, 0, true, &.{
        .{ .item_id = 3, .count = 50 },
        .{ .item_id = 2, .count = 15000 },
    }, 19500);
    var items: std.ArrayList(pb.MapEntry(i32, i32)) = .empty;
    try items.append(alloc.arena, .{ .key = 3, .value = 50 });
    txn.respond(.{
        .Code = .Success,
        .InfluenceId = txn.message.InfluenceId,
        .RewardIndex = next_idx,
        .RewardItems = items,
    });
}

pub fn onStorageInfoRequest(
    txn: *Transaction(pb.StorageInfoRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    var infos: std.ArrayList(pb.ClientStorageInfo) = .empty;
    var kbuf: [48]u8 = undefined;
    var seen_sys: std.AutoHashMapUnmanaged(i32, void) = .empty;
    for (progress_comp.info.dynamic.strs) |entry| {
        if (std.mem.startsWith(u8, entry.key, "cstorage:msg:")) {
            const sys_id = std.fmt.parseInt(i32, entry.key["cstorage:msg:".len..], 10) catch continue;
            if (pb_kv.loadMsg(alloc.arena, progress_comp, entry.key, pb.ClientStorageInfo)) |msg| {
                try seen_sys.put(alloc.arena, sys_id, {});
                try infos.append(alloc.arena, msg);
            }
        }
    }
    for (progress_comp.info.dynamic.ints) |entry| {
        if (!std.mem.startsWith(u8, entry.key, "cstorage:i:")) continue;
        const sys_id = std.fmt.parseInt(i32, entry.key["cstorage:i:".len..], 10) catch continue;
        if (seen_sys.contains(sys_id)) continue;
        try seen_sys.put(alloc.arena, sys_id, {});
        try infos.append(alloc.arena, .{
            .SystemId = sys_id,
            .Data = .{ .LongData = .{ .Data = entry.val } },
        });
    }
    for (progress_comp.info.dynamic.strs) |entry| {
        if (!std.mem.startsWith(u8, entry.key, "cstorage:s:")) continue;
        const sys_id = std.fmt.parseInt(i32, entry.key["cstorage:s:".len..], 10) catch continue;
        if (seen_sys.contains(sys_id)) continue;
        try seen_sys.put(alloc.arena, sys_id, {});
        try infos.append(alloc.arena, .{
            .SystemId = sys_id,
            .Data = .{ .StringData = .{ .Data = try alloc.arena.dupe(u8, entry.val) } },
        });
    }
    _ = &kbuf;
    txn.respond(.{ .Infos = infos });
}

pub fn onStorageInfoUpdateRequest(
    txn: *Transaction(pb.StorageInfoUpdateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var dirty = false;
    var kbuf: [48]u8 = undefined;
    for (txn.message.Infos.items) |info| {
        const data = info.Data orelse continue;
        const mkey = try std.fmt.bufPrint(&kbuf, "cstorage:msg:{d}", .{info.SystemId});
        try pb_kv.saveMsg(alloc, fs, progress_comp, mkey, info);
        dirty = true;
        switch (data) {
            .IntData => |v_opt| if (v_opt) |v| {
                const k = try std.fmt.bufPrint(&kbuf, "cstorage:i:{d}", .{info.SystemId});
                try progress_comp.info.setKeyInt(alloc.gpa, k, v.Data);
            },
            .LongData => |v_opt| if (v_opt) |v| {
                const k = try std.fmt.bufPrint(&kbuf, "cstorage:i:{d}", .{info.SystemId});
                try progress_comp.info.setKeyInt(alloc.gpa, k, v.Data);
            },
            .BoolData => |v_opt| if (v_opt) |v| {
                const k = try std.fmt.bufPrint(&kbuf, "cstorage:i:{d}", .{info.SystemId});
                try progress_comp.info.setKeyInt(alloc.gpa, k, if (v.Data) 1 else 0);
            },
            .StringData => |v_opt| if (v_opt) |v| {
                const k = try std.fmt.bufPrint(&kbuf, "cstorage:s:{d}", .{info.SystemId});
                try progress_comp.info.setKeyStr(alloc.gpa, k, v.Data);
            },
            else => {},
        }
    }
    if (dirty) try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}
