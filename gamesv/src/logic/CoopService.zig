const CoopService = @This();
const std = @import("std");
const common = @import("common");
const pb = @import("proto").pb;
const Connection = @import("../network/Connection.zig");
const file_util = @import("../fs/file_util.zig");

const Io = std.Io;
const Allocator = std.mem.Allocator;
const FileSystem = common.FileSystem;

pub const FriendRecord = struct {
    id: i32,
    remark: []const u8 = "",
};

pub const RecentTeamRecord = struct {
    id: i32,
    team_time: i64 = 0,
};

pub const SocialRecord = struct {
    pub const default: SocialRecord = .{};

    name: []const u8 = "",
    level: i32 = 1,
    origin_world_level: i32 = 1,
    cur_world_level: i32 = 1,
    head_id: i32 = 82001209,
    head_frame_id: i32 = 0,
    signature: []const u8 = "",
    sex: i32 = 0,
    friends: []FriendRecord = &.{},
    pending_applies: []i32 = &.{},
    blocked: []i32 = &.{},
    recent_teams: []RecentTeamRecord = &.{},

    pub fn deinit(self: SocialRecord, gpa: Allocator) void {
        std.zon.parse.free(gpa, self);
    }
};

pub const AdviceContentEntry = struct {
    content_type: i32 = 0,
    id: i32 = 0,
    word: i32 = 0,
};

pub const AdviceRecord = struct {
    id: i64,
    area_id: i32 = 1,
    owner_id: i32 = 0,
    up_vote: i32 = 0,
    pos_x: f32 = 0,
    pos_y: f32 = 0,
    pos_z: f32 = 0,
    rot_pitch: f32 = 0,
    rot_yaw: f32 = 0,
    rot_roll: f32 = 0,
    voter_ids: []i32 = &.{},
    contents: []AdviceContentEntry = &.{},
};

pub const AdviceStore = struct {
    pub const default: AdviceStore = .{};
    next_id: i64 = 1,
    advices: []AdviceRecord = &.{},

    pub fn deinit(self: AdviceStore, gpa: Allocator) void {
        std.zon.parse.free(gpa, self);
    }
};

var social_file_mutex: Io.Mutex = .init;
var advice_file_mutex: Io.Mutex = .init;

pub const OnlinePlayer = struct {
    player_id: i32,
    conn: *Connection,
    name: []const u8,
    level: i32 = 1,
    origin_world_level: i32 = 1,
    cur_world_level: i32 = 1,
    head_id: i32 = 82001209,
    head_frame_id: i32 = 0,
    signature: []const u8 = "",
    sex: i32 = 0,
    cur_role_id: i32 = 1104,
    enter_permission: pb.WorldEnterPermission = .ConfirmJoin,
    is_can_lobby_online: bool = true,
    room_host_id: i32 = 0,
    match_inst_id: i32 = 0,
    match_host_id: i32 = 0,
    match_ready: bool = false,
    match_confirm: bool = false,
    match_ui_state: pb.EMatchPlayerUiState = .Wait,
    match_roles: [4]i32 = .{ 0, 0, 0, 0 },
    area_id: i32 = 1,
};

mutex: Io.Mutex = .init,
social_mutex: Io.Mutex = .init,
advices_mutex: Io.Mutex = .init,
online: std.AutoHashMapUnmanaged(i32, OnlinePlayer) = .empty,

pub fn deinit(self: *CoopService, gpa: Allocator) void {
    var it = self.online.valueIterator();
    while (it.next()) |p| {
        if (p.name.len != 0) gpa.free(p.name);
        if (p.signature.len != 0) gpa.free(p.signature);
    }
    self.online.deinit(gpa);
}

fn freeOnlineStrings(gpa: Allocator, p: OnlinePlayer) void {
    if (p.name.len != 0) gpa.free(p.name);
    if (p.signature.len != 0) gpa.free(p.signature);
}

pub fn registerOnline(
    self: *CoopService,
    gpa: Allocator,
    io: Io,
    entry: OnlinePlayer,
) void {
    self.mutex.lockUncancelable(io);
    defer self.mutex.unlock(io);

    const dup_name = gpa.dupe(u8, entry.name) catch return;
    const dup_sig = gpa.dupe(u8, entry.signature) catch {
        gpa.free(dup_name);
        return;
    };

    var stored = entry;
    stored.name = dup_name;
    stored.signature = dup_sig;
    if (stored.room_host_id == 0) stored.room_host_id = entry.player_id;

    if (self.online.get(entry.player_id)) |prev| {
        stored.room_host_id = prev.room_host_id;
        stored.enter_permission = prev.enter_permission;
        freeOnlineStrings(gpa, prev);
    }
    self.online.put(gpa, entry.player_id, stored) catch {
        freeOnlineStrings(gpa, stored);
    };
}

