const std = @import("std");
const pb = @import("proto").pb;
const Scene = @import("../../logic/Scene.zig");
const Assets = @import("../../data/Assets.zig");
const FileSystem = @import("common").FileSystem;
const Connection = @import("../../network/Connection.zig");
const mem = @import("../../mem.zig");
const comp_util = @import("../../logic/component/comp_util.zig");
const RoleInfo = @import("../../fs/RoleInfo.zig");
const WeaponItem = @import("../../fs/WeaponItem.zig");
const special_item_incr = @import("../../fs/special_item_incr.zig");
const PlayerBasicComponent = @import("../../logic/component/player/PlayerBasicComponent.zig");
const PlayerRoleComponent = @import("../../logic/component/player/PlayerRoleComponent.zig");
const PlayerWeaponComponent = @import("../../logic/component/player/PlayerWeaponComponent.zig");
const PlayerEchoComponent = @import("../../logic/component/player/PlayerEchoComponent.zig");
const RoleEntityTemplates = @import("../../logic/templates/RoleEntityTemplates.zig");
const BuffTimerScheduler = @import("../schedulers/BuffTimerScheduler.zig");
const EventQueue = @import("../EventQueue.zig");
const Events = @import("../events.zig");
const FormationInfo = @import("../../fs/FormationInfo.zig");
const attributes_helper = @import("attributes.zig");
const entity_proto = @import("entity_proto.zig");

pub const MainRoleTransferResult = struct {
    formation_changed: bool = false,
    basic_changed: bool = false,
};

pub fn mainRoleIdForElement(assets: *const Assets, gender: i32, element_id: i32) ?i32 {
    for (assets.tables.main_role_config.items) |config| {
        if (config.Gender != gender) continue;

        const role_config = assets.tables.role_info.getDataById(config.Id) orelse continue;
        if (role_config.ElementId == element_id) return config.Id;
    }

    return null;
}

pub fn mainRoleUnlockList(assets: *const Assets, gender: i32, arena: std.mem.Allocator) !std.ArrayList(i32) {
    var list: std.ArrayList(i32) = .empty;
    for (assets.tables.main_role_config.items) |config| {
        if (config.Gender == gender) try list.append(arena, config.Id);
    }

    return list;
}

pub fn isMainRole(assets: *const Assets, role_id: i32) bool {
    return assets.tables.main_role_config.getDataById(role_id) != null;
}

fn currentMainRoleId(assets: *const Assets, role_comp: *PlayerRoleComponent, gender: i32) ?i32 {
    for (assets.tables.main_role_config.items) |config| {
        if (config.Gender == gender and role_comp.role_map.contains(config.Id)) return config.Id;
    }

    for (assets.tables.main_role_config.items) |config| {
        if (role_comp.role_map.contains(config.Id)) return config.Id;
    }

    return null;
}

pub fn selectedMainRoleId(
    assets: *const Assets,
    role_comp: *PlayerRoleComponent,
    gender: i32,
    selected_role_id: i32,
) ?i32 {
    if (selected_role_id != 0 and
        isMainRoleForGender(assets, gender, selected_role_id) and
        role_comp.role_map.contains(selected_role_id))
    {
        return selected_role_id;
    }

    return currentMainRoleId(assets, role_comp, gender);
}

pub fn isMainRoleForGender(assets: *const Assets, gender: i32, role_id: i32) bool {
    const config = assets.tables.main_role_config.getDataById(role_id) orelse return false;
    return config.Gender == gender;
}

pub fn currentFormationMainRoleId(assets: *const Assets, scene: *const Scene, gender: i32) ?i32 {
    if (scene.formation_info.formations.len == 0 or scene.formation_info.cur_formation < 0) return null;

    const formation_index: usize = @intCast(scene.formation_info.cur_formation);
    if (formation_index >= scene.formation_info.formations.len) return null;

    const formation = scene.formation_info.formations[formation_index];
    if (isMainRoleForGender(assets, gender, formation.cur_role)) return formation.cur_role;

    for (formation.roles) |maybe_role| {
        const role = maybe_role orelse continue;
        if (isMainRoleForGender(assets, gender, role.role_id)) return role.role_id;
    }

    return null;
}

