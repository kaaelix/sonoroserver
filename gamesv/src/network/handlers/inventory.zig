const std = @import("std");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const FileSystem = @import("common").FileSystem;
const Assets = @import("../../data/Assets.zig");
const GiftPackage = Assets.DataTables.GiftPackage;
const comp_util = @import("../../logic/component/comp_util.zig");
const RoleInfo = @import("../../fs/RoleInfo.zig");
const Transaction = @import("../handlers.zig").Transaction;
const PlayerBasicComponent = @import("../../logic/component/player/PlayerBasicComponent.zig");
const PlayerEchoComponent = @import("../../logic/component/player/PlayerEchoComponent.zig");
const PlayerInventoryComponent = @import("../../logic/component/player/PlayerInventoryComponent.zig");
const PlayerRoleComponent = @import("../../logic/component/player/PlayerRoleComponent.zig");
const PlayerWeaponComponent = @import("../../logic/component/player/PlayerWeaponComponent.zig");
const ConsumeHelper = @import("../../logic/helpers/consume.zig");
const EnergyHelper = @import("../../logic/helpers/energy.zig");
const QuestEngine = @import("../../logic/helpers/quest_engine.zig");
const LevelPlayHelper = @import("../../logic/helpers/level_play.zig");
const RewardService = @import("../../logic/helpers/reward.zig");
const RoleHelper = @import("../../logic/helpers/role.zig");
const RoleStats = @import("../../logic/helpers/role_stats.zig");
const PlayerProgressComponent = @import("../../logic/component/player/PlayerProgressComponent.zig");

pub fn onNormalItemRequest(
    txn: *Transaction(pb.NormalItemRequest),
    alloc: mem.Alloc,
    basic_comp: *const PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
) !void {
    var normal_items = try inventory_comp.info.normalItemList(alloc.arena);
    try appendNormalItem(&normal_items, alloc.arena, 2, basic_comp.info.attributes.coin);
    try appendNormalItem(&normal_items, alloc.arena, 3, basic_comp.info.attributes.rare_coin);
    txn.respond(.{ .NormalItemList = normal_items });
}

fn appendNormalItem(list: *std.ArrayList(pb.NormalItem), arena: std.mem.Allocator, id: i32, count: i32) !void {
    if (count <= 0) return;

    for (list.items) |*item| {
        if (item.Id != id) continue;
        item.Count = count;
        return;
    }

    try list.append(arena, .{ .Id = id, .Count = count, .ExpireTime = 0 });
}

