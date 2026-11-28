use std::collections::BTreeMap;

use serde_json::Value;
use sonoro_gamesv_data::Assets;
use sonoro_gamesv_fs::role_info::RoleInfo;
use sonoro_gamesv_logic::component::comp_util;
use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::helpers::{
    condition, craft, energy, quest_engine, reward, role as role_helper, role_stats,
};
use sonoro_proto::enums::ErrorCode;
use sonoro_proto::pb::{
    PbGetRoleListNotify, TowerApplyFloorDataRequest, TowerApplyFloorDataResponse, TowerAreaPb,
    TowerDifficultyPb, TowerDifficultyUpdateNotify, TowerFloorPb, TowerFloorUpdateNotify,
    TowerFormationRecommendRequest, TowerFormationRecommendResponse, TowerGuideActivityInfoRequest,
    TowerGuideActivityInfoResponse, TowerGuideActivityRewardRequest,
    TowerGuideActivityRewardResponse, TowerInfo, TowerRecommendFormation, TowerRecommendRole,
    TowerRequest, TowerResetRequest, TowerResetResponse, TowerResponse, TowerRewardRequest,
    TowerRewardResponse, TowerSeasonUpdateRequest, TowerSeasonUpdateResponse,
    TowerSeasonUpdateResponseTowersOneof, TowerStartRequest, TowerStartResponse,
    WeaponItemAddNotify,
};

use super::State;

fn build_difficulty_pb(state: &State, diff: i32) -> TowerDifficultyPb {
    let progress = &state.player_components.progress.info;
    let claimed: Vec<i32> = progress
        .challenge
        .tower_claimed_rewards
        .iter()
        .filter(|rec| rec.difficulty == diff)
        .map(|rec| rec.reward_index)
        .collect();

    let mut area_nums = Vec::new();
    for cfg in &state.assets.tables.tower_config.items {
        if cfg.Difficulty == diff && !area_nums.contains(&cfg.AreaNum) {
            area_nums.push(cfg.AreaNum);
        }
    }

    let mut max_star = 0i32;
    let mut areas = Vec::new();
    for anum in area_nums {
        let mut floors = Vec::new();
        for fl in &progress.challenge.tower_floors {
            if fl.difficulty != diff || fl.area_num != anum {
                continue;
            }
            let mut star_idx = Vec::new();
            for bit_i in 1..=3 {
                if (fl.star_mask & (1 << (bit_i - 1))) != 0 {
                    star_idx.push(bit_i);
                }
            }
            max_star += fl.star;
            floors.push(TowerFloorPb {
                TowerConfigId: fl.config_id,
                Star: fl.star,
                StarIndex: star_idx,
                Formation: Vec::new(),
                IsQuickPass: false,
            });
        }
        areas.push(TowerAreaPb {
            AreaNum: anum,
            TowerFloors: floors,
        });
    }

    TowerDifficultyPb {
        Difficulty: diff,
        RewardIndex: claimed,
        TowerAreas: areas,
        MaxStar: max_star,
    }
}

fn current_unlock_difficulty(state: &State) -> i32 {
    if !condition::is_function_open(&state.eval_context(), 10055) {
        return 0;
    }
    let challenge = &state.player_components.progress.info.challenge;
    let season = challenge.tower_season;
    let mut unlocked = 1i32;
    for d in 1..=3 {
        let mut any_floor = false;
        let mut all_cleared = true;
        for cfg in &state.assets.tables.tower_config.items {
            if cfg.Difficulty == d && (cfg.Season == 0 || cfg.Season == season) {
                any_floor = true;
                if !challenge
                    .tower_floors
                    .iter()
                    .any(|fl| fl.config_id == cfg.Id && fl.star > 0)
                {
                    all_cleared = false;
                    break;
                }
            }
        }
        if any_floor && all_cleared {
            unlocked = d + 1;
        } else {
            break;
        }
    }
    unlocked
}