fn replaceFormationRole(scene: *Scene, source_role_id: i32, target_role_id: i32) bool {
    if (source_role_id == target_role_id) return false;

    var changed = false;
    for (&scene.formation_info.formations) |*formation| {
        if (formation.cur_role == source_role_id) {
            formation.cur_role = target_role_id;
            changed = true;
        }

        for (&formation.roles) |*maybe_role| {
            const role = &(maybe_role.* orelse continue);
            if (role.role_id != source_role_id) continue;

            role.role_id = target_role_id;
            changed = true;
        }
    }

    return changed;
}

pub fn saveBasicInfo(fs: *FileSystem, arena: std.mem.Allocator, basic_comp: *const PlayerBasicComponent) !void {
    const path = try std.fmt.allocPrint(arena, "player/{}/basic_info", .{basic_comp.player_id});
    try comp_util.saveStruct(fs, basic_comp.info, path, arena);
}

pub fn saveRoleInfo(fs: *FileSystem, arena: std.mem.Allocator, role_comp: *const PlayerRoleComponent, role_id: i32) !void {
    if (role_comp.isTrialRole(role_id)) return;
    const role_info = role_comp.role_map.getPtr(role_id) orelse return;
    const path = try std.fmt.allocPrint(arena, "player/{}/role/{}", .{ role_comp.player_id, role_id });
    try comp_util.saveStruct(fs, role_info, path, arena);
}

pub fn setSelectedMainRole(basic_comp: *PlayerBasicComponent, role_id: i32) bool {
    if (basic_comp.info.selected_main_role_id == role_id) return false;
    basic_comp.info.selected_main_role_id = role_id;
    return true;
}

fn replaceBasicShowRole(basic_comp: *PlayerBasicComponent, source_role_id: i32, target_role_id: i32) bool {
    if (source_role_id == target_role_id) return false;

    var changed = false;
    for (basic_comp.info.role_show_list) |*role_id| {
        if (role_id.* != source_role_id) continue;

        role_id.* = target_role_id;
        changed = true;
    }

    return changed;
}

fn clearWeaponOwnersExcept(weapon_comp: *PlayerWeaponComponent, role_id: i32, keep_incr_id: i32) bool {
    var changed = false;
    var iterator = weapon_comp.weapon_map.iterator();
    while (iterator.next()) |kv| {
        if (kv.key_ptr.* == keep_incr_id) continue;
        if (kv.value_ptr.role_id == null or kv.value_ptr.role_id.? != role_id) continue;
        kv.value_ptr.role_id = null;
        changed = true;
    }

    return changed;
}

fn firstOwnedWeaponId(weapon_comp: *PlayerWeaponComponent, role_id: i32) ?i32 {
    var iterator = weapon_comp.weapon_map.iterator();
    while (iterator.next()) |kv| {
        if (kv.value_ptr.role_id == null or kv.value_ptr.role_id.? != role_id) continue;
        return kv.key_ptr.*;
    }

    return null;
}

fn replaceEchoRoleId(echo_comp: *PlayerEchoComponent, source_role_id: i32, target_role_id: i32) bool {
    if (source_role_id == target_role_id) return false;

    var changed = false;
    var iterator = echo_comp.echo_map.iterator();
    while (iterator.next()) |kv| {
        if (kv.value_ptr.role_id == null or kv.value_ptr.role_id.? != target_role_id) continue;

        kv.value_ptr.role_id = null;
        changed = true;
    }

    iterator = echo_comp.echo_map.iterator();
    while (iterator.next()) |kv| {
        if (kv.value_ptr.role_id == null or kv.value_ptr.role_id.? != source_role_id) continue;

        kv.value_ptr.role_id = target_role_id;
        changed = true;
    }

    return changed;
}

fn normalizeTargetRoleInfo(gpa: std.mem.Allocator, assets: *const Assets, role_info: *RoleInfo, target_role_id: i32) !void {
    const target_config = assets.tables.role_info.getDataById(target_role_id) orelse return error.RoleNotFound;

    role_info.role_skin_id = target_config.SkinId;
    role_info.cur_model = 0;
    if (role_info.models.len != 0) {
        gpa.free(role_info.models);
        role_info.models = &.{};
    }
    try role_info.resetProperties(gpa, assets, target_role_id);
}

