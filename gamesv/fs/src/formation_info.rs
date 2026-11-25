use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct FormationRole {
    pub role_id: i32,
    #[serde(default = "default_entity_id")]
    pub entity_id: i64,
    pub on_stage_without_control: bool,
}

fn default_entity_id() -> i64 {
    -1
}

impl Default for FormationRole {
    fn default() -> Self {
        Self {
            role_id: 0,
            entity_id: -1,
            on_stage_without_control: false,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Formation {
    pub cur_role: i32,
    pub name: Option<String>,
    pub roles: [Option<FormationRole>; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct FormationInfo {
    pub cur_formation: i32,
    pub formations: [Formation; 20],
}

impl Default for FormationInfo {
    fn default() -> Self {
        Self {
            cur_formation: 0,
            formations: std::array::from_fn(|_| Formation::default()),
        }
    }
}
