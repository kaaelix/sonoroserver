const std = @import("std");
const common = @import("common");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const Assets = @import("../../data/Assets.zig");
const FileSystem = common.FileSystem;
const Scene = @import("../Scene.zig");
const Connection = @import("../../network/Connection.zig");
const PlayerBasicComponent = @import("../component/player/PlayerBasicComponent.zig");
const PlayerRoleComponent = @import("../component/player/PlayerRoleComponent.zig");
const PlayerWeaponComponent = @import("../component/player/PlayerWeaponComponent.zig");
const PlayerEchoComponent = @import("../component/player/PlayerEchoComponent.zig");
const PlayerQuestComponent = @import("../component/player/PlayerQuestComponent.zig");
const RoleInfo = @import("../../fs/RoleInfo.zig");
const WeaponItem = @import("../../fs/WeaponItem.zig");
const EchoInfo = @import("../../fs/EchoInfo.zig");
const QuestInfo = @import("../../fs/QuestInfo.zig");
const FormationInfo = @import("../../fs/FormationInfo.zig");
const RoleStats = @import("role_stats.zig");
const RoleHelper = @import("role.zig");
const RoleEntityTemplates = @import("../templates/RoleEntityTemplates.zig");
const EventQueue = @import("../EventQueue.zig");
const entity_proto = @import("entity_proto.zig");
const special_item_incr = @import("../../fs/special_item_incr.zig");

const Allocator = std.mem.Allocator;

fn nextTrialIncrId(map: anytype) i32 {
    var candidate: i32 = special_item_incr.trial_incr_min;
    while (map.contains(candidate)) {
        candidate += 1;
    }
    return candidate;
}

fn breachForRoleLevel(assets: *const Assets, breach_id: i32, level: i32) i32 {
    var best_breach: i32 = 0;
    for (assets.tables.role_breach.items) |entry| {
        if (entry.BreachGroupId != breach_id) continue;
        if (level > entry.MaxLevel and entry.BreachLevel + 1 > best_breach) {
            best_breach = entry.BreachLevel + 1;
        }
    }
    return best_breach;
}

fn breachForWeaponLevel(assets: *const Assets, weapon_id: i32, level: i32) i32 {
    const conf = assets.tables.weapon_conf.getDataById(weapon_id) orelse return 0;
    var best_breach: i32 = 0;
    for (assets.tables.weapon_breach.items) |entry| {
        if (entry.BreachId != conf.BreachId) continue;
        if (level > entry.LevelLimit and entry.Level + 1 > best_breach) {
            best_breach = entry.Level + 1;
        }
    }
    return best_breach;
}

fn containsI32(items: []const i32, target: i32) bool {
    for (items) |v| if (v == target) return true;
    return false;
}

fn cleanupTrialDataForRole(
    gpa: Allocator,
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    trial_id: i32,
) void {
    role_comp.removeEquipmentCache(gpa, trial_id);

    if (role_comp.role_map.fetchSwapRemove(trial_id)) |removed| {
        var role_val = removed.value;
        if (role_val.weapon >= special_item_incr.trial_incr_min) {
            if (weapon_comp.weapon_map.fetchSwapRemove(role_val.weapon)) |wrem| {
                var w = wrem.value;
                w.deinit(gpa);
            }
        }
        role_val.deinit(gpa);
    }
    _ = role_comp.trial_roles.remove(trial_id);

    var echo_keys: std.ArrayListUnmanaged(i32) = .empty;
    defer echo_keys.deinit(gpa);
    var it = echo_comp.echo_map.iterator();
    while (it.next()) |kv| {
        if (kv.key_ptr.* >= special_item_incr.trial_incr_min and kv.value_ptr.role_id == trial_id) {
            echo_keys.append(gpa, kv.key_ptr.*) catch {};
        }
    }
    for (echo_keys.items) |ek| {
        if (echo_comp.echo_map.fetchSwapRemove(ek)) |erem| {
            var e = erem.value;
            e.deinit(gpa);
        }
    }
}

