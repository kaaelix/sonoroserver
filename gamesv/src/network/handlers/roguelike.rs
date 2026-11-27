use std::collections::BTreeMap;

use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::helpers::reward::RewardItem;
use sonoro_gamesv_logic::helpers::{energy, pb_kv, quest_engine, reward};
use sonoro_proto::enums::{ErrorCode, RoverRogueGrade};
use sonoro_proto::pb::{
    MulRogueWeeklyRewardRequest, MulRogueWeeklyRewardResponse, MulRoguelikeWriteCatRewardRequest,
    MulRoguelikeWriteCatRewardResponse, PopularEntrie, RogueArchiveInfo,
    RogueArchiveListRequest, RogueArchiveListResponse, RogueArchiveSaveRequest,
    RogueArchiveSaveResponse, RogueBlackFlowerRequest, RogueBlackFlowerResponse,
    RogueInstExtraInfo, RogueInstExtraInfoRequest, RogueInstExtraInfoResponse,
    RogueResChangeFormationRequest, RogueResChangeFormationResponse, RogueResEndingRewardRequest,
    RogueResEndingRewardResponse, RogueResFormation, RogueResGotoNextRoomRequest,
    RogueResGotoNextRoomResponse, RogueResGridEventRequest, RogueResGridEventResponse,
    RogueResIllustrationAwardRequest, RogueResIllustrationAwardResponse, InstResultView,
    RogueResInstResultRequest, RogueResInstResultResponse, RogueResInstStartRequest,
    RogueResInstStartResponse, RogueResLastInstInfoRequest, RogueResLastInstInfoResponse,
    RogueResMoveRequest, RogueResMoveResponse, RogueResOpDoneRequest, RogueResOpDoneResponse,
    RogueResSwitchFormationRequest, RogueResSwitchFormationResponse,
    RogueResTalentSkillLevelRequest, RogueResTalentSkillLevelResponse, RogueResTaskAwardRequest,
    RogueResTaskAwardResponse, RogueTowerTrialGiveUpTempArchiveRequest,
    RogueTowerTrialGiveUpTempArchiveResponse, RogueTowerTrialResultInfo,
    RogueTowerTrialResultRequest, RogueTowerTrialResultResponse,
    RogueTowerTrialSelectBossRequest, RogueTowerTrialSelectBossResponse,
    RogueTowerTrialStartRequest, RogueTowerTrialStartResponse, RogueWeeklyArtifactInfoRequest,
    RogueWeeklyArtifactInfoResponse, RogueWeeklyArtifactSelectRequest,
    RogueWeeklyArtifactSelectResponse, RogueWeeklyBlackFlowerRequest,
    RogueWeeklyBlackFlowerResponse, RogueWeeklyEntry, RogueWeeklyGotoNextRoomRequest,
    RogueWeeklyGotoNextRoomResponse, RogueWeeklyLastInfo, RogueWeeklyLastInfoRequest,
    RogueWeeklyLastInfoResponse, RogueWeeklyOptionSelectRequest,
    RogueWeeklyOptionSelectResponse, RogueWeeklyResultRequest, RogueWeeklyResultResponse,
    RogueWeeklyStartRequest, RogueWeeklyStartResponse, RogueWhiteCatLimitedRewardRequest,
    RogueWhiteCatLimitedRewardResponse, RogueWhiteCatRecordRequest, RogueWhiteCatRecordResponse,
    RoguelikeChooseData, RoguelikeChooseDataNotify, RoguelikeChooseDataRequest,
    RoguelikeChooseDataResponse, RoguelikeChooseDataResultRequest,
    RoguelikeChooseDataResultResponse, RoguelikeGiveUpGainRequest, RoguelikeGiveUpGainResponse,
    RoguelikeGotoNextRoomRequest, RoguelikeGotoNextRoomResponse, RoguelikeLastInfo,
    RoguelikeLastInfoRequest, RoguelikeLastInfoResponse, RoguelikePopularEntriesChangeRequest,
    RoguelikePopularEntriesChangeResponse, RoguelikePopularEntriesInfoRequest,
    RoguelikePopularEntriesInfoResponse, RoguelikeQuitRequest, RoguelikeQuitResponse,
    RoguelikeRefreshGainRequest, RoguelikeRefreshGainResponse, RoguelikeResultInfo,
    RoguelikeResultRequest, RoguelikeResultResponse, RoguelikeRoleInstStartRequest,
    RoguelikeRoleInstStartResponse, RoguelikeRoleRoomSelectRequest,
    RoguelikeRoleRoomSelectResponse, RoguelikeSeasonRewardReceiveRequest,
    RoguelikeSeasonRewardReceiveResponse, RoguelikeStartRequest, RoguelikeStartResponse,
    RoguelikeTalentInfoRequest, RoguelikeTalentInfoResponse, RoguelikeTalentLevelUpRequest,
    RoguelikeTalentLevelUpResponse, RoguelikeTokenReceiveRequest, RoguelikeTokenReceiveResponse,
    RoguelikeTrialRoleInfoRequest, RoguelikeTrialRoleInfoResponse,
    RoguelikeWriteCatBossRewardRequest, RoguelikeWriteCatBossRewardResponse,
    RoguelikeWriteCatLevelPlayRewardRequest, RoguelikeWriteCatLevelPlayRewardResponse,
    RoverRogueBlessGroupSelectRequest, RoverRogueBlessGroupSelectResponse,
    RoverRogueChooseDataResultRequest, RoverRogueChooseDataResultResponse,
    RoverRogueEventSelectRequest, RoverRogueEventSelectResponse, RoverRogueGainEntry,
    RoverRogueGiveUpGainRequest, RoverRogueGiveUpGainResponse,
    RoverRogueGotoNextRoomCommitRequest, RoverRogueGotoNextRoomCommitResponse,
    RoverRogueInsEntry, RoverRogueInsListRequest, RoverRogueInsListResponse,
    RoverRogueLootChangeRequest, RoverRogueLootChangeResponse, RoverRogueLootInfoRequest,
    RoverRogueLootInfoResponse, RoverRogueQuitRequest, RoverRogueQuitResponse,
    RoverRogueChooseData, RoverRogueRefreshGainRequest, RoverRogueRefreshGainResponse,
    RoverRogueResultInfo, RoverRogueResultRequest, RoverRogueResultResponse,
    RoverRogueShopBuyRequest, RoverRogueShopBuyResponse, RoverRogueShopRefreshRequest,
    RoverRogueShopRefreshResponse, RoverRogueShopSnapshot, RoverRogueTalentUnlockRequest,
    RoverRogueTalentUnlockResponse, RoverRogueTaskRewardTakeRequest,
    RoverRogueTaskRewardTakeResponse,
};

