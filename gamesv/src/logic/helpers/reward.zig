const std = @import("std");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const common = @import("common");
const FileSystem = common.FileSystem;
const Connection = @import("../../network/Connection.zig");
const Assets = @import("../../data/Assets.zig");
const comp_util = @import("../component/comp_util.zig");
const PlayerBasicComponent = @import("../component/player/PlayerBasicComponent.zig");
const PlayerInventoryComponent = @import("../component/player/PlayerInventoryComponent.zig");
const PlayerWeaponComponent = @import("../component/player/PlayerWeaponComponent.zig");
const PlayerEchoComponent = @import("../component/player/PlayerEchoComponent.zig");
const InventoryInfo = @import("../../fs/InventoryInfo.zig");
const WeaponItem = @import("../../fs/WeaponItem.zig");
const EchoInfo = @import("../../fs/EchoInfo.zig");
const special_item_incr = @import("../../fs/special_item_incr.zig");
const CalabashHelper = @import("calabash.zig");
const EnergyHelper = @import("energy.zig");
const MoraleHelper = @import("morale.zig");
const PlayerProgressComponent = @import("../component/player/PlayerProgressComponent.zig");

pub const RewardItem = struct {
    item_id: i32,
    count: i32,
};

pub const RewardContext = struct {
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    assets: *const Assets,
    conn: *Connection,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    progress_comp: ?*PlayerProgressComponent = null,
};

pub const GrantResult = struct {
    leveled_up: bool = false,
    new_level: i32 = 0,
    granted_count: usize = 0,
};

pub fn grantDropPackage(ctx: RewardContext, drop_id: i32, reason: i32) !GrantResult {
    return try grantDropPackageInternal(ctx, drop_id, reason, null);
}

pub fn grantDropPackagePopup(ctx: RewardContext, drop_id: i32, reason: i32) !GrantResult {
    return try grantDropPackageInternal(ctx, drop_id, reason, true);
}

fn grantDropPackageInternal(ctx: RewardContext, drop_id: i32, reason: i32, force_popup: ?bool) !GrantResult {
    if (drop_id <= 0) return .{};

    const pkg = ctx.assets.tables.drop_package.getDataById(drop_id) orelse {
        std.log.scoped(.reward).debug("DropPackage {d} not found, skipping", .{drop_id});
        return .{};
    };

    var grants: std.ArrayList(RewardItem) = .empty;
    var preview_it = pkg.DropPreview.map.iterator();
    while (preview_it.next()) |kv| {
        const item_id = kv.key_ptr.*;
        const raw_count = kv.value_ptr.*;
        if (item_id <= 0) continue;
        const count: i32 = if (raw_count > 0)
            raw_count
        else if (item_id == 1)
            50
        else if (item_id == 2)
            200
        else if (item_id == 3)
            10
        else
            1;
        try grants.append(ctx.alloc.arena, .{ .item_id = item_id, .count = count });
    }

    if (grants.items.len == 0) {
        return .{};
    }

    return try grantItems(ctx, drop_id, force_popup orelse pkg.ShowBg, grants.items, reason);
}

pub fn grantTreasureBoxReward(
    ctx: RewardContext,
    chest_type_id: i32,
    drop_id: i32,
    reason: i32,
) !GrantResult {
    if (drop_id > 0) {
        const pkg_res = try grantDropPackagePopup(ctx, drop_id, reason);
        if (pkg_res.granted_count > 0) return pkg_res;
    }

    const tier: i32 = std.math.clamp(if (chest_type_id > 0) chest_type_id else 1, 1, 4);
    const astrite: i32 = switch (tier) {
        1 => 5,
        2 => 10,
        3 => 20,
        else => 40,
    };
    const items = [_]RewardItem{
        .{ .item_id = 3, .count = astrite },
        .{ .item_id = 1, .count = tier * 10 },
        .{ .item_id = 2, .count = tier * 1000 },
        .{ .item_id = 41100011, .count = 1 + @divTrunc(tier, 2) },
        .{ .item_id = 41100021, .count = 1 + @divTrunc(tier, 2) },
    };
    return try grantItems(ctx, if (drop_id > 0) drop_id else 1001, true, &items, reason);
}

