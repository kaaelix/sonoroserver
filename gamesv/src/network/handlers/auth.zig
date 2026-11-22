const std = @import("std");
const proto = @import("proto");
const encryption = @import("../encryption.zig");

const Io = std.Io;
const pb = proto.pb;
const Allocator = std.mem.Allocator;
const ArenaAllocator = std.heap.ArenaAllocator;

const Message = @import("../Message.zig");
const Connection = @import("../Connection.zig");
const Account = @import("../../fs/Account.zig");
const BasicInfo = @import("../../fs/BasicInfo.zig");
const file_util = @import("../../fs/file_util.zig");
const FileSystem = @import("common").FileSystem;

const proto_key_type: i32 = 2;
const reconnect_token_bytes = 32;

pub const PendingReconnect = struct {
    rpc_id: u16,
    last_server_seq_no: i32,
};

pub fn handleAuthGroupRequest(
    message: Message,
    conn: *Connection,
    fs: *FileSystem,
    gpa: Allocator,
    io: Io,
    player_id: *?i32,
    enter: *bool,
    pending_reconnect: *?PendingReconnect,
) !void {
    if (std.meta.activeTag(message.header) != .request) return error.InvalidMessageType;
    const auth_message_type = std.enums.fromInt(AuthMessageType, message.header.request.msg_id) orelse {
        const log = std.log.scoped(.auth);
        log.warn("unhandled auth message id: {d}, ignoring to keep session alive", .{message.header.request.msg_id});
        return;
    };

    switch (auth_message_type) {
        inline else => |msg| {
            inline for (comptime std.meta.declarations(@This())) |decl| {
                switch (@typeInfo(@TypeOf(@field(@This(), decl.name)))) {
                    .@"fn" => |f| {
                        if (f.params.len != 5) continue;
                        const Context = f.params[0].type.?;
                        if (!@hasDecl(Context, "Request")) continue;

                        if (comptime !std.mem.eql(u8, @tagName(msg), @typeName(Context.Request)[3..])) continue;

                        var arena = ArenaAllocator.init(gpa);
                        defer arena.deinit();

                        var reader = Io.Reader.fixed(message.payload);
                        const request = try proto.decodeMessage(&reader, arena.allocator(), Context.Request, proto.pb_desc);

                        try @field(@This(), decl.name)(.{
                            .rpc_id = message.header.request.rpc_id,
                            .request = request,
                            .conn = conn,
                            .player_id = player_id,
                            .enter = enter,
                            .pending_reconnect = pending_reconnect,
                        }, arena.allocator(), fs, gpa, io);
                    },
                    else => {},
                }
            }
        },
    }
}

const AuthMessageType = blk: {
    var names: []const []const u8 = &.{};

    for (std.meta.declarations(@This())) |decl| {
        switch (@typeInfo(@TypeOf(@field(@This(), decl.name)))) {
            .@"fn" => |f| {
                if (f.params.len != 5) continue;
                const Context = f.params[0].type.?;

                if (!@hasDecl(Context, "Request")) continue;

                names = names ++ .{@typeName(Context.Request)[3..]};
            },
            else => {},
        }
    }

    var msg_ids: [names.len]u16 = @splat(0);
    for (names, 0..) |name, i| {
        msg_ids[i] = @field(proto.pb_desc, name).msg_id;
    }

    break :blk @Enum(u16, .exhaustive, names, &msg_ids);
};

fn AuthContext(comptime R: type) type {
    return struct {
        pub const Request = R;

        rpc_id: u16,
        request: Request,
        conn: *Connection,
        player_id: *?i32,
        enter: *bool,
        pending_reconnect: *?PendingReconnect,

        pub fn respond(ac: @This(), gpa: Allocator, io: Io, response: anytype) !void {
            try ac.conn.respond(gpa, io, ac.rpc_id, response);
        }
    };
}

pub fn handleProtoKeyRequest(ac: AuthContext(pb.ProtoKeyRequest), arena: Allocator, fs: *FileSystem, gpa: Allocator, io: Io) !void {
    const log = std.log.scoped(.proto_key);
    log.debug("request: {}", .{ac.request});

    var session_key: [32]u8 = undefined;
    Io.random(fs.io, &session_key);

    const public_key_der = try fs.readFile(arena, "gateway/client_public_key.der") orelse return error.NoClientPublicKey;

    var encrypted_key: [encryption.pkcsv1_5_padded_size]u8 = undefined;
    try encryption.rsaEncrypt(public_key_der, session_key[0..], encrypted_key[0..], fs.io);

    if (ac.request.IsLogin) {
        try ac.respond(gpa, io, pb.ProtoKeyResponse{
            .Type = proto_key_type,
            .Key = encrypted_key[0..],
        });
    } else {
        try ac.conn.respondSeq0(gpa, io, ac.rpc_id, pb.ProtoKeyResponse{
            .Type = proto_key_type,
            .Key = encrypted_key[0..],
        });
    }

    ac.conn.session_key = session_key;
}