pub fn unregisterOnline(self: *CoopService, gpa: Allocator, io: Io, player_id: i32) void {
    self.mutex.lockUncancelable(io);
    defer self.mutex.unlock(io);

    if (self.online.fetchRemove(player_id)) |kv| {
        freeOnlineStrings(gpa, kv.value);
    }
}

pub fn setPermission(self: *CoopService, io: Io, player_id: i32, perm: pb.WorldEnterPermission) void {
    self.mutex.lockUncancelable(io);
    defer self.mutex.unlock(io);
    if (self.online.getPtr(player_id)) |p| {
        p.enter_permission = perm;
        p.is_can_lobby_online = perm != .ForbidJoin;
    }
}

pub fn pushToPlayer(self: *CoopService, gpa: Allocator, io: Io, target_id: i32, msg: anytype) bool {
    self.mutex.lockUncancelable(io);
    const maybe_conn = if (self.online.get(target_id)) |p| p.conn else null;
    self.mutex.unlock(io);

    const conn = maybe_conn orelse return false;
    conn.push(gpa, io, msg) catch return false;
    return true;
}

pub fn broadcastRoom(self: *CoopService, gpa: Allocator, io: Io, host_id: i32, exclude_id: i32, msg: anytype) void {
    var conns: [8]*Connection = undefined;
    var count: usize = 0;

    self.mutex.lockUncancelable(io);
    var it = self.online.valueIterator();
    while (it.next()) |p| {
        if (p.room_host_id == host_id and p.player_id != exclude_id and count < conns.len) {
            conns[count] = p.conn;
            count += 1;
        }
    }
    self.mutex.unlock(io);

    for (conns[0..count]) |conn| {
        conn.pushBestEffort(gpa, io, msg);
    }
}

pub fn broadcastMoveExcept(self: *CoopService, gpa: Allocator, io: Io, sender_id: i32, moving: std.ArrayList(pb.MovingEntityData)) void {
    var host_id: i32 = 0;
    var conns: [8]*Connection = undefined;
    var count: usize = 0;

    self.mutex.lockUncancelable(io);
    if (self.online.get(sender_id)) |sender| {
        host_id = sender.room_host_id;
    }
    if (host_id > 0) {
        var it = self.online.valueIterator();
        while (it.next()) |p| {
            if (p.room_host_id == host_id and p.player_id != sender_id and count < conns.len) {
                conns[count] = p.conn;
                count += 1;
            }
        }
    }
    self.mutex.unlock(io);

    if (count == 0) return;
    const notify: pb.MovePackageNotify = .{ .MovingEntities = moving };
    for (conns[0..count]) |conn| {
        conn.pushBestEffort(gpa, io, notify);
    }
}

fn toDetails(arena: Allocator, p: OnlinePlayer, count: i32) !pb.PlayerDetails {
    return .{
        .PlayerId = p.player_id,
        .Name = try arena.dupe(u8, p.name),
        .Level = p.level,
        .OriginWorldLevel = p.origin_world_level,
        .CurWorldLevel = p.cur_world_level,
        .HeadId = p.head_id,
        .HeadFrameId = p.head_frame_id,
        .Signature = try arena.dupe(u8, p.signature),
        .IsOnline = true,
        .IsCanLobbyOnline = p.is_can_lobby_online,
        .TeamMemberCount = count,
        .Sex = p.sex,
        .CrossPlayEnabled = true,
    };
}

fn toWorldTeamMember(arena: Allocator, p: OnlinePlayer, details: pb.PlayerDetails) !pb.WorldTeamPlayerInfo {
    var roles: std.ArrayList(pb.WorldTeamRoleInfo) = .empty;
    try roles.append(arena, .{ .RoleId = p.cur_role_id, .RoleLevel = 90 });
    return .{
        .PlayerId = p.player_id,
        .PlayerName = try arena.dupe(u8, p.name),
        .HeadId = p.head_id,
        .Level = p.level,
        .Signature = try arena.dupe(u8, p.signature),
        .FightInfo = .{
            .CurRoleId = p.cur_role_id,
            .RoleInfos = roles,
        },
        .Item = details,
        .Sex = p.sex,
        .WorldLevel = p.cur_world_level,
    };
}

