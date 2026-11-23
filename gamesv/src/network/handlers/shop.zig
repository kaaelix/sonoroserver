const std = @import("std");
const common = @import("common");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const Transaction = @import("../handlers.zig").Transaction;
const Assets = @import("../../data/Assets.zig");
const ProgressInfo = @import("../../fs/ProgressInfo.zig");
const PlayerProgressComponent = @import("../../logic/component/player/PlayerProgressComponent.zig");
const PlayerBasicComponent = @import("../../logic/component/player/PlayerBasicComponent.zig");
const PlayerInventoryComponent = @import("../../logic/component/player/PlayerInventoryComponent.zig");
const PlayerWeaponComponent = @import("../../logic/component/player/PlayerWeaponComponent.zig");
const PlayerEchoComponent = @import("../../logic/component/player/PlayerEchoComponent.zig");
const PlayerMapComponent = @import("../../logic/component/player/PlayerMapComponent.zig");
const PlayerQuestComponent = @import("../../logic/component/player/PlayerQuestComponent.zig");
const PlayerRoleComponent = @import("../../logic/component/player/PlayerRoleComponent.zig");
const PlayerGuideComponent = @import("../../logic/component/player/PlayerGuideComponent.zig");
const ConditionEvaluator = @import("../../logic/helpers/condition.zig");
const ConsumeHelper = @import("../../logic/helpers/consume.zig");
const CraftHelper = @import("../../logic/helpers/craft.zig");
const EnergyHelper = @import("../../logic/helpers/energy.zig");

const FileSystem = common.FileSystem;

const PayCatalogItem = struct {
    id: i32,
    shop_id: i32,
    tab_id: i32,
    item_id: i32,
    item_count: i32,
    price_id: i32,
    price_count: i32,
    buy_limit: i32,
};

// Standard Wuthering Waves exchange & store conversion catalog when client PayShopDirectGoods.json is empty.
// Includes GachaItemIdToShopId mappings (104300001..104300005) and skin recommend goods (106100007..106100008).
const default_pay_catalog = [_]PayCatalogItem{
    .{ .id = 10001, .shop_id = 100, .tab_id = 1, .item_id = 3, .item_count = 1, .price_id = 4, .price_count = 1, .buy_limit = -1 },
    .{ .id = 40001, .shop_id = 4, .tab_id = 1, .item_id = 50001, .item_count = 1, .price_id = 50004, .price_count = 8, .buy_limit = -1 },
    .{ .id = 40002, .shop_id = 4, .tab_id = 1, .item_id = 50005, .item_count = 1, .price_id = 50004, .price_count = 8, .buy_limit = -1 },
    .{ .id = 40003, .shop_id = 4, .tab_id = 1, .item_id = 50002, .item_count = 1, .price_id = 50004, .price_count = 8, .buy_limit = -1 },
    .{ .id = 40004, .shop_id = 4, .tab_id = 2, .item_id = 50001, .item_count = 1, .price_id = 50003, .price_count = 70, .buy_limit = 7 },
    .{ .id = 40005, .shop_id = 4, .tab_id = 2, .item_id = 50005, .item_count = 1, .price_id = 50003, .price_count = 70, .buy_limit = 7 },
    .{ .id = 40006, .shop_id = 4, .tab_id = 2, .item_id = 50002, .item_count = 1, .price_id = 50003, .price_count = 70, .buy_limit = 7 },
    .{ .id = 104300001, .shop_id = 4, .tab_id = 3, .item_id = 50002, .item_count = 1, .price_id = 3, .price_count = 160, .buy_limit = -1 },
    .{ .id = 104300002, .shop_id = 4, .tab_id = 3, .item_id = 50005, .item_count = 1, .price_id = 3, .price_count = 160, .buy_limit = -1 },
    .{ .id = 104300003, .shop_id = 4, .tab_id = 3, .item_id = 50001, .item_count = 1, .price_id = 3, .price_count = 160, .buy_limit = -1 },
    .{ .id = 104300004, .shop_id = 4, .tab_id = 3, .item_id = 50007, .item_count = 1, .price_id = 3, .price_count = 160, .buy_limit = -1 },
    .{ .id = 104300005, .shop_id = 4, .tab_id = 3, .item_id = 50008, .item_count = 1, .price_id = 3, .price_count = 160, .buy_limit = -1 },
    .{ .id = 106100007, .shop_id = 6, .tab_id = 3, .item_id = 81011508, .item_count = 1, .price_id = 4, .price_count = 1680, .buy_limit = 1 },
    .{ .id = 106100008, .shop_id = 6, .tab_id = 3, .item_id = 81011509, .item_count = 1, .price_id = 4, .price_count = 1680, .buy_limit = 1 },
};

