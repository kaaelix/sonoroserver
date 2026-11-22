const std = @import("std");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const common = @import("common");
const FileSystem = common.FileSystem;
const Transaction = @import("../handlers.zig").Transaction;
const PlayerProgressComponent = @import("../../logic/component/player/PlayerProgressComponent.zig");
const RewardService = @import("../../logic/helpers/reward.zig");

fn formatIdsCsv(ids: []const i32, arena: std.mem.Allocator) ![]const u8 {
    var out: std.ArrayList(u8) = .empty;
    for (ids, 0..) |id, idx| {
        if (idx > 0) try out.append(arena, ',');
        try out.print(arena, "{d}", .{id});
    }
    return out.items;
}

fn parseIdsCsv(csv: []const u8, arena: std.mem.Allocator) !std.ArrayList(i32) {
    var list: std.ArrayList(i32) = .empty;
    if (csv.len == 0) return list;
    var it = std.mem.splitScalar(u8, csv, ',');
    while (it.next()) |tok| {
        if (tok.len == 0) continue;
        if (std.fmt.parseInt(i32, tok, 10)) |val| {
            try list.append(arena, val);
        } else |_| {}
    }
    return list;
}

pub fn onPhantomBattleCardGroupAddRequest(
    txn: *Transaction(pb.PhantomBattleCardGroupAddRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const cnt_key = try std.fmt.allocPrint(alloc.arena, "pb_deck_cnt:{d}", .{txn.message.ActivityId});
    const idx_val = progress_comp.info.getKeyInt(cnt_key);
    const idx: i32 = @intCast(idx_val);
    try progress_comp.info.setKeyInt(alloc.gpa, cnt_key, idx + 1);

    const nkey = try std.fmt.allocPrint(alloc.arena, "pb_deck:{d}:{d}:name", .{ txn.message.ActivityId, idx });
    const ckey = try std.fmt.allocPrint(alloc.arena, "pb_deck:{d}:{d}:cards", .{ txn.message.ActivityId, idx });
    try progress_comp.info.setKeyStr(alloc.gpa, nkey, txn.message.Name);
    try progress_comp.info.setKeyStr(alloc.gpa, ckey, try formatIdsCsv(txn.message.EquipCardIds.items, alloc.arena));
    try progress_comp.save(fs, alloc.arena);

    var cards: std.ArrayList(i32) = .empty;
    try cards.appendSlice(alloc.arena, txn.message.EquipCardIds.items);
    txn.respond(.{
        .ErrorCode = .Success,
        .PhantomBattleCardGroupInfo = .{
            .Name = txn.message.Name,
            .EquipCardIds = cards,
            .CanUse = true,
            .Index = idx,
        },
    });
}

pub fn onPhantomBattleCardGroupUpdateRequest(
    txn: *Transaction(pb.PhantomBattleCardGroupUpdateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const nkey = try std.fmt.allocPrint(alloc.arena, "pb_deck:{d}:{d}:name", .{ txn.message.ActivityId, txn.message.Index });
    const ckey = try std.fmt.allocPrint(alloc.arena, "pb_deck:{d}:{d}:cards", .{ txn.message.ActivityId, txn.message.Index });
    try progress_comp.info.setKeyStr(alloc.gpa, ckey, try formatIdsCsv(txn.message.EquipCardIds.items, alloc.arena));
    try progress_comp.save(fs, alloc.arena);

    var cards: std.ArrayList(i32) = .empty;
    try cards.appendSlice(alloc.arena, txn.message.EquipCardIds.items);
    txn.respond(.{
        .ErrorCode = .Success,
        .PhantomBattleCardGroupInfo = .{
            .Name = progress_comp.info.getKeyStr(nkey),
            .EquipCardIds = cards,
            .CanUse = true,
            .Index = txn.message.Index,
        },
    });
}

pub fn onPhantomBattleCardGroupNameRequest(
    txn: *Transaction(pb.PhantomBattleCardGroupNameRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const nkey = try std.fmt.allocPrint(alloc.arena, "pb_deck:{d}:{d}:name", .{ txn.message.ActivityId, txn.message.Index });
    const ckey = try std.fmt.allocPrint(alloc.arena, "pb_deck:{d}:{d}:cards", .{ txn.message.ActivityId, txn.message.Index });
    try progress_comp.info.setKeyStr(alloc.gpa, nkey, txn.message.Name);
    try progress_comp.save(fs, alloc.arena);

    txn.respond(.{
        .ErrorCode = .Success,
        .PhantomBattleCardGroupInfo = .{
            .Name = txn.message.Name,
            .EquipCardIds = try parseIdsCsv(progress_comp.info.getKeyStr(ckey), alloc.arena),
            .CanUse = true,
            .Index = txn.message.Index,
        },
    });
}

pub fn onPhantomBattleCardGroupDeleteRequest(
    txn: *Transaction(pb.PhantomBattleCardGroupDeleteRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const nkey = try std.fmt.allocPrint(alloc.arena, "pb_deck:{d}:{d}:name", .{ txn.message.ActivityId, txn.message.Index });
    const ckey = try std.fmt.allocPrint(alloc.arena, "pb_deck:{d}:{d}:cards", .{ txn.message.ActivityId, txn.message.Index });
    try progress_comp.info.setKeyStr(alloc.gpa, nkey, "");
    try progress_comp.info.setKeyStr(alloc.gpa, ckey, "");

    const cnt_key = try std.fmt.allocPrint(alloc.arena, "pb_deck_cnt:{d}", .{txn.message.ActivityId});
    const cnt: i32 = @intCast(progress_comp.info.getKeyInt(cnt_key));
    var list: std.ArrayList(pb.PhantomBattleCardGroupInfo) = .empty;
    var i: i32 = 0;
    while (i < cnt) : (i += 1) {
        const ik = try std.fmt.allocPrint(alloc.arena, "pb_deck:{d}:{d}:cards", .{ txn.message.ActivityId, i });
        const csv = progress_comp.info.getKeyStr(ik);
        if (csv.len == 0) continue;
        const ink = try std.fmt.allocPrint(alloc.arena, "pb_deck:{d}:{d}:name", .{ txn.message.ActivityId, i });
        try list.append(alloc.arena, .{
            .Name = progress_comp.info.getKeyStr(ink),
            .EquipCardIds = try parseIdsCsv(csv, alloc.arena),
            .CanUse = true,
            .Index = i,
        });
    }
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .ErrorCode = .Success,
        .LastUseCardGroupIndex = 0,
        .PhantomBattleCardGroupInfo = list,
    });
}

pub fn onPhantomBattleCardUnlockRequest(
    txn: *Transaction(pb.PhantomBattleCardUnlockRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "pb_card:{d}", .{txn.message.CardId});
    _ = try progress_comp.info.markClaim(alloc.gpa, key);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onPhantomBattleCardOutLookUpRequest(
    txn: *Transaction(pb.PhantomBattleCardOutLookUpRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "pb:card_outlook:{d}", .{txn.message.CardId});
    _ = try progress_comp.info.markClaim(alloc.gpa, key);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onPhantomBattleCheckCardSkillUnlockRequest(
    txn: *Transaction(pb.PhantomBattleCheckCardSkillUnlockRequest),
    alloc: mem.Alloc,
) !void {
    var cfgs: std.ArrayList(i32) = .empty;
    try cfgs.appendSlice(alloc.arena, txn.message.CardConfig.items);
    txn.respond(.{
        .IsUnlock = true,
        .CardConfig = cfgs,
        .ErrorCode = .Success,
        .CurNum = @intCast(txn.message.CardConfig.items.len),
        .TargetNum = @intCast(txn.message.CardConfig.items.len),
    });
}

pub fn onPhantomBattleStartRequest(
    txn: *Transaction(pb.PhantomBattleStartRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "pb:round", 1);
    try progress_comp.info.setKeyInt(alloc.gpa, "pb:lib_num", 20);
    try progress_comp.info.setKeyInt(alloc.gpa, "pb:hand_num", 5);
    const active_ch = progress_comp.info.getKeyInt("pb:challenge");
    if (active_ch > 0) {
        const ck = try std.fmt.allocPrint(alloc.arena, "pb:challenge:{d}", .{active_ch});
        _ = try progress_comp.info.markClaim(alloc.gpa, ck);
    }
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onPhantomBattleBvBLoadingFinishRequest(
    txn: *Transaction(pb.PhantomBattleBvBLoadingFinishRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    try progress_comp.info.setKeyInt(alloc.gpa, "pb:bvb_loaded", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onPhantomBattleNpcShowOverRequest(
    txn: *Transaction(pb.PhantomBattleNpcShowOverRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    try progress_comp.info.setKeyInt(alloc.gpa, "pb:npc_shown", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onPhantomBattleReChallengeRequest(
    txn: *Transaction(pb.PhantomBattleReChallengeRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "pb:challenge", txn.message.ChallengeId);
    try progress_comp.info.setKeyInt(alloc.gpa, "pb:round", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .ChallengeId = txn.message.ChallengeId,
        .CardGroupId = 0,
        .ErrorCode = .Success,
    });
}

pub fn onPhantomBattleRoundOverRequest(
    txn: *Transaction(pb.PhantomBattleRoundOverRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "pb:round", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onPhantomBattleEnterSlotRequest(
    txn: *Transaction(pb.PhantomBattleEnterSlotRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "pb:slot:{d}", .{txn.message.SlotIndex});
    try progress_comp.info.setKeyInt(alloc.gpa, key, txn.message.CardUid);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .ErrorCode = .Success,
        .PhantomBattleFighterInfo = .{ .UId = txn.message.CardUid },
    });
}

pub fn onPhantomBattleLeaveSlotRequest(
    txn: *Transaction(pb.PhantomBattleLeaveSlotRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "pb:slot:{d}", .{txn.message.SlotIndex});
    const uid: i32 = @intCast(progress_comp.info.getKeyInt(key));
    try progress_comp.info.setKeyInt(alloc.gpa, key, 0);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .ErrorCode = .Success,
        .PhantomBattleHandCardInfo = .{ .CardUid = uid, .CanUse = true },
    });
}

pub fn onPhantomBattleSlotInsteadRequest(
    txn: *Transaction(pb.PhantomBattleSlotInsteadRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const skey = try std.fmt.allocPrint(alloc.arena, "pb:slot:{d}", .{txn.message.StartSlotIndex});
    const tkey = try std.fmt.allocPrint(alloc.arena, "pb:slot:{d}", .{txn.message.TargetSlotIndex});
    const sv = progress_comp.info.getKeyInt(skey);
    const tv = progress_comp.info.getKeyInt(tkey);
    try progress_comp.info.setKeyInt(alloc.gpa, skey, tv);
    try progress_comp.info.setKeyInt(alloc.gpa, tkey, sv);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .StartSlotIndex = txn.message.StartSlotIndex,
        .TargetSlotIndex = txn.message.TargetSlotIndex,
        .ErrorCode = .Success,
    });
}

pub fn onPhantomBattleEvolveRequest(
    txn: *Transaction(pb.PhantomBattleEvolveRequest),
    progress_comp: *const PlayerProgressComponent,
) !void {
    txn.respond(.{
        .ErrorCode = .Success,
        .PhantomBattleFighterInfo = .{ .UId = txn.message.CardUid },
        .CardLibraryNum = @intCast(@max(0, progress_comp.info.getKeyInt("pb:lib_num"))),
    });
}

pub fn onPhantomBattleBackLibraryRequest(
    txn: *Transaction(pb.PhantomBattleBackLibraryRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const lib_num = try progress_comp.info.addKeyInt(alloc.gpa, "pb:lib_num", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .CardUid = txn.message.CardUid,
        .CurCardLibraryNum = @intCast(lib_num),
        .ErrorCode = .Success,
    });
}

pub fn onPhantomBattleBackSlotCardLibraryRequest(
    txn: *Transaction(pb.PhantomBattleBackSlotCardLibraryRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const lib_num = try progress_comp.info.addKeyInt(alloc.gpa, "pb:lib_num", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .CardUid = txn.message.CardUid,
        .CurCardLibraryNum = @intCast(lib_num),
        .ErrorCode = .Success,
    });
}

pub fn onPhantomBattleCallCardRequest(
    txn: *Transaction(pb.PhantomBattleCallCardRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    var fighters: std.ArrayList(pb.PhantomBattleFighterInfo) = .empty;
    for (txn.message.SelectedCardUid.items) |uid| {
        try fighters.append(alloc.arena, .{ .UId = uid });
    }
    txn.respond(.{
        .PhantomBattleFighterInfo = fighters,
        .EffectUid = txn.message.EffectUid,
        .HeapNum = @intCast(@max(0, progress_comp.info.getKeyInt("pb:lib_num"))),
        .ErrorCode = .Success,
    });
}

pub fn onPhantomBattleReconstructCardRequest(
    txn: *Transaction(pb.PhantomBattleReconstructCardRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    var sel: std.ArrayList(i32) = .empty;
    try sel.appendSlice(alloc.arena, txn.message.SelectedCardUid.items);
    txn.respond(.{
        .ErrorCode = .Success,
        .HeapNum = @intCast(@max(0, progress_comp.info.getKeyInt("pb:lib_num"))),
        .EffectUid = txn.message.EffectUid,
        .SelectedCardUid = sel,
    });
}

pub fn onPhantomBattleCardSelectRequest(
    txn: *Transaction(pb.PhantomBattleCardSelectRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    var hand: std.ArrayList(pb.PhantomBattleHandCardInfo) = .empty;
    for (txn.message.CardUid.items) |uid| {
        try hand.append(alloc.arena, .{ .CardUid = uid, .CanUse = true });
    }
    txn.respond(.{
        .Error = .Success,
        .PhantomBattleHandCardInfos = hand,
        .CurCardLibraryNum = @intCast(@max(0, progress_comp.info.getKeyInt("pb:lib_num"))),
    });
}

pub fn onPhantomBattleDealCardReplaceRequest(
    txn: *Transaction(pb.PhantomBattleDealCardReplaceRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    var hand: std.ArrayList(pb.PhantomBattleHandCardInfo) = .empty;
    for (txn.message.ReplaceCardUid.items) |uid| {
        try hand.append(alloc.arena, .{ .CardUid = uid, .CanUse = true });
    }
    txn.respond(.{
        .CurCardLibraryNum = @intCast(@max(0, progress_comp.info.getKeyInt("pb:lib_num"))),
        .ErrorCode = .Success,
        .PhantomBattleHandCardInfos = hand,
    });
}

pub fn onPhantomBattleDiscardCardRequest(
    txn: *Transaction(pb.PhantomBattleDiscardCardRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    var discards: std.ArrayList(i32) = .empty;
    try discards.appendSlice(alloc.arena, txn.message.DiscardCardIds.items);
    txn.respond(.{
        .DiscardCardIds = discards,
        .HandCardNum = @intCast(@max(0, progress_comp.info.getKeyInt("pb:hand_num"))),
        .ErrorCode = .Success,
        .CurCardLibraryNum = @intCast(@max(0, progress_comp.info.getKeyInt("pb:lib_num"))),
    });
}

pub fn onPhantomBattleSelectReserveCardRequest(
    txn: *Transaction(pb.PhantomBattleSelectReserveCardRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    var res: std.ArrayList(i32) = .empty;
    try res.appendSlice(alloc.arena, txn.message.ReserveCardUIds.items);
    txn.respond(.{
        .ReserveCardUIds = res,
        .CurCardLibraryNum = @intCast(@max(0, progress_comp.info.getKeyInt("pb:lib_num"))),
        .ErrorCode = .Success,
        .HandCardNum = @intCast(@max(0, progress_comp.info.getKeyInt("pb:hand_num"))),
    });
}

pub fn onPhantomBattleCardSkillRequest(
    txn: *Transaction(pb.PhantomBattleCardSkillRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    var sel: std.ArrayList(i32) = .empty;
    try sel.appendSlice(alloc.arena, txn.message.SelectFighterUId.items);
    txn.respond(.{
        .CardUId = txn.message.CardUId,
        .ErrorCode = .Success,
        .LibraryCount = @intCast(@max(0, progress_comp.info.getKeyInt("pb:lib_num"))),
        .PhantomBattleSkillTriggerInfo = .{
            .SourceFighterUId = txn.message.CardUId,
            .SelectFighterUId = sel,
        },
    });
}

pub fn onPhantomBattleSlotCardSkillRequest(
    txn: *Transaction(pb.PhantomBattleSlotCardSkillRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    var sel: std.ArrayList(i32) = .empty;
    try sel.appendSlice(alloc.arena, txn.message.SelectFighterUId.items);
    txn.respond(.{
        .ErrorCode = .Success,
        .LibraryCount = @intCast(@max(0, progress_comp.info.getKeyInt("pb:lib_num"))),
        .CardUId = txn.message.CardUId,
        .SkillId = txn.message.SkillId,
        .PhantomBattleSkillTriggerInfo = .{
            .SourceFighterUId = txn.message.CardUId,
            .SkillId = txn.message.SkillId,
            .SelectFighterUId = sel,
        },
    });
}

pub fn onPhantomBattleCardRoleSkillRequest(
    txn: *Transaction(pb.PhantomBattleCardRoleSkillRequest),
    alloc: mem.Alloc,
) !void {
    var sel: std.ArrayList(i32) = .empty;
    try sel.appendSlice(alloc.arena, txn.message.SelectFighterUId.items);
    txn.respond(.{
        .ErrorCode = .Success,
        .PhantomBattleSkillTriggerInfo = .{
            .SkillId = txn.message.SkillId,
            .SelectFighterUId = sel,
        },
    });
}

pub fn onPhantomBattleCardTargetInfoRequest(
    txn: *Transaction(pb.PhantomBattleCardTargetInfoRequest),
    alloc: mem.Alloc,
) !void {
    var uids: std.ArrayList(i32) = .empty;
    if (txn.message.CardUId > 0) try uids.append(alloc.arena, txn.message.CardUId);
    txn.respond(.{
        .CardUId = txn.message.CardUId,
        .EffectType = 1,
        .SkillId = txn.message.SkillId,
        .FighterUId = uids,
        .Num = @intCast(@max(1, uids.items.len)),
        .ErrorCode = .Success,
    });
}

pub fn onPhantomBattleCardRoleTargetInfoRequest(
    txn: *Transaction(pb.PhantomBattleCardRoleTargetInfoRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    var uids: std.ArrayList(i32) = .empty;
    const ch = @as(i32, @intCast(@max(0, progress_comp.info.getKeyInt("pb:challenge"))));
    if (ch > 0) try uids.append(alloc.arena, ch);
    txn.respond(.{
        .EffectType = 1,
        .SkillId = txn.message.SkillId,
        .FighterUId = uids,
        .Num = @intCast(@max(1, uids.items.len)),
        .ErrorCode = .Success,
    });
}

pub fn onPhantomBattleSelectTargetRequest(
    txn: *Transaction(pb.PhantomBattleSelectTargetRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const first_uid: i64 = if (txn.message.FighterUId.items.len > 0) txn.message.FighterUId.items[0] else 0;
    try progress_comp.info.setKeyInt(alloc.gpa, "pb:target_uid", first_uid);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

fn claimPbRewardIds(
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
    prefix: []const u8,
    table: []const u8,
    ids: []const i32,
) !void {
    for (ids) |id| {
        const key = try std.fmt.allocPrint(alloc.arena, "{s}:{d}", .{ prefix, id });
        if (try progress_comp.info.markClaim(alloc.gpa, key)) {
            const fallback = [_]RewardService.RewardItem{.{ .item_id = 3, .count = 40 }, .{ .item_id = 2, .count = 15000 }};
            _ = try RewardService.grantTableReward(reward_ctx, table, id, false, true, 1, &fallback, null);
        }
    }
    try progress_comp.save(fs, alloc.arena);
}

pub fn onPhantomBattleBadgeRewardRequest(txn: *Transaction(pb.PhantomBattleBadgeRewardRequest), alloc: mem.Alloc, fs: *FileSystem, progress_comp: *PlayerProgressComponent, reward_ctx: RewardService.RewardContext) !void {
    try claimPbRewardIds(alloc, fs, progress_comp, reward_ctx, "pb_badge", "PhantomBattleBadgeReward", txn.message.BadgeRewardId.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onPhantomBattleCardRewardRequest(txn: *Transaction(pb.PhantomBattleCardRewardRequest), alloc: mem.Alloc, fs: *FileSystem, progress_comp: *PlayerProgressComponent, reward_ctx: RewardService.RewardContext) !void {
    try claimPbRewardIds(alloc, fs, progress_comp, reward_ctx, "pb_card_rw", "PhantomBattleCardReward", txn.message.CardRewardId.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onPhantomBattleGuideRewardRequest(txn: *Transaction(pb.PhantomBattleGuideRewardRequest), alloc: mem.Alloc, fs: *FileSystem, progress_comp: *PlayerProgressComponent, reward_ctx: RewardService.RewardContext) !void {
    const ids = [_]i32{1};
    try claimPbRewardIds(alloc, fs, progress_comp, reward_ctx, "pb_guide_rw", "PhantomBattleCardReward", &ids);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onPhantomBattleTaskRewardRequest(txn: *Transaction(pb.PhantomBattleTaskRewardRequest), alloc: mem.Alloc, fs: *FileSystem, progress_comp: *PlayerProgressComponent, reward_ctx: RewardService.RewardContext) !void {
    const ids = [_]i32{txn.message.TaskId};
    try claimPbRewardIds(alloc, fs, progress_comp, reward_ctx, "pb_task", "PhantomBattleTask", &ids);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onPhantomBattleRepeatedTaskRewardRequest(txn: *Transaction(pb.PhantomBattleRepeatedTaskRewardRequest), alloc: mem.Alloc, fs: *FileSystem, progress_comp: *PlayerProgressComponent, reward_ctx: RewardService.RewardContext) !void {
    try claimPbRewardIds(alloc, fs, progress_comp, reward_ctx, "pb_rtask", "PhantomBattleTask", txn.message.TaskId.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onPhantomBattleRoleRewardRequest(txn: *Transaction(pb.PhantomBattleRoleRewardRequest), alloc: mem.Alloc, fs: *FileSystem, progress_comp: *PlayerProgressComponent, reward_ctx: RewardService.RewardContext) !void {
    const ids = [_]i32{txn.message.PhantomBattleRoleId};
    try claimPbRewardIds(alloc, fs, progress_comp, reward_ctx, "pb_role_rw", "PhantomBattleCardRole", &ids);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onMulPhantomBattleMasterLevelRewardRequest(
    txn: *Transaction(pb.MulPhantomBattleMasterLevelRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    try claimPbRewardIds(alloc, fs, progress_comp, reward_ctx, "pb_master", "PhantomBattleMasterLevel", txn.message.PhantomBattleMasterLevelId.items);
    var out: std.ArrayList(i32) = .empty;
    try out.appendSlice(alloc.arena, txn.message.PhantomBattleMasterLevelId.items);
    txn.respond(.{
        .PhantomBattleMasterLevelId = out,
        .ErrorCode = .Success,
    });
}
