const std = @import("std");
const pb = @import("proto").pb;
const EventQueue = @import("../EventQueue.zig");
const Scene = @import("../Scene.zig");
const Assets = @import("../../data/Assets.zig");
const FileSystem = @import("common").FileSystem;
const Entity = Scene.Entity;
const mem = @import("../../mem.zig");
const Connection = @import("../../network/Connection.zig");
const BuffTimerScheduler = @import("../schedulers/BuffTimerScheduler.zig");
const buff_helper = @import("../helpers/buff.zig");
const gameplay_tags = @import("../helpers/gameplay_tags.zig");
const Events = @import("../events.zig");

pub fn removeBuffFromEntity(
    event: EventQueue.Dequeue(.buff_removal),
    conn: *Connection,
    events: *EventQueue,
    scene: *Scene,
    assets: *const Assets,
    alloc: mem.Alloc,
    buff_timers: *BuffTimerScheduler,
    query: Scene.Query(&.{ *Entity.FightBuffComponent, ?*Entity.AttributeComponent }),
    io: std.Io,
) !void {
    const log = std.log.scoped(.buff_removal);
    const item = query.byEntityHandle(event.data.entity) orelse {
        for (event.data.handle_ids) |handle_id| {
            buff_timers.cancelHandle(event.data.entity.net_id, handle_id);
        }
        return;
    };
    const buff_comp, const maybe_attr = item;
    const combat_common: pb.CombatCommon = .{ .EntityId = event.data.entity.net_id };
    var notify: pb.CombatReceivePackNotify = .{};
    var committed = false;
    errdefer if (!committed) {
        if (event.data.natural_expiration) |expiration| {
            for (event.data.handle_ids) |handle_id| {
                buff_timers.deferExpirationRetry(
                    event.data.entity.net_id,
                    handle_id,
                    expiration.now_ms,
                );
            }
        }
    };

    for (event.data.handle_ids) |handle_id| {
        try buff_helper.appendShieldDelForHandle(
            &notify.Data,
            event.data.entity.net_id,
            buff_comp,
            maybe_attr,
            handle_id,
            alloc,
            io,
        );
        try notify.Data.append(alloc.arena, .{ .Message = .{
            .CombatNotifyData = .{
                .CombatCommon = combat_common,
                .Message = .{
                    .RemoveGameplayEffectNotify = .{
                        .EntityId = event.data.entity.net_id,
                        .Handle = handle_id,
                    },
                },
            },
        } });
    }

    try events.enqueue(.buff_change, .{ .entity = event.data.entity });
    if (event.data.natural_expiration) |expiration| {
        if (expiration.follow_up_buffs.len != 0) {
            try events.enqueue(.buff_addition, .{
                .target = event.data.entity,
                .instigator = expiration.instigator,
                .buffs = expiration.follow_up_buffs,
            });
        }
    }

    for (event.data.handle_ids) |handle_id| {
        buff_timers.cancelHandle(event.data.entity.net_id, handle_id);
        buff_comp.removeByHandleId(alloc.gpa, handle_id);
    }
    committed = true;

    if (maybe_attr) |attr_comp| {
        try buff_helper.syncContinuousBuffModifiers(
            &notify.Data,
            scene,
            event.data.entity.net_id,
            buff_comp,
            attr_comp,
            assets,
            alloc,
            io,
        );
    }

    try conn.push(alloc.gpa, io, notify);

    log.debug("removed these buffs from {d}: {any}", .{
        event.data.entity.net_id,
        event.data.handle_ids,
    });
}

pub fn removeBuffFromEntityById(
    event: EventQueue.Dequeue(.buff_removal_by_id),
    conn: *Connection,
    events: *EventQueue,
    scene: *Scene,
    assets: *const Assets,
    alloc: mem.Alloc,
    buff_timers: *BuffTimerScheduler,
    query: Scene.Query(&.{ *Entity.FightBuffComponent, ?*Entity.AttributeComponent }),
    io: std.Io,
) !void {
    const item = query.byEntityHandle(event.data.entity) orelse return;
    var handles: std.ArrayList(i32) = .empty;
    for (item[0].fight_buff_infos) |b| {
        if (b.BuffId == event.data.buff_id) try handles.append(alloc.arena, b.HandleId);
    }
    if (handles.items.len == 0) return;
    try removeBuffHandlesWithSync(event.data.entity, handles.items, item[0], item[1], scene, assets, conn, events, alloc, buff_timers, io);
}