fn skillLevel(role_info: *const RoleInfo, skill_id: i32) ?i32 {
    for (role_info.skills) |entry| {
        if (entry[0] == skill_id) return entry[1];
    }

    return null;
}

fn skillNodeState(role_info: *const RoleInfo, node_id: i32) ?RoleInfo.SkillNode {
    for (role_info.skill_node_state) |node| {
        if (node.node_id == node_id) return node;
    }

    return null;
}

fn matchingSkillTreeNode(
    assets: *const Assets,
    node_group: i32,
    node_index: i32,
    node_type: i32,
) ?Assets.DataTables.SkillTree {
    for (assets.tables.skill_tree.items) |node| {
        if (node.NodeGroup == node_group and
            node.NodeIndex == node_index and
            node.NodeType == node_type)
        {
            return node;
        }
    }

    return null;
}

fn maxResonantChainIndex(assets: *const Assets, group_id: i32) i32 {
    var max_index: i32 = 0;
    for (assets.tables.resonant_chain.items) |chain| {
        if (chain.GroupId == group_id and chain.GroupIndex > max_index) max_index = chain.GroupIndex;
    }

    return max_index;
}

fn copyMainRoleProgression(
    gpa: std.mem.Allocator,
    assets: *const Assets,
    source_role_id: i32,
    source_role: *const RoleInfo,
    target_role_id: i32,
    target_role: *RoleInfo,
) !void {
    const source_config = assets.tables.role_info.getDataById(source_role_id) orelse return error.RoleNotFound;
    const target_config = assets.tables.role_info.getDataById(target_role_id) orelse return error.RoleNotFound;

    var skills: std.ArrayList(@typeInfo(@TypeOf(target_role.skills)).pointer.child) = .empty;
    errdefer skills.deinit(gpa);
    var nodes: std.ArrayList(RoleInfo.SkillNode) = .empty;
    errdefer nodes.deinit(gpa);

    for (assets.tables.skill_tree.items) |target_node| {
        if (target_node.NodeGroup != target_config.SkillTreeGroupId) continue;

        const source_node = matchingSkillTreeNode(
            assets,
            source_config.SkillTreeGroupId,
            target_node.NodeIndex,
            target_node.NodeType,
        );
        const default_active = target_node.Condition.len == 0 and target_node.UnLockCondition == 0;
        const source_state = if (source_node) |node| skillNodeState(source_role, node.Id) else null;
        const active = if (source_state) |state| state.active else default_active;
        const source_level = if (source_node) |node|
            if (node.SkillId != 0) skillLevel(source_role, node.SkillId) else null
        else
            null;
        const default_level: i32 = if (active and target_node.SkillId != 0) 1 else 0;
        const level = source_level orelse default_level;

        try nodes.append(gpa, .{
            .node_id = target_node.Id,
            .active = active,
            .skill_id = target_node.SkillId,
        });
        if (active and target_node.SkillId != 0) {
            try skills.append(gpa, .{ target_node.SkillId, level });
        }
    }

    const new_skills = try skills.toOwnedSlice(gpa);
    errdefer gpa.free(new_skills);
    const new_nodes = try nodes.toOwnedSlice(gpa);
    errdefer gpa.free(new_nodes);

    if (target_role.skills.len != 0) gpa.free(target_role.skills);
    if (target_role.skill_node_state.len != 0) gpa.free(target_role.skill_node_state);

    target_role.level = source_role.level;
    target_role.exp = source_role.exp;
    target_role.breakthrough = source_role.breakthrough;
    target_role.star = source_role.star;
    target_role.favor = source_role.favor;
    target_role.calabash_skin_id = source_role.calabash_skin_id;
    target_role.resonant_chain_group_index = @min(
        source_role.resonant_chain_group_index,
        maxResonantChainIndex(assets, target_config.ResonantChainGroupId),
    );
    target_role.skills = new_skills;
    target_role.skill_node_state = new_nodes;
}

