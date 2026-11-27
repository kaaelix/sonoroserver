use sonoro_gamesv_data::tables::pay_shop::PayShop;
use sonoro_gamesv_data::tables::pay_shop_recommend::PayShopRecommend;
use sonoro_gamesv_fs::progress_info::ProgressInfo;
use sonoro_gamesv_logic::helpers::{condition, craft, energy};
use sonoro_gamesv_logic::sink::PushSink;
use sonoro_proto::enums::{ErrorCode, PayShopItemType, PayUpdateType};
use sonoro_proto::pb::{
    ItemExchangeRequest, ItemExchangeResponse, PayShopBuyRequest, PayShopBuyResponse, PayShopInfo,
    PayShopInfoRequest, PayShopInfoResponse, PayShopItem, PayShopItemUpdateRequest,
    PayShopItemUpdateResponse, PayShopPrice, PayShopUpdateRequest, PayShopUpdateResponse,
    ShopBuyRequest, ShopBuyResponse, ShopInfo, ShopInfoRequest, ShopInfoResponse,
    ShopInfoUpdateNotify, ShopItemInfoNew, ShopRecommend, ShopTab, ShopUpdateRequest,
    ShopUpdateResponse, ShoppMoneyInfo,
};

use super::{qol, State};

struct PayCatalogItem {
    id: i32,
    shop_id: i32,
    tab_id: i32,
    item_id: i32,
    item_count: i32,
    price_id: i32,
    price_count: i32,
    buy_limit: i32,
}

