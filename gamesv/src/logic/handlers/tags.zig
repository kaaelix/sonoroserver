const std = @import("std");
const pb = @import("proto").pb;
const EventQueue = @import("../EventQueue.zig");
const Scene = @import("../Scene.zig");
const mem = @import("../../mem.zig");
const Connection = @import("../../network/Connection.zig");

const EntityTagTargetQuery = Scene.Query(&.{
    Scene.Entity,
    *Scene.Entity.ConfigComponent,
    ?*Scene.Entity.VehicleComponent,
    ?*Scene.Entity.FsmComponent,
});

pub fn updateGameplayTags(
    event: EventQueue.Dequeue(.gameplay_tag_change),
    conn: *Connection,
    target_query: EntityTagTargetQuery,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    if (event.data.add_tag_ids.len == 0 and event.data.remove_tag_ids.len == 0) return;

    const has_level_tag_comp = if (target_query.byNetId(event.data.entity.net_id)) |item| blk: {
        _, const cfg, const vehicle, const fsm = item;
        break :blk switch (cfg.entity_type) {
            .monster, .scene_item, .custom, .scene_entity => true,
            .npc => vehicle == null and fsm != null,
            else => false,
        };
    } else false;

    if (has_level_tag_comp) {
        // LevelTagComponent entities (monsters/scene items/scene entities/NPCs) sync via EntityCommonTagNotify
        // even when IsMoveAutonomousProxy is true; Role/Vehicle/Vision lack LevelTagComponent on client.
        var common_notify: pb.EntityCommonTagNotify = .{ .Id = event.data.entity.net_id };
        try appendCommonTagNotifies(
            &common_notify,
            alloc,
            event.data.add_tag_ids,
            event.data.remove_tag_ids,
            event.data.remove_before_add,
        );
        if (common_notify.Tags.items.len != 0) {
            try conn.push(alloc.gpa, io, common_notify);
        }
        return;
    }

    const combat_common: pb.CombatCommon = .{ .EntityId = event.data.entity.net_id };
    var notify: pb.CombatReceivePackNotify = .{};

    try appendGameplayTagNotifies(
        &notify,
        alloc,
        combat_common,
        event.data.add_tag_ids,
        event.data.remove_tag_ids,
        event.data.remove_before_add,
    );

    if (notify.Data.items.len != 0) {
        try conn.push(alloc.gpa, io, notify);
    }
}

fn appendCommonTagNotifies(
    notify: *pb.EntityCommonTagNotify,
    alloc: mem.Alloc,
    add_tag_ids: []const i32,
    remove_tag_ids: []const i32,
    remove_before_add: bool,
) !void {
    // CommonTagData field 2 is IsAdd (stale generated name RemoveTagIds): false removes, true adds.
    if (remove_before_add) {
        for (remove_tag_ids) |tag_id| {
            if (tag_id != 0) try notify.Tags.append(alloc.arena, .{ .TagId = tag_id, .RemoveTagIds = false });
        }
        for (add_tag_ids) |tag_id| {
            if (tag_id != 0) try notify.Tags.append(alloc.arena, .{ .TagId = tag_id, .RemoveTagIds = true });
        }
    } else {
        for (add_tag_ids) |tag_id| {
            if (tag_id != 0) try notify.Tags.append(alloc.arena, .{ .TagId = tag_id, .RemoveTagIds = true });
        }
        for (remove_tag_ids) |tag_id| {
            if (tag_id != 0) try notify.Tags.append(alloc.arena, .{ .TagId = tag_id, .RemoveTagIds = false });
        }
    }
}

fn appendGameplayTagNotifies(
    notify: *pb.CombatReceivePackNotify,
    alloc: mem.Alloc,
    combat_common: pb.CombatCommon,
    add_tag_ids: []const i32,
    remove_tag_ids: []const i32,
    remove_before_add: bool,
) !void {
    if (remove_before_add) {
        for (remove_tag_ids) |tag_id| try appendTagNotify(notify, alloc, combat_common, tag_id, false);
        for (add_tag_ids) |tag_id| try appendTagNotify(notify, alloc, combat_common, tag_id, true);
    } else {
        for (add_tag_ids) |tag_id| try appendTagNotify(notify, alloc, combat_common, tag_id, true);
        for (remove_tag_ids) |tag_id| try appendTagNotify(notify, alloc, combat_common, tag_id, false);
    }
}

fn appendTagNotify(
    notify: *pb.CombatReceivePackNotify,
    alloc: mem.Alloc,
    combat_common: pb.CombatCommon,
    tag_id: i32,
    add: bool,
) !void {
    // Generated names are stale: AddTagIds is the tag id, RemoveTagIds is the add/remove flag.
    try notify.Data.append(alloc.arena, .{ .Message = .{
        .CombatNotifyData = .{
            .CombatCommon = combat_common,
            .Message = .{
                .AnimationGameplayTagNotify = .{
                    .AddTagIds = tag_id,
                    .RemoveTagIds = add,
                },
            },
        },
    } });
}