pub fn countRoomMembers(self: *CoopService, io: Io, host_id: i32) i32 {
    self.mutex.lockUncancelable(io);
    defer self.mutex.unlock(io);
    return self.countRoomMembersLocked(host_id);
}

fn countRoomMembersLocked(self: *const CoopService, host_id: i32) i32 {
    var n: i32 = 0;
    var it = self.online.valueIterator();
    while (it.next()) |p| {
        if (p.room_host_id == host_id) n += 1;
    }
    return n;
}

pub fn buildLobbyList(self: *CoopService, arena: Allocator, io: Io, caller_id: i32) !std.ArrayList(pb.PlayerDetails) {
    self.mutex.lockUncancelable(io);
    defer self.mutex.unlock(io);

    var out: std.ArrayList(pb.PlayerDetails) = .empty;
    var it = self.online.valueIterator();
    while (it.next()) |p| {
        if (p.player_id == caller_id) continue;
        if (p.enter_permission == .ForbidJoin) continue;
        const cnt = self.countRoomMembersLocked(p.room_host_id);
        try out.append(arena, try toDetails(arena, p.*, cnt));
    }
    return out;
}

pub fn getOnlineDetails(self: *CoopService, arena: Allocator, io: Io, target_id: i32) !?pb.PlayerDetails {
    self.mutex.lockUncancelable(io);
    defer self.mutex.unlock(io);
    const p = self.online.get(target_id) orelse return null;
    const cnt = self.countRoomMembersLocked(p.room_host_id);
    return try toDetails(arena, p, cnt);
}

pub fn requestJoinWorld(
    self: *CoopService,
    gpa: Allocator,
    arena: Allocator,
    io: Io,
    fs: *FileSystem,
    requester_id: i32,
    host_id: i32,
) !pb.ErrorCode {
    self.mutex.lockUncancelable(io);
    const host = self.online.get(host_id);
    const req = self.online.get(requester_id);
    self.mutex.unlock(io);

    const h = host orelse return .PlayerLoggedOut;
    const r = req orelse return .InternalError;
    if (h.enter_permission == .ForbidJoin) return .ErrHostForbidJoin;

    if (h.enter_permission == .DirectJoin) {
        _ = try self.admitToRoom(gpa, arena, io, fs, host_id, requester_id, true);
        return .Success;
    }

    _ = self.pushToPlayer(gpa, io, host_id, pb.ApplyJoinWorldNotify{
        .PlayerId = r.player_id,
        .PlayerName = try arena.dupe(u8, r.name),
        .Level = r.level,
        .HeadId = r.head_id,
    });
    return .Success;
}

pub fn admitToRoom(
    self: *CoopService,
    gpa: Allocator,
    arena: Allocator,
    io: Io,
    fs: *FileSystem,
    host_id: i32,
    guest_id: i32,
    accept: bool,
) !pb.ErrorCode {
    self.mutex.lockUncancelable(io);
    const maybe_host = self.online.get(host_id);
    const maybe_guest = self.online.getPtr(guest_id);
    if (maybe_host == null or maybe_guest == null) {
        self.mutex.unlock(io);
        return .PlayerLoggedOut;
    }
    const host_snap = maybe_host.?;
    if (!accept) {
        self.mutex.unlock(io);
        _ = self.pushToPlayer(gpa, io, guest_id, pb.AgreeJoinResultNotify{
            .ErrorCode = .ErrHostRefuse,
            .PlayerName = try arena.dupe(u8, host_snap.name),
        });
        return .Success;
    }

    maybe_guest.?.room_host_id = host_id;

    var room_members: std.ArrayList(pb.WorldTeamPlayerInfo) = .empty;
    var guest_world_info: ?pb.WorldTeamPlayerInfo = null;
    var it = self.online.valueIterator();
    while (it.next()) |p| {
        if (p.room_host_id != host_id) continue;
        const det = try toDetails(arena, p.*, self.countRoomMembersLocked(host_id));
        const wi = try toWorldTeamMember(arena, p.*, det);
        try room_members.append(arena, wi);
        if (p.player_id == guest_id) guest_world_info = wi;
    }
    self.mutex.unlock(io);

    try self.recordRecentTeam(gpa, arena, fs, host_id, guest_id, io);
    try self.recordRecentTeam(gpa, arena, fs, guest_id, host_id, io);

    _ = self.pushToPlayer(gpa, io, guest_id, pb.AgreeJoinResultNotify{
        .ErrorCode = .Success,
        .PlayerName = try arena.dupe(u8, host_snap.name),
    });
    _ = self.pushToPlayer(gpa, io, guest_id, pb.JoinWorldTeamNotify{
        .OwnerId = host_id,
        .PlayerInfos = room_members,
    });
    if (guest_world_info) |gwi| {
        self.broadcastRoom(gpa, io, host_id, guest_id, pb.PlayerEnterWorldTeamNotify{
            .PlayerInfo = gwi,
        });
    }
    return .Success;
}

