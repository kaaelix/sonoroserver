const std = @import("std");
const proto = @import("proto");
const State = @import("State.zig");

fn nonSuccessCode(comptime EnumType: type) EnumType {
    if (@hasField(EnumType, "InternalError")) return .InternalError;
    if (@hasField(EnumType, "UnKnownError")) return .UnKnownError;
    if (@hasField(EnumType, "RequestParamError")) return .RequestParamError;
    inline for (@typeInfo(EnumType).@"enum".fields) |ef| {
        if (ef.value != 0) return @enumFromInt(ef.value);
    }
    return @enumFromInt(0);
}

fn failResponse(comptime Resp: type) Resp {
    var resp: Resp = .{};
    inline for (.{ "ErrorCode", "ErrCode", "ErrorId", "Code" }) |fname| {
        if (@hasField(Resp, fname)) {
            const F = @FieldType(Resp, fname);
            switch (@typeInfo(F)) {
                .optional => |opt| switch (@typeInfo(opt.child)) {
                    .@"enum" => @field(resp, fname) = nonSuccessCode(opt.child),
                    .int => @field(resp, fname) = 1,
                    else => {},
                },
                .@"enum" => @field(resp, fname) = nonSuccessCode(F),
                .int => @field(resp, fname) = 1,
                else => {},
            }
        }
    }
    return resp;
}
const Message = @import("Message.zig");
const Connection = @import("Connection.zig");
const EventQueue = @import("../logic/EventQueue.zig");
const quest_types = @import("../logic/helpers/quest_types.zig");
const logic_handlers = @import("../logic/handlers.zig");
const RewardService = @import("../logic/helpers/reward.zig");
const aki_logic_bridge = @import("aki_logic_bridge.zig");

const Io = std.Io;

const log = std.log.scoped(.net_handler);

const net_namespaces: []const type = &.{
    @import("handlers/player.zig"),
    @import("handlers/profile.zig"),
    @import("handlers/scene.zig"),
    @import("handlers/formation.zig"),
    @import("handlers/weapon.zig"),
    @import("handlers/guide.zig"),
    @import("handlers/adventure.zig"),
    @import("handlers/quest.zig"),
    @import("handlers/map.zig"),
    @import("handlers/teleport.zig"),
    @import("handlers/explore.zig"),
    @import("handlers/voxel.zig"),
    @import("handlers/settings.zig"),
    @import("handlers/mail.zig"),
    @import("handlers/access_path.zig"),
    @import("handlers/loading.zig"),
    @import("handlers/music.zig"),
    @import("handlers/direct_train.zig"),
    @import("handlers/lord_gym.zig"),
    @import("handlers/tower.zig"),
    @import("handlers/fishing.zig"),
    @import("handlers/shop.zig"),
    @import("handlers/craft.zig"),
    @import("handlers/achievement.zig"),
    @import("handlers/liveness.zig"),
    @import("handlers/combat.zig"),
    @import("handlers/role.zig"),
    @import("handlers/chat.zig"),
    @import("handlers/friend.zig"),
    @import("handlers/damage.zig"),
    @import("handlers/flow.zig"),
    @import("handlers/buff.zig"),
    @import("handlers/time.zig"),
    @import("handlers/skin.zig"),
    @import("handlers/inventory.zig"),
    @import("handlers/energy.zig"),
    @import("handlers/motor.zig"),
    @import("handlers/gacha.zig"),
    @import("handlers/minimal.zig"),
    @import("handlers/compat_3811.zig"),
    @import("handlers/qol.zig"),
    @import("handlers/endgame.zig"),
    @import("handlers/region_prog.zig"),
    @import("handlers/coop.zig"),
    @import("handlers/combat_sync.zig"),
    @import("handlers/world_puzzle.zig"),
    @import("handlers/roguelike.zig"),
    @import("handlers/phantom_battle.zig"),
    @import("handlers/events_major.zig"),
    @import("handlers/events_misc.zig"),
    @import("handlers/echo.zig"),
    @import("handlers/echo/presets.zig"),
    @import("handlers/echo/upgrade.zig"),
    @import("handlers/echo/calabash.zig"),
    @import("handlers/echo/projector.zig"),
    @import("handlers/echo/refine.zig"),
    @import("handlers/camera.zig"),
    @import("handlers/inst.zig"),
    @import("handlers/activity.zig"),
};