pub fn materializeTrialRole(
    gpa: Allocator,
    assets: *const Assets,
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    trial_id: i32,
) !bool {
    const trial = assets.tables.trial_role_info.getDataById(trial_id) orelse return false;
    const parent_cfg = assets.tables.resolveBaseRoleInfo(trial_id) orelse return false;

    if (role_comp.role_map.contains(trial_id) and !role_comp.isTrialRole(trial_id)) return false;
    if (role_comp.isTrialRole(trial_id) and role_comp.role_map.contains(trial_id)) return true;

    const tw_opt = assets.tables.trial_weapon_info.getDataById(trial.TrailWeapon);
    const weapon_cfg_id = if (tw_opt) |tw| if (tw.WeaponId != 0) tw.WeaponId else parent_cfg.InitWeaponItemId else parent_cfg.InitWeaponItemId;
    const weapon_level = if (tw_opt) |tw| @max(1, tw.WeaponLevel) else 1;
    const weapon_reson = if (tw_opt) |tw| @max(1, tw.WeaponResonanceLevel) else 1;
    const weapon_skin = if (tw_opt) |tw| tw.WeaponSkinId else 0;
    const weapon_incr_id = nextTrialIncrId(&weapon_comp.weapon_map);

    try weapon_comp.weapon_map.put(gpa, weapon_incr_id, .{
        .id = weapon_cfg_id,
        .level = weapon_level,
        .breach = breachForWeaponLevel(assets, weapon_cfg_id, weapon_level),
        .reson_level = weapon_reson,
        .func_value = WeaponItem.defaultFuncValue(assets, weapon_cfg_id),
        .role_id = trial_id,
    });

    var created_echo_ids: std.ArrayListUnmanaged(i32) = .empty;
    defer created_echo_ids.deinit(gpa);
    errdefer {
        for (created_echo_ids.items) |eid| {
            if (echo_comp.echo_map.fetchSwapRemove(eid)) |removed| {
                var e = removed.value;
                e.deinit(gpa);
            }
        }
        if (weapon_comp.weapon_map.fetchSwapRemove(weapon_incr_id)) |wrem| {
            var w = wrem.value;
            w.deinit(gpa);
        }
    }

    const echo_count = @min(trial.PhantomEquipList.len, 5);
    for (trial.PhantomEquipList[0..echo_count], 0..) |entry, slot_idx| {
        if (entry.Item1 == 0) continue;
        const echo_incr_id = nextTrialIncrId(&echo_comp.echo_map);
        const tpp_opt = assets.tables.trail_phantom_prop.getDataById(entry.Item2);

        var main_props: std.ArrayListUnmanaged(EchoInfo.Prop) = .empty;
        errdefer main_props.deinit(gpa);
        var sub_props: std.ArrayListUnmanaged(EchoInfo.Prop) = .empty;
        errdefer sub_props.deinit(gpa);

        var level: i32 = 0;
        var fetter_group_id: i32 = 0;
        var skin_id: i32 = 0;

        if (tpp_opt) |tpp| {
            level = @max(0, tpp.Level);
            fetter_group_id = tpp.FetterGroupId;
            skin_id = tpp.SkinItemId;

            for (tpp.MainProps) |mp_id| {
                if (assets.tables.phantom_main_prop_item.getDataById(mp_id)) |pmp| {
                    try main_props.append(gpa, .{
                        .id = pmp.Id,
                        .value = EchoInfo.mainPropValue(assets, pmp, level),
                    });
                } else if (assets.tables.trial_phantom_prop_item.getDataById(mp_id)) |tppi| {
                    try main_props.append(gpa, .{
                        .id = tppi.Id,
                        .value = tppi.Prop.Value,
                    });
                }
            }
            if (main_props.items.len == 0 and tpp.MainProp.Id != 0) {
                try main_props.append(gpa, .{
                    .id = tpp.MainProp.Id,
                    .value = tpp.MainProp.Value,
                });
            }

            for (tpp.SubPropList) |sp_id| {
                if (assets.tables.trial_phantom_prop_item.getDataById(sp_id)) |tppi| {
                    try sub_props.append(gpa, .{
                        .id = tppi.Id,
                        .value = tppi.Prop.Value,
                    });
                }
            }
        }

        try echo_comp.echo_map.put(gpa, echo_incr_id, .{
            .id = entry.Item1,
            .level = level,
            .fetter_group_id = fetter_group_id,
            .skin_id = skin_id,
            .role_id = trial_id,
            .equipped_pos = @intCast(slot_idx),
            .main_prop = try main_props.toOwnedSlice(gpa),
            .sub_prop = try sub_props.toOwnedSlice(gpa),
        });
        try created_echo_ids.append(gpa, echo_incr_id);
    }

    const level = @max(1, trial.Level);
    const breach = breachForRoleLevel(assets, parent_cfg.BreachId, level);

    var nodes: std.ArrayListUnmanaged(RoleInfo.SkillNode) = .empty;
    errdefer nodes.deinit(gpa);
    var skills: std.ArrayListUnmanaged(struct { i32, i32 }) = .empty;
    errdefer skills.deinit(gpa);

    const skill_lvl = @max(1, trial.UnlockSkillLevel);
    for (assets.tables.skill_tree.items) |node| {
        if (node.NodeGroup != parent_cfg.SkillTreeGroupId) continue;
        const default_active = node.Condition.len == 0 and node.UnLockCondition == 0;
        const active = default_active or containsI32(trial.UnlockSkillNodeList, node.NodeIndex);
        try nodes.append(gpa, .{
            .node_id = node.Id,
            .active = active,
            .skill_id = node.SkillId,
        });
        if (active and node.SkillId != 0) {
            try skills.append(gpa, .{ node.SkillId, skill_lvl });
        }
    }

    var reson: std.ArrayListUnmanaged(RoleInfo.Reson) = .empty;
    errdefer reson.deinit(gpa);
    for (assets.tables.resonant_chain.items) |chain| {
        if (chain.GroupId != parent_cfg.ResonantChainGroupId) continue;
        try reson.append(gpa, .{
            .id = chain.Id,
            .open = chain.GroupIndex <= trial.ResonanceLevel,
            .increase = 0,
        });
    }

    var base_prop_buf: [@intFromEnum(pb.EAttributeType.MAX)]i32 = undefined;
    if (!RoleInfo.computeScaledBaseProps(assets, parent_cfg.Id, level, breach, &base_prop_buf)) {
        @memset(&base_prop_buf, 0);
    }

    const role_info: RoleInfo = .{
        .level = level,
        .breakthrough = breach,
        .star = @max(parent_cfg.QualityId, 4),
        .weapon = weapon_incr_id,
        .role_skin_id = if (trial.RoleSkin != 0) trial.RoleSkin else parent_cfg.SkinId,
        .weapon_skin_id = weapon_skin,
        .skill_branch = if (trial.DefaultSkillBranchId != 0) trial.DefaultSkillBranchId else parent_cfg.DefaultSkillBranchId,
        .resonant_chain_group_index = trial.ResonanceLevel,
        .base_prop = try gpa.dupe(i32, &base_prop_buf),
        .skills = try skills.toOwnedSlice(gpa),
        .skill_node_state = try nodes.toOwnedSlice(gpa),
        .reson = try reson.toOwnedSlice(gpa),
    };

    try role_comp.role_map.put(gpa, trial_id, role_info);
    try role_comp.trial_roles.put(gpa, trial_id, {});
    return true;
}

