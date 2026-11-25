use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::helpers::level_play::LevelPlayEventKind;
use sonoro_gamesv_logic::helpers::quest_engine;
use sonoro_gamesv_logic::helpers::reward::RewardItem;
use sonoro_gamesv_logic::helpers::{energy, level_play, pb_kv, reward};
use sonoro_proto::enums::{ActivityTaskState, ErrorCode};
use sonoro_proto::pb::{
    AbyssChallengeGotoNextRoomRequest, AbyssChallengeGotoNextRoomResponse, AbyssChallengeInfo,
    AbyssChallengePassPlayerInfo, AbyssChallengeQuitRequest, AbyssChallengeQuitResponse,
    AbyssChallengeUnlockRequest, AbyssChallengeUnlockResponse, AbyssLikeRequest, AbyssLikeResponse,
    AbyssLittleRoleLevelUpRequest, AbyssLittleRoleLevelUpResponse, AbyssPluginPutRequest,
    AbyssPluginPutResponse, AbyssPluginSynthesisRequest, AbyssPluginSynthesisResponse,
    AbyssRankListRequest, AbyssRankListResponse, AbyssReviveCharacterRequest,
    AbyssReviveCharacterResponse, AbyssReviveNotify, AbyssRewardRequest, AbyssRewardResponse,
    AbyssSelfPassRequest, AbyssSelfPassResponse, AbyssShowNameRequest, AbyssShowNameResponse,
    ActivityTask, AddCountItemInfo, ArrayIntInt, BabelActivitySettleInfoNotify,
    BabelTowerDailyTaskRewardRequest, BabelTowerDailyTaskRewardResponse,
    BabelTowerRankListRequest, BabelTowerRankListResponse, BabelTowerSelfPassRequest,
    BabelTowerSelfPassResponse, BabelTowerSettlementRequest, BabelTowerSettlementResponse,
    BabelTowerShowNameRequest, BabelTowerShowNameResponse, BabelTowerTaskRewardRequest,
    BabelTowerTaskRewardResponse, BossPilingRewardRequest, BossPilingRewardResponse,
    BossPilingSettleRequest, BossPilingSettleResponse, BossRushBatchClaimRewardRequest,
    BossRushBatchClaimRewardResponse, BossRushBuffChooseResultRequest,
    BossRushBuffChooseResultResponse, BossRushChooseBuffRequest, BossRushChooseBuffResponse,
    BossRushClaimRewardRequest, BossRushClaimRewardResponse, BossRushLevelScoreRewardRequest,
    BossRushLevelScoreRewardResponse, BossRushResultNotify, BossRushSettlementRequest,
    BossRushSettlementResponse, BossRushTaskRewardRequest, BossRushTaskRewardResponse,
    EndLessHistoryRequest, EndLessHistoryResponse, EndlessBattleRecord, HalfEndlessBattleRecord,
    NewTowerClearRecordRequest, NewTowerClearRecordResponse, NewTowerClimbingLevelRecord,
    NewTowerCycleRecordPb, NewTowerCycleStartRequest, NewTowerCycleStartResponse,
    NewTowerLastCycleInfoRequest, NewTowerLastCycleInfoResponse, NewTowerMedalInfo,
    NewTowerMedalInfoRequest, NewTowerMedalInfoResponse, NewTowerRecommendData,
    NewTowerRecommendRequest, NewTowerRecommendResponse, NewTowerRewardRequest,
    NewTowerRewardResponse, NewTowerRoleEnergyRequest, NewTowerRoleEnergyResponse,
    NewTowerSeasonRewardRequest, NewTowerSeasonRewardResponse, NewTowerSeasonScoreRewardRequest,
    NewTowerSeasonScoreRewardResponse, NewTowerSeasonTaskDataUpdateRequest,
    NewTowerSeasonTaskDataUpdateResponse, NewTowerUpdateRecordRequest,
    NewTowerUpdateRecordResponse, PublicBabelTowerInfo, RecordRoleEnergy,
    ResetBabelTowerLevelRequest, ResetBabelTowerLevelResponse, RolePropInfo,
    RoleTowerAttributeQueryRequest, RoleTowerAttributeQueryResponse, RoleTowerBestRecord,
    RoleTowerChallengeSettleInfo,
    RoleTowerChallengeSettleRequest, RoleTowerChallengeSettleResponse,
    RoleTowerConsumeItemRequest, RoleTowerConsumeItemResponse, RoleTowerPhantomAddAttrsRequest,
    RoleTowerPhantomAddAttrsResponse, RoleTowerRankEntry, RoleTowerRankInfo,
    RoleTowerRankRequest, RoleTowerRankResponse, RoleTowerRankRewardReceiveRequest,
    RoleTowerRankRewardReceiveResponse, RoleTowerRewardTaskReceiveRequest,
    RoleTowerRewardTaskReceiveResponse, RoleTowerRoleAttributeInfo,
    RoleTowerSaveDeleteRequest, RoleTowerSaveDeleteResponse, RoleTowerScoreRewardReceiveRequest,
    RoleTowerScoreRewardReceiveResponse, RoleTowerStageRetryRequest,
    RoleTowerStageRetryResponse, RoleTowerStageSettleRequest, RoleTowerStageSettleResponse,
    RoleTowerStepAdvanceRequest, RoleTowerStepAdvanceResponse, SelectBabelActivityDeTermRequest,
    SelectBabelActivityDeTermResponse, SlashAndTowerInfoRequest, SlashAndTowerInfoResponse,
    SlashAndTowerRecommendRequest, SlashAndTowerRecommendResponse, SlashAndTowerResetRequest,
    SlashAndTowerResetResponse, SlashAndTowerSaveRecordRequest, SlashAndTowerSaveRecordResponse,
    SlashAndTowerScoreRewardRequest, SlashAndTowerScoreRewardResponse, SlashLevelPlayInfo,
    SlashLevelRecommend,
};

