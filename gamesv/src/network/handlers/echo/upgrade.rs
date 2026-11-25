use std::collections::{BTreeMap, HashSet};

use sonoro_gamesv_data::Assets;
use sonoro_gamesv_data::tables::phantom_item::PhantomItem as PhantomItemConfig;
use sonoro_gamesv_data::tables::phantom_main_property::PhantomMainProperty;
use sonoro_gamesv_fs::echo_info::{self, EchoInfo, Prop, ZigDefaultPrng};
use sonoro_gamesv_logic::component::player::PlayerEchoComponent;
use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::helpers::{calabash, consume, quest_engine, reward};
use sonoro_gamesv_logic::sink::PushSink;
use sonoro_proto::enums::ErrorCode;
use sonoro_proto::pb::{
    PhantomBatchPolishRequest, PhantomBatchPolishResponse, PhantomIdentifyRequest,
    PhantomIdentifyResponse, PhantomItemRemoveNotify, PhantomItemUpdateNotify,
    PhantomLevelUpRequest, PhantomLevelUpResponse, PhantomPolishRequest, PhantomPolishResponse,
    PhantomPropInfo, PhantomVicePolishAckRequest, PhantomVicePolishAckResponse,
    PhantomVicePolishRequest, PhantomVicePolishResponse,
};

use super::super::State;
use super::shared;

fn random_seed() -> Result<u64, String> {
    let mut seed_bytes = [0u8; 8];
    getrandom::fill(&mut seed_bytes).map_err(|e| format!("rng failed: {e}"))?;
    Ok(u64::from_le_bytes(seed_bytes))
}

fn add_consume(map: &mut BTreeMap<i32, i32>, item_id: i32, count: i32) {
    *map.entry(item_id).or_insert(0) += count;
}