fn removeEquipmentCache(role_comp: *PlayerRoleComponent, gpa: std.mem.Allocator, role_id: i32) void {
    if (role_comp.equipment_map.fetchSwapRemove(role_id)) |removed| {
        var equipment = removed.value;
        equipment.deinit(gpa);
    }
}

fn createDefaultWeapon(
    fs: *FileSystem,
    alloc: mem.Alloc,
    assets: *const Assets,
    role_comp: *const PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    role_id: i32,
) !i32 {
    const role_config = assets.tables.role_info.getDataById(role_id) orelse return error.RoleNotFound;
    const weapon: WeaponItem = .{
        .id = role_config.InitWeaponItemId,
        .func_value = WeaponItem.defaultFuncValue(assets, role_config.InitWeaponItemId),
        .role_id = role_id,
    };

    const incr_id = try special_item_incr.next(alloc.gpa, fs, role_comp.player_id);
    try weapon_comp.weapon_map.put(alloc.gpa, incr_id, weapon);
    return incr_id;
}

pub fn ensureRoleWeapon(
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    role_id: i32,
) !bool {
    var role_changed = false;
    var weapon_changed = false;
    const role_info = role_comp.role_map.getPtr(role_id) orelse return error.RoleNotFound;

    if (role_info.weapon != 0) {
        if (weapon_comp.weapon_map.getPtr(role_info.weapon)) |weapon_info| {
            if (weapon_info.role_id == null or weapon_info.role_id.? != role_id) {
                weapon_info.role_id = role_id;
                weapon_changed = true;
            }
            if (clearWeaponOwnersExcept(weapon_comp, role_id, role_info.weapon)) weapon_changed = true;

            if (weapon_changed) {
                try PlayerWeaponComponent.saveAll(alloc.gpa, fs, weapon_comp.player_id, weapon_comp.weapon_map);
                removeEquipmentCache(role_comp, alloc.gpa, role_id);
            }
            return weapon_changed;
        }
    }

    if (firstOwnedWeaponId(weapon_comp, role_id)) |owned_weapon_id| {
        role_info.weapon = owned_weapon_id;
        role_changed = true;
        if (clearWeaponOwnersExcept(weapon_comp, role_id, owned_weapon_id)) weapon_changed = true;
    } else {
        role_info.weapon = try createDefaultWeapon(fs, alloc, assets, role_comp, weapon_comp, role_id);
        role_changed = true;
        weapon_changed = true;
    }

    if (role_changed) try saveRoleInfo(fs, alloc.arena, role_comp, role_id);
    if (weapon_changed) {
        try PlayerWeaponComponent.saveAll(alloc.gpa, fs, weapon_comp.player_id, weapon_comp.weapon_map);
        removeEquipmentCache(role_comp, alloc.gpa, role_id);
    }

    return role_changed or weapon_changed;
}

fn moveSelectedWeapon(
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    source_role_id: i32,
    target_role_id: i32,
) bool {
    var changed = false;
    if (source_role_id == target_role_id) return false;

    const source_role = role_comp.role_map.getPtr(source_role_id) orelse return false;
    const target_role = role_comp.role_map.getPtr(target_role_id) orelse return false;

    const selected_weapon_id = blk: {
        if (source_role.weapon != 0 and weapon_comp.weapon_map.contains(source_role.weapon)) {
            break :blk source_role.weapon;
        }
        break :blk firstOwnedWeaponId(weapon_comp, source_role_id) orelse 0;
    };
    if (selected_weapon_id == 0) return false;

    var iterator = weapon_comp.weapon_map.iterator();
    while (iterator.next()) |kv| {
        if (kv.key_ptr.* == selected_weapon_id) continue;
        if (kv.value_ptr.role_id == null) continue;
        if (kv.value_ptr.role_id.? == source_role_id or kv.value_ptr.role_id.? == target_role_id) {
            kv.value_ptr.role_id = null;
            changed = true;
        }
    }

    if (weapon_comp.weapon_map.getPtr(selected_weapon_id)) |weapon_info| {
        if (weapon_info.role_id == null or weapon_info.role_id.? != target_role_id) {
            weapon_info.role_id = target_role_id;
            changed = true;
        }
    }

    if (target_role.weapon != selected_weapon_id) {
        target_role.weapon = selected_weapon_id;
        changed = true;
    }
    if (source_role.weapon == selected_weapon_id) {
        source_role.weapon = 0;
        changed = true;
    }

    return changed;
}

