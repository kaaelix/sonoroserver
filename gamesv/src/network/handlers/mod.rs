pub mod access_path;
pub mod achievement;
pub mod activity;
pub mod adventure;
pub mod attach;
pub mod auth;
pub mod buff;
pub mod camera;
pub mod chat;
pub mod combat;
pub mod combat_envelope_handlers;
pub mod combat_sync;
pub mod compat_3811;
pub mod coop;
pub mod craft;
pub mod damage;
pub mod direct_train;
pub mod echo;
pub mod energy;
pub mod endgame;
pub mod events_major;
pub mod events_misc;
pub mod explore;
pub mod fishing;
pub mod flow;
pub mod formation;
pub mod friend;
pub mod fsm;
pub mod gacha;
pub mod guide;
pub mod inst;
pub mod inventory;
pub mod liveness;
pub mod lord_gym;
pub mod mail;
pub mod map;
pub mod minimal;
pub mod motor;
pub mod music;
pub mod phantom_battle;
pub mod player;
pub mod profile;
pub mod qol;
pub mod quest;
pub mod region_prog;
pub mod roguelike;
pub mod role;
pub mod scene;
pub mod settings;
pub mod shop;
pub mod skill;
pub mod skin;
pub mod teleport;
pub mod time;
pub mod tower;
pub mod voxel;
pub mod weapon;
pub mod world_puzzle;

use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::helpers::reward::{self, RewardItem};
use sonoro_proto::PbMessage;
use sonoro_proto::pb;
use sonoro_proto::registry::msg_id::{
    ACHIEVEMENT_RECEIVE_RESPONSE, BATTLE_PASS_TAKE_ALL_REWARD_RESPONSE,
    BATTLE_PASS_TAKE_REWARD_RESPONSE, BATTLE_PASS_TASK_TAKE_RESPONSE,
    GET_ADVENTURE_REWARD_RESPONSE, HEARTBEAT_REQUEST, LIVENESS_TAKE_RESPONSE,
    LIVENESS_TASK_TAKE_RESPONSE, MONTH_CARD_RESPONSE, MUL_ACHIEVEMENT_RECEIVE_RESPONSE,
    PAY_SHOP_DIRECT_BUY_RESPONSE, TIME_CHECK_REQUEST,
};
use sonoro_proto::registry::response_id_for;

use super::aki_logic_bridge::{self, LogicResponse};
use super::connection::drain_event_queue_best_effort;
use super::message::{Header, Message};
use super::state::State;

#[derive(Debug)]
pub enum DispatchError {
    HandlerNotFound,
    Closed,
    Message(String),
}

fn aki_reward_reason(resp_msg_id: u16) -> i32 {
    match resp_msg_id {
        ACHIEVEMENT_RECEIVE_RESPONSE | MUL_ACHIEVEMENT_RECEIVE_RESPONSE => 19100,
        BATTLE_PASS_TAKE_REWARD_RESPONSE
        | BATTLE_PASS_TAKE_ALL_REWARD_RESPONSE
        | BATTLE_PASS_TASK_TAKE_RESPONSE => 18500,
        LIVENESS_TAKE_RESPONSE | LIVENESS_TASK_TAKE_RESPONSE => 19600,
        PAY_SHOP_DIRECT_BUY_RESPONSE => 8001,
        GET_ADVENTURE_REWARD_RESPONSE => 19500,
        _ => 10001,
    }
}

pub(crate) fn deliver_aki_response(
    state: &mut State,
    rpc_id: u16,
    resp: LogicResponse,
) -> Result<(), DispatchError> {
    if !resp.grants.is_empty() {
        let show_popup = resp.msg_id != MONTH_CARD_RESPONSE;
        let grants: Vec<RewardItem> = resp.grants;
        let mut rctx = state.reward_context();
        reward::grant_items(
            &mut rctx,
            2105,
            show_popup,
            &grants,
            aki_reward_reason(resp.msg_id),
        )
        .map_err(|e| DispatchError::Message(e.to_string()))?;
    }
    for p in resp.pushes {
        state
            .conn
            .push_raw_with_message_id(p.msg_id, &p.payload)
            .map_err(|_| DispatchError::Closed)?;
    }
    state
        .conn
        .respond_raw_with_message_id(rpc_id, resp.msg_id, &resp.payload)
        .map_err(|_| DispatchError::Closed)?;
    Ok(())
}

fn run_req_respond_first<Req, Resp, F>(
    state: &mut State,
    rpc_id: u16,
    msg_id: u16,
    payload: &[u8],
    early_resp: Resp,
    f: F,
) -> Result<(), DispatchError>
where
    Req: PbMessage,
    Resp: PbMessage,
    F: FnOnce(&mut State, &mut EventQueue, Req) -> Result<Resp, String>,
{
    let req = match Req::decode(payload) {
        Ok(r) => r,
        Err(err) => {
            tracing::error!("failed to decode request msg_id={msg_id}: {err}");
            if rpc_id != 0
                && let Some(resp_id) = response_id_for(msg_id)
            {
                let _ = state.conn.respond_with_msg_id(rpc_id, resp_id, &early_resp);
            }
            return Ok(());
        }
    };

    if let Some(resp_id) = response_id_for(msg_id) {
        state
            .conn
            .respond_with_msg_id(rpc_id, resp_id, &early_resp)
            .map_err(|_| DispatchError::Closed)?;
    }

    let mut events = EventQueue::new();
    if let Err(err) = f(state, &mut events, req) {
        tracing::error!("handler failed for msg_id={msg_id}: {err}");
    }
    drain_event_queue_best_effort(&mut events, state);
    Ok(())
}

fn run_req<Req, Resp, F>(
    state: &mut State,
    rpc_id: u16,
    msg_id: u16,
    payload: &[u8],
    fail_fn: fn() -> Resp,
    f: F,
) -> Result<(), DispatchError>
where
    Req: PbMessage,
    Resp: PbMessage,
    F: FnOnce(&mut State, &mut EventQueue, Req) -> Result<Resp, String>,
{
    let req = match Req::decode(payload) {
        Ok(r) => r,
        Err(err) => {
            tracing::error!("failed to decode request msg_id={msg_id}: {err}");
            if rpc_id != 0
                && let Some(resp_id) = response_id_for(msg_id)
            {
                let _ = state.conn.respond_with_msg_id(rpc_id, resp_id, &fail_fn());
            }
            return Ok(());
        }
    };

    let mut events = EventQueue::new();
    let res = f(state, &mut events, req);
    drain_event_queue_best_effort(&mut events, state);

    let resp = match res {
        Ok(r) => r,
        Err(err) => {
            tracing::error!("handler failed for msg_id={msg_id}: {err}");
            fail_fn()
        }
    };
    if let Some(resp_id) = response_id_for(msg_id) {
        state
            .conn
            .respond_with_msg_id(rpc_id, resp_id, &resp)
            .map_err(|_| DispatchError::Closed)?;
    }
    Ok(())
}

fn run_req_respond_before_events<Req, Resp, F>(
    state: &mut State,
    rpc_id: u16,
    msg_id: u16,
    payload: &[u8],
    fail_fn: fn() -> Resp,
    f: F,
) -> Result<(), DispatchError>
where
    Req: PbMessage,
    Resp: PbMessage,
    F: FnOnce(&mut State, &mut EventQueue, Req) -> Result<Resp, String>,
{
    let req = match Req::decode(payload) {
        Ok(r) => r,
        Err(err) => {
            tracing::error!("failed to decode request msg_id={msg_id}: {err}");
            if rpc_id != 0
                && let Some(resp_id) = response_id_for(msg_id)
            {
                let _ = state.conn.respond_with_msg_id(rpc_id, resp_id, &fail_fn());
            }
            return Ok(());
        }
    };

    let mut events = EventQueue::new();
    let resp = match f(state, &mut events, req) {
        Ok(r) => r,
        Err(err) => {
            tracing::error!("handler failed for msg_id={msg_id}: {err}");
            fail_fn()
        }
    };
    if let Some(resp_id) = response_id_for(msg_id) {
        state
            .conn
            .respond_with_msg_id(rpc_id, resp_id, &resp)
            .map_err(|_| DispatchError::Closed)?;
    }
    drain_event_queue_best_effort(&mut events, state);
    Ok(())
}

fn run_push<Req, F>(state: &mut State, msg_id: u16, payload: &[u8], f: F) -> Result<(), DispatchError>
where
    Req: PbMessage,
    F: FnOnce(&mut State, &mut EventQueue, Req) -> Result<(), String>,
{
    let req = match Req::decode(payload) {
        Ok(r) => r,
        Err(err) => {
            tracing::error!("failed to decode push msg_id={msg_id}: {err}");
            return Ok(());
        }
    };
    let mut events = EventQueue::new();
    if let Err(err) = f(state, &mut events, req) {
        tracing::error!("push handler failed for msg_id={msg_id}: {err}");
    }
    drain_event_queue_best_effort(&mut events, state);
    Ok(())
}

macro_rules! req_ev {
    ($state:expr, $rpc:expr, $msg_id:expr, $payload:expr, $req:ty, $resp:ty, $func:expr) => {
        run_req::<$req, $resp, _>(
            $state,
            $rpc,
            $msg_id,
            $payload,
            <$resp as sonoro_proto::FailResponse>::fail_response,
            $func,
        )
    };
}

macro_rules! req_ev_fast {
    ($state:expr, $rpc:expr, $msg_id:expr, $payload:expr, $req:ty, $resp:ty, $func:expr) => {
        run_req_respond_before_events::<$req, $resp, _>(
            $state,
            $rpc,
            $msg_id,
            $payload,
            <$resp as sonoro_proto::FailResponse>::fail_response,
            $func,
        )
    };
}

macro_rules! req_noev {
    ($state:expr, $rpc:expr, $msg_id:expr, $payload:expr, $req:ty, $resp:ty, $func:expr) => {
        run_req::<$req, $resp, _>(
            $state,
            $rpc,
            $msg_id,
            $payload,
            <$resp as sonoro_proto::FailResponse>::fail_response,
            |s, _ev, r| $func(s, r),
        )
    };
}

macro_rules! push_ev {
    ($state:expr, $msg_id:expr, $payload:expr, $req:ty, $func:expr) => {
        run_push::<$req, _>($state, $msg_id, $payload, $func)
    };
}

macro_rules! push_noev {
    ($state:expr, $msg_id:expr, $payload:expr, $req:ty, $func:expr) => {
        run_push::<$req, _>($state, $msg_id, $payload, |s, _ev, r| $func(s, r))
    };
}

