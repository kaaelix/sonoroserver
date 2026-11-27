use std::collections::BTreeMap;
use std::sync::OnceLock;

use serde_json::Value;
use sonoro_gamesv_data::Assets;
use sonoro_gamesv_fs::progress_info::{FishingCabinItem, ProgressInfo};
use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::helpers::consume::CostItem;
use sonoro_gamesv_logic::helpers::reward::RewardItem;
use sonoro_gamesv_logic::helpers::{consume, pb_kv, quest_engine, reward};
use sonoro_proto::enums::{CabinType, ErrorCode, FishCup, FishingItemRotate};
use sonoro_proto::pb::{
    CabinInfo, FishingActivityLimitRewardRequest,
    FishingActivityLimitRewardResponse, FishingActivityMilestoneRewardRequest,
    FishingActivityMilestoneRewardResponse, FishingCabinPutRequest, FishingCabinPutResponse,
    FishingCageInfoRequest, FishingCageInfoResponse, FishingCompleteRequest,
    FishingCompleteResponse, FishingEntrustAcceptRequest, FishingEntrustAcceptResponse,
    FishingEntrustHandInRequest, FishingEntrustHandInResponse, FishingEntrustRefreshRequest,
    FishingEntrustRefreshResponse, FishingEntrustTraceRequest, FishingEntrustTraceResponse,
    FishingExitRequest, FishingExitResponse, FishingHandInRequest, FishingHandInResponse,
    FishingIllustratedInfo, FishingIllustratedRewardRequest, FishingIllustratedRewardResponse,
    FishingInfo, FishingInfoRequest, FishingInfoResponse, FishingItemInfo, FishingPointInfoRequest,
    FishingPointInfoResponse, FishingQuickSellPreviewRequest, FishingQuickSellPreviewResponse,
    FishingQuickSellRequest, FishingQuickSellResponse, FishingSailingRequest,
    FishingSailingResponse, FishingSellPreviewRequest, FishingSellPreviewResponse,
    FishingSellRequest, FishingSellResponse, FishingShipFixRequest, FishingShipFixResponse,
    FishingShipInfo, FishingShipSkinChangeRequest, FishingShipSkinChangeResponse,
    FishingSkillBaitRequest, FishingSkillBaitResponse, FishingSkillBombRequest,
    FishingSkillBombResponse, FishingSkillTeleportRequest, FishingSkillTeleportResponse,
    FishingTechInfo, FishingTechLevelUpRequest, FishingTechLevelUpResponse,
    FishingTeleportToBoatRequest, FishingTeleportToBoatResponse, IntVector2D, OneFishCage,
    OneFishPointInfo,
};
use sonoro_proto::wire::PbEnum;

use super::State;

fn cabin_item_to_pb(rec: &FishingCabinItem) -> FishingItemInfo {
    FishingItemInfo {
        ItemId: rec.item_id,
        IncrId: rec.incr_id,
        Rotate: Some(FishingItemRotate::from_i32_or_default(rec.rotate)),
        Pos: Some(IntVector2D { X: rec.x, Y: rec.y }),
        Size: rec.size,
        Cup: Some(FishCup::from_i32_or_default(rec.cup)),
        Quality: rec.quality,
        Price: rec.price,
    }
}

fn tech_effect_consume(assets: &Assets, effect_id: i32) -> Vec<CostItem> {
    static TABLE: OnceLock<Vec<Value>> = OnceLock::new();
    let items = TABLE.get_or_init(|| assets.read_raw_json_array("FishingTechEffect"));
    for entry in items {
        let Some(obj) = entry.as_object() else {
            continue;
        };
        let Some(id) = obj.get("Id").and_then(Value::as_i64) else {
            continue;
        };
        if id != i64::from(effect_id) {
            continue;
        }
        let Some(consume_obj) = obj.get("Consume").and_then(Value::as_object) else {
            return Vec::new();
        };
        let mut costs = Vec::new();
        for (k, v) in consume_obj {
            let Ok(item_id) = k.parse::<i32>() else {
                continue;
            };
            let Some(cnt) = v.as_i64() else {
                continue;
            };
            let cnt = cnt.clamp(0, i64::from(i32::MAX)) as i32;
            if item_id > 0 && cnt > 0 {
                costs.push(CostItem {
                    id: item_id,
                    count: cnt,
                });
            }
        }
        return costs;
    }
    Vec::new()
}

