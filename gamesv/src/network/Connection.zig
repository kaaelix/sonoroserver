const Connection = @This();
const std = @import("std");
const proto = @import("proto");
const common = @import("common");
const auth = @import("handlers/auth.zig");
const net_handlers = @import("handlers.zig");
const logic_handlers = @import("../logic/handlers.zig");

const Io = std.Io;
const State = @import("State.zig");
const Allocator = std.mem.Allocator;
const FileSystem = common.FileSystem;
const Message = @import("Message.zig");
const Assets = @import("../data/Assets.zig");
const EventQueue = @import("../logic/EventQueue.zig");
const OwnedMessage = @import("../network.zig").OwnedMessage;
const OwnedFrame = @import("../network.zig").OwnedFrame;
const ConnectionHandle = @import("../network.zig").ConnectionHandle;
const SessionManager = @import("../network.zig").SessionManager;
const CoopService = @import("../logic/CoopService.zig");
const PlayerComponentStorage = @import("../logic/component/player/PlayerComponentStorage.zig");
const gm_bridge = @import("gm_bridge.zig");
const GmHttp = @import("GmHttp.zig");

outbound: *Io.Queue(OwnedFrame),
transport_work: *Io.Event,
send_mutex: Io.Mutex = .init,
session_key: ?[32]u8 = null,
seq: u32 = 0,
last_client_seq_no: u32 = 0,
last_scene_transition_seq: u32 = 0,

pub fn recordClientSeqNo(connection: *Connection, header: Message.Header) void {
    const seq_no = switch (header) {
        .request, .push => header.getSeqNo(),
        .response => return,
    };
    if (seq_no > connection.last_client_seq_no) {
        connection.last_client_seq_no = seq_no;
    }
}

pub fn resolveReconnectLastRecvSeqNo(restored_client_seq_no: u32, fallback_seq_no: i32) i32 {
    if (restored_client_seq_no > 0) {
        return @intCast(@min(restored_client_seq_no, @as(u32, std.math.maxInt(i32))));
    }
    return fallback_seq_no;
}

pub const GameplayState = struct {
    connection: Connection,
    state: ?State = null,
    claimed_player_id: ?i32 = null,

    pub fn init(handle: *ConnectionHandle) GameplayState {
        return .{
            .connection = .{
                .outbound = &handle.outbound,
                .transport_work = handle.transport_work,
            },
        };
    }
};

const gameplay_event_limit = 256;
const gameplay_time_limit_ns = 8 * std.time.ns_per_ms;
const inbound_batch_limit = 128;

// Binary search sorted 3.8.11 tables at runtime instead of materializing 65k comptime slots.
fn lookupResponseId(msg_id: u32) ?u16 {
    if (msg_id > std.math.maxInt(u16)) return null;
    const pairs = proto.registry_3811.requests[0..];
    var lo: usize = 0;
    var hi: usize = pairs.len;
    while (lo < hi) {
        const mid = lo + (hi - lo) / 2;
        const entry = pairs[mid];
        if (entry.req == msg_id) return entry.resp;
        if (entry.req < msg_id) lo = mid + 1 else hi = mid;
    }
    return null;
}

fn lookupMsgName(msg_id: u32) ?[]const u8 {
    if (msg_id > std.math.maxInt(u16)) return null;
    const names = proto.registry_3811.names[0..];
    var lo: usize = 0;
    var hi: usize = names.len;
    while (lo < hi) {
        const mid = lo + (hi - lo) / 2;
        const entry = names[mid];
        if (entry.id == msg_id) return entry.name;
        if (entry.id < msg_id) lo = mid + 1 else hi = mid;
    }
    return null;
}

const GameplayWake = struct {
    inbound_ready: ?OwnedMessage = null,
    gm_job: ?*gm_bridge.GmJob = null,
    gameplay_deadline: bool = false,
    closed: bool = false,
};

const GameplaySelectResult = union(enum) {
    inbound_ready: (Io.QueueClosedError || Io.Cancelable)!OwnedMessage,
    gm_job: (Io.QueueClosedError || Io.Cancelable)!*gm_bridge.GmJob,
    gameplay_deadline: Io.Cancelable!void,
    closed: Io.Cancelable!void,
};

const GameplayBudget = struct {
    started_ns: i96,
    completed_events: usize = 0,

    fn init(started_ns: i96) GameplayBudget {
        return .{ .started_ns = started_ns };
    }

    fn canStart(budget: GameplayBudget, now_ns: i96) bool {
        if (budget.completed_events == 0) return true;
        return budget.completed_events < gameplay_event_limit and
            now_ns - budget.started_ns < gameplay_time_limit_ns;
    }

    fn complete(budget: *GameplayBudget) void {
        budget.completed_events += 1;
    }
};

fn waitForInbound(io: Io, handle: *ConnectionHandle) (Io.QueueClosedError || Io.Cancelable)!OwnedMessage {
    const message = try handle.inbound.getOne(io);
    handle.signalTransportWork(io);
    return message;
}

fn waitForDeadline(io: Io, delay_ms: i64) Io.Cancelable!void {
    const sleep_ms = if (delay_ms > 0) delay_ms else 0;
    try io.sleep(.fromMilliseconds(sleep_ms), .awake);
}

fn waitForClosed(io: Io, handle: *ConnectionHandle) Io.Cancelable!void {
    try handle.closed_event.wait(io);
}

fn applyGameplaySelectResult(wake: *GameplayWake, result: GameplaySelectResult) void {
    switch (result) {
        .inbound_ready => |frame_result| {
            wake.inbound_ready = frame_result catch |err| switch (err) {
                error.Canceled => return,
                error.Closed => {
                    wake.closed = true;
                    return;
                },
            };
        },
        .gm_job => |job_result| {
            wake.gm_job = job_result catch |err| switch (err) {
                error.Canceled, error.Closed => return,
            };
        },
        .gameplay_deadline => |deadline_result| {
            deadline_result catch |err| switch (err) {
                error.Canceled => return,
            };
            wake.gameplay_deadline = true;
        },
        .closed => |closed_result| {
            closed_result catch |err| switch (err) {
                error.Canceled => return,
            };
            wake.closed = true;
        },
    }
}

