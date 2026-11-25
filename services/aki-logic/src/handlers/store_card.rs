//! Store, PayShop, MonthCard, WeekCard, SignActivity, and Adventure handlers.

use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

use sonoro_proto::PbMessage;
use sonoro_proto::enums::*;
use sonoro_proto::pb::*;
use sonoro_proto::registry::msg_id;

use crate::bin_data;
use crate::ipc::AkiResult;
use crate::sqlite_store;

fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

pub fn handle_battle_pass_task_take(player_id: i32, payload: &[u8]) -> AkiResult {
    let req = BattlePassTaskTakeRequest::decode(payload).unwrap_or_default();
    let mut row = sqlite_store::get_or_create_battle_pass(player_id);
    let mut taken_set: HashSet<i32> = row.taken_tasks.iter().copied().collect();
    let finished_set = sqlite_store::get_finished_bp_tasks(player_id);

    let mut resp = BattlePassTaskTakeResponse {
        ErrorCode: Some(ErrorCode::Success),
        ..Default::default()
    };
    let mut grants: HashMap<i32, i32> = HashMap::new();
    let mut gained_exp = 0;
    let cache = bin_data::get();

    for tid in req.Ids {
        if taken_set.contains(&tid) {
            continue;
        }
        let Some(cfg) = cache.bp_tasks.get(&tid) else {
            continue;
        };
        if cfg.update_type != 1 && !finished_set.contains(&tid) {
            continue;
        }
        taken_set.insert(tid);
        resp.Ids.push(tid);
        for (&k, &v) in &cfg.task_reward {
            if k == 8 {
                gained_exp += v;
            } else if k > 0 && v > 0 {
                *grants.entry(k).or_insert(0) += v;
            }
        }
    }

    let mut new_level = row.level;
    let mut new_exp = row.exp + gained_exp;
    let new_weekly = row.weekly_exp + gained_exp;
    while new_exp >= 1000 && new_level < 70 {
        new_exp -= 1000;
        new_level += 1;
    }

    if !resp.Ids.is_empty() {
        row.level = new_level;
        row.exp = new_exp;
        row.weekly_exp = new_weekly;
        row.taken_tasks = taken_set.into_iter().collect();
        sqlite_store::save_battle_pass(&row);
    }

    let notify = BattlePassExpUpdateNotify {
        Level: new_level,
        Exp: new_exp,
        WeeklyTotalExp: new_weekly,
        ..Default::default()
    };

    AkiResult::ok(msg_id::BATTLE_PASS_TASK_TAKE_RESPONSE, &resp)
        .with_push(msg_id::BATTLE_PASS_EXP_UPDATE_NOTIFY, &notify)
        .with_grants(grants)
}

pub fn handle_pay_shop_direct_buy(player_id: i32, payload: &[u8]) -> AkiResult {
    let req = PayShopDirectBuyRequest::decode(payload).unwrap_or_default();
    let shop_item_id = req.Id;
    let gift_item_id = bin_data::get()
        .pay_gift_items
        .get(&shop_item_id)
        .copied()
        .unwrap_or(0);
    let item_id = if gift_item_id > 0 { gift_item_id } else { 3 };
    let item_count = if gift_item_id > 0 { 1 } else { 60 };
    let receipt_id = format!("aki_direct_{player_id}_{shop_item_id}_{}", unix_now());

    if player_id > 0 && shop_item_id > 0 {
        sqlite_store::increment_shop_bought(player_id, 0, shop_item_id, 1);
    }

    let resp = PayShopDirectBuyResponse {
        ErrorCode: Some(ErrorCode::Success),
        ReceiptId: receipt_id.as_bytes().to_vec(),
        ..Default::default()
    };
    let notify = PayShopDirectBuyNotify {
        ReceiptId: receipt_id.into_bytes(),
        ShopItemId: shop_item_id,
        ItemId: item_id,
        ItemCount: item_count,
    };

    let mut result = AkiResult::ok(msg_id::PAY_SHOP_DIRECT_BUY_RESPONSE, &resp)
        .with_push(msg_id::PAY_SHOP_DIRECT_BUY_NOTIFY, &notify);
    if shop_item_id > 0 {
        result = result.with_grant(item_id, item_count);
    }
    result
}