pub fn build_tower_info(state: &State, now_sec: i64) -> TowerInfo {
    let diffs = state
        .assets
        .tables
        .tower_difficulty
        .items
        .iter()
        .map(|d| build_difficulty_pb(state, d.Difficulty))
        .collect();
    let season = state.player_components.progress.info.challenge.tower_season;
    TowerInfo {
        CurrentSeason: season,
        DataSeason: season,
        TowerDifficulties: diffs,
        BeginTime: now_sec - 86_400,
        EndTime: now_sec + 2_592_000,
        MaxUnlockDifficulty: current_unlock_difficulty(state),
        QuickPassId: 0,
    }
}

pub fn handle_tower_request(
    state: &mut State,
    _req: TowerRequest,
) -> Result<TowerResponse, String> {
    Ok(TowerResponse {
        TowerInfo: Some(build_tower_info(state, energy::now_seconds())),
    })
}

pub fn handle_tower_season_update_request(
    state: &mut State,
    _req: TowerSeasonUpdateRequest,
) -> Result<TowerSeasonUpdateResponse, String> {
    let unlock_diff = current_unlock_difficulty(state);
    let announced = state
        .player_components
        .progress
        .info
        .get_key_int("tower:announced_unlock_diff") as i32;
    let newly_unlocked = if unlock_diff > 0 && unlock_diff > announced {
        state
            .player_components
            .progress
            .info
            .set_key_int("tower:announced_unlock_diff", i64::from(unlock_diff));
        let _ = state.player_components.progress.save(&state.fs);
        unlock_diff
    } else {
        0
    };
    let info = build_tower_info(state, energy::now_seconds());
    Ok(TowerSeasonUpdateResponse {
        Towers: Some(TowerSeasonUpdateResponseTowersOneof::TowerInfo(info)),
        MaxUnlockDifficulty: newly_unlocked,
    })
}

pub fn handle_tower_start_request(
    state: &mut State,
    events: &mut EventQueue,
    req: TowerStartRequest,
) -> Result<TowerStartResponse, String> {
    let Some(cfg) = state
        .assets
        .tables
        .tower_config
        .get_data_by_id(&req.TowerConfigId)
        .map(|c| c.into_owned())
    else {
        return Ok(TowerStartResponse {
            ErrorCode: Some(ErrorCode::ErrTowerConfigNotFound),
            NeedUpdateSeason: false,
        });
    };
    let pc = &mut state.player_components.progress;
    pc.info
        .set_key_int("tower:start_ms", energy::now_seconds().saturating_mul(1000));
    pc.save(&state.fs).map_err(|e| e.to_string())?;
    if cfg.InstanceId > 0 {
        let mut qctx = state.quest_context(events);
        quest_engine::enter_inst(&mut qctx, cfg.InstanceId, 0).map_err(|e| e.to_string())?;
    }
    Ok(TowerStartResponse {
        ErrorCode: Some(ErrorCode::Success),
        NeedUpdateSeason: false,
    })
}

