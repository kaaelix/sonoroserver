const std = @import("std");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const FileSystem = @import("common").FileSystem;
const MailInfo = @import("../../fs/MailInfo.zig");
const PlayerMailComponent = @import("../../logic/component/player/PlayerMailComponent.zig");
const RewardService = @import("../../logic/helpers/reward.zig");
const EnergyHelper = @import("../../logic/helpers/energy.zig");
const QuestEngine = @import("../../logic/helpers/quest_engine.zig");
const Transaction = @import("../handlers.zig").Transaction;

pub fn onMailBindInfoRequest(
    txn: *Transaction(pb.MailBindInfoRequest),
    mail_comp: *const PlayerMailComponent,
) !void {
    txn.respond(.{
        .MailBind = .{
            .IsBind = mail_comp.info.bind_rewarded,
            .IsReward = mail_comp.info.bind_rewarded,
            .CloseTime = 0,
        },
    });
}

pub fn onMailReadRequest(
    txn: *Transaction(pb.MailReadRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    io: std.Io,
    mail_comp: *PlayerMailComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    const now_sec = EnergyHelper.nowSeconds(io);
    const pruned = try mail_comp.info.pruneExpired(alloc.gpa, now_sec);

    const entry = mail_comp.info.findPtr(txn.message.Id) orelse {
        if (pruned) try mail_comp.save(alloc.arena, fs);
        txn.respond(.{
            .Id = txn.message.Id,
            .ErrorCode = .ErrMailNotExist,
        });
        return;
    };

    var dirty = pruned;
    if ((entry.state & MailInfo.State.read) == 0) {
        entry.state |= MailInfo.State.read;
        dirty = true;
    }
    if (entry.read_time == 0) {
        entry.read_time = now_sec;
        dirty = true;
    }
    if (dirty) {
        try mail_comp.save(alloc.arena, fs);
    }

    try QuestEngine.onMailRead(qctx, entry.id);

    txn.respond(.{
        .ExpiryTime = entry.expiry_time,
        .Id = try alloc.arena.dupe(u8, entry.id),
        .State = MailInfo.wireState(entry.*),
        .ReadTime = entry.read_time,
        .ErrorCode = .Success,
    });
}

pub fn onMailGetAttachmentRequest(
    txn: *Transaction(pb.MailGetAttachmentRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    io: std.Io,
    mail_comp: *PlayerMailComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const now_sec = EnergyHelper.nowSeconds(io);
    var dirty = try mail_comp.info.pruneExpired(alloc.gpa, now_sec);
    var success_map: std.ArrayList(pb.MapEntry([]const u8, i32)) = .empty;
    var grants: std.ArrayList(RewardService.RewardItem) = .empty;

    for (txn.message.MailIds.items) |mail_id| {
        const entry = mail_comp.info.findPtr(mail_id) orelse continue;
        if (MailInfo.isClaimed(entry.*)) {
            try success_map.append(alloc.arena, .{ .key = try alloc.arena.dupe(u8, entry.id), .value = 0 });
            continue;
        }
        for (entry.attachments) |att| {
            if (att.item_id > 0 and att.count > 0) {
                try grants.append(alloc.arena, .{ .item_id = att.item_id, .count = att.count });
            }
        }
        entry.state |= MailInfo.State.read | MailInfo.State.claimed;
        if (entry.read_time == 0) entry.read_time = now_sec;
        dirty = true;
        try success_map.append(alloc.arena, .{ .key = try alloc.arena.dupe(u8, entry.id), .value = 0 });
    }

    if (grants.items.len > 0) {
        _ = try RewardService.grantItems(reward_ctx, 0, true, grants.items, 7000);
    }
    if (dirty) {
        try mail_comp.save(alloc.arena, fs);
        try txn.conn.push(alloc.gpa, io, try mail_comp.info.toProtoNotify(alloc.arena));
    }

    txn.respond(.{
        .ErrorCode = .Success,
        .SuccessIdMap = success_map,
    });
}

pub fn onMailDeleteRequest(
    txn: *Transaction(pb.MailDeleteRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    io: std.Io,
    mail_comp: *PlayerMailComponent,
) !void {
    const now_sec = EnergyHelper.nowSeconds(io);
    var dirty = try mail_comp.info.pruneExpired(alloc.gpa, now_sec);
    var success_ids: std.ArrayList([]const u8) = .empty;

    for (txn.message.MailIds.items) |mail_id| {
        if (mail_comp.info.findPtr(mail_id)) |entry| {
            if (MailInfo.isFavorite(entry.*)) continue;
            if (entry.attachments.len > 0 and !MailInfo.isClaimed(entry.*)) continue;
        }
        if (try mail_comp.info.removeMail(alloc.gpa, mail_id)) {
            dirty = true;
            try success_ids.append(alloc.arena, try alloc.arena.dupe(u8, mail_id));
        }
    }
    if (dirty) {
        try mail_comp.save(alloc.arena, fs);
    }

    txn.respond(.{
        .ErrorCode = .Success,
        .SuccessIds = success_ids,
    });
}

pub fn onMailFavoriteRequest(
    txn: *Transaction(pb.MailFavoriteRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    mail_comp: *PlayerMailComponent,
) !void {
    const entry = mail_comp.info.findPtr(txn.message.Id) orelse {
        txn.respond(.{
            .Id = txn.message.Id,
            .ErrorCode = .ErrMailNotExist,
        });
        return;
    };
    entry.is_favorite = txn.message.IsFavorite;
    if (txn.message.IsFavorite) {
        entry.state |= MailInfo.State.favorite;
    } else {
        entry.state &= ~MailInfo.State.favorite;
    }
    try mail_comp.save(alloc.arena, fs);

    txn.respond(.{
        .ErrorCode = .Success,
        .Id = try alloc.arena.dupe(u8, entry.id),
        .State = MailInfo.wireState(entry.*),
        .ExpiryTime = entry.expiry_time,
    });
}

pub fn onMailBindRequest(
    txn: *Transaction(pb.MailBindRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    mail_comp: *PlayerMailComponent,
) !void {
    if (!mail_comp.info.bind_rewarded) {
        mail_comp.info.bind_rewarded = true;
        try mail_comp.save(alloc.arena, fs);
    }
    txn.respond(.{});
}

pub fn onMailBindRewardRequest(
    txn: *Transaction(pb.MailBindRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    mail_comp: *PlayerMailComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    if (!mail_comp.info.bind_rewarded) {
        mail_comp.info.bind_rewarded = true;
        try mail_comp.save(alloc.arena, fs);
        _ = try RewardService.grantItems(reward_ctx, 0, true, &.{
            .{ .item_id = 3, .count = 100 },
            .{ .item_id = 2, .count = 10000 },
        }, 7000);
    }
    txn.respond(.{ .ErrorCode = .Success });
}
