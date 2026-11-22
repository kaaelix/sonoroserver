const std = @import("std");
const pb = @import("proto").pb;
const Transaction = @import("../handlers.zig").Transaction;
const FileSystem = @import("common").FileSystem;
const Assets = @import("../../data/Assets.zig");
const Scene = @import("../../logic/Scene.zig");
const mem = @import("../../mem.zig");
const PlayerWeaponComponent = @import("../../logic/component/player/PlayerWeaponComponent.zig");
const PlayerRoleComponent = @import("../../logic/component/player/PlayerRoleComponent.zig");
const PlayerEchoComponent = @import("../../logic/component/player/PlayerEchoComponent.zig");
const PlayerInventoryComponent = @import("../../logic/component/player/PlayerInventoryComponent.zig");
const PlayerBasicComponent = @import("../../logic/component/player/PlayerBasicComponent.zig");
const InventoryInfo = @import("../../fs/InventoryInfo.zig");
const FightBuffComponent = @import("../../logic/component/entity/FightBuffComponent.zig");
const WeaponItem = @import("../../fs/WeaponItem.zig");
const comp_util = @import("../../logic/component/comp_util.zig");
const EventQueue = @import("../../logic/EventQueue.zig");
const RoleAttributeSync = @import("../helpers/role_attribute_sync.zig");
const BuffTimerScheduler = @import("../../logic/schedulers/BuffTimerScheduler.zig");
const buff_helper = @import("../../logic/helpers/buff.zig");
const ConsumeHelper = @import("../../logic/helpers/consume.zig");
const RewardService = @import("../../logic/helpers/reward.zig");
const ConditionEvaluator = @import("../../logic/helpers/condition.zig");
const QuestEngine = @import("../../logic/helpers/quest_engine.zig");
const quest_types = @import("../../logic/helpers/quest_types.zig");

fn saveWeaponImmediate(
    fs: *FileSystem,
    arena: std.mem.Allocator,
    player_id: i32,
    incr_id: i32,
    weapon: WeaponItem,
) !void {
    const path = try std.fmt.allocPrint(arena, "player/{}/{s}/{}", .{ player_id, WeaponItem.data_dir, incr_id });
    try comp_util.saveStruct(fs, weapon, path, arena);
}

fn findWeaponBreachEntry(
    assets: *const Assets,
    breach_id: i32,
    breach_level: i32,
) ?Assets.DataTables.WeaponBreach {
    for (assets.tables.weapon_breach.items) |entry| {
        if (entry.BreachId == breach_id and entry.Level == breach_level) return entry;
    }
    return null;
}

fn appendEquipTakeOnData(
    list: *std.ArrayList(pb.RoleLoadEquipData),
    arena: std.mem.Allocator,
    role_id: i32,
    equip_inc_id: i32,
    pos: pb.EquipPos,
) !void {
    try list.append(arena, .{
        .RoleId = role_id,
        .Pos = pos,
        .EquipIncId = equip_inc_id,
    });
}