const DEFAULT_PAY_CATALOG: &[PayCatalogItem] = &[
    // Purchase (Shop 100): 1:1 Lunite -> Astrite exchange entry
    PayCatalogItem { id: 10001, shop_id: 100, tab_id: 1, item_id: 3, item_count: 1, price_id: 4, price_count: 1, buy_limit: -1 },
    // Bundles (Shop 3): Lunite-purchasable supply bundles alongside USD PayGift packs
    PayCatalogItem { id: 30001, shop_id: 3, tab_id: 2, item_id: 52001, item_count: 1, price_id: 4, price_count: 980, buy_limit: 1 },
    PayCatalogItem { id: 30002, shop_id: 3, tab_id: 2, item_id: 52002, item_count: 1, price_id: 4, price_count: 680, buy_limit: 1 },
    PayCatalogItem { id: 30003, shop_id: 3, tab_id: 1, item_id: 51006, item_count: 1, price_id: 4, price_count: 300, buy_limit: 5 },
    PayCatalogItem { id: 30004, shop_id: 3, tab_id: 1, item_id: 51007, item_count: 1, price_id: 4, price_count: 680, buy_limit: 5 },
    // Item Exchange (Shop 4) — Tab 1: Afterglow Coral (50004)
    PayCatalogItem { id: 40001, shop_id: 4, tab_id: 1, item_id: 50001, item_count: 1, price_id: 50004, price_count: 8, buy_limit: -1 },
    PayCatalogItem { id: 40002, shop_id: 4, tab_id: 1, item_id: 50005, item_count: 1, price_id: 50004, price_count: 8, buy_limit: -1 },
    PayCatalogItem { id: 40003, shop_id: 4, tab_id: 1, item_id: 50002, item_count: 1, price_id: 50004, price_count: 8, buy_limit: -1 },
    PayCatalogItem { id: 40010, shop_id: 4, tab_id: 1, item_id: 10001503, item_count: 1, price_id: 50004, price_count: 360, buy_limit: 2 },
    PayCatalogItem { id: 40011, shop_id: 4, tab_id: 1, item_id: 10001203, item_count: 1, price_id: 50004, price_count: 360, buy_limit: 2 },
    PayCatalogItem { id: 40012, shop_id: 4, tab_id: 1, item_id: 10001301, item_count: 1, price_id: 50004, price_count: 360, buy_limit: 2 },
    PayCatalogItem { id: 40013, shop_id: 4, tab_id: 1, item_id: 10001104, item_count: 1, price_id: 50004, price_count: 360, buy_limit: 2 },
    PayCatalogItem { id: 40014, shop_id: 4, tab_id: 1, item_id: 10001404, item_count: 1, price_id: 50004, price_count: 360, buy_limit: 2 },
    PayCatalogItem { id: 40015, shop_id: 4, tab_id: 1, item_id: 10001102, item_count: 1, price_id: 50004, price_count: 80, buy_limit: 2 },
    PayCatalogItem { id: 40016, shop_id: 4, tab_id: 1, item_id: 10001204, item_count: 1, price_id: 50004, price_count: 80, buy_limit: 2 },
    // Item Exchange (Shop 4) — Tab 2: Oscillated Coral (50003)
    PayCatalogItem { id: 40004, shop_id: 4, tab_id: 2, item_id: 50001, item_count: 1, price_id: 50003, price_count: 70, buy_limit: 7 },
    PayCatalogItem { id: 40005, shop_id: 4, tab_id: 2, item_id: 50005, item_count: 1, price_id: 50003, price_count: 70, buy_limit: 7 },
    PayCatalogItem { id: 40006, shop_id: 4, tab_id: 2, item_id: 50002, item_count: 1, price_id: 50003, price_count: 70, buy_limit: 7 },
    PayCatalogItem { id: 40020, shop_id: 4, tab_id: 2, item_id: 43010003, item_count: 1, price_id: 50003, price_count: 12, buy_limit: 40 },
    PayCatalogItem { id: 40021, shop_id: 4, tab_id: 2, item_id: 43020003, item_count: 1, price_id: 50003, price_count: 12, buy_limit: 40 },
    PayCatalogItem { id: 40022, shop_id: 4, tab_id: 2, item_id: 36000003, item_count: 1, price_id: 50003, price_count: 12, buy_limit: 40 },
    PayCatalogItem { id: 40023, shop_id: 4, tab_id: 2, item_id: 36000014, item_count: 10, price_id: 50003, price_count: 30, buy_limit: 10 },
    PayCatalogItem { id: 40024, shop_id: 4, tab_id: 2, item_id: 2, item_count: 10_000, price_id: 50003, price_count: 6, buy_limit: 50 },
    PayCatalogItem { id: 40025, shop_id: 4, tab_id: 2, item_id: 41100013, item_count: 1, price_id: 50003, price_count: 10, buy_limit: 30 },
    // Item Exchange (Shop 4) — Tab 3: Astrite Exchange (3)
    PayCatalogItem { id: 104300001, shop_id: 4, tab_id: 3, item_id: 50002, item_count: 1, price_id: 3, price_count: 160, buy_limit: -1 },
    PayCatalogItem { id: 104300002, shop_id: 4, tab_id: 3, item_id: 50005, item_count: 1, price_id: 3, price_count: 160, buy_limit: -1 },
    PayCatalogItem { id: 104300003, shop_id: 4, tab_id: 3, item_id: 50001, item_count: 1, price_id: 3, price_count: 160, buy_limit: -1 },
    PayCatalogItem { id: 104300004, shop_id: 4, tab_id: 3, item_id: 50007, item_count: 1, price_id: 3, price_count: 160, buy_limit: -1 },
    PayCatalogItem { id: 104300005, shop_id: 4, tab_id: 3, item_id: 50008, item_count: 1, price_id: 3, price_count: 160, buy_limit: -1 },
    // Points Shop (Shop 5): Hazard Record (12) & Data Set (34, 11)
    PayCatalogItem { id: 50001, shop_id: 5, tab_id: 1, item_id: 36000004, item_count: 1, price_id: 12, price_count: 60, buy_limit: 20 },
    PayCatalogItem { id: 50002, shop_id: 5, tab_id: 1, item_id: 36000014, item_count: 10, price_id: 12, price_count: 40, buy_limit: 20 },
    PayCatalogItem { id: 50003, shop_id: 5, tab_id: 1, item_id: 43010003, item_count: 1, price_id: 12, price_count: 25, buy_limit: 40 },
    PayCatalogItem { id: 50004, shop_id: 5, tab_id: 1, item_id: 43020003, item_count: 1, price_id: 12, price_count: 25, buy_limit: 40 },
    PayCatalogItem { id: 50005, shop_id: 5, tab_id: 1, item_id: 2, item_count: 10_000, price_id: 12, price_count: 20, buy_limit: 50 },
    PayCatalogItem { id: 50006, shop_id: 5, tab_id: 2, item_id: 36000004, item_count: 1, price_id: 34, price_count: 50, buy_limit: 20 },
    PayCatalogItem { id: 50007, shop_id: 5, tab_id: 2, item_id: 36000014, item_count: 10, price_id: 34, price_count: 30, buy_limit: 20 },
    PayCatalogItem { id: 50008, shop_id: 5, tab_id: 3, item_id: 43010003, item_count: 1, price_id: 11, price_count: 20, buy_limit: 30 },
    // Outfits (Shop 6) — Tab 1: RoleSkinTabView (must be tab_id: 1, not 3 which is MotorSkinTabView)
    PayCatalogItem { id: 106100001, shop_id: 6, tab_id: 1, item_id: 53001, item_count: 1, price_id: 4, price_count: 3280, buy_limit: 1 },
    PayCatalogItem { id: 106100002, shop_id: 6, tab_id: 1, item_id: 53002, item_count: 1, price_id: 4, price_count: 1680, buy_limit: 1 },
    PayCatalogItem { id: 106100003, shop_id: 6, tab_id: 1, item_id: 53003, item_count: 1, price_id: 4, price_count: 2480, buy_limit: 1 },
    PayCatalogItem { id: 106100004, shop_id: 6, tab_id: 1, item_id: 53004, item_count: 1, price_id: 4, price_count: 2480, buy_limit: 1 },
    PayCatalogItem { id: 106100005, shop_id: 6, tab_id: 1, item_id: 53005, item_count: 1, price_id: 4, price_count: 2480, buy_limit: 1 },
    PayCatalogItem { id: 106100006, shop_id: 6, tab_id: 1, item_id: 53008, item_count: 1, price_id: 4, price_count: 1680, buy_limit: 1 },
    PayCatalogItem { id: 106100007, shop_id: 6, tab_id: 1, item_id: 53009, item_count: 1, price_id: 4, price_count: 1680, buy_limit: 1 },
    PayCatalogItem { id: 106100008, shop_id: 6, tab_id: 1, item_id: 53010, item_count: 1, price_id: 4, price_count: 1680, buy_limit: 1 },
    PayCatalogItem { id: 106100009, shop_id: 6, tab_id: 1, item_id: 53011, item_count: 1, price_id: 4, price_count: 1680, buy_limit: 1 },
];

