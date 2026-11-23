const std = @import("std");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const dispatch = @import("combat.zig");
const Scene = @import("../../logic/Scene.zig");
const Connection = @import("../../network/Connection.zig");
const Entity = @import("../../logic/Scene.zig").Entity;
const Assets = @import("../../data/Assets.zig");
const FileSystem = @import("common").FileSystem;
const EventQueue = @import("../../logic/EventQueue.zig");
const Transaction = @import("../handlers.zig").Transaction;
const PlayerBasicComponent = @import("../../logic/component/player/PlayerBasicComponent.zig");
const PlayerInventoryComponent = @import("../../logic/component/player/PlayerInventoryComponent.zig");
const PlayerWeaponComponent = @import("../../logic/component/player/PlayerWeaponComponent.zig");
const PlayerEchoComponent = @import("../../logic/component/player/PlayerEchoComponent.zig");
const PlayerMapComponent = @import("../../logic/component/player/PlayerMapComponent.zig");
const PlayerQuestComponent = @import("../../logic/component/player/PlayerQuestComponent.zig");
const PlayerRoleComponent = @import("../../logic/component/player/PlayerRoleComponent.zig");
const damage_helper = @import("../../logic/helpers/damage.zig");
const attributes_helper = @import("../../logic/helpers/attributes.zig");
const buff_helper = @import("../../logic/helpers/buff.zig");
const RoleHelper = @import("../../logic/helpers/role.zig");
const BuffTimerScheduler = @import("../../logic/schedulers/BuffTimerScheduler.zig");
const QuestEngine = @import("../../logic/helpers/quest_engine.zig");
const LevelPlayHelper = @import("../../logic/helpers/level_play.zig");

pub fn DamageExecuteRequest(
    txn: *dispatch.CombatRequestTxn(.DamageExecuteRequest),
    assets: *const Assets,
    scene: *Scene,
    fs: *FileSystem,
    events: *EventQueue,
    buff_timers: *BuffTimerScheduler,
    qctx: QuestEngine.QuestContext,
    role_comp: *PlayerRoleComponent,
    query: Scene.Query(&.{
        Entity,
        *Entity.FightBuffComponent,
        *Entity.AttributeComponent,
    }),
    io: std.Io,
    alloc: mem.Alloc,
) !void {
    const request: pb.DamageExecuteRequest = txn.payload;
    const damage = assets.tables.damage.getDataById(request.DamageId) orelse {
        txn.respond(.{ .ErrorCode = .ErrConfDamageNotFound });
        return;
    };

    var combat_receive_pack: std.ArrayList(pb.CombatReceiveData) = .empty;

    const execute_data = try damage_helper.damageEntity(
        &combat_receive_pack,
        request,
        &damage,
        assets,
        scene,
        buff_timers,
        fs,
        io,
        query,
        alloc,
    );
    try txn.receive_data_pack.appendSlice(alloc.arena, combat_receive_pack.items);

    const target_idx = scene.net_id_map.get(execute_data.TargetEntityId);
    var target_is_player = false;
    if (target_idx) |idx| {
        const cfg = scene.entities.items(.config)[idx];
        target_is_player = cfg.entity_type == .player;
        if (target_is_player and cfg.config_type == .character) {
            if (scene.entities.items(.attribute)[idx]) |attr| {
                const life_idx = @intFromEnum(pb.EAttributeType.Life);
                const cur_hp = @max(0, attr.attributes[life_idx].current);
                if (role_comp.role_map.getPtr(cfg.config_id)) |role_info| {
                    if (role_info.current_hp == null or role_info.current_hp.? != cur_hp) {
                        role_info.current_hp = cur_hp;
                        events.enqueueBestEffort(.role_info_modified, .{ .role_id = cfg.config_id });
                        if (execute_data.KilledTarget) {
                            events.enqueueBestEffort(.update_formations, .{});
                            RoleHelper.saveRoleInfo(fs, alloc.arena, role_comp, cfg.config_id) catch |err| {
                                std.log.scoped(.damage).warn("saveRoleInfo({d}) failed: {t}", .{ cfg.config_id, err });
                            };
                            RoleHelper.maybePushPartyWipeNotify(qctx.conn, scene, alloc, io);
                        }
                    }
                }
            }
        }
    }

    if (execute_data.KilledTarget and !target_is_player) {
        QuestEngine.onEntityKilled(qctx, execute_data.TargetEntityId, null) catch |err| QuestEngine.logHookErr("onEntityKilled", err);
        if (LevelPlayHelper.fromQuestContext(qctx)) |lp_ctx| {
            LevelPlayHelper.onEntityKilledWithConditions(lp_ctx, qctx.progress_comp, execute_data.TargetEntityId, null) catch |err| {
                std.log.scoped(.level_play).warn("onEntityKilled failed: {t}", .{err});
            };
        }
    } else if (!execute_data.KilledTarget and !target_is_player) {
        if (target_idx) |idx| {
            const cfg_id = scene.entities.items(.config)[idx].config_id;
            if (scene.entities.items(.attribute)[idx]) |attr| {
                const life_idx = @intFromEnum(pb.EAttributeType.Life);
                const max_idx = @intFromEnum(pb.EAttributeType.LifeMax);
                const cur_hp: f32 = @floatFromInt(@max(0, attr.attributes[life_idx].current));
                const max_hp_i = @max(1, @max(attr.attributes[max_idx].current, attr.attributes[max_idx].base));
                const ratio = cur_hp / @as(f32, @floatFromInt(max_hp_i));
                QuestEngine.onEntityDamaged(qctx, cfg_id, ratio) catch |err| QuestEngine.logHookErr("onEntityDamaged", err);
            }
        }
    }

    txn.respond(.{
        .ErrorCode = .Success,
        .AttackerEntityId = execute_data.AttackerEntityId,
        .TargetEntityId = execute_data.TargetEntityId,
        .Damage = execute_data.Damage,
        .PartIndex = execute_data.PartIndex,
        .IsCrit = execute_data.IsCrit,
        .KilledTarget = execute_data.KilledTarget,
        .ShieldCoverDamage = execute_data.ShieldCoverDamage,
        .ElementType = execute_data.ElementType,
        .ChangeLife = execute_data.ChangeLife,
    });

    if (execute_data.KilledTarget and !target_is_player) {
        queueKilledMonsterRemoval(scene, alloc.gpa, execute_data.TargetEntityId) catch |err| {
            std.log.scoped(.damage).warn("queueKilledMonsterRemoval({d}) failed: {t}", .{ execute_data.TargetEntityId, err });
        };
    }
}