pub fn transferMainRoleState(
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    scene: ?*Scene,
    basic_comp: *PlayerBasicComponent,
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    source_role_id: i32,
    target_role_id: i32,
) !MainRoleTransferResult {
    var result: MainRoleTransferResult = .{};
    var role_changed = false;
    var weapon_changed = false;
    var echo_changed = false;

    if (!isMainRole(assets, target_role_id)) return error.RoleNotFound;
    if (role_comp.role_map.contains(source_role_id)) {
        _ = try ensureRoleWeapon(alloc, fs, assets, role_comp, weapon_comp, source_role_id);
    }
    if (!role_comp.role_map.contains(target_role_id)) {
        var new_role = try RoleInfo.createDefault(alloc.gpa, assets, target_role_id);
        errdefer new_role.deinit(alloc.gpa);
        try role_comp.role_map.put(alloc.gpa, target_role_id, new_role);
        role_changed = true;
    }

    if (source_role_id == target_role_id) {
        result.basic_changed = setSelectedMainRole(basic_comp, target_role_id);
        if (role_changed) {
            if (role_comp.role_map.getPtr(target_role_id)) |target_role| {
                try normalizeTargetRoleInfo(alloc.gpa, assets, target_role, target_role_id);
            }
            _ = try ensureRoleWeapon(alloc, fs, assets, role_comp, weapon_comp, target_role_id);
            _ = try role_comp.rebuildEquipment(alloc.gpa, assets, target_role_id, weapon_comp, echo_comp);
            try saveRoleInfo(fs, alloc.arena, role_comp, target_role_id);
        }
        return result;
    }

    result.basic_changed = setSelectedMainRole(basic_comp, target_role_id) or
        replaceBasicShowRole(basic_comp, source_role_id, target_role_id);
    echo_changed = replaceEchoRoleId(echo_comp, source_role_id, target_role_id);
    if (scene) |scene_ptr| result.formation_changed = replaceFormationRole(scene_ptr, source_role_id, target_role_id);

    const target_role = role_comp.role_map.getPtr(target_role_id) orelse return error.RoleNotFound;
    if (role_comp.role_map.getPtr(source_role_id)) |source_role| {
        try copyMainRoleProgression(alloc.gpa, assets, source_role_id, source_role, target_role_id, target_role);
    }
    try normalizeTargetRoleInfo(alloc.gpa, assets, target_role, target_role_id);
    role_changed = true;

    if (moveSelectedWeapon(role_comp, weapon_comp, source_role_id, target_role_id)) {
        role_changed = true;
        weapon_changed = true;
    }

    _ = try ensureRoleWeapon(alloc, fs, assets, role_comp, weapon_comp, target_role_id);

    removeEquipmentCache(role_comp, alloc.gpa, source_role_id);
    removeEquipmentCache(role_comp, alloc.gpa, target_role_id);
    _ = try role_comp.rebuildEquipment(alloc.gpa, assets, target_role_id, weapon_comp, echo_comp);

    if (role_changed) try saveRoleInfo(fs, alloc.arena, role_comp, target_role_id);
    if (role_comp.role_map.contains(source_role_id)) try saveRoleInfo(fs, alloc.arena, role_comp, source_role_id);
    if (weapon_changed) try PlayerWeaponComponent.saveAll(alloc.gpa, fs, weapon_comp.player_id, weapon_comp.weapon_map);
    if (echo_changed) try PlayerEchoComponent.saveAll(alloc.gpa, fs, echo_comp.player_id, echo_comp.echo_map);

    return result;
}