struct FishPick {
    item_id: i32,
    quality: i32,
    price: i32,
    size: i32,
}

fn fish_pick_from_item(assets: &Assets, item_id: i32) -> FishPick {
    if let Some(fi) = assets.tables.fishing_item.get_data_by_id(&item_id) {
        let price = fi.Price.first().copied().unwrap_or(100);
        FishPick {
            item_id: fi.Id,
            quality: fi.Quality,
            price,
            size: 100,
        }
    } else {
        FishPick {
            item_id,
            quality: 2,
            price: 100,
            size: 100,
        }
    }
}

fn pick_fish_for_point(assets: &Assets, entity_id: i64, seed: i64) -> FishPick {
    static POINTS: OnceLock<Vec<Value>> = OnceLock::new();
    let points = POINTS.get_or_init(|| assets.read_raw_json_array("FishingPoint"));
    for entry in points {
        let Some(obj) = entry.as_object() else {
            continue;
        };
        let Some(eid) = obj
            .get("EntityConfigId")
            .or_else(|| obj.get("Id"))
            .and_then(Value::as_i64)
        else {
            continue;
        };
        if eid != entity_id {
            continue;
        }
        if let Some(rw) = obj.get("RewardItem").and_then(Value::as_object)
            && !rw.is_empty()
        {
            let pick_idx = seed.rem_euclid(rw.len() as i64) as usize;
            if let Some((k, _)) = rw.iter().nth(pick_idx)
                && let Ok(item_id) = k.parse::<i32>()
            {
                return fish_pick_from_item(assets, item_id);
            }
        }
        if let Some(si) = obj.get("ShowItem").and_then(Value::as_i64)
            && si > 0
        {
            let item_id = si.clamp(1, i64::from(i32::MAX)) as i32;
            if let Some(fi) = assets.tables.fishing_item.get_data_by_id(&item_id) {
                let price = fi.Price.first().copied().unwrap_or(100);
                return FishPick {
                    item_id: fi.Id,
                    quality: fi.Quality,
                    price,
                    size: 100,
                };
            }
        }
        break;
    }
    let items = &assets.tables.fishing_item.items;
    if !items.is_empty() {
        let idx = seed.rem_euclid(items.len() as i64) as usize;
        let fi = &items[idx];
        let price = fi.Price.first().copied().unwrap_or(100);
        return FishPick {
            item_id: fi.Id,
            quality: fi.Quality,
            price,
            size: 100,
        };
    }
    FishPick {
        item_id: 80121101,
        quality: 2,
        price: 12,
        size: 100,
    }
}

pub fn handle_fishing_tech_level_up_request(
    state: &mut State,
    req: FishingTechLevelUpRequest,
) -> Result<FishingTechLevelUpResponse, String> {
    let node_id = req.NodeId;
    let Some(row) = state.assets.tables.fishing_tech.get_data_by_id(&node_id) else {
        return Ok(FishingTechLevelUpResponse {
            ErrorCode: Some(ErrorCode::ErrFishingTechConfig),
            FishingTech: None,
        });
    };
    let fishing = &state.player_components.progress.info.fishing;
    if row.PreNode != 0 && ProgressInfo::get_id_count(&fishing.tech_levels, row.PreNode) <= 0 {
        return Ok(FishingTechLevelUpResponse {
            ErrorCode: Some(ErrorCode::ErrFishingTechConfig),
            FishingTech: None,
        });
    }
    let cur_lvl = ProgressInfo::get_id_count(&fishing.tech_levels, node_id);
    if cur_lvl < 0 || (cur_lvl as usize) >= row.Effect.len() {
        return Ok(FishingTechLevelUpResponse {
            FishingTech: Some(FishingTechInfo {
                NodeId: node_id,
                Level: cur_lvl,
                CanUnlock: true,
            }),
            ErrorCode: Some(ErrorCode::Success),
        });
    }
    let effect_id = row.Effect[cur_lvl as usize];
    let costs = tech_effect_consume(&state.assets, effect_id);
    if !costs.is_empty() {
        let ok = consume::consume_costs(
            &mut state.conn,
            &state.fs,
            &mut state.player_components.basic,
            &mut state.player_components.inventory,
            &costs,
        )
        .map_err(|e| e.to_string())?;
        if !ok {
            return Ok(FishingTechLevelUpResponse {
                ErrorCode: Some(ErrorCode::ErrItemNotEnough),
                FishingTech: None,
            });
        }
    }
    let new_lvl = ProgressInfo::add_id_count(
        &mut state.player_components.progress.info.fishing.tech_levels,
        node_id,
        1,
    );
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(FishingTechLevelUpResponse {
        FishingTech: Some(FishingTechInfo {
            NodeId: node_id,
            Level: new_lvl,
            CanUnlock: true,
        }),
        ErrorCode: Some(ErrorCode::Success),
    })
}