pub fn removeBuffFromEntityByFsmBind(
    event: EventQueue.Dequeue(.buff_removal_by_fsm_bind),
    conn: *Connection,
    events: *EventQueue,
    scene: *Scene,
    assets: *const Assets,
    alloc: mem.Alloc,
    buff_timers: *BuffTimerScheduler,
    query: Scene.Query(&.{ *Entity.FightBuffComponent, ?*Entity.AttributeComponent }),
    io: std.Io,
) !void {
    const item = query.byEntityHandle(event.data.entity) orelse return;
    const release = item[0].releaseFsmBindLease(alloc.gpa, event.data.source) orelse return;
    switch (release) {
        .preserve => try events.enqueue(.buff_change, .{ .entity = event.data.entity }),
        .remove => |handle_id| {
            if (item[0].getByHandleId(handle_id) != null) {
                const handles = [_]i32{handle_id};
                try removeBuffHandlesWithSync(event.data.entity, &handles, item[0], item[1], scene, assets, conn, events, alloc, buff_timers, io);
            } else {
                try events.enqueue(.buff_change, .{ .entity = event.data.entity });
            }
        },
    }
}

pub fn removeBuffHandles(
    entity: Entity,
    handle_ids: []const i32,
    buffs: *Entity.FightBuffComponent,
    conn: *Connection,
    events: *EventQueue,
    alloc: mem.Alloc,
    buff_timers: *BuffTimerScheduler,
    io: std.Io,
) !void {
    try removeBuffHandlesWithSync(entity, handle_ids, buffs, null, null, null, conn, events, alloc, buff_timers, io);
}

pub fn removeBuffHandlesWithSync(
    entity: Entity,
    handle_ids: []const i32,
    buffs: *Entity.FightBuffComponent,
    maybe_attr: ?*Entity.AttributeComponent,
    maybe_scene: ?*Scene,
    maybe_assets: ?*const Assets,
    conn: *Connection,
    events: *EventQueue,
    alloc: mem.Alloc,
    buff_timers: *BuffTimerScheduler,
    io: std.Io,
) !void {
    const combat_common: pb.CombatCommon = .{ .EntityId = entity.net_id };
    var notify: pb.CombatReceivePackNotify = .{};

    for (handle_ids) |handle_id| {
        if (buffs.getByHandleId(handle_id) == null) continue;
        try buff_helper.appendShieldDelForHandle(&notify.Data, entity.net_id, buffs, maybe_attr, handle_id, alloc, io);
        buff_timers.cancelHandle(entity.net_id, handle_id);
        buffs.removeByHandleId(alloc.gpa, handle_id);
        try notify.Data.append(alloc.arena, .{ .Message = .{
            .CombatNotifyData = .{
                .CombatCommon = combat_common,
                .Message = .{
                    .RemoveGameplayEffectNotify = .{
                        .EntityId = entity.net_id,
                        .Handle = handle_id,
                    },
                },
            },
        } });
    }

    if (notify.Data.items.len == 0) return;
    if (maybe_attr) |attr| {
        if (maybe_scene) |scene| if (maybe_assets) |assets| {
            try buff_helper.syncContinuousBuffModifiers(&notify.Data, scene, entity.net_id, buffs, attr, assets, alloc, io);
        };
    }
    try conn.push(alloc.gpa, io, notify);
    try events.enqueue(.buff_change, .{ .entity = entity });
}

