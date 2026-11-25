use sonoro_gamesv_logic::component::comp_util;
use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::helpers::{
    level_play::{self, LevelPlayContext},
    quest_engine,
};
use sonoro_proto::enums::{ErrorCode, QuestFocusReason};
use sonoro_proto::pb::{
    AcceptFocusWaitQuestRequest, AcceptFocusWaitQuestResponse, ActionFinishRequest,
    ActionFinishResponse, BtGiveUpRequest, BtGiveUpResponse, BtRollbackRequest, BtRollbackResponse,
    BtSetTimerInfoRequest, BtSetTimerInfoResponse, CancelQuestFocusModeRequest,
    CancelQuestFocusModeResponse, ConditionGroupStateRequest, ConditionGroupStateResponse,
    ConfirmQuestResourceRequest, ConfirmQuestResourceResponse, DailyQuestChangePreferRequest,
    DailyQuestChangePreferResponse, DailyQuestGetRewardRequest, DailyQuestGetRewardResponse,
    FocusQuestSetNotify, QuestActionRequest, QuestActionResponse, QuestBranchRequest,
    QuestBranchResponse, QuestBranchSelectRequest, QuestBranchSelectResponse,
    QuestFinishActionEndRequest, QuestFinishActionEndResponse, QuestFinishActionRequest,
    QuestFinishActionResponse, QuestRedDotRequest, QuestRedDotResponse, QuestReviewInfoRequest,
    QuestReviewInfoResponse, QuestTreeInfoRequest, QuestTreeInfoResponse,
    RoleFavorAcceptQuestRequest, RoleFavorAcceptQuestResponse, RoleQuestAcceptRequest,
    RoleQuestAcceptResponse, SetFocusModeDeterCondRequest, SetFocusModeDeterCondResponse,
    SetQuestFocusModeRequest, SetQuestFocusModeResponse, SubmitAwakeAndLoadEntityNodeRequest,
    SubmitAwakeAndLoadEntityNodeResponse, SubmitNodeRequest, SubmitNodeResponse,
    SubmitPhotoNodeRequest, SubmitPhotoNodeResponse, TimerEndRequest, TimerEndResponse,
    TraceQuestNotify, TraceQuestRequest, TraceQuestResponse,
};

use super::State;