pub fn Transaction(comptime T: type) type {
    return struct {
        pub const MessagePB = T;

        pub const Response = blk: {
            const message_name = @typeName(T)[3..];
            if (!(std.mem.endsWith(u8, message_name, "Request"))) break :blk void;
            const response_name = message_name[0 .. message_name.len - 7] ++ "Response";
            if (!@hasDecl(proto.pb, response_name)) break :blk void;
            break :blk @field(proto.pb, response_name);
        };

        conn: *Connection,
        message: T,
        raw_payload: []const u8,
        rpc_id: u16,
        response: ?Response,
        responded_raw: bool = false,

        pub fn respond(txn: *@This(), response: Response) void {
            if (txn.response != null or txn.responded_raw) {
                log.err("duplicate response ignored for {s} (rpc_id: {d})", .{ @typeName(T), txn.rpc_id });
                return;
            }
            txn.response = response;
        }

        pub fn respondRaw(txn: *@This(), gpa: std.mem.Allocator, io: Io, message_id: u16, raw_payload: []const u8) !void {
            if (txn.response != null or txn.responded_raw) {
                log.err("duplicate raw response ignored for {s} (rpc_id: {d})", .{ @typeName(T), txn.rpc_id });
                return;
            }
            try txn.conn.respondRawWithMessageId(gpa, io, txn.rpc_id, message_id, raw_payload);
            txn.responded_raw = true;
        }
    };
}

pub const NetEventHandler = struct {
    namespace: type,
    Message: type,
    Txn: type,
    name: []const u8,

    pub inline fn invoke(
        comptime handler: NetEventHandler,
        state: *State,
        gpa: std.mem.Allocator,
        io: Io,
        message: handler.Message,
        raw_payload: []const u8,
        rpc_id: u16,
    ) !void {
        const Txn = handler.Txn;
        var txn: Txn = .{
            .conn = state.conn,
            .message = message,
            .raw_payload = raw_payload,
            .rpc_id = rpc_id,
            .response = null,
            .responded_raw = false,
        };

        var event_queue: EventQueue = .{ .arena = state.arena.allocator() };

        const target_fn = @field(handler.namespace, handler.name);
        const Args = std.meta.ArgsTuple(@TypeOf(target_fn));

        var args: Args = undefined;
        args[0] = &txn;

        inline for (comptime std.meta.fields(Args)[1..], 1..) |param, i| {
            if (param.type == *EventQueue)
                args[i] = &event_queue
            else if (param.type == quest_types.QuestContext)
                args[i] = state.questContext(gpa, io, &event_queue)
            else if (param.type == RewardService.RewardContext)
                args[i] = state.rewardContext(gpa, io)
            else
                args[i] = try state.extract(gpa, io, param.type);
        }

        try @call(.auto, target_fn, args);

        try logic_handlers.drainEventQueue(&event_queue, state, gpa, io);

        if (Txn.Response != void and !txn.responded_raw) {
            if (txn.response) |response| {
                try txn.conn.respond(gpa, io, rpc_id, response);
            } else log.err("response of type {s} is not set", .{@typeName(Txn.Response)});
        }
    }
};

const DispatchFn = *const fn (*State, std.mem.Allocator, Io, Message, u16) anyerror!void;

fn makeDispatchFn(
    comptime namespace: type,
    comptime decl_name: []const u8,
    comptime MessagePB: type,
    comptime Txn: type,
) DispatchFn {
    return struct {
        fn call(
            state: *State,
            gpa: std.mem.Allocator,
            io: Io,
            message: Message,
            rpc_id: u16,
        ) anyerror!void {
            var reader = Io.Reader.fixed(message.payload);
            const msg = proto.decodeMessage(&reader, state.arena.allocator(), MessagePB, proto.pb_desc) catch |err| {
                log.err("failed to decode message of type {s}: {}", .{ @typeName(MessagePB), err });
                if (std.meta.activeTag(message.header) == .request and Txn.Response != void) {
                    if (state.conn.respond(gpa, io, rpc_id, failResponse(Txn.Response))) {
                        log.warn("sent error response for failed decode {s} (rpc_id: {d})", .{ @typeName(MessagePB), rpc_id });
                    } else |send_err| {
                        log.warn("failed to send error response for decode error {s} (rpc_id: {d}): {t}", .{ @typeName(MessagePB), rpc_id, send_err });
                    }
                }
                return;
            };

            const handler: NetEventHandler = .{
                .namespace = namespace,
                .Message = MessagePB,
                .Txn = Txn,
                .name = decl_name,
            };
            handler.invoke(state, gpa, io, msg, message.payload, rpc_id) catch |err| switch (err) {
                error.Closed, error.Canceled => return err,
                else => {
                    log.err("failed to handle message of type {s}: {}", .{ @typeName(MessagePB), err });
                    if (std.meta.activeTag(message.header) == .request and Txn.Response != void) {
                        if (state.conn.respond(gpa, io, rpc_id, failResponse(Txn.Response))) {
                            log.warn("sent error response for failed request {s} (rpc_id: {d})", .{ @typeName(MessagePB), rpc_id });
                        } else |send_err| {
                            log.warn("failed to send error response for failed request {s} (rpc_id: {d}): {t}", .{ @typeName(MessagePB), rpc_id, send_err });
                        }
                    }
                    return;
                },
            };
        }
    }.call;
}

