const std = @import("std");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const dispatch = @import("combat.zig");
const Assets = @import("../../data/Assets.zig");
const Scene = @import("../../logic/Scene.zig");
const FileSystem = @import("common").FileSystem;
const Transaction = @import("../handlers.zig").Transaction;
const PlayerRoleComponent = @import("../../logic/component/player/PlayerRoleComponent.zig");
const PlayerBasicComponent = @import("../../logic/component/player/PlayerBasicComponent.zig");
const PlayerWeaponComponent = @import("../../logic/component/player/PlayerWeaponComponent.zig");
const PlayerEchoComponent = @import("../../logic/component/player/PlayerEchoComponent.zig");
const PlayerInventoryComponent = @import("../../logic/component/player/PlayerInventoryComponent.zig");
const InventoryInfo = @import("../../fs/InventoryInfo.zig");
const RoleInfo = @import("../../fs/RoleInfo.zig");
const Account = @import("../../fs/Account.zig");
const comp_util = @import("../../logic/component/comp_util.zig");
const EventQueue = @import("../../logic/EventQueue.zig");
const Events = @import("../../logic/events.zig");
const RoleAttributeSync = @import("../helpers/role_attribute_sync.zig");
const RoleStats = @import("../../logic/helpers/role_stats.zig");
const RoleEquipment = @import("../../logic/helpers/role_equipment.zig");
const RoleHelper = @import("../../logic/helpers/role.zig");
const ConsumeHelper = @import("../../logic/helpers/consume.zig");
const ConditionEvaluator = @import("../../logic/helpers/condition.zig");
const RewardService = @import("../../logic/helpers/reward.zig");
const quest_types = @import("../../logic/helpers/quest_types.zig");
const EchoShared = @import("echo/shared.zig");
const BuffTimerScheduler = @import("../../logic/schedulers/BuffTimerScheduler.zig");
const QuestEngine = @import("../../logic/helpers/quest_engine.zig");

fn saveRoleImmediate(
    fs: *FileSystem,
    arena: std.mem.Allocator,
    player_id: i32,
    role_id: i32,
    role: RoleInfo,
) !void {
    const path = try std.fmt.allocPrint(arena, "player/{}/{s}/{}", .{ player_id, RoleInfo.data_dir, role_id });
    try comp_util.saveStruct(fs, role, path, arena);
}

fn findSkillLevelConfig(
    assets: *const Assets,
    skill_level_group_id: i32,
    target_level: i32,
) ?Assets.DataTables.SkillLevel {
    for (assets.tables.skill_level.items) |entry| {
        if (entry.SkillLevelGroupId == skill_level_group_id and entry.SkillId == target_level) {
            return entry;
        }
    }
    return null;
}

fn isSkillNodeActive(role: *const RoleInfo, node_id: i32) bool {
    for (role.skill_node_state) |entry| {
        if (entry.node_id == node_id) return entry.active;
    }
    return false;
}

fn findRoleBreachEntry(
    assets: *const Assets,
    breach_group_id: i32,
    target_breach: i32,
) ?Assets.DataTables.RoleBreach {
    for (assets.tables.role_breach.items) |entry| {
        if (entry.BreachGroupId == breach_group_id and entry.BreachLevel == target_breach) {
            return entry;
        }
    }
    return null;
}

fn isValidSkillBranchForRole(
    assets: *const Assets,
    role_config: Assets.DataTables.RoleInfo,
    role_id: i32,
    branch_id: i32,
) bool {
    if (branch_id == 0) return role_config.DefaultSkillBranchId == 0;
    for (role_config.SkillBranchIds) |allowed| {
        if (allowed == branch_id) return true;
    }
    if (assets.tables.skill_branch_buff.getDataById(branch_id)) |branch_data| {
        return branch_data.RoleId == role_id;
    }
    return false;
}

fn buildPreviewFinalProps(
    gpa: std.mem.Allocator,
    arena: std.mem.Allocator,
    assets: *const Assets,
    role_id: i32,
    role: *const RoleInfo,
    preview_level: i32,
    preview_breakthrough: i32,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
) !std.ArrayList(pb.ArrayIntDouble) {
    var tmp_base_prop: [@intFromEnum(pb.EAttributeType.MAX)]i32 = undefined;
    var tmp_role = role.*;
    tmp_role.level = preview_level;
    tmp_role.breakthrough = preview_breakthrough;
    if (RoleInfo.computeScaledBaseProps(assets, role_id, preview_level, preview_breakthrough, &tmp_base_prop)) {
        tmp_role.base_prop = &tmp_base_prop;
    }

    var final_prop: std.ArrayList(pb.ArrayIntDouble) = .empty;
    const snapshot = try RoleStats.buildPreviewSnapshot(gpa, assets, role_id, &tmp_role, weapon_comp, echo_comp);
    defer snapshot.deinit(gpa);
    for (snapshot.base_props, 0..) |base_val, i| {
        const add_val = if (i < snapshot.add_props.len) snapshot.add_props[i] else 0;
        try final_prop.append(arena, .{
            .Key = @intCast(i),
            .Value = @floatFromInt(base_val + add_val),
        });
    }
    return final_prop;
}

fn pushWeaponEquipNotify(txn: anytype, alloc: mem.Alloc, role_id: i32, weapon_id: i32, io: std.Io) !void {
    if (weapon_id == 0) return;

    var data_list: std.ArrayList(pb.RoleLoadEquipData) = .empty;
    defer data_list.deinit(alloc.arena);
    try data_list.append(alloc.arena, .{
        .RoleId = role_id,
        .Pos = .Weapon,
        .EquipIncId = weapon_id,
    });

    try txn.conn.push(alloc.gpa, io, pb.EquipTakeOnNotify{ .DataList = data_list });
}

fn pushPhantomEquipNotify(
    txn: anytype,
    alloc: mem.Alloc,
    assets: *const Assets,
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    role_id: i32,
    io: std.Io,
) !void {
    var changed_roles: std.array_hash_map.Auto(i32, void) = .empty;
    defer changed_roles.deinit(alloc.gpa);
    try changed_roles.put(alloc.gpa, role_id, {});

    try EchoShared.pushRolePropUpdate(txn, alloc, assets, role_comp, echo_comp, weapon_comp, changed_roles, io);
    try txn.conn.push(alloc.gpa, io, pb.PhantomPutOnNotify{
        .EquipInfoList = try EchoShared.changedEquipInfoList(echo_comp, changed_roles, alloc.arena),
    });
}