pub fn materializeActiveTrialRoles(
    gpa: Allocator,
    assets: *const Assets,
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    trial_ids: []const i32,
) !void {
    for (trial_ids) |rid| {
        _ = try materializeTrialRole(gpa, assets, role_comp, weapon_comp, echo_comp, rid);
    }
}

pub fn sanitizeFormationForLogin(
    formation_info: *FormationInfo,
    role_comp: *const PlayerRoleComponent,
    basic_comp: *const PlayerBasicComponent,
    active_trial_ids: []const i32,
) void {
    const cur_idx: usize = @intCast(@max(0, @min(formation_info.cur_formation, @as(i32, @intCast(formation_info.formations.len - 1)))));
    for (&formation_info.formations) |*form| {
        var any_valid = false;
        var has_cur = false;
        for (&form.roles) |*maybe_slot| {
            if (maybe_slot.*) |slot| {
                const is_trial_allowed = containsI32(active_trial_ids, slot.role_id);
                const valid_owned = role_comp.role_map.contains(slot.role_id) and (!role_comp.isTrialRole(slot.role_id) or is_trial_allowed);
                if (!valid_owned) {
                    maybe_slot.* = null;
                    continue;
                }
                any_valid = true;
                if (slot.role_id == form.cur_role) has_cur = true;
            }
        }
        if (any_valid and !has_cur) {
            for (form.roles) |maybe_slot| {
                if (maybe_slot) |slot| {
                    form.cur_role = slot.role_id;
                    break;
                }
            }
        }
    }

    const cur_form = &formation_info.formations[cur_idx];
    const cur_has_any = for (cur_form.roles) |maybe_slot| {
        if (maybe_slot != null) break true;
    } else false;
    if (!cur_has_any) {
        const fallback_id: i32 = if (basic_comp.info.selected_main_role_id != 0 and role_comp.role_map.contains(basic_comp.info.selected_main_role_id))
            basic_comp.info.selected_main_role_id
        else blk: {
            var it = role_comp.role_map.iterator();
            while (it.next()) |kv| {
                if (!role_comp.isTrialRole(kv.key_ptr.*)) break :blk kv.key_ptr.*;
            }
            break :blk 1502;
        };
        cur_form.roles[0] = .{ .role_id = fallback_id };
        cur_form.cur_role = fallback_id;
    }
}

