const std = @import("std");
const mem = @import("../../mem.zig");
const PlayerID = @import("../../logic/PlayerID.zig");
const pb = @import("proto").pb;
const SceneInstance = @import("../../fs/SceneInstance.zig");
const Assets = @import("../../data/Assets.zig");
const Scene = @import("../../logic/Scene.zig");
const EventQueue = @import("../../logic/EventQueue.zig");
const Transaction = @import("../handlers.zig").Transaction;
const FormationInfo = @import("../../fs/FormationInfo.zig");
const FileSystem = @import("common").FileSystem;
const RoleEntityTemplates = @import("../../logic/templates/RoleEntityTemplates.zig");
const PlayerRoleComponent = @import("../../logic/component/player/PlayerRoleComponent.zig");
const PlayerWeaponComponent = @import("../../logic/component/player/PlayerWeaponComponent.zig");
const PlayerEchoComponent = @import("../../logic/component/player/PlayerEchoComponent.zig");
const EntityComponentStorage = @import("../../logic/component/entity/EntityComponentStorage.zig");
const BuffTimerScheduler = @import("../../logic/schedulers/BuffTimerScheduler.zig");
const RoleHelper = @import("../../logic/helpers/role.zig");
const entity_proto = @import("../../logic/helpers/entity_proto.zig");
const QuestEngine = @import("../../logic/helpers/quest_engine.zig");

fn build_fight_formations(
    alloc: mem.Alloc,
    scene: *Scene,
) !std.ArrayList(pb.FightFormation) {
    var fight_formations: std.ArrayList(pb.FightFormation) = .empty;

    for (scene.formation_info.formations, 0..) |fight_formation, i| {
        var role_ids: std.ArrayList(i32) = .empty;

        for (fight_formation.roles) |maybe_role| {
            if (maybe_role) |role| {
                try role_ids.append(alloc.arena, role.role_id);
            }
        }

        try fight_formations.append(alloc.arena, .{
            .FormationId = @intCast(i + 1),
            .Name = fight_formation.name orelse "",
            .CurRole = fight_formation.cur_role,
            .RoleIds = role_ids,
            .IsCurrent = (i == scene.formation_info.cur_formation),
        });
    }

    return fight_formations;
}

// pub fn onFormationAttrRequest(
//     txn: *Transaction(pb.FormationAttrRequest),
//     alloc: mem.Alloc,
//     scene: *Scene,
// ) !void {
//     for (scene.instance.players) |scene_player| {
//         if (scene_player.id == scene.player_id) {
//             for (scene.formation_info.formations) |formation| {
//                 for (formation.roles) |_| {
//                     const formation_attr_notify: pb.FormationAttrNotify = .{
//                         .Duration = txn.message.Duration,
//                         .FormationAttrs = txn.message.FormationAttrs,
//                     };
//                     try txn.conn.push(alloc.gpa, io, formation_attr_notify);
//                 }
//             }
//             break;
//         }
//     } else {
//         return error.PlayerNotFoundInScene;
//     }
//     txn.respond(.{});
// }

fn buildValidatedFormationRoles(
    role_comp: *const PlayerRoleComponent,
    role_ids: []const i32,
    old_formation: ?FormationInfo.Formation,
) ?[3]?FormationInfo.Formation.Role {
    var roles: [3]?FormationInfo.Formation.Role = .{ null, null, null };
    var count: usize = 0;

    for (role_ids) |role_id| {
        if (role_id <= 0 or !role_comp.role_map.contains(role_id)) return null;
        const dup = for (roles[0..count]) |maybe_existing| {
            if (maybe_existing) |existing| if (existing.role_id == role_id) break true;
        } else false;
        if (dup) continue;
        if (count >= 3) break;

        const existing_entity_id: i64 = if (old_formation) |old| blk: {
            for (old.roles) |maybe_existing| {
                if (maybe_existing) |existing| {
                    if (existing.role_id == role_id) break :blk existing.entity_id;
                }
            }
            break :blk -1;
        } else -1;

        roles[count] = .{
            .role_id = role_id,
            .entity_id = existing_entity_id,
            .on_stage_without_control = false,
        };
        count += 1;
    }
    return roles;
}