/// Queues a killed monster for removal after `CombatSendPackRequest` responds
/// with `ReceivePackNotify`, so the client processes `EntityLivingStatusNotify(.Dead)`
/// before `EntityRemoveNotify(.HpIsZero)` moves it to `DelayRemoveContainer`.
fn queueKilledMonsterRemoval(
    scene: *Scene,
    gpa: std.mem.Allocator,
    eid: i64,
) !void {
    const idx = scene.net_id_map.get(eid) orelse return;
    if (scene.entities.items(.config)[idx].entity_type != .monster) return;
    for (scene.pending_killed_removals.items) |existing| {
        if (existing == eid) return;
    }
    try scene.pending_killed_removals.append(gpa, eid);
}

pub fn HitEndPush(_: pb.HitEndPush) !void {}

pub fn FragileChangeRequest(
    txn: *dispatch.CombatRequestTxn(.FragileChangeRequest),
    scene: *Scene,
    fs: *FileSystem,
    io: std.Io,
    alloc: mem.Alloc,
) !void {
    const eid = if (txn.payload.EntityId != 0)
        txn.payload.EntityId
    else if (txn.common) |c|
        c.EntityId
    else
        0;
    if (eid != 0) {
        if (scene.net_id_map.get(eid)) |idx| {
            if (scene.entities.items(.attribute)[idx]) |*attr| {
                const target_val: i32 = if (txn.payload.Flag)
                    0
                else
                    @max(0, attributes_helper.get_attr_value(attr, .ToughMax));
                const cur_tough = attributes_helper.get_attr_value(attr, .Tough);
                if ((txn.payload.Flag and cur_tough > 0) or (!txn.payload.Flag and cur_tough <= 0 and target_val > 0)) {
                    const ch = try attributes_helper.change_attr(attr, .Tough, .Override, .Current, target_val, alloc);
                    if (ch.items.len != 0) {
                        try attributes_helper.generate_attr_messages(txn.receive_data_pack, eid, attr, &ch, alloc, io);
                        try scene.markFsmDirty(alloc.gpa, eid, Scene.Entity.FsmComponent.WakeReason.attribute);
                        try scene.saveComponents(fs, alloc.gpa, .{ .net_id = eid, .index = idx }, &.{Scene.Entity.AttributeComponent});
                    }
                }
            }
        }
    }
    txn.respond(.{ .ErrorCode = .Success });
}