pub fn addBuffToEntity(
    event: EventQueue.Dequeue(.buff_addition),
    conn: *Connection,
    events: *EventQueue,
    scene: *Scene,
    buff_timers: *BuffTimerScheduler,
    assets: *const Assets,
    fs: *FileSystem,
    io: std.Io,
    alloc: mem.Alloc,
    query: Scene.Query(&.{
        Entity,
        *Entity.FightBuffComponent,
        ?*Entity.AttributeComponent,
    }),
) !void {
    const log = std.log.scoped(.buff_addition);
    const item = query.byEntityHandle(event.data.target) orelse return;
    const buff_component = item[1];
    const maybe_attr = item[2];
    const combat_common: pb.CombatCommon = .{
        .EntityId = event.data.target.net_id,
        .MessageId = -1,
        .PreMessageId = -1,
    };
    var notify: pb.CombatReceivePackNotify = .{};
    var buff_change_enqueued = false;
    const clock: std.Io.Clock = .awake;
    const now_ms = clock.now(io).toMilliseconds();
    try buff_timers.ensureEntityRegistered(
        alloc.gpa,
        scene,
        assets,
        event.data.target.net_id,
        now_ms,
    );

    for (event.data.buffs) |entry| {
        const buff_data = assets.tables.buff.getDataById(entry.id) orelse continue;
        if (buff_data.RemoveBuffWithTags.len != 0) {
            try removeBuffsWithTagsInline(
                &notify.Data,
                event.data.target,
                buff_component,
                maybe_attr,
                buff_data.RemoveBuffWithTags,
                assets,
                events,
                buff_timers,
                &buff_change_enqueued,
                alloc,
                io,
            );
        }

        if (buff_data.DurationPolicy == .Instant) {
            try notify.Data.append(alloc.arena, .{ .Message = .{
                .CombatNotifyData = .{
                    .CombatCommon = combat_common,
                    .Message = .{
                        .ApplyGameplayEffectNotify = .{
                            .Handle = -2,
                            .Id = buff_data.Id,
                            .EntityId = event.data.target.net_id,
                            .InstigatorId = event.data.instigator.net_id,
                            .IsActive = true,
                        },
                    },
                },
            } });
            try buff_helper.execute_buff_effects(
                &notify.Data,
                event.data.target.net_id,
                event.data.instigator.net_id,
                &buff_data,
                scene,
                fs,
                io,
                query,
                alloc,
            );
            continue;
        }
        if (entry.fsm_bind_source) |source| {
            if (!buff_component.prepareFsmBindAcquire(alloc.gpa, source)) continue;
        }
        const existing: ?*pb.FightBuffInformation = switch (buff_data.StackingType) {
            .None => null,
            .ByInstigator => buff_component.getByBuffIdAndInstigator(entry.id, std.math.cast(i32, event.data.instigator.net_id) orelse @as(i32, @truncate(event.data.instigator.net_id))),
            .ByTarget => buff_component.getByBuffId(entry.id),
        };
        const stack_limit: i32 = if (buff_data.StackLimitCount > 0)
            @intCast(@min(buff_data.StackLimitCount, @as(u32, std.math.maxInt(i32))))
        else
            std.math.maxInt(i32);

        if (buff_data.StackingType == .None and buff_data.StackLimitCount > 0) {
            var count: i32 = 0;
            for (buff_component.fight_buff_infos) |b| {
                if (b.BuffId == entry.id) count += 1;
            }
            if (count >= stack_limit) {
                try enqueueOverflowEffects(events, event.data.target, event.data.instigator, &buff_data, alloc.arena);
                if (buff_data.bClearStackOnOverflow) {
                    var clear_handles: std.ArrayList(i32) = .empty;
                    for (buff_component.fight_buff_infos) |b| {
                        if (b.BuffId == entry.id) try clear_handles.append(alloc.arena, b.HandleId);
                    }
                    for (clear_handles.items) |hid| {
                        try removeSingleBuffInline(&notify.Data, event.data.target, buff_component, maybe_attr, hid, events, buff_timers, &buff_change_enqueued, alloc, io);
                    }
                    continue;
                }
                if (buff_data.bDenyOverflowApplication) continue;
            }
        }

        if (existing) |buff| {
            const append_stacks: i32 = if (entry.stack_count > 0)
                entry.stack_count
            else if (buff_data.StackAppendCount > 0)
                @intCast(buff_data.StackAppendCount)
            else
                0;

            const desired_total: i64 = @as(i64, buff.StackCount) + append_stacks;
            const overflowed = buff.StackCount >= stack_limit or desired_total > stack_limit;
            if (overflowed) {
                try enqueueOverflowEffects(events, event.data.target, event.data.instigator, &buff_data, alloc.arena);
                if (buff_data.bClearStackOnOverflow) {
                    try removeSingleBuffInline(&notify.Data, event.data.target, buff_component, maybe_attr, buff.HandleId, events, buff_timers, &buff_change_enqueued, alloc, io);
                    continue;
                }
                if (buff_data.bDenyOverflowApplication and buff.StackCount >= stack_limit) {
                    continue;
                }
            }
            const next_stack_count: i32 = @max(1, @as(i32, @intCast(@min(desired_total, @as(i64, stack_limit)))));

            if (entry.fsm_bind_source) |source| {
                try buff_component.addFsmBindLease(alloc.gpa, source, buff.HandleId, false);
            } else {
                buff_component.markFsmBindHandleExternal(alloc.gpa, buff.HandleId);
            }
            buff_timers.syncHandleLeftDuration(
                scene,
                event.data.target.net_id,
                buff.HandleId,
                now_ms,
            );
            const was_active = buff.IsActive;
            const disposition = BuffTimerScheduler.applicationDisposition(
                &buff_data,
                false,
                was_active,
                entry.is_active,
            );
            var next_buff = buff.*;
            next_buff.Level = if (buff.Level > 0) buff.Level else 1;
            next_buff.StackCount = next_stack_count;
            next_buff.InstigatorId = event.data.instigator.net_id;
            next_buff.EntityId = event.data.target.net_id;
            if (disposition.refresh_duration) {
                applyBuffDuration(&next_buff, &buff_data, entry.duration_seconds);
            }
            next_buff.ApplyType = .Common;
            next_buff.IsActive = entry.is_active;
            next_buff.MessageId = -1;

            try appendStackCountNotify(
                &notify.Data,
                combat_common,
                next_buff,
                disposition,
                alloc.arena,
            );
            if (was_active != next_buff.IsActive) {
                try appendActivateBuffNotify(
                    &notify.Data,
                    combat_common,
                    next_buff.HandleId,
                    next_buff.IsActive,
                    alloc.arena,
                );
            }
            try enqueueBuffChangeOnce(
                events,
                event.data.target,
                &buff_change_enqueued,
            );
            try buff_timers.refreshBuff(
                alloc.gpa,
                next_buff,
                &buff_data,
                event.data.target.net_id,
                now_ms,
                disposition,
            );
            buff.* = next_buff;
            try buff_helper.syncBuffShield(
                &notify.Data,
                scene,
                assets,
                event.data.target.net_id,
                buff_component,
                maybe_attr,
                next_buff,
                &buff_data,
                alloc,
                io,
            );

            if (disposition.execute_periodic_now) {
                try buff_helper.executePeriodicWithAssets(
                    &notify.Data,
                    event.data.target.net_id,
                    next_buff.InstigatorId,
                    &buff_data,
                    scene,
                    assets,
                    fs,
                    io,
                    query,
                    alloc,
                );
            }
        } else {
            const raw_init_stacks: i32 = if (entry.stack_count > 0)
                entry.stack_count
            else if (buff_data.DefaultStackCount > 0)
                @intCast(buff_data.DefaultStackCount)
            else
                1;
            const init_stacks: i32 = @max(1, @min(raw_init_stacks, stack_limit));

            const handle_id = scene.instance.buff_handle + 1;
            var new_buff = Assets.DataTables.createBuffInformation(
                handle_id,
                entry.id,
                event.data.instigator.net_id,
                event.data.target.net_id,
                entry.is_active,
            );
            new_buff.StackCount = init_stacks;
            applyBuffDuration(&new_buff, &buff_data, entry.duration_seconds);
            const disposition = BuffTimerScheduler.applicationDisposition(
                &buff_data,
                true,
                false,
                entry.is_active,
            );
            try notify.Data.append(alloc.arena, .{ .Message = .{
                .CombatNotifyData = .{
                    .CombatCommon = combat_common,
                    .Message = .{
                        .ApplyGameplayEffectNotify = .{
                            .Handle = new_buff.HandleId,
                            .Id = new_buff.BuffId,
                            .Level = new_buff.Level,
                            .EntityId = new_buff.EntityId,
                            .InstigatorId = new_buff.InstigatorId,
                            .ApplyType = if (new_buff.ApplyType) |apply_type| apply_type else .Common,
                            .IsActive = new_buff.IsActive,
                            .ServerId = new_buff.ServerId,
                            .StackCount = new_buff.StackCount,
                            .CRoundAction = .{ .Duration = new_buff.Duration },
                            .Time = .{ .LeftDuration = new_buff.LeftDuration },
                            .ConfBuffId = new_buff.ConfBuffId,
                        },
                    },
                },
            } });
            try enqueueBuffChangeOnce(
                events,
                event.data.target,
                &buff_change_enqueued,
            );
            try buff_timers.scheduleNewBuff(
                alloc.gpa,
                new_buff,
                &buff_data,
                event.data.target.net_id,
                now_ms,
            );
            buff_component.fight_buff_infos = alloc.gpa.realloc(
                buff_component.fight_buff_infos,
                buff_component.fight_buff_infos.len + 1,
            ) catch |err| {
                buff_timers.cancelHandle(event.data.target.net_id, handle_id);
                return err;
            };
            buff_component.fight_buff_infos[buff_component.fight_buff_infos.len - 1] = new_buff;
            if (entry.fsm_bind_source) |source| {
                buff_component.addFsmBindLease(alloc.gpa, source, new_buff.HandleId, true) catch |err| {
                    buff_timers.cancelHandle(event.data.target.net_id, new_buff.HandleId);
                    buff_component.removeByHandleId(alloc.gpa, new_buff.HandleId);
                    return err;
                };
            }
            scene.instance.buff_handle = handle_id;
            try buff_helper.syncBuffShield(
                &notify.Data,
                scene,
                assets,
                event.data.target.net_id,
                buff_component,
                maybe_attr,
                new_buff,
                &buff_data,
                alloc,
                io,
            );

            if (disposition.execute_periodic_now) {
                try buff_helper.executePeriodicWithAssets(
                    &notify.Data,
                    event.data.target.net_id,
                    new_buff.InstigatorId,
                    &buff_data,
                    scene,
                    assets,
                    fs,
                    io,
                    query,
                    alloc,
                );
            }
        }
    }

    if (maybe_attr) |attr_comp| {
        try buff_helper.syncContinuousBuffModifiers(
            &notify.Data,
            scene,
            event.data.target.net_id,
            buff_component,
            attr_comp,
            assets,
            alloc,
            io,
        );
    }

    try conn.push(alloc.gpa, io, notify);

    log.debug("eid {d}: added these buffs to {d}: {any}", .{
        event.data.instigator.net_id,
        event.data.target.net_id,
        event.data.buffs,
    });
}

