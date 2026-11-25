use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

use sonoro_gamesv_data::Assets;
use sonoro_gamesv_data::data_tables::create_buff_information;
use sonoro_gamesv_data::tables::weapon_breach::WeaponBreach;
use sonoro_gamesv_fs::weapon_item::WeaponItem;
use sonoro_gamesv_logic::component::comp_util;
use sonoro_gamesv_logic::component::player::PlayerWeaponComponent;
use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::events::Event;
use sonoro_gamesv_logic::fsm::types::wake_reason;
use sonoro_gamesv_logic::helpers::{
    buff as buff_helper, condition, consume, quest_engine, reward, role_attribute_sync,
};
use sonoro_gamesv_logic::schedulers::BuffTimerScheduler;
use sonoro_proto::enums::{ApplyGEType, EquipPos, ErrorCode};
use sonoro_proto::pb::{
    ApplyBuffS2cRequestNotify, CombatCommon, CombatNotifyData, CombatNotifyDataMessageOneof,
    CombatReceiveData, CombatReceiveDataMessageOneof, CombatReceivePackNotify,
    EntityEquipChangeNotify, EquipTakeOnNotify, EquipTakeOnRequest, EquipTakeOnResponse,
    RemoveBuffS2cRequestNotify, RoleLoadEquipData, WeaponBreachRequest, WeaponBreachResponse,
    WeaponItemRequest, WeaponItemResponse, WeaponItemRemoveNotify, WeaponLevelUpRequest,
    WeaponLevelUpResponse, WeaponResonUpRequest, WeaponResonUpResponse,
};

use super::State;
use sonoro_gamesv_logic::sink::PushSink;

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_millis() as i64)
}

fn save_weapon_immediate(
    fs: &sonoro_common::fs::FileSystem,
    player_id: i32,
    incr_id: i32,
    weapon: &WeaponItem,
) -> Result<(), String> {
    let path = format!("player/{player_id}/{}/{incr_id}", WeaponItem::DATA_DIR);
    comp_util::save_struct(fs, weapon, &path).map_err(|e| e.to_string())
}

fn find_weapon_breach_entry(
    assets: &Assets,
    breach_id: i32,
    breach_level: i32,
) -> Option<WeaponBreach> {
    assets
        .tables
        .weapon_breach
        .items
        .iter()
        .find(|entry| entry.BreachId == breach_id && entry.Level == breach_level)
        .cloned()
}

fn sync_weapon_passive_buffs(
    state: &mut State,
    _events: &mut EventQueue,
    role_id: i32,
    remove_ids: &[i64],
    add_ids: &[i64],
    now_ms_val: i64,
) -> bool {
    let State {
        fs,
        conn,
        assets,
        scene: scene_opt,
        buff_timers,
        ..
    } = state;
    let Some(scene) = scene_opt.as_mut() else {
        return false;
    };
    let Some(idx) = scene
        .entities
        .iter()
        .position(|s| s.config.config_id == role_id && s.buffs.is_some())
    else {
        return false;
    };
    let net_id = scene.entities[idx].net_id;
    buff_timers.ensure_entity_registered(scene, assets, net_id, now_ms_val);

    let mut notify_data: Vec<CombatReceiveData> = Vec::new();

    for &buff_id in remove_ids {
        let Some((handle_id, stack_count)) = scene.entities[idx]
            .buffs
            .as_ref()
            .and_then(|b| b.get_by_buff_id(buff_id))
            .map(|b| (b.HandleId, b.StackCount))
        else {
            continue;
        };
        buff_timers.cancel_handle(net_id, handle_id);
        scene.entities[idx]
            .buffs
            .as_mut()
            .expect("buffs checked")
            .remove_by_handle(handle_id);
        notify_data.push(CombatReceiveData {
            Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
                CombatNotifyData {
                    CombatCommon: Some(CombatCommon {
                        EntityId: net_id,
                        ..Default::default()
                    }),
                    Message: Some(CombatNotifyDataMessageOneof::RemoveBuffS2cRequestNotify(
                        RemoveBuffS2cRequestNotify {
                            Handle: handle_id,
                            StackCount: stack_count,
                            Reason: 41,
                            InstigatorId: net_id,
                        },
                    )),
                },
            )),
        });
    }

    for &buff_id in add_ids {
        if scene.entities[idx]
            .buffs
            .as_ref()
            .is_some_and(|b| b.get_by_buff_id(buff_id).is_some())
        {
            continue;
        }
        let Some(buff_data) = assets
            .tables
            .buff
            .get_data_by_id(&buff_id)
            .map(|r| r.into_owned())
        else {
            continue;
        };
        scene.instance.buff_handle += 1;
        let buff_info = create_buff_information(
            scene.instance.buff_handle,
            buff_id,
            net_id,
            net_id,
            true,
        );
        scene.entities[idx]
            .buffs
            .as_mut()
            .expect("buffs checked")
            .buffs
            .push(buff_info.clone());
        buff_timers.schedule_new_buff(&buff_info, &buff_data, net_id, now_ms_val);
        notify_data.push(CombatReceiveData {
            Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
                CombatNotifyData {
                    CombatCommon: Some(CombatCommon {
                        EntityId: net_id,
                        MessageId: -1,
                        PreMessageId: -1,
                        ..Default::default()
                    }),
                    Message: Some(CombatNotifyDataMessageOneof::ApplyBuffS2cRequestNotify(
                        ApplyBuffS2cRequestNotify {
                            Id: buff_id,
                            Level: buff_info.Level,
                            InstigatorId: net_id,
                            ApplyType: Some(buff_info.ApplyType.unwrap_or(ApplyGEType::Common)),
                            ServerId: buff_info.ServerId,
                            StackCount: buff_info.StackCount,
                            IsIterable: true,
                            Reason: 12,
                            Time: None,
                        },
                    )),
                },
            )),
        });

        let disposition =
            BuffTimerScheduler::application_disposition(&buff_data, true, false, true);
        if disposition.execute_periodic_now {
            let _ = scene.save_entity_by_net_id(fs, net_id);
            buff_helper::execute_periodic_buff_effects(
                &mut notify_data,
                net_id,
                net_id,
                &buff_data,
                scene,
            );
        }
    }

    if notify_data.is_empty() {
        return false;
    }
    conn.push(
        CombatReceivePackNotify::MSG_ID,
        &CombatReceivePackNotify { Data: notify_data },
    );
    scene.mark_fsm_dirty(net_id, wake_reason::BUFF);
    true
}

