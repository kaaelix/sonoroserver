use std::collections::BTreeMap;

use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::helpers::{condition, craft, energy, quest_engine};
use sonoro_proto::enums::ErrorCode;
use sonoro_proto::pb::{
    TowerApplyFloorDataRequest, TowerApplyFloorDataResponse, TowerAreaPb, TowerDifficultyPb,
    TowerDifficultyUpdateNotify, TowerFloorPb, TowerFloorUpdateNotify,
    TowerFormationRecommendRequest, TowerFormationRecommendResponse, TowerGuideActivityInfoRequest,
    TowerGuideActivityInfoResponse, TowerGuideActivityRewardRequest,
    TowerGuideActivityRewardResponse, TowerInfo, TowerRecommendFormation, TowerRecommendRole,
    TowerRequest, TowerResetRequest, TowerResetResponse, TowerResponse, TowerRewardRequest,
    TowerRewardResponse, TowerSeasonUpdateRequest, TowerSeasonUpdateResponse,
    TowerSeasonUpdateResponseTowersOneof, TowerStartRequest, TowerStartResponse,
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

pub fn handle_tower_guide_activity_reward_request(
    state: &mut State,
    req: TowerGuideActivityRewardRequest,
) -> Result<TowerGuideActivityRewardResponse, String> {
    let key = format!("tower_guide:{}", req.TowerGuideId);
    if state.player_components.progress.info.mark_claim(&key) {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        let mut rctx = state.reward_context();
        let _ = sonoro_gamesv_logic::helpers::reward::grant_items(
            &mut rctx,
            0,
            true,
            &[
                sonoro_gamesv_logic::helpers::reward::RewardItem {
                    item_id: 3,
                    count: 100,
                },
                sonoro_gamesv_logic::helpers::reward::RewardItem {
                    item_id: 2,
                    count: 20000,
                },
            ],
            19500,
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(TowerGuideActivityRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}
