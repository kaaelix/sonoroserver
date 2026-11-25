const std = @import("std");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const dispatch = @import("combat.zig");
const Scene = @import("../../logic/Scene.zig");
const Connection = @import("../../network/Connection.zig");
const Entity = @import("../../logic/Scene.zig").Entity;
const Assets = @import("../../data/Assets.zig");
const FileSystem = @import("common").FileSystem;
const Transaction = @import("../handlers.zig").Transaction;
const buff_helper = @import("../../logic/helpers/buff.zig");
const gameplay_tags = @import("../../logic/helpers/gameplay_tags.zig");
const BuffTimerScheduler = @import("../../logic/schedulers/BuffTimerScheduler.zig");
const EventQueue = @import("../../logic/EventQueue.zig");
const Events = @import("../../logic/events.zig");
const attributes_helper = @import("../../logic/helpers/attributes.zig");

pub fn onAttributeChangedRequest(
    txn: *Transaction(pb.AttributeChangedRequest),
    scene: *Scene,
    fs: *FileSystem,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    const idx = scene.net_id_map.get(txn.message.Id) orelse {
        txn.respond(.{ .ErrorCode = .RequestParamError });
        return;
    };
    const slice = scene.entities.slice();
    const attr_ptr = &(slice.items(.attribute)[idx] orelse {
        txn.respond(.{ .ErrorCode = .Success });
        return;
    });

    var changed: std.ArrayList(pb.EAttributeType) = .empty;
    for (txn.message.Attributes.items) |attr_pb| {
        const attr_type = attr_pb.AttributeType orelse continue;
        const ch = try attributes_helper.change_attr(attr_ptr, attr_type, .Override, .Current, attr_pb.CurrentValue, alloc);
        try changed.appendSlice(alloc.arena, ch.items);
    }

    if (changed.items.len != 0) {
        const entity = Entity{ .net_id = txn.message.Id, .index = idx };
        var pack: std.ArrayList(pb.CombatReceiveData) = .empty;
        try attributes_helper.generate_attr_messages(&pack, txn.message.Id, attr_ptr, &changed, alloc, io);
        if (pack.items.len != 0) {
            try txn.conn.push(alloc.gpa, io, pb.CombatReceivePackNotify{ .Data = pack });
        }
        try scene.markFsmDirty(alloc.gpa, txn.message.Id, Entity.FsmComponent.WakeReason.attribute);
        try scene.saveComponents(fs, alloc.gpa, entity, &.{Entity.AttributeComponent});
    }
    txn.respond(.{ .ErrorCode = .Success });
}

const BuffQuery = Scene.Query(&.{
    Entity,
    *Entity.FightBuffComponent,
    ?*Entity.AttributeComponent,
});

const PassiveSkillQuery = Scene.Query(&.{
    Entity,
    ?*Entity.PassiveGaSkillComponent,
});

const max_duration_seconds: f64 =
    @as(f64, @floatFromInt(std.math.maxInt(i64))) / 1000.0;

/// How a buff application reached the server.
const BuffOrigin = enum {
    /// The client asks the server to apply the buff (`OrderApplyBuffRequest`, `ApplyGameplayEffectRequest`).
    client_order,
    /// The client already applied the buff locally and only reports it (`ApplyGameplayEffectPush`).
    client_report,
};

/// Whether the client still has to be told that the buff was applied. A reported buff already
/// exists on the client: announcing it makes the client apply it a second time, and for a buff
/// shared by the formation every extra application is shared and reported again without end.
fn announcesApplication(origin: BuffOrigin) bool {
    return switch (origin) {
        .client_order => true,
        .client_report => false,
    };
}