pub fn handle_phantom_level_up_request(
    state: &mut State,
    events: &mut EventQueue,
    req: PhantomLevelUpRequest,
) -> Result<PhantomLevelUpResponse, String> {
    let Some((item_id, item_level, item_exp, item_sub_prop_len)) = state
        .player_components
        .echo
        .echo_map
        .get(&req.IncId)
        .map(|e| (e.id, e.level, e.exp, e.sub_prop.len()))
    else {
        return Ok(PhantomLevelUpResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomItemNotExist),
            ..Default::default()
        });
    };
    let Some(item_config) = state
        .assets
        .tables
        .phantom_item
        .get_data_by_id(&item_id)
        .map(|r| r.into_owned())
    else {
        return Ok(PhantomLevelUpResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomConfigNotFound),
            ..Default::default()
        });
    };
    let Some(quality) = state
        .assets
        .tables
        .phantom_quality
        .get_data_by_id(&item_config.QualityId)
        .map(|r| r.into_owned())
    else {
        return Ok(PhantomLevelUpResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomQaulityNotExist),
            ..Default::default()
        });
    };
    let Some(max_cfg_level) =
        shared::highest_phantom_level(&state.assets, item_config.LevelUpGroupId)
    else {
        return Ok(PhantomLevelUpResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomLevelConfigNotExist),
            ..Default::default()
        });
    };
    let highest_level = quality.LevelLimit.min(max_cfg_level);

    let slot_count = req.SlotCount.max(0);
    if slot_count > 0 {
        let Some(max_sub_props) =
            echo_info::max_unlocked_sub_prop_count(&state.assets, item_config.QualityId)
        else {
            return Ok(PhantomLevelUpResponse {
                ErrorCode: Some(ErrorCode::ErrPhantomSubPropNotEnough),
                ..Default::default()
            });
        };
        if item_sub_prop_len >= max_sub_props
            || item_sub_prop_len + (slot_count as usize) > max_sub_props
        {
            return Ok(PhantomLevelUpResponse {
                ErrorCode: Some(ErrorCode::ErrPhantomSubPropNotEnough),
                ..Default::default()
            });
        }
    } else if item_level >= highest_level {
        return Ok(PhantomLevelUpResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomLvupMax),
            ..Default::default()
        });
    }

    let mut consumes = BTreeMap::new();
    let mut refund_map = BTreeMap::new();
    let mut seen_fodder = HashSet::new();
    let mut fodder_consumes: Vec<(i32, i32)> = Vec::new();
    let mut item_exp_total = 0i32;
    let mut fodder_exp_total = 0i32;

    for consume in &req.ConsumeList {
        if consume.Count <= 0 {
            return Ok(PhantomLevelUpResponse {
                ErrorCode: Some(ErrorCode::ErrPhantomConsumeItemCount),
                ..Default::default()
            });
        }
        if consume.IncId != 0 {
            if consume.IncId == req.IncId {
                return Ok(PhantomLevelUpResponse {
                    ErrorCode: Some(ErrorCode::ErrPhantomConsumeItem),
                    ..Default::default()
                });
            }
            if !seen_fodder.insert(consume.IncId) {
                return Ok(PhantomLevelUpResponse {
                    ErrorCode: Some(ErrorCode::ErrPhantomConsumeItemIncrDuplicate),
                    ..Default::default()
                });
            }
            let Some(fodder) = state.player_components.echo.echo_map.get(&consume.IncId) else {
                return Ok(PhantomLevelUpResponse {
                    ErrorCode: Some(ErrorCode::ErrPhantomItemNotExist),
                    ..Default::default()
                });
            };
            if consume.Count > fodder.stack_count.max(1) {
                return Ok(PhantomLevelUpResponse {
                    ErrorCode: Some(ErrorCode::ErrPhantomConsumeItemCount),
                    ..Default::default()
                });
            }
            if fodder.func_value == 1 {
                return Ok(PhantomLevelUpResponse {
                    ErrorCode: Some(ErrorCode::ErrPhantomLevelUpMaterialLock),
                    ..Default::default()
                });
            }
            if fodder.role_id.is_some() {
                return Ok(PhantomLevelUpResponse {
                    ErrorCode: Some(ErrorCode::ErrPhantomDecomposeEquiped),
                    ..Default::default()
                });
            }
            let Some(fodder_cfg) = state.assets.tables.phantom_item.get_data_by_id(&fodder.id)
            else {
                return Ok(PhantomLevelUpResponse {
                    ErrorCode: Some(ErrorCode::ErrPhantomConfigNotFound),
                    ..Default::default()
                });
            };
            if fodder.level == 0 && fodder.exp == 0 {
                return Ok(PhantomLevelUpResponse {
                    ErrorCode: Some(ErrorCode::ErrPhantomConsumeNoExp),
                    ..Default::default()
                });
            }
            let invested =
                shared::total_invested_phantom_exp(&state.assets, fodder_cfg.LevelUpGroupId, fodder);
            let single_exp = ((invested * 75) / 100).max(1);
            let Some(batch_exp) = single_exp.checked_mul(consume.Count) else {
                return Ok(PhantomLevelUpResponse {
                    ErrorCode: Some(ErrorCode::ErrPhantomConsumeItemCount),
                    ..Default::default()
                });
            };
            let Some(next_fodder) = fodder_exp_total.checked_add(batch_exp) else {
                return Ok(PhantomLevelUpResponse {
                    ErrorCode: Some(ErrorCode::ErrPhantomConsumeItemCount),
                    ..Default::default()
                });
            };
            fodder_exp_total = next_fodder;

            if !fodder.sub_prop.is_empty()
                && let Some(fodder_tc) = shared::tuner_cost(&state.assets, fodder_cfg.QualityId)
            {
                let refund_count = (((fodder.sub_prop.len() as i32) * fodder_tc.count * 30)
                    / 100)
                    .saturating_mul(consume.Count);
                if refund_count > 0 {
                    add_consume(&mut refund_map, fodder_tc.item_id, refund_count);
                }
            }
            fodder_consumes.push((consume.IncId, consume.Count));
        } else {
            let Some(exp_item) = state
                .assets
                .tables
                .phantom_exp_item
                .get_data_by_id(&consume.ItemId)
            else {
                return Ok(PhantomLevelUpResponse {
                    ErrorCode: Some(ErrorCode::ErrPhantomExpItemNotExist),
                    ..Default::default()
                });
            };
            add_consume(&mut consumes, consume.ItemId, consume.Count);
            let Some(mul_exp) = exp_item.Exp.checked_mul(consume.Count) else {
                return Ok(PhantomLevelUpResponse {
                    ErrorCode: Some(ErrorCode::ErrPhantomConsumeItemCount),
                    ..Default::default()
                });
            };
            let Some(next_item) = item_exp_total.checked_add(mul_exp) else {
                return Ok(PhantomLevelUpResponse {
                    ErrorCode: Some(ErrorCode::ErrPhantomConsumeItemCount),
                    ..Default::default()
                });
            };
            item_exp_total = next_item;
        }
    }

    let Some(exp_total) = item_exp_total.checked_add(fodder_exp_total) else {
        return Ok(PhantomLevelUpResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomConsumeItemCount),
            ..Default::default()
        });
    };
    if exp_total == 0 && slot_count == 0 {
        return Ok(PhantomLevelUpResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomConsumeNoExp),
            ..Default::default()
        });
    }

    let mut next_level = item_level;
    let mut next_exp = item_exp;
    let mut excess_exp = 0i32;
    if exp_total > 0 {
        let mut total = i64::from(item_exp) + i64::from(exp_total);
        while next_level < highest_level {
            let lvl_up = next_level + 1;
            let Some(req_exp) =
                shared::phantom_level_exp(&state.assets, item_config.LevelUpGroupId, lvl_up)
            else {
                break;
            };
            if total < i64::from(req_exp) {
                break;
            }
            next_level = lvl_up;
            total -= i64::from(req_exp);
        }
        if next_level >= highest_level {
            next_exp = 0;
            excess_exp = total.min(i64::from(i32::MAX)) as i32;
        } else {
            next_exp = total as i32;
        }
    }

    if slot_count > 0 {
        let Some(predicted_unlocked) = echo_info::unlocked_sub_prop_count(
            &state.assets,
            item_config.QualityId,
            next_level,
        ) else {
            return Ok(PhantomLevelUpResponse {
                ErrorCode: Some(ErrorCode::ErrPhantomSubPropNotEnough),
                ..Default::default()
            });
        };
        if item_sub_prop_len + (slot_count as usize) > predicted_unlocked {
            return Ok(PhantomLevelUpResponse {
                ErrorCode: Some(ErrorCode::ErrPhantomSubPropNotEnough),
                ..Default::default()
            });
        }
        let Some(tc) = shared::tuner_cost(&state.assets, item_config.QualityId) else {
            return Ok(PhantomLevelUpResponse {
                ErrorCode: Some(ErrorCode::ErrPhantomQaulityNotExist),
                ..Default::default()
            });
        };
        let Some(tuner_amt) = slot_count.checked_mul(tc.count) else {
            return Ok(PhantomLevelUpResponse {
                ErrorCode: Some(ErrorCode::ErrPhantomConsumeItemCount),
                ..Default::default()
            });
        };
        add_consume(&mut consumes, tc.item_id, tuner_amt);
        if quality.IdentifyCoin > 0 {
            let Some(coin_amt) = slot_count.checked_mul(quality.IdentifyCoin) else {
                return Ok(PhantomLevelUpResponse {
                    ErrorCode: Some(ErrorCode::ErrPhantomConsumeItemCount),
                    ..Default::default()
                });
            };
            add_consume(&mut consumes, 2, coin_amt);
        }
    }

    if item_exp_total > 0 {
        let exp_coin = ((i64::from(item_exp_total) * 10) / 100).max(1) as i32;
        add_consume(&mut consumes, 2, exp_coin);
    }

    let mut cost_items = Vec::new();
    consume::append_int_map(&mut cost_items, &consumes);
    let paid = {
        let fs = &state.fs;
        let conn = &mut state.conn;
        let basic_comp = &mut state.player_components.basic;
        let inventory_comp = &mut state.player_components.inventory;
        consume::consume_costs(conn, fs, basic_comp, inventory_comp, &cost_items)
            .map_err(|e| e.to_string())?
    };
    if !paid {
        return Ok(PhantomLevelUpResponse {
            ErrorCode: Some(ErrorCode::ErrItemNotEnough),
            ..Default::default()
        });
    }

    if !fodder_consumes.is_empty() {
        let mut removed_ids = Vec::new();
        let mut updated_fodders = Vec::new();
        for (inc_id, cnt) in fodder_consumes {
            let mut decremented = false;
            if let Some(fodder) = state.player_components.echo.echo_map.get_mut(&inc_id)
                && fodder.stack_count > cnt
            {
                fodder.stack_count -= cnt;
                updated_fodders.push(fodder.to_proto(inc_id));
                decremented = true;
            }
            if !decremented {
                state.player_components.echo.echo_map.remove(&inc_id);
                PlayerEchoComponent::delete_echo_file(&state.fs, state.player_id, inc_id)
                    .map_err(|e| e.to_string())?;
                removed_ids.push(inc_id);
            }
        }
        if !removed_ids.is_empty() {
            state.conn.push(
                PhantomItemRemoveNotify::MSG_ID,
                &PhantomItemRemoveNotify {
                    PhantomItemIncrIdList: removed_ids,
                },
            );
        }
        if !updated_fodders.is_empty() {
            state.conn.push(
                PhantomItemUpdateNotify::MSG_ID,
                &PhantomItemUpdateNotify {
                    UpdateInfo: updated_fodders,
                },
            );
        }
    }

    let stack_split =
        shared::split_single_from_stack(&state.fs, &mut state.player_components.echo, req.IncId)?;
    if !state.player_components.echo.echo_map.contains_key(&req.IncId) {
        return Ok(PhantomLevelUpResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomItemNotExist),
            ..Default::default()
        });
    }

    if !refund_map.is_empty() {
        let refunds: Vec<reward::RewardItem> = refund_map
            .into_iter()
            .map(|(item_id, count)| reward::RewardItem { item_id, count })
            .collect();
        let mut rctx = state.reward_context();
        reward::grant_items(&mut rctx, 0, false, &refunds, 0).map_err(|e| e.to_string())?;
    }

    let seed = if slot_count > 0 { random_seed()? } else { 0 };
    let update_proto = {
        let target_item = state
            .player_components
            .echo
            .echo_map
            .get_mut(&req.IncId)
            .unwrap();
        if exp_total > 0 {
            target_item.level = next_level;
            target_item.exp = next_exp;
            target_item.refresh_main_prop_values(&state.assets);
        }
        if slot_count > 0
            && let Some(unlocked_sub_props) = echo_info::unlocked_sub_prop_count(
                &state.assets,
                item_config.QualityId,
                target_item.level,
            )
        {
            target_item.append_sub_props(
                &state.assets,
                item_config.QualityId,
                unlocked_sub_props,
                slot_count as usize,
                seed,
            );
        }
        target_item.to_proto(req.IncId)
    };

    PlayerEchoComponent::save_all(
        &state.fs,
        state.player_id,
        &state.player_components.echo.echo_map,
    )
    .map_err(|e| e.to_string())?;

    let changed_roles = shared::roles_equipping_echo(&state.player_components.echo, req.IncId);
    if !changed_roles.is_empty() {
        shared::push_role_prop_update(state, &changed_roles)?;
        shared::refresh_role_entities(state, &changed_roles)?;
    }

    let mut overflow_item_map = BTreeMap::new();
    if excess_exp > 0 {
        let mut rem = excess_exp;
        let echo_pots = [
            (36000004, 5000),
            (36000003, 2000),
            (36000002, 1000),
            (36000001, 500),
        ];
        let mut exp_refunds = Vec::new();
        for (id, exp) in echo_pots {
            if rem >= exp {
                let cnt = rem / exp;
                if cnt > 0 {
                    rem -= cnt * exp;
                    exp_refunds.push(reward::RewardItem {
                        item_id: id,
                        count: cnt,
                    });
                    overflow_item_map.insert(id, cnt);
                }
            }
        }
        if !exp_refunds.is_empty() {
            let mut rctx = state.reward_context();
            reward::grant_items(&mut rctx, 0, false, &exp_refunds, 0)
                .map_err(|e| e.to_string())?;
        }
    }

    {
        let mut qctx = state.quest_context(events);
        quest_engine::on_condition_state_changed(&mut qctx).map_err(|e| e.to_string())?;
    }

    Ok(PhantomLevelUpResponse {
        ErrorCode: Some(ErrorCode::Success),
        UpdateInfo: Some(update_proto),
        ItemMap: overflow_item_map,
        StackSplitInfo: stack_split,
    })
}