pub fn leaveOrKickRoom(
    self: *CoopService,
    gpa: Allocator,
    io: Io,
    target_id: i32,
    reason: pb.WorldTeamLeaveReason,
) void {
    self.mutex.lockUncancelable(io);
    const old_host: i32 = if (self.online.getPtr(target_id)) |p| blk: {
        const h = p.room_host_id;
        p.room_host_id = target_id;
        break :blk h;
    } else 0;
    self.mutex.unlock(io);

    if (old_host > 0) {
        self.broadcastRoom(gpa, io, old_host, 0, pb.PlayerLeaveWorldTeamNotify{
            .PlayerId = target_id,
            .Reason = reason,
        });
    }
    _ = self.pushToPlayer(gpa, io, target_id, pb.PlayerLeaveWorldTeamNotify{
        .PlayerId = target_id,
        .Reason = reason,
    });
}

// === Persistent Social Store ===
pub fn loadSocial(gpa: Allocator, arena: Allocator, fs: *FileSystem, player_id: i32) !SocialRecord {
    social_file_mutex.lockUncancelable(fs.io);
    defer social_file_mutex.unlock(fs.io);
    return try file_util.loadOrCreateZon(SocialRecord, gpa, arena, fs, "social/{d}", .{player_id});
}

pub fn saveSocial(arena: Allocator, fs: *FileSystem, player_id: i32, rec: SocialRecord) !void {
    social_file_mutex.lockUncancelable(fs.io);
    defer social_file_mutex.unlock(fs.io);
    const path = try std.fmt.allocPrint(arena, "social/{d}", .{player_id});
    const ser = try file_util.serializeZon(arena, rec);
    try fs.writeFile(path, ser);
}

pub fn appendUniqueI32(slice: *[]i32, gpa: Allocator, val: i32) !bool {
    for (slice.*) |v| if (v == val) return false;
    const next = try gpa.alloc(i32, slice.len + 1);
    @memcpy(next[0..slice.len], slice.*);
    next[slice.len] = val;
    if (slice.len != 0) gpa.free(slice.*);
    slice.* = next;
    return true;
}

pub fn removeI32(slice: *[]i32, gpa: Allocator, val: i32) !bool {
    var idx: ?usize = null;
    for (slice.*, 0..) |v, i| {
        if (v == val) {
            idx = i;
            break;
        }
    }
    const pos = idx orelse return false;
    const next = try gpa.alloc(i32, slice.len - 1);
    @memcpy(next[0..pos], slice.*[0..pos]);
    @memcpy(next[pos..], slice.*[pos + 1 ..]);
    gpa.free(slice.*);
    slice.* = next;
    return true;
}

pub fn addFriend(rec: *SocialRecord, gpa: Allocator, target_id: i32, remark: []const u8) !bool {
    for (rec.friends) |f| if (f.id == target_id) return false;
    const dup_remark = try gpa.dupe(u8, remark);
    errdefer if (dup_remark.len != 0) gpa.free(dup_remark);
    const next = try gpa.alloc(FriendRecord, rec.friends.len + 1);
    @memcpy(next[0..rec.friends.len], rec.friends);
    next[rec.friends.len] = .{
        .id = target_id,
        .remark = dup_remark,
    };
    if (rec.friends.len != 0) gpa.free(rec.friends);
    rec.friends = next;
    return true;
}

pub fn removeFriend(rec: *SocialRecord, gpa: Allocator, target_id: i32) !bool {
    var idx: ?usize = null;
    for (rec.friends, 0..) |f, i| {
        if (f.id == target_id) {
            idx = i;
            break;
        }
    }
    const pos = idx orelse return false;
    const next = try gpa.alloc(FriendRecord, rec.friends.len - 1);
    @memcpy(next[0..pos], rec.friends[0..pos]);
    @memcpy(next[pos..], rec.friends[pos + 1 ..]);
    if (rec.friends[pos].remark.len != 0) gpa.free(rec.friends[pos].remark);
    gpa.free(rec.friends);
    rec.friends = next;
    return true;
}