use super::State;

fn reward_to_map(
    state: &mut State,
    table_name: &str,
    key: i64,
    _fallback: &[RewardItem],
) -> Result<BTreeMap<i32, i32>, String> {
    let mut granted = Vec::new();
    let mut rctx = state.reward_context();
    reward::grant_table_reward(
        &mut rctx,
        table_name,
        key,
        false,
        true,
        19500,
        &[],
        Some(&mut granted),
    )
    .map_err(|e| e.to_string())?;
    let mut map = BTreeMap::new();
    for it in granted {
        map.insert(it.item_id, it.count);
    }
    Ok(map)
}

pub fn handle_roguelike_start_request(
    state: &mut State,
    req: RoguelikeStartRequest,
) -> Result<RoguelikeStartResponse, String> {
    let info = &mut state.player_components.progress.info;
    info.set_key_int("rogue:season", i64::from(req.SeasonId));
    info.set_key_int("rogue:inst", i64::from(req.InstId));
    info.set_key_int("rogue:hot_group", i64::from(req.SelectHotEntryGroupId));
    if !req.ContinueLastProgress {
        info.set_key_int("rogue:layer", 1);
        info.set_key_int("rogue:gain_refresh", 0);
        info.set_key_int("rogue:gain_count", 0);
    }
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RoguelikeStartResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_roguelike_last_info_request(
    state: &mut State,
    _req: RoguelikeLastInfoRequest,
) -> Result<RoguelikeLastInfoResponse, String> {
    let pc = &state.player_components.progress.info;
    let inst_id = pc.get_key_int("rogue:inst") as i32;
    let cur_layer = pc.get_key_int("rogue:layer").max(1) as i32;
    let infos = if inst_id > 0 {
        vec![RoguelikeLastInfo {
            InstId: inst_id,
            CurLayer: cur_layer,
            MaxLayer: 10,
            ..Default::default()
        }]
    } else {
        Vec::new()
    };
    Ok(RoguelikeLastInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        RoguelikeLastInfos: infos,
        HasData: inst_id > 0,
        ..Default::default()
    })
}

pub fn handle_roguelike_choose_data_request(
    state: &mut State,
    req: RoguelikeChooseDataRequest,
) -> Result<RoguelikeChooseDataResponse, String> {
    let pc = &state.player_components.progress.info;
    let layer = if req.Layer > 0 {
        req.Layer
    } else {
        pc.get_key_int("rogue:layer").max(1) as i32
    };
    let used = pc.get_key_int("rogue:gain_refresh").clamp(0, 3) as i32;
    Ok(RoguelikeChooseDataResponse {
        ErrorCode: Some(ErrorCode::Success),
        RoguelikeChooseData: Some(RoguelikeChooseData {
            Index: req.Index,
            Layer: layer,
            MaxTime: 3,
            UseTime: used,
            IsSelect: false,
            ..Default::default()
        }),
        ..Default::default()
    })
}

pub fn handle_roguelike_choose_data_result_request(
    state: &mut State,
    req: RoguelikeChooseDataResultRequest,
) -> Result<RoguelikeChooseDataResultResponse, String> {
    let info = &mut state.player_components.progress.info;
    info.set_key_int(
        &format!("rogue:gain:{}:{}", req.Layer, req.BindId),
        i64::from(req.Index),
    );
    info.add_key_int("rogue:gain_count", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RoguelikeChooseDataResultResponse {
        ErrorCode: Some(ErrorCode::Success),
        IsSelect: true,
        RoguelikeChooseDataNotify: Some(RoguelikeChooseDataNotify::default()),
        ..Default::default()
    })
}

pub fn handle_roguelike_refresh_gain_request(
    state: &mut State,
    req: RoguelikeRefreshGainRequest,
) -> Result<RoguelikeRefreshGainResponse, String> {
    let cur = state
        .player_components
        .progress
        .info
        .get_key_int("rogue:gain_refresh")
        .max(0);
    if cur >= 3 {
        return Ok(RoguelikeRefreshGainResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
            RoguelikeChooseData: Some(RoguelikeChooseData {
                Index: req.BindId,
                Layer: req.Layer,
                MaxTime: 3,
                UseTime: 3,
                ..Default::default()
            }),
            ..Default::default()
        });
    }
    let cnt = state
        .player_components
        .progress
        .info
        .add_key_int("rogue:gain_refresh", 1)
        .clamp(0, 3) as i32;
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RoguelikeRefreshGainResponse {
        ErrorCode: Some(ErrorCode::Success),
        RoguelikeChooseData: Some(RoguelikeChooseData {
            Index: req.BindId,
            Layer: req.Layer,
            MaxTime: 3,
            UseTime: cnt,
            ..Default::default()
        }),
        ..Default::default()
    })
}