fn waitGameplayWake(
    io: Io,
    handle: *ConnectionHandle,
    gpa: Allocator,
    gameplay_delay_ms: ?i64,
) (Io.ConcurrentError || Io.Cancelable)!GameplayWake {
    var buffer: [4]GameplaySelectResult = undefined;
    var select: Io.Select(GameplaySelectResult) = .init(io, &buffer);
    errdefer {
        while (select.cancel()) |result| {
            switch (result) {
                .inbound_ready => |message_result| {
                    if (message_result) |message| {
                        gpa.free(message.bytes);
                    } else |_| {}
                },
                .gm_job => |job_result| {
                    if (job_result) |job| {
                        job.failAndSignal(io, "session interrupted");
                    } else |_| {}
                },
                .gameplay_deadline, .closed => {},
            }
        }
    }

    try select.concurrent(.inbound_ready, waitForInbound, .{ io, handle });
    try select.concurrent(.gm_job, Io.Queue(*gm_bridge.GmJob).getOne, .{ &handle.gm_inbox, io });
    if (gameplay_delay_ms) |delay_ms| {
        try select.concurrent(.gameplay_deadline, waitForDeadline, .{ io, delay_ms });
    }
    try select.concurrent(.closed, waitForClosed, .{ io, handle });

    const first = try select.await();

    var wake: GameplayWake = .{};
    applyGameplaySelectResult(&wake, first);
    while (select.cancel()) |extra| {
        applyGameplaySelectResult(&wake, extra);
    }

    return wake;
}

fn drainGameplayDeadlines(state: *State, gpa: Allocator, io: Io, now_ms: i64) bool {
    const clock: Io.Clock = .awake;
    var budget = GameplayBudget.init(clock.now(io).toNanoseconds());
    var scheduled_changed = false;

    if (state.dirty_saves.isDue(now_ms) and budget.canStart(clock.now(io).toNanoseconds())) {
        var event_queue: EventQueue = .{ .arena = state.arena.allocator() };
        if (state.dirty_saves.enqueueDue(&event_queue, now_ms)) |dirty_due| {
            if (dirty_due) {
                drainEventChain(state, gpa, io, &event_queue);
                budget.complete();
                scheduled_changed = true;
            }
        } else |err| {
            std.log.scoped(.connection).err("failed to schedule dirty save: {t}", .{err});
            _ = state.arena.reset(.free_all);
        }
    }

    if (state.scene) |*scene| {
        if (!budget.canStart(clock.now(io).toNanoseconds())) return scheduled_changed;

        var buffs_ready = true;
        state.buff_timers.ensureInitialized(
            gpa,
            scene,
            state.assets,
            now_ms,
        ) catch |err| {
            std.log.scoped(.connection).err("failed to initialize buff timers: {t}", .{err});
            buffs_ready = false;
        };

        if (buffs_ready and
            buffDeadlineDue(&state.buff_timers, now_ms) and
            budget.canStart(clock.now(io).toNanoseconds()))
        {
            var event_queue: EventQueue = .{ .arena = state.arena.allocator() };
            event_queue.enqueue(.buff_timers_due, .{ .now_ms = now_ms }) catch |err| {
                std.log.scoped(.connection).err("failed to schedule buff timer: {t}", .{err});
                _ = state.arena.reset(.free_all);
                buffs_ready = false;
            };
            if (buffs_ready) {
                drainEventChain(state, gpa, io, &event_queue);
                budget.complete();
                scheduled_changed = true;
            }
        }

        if (deadlineDue(scene.fsm_wakes.nextWakeDelayMs(now_ms)) and
            budget.canStart(clock.now(io).toNanoseconds()))
        {
            var event_queue: EventQueue = .{ .arena = state.arena.allocator() };
            event_queue.enqueue(.fsm_wake, .{ .now_ms = now_ms }) catch |err| {
                std.log.scoped(.connection).err("failed to schedule FSM work: {t}", .{err});
                _ = state.arena.reset(.free_all);
                return scheduled_changed;
            };
            drainEventChain(state, gpa, io, &event_queue);
            budget.complete();
            scheduled_changed = true;
        }

        if (deadlineDue(scene.nextQuestWakeDelayMs(now_ms)) and
            budget.canStart(clock.now(io).toNanoseconds()))
        {
            var event_queue: EventQueue = .{ .arena = state.arena.allocator() };
            event_queue.enqueue(.quest_timers_due, .{ .now_ms = now_ms }) catch |err| {
                std.log.scoped(.connection).err("failed to schedule quest timers: {t}", .{err});
                _ = state.arena.reset(.free_all);
                return scheduled_changed;
            };
            drainEventChain(state, gpa, io, &event_queue);
            budget.complete();
            scheduled_changed = true;
        }
        if (deadlineDue(scene.pendingUpdateWakeDelayMs(now_ms)) and
            budget.canStart(clock.now(io).toNanoseconds()))
        {
            var event_queue: EventQueue = .{ .arena = state.arena.allocator() };
            event_queue.enqueue(.scene_pending_updates, .{ .now_ms = now_ms }) catch |err| {
                std.log.scoped(.connection).err("failed to schedule scene pending updates: {t}", .{err});
                _ = state.arena.reset(.free_all);
                return scheduled_changed;
            };
            drainEventChain(state, gpa, io, &event_queue);
            budget.complete();
            scheduled_changed = true;
        }

        while (buffs_ready and
            buffDeadlineDue(&state.buff_timers, now_ms) and
            budget.canStart(clock.now(io).toNanoseconds()))
        {
            var event_queue: EventQueue = .{ .arena = state.arena.allocator() };
            event_queue.enqueue(.buff_timers_due, .{ .now_ms = now_ms }) catch |err| {
                std.log.scoped(.connection).err("failed to continue buff timers: {t}", .{err});
                _ = state.arena.reset(.free_all);
                break;
            };
            drainEventChain(state, gpa, io, &event_queue);
            budget.complete();
            scheduled_changed = true;
        }
    }

    return scheduled_changed;
}

