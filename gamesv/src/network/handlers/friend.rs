use sonoro_gamesv_logic::coop_service::CoopService;
use sonoro_proto::enums::ErrorCode;
use sonoro_proto::pb::{
    BlockListRequest, BlockListResponse, BlockPlayerRequest, BlockPlayerResponse, FriendAllRequest,
    FriendAllResponse, FriendApply, FriendApplyDeletedNotify, FriendApplyHandleRequest,
    FriendApplyHandleResponse, FriendApplySendRequest, FriendApplySendResponse,
    FriendDeleteRequest, FriendDeleteResponse, FriendInfo, FriendRecentlyTeamRequest,
    FriendRecentlyTeamResponse, FriendRemarkRequest, FriendRemarkResponse,
    PlayerBasicInfoGetRequest, PlayerBasicInfoGetResponse, PlayerDetails, RecentlyTeamInfo,
    ReportPlayerRequest, ReportPlayerResponse, UnblockPlayerRequest, UnblockPlayerResponse,
};

use super::State;

fn resolve_player_details(state: &State, target_id: i32) -> PlayerDetails {
    if let Some(coop) = state.coop_service.as_ref()
        && let Some(details) = coop.get_online_details(target_id)
    {
        return details;
    }
    let rec = CoopService::load_social(&state.fs, target_id).unwrap_or_default();
    PlayerDetails {
        PlayerId: target_id,
        Name: if rec.name.is_empty() {
            format!("Rover {target_id}").into_bytes()
        } else {
            rec.name.into_bytes()
        },
        Level: rec.level,
        OriginWorldLevel: rec.origin_world_level,
        CurWorldLevel: rec.cur_world_level,
        HeadId: rec.head_id,
        HeadFrameId: rec.head_frame_id,
        Signature: rec.signature.into_bytes(),
        IsOnline: false,
        Sex: rec.sex,
        ..Default::default()
    }
}