fn syncWeaponPassiveBuffs(
    txn: anytype,
    alloc: mem.Alloc,
    scene: *Scene,
    assets: *const Assets,
    fs: *FileSystem,
    buff_timers: *BuffTimerScheduler,
    io: std.Io,
    now_ms: i64,
    entity: Scene.Entity,
    buffs: *FightBuffComponent,
    remove_ids: []const i64,
    add_ids: []const i64,
) !bool {
    try buff_timers.ensureEntityRegistered(
        alloc.gpa,
        scene,
        assets,
        entity.net_id,
        now_ms,
    );
    var notify: pb.CombatReceivePackNotify = .{};

    for (remove_ids) |buff_id| {
        const buff = buffs.getByBuffId(buff_id) orelse continue;
        const handle_id = buff.HandleId;
        const stack_count = buff.StackCount;
        buff_timers.cancelHandle(entity.net_id, handle_id);
        buffs.removeByHandleId(alloc.gpa, handle_id);
        try notify.Data.append(alloc.arena, .{ .Message = .{
            .CombatNotifyData = .{
                .CombatCommon = .{ .EntityId = entity.net_id },
                .Message = .{
                    .RemoveBuffS2cRequestNotify = .{
                        .Handle = handle_id,
                        .StackCount = stack_count,
                        .Reason = 41,
                        .InstigatorId = entity.net_id,
                    },
                },
            },
        } });
    }

    for (add_ids) |buff_id| {
        if (buffs.getByBuffId(buff_id) != null) continue;
        const buff_data = assets.tables.buff.getDataById(buff_id) orelse continue;
        scene.instance.buff_handle += 1;
        buffs.fight_buff_infos = try alloc.gpa.realloc(buffs.fight_buff_infos, buffs.fight_buff_infos.len + 1);
        const buff_info = Assets.DataTables.createBuffInformation(scene.instance.buff_handle, buff_id, entity.net_id, entity.net_id, true);
        buffs.fight_buff_infos[buffs.fight_buff_infos.len - 1] = buff_info;
        try buff_timers.scheduleNewBuff(alloc.gpa, buff_info, &buff_data, entity.net_id, now_ms);
        try notify.Data.append(alloc.arena, .{ .Message = .{
            .CombatNotifyData = .{
                .CombatCommon = .{ .EntityId = entity.net_id, .MessageId = -1, .PreMessageId = -1 },
                .Message = .{
                    .ApplyBuffS2cRequestNotify = .{
                        .Id = buff_id,
                        .Level = buff_info.Level,
                        .InstigatorId = entity.net_id,
                        .ApplyType = buff_info.ApplyType orelse .Common,
                        .ServerId = buff_info.ServerId,
                        .StackCount = buff_info.StackCount,
                        .IsIterable = true,
                        .Reason = 12,
                    },
                },
            },
        } });
        const disposition = BuffTimerScheduler.applicationDisposition(&buff_data, true, false, true);
        if (disposition.execute_periodic_now) {
            try scene.saveComponents(fs, alloc.gpa, entity, &.{Scene.Entity.FightBuffComponent});
            const effect_query: Scene.Query(&.{
                Scene.Entity,
                *Scene.Entity.FightBuffComponent,
                ?*Scene.Entity.AttributeComponent,
            }) = .{ .iterator = .{ .entities = &scene.entities } };
            try buff_helper.execute_periodic_buff_effects(
                &notify.Data,
                entity.net_id,
                entity.net_id,
                &buff_data,
                scene,
                fs,
                io,
                effect_query,
                alloc,
            );
        }
    }

    if (notify.Data.items.len == 0) return false;
    try txn.conn.push(alloc.gpa, io, notify);
    try scene.markFsmDirty(alloc.gpa, entity.net_id, Scene.Entity.FsmComponent.WakeReason.buff);
    return true;
}

pub fn onWeaponItemRequest(
    txn: *Transaction(pb.WeaponItemRequest),
    alloc: mem.Alloc,
    weapon_comp: *PlayerWeaponComponent,
) !void {
    var list: std.ArrayList(pb.WeaponItem) = .empty;
    try list.ensureTotalCapacity(alloc.arena, weapon_comp.weapon_map.count());

    var iterator = weapon_comp.weapon_map.iterator();
    while (iterator.next()) |kv| {
        list.appendAssumeCapacity(kv.value_ptr.toProto(kv.key_ptr.*));
    }

    txn.respond(.{ .WeaponItemList = list });
}

fn weaponLevelExp(assets: *const Assets, level_id: i32, level: i32) i32 {
    for (assets.tables.weapon_level.items) |entry| {
        if (entry.LevelId == level_id and entry.Level == level) return entry.Exp;
    }
    return 0;
}

fn weaponLevelLimit(assets: *const Assets, breach_id: i32, breach: i32) i32 {
    for (assets.tables.weapon_breach.items) |entry| {
        if (entry.BreachId == breach_id and entry.Level == breach) return entry.LevelLimit;
    }
    return 90;
}

fn weaponExpItemValue(assets: *const Assets, item_id: i32) i32 {
    const item = assets.tables.weapon_exp_item.getDataById(item_id) orelse return 0;
    return item.BasicExp;
}

fn weaponQualityBaseExp(quality: i32) i32 {
    return switch (quality) {
        1 => 100,
        2 => 800,
        3 => 1800,
        4 => 21000,
        5 => 90000,
        else => 800,
    };
}

fn totalWeaponInvestedExp(assets: *const Assets, level_id: i32, weapon: WeaponItem) i64 {
    var total: i64 = @max(0, weapon.exp);
    var lv: i32 = 1;
    while (lv < weapon.level) : (lv += 1) {
        total += @max(0, weaponLevelExp(assets, level_id, lv));
    }
    return total;
}