fn removeSingleBuffInline(
    data: *std.ArrayList(pb.CombatReceiveData),
    target: Entity,
    buffs: *Entity.FightBuffComponent,
    maybe_attr: ?*Entity.AttributeComponent,
    handle_id: i32,
    events: *EventQueue,
    buff_timers: *BuffTimerScheduler,
    enqueued: *bool,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    if (buffs.getByHandleId(handle_id) == null) return;
    try buff_helper.appendShieldDelForHandle(data, target.net_id, buffs, maybe_attr, handle_id, alloc, io);
    buff_timers.cancelHandle(target.net_id, handle_id);
    buffs.removeByHandleId(alloc.gpa, handle_id);
    try data.append(alloc.arena, .{ .Message = .{
        .CombatNotifyData = .{
            .CombatCommon = .{ .EntityId = target.net_id },
            .Message = .{
                .RemoveGameplayEffectNotify = .{
                    .EntityId = target.net_id,
                    .Handle = handle_id,
                },
            },
        },
    } });
    try enqueueBuffChangeOnce(events, target, enqueued);
}

fn removeBuffsWithTagsInline(
    data: *std.ArrayList(pb.CombatReceiveData),
    target: Entity,
    buffs: *Entity.FightBuffComponent,
    maybe_attr: ?*Entity.AttributeComponent,
    tag_names: []const []const u8,
    assets: *const Assets,
    events: *EventQueue,
    buff_timers: *BuffTimerScheduler,
    enqueued: *bool,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    if (tag_names.len == 0 or buffs.fight_buff_infos.len == 0) return;

    var removal_tag_ids: std.ArrayList(i32) = .empty;
    for (tag_names) |name| {
        const id = gameplay_tags.idFromName(&assets.tables.gameplay_tag_remap, name) catch continue;
        try removal_tag_ids.append(alloc.arena, id);
    }
    if (removal_tag_ids.items.len == 0) return;

    var to_remove: std.ArrayList(i32) = .empty;
    for (buffs.fight_buff_infos) |info| {
        const candidate = assets.tables.buff.getDataById(info.BuffId) orelse continue;
        for (candidate.GrantedTags) |granted_name| {
            const granted_id = gameplay_tags.idFromName(&assets.tables.gameplay_tag_remap, granted_name) catch continue;
            const matched = for (removal_tag_ids.items) |rem_id| {
                if (gameplay_tags.contains(&assets.tables.gameplay_tag_parent, granted_id, rem_id)) break true;
            } else false;
            if (matched) {
                try to_remove.append(alloc.arena, info.HandleId);
                break;
            }
        }
    }

    for (to_remove.items) |hid| {
        try removeSingleBuffInline(data, target, buffs, maybe_attr, hid, events, buff_timers, enqueued, alloc, io);
    }
}

