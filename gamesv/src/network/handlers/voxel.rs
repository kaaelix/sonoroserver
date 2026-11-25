use sonoro_gamesv_logic::helpers::pb_kv;
use sonoro_proto::enums::ErrorCode;
use sonoro_proto::pb::{PlayerVoxelEnvRequest, PlayerVoxelEnvResponse};

use super::State;

pub fn handle_player_voxel_env_request(
    state: &mut State,
    req: PlayerVoxelEnvRequest,
) -> Result<PlayerVoxelEnvResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "player:voxel_env",
        i64::from(req.VoxelEnv),
    )
    .map_err(|e| e.to_string())?;
    Ok(PlayerVoxelEnvResponse {
        ErrorCode: Some(ErrorCode::Success),
        VoxelEnv: req.VoxelEnv,
    })
}
