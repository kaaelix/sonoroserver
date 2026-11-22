const std = @import("std");
const mem = @import("../../mem.zig");
const common = @import("common");
const FileSystem = common.FileSystem;
const Connection = @import("../../network/Connection.zig");
const Assets = @import("../../data/Assets.zig");
const Scene = @import("../Scene.zig");
const EventQueue = @import("../EventQueue.zig");
const PlayerBasicComponent = @import("../component/player/PlayerBasicComponent.zig");
const PlayerInventoryComponent = @import("../component/player/PlayerInventoryComponent.zig");
const PlayerWeaponComponent = @import("../component/player/PlayerWeaponComponent.zig");
const PlayerEchoComponent = @import("../component/player/PlayerEchoComponent.zig");
const PlayerMapComponent = @import("../component/player/PlayerMapComponent.zig");
const PlayerQuestComponent = @import("../component/player/PlayerQuestComponent.zig");
const PlayerGuideComponent = @import("../component/player/PlayerGuideComponent.zig");
const PlayerSceneComponent = @import("../component/player/PlayerSceneComponent.zig");
const PlayerRoleComponent = @import("../component/player/PlayerRoleComponent.zig");
const PlayerMailComponent = @import("../component/player/PlayerMailComponent.zig");
const PlayerProgressComponent = @import("../component/player/PlayerProgressComponent.zig");
const ConditionEvaluator = @import("condition.zig");
const RewardService = @import("reward.zig");

/// Shared execution context passed across quest evaluation, flow execution, and action helpers.
pub const QuestContext = struct {
    assets: *const Assets,
    fs: *FileSystem,
    conn: *Connection,
    scene: ?*Scene = null,
    scene_comp: ?*PlayerSceneComponent = null,
    events: *EventQueue,
    alloc: mem.Alloc,
    io: std.Io,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    map_comp: *PlayerMapComponent,
    quest_comp: *PlayerQuestComponent,
    guide_comp: ?*PlayerGuideComponent = null,
    role_comp: ?*PlayerRoleComponent = null,
    mail_comp: ?*PlayerMailComponent = null,
    progress_comp: ?*PlayerProgressComponent = null,
};

/// Decouples quest progression triggers from `quest_actions` to prevent circular imports.
pub const ActionCallbacks = struct {
    start_flow: ?*const fn (
        ctx: QuestContext,
        quest_id: i32,
        node_id: i32,
        obj: std.json.ObjectMap,
        enqueue_if_busy: bool,
    ) anyerror!void = null,
    start_flow_state: ?*const fn (
        ctx: QuestContext,
        quest_id: i32,
        node_id: i32,
        flow_name: []const u8,
        flow_id: i32,
        state_id: i32,
        enqueue_if_busy: bool,
    ) anyerror!void = null,
    complete_guide: ?*const fn (ctx: QuestContext, guide_id: i32) anyerror!void = null,
    on_entity_state_changed: ?*const fn (ctx: QuestContext, entity_config_id: i32, state_name: []const u8) anyerror!void = null,
    on_phone_message: ?*const fn (ctx: QuestContext, phone_message_id: i32, telecom_id: i32) anyerror!void = null,
    on_npc_mail: ?*const fn (ctx: QuestContext, mail_id: i32) anyerror!void = null,
    on_time_changed: ?*const fn (ctx: QuestContext, hour: i32, minute: i32) anyerror!void = null,
    on_weather_changed: ?*const fn (ctx: QuestContext, weather_id: i32) anyerror!void = null,
    on_item_gained: ?*const fn (ctx: QuestContext, item_id: i32) anyerror!void = null,
    on_level_play_enabled: ?*const fn (ctx: QuestContext, level_play_id: i32, enabled: bool) anyerror!void = null,
    on_accept_focus_wait: ?*const fn (ctx: QuestContext) anyerror!void = null,
    on_finish_flow_state: ?*const fn (ctx: QuestContext) anyerror!void = null,
    on_destroy_quest: ?*const fn (ctx: QuestContext, quest_id: i32) anyerror!void = null,
    on_finish_condition: ?*const fn (ctx: QuestContext, quest_id: i32, node_id: i32) anyerror!void = null,
    on_trace_current_quest: ?*const fn (ctx: QuestContext, quest_id: i32) anyerror!void = null,
};

pub fn makeEvalContext(ctx: QuestContext) ConditionEvaluator.EvalContext {
    return makeEvalContextForRole(ctx, 0);
}

pub fn makeEvalContextForQuest(ctx: QuestContext, quest_id: i32) ConditionEvaluator.EvalContext {
    var eval_ctx = makeEvalContextForRole(ctx, 0);
    eval_ctx.owner_quest_id = quest_id;
    return eval_ctx;
}

