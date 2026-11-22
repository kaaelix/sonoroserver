const std = @import("std");
const common = @import("common");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const comp_util = @import("../../logic/component/comp_util.zig");
const CoopService = @import("../../logic/CoopService.zig");
const PlayerBasicComponent = @import("../../logic/component/player/PlayerBasicComponent.zig");
const PlayerProgressComponent = @import("../../logic/component/player/PlayerProgressComponent.zig");
const QuestEngine = @import("../../logic/helpers/quest_engine.zig");
const RewardService = @import("../../logic/helpers/reward.zig");
const EnergyHelper = @import("../../logic/helpers/energy.zig");
const PlayerID = @import("../../logic/PlayerID.zig");
const Transaction = @import("../handlers.zig").Transaction;

const FileSystem = common.FileSystem;

pub fn onLobbyListRequest(
    txn: *Transaction(pb.LobbyListRequest),
    alloc: mem.Alloc,
    io: std.Io,
    pid: PlayerID,
    coop: *CoopService,
) !void {
    _ = txn.message;
    const list = try coop.buildLobbyList(alloc.arena, io, pid.id);
    txn.respond(.{ .ErrorCode = .Success, .ItemList = list });
}

pub fn onLobbyQueryPlayersRequest(
    txn: *Transaction(pb.LobbyQueryPlayersRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    coop: *CoopService,
) !void {
    const target_id = txn.message.PlayerId;
    if (try coop.getOnlineDetails(alloc.arena, io, target_id)) |det| {
        txn.respond(.{ .ErrorCode = .Success, .Item = det });
        return;
    }
    var rec = CoopService.loadSocial(alloc.gpa, alloc.arena, fs, target_id) catch {
        txn.respond(.{ .ErrorCode = .PlayerLoggedOut });
        return;
    };
    defer rec.deinit(alloc.gpa);
    txn.respond(.{
        .ErrorCode = .Success,
        .Item = .{
            .PlayerId = target_id,
            .Name = try alloc.arena.dupe(u8, if (rec.name.len > 0) rec.name else "Rover"),
            .Level = rec.level,
            .OriginWorldLevel = rec.origin_world_level,
            .CurWorldLevel = rec.cur_world_level,
            .HeadId = rec.head_id,
            .Signature = try alloc.arena.dupe(u8, rec.signature),
            .IsOnline = false,
            .IsCanLobbyOnline = false,
        },
    });
}

pub fn onWorldEnterPermissionsRequest(
    txn: *Transaction(pb.WorldEnterPermissionsRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    pid: PlayerID,
    basic_comp: *PlayerBasicComponent,
    coop: *CoopService,
) !void {
    const perm = txn.message.Type orelse .ConfirmJoin;
    basic_comp.info.attributes.world_permission = @intFromEnum(perm);
    try comp_util.saveStruct(fs, basic_comp.info, try std.fmt.allocPrint(alloc.arena, "player/{}/basic_info", .{basic_comp.player_id}), alloc.arena);
    coop.setPermission(io, pid.id, perm);
    txn.respond(.{ .ErrorCode = .Success, .Type = perm });
}

pub fn onApplyJoinWorldRequest(
    txn: *Transaction(pb.ApplyJoinWorldRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    pid: PlayerID,
    coop: *CoopService,
) !void {
    const code = try coop.requestJoinWorld(alloc.gpa, alloc.arena, io, fs, pid.id, txn.message.PlayerId);
    txn.respond(.{ .ErrorCode = code });
}

pub fn onAgreeJoinResultRequest(
    txn: *Transaction(pb.AgreeJoinResultRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    pid: PlayerID,
    coop: *CoopService,
) !void {
    const code = try coop.admitToRoom(alloc.gpa, alloc.arena, io, fs, pid.id, txn.message.PlayerId, txn.message.Result);
    txn.respond(.{ .ErrorCode = code });
}

pub fn onLeaveWorldTeamRequest(
    txn: *Transaction(pb.LeaveWorldTeamRequest),
    alloc: mem.Alloc,
    io: std.Io,
    pid: PlayerID,
    coop: *CoopService,
) !void {
    const target = if (txn.message.PlayerId > 0) txn.message.PlayerId else pid.id;
    coop.leaveOrKickRoom(alloc.gpa, io, target, txn.message.Reason orelse .Initiative);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onKickWorldTeamRequest(
    txn: *Transaction(pb.KickWorldTeamRequest),
    alloc: mem.Alloc,
    io: std.Io,
    pid: PlayerID,
    coop: *CoopService,
) !void {
    if (txn.message.PlayerId == pid.id or !coop.isRoomHost(io, pid.id, txn.message.PlayerId)) {
        txn.respond(.{ .ErrorCode = .ErrNotHostCanNotKick });
        return;
    }
    coop.leaveOrKickRoom(alloc.gpa, io, txn.message.PlayerId, .BeKick);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onQueryOnlinePlayersAreaRequest(
    txn: *Transaction(pb.QueryOnlinePlayersAreaRequest),
    alloc: mem.Alloc,
    io: std.Io,
    coop: *CoopService,
) !void {
    _ = txn.message;
    var areas: std.ArrayList(pb.MapEntry(i32, i32)) = .empty;
    coop.mutex.lockUncancelable(io);
    defer coop.mutex.unlock(io);
    var it = coop.online.valueIterator();
    while (it.next()) |p| {
        try areas.append(alloc.arena, .{ .key = p.player_id, .value = p.area_id });
    }
    txn.respond(.{ .ErrorCode = .Success, .AreaDict = areas });
}

// === Matchmaking & Team Challenges ===
pub fn onStartMatchRequest(
    txn: *Transaction(pb.StartMatchRequest),
    alloc: mem.Alloc,
    io: std.Io,
    pid: PlayerID,
    coop: *CoopService,
) !void {
    try coop.startMatch(alloc.gpa, alloc.arena, io, pid.id, txn.message.InstId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onCancelMatchRequest(
    txn: *Transaction(pb.CancelMatchRequest),
    alloc: mem.Alloc,
    io: std.Io,
    pid: PlayerID,
    coop: *CoopService,
) !void {
    coop.cancelMatch(alloc.gpa, io, pid.id);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onMatchConfirmRequest(
    txn: *Transaction(pb.MatchConfirmRequest),
    alloc: mem.Alloc,
    io: std.Io,
    pid: PlayerID,
    coop: *CoopService,
) !void {
    coop.confirmMatch(alloc.gpa, io, pid.id, txn.message.IsAccept);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onMatchChangeReadyRequest(
    txn: *Transaction(pb.MatchChangeReadyRequest),
    alloc: mem.Alloc,
    io: std.Io,
    pid: PlayerID,
    coop: *CoopService,
) !void {
    coop.mutex.lockUncancelable(io);
    if (coop.online.getPtr(pid.id)) |me| me.match_ready = txn.message.IsReady;
    coop.mutex.unlock(io);
    coop.broadcastMatch(alloc.gpa, io, pid.id, 0, pb.MatchChangeReadyNotify{
        .PlayerId = pid.id,
        .IsReady = txn.message.IsReady,
    });
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onMatchChangeRoleRequest(
    txn: *Transaction(pb.MatchChangeRoleRequest),
    alloc: mem.Alloc,
    io: std.Io,
    pid: PlayerID,
    coop: *CoopService,
) !void {
    coop.mutex.lockUncancelable(io);
    if (coop.online.getPtr(pid.id)) |me| {
        for (0..4) |i| {
            me.match_roles[i] = if (i < txn.message.RoleId.items.len) txn.message.RoleId.items[i] else 0;
        }
    }
    coop.mutex.unlock(io);
    coop.broadcastMatch(alloc.gpa, io, pid.id, 0, pb.MatchChangeRoleNotify{ .PlayerId = pid.id });
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onMatchChangePlayerUiStateRequest(
    txn: *Transaction(pb.MatchChangePlayerUiStateRequest),
    alloc: mem.Alloc,
    io: std.Io,
    pid: PlayerID,
    coop: *CoopService,
) !void {
    const ui_state = txn.message.MatchUiState orelse .Wait;
    coop.mutex.lockUncancelable(io);
    if (coop.online.getPtr(pid.id)) |me| me.match_ui_state = ui_state;
    coop.mutex.unlock(io);
    coop.broadcastMatch(alloc.gpa, io, pid.id, 0, pb.MatchChangePlayerUiStateNotify{
        .PlayerId = pid.id,
        .MatchUiState = ui_state,
    });
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onLeaveMatchTeamRequest(
    txn: *Transaction(pb.LeaveMatchTeamRequest),
    alloc: mem.Alloc,
    io: std.Io,
    pid: PlayerID,
    coop: *CoopService,
) !void {
    coop.broadcastMatch(alloc.gpa, io, pid.id, 0, pb.LeaveMatchTeamNotify{
        .PlayerId = pid.id,
        .LeaveReason = .Initiative,
    });
    coop.cancelMatch(alloc.gpa, io, pid.id);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onKickMatchTeamPlayerRequest(
    txn: *Transaction(pb.KickMatchTeamPlayerRequest),
    alloc: mem.Alloc,
    io: std.Io,
    pid: PlayerID,
    coop: *CoopService,
) !void {
    if (!coop.isMatchHost(io, pid.id)) {
        txn.respond(.{ .ErrorCode = .ErrMatchNotHostCanNotKick });
        return;
    }
    coop.broadcastMatch(alloc.gpa, io, txn.message.PlayerId, 0, pb.LeaveMatchTeamNotify{
        .PlayerId = txn.message.PlayerId,
        .LeaveReason = .BeKick,
    });
    coop.cancelMatch(alloc.gpa, io, txn.message.PlayerId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSetMatchTeamMatchFlagRequest(
    txn: *Transaction(pb.SetMatchTeamMatchFlagRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    io: std.Io,
    pid: PlayerID,
    coop: *CoopService,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "coop:match_flag", if (txn.message.IsMatch) 1 else 0);
    try progress_comp.save(fs, alloc.arena);
    coop.mutex.lockUncancelable(io);
    if (coop.online.getPtr(pid.id)) |me| me.match_confirm = txn.message.IsMatch;
    coop.mutex.unlock(io);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onEnterMatchInstRequest(
    txn: *Transaction(pb.EnterMatchInstRequest),
    io: std.Io,
    pid: PlayerID,
    coop: *CoopService,
    qctx: QuestEngine.QuestContext,
) !void {
    coop.mutex.lockUncancelable(io);
    const inst_id = if (coop.online.get(pid.id)) |me| me.match_inst_id else 0;
    coop.mutex.unlock(io);
    if (inst_id > 0) {
        try QuestEngine.enterInst(qctx, inst_id, 0);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onTeamChallengeRequest(
    txn: *Transaction(pb.TeamChallengeRequest),
    qctx: QuestEngine.QuestContext,
) !void {
    if (txn.message.InstId > 0) {
        try QuestEngine.enterInst(qctx, txn.message.InstId, 0);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onTeamMatchInviteRequest(
    txn: *Transaction(pb.TeamMatchInviteRequest),
    alloc: mem.Alloc,
    io: std.Io,
    pid: PlayerID,
    coop: *CoopService,
) !void {
    _ = coop.pushToPlayer(alloc.gpa, io, pid.id, pb.TeamMatchInviteRetToHostNotify{
        .ErrorCode = .Success,
        .InstId = txn.message.InstId,
    });
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onTeamMatchAcceptInviteRequest(
    txn: *Transaction(pb.TeamMatchAcceptInviteRequest),
    alloc: mem.Alloc,
    io: std.Io,
    pid: PlayerID,
    coop: *CoopService,
) !void {
    _ = coop.pushToPlayer(alloc.gpa, io, txn.message.HostId, pb.TeamMatchAcceptInviteNotify{
        .PlayerId = pid.id,
        .AcceptResult = txn.message.AcceptResult,
    });
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onTeamParkourRewardRequest(
    txn: *Transaction(pb.TeamParkourRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "team_pk:{d}:{d}", .{ txn.message.ActivityId, txn.message.RewardId });
    if (try progress_comp.info.markClaim(alloc.gpa, k)) {
        try progress_comp.save(fs, alloc.arena);
        _ = try RewardService.grantTableReward(reward_ctx, "TeamParKOurReward", txn.message.RewardId, false, true, 1, &.{.{ .item_id = 3, .count = 40 }}, null);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onCoopRoleRewardRequest(
    txn: *Transaction(pb.CoopRoleRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "coop:role:{d}", .{txn.message.CoopId});
    if (try progress_comp.info.markClaim(alloc.gpa, k)) {
        try progress_comp.save(fs, alloc.arena);
        _ = try RewardService.grantTableReward(reward_ctx, "CoopTaskConfig", txn.message.CoopId, false, true, 1, &.{.{ .item_id = 3, .count = 40 }}, null);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onCoopSpRewardRequest(
    txn: *Transaction(pb.CoopSpRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var kbuf: [48]u8 = undefined;
    var dirty = false;
    for (txn.message.RewardIds.items) |rid| {
        const k = try std.fmt.bufPrint(&kbuf, "coop:sp:{d}", .{rid});
        if (try progress_comp.info.markClaim(alloc.gpa, k)) {
            dirty = true;
            _ = try RewardService.grantTableReward(reward_ctx, "CoopSpReward", rid, false, true, 1, &.{.{ .item_id = 3, .count = 50 }}, null);
        }
    }
    if (dirty) try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onWorldLevelDownRequest(
    txn: *Transaction(pb.WorldLevelDownRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    io: std.Io,
    basic_comp: *PlayerBasicComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    if (basic_comp.info.attributes.cur_world_level > 1 and
        basic_comp.info.attributes.cur_world_level == basic_comp.info.attributes.origin_world_level)
    {
        basic_comp.info.attributes.cur_world_level -= 1;
        basic_comp.info.attributes.world_level_time_stamp = @intCast(EnergyHelper.nowSeconds(io));
        try comp_util.saveStruct(fs, basic_comp.info, try std.fmt.allocPrint(alloc.arena, "player/{}/basic_info", .{basic_comp.player_id}), alloc.arena);
        QuestEngine.onDifficultyChanged(qctx) catch |err| QuestEngine.logHookErr("onWorldLevelDown", err);
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .OriginWorldLevel = basic_comp.info.attributes.origin_world_level,
        .CurWorldLevel = basic_comp.info.attributes.cur_world_level,
        .WorldLevelTimeStamp = basic_comp.info.attributes.world_level_time_stamp,
    });
}

pub fn onWorldLevelRegainRequest(
    txn: *Transaction(pb.WorldLevelRegainRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    io: std.Io,
    basic_comp: *PlayerBasicComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    const changed = basic_comp.info.attributes.cur_world_level != basic_comp.info.attributes.origin_world_level;
    basic_comp.info.attributes.cur_world_level = basic_comp.info.attributes.origin_world_level;
    basic_comp.info.attributes.world_level_time_stamp = @intCast(EnergyHelper.nowSeconds(io));
    try comp_util.saveStruct(fs, basic_comp.info, try std.fmt.allocPrint(alloc.arena, "player/{}/basic_info", .{basic_comp.player_id}), alloc.arena);
    if (changed) {
        QuestEngine.onDifficultyChanged(qctx) catch |err| QuestEngine.logHookErr("onWorldLevelRegain", err);
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .OriginWorldLevel = basic_comp.info.attributes.origin_world_level,
        .CurWorldLevel = basic_comp.info.attributes.cur_world_level,
        .WorldLevelTimeStamp = basic_comp.info.attributes.world_level_time_stamp,
    });
}

pub fn onWorldBlackboardRequest(
    txn: *Transaction(pb.WorldBlackboardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var dirty = false;
    for (txn.message.Params.items) |param| {
        if (param.Key.len == 0) continue;
        const val = param.Value orelse continue;
        const k = try std.fmt.allocPrint(alloc.arena, "world_bb:{s}", .{param.Key});
        switch (val) {
            .IntValue => |v| {
                try progress_comp.info.setKeyInt(alloc.gpa, k, v);
                dirty = true;
            },
            .LongValue => |v| {
                try progress_comp.info.setKeyInt(alloc.gpa, k, v);
                dirty = true;
            },
            .BooleanValue => |v| {
                try progress_comp.info.setKeyInt(alloc.gpa, k, if (v) 1 else 0);
                dirty = true;
            },
            .StringValue => |v| {
                try progress_comp.info.setKeyStr(alloc.gpa, k, v);
                dirty = true;
            },
            .FloatValue => |v| {
                const scaled: f64 = @as(f64, v) * 1000.0;
                const iv: i64 = if (std.math.isFinite(scaled))
                    @intFromFloat(std.math.clamp(scaled, -9_000_000_000_000_000_000.0, 9_000_000_000_000_000_000.0))
                else
                    0;
                try progress_comp.info.setKeyInt(alloc.gpa, k, iv);
                dirty = true;
            },
            else => {
                _ = try progress_comp.info.markClaim(alloc.gpa, k);
                dirty = true;
            },
        }
    }
    if (dirty) try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

// === Advice Signposts ===
pub fn onAdviceRequest(
    txn: *Transaction(pb.AdviceRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    io: std.Io,
    pid: PlayerID,
    coop: *CoopService,
) !void {
    coop.advices_mutex.lockUncancelable(io);
    defer coop.advices_mutex.unlock(io);

    var store = CoopService.loadAdvices(alloc.gpa, alloc.arena, fs) catch {
        txn.respond(.{ .ErrorCode = .Success });
        return;
    };
    defer store.deinit(alloc.gpa);

    var out: std.ArrayList(pb.PbAdvice) = .empty;
    var up_vote_ids: std.ArrayList(i64) = .empty;
    for (store.advices) |a| {
        var contents: std.ArrayList(pb.PbAdviceContent) = .empty;
        for (a.contents) |c| {
            try contents.append(alloc.arena, .{
                .Type = std.enums.fromInt(pb.PbAdviceContentType, c.content_type) orelse .default,
                .Id = c.id,
                .Word = c.word,
            });
        }
        try out.append(alloc.arena, .{
            .Id = a.id,
            .AreaId = a.area_id,
            .Contents = contents,
            .UpVote = a.up_vote,
        });
        for (a.voter_ids) |vid| {
            if (vid == pid.id) {
                try up_vote_ids.append(alloc.arena, a.id);
                break;
            }
        }
    }
    txn.respond(.{ .ErrorCode = .Success, .Advices = out, .UpVoteIds = up_vote_ids });
}

pub fn onAdviceCreateRequest(
    txn: *Transaction(pb.AdviceCreateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    io: std.Io,
    pid: PlayerID,
    basic_comp: *const PlayerBasicComponent,
    coop: *CoopService,
) !void {
    coop.advices_mutex.lockUncancelable(io);
    defer coop.advices_mutex.unlock(io);

    var store = try CoopService.loadAdvices(alloc.gpa, alloc.arena, fs);
    defer store.deinit(alloc.gpa);

    const id = store.next_id;
    store.next_id += 1;
    const contents_slice = try alloc.gpa.alloc(CoopService.AdviceContentEntry, txn.message.Contents.items.len);
    var contents_owned = false;
    errdefer if (!contents_owned) alloc.gpa.free(contents_slice);

    for (txn.message.Contents.items, 0..) |c, i| {
        contents_slice[i] = .{
            .content_type = if (c.Type) |t| @intFromEnum(t) else 0,
            .id = c.Id,
            .word = c.Word,
        };
    }

    const pos = txn.message.Pos orelse pb.Vector{};
    const rot = txn.message.Rot orelse pb.Rotator{};
    const next = try alloc.gpa.alloc(CoopService.AdviceRecord, store.advices.len + 1);
    @memcpy(next[0..store.advices.len], store.advices);
    next[store.advices.len] = .{
        .id = id,
        .area_id = basic_comp.info.attributes.area_id,
        .owner_id = pid.id,
        .up_vote = 0,
        .pos_x = pos.X,
        .pos_y = pos.Y,
        .pos_z = pos.Z,
        .rot_pitch = rot.Pitch,
        .rot_yaw = rot.Yaw,
        .rot_roll = rot.Roll,
        .contents = contents_slice,
    };
    contents_owned = true;
    if (store.advices.len != 0) alloc.gpa.free(store.advices);
    store.advices = next;
    try CoopService.saveAdvices(alloc.arena, fs, store);

    txn.respond(.{
        .ErrorCode = .Success,
        .Advice = .{
            .Id = id,
            .AreaId = basic_comp.info.attributes.area_id,
            .Contents = txn.message.Contents,
            .UpVote = 0,
        },
    });
}

pub fn onAdviceDeleteRequest(
    txn: *Transaction(pb.AdviceDeleteRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    io: std.Io,
    pid: PlayerID,
    coop: *CoopService,
) !void {
    coop.advices_mutex.lockUncancelable(io);
    defer coop.advices_mutex.unlock(io);

    var store = try CoopService.loadAdvices(alloc.gpa, alloc.arena, fs);
    defer store.deinit(alloc.gpa);

    var idx: ?usize = null;
    for (store.advices, 0..) |a, i| {
        if (a.id == txn.message.Id) {
            if (a.owner_id != pid.id) {
                txn.respond(.{ .ErrorCode = .RequestParamError });
                return;
            }
            idx = i;
            break;
        }
    }
    const pos = idx orelse {
        txn.respond(.{ .ErrorCode = .ErrAdviceNotFound });
        return;
    };

    const next = try alloc.gpa.alloc(CoopService.AdviceRecord, store.advices.len - 1);
    @memcpy(next[0..pos], store.advices[0..pos]);
    @memcpy(next[pos..], store.advices[pos + 1 ..]);
    if (store.advices[pos].contents.len != 0) alloc.gpa.free(store.advices[pos].contents);
    if (store.advices[pos].voter_ids.len != 0) alloc.gpa.free(store.advices[pos].voter_ids);
    alloc.gpa.free(store.advices);
    store.advices = next;
    try CoopService.saveAdvices(alloc.arena, fs, store);

    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onAdviceModifyRequest(
    txn: *Transaction(pb.AdviceModifyRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    io: std.Io,
    pid: PlayerID,
    coop: *CoopService,
) !void {
    coop.advices_mutex.lockUncancelable(io);
    defer coop.advices_mutex.unlock(io);

    var store = try CoopService.loadAdvices(alloc.gpa, alloc.arena, fs);
    defer store.deinit(alloc.gpa);
    for (store.advices) |*a| {
        if (a.id != txn.message.Id) continue;
        if (a.owner_id != pid.id) {
            txn.respond(.{ .ErrorCode = .RequestParamError });
            return;
        }
        const cslice = try alloc.gpa.alloc(CoopService.AdviceContentEntry, txn.message.Contents.items.len);
        for (txn.message.Contents.items, 0..) |c, i| {
            cslice[i] = .{
                .content_type = if (c.Type) |t| @intFromEnum(t) else 0,
                .id = c.Id,
                .word = c.Word,
            };
        }
        if (a.contents.len != 0) alloc.gpa.free(a.contents);
        a.contents = cslice;
        try CoopService.saveAdvices(alloc.arena, fs, store);
        txn.respond(.{ .ErrorCode = .Success });
        return;
    }
    txn.respond(.{ .ErrorCode = .ErrAdviceNotFound });
}

pub fn onAdviceVoteRequest(
    txn: *Transaction(pb.AdviceVoteRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    io: std.Io,
    pid: PlayerID,
    coop: *CoopService,
) !void {
    coop.advices_mutex.lockUncancelable(io);
    defer coop.advices_mutex.unlock(io);

    var store = try CoopService.loadAdvices(alloc.gpa, alloc.arena, fs);
    defer store.deinit(alloc.gpa);
    for (store.advices) |*a| {
        if (a.id != txn.message.Id) continue;
        const vote_type = txn.message.Type orelse .Up;
        if (vote_type == .Up) {
            if (!(try CoopService.appendUniqueI32(&a.voter_ids, alloc.gpa, pid.id))) {
                txn.respond(.{ .ErrorCode = .ErrAdviceIsVoteUp });
                return;
            }
            a.up_vote += 1;
        } else {
            if (!(try CoopService.removeI32(&a.voter_ids, alloc.gpa, pid.id))) {
                txn.respond(.{ .ErrorCode = .ErrAdviceIsVoteDown });
                return;
            }
            a.up_vote = @max(0, a.up_vote - 1);
        }
        try CoopService.saveAdvices(alloc.arena, fs, store);
        try txn.conn.push(alloc.gpa, io, pb.AdviceVoteUpdateNotify{
            .Id = a.id,
            .UpVote = a.up_vote,
        });
        txn.respond(.{ .ErrorCode = .Success });
        return;
    }
    txn.respond(.{ .ErrorCode = .ErrAdviceNotFound });
}