fn resolvePhantomDropItemId(assets: *const Assets, vision_capture_id: i32, calabash_level: i32) ?i32 {
    if (vision_capture_id <= 0) return null;
    const max_quality: i32 = if (calabash_level >= 15) 5 else if (calabash_level >= 9) 4 else if (calabash_level >= 5) 3 else 2;

    var q: i32 = max_quality;
    while (q >= 2) : (q -= 1) {
        const candidate_i64 = @as(i64, vision_capture_id) * 10 + @as(i64, q);
        if (candidate_i64 <= std.math.maxInt(i32)) {
            const candidate: i32 = @intCast(candidate_i64);
            if (assets.tables.phantom_item.getDataById(candidate) != null) return candidate;
        }
    }
    if (assets.tables.phantom_item.getDataById(vision_capture_id) != null) return vision_capture_id;

    var best_id: ?i32 = null;
    var best_quality: i32 = -1;
    for (assets.tables.phantom_item.items) |p| {
        if (p.MonsterId == vision_capture_id and p.QualityId <= max_quality and p.QualityId > best_quality) {
            best_quality = p.QualityId;
            best_id = p.ItemId;
        }
    }
    return best_id;
}

pub fn grantMonsterDrop(
    ctx: RewardContext,
    reward_id: i32,
    vision_capture_id: i32,
    is_boss: bool,
    calabash_level: i32,
) !GrantResult {
    return try grantMonsterDropWithEchoOption(ctx, reward_id, vision_capture_id, is_boss, calabash_level, true);
}

pub fn grantMonsterDropWithEchoOption(
    ctx: RewardContext,
    reward_id: i32,
    vision_capture_id: i32,
    is_boss: bool,
    calabash_level: i32,
    grant_echo_now: bool,
) !GrantResult {
    var total: GrantResult = .{};
    if (reward_id > 0) {
        total = try grantDropPackage(ctx, reward_id, 1);
    }

    var extras: std.ArrayList(RewardItem) = .empty;
    if (total.granted_count == 0) {
        try extras.append(ctx.alloc.arena, .{ .item_id = 1, .count = if (is_boss) 200 else 10 });
        try extras.append(ctx.alloc.arena, .{ .item_id = 2, .count = if (is_boss) 2000 else 150 });
        if (is_boss) {
            try extras.append(ctx.alloc.arena, .{ .item_id = 41100012, .count = 2 });
        }
    }

    if (grant_echo_now and vision_capture_id > 0) {
        if (resolvePhantomDropItemId(ctx.assets, vision_capture_id, calabash_level)) |echo_item_id| {
            try extras.append(ctx.alloc.arena, .{ .item_id = echo_item_id, .count = 1 });
        }
    }

    if (extras.items.len > 0) {
        const extra_res = try grantItems(ctx, 0, false, extras.items, 1);
        total.granted_count += extra_res.granted_count;
        total.leveled_up = total.leveled_up or extra_res.leveled_up;
        if (extra_res.new_level > total.new_level) total.new_level = extra_res.new_level;
    }

    return total;
}

pub fn grantEchoCaptureOnly(
    ctx: RewardContext,
    vision_capture_id: i32,
    calabash_level: i32,
) !GrantResult {
    if (vision_capture_id <= 0) return .{};
    const echo_item_id = resolvePhantomDropItemId(ctx.assets, vision_capture_id, calabash_level) orelse return .{};
    const items = [_]RewardItem{.{ .item_id = echo_item_id, .count = 1 }};
    return try grantItems(ctx, 0, false, &items, 1);
}