pub fn recordRecentTeam(self: *CoopService, gpa: Allocator, arena: Allocator, fs: *FileSystem, player_id: i32, mate_id: i32, io: Io) !void {
    if (player_id == mate_id) return;
    self.social_mutex.lockUncancelable(io);
    defer self.social_mutex.unlock(io);
    var rec = try loadSocial(gpa, arena, fs, player_id);
    defer rec.deinit(gpa);
    const now_s = std.Io.Clock.real.now(io).toSeconds();
    for (rec.recent_teams) |*r| {
        if (r.id == mate_id) {
            r.team_time = now_s;
            try saveSocial(arena, fs, player_id, rec);
            return;
        }
    }
    const next = try gpa.alloc(RecentTeamRecord, rec.recent_teams.len + 1);
    @memcpy(next[0..rec.recent_teams.len], rec.recent_teams);
    next[rec.recent_teams.len] = .{ .id = mate_id, .team_time = now_s };
    if (rec.recent_teams.len != 0) gpa.free(rec.recent_teams);
    rec.recent_teams = next;
    try saveSocial(arena, fs, player_id, rec);
}

pub fn sendFriendApply(
    self: *CoopService,
    gpa: Allocator,
    arena: Allocator,
    io: Io,
    fs: *FileSystem,
    sender_id: i32,
    target_id: i32,
) !pb.ErrorCode {
    if (sender_id == target_id) return .RequestParamError;
    self.social_mutex.lockUncancelable(io);
    {
        defer self.social_mutex.unlock(io);
        var target = try loadSocial(gpa, arena, fs, target_id);
        defer target.deinit(gpa);
        for (target.blocked) |b| if (b == sender_id) return .RequestParamError;
        _ = try appendUniqueI32(&target.pending_applies, gpa, sender_id);
        try saveSocial(arena, fs, target_id, target);
    }

    if (try self.getOnlineDetails(arena, io, sender_id)) |det| {
        _ = self.pushToPlayer(gpa, io, target_id, pb.FriendApplyReceivedNotify{
            .FriendApply = .{
                .Info = det,
                .CreatedTime = std.Io.Clock.real.now(io).toSeconds(),
            },
        });
    }
    return .Success;
}

pub fn handleFriendApply(
    self: *CoopService,
    gpa: Allocator,
    arena: Allocator,
    io: Io,
    fs: *FileSystem,
    player_id: i32,
    requester_id: i32,
    approve: bool,
) !void {
    self.social_mutex.lockUncancelable(io);
    {
        defer self.social_mutex.unlock(io);
        var mine = try loadSocial(gpa, arena, fs, player_id);
        defer mine.deinit(gpa);
        _ = try removeI32(&mine.pending_applies, gpa, requester_id);
        if (approve) {
            _ = try addFriend(&mine, gpa, requester_id, "");
        }
        try saveSocial(arena, fs, player_id, mine);

        if (approve) {
            var other = try loadSocial(gpa, arena, fs, requester_id);
            defer other.deinit(gpa);
            _ = try addFriend(&other, gpa, player_id, "");
            try saveSocial(arena, fs, requester_id, other);
        }
    }

    if (approve) {
        if (try self.getOnlineDetails(arena, io, player_id)) |det| {
            _ = self.pushToPlayer(gpa, io, requester_id, pb.FriendAddedNotify{
                .Info = .{ .Info = det, .Remark = "" },
            });
        }
        if (try self.getOnlineDetails(arena, io, requester_id)) |det| {
            _ = self.pushToPlayer(gpa, io, player_id, pb.FriendAddedNotify{
                .Info = .{ .Info = det, .Remark = "" },
            });
        }
    } else {
        _ = self.pushToPlayer(gpa, io, requester_id, pb.FriendApplyDeletedNotify{ .Id = player_id });
    }
}

