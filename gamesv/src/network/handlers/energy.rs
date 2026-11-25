use sonoro_gamesv_logic::helpers::energy;
use sonoro_proto::enums::ErrorCode;
use sonoro_proto::pb::{EnergySyncRequest, EnergySyncResponse};

use super::State;

pub fn handle_energy_sync_request(
    state: &mut State,
    req: EnergySyncRequest,
) -> Result<EnergySyncResponse, String> {
    energy::sync_and_save(&state.fs, &mut state.player_components.basic)
        .map_err(|e| e.to_string())?;
    let list = energy::build_energy_list(&state.player_components.basic, &req.EnergyTypes);
    Ok(EnergySyncResponse {
        ErrorCode: Some(ErrorCode::Success),
        SyncInfo: list,
    })
}