pub fn dispatch_message(state: &mut State, message: &Message) -> Result<(), DispatchError> {
    let msg_id = message.header.message_id();
    let rpc_id = match message.header {
        Header::Request(req) => req.rpc_id,
        _ => 0,
    };
    let payload = &message.payload;

    // 1. Universal primary: delegate requests to AkiLogicService (excluding heartbeat, time check, 16076, 29312)
    if matches!(message.header, Header::Request(_))
        && msg_id != 29312
        && msg_id != HEARTBEAT_REQUEST
        && msg_id != TIME_CHECK_REQUEST
        && msg_id != 16076
        && let Ok(resp) = aki_logic_bridge::call_player(state.player_id, 0, msg_id, payload)
        && resp.msg_id != 0
    {
        return deliver_aki_response(state, rpc_id, resp);
    }

    // 2. Native Rust handler dispatch LUT
    match msg_id {
        // access_path
        pb::AccessPathTimeServerConfigRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AccessPathTimeServerConfigRequest,
            pb::AccessPathTimeServerConfigResponse,
            access_path::handle_access_path_time_server_config_request
        ),
        // voxel
        pb::PlayerVoxelEnvRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PlayerVoxelEnvRequest,
            pb::PlayerVoxelEnvResponse,
            voxel::handle_player_voxel_env_request
        ),
        // energy
        pb::EnergySyncRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EnergySyncRequest,
            pb::EnergySyncResponse,
            energy::handle_energy_sync_request
        ),
        // settings
        pb::LanguageSettingUpdateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LanguageSettingUpdateRequest,
            pb::LanguageSettingUpdateResponse,
            settings::handle_language_setting_update_request
        ),
        pb::InputSettingRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::InputSettingRequest,
            pb::InputSettingResponse,
            settings::handle_input_setting_request
        ),
        pb::InputSettingUpdateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::InputSettingUpdateRequest,
            pb::InputSettingUpdateResponse,
            settings::handle_input_setting_update_request
        ),
        // chat
        pb::PrivateChatDataRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PrivateChatDataRequest,
            pb::PrivateChatDataResponse,
            chat::handle_private_chat_data_request
        ),
        pb::PrivateChatHistoryRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PrivateChatHistoryRequest,
            pb::PrivateChatHistoryResponse,
            chat::handle_private_chat_history_request
        ),
        pb::PrivateChatRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PrivateChatRequest,
            pb::PrivateChatResponse,
            chat::handle_private_chat_request
        ),
        pb::ChatRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ChatRequest,
            pb::ChatResponse,
            chat::handle_chat_request
        ),
        pb::ChannelChatRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ChannelChatRequest,
            pb::ChannelChatResponse,
            chat::handle_channel_chat_request
        ),
        // direct_train & activity
        pb::ActivityRequest::MSG_ID => activity::on_activity_request(state, rpc_id, payload),
        pb::DirectTrainGetPlayerIdRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DirectTrainGetPlayerIdRequest,
            pb::DirectTrainGetPlayerIdResponse,
            direct_train::handle_direct_train_get_player_id_request
        ),
        // time
        pb::TimeStopPush::MSG_ID => {
            run_push(state, msg_id, payload, |s, _ev, r| time::handle_time_stop_push(s, r))
        }
        pb::TimeCheckRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TimeCheckRequest,
            pb::TimeCheckResponse,
            time::handle_time_check_request
        ),
        pb::AiControlSwitchRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AiControlSwitchRequest,
            pb::AiControlSwitchResponse,
            time::handle_ai_control_switch_request
        ),
        // player
        pb::HeartbeatRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::HeartbeatRequest,
            pb::HeartbeatResponse,
            player::handle_heartbeat_request
        ),
        // camera
        pb::CameraSettingQueryRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::CameraSettingQueryRequest,
            pb::CameraSettingQueryResponse,
            camera::handle_camera_setting_query_request
        ),
        pb::CameraSettingModifyRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::CameraSettingModifyRequest,
            pb::CameraSettingModifyResponse,
            camera::handle_camera_setting_modify_request
        ),
        pb::SelfPortraitStateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SelfPortraitStateRequest,
            pb::SelfPortraitStateResponse,
            camera::handle_self_portrait_state_request
        ),
        // music
        pb::GetMusicInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::GetMusicInfoRequest,
            pb::GetMusicInfoResponse,
            music::handle_get_music_info_request
        ),
        pb::FavoriteMusicMotorCycleUpdateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FavoriteMusicMotorCycleUpdateRequest,
            pb::FavoriteMusicMotorCycleUpdateResponse,
            music::handle_favorite_music_motorcycle_update_request
        ),
        // lord_gym
        pb::LordGymInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LordGymInfoRequest,
            pb::LordGymInfoResponse,
            lord_gym::handle_lord_gym_info_request
        ),
        pb::LordGymReadRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LordGymReadRequest,
            pb::LordGymReadResponse,
            lord_gym::handle_lord_gym_read_request
        ),
        pb::LordGymBeginRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LordGymBeginRequest,
            pb::LordGymBeginResponse,
            lord_gym::handle_lord_gym_begin_request
        ),
        // liveness
        pb::LivenessRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LivenessRequest,
            pb::LivenessResponse,
            liveness::handle_liveness_request
        ),
        pb::LivenessTakeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LivenessTakeRequest,
            pb::LivenessTakeResponse,
            liveness::handle_liveness_take_request
        ),
        pb::LivenessTaskTakeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LivenessTaskTakeRequest,
            pb::LivenessTaskTakeResponse,
            liveness::handle_liveness_task_take_request
        ),
        // mail
        pb::MailBindInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MailBindInfoRequest,
            pb::MailBindInfoResponse,
            mail::handle_mail_bind_info_request
        ),
        pb::MailReadRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MailReadRequest,
            pb::MailReadResponse,
            mail::handle_mail_read_request
        ),
        pb::MailGetAttachmentRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MailGetAttachmentRequest,
            pb::MailGetAttachmentResponse,
            mail::handle_mail_get_attachment_request
        ),
        pb::MailDeleteRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MailDeleteRequest,
            pb::MailDeleteResponse,
            mail::handle_mail_delete_request
        ),
        pb::MailFavoriteRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MailFavoriteRequest,
            pb::MailFavoriteResponse,
            mail::handle_mail_favorite_request
        ),
        pb::MailBindRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MailBindRequest,
            pb::MailBindResponse,
            mail::handle_mail_bind_request
        ),
        pb::MailBindRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MailBindRewardRequest,
            pb::MailBindRewardResponse,
            mail::handle_mail_bind_reward_request
        ),
        // flow
        pb::FlowEndRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FlowEndRequest,
            pb::FlowEndResponse,
            flow::handle_flow_end_request
        ),
        pb::FlowActionRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FlowActionRequest,
            pb::FlowActionResponse,
            flow::handle_flow_action_request
        ),
        // combat
        pb::CombatSendPackRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::CombatSendPackRequest,
            pb::CombatSendPackResponse,
            combat::handle_combat_send_pack_request
        ),
        // friend
        pb::FriendAllRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FriendAllRequest,
            pb::FriendAllResponse,
            friend::handle_friend_all_request
        ),
        pb::PlayerBasicInfoGetRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PlayerBasicInfoGetRequest,
            pb::PlayerBasicInfoGetResponse,
            friend::handle_player_basic_info_get_request
        ),
        pb::FriendApplySendRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FriendApplySendRequest,
            pb::FriendApplySendResponse,
            friend::handle_friend_apply_send_request
        ),
        pb::FriendApplyHandleRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FriendApplyHandleRequest,
            pb::FriendApplyHandleResponse,
            friend::handle_friend_apply_handle_request
        ),
        pb::FriendDeleteRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FriendDeleteRequest,
            pb::FriendDeleteResponse,
            friend::handle_friend_delete_request
        ),
        pb::FriendRemarkRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FriendRemarkRequest,
            pb::FriendRemarkResponse,
            friend::handle_friend_remark_request
        ),
        pb::FriendRecentlyTeamRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FriendRecentlyTeamRequest,
            pb::FriendRecentlyTeamResponse,
            friend::handle_friend_recently_team_request
        ),
        pb::BlockPlayerRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BlockPlayerRequest,
            pb::BlockPlayerResponse,
            friend::handle_block_player_request
        ),
        pb::UnblockPlayerRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::UnblockPlayerRequest,
            pb::UnblockPlayerResponse,
            friend::handle_unblock_player_request
        ),
        pb::BlockListRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BlockListRequest,
            pb::BlockListResponse,
            friend::handle_block_list_request
        ),
        pb::ReportPlayerRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ReportPlayerRequest,
            pb::ReportPlayerResponse,
            friend::handle_report_player_request
        ),
        // achievement
        pb::AchievementInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AchievementInfoRequest,
            pb::AchievementInfoResponse,
            achievement::handle_achievement_info_request
        ),
        pb::UpdateAchievementInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::UpdateAchievementInfoRequest,
            pb::UpdateAchievementInfoResponse,
            achievement::handle_update_achievement_info_request
        ),
        pb::AchievementFinishRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AchievementFinishRequest,
            pb::AchievementFinishResponse,
            achievement::handle_achievement_finish_request
        ),
        pb::AchievementReceiveRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AchievementReceiveRequest,
            pb::AchievementReceiveResponse,
            achievement::handle_achievement_receive_request
        ),
        pb::MulAchievementReceiveRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MulAchievementReceiveRequest,
            pb::MulAchievementReceiveResponse,
            achievement::handle_mul_achievement_receive_request
        ),
        // profile
        pb::PlayerHeadDataRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PlayerHeadDataRequest,
            pb::PlayerHeadDataResponse,
            profile::handle_player_head_data_request
        ),
        pb::PlayerTitleDataRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PlayerTitleDataRequest,
            pb::PlayerTitleDataResponse,
            profile::handle_player_title_data_request
        ),
        pb::RoleShowListUpdateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleShowListUpdateRequest,
            pb::RoleShowListUpdateResponse,
            profile::handle_role_show_list_update_request
        ),
        pb::ChangeCardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ChangeCardRequest,
            pb::ChangeCardResponse,
            profile::handle_change_card_request
        ),
        pb::ReadCardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ReadCardRequest,
            pb::ReadCardResponse,
            profile::handle_read_card_request
        ),
        pb::ChangePlayerTitleRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ChangePlayerTitleRequest,
            pb::ChangePlayerTitleResponse,
            profile::handle_change_player_title_request
        ),
        pb::ModifyNameRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ModifyNameRequest,
            pb::ModifyNameResponse,
            profile::handle_modify_name_request
        ),
        pb::ChangeHeadPhotoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ChangeHeadPhotoRequest,
            pb::ChangeHeadPhotoResponse,
            profile::handle_change_head_photo_request
        ),
        pb::ModifySignatureRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ModifySignatureRequest,
            pb::ModifySignatureResponse,
            profile::handle_modify_signature_request
        ),
        pb::BirthdayInitRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BirthdayInitRequest,
            pb::BirthdayInitResponse,
            profile::handle_birthday_init_request
        ),
        pb::BirthdayShowSetRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BirthdayShowSetRequest,
            pb::BirthdayShowSetResponse,
            profile::handle_birthday_show_set_request
        ),
        // guide
        pb::GuideInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::GuideInfoRequest,
            pb::GuideInfoResponse,
            guide::handle_guide_info_request
        ),
        pb::GuideTriggerRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::GuideTriggerRequest,
            pb::GuideTriggerResponse,
            guide::handle_guide_trigger_request
        ),
        pb::GuideFinishRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::GuideFinishRequest,
            pb::GuideFinishResponse,
            guide::handle_guide_finish_request
        ),
        pb::IllustratedInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::IllustratedInfoRequest,
            pb::IllustratedInfoResponse,
            guide::handle_illustrated_info_request
        ),
        pb::IllustratedRedDotRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::IllustratedRedDotRequest,
            pb::IllustratedRedDotResponse,
            guide::handle_illustrated_red_dot_request
        ),
        pb::IllustratedReadRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::IllustratedReadRequest,
            pb::IllustratedReadResponse,
            guide::handle_illustrated_read_request
        ),
        pb::RoleIllustratedInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleIllustratedInfoRequest,
            pb::RoleIllustratedInfoResponse,
            guide::handle_role_illustrated_info_request
        ),
        pb::TutorialInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TutorialInfoRequest,
            pb::TutorialInfoResponse,
            guide::handle_tutorial_info_request
        ),
        pb::TutorialUnlockRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TutorialUnlockRequest,
            pb::TutorialUnlockResponse,
            guide::handle_tutorial_unlock_request
        ),
        pb::TutorialReceiveRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TutorialReceiveRequest,
            pb::TutorialReceiveResponse,
            guide::handle_tutorial_receive_request
        ),
        // inst
        pb::EnterInstRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EnterInstRequest,
            pb::EnterInstResponse,
            inst::handle_enter_inst_request
        ),
        pb::EnterBigWorldInstRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EnterBigWorldInstRequest,
            pb::EnterBigWorldInstResponse,
            inst::handle_enter_big_world_inst_request
        ),
        pb::LeaveInstRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LeaveInstRequest,
            pb::LeaveInstResponse,
            inst::handle_leave_inst_request
        ),
        pb::TeleportDungeonActionRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TeleportDungeonActionRequest,
            pb::TeleportDungeonActionResponse,
            inst::handle_teleport_dungeon_action_request
        ),
        pb::TeleportFlowEndRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TeleportFlowEndRequest,
            pb::TeleportFlowEndResponse,
            inst::handle_teleport_flow_end_request
        ),
        pb::SceneSubLevelsRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SceneSubLevelsRequest,
            pb::SceneSubLevelsResponse,
            inst::handle_scene_sub_levels_request
        ),
        pb::ChangeDataLayerFinishRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ChangeDataLayerFinishRequest,
            pb::ChangeDataLayerFinishResponse,
            inst::handle_change_data_layer_finish_request
        ),
        pb::FlowRestartRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FlowRestartRequest,
            pb::FlowRestartResponse,
            inst::handle_flow_restart_request
        ),
        pb::LevelPlayOpenRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LevelPlayOpenRequest,
            pb::LevelPlayOpenResponse,
            inst::handle_level_play_open_request
        ),
        pb::LevelPlayRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LevelPlayRewardRequest,
            pb::LevelPlayRewardResponse,
            inst::handle_level_play_reward_request
        ),
        pb::GetLevelPlayRewardsRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::GetLevelPlayRewardsRequest,
            pb::GetLevelPlayRewardsResponse,
            inst::handle_get_level_play_rewards_request
        ),
        pb::LevelPlayFastReturnPosRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LevelPlayFastReturnPosRequest,
            pb::LevelPlayFastReturnPosResponse,
            inst::handle_level_play_fast_return_pos_request
        ),
        pb::LevelPlaySwitchSportModeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LevelPlaySwitchSportModeRequest,
            pb::LevelPlaySwitchSportModeResponse,
            inst::handle_level_play_switch_sport_mode_request
        ),
        pb::SingleInstRechallengeRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SingleInstRechallengeRequest,
            pb::SingleInstRechallengeResponse,
            inst::handle_single_inst_rechallenge_request
        ),
        pb::ApplyRechallengeRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ApplyRechallengeRequest,
            pb::ApplyRechallengeResponse,
            inst::handle_apply_rechallenge_request
        ),
        pb::InstFightEndPush::MSG_ID => {
            run_push(state, msg_id, payload, inst::handle_inst_fight_end_push)
        }
        pb::InstTeleportToResetPointRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::InstTeleportToResetPointRequest,
            pb::InstTeleportToResetPointResponse,
            inst::handle_inst_teleport_to_reset_point_request
        ),
        // tower
        pb::TowerRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TowerRequest,
            pb::TowerResponse,
            tower::handle_tower_request
        ),
        pb::TowerSeasonUpdateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TowerSeasonUpdateRequest,
            pb::TowerSeasonUpdateResponse,
            tower::handle_tower_season_update_request
        ),
        pb::TowerStartRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TowerStartRequest,
            pb::TowerStartResponse,
            tower::handle_tower_start_request
        ),
        pb::TowerResetRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TowerResetRequest,
            pb::TowerResetResponse,
            tower::handle_tower_reset_request
        ),
        pb::TowerRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TowerRewardRequest,
            pb::TowerRewardResponse,
            tower::handle_tower_reward_request
        ),
        pb::TowerApplyFloorDataRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TowerApplyFloorDataRequest,
            pb::TowerApplyFloorDataResponse,
            tower::handle_tower_apply_floor_data_request
        ),
        pb::TowerFormationRecommendRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TowerFormationRecommendRequest,
            pb::TowerFormationRecommendResponse,
            tower::handle_tower_formation_recommend_request
        ),
        pb::TowerGuideActivityInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TowerGuideActivityInfoRequest,
            pb::TowerGuideActivityInfoResponse,
            tower::handle_tower_guide_activity_info_request
        ),
        pb::TowerGuideActivityRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TowerGuideActivityRewardRequest,
            pb::TowerGuideActivityRewardResponse,
            tower::handle_tower_guide_activity_reward_request
        ),
        // shop
        pb::PayShopInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PayShopInfoRequest,
            pb::PayShopInfoResponse,
            shop::handle_pay_shop_info_request
        ),
        pb::PayShopUpdateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PayShopUpdateRequest,
            pb::PayShopUpdateResponse,
            shop::handle_pay_shop_update_request
        ),
        pb::PayShopItemUpdateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PayShopItemUpdateRequest,
            pb::PayShopItemUpdateResponse,
            shop::handle_pay_shop_item_update_request
        ),
        pb::PayShopBuyRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PayShopBuyRequest,
            pb::PayShopBuyResponse,
            shop::handle_pay_shop_buy_request
        ),
        pb::ShopInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ShopInfoRequest,
            pb::ShopInfoResponse,
            shop::handle_shop_info_request
        ),
        pb::ShopUpdateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ShopUpdateRequest,
            pb::ShopUpdateResponse,
            shop::handle_shop_update_request
        ),
        pb::ShopBuyRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ShopBuyRequest,
            pb::ShopBuyResponse,
            shop::handle_shop_buy_request
        ),
        pb::ItemExchangeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ItemExchangeRequest,
            pb::ItemExchangeResponse,
            shop::handle_item_exchange_request
        ),
        // craft
        pb::ForgeInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ForgeInfoRequest,
            pb::ForgeInfoResponse,
            craft::handle_forge_info_request
        ),
        pb::ForgeFormulaUnlockRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ForgeFormulaUnlockRequest,
            pb::ForgeFormulaUnlockResponse,
            craft::handle_forge_formula_unlock_request
        ),
        pb::ForgeItemRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ForgeItemRequest,
            pb::ForgeItemResponse,
            craft::handle_forge_item_request
        ),
        pb::SynthesisInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SynthesisInfoRequest,
            pb::SynthesisInfoResponse,
            craft::handle_synthesis_info_request
        ),
        pb::SynthesisFormulaUnlockRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SynthesisFormulaUnlockRequest,
            pb::SynthesisFormulaUnlockResponse,
            craft::handle_synthesis_formula_unlock_request
        ),
        pb::SynthesisItemRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SynthesisItemRequest,
            pb::SynthesisItemResponse,
            craft::handle_synthesis_item_request
        ),
        pb::BatchSynthesisItemRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BatchSynthesisItemRequest,
            pb::BatchSynthesisItemResponse,
            craft::handle_batch_synthesis_item_request
        ),
        pb::SynthesisLevelRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SynthesisLevelRewardRequest,
            pb::SynthesisLevelRewardResponse,
            craft::handle_synthesis_level_reward_request
        ),
        pb::CookingDataRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::CookingDataRequest,
            pb::CookingDataResponse,
            craft::handle_cooking_data_request
        ),
        pb::CookFormulaRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::CookFormulaRequest,
            pb::CookFormulaResponse,
            craft::handle_cook_formula_request
        ),
        pb::CookFoodRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::CookFoodRequest,
            pb::CookFoodResponse,
            craft::handle_cook_food_request
        ),
        // explore
        pb::EnterAreaRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EnterAreaRequest,
            pb::EnterAreaResponse,
            explore::handle_enter_area_request
        ),
        pb::VisionExploreSkillSetRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::VisionExploreSkillSetRequest,
            pb::VisionExploreSkillSetResponse,
            explore::handle_vision_explore_skill_set_request
        ),
        pb::ExploreSkillRouletteSetRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ExploreSkillRouletteSetRequest,
            pb::ExploreSkillRouletteSetResponse,
            explore::handle_explore_skill_roulette_set_request
        ),
        pb::SummonRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SummonRequest,
            pb::SummonResponse,
            explore::handle_summon_request
        ),
        pb::Summon3Request::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::Summon3Request,
            pb::Summon3Response,
            explore::handle_summon3_request
        ),
        pb::RemoveSummonEntityRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RemoveSummonEntityRequest,
            pb::RemoveSummonEntityResponse,
            explore::handle_remove_summon_entity_request
        ),
        pb::PlayerAccessCasketRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PlayerAccessCasketRequest,
            pb::PlayerAccessCasketResponse,
            explore::handle_player_access_casket_request
        ),
        pb::EntityAccessCasketRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EntityAccessCasketRequest,
            pb::EntityAccessCasketResponse,
            explore::handle_entity_access_casket_request
        ),
        pb::ExploreEntityTraceRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ExploreEntityTraceRequest,
            pb::ExploreEntityTraceResponse,
            explore::handle_explore_entity_trace_request
        ),
        pb::MoralePosRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MoralePosRequest,
            pb::MoralePosResponse,
            explore::handle_morale_pos_request
        ),
        pb::MoraleTreasureBoxTraceRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MoraleTreasureBoxTraceRequest,
            pb::MoraleTreasureBoxTraceResponse,
            explore::handle_morale_treasure_box_trace_request
        ),
        // compat_3811
        pb::FlowSeqSetPlayerPosRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FlowSeqSetPlayerPosRequest,
            pb::FlowSeqSetPlayerPosResponse,
            compat_3811::handle_flow_seq_set_player_pos_request
        ),
        pb::FinishDownloadQuestResourceRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FinishDownloadQuestResourceRequest,
            pb::FinishDownloadQuestResourceResponse,
            compat_3811::handle_finish_download_quest_resource_request
        ),
        pb::SetQuestResourceStateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SetQuestResourceStateRequest,
            pb::SetQuestResourceStateResponse,
            compat_3811::handle_set_quest_resource_state_request
        ),
        pb::GetDetectionTargetRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::GetDetectionTargetRequest,
            pb::GetDetectionTargetResponse,
            compat_3811::handle_get_detection_target_request
        ),
        pb::OutputDebugInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::OutputDebugInfoRequest,
            pb::OutputDebugInfoResponse,
            compat_3811::handle_output_debug_info_request
        ),
        pb::InvalidItemRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::InvalidItemRequest,
            pb::InvalidItemResponse,
            compat_3811::handle_invalid_item_request
        ),
        pb::ShowInvalidItemRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ShowInvalidItemRequest,
            pb::ShowInvalidItemResponse,
            compat_3811::handle_show_invalid_item_request
        ),
        pb::GameStatsShowRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::GameStatsShowRequest,
            pb::GameStatsShowResponse,
            compat_3811::handle_game_stats_show_request
        ),
        pb::ThirdPartyShareRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ThirdPartyShareRequest,
            pb::ThirdPartyShareResponse,
            compat_3811::handle_third_party_share_request
        ),
        pb::LevelPlayStateListRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LevelPlayStateListRequest,
            pb::LevelPlayStateListResponse,
            compat_3811::handle_level_play_state_list_request
        ),
        pb::IllustratedUnlockRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::IllustratedUnlockRequest,
            pb::IllustratedUnlockResponse,
            compat_3811::handle_illustrated_unlock_request
        ),
        pb::EntityPatrolStartRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EntityPatrolStartRequest,
            pb::EntityPatrolStartResponse,
            compat_3811::handle_entity_patrol_start_request
        ),
        pb::LandingDamageRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LandingDamageRequest,
            pb::LandingDamageResponse,
            compat_3811::handle_landing_damage_request
        ),
        pb::SceneBlockChangePush::MSG_ID => run_push(
            state,
            msg_id,
            payload,
            compat_3811::handle_scene_block_change_push,
        ),
        pb::MapOpenPush::MSG_ID => run_push(state, msg_id, payload, compat_3811::handle_map_open_push),
        pb::AceAntiDataPush::MSG_ID => {
            run_push(state, msg_id, payload, compat_3811::handle_ace_anti_data_push)
        }
        pb::SceneBlockSplitClientLoginPush::MSG_ID => run_push(
            state,
            msg_id,
            payload,
            compat_3811::handle_scene_block_split_client_login_push,
        ),
        pb::SceneBlockSplitSwitchStatePush::MSG_ID => run_push(
            state,
            msg_id,
            payload,
            compat_3811::handle_scene_block_split_switch_state_push,
        ),
        // echo & calabash & projector
        pb::PhantomItemRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomItemRequest,
            pb::PhantomItemResponse,
            echo::handle_phantom_item_request
        ),
        pb::PhantomSkinChangeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomSkinChangeRequest,
            pb::PhantomSkinChangeResponse,
            echo::handle_phantom_skin_change_request
        ),
        pb::PhantomPutOnRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomPutOnRequest,
            pb::PhantomPutOnResponse,
            echo::handle_phantom_put_on_request
        ),
        pb::PhantomAutoPutRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomAutoPutRequest,
            pb::PhantomAutoPutResponse,
            echo::handle_phantom_auto_put_request
        ),
        pb::PhantomLevelUpRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomLevelUpRequest,
            pb::PhantomLevelUpResponse,
            echo::upgrade::handle_phantom_level_up_request
        ),
        pb::PhantomIdentifyRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomIdentifyRequest,
            pb::PhantomIdentifyResponse,
            echo::upgrade::handle_phantom_identify_request
        ),
        pb::PhantomPolishRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomPolishRequest,
            pb::PhantomPolishResponse,
            echo::upgrade::handle_phantom_polish_request
        ),
        pb::PhantomBatchPolishRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBatchPolishRequest,
            pb::PhantomBatchPolishResponse,
            echo::upgrade::handle_phantom_batch_polish_request
        ),
        pb::PhantomVicePolishRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomVicePolishRequest,
            pb::PhantomVicePolishResponse,
            echo::upgrade::handle_phantom_vice_polish_request
        ),
        pb::PhantomVicePolishAckRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomVicePolishAckRequest,
            pb::PhantomVicePolishAckResponse,
            echo::upgrade::handle_phantom_vice_polish_ack_request
        ),
        pb::PhantomRefiningRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomRefiningRequest,
            pb::PhantomRefiningResponse,
            echo::refine::handle_phantom_refining_request
        ),
        pb::PhantomBatchRefiningRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBatchRefiningRequest,
            pb::PhantomBatchRefiningResponse,
            echo::refine::handle_phantom_batch_refining_request
        ),
        // Note: CalabashBatchRefiningRequest shares MSG_ID with PhantomBatchRefiningRequest.
        pb::PhantomBatchDirectRefiningRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBatchDirectRefiningRequest,
            pb::PhantomBatchDirectRefiningResponse,
            echo::refine::handle_phantom_batch_direct_refining_request
        ),
        pb::CalabashLevelRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::CalabashLevelRewardRequest,
            pb::CalabashLevelRewardResponse,
            echo::calabash::handle_calabash_level_reward_request
        ),
        pb::MulCalabashLevelRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MulCalabashLevelRewardRequest,
            pb::MulCalabashLevelRewardResponse,
            echo::calabash::handle_mul_calabash_level_reward_request
        ),
        pb::PhantomCollectReceiveRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomCollectReceiveRequest,
            pb::PhantomCollectReceiveResponse,
            echo::calabash::handle_phantom_collect_receive_request
        ),
        pb::CalabashMsgRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::CalabashMsgRequest,
            pb::CalabashMsgResponse,
            echo::calabash::handle_calabash_msg_request
        ),
        pb::PhantomInteractionEquipRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomInteractionEquipRequest,
            pb::PhantomInteractionEquipResponse,
            echo::projector::handle_phantom_interaction_equip_request
        ),
        pb::PhantomInteractionSkinChangeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomInteractionSkinChangeRequest,
            pb::PhantomInteractionSkinChangeResponse,
            echo::projector::handle_phantom_interaction_skin_change_request
        ),
        pb::VisionEquipGroupInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::VisionEquipGroupInfoRequest,
            pb::VisionEquipGroupInfoResponse,
            echo::presets::handle_vision_equip_group_info_request
        ),
        pb::AddVisionEquipGroupRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AddVisionEquipGroupRequest,
            pb::AddVisionEquipGroupResponse,
            echo::presets::handle_add_vision_equip_group_request
        ),
        pb::ApplyVisionGroupRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ApplyVisionGroupRequest,
            pb::ApplyVisionGroupResponse,
            echo::presets::handle_apply_vision_group_request
        ),
        pb::PutVisionGroupToTopRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PutVisionGroupToTopRequest,
            pb::PutVisionGroupToTopResponse,
            echo::presets::handle_put_vision_group_to_top_request
        ),
        pb::ChangeVisionGroupNameRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ChangeVisionGroupNameRequest,
            pb::ChangeVisionGroupNameResponse,
            echo::presets::handle_change_vision_group_name_request
        ),
        pb::DeleteVisionEquipGroupRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DeleteVisionEquipGroupRequest,
            pb::DeleteVisionEquipGroupResponse,
            echo::presets::handle_delete_vision_equip_group_request
        ),
        // Note: PhantomEquipGroupInfosRequest / AddPhantomEquipGroupRequest /
        // TopPhantomEquipGroupRequest / ChangePhantomEquipGroupNameRequest /
        // DeletePhantomEquipGroupRequest / UsePhantomEquipGroupRequest share the same MSG_IDs
        // as the VisionEquipGroup* requests above.
        pb::PhantomBatchOperRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBatchOperRequest,
            pb::PhantomBatchOperResponse,
            echo::presets::handle_phantom_batch_oper_request
        ),
        pb::PhantomManageConfigRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomManageConfigRequest,
            pb::PhantomManageConfigResponse,
            minimal::handle_phantom_manage_config_request
        ),
        pb::PhantomManageConfigUpdateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomManageConfigUpdateRequest,
            pb::PhantomManageConfigUpdateResponse,
            minimal::handle_phantom_manage_config_update_request
        ),
        pb::PhantomAttrRecommendRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomAttrRecommendRequest,
            pb::PhantomAttrRecommendResponse,
            echo::presets::handle_phantom_attr_recommend_request
        ),
        pb::PhantomFetterRecommendRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomFetterRecommendRequest,
            pb::PhantomFetterRecommendResponse,
            echo::presets::handle_phantom_fetter_recommend_request
        ),
        // adventure
        pb::AdventureManualRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AdventureManualRequest,
            pb::AdventureManualResponse,
            adventure::handle_adventure_manual_request
        ),
        pb::AdventureManualDataRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AdventureManualDataRequest,
            pb::AdventureManualDataResponse,
            adventure::handle_adventure_manual_data_request
        ),
        pb::WeeklyFrameworkInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::WeeklyFrameworkInfoRequest,
            pb::WeeklyFrameworkInfoResponse,
            adventure::handle_weekly_framework_info_request
        ),
        pb::GetAdventureRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::GetAdventureRewardRequest,
            pb::GetAdventureRewardResponse,
            adventure::handle_get_adventure_reward_request
        ),
        pb::MulGetAdventureRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MulGetAdventureRewardRequest,
            pb::MulGetAdventureRewardResponse,
            adventure::handle_mul_get_adventure_reward_request
        ),
        pb::GetChapterRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::GetChapterRewardRequest,
            pb::GetChapterRewardResponse,
            adventure::handle_get_chapter_reward_request
        ),
        pb::AddChapterResultRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AddChapterResultRequest,
            pb::AddChapterResultResponse,
            adventure::handle_add_chapter_result_request
        ),
        pb::ChapterResultRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ChapterResultRewardRequest,
            pb::ChapterResultRewardResponse,
            adventure::handle_chapter_result_reward_request
        ),
        pb::ReceivedSilentFirstAwardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ReceivedSilentFirstAwardRequest,
            pb::ReceivedSilentFirstAwardResponse,
            adventure::handle_received_silent_first_award_request
        ),
        pb::NormalMonsterManualInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::NormalMonsterManualInfoRequest,
            pb::NormalMonsterManualInfoResponse,
            adventure::handle_normal_monster_manual_info_request
        ),
        pb::WeeklyFrameworkScoreRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::WeeklyFrameworkScoreRewardRequest,
            pb::WeeklyFrameworkScoreRewardResponse,
            adventure::handle_weekly_framework_score_reward_request
        ),
        // inventory (20767 ItemDisuseRequest == ItemDeprecateRequest)
        pb::NormalItemRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::NormalItemRequest,
            pb::NormalItemResponse,
            inventory::handle_normal_item_request
        ),
        pb::ItemLockRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ItemLockRequest,
            pb::ItemLockResponse,
            inventory::handle_item_lock_request
        ),
        pb::ItemDisuseRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ItemDisuseRequest,
            pb::ItemDisuseResponse,
            inventory::handle_item_disuse_request
        ),
        pb::ItemUseRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ItemUseRequest,
            pb::ItemUseResponse,
            inventory::handle_item_use_request
        ),
        pb::ItemGiftUseRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ItemGiftUseRequest,
            pb::ItemGiftUseResponse,
            inventory::handle_item_gift_use_request
        ),
        pb::ItemDecomposePreviewRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ItemDecomposePreviewRequest,
            pb::ItemDecomposePreviewResponse,
            inventory::handle_item_decompose_preview_request
        ),
        pb::ItemDecomposeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ItemDecomposeRequest,
            pb::ItemDecomposeResponse,
            inventory::handle_item_decompose_request
        ),
        pb::GiftPackPreviewRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::GiftPackPreviewRequest,
            pb::GiftPackPreviewResponse,
            inventory::handle_gift_pack_preview_request
        ),
        pb::GetInstExchangeRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::GetInstExchangeRewardRequest,
            pb::GetInstExchangeRewardResponse,
            inventory::handle_get_inst_exchange_reward_request
        ),
        pb::HandInItemRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::HandInItemRequest,
            pb::HandInItemResponse,
            inventory::handle_hand_in_item_request
        ),
        pb::NewHandInItemRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::NewHandInItemRequest,
            pb::NewHandInItemResponse,
            inventory::handle_new_hand_in_item_request
        ),
        pb::HandInMingSuRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::HandInMingSuRequest,
            pb::HandInMingSuResponse,
            inventory::handle_hand_in_ming_su_request
        ),
        pb::MingSuHandRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MingSuHandRewardRequest,
            pb::MingSuHandRewardResponse,
            inventory::handle_ming_su_hand_reward_request
        ),
        // weapon
        pb::WeaponItemRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::WeaponItemRequest,
            pb::WeaponItemResponse,
            weapon::handle_weapon_item_request
        ),
        pb::WeaponLevelUpRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::WeaponLevelUpRequest,
            pb::WeaponLevelUpResponse,
            weapon::handle_weapon_level_up_request
        ),
        pb::WeaponBreachRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::WeaponBreachRequest,
            pb::WeaponBreachResponse,
            weapon::handle_weapon_breach_request
        ),
        pb::WeaponResonUpRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::WeaponResonUpRequest,
            pb::WeaponResonUpResponse,
            weapon::handle_weapon_reson_up_request
        ),
        pb::EquipTakeOnRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EquipTakeOnRequest,
            pb::EquipTakeOnResponse,
            weapon::handle_equip_take_on_request
        ),
        // formation
        pb::UpdateFormationRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::UpdateFormationRequest,
            pb::UpdateFormationResponse,
            formation::handle_update_formation_request
        ),
        pb::GetFormationDataRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::GetFormationDataRequest,
            pb::GetFormationDataResponse,
            formation::handle_get_formation_data_request
        ),
        pb::RenameFormationRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RenameFormationRequest,
            pb::RenameFormationResponse,
            formation::handle_rename_formation_request
        ),
        // skin (aliases sharing MSG_IDs: 20817, 26601, 28824, 16426, 25017)
        pb::LoadCalabashSkinInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LoadCalabashSkinInfoRequest,
            pb::LoadCalabashSkinInfoResponse,
            skin::handle_load_calabash_skin_info_request
        ),
        pb::CalabashSkinTakeOnRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::CalabashSkinTakeOnRequest,
            pb::CalabashSkinTakeOnResponse,
            skin::handle_calabash_skin_take_on_request
        ),
        pb::RoleSkinRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleSkinRequest,
            pb::RoleSkinResponse,
            skin::handle_role_skin_request
        ),
        pb::LoadEquipInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LoadEquipInfoRequest,
            pb::LoadEquipInfoResponse,
            skin::handle_load_equip_info_request
        ),
        pb::EquipSkinTakeOnRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EquipSkinTakeOnRequest,
            pb::EquipSkinTakeOnResponse,
            skin::handle_equip_skin_take_on_request
        ),
        pb::EquipSkinUnLoadRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EquipSkinUnLoadRequest,
            pb::EquipSkinUnLoadResponse,
            skin::handle_equip_skin_unload_request
        ),
        pb::FlySkinWearRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FlySkinWearRequest,
            pb::FlySkinWearResponse,
            skin::handle_fly_skin_wear_request
        ),
        pb::FlySkinWearAllRoleRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FlySkinWearAllRoleRequest,
            pb::FlySkinWearAllRoleResponse,
            skin::handle_fly_skin_wear_all_role_request
        ),
        pb::RoleSkinChangeRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleSkinChangeRequest,
            pb::RoleSkinChangeResponse,
            skin::handle_role_skin_change_request
        ),
        pb::ChangeOrnamentRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ChangeOrnamentRequest,
            pb::ChangeOrnamentResponse,
            skin::handle_change_ornament_request
        ),
        pb::FlySkinUnLoadRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FlySkinUnLoadRequest,
            pb::FlySkinUnLoadResponse,
            skin::handle_fly_skin_unload_request
        ),
        pb::FlySkinAllUnLoadRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FlySkinAllUnLoadRequest,
            pb::FlySkinAllUnLoadResponse,
            skin::handle_fly_skin_all_unload_request
        ),
        pb::SkinRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SkinRewardRequest,
            pb::SkinRewardResponse,
            skin::handle_skin_reward_request
        ),
        pb::PhantomMusicSaveRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomMusicSaveRequest,
            pb::PhantomMusicSaveResponse,
            skin::handle_phantom_music_save_request
        ),
        pb::PhantomMusicSwitchRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomMusicSwitchRequest,
            pb::PhantomMusicSwitchResponse,
            skin::handle_phantom_music_switch_request
        ),
        pb::PhantomMusicUnlockRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomMusicUnlockRequest,
            pb::PhantomMusicUnlockResponse,
            skin::handle_phantom_music_unlock_request
        ),
        // qol
        pb::RoleFavorUnLockRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleFavorUnLockRequest,
            pb::RoleFavorUnLockResponse,
            qol::handle_role_favor_unlock_request
        ),
        pb::RoleGiftRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleGiftRewardRequest,
            pb::RoleGiftRewardResponse,
            qol::handle_role_gift_reward_request
        ),
        pb::RoleMotionUnLockRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleMotionUnLockRequest,
            pb::RoleMotionUnLockResponse,
            qol::handle_role_motion_unlock_request
        ),
        pb::RoleTrainRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleTrainRequest,
            pb::RoleTrainResponse,
            qol::handle_role_train_request
        ),
        pb::RoleUpdateDevelopTargetRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleUpdateDevelopTargetRequest,
            pb::RoleUpdateDevelopTargetResponse,
            qol::handle_role_update_develop_target_request
        ),
        pb::ShortMessageInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ShortMessageInfoRequest,
            pb::ShortMessageInfoResponse,
            qol::handle_short_message_info_request
        ),
        pb::ShortMessageReadRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ShortMessageReadRequest,
            pb::ShortMessageReadResponse,
            qol::handle_short_message_read_request
        ),
        pb::ShortMessageReceivedRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ShortMessageReceivedRequest,
            pb::ShortMessageReceivedResponse,
            qol::handle_short_message_received_request
        ),
        pb::ShortMessageReplyRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ShortMessageReplyRequest,
            pb::ShortMessageReplyResponse,
            qol::handle_short_message_reply_request
        ),
        pb::ShortMessageUpdateRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ShortMessageUpdateRequest,
            pb::ShortMessageUpdateResponse,
            qol::handle_short_message_update_request
        ),
        pb::ShortMessageChatCharmChangeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ShortMessageChatCharmChangeRequest,
            pb::ShortMessageChatCharmChangeResponse,
            qol::handle_short_message_chat_charm_change_request
        ),
        pb::ShortMessageChatThemeChangeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ShortMessageChatThemeChangeRequest,
            pb::ShortMessageChatThemeChangeResponse,
            qol::handle_short_message_chat_theme_change_request
        ),
        pb::ShortMessageShowChangeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ShortMessageShowChangeRequest,
            pb::ShortMessageShowChangeResponse,
            qol::handle_short_message_show_change_request
        ),
        pb::BirthDayCardRoleRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BirthDayCardRoleRequest,
            pb::BirthDayCardRoleResponse,
            qol::handle_birthday_card_role_request
        ),
        pb::BirthDayRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BirthDayRewardRequest,
            pb::BirthDayRewardResponse,
            qol::handle_birthday_reward_request
        ),
        pb::BattlePassRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BattlePassRequest,
            pb::BattlePassResponse,
            qol::handle_battle_pass_request
        ),
        pb::BattlePassEnterPush::MSG_ID => run_push::<pb::BattlePassEnterPush, _>(
            state,
            msg_id,
            payload,
            |s, _ev, r| qol::handle_battle_pass_enter_push(s, r),
        ),
        pb::BattlePassLevelUpRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BattlePassLevelUpRequest,
            pb::BattlePassLevelUpResponse,
            qol::handle_battle_pass_level_up_request
        ),
        pb::BattlePassTakeRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BattlePassTakeRewardRequest,
            pb::BattlePassTakeRewardResponse,
            qol::handle_battle_pass_take_reward_request
        ),
        pb::BattlePassTakeAllRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BattlePassTakeAllRewardRequest,
            pb::BattlePassTakeAllRewardResponse,
            qol::handle_battle_pass_take_all_reward_request
        ),
        pb::BattlePassTaskRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BattlePassTaskRequest,
            pb::BattlePassTaskResponse,
            qol::handle_battle_pass_task_request
        ),
        pb::BattlePassTaskTakeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BattlePassTaskTakeRequest,
            pb::BattlePassTaskTakeResponse,
            qol::handle_battle_pass_task_take_request
        ),
        pb::BattlePassRecurringTakeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BattlePassRecurringTakeRequest,
            pb::BattlePassRecurringTakeResponse,
            qol::handle_battle_pass_recurring_take_request
        ),
        pb::MonthCardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MonthCardRequest,
            pb::MonthCardResponse,
            qol::handle_month_card_request
        ),
        pb::WeekCardInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::WeekCardInfoRequest,
            pb::WeekCardInfoResponse,
            qol::handle_week_card_info_request
        ),
        pb::WeekCardRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::WeekCardRewardRequest,
            pb::WeekCardRewardResponse,
            qol::handle_week_card_reward_request
        ),
        pb::PayGiftInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PayGiftInfoRequest,
            pb::PayGiftInfoResponse,
            qol::handle_pay_gift_info_request
        ),
        pb::PayGiftRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PayGiftRequest,
            pb::PayGiftResponse,
            qol::handle_pay_gift_request
        ),
        pb::PayItemRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PayItemRequest,
            pb::PayItemResponse,
            qol::handle_pay_item_request
        ),
        pb::PayShopDirectBuyRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PayShopDirectBuyRequest,
            pb::PayShopDirectBuyResponse,
            qol::handle_pay_shop_direct_buy_request
        ),
        pb::GachaAccumulateInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::GachaAccumulateInfoRequest,
            pb::GachaAccumulateInfoResponse,
            qol::handle_gacha_accumulate_info_request
        ),
        pb::GachaAccumulateRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::GachaAccumulateRewardRequest,
            pb::GachaAccumulateRewardResponse,
            qol::handle_gacha_accumulate_reward_request
        ),
        pb::TotalTopUpScoreInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TotalTopUpScoreInfoRequest,
            pb::TotalTopUpScoreInfoResponse,
            qol::handle_total_top_up_score_info_request
        ),
        pb::TotalTopUpRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TotalTopUpRewardRequest,
            pb::TotalTopUpRewardResponse,
            qol::handle_total_top_up_reward_request
        ),
        pb::InfluenceInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::InfluenceInfoRequest,
            pb::InfluenceInfoResponse,
            qol::handle_influence_info_request
        ),
        pb::InfluenceRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::InfluenceRewardRequest,
            pb::InfluenceRewardResponse,
            qol::handle_influence_reward_request
        ),
        pb::StorageInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::StorageInfoRequest,
            pb::StorageInfoResponse,
            qol::handle_storage_info_request
        ),
        pb::StorageInfoUpdateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::StorageInfoUpdateRequest,
            pb::StorageInfoUpdateResponse,
            qol::handle_storage_info_update_request
        ),
        // role (29283 RolePropRequest == RobotRolePropRequest)
        pb::RoleFavorListRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleFavorListRequest,
            pb::RoleFavorListResponse,
            role::handle_role_favor_list_request
        ),
        pb::RoleSexChangeRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleSexChangeRequest,
            pb::RoleSexChangeResponse,
            role::handle_role_sex_change_request
        ),
        pb::RoleElementChangeRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleElementChangeRequest,
            pb::RoleElementChangeResponse,
            role::handle_role_element_change_request
        ),
        pb::RolePropRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RolePropRequest,
            pb::RolePropResponse,
            role::handle_role_prop_request
        ),
        pb::PbUpLevelSkillRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PbUpLevelSkillRequest,
            pb::PbUpLevelSkillResponse,
            role::handle_pb_up_level_skill_request
        ),
        pb::RoleActivateSkillRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleActivateSkillRequest,
            pb::RoleActivateSkillResponse,
            role::handle_role_activate_skill_request
        ),
        pb::RoleSkillLevelUpViewRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleSkillLevelUpViewRequest,
            pb::RoleSkillLevelUpViewResponse,
            role::handle_role_skill_level_up_view_request
        ),
        pb::RoleSkillViewRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleSkillViewRequest,
            pb::RoleSkillViewResponse,
            role::handle_role_skill_view_request
        ),
        pb::RoleSkillQuickLevelUpRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleSkillQuickLevelUpRequest,
            pb::RoleSkillQuickLevelUpResponse,
            role::handle_role_skill_quick_level_up_request
        ),
        pb::PlayerVoiceLanguageRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PlayerVoiceLanguageRequest,
            pb::PlayerVoiceLanguageResponse,
            role::handle_player_voice_language_request
        ),
        pb::PlayerRoleVoiceSetRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PlayerRoleVoiceSetRequest,
            pb::PlayerRoleVoiceSetResponse,
            role::handle_player_role_voice_set_request
        ),
        pb::ResonantChainUnlockRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ResonantChainUnlockRequest,
            pb::ResonantChainUnlockResponse,
            role::handle_resonant_chain_unlock_request
        ),
        pb::RoleSkillBranchModifyRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleSkillBranchModifyRequest,
            pb::RoleSkillBranchModifyResponse,
            role::handle_role_skill_branch_modify_request
        ),
        pb::RoleLevelUpViewRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleLevelUpViewRequest,
            pb::RoleLevelUpViewResponse,
            role::handle_role_level_up_view_request
        ),
        pb::PbUpLevelRoleRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PbUpLevelRoleRequest,
            pb::PbUpLevelRoleResponse,
            role::handle_pb_up_level_role_request
        ),
        pb::RoleBreakThroughViewRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleBreakThroughViewRequest,
            pb::RoleBreakThroughViewResponse,
            role::handle_role_break_through_view_request
        ),
        pb::PbOverRoleRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PbOverRoleRequest,
            pb::PbOverRoleResponse,
            role::handle_pb_over_role_request
        ),
        pb::PbGetRoleListRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PbGetRoleListRequest,
            pb::PbGetRoleListResponse,
            role::handle_pb_get_role_list_request
        ),
        pb::CharacterDeathRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::CharacterDeathRequest,
            pb::CharacterDeathResponse,
            role::handle_character_death_request
        ),
        // buff
        pb::AttributeChangedRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AttributeChangedRequest,
            pb::AttributeChangedResponse,
            buff::handle_attribute_changed_request
        ),
        // quest
        pb::QuestReviewInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::QuestReviewInfoRequest,
            pb::QuestReviewInfoResponse,
            quest::handle_quest_review_info_request
        ),
        pb::SetFocusModeDeterCondRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SetFocusModeDeterCondRequest,
            pb::SetFocusModeDeterCondResponse,
            quest::handle_set_focus_mode_deter_cond_request
        ),
        pb::SetQuestFocusModeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SetQuestFocusModeRequest,
            pb::SetQuestFocusModeResponse,
            quest::handle_set_quest_focus_mode_request
        ),
        pb::CancelQuestFocusModeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::CancelQuestFocusModeRequest,
            pb::CancelQuestFocusModeResponse,
            quest::handle_cancel_quest_focus_mode_request
        ),
        pb::AcceptFocusWaitQuestRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AcceptFocusWaitQuestRequest,
            pb::AcceptFocusWaitQuestResponse,
            quest::handle_accept_focus_wait_quest_request
        ),
        pb::ConfirmQuestResourceRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ConfirmQuestResourceRequest,
            pb::ConfirmQuestResourceResponse,
            quest::handle_confirm_quest_resource_request
        ),
        pb::QuestBranchRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::QuestBranchRequest,
            pb::QuestBranchResponse,
            quest::handle_quest_branch_request
        ),
        pb::QuestBranchSelectRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::QuestBranchSelectRequest,
            pb::QuestBranchSelectResponse,
            quest::handle_quest_branch_select_request
        ),
        pb::ConditionGroupStateRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ConditionGroupStateRequest,
            pb::ConditionGroupStateResponse,
            quest::handle_condition_group_state_request
        ),
        pb::DailyQuestGetRewardRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DailyQuestGetRewardRequest,
            pb::DailyQuestGetRewardResponse,
            quest::handle_daily_quest_get_reward_request
        ),
        pb::DailyQuestChangePreferRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DailyQuestChangePreferRequest,
            pb::DailyQuestChangePreferResponse,
            quest::handle_daily_quest_change_prefer_request
        ),
        pb::SubmitPhotoNodeRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SubmitPhotoNodeRequest,
            pb::SubmitPhotoNodeResponse,
            quest::handle_submit_photo_node_request
        ),
        pb::SubmitNodeRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SubmitNodeRequest,
            pb::SubmitNodeResponse,
            quest::handle_submit_node_request
        ),
        pb::TimerEndRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TimerEndRequest,
            pb::TimerEndResponse,
            quest::handle_timer_end_request
        ),
        pb::BtSetTimerInfoRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BtSetTimerInfoRequest,
            pb::BtSetTimerInfoResponse,
            quest::handle_bt_set_timer_info_request
        ),
        pb::TraceQuestRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TraceQuestRequest,
            pb::TraceQuestResponse,
            quest::handle_trace_quest_request
        ),
        pb::QuestRedDotRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::QuestRedDotRequest,
            pb::QuestRedDotResponse,
            quest::handle_quest_red_dot_request
        ),
        pb::ActionFinishRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ActionFinishRequest,
            pb::ActionFinishResponse,
            quest::handle_action_finish_request
        ),
        pb::BtGiveUpRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BtGiveUpRequest,
            pb::BtGiveUpResponse,
            quest::handle_bt_give_up_request
        ),
        pb::BtRollbackRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BtRollbackRequest,
            pb::BtRollbackResponse,
            quest::handle_bt_rollback_request
        ),
        pb::QuestActionRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::QuestActionRequest,
            pb::QuestActionResponse,
            quest::handle_quest_action_request
        ),
        pb::QuestFinishActionRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::QuestFinishActionRequest,
            pb::QuestFinishActionResponse,
            quest::handle_quest_finish_action_request
        ),
        pb::QuestFinishActionEndRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::QuestFinishActionEndRequest,
            pb::QuestFinishActionEndResponse,
            quest::handle_quest_finish_action_end_request
        ),
        pb::QuestTreeInfoRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::QuestTreeInfoRequest,
            pb::QuestTreeInfoResponse,
            quest::handle_quest_tree_info_request
        ),
        pb::RoleQuestAcceptRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleQuestAcceptRequest,
            pb::RoleQuestAcceptResponse,
            quest::handle_role_quest_accept_request
        ),
        pb::RoleFavorAcceptQuestRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleFavorAcceptQuestRequest,
            pb::RoleFavorAcceptQuestResponse,
            quest::handle_role_favor_accept_quest_request
        ),
        pb::SubmitAwakeAndLoadEntityNodeRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SubmitAwakeAndLoadEntityNodeRequest,
            pb::SubmitAwakeAndLoadEntityNodeResponse,
            quest::handle_submit_awake_and_load_entity_node_request
        ),
        // teleport
        pb::TeleportDataRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TeleportDataRequest,
            pb::TeleportDataResponse,
            teleport::handle_teleport_data_request
        ),
        pb::TeleportFinishRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TeleportFinishRequest,
            pb::TeleportFinishResponse,
            teleport::handle_teleport_finish_request
        ),
        pb::TeleportTransferRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TeleportTransferRequest,
            pb::TeleportTransferResponse,
            teleport::handle_teleport_transfer_request
        ),
        pb::AddTemporaryTeleportRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AddTemporaryTeleportRequest,
            pb::AddTemporaryTeleportResponse,
            teleport::handle_add_temporary_teleport_request
        ),
        pb::RemoveTemporaryTeleportRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RemoveTemporaryTeleportRequest,
            pb::RemoveTemporaryTeleportResponse,
            teleport::handle_remove_temporary_teleport_request
        ),
        pb::TeleportToTargetRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TeleportToTargetRequest,
            pb::TeleportToTargetResponse,
            teleport::handle_teleport_to_target_request
        ),
        // scene
        pb::JoinSceneNotifyConfirmRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::JoinSceneNotifyConfirmRequest,
            pb::JoinSceneNotifyConfirmResponse,
            scene::handle_join_scene_notify_confirm_request
        ),
        pb::UpdateSceneDateRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::UpdateSceneDateRequest,
            pb::UpdateSceneDateResponse,
            scene::handle_update_scene_date_request
        ),
        pb::ChangeWeatherRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ChangeWeatherRequest,
            pb::ChangeWeatherResponse,
            scene::handle_change_weather_request
        ),
        pb::EntityActiveRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EntityActiveRequest,
            pb::EntityActiveResponse,
            scene::handle_entity_active_request
        ),
        pb::PlayerMotionRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PlayerMotionRequest,
            pb::PlayerMotionResponse,
            scene::handle_player_motion_request
        ),
        pb::EntityPositionRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EntityPositionRequest,
            pb::EntityPositionResponse,
            scene::handle_entity_position_request
        ),
        pb::SceneLoadingFinishRequest::MSG_ID => run_req_respond_first(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SceneLoadingFinishResponse {
                ErrorCode: Some(sonoro_proto::enums::ErrorCode::Success),
            },
            scene::handle_scene_loading_finish_request,
        ),
        pb::VehicleRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::VehicleRequest,
            pb::VehicleResponse,
            scene::handle_vehicle_request
        ),
        pb::VehicleFinishRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::VehicleFinishRequest,
            pb::VehicleFinishResponse,
            scene::handle_vehicle_finish_request
        ),
        pb::VehicleChangePassengerRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::VehicleChangePassengerRequest,
            pb::VehicleChangePassengerResponse,
            scene::handle_vehicle_change_passenger_request
        ),
        pb::VehicleRemovePassengerRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::VehicleRemovePassengerRequest,
            pb::VehicleRemovePassengerResponse,
            scene::handle_vehicle_remove_passenger_request
        ),
        pb::VehicleEnterSharingRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::VehicleEnterSharingRequest,
            pb::VehicleEnterSharingResponse,
            scene::handle_vehicle_enter_sharing_request
        ),
        pb::AwakeItemVehicleRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AwakeItemVehicleRequest,
            pb::AwakeItemVehicleResponse,
            scene::handle_awake_item_vehicle_request
        ),
        pb::EntityEnterVehicleRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EntityEnterVehicleRequest,
            pb::EntityEnterVehicleResponse,
            scene::handle_entity_enter_vehicle_request
        ),
        pb::MovePackagePush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::MovePackagePush,
            scene::handle_move_package_push
        ),
        pb::ApplySceneClientEntityIncIdRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ApplySceneClientEntityIncIdRequest,
            pb::ApplySceneClientEntityIncIdResponse,
            scene::handle_apply_scene_client_entity_inc_id_request
        ),
        pb::EntityInteractRequest::MSG_ID => req_ev_fast!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EntityInteractRequest,
            pb::EntityInteractResponse,
            scene::handle_entity_interact_request
        ),
        pb::GetRewardTreasureBoxRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::GetRewardTreasureBoxRequest,
            pb::GetRewardTreasureBoxResponse,
            scene::handle_get_reward_treasure_box_request
        ),
        pb::AnimalDropRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AnimalDropRequest,
            pb::AnimalDropResponse,
            scene::handle_animal_drop_request
        ),
        pb::AnimalDieRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AnimalDieRequest,
            pb::AnimalDieResponse,
            scene::handle_animal_die_request
        ),
        pb::AnimalDestroyRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AnimalDestroyRequest,
            pb::AnimalDestroyResponse,
            scene::handle_animal_destroy_request
        ),
        pb::EntityDynamicInteractRequest::MSG_ID => req_ev_fast!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EntityDynamicInteractRequest,
            pb::EntityDynamicInteractResponse,
            scene::handle_entity_dynamic_interact_request
        ),
        pb::EntityPosAbnormalRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EntityPosAbnormalRequest,
            pb::EntityPosAbnormalResponse,
            scene::handle_entity_pos_abnormal_request
        ),
        // map
        pb::MapTraceRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MapTraceRequest,
            pb::MapTraceResponse,
            map::handle_map_trace_request
        ),
        pb::MapCancelTraceRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MapCancelTraceRequest,
            pb::MapCancelTraceResponse,
            map::handle_map_cancel_trace_request
        ),
        pb::MapTraceInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MapTraceInfoRequest,
            pb::MapTraceInfoResponse,
            map::handle_map_trace_info_request
        ),
        pb::MapMarkRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MapMarkRequest,
            pb::MapMarkResponse,
            map::handle_map_mark_request
        ),
        pb::MapRemoveMarkRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MapRemoveMarkRequest,
            pb::MapRemoveMarkResponse,
            map::handle_map_remove_mark_request
        ),
        pb::MapMarkInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MapMarkInfoRequest,
            pb::MapMarkInfoResponse,
            map::handle_map_mark_info_request
        ),
        pb::MapRemoveAllMarkRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MapRemoveAllMarkRequest,
            pb::MapRemoveAllMarkResponse,
            map::handle_map_remove_all_mark_request
        ),
        pb::MapUnlockFieldInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MapUnlockFieldInfoRequest,
            pb::MapUnlockFieldInfoResponse,
            map::handle_map_unlock_field_info_request
        ),
        pb::ExploreProgressRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ExploreProgressRequest,
            pb::ExploreProgressResponse,
            map::handle_explore_progress_request
        ),
        pb::ExploreProgressRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ExploreProgressRewardRequest,
            pb::ExploreProgressRewardResponse,
            map::handle_explore_progress_reward_request
        ),
        pb::CountryExploreScoreInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::CountryExploreScoreInfoRequest,
            pb::CountryExploreScoreInfoResponse,
            map::handle_country_explore_score_info_request
        ),
        pb::ExploreScoreRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ExploreScoreRewardRequest,
            pb::ExploreScoreRewardResponse,
            map::handle_explore_score_reward_request
        ),
        pb::MulExploreScoreRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MulExploreScoreRewardRequest,
            pb::MulExploreScoreRewardResponse,
            map::handle_mul_explore_score_reward_request
        ),
        pb::PlayerAccessRangeRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PlayerAccessRangeRequest,
            pb::PlayerAccessRangeResponse,
            map::handle_player_access_range_request
        ),
        pb::EntityAccessRangeRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EntityAccessRangeRequest,
            pb::EntityAccessRangeResponse,
            map::handle_entity_access_range_request
        ),
        pb::InitRangeRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::InitRangeRequest,
            pb::InitRangeResponse,
            map::handle_init_range_request
        ),
        pb::InfrV2InfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::InfrV2InfoRequest,
            pb::InfrV2InfoResponse,
            map::handle_infr_v2_info_request
        ),
        pb::InfrInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::InfrInfoRequest,
            pb::InfrInfoResponse,
            map::handle_infr_info_request
        ),
        pb::MapTravelLevelUpRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MapTravelLevelUpRequest,
            pb::MapTravelLevelUpResponse,
            map::handle_map_travel_level_up_request
        ),
        pb::MapTravelFullRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MapTravelFullRewardRequest,
            pb::MapTravelFullRewardResponse,
            map::handle_map_travel_full_reward_request
        ),
        pb::MulMapTravelSoarRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MulMapTravelSoarRewardRequest,
            pb::MulMapTravelSoarRewardResponse,
            map::handle_mul_map_travel_soar_reward_request
        ),
        pb::MulMapTravelTaskRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MulMapTravelTaskRewardRequest,
            pb::MulMapTravelTaskRewardResponse,
            map::handle_mul_map_travel_task_reward_request
        ),
        pb::MapReplaceMarkRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MapReplaceMarkRequest,
            pb::MapReplaceMarkResponse,
            map::handle_map_replace_mark_request
        ),
        pb::MapMarkInfoUpdateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MapMarkInfoUpdateRequest,
            pb::MapMarkInfoUpdateResponse,
            map::handle_map_mark_info_update_request
        ),
        // combat_sync requests
        pb::AnimationStateChangedRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AnimationStateChangedRequest,
            pb::AnimationStateChangedResponse,
            combat_sync::handle_animation_state_changed_request
        ),
        pb::CreateBulletRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::CreateBulletRequest,
            pb::CreateBulletResponse,
            combat_sync::handle_create_bullet_request
        ),
        pb::DamageExecuteRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DamageExecuteRequest,
            pb::DamageExecuteResponse,
            combat_sync::handle_damage_execute_request
        ),
        pb::EntityIsVisibleRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EntityIsVisibleRequest,
            pb::EntityIsVisibleResponse,
            combat_sync::handle_entity_is_visible_request
        ),
        pb::EntityStaticHookMoveRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EntityStaticHookMoveRequest,
            pb::EntityStaticHookMoveResponse,
            combat_sync::handle_entity_static_hook_move_request
        ),
        pb::FsmStateBehaviorRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FsmStateBehaviorRequest,
            pb::FsmStateBehaviorResponse,
            combat_sync::handle_fsm_state_behavior_request
        ),
        pb::HitRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::HitRequest,
            pb::HitResponse,
            combat_sync::handle_hit_request
        ),
        pb::OrderApplyBuffRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::OrderApplyBuffRequest,
            pb::OrderApplyBuffResponse,
            combat_sync::handle_order_apply_buff_request
        ),
        pb::OrderRemoveBuffRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::OrderRemoveBuffRequest,
            pb::OrderRemoveBuffResponse,
            combat_sync::handle_order_remove_buff_request
        ),
        pb::OrderRemoveBuffByTagsRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::OrderRemoveBuffByTagsRequest,
            pb::OrderRemoveBuffByTagsResponse,
            combat_sync::handle_order_remove_buff_by_tags_request
        ),
        pb::SkillRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SkillRequest,
            pb::SkillResponse,
            combat_sync::handle_skill_request
        ),
        pb::UseSkillRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::UseSkillRequest,
            pb::UseSkillResponse,
            combat_sync::handle_use_skill_request
        ),
        pb::ChangeStateRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ChangeStateRequest,
            pb::ChangeStateResponse,
            combat_sync::handle_change_state_request
        ),
        pb::SwitchRoleRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SwitchRoleRequest,
            pb::SwitchRoleResponse,
            combat_sync::handle_switch_role_request
        ),
        pb::FragileChangeRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FragileChangeRequest,
            pb::FragileChangeResponse,
            combat_sync::handle_fragile_change_request
        ),
        pb::ChangeSummonEntityStateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ChangeSummonEntityStateRequest,
            pb::ChangeSummonEntityStateResponse,
            combat_sync::handle_change_summon_entity_state_request
        ),
        pb::ExecuteQteRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ExecuteQteRequest,
            pb::ExecuteQteResponse,
            combat_sync::handle_execute_qte_request
        ),
        pb::BuffItemRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BuffItemRequest,
            pb::BuffItemResponse,
            combat_sync::handle_buff_item_request
        ),
        pb::AtomicProcessAddBuffRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AtomicProcessAddBuffRequest,
            pb::AtomicProcessAddBuffResponse,
            combat_sync::handle_atomic_process_add_buff_request
        ),
        pb::AtomicProcessRemoveBuffRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AtomicProcessRemoveBuffRequest,
            pb::AtomicProcessRemoveBuffResponse,
            combat_sync::handle_atomic_process_remove_buff_request
        ),
        pb::AtomicProcessTeleportRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AtomicProcessTeleportRequest,
            pb::AtomicProcessTeleportResponse,
            combat_sync::handle_atomic_process_teleport_request
        ),
        pb::AvoidBlockClientInitDoneRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AvoidBlockClientInitDoneRequest,
            pb::AvoidBlockClientInitDoneResponse,
            combat_sync::handle_avoid_block_client_init_done_request
        ),
        pb::AvoidBlockSubmitOverNodeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AvoidBlockSubmitOverNodeRequest,
            pb::AvoidBlockSubmitOverNodeResponse,
            combat_sync::handle_avoid_block_submit_over_node_request
        ),
        pb::BeamCastHitPlayerRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BeamCastHitPlayerRequest,
            pb::BeamCastHitPlayerResponse,
            combat_sync::handle_beam_cast_hit_player_request
        ),
        pb::DeadeyeShootHitRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DeadeyeShootHitRequest,
            pb::DeadeyeShootHitResponse,
            combat_sync::handle_deadeye_shoot_hit_request
        ),
        pb::TargetGearHitRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TargetGearHitRequest,
            pb::TargetGearHitResponse,
            combat_sync::handle_target_gear_hit_request
        ),
        pb::ThrowDamageRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ThrowDamageRequest,
            pb::ThrowDamageResponse,
            combat_sync::handle_throw_damage_request
        ),
        pb::ExploreSkillActionRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ExploreSkillActionRequest,
            pb::ExploreSkillActionResponse,
            combat_sync::handle_explore_skill_action_request
        ),
        pb::EntityBuffProducerRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EntityBuffProducerRequest,
            pb::EntityBuffProducerResponse,
            combat_sync::handle_entity_buff_producer_request
        ),
        pb::EntityBuffProducerOperateRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EntityBuffProducerOperateRequest,
            pb::EntityBuffProducerOperateResponse,
            combat_sync::handle_entity_buff_producer_operate_request
        ),
        pb::QuickHackSkillRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::QuickHackSkillRequest,
            pb::QuickHackSkillResponse,
            combat_sync::handle_quick_hack_skill_request
        ),
        pb::CheckUseSoundBoxSkillRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::CheckUseSoundBoxSkillRequest,
            pb::CheckUseSoundBoxSkillResponse,
            combat_sync::handle_check_use_sound_box_skill_request
        ),
        pb::UseSoundBoxSkillRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::UseSoundBoxSkillRequest,
            pb::UseSoundBoxSkillResponse,
            combat_sync::handle_use_sound_box_skill_request
        ),
        pb::GmEntityFsmGroupInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::GmEntityFsmGroupInfoRequest,
            pb::GmEntityFsmGroupInfoResponse,
            combat_sync::handle_gm_entity_fsm_group_info_request
        ),
        pb::SimpleCombatEntityDieRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SimpleCombatEntityDieRequest,
            pb::SimpleCombatEntityDieResponse,
            combat_sync::handle_simple_combat_entity_die_request
        ),
        pb::SetClientGlobalVarRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SetClientGlobalVarRequest,
            pb::SetClientGlobalVarResponse,
            combat_sync::handle_set_client_global_var_request
        ),
        pb::ClientBasicInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ClientBasicInfoRequest,
            pb::ClientBasicInfoResponse,
            combat_sync::handle_client_basic_info_request
        ),
        pb::SimpleTrackReportRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SimpleTrackReportRequest,
            pb::SimpleTrackReportResponse,
            combat_sync::handle_simple_track_report_request
        ),
        pb::SecFBRoundStartFlowRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SecFBRoundStartFlowRequest,
            pb::SecFBRoundStartFlowResponse,
            combat_sync::handle_sec_fb_round_start_flow_request
        ),
        pb::SecFBRoundEndFlowReportRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SecFBRoundEndFlowReportRequest,
            pb::SecFBRoundEndFlowReportResponse,
            combat_sync::handle_sec_fb_round_end_flow_report_request
        ),
        pb::SecRoleFightFlowRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SecRoleFightFlowRequest,
            pb::SecRoleFightFlowResponse,
            combat_sync::handle_sec_role_fight_flow_request
        ),
        pb::SecWorldStartFlowRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SecWorldStartFlowRequest,
            pb::SecWorldStartFlowResponse,
            combat_sync::handle_sec_world_start_flow_request
        ),
        pb::SecWorldInfoFlowRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SecWorldInfoFlowRequest,
            pb::SecWorldInfoFlowResponse,
            combat_sync::handle_sec_world_info_flow_request
        ),
        pb::SecWorldSEndFlowRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SecWorldSEndFlowRequest,
            pb::SecWorldSEndFlowResponse,
            combat_sync::handle_sec_world_s_end_flow_request
        ),
        // combat_sync pushes
        pb::ActivateBuffPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::ActivateBuffPush,
            combat_sync::handle_activate_buff_push
        ),
        pb::LogicStateInitPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::LogicStateInitPush,
            combat_sync::handle_logic_state_init_push
        ),
        pb::RemoveGameplayEffectPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::RemoveGameplayEffectPush,
            combat_sync::handle_remove_gameplay_effect_push
        ),
        pb::ActorVisiblePush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::ActorVisiblePush,
            combat_sync::handle_actor_visible_push
        ),
        pb::EntityIsVisiblePush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::EntityIsVisiblePush,
            combat_sync::handle_entity_is_visible_push
        ),
        pb::EntityStaticHookMovePush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::EntityStaticHookMovePush,
            combat_sync::handle_entity_static_hook_move_push
        ),
        pb::BeforeHitPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::BeforeHitPush,
            combat_sync::handle_before_hit_push
        ),
        pb::CreateBulletPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::CreateBulletPush,
            combat_sync::handle_create_bullet_push
        ),
        pb::DestroyBulletPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::DestroyBulletPush,
            combat_sync::handle_destroy_bullet_push
        ),
        pb::GameplayTagPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::GameplayTagPush,
            combat_sync::handle_gameplay_tag_push
        ),
        pb::RTimeStopAnimPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::RTimeStopAnimPush,
            combat_sync::handle_r_time_stop_anim_push
        ),
        pb::SlowTimePush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::SlowTimePush,
            combat_sync::handle_slow_time_push
        ),
        pb::RoleTrialUiEndPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::RoleTrialUiEndPush,
            combat_sync::handle_role_trial_ui_end_push
        ),
        pb::RoleSkinTrialUiEndPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::RoleSkinTrialUiEndPush,
            combat_sync::handle_role_skin_trial_ui_end_push
        ),
        pb::CheckClientPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::CheckClientPush,
            combat_sync::handle_check_client_push
        ),
        pb::ExecuteQtePush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::ExecuteQtePush,
            combat_sync::handle_execute_qte_push
        ),
        pb::SimpleCombatEntitySyncAttributePush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::SimpleCombatEntitySyncAttributePush,
            combat_sync::handle_simple_combat_entity_sync_attribute_push
        ),
        pb::ANStartPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::ANStartPush,
            combat_sync::handle_an_start_push
        ),
        pb::AnimationGameplayTagPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::AnimationGameplayTagPush,
            combat_sync::handle_animation_gameplay_tag_push
        ),
        pb::AnimationStateChangedPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::AnimationStateChangedPush,
            combat_sync::handle_animation_state_changed_push
        ),
        pb::AnimationStateInitPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::AnimationStateInitPush,
            combat_sync::handle_animation_state_init_push
        ),
        pb::ApplyBuffS2cResponsePush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::ApplyBuffS2cResponsePush,
            combat_sync::handle_apply_buff_s2c_response_push
        ),
        pb::ApplyGameplayEffectPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::ApplyGameplayEffectPush,
            combat_sync::handle_apply_gameplay_effect_push
        ),
        pb::BuffEffectExecutePush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::BuffEffectExecutePush,
            combat_sync::handle_buff_effect_execute_push
        ),
        pb::BuffEffectPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::BuffEffectPush,
            combat_sync::handle_buff_effect_push
        ),
        pb::BuffStackCountPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::BuffStackCountPush,
            combat_sync::handle_buff_stack_count_push
        ),
        pb::BulletPatternPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::BulletPatternPush,
            combat_sync::handle_bullet_pattern_push
        ),
        pb::ClientCurrentRoleReportPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::ClientCurrentRoleReportPush,
            combat_sync::handle_client_current_role_report_push
        ),
        pb::DodgeInfoPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::DodgeInfoPush,
            combat_sync::handle_dodge_info_push
        ),
        pb::EndSkillPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::EndSkillPush,
            combat_sync::handle_end_skill_push
        ),
        pb::FsmConditionPassPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::FsmConditionPassPush,
            combat_sync::handle_fsm_condition_pass_push
        ),
        pb::GameplayCuePush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::GameplayCuePush,
            combat_sync::handle_gameplay_cue_push
        ),
        pb::HitEndPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::HitEndPush,
            combat_sync::handle_hit_end_push
        ),
        pb::InterruptSkillInDelayPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::InterruptSkillInDelayPush,
            combat_sync::handle_interrupt_skill_in_delay_push
        ),
        pb::ModifyBulletParamsPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::ModifyBulletParamsPush,
            combat_sync::handle_modify_bullet_params_push
        ),
        pb::PassiveSkillAddPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::PassiveSkillAddPush,
            combat_sync::handle_passive_skill_add_push
        ),
        pb::PassiveSkillRemovePush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::PassiveSkillRemovePush,
            combat_sync::handle_passive_skill_remove_push
        ),
        pb::QuickHackOpenPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::QuickHackOpenPush,
            combat_sync::handle_quick_hack_open_push
        ),
        pb::QuickHackRamVerifyPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::QuickHackRamVerifyPush,
            combat_sync::handle_quick_hack_ram_verify_push
        ),
        pb::RTimeStopPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::RTimeStopPush,
            combat_sync::handle_r_time_stop_push
        ),
        pb::RefreshBuffDurationPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::RefreshBuffDurationPush,
            combat_sync::handle_refresh_buff_duration_push
        ),
        pb::RemoveBuffByIdS2cResponsePush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::RemoveBuffByIdS2cResponsePush,
            combat_sync::handle_remove_buff_by_id_s2c_response_push
        ),
        pb::RemoveBuffS2cResponsePush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::RemoveBuffS2cResponsePush,
            combat_sync::handle_remove_buff_s2c_response_push
        ),
        pb::RoleGoDownPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::RoleGoDownPush,
            combat_sync::handle_role_go_down_push
        ),
        pb::SwitchLogicStatePush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::SwitchLogicStatePush,
            combat_sync::handle_switch_logic_state_push
        ),
        pb::ToughCalcExtraRatioChangePush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::ToughCalcExtraRatioChangePush,
            combat_sync::handle_tough_calc_extra_ratio_change_push
        ),
        pb::UseSkillFailPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::UseSkillFailPush,
            combat_sync::handle_use_skill_fail_push
        ),
        pb::ChangeStateConfirmPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::ChangeStateConfirmPush,
            combat_sync::handle_change_state_confirm_push
        ),
        pb::BoneVisibleChangePush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::BoneVisibleChangePush,
            combat_sync::handle_bone_visible_change_push
        ),
        pb::CaughtPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::CaughtPush,
            combat_sync::handle_caught_push
        ),
        pb::DrownEndTeleportPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::DrownEndTeleportPush,
            combat_sync::handle_drown_end_teleport_push
        ),
        pb::DrownPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::DrownPush,
            combat_sync::handle_drown_push
        ),
        pb::EnterViewDirectionPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::EnterViewDirectionPush,
            combat_sync::handle_enter_view_direction_push
        ),
        pb::ExitViewDirectionPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::ExitViewDirectionPush,
            combat_sync::handle_exit_view_direction_push
        ),
        pb::MaterialPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::MaterialPush,
            combat_sync::handle_material_push
        ),
        pb::MonsterBoomPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::MonsterBoomPush,
            combat_sync::handle_monster_boom_push
        ),
        pb::MonsterDrownPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::MonsterDrownPush,
            combat_sync::handle_monster_drown_push
        ),
        pb::NewLinkBurstPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::NewLinkBurstPush,
            combat_sync::handle_new_link_burst_push
        ),
        pb::VisionTriggerPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::VisionTriggerPush,
            combat_sync::handle_vision_trigger_push
        ),
        // region_prog
        pb::InfrArchiveReadRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::InfrArchiveReadRequest,
            pb::InfrArchiveReadResponse,
            region_prog::handle_infr_archive_read_request
        ),
        pb::InfrArchiveTaskRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::InfrArchiveTaskRewardRequest,
            pb::InfrArchiveTaskRewardResponse,
            region_prog::handle_infr_archive_task_reward_request
        ),
        pb::InfrFireNoticeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::InfrFireNoticeRequest,
            pb::InfrFireNoticeResponse,
            region_prog::handle_infr_fire_notice_request
        ),
        pb::InfrLevelUpRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::InfrLevelUpRequest,
            pb::InfrLevelUpResponse,
            region_prog::handle_infr_level_up_request
        ),
        pb::InfrManualCancelTraceRoadRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::InfrManualCancelTraceRoadRequest,
            pb::InfrManualCancelTraceRoadResponse,
            region_prog::handle_infr_manual_cancel_trace_road_request
        ),
        pb::InfrManualSwitchTraceRoadRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::InfrManualSwitchTraceRoadRequest,
            pb::InfrManualSwitchTraceRoadResponse,
            region_prog::handle_infr_manual_switch_trace_road_request
        ),
        pb::InfrPhoneTaskRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::InfrPhoneTaskRewardRequest,
            pb::InfrPhoneTaskRewardResponse,
            region_prog::handle_infr_phone_task_reward_request
        ),
        pb::InfrRoadBuildRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::InfrRoadBuildRequest,
            pb::InfrRoadBuildResponse,
            region_prog::handle_infr_road_build_request
        ),
        pb::InfrThemeActivityAwardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::InfrThemeActivityAwardRequest,
            pb::InfrThemeActivityAwardResponse,
            region_prog::handle_infr_theme_activity_award_request
        ),
        pb::InfrV2LevelUpRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::InfrV2LevelUpRequest,
            pb::InfrV2LevelUpResponse,
            region_prog::handle_infr_v2_level_up_request
        ),
        pb::InfrV2ManualCancelTraceTreeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::InfrV2ManualCancelTraceTreeRequest,
            pb::InfrV2ManualCancelTraceTreeResponse,
            region_prog::handle_infr_v2_manual_cancel_trace_tree_request
        ),
        pb::InfrV2ManualSwitchTraceTreeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::InfrV2ManualSwitchTraceTreeRequest,
            pb::InfrV2ManualSwitchTraceTreeResponse,
            region_prog::handle_infr_v2_manual_switch_trace_tree_request
        ),
        pb::InfrV2ScoreRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::InfrV2ScoreRewardRequest,
            pb::InfrV2ScoreRewardResponse,
            region_prog::handle_infr_v2_score_reward_request
        ),
        pb::InfrV2TaskRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::InfrV2TaskRewardRequest,
            pb::InfrV2TaskRewardResponse,
            region_prog::handle_infr_v2_task_reward_request
        ),
        pb::InfrV2TreeBuildRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::InfrV2TreeBuildRequest,
            pb::InfrV2TreeBuildResponse,
            region_prog::handle_infr_v2_tree_build_request
        ),
        pb::SolarSchoolBatchSyncRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SolarSchoolBatchSyncRequest,
            pb::SolarSchoolBatchSyncResponse,
            region_prog::handle_solar_school_batch_sync_request
        ),
        pb::SolarSchoolLoadRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SolarSchoolLoadRequest,
            pb::SolarSchoolLoadResponse,
            region_prog::handle_solar_school_load_request
        ),
        pb::SolarSchoolOneKeyGalleryRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SolarSchoolOneKeyGalleryRewardRequest,
            pb::SolarSchoolOneKeyGalleryRewardResponse,
            region_prog::handle_solar_school_one_key_gallery_reward_request
        ),
        pb::SolarSchoolOneKeyRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SolarSchoolOneKeyRewardRequest,
            pb::SolarSchoolOneKeyRewardResponse,
            region_prog::handle_solar_school_one_key_reward_request
        ),
        pb::SolarSchoolReachEndingRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SolarSchoolReachEndingRequest,
            pb::SolarSchoolReachEndingResponse,
            region_prog::handle_solar_school_reach_ending_request
        ),
        pb::SolarSchoolSaveRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SolarSchoolSaveRequest,
            pb::SolarSchoolSaveResponse,
            region_prog::handle_solar_school_save_request
        ),
        pb::SolarSchoolSetFlowRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SolarSchoolSetFlowRequest,
            pb::SolarSchoolSetFlowResponse,
            region_prog::handle_solar_school_set_flow_request
        ),
        pb::SolarSchoolTaskProgressRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SolarSchoolTaskProgressRewardRequest,
            pb::SolarSchoolTaskProgressRewardResponse,
            region_prog::handle_solar_school_task_progress_reward_request
        ),
        pb::TrackMoonActivityRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TrackMoonActivityRewardRequest,
            pb::TrackMoonActivityRewardResponse,
            region_prog::handle_track_moon_activity_reward_request
        ),
        pb::TrackMoonBuildFlowRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TrackMoonBuildFlowRequest,
            pb::TrackMoonBuildFlowResponse,
            region_prog::handle_track_moon_build_flow_request
        ),
        pb::TrackMoonBuildingDataRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TrackMoonBuildingDataRequest,
            pb::TrackMoonBuildingDataResponse,
            region_prog::handle_track_moon_building_data_request
        ),
        pb::TrackMoonEntrustDataRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TrackMoonEntrustDataRequest,
            pb::TrackMoonEntrustDataResponse,
            region_prog::handle_track_moon_entrust_data_request
        ),
        pb::TrackMoonIllustratedRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TrackMoonIllustratedRequest,
            pb::TrackMoonIllustratedResponse,
            region_prog::handle_track_moon_illustrated_request
        ),
        pb::TrackMoonRoleDataRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TrackMoonRoleDataRequest,
            pb::TrackMoonRoleDataResponse,
            region_prog::handle_track_moon_role_data_request
        ),
        pb::TrackMoonSettleRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TrackMoonSettleRequest,
            pb::TrackMoonSettleResponse,
            region_prog::handle_track_moon_settle_request
        ),
        pb::TrackMoonTargetRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TrackMoonTargetRewardRequest,
            pb::TrackMoonTargetRewardResponse,
            region_prog::handle_track_moon_target_reward_request
        ),
        pb::StartRecallRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::StartRecallRequest,
            pb::StartRecallResponse,
            region_prog::handle_start_recall_request
        ),
        pb::QueryRecallItemBagRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::QueryRecallItemBagRequest,
            pb::QueryRecallItemBagResponse,
            region_prog::handle_query_recall_item_bag_request
        ),
        pb::TraceRecallQuestRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TraceRecallQuestRequest,
            pb::TraceRecallQuestResponse,
            region_prog::handle_trace_recall_quest_request
        ),
        pb::RecallExploreSkillRouletteSetRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RecallExploreSkillRouletteSetRequest,
            pb::RecallExploreSkillRouletteSetResponse,
            region_prog::handle_recall_explore_skill_roulette_set_request
        ),
        pb::RecallItemUseRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RecallItemUseRequest,
            pb::RecallItemUseResponse,
            region_prog::handle_recall_item_use_request
        ),
        pb::RecallQuestActionRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RecallQuestActionRequest,
            pb::RecallQuestActionResponse,
            region_prog::handle_recall_quest_action_request
        ),
        pb::RecallQuestFinishActionRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RecallQuestFinishActionRequest,
            pb::RecallQuestFinishActionResponse,
            region_prog::handle_recall_quest_finish_action_request
        ),
        pb::RecallQuestFinishActionEndRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RecallQuestFinishActionEndRequest,
            pb::RecallQuestFinishActionEndResponse,
            region_prog::handle_recall_quest_finish_action_end_request
        ),
        pb::RecallQuestNpcMoveOverRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RecallQuestNpcMoveOverRequest,
            pb::RecallQuestNpcMoveOverResponse,
            region_prog::handle_recall_quest_npc_move_over_request
        ),
        pb::RecallQuestReadyForGiveUpRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RecallQuestReadyForGiveUpRequest,
            pb::RecallQuestReadyForGiveUpResponse,
            region_prog::handle_recall_quest_ready_for_give_up_request
        ),
        pb::RecallTeleportToMarkIdRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RecallTeleportToMarkIdRequest,
            pb::RecallTeleportToMarkIdResponse,
            region_prog::handle_recall_teleport_to_mark_id_request
        ),
        pb::RoadBookFullRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoadBookFullRewardRequest,
            pb::RoadBookFullRewardResponse,
            region_prog::handle_road_book_full_reward_request
        ),
        pb::RoadBookLevelUpRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoadBookLevelUpRequest,
            pb::RoadBookLevelUpResponse,
            region_prog::handle_road_book_level_up_request
        ),
        pb::RoadBookMotorRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoadBookMotorRewardRequest,
            pb::RoadBookMotorRewardResponse,
            region_prog::handle_road_book_motor_reward_request
        ),
        pb::RoadBookTaskRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoadBookTaskRewardRequest,
            pb::RoadBookTaskRewardResponse,
            region_prog::handle_road_book_task_reward_request
        ),
        pb::RealmBetweenFullRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RealmBetweenFullRewardRequest,
            pb::RealmBetweenFullRewardResponse,
            region_prog::handle_realm_between_full_reward_request
        ),
        pb::RealmBetweenLevelUpRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RealmBetweenLevelUpRequest,
            pb::RealmBetweenLevelUpResponse,
            region_prog::handle_realm_between_level_up_request
        ),
        pb::RealmBetweenMotorRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RealmBetweenMotorRewardRequest,
            pb::RealmBetweenMotorRewardResponse,
            region_prog::handle_realm_between_motor_reward_request
        ),
        pb::RealmBetweenTaskRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RealmBetweenTaskRewardRequest,
            pb::RealmBetweenTaskRewardResponse,
            region_prog::handle_realm_between_task_reward_request
        ),
        pb::NewBieCourseRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::NewBieCourseRewardRequest,
            pb::NewBieCourseRewardResponse,
            region_prog::handle_new_bie_course_reward_request
        ),
        pb::NewJourneyRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::NewJourneyRequest,
            pb::NewJourneyResponse,
            region_prog::handle_new_journey_request
        ),
        pb::NewPlayerSupportRewardTaskRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::NewPlayerSupportRewardTaskRequest,
            pb::NewPlayerSupportRewardTaskResponse,
            region_prog::handle_new_player_support_reward_task_request
        ),
        pb::NewPlayerSupportSetCurUseTrialRoleRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::NewPlayerSupportSetCurUseTrialRoleRequest,
            pb::NewPlayerSupportSetCurUseTrialRoleResponse,
            region_prog::handle_new_player_support_set_cur_use_trial_role_request
        ),
        pb::NewPlayerSupportSetCurUseTrialRoleV2Request::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::NewPlayerSupportSetCurUseTrialRoleV2Request,
            pb::NewPlayerSupportSetCurUseTrialRoleV2Response,
            region_prog::handle_new_player_support_set_cur_use_trial_role_v2_request
        ),
        pb::NewPlayerSupportTrialRoleLvUpRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::NewPlayerSupportTrialRoleLvUpRequest,
            pb::NewPlayerSupportTrialRoleLvUpResponse,
            region_prog::handle_new_player_support_trial_role_lv_up_request
        ),
        pb::NewPlayerSupportTrialRoleLvUpV2Request::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::NewPlayerSupportTrialRoleLvUpV2Request,
            pb::NewPlayerSupportTrialRoleLvUpV2Response,
            region_prog::handle_new_player_support_trial_role_lv_up_v2_request
        ),
        pb::NewTrialRoleGetNightmarePhantomInstInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::NewTrialRoleGetNightmarePhantomInstInfoRequest,
            pb::NewTrialRoleGetNightmarePhantomInstInfoResponse,
            region_prog::handle_new_trial_role_get_nightmare_phantom_inst_info_request
        ),
        pb::NewbieAdventureChapterDropRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::NewbieAdventureChapterDropRequest,
            pb::NewbieAdventureChapterDropResponse,
            region_prog::handle_newbie_adventure_chapter_drop_request
        ),
        pb::NewbieAdventureRoleRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::NewbieAdventureRoleRewardRequest,
            pb::NewbieAdventureRoleRewardResponse,
            region_prog::handle_newbie_adventure_role_reward_request
        ),
        pb::NewbieAdventureTaskRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::NewbieAdventureTaskRewardRequest,
            pb::NewbieAdventureTaskRewardResponse,
            region_prog::handle_newbie_adventure_task_reward_request
        ),
        pb::NewbieCarnivalAwardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::NewbieCarnivalAwardRequest,
            pb::NewbieCarnivalAwardResponse,
            region_prog::handle_newbie_carnival_award_request
        ),
        pb::NewbieCarnivalSwitchRoleRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::NewbieCarnivalSwitchRoleRequest,
            pb::NewbieCarnivalSwitchRoleResponse,
            region_prog::handle_newbie_carnival_switch_role_request
        ),
        pb::NewbieCourseV2RewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::NewbieCourseV2RewardRequest,
            pb::NewbieCourseV2RewardResponse,
            region_prog::handle_newbie_course_v2_reward_request
        ),
        pb::NewbieFetchScoreRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::NewbieFetchScoreRewardRequest,
            pb::NewbieFetchScoreRewardResponse,
            region_prog::handle_newbie_fetch_score_reward_request
        ),
        pb::ThroughTrainRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ThroughTrainRequest,
            pb::ThroughTrainResponse,
            region_prog::handle_through_train_request
        ),
        pb::ThroughTrainFinishViewRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ThroughTrainFinishViewRequest,
            pb::ThroughTrainFinishViewResponse,
            region_prog::handle_through_train_finish_view_request
        ),
        // coop
        pb::LobbyListRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LobbyListRequest,
            pb::LobbyListResponse,
            coop::handle_lobby_list_request
        ),
        pb::LobbyQueryPlayersRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LobbyQueryPlayersRequest,
            pb::LobbyQueryPlayersResponse,
            coop::handle_lobby_query_players_request
        ),
        pb::WorldEnterPermissionsRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::WorldEnterPermissionsRequest,
            pb::WorldEnterPermissionsResponse,
            coop::handle_world_enter_permissions_request
        ),
        pb::ApplyJoinWorldRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ApplyJoinWorldRequest,
            pb::ApplyJoinWorldResponse,
            coop::handle_apply_join_world_request
        ),
        pb::AgreeJoinResultRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AgreeJoinResultRequest,
            pb::AgreeJoinResultResponse,
            coop::handle_agree_join_result_request
        ),
        pb::LeaveWorldTeamRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LeaveWorldTeamRequest,
            pb::LeaveWorldTeamResponse,
            coop::handle_leave_world_team_request
        ),
        pb::KickWorldTeamRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::KickWorldTeamRequest,
            pb::KickWorldTeamResponse,
            coop::handle_kick_world_team_request
        ),
        pb::QueryOnlinePlayersAreaRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::QueryOnlinePlayersAreaRequest,
            pb::QueryOnlinePlayersAreaResponse,
            coop::handle_query_online_players_area_request
        ),
        pb::StartMatchRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::StartMatchRequest,
            pb::StartMatchResponse,
            coop::handle_start_match_request
        ),
        pb::CancelMatchRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::CancelMatchRequest,
            pb::CancelMatchResponse,
            coop::handle_cancel_match_request
        ),
        pb::MatchConfirmRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MatchConfirmRequest,
            pb::MatchConfirmResponse,
            coop::handle_match_confirm_request
        ),
        pb::MatchChangeReadyRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MatchChangeReadyRequest,
            pb::MatchChangeReadyResponse,
            coop::handle_match_change_ready_request
        ),
        pb::MatchChangeRoleRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MatchChangeRoleRequest,
            pb::MatchChangeRoleResponse,
            coop::handle_match_change_role_request
        ),
        pb::MatchChangePlayerUiStateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MatchChangePlayerUiStateRequest,
            pb::MatchChangePlayerUiStateResponse,
            coop::handle_match_change_player_ui_state_request
        ),
        pb::LeaveMatchTeamRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LeaveMatchTeamRequest,
            pb::LeaveMatchTeamResponse,
            coop::handle_leave_match_team_request
        ),
        pb::KickMatchTeamPlayerRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::KickMatchTeamPlayerRequest,
            pb::KickMatchTeamPlayerResponse,
            coop::handle_kick_match_team_player_request
        ),
        pb::SetMatchTeamMatchFlagRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SetMatchTeamMatchFlagRequest,
            pb::SetMatchTeamMatchFlagResponse,
            coop::handle_set_match_team_match_flag_request
        ),
        pb::EnterMatchInstRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EnterMatchInstRequest,
            pb::EnterMatchInstResponse,
            coop::handle_enter_match_inst_request
        ),
        pb::TeamChallengeRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TeamChallengeRequest,
            pb::TeamChallengeResponse,
            coop::handle_team_challenge_request
        ),
        pb::TeamMatchInviteRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TeamMatchInviteRequest,
            pb::TeamMatchInviteResponse,
            coop::handle_team_match_invite_request
        ),
        pb::TeamMatchAcceptInviteRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TeamMatchAcceptInviteRequest,
            pb::TeamMatchAcceptInviteResponse,
            coop::handle_team_match_accept_invite_request
        ),
        pb::TeamParkourRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TeamParkourRewardRequest,
            pb::TeamParkourRewardResponse,
            coop::handle_team_parkour_reward_request
        ),
        pb::CoopRoleRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::CoopRoleRewardRequest,
            pb::CoopRoleRewardResponse,
            coop::handle_coop_role_reward_request
        ),
        pb::CoopSpRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::CoopSpRewardRequest,
            pb::CoopSpRewardResponse,
            coop::handle_coop_sp_reward_request
        ),
        pb::WorldLevelDownRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::WorldLevelDownRequest,
            pb::WorldLevelDownResponse,
            coop::handle_world_level_down_request
        ),
        pb::WorldLevelRegainRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::WorldLevelRegainRequest,
            pb::WorldLevelRegainResponse,
            coop::handle_world_level_regain_request
        ),
        pb::WorldBlackboardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::WorldBlackboardRequest,
            pb::WorldBlackboardResponse,
            coop::handle_world_blackboard_request
        ),
        pb::AdviceRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AdviceRequest,
            pb::AdviceResponse,
            coop::handle_advice_request
        ),
        pb::AdviceCreateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AdviceCreateRequest,
            pb::AdviceCreateResponse,
            coop::handle_advice_create_request
        ),
        pb::AdviceDeleteRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AdviceDeleteRequest,
            pb::AdviceDeleteResponse,
            coop::handle_advice_delete_request
        ),
        pb::AdviceModifyRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AdviceModifyRequest,
            pb::AdviceModifyResponse,
            coop::handle_advice_modify_request
        ),
        pb::AdviceVoteRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AdviceVoteRequest,
            pb::AdviceVoteResponse,
            coop::handle_advice_vote_request
        ),
        // motor
        pb::MotorInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MotorInfoRequest,
            pb::MotorInfoResponse,
            motor::handle_motor_info_request
        ),
        pb::MotorDiyInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MotorDiyInfoRequest,
            pb::MotorDiyInfoResponse,
            motor::handle_motor_diy_info_request
        ),
        pb::MotorUseSceneRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MotorUseSceneRequest,
            pb::MotorUseSceneResponse,
            motor::handle_motor_use_scene_request
        ),
        pb::MotorCreateRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MotorCreateRequest,
            pb::MotorCreateResponse,
            motor::handle_motor_create_request
        ),
        pb::MotorTechLevelUpRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MotorTechLevelUpRequest,
            pb::MotorTechLevelUpResponse,
            motor::handle_motor_tech_level_up_request
        ),
        pb::MotorTechTreeSwitchRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MotorTechTreeSwitchRequest,
            pb::MotorTechTreeSwitchResponse,
            motor::handle_motor_tech_tree_switch_request
        ),
        pb::MotorLevelOneKeyRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MotorLevelOneKeyRewardRequest,
            pb::MotorLevelOneKeyRewardResponse,
            motor::handle_motor_level_one_key_reward_request
        ),
        pb::MotorTaskOneKeyRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MotorTaskOneKeyRewardRequest,
            pb::MotorTaskOneKeyRewardResponse,
            motor::handle_motor_task_one_key_reward_request
        ),
        pb::MotorUseSkinRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MotorUseSkinRequest,
            pb::MotorUseSkinResponse,
            motor::handle_motor_use_skin_request
        ),
        pb::MotorChangeOutlookRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MotorChangeOutlookRequest,
            pb::MotorChangeOutlookResponse,
            motor::handle_motor_change_outlook_request
        ),
        pb::MotorOutlookCreatePresetRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MotorOutlookCreatePresetRequest,
            pb::MotorOutlookCreatePresetResponse,
            motor::handle_motor_outlook_create_preset_request
        ),
        pb::MotorOutlookDeletePresetRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MotorOutlookDeletePresetRequest,
            pb::MotorOutlookDeletePresetResponse,
            motor::handle_motor_outlook_delete_preset_request
        ),
        pb::MotorOutlookEditPresetRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MotorOutlookEditPresetRequest,
            pb::MotorOutlookEditPresetResponse,
            motor::handle_motor_outlook_edit_preset_request
        ),
        pb::MotorAnimReplaySampleUdpPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::MotorAnimReplaySampleUdpPush,
            motor::handle_motor_anim_replay_sample_udp_push
        ),
        pb::MotorCycleCruiseStatePush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::MotorCycleCruiseStatePush,
            motor::handle_motor_cycle_cruise_state_push
        ),
        pb::MotorIsEnablePush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::MotorIsEnablePush,
            motor::handle_motor_is_enable_push
        ),
        pb::MotorOnlineProgressReportPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::MotorOnlineProgressReportPush,
            motor::handle_motor_online_progress_report_push
        ),
        pb::MotorPositionStatePush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::MotorPositionStatePush,
            motor::handle_motor_position_state_push
        ),
        pb::MotorSummonAndRidePush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::MotorSummonAndRidePush,
            motor::handle_motor_summon_and_ride_push
        ),
        pb::MotorFightRefreshBossPush::MSG_ID => push_ev!(
            state,
            msg_id,
            payload,
            pb::MotorFightRefreshBossPush,
            motor::handle_motor_fight_refresh_boss_push
        ),
        pb::MotorDecalMotorIpRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MotorDecalMotorIpRequest,
            pb::MotorDecalMotorIpResponse,
            motor::handle_motor_decal_motor_ip_request
        ),
        pb::MotorDecalTaskRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MotorDecalTaskRequest,
            pb::MotorDecalTaskResponse,
            motor::handle_motor_decal_task_request
        ),
        pb::MotorDevelopTaskRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MotorDevelopTaskRewardRequest,
            pb::MotorDevelopTaskRewardResponse,
            motor::handle_motor_develop_task_reward_request
        ),
        pb::MotorLinkTaskRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MotorLinkTaskRequest,
            pb::MotorLinkTaskResponse,
            motor::handle_motor_link_task_request
        ),
        pb::MotorParkourLevelRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MotorParkourLevelRewardRequest,
            pb::MotorParkourLevelRewardResponse,
            motor::handle_motor_parkour_level_reward_request
        ),
        pb::MotorSliderRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MotorSliderRequest,
            pb::MotorSliderResponse,
            motor::handle_motor_slider_request
        ),
        pb::MotorFightEnterSubLevelRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MotorFightEnterSubLevelRequest,
            pb::MotorFightEnterSubLevelResponse,
            motor::handle_motor_fight_enter_sub_level_request
        ),
        pb::MotorFightKillRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MotorFightKillRequest,
            pb::MotorFightKillResponse,
            motor::handle_motor_fight_kill_request
        ),
        pb::MotorFightLastSaveDataRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MotorFightLastSaveDataRequest,
            pb::MotorFightLastSaveDataResponse,
            motor::handle_motor_fight_last_save_data_request
        ),
        pb::MotorFightOneKeyRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MotorFightOneKeyRewardRequest,
            pb::MotorFightOneKeyRewardResponse,
            motor::handle_motor_fight_one_key_reward_request
        ),
        pb::MotorFightPlayerDeadRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MotorFightPlayerDeadRequest,
            pb::MotorFightPlayerDeadResponse,
            motor::handle_motor_fight_player_dead_request
        ),
        pb::MotorFightRankListRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MotorFightRankListRequest,
            pb::MotorFightRankListResponse,
            motor::handle_motor_fight_rank_list_request
        ),
        pb::MotorFightRankListSelfDataRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MotorFightRankListSelfDataRequest,
            pb::MotorFightRankListSelfDataResponse,
            motor::handle_motor_fight_rank_list_self_data_request
        ),
        pb::MotorFightRefreshCollectionRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MotorFightRefreshCollectionRequest,
            pb::MotorFightRefreshCollectionResponse,
            motor::handle_motor_fight_refresh_collection_request
        ),
        pb::MotorFightRefreshNextWaveGroupRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MotorFightRefreshNextWaveGroupRequest,
            pb::MotorFightRefreshNextWaveGroupResponse,
            motor::handle_motor_fight_refresh_next_wave_group_request
        ),
        pb::MotorFightSelectBuffGateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MotorFightSelectBuffGateRequest,
            pb::MotorFightSelectBuffGateResponse,
            motor::handle_motor_fight_select_buff_gate_request
        ),
        pb::MotorFightSelectCollectionRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MotorFightSelectCollectionRequest,
            pb::MotorFightSelectCollectionResponse,
            motor::handle_motor_fight_select_collection_request
        ),
        pb::MotorFightSettleRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MotorFightSettleRequest,
            pb::MotorFightSettleResponse,
            motor::handle_motor_fight_settle_request
        ),
        pb::MotorFightTalentUpRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MotorFightTalentUpRequest,
            pb::MotorFightTalentUpResponse,
            motor::handle_motor_fight_talent_up_request
        ),
        // gacha
        pb::GachaInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::GachaInfoRequest,
            pb::GachaInfoResponse,
            gacha::handle_gacha_info_request
        ),
        pb::GachaUsePoolRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::GachaUsePoolRequest,
            pb::GachaUsePoolResponse,
            gacha::handle_gacha_use_pool_request
        ),
        pb::GachaPoolDetailRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::GachaPoolDetailRequest,
            pb::GachaPoolDetailResponse,
            gacha::handle_gacha_pool_detail_request
        ),
        pb::GachaRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::GachaRequest,
            pb::GachaResponse,
            gacha::handle_gacha_request
        ),
        // fishing
        pb::FishingTechLevelUpRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FishingTechLevelUpRequest,
            pb::FishingTechLevelUpResponse,
            fishing::handle_fishing_tech_level_up_request
        ),
        pb::FishingCabinPutRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FishingCabinPutRequest,
            pb::FishingCabinPutResponse,
            fishing::handle_fishing_cabin_put_request
        ),
        pb::FishingSellRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FishingSellRequest,
            pb::FishingSellResponse,
            fishing::handle_fishing_sell_request
        ),
        pb::FishingQuickSellRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FishingQuickSellRequest,
            pb::FishingQuickSellResponse,
            fishing::handle_fishing_quick_sell_request
        ),
        pb::FishingInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FishingInfoRequest,
            pb::FishingInfoResponse,
            fishing::handle_fishing_info_request
        ),
        pb::FishingActivityLimitRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FishingActivityLimitRewardRequest,
            pb::FishingActivityLimitRewardResponse,
            fishing::handle_fishing_activity_limit_reward_request
        ),
        pb::FishingActivityMilestoneRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FishingActivityMilestoneRewardRequest,
            pb::FishingActivityMilestoneRewardResponse,
            fishing::handle_fishing_activity_milestone_reward_request
        ),
        pb::FishingIllustratedRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FishingIllustratedRewardRequest,
            pb::FishingIllustratedRewardResponse,
            fishing::handle_fishing_illustrated_reward_request
        ),
        pb::FishingCompleteRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FishingCompleteRequest,
            pb::FishingCompleteResponse,
            fishing::handle_fishing_complete_request
        ),
        pb::FishingEntrustAcceptRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FishingEntrustAcceptRequest,
            pb::FishingEntrustAcceptResponse,
            fishing::handle_fishing_entrust_accept_request
        ),
        pb::FishingEntrustHandInRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FishingEntrustHandInRequest,
            pb::FishingEntrustHandInResponse,
            fishing::handle_fishing_entrust_hand_in_request
        ),
        pb::FishingEntrustRefreshRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FishingEntrustRefreshRequest,
            pb::FishingEntrustRefreshResponse,
            fishing::handle_fishing_entrust_refresh_request
        ),
        pb::FishingEntrustTraceRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FishingEntrustTraceRequest,
            pb::FishingEntrustTraceResponse,
            fishing::handle_fishing_entrust_trace_request
        ),
        pb::FishingExitRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FishingExitRequest,
            pb::FishingExitResponse,
            fishing::handle_fishing_exit_request
        ),
        pb::FishingHandInRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FishingHandInRequest,
            pb::FishingHandInResponse,
            fishing::handle_fishing_hand_in_request
        ),
        pb::FishingPointInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FishingPointInfoRequest,
            pb::FishingPointInfoResponse,
            fishing::handle_fishing_point_info_request
        ),
        pb::FishingCageInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FishingCageInfoRequest,
            pb::FishingCageInfoResponse,
            fishing::handle_fishing_cage_info_request
        ),
        pb::FishingSailingRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FishingSailingRequest,
            pb::FishingSailingResponse,
            fishing::handle_fishing_sailing_request
        ),
        pb::FishingShipFixRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FishingShipFixRequest,
            pb::FishingShipFixResponse,
            fishing::handle_fishing_ship_fix_request
        ),
        pb::FishingShipSkinChangeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FishingShipSkinChangeRequest,
            pb::FishingShipSkinChangeResponse,
            fishing::handle_fishing_ship_skin_change_request
        ),
        pb::FishingSkillBaitRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FishingSkillBaitRequest,
            pb::FishingSkillBaitResponse,
            fishing::handle_fishing_skill_bait_request
        ),
        pb::FishingSkillBombRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FishingSkillBombRequest,
            pb::FishingSkillBombResponse,
            fishing::handle_fishing_skill_bomb_request
        ),
        pb::FishingSkillTeleportRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FishingSkillTeleportRequest,
            pb::FishingSkillTeleportResponse,
            fishing::handle_fishing_skill_teleport_request
        ),
        pb::FishingTeleportToBoatRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FishingTeleportToBoatRequest,
            pb::FishingTeleportToBoatResponse,
            fishing::handle_fishing_teleport_to_boat_request
        ),
        pb::FishingQuickSellPreviewRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FishingQuickSellPreviewRequest,
            pb::FishingQuickSellPreviewResponse,
            fishing::handle_fishing_quick_sell_preview_request
        ),
        pb::FishingSellPreviewRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FishingSellPreviewRequest,
            pb::FishingSellPreviewResponse,
            fishing::handle_fishing_sell_preview_request
        ),
        // phantom_battle
        pb::PhantomBattleCardGroupAddRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleCardGroupAddRequest,
            pb::PhantomBattleCardGroupAddResponse,
            phantom_battle::handle_phantom_battle_card_group_add_request
        ),
        pb::PhantomBattleCardGroupUpdateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleCardGroupUpdateRequest,
            pb::PhantomBattleCardGroupUpdateResponse,
            phantom_battle::handle_phantom_battle_card_group_update_request
        ),
        pb::PhantomBattleCardGroupNameRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleCardGroupNameRequest,
            pb::PhantomBattleCardGroupNameResponse,
            phantom_battle::handle_phantom_battle_card_group_name_request
        ),
        pb::PhantomBattleCardGroupDeleteRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleCardGroupDeleteRequest,
            pb::PhantomBattleCardGroupDeleteResponse,
            phantom_battle::handle_phantom_battle_card_group_delete_request
        ),
        pb::PhantomBattleCardUnlockRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleCardUnlockRequest,
            pb::PhantomBattleCardUnlockResponse,
            phantom_battle::handle_phantom_battle_card_unlock_request
        ),
        pb::PhantomBattleCardOutLookUpRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleCardOutLookUpRequest,
            pb::PhantomBattleCardOutLookUpResponse,
            phantom_battle::handle_phantom_battle_card_out_look_up_request
        ),
        pb::PhantomBattleCheckCardSkillUnlockRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleCheckCardSkillUnlockRequest,
            pb::PhantomBattleCheckCardSkillUnlockResponse,
            phantom_battle::handle_phantom_battle_check_card_skill_unlock_request
        ),
        pb::PhantomBattleStartRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleStartRequest,
            pb::PhantomBattleStartResponse,
            phantom_battle::handle_phantom_battle_start_request
        ),
        pb::PhantomBattleBvBLoadingFinishRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleBvBLoadingFinishRequest,
            pb::PhantomBattleBvBLoadingFinishResponse,
            phantom_battle::handle_phantom_battle_bvb_loading_finish_request
        ),
        pb::PhantomBattleNpcShowOverRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleNpcShowOverRequest,
            pb::PhantomBattleNpcShowOverResponse,
            phantom_battle::handle_phantom_battle_npc_show_over_request
        ),
        pb::PhantomBattleReChallengeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleReChallengeRequest,
            pb::PhantomBattleReChallengeResponse,
            phantom_battle::handle_phantom_battle_re_challenge_request
        ),
        pb::PhantomBattleRoundOverRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleRoundOverRequest,
            pb::PhantomBattleRoundOverResponse,
            phantom_battle::handle_phantom_battle_round_over_request
        ),
        pb::PhantomBattleEnterSlotRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleEnterSlotRequest,
            pb::PhantomBattleEnterSlotResponse,
            phantom_battle::handle_phantom_battle_enter_slot_request
        ),
        pb::PhantomBattleLeaveSlotRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleLeaveSlotRequest,
            pb::PhantomBattleLeaveSlotResponse,
            phantom_battle::handle_phantom_battle_leave_slot_request
        ),
        pb::PhantomBattleSlotInsteadRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleSlotInsteadRequest,
            pb::PhantomBattleSlotInsteadResponse,
            phantom_battle::handle_phantom_battle_slot_instead_request
        ),
        pb::PhantomBattleEvolveRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleEvolveRequest,
            pb::PhantomBattleEvolveResponse,
            phantom_battle::handle_phantom_battle_evolve_request
        ),
        pb::PhantomBattleBackLibraryRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleBackLibraryRequest,
            pb::PhantomBattleBackLibraryResponse,
            phantom_battle::handle_phantom_battle_back_library_request
        ),
        pb::PhantomBattleBackSlotCardLibraryRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleBackSlotCardLibraryRequest,
            pb::PhantomBattleBackSlotCardLibraryResponse,
            phantom_battle::handle_phantom_battle_back_slot_card_library_request
        ),
        pb::PhantomBattleCallCardRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleCallCardRequest,
            pb::PhantomBattleCallCardResponse,
            phantom_battle::handle_phantom_battle_call_card_request
        ),
        pb::PhantomBattleReconstructCardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleReconstructCardRequest,
            pb::PhantomBattleReconstructCardResponse,
            phantom_battle::handle_phantom_battle_reconstruct_card_request
        ),
        pb::PhantomBattleCardSelectRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleCardSelectRequest,
            pb::PhantomBattleCardSelectResponse,
            phantom_battle::handle_phantom_battle_card_select_request
        ),
        pb::PhantomBattleDealCardReplaceRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleDealCardReplaceRequest,
            pb::PhantomBattleDealCardReplaceResponse,
            phantom_battle::handle_phantom_battle_deal_card_replace_request
        ),
        pb::PhantomBattleDiscardCardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleDiscardCardRequest,
            pb::PhantomBattleDiscardCardResponse,
            phantom_battle::handle_phantom_battle_discard_card_request
        ),
        pb::PhantomBattleSelectReserveCardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleSelectReserveCardRequest,
            pb::PhantomBattleSelectReserveCardResponse,
            phantom_battle::handle_phantom_battle_select_reserve_card_request
        ),
        pb::PhantomBattleCardSkillRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleCardSkillRequest,
            pb::PhantomBattleCardSkillResponse,
            phantom_battle::handle_phantom_battle_card_skill_request
        ),
        pb::PhantomBattleSlotCardSkillRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleSlotCardSkillRequest,
            pb::PhantomBattleSlotCardSkillResponse,
            phantom_battle::handle_phantom_battle_slot_card_skill_request
        ),
        pb::PhantomBattleCardRoleSkillRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleCardRoleSkillRequest,
            pb::PhantomBattleCardRoleSkillResponse,
            phantom_battle::handle_phantom_battle_card_role_skill_request
        ),
        pb::PhantomBattleCardTargetInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleCardTargetInfoRequest,
            pb::PhantomBattleCardTargetInfoResponse,
            phantom_battle::handle_phantom_battle_card_target_info_request
        ),
        pb::PhantomBattleCardRoleTargetInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleCardRoleTargetInfoRequest,
            pb::PhantomBattleCardRoleTargetInfoResponse,
            phantom_battle::handle_phantom_battle_card_role_target_info_request
        ),
        pb::PhantomBattleSelectTargetRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleSelectTargetRequest,
            pb::PhantomBattleSelectTargetResponse,
            phantom_battle::handle_phantom_battle_select_target_request
        ),
        pb::PhantomBattleBadgeRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleBadgeRewardRequest,
            pb::PhantomBattleBadgeRewardResponse,
            phantom_battle::handle_phantom_battle_badge_reward_request
        ),
        pb::PhantomBattleCardRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleCardRewardRequest,
            pb::PhantomBattleCardRewardResponse,
            phantom_battle::handle_phantom_battle_card_reward_request
        ),
        pb::PhantomBattleGuideRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleGuideRewardRequest,
            pb::PhantomBattleGuideRewardResponse,
            phantom_battle::handle_phantom_battle_guide_reward_request
        ),
        pb::PhantomBattleTaskRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleTaskRewardRequest,
            pb::PhantomBattleTaskRewardResponse,
            phantom_battle::handle_phantom_battle_task_reward_request
        ),
        pb::PhantomBattleRepeatedTaskRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleRepeatedTaskRewardRequest,
            pb::PhantomBattleRepeatedTaskRewardResponse,
            phantom_battle::handle_phantom_battle_repeated_task_reward_request
        ),
        pb::PhantomBattleRoleRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomBattleRoleRewardRequest,
            pb::PhantomBattleRoleRewardResponse,
            phantom_battle::handle_phantom_battle_role_reward_request
        ),
        pb::MulPhantomBattleMasterLevelRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MulPhantomBattleMasterLevelRewardRequest,
            pb::MulPhantomBattleMasterLevelRewardResponse,
            phantom_battle::handle_mul_phantom_battle_master_level_reward_request
        ),
        // roguelike
        pb::RoguelikeStartRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoguelikeStartRequest,
            pb::RoguelikeStartResponse,
            roguelike::handle_roguelike_start_request
        ),
        pb::RoguelikeLastInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoguelikeLastInfoRequest,
            pb::RoguelikeLastInfoResponse,
            roguelike::handle_roguelike_last_info_request
        ),
        pb::RoguelikeChooseDataRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoguelikeChooseDataRequest,
            pb::RoguelikeChooseDataResponse,
            roguelike::handle_roguelike_choose_data_request
        ),
        pb::RoguelikeChooseDataResultRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoguelikeChooseDataResultRequest,
            pb::RoguelikeChooseDataResultResponse,
            roguelike::handle_roguelike_choose_data_result_request
        ),
        pb::RoguelikeRefreshGainRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoguelikeRefreshGainRequest,
            pb::RoguelikeRefreshGainResponse,
            roguelike::handle_roguelike_refresh_gain_request
        ),
        pb::RoguelikeGiveUpGainRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoguelikeGiveUpGainRequest,
            pb::RoguelikeGiveUpGainResponse,
            roguelike::handle_roguelike_give_up_gain_request
        ),
        pb::RoguelikeGotoNextRoomRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoguelikeGotoNextRoomRequest,
            pb::RoguelikeGotoNextRoomResponse,
            roguelike::handle_roguelike_goto_next_room_request
        ),
        pb::RoguelikeQuitRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoguelikeQuitRequest,
            pb::RoguelikeQuitResponse,
            roguelike::handle_roguelike_quit_request
        ),
        pb::RoguelikeResultRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoguelikeResultRequest,
            pb::RoguelikeResultResponse,
            roguelike::handle_roguelike_result_request
        ),
        pb::RoguelikePopularEntriesInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoguelikePopularEntriesInfoRequest,
            pb::RoguelikePopularEntriesInfoResponse,
            roguelike::handle_roguelike_popular_entries_info_request
        ),
        pb::RoguelikePopularEntriesChangeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoguelikePopularEntriesChangeRequest,
            pb::RoguelikePopularEntriesChangeResponse,
            roguelike::handle_roguelike_popular_entries_change_request
        ),
        pb::RoguelikeSeasonRewardReceiveRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoguelikeSeasonRewardReceiveRequest,
            pb::RoguelikeSeasonRewardReceiveResponse,
            roguelike::handle_roguelike_season_reward_receive_request
        ),
        pb::RoguelikeTokenReceiveRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoguelikeTokenReceiveRequest,
            pb::RoguelikeTokenReceiveResponse,
            roguelike::handle_roguelike_token_receive_request
        ),
        pb::RoguelikeTalentInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoguelikeTalentInfoRequest,
            pb::RoguelikeTalentInfoResponse,
            roguelike::handle_roguelike_talent_info_request
        ),
        pb::RoguelikeTalentLevelUpRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoguelikeTalentLevelUpRequest,
            pb::RoguelikeTalentLevelUpResponse,
            roguelike::handle_roguelike_talent_level_up_request
        ),
        pb::RoguelikeTrialRoleInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoguelikeTrialRoleInfoRequest,
            pb::RoguelikeTrialRoleInfoResponse,
            roguelike::handle_roguelike_trial_role_info_request
        ),
        pb::RoguelikeRoleInstStartRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoguelikeRoleInstStartRequest,
            pb::RoguelikeRoleInstStartResponse,
            roguelike::handle_roguelike_role_inst_start_request
        ),
        pb::RoguelikeRoleRoomSelectRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoguelikeRoleRoomSelectRequest,
            pb::RoguelikeRoleRoomSelectResponse,
            roguelike::handle_roguelike_role_room_select_request
        ),
        pb::RogueInstExtraInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RogueInstExtraInfoRequest,
            pb::RogueInstExtraInfoResponse,
            roguelike::handle_rogue_inst_extra_info_request
        ),
        pb::RogueArchiveSaveRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RogueArchiveSaveRequest,
            pb::RogueArchiveSaveResponse,
            roguelike::handle_rogue_archive_save_request
        ),
        pb::RogueArchiveListRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RogueArchiveListRequest,
            pb::RogueArchiveListResponse,
            roguelike::handle_rogue_archive_list_request
        ),
        pb::RogueBlackFlowerRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RogueBlackFlowerRequest,
            pb::RogueBlackFlowerResponse,
            roguelike::handle_rogue_black_flower_request
        ),
        pb::RogueResInstStartRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RogueResInstStartRequest,
            pb::RogueResInstStartResponse,
            roguelike::handle_rogue_res_inst_start_request
        ),
        pb::RogueResLastInstInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RogueResLastInstInfoRequest,
            pb::RogueResLastInstInfoResponse,
            roguelike::handle_rogue_res_last_inst_info_request
        ),
        pb::RogueResGotoNextRoomRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RogueResGotoNextRoomRequest,
            pb::RogueResGotoNextRoomResponse,
            roguelike::handle_rogue_res_goto_next_room_request
        ),
        pb::RogueResMoveRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RogueResMoveRequest,
            pb::RogueResMoveResponse,
            roguelike::handle_rogue_res_move_request
        ),
        pb::RogueResGridEventRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RogueResGridEventRequest,
            pb::RogueResGridEventResponse,
            roguelike::handle_rogue_res_grid_event_request
        ),
        pb::RogueResOpDoneRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RogueResOpDoneRequest,
            pb::RogueResOpDoneResponse,
            roguelike::handle_rogue_res_op_done_request
        ),
        pb::RogueResChangeFormationRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RogueResChangeFormationRequest,
            pb::RogueResChangeFormationResponse,
            roguelike::handle_rogue_res_change_formation_request
        ),
        pb::RogueResSwitchFormationRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RogueResSwitchFormationRequest,
            pb::RogueResSwitchFormationResponse,
            roguelike::handle_rogue_res_switch_formation_request
        ),
        pb::RogueResTalentSkillLevelRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RogueResTalentSkillLevelRequest,
            pb::RogueResTalentSkillLevelResponse,
            roguelike::handle_rogue_res_talent_skill_level_request
        ),
        pb::RogueResInstResultRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RogueResInstResultRequest,
            pb::RogueResInstResultResponse,
            roguelike::handle_rogue_res_inst_result_request
        ),
        pb::RogueResEndingRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RogueResEndingRewardRequest,
            pb::RogueResEndingRewardResponse,
            roguelike::handle_rogue_res_ending_reward_request
        ),
        pb::RogueResIllustrationAwardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RogueResIllustrationAwardRequest,
            pb::RogueResIllustrationAwardResponse,
            roguelike::handle_rogue_res_illustration_award_request
        ),
        pb::RogueResTaskAwardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RogueResTaskAwardRequest,
            pb::RogueResTaskAwardResponse,
            roguelike::handle_rogue_res_task_award_request
        ),
        pb::RogueTowerTrialStartRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RogueTowerTrialStartRequest,
            pb::RogueTowerTrialStartResponse,
            roguelike::handle_rogue_tower_trial_start_request
        ),
        pb::RogueTowerTrialSelectBossRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RogueTowerTrialSelectBossRequest,
            pb::RogueTowerTrialSelectBossResponse,
            roguelike::handle_rogue_tower_trial_select_boss_request
        ),
        pb::RogueTowerTrialGiveUpTempArchiveRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RogueTowerTrialGiveUpTempArchiveRequest,
            pb::RogueTowerTrialGiveUpTempArchiveResponse,
            roguelike::handle_rogue_tower_trial_give_up_temp_archive_request
        ),
        pb::RogueTowerTrialResultRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RogueTowerTrialResultRequest,
            pb::RogueTowerTrialResultResponse,
            roguelike::handle_rogue_tower_trial_result_request
        ),
        pb::RogueWeeklyStartRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RogueWeeklyStartRequest,
            pb::RogueWeeklyStartResponse,
            roguelike::handle_rogue_weekly_start_request
        ),
        pb::RogueWeeklyLastInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RogueWeeklyLastInfoRequest,
            pb::RogueWeeklyLastInfoResponse,
            roguelike::handle_rogue_weekly_last_info_request
        ),
        pb::RogueWeeklyArtifactInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RogueWeeklyArtifactInfoRequest,
            pb::RogueWeeklyArtifactInfoResponse,
            roguelike::handle_rogue_weekly_artifact_info_request
        ),
        pb::RogueWeeklyArtifactSelectRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RogueWeeklyArtifactSelectRequest,
            pb::RogueWeeklyArtifactSelectResponse,
            roguelike::handle_rogue_weekly_artifact_select_request
        ),
        pb::RogueWeeklyOptionSelectRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RogueWeeklyOptionSelectRequest,
            pb::RogueWeeklyOptionSelectResponse,
            roguelike::handle_rogue_weekly_option_select_request
        ),
        pb::RogueWeeklyGotoNextRoomRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RogueWeeklyGotoNextRoomRequest,
            pb::RogueWeeklyGotoNextRoomResponse,
            roguelike::handle_rogue_weekly_goto_next_room_request
        ),
        pb::RogueWeeklyBlackFlowerRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RogueWeeklyBlackFlowerRequest,
            pb::RogueWeeklyBlackFlowerResponse,
            roguelike::handle_rogue_weekly_black_flower_request
        ),
        pb::RogueWeeklyResultRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RogueWeeklyResultRequest,
            pb::RogueWeeklyResultResponse,
            roguelike::handle_rogue_weekly_result_request
        ),
        pb::MulRogueWeeklyRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MulRogueWeeklyRewardRequest,
            pb::MulRogueWeeklyRewardResponse,
            roguelike::handle_mul_rogue_weekly_reward_request
        ),
        pb::RogueWhiteCatLimitedRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RogueWhiteCatLimitedRewardRequest,
            pb::RogueWhiteCatLimitedRewardResponse,
            roguelike::handle_rogue_white_cat_limited_reward_request
        ),
        pb::RogueWhiteCatRecordRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RogueWhiteCatRecordRequest,
            pb::RogueWhiteCatRecordResponse,
            roguelike::handle_rogue_white_cat_record_request
        ),
        pb::MulRoguelikeWriteCatRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MulRoguelikeWriteCatRewardRequest,
            pb::MulRoguelikeWriteCatRewardResponse,
            roguelike::handle_mul_roguelike_write_cat_reward_request
        ),
        pb::RoguelikeWriteCatBossRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoguelikeWriteCatBossRewardRequest,
            pb::RoguelikeWriteCatBossRewardResponse,
            roguelike::handle_roguelike_write_cat_boss_reward_request
        ),
        pb::RoguelikeWriteCatLevelPlayRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoguelikeWriteCatLevelPlayRewardRequest,
            pb::RoguelikeWriteCatLevelPlayRewardResponse,
            roguelike::handle_roguelike_write_cat_level_play_reward_request
        ),
        pb::RoverRogueInsListRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoverRogueInsListRequest,
            pb::RoverRogueInsListResponse,
            roguelike::handle_rover_rogue_ins_list_request
        ),
        pb::RoverRogueBlessGroupSelectRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoverRogueBlessGroupSelectRequest,
            pb::RoverRogueBlessGroupSelectResponse,
            roguelike::handle_rover_rogue_bless_group_select_request
        ),
        pb::RoverRogueChooseDataResultRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoverRogueChooseDataResultRequest,
            pb::RoverRogueChooseDataResultResponse,
            roguelike::handle_rover_rogue_choose_data_result_request
        ),
        pb::RoverRogueEventSelectRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoverRogueEventSelectRequest,
            pb::RoverRogueEventSelectResponse,
            roguelike::handle_rover_rogue_event_select_request
        ),
        pb::RoverRogueGiveUpGainRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoverRogueGiveUpGainRequest,
            pb::RoverRogueGiveUpGainResponse,
            roguelike::handle_rover_rogue_give_up_gain_request
        ),
        pb::RoverRogueGotoNextRoomCommitRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoverRogueGotoNextRoomCommitRequest,
            pb::RoverRogueGotoNextRoomCommitResponse,
            roguelike::handle_rover_rogue_goto_next_room_commit_request
        ),
        pb::RoverRogueLootInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoverRogueLootInfoRequest,
            pb::RoverRogueLootInfoResponse,
            roguelike::handle_rover_rogue_loot_info_request
        ),
        pb::RoverRogueLootChangeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoverRogueLootChangeRequest,
            pb::RoverRogueLootChangeResponse,
            roguelike::handle_rover_rogue_loot_change_request
        ),
        pb::RoverRogueRefreshGainRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoverRogueRefreshGainRequest,
            pb::RoverRogueRefreshGainResponse,
            roguelike::handle_rover_rogue_refresh_gain_request
        ),
        pb::RoverRogueShopRefreshRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoverRogueShopRefreshRequest,
            pb::RoverRogueShopRefreshResponse,
            roguelike::handle_rover_rogue_shop_refresh_request
        ),
        pb::RoverRogueShopBuyRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoverRogueShopBuyRequest,
            pb::RoverRogueShopBuyResponse,
            roguelike::handle_rover_rogue_shop_buy_request
        ),
        pb::RoverRogueTalentUnlockRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoverRogueTalentUnlockRequest,
            pb::RoverRogueTalentUnlockResponse,
            roguelike::handle_rover_rogue_talent_unlock_request
        ),
        pb::RoverRogueTaskRewardTakeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoverRogueTaskRewardTakeRequest,
            pb::RoverRogueTaskRewardTakeResponse,
            roguelike::handle_rover_rogue_task_reward_take_request
        ),
        pb::RoverRogueResultRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoverRogueResultRequest,
            pb::RoverRogueResultResponse,
            roguelike::handle_rover_rogue_result_request
        ),
        pb::RoverRogueQuitRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoverRogueQuitRequest,
            pb::RoverRogueQuitResponse,
            roguelike::handle_rover_rogue_quit_request
        ),
        pb::SlashAndTowerInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SlashAndTowerInfoRequest,
            pb::SlashAndTowerInfoResponse,
            endgame::handle_slash_and_tower_info_request
        ),
        pb::SlashAndTowerRecommendRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SlashAndTowerRecommendRequest,
            pb::SlashAndTowerRecommendResponse,
            endgame::handle_slash_and_tower_recommend_request
        ),
        pb::SlashAndTowerResetRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SlashAndTowerResetRequest,
            pb::SlashAndTowerResetResponse,
            endgame::handle_slash_and_tower_reset_request
        ),
        pb::SlashAndTowerSaveRecordRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SlashAndTowerSaveRecordRequest,
            pb::SlashAndTowerSaveRecordResponse,
            endgame::handle_slash_and_tower_save_record_request
        ),
        pb::SlashAndTowerScoreRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SlashAndTowerScoreRewardRequest,
            pb::SlashAndTowerScoreRewardResponse,
            endgame::handle_slash_and_tower_score_reward_request
        ),
        pb::NewTowerClearRecordRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::NewTowerClearRecordRequest,
            pb::NewTowerClearRecordResponse,
            endgame::handle_new_tower_clear_record_request
        ),
        pb::NewTowerCycleStartRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::NewTowerCycleStartRequest,
            pb::NewTowerCycleStartResponse,
            endgame::handle_new_tower_cycle_start_request
        ),
        pb::NewTowerLastCycleInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::NewTowerLastCycleInfoRequest,
            pb::NewTowerLastCycleInfoResponse,
            endgame::handle_new_tower_last_cycle_info_request
        ),
        pb::NewTowerMedalInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::NewTowerMedalInfoRequest,
            pb::NewTowerMedalInfoResponse,
            endgame::handle_new_tower_medal_info_request
        ),
        pb::NewTowerRecommendRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::NewTowerRecommendRequest,
            pb::NewTowerRecommendResponse,
            endgame::handle_new_tower_recommend_request
        ),
        pb::NewTowerRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::NewTowerRewardRequest,
            pb::NewTowerRewardResponse,
            endgame::handle_new_tower_reward_request
        ),
        pb::NewTowerRoleEnergyRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::NewTowerRoleEnergyRequest,
            pb::NewTowerRoleEnergyResponse,
            endgame::handle_new_tower_role_energy_request
        ),
        pb::NewTowerSeasonRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::NewTowerSeasonRewardRequest,
            pb::NewTowerSeasonRewardResponse,
            endgame::handle_new_tower_season_reward_request
        ),
        pb::NewTowerSeasonScoreRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::NewTowerSeasonScoreRewardRequest,
            pb::NewTowerSeasonScoreRewardResponse,
            endgame::handle_new_tower_season_score_reward_request
        ),
        pb::NewTowerSeasonTaskDataUpdateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::NewTowerSeasonTaskDataUpdateRequest,
            pb::NewTowerSeasonTaskDataUpdateResponse,
            endgame::handle_new_tower_season_task_data_update_request
        ),
        pb::NewTowerUpdateRecordRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::NewTowerUpdateRecordRequest,
            pb::NewTowerUpdateRecordResponse,
            endgame::handle_new_tower_update_record_request
        ),
        pb::RoleTowerAttributeQueryRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleTowerAttributeQueryRequest,
            pb::RoleTowerAttributeQueryResponse,
            endgame::handle_role_tower_attribute_query_request
        ),
        pb::RoleTowerChallengeSettleRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleTowerChallengeSettleRequest,
            pb::RoleTowerChallengeSettleResponse,
            endgame::handle_role_tower_challenge_settle_request
        ),
        pb::RoleTowerConsumeItemRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleTowerConsumeItemRequest,
            pb::RoleTowerConsumeItemResponse,
            endgame::handle_role_tower_consume_item_request
        ),
        pb::RoleTowerPhantomAddAttrsRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleTowerPhantomAddAttrsRequest,
            pb::RoleTowerPhantomAddAttrsResponse,
            endgame::handle_role_tower_phantom_add_attrs_request
        ),
        pb::RoleTowerRankRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleTowerRankRequest,
            pb::RoleTowerRankResponse,
            endgame::handle_role_tower_rank_request
        ),
        pb::RoleTowerRankRewardReceiveRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleTowerRankRewardReceiveRequest,
            pb::RoleTowerRankRewardReceiveResponse,
            endgame::handle_role_tower_rank_reward_receive_request
        ),
        pb::RoleTowerRewardTaskReceiveRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleTowerRewardTaskReceiveRequest,
            pb::RoleTowerRewardTaskReceiveResponse,
            endgame::handle_role_tower_reward_task_receive_request
        ),
        pb::RoleTowerSaveDeleteRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleTowerSaveDeleteRequest,
            pb::RoleTowerSaveDeleteResponse,
            endgame::handle_role_tower_save_delete_request
        ),
        pb::RoleTowerScoreRewardReceiveRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleTowerScoreRewardReceiveRequest,
            pb::RoleTowerScoreRewardReceiveResponse,
            endgame::handle_role_tower_score_reward_receive_request
        ),
        pb::RoleTowerStageRetryRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleTowerStageRetryRequest,
            pb::RoleTowerStageRetryResponse,
            endgame::handle_role_tower_stage_retry_request
        ),
        pb::RoleTowerStageSettleRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleTowerStageSettleRequest,
            pb::RoleTowerStageSettleResponse,
            endgame::handle_role_tower_stage_settle_request
        ),
        pb::RoleTowerStepAdvanceRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleTowerStepAdvanceRequest,
            pb::RoleTowerStepAdvanceResponse,
            endgame::handle_role_tower_step_advance_request
        ),
        pb::AbyssChallengeGotoNextRoomRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AbyssChallengeGotoNextRoomRequest,
            pb::AbyssChallengeGotoNextRoomResponse,
            endgame::handle_abyss_challenge_goto_next_room_request
        ),
        pb::AbyssChallengeQuitRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AbyssChallengeQuitRequest,
            pb::AbyssChallengeQuitResponse,
            endgame::handle_abyss_challenge_quit_request
        ),
        pb::AbyssChallengeUnlockRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AbyssChallengeUnlockRequest,
            pb::AbyssChallengeUnlockResponse,
            endgame::handle_abyss_challenge_unlock_request
        ),
        pb::AbyssLikeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AbyssLikeRequest,
            pb::AbyssLikeResponse,
            endgame::handle_abyss_like_request
        ),
        pb::AbyssLittleRoleLevelUpRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AbyssLittleRoleLevelUpRequest,
            pb::AbyssLittleRoleLevelUpResponse,
            endgame::handle_abyss_little_role_level_up_request
        ),
        pb::AbyssPluginPutRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AbyssPluginPutRequest,
            pb::AbyssPluginPutResponse,
            endgame::handle_abyss_plugin_put_request
        ),
        pb::AbyssPluginSynthesisRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AbyssPluginSynthesisRequest,
            pb::AbyssPluginSynthesisResponse,
            endgame::handle_abyss_plugin_synthesis_request
        ),
        pb::AbyssRankListRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AbyssRankListRequest,
            pb::AbyssRankListResponse,
            endgame::handle_abyss_rank_list_request
        ),
        pb::AbyssReviveCharacterRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AbyssReviveCharacterRequest,
            pb::AbyssReviveCharacterResponse,
            endgame::handle_abyss_revive_character_request
        ),
        pb::AbyssRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AbyssRewardRequest,
            pb::AbyssRewardResponse,
            endgame::handle_abyss_reward_request
        ),
        pb::AbyssSelfPassRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AbyssSelfPassRequest,
            pb::AbyssSelfPassResponse,
            endgame::handle_abyss_self_pass_request
        ),
        pb::AbyssShowNameRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AbyssShowNameRequest,
            pb::AbyssShowNameResponse,
            endgame::handle_abyss_show_name_request
        ),
        pb::BossRushBatchClaimRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BossRushBatchClaimRewardRequest,
            pb::BossRushBatchClaimRewardResponse,
            endgame::handle_boss_rush_batch_claim_reward_request
        ),
        pb::BossRushBuffChooseResultRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BossRushBuffChooseResultRequest,
            pb::BossRushBuffChooseResultResponse,
            endgame::handle_boss_rush_buff_choose_result_request
        ),
        pb::BossRushChooseBuffRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BossRushChooseBuffRequest,
            pb::BossRushChooseBuffResponse,
            endgame::handle_boss_rush_choose_buff_request
        ),
        pb::BossRushClaimRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BossRushClaimRewardRequest,
            pb::BossRushClaimRewardResponse,
            endgame::handle_boss_rush_claim_reward_request
        ),
        pb::BossRushLevelScoreRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BossRushLevelScoreRewardRequest,
            pb::BossRushLevelScoreRewardResponse,
            endgame::handle_boss_rush_level_score_reward_request
        ),
        pb::BossRushSettlementRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BossRushSettlementRequest,
            pb::BossRushSettlementResponse,
            endgame::handle_boss_rush_settlement_request
        ),
        pb::BossRushTaskRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BossRushTaskRewardRequest,
            pb::BossRushTaskRewardResponse,
            endgame::handle_boss_rush_task_reward_request
        ),
        pb::BossPilingRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BossPilingRewardRequest,
            pb::BossPilingRewardResponse,
            endgame::handle_boss_piling_reward_request
        ),
        pb::BossPilingSettleRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BossPilingSettleRequest,
            pb::BossPilingSettleResponse,
            endgame::handle_boss_piling_settle_request
        ),
        pb::BabelTowerDailyTaskRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BabelTowerDailyTaskRewardRequest,
            pb::BabelTowerDailyTaskRewardResponse,
            endgame::handle_babel_tower_daily_task_reward_request
        ),
        pb::BabelTowerRankListRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BabelTowerRankListRequest,
            pb::BabelTowerRankListResponse,
            endgame::handle_babel_tower_rank_list_request
        ),
        pb::BabelTowerSelfPassRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BabelTowerSelfPassRequest,
            pb::BabelTowerSelfPassResponse,
            endgame::handle_babel_tower_self_pass_request
        ),
        pb::BabelTowerSettlementRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BabelTowerSettlementRequest,
            pb::BabelTowerSettlementResponse,
            endgame::handle_babel_tower_settlement_request
        ),
        pb::BabelTowerShowNameRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BabelTowerShowNameRequest,
            pb::BabelTowerShowNameResponse,
            endgame::handle_babel_tower_show_name_request
        ),
        pb::BabelTowerTaskRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BabelTowerTaskRewardRequest,
            pb::BabelTowerTaskRewardResponse,
            endgame::handle_babel_tower_task_reward_request
        ),
        pb::ResetBabelTowerLevelRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ResetBabelTowerLevelRequest,
            pb::ResetBabelTowerLevelResponse,
            endgame::handle_reset_babel_tower_level_request
        ),
        pb::SelectBabelActivityDeTermRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SelectBabelActivityDeTermRequest,
            pb::SelectBabelActivityDeTermResponse,
            endgame::handle_select_babel_activity_de_term_request
        ),
        pb::EndLessHistoryRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EndLessHistoryRequest,
            pb::EndLessHistoryResponse,
            endgame::handle_end_less_history_request
        ),
        pb::DollSmallMapInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DollSmallMapInfoRequest,
            pb::DollSmallMapInfoResponse,
            minimal::handle_doll_small_map_info_request
        ),
        pb::RoleVisionMainPhantomRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleVisionMainPhantomRequest,
            pb::RoleVisionMainPhantomResponse,
            minimal::handle_role_vision_main_phantom_request
        ),
        pb::PhBaPlanUsePlanRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhBaPlanUsePlanRequest,
            pb::PhBaPlanUsePlanResponse,
            minimal::handle_ph_ba_plan_use_plan_request
        ),
        pb::PhBaPlanSaveUsePlanRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhBaPlanSaveUsePlanRequest,
            pb::PhBaPlanSaveUsePlanResponse,
            minimal::handle_ph_ba_plan_save_use_plan_request
        ),
        pb::PhBaPlanFindPlanRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhBaPlanFindPlanRequest,
            pb::PhBaPlanFindPlanResponse,
            minimal::handle_ph_ba_plan_find_plan_request
        ),
        pb::PhBaPlanUpdatePlanRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhBaPlanUpdatePlanRequest,
            pb::PhBaPlanUpdatePlanResponse,
            minimal::handle_ph_ba_plan_update_plan_request
        ),
        pb::PhBaPlanSetFiveStarSwitchRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhBaPlanSetFiveStarSwitchRequest,
            pb::PhBaPlanSetFiveStarSwitchResponse,
            minimal::handle_ph_ba_plan_set_five_star_switch_request
        ),
        pb::PhBaPlanSetPlanStatusRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhBaPlanSetPlanStatusRequest,
            pb::PhBaPlanSetPlanStatusResponse,
            minimal::handle_ph_ba_plan_set_plan_status_request
        ),
        pb::PhBaPlanBatchOperRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhBaPlanBatchOperRequest,
            pb::PhBaPlanBatchOperResponse,
            minimal::handle_ph_ba_plan_batch_oper_request
        ),
        pb::PhantomSettingBatchUpdateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhantomSettingBatchUpdateRequest,
            pb::PhantomSettingBatchUpdateResponse,
            minimal::handle_phantom_setting_batch_update_request
        ),
        pb::GetDetectionLabelInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::GetDetectionLabelInfoRequest,
            pb::GetDetectionLabelInfoResponse,
            minimal::handle_get_detection_label_info_request
        ),
        pb::DetectionRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DetectionRequest,
            pb::DetectionResponse,
            minimal::handle_detection_request
        ),
        pb::SelectDetectionTargetRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SelectDetectionTargetRequest,
            pb::SelectDetectionTargetResponse,
            minimal::handle_select_detection_target_request
        ),
        pb::MonsterDetectionInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MonsterDetectionInfoRequest,
            pb::MonsterDetectionInfoResponse,
            minimal::handle_monster_detection_info_request
        ),
        pb::LevelPlayVarAsyncRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LevelPlayVarAsyncRequest,
            pb::LevelPlayVarAsyncResponse,
            minimal::handle_level_play_var_async_request
        ),
        pb::RoleDevelopConfigRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleDevelopConfigRequest,
            pb::RoleDevelopConfigResponse,
            minimal::handle_role_develop_config_request
        ),
        pb::ServerPlayStationPlayOnlyStateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ServerPlayStationPlayOnlyStateRequest,
            pb::ServerPlayStationPlayOnlyStateResponse,
            minimal::handle_server_play_station_play_only_state_request
        ),
        pb::XboxSettingRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::XboxSettingRequest,
            pb::XboxSettingResponse,
            minimal::handle_xbox_setting_request
        ),
        pb::PhotoMemoryRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhotoMemoryRequest,
            pb::PhotoMemoryResponse,
            minimal::handle_photo_memory_request
        ),
        pb::SheriffZoneInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SheriffZoneInfoRequest,
            pb::SheriffZoneInfoResponse,
            minimal::handle_sheriff_zone_info_request
        ),
        pb::WebSignRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::WebSignRequest,
            pb::WebSignResponse,
            minimal::handle_web_sign_request
        ),
        pb::ClientPullResourcePackageRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ClientPullResourcePackageRequest,
            pb::ClientPullResourcePackageResponse,
            minimal::handle_client_pull_resource_package_request
        ),
        pb::FormationAttrRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FormationAttrRequest,
            pb::FormationAttrResponse,
            minimal::handle_formation_attr_request
        ),
        pb::CombatMessagePrefixInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::CombatMessagePrefixInfoRequest,
            pb::CombatMessagePrefixInfoResponse,
            minimal::handle_combat_message_prefix_info_request
        ),
        pb::PrivateChatOperateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PrivateChatOperateRequest,
            pb::PrivateChatOperateResponse,
            minimal::handle_private_chat_operate_request
        ),
        pb::EntityPatrolStopRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EntityPatrolStopRequest,
            pb::EntityPatrolStopResponse,
            minimal::handle_entity_patrol_stop_request
        ),
        pb::GivebackInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::GivebackInfoRequest,
            pb::GivebackInfoResponse,
            minimal::handle_giveback_info_request
        ),
        pb::ValidTimeItemRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ValidTimeItemRequest,
            pb::ValidTimeItemResponse,
            minimal::handle_valid_time_item_request
        ),
        pb::ExchangeRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ExchangeRewardRequest,
            pb::ExchangeRewardResponse,
            minimal::handle_exchange_reward_request
        ),
        pb::UpdatePlayStationBlockAccountRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::UpdatePlayStationBlockAccountRequest,
            pb::UpdatePlayStationBlockAccountResponse,
            minimal::handle_update_play_station_block_account_request
        ),
        pb::PlayerXboxBlockListRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PlayerXboxBlockListRequest,
            pb::PlayerXboxBlockListResponse,
            minimal::handle_player_xbox_block_list_request
        ),
        pb::TeamRecommendRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TeamRecommendRequest,
            pb::TeamRecommendResponse,
            minimal::handle_team_recommend_request
        ),
        pb::ItemExchangeInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ItemExchangeInfoRequest,
            pb::ItemExchangeInfoResponse,
            minimal::handle_item_exchange_info_request
        ),
        pb::PayInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PayInfoRequest,
            pb::PayInfoResponse,
            minimal::handle_pay_info_request
        ),
        pb::BtnStateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BtnStateRequest,
            pb::BtnStateResponse,
            minimal::handle_btn_state_request
        ),
        pb::LevelPlayStateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LevelPlayStateRequest,
            pb::LevelPlayStateResponse,
            minimal::handle_level_play_state_request
        ),
        pb::SecGetReportData2FlowRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SecGetReportData2FlowRequest,
            pb::SecGetReportData2FlowResponse,
            minimal::handle_sec_get_report_data2_flow_request
        ),
        pb::SceneBlockSplitPlayerNeedBlockPush::MSG_ID => push_noev!(
            state,
            msg_id,
            payload,
            pb::SceneBlockSplitPlayerNeedBlockPush,
            minimal::handle_scene_block_split_player_need_block_push
        ),
        pb::BlackScreenStatePush::MSG_ID => push_noev!(
            state,
            msg_id,
            payload,
            pb::BlackScreenStatePush,
            minimal::handle_black_screen_state_push
        ),
        pb::AdviceSetRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AdviceSetRequest,
            pb::AdviceSetResponse,
            minimal::handle_advice_set_request
        ),
        pb::EntityFollowTrackRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EntityFollowTrackRequest,
            pb::EntityFollowTrackResponse,
            minimal::handle_entity_follow_track_request
        ),
        pb::EntityOnLandedRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EntityOnLandedRequest,
            pb::EntityOnLandedResponse,
            minimal::handle_entity_on_landed_request
        ),
        pb::CombatMessagePostInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::CombatMessagePostInfoRequest,
            pb::CombatMessagePostInfoResponse,
            minimal::handle_combat_message_post_info_request
        ),
        pb::RoleOperateSelfBgmRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RoleOperateSelfBgmRequest,
            pb::RoleOperateSelfBgmResponse,
            minimal::handle_role_operate_self_bgm_request
        ),
        pb::CdKeyVerifyRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::CdKeyVerifyRequest,
            pb::CdKeyVerifyResponse,
            minimal::handle_cd_key_verify_request
        ),
        pb::DrownRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DrownRequest,
            pb::DrownResponse,
            minimal::handle_drown_request
        ),
        pb::DrownEndTeleportRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DrownEndTeleportRequest,
            pb::DrownEndTeleportResponse,
            minimal::handle_drown_end_teleport_request
        ),
        pb::EnterViewDirectionRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EnterViewDirectionRequest,
            pb::EnterViewDirectionResponse,
            minimal::handle_enter_view_direction_request
        ),
        pb::ExitViewDirectionRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ExitViewDirectionRequest,
            pb::ExitViewDirectionResponse,
            minimal::handle_exit_view_direction_request
        ),
        pb::ClientCurrentRoleReportRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ClientCurrentRoleReportRequest,
            pb::ClientCurrentRoleReportResponse,
            minimal::handle_client_current_role_report_request
        ),
        pb::CharacterDeathPush::MSG_ID => push_noev!(
            state,
            msg_id,
            payload,
            pb::CharacterDeathPush,
            minimal::handle_character_death_push
        ),
        pb::PlayerNetStatePush::MSG_ID => push_noev!(
            state,
            msg_id,
            payload,
            pb::PlayerNetStatePush,
            minimal::handle_player_net_state_push
        ),
        pb::RobotDestinationPush::MSG_ID => push_noev!(
            state,
            msg_id,
            payload,
            pb::RobotDestinationPush,
            minimal::handle_robot_destination_push
        ),
        pb::NetTestDataPush::MSG_ID => push_noev!(
            state,
            msg_id,
            payload,
            pb::NetTestDataPush,
            minimal::handle_net_test_data_push
        ),
        pb::MaxMessageIdPush::MSG_ID => push_noev!(
            state,
            msg_id,
            payload,
            pb::MaxMessageIdPush,
            minimal::handle_max_message_id_push
        ),
        pb::DangoBrawlBagOperateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DangoBrawlBagOperateRequest,
            pb::DangoBrawlBagOperateResponse,
            events_major::handle_dango_brawl_bag_operate_request
        ),
        pb::DangoBrawlEventRefreshRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DangoBrawlEventRefreshRequest,
            pb::DangoBrawlEventRefreshResponse,
            events_major::handle_dango_brawl_event_refresh_request
        ),
        pb::DangoBrawlEventSelectRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DangoBrawlEventSelectRequest,
            pb::DangoBrawlEventSelectResponse,
            events_major::handle_dango_brawl_event_select_request
        ),
        pb::DangoBrawlStartFightRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DangoBrawlStartFightRequest,
            pb::DangoBrawlStartFightResponse,
            events_major::handle_dango_brawl_start_fight_request
        ),
        pb::DangoBrawlFightOverRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DangoBrawlFightOverRequest,
            pb::DangoBrawlFightOverResponse,
            events_major::handle_dango_brawl_fight_over_request
        ),
        pb::DangoBrawlSettleRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DangoBrawlSettleRequest,
            pb::DangoBrawlSettleResponse,
            events_major::handle_dango_brawl_settle_request
        ),
        pb::DangoBrawlScoreRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DangoBrawlScoreRewardRequest,
            pb::DangoBrawlScoreRewardResponse,
            events_major::handle_dango_brawl_score_reward_request
        ),
        pb::DangoBrawlTaskRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DangoBrawlTaskRewardRequest,
            pb::DangoBrawlTaskRewardResponse,
            events_major::handle_dango_brawl_task_reward_request
        ),
        pb::DangoBrawlShopBuyRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DangoBrawlShopBuyRequest,
            pb::DangoBrawlShopBuyResponse,
            events_major::handle_dango_brawl_shop_buy_request
        ),
        pb::DangoBrawlShopComposeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DangoBrawlShopComposeRequest,
            pb::DangoBrawlShopComposeResponse,
            events_major::handle_dango_brawl_shop_compose_request
        ),
        pb::DangoBrawlShopLockRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DangoBrawlShopLockRequest,
            pb::DangoBrawlShopLockResponse,
            events_major::handle_dango_brawl_shop_lock_request
        ),
        pb::DangoBrawlShopRefreshRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DangoBrawlShopRefreshRequest,
            pb::DangoBrawlShopRefreshResponse,
            events_major::handle_dango_brawl_shop_refresh_request
        ),
        pb::DangoBrawlShopSellRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DangoBrawlShopSellRequest,
            pb::DangoBrawlShopSellResponse,
            events_major::handle_dango_brawl_shop_sell_request
        ),
        pb::DangoMonopolyDiceRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DangoMonopolyDiceRequest,
            pb::DangoMonopolyDiceResponse,
            events_major::handle_dango_monopoly_dice_request
        ),
        pb::DangoMonopolyGridRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DangoMonopolyGridRewardRequest,
            pb::DangoMonopolyGridRewardResponse,
            events_major::handle_dango_monopoly_grid_reward_request
        ),
        pb::DangoMonopolyNextBoardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DangoMonopolyNextBoardRequest,
            pb::DangoMonopolyNextBoardResponse,
            events_major::handle_dango_monopoly_next_board_request
        ),
        pb::DangoMonopolyBoardRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DangoMonopolyBoardRewardRequest,
            pb::DangoMonopolyBoardRewardResponse,
            events_major::handle_dango_monopoly_board_reward_request
        ),
        pb::DangoMonopolyTaskRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DangoMonopolyTaskRewardRequest,
            pb::DangoMonopolyTaskRewardResponse,
            events_major::handle_dango_monopoly_task_reward_request
        ),
        pb::DollDeliveryRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DollDeliveryRequest,
            pb::DollDeliveryResponse,
            events_major::handle_doll_delivery_request
        ),
        pb::DollGarbInsertCoinRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DollGarbInsertCoinRequest,
            pb::DollGarbInsertCoinResponse,
            events_major::handle_doll_garb_insert_coin_request
        ),
        pb::DollGarbStartRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DollGarbStartRequest,
            pb::DollGarbStartResponse,
            events_major::handle_doll_garb_start_request
        ),
        pb::DollGrabPauseStartRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DollGrabPauseStartRequest,
            pb::DollGrabPauseStartResponse,
            events_major::handle_doll_grab_pause_start_request
        ),
        pb::DollGrabPauseEndRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DollGrabPauseEndRequest,
            pb::DollGrabPauseEndResponse,
            events_major::handle_doll_grab_pause_end_request
        ),
        pb::DollGrabRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DollGrabRequest,
            pb::DollGrabResponse,
            events_major::handle_doll_grab_request
        ),
        pb::DollGrabTimeBallRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DollGrabTimeBallRequest,
            pb::DollGrabTimeBallResponse,
            events_major::handle_doll_grab_time_ball_request
        ),
        pb::DollGrabStopRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DollGrabStopRequest,
            pb::DollGrabStopResponse,
            events_major::handle_doll_grab_stop_request
        ),
        pb::FloroRanchFirstEnterRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FloroRanchFirstEnterRequest,
            pb::FloroRanchFirstEnterResponse,
            events_major::handle_floro_ranch_first_enter_request
        ),
        pb::FloroRanchAnimationRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FloroRanchAnimationRequest,
            pb::FloroRanchAnimationResponse,
            events_major::handle_floro_ranch_animation_request
        ),
        pb::FloroRanchStartPlayRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FloroRanchStartPlayRequest,
            pb::FloroRanchStartPlayResponse,
            events_major::handle_floro_ranch_start_play_request
        ),
        pb::FloroRanchPlayNextDayRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FloroRanchPlayNextDayRequest,
            pb::FloroRanchPlayNextDayResponse,
            events_major::handle_floro_ranch_play_next_day_request
        ),
        pb::FloroRanchPlayTributeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FloroRanchPlayTributeRequest,
            pb::FloroRanchPlayTributeResponse,
            events_major::handle_floro_ranch_play_tribute_request
        ),
        pb::FloroRanchPlayUnlimitedModeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FloroRanchPlayUnlimitedModeRequest,
            pb::FloroRanchPlayUnlimitedModeResponse,
            events_major::handle_floro_ranch_play_unlimited_mode_request
        ),
        pb::FloroRanchPlayGachaRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FloroRanchPlayGachaRequest,
            pb::FloroRanchPlayGachaResponse,
            events_major::handle_floro_ranch_play_gacha_request
        ),
        pb::FloroRanchPlayGachaRefreshRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FloroRanchPlayGachaRefreshRequest,
            pb::FloroRanchPlayGachaRefreshResponse,
            events_major::handle_floro_ranch_play_gacha_refresh_request
        ),
        pb::FloroRanchPlayRefreshShopRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FloroRanchPlayRefreshShopRequest,
            pb::FloroRanchPlayRefreshShopResponse,
            events_major::handle_floro_ranch_play_refresh_shop_request
        ),
        pb::FloroRanchPlayShopBuyRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FloroRanchPlayShopBuyRequest,
            pb::FloroRanchPlayShopBuyResponse,
            events_major::handle_floro_ranch_play_shop_buy_request
        ),
        pb::FloroRanchPlaySelectCardGroupRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FloroRanchPlaySelectCardGroupRequest,
            pb::FloroRanchPlaySelectCardGroupResponse,
            events_major::handle_floro_ranch_play_select_card_group_request
        ),
        pb::FloroRanchPlayRemoveUnitRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FloroRanchPlayRemoveUnitRequest,
            pb::FloroRanchPlayRemoveUnitResponse,
            events_major::handle_floro_ranch_play_remove_unit_request
        ),
        pb::FloroRanchCloseTaskRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FloroRanchCloseTaskRequest,
            pb::FloroRanchCloseTaskResponse,
            events_major::handle_floro_ranch_close_task_request
        ),
        pb::FloroRanchEventChoiceRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FloroRanchEventChoiceRequest,
            pb::FloroRanchEventChoiceResponse,
            events_major::handle_floro_ranch_event_choice_request
        ),
        pb::FloroRanchExecuteSkillPlayRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FloroRanchExecuteSkillPlayRequest,
            pb::FloroRanchExecuteSkillPlayResponse,
            events_major::handle_floro_ranch_execute_skill_play_request
        ),
        pb::FloroRanchOutRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FloroRanchOutRequest,
            pb::FloroRanchOutResponse,
            events_major::handle_floro_ranch_out_request
        ),
        pb::FloroRanchUnlockTechPointRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FloroRanchUnlockTechPointRequest,
            pb::FloroRanchUnlockTechPointResponse,
            events_major::handle_floro_ranch_unlock_tech_point_request
        ),
        pb::FloroRanchRewardMilestoneRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FloroRanchRewardMilestoneRequest,
            pb::FloroRanchRewardMilestoneResponse,
            events_major::handle_floro_ranch_reward_milestone_request
        ),
        pb::FloroRanchRewardTaskRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FloroRanchRewardTaskRequest,
            pb::FloroRanchRewardTaskResponse,
            events_major::handle_floro_ranch_reward_task_request
        ),
        pb::FloroRanchSettleDataRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FloroRanchSettleDataRequest,
            pb::FloroRanchSettleDataResponse,
            events_major::handle_floro_ranch_settle_data_request
        ),
        pb::FloroRanchSettleRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FloroRanchSettleRequest,
            pb::FloroRanchSettleResponse,
            events_major::handle_floro_ranch_settle_request
        ),
        pb::HonamiStoryActivityInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::HonamiStoryActivityInfoRequest,
            pb::HonamiStoryActivityInfoResponse,
            events_major::handle_honami_story_activity_info_request
        ),
        pb::HonamiStoryActivateTalentRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::HonamiStoryActivateTalentRequest,
            pb::HonamiStoryActivateTalentResponse,
            events_major::handle_honami_story_activate_talent_request
        ),
        pb::HonamiStoryAreaSecretRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::HonamiStoryAreaSecretRewardRequest,
            pb::HonamiStoryAreaSecretRewardResponse,
            events_major::handle_honami_story_area_secret_reward_request
        ),
        pb::HonamiStoryBagOperateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::HonamiStoryBagOperateRequest,
            pb::HonamiStoryBagOperateResponse,
            events_major::handle_honami_story_bag_operate_request
        ),
        pb::HonamiStoryDiscardItemRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::HonamiStoryDiscardItemRequest,
            pb::HonamiStoryDiscardItemResponse,
            events_major::handle_honami_story_discard_item_request
        ),
        pb::HonamiStoryPickUpItemRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::HonamiStoryPickUpItemRequest,
            pb::HonamiStoryPickUpItemResponse,
            events_major::handle_honami_story_pick_up_item_request
        ),
        pb::HonamiStoryLockItemRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::HonamiStoryLockItemRequest,
            pb::HonamiStoryLockItemResponse,
            events_major::handle_honami_story_lock_item_request
        ),
        pb::HonamiStorySellItemRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::HonamiStorySellItemRequest,
            pb::HonamiStorySellItemResponse,
            events_major::handle_honami_story_sell_item_request
        ),
        pb::HonamiStoryEquipRoleRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::HonamiStoryEquipRoleRequest,
            pb::HonamiStoryEquipRoleResponse,
            events_major::handle_honami_story_equip_role_request
        ),
        pb::HonamiStoryInstLeaveRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::HonamiStoryInstLeaveRequest,
            pb::HonamiStoryInstLeaveResponse,
            events_major::handle_honami_story_inst_leave_request
        ),
        pb::HonamiStoryItemCollectionRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::HonamiStoryItemCollectionRequest,
            pb::HonamiStoryItemCollectionResponse,
            events_major::handle_honami_story_item_collection_request
        ),
        pb::HonamiStoryLifeSupportUpRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::HonamiStoryLifeSupportUpRequest,
            pb::HonamiStoryLifeSupportUpResponse,
            events_major::handle_honami_story_life_support_up_request
        ),
        pb::HonamiStoryLimitTaskRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::HonamiStoryLimitTaskRewardRequest,
            pb::HonamiStoryLimitTaskRewardResponse,
            events_major::handle_honami_story_limit_task_reward_request
        ),
        pb::HonamiStoryResidentTaskRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::HonamiStoryResidentTaskRewardRequest,
            pb::HonamiStoryResidentTaskRewardResponse,
            events_major::handle_honami_story_resident_task_reward_request
        ),
        pb::HonamiStoryMascotRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::HonamiStoryMascotRewardRequest,
            pb::HonamiStoryMascotRewardResponse,
            events_major::handle_honami_story_mascot_reward_request
        ),
        pb::HonamiStoryScoreRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::HonamiStoryScoreRewardRequest,
            pb::HonamiStoryScoreRewardResponse,
            events_major::handle_honami_story_score_reward_request
        ),
        pb::HonamiStoryUnlockSlotRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::HonamiStoryUnlockSlotRequest,
            pb::HonamiStoryUnlockSlotResponse,
            events_major::handle_honami_story_unlock_slot_request
        ),
        pb::HonamiStoryWeaponDressRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::HonamiStoryWeaponDressRequest,
            pb::HonamiStoryWeaponDressResponse,
            events_major::handle_honami_story_weapon_dress_request
        ),
        pb::JokerGuessStartNewRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::JokerGuessStartNewRequest,
            pb::JokerGuessStartNewResponse,
            events_major::handle_joker_guess_start_new_request
        ),
        pb::JokerGuessDrawCardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::JokerGuessDrawCardRequest,
            pb::JokerGuessDrawCardResponse,
            events_major::handle_joker_guess_draw_card_request
        ),
        pb::JokerGuessPlayCardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::JokerGuessPlayCardRequest,
            pb::JokerGuessPlayCardResponse,
            events_major::handle_joker_guess_play_card_request
        ),
        pb::JokerGuessUseSkillRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::JokerGuessUseSkillRequest,
            pb::JokerGuessUseSkillResponse,
            events_major::handle_joker_guess_use_skill_request
        ),
        pb::JokerGuessRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::JokerGuessRewardRequest,
            pb::JokerGuessRewardResponse,
            events_major::handle_joker_guess_reward_request
        ),
        pb::KurotatoSyncEntityDealDamagePush::MSG_ID => push_noev!(
            state,
            msg_id,
            payload,
            pb::KurotatoSyncEntityDealDamagePush,
            events_major::handle_kurotato_sync_entity_deal_damage_push
        ),
        pb::KurotatoChestRewardSelectRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::KurotatoChestRewardSelectRequest,
            pb::KurotatoChestRewardSelectResponse,
            events_major::handle_kurotato_chest_reward_select_request
        ),
        pb::KurotatoEndShopStepRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::KurotatoEndShopStepRequest,
            pb::KurotatoEndShopStepResponse,
            events_major::handle_kurotato_end_shop_step_request
        ),
        pb::KurotatoEndSpecialWaveCombatRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::KurotatoEndSpecialWaveCombatRequest,
            pb::KurotatoEndSpecialWaveCombatResponse,
            events_major::handle_kurotato_end_special_wave_combat_request
        ),
        pb::KurotatoGoldAddRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::KurotatoGoldAddRequest,
            pb::KurotatoGoldAddResponse,
            events_major::handle_kurotato_gold_add_request
        ),
        pb::KurotatoLevelRecordDeleteRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::KurotatoLevelRecordDeleteRequest,
            pb::KurotatoLevelRecordDeleteResponse,
            events_major::handle_kurotato_level_record_delete_request
        ),
        pb::KurotatoLimitTaskReceiveRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::KurotatoLimitTaskReceiveRequest,
            pb::KurotatoLimitTaskReceiveResponse,
            events_major::handle_kurotato_limit_task_receive_request
        ),
        pb::KurotatoResTaskReceiveRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::KurotatoResTaskReceiveRequest,
            pb::KurotatoResTaskReceiveResponse,
            events_major::handle_kurotato_res_task_receive_request
        ),
        pb::KurotatoScoreTaskReceiveRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::KurotatoScoreTaskReceiveRequest,
            pb::KurotatoScoreTaskReceiveResponse,
            events_major::handle_kurotato_score_task_receive_request
        ),
        pb::KurotatoReChallengeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::KurotatoReChallengeRequest,
            pb::KurotatoReChallengeResponse,
            events_major::handle_kurotato_re_challenge_request
        ),
        pb::KurotatoRefineWeaponRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::KurotatoRefineWeaponRequest,
            pb::KurotatoRefineWeaponResponse,
            events_major::handle_kurotato_refine_weapon_request
        ),
        pb::KurotatoSellWeaponRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::KurotatoSellWeaponRequest,
            pb::KurotatoSellWeaponResponse,
            events_major::handle_kurotato_sell_weapon_request
        ),
        pb::KurotatoRoleRecordRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::KurotatoRoleRecordRequest,
            pb::KurotatoRoleRecordResponse,
            events_major::handle_kurotato_role_record_request
        ),
        pb::KurotatoSettlementRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::KurotatoSettlementRequest,
            pb::KurotatoSettlementResponse,
            events_major::handle_kurotato_settlement_request
        ),
        pb::KurotatoShopRewardLockRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::KurotatoShopRewardLockRequest,
            pb::KurotatoShopRewardLockResponse,
            events_major::handle_kurotato_shop_reward_lock_request
        ),
        pb::KurotatoShopRewardRefreshRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::KurotatoShopRewardRefreshRequest,
            pb::KurotatoShopRewardRefreshResponse,
            events_major::handle_kurotato_shop_reward_refresh_request
        ),
        pb::KurotatoShopRewardSelectRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::KurotatoShopRewardSelectRequest,
            pb::KurotatoShopRewardSelectResponse,
            events_major::handle_kurotato_shop_reward_select_request
        ),
        pb::KurotatoStepAdvanceRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::KurotatoStepAdvanceRequest,
            pb::KurotatoStepAdvanceResponse,
            events_major::handle_kurotato_step_advance_request
        ),
        pb::KurotatoUpgradeRewardRefreshRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::KurotatoUpgradeRewardRefreshRequest,
            pb::KurotatoUpgradeRewardRefreshResponse,
            events_major::handle_kurotato_upgrade_reward_refresh_request
        ),
        pb::KurotatoUpgradeRewardSelectRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::KurotatoUpgradeRewardSelectRequest,
            pb::KurotatoUpgradeRewardSelectResponse,
            events_major::handle_kurotato_upgrade_reward_select_request
        ),
        pb::PinballBonusFullWinRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PinballBonusFullWinRequest,
            pb::PinballBonusFullWinResponse,
            events_major::handle_pinball_bonus_full_win_request
        ),
        pb::PinballFriendRankRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PinballFriendRankRequest,
            pb::PinballFriendRankResponse,
            events_major::handle_pinball_friend_rank_request
        ),
        pb::PinballInGameKickBallRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PinballInGameKickBallRequest,
            pb::PinballInGameKickBallResponse,
            events_major::handle_pinball_in_game_kick_ball_request
        ),
        pb::PinballInGameSyncPayloadRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PinballInGameSyncPayloadRequest,
            pb::PinballInGameSyncPayloadResponse,
            events_major::handle_pinball_in_game_sync_payload_request
        ),
        pb::PinballQuitRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PinballQuitRequest,
            pb::PinballQuitResponse,
            events_major::handle_pinball_quit_request
        ),
        pb::PinballRoleLevelUpRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PinballRoleLevelUpRequest,
            pb::PinballRoleLevelUpResponse,
            events_major::handle_pinball_role_level_up_request
        ),
        pb::PinballSetPayloadRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PinballSetPayloadRequest,
            pb::PinballSetPayloadResponse,
            events_major::handle_pinball_set_payload_request
        ),
        pb::PinballTaskRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PinballTaskRewardRequest,
            pb::PinballTaskRewardResponse,
            events_major::handle_pinball_task_reward_request
        ),
        pb::PinballTeamDieRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PinballTeamDieRequest,
            pb::PinballTeamDieResponse,
            events_major::handle_pinball_team_die_request
        ),
        pb::PinballTeamReviveRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PinballTeamReviveRequest,
            pb::PinballTeamReviveResponse,
            events_major::handle_pinball_team_revive_request
        ),
        pb::PinballWeaponDegradeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PinballWeaponDegradeRequest,
            pb::PinballWeaponDegradeResponse,
            events_major::handle_pinball_weapon_degrade_request
        ),
        pb::PinballWeaponWearRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PinballWeaponWearRequest,
            pb::PinballWeaponWearResponse,
            events_major::handle_pinball_weapon_wear_request
        ),
        pb::RacingBetMatchInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RacingBetMatchInfoRequest,
            pb::RacingBetMatchInfoResponse,
            events_major::handle_racing_bet_match_info_request
        ),
        pb::RacingBetMatchActionRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RacingBetMatchActionRequest,
            pb::RacingBetMatchActionResponse,
            events_major::handle_racing_bet_match_action_request
        ),
        pb::RacingBetMatchRoundActionRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RacingBetMatchRoundActionRequest,
            pb::RacingBetMatchRoundActionResponse,
            events_major::handle_racing_bet_match_round_action_request
        ),
        pb::RacingBetsBulletScreenRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RacingBetsBulletScreenRequest,
            pb::RacingBetsBulletScreenResponse,
            events_major::handle_racing_bets_bullet_screen_request
        ),
        pb::RacingBetsCloseSettleMenuRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RacingBetsCloseSettleMenuRequest,
            pb::RacingBetsCloseSettleMenuResponse,
            events_major::handle_racing_bets_close_settle_menu_request
        ),
        pb::RacingBetsGearRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RacingBetsGearRequest,
            pb::RacingBetsGearResponse,
            events_major::handle_racing_bets_gear_request
        ),
        pb::RacingBetsGearRefundRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RacingBetsGearRefundRequest,
            pb::RacingBetsGearRefundResponse,
            events_major::handle_racing_bets_gear_refund_request
        ),
        pb::RacingBetsMatchInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RacingBetsMatchInfoRequest,
            pb::RacingBetsMatchInfoResponse,
            events_major::handle_racing_bets_match_info_request
        ),
        pb::RacingBetsRankRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RacingBetsRankRequest,
            pb::RacingBetsRankResponse,
            events_major::handle_racing_bets_rank_request
        ),
        pb::RacingBetsUpdateOddsRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RacingBetsUpdateOddsRequest,
            pb::RacingBetsUpdateOddsResponse,
            events_major::handle_racing_bets_update_odds_request
        ),
        pb::RacingBetsTaskRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RacingBetsTaskRewardRequest,
            pb::RacingBetsTaskRewardResponse,
            events_major::handle_racing_bets_task_reward_request
        ),
        pb::RacingBetsRepeatedTaskRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RacingBetsRepeatedTaskRewardRequest,
            pb::RacingBetsRepeatedTaskRewardResponse,
            events_major::handle_racing_bets_repeated_task_reward_request
        ),
        pb::RegressBonusRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RegressBonusRequest,
            pb::RegressBonusResponse,
            events_major::handle_regress_bonus_request
        ),
        pb::RegressBuyBonusLvRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RegressBuyBonusLvRequest,
            pb::RegressBuyBonusLvResponse,
            events_major::handle_regress_buy_bonus_lv_request
        ),
        pb::RegressDisposableRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RegressDisposableRewardRequest,
            pb::RegressDisposableRewardResponse,
            events_major::handle_regress_disposable_reward_request
        ),
        pb::RegressQuestionOpenRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RegressQuestionOpenRequest,
            pb::RegressQuestionOpenResponse,
            events_major::handle_regress_question_open_request
        ),
        pb::RegressTaskRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RegressTaskRequest,
            pb::RegressTaskResponse,
            events_major::handle_regress_task_request
        ),
        pb::RegressRepeatedTaskRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RegressRepeatedTaskRequest,
            pb::RegressRepeatedTaskResponse,
            events_major::handle_regress_repeated_task_request
        ),
        pb::RegressSetCurUseTrialRoleRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RegressSetCurUseTrialRoleRequest,
            pb::RegressSetCurUseTrialRoleResponse,
            events_major::handle_regress_set_cur_use_trial_role_request
        ),
        pb::RegressTrialRoleLvUpRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RegressTrialRoleLvUpRequest,
            pb::RegressTrialRoleLvUpResponse,
            events_major::handle_regress_trial_role_lv_up_request
        ),
        pb::RhythmChangePartnerRoleRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RhythmChangePartnerRoleRequest,
            pb::RhythmChangePartnerRoleResponse,
            events_major::handle_rhythm_change_partner_role_request
        ),
        pb::RhythmMainLineInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RhythmMainLineInfoRequest,
            pb::RhythmMainLineInfoResponse,
            events_major::handle_rhythm_main_line_info_request
        ),
        pb::RhythmRankListRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RhythmRankListRequest,
            pb::RhythmRankListResponse,
            events_major::handle_rhythm_rank_list_request
        ),
        pb::RhythmRankListSelfDataRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RhythmRankListSelfDataRequest,
            pb::RhythmRankListSelfDataResponse,
            events_major::handle_rhythm_rank_list_self_data_request
        ),
        pb::RhythmSetRedDotRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RhythmSetRedDotRequest,
            pb::RhythmSetRedDotResponse,
            events_major::handle_rhythm_set_red_dot_request
        ),
        pb::RhythmSettleRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RhythmSettleRequest,
            pb::RhythmSettleResponse,
            events_major::handle_rhythm_settle_request
        ),
        pb::RhythmTaskOneKeyRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RhythmTaskOneKeyRewardRequest,
            pb::RhythmTaskOneKeyRewardResponse,
            events_major::handle_rhythm_task_one_key_reward_request
        ),
        pb::RollBlockExitGamePlayRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RollBlockExitGamePlayRequest,
            pb::RollBlockExitGamePlayResponse,
            events_major::handle_roll_block_exit_game_play_request
        ),
        pb::RollBlockGamePlayReadyRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RollBlockGamePlayReadyRequest,
            pb::RollBlockGamePlayReadyResponse,
            events_major::handle_roll_block_game_play_ready_request
        ),
        pb::RollBlockInputRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RollBlockInputRequest,
            pb::RollBlockInputResponse,
            events_major::handle_roll_block_input_request
        ),
        pb::RollBlockMovementFinishRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RollBlockMovementFinishRequest,
            pb::RollBlockMovementFinishResponse,
            events_major::handle_roll_block_movement_finish_request
        ),
        pb::RollBlockResetGamePlayRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RollBlockResetGamePlayRequest,
            pb::RollBlockResetGamePlayResponse,
            events_major::handle_roll_block_reset_game_play_request
        ),
        pb::RollBlockSwitchControlBlockRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RollBlockSwitchControlBlockRequest,
            pb::RollBlockSwitchControlBlockResponse,
            events_major::handle_roll_block_switch_control_block_request
        ),
        pb::RollBlockToggleHintRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RollBlockToggleHintRequest,
            pb::RollBlockToggleHintResponse,
            events_major::handle_roll_block_toggle_hint_request
        ),
        pb::SpringFestivalAtmosphereRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SpringFestivalAtmosphereRewardRequest,
            pb::SpringFestivalAtmosphereRewardResponse,
            events_major::handle_spring_festival_atmosphere_reward_request
        ),
        pb::SpringFestivalScoreRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SpringFestivalScoreRewardRequest,
            pb::SpringFestivalScoreRewardResponse,
            events_major::handle_spring_festival_score_reward_request
        ),
        pb::SpringFestivalTaskRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SpringFestivalTaskRewardRequest,
            pb::SpringFestivalTaskRewardResponse,
            events_major::handle_spring_festival_task_reward_request
        ),
        pb::SpringSignDrawRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SpringSignDrawRewardRequest,
            pb::SpringSignDrawRewardResponse,
            events_major::handle_spring_sign_draw_reward_request
        ),
        pb::SpringSignDrawRoleRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SpringSignDrawRoleRequest,
            pb::SpringSignDrawRoleResponse,
            events_major::handle_spring_sign_draw_role_request
        ),
        pb::SpringSignPhotoRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SpringSignPhotoRewardRequest,
            pb::SpringSignPhotoRewardResponse,
            events_major::handle_spring_sign_photo_reward_request
        ),
        pb::SpringSignSkinRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SpringSignSkinRewardRequest,
            pb::SpringSignSkinRewardResponse,
            events_major::handle_spring_sign_skin_reward_request
        ),
        pb::SurvivorsLastInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SurvivorsLastInfoRequest,
            pb::SurvivorsLastInfoResponse,
            events_major::handle_survivors_last_info_request
        ),
        pb::SurvivorsLockRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SurvivorsLockRequest,
            pb::SurvivorsLockResponse,
            events_major::handle_survivors_lock_request
        ),
        pb::SurvivorsNormalAwardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SurvivorsNormalAwardRequest,
            pb::SurvivorsNormalAwardResponse,
            events_major::handle_survivors_normal_award_request
        ),
        pb::SurvivorsScoreAwardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SurvivorsScoreAwardRequest,
            pb::SurvivorsScoreAwardResponse,
            events_major::handle_survivors_score_award_request
        ),
        pb::SurvivorsOpDoneRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SurvivorsOpDoneRequest,
            pb::SurvivorsOpDoneResponse,
            events_major::handle_survivors_op_done_request
        ),
        pb::SurvivorsStepAdvanceRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SurvivorsStepAdvanceRequest,
            pb::SurvivorsStepAdvanceResponse,
            events_major::handle_survivors_step_advance_request
        ),
        pb::SurvivorsResultRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SurvivorsResultRequest,
            pb::SurvivorsResultResponse,
            events_major::handle_survivors_result_request
        ),
        pb::SurvivorsTalentLevelUpRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SurvivorsTalentLevelUpRequest,
            pb::SurvivorsTalentLevelUpResponse,
            events_major::handle_survivors_talent_level_up_request
        ),
        pb::SurvivorsWeaponInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SurvivorsWeaponInfoRequest,
            pb::SurvivorsWeaponInfoResponse,
            events_major::handle_survivors_weapon_info_request
        ),
        pb::TowerDefenceChallengeQuitRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TowerDefenceChallengeQuitRequest,
            pb::TowerDefenceChallengeQuitResponse,
            events_major::handle_tower_defence_challenge_quit_request
        ),
        pb::TowerDefenceInstanceRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TowerDefenceInstanceRewardRequest,
            pb::TowerDefenceInstanceRewardResponse,
            events_major::handle_tower_defence_instance_reward_request
        ),
        pb::TowerDefenceScoreRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TowerDefenceScoreRewardRequest,
            pb::TowerDefenceScoreRewardResponse,
            events_major::handle_tower_defence_score_reward_request
        ),
        pb::TowerDefenceRandListRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TowerDefenceRandListRequest,
            pb::TowerDefenceRandListResponse,
            events_major::handle_tower_defence_rand_list_request
        ),
        pb::TowerDefenceRankShowNameRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TowerDefenceRankShowNameRequest,
            pb::TowerDefenceRankShowNameResponse,
            events_major::handle_tower_defence_rank_show_name_request
        ),
        pb::TowerDefenceSelfPassRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TowerDefenceSelfPassRequest,
            pb::TowerDefenceSelfPassResponse,
            events_major::handle_tower_defence_self_pass_request
        ),
        pb::TrapDefenseAuxiliaryBranchRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TrapDefenseAuxiliaryBranchRequest,
            pb::TrapDefenseAuxiliaryBranchResponse,
            events_major::handle_trap_defense_auxiliary_branch_request
        ),
        pb::TrapDefenseAuxiliaryLevelUpRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TrapDefenseAuxiliaryLevelUpRequest,
            pb::TrapDefenseAuxiliaryLevelUpResponse,
            events_major::handle_trap_defense_auxiliary_level_up_request
        ),
        pb::TrapDefenseAuxiliaryResetRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TrapDefenseAuxiliaryResetRequest,
            pb::TrapDefenseAuxiliaryResetResponse,
            events_major::handle_trap_defense_auxiliary_reset_request
        ),
        pb::TrapDefenseBuildingBranchRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TrapDefenseBuildingBranchRequest,
            pb::TrapDefenseBuildingBranchResponse,
            events_major::handle_trap_defense_building_branch_request
        ),
        pb::TrapDefenseBuildingLevelUpRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TrapDefenseBuildingLevelUpRequest,
            pb::TrapDefenseBuildingLevelUpResponse,
            events_major::handle_trap_defense_building_level_up_request
        ),
        pb::TrapDefenseBuildingResetRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TrapDefenseBuildingResetRequest,
            pb::TrapDefenseBuildingResetResponse,
            events_major::handle_trap_defense_building_reset_request
        ),
        pb::TrapDefenseBuyShopProductRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TrapDefenseBuyShopProductRequest,
            pb::TrapDefenseBuyShopProductResponse,
            events_major::handle_trap_defense_buy_shop_product_request
        ),
        pb::TrapDefenseChallengeQuitRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TrapDefenseChallengeQuitRequest,
            pb::TrapDefenseChallengeQuitResponse,
            events_major::handle_trap_defense_challenge_quit_request
        ),
        pb::TrapDefenseCurChallengeInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TrapDefenseCurChallengeInfoRequest,
            pb::TrapDefenseCurChallengeInfoResponse,
            events_major::handle_trap_defense_cur_challenge_info_request
        ),
        pb::TrapDefenseDevelopResetRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TrapDefenseDevelopResetRequest,
            pb::TrapDefenseDevelopResetResponse,
            events_major::handle_trap_defense_develop_reset_request
        ),
        pb::TrapDefenseGetRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TrapDefenseGetRewardRequest,
            pb::TrapDefenseGetRewardResponse,
            events_major::handle_trap_defense_get_reward_request
        ),
        pb::TrapDefenseGetSpecialRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TrapDefenseGetSpecialRewardRequest,
            pb::TrapDefenseGetSpecialRewardResponse,
            events_major::handle_trap_defense_get_special_reward_request
        ),
        pb::TrapDefensePreviewFinishRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TrapDefensePreviewFinishRequest,
            pb::TrapDefensePreviewFinishResponse,
            events_major::handle_trap_defense_preview_finish_request
        ),
        pb::TrapDefenseRefreshGainingGroupRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TrapDefenseRefreshGainingGroupRequest,
            pb::TrapDefenseRefreshGainingGroupResponse,
            events_major::handle_trap_defense_refresh_gaining_group_request
        ),
        pb::TrapDefenseRefreshShopRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TrapDefenseRefreshShopRequest,
            pb::TrapDefenseRefreshShopResponse,
            events_major::handle_trap_defense_refresh_shop_request
        ),
        pb::TrapDefenseSelectGainingGroupRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TrapDefenseSelectGainingGroupRequest,
            pb::TrapDefenseSelectGainingGroupResponse,
            events_major::handle_trap_defense_select_gaining_group_request
        ),
        pb::TrapDefenseSlotUpdateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TrapDefenseSlotUpdateRequest,
            pb::TrapDefenseSlotUpdateResponse,
            events_major::handle_trap_defense_slot_update_request
        ),
        pb::TrapDefenseTechUnlockRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TrapDefenseTechUnlockRequest,
            pb::TrapDefenseTechUnlockResponse,
            events_major::handle_trap_defense_tech_unlock_request
        ),
        pb::TrapDefenseUseItemRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TrapDefenseUseItemRequest,
            pb::TrapDefenseUseItemResponse,
            events_major::handle_trap_defense_use_item_request
        ),
        pb::ActivityFirstReadRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ActivityFirstReadRequest,
            pb::ActivityFirstReadResponse,
            events_misc::handle_activity_first_read_request
        ),
        pb::ActivityPreOpenRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ActivityPreOpenRequest,
            pb::ActivityPreOpenResponse,
            events_misc::handle_activity_pre_open_request
        ),
        pb::ActivityPayShopBuyRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ActivityPayShopBuyRequest,
            pb::ActivityPayShopBuyResponse,
            events_misc::handle_activity_pay_shop_buy_request
        ),
        pb::ActivityResultRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ActivityResultRewardRequest,
            pb::ActivityResultRewardResponse,
            events_misc::handle_activity_result_reward_request
        ),
        pb::SignActivityRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SignActivityRequest,
            pb::SignActivityResponse,
            events_misc::handle_sign_activity_request
        ),
        pb::SignActivityFreeDropRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SignActivityFreeDropRequest,
            pb::SignActivityFreeDropResponse,
            events_misc::handle_sign_activity_free_drop_request
        ),
        pb::DailyAdventurePtRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DailyAdventurePtRewardRequest,
            pb::DailyAdventurePtRewardResponse,
            events_misc::handle_daily_adventure_pt_reward_request
        ),
        pb::DailyAdventureTaskRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DailyAdventureTaskRewardRequest,
            pb::DailyAdventureTaskRewardResponse,
            events_misc::handle_daily_adventure_task_reward_request
        ),
        pb::RunTurntableActivityRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RunTurntableActivityRequest,
            pb::RunTurntableActivityResponse,
            events_misc::handle_run_turntable_activity_request
        ),
        pb::TurntableCompleteRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TurntableCompleteRequest,
            pb::TurntableCompleteResponse,
            events_misc::handle_turntable_complete_request
        ),
        pb::ScratchCardActivityInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ScratchCardActivityInfoRequest,
            pb::ScratchCardActivityInfoResponse,
            events_misc::handle_scratch_card_activity_info_request
        ),
        pb::ScratchCardRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ScratchCardRewardRequest,
            pb::ScratchCardRewardResponse,
            events_misc::handle_scratch_card_reward_request
        ),
        pb::WuWuKujiGachaRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::WuWuKujiGachaRequest,
            pb::WuWuKujiGachaResponse,
            events_misc::handle_wu_wu_kuji_gacha_request
        ),
        pb::WuWuPackageRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::WuWuPackageRewardRequest,
            pb::WuWuPackageRewardResponse,
            events_misc::handle_wu_wu_package_reward_request
        ),
        pb::WuWuTaskRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::WuWuTaskRewardRequest,
            pb::WuWuTaskRewardResponse,
            events_misc::handle_wu_wu_task_reward_request
        ),
        pb::ConsumptiveActivityInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ConsumptiveActivityInfoRequest,
            pb::ConsumptiveActivityInfoResponse,
            events_misc::handle_consumptive_activity_info_request
        ),
        pb::ConsumptiveActivityScoreRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ConsumptiveActivityScoreRequest,
            pb::ConsumptiveActivityScoreResponse,
            events_misc::handle_consumptive_activity_score_request
        ),
        pb::ConsumptiveRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ConsumptiveRewardRequest,
            pb::ConsumptiveRewardResponse,
            events_misc::handle_consumptive_reward_request
        ),
        pb::PreheatSignSurveyInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PreheatSignSurveyInfoRequest,
            pb::PreheatSignSurveyInfoResponse,
            events_misc::handle_preheat_sign_survey_info_request
        ),
        pb::PreheatSignRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PreheatSignRewardRequest,
            pb::PreheatSignRewardResponse,
            events_misc::handle_preheat_sign_reward_request
        ),
        pb::H5CircumFluenceActivityReadRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::H5CircumFluenceActivityReadRequest,
            pb::H5CircumFluenceActivityReadResponse,
            events_misc::handle_h5_circum_fluence_activity_read_request
        ),
        pb::H5ViewActivityReadRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::H5ViewActivityReadRequest,
            pb::H5ViewActivityReadResponse,
            events_misc::handle_h5_view_activity_read_request
        ),
        pb::ArtemisNodeUpdateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ArtemisNodeUpdateRequest,
            pb::ArtemisNodeUpdateResponse,
            events_misc::handle_artemis_node_update_request
        ),
        pb::AvignonRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AvignonRewardRequest,
            pb::AvignonRewardResponse,
            events_misc::handle_avignon_reward_request
        ),
        pb::BlackCoastThemeActiveRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BlackCoastThemeActiveRewardRequest,
            pb::BlackCoastThemeActiveRewardResponse,
            events_misc::handle_black_coast_theme_active_reward_request
        ),
        pb::BlackCoastThemeTaskRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BlackCoastThemeTaskRewardRequest,
            pb::BlackCoastThemeTaskRewardResponse,
            events_misc::handle_black_coast_theme_task_reward_request
        ),
        pb::BlackSwordChallengeInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BlackSwordChallengeInfoRequest,
            pb::BlackSwordChallengeInfoResponse,
            events_misc::handle_black_sword_challenge_info_request
        ),
        pb::BlackSwordChallengeMarkItemPanelInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BlackSwordChallengeMarkItemPanelInfoRequest,
            pb::BlackSwordChallengeMarkItemPanelInfoResponse,
            events_misc::handle_black_sword_challenge_mark_item_panel_info_request
        ),
        pb::BrochureRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BrochureRewardRequest,
            pb::BrochureRewardResponse,
            events_misc::handle_brochure_reward_request
        ),
        pb::BuildingUnLockRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BuildingUnLockRequest,
            pb::BuildingUnLockResponse,
            events_misc::handle_building_un_lock_request
        ),
        pb::BuildingLevelUpRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BuildingLevelUpRequest,
            pb::BuildingLevelUpResponse,
            events_misc::handle_building_level_up_request
        ),
        pb::CertificateLevelRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::CertificateLevelRewardRequest,
            pb::CertificateLevelRewardResponse,
            events_misc::handle_certificate_level_reward_request
        ),
        pb::CorniceChallengeTransRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::CorniceChallengeTransRequest,
            pb::CorniceChallengeTransResponse,
            events_misc::handle_cornice_challenge_trans_request
        ),
        pb::MulCorniceMeetingRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MulCorniceMeetingRewardRequest,
            pb::MulCorniceMeetingRewardResponse,
            events_misc::handle_mul_cornice_meeting_reward_request
        ),
        pb::DarkCoastInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DarkCoastInfoRequest,
            pb::DarkCoastInfoResponse,
            events_misc::handle_dark_coast_info_request
        ),
        pb::DragonPoolConfRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DragonPoolConfRequest,
            pb::DragonPoolConfResponse,
            events_misc::handle_dragon_pool_conf_request
        ),
        pb::EdgeRunnerFunctionPreUnlockRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EdgeRunnerFunctionPreUnlockRequest,
            pb::EdgeRunnerFunctionPreUnlockResponse,
            events_misc::handle_edge_runner_function_pre_unlock_request
        ),
        pb::EdgeRunnerScoreRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EdgeRunnerScoreRewardRequest,
            pb::EdgeRunnerScoreRewardResponse,
            events_misc::handle_edge_runner_score_reward_request
        ),
        pb::EdgeRunnerTaskRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EdgeRunnerTaskRewardRequest,
            pb::EdgeRunnerTaskRewardResponse,
            events_misc::handle_edge_runner_task_reward_request
        ),
        pb::FeiXuePreheatInfoRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FeiXuePreheatInfoRewardRequest,
            pb::FeiXuePreheatInfoRewardResponse,
            events_misc::handle_fei_xue_preheat_info_reward_request
        ),
        pb::FlagChallengeRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FlagChallengeRewardRequest,
            pb::FlagChallengeRewardResponse,
            events_misc::handle_flag_challenge_reward_request
        ),
        pb::FunPlayChallengeAwardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FunPlayChallengeAwardRequest,
            pb::FunPlayChallengeAwardResponse,
            events_misc::handle_fun_play_challenge_award_request
        ),
        pb::FuncShowRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FuncShowRequest,
            pb::FuncShowResponse,
            events_misc::handle_func_show_request
        ),
        pb::GatherActivityRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::GatherActivityRewardRequest,
            pb::GatherActivityRewardResponse,
            events_misc::handle_gather_activity_reward_request
        ),
        pb::GiantDeliveryItemRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::GiantDeliveryItemRequest,
            pb::GiantDeliveryItemResponse,
            events_misc::handle_giant_delivery_item_request
        ),
        pb::GiantMaxLevelDeliveryItemRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::GiantMaxLevelDeliveryItemRequest,
            pb::GiantMaxLevelDeliveryItemResponse,
            events_misc::handle_giant_max_level_delivery_item_request
        ),
        pb::GiantStableValueActivityRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::GiantStableValueActivityRequest,
            pb::GiantStableValueActivityResponse,
            events_misc::handle_giant_stable_value_activity_request
        ),
        pb::GivebackRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::GivebackRewardRequest,
            pb::GivebackRewardResponse,
            events_misc::handle_giveback_reward_request
        ),
        pb::LinkageCheckInRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LinkageCheckInRewardRequest,
            pb::LinkageCheckInRewardResponse,
            events_misc::handle_linkage_check_in_reward_request
        ),
        pb::LinkageRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LinkageRewardRequest,
            pb::LinkageRewardResponse,
            events_misc::handle_linkage_reward_request
        ),
        pb::LongShanScoreRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LongShanScoreRewardRequest,
            pb::LongShanScoreRewardResponse,
            events_misc::handle_long_shan_score_reward_request
        ),
        pb::LongShanTaskRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LongShanTaskRewardRequest,
            pb::LongShanTaskRewardResponse,
            events_misc::handle_long_shan_task_reward_request
        ),
        pb::MoonPhaseRandomRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MoonPhaseRandomRequest,
            pb::MoonPhaseRandomResponse,
            events_misc::handle_moon_phase_random_request
        ),
        pb::MoonPhaseRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MoonPhaseRewardRequest,
            pb::MoonPhaseRewardResponse,
            events_misc::handle_moon_phase_reward_request
        ),
        pb::TraceMoonPhaseRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TraceMoonPhaseRewardRequest,
            pb::TraceMoonPhaseRewardResponse,
            events_misc::handle_trace_moon_phase_reward_request
        ),
        pb::OccupyRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::OccupyRewardRequest,
            pb::OccupyRewardResponse,
            events_misc::handle_occupy_reward_request
        ),
        pb::QingXiaoTaskRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::QingXiaoTaskRewardRequest,
            pb::QingXiaoTaskRewardResponse,
            events_misc::handle_qing_xiao_task_reward_request
        ),
        pb::ScheduleRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ScheduleRewardRequest,
            pb::ScheduleRewardResponse,
            events_misc::handle_schedule_reward_request
        ),
        pb::ThemePersonalScoreRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ThemePersonalScoreRequest,
            pb::ThemePersonalScoreResponse,
            events_misc::handle_theme_personal_score_request
        ),
        pb::ThemeWorldScoreRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ThemeWorldScoreRequest,
            pb::ThemeWorldScoreResponse,
            events_misc::handle_theme_world_score_request
        ),
        pb::TimePointActivityRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TimePointActivityRewardRequest,
            pb::TimePointActivityRewardResponse,
            events_misc::handle_time_point_activity_reward_request
        ),
        pb::TimePointActivityBatchRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TimePointActivityBatchRewardRequest,
            pb::TimePointActivityBatchRewardResponse,
            events_misc::handle_time_point_activity_batch_reward_request
        ),
        pb::TrialRoleRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TrialRoleRewardRequest,
            pb::TrialRoleRewardResponse,
            events_misc::handle_trial_role_reward_request
        ),
        pb::TrialRoleSkinRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TrialRoleSkinRewardRequest,
            pb::TrialRoleSkinRewardResponse,
            events_misc::handle_trial_role_skin_reward_request
        ),
        pb::AreaTerminalPinOperationRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AreaTerminalPinOperationRequest,
            pb::AreaTerminalPinOperationResponse,
            events_misc::handle_area_terminal_pin_operation_request
        ),
        pb::BathChangeCameraRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BathChangeCameraRequest,
            pb::BathChangeCameraResponse,
            events_misc::handle_bath_change_camera_request
        ),
        pb::BathPhotoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BathPhotoRequest,
            pb::BathPhotoResponse,
            events_misc::handle_bath_photo_request
        ),
        pb::PlaceItemOnBoardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PlaceItemOnBoardRequest,
            pb::PlaceItemOnBoardResponse,
            events_misc::handle_place_item_on_board_request
        ),
        pb::CommitBoardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::CommitBoardRequest,
            pb::CommitBoardResponse,
            events_misc::handle_commit_board_request
        ),
        pb::ConstructTrapRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ConstructTrapRequest,
            pb::ConstructTrapResponse,
            events_misc::handle_construct_trap_request
        ),
        pb::DeconstructTrapRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DeconstructTrapRequest,
            pb::DeconstructTrapResponse,
            events_misc::handle_deconstruct_trap_request
        ),
        pb::DropCatchInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DropCatchInfoRequest,
            pb::DropCatchInfoResponse,
            events_misc::handle_drop_catch_info_request
        ),
        pb::DropCatchLevelFinishRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DropCatchLevelFinishRequest,
            pb::DropCatchLevelFinishResponse,
            events_misc::handle_drop_catch_level_finish_request
        ),
        pb::DropCatchLevelRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DropCatchLevelRewardRequest,
            pb::DropCatchLevelRewardResponse,
            events_misc::handle_drop_catch_level_reward_request
        ),
        pb::EncircleChallengeStartRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EncircleChallengeStartRequest,
            pb::EncircleChallengeStartResponse,
            events_misc::handle_encircle_challenge_start_request
        ),
        pb::EncircleChallengeCompleteRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EncircleChallengeCompleteRequest,
            pb::EncircleChallengeCompleteResponse,
            events_misc::handle_encircle_challenge_complete_request
        ),
        pb::ExhibitionItemSaveRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ExhibitionItemSaveRequest,
            pb::ExhibitionItemSaveResponse,
            events_misc::handle_exhibition_item_save_request
        ),
        pb::FarmGoldLevelDiffRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FarmGoldLevelDiffRequest,
            pb::FarmGoldLevelDiffResponse,
            events_misc::handle_farm_gold_level_diff_request
        ),
        pb::FarmGoldLevelPlayRepeatedRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FarmGoldLevelPlayRepeatedRequest,
            pb::FarmGoldLevelPlayRepeatedResponse,
            events_misc::handle_farm_gold_level_play_repeated_request
        ),
        pb::FarmGoldPointRepeatedRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FarmGoldPointRepeatedRequest,
            pb::FarmGoldPointRepeatedResponse,
            events_misc::handle_farm_gold_point_repeated_request
        ),
        pb::FarmGoldResultRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FarmGoldResultRequest,
            pb::FarmGoldResultResponse,
            events_misc::handle_farm_gold_result_request
        ),
        pb::FixToolRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FixToolRequest,
            pb::FixToolResponse,
            events_misc::handle_fix_tool_request
        ),
        pb::FlowerPollutionRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FlowerPollutionRequest,
            pb::FlowerPollutionResponse,
            events_misc::handle_flower_pollution_request
        ),
        pb::FlowerPollutionSplinePush::MSG_ID => push_noev!(
            state,
            msg_id,
            payload,
            pb::FlowerPollutionSplinePush,
            events_misc::handle_flower_pollution_spline_push
        ),
        pb::FoundationRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FoundationRequest,
            pb::FoundationResponse,
            events_misc::handle_foundation_request
        ),
        pb::FoundationCoreOperateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FoundationCoreOperateRequest,
            pb::FoundationCoreOperateResponse,
            events_misc::handle_foundation_core_operate_request
        ),
        pb::PullingFoundationRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PullingFoundationRequest,
            pb::PullingFoundationResponse,
            events_misc::handle_pulling_foundation_request
        ),
        pb::GolemCrackUpdateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::GolemCrackUpdateRequest,
            pb::GolemCrackUpdateResponse,
            events_misc::handle_golem_crack_update_request
        ),
        pb::HarvestActivityLevelDiffRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::HarvestActivityLevelDiffRequest,
            pb::HarvestActivityLevelDiffResponse,
            events_misc::handle_harvest_activity_level_diff_request
        ),
        pb::HarvestActivityLevelRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::HarvestActivityLevelRequest,
            pb::HarvestActivityLevelResponse,
            events_misc::handle_harvest_activity_level_request
        ),
        pb::HarvestActivityPointRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::HarvestActivityPointRequest,
            pb::HarvestActivityPointResponse,
            events_misc::handle_harvest_activity_point_request
        ),
        pb::HarvestActivityResultRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::HarvestActivityResultRequest,
            pb::HarvestActivityResultResponse,
            events_misc::handle_harvest_activity_result_request
        ),
        pb::InspirationUnlockChoiceRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::InspirationUnlockChoiceRequest,
            pb::InspirationUnlockChoiceResponse,
            events_misc::handle_inspiration_unlock_choice_request
        ),
        pb::InvestRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::InvestRequest,
            pb::InvestResponse,
            events_misc::handle_invest_request
        ),
        pb::LifePointChallengeStartRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LifePointChallengeStartRequest,
            pb::LifePointChallengeStartResponse,
            events_misc::handle_life_point_challenge_start_request
        ),
        pb::LineCrossChallengeStartRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LineCrossChallengeStartRequest,
            pb::LineCrossChallengeStartResponse,
            events_misc::handle_line_cross_challenge_start_request
        ),
        pb::MatchThreeStarRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MatchThreeStarRequest,
            pb::MatchThreeStarResponse,
            events_misc::handle_match_three_star_request
        ),
        pb::MatchThreeEndRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MatchThreeEndRequest,
            pb::MatchThreeEndResponse,
            events_misc::handle_match_three_end_request
        ),
        pb::MiniWriteLetterSaveContentRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MiniWriteLetterSaveContentRequest,
            pb::MiniWriteLetterSaveContentResponse,
            events_misc::handle_mini_write_letter_save_content_request
        ),
        pb::MixDrinkRoleInviteRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MixDrinkRoleInviteRequest,
            pb::MixDrinkRoleInviteResponse,
            events_misc::handle_mix_drink_role_invite_request
        ),
        pb::MixDrinkRoleRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MixDrinkRoleRewardRequest,
            pb::MixDrinkRoleRewardResponse,
            events_misc::handle_mix_drink_role_reward_request
        ),
        pb::MixDrinkSettleRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MixDrinkSettleRequest,
            pb::MixDrinkSettleResponse,
            events_misc::handle_mix_drink_settle_request
        ),
        pb::MowTowerLevelsRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MowTowerLevelsRewardRequest,
            pb::MowTowerLevelsRewardResponse,
            events_misc::handle_mow_tower_levels_reward_request
        ),
        pb::MowTowerSettlementRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MowTowerSettlementRequest,
            pb::MowTowerSettlementResponse,
            events_misc::handle_mow_tower_settlement_request
        ),
        pb::OpenSystemBoardResultRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::OpenSystemBoardResultRequest,
            pb::OpenSystemBoardResultResponse,
            events_misc::handle_open_system_board_result_request
        ),
        pb::ParkourChallengeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ParkourChallengeRequest,
            pb::ParkourChallengeResponse,
            events_misc::handle_parkour_challenge_request
        ),
        pb::ParkourChallengeTakeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ParkourChallengeTakeRequest,
            pb::ParkourChallengeTakeResponse,
            events_misc::handle_parkour_challenge_take_request
        ),
        pb::ParkourChallengeTransRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ParkourChallengeTransRequest,
            pb::ParkourChallengeTransResponse,
            events_misc::handle_parkour_challenge_trans_request
        ),
        pb::PhotoFightSettleRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhotoFightSettleRequest,
            pb::PhotoFightSettleResponse,
            events_misc::handle_photo_fight_settle_request
        ),
        pb::PhotoFightTaskRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhotoFightTaskRewardRequest,
            pb::PhotoFightTaskRewardResponse,
            events_misc::handle_photo_fight_task_reward_request
        ),
        pb::PhotoMemoryRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PhotoMemoryRewardRequest,
            pb::PhotoMemoryRewardResponse,
            events_misc::handle_photo_memory_reward_request
        ),
        pb::RenjuExitMatchedActionRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RenjuExitMatchedActionRequest,
            pb::RenjuExitMatchedActionResponse,
            events_misc::handle_renju_exit_matched_action_request
        ),
        pb::RenjuResetRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RenjuResetRequest,
            pb::RenjuResetResponse,
            events_misc::handle_renju_reset_request
        ),
        pb::RiskHarvestInstRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RiskHarvestInstRewardRequest,
            pb::RiskHarvestInstRewardResponse,
            events_misc::handle_risk_harvest_inst_reward_request
        ),
        pb::RiskHarvestScoreRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RiskHarvestScoreRewardRequest,
            pb::RiskHarvestScoreRewardResponse,
            events_misc::handle_risk_harvest_score_reward_request
        ),
        pb::RiskHarvestStarRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RiskHarvestStarRewardRequest,
            pb::RiskHarvestStarRewardResponse,
            events_misc::handle_risk_harvest_star_reward_request
        ),
        pb::RiskHarvestSettleRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RiskHarvestSettleRequest,
            pb::RiskHarvestSettleResponse,
            events_misc::handle_risk_harvest_settle_request
        ),
        pb::SunSpiritActionOperationEndPush::MSG_ID => push_noev!(
            state,
            msg_id,
            payload,
            pb::SunSpiritActionOperationEndPush,
            events_misc::handle_sun_spirit_action_operation_end_push
        ),
        pb::TetrisFinishRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TetrisFinishRequest,
            pb::TetrisFinishResponse,
            events_misc::handle_tetris_finish_request
        ),
        pb::TetrisUpdateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TetrisUpdateRequest,
            pb::TetrisUpdateResponse,
            events_misc::handle_tetris_update_request
        ),
        pb::VacuumCleanerRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::VacuumCleanerRequest,
            pb::VacuumCleanerResponse,
            events_misc::handle_vacuum_cleaner_request
        ),
        pb::ZitherPlayNotePush::MSG_ID => push_noev!(
            state,
            msg_id,
            payload,
            pb::ZitherPlayNotePush,
            events_misc::handle_zither_play_note_push
        ),
        pb::ZitherPlayStateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ZitherPlayStateRequest,
            pb::ZitherPlayStateResponse,
            events_misc::handle_zither_play_state_request
        ),
        pb::ChangeEntityStateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ChangeEntityStateRequest,
            pb::ChangeEntityStateResponse,
            world_puzzle::handle_change_entity_state_request
        ),
        pb::ChangeEntityRoleRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ChangeEntityRoleRequest,
            pb::ChangeEntityRoleResponse,
            world_puzzle::handle_change_entity_role_request
        ),
        pb::EntityBlackboardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EntityBlackboardRequest,
            pb::EntityBlackboardResponse,
            world_puzzle::handle_entity_blackboard_request
        ),
        pb::UpdateSceneItemBlackboardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::UpdateSceneItemBlackboardRequest,
            pb::UpdateSceneItemBlackboardResponse,
            world_puzzle::handle_update_scene_item_blackboard_request
        ),
        pb::EntityAdsorbRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EntityAdsorbRequest,
            pb::EntityAdsorbResponse,
            world_puzzle::handle_entity_adsorb_request
        ),
        pb::EntityBattleInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EntityBattleInfoRequest,
            pb::EntityBattleInfoResponse,
            world_puzzle::handle_entity_battle_info_request
        ),
        pb::EntityBeamReceiveRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EntityBeamReceiveRequest,
            pb::EntityBeamReceiveResponse,
            world_puzzle::handle_entity_beam_receive_request
        ),
        pb::EntityCameraAlertRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EntityCameraAlertRequest,
            pb::EntityCameraAlertResponse,
            world_puzzle::handle_entity_camera_alert_request
        ),
        pb::EntityPatrolAddTagRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EntityPatrolAddTagRequest,
            pb::EntityPatrolAddTagResponse,
            world_puzzle::handle_entity_patrol_add_tag_request
        ),
        pb::EntityPatrolChangeDirRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EntityPatrolChangeDirRequest,
            pb::EntityPatrolChangeDirResponse,
            world_puzzle::handle_entity_patrol_change_dir_request
        ),
        pb::EntityRandomInteractRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EntityRandomInteractRequest,
            pb::EntityRandomInteractResponse,
            world_puzzle::handle_entity_random_interact_request
        ),
        pb::EntitySendEventRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EntitySendEventRequest,
            pb::EntitySendEventResponse,
            world_puzzle::handle_entity_send_event_request
        ),
        pb::EntityWalkingPatternFinishRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EntityWalkingPatternFinishRequest,
            pb::EntityWalkingPatternFinishResponse,
            world_puzzle::handle_entity_walking_pattern_finish_request
        ),
        pb::SubmitWalkingPatternPointRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SubmitWalkingPatternPointRequest,
            pb::SubmitWalkingPatternPointResponse,
            world_puzzle::handle_submit_walking_pattern_point_request
        ),
        pb::MonsterPatrolPointReachedRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MonsterPatrolPointReachedRequest,
            pb::MonsterPatrolPointReachedResponse,
            world_puzzle::handle_monster_patrol_point_reached_request
        ),
        pb::VfxNpcPatrolFinishRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::VfxNpcPatrolFinishRequest,
            pb::VfxNpcPatrolFinishResponse,
            world_puzzle::handle_vfx_npc_patrol_finish_request
        ),
        pb::NpcTraceFailedRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::NpcTraceFailedRequest,
            pb::NpcTraceFailedResponse,
            world_puzzle::handle_npc_trace_failed_request
        ),
        pb::QuestNpcMoveOverRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::QuestNpcMoveOverRequest,
            pb::QuestNpcMoveOverResponse,
            world_puzzle::handle_quest_npc_move_over_request
        ),
        pb::ProgressBarFinishRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ProgressBarFinishRequest,
            pb::ProgressBarFinishResponse,
            world_puzzle::handle_progress_bar_finish_request
        ),
        pb::SetInitTagRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SetInitTagRequest,
            pb::SetInitTagResponse,
            world_puzzle::handle_set_init_tag_request
        ),
        pb::SetBaoziStateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SetBaoziStateRequest,
            pb::SetBaoziStateResponse,
            world_puzzle::handle_set_baozi_state_request
        ),
        pb::SetFanNumberOfTurnsRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SetFanNumberOfTurnsRequest,
            pb::SetFanNumberOfTurnsResponse,
            world_puzzle::handle_set_fan_number_of_turns_request
        ),
        pb::SetFanStateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SetFanStateRequest,
            pb::SetFanStateResponse,
            world_puzzle::handle_set_fan_state_request
        ),
        pb::ElevatorStateChangeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ElevatorStateChangeRequest,
            pb::ElevatorStateChangeResponse,
            world_puzzle::handle_elevator_state_change_request
        ),
        pb::HardnessModeChangedRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::HardnessModeChangedRequest,
            pb::HardnessModeChangedResponse,
            world_puzzle::handle_hardness_mode_changed_request
        ),
        pb::SceneItemStateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SceneItemStateRequest,
            pb::SceneItemStateResponse,
            world_puzzle::handle_scene_item_state_request
        ),
        pb::SceneItemMoveSplineRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SceneItemMoveSplineRequest,
            pb::SceneItemMoveSplineResponse,
            world_puzzle::handle_scene_item_move_spline_request
        ),
        pb::SceneItemMoveTargetRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SceneItemMoveTargetRequest,
            pb::SceneItemMoveTargetResponse,
            world_puzzle::handle_scene_item_move_target_request
        ),
        pb::SceneItemSequenceFrameStartRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SceneItemSequenceFrameStartRequest,
            pb::SceneItemSequenceFrameStartResponse,
            world_puzzle::handle_scene_item_sequence_frame_start_request
        ),
        pb::SceneItemSequenceFrameEndRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SceneItemSequenceFrameEndRequest,
            pb::SceneItemSequenceFrameEndResponse,
            world_puzzle::handle_scene_item_sequence_frame_end_request
        ),
        pb::SceneItemStopRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SceneItemStopRequest,
            pb::SceneItemStopResponse,
            world_puzzle::handle_scene_item_stop_request
        ),
        pb::MoveSceneItemResetPositionRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MoveSceneItemResetPositionRequest,
            pb::MoveSceneItemResetPositionResponse,
            world_puzzle::handle_move_scene_item_reset_position_request
        ),
        pb::MovePlacementRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MovePlacementRequest,
            pb::MovePlacementResponse,
            world_puzzle::handle_move_placement_request
        ),
        pb::SplineMoveRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SplineMoveRequest,
            pb::SplineMoveResponse,
            world_puzzle::handle_spline_move_request
        ),
        pb::SplinePositionUpdateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SplinePositionUpdateRequest,
            pb::SplinePositionUpdateResponse,
            world_puzzle::handle_spline_position_update_request
        ),
        pb::StopMoveSplineRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::StopMoveSplineRequest,
            pb::StopMoveSplineResponse,
            world_puzzle::handle_stop_move_spline_request
        ),
        pb::GravityFlipInteractRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::GravityFlipInteractRequest,
            pb::GravityFlipInteractResponse,
            world_puzzle::handle_gravity_flip_interact_request
        ),
        pb::GravityHookLockPointChangeGravityRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::GravityHookLockPointChangeGravityRequest,
            pb::GravityHookLockPointChangeGravityResponse,
            world_puzzle::handle_gravity_hook_lock_point_change_gravity_request
        ),
        pb::HookLockPointRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::HookLockPointRequest,
            pb::HookLockPointResponse,
            world_puzzle::handle_hook_lock_point_request
        ),
        pb::HookLockPointFinishRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::HookLockPointFinishRequest,
            pb::HookLockPointFinishResponse,
            world_puzzle::handle_hook_lock_point_finish_request
        ),
        pb::HookLockPointExitRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::HookLockPointExitRequest,
            pb::HookLockPointExitResponse,
            world_puzzle::handle_hook_lock_point_exit_request
        ),
        pb::HookLockBatchCollectRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::HookLockBatchCollectRequest,
            pb::HookLockBatchCollectResponse,
            world_puzzle::handle_hook_lock_batch_collect_request
        ),
        pb::ConnectorCanConnectRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ConnectorCanConnectRequest,
            pb::ConnectorCanConnectResponse,
            world_puzzle::handle_connector_can_connect_request
        ),
        pb::ConnectorConnectEntityRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ConnectorConnectEntityRequest,
            pb::ConnectorConnectEntityResponse,
            world_puzzle::handle_connector_connect_entity_request
        ),
        pb::BeControlledRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BeControlledRequest,
            pb::BeControlledResponse,
            world_puzzle::handle_be_controlled_request
        ),
        pb::BeControlledShowRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BeControlledShowRequest,
            pb::BeControlledShowResponse,
            world_puzzle::handle_be_controlled_show_request
        ),
        pb::BeControlledEntityHoverRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BeControlledEntityHoverRequest,
            pb::BeControlledEntityHoverResponse,
            world_puzzle::handle_be_controlled_entity_hover_request
        ),
        pb::EntityTimeFlowPush::MSG_ID => push_noev!(
            state,
            msg_id,
            payload,
            pb::EntityTimeFlowPush,
            world_puzzle::handle_entity_time_flow_push
        ),
        pb::SceneModeChangeFinishRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SceneModeChangeFinishRequest,
            pb::SceneModeChangeFinishResponse,
            world_puzzle::handle_scene_mode_change_finish_request
        ),
        pb::BeControlledThrowRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BeControlledThrowRequest,
            pb::BeControlledThrowResponse,
            world_puzzle::handle_be_controlled_throw_request
        ),
        pb::BeControlledDestroyRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BeControlledDestroyRequest,
            pb::BeControlledDestroyResponse,
            world_puzzle::handle_be_controlled_destroy_request
        ),
        pb::CombineEntitiesRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::CombineEntitiesRequest,
            pb::CombineEntitiesResponse,
            world_puzzle::handle_combine_entities_request
        ),
        pb::DissolveCombineRelationRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DissolveCombineRelationRequest,
            pb::DissolveCombineRelationResponse,
            world_puzzle::handle_dissolve_combine_relation_request
        ),
        pb::AttachEntityRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AttachEntityRequest,
            pb::AttachEntityResponse,
            world_puzzle::handle_attach_entity_request
        ),
        pb::HoldHandRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::HoldHandRequest,
            pb::HoldHandResponse,
            world_puzzle::handle_hold_hand_request
        ),
        pb::SitChairRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SitChairRequest,
            pb::SitChairResponse,
            world_puzzle::handle_sit_chair_request
        ),
        pb::SneakRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SneakRequest,
            pb::SneakResponse,
            world_puzzle::handle_sneak_request
        ),
        pb::SneakFinishRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SneakFinishRequest,
            pb::SneakFinishResponse,
            world_puzzle::handle_sneak_finish_request
        ),
        pb::SneakTimeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SneakTimeRequest,
            pb::SneakTimeResponse,
            world_puzzle::handle_sneak_time_request
        ),
        pb::TimelineTraceStartRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TimelineTraceStartRequest,
            pb::TimelineTraceStartResponse,
            world_puzzle::handle_timeline_trace_start_request
        ),
        pb::TimelineTraceControlRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TimelineTraceControlRequest,
            pb::TimelineTraceControlResponse,
            world_puzzle::handle_timeline_trace_control_request
        ),
        pb::TimelineTraceExitRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::TimelineTraceExitRequest,
            pb::TimelineTraceExitResponse,
            world_puzzle::handle_timeline_trace_exit_request
        ),
        pb::RemoteBatchCollectRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RemoteBatchCollectRequest,
            pb::RemoteBatchCollectResponse,
            world_puzzle::handle_remote_batch_collect_request
        ),
        pb::CaptureEntityRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::CaptureEntityRequest,
            pb::CaptureEntityResponse,
            world_puzzle::handle_capture_entity_request
        ),
        pb::CaptureBatchEntityRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::CaptureBatchEntityRequest,
            pb::CaptureBatchEntityResponse,
            world_puzzle::handle_capture_batch_entity_request
        ),
        pb::PickUpDropItemRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PickUpDropItemRequest,
            pb::PickUpDropItemResponse,
            world_puzzle::handle_pick_up_drop_item_request
        ),
        pb::PilotThrowOperationRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PilotThrowOperationRequest,
            pb::PilotThrowOperationResponse,
            world_puzzle::handle_pilot_throw_operation_request
        ),
        pb::CommonOrganResetTimeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::CommonOrganResetTimeRequest,
            pb::CommonOrganResetTimeResponse,
            world_puzzle::handle_common_organ_reset_time_request
        ),
        pb::Summon2Request::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::Summon2Request,
            pb::Summon2Response,
            world_puzzle::handle_summon2_request
        ),
        pb::SummonRandomRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SummonRandomRequest,
            pb::SummonRandomResponse,
            world_puzzle::handle_summon_random_request
        ),
        pb::PlayerEnterPollutionRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PlayerEnterPollutionRequest,
            pb::PlayerEnterPollutionResponse,
            world_puzzle::handle_player_enter_pollution_request
        ),
        pb::EnrichmentAreaRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::EnrichmentAreaRequest,
            pb::EnrichmentAreaResponse,
            world_puzzle::handle_enrichment_area_request
        ),
        pb::AddDecalRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AddDecalRequest,
            pb::AddDecalResponse,
            world_puzzle::handle_add_decal_request
        ),
        pb::BatchMaterialReplaceRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BatchMaterialReplaceRequest,
            pb::BatchMaterialReplaceResponse,
            world_puzzle::handle_batch_material_replace_request
        ),
        pb::MaterialReplaceRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MaterialReplaceRequest,
            pb::MaterialReplaceResponse,
            world_puzzle::handle_material_replace_request
        ),
        pb::FurnitureDiySaveRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FurnitureDiySaveRequest,
            pb::FurnitureDiySaveResponse,
            world_puzzle::handle_furniture_diy_save_request
        ),
        pb::AcceptEntrustRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AcceptEntrustRequest,
            pb::AcceptEntrustResponse,
            world_puzzle::handle_accept_entrust_request
        ),
        pb::UiGamePlayRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::UiGamePlayRequest,
            pb::UiGamePlayResponse,
            world_puzzle::handle_ui_game_play_request
        ),
        pb::WellStreamingFinisRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::WellStreamingFinisRequest,
            pb::WellStreamingFinisResponse,
            world_puzzle::handle_well_streaming_finis_request
        ),
        pb::ReadDisplayInfoRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ReadDisplayInfoRequest,
            pb::ReadDisplayInfoResponse,
            world_puzzle::handle_read_display_info_request
        ),
        pb::LoopTeleportVolumeActionRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LoopTeleportVolumeActionRequest,
            pb::LoopTeleportVolumeActionResponse,
            world_puzzle::handle_loop_teleport_volume_action_request
        ),
        pb::BtForcedOccupationRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::BtForcedOccupationRequest,
            pb::BtForcedOccupationResponse,
            world_puzzle::handle_bt_forced_occupation_request
        ),
        pb::ActionOperationScreenSuccessRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ActionOperationScreenSuccessRequest,
            pb::ActionOperationScreenSuccessResponse,
            world_puzzle::handle_action_operation_screen_success_request
        ),
        pb::ActionResetPosPrePerformanceEndRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ActionResetPosPrePerformanceEndRequest,
            pb::ActionResetPosPrePerformanceEndResponse,
            world_puzzle::handle_action_reset_pos_pre_performance_end_request
        ),
        pb::AccessPasserbyNpcSpawnerRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AccessPasserbyNpcSpawnerRequest,
            pb::AccessPasserbyNpcSpawnerResponse,
            world_puzzle::handle_access_passerby_npc_spawner_request
        ),
        pb::VehiclePlatformGetOnRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::VehiclePlatformGetOnRequest,
            pb::VehiclePlatformGetOnResponse,
            world_puzzle::handle_vehicle_platform_get_on_request
        ),
        pb::WeatherControlInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::WeatherControlInfoRequest,
            pb::WeatherControlInfoResponse,
            world_puzzle::handle_weather_control_info_request
        ),
        pb::WeatherControlRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::WeatherControlRequest,
            pb::WeatherControlResponse,
            world_puzzle::handle_weather_control_request
        ),
        pb::WeatherSwitchInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::WeatherSwitchInfoRequest,
            pb::WeatherSwitchInfoResponse,
            world_puzzle::handle_weather_switch_info_request
        ),
        pb::SeamlessTeleportRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SeamlessTeleportRequest,
            pb::SeamlessTeleportResponse,
            world_puzzle::handle_seamless_teleport_request
        ),
        pb::SeamlessTeleportVehicleRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SeamlessTeleportVehicleRequest,
            pb::SeamlessTeleportVehicleResponse,
            world_puzzle::handle_seamless_teleport_vehicle_request
        ),
        pb::PassPortalRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PassPortalRequest,
            pb::PassPortalResponse,
            world_puzzle::handle_pass_portal_request
        ),
        pb::AwakeDynamicPortalRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AwakeDynamicPortalRequest,
            pb::AwakeDynamicPortalResponse,
            world_puzzle::handle_awake_dynamic_portal_request
        ),
        pb::InstEntityTeleportRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::InstEntityTeleportRequest,
            pb::InstEntityTeleportResponse,
            world_puzzle::handle_inst_entity_teleport_request
        ),
        pb::FlowTeleportToPlotPosRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FlowTeleportToPlotPosRequest,
            pb::FlowTeleportToPlotPosResponse,
            world_puzzle::handle_flow_teleport_to_plot_pos_request
        ),
        pb::QuestFastReturnPosRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::QuestFastReturnPosRequest,
            pb::QuestFastReturnPosResponse,
            world_puzzle::handle_quest_fast_return_pos_request
        ),
        pb::FastReturnDungeonRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FastReturnDungeonRequest,
            pb::FastReturnDungeonResponse,
            world_puzzle::handle_fast_return_dungeon_request
        ),
        pb::UnOpenedAreaPullbackRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::UnOpenedAreaPullbackRequest,
            pb::UnOpenedAreaPullbackResponse,
            world_puzzle::handle_un_opened_area_pullback_request
        ),
        pb::ReviveRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ReviveRequest,
            pb::ReviveResponse,
            world_puzzle::handle_revive_request
        ),
        pb::LowMemoryChangeSceneFinishRequest::MSG_ID => req_ev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LowMemoryChangeSceneFinishRequest,
            pb::LowMemoryChangeSceneFinishResponse,
            world_puzzle::handle_low_memory_change_scene_finish_request
        ),
        pb::InstEntranceDetailRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::InstEntranceDetailRequest,
            pb::InstEntranceDetailResponse,
            world_puzzle::handle_inst_entrance_detail_request
        ),
        pb::InviteRechallengeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::InviteRechallengeRequest,
            pb::InviteRechallengeResponse,
            world_puzzle::handle_invite_rechallenge_request
        ),
        pb::ReceiveRechallengeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ReceiveRechallengeRequest,
            pb::ReceiveRechallengeResponse,
            world_puzzle::handle_receive_rechallenge_request
        ),
        pb::LevelSequenceFrameEventActionRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LevelSequenceFrameEventActionRequest,
            pb::LevelSequenceFrameEventActionResponse,
            world_puzzle::handle_level_sequence_frame_event_action_request
        ),
        pb::LoadingLevelAreaRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LoadingLevelAreaRequest,
            pb::LoadingLevelAreaResponse,
            world_puzzle::handle_loading_level_area_request
        ),
        pb::LstBattleRecordRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LstBattleRecordRequest,
            pb::LstBattleRecordResponse,
            world_puzzle::handle_lst_battle_record_request
        ),
        pb::QuestReadyForGiveUpRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::QuestReadyForGiveUpRequest,
            pb::QuestReadyForGiveUpResponse,
            world_puzzle::handle_quest_ready_for_give_up_request
        ),
        pb::UpdateFightRoleRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::UpdateFightRoleRequest,
            pb::UpdateFightRoleResponse,
            world_puzzle::handle_update_fight_role_request
        ),
        pb::ReorderFormationRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ReorderFormationRequest,
            pb::ReorderFormationResponse,
            world_puzzle::handle_reorder_formation_request
        ),
        pb::SceneGroupDataAndCurFormationEntityRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SceneGroupDataAndCurFormationEntityRequest,
            pb::SceneGroupDataAndCurFormationEntityResponse,
            world_puzzle::handle_scene_group_data_and_cur_formation_entity_request
        ),
        pb::UpdateSceneGroupDataRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::UpdateSceneGroupDataRequest,
            pb::UpdateSceneGroupDataResponse,
            world_puzzle::handle_update_scene_group_data_request
        ),
        pb::SwitchBattleModeRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SwitchBattleModeRequest,
            pb::SwitchBattleModeResponse,
            world_puzzle::handle_switch_battle_mode_request
        ),
        pb::ResonantChainOptionalLimitRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ResonantChainOptionalLimitRequest,
            pb::ResonantChainOptionalLimitResponse,
            world_puzzle::handle_resonant_chain_optional_limit_request
        ),
        pb::FoodProcessRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::FoodProcessRequest,
            pb::FoodProcessResponse,
            world_puzzle::handle_food_process_request
        ),
        pb::DiscardWeaponRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::DiscardWeaponRequest,
            pb::DiscardWeaponResponse,
            world_puzzle::handle_discard_weapon_request
        ),
        pb::DiscardWeaponPush::MSG_ID => push_noev!(
            state,
            msg_id,
            payload,
            pb::DiscardWeaponPush,
            world_puzzle::handle_discard_weapon_push
        ),
        pb::LastBigSceneMiniMapInfoRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::LastBigSceneMiniMapInfoRequest,
            pb::LastBigSceneMiniMapInfoResponse,
            world_puzzle::handle_last_big_scene_mini_map_info_request
        ),
        pb::NavigationFindPathRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::NavigationFindPathRequest,
            pb::NavigationFindPathResponse,
            world_puzzle::handle_navigation_find_path_request
        ),
        pb::AddTreasureBoxSlotRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::AddTreasureBoxSlotRequest,
            pb::AddTreasureBoxSlotResponse,
            world_puzzle::handle_add_treasure_box_slot_request
        ),
        pb::RemoveTreasureBoxSlotRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::RemoveTreasureBoxSlotRequest,
            pb::RemoveTreasureBoxSlotResponse,
            world_puzzle::handle_remove_treasure_box_slot_request
        ),
        pb::ExploreActivityTaskRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ExploreActivityTaskRewardRequest,
            pb::ExploreActivityTaskRewardResponse,
            world_puzzle::handle_explore_activity_task_reward_request
        ),
        pb::MulFishingIllustratedRewardRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MulFishingIllustratedRewardRequest,
            pb::MulFishingIllustratedRewardResponse,
            world_puzzle::handle_mul_fishing_illustrated_reward_request
        ),
        pb::OnlineMotorActivityLevelRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::OnlineMotorActivityLevelRequest,
            pb::OnlineMotorActivityLevelResponse,
            world_puzzle::handle_online_motor_activity_level_request
        ),
        pb::PlayerCreatePsnSessionRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PlayerCreatePsnSessionRequest,
            pb::PlayerCreatePsnSessionResponse,
            world_puzzle::handle_player_create_psn_session_request
        ),
        pb::PlayerPsnInfoGetRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PlayerPsnInfoGetRequest,
            pb::PlayerPsnInfoGetResponse,
            world_puzzle::handle_player_psn_info_get_request
        ),
        pb::PlayerXboxInfoGetRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PlayerXboxInfoGetRequest,
            pb::PlayerXboxInfoGetResponse,
            world_puzzle::handle_player_xbox_info_get_request
        ),
        pb::PsnSettingUpdateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::PsnSettingUpdateRequest,
            pb::PsnSettingUpdateResponse,
            world_puzzle::handle_psn_setting_update_request
        ),
        pb::XboxSettingUpdateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::XboxSettingUpdateRequest,
            pb::XboxSettingUpdateResponse,
            world_puzzle::handle_xbox_setting_update_request
        ),
        pb::CommonTouchUiSettingUpdateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::CommonTouchUiSettingUpdateRequest,
            pb::CommonTouchUiSettingUpdateResponse,
            world_puzzle::handle_common_touch_ui_setting_update_request
        ),
        pb::MobileButtonSettingUpdateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::MobileButtonSettingUpdateRequest,
            pb::MobileButtonSettingUpdateResponse,
            world_puzzle::handle_mobile_button_setting_update_request
        ),
        pb::ChatMutePlayerRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::ChatMutePlayerRequest,
            pb::ChatMutePlayerResponse,
            world_puzzle::handle_chat_mute_player_request
        ),
        pb::EntitySimplyMoveInfoPackagePush::MSG_ID => push_noev!(
            state,
            msg_id,
            payload,
            pb::EntitySimplyMoveInfoPackagePush,
            world_puzzle::handle_entity_simply_move_info_package_push
        ),
        pb::MoveSceneItemPush::MSG_ID => push_noev!(
            state,
            msg_id,
            payload,
            pb::MoveSceneItemPush,
            world_puzzle::handle_move_scene_item_push
        ),
        pb::CharacterMoveToPointEndPush::MSG_ID => push_noev!(
            state,
            msg_id,
            payload,
            pb::CharacterMoveToPointEndPush,
            world_puzzle::handle_character_move_to_point_end_push
        ),
        pb::FollowerChairPresentationPush::MSG_ID => push_noev!(
            state,
            msg_id,
            payload,
            pb::FollowerChairPresentationPush,
            world_puzzle::handle_follower_chair_presentation_push
        ),
        pb::HoldWeaponPush::MSG_ID => push_noev!(
            state,
            msg_id,
            payload,
            pb::HoldWeaponPush,
            world_puzzle::handle_hold_weapon_push
        ),
        pb::PhotographPush::MSG_ID => push_noev!(
            state,
            msg_id,
            payload,
            pb::PhotographPush,
            world_puzzle::handle_photograph_push
        ),
        pb::PlayEntityMontagePush::MSG_ID => push_noev!(
            state,
            msg_id,
            payload,
            pb::PlayEntityMontagePush,
            world_puzzle::handle_play_entity_montage_push
        ),
        pb::RoadNetworkEntityUpdateCurRoadPush::MSG_ID => push_noev!(
            state,
            msg_id,
            payload,
            pb::RoadNetworkEntityUpdateCurRoadPush,
            world_puzzle::handle_road_network_entity_update_cur_road_push
        ),
        pb::TestSpawnTemplateEntityPush::MSG_ID => push_noev!(
            state,
            msg_id,
            payload,
            pb::TestSpawnTemplateEntityPush,
            world_puzzle::handle_test_spawn_template_entity_push
        ),
        pb::PlayerNetStateBackServerPush::MSG_ID => push_noev!(
            state,
            msg_id,
            payload,
            pb::PlayerNetStateBackServerPush,
            world_puzzle::handle_player_net_state_back_server_push
        ),
        pb::ExitGamePush::MSG_ID => push_noev!(
            state,
            msg_id,
            payload,
            pb::ExitGamePush,
            world_puzzle::handle_exit_game_push
        ),
        pb::UDPMovePackagePush::MSG_ID => push_noev!(
            state,
            msg_id,
            payload,
            pb::UDPMovePackagePush,
            world_puzzle::handle_udp_move_package_push
        ),
        pb::InteractiveUpdateRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::InteractiveUpdateRequest,
            pb::InteractiveUpdateResponse,
            world_puzzle::handle_interactive_update_request
        ),
        pb::SceneItemSetPosSenderRequest::MSG_ID => req_noev!(
            state,
            rpc_id,
            msg_id,
            payload,
            pb::SceneItemSetPosSenderRequest,
            pb::SceneItemSetPosSenderResponse,
            world_puzzle::handle_scene_item_set_pos_sender_request
        ),
        _ => Err(DispatchError::HandlerNotFound),
    }
}