use super::super::connection::drain_event_queue_best_effort;
use super::State;

fn claim_reward_list(
    state: &mut State,
    prefix: &str,
    table_name: &str,
    ids: &[i32],
) -> Result<Vec<i32>, String> {
    const FALLBACK: [RewardItem; 2] = [
        RewardItem {
            item_id: 3,
            count: 50,
        },
        RewardItem {
            item_id: 2,
            count: 15000,
        },
    ];
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
            &FALLBACK,
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

pub fn handle_slash_and_tower_recommend_request(
    _state: &mut State,
    _req: SlashAndTowerRecommendRequest,
) -> Result<SlashAndTowerRecommendResponse, String> {
    Ok(SlashAndTowerRecommendResponse {
        ErrorCode: Some(ErrorCode::Success),
        RecommendInfos: vec![SlashLevelRecommend {
            Usage: 1,
            ..Default::default()
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
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("slash:saved:{}", req.Id),
        1,
    )
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
    _state: &mut State,
    _req: NewTowerRecommendRequest,
) -> Result<NewTowerRecommendResponse, String> {
    Ok(NewTowerRecommendResponse {
        ErrorCode: Some(ErrorCode::Success),
        NewTowerRecommendData: Some(NewTowerRecommendData::default()),
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
    for rid in req.RoleIds {
        let level = state
            .player_components
            .role
            .role_map
            .get(&rid)
            .map(|r| r.level)
            .unwrap_or(90);
        infos.push(RoleTowerRoleAttributeInfo {
            RoleProp: Some(RolePropInfo {
                RoleId: rid,
                BaseProp: vec![ArrayIntInt {
                    Key: 1,
                    Value: level,
                }],
                ..Default::default()
            }),
            ..Default::default()
        });
    }
    Ok(RoleTowerAttributeQueryResponse {
        ErrorCode: Some(ErrorCode::Success),
        RoleInfos: infos,
        PhantomLevel: 25,
    })
}

pub fn handle_role_tower_challenge_settle_request(
    state: &mut State,
    req: RoleTowerChallengeSettleRequest,
) -> Result<RoleTowerChallengeSettleResponse, String> {
    let info = &mut state.player_components.progress.info;
    let _score = info.add_key_int("role_tower:score", 1000);
    let floor = info.add_key_int("role_tower:stage", 1);
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
                floor as i32,
            );
        }
    }
    drain_event_queue_best_effort(&mut events, state);
    let rec = RoleTowerBestRecord {
        ChallengeId: req.ChallengeId,
        PassedFloorCount: floor.max(0) as i32,
        IsAllPassed: true,
        IsNewRecord: true,
        ..Default::default()
    };
    Ok(RoleTowerChallengeSettleResponse {
        ErrorCode: Some(ErrorCode::Success),
        ChallengeSettleInfo: Some(RoleTowerChallengeSettleInfo {
            ChallengeId: req.ChallengeId,
            CurrentRecord: Some(rec.clone()),
            BestRecord: Some(rec),
            IsBestRecordUpdated: true,
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
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("role_tower:attr_cnt:{}", req.ChallengeId),
        req.RoleIds.len() as i64,
    )
    .map_err(|e| e.to_string())?;
    Ok(RoleTowerPhantomAddAttrsResponse {
        ErrorCode: Some(ErrorCode::Success),
        ..Default::default()
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
    let self_entry = RoleTowerRankEntry {
        PlayerId: state.player_id,
        Name: state.player_components.basic.info.attributes.name.as_bytes().to_vec(),
        Rank: 1,
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
    let info = &mut state.player_components.progress.info;
    let floor = info.add_key_int("role_tower:stage", 1);
    info.mark_claim(&format!("role_tower:stage:{floor}"));
    info.add_key_int("role_tower:score", 500);
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
                floor as i32,
            );
        }
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
    state
        .player_components
        .progress
        .info
        .add_key_int("abyss:plugin_synth", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    let out = req
        .PluginItemIncrId
        .first()
        .map(|&incr_id| {
            vec![AddCountItemInfo {
                Id: 1,
                Count: 1,
                IncrId: incr_id,
            }]
        })
        .unwrap_or_default();
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
    Ok(AbyssRankListResponse {
        ErrorCode: Some(ErrorCode::Success),
        SelfPassData: vec![AbyssChallengeInfo {
            PlayerInfos: vec![pinfo],
            ChallengeId: 1,
            PassWave: pass_wave,
            IsSingle: true,
            OwnerId: state.player_id,
            ..Default::default()
        }],
        ..Default::default()
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
    let _ = claim_reward_list(state, "bossrush:clm", "BossRushScore", &[req.RewardId])?;
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
    let score = state
        .player_components
        .progress
        .info
        .add_key_int("bossrush:score", 2000);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    let _ = state.conn.push_msg(
        BossRushResultNotify::MSG_ID,
        &BossRushResultNotify {
            ActivityId: 1,
            InstId: 1,
            KilMonsterScore: 1000,
            TimerScore: 500,
            TechnicalScore: 500,
            HighScore: score.max(2000) as i32,
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
    let info = &mut state.player_components.progress.info;
    info.set_key_int("bp:hp:1001", 100);
    info.set_key_int("bp:max_hp", 100);
    info.add_key_int("bp:score", 1000);
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
        ..Default::default()
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
    let info = &mut state.player_components.progress.info;
    info.add_key_int("babel:finish", 1);
    info.add_key_int("babel:star", 3);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    let _ = state.conn.push_msg(
        BabelActivitySettleInfoNotify::MSG_ID,
        &BabelActivitySettleInfoNotify {
            IsPass: true,
            Star: 3,
            PassTime: 60,
            UseTime: 60,
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
    let mut best = pc.get_key_int("slash_tower:score");
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
        if s1 + s2 > best {
            best = s1 + s2;
            best_id = id;
            best_s1 = s1 as i32;
            best_s2 = s2 as i32;
        }
    }
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
