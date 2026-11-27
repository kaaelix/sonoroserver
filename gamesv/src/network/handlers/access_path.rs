use sonoro_gamesv_logic::helpers::energy;
use sonoro_proto::pb::{
    AccessPathTimeServerConfig, AccessPathTimeServerConfigRequest, AccessPathTimeServerConfigResponse,
};

use super::State;

pub fn handle_access_path_time_server_config_request(
    state: &mut State,
    _req: AccessPathTimeServerConfigRequest,
) -> Result<AccessPathTimeServerConfigResponse, String> {
    let rows = state.assets.reward_index.get_rows("AccessPath");
    // SkipInterfaceModel.IsAccessPathInOpenTime compares against server timestamp in milliseconds.
    let end_ms = (energy::now_seconds() + 315_360_000).saturating_mul(1000);
    let mut configs = Vec::with_capacity(rows.len());
    for row in rows.iter() {
        if row.key <= 0 {
            continue;
        }
        configs.push(AccessPathTimeServerConfig {
            Id: row.key as i32,
            BeginTime: 1,
            EndTime: end_ms,
        });
    }
    Ok(AccessPathTimeServerConfigResponse {
        AccessPathTimeServerConfig: configs,
    })
}
