use sonoro_gamesv_data::Assets;
use sonoro_gamesv_fs::role_info::RoleInfo;
use sonoro_gamesv_logic::component::player::PlayerProgressComponent;
use sonoro_gamesv_logic::helpers::condition::EvalContext as CondEvalContext;
use sonoro_gamesv_logic::helpers::reward::RewardItem;
use sonoro_gamesv_logic::helpers::{condition as cond_helper, pb_kv, reward};
use sonoro_proto::enums::ErrorCode;
use sonoro_proto::pb::{
    ArrayIntInt, BuildingInfo, EntrustInfo, EntrustRoleInfo, InfrArchiveReadRequest,
    InfrArchiveReadResponse, InfrArchiveTaskRewardRequest, InfrArchiveTaskRewardResponse,
    InfrFireNoticeRequest, InfrFireNoticeResponse, InfrLevelUpRequest, InfrLevelUpResponse,
    InfrManualCancelTraceRoadRequest, InfrManualCancelTraceRoadResponse,
    InfrManualSwitchTraceRoadRequest, InfrManualSwitchTraceRoadResponse, InfrNotice,
    InfrPhoneTaskRewardRequest, InfrPhoneTaskRewardResponse, InfrRoadBuildRequest,
    InfrRoadBuildResponse, InfrThemeActivityAwardRequest, InfrThemeActivityAwardResponse,
    InfrV2LevelUpRequest, InfrV2LevelUpResponse, InfrV2ManualCancelTraceTreeRequest,
    InfrV2ManualCancelTraceTreeResponse, InfrV2ManualSwitchTraceTreeRequest,
    InfrV2ManualSwitchTraceTreeResponse, InfrV2ScoreRewardRequest, InfrV2ScoreRewardResponse,
    InfrV2TaskRewardRequest, InfrV2TaskRewardResponse, InfrV2TreeBuildRequest,
    InfrV2TreeBuildResponse, NewBieCourseRewardRequest, NewBieCourseRewardResponse,
    NewJourneyRequest, NewJourneyResponse, NewJourneyTask, NewPlayerSupportRewardTaskRequest,
    NewPlayerSupportRewardTaskResponse, NewPlayerSupportSetCurUseTrialRoleRequest,
    NewPlayerSupportSetCurUseTrialRoleResponse, NewPlayerSupportSetCurUseTrialRoleV2Request,
    NewPlayerSupportSetCurUseTrialRoleV2Response, NewPlayerSupportTrialRoleLvUpRequest,
    NewPlayerSupportTrialRoleLvUpResponse, NewPlayerSupportTrialRoleLvUpV2Request,
    NewPlayerSupportTrialRoleLvUpV2Response, NewTrialRoleGetNightmarePhantomInstInfoRequest,
    NewTrialRoleGetNightmarePhantomInstInfoResponse, NewbieAdventureChapterDropRequest,
    NewbieAdventureChapterDropResponse, NewbieAdventureRoleRewardRequest,
    NewbieAdventureRoleRewardResponse, NewbieAdventureTaskRewardRequest,
    NewbieAdventureTaskRewardResponse, NewbieCarnivalAwardRequest, NewbieCarnivalAwardResponse,
    NewbieCarnivalSwitchRoleRequest, NewbieCarnivalSwitchRoleResponse,
    NewbieCourseV2RewardRequest, NewbieCourseV2RewardResponse, NewbieFetchScoreRewardRequest,
    NewbieFetchScoreRewardResponse, NightmarePhantomInstInfo, QueryRecallItemBagRequest,
    QueryRecallItemBagResponse, RealmBetweenFullRewardRequest, RealmBetweenFullRewardResponse,
    RealmBetweenLevelUpRequest, RealmBetweenLevelUpResponse, RealmBetweenMotorRewardRequest,
    RealmBetweenMotorRewardResponse, RealmBetweenTaskRewardRequest, RealmBetweenTaskRewardResponse,
    RecallExploreSkillRouletteSetRequest, RecallExploreSkillRouletteSetResponse,
    RecallItemBagItem, RecallItemUseRequest, RecallItemUseResponse, RecallQuestActionRequest,
    RecallQuestActionResponse, RecallQuestFinishActionEndRequest,
    RecallQuestFinishActionEndResponse, RecallQuestFinishActionRequest,
    RecallQuestFinishActionResponse, RecallQuestNpcMoveOverRequest,
    RecallQuestNpcMoveOverResponse, RecallQuestReadyForGiveUpRequest,
    RecallQuestReadyForGiveUpResponse, RecallTeleportToMarkIdRequest,
    RecallTeleportToMarkIdResponse, RoadBookFullRewardRequest, RoadBookFullRewardResponse,
    RoadBookLevelUpRequest, RoadBookLevelUpResponse, RoadBookMotorRewardRequest,
    RoadBookMotorRewardResponse, RoadBookTaskRewardRequest, RoadBookTaskRewardResponse,
    RoleProperSettle, SettleInfo, SolarSchoolActivityPb, SolarSchoolBatchSyncRequest,
    SolarSchoolBatchSyncResponse, SolarSchoolFlowPb, SolarSchoolLoadRequest,
    SolarSchoolLoadResponse, SolarSchoolOneKeyGalleryRewardRequest,
    SolarSchoolOneKeyGalleryRewardResponse, SolarSchoolOneKeyRewardRequest,
    SolarSchoolOneKeyRewardResponse, SolarSchoolReachEndingRequest,
    SolarSchoolReachEndingResponse, SolarSchoolSavePb, SolarSchoolSaveRequest,
    SolarSchoolSaveResponse, SolarSchoolSetFlowRequest, SolarSchoolSetFlowResponse,
    SolarSchoolTaskProgressRewardRequest, SolarSchoolTaskProgressRewardResponse,
    StartRecallRequest, StartRecallResponse, ThroughTrainFinishViewRequest,
    ThroughTrainFinishViewResponse, ThroughTrainRequest, ThroughTrainResponse,
    TraceRecallQuestRequest, TraceRecallQuestResponse, TrackMoonActivityRewardRequest,
    TrackMoonActivityRewardResponse, TrackMoonBuildFlowRequest, TrackMoonBuildFlowResponse,
    TrackMoonBuildingDataRequest, TrackMoonBuildingDataResponse, TrackMoonEntrustDataRequest,
    TrackMoonEntrustDataResponse, TrackMoonIllustratedRequest, TrackMoonIllustratedResponse,
    TrackMoonRoleDataRequest, TrackMoonRoleDataResponse, TrackMoonSettleRequest,
    TrackMoonSettleResponse, TrackMoonTargetRewardRequest, TrackMoonTargetRewardResponse,
    roleInfo,
};
use sonoro_proto::wire::PbEnum;

