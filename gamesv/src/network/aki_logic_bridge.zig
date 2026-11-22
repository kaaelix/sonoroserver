const std = @import("std");
const RewardService = @import("../logic/helpers/reward.zig");
const Io = std.Io;
const Allocator = std.mem.Allocator;

const log = std.log.scoped(.aki_logic_bridge);

pub const PushFrame = struct {
    msg_id: u16,
    payload: []const u8,
};

pub const LogicResponse = struct {
    msg_id: u16,
    payload: []const u8,
    pushes: []const PushFrame = &.{},
    grants: []const RewardService.RewardItem = &.{},
};

pub const BridgeConfig = struct {
    unix_socket_path: []const u8 = "/tmp/aki_logic.sock",
    tcp_fallback_addr: []const u8 = "127.0.0.1:19999",
};

pub var bridge_config: BridgeConfig = .{};

pub fn configureFromEnvironment(environ: *const std.process.Environ.Map) void {
    if (environ.get("AKI_LOGIC_SOCKET_PATH")) |path| {
        bridge_config.unix_socket_path = path;
    }
    if (environ.get("AKI_LOGIC_TCP_ADDR")) |address| {
        bridge_config.tcp_fallback_addr = address;
    }
}

var ipc_mutex: Io.Mutex = .init;
var cached_stream: ?Io.net.Stream = null;

fn applySocketTimeouts(stream: Io.net.Stream) void {
    const tv: std.posix.timeval = .{ .sec = 2, .usec = 0 };
    std.posix.setsockopt(stream.socket.handle, std.posix.SOL.SOCKET, std.posix.SO.RCVTIMEO, std.mem.asBytes(&tv)) catch {};
    std.posix.setsockopt(stream.socket.handle, std.posix.SOL.SOCKET, std.posix.SO.SNDTIMEO, std.mem.asBytes(&tv)) catch {};
}

fn connectStream(io: Io) !Io.net.Stream {
    if (Io.net.UnixAddress.init(bridge_config.unix_socket_path)) |ua| {
        if (ua.connect(io)) |stream| {
            applySocketTimeouts(stream);
            return stream;
        } else |_| {}
    } else |_| {}

    const tcp_addr = Io.net.IpAddress.parseLiteral(bridge_config.tcp_fallback_addr) catch |err| {
        log.err("Invalid TCP fallback address: {t}", .{err});
        return error.ConnectionRefused;
    };
    const stream = try tcp_addr.connect(io, .{ .mode = .stream });
    applySocketTimeouts(stream);
    return stream;
}

fn getOrConnectStream(io: Io) !Io.net.Stream {
    if (cached_stream) |s| return s;
    const s = try connectStream(io);
    cached_stream = s;
    return s;
}

fn closeCachedStream(io: Io) void {
    if (cached_stream) |s| {
        s.close(io);
        cached_stream = null;
    }
}

fn roundTrip(
    io: Io,
    arena: Allocator,
    stream: Io.net.Stream,
    player_id: i32,
    flags: u8,
    msg_id: u16,
    payload: []const u8,
    request_sent: *bool,
) !LogicResponse {
    var read_buf: [4096]u8 = undefined;
    var write_buf: [4096]u8 = undefined;

    var reader = stream.reader(io, &read_buf);
    var writer = stream.writer(io, &write_buf);

    const r = &reader.interface;
    const w = &writer.interface;

    // IPC v3 Request: [4B player_id][1B flags][2B msg_id][4B payload_len][payload]
    try w.writeInt(i32, player_id, .little);
    try w.writeByte(flags);
    try w.writeInt(u16, msg_id, .little);
    try w.writeInt(u32, @intCast(payload.len), .little);
    if (payload.len > 0) {
        try w.writeAll(payload);
    }
    try w.flush();
    // Avoid replaying mutating claims on AkiLogicService once the frame left the socket.
    request_sent.* = true;

    // IPC v3 Response:
    // [2B resp_msg_id][4B resp_len][resp_bytes]
    // [2B push_count] + push_count * ([2B push_msg_id][4B push_len][push_bytes])
    // [2B grant_count] + grant_count * ([4B item_id][4B count])
    const resp_msg_id = try r.takeInt(u16, .little);
    const resp_len = try r.takeInt(u32, .little);

    const resp_bytes: []const u8 = if (resp_len > 0)
        try r.readAlloc(arena, @intCast(resp_len))
    else
        &.{};

    const push_count = try r.takeInt(u16, .little);
    const pushes = try arena.alloc(PushFrame, push_count);
    for (0..push_count) |i| {
        const p_id = try r.takeInt(u16, .little);
        const p_len = try r.takeInt(u32, .little);
        const p_bytes: []const u8 = if (p_len > 0) try r.readAlloc(arena, @intCast(p_len)) else &.{};
        pushes[i] = .{ .msg_id = p_id, .payload = p_bytes };
    }

    const grant_count = try r.takeInt(u16, .little);
    const grants = try arena.alloc(RewardService.RewardItem, grant_count);
    for (0..grant_count) |i| {
        const item_id = try r.takeInt(i32, .little);
        const count = try r.takeInt(i32, .little);
        grants[i] = .{ .item_id = item_id, .count = count };
    }

    if (resp_msg_id == 0 and msg_id != 0) {
        return error.UnhandledByLogicService;
    }

    return .{
        .msg_id = resp_msg_id,
        .payload = resp_bytes,
        .pushes = pushes,
        .grants = grants,
    };
}

pub fn callPlayer(
    io: Io,
    arena: Allocator,
    player_id: i32,
    flags: u8,
    msg_id: u16,
    payload: []const u8,
) !LogicResponse {
    ipc_mutex.lockUncancelable(io);
    defer ipc_mutex.unlock(io);

    var attempt: usize = 0;
    while (attempt < 2) : (attempt += 1) {
        const stream = try getOrConnectStream(io);
        var request_sent = false;
        return roundTrip(io, arena, stream, player_id, flags, msg_id, payload, &request_sent) catch |err| switch (err) {
            error.UnhandledByLogicService => return err,
            else => {
                closeCachedStream(io);
                if (attempt == 0 and !request_sent) continue;
                return err;
            },
        };
    }
    return error.ConnectionRefused;
}

pub fn call(
    io: Io,
    arena: Allocator,
    msg_id: u16,
    payload: []const u8,
) !LogicResponse {
    return callPlayer(io, arena, 0, 0, msg_id, payload);
}

pub fn isHealthy(io: Io) bool {
    var buf: [256]u8 = undefined;
    var fba = std.heap.FixedBufferAllocator.init(&buf);
    const resp = callPlayer(io, fba.allocator(), 0, 0, 0, &.{}) catch return false;
    return resp.msg_id == 0;
}