pub fn grantItems(
    ctx: RewardContext,
    drop_id: i32,
    show_popup: bool,
    items: []const RewardItem,
    reason: i32,
) !GrantResult {
    var result: GrantResult = .{};
    if (items.len == 0) return result;

    var basic_dirty = false;
    var inv_dirty = false;
    var calabash_dirty = false;
    var energy_dirty = false;
    var morale_dirty = false;
    var bonus_union_exp: i32 = 0;

    var add_normal_list: std.ArrayList(pb.NormalItem) = .empty;
    var upd_normal_list: std.ArrayList(pb.NormalItem) = .empty;
    var add_weapons: std.ArrayList(pb.WeaponItem) = .empty;
    var add_echoes: std.ArrayList(pb.PhantomItem) = .empty;
    var reward_infos: std.ArrayList(pb.RewardItemInfo) = .empty;

    for (items) |entry| {
        if (entry.item_id <= 0 or entry.count <= 0) continue;

        if (entry.item_id == 1) {
            result.granted_count += 1;
            // Item 1: Union EXP -> PlayerBasicComponent level progression
            addUnionExp(ctx.assets, ctx.basic_comp, entry.count, &result);
            basic_dirty = true;

            try reward_infos.append(ctx.alloc.arena, .{ .ItemId = 1, .Count = entry.count });
        } else if (entry.item_id == 2) {
            result.granted_count += 1;
            // Item 2: Shell Credit
            ctx.basic_comp.info.attributes.coin +|= entry.count;
            basic_dirty = true;
            try upd_normal_list.append(ctx.alloc.arena, .{
                .Id = 2,
                .Count = ctx.basic_comp.info.attributes.coin,
                .ExpireTime = 0,
            });
            try reward_infos.append(ctx.alloc.arena, .{ .ItemId = 2, .Count = entry.count });
        } else if (entry.item_id == 3) {
            result.granted_count += 1;
            // Item 3: Astrite
            ctx.basic_comp.info.attributes.rare_coin +|= entry.count;
            basic_dirty = true;
            const old_count = ctx.inventory_comp.info.normalItemCount(3);
            try InventoryInfo.addNormalItem(&ctx.inventory_comp.info, ctx.alloc.gpa, 3, entry.count);
            inv_dirty = true;
            const new_total = ctx.inventory_comp.info.normalItemCount(3);
            const pb_item: pb.NormalItem = .{ .Id = 3, .Count = new_total, .ExpireTime = 0 };
            if (old_count == 0) {
                try add_normal_list.append(ctx.alloc.arena, pb_item);
            } else {
                try upd_normal_list.append(ctx.alloc.arena, pb_item);
            }
            try reward_infos.append(ctx.alloc.arena, .{ .ItemId = 3, .Count = entry.count });
        } else if (entry.item_id == 4) {
            result.granted_count += 1;
            // Item 4: Lunite
            ctx.basic_comp.info.attributes.cash_coin +|= entry.count;
            basic_dirty = true;
            try upd_normal_list.append(ctx.alloc.arena, .{
                .Id = 4,
                .Count = ctx.basic_comp.info.attributes.cash_coin,
                .ExpireTime = 0,
            });
            try reward_infos.append(ctx.alloc.arena, .{ .ItemId = 4, .Count = entry.count });
        } else if (entry.item_id == 5) {
            result.granted_count += 1;
            // Item 5: Waveplate
            const now_sec = EnergyHelper.nowSeconds(ctx.io);
            _ = EnergyHelper.syncEnergy(ctx.basic_comp, now_sec);
            ctx.basic_comp.info.energy_count = @min(EnergyHelper.waveplate_hard_cap, ctx.basic_comp.info.energy_count + entry.count);
            basic_dirty = true;
            energy_dirty = true;
            try reward_infos.append(ctx.alloc.arena, .{ .ItemId = 5, .Count = entry.count });
        } else if (entry.item_id == 6) {
            result.granted_count += 1;
            // Item 6: Crystalwave Plate
            const now_sec = EnergyHelper.nowSeconds(ctx.io);
            _ = EnergyHelper.syncEnergy(ctx.basic_comp, now_sec);
            ctx.basic_comp.info.store_energy_count = @min(EnergyHelper.crystalwave_cap, ctx.basic_comp.info.store_energy_count + entry.count);
            basic_dirty = true;
            energy_dirty = true;
            try reward_infos.append(ctx.alloc.arena, .{ .ItemId = 6, .Count = entry.count });
        } else if (ctx.assets.tables.weapon_conf.getDataById(entry.item_id) != null) {
            result.granted_count += 1;
            // Weapon item
            var n: i32 = 0;
            while (n < entry.count) : (n += 1) {
                const incr_id = try special_item_incr.next(ctx.alloc.gpa, ctx.fs, ctx.weapon_comp.player_id);
                const weapon: WeaponItem = .{
                    .id = entry.item_id,
                    .func_value = WeaponItem.defaultFuncValue(ctx.assets, entry.item_id),
                };
                try ctx.weapon_comp.weapon_map.put(ctx.alloc.gpa, incr_id, weapon);
                const w_path = try std.fmt.allocPrint(
                    ctx.alloc.arena,
                    "player/{}/{s}/{}",
                    .{ ctx.weapon_comp.player_id, WeaponItem.data_dir, incr_id },
                );
                try comp_util.saveStruct(ctx.fs, weapon, w_path, ctx.alloc.arena);
                try add_weapons.append(ctx.alloc.arena, weapon.toProto(incr_id));
                try reward_infos.append(ctx.alloc.arena, .{ .ItemId = entry.item_id, .Count = 1, .IncrId = incr_id });
            }
        } else if (ctx.assets.tables.phantom_item.getDataById(entry.item_id)) |phantom_cfg| {
            result.granted_count += 1;
            // Echo (Phantom) item
            if (EchoInfo.validDefaultOwnedItem(ctx.assets, phantom_cfg)) {
                const fetter = phantom_cfg.FetterGroup[0];
                var n: i32 = 0;
                while (n < entry.count) : (n += 1) {
                    const incr_id = try special_item_incr.next(ctx.alloc.gpa, ctx.fs, ctx.echo_comp.player_id);
                    const new_echo = try EchoInfo.buildDefaultEcho(ctx.alloc.gpa, ctx.assets, phantom_cfg, fetter);
                    try ctx.echo_comp.echo_map.put(ctx.alloc.gpa, incr_id, new_echo);
                    const e_path = try std.fmt.allocPrint(
                        ctx.alloc.arena,
                        "player/{}/{s}/{}",
                        .{ ctx.echo_comp.player_id, EchoInfo.data_dir, incr_id },
                    );
                    try comp_util.saveStruct(ctx.fs, new_echo, e_path, ctx.alloc.arena);
                    try add_echoes.append(ctx.alloc.arena, try new_echo.toProto(incr_id, ctx.alloc.arena));
                    try reward_infos.append(ctx.alloc.arena, .{ .ItemId = entry.item_id, .Count = 1, .IncrId = incr_id });

                    const discovery = try CalabashHelper.recordEchoDiscovery(
                        ctx.alloc.gpa,
                        ctx.assets,
                        ctx.echo_comp,
                        phantom_cfg.MonsterId,
                        phantom_cfg.QualityId,
                    );
                    if (discovery.changed) calabash_dirty = true;
                    bonus_union_exp += discovery.union_exp;
                }
            }
        } else if (entry.item_id == MoraleHelper.default_keep_item_id or
            entry.item_id == MoraleHelper.default_temp_item_id or
            ctx.assets.tables.item_info.getDataById(entry.item_id) != null)
        {
            result.granted_count += 1;
            // Normal inventory item (consumables, upgrade materials, quest items, morale EXP items, etc.)
            const old_count = ctx.inventory_comp.info.normalItemCount(entry.item_id);
            try InventoryInfo.addNormalItem(&ctx.inventory_comp.info, ctx.alloc.gpa, entry.item_id, entry.count);
            inv_dirty = true;
            if (entry.item_id == MoraleHelper.default_keep_item_id or entry.item_id == MoraleHelper.default_temp_item_id) {
                morale_dirty = true;
            }
            const new_total = ctx.inventory_comp.info.normalItemCount(entry.item_id);
            const pb_item: pb.NormalItem = .{ .Id = entry.item_id, .Count = new_total, .ExpireTime = 0 };
            if (old_count == 0) {
                try add_normal_list.append(ctx.alloc.arena, pb_item);
            } else {
                try upd_normal_list.append(ctx.alloc.arena, pb_item);
            }
            try reward_infos.append(ctx.alloc.arena, .{ .ItemId = entry.item_id, .Count = entry.count });
        }
    }

    if (bonus_union_exp > 0) {
        addUnionExp(ctx.assets, ctx.basic_comp, bonus_union_exp, &result);
        basic_dirty = true;
    }

    if (basic_dirty) {
        const b_path = try std.fmt.allocPrint(ctx.alloc.arena, "player/{}/basic_info", .{ctx.basic_comp.player_id});
        try comp_util.saveStruct(ctx.fs, ctx.basic_comp.info, b_path, ctx.alloc.arena);

        var attrs: std.ArrayList(pb.PlayerAttr) = .empty;
        try attrs.append(ctx.alloc.arena, .{ .Key = .Level, .ValueType = .Int32, .Value = .{ .Int32Value = ctx.basic_comp.info.attributes.level } });
        try attrs.append(ctx.alloc.arena, .{ .Key = .Exp, .ValueType = .Int32, .Value = .{ .Int32Value = ctx.basic_comp.info.attributes.exp } });
        try attrs.append(ctx.alloc.arena, .{ .Key = .Coin, .ValueType = .Int32, .Value = .{ .Int32Value = ctx.basic_comp.info.attributes.coin } });
        try attrs.append(ctx.alloc.arena, .{ .Key = .RareCoin, .ValueType = .Int32, .Value = .{ .Int32Value = ctx.basic_comp.info.attributes.rare_coin } });
        try attrs.append(ctx.alloc.arena, .{ .Key = .CashCoin, .ValueType = .Int32, .Value = .{ .Int32Value = ctx.basic_comp.info.attributes.cash_coin } });
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.PlayerAttrNotify{ .Attributes = attrs });
    }

    if (energy_dirty) {
        try EnergyHelper.pushEnergyNotify(ctx.conn, ctx.alloc, ctx.basic_comp, ctx.io);
    }

    if (inv_dirty) {
        const i_path = try std.fmt.allocPrint(ctx.alloc.arena, "player/{}/{s}", .{ ctx.inventory_comp.player_id, InventoryInfo.data_path });
        try comp_util.saveStruct(ctx.fs, ctx.inventory_comp.info, i_path, ctx.alloc.arena);
    }

    if (add_normal_list.items.len > 0) {
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.NormalItemAddNotify{
            .NormalItemList = add_normal_list,
            .NoTips = !show_popup,
            .Reason = reason,
        });
    }
    if (upd_normal_list.items.len > 0) {
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.NormalItemUpdateNotify{
            .NormalItemList = upd_normal_list,
            .NoTips = !show_popup,
        });
    }
    if (add_weapons.items.len > 0) {
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.WeaponItemAddNotify{
            .WeaponItemList = add_weapons,
            .AddFromRole = false,
            .Reason = reason,
        });
    }
    if (add_echoes.items.len > 0) {
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.PhantomItemAddNotify{
            .PhantomItemList = add_echoes,
        });
    }
    if (calabash_dirty) {
        try PlayerEchoComponent.saveCalabash(ctx.alloc.gpa, ctx.fs, ctx.echo_comp.player_id, ctx.echo_comp.calabash_info);
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, try CalabashHelper.buildMsgNotify(ctx.alloc.arena, ctx.assets, ctx.basic_comp, ctx.echo_comp));
    }

    if (morale_dirty) {
        _ = try MoraleHelper.syncFromInventory(
            ctx.alloc,
            ctx.fs,
            ctx.io,
            ctx.conn,
            ctx.assets,
            ctx.basic_comp,
            ctx.inventory_comp,
            ctx.progress_comp,
            null,
            null,
            true,
        );
    }

    if (show_popup and reward_infos.items.len > 0) {
        const effective_reason: i32 = if (reason > 2) reason else 19500;
        var reward_entries: std.ArrayList(pb.MapEntry(i32, pb.RewardItemInfoList)) = .empty;
        try reward_entries.append(ctx.alloc.arena, .{
            .key = 0,
            .value = .{ .ItemList = reward_infos },
        });
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.ItemRewardNotify{
            .DropId = if (drop_id > 0 and ctx.assets.tables.drop_package.getDataById(drop_id) != null)
                drop_id
            else
                1201,
            .Reason = effective_reason,
            .Magnification = 1,
            .RewardItems = reward_entries,
        });
    }

    return result;
}