pub fn handle_phantom_identify_request(
    state: &mut State,
    events: &mut EventQueue,
    req: PhantomIdentifyRequest,
) -> Result<PhantomIdentifyResponse, String> {
    if req.Count <= 0 {
        return Ok(PhantomIdentifyResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomConsumeItemCount),
            ..Default::default()
        });
    }
    let Some((item_id, item_level, sub_prop_len)) = state
        .player_components
        .echo
        .echo_map
        .get(&req.IncrId)
        .map(|e| (e.id, e.level, e.sub_prop.len()))
    else {
        return Ok(PhantomIdentifyResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomItemNotExist),
            ..Default::default()
        });
    };
    let Some(item_config) = state
        .assets
        .tables
        .phantom_item
        .get_data_by_id(&item_id)
        .map(|r| r.into_owned())
    else {
        return Ok(PhantomIdentifyResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomConfigNotFound),
            ..Default::default()
        });
    };
    let Some(quality) = state
        .assets
        .tables
        .phantom_quality
        .get_data_by_id(&item_config.QualityId)
        .map(|r| r.into_owned())
    else {
        return Ok(PhantomIdentifyResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomQaulityNotExist),
            ..Default::default()
        });
    };
    let Some(max_sub_props) =
        echo_info::max_unlocked_sub_prop_count(&state.assets, item_config.QualityId)
    else {
        return Ok(PhantomIdentifyResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomSubPropNotEnough),
            ..Default::default()
        });
    };
    if sub_prop_len >= max_sub_props || sub_prop_len + (req.Count as usize) > max_sub_props {
        return Ok(PhantomIdentifyResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomSubPropNotEnough),
            ..Default::default()
        });
    }
    let Some(unlocked_sub_props) =
        echo_info::unlocked_sub_prop_count(&state.assets, item_config.QualityId, item_level)
    else {
        return Ok(PhantomIdentifyResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomSubPropNotEnough),
            ..Default::default()
        });
    };
    if sub_prop_len + (req.Count as usize) > unlocked_sub_props {
        return Ok(PhantomIdentifyResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomSubPropNotEnough),
            ..Default::default()
        });
    }
    let Some(tc) = shared::tuner_cost(&state.assets, item_config.QualityId) else {
        return Ok(PhantomIdentifyResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomQaulityNotExist),
            ..Default::default()
        });
    };

    let cost_list = [
        consume::CostItem {
            id: tc.item_id,
            count: tc.count * req.Count,
        },
        consume::CostItem {
            id: 2,
            count: quality.IdentifyCoin * req.Count,
        },
    ];
    let cost_slice = if quality.IdentifyCoin > 0 {
        &cost_list[..2]
    } else {
        &cost_list[..1]
    };
    if !consume::has_costs(
        &state.player_components.basic,
        &state.player_components.inventory,
        cost_slice,
    ) {
        return Ok(PhantomIdentifyResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomIdentifyNoCost),
            ..Default::default()
        });
    }

    let seed = random_seed()?;
    let (added, update_proto) = {
        let item = state
            .player_components
            .echo
            .echo_map
            .get_mut(&req.IncrId)
            .unwrap();
        let added = item.append_sub_props(
            &state.assets,
            item_config.QualityId,
            unlocked_sub_props,
            req.Count as usize,
            seed,
        );
        (added, item.to_proto(req.IncrId))
    };
    if added == 0 {
        return Ok(PhantomIdentifyResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomSubPropRandomErr),
            ..Default::default()
        });
    }

    {
        let fs = &state.fs;
        let conn = &mut state.conn;
        let basic_comp = &mut state.player_components.basic;
        let inventory_comp = &mut state.player_components.inventory;
        let _ = consume::consume_costs(conn, fs, basic_comp, inventory_comp, cost_slice)
            .map_err(|e| e.to_string())?;
    }
    PlayerEchoComponent::save_all(
        &state.fs,
        state.player_id,
        &state.player_components.echo.echo_map,
    )
    .map_err(|e| e.to_string())?;

    let changed_roles = shared::roles_equipping_echo(&state.player_components.echo, req.IncrId);
    if !changed_roles.is_empty() {
        shared::push_role_prop_update(state, &changed_roles)?;
        shared::refresh_role_entities(state, &changed_roles)?;
    }

    {
        let mut qctx = state.quest_context(events);
        if let Err(err) = quest_engine::on_vision_system_event(&mut qctx, "DoVisionIdentify")
        {
            tracing::warn!("on_vision_system_event(DoVisionIdentify) error: {err}");
        }
    }

    Ok(PhantomIdentifyResponse {
        ErrorCode: Some(ErrorCode::Success),
        UpdateInfo: Some(update_proto),
    })
}