pub fn resetRoles(
    scene: *Scene,
    fs: *FileSystem,
    assets: *const Assets,
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    conn: *Connection,
    alloc: mem.Alloc,
    role_ids: ?[]const i32,
    buff_timers: *BuffTimerScheduler,
    now_ms: i64,
    io: std.Io,
) !void {
    const log = std.log.scoped(.reset_formation);
    const instance_dungeon = assets.tables.instance_dungeon.getDataById(scene.instance_id) orelse {
        log.err(
            "player({d}) last scene instance id {d} doesn't exist",
            .{ scene.player_id, scene.instance_id },
        );
        return;
    };

    if (scene.formation_info.cur_formation < 0) return;
    const cur_form_idx: usize = @intCast(scene.formation_info.cur_formation);
    if (cur_form_idx >= scene.formation_info.formations.len) return;
    const formation = &scene.formation_info.formations[cur_form_idx];

    var removed_entity_ids: std.ArrayList(i64) = .empty;
    for (formation.*.roles) |maybe_role| {
        const role = maybe_role orelse continue;
        if (role_ids) |ids| {
            const should_reset = for (ids) |id| {
                if (id == role.role_id) break true;
            } else false;
            if (!should_reset) continue;
        }
        try removed_entity_ids.append(alloc.arena, role.entity_id);
    }

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
    try conn.push(alloc.gpa, io, remove_notify);

    var role_entity_pbs: std.ArrayList(pb.EntityPb) = .empty;
    defer role_entity_pbs.deinit(alloc.gpa);
    var concom_entity_pbs: std.ArrayList(pb.EntityPb) = .empty;
    defer concom_entity_pbs.deinit(alloc.gpa);

    for (&formation.roles) |*maybe_role| if (maybe_role.*) |*role| {
        if (role_ids) |ids| {
            const should_reset = for (ids) |id| {
                if (id == role.role_id) break true;
            } else false;
            if (!should_reset) continue;
        }
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
            role.role_id,
        );
        role.entity_id = entity.net_id;
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
    };

    var role_entity_add_notify: pb.EntityAddNotify = .{ .RemoveTagIds = false };
    role_entity_add_notify.EntityPbs = role_entity_pbs;
    try conn.push(alloc.gpa, io, role_entity_add_notify);
    var concom_entity_add_notify: pb.EntityAddNotify = .{ .RemoveTagIds = false };
    concom_entity_add_notify.EntityPbs = concom_entity_pbs;
    try conn.push(alloc.gpa, io, concom_entity_add_notify);

    var fight_role_infos: std.ArrayList(pb.FightRoleInfo) = .empty;
    defer fight_role_infos.deinit(alloc.gpa);
    for (formation.roles) |maybe_slot| {
        const slot = maybe_slot orelse continue;
        try fight_role_infos.append(alloc.gpa, .{
            .RoleId = slot.role_id,
            .EntityId = slot.entity_id,
            .OnStageWithoutControl = false,
        });
    }

    var fight_role_infos_wrapper: std.ArrayList(pb.FightRoleInfos) = .empty;
    defer fight_role_infos_wrapper.deinit(alloc.gpa);
    try fight_role_infos_wrapper.append(alloc.gpa, .{
        .GroupType = 1,
        .FightRoleInfos = fight_role_infos,
        .CurRole = formation.cur_role,
        .LivingStatus = formationLivingStatus(scene, formation.*),
        .IsFixedLocation = false,
    });

    var group_formations: std.ArrayList(pb.GroupFormation) = .empty;
    defer group_formations.deinit(alloc.gpa);
    try group_formations.append(alloc.gpa, .{
        .PlayerId = scene.player_id,
        .FightRoleInfos = fight_role_infos_wrapper,
        .CurrentGroupType = 1,
    });

    try conn.push(alloc.gpa, io, pb.UpdateGroupFormationNotify{
        .GroupFormation = group_formations,
    });
}

pub fn isRoleEntityAlive(scene: *const Scene, role_id: i32) bool {
    const slice = scene.entities.slice();
    for (slice.items(.config), slice.items(.attribute)) |cfg, maybe_attr| {
        if (cfg.entity_type != .player or cfg.config_type != .character) continue;
        if (cfg.config_id != role_id) continue;
        const attr = maybe_attr orelse return true;
        const max_hp = attributes_helper.get_attr_value(&attr, .LifeMax);
        if (max_hp <= 0) return true;
        return attributes_helper.get_attr_value(&attr, .Life) > 0;
    }
    return true;
}