fn hasAnyFormationRole(roles: [3]?FormationInfo.Formation.Role) bool {
    for (roles) |maybe_role| {
        if (maybe_role != null) return true;
    }
    return false;
}

fn containsFormationRole(roles: [3]?FormationInfo.Formation.Role, role_id: i32) bool {
    if (role_id <= 0) return false;
    for (roles) |maybe_role| {
        if (maybe_role) |r| if (r.role_id == role_id) return true;
    }
    return false;
}

fn firstFormationRoleId(roles: [3]?FormationInfo.Formation.Role) ?i32 {
    for (roles) |maybe_role| {
        if (maybe_role) |r| return r.role_id;
    }
    return null;
}

pub fn onUpdateFormationRequest(
    txn: *Transaction(pb.UpdateFormationRequest),
    events: *EventQueue,
    alloc: mem.Alloc,
    fs: *FileSystem,
    scene: *Scene,
    assets: *const Assets,
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    buff_timers: *BuffTimerScheduler,
    qctx: QuestEngine.QuestContext,
    io: std.Io,
) !void {
    const log = std.log.scoped(.formation_update);
    const now_ms = (std.Io.Clock.awake).now(io).toMilliseconds();

    for (txn.message.Formations.items) |fight_formation| {
        if (fight_formation.FormationId < 1 or fight_formation.FormationId > @as(i32, @intCast(scene.formation_info.formations.len))) {
            txn.respond(.{ .ErrorCode = .ErrFormationIdOutOfRange });
            return;
        }
        const validated = buildValidatedFormationRoles(role_comp, fight_formation.RoleIds.items, null) orelse {
            txn.respond(.{ .ErrorCode = .ErrFightFormationRoleNotExist });
            return;
        };
        if (fight_formation.IsCurrent and !hasAnyFormationRole(validated)) {
            txn.respond(.{ .ErrorCode = .ErrFormationEmpty });
            return;
        }
    }

    for (txn.message.Formations.items) |fight_formation| {
        const idx: usize = @intCast(fight_formation.FormationId - 1);
        const existing_name = scene.formation_info.formations[idx].name;
        const is_current = fight_formation.IsCurrent;

        if (is_current) {
            const cur_idx: usize = @intCast(@max(0, scene.formation_info.cur_formation));
            const old_formation = scene.formation_info.formations[cur_idx];
            scene.formation_info.cur_formation = @intCast(idx);
            const current_formation = &scene.formation_info.formations[idx];
            const roles = buildValidatedFormationRoles(role_comp, fight_formation.RoleIds.items, old_formation) orelse {
                txn.respond(.{ .ErrorCode = .ErrFightFormationRoleNotExist });
                return;
            };
            current_formation.* = .{
                .cur_role = fight_formation.CurRole,
                .name = existing_name,
                .roles = roles,
            };

            var added_roles: std.ArrayList(i32) = .empty;
            for (current_formation.roles) |maybe_role| {
                const role = maybe_role orelse continue;
                const contained_in_old = blk: {
                    for (old_formation.roles) |maybe_old_role| {
                        const old_role = maybe_old_role orelse continue;
                        if (old_role.role_id == role.role_id) break :blk true;
                    }
                    break :blk false;
                };
                if (!contained_in_old) {
                    try added_roles.append(alloc.arena, role.role_id);
                }
            }

            var removed_entity_ids: std.ArrayList(i64) = .empty;
            for (old_formation.roles) |maybe_role| {
                const role = maybe_role orelse continue;
                const contained_in_current = blk: {
                    for (current_formation.roles) |maybe_cf_role| {
                        const cf_role = maybe_cf_role orelse continue;
                        if (cf_role.role_id == role.role_id) break :blk true;
                    }
                    break :blk false;
                };
                if (!contained_in_current) {
                    try removed_entity_ids.append(alloc.arena, role.entity_id);
                }
            }

            log.debug("added: {any}, removed: {any}", .{ added_roles.items, removed_entity_ids.items });

            var remove_notify: pb.EntityRemoveNotify = .{ .IsRemove = true };
            var remove_infos: std.ArrayList(pb.EntityRemoveInfo) = .empty;
            defer remove_infos.deinit(alloc.gpa);
            for (removed_entity_ids.items) |entity_id| {
                const entity_index = scene.net_id_map.get(entity_id) orelse continue;
                const storage = scene.entities.get(entity_index);
                if (storage.concomitant) |concomitant| {
                    for (concomitant.vision_entity_id) |concom_id| {
                        try scene.remove(alloc.gpa, fs, concom_id);
                        try remove_infos.append(alloc.gpa, .{ .EntityId = concom_id });
                    }
                    for (concomitant.custom_entity_ids) |concom_id| {
                        try scene.remove(alloc.gpa, fs, concom_id);
                        try remove_infos.append(alloc.gpa, .{ .EntityId = concom_id });
                    }
                }

                try scene.remove(alloc.gpa, fs, entity_id);
                try remove_infos.append(alloc.gpa, .{ .EntityId = entity_id });
            }
            remove_notify.RemoveInfos = remove_infos;
            try txn.conn.push(alloc.gpa, io, remove_notify);

            const instance_dungeon = assets.tables.instance_dungeon.getDataById(scene.instance_id) orelse
                return error.InstanceDungeonNotFound;

            var role_entity_pbs: std.ArrayList(pb.EntityPb) = .empty;
            defer role_entity_pbs.deinit(alloc.gpa);
            var concom_entity_pbs: std.ArrayList(pb.EntityPb) = .empty;
            defer concom_entity_pbs.deinit(alloc.gpa);

            for (added_roles.items) |role_id| {
                const entity = try RoleEntityTemplates.createRoleEntity(
                    fs,
                    scene,
                    alloc,
                    scene.player_id,
                    assets,
                    role_comp,
                    weapon_comp,
                    echo_comp,
                    instance_dungeon,
                    role_id,
                );
                for (&current_formation.roles) |*maybe_slot| {
                    const slot = &(maybe_slot.* orelse continue);
                    if (slot.role_id == role_id) {
                        slot.entity_id = entity.net_id;
                        break;
                    }
                }
                const storage = scene.entities.get(entity.index);
                const entity_pb = try entity_proto.build(
                    alloc,
                    assets,
                    scene,
                    buff_timers,
                    entity.net_id,
                    now_ms,
                );
                try role_entity_pbs.append(alloc.gpa, entity_pb);

                if (storage.concomitant) |concomitant| {
                    for (concomitant.vision_entity_id) |concom_id| {
                        if (!scene.net_id_map.contains(concom_id)) continue;
                        const concom_pb = try entity_proto.build(
                            alloc,
                            assets,
                            scene,
                            buff_timers,
                            concom_id,
                            now_ms,
                        );
                        try concom_entity_pbs.append(alloc.gpa, concom_pb);
                    }
                    for (concomitant.custom_entity_ids) |concom_id| {
                        if (!scene.net_id_map.contains(concom_id)) continue;
                        const concom_pb = try entity_proto.build(
                            alloc,
                            assets,
                            scene,
                            buff_timers,
                            concom_id,
                            now_ms,
                        );
                        try concom_entity_pbs.append(alloc.gpa, concom_pb);
                    }
                }
            }

            var role_entity_add_notify: pb.EntityAddNotify = .{ .RemoveTagIds = false };
            role_entity_add_notify.EntityPbs = role_entity_pbs;
            try txn.conn.push(alloc.gpa, io, role_entity_add_notify);
            var concom_entity_add_notify: pb.EntityAddNotify = .{ .RemoveTagIds = false };
            concom_entity_add_notify.EntityPbs = concom_entity_pbs;
            try txn.conn.push(alloc.gpa, io, concom_entity_add_notify);

            current_formation.cur_role = blk: {
                if (containsFormationRole(current_formation.roles, current_formation.cur_role)) {
                    break :blk current_formation.cur_role;
                }
                if (containsFormationRole(current_formation.roles, old_formation.cur_role)) {
                    break :blk old_formation.cur_role;
                }
                const slot = slot_blk: {
                    for (old_formation.roles, 0..) |maybe_role, i| {
                        const role = maybe_role orelse continue;
                        if (role.role_id == old_formation.cur_role) break :slot_blk i;
                    }
                    break :slot_blk 0;
                };

                var i: usize = slot;
                while (true) {
                    if (current_formation.roles[i]) |r| break :blk r.role_id;
                    if (i == 0) break;
                    i -= 1;
                }

                break :blk firstFormationRoleId(current_formation.roles) orelse 0;
            };

            var fight_role_infos: std.ArrayList(pb.FightRoleInfo) = .empty;
            defer fight_role_infos.deinit(alloc.gpa);
            for (current_formation.roles) |maybe_slot| {
                const slot = maybe_slot orelse continue;
                try fight_role_infos.append(alloc.gpa, .{
                    .RoleId = slot.role_id,
                    .EntityId = slot.entity_id,
                    .OnStageWithoutControl = false,
                });
            }
            log.debug("fight_role_infos: {any}", .{fight_role_infos});

            var fight_role_infos_wrapper: std.ArrayList(pb.FightRoleInfos) = .empty;
            defer fight_role_infos_wrapper.deinit(alloc.gpa);
            try fight_role_infos_wrapper.append(alloc.gpa, .{
                .GroupType = 1,
                .FightRoleInfos = fight_role_infos,
                .CurRole = current_formation.cur_role,
                .LivingStatus = RoleHelper.formationLivingStatus(scene, current_formation.*),
                .IsFixedLocation = false,
            });

            var group_formations: std.ArrayList(pb.GroupFormation) = .empty;
            defer group_formations.deinit(alloc.gpa);
            try group_formations.append(alloc.gpa, .{
                .PlayerId = scene.player_id,
                .FightRoleInfos = fight_role_infos_wrapper,
                .CurrentGroupType = 1,
            });

            try txn.conn.push(alloc.gpa, io, pb.UpdateGroupFormationNotify{
                .GroupFormation = group_formations,
            });
        } else {
            const roles = buildValidatedFormationRoles(role_comp, fight_formation.RoleIds.items, null) orelse {
                txn.respond(.{ .ErrorCode = .ErrFightFormationRoleNotExist });
                return;
            };
            const resolved_cur_role = if (containsFormationRole(roles, fight_formation.CurRole))
                fight_formation.CurRole
            else
                firstFormationRoleId(roles) orelse 0;
            scene.formation_info.formations[idx] = .{
                .cur_role = resolved_cur_role,
                .name = existing_name,
                .roles = roles,
            };
        }
    }

    try buff_timers.ensureAllRegistered(alloc.gpa, scene, assets, now_ms);
    buff_timers.syncAllLeftDurations(scene, now_ms);
    try scene.savePersistent(fs, alloc.gpa);
    try events.enqueue(.update_formations, .{});
    txn.respond(.{ .ErrorCode = .Success });
    try QuestEngine.onConditionStateChanged(qctx);
}