pub fn deleteFriend(self: *CoopService, gpa: Allocator, arena: Allocator, io: Io, fs: *FileSystem, player_id: i32, target_id: i32) !void {
    self.social_mutex.lockUncancelable(io);
    {
        defer self.social_mutex.unlock(io);
        var mine = try loadSocial(gpa, arena, fs, player_id);
        defer mine.deinit(gpa);
        _ = try removeFriend(&mine, gpa, target_id);
        try saveSocial(arena, fs, player_id, mine);

        var other = try loadSocial(gpa, arena, fs, target_id);
        defer other.deinit(gpa);
        _ = try removeFriend(&other, gpa, player_id);
        try saveSocial(arena, fs, target_id, other);
    }

    _ = self.pushToPlayer(gpa, io, target_id, pb.FriendDeletedNotify{ .Id = player_id });
}

pub fn blockPlayer(gpa: Allocator, arena: Allocator, fs: *FileSystem, player_id: i32, target_id: i32, is_block: bool) !void {
    var mine = try loadSocial(gpa, arena, fs, player_id);
    defer mine.deinit(gpa);
    if (is_block) {
        _ = try appendUniqueI32(&mine.blocked, gpa, target_id);
    } else {
        _ = try removeI32(&mine.blocked, gpa, target_id);
    }
    try saveSocial(arena, fs, player_id, mine);
}

// === Advice Persistence ===
pub fn loadAdvices(gpa: Allocator, arena: Allocator, fs: *FileSystem) !AdviceStore {
    advice_file_mutex.lockUncancelable(fs.io);
    defer advice_file_mutex.unlock(fs.io);
    return try file_util.loadOrCreateZon(AdviceStore, gpa, arena, fs, "social/advices", .{});
}

pub fn saveAdvices(arena: Allocator, fs: *FileSystem, store: AdviceStore) !void {
    advice_file_mutex.lockUncancelable(fs.io);
    defer advice_file_mutex.unlock(fs.io);
    const ser = try file_util.serializeZon(arena, store);
    try fs.writeFile("social/advices", ser);
}

pub fn isRoomHost(self: *CoopService, io: Io, caller_id: i32, target_id: i32) bool {
    self.mutex.lockUncancelable(io);
    defer self.mutex.unlock(io);
    const caller = self.online.get(caller_id) orelse return false;
    if (caller.room_host_id != caller_id) return false;
    const target = self.online.get(target_id) orelse return false;
    return target.room_host_id == caller_id;
}

pub fn isMatchHost(self: *CoopService, io: Io, caller_id: i32) bool {
    self.mutex.lockUncancelable(io);
    defer self.mutex.unlock(io);
    const caller = self.online.get(caller_id) orelse return false;
    return caller.match_host_id == caller_id or caller.match_host_id == 0;
}

pub fn broadcastMatch(self: *CoopService, gpa: Allocator, io: Io, player_id: i32, exclude_id: i32, msg: anytype) void {
    var conns: [8]*Connection = undefined;
    var count: usize = 0;

    self.mutex.lockUncancelable(io);
    const host_id: i32 = if (self.online.get(player_id)) |p|
        (if (p.match_host_id > 0) p.match_host_id else player_id)
    else
        0;
    if (host_id > 0) {
        var it = self.online.valueIterator();
        while (it.next()) |p| {
            const p_host = if (p.match_host_id > 0) p.match_host_id else p.player_id;
            if ((p_host == host_id or p.player_id == player_id) and p.player_id != exclude_id and count < conns.len) {
                conns[count] = p.conn;
                count += 1;
            }
        }
    }
    self.mutex.unlock(io);

    for (conns[0..count]) |conn| {
        conn.pushBestEffort(gpa, io, msg);
    }
}

pub fn confirmMatch(self: *CoopService, gpa: Allocator, io: Io, player_id: i32, is_accept: bool) void {
    if (!is_accept) {
        self.broadcastMatch(gpa, io, player_id, 0, pb.MatchFailNotify{ .Reason = .NotAccept });
        self.mutex.lockUncancelable(io);
        const host_id: i32 = if (self.online.get(player_id)) |p|
            (if (p.match_host_id > 0) p.match_host_id else player_id)
        else
            0;
        var it = self.online.valueIterator();
        while (it.next()) |p| {
            if (p.player_id == player_id or (host_id > 0 and p.match_host_id == host_id)) {
                p.match_inst_id = 0;
                p.match_host_id = 0;
                p.match_ready = false;
                p.match_confirm = false;
                p.match_ui_state = .Wait;
            }
        }
        self.mutex.unlock(io);
        return;
    }

    self.mutex.lockUncancelable(io);
    var host_id: i32 = 0;
    if (self.online.getPtr(player_id)) |me| {
        me.match_confirm = true;
        host_id = if (me.match_host_id > 0) me.match_host_id else player_id;
    }
    var all_confirmed = host_id > 0;
    if (host_id > 0) {
        var it = self.online.valueIterator();
        while (it.next()) |p| {
            if (p.match_host_id == host_id and !p.match_confirm) {
                all_confirmed = false;
                break;
            }
        }
    }
    self.mutex.unlock(io);

    if (all_confirmed) {
        self.broadcastMatch(gpa, io, player_id, 0, pb.MatchTeamStateNotify{ .TeamState = .EnterInst });
    }
}

