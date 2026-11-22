const std = @import("std");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const EventQueue = @import("../../logic/EventQueue.zig");
const Scene = @import("../../logic/Scene.zig");
const Connection = @import("../../network/Connection.zig");
const FileSystem = @import("common").FileSystem;
const attributes_helper = @import("../../logic/helpers/attributes.zig");
const RoleHelper = @import("../../logic/helpers/role.zig");
const PlayerRoleComponent = @import("../../logic/component/player/PlayerRoleComponent.zig");
const FormationInfo = @import("../../fs/FormationInfo.zig");

fn healAndReviveTeam(
    events: *EventQueue,
    scene: *Scene,
    fs: *FileSystem,
    role_comp: *PlayerRoleComponent,
    conn: *Connection,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    try RoleHelper.reviveAndHealFormation(conn, scene, role_comp, events, fs, alloc, io);
    try events.enqueue(.chat_command_response, .{ .content = "revived and healed active formation" });
}

pub const heal = struct {
    pub const alias = "hl";
    pub const description = "heals and revives all roles in the current formation.\nusage: heal";
    pub fn call(
        events: *EventQueue,
        scene: *Scene,
        fs: *FileSystem,
        role_comp: *PlayerRoleComponent,
        conn: *Connection,
        alloc: mem.Alloc,
        io: std.Io,
    ) !void {
        try healAndReviveTeam(events, scene, fs, role_comp, conn, alloc, io);
    }
};

pub const revive = struct {
    pub const alias = "rv";
    pub const description = "revives and heals all roles in the current formation.\nusage: revive";
    pub fn call(
        events: *EventQueue,
        scene: *Scene,
        fs: *FileSystem,
        role_comp: *PlayerRoleComponent,
        conn: *Connection,
        alloc: mem.Alloc,
        io: std.Io,
    ) !void {
        try healAndReviveTeam(events, scene, fs, role_comp, conn, alloc, io);
    }
};

pub const max_attribute = struct {
    pub const alias = "max";
    pub const description = "set the inputted attribute to its maximum possible value.\nusage: max_attribute [config_id|team] [EAttributeType]\nconfig_id is a number, for example 1108 is hiyuki's config id. use \"team\" to apply to all current team members.\nEAttributeType is a number.";
    pub fn call(
        events: *EventQueue,
        scene: *Scene,
        fs: *FileSystem,
        query: Scene.Query(&.{ Scene.Entity, *Scene.Entity.ConfigComponent, *Scene.Entity.AttributeComponent }),
        conn: *Connection,
        alloc: mem.Alloc,
        config_id_str: []const u8,
        attr: i32,
        io: std.Io,
    ) !void {
        var notify: pb.CombatReceivePackNotify = .{};
        var id_buf: [3]i32 = undefined;
        const ids = resolve_config_ids(config_id_str, &scene.formation_info, &id_buf);

        for (ids) |config_id| {
            try apply_attr_change(&notify, scene, fs, query, alloc, config_id, attr, .max, 0);
        }

        try conn.push(alloc.gpa, io, notify);
        try events.enqueue(.chat_command_response, .{ .content = "maxxed out stat" });
    }
};

pub const empty_attribute = struct {
    pub const alias = "empty";
    pub const description = "set the inputted attribute to its lowest possible value (0).\nusage: empty_attribute [config_id|team] [EAttributeType]\nconfig_id is a number, for example 1108 is hiyuki's config id. use \"team\" to apply to all current team members.\nEAttributeType is a number.";
    pub fn call(
        events: *EventQueue,
        scene: *Scene,
        fs: *FileSystem,
        query: Scene.Query(&.{ Scene.Entity, *Scene.Entity.ConfigComponent, *Scene.Entity.AttributeComponent }),
        conn: *Connection,
        alloc: mem.Alloc,
        config_id_str: []const u8,
        attr: i32,
        io: std.Io,
    ) !void {
        var notify: pb.CombatReceivePackNotify = .{};
        var id_buf: [3]i32 = undefined;
        const ids = resolve_config_ids(config_id_str, &scene.formation_info, &id_buf);

        for (ids) |config_id| {
            try apply_attr_change(&notify, scene, fs, query, alloc, config_id, attr, .empty, 0);
        }

        try conn.push(alloc.gpa, io, notify);
        try events.enqueue(.chat_command_response, .{ .content = "emptied out stat" });
    }
};

