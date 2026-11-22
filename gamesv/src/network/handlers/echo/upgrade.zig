const std = @import("std");
const pb = @import("proto").pb;
const Transaction = @import("../../handlers.zig").Transaction;
const mem = @import("../../../mem.zig");
const FileSystem = @import("common").FileSystem;
const Assets = @import("../../../data/Assets.zig");
const Scene = @import("../../../logic/Scene.zig");
const PlayerRoleComponent = @import("../../../logic/component/player/PlayerRoleComponent.zig");
const PlayerBasicComponent = @import("../../../logic/component/player/PlayerBasicComponent.zig");
const PlayerEchoComponent = @import("../../../logic/component/player/PlayerEchoComponent.zig");
const PlayerWeaponComponent = @import("../../../logic/component/player/PlayerWeaponComponent.zig");
const PlayerInventoryComponent = @import("../../../logic/component/player/PlayerInventoryComponent.zig");
const EchoInfo = @import("../../../fs/EchoInfo.zig");
const ConsumeHelper = @import("../../../logic/helpers/consume.zig");
const RewardService = @import("../../../logic/helpers/reward.zig");
const CalabashHelper = @import("../../../logic/helpers/calabash.zig");
const Entity = Scene.Entity;
const BuffTimerScheduler = @import("../../../logic/schedulers/BuffTimerScheduler.zig");
const shared = @import("shared.zig");
const QuestEngine = @import("../../../logic/helpers/quest_engine.zig");

const addConsume = shared.addConsume;
const tunerCost = shared.tunerCost;
const highestPhantomLevel = shared.highestPhantomLevel;
const phantomLevelExp = shared.phantomLevelExp;
const rolesEquippingEcho = shared.rolesEquippingEcho;
const appendRolesEquippingEcho = shared.appendRolesEquippingEcho;
const pushRolePropUpdate = shared.pushRolePropUpdate;
const refreshRoleEntities = shared.refreshRoleEntities;