fn recommendTabImage(r: Assets.DataTables.PayShopRecommend) []const u8 {
    if (std.mem.eql(u8, r.PrefabPath, "UiItem_ShopSkinQianXiao")) return "T_ShopTabRoleQianxiao1";
    if (std.mem.eql(u8, r.PrefabPath, "UiItem_ShopSkinLinNai")) return "T_ShopTabRoleLinnai1";
    if (std.mem.eql(u8, r.PrefabPath, "UiItem_ShopSkinMoNing")) return "T_ShopTabRoleMoning";
    if (std.mem.eql(u8, r.PrefabPath, "UiItem_ShopSkinMoto")) return "T_ShopTabMoto";
    if (std.mem.eql(u8, r.PrefabPath, "UiItem_ShopSkinMotoFeixue")) return "T_ShopTabMotorFeixue";
    if (std.mem.eql(u8, r.PrefabPath, "UiItem_ShopSkinMotoYangyang")) return "T_ShopTabMotoYangyang";
    return "T_ShopTabMonthlyCard";
}

fn buildPayShopItemPb(entry: PayCatalogItem, bought_count: i32) pb.PayShopItem {
    const sold_out = entry.buy_limit > 0 and bought_count >= entry.buy_limit;
    return .{
        .Id = entry.id,
        .ItemId = entry.item_id,
        .ItemCount = entry.item_count,
        .Locked = false,
        .BuyLimit = entry.buy_limit,
        .BoughtCount = bought_count,
        .Price = .{ .Id = entry.price_id, .Count = entry.price_count, .PromotionCount = entry.price_count },
        .UpdateType = if (entry.buy_limit > 0) .Monthly else .Forever,
        .ShopItemType = .Normal,
        .CanBuyGoods = !sold_out,
        .TabId = entry.tab_id,
        .ShopId = entry.shop_id,
        .SoldOut = sold_out,
        .Show = true,
        .Quality = 5,
        .IsBuyMaxButton = true,
    };
}

fn buildPayShopInfo(
    arena: std.mem.Allocator,
    shop_cfg: Assets.DataTables.PayShop,
    progress: ProgressInfo,
) !pb.PayShopInfo {
    var moneys: std.ArrayList(i32) = .empty;
    try moneys.appendSlice(arena, shop_cfg.Money);

    var items: std.ArrayList(pb.PayShopItem) = .empty;
    for (default_pay_catalog) |entry| {
        if (entry.shop_id != shop_cfg.Id) continue;
        const bought = ProgressInfo.getIdCount(progress.shop.payshop_bought, entry.id);
        try items.append(arena, buildPayShopItemPb(entry, bought));
    }

    return .{
        .Id = shop_cfg.Id,
        .Items = items,
        .ShopTabViewType = shop_cfg.ShopTabViewType,
        .DynamicTabId = shop_cfg.DynamicTabId,
        .Sort = shop_cfg.Sort,
        .Money = moneys,
        .SortRule = shop_cfg.SortRule,
    };
}

pub fn onPayShopInfoRequest(
    txn: *Transaction(pb.PayShopInfoRequest),
    assets: *const Assets,
    progress_comp: *const PlayerProgressComponent,
    alloc: mem.Alloc,
) !void {
    var infos: std.ArrayList(pb.PayShopInfo) = .empty;
    for (assets.tables.pay_shop.items) |shop_cfg| {
        try infos.append(alloc.arena, try buildPayShopInfo(alloc.arena, shop_cfg, progress_comp.info));
    }

    var tabs: std.ArrayList(pb.ShopTab) = .empty;
    for (assets.tables.pay_shop_tab.items) |t| {
        try tabs.append(alloc.arena, .{
            .ShopId = t.ShopId,
            .TabId = t.TabId,
            .Sort = t.Sort,
            .name = t.Name,
            .Logic = t.Logic,
            .Enable = t.Enable,
        });
    }

    var recs: std.ArrayList(pb.ShopRecommend) = .empty;
    for (assets.tables.pay_shop_recommend.items) |r| {
        if (!r.Show or r.PrefabPath.len == 0) continue;
        try recs.append(alloc.arena, .{
            .Id = r.Id,
            .RecommendType = r.RecommendType,
            .RecommendId = r.RecommendId,
            .TabName = r.TabName,
            .PrefabPath = r.PrefabPath,
            .Sort = r.Sort,
            .Show = r.Show,
            .TabImage = recommendTabImage(r),
        });
    }

    txn.respond(.{
        .Infos = infos,
        .Version = "3.8.11",
        .ErrorCode = .Success,
        .PayShopTabData = tabs,
        .PayShopRecommendData = recs,
    });
}