pub fn onFormationDataRequest(
    txn: *Transaction(pb.GetFormationDataRequest),
    alloc: mem.Alloc,
    scene_opt: *?Scene,
) !void {
    if (scene_opt.*) |*scene| {
        const fight_formations: std.ArrayList(pb.FightFormation) = try build_fight_formations(alloc, scene);
        txn.respond(.{ .ErrorCode = .Success, .Formations = fight_formations });
    } else {
        txn.respond(.{ .ErrorCode = .Success });
    }
}

pub fn onRenameFormationRequest(
    txn: *Transaction(pb.RenameFormationRequest),
    events: *EventQueue,
    alloc: mem.Alloc,
    fs: *FileSystem,
    scene: *Scene,
) !void {
    if (txn.message.FormationId < 1 or txn.message.FormationId > @as(i32, @intCast(scene.formation_info.formations.len))) {
        txn.respond(.{ .ErrorCode = .ErrFormationIdOutOfRange });
        return;
    }
    const trimmed = std.mem.trim(u8, txn.message.Name, " \t\r\n");
    if (trimmed.len == 0 or trimmed.len > 64) {
        txn.respond(.{ .ErrorCode = .ErrFightFormationNameLength });
        return;
    }
    const idx: usize = @intCast(txn.message.FormationId - 1);
    const owned = try alloc.gpa.dupe(u8, trimmed);
    if (scene.formation_info.formations[idx].name) |old_name| {
        if (old_name.len != 0) alloc.gpa.free(old_name);
    }
    scene.formation_info.formations[idx].name = owned;
    try scene.savePersistent(fs, alloc.gpa);
    try events.enqueue(.update_formations, .{});
    txn.respond(.{ .ErrorCode = .Success });
}
