use std::collections::BTreeMap;

use sonoro_gamesv_logic::component::comp_util;
use sonoro_gamesv_logic::coop_service::{
    self, AdviceContentEntry, AdviceRecord, CoopService,
};
use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::helpers::{energy, pb_kv, quest_engine, reward};
use sonoro_proto::enums::{
    ActivityTaskState, EMatchPlayerUiState, ErrorCode, MatchPlayerLeaveReason, PbAdviceContentType,
    PbAdviceVoteType, WorldEnterPermission, WorldTeamLeaveReason,
};
use sonoro_proto::pb::{
    ActivityTask, AdviceCreateRequest, AdviceCreateResponse, AdviceDeleteRequest,
    AdviceDeleteResponse, AdviceModifyRequest, AdviceModifyResponse, AdviceRequest, AdviceResponse,
    AdviceVoteRequest, AdviceVoteResponse, AdviceVoteUpdateNotify, AgreeJoinResultRequest,
    AgreeJoinResultResponse, ApplyJoinWorldRequest, ApplyJoinWorldResponse, BlackboardParam,
    BlackboardParamValueOneof, CancelMatchRequest, CancelMatchResponse, CoopRoleInfo,
    CoopRoleInfoUpdateNotify, CoopRoleRewardRequest, CoopRoleRewardResponse, CoopSpRewardRequest,
    CoopSpRewardResponse, EnterMatchInstRequest, EnterMatchInstResponse,
    KickMatchTeamPlayerRequest, KickMatchTeamPlayerResponse, KickWorldTeamRequest,
    KickWorldTeamResponse, LeaveMatchTeamNotify, LeaveMatchTeamRequest, LeaveMatchTeamResponse,
    LeaveWorldTeamRequest, LeaveWorldTeamResponse, LobbyListRequest, LobbyListResponse,
    LobbyQueryPlayersRequest, LobbyQueryPlayersResponse, MatchChangePlayerUiStateNotify,
    MatchChangePlayerUiStateRequest, MatchChangePlayerUiStateResponse, MatchChangeReadyNotify,
    MatchChangeReadyRequest, MatchChangeReadyResponse, MatchChangeRoleNotify,
    MatchChangeRoleRequest, MatchChangeRoleResponse, MatchConfirmRequest, MatchConfirmResponse,
    MatchRoleInfo, MutiplayerInfo, MutiplayerTeamRefreshNearListRequest,
    MutiplayerTeamRefreshNearListResponse, PbAdvice, PbAdviceContent, PlayerDetails,
    QueryOnlinePlayersAreaRequest, QueryOnlinePlayersAreaResponse, SetMatchTeamMatchFlagRequest,
    SetMatchTeamMatchFlagResponse, StartMatchRequest, StartMatchResponse, TeamChallengeRequest,
    TeamChallengeResponse, TeamMatchAcceptInviteNotify, TeamMatchAcceptInviteRequest,
    TeamMatchAcceptInviteResponse, TeamMatchInviteRequest, TeamMatchInviteResponse,
    TeamMatchInviteRetToHostNotify, TeamParkourRewardRequest, TeamParkourRewardResponse,
    TeamParkourTaskNotify, WorldBlackboardNotify, WorldBlackboardRequest,
    WorldBlackboardResponse, WorldEnterPermissionsRequest, WorldEnterPermissionsResponse,
    WorldLevelDownRequest, WorldLevelDownResponse, WorldLevelRegainRequest,
    WorldLevelRegainResponse,
};
use sonoro_proto::wire::PbEnum;

use super::State;

pub fn handle_mutiplayer_team_refresh_near_list_request(
    state: &mut State,
    _req: MutiplayerTeamRefreshNearListRequest,
) -> Result<MutiplayerTeamRefreshNearListResponse, String> {
    let info_list = state
        .coop_service
        .as_ref()
        .map(|coop| {
            coop.build_lobby_list(state.player_id)
                .into_iter()
                .map(|det| MutiplayerInfo {
                    PlayerId: det.PlayerId,
                    Name: det.Name,
                    IconId: det.HeadId,
                    Level: det.Level,
                    GuildName: Vec::new(),
                    GuildIntro: det.Signature,
                })
                .collect()
        })
        .unwrap_or_default();
    Ok(MutiplayerTeamRefreshNearListResponse {
        ErrorCode: Some(ErrorCode::Success),
        InfoList: info_list,
        NextRefreshTime: 5,
        ..Default::default()
    })
}