pub fn handleLoginRequest(ac: AuthContext(pb.LoginRequest), arena: Allocator, fs: *FileSystem, gpa: Allocator, io: Io) !void {
    const log = std.log.scoped(.login);
    log.debug("request: {}", .{ac.request});

    if (!Account.isSafeIdentifier(ac.request.Account) or
        !(try Account.verifyLoginToken(arena, fs, ac.request.Account, ac.request.Token)))
    {
        log.warn("login rejected for account '{s}'", .{ac.request.Account});
        try ac.respond(gpa, io, pb.LoginResponse{
            .ErrorCode = .InvalidToken,
            .Timestamp = Io.Clock.real.now(fs.io).toMilliseconds(),
        });
        return;
    }

    const account = try Account.loadOrCreate(arena, fs, ac.request.Account);
    ac.player_id.* = account.player_id;

    const basic_info_path = try std.fmt.allocPrint(arena, "player/{}/basic_info", .{account.player_id});
    const has_character = (try fs.readFile(arena, basic_info_path)) != null;

    if (!has_character) {
        log.info("player {d} has no character yet, awaiting create character request", .{account.player_id});
        try ac.respond(gpa, io, pb.LoginResponse{
            .ErrorCode = .HaveNoCharacter,
            .ReconnectToken = try issueReconnectToken(arena, fs, account.player_id),
            .Timestamp = Io.Clock.real.now(fs.io).toMilliseconds(),
        });
        return;
    }

    try fs.deleteFile(try freshReconnectPath(arena, account.player_id));

    try ac.respond(gpa, io, pb.LoginResponse{
        .ErrorCode = .Success,
        .ReconnectToken = try issueReconnectToken(arena, fs, account.player_id),
        .Timestamp = Io.Clock.real.now(fs.io).toMilliseconds(),
    });
}

pub fn handleCreateCharacterRequest(ac: AuthContext(pb.CreateCharacterRequest), arena: Allocator, fs: *FileSystem, gpa: Allocator, io: Io) !void {
    const log = std.log.scoped(.create_character);
    log.info("request: Sex={d}, Name='{s}'", .{ ac.request.Sex, ac.request.Name });

    const player_id = ac.player_id.* orelse {
        log.err("create character request received without player_id in session", .{});
        try ac.respond(gpa, io, pb.CreateCharacterResponse{
            .ErrorCode = .CreateCharacterFailed,
        });
        return;
    };

    const trimmed_name = std.mem.trim(u8, ac.request.Name, " \t\r\n");
    if (trimmed_name.len == 0 or trimmed_name.len > 64 or (ac.request.Sex != 0 and ac.request.Sex != 1)) {
        try ac.respond(gpa, io, pb.CreateCharacterResponse{
            .ErrorCode = .CreateCharacterFailed,
        });
        return;
    }

    const basic_info_path = try std.fmt.allocPrint(arena, "player/{}/basic_info", .{player_id});
    if ((try fs.readFile(arena, basic_info_path)) != null) {
        try ac.respond(gpa, io, pb.CreateCharacterResponse{
            .ErrorCode = .CreateCharacterDuplicateKey,
        });
        return;
    }

    const basic_info: BasicInfo = .{
        .attributes = .{
            .name = try arena.dupe(u8, trimmed_name),
            .sex = ac.request.Sex,
            .level = 1,
            .cur_world_level = 1,
            .origin_world_level = 1,
        },
    };
    const basic_info_data = try file_util.serializeZon(arena, basic_info);
    try fs.writeFile(basic_info_path, basic_info_data);

    try Account.syncLoginSexForPlayer(arena, fs, player_id, ac.request.Sex);
    // Character creation login returns HaveNoCharacter before EnterGame, so the client may not have stored a ReconnectToken yet.
    try fs.writeFile(try freshReconnectPath(arena, player_id), "1");

    const now_s: i32 = @intCast(Io.Clock.real.now(fs.io).toSeconds());
    try ac.respond(gpa, io, pb.CreateCharacterResponse{
        .ErrorCode = .Success,
        .Name = trimmed_name,
        .PlayerId = player_id,
        .CreateTime = now_s,
    });
}