pub fn pushTrialRoleList(
    conn: *Connection,
    alloc: mem.Alloc,
    assets: *const Assets,
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    role_ids: []const i32,
    io: std.Io,
) !void {
    var list: std.ArrayList(pb.RoleInfo) = .empty;
    for (role_ids) |rid| {
        const role = role_comp.role_map.getPtr(rid) orelse continue;
        try list.append(alloc.arena, try RoleStats.toClientRoleInfo(
            alloc.gpa,
            alloc.arena,
            assets,
            role_comp,
            rid,
            role,
            weapon_comp,
            echo_comp,
        ));
    }
    if (list.items.len != 0) {
        try conn.push(alloc.gpa, io, pb.PbGetRoleListNotify{ .RoleList = list });
    }
}

pub fn savePreTrialFormation(quest_comp: *PlayerQuestComponent, gpa: Allocator, scene: *const Scene) !void {
    if (quest_comp.info.env.pre_trial_roles.len != 0) return;
    if (scene.formation_info.cur_formation < 0) return;
    const cur_idx: usize = @intCast(scene.formation_info.cur_formation);
    if (cur_idx >= scene.formation_info.formations.len) return;
    const cur_form = scene.formation_info.formations[cur_idx];

    var saved: std.ArrayListUnmanaged(i32) = .empty;
    errdefer saved.deinit(gpa);
    for (cur_form.roles) |maybe_slot| {
        if (maybe_slot) |slot| try saved.append(gpa, slot.role_id);
    }
    if (saved.items.len != 0) {
        quest_comp.info.env.pre_trial_roles = try saved.toOwnedSlice(gpa);
        quest_comp.info.env.pre_trial_cur_role = cur_form.cur_role;
    }
}

pub fn clearPreTrialFormation(quest_comp: *PlayerQuestComponent, gpa: Allocator) void {
    if (quest_comp.info.env.pre_trial_roles.len != 0) {
        gpa.free(quest_comp.info.env.pre_trial_roles);
        quest_comp.info.env.pre_trial_roles = &.{};
    }
    quest_comp.info.env.pre_trial_cur_role = 0;
}

pub fn cleanupRemovedTrialRoles(
    gpa: Allocator,
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    removed_ids: []const i32,
) void {
    for (removed_ids) |rid| {
        cleanupTrialDataForRole(gpa, role_comp, weapon_comp, echo_comp, rid);
    }
}

