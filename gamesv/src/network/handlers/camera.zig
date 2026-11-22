const std = @import("std");
const pb = @import("proto").pb;
const common = @import("common");
const mem = @import("../../mem.zig");
const PlayerSettingsComponent = @import("../../logic/component/player/PlayerSettingsComponent.zig");
const PlayerBasicComponent = @import("../../logic/component/player/PlayerBasicComponent.zig");
const Assets = @import("../../data/Assets.zig");
const RoleHelper = @import("../../logic/helpers/role.zig");
const Transaction = @import("../handlers.zig").Transaction;
const Scene = @import("../../logic/Scene.zig");

const FileSystem = common.FileSystem;

pub fn onCameraSettingQueryRequest(
    txn: *Transaction(pb.CameraSettingQueryRequest),
    settings_comp: *PlayerSettingsComponent,
) !void {
    txn.respond(.{
        .CameraSetting = .{
            .CameraParam = settings_comp.camera.toProto(),
        },
    });
}

pub fn onCameraSettingModifyRequest(
    txn: *Transaction(pb.CameraSettingModifyRequest),
    settings_comp: *PlayerSettingsComponent,
    fs: *FileSystem,
    alloc: mem.Alloc,
) !void {
    if (txn.message.CameraSetting) |setting| {
        if (setting.CameraParam) |param| {
            settings_comp.camera.fromProto(param);
            try settings_comp.save(fs, alloc.arena);
        }
    }

    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSelfPortraitStateRequest(
    txn: *Transaction(pb.SelfPortraitStateRequest),
    scene: *Scene,
    assets: *const Assets,
    basic_comp: *const PlayerBasicComponent,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    const loc: [3]f32 = if (scene.instance.players.len > 0) scene.instance.players[0].location else .{ 0, 0, 0 };
    const rot: [3]f32 = if (scene.instance.players.len > 0) scene.instance.players[0].rotation else .{ 0, 0, 0 };
    var switch_role_notify: pb.SwitchRoleNotify = .{
        .PlayerId = scene.player_id,
        .transform = .{ .Transform = .{
            .Pos = .{ .X = loc[0], .Y = loc[1], .Z = loc[2] },
            .Rot = .{ .Roll = rot[0], .Pitch = rot[1], .Yaw = rot[2] },
        } },
    };
    if (std.math.cast(usize, scene.formation_info.cur_formation)) |formation_idx| {
        if (formation_idx < scene.formation_info.formations.len) {
            const current_formation = &scene.formation_info.formations[formation_idx];
            var cur_entity_id: i64 = 0;
            var main_entity_id: i64 = 0;
            var main_role_id: i32 = 0;
            const gender = basic_comp.info.attributes.sex;
            for (current_formation.roles) |maybe_slot| {
                const slot = maybe_slot orelse continue;
                if (slot.role_id == current_formation.cur_role) {
                    cur_entity_id = slot.entity_id;
                }
                if (main_entity_id == 0 and RoleHelper.isMainRoleForGender(assets, gender, slot.role_id)) {
                    main_entity_id = slot.entity_id;
                    main_role_id = slot.role_id;
                }
            }
            const enter_selfie = (txn.message.State orelse .SelfPortraitEnter) == .SelfPortraitEnter;
            if (enter_selfie and main_entity_id > 0 and main_entity_id != cur_entity_id) {
                switch_role_notify.DownEntityId = cur_entity_id;
                switch_role_notify.UpEntityId = main_entity_id;
                current_formation.cur_role = main_role_id;
            } else {
                switch_role_notify.DownEntityId = cur_entity_id;
                switch_role_notify.UpEntityId = cur_entity_id;
            }
        }
    }

    try txn.conn.push(alloc.gpa, io, switch_role_notify);

    txn.respond(.{ .ErrorCode = .Success });
}