fn addUnionExp(assets: *const Assets, basic_comp: *PlayerBasicComponent, amount: i32, result: *GrantResult) void {
    if (amount <= 0) return;
    basic_comp.info.attributes.exp += amount;
    const prev_level = basic_comp.info.attributes.level;
    applyPlayerExpLevelUp(assets, &basic_comp.info.attributes.level, &basic_comp.info.attributes.exp);
    if (basic_comp.info.attributes.level > prev_level) {
        result.leveled_up = true;
    }
    result.new_level = basic_comp.info.attributes.level;
}

fn applyPlayerExpLevelUp(assets: *const Assets, level_ptr: *i32, exp_ptr: *i32) void {
    while (level_ptr.* < 80) {
        const cfg = assets.tables.player_exp.getDataById(level_ptr.*) orelse break;
        if (cfg.LevelExp <= 0 or exp_ptr.* < cfg.LevelExp) break;
        exp_ptr.* -= cfg.LevelExp;
        level_ptr.* += 1;
    }
    if (level_ptr.* >= 80) {
        exp_ptr.* = 0;
    }
}

pub fn collectDropItems(ctx: RewardContext, drop_id: i32, out_items: *std.ArrayList(RewardItem)) !void {
    if (drop_id <= 0) return;
    const pkg = ctx.assets.tables.drop_package.getDataById(drop_id) orelse return;
    var preview_it = pkg.DropPreview.map.iterator();
    while (preview_it.next()) |kv| {
        const item_id = kv.key_ptr.*;
        const raw_count = kv.value_ptr.*;
        if (item_id <= 0) continue;
        const count: i32 = if (raw_count > 0)
            raw_count
        else if (item_id == 1)
            50
        else if (item_id == 2)
            200
        else if (item_id == 3)
            10
        else
            1;
        try out_items.append(ctx.alloc.arena, .{ .item_id = item_id, .count = count });
    }
}