fn enqueueOverflowEffects(
    events: *EventQueue,
    target: Entity,
    instigator: Entity,
    buff_data: *const Assets.DataTables.Buff,
    arena: std.mem.Allocator,
) !void {
    if (buff_data.OverflowEffects.len == 0) return;
    var follow_ups: std.ArrayList(Events.BuffAdditionEntry) = .empty;
    for (buff_data.OverflowEffects) |ov_id| {
        if (ov_id == buff_data.Id) continue;
        try follow_ups.append(arena, .{
            .id = ov_id,
            .is_active = true,
        });
    }
    if (follow_ups.items.len != 0) {
        try events.enqueue(.buff_addition, .{
            .target = target,
            .instigator = instigator,
            .buffs = follow_ups.items,
        });
    }
}

fn appendStackCountNotify(
    data: *std.ArrayList(pb.CombatReceiveData),
    combat_common: pb.CombatCommon,
    buff: pb.FightBuffInformation,
    disposition: BuffTimerScheduler.ApplicationDisposition,
    arena: std.mem.Allocator,
) !void {
    var stack_notify: pb.BuffStackCountNotify = .{
        .HandleId = buff.HandleId,
        .NewStackCount = buff.StackCount,
        .InstigatorId = buff.InstigatorId,
        .NotRefreshDuration = !disposition.refresh_duration,
        .NotRefreshPeriod = !disposition.reset_period,
    };
    if (disposition.refresh_duration) {
        stack_notify.Time = .{ .Duration = buff.Duration };
        if (buff.LeftDuration > 0) {
            stack_notify.gFs = .{ .LeftDuration = buff.LeftDuration };
        }
    }
    try data.append(arena, .{ .Message = .{
        .CombatNotifyData = .{
            .CombatCommon = combat_common,
            .Message = .{ .BuffStackCountNotify = stack_notify },
        },
    } });
}