pub fn onPhantomLevelUpRequest(
    txn: *Transaction(pb.PhantomLevelUpRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    scene: *Scene,
    basic_comp: *PlayerBasicComponent,
    role_comp: *PlayerRoleComponent,
    echo_comp: *PlayerEchoComponent,
    inventory_comp: *PlayerInventoryComponent,
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
    const item = echo_comp.echo_map.getPtr(txn.message.IncId) orelse {
        txn.respond(.{ .ErrorCode = .ErrPhantomItemNotExist });
        return;
    };
    const item_config = assets.tables.phantom_item.getDataById(item.id) orelse {
        txn.respond(.{ .ErrorCode = .ErrPhantomConfigNotFound });
        return;
    };
    const quality = assets.tables.phantom_quality.getDataById(item_config.QualityId) orelse {
        txn.respond(.{ .ErrorCode = .ErrPhantomQaulityNotExist });
        return;
    };
    const highest_level = @min(quality.LevelLimit, highestPhantomLevel(assets, item_config.LevelUpGroupId) orelse {
        txn.respond(.{ .ErrorCode = .ErrPhantomLevelConfigNotExist });
        return;
    });

    const slot_count = if (txn.message.SlotCount > 0) txn.message.SlotCount else 0;
    if (slot_count > 0) {
        const max_sub_props = EchoInfo.maxUnlockedSubPropCount(assets, item_config.QualityId) orelse {
            txn.respond(.{ .ErrorCode = .ErrPhantomSubPropNotEnough });
            return;
        };
        if (item.sub_prop.len >= max_sub_props or item.sub_prop.len + @as(usize, @intCast(slot_count)) > max_sub_props) {
            txn.respond(.{ .ErrorCode = .ErrPhantomSubPropNotEnough });
            return;
        }
    } else if (item.level >= highest_level) {
        txn.respond(.{ .ErrorCode = .ErrPhantomLvupMax });
        return;
    }

    var consumes: std.array_hash_map.Auto(i32, i32) = .empty;
    defer consumes.deinit(alloc.gpa);
    var refund_map: std.array_hash_map.Auto(i32, i32) = .empty;
    defer refund_map.deinit(alloc.gpa);
    var seen_fodder: std.AutoHashMapUnmanaged(i32, void) = .empty;
    defer seen_fodder.deinit(alloc.gpa);
    var fodder_consumes: std.ArrayList(struct { inc_id: i32, count: i32 }) = .empty;
    defer fodder_consumes.deinit(alloc.gpa);

    var item_exp_total: i32 = 0;
    var fodder_exp_total: i32 = 0;
    for (txn.message.ConsumeList.items) |consume| {
        if (consume.Count <= 0) {
            txn.respond(.{ .ErrorCode = .ErrPhantomConsumeItemCount });
            return;
        }
        if (consume.IncId != 0) {
            if (consume.IncId == txn.message.IncId) {
                txn.respond(.{ .ErrorCode = .ErrPhantomConsumeItem });
                return;
            }
            if (seen_fodder.contains(consume.IncId)) {
                txn.respond(.{ .ErrorCode = .ErrPhantomConsumeItemIncrDuplicate });
                return;
            }
            try seen_fodder.put(alloc.gpa, consume.IncId, {});

            const fodder = echo_comp.echo_map.get(consume.IncId) orelse {
                txn.respond(.{ .ErrorCode = .ErrPhantomItemNotExist });
                return;
            };
            if (consume.Count > @max(1, fodder.stack_count)) {
                txn.respond(.{ .ErrorCode = .ErrPhantomConsumeItemCount });
                return;
            }
            if (fodder.func_value == 1) {
                txn.respond(.{ .ErrorCode = .ErrPhantomLevelUpMaterialLock });
                return;
            }
            if (fodder.role_id != null) {
                txn.respond(.{ .ErrorCode = .ErrPhantomDecomposeEquiped });
                return;
            }
            const fodder_cfg = assets.tables.phantom_item.getDataById(fodder.id) orelse {
                txn.respond(.{ .ErrorCode = .ErrPhantomConfigNotFound });
                return;
            };
            if (fodder.level == 0 and fodder.exp == 0) {
                txn.respond(.{ .ErrorCode = .ErrPhantomConsumeNoExp });
                return;
            }
            const invested = shared.totalInvestedPhantomExp(assets, fodder_cfg.LevelUpGroupId, fodder);
            const single_exp = @max(1, @divTrunc(invested * 75, 100));
            const batch_exp = std.math.mul(i32, single_exp, consume.Count) catch {
                txn.respond(.{ .ErrorCode = .ErrPhantomConsumeItemCount });
                return;
            };
            fodder_exp_total = std.math.add(i32, fodder_exp_total, batch_exp) catch {
                txn.respond(.{ .ErrorCode = .ErrPhantomConsumeItemCount });
                return;
            };

            if (fodder.sub_prop.len > 0) {
                if (tunerCost(assets, fodder_cfg.QualityId)) |fodder_tc| {
                    const refund_count = @divTrunc(@as(i32, @intCast(fodder.sub_prop.len)) * fodder_tc.count * 30, 100) *| consume.Count;
                    if (refund_count > 0) {
                        try addConsume(alloc.gpa, &refund_map, fodder_tc.item_id, refund_count);
                    }
                }
            }
            try fodder_consumes.append(alloc.gpa, .{ .inc_id = consume.IncId, .count = consume.Count });
        } else {
            const exp_item = assets.tables.phantom_exp_item.getDataById(consume.ItemId) orelse {
                txn.respond(.{ .ErrorCode = .ErrPhantomExpItemNotExist });
                return;
            };
            try addConsume(alloc.gpa, &consumes, consume.ItemId, consume.Count);
            const mul_exp = std.math.mul(i32, exp_item.Exp, consume.Count) catch {
                txn.respond(.{ .ErrorCode = .ErrPhantomConsumeItemCount });
                return;
            };
            item_exp_total = std.math.add(i32, item_exp_total, mul_exp) catch {
                txn.respond(.{ .ErrorCode = .ErrPhantomConsumeItemCount });
                return;
            };
        }
    }

    const exp_total = std.math.add(i32, item_exp_total, fodder_exp_total) catch {
        txn.respond(.{ .ErrorCode = .ErrPhantomConsumeItemCount });
        return;
    };
    if (exp_total == 0 and slot_count == 0) {
        txn.respond(.{ .ErrorCode = .ErrPhantomConsumeNoExp });
        return;
    }

    var next_level = item.level;
    var next_exp = item.exp;
    var excess_exp: i32 = 0;
    if (exp_total > 0) {
        var total: i64 = @as(i64, item.exp) + exp_total;
        while (next_level < highest_level) {
            const lvl_up = next_level + 1;
            const req_exp = phantomLevelExp(assets, item_config.LevelUpGroupId, lvl_up) orelse break;
            if (total < req_exp) break;
            next_level = lvl_up;
            total -= req_exp;
        }
        if (next_level >= highest_level) {
            next_exp = 0;
            excess_exp = @intCast(@min(total, @as(i64, std.math.maxInt(i32))));
        } else {
            next_exp = @intCast(total);
        }
    }

    if (slot_count > 0) {
        const predicted_unlocked = EchoInfo.unlockedSubPropCount(assets, item_config.QualityId, next_level) orelse {
            txn.respond(.{ .ErrorCode = .ErrPhantomSubPropNotEnough });
            return;
        };
        if (item.sub_prop.len + @as(usize, @intCast(slot_count)) > predicted_unlocked) {
            txn.respond(.{ .ErrorCode = .ErrPhantomSubPropNotEnough });
            return;
        }
        const tuner_cost = tunerCost(assets, item_config.QualityId) orelse {
            txn.respond(.{ .ErrorCode = .ErrPhantomQaulityNotExist });
            return;
        };
        try addConsume(alloc.gpa, &consumes, tuner_cost.item_id, std.math.mul(i32, slot_count, tuner_cost.count) catch {
            txn.respond(.{ .ErrorCode = .ErrPhantomConsumeItemCount });
            return;
        });
        if (quality.IdentifyCoin > 0) {
            try addConsume(alloc.gpa, &consumes, 2, std.math.mul(i32, slot_count, quality.IdentifyCoin) catch {
                txn.respond(.{ .ErrorCode = .ErrPhantomConsumeItemCount });
                return;
            });
        }
    }

    if (item_exp_total > 0) {
        const exp_coin: i32 = @intCast(@max(@as(i64, 1), @divTrunc(@as(i64, item_exp_total) * 10, 100)));
        try addConsume(alloc.gpa, &consumes, 2, exp_coin);
    }

    var cost_items: std.ArrayList(ConsumeHelper.CostItem) = .empty;
    try ConsumeHelper.appendIntMap(&cost_items, alloc.arena, .{ .map = consumes });
    if (!(try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, cost_items.items, io))) {
        txn.respond(.{ .ErrorCode = .ErrItemNotEnough });
        return;
    }

    if (fodder_consumes.items.len > 0) {
        var removed_ids: std.ArrayList(i32) = .empty;
        var updated_fodders: std.ArrayList(pb.PhantomItem) = .empty;
        for (fodder_consumes.items) |fc| {
            if (echo_comp.echo_map.getPtr(fc.inc_id)) |fodder| {
                if (fodder.stack_count > fc.count) {
                    fodder.stack_count -= fc.count;
                    try updated_fodders.append(alloc.arena, try fodder.toProto(fc.inc_id, alloc.arena));
                    continue;
                }
            }
            if (echo_comp.echo_map.fetchSwapRemove(fc.inc_id)) |removed| {
                var old = removed.value;
                old.deinit(alloc.gpa);
            }
            try PlayerEchoComponent.deleteEchoFile(alloc.gpa, fs, echo_comp.player_id, fc.inc_id);
            try removed_ids.append(alloc.arena, fc.inc_id);
        }
        if (removed_ids.items.len > 0) {
            try txn.conn.push(alloc.gpa, io, pb.PhantomItemRemoveNotify{
                .PhantomItemIncrIdList = removed_ids,
            });
        }
        if (updated_fodders.items.len > 0) {
            try txn.conn.push(alloc.gpa, io, pb.PhantomItemUpdateNotify{
                .UpdateInfo = updated_fodders,
            });
        }
    }

    const stack_split = try shared.splitSingleFromStack(alloc.gpa, alloc.arena, fs, echo_comp, txn.message.IncId);
    const target_item = echo_comp.echo_map.getPtr(txn.message.IncId) orelse {
        txn.respond(.{ .ErrorCode = .ErrPhantomItemNotExist });
        return;
    };

    if (refund_map.count() > 0) {
        var refunds: std.ArrayList(RewardService.RewardItem) = .empty;
        var rit = refund_map.iterator();
        while (rit.next()) |kv| {
            try refunds.append(alloc.arena, .{ .item_id = kv.key_ptr.*, .count = kv.value_ptr.* });
        }
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

    if (exp_total > 0) {
        target_item.level = next_level;
        target_item.exp = next_exp;
        target_item.refreshMainPropValues(assets);
    }

    if (slot_count > 0) {
        if (EchoInfo.unlockedSubPropCount(assets, item_config.QualityId, target_item.level)) |unlocked_sub_props| {
            _ = try target_item.appendSubProps(
                alloc.gpa,
                assets,
                item_config.QualityId,
                unlocked_sub_props,
                @intCast(slot_count),
                randomSeed(fs),
            );
        }
    }

    try PlayerEchoComponent.saveAll(alloc.gpa, fs, echo_comp.player_id, echo_comp.echo_map);

    var changed_roles = try rolesEquippingEcho(echo_comp, alloc.gpa, txn.message.IncId);
    defer changed_roles.deinit(alloc.gpa);
    if (changed_roles.count() != 0) {
        try pushRolePropUpdate(txn, alloc, assets, role_comp, echo_comp, weapon_comp, changed_roles, io);
        try refreshRoleEntities(txn, alloc, fs, assets, scene, role_comp, echo_comp, weapon_comp, query, changed_roles, buff_timers, io, now_ms);
    }

    var overflow_item_map: std.ArrayList(pb.MapEntry(i32, i32)) = .empty;
    if (excess_exp > 0) {
        var rem = excess_exp;
        const echo_pots = [_]struct { id: i32, exp: i32 }{
            .{ .id = 36000004, .exp = 5000 },
            .{ .id = 36000003, .exp = 2000 },
            .{ .id = 36000002, .exp = 1000 },
            .{ .id = 36000001, .exp = 500 },
        };
        var exp_refunds: std.ArrayList(RewardService.RewardItem) = .empty;
        for (echo_pots) |pot| {
            if (rem >= pot.exp) {
                const cnt = @divTrunc(rem, pot.exp);
                if (cnt > 0) {
                    rem -= cnt * pot.exp;
                    try exp_refunds.append(alloc.arena, .{ .item_id = pot.id, .count = cnt });
                    try overflow_item_map.append(alloc.arena, .{ .key = pot.id, .value = cnt });
                }
            }
        }
        if (exp_refunds.items.len > 0) {
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
            }, 0, false, exp_refunds.items, 0);
        }
    }

    txn.respond(.{
        .ErrorCode = .Success,
        .UpdateInfo = try target_item.toProto(txn.message.IncId, alloc.arena),
        .ItemMap = overflow_item_map,
        .StackSplitInfo = stack_split,
    });
    try QuestEngine.onConditionStateChanged(qctx);
}