pub fn handle_weapon_item_request(
    state: &mut State,
    _req: WeaponItemRequest,
) -> Result<WeaponItemResponse, String> {
    let list = state
        .player_components
        .weapon
        .weapon_map
        .iter()
        .map(|(&incr_id, w)| w.to_proto(incr_id))
        .collect();
    Ok(WeaponItemResponse {
        WeaponItemList: list,
    })
}

fn weapon_level_exp(assets: &Assets, level_id: i32, level: i32) -> i32 {
    assets
        .tables
        .weapon_level
        .items
        .iter()
        .find(|entry| entry.LevelId == level_id && entry.Level == level)
        .map_or(0, |entry| entry.Exp)
}

fn weapon_level_limit(assets: &Assets, breach_id: i32, breach: i32) -> i32 {
    find_weapon_breach_entry(assets, breach_id, breach).map_or(90, |entry| entry.LevelLimit)
}

fn weapon_exp_item_value(assets: &Assets, item_id: i32) -> i32 {
    assets
        .tables
        .weapon_exp_item
        .get_data_by_id(&item_id)
        .map_or(0, |item| item.BasicExp)
}

fn weapon_quality_base_exp(quality: i32) -> i32 {
    match quality {
        1 => 100,
        2 => 800,
        3 => 1800,
        4 => 21000,
        5 => 90000,
        _ => 800,
    }
}

fn total_weapon_invested_exp(assets: &Assets, level_id: i32, weapon: &WeaponItem) -> i64 {
    let mut total = i64::from(weapon.exp.max(0));
    for lv in 1..weapon.level {
        total += i64::from(weapon_level_exp(assets, level_id, lv).max(0));
    }
    total
}

