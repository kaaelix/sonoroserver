use std::collections::{BTreeMap, HashSet};

use serde_json::Value;
use sonoro_gamesv_data::Assets;
use sonoro_gamesv_fs::echo_info::EchoInfo;
use sonoro_gamesv_logic::component::entity::small::EntityType;
use sonoro_gamesv_logic::coop_service::CoopService;
use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::helpers::level_play::LevelPlayEventKind;
use sonoro_gamesv_logic::helpers::{attributes, energy, level_play, pb_kv, quest_engine, reward, role_stats};
use sonoro_proto::enums::{ActivityTaskState, BossRushRewardType, EAttributeType, ErrorCode};
use sonoro_proto::pb::{
    AbyssChallengeGotoNextRoomRequest, AbyssChallengeGotoNextRoomResponse, AbyssChallengeInfo,
    AbyssChallengePassPlayerInfo, AbyssChallengeQuitRequest, AbyssChallengeQuitResponse,
    AbyssChallengeUnlockRequest, AbyssChallengeUnlockResponse, AbyssLikeRequest, AbyssLikeResponse,
    AbyssLittleRoleLevelUpRequest, AbyssLittleRoleLevelUpResponse, AbyssPluginItemRemoveNotify,
    AbyssPluginPutRequest, AbyssPluginPutResponse, AbyssPluginSynthesisRequest,
    AbyssPluginSynthesisResponse, AbyssRankListRequest, AbyssRankListResponse,
    AbyssReviveCharacterRequest, AbyssReviveCharacterResponse, AbyssReviveNotify,
    AbyssRewardRequest, AbyssRewardResponse, AbyssSelfPassRequest, AbyssSelfPassResponse,
    AbyssShowNameRequest, AbyssShowNameResponse, ActivityTask, AddCountItemInfo, ArrayIntInt,
    BabelActivitySettleInfoNotify, BattleFormation, BabelTowerDailyTaskRewardRequest,
    BabelTowerDailyTaskRewardResponse, BabelTowerRankListRequest, BabelTowerRankListResponse,
    BabelTowerSelfPassRequest, BabelTowerSelfPassResponse, BabelTowerSettlementRequest,
    BabelTowerSettlementResponse, BabelTowerShowNameRequest, BabelTowerShowNameResponse,
    BabelTowerTaskRewardRequest, BabelTowerTaskRewardResponse, BossPilingRewardRequest,
    BossPilingRewardResponse, BossPilingSettleRequest, BossPilingSettleResponse,
    BossRushBatchClaimRewardRequest, BossRushBatchClaimRewardResponse,
    BossRushBuffChooseResultRequest, BossRushBuffChooseResultResponse, BossRushChooseBuffRequest,
    BossRushChooseBuffResponse, BossRushClaimRewardRequest, BossRushClaimRewardResponse,
    BossRushLevelScoreRewardRequest, BossRushLevelScoreRewardResponse, BossRushResultNotify,
    BossRushSettlementRequest, BossRushSettlementResponse, BossRushTaskRewardRequest,
    BossRushTaskRewardResponse, EndLessHistoryRequest, EndLessHistoryResponse, EndlessBattleRecord,
    HalfEndlessBattleRecord, NewTowerClearRecordRequest, NewTowerClearRecordResponse,
    NewTowerClimbingLevelRecord, NewTowerCycleRecordPb, NewTowerCycleStartRequest,
    NewTowerCycleStartResponse, NewTowerLastCycleInfoRequest, NewTowerLastCycleInfoResponse,
    NewTowerMedalInfo, NewTowerMedalInfoRequest, NewTowerMedalInfoResponse,
    NewTowerRecommendData, NewTowerRecommendLevel, NewTowerRecommendRequest,
    NewTowerRecommendResponse, NewTowerRecommendTeam, NewTowerRewardRequest,
    NewTowerRewardResponse, NewTowerRoleEnergyRequest, NewTowerRoleEnergyResponse,
    NewTowerSeasonRewardRequest, NewTowerSeasonRewardResponse, NewTowerSeasonScoreRewardRequest,
    NewTowerSeasonScoreRewardResponse, NewTowerSeasonTaskDataUpdateRequest,
    NewTowerSeasonTaskDataUpdateResponse, NewTowerUpdateRecordRequest,
    NewTowerUpdateRecordResponse, PublicBabelTowerInfo, RecordRoleEnergy,
    ResetBabelTowerLevelRequest, ResetBabelTowerLevelResponse, RolePhantomPropInfo, RolePropInfo,
    RoleTowerAttributeQueryRequest, RoleTowerAttributeQueryResponse, RoleTowerBestRecord,
    RoleTowerChallengeSettleInfo, RoleTowerChallengeSettleRequest,
    RoleTowerChallengeSettleResponse, RoleTowerConsumeItemRequest, RoleTowerConsumeItemResponse,
    RoleTowerPhantomAddAttr, RoleTowerPhantomAddAttrsRequest, RoleTowerPhantomAddAttrsResponse,
    RoleTowerPhantomAttributeInfo, RoleTowerRankEntry, RoleTowerRankInfo, RoleTowerRankRequest,
    RoleTowerRankResponse, RoleTowerRankRewardReceiveRequest, RoleTowerRankRewardReceiveResponse,
    RoleTowerRewardTaskReceiveRequest, RoleTowerRewardTaskReceiveResponse,
    RoleTowerRoleAttributeInfo, RoleTowerSaveDeleteRequest, RoleTowerSaveDeleteResponse,
    RoleTowerScoreRewardReceiveRequest, RoleTowerScoreRewardReceiveResponse,
    RoleTowerStageRetryRequest, RoleTowerStageRetryResponse, RoleTowerStageSettleRequest,
    RoleTowerStageSettleResponse, RoleTowerStepAdvanceRequest, RoleTowerStepAdvanceResponse,
    SelectBabelActivityDeTermRequest, SelectBabelActivityDeTermResponse, SlashAndTowerInfoRequest,
    SlashAndTowerInfoResponse, SlashAndTowerRecommendRequest, SlashAndTowerRecommendResponse,
    SlashAndTowerResetRequest, SlashAndTowerResetResponse, SlashAndTowerSaveRecordRequest,
    SlashAndTowerSaveRecordResponse, SlashAndTowerScoreRewardRequest,
    SlashAndTowerScoreRewardResponse, SlashLevelPlayInfo, SlashLevelRecommend,
};

use super::super::connection::drain_event_queue_best_effort;
use super::State;