pub fn onPhantomIdentifyRequest(
    txn: *Transaction(pb.PhantomIdentifyRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    scene: *Scene,
    basic_comp: *PlayerBasicComponent,
    role_comp: *PlayerRoleComponent,
    echo_comp: *PlayerEchoComponent,
    inventory_comp: *PlayerInventoryComponent,
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
    if (txn.message.Count <= 0) {
        txn.respond(.{ .ErrorCode = .ErrPhantomConsumeItemCount });
        return;
    }
    const item = echo_comp.echo_map.getPtr(txn.message.IncrId) orelse {
        txn.respond(.{ .ErrorCode = .ErrPhantomItemNotExist });
        return;
    };
    const item_config = assets.tables.phantom_item.getDataById(item.id) orelse {
        txn.respond(.{ .ErrorCode = .ErrPhantomConfigNotFound });
        return;
    };
    const quality = assets.tables.phantom_quality.getDataById(item_config.QualityId) orelse {
        txn.respond(.{ .ErrorCode = .ErrPhantomQaulityNotExist });
        return;
    };
    const max_sub_props = EchoInfo.maxUnlockedSubPropCount(assets, item_config.QualityId) orelse {
        txn.respond(.{ .ErrorCode = .ErrPhantomSubPropNotEnough });
        return;
    };
    if (item.sub_prop.len >= max_sub_props or item.sub_prop.len + @as(usize, @intCast(txn.message.Count)) > max_sub_props) {
        txn.respond(.{ .ErrorCode = .ErrPhantomSubPropNotEnough });
        return;
    }
    const unlocked_sub_props = EchoInfo.unlockedSubPropCount(assets, item_config.QualityId, item.level) orelse {
        txn.respond(.{ .ErrorCode = .ErrPhantomSubPropNotEnough });
        return;
    };
    if (item.sub_prop.len + @as(usize, @intCast(txn.message.Count)) > unlocked_sub_props) {
        txn.respond(.{ .ErrorCode = .ErrPhantomSubPropNotEnough });
        return;
    }

    const tuner_cost = tunerCost(assets, item_config.QualityId) orelse {
        txn.respond(.{ .ErrorCode = .ErrPhantomQaulityNotExist });
        return;
    };

    var cost_list: [2]ConsumeHelper.CostItem = .{
        .{ .id = tuner_cost.item_id, .count = tuner_cost.count * txn.message.Count },
        .{ .id = 2, .count = quality.IdentifyCoin * txn.message.Count },
    };
    const cost_slice = if (quality.IdentifyCoin > 0) cost_list[0..2] else cost_list[0..1];
    if (!(try ConsumeHelper.hasCosts(alloc.arena, basic_comp, inventory_comp, cost_slice))) {
        txn.respond(.{ .ErrorCode = .ErrPhantomIdentifyNoCost });
        return;
    }

    const added = try item.appendSubProps(
        alloc.gpa,
        assets,
        item_config.QualityId,
        unlocked_sub_props,
        @intCast(txn.message.Count),
        randomSeed(fs),
    );
    if (added == 0) {
        txn.respond(.{ .ErrorCode = .ErrPhantomSubPropRandomErr });
        return;
    }

    _ = try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, cost_slice, io);
    try PlayerEchoComponent.saveAll(alloc.gpa, fs, echo_comp.player_id, echo_comp.echo_map);

    var changed_roles = try rolesEquippingEcho(echo_comp, alloc.gpa, txn.message.IncrId);
    defer changed_roles.deinit(alloc.gpa);
    if (changed_roles.count() != 0) {
        try pushRolePropUpdate(txn, alloc, assets, role_comp, echo_comp, weapon_comp, changed_roles, io);
        try refreshRoleEntities(txn, alloc, fs, assets, scene, role_comp, echo_comp, weapon_comp, query, changed_roles, buff_timers, io, now_ms);
    }

    QuestEngine.onVisionSystemEvent(qctx, "DoVisionIdentify") catch |err| QuestEngine.logHookErr("onVisionSystemEvent", err);

    txn.respond(.{
        .ErrorCode = .Success,
        .UpdateInfo = try item.toProto(txn.message.IncrId, alloc.arena),
    });
}

