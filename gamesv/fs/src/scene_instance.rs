use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct RespawnCooldown {
    pub config_id: i32,
    pub ready_at_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct MapTime {
    pub hour: i32,
    pub minute: i32,
    pub owner_clock_time_span: i64,
    pub day: u32,
}

impl Default for MapTime {
    fn default() -> Self {
        Self {
            hour: 8,
            minute: 0,
            owner_clock_time_span: 0,
            day: 1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct ScenePlayer {
    pub id: i32,
    pub group_type: i32,
    pub location: [f32; 3],
    pub rotation: [f32; 3],
}

impl Default for ScenePlayer {
    fn default() -> Self {
        Self {
            id: 0,
            group_type: 1,
            location: [0.0; 3],
            rotation: [0.0; 3],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct SceneInstance {
    pub owner_id: i32,
    pub buff_handle: i32,
    pub map_time: MapTime,
    pub players: Vec<ScenePlayer>,
    pub active_dungeon_id: i32,
    pub prev_world_instance_id: i32,
    pub prev_world_location: [f32; 3],
    pub prev_world_rotation: [f32; 3],
    pub respawn_cooldowns: Vec<RespawnCooldown>,
}

impl Default for SceneInstance {
    fn default() -> Self {
        Self {
            owner_id: 0,
            buff_handle: 0,
            map_time: MapTime::default(),
            players: Vec::new(),
            active_dungeon_id: 0,
            prev_world_instance_id: 8,
            prev_world_location: [0.0; 3],
            prev_world_rotation: [0.0; 3],
            respawn_cooldowns: Vec::new(),
        }
    }
}

impl SceneInstance {
    pub fn is_on_respawn_cooldown(&self, config_id: i32, now_ms: i64) -> bool {
        config_id > 0
            && self
                .respawn_cooldowns
                .iter()
                .any(|e| e.config_id == config_id && now_ms < e.ready_at_ms)
    }

    pub fn set_respawn_cooldown(&mut self, config_id: i32, now_ms: i64, ready_at_ms: i64) {
        if config_id <= 0 || ready_at_ms <= now_ms {
            return;
        }
        self.respawn_cooldowns.retain(|e| e.ready_at_ms > now_ms);
        if let Some(entry) = self
            .respawn_cooldowns
            .iter_mut()
            .find(|e| e.config_id == config_id)
        {
            entry.ready_at_ms = ready_at_ms;
        } else {
            self.respawn_cooldowns.push(RespawnCooldown {
                config_id,
                ready_at_ms,
            });
        }
    }
}