pub fn applyTrialRolesToSceneFormation(
    scene: *Scene,
    fs: *FileSystem,
    assets: *const Assets,
    conn: *Connection,
    events: *EventQueue,
    alloc: mem.Alloc,
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    trial_ids: []const i32,
    create_temp_team: bool,
    auto_change: bool,
    io: std.Io,
) !void {
    if (trial_ids.len == 0 or scene.formation_info.cur_formation < 0) return;
    const cur_idx: usize = @intCast(scene.formation_info.cur_formation);
    if (cur_idx >= scene.formation_info.formations.len) return;
    const cur_form = &scene.formation_info.formations[cur_idx];

    var target_ids: std.ArrayList(i32) = .empty;
    for (trial_ids) |rid| {
        if (target_ids.items.len >= 3) break;
        if (!role_comp.role_map.contains(rid)) continue;
        if (containsI32(target_ids.items, rid)) continue;
        try target_ids.append(alloc.arena, rid);
    }
    if (!create_temp_team) {
        for (cur_form.roles) |maybe_slot| {
            if (target_ids.items.len >= 3) break;
            if (maybe_slot) |slot| {
                if (!role_comp.role_map.contains(slot.role_id)) continue;
                if (containsI32(target_ids.items, slot.role_id)) continue;
                if (conflictsWithTargetRoles(assets, target_ids.items, slot.role_id)) continue;
                try target_ids.append(alloc.arena, slot.role_id);
            }
        }
    }
    if (target_ids.items.len == 0) return;

    const desired_cur = if (auto_change or !containsI32(target_ids.items, cur_form.cur_role))
        target_ids.items[0]
    else
        cur_form.cur_role;

    try syncFormationEntities(scene, fs, assets, conn, events, alloc, role_comp, weapon_comp, echo_comp, target_ids.items, desired_cur, io);
}

fn canonicalRoleFamily(assets: *const Assets, role_id: i32) i32 {
    const base_id: i32 = if (assets.tables.trial_role_info.getDataById(role_id)) |trial|
        if (trial.ParentId > 0) trial.ParentId else role_id
    else
        role_id;
    return switch (base_id) {
        // All Rover elements (Spectro/Havoc/Aero, Male/Female) share one character slot.
        1406, 1408, 1501, 1502, 1604, 1605 => 1501,
        else => base_id,
    };
}

fn conflictsWithTargetRoles(assets: *const Assets, targets: []const i32, candidate: i32) bool {
    const cand_fam = canonicalRoleFamily(assets, candidate);
    for (targets) |tid| {
        if (canonicalRoleFamily(assets, tid) == cand_fam) return true;
    }
    return false;
}

pub fn restoreFormationAfterTrialRemoval(
    scene: *Scene,
    fs: *FileSystem,
    assets: *const Assets,
    conn: *Connection,
    events: *EventQueue,
    alloc: mem.Alloc,
    basic_comp: *const PlayerBasicComponent,
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    quest_comp: *PlayerQuestComponent,
    removed_ids: []const i32,
    remaining_trial_ids: []const i32,
    io: std.Io,
) !void {
    if (scene.formation_info.cur_formation < 0) return;
    const cur_idx: usize = @intCast(scene.formation_info.cur_formation);
    if (cur_idx >= scene.formation_info.formations.len) return;
    const cur_form = &scene.formation_info.formations[cur_idx];

    var next_ids: std.ArrayList(i32) = .empty;
    for (cur_form.roles) |maybe_slot| {
        if (maybe_slot) |slot| {
            if (containsI32(removed_ids, slot.role_id)) continue;
            if (role_comp.isTrialRole(slot.role_id) and !containsI32(remaining_trial_ids, slot.role_id)) continue;
            if (!role_comp.role_map.contains(slot.role_id)) continue;
            if (!containsI32(next_ids.items, slot.role_id)) try next_ids.append(alloc.arena, slot.role_id);
        }
    }

    if (remaining_trial_ids.len == 0) {
        for (quest_comp.info.env.pre_trial_roles) |rid| {
            if (next_ids.items.len >= 3) break;
            if (!role_comp.role_map.contains(rid) or role_comp.isTrialRole(rid)) continue;
            if (!containsI32(next_ids.items, rid)) try next_ids.append(alloc.arena, rid);
        }
    }

    if (next_ids.items.len == 0) {
        if (basic_comp.info.selected_main_role_id != 0 and role_comp.role_map.contains(basic_comp.info.selected_main_role_id)) {
            try next_ids.append(alloc.arena, basic_comp.info.selected_main_role_id);
        } else {
            var it = role_comp.role_map.iterator();
            while (it.next()) |kv| {
                if (!role_comp.isTrialRole(kv.key_ptr.*)) {
                    try next_ids.append(alloc.arena, kv.key_ptr.*);
                    break;
                }
            }
        }
    }
    if (next_ids.items.len == 0) return;

    const desired_cur = if (remaining_trial_ids.len == 0 and containsI32(next_ids.items, quest_comp.info.env.pre_trial_cur_role))
        quest_comp.info.env.pre_trial_cur_role
    else if (containsI32(next_ids.items, cur_form.cur_role))
        cur_form.cur_role
    else
        next_ids.items[0];

    try syncFormationEntities(scene, fs, assets, conn, events, alloc, role_comp, weapon_comp, echo_comp, next_ids.items, desired_cur, io);
    if (remaining_trial_ids.len == 0) clearPreTrialFormation(quest_comp, alloc.gpa);
}