pub fn handle_weapon_level_up_request(
    state: &mut State,
    events: &mut EventQueue,
    req: WeaponLevelUpRequest,
) -> Result<WeaponLevelUpResponse, String> {
    let Some(weapon_snapshot) = state
        .player_components
        .weapon
        .weapon_map
        .get(&req.IncId)
        .cloned()
    else {
        return Ok(WeaponLevelUpResponse {
            ErrorCode: Some(ErrorCode::ErrWeaponConsumeItemNotFound),
            ..Default::default()
        });
    };
    let Some(config) = state
        .assets
        .tables
        .weapon_conf
        .get_data_by_id(&weapon_snapshot.id)
        .map(|r| r.into_owned())
    else {
        return Ok(WeaponLevelUpResponse {
            ErrorCode: Some(ErrorCode::ErrWeaponResonConfigNotFound),
            ..Default::default()
        });
    };

    let limit = weapon_level_limit(&state.assets, config.BreachId, weapon_snapshot.breach);
    if weapon_snapshot.level >= limit {
        return Ok(WeaponLevelUpResponse {
            ErrorCode: Some(ErrorCode::ErrWeaponLevelLimit),
            IncId: req.IncId,
            WeaponLevel: weapon_snapshot.level,
            WeaponExp: weapon_snapshot.exp,
            ..Default::default()
        });
    }

    let err_invalid = || WeaponLevelUpResponse {
        ErrorCode: Some(ErrorCode::ErrWeaponConsumeInvalid),
        IncId: req.IncId,
        WeaponLevel: weapon_snapshot.level,
        WeaponExp: weapon_snapshot.exp,
        ..Default::default()
    };

    let mut gained_exp_i64 = 0i64;
    let mut item_exp_i64 = 0i64;
    let mut costs = Vec::new();
    let mut seen_fodder = HashSet::new();
    let mut fodder_ids = Vec::new();

    for item in &req.ConsumeList {
        if item.IncId > 0 {
            if item.Count != 1 || item.IncId == req.IncId || !seen_fodder.insert(item.IncId) {
                return Ok(err_invalid());
            }
            let Some(fodder) = state.player_components.weapon.weapon_map.get(&item.IncId) else {
                return Ok(WeaponLevelUpResponse {
                    ErrorCode: Some(ErrorCode::ErrWeaponConsumeItemNotFound),
                    IncId: req.IncId,
                    WeaponLevel: weapon_snapshot.level,
                    WeaponExp: weapon_snapshot.exp,
                    ..Default::default()
                });
            };
            if fodder.role_id.is_some() || fodder.func_value == 1 {
                return Ok(err_invalid());
            }
            let Some(fodder_cfg) = state
                .assets
                .tables
                .weapon_conf
                .get_data_by_id(&fodder.id)
                .map(|r| r.into_owned())
            else {
                return Ok(WeaponLevelUpResponse {
                    ErrorCode: Some(ErrorCode::ErrWeaponResonConfigNotFound),
                    ..Default::default()
                });
            };
            let base_exp = i64::from(weapon_quality_base_exp(fodder_cfg.QualityId));
            let invested =
                (total_weapon_invested_exp(&state.assets, fodder_cfg.LevelId, fodder) * 90) / 100;
            gained_exp_i64 += base_exp + invested;
            fodder_ids.push(item.IncId);
        } else {
            if item.ItemId == 0 || item.Count <= 0 {
                continue;
            }
            let exp = weapon_exp_item_value(&state.assets, item.ItemId);
            if exp == 0 {
                continue;
            }
            let Some(add) = i64::from(exp).checked_mul(i64::from(item.Count)) else {
                return Ok(err_invalid());
            };
            item_exp_i64 += add;
            gained_exp_i64 += add;
            costs.push(consume::CostItem {
                id: item.ItemId,
                count: item.Count,
            });
        }
    }

    if gained_exp_i64 <= 0 || gained_exp_i64 > i64::from(i32::MAX) {
        return Ok(WeaponLevelUpResponse {
            ErrorCode: Some(ErrorCode::ErrWeaponLevelUpNoExp),
            IncId: req.IncId,
            WeaponLevel: weapon_snapshot.level,
            WeaponExp: weapon_snapshot.exp,
            ..Default::default()
        });
    }
    let gained_exp = gained_exp_i64 as i32;

    if item_exp_i64 > 0 {
        let coin_cost_i64 = ((item_exp_i64 * 40) / 100).max(1);
        if coin_cost_i64 > i64::from(i32::MAX) {
            return Ok(err_invalid());
        }
        costs.push(consume::CostItem {
            id: 2,
            count: coin_cost_i64 as i32,
        });
    }
    if !costs.is_empty() {
        let ok = {
            let State {
                conn,
                fs,
                player_components,
                ..
            } = state;
            consume::consume_costs(
                conn,
                fs,
                &mut player_components.basic,
                &mut player_components.inventory,
                &costs,
            )
            .map_err(|e| e.to_string())?
        };
        if !ok {
            return Ok(WeaponLevelUpResponse {
                ErrorCode: Some(ErrorCode::ErrItemNotEnough),
                IncId: req.IncId,
                WeaponLevel: weapon_snapshot.level,
                WeaponExp: weapon_snapshot.exp,
                ..Default::default()
            });
        }
    }

    if !fodder_ids.is_empty() {
        for &fid in &fodder_ids {
            state.player_components.weapon.weapon_map.remove(&fid);
            PlayerWeaponComponent::delete_weapon_file(
                &state.fs,
                state.player_components.weapon.player_id,
                fid,
            )
            .map_err(|e| e.to_string())?;
        }
        state.conn.push(
            WeaponItemRemoveNotify::MSG_ID,
            &WeaponItemRemoveNotify {
                WeaponItemIncrIdList: fodder_ids,
            },
        );
    }

    let mut new_level = weapon_snapshot.level;
    let mut total_exp = i64::from(weapon_snapshot.exp) + i64::from(gained_exp);
    while new_level < limit {
        let needed_exp = weapon_level_exp(&state.assets, config.LevelId, new_level);
        if needed_exp <= 0 || total_exp < i64::from(needed_exp) {
            break;
        }
        total_exp -= i64::from(needed_exp);
        new_level += 1;
    }

    let mut item_map = std::collections::BTreeMap::new();
    let new_exp = if new_level >= limit {
        let mut excess = total_exp;
        const REFUND_POTS: [(i32, i64); 4] = [
            (43020004, 20000),
            (43020003, 8000),
            (43020002, 3000),
            (43020001, 1000),
        ];
        let mut refunds = Vec::new();
        for (id, pot_exp) in REFUND_POTS {
            if excess >= pot_exp {
                let cnt = (excess / pot_exp).min(i64::from(i32::MAX)) as i32;
                if cnt > 0 {
                    excess -= i64::from(cnt) * pot_exp;
                    refunds.push(reward::RewardItem { item_id: id, count: cnt });
                    item_map.insert(id, cnt);
                }
            }
        }
        if !refunds.is_empty() {
            let mut rctx = state.reward_context();
            reward::grant_items(&mut rctx, 0, false, &refunds, 0).map_err(|e| e.to_string())?;
        }
        0
    } else {
        total_exp as i32
    };

    let equipped_role = {
        let weapon = state
            .player_components
            .weapon
            .weapon_map
            .get_mut(&req.IncId)
            .expect("checked above");
        weapon.level = new_level;
        weapon.exp = new_exp;
        let w_clone = weapon.clone();
        let pid = state.player_components.weapon.player_id;
        save_weapon_immediate(&state.fs, pid, req.IncId, &w_clone)?;
        w_clone.role_id
    };

    events.enqueue(Event::WeaponInfoModified {
        player_id: state.player_id,
        incr_id: req.IncId,
    });
    if let Some(role_id) = equipped_role {
        let State {
            conn,
            fs,
            assets,
            player_components,
            scene: scene_opt,
            ..
        } = state;
        if let Some(scene) = scene_opt.as_mut() {
            role_attribute_sync::refresh_role(
                conn,
                fs,
                scene,
                assets,
                &mut player_components.role,
                &player_components.weapon,
                &player_components.echo,
                role_id,
            )
            .map_err(|e| e.to_string())?;
        }
    }

    let mut qctx = state.quest_context(events);
    quest_engine::on_condition_state_changed(&mut qctx).map_err(|e| e.to_string())?;
    Ok(WeaponLevelUpResponse {
        ErrorCode: Some(ErrorCode::Success),
        IncId: req.IncId,
        WeaponLevel: new_level,
        WeaponExp: new_exp,
        ItemMap: item_map,
    })
}