pub fn handle_lobby_list_request(
    state: &mut State,
    _req: LobbyListRequest,
) -> Result<LobbyListResponse, String> {
    let list = state
        .coop_service
        .as_ref()
        .map(|coop| coop.build_lobby_list(state.player_id))
        .unwrap_or_default();
    Ok(LobbyListResponse {
        ErrorCode: Some(ErrorCode::Success),
        ItemList: list,
    })
}

pub fn handle_lobby_query_players_request(
    state: &mut State,
    req: LobbyQueryPlayersRequest,
) -> Result<LobbyQueryPlayersResponse, String> {
    let target_id = req.PlayerId;
    if let Some(coop) = state.coop_service.as_ref()
        && let Some(det) = coop.get_online_details(target_id)
    {
        return Ok(LobbyQueryPlayersResponse {
            ErrorCode: Some(ErrorCode::Success),
            Item: Some(det),
        });
    }
    let Ok(rec) = CoopService::load_social(&state.fs, target_id) else {
        return Ok(LobbyQueryPlayersResponse {
            ErrorCode: Some(ErrorCode::PlayerLoggedOut),
            Item: None,
        });
    };
    let name = if rec.name.is_empty() {
        b"Rover".to_vec()
    } else {
        rec.name.into_bytes()
    };
    Ok(LobbyQueryPlayersResponse {
        ErrorCode: Some(ErrorCode::Success),
        Item: Some(PlayerDetails {
            PlayerId: target_id,
            Name: name,
            Level: rec.level,
            OriginWorldLevel: rec.origin_world_level,
            CurWorldLevel: rec.cur_world_level,
            HeadId: rec.head_id,
            Signature: rec.signature.into_bytes(),
            IsOnline: false,
            IsCanLobbyOnline: false,
            ..Default::default()
        }),
    })
}

pub fn handle_world_enter_permissions_request(
    state: &mut State,
    req: WorldEnterPermissionsRequest,
) -> Result<WorldEnterPermissionsResponse, String> {
    let perm = req.Type.unwrap_or(WorldEnterPermission::ConfirmJoin);
    let basic = &mut state.player_components.basic;
    basic.info.attributes.world_permission = perm.to_i32();
    comp_util::save_struct(
        &state.fs,
        &basic.info,
        &format!("player/{}/basic_info", basic.player_id),
    )
    .map_err(|e| e.to_string())?;
    if let Some(coop) = state.coop_service.as_ref() {
        coop.set_permission(state.player_id, perm);
    }
    Ok(WorldEnterPermissionsResponse {
        ErrorCode: Some(ErrorCode::Success),
        Type: Some(perm),
    })
}

pub fn handle_apply_join_world_request(
    state: &mut State,
    req: ApplyJoinWorldRequest,
) -> Result<ApplyJoinWorldResponse, String> {
    let code = if let Some(coop) = state.coop_service.as_ref() {
        coop.request_join_world(&state.fs, state.player_id, req.PlayerId)
            .map_err(|e| e.to_string())?
    } else {
        ErrorCode::PlayerLoggedOut
    };
    Ok(ApplyJoinWorldResponse {
        ErrorCode: Some(code),
        ErrorParams: Vec::new(),
    })
}

pub fn handle_agree_join_result_request(
    state: &mut State,
    req: AgreeJoinResultRequest,
) -> Result<AgreeJoinResultResponse, String> {
    let code = if let Some(coop) = state.coop_service.as_ref() {
        coop.admit_to_room(&state.fs, state.player_id, req.PlayerId, req.Result)
            .map_err(|e| e.to_string())?
    } else {
        ErrorCode::PlayerLoggedOut
    };
    Ok(AgreeJoinResultResponse {
        ErrorCode: Some(code),
        ..Default::default()
    })
}