use super::State;

fn claim_ids(
    state: &mut State,
    prefix: &str,
    table_name: &str,
    ids: &[i32],
) -> Result<Vec<i32>, String> {
    let mut claimed = Vec::new();
    let mut dirty = false;
    let fallback = [
        RewardItem {
            item_id: 3,
            count: 40,
        },
        RewardItem {
            item_id: 2,
            count: 10000,
        },
    ];
    for &id in ids {
        let key = format!("{prefix}:{id}");
        if !state.player_components.progress.info.mark_claim(&key) {
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
            1,
            &fallback,
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

pub fn handle_infr_archive_read_request(
    state: &mut State,
    req: InfrArchiveReadRequest,
) -> Result<InfrArchiveReadResponse, String> {
    for id in req.ArchiveId {
        state
            .player_components
            .progress
            .info
            .mark_claim(&format!("infr:arch:{id}"));
    }
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(InfrArchiveReadResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_infr_archive_task_reward_request(
    state: &mut State,
    req: InfrArchiveTaskRewardRequest,
) -> Result<InfrArchiveTaskRewardResponse, String> {
    claim_ids(state, "infr:arch_rwd", "InfrArchiveTask", &req.TargetIds)?;
    Ok(InfrArchiveTaskRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_infr_fire_notice_request(
    state: &mut State,
    _req: InfrFireNoticeRequest,
) -> Result<InfrFireNoticeResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int("infr:fire_notice_ack", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    let mut notices = Vec::new();
    if state.player_components.progress.info.get_key_int("infr:lvl") > 0 {
        notices.push(InfrNotice::default());
    }
    Ok(InfrFireNoticeResponse { Notices: notices })
}

pub fn handle_infr_level_up_request(
    state: &mut State,
    _req: InfrLevelUpRequest,
) -> Result<InfrLevelUpResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int("infr:lvl", 1);
    state
        .player_components
        .progress
        .info
        .add_key_int("infr:exp", 100);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(InfrLevelUpResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_infr_manual_cancel_trace_road_request(
    state: &mut State,
    _req: InfrManualCancelTraceRoadRequest,
) -> Result<InfrManualCancelTraceRoadResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "infr:trace_road",
        0,
    )
    .map_err(|e| e.to_string())?;
    Ok(InfrManualCancelTraceRoadResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_infr_manual_switch_trace_road_request(
    state: &mut State,
    req: InfrManualSwitchTraceRoadRequest,
) -> Result<InfrManualSwitchTraceRoadResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "infr:trace_road",
        i64::from(req.RoadId),
    )
    .map_err(|e| e.to_string())?;
    Ok(InfrManualSwitchTraceRoadResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_infr_phone_task_reward_request(
    state: &mut State,
    req: InfrPhoneTaskRewardRequest,
) -> Result<InfrPhoneTaskRewardResponse, String> {
    claim_ids(state, "infr:phone", "InfrPhoneTask", &req.TargetIds)?;
    Ok(InfrPhoneTaskRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_infr_road_build_request(
    state: &mut State,
    req: InfrRoadBuildRequest,
) -> Result<InfrRoadBuildResponse, String> {
    state
        .player_components
        .progress
        .info
        .mark_claim(&format!("infr:road:{}", req.RoadId));
    state
        .player_components
        .progress
        .info
        .add_key_int("infr:exp", 50);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(InfrRoadBuildResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_infr_theme_activity_award_request(
    state: &mut State,
    req: InfrThemeActivityAwardRequest,
) -> Result<InfrThemeActivityAwardResponse, String> {
    claim_ids(state, "infr:task", "InfrActivityTask", &req.TaskId)?;
    Ok(InfrThemeActivityAwardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_infr_v2_level_up_request(
    state: &mut State,
    _req: InfrV2LevelUpRequest,
) -> Result<InfrV2LevelUpResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int("infr2:lvl", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(InfrV2LevelUpResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_infr_v2_manual_cancel_trace_tree_request(
    state: &mut State,
    _req: InfrV2ManualCancelTraceTreeRequest,
) -> Result<InfrV2ManualCancelTraceTreeResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "infr2:trace_tree",
        0,
    )
    .map_err(|e| e.to_string())?;
    Ok(InfrV2ManualCancelTraceTreeResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_infr_v2_manual_switch_trace_tree_request(
    state: &mut State,
    req: InfrV2ManualSwitchTraceTreeRequest,
) -> Result<InfrV2ManualSwitchTraceTreeResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "infr2:trace_tree",
        i64::from(req.TreeId),
    )
    .map_err(|e| e.to_string())?;
    Ok(InfrV2ManualSwitchTraceTreeResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_infr_v2_score_reward_request(
    state: &mut State,
    req: InfrV2ScoreRewardRequest,
) -> Result<InfrV2ScoreRewardResponse, String> {
    claim_ids(state, "infr2:score", "InfrV2ScoreReward", &req.ScoreIds)?;
    Ok(InfrV2ScoreRewardResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_infr_v2_task_reward_request(
    state: &mut State,
    req: InfrV2TaskRewardRequest,
) -> Result<InfrV2TaskRewardResponse, String> {
    claim_ids(state, "infr2:task", "InfrV2Reward", &req.TaskIds)?;
    Ok(InfrV2TaskRewardResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_infr_v2_tree_build_request(
    state: &mut State,
    req: InfrV2TreeBuildRequest,
) -> Result<InfrV2TreeBuildResponse, String> {
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        &format!("infr2:tree:{}", req.Id),
    )
    .map_err(|e| e.to_string())?;
    Ok(InfrV2TreeBuildResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_solar_school_batch_sync_request(
    state: &mut State,
    req: SolarSchoolBatchSyncRequest,
) -> Result<SolarSchoolBatchSyncResponse, String> {
    let act_id = req.ActivityId;
    let info = &mut state.player_components.progress.info;
    info.set_key_int(&format!("solar:day:{act_id}"), i64::from(req.GameDayNow));
    for entry in req.SyncData {
        if let Some(ac) = entry.AttrChange {
            let at = ac.AttrType.map_or(0, |t| t.to_i32());
            info.add_key_int(&format!("solar:attr:{act_id}:{at}"), i64::from(ac.Delta));
        }
        if let Some(un) = entry.UnlockNpc {
            for nid in un.NpcId {
                info.mark_claim(&format!("solar:npc:{act_id}:{nid}"));
            }
        }
        if let Some(nfc) = entry.NpcFavorChange {
            info.set_key_int(
                &format!("solar:favor:{act_id}:{}", nfc.NpcId),
                i64::from(nfc.FavorAbsolute),
            );
        }
        if let Some(ji) = entry.JoinInfluence {
            for iid in ji.InfluenceId {
                info.mark_claim(&format!("solar:inf:{act_id}:{iid}"));
            }
        }
        if let Some(ef) = entry.ExamFinish {
            info.set_key_int(
                &format!("solar:exam:{act_id}:{}", ef.ExamId),
                i64::from(ef.Score),
            );
        }
        if let Some(sb) = entry.ShopBuy {
            info.add_key_int(
                &format!("solar:shop:{act_id}:{}:{}", sb.ShopId, sb.GoodsId),
                i64::from(sb.Count),
            );
        }
        if let Some(cq) = entry.CompleteQuest {
            for qid in cq.QuestId {
                info.mark_claim(&format!("solar:quest:{act_id}:{qid}"));
            }
        }
        if let Some(ic) = entry.ItemChange
            && let Some(it) = ic.Item
        {
            info.add_key_int(
                &format!("solar:item:{act_id}:{}", it.ItemId),
                i64::from(it.ItemAdd) - i64::from(it.ItemCost),
            );
        }
        if let Some(bh) = entry.Behave {
            info.set_key_int(
                &format!("solar:behave:{act_id}:{}", bh.BehaviorId),
                i64::from(bh.IsSuccess),
            );
        }
    }
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(SolarSchoolBatchSyncResponse {
        ErrorCode: Some(ErrorCode::Success),
        FailedIndex: -1,
    })
}

pub fn handle_solar_school_load_request(
    state: &mut State,
    req: SolarSchoolLoadRequest,
) -> Result<SolarSchoolLoadResponse, String> {
    let pc = &state.player_components.progress;
    let slot_key = format!("solar:save:{}:{}", req.ActivityId, req.SlotId);
    let latest_key = format!("solar:save:{}", req.ActivityId);
    let loaded = pb_kv::load_msg::<SolarSchoolSavePb>(pc, &slot_key)
        .or_else(|| pb_kv::load_msg::<SolarSchoolSavePb>(pc, &latest_key));
    Ok(SolarSchoolLoadResponse {
        ErrorCode: Some(ErrorCode::Success),
        Flow: Vec::new(),
        SavePb: Some(loaded.unwrap_or_else(|| SolarSchoolSavePb {
            ActivityId: req.ActivityId,
            ..Default::default()
        })),
    })
}

pub fn handle_solar_school_one_key_gallery_reward_request(
    state: &mut State,
    req: SolarSchoolOneKeyGalleryRewardRequest,
) -> Result<SolarSchoolOneKeyGalleryRewardResponse, String> {
    claim_ids(state, "solar:gallery", "SolarSchoolEnding", &req.Id)?;
    Ok(SolarSchoolOneKeyGalleryRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_solar_school_one_key_reward_request(
    state: &mut State,
    req: SolarSchoolOneKeyRewardRequest,
) -> Result<SolarSchoolOneKeyRewardResponse, String> {
    claim_ids(state, "solar:task", "SolarSchoolTask", &req.TaskId)?;
    Ok(SolarSchoolOneKeyRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_solar_school_reach_ending_request(
    state: &mut State,
    req: SolarSchoolReachEndingRequest,
) -> Result<SolarSchoolReachEndingResponse, String> {
    for id in req.Id {
        state
            .player_components
            .progress
            .info
            .mark_claim(&format!("solar:ending:{id}"));
    }
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(SolarSchoolReachEndingResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_solar_school_save_request(
    state: &mut State,
    req: SolarSchoolSaveRequest,
) -> Result<SolarSchoolSaveResponse, String> {
    let pc = &mut state.player_components.progress;
    pc.info.set_key_int(
        &format!("solar:slot:{}", req.ActivityId),
        i64::from(req.LoadSourceSlotId),
    );
    if let Some(save_pb) = req.SavePb {
        pb_kv::save_msg(
            &state.fs,
            pc,
            &format!("solar:save:{}", req.ActivityId),
            &save_pb,
        )
        .map_err(|e| e.to_string())?;
        if req.LoadSourceSlotId >= 0 {
            pb_kv::save_msg(
                &state.fs,
                pc,
                &format!("solar:save:{}:{}", req.ActivityId, req.LoadSourceSlotId),
                &save_pb,
            )
            .map_err(|e| e.to_string())?;
        }
    } else {
        pc.save(&state.fs).map_err(|e| e.to_string())?;
    }
    let mut flows = Vec::new();
    let claims: Vec<String> = pc.info.dynamic.claims.clone();
    for c in claims {
        if let Some(rest) = c.strip_prefix("solar:flow:")
            && let Ok(fid) = rest.parse::<i32>()
        {
            let fpk = format!("solar:flow_pb:{fid}");
            let fpb = pb_kv::load_msg::<SolarSchoolFlowPb>(pc, &fpk).unwrap_or(SolarSchoolFlowPb {
                SolarSchoolFlowId: fid,
                ..Default::default()
            });
            flows.push(fpb);
        }
    }
    Ok(SolarSchoolSaveResponse {
        ErrorCode: Some(ErrorCode::Success),
        Flow: flows,
    })
}

pub fn handle_solar_school_set_flow_request(
    state: &mut State,
    req: SolarSchoolSetFlowRequest,
) -> Result<SolarSchoolSetFlowResponse, String> {
    let pc = &mut state.player_components.progress;
    pc.info
        .mark_claim(&format!("solar:flow:{}", req.SolarSchoolFlowId));
    let fpk = format!("solar:flow_pb:{}", req.SolarSchoolFlowId);
    let mut flow_pb = pb_kv::load_msg::<SolarSchoolFlowPb>(pc, &fpk).unwrap_or(SolarSchoolFlowPb {
        SolarSchoolFlowId: req.SolarSchoolFlowId,
        ..Default::default()
    });
    flow_pb.SolarSchoolFlowId = req.SolarSchoolFlowId;
    for pid in req.ReadPlot {
        if pid < 0 {
            continue;
        }
        let word_idx = (pid / 64) as usize;
        let bit_idx = (pid % 64) as u32;
        if flow_pb.ReadPlotFlatBit.len() <= word_idx {
            flow_pb.ReadPlotFlatBit.resize(word_idx + 1, 0);
        }
        let cur = flow_pb.ReadPlotFlatBit[word_idx] as u64;
        flow_pb.ReadPlotFlatBit[word_idx] = (cur | (1u64 << bit_idx)) as i64;
    }
    pb_kv::save_msg(&state.fs, pc, &fpk, &flow_pb).map_err(|e| e.to_string())?;
    Ok(SolarSchoolSetFlowResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_solar_school_task_progress_reward_request(
    state: &mut State,
    req: SolarSchoolTaskProgressRewardRequest,
) -> Result<SolarSchoolTaskProgressRewardResponse, String> {
    claim_ids(
        state,
        "solar:prog",
        "SolarSchoolActivity",
        &[req.ActivityId],
    )?;
    Ok(SolarSchoolTaskProgressRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
        Activity: Some(SolarSchoolActivityPb {
            LimitTaskProgressRewarded: true,
            ..Default::default()
        }),
    })
}

pub fn handle_track_moon_activity_reward_request(
    state: &mut State,
    req: TrackMoonActivityRewardRequest,
) -> Result<TrackMoonActivityRewardResponse, String> {
    claim_ids(
        state,
        "trackmoon:task",
        "TrackMoonActivityReward",
        &[req.Id],
    )?;
    Ok(TrackMoonActivityRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_track_moon_build_flow_request(
    state: &mut State,
    req: TrackMoonBuildFlowRequest,
) -> Result<TrackMoonBuildFlowResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int(&format!("trackmoon:bld:{}", req.BuildingId), 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(TrackMoonBuildFlowResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_track_moon_building_data_request(
    state: &mut State,
    _req: TrackMoonBuildingDataRequest,
) -> Result<TrackMoonBuildingDataResponse, String> {
    let mut infos = Vec::with_capacity(10);
    for id in 1..=10 {
        let raw = state
            .player_components
            .progress
            .info
            .get_key_int(&format!("trackmoon:bld:{id}"));
        let lvl = raw.max(i64::from(id == 1)) as i32;
        infos.push(BuildingInfo {
            BuildingId: id,
            Level: lvl,
            IsUnlock: raw > 0 || id == 1,
        });
    }
    Ok(TrackMoonBuildingDataResponse {
        BuildingInfos: infos,
    })
}

pub fn handle_track_moon_entrust_data_request(
    state: &mut State,
    _req: TrackMoonEntrustDataRequest,
) -> Result<TrackMoonEntrustDataResponse, String> {
    let mut infos = Vec::with_capacity(18);
    for id in 1..=18 {
        let lvl = state
            .player_components
            .progress
            .info
            .get_key_int(&format!("trackmoon:entrust:{id}"))
            .max(0) as i32;
        infos.push(EntrustInfo {
            EntrustId: id,
            IsVisiable: true,
            HistoryLevel: lvl,
        });
    }
    Ok(TrackMoonEntrustDataResponse {
        EntrustInfos: infos,
    })
}

pub fn handle_track_moon_illustrated_request(
    state: &mut State,
    req: TrackMoonIllustratedRequest,
) -> Result<TrackMoonIllustratedResponse, String> {
    let mut dirty = false;
    for id in &req.Ids {
        if state
            .player_components
            .progress
            .info
            .mark_claim(&format!("trackmoon:illus:{id}"))
        {
            dirty = true;
        }
    }
    if dirty {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    let out_ids: Vec<i32> = state
        .player_components
        .progress
        .info
        .dynamic
        .claims
        .iter()
        .filter_map(|c| c.strip_prefix("trackmoon:illus:")?.parse::<i32>().ok())
        .collect();
    Ok(TrackMoonIllustratedResponse {
        Ids: if out_ids.is_empty() { req.Ids } else { out_ids },
    })
}

pub fn handle_track_moon_role_data_request(
    state: &mut State,
    _req: TrackMoonRoleDataRequest,
) -> Result<TrackMoonRoleDataResponse, String> {
    let pc = &state.player_components.progress.info;
    let mut roles = Vec::with_capacity(14);
    for id in 1..=14 {
        let lvl = pc.get_key_int(&format!("trackmoon:role_lvl:{id}")).max(1) as i32;
        roles.push(EntrustRoleInfo {
            Level: lvl,
            RoleAttr: Some(RoleProperSettle {
                RoleId: id,
                Level: lvl,
                PropertyA: pc.get_key_int(&format!("trackmoon:role_a:{id}")).max(100) as i32,
                PropertyB: pc.get_key_int(&format!("trackmoon:role_b:{id}")).max(100) as i32,
                PropertyC: pc.get_key_int(&format!("trackmoon:role_c:{id}")).max(100) as i32,
            }),
        });
    }
    Ok(TrackMoonRoleDataResponse {
        EntrustRoleInfos: roles,
    })
}

pub fn handle_track_moon_settle_request(
    state: &mut State,
    _req: TrackMoonSettleRequest,
) -> Result<TrackMoonSettleResponse, String> {
    let pc = &mut state.player_components.progress;
    let cnt = pc.info.add_key_int("trackmoon:settle_cnt", 1) as i32;
    pc.info.add_key_int("trackmoon:popular", 10);
    pc.save(&state.fs).map_err(|e| e.to_string())?;
    let entrust_do = pc.info.get_key_int("trackmoon:entrust_done").max(i64::from(cnt)) as i32;
    let wish_total = pc.info.get_key_int("trackmoon:wish").max(i64::from(cnt * 100)) as i32;
    let gold_total = pc.info.get_key_int("trackmoon:gold").max(i64::from(cnt * 1000)) as i32;
    let pop_total = pc.info.get_key_int("trackmoon:popular").max(i64::from(cnt * 500)) as i32;
    let fav_id = pc.info.get_key_int("trackmoon:fav_role").max(1) as i32;
    Ok(TrackMoonSettleResponse {
        ErrorCode: Some(ErrorCode::Success),
        MostLikeRole: Some(SettleInfo {
            Id: fav_id,
            Num: cnt,
        }),
        EntrustDoMost: Some(SettleInfo {
            Id: 1,
            Num: entrust_do,
        }),
        EntrustDo: entrust_do,
        AlwaysRoleId: Some(SettleInfo {
            Id: fav_id,
            Num: entrust_do,
        }),
        WishTotal: wish_total,
        GoldTotal: gold_total,
        PopularTotal: pop_total,
    })
}

pub fn handle_track_moon_target_reward_request(
    state: &mut State,
    req: TrackMoonTargetRewardRequest,
) -> Result<TrackMoonTargetRewardResponse, String> {
    claim_ids(state, "trackmoon:task", "TrackMoonTarget", &[req.Id])?;
    Ok(TrackMoonTargetRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_start_recall_request(
    state: &mut State,
    req: StartRecallRequest,
) -> Result<StartRecallResponse, String> {
    let pc = &mut state.player_components.progress;
    pc.info.set_key_int("recall:cur", i64::from(req.RecallId));
    pc.info.mark_claim(&format!("recall:started:{}", req.RecallId));
    pc.save(&state.fs).map_err(|e| e.to_string())?;
    let saved_inst = pc
        .info
        .get_key_int(&format!("recall:inst:{}", req.RecallId))
        .max(0) as i32;
    let default_inst = state
        .assets
        .tables
        .recall_config
        .get_data_by_id(&req.RecallId)
        .and_then(|rc| rc.DungeonIds.first().copied())
        .unwrap_or(0);
    let enter_inst_id = if req.Continue && saved_inst > 0 {
        saved_inst
    } else {
        default_inst
    };
    Ok(StartRecallResponse {
        ErrorCode: Some(ErrorCode::Success),
        CanContinue: req.Continue,
        EnterInstId: enter_inst_id,
    })
}

pub fn handle_query_recall_item_bag_request(
    state: &mut State,
    req: QueryRecallItemBagRequest,
) -> Result<QueryRecallItemBagResponse, String> {
    let prefix = format!("recall:item:{}:", req.RecallId);
    let items = state
        .player_components
        .progress
        .info
        .dynamic
        .ints
        .iter()
        .filter_map(|e| {
            if e.val <= 0 {
                return None;
            }
            let item_id = e.key.strip_prefix(&prefix)?.parse::<i32>().ok()?;
            Some(RecallItemBagItem {
                ItemId: item_id,
                Count: e.val as i32,
            })
        })
        .collect();
    Ok(QueryRecallItemBagResponse {
        Items: items,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_trace_recall_quest_request(
    state: &mut State,
    req: TraceRecallQuestRequest,
) -> Result<TraceRecallQuestResponse, String> {
    let traced = if req.TraceType == 2 || (req.TraceType == 0 && req.Operate == 0) {
        0
    } else {
        1
    };
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("recall:trace:{}", req.RecallQuestId),
        traced,
    )
    .map_err(|e| e.to_string())?;
    Ok(TraceRecallQuestResponse {
        ErrorId: Some(ErrorCode::Success),
    })
}

pub fn handle_recall_explore_skill_roulette_set_request(
    state: &mut State,
    req: RecallExploreSkillRouletteSetRequest,
) -> Result<RecallExploreSkillRouletteSetResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "recall:roulette",
        i64::from(req.ExtraItemId),
    )
    .map_err(|e| e.to_string())?;
    Ok(RecallExploreSkillRouletteSetResponse {
        ErrorId: Some(ErrorCode::Success),
    })
}

pub fn handle_recall_item_use_request(
    state: &mut State,
    req: RecallItemUseRequest,
) -> Result<RecallItemUseResponse, String> {
    state.player_components.progress.info.add_key_int(
        &format!("recall:item_used:{}", req.ItemId),
        i64::from(req.Count.max(1)),
    );
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RecallItemUseResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_recall_quest_action_request(
    state: &mut State,
    req: RecallQuestActionRequest,
) -> Result<RecallQuestActionResponse, String> {
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        &format!(
            "recall:qact:{}:{}:{}",
            req.RecallQuestId, req.NodeId, req.ActionId
        ),
    )
    .map_err(|e| e.to_string())?;
    Ok(RecallQuestActionResponse {
        ErrorId: Some(ErrorCode::Success),
    })
}

pub fn handle_recall_quest_finish_action_request(
    state: &mut State,
    req: RecallQuestFinishActionRequest,
) -> Result<RecallQuestFinishActionResponse, String> {
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        &format!(
            "recall:qfin:{}:{}:{}",
            req.RecallQuestId, req.NodeId, req.ActionId
        ),
    )
    .map_err(|e| e.to_string())?;
    Ok(RecallQuestFinishActionResponse {
        ErrorId: Some(ErrorCode::Success),
    })
}

pub fn handle_recall_quest_finish_action_end_request(
    state: &mut State,
    req: RecallQuestFinishActionEndRequest,
) -> Result<RecallQuestFinishActionEndResponse, String> {
    let mut dirty = false;
    for qid in req.RecallQuestId {
        if state
            .player_components
            .progress
            .info
            .mark_claim(&format!("recall:qend:{qid}"))
        {
            dirty = true;
        }
    }
    if dirty {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(RecallQuestFinishActionEndResponse {
        ErrorId: Some(ErrorCode::Success),
    })
}

pub fn handle_recall_quest_npc_move_over_request(
    state: &mut State,
    req: RecallQuestNpcMoveOverRequest,
) -> Result<RecallQuestNpcMoveOverResponse, String> {
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        &format!("recall:npcmove:{}", req.EntityId),
    )
    .map_err(|e| e.to_string())?;
    Ok(RecallQuestNpcMoveOverResponse {
        ErrorId: Some(ErrorCode::Success),
    })
}

pub fn handle_recall_quest_ready_for_give_up_request(
    state: &mut State,
    req: RecallQuestReadyForGiveUpRequest,
) -> Result<RecallQuestReadyForGiveUpResponse, String> {
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        &format!("recall:giveup:{}", req.RecallQuestId),
    )
    .map_err(|e| e.to_string())?;
    Ok(RecallQuestReadyForGiveUpResponse {
        ErrorId: Some(ErrorCode::Success),
    })
}

pub fn handle_recall_teleport_to_mark_id_request(
    state: &mut State,
    req: RecallTeleportToMarkIdRequest,
) -> Result<RecallTeleportToMarkIdResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "recall:tp_mark",
        i64::from(req.MarkId),
    )
    .map_err(|e| e.to_string())?;
    Ok(RecallTeleportToMarkIdResponse {
        ErrorId: Some(ErrorCode::Success),
    })
}

pub fn handle_road_book_full_reward_request(
    state: &mut State,
    _req: RoadBookFullRewardRequest,
) -> Result<RoadBookFullRewardResponse, String> {
    if state
        .player_components
        .progress
        .info
        .mark_claim("roadbook:full")
    {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        let mut rctx = state.reward_context();
        reward::grant_items(
            &mut rctx,
            0,
            true,
            &[RewardItem {
                item_id: 3,
                count: 160,
            }],
            1,
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(RoadBookFullRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_road_book_level_up_request(
    state: &mut State,
    _req: RoadBookLevelUpRequest,
) -> Result<RoadBookLevelUpResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int("roadbook:lvl", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RoadBookLevelUpResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_road_book_motor_reward_request(
    state: &mut State,
    req: RoadBookMotorRewardRequest,
) -> Result<RoadBookMotorRewardResponse, String> {
    let claimed = claim_ids(state, "roadbook:motor", "RoadBookTask", &req.TaskIds)?;
    Ok(RoadBookMotorRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
        TaskIds: claimed,
    })
}

pub fn handle_road_book_task_reward_request(
    state: &mut State,
    req: RoadBookTaskRewardRequest,
) -> Result<RoadBookTaskRewardResponse, String> {
    let claimed = claim_ids(state, "roadbook:task", "RoadBookTask", &req.TaskIds)?;
    Ok(RoadBookTaskRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
        TaskIds: claimed,
    })
}

pub fn handle_realm_between_full_reward_request(
    state: &mut State,
    _req: RealmBetweenFullRewardRequest,
) -> Result<RealmBetweenFullRewardResponse, String> {
    if state
        .player_components
        .progress
        .info
        .mark_claim("realmbetween:full")
    {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        let mut rctx = state.reward_context();
        reward::grant_items(
            &mut rctx,
            0,
            true,
            &[RewardItem {
                item_id: 3,
                count: 160,
            }],
            1,
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(RealmBetweenFullRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_realm_between_level_up_request(
    state: &mut State,
    _req: RealmBetweenLevelUpRequest,
) -> Result<RealmBetweenLevelUpResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int("realmbetween:lvl", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RealmBetweenLevelUpResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_realm_between_motor_reward_request(
    state: &mut State,
    req: RealmBetweenMotorRewardRequest,
) -> Result<RealmBetweenMotorRewardResponse, String> {
    let claimed = claim_ids(state, "realmbetween:motor", "RoadBookTask", &req.TaskIds)?;
    Ok(RealmBetweenMotorRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
        TaskIds: claimed,
    })
}

pub fn handle_realm_between_task_reward_request(
    state: &mut State,
    req: RealmBetweenTaskRewardRequest,
) -> Result<RealmBetweenTaskRewardResponse, String> {
    let claimed = claim_ids(state, "realm:task", "RoadBookTask", &req.TaskIds)?;
    Ok(RealmBetweenTaskRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
        TaskIds: claimed,
    })
}

pub fn handle_new_bie_course_reward_request(
    state: &mut State,
    req: NewBieCourseRewardRequest,
) -> Result<NewBieCourseRewardResponse, String> {
    claim_ids(state, "newbie:course", "NewBieCourse", &[req.Level])?;
    Ok(NewBieCourseRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_new_journey_request(
    state: &mut State,
    req: NewJourneyRequest,
) -> Result<NewJourneyResponse, String> {
    let mut cond_ctx = CondEvalContext::new(
        &state.player_components.basic,
        &state.player_components.echo,
        &state.player_components.inventory,
        &state.player_components.map,
        &state.player_components.quest,
    );
    cond_ctx.assets = Some(&state.assets);
    cond_ctx.progress_comp = Some(&state.player_components.progress);

    let mut tasks = Vec::new();
    if req.TaskIds.is_empty() {
        for cfg in &state.assets.tables.world_new_journey.items {
            let cond_ok =
                cfg.ConditionId <= 0 || cond_helper::eval_condition_or_group(&cond_ctx, cfg.ConditionId);
            let claimed = state
                .player_components
                .progress
                .info
                .has_claim(&format!("newbie:journey:{}", cfg.Id));
            tasks.push(NewJourneyTask {
                TaskIdx: cfg.Id,
                CanJump: cond_ok,
                Complete: claimed || (cfg.ConditionId > 0 && cond_ok),
            });
        }
    } else {
        for id in req.TaskIds {
            let cfg = state.assets.tables.world_new_journey.get_data_by_id(&id);
            let cond_ok = cfg.as_ref().is_none_or(|c| {
                c.ConditionId <= 0 || cond_helper::eval_condition_or_group(&cond_ctx, c.ConditionId)
            });
            let claimed = state
                .player_components
                .progress
                .info
                .has_claim(&format!("newbie:journey:{id}"));
            tasks.push(NewJourneyTask {
                TaskIdx: id,
                CanJump: cond_ok,
                Complete: claimed || cfg.is_some_and(|c| c.ConditionId > 0 && cond_ok),
            });
        }
    }
    Ok(NewJourneyResponse {
        NewJourneyTask: tasks,
    })
}

pub fn handle_new_player_support_reward_task_request(
    state: &mut State,
    req: NewPlayerSupportRewardTaskRequest,
) -> Result<NewPlayerSupportRewardTaskResponse, String> {
    claim_ids(
        state,
        "newbie:support_task",
        "NewPlayerSupportTask",
        &req.TaskIds,
    )?;
    Ok(NewPlayerSupportRewardTaskResponse {
        Code: Some(ErrorCode::Success),
        ..Default::default()
    })
}

fn build_support_trial_role_info(
    assets: &Assets,
    progress_comp: &PlayerProgressComponent,
    lv_prefix: &str,
    trial_role_id: i32,
) -> roleInfo {
    let stored_lv = progress_comp
        .info
        .get_key_int(&format!("{lv_prefix}:{trial_role_id}"));
    let trial_opt = assets.tables.trial_role_info.get_data_by_id(&trial_role_id);
    let lv = if stored_lv > 0 {
        stored_lv.min(90) as i32
    } else if let Some(ref t) = trial_opt {
        t.Level.max(1)
    } else {
        80
    };

    let mut base_props = Vec::new();
    let mut add_props = Vec::new();
    let mut breach = 0;
    let mut star = 4;
    let mut skin_id = 0;
    if let Some(base_cfg) = assets.tables.resolve_base_role_info(trial_role_id) {
        star = base_cfg.QualityId.max(4);
        skin_id = trial_opt
            .filter(|t| t.RoleSkin != 0)
            .map_or(base_cfg.SkinId, |t| t.RoleSkin);
        for entry in &assets.tables.role_breach.items {
            if entry.BreachGroupId == base_cfg.BreachId
                && lv > entry.MaxLevel
                && entry.BreachLevel + 1 > breach
            {
                breach = entry.BreachLevel + 1;
            }
        }
        if let Some(prop_buf) = RoleInfo::compute_scaled_base_props(assets, base_cfg.Id, lv, breach)
        {
            for (idx, &val) in prop_buf.iter().enumerate() {
                base_props.push(ArrayIntInt {
                    Key: idx as i32,
                    Value: val,
                });
                add_props.push(ArrayIntInt {
                    Key: idx as i32,
                    Value: 0,
                });
            }
        }
    }

    roleInfo {
        RoleId: trial_role_id,
        Level: lv,
        Breakthrough: breach,
        Star: star,
        SkinId: skin_id,
        BaseProp: base_props,
        AddProp: add_props,
        ..Default::default()
    }
}

pub fn handle_new_player_support_set_cur_use_trial_role_request(
    state: &mut State,
    req: NewPlayerSupportSetCurUseTrialRoleRequest,
) -> Result<NewPlayerSupportSetCurUseTrialRoleResponse, String> {
    let prev = state
        .player_components
        .progress
        .info
        .get_key_int("newbie:support_role") as i32;
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "newbie:support_role",
        i64::from(req.TrialRoleId),
    )
    .map_err(|e| e.to_string())?;
    let info = build_support_trial_role_info(
        &state.assets,
        &state.player_components.progress,
        "newbie:support_role_lv",
        req.TrialRoleId,
    );
    Ok(NewPlayerSupportSetCurUseTrialRoleResponse {
        Code: Some(ErrorCode::Success),
        RoleId: req.TrialRoleId,
        ReplacedRoleId: prev,
        ReplacedCurRole: prev != 0 && prev != req.TrialRoleId,
        CurUseRoleInfo: Some(info),
    })
}

pub fn handle_new_player_support_set_cur_use_trial_role_v2_request(
    state: &mut State,
    req: NewPlayerSupportSetCurUseTrialRoleV2Request,
) -> Result<NewPlayerSupportSetCurUseTrialRoleV2Response, String> {
    let prev = state
        .player_components
        .progress
        .info
        .get_key_int("newbie:support_role_v2") as i32;
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "newbie:support_role_v2",
        i64::from(req.TrialRoleId),
    )
    .map_err(|e| e.to_string())?;
    let info = build_support_trial_role_info(
        &state.assets,
        &state.player_components.progress,
        "newbie:support_role_v2_lv",
        req.TrialRoleId,
    );
    Ok(NewPlayerSupportSetCurUseTrialRoleV2Response {
        Code: Some(ErrorCode::Success),
        RoleId: req.TrialRoleId,
        ReplacedRoleId: prev,
        ReplacedCurRole: prev != 0 && prev != req.TrialRoleId,
        CurUseRoleInfo: Some(info),
    })
}

pub fn handle_new_player_support_trial_role_lv_up_request(
    state: &mut State,
    req: NewPlayerSupportTrialRoleLvUpRequest,
) -> Result<NewPlayerSupportTrialRoleLvUpResponse, String> {
    let key = format!("newbie:support_role_lv:{}", req.TrialRoleId);
    let prev = state.player_components.progress.info.get_key_int(&key);
    let next_lv = (prev.max(1) + 10).min(90);
    pb_kv::persist_int(&state.fs, &mut state.player_components.progress, &key, next_lv)
        .map_err(|e| e.to_string())?;
    let info = build_support_trial_role_info(
        &state.assets,
        &state.player_components.progress,
        "newbie:support_role_lv",
        req.TrialRoleId,
    );
    Ok(NewPlayerSupportTrialRoleLvUpResponse {
        Code: Some(ErrorCode::Success),
        RoleId: req.TrialRoleId,
        CurUseRoleInfo: Some(info),
    })
}

pub fn handle_new_player_support_trial_role_lv_up_v2_request(
    state: &mut State,
    req: NewPlayerSupportTrialRoleLvUpV2Request,
) -> Result<NewPlayerSupportTrialRoleLvUpV2Response, String> {
    let key = format!("newbie:support_role_v2_lv:{}", req.TrialRoleId);
    let prev = state.player_components.progress.info.get_key_int(&key);
    let next_lv = (prev.max(1) + 10).min(90);
    pb_kv::persist_int(&state.fs, &mut state.player_components.progress, &key, next_lv)
        .map_err(|e| e.to_string())?;
    let info = build_support_trial_role_info(
        &state.assets,
        &state.player_components.progress,
        "newbie:support_role_v2_lv",
        req.TrialRoleId,
    );
    Ok(NewPlayerSupportTrialRoleLvUpV2Response {
        RoleId: req.TrialRoleId,
        Code: Some(ErrorCode::Success),
        CurUseRoleInfo: Some(info),
    })
}

pub fn handle_new_trial_role_get_nightmare_phantom_inst_info_request(
    state: &mut State,
    _req: NewTrialRoleGetNightmarePhantomInstInfoRequest,
) -> Result<NewTrialRoleGetNightmarePhantomInstInfoResponse, String> {
    let mut ins_list = Vec::new();
    let mut save_ids = Vec::new();
    for entry in &state.player_components.progress.info.dynamic.ints {
        if let Some(rest) = entry.key.strip_prefix("newbie:nightmare_kill:") {
            if let Ok(ins_id) = rest.parse::<i32>() {
                ins_list.push(NightmarePhantomInstInfo {
                    InsId: ins_id,
                    KillNum: entry.val,
                });
            }
        } else if let Some(rest) = entry.key.strip_prefix("newbie:nightmare_save:")
            && entry.val > 0
            && let Ok(ins_id) = rest.parse::<i32>()
        {
            save_ids.push(ins_id);
        }
    }
    Ok(NewTrialRoleGetNightmarePhantomInstInfoResponse {
        Code: Some(ErrorCode::Success),
        InsInfoList: ins_list,
        SaveInsIdList: save_ids,
    })
}

pub fn handle_newbie_adventure_chapter_drop_request(
    state: &mut State,
    req: NewbieAdventureChapterDropRequest,
) -> Result<NewbieAdventureChapterDropResponse, String> {
    claim_ids(
        state,
        "newbie:adv_chap",
        "AdventureTaskChapter",
        &[req.ChapterId],
    )?;
    Ok(NewbieAdventureChapterDropResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_newbie_adventure_role_reward_request(
    state: &mut State,
    req: NewbieAdventureRoleRewardRequest,
) -> Result<NewbieAdventureRoleRewardResponse, String> {
    claim_ids(state, "newbie:adv_role", "AdventureRole", &[req.RoleId])?;
    Ok(NewbieAdventureRoleRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_newbie_adventure_task_reward_request(
    state: &mut State,
    req: NewbieAdventureTaskRewardRequest,
) -> Result<NewbieAdventureTaskRewardResponse, String> {
    claim_ids(state, "newbie:adv_task", "AdventureTask", &req.TaskIds)?;
    Ok(NewbieAdventureTaskRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_newbie_carnival_award_request(
    state: &mut State,
    req: NewbieCarnivalAwardRequest,
) -> Result<NewbieCarnivalAwardResponse, String> {
    claim_ids(state, "newbie:carnival", "NewbieCarnival", &[req.Config])?;
    Ok(NewbieCarnivalAwardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_newbie_carnival_switch_role_request(
    state: &mut State,
    req: NewbieCarnivalSwitchRoleRequest,
) -> Result<NewbieCarnivalSwitchRoleResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "newbie:carnival_role",
        i64::from(req.RoleId),
    )
    .map_err(|e| e.to_string())?;
    Ok(NewbieCarnivalSwitchRoleResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_newbie_course_v2_reward_request(
    state: &mut State,
    req: NewbieCourseV2RewardRequest,
) -> Result<NewbieCourseV2RewardResponse, String> {
    claim_ids(state, "newbie:course2", "NewbieCourseV2", &req.Level)?;
    Ok(NewbieCourseV2RewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_newbie_fetch_score_reward_request(
    state: &mut State,
    req: NewbieFetchScoreRewardRequest,
) -> Result<NewbieFetchScoreRewardResponse, String> {
    claim_ids(state, "newbie:score", "NewbieScore", &req.ScoreRewardIds)?;
    Ok(NewbieFetchScoreRewardResponse {
        Code: Some(ErrorCode::Success),
    })
}

pub fn handle_through_train_request(
    state: &mut State,
    req: ThroughTrainRequest,
) -> Result<ThroughTrainResponse, String> {
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        &format!("throughtrain:entered:{}", req.ActivityId),
    )
    .map_err(|e| e.to_string())?;
    Ok(ThroughTrainResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_through_train_finish_view_request(
    state: &mut State,
    req: ThroughTrainFinishViewRequest,
) -> Result<ThroughTrainFinishViewResponse, String> {
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        &format!("through_train:{}", req.ActivityId),
    )
    .map_err(|e| e.to_string())?;
    Ok(ThroughTrainFinishViewResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}