pub fn handle_pay_info_request(player_id: i32, _payload: &[u8]) -> AkiResult {
    let bought_map = if player_id > 0 {
        sqlite_store::get_shop_bought_counts(player_id, 999)
    } else {
        HashMap::new()
    };

    let infos = bin_data::get()
        .pay_items
        .iter()
        .map(|item| {
            let can_special =
                item.special_bonus_item_count > 0 && !bought_map.contains_key(&item.id);
            PayItemInfo {
                Id: item.id,
                PayId: item.pay_id,
                ItemId: item.item_id,
                ItemCount: item.item_count,
                BonusItemCount: item.bonus_item_count,
                SpecialBonusItemCount: item.special_bonus_item_count,
                CanSpecialBonus: can_special,
                StageImage: item.stage_image.as_bytes().to_vec(),
                ProductId: item.product_id.as_bytes().to_vec(),
                Amount: item.amount.as_bytes().to_vec(),
                Quality: 0,
                ..Default::default()
            }
        })
        .collect();

    AkiResult::ok(
        msg_id::PAY_INFO_RESPONSE,
        &PayInfoResponse {
            Version: b"3.8.11".to_vec(),
            ErrorCode: Some(ErrorCode::Success),
            Infos: infos,
        },
    )
}

pub fn handle_pay_gift_info_request(player_id: i32, _payload: &[u8]) -> AkiResult {
    let bought_map = if player_id > 0 {
        sqlite_store::get_shop_bought_counts(player_id, 0)
    } else {
        HashMap::new()
    };

    let infos = bin_data::get()
        .pay_gift_items
        .iter()
        .map(|(&id, &item_id)| PayGiftInfo {
            Id: id,
            PayId: id,
            ItemId: item_id,
            ItemCount: 1,
            BuyLimit: 99,
            BoughtCount: bought_map.get(&id).copied().unwrap_or(0),
            BeginTime: 1,
            EndTime: 2_000_000_000,
            IsCanBuy: true,
            ..Default::default()
        })
        .collect();

    AkiResult::ok(
        msg_id::PAY_GIFT_INFO_RESPONSE,
        &PayGiftInfoResponse {
            Version: b"3.8.11".to_vec(),
            ErrorCode: Some(ErrorCode::Success),
            Infos: infos,
        },
    )
}

pub fn handle_get_adventure_reward(player_id: i32, payload: &[u8]) -> AkiResult {
    let req = GetAdventureRewardRequest::decode(payload).unwrap_or_default();
    let task_id = req.Id;
    let mut result = AkiResult::ok(
        msg_id::GET_ADVENTURE_REWARD_RESPONSE,
        &GetAdventureRewardResponse {
            Code: Some(ErrorCode::Success),
            Id: task_id,
        },
    );

    if player_id > 0 && task_id > 0 {
        let claim_key = format!("adv_task:{task_id}");
        let claims = sqlite_store::get_activity_claims(player_id);
        let (_, prog_claims) = sqlite_store::get_progress_state(player_id);
        if !claims.contains(&claim_key) && !prog_claims.contains(&claim_key) {
            sqlite_store::set_activity_claim(player_id, &claim_key, "1");
            let cache = bin_data::get();
            let drop_id = cache
                .adventure_task_drops
                .get(&task_id)
                .copied()
                .unwrap_or(0);
            let mut drops = cache.resolve_drop(drop_id);
            if drops.is_empty() {
                drops.push((3, 40));
                drops.push((1, 100));
            }
            result = result.with_grants(drops);
        }
    }

    result
}

pub fn handle_month_card(player_id: i32, _payload: &[u8]) -> AkiResult {
    let day_bucket = unix_now() / 86_400;
    let already_got = if player_id > 0 {
        sqlite_store::get_kv(player_id, "wallet", "month_card_last_day")
            .and_then(|s| s.parse::<i64>().ok())
            == Some(day_bucket)
    } else {
        false
    };

    let mut result = AkiResult::ok(
        msg_id::MONTH_CARD_RESPONSE,
        &MonthCardResponse {
            Days: 30,
            IsDailyGot: true,
            ErrorCode: Some(ErrorCode::Success),
        },
    );

    if !already_got && player_id > 0 {
        sqlite_store::set_kv(
            player_id,
            "wallet",
            "month_card_last_day",
            &day_bucket.to_string(),
        );
        result = result.with_grant(3, 90);
    }

    result
}