pub fn handle_weapon_breach_request(
    state: &mut State,
    events: &mut EventQueue,
    req: WeaponBreachRequest,
) -> Result<WeaponBreachResponse, String> {
    let Some(weapon_snapshot) = state
        .player_components
        .weapon
        .weapon_map
        .get(&req.IncId)
        .cloned()
    else {
        return Ok(WeaponBreachResponse {
            ErrorCode: Some(ErrorCode::ErrWeaponConsumeItemNotFound),
            ..Default::default()
        });
    };
    let Some(config) = state
        .assets
        .tables
        .weapon_conf
        .get_data_by_id(&weapon_snapshot.id)
        .map(|r| r.into_owned())
    else {
        return Ok(WeaponBreachResponse {
            ErrorCode: Some(ErrorCode::ErrWeaponResonConfigNotFound),
            ..Default::default()
        });
    };

    let Some(breach_cfg) =
        find_weapon_breach_entry(&state.assets, config.BreachId, weapon_snapshot.breach)
    else {
        return Ok(WeaponBreachResponse {
            ErrorCode: Some(ErrorCode::ErrWeaponBreachLimit),
            IncId: req.IncId,
            WeaponBreach: weapon_snapshot.breach,
        });
    };
    if find_weapon_breach_entry(&state.assets, config.BreachId, weapon_snapshot.breach + 1)
        .is_none()
    {
        return Ok(WeaponBreachResponse {
            ErrorCode: Some(ErrorCode::ErrWeaponBreachLimit),
            IncId: req.IncId,
            WeaponBreach: weapon_snapshot.breach,
        });
    }
    if weapon_snapshot.level < breach_cfg.LevelLimit {
        return Ok(WeaponBreachResponse {
            ErrorCode: Some(ErrorCode::ErrWeaponLevelLimit),
            IncId: req.IncId,
            WeaponBreach: weapon_snapshot.breach,
        });
    }
    {
        let qctx = state.quest_context(events);
        if !condition::eval_condition_or_group(&qctx.make_eval_context(), breach_cfg.ConditionId) {
            return Ok(WeaponBreachResponse {
                ErrorCode: Some(ErrorCode::ErrWeaponBreachLimit),
                IncId: req.IncId,
                WeaponBreach: weapon_snapshot.breach,
            });
        }
    }

    let mut costs = Vec::new();
    consume::append_int_map(&mut costs, &breach_cfg.Consume);
    if breach_cfg.GoldConsume > 0 {
        costs.push(consume::CostItem {
            id: 2,
            count: breach_cfg.GoldConsume,
        });
    }
    {
        let State {
            conn,
            fs,
            player_components,
            ..
        } = state;
        if !consume::consume_costs(
            conn,
            fs,
            &mut player_components.basic,
            &mut player_components.inventory,
            &costs,
        )
        .map_err(|e| e.to_string())?
        {
            return Ok(WeaponBreachResponse {
                ErrorCode: Some(ErrorCode::ErrItemNotEnough),
                IncId: req.IncId,
                WeaponBreach: weapon_snapshot.breach,
            });
        }
    }

    let new_breach = weapon_snapshot.breach + 1;
    let equipped_role = {
        let weapon = state
            .player_components
            .weapon
            .weapon_map
            .get_mut(&req.IncId)
            .expect("checked above");
        weapon.breach = new_breach;
        let w_clone = weapon.clone();
        save_weapon_immediate(
            &state.fs,
            state.player_components.weapon.player_id,
            req.IncId,
            &w_clone,
        )?;
        w_clone.role_id
    };
    events.enqueue(Event::WeaponInfoModified {
        player_id: state.player_id,
        incr_id: req.IncId,
    });

    if let Some(role_id) = equipped_role {
        let State {
            conn,
            fs,
            assets,
            player_components,
            scene: scene_opt,
            ..
        } = state;
        if let Some(scene) = scene_opt.as_mut() {
            let mut updated_equip = None;
            for storage in &mut scene.entities {
                if storage.config.config_id != role_id {
                    continue;
                }
                if let Some(equip) = storage.equip.as_mut() {
                    equip.weapon_breach_level = new_breach;
                    updated_equip = Some((storage.net_id, equip.to_proto()));
                }
                break;
            }
            if let Some((net_id, pb_equip)) = updated_equip {
                scene
                    .save_entity_by_net_id(fs, net_id)
                    .map_err(|e| e.to_string())?;
                conn.push(
                    EntityEquipChangeNotify::MSG_ID,
                    &EntityEquipChangeNotify {
                        EntityId: net_id,
                        EquipComponent: Some(pb_equip),
                    },
                );
            }
            role_attribute_sync::refresh_role(
                conn,
                fs,
                scene,
                assets,
                &mut player_components.role,
                &player_components.weapon,
                &player_components.echo,
                role_id,
            )
            .map_err(|e| e.to_string())?;
        }
    }

    events.enqueue(Event::UpdateFormations {
        player_id: state.player_id,
        formation_id: 0,
        role_ids: Vec::new(),
    });
    Ok(WeaponBreachResponse {
        ErrorCode: Some(ErrorCode::Success),
        IncId: req.IncId,
        WeaponBreach: new_breach,
    })
}