pub fn onWeaponLevelUpRequest(
    txn: *Transaction(pb.WeaponLevelUpRequest),
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
    const weapon = weapon_comp.weapon_map.getPtr(request.IncId) orelse {
        txn.respond(.{ .ErrorCode = .ErrWeaponConsumeItemNotFound });
        return;
    };
    const config = assets.tables.weapon_conf.getDataById(weapon.id) orelse {
        txn.respond(.{ .ErrorCode = .ErrWeaponResonConfigNotFound });
        return;
    };

    const limit = weaponLevelLimit(assets, config.BreachId, weapon.breach);
    if (weapon.level >= limit) {
        txn.respond(.{
            .ErrorCode = .ErrWeaponLevelLimit,
            .IncId = request.IncId,
            .WeaponLevel = weapon.level,
            .WeaponExp = weapon.exp,
        });
        return;
    }

    var gained_exp_i64: i64 = 0;
    var item_exp_i64: i64 = 0;
    var costs: std.ArrayList(ConsumeHelper.CostItem) = .empty;
    var seen_fodder: std.AutoHashMapUnmanaged(i32, void) = .empty;
    defer seen_fodder.deinit(alloc.arena);
    var fodder_ids: std.ArrayList(i32) = .empty;

    for (request.ConsumeList.items) |item| {
        if (item.IncId > 0) {
            if (item.Count != 1 or item.IncId == request.IncId) {
                txn.respond(.{ .ErrorCode = .ErrWeaponConsumeInvalid, .IncId = request.IncId, .WeaponLevel = weapon.level, .WeaponExp = weapon.exp });
                return;
            }
            if ((try seen_fodder.getOrPut(alloc.arena, item.IncId)).found_existing) {
                txn.respond(.{ .ErrorCode = .ErrWeaponConsumeInvalid, .IncId = request.IncId, .WeaponLevel = weapon.level, .WeaponExp = weapon.exp });
                return;
            }
            const fodder = weapon_comp.weapon_map.get(item.IncId) orelse {
                txn.respond(.{ .ErrorCode = .ErrWeaponConsumeItemNotFound, .IncId = request.IncId, .WeaponLevel = weapon.level, .WeaponExp = weapon.exp });
                return;
            };
            if (fodder.role_id != null or fodder.func_value == 1) {
                txn.respond(.{ .ErrorCode = .ErrWeaponConsumeInvalid, .IncId = request.IncId, .WeaponLevel = weapon.level, .WeaponExp = weapon.exp });
                return;
            }
            const fodder_cfg = assets.tables.weapon_conf.getDataById(fodder.id) orelse {
                txn.respond(.{ .ErrorCode = .ErrWeaponResonConfigNotFound });
                return;
            };
            const base_exp: i64 = weaponQualityBaseExp(fodder_cfg.QualityId);
            const invested: i64 = @divTrunc(totalWeaponInvestedExp(assets, fodder_cfg.LevelId, fodder) * 90, 100);
            gained_exp_i64 += base_exp + invested;
            try fodder_ids.append(alloc.arena, item.IncId);
        } else {
            if (item.ItemId == 0 or item.Count <= 0) continue;
            const exp = weaponExpItemValue(assets, item.ItemId);
            if (exp == 0) continue;
            const add = std.math.mul(i64, exp, item.Count) catch {
                txn.respond(.{ .ErrorCode = .ErrWeaponConsumeInvalid, .IncId = request.IncId, .WeaponLevel = weapon.level, .WeaponExp = weapon.exp });
                return;
            };
            item_exp_i64 += add;
            gained_exp_i64 += add;
            try costs.append(alloc.arena, .{ .id = item.ItemId, .count = item.Count });
        }
    }

    if (gained_exp_i64 <= 0 or gained_exp_i64 > std.math.maxInt(i32)) {
        txn.respond(.{
            .ErrorCode = .ErrWeaponLevelUpNoExp,
            .IncId = request.IncId,
            .WeaponLevel = weapon.level,
            .WeaponExp = weapon.exp,
        });
        return;
    }
    const gained_exp: i32 = @intCast(gained_exp_i64);

    if (item_exp_i64 > 0) {
        const coin_cost_i64 = @max(@as(i64, 1), @divTrunc(item_exp_i64 * 40, 100));
        if (coin_cost_i64 > std.math.maxInt(i32)) {
            txn.respond(.{ .ErrorCode = .ErrWeaponConsumeInvalid, .IncId = request.IncId, .WeaponLevel = weapon.level, .WeaponExp = weapon.exp });
            return;
        }
        try costs.append(alloc.arena, .{ .id = 2, .count = @intCast(coin_cost_i64) });
    }
    if (costs.items.len > 0 and !(try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, costs.items, io))) {
        txn.respond(.{
            .ErrorCode = .ErrItemNotEnough,
            .IncId = request.IncId,
            .WeaponLevel = weapon.level,
            .WeaponExp = weapon.exp,
        });
        return;
    }

    if (fodder_ids.items.len > 0) {
        for (fodder_ids.items) |fid| {
            if (weapon_comp.weapon_map.fetchOrderedRemove(fid)) |removed| {
                var old = removed.value;
                old.deinit(alloc.gpa);
            }
            try PlayerWeaponComponent.deleteWeaponFile(alloc.gpa, fs, weapon_comp.player_id, fid);
        }
        try txn.conn.push(alloc.gpa, io, pb.WeaponItemRemoveNotify{ .WeaponItemIncrIdList = fodder_ids });
    }

    var total_exp: i64 = @as(i64, weapon.exp) + gained_exp;
    while (weapon.level < limit) {
        const needed_exp = weaponLevelExp(assets, config.LevelId, weapon.level);
        if (needed_exp <= 0 or total_exp < needed_exp) break;
        total_exp -= needed_exp;
        weapon.level += 1;
    }

    var item_map: std.ArrayList(pb.MapEntry(i32, i32)) = .empty;
    if (weapon.level >= limit) {
        var excess: i64 = total_exp;
        weapon.exp = 0;
        const refund_pots = [_]struct { id: i32, exp: i64 }{
            .{ .id = 43020004, .exp = 20000 },
            .{ .id = 43020003, .exp = 8000 },
            .{ .id = 43020002, .exp = 3000 },
            .{ .id = 43020001, .exp = 1000 },
        };
        var refunds: std.ArrayList(RewardService.RewardItem) = .empty;
        for (refund_pots) |pot| {
            if (excess >= pot.exp) {
                const cnt: i32 = @intCast(@min(@divTrunc(excess, pot.exp), @as(i64, std.math.maxInt(i32))));
                if (cnt > 0) {
                    excess -= @as(i64, cnt) * pot.exp;
                    try refunds.append(alloc.arena, .{ .item_id = pot.id, .count = cnt });
                    try item_map.append(alloc.arena, .{ .key = pot.id, .value = cnt });
                }
            }
        }
        if (refunds.items.len > 0) {
            _ = try RewardService.grantItems(.{
                .alloc = alloc,
                .io = io,
                .fs = fs,
                .assets = assets,
                .conn = txn.conn,
                .basic_comp = basic_comp,
                .inventory_comp = inventory_comp,
                .weapon_comp = weapon_comp,
                .echo_comp = echo_comp,
            }, 0, false, refunds.items, 0);
        }
    } else {
        weapon.exp = @intCast(total_exp);
    }

    try saveWeaponImmediate(fs, alloc.arena, weapon_comp.player_id, request.IncId, weapon.*);
    try events.enqueue(.weapon_info_modified, .{ .incr_id = request.IncId });
    if (weapon.role_id) |role_id| {
        try RoleAttributeSync.refreshRole(txn, alloc, fs, scene, assets, role_comp, weapon_comp, echo_comp, stat_query, role_id, io);
    }

    txn.respond(.{
        .ErrorCode = .Success,
        .IncId = request.IncId,
        .WeaponLevel = weapon.level,
        .WeaponExp = weapon.exp,
        .ItemMap = item_map,
    });
    try QuestEngine.onConditionStateChanged(qctx);
}

