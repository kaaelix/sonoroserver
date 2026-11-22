const std = @import("std");
const EventQueue = @import("EventQueue.zig");
const events = @import("events.zig");
const mem = @import("../mem.zig");
const Scene = @import("Scene.zig");
const Entity = Scene.Entity;

pub fn enqueueEffects(
    entity: Entity,
    fsm: *Entity.FsmComponent,
    event_queue: *EventQueue,
    alloc: mem.Alloc,
    now_ms: i64,
) !bool {
    return enqueueEffectsWithRecheck(entity, fsm, event_queue, alloc, now_ms, true);
}

pub fn enqueueEffectsWithoutRecheck(
    entity: Entity,
    fsm: *Entity.FsmComponent,
    event_queue: *EventQueue,
    alloc: mem.Alloc,
    now_ms: i64,
) !bool {
    return enqueueEffectsWithRecheck(entity, fsm, event_queue, alloc, now_ms, false);
}

const AggregatedTagChange = struct {
    tag_id: i32,
    first_was_present: bool,
    last_is_present: bool,
    net_delta_after_clear: i64,
    had_clear: bool,
};

fn enqueueEffectsWithRecheck(
    entity: Entity,
    fsm: *Entity.FsmComponent,
    event_queue: *EventQueue,
    alloc: mem.Alloc,
    now_ms: i64,
    recheck: bool,
) !bool {
    if (fsm.lifecycleEffectsPending()) {
        fsm.requestLifecycleRecheck(recheck);
        return true;
    }
    const effects = fsm.lifecycleEffects();
    if (effects.len == 0) return false;

    var tag_summaries: std.ArrayList(AggregatedTagChange) = .empty;
    var enqueued_any = false;

    for (effects) |effect| {
        switch (effect) {
            .enter_fight => {
                try event_queue.enqueue(.fsm_server_action, .{
                    .entity = entity,
                    .kind = .enter_fight,
                });
                enqueued_any = true;
            },
            .add_buff => |buff_id| {
                const buffs = try alloc.arena.alloc(events.BuffAdditionEntry, 1);
                buffs[0] = .{ .id = buff_id, .is_active = true };
                try event_queue.enqueue(.buff_addition, .{
                    .target = entity,
                    .instigator = entity,
                    .buffs = buffs,
                });
                enqueued_any = true;
            },
            .remove_buff => |buff_id| {
                try event_queue.enqueue(.buff_removal_by_id, .{
                    .entity = entity,
                    .buff_id = buff_id,
                });
                enqueued_any = true;
            },
            .bind_buff_add => |addition| {
                const buffs = try alloc.arena.alloc(events.BuffAdditionEntry, 1);
                buffs[0] = .{
                    .id = addition.buff_id,
                    .is_active = true,
                    .fsm_bind_source = addition.source,
                };
                try event_queue.enqueue(.buff_addition, .{
                    .target = entity,
                    .instigator = entity,
                    .buffs = buffs,
                });
                enqueued_any = true;
            },
            .bind_buff_remove => |source| {
                try event_queue.enqueue(.buff_removal_by_fsm_bind, .{
                    .entity = entity,
                    .source = source,
                });
                enqueued_any = true;
            },
            .cue_paralysis => {
                try event_queue.enqueue(.fsm_server_action, .{
                    .entity = entity,
                    .kind = .cue_paralysis,
                });
                enqueued_any = true;
            },
            .reset_status => {
                try event_queue.enqueue(.fsm_server_action, .{
                    .entity = entity,
                    .kind = .reset_status,
                });
                enqueued_any = true;
            },
            .set_rage_full => {
                try event_queue.enqueue(.fsm_server_action, .{
                    .entity = entity,
                    .kind = .set_rage_full,
                });
                enqueued_any = true;
            },
            .set_instance_state => |tag_id| {
                try event_queue.enqueue(.fsm_server_action, .{
                    .entity = entity,
                    .kind = .{ .instance_state = tag_id },
                });
                enqueued_any = true;
            },
            .activate_part => |part| {
                try event_queue.enqueue(.fsm_server_action, .{
                    .entity = entity,
                    .kind = .{ .activate_part = part },
                });
                enqueued_any = true;
            },
            .reset_part => |part| {
                try event_queue.enqueue(.fsm_server_action, .{
                    .entity = entity,
                    .kind = .{ .reset_part = part },
                });
                enqueued_any = true;
            },
            .tag_mirror => |mirror| {
                for (tag_summaries.items) |*entry| {
                    if (entry.tag_id != mirror.tag_id) continue;
                    if (mirror.clear) {
                        entry.had_clear = true;
                        entry.net_delta_after_clear = 0;
                        entry.last_is_present = false;
                    } else {
                        entry.net_delta_after_clear += mirror.delta;
                        entry.last_is_present = mirror.is_present;
                    }
                    break;
                } else {
                    try tag_summaries.append(alloc.arena, .{
                        .tag_id = mirror.tag_id,
                        .first_was_present = mirror.was_present,
                        .last_is_present = mirror.is_present,
                        .net_delta_after_clear = if (mirror.clear) 0 else mirror.delta,
                        .had_clear = mirror.clear,
                    });
                }
            },
        }
    }

    if (tag_summaries.items.len != 0) {
        var add_tags: std.ArrayList(i32) = .empty;
        var remove_tags: std.ArrayList(i32) = .empty;
        var mirrors_pushed: usize = 0;

        for (tag_summaries.items) |entry| {
            if (entry.had_clear) {
                try fsm.pending_tag_mirrors.append(alloc.gpa, .{
                    .tag_id = entry.tag_id,
                    .clear = true,
                });
                mirrors_pushed += 1;
                if (entry.net_delta_after_clear > 0) {
                    const clamped: i32 = std.math.cast(i32, entry.net_delta_after_clear) orelse std.math.maxInt(i32);
                    try fsm.pending_tag_mirrors.append(alloc.gpa, .{
                        .tag_id = entry.tag_id,
                        .delta = clamped,
                    });
                    mirrors_pushed += 1;
                }
                if (!entry.last_is_present or entry.first_was_present) {
                    try remove_tags.append(alloc.arena, entry.tag_id);
                }
                if (entry.last_is_present) {
                    try add_tags.append(alloc.arena, entry.tag_id);
                }
            } else {
                if (entry.net_delta_after_clear != 0) {
                    const clamped: i32 = std.math.cast(i32, entry.net_delta_after_clear) orelse if (entry.net_delta_after_clear < 0)
                        std.math.minInt(i32)
                    else
                        std.math.maxInt(i32);
                    try fsm.pending_tag_mirrors.append(alloc.gpa, .{
                        .tag_id = entry.tag_id,
                        .delta = clamped,
                    });
                    mirrors_pushed += 1;
                }
                if (!entry.first_was_present and entry.last_is_present) {
                    try add_tags.append(alloc.arena, entry.tag_id);
                } else if (entry.first_was_present and !entry.last_is_present) {
                    try remove_tags.append(alloc.arena, entry.tag_id);
                }
            }
        }

        if (mirrors_pushed != 0 or add_tags.items.len != 0 or remove_tags.items.len != 0) {
            try event_queue.enqueue(.gameplay_tag_change, .{
                .entity = entity,
                .add_tag_ids = add_tags.items,
                .remove_tag_ids = remove_tags.items,
                .remove_before_add = true,
            });
            enqueued_any = true;
        }
    }

    if (!enqueued_any) {
        fsm.clearLifecycleEffects(alloc.gpa);
        return false;
    }

    try event_queue.enqueue(.fsm_lifecycle_complete, .{
        .entity = entity,
        .now_ms = now_ms,
        .recheck = recheck,
    });
    fsm.markLifecycleEffectsEnqueued(alloc.gpa);
    return true;
}