fn drainEventChain(state: *State, gpa: Allocator, io: Io, event_queue: *EventQueue) void {
    logic_handlers.drainEventQueueBestEffort(event_queue, state, gpa, io);
    _ = state.arena.reset(.free_all);
}

fn buffDeadlineDue(scheduler: *const @import("../logic/schedulers/BuffTimerScheduler.zig"), now_ms: i64) bool {
    const due_ms = scheduler.peekDueMs() orelse return false;
    return due_ms <= now_ms;
}

fn deadlineDue(delay_ms: ?i64) bool {
    return if (delay_ms) |delay| delay <= 0 else false;
}

fn nextGameplayWakeDelayMs(state: ?*const State, now_ms: i64) ?i64 {
    const s = state orelse return null;
    const scene = if (s.scene) |*scene| scene else null;
    return gameplayWakeDelayMs(
        if (scene != null) s.buff_timers.nextWakeDelayMs(now_ms) else null,
        if (scene) |active_scene| active_scene.fsm_wakes.nextWakeDelayMs(now_ms) else null,
        if (scene) |active_scene| active_scene.nextQuestWakeDelayMs(now_ms) else null,
        if (scene) |active_scene| active_scene.pendingUpdateWakeDelayMs(now_ms) else null,
        s.dirty_saves.nextWakeDelayMs(now_ms),
    );
}

fn gameplayWakeDelayMs(
    buff_delay_ms: ?i64,
    fsm_delay_ms: ?i64,
    quest_delay_ms: ?i64,
    pending_update_delay_ms: ?i64,
    dirty_delay_ms: ?i64,
) ?i64 {
    var delay_ms: ?i64 = null;
    for ([_]?i64{
        buff_delay_ms,
        fsm_delay_ms,
        quest_delay_ms,
        pending_update_delay_ms,
        dirty_delay_ms,
    }) |candidate| {
        if (candidate) |delay| delay_ms = if (delay_ms) |current| @min(current, delay) else delay;
    }
    return delay_ms;
}

var unknown_msg_logged_bits: [1024]std.atomic.Value(u64) = [_]std.atomic.Value(u64){std.atomic.Value(u64).init(0)} ** 1024;

fn markUnknownMsgLogged(msg_id: u32) bool {
    const id16 = std.math.cast(u16, msg_id) orelse return false;
    const word_idx = id16 >> 6;
    const bit: u64 = @as(u64, 1) << @as(u6, @truncate(id16));
    const prev = unknown_msg_logged_bits[word_idx].fetchOr(bit, .monotonic);
    return (prev & bit) != 0;
}

fn logUnknownMessageOnce(
    log: anytype,
    msg_id: u32,
    rpc_id: u16,
    is_request: bool,
    payload: []const u8,
) void {
    if (msg_id == 31999 or std.mem.indexOf(u8, payload, "ZPROBE") != null) {
        log.warn("[ZPROBE] unmapped msg_id={d}, rpc_id={d}, len={d}, utf8={s}", .{
            msg_id,
            rpc_id,
            payload.len,
            payload,
        });
        return;
    }

    if (markUnknownMsgLogged(msg_id)) {
        log.debug("repeated unmapped {s} (msg_id: {d}, rpc_id: {d}, len: {d})", .{
            if (is_request) "request" else "push",
            msg_id,
            rpc_id,
            payload.len,
        });
        return;
    }

    var hex_buf: [2048]u8 = undefined;
    const hex_slice = formatHexPrefix(&hex_buf, payload);
    var wire_buf: [512]u8 = undefined;
    const wire_slice = formatProtoWireSummary(&wire_buf, payload);
    log.warn("unhandled unmapped {s} (msg_id: {d}, rpc_id: {d}, len: {d}, hex: {s}, fields: [{s}])", .{
        if (is_request) "request" else "push",
        msg_id,
        rpc_id,
        payload.len,
        hex_slice,
        wire_slice,
    });
}

fn formatHexPrefix(out: []u8, bytes: []const u8) []const u8 {
    const hex_digits = "0123456789abcdef";
    const max_bytes = @min(bytes.len, out.len / 2);
    for (bytes[0..max_bytes], 0..) |byte, idx| {
        out[idx * 2] = hex_digits[byte >> 4];
        out[idx * 2 + 1] = hex_digits[byte & 0x0f];
    }
    return out[0 .. max_bytes * 2];
}

fn readVarint(bytes: []const u8, pos: *usize) ?u64 {
    var result: u64 = 0;
    var shift: u6 = 0;
    var count: usize = 0;
    while (pos.* < bytes.len and count < 10) : (count += 1) {
        const b = bytes[pos.*];
        pos.* += 1;
        result |= @as(u64, b & 0x7f) << shift;
        if ((b & 0x80) == 0) return result;
        if (shift >= 57) return null;
        shift += 7;
    }
    return null;
}

fn formatProtoWireSummary(out: []u8, payload: []const u8) []const u8 {
    var pos: usize = 0;
    var used: usize = 0;
    var fields: usize = 0;
    while (pos < payload.len and fields < 16) : (fields += 1) {
        const tag = readVarint(payload, &pos) orelse break;
        const field_no = tag >> 3;
        const wire_type: u3 = @truncate(tag);
        if (field_no == 0) break;

        if (fields > 0 and used + 2 <= out.len) {
            out[used] = ',';
            out[used + 1] = ' ';
            used += 2;
        }
        const written = std.fmt.bufPrint(out[used..], "{d}:{d}", .{ field_no, wire_type }) catch break;
        used += written.len;

        switch (wire_type) {
            0 => _ = readVarint(payload, &pos) orelse break,
            1 => {
                if (pos + 8 > payload.len) break;
                pos += 8;
            },
            2 => {
                const len = readVarint(payload, &pos) orelse break;
                if (len > payload.len - pos) break;
                pos += @intCast(len);
            },
            5 => {
                if (pos + 4 > payload.len) break;
                pos += 4;
            },
            else => break,
        }
    }
    return out[0..used];
}