fn recommend_tab_image(r: &PayShopRecommend) -> &'static [u8] {
    match r.PrefabPath.as_str() {
        "UiItem_ShopSkinQianXiao" => b"T_ShopTabRoleQianxiao1",
        "UiItem_ShopSkinLinNai" => b"T_ShopTabRoleLinnai1",
        "UiItem_ShopSkinFeixue" => b"T_ShopTabRoleFeixue1",
        "UiItem_ShopSkinMoNing" => b"T_ShopTabRoleMoning",
        "UiItem_ShopSkinMoto" => b"T_ShopTabMoto",
        "UiItem_ShopSkinMotoFeixue" => b"T_ShopTabMotorFeixue",
        "UiItem_ShopSkinMotoYangyang" => b"T_ShopTabMotoYangyang",
        _ => b"T_ShopTabMonthlyCard",
    }
}

/// Client subtab widgets call `SetText(..., true)` literally without `SetLocalTextNew`,
/// so resolve raw table keys to readable live Store labels.
fn localize_shop_tab_name(shop_id: i32, tab_id: i32, raw: &str) -> Vec<u8> {
    let text = match (shop_id, tab_id) {
        (1, 1) => "Featured",
        (1, 2) => "Recommended",
        (3, 1) => "Featured Bundles",
        (3, 2) => "Permanent Bundles",
        (3, 3) => "Special Bundles",
        (3, 4) => "Growth Bundles",
        (3, 5) => "Supply Bundles",
        (4, 1) => "Afterglow Coral",
        (4, 2) => "Oscillated Coral",
        (4, 3) => "Astrite Exchange",
        (4, 4) => "Resonance Exchange",
        (5, 1) => "Hazard Record",
        (5, 2) => "Tactical Hologram",
        (5, 3) => "Thousand Gateways",
        (5, 4) => "Pioneer Exchange",
        (5, 5) => "Event Points",
        (6, 1) => "Outfits",
        (6, 2) => "Flight Aspects",
        (6, 3) => "Motorbike Coatings",
        (6, 4) => "Accessories",
        (7, 1) => "Voyager Supply",
        (7, 2) => "Starter Bundles",
        (7, 3) => "Growth Supply",
        (100, _) => "Lunite Purchase",
        _ if raw.contains("Rogue") => "Supply Exchange",
        _ => raw,
    };
    text.as_bytes().to_vec()
}