pub fn onWeaponBreachRequest(
    txn: *Transaction(pb.WeaponBreachRequest),
    events: *EventQueue,
    alloc: mem.Alloc,
    fs: *FileSystem,
    scene: *Scene,
    query: Scene.Query(&.{
        Scene.Entity,
        *Scene.Entity.ConfigComponent,
        *Scene.Entity.EquipComponent,
    }),
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
    const weapon = weapon_comp.weapon_map.getPtr(request.IncId) orelse {
        txn.respond(.{ .ErrorCode = .ErrWeaponConsumeItemNotFound });
        return;
    };
    const config = assets.tables.weapon_conf.getDataById(weapon.id) orelse {
        txn.respond(.{ .ErrorCode = .ErrWeaponResonConfigNotFound });
        return;
    };

    const breach_cfg = findWeaponBreachEntry(assets, config.BreachId, weapon.breach) orelse {
        txn.respond(.{ .ErrorCode = .ErrWeaponBreachLimit, .IncId = request.IncId, .WeaponBreach = weapon.breach });
        return;
    };
    if (findWeaponBreachEntry(assets, config.BreachId, weapon.breach + 1) == null) {
        txn.respond(.{ .ErrorCode = .ErrWeaponBreachLimit, .IncId = request.IncId, .WeaponBreach = weapon.breach });
        return;
    }
    if (weapon.level < breach_cfg.LevelLimit) {
        txn.respond(.{ .ErrorCode = .ErrWeaponLevelLimit, .IncId = request.IncId, .WeaponBreach = weapon.breach });
        return;
    }
    if (!ConditionEvaluator.evalConditionOrGroup(quest_types.makeEvalContext(qctx), breach_cfg.ConditionId)) {
        txn.respond(.{ .ErrorCode = .ErrWeaponBreachLimit, .IncId = request.IncId, .WeaponBreach = weapon.breach });
        return;
    }

    var costs: std.ArrayList(ConsumeHelper.CostItem) = .empty;
    try ConsumeHelper.appendIntMap(&costs, alloc.arena, breach_cfg.Consume);
    if (breach_cfg.GoldConsume > 0) {
        try costs.append(alloc.arena, .{ .id = 2, .count = breach_cfg.GoldConsume });
    }
    if (!(try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, costs.items, io))) {
        txn.respond(.{ .ErrorCode = .ErrItemNotEnough, .IncId = request.IncId, .WeaponBreach = weapon.breach });
        return;
    }

    weapon.breach += 1;
    try saveWeaponImmediate(fs, alloc.arena, weapon_comp.player_id, request.IncId, weapon.*);
    try events.enqueue(.weapon_info_modified, .{ .incr_id = request.IncId });

    if (weapon.role_id) |role_id| {
        var it = query.iterator;
        while (it.next()) |item| {
            const entity, const config_item, const equip = item;
            if (config_item.config_id != role_id) continue;

            equip.weapon_breach_level = weapon.breach;
            try scene.saveComponents(fs, alloc.gpa, entity, &.{Scene.Entity.EquipComponent});
            try txn.conn.push(alloc.gpa, io, pb.EntityEquipChangeNotify{
                .EntityId = entity.net_id,
                .EquipComponent = try equip.toProto(),
            });
            break;
        }
        try RoleAttributeSync.refreshRole(txn, alloc, fs, scene, assets, role_comp, weapon_comp, echo_comp, stat_query, role_id, io);
    }

    try events.enqueue(.update_formations, .{});
    txn.respond(.{
        .ErrorCode = .Success,
        .IncId = request.IncId,
        .WeaponBreach = weapon.breach,
    });
}