pub fn handleReconnectRequest(ac: AuthContext(pb.ReconnectRequest), arena: Allocator, fs: *FileSystem, gpa: Allocator, io: Io) !void {
    const log = std.log.scoped(.reconnect);
    log.debug("request: {}", .{ac.request});

    var resolved_player_id = ac.request.PlayerId;
    if (resolved_player_id <= 0 and ac.request.ReconnectToken.len == reconnect_token_bytes * 2) {
        if (ac.player_id.*) |known_pid| {
            if (try validateReconnectToken(arena, fs, known_pid, ac.request.ReconnectToken)) {
                resolved_player_id = known_pid;
            }
        }
        if (resolved_player_id <= 0) {
            const fallback_pid: i32 = 597938510;
            if (try validateReconnectToken(arena, fs, fallback_pid, ac.request.ReconnectToken)) {
                resolved_player_id = fallback_pid;
            }
        }
    }

    if (resolved_player_id <= 0) {
        try ac.respond(gpa, io, pb.ReconnectResponse{
            .ErrorCode = .ErrReconnectGwgetGatePlayerFailed,
            .Timestamp = Io.Clock.real.now(fs.io).toMilliseconds(),
            .IsPermittedSilentLogin = false,
        });
        return;
    }

    const basic_info_path = try std.fmt.allocPrint(arena, "player/{}/basic_info", .{resolved_player_id});
    if (try fs.readFile(arena, basic_info_path) == null) {
        try ac.respond(gpa, io, pb.ReconnectResponse{
            .ErrorCode = .ErrReconnectGwgetGatePlayerFailed,
            .Timestamp = Io.Clock.real.now(fs.io).toMilliseconds(),
            .IsPermittedSilentLogin = false,
        });
        return;
    }

    const fresh_path = try freshReconnectPath(arena, resolved_player_id);
    const has_fresh_marker = (try fs.readFile(arena, fresh_path)) != null;
    const token_valid = if (ac.request.ReconnectToken.len == 0 and has_fresh_marker) blk: {
        try fs.deleteFile(fresh_path);
        break :blk true;
    } else blk: {
        const valid = try validateReconnectToken(arena, fs, resolved_player_id, ac.request.ReconnectToken);
        if (valid and has_fresh_marker) try fs.deleteFile(fresh_path);
        break :blk valid;
    };

    if (!token_valid) {
        try ac.respond(gpa, io, pb.ReconnectResponse{
            .ErrorCode = .ErrReconnectGwgetGatePlayerFailed,
            .Timestamp = Io.Clock.real.now(fs.io).toMilliseconds(),
            .IsPermittedSilentLogin = false,
        });
        return;
    }

    ac.player_id.* = resolved_player_id;
    ac.enter.* = true;
    ac.pending_reconnect.* = .{
        .rpc_id = ac.rpc_id,
        .last_server_seq_no = ac.request.LastSvrSeqNo,
    };
}

pub fn handleHeartbeatRequest(ac: AuthContext(pb.HeartbeatRequest), _: Allocator, _: *FileSystem, gpa: Allocator, io: Io) !void {
    const log = std.log.scoped(.heartbeat);
    log.debug("heartbeat received in auth phase", .{});
    try ac.respond(gpa, io, pb.HeartbeatResponse{});
}

pub fn handleEnterGameRequest(ac: AuthContext(pb.EnterGameRequest), arena: Allocator, fs: *FileSystem, gpa: Allocator, io: Io) !void {
    const log = std.log.scoped(.enter_game);
    log.debug("request: {}", .{ac.request});

    const player_id = ac.player_id.* orelse {
        try ac.respond(gpa, io, pb.EnterGameResponse{ .ErrorCode = .HaveNoCharacter });
        return;
    };
    const basic_info_path = try std.fmt.allocPrint(arena, "player/{}/basic_info", .{player_id});
    if ((try fs.readFile(arena, basic_info_path)) == null) {
        try ac.respond(gpa, io, pb.EnterGameResponse{ .ErrorCode = .HaveNoCharacter });
        return;
    }

    ac.enter.* = true; // switch state
    try ac.respond(gpa, io, pb.EnterGameResponse{});
}

fn issueReconnectToken(arena: Allocator, fs: *FileSystem, player_id: i32) ![]const u8 {
    var random: [reconnect_token_bytes]u8 = undefined;
    Io.random(fs.io, &random);

    const token_hex = std.fmt.bytesToHex(random, .lower);
    const token = try arena.dupe(u8, &token_hex);
    try fs.writeFile(try reconnectTokenPath(arena, player_id), token);
    return token;
}

fn validateReconnectToken(arena: Allocator, fs: *FileSystem, player_id: i32, token: []const u8) !bool {
    if (token.len != reconnect_token_bytes * 2) return false;
    const saved = try fs.readFile(arena, try reconnectTokenPath(arena, player_id)) orelse return false;
    return std.mem.eql(u8, saved, token);
}

fn reconnectTokenPath(arena: Allocator, player_id: i32) ![]const u8 {
    return std.fmt.allocPrint(arena, "player/{}/reconnect_token", .{player_id});
}

fn freshReconnectPath(arena: Allocator, player_id: i32) ![]const u8 {
    return std.fmt.allocPrint(arena, "player/{}/fresh_reconnect", .{player_id});
}