pub fn handle_weapon_reson_up_request(
    state: &mut State,
    events: &mut EventQueue,
    req: WeaponResonUpRequest,
) -> Result<WeaponResonUpResponse, String> {
    let now_ms_val = now_ms();
    let Some(weapon_snapshot) = state
        .player_components
        .weapon
        .weapon_map
        .get(&req.IncId)
        .cloned()
    else {
        return Ok(WeaponResonUpResponse {
            ErrorCode: Some(ErrorCode::ErrWeaponConsumeItemNotFound),
            ..Default::default()
        });
    };
    let Some(config) = state
        .assets
        .tables
        .weapon_conf
        .get_data_by_id(&weapon_snapshot.id)
        .map(|r| r.into_owned())
    else {
        return Ok(WeaponResonUpResponse {
            ErrorCode: Some(ErrorCode::ErrWeaponResonConfigNotFound),
            ..Default::default()
        });
    };

    if weapon_snapshot.reson_level >= config.ResonLevelLimit {
        return Ok(WeaponResonUpResponse {
            ErrorCode: Some(ErrorCode::ErrWeaponResonLevelLimit),
            IncId: req.IncId,
            ResonLevel: weapon_snapshot.reson_level,
        });
    }

    let old_reson_level = weapon_snapshot.reson_level;
    let max_gain = config.ResonLevelLimit - old_reson_level;
    let mut gained_ranks = 0i32;
    let mut seen_fodder = HashSet::new();
    let mut fodder_ids = Vec::new();
    let mut costs = Vec::new();

    for &consume_id in &req.ConsumeList {
        if gained_ranks >= max_gain {
            break;
        }
        if consume_id == req.IncId || !seen_fodder.insert(consume_id) {
            continue;
        }
        let Some(consumed) = state.player_components.weapon.weapon_map.get(&consume_id) else {
            continue;
        };
        if consumed.id != weapon_snapshot.id
            || consumed.role_id.is_some()
            || consumed.func_value != 0
        {
            continue;
        }
        fodder_ids.push(consume_id);
        if let Some(rcfg) = state
            .assets
            .tables
            .get_weapon_reson(config.ResonId, old_reson_level + gained_ranks)
            && rcfg.GoldConsume > 0
        {
            costs.push(consume::CostItem {
                id: 2,
                count: rcfg.GoldConsume,
            });
        }
        gained_ranks += 1;
    }

    for item in &req.ConsumeItemList {
        if gained_ranks >= max_gain {
            break;
        }
        if item.IncId > 0 {
            let consume_id = item.IncId;
            if consume_id == req.IncId || !seen_fodder.insert(consume_id) {
                continue;
            }
            let Some(consumed) = state.player_components.weapon.weapon_map.get(&consume_id) else {
                continue;
            };
            if consumed.id != weapon_snapshot.id
                || consumed.role_id.is_some()
                || consumed.func_value != 0
            {
                continue;
            }
            fodder_ids.push(consume_id);
            if let Some(rcfg) = state
                .assets
                .tables
                .get_weapon_reson(config.ResonId, old_reson_level + gained_ranks)
                && rcfg.GoldConsume > 0
            {
                costs.push(consume::CostItem {
                    id: 2,
                    count: rcfg.GoldConsume,
                });
            }
            gained_ranks += 1;
        } else if item.ItemId > 0 && item.Count > 0 {
            let mut remaining_mat = item.Count;
            while remaining_mat > 0 && gained_ranks < max_gain {
                let Some(rcfg) = state
                    .assets
                    .tables
                    .get_weapon_reson(config.ResonId, old_reson_level + gained_ranks)
                else {
                    break;
                };
                if !rcfg.AlternativeConsume.contains(&item.ItemId) {
                    break;
                }
                let need_per_rank = rcfg.Consume.max(1);
                if remaining_mat < need_per_rank {
                    break;
                }
                remaining_mat -= need_per_rank;
                costs.push(consume::CostItem {
                    id: item.ItemId,
                    count: need_per_rank,
                });
                if rcfg.GoldConsume > 0 {
                    costs.push(consume::CostItem {
                        id: 2,
                        count: rcfg.GoldConsume,
                    });
                }
                gained_ranks += 1;
            }
        }
    }

    if gained_ranks == 0 {
        return Ok(WeaponResonUpResponse {
            ErrorCode: Some(ErrorCode::ErrWeaponResonConsumeItem),
            IncId: req.IncId,
            ResonLevel: weapon_snapshot.reson_level,
        });
    }

    if !costs.is_empty() {
        let ok = {
            let State {
                conn,
                fs,
                player_components,
                ..
            } = state;
            consume::consume_costs(
                conn,
                fs,
                &mut player_components.basic,
                &mut player_components.inventory,
                &costs,
            )
            .map_err(|e| e.to_string())?
        };
        if !ok {
            return Ok(WeaponResonUpResponse {
                ErrorCode: Some(ErrorCode::ErrItemNotEnough),
                IncId: req.IncId,
                ResonLevel: weapon_snapshot.reson_level,
            });
        }
    }

    for &fid in &fodder_ids {
        state.player_components.weapon.weapon_map.remove(&fid);
        PlayerWeaponComponent::delete_weapon_file(
            &state.fs,
            state.player_components.weapon.player_id,
            fid,
        )
        .map_err(|e| e.to_string())?;
    }

    let new_reson_level = (old_reson_level + gained_ranks).min(config.ResonLevelLimit);
    let equipped_role = {
        let Some(target_weapon) = state.player_components.weapon.weapon_map.get_mut(&req.IncId)
        else {
            return Ok(WeaponResonUpResponse {
                ErrorCode: Some(ErrorCode::ErrWeaponConsumeItemNotFound),
                ..Default::default()
            });
        };
        target_weapon.reson_level = new_reson_level;
        let w_clone = target_weapon.clone();
        save_weapon_immediate(
            &state.fs,
            state.player_components.weapon.player_id,
            req.IncId,
            &w_clone,
        )?;
        w_clone.role_id
    };

    events.enqueue(Event::WeaponInfoModified {
        player_id: state.player_id,
        incr_id: req.IncId,
    });
    if !fodder_ids.is_empty() {
        state.conn.push(
            WeaponItemRemoveNotify::MSG_ID,
            &WeaponItemRemoveNotify {
                WeaponItemIncrIdList: fodder_ids,
            },
        );
    }

    if let Some(role_id) = equipped_role {
        let old_effects: Vec<i64> = state
            .assets
            .tables
            .get_weapon_reson(config.ResonId, old_reson_level)
            .map(|r| r.Effect.clone())
            .unwrap_or_default();
        let new_effects: Vec<i64> = state
            .assets
            .tables
            .get_weapon_reson(config.ResonId, new_reson_level)
            .map(|r| r.Effect.clone())
            .unwrap_or_default();
        let buff_changed = sync_weapon_passive_buffs(
            state,
            events,
            role_id,
            &old_effects,
            &new_effects,
            now_ms_val,
        );
        let State {
            conn,
            fs,
            assets,
            player_components,
            scene: scene_opt,
            ..
        } = state;
        if let Some(scene) = scene_opt.as_mut() {
            if buff_changed && let Some(net_id) = scene.find_entity_net_id_by_config_id(role_id) {
                scene
                    .save_entity_by_net_id(fs, net_id)
                    .map_err(|e| e.to_string())?;
            }
            role_attribute_sync::refresh_role(
                conn,
                fs,
                scene,
                assets,
                &mut player_components.role,
                &player_components.weapon,
                &player_components.echo,
                role_id,
            )
            .map_err(|e| e.to_string())?;
        }
    }

    Ok(WeaponResonUpResponse {
        ErrorCode: Some(ErrorCode::Success),
        IncId: req.IncId,
        ResonLevel: new_reson_level,
    })
}