pub fn handle_friend_all_request(
    state: &mut State,
    _req: FriendAllRequest,
) -> Result<FriendAllResponse, String> {
    let rec = CoopService::load_social(&state.fs, state.player_id).unwrap_or_default();
    let friends = rec
        .friends
        .iter()
        .map(|f| FriendInfo {
            Info: Some(resolve_player_details(state, f.id)),
            Remark: f.remark.as_bytes().to_vec(),
        })
        .collect();
    let applies = rec
        .pending_applies
        .iter()
        .map(|&from_id| FriendApply {
            Info: Some(resolve_player_details(state, from_id)),
            CreatedTime: 1,
        })
        .collect();

    Ok(FriendAllResponse {
        FriendInfoList: friends,
        FriendApplyList: applies,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_player_basic_info_get_request(
    state: &mut State,
    req: PlayerBasicInfoGetRequest,
) -> Result<PlayerBasicInfoGetResponse, String> {
    if req.Id <= 0 {
        return Ok(PlayerBasicInfoGetResponse {
            Info: None,
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    }
    Ok(PlayerBasicInfoGetResponse {
        Info: Some(resolve_player_details(state, req.Id)),
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_friend_apply_send_request(
    state: &mut State,
    req: FriendApplySendRequest,
) -> Result<FriendApplySendResponse, String> {
    let Some(coop) = state.coop_service.as_ref() else {
        return Ok(FriendApplySendResponse {
            ErrorCode: Some(ErrorCode::InternalError),
        });
    };
    let err = coop
        .send_friend_apply(&state.fs, state.player_id, req.Id)
        .map_err(|e| e.to_string())?;
    Ok(FriendApplySendResponse {
        ErrorCode: Some(err),
    })
}

pub fn handle_friend_apply_handle_request(
    state: &mut State,
    req: FriendApplyHandleRequest,
) -> Result<FriendApplyHandleResponse, String> {
    let Some(coop) = state.coop_service.as_ref() else {
        return Ok(FriendApplyHandleResponse {
            ErrorCode: Some(ErrorCode::InternalError),
            ..Default::default()
        });
    };
    let accept = req
        .Operator
        .is_none_or(|op| op == sonoro_proto::enums::FriendApplyOperator::Approve);
    let mut handled = std::collections::BTreeMap::new();
    for from_id in req.Ids {
        let code = match coop.handle_friend_apply(&state.fs, state.player_id, from_id, accept) {
            Ok(()) => ErrorCode::Success,
            Err(_) => ErrorCode::InternalError,
        };
        handled.insert(from_id, code);
        let _ = state.conn.push_msg(
            FriendApplyDeletedNotify::MSG_ID,
            &FriendApplyDeletedNotify { Id: from_id },
        );
    }
    Ok(FriendApplyHandleResponse {
        ErrorCode: Some(ErrorCode::Success),
        HandledMap: handled,
    })
}

pub fn handle_friend_delete_request(
    state: &mut State,
    req: FriendDeleteRequest,
) -> Result<FriendDeleteResponse, String> {
    let Some(coop) = state.coop_service.as_ref() else {
        return Ok(FriendDeleteResponse {
            ErrorCode: Some(ErrorCode::InternalError),
        });
    };
    coop.delete_friend(&state.fs, state.player_id, req.Id)
        .map_err(|e| e.to_string())?;
    Ok(FriendDeleteResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_friend_remark_request(
    state: &mut State,
    req: FriendRemarkRequest,
) -> Result<FriendRemarkResponse, String> {
    let Some(coop) = state.coop_service.as_ref() else {
        return Ok(FriendRemarkResponse {
            ErrorCode: Some(ErrorCode::InternalError),
        });
    };
    let err = {
        let _guard = coop.social_lock();
        let mut rec = CoopService::load_social(&state.fs, state.player_id).unwrap_or_default();
        let remark = String::from_utf8_lossy(&req.Remark).into_owned();
        if let Some(friend) = rec.friends.iter_mut().find(|f| f.id == req.Id) {
            friend.remark = remark;
            let _ = CoopService::save_social(&state.fs, state.player_id, &rec);
            ErrorCode::Success
        } else {
            ErrorCode::RequestParamError
        }
    };
    Ok(FriendRemarkResponse {
        ErrorCode: Some(err),
    })
}

pub fn handle_friend_recently_team_request(
    state: &mut State,
    _req: FriendRecentlyTeamRequest,
) -> Result<FriendRecentlyTeamResponse, String> {
    let rec = CoopService::load_social(&state.fs, state.player_id).unwrap_or_default();
    let infos = rec
        .recent_teams
        .iter()
        .map(|rt| RecentlyTeamInfo {
            Info: Some(resolve_player_details(state, rt.id)),
            TeamTime: rt.team_time,
        })
        .collect();
    Ok(FriendRecentlyTeamResponse {
        Infos: infos,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_block_player_request(
    state: &mut State,
    req: BlockPlayerRequest,
) -> Result<BlockPlayerResponse, String> {
    if state.coop_service.is_none() {
        return Ok(BlockPlayerResponse {
            ErrorCode: Some(ErrorCode::InternalError),
            Info: None,
        });
    }
    CoopService::block_player(&state.fs, state.player_id, req.Id, true)
        .map_err(|e| e.to_string())?;
    Ok(BlockPlayerResponse {
        ErrorCode: Some(ErrorCode::Success),
        Info: Some(resolve_player_details(state, req.Id)),
    })
}

pub fn handle_unblock_player_request(
    state: &mut State,
    req: UnblockPlayerRequest,
) -> Result<UnblockPlayerResponse, String> {
    if state.coop_service.is_none() {
        return Ok(UnblockPlayerResponse {
            ErrorCode: Some(ErrorCode::InternalError),
        });
    }
    CoopService::block_player(&state.fs, state.player_id, req.Id, false)
        .map_err(|e| e.to_string())?;
    Ok(UnblockPlayerResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_block_list_request(
    state: &mut State,
    _req: BlockListRequest,
) -> Result<BlockListResponse, String> {
    let rec = CoopService::load_social(&state.fs, state.player_id).unwrap_or_default();
    let list = rec
        .blocked
        .iter()
        .map(|&bid| resolve_player_details(state, bid))
        .collect();
    Ok(BlockListResponse {
        BlockList: list,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_report_player_request(
    state: &mut State,
    req: ReportPlayerRequest,
) -> Result<ReportPlayerResponse, String> {
    let key = format!("social:report:{}", req.TargetPlayerId);
    let cnt = state
        .player_components
        .progress
        .info
        .add_key_int(&key, 1);
    let _ = state.player_components.progress.save(&state.fs);
    tracing::info!(
        target: "social",
        "player {} reported {} (reason={}, source={})",
        state.player_id,
        req.TargetPlayerId,
        req.ReportReason,
        req.ReportSource
    );
    Ok(ReportPlayerResponse {
        ReportCount: cnt.clamp(0, i64::from(i32::MAX)) as i32,
        ErrorCode: Some(ErrorCode::Success),
    })
}