pub fn onPhantomPolishRequest(
    txn: *Transaction(pb.PhantomPolishRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    scene: *Scene,
    basic_comp: *PlayerBasicComponent,
    role_comp: *PlayerRoleComponent,
    echo_comp: *PlayerEchoComponent,
    inventory_comp: *PlayerInventoryComponent,
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
    io: std.Io,
) !void {
    const now_ms = (std.Io.Clock.awake).now(io).toMilliseconds();
    const item = echo_comp.echo_map.getPtr(txn.message.IncrId) orelse {
        txn.respond(.{ .ErrorCode = .ErrPhantomItemNotExist });
        return;
    };
    const item_config = assets.tables.phantom_item.getDataById(item.id) orelse {
        txn.respond(.{ .ErrorCode = .ErrPhantomConfigNotFound });
        return;
    };
    const rarity_cfg = assets.tables.phantom_rarity.getDataById(item_config.Rarity) orelse {
        txn.respond(.{ .ErrorCode = .ErrPhantomConfigNotFound });
        return;
    };
    if (validateMainPropPolish(assets, item.*, item_config, txn.message.PhantomMainPropItemId)) |err| {
        txn.respond(.{ .ErrorCode = err });
        return;
    }

    var costs: std.ArrayList(ConsumeHelper.CostItem) = .empty;
    try shared.appendIntMapCosts(alloc.arena, &costs, rarity_cfg.PolishCost);
    if (!(try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, costs.items, io))) {
        txn.respond(.{ .ErrorCode = .ErrItemNotEnough });
        return;
    }

    const stack_split = try shared.splitSingleFromStack(alloc.gpa, alloc.arena, fs, echo_comp, txn.message.IncrId);
    const target_item = echo_comp.echo_map.getPtr(txn.message.IncrId) orelse {
        txn.respond(.{ .ErrorCode = .ErrPhantomItemNotExist });
        return;
    };
    if (try applyMainPropPolish(alloc.gpa, assets, target_item, item_config, txn.message.PhantomMainPropItemId)) |err| {
        txn.respond(.{ .ErrorCode = err });
        return;
    }

    try PlayerEchoComponent.saveAll(alloc.gpa, fs, echo_comp.player_id, echo_comp.echo_map);

    var changed_roles = try rolesEquippingEcho(echo_comp, alloc.gpa, txn.message.IncrId);
    defer changed_roles.deinit(alloc.gpa);
    if (changed_roles.count() != 0) {
        try pushRolePropUpdate(txn, alloc, assets, role_comp, echo_comp, weapon_comp, changed_roles, io);
        try refreshRoleEntities(txn, alloc, fs, assets, scene, role_comp, echo_comp, weapon_comp, query, changed_roles, buff_timers, io, now_ms);
    }

    txn.respond(.{
        .ErrorCode = .Success,
        .UpdateInfo = try target_item.toProto(txn.message.IncrId, alloc.arena),
        .StackSplitInfo = stack_split,
    });
}