fn syncFormationEntities(
    scene: *Scene,
    fs: *FileSystem,
    assets: *const Assets,
    conn: *Connection,
    events: *EventQueue,
    alloc: mem.Alloc,
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    target_role_ids: []const i32,
    desired_cur_role: i32,
    io: std.Io,
) !void {
    const cur_idx: usize = @intCast(scene.formation_info.cur_formation);
    const cur_form = &scene.formation_info.formations[cur_idx];
    const old_roles = cur_form.roles;

    const already_synced = blk: {
        if (cur_form.cur_role != desired_cur_role) break :blk false;
        var old_count: usize = 0;
        for (old_roles) |maybe_old| {
            if (maybe_old != null) old_count += 1;
        }
        const expected_count = @min(target_role_ids.len, 3);
        if (old_count != expected_count) break :blk false;
        for (target_role_ids[0..expected_count], 0..) |rid, i| {
            const old = old_roles[i] orelse break :blk false;
            if (old.role_id != rid) break :blk false;
            if (old.entity_id == -1 or !scene.net_id_map.contains(old.entity_id)) break :blk false;
        }
        break :blk true;
    };
    if (already_synced) return;

    var remove_infos: std.ArrayList(pb.EntityRemoveInfo) = .empty;
    for (old_roles) |maybe_old| {
        const old = maybe_old orelse continue;
        if (containsI32(target_role_ids, old.role_id)) continue;
        if (old.entity_id != -1) {
            if (scene.net_id_map.get(old.entity_id)) |entity_index| {
                const storage = scene.entities.get(entity_index);
                if (storage.concomitant) |concomitant| {
                    for (concomitant.vision_entity_id) |cid| {
                        try scene.remove(alloc.gpa, fs, cid);
                        try remove_infos.append(alloc.arena, .{ .EntityId = cid });
                    }
                    for (concomitant.custom_entity_ids) |cid| {
                        try scene.remove(alloc.gpa, fs, cid);
                        try remove_infos.append(alloc.arena, .{ .EntityId = cid });
                    }
                }
                try scene.remove(alloc.gpa, fs, old.entity_id);
                try remove_infos.append(alloc.arena, .{ .EntityId = old.entity_id });
            }
        }
    }
    if (remove_infos.items.len != 0) {
        try conn.push(alloc.gpa, io, pb.EntityRemoveNotify{
            .IsRemove = true,
            .RemoveInfos = remove_infos,
        });
    }

    var new_roles: [3]?FormationInfo.Formation.Role = .{ null, null, null };
    for (target_role_ids[0..@min(target_role_ids.len, 3)], 0..) |rid, idx| {
        const kept_eid: i64 = blk: {
            for (old_roles) |maybe_old| {
                if (maybe_old) |old| if (old.role_id == rid) break :blk old.entity_id;
            }
            break :blk -1;
        };
        new_roles[idx] = .{
            .role_id = rid,
            .entity_id = kept_eid,
            .on_stage_without_control = false,
        };
    }
    cur_form.roles = new_roles;
    cur_form.cur_role = desired_cur_role;

    const instance_dungeon = assets.tables.instance_dungeon.getDataById(scene.instance_id) orelse return;
    const now_ms = (std.Io.Clock.awake).now(io).toMilliseconds();

    var new_role_net_ids: std.ArrayList(i64) = .empty;
    for (&cur_form.roles) |*maybe_slot| {
        const slot = &(maybe_slot.* orelse continue);
        if (slot.entity_id == -1 or !scene.net_id_map.contains(slot.entity_id)) {
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
                slot.role_id,
            );
            slot.entity_id = entity.net_id;
            try new_role_net_ids.append(alloc.arena, entity.net_id);
        }
    }

    // Update visibility matching cur_role before serializing EntityPb payloads
    const slice = scene.entities.slice();
    for (slice.items(.config), 0..) |config, i| {
        if (config.entity_type != .player or config.config_type != .character) continue;
        if (config.config_id == desired_cur_role) {
            slice.items(.visible)[i] = .{};
        } else {
            slice.items(.visible)[i] = null;
        }
    }

    var add_role_pbs: std.ArrayList(pb.EntityPb) = .empty;
    var add_concom_pbs: std.ArrayList(pb.EntityPb) = .empty;
    for (new_role_net_ids.items) |net_id| {
        const idx = scene.net_id_map.get(net_id) orelse continue;
        try add_role_pbs.append(
            alloc.arena,
            try entity_proto.buildOptional(alloc, assets, scene, null, net_id, now_ms),
        );
        const storage = scene.entities.get(idx);
        if (storage.concomitant) |concomitant| {
            for (concomitant.vision_entity_id) |cid| {
                if (!scene.net_id_map.contains(cid)) continue;
                try add_concom_pbs.append(
                    alloc.arena,
                    try entity_proto.buildOptional(alloc, assets, scene, null, cid, now_ms),
                );
            }
            for (concomitant.custom_entity_ids) |cid| {
                if (!scene.net_id_map.contains(cid)) continue;
                try add_concom_pbs.append(
                    alloc.arena,
                    try entity_proto.buildOptional(alloc, assets, scene, null, cid, now_ms),
                );
            }
        }
    }

    if (add_role_pbs.items.len != 0) {
        try conn.push(alloc.gpa, io, pb.EntityAddNotify{
            .EntityPbs = add_role_pbs,
            .RemoveTagIds = false,
        });
    }
    if (add_concom_pbs.items.len != 0) {
        try conn.push(alloc.gpa, io, pb.EntityAddNotify{
            .EntityPbs = add_concom_pbs,
            .RemoveTagIds = false,
        });
    }

    var fight_role_infos: std.ArrayList(pb.FightRoleInfo) = .empty;
    for (cur_form.roles) |maybe_slot| {
        const slot = maybe_slot orelse continue;
        try fight_role_infos.append(alloc.arena, .{
            .RoleId = slot.role_id,
            .EntityId = slot.entity_id,
            .OnStageWithoutControl = false,
        });
    }
    var fight_role_infos_wrapper: std.ArrayList(pb.FightRoleInfos) = .empty;
    try fight_role_infos_wrapper.append(alloc.arena, .{
        .GroupType = 1,
        .FightRoleInfos = fight_role_infos,
        .CurRole = cur_form.cur_role,
        .LivingStatus = RoleHelper.formationLivingStatus(scene, cur_form.*),
        .IsFixedLocation = false,
    });
    var group_formations: std.ArrayList(pb.GroupFormation) = .empty;
    try group_formations.append(alloc.arena, .{
        .PlayerId = scene.player_id,
        .FightRoleInfos = fight_role_infos_wrapper,
        .CurrentGroupType = 1,
    });
    try conn.push(alloc.gpa, io, pb.UpdateGroupFormationNotify{
        .GroupFormation = group_formations,
    });

    try scene.save(fs, alloc.gpa);
    try events.enqueue(.update_formations, .{});
}