pub fn handle_tower_reset_request(
    state: &mut State,
    req: TowerResetRequest,
) -> Result<TowerResetResponse, String> {
    let id = req.TowerConfigId;
    let Some(cfg) = state
        .assets
        .tables
        .tower_config
        .get_data_by_id(&id)
        .map(|c| c.into_owned())
    else {
        return Ok(TowerResetResponse {
            TowerConfigId: id,
            ErrorCode: Some(ErrorCode::ErrTowerConfigNotFound),
            ..Default::default()
        });
    };

    let floors = &mut state.player_components.progress.info.challenge.tower_floors;
    let before = floors.len();
    floors.retain(|fl| fl.config_id != id);
    if floors.len() < before {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }

    let _ = state.conn.push_msg(
        TowerFloorUpdateNotify::MSG_ID,
        &TowerFloorUpdateNotify {
            TowerFloors: vec![TowerFloorPb {
                TowerConfigId: id,
                Star: 0,
                IsQuickPass: false,
                ..Default::default()
            }],
        },
    );
    let diff_pb = build_difficulty_pb(state, cfg.Difficulty);
    let _ = state.conn.push_msg(
        TowerDifficultyUpdateNotify::MSG_ID,
        &TowerDifficultyUpdateNotify {
            TowerDifficulties: vec![diff_pb],
        },
    );

    Ok(TowerResetResponse {
        NeedUpdateSeason: false,
        TowerConfigId: id,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_tower_reward_request(
    state: &mut State,
    req: TowerRewardRequest,
) -> Result<TowerRewardResponse, String> {
    let diff = req.Difficulty;
    let Some(diff_cfg) = state
        .assets
        .tables
        .tower_difficulty
        .get_data_by_id(&diff)
        .map(|c| c.into_owned())
    else {
        return Ok(TowerRewardResponse {
            ErrorCode: Some(ErrorCode::ErrTowerRewardNotFound),
            ..Default::default()
        });
    };

    let indices: Vec<i32> = if !req.RewardIndexList.is_empty() {
        req.RewardIndexList
    } else if req.RewardIndex >= 0 {
        vec![req.RewardIndex]
    } else {
        Vec::new()
    };

    let stars: i32 = state
        .player_components
        .progress
        .info
        .challenge
        .tower_floors
        .iter()
        .filter(|fl| fl.difficulty == diff)
        .map(|fl| fl.star)
        .sum();

    let mut claimed = Vec::new();
    let mut item_map = BTreeMap::new();
    let mut changed = false;

    for ridx in indices {
        if ridx < 0 || (ridx as usize) >= diff_cfg.Reward.len() {
            continue;
        }
        let entry = &diff_cfg.Reward[ridx as usize];
        if stars < entry.Item1 {
            continue;
        }
        if !state
            .player_components
            .progress
            .info
            .claim_tower_reward(diff, ridx)
        {
            continue;
        }
        let mut cctx = state.craft_context();
        craft::grant_drop_and_collect_items(&mut cctx, entry.Item2, 3, 50, 19500, &mut item_map)
            .map_err(|e| e.to_string())?;
        claimed.push(ridx);
        changed = true;
    }

    if changed {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        let diff_pb = build_difficulty_pb(state, diff);
        let _ = state.conn.push_msg(
            TowerDifficultyUpdateNotify::MSG_ID,
            &TowerDifficultyUpdateNotify {
                TowerDifficulties: vec![diff_pb],
            },
        );
    }

    Ok(TowerRewardResponse {
        NeedUpdateSeason: false,
        ErrorCode: Some(ErrorCode::Success),
        RewardIndexList: claimed,
    })
}

pub fn handle_tower_apply_floor_data_request(
    state: &mut State,
    req: TowerApplyFloorDataRequest,
) -> Result<TowerApplyFloorDataResponse, String> {
    if req.Apply {
        let pc = &mut state.player_components.progress;
        pc.info.set_key_int("tower:apply_floor", 1);
        pc.save(&state.fs).map_err(|e| e.to_string())?;
        let last_diff = pc.info.get_key_int("tower:last_diff");
        if last_diff > 0 {
            let diff_pb = build_difficulty_pb(state, last_diff as i32);
            let _ = state.conn.push_msg(
                TowerDifficultyUpdateNotify::MSG_ID,
                &TowerDifficultyUpdateNotify {
                    TowerDifficulties: vec![diff_pb],
                },
            );
        }
    }
    Ok(TowerApplyFloorDataResponse {
        ErrorCode: Some(ErrorCode::Success),
        NeedUpdateSeason: false,
    })
}

pub fn handle_tower_formation_recommend_request(
    state: &mut State,
    req: TowerFormationRecommendRequest,
) -> Result<TowerFormationRecommendResponse, String> {
    let roles: Vec<TowerRecommendRole> = state
        .assets
        .tables
        .role_info
        .items
        .iter()
        .filter(|r| (1000..2000).contains(&r.Id))
        .take(3)
        .map(|r| TowerRecommendRole {
            RoleId: r.Id,
            Level: 90,
            ..Default::default()
        })
        .collect();
    let recs = if roles.is_empty() {
        Vec::new()
    } else {
        vec![TowerRecommendFormation {
            Formation: roles,
            Usage: 100,
        }]
    };
    Ok(TowerFormationRecommendResponse {
        ErrorCode: Some(ErrorCode::Success),
        TowerConfigId: req.TowerConfigId,
        Formations: recs,
        NeedUpdateSeason: false,
    })
}

pub fn handle_tower_guide_activity_info_request(
    state: &mut State,
    _req: TowerGuideActivityInfoRequest,
) -> Result<TowerGuideActivityInfoResponse, String> {
    let ids = state
        .player_components
        .progress
        .info
        .dynamic
        .claims
        .iter()
        .filter_map(|k| k.strip_prefix("tower_guide:")?.parse::<i32>().ok())
        .collect();
    Ok(TowerGuideActivityInfoResponse { TowerGuideId: ids })
}

fn is_tower_difficulty_completed(state: &State, difficulty: i32) -> bool {
    if difficulty <= 0 {
        return false;
    }
    let challenge = &state.player_components.progress.info.challenge;
    let season = challenge.tower_season;
    let mut any_floor = false;
    for cfg in &state.assets.tables.tower_config.items {
        if cfg.Difficulty == difficulty && (cfg.Season == 0 || cfg.Season == season) {
            any_floor = true;
            if !challenge
                .tower_floors
                .iter()
                .any(|fl| fl.config_id == cfg.Id && fl.star > 0)
            {
                return false;
            }
        }
    }
    any_floor
}

fn guide_role_waveband_id(assets: &Assets, role_id: i32) -> Option<i32> {
    let role = assets.tables.role_info.get_data_by_id(&role_id)?;
    if role.ResonantChainGroupId == 0 {
        return None;
    }
    assets
        .tables
        .resonant_chain
        .items
        .iter()
        .find(|chain| chain.GroupId == role.ResonantChainGroupId && chain.GroupIndex == 1)
        .and_then(|chain| chain.ActivateConsume.keys().next().copied())
}

fn grant_tower_guide_rewards(
    state: &mut State,
    raw_items: &[reward::RewardItem],
) -> Result<(), String> {
    let mut normal_items = Vec::new();
    let mut starter_weapons = Vec::new();
    let mut has_new_roles = false;

    for &entry in raw_items {
        if entry.item_id <= 0 || entry.count <= 0 {
            continue;
        }
        if state
            .assets
            .tables
            .role_info
            .get_data_by_id(&entry.item_id)
            .is_some()
        {
            for _ in 0..entry.count {
                if state
                    .player_components
                    .role
                    .role_map
                    .contains_key(&entry.item_id)
                {
                    let wb = guide_role_waveband_id(&state.assets, entry.item_id).unwrap_or(50003);
                    normal_items.push(reward::RewardItem {
                        item_id: wb,
                        count: if wb == 50003 { 3 } else { 1 },
                    });
                } else {
                    let new_role = RoleInfo::create_default(&state.assets, entry.item_id)
                        .map_err(|e| e.to_string())?;
                    state
                        .player_components
                        .role
                        .role_map
                        .insert(entry.item_id, new_role.clone());
                    let role_path = format!(
                        "player/{}/role/{}",
                        state.player_components.role.player_id, entry.item_id
                    );
                    comp_util::save_struct(&state.fs, &new_role, &role_path)
                        .map_err(|e| e.to_string())?;
                    if role_helper::ensure_role_weapon(
                        &state.fs,
                        &state.assets,
                        &mut state.player_components.role,
                        &mut state.player_components.weapon,
                        entry.item_id,
                    )
                    .unwrap_or(false)
                        && let Some(rinfo) = state.player_components.role.role_map.get(&entry.item_id)
                        && let Some(witem) = state.player_components.weapon.weapon_map.get(&rinfo.weapon)
                    {
                        starter_weapons.push(witem.to_proto(rinfo.weapon));
                    }
                    has_new_roles = true;
                }
            }
        } else {
            normal_items.push(entry);
        }
    }

    if !starter_weapons.is_empty() {
        let _ = state.conn.push_msg(
            WeaponItemAddNotify::MSG_ID,
            &WeaponItemAddNotify {
                WeaponItemList: starter_weapons,
                AddFromRole: true,
                Reason: 1,
            },
        );
    }

    if !normal_items.is_empty() {
        let mut rctx = state.reward_context();
        reward::grant_items(&mut rctx, 0, true, &normal_items, 19500)
            .map_err(|e| e.to_string())?;
    }

    if has_new_roles {
        let selected_main_role = role_helper::selected_main_role_id(
            &state.assets,
            &state.player_components.role,
            state.player_components.basic.info.attributes.sex,
            state.player_components.basic.info.selected_main_role_id,
        );
        let role_ids: Vec<i32> = state
            .player_components
            .role
            .role_map
            .keys()
            .copied()
            .filter(|&id| {
                (1000..=1999).contains(&id)
                    && selected_main_role.is_none_or(|selected| {
                        !role_helper::is_main_role(&state.assets, id) || id == selected
                    })
            })
            .collect();
        let mut role_list = Vec::with_capacity(role_ids.len());
        for role_id in role_ids {
            let Some(role_info) = state.player_components.role.role_map.get(&role_id).cloned()
            else {
                continue;
            };
            let pb_role = role_stats::to_client_role_info(
                &state.assets,
                &mut state.player_components.role,
                role_id,
                &role_info,
                &state.player_components.weapon,
                &state.player_components.echo,
            )
            .map_err(|e| e.to_string())?;
            role_list.push(pb_role);
        }
        let _ = state.conn.push_msg(
            PbGetRoleListNotify::MSG_ID,
            &PbGetRoleListNotify {
                RoleList: role_list,
            },
        );
    }
    Ok(())
}

pub fn handle_tower_guide_activity_reward_request(
    state: &mut State,
    req: TowerGuideActivityRewardRequest,
) -> Result<TowerGuideActivityRewardResponse, String> {
    let guide_row = state
        .assets
        .raw_json_cached("TowerGuide")
        .iter()
        .find(|row| row.get("Id").and_then(Value::as_i64) == Some(i64::from(req.TowerGuideId)))
        .cloned();
    let Some(row) = guide_row else {
        return Ok(TowerGuideActivityRewardResponse {
            ErrorCode: Some(ErrorCode::ErrTowerGuideConfig),
        });
    };

    let difficulty = row
        .get("Difficulty")
        .and_then(Value::as_i64)
        .unwrap_or(0) as i32;
    if !is_tower_difficulty_completed(state, difficulty) {
        return Ok(TowerGuideActivityRewardResponse {
            ErrorCode: Some(ErrorCode::ErrTowerGuideNoOpen),
        });
    }

    let key = format!("tower_guide:{}", req.TowerGuideId);
    if !state.player_components.progress.info.mark_claim(&key) {
        return Ok(TowerGuideActivityRewardResponse {
            ErrorCode: Some(ErrorCode::ErrTowerGuideRewardHad),
        });
    }
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;

    let mut ordered_items: BTreeMap<i32, i32> = BTreeMap::new();
    if let Some(reward_obj) = row.get("RewardItem").and_then(Value::as_object) {
        for (k, v) in reward_obj {
            if let (Ok(item_id), Some(count)) = (k.parse::<i32>(), v.as_i64())
                && item_id > 0
                && count > 0
            {
                *ordered_items.entry(item_id).or_default() += count as i32;
            }
        }
    }
    let reward_items: Vec<reward::RewardItem> = ordered_items
        .into_iter()
        .map(|(item_id, count)| reward::RewardItem { item_id, count })
        .collect();
    if !reward_items.is_empty() {
        grant_tower_guide_rewards(state, &reward_items)?;
    }

    Ok(TowerGuideActivityRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}