pub fn onPhantomBatchPolishRequest(
    txn: *Transaction(pb.PhantomBatchPolishRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    scene: *Scene,
    basic_comp: *PlayerBasicComponent,
    role_comp: *PlayerRoleComponent,
    echo_comp: *PlayerEchoComponent,
    inventory_comp: *PlayerInventoryComponent,
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
    io: std.Io,
) !void {
    const now_ms = (std.Io.Clock.awake).now(io).toMilliseconds();
    if (txn.message.StackEntries.items.len == 0 or txn.message.StackEntries.items.len > 10) {
        txn.respond(.{ .ErrorCode = .ErrPhantomBatchPolishCount });
        return;
    }

    var seen: std.hash_map.AutoHashMapUnmanaged(i32, void) = .empty;
    defer seen.deinit(alloc.gpa);
    var costs: std.ArrayList(ConsumeHelper.CostItem) = .empty;

    for (txn.message.StackEntries.items) |stack| {
        const inc_id = stack.IncrId;
        if (seen.contains(inc_id)) {
            txn.respond(.{ .ErrorCode = .ErrPhantomBatchPolishDuplicate });
            return;
        }
        try seen.put(alloc.gpa, inc_id, {});

        const item = echo_comp.echo_map.get(inc_id) orelse {
            txn.respond(.{ .ErrorCode = .ErrPhantomItemNotExist });
            return;
        };
        const item_config = assets.tables.phantom_item.getDataById(item.id) orelse {
            txn.respond(.{ .ErrorCode = .ErrPhantomConfigNotFound });
            return;
        };
        if (item_config.QualityId < 5) {
            txn.respond(.{ .ErrorCode = .ErrPhantomBatchPolishNoValidItem });
            return;
        }
        const rarity_cfg = assets.tables.phantom_rarity.getDataById(item_config.Rarity) orelse {
            txn.respond(.{ .ErrorCode = .ErrPhantomConfigNotFound });
            return;
        };
        if (validateMainPropPolish(assets, item, item_config, txn.message.PhantomMainPropItemId)) |err| {
            txn.respond(.{ .ErrorCode = err });
            return;
        }
        try shared.appendIntMapCosts(alloc.arena, &costs, rarity_cfg.PolishCost);
    }

    if (!(try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, costs.items, io))) {
        txn.respond(.{ .ErrorCode = .ErrItemNotEnough });
        return;
    }

    var changed_roles: std.array_hash_map.Auto(i32, void) = .empty;
    defer changed_roles.deinit(alloc.gpa);
    var update_infos: std.ArrayList(pb.PhantomItem) = .empty;
    var split_infos: std.ArrayList(pb.PhantomStackSplitInfo) = .empty;

    for (txn.message.StackEntries.items) |stack| {
        const inc_id = stack.IncrId;
        if (try shared.splitSingleFromStack(alloc.gpa, alloc.arena, fs, echo_comp, inc_id)) |split| {
            try split_infos.append(alloc.arena, split);
        }
        const item = echo_comp.echo_map.getPtr(inc_id) orelse {
            txn.respond(.{ .ErrorCode = .ErrPhantomItemNotExist });
            return;
        };
        const item_config = assets.tables.phantom_item.getDataById(item.id) orelse {
            txn.respond(.{ .ErrorCode = .ErrPhantomConfigNotFound });
            return;
        };
        if (try applyMainPropPolish(alloc.gpa, assets, item, item_config, txn.message.PhantomMainPropItemId)) |err| {
            txn.respond(.{ .ErrorCode = err });
            return;
        }

        try appendRolesEquippingEcho(echo_comp, alloc.gpa, inc_id, &changed_roles);
        try update_infos.append(alloc.arena, try item.toProto(inc_id, alloc.arena));
    }

    try PlayerEchoComponent.saveAll(alloc.gpa, fs, echo_comp.player_id, echo_comp.echo_map);

    if (changed_roles.count() != 0) {
        try pushRolePropUpdate(txn, alloc, assets, role_comp, echo_comp, weapon_comp, changed_roles, io);
        try refreshRoleEntities(txn, alloc, fs, assets, scene, role_comp, echo_comp, weapon_comp, query, changed_roles, buff_timers, io, now_ms);
    }

    txn.respond(.{
        .ErrorCode = .Success,
        .UpdateInfos = update_infos,
        .StackSplitInfos = split_infos,
    });
}

