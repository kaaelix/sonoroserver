use serde::Deserialize;

pub const DEFAULT_STATE_MACHINE: &str = "SM_RoleBeHit";

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct RoleBeHitMap {
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub Id: i32,
    #[serde(default, deserialize_with = "super::de_null_default")]
    pub StateMachineName: String,
}

impl RoleBeHitMap {
    pub fn state_machine_name(config: Option<&Self>) -> &str {
        config.map_or(DEFAULT_STATE_MACHINE, |entry| entry.StateMachineName.as_str())
    }
}