test "formatProtoWireSummary inspects protobuf field numbers without panicking" {
    // field 1 varint (0x08 0x96 0x01), field 2 bytes "ab" (0x12 0x02 0x61 0x62)
    var buf: [64]u8 = undefined;
    const summary = formatProtoWireSummary(&buf, &.{ 0x08, 0x96, 0x01, 0x12, 0x02, 'a', 'b' });
    try std.testing.expectEqualStrings("1:0, 2:2", summary);

    // Truncated payload stays safe
    try std.testing.expectEqualStrings("1:2", formatProtoWireSummary(&buf, &.{ 0x0a, 0x10, 0x01 }));
}

fn normalizeWaitDelayMs(delay_ms: ?i64) ?i64 {
    const delay = delay_ms orelse return null;
    return if (delay <= 0) 1 else delay;
}

fn processInboundMessage(
    io: Io,
    handle: *ConnectionHandle,
    session_manager: *SessionManager,
    gpa: Allocator,
    fs: *FileSystem,
    assets: *const Assets,
    gameplay: *GameplayState,
    player_id: *?i32,
    enter: *bool,
    pending_reconnect: *?auth.PendingReconnect,
    owned_message: OwnedMessage,
) bool {
    const log = std.log.scoped(.connection);
    defer gpa.free(owned_message.bytes);

    var reader = Io.Reader.fixed(owned_message.bytes);
    var ready_message = Message.decode(&reader) catch |err| {
        log.err("failed to decode application message: {t}, disconnecting", .{err});
        return false;
    };
    if (gameplay.connection.session_key) |key| {
        ready_message.decrypt(key) catch |err| {
            log.err("failed to decrypt application message: {t}, disconnecting", .{err});
            return false;
        };
    }

    if (gameplay.state) |*s| {
        net_handlers.dispatchMessage(s, gpa, io, ready_message) catch |err| switch (err) {
            error.HandlerNotFound => {
                const msg_id = ready_message.header.getMessageId();
                const is_request = std.meta.activeTag(ready_message.header) == .request;
                if (is_request) {
                    if (lookupResponseId(msg_id)) |response_id| {
                        gameplay.connection.respondWithMessageId(
                            gpa,
                            io,
                            ready_message.header.request.rpc_id,
                            response_id,
                            proto.pb.HeartbeatResponse{},
                        ) catch |send_err| switch (send_err) {
                            error.Closed, error.Canceled => return false,
                            else => {},
                        };
                        if (lookupMsgName(msg_id)) |name| {
                            log.warn("sent mapped empty response (msg_id: {d}) for unhandled {s} (msg_id: {d}, rpc_id: {d})", .{
                                response_id,
                                name,
                                msg_id,
                                ready_message.header.request.rpc_id,
                            });
                        } else {
                            log.warn("sent mapped empty response (msg_id: {d}) for unmapped-name request (msg_id: {d}, rpc_id: {d})", .{
                                response_id,
                                msg_id,
                                ready_message.header.request.rpc_id,
                            });
                        }
                    } else {
                        // Do not spoof `msg_id +% 1` for unmapped 3.8.11 RPCs: WuWa response IDs are shuffled,
                        // so sending a synthetic response triggers client RPC desync/reconnect storms.
                        logUnknownMessageOnce(
                            log,
                            msg_id,
                            ready_message.header.request.rpc_id,
                            true,
                            ready_message.payload,
                        );
                    }
                } else if (lookupMsgName(msg_id)) |name| {
                    log.warn("no handler for {s} (msg_id: {d}, len: {d})", .{
                        name,
                        msg_id,
                        ready_message.payload.len,
                    });
                } else {
                    logUnknownMessageOnce(log, msg_id, 0, false, ready_message.payload);
                }
            },
            error.Closed, error.Canceled => return false,
            else => {
                log.err("failed to dispatch application message: {t}, disconnecting", .{err});
                return false;
            },
        };
        gameplay.connection.recordClientSeqNo(ready_message.header);
        _ = s.arena.reset(.free_all);
        return true;
    }

    auth.handleAuthGroupRequest(
        ready_message,
        &gameplay.connection,
        fs,
        gpa,
        io,
        player_id,
        enter,
        pending_reconnect,
    ) catch |err| {
        log.err("failed to handle auth request: {t}, disconnecting", .{err});
        return false;
    };

    if (enter.*) if (player_id.*) |id| { // Auth step finished. Initialize the state.
        session_manager.acquire(io, gpa, handle, id) catch |err| {
            log.err("failed to acquire player session: {t}, disconnecting", .{err});
            return false;
        };
        gameplay.claimed_player_id = id;
        handle.markAuthenticated(io);

        const is_reconnect = pending_reconnect.* != null;
        var maybe_handoff = session_manager.takeHandoff(io, id);
        if (pending_reconnect.*) |reconnect| {
            const last_seq: u32 = @intCast(@max(reconnect.last_server_seq_no, 0));
            if (maybe_handoff) |*handoff| {
                if (last_seq < handoff.last_scene_transition_seq) {
                    log.info(
                        "player({d}) reconnect seq={d} missed scene transition seq={d}; forcing fresh login",
                        .{ id, last_seq, handoff.last_scene_transition_seq },
                    );
                    handoff.state.deinit(gpa, io, fs);
                    maybe_handoff = null;
                }
            }
            const permit_silent_login = maybe_handoff != null;
            const restored_client_seq_no: u32 = if (maybe_handoff) |handoff| handoff.last_client_seq_no else 0;
            gameplay.connection.seq = if (maybe_handoff) |handoff|
                @max(handoff.seq, last_seq)
            else
                last_seq;
            gameplay.connection.last_client_seq_no = restored_client_seq_no;
            gameplay.connection.last_scene_transition_seq = if (maybe_handoff) |handoff|
                handoff.last_scene_transition_seq
            else
                0;

            gameplay.connection.respond(gpa, io, reconnect.rpc_id, proto.pb.ReconnectResponse{
                .ErrorCode = if (permit_silent_login) .Success else .ErrReconnectGwgetGatePlayerFailed,
                .LastRecvSeqNo = resolveReconnectLastRecvSeqNo(restored_client_seq_no, reconnect.last_server_seq_no),
                .Timestamp = Io.Clock.real.now(fs.io).toMilliseconds(),
                .IsPermittedSilentLogin = permit_silent_login,
            }) catch |err| {
                if (maybe_handoff) |*handoff| handoff.state.deinit(gpa, io, fs);
                log.err("failed to send reconnect response: {t}, disconnecting", .{err});
                return false;
            };
            pending_reconnect.* = null;
            if (!permit_silent_login) {
                log.info("player({d}) reconnect requires fresh login (no compatible live session to hand off)", .{id});
                return false;
            }
        }

        if (is_reconnect) {
            var handoff = maybe_handoff.?;
            handoff.state.conn = &gameplay.connection;
            handoff.state.coop_service = &session_manager.coop;
            gameplay.state = handoff.state;
            registerOnlineCoop(&session_manager.coop, gpa, io, &gameplay.state.?, id);
            log.info("player({d}) preserved live scene state on silent reconnect (seq={d})", .{
                id,
                gameplay.connection.seq,
            });

            var event_queue: EventQueue = .{ .arena = gameplay.state.?.arena.allocator() };
            event_queue.enqueue(.reconnect_resync, .{}) catch |err| {
                log.err("failed to enqueue reconnect resync event: {t}, disconnecting", .{err});
                return false;
            };

            logic_handlers.drainEventQueue(&event_queue, &gameplay.state.?, gpa, io) catch |err| {
                log.err("failed to execute reconnect resync event chain: {t}, disconnecting", .{err});
                return false;
            };
            _ = gameplay.state.?.arena.reset(.free_all);
        } else {
            if (maybe_handoff) |*handoff| {
                handoff.state.deinit(gpa, io, fs);
                maybe_handoff = null;
            }

            const player_components = PlayerComponentStorage.init(gpa, fs, assets, id) catch |err| {
                log.err("failed to init player component storage: {t}, disconnecting", .{err});
                return false;
            };

            gameplay.state = .init(
                gpa,
                fs,
                &gameplay.connection,
                assets,
                player_components,
                id,
            );
            gameplay.state.?.coop_service = &session_manager.coop;
            registerOnlineCoop(&session_manager.coop, gpa, io, &gameplay.state.?, id);

            var event_queue: EventQueue = .{ .arena = gameplay.state.?.arena.allocator() };
            event_queue.enqueue(.enter_game, .{}) catch |err| {
                log.err("failed to enqueue enter game event: {t}, disconnecting", .{err});
                return false;
            };

            logic_handlers.drainEventQueue(&event_queue, &gameplay.state.?, gpa, io) catch |err| {
                log.err("failed to execute initial event chain: {t}, disconnecting", .{err});
                return false;
            };
            _ = gameplay.state.?.arena.reset(.free_all);
        }
    };

    return true;
}