pub fn handle_phantom_polish_request(
    state: &mut State,
    req: PhantomPolishRequest,
) -> Result<PhantomPolishResponse, String> {
    let Some(item_snap) = state.player_components.echo.echo_map.get(&req.IncrId).cloned() else {
        return Ok(PhantomPolishResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomItemNotExist),
            ..Default::default()
        });
    };
    let Some(item_config) = state
        .assets
        .tables
        .phantom_item
        .get_data_by_id(&item_snap.id)
        .map(|r| r.into_owned())
    else {
        return Ok(PhantomPolishResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomConfigNotFound),
            ..Default::default()
        });
    };
    let Some(rarity_cfg) = state
        .assets
        .tables
        .phantom_rarity
        .get_data_by_id(&item_config.Rarity)
        .map(|r| r.into_owned())
    else {
        return Ok(PhantomPolishResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomConfigNotFound),
            ..Default::default()
        });
    };
    if let Some(err) = validate_main_prop_polish(
        &state.assets,
        &item_snap,
        &item_config,
        req.PhantomMainPropItemId,
    ) {
        return Ok(PhantomPolishResponse {
            ErrorCode: Some(err),
            ..Default::default()
        });
    }

    let mut costs = Vec::new();
    consume::append_int_map(&mut costs, &rarity_cfg.PolishCost);
    let paid = {
        let fs = &state.fs;
        let conn = &mut state.conn;
        let basic_comp = &mut state.player_components.basic;
        let inventory_comp = &mut state.player_components.inventory;
        consume::consume_costs(conn, fs, basic_comp, inventory_comp, &costs)
            .map_err(|e| e.to_string())?
    };
    if !paid {
        return Ok(PhantomPolishResponse {
            ErrorCode: Some(ErrorCode::ErrItemNotEnough),
            ..Default::default()
        });
    }

    let stack_split =
        shared::split_single_from_stack(&state.fs, &mut state.player_components.echo, req.IncrId)?;
    let Some(target_item) = state.player_components.echo.echo_map.get_mut(&req.IncrId) else {
        return Ok(PhantomPolishResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomItemNotExist),
            ..Default::default()
        });
    };
    if let Some(err) = apply_main_prop_polish(
        &state.assets,
        target_item,
        &item_config,
        req.PhantomMainPropItemId,
    ) {
        return Ok(PhantomPolishResponse {
            ErrorCode: Some(err),
            ..Default::default()
        });
    }
    let update_proto = target_item.to_proto(req.IncrId);

    PlayerEchoComponent::save_all(
        &state.fs,
        state.player_id,
        &state.player_components.echo.echo_map,
    )
    .map_err(|e| e.to_string())?;

    let changed_roles = shared::roles_equipping_echo(&state.player_components.echo, req.IncrId);
    if !changed_roles.is_empty() {
        shared::push_role_prop_update(state, &changed_roles)?;
        shared::refresh_role_entities(state, &changed_roles)?;
    }

    Ok(PhantomPolishResponse {
        ErrorCode: Some(ErrorCode::Success),
        UpdateInfo: Some(update_proto),
        StackSplitInfo: stack_split,
        StackMergeInfos: None,
    })
}