fn isWeaponResonAltItem(reson_cfg: Assets.DataTables.WeaponReson, item_id: i32) bool {
    for (reson_cfg.AlternativeConsume) |alt_id| {
        if (alt_id == item_id) return true;
    }
    return false;
}

pub fn onWeaponResonUpRequest(
    txn: *Transaction(pb.WeaponResonUpRequest),
    events: *EventQueue,
    alloc: mem.Alloc,
    fs: *FileSystem,
    scene: *Scene,
    query: Scene.Query(&.{
        Scene.Entity,
        *Scene.Entity.ConfigComponent,
        *Scene.Entity.FightBuffComponent,
    }),
    stat_query: Scene.Query(&.{
        Scene.Entity,
        *Scene.Entity.ConfigComponent,
        *Scene.Entity.AttributeComponent,
    }),
    assets: *const Assets,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    buff_timers: *BuffTimerScheduler,
    io: std.Io,
) !void {
    const now_ms = (std.Io.Clock.awake).now(io).toMilliseconds();
    const request = txn.message;
    const weapon = weapon_comp.weapon_map.getPtr(request.IncId) orelse {
        txn.respond(.{ .ErrorCode = .ErrWeaponConsumeItemNotFound });
        return;
    };
    const config = assets.tables.weapon_conf.getDataById(weapon.id) orelse {
        txn.respond(.{ .ErrorCode = .ErrWeaponResonConfigNotFound });
        return;
    };

    if (weapon.reson_level >= config.ResonLevelLimit) {
        txn.respond(.{ .ErrorCode = .ErrWeaponResonLevelLimit, .IncId = request.IncId, .ResonLevel = weapon.reson_level });
        return;
    }

    const old_reson_level = weapon.reson_level;
    const max_gain = config.ResonLevelLimit - old_reson_level;
    var gained_ranks: i32 = 0;
    var seen_fodder: std.AutoHashMapUnmanaged(i32, void) = .empty;
    defer seen_fodder.deinit(alloc.arena);
    var fodder_ids: std.ArrayList(i32) = .empty;
    var costs: std.ArrayList(ConsumeHelper.CostItem) = .empty;

    for (request.ConsumeList.items) |consume_id| {
        if (gained_ranks >= max_gain) break;
        if (consume_id == request.IncId or (try seen_fodder.getOrPut(alloc.arena, consume_id)).found_existing) continue;
        const consumed = weapon_comp.weapon_map.get(consume_id) orelse continue;
        if (consumed.id != weapon.id or consumed.role_id != null or consumed.func_value != 0) continue;
        try fodder_ids.append(alloc.arena, consume_id);
        if (assets.tables.getWeaponReson(config.ResonId, old_reson_level + gained_ranks)) |rcfg| {
            if (rcfg.GoldConsume > 0) try costs.append(alloc.arena, .{ .id = 2, .count = rcfg.GoldConsume });
        }
        gained_ranks += 1;
    }

    for (request.ConsumeItemList.items) |item| {
        if (gained_ranks >= max_gain) break;
        if (item.IncId > 0) {
            const consume_id = item.IncId;
            if (consume_id == request.IncId or (try seen_fodder.getOrPut(alloc.arena, consume_id)).found_existing) continue;
            const consumed = weapon_comp.weapon_map.get(consume_id) orelse continue;
            if (consumed.id != weapon.id or consumed.role_id != null or consumed.func_value != 0) continue;
            try fodder_ids.append(alloc.arena, consume_id);
            if (assets.tables.getWeaponReson(config.ResonId, old_reson_level + gained_ranks)) |rcfg| {
                if (rcfg.GoldConsume > 0) try costs.append(alloc.arena, .{ .id = 2, .count = rcfg.GoldConsume });
            }
            gained_ranks += 1;
        } else if (item.ItemId > 0 and item.Count > 0) {
            var remaining_mat = item.Count;
            while (remaining_mat > 0 and gained_ranks < max_gain) {
                const rcfg = assets.tables.getWeaponReson(config.ResonId, old_reson_level + gained_ranks) orelse break;
                if (!isWeaponResonAltItem(rcfg.*, item.ItemId)) break;
                const need_per_rank = @max(1, rcfg.Consume);
                if (remaining_mat < need_per_rank) break;
                remaining_mat -= need_per_rank;
                try costs.append(alloc.arena, .{ .id = item.ItemId, .count = need_per_rank });
                if (rcfg.GoldConsume > 0) try costs.append(alloc.arena, .{ .id = 2, .count = rcfg.GoldConsume });
                gained_ranks += 1;
            }
        }
    }

    if (gained_ranks == 0) {
        txn.respond(.{ .ErrorCode = .ErrWeaponResonConsumeItem, .IncId = request.IncId, .ResonLevel = weapon.reson_level });
        return;
    }

    if (costs.items.len > 0 and !(try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, costs.items, io))) {
        txn.respond(.{ .ErrorCode = .ErrItemNotEnough, .IncId = request.IncId, .ResonLevel = weapon.reson_level });
        return;
    }

    for (fodder_ids.items) |fid| {
        if (weapon_comp.weapon_map.fetchOrderedRemove(fid)) |removed| {
            var old = removed.value;
            old.deinit(alloc.gpa);
        }
        try PlayerWeaponComponent.deleteWeaponFile(alloc.gpa, fs, weapon_comp.player_id, fid);
    }

    const target_weapon = weapon_comp.weapon_map.getPtr(request.IncId) orelse {
        txn.respond(.{ .ErrorCode = .ErrWeaponConsumeItemNotFound });
        return;
    };
    target_weapon.reson_level = @min(old_reson_level + gained_ranks, config.ResonLevelLimit);
    try saveWeaponImmediate(fs, alloc.arena, weapon_comp.player_id, request.IncId, target_weapon.*);
    try events.enqueue(.weapon_info_modified, .{ .incr_id = request.IncId });
    if (fodder_ids.items.len > 0) {
        try txn.conn.push(alloc.gpa, io, pb.WeaponItemRemoveNotify{ .WeaponItemIncrIdList = fodder_ids });
    }

    if (target_weapon.role_id) |role_id| {
        const old_reson = assets.tables.getWeaponReson(config.ResonId, old_reson_level);
        const new_reson = assets.tables.getWeaponReson(config.ResonId, target_weapon.reson_level);

        var it = query.iterator;
        while (it.next()) |item| {
            const entity, const config_item, const buffs = item;
            if (config_item.config_id != role_id) continue;

            const changed = try syncWeaponPassiveBuffs(
                txn,
                alloc,
                scene,
                assets,
                fs,
                buff_timers,
                io,
                now_ms,
                entity,
                buffs,
                if (old_reson) |old| old.Effect else &.{},
                if (new_reson) |new| new.Effect else &.{},
            );
            if (changed) {
                buff_timers.syncEntityLeftDurations(scene, entity.net_id, now_ms);
                try scene.saveComponents(fs, alloc.gpa, entity, &.{Scene.Entity.FightBuffComponent});
            }
            break;
        }
        try RoleAttributeSync.refreshRole(txn, alloc, fs, scene, assets, role_comp, weapon_comp, echo_comp, stat_query, role_id, io);
    }

    txn.respond(.{
        .ErrorCode = .Success,
        .IncId = request.IncId,
        .ResonLevel = target_weapon.reson_level,
    });
}

