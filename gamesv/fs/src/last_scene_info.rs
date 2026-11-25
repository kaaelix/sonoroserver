use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct LastSceneInfo {
    pub instance_id: i32,
}

impl Default for LastSceneInfo {
    fn default() -> Self {
        Self { instance_id: 8 }
    }
}