pub fn formationLivingStatus(scene: *const Scene, formation: FormationInfo.Formation) pb.LivingStatus {
    var has_checked_entity = false;
    for (formation.roles) |maybe_slot| {
        const slot = maybe_slot orelse continue;
        if (slot.entity_id == 0) continue;
        const idx = scene.net_id_map.get(slot.entity_id) orelse continue;
        const attr = scene.entities.items(.attribute)[idx] orelse continue;
        const max_hp = attributes_helper.get_attr_value(&attr, .LifeMax);
        if (max_hp <= 0) continue;
        has_checked_entity = true;
        if (attributes_helper.get_attr_value(&attr, .Life) > 0) return .Alive;
    }
    return if (has_checked_entity) .Dead else .Alive;
}

pub fn ensureAliveCurRole(scene: *Scene, formation: *FormationInfo.Formation) bool {
    if (formation.cur_role > 0 and isRoleEntityAlive(scene, formation.cur_role)) return false;
    for (formation.roles) |maybe_slot| {
        const slot = maybe_slot orelse continue;
        if (!isRoleEntityAlive(scene, slot.role_id)) continue;
        const prev_role = formation.cur_role;
        if (prev_role == slot.role_id) return false;
        formation.cur_role = slot.role_id;
        const slice = scene.entities.slice();
        for (slice.items(.config), 0..) |cfg, i| {
            if (cfg.entity_type != .player or cfg.config_type != .character) continue;
            if (cfg.config_id == prev_role) {
                slice.items(.visible)[i] = null;
            } else if (cfg.config_id == slot.role_id) {
                slice.items(.visible)[i] = .{};
            }
        }
        return true;
    }
    return false;
}

pub fn isResonanceNexusConfig(assets: *const Assets, entity_config_id: i64) bool {
    if (entity_config_id == 0) return false;
    for (assets.tables.map_mark.items) |mark| {
        if (@as(i64, mark.EntityConfigId) == entity_config_id and mark.ObjectType == 8) return true;
    }
    return false;
}

pub fn isResonanceNexusTeleporter(assets: *const Assets, teleporter_id: i32) bool {
    const tp = assets.tables.teleporter.getDataById(teleporter_id) orelse return false;
    return isResonanceNexusConfig(assets, tp.TeleportEntityConfigId);
}

pub fn reviveAndHealFormation(
    conn: *Connection,
    scene: *Scene,
    role_comp: *PlayerRoleComponent,
    events: *EventQueue,
    fs: *FileSystem,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    var pack_data: std.ArrayList(pb.CombatReceiveData) = .empty;
    var any_revived = false;

    const slice = scene.entities.slice();
    for (slice.items(.entity_id), slice.items(.config), slice.items(.attribute)) |eid, cfg, *maybe_attr| {
        if (cfg.entity_type != .player or cfg.config_type != .character) continue;
        if (maybe_attr.*) |*attr| {
            const life_max = @max(1, attributes_helper.get_attr_value(attr, .LifeMax));
            const cur_life = attributes_helper.get_attr_value(attr, .Life);
            const was_dead = cur_life <= 0;
            if (was_dead or cur_life < life_max) {
                _ = attributes_helper.revive_entity(attr, null);
                if (was_dead) {
                    any_revived = true;
                    try pack_data.append(alloc.arena, .{ .Message = .{ .CombatNotifyData = .{
                        .CombatCommon = .{ .EntityId = eid.net_id },
                        .Message = .{ .EntityLivingStatusNotify = .{
                            .Id = eid.net_id,
                            .LivingStatus = .Alive,
                        } },
                    } } });
                }
                var changed: std.ArrayList(pb.EAttributeType) = .empty;
                try changed.append(alloc.arena, .Life);
                try attributes_helper.generate_attr_messages(&pack_data, eid.net_id, attr, &changed, alloc, io);
                try scene.markFsmDirty(alloc.gpa, eid.net_id, Scene.Entity.FsmComponent.WakeReason.attribute);
            }
        }
    }

    var role_it = role_comp.role_map.iterator();
    while (role_it.next()) |kv| {
        if (kv.value_ptr.current_hp != null) {
            kv.value_ptr.current_hp = null;
            if (!role_comp.isTrialRole(kv.key_ptr.*)) {
                events.enqueueBestEffort(.role_info_modified, .{ .role_id = kv.key_ptr.* });
                saveRoleInfo(fs, alloc.arena, role_comp, kv.key_ptr.*) catch |err| {
                    std.log.scoped(.role).warn("saveRoleInfo({d}) failed: {t}", .{ kv.key_ptr.*, err });
                };
            }
        }
    }

    if (pack_data.items.len != 0) {
        try conn.push(alloc.gpa, io, pb.CombatReceivePackNotify{ .Data = pack_data });
        const cur_eid = scene.currentRoleEntityId();
        if (cur_eid != 0) {
            if (scene.net_id_map.get(cur_eid)) |idx| {
                const target: Scene.Entity = .{ .index = idx, .net_id = cur_eid };
                if (alloc.arena.alloc(Events.BuffAdditionEntry, 1)) |entries| {
                    entries[0] = .{ .id = 640000001, .stack_count = 1, .is_active = true };
                    events.enqueueBestEffort(.buff_addition, .{
                        .target = target,
                        .instigator = target,
                        .buffs = entries,
                    });
                } else |_| {}
            }
        }
        conn.pushBestEffort(alloc.gpa, io, pb.JSPatchNotify{
            .Content = "try { const g = require('QuicklyRequire')('Game/Module/GenericPrompt/GenericPromptController'); if (g && g.GenericPromptController) g.GenericPromptController.ShowPromptByCode('30'); } catch (_) {}",
        });
    }
    if (any_revived) {
        try events.enqueue(.update_formations, .{});
    }
}