/// Looks up `table_name` row `key` in `RewardIndex` (direct items, pay_items if `use_pay`,
/// or `drop_id` via DropPackage), falls back to `fallback` if absent/empty, grants all items,
/// and optionally appends the granted list to `out_items`.
pub fn grantTableReward(
    ctx: RewardContext,
    table_name: []const u8,
    key: i64,
    use_pay: bool,
    show_popup: bool,
    reason: i32,
    fallback: []const RewardItem,
    out_items: ?*std.ArrayList(RewardItem),
) !GrantResult {
    var grants: std.ArrayList(RewardItem) = .empty;
    var used_drop_id: i32 = 0;

    if (ctx.assets.reward_index.getRow(ctx.io, table_name, key)) |row| {
        const pair_slice = if (use_pay) row.pay_items else row.items;
        for (pair_slice) |p| {
            if (p.item_id > 0 and p.count > 0) {
                try grants.append(ctx.alloc.arena, .{ .item_id = p.item_id, .count = p.count });
            }
        }
        const drop_id = if (use_pay and row.alt_drop_id > 0) row.alt_drop_id else row.drop_id;
        if (grants.items.len == 0 and drop_id > 0) {
            used_drop_id = drop_id;
            try collectDropItems(ctx, drop_id, &grants);
        }
    }

    if (grants.items.len == 0) {
        for (fallback) |item| {
            if (item.item_id > 0 and item.count > 0) {
                try grants.append(ctx.alloc.arena, item);
            }
        }
    }

    if (out_items) |out| {
        try out.appendSlice(ctx.alloc.arena, grants.items);
    }
    if (grants.items.len == 0) return .{};
    return try grantItems(ctx, used_drop_id, show_popup, grants.items, reason);
}