pub fn handle_phantom_batch_polish_request(
    state: &mut State,
    req: PhantomBatchPolishRequest,
) -> Result<PhantomBatchPolishResponse, String> {
    if req.StackEntries.is_empty() || req.StackEntries.len() > 10 {
        return Ok(PhantomBatchPolishResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomBatchPolishCount),
            ..Default::default()
        });
    }

    let mut seen = HashSet::new();
    let mut costs = Vec::new();
    for stack in &req.StackEntries {
        let inc_id = stack.IncrId;
        if !seen.insert(inc_id) {
            return Ok(PhantomBatchPolishResponse {
                ErrorCode: Some(ErrorCode::ErrPhantomBatchPolishDuplicate),
                ..Default::default()
            });
        }
        let Some(item) = state.player_components.echo.echo_map.get(&inc_id) else {
            return Ok(PhantomBatchPolishResponse {
                ErrorCode: Some(ErrorCode::ErrPhantomItemNotExist),
                ..Default::default()
            });
        };
        let Some(item_config) = state.assets.tables.phantom_item.get_data_by_id(&item.id) else {
            return Ok(PhantomBatchPolishResponse {
                ErrorCode: Some(ErrorCode::ErrPhantomConfigNotFound),
                ..Default::default()
            });
        };
        if item_config.QualityId < 5 {
            return Ok(PhantomBatchPolishResponse {
                ErrorCode: Some(ErrorCode::ErrPhantomBatchPolishNoValidItem),
                ..Default::default()
            });
        }
        let Some(rarity_cfg) = state
            .assets
            .tables
            .phantom_rarity
            .get_data_by_id(&item_config.Rarity)
        else {
            return Ok(PhantomBatchPolishResponse {
                ErrorCode: Some(ErrorCode::ErrPhantomConfigNotFound),
                ..Default::default()
            });
        };
        if let Some(err) = validate_main_prop_polish(
            &state.assets,
            item,
            &item_config,
            req.PhantomMainPropItemId,
        ) {
            return Ok(PhantomBatchPolishResponse {
                ErrorCode: Some(err),
                ..Default::default()
            });
        }
        consume::append_int_map(&mut costs, &rarity_cfg.PolishCost);
    }

    let paid = {
        let fs = &state.fs;
        let conn = &mut state.conn;
        let basic_comp = &mut state.player_components.basic;
        let inventory_comp = &mut state.player_components.inventory;
        consume::consume_costs(conn, fs, basic_comp, inventory_comp, &costs)
            .map_err(|e| e.to_string())?
    };
    if !paid {
        return Ok(PhantomBatchPolishResponse {
            ErrorCode: Some(ErrorCode::ErrItemNotEnough),
            ..Default::default()
        });
    }

    let mut changed_roles = Vec::new();
    let mut update_infos = Vec::with_capacity(req.StackEntries.len());
    let mut split_infos = Vec::new();

    for stack in &req.StackEntries {
        let inc_id = stack.IncrId;
        if let Some(split) =
            shared::split_single_from_stack(&state.fs, &mut state.player_components.echo, inc_id)?
        {
            split_infos.push(split);
        }
        let Some(item) = state.player_components.echo.echo_map.get_mut(&inc_id) else {
            return Ok(PhantomBatchPolishResponse {
                ErrorCode: Some(ErrorCode::ErrPhantomItemNotExist),
                ..Default::default()
            });
        };
        let Some(item_config) = state
            .assets
            .tables
            .phantom_item
            .get_data_by_id(&item.id)
            .map(|r| r.into_owned())
        else {
            return Ok(PhantomBatchPolishResponse {
                ErrorCode: Some(ErrorCode::ErrPhantomConfigNotFound),
                ..Default::default()
            });
        };
        if let Some(err) =
            apply_main_prop_polish(&state.assets, item, &item_config, req.PhantomMainPropItemId)
        {
            return Ok(PhantomBatchPolishResponse {
                ErrorCode: Some(err),
                ..Default::default()
            });
        }
        update_infos.push(item.to_proto(inc_id));
        shared::append_roles_equipping_echo(&state.player_components.echo, inc_id, &mut changed_roles);
    }

    PlayerEchoComponent::save_all(
        &state.fs,
        state.player_id,
        &state.player_components.echo.echo_map,
    )
    .map_err(|e| e.to_string())?;

    if !changed_roles.is_empty() {
        shared::push_role_prop_update(state, &changed_roles)?;
        shared::refresh_role_entities(state, &changed_roles)?;
    }

    Ok(PhantomBatchPolishResponse {
        ErrorCode: Some(ErrorCode::Success),
        UpdateInfos: update_infos,
        StackSplitInfos: split_infos,
        StackMergeInfos: Vec::new(),
    })
}

