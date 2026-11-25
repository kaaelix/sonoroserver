use sonoro_proto::pb::{HeartbeatRequest, HeartbeatResponse};

use super::State;

pub fn handle_heartbeat_request(
    _state: &mut State,
    _req: HeartbeatRequest,
) -> Result<HeartbeatResponse, String> {
    Ok(HeartbeatResponse {})
}