pub fn onPayShopUpdateRequest(
    txn: *Transaction(pb.PayShopUpdateRequest),
    assets: *const Assets,
    progress_comp: *const PlayerProgressComponent,
    alloc: mem.Alloc,
) !void {
    const shop_cfg = assets.tables.pay_shop.getDataById(txn.message.Id) orelse {
        txn.respond(.{ .ErrorCode = .RequestParamError });
        return;
    };
    txn.respond(.{
        .Info = try buildPayShopInfo(alloc.arena, shop_cfg, progress_comp.info),
        .ErrorCode = .Success,
    });
}

pub fn onPayShopItemUpdateRequest(
    txn: *Transaction(pb.PayShopItemUpdateRequest),
    progress_comp: *const PlayerProgressComponent,
    alloc: mem.Alloc,
) !void {
    var items: std.ArrayList(pb.PayShopItem) = .empty;
    for (txn.message.ShopItemIds.items) |id| {
        for (default_pay_catalog) |entry| {
            if (entry.id != id) continue;
            const bought = ProgressInfo.getIdCount(progress_comp.info.shop.payshop_bought, id);
            try items.append(alloc.arena, buildPayShopItemPb(entry, bought));
        }
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .Items = items,
    });
}

pub fn onPayShopBuyRequest(
    txn: *Transaction(pb.PayShopBuyRequest),
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    const count = txn.message.Count;
    if (count <= 0) {
        txn.respond(.{ .Id = txn.message.Id, .Count = 0, .ErrorCode = .RequestParamError });
        return;
    }
    var found: ?PayCatalogItem = null;
    for (default_pay_catalog) |entry| {
        if (entry.id == txn.message.Id) {
            found = entry;
            break;
        }
    }
    const item = found orelse {
        txn.respond(.{ .Id = txn.message.Id, .Count = 0, .ErrorCode = .ErrItemCfgNotFound });
        return;
    };

    const bought = ProgressInfo.getIdCount(progress_comp.info.shop.payshop_bought, item.id);
    if (item.buy_limit > 0 and bought + count > item.buy_limit) {
        txn.respond(.{ .Id = item.id, .Count = bought, .ErrorCode = .ErrItemCount });
        return;
    }

    const total_cost = std.math.mul(i32, item.price_count, count) catch {
        txn.respond(.{ .Id = item.id, .Count = bought, .ErrorCode = .RequestParamError });
        return;
    };
    const total_reward = std.math.mul(i32, item.item_count, count) catch {
        txn.respond(.{ .Id = item.id, .Count = bought, .ErrorCode = .RequestParamError });
        return;
    };

    const outcome = try CraftHelper.consumeAndGrant(.{
        .conn = txn.conn,
        .alloc = alloc,
        .io = io,
        .fs = fs,
        .assets = assets,
        .basic_comp = basic_comp,
        .inventory_comp = inventory_comp,
        .weapon_comp = weapon_comp,
        .echo_comp = echo_comp,
    }, &.{.{ .id = item.price_id, .count = total_cost }}, item.item_id, total_reward, 2);

    if (!outcome.ok) {
        txn.respond(.{ .Id = item.id, .Count = bought, .ErrorCode = outcome.err });
        return;
    }

    _ = try ProgressInfo.addIdCount(&progress_comp.info.shop.payshop_bought, alloc.gpa, item.id, count);
    try progress_comp.save(fs, alloc.arena);

    txn.respond(.{
        .Count = count,
        .Id = item.id,
        .ErrorCode = .Success,
    });
}

fn buildFixedShopPb(
    arena: std.mem.Allocator,
    shop_id: i32,
    assets: *const Assets,
    progress: ProgressInfo,
) !pb.ShopInfo {
    var items: std.ArrayList(pb.ShopItemInfoNew) = .empty;
    for (assets.tables.shop_fixed.items) |row| {
        if (row.ShopId != shop_id) continue;
        var money_list: std.ArrayList(pb.ShoppMoneyInfo) = .empty;
        var pit = row.Price.map.iterator();
        while (pit.next()) |kv| {
            try money_list.append(arena, .{ .MoneyId = kv.key_ptr.*, .MoneyNum = kv.value_ptr.* });
        }
        var orig_list: std.ArrayList(pb.ShoppMoneyInfo) = .empty;
        var oit = row.OriginalPrice.map.iterator();
        while (oit.next()) |kv| {
            try orig_list.append(arena, .{ .MoneyId = kv.key_ptr.*, .MoneyNum = kv.value_ptr.* });
        }
        const bought = progress.getShopGoodCount(shop_id, row.Id);
        try items.append(arena, .{
            .Id = row.Id,
            .BoughtCount = bought,
            .Lock = false,
            .ItemId = row.ItemId,
            .ItemNum = row.ItemNum,
            .MoneyList = money_list,
            .LimitNum = row.LimitNum,
            .OriginalMoneyList = orig_list,
            .Label = row.Label,
            .UnlockConditionId = row.CondId,
            .SortIndex = row.SortIndex,
        });
    }
    return .{
        .ShopId = shop_id,
        .ItemInfoList = items,
    };
}