fn registerOnlineCoop(coop: *CoopService, gpa: Allocator, io: Io, s: *State, player_id: i32) void {
    const attrs = &s.player_components.basic.info.attributes;
    const perm = std.enums.fromInt(proto.pb.WorldEnterPermission, attrs.world_permission) orelse .ConfirmJoin;
    coop.registerOnline(gpa, io, .{
        .player_id = player_id,
        .conn = s.conn,
        .name = attrs.name,
        .level = attrs.level,
        .origin_world_level = attrs.origin_world_level,
        .cur_world_level = attrs.cur_world_level,
        .head_id = attrs.head_photo,
        .head_frame_id = attrs.head_frame,
        .signature = attrs.sign,
        .sex = attrs.sex,
        .enter_permission = perm,
        .is_can_lobby_online = perm != .ForbidJoin,
        .area_id = attrs.area_id,
    });
}

fn runGmJob(
    gameplay: *GameplayState,
    session_manager: *SessionManager,
    gpa: Allocator,
    io: Io,
    job: *gm_bridge.GmJob,
) void {
    if (gameplay.state) |*live_state| {
        GmHttp.executeSessionJob(live_state, gpa, io, session_manager, job);
    } else {
        job.failAndSignal(io, "Session is not logged in");
    }
}

pub fn runGameplay(
    io: Io,
    handle: *ConnectionHandle,
    session_manager: *SessionManager,
    gpa: Allocator,
    fs: *FileSystem,
    assets: *const Assets,
    gameplay: *GameplayState,
) void {
    const log = std.log.scoped(.connection);

    var player_id: ?i32 = null;
    var enter: bool = false;
    var pending_reconnect: ?auth.PendingReconnect = null;

    while (true) {
        if (handle.closeReason() != .active) return;

        var gm_batch: [16]*gm_bridge.GmJob = undefined;
        const gm_drained = handle.gm_inbox.get(io, &gm_batch, 0) catch 0;
        if (gm_drained > 0) {
            for (gm_batch[0..gm_drained]) |job| {
                runGmJob(gameplay, session_manager, gpa, io, job);
            }
            continue;
        }

        var batch: [inbound_batch_limit]OwnedMessage = undefined;
        const drained_count = handle.inbound.get(io, &batch, 0) catch |err| switch (err) {
            error.Closed, error.Canceled => return,
        };
        if (drained_count > 0) {
            handle.signalTransportWork(io);
            for (batch[0..drained_count], 0..) |owned_message, idx| {
                if (handle.closeReason() != .active) {
                    for (batch[idx..drained_count]) |remaining| gpa.free(remaining.bytes);
                    return;
                }
                if (!processInboundMessage(
                    io,
                    handle,
                    session_manager,
                    gpa,
                    fs,
                    assets,
                    gameplay,
                    &player_id,
                    &enter,
                    &pending_reconnect,
                    owned_message,
                )) {
                    for (batch[idx + 1 .. drained_count]) |remaining| gpa.free(remaining.bytes);
                    return;
                }
            }
            continue;
        }

        if (handle.closeReason() != .active) return;

        const now_ms = (Io.Clock.awake).now(io).toMilliseconds();
        const state_ptr: ?*const State = if (gameplay.state) |*s| s else null;
        var delay_ms = nextGameplayWakeDelayMs(state_ptr, now_ms);
        if (delay_ms) |delay| {
            if (delay <= 0) {
                if (gameplay.state) |*s| {
                    const progressed = drainGameplayDeadlines(s, gpa, io, now_ms);
                    if (handle.closeReason() != .active) return;
                    if (progressed) continue;
                    const after_ms = (Io.Clock.awake).now(io).toMilliseconds();
                    delay_ms = nextGameplayWakeDelayMs(state_ptr, after_ms);
                }
            }
        }

        const wake = waitGameplayWake(
            io,
            handle,
            gpa,
            normalizeWaitDelayMs(delay_ms),
        ) catch |err| {
            log.err("failed to wait for gameplay work: {t}", .{err});
            break;
        };

        if (wake.closed or handle.closeReason() != .active) {
            if (wake.inbound_ready) |message| gpa.free(message.bytes);
            if (wake.gm_job) |job| job.failAndSignal(io, "session closed");
            return;
        }

        if (wake.gm_job) |job| {
            runGmJob(gameplay, session_manager, gpa, io, job);
        }

        if (wake.inbound_ready) |owned_message| {
            if (!processInboundMessage(
                io,
                handle,
                session_manager,
                gpa,
                fs,
                assets,
                gameplay,
                &player_id,
                &enter,
                &pending_reconnect,
                owned_message,
            )) return;
        }

        if (wake.gameplay_deadline) {
            if (handle.closeReason() != .active) return;
            if (gameplay.state) |*s| {
                _ = drainGameplayDeadlines(s, gpa, io, (Io.Clock.awake).now(io).toMilliseconds());
            }
        }
    }
}