const handler_lut: [32768]?DispatchFn = build_lut: {
    @setEvalBranchQuota(200_000);
    var lut: [32768]?DispatchFn = @splat(null);

    for (net_namespaces) |namespace| {
        for (std.meta.declarations(namespace)) |decl| {
            switch (@typeInfo(@TypeOf(@field(namespace, decl.name)))) {
                .@"fn" => |fn_info| {
                    if (fn_info.params.len == 0) continue;
                    const param_type = fn_info.params[0].type orelse continue;
                    if (@typeInfo(param_type) != .pointer) continue;
                    const Txn = @typeInfo(param_type).pointer.child;
                    if (!@hasDecl(Txn, "MessagePB")) continue;

                    const MessagePB = Txn.MessagePB;
                    const message_name = @typeName(MessagePB)[3..];
                    if (!@hasDecl(proto.pb_desc, message_name)) continue;

                    const message_desc = @field(proto.pb_desc, message_name);
                    if (!@hasDecl(message_desc, "msg_id")) continue;

                    const msg_id: u16 = message_desc.msg_id;
                    if (msg_id < lut.len and lut[msg_id] == null) {
                        lut[msg_id] = makeDispatchFn(namespace, decl.name, MessagePB, Txn);
                    }
                },
                else => {},
            }
        }
    }

    break :build_lut lut;
};

fn akiRewardReason(resp_msg_id: u16) i32 {
    const desc = proto.pb_desc;
    return switch (resp_msg_id) {
        desc.AchievementReceiveResponse.msg_id,
        desc.MulAchievementReceiveResponse.msg_id,
        => 19100, // ACHIEVEMENT_RESON -> RewardViewId 1009
        desc.BattlePassTakeRewardResponse.msg_id,
        desc.BattlePassTakeAllRewardResponse.msg_id,
        desc.BattlePassTaskTakeResponse.msg_id,
        => 18500, // BATTLE_PASS_REWARD_REASON -> RewardViewId 1009
        desc.LivenessTakeResponse.msg_id,
        desc.LivenessTaskTakeResponse.msg_id,
        => 19600, // DAILY_ACTIVITY_REWARD_REASON -> RewardViewId 1009
        desc.PayShopDirectBuyResponse.msg_id,
        => 8001, // PAY_REASON -> RewardViewId 1009
        desc.GetAdventureRewardResponse.msg_id,
        => 19500, // AdventureGuide reward -> RewardViewId 1009
        else => 10001, // General RewardViewId 1009
    };
}

fn deliverAkiResponse(
    state: *State,
    gpa: std.mem.Allocator,
    io: Io,
    rpc_id: u16,
    resp: aki_logic_bridge.LogicResponse,
) !void {
    if (resp.grants.len > 0) {
        // MonthCardRequest runs during OnLoadingNetDataDone before WorldDone; opening CommonRewardView
        // mid-loading locks UI input behind LoadingView / NewcomerSplashView.
        const show_popup = resp.msg_id != proto.pb_desc.MonthCardResponse.msg_id;
        // DropPackage 2105 (ShowBg = false) avoids null DropPackage.Value in ItemHintController
        // while still emitting OnItemRewardNotify -> ItemRewardController.OpenCommonRewardView(1009).
        _ = try RewardService.grantItems(
            state.rewardContext(gpa, io),
            2105,
            show_popup,
            resp.grants,
            akiRewardReason(resp.msg_id),
        );
    }
    for (resp.pushes) |p| {
        try state.conn.pushRawWithMessageId(gpa, io, p.msg_id, p.payload);
    }
    try state.conn.respondRawWithMessageId(gpa, io, rpc_id, resp.msg_id, resp.payload);
}

pub fn dispatchMessage(
    state: *State,
    gpa: std.mem.Allocator,
    io: Io,
    message: Message,
) !void {
    const msg_id = message.header.getMessageId();
    const rpc_id = switch (message.header) {
        .request => |request| request.rpc_id,
        else => 0,
    };

    // 1. UNIVERSAL PRIMARY: Delegate request to AkiLogicService (.NET 9 Logic Engine + SQLite + Kuro DLLs)
    // Exclude network heartbeats (ID 29312 / 1650), TimeCheckRequest, and ActivityRequest (16076, which activity.zig patches with PreConditionGroupId).
    if (std.meta.activeTag(message.header) == .request and
        msg_id <= std.math.maxInt(u16) and
        msg_id != 29312 and
        msg_id != proto.pb_desc.HeartbeatRequest.msg_id and
        msg_id != proto.pb_desc.TimeCheckRequest.msg_id and
        msg_id != 16076)
    {
        if (aki_logic_bridge.callPlayer(io, state.arena.allocator(), state.player_id.id, 0, @intCast(msg_id), message.payload)) |resp| {
            if (resp.msg_id != 0) {
                try deliverAkiResponse(state, gpa, io, rpc_id, resp);
                return;
            }
        } else |_| {
            // AkiLogicService offline or unhandled: fallback immediately to Zig
        }
    }

    // 2. FALLBACK / SECONDARY: Native Zig handler from LUT
    if (if (msg_id < handler_lut.len) handler_lut[msg_id] else null) |dispatch_fn| {
        return dispatch_fn(state, gpa, io, message, rpc_id);
    }

    return error.HandlerNotFound;
}