fn upsert_cabin_items(
    cabin_type: i32,
    cabin_id: i32,
    items: &[FishingItemInfo],
    progress: &mut ProgressInfo,
) {
    for it in items {
        if it.IncrId <= 0 {
            continue;
        }
        let pos = it.Pos.clone().unwrap_or_default();
        let rot = it.Rotate.map_or(0, |r| r.to_i32());
        let cup = it.Cup.map_or(0, |c| c.to_i32());
        if let Some(existing) = progress
            .fishing
            .cabin_items
            .iter_mut()
            .find(|e| e.incr_id == it.IncrId)
        {
            existing.cabin_type = cabin_type;
            existing.id = cabin_id;
            existing.item_id = it.ItemId;
            existing.x = pos.X;
            existing.y = pos.Y;
            existing.rotate = rot;
            existing.size = it.Size;
            existing.cup = cup;
            existing.quality = it.Quality;
            existing.price = it.Price;
        } else {
            progress.fishing.cabin_items.push(FishingCabinItem {
                cabin_type,
                id: cabin_id,
                incr_id: it.IncrId,
                item_id: it.ItemId,
                x: pos.X,
                y: pos.Y,
                rotate: rot,
                size: it.Size,
                cup,
                quality: it.Quality,
                price: it.Price,
            });
        }
    }
}