/// Resolves `PayShopRecommend.TabName` keys since `PayShopRecommendSwitchItem` uses literal `SetText`.
fn localize_recommend_tab_name(r: &PayShopRecommend) -> Vec<u8> {
    let text = match r.Id {
        1 => "Lunite Subscription",
        2 => "Laurel Nymph",
        3 => "Splashing Summer",
        4 => "Zani Outfit",
        5 => "15-Day Supply",
        6 => "Motorbike Frame I",
        7 => "Feixue Motorbike",
        8 => "Mornye Outfit",
        9 => "Chisa Outfit",
        10 => "Lynae Outfit",
        11 => "Yangyang Motorbike",
        12 => "Feixue Outfit",
        _ => r.TabName.as_str(),
    };
    text.as_bytes().to_vec()
}

fn build_pay_shop_item_pb(entry: &PayCatalogItem, sort: i32, bought_count: i32) -> PayShopItem {
    let sold_out = entry.buy_limit > 0 && bought_count >= entry.buy_limit;
    let is_monthly = entry.buy_limit > 0 && entry.shop_id == 4 && entry.tab_id == 2;
    let update_type = if is_monthly {
        PayUpdateType::Monthly
    } else if entry.buy_limit > 0 {
        PayUpdateType::Forever
    } else {
        PayUpdateType::None
    };
    let update_time = if is_monthly {
        energy::now_seconds() + 30 * 86_400
    } else {
        0
    };
    PayShopItem {
        Id: entry.id,
        ItemId: entry.item_id,
        ItemCount: entry.item_count,
        Locked: false,
        BuyLimit: entry.buy_limit,
        BoughtCount: bought_count,
        Price: Some(PayShopPrice {
            Id: entry.price_id,
            Count: entry.price_count,
            PromotionCount: entry.price_count,
        }),
        UpdateType: Some(update_type),
        UpdateTime: update_time,
        ShopItemType: Some(PayShopItemType::Normal),
        CanBuyGoods: !sold_out,
        TabId: entry.tab_id,
        ShopId: entry.shop_id,
        Sort: sort,
        SoldOut: sold_out,
        Show: true,
        Quality: 0,
        IsBuyMaxButton: entry.buy_limit != 1,
        ..Default::default()
    }
}

fn build_pay_shop_info(shop_cfg: &PayShop, progress: &ProgressInfo) -> PayShopInfo {
    let items = DEFAULT_PAY_CATALOG
        .iter()
        .enumerate()
        .filter(|(_, entry)| entry.shop_id == shop_cfg.Id)
        .map(|(idx, entry)| {
            let bought = ProgressInfo::get_id_count(&progress.shop.payshop_bought, entry.id);
            build_pay_shop_item_pb(entry, (idx + 1) as i32, bought)
        })
        .collect();

    PayShopInfo {
        Id: shop_cfg.Id,
        Items: items,
        ShopTabViewType: shop_cfg.ShopTabViewType,
        DynamicTabId: shop_cfg.DynamicTabId,
        Sort: shop_cfg.Sort,
        Money: shop_cfg.Money.clone(),
        SortRule: shop_cfg.SortRule,
        ..Default::default()
    }
}

pub fn handle_pay_shop_info_request(
    state: &mut State,
    req: PayShopInfoRequest,
) -> Result<PayShopInfoResponse, String> {
    let infos = state
        .assets
        .tables
        .pay_shop
        .items
        .iter()
        .map(|shop_cfg| build_pay_shop_info(shop_cfg, &state.player_components.progress.info))
        .collect();

    let tabs = state
        .assets
        .tables
        .pay_shop_tab
        .items
        .iter()
        .map(|t| ShopTab {
            ShopId: t.ShopId,
            TabId: t.TabId,
            Sort: t.Sort,
            name: localize_shop_tab_name(t.ShopId, t.TabId, &t.Name),
            Logic: t.Logic,
            Enable: t.Enable,
            ..Default::default()
        })
        .collect();

    let recs = state
        .assets
        .tables
        .pay_shop_recommend
        .items
        .iter()
        .filter(|r| r.Show)
        .map(|r| ShopRecommend {
            Id: r.Id,
            RecommendType: r.RecommendType,
            RecommendId: r.RecommendId,
            TabName: localize_recommend_tab_name(r),
            PrefabPath: if r.PrefabPath.is_empty() {
                b"UiItem_MonthlyCardStore".to_vec()
            } else {
                r.PrefabPath.as_bytes().to_vec()
            },
            Sort: r.Sort,
            Show: r.Show,
            TabImage: recommend_tab_image(r).to_vec(),
        })
        .collect();

    let pay_gift_shop_info = qol::build_pay_gift_shop_info(state, req.Version);

    Ok(PayShopInfoResponse {
        Infos: infos,
        Version: b"3.8.11".to_vec(),
        ErrorCode: Some(ErrorCode::Success),
        PayGiftShopInfo: Some(pay_gift_shop_info),
        PayShopTabData: tabs,
        PayShopRecommendData: recs,
    })
}