pub fn onRoleFavorListRequest(
    txn: *Transaction(pb.RoleFavorListRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
    role_comp: *PlayerRoleComponent,
) !void {
    var favor_list: std.ArrayList(pb.RoleFavor) = .empty;

    var iterator = role_comp.role_map.iterator();
    while (iterator.next()) |role_entry| {
        const role_id = role_entry.key_ptr.*;

        var role_favor: pb.RoleFavor = .{
            .RoleId = role_id,
            .Level = 5,
            .Exp = 16800,
        };

        for (assets.tables.favor_word.items) |word| {
            if (word.RoleId == role_id) {
                try role_favor.WordIds.append(alloc.arena, .{
                    .Id = word.Id,
                    .Status = .ItemUnLocked,
                });
            }
        }

        for (assets.tables.favor_story.items) |story| {
            if (story.RoleId == role_id) {
                try role_favor.StoryIds.append(alloc.arena, .{
                    .Id = story.Id,
                    .Status = .ItemUnLocked,
                });
            }
        }

        for (assets.tables.favor_goods.items) |goods| {
            if (goods.RoleId == role_id) {
                try role_favor.GoodsIds.append(alloc.arena, .{
                    .Id = goods.Id,
                    .Status = .ItemUnLocked,
                });
            }
        }

        try favor_list.append(alloc.arena, role_favor);
    }

    txn.respond(.{ .FavorList = favor_list });
}

pub fn onRoleSexChangeRequest(
    txn: *Transaction(pb.RoleSexChangeRequest),
    events: *EventQueue,
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    scene: *Scene,
    basic_comp: *PlayerBasicComponent,
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    buff_timers: *BuffTimerScheduler,
    io: std.Io,
) !void {
    const now_ms = (std.Io.Clock.awake).now(io).toMilliseconds();
    const target_gender = txn.message.Sex;
    const current_gender = basic_comp.info.attributes.sex;

    if (target_gender == current_gender) {
        txn.respond(.{
            .ErrorCode = .Success,
            .Sex = current_gender,
        });
        return;
    }

    const source_role_id = RoleHelper.currentFormationMainRoleId(assets, scene, current_gender) orelse
        RoleHelper.selectedMainRoleId(assets, role_comp, current_gender, basic_comp.info.selected_main_role_id) orelse {
        txn.respond(.{
            .ErrorCode = .ErrRoleSexFuncNotOpen,
            .Sex = current_gender,
        });
        return;
    };
    const source_role_config = assets.tables.role_info.getDataById(source_role_id) orelse {
        txn.respond(.{
            .ErrorCode = .RequestParamError,
            .Sex = current_gender,
        });
        return;
    };
    const target_role_id = RoleHelper.mainRoleIdForElement(assets, target_gender, source_role_config.ElementId) orelse {
        txn.respond(.{
            .ErrorCode = .ErrRoleSexFuncNotOpen,
            .Sex = current_gender,
        });
        return;
    };

    const transfer = try RoleHelper.transferMainRoleState(
        alloc,
        fs,
        assets,
        scene,
        basic_comp,
        role_comp,
        weapon_comp,
        echo_comp,
        source_role_id,
        target_role_id,
    );
    try events.enqueue(.role_info_modified, .{ .role_id = target_role_id });

    basic_comp.info.attributes.sex = target_gender;
    try RoleHelper.saveBasicInfo(fs, alloc.arena, basic_comp);
    try Account.syncLoginSexForPlayer(alloc.arena, fs, basic_comp.player_id, target_gender);

    if (transfer.formation_changed) {
        try RoleHelper.resetRoles(
            scene,
            fs,
            assets,
            role_comp,
            weapon_comp,
            echo_comp,
            txn.conn,
            alloc,
            &.{target_role_id},
            buff_timers,
            now_ms,
            io,
        );
        try buff_timers.ensureAllRegistered(alloc.gpa, scene, assets, now_ms);
        buff_timers.syncAllLeftDurations(scene, now_ms);
        try scene.save(fs, alloc.gpa);
        try events.enqueue(.update_formations, .{});
    }

    const target_role = role_comp.role_map.getPtr(target_role_id) orelse {
        txn.respond(.{
            .ErrorCode = .RequestParamError,
            .Sex = current_gender,
        });
        return;
    };

    try txn.conn.push(alloc.gpa, io, pb.RoleChangeNotify{
        .SourceRoleId = source_role_id,
        .RoleInfo = try RoleStats.toClientRoleInfo(
            alloc.gpa,
            alloc.arena,
            assets,
            role_comp,
            target_role_id,
            target_role,
            weapon_comp,
            echo_comp,
        ),
    });
    try txn.conn.push(alloc.gpa, io, pb.RoleChangeUnlockNotify{
        .UnlockRoleIds = try RoleHelper.mainRoleUnlockList(assets, target_gender, alloc.arena),
    });

    txn.respond(.{
        .ErrorCode = .Success,
        .Sex = target_gender,
    });
    try txn.conn.push(alloc.gpa, io, pb.LogoutNotify{
        .ErrorCode = .ErrSexChangeLogout,
    });
}

pub fn onRoleElementChangeRequest(
    txn: *Transaction(pb.RoleElementChangeRequest),
    events: *EventQueue,
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    scene: *Scene,
    basic_comp: *PlayerBasicComponent,
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    buff_timers: *BuffTimerScheduler,
    io: std.Io,
) !void {
    const now_ms = (std.Io.Clock.awake).now(io).toMilliseconds();
    const gender = basic_comp.info.attributes.sex;
    const target_role_id = RoleHelper.mainRoleIdForElement(assets, gender, txn.message.ElementType) orelse {
        txn.respond(.{ .ErrorCode = .ErrRoleChangeElementFunc });
        return;
    };
    const source_role_id = RoleHelper.currentFormationMainRoleId(assets, scene, gender) orelse
        RoleHelper.selectedMainRoleId(assets, role_comp, gender, basic_comp.info.selected_main_role_id) orelse target_role_id;

    const transfer = try RoleHelper.transferMainRoleState(
        alloc,
        fs,
        assets,
        scene,
        basic_comp,
        role_comp,
        weapon_comp,
        echo_comp,
        source_role_id,
        target_role_id,
    );
    if (transfer.basic_changed) try RoleHelper.saveBasicInfo(fs, alloc.arena, basic_comp);
    try events.enqueue(.role_info_modified, .{ .role_id = target_role_id });

    if (transfer.formation_changed) {
        try RoleHelper.resetRoles(
            scene,
            fs,
            assets,
            role_comp,
            weapon_comp,
            echo_comp,
            txn.conn,
            alloc,
            &.{target_role_id},
            buff_timers,
            now_ms,
            io,
        );
        try buff_timers.ensureAllRegistered(alloc.gpa, scene, assets, now_ms);
        buff_timers.syncAllLeftDurations(scene, now_ms);
        try scene.save(fs, alloc.gpa);
        try events.enqueue(.update_formations, .{});
    }

    const target_role = role_comp.role_map.getPtr(target_role_id) orelse {
        txn.respond(.{ .ErrorCode = .RequestParamError });
        return;
    };

    try pushWeaponEquipNotify(txn, alloc, target_role_id, target_role.weapon, io);
    try txn.conn.push(alloc.gpa, io, pb.RoleChangeNotify{
        .SourceRoleId = source_role_id,
        .RoleInfo = try RoleStats.toClientRoleInfo(
            alloc.gpa,
            alloc.arena,
            assets,
            role_comp,
            target_role_id,
            target_role,
            weapon_comp,
            echo_comp,
        ),
    });
    try pushPhantomEquipNotify(txn, alloc, assets, role_comp, weapon_comp, echo_comp, target_role_id, io);

    txn.respond(.{ .ErrorCode = .Success });
}

pub fn SwitchRoleRequest(
    txn: *dispatch.CombatRequestTxn(.SwitchRoleRequest),
    scene: *Scene,
    fs: *FileSystem,
    alloc: mem.Alloc,
    assets: *const Assets,
    buff_timers: *BuffTimerScheduler,
    io: std.Io,
) !void {
    const now_ms = (std.Io.Clock.awake).now(io).toMilliseconds();
    const request: pb.SwitchRoleRequest = txn.payload;
    if (!roleInCurrentFormation(scene, request.RoleId)) {
        txn.respond(.{ .ErrorCode = .SwitchRoleNotInCurrentFormation, .RoleId = request.RoleId });
        return;
    }
    if (!RoleHelper.isRoleEntityAlive(scene, request.RoleId)) {
        txn.respond(.{ .ErrorCode = .ErrSwitchRoleIsDead, .RoleId = request.RoleId });
        return;
    }
    _ = applyActiveRoleSwitch(scene, request.RoleId);

    try buff_timers.ensureAllRegistered(alloc.gpa, scene, assets, now_ms);
    buff_timers.syncAllLeftDurations(scene, now_ms);
    try scene.save(fs, alloc.gpa);
    txn.respond(.{ .ErrorCode = .Success, .RoleId = request.RoleId });
}

pub fn ClientCurrentRoleReportPush(
    push: pb.ClientCurrentRoleReportPush,
    scene: *Scene,
    fs: *FileSystem,
    alloc: mem.Alloc,
) !void {
    if (push.CurrentRoleId <= 0) return;
    if (applyActiveRoleSwitch(scene, push.CurrentRoleId)) {
        try scene.save(fs, alloc.gpa);
    }
}

fn roleInCurrentFormation(scene: *const Scene, role_id: i32) bool {
    if (role_id <= 0 or scene.formation_info.cur_formation < 0) return false;
    const form_idx: usize = @intCast(scene.formation_info.cur_formation);
    if (form_idx >= scene.formation_info.formations.len) return false;
    for (scene.formation_info.formations[form_idx].roles) |maybe_role| {
        const role = maybe_role orelse continue;
        if (role.role_id == role_id) return true;
    }
    return false;
}

fn applyActiveRoleSwitch(scene: *Scene, role_id: i32) bool {
    if (!roleInCurrentFormation(scene, role_id)) return false;
    const form_idx: usize = @intCast(scene.formation_info.cur_formation);

    const formation = &scene.formation_info.formations[form_idx];
    const previous_role = formation.cur_role;
    if (previous_role == role_id) return false;

    formation.cur_role = role_id;
    const slice = scene.entities.slice();
    for (slice.items(.config), 0..) |config, i| {
        if (config.config_id == previous_role) {
            slice.items(.visible)[i] = null;
        } else if (config.config_id == role_id) {
            slice.items(.visible)[i] = .{};
        }
    }
    return true;
}

fn setSkillLevel(role: *RoleInfo, gpa: std.mem.Allocator, skill_id: i32, level: i32) !void {
    for (role.skills) |*entry| {
        if (entry[0] == skill_id) {
            entry[1] = level;
            return;
        }
    }

    const new_skills = try gpa.alloc(@typeInfo(@TypeOf(role.skills)).pointer.child, role.skills.len + 1);
    @memcpy(new_skills[0..role.skills.len], role.skills);
    new_skills[role.skills.len] = .{ skill_id, level };

    if (role.skills.len != 0) gpa.free(role.skills);
    role.skills = new_skills;
}

fn getSkillLevel(role: *const RoleInfo, skill_id: i32) i32 {
    for (role.skills) |entry| {
        if (entry[0] == skill_id) return entry[1];
    }
    return 0;
}

fn setSkillNodeState(role: *RoleInfo, gpa: std.mem.Allocator, node_id: i32, active: bool, skill_id: i32) !void {
    for (role.skill_node_state) |*entry| {
        if (entry.node_id == node_id) {
            entry.active = active;
            entry.skill_id = skill_id;
            return;
        }
    }

    const new_nodes = try gpa.alloc(@typeInfo(@TypeOf(role.skill_node_state)).pointer.child, role.skill_node_state.len + 1);
    @memcpy(new_nodes[0..role.skill_node_state.len], role.skill_node_state);
    new_nodes[role.skill_node_state.len] = .{ .node_id = node_id, .active = active, .skill_id = skill_id };

    if (role.skill_node_state.len != 0) gpa.free(role.skill_node_state);
    role.skill_node_state = new_nodes;
}

fn getMaxResonantChainIndex(assets: *const Assets, group_id: i32) i32 {
    var max_index: i32 = 0;
    for (assets.tables.resonant_chain.items) |chain| {
        if (chain.GroupId == group_id and chain.GroupIndex > max_index) max_index = chain.GroupIndex;
    }
    return max_index;
}

fn getResonantChainByIndex(assets: *const Assets, group_id: i32, index: i32) ?Assets.DataTables.ResonantChain {
    for (assets.tables.resonant_chain.items) |chain| {
        if (chain.GroupId == group_id and chain.GroupIndex == index) return chain;
    }
    return null;
}

fn hasActivateConsumeItems(info: InventoryInfo, chain: Assets.DataTables.ResonantChain) bool {
    var iterator = chain.ActivateConsume.map.iterator();
    while (iterator.next()) |entry| {
        if (info.normalItemCount(entry.key_ptr.*) < entry.value_ptr.*) return false;
    }
    return true;
}

fn consumeActivateItems(
    info: *InventoryInfo,
    gpa: std.mem.Allocator,
    arena: std.mem.Allocator,
    chain: Assets.DataTables.ResonantChain,
) !std.ArrayList(pb.NormalItem) {
    var updated_items: std.ArrayList(pb.NormalItem) = .empty;
    var iterator = chain.ActivateConsume.map.iterator();
    while (iterator.next()) |entry| {
        _ = try InventoryInfo.consumeNormalItem(info, gpa, entry.key_ptr.*, entry.value_ptr.*);
        try updated_items.append(arena, .{
            .Id = entry.key_ptr.*,
            .Count = info.normalItemCount(entry.key_ptr.*),
            .ExpireTime = 0,
        });
    }
    return updated_items;
}

fn toClientRoleInfo(info: *const RoleInfo, arena: std.mem.Allocator, id: i32) !pb.RoleInfo {
    var proto: pb.RoleInfo = .{
        .RoleId = id,
        .Name = "",
        .Level = info.level,
        .Exp = info.exp,
        .Breakthrough = info.breakthrough,
        .Star = info.star,
        .Favor = info.favor,
        .CurModel = info.cur_model,
        .CreateTime = info.create_time,
        .ResonantChainGroupIndex = info.resonant_chain_group_index,
        .SkinId = info.role_skin_id,
        .EnableSelfBgm = info.enable_self_bgm,
    };

    try proto.Skills.ensureTotalCapacity(arena, info.skills.len);
    for (info.skills) |entry| proto.Skills.appendAssumeCapacity(.{ .Key = entry[0], .Value = entry[1] });

    try proto.Phantom.ensureTotalCapacity(arena, info.phantom.len);
    for (info.phantom) |entry| proto.Phantom.appendAssumeCapacity(.{ .Key = entry[0], .Value = entry[1] });

    try proto.BaseProp.ensureTotalCapacity(arena, info.base_prop.len);
    for (info.base_prop, 0..) |value, key| proto.BaseProp.appendAssumeCapacity(.{ .Key = @intCast(key), .Value = value });

    try proto.AddProp.ensureTotalCapacity(arena, info.add_prop.len);
    for (info.add_prop, 0..) |value, key| proto.AddProp.appendAssumeCapacity(.{ .Key = @intCast(key), .Value = value });

    try proto.Reson.ensureTotalCapacity(arena, info.reson.len);
    for (info.reson) |entry| proto.Reson.appendAssumeCapacity(.{ .ResonId = entry.id, .IsOpen = entry.open, .Increase = entry.increase });

    try proto.Models.appendSlice(arena, info.models);

    try proto.SkillNodeState.ensureTotalCapacity(arena, info.skill_node_state.len);
    for (info.skill_node_state) |node| {
        proto.SkillNodeState.appendAssumeCapacity(.{ .SkillNodeId = node.node_id, .IsActive = node.active, .SkillId = node.skill_id });
    }

    return proto;
}

pub fn onRobotRolePropRequest(
    txn: *Transaction(pb.RobotRolePropRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
) !void {
    var prop_list: std.ArrayList(pb.RobotRoleInfo) = .empty;

    for (txn.message.RoleIds.items) |role_id| {
        if (!role_comp.role_map.contains(role_id)) continue;
        const snapshot = try RoleStats.buildSnapshot(alloc.gpa, assets, role_comp, weapon_comp, echo_comp, role_id);
        defer snapshot.deinit(alloc.gpa);

        var prop: pb.RobotRoleInfo = .{ .RoleId = role_id };

        try prop.BaseProp.ensureTotalCapacity(alloc.arena, snapshot.base_props.len);
        for (snapshot.base_props, 0..) |value, key| {
            prop.BaseProp.appendAssumeCapacity(.{ .Key = @intCast(key), .Value = value });
        }

        try prop.AddProp.ensureTotalCapacity(alloc.arena, snapshot.add_props.len);
        for (snapshot.add_props, 0..) |value, key| {
            prop.AddProp.appendAssumeCapacity(.{ .Key = @intCast(key), .Value = value });
        }

        try prop_list.append(alloc.arena, prop);
    }

    txn.respond(.{
        .Error = .Success,
        .RobotRoleInfo = prop_list,
    });
}

pub fn onRolePropRequest(
    txn: *Transaction(pb.RolePropRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
) !void {
    var prop_list: std.ArrayList(pb.RolePropInfo) = .empty;
    for (txn.message.RoleIds.items) |role_id| {
        if (!role_comp.role_map.contains(role_id)) continue;
        const snapshot = try RoleStats.buildSnapshot(alloc.gpa, assets, role_comp, weapon_comp, echo_comp, role_id);
        defer snapshot.deinit(alloc.gpa);

        var prop: pb.RolePropInfo = .{ .RoleId = role_id };
        try prop.BaseProp.ensureTotalCapacity(alloc.arena, snapshot.base_props.len);
        for (snapshot.base_props, 0..) |value, key| {
            prop.BaseProp.appendAssumeCapacity(.{ .Key = @intCast(key), .Value = value });
        }
        try prop.AddProp.ensureTotalCapacity(alloc.arena, snapshot.add_props.len);
        for (snapshot.add_props, 0..) |value, key| {
            prop.AddProp.appendAssumeCapacity(.{ .Key = @intCast(key), .Value = value });
        }
        try prop_list.append(alloc.arena, prop);
    }
    txn.respond(.{ .Error = .Success, .RoleProps = prop_list });
}

pub fn onPbUpLevelSkillRequest(
    txn: *Transaction(pb.PbUpLevelSkillRequest),
    events: *EventQueue,
    alloc: mem.Alloc,
    fs: *FileSystem,
    scene: *Scene,
    stat_query: Scene.Query(&.{
        Scene.Entity,
        *Scene.Entity.ConfigComponent,
        *Scene.Entity.AttributeComponent,
    }),
    assets: *const Assets,
    basic_comp: *PlayerBasicComponent,
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    inventory_comp: *PlayerInventoryComponent,
    qctx: QuestEngine.QuestContext,
    io: std.Io,
) !void {
    const request = txn.message;
    const role = role_comp.role_map.getPtr(request.RoleId) orelse {
        txn.respond(.{ .ErrorCode = .RequestParamError });
        return;
    };
    const role_config = assets.tables.role_info.getDataById(request.RoleId) orelse {
        txn.respond(.{ .ErrorCode = .ErrRoleNoConfig });
        return;
    };

    const skill = assets.tables.skill.getDataById(request.SkillId) orelse {
        txn.respond(.{ .ErrorCode = .ErrSkillInfoParamError });
        return;
    };
    if (skill.SkillGroupId != role_config.SkillId) {
        txn.respond(.{ .ErrorCode = .ErrSkillInfoParamError });
        return;
    }

    const current_level = getSkillLevel(role, request.SkillId);
    if (current_level <= 0 or current_level >= skill.MaxSkillLevel) {
        txn.respond(.{ .ErrorCode = .ErrRoleLevelMax });
        return;
    }
    const next_level = current_level + 1;

    if (findSkillLevelConfig(assets, skill.SkillLevelGroupId, next_level)) |lvl_cfg| {
        const eval_ctx = quest_types.makeEvalContextForRole(qctx, request.RoleId);
        if (!ConditionEvaluator.evalConditionOrGroup(eval_ctx, lvl_cfg.Condition)) {
            txn.respond(.{ .ErrorCode = .ErrRoleConditionNoEnough });
            return;
        }
        var costs: std.ArrayList(ConsumeHelper.CostItem) = .empty;
        try ConsumeHelper.appendIntMap(&costs, alloc.arena, lvl_cfg.Consume);
        if (!(try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, costs.items, io))) {
            txn.respond(.{ .ErrorCode = .ErrItemNotEnough });
            return;
        }
    }

    try setSkillLevel(role, alloc.gpa, request.SkillId, next_level);
    try saveRoleImmediate(fs, alloc.arena, role_comp.player_id, request.RoleId, role.*);
    try events.enqueue(.role_info_modified, .{ .role_id = request.RoleId });
    try RoleAttributeSync.refreshRole(txn, alloc, fs, scene, assets, role_comp, weapon_comp, echo_comp, stat_query, request.RoleId, io);
    QuestEngine.onSkillUnlock(qctx, request.SkillId) catch |err| QuestEngine.logHookErr("onSkillUnlock", err);

    txn.respond(.{
        .ErrorCode = .Success,
        .RoleId = request.RoleId,
        .SkillInfo = .{ .Key = request.SkillId, .Value = next_level },
    });
}

pub fn onRoleActivateSkillRequest(
    txn: *Transaction(pb.RoleActivateSkillRequest),
    events: *EventQueue,
    alloc: mem.Alloc,
    fs: *FileSystem,
    scene: *Scene,
    stat_query: Scene.Query(&.{
        Scene.Entity,
        *Scene.Entity.ConfigComponent,
        *Scene.Entity.AttributeComponent,
    }),
    assets: *const Assets,
    basic_comp: *PlayerBasicComponent,
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    inventory_comp: *PlayerInventoryComponent,
    qctx: QuestEngine.QuestContext,
    io: std.Io,
) !void {
    const request = txn.message;
    const role = role_comp.role_map.getPtr(request.RoleId) orelse {
        txn.respond(.{ .ErrorCode = .RequestParamError });
        return;
    };
    const role_config = assets.tables.role_info.getDataById(request.RoleId) orelse {
        txn.respond(.{ .ErrorCode = .ErrRoleNoConfig });
        return;
    };

    const node = assets.tables.skill_tree.getDataById(request.SkillNodeId) orelse {
        txn.respond(.{ .ErrorCode = .ErrRolSkillNodeTypeUlock });
        return;
    };
    if (node.NodeGroup != role_config.SkillTreeGroupId) {
        txn.respond(.{ .ErrorCode = .ErrRolSkillNodeTypeUlock });
        return;
    }
    if (isSkillNodeActive(role, request.SkillNodeId)) {
        txn.respond(.{ .ErrorCode = .ErrRolSkillNodeTypeActive });
        return;
    }

    const eval_ctx = quest_types.makeEvalContextForRole(qctx, request.RoleId);
    if (!ConditionEvaluator.evalConditionOrGroup(eval_ctx, node.UnLockCondition)) {
        txn.respond(.{ .ErrorCode = .ErrRolSkillNodeTypeUlock });
        return;
    }
    for (node.Condition) |cond_id| {
        if (!ConditionEvaluator.evalConditionOrGroup(eval_ctx, cond_id)) {
            txn.respond(.{ .ErrorCode = .ErrRolSkillNodeTypeUlock });
            return;
        }
    }

    var costs: std.ArrayList(ConsumeHelper.CostItem) = .empty;
    try ConsumeHelper.appendIntMap(&costs, alloc.arena, node.Consume);
    if (!(try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, costs.items, io))) {
        txn.respond(.{ .ErrorCode = .ErrItemNotEnough });
        return;
    }

    try setSkillNodeState(role, alloc.gpa, request.SkillNodeId, true, node.SkillId);
    if (node.SkillId > 0 and getSkillLevel(role, node.SkillId) == 0) {
        try setSkillLevel(role, alloc.gpa, node.SkillId, 1);
    }
    try saveRoleImmediate(fs, alloc.arena, role_comp.player_id, request.RoleId, role.*);
    try events.enqueue(.role_info_modified, .{ .role_id = request.RoleId });

    var node_states: std.ArrayList(pb.ArraySkillNode) = .empty;
    try node_states.ensureTotalCapacity(alloc.arena, role.skill_node_state.len);
    for (role.skill_node_state) |entry| {
        node_states.appendAssumeCapacity(.{
            .SkillNodeId = entry.node_id,
            .IsActive = entry.active,
            .SkillId = entry.skill_id,
        });
    }

    try txn.conn.push(alloc.gpa, io, pb.RoleSkillNodeNotify{
        .RoleId = request.RoleId,
        .SkillNodeState = node_states,
    });
    try RoleAttributeSync.refreshRole(txn, alloc, fs, scene, assets, role_comp, weapon_comp, echo_comp, stat_query, request.RoleId, io);
    QuestEngine.onSkillUnlock(qctx, node.SkillId) catch |err| QuestEngine.logHookErr("onSkillUnlock", err);

    txn.respond(.{
        .ErrorCode = .Success,
        .RoleId = request.RoleId,
        .SkillInfo = .{ .Key = node.SkillId, .Value = 1 },
    });
}

pub fn onRoleSkillLevelUpViewRequest(
    txn: *Transaction(pb.RoleSkillLevelUpViewRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
    role_comp: *PlayerRoleComponent,
) !void {
    const role = role_comp.role_map.get(txn.message.RoleId) orelse {
        txn.respond(.{ .ErrorCode = .RequestParamError });
        return;
    };
    const role_config = assets.tables.role_info.getDataById(txn.message.RoleId) orelse {
        txn.respond(.{ .ErrorCode = .ErrRoleNoConfig });
        return;
    };
    const skill = assets.tables.skill.getDataById(txn.message.SkillId) orelse {
        txn.respond(.{ .ErrorCode = .ErrSkillInfoParamError });
        return;
    };
    if (skill.SkillGroupId != role_config.SkillId) {
        txn.respond(.{ .ErrorCode = .ErrSkillInfoParamError });
        return;
    }

    var cost_list: std.ArrayList(pb.ArrayIntInt) = .empty;
    const next_level = getSkillLevel(&role, txn.message.SkillId) + 1;
    if (next_level <= skill.MaxSkillLevel) {
        if (findSkillLevelConfig(assets, skill.SkillLevelGroupId, next_level)) |lvl_cfg| {
            var it = lvl_cfg.Consume.map.iterator();
            while (it.next()) |kv| {
                if (kv.key_ptr.* <= 0 or kv.value_ptr.* <= 0) continue;
                try cost_list.append(alloc.arena, .{ .Key = kv.key_ptr.*, .Value = kv.value_ptr.* });
            }
        }
    }

    txn.respond(.{ .ErrorCode = .Success, .CostList = cost_list });
}

pub fn onRoleSkillViewRequest(
    txn: *Transaction(pb.RoleSkillViewRequest),
    assets: *const Assets,
    role_comp: *PlayerRoleComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    const role = role_comp.role_map.get(txn.message.RoleId) orelse {
        txn.respond(.{ .ErrorCode = .RequestParamError });
        return;
    };
    const role_config = assets.tables.role_info.getDataById(txn.message.RoleId) orelse {
        txn.respond(.{ .ErrorCode = .ErrRoleNoConfig });
        return;
    };
    const skill = assets.tables.skill.getDataById(txn.message.SkillId) orelse {
        txn.respond(.{ .ErrorCode = .ErrSkillInfoParamError });
        return;
    };
    if (skill.SkillGroupId != role_config.SkillId) {
        txn.respond(.{ .ErrorCode = .ErrSkillInfoParamError });
        return;
    }

    const next_level = getSkillLevel(&role, txn.message.SkillId) + 1;
    const cond_ok = if (next_level <= skill.MaxSkillLevel) blk: {
        if (findSkillLevelConfig(assets, skill.SkillLevelGroupId, next_level)) |lvl_cfg| {
            break :blk ConditionEvaluator.evalConditionOrGroup(quest_types.makeEvalContextForRole(qctx, txn.message.RoleId), lvl_cfg.Condition);
        }
        break :blk true;
    } else false;

    txn.respond(.{
        .ErrorCode = .Success,
        .IsConditionFinish = cond_ok,
    });
}

pub fn onRoleSkillQuickLevelUpRequest(
    txn: *Transaction(pb.RoleSkillQuickLevelUpRequest),
    events: *EventQueue,
    alloc: mem.Alloc,
    fs: *FileSystem,
    scene: *Scene,
    stat_query: Scene.Query(&.{
        Scene.Entity,
        *Scene.Entity.ConfigComponent,
        *Scene.Entity.AttributeComponent,
    }),
    assets: *const Assets,
    basic_comp: *PlayerBasicComponent,
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    inventory_comp: *PlayerInventoryComponent,
    qctx: QuestEngine.QuestContext,
    io: std.Io,
) !void {
    const request = txn.message;
    const role = role_comp.role_map.getPtr(request.RoleId) orelse {
        txn.respond(.{ .ErrorCode = .RequestParamError });
        return;
    };
    const role_config = assets.tables.role_info.getDataById(request.RoleId) orelse {
        txn.respond(.{ .ErrorCode = .ErrRoleNoConfig });
        return;
    };

    const skill = assets.tables.skill.getDataById(request.SkillId) orelse {
        txn.respond(.{ .ErrorCode = .ErrSkillInfoParamError });
        return;
    };
    if (skill.SkillGroupId != role_config.SkillId) {
        txn.respond(.{ .ErrorCode = .ErrSkillInfoParamError });
        return;
    }

    const current_level = getSkillLevel(role, request.SkillId);
    if (current_level <= 0 or current_level >= skill.MaxSkillLevel) {
        txn.respond(.{ .ErrorCode = .ErrRoleLevelMax });
        return;
    }
    const requested_level = if (request.TargetLevel > current_level) request.TargetLevel else current_level + 1;
    const next_level = @min(requested_level, skill.MaxSkillLevel);

    const eval_ctx = quest_types.makeEvalContextForRole(qctx, request.RoleId);
    var costs: std.ArrayList(ConsumeHelper.CostItem) = .empty;
    var lv: i32 = current_level + 1;
    while (lv <= next_level) : (lv += 1) {
        if (findSkillLevelConfig(assets, skill.SkillLevelGroupId, lv)) |lvl_cfg| {
            if (!ConditionEvaluator.evalConditionOrGroup(eval_ctx, lvl_cfg.Condition)) {
                txn.respond(.{ .ErrorCode = .ErrRoleConditionNoEnough });
                return;
            }
            try ConsumeHelper.appendIntMap(&costs, alloc.arena, lvl_cfg.Consume);
        }
    }

    if (!(try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, costs.items, io))) {
        txn.respond(.{ .ErrorCode = .ErrItemNotEnough });
        return;
    }

    try setSkillLevel(role, alloc.gpa, request.SkillId, next_level);
    try saveRoleImmediate(fs, alloc.arena, role_comp.player_id, request.RoleId, role.*);
    try events.enqueue(.role_info_modified, .{ .role_id = request.RoleId });
    try RoleAttributeSync.refreshRole(txn, alloc, fs, scene, assets, role_comp, weapon_comp, echo_comp, stat_query, request.RoleId, io);
    QuestEngine.onSkillUnlock(qctx, request.SkillId) catch |err| QuestEngine.logHookErr("onSkillUnlock", err);

    txn.respond(.{
        .ErrorCode = .Success,
        .RoleInfo = try RoleStats.toClientRoleInfo(
            alloc.gpa,
            alloc.arena,
            assets,
            role_comp,
            request.RoleId,
            role,
            weapon_comp,
            echo_comp,
        ),
    });
}

pub fn onPlayerVoiceLanguageRequest(
    txn: *Transaction(pb.PlayerVoiceLanguageRequest),
    alloc: mem.Alloc,
    role_comp: *PlayerRoleComponent,
) !void {
    var role_voices: std.ArrayList(pb.RoleVoice) = .empty;

    var iterator = role_comp.role_map.iterator();
    while (iterator.next()) |entry| {
        if (entry.value_ptr.voice_language == 0) continue;

        try role_voices.append(alloc.arena, .{
            .RoleId = entry.key_ptr.*,
            .VoiceId = entry.value_ptr.voice_language,
        });
    }

    txn.respond(.{
        .ErrorCode = .Success,
        .RoleVoices = role_voices,
    });
}

pub fn onPlayerRoleVoiceSetRequest(
    txn: *Transaction(pb.PlayerRoleVoiceSetRequest),
    events: *EventQueue,
    role_comp: *PlayerRoleComponent,
) !void {
    if (txn.message.RoleVoices.items.len == 0) {
        var iterator = role_comp.role_map.iterator();
        while (iterator.next()) |entry| {
            if (entry.value_ptr.voice_language == 0) continue;
            entry.value_ptr.voice_language = 0;
            try events.enqueue(.role_info_modified, .{ .role_id = entry.key_ptr.* });
        }

        txn.respond(.{ .ErrorCode = .Success });
        return;
    }

    for (txn.message.RoleVoices.items) |voice| {
        const role = role_comp.role_map.getPtr(voice.RoleId) orelse continue;
        if (role.voice_language == voice.VoiceLanguage) continue;

        role.voice_language = voice.VoiceLanguage;
        try events.enqueue(.role_info_modified, .{ .role_id = voice.RoleId });
    }

    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onResonantChainUnlockRequest(
    txn: *Transaction(pb.ResonantChainUnlockRequest),
    events: *EventQueue,
    alloc: mem.Alloc,
    fs: *FileSystem,
    scene: *Scene,
    query: Scene.Query(&.{
        Scene.Entity,
        *Scene.Entity.ConfigComponent,
    }),
    stat_query: Scene.Query(&.{
        Scene.Entity,
        *Scene.Entity.ConfigComponent,
        *Scene.Entity.AttributeComponent,
    }),
    assets: *const Assets,
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    inventory_comp: *PlayerInventoryComponent,
    io: std.Io,
) !void {
    const request = txn.message;
    const role = role_comp.role_map.getPtr(request.RoleId) orelse {
        txn.respond(.{ .ErrorCode = .RequestParamError });
        return;
    };
    const role_config = assets.tables.role_info.getDataById(request.RoleId) orelse {
        txn.respond(.{ .ErrorCode = .RequestParamError });
        return;
    };

    const max_index = getMaxResonantChainIndex(assets, role_config.ResonantChainGroupId);
    if (max_index == 0) {
        txn.respond(.{ .ErrorCode = .RequestParamError });
        return;
    }
    if (role.resonant_chain_group_index >= max_index) {
        txn.respond(.{
            .ErrorCode = .Success,
            .RoleId = request.RoleId,
            .ResonantChainGroupIndex = role.resonant_chain_group_index,
        });
        return;
    }

    const next_index = role.resonant_chain_group_index + 1;
    const chain = getResonantChainByIndex(assets, role_config.ResonantChainGroupId, next_index) orelse {
        txn.respond(.{ .ErrorCode = .RequestParamError });
        return;
    };
    if (!hasActivateConsumeItems(inventory_comp.info, chain)) {
        txn.respond(.{ .ErrorCode = .ErrRoleItemListNoEnough });
        return;
    }
    const updated_items = try consumeActivateItems(&inventory_comp.info, alloc.gpa, alloc.arena, chain);

    role.resonant_chain_group_index = @min(role.resonant_chain_group_index + 1, max_index);
    try saveRoleImmediate(fs, alloc.arena, role_comp.player_id, request.RoleId, role.*);
    try events.enqueue(.role_info_modified, .{ .role_id = request.RoleId });

    if (getResonantChainByIndex(assets, role_config.ResonantChainGroupId, role.resonant_chain_group_index)) |new_chain| {
        if (new_chain.BuffIds.len > 0) {
            var entity_handle: ?Scene.Entity = null;
            var it = query.iterator;
            while (it.next()) |item| {
                const entity, const config_item = item;
                if (config_item.config_id == request.RoleId) {
                    entity_handle = entity;
                    break;
                }
            }

            if (entity_handle) |handle| {
                var buffs = try std.ArrayList(Events.BuffAdditionEntry).initCapacity(alloc.arena, new_chain.BuffIds.len);
                for (new_chain.BuffIds) |buff_id| {
                    buffs.appendAssumeCapacity(.{
                        .id = buff_id,
                        .is_active = true,
                        .stack_count = 1,
                    });
                }
                try events.enqueue(.buff_addition, .{
                    .target = handle,
                    .instigator = handle,
                    .buffs = buffs.items,
                });
            }
        }
    }

    const path = try std.fmt.allocPrint(alloc.arena, "player/{}/{s}", .{ inventory_comp.player_id, InventoryInfo.data_path });
    try comp_util.saveStruct(fs, inventory_comp.info, path, alloc.arena);
    try txn.conn.push(alloc.gpa, io, pb.NormalItemUpdateNotify{
        .NormalItemList = updated_items,
        .NoTips = true,
    });
    try RoleAttributeSync.refreshRole(txn, alloc, fs, scene, assets, role_comp, weapon_comp, echo_comp, stat_query, request.RoleId, io);

    txn.respond(.{
        .ErrorCode = .Success,
        .RoleId = request.RoleId,
        .ResonantChainGroupIndex = role.resonant_chain_group_index,
    });
}

pub fn onRoleSkillBranchModifyRequest(
    txn: *Transaction(pb.RoleSkillBranchModifyRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    events: *EventQueue,
    role_comp: *PlayerRoleComponent,
    query: Scene.Query(&.{
        Scene.Entity,
        *Scene.Entity.ConfigComponent,
    }),
) !void {
    const role = role_comp.role_map.getPtr(txn.message.RoleId) orelse {
        txn.respond(.{ .ErrorCode = .ErrSkillBranchRoleValid });
        return;
    };
    const role_config = assets.tables.role_info.getDataById(txn.message.RoleId) orelse {
        txn.respond(.{ .ErrorCode = .ErrRoleNoConfig });
        return;
    };
    if (!isValidSkillBranchForRole(assets, role_config, txn.message.RoleId, txn.message.SkillBranch)) {
        txn.respond(.{ .ErrorCode = .ErrSkillBranchNotValid });
        return;
    }

    const prev_skill_branch = role.skill_branch;
    var entity_handle: ?Scene.Entity = null;
    var it = query.iterator;
    while (it.next()) |item| {
        const entity, const config = item;
        if (config.config_id == txn.message.RoleId) {
            entity_handle = entity;
            break;
        }
    }

    if (entity_handle) |handle| {
        if (prev_skill_branch != 0) {
            if (assets.tables.skill_branch_buff.getDataById(prev_skill_branch)) |prev_branch_data| {
                for (prev_branch_data.Buffs) |buff_id| {
                    try events.enqueue(.buff_removal_by_id, .{
                        .entity = handle,
                        .buff_id = buff_id,
                    });
                }
            }
        }

        if (txn.message.SkillBranch != 0) {
            if (assets.tables.skill_branch_buff.getDataById(txn.message.SkillBranch)) |new_branch_data| {
                var add_buffs: std.ArrayList(Events.BuffAdditionEntry) = .empty;
                for (new_branch_data.Buffs) |buff_id| {
                    try add_buffs.append(alloc.arena, .{
                        .id = buff_id,
                        .is_active = true,
                        .stack_count = 1,
                    });
                }
                try events.enqueue(.buff_addition, .{
                    .target = handle,
                    .instigator = handle,
                    .buffs = add_buffs.items,
                });
            }
        }
    }

    role.skill_branch = txn.message.SkillBranch;
    try saveRoleImmediate(fs, alloc.arena, role_comp.player_id, txn.message.RoleId, role.*);
    try events.enqueue(.role_info_modified, .{ .role_id = txn.message.RoleId });
    try events.enqueue(.update_formations, .{});

    txn.respond(.{ .ErrorCode = .Success });
}

fn roleLevelExp(assets: *const Assets, consume_group_id: i32, level: i32) i32 {
    for (assets.tables.role_level_consume.items) |entry| {
        if (entry.ConsumeGroupId == consume_group_id and entry.Level == level) {
            return if (entry.ExpCount > 0) entry.ExpCount else 1000;
        }
    }
    return 1000;
}

fn roleLevelLimit(assets: *const Assets, breach_group_id: i32, breach: i32) i32 {
    for (assets.tables.role_breach.items) |entry| {
        if (entry.BreachGroupId == breach_group_id and entry.BreachLevel == breach) {
            return entry.MaxLevel;
        }
    }
    return switch (breach) {
        0 => 20,
        1 => 40,
        2 => 50,
        3 => 60,
        4 => 70,
        5 => 80,
        else => 90,
    };
}

fn roleExpItemValue(assets: *const Assets, item_id: i32) i32 {
    const item = assets.tables.role_exp_item.getDataById(item_id) orelse return 0;
    return item.BasicExp;
}

pub fn onRoleLevelUpViewRequest(
    txn: *Transaction(pb.RoleLevelUpViewRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    inventory_comp: *PlayerInventoryComponent,
) !void {
    const request = txn.message;
    const role = role_comp.role_map.getPtr(request.RoleId) orelse {
        txn.respond(.{ .ErrorCode = .RequestParamError });
        return;
    };
    const role_config = assets.tables.role_info.getDataById(request.RoleId) orelse {
        txn.respond(.{ .ErrorCode = .ErrRoleNoConfig });
        return;
    };

    const max_level = roleLevelLimit(assets, role_config.BreachId, role.breakthrough);
    var level_exp_info: std.ArrayList(pb.ArrayIntInt) = .empty;
    for (assets.tables.role_level_consume.items) |entry| {
        if (entry.ConsumeGroupId == role_config.LevelConsumeId) {
            try level_exp_info.append(alloc.arena, .{ .Key = entry.Level, .Value = entry.ExpCount });
        }
    }

    var selected_items: std.ArrayList(pb.ArrayIntInt) = .empty;
    var add_exp_i64: i64 = 0;
    if (request.ItemList.items.len > 0) {
        var used_counts: std.AutoHashMapUnmanaged(i32, i64) = .empty;
        defer used_counts.deinit(alloc.arena);
        for (request.ItemList.items) |item| {
            if (item.Key <= 0 or item.Value <= 0) continue;
            const exp_val = roleExpItemValue(assets, item.Key);
            if (exp_val <= 0) continue;
            const owned: i64 = inventory_comp.info.normalItemCount(item.Key);
            const already: i64 = used_counts.get(item.Key) orelse 0;
            const take_i64 = @min(@as(i64, item.Value), @max(@as(i64, 0), owned - already));
            if (take_i64 <= 0) continue;
            try used_counts.put(alloc.arena, item.Key, already + take_i64);
            try selected_items.append(alloc.arena, .{ .Key = item.Key, .Value = @intCast(take_i64) });
            add_exp_i64 += @as(i64, exp_val) * take_i64;
        }
    } else if (request.MaxItemId > 0 and role.level < max_level) {
        var needed_total: i64 = -@as(i64, role.exp);
        var lv: i32 = role.level;
        while (lv < max_level) : (lv += 1) {
            needed_total += roleLevelExp(assets, role_config.LevelConsumeId, lv + 1);
        }
        var remaining: i64 = @max(needed_total, 0);

        // Iterate descending by BasicExp to select items efficiently within MaxItemId.
        var idx = assets.tables.role_exp_item.items.len;
        while (idx > 0 and remaining > 0) {
            idx -= 1;
            const exp_item = assets.tables.role_exp_item.items[idx];
            if (exp_item.Id > request.MaxItemId or exp_item.BasicExp <= 0) continue;
            const owned: i64 = inventory_comp.info.normalItemCount(exp_item.Id);
            if (owned <= 0) continue;
            const b_exp: i64 = exp_item.BasicExp;
            const take_raw = if (idx == 0)
                @divTrunc(remaining + b_exp - 1, b_exp)
            else
                @divTrunc(remaining, b_exp);
            const count_i64: i64 = @min(owned, @max(take_raw, 0));
            if (count_i64 <= 0) continue;
            try selected_items.append(alloc.arena, .{ .Key = exp_item.Id, .Value = @intCast(count_i64) });
            const gain = count_i64 * b_exp;
            add_exp_i64 += gain;
            remaining -= gain;
        }
    }
    const add_exp: i32 = @intCast(@min(add_exp_i64, @as(i64, std.math.maxInt(i32))));

    var sim_level = role.level;
    var sim_exp = role.exp + add_exp;
    while (sim_level < max_level) {
        const next_req = roleLevelExp(assets, role_config.LevelConsumeId, sim_level + 1);
        if (next_req <= 0 or sim_exp < next_req) break;
        sim_exp -= next_req;
        sim_level += 1;
    }
    if (sim_level >= max_level) sim_exp = 0;

    const final_prop = try buildPreviewFinalProps(
        alloc.gpa,
        alloc.arena,
        assets,
        request.RoleId,
        role,
        sim_level,
        role.breakthrough,
        weapon_comp,
        echo_comp,
    );

    var cost_list: std.ArrayList(pb.ArrayIntInt) = .empty;
    if (add_exp > 0) {
        try cost_list.append(alloc.arena, .{ .Key = 2, .Value = @max(1, @divTrunc(add_exp * 35, 100)) });
    }

    txn.respond(.{
        .ErrorCode = .Success,
        .Level = sim_level,
        .LevelExpInfo = level_exp_info,
        .Exp = sim_exp,
        .AddExp = add_exp,
        .FinalProp = final_prop,
        .CostList = cost_list,
        .ItemList = selected_items,
    });
}

pub fn onPbUpLevelRoleRequest(
    txn: *Transaction(pb.PbUpLevelRoleRequest),
    events: *EventQueue,
    alloc: mem.Alloc,
    fs: *FileSystem,
    scene: *Scene,
    stat_query: Scene.Query(&.{
        Scene.Entity,
        *Scene.Entity.ConfigComponent,
        *Scene.Entity.AttributeComponent,
    }),
    assets: *const Assets,
    basic_comp: *PlayerBasicComponent,
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    inventory_comp: *PlayerInventoryComponent,
    qctx: QuestEngine.QuestContext,
    io: std.Io,
) !void {
    const request = txn.message;
    const role = role_comp.role_map.getPtr(request.RoleId) orelse {
        txn.respond(.{ .ErrorCode = .RequestParamError });
        return;
    };
    const role_config = assets.tables.role_info.getDataById(request.RoleId) orelse {
        txn.respond(.{ .ErrorCode = .ErrRoleNoConfig });
        return;
    };

    const max_level = roleLevelLimit(assets, role_config.BreachId, role.breakthrough);
    if (role.level >= max_level) {
        txn.respond(.{
            .ErrorCode = .ErrRoleLevelMax,
            .RoleId = request.RoleId,
            .Level = role.level,
            .Exp = role.exp,
        });
        return;
    }

    var gained_exp_i64: i64 = 0;
    var req_counts: std.AutoHashMapUnmanaged(i32, i64) = .empty;
    defer req_counts.deinit(alloc.arena);
    for (request.ItemList.items) |item| {
        if (item.Key <= 0 or item.Value <= 0) continue;
        const exp_val = roleExpItemValue(assets, item.Key);
        if (exp_val <= 0) continue;
        const gop = try req_counts.getOrPut(alloc.arena, item.Key);
        if (!gop.found_existing) gop.value_ptr.* = 0;
        gop.value_ptr.* += item.Value;
        if (inventory_comp.info.normalItemCount(item.Key) < gop.value_ptr.*) {
            txn.respond(.{
                .ErrorCode = .ErrRoleItemListNoEnough,
                .RoleId = request.RoleId,
                .Level = role.level,
                .Exp = role.exp,
            });
            return;
        }
        gained_exp_i64 += std.math.mul(i64, exp_val, item.Value) catch {
            txn.respond(.{ .ErrorCode = .ErrRoleItemExpError, .RoleId = request.RoleId, .Level = role.level, .Exp = role.exp });
            return;
        };
    }

    if (gained_exp_i64 <= 0 or gained_exp_i64 > std.math.maxInt(i32)) {
        txn.respond(.{
            .ErrorCode = .ErrRoleItemExpError,
            .RoleId = request.RoleId,
            .Level = role.level,
            .Exp = role.exp,
        });
        return;
    }
    const gained_exp: i32 = @intCast(gained_exp_i64);

    const coin_cost_i64: i64 = @max(@as(i64, 1), @divTrunc(gained_exp_i64 * 35, 100));
    if (coin_cost_i64 > std.math.maxInt(i32) or basic_comp.info.attributes.coin < @as(i32, @intCast(coin_cost_i64))) {
        txn.respond(.{
            .ErrorCode = .ErrItemNotEnough,
            .RoleId = request.RoleId,
            .Level = role.level,
            .Exp = role.exp,
        });
        return;
    }
    const coin_cost: i32 = @intCast(coin_cost_i64);

    var item_map: std.ArrayList(pb.MapEntry(i32, i32)) = .empty;
    var updated_items: std.ArrayList(pb.NormalItem) = .empty;
    var req_it = req_counts.iterator();
    while (req_it.next()) |kv| {
        const consumed_cnt: i32 = @intCast(kv.value_ptr.*);
        _ = try InventoryInfo.consumeNormalItem(&inventory_comp.info, alloc.gpa, kv.key_ptr.*, consumed_cnt);
    }

    var total_exp: i64 = @as(i64, role.exp) + gained_exp;
    while (role.level < max_level) {
        const next_req = roleLevelExp(assets, role_config.LevelConsumeId, role.level + 1);
        if (next_req <= 0 or total_exp < next_req) break;
        total_exp -= next_req;
        role.level += 1;
    }

    var touched_potions = std.AutoHashMapUnmanaged(i32, void).empty;
    defer touched_potions.deinit(alloc.arena);
    var req_it2 = req_counts.iterator();
    while (req_it2.next()) |kv| {
        try touched_potions.put(alloc.arena, kv.key_ptr.*, {});
    }

    if (role.level >= max_level) {
        var excess: i64 = total_exp;
        role.exp = 0;
        const refund_pots = [_]struct { id: i32, exp: i64 }{
            .{ .id = 43010004, .exp = 20000 },
            .{ .id = 43010003, .exp = 8000 },
            .{ .id = 43010002, .exp = 3000 },
            .{ .id = 43010001, .exp = 1000 },
        };
        for (refund_pots) |pot| {
            if (excess >= pot.exp) {
                const cnt: i32 = @intCast(@min(@divTrunc(excess, pot.exp), @as(i64, std.math.maxInt(i32))));
                if (cnt > 0) {
                    excess -= @as(i64, cnt) * pot.exp;
                    try InventoryInfo.addNormalItem(&inventory_comp.info, alloc.gpa, pot.id, cnt);
                    try item_map.append(alloc.arena, .{ .key = pot.id, .value = cnt });
                    try touched_potions.put(alloc.arena, pot.id, {});
                }
            }
        }
    } else {
        role.exp = @intCast(total_exp);
    }

    var touched_it = touched_potions.keyIterator();
    while (touched_it.next()) |pid| {
        const remaining = inventory_comp.info.normalItemCount(pid.*);
        try updated_items.append(alloc.arena, .{ .Id = pid.*, .Count = remaining, .ExpireTime = 0 });
    }

    basic_comp.info.attributes.coin -= coin_cost;
    try updated_items.append(alloc.arena, .{ .Id = 2, .Count = basic_comp.info.attributes.coin, .ExpireTime = 0 });

    const inv_path = try std.fmt.allocPrint(alloc.arena, "player/{}/{s}", .{ inventory_comp.player_id, InventoryInfo.data_path });
    try comp_util.saveStruct(fs, inventory_comp.info, inv_path, alloc.arena);
    const basic_path = try std.fmt.allocPrint(alloc.arena, "player/{}/basic_info", .{basic_comp.player_id});
    try comp_util.saveStruct(fs, basic_comp.info, basic_path, alloc.arena);

    if (updated_items.items.len > 0) {
        try txn.conn.push(alloc.gpa, io, pb.NormalItemUpdateNotify{
            .NormalItemList = updated_items,
            .NoTips = true,
        });
    }
    var coin_attrs: std.ArrayList(pb.PlayerAttr) = .empty;
    try coin_attrs.append(alloc.arena, .{ .Key = .Coin, .ValueType = .Int32, .Value = .{ .Int32Value = basic_comp.info.attributes.coin } });
    try txn.conn.push(alloc.gpa, io, pb.PlayerAttrNotify{ .Attributes = coin_attrs });

    _ = try role.refreshScaledProperties(alloc.gpa, assets, request.RoleId);
    try saveRoleImmediate(fs, alloc.arena, role_comp.player_id, request.RoleId, role.*);

    try events.enqueue(.role_info_modified, .{ .role_id = request.RoleId });
    try events.enqueue(.update_formations, .{});
    try RoleAttributeSync.refreshRole(txn, alloc, fs, scene, assets, role_comp, weapon_comp, echo_comp, stat_query, request.RoleId, io);

    txn.respond(.{
        .ErrorCode = .Success,
        .RoleId = request.RoleId,
        .Level = role.level,
        .Exp = role.exp,
        .ItemMap = item_map,
    });
    try QuestEngine.onConditionStateChanged(qctx);
}

pub fn onRoleBreakThroughViewRequest(
    txn: *Transaction(pb.RoleBreakThroughViewRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    const request = txn.message;
    const role = role_comp.role_map.getPtr(request.RoleId) orelse {
        txn.respond(.{ .ErrorCode = .RequestParamError });
        return;
    };
    const role_config = assets.tables.role_info.getDataById(request.RoleId) orelse {
        txn.respond(.{ .ErrorCode = .ErrRoleNoConfig });
        return;
    };

    const target_breach = @min(role.breakthrough + 1, 6);
    var cost_list: std.ArrayList(pb.ArrayIntInt) = .empty;
    var level_limit = roleLevelLimit(assets, role_config.BreachId, target_breach);
    var is_condition_finish = true;

    if (findRoleBreachEntry(assets, role_config.BreachId, target_breach)) |entry| {
        level_limit = entry.MaxLevel;
        is_condition_finish = ConditionEvaluator.evalConditionOrGroup(
            quest_types.makeEvalContextForRole(qctx, request.RoleId),
            entry.ConditionId,
        );
        var it = entry.BreachConsume.map.iterator();
        while (it.next()) |kv| {
            if (kv.key_ptr.* <= 0 or kv.value_ptr.* <= 0) continue;
            try cost_list.append(alloc.arena, .{ .Key = kv.key_ptr.*, .Value = kv.value_ptr.* });
        }
    }

    const final_prop = try buildPreviewFinalProps(
        alloc.gpa,
        alloc.arena,
        assets,
        request.RoleId,
        role,
        role.level,
        target_breach,
        weapon_comp,
        echo_comp,
    );

    txn.respond(.{
        .ErrorCode = .Success,
        .LevelLimit = level_limit,
        .CostList = cost_list,
        .FinalProp = final_prop,
        .IsConditionFinish = is_condition_finish,
    });
}

pub fn onPbOverRoleRequest(
    txn: *Transaction(pb.PbOverRoleRequest),
    events: *EventQueue,
    alloc: mem.Alloc,
    fs: *FileSystem,
    scene: *Scene,
    stat_query: Scene.Query(&.{
        Scene.Entity,
        *Scene.Entity.ConfigComponent,
        *Scene.Entity.AttributeComponent,
    }),
    assets: *const Assets,
    basic_comp: *PlayerBasicComponent,
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    inventory_comp: *PlayerInventoryComponent,
    qctx: QuestEngine.QuestContext,
    io: std.Io,
) !void {
    const request = txn.message;
    const role = role_comp.role_map.getPtr(request.RoleId) orelse {
        txn.respond(.{ .ErrorCode = .RequestParamError });
        return;
    };
    const role_config = assets.tables.role_info.getDataById(request.RoleId) orelse {
        txn.respond(.{ .ErrorCode = .ErrRoleNoConfig });
        return;
    };

    if (role.breakthrough >= 6) {
        txn.respond(.{
            .ErrorCode = .ErrRoleOverNotEnough,
            .RoleId = request.RoleId,
            .Breakthrough = role.breakthrough,
        });
        return;
    }

    const current_limit = roleLevelLimit(assets, role_config.BreachId, role.breakthrough);
    if (role.level < current_limit) {
        txn.respond(.{
            .ErrorCode = .ErrRoleLevelNotEnough,
            .RoleId = request.RoleId,
            .Breakthrough = role.breakthrough,
        });
        return;
    }

    const target_breach = role.breakthrough + 1;
    const breach_cfg = findRoleBreachEntry(assets, role_config.BreachId, target_breach) orelse {
        txn.respond(.{
            .ErrorCode = .ErrRoleOverNotEnough,
            .RoleId = request.RoleId,
            .Breakthrough = role.breakthrough,
        });
        return;
    };

    if (!ConditionEvaluator.evalConditionOrGroup(
        quest_types.makeEvalContextForRole(qctx, request.RoleId),
        breach_cfg.ConditionId,
    )) {
        txn.respond(.{
            .ErrorCode = .ErrRoleConditionNoEnough,
            .RoleId = request.RoleId,
            .Breakthrough = role.breakthrough,
        });
        return;
    }

    var costs: std.ArrayList(ConsumeHelper.CostItem) = .empty;
    try ConsumeHelper.appendIntMap(&costs, alloc.arena, breach_cfg.BreachConsume);
    if (!(try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, costs.items, io))) {
        txn.respond(.{
            .ErrorCode = .ErrItemNotEnough,
            .RoleId = request.RoleId,
            .Breakthrough = role.breakthrough,
        });
        return;
    }

    if (breach_cfg.BreachReward > 0) {
        _ = try RewardService.grantDropPackage(quest_types.makeRewardContext(qctx), breach_cfg.BreachReward, 1);
    }

    role.breakthrough = target_breach;
    _ = try role.refreshScaledProperties(alloc.gpa, assets, request.RoleId);
    try saveRoleImmediate(fs, alloc.arena, role_comp.player_id, request.RoleId, role.*);

    try events.enqueue(.role_info_modified, .{ .role_id = request.RoleId });
    try events.enqueue(.update_formations, .{});
    try RoleAttributeSync.refreshRole(txn, alloc, fs, scene, assets, role_comp, weapon_comp, echo_comp, stat_query, request.RoleId, io);

    txn.respond(.{
        .ErrorCode = .Success,
        .RoleId = request.RoleId,
        .Breakthrough = role.breakthrough,
    });
}

pub fn onPbGetRoleListRequest(
    txn: *Transaction(pb.PbGetRoleListRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
    basic_comp: *const PlayerBasicComponent,
    role_comp: *PlayerRoleComponent,
) !void {
    const selected_main_role = RoleHelper.selectedMainRoleId(
        assets,
        role_comp,
        basic_comp.info.attributes.sex,
        0,
    );
    var role_list: std.ArrayList(pb.roleInfo) = .empty;
    var iterator = role_comp.role_map.iterator();
    while (iterator.next()) |entry| {
        const rid = entry.key_ptr.*;
        if (rid > 1999 or rid < 1000) continue;
        if (selected_main_role) |selected| {
            if (RoleHelper.isMainRole(assets, rid) and rid != selected) continue;
        }
        try role_list.append(alloc.arena, .{
            .RoleId = rid,
            .Level = entry.value_ptr.level,
            .Exp = entry.value_ptr.exp,
            .Breakthrough = entry.value_ptr.breakthrough,
        });
    }
    txn.respond(.{
        .Code = .Success,
        .RoleList = role_list,
    });
}

pub fn onCharacterDeathRequest(txn: *Transaction(pb.CharacterDeathRequest)) !void {
    _ = txn.message;
    txn.respond(.{});
}
