const std = @import("std");
const common = @import("common");
const pb = @import("proto").pb;
const Transaction = @import("../handlers.zig").Transaction;
const mem = @import("../../mem.zig");
const CoopService = @import("../../logic/CoopService.zig");
const PlayerID = @import("../../logic/PlayerID.zig");

const FileSystem = common.FileSystem;

fn buildDetailsForId(
    coop: *CoopService,
    gpa: std.mem.Allocator,
    arena: std.mem.Allocator,
    io: std.Io,
    fs: *FileSystem,
    target_id: i32,
) !pb.PlayerDetails {
    if (try coop.getOnlineDetails(arena, io, target_id)) |det| return det;
    var rec = CoopService.loadSocial(gpa, arena, fs, target_id) catch return .{
        .PlayerId = target_id,
        .Name = "Rover",
        .Level = 1,
    };
    defer rec.deinit(gpa);
    return .{
        .PlayerId = target_id,
        .Name = try arena.dupe(u8, if (rec.name.len > 0) rec.name else "Rover"),
        .Level = rec.level,
        .OriginWorldLevel = rec.origin_world_level,
        .CurWorldLevel = rec.cur_world_level,
        .HeadId = rec.head_id,
        .Signature = try arena.dupe(u8, rec.signature),
    };
}

pub fn onFriendAllRequest(
    txn: *Transaction(pb.FriendAllRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    pid: PlayerID,
    coop: *CoopService,
) !void {
    _ = txn.message;
    var friends: std.ArrayList(pb.FriendInfo) = .empty;
    var applies: std.ArrayList(pb.FriendApply) = .empty;

    var rec = try CoopService.loadSocial(alloc.gpa, alloc.arena, fs, pid.id);
    defer rec.deinit(alloc.gpa);

    for (rec.friends) |f| {
        try friends.append(alloc.arena, .{
            .Info = try buildDetailsForId(coop, alloc.gpa, alloc.arena, io, fs, f.id),
            .Remark = try alloc.arena.dupe(u8, f.remark),
        });
    }
    for (rec.pending_applies) |aid| {
        try applies.append(alloc.arena, .{
            .Info = try buildDetailsForId(coop, alloc.gpa, alloc.arena, io, fs, aid),
            .CreatedTime = 0,
        });
    }

    txn.respond(.{
        .FriendInfoList = friends,
        .FriendApplyList = applies,
        .ErrorCode = .Success,
    });
}

pub fn onPlayerBasicInfoGetRequest(
    txn: *Transaction(pb.PlayerBasicInfoGetRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    coop: *CoopService,
) !void {
    const det = try buildDetailsForId(coop, alloc.gpa, alloc.arena, io, fs, txn.message.Id);
    txn.respond(.{
        .Info = det,
        .ErrorCode = .Success,
    });
}

pub fn onFriendApplySendRequest(
    txn: *Transaction(pb.FriendApplySendRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    pid: PlayerID,
    coop: *CoopService,
) !void {
    const code = try coop.sendFriendApply(alloc.gpa, alloc.arena, io, fs, pid.id, txn.message.Id);
    txn.respond(.{ .ErrorCode = code });
}

pub fn onFriendApplyHandleRequest(
    txn: *Transaction(pb.FriendApplyHandleRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    pid: PlayerID,
    coop: *CoopService,
) !void {
    const approve = (txn.message.Operator orelse .Approve) == .Approve;
    var handled: std.ArrayList(pb.MapEntry(i32, pb.ErrorCode)) = .empty;
    for (txn.message.Ids.items) |req_id| {
        try coop.handleFriendApply(alloc.gpa, alloc.arena, io, fs, pid.id, req_id, approve);
        try handled.append(alloc.arena, .{ .key = req_id, .value = .Success });
    }
    txn.respond(.{ .ErrorCode = .Success, .HandledMap = handled });
}

pub fn onFriendDeleteRequest(
    txn: *Transaction(pb.FriendDeleteRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    pid: PlayerID,
    coop: *CoopService,
) !void {
    try coop.deleteFriend(alloc.gpa, alloc.arena, io, fs, pid.id, txn.message.Id);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onFriendRemarkRequest(
    txn: *Transaction(pb.FriendRemarkRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pid: PlayerID,
) !void {
    var rec = try CoopService.loadSocial(alloc.gpa, alloc.arena, fs, pid.id);
    defer rec.deinit(alloc.gpa);
    for (rec.friends) |*f| {
        if (f.id == txn.message.Id) {
            if (f.remark.len != 0) alloc.gpa.free(f.remark);
            f.remark = try alloc.gpa.dupe(u8, txn.message.Remark);
            try CoopService.saveSocial(alloc.arena, fs, pid.id, rec);
            break;
        }
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onFriendRecentlyTeamRequest(
    txn: *Transaction(pb.FriendRecentlyTeamRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    pid: PlayerID,
    coop: *CoopService,
) !void {
    var rec = try CoopService.loadSocial(alloc.gpa, alloc.arena, fs, pid.id);
    defer rec.deinit(alloc.gpa);
    var infos: std.ArrayList(pb.RecentlyTeamInfo) = .empty;
    for (rec.recent_teams) |rt| {
        try infos.append(alloc.arena, .{
            .Info = try buildDetailsForId(coop, alloc.gpa, alloc.arena, io, fs, rt.id),
            .TeamTime = rt.team_time,
        });
    }
    txn.respond(.{ .ErrorCode = .Success, .Infos = infos });
}

pub fn onBlockPlayerRequest(
    txn: *Transaction(pb.BlockPlayerRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    pid: PlayerID,
    coop: *CoopService,
) !void {
    try CoopService.blockPlayer(alloc.gpa, alloc.arena, fs, pid.id, txn.message.Id, true);
    const det = try buildDetailsForId(coop, alloc.gpa, alloc.arena, io, fs, txn.message.Id);
    txn.respond(.{ .ErrorCode = .Success, .Info = det });
}

pub fn onUnblockPlayerRequest(
    txn: *Transaction(pb.UnblockPlayerRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pid: PlayerID,
) !void {
    try CoopService.blockPlayer(alloc.gpa, alloc.arena, fs, pid.id, txn.message.Id, false);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onBlockListRequest(
    txn: *Transaction(pb.BlockListRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    pid: PlayerID,
    coop: *CoopService,
) !void {
    var rec = try CoopService.loadSocial(alloc.gpa, alloc.arena, fs, pid.id);
    defer rec.deinit(alloc.gpa);
    var list: std.ArrayList(pb.PlayerDetails) = .empty;
    for (rec.blocked) |bid| {
        try list.append(alloc.arena, try buildDetailsForId(coop, alloc.gpa, alloc.arena, io, fs, bid));
    }
    txn.respond(.{ .ErrorCode = .Success, .BlockList = list });
}

pub fn onReportPlayerRequest(
    txn: *Transaction(pb.ReportPlayerRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *@import("../../logic/component/player/PlayerProgressComponent.zig"),
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "social:report:{d}", .{txn.message.TargetPlayerId});
    const cnt = try progress_comp.info.addKeyInt(alloc.gpa, key, 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success, .ReportCount = @intCast(cnt) });
}