pub fn onShopInfoRequest(
    txn: *Transaction(pb.ShopInfoRequest),
    assets: *const Assets,
    progress_comp: *const PlayerProgressComponent,
    alloc: mem.Alloc,
) !void {
    var list: std.ArrayList(pb.ShopInfo) = .empty;
    for (assets.tables.shop_info.items) |shop_cfg| {
        try list.append(alloc.arena, try buildFixedShopPb(alloc.arena, shop_cfg.Id, assets, progress_comp.info));
    }
    txn.respond(.{
        .ShopList = list,
        .VersionStr = "3.8.11",
        .ErrorCode = .Success,
    });
}

pub fn onShopUpdateRequest(
    txn: *Transaction(pb.ShopUpdateRequest),
    assets: *const Assets,
    progress_comp: *const PlayerProgressComponent,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    const shop_info = try buildFixedShopPb(alloc.arena, txn.message.ShopId, assets, progress_comp.info);
    txn.respond(.{
        .ErrorCode = .Success,
        .Info = .{
            .ShopId = txn.message.ShopId,
            .UpdateTime = @intCast(std.math.clamp(EnergyHelper.nowSeconds(io), 0, std.math.maxInt(u32))),
            .ItemInfoList = shop_info.ItemInfoList,
        },
    });
}

pub fn onShopBuyRequest(
    txn: *Transaction(pb.ShopBuyRequest),
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    map_comp: *PlayerMapComponent,
    quest_comp: *PlayerQuestComponent,
    role_comp: *PlayerRoleComponent,
    guide_comp: *PlayerGuideComponent,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    const req = txn.message;
    const row = assets.tables.shop_fixed.getDataById(req.Id) orelse {
        txn.respond(.{ .Id = req.Id, .ShopId = req.ShopId, .ErrorCode = .ErrItemCfgNotFound });
        return;
    };
    if (req.ShopId != 0 and req.ShopId != row.ShopId) {
        txn.respond(.{ .Id = req.Id, .ShopId = req.ShopId, .ErrorCode = .ErrShopIdNotExit });
        return;
    }
    const count = req.Num;
    if (count <= 0) {
        txn.respond(.{ .Id = req.Id, .ShopId = req.ShopId, .ErrorCode = .RequestParamError });
        return;
    }
    if (row.CondId > 0) {
        const eval_ctx: ConditionEvaluator.EvalContext = .{
            .assets = assets,
            .basic_comp = basic_comp,
            .echo_comp = echo_comp,
            .inventory_comp = inventory_comp,
            .map_comp = map_comp,
            .quest_comp = quest_comp,
            .role_comp = role_comp,
            .weapon_comp = weapon_comp,
            .guide_comp = guide_comp,
            .progress_comp = progress_comp,
        };
        if (!ConditionEvaluator.evalConditionId(eval_ctx, row.CondId)) {
            txn.respond(.{ .Id = req.Id, .ShopId = row.ShopId, .ErrorCode = .ErrShopCondLimit });
            return;
        }
    }
    const bought = progress_comp.info.getShopGoodCount(row.ShopId, row.Id);
    if (row.LimitNum > 0 and (std.math.add(i32, bought, count) catch std.math.maxInt(i32)) > row.LimitNum) {
        txn.respond(.{ .Id = req.Id, .ShopId = row.ShopId, .BoughtCount = bought, .ErrorCode = .ErrItemCount });
        return;
    }

    var cost_id: i32 = 0;
    var cost_unit: i32 = 0;
    if (req.MoneyId > 0) {
        if (row.Price.map.get(req.MoneyId)) |unit_price| {
            cost_id = req.MoneyId;
            cost_unit = unit_price;
        } else {
            txn.respond(.{ .Id = req.Id, .ShopId = row.ShopId, .BoughtCount = bought, .ErrorCode = .ErrShopMoneyId });
            return;
        }
    } else {
        var pit = row.Price.map.iterator();
        if (pit.next()) |kv| {
            cost_id = kv.key_ptr.*;
            cost_unit = kv.value_ptr.*;
        }
    }
    if (row.Price.map.count() > 0 and cost_id == 0) {
        txn.respond(.{ .Id = req.Id, .ShopId = row.ShopId, .BoughtCount = bought, .ErrorCode = .ErrShopMoneyId });
        return;
    }

    const total_cost = std.math.mul(i32, cost_unit, count) catch {
        txn.respond(.{ .Id = req.Id, .ShopId = row.ShopId, .ErrorCode = .RequestParamError });
        return;
    };
    const total_items = std.math.mul(i32, row.ItemNum, count) catch {
        txn.respond(.{ .Id = req.Id, .ShopId = row.ShopId, .ErrorCode = .RequestParamError });
        return;
    };

    const costs: []const ConsumeHelper.CostItem = if (cost_id > 0 and total_cost > 0)
        &.{.{ .id = cost_id, .count = total_cost }}
    else
        &.{};

    const outcome = try CraftHelper.consumeAndGrant(.{
        .conn = txn.conn,
        .alloc = alloc,
        .io = io,
        .fs = fs,
        .assets = assets,
        .basic_comp = basic_comp,
        .inventory_comp = inventory_comp,
        .weapon_comp = weapon_comp,
        .echo_comp = echo_comp,
    }, costs, row.ItemId, total_items, 2);

    if (!outcome.ok) {
        txn.respond(.{ .Id = req.Id, .ShopId = row.ShopId, .BoughtCount = bought, .ErrorCode = outcome.err });
        return;
    }

    const new_bought = try progress_comp.info.addShopGoodCount(alloc.gpa, row.ShopId, row.Id, count);
    try progress_comp.save(fs, alloc.arena);

    try txn.conn.push(alloc.gpa, io, pb.ShopInfoUpdateNotify{
        .ShopInfo = try buildFixedShopPb(alloc.arena, row.ShopId, assets, progress_comp.info),
    });

    txn.respond(.{
        .Id = row.Id,
        .ShopId = row.ShopId,
        .BoughtCount = new_bought,
        .ErrorCode = .Success,
    });
}