fn activePlayerRoleEntityId(scene: *const Scene, common: ?pb.CombatCommon) ?i64 {
    if (common) |c| {
        if (c.EntityId != 0) return c.EntityId;
    }
    if (scene.formation_info.cur_formation < 0) return null;
    const f_idx: usize = @intCast(scene.formation_info.cur_formation);
    if (f_idx >= scene.formation_info.formations.len) return null;
    const cur_role = scene.formation_info.formations[f_idx].cur_role;
    for (scene.formation_info.formations[f_idx].roles) |maybe_slot| {
        const slot = maybe_slot orelse continue;
        if (slot.role_id == cur_role and slot.entity_id != 0) return slot.entity_id;
    }
    return null;
}

fn persistRoleHp(
    scene: *const Scene,
    entity_id: i64,
    role_comp: *PlayerRoleComponent,
    events: *EventQueue,
    fs: *FileSystem,
    alloc: mem.Alloc,
    immediate_save: bool,
) void {
    const idx = scene.net_id_map.get(entity_id) orelse return;
    const cfg = scene.entities.items(.config)[idx];
    if (cfg.entity_type != .player or cfg.config_type != .character) return;
    const attr = scene.entities.items(.attribute)[idx] orelse return;
    const cur_hp = @max(0, attributes_helper.get_attr_value(&attr, .Life));
    if (role_comp.role_map.getPtr(cfg.config_id)) |role_info| {
        role_info.current_hp = cur_hp;
        events.enqueueBestEffort(.role_info_modified, .{ .role_id = cfg.config_id });
        if (immediate_save) {
            RoleHelper.saveRoleInfo(fs, alloc.arena, role_comp, cfg.config_id) catch |err| {
                std.log.scoped(.damage).warn("saveRoleInfo({d}) failed: {t}", .{ cfg.config_id, err });
            };
        }
    }
}

fn handleDrownImpl(
    common: ?pb.CombatCommon,
    assets: *const Assets,
    scene: *Scene,
    conn: *Connection,
    buff_timers: *BuffTimerScheduler,
    role_comp: *PlayerRoleComponent,
    events: *EventQueue,
    fs: *FileSystem,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
    io: std.Io,
    alloc: mem.Alloc,
) !void {
    const eid = activePlayerRoleEntityId(scene, common) orelse return;
    const idx = scene.net_id_map.get(eid) orelse return;
    const slice = scene.entities.slice();
    const attr = if (slice.items(.attribute)[idx]) |*a| a else return;
    const max_hp = @max(1, attributes_helper.get_attr_value(attr, .LifeMax));
    const cur_hp = attributes_helper.get_attr_value(attr, .Life);
    if (cur_hp <= 0) return;

    const drown_dmg: i32 = @max(1, @divTrunc(max_hp, 10));
    const change = try attributes_helper.change_attr(attr, .Life, .Delta, .Current, -drown_dmg, alloc);
    try attributes_helper.generate_attr_messages(receive_data_pack, eid, attr, &change, alloc, io);
    try scene.markFsmDirty(alloc.gpa, eid, Entity.FsmComponent.WakeReason.attribute);

    const after_hp = attributes_helper.get_attr_value(attr, .Life);
    if (after_hp <= 0) {
        if (slice.items(.buffs)[idx]) |*buffs| {
            const removed = try buff_helper.handleEntityDeath(receive_data_pack, scene, assets, eid, buffs, attr, alloc, io);
            for (removed) |hid| buff_timers.cancelHandle(eid, hid);
        } else {
            try scene.setBattleEntityActive(alloc.gpa, eid, false);
            try receive_data_pack.append(alloc.arena, .{ .Message = .{ .CombatNotifyData = .{
                .CombatCommon = .{ .EntityId = eid },
                .Message = .{ .EntityLivingStatusNotify = .{ .Id = eid, .LivingStatus = .Dead } },
            } } });
        }
        _ = try scene.appendBattleStateNotify(alloc.arena, receive_data_pack);
        persistRoleHp(scene, eid, role_comp, events, fs, alloc, true);
        events.enqueueBestEffort(.update_formations, .{});
        RoleHelper.maybePushPartyWipeNotify(conn, scene, alloc, io);
    } else {
        persistRoleHp(scene, eid, role_comp, events, fs, alloc, false);
    }

    try receive_data_pack.append(alloc.arena, .{ .Message = .{ .CombatNotifyData = .{
        .CombatCommon = .{ .EntityId = eid },
        .Message = .{ .DrownNotify = .{} },
    } } });
}