pub fn handle_equip_take_on_request(
    state: &mut State,
    events: &mut EventQueue,
    req: EquipTakeOnRequest,
) -> Result<EquipTakeOnResponse, String> {
    let now_ms_val = now_ms();
    let Some(data) = req.Data else {
        return Ok(EquipTakeOnResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
            ..Default::default()
        });
    };

    if state.player_components.role.is_trial_role(data.RoleId) {
        return Ok(EquipTakeOnResponse {
            ErrorCode: Some(ErrorCode::ErrLoadEquipInvalidRole),
            ..Default::default()
        });
    }

    let Some(current_role_weapon) = state
        .player_components
        .role
        .role_map
        .get(&data.RoleId)
        .map(|r| r.weapon)
    else {
        return Ok(EquipTakeOnResponse {
            ErrorCode: Some(ErrorCode::ErrLoadEquipInvalidRole),
            ..Default::default()
        });
    };

    let Some(role_cfg) = state.assets.tables.resolve_base_role_info(data.RoleId) else {
        return Ok(EquipTakeOnResponse {
            ErrorCode: Some(ErrorCode::ErrLoadEquipRoleConfig),
            ..Default::default()
        });
    };

    let mut send_data = Vec::new();
    if current_role_weapon == data.EquipIncId {
        send_data.push(RoleLoadEquipData {
            RoleId: data.RoleId,
            Pos: Some(EquipPos::Weapon),
            EquipIncId: data.EquipIncId,
        });
        return Ok(EquipTakeOnResponse {
            ErrorCode: Some(ErrorCode::Success),
            DataList: send_data,
        });
    }

    let Some(weapon_snapshot) = state
        .player_components
        .weapon
        .weapon_map
        .get(&data.EquipIncId)
        .cloned()
    else {
        return Ok(EquipTakeOnResponse {
            ErrorCode: Some(ErrorCode::ErrWeaponConsumeItemNotFound),
            ..Default::default()
        });
    };
    let Some(weapon_conf) = state
        .assets
        .tables
        .weapon_conf
        .get_data_by_id(&weapon_snapshot.id)
        .map(|r| r.into_owned())
    else {
        return Ok(EquipTakeOnResponse {
            ErrorCode: Some(ErrorCode::ErrWeaponResonConfigNotFound),
            ..Default::default()
        });
    };

    let weapon_type = if weapon_conf.WeaponType > 0 {
        weapon_conf.WeaponType
    } else {
        (weapon_conf.ItemId / 10000) % 100
    };
    if role_cfg.WeaponType != weapon_type {
        return Ok(EquipTakeOnResponse {
            ErrorCode: Some(ErrorCode::ErrLoadEquipInvalidRole),
            ..Default::default()
        });
    }

    let old_weapon_snapshot = (current_role_weapon != 0)
        .then(|| {
            state
                .player_components
                .weapon
                .weapon_map
                .get(&current_role_weapon)
                .cloned()
        })
        .flatten();

    let old_weapon_effects: Vec<i64> = old_weapon_snapshot
        .as_ref()
        .and_then(|ow| state.assets.tables.weapon_conf.get_data_by_id(&ow.id))
        .and_then(|ow_conf| state.assets.tables.get_weapon_reson(ow_conf.ResonId, old_weapon_snapshot.as_ref()?.reson_level))
        .map(|r| r.Effect.clone())
        .unwrap_or_default();

    let current_weapon_effects: Vec<i64> = state
        .assets
        .tables
        .get_weapon_reson(weapon_conf.ResonId, weapon_snapshot.reson_level)
        .map(|r| r.Effect.clone())
        .unwrap_or_default();

    let mut swapped_role_id = None;
    if let Some(old_role_id) = weapon_snapshot.role_id {
        if state.player_components.role.is_trial_role(old_role_id)
            || !state.player_components.role.role_map.contains_key(&old_role_id)
        {
            return Ok(EquipTakeOnResponse {
                ErrorCode: Some(ErrorCode::ErrLoadEquipInvalidRole),
                ..Default::default()
            });
        }
        if let Some(ow_snap) = old_weapon_snapshot.as_ref() {
            swapped_role_id = Some(old_role_id);
            if let Some(ow) = state
                .player_components
                .weapon
                .weapon_map
                .get_mut(&current_role_weapon)
            {
                ow.role_id = Some(old_role_id);
            }
            if let Some(old_role) = state.player_components.role.role_map.get_mut(&old_role_id) {
                old_role.weapon = current_role_weapon;
            }
            send_data.push(RoleLoadEquipData {
                RoleId: old_role_id,
                Pos: Some(EquipPos::Weapon),
                EquipIncId: current_role_weapon,
            });
            let _ = sync_weapon_passive_buffs(
                state,
                events,
                old_role_id,
                &current_weapon_effects,
                &old_weapon_effects,
                now_ms_val,
            );
            if let Some(scene) = state.scene.as_mut() {
                let mut updated = None;
                for storage in &mut scene.entities {
                    if storage.config.config_id != old_role_id {
                        continue;
                    }
                    if let Some(equip) = storage.equip.as_mut() {
                        equip.weapon_id = ow_snap.id;
                        equip.weapon_breach_level = ow_snap.breach;
                        updated = Some((storage.net_id, equip.to_proto()));
                    }
                    break;
                }
                if let Some((net_id, pb_equip)) = updated {
                    scene
                        .save_entity_by_net_id(&state.fs, net_id)
                        .map_err(|e| e.to_string())?;
                    state.conn.push(
                        EntityEquipChangeNotify::MSG_ID,
                        &EntityEquipChangeNotify {
                            EntityId: net_id,
                            EquipComponent: Some(pb_equip),
                        },
                    );
                }
            }
            events.enqueue(Event::WeaponInfoModified {
                player_id: state.player_id,
                incr_id: current_role_weapon,
            });
            events.enqueue(Event::RoleInfoModified {
                player_id: state.player_id,
                role_id: old_role_id,
            });
        } else {
            if let Some(old_role) = state.player_components.role.role_map.get_mut(&old_role_id) {
                old_role.weapon = 0;
            }
            events.enqueue(Event::RoleInfoModified {
                player_id: state.player_id,
                role_id: old_role_id,
            });
        }
    } else if current_role_weapon != 0
        && let Some(old_weapon) = state
            .player_components
            .weapon
            .weapon_map
            .get_mut(&current_role_weapon)
    {
        old_weapon.role_id = None;
        events.enqueue(Event::WeaponInfoModified {
            player_id: state.player_id,
            incr_id: current_role_weapon,
        });
        send_data.push(RoleLoadEquipData {
            RoleId: 0,
            Pos: Some(EquipPos::Weapon),
            EquipIncId: current_role_weapon,
        });
    }

    if let Some(weapon) = state
        .player_components
        .weapon
        .weapon_map
        .get_mut(&data.EquipIncId)
    {
        weapon.role_id = Some(data.RoleId);
    }
    if let Some(cur_role) = state.player_components.role.role_map.get_mut(&data.RoleId) {
        cur_role.weapon = data.EquipIncId;
    }

    send_data.push(RoleLoadEquipData {
        RoleId: data.RoleId,
        Pos: Some(EquipPos::Weapon),
        EquipIncId: data.EquipIncId,
    });
    let _ = sync_weapon_passive_buffs(
        state,
        events,
        data.RoleId,
        &old_weapon_effects,
        &current_weapon_effects,
        now_ms_val,
    );
    if let Some(scene) = state.scene.as_mut() {
        let mut updated = None;
        for storage in &mut scene.entities {
            if storage.config.config_id != data.RoleId {
                continue;
            }
            if let Some(equip) = storage.equip.as_mut() {
                equip.weapon_id = weapon_snapshot.id;
                equip.weapon_breach_level = weapon_snapshot.breach;
                updated = Some((storage.net_id, equip.to_proto()));
            }
            break;
        }
        if let Some((net_id, pb_equip)) = updated {
            scene
                .save_entity_by_net_id(&state.fs, net_id)
                .map_err(|e| e.to_string())?;
            state.conn.push(
                EntityEquipChangeNotify::MSG_ID,
                &EntityEquipChangeNotify {
                    EntityId: net_id,
                    EquipComponent: Some(pb_equip),
                },
            );
        }
    }

    events.enqueue(Event::RoleInfoModified {
        player_id: state.player_id,
        role_id: data.RoleId,
    });
    events.enqueue(Event::WeaponInfoModified {
        player_id: state.player_id,
        incr_id: data.EquipIncId,
    });
    events.enqueue(Event::UpdateFormations {
        player_id: state.player_id,
        formation_id: 0,
        role_ids: Vec::new(),
    });

    if let Some(old_role_id) = swapped_role_id {
        let State {
            conn,
            fs,
            assets,
            player_components,
            scene: scene_opt,
            ..
        } = state;
        if let Some(scene) = scene_opt.as_mut() {
            role_attribute_sync::refresh_role(
                conn,
                fs,
                scene,
                assets,
                &mut player_components.role,
                &player_components.weapon,
                &player_components.echo,
                old_role_id,
            )
            .map_err(|e| e.to_string())?;
        }
    }
    {
        let State {
            conn,
            fs,
            assets,
            player_components,
            scene: scene_opt,
            ..
        } = state;
        if let Some(scene) = scene_opt.as_mut() {
            role_attribute_sync::refresh_role(
                conn,
                fs,
                scene,
                assets,
                &mut player_components.role,
                &player_components.weapon,
                &player_components.echo,
                data.RoleId,
            )
            .map_err(|e| e.to_string())?;
        }
    }

    state.conn.push(
        EquipTakeOnNotify::MSG_ID,
        &EquipTakeOnNotify {
            DataList: send_data.clone(),
        },
    );
    Ok(EquipTakeOnResponse {
        ErrorCode: Some(ErrorCode::Success),
        DataList: send_data,
    })
}