pub fn handle_fishing_cabin_put_request(
    state: &mut State,
    req: FishingCabinPutRequest,
) -> Result<FishingCabinPutResponse, String> {
    let ctype = req.CabinType.map_or(0, |c| c.to_i32());
    let pc = &mut state.player_components.progress;
    if req.RemoveIncrId > 0
        && let Some(pos) = pc
            .info
            .fishing
            .cabin_items
            .iter()
            .position(|e| e.incr_id == req.RemoveIncrId)
    {
        pc.info.fishing.cabin_items.remove(pos);
    }
    upsert_cabin_items(ctype, req.Id, &req.LeftFishingItems, &mut pc.info);
    upsert_cabin_items(ctype, req.Id, &req.RightFishingItems, &mut pc.info);
    pc.save(&state.fs).map_err(|e| e.to_string())?;
    Ok(FishingCabinPutResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

fn sell_fish_value(assets: &Assets, rec: &FishingCabinItem) -> i32 {
    if rec.price > 0 {
        return rec.price;
    }
    if let Some(row) = assets.tables.fishing_item.get_data_by_id(&rec.item_id)
        && let Some(&p) = row.Price.first()
    {
        return p;
    }
    100
}

pub fn handle_fishing_sell_request(
    state: &mut State,
    req: FishingSellRequest,
) -> Result<FishingSellResponse, String> {
    let mut total_coins = 0i32;
    let mut keep = Vec::new();
    for rec in state.player_components.progress.info.fishing.cabin_items.drain(..) {
        if req.IncrIds.contains(&rec.incr_id) {
            total_coins += sell_fish_value(&state.assets, &rec);
        } else {
            keep.push(rec);
        }
    }
    state.player_components.progress.info.fishing.cabin_items = keep;
    if total_coins > 0 {
        state.player_components.progress.info.fishing.total_sold += total_coins;
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        let mut rctx = state.reward_context();
        reward::grant_items(
            &mut rctx,
            0,
            false,
            &[RewardItem {
                item_id: 80100001,
                count: total_coins,
            }],
            2,
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(FishingSellResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_fishing_quick_sell_request(
    state: &mut State,
    _req: FishingQuickSellRequest,
) -> Result<FishingQuickSellResponse, String> {
    let total_coins: i32 = state
        .player_components
        .progress
        .info
        .fishing
        .cabin_items
        .iter()
        .map(|rec| sell_fish_value(&state.assets, rec))
        .sum();
    if !state
        .player_components
        .progress
        .info
        .fishing
        .cabin_items
        .is_empty()
    {
        state
            .player_components
            .progress
            .info
            .fishing
            .cabin_items
            .clear();
        state.player_components.progress.info.fishing.total_sold += total_coins;
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    if total_coins > 0 {
        let mut rctx = state.reward_context();
        reward::grant_items(
            &mut rctx,
            0,
            false,
            &[RewardItem {
                item_id: 80100001,
                count: total_coins,
            }],
            2,
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(FishingQuickSellResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_fishing_info_request(
    state: &mut State,
    _req: FishingInfoRequest,
) -> Result<FishingInfoResponse, String> {
    let pc = &state.player_components.progress.info;
    let eq_skin = pc.get_key_int("fish:ship_skin").max(1) as i32;
    let mut unlock_skins = vec![1];
    if eq_skin != 1 {
        unlock_skins.push(eq_skin);
    }
    let techs = state
        .assets
        .tables
        .fishing_tech
        .items
        .iter()
        .map(|row| {
            let lvl = ProgressInfo::get_id_count(&pc.fishing.tech_levels, row.Id);
            let can_unlock =
                row.PreNode == 0 || ProgressInfo::get_id_count(&pc.fishing.tech_levels, row.PreNode) > 0;
            FishingTechInfo {
                NodeId: row.Id,
                Level: lvl,
                CanUnlock: can_unlock,
            }
        })
        .collect();
    let ship_type = CabinType::ShipCabin.to_i32();
    let ship_items = pc
        .fishing
        .cabin_items
        .iter()
        .filter(|r| r.cabin_type == ship_type)
        .map(cabin_item_to_pb)
        .collect();

    Ok(FishingInfoResponse {
        FishingInfo: Some(FishingInfo {
            TraceEntrusts: pc.get_key_int("fish:trace_entrust").max(0) as i32,
            FishingTech: techs,
            ShipInfo: Some(FishingShipInfo {
                SkinId: eq_skin,
                PortId: pc.get_key_int("fish:port").max(1) as i32,
                IsInPort: pc.get_key_int("fish:sailing") == 0,
                CabinInfo: Some(CabinInfo {
                    CabinShape: 700001,
                    QuickSellShape: 700005,
                    FishingItem: ship_items,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            IllustratedInfo: Some(FishingIllustratedInfo::default()),
            UnlockPortId: vec![1, 2, 3, 4],
            UnlockSkins: unlock_skins,
            EntrustRefreshRatio: 100,
            ..Default::default()
        }),
    })
}

pub fn handle_fishing_activity_limit_reward_request(
    state: &mut State,
    req: FishingActivityLimitRewardRequest,
) -> Result<FishingActivityLimitRewardResponse, String> {
    if state
        .player_components
        .progress
        .info
        .mark_claim(&format!("fish:lim:{}", req.TaskId))
    {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        let mut rctx = state.reward_context();
        reward::grant_table_reward(
            &mut rctx,
            "FishingActivityLimitTask",
            i64::from(req.TaskId),
            false,
            true,
            1,
            &[],
            None,
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(FishingActivityLimitRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_fishing_activity_milestone_reward_request(
    state: &mut State,
    req: FishingActivityMilestoneRewardRequest,
) -> Result<FishingActivityMilestoneRewardResponse, String> {
    let mut dirty = false;
    for mid in req.MilestoneId {
        if state
            .player_components
            .progress
            .info
            .mark_claim(&format!("fish:ms:{mid}"))
        {
            dirty = true;
            let mut rctx = state.reward_context();
            reward::grant_table_reward(
                &mut rctx,
                "FishingActivityMilestone",
                i64::from(mid),
                false,
                true,
                1,
                &[],
                None,
            )
            .map_err(|e| e.to_string())?;
        }
    }
    if dirty {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(FishingActivityMilestoneRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_fishing_illustrated_reward_request(
    state: &mut State,
    req: FishingIllustratedRewardRequest,
) -> Result<FishingIllustratedRewardResponse, String> {
    if state
        .player_components
        .progress
        .info
        .mark_claim(&format!("fish:ill:{}", req.RewardId))
    {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        let mut rctx = state.reward_context();
        reward::grant_table_reward(
            &mut rctx,
            "FishingIllustratedReward",
            i64::from(req.RewardId),
            false,
            true,
            1,
            &[],
            None,
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(FishingIllustratedRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_fishing_complete_request(
    state: &mut State,
    req: FishingCompleteRequest,
) -> Result<FishingCompleteResponse, String> {
    let info = &mut state.player_components.progress.info;
    let pt_cnt = info.add_key_int(&format!("fish:point_cnt:{}", req.EntityId), 1);
    let total_cnt = info.add_key_int("fish:catch_cnt", 1);
    let fish = pick_fish_for_point(&state.assets, req.EntityId, pt_cnt + total_cnt);
    let incr = info.fishing.next_fish_incr_id;
    info.fishing.next_fish_incr_id += 1;
    let rec = FishingCabinItem {
        cabin_type: CabinType::ShipCabin.to_i32(),
        id: 1,
        incr_id: incr,
        item_id: fish.item_id,
        size: fish.size,
        quality: fish.quality,
        price: fish.price,
        ..Default::default()
    };
    let pb_item = cabin_item_to_pb(&rec);
    info.fishing.cabin_items.push(rec);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(FishingCompleteResponse {
        ErrorCode: Some(ErrorCode::Success),
        Items: vec![pb_item],
        ..Default::default()
    })
}

pub fn handle_fishing_entrust_accept_request(
    state: &mut State,
    req: FishingEntrustAcceptRequest,
) -> Result<FishingEntrustAcceptResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("fish:entrust_acc:{}", req.EntrustId),
        i64::from(req.Accept),
    )
    .map_err(|e| e.to_string())?;
    Ok(FishingEntrustAcceptResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_fishing_entrust_hand_in_request(
    state: &mut State,
    events: &mut EventQueue,
    req: FishingEntrustHandInRequest,
) -> Result<FishingEntrustHandInResponse, String> {
    if state
        .player_components
        .progress
        .info
        .mark_claim(&format!("fish:entrust_done:{}", req.EntrustId))
    {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        let mut rctx = state.reward_context();
        reward::grant_table_reward(
            &mut rctx,
            "FishingEntrust",
            i64::from(req.EntrustId),
            false,
            true,
            1,
            &[],
            None,
        )
        .map_err(|e| e.to_string())?;
    }
    let mut qctx = state.quest_context(events);
    quest_engine::events::on_condition_state_changed(&mut qctx).map_err(|e| e.to_string())?;
    Ok(FishingEntrustHandInResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_fishing_entrust_refresh_request(
    state: &mut State,
    req: FishingEntrustRefreshRequest,
) -> Result<FishingEntrustRefreshResponse, String> {
    let next_id = state
        .player_components
        .progress
        .info
        .add_key_int("fish:entrust_refresh", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    let out_id = if req.EntrustId > 0 {
        req.EntrustId
    } else {
        ((next_id - 1).rem_euclid(5) + 1) as i32
    };
    Ok(FishingEntrustRefreshResponse {
        ErrorCode: Some(ErrorCode::Success),
        EntrustId: out_id,
    })
}

pub fn handle_fishing_entrust_trace_request(
    state: &mut State,
    req: FishingEntrustTraceRequest,
) -> Result<FishingEntrustTraceResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "fish:trace_entrust",
        i64::from(req.EntrustId),
    )
    .map_err(|e| e.to_string())?;
    Ok(FishingEntrustTraceResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_fishing_exit_request(
    state: &mut State,
    _req: FishingExitRequest,
) -> Result<FishingExitResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "fish:sailing",
        0,
    )
    .map_err(|e| e.to_string())?;
    Ok(FishingExitResponse {})
}

pub fn handle_fishing_hand_in_request(
    state: &mut State,
    req: FishingHandInRequest,
) -> Result<FishingHandInResponse, String> {
    let info = &mut state.player_components.progress.info;
    info.add_key_int(&format!("fish:handin:{}", req.Id), 1);
    info.add_key_int("fish:sell_cnt", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    let _ = (req.LeftFishingItems.len(), req.RightFishingItems.len());
    let mut rctx = state.reward_context();
    reward::grant_table_reward(
        &mut rctx,
        "FishingDelivery",
        i64::from(req.Id),
        false,
        true,
        1,
        &[],
        None,
    )
    .map_err(|e| e.to_string())?;
    Ok(FishingHandInResponse {
        ErrorCode: Some(ErrorCode::Success),
        HandInComplete: true,
    })
}

pub fn handle_fishing_point_info_request(
    state: &mut State,
    req: FishingPointInfoRequest,
) -> Result<FishingPointInfoResponse, String> {
    let used = state
        .player_components
        .progress
        .info
        .get_key_int(&format!("fish:point_cnt:{}", req.Id));
    let remain = (5 - used.min(5)).max(0) as i32;
    Ok(FishingPointInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        FishPointInfo: Some(OneFishPointInfo {
            Id: req.Id,
            EntityConfigId: req.Id,
            CurCount: remain,
            MaxCount: 5,
            RefreshTime: 60,
            ..Default::default()
        }),
    })
}

pub fn handle_fishing_cage_info_request(
    state: &mut State,
    req: FishingCageInfoRequest,
) -> Result<FishingCageInfoResponse, String> {
    static CAGES: OnceLock<Vec<Value>> = OnceLock::new();
    let cages = CAGES.get_or_init(|| state.assets.read_raw_json_array("FishingCage"));
    let mut entity_cfg_id = req.Id;
    let mut max_count = 5i32;
    let mut refresh_time = 120i32;
    for entry in cages {
        let Some(obj) = entry.as_object() else {
            continue;
        };
        let Some(cid) = obj.get("Id").and_then(Value::as_i64) else {
            continue;
        };
        let Some(ecid) = obj.get("EntityConfigId").and_then(Value::as_i64) else {
            continue;
        };
        if cid == i64::from(req.Id) || ecid == i64::from(req.Id) {
            entity_cfg_id = ecid.clamp(0, i64::from(i32::MAX)) as i32;
            if let Some(cl) = obj.get("CountLimit").and_then(Value::as_i64)
                && cl > 0
            {
                max_count = cl.clamp(1, i64::from(i32::MAX)) as i32;
            }
            if let Some(rt) = obj.get("RefreshTime").and_then(Value::as_i64)
                && rt > 0
            {
                refresh_time = rt.clamp(1, i64::from(i32::MAX)) as i32;
            }
            break;
        }
    }

    let cage_type = CabinType::Cage.to_i32();
    let mut items: Vec<FishingItemInfo> = state
        .player_components
        .progress
        .info
        .fishing
        .cabin_items
        .iter()
        .filter(|r| r.cabin_type == cage_type && r.id == req.Id)
        .map(cabin_item_to_pb)
        .collect();
    if items.is_empty() && !state.assets.tables.fishing_item.items.is_empty() {
        let seed = state
            .player_components
            .progress
            .info
            .get_key_int("fish:catch_cnt");
        let count = (max_count.max(0) as usize).min(2);
        for i in 0..count {
            let fish = pick_fish_for_point(&state.assets, i64::from(entity_cfg_id), seed + i as i64);
            let info = &mut state.player_components.progress.info;
            let incr = info.fishing.next_fish_incr_id;
            info.fishing.next_fish_incr_id += 1;
            let rec = FishingCabinItem {
                cabin_type: cage_type,
                id: req.Id,
                incr_id: incr,
                item_id: fish.item_id,
                size: fish.size,
                quality: fish.quality,
                price: fish.price,
                ..Default::default()
            };
            items.push(cabin_item_to_pb(&rec));
            info.fishing.cabin_items.push(rec);
        }
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }

    Ok(FishingCageInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        FishCageInfo: Some(OneFishCage {
            Id: req.Id,
            EntityConfigId: entity_cfg_id,
            MaxCount: max_count,
            RefreshTime: refresh_time,
            Items: items,
            ..Default::default()
        }),
    })
}

pub fn handle_fishing_sailing_request(
    state: &mut State,
    req: FishingSailingRequest,
) -> Result<FishingSailingResponse, String> {
    let info = &mut state.player_components.progress.info;
    info.set_key_int("fish:port", i64::from(req.PortId.max(1)));
    info.set_key_int("fish:sailing", 1);
    if req.Fix {
        info.set_key_int("fish:ship_dmg", 0);
    }
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(FishingSailingResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_fishing_ship_fix_request(
    state: &mut State,
    _req: FishingShipFixRequest,
) -> Result<FishingShipFixResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "fish:ship_dmg",
        0,
    )
    .map_err(|e| e.to_string())?;
    Ok(FishingShipFixResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_fishing_ship_skin_change_request(
    state: &mut State,
    req: FishingShipSkinChangeRequest,
) -> Result<FishingShipSkinChangeResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "fish:ship_skin",
        i64::from(req.SkinId),
    )
    .map_err(|e| e.to_string())?;
    Ok(FishingShipSkinChangeResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_fishing_skill_bait_request(
    state: &mut State,
    req: FishingSkillBaitRequest,
) -> Result<FishingSkillBaitResponse, String> {
    let info = &mut state.player_components.progress.info;
    info.set_key_int("fish:active_bait", i64::from(req.ItemId));
    info.add_key_int("fish:skill_use", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(FishingSkillBaitResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_fishing_skill_bomb_request(
    state: &mut State,
    req: FishingSkillBombRequest,
) -> Result<FishingSkillBombResponse, String> {
    if req.ItemId > 0 {
        let costs = [CostItem {
            id: req.ItemId,
            count: 1,
        }];
        let ok = consume::consume_costs(
            &mut state.conn,
            &state.fs,
            &mut state.player_components.basic,
            &mut state.player_components.inventory,
            &costs,
        )
        .map_err(|e| e.to_string())?;
        if !ok {
            return Ok(FishingSkillBombResponse {
                ErrorCode: Some(ErrorCode::ErrItemNotEnough),
                Items: Vec::new(),
            });
        }
    }
    let info = &mut state.player_components.progress.info;
    info.add_key_int("fish:skill_use", 1);
    let base_cnt = info.add_key_int("fish:catch_cnt", 2);
    let mut items = Vec::with_capacity(2);
    for i in 0..2 {
        let fish = pick_fish_for_point(&state.assets, 0, base_cnt + i);
        let info = &mut state.player_components.progress.info;
        let incr = info.fishing.next_fish_incr_id;
        info.fishing.next_fish_incr_id += 1;
        let rec = FishingCabinItem {
            cabin_type: CabinType::ShipCabin.to_i32(),
            id: 1,
            incr_id: incr,
            item_id: fish.item_id,
            size: 12,
            quality: fish.quality,
            price: fish.price,
            ..Default::default()
        };
        items.push(cabin_item_to_pb(&rec));
        info.fishing.cabin_items.push(rec);
    }
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(FishingSkillBombResponse {
        ErrorCode: Some(ErrorCode::Success),
        Items: items,
    })
}

pub fn handle_fishing_skill_teleport_request(
    state: &mut State,
    _req: FishingSkillTeleportRequest,
) -> Result<FishingSkillTeleportResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int("fish:skill_use", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(FishingSkillTeleportResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_fishing_teleport_to_boat_request(
    state: &mut State,
    _req: FishingTeleportToBoatRequest,
) -> Result<FishingTeleportToBoatResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int("fish:boat_tp", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(FishingTeleportToBoatResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_fishing_quick_sell_preview_request(
    state: &mut State,
    _req: FishingQuickSellPreviewRequest,
) -> Result<FishingQuickSellPreviewResponse, String> {
    let total: i32 = state
        .player_components
        .progress
        .info
        .fishing
        .cabin_items
        .iter()
        .map(|rec| sell_fish_value(&state.assets, rec))
        .sum();
    let mut reward_items = BTreeMap::new();
    if total > 0 {
        reward_items.insert(80100001, total);
    }
    Ok(FishingQuickSellPreviewResponse {
        ErrorCode: Some(ErrorCode::Success),
        RewardItems: reward_items,
        SellItems: Vec::new(),
    })
}

pub fn handle_fishing_sell_preview_request(
    state: &mut State,
    req: FishingSellPreviewRequest,
) -> Result<FishingSellPreviewResponse, String> {
    let total: i32 = state
        .player_components
        .progress
        .info
        .fishing
        .cabin_items
        .iter()
        .filter(|rec| req.IncrIds.contains(&rec.incr_id))
        .map(|rec| sell_fish_value(&state.assets, rec))
        .sum();
    let mut reward_items = BTreeMap::new();
    if total > 0 {
        reward_items.insert(80100001, total);
    }
    Ok(FishingSellPreviewResponse {
        ErrorCode: Some(ErrorCode::Success),
        RewardItems: reward_items,
    })
}