pub fn handle_phantom_vice_polish_request(
    state: &mut State,
    req: PhantomVicePolishRequest,
) -> Result<PhantomVicePolishResponse, String> {
    if calabash::current_level(
        &state.assets,
        &state.player_components.basic,
        &state.player_components.echo,
    ) < 21
    {
        return Ok(PhantomVicePolishResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomVicePolishLimit),
            ..Default::default()
        });
    }
    let Some(item_snap) = state.player_components.echo.echo_map.get(&req.IncrId).cloned() else {
        return Ok(PhantomVicePolishResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomItemNotExist),
            ..Default::default()
        });
    };
    let Some(item_config) = state
        .assets
        .tables
        .phantom_item
        .get_data_by_id(&item_snap.id)
        .map(|r| r.into_owned())
    else {
        return Ok(PhantomVicePolishResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomConfigNotFound),
            ..Default::default()
        });
    };
    let Some(max_sub_props) =
        echo_info::max_unlocked_sub_prop_count(&state.assets, item_config.QualityId)
    else {
        return Ok(PhantomVicePolishResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomVicePolishItemPropLimit),
            ..Default::default()
        });
    };
    if max_sub_props == 0 {
        return Ok(PhantomVicePolishResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomVicePolishItemPropLimit),
            ..Default::default()
        });
    }
    if item_snap.sub_prop.is_empty() {
        return Ok(PhantomVicePolishResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomVicePolishNoneProp),
            ..Default::default()
        });
    }
    let Some(unlocked_sub_props) =
        echo_info::unlocked_sub_prop_count(&state.assets, item_config.QualityId, item_snap.level)
    else {
        return Ok(PhantomVicePolishResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomVicePolishItemLevelLimit),
            ..Default::default()
        });
    };
    if unlocked_sub_props < max_sub_props {
        return Ok(PhantomVicePolishResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomVicePolishItemLevelLimit),
            ..Default::default()
        });
    }
    if item_snap.sub_prop.len() < max_sub_props {
        return Ok(PhantomVicePolishResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomVicePolishItemPropLimit),
            ..Default::default()
        });
    }

    let Ok(lock_indices) =
        validated_lock_prop_indexes(item_snap.sub_prop.len(), &req.LockPropIndex)
    else {
        return Ok(PhantomVicePolishResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomVicePolishParam),
            ..Default::default()
        });
    };
    let Some(vice_cfg) = state
        .assets
        .tables
        .phantom_vice_polish_config
        .get_data_by_id(&(lock_indices.len() as i32))
        .map(|r| r.into_owned())
    else {
        return Ok(PhantomVicePolishResponse {
            ErrorCode: Some(ErrorCode::ErrPhantomVicePolishLimit),
            ..Default::default()
        });
    };

    let mut costs = Vec::new();
    consume::append_int_map(&mut costs, &vice_cfg.Cost);
    let paid = {
        let fs = &state.fs;
        let conn = &mut state.conn;
        let basic_comp = &mut state.player_components.basic;
        let inventory_comp = &mut state.player_components.inventory;
        consume::consume_costs(conn, fs, basic_comp, inventory_comp, &costs)
            .map_err(|e| e.to_string())?
    };
    if !paid {
        return Ok(PhantomVicePolishResponse {
            ErrorCode: Some(ErrorCode::ErrItemNotEnough),
            ..Default::default()
        });
    }

    let new_props = build_vice_polish_props(
        &state.assets,
        &item_snap,
        &item_config,
        &lock_indices,
        random_seed()?,
    );
    let sub_prop_protos = prop_list(&new_props);
    {
        let item = state
            .player_components
            .echo
            .echo_map
            .get_mut(&req.IncrId)
            .unwrap();
        item.unack_sub_prop = new_props;
        item.lock_prop_index = req.LockPropIndex;
    }

    PlayerEchoComponent::save_all(
        &state.fs,
        state.player_id,
        &state.player_components.echo.echo_map,
    )
    .map_err(|e| e.to_string())?;

    Ok(PhantomVicePolishResponse {
        ErrorCode: Some(ErrorCode::Success),
        PhantomSubProp: sub_prop_protos,
    })
}