pub fn onPhantomVicePolishRequest(
    txn: *Transaction(pb.PhantomVicePolishRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    basic_comp: *PlayerBasicComponent,
    echo_comp: *PlayerEchoComponent,
    inventory_comp: *PlayerInventoryComponent,
    io: std.Io,
) !void {
    if (CalabashHelper.currentLevel(assets, basic_comp, echo_comp) < 21) {
        txn.respond(.{ .ErrorCode = .ErrPhantomVicePolishLimit });
        return;
    }
    const item = echo_comp.echo_map.getPtr(txn.message.IncrId) orelse {
        txn.respond(.{ .ErrorCode = .ErrPhantomItemNotExist });
        return;
    };
    const item_config = assets.tables.phantom_item.getDataById(item.id) orelse {
        txn.respond(.{ .ErrorCode = .ErrPhantomConfigNotFound });
        return;
    };
    const max_sub_props = EchoInfo.maxUnlockedSubPropCount(assets, item_config.QualityId) orelse {
        txn.respond(.{ .ErrorCode = .ErrPhantomVicePolishItemPropLimit });
        return;
    };
    if (max_sub_props == 0) {
        txn.respond(.{ .ErrorCode = .ErrPhantomVicePolishItemPropLimit });
        return;
    }
    if (item.sub_prop.len == 0) {
        txn.respond(.{ .ErrorCode = .ErrPhantomVicePolishNoneProp });
        return;
    }
    const unlocked_sub_props = EchoInfo.unlockedSubPropCount(assets, item_config.QualityId, item.level) orelse {
        txn.respond(.{ .ErrorCode = .ErrPhantomVicePolishItemLevelLimit });
        return;
    };
    if (unlocked_sub_props < max_sub_props) {
        txn.respond(.{ .ErrorCode = .ErrPhantomVicePolishItemLevelLimit });
        return;
    }
    if (item.sub_prop.len < max_sub_props) {
        txn.respond(.{ .ErrorCode = .ErrPhantomVicePolishItemPropLimit });
        return;
    }

    var lock_indices = validatedLockPropIndexes(alloc.gpa, item.sub_prop.len, txn.message.LockPropIndex.items) catch {
        txn.respond(.{ .ErrorCode = .ErrPhantomVicePolishParam });
        return;
    };
    defer lock_indices.deinit(alloc.gpa);
    const vice_cfg = assets.tables.phantom_vice_polish_config.getDataById(@intCast(lock_indices.count())) orelse {
        txn.respond(.{ .ErrorCode = .ErrPhantomVicePolishLimit });
        return;
    };

    var costs: std.ArrayList(ConsumeHelper.CostItem) = .empty;
    try shared.appendIntMapCosts(alloc.arena, &costs, vice_cfg.Cost);
    if (!(try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, costs.items, io))) {
        txn.respond(.{ .ErrorCode = .ErrItemNotEnough });
        return;
    }

    const new_props = try buildVicePolishProps(alloc.gpa, assets, item.*, item_config, lock_indices, randomSeed(fs));
    replacePropSlice(alloc.gpa, &item.unack_sub_prop, new_props);
    try replaceIntSlice(alloc.gpa, &item.lock_prop_index, txn.message.LockPropIndex.items);

    try PlayerEchoComponent.saveAll(alloc.gpa, fs, echo_comp.player_id, echo_comp.echo_map);
    txn.respond(.{
        .ErrorCode = .Success,
        .PhantomSubProp = try propList(item.unack_sub_prop, alloc.arena),
    });
}