fn appendActivateBuffNotify(
    data: *std.ArrayList(pb.CombatReceiveData),
    combat_common: pb.CombatCommon,
    handle_id: i32,
    is_active: bool,
    arena: std.mem.Allocator,
) !void {
    try data.append(arena, .{ .Message = .{
        .CombatNotifyData = .{
            .CombatCommon = combat_common,
            .Message = .{ .ActivateBuffNotify = .{
                .Handle = handle_id,
                .On = is_active,
            } },
        },
    } });
}

fn enqueueBuffChangeOnce(
    events: *EventQueue,
    entity: Entity,
    enqueued: *bool,
) !void {
    if (enqueued.*) return;
    try events.enqueue(.buff_change, .{ .entity = entity });
    enqueued.* = true;
}

pub fn handleBuffTimersDue(
    event: EventQueue.Dequeue(.buff_timers_due),
    conn: *Connection,
    events: *EventQueue,
    buff_timers: *BuffTimerScheduler,
    scene: *Scene,
    assets: *const Assets,
    fs: *FileSystem,
    io: std.Io,
    alloc: mem.Alloc,
    query: Scene.Query(&.{
        Entity,
        *Entity.FightBuffComponent,
        ?*Entity.AttributeComponent,
    }),
) !void {
    var combat_receive_pack: std.ArrayList(pb.CombatReceiveData) = .empty;
    try buff_timers.drainOneDue(
        event,
        events,
        scene,
        assets,
        fs,
        io,
        query,
        &combat_receive_pack,
        alloc,
    );

    if (combat_receive_pack.items.len != 0) {
        try conn.push(alloc.gpa, io, pb.CombatReceivePackNotify{ .Data = combat_receive_pack });
    }
}

