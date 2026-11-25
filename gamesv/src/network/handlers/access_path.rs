use sonoro_proto::pb::{
    AccessPathTimeServerConfig, AccessPathTimeServerConfigRequest, AccessPathTimeServerConfigResponse,
};

use super::State;

pub fn handle_access_path_time_server_config_request(
    state: &mut State,
    _req: AccessPathTimeServerConfigRequest,
) -> Result<AccessPathTimeServerConfigResponse, String> {
    let rows = state.assets.reward_index.get_rows("AccessPath");
    let mut configs = Vec::new();
    for row in rows.iter() {
        if row.key <= 0 {
            continue;
        }
        configs.push(AccessPathTimeServerConfig {
            Id: row.key as i32,
            BeginTime: 1,
            EndTime: 2_000_000_000,
        });
        if configs.len() >= 128 {
            break;
        }
    }
    Ok(AccessPathTimeServerConfigResponse {
        AccessPathTimeServerConfig: configs,
    })
}