fn claim_reward_list(
    state: &mut State,
    prefix: &str,
    table_name: &str,
    ids: &[i32],
) -> Result<Vec<i32>, String> {
    let mut claimed = Vec::new();
    let mut dirty = false;
    for &id in ids {
        let k = format!("{prefix}:{id}");
        if !state.player_components.progress.info.mark_claim(&k) {
            continue;
        }
        dirty = true;
        claimed.push(id);
        let mut rctx = state.reward_context();
        reward::grant_table_reward(
            &mut rctx,
            table_name,
            i64::from(id),
            false,
            true,
            19500,
            &[],
            None,
        )
        .map_err(|e| e.to_string())?;
    }
    if dirty {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(claimed)
}

fn json_i32(val: &Value, key: &str) -> i32 {
    val.get(key).and_then(Value::as_i64).unwrap_or(0) as i32
}

fn resolve_echo_prop_id(assets: &Assets, raw_id: i32, is_main: bool) -> i32 {
    if is_main {
        if let Some(cfg) = assets.tables.phantom_main_prop_item.get_data_by_id(&raw_id) {
            return cfg.PropId;
        }
    } else if let Some(cfg) = assets.tables.phantom_sub_property.get_data_by_id(&raw_id) {
        return cfg.PropId;
    }
    if let Some(tppi) = assets.tables.trial_phantom_prop_item.get_data_by_id(&raw_id) {
        return tppi.Prop.Id;
    }
    raw_id
}

fn compute_echo_attr_map(assets: &Assets, echo: &EchoInfo) -> BTreeMap<i32, i32> {
    let mut attrs: BTreeMap<i32, i32> = BTreeMap::new();
    for p in &echo.main_prop {
        let key = resolve_echo_prop_id(assets, p.id, true);
        if key != 0 && p.value != 0 {
            *attrs.entry(key).or_default() += p.value;
        }
    }
    for p in &echo.sub_prop {
        let key = resolve_echo_prop_id(assets, p.id, false);
        if key != 0 && p.value != 0 {
            *attrs.entry(key).or_default() += p.value;
        }
    }
    attrs
}

fn role_tower_floor_count(assets: &Assets, challenge_id: i32) -> i32 {
    let floor_group_id = assets
        .raw_json_cached("RoleTowerLevel")
        .iter()
        .find(|row| json_i32(row, "Id") == challenge_id)
        .map_or(0, |row| json_i32(row, "FloorGroupId"));
    if floor_group_id <= 0 {
        return 0;
    }
    assets
        .raw_json_cached("RoleTowerFloor")
        .iter()
        .filter(|row| json_i32(row, "FloorGroupId") == floor_group_id)
        .count() as i32
}

fn find_boss_rush_row(assets: &Assets, inst_id: i32) -> Option<(i32, i32)> {
    if inst_id <= 0 {
        return None;
    }
    assets
        .raw_json_cached("BossRushActivity")
        .iter()
        .find(|row| json_i32(row, "InstId") == inst_id)
        .map(|row| (json_i32(row, "ActivityId"), json_i32(row, "RewardScore")))
}

// === SlashAndTower (Whimpering Wastes) ===

pub fn handle_slash_and_tower_info_request(
    state: &mut State,
    _req: SlashAndTowerInfoRequest,
) -> Result<SlashAndTowerInfoResponse, String> {
    let now_s = energy::now_seconds();
    let rows = state.assets.reward_index.get_rows("SlashAndTowerCfg");
    let pc = &state.player_components.progress.info;
    let mut plays = Vec::new();
    for row in rows.iter() {
        let id = row.key as i32;
        if id <= 0 {
            continue;
        }
        let s1 = pc.get_key_int(&format!("slash:s1:{id}")) as i32;
        let s2 = pc.get_key_int(&format!("slash:s2:{id}")) as i32;
        plays.push(SlashLevelPlayInfo {
            Id: id,
            IsLocked: false,
            FirstScore: s1,
            SecondScore: s2,
            IsPassed: (s1 + s2) > 0,
            ..Default::default()
        });
    }
    let recvd = pc
        .dynamic
        .claims
        .iter()
        .filter_map(|k| k.strip_prefix("slash:clm:")?.parse::<i32>().ok())
        .collect();
    Ok(SlashAndTowerInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        SlashLevelPlayInfo: plays,
        RewardsReceived: recvd,
        CurSeasonEndTime: now_s + 2_592_000,
        ..Default::default()
    })
}

fn owned_role_ids_sorted(state: &State) -> Vec<i32> {
    let mut ids: Vec<i32> = state
        .player_components
        .role
        .role_map
        .keys()
        .copied()
        .collect();
    ids.sort_unstable();
    if ids.is_empty() {
        ids = vec![1402, 1102, 1202];
    }
    ids
}

pub fn handle_slash_and_tower_recommend_request(
    state: &mut State,
    _req: SlashAndTowerRecommendRequest,
) -> Result<SlashAndTowerRecommendResponse, String> {
    let roles = owned_role_ids_sorted(state);
    let first: Vec<i32> = roles.iter().copied().take(3).collect();
    let second: Vec<i32> = if roles.len() > 3 {
        roles.iter().copied().skip(3).take(3).collect()
    } else {
        first.clone()
    };
    Ok(SlashAndTowerRecommendResponse {
        ErrorCode: Some(ErrorCode::Success),
        RecommendInfos: vec![SlashLevelRecommend {
            FormationFirst: Some(BattleFormation {
                SelectRoles: first,
                BuffSelect: 0,
                SkillBranchIds: Vec::new(),
            }),
            FormationSecond: Some(BattleFormation {
                SelectRoles: second,
                BuffSelect: 0,
                SkillBranchIds: Vec::new(),
            }),
            Usage: 1,
        }],
    })
}