pub fn makeEvalContextForRole(ctx: QuestContext, target_role_id: i32) ConditionEvaluator.EvalContext {
    return .{
        .assets = ctx.assets,
        .basic_comp = ctx.basic_comp,
        .echo_comp = ctx.echo_comp,
        .inventory_comp = ctx.inventory_comp,
        .map_comp = ctx.map_comp,
        .quest_comp = ctx.quest_comp,
        .role_comp = ctx.role_comp,
        .guide_comp = ctx.guide_comp,
        .progress_comp = ctx.progress_comp,
        .scene = ctx.scene,
        .target_role_id = target_role_id,
    };
}

pub fn makeRewardContext(ctx: QuestContext) RewardService.RewardContext {
    return .{
        .alloc = ctx.alloc,
        .io = ctx.io,
        .fs = ctx.fs,
        .assets = ctx.assets,
        .conn = ctx.conn,
        .basic_comp = ctx.basic_comp,
        .inventory_comp = ctx.inventory_comp,
        .weapon_comp = ctx.weapon_comp,
        .echo_comp = ctx.echo_comp,
    };
}

pub fn jsonStr(v_opt: ?std.json.Value) ?[]const u8 {
    const v = v_opt orelse return null;
    return switch (v) {
        .string => |s| s,
        else => null,
    };
}

pub fn jsonInt(v_opt: ?std.json.Value) ?i32 {
    const v = v_opt orelse return null;
    return switch (v) {
        .integer => |i| std.math.cast(i32, i),
        .float => |f| {
            if (!std.math.isFinite(f)) return null;
            const min_f: f64 = @floatFromInt(std.math.minInt(i32));
            const max_f: f64 = @floatFromInt(std.math.maxInt(i32));
            if (f < min_f or f > max_f) return null;
            return @intFromFloat(f);
        },
        .string => |s| std.fmt.parseInt(i32, s, 10) catch null,
        .bool => |b| if (b) @as(i32, 1) else 0,
        else => null,
    };
}

pub fn jsonI64(v_opt: ?std.json.Value) ?i64 {
    const v = v_opt orelse return null;
    return switch (v) {
        .integer => |i| i,
        .float => |f| {
            if (!std.math.isFinite(f)) return null;
            const min_f: f64 = @floatFromInt(std.math.minInt(i64));
            const max_f: f64 = @floatFromInt(std.math.maxInt(i64));
            if (f <= min_f or f >= max_f) return null;
            return @intFromFloat(f);
        },
        .string => |s| std.fmt.parseInt(i64, s, 10) catch null,
        else => null,
    };
}

pub fn jsonFloat(v_opt: ?std.json.Value) ?f32 {
    const v = v_opt orelse return null;
    return switch (v) {
        .float => |f| @floatCast(f),
        .integer => |i| @floatFromInt(i),
        else => null,
    };
}

pub fn jsonBool(v_opt: ?std.json.Value) ?bool {
    const v = v_opt orelse return null;
    return switch (v) {
        .bool => |b| b,
        .integer => |i| i != 0,
        else => null,
    };
}

pub fn resolveVarInt(ctx: QuestContext, v_opt: ?std.json.Value) i32 {
    const v = v_opt orelse return 0;
    if (v != .object) return 0;
    const src = jsonStr(v.object.get("Source")) orelse "Constant";
    if (std.mem.eql(u8, src, "Constant")) {
        return jsonInt(v.object.get("Value")) orelse 0;
    }
    if (jsonStr(v.object.get("Name"))) |name| {
        if (jsonStr(v.object.get("Type"))) |var_ty| {
            if (std.mem.eql(u8, var_ty, "Bool")) {
                return if (ctx.quest_comp.info.getVarBool(name)) 1 else 0;
            }
        }
        return ctx.quest_comp.info.getVarInt(name);
    }
    return 0;
}

pub fn parseIntList(arena: std.mem.Allocator, val_opt: ?std.json.Value) ![]const i32 {
    const arr = val_opt orelse return &.{};
    if (arr != .array or arr.array.items.len == 0) return &.{};
    var out: std.ArrayList(i32) = .empty;
    for (arr.array.items) |item| {
        if (jsonInt(item)) |id| try out.append(arena, id);
    }
    return out.items;
}

pub fn parseStrList(arena: std.mem.Allocator, val_opt: ?std.json.Value) ![]const []const u8 {
    const arr = val_opt orelse return &.{};
    if (arr != .array or arr.array.items.len == 0) return &.{};
    var out: std.ArrayList([]const u8) = .empty;
    for (arr.array.items) |item| {
        if (jsonStr(item)) |s| {
            if (s.len != 0) try out.append(arena, s);
        }
    }
    return out.items;
}