pub fn handle_quest_review_info_request(
    state: &mut State,
    _req: QuestReviewInfoRequest,
) -> Result<QuestReviewInfoResponse, String> {
    let finished_cnt = i64::try_from(state.player_components.quest.info.finished_quests.len())
        .unwrap_or(i64::MAX);
    state
        .player_components
        .progress
        .info
        .set_key_int("quest:finished_cnt", finished_cnt);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(QuestReviewInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_set_focus_mode_deter_cond_request(
    state: &mut State,
    events: &mut EventQueue,
    req: SetFocusModeDeterCondRequest,
) -> Result<SetFocusModeDeterCondResponse, String> {
    let mut qctx = state.quest_context(events);
    quest_engine::set_focus_deter_cond(&mut qctx, req.DeterCond).map_err(|e| e.to_string())?;
    Ok(SetFocusModeDeterCondResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_set_quest_focus_mode_request(
    state: &mut State,
    req: SetQuestFocusModeRequest,
) -> Result<SetQuestFocusModeResponse, String> {
    state.player_components.quest.info.focus_quest_id = req.QuestId;
    comp_util::save_struct(
        &state.fs,
        &state.player_components.quest.info,
        &format!("player/{}/quest_info", state.player_id),
    )
    .map_err(|e| e.to_string())?;
    let _ = state.conn.push_msg(
        FocusQuestSetNotify::MSG_ID,
        &FocusQuestSetNotify {
            FocusQuestId: req.QuestId,
            Reason: Some(QuestFocusReason::Request),
        },
    );
    Ok(SetQuestFocusModeResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_cancel_quest_focus_mode_request(
    state: &mut State,
    _req: CancelQuestFocusModeRequest,
) -> Result<CancelQuestFocusModeResponse, String> {
    state.player_components.quest.info.focus_quest_id = 0;
    comp_util::save_struct(
        &state.fs,
        &state.player_components.quest.info,
        &format!("player/{}/quest_info", state.player_id),
    )
    .map_err(|e| e.to_string())?;
    Ok(CancelQuestFocusModeResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_accept_focus_wait_quest_request(
    state: &mut State,
    events: &mut EventQueue,
    req: AcceptFocusWaitQuestRequest,
) -> Result<AcceptFocusWaitQuestResponse, String> {
    let mut qctx = state.quest_context(events);
    quest_engine::accept_focus_wait_quest(&mut qctx, req.QuestId).map_err(|e| e.to_string())?;
    Ok(AcceptFocusWaitQuestResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_confirm_quest_resource_request(
    state: &mut State,
    events: &mut EventQueue,
    req: ConfirmQuestResourceRequest,
) -> Result<ConfirmQuestResourceResponse, String> {
    let mut qctx = state.quest_context(events);
    quest_engine::sync_quest_resource_availability(&mut qctx, &[], &req.QuestIds, false)
        .map_err(|e| e.to_string())?;
    Ok(ConfirmQuestResourceResponse {
        ErrorId: Some(ErrorCode::Success),
    })
}

pub fn handle_quest_branch_request(
    state: &mut State,
    events: &mut EventQueue,
    _req: QuestBranchRequest,
) -> Result<QuestBranchResponse, String> {
    let qctx = state.quest_context(events);
    Ok(QuestBranchResponse {
        errorCode: Some(ErrorCode::Success),
        QuestBranchInfo: Some(quest_engine::build_quest_branch_info(&qctx)),
    })
}

pub fn handle_quest_branch_select_request(
    state: &mut State,
    events: &mut EventQueue,
    req: QuestBranchSelectRequest,
) -> Result<QuestBranchSelectResponse, String> {
    let mut qctx = state.quest_context(events);
    let info = quest_engine::select_quest_branch(&mut qctx, req.BranchPageId, req.BranchId)
        .map_err(|e| e.to_string())?;
    Ok(QuestBranchSelectResponse {
        ErrorCode: Some(ErrorCode::Success),
        OneQuestBranchPageInfo: Some(info),
    })
}

pub fn handle_condition_group_state_request(
    state: &mut State,
    events: &mut EventQueue,
    req: ConditionGroupStateRequest,
) -> Result<ConditionGroupStateResponse, String> {
    let qctx = state.quest_context(events);
    let infos = quest_engine::eval_condition_groups(&qctx, &req.ConditionGroupIds);
    Ok(ConditionGroupStateResponse {
        ConditionGroupFinishInfos: infos,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_daily_quest_get_reward_request(
    state: &mut State,
    events: &mut EventQueue,
    req: DailyQuestGetRewardRequest,
) -> Result<DailyQuestGetRewardResponse, String> {
    let mut qctx = state.quest_context(events);
    let rewards =
        quest_engine::claim_daily_rewards(&mut qctx, &req.QuestId).map_err(|e| e.to_string())?;
    Ok(DailyQuestGetRewardResponse {
        Rewards: rewards,
        ErrorId: Some(ErrorCode::Success),
    })
}

pub fn handle_daily_quest_change_prefer_request(
    state: &mut State,
    events: &mut EventQueue,
    req: DailyQuestChangePreferRequest,
) -> Result<DailyQuestChangePreferResponse, String> {
    let mut qctx = state.quest_context(events);
    quest_engine::change_daily_prefer(&mut qctx, req.TargetArea, req.TargetInfluence)
        .map_err(|e| e.to_string())?;
    Ok(DailyQuestChangePreferResponse {
        ErrorId: Some(ErrorCode::Success),
    })
}

pub fn handle_submit_photo_node_request(
    state: &mut State,
    events: &mut EventQueue,
    req: SubmitPhotoNodeRequest,
) -> Result<SubmitPhotoNodeResponse, String> {
    state
        .player_components
        .progress
        .info
        .mark_claim("photo:submitted");
    let _ = state.player_components.progress.save(&state.fs);
    let mut qctx = state.quest_context(events);
    quest_engine::events::on_photo_submitted(&mut qctx, req.TreeOwnerId, req.TreeIncId, req.NodeId)
        .map_err(|e| e.to_string())?;
    let _ = quest_engine::events::on_condition_state_changed(&mut qctx);
    if let Some(mut lp_ctx) = LevelPlayContext::from_quest_ctx(&mut qctx) {
        let _ = level_play::on_submit_photo_level_play_node(
            &mut lp_ctx,
            req.TreeOwnerId,
            req.TreeIncId,
            req.NodeId,
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(SubmitPhotoNodeResponse {
        ErrorId: Some(ErrorCode::Success),
    })
}

pub fn handle_submit_node_request(
    state: &mut State,
    events: &mut EventQueue,
    req: SubmitNodeRequest,
) -> Result<SubmitNodeResponse, String> {
    let mut qctx = state.quest_context(events);
    let err_id =
        quest_engine::events::on_submit_node(&mut qctx, req.TreeOwnerId, req.TreeIncId, req.NodeId)
            .map_err(|e| e.to_string())?;
    if let Some(mut lp_ctx) = LevelPlayContext::from_quest_ctx(&mut qctx) {
        let _ = level_play::on_submit_level_play_node(
            &mut lp_ctx,
            req.TreeOwnerId,
            req.TreeIncId,
            req.NodeId,
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(SubmitNodeResponse {
        TreeOwnerId: req.TreeOwnerId,
        TreeIncId: req.TreeIncId,
        NodeId: req.NodeId,
        ErrorId: Some(err_id),
    })
}

pub fn handle_timer_end_request(
    state: &mut State,
    events: &mut EventQueue,
    req: TimerEndRequest,
) -> Result<TimerEndResponse, String> {
    let timer_type = String::from_utf8_lossy(&req.TimerType).into_owned();
    let mut qctx = state.quest_context(events);
    quest_engine::events::on_bt_timer_end(&mut qctx, req.TreeOwnerId, req.TreeIncId, &timer_type)
        .map_err(|e| e.to_string())?;
    Ok(TimerEndResponse {
        ErrorId: Some(ErrorCode::Success),
    })
}

pub fn handle_bt_set_timer_info_request(
    state: &mut State,
    events: &mut EventQueue,
    req: BtSetTimerInfoRequest,
) -> Result<BtSetTimerInfoResponse, String> {
    let timer_type = String::from_utf8_lossy(&req.TimerType).into_owned();
    let mut qctx = state.quest_context(events);
    quest_engine::events::on_bt_set_timer_info(
        &mut qctx,
        req.TreeOwnerId,
        req.TreeIncId,
        req.NodeId,
        &timer_type,
        req.Second,
        req.SetType.unwrap_or_default(),
    )
    .map_err(|e| e.to_string())?;
    Ok(BtSetTimerInfoResponse {
        ErrorId: Some(ErrorCode::Success),
    })
}

pub fn handle_trace_quest_request(
    state: &mut State,
    events: &mut EventQueue,
    req: TraceQuestRequest,
) -> Result<TraceQuestResponse, String> {
    let is_untrack = req.TraceType == 2 || (req.TraceType == 0 && req.Operate == 0);
    let qid = if is_untrack || req.QuestId <= 0 {
        0
    } else {
        req.QuestId
    };
    {
        let mut qctx = state.quest_context(events);
        quest_engine::set_tracked_quest(&mut qctx, qid).map_err(|e| e.to_string())?;
    }
    let _ = state.conn.push_msg(
        TraceQuestNotify::MSG_ID,
        &TraceQuestNotify {
            QuestId: state.player_components.quest.info.tracked_quest_id,
        },
    );
    Ok(TraceQuestResponse {
        ErrorId: Some(ErrorCode::Success),
    })
}

pub fn handle_quest_red_dot_request(
    state: &mut State,
    req: QuestRedDotRequest,
) -> Result<QuestRedDotResponse, String> {
    let k = format!("quest:red_dot:{}", req.QuestId);
    state
        .player_components
        .progress
        .info
        .set_key_int(&k, i64::from(req.Operate));
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(QuestRedDotResponse {
        ErrorId: Some(ErrorCode::Success),
    })
}

pub fn handle_action_finish_request(
    state: &mut State,
    events: &mut EventQueue,
    req: ActionFinishRequest,
) -> Result<ActionFinishResponse, String> {
    let k = format!("quest:act_fin:{}:{}", req.IncId, req.StartIndex);
    state.player_components.progress.info.mark_claim(&k);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    let mut qctx = state.quest_context(events);
    quest_engine::events::on_condition_state_changed(&mut qctx).map_err(|e| e.to_string())?;
    Ok(ActionFinishResponse {
        Code: Some(ErrorCode::Success),
    })
}

pub fn handle_bt_give_up_request(
    state: &mut State,
    events: &mut EventQueue,
    req: BtGiveUpRequest,
) -> Result<BtGiveUpResponse, String> {
    let mut qctx = state.quest_context(events);
    quest_engine::events::on_give_up_quest(&mut qctx, req.TreeOwnerId, req.TreeIncId, req.NodeId)
        .map_err(|e| e.to_string())?;
    Ok(BtGiveUpResponse {
        ErrorId: Some(ErrorCode::Success),
    })
}

pub fn handle_bt_rollback_request(
    state: &mut State,
    events: &mut EventQueue,
    req: BtRollbackRequest,
) -> Result<BtRollbackResponse, String> {
    let mut qctx = state.quest_context(events);
    quest_engine::events::on_client_bt_rollback(&mut qctx, req.TreeOwnerId, req.TreeIncId, req.Select)
        .map_err(|e| e.to_string())?;
    Ok(BtRollbackResponse {
        ErrorId: Some(ErrorCode::Success),
    })
}

pub fn handle_quest_action_request(
    state: &mut State,
    events: &mut EventQueue,
    req: QuestActionRequest,
) -> Result<QuestActionResponse, String> {
    if req.QuestId > 0 {
        let k = format!("quest:act:{}:{}:{}", req.QuestId, req.NodeId, req.ActionId);
        if state.player_components.progress.info.mark_claim(&k) {
            state
                .player_components
                .progress
                .save(&state.fs)
                .map_err(|e| e.to_string())?;
        }
    }
    let mut qctx = state.quest_context(events);
    quest_engine::events::on_condition_state_changed(&mut qctx).map_err(|e| e.to_string())?;
    Ok(QuestActionResponse {
        ErrorId: Some(ErrorCode::Success),
    })
}

pub fn handle_quest_finish_action_request(
    state: &mut State,
    events: &mut EventQueue,
    _req: QuestFinishActionRequest,
) -> Result<QuestFinishActionResponse, String> {
    let mut qctx = state.quest_context(events);
    quest_engine::events::on_condition_state_changed(&mut qctx).map_err(|e| e.to_string())?;
    Ok(QuestFinishActionResponse {
        ErrorId: Some(ErrorCode::Success),
    })
}

pub fn handle_quest_finish_action_end_request(
    state: &mut State,
    events: &mut EventQueue,
    _req: QuestFinishActionEndRequest,
) -> Result<QuestFinishActionEndResponse, String> {
    let mut qctx = state.quest_context(events);
    quest_engine::events::on_condition_state_changed(&mut qctx).map_err(|e| e.to_string())?;
    Ok(QuestFinishActionEndResponse {
        ErrorId: Some(ErrorCode::Success),
    })
}

pub fn handle_quest_tree_info_request(
    state: &mut State,
    events: &mut EventQueue,
    _req: QuestTreeInfoRequest,
) -> Result<QuestTreeInfoResponse, String> {
    let mut qctx = state.quest_context(events);
    quest_engine::resync_on_reconnect(&mut qctx).map_err(|e| e.to_string())?;
    Ok(QuestTreeInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_role_quest_accept_request(
    state: &mut State,
    events: &mut EventQueue,
    req: RoleQuestAcceptRequest,
) -> Result<RoleQuestAcceptResponse, String> {
    let mut qctx = state.quest_context(events);
    quest_engine::accept_quest_by_id(&mut qctx, req.QuestId).map_err(|e| e.to_string())?;
    Ok(RoleQuestAcceptResponse {
        ErrorId: Some(ErrorCode::Success),
    })
}

pub fn handle_role_favor_accept_quest_request(
    state: &mut State,
    events: &mut EventQueue,
    req: RoleFavorAcceptQuestRequest,
) -> Result<RoleFavorAcceptQuestResponse, String> {
    let qt_id = req.QuestType.map_or(0, |qt| qt as i32);
    let k = format!("role_favor_quest:{}:{qt_id}", req.RoleId);
    state.player_components.progress.info.mark_claim(&k);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    let mut qctx = state.quest_context(events);
    if let Err(err) = quest_engine::events::on_condition_state_changed(&mut qctx) {
        tracing::warn!(target: "quest", "on_role_favor_accept_quest failed: {err}");
    }
    Ok(RoleFavorAcceptQuestResponse {
        QuestType: req.QuestType,
        RoleId: req.RoleId,
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_submit_awake_and_load_entity_node_request(
    state: &mut State,
    events: &mut EventQueue,
    req: SubmitAwakeAndLoadEntityNodeRequest,
) -> Result<SubmitAwakeAndLoadEntityNodeResponse, String> {
    let mut dirty = false;
    if req.TreeOwnerId > 0 {
        state
            .player_components
            .progress
            .info
            .set_key_int(&format!("awake_node:{}", req.TreeOwnerId), i64::from(req.NodeId));
        dirty = true;
    }
    if req.TreeIncId > 0 {
        state
            .player_components
            .progress
            .info
            .set_key_int(&format!("awake_node:{}", req.TreeIncId), i64::from(req.NodeId));
        dirty = true;
    }
    if dirty {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }

    let owner = if req.TreeOwnerId > 0 {
        req.TreeOwnerId
    } else {
        i32::try_from(req.TreeIncId).unwrap_or(0)
    };
    let has_active_quest = owner > 0
        && state
            .player_components
            .quest
            .info
            .find_active_quest(owner)
            .is_some();
    let mut qctx = state.quest_context(events);
    if owner > 0 && !has_active_quest {
        if let Some(mut lp_ctx) = LevelPlayContext::from_quest_ctx(&mut qctx) {
            let _ = level_play::on_submit_level_play_node(
                &mut lp_ctx,
                req.TreeOwnerId,
                req.TreeIncId,
                req.NodeId,
            )
            .map_err(|e| e.to_string())?;
        }
    } else {
        let _ = quest_engine::events::on_submit_node(
            &mut qctx,
            req.TreeOwnerId,
            req.TreeIncId,
            req.NodeId,
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(SubmitAwakeAndLoadEntityNodeResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}
