const MailInfo = @This();
const std = @import("std");
const pb = @import("proto").pb;

const Allocator = std.mem.Allocator;

pub const default: MailInfo = .{};
pub const data_path = "mail_info";

pub const State = struct {
    pub const unread: i32 = 0;
    pub const read: i32 = 1;
    pub const claimed: i32 = 2;
    pub const favorite: i32 = 4;
};

pub const Attachment = struct {
    item_id: i32,
    count: i32,
};

pub const MailEntry = struct {
    id: []const u8,
    config_id: i32 = 0,
    title: []const u8 = "",
    content: []const u8 = "",
    sender: []const u8 = "",
    level: i32 = 1,
    received_time: i64 = 0,
    read_time: i64 = 0,
    state: i32 = 0,
    valid_time: i32 = 2_592_000,
    read_valid_time: i32 = 2_592_000,
    expiry_time: i64 = 0,
    is_favorite: bool = false,
    attachments: []Attachment = &.{},
};

mails: []MailEntry = &.{},
bind_rewarded: bool = false,

pub fn deinit(info: MailInfo, gpa: Allocator) void {
    std.zon.parse.free(gpa, info);
}

pub fn findPtr(info: *MailInfo, id: []const u8) ?*MailEntry {
    for (info.mails) |*mail| {
        if (std.mem.eql(u8, mail.id, id)) return mail;
    }
    return null;
}

pub fn upsertNpcMail(info: *MailInfo, gpa: Allocator, mail_id: i32, now_sec: i64) !*MailEntry {
    var id_buf: [32]u8 = undefined;
    const id_str = try std.fmt.bufPrint(&id_buf, "{d}", .{mail_id});

    for (info.mails) |*mail| {
        if (mail.config_id == mail_id or std.mem.eql(u8, mail.id, id_str)) {
            return mail;
        }
    }

    const dup_id = try gpa.dupe(u8, id_str);
    errdefer gpa.free(dup_id);

    const next = try gpa.alloc(MailEntry, info.mails.len + 1);
    @memcpy(next[0..info.mails.len], info.mails);
    next[info.mails.len] = .{
        .id = dup_id,
        .config_id = mail_id,
        .received_time = now_sec,
        .state = State.unread,
        .valid_time = 2_592_000,
        .read_valid_time = 2_592_000,
        .expiry_time = now_sec + 2_592_000,
    };
    if (info.mails.len != 0) gpa.free(info.mails);
    info.mails = next;
    return &info.mails[info.mails.len - 1];
}

pub fn addMail(
    info: *MailInfo,
    gpa: Allocator,
    id: []const u8,
    config_id: i32,
    title: []const u8,
    content: []const u8,
    sender: []const u8,
    attachments: []const Attachment,
    now_sec: i64,
    valid_seconds: i32,
) !*MailEntry {
    if (findPtr(info, id)) |existing| return existing;

    const dup_id = try gpa.dupe(u8, id);
    errdefer gpa.free(dup_id);
    const dup_title = try gpa.dupe(u8, title);
    errdefer gpa.free(dup_title);
    const dup_content = try gpa.dupe(u8, content);
    errdefer gpa.free(dup_content);
    const dup_sender = try gpa.dupe(u8, sender);
    errdefer gpa.free(dup_sender);
    const dup_att = if (attachments.len != 0) try gpa.dupe(Attachment, attachments) else &.{};
    errdefer if (dup_att.len != 0) gpa.free(dup_att);

    const ttl: i32 = if (valid_seconds > 0) valid_seconds else 2_592_000;
    const next = try gpa.alloc(MailEntry, info.mails.len + 1);
    @memcpy(next[0..info.mails.len], info.mails);
    next[info.mails.len] = .{
        .id = dup_id,
        .config_id = config_id,
        .title = dup_title,
        .content = dup_content,
        .sender = dup_sender,
        .received_time = now_sec,
        .state = State.unread,
        .valid_time = ttl,
        .read_valid_time = ttl,
        .expiry_time = now_sec + ttl,
        .attachments = dup_att,
    };
    if (info.mails.len != 0) gpa.free(info.mails);
    info.mails = next;
    return &info.mails[info.mails.len - 1];
}