pub fn handle_phantom_vice_polish_ack_request(
    state: &mut State,
    req: PhantomVicePolishAckRequest,
) -> Result<PhantomVicePolishAckResponse, String> {
    let update_proto = {
        let Some(item) = state.player_components.echo.echo_map.get_mut(&req.IncrId) else {
            return Ok(PhantomVicePolishAckResponse {
                ErrorCode: Some(ErrorCode::ErrPhantomItemNotExist),
                ..Default::default()
            });
        };
        if item.unack_sub_prop.is_empty() {
            return Ok(PhantomVicePolishAckResponse {
                ErrorCode: Some(ErrorCode::ErrPhantomVicePolishNoAck),
                ..Default::default()
            });
        }
        if req.Ack {
            item.sub_prop = std::mem::take(&mut item.unack_sub_prop);
        } else {
            item.unack_sub_prop.clear();
        }
        item.lock_prop_index.clear();
        item.to_proto(req.IncrId)
    };

    PlayerEchoComponent::save_all(
        &state.fs,
        state.player_id,
        &state.player_components.echo.echo_map,
    )
    .map_err(|e| e.to_string())?;

    if req.Ack {
        let changed_roles = shared::roles_equipping_echo(&state.player_components.echo, req.IncrId);
        if !changed_roles.is_empty() {
            shared::push_role_prop_update(state, &changed_roles)?;
            shared::refresh_role_entities(state, &changed_roles)?;
        }
    }

    Ok(PhantomVicePolishAckResponse {
        ErrorCode: Some(ErrorCode::Success),
        UpdateInfo: Some(update_proto),
    })
}

