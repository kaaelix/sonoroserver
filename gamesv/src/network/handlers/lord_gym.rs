use sonoro_gamesv_fs::progress_info::ProgressInfo;
use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::helpers::{level_play, quest_engine};
use sonoro_proto::enums::ErrorCode;
use sonoro_proto::pb::{
    EnterLevelPlayNotify, LevelPlayStateNotify, LordGymBeginRequest, LordGymBeginResponse,
    LordGymEntranceInfo, LordGymGroupInfo, LordGymInfoRequest, LordGymInfoResponse,
    LordGymPassRecord, LordGymReadRequest, LordGymReadResponse,
};

use super::State;

pub fn handle_lord_gym_info_request(
    state: &mut State,
    _req: LordGymInfoRequest,
) -> Result<LordGymInfoResponse, String> {
    let unlock_ids: Vec<i32> = state
        .assets
        .tables
        .lord_gym
        .items
        .iter()
        .map(|row| row.Id)
        .collect();

    let read_ids = state
        .player_components
        .progress
        .info
        .challenge
        .read_lord_gyms
        .clone();

    let entrance_infos: Vec<LordGymEntranceInfo> = state
        .assets
        .tables
        .lord_gym_entrance
        .items
        .iter()
        .map(|row| LordGymEntranceInfo {
            ConfigId: row.Id,
            EffectBeginTime: 0,
            EffectEndTime: 0,
        })
        .collect();

    let pass_records: Vec<LordGymPassRecord> = state
        .player_components
        .progress
        .info
        .challenge
        .passed_lord_gyms
        .iter()
        .map(|rec| LordGymPassRecord {
            LoadGymId: rec.gym_id,
            PassTime: rec.pass_time,
            RoleIds: Vec::new(),
        })
        .collect();

    let mut group_infos = Vec::new();
    for ent in &state.assets.tables.lord_gym_entrance.items {
        let mut max_diff = 0i32;
        for cfg in &state.assets.tables.lord_gym.items {
            if cfg.Id / 100 != ent.Id / 10 {
                continue;
            }
            if state
                .player_components
                .progress
                .info
                .has_passed_lord_gym(cfg.Id)
                && cfg.Difficulty > max_diff
            {
                max_diff = cfg.Difficulty;
            }
        }
        if max_diff > 0 {
            group_infos.push(LordGymGroupInfo {
                GroupId: ent.Id,
                PassDiff: max_diff,
            });
        }
    }

    Ok(LordGymInfoResponse {
        UnlockLoadGymIds: unlock_ids,
        ReadLoadGymIds: read_ids,
        LordGymPassRecords: pass_records,
        LordGymEntranceInfos: entrance_infos,
        LordGymGroupInfos: group_infos,
    })
}

pub fn handle_lord_gym_read_request(
    state: &mut State,
    req: LordGymReadRequest,
) -> Result<LordGymReadResponse, String> {
    if state
        .assets
        .tables
        .lord_gym
        .get_data_by_id(&req.LoadGymId)
        .is_none()
    {
        return Ok(LordGymReadResponse {
            ErrorCode: Some(ErrorCode::ErrLordGymConfigNotFound),
        });
    }
    let pc = &mut state.player_components.progress;
    if ProgressInfo::append_unique_id(&mut pc.info.challenge.read_lord_gyms, req.LoadGymId) {
        pc.save(&state.fs).map_err(|e| e.to_string())?;
    }
    Ok(LordGymReadResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_lord_gym_begin_request(
    state: &mut State,
    events: &mut EventQueue,
    req: LordGymBeginRequest,
) -> Result<LordGymBeginResponse, String> {
    let Some(gym) = state
        .assets
        .tables
        .lord_gym
        .get_data_by_id(&req.LoadGymId)
        .map(|row| row.into_owned())
    else {
        return Ok(LordGymBeginResponse {
            ErrorCode: Some(ErrorCode::ErrLordGymConfigNotFound),
        });
    };

    if gym.PlayId > 0 {
        if let Some(lp) = state
            .assets
            .tables
            .level_play_data
            .get_data_by_id(&gym.PlayId)
        {
            let target_inst = if lp.Data.InstanceId != 0 {
                lp.Data.InstanceId
            } else {
                lp.Data.LevelId
            };
            let cur_inst = state.scene.as_ref().map_or(0, |s| s.instance_id);
            let in_lord_gym_inst = state
                .assets
                .tables
                .instance_dungeon
                .get_data_by_id(&cur_inst)
                .is_some_and(|inst| inst.InstSubType == 48);
            if target_inst > 0
                && target_inst != cur_inst
                && !in_lord_gym_inst
                && state
                    .assets
                    .tables
                    .instance_dungeon
                    .get_data_by_id(&target_inst)
                    .is_some()
            {
                let mut qctx = state.quest_context(events);
                let _ = quest_engine::enter_inst(&mut qctx, target_inst, 0);
            }
        }

        let mut started_in_scene = false;
        {
            let mut qctx = state.quest_context(events);
            if let Some(mut lp_ctx) = level_play::LevelPlayContext::from_quest_ctx(&mut qctx) {
                let _ = level_play::on_level_play_enabled(&mut lp_ctx, gym.PlayId, true);
                started_in_scene = true;
            }
        }
        if !started_in_scene {
            let _ = state.conn.push_msg(
                EnterLevelPlayNotify::MSG_ID,
                &EnterLevelPlayNotify {
                    Id: gym.PlayId,
                    State: 1,
                    CanGetReward: false,
                },
            );
            let _ = state.conn.push_msg(
                LevelPlayStateNotify::MSG_ID,
                &LevelPlayStateNotify {
                    Id: gym.PlayId,
                    State: 1,
                    CompleteNumber: 0,
                },
            );
        }
    }

    Ok(LordGymBeginResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}