fn freeEntry(gpa: Allocator, mail: MailEntry) void {
    if (mail.id.len != 0) gpa.free(mail.id);
    if (mail.title.len != 0) gpa.free(mail.title);
    if (mail.content.len != 0) gpa.free(mail.content);
    if (mail.sender.len != 0) gpa.free(mail.sender);
    if (mail.attachments.len != 0) gpa.free(mail.attachments);
}

pub fn pruneExpired(info: *MailInfo, gpa: Allocator, now_sec: i64) !bool {
    var expired_count: usize = 0;
    for (info.mails) |mail| {
        if (mail.expiry_time > 0 and mail.expiry_time <= now_sec) expired_count += 1;
    }
    if (expired_count == 0) return false;

    const keep_len = info.mails.len - expired_count;
    const kept: []MailEntry = if (keep_len != 0) try gpa.alloc(MailEntry, keep_len) else &.{};
    var idx: usize = 0;
    for (info.mails) |mail| {
        if (mail.expiry_time > 0 and mail.expiry_time <= now_sec) {
            freeEntry(gpa, mail);
            continue;
        }
        kept[idx] = mail;
        idx += 1;
    }
    if (info.mails.len != 0) gpa.free(info.mails);
    info.mails = kept;
    return true;
}

pub fn removeMail(info: *MailInfo, gpa: Allocator, id: []const u8) !bool {
    var found = false;
    for (info.mails) |mail| {
        if (std.mem.eql(u8, mail.id, id)) {
            found = true;
            break;
        }
    }
    if (!found) return false;

    const keep_len = info.mails.len - 1;
    const kept: []MailEntry = if (keep_len != 0) try gpa.alloc(MailEntry, keep_len) else &.{};
    var idx: usize = 0;
    for (info.mails) |mail| {
        if (std.mem.eql(u8, mail.id, id)) {
            freeEntry(gpa, mail);
            continue;
        }
        kept[idx] = mail;
        idx += 1;
    }
    if (info.mails.len != 0) gpa.free(info.mails);
    info.mails = kept;
    return true;
}

pub fn isRead(mail: MailEntry) bool {
    return (mail.state & State.read) != 0;
}

pub fn isClaimed(mail: MailEntry) bool {
    return (mail.state & State.claimed) != 0;
}

pub fn isFavorite(mail: MailEntry) bool {
    return mail.is_favorite or (mail.state & State.favorite) != 0;
}

pub fn wireState(mail: MailEntry) i32 {
    return if (mail.is_favorite) (mail.state | State.favorite) else (mail.state & ~State.favorite);
}

pub fn entryToProto(mail: MailEntry, arena: Allocator) !pb.PbMailInfo {
    var pb_att: std.ArrayList(pb.PbMailAttachment) = .empty;
    for (mail.attachments) |att| {
        if (att.item_id <= 0 or att.count <= 0) continue;
        try pb_att.append(arena, .{ .Id = att.item_id, .Count = att.count });
    }
    return .{
        .Id = try arena.dupe(u8, mail.id),
        .ReceivedTime = mail.received_time,
        .ReadTime = mail.read_time,
        .State = wireState(mail),
        .Level = std.enums.fromInt(pb.MailLevel, mail.level) orelse .General,
        .Title = try arena.dupe(u8, mail.title),
        .Content = try arena.dupe(u8, mail.content),
        .Sender = try arena.dupe(u8, mail.sender),
        .ValidTime = mail.valid_time,
        .ReadValidTime = mail.read_valid_time,
        .Attachments = pb_att,
        .ConfigId = mail.config_id,
        .ExpiryTime = mail.expiry_time,
    };
}

pub fn toProtoNotify(info: MailInfo, arena: Allocator) !pb.MailInfosNotify {
    var list: std.ArrayList(pb.PbMailInfo) = .empty;
    for (info.mails) |mail| {
        try list.append(arena, try entryToProto(mail, arena));
    }
    return .{ .MailInfos = list };
}