pub fn handle_roguelike_give_up_gain_request(
    state: &mut State,
    req: RoguelikeGiveUpGainRequest,
) -> Result<RoguelikeGiveUpGainResponse, String> {
    let item_map = reward_to_map(
        state,
        "RogueSeasonReward",
        i64::from(req.BindId),
        &[RewardItem {
            item_id: 2,
            count: 5000,
        }],
    )?;
    Ok(RoguelikeGiveUpGainResponse {
        ErrorCode: Some(ErrorCode::Success),
        BindId: req.BindId,
        ItemMap: item_map,
    })
}

pub fn handle_roguelike_goto_next_room_request(
    state: &mut State,
    _req: RoguelikeGotoNextRoomRequest,
) -> Result<RoguelikeGotoNextRoomResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int("rogue:layer", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RoguelikeGotoNextRoomResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_roguelike_quit_request(
    state: &mut State,
    _req: RoguelikeQuitRequest,
) -> Result<RoguelikeQuitResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "rogue:inst",
        0,
    )
    .map_err(|e| e.to_string())?;
    Ok(RoguelikeQuitResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_roguelike_result_request(
    state: &mut State,
    req: RoguelikeResultRequest,
) -> Result<RoguelikeResultResponse, String> {
    let pc = &state.player_components.progress.info;
    let cur_layer = pc.get_key_int("rogue:layer").max(1) as i32;
    let gain_count = pc.get_key_int("rogue:gain_count").max(0) as i32;
    let rewards = reward_to_map(
        state,
        "RogueSeasonReward",
        i64::from(req.InstId),
        &[
            RewardItem {
                item_id: 3,
                count: 60,
            },
            RewardItem {
                item_id: 2,
                count: 20000,
            },
        ],
    )?;
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "rogue:inst",
        0,
    )
    .map_err(|e| e.to_string())?;
    Ok(RoguelikeResultResponse {
        ErrorCode: Some(ErrorCode::Success),
        RoguelikeResultInfo: Some(RoguelikeResultInfo {
            InstId: req.InstId,
            CurLayer: cur_layer,
            MaxLayer: 10,
            GetGainCount: gain_count,
            Rewards: rewards,
            Time: energy::now_seconds().clamp(0, i64::from(u32::MAX)) as u32,
            ..Default::default()
        }),
    })
}

pub fn handle_roguelike_popular_entries_info_request(
    state: &mut State,
    req: RoguelikePopularEntriesInfoRequest,
) -> Result<RoguelikePopularEntriesInfoResponse, String> {
    let saved = state
        .player_components
        .progress
        .info
        .get_key_int("rogue:pop_entry");
    let entries = if saved > 0 {
        vec![saved as i32]
    } else {
        Vec::new()
    };
    Ok(RoguelikePopularEntriesInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        PopularEntrie: Some(PopularEntrie {
            InstId: req.InstId,
            SeasonId: req.SeasonId,
            Entries: entries,
        }),
    })
}

pub fn handle_roguelike_popular_entries_change_request(
    state: &mut State,
    req: RoguelikePopularEntriesChangeRequest,
) -> Result<RoguelikePopularEntriesChangeResponse, String> {
    if let Some(&first) = req.Entries.first() {
        pb_kv::persist_int(
            &state.fs,
            &mut state.player_components.progress,
            "rogue:pop_entry",
            i64::from(first),
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(RoguelikePopularEntriesChangeResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_roguelike_season_reward_receive_request(
    state: &mut State,
    req: RoguelikeSeasonRewardReceiveRequest,
) -> Result<RoguelikeSeasonRewardReceiveResponse, String> {
    let mut item_map = BTreeMap::new();
    for idx in req.Index {
        let key = format!("rogue:season_rw:{}:{idx}", req.SeasonId);
        if state.player_components.progress.info.mark_claim(&key) {
            let m = reward_to_map(
                state,
                "RogueSeasonReward",
                i64::from(idx),
                &[
                    RewardItem {
                        item_id: 3,
                        count: 50,
                    },
                    RewardItem {
                        item_id: 2,
                        count: 15000,
                    },
                ],
            )?;
            for (k, v) in m {
                *item_map.entry(k).or_insert(0) += v;
            }
        }
    }
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RoguelikeSeasonRewardReceiveResponse {
        ErrorCode: Some(ErrorCode::Success),
        ItemMap: item_map,
    })
}

pub fn handle_roguelike_token_receive_request(
    state: &mut State,
    req: RoguelikeTokenReceiveRequest,
) -> Result<RoguelikeTokenReceiveResponse, String> {
    let key = format!("rogue:token:{}:{}", req.SeasonId, req.Id);
    let mut item_map = BTreeMap::new();
    if state.player_components.progress.info.mark_claim(&key) {
        item_map = reward_to_map(
            state,
            "RogueToken",
            i64::from(req.Id),
            &[RewardItem {
                item_id: 3,
                count: 40,
            }],
        )?;
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(RoguelikeTokenReceiveResponse {
        ErrorCode: Some(ErrorCode::Success),
        ItemMap: item_map,
    })
}

pub fn handle_roguelike_talent_info_request(
    state: &mut State,
    _req: RoguelikeTalentInfoRequest,
) -> Result<RoguelikeTalentInfoResponse, String> {
    let dict = state
        .player_components
        .progress
        .info
        .dynamic
        .ints
        .iter()
        .filter_map(|e| {
            let sid = e.key.strip_prefix("rogue:talent:")?.parse::<i32>().ok()?;
            Some((sid, e.val as i32))
        })
        .collect();
    Ok(RoguelikeTalentInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        TalentSkillDict: dict,
    })
}

pub fn handle_roguelike_talent_level_up_request(
    state: &mut State,
    req: RoguelikeTalentLevelUpRequest,
) -> Result<RoguelikeTalentLevelUpResponse, String> {
    let lv = state
        .player_components
        .progress
        .info
        .add_key_int(&format!("rogue:talent:{}", req.SkillId), 1) as i32;
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RoguelikeTalentLevelUpResponse {
        ErrorCode: Some(ErrorCode::Success),
        Level: lv,
    })
}

pub fn handle_roguelike_trial_role_info_request(
    _state: &mut State,
    req: RoguelikeTrialRoleInfoRequest,
) -> Result<RoguelikeTrialRoleInfoResponse, String> {
    Ok(RoguelikeTrialRoleInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        ValidTrialRoleIds: req.TrialRoleIds,
        RoleLevel: 90,
        MaxLevel: 90,
        WeaponLevel: 90,
        ..Default::default()
    })
}

pub fn handle_roguelike_role_inst_start_request(
    state: &mut State,
    req: RoguelikeRoleInstStartRequest,
) -> Result<RoguelikeRoleInstStartResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "rogue:role_inst_idx",
        i64::from(req.Index),
    )
    .map_err(|e| e.to_string())?;
    Ok(RoguelikeRoleInstStartResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_roguelike_role_room_select_request(
    state: &mut State,
    req: RoguelikeRoleRoomSelectRequest,
) -> Result<RoguelikeRoleRoomSelectResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("rogue:role_room:{}", req.RoleRoomId),
        i64::from(req.Index),
    )
    .map_err(|e| e.to_string())?;
    Ok(RoguelikeRoleRoomSelectResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_rogue_inst_extra_info_request(
    _state: &mut State,
    req: RogueInstExtraInfoRequest,
) -> Result<RogueInstExtraInfoResponse, String> {
    Ok(RogueInstExtraInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        RogueInstExtraInfo: Some(RogueInstExtraInfo {
            SeasonId: req.SeasonId,
            ..Default::default()
        }),
    })
}

