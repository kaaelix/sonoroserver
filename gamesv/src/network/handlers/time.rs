use sonoro_gamesv_logic::scene::now_millis;
use sonoro_proto::enums::ErrorCode;
use sonoro_proto::pb::{
    AiControlSwitchRequest, AiControlSwitchResponse, TimeCheckRequest, TimeCheckResponse,
    TimeStopPush,
};

use super::State;

pub fn handle_time_stop_push(state: &mut State, req: TimeStopPush) -> Result<(), String> {
    if let Some(scene) = state.scene.as_mut() {
        scene
            .scene_time
            .set_dilation(now_millis(), f64::from(req.TimeDilation));
    }
    Ok(())
}

pub fn handle_time_check_request(
    state: &mut State,
    req: TimeCheckRequest,
) -> Result<TimeCheckResponse, String> {
    let monotonic_now = now_millis();
    let flow_timestamp = match state.scene.as_ref() {
        Some(scene) => scene.scene_time.current_flow_timestamp(monotonic_now),
        None => monotonic_now,
    };
    let wall_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| i64::try_from(d.as_millis()).unwrap_or(i64::MAX));
    Ok(TimeCheckResponse {
        ErrorCode: Some(ErrorCode::Success),
        ClientTime: req.ClientTime,
        ServerTime: wall_ms,
        ServerCombatTime: wall_ms,
        ServerStopTime: wall_ms,
        ServerFlowTimestamp: flow_timestamp,
    })
}

pub fn handle_ai_control_switch_request(
    _state: &mut State,
    req: AiControlSwitchRequest,
) -> Result<AiControlSwitchResponse, String> {
    if req.EntityId <= 0 {
        return Ok(AiControlSwitchResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    }
    Ok(AiControlSwitchResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}