pub fn onItemLockRequest(
    txn: *Transaction(pb.ItemLockRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    io: std.Io,
) !void {
    const func_value: i32 = if (txn.message.ItemId == 1) 1 else 0;

    if (weapon_comp.weapon_map.getPtr(txn.message.IncrId)) |item| {
        item.func_value = func_value;
        try PlayerWeaponComponent.saveAll(alloc.gpa, fs, weapon_comp.player_id, weapon_comp.weapon_map);
        try txn.conn.push(alloc.gpa, io, pb.ItemFuncValueUpdateNotify{
            .IncrId = txn.message.IncrId,
            .FuncValue = item.func_value,
        });
        txn.respond(.{ .ErrorCode = .Success });
        return;
    }

    const item = echo_comp.echo_map.getPtr(txn.message.IncrId) orelse {
        txn.respond(.{ .ErrorCode = .ErrPhantomItemNotExist });
        return;
    };

    item.func_value = func_value;
    try PlayerEchoComponent.saveAll(alloc.gpa, fs, echo_comp.player_id, echo_comp.echo_map);
    try txn.conn.push(alloc.gpa, io, pb.ItemFuncValueUpdateNotify{
        .IncrId = txn.message.IncrId,
        .FuncValue = item.func_value,
    });

    txn.respond(.{ .ErrorCode = .Success });
}

fn applyItemFuncValueUpdate(
    conn: anytype,
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    echo_comp: *PlayerEchoComponent,
    weapon_comp: *PlayerWeaponComponent,
    incr_id: i32,
    new_func: i32,
) !pb.ErrorCode {
    if (echo_comp.echo_map.getPtr(incr_id)) |item| {
        item.func_value = new_func;
        try PlayerEchoComponent.saveAll(alloc.gpa, fs, echo_comp.player_id, echo_comp.echo_map);
    } else if (weapon_comp.weapon_map.getPtr(incr_id)) |weapon| {
        weapon.func_value = new_func;
        try PlayerWeaponComponent.saveAll(alloc.gpa, fs, weapon_comp.player_id, weapon_comp.weapon_map);
    } else {
        return .ErrPhantomItemNotExist;
    }

    try conn.push(alloc.gpa, io, pb.ItemFuncValueUpdateNotify{
        .IncrId = incr_id,
        .FuncValue = new_func,
    });
    return .Success;
}

pub fn onItemDeprecateRequest(
    txn: *Transaction(pb.ItemDeprecateRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    echo_comp: *PlayerEchoComponent,
    weapon_comp: *PlayerWeaponComponent,
) !void {
    const new_func: i32 = if (txn.message.ItemId == 1) 2 else 0;
    const err = try applyItemFuncValueUpdate(txn.conn, alloc, io, fs, echo_comp, weapon_comp, txn.message.IncrId, new_func);
    txn.respond(.{ .ErrorCode = err });
}

pub fn onItemDisuseRequest(
    txn: *Transaction(pb.ItemDisuseRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    echo_comp: *PlayerEchoComponent,
    weapon_comp: *PlayerWeaponComponent,
) !void {
    const new_func: i32 = if (txn.message.Oper == 1) 2 else 0;
    const err = try applyItemFuncValueUpdate(txn.conn, alloc, io, fs, echo_comp, weapon_comp, txn.message.IncrId, new_func);
    txn.respond(.{ .ErrCode = err });
}

fn findGiftPackage(assets: *const Assets, item_cfg: Assets.DataTables.ItemInfo) ?GiftPackage {
    inline for (.{ 2, 4, 8 }) |key| {
        if (item_cfg.Parameters.map.get(key)) |pkg_id| {
            if (assets.tables.gift_package.getDataById(pkg_id)) |pkg| return pkg;
        }
    }
    return null;
}

fn rollRandomUsize(io: std.Io, upper: usize) usize {
    if (upper <= 1) return 0;
    var bytes: [8]u8 = undefined;
    std.Io.random(io, &bytes);
    return @intCast(std.mem.readInt(u64, &bytes, .little) % @as(u64, @intCast(upper)));
}

fn expandGiftPackage(
    io: std.Io,
    arena: std.mem.Allocator,
    pkg: GiftPackage,
    open_count: i32,
) !std.ArrayList(RewardService.RewardItem) {
    var out: std.ArrayList(RewardService.RewardItem) = .empty;
    const total_entries = pkg.Content.map.count();
    if (total_entries == 0 or open_count <= 0) return out;

    if (pkg.Type == 0) {
        var it = pkg.Content.map.iterator();
        while (it.next()) |kv| {
            if (kv.key_ptr.* <= 0 or kv.value_ptr.* <= 0) continue;
            try out.append(arena, .{
                .item_id = kv.key_ptr.*,
                .count = kv.value_ptr.* * open_count,
            });
        }
        return out;
    }

    var pool_ids: std.ArrayList(i32) = .empty;
    var pool_counts: std.ArrayList(i32) = .empty;
    var pool_weights: std.ArrayList(i32) = .empty;
    var it = pkg.Content.map.iterator();
    while (it.next()) |kv| {
        if (kv.key_ptr.* <= 0 or kv.value_ptr.* <= 0) continue;
        try pool_ids.append(arena, kv.key_ptr.*);
        try pool_counts.append(arena, kv.value_ptr.*);
        const w = pkg.Weight.map.get(kv.key_ptr.*) orelse 1;
        try pool_weights.append(arena, @max(1, w));
    }
    if (pool_ids.items.len == 0) return out;

    const picks_per_open: usize = @intCast(@max(1, pkg.AvailableNum));
    var c: i32 = 0;
    while (c < open_count) : (c += 1) {
        if (pkg.Type == 1 or pkg.Type == 5 or pkg.Type == 6) {
            // Selectable package (pb.ItemUseRequest has no choice index): grant random distinct options up to AvailableNum.
            var picked = try arena.alloc(bool, pool_ids.items.len);
            @memset(picked, false);
            const target = @min(picks_per_open, pool_ids.items.len);
            var n: usize = 0;
            while (n < target) : (n += 1) {
                const start_idx = rollRandomUsize(io, pool_ids.items.len);
                var step: usize = 0;
                while (step < pool_ids.items.len) : (step += 1) {
                    const idx = (start_idx + step) % pool_ids.items.len;
                    if (!picked[idx]) {
                        picked[idx] = true;
                        try out.append(arena, .{ .item_id = pool_ids.items[idx], .count = pool_counts.items[idx] });
                        break;
                    }
                }
            }
        } else {
            var total_weight: usize = 0;
            for (pool_weights.items) |w| total_weight += @intCast(w);
            var n: usize = 0;
            while (n < picks_per_open) : (n += 1) {
                var target = if (total_weight > 0) rollRandomUsize(io, total_weight) else 0;
                var chosen_idx: usize = 0;
                for (pool_weights.items, 0..) |w, idx| {
                    const uw: usize = @intCast(w);
                    if (target < uw) {
                        chosen_idx = idx;
                        break;
                    }
                    target -= uw;
                }
                try out.append(arena, .{
                    .item_id = pool_ids.items[chosen_idx],
                    .count = pool_counts.items[chosen_idx],
                });
            }
        }
    }

    return out;
}

fn roleWavebandId(assets: *const Assets, role_id: i32) ?i32 {
    const role = assets.tables.role_info.getDataById(role_id) orelse return null;
    if (role.ResonantChainGroupId == 0) return null;
    for (assets.tables.resonant_chain.items) |chain| {
        if (chain.GroupId != role.ResonantChainGroupId or chain.GroupIndex != 1) continue;
        var it = chain.ActivateConsume.map.iterator();
        if (it.next()) |kv| return kv.key_ptr.*;
        return null;
    }
    return null;
}

fn grantGiftRewards(
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    assets: *const Assets,
    conn: *@import("../Connection.zig"),
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    role_comp: *PlayerRoleComponent,
    raw_items: []const RewardService.RewardItem,
) !void {
    var normal_items: std.ArrayList(RewardService.RewardItem) = .empty;
    var starter_weapons: std.ArrayList(pb.WeaponItem) = .empty;
    var has_new_roles = false;

    for (raw_items) |entry| {
        if (entry.item_id <= 0 or entry.count <= 0) continue;
        if (assets.tables.role_info.getDataById(entry.item_id) != null) {
            var n: i32 = 0;
            while (n < entry.count) : (n += 1) {
                if (role_comp.role_map.contains(entry.item_id)) {
                    const wb = roleWavebandId(assets, entry.item_id) orelse 50003;
                    try normal_items.append(alloc.arena, .{ .item_id = wb, .count = if (wb == 50003) 3 else 1 });
                } else {
                    const new_role = try RoleInfo.createDefault(alloc.gpa, assets, entry.item_id);
                    try role_comp.role_map.put(alloc.gpa, entry.item_id, new_role);
                    const role_path = try std.fmt.allocPrint(alloc.arena, "player/{}/role/{}", .{ role_comp.player_id, entry.item_id });
                    try comp_util.saveStruct(fs, new_role, role_path, alloc.arena);
                    if (RoleHelper.ensureRoleWeapon(alloc, fs, assets, role_comp, weapon_comp, entry.item_id) catch false) {
                        if (role_comp.role_map.get(entry.item_id)) |rinfo| {
                            if (weapon_comp.weapon_map.get(rinfo.weapon)) |witem| {
                                try starter_weapons.append(alloc.arena, witem.toProto(rinfo.weapon));
                            }
                        }
                    }
                    has_new_roles = true;
                }
            }
        } else {
            try normal_items.append(alloc.arena, entry);
        }
    }

    if (starter_weapons.items.len > 0) {
        try conn.push(alloc.gpa, io, pb.WeaponItemAddNotify{
            .WeaponItemList = starter_weapons,
            .AddFromRole = true,
            .Reason = 1,
        });
    }

    if (normal_items.items.len > 0) {
        _ = try RewardService.grantItems(.{
            .alloc = alloc,
            .io = io,
            .fs = fs,
            .assets = assets,
            .conn = conn,
            .basic_comp = basic_comp,
            .inventory_comp = inventory_comp,
            .weapon_comp = weapon_comp,
            .echo_comp = echo_comp,
        }, 0, true, normal_items.items, 1);
    }

    if (has_new_roles) {
        const selected_main_role = RoleHelper.selectedMainRoleId(
            assets,
            role_comp,
            basic_comp.info.attributes.sex,
            basic_comp.info.selected_main_role_id,
        );
        var notify: pb.PbGetRoleListNotify = .default;
        try notify.RoleList.ensureTotalCapacity(alloc.arena, role_comp.role_map.count());
        var it = role_comp.role_map.iterator();
        while (it.next()) |role| {
            if (role.key_ptr.* > 1999 or role.key_ptr.* < 1000) continue;
            if (selected_main_role) |selected| {
                if (RoleHelper.isMainRole(assets, role.key_ptr.*) and role.key_ptr.* != selected) continue;
            }
            notify.RoleList.appendAssumeCapacity(try RoleStats.toClientRoleInfo(
                alloc.gpa,
                alloc.arena,
                assets,
                role_comp,
                role.key_ptr.*,
                role.value_ptr,
                weapon_comp,
                echo_comp,
            ));
        }
        try conn.push(alloc.gpa, io, notify);
    }
}

pub fn onItemUseRequest(
    txn: *Transaction(pb.ItemUseRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    assets: *const Assets,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    role_comp: *PlayerRoleComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    const item_id = txn.message.ItemId;
    const count = txn.message.Count;
    if (item_id <= 0 or count <= 0) {
        txn.respond(.{ .ErrCode = .ErrItemCount });
        return;
    }

    const item_cfg = assets.tables.item_info.getDataById(item_id);
    if (item_cfg) |cfg| {
        if (cfg.UseLevel > 0 and basic_comp.info.attributes.level < cfg.UseLevel) {
            txn.respond(.{ .ErrCode = .ErrItemUseLevelLimit });
            return;
        }
        if (cfg.UseCountLimit > 0 and count > cfg.UseCountLimit) {
            txn.respond(.{ .ErrCode = .ErrGiftPackUseLimit });
            return;
        }

        // Case 1: Waveplate Crystal Solvent (Parameters['10'] == 1, item 70000002)
        if (cfg.Parameters.map.get(10)) |strength_kind| {
            if (strength_kind == 1) {
                const costs = [_]ConsumeHelper.CostItem{.{ .id = item_id, .count = count }};
                if (!(try ConsumeHelper.hasCosts(alloc.arena, basic_comp, inventory_comp, &costs))) {
                    txn.respond(.{ .ErrCode = .ErrItemNotEnough });
                    return;
                }
                const waveplate_gain = 60 * count;
                if (!EnergyHelper.canAddWaveplate(basic_comp, io, waveplate_gain)) {
                    txn.respond(.{ .ErrCode = .ErrEnergyMaxCharge });
                    return;
                }
                if (!(try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, &costs, io))) {
                    txn.respond(.{ .ErrCode = .ErrItemNotEnough });
                    return;
                }
                _ = try EnergyHelper.addWaveplate(txn.conn, alloc, fs, basic_comp, waveplate_gain, io);
                try QuestEngine.onItemUse(qctx, item_id, count);
                txn.respond(.{ .ErrCode = .Success });
                return;
            }
        }

        // Case 2: Gift Package / Morphable Echo / Supply Chest (Parameters['2'/'4'/'8'] -> GiftPackage)
        if (findGiftPackage(assets, cfg)) |pkg| {
            const costs = [_]ConsumeHelper.CostItem{.{ .id = item_id, .count = count }};
            if (!(try ConsumeHelper.hasCosts(alloc.arena, basic_comp, inventory_comp, &costs))) {
                txn.respond(.{ .ErrCode = .ErrItemNotEnough });
                return;
            }
            const expanded = try expandGiftPackage(io, alloc.arena, pkg, count);
            if (!(try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, &costs, io))) {
                txn.respond(.{ .ErrCode = .ErrItemNotEnough });
                return;
            }
            try grantGiftRewards(alloc, io, fs, assets, txn.conn, basic_comp, inventory_comp, weapon_comp, echo_comp, role_comp, expanded.items);
            try QuestEngine.onItemUse(qctx, item_id, count);
            txn.respond(.{ .ErrCode = .Success });
            return;
        }

        // Case 3: ConvertItems mapping
        if (cfg.ConvertItems.map.count() > 0) {
            const costs = [_]ConsumeHelper.CostItem{.{ .id = item_id, .count = count }};
            if (!(try ConsumeHelper.hasCosts(alloc.arena, basic_comp, inventory_comp, &costs))) {
                txn.respond(.{ .ErrCode = .ErrItemNotEnough });
                return;
            }
            var rewards: std.ArrayList(RewardService.RewardItem) = .empty;
            var it = cfg.ConvertItems.map.iterator();
            while (it.next()) |kv| {
                if (kv.key_ptr.* > 0 and kv.value_ptr.* > 0) {
                    try rewards.append(alloc.arena, .{ .item_id = kv.key_ptr.*, .count = kv.value_ptr.* * count });
                }
            }
            if (!(try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, &costs, io))) {
                txn.respond(.{ .ErrCode = .ErrItemNotEnough });
                return;
            }
            try grantGiftRewards(alloc, io, fs, assets, txn.conn, basic_comp, inventory_comp, weapon_comp, echo_comp, role_comp, rewards.items);
            try QuestEngine.onItemUse(qctx, item_id, count);
            txn.respond(.{ .ErrCode = .Success });
            return;
        }

        // Regular non-quest consumable present in inventory
        if (cfg.ItemType != 7 and cfg.ItemType != 8 and cfg.ItemType != 13 and cfg.ShowUseButton) {
            const costs = [_]ConsumeHelper.CostItem{.{ .id = item_id, .count = count }};
            if (!(try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, &costs, io))) {
                txn.respond(.{ .ErrCode = .ErrItemNotEnough });
                return;
            }
        }
    }

    try QuestEngine.onItemUse(qctx, item_id, count);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onItemGiftUseRequest(
    txn: *Transaction(pb.ItemGiftUseRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    assets: *const Assets,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    role_comp: *PlayerRoleComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    const item_id = txn.message.ItemId;
    const count = txn.message.Count;
    if (item_id <= 0 or count <= 0) {
        txn.respond(.{ .ErrorCode = .ErrItemCount });
        return;
    }

    const costs = [_]ConsumeHelper.CostItem{.{ .id = item_id, .count = count }};
    if (!(try ConsumeHelper.hasCosts(alloc.arena, basic_comp, inventory_comp, &costs))) {
        txn.respond(.{ .ErrorCode = .ErrItemNotEnough });
        return;
    }

    var rewards: std.ArrayList(RewardService.RewardItem) = .empty;
    if (assets.tables.item_info.getDataById(item_id)) |cfg| {
        if (findGiftPackage(assets, cfg)) |pkg| {
            if (txn.message.SelectedId.items.len > 0) {
                for (txn.message.SelectedId.items) |sel_id| {
                    if (sel_id <= 0) continue;
                    const unit_count = pkg.Content.map.get(sel_id) orelse 1;
                    try rewards.append(alloc.arena, .{ .item_id = sel_id, .count = @max(1, unit_count) * count });
                }
            }
            if (rewards.items.len == 0) {
                rewards = try expandGiftPackage(io, alloc.arena, pkg, count);
            }
        }
    }
    if (rewards.items.len == 0) {
        for (txn.message.SelectedId.items) |sel_id| {
            if (sel_id > 0) try rewards.append(alloc.arena, .{ .item_id = sel_id, .count = count });
        }
    }

    if (!(try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, &costs, io))) {
        txn.respond(.{ .ErrorCode = .ErrItemNotEnough });
        return;
    }

    try grantGiftRewards(alloc, io, fs, assets, txn.conn, basic_comp, inventory_comp, weapon_comp, echo_comp, role_comp, rewards.items);
    try QuestEngine.onItemUse(qctx, item_id, count);

    var infos: std.ArrayList(pb.AddCountItemInfo) = .empty;
    for (rewards.items) |r| {
        try infos.append(alloc.arena, .{ .Id = r.item_id, .Count = r.count });
    }
    txn.respond(.{ .ErrorCode = .Success, .Infos = infos });
}

fn calcDecomposeRewards(
    assets: *const Assets,
    weapon_comp: *const PlayerWeaponComponent,
    echo_comp: *const PlayerEchoComponent,
    items: []const pb.DecomposeItemInfo,
    arena: std.mem.Allocator,
) !std.ArrayList(RewardService.RewardItem) {
    var rewards: std.ArrayList(RewardService.RewardItem) = .empty;
    for (items) |entry| {
        if (entry.IncrId > 0) {
            if (weapon_comp.weapon_map.get(entry.IncrId)) |w| {
                const tier = @max(1, w.level);
                try rewards.append(arena, .{ .item_id = 43010001, .count = @max(1, @divFloor(tier, 2)) });
                try rewards.append(arena, .{ .item_id = 2, .count = tier * 100 });
            } else if (echo_comp.echo_map.get(entry.IncrId)) |e| {
                const quality = if (assets.tables.phantom_item.getDataById(e.id)) |p| @max(1, p.QualityId) else 2;
                try rewards.append(arena, .{ .item_id = 41100011, .count = @max(1, quality) });
                try rewards.append(arena, .{ .item_id = 2, .count = quality * 250 + e.level * 100 });
            }
        } else if (entry.ItemId > 0 and entry.Count > 0) {
            try rewards.append(arena, .{ .item_id = 2, .count = entry.Count * 100 });
        }
    }
    return rewards;
}

pub fn onItemDecomposePreviewRequest(
    txn: *Transaction(pb.ItemDecomposePreviewRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
    weapon_comp: *const PlayerWeaponComponent,
    echo_comp: *const PlayerEchoComponent,
) !void {
    const rewards = try calcDecomposeRewards(assets, weapon_comp, echo_comp, txn.message.ItemList.items, alloc.arena);
    var item_map: std.ArrayList(pb.MapEntry(i32, i32)) = .empty;
    for (rewards.items) |r| {
        try item_map.append(alloc.arena, .{ .key = r.item_id, .value = r.count });
    }
    txn.respond(.{ .ErrorCode = .Success, .ItemMap = item_map });
}

pub fn onItemDecomposeRequest(
    txn: *Transaction(pb.ItemDecomposeRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    assets: *const Assets,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    if (txn.message.ItemList.items.len == 0) {
        txn.respond(.{ .ErrCode = .ErrItemCount });
        return;
    }

    var seen_incr: std.AutoHashMapUnmanaged(i32, void) = .empty;
    defer seen_incr.deinit(alloc.arena);
    var costs: std.ArrayList(ConsumeHelper.CostItem) = .empty;
    var incr_weapons: std.ArrayList(i32) = .empty;
    var incr_echoes: std.ArrayList(i32) = .empty;

    for (txn.message.ItemList.items) |entry| {
        if (entry.IncrId > 0) {
            if ((try seen_incr.getOrPut(alloc.arena, entry.IncrId)).found_existing) {
                txn.respond(.{ .ErrCode = .ErrPhantomConsumeItemIncrDuplicate });
                return;
            }
            if (weapon_comp.weapon_map.get(entry.IncrId)) |w| {
                if (w.func_value == 1 or w.role_id != null) {
                    txn.respond(.{ .ErrCode = if (w.role_id != null) .ErrPhantomDecomposeEquiped else .ErrPhantomLevelUpMaterialLock });
                    return;
                }
                try incr_weapons.append(alloc.arena, entry.IncrId);
            } else if (echo_comp.echo_map.get(entry.IncrId)) |e| {
                if (e.func_value == 1 or e.role_id != null) {
                    txn.respond(.{ .ErrCode = if (e.role_id != null) .ErrPhantomDecomposeEquiped else .ErrPhantomLevelUpMaterialLock });
                    return;
                }
                try incr_echoes.append(alloc.arena, entry.IncrId);
            } else {
                txn.respond(.{ .ErrCode = .ErrPhantomItemNotExist });
                return;
            }
        } else if (entry.ItemId > 0 and entry.Count > 0) {
            try costs.append(alloc.arena, .{ .id = entry.ItemId, .count = entry.Count });
        } else {
            txn.respond(.{ .ErrCode = .ErrItemCount });
            return;
        }
    }

    const rewards = try calcDecomposeRewards(assets, weapon_comp, echo_comp, txn.message.ItemList.items, alloc.arena);

    if (costs.items.len > 0) {
        if (!(try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, costs.items, io))) {
            txn.respond(.{ .ErrCode = .ErrItemNotEnough });
            return;
        }
    }

    for (incr_weapons.items) |incr_id| {
        if (weapon_comp.weapon_map.fetchOrderedRemove(incr_id)) |removed| {
            var w_old = removed.value;
            w_old.deinit(alloc.gpa);
        }
        try PlayerWeaponComponent.deleteWeaponFile(alloc.gpa, fs, weapon_comp.player_id, incr_id);
    }
    for (incr_echoes.items) |incr_id| {
        if (echo_comp.echo_map.fetchOrderedRemove(incr_id)) |removed| {
            var e_old = removed.value;
            e_old.deinit(alloc.gpa);
        }
        try PlayerEchoComponent.deleteEchoFile(alloc.gpa, fs, echo_comp.player_id, incr_id);
    }

    if (incr_weapons.items.len > 0) {
        try PlayerWeaponComponent.saveAll(alloc.gpa, fs, weapon_comp.player_id, weapon_comp.weapon_map);
        try txn.conn.push(alloc.gpa, io, pb.WeaponItemRemoveNotify{ .WeaponItemIncrIdList = incr_weapons });
    }
    if (incr_echoes.items.len > 0) {
        try PlayerEchoComponent.saveAll(alloc.gpa, fs, echo_comp.player_id, echo_comp.echo_map);
        try txn.conn.push(alloc.gpa, io, pb.PhantomItemRemoveNotify{ .PhantomItemIncrIdList = incr_echoes });
    }
    if (rewards.items.len > 0) {
        _ = try RewardService.grantItems(reward_ctx, 0, true, rewards.items, 1);
    }
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onGiftPackPreviewRequest(
    txn: *Transaction(pb.GiftPackPreviewRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
) !void {
    var preview: std.ArrayList(pb.MapEntry(i32, pb.PhantomItem)) = .empty;
    if (assets.tables.gift_package.getDataById(txn.message.GiftPackId)) |pkg| {
        var it = pkg.Content.map.iterator();
        while (it.next()) |kv| {
            if (assets.tables.phantom_item.getDataById(kv.key_ptr.*) != null) {
                try preview.append(alloc.arena, .{
                    .key = kv.key_ptr.*,
                    .value = .{ .Id = kv.key_ptr.*, .PhantomLevel = 0 },
                });
            }
        }
    }
    txn.respond(.{ .PhantomPreview = preview });
}

pub fn onGetInstExchangeRewardRequest(
    txn: *Transaction(pb.GetInstExchangeRewardRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    basic_comp: *PlayerBasicComponent,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const cnt = txn.message.ExchangeCount;
    if (cnt <= 0) {
        txn.respond(.{ .ErrorCode = .RequestParamError });
        return;
    }
    const waveplate_cost = std.math.mul(i32, cnt, 60) catch {
        txn.respond(.{ .ErrorCode = .RequestParamError });
        return;
    };
    if (!(try EnergyHelper.consumeWaveplate(txn.conn, alloc, fs, basic_comp, waveplate_cost, io))) {
        txn.respond(.{ .ErrorCode = .ErrItemNotEnough });
        return;
    }
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "inst_exchange_count", cnt);
    try progress_comp.save(fs, alloc.arena);
    _ = try RewardService.grantItems(reward_ctx, 0, true, &.{
        .{ .item_id = 3, .count = cnt * 20 },
        .{ .item_id = 2, .count = cnt * 5000 },
    }, 1);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onHandInItemRequest(
    txn: *Transaction(pb.HandInItemRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    var costs: std.ArrayList(ConsumeHelper.CostItem) = .empty;
    for (txn.message.HandInItem.items) |hand_in| {
        for (hand_in.HandItemInfo.items) |info| {
            if (info.ItemId > 0 and info.Num > 0) {
                try costs.append(alloc.arena, .{ .id = info.ItemId, .count = info.Num });
            }
        }
    }
    if (costs.items.len > 0) {
        if (!(try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, costs.items, io))) {
            txn.respond(.{ .ErrorId = .ErrItemNotEnough });
            return;
        }
    }
    if (txn.message.NodeId > 0) {
        _ = try QuestEngine.onSubmitNode(qctx, @intCast(@max(0, txn.message.TreeOwnerId)), txn.message.TreeIncId, txn.message.NodeId);
    }
    txn.respond(.{ .ErrorId = .Success });
}

pub fn onNewHandInItemRequest(
    txn: *Transaction(pb.NewHandInItemRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    progress_comp: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    if (txn.message.ProjectionMachine) |pm| {
        if (pm.ItemId > 0) {
            const cost = [_]ConsumeHelper.CostItem{.{ .id = pm.ItemId, .count = 1 }};
            if (!(try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, &cost, io))) {
                txn.respond(.{ .ErrorCode = .ErrItemNotEnough });
                return;
            }
            var buf: [48]u8 = undefined;
            const k = try std.fmt.bufPrint(&buf, "handin_pm:{d}", .{pm.EntityCfgId});
            try progress_comp.info.setKeyInt(alloc.gpa, k, pm.ItemId);
            _ = try progress_comp.info.addKeyInt(alloc.gpa, "handin_pm:cnt", 1);
            try progress_comp.save(fs, alloc.arena);
            if (LevelPlayHelper.fromQuestContext(qctx)) |lp_ctx| {
                try LevelPlayHelper.onConditionEvent(lp_ctx, progress_comp, .new_hand_in_items, pm.EntityCfgId, pm.ItemId);
            }
            try QuestEngine.onConditionStateChanged(qctx);
        }
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onHandInMingSuRequest(
    txn: *Transaction(pb.HandInMingSuRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    assets: *const Assets,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    progress_comp: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    const pool = assets.tables.dragon_pool.getDataById(txn.message.DragonPoolId) orelse {
        txn.respond(.{ .ErrorCode = .RequestParamError });
        return;
    };
    var lvl_buf: [48]u8 = undefined;
    const lvl_k = try std.fmt.bufPrint(&lvl_buf, "mingsu:lvl:{d}", .{txn.message.DragonPoolId});
    var inj_buf: [48]u8 = undefined;
    const inj_k = try std.fmt.bufPrint(&inj_buf, "mingsu:inj:{d}", .{txn.message.DragonPoolId});
    const cur_lvl: usize = @intCast(@max(0, progress_comp.info.getKeyInt(lvl_k)));
    if (cur_lvl >= pool.Goal.len) {
        txn.respond(.{
            .ErrorCode = .Success,
            .DragonPoolId = txn.message.DragonPoolId,
            .Level = @intCast(cur_lvl),
            .InjectedCoreItemCount = @intCast(@max(0, progress_comp.info.getKeyInt(inj_k))),
            .LevelGains = 0,
        });
        return;
    }

    const core_id = pool.CoreId;
    if (core_id > 0) {
        const owned = inventory_comp.info.normalItemCount(core_id);
        if (owned <= 0) {
            txn.respond(.{ .ErrorCode = .ErrItemNotEnough });
            return;
        }
        const cur_inj: i32 = @intCast(@max(0, progress_comp.info.getKeyInt(inj_k)));
        const next_goal = pool.Goal[cur_lvl];
        const needed = @max(1, next_goal - cur_inj);
        const take = @min(owned, needed);
        const cost = [_]ConsumeHelper.CostItem{.{ .id = core_id, .count = take }};
        if (!(try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, &cost, io))) {
            txn.respond(.{ .ErrorCode = .ErrItemNotEnough });
            return;
        }
        const total_inj = cur_inj + take;
        try progress_comp.info.setKeyInt(alloc.gpa, inj_k, total_inj);
        var new_lvl = cur_lvl;
        while (new_lvl < pool.Goal.len and total_inj >= pool.Goal[new_lvl]) {
            new_lvl += 1;
        }
        const gains: i32 = @intCast(new_lvl - cur_lvl);
        try progress_comp.info.setKeyInt(alloc.gpa, lvl_k, @intCast(new_lvl));
        try progress_comp.save(fs, alloc.arena);
        if (gains > 0) {
            QuestEngine.onConditionStateChanged(qctx) catch |err| QuestEngine.logHookErr("onHandInMingSu", err);
        }
        txn.respond(.{
            .ErrorCode = .Success,
            .DragonPoolId = txn.message.DragonPoolId,
            .Level = @intCast(new_lvl),
            .InjectedCoreItemCount = total_inj,
            .LevelGains = gains,
        });
        return;
    }

    const next_lvl = @as(i32, @intCast(try progress_comp.info.addKeyInt(alloc.gpa, lvl_k, 1)));
    try progress_comp.save(fs, alloc.arena);
    QuestEngine.onConditionStateChanged(qctx) catch |err| QuestEngine.logHookErr("onHandInMingSu", err);
    txn.respond(.{
        .ErrorCode = .Success,
        .DragonPoolId = txn.message.DragonPoolId,
        .Level = next_lvl,
        .InjectedCoreItemCount = next_lvl,
        .LevelGains = 1,
    });
}

pub fn onMingSuHandRewardRequest(
    txn: *Transaction(pb.MingSuHandRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const pool = assets.tables.dragon_pool.getDataById(txn.message.DragonPoolId);
    var buf: [48]u8 = undefined;
    const lvl_k = try std.fmt.bufPrint(&buf, "mingsu:lvl:{d}", .{txn.message.DragonPoolId});
    const cur_lvl = @as(i32, @intCast(@max(0, progress_comp.info.getKeyInt(lvl_k))));
    if (cur_lvl <= 0) {
        txn.respond(.{ .ErrorCode = .RequestParamError, .LevelGains = 0 });
        return;
    }
    var clm_buf: [48]u8 = undefined;
    const clm_k = try std.fmt.bufPrint(&clm_buf, "mingsu:clm:{d}:{d}", .{ txn.message.DragonPoolId, cur_lvl });
    if (try progress_comp.info.markClaim(alloc.gpa, clm_k)) {
        try progress_comp.save(fs, alloc.arena);
        const drop_id = if (pool) |p| blk: {
            const idx: usize = @intCast(cur_lvl - 1);
            break :blk if (idx < p.DropIds.len) p.DropIds[idx] else 0;
        } else 0;
        if (drop_id > 0) {
            _ = try RewardService.grantDropPackagePopup(reward_ctx, drop_id, 19500);
        } else {
            _ = try RewardService.grantItems(reward_ctx, 0, true, &.{
                .{ .item_id = 3, .count = 50 },
                .{ .item_id = 2, .count = 15000 },
            }, 19500);
        }
    }
    txn.respond(.{ .ErrorCode = .Success, .LevelGains = cur_lvl });
}