pub fn handle_rogue_archive_save_request(
    state: &mut State,
    req: RogueArchiveSaveRequest,
) -> Result<RogueArchiveSaveResponse, String> {
    let now_s = energy::now_seconds();
    let info = &mut state.player_components.progress.info;
    let inst = info.get_key_int("rogue:inst");
    info.set_key_int(&format!("rogue:arch_time:{}", req.SlotId), now_s);
    info.set_key_int(&format!("rogue:arch_inst:{}", req.SlotId), inst);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RogueArchiveSaveResponse {
        SaveTime: now_s,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_rogue_archive_list_request(
    state: &mut State,
    req: RogueArchiveListRequest,
) -> Result<RogueArchiveListResponse, String> {
    let pc = &state.player_components.progress.info;
    let list = (1..=4)
        .map(|slot| {
            let stime = pc.get_key_int(&format!("rogue:arch_time:{slot}"));
            let inst = pc.get_key_int(&format!("rogue:arch_inst:{slot}"));
            RogueArchiveInfo {
                SlotId: slot,
                HasData: stime > 0,
                SaveTime: stime,
                InstId: inst as i32,
                SeasonId: req.SeasonId,
                ..Default::default()
            }
        })
        .collect();
    Ok(RogueArchiveListResponse {
        ErrorCode: Some(ErrorCode::Success),
        ArchiveInfos: list,
    })
}

pub fn handle_rogue_black_flower_request(
    state: &mut State,
    req: RogueBlackFlowerRequest,
) -> Result<RogueBlackFlowerResponse, String> {
    state
        .player_components
        .progress
        .info
        .mark_claim(&format!("rogue:bf:{}", req.EntityConfigId));
    if req.Ret {
        let mut rctx = state.reward_context();
        reward::grant_table_reward(
            &mut rctx,
            "RogueSeasonReward",
            i64::from(req.EntityConfigId),
            false,
            true,
            1,
            &[],
            None,
        )
        .map_err(|e| e.to_string())?;
    }
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RogueBlackFlowerResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_rogue_res_inst_start_request(
    state: &mut State,
    req: RogueResInstStartRequest,
) -> Result<RogueResInstStartResponse, String> {
    let info = &mut state.player_components.progress.info;
    info.set_key_int("rogue_res:inst", i64::from(req.InstId));
    info.set_key_int("rogue_res:room", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RogueResInstStartResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_rogue_res_last_inst_info_request(
    state: &mut State,
    _req: RogueResLastInstInfoRequest,
) -> Result<RogueResLastInstInfoResponse, String> {
    Ok(RogueResLastInstInfoResponse {
        InstId: state
            .player_components
            .progress
            .info
            .get_key_int("rogue_res:inst") as i32,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_rogue_res_goto_next_room_request(
    state: &mut State,
    _req: RogueResGotoNextRoomRequest,
) -> Result<RogueResGotoNextRoomResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int("rogue_res:room", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RogueResGotoNextRoomResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_rogue_res_move_request(
    state: &mut State,
    req: RogueResMoveRequest,
) -> Result<RogueResMoveResponse, String> {
    let info = &mut state.player_components.progress.info;
    if let Some(op) = req.MoveOp
        && let Some(&last) = op.Paths.last()
    {
        info.set_key_int("rogue_res:grid", i64::from(last));
    }
    info.add_key_int("rogue_res:mood_cost", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RogueResMoveResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_rogue_res_grid_event_request(
    state: &mut State,
    req: RogueResGridEventRequest,
) -> Result<RogueResGridEventResponse, String> {
    let info = &mut state.player_components.progress.info;
    info.mark_claim(&format!("rogue_res:evt:{}", req.GridIndex));
    info.set_key_int(
        &format!("rogue_res:evt_step:{}", req.GridIndex),
        i64::from(req.OptionIndex),
    );
    info.add_key_int("rogue_res:evt", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RogueResGridEventResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_rogue_res_op_done_request(
    state: &mut State,
    req: RogueResOpDoneRequest,
) -> Result<RogueResOpDoneResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("rogue_res:op:{}", req.IncId),
        i64::from(req.ClientSelectId),
    )
    .map_err(|e| e.to_string())?;
    Ok(RogueResOpDoneResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_rogue_res_change_formation_request(
    state: &mut State,
    req: RogueResChangeFormationRequest,
) -> Result<RogueResChangeFormationResponse, String> {
    if let Some(ref form) = req.RogueResFormation {
        pb_kv::save_msg::<RogueResFormation>(
            &state.fs,
            &mut state.player_components.progress,
            &format!("rogue_res:form:{}", req.Index),
            form,
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(RogueResChangeFormationResponse {
        ErrorCode: Some(ErrorCode::Success),
        RogueResFormation: req.RogueResFormation,
    })
}

pub fn handle_rogue_res_switch_formation_request(
    state: &mut State,
    req: RogueResSwitchFormationRequest,
) -> Result<RogueResSwitchFormationResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "rogue_res:form_idx",
        i64::from(req.Index),
    )
    .map_err(|e| e.to_string())?;
    Ok(RogueResSwitchFormationResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_rogue_res_talent_skill_level_request(
    state: &mut State,
    req: RogueResTalentSkillLevelRequest,
) -> Result<RogueResTalentSkillLevelResponse, String> {
    let lv = state
        .player_components
        .progress
        .info
        .add_key_int(&format!("rogue_res:skill:{}", req.SkillId), 1) as i32;
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RogueResTalentSkillLevelResponse {
        Level: lv,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_rogue_res_inst_result_request(
    state: &mut State,
    _req: RogueResInstResultRequest,
) -> Result<RogueResInstResultResponse, String> {
    let inst_id = state
        .player_components
        .progress
        .info
        .get_key_int("rogue_res:inst") as i32;
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "rogue_res:inst",
        0,
    )
    .map_err(|e| e.to_string())?;
    Ok(RogueResInstResultResponse {
        ErrorCode: Some(ErrorCode::Success),
        InstResultView: Some(InstResultView {
            InstId: inst_id,
            IsSucc: true,
            Score: 1000,
            ..Default::default()
        }),
    })
}

pub fn handle_rogue_res_ending_reward_request(
    state: &mut State,
    req: RogueResEndingRewardRequest,
) -> Result<RogueResEndingRewardResponse, String> {
    let key = format!("rogue_res:end:{}:{}", req.SeasonId, req.Index);
    if state.player_components.progress.info.mark_claim(&key) {
        let mut rctx = state.reward_context();
        reward::grant_table_reward(
            &mut rctx,
            "RogueResEnding",
            i64::from(req.Index),
            false,
            true,
            1,
            &[],
            None,
        )
        .map_err(|e| e.to_string())?;
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(RogueResEndingRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_rogue_res_illustration_award_request(
    state: &mut State,
    req: RogueResIllustrationAwardRequest,
) -> Result<RogueResIllustrationAwardResponse, String> {
    for cid in req.ConfigIds {
        if state
            .player_components
            .progress
            .info
            .mark_claim(&format!("rogue_res:ill:{cid}"))
        {
            let mut rctx = state.reward_context();
            reward::grant_table_reward(
                &mut rctx,
                "RogueResIllustration",
                i64::from(cid),
                false,
                true,
                1,
                &[],
                None,
            )
            .map_err(|e| e.to_string())?;
        }
    }
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RogueResIllustrationAwardResponse {
        ErrorCode: Some(ErrorCode::Success),
        IsFull: false,
    })
}

pub fn handle_rogue_res_task_award_request(
    state: &mut State,
    req: RogueResTaskAwardRequest,
) -> Result<RogueResTaskAwardResponse, String> {
    if state
        .player_components
        .progress
        .info
        .mark_claim(&format!("rogue_res:task:{}", req.ConfigId))
    {
        let mut rctx = state.reward_context();
        reward::grant_table_reward(
            &mut rctx,
            "RogueResTask",
            i64::from(req.ConfigId),
            false,
            true,
            1,
            &[],
            None,
        )
        .map_err(|e| e.to_string())?;
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(RogueResTaskAwardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_rogue_tower_trial_start_request(
    state: &mut State,
    req: RogueTowerTrialStartRequest,
) -> Result<RogueTowerTrialStartResponse, String> {
    let info = &mut state.player_components.progress.info;
    info.set_key_int("rogue_trial:inst", i64::from(req.InstId));
    info.set_key_int("rogue_trial:slot", i64::from(req.SlotId));
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RogueTowerTrialStartResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_rogue_tower_trial_select_boss_request(
    state: &mut State,
    req: RogueTowerTrialSelectBossRequest,
) -> Result<RogueTowerTrialSelectBossResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "rogue_trial:boss",
        i64::from(req.Id),
    )
    .map_err(|e| e.to_string())?;
    Ok(RogueTowerTrialSelectBossResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_rogue_tower_trial_give_up_temp_archive_request(
    state: &mut State,
    _req: RogueTowerTrialGiveUpTempArchiveRequest,
) -> Result<RogueTowerTrialGiveUpTempArchiveResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "rogue_trial:slot",
        0,
    )
    .map_err(|e| e.to_string())?;
    Ok(RogueTowerTrialGiveUpTempArchiveResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_rogue_tower_trial_result_request(
    state: &mut State,
    _req: RogueTowerTrialResultRequest,
) -> Result<RogueTowerTrialResultResponse, String> {
    let pc = &state.player_components.progress.info;
    Ok(RogueTowerTrialResultResponse {
        ErrorCode: Some(ErrorCode::Success),
        RogueTowerTrialResultInfo: Some(RogueTowerTrialResultInfo {
            SlotId: pc.get_key_int("rogue_trial:slot") as i32,
            ChallengingBossId: pc.get_key_int("rogue_trial:boss") as i32,
            ..Default::default()
        }),
    })
}

pub fn handle_rogue_weekly_start_request(
    state: &mut State,
    req: RogueWeeklyStartRequest,
) -> Result<RogueWeeklyStartResponse, String> {
    let info = &mut state.player_components.progress.info;
    info.set_key_int("rogue_weekly:cycle", i64::from(req.CycleId));
    info.set_key_int("rogue_weekly:layer", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RogueWeeklyStartResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_rogue_weekly_last_info_request(
    state: &mut State,
    _req: RogueWeeklyLastInfoRequest,
) -> Result<RogueWeeklyLastInfoResponse, String> {
    let pc = &state.player_components.progress.info;
    Ok(RogueWeeklyLastInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        RogueWeeklyLastInfo: Some(RogueWeeklyLastInfo {
            InstId: pc.get_key_int("rogue_weekly:cycle") as i32,
            CurLayer: pc.get_key_int("rogue_weekly:layer").max(1) as i32,
            MaxLayer: 4,
            WorldLevel: 1,
        }),
    })
}

pub fn handle_rogue_weekly_artifact_info_request(
    state: &mut State,
    _req: RogueWeeklyArtifactInfoRequest,
) -> Result<RogueWeeklyArtifactInfoResponse, String> {
    let entries = state
        .assets
        .reward_index
        .get_rows("RogueWeeklyBF")
        .iter()
        .take(6)
        .enumerate()
        .map(|(i, row)| RogueWeeklyEntry {
            Index: i as i32,
            ConfigId: row.key as i32,
            ..Default::default()
        })
        .collect();
    Ok(RogueWeeklyArtifactInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        RogueWeeklyEntrys: entries,
    })
}

pub fn handle_rogue_weekly_artifact_select_request(
    state: &mut State,
    req: RogueWeeklyArtifactSelectRequest,
) -> Result<RogueWeeklyArtifactSelectResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "rogue_weekly:artifact",
        i64::from(req.Index),
    )
    .map_err(|e| e.to_string())?;
    Ok(RogueWeeklyArtifactSelectResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_rogue_weekly_option_select_request(
    state: &mut State,
    req: RogueWeeklyOptionSelectRequest,
) -> Result<RogueWeeklyOptionSelectResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("rogue_weekly:opt:{}", req.BindId),
        i64::from(req.Index),
    )
    .map_err(|e| e.to_string())?;
    Ok(RogueWeeklyOptionSelectResponse {
        ErrorCode: Some(ErrorCode::Success),
        ..Default::default()
    })
}

pub fn handle_rogue_weekly_goto_next_room_request(
    state: &mut State,
    req: RogueWeeklyGotoNextRoomRequest,
) -> Result<RogueWeeklyGotoNextRoomResponse, String> {
    if req.Confirm {
        state
            .player_components
            .progress
            .info
            .add_key_int("rogue_weekly:layer", 1);
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(RogueWeeklyGotoNextRoomResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_rogue_weekly_black_flower_request(
    state: &mut State,
    req: RogueWeeklyBlackFlowerRequest,
) -> Result<RogueWeeklyBlackFlowerResponse, String> {
    let cur = state
        .player_components
        .progress
        .info
        .get_key_int("rogue_weekly:bf_used")
        .max(0);
    if cur >= 3 {
        return Ok(RogueWeeklyBlackFlowerResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
            MaxFreeCount: 3,
            UseFreeCount: 3,
        });
    }
    let used = state
        .player_components
        .progress
        .info
        .add_key_int("rogue_weekly:bf_used", 1)
        .clamp(0, 3) as i32;
    let exchange_drop_id = state
        .assets
        .raw_json_cached("RogueWeeklyBF")
        .iter()
        .filter_map(|v| v.as_object())
        .find(|o| {
            o.get("SilentAreaDetectionId").and_then(|v| v.as_i64())
                == Some(i64::from(req.SilentAreaId))
                || o.get("Id").and_then(|v| v.as_i64()) == Some(i64::from(req.SilentAreaId))
        })
        .and_then(|o| o.get("ExchangeRewardId").and_then(|v| v.as_i64()))
        .and_then(|id| i32::try_from(id).ok())
        .unwrap_or(0);
    let mut rctx = state.reward_context();
    if exchange_drop_id > 0 {
        reward::grant_table_reward(
            &mut rctx,
            "RogueWeeklyBF",
            i64::from(req.SilentAreaId),
            false,
            true,
            1,
            &[],
            None,
        )
        .map_err(|e| e.to_string())?;
    } else {
        reward::grant_table_reward(
            &mut rctx,
            "RogueWeeklyReward",
            i64::from(req.SilentAreaId),
            false,
            true,
            1,
            &[],
            None,
        )
        .map_err(|e| e.to_string())?;
    }
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RogueWeeklyBlackFlowerResponse {
        ErrorCode: Some(ErrorCode::Success),
        MaxFreeCount: 3,
        UseFreeCount: used,
    })
}

pub fn handle_rogue_weekly_result_request(
    state: &mut State,
    _req: RogueWeeklyResultRequest,
) -> Result<RogueWeeklyResultResponse, String> {
    let info = &mut state.player_components.progress.info;
    info.add_key_int("rogue_weekly:score", 500);
    info.set_key_int("rogue_weekly:layer", 0);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RogueWeeklyResultResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_mul_rogue_weekly_reward_request(
    state: &mut State,
    req: MulRogueWeeklyRewardRequest,
) -> Result<MulRogueWeeklyRewardResponse, String> {
    for &cid in &req.ConfigId {
        let key = format!("rogue_weekly:rw:{}:{cid}", req.ActivityId);
        if state.player_components.progress.info.mark_claim(&key) {
            let mut rctx = state.reward_context();
            reward::grant_table_reward(
                &mut rctx,
                "RogueWeeklyReward",
                i64::from(cid),
                false,
                true,
                1,
                &[],
                None,
            )
            .map_err(|e| e.to_string())?;
        }
    }
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(MulRogueWeeklyRewardResponse {
        ConfigId: req.ConfigId,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_rogue_white_cat_limited_reward_request(
    state: &mut State,
    req: RogueWhiteCatLimitedRewardRequest,
) -> Result<RogueWhiteCatLimitedRewardResponse, String> {
    if state
        .player_components
        .progress
        .info
        .mark_claim(&format!("rogue_cat:lim:{}", req.Index))
    {
        let mut rctx = state.reward_context();
        reward::grant_table_reward(
            &mut rctx,
            "RogueLimitTimeReward",
            i64::from(req.Index),
            false,
            true,
            1,
            &[],
            None,
        )
        .map_err(|e| e.to_string())?;
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(RogueWhiteCatLimitedRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_rogue_white_cat_record_request(
    state: &mut State,
    req: RogueWhiteCatRecordRequest,
) -> Result<RogueWhiteCatRecordResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("rogue_cat:rec:{}", req.ActivityId),
        i64::from(req.Value),
    )
    .map_err(|e| e.to_string())?;
    Ok(RogueWhiteCatRecordResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_mul_roguelike_write_cat_reward_request(
    state: &mut State,
    req: MulRoguelikeWriteCatRewardRequest,
) -> Result<MulRoguelikeWriteCatRewardResponse, String> {
    for &idx in &req.Index {
        let key = format!("rogue_cat:rw:{}:{idx}", req.ActivityId);
        if state.player_components.progress.info.mark_claim(&key) {
            let mut rctx = state.reward_context();
            reward::grant_table_reward(
                &mut rctx,
                "RogueWhiteCatReward",
                i64::from(idx),
                false,
                true,
                1,
                &[],
                None,
            )
            .map_err(|e| e.to_string())?;
        }
    }
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(MulRoguelikeWriteCatRewardResponse {
        Index: req.Index,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_roguelike_write_cat_boss_reward_request(
    state: &mut State,
    req: RoguelikeWriteCatBossRewardRequest,
) -> Result<RoguelikeWriteCatBossRewardResponse, String> {
    let key = format!("rogue_cat:boss:{}:{}", req.ActivityId, req.Index);
    if state.player_components.progress.info.mark_claim(&key) {
        let mut rctx = state.reward_context();
        reward::grant_table_reward(
            &mut rctx,
            "RogueWhiteCatBossReward",
            i64::from(req.Index),
            false,
            true,
            1,
            &[],
            None,
        )
        .map_err(|e| e.to_string())?;
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(RoguelikeWriteCatBossRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_roguelike_write_cat_level_play_reward_request(
    state: &mut State,
    req: RoguelikeWriteCatLevelPlayRewardRequest,
) -> Result<RoguelikeWriteCatLevelPlayRewardResponse, String> {
    let key = format!("rogue_cat:lp:{}:{}", req.ActivityId, req.Index);
    if state.player_components.progress.info.mark_claim(&key) {
        let mut rctx = state.reward_context();
        reward::grant_table_reward(
            &mut rctx,
            "RogueWhiteCatReward",
            i64::from(req.Index),
            false,
            true,
            1,
            &[],
            None,
        )
        .map_err(|e| e.to_string())?;
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(RoguelikeWriteCatLevelPlayRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_rover_rogue_ins_list_request(
    state: &mut State,
    _req: RoverRogueInsListRequest,
) -> Result<RoverRogueInsListResponse, String> {
    let list = state
        .assets
        .reward_index
        .get_rows("RoverRogueIns")
        .iter()
        .map(|row| RoverRogueInsEntry {
            instId: row.key as i32,
            Level: 1,
            Unlocked: true,
            BestGrade: Some(RoverRogueGrade::RoverRogueGradeS),
            ..Default::default()
        })
        .collect();
    Ok(RoverRogueInsListResponse {
        ErrorCode: Some(ErrorCode::Success),
        List: list,
    })
}

pub fn handle_rover_rogue_bless_group_select_request(
    state: &mut State,
    events: &mut EventQueue,
    req: RoverRogueBlessGroupSelectRequest,
) -> Result<RoverRogueBlessGroupSelectResponse, String> {
    let info = &mut state.player_components.progress.info;
    info.set_key_int("rover_rogue:bless_group", i64::from(req.BlessGroupId));
    info.mark_claim("rover_rogue:select:BlessGroup");
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    if state.scene.is_some() {
        let mut qctx = state.quest_context(events);
        let _ = quest_engine::events::on_condition_state_changed(&mut qctx);
    }
    Ok(RoverRogueBlessGroupSelectResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_rover_rogue_choose_data_result_request(
    state: &mut State,
    events: &mut EventQueue,
    req: RoverRogueChooseDataResultRequest,
) -> Result<RoverRogueChooseDataResultResponse, String> {
    let info = &mut state.player_components.progress.info;
    info.set_key_int(&format!("rover_rogue:gain:{}", req.BindId), i64::from(req.IncId));
    info.mark_claim("rover_rogue:select:Gain");
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    if state.scene.is_some() {
        let mut qctx = state.quest_context(events);
        let _ = quest_engine::events::on_condition_state_changed(&mut qctx);
    }
    Ok(RoverRogueChooseDataResultResponse {
        IsSelect: true,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_rover_rogue_event_select_request(
    state: &mut State,
    events: &mut EventQueue,
    req: RoverRogueEventSelectRequest,
) -> Result<RoverRogueEventSelectResponse, String> {
    let info = &mut state.player_components.progress.info;
    info.set_key_int(
        &format!("rover_rogue:evt:{}", req.EventIncId),
        i64::from(req.ChoiceId),
    );
    info.mark_claim("rover_rogue:select:Event");
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    if state.scene.is_some() {
        let mut qctx = state.quest_context(events);
        let _ = quest_engine::events::on_condition_state_changed(&mut qctx);
    }
    Ok(RoverRogueEventSelectResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_rover_rogue_give_up_gain_request(
    state: &mut State,
    _req: RoverRogueGiveUpGainRequest,
) -> Result<RoverRogueGiveUpGainResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int("rover_rogue:giveup_cnt", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RoverRogueGiveUpGainResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_rover_rogue_goto_next_room_commit_request(
    state: &mut State,
    _req: RoverRogueGotoNextRoomCommitRequest,
) -> Result<RoverRogueGotoNextRoomCommitResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int("rover_rogue:layer", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RoverRogueGotoNextRoomCommitResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_rover_rogue_loot_info_request(
    state: &mut State,
    _req: RoverRogueLootInfoRequest,
) -> Result<RoverRogueLootInfoResponse, String> {
    let loots = state
        .player_components
        .progress
        .info
        .dynamic
        .claims
        .iter()
        .filter_map(|c| {
            let lid = c.strip_prefix("rover_rogue:loot:")?.parse::<i32>().ok()?;
            Some(RoverRogueGainEntry {
                ConfigId: lid,
                IncId: lid,
                ..Default::default()
            })
        })
        .collect();
    Ok(RoverRogueLootInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        Loots: loots,
    })
}

pub fn handle_rover_rogue_loot_change_request(
    state: &mut State,
    req: RoverRogueLootChangeRequest,
) -> Result<RoverRogueLootChangeResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("rover_rogue:loot:{}", req.IncId),
        i64::from(req.LootId),
    )
    .map_err(|e| e.to_string())?;
    Ok(RoverRogueLootChangeResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_rover_rogue_refresh_gain_request(
    state: &mut State,
    req: RoverRogueRefreshGainRequest,
) -> Result<RoverRogueRefreshGainResponse, String> {
    let cnt = state
        .player_components
        .progress
        .info
        .add_key_int("rover_rogue:gain_refresh", 1) as i32;
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    let layer = state
        .player_components
        .progress
        .info
        .get_key_int("rover_rogue:layer")
        .max(1) as i32;
    Ok(RoverRogueRefreshGainResponse {
        ErrorCode: Some(ErrorCode::Success),
        ChooseData: Some(RoverRogueChooseData {
            BindId: req.BindId,
            Layer: layer,
            MaxTime: 3,
            UseTime: cnt,
            ..Default::default()
        }),
    })
}

pub fn handle_rover_rogue_shop_refresh_request(
    state: &mut State,
    _req: RoverRogueShopRefreshRequest,
) -> Result<RoverRogueShopRefreshResponse, String> {
    let cnt = state
        .player_components
        .progress
        .info
        .add_key_int("rover_rogue:shop_refresh", 1) as i32;
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RoverRogueShopRefreshResponse {
        ErrorCode: Some(ErrorCode::Success),
        Snapshot: Some(RoverRogueShopSnapshot {
            RefreshCount: cnt,
            MaxRefreshCount: 5,
            ..Default::default()
        }),
    })
}

pub fn handle_rover_rogue_shop_buy_request(
    state: &mut State,
    events: &mut EventQueue,
    req: RoverRogueShopBuyRequest,
) -> Result<RoverRogueShopBuyResponse, String> {
    let info = &mut state.player_components.progress.info;
    info.mark_claim(&format!("rover_rogue:buy:{}", req.GoodIncId));
    info.mark_claim("rover_rogue:select:Shop");
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    if state.scene.is_some() {
        let mut qctx = state.quest_context(events);
        let _ = quest_engine::events::on_condition_state_changed(&mut qctx);
    }
    Ok(RoverRogueShopBuyResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_rover_rogue_talent_unlock_request(
    state: &mut State,
    req: RoverRogueTalentUnlockRequest,
) -> Result<RoverRogueTalentUnlockResponse, String> {
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        &format!("rover_rogue:talent:{}", req.TalentId),
    )
    .map_err(|e| e.to_string())?;
    Ok(RoverRogueTalentUnlockResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_rover_rogue_task_reward_take_request(
    state: &mut State,
    req: RoverRogueTaskRewardTakeRequest,
) -> Result<RoverRogueTaskRewardTakeResponse, String> {
    for &rid in &req.RewardIds {
        let key = format!("rover_rogue:task:{}:{rid}", req.ActivityId);
        if state.player_components.progress.info.mark_claim(&key) {
            let mut rctx = state.reward_context();
            reward::grant_table_reward(
                &mut rctx,
                "RoverRogueReward",
                i64::from(rid),
                false,
                true,
                1,
                &[],
                None,
            )
            .map_err(|e| e.to_string())?;
        }
    }
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RoverRogueTaskRewardTakeResponse {
        ErrorCode: Some(ErrorCode::Success),
        SuccessRewardIds: req.RewardIds,
    })
}

pub fn handle_rover_rogue_result_request(
    state: &mut State,
    req: RoverRogueResultRequest,
) -> Result<RoverRogueResultResponse, String> {
    let layer = state
        .player_components
        .progress
        .info
        .get_key_int("rover_rogue:layer")
        .max(1) as i32;
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "rover_rogue:layer",
        0,
    )
    .map_err(|e| e.to_string())?;
    Ok(RoverRogueResultResponse {
        ErrorCode: Some(ErrorCode::Success),
        ResultInfo: Some(RoverRogueResultInfo {
            InstId: req.InstId,
            Grade: Some(RoverRogueGrade::RoverRogueGradeS),
            CurLayer: layer,
            MaxLayer: 5,
            RoomPassedCount: layer,
            ..Default::default()
        }),
    })
}

pub fn handle_rover_rogue_quit_request(
    state: &mut State,
    _req: RoverRogueQuitRequest,
) -> Result<RoverRogueQuitResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "rover_rogue:layer",
        0,
    )
    .map_err(|e| e.to_string())?;
    Ok(RoverRogueQuitResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}