pub fn onPhantomVicePolishAckRequest(
    txn: *Transaction(pb.PhantomVicePolishAckRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    scene: *Scene,
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
    io: std.Io,
) !void {
    const now_ms = (std.Io.Clock.awake).now(io).toMilliseconds();
    const item = echo_comp.echo_map.getPtr(txn.message.IncrId) orelse {
        txn.respond(.{ .ErrorCode = .ErrPhantomItemNotExist });
        return;
    };
    if (item.unack_sub_prop.len == 0) {
        txn.respond(.{ .ErrorCode = .ErrPhantomVicePolishNoAck });
        return;
    }

    if (txn.message.Ack) {
        clearPropSlice(alloc.gpa, &item.sub_prop);
        item.sub_prop = item.unack_sub_prop;
        item.unack_sub_prop = &.{};
    } else {
        clearPropSlice(alloc.gpa, &item.unack_sub_prop);
    }
    clearIntSlice(alloc.gpa, &item.lock_prop_index);

    try PlayerEchoComponent.saveAll(alloc.gpa, fs, echo_comp.player_id, echo_comp.echo_map);

    if (txn.message.Ack) {
        var changed_roles = try rolesEquippingEcho(echo_comp, alloc.gpa, txn.message.IncrId);
        defer changed_roles.deinit(alloc.gpa);
        if (changed_roles.count() != 0) {
            try pushRolePropUpdate(txn, alloc, assets, role_comp, echo_comp, weapon_comp, changed_roles, io);
            try refreshRoleEntities(txn, alloc, fs, assets, scene, role_comp, echo_comp, weapon_comp, query, changed_roles, buff_timers, io, now_ms);
        }
    }

    txn.respond(.{
        .ErrorCode = .Success,
        .UpdateInfo = try item.toProto(txn.message.IncrId, alloc.arena),
    });
}

fn validateMainPropPolish(
    assets: *const Assets,
    item: EchoInfo,
    item_config: Assets.DataTables.PhantomItem,
    prop_item_id: i32,
) ?pb.ErrorCode {
    _ = findMainPropertyWithProp(assets, item_config.MainProp.RandGroupId, prop_item_id) orelse {
        return .ErrPhantomBatchPolishPropNotMatch;
    };
    if (containsProp(item.main_prop, prop_item_id)) return .PhantomPolishSamePro;
    return null;
}

fn applyMainPropPolish(
    gpa: std.mem.Allocator,
    assets: *const Assets,
    item: *EchoInfo,
    item_config: Assets.DataTables.PhantomItem,
    prop_item_id: i32,
) !?pb.ErrorCode {
    const main_property = findMainPropertyWithProp(assets, item_config.MainProp.RandGroupId, prop_item_id) orelse {
        return .ErrPhantomBatchPolishPropNotMatch;
    };
    if (containsProp(item.main_prop, prop_item_id)) return .PhantomPolishSamePro;

    var props: std.ArrayList(EchoInfo.Prop) = .empty;
    errdefer props.deinit(gpa);
    try props.ensureTotalCapacity(gpa, main_property.PropGroup.len);
    for (main_property.PropGroup) |main_prop_item_id| {
        const prop_item = assets.tables.phantom_main_prop_item.getDataById(main_prop_item_id) orelse continue;
        props.appendAssumeCapacity(.{
            .id = prop_item.Id,
            .value = mainPropValue(assets, prop_item, item.level),
        });
    }

    replacePropSlice(gpa, &item.main_prop, try props.toOwnedSlice(gpa));
    clearPropSlice(gpa, &item.unack_sub_prop);
    clearIntSlice(gpa, &item.lock_prop_index);
    return null;
}

fn findMainPropertyWithProp(
    assets: *const Assets,
    rand_group_id: i32,
    prop_item_id: i32,
) ?Assets.DataTables.PhantomMainProperty {
    for (assets.tables.phantom_main_property.items) |entry| {
        if (entry.RandGroupId != rand_group_id) continue;
        for (entry.PropGroup) |entry_prop_id| {
            if (entry_prop_id == prop_item_id) return entry;
        }
    }
    return null;
}

fn mainPropValue(assets: *const Assets, prop: Assets.DataTables.PhantomMainPropItem, level: i32) i32 {
    var growth: i32 = 10000;
    for (assets.tables.phantom_growth.items) |entry| {
        if (entry.GrowthId == prop.GrowthId and entry.Level == level) {
            growth = entry.Value;
            break;
        }
    }
    return @divTrunc(prop.StandardProperty * growth, 10000);
}

fn validatedLockPropIndexes(
    gpa: std.mem.Allocator,
    prop_count: usize,
    indexes: []const i32,
) !std.array_hash_map.Auto(usize, void) {
    var lock_indices: std.array_hash_map.Auto(usize, void) = .empty;
    errdefer lock_indices.deinit(gpa);

    for (indexes) |index| {
        if (index < 0 or index >= @as(i32, @intCast(prop_count))) return error.InvalidLockIndex;
        const index_usize: usize = @intCast(index);
        if (lock_indices.contains(index_usize)) return error.DuplicateLockIndex;
        try lock_indices.put(gpa, index_usize, {});
    }

    return lock_indices;
}

fn randomSeed(fs: *FileSystem) u64 {
    var seed_bytes: [8]u8 = undefined;
    std.Io.random(fs.io, &seed_bytes);
    return std.mem.readInt(u64, &seed_bytes, .little);
}

fn buildVicePolishProps(
    gpa: std.mem.Allocator,
    assets: *const Assets,
    item: EchoInfo,
    item_config: Assets.DataTables.PhantomItem,
    lock_indices: std.array_hash_map.Auto(usize, void),
    random_seed: u64,
) ![]EchoInfo.Prop {
    var prng = std.Random.DefaultPrng.init(random_seed);
    const rng = prng.random();

    var props: std.ArrayList(EchoInfo.Prop) = .empty;
    errdefer props.deinit(gpa);
    try props.ensureTotalCapacity(gpa, item.sub_prop.len);

    var used_props: std.ArrayList(EchoInfo.Prop) = .empty;
    defer used_props.deinit(gpa);
    try used_props.ensureTotalCapacity(gpa, item.sub_prop.len);
    for (item.sub_prop, 0..) |old_prop, index| {
        if (lock_indices.contains(index)) used_props.appendAssumeCapacity(old_prop);
    }

    for (item.sub_prop, 0..) |old_prop, index| {
        if (lock_indices.contains(index)) {
            props.appendAssumeCapacity(old_prop);
        } else {
            const replacement = pickReplacementSubProp(
                assets,
                used_props.items,
                old_prop,
                item_config.QualityId,
                rng,
            );
            props.appendAssumeCapacity(replacement);
            used_props.appendAssumeCapacity(replacement);
        }
    }

    return props.toOwnedSlice(gpa);
}

fn pickReplacementSubProp(
    assets: *const Assets,
    current: []const EchoInfo.Prop,
    old_prop: EchoInfo.Prop,
    quality: i32,
    rng: std.Random,
) EchoInfo.Prop {
    const prop = EchoInfo.pickRandomSubProperty(assets, current, rng) orelse return old_prop;
    return .{ .id = prop.Id, .value = EchoInfo.randomSubPropValue(quality, prop, rng) };
}

fn propList(data: []const EchoInfo.Prop, arena: std.mem.Allocator) !std.ArrayList(pb.PhantomPropInfo) {
    var result: std.ArrayList(pb.PhantomPropInfo) = .empty;
    try result.ensureTotalCapacity(arena, data.len);
    for (data) |prop| {
        result.appendAssumeCapacity(.{ .PhantomPropId = prop.id, .Value = prop.value });
    }
    return result;
}

fn containsProp(props: []const EchoInfo.Prop, id: i32) bool {
    for (props) |prop| {
        if (prop.id == id) return true;
    }
    return false;
}

fn replacePropSlice(gpa: std.mem.Allocator, target: *[]EchoInfo.Prop, data: []EchoInfo.Prop) void {
    clearPropSlice(gpa, target);
    target.* = data;
}

fn clearPropSlice(gpa: std.mem.Allocator, target: *[]EchoInfo.Prop) void {
    if (target.*.len != 0) gpa.free(target.*);
    target.* = &.{};
}

fn replaceIntSlice(gpa: std.mem.Allocator, target: *[]i32, data: []const i32) !void {
    clearIntSlice(gpa, target);
    target.* = try gpa.dupe(i32, data);
}

fn clearIntSlice(gpa: std.mem.Allocator, target: *[]i32) void {
    if (target.*.len != 0) gpa.free(target.*);
    target.* = &.{};
}