pub fn nextSeqNo(conn: *Connection) u32 {
    conn.seq += 1;
    return conn.seq;
}

fn enqueue(conn: *Connection, gpa: Allocator, io: Io, header: Message.Header, message: anytype) !void {
    const bytes = try Message.encodeAlloc(header, message, gpa, conn.session_key);
    errdefer gpa.free(bytes);
    try conn.outbound.putOne(io, .{ .bytes = bytes });
    conn.transport_work.set(io);
}

pub fn push(conn: *Connection, gpa: Allocator, io: Io, message: anytype) !void {
    const name = @typeName(@TypeOf(message))[3..];
    if (!@hasDecl(proto.pb_desc, name)) return;

    conn.send_mutex.lockUncancelable(io);
    defer conn.send_mutex.unlock(io);

    const header: Message.Header.Push = .{
        .seq_no = conn.nextSeqNo(),
        .msg_id = @field(proto.pb_desc, name).msg_id,
    };
    if (@TypeOf(message) == proto.pb.LeaveSceneNotify or @TypeOf(message) == proto.pb.JoinSceneNotify) {
        conn.last_scene_transition_seq = header.seq_no;
    }

    try conn.enqueue(gpa, io, .{ .push = header }, message);
}

/// Best-effort push for non-critical notifications; logs errors instead of dropping silently.
pub fn pushBestEffort(conn: *Connection, gpa: Allocator, io: Io, message: anytype) void {
    conn.push(gpa, io, message) catch |err| {
        std.log.scoped(.conn).warn("best-effort push {s} failed: {t}", .{ @typeName(@TypeOf(message)), err });
    };
}

pub fn respond(conn: *Connection, gpa: Allocator, io: Io, rpc_id: u16, response: anytype) !void {
    const name = @typeName(@TypeOf(response))[3..];
    if (!@hasDecl(proto.pb_desc, name)) return;
    try conn.respondWithMessageId(gpa, io, rpc_id, @field(proto.pb_desc, name).msg_id, response);
}

pub fn respondSeq0(conn: *Connection, gpa: Allocator, io: Io, rpc_id: u16, response: anytype) !void {
    const name = @typeName(@TypeOf(response))[3..];
    if (!@hasDecl(proto.pb_desc, name)) return;
    conn.send_mutex.lockUncancelable(io);
    defer conn.send_mutex.unlock(io);

    const header: Message.Header.Response = .{
        .rpc_id = rpc_id,
        .seq_no = 0,
        .msg_id = @field(proto.pb_desc, name).msg_id,
    };
    try conn.enqueue(gpa, io, .{ .response = header }, response);
}

fn respondWithMessageId(conn: *Connection, gpa: Allocator, io: Io, rpc_id: u16, message_id: u16, response: anytype) !void {
    conn.send_mutex.lockUncancelable(io);
    defer conn.send_mutex.unlock(io);

    const header: Message.Header.Response = .{
        .rpc_id = rpc_id,
        .seq_no = conn.nextSeqNo(),
        .msg_id = message_id,
    };
    try conn.enqueue(gpa, io, .{ .response = header }, response);
}

fn enqueueRawLocked(conn: *Connection, gpa: Allocator, io: Io, header: Message.Header, raw_payload: []const u8) !void {
    const bytes = try Message.encodeRawAlloc(header, raw_payload, gpa, conn.session_key);
    errdefer gpa.free(bytes);
    try conn.outbound.putOne(io, .{ .bytes = bytes });
    conn.transport_work.set(io);
}