fn applyBuffDuration(
    buff: *pb.FightBuffInformation,
    buff_data: *const Assets.DataTables.Buff,
    requested_duration: ?f32,
) void {
    switch (buff_data.DurationPolicy) {
        .Instant => {
            buff.Duration = 0;
            buff.LeftDuration = 0;
        },
        .Infinite => {
            buff.Duration = -1;
            buff.LeftDuration = -1;
        },
        .HasDuration => {
            const duration = requested_duration orelse
                buff_helper.configured_duration_seconds(buff_data);
            if (duration > 0) {
                buff.Duration = duration;
                buff.LeftDuration = duration;
            } else {
                buff.Duration = -1;
                buff.LeftDuration = -1;
            }
        },
    }
}

test "stack refresh notify omits preserved duration" {
    var data: std.ArrayList(pb.CombatReceiveData) = .empty;
    defer data.deinit(std.testing.allocator);

    try appendStackCountNotify(
        &data,
        .{ .EntityId = 10 },
        .{
            .HandleId = 20,
            .StackCount = 3,
            .InstigatorId = 30,
            .Duration = 2,
            .LeftDuration = 1,
        },
        .{
            .refresh_duration = false,
            .reset_period = false,
            .execute_periodic_now = false,
        },
        std.testing.allocator,
    );

    const notify = data.items[0].Message.?.CombatNotifyData.?
        .Message.?.BuffStackCountNotify.?;
    try std.testing.expect(notify.NotRefreshDuration);
    try std.testing.expect(notify.NotRefreshPeriod);
    try std.testing.expect(notify.Time == null);
    try std.testing.expect(notify.gFs == null);
}

test "stack refresh notify includes refreshed duration" {
    var data: std.ArrayList(pb.CombatReceiveData) = .empty;
    defer data.deinit(std.testing.allocator);

    try appendStackCountNotify(
        &data,
        .{ .EntityId = 10 },
        .{
            .HandleId = 20,
            .StackCount = 3,
            .InstigatorId = 30,
            .Duration = 2,
            .LeftDuration = 2,
        },
        .{
            .refresh_duration = true,
            .reset_period = true,
            .execute_periodic_now = false,
        },
        std.testing.allocator,
    );

    const notify = data.items[0].Message.?.CombatNotifyData.?
        .Message.?.BuffStackCountNotify.?;
    try std.testing.expect(!notify.NotRefreshDuration);
    try std.testing.expect(!notify.NotRefreshPeriod);
    try std.testing.expectEqual(@as(f32, 2), notify.Time.?.Duration);
    try std.testing.expectEqual(@as(f32, 2), notify.gFs.?.LeftDuration);
}

test "activation notify carries the changed active state" {
    var data: std.ArrayList(pb.CombatReceiveData) = .empty;
    defer data.deinit(std.testing.allocator);

    try appendActivateBuffNotify(
        &data,
        .{ .EntityId = 10 },
        20,
        true,
        std.testing.allocator,
    );

    const notify = data.items[0].Message.?.CombatNotifyData.?
        .Message.?.ActivateBuffNotify.?;
    try std.testing.expectEqual(@as(i32, 20), notify.Handle);
    try std.testing.expect(notify.On);
}