// TODO: split this into events when implementing role element change
pub fn onEquipTakeOnRequest(
    txn: *Transaction(pb.EquipTakeOnRequest),
    events: *EventQueue,
    alloc: mem.Alloc,
    assets: *const Assets,
    fs: *FileSystem,
    scene: *Scene,
    query: Scene.Query(&.{
        Scene.Entity,
        *Scene.Entity.EquipComponent,
        *Scene.Entity.ConfigComponent,
        *Scene.Entity.FightBuffComponent,
    }),
    stat_query: Scene.Query(&.{
        Scene.Entity,
        *Scene.Entity.ConfigComponent,
        *Scene.Entity.AttributeComponent,
    }),
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    buff_timers: *BuffTimerScheduler,
    io: std.Io,
) !void {
    const now_ms = (std.Io.Clock.awake).now(io).toMilliseconds();
    const data = txn.message.Data orelse {
        txn.respond(.{ .ErrorCode = .RequestParamError });
        return;
    };

    if (role_comp.isTrialRole(data.RoleId)) {
        txn.respond(.{ .ErrorCode = .ErrLoadEquipInvalidRole });
        return;
    }

    var send_data: std.ArrayList(pb.RoleLoadEquipData) = .empty;
    defer send_data.deinit(alloc.arena);

    const current_role = role_comp.role_map.getPtr(data.RoleId) orelse {
        txn.respond(.{ .ErrorCode = .ErrLoadEquipInvalidRole });
        return;
    };
    const role_cfg = assets.tables.resolveBaseRoleInfo(data.RoleId) orelse {
        txn.respond(.{ .ErrorCode = .ErrLoadEquipRoleConfig });
        return;
    };
    if (current_role.weapon == data.EquipIncId) {
        try appendEquipTakeOnData(&send_data, alloc.arena, data.RoleId, data.EquipIncId, .Weapon);
        txn.respond(.{ .ErrorCode = .Success, .DataList = send_data });
        return;
    }
    var weapon = weapon_comp.weapon_map.getPtr(data.EquipIncId) orelse {
        txn.respond(.{ .ErrorCode = .ErrWeaponConsumeItemNotFound });
        return;
    };
    const weapon_conf = assets.tables.weapon_conf.getDataById(weapon.id) orelse {
        txn.respond(.{ .ErrorCode = .ErrWeaponResonConfigNotFound });
        return;
    };
    const weapon_type = if (weapon_conf.WeaponType > 0) weapon_conf.WeaponType else @rem(@divTrunc(weapon_conf.ItemId, 10000), 100);
    if (role_cfg.WeaponType != weapon_type) {
        txn.respond(.{ .ErrorCode = .ErrLoadEquipInvalidRole });
        return;
    }

    const old_weapon_opt: ?*WeaponItem = if (current_role.weapon != 0)
        weapon_comp.weapon_map.getPtr(current_role.weapon)
    else
        null;
    const old_weapon_reson = if (old_weapon_opt) |ow| blk: {
        const ow_conf = assets.tables.weapon_conf.getDataById(ow.id) orelse break :blk null;
        break :blk assets.tables.getWeaponReson(ow_conf.ResonId, ow.reson_level);
    } else null;
    const current_weapon_reson = assets.tables.getWeaponReson(weapon_conf.ResonId, weapon.reson_level);
    const current_weapon_effects: []const i64 = if (current_weapon_reson) |r| r.Effect else &.{};
    const old_weapon_effects: []const i64 = if (old_weapon_reson) |r| r.Effect else &.{};
    var swapped_role_id: ?i32 = null;

    if (weapon.role_id) |old_role_id| {
        if (role_comp.isTrialRole(old_role_id)) {
            txn.respond(.{ .ErrorCode = .ErrLoadEquipInvalidRole });
            return;
        }
        var old_role = role_comp.role_map.getPtr(old_role_id) orelse {
            txn.respond(.{ .ErrorCode = .ErrLoadEquipInvalidRole });
            return;
        };
        if (old_weapon_opt) |old_weapon| {
            swapped_role_id = old_role_id;
            old_weapon.role_id = old_role_id;
            old_role.weapon = current_role.weapon;

            try appendEquipTakeOnData(&send_data, alloc.arena, old_role_id, current_role.weapon, .Weapon);
            var iterator = query.iterator;
            while (iterator.next()) |item| {
                const entity, const equip, const config, const buffs: *FightBuffComponent = item;
                if (config.config_id != old_role_id) continue;

                const buff_changed = try syncWeaponPassiveBuffs(
                    txn,
                    alloc,
                    scene,
                    assets,
                    fs,
                    buff_timers,
                    io,
                    now_ms,
                    entity,
                    buffs,
                    current_weapon_effects,
                    old_weapon_effects,
                );

                equip.weapon_id = old_weapon.id;
                equip.weapon_breach_level = old_weapon.breach;
                if (buff_changed) {
                    buff_timers.syncEntityLeftDurations(scene, entity.net_id, now_ms);
                    try scene.saveComponents(fs, alloc.gpa, entity, &.{ Scene.Entity.EquipComponent, Scene.Entity.FightBuffComponent });
                } else {
                    try scene.saveComponents(fs, alloc.gpa, entity, &.{Scene.Entity.EquipComponent});
                }

                try txn.conn.push(alloc.gpa, io, pb.EntityEquipChangeNotify{
                    .EntityId = entity.net_id,
                    .EquipComponent = try equip.toProto(),
                });
                break;
            }
            try events.enqueue(.weapon_info_modified, .{ .incr_id = current_role.weapon });
            try events.enqueue(.role_info_modified, .{ .role_id = old_role_id });
        } else {
            old_role.weapon = 0;
            try events.enqueue(.role_info_modified, .{ .role_id = old_role_id });
        }
    } else if (old_weapon_opt) |old_weapon| {
        old_weapon.role_id = null;
        try events.enqueue(.weapon_info_modified, .{ .incr_id = current_role.weapon });
        try appendEquipTakeOnData(&send_data, alloc.arena, 0, current_role.weapon, .Weapon);
    }

    weapon.role_id = data.RoleId;
    current_role.weapon = data.EquipIncId;

    try appendEquipTakeOnData(&send_data, alloc.arena, data.RoleId, data.EquipIncId, .Weapon);
    var iterator = query.iterator;
    while (iterator.next()) |item| {
        const entity, const equip, const config, const buffs = item;
        if (config.config_id == data.RoleId) {
            const buff_changed = try syncWeaponPassiveBuffs(
                txn,
                alloc,
                scene,
                assets,
                fs,
                buff_timers,
                io,
                now_ms,
                entity,
                buffs,
                old_weapon_effects,
                current_weapon_effects,
            );

            equip.weapon_id = weapon.id;
            equip.weapon_breach_level = weapon.breach;
            if (buff_changed) {
                buff_timers.syncEntityLeftDurations(scene, entity.net_id, now_ms);
                try scene.saveComponents(fs, alloc.gpa, entity, &.{ Scene.Entity.EquipComponent, Scene.Entity.FightBuffComponent });
            } else {
                try scene.saveComponents(fs, alloc.gpa, entity, &.{Scene.Entity.EquipComponent});
            }

            try txn.conn.push(alloc.gpa, io, pb.EntityEquipChangeNotify{
                .EntityId = entity.net_id,
                .EquipComponent = try equip.toProto(),
            });
            break;
        }
    }

    try events.enqueue(.role_info_modified, .{ .role_id = data.RoleId });
    try events.enqueue(.weapon_info_modified, .{ .incr_id = data.EquipIncId });
    try events.enqueue(.update_formations, .{});
    if (swapped_role_id) |old_role_id| {
        try RoleAttributeSync.refreshRole(txn, alloc, fs, scene, assets, role_comp, weapon_comp, echo_comp, stat_query, old_role_id, io);
    }
    try RoleAttributeSync.refreshRole(txn, alloc, fs, scene, assets, role_comp, weapon_comp, echo_comp, stat_query, data.RoleId, io);
    try txn.conn.push(alloc.gpa, io, pb.EquipTakeOnNotify{ .DataList = send_data });
    txn.respond(.{ .ErrorCode = .Success, .DataList = send_data });
}
