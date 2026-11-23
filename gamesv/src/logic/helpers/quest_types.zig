const std = @import("std");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const common = @import("common");
const FileSystem = common.FileSystem;
const Connection = @import("../../network/Connection.zig");
const Assets = @import("../../data/Assets.zig");
const LazyNodeStore = @import("../../data/LazyNodeStore.zig");
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

pub const DeferredBtVar = struct {
    owner_id: i32,
    var_name: []const u8,
    var_define: pb.VarDefinePb,
};

/// Decouples quest progression triggers from `quest_actions` to prevent circular imports.
pub const ActionCallbacks = struct {
    start_flow: ?*const fn (
        ctx: QuestContext,
        quest_id: i32,
        node_id: i32,
        obj: std.json.ObjectMap,
        enqueue_if_busy: bool,
        is_async_override: ?bool,
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
    on_var_changed: ?*const fn (ctx: QuestContext, owner_id: i32, var_name: []const u8) anyerror!void = null,
    bt_type: pb.BtType = .BtTypeQuest,
    is_flow_action: bool = false,
    defer_flow_transition: bool = false,
    deferred_bt_vars: ?*std.ArrayList(DeferredBtVar) = null,
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
        .progress_comp = ctx.progress_comp,
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

pub fn resolveVarName(obj: std.json.ObjectMap) ?[]const u8 {
    if (jsonStr(obj.get("Name"))) |n| {
        if (n.len > 0) return n;
    }
    if (jsonStr(obj.get("Keyword"))) |k| {
        if (k.len > 0) return k;
    }
    return null;
}

pub fn scopedVarKeyAlloc(arena: std.mem.Allocator, obj: std.json.ObjectMap, base_name: []const u8) ?[]const u8 {
    return scopedVarKeyWithOwnerAlloc(arena, null, 0, obj, base_name);
}

/// Scopes `Source == "Self"` variables cleanly across quests, entities, and ambient NPC flows so NPC flow variables never collide globally.
pub fn scopedVarKeyWithOwnerAlloc(
    arena: std.mem.Allocator,
    scene_opt: ?*const Scene,
    owner_id: i32,
    obj: std.json.ObjectMap,
    base_name: []const u8,
) ?[]const u8 {
    const src = jsonStr(obj.get("Source")) orelse "";
    if (std.mem.eql(u8, src, "Global")) {
        return std.fmt.allocPrint(arena, "var:{s}", .{base_name}) catch null;
    }
    if (jsonInt(obj.get("RefId"))) |ref_id| {
        if (ref_id > 0) {
            const ref_type = jsonStr(obj.get("RefType")) orelse "Entity";
            return std.fmt.allocPrint(arena, "{s}:{d}:{s}", .{ ref_type, ref_id, base_name }) catch null;
        }
    }
    if (owner_id > 0) {
        return std.fmt.allocPrint(arena, "Self:{d}:{s}", .{ owner_id, base_name }) catch null;
    }
    if (scene_opt) |scene| {
        if (scene.active_flow_owner_entity_id != 0) {
            return std.fmt.allocPrint(arena, "EntSelf:{d}:{s}", .{ scene.active_flow_owner_entity_id, base_name }) catch null;
        }
        if (scene.active_flow_id > 0) {
            return std.fmt.allocPrint(arena, "FlowSelf:{d}:{s}", .{ scene.active_flow_id, base_name }) catch null;
        }
    }
    return null;
}

pub fn resolveVarInt(ctx: QuestContext, v_opt: ?std.json.Value) i32 {
    return resolveVarIntForOwner(ctx, 0, v_opt);
}

pub fn resolveVarIntForOwner(ctx: QuestContext, owner_id: i32, v_opt: ?std.json.Value) i32 {
    const v = v_opt orelse return 0;
    if (v != .object) return 0;
    const src = jsonStr(v.object.get("Source")) orelse "Constant";
    if (std.mem.eql(u8, src, "Constant")) {
        return jsonInt(v.object.get("Value")) orelse 0;
    }
    if (resolveVarName(v.object)) |name| {
        const is_bool = if (jsonStr(v.object.get("Type"))) |var_ty|
            std.mem.eql(u8, var_ty, "Bool") or std.mem.eql(u8, var_ty, "Boolean")
        else
            false;
        if (std.mem.eql(u8, src, "Global")) {
            if (ctx.progress_comp) |pc| {
                if (scopedVarKeyAlloc(ctx.alloc.arena, v.object, name)) |gkey| {
                    for (pc.info.dynamic.ints) |entry| {
                        if (std.mem.eql(u8, entry.key, gkey)) {
                            return @intCast(std.math.clamp(entry.val, std.math.minInt(i32), std.math.maxInt(i32)));
                        }
                    }
                }
            }
        }
        if (scopedVarKeyWithOwnerAlloc(ctx.alloc.arena, ctx.scene, owner_id, v.object, name)) |skey| {
            if (ctx.quest_comp.info.hasVar(skey)) {
                if (is_bool) return if (ctx.quest_comp.info.getVarBool(skey)) 1 else 0;
                return ctx.quest_comp.info.getVarInt(skey);
            }
        }
        if (is_bool) {
            return if (ctx.quest_comp.info.getVarBool(name)) 1 else 0;
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

/// Builds a unique 64-bit cooldown key from an owner ID (positive for LevelPlay, negative for Quest) and node ID.
pub fn repeaterCooldownKey(owner_id: i32, node_id: i32) i64 {
    return (@as(i64, owner_id) << 32) | @as(i64, @as(u32, @bitCast(node_id)));
}

fn waitActionDelayMs(act_val: std.json.Value) i64 {
    if (act_val != .object) return 0;
    const name = jsonStr(act_val.object.get("Name")) orelse return 0;
    if (!std.mem.eql(u8, name, "Wait")) return 0;
    const p = act_val.object.get("Params") orelse return 0;
    if (p != .object) return 0;
    const secs = jsonFloat(p.object.get("Time")) orelse return 0;
    if (secs <= 0) return 0;
    return @max(@as(i64, 200), @as(i64, @intFromFloat(secs * 1000.0)));
}

pub fn actionListWaitDelayMs(actions_opt: ?std.json.Value) i64 {
    const av = actions_opt orelse return 0;
    if (av != .array) return 0;
    var total_ms: i64 = 0;
    for (av.array.items) |act_val| {
        total_ms += waitActionDelayMs(act_val);
    }
    return total_ms;
}

/// Sums any `Wait` action durations under a Repeater subtree so timer-loop nodes throttle to their Wait cadence.
pub fn subtreeWaitDelayMs(nodes: []const LazyNodeStore.NodeEntry, parent_id: i32) i64 {
    var total_ms: i64 = 0;
    for (nodes) |child| {
        if (child.parent_id != parent_id) continue;
        if (child.data == .object) {
            total_ms += actionListWaitDelayMs(child.data.object.get("Actions"));
            total_ms += actionListWaitDelayMs(child.data.object.get("FinishActions"));
            if (child.data.object.get("Action")) |sv| {
                total_ms += waitActionDelayMs(sv);
            }
        }
        total_ms += subtreeWaitDelayMs(nodes, child.id);
    }
    return total_ms;
}