fn killEntityDirect(
    eid: i64,
    assets: *const Assets,
    scene: *Scene,
    fs: *FileSystem,
    events: *EventQueue,
    buff_timers: *BuffTimerScheduler,
    qctx: QuestEngine.QuestContext,
    role_comp: *PlayerRoleComponent,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
    io: std.Io,
    alloc: mem.Alloc,
) !void {
    const idx = scene.net_id_map.get(eid) orelse return;
    const slice = scene.entities.slice();
    const cfg = slice.items(.config)[idx];
    const maybe_attr = if (slice.items(.attribute)[idx]) |*a| a else null;

    if (maybe_attr) |attr| {
        const before_hp = attributes_helper.get_attr_value(attr, .Life);
        if (before_hp > 0) {
            _ = attributes_helper.set_life_unconditional(attr, 0);
            var changed: std.ArrayList(pb.EAttributeType) = .empty;
            try changed.append(alloc.arena, .Life);
            try attributes_helper.generate_attr_messages(receive_data_pack, eid, attr, &changed, alloc, io);
            try scene.markFsmDirty(alloc.gpa, eid, Entity.FsmComponent.WakeReason.attribute);
        }
    }

    if (slice.items(.buffs)[idx]) |*buffs| {
        const removed = try buff_helper.handleEntityDeath(receive_data_pack, scene, assets, eid, buffs, maybe_attr, alloc, io);
        for (removed) |hid| buff_timers.cancelHandle(eid, hid);
    } else {
        try scene.setBattleEntityActive(alloc.gpa, eid, false);
        try receive_data_pack.append(alloc.arena, .{ .Message = .{ .CombatNotifyData = .{
            .CombatCommon = .{ .EntityId = eid },
            .Message = .{ .EntityLivingStatusNotify = .{ .Id = eid, .LivingStatus = .Dead } },
        } } });
    }
    _ = try scene.appendBattleStateNotify(alloc.arena, receive_data_pack);

    if (cfg.entity_type == .player and cfg.config_type == .character) {
        persistRoleHp(scene, eid, role_comp, events, fs, alloc, true);
        events.enqueueBestEffort(.update_formations, .{});
        RoleHelper.maybePushPartyWipeNotify(qctx.conn, scene, alloc, io);
        QuestEngine.onConditionStateChanged(qctx) catch |err| QuestEngine.logHookErr("killEntityDirectPlayer", err);
    } else if (cfg.entity_type != .player) {
        QuestEngine.onEntityKilled(qctx, eid, null) catch |err| QuestEngine.logHookErr("onEntityKilled", err);
        if (LevelPlayHelper.fromQuestContext(qctx)) |lp_ctx| {
            LevelPlayHelper.onEntityKilledWithConditions(lp_ctx, qctx.progress_comp, eid, null) catch |err| {
                std.log.scoped(.level_play).warn("onEntityKilled failed: {t}", .{err});
            };
        }
        queueKilledMonsterRemoval(scene, alloc.gpa, eid) catch |err| {
            std.log.scoped(.damage).warn("queueKilledMonsterRemoval({d}) failed: {t}", .{ eid, err });
        };
    }
}