pub fn pushRawWithMessageId(conn: *Connection, gpa: Allocator, io: Io, message_id: u16, raw_payload: []const u8) !void {
    conn.send_mutex.lockUncancelable(io);
    defer conn.send_mutex.unlock(io);

    const header: Message.Header.Push = .{
        .seq_no = conn.nextSeqNo(),
        .msg_id = message_id,
    };
    try conn.enqueueRawLocked(gpa, io, .{ .push = header }, raw_payload);
}

pub fn respondRawWithMessageId(conn: *Connection, gpa: Allocator, io: Io, rpc_id: u16, message_id: u16, raw_payload: []const u8) !void {
    conn.send_mutex.lockUncancelable(io);
    defer conn.send_mutex.unlock(io);

    const header: Message.Header.Response = .{
        .rpc_id = rpc_id,
        .seq_no = conn.nextSeqNo(),
        .msg_id = message_id,
    };
    try conn.enqueueRawLocked(gpa, io, .{ .response = header }, raw_payload);
}

fn decodeOwnedFrame(frame: OwnedFrame, key: ?[32]u8) !Message {
    var reader = Io.Reader.fixed(frame.bytes[3..]);
    var message = try Message.decode(&reader);
    if (key) |session_key| try message.decrypt(session_key);
    return message;
}

test "connection queues owned push and response frames in sequence order" {
    var outbound_buffer: [4]OwnedFrame = undefined;
    var outbound = Io.Queue(OwnedFrame).init(&outbound_buffer);
    var transport_work: Io.Event = .unset;
    var conn: Connection = .{
        .outbound = &outbound,
        .transport_work = &transport_work,
    };

    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const content = try arena.allocator().dupe(u8, "owned after arena reset");
    try conn.push(std.testing.allocator, std.testing.io, proto.pb.JSPatchNotify{ .Content = content });
    _ = arena.reset(.free_all);
    try conn.respond(std.testing.allocator, std.testing.io, 77, proto.pb.HeartbeatResponse{});
    try conn.respondWithMessageId(std.testing.allocator, std.testing.io, 88, 1234, proto.pb.HeartbeatResponse{});

    const push_frame = try outbound.getOne(std.testing.io);
    defer std.testing.allocator.free(push_frame.bytes);
    const response_frame = try outbound.getOne(std.testing.io);
    defer std.testing.allocator.free(response_frame.bytes);
    const fallback_frame = try outbound.getOne(std.testing.io);
    defer std.testing.allocator.free(fallback_frame.bytes);

    const push_message = try decodeOwnedFrame(push_frame, null);
    const response_message = try decodeOwnedFrame(response_frame, null);
    const fallback_message = try decodeOwnedFrame(fallback_frame, null);
    try std.testing.expectEqual(@as(u32, 1), push_message.header.getSeqNo());
    try std.testing.expectEqual(@as(u32, 2), response_message.header.getSeqNo());
    try std.testing.expectEqual(@as(u32, 3), fallback_message.header.getSeqNo());
    try std.testing.expectEqual(@as(u16, 77), response_message.header.response.rpc_id);
    try std.testing.expectEqual(@as(u16, 88), fallback_message.header.response.rpc_id);
    try std.testing.expectEqual(@as(u16, 1234), fallback_message.header.response.msg_id);

    var decode_arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer decode_arena.deinit();
    var payload_reader = Io.Reader.fixed(push_message.payload);
    const notify = try proto.decodeMessage(
        &payload_reader,
        decode_arena.allocator(),
        proto.pb.JSPatchNotify,
        proto.pb_desc,
    );
    try std.testing.expectEqualStrings("owned after arena reset", notify.Content);
}

test "queued proto key response retains the old key boundary" {
    var outbound_buffer: [2]OwnedFrame = undefined;
    var outbound = Io.Queue(OwnedFrame).init(&outbound_buffer);
    var transport_work: Io.Event = .unset;
    var conn: Connection = .{
        .outbound = &outbound,
        .transport_work = &transport_work,
    };

    try conn.respond(std.testing.allocator, std.testing.io, 9, proto.pb.ProtoKeyResponse{
        .Type = 2,
        .Key = "unencrypted",
    });

    const session_key: [32]u8 = @splat(0x5a);
    conn.session_key = session_key;
    try conn.push(std.testing.allocator, std.testing.io, proto.pb.JSPatchNotify{ .Content = "encrypted" });

    const key_frame = try outbound.getOne(std.testing.io);
    defer std.testing.allocator.free(key_frame.bytes);
    const encrypted_frame = try outbound.getOne(std.testing.io);
    defer std.testing.allocator.free(encrypted_frame.bytes);

    const key_message = try decodeOwnedFrame(key_frame, null);
    const encrypted_message = try decodeOwnedFrame(encrypted_frame, session_key);
    try std.testing.expectEqual(@as(u32, 1), key_message.header.getSeqNo());
    try std.testing.expectEqual(@as(u32, 2), encrypted_message.header.getSeqNo());

    var decode_arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer decode_arena.deinit();
    var key_payload_reader = Io.Reader.fixed(key_message.payload);
    const key_response = try proto.decodeMessage(
        &key_payload_reader,
        decode_arena.allocator(),
        proto.pb.ProtoKeyResponse,
        proto.pb_desc,
    );
    try std.testing.expectEqualStrings("unencrypted", key_response.Key);

    var payload_reader = Io.Reader.fixed(encrypted_message.payload);
    const notify = try proto.decodeMessage(
        &payload_reader,
        decode_arena.allocator(),
        proto.pb.JSPatchNotify,
        proto.pb_desc,
    );
    try std.testing.expectEqualStrings("encrypted", notify.Content);
}