pub const set_attribute = struct {
    pub const alias = "set";
    pub const description = "set the inputted attribute to the inputted value.\nusage: set_attribute [config_id|team] [EAttributeType] [value]\nconfig_id is a number, for example 1108 is hiyuki's config id. use \"team\" to apply to all current team members.\nEAttributeType is a number.\nvalue is a number";
    pub fn call(
        events: *EventQueue,
        scene: *Scene,
        fs: *FileSystem,
        query: Scene.Query(&.{ Scene.Entity, *Scene.Entity.ConfigComponent, *Scene.Entity.AttributeComponent }),
        conn: *Connection,
        alloc: mem.Alloc,
        config_id_str: []const u8,
        attr: i32,
        value: i32,
        io: std.Io,
    ) !void {
        var notify: pb.CombatReceivePackNotify = .{};
        var id_buf: [3]i32 = undefined;
        const ids = resolve_config_ids(config_id_str, &scene.formation_info, &id_buf);

        for (ids) |config_id| {
            try apply_attr_change(&notify, scene, fs, query, alloc, config_id, attr, .set, value);
        }

        try conn.push(alloc.gpa, io, notify);
        try events.enqueue(.chat_command_response, .{ .content = "set stat to inputted value" });
    }
};

fn resolve_config_ids(
    config_id_str: []const u8,
    formation_info: *const FormationInfo,
    buf: []i32,
) []i32 {
    if (!std.mem.eql(u8, config_id_str, "team")) {
        const id = std.fmt.parseInt(i32, config_id_str, 10) catch return buf[0..0];
        buf[0] = id;
        return buf[0..1];
    }

    const formation = blk: {
        if (formation_info.cur_formation < 0) return buf[0..0];
        const idx: usize = @intCast(formation_info.cur_formation);
        if (idx >= formation_info.formations.len) return buf[0..0];
        break :blk formation_info.formations[idx];
    };

    var count: usize = 0;
    for (formation.roles) |maybe_role| {
        const role = maybe_role orelse continue;
        if (count >= buf.len) break;
        buf[count] = role.role_id;
        count += 1;
    }

    return buf[0..count];
}

fn apply_attr_change(
    notify: *pb.CombatReceivePackNotify,
    scene: *Scene,
    fs: *FileSystem,
    query: Scene.Query(&.{ Scene.Entity, *Scene.Entity.ConfigComponent, *Scene.Entity.AttributeComponent }),
    alloc: mem.Alloc,
    config_id: i32,
    attr: i32,
    comptime mode: enum { max, empty, set },
    value: i32,
) !void {
    var it = query.iterator;
    while (it.next()) |comps| {
        const entity: Scene.Entity, const config_comp: *Scene.Entity.ConfigComponent, const attr_comp: *Scene.Entity.AttributeComponent = comps;
        if (config_comp.config_id != config_id) continue;

        const attr_type = std.enums.fromInt(pb.EAttributeType, attr) orelse continue;
        if (attr < 0) continue;
        const attr_idx: usize = @intCast(attr);
        if (attr_idx >= attr_comp.attributes.len) continue;
        var attrs_for_notify: std.ArrayList(pb.GameplayAttributeData) = try .initCapacity(alloc.arena, 1);

        const effective_value: i32 = switch (mode) {
            .max => blk: {
                const max_type = try attributes_helper.get_related_attr(attr_type, .Max, alloc.gpa) orelse {
                    break :blk 10000000;
                };
                const max_idx_raw = @intFromEnum(max_type);
                if (max_idx_raw < 0) break :blk 10000000;
                const max_idx: usize = @intCast(max_idx_raw);
                if (max_idx >= attr_comp.attributes.len) break :blk 10000000;
                break :blk attr_comp.attributes[max_idx].current;
            },
            .empty => 0,
            .set => value,
        };

        _ = try attributes_helper.change_attr(attr_comp, attr_type, .Override, .Current, effective_value, alloc);
        try scene.markFsmDirty(alloc.gpa, entity.net_id, Scene.Entity.FsmComponent.WakeReason.attribute);
        try attrs_for_notify.append(alloc.arena, attributes_helper.gameplayAttributeData(attr_type, attr_comp.attributes[attr_idx]));

        try notify.Data.append(alloc.arena, .{ .Message = .{
            .CombatNotifyData = .{
                .CombatCommon = .{ .EntityId = entity.net_id },
                .Message = .{
                    .AttributeChangedNotify = .{
                        .Attributes = attrs_for_notify,
                    },
                },
            },
        } });
        try scene.saveComponents(fs, alloc.gpa, entity, &.{Scene.Entity.AttributeComponent});
    }
}