fn validate_main_prop_polish(
    assets: &Assets,
    item: &EchoInfo,
    item_config: &PhantomItemConfig,
    prop_item_id: i32,
) -> Option<ErrorCode> {
    find_main_property_with_prop(assets, item_config.MainProp.RandGroupId, prop_item_id)
        .ok_or(ErrorCode::ErrPhantomBatchPolishPropNotMatch)
        .err()
        .or_else(|| {
            if item.main_prop.iter().any(|p| p.id == prop_item_id) {
                Some(ErrorCode::PhantomPolishSamePro)
            } else {
                None
            }
        })
}

fn apply_main_prop_polish(
    assets: &Assets,
    item: &mut EchoInfo,
    item_config: &PhantomItemConfig,
    prop_item_id: i32,
) -> Option<ErrorCode> {
    let Some(main_property) =
        find_main_property_with_prop(assets, item_config.MainProp.RandGroupId, prop_item_id)
    else {
        return Some(ErrorCode::ErrPhantomBatchPolishPropNotMatch);
    };
    if item.main_prop.iter().any(|p| p.id == prop_item_id) {
        return Some(ErrorCode::PhantomPolishSamePro);
    }

    item.main_prop = echo_info::build_main_props_from_group(
        assets,
        &main_property.PropGroup,
        item.level,
    );
    item.unack_sub_prop.clear();
    item.lock_prop_index.clear();
    None
}

fn find_main_property_with_prop(
    assets: &Assets,
    rand_group_id: i32,
    prop_item_id: i32,
) -> Option<&PhantomMainProperty> {
    assets
        .tables
        .phantom_main_property
        .items
        .iter()
        .find(|e| e.RandGroupId == rand_group_id && e.PropGroup.contains(&prop_item_id))
}

fn validated_lock_prop_indexes(prop_count: usize, indexes: &[i32]) -> Result<HashSet<usize>, ()> {
    let mut lock_indices = HashSet::with_capacity(indexes.len());
    for &index in indexes {
        if index < 0 || (index as usize) >= prop_count {
            return Err(());
        }
        if !lock_indices.insert(index as usize) {
            return Err(());
        }
    }
    Ok(lock_indices)
}

fn build_vice_polish_props(
    assets: &Assets,
    item: &EchoInfo,
    item_config: &PhantomItemConfig,
    lock_indices: &HashSet<usize>,
    seed: u64,
) -> Vec<Prop> {
    let mut rng = ZigDefaultPrng::init(seed);
    let mut props = Vec::with_capacity(item.sub_prop.len());
    let mut used_props: Vec<Prop> = item
        .sub_prop
        .iter()
        .enumerate()
        .filter(|(idx, _)| lock_indices.contains(idx))
        .map(|(_, p)| p.clone())
        .collect();

    for (index, old_prop) in item.sub_prop.iter().enumerate() {
        if lock_indices.contains(&index) {
            props.push(old_prop.clone());
        } else {
            let replacement = if let Some(prop) =
                echo_info::pick_random_sub_property(assets, &used_props, &mut rng)
            {
                Prop {
                    id: prop.Id,
                    value: echo_info::random_sub_prop_value(item_config.QualityId, &prop, &mut rng),
                }
            } else {
                old_prop.clone()
            };
            props.push(replacement.clone());
            used_props.push(replacement);
        }
    }
    props
}

fn prop_list(data: &[Prop]) -> Vec<PhantomPropInfo> {
    data.iter()
        .map(|prop| PhantomPropInfo {
            PhantomPropId: prop.id,
            Value: prop.value,
        })
        .collect()
}