pub fn maybePushPartyWipeNotify(
    conn: *Connection,
    scene: *const Scene,
    alloc: mem.Alloc,
    io: std.Io,
) void {
    if (scene.formation_info.cur_formation < 0) return;
    const f_idx: usize = @intCast(scene.formation_info.cur_formation);
    if (f_idx >= scene.formation_info.formations.len) return;
    const form = scene.formation_info.formations[f_idx];
    if (formationLivingStatus(scene, form) == .Dead) {
        conn.pushBestEffort(alloc.gpa, io, pb.PlayerDeadNotify{
            .PlayerId = scene.player_id,
            .IsShowRevive = true,
        });
    }
}

/// Auto-heals damaged or defeated formation characters when the player moves
/// within 15m of any Resonance Nexus or Resonance Beacon in the current map.
pub fn healFormationIfNearNexus(
    conn: *Connection,
    scene: *Scene,
    assets: *const Assets,
    role_comp: *PlayerRoleComponent,
    events: *EventQueue,
    fs: *FileSystem,
    alloc: mem.Alloc,
    io: std.Io,
    player_loc: [3]f32,
) !void {
    var needs_heal = false;
    const slice = scene.entities.slice();
    for (slice.items(.config), slice.items(.attribute)) |cfg, maybe_attr| {
        if (cfg.entity_type != .player or cfg.config_type != .character) continue;
        if (maybe_attr) |attr| {
            const life_max = @max(1, attributes_helper.get_attr_value(&attr, .LifeMax));
            const cur_life = attributes_helper.get_attr_value(&attr, .Life);
            if (cur_life < life_max) {
                needs_heal = true;
                break;
            }
        }
    }
    if (!needs_heal) return;

    const map_id = assets.resolveMapConfigId(scene.instance_id);
    for (assets.indexes.teleporterIndicesForMap(map_id)) |tp_idx| {
        const tp = assets.tables.teleporter.items[tp_idx];
        const cfg = assets.findLevelEntity(tp.MapId, tp.TeleportEntityConfigId) orelse continue;
        const tp_pos = Assets.AssetIndexes.entityWorldPos(cfg) orelse continue;
        const dx = tp_pos[0] - player_loc[0];
        const dy = tp_pos[1] - player_loc[1];
        const dz = tp_pos[2] - player_loc[2];
        if (dx * dx + dy * dy + dz * dz <= 1500.0 * 1500.0) {
            try reviveAndHealFormation(conn, scene, role_comp, events, fs, alloc, io);
            return;
        }
    }
}