test "closing a full outbound queue wakes and releases its producer" {
    var outbound_buffer: [1]OwnedFrame = undefined;
    var outbound = Io.Queue(OwnedFrame).init(&outbound_buffer);
    var transport_work: Io.Event = .unset;
    var conn: Connection = .{
        .outbound = &outbound,
        .transport_work = &transport_work,
    };
    try conn.push(std.testing.allocator, std.testing.io, proto.pb.JSPatchNotify{ .Content = "queued" });

    const Producer = struct {
        fn run(connection: *Connection) !void {
            try connection.push(std.testing.allocator, std.testing.io, proto.pb.JSPatchNotify{ .Content = "blocked" });
        }
    };
    const args = .{&conn};
    var producer = std.testing.io.concurrent(Producer.run, args) catch
        std.testing.io.async(Producer.run, args);

    outbound.close(std.testing.io);
    try std.testing.expectError(error.Closed, producer.await(std.testing.io));

    const queued = try outbound.getOne(std.testing.io);
    std.testing.allocator.free(queued.bytes);
    try std.testing.expectError(error.Closed, outbound.getOne(std.testing.io));
}

test "gameplay wake preserves simultaneous completions" {
    var gameplay: GameplayWake = .{};
    const message: OwnedMessage = .{ .bytes = undefined };
    applyGameplaySelectResult(&gameplay, .{ .inbound_ready = message });
    applyGameplaySelectResult(&gameplay, .{ .gameplay_deadline = {} });
    try std.testing.expect(gameplay.inbound_ready != null);
    try std.testing.expect(gameplay.gameplay_deadline);
    try std.testing.expect(!gameplay.closed);
}

test "canceled gameplay waits do not become wake reasons" {
    var gameplay: GameplayWake = .{};
    applyGameplaySelectResult(&gameplay, .{ .inbound_ready = error.Canceled });
    applyGameplaySelectResult(&gameplay, .{ .gameplay_deadline = error.Canceled });
    applyGameplaySelectResult(&gameplay, .{ .closed = error.Canceled });
    try std.testing.expect(gameplay.inbound_ready == null);
    try std.testing.expect(!gameplay.gameplay_deadline);
    try std.testing.expect(!gameplay.closed);
}

test "gameplay queue and explicit closure are terminal" {
    var queue_closed: GameplayWake = .{};
    applyGameplaySelectResult(&queue_closed, .{ .inbound_ready = error.Closed });
    try std.testing.expect(queue_closed.closed);
    var event_closed: GameplayWake = .{};
    applyGameplaySelectResult(&event_closed, .{ .closed = {} });
    try std.testing.expect(event_closed.closed);
}

test "gameplay deadline remains independent from inbound messages" {
    var gameplay_only: GameplayWake = .{};
    applyGameplaySelectResult(&gameplay_only, .{ .gameplay_deadline = {} });
    try std.testing.expect(gameplay_only.gameplay_deadline);
}

test "gameplay budget always starts one chain and then enforces both limits" {
    var timed = GameplayBudget.init(100);
    try std.testing.expect(timed.canStart(100 + gameplay_time_limit_ns));
    timed.complete();
    try std.testing.expect(timed.canStart(100 + gameplay_time_limit_ns - 1));
    try std.testing.expect(!timed.canStart(100 + gameplay_time_limit_ns));

    var counted = GameplayBudget.init(0);
    while (counted.completed_events < gameplay_event_limit) counted.complete();
    try std.testing.expect(!counted.canStart(0));
}

test "gameplay wake selects the earliest exact deadline" {
    try std.testing.expectEqual(@as(?i64, 0), gameplayWakeDelayMs(50, 0, null, 1_000, 30_000));
    try std.testing.expectEqual(@as(?i64, 25), gameplayWakeDelayMs(50, 100, 200, null, 25));
    try std.testing.expectEqual(@as(?i64, 15), gameplayWakeDelayMs(50, 100, 15, 1_000, null));
    try std.testing.expectEqual(@as(?i64, 1_000), gameplayWakeDelayMs(null, null, null, 1_000, 30_000));
    try std.testing.expect(gameplayWakeDelayMs(null, null, null, null, null) == null);
}

test "wait delay normalization prevents zero-millisecond select spinning" {
    try std.testing.expect(normalizeWaitDelayMs(null) == null);
    try std.testing.expectEqual(@as(?i64, 1), normalizeWaitDelayMs(-50));
    try std.testing.expectEqual(@as(?i64, 1), normalizeWaitDelayMs(0));
    try std.testing.expectEqual(@as(?i64, 25), normalizeWaitDelayMs(25));
}

test "tracks highest client upstream seq_no and resolves reconnect ack" {
    var conn: Connection = .{
        .outbound = undefined,
        .transport_work = undefined,
    };

    try std.testing.expectEqual(@as(u32, 0), conn.last_client_seq_no);
    try std.testing.expectEqual(@as(i32, 1), resolveReconnectLastRecvSeqNo(conn.last_client_seq_no, 1));

    conn.recordClientSeqNo(.{ .request = .{ .seq_no = 245, .rpc_id = 9, .msg_id = 100 } });
    conn.recordClientSeqNo(.{ .push = .{ .seq_no = 200, .msg_id = 101 } });
    conn.recordClientSeqNo(.{ .response = .{ .seq_no = 999, .rpc_id = 1, .msg_id = 102 } });
    try std.testing.expectEqual(@as(u32, 245), conn.last_client_seq_no);
    try std.testing.expectEqual(@as(i32, 245), resolveReconnectLastRecvSeqNo(conn.last_client_seq_no, 1));

    conn.recordClientSeqNo(.{ .push = .{ .seq_no = 246, .msg_id = 101 } });
    try std.testing.expectEqual(@as(u32, 246), conn.last_client_seq_no);
    try std.testing.expectEqual(@as(i32, std.math.maxInt(i32)), resolveReconnectLastRecvSeqNo(std.math.maxInt(u32), 1));
}
