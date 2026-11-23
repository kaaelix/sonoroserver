const std = @import("std");
const pb = @import("proto").pb;
const Assets = @import("../../data/Assets.zig");
const EventQueue = @import("../EventQueue.zig");
const Scene = @import("../Scene.zig");
const mem = @import("../../mem.zig");
const Connection = @import("../../network/Connection.zig");

/// Client only attaches `LevelTagComponent` on monsters, scene items, custom/scene entities,
/// and non-vehicle `Common` (`NpcType == 0`) NPCs (`Simple`/`Passerby`/`Vehicle`/`Tuanzi` lack it).
pub fn hasClientLevelTagComponent(scene: *const Scene, assets: ?*const Assets, net_id: i64) bool {
    const idx = scene.net_id_map.get(net_id) orelse return false;
    const cfg = scene.entities.items(.config)[idx];
    return switch (cfg.entity_type) {
        .monster, .scene_item, .custom, .scene_entity => true,
        .npc => blk: {
            if (scene.entities.items(.vehicle)[idx] != null) break :blk false;
            if (scene.entities.items(.fsm)[idx] == null) break :blk false;
            if (assets) |a| {
                if (a.findLevelEntity(scene.instance_id, cfg.config_id)) |le| {
                    if (le.ComponentsData.BaseInfoComponent) |bi| {
                        if (bi.Category) |cat| {
                            if (cat.NpcType) |nt| {
                                if (nt != 0) break :blk false;
                            }
                        }
                    }
                    if (std.mem.eql(u8, le.BlueprintType, "PasserbyNpc") or
                        std.mem.eql(u8, le.BlueprintType, "SimpleNpc")) break :blk false;
                }
            }
            break :blk true;
        },
        else => false,
    };
}

pub fn updateGameplayTags(
    event: EventQueue.Dequeue(.gameplay_tag_change),
    conn: *Connection,
    scene: *Scene,
    assets: *const Assets,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    if (event.data.add_tag_ids.len == 0 and event.data.remove_tag_ids.len == 0) return;

    if (hasClientLevelTagComponent(scene, assets, event.data.entity.net_id)) {
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
