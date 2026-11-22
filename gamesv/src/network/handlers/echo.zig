const std = @import("std");
const pb = @import("proto").pb;
const Transaction = @import("../handlers.zig").Transaction;
const mem = @import("../../mem.zig");
const FileSystem = @import("common").FileSystem;
const Assets = @import("../../data/Assets.zig");
const Scene = @import("../../logic/Scene.zig");
const PlayerRoleComponent = @import("../../logic/component/player/PlayerRoleComponent.zig");
const PlayerBasicComponent = @import("../../logic/component/player/PlayerBasicComponent.zig");
const PlayerEchoComponent = @import("../../logic/component/player/PlayerEchoComponent.zig");
const PlayerWeaponComponent = @import("../../logic/component/player/PlayerWeaponComponent.zig");
const PlayerCosmeticComponent = @import("../../logic/component/player/PlayerCosmeticComponent.zig");
const CosmeticInfo = @import("../../fs/CosmeticInfo.zig");
const CalabashHelper = @import("../../logic/helpers/calabash.zig");
const Entity = Scene.Entity;
const shared = @import("echo/shared.zig");
const BuffTimerScheduler = @import("../../logic/schedulers/BuffTimerScheduler.zig");
const QuestEngine = @import("../../logic/helpers/quest_engine.zig");

const phantomItemList = shared.phantomItemList;
const equipInfoList = shared.equipInfoList;
const rolePropInfoList = shared.rolePropInfoList;
const changedEquipInfoList = shared.changedEquipInfoList;
const appendRolesWithMainEcho = shared.appendRolesWithMainEcho;
const pushRolePropUpdate = shared.pushRolePropUpdate;
const refreshRoleEntities = shared.refreshRoleEntities;