// === Matchmaking ===
pub fn startMatch(self: *CoopService, gpa: Allocator, arena: Allocator, io: Io, player_id: i32, inst_id: i32) !void {
    var matched_ids: [4]i32 = .{ player_id, 0, 0, 0 };
    var count: usize = 1;

    self.mutex.lockUncancelable(io);
    if (self.online.getPtr(player_id)) |me| {
        me.match_inst_id = inst_id;
        me.match_host_id = player_id;
        me.match_ready = true;
        me.match_confirm = false;
        me.match_ui_state = .Confirm;
    }
    var it = self.online.valueIterator();
    while (it.next()) |p| {
        if (p.player_id == player_id) continue;
        if (p.match_inst_id == inst_id and count < 4) {
            p.match_host_id = player_id;
            matched_ids[count] = p.player_id;
            count += 1;
        }
    }

    var team_players: std.ArrayList(pb.MatchPlayerInfo) = .empty;
    for (matched_ids[0..count]) |mid| {
        if (self.online.get(mid)) |mp| {
            try team_players.append(arena, .{
                .PlayerId = mp.player_id,
                .PlayerName = try arena.dupe(u8, mp.name),
                .IsConfirm = mp.match_confirm,
                .IsReady = mp.match_ready,
                .MatchUiState = mp.match_ui_state,
            });
        }
    }
    self.mutex.unlock(io);

    for (matched_ids[0..count]) |mid| {
        _ = self.pushToPlayer(gpa, io, mid, pb.StartMatchNotify{ .InstId = inst_id });
        _ = self.pushToPlayer(gpa, io, mid, pb.MatchTeamNotify{
            .TeamInfo = .{
                .HostId = player_id,
                .PlayerInfos = team_players,
                .TeamState = .ReadyConfirm,
            },
        });
        _ = self.pushToPlayer(gpa, io, mid, pb.MatchConfirmNotify{ .ConfirmId = inst_id });
    }
}

pub fn cancelMatch(self: *CoopService, gpa: Allocator, io: Io, player_id: i32) void {
    self.broadcastMatch(gpa, io, player_id, 0, pb.CancelMatchNotify{ .PlayerId = player_id });
    self.mutex.lockUncancelable(io);
    if (self.online.getPtr(player_id)) |me| {
        me.match_inst_id = 0;
        me.match_host_id = 0;
        me.match_ready = false;
        me.match_confirm = false;
        me.match_ui_state = .Wait;
    }
    self.mutex.unlock(io);
}

test "CoopService manages online lobby, rooms, and frees cleanly" {
    const gpa = std.testing.allocator;
    const io = std.testing.io;
    var coop: CoopService = .{};
    defer coop.deinit(gpa);

    var dummy_buf: [2]@import("../network.zig").OwnedFrame = undefined;
    var q = Io.Queue(@import("../network.zig").OwnedFrame).init(&dummy_buf);
    var ev: Io.Event = .unset;
    var conn: Connection = .{ .outbound = &q, .transport_work = &ev };

    coop.registerOnline(gpa, io, .{
        .player_id = 10,
        .conn = &conn,
        .name = "HostPlayer",
        .enter_permission = .DirectJoin,
    });
    coop.registerOnline(gpa, io, .{
        .player_id = 20,
        .conn = &conn,
        .name = "GuestPlayer",
    });

    try std.testing.expectEqual(@as(i32, 1), coop.countRoomMembers(io, 10));
    var arena = std.heap.ArenaAllocator.init(gpa);
    defer arena.deinit();
    const lobby = try coop.buildLobbyList(arena.allocator(), io, 20);
    try std.testing.expectEqual(@as(usize, 1), lobby.items.len);
    try std.testing.expectEqual(@as(i32, 10), lobby.items[0].PlayerId);
}