pub fn handle_pay_shop_update_request(
    state: &mut State,
    req: PayShopUpdateRequest,
) -> Result<PayShopUpdateResponse, String> {
    let Some(shop_cfg) = state.assets.tables.pay_shop.get_data_by_id(&req.Id) else {
        return Ok(PayShopUpdateResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
            Info: None,
        });
    };
    let info = build_pay_shop_info(&shop_cfg, &state.player_components.progress.info);
    Ok(PayShopUpdateResponse {
        Info: Some(info),
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_pay_shop_item_update_request(
    state: &mut State,
    req: PayShopItemUpdateRequest,
) -> Result<PayShopItemUpdateResponse, String> {
    let progress = &state.player_components.progress.info;
    let mut items = Vec::new();
    for id in req.ShopItemIds {
        for (idx, entry) in DEFAULT_PAY_CATALOG.iter().enumerate() {
            if entry.id == id {
                let bought = ProgressInfo::get_id_count(&progress.shop.payshop_bought, id);
                items.push(build_pay_shop_item_pb(entry, (idx + 1) as i32, bought));
            }
        }
    }
    Ok(PayShopItemUpdateResponse {
        ErrorCode: Some(ErrorCode::Success),
        Items: items,
    })
}

pub fn handle_pay_shop_buy_request(
    state: &mut State,
    req: PayShopBuyRequest,
) -> Result<PayShopBuyResponse, String> {
    let count = req.Count;
    if count <= 0 {
        return Ok(PayShopBuyResponse {
            Id: req.Id,
            Count: 0,
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    }
    let Some(item) = DEFAULT_PAY_CATALOG.iter().find(|entry| entry.id == req.Id) else {
        return Ok(PayShopBuyResponse {
            Id: req.Id,
            Count: 0,
            ErrorCode: Some(ErrorCode::ErrItemCfgNotFound),
        });
    };

    let bought = ProgressInfo::get_id_count(
        &state.player_components.progress.info.shop.payshop_bought,
        item.id,
    );
    if item.buy_limit > 0 && bought + count > item.buy_limit {
        return Ok(PayShopBuyResponse {
            Id: item.id,
            Count: bought,
            ErrorCode: Some(ErrorCode::ErrItemCount),
        });
    }

    let (Some(total_cost), Some(total_reward)) = (
        item.price_count.checked_mul(count),
        item.item_count.checked_mul(count),
    ) else {
        return Ok(PayShopBuyResponse {
            Id: item.id,
            Count: bought,
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    };

    let costs = [sonoro_gamesv_logic::helpers::consume::CostItem {
        id: item.price_id,
        count: total_cost,
    }];
    let outcome = {
        let mut ctx = state.craft_context();
        craft::consume_and_grant(&mut ctx, &costs, item.item_id, total_reward, 2)
            .map_err(|e| e.to_string())?
    };
    if !outcome.ok {
        return Ok(PayShopBuyResponse {
            Id: item.id,
            Count: bought,
            ErrorCode: Some(outcome.err),
        });
    }

    ProgressInfo::add_id_count(
        &mut state.player_components.progress.info.shop.payshop_bought,
        item.id,
        count,
    );
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;

    Ok(PayShopBuyResponse {
        Count: count,
        Id: item.id,
        ErrorCode: Some(ErrorCode::Success),
    })
}

fn build_fixed_shop_pb(
    shop_id: i32,
    assets: &sonoro_gamesv_data::Assets,
    progress: &ProgressInfo,
) -> ShopInfo {
    let items = assets
        .tables
        .shop_fixed
        .items
        .iter()
        .filter(|row| row.ShopId == shop_id)
        .map(|row| {
            let money_list = row
                .Price
                .iter()
                .map(|(&id, &num)| ShoppMoneyInfo {
                    MoneyId: id,
                    MoneyNum: num,
                })
                .collect();
            let orig_list = row
                .OriginalPrice
                .iter()
                .map(|(&id, &num)| ShoppMoneyInfo {
                    MoneyId: id,
                    MoneyNum: num,
                })
                .collect();
            let bought = progress.get_shop_good_count(shop_id, row.Id);
            ShopItemInfoNew {
                Id: row.Id,
                BoughtCount: bought,
                Lock: false,
                ItemId: row.ItemId,
                ItemNum: row.ItemNum,
                MoneyList: money_list,
                LimitNum: row.LimitNum,
                OriginalMoneyList: orig_list,
                Label: row.Label.as_bytes().to_vec(),
                UnlockConditionId: row.CondId,
                SortIndex: row.SortIndex,
                ..Default::default()
            }
        })
        .collect();

    ShopInfo {
        ShopId: shop_id,
        UpdateTime: 0,
        ItemInfoList: items,
    }
}

pub fn handle_shop_info_request(
    state: &mut State,
    _req: ShopInfoRequest,
) -> Result<ShopInfoResponse, String> {
    let progress = &state.player_components.progress.info;
    let list = state
        .assets
        .tables
        .shop_info
        .items
        .iter()
        .map(|shop_cfg| build_fixed_shop_pb(shop_cfg.Id, &state.assets, progress))
        .collect();

    Ok(ShopInfoResponse {
        ShopList: list,
        VersionStr: b"3.8.11".to_vec(),
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_shop_update_request(
    state: &mut State,
    req: ShopUpdateRequest,
) -> Result<ShopUpdateResponse, String> {
    let mut shop_info = build_fixed_shop_pb(
        req.ShopId,
        &state.assets,
        &state.player_components.progress.info,
    );
    shop_info.UpdateTime = energy::now_seconds().clamp(0, i64::from(u32::MAX)) as u32;
    Ok(ShopUpdateResponse {
        ErrorCode: Some(ErrorCode::Success),
        Info: Some(shop_info),
    })
}

pub fn handle_shop_buy_request(
    state: &mut State,
    req: ShopBuyRequest,
) -> Result<ShopBuyResponse, String> {
    let Some(row) = state
        .assets
        .tables
        .shop_fixed
        .get_data_by_id(&req.Id)
        .map(|r| r.into_owned())
    else {
        return Ok(ShopBuyResponse {
            Id: req.Id,
            ShopId: req.ShopId,
            BoughtCount: 0,
            ErrorCode: Some(ErrorCode::ErrItemCfgNotFound),
        });
    };
    if req.ShopId != 0 && req.ShopId != row.ShopId {
        return Ok(ShopBuyResponse {
            Id: req.Id,
            ShopId: req.ShopId,
            BoughtCount: 0,
            ErrorCode: Some(ErrorCode::ErrShopIdNotExit),
        });
    }
    let count = req.Num;
    if count <= 0 {
        return Ok(ShopBuyResponse {
            Id: req.Id,
            ShopId: req.ShopId,
            BoughtCount: 0,
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    }
    if row.CondId > 0 && !condition::eval_condition_id(&state.eval_context(), row.CondId) {
        return Ok(ShopBuyResponse {
            Id: req.Id,
            ShopId: row.ShopId,
            BoughtCount: 0,
            ErrorCode: Some(ErrorCode::ErrShopCondLimit),
        });
    }

    let bought = state
        .player_components
        .progress
        .info
        .get_shop_good_count(row.ShopId, row.Id);
    if row.LimitNum > 0 && bought.saturating_add(count) > row.LimitNum {
        return Ok(ShopBuyResponse {
            Id: req.Id,
            ShopId: row.ShopId,
            BoughtCount: bought,
            ErrorCode: Some(ErrorCode::ErrItemCount),
        });
    }

    let (cost_id, cost_unit) = if req.MoneyId > 0 {
        if let Some(&unit_price) = row.Price.get(&req.MoneyId) {
            (req.MoneyId, unit_price)
        } else {
            return Ok(ShopBuyResponse {
                Id: req.Id,
                ShopId: row.ShopId,
                BoughtCount: bought,
                ErrorCode: Some(ErrorCode::ErrShopMoneyId),
            });
        }
    } else if let Some((&k, &v)) = row.Price.iter().next() {
        (k, v)
    } else {
        (0, 0)
    };

    if !row.Price.is_empty() && cost_id == 0 {
        return Ok(ShopBuyResponse {
            Id: req.Id,
            ShopId: row.ShopId,
            BoughtCount: bought,
            ErrorCode: Some(ErrorCode::ErrShopMoneyId),
        });
    }

    let (Some(total_cost), Some(total_items)) =
        (cost_unit.checked_mul(count), row.ItemNum.checked_mul(count))
    else {
        return Ok(ShopBuyResponse {
            Id: req.Id,
            ShopId: row.ShopId,
            BoughtCount: 0,
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    };

    let costs_arr;
    let costs: &[sonoro_gamesv_logic::helpers::consume::CostItem] =
        if cost_id > 0 && total_cost > 0 {
            costs_arr = [sonoro_gamesv_logic::helpers::consume::CostItem {
                id: cost_id,
                count: total_cost,
            }];
            &costs_arr
        } else {
            &[]
        };

    let outcome = {
        let mut ctx = state.craft_context();
        craft::consume_and_grant(&mut ctx, costs, row.ItemId, total_items, 2)
            .map_err(|e| e.to_string())?
    };
    if !outcome.ok {
        return Ok(ShopBuyResponse {
            Id: req.Id,
            ShopId: row.ShopId,
            BoughtCount: bought,
            ErrorCode: Some(outcome.err),
        });
    }

    let new_bought = state
        .player_components
        .progress
        .info
        .add_shop_good_count(row.ShopId, row.Id, count);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;

    let shop_info = build_fixed_shop_pb(
        row.ShopId,
        &state.assets,
        &state.player_components.progress.info,
    );
    state.conn.push(
        ShopInfoUpdateNotify::MSG_ID,
        &ShopInfoUpdateNotify {
            ShopInfo: Some(shop_info),
        },
    );

    Ok(ShopBuyResponse {
        Id: row.Id,
        ShopId: row.ShopId,
        BoughtCount: new_bought,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_item_exchange_request(
    state: &mut State,
    req: ItemExchangeRequest,
) -> Result<ItemExchangeResponse, String> {
    let item_id = req.ItemId;
    let times = req.ExchangeTimes;
    if times <= 0 || item_id <= 0 {
        return Ok(ItemExchangeResponse {
            ItemId: item_id,
            ItemCount: 0,
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    }
    let Some(ex_cfg) = state
        .assets
        .tables
        .item_exchange_content
        .get_data_by_id(&item_id)
        .map(|r| r.into_owned())
    else {
        return Ok(ItemExchangeResponse {
            ItemId: item_id,
            ItemCount: 0,
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    };
    let Some((&price_id, &unit_price)) = ex_cfg.Consume.iter().next() else {
        return Ok(ItemExchangeResponse {
            ItemId: item_id,
            ItemCount: 0,
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    };
    let (Some(total_cost), Some(gain_count)) = (
        unit_price.checked_mul(times),
        ex_cfg.GainCount.max(1).checked_mul(times),
    ) else {
        return Ok(ItemExchangeResponse {
            ItemId: item_id,
            ItemCount: 0,
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    };

    let costs = [sonoro_gamesv_logic::helpers::consume::CostItem {
        id: price_id,
        count: total_cost,
    }];
    let outcome = {
        let mut ctx = state.craft_context();
        craft::consume_and_grant(&mut ctx, &costs, item_id, gain_count, 2)
            .map_err(|e| e.to_string())?
    };
    if !outcome.ok {
        return Ok(ItemExchangeResponse {
            ItemId: item_id,
            ItemCount: 0,
            ErrorCode: Some(outcome.err),
        });
    }

    ProgressInfo::add_id_count(
        &mut state
            .player_components
            .progress
            .info
            .shop
            .item_exchange_counts,
        item_id,
        times,
    );
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;

    Ok(ItemExchangeResponse {
        ErrorCode: Some(ErrorCode::Success),
        ItemId: item_id,
        ItemCount: gain_count,
    })
}