pub fn onPhantomItemRequest(
    txn: *Transaction(pb.PhantomItemRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
    basic_comp: *PlayerBasicComponent,
    role_comp: *PlayerRoleComponent,
    echo_comp: *PlayerEchoComponent,
    weapon_comp: *PlayerWeaponComponent,
    cosmetic_comp: *PlayerCosmeticComponent,
) !void {
    txn.respond(.{
        .PhantomItemList = try phantomItemList(echo_comp, alloc.arena),
        .EquipInfoList = try equipInfoList(role_comp, echo_comp, alloc.arena),
        .PropInfo = try rolePropInfoList(assets, role_comp, echo_comp, weapon_comp, alloc.gpa, alloc.arena),
        .MaxCost = CalabashHelper.currentCost(assets, basic_comp, echo_comp),
        .PhantomSkinList = try CosmeticInfo.intList(cosmetic_comp.info.phantom_skins, alloc.arena),
        .DirectRefineWeekTimes = echo_comp.calabash_info.direct_refine_week_times,
    });
}

pub fn onPhantomSkinChangeRequest(
    txn: *Transaction(pb.PhantomSkinChangeRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    scene_opt: *?Scene,
    role_comp: *PlayerRoleComponent,
    echo_comp: *PlayerEchoComponent,
    weapon_comp: *PlayerWeaponComponent,
    cosmetic_comp: *PlayerCosmeticComponent,
    buff_timers: *BuffTimerScheduler,
    io: std.Io,
) !void {
    const now_ms = (std.Io.Clock.awake).now(io).toMilliseconds();
    const echo = echo_comp.echo_map.getPtr(txn.message.IncrId) orelse {
        txn.respond(.{ .ErrorCode = .ErrPhantomItemNotExist });
        return;
    };
    const item_config = assets.tables.phantom_item.getDataById(echo.id) orelse {
        txn.respond(.{ .ErrorCode = .ErrPhantomItemNotExist });
        return;
    };

    if (txn.message.SkinId != 0) {
        const skin_config = assets.tables.phantom_item.getDataById(txn.message.SkinId) orelse {
            txn.respond(.{ .ErrorCode = .ErrPhantomSkinNotExist });
            return;
        };
        if (skin_config.ParentMonsterId != item_config.MonsterId or !CosmeticInfo.has(cosmetic_comp.info.phantom_skins, txn.message.SkinId)) {
            txn.respond(.{ .ErrorCode = .ErrPhantomSkinNotUnlock });
            return;
        }
    }

    var changed_roles: std.array_hash_map.Auto(i32, void) = .empty;
    defer changed_roles.deinit(alloc.gpa);
    try appendRolesWithMainEcho(echo_comp, alloc.gpa, txn.message.IncrId, &changed_roles);

    echo.skin_id = txn.message.SkinId;
    if (txn.message.ChangeDefault) {
        for (echo_comp.echo_map.keys(), echo_comp.echo_map.values()) |inc_id, *other| {
            const other_config = assets.tables.phantom_item.getDataById(other.id) orelse continue;
            if (other_config.MonsterId == item_config.MonsterId) {
                other.skin_id = txn.message.SkinId;
                try appendRolesWithMainEcho(echo_comp, alloc.gpa, inc_id, &changed_roles);
            }
        }
    }

    try PlayerEchoComponent.saveAll(alloc.gpa, fs, echo_comp.player_id, echo_comp.echo_map);
    if (changed_roles.count() != 0) {
        const scene = &(scene_opt.* orelse {
            txn.respond(.{ .ErrorCode = .Success });
            return;
        });
        const query: Scene.Query(&.{
            Entity,
            *Entity.ConfigComponent,
            *Entity.VisionSkillComponent,
            *Entity.ConcomitantComponent,
            *Entity.AttributeComponent,
            *Entity.FightBuffComponent,
        }) = .{ .iterator = .{ .entities = &scene.entities } };
        try refreshRoleEntities(txn, alloc, fs, assets, scene, role_comp, echo_comp, weapon_comp, query, changed_roles, buff_timers, io, now_ms);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onPhantomPutOnRequest(
    txn: *Transaction(pb.PhantomPutOnRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    scene: *Scene,
    basic_comp: *PlayerBasicComponent,
    role_comp: *PlayerRoleComponent,
    echo_comp: *PlayerEchoComponent,
    weapon_comp: *PlayerWeaponComponent,
    query: Scene.Query(&.{
        Entity,
        *Entity.ConfigComponent,
        *Entity.VisionSkillComponent,
        *Entity.ConcomitantComponent,
        *Entity.AttributeComponent,
        *Entity.FightBuffComponent,
    }),
    buff_timers: *BuffTimerScheduler,
    qctx: QuestEngine.QuestContext,
    io: std.Io,
) !void {
    const now_ms = (std.Io.Clock.awake).now(io).toMilliseconds();
    if (txn.message.Pos < 0 or txn.message.Pos >= 5) {
        txn.respond(.{ .ErrorCode = .RequestParamError });
        return;
    }
    if (!shared.roleExistsAndEditable(role_comp, txn.message.RoleId)) {
        txn.respond(.{ .ErrorCode = .ErrRoleNotActive });
        return;
    }

    if (txn.message.IncId != 0 and echo_comp.echo_map.get(txn.message.IncId) == null) {
        txn.respond(.{ .ErrorCode = .ErrPhantomItemNotExist });
        return;
    }

    const target_pos = txn.message.Pos;
    const target = echo_comp.equippedEcho(txn.message.RoleId, target_pos);
    const source = if (txn.message.IncId != 0) echo_comp.equippedEchoByIncrId(txn.message.IncId) else null;

    if (source) |src_equip| {
        if (!shared.roleExistsAndEditable(role_comp, src_equip.role_id)) {
            txn.respond(.{ .ErrorCode = .ErrPhantomCannotReplace });
            return;
        }
    }

    if (txn.message.IncId != 0) {
        var next_slots = echo_comp.roleEquip(txn.message.RoleId).slots;
        next_slots[@intCast(target_pos)] = txn.message.IncId;
        if (source) |src_equip| {
            if (src_equip.role_id == txn.message.RoleId) {
                next_slots[@intCast(src_equip.pos)] = if (target) |t| t.inc_id else 0;
            }
        }
        if (shared.validateEquipSlots(assets, basic_comp, echo_comp, next_slots, .ErrPhantomEquipSourceCost)) |err| {
            txn.respond(.{ .ErrorCode = err });
            return;
        }
        if (source) |src_equip| {
            if (src_equip.role_id != txn.message.RoleId) {
                var partner_slots = echo_comp.roleEquip(src_equip.role_id).slots;
                partner_slots[@intCast(src_equip.pos)] = if (target) |t| t.inc_id else 0;
                if (shared.validateEquipSlots(assets, basic_comp, echo_comp, partner_slots, .ErrPhantomEquipTargetCost)) |err| {
                    txn.respond(.{ .ErrorCode = err });
                    return;
                }
            }
        }
    }

    var changed_roles: std.array_hash_map.Auto(i32, void) = .empty;
    defer changed_roles.deinit(alloc.gpa);
    try changed_roles.put(alloc.gpa, txn.message.RoleId, {});

    if (target) |equip| try changed_roles.put(alloc.gpa, equip.role_id, {});
    if (source) |equip| try changed_roles.put(alloc.gpa, equip.role_id, {});

    var split_infos: std.ArrayList(pb.PhantomStackSplitInfo) = .empty;
    if (txn.message.IncId == 0) {
        if (target) |equip| {
            if (echo_comp.echo_map.getPtr(equip.inc_id)) |item| {
                item.role_id = null;
                item.equipped_pos = -1;
            }
        }
    } else {
        if (target) |target_equip| {
            if (target_equip.inc_id != txn.message.IncId) {
                if (echo_comp.echo_map.getPtr(target_equip.inc_id)) |target_item| {
                    if (source) |source_equip| {
                        target_item.role_id = source_equip.role_id;
                        target_item.equipped_pos = source_equip.pos;
                    } else {
                        target_item.role_id = null;
                        target_item.equipped_pos = -1;
                    }
                }
            }
        }
        if (try shared.splitSingleFromStack(alloc.gpa, alloc.arena, fs, echo_comp, txn.message.IncId)) |split| {
            try split_infos.append(alloc.arena, split);
        }
        const item = echo_comp.echo_map.getPtr(txn.message.IncId) orelse {
            txn.respond(.{ .ErrorCode = .ErrPhantomItemNotExist });
            return;
        };
        item.role_id = txn.message.RoleId;
        item.equipped_pos = target_pos;
    }

    try PlayerEchoComponent.saveAll(alloc.gpa, fs, echo_comp.player_id, echo_comp.echo_map);
    try pushRolePropUpdate(txn, alloc, assets, role_comp, echo_comp, weapon_comp, changed_roles, io);
    try refreshRoleEntities(txn, alloc, fs, assets, scene, role_comp, echo_comp, weapon_comp, query, changed_roles, buff_timers, io, now_ms);
    if (txn.message.IncId != 0) {
        QuestEngine.onVisionSystemEvent(qctx, "VisionSetActivated") catch |err| QuestEngine.logHookErr("onVisionSystemEvent", err);
    }

    txn.respond(.{
        .ErrorCode = .Success,
        .EquipInfoList = try changedEquipInfoList(echo_comp, changed_roles, alloc.arena),
        .StackSplitInfos = split_infos,
    });
    try QuestEngine.onConditionStateChanged(qctx);
}

pub fn onPhantomAutoPutRequest(
    txn: *Transaction(pb.PhantomAutoPutRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    scene: *Scene,
    basic_comp: *PlayerBasicComponent,
    role_comp: *PlayerRoleComponent,
    echo_comp: *PlayerEchoComponent,
    weapon_comp: *PlayerWeaponComponent,
    query: Scene.Query(&.{
        Entity,
        *Entity.ConfigComponent,
        *Entity.VisionSkillComponent,
        *Entity.ConcomitantComponent,
        *Entity.AttributeComponent,
        *Entity.FightBuffComponent,
    }),
    buff_timers: *BuffTimerScheduler,
    qctx: QuestEngine.QuestContext,
    io: std.Io,
) !void {
    const now_ms = (std.Io.Clock.awake).now(io).toMilliseconds();
    if (!shared.roleExistsAndEditable(role_comp, txn.message.RoleId)) {
        txn.respond(.{ .ErrorCode = .ErrRoleNotActive });
        return;
    }

    var changed_roles: std.array_hash_map.Auto(i32, void) = .empty;
    defer changed_roles.deinit(alloc.gpa);
    try changed_roles.put(alloc.gpa, txn.message.RoleId, {});

    var requested: [5]i32 = .{ 0, 0, 0, 0, 0 };
    var seen: std.hash_map.AutoHashMapUnmanaged(i32, void) = .empty;
    defer seen.deinit(alloc.gpa);

    for (txn.message.PhantomItemIncrId.items, 0..) |inc_id, index| {
        if (index >= requested.len) break;
        if (inc_id != 0) {
            if (echo_comp.echo_map.get(inc_id) == null or seen.contains(inc_id)) {
                txn.respond(.{ .ErrorCode = .RequestParamError });
                return;
            }
            if (echo_comp.equippedEchoByIncrId(inc_id)) |prev_equip| {
                if (!shared.roleExistsAndEditable(role_comp, prev_equip.role_id)) {
                    txn.respond(.{ .ErrorCode = .ErrPhantomAutoEquipFromOther });
                    return;
                }
            }
            try seen.put(alloc.gpa, inc_id, {});
        }
        requested[index] = inc_id;
    }

    if (shared.validateEquipSlots(assets, basic_comp, echo_comp, requested, .ErrPhantomEquipSourceCost)) |err| {
        txn.respond(.{ .ErrorCode = err });
        return;
    }

    var iterator = echo_comp.echo_map.iterator();
    while (iterator.next()) |kv| {
        const role_id = kv.value_ptr.role_id orelse continue;
        const equipped_pos = kv.value_ptr.equipped_pos;
        if (equipped_pos < 0 or equipped_pos >= 5) continue;
        if (role_id == txn.message.RoleId or seen.contains(kv.key_ptr.*)) {
            try changed_roles.put(alloc.gpa, role_id, {});
            kv.value_ptr.role_id = null;
            kv.value_ptr.equipped_pos = -1;
        }
    }
    var split_infos: std.ArrayList(pb.PhantomStackSplitInfo) = .empty;
    for (requested, 0..) |inc_id, index| {
        if (inc_id == 0) continue;
        if (try shared.splitSingleFromStack(alloc.gpa, alloc.arena, fs, echo_comp, inc_id)) |split| {
            try split_infos.append(alloc.arena, split);
        }
        const item = echo_comp.echo_map.getPtr(inc_id) orelse continue;
        item.role_id = txn.message.RoleId;
        item.equipped_pos = @intCast(index);
    }

    try PlayerEchoComponent.saveAll(alloc.gpa, fs, echo_comp.player_id, echo_comp.echo_map);
    try pushRolePropUpdate(txn, alloc, assets, role_comp, echo_comp, weapon_comp, changed_roles, io);
    try refreshRoleEntities(txn, alloc, fs, assets, scene, role_comp, echo_comp, weapon_comp, query, changed_roles, buff_timers, io, now_ms);
    if (txn.message.PhantomItemIncrId.items.len > 0) {
        QuestEngine.onVisionSystemEvent(qctx, "VisionSetActivated") catch |err| QuestEngine.logHookErr("onVisionSystemEvent", err);
    }

    txn.respond(.{
        .ErrorCode = .Success,
        .EquipInfoList = try changedEquipInfoList(echo_comp, changed_roles, alloc.arena),
        .StackSplitInfos = split_infos,
    });
    try QuestEngine.onConditionStateChanged(qctx);
}