pub fn handle_week_card_info(player_id: i32, payload: &[u8]) -> AkiResult {
    let req = WeekCardInfoRequest::decode(payload).unwrap_or_default();
    let now_s = unix_now();
    let claims = if player_id > 0 {
        sqlite_store::get_activity_claims(player_id)
    } else {
        HashSet::new()
    };

    let mut resp = WeekCardInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        ..Default::default()
    };

    for wid in req.WeekCardIds {
        let mut content_infos = Vec::with_capacity(3);
        for day in 1..=3 {
            let cid = wid * 100 + day;
            content_infos.push(WeekCardContentInfo {
                WeekCardContentId: cid,
                UnlockDay: day,
                UnlockTimeStamp: now_s,
                Status: if claims.contains(&format!("week_card:{cid}")) {
                    2
                } else {
                    1
                },
                ..Default::default()
            });
        }
        resp.WeekCardInfos.push(WeekCardInfo {
            WeekCardId: wid,
            Days: 7,
            BuyTimeStamp: now_s,
            EndTimeStamp: now_s + 7 * 86_400,
            WeekCardContentInfos: content_infos,
        });
    }

    resp.Infos = Some(
        resp.WeekCardInfos
            .first()
            .cloned()
            .unwrap_or_else(|| WeekCardInfo {
                WeekCardId: req.WeekCardId,
                Days: 7,
                BuyTimeStamp: now_s,
                EndTimeStamp: now_s + 7 * 86_400,
                ..Default::default()
            }),
    );

    AkiResult::ok(msg_id::WEEK_CARD_INFO_RESPONSE, &resp)
}

pub fn handle_week_card_reward(player_id: i32, payload: &[u8]) -> AkiResult {
    let req = WeekCardRewardRequest::decode(payload).unwrap_or_default();
    let mut result = AkiResult::ok(
        msg_id::WEEK_CARD_REWARD_RESPONSE,
        &WeekCardRewardResponse {
            ErrorCode: Some(ErrorCode::Success),
        },
    );

    if player_id > 0 && req.WeekCardContentId > 0 {
        let key = format!("week_card:{}", req.WeekCardContentId);
        let claims = sqlite_store::get_activity_claims(player_id);
        if !claims.contains(&key) {
            sqlite_store::set_activity_claim(player_id, &key, "1");
            result = result.with_grant(3, 40).with_grant(2, 10_000);
        }
    }

    result
}

pub fn handle_sign_activity(player_id: i32, payload: &[u8]) -> AkiResult {
    let req = SignActivityRequest::decode(payload).unwrap_or_default();
    let mut resp = SignActivityResponse {
        ErrorCode: Some(ErrorCode::Success),
        ..Default::default()
    };

    if player_id > 0 {
        let key = format!("sign_act:{}:{}", req.ActivityId, req.Index);
        let claims = sqlite_store::get_activity_claims(player_id);
        if !claims.contains(&key) {
            sqlite_store::set_activity_claim(player_id, &key, "1");
            resp.ItemMap.insert(3, 30);
            resp.ItemMap.insert(2, 10_000);
            return AkiResult::ok(msg_id::SIGN_ACTIVITY_RESPONSE, &resp)
                .with_grant(3, 30)
                .with_grant(2, 10_000);
        }
    }

    AkiResult::ok(msg_id::SIGN_ACTIVITY_RESPONSE, &resp)
}

pub fn handle_sign_activity_free_drop(player_id: i32, payload: &[u8]) -> AkiResult {
    let req = SignActivityFreeDropRequest::decode(payload).unwrap_or_default();
    let mut result = AkiResult::ok(
        msg_id::SIGN_ACTIVITY_FREE_DROP_RESPONSE,
        &SignActivityFreeDropResponse {
            ErrorCode: Some(ErrorCode::Success),
        },
    );

    if player_id > 0 {
        let key = format!("sign_free:{}", req.ActivityId);
        let claims = sqlite_store::get_activity_claims(player_id);
        if !claims.contains(&key) {
            sqlite_store::set_activity_claim(player_id, &key, "1");
            result = result.with_grant(3, 60).with_grant(2, 20_000);
        }
    }

    result
}