pub fn handle_leave_world_team_request(
    state: &mut State,
    req: LeaveWorldTeamRequest,
) -> Result<LeaveWorldTeamResponse, String> {
    let target = if req.PlayerId > 0 {
        req.PlayerId
    } else {
        state.player_id
    };
    if let Some(coop) = state.coop_service.as_ref() {
        coop.leave_or_kick_room(target, req.Reason.unwrap_or(WorldTeamLeaveReason::Initiative));
    }
    Ok(LeaveWorldTeamResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_kick_world_team_request(
    state: &mut State,
    req: KickWorldTeamRequest,
) -> Result<KickWorldTeamResponse, String> {
    let Some(coop) = state.coop_service.as_ref() else {
        return Ok(KickWorldTeamResponse {
            ErrorCode: Some(ErrorCode::ErrNotHostCanNotKick),
        });
    };
    if req.PlayerId == state.player_id || !coop.is_room_host(state.player_id, req.PlayerId) {
        return Ok(KickWorldTeamResponse {
            ErrorCode: Some(ErrorCode::ErrNotHostCanNotKick),
        });
    }
    coop.leave_or_kick_room(req.PlayerId, WorldTeamLeaveReason::BeKick);
    Ok(KickWorldTeamResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_query_online_players_area_request(
    state: &mut State,
    _req: QueryOnlinePlayersAreaRequest,
) -> Result<QueryOnlinePlayersAreaResponse, String> {
    let mut area_dict = BTreeMap::new();
    if let Some(coop) = state.coop_service.as_ref() {
        for (pid, aid) in coop.online_player_areas() {
            area_dict.insert(pid, aid);
        }
    }
    Ok(QueryOnlinePlayersAreaResponse {
        ErrorCode: Some(ErrorCode::Success),
        AreaDict: area_dict,
    })
}

pub fn handle_start_match_request(
    state: &mut State,
    req: StartMatchRequest,
) -> Result<StartMatchResponse, String> {
    if let Some(coop) = state.coop_service.as_ref() {
        coop.start_match(state.player_id, req.InstId);
    }
    Ok(StartMatchResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_cancel_match_request(
    state: &mut State,
    _req: CancelMatchRequest,
) -> Result<CancelMatchResponse, String> {
    if let Some(coop) = state.coop_service.as_ref() {
        coop.cancel_match(state.player_id);
    }
    Ok(CancelMatchResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_match_confirm_request(
    state: &mut State,
    req: MatchConfirmRequest,
) -> Result<MatchConfirmResponse, String> {
    if let Some(coop) = state.coop_service.as_ref() {
        coop.confirm_match(state.player_id, req.IsAccept);
    }
    Ok(MatchConfirmResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_match_change_ready_request(
    state: &mut State,
    req: MatchChangeReadyRequest,
) -> Result<MatchChangeReadyResponse, String> {
    if let Some(coop) = state.coop_service.as_ref() {
        coop.with_online_player_mut(state.player_id, |me| {
            me.match_ready = req.IsReady;
        });
        coop.broadcast_match(
            state.player_id,
            0,
            MatchChangeReadyNotify::MSG_ID,
            &MatchChangeReadyNotify {
                PlayerId: state.player_id,
                IsReady: req.IsReady,
            },
        );
    }
    Ok(MatchChangeReadyResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_match_change_role_request(
    state: &mut State,
    req: MatchChangeRoleRequest,
) -> Result<MatchChangeRoleResponse, String> {
    if let Some(coop) = state.coop_service.as_ref() {
        coop.with_online_player_mut(state.player_id, |me| {
            for (i, slot) in me.match_roles.iter_mut().enumerate() {
                *slot = req.RoleId.get(i).copied().unwrap_or(0);
            }
        });
        let mut role_infos = Vec::new();
        for (idx, &rid) in req.RoleId.iter().enumerate() {
            if rid <= 0 {
                continue;
            }
            let lv = state
                .player_components
                .role
                .role_map
                .get(&rid)
                .map_or(1, |r| r.level);
            let branch_id = req.SkillBranchId.get(idx).copied().unwrap_or(0);
            let phantom_id = req.TowerDefencePhantomId.get(idx).copied().unwrap_or(0);
            role_infos.push(MatchRoleInfo {
                RoleId: rid,
                RoleLevel: lv,
                SkillBranchId: branch_id,
                TowerDefencePhantomId: phantom_id,
                ..Default::default()
            });
        }
        coop.broadcast_match(
            state.player_id,
            0,
            MatchChangeRoleNotify::MSG_ID,
            &MatchChangeRoleNotify {
                PlayerId: state.player_id,
                RoleInfo: role_infos,
            },
        );
    }
    Ok(MatchChangeRoleResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_match_change_player_ui_state_request(
    state: &mut State,
    req: MatchChangePlayerUiStateRequest,
) -> Result<MatchChangePlayerUiStateResponse, String> {
    let ui_state = req.MatchUiState.unwrap_or(EMatchPlayerUiState::Wait);
    if let Some(coop) = state.coop_service.as_ref() {
        coop.with_online_player_mut(state.player_id, |me| {
            me.match_ui_state = ui_state;
        });
        coop.broadcast_match(
            state.player_id,
            0,
            MatchChangePlayerUiStateNotify::MSG_ID,
            &MatchChangePlayerUiStateNotify {
                PlayerId: state.player_id,
                MatchUiState: Some(ui_state),
            },
        );
    }
    Ok(MatchChangePlayerUiStateResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_leave_match_team_request(
    state: &mut State,
    _req: LeaveMatchTeamRequest,
) -> Result<LeaveMatchTeamResponse, String> {
    if let Some(coop) = state.coop_service.as_ref() {
        coop.broadcast_match(
            state.player_id,
            0,
            LeaveMatchTeamNotify::MSG_ID,
            &LeaveMatchTeamNotify {
                PlayerId: state.player_id,
                LeaveReason: Some(MatchPlayerLeaveReason::Initiative),
            },
        );
        coop.cancel_match(state.player_id);
    }
    Ok(LeaveMatchTeamResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_kick_match_team_player_request(
    state: &mut State,
    req: KickMatchTeamPlayerRequest,
) -> Result<KickMatchTeamPlayerResponse, String> {
    let Some(coop) = state.coop_service.as_ref() else {
        return Ok(KickMatchTeamPlayerResponse {
            ErrorCode: Some(ErrorCode::ErrMatchNotHostCanNotKick),
        });
    };
    if !coop.is_match_host(state.player_id) {
        return Ok(KickMatchTeamPlayerResponse {
            ErrorCode: Some(ErrorCode::ErrMatchNotHostCanNotKick),
        });
    }
    coop.broadcast_match(
        req.PlayerId,
        0,
        LeaveMatchTeamNotify::MSG_ID,
        &LeaveMatchTeamNotify {
            PlayerId: req.PlayerId,
            LeaveReason: Some(MatchPlayerLeaveReason::BeKick),
        },
    );
    coop.cancel_match(req.PlayerId);
    Ok(KickMatchTeamPlayerResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_set_match_team_match_flag_request(
    state: &mut State,
    req: SetMatchTeamMatchFlagRequest,
) -> Result<SetMatchTeamMatchFlagResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "coop:match_flag",
        i64::from(req.IsMatch),
    )
    .map_err(|e| e.to_string())?;
    if let Some(coop) = state.coop_service.as_ref() {
        coop.with_online_player_mut(state.player_id, |me| {
            me.match_confirm = req.IsMatch;
        });
    }
    Ok(SetMatchTeamMatchFlagResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_enter_match_inst_request(
    state: &mut State,
    events: &mut EventQueue,
    _req: EnterMatchInstRequest,
) -> Result<EnterMatchInstResponse, String> {
    let inst_id = state
        .coop_service
        .as_ref()
        .and_then(|coop| coop.with_online_player(state.player_id, |me| me.match_inst_id))
        .unwrap_or(0);
    if inst_id > 0 {
        let mut qctx = state.quest_context(events);
        quest_engine::enter_inst(&mut qctx, inst_id, 0).map_err(|e| e.to_string())?;
    }
    Ok(EnterMatchInstResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_team_challenge_request(
    state: &mut State,
    events: &mut EventQueue,
    req: TeamChallengeRequest,
) -> Result<TeamChallengeResponse, String> {
    if req.InstId > 0 {
        let mut qctx = state.quest_context(events);
        quest_engine::enter_inst(&mut qctx, req.InstId, 0).map_err(|e| e.to_string())?;
    }
    Ok(TeamChallengeResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_team_match_invite_request(
    state: &mut State,
    req: TeamMatchInviteRequest,
) -> Result<TeamMatchInviteResponse, String> {
    if let Some(coop) = state.coop_service.as_ref() {
        coop.push_to_player(
            state.player_id,
            TeamMatchInviteRetToHostNotify::MSG_ID,
            &TeamMatchInviteRetToHostNotify {
                ErrorCode: Some(ErrorCode::Success),
                InstId: req.InstId,
            },
        );
    }
    Ok(TeamMatchInviteResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_team_match_accept_invite_request(
    state: &mut State,
    req: TeamMatchAcceptInviteRequest,
) -> Result<TeamMatchAcceptInviteResponse, String> {
    if let Some(coop) = state.coop_service.as_ref() {
        coop.push_to_player(
            req.HostId,
            TeamMatchAcceptInviteNotify::MSG_ID,
            &TeamMatchAcceptInviteNotify {
                PlayerId: state.player_id,
                AcceptResult: req.AcceptResult,
            },
        );
    }
    Ok(TeamMatchAcceptInviteResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_team_parkour_reward_request(
    state: &mut State,
    req: TeamParkourRewardRequest,
) -> Result<TeamParkourRewardResponse, String> {
    let reward_row = state
        .assets
        .team_parkour_rewards()
        .iter()
        .find(|r| r.Id == req.RewardId)
        .cloned();
    let key = format!("team_pk:{}:{}", req.ActivityId, req.RewardId);
    if state.player_components.progress.info.mark_claim(&key) {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        let mut rctx = state.reward_context();
        if let Some(ref row) = reward_row
            && row.Reward > 0
        {
            reward::grant_drop_package_popup(&mut rctx, row.Reward, 19500)
                .map_err(|e| e.to_string())?;
        } else {
            reward::grant_table_reward(
                &mut rctx,
                "TeamParKOurReward",
                i64::from(req.RewardId),
                false,
                true,
                1,
                &[],
                None,
            )
            .map_err(|e| e.to_string())?;
        }
        let target = reward_row.as_ref().map_or(1, |r| r.RewardThreshold.max(1));
        let _ = state.conn.push_msg(
            TeamParkourTaskNotify::MSG_ID,
            &TeamParkourTaskNotify {
                ActivityTask: Some(ActivityTask {
                    Id: req.RewardId,
                    Current: target,
                    Target: target,
                    Status: Some(ActivityTaskState::ActivityTaskTaken),
                    ..Default::default()
                }),
            },
        );
    }
    Ok(TeamParkourRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_coop_role_reward_request(
    state: &mut State,
    req: CoopRoleRewardRequest,
) -> Result<CoopRoleRewardResponse, String> {
    let level_row = state
        .assets
        .coop_role_levels()
        .iter()
        .find(|r| r.Id == req.CoopId)
        .cloned();
    let Some(row) = level_row else {
        return Ok(CoopRoleRewardResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    };
    let key = format!("coop:role:{}", req.CoopId);
    if state.player_components.progress.info.mark_claim(&key) {
        let role_id = row.CoopRoleId;
        let role_lv = {
            let info = &mut state.player_components.progress.info;
            let lv_key = format!("coop:role_lv:{role_id}");
            let cur_lv = info.get_key_int(&lv_key).max(i64::from(row.CoopLevel)) as i32;
            info.set_key_int(&lv_key, i64::from(cur_lv));
            let rw_key = format!("coop:role_rw:{role_id}");
            let cur_rw = info.get_key_int(&rw_key).max(i64::from(row.CoopLevel)) as i32;
            info.set_key_int(&rw_key, i64::from(cur_rw));
            cur_lv
        };
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        if row.Reward > 0 {
            let mut rctx = state.reward_context();
            reward::grant_drop_package_popup(&mut rctx, row.Reward, 19500)
                .map_err(|e| e.to_string())?;
        }
        let _ = state.conn.push_msg(
            CoopRoleInfoUpdateNotify::MSG_ID,
            &CoopRoleInfoUpdateNotify {
                ActivityId: row.ActivityId,
                CoopRoleInfo: Some(CoopRoleInfo {
                    CoopRoleId: role_id,
                    RoleLevel: role_lv,
                    RewardLevel: role_lv,
                    FinishTime: energy::now_seconds(),
                }),
                ..Default::default()
            },
        );
    }
    Ok(CoopRoleRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_coop_sp_reward_request(
    state: &mut State,
    req: CoopSpRewardRequest,
) -> Result<CoopSpRewardResponse, String> {
    let sp_rows = state.assets.coop_sp_rewards().to_vec();
    let role_sum: i32 = state
        .assets
        .coop_roles()
        .iter()
        .map(|r| {
            state
                .player_components
                .progress
                .info
                .get_key_int(&format!("coop:role_lv:{}", r.Id)) as i32
        })
        .sum();
    let mut dirty = false;
    for rid in req.RewardIds {
        let Some(sp_row) = sp_rows.iter().find(|r| r.Id == rid) else {
            continue;
        };
        if sp_row.RoleLevelSum > role_sum {
            // Auto-sync role level sum only if claims were already unlocked via quest progression.
            let unlocked_by_claims: i32 = state
                .assets
                .coop_role_levels()
                .iter()
                .filter(|cl| {
                    state
                        .player_components
                        .progress
                        .info
                        .has_claim(&format!("coop:role:{}", cl.Id))
                })
                .count() as i32;
            if sp_row.RoleLevelSum > unlocked_by_claims.max(role_sum) {
                continue;
            }
        }
        let key = format!("coop:sp:{rid}");
        if state.player_components.progress.info.mark_claim(&key) {
            dirty = true;
            if sp_row.Reward > 0 {
                let mut rctx = state.reward_context();
                reward::grant_drop_package_popup(&mut rctx, sp_row.Reward, 19500)
                    .map_err(|e| e.to_string())?;
            }
        }
    }
    if dirty {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(CoopSpRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_world_level_down_request(
    state: &mut State,
    events: &mut EventQueue,
    _req: WorldLevelDownRequest,
) -> Result<WorldLevelDownResponse, String> {
    let attrs = &mut state.player_components.basic.info.attributes;
    if attrs.cur_world_level > 1 && attrs.cur_world_level == attrs.origin_world_level {
        attrs.cur_world_level -= 1;
        attrs.world_level_time_stamp = energy::now_seconds() as i32;
        let path = format!("player/{}/basic_info", state.player_components.basic.player_id);
        comp_util::save_struct(&state.fs, &state.player_components.basic.info, &path)
            .map_err(|e| e.to_string())?;
        let mut qctx = state.quest_context(events);
        if let Err(err) = quest_engine::events::on_difficulty_changed(&mut qctx) {
            tracing::warn!(target: "coop", "on_world_level_down failed: {err}");
        }
    }
    let attrs = &state.player_components.basic.info.attributes;
    Ok(WorldLevelDownResponse {
        ErrorCode: Some(ErrorCode::Success),
        OriginWorldLevel: attrs.origin_world_level,
        CurWorldLevel: attrs.cur_world_level,
        WorldLevelTimeStamp: attrs.world_level_time_stamp,
    })
}

pub fn handle_world_level_regain_request(
    state: &mut State,
    events: &mut EventQueue,
    _req: WorldLevelRegainRequest,
) -> Result<WorldLevelRegainResponse, String> {
    let attrs = &mut state.player_components.basic.info.attributes;
    let changed = attrs.cur_world_level != attrs.origin_world_level;
    attrs.cur_world_level = attrs.origin_world_level;
    attrs.world_level_time_stamp = energy::now_seconds() as i32;
    let path = format!("player/{}/basic_info", state.player_components.basic.player_id);
    comp_util::save_struct(&state.fs, &state.player_components.basic.info, &path)
        .map_err(|e| e.to_string())?;
    if changed {
        let mut qctx = state.quest_context(events);
        if let Err(err) = quest_engine::events::on_difficulty_changed(&mut qctx) {
            tracing::warn!(target: "coop", "on_world_level_regain failed: {err}");
        }
    }
    let attrs = &state.player_components.basic.info.attributes;
    Ok(WorldLevelRegainResponse {
        ErrorCode: Some(ErrorCode::Success),
        OriginWorldLevel: attrs.origin_world_level,
        CurWorldLevel: attrs.cur_world_level,
        WorldLevelTimeStamp: attrs.world_level_time_stamp,
    })
}

pub fn handle_world_blackboard_request(
    state: &mut State,
    req: WorldBlackboardRequest,
) -> Result<WorldBlackboardResponse, String> {
    let mut dirty = false;
    let mut synced_params: Vec<BlackboardParam> = Vec::with_capacity(req.Params.len());
    for param in req.Params {
        if param.Key.is_empty() || param.Value.is_none() {
            continue;
        }
        let raw_key = String::from_utf8_lossy(&param.Key).into_owned();
        let key = format!("world_bb:{raw_key}");
        let info = &mut state.player_components.progress.info;
        let is_scalar = match &param.Value {
            Some(BlackboardParamValueOneof::IntValue(v)) => {
                info.set_key_int(&key, i64::from(*v));
                true
            }
            Some(BlackboardParamValueOneof::LongValue(v)) => {
                info.set_key_int(&key, *v);
                true
            }
            Some(BlackboardParamValueOneof::BooleanValue(v)) => {
                info.set_key_int(&key, i64::from(*v));
                true
            }
            Some(BlackboardParamValueOneof::StringValue(v)) => {
                info.set_key_str(&key, &String::from_utf8_lossy(v));
                true
            }
            Some(BlackboardParamValueOneof::FloatValue(v)) => {
                let scaled = f64::from(*v) * 1000.0;
                let iv = if scaled.is_finite() {
                    scaled.clamp(-9e18, 9e18) as i64
                } else {
                    0
                };
                info.set_key_int(&key, iv);
                true
            }
            _ => false,
        };
        if !is_scalar {
            pb_kv::save_msg(
                &state.fs,
                &mut state.player_components.progress,
                &format!("world_bb_pb:{raw_key}"),
                &param,
            )
            .map_err(|e| e.to_string())?;
        }
        synced_params.push(param);
        dirty = true;
    }
    if dirty {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        if let Some(coop) = state.coop_service.as_ref()
            && !synced_params.is_empty()
        {
            coop.broadcast_room_except(
                state.player_id,
                WorldBlackboardNotify::MSG_ID,
                &WorldBlackboardNotify {
                    Params: synced_params,
                },
            );
        }
    }
    Ok(WorldBlackboardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_advice_request(
    state: &mut State,
    _req: AdviceRequest,
) -> Result<AdviceResponse, String> {
    let _guard = state.coop_service.as_ref().map(|c| c.advices_lock());
    let Ok(store) = CoopService::load_advices(&state.fs) else {
        return Ok(AdviceResponse {
            ErrorCode: Some(ErrorCode::Success),
            ..Default::default()
        });
    };
    let mut out = Vec::with_capacity(store.advices.len());
    let mut up_vote_ids = Vec::new();
    for a in store.advices {
        let contents = a
            .contents
            .iter()
            .map(|c| PbAdviceContent {
                Type: Some(PbAdviceContentType::from_i32_or_default(c.content_type)),
                Id: c.id,
                Word: c.word,
            })
            .collect();
        out.push(PbAdvice {
            Id: a.id,
            AreaId: a.area_id,
            Contents: contents,
            UpVote: a.up_vote,
        });
        if a.voter_ids.contains(&state.player_id) {
            up_vote_ids.push(a.id);
        }
    }
    Ok(AdviceResponse {
        ErrorCode: Some(ErrorCode::Success),
        Advices: out,
        UpVoteIds: up_vote_ids,
    })
}

pub fn handle_advice_create_request(
    state: &mut State,
    req: AdviceCreateRequest,
) -> Result<AdviceCreateResponse, String> {
    let _guard = state.coop_service.as_ref().map(|c| c.advices_lock());
    let mut store = CoopService::load_advices(&state.fs).map_err(|e| e.to_string())?;
    let id = store.next_id;
    store.next_id += 1;
    let contents: Vec<AdviceContentEntry> = req
        .Contents
        .iter()
        .map(|c| AdviceContentEntry {
            content_type: c.Type.map_or(0, |t| t.to_i32()),
            id: c.Id,
            word: c.Word,
        })
        .collect();
    let pos = req.Pos.unwrap_or_default();
    let rot = req.Rot.unwrap_or_default();
    let area_id = state.player_components.basic.info.attributes.area_id;
    store.advices.push(AdviceRecord {
        id,
        area_id,
        owner_id: state.player_id,
        up_vote: 0,
        pos_x: pos.X,
        pos_y: pos.Y,
        pos_z: pos.Z,
        rot_pitch: rot.Pitch,
        rot_yaw: rot.Yaw,
        rot_roll: rot.Roll,
        voter_ids: Vec::new(),
        contents,
    });
    CoopService::save_advices(&state.fs, &store).map_err(|e| e.to_string())?;
    Ok(AdviceCreateResponse {
        ErrorCode: Some(ErrorCode::Success),
        Advice: Some(PbAdvice {
            Id: id,
            AreaId: area_id,
            Contents: req.Contents,
            UpVote: 0,
        }),
    })
}

pub fn handle_advice_delete_request(
    state: &mut State,
    req: AdviceDeleteRequest,
) -> Result<AdviceDeleteResponse, String> {
    let _guard = state.coop_service.as_ref().map(|c| c.advices_lock());
    let mut store = CoopService::load_advices(&state.fs).map_err(|e| e.to_string())?;
    let Some(pos) = store.advices.iter().position(|a| a.id == req.Id) else {
        return Ok(AdviceDeleteResponse {
            ErrorCode: Some(ErrorCode::ErrAdviceNotFound),
        });
    };
    if store.advices[pos].owner_id != state.player_id {
        return Ok(AdviceDeleteResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    }
    store.advices.remove(pos);
    CoopService::save_advices(&state.fs, &store).map_err(|e| e.to_string())?;
    Ok(AdviceDeleteResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_advice_modify_request(
    state: &mut State,
    req: AdviceModifyRequest,
) -> Result<AdviceModifyResponse, String> {
    let _guard = state.coop_service.as_ref().map(|c| c.advices_lock());
    let mut store = CoopService::load_advices(&state.fs).map_err(|e| e.to_string())?;
    for a in &mut store.advices {
        if a.id != req.Id {
            continue;
        }
        if a.owner_id != state.player_id {
            return Ok(AdviceModifyResponse {
                ErrorCode: Some(ErrorCode::RequestParamError),
            });
        }
        a.contents = req
            .Contents
            .iter()
            .map(|c| AdviceContentEntry {
                content_type: c.Type.map_or(0, |t| t.to_i32()),
                id: c.Id,
                word: c.Word,
            })
            .collect();
        CoopService::save_advices(&state.fs, &store).map_err(|e| e.to_string())?;
        return Ok(AdviceModifyResponse {
            ErrorCode: Some(ErrorCode::Success),
        });
    }
    Ok(AdviceModifyResponse {
        ErrorCode: Some(ErrorCode::ErrAdviceNotFound),
    })
}

pub fn handle_advice_vote_request(
    state: &mut State,
    req: AdviceVoteRequest,
) -> Result<AdviceVoteResponse, String> {
    let _guard = state.coop_service.as_ref().map(|c| c.advices_lock());
    let mut store = CoopService::load_advices(&state.fs).map_err(|e| e.to_string())?;
    for a in &mut store.advices {
        if a.id != req.Id {
            continue;
        }
        let vote_type = req.Type.unwrap_or(PbAdviceVoteType::Up);
        if vote_type == PbAdviceVoteType::Up {
            if !coop_service::append_unique_i32(&mut a.voter_ids, state.player_id) {
                return Ok(AdviceVoteResponse {
                    ErrorCode: Some(ErrorCode::ErrAdviceIsVoteUp),
                });
            }
            a.up_vote += 1;
        } else {
            if !coop_service::remove_i32(&mut a.voter_ids, state.player_id) {
                return Ok(AdviceVoteResponse {
                    ErrorCode: Some(ErrorCode::ErrAdviceIsVoteDown),
                });
            }
            a.up_vote = (a.up_vote - 1).max(0);
        }
        let aid = a.id;
        let up_vote = a.up_vote;
        CoopService::save_advices(&state.fs, &store).map_err(|e| e.to_string())?;
        let _ = state.conn.push_msg(
            AdviceVoteUpdateNotify::MSG_ID,
            &AdviceVoteUpdateNotify {
                Id: aid,
                UpVote: up_vote,
            },
        );
        return Ok(AdviceVoteResponse {
            ErrorCode: Some(ErrorCode::Success),
        });
    }
    Ok(AdviceVoteResponse {
        ErrorCode: Some(ErrorCode::ErrAdviceNotFound),
    })
}