pub fn onItemExchangeRequest(
    txn: *Transaction(pb.ItemExchangeRequest),
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    const item_id = txn.message.ItemId;
    const times = txn.message.ExchangeTimes;
    if (times <= 0 or item_id <= 0) {
        txn.respond(.{ .ItemId = item_id, .ItemCount = 0, .ErrorCode = .RequestParamError });
        return;
    }
    const ex_cfg = assets.tables.item_exchange_content.getDataById(item_id) orelse {
        txn.respond(.{ .ItemId = item_id, .ItemCount = 0, .ErrorCode = .RequestParamError });
        return;
    };
    var price_it = ex_cfg.Consume.map.iterator();
    const price_kv = price_it.next() orelse {
        txn.respond(.{ .ItemId = item_id, .ItemCount = 0, .ErrorCode = .RequestParamError });
        return;
    };
    const price_id: i32 = price_kv.key_ptr.*;
    const unit_price: i32 = price_kv.value_ptr.*;
    const total_cost = std.math.mul(i32, unit_price, times) catch {
        txn.respond(.{ .ItemId = item_id, .ItemCount = 0, .ErrorCode = .RequestParamError });
        return;
    };
    const gain_count = std.math.mul(i32, @max(1, ex_cfg.GainCount), times) catch {
        txn.respond(.{ .ItemId = item_id, .ItemCount = 0, .ErrorCode = .RequestParamError });
        return;
    };

    const outcome = try CraftHelper.consumeAndGrant(.{
        .conn = txn.conn,
        .alloc = alloc,
        .io = io,
        .fs = fs,
        .assets = assets,
        .basic_comp = basic_comp,
        .inventory_comp = inventory_comp,
        .weapon_comp = weapon_comp,
        .echo_comp = echo_comp,
    }, &.{.{ .id = price_id, .count = total_cost }}, item_id, gain_count, 2);

    if (!outcome.ok) {
        txn.respond(.{ .ItemId = item_id, .ItemCount = 0, .ErrorCode = outcome.err });
        return;
    }

    _ = try ProgressInfo.addIdCount(&progress_comp.info.shop.item_exchange_counts, alloc.gpa, item_id, times);
    try progress_comp.save(fs, alloc.arena);

    txn.respond(.{
        .ErrorCode = .Success,
        .ItemId = item_id,
        .ItemCount = gain_count,
    });
}