fn applyBuffRequestImpl(
    origin: BuffOrigin,
    entity_id: i64,
    buff_id: i64,
    instigator_id: i64,
    stack_count: i32,
    is_active: bool,
    duration_seconds: ?f32,
    events: *EventQueue,
    assets: *const Assets,
    scene: *Scene,
    fs: *FileSystem,
    query: BuffQuery,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
    io: std.Io,
    alloc: mem.Alloc,
) !bool {
    const buff_data = assets.tables.buff.getDataById(buff_id) orelse return false;

    switch (buff_data.DurationPolicy) {
        .Instant => {
            if (announcesApplication(origin)) {
                try receive_data_pack.append(alloc.arena, .{ .Message = .{
                    .CombatNotifyData = .{
                        .CombatCommon = .{ .EntityId = entity_id, .MessageId = -1, .PreMessageId = -1 },
                        .Message = .{
                            .ApplyGameplayEffectNotify = .{
                                .Handle = -2,
                                .Id = buff_data.Id,
                                .EntityId = entity_id,
                                .InstigatorId = instigator_id,
                                .IsActive = true,
                            },
                        },
                    },
                } });
            }
            try buff_helper.execute_buff_effects(
                receive_data_pack,
                entity_id,
                instigator_id,
                &buff_data,
                scene,
                fs,
                io,
                query,
                alloc,
            );
            return true;
        },
        else => {
            const target, _, _ = query.byNetId(entity_id) orelse return false;
            const instigator = if (instigator_id != 0)
                if (query.byNetId(instigator_id)) |item| item[0] else target
            else
                target;

            const buff_entries = try alloc.arena.alloc(Events.BuffAdditionEntry, 1);
            buff_entries[0] = .{
                .id = buff_id,
                .stack_count = stack_count,
                .is_active = is_active,
                .duration_seconds = duration_seconds,
            };
            try events.enqueue(.buff_addition, .{
                .target = target,
                .instigator = instigator,
                .buffs = buff_entries,
            });
            return true;
        },
    }
}