pub fn DrownRequest(
    txn: *dispatch.CombatRequestTxn(.DrownRequest),
    assets: *const Assets,
    scene: *Scene,
    conn: *Connection,
    buff_timers: *BuffTimerScheduler,
    role_comp: *PlayerRoleComponent,
    events: *EventQueue,
    fs: *FileSystem,
    io: std.Io,
    alloc: mem.Alloc,
) !void {
    try handleDrownImpl(txn.common, assets, scene, conn, buff_timers, role_comp, events, fs, txn.receive_data_pack, io, alloc);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn DrownPush(
    _: pb.DrownPush,
    common: ?pb.CombatCommon,
    assets: *const Assets,
    scene: *Scene,
    conn: *Connection,
    buff_timers: *BuffTimerScheduler,
    role_comp: *PlayerRoleComponent,
    events: *EventQueue,
    fs: *FileSystem,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
    io: std.Io,
    alloc: mem.Alloc,
) !void {
    try handleDrownImpl(common, assets, scene, conn, buff_timers, role_comp, events, fs, receive_data_pack, io, alloc);
}

pub fn MonsterDrownRequest(
    txn: *dispatch.CombatRequestTxn(.MonsterDrownRequest),
    assets: *const Assets,
    scene: *Scene,
    fs: *FileSystem,
    events: *EventQueue,
    buff_timers: *BuffTimerScheduler,
    qctx: QuestEngine.QuestContext,
    role_comp: *PlayerRoleComponent,
    io: std.Io,
    alloc: mem.Alloc,
) !void {
    if (txn.common) |c| {
        if (c.EntityId != 0) {
            try killEntityDirect(c.EntityId, assets, scene, fs, events, buff_timers, qctx, role_comp, txn.receive_data_pack, io, alloc);
        }
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn MonsterDrownPush(
    _: pb.MonsterDrownPush,
    common: ?pb.CombatCommon,
    assets: *const Assets,
    scene: *Scene,
    fs: *FileSystem,
    events: *EventQueue,
    buff_timers: *BuffTimerScheduler,
    qctx: QuestEngine.QuestContext,
    role_comp: *PlayerRoleComponent,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
    io: std.Io,
    alloc: mem.Alloc,
) !void {
    const c = common orelse return;
    if (c.EntityId == 0) return;
    try killEntityDirect(c.EntityId, assets, scene, fs, events, buff_timers, qctx, role_comp, receive_data_pack, io, alloc);
}

pub fn MonsterBoomRequest(
    txn: *dispatch.CombatRequestTxn(.MonsterBoomRequest),
    assets: *const Assets,
    scene: *Scene,
    fs: *FileSystem,
    events: *EventQueue,
    buff_timers: *BuffTimerScheduler,
    qctx: QuestEngine.QuestContext,
    role_comp: *PlayerRoleComponent,
    io: std.Io,
    alloc: mem.Alloc,
) !void {
    if (txn.common) |c| {
        if (c.EntityId != 0) {
            try killEntityDirect(c.EntityId, assets, scene, fs, events, buff_timers, qctx, role_comp, txn.receive_data_pack, io, alloc);
        }
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn MonsterBoomPush(
    _: pb.MonsterBoomPush,
    common: ?pb.CombatCommon,
    assets: *const Assets,
    scene: *Scene,
    fs: *FileSystem,
    events: *EventQueue,
    buff_timers: *BuffTimerScheduler,
    qctx: QuestEngine.QuestContext,
    role_comp: *PlayerRoleComponent,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
    io: std.Io,
    alloc: mem.Alloc,
) !void {
    const c = common orelse return;
    if (c.EntityId == 0) return;
    try killEntityDirect(c.EntityId, assets, scene, fs, events, buff_timers, qctx, role_comp, receive_data_pack, io, alloc);
}

pub fn RoleGoDownPush(
    _: pb.RoleGoDownPush,
    common: ?pb.CombatCommon,
    scene: *Scene,
    conn: *Connection,
    io: std.Io,
    alloc: mem.Alloc,
    qctx: QuestEngine.QuestContext,
) !void {
    const eid = activePlayerRoleEntityId(scene, common) orelse return;
    try conn.push(alloc.gpa, io, pb.RoleGoDownNotify{ .EntityId = eid });
    QuestEngine.onConditionStateChanged(qctx) catch |err| QuestEngine.logHookErr("RoleGoDown", err);
}