pub fn handle_slash_and_tower_reset_request(
    state: &mut State,
    req: SlashAndTowerResetRequest,
) -> Result<SlashAndTowerResetResponse, String> {
    let info = &mut state.player_components.progress.info;
    info.set_key_int(&format!("slash:s1:{}", req.Id), 0);
    info.set_key_int(&format!("slash:s2:{}", req.Id), 0);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(SlashAndTowerResetResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_slash_and_tower_save_record_request(
    state: &mut State,
    req: SlashAndTowerSaveRecordRequest,
) -> Result<SlashAndTowerSaveRecordResponse, String> {
    let pass_score = state
        .assets
        .raw_json_cached("SlashAndTowerCfg")
        .iter()
        .find(|row| json_i32(row, "Id") == req.Id)
        .map_or(0, |row| i64::from(json_i32(row, "PassScore").max(0)));

    let info = &mut state.player_components.progress.info;
    let mut s1 = info
        .get_key_int(&format!("slash:cur_s1:{}", req.Id))
        .max(info.get_key_int(&format!("slash:s1:{}", req.Id)))
        .max(0);
    let mut s2 = info
        .get_key_int(&format!("slash:cur_s2:{}", req.Id))
        .max(info.get_key_int(&format!("slash:s2:{}", req.Id)))
        .max(0);

    let active_lvl = info.get_key_int("slash:level_id");
    let active_floor = info.get_key_int("slash:floor");
    if s1 == 0 && s2 == 0 && active_lvl == i64::from(req.Id) && active_floor > 0 && pass_score > 0 {
        s1 = pass_score;
        if active_floor > 1 {
            s2 = pass_score;
        }
    }

    info.set_key_int(&format!("slash:s1:{}", req.Id), s1);
    info.set_key_int(&format!("slash:s2:{}", req.Id), s2);
    info.set_key_int(&format!("slash:saved:{}", req.Id), 1);

    let mut total_best = 0i64;
    let s1_entries: Vec<(i32, i64)> = info
        .dynamic
        .ints
        .iter()
        .filter_map(|e| {
            let id = e.key.strip_prefix("slash:s1:")?.parse::<i32>().ok()?;
            Some((id, e.val.max(0)))
        })
        .collect();
    for (id, v1) in s1_entries {
        let v2 = info.get_key_int(&format!("slash:s2:{id}")).max(0);
        total_best = total_best.max(v1 + v2);
    }
    let prev_total = info.get_key_int("slash_tower:score");
    info.set_key_int("slash_tower:score", prev_total.max(total_best));

    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(SlashAndTowerSaveRecordResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_slash_and_tower_score_reward_request(
    state: &mut State,
    req: SlashAndTowerScoreRewardRequest,
) -> Result<SlashAndTowerScoreRewardResponse, String> {
    let claimed = claim_reward_list(state, "slash:clm", "SlashAndTowerReward", &req.RewardIds)?;
    Ok(SlashAndTowerScoreRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
        RewardIds: claimed,
    })
}

// === NewTower ===

pub fn handle_new_tower_clear_record_request(
    state: &mut State,
    req: NewTowerClearRecordRequest,
) -> Result<NewTowerClearRecordResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("newtower:score:{}", req.LevelId),
        0,
    )
    .map_err(|e| e.to_string())?;
    Ok(NewTowerClearRecordResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_new_tower_cycle_start_request(
    state: &mut State,
    _req: NewTowerCycleStartRequest,
) -> Result<NewTowerCycleStartResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int("newtower:cycle", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(NewTowerCycleStartResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_new_tower_last_cycle_info_request(
    state: &mut State,
    _req: NewTowerLastCycleInfoRequest,
) -> Result<NewTowerLastCycleInfoResponse, String> {
    let cycle_id = state
        .player_components
        .progress
        .info
        .get_key_int("newtower:cycle")
        .max(1) as i32;
    Ok(NewTowerLastCycleInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        NewTowerCycleRecordPb: Some(NewTowerCycleRecordPb {
            CycleId: cycle_id,
            ..Default::default()
        }),
    })
}

pub fn handle_new_tower_medal_info_request(
    state: &mut State,
    _req: NewTowerMedalInfoRequest,
) -> Result<NewTowerMedalInfoResponse, String> {
    let pc = &state.player_components.progress.info;
    let score = pc.get_key_int("new_tower:all_score").max(0) as i32;
    let cycle_id = pc.get_key_int("newtower:cycle").max(1) as i32;
    Ok(NewTowerMedalInfoResponse {
        NewTowerMedalInfos: vec![NewTowerMedalInfo {
            MedalGroupId: 1,
            SeasonId: 1,
            CycleId: cycle_id,
            Progress: score,
            CompleteTime: if score > 0 { 1 } else { 0 },
        }],
    })
}

pub fn handle_new_tower_recommend_request(
    state: &mut State,
    _req: NewTowerRecommendRequest,
) -> Result<NewTowerRecommendResponse, String> {
    let roles = owned_role_ids_sorted(state);
    let team_roles: Vec<i32> = roles.into_iter().take(3).collect();
    let mut levels: Vec<NewTowerRecommendLevel> = state
        .assets
        .reward_index
        .get_rows("NewTowerLevel")
        .iter()
        .filter_map(|row| i32::try_from(row.key).ok().filter(|&id| id > 0))
        .take(5)
        .map(|level_id| NewTowerRecommendLevel {
            LevelId: level_id,
            RecommendId: level_id,
            RecommendTeams: vec![NewTowerRecommendTeam {
                RoleIds: team_roles.clone(),
                SkillBranchIds: Vec::new(),
                BuffIds: vec![1],
                UsageRate: 5000,
            }],
        })
        .collect();
    if levels.is_empty() {
        levels.push(NewTowerRecommendLevel {
            LevelId: 1,
            RecommendId: 1,
            RecommendTeams: vec![NewTowerRecommendTeam {
                RoleIds: team_roles,
                SkillBranchIds: Vec::new(),
                BuffIds: vec![1],
                UsageRate: 5000,
            }],
        });
    }
    Ok(NewTowerRecommendResponse {
        ErrorCode: Some(ErrorCode::Success),
        NewTowerRecommendData: Some(NewTowerRecommendData {
            RecommendLevels: levels,
        }),
    })
}

pub fn handle_new_tower_reward_request(
    state: &mut State,
    req: NewTowerRewardRequest,
) -> Result<NewTowerRewardResponse, String> {
    let _ = claim_reward_list(state, "newtower:clm", "NewTowerScoreReward", &req.Indexs)?;
    Ok(NewTowerRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_new_tower_role_energy_request(
    state: &mut State,
    _req: NewTowerRoleEnergyRequest,
) -> Result<NewTowerRoleEnergyResponse, String> {
    let lv = state
        .player_components
        .progress
        .info
        .get_key_int("newtower:cycle")
        .max(1) as i32;
    Ok(NewTowerRoleEnergyResponse {
        ErrorCode: Some(ErrorCode::Success),
        RecordRoleEnergys: vec![RecordRoleEnergy {
            LevelId: lv,
            ..Default::default()
        }],
    })
}

pub fn handle_new_tower_season_reward_request(
    state: &mut State,
    req: NewTowerSeasonRewardRequest,
) -> Result<NewTowerSeasonRewardResponse, String> {
    let _ = claim_reward_list(state, "newtower:sclm", "NewTowerSeasonAward", &req.Indexs)?;
    Ok(NewTowerSeasonRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_new_tower_season_score_reward_request(
    state: &mut State,
    req: NewTowerSeasonScoreRewardRequest,
) -> Result<NewTowerSeasonScoreRewardResponse, String> {
    let _ = claim_reward_list(state, "newtower:ssclm", "NewTowerSeasonSAward", &req.Indexs)?;
    Ok(NewTowerSeasonScoreRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_new_tower_season_task_data_update_request(
    state: &mut State,
    _req: NewTowerSeasonTaskDataUpdateRequest,
) -> Result<NewTowerSeasonTaskDataUpdateResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int("new_tower:task_sync", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    let pc = &state.player_components.progress.info;
    let score = pc.get_key_int("new_tower:all_score").max(0) as i32;
    let mut tasks = Vec::new();
    for row in &state.assets.tables.new_tower_season_award.items {
        let claimed = pc.has_claim(&format!("newtower:sclm:{}", row.Id));
        let status = if claimed {
            ActivityTaskState::ActivityTaskTaken
        } else if score > 0 {
            ActivityTaskState::ActivityTaskFinish
        } else {
            ActivityTaskState::ActivityTaskRunning
        };
        tasks.push(ActivityTask {
            Id: row.Id,
            Current: if claimed || score > 0 { 1 } else { 0 },
            Target: 1,
            Status: Some(status),
            ..Default::default()
        });
    }
    Ok(NewTowerSeasonTaskDataUpdateResponse {
        SeasonTasks: tasks,
    })
}

pub fn handle_new_tower_update_record_request(
    state: &mut State,
    req: NewTowerUpdateRecordRequest,
) -> Result<NewTowerUpdateRecordResponse, String> {
    let score = state
        .player_components
        .progress
        .info
        .get_key_int(&format!("newtower:score:{}", req.LevelId)) as i32;
    Ok(NewTowerUpdateRecordResponse {
        ErrorCode: Some(ErrorCode::Success),
        Record: Some(NewTowerClimbingLevelRecord {
            LevelId: req.LevelId,
            Score: score,
            HistoryScore: score,
            IsUnlock: true,
            ..Default::default()
        }),
    })
}

// === RoleTower ===

pub fn handle_role_tower_attribute_query_request(
    state: &mut State,
    req: RoleTowerAttributeQueryRequest,
) -> Result<RoleTowerAttributeQueryResponse, String> {
    let mut infos = Vec::new();
    let mut max_phantom_level = 0i32;

    for rid in req.RoleIds {
        let mut phantom_infos = Vec::new();
        if let Some(role_info) = state.player_components.role.role_map.get(&rid).cloned() {
            let snapshot = role_stats::build_snapshot(
                &state.assets,
                &mut state.player_components.role,
                &state.player_components.weapon,
                &state.player_components.echo,
                rid,
            )
            .ok();
            let base_props = if let Some(ref snap) = snapshot {
                snap.base_props
                    .iter()
                    .enumerate()
                    .filter(|&(k, &v)| k == 1 || v != 0)
                    .map(|(k, &v)| ArrayIntInt {
                        Key: k as i32,
                        Value: v,
                    })
                    .collect()
            } else {
                vec![ArrayIntInt {
                    Key: 1,
                    Value: role_info.level,
                }]
            };
            let add_props = if let Some(ref snap) = snapshot {
                snap.add_props
                    .iter()
                    .enumerate()
                    .filter(|&(_, &v)| v != 0)
                    .map(|(k, &v)| ArrayIntInt {
                        Key: k as i32,
                        Value: v,
                    })
                    .collect()
            } else {
                Vec::new()
            };
            let phantom_prop = role_stats::build_phantom_display_props(
                &state.assets,
                &mut state.player_components.role,
                &state.player_components.weapon,
                &state.player_components.echo,
                rid,
            )
            .ok()
            .map(|display| RolePhantomPropInfo {
                RoleId: rid,
                BaseProp: display
                    .base_props
                    .into_iter()
                    .map(|(k, v)| ArrayIntInt { Key: k, Value: v })
                    .collect(),
                AddProp: display
                    .add_props
                    .into_iter()
                    .map(|(k, v)| ArrayIntInt { Key: k, Value: v })
                    .collect(),
            });

            let equip = state.player_components.echo.role_equip(rid);
            for (slot_idx, &inc_id) in equip.slots.iter().enumerate() {
                if inc_id == 0 {
                    continue;
                }
                let Some(echo) = state.player_components.echo.echo_map.get(&inc_id) else {
                    continue;
                };
                max_phantom_level = max_phantom_level.max(echo.level);
                let attr_map = compute_echo_attr_map(&state.assets, echo);
                let slot_props: Vec<ArrayIntInt> = attr_map
                    .into_iter()
                    .map(|(k, v)| ArrayIntInt { Key: k, Value: v })
                    .collect();
                phantom_infos.push(RoleTowerPhantomAttributeInfo {
                    PhantomId: echo.id,
                    SlotIndex: slot_idx as i32,
                    TotalAttrs: Some(RolePhantomPropInfo {
                        RoleId: rid,
                        BaseProp: slot_props,
                        AddProp: Vec::new(),
                    }),
                });
            }

            infos.push(RoleTowerRoleAttributeInfo {
                RoleProp: Some(RolePropInfo {
                    RoleId: rid,
                    BaseProp: base_props,
                    AddProp: add_props,
                    PhantomProp: phantom_prop,
                }),
                PhantomInfos: phantom_infos,
            });
        } else {
            let level = state
                .assets
                .tables
                .trial_role_info
                .get_data_by_id(&rid)
                .map_or(1, |t| t.Level.max(1));
            infos.push(RoleTowerRoleAttributeInfo {
                RoleProp: Some(RolePropInfo {
                    RoleId: rid,
                    BaseProp: vec![ArrayIntInt {
                        Key: 1,
                        Value: level,
                    }],
                    ..Default::default()
                }),
                PhantomInfos: phantom_infos,
            });
        }
    }

    let configured_phantom_lv = state
        .player_components
        .progress
        .info
        .get_key_int("role_tower:echo_lv")
        .max(0) as i32;
    let phantom_level = if configured_phantom_lv > 0 {
        configured_phantom_lv
    } else {
        max_phantom_level
    };
    if req.ChallengeId > 0 {
        state.player_components.progress.info.set_key_int(
            "role_tower:cur_challenge",
            i64::from(req.ChallengeId),
        );
    }

    Ok(RoleTowerAttributeQueryResponse {
        ErrorCode: Some(ErrorCode::Success),
        RoleInfos: infos,
        PhantomLevel: phantom_level,
    })
}

pub fn handle_role_tower_challenge_settle_request(
    state: &mut State,
    req: RoleTowerChallengeSettleRequest,
) -> Result<RoleTowerChallengeSettleResponse, String> {
    let total_floors = role_tower_floor_count(&state.assets, req.ChallengeId);
    let info = &mut state.player_components.progress.info;
    let ch_stage_key = format!("role_tower:ch_stage:{}", req.ChallengeId);
    let ch_score_key = format!("role_tower:ch_score:{}", req.ChallengeId);
    let best_floor_key = format!("role_tower:best_floor:{}", req.ChallengeId);
    let best_score_key = format!("role_tower:best_score:{}", req.ChallengeId);

    if req.ChallengeId > 0 {
        info.set_key_int("role_tower:cur_challenge", i64::from(req.ChallengeId));
    }

    let ch_stage = info
        .get_key_int(&ch_stage_key)
        .max(info.get_key_int("role_tower:stage"))
        .max(0);
    let ch_score = info
        .get_key_int(&ch_score_key)
        .max(info.get_key_int("role_tower:score"))
        .max(0);

    let prev_best_floor = info.get_key_int(&best_floor_key).max(0);
    let prev_best_score = info.get_key_int(&best_score_key).max(0);
    let is_new_record = ch_stage > prev_best_floor || ch_score > prev_best_score;
    let best_floor = prev_best_floor.max(ch_stage);
    let best_score = prev_best_score.max(ch_score);

    info.set_key_int(&best_floor_key, best_floor);
    info.set_key_int(&best_score_key, best_score);
    let prev_global_score = info.get_key_int("role_tower:score").max(0);
    if ch_score > prev_global_score {
        info.set_key_int("role_tower:score", ch_score);
    }
    let prev_global_stage = info.get_key_int("role_tower:stage").max(0);
    if ch_stage > prev_global_stage {
        info.set_key_int("role_tower:stage", ch_stage);
    }
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;

    let is_all_passed = if total_floors > 0 {
        ch_stage >= i64::from(total_floors)
    } else {
        ch_stage > 0
    };
    let best_all_passed = if total_floors > 0 {
        best_floor >= i64::from(total_floors)
    } else {
        best_floor > 0
    };
    let stars = if is_all_passed {
        3
    } else if ch_stage > 0 {
        1
    } else {
        0
    };

    let mut events = EventQueue::new();
    {
        let mut qctx = state.quest_context(&mut events);
        if let Some(mut lp_ctx) = level_play::LevelPlayContext::from_quest_ctx(&mut qctx) {
            let _ = level_play::on_condition_event(
                &mut lp_ctx,
                LevelPlayEventKind::FinishRoleTower,
                0,
                ch_stage as i32,
            );
        }
        let _ = quest_engine::events::on_tower_floor_event(&mut qctx, ch_stage as i32, stars);
    }
    drain_event_queue_best_effort(&mut events, state);

    let now_s = energy::now_seconds();
    let cur_rec = RoleTowerBestRecord {
        ChallengeId: req.ChallengeId,
        PassedFloorCount: ch_stage as i32,
        SettleTime: now_s,
        IsAllPassed: is_all_passed,
        IsNewRecord: is_new_record,
        ..Default::default()
    };
    let best_rec = RoleTowerBestRecord {
        ChallengeId: req.ChallengeId,
        PassedFloorCount: best_floor as i32,
        SettleTime: now_s,
        IsAllPassed: best_all_passed,
        IsNewRecord: is_new_record,
        ..Default::default()
    };
    Ok(RoleTowerChallengeSettleResponse {
        ErrorCode: Some(ErrorCode::Success),
        ChallengeSettleInfo: Some(RoleTowerChallengeSettleInfo {
            ChallengeId: req.ChallengeId,
            CurrentRecord: Some(cur_rec),
            BestRecord: Some(best_rec),
            IsBestRecordUpdated: is_new_record,
            ..Default::default()
        }),
    })
}

pub fn handle_role_tower_consume_item_request(
    state: &mut State,
    req: RoleTowerConsumeItemRequest,
) -> Result<RoleTowerConsumeItemResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int(&format!("roletower:item:{}", req.ItemId), 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RoleTowerConsumeItemResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_role_tower_phantom_add_attrs_request(
    state: &mut State,
    req: RoleTowerPhantomAddAttrsRequest,
) -> Result<RoleTowerPhantomAddAttrsResponse, String> {
    let mut attr_infos = Vec::new();
    for &rid in &req.RoleIds {
        let equip = state.player_components.echo.role_equip(rid);
        for (slot_idx, &inc_id) in equip.slots.iter().enumerate() {
            if inc_id == 0 {
                continue;
            }
            let Some(echo) = state.player_components.echo.echo_map.get(&inc_id) else {
                continue;
            };
            let attrs = compute_echo_attr_map(&state.assets, echo);
            if !attrs.is_empty() {
                attr_infos.push(RoleTowerPhantomAddAttr {
                    PhantomId: echo.id,
                    SlotIndex: slot_idx as i32,
                    Attrs: attrs,
                    RoleIdOrTrialRoleId: rid,
                });
            }
        }
    }
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("role_tower:attr_cnt:{}", req.ChallengeId),
        req.RoleIds.len() as i64,
    )
    .map_err(|e| e.to_string())?;
    Ok(RoleTowerPhantomAddAttrsResponse {
        PhantomAddAttrInfos: attr_infos,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_role_tower_rank_request(
    state: &mut State,
    _req: RoleTowerRankRequest,
) -> Result<RoleTowerRankResponse, String> {
    let floor = state
        .player_components
        .progress
        .info
        .get_key_int("role_tower:stage")
        .max(0) as i32;
    let mut peer_entries: Vec<RoleTowerRankEntry> = CoopService::collect_server_player_snapshots(
        &state.fs,
        state.coop_service.as_deref(),
        state.player_id,
    )
    .into_iter()
    .filter_map(|p| {
        let p_floor = p.progress.get_key_int("role_tower:stage").max(0) as i32;
        if p_floor <= 0 {
            return None;
        }
        Some(RoleTowerRankEntry {
            PlayerId: p.player_id,
            Name: p.name.into_bytes(),
            Rank: 0,
            RankPercent: 1,
            Record: Some(RoleTowerBestRecord {
                ChallengeId: 1,
                PassedFloorCount: p_floor,
                IsAllPassed: p_floor > 0,
                ..Default::default()
            }),
            ..Default::default()
        })
    })
    .collect();
    peer_entries.sort_by(|a, b| {
        let fa = a.Record.as_ref().map_or(0, |r| r.PassedFloorCount);
        let fb = b.Record.as_ref().map_or(0, |r| r.PassedFloorCount);
        fb.cmp(&fa)
    });
    for (idx, e) in peer_entries.iter_mut().enumerate() {
        e.Rank = (idx + 1) as i32;
    }
    let self_rank = peer_entries
        .iter()
        .filter(|e| e.Record.as_ref().map_or(0, |r| r.PassedFloorCount) > floor)
        .count() as i32
        + 1;
    let self_entry = RoleTowerRankEntry {
        PlayerId: state.player_id,
        Name: state.player_components.basic.info.attributes.name.as_bytes().to_vec(),
        Rank: self_rank,
        RankPercent: 1,
        Record: Some(RoleTowerBestRecord {
            ChallengeId: 1,
            PassedFloorCount: floor,
            IsAllPassed: floor > 0,
            ..Default::default()
        }),
        ..Default::default()
    };
    Ok(RoleTowerRankResponse {
        ErrorCode: Some(ErrorCode::Success),
        RankInfo: vec![RoleTowerRankInfo {
            ChallengeId: 1,
            FriendEntries: peer_entries,
            SelfServerEntry: Some(self_entry),
            CanGetServerRankReward: floor > 0,
            ..Default::default()
        }],
    })
}

pub fn handle_role_tower_rank_reward_receive_request(
    state: &mut State,
    req: RoleTowerRankRewardReceiveRequest,
) -> Result<RoleTowerRankRewardReceiveResponse, String> {
    let _ = claim_reward_list(
        state,
        "roletower:rank",
        "RoleTowerRankReward",
        &req.ChallengeIds,
    )?;
    Ok(RoleTowerRankRewardReceiveResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_role_tower_reward_task_receive_request(
    state: &mut State,
    req: RoleTowerRewardTaskReceiveRequest,
) -> Result<RoleTowerRewardTaskReceiveResponse, String> {
    let _ = claim_reward_list(state, "roletower:task", "RoleTowerTask", &req.TaskIds)?;
    Ok(RoleTowerRewardTaskReceiveResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_role_tower_save_delete_request(
    state: &mut State,
    req: RoleTowerSaveDeleteRequest,
) -> Result<RoleTowerSaveDeleteResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("roletower:step:{}", req.ChallengeId),
        0,
    )
    .map_err(|e| e.to_string())?;
    Ok(RoleTowerSaveDeleteResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_role_tower_score_reward_receive_request(
    state: &mut State,
    req: RoleTowerScoreRewardReceiveRequest,
) -> Result<RoleTowerScoreRewardReceiveResponse, String> {
    let _ = claim_reward_list(state, "roletower:score", "RoleTowerScoreReward", &req.RewardIds)?;
    Ok(RoleTowerScoreRewardReceiveResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_role_tower_stage_retry_request(
    state: &mut State,
    req: RoleTowerStageRetryRequest,
) -> Result<RoleTowerStageRetryResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int(&format!("role_tower:retry:{}", req.ChallengeId), 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RoleTowerStageRetryResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_role_tower_stage_settle_request(
    state: &mut State,
    _req: RoleTowerStageSettleRequest,
) -> Result<RoleTowerStageSettleResponse, String> {
    let cur_inst = state.scene.as_ref().map_or(0, |s| s.instance_id);
    let cur_challenge = {
        let saved = state
            .player_components
            .progress
            .info
            .get_key_int("role_tower:cur_challenge")
            .max(0) as i32;
        if saved > 0 {
            saved
        } else if cur_inst > 0 {
            state
                .assets
                .raw_json_cached("RoleTowerLevel")
                .iter()
                .find(|row| json_i32(row, "InstId") == cur_inst)
                .map_or(0, |row| json_i32(row, "Id"))
        } else {
            0
        }
    };
    let total_floors = if cur_challenge > 0 {
        role_tower_floor_count(&state.assets, cur_challenge)
    } else {
        0
    };

    let info = &mut state.player_components.progress.info;
    let cur_stage = if cur_challenge > 0 {
        info.get_key_int(&format!("role_tower:ch_stage:{cur_challenge}"))
            .max(0)
    } else {
        info.get_key_int("role_tower:stage").max(0)
    };
    let next_stage = if total_floors > 0 {
        (cur_stage + 1).min(i64::from(total_floors))
    } else {
        cur_stage + 1
    };
    if cur_challenge > 0 {
        info.set_key_int(&format!("role_tower:ch_stage:{cur_challenge}"), next_stage);
    }
    let prev_global = info.get_key_int("role_tower:stage").max(0);
    if next_stage > prev_global {
        info.set_key_int("role_tower:stage", next_stage);
    }
    let newly_cleared = info.mark_claim(&format!("role_tower:stage:{next_stage}"));
    let stage_score = info.get_key_int("role_tower:cur_stage_score").max(0);
    if stage_score > 0 || newly_cleared {
        let delta = if stage_score > 0 { stage_score } else { 500 };
        if cur_challenge > 0 {
            info.add_key_int(&format!("role_tower:ch_score:{cur_challenge}"), delta);
        }
        info.add_key_int("role_tower:score", delta);
    }
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    let mut events = EventQueue::new();
    {
        let mut qctx = state.quest_context(&mut events);
        if let Some(mut lp_ctx) = level_play::LevelPlayContext::from_quest_ctx(&mut qctx) {
            let _ = level_play::on_condition_event(
                &mut lp_ctx,
                LevelPlayEventKind::FinishRoleTower,
                0,
                next_stage as i32,
            );
        }
        let _ = quest_engine::events::on_tower_floor_event(&mut qctx, next_stage as i32, 3);
    }
    drain_event_queue_best_effort(&mut events, state);
    Ok(RoleTowerStageSettleResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_role_tower_step_advance_request(
    state: &mut State,
    _req: RoleTowerStepAdvanceRequest,
) -> Result<RoleTowerStepAdvanceResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int("roletower:steps", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    let mut events = EventQueue::new();
    {
        let mut qctx = state.quest_context(&mut events);
        if let Some(mut lp_ctx) = level_play::LevelPlayContext::from_quest_ctx(&mut qctx) {
            let _ = level_play::on_condition_event(
                &mut lp_ctx,
                LevelPlayEventKind::FinishRoleTower,
                0,
                0,
            );
        }
    }
    drain_event_queue_best_effort(&mut events, state);
    Ok(RoleTowerStepAdvanceResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

// === Abyss ===

pub fn handle_abyss_challenge_goto_next_room_request(
    state: &mut State,
    req: AbyssChallengeGotoNextRoomRequest,
) -> Result<AbyssChallengeGotoNextRoomResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "abyss:room",
        i64::from(req.RoomId),
    )
    .map_err(|e| e.to_string())?;
    Ok(AbyssChallengeGotoNextRoomResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_abyss_challenge_quit_request(
    state: &mut State,
    _req: AbyssChallengeQuitRequest,
) -> Result<AbyssChallengeQuitResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "abyss:room",
        0,
    )
    .map_err(|e| e.to_string())?;
    Ok(AbyssChallengeQuitResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_abyss_challenge_unlock_request(
    state: &mut State,
    req: AbyssChallengeUnlockRequest,
) -> Result<AbyssChallengeUnlockResponse, String> {
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        &format!("abyss:unl:{}", req.ChallengeId),
    )
    .map_err(|e| e.to_string())?;
    Ok(AbyssChallengeUnlockResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_abyss_like_request(
    state: &mut State,
    req: AbyssLikeRequest,
) -> Result<AbyssLikeResponse, String> {
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        &format!("abyss:like:{}", req.PlayerId),
    )
    .map_err(|e| e.to_string())?;
    Ok(AbyssLikeResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_abyss_little_role_level_up_request(
    state: &mut State,
    req: AbyssLittleRoleLevelUpRequest,
) -> Result<AbyssLittleRoleLevelUpResponse, String> {
    if req.Id <= 0
        || req.Level <= 0
        || state
            .assets
            .tables
            .abyss_little_role
            .get_data_by_id(&req.Id)
            .is_none()
    {
        return Ok(AbyssLittleRoleLevelUpResponse {
            ErrorCode: Some(ErrorCode::ErrAbyssRoleLevelConfig),
        });
    }
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("abyss:lrole:{}", req.Id),
        i64::from(req.Level.max(1)),
    )
    .map_err(|e| e.to_string())?;
    let mut events = EventQueue::new();
    {
        let mut qctx = state.quest_context(&mut events);
        let _ = quest_engine::on_condition_state_changed(&mut qctx);
    }
    drain_event_queue_best_effort(&mut events, state);
    Ok(AbyssLittleRoleLevelUpResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_abyss_plugin_put_request(
    state: &mut State,
    req: AbyssPluginPutRequest,
) -> Result<AbyssPluginPutResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("abyss:plugin_cnt:{}", req.LittleRoleId),
        req.EquipItems.len() as i64,
    )
    .map_err(|e| e.to_string())?;
    Ok(AbyssPluginPutResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_abyss_plugin_synthesis_request(
    state: &mut State,
    req: AbyssPluginSynthesisRequest,
) -> Result<AbyssPluginSynthesisResponse, String> {
    let mut seen_incrs = HashSet::new();
    let mut consumed_incrs = Vec::new();
    let mut outputs: BTreeMap<i32, i32> = BTreeMap::new();
    let mut inv_dirty = false;

    for &incr_id in &req.PluginItemIncrId {
        if incr_id <= 0 || !seen_incrs.insert(incr_id) {
            continue;
        }
        let track_key = format!("abyss:plugin_item:{incr_id}");
        let tracked_item_id = state
            .player_components
            .progress
            .info
            .get_key_int(&track_key) as i32;
        let item_id = if tracked_item_id > 0 {
            state
                .player_components
                .progress
                .info
                .set_key_int(&track_key, 0);
            consumed_incrs.push(incr_id);
            tracked_item_id
        } else if state
            .assets
            .raw_json_cached("AbyssItem")
            .iter()
            .any(|row| json_i32(row, "Id") == incr_id)
            && state
                .player_components
                .inventory
                .info
                .consume_normal_item(incr_id, 1)
        {
            inv_dirty = true;
            consumed_incrs.push(incr_id);
            incr_id
        } else {
            continue;
        };

        let quality = state
            .assets
            .raw_json_cached("AbyssItem")
            .iter()
            .find(|row| json_i32(row, "Id") == item_id)
            .map_or(1, |row| json_i32(row, "QualityId").max(1));

        let synth_rows = state.assets.raw_json_cached("AbyssSynthesis");
        let synth_row = synth_rows
            .iter()
            .find(|row| json_i32(row, "Quality") == quality)
            .or_else(|| synth_rows.first());
        if let Some(decomp) = synth_row.and_then(|r| r.get("DecomposeInfo")).and_then(Value::as_object)
        {
            for (k, v) in decomp {
                if let (Ok(out_id), Some(cnt)) = (k.parse::<i32>(), v.as_i64())
                    && out_id > 0
                    && cnt > 0
                {
                    *outputs.entry(out_id).or_default() += cnt as i32;
                }
            }
        }
    }

    if !consumed_incrs.is_empty() {
        state
            .player_components
            .progress
            .info
            .add_key_int("abyss:plugin_synth", consumed_incrs.len() as i64);
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        let _ = state.conn.push_msg(
            AbyssPluginItemRemoveNotify::MSG_ID,
            &AbyssPluginItemRemoveNotify {
                IncrIds: consumed_incrs,
            },
        );
    }
    if inv_dirty {
        state
            .player_components
            .inventory
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }

    let grant_items_list: Vec<reward::RewardItem> = outputs
        .iter()
        .map(|(&item_id, &count)| reward::RewardItem { item_id, count })
        .collect();
    if !grant_items_list.is_empty() {
        let mut rctx = state.reward_context();
        reward::grant_items(&mut rctx, 0, true, &grant_items_list, 19500)
            .map_err(|e| e.to_string())?;
    }

    let out = outputs
        .into_iter()
        .map(|(id, count)| AddCountItemInfo {
            Id: id,
            Count: count,
            IncrId: 0,
        })
        .collect();
    Ok(AbyssPluginSynthesisResponse {
        ErrorCode: Some(ErrorCode::Success),
        OutputItems: out,
    })
}

pub fn handle_abyss_rank_list_request(
    state: &mut State,
    _req: AbyssRankListRequest,
) -> Result<AbyssRankListResponse, String> {
    let pass_wave = state
        .player_components
        .progress
        .info
        .get_key_int("abyss:room")
        .max(0) as i32;
    let pinfo = AbyssChallengePassPlayerInfo {
        PlayerId: state.player_id,
        Name: state
            .player_components
            .basic
            .info
            .attributes
            .name
            .as_bytes()
            .to_vec(),
        ..Default::default()
    };
    let mut friend_pass: Vec<AbyssChallengeInfo> = CoopService::collect_server_player_snapshots(
        &state.fs,
        state.coop_service.as_deref(),
        state.player_id,
    )
    .into_iter()
    .filter_map(|p| {
        let pw = p.progress.get_key_int("abyss:room").max(0) as i32;
        if pw <= 0 {
            return None;
        }
        Some(AbyssChallengeInfo {
            PlayerInfos: vec![AbyssChallengePassPlayerInfo {
                PlayerId: p.player_id,
                Name: p.name.into_bytes(),
                ..Default::default()
            }],
            ChallengeId: 1,
            PassWave: pw,
            IsSingle: true,
            OwnerId: p.player_id,
            ..Default::default()
        })
    })
    .collect();
    friend_pass.sort_by_key(|a| std::cmp::Reverse(a.PassWave));
    Ok(AbyssRankListResponse {
        ErrorCode: Some(ErrorCode::Success),
        FriendPassData: friend_pass,
        SelfPassData: vec![AbyssChallengeInfo {
            PlayerInfos: vec![pinfo],
            ChallengeId: 1,
            PassWave: pass_wave,
            IsSingle: true,
            OwnerId: state.player_id,
            ..Default::default()
        }],
    })
}

pub fn handle_abyss_revive_character_request(
    state: &mut State,
    _req: AbyssReviveCharacterRequest,
) -> Result<AbyssReviveCharacterResponse, String> {
    let rev_cnt = state
        .player_components
        .progress
        .info
        .add_key_int("abyss:revive_cnt", 1) as i32;
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    let _ = state.conn.push_msg(
        AbyssReviveNotify::MSG_ID,
        &AbyssReviveNotify {
            MaxReviveTimes: 3,
            CurReviveTimes: rev_cnt,
            ..Default::default()
        },
    );
    Ok(AbyssReviveCharacterResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_abyss_reward_request(
    state: &mut State,
    req: AbyssRewardRequest,
) -> Result<AbyssRewardResponse, String> {
    if !req.Ids.is_empty() {
        let _ = claim_reward_list(state, "abyss:clm", "AbyssReward", &req.Ids)?;
    } else if req.Id > 0 {
        let _ = claim_reward_list(state, "abyss:clm", "AbyssReward", &[req.Id])?;
    }
    Ok(AbyssRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_abyss_self_pass_request(
    state: &mut State,
    req: AbyssSelfPassRequest,
) -> Result<AbyssSelfPassResponse, String> {
    let mut list = Vec::new();
    if state
        .player_components
        .progress
        .info
        .has_claim(&format!("abyss:unl:{}", req.Id))
    {
        list.push(AbyssChallengeInfo {
            ChallengeId: req.Id,
            IsSingle: true,
            ..Default::default()
        });
    }
    Ok(AbyssSelfPassResponse {
        SelfPassData: list,
    })
}

pub fn handle_abyss_show_name_request(
    state: &mut State,
    req: AbyssShowNameRequest,
) -> Result<AbyssShowNameResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "abyss:showname",
        i64::from(req.ShowName),
    )
    .map_err(|e| e.to_string())?;
    Ok(AbyssShowNameResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

// === BossRush & BossPiling ===

pub fn handle_boss_rush_batch_claim_reward_request(
    state: &mut State,
    req: BossRushBatchClaimRewardRequest,
) -> Result<BossRushBatchClaimRewardResponse, String> {
    let _ = claim_reward_list(
        state,
        "bossrush:task",
        "BossRushTaskConfig",
        &req.TaskRewardIds,
    )?;
    Ok(BossRushBatchClaimRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_boss_rush_buff_choose_result_request(
    state: &mut State,
    req: BossRushBuffChooseResultRequest,
) -> Result<BossRushBuffChooseResultResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "bossrush:buff_idx",
        i64::from(req.Index),
    )
    .map_err(|e| e.to_string())?;
    Ok(BossRushBuffChooseResultResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_boss_rush_choose_buff_request(
    state: &mut State,
    _req: BossRushChooseBuffRequest,
) -> Result<BossRushChooseBuffResponse, String> {
    let pick_cnt = state
        .player_components
        .progress
        .info
        .add_key_int("bossrush:buff_pick", 1);
    let mut buff_ids = Vec::new();
    let len = state.assets.tables.boss_rush_buff.items.len();
    if len > 0 {
        let count = 3.min(len);
        let start = ((pick_cnt - 1).max(0) as usize * 3) % len;
        for i in 0..count {
            let bid = state.assets.tables.boss_rush_buff.items[(start + i) % len].Id;
            buff_ids.push(bid);
            state
                .player_components
                .progress
                .info
                .set_key_int(&format!("bossrush:offer:{i}"), i64::from(bid));
        }
    }
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(BossRushChooseBuffResponse {
        ErrorCode: Some(ErrorCode::Success),
        BuffIds: buff_ids,
    })
}

pub fn handle_boss_rush_claim_reward_request(
    state: &mut State,
    req: BossRushClaimRewardRequest,
) -> Result<BossRushClaimRewardResponse, String> {
    match req.RewardType.unwrap_or(BossRushRewardType::Level) {
        BossRushRewardType::Score => {
            let _ = claim_reward_list(state, "bossrush:lscore", "BossRushScore", &[req.RewardId])?;
        }
        BossRushRewardType::Level => {
            let _ = claim_reward_list(state, "bossrush:clm", "BossRushActivity", &[req.RewardId])?;
        }
    }
    Ok(BossRushClaimRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_boss_rush_level_score_reward_request(
    state: &mut State,
    req: BossRushLevelScoreRewardRequest,
) -> Result<BossRushLevelScoreRewardResponse, String> {
    let _ = claim_reward_list(state, "bossrush:lscore", "BossRushScore", &[req.Index])?;
    Ok(BossRushLevelScoreRewardResponse {
        Error: Some(ErrorCode::Success),
    })
}

pub fn handle_boss_rush_settlement_request(
    state: &mut State,
    _req: BossRushSettlementRequest,
) -> Result<BossRushSettlementResponse, String> {
    let scene_inst = state
        .scene
        .as_ref()
        .map(|s| s.instance_id)
        .filter(|&id| find_boss_rush_row(&state.assets, id).is_some())
        .unwrap_or(0);
    let stored_inst = state
        .player_components
        .progress
        .info
        .get_key_int("bossrush:inst")
        .max(0) as i32;
    let inst_id = if stored_inst > 0 {
        stored_inst
    } else {
        scene_inst
    };

    let row_info = find_boss_rush_row(&state.assets, inst_id);
    let stored_act = state
        .player_components
        .progress
        .info
        .get_key_int("bossrush:act")
        .max(0) as i32;
    let activity_id = if stored_act > 0 {
        stored_act
    } else {
        row_info.map_or(0, |r| r.0)
    };

    let cur_score = state
        .player_components
        .progress
        .info
        .get_key_int("bossrush:score")
        .max(0);
    let run_score = state
        .player_components
        .progress
        .info
        .get_key_int("bossrush:run_score")
        .max(
            state
                .player_components
                .progress
                .info
                .get_key_int("bossrush:cur_score"),
        )
        .max(0);
    let new_score = if run_score > 0 {
        run_score
    } else if let Some((_, reward_score)) = row_info.filter(|&(_, s)| s > 0) {
        i64::from(reward_score).max(cur_score)
    } else {
        state
            .assets
            .tables
            .boss_rush_score
            .items
            .iter()
            .map(|cfg| i64::from(cfg.Score))
            .filter(|&tier_score| tier_score > cur_score)
            .min()
            .unwrap_or_else(|| cur_score.max(1500))
    };

    let info = &mut state.player_components.progress.info;
    let effective_score = cur_score.max(new_score);
    info.set_key_int("bossrush:score", effective_score);
    if inst_id > 0 {
        let inst_key = format!("bossrush:score:{inst_id}");
        let prev_inst = info.get_key_int(&inst_key).max(0);
        info.set_key_int(&inst_key, prev_inst.max(new_score));
    }
    let prev_high = info.get_key_int("bossrush:high_score").max(0);
    let high_score = prev_high.max(effective_score);
    info.set_key_int("bossrush:high_score", high_score);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    let total_i32 = new_score.clamp(0, i64::from(i32::MAX)) as i32;
    let kill_score = total_i32 / 2;
    let timer_score = total_i32 / 4;
    let tech_score = total_i32 - kill_score - timer_score;
    let _ = state.conn.push_msg(
        BossRushResultNotify::MSG_ID,
        &BossRushResultNotify {
            ActivityId: activity_id,
            InstId: inst_id,
            KilMonsterScore: kill_score,
            TimerScore: timer_score,
            TechnicalScore: tech_score,
            HighScore: high_score.clamp(0, i64::from(i32::MAX)) as i32,
            LeftTime: 60,
        },
    );
    Ok(BossRushSettlementResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_boss_rush_task_reward_request(
    state: &mut State,
    req: BossRushTaskRewardRequest,
) -> Result<BossRushTaskRewardResponse, String> {
    let _ = claim_reward_list(state, "bossrush:task", "BossRushTaskConfig", &[req.TaskId])?;
    Ok(BossRushTaskRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_boss_piling_reward_request(
    state: &mut State,
    req: BossPilingRewardRequest,
) -> Result<BossPilingRewardResponse, String> {
    let _ = claim_reward_list(
        state,
        "bosspiling:task",
        "BossPilingTask",
        &req.BossPilingTaskIds,
    )?;
    Ok(BossPilingRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_boss_piling_settle_request(
    state: &mut State,
    _req: BossPilingSettleRequest,
) -> Result<BossPilingSettleResponse, String> {
    let cur_inst = state.scene.as_ref().map_or(0, |s| s.instance_id);
    let saved_id = state
        .player_components
        .progress
        .info
        .get_key_int("bp:level_id")
        .max(0) as i32;
    let level_id = if saved_id > 0 {
        saved_id
    } else if cur_inst > 0 {
        state
            .assets
            .boss_piling_levels()
            .iter()
            .find(|l| l.InstIds.contains(&cur_inst))
            .map_or(0, |l| l.Id)
    } else {
        0
    };

    let scene_hp_stats = state.scene.as_ref().and_then(|scene| {
        let mut total_max = 0i64;
        let mut total_cur = 0i64;
        for ent in &scene.entities {
            if ent.config.entity_type != EntityType::Monster {
                continue;
            }
            let Some(attr) = ent.attribute.as_ref() else {
                continue;
            };
            let max_hp = attributes::get_attr_value(attr, EAttributeType::LifeMax);
            let cur_hp = attributes::get_attr_value(attr, EAttributeType::Life).max(0);
            if max_hp > 0 {
                total_max += i64::from(max_hp);
                total_cur += i64::from(cur_hp.min(max_hp));
            }
        }
        (total_max > 0).then(|| {
            let cleared_pct = ((total_max - total_cur) * 100 / total_max).clamp(0, 100);
            (cleared_pct, total_max - total_cur)
        })
    });

    let info = &mut state.player_components.progress.info;
    let recorded_hp = info.get_key_int("bp:cur_hp").max(0);
    let settled_hp = scene_hp_stats
        .map(|(pct, _)| pct)
        .filter(|&p| p > 0)
        .or_else(|| (recorded_hp > 0).then_some(recorded_hp))
        .unwrap_or_else(|| {
            if level_id > 0 {
                info.get_key_int(&format!("bp:hp:{level_id}")).max(0)
            } else {
                0
            }
        });
    let cur_score = info.get_key_int("bp:cur_score").max(0);
    let score_gain = if cur_score > 0 {
        cur_score
    } else {
        scene_hp_stats.map_or(0, |(_, dmg)| dmg)
    };

    if level_id > 0 {
        let prev_hp = info.get_key_int(&format!("bp:hp:{level_id}")).max(0);
        let best_hp = prev_hp.max(settled_hp);
        info.set_key_int(&format!("bp:hp:{level_id}"), best_hp);
        info.set_key_int(&format!("bpiling:hp:{level_id}"), best_hp);
    }
    let prev_max_hp = info.get_key_int("bp:max_hp").max(0);
    info.set_key_int("bp:max_hp", prev_max_hp.max(settled_hp));
    if score_gain > 0 {
        info.add_key_int("bp:score", score_gain);
    }
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(BossPilingSettleResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

// === BabelTower & EndLessHistory ===

pub fn handle_babel_tower_daily_task_reward_request(
    state: &mut State,
    req: BabelTowerDailyTaskRewardRequest,
) -> Result<BabelTowerDailyTaskRewardResponse, String> {
    let _ = claim_reward_list(state, "babel:daily", "BabelTowerDailyTask", &[req.TaskId])?;
    Ok(BabelTowerDailyTaskRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_babel_tower_rank_list_request(
    state: &mut State,
    _req: BabelTowerRankListRequest,
) -> Result<BabelTowerRankListResponse, String> {
    let peers: Vec<PublicBabelTowerInfo> = CoopService::collect_server_player_snapshots(
        &state.fs,
        state.coop_service.as_deref(),
        state.player_id,
    )
    .into_iter()
    .filter(|p| p.progress.get_key_int("babel:finish") > 0 || p.progress.get_key_int("babel:star") > 0)
    .map(|p| PublicBabelTowerInfo {
        PlayerId: p.player_id,
        Name: p.name.into_bytes(),
        HeadId: p.head_id,
        ..Default::default()
    })
    .collect();
    Ok(BabelTowerRankListResponse {
        ErrorCode: Some(ErrorCode::Success),
        SelfPassData: Some(PublicBabelTowerInfo {
            PlayerId: state.player_id,
            Name: state
                .player_components
                .basic
                .info
                .attributes
                .name
                .as_bytes()
                .to_vec(),
            HeadId: state.player_components.basic.info.attributes.head_photo,
            ..Default::default()
        }),
        FriendPassData: peers,
    })
}

pub fn handle_babel_tower_self_pass_request(
    state: &mut State,
    _req: BabelTowerSelfPassRequest,
) -> Result<BabelTowerSelfPassResponse, String> {
    Ok(BabelTowerSelfPassResponse {
        ErrorCode: Some(ErrorCode::Success),
        SelfPassData: Some(PublicBabelTowerInfo {
            PlayerId: state.player_id,
            Name: state
                .player_components
                .basic
                .info
                .attributes
                .name
                .as_bytes()
                .to_vec(),
            HeadId: state.player_components.basic.info.attributes.head_photo,
            ..Default::default()
        }),
    })
}

pub fn handle_babel_tower_settlement_request(
    state: &mut State,
    _req: BabelTowerSettlementRequest,
) -> Result<BabelTowerSettlementResponse, String> {
    let cur_inst = state.scene.as_ref().map_or(0, |s| s.instance_id);
    let saved_lid = state
        .player_components
        .progress
        .info
        .get_key_int("babel:cur_level")
        .max(
            state
                .player_components
                .progress
                .info
                .get_key_int("babel:level_id"),
        )
        .max(0) as i32;
    let babel_rows = state.assets.raw_json_cached("BabelTowerLevel");
    let level_row = babel_rows
        .iter()
        .find(|row| {
            let id = json_i32(row, "Id");
            (saved_lid > 0 && id == saved_lid)
                || (cur_inst > 0 && json_i32(row, "InstId") == cur_inst)
        });
    let level_id = level_row.map_or(saved_lid, |row| json_i32(row, "Id"));
    let cfg_pass_star = level_row.map_or(0, |row| json_i32(row, "PassStar").max(0));

    let has_living_boss = state.scene.as_ref().is_some_and(|scene| {
        scene.entities.iter().any(|ent| {
            ent.config.entity_type == EntityType::Monster
                && !ent.dead_notified
                && ent.attribute.as_ref().is_some_and(|a| {
                    attributes::get_attr_value(a, EAttributeType::Life) > 0
                })
        })
    });

    let info = &mut state.player_components.progress.info;
    let cur_pass_flag = info.get_key_int("babel:cur_pass");
    let is_pass = if cur_pass_flag != 0 {
        cur_pass_flag > 0
    } else {
        !has_living_boss
    };
    let cur_star = info.get_key_int("babel:cur_star").max(0) as i32;
    let stars = if !is_pass {
        0
    } else if cur_star > 0 {
        cur_star
    } else if cfg_pass_star > 0 {
        cfg_pass_star
    } else {
        info.get_key_int("babel:determ_cnt").max(0) as i32
    };
    let start_ms = info.get_key_int("babel:start_ms");
    let use_time = if start_ms > 0 {
        let now_s = energy::now_seconds();
        (now_s - start_ms / 1000).clamp(1, i64::from(i32::MAX)) as i32
    } else {
        0
    };

    if is_pass {
        if level_id > 0 {
            info.mark_claim(&format!("babel:done:{level_id}"));
            let star_key = format!("babel:star:{level_id}");
            let prev_star = info.get_key_int(&star_key).max(0);
            info.set_key_int(&star_key, prev_star.max(i64::from(stars)));
            let total_stars: i64 = info
                .dynamic
                .ints
                .iter()
                .filter(|e| e.key.starts_with("babel:star:"))
                .map(|e| e.val.max(0))
                .sum();
            let total_done = info
                .dynamic
                .claims
                .iter()
                .filter(|c| c.starts_with("babel:done:"))
                .count() as i64;
            info.set_key_int("babel:star", total_stars);
            info.set_key_int("babel:finish", total_done.max(1));
        } else {
            info.add_key_int("babel:finish", 1);
            if stars > 0 {
                info.add_key_int("babel:star", i64::from(stars));
            }
        }
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }

    let _ = state.conn.push_msg(
        BabelActivitySettleInfoNotify::MSG_ID,
        &BabelActivitySettleInfoNotify {
            IsPass: is_pass,
            Star: stars,
            PassTime: i64::from(use_time),
            UseTime: use_time,
            IsActivityClose: false,
            ..Default::default()
        },
    );
    Ok(BabelTowerSettlementResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_babel_tower_show_name_request(
    state: &mut State,
    req: BabelTowerShowNameRequest,
) -> Result<BabelTowerShowNameResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "babel:showname",
        i64::from(req.ShowName),
    )
    .map_err(|e| e.to_string())?;
    Ok(BabelTowerShowNameResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_babel_tower_task_reward_request(
    state: &mut State,
    req: BabelTowerTaskRewardRequest,
) -> Result<BabelTowerTaskRewardResponse, String> {
    if !req.TaskIds.is_empty() {
        let _ = claim_reward_list(state, "babel:task", "BabelTowerTask", &req.TaskIds)?;
    } else if req.TaskId > 0 {
        let _ = claim_reward_list(state, "babel:task", "BabelTowerTask", &[req.TaskId])?;
    }
    Ok(BabelTowerTaskRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_reset_babel_tower_level_request(
    state: &mut State,
    _req: ResetBabelTowerLevelRequest,
) -> Result<ResetBabelTowerLevelResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "babel:cur_level",
        0,
    )
    .map_err(|e| e.to_string())?;
    Ok(ResetBabelTowerLevelResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_select_babel_activity_de_term_request(
    state: &mut State,
    _req: SelectBabelActivityDeTermRequest,
) -> Result<SelectBabelActivityDeTermResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int("babel:determ_cnt", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(SelectBabelActivityDeTermResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_end_less_history_request(
    state: &mut State,
    _req: EndLessHistoryRequest,
) -> Result<EndLessHistoryResponse, String> {
    let now_s = energy::now_seconds();
    let pc = &state.player_components.progress.info;
    let mut best_pair = -1i64;
    let mut best_id: i32 = 1;
    let mut best_s1: i32 = 0;
    let mut best_s2: i32 = 0;
    for entry in &pc.dynamic.ints {
        let Some(rest) = entry.key.strip_prefix("slash:s1:") else {
            continue;
        };
        let Ok(id) = rest.parse::<i32>() else {
            continue;
        };
        let s1 = entry.val.max(0);
        let s2 = pc.get_key_int(&format!("slash:s2:{id}")).max(0);
        if s1 + s2 > best_pair {
            best_pair = s1 + s2;
            best_id = id;
            best_s1 = s1 as i32;
            best_s2 = s2 as i32;
        }
    }
    let best = pc.get_key_int("slash_tower:score").max(best_pair.max(0));
    let rec = (best > 0).then(|| EndlessBattleRecord {
        Id: best_id,
        FirstRecord: (best_s1 > 0).then(|| HalfEndlessBattleRecord {
            MaxScore: best_s1,
            ..Default::default()
        }),
        SecondRecord: (best_s2 > 0).then(|| HalfEndlessBattleRecord {
            MaxScore: best_s2,
            ..Default::default()
        }),
        SaveTime: now_s,
    });
    Ok(EndLessHistoryResponse {
        ErrorCode: Some(ErrorCode::Success),
        NowSeasonRecord: rec.clone(),
        HistoryRecord: rec,
    })
}