pub fn OrderApplyBuffRequest(
    txn: *dispatch.CombatRequestTxn(.OrderApplyBuffRequest),
    events: *EventQueue,
    assets: *const Assets,
    scene: *Scene,
    fs: *FileSystem,
    query: BuffQuery,
    io: std.Io,
    alloc: mem.Alloc,
) !void {
    const entity_id = commonEntityId(txn.common) orelse {
        txn.respond(.{ .ErrorCode = .ErrOrderApplyBuffFailed });
        return;
    };
    const request: pb.OrderApplyBuffRequest = txn.payload;
    const duration_seconds = requestDuration(request.Time) catch {
        txn.respond(.{ .ErrorCode = .ErrOrderApplyBuffFailed });
        return;
    };

    var combat_receive_pack: std.ArrayList(pb.CombatReceiveData) = .empty;
    const ok = try applyBuffRequestImpl(
        .client_order,
        entity_id,
        request.Id,
        request.InstigatorId,
        request.StackCount,
        true,
        duration_seconds,
        events,
        assets,
        scene,
        fs,
        query,
        &combat_receive_pack,
        io,
        alloc,
    );
    if (!ok) {
        txn.respond(.{ .ErrorCode = .ErrOrderApplyBuffFailed });
        return;
    }

    try txn.receive_data_pack.appendSlice(alloc.arena, combat_receive_pack.items);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn ApplyGameplayEffectRequest(
    txn: *dispatch.CombatRequestTxn(.ApplyGameplayEffectRequest),
    events: *EventQueue,
    assets: *const Assets,
    scene: *Scene,
    fs: *FileSystem,
    query: BuffQuery,
    io: std.Io,
    alloc: mem.Alloc,
) !void {
    const entity_id = commonEntityId(txn.common) orelse {
        txn.respond(.{ .ErrorCode = .ErrOrderApplyBuffFailed });
        return;
    };
    const request: pb.ApplyGameplayEffectRequest = txn.payload;
    const dur: ?f32 = if (request.Time) |value| switch (value) {
        .Duration => |duration| if (!std.math.isFinite(duration) or
            (duration > 0 and @as(f64, duration) > max_duration_seconds))
        {
            txn.respond(.{ .ErrorCode = .ErrOrderApplyBuffFailed });
            return;
        } else duration,
    } else null;

    var combat_receive_pack: std.ArrayList(pb.CombatReceiveData) = .empty;
    const ok = try applyBuffRequestImpl(
        .client_order,
        entity_id,
        request.Id,
        request.InstigatorId,
        request.StackCount,
        request.IsActive,
        dur,
        events,
        assets,
        scene,
        fs,
        query,
        &combat_receive_pack,
        io,
        alloc,
    );
    if (!ok) {
        txn.respond(.{ .ErrorCode = .ErrOrderApplyBuffFailed });
        return;
    }

    try txn.receive_data_pack.appendSlice(alloc.arena, combat_receive_pack.items);
    txn.respond(.{ .ErrorCode = .Success });
}

fn requestDuration(time: @FieldType(pb.OrderApplyBuffRequest, "Time")) !?f32 {
    const duration = if (time) |value| switch (value) {
        .Duration => |duration| duration,
    } else return null;

    if (!std.math.isFinite(duration) or
        (duration > 0 and @as(f64, duration) > max_duration_seconds))
    {
        return error.InvalidBuffDuration;
    }
    return duration;
}

pub fn ApplyGameplayEffectPush(
    push: pb.ApplyGameplayEffectPush,
    common: ?pb.CombatCommon,
    events: *EventQueue,
    assets: *const Assets,
    scene: *Scene,
    fs: *FileSystem,
    query: BuffQuery,
    io: std.Io,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    const entity_id = commonEntityId(common) orelse return;
    const duration_seconds: ?f32 = if (push.Time) |t| blk: {
        const dur = t.Duration;
        if (!std.math.isFinite(dur) or (dur > 0 and @as(f64, dur) > max_duration_seconds)) return;
        break :blk dur;
    } else null;
    const stack_count = if (push.StackCount > 0) push.StackCount else 1;
    _ = try applyBuffRequestImpl(
        .client_report,
        entity_id,
        push.Id,
        push.InstigatorId,
        stack_count,
        push.IsActive,
        duration_seconds,
        events,
        assets,
        scene,
        fs,
        query,
        receive_data_pack,
        io,
        alloc,
    );
}

pub fn OrderRemoveBuffRequest(
    txn: *dispatch.CombatRequestTxn(.OrderRemoveBuffRequest),
    events: *EventQueue,
    scene: *Scene,
    assets: *const Assets,
    buff_timers: *BuffTimerScheduler,
    query: BuffQuery,
    io: std.Io,
    alloc: mem.Alloc,
) !void {
    const entity_id = commonEntityId(txn.common) orelse {
        txn.respond(.{ .ErrorCode = .ErrOrderRemoveBuffEntityNotExists });
        return;
    };
    const item = query.byNetId(entity_id) orelse {
        txn.respond(.{ .ErrorCode = .ErrOrderRemoveBuffEntityNotExists });
        return;
    };

    const entity, const buffs, const maybe_attr = item;
    const is_none_stacking = if (assets.tables.buff.getDataById(txn.payload.Id)) |cfg| cfg.StackingType == .None else false;
    const remove_count = txn.payload.StackCount;
    if (is_none_stacking and remove_count <= 0) {
        var handles: std.ArrayList(i32) = .empty;
        for (buffs.fight_buff_infos) |b| {
            if (b.BuffId == txn.payload.Id) try handles.append(alloc.arena, b.HandleId);
        }
        for (handles.items) |hid| {
            try removeBuffHandleInPack(entity, hid, txn.payload.InstigatorId, true, buffs, maybe_attr, scene, assets, buff_timers, events, txn.receive_data_pack, io, alloc);
        }
    } else if (buffs.getByBuffId(txn.payload.Id)) |buff| {
        if (remove_count <= 0 or remove_count >= buff.StackCount) {
            try removeBuffHandleInPack(entity, buff.HandleId, txn.payload.InstigatorId, true, buffs, maybe_attr, scene, assets, buff_timers, events, txn.receive_data_pack, io, alloc);
        } else {
            buff.StackCount -= remove_count;
            try appendStackCountNotifyInPack(entity.net_id, buff.HandleId, buff.StackCount, txn.payload.InstigatorId, -1, txn.receive_data_pack, alloc);
            if (assets.tables.buff.getDataById(buff.BuffId)) |buff_data| {
                try buff_helper.syncBuffShield(txn.receive_data_pack, scene, assets, entity.net_id, buffs, maybe_attr, buff.*, &buff_data, alloc, io);
            }
            if (maybe_attr) |attr| {
                try buff_helper.syncContinuousBuffModifiers(txn.receive_data_pack, scene, entity.net_id, buffs, attr, assets, alloc, io);
            }
            try events.enqueue(.buff_change, .{ .entity = entity });
        }
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn OrderRemoveBuffByTagsRequest(
    txn: *dispatch.CombatRequestTxn(.OrderRemoveBuffByTagsRequest),
    assets: *const Assets,
    scene: *Scene,
    events: *EventQueue,
    buff_timers: *BuffTimerScheduler,
    query: BuffQuery,
    io: std.Io,
    alloc: mem.Alloc,
) !void {
    const entity_id = commonEntityId(txn.common) orelse {
        txn.respond(.{ .ErrorCode = .ErrOrderRemoveBuffEntityNotExists });
        return;
    };
    const item = query.byNetId(entity_id) orelse {
        txn.respond(.{ .ErrorCode = .ErrOrderRemoveBuffEntityNotExists });
        return;
    };

    const entity, const buffs, const maybe_attr = item;
    if (txn.payload.TagIds.items.len > 0 and buffs.fight_buff_infos.len > 0) {
        var handles_to_remove: std.ArrayList(i32) = .empty;
        for (buffs.fight_buff_infos) |buff_info| {
            const buff_data = assets.tables.buff.getDataById(buff_info.BuffId) orelse continue;
            if (buffHasAnyTag(&assets.tables, buff_data.GrantedTags, txn.payload.TagIds.items)) {
                try handles_to_remove.append(alloc.arena, buff_info.HandleId);
            }
        }
        for (handles_to_remove.items) |handle_id| {
            try removeBuffHandleInPack(entity, handle_id, txn.payload.InstigatorId, true, buffs, maybe_attr, scene, assets, buff_timers, events, txn.receive_data_pack, io, alloc);
        }
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn RemoveGameplayEffectRequest(
    txn: *dispatch.CombatRequestTxn(.RemoveGameplayEffectRequest),
    events: *EventQueue,
    scene: *Scene,
    assets: *const Assets,
    buff_timers: *BuffTimerScheduler,
    query: BuffQuery,
    io: std.Io,
    alloc: mem.Alloc,
) !void {
    const req = txn.payload;
    const entity_id = if (req.EntityId != 0) req.EntityId else commonEntityId(txn.common) orelse {
        txn.respond(.{ .ErrorCode = .ErrOrderRemoveBuffEntityNotExists, .Handle = req.Handle });
        return;
    };
    const item = query.byNetId(entity_id) orelse {
        txn.respond(.{ .ErrorCode = .ErrOrderRemoveBuffEntityNotExists, .Handle = req.Handle });
        return;
    };
    const entity, const buffs, const maybe_attr = item;
    try removeBuffHandleInPack(entity, req.Handle, 0, req.IsPrematureRemoval, buffs, maybe_attr, scene, assets, buff_timers, events, txn.receive_data_pack, io, alloc);
    txn.respond(.{ .ErrorCode = .Success, .Handle = req.Handle });
}

pub fn RemoveGameplayEffectPush(
    push: pb.RemoveGameplayEffectPush,
    common: ?pb.CombatCommon,
    events: *EventQueue,
    scene: *Scene,
    assets: *const Assets,
    buff_timers: *BuffTimerScheduler,
    query: BuffQuery,
    io: std.Io,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    const entity_id = if (push.EntityId != 0) push.EntityId else commonEntityId(common) orelse return;
    const item = query.byNetId(entity_id) orelse return;
    const entity, const buffs, const maybe_attr = item;
    try removeBuffHandleInPack(entity, push.Handle, push.InstigatorId, push.IsPrematureRemoval, buffs, maybe_attr, scene, assets, buff_timers, events, receive_data_pack, io, alloc);
}

fn setBuffActiveImpl(
    entity_id: i64,
    handle: i32,
    on: bool,
    events: *EventQueue,
    scene: *Scene,
    assets: *const Assets,
    query: BuffQuery,
    io: std.Io,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    const item = query.byNetId(entity_id) orelse return;
    const entity, const buffs, const maybe_attr = item;
    const buff = buffs.getByHandleId(handle) orelse return;
    if (buff.IsActive == on) return;

    buff.IsActive = on;
    try receive_data_pack.append(alloc.arena, .{ .Message = .{
        .CombatNotifyData = .{
            .CombatCommon = .{ .EntityId = entity.net_id },
            .Message = .{
                .ActivateBuffNotify = .{
                    .Handle = handle,
                    .On = on,
                },
            },
        },
    } });
    if (assets.tables.buff.getDataById(buff.BuffId)) |buff_data| {
        try buff_helper.syncBuffShield(receive_data_pack, scene, assets, entity.net_id, buffs, maybe_attr, buff.*, &buff_data, alloc, io);
    }
    if (maybe_attr) |attr| {
        try buff_helper.syncContinuousBuffModifiers(receive_data_pack, scene, entity.net_id, buffs, attr, assets, alloc, io);
    }
    try events.enqueue(.buff_change, .{ .entity = entity });
}

pub fn ActivateBuffRequest(
    txn: *dispatch.CombatRequestTxn(.ActivateBuffRequest),
    events: *EventQueue,
    scene: *Scene,
    assets: *const Assets,
    query: BuffQuery,
    io: std.Io,
    alloc: mem.Alloc,
) !void {
    if (commonEntityId(txn.common)) |entity_id| {
        try setBuffActiveImpl(entity_id, txn.payload.Handle, txn.payload.On, events, scene, assets, query, io, alloc, txn.receive_data_pack);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn ActiveBuffPush(
    push: pb.ActiveBuffPush,
    common: ?pb.CombatCommon,
    events: *EventQueue,
    scene: *Scene,
    assets: *const Assets,
    query: BuffQuery,
    io: std.Io,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    const entity_id = commonEntityId(common) orelse return;
    try setBuffActiveImpl(entity_id, push.Handle, push.On, events, scene, assets, query, io, alloc, receive_data_pack);
}

fn setBuffStackCountImpl(
    entity_id: i64,
    handle_id: i32,
    new_stack_count: i32,
    instigator_id: i64,
    is_premature: bool,
    not_refresh_duration: bool,
    duration: f32,
    events: *EventQueue,
    scene: *Scene,
    assets: *const Assets,
    buff_timers: *BuffTimerScheduler,
    query: BuffQuery,
    io: std.Io,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    const item = query.byNetId(entity_id) orelse return;
    const entity, const buffs, const maybe_attr = item;

    if (new_stack_count <= 0) {
        try removeBuffHandleInPack(entity, handle_id, instigator_id, is_premature, buffs, maybe_attr, scene, assets, buff_timers, events, receive_data_pack, io, alloc);
        return;
    }

    const buff = buffs.getByHandleId(handle_id) orelse return;
    const maybe_cfg = assets.tables.buff.getDataById(buff.BuffId);
    var final_count = new_stack_count;

    if (maybe_cfg) |buff_data| {
        if (buff_data.StackLimitCount > 0) {
            const limit: i32 = @intCast(@min(buff_data.StackLimitCount, @as(u32, std.math.maxInt(i32))));
            if (new_stack_count > limit) {
                if (buff_data.OverflowEffects.len != 0) {
                    var follow_ups: std.ArrayList(Events.BuffAdditionEntry) = .empty;
                    for (buff_data.OverflowEffects) |ov_id| {
                        if (ov_id != buff_data.Id) try follow_ups.append(alloc.arena, .{ .id = ov_id, .is_active = true });
                    }
                    if (follow_ups.items.len != 0) {
                        const inst = if (instigator_id != 0)
                            (if (query.byNetId(instigator_id)) |ii| ii[0] else entity)
                        else
                            entity;
                        try events.enqueue(.buff_addition, .{ .target = entity, .instigator = inst, .buffs = follow_ups.items });
                    }
                }
                if (buff_data.bClearStackOnOverflow) {
                    try removeBuffHandleInPack(entity, handle_id, instigator_id, is_premature, buffs, maybe_attr, scene, assets, buff_timers, events, receive_data_pack, io, alloc);
                    return;
                }
                if (buff_data.bDenyOverflowApplication and buff.StackCount >= limit) return;
                final_count = limit;
            }
        }
    }

    buff.StackCount = final_count;
    if (!not_refresh_duration and duration > 0) {
        buff.Duration = duration;
        buff.LeftDuration = duration;
        if (maybe_cfg) |buff_data| {
            const now_ms = (std.Io.Clock.awake).now(io).toMilliseconds();
            try buff_timers.refreshBuff(alloc.gpa, buff.*, &buff_data, entity.net_id, now_ms, .{
                .refresh_duration = true,
                .reset_period = false,
                .execute_periodic_now = false,
            });
        }
    }
    try appendStackCountNotifyInPack(entity.net_id, handle_id, final_count, instigator_id, if (!not_refresh_duration and duration > 0) duration else -1, receive_data_pack, alloc);

    if (maybe_cfg) |buff_data| {
        try buff_helper.syncBuffShield(receive_data_pack, scene, assets, entity.net_id, buffs, maybe_attr, buff.*, &buff_data, alloc, io);
    }
    if (maybe_attr) |attr| {
        try buff_helper.syncContinuousBuffModifiers(receive_data_pack, scene, entity.net_id, buffs, attr, assets, alloc, io);
    }
    try events.enqueue(.buff_change, .{ .entity = entity });
}

pub fn BuffStackCountRequest(
    txn: *dispatch.CombatRequestTxn(.BuffStackCountRequest),
    events: *EventQueue,
    scene: *Scene,
    assets: *const Assets,
    buff_timers: *BuffTimerScheduler,
    query: BuffQuery,
    io: std.Io,
    alloc: mem.Alloc,
) !void {
    if (commonEntityId(txn.common)) |entity_id| {
        const req = txn.payload;
        try setBuffStackCountImpl(
            entity_id,
            req.HandleId,
            req.NewStackCount,
            req.InstigatorId,
            req.IsPrematureRemoval,
            true,
            -1,
            events,
            scene,
            assets,
            buff_timers,
            query,
            io,
            alloc,
            txn.receive_data_pack,
        );
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn BuffStackCountPush(
    push: pb.BuffStackCountPush,
    common: ?pb.CombatCommon,
    events: *EventQueue,
    scene: *Scene,
    assets: *const Assets,
    buff_timers: *BuffTimerScheduler,
    query: BuffQuery,
    io: std.Io,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    const entity_id = commonEntityId(common) orelse return;
    try setBuffStackCountImpl(
        entity_id,
        push.HandleId,
        push.NewStackCount,
        push.InstigatorId,
        push.IsPrematureRemoval,
        push.NotRefreshDuration,
        push.Duration,
        events,
        scene,
        assets,
        buff_timers,
        query,
        io,
        alloc,
        receive_data_pack,
    );
}

pub fn RefreshBuffDurationPush(
    push: pb.RefreshBuffDurationPush,
    common: ?pb.CombatCommon,
    events: *EventQueue,
    assets: *const Assets,
    buff_timers: *BuffTimerScheduler,
    query: BuffQuery,
    io: std.Io,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    const entity_id = commonEntityId(common) orelse return;
    const item = query.byNetId(entity_id) orelse return;
    const entity, const buffs, _ = item;
    if (push.BuffIds.items.len == 0 or buffs.fight_buff_infos.len == 0) return;

    const now_ms = (std.Io.Clock.awake).now(io).toMilliseconds();
    var any_changed = false;

    for (buffs.fight_buff_infos) |*buff| {
        if (std.mem.indexOfScalar(i64, push.BuffIds.items, buff.BuffId) == null) continue;
        const buff_data = assets.tables.buff.getDataById(buff.BuffId) orelse continue;
        if (buff_data.DurationPolicy != .HasDuration) continue;
        const dur = if (buff.Duration > 0) buff.Duration else buff_helper.configured_duration_seconds(&buff_data);
        if (dur <= 0) continue;

        buff.Duration = dur;
        buff.LeftDuration = dur;
        try buff_timers.refreshBuff(alloc.gpa, buff.*, &buff_data, entity_id, now_ms, .{
            .refresh_duration = true,
            .reset_period = false,
            .execute_periodic_now = false,
        });
        try receive_data_pack.append(alloc.arena, .{ .Message = .{
            .CombatNotifyData = .{
                .CombatCommon = .{ .EntityId = entity_id },
                .Message = .{
                    .BuffDurationNotify = .{
                        .HandleId = buff.HandleId,
                        .Time = .{ .Duration = dur },
                        .gFs = .{ .LeftDuration = dur },
                    },
                },
            },
        } });
        any_changed = true;
    }

    if (any_changed) {
        try events.enqueue(.buff_change, .{ .entity = entity });
    }
}

fn handlePassiveSkillAdd(
    target_entity_id: i64,
    passive_skill_id: i64,
    query: PassiveSkillQuery,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    const skill_id = std.math.cast(i32, passive_skill_id) orelse return;
    if (skill_id == 0 or target_entity_id == 0) return;
    const item = query.byNetId(target_entity_id) orelse return;
    const entity, const maybe_passive = item;
    const passive = maybe_passive orelse return;
    if (!try passive.addSkill(alloc.gpa, skill_id)) return;

    var list: std.ArrayList(pb.PassiveSkillItemPb) = .empty;
    try list.append(alloc.arena, .{
        .CombatCommon = .{ .EntityId = entity.net_id, .MessageId = -1, .PreMessageId = -1 },
        .SkillId = passive_skill_id,
    });
    try receive_data_pack.append(alloc.arena, .{ .Message = .{
        .CombatNotifyData = .{
            .CombatCommon = .{ .EntityId = entity.net_id, .MessageId = -1, .PreMessageId = -1 },
            .Message = .{ .PassiveSkillAddNotify = .{
                .EntityId = entity.net_id,
                .PassiveSkillItemPbList = list,
            } },
        },
    } });
}

fn handlePassiveSkillRemove(
    target_entity_id: i64,
    passive_skill_id: i64,
    query: PassiveSkillQuery,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    const skill_id = std.math.cast(i32, passive_skill_id) orelse return;
    if (skill_id == 0 or target_entity_id == 0) return;
    const item = query.byNetId(target_entity_id) orelse return;
    const entity, const maybe_passive = item;
    const passive = maybe_passive orelse return;
    if (!passive.removeSkill(alloc.gpa, skill_id)) return;

    var list: std.ArrayList(i64) = .empty;
    try list.append(alloc.arena, passive_skill_id);
    try receive_data_pack.append(alloc.arena, .{ .Message = .{
        .CombatNotifyData = .{
            .CombatCommon = .{ .EntityId = entity.net_id },
            .Message = .{ .PassiveSkillRemoveNotify = .{
                .EntityId = entity.net_id,
                .SkillIdList = list,
            } },
        },
    } });
}

pub fn PassiveSkillAddRequest(
    txn: *dispatch.CombatRequestTxn(.PassiveSkillAddRequest),
    query: PassiveSkillQuery,
    alloc: mem.Alloc,
) !void {
    const tid = if (txn.payload.TargetEntityId != 0) txn.payload.TargetEntityId else commonEntityId(txn.common) orelse 0;
    try handlePassiveSkillAdd(tid, txn.payload.PassiveSkillId, query, alloc, txn.receive_data_pack);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn PassiveSkillAddPush(
    push: pb.PassiveSkillAddPush,
    common: ?pb.CombatCommon,
    query: PassiveSkillQuery,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    const tid = if (push.TargetEntityId != 0) push.TargetEntityId else commonEntityId(common) orelse 0;
    try handlePassiveSkillAdd(tid, push.PassiveSkillId, query, alloc, receive_data_pack);
}

pub fn PassiveSkillRemoveRequest(
    txn: *dispatch.CombatRequestTxn(.PassiveSkillRemoveRequest),
    query: PassiveSkillQuery,
    alloc: mem.Alloc,
) !void {
    const tid = if (txn.payload.TargetEntityId != 0) txn.payload.TargetEntityId else commonEntityId(txn.common) orelse 0;
    try handlePassiveSkillRemove(tid, txn.payload.PassiveSkillId, query, alloc, txn.receive_data_pack);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn PassiveSkillRemovePush(
    push: pb.PassiveSkillRemovePush,
    common: ?pb.CombatCommon,
    query: PassiveSkillQuery,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    const tid = if (push.TargetEntityId != 0) push.TargetEntityId else commonEntityId(common) orelse 0;
    try handlePassiveSkillRemove(tid, push.PassiveSkillId, query, alloc, receive_data_pack);
}

fn commonEntityId(common: ?pb.CombatCommon) ?i64 {
    const value = common orelse return null;
    if (value.EntityId == 0) return null;
    return value.EntityId;
}

fn buffHasAnyTag(
    tables: *const Assets.DataTables,
    granted_tags: []const []const u8,
    target_tag_ids: []const i32,
) bool {
    for (granted_tags) |tag_name| {
        const granted_id = gameplay_tags.idFromName(&tables.gameplay_tag_remap, tag_name) catch continue;
        for (target_tag_ids) |target_id| {
            if (gameplay_tags.contains(&tables.gameplay_tag_parent, granted_id, target_id)) return true;
        }
    }
    return false;
}

fn removeBuffHandleInPack(
    entity: Entity,
    handle_id: i32,
    instigator_id: i64,
    is_premature: bool,
    buffs: *Entity.FightBuffComponent,
    maybe_attr: ?*Entity.AttributeComponent,
    scene: *Scene,
    assets: *const Assets,
    buff_timers: *BuffTimerScheduler,
    events: *EventQueue,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
    io: std.Io,
    alloc: mem.Alloc,
) !void {
    const buff_info = buffs.getByHandleId(handle_id) orelse return;
    const buff_id = buff_info.BuffId;
    const orig_instigator = buff_info.InstigatorId;

    try buff_helper.appendShieldDelForHandle(receive_data_pack, entity.net_id, buffs, maybe_attr, handle_id, alloc, io);
    buff_timers.cancelHandle(entity.net_id, handle_id);
    buffs.removeByHandleId(alloc.gpa, handle_id);
    try receive_data_pack.append(alloc.arena, .{ .Message = .{
        .CombatNotifyData = .{
            .CombatCommon = .{ .EntityId = entity.net_id },
            .Message = .{
                .RemoveGameplayEffectNotify = .{
                    .EntityId = entity.net_id,
                    .Handle = handle_id,
                    .InstigatorId = instigator_id,
                },
            },
        },
    } });
    if (is_premature) {
        if (assets.tables.buff.getDataById(buff_id)) |buff_data| {
            if (buff_data.PrematureExpirationEffects.len != 0) {
                var follow_ups: std.ArrayList(Events.BuffAdditionEntry) = .empty;
                for (buff_data.PrematureExpirationEffects) |fid| {
                    if (fid != buff_id) try follow_ups.append(alloc.arena, .{ .id = fid, .is_active = true });
                }
                if (follow_ups.items.len != 0) {
                    const inst_entity = if (orig_instigator != 0)
                        if (scene.net_id_map.get(orig_instigator)) |ii|
                            Entity{ .index = ii, .net_id = orig_instigator }
                        else
                            entity
                    else
                        entity;
                    try events.enqueue(.buff_addition, .{
                        .target = entity,
                        .instigator = inst_entity,
                        .buffs = follow_ups.items,
                    });
                }
            }
        }
    }
    if (maybe_attr) |attr| {
        try buff_helper.syncContinuousBuffModifiers(receive_data_pack, scene, entity.net_id, buffs, attr, assets, alloc, io);
    }
    try events.enqueue(.buff_change, .{ .entity = entity });
}

fn appendStackCountNotifyInPack(
    entity_id: i64,
    handle_id: i32,
    new_stack_count: i32,
    instigator_id: i64,
    duration: f32,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
    alloc: mem.Alloc,
) !void {
    try receive_data_pack.append(alloc.arena, .{ .Message = .{
        .CombatNotifyData = .{
            .CombatCommon = .{ .EntityId = entity_id },
            .Message = .{
                .BuffStackCountNotify = .{
                    .HandleId = handle_id,
                    .NewStackCount = new_stack_count,
                    .InstigatorId = instigator_id,
                    .NotRefreshDuration = duration <= 0,
                    .NotRefreshPeriod = duration <= 0,
                    .Time = if (duration > 0) .{ .Duration = duration } else null,
                    .gFs = if (duration > 0) .{ .LeftDuration = duration } else null,
                },
            },
        },
    } });
}

test "a buff the client orders is announced to it" {
    try std.testing.expect(announcesApplication(.client_order));
}

test "a buff the client reports is not announced back to it" {
    try std.testing.expect(!announcesApplication(.client_report));
}

test "requested buff duration validates explicit client values" {
    try std.testing.expectEqual(@as(?f32, null), try requestDuration(null));
    try std.testing.expectEqual(@as(?f32, 1.5), try requestDuration(.{ .Duration = 1.5 }));
    try std.testing.expectEqual(@as(?f32, 0), try requestDuration(.{ .Duration = 0 }));
    try std.testing.expectEqual(@as(?f32, -1), try requestDuration(.{ .Duration = -1 }));

    try std.testing.expectError(
        error.InvalidBuffDuration,
        requestDuration(.{ .Duration = std.math.nan(f32) }),
    );
    try std.testing.expectError(
        error.InvalidBuffDuration,
        requestDuration(.{ .Duration = std.math.inf(f32) }),
    );
    try std.testing.expectError(
        error.InvalidBuffDuration,
        requestDuration(.{ .Duration = -std.math.inf(f32) }),
    );
    try std.testing.expectError(
        error.InvalidBuffDuration,
        requestDuration(.{ .Duration = std.math.floatMax(f32) }),
    );
}
