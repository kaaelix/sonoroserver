use serde::{Deserialize, Serialize};
use sonoro_proto::enums::MarkPointState;
use sonoro_proto::pb::{
    MarkPointInfo, MarkPointRequestInfo, Rotator, TemporaryTeleportInfo, Vector,
};

const fn custom_mark_state(state: i32) -> MarkPointState {
    match state {
        1 => MarkPointState::MarkDisable,
        2 => MarkPointState::MarkComplete,
        _ => MarkPointState::MarkNormal,
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct CustomMark {
    pub id: i32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub is_trace: i32,
    pub mark_type: i32,
    pub map_id: i32,
    pub config_id: i32,
    pub entity_config_id: i32,
    pub locked: bool,
    pub state: i32,
}

impl CustomMark {
    pub fn to_pb(&self) -> MarkPointInfo {
        MarkPointInfo {
            PosX: self.x,
            PosY: self.y,
            PosZ: self.z,
            ConfigId: self.config_id,
            MarkId: self.id,
            MapId: self.map_id,
            MarkType: self.mark_type,
            EntityConfigId: self.entity_config_id,
            IsServerDisable: self.locked,
            MarkPointState: Some(custom_mark_state(self.state)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct TempTeleport {
    pub id: i64,
    pub mark_id: i32,
    pub map_id: i32,
    pub inst_id: i32,
    pub area_id: i32,
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
    pub rot_pitch: f32,
    pub rot_yaw: f32,
    pub rot_roll: f32,
}

impl Default for TempTeleport {
    fn default() -> Self {
        Self {
            id: 0,
            mark_id: 0,
            map_id: 8,
            inst_id: 8,
            area_id: 0,
            pos_x: 0.0,
            pos_y: 0.0,
            pos_z: 0.0,
            rot_pitch: 0.0,
            rot_yaw: 0.0,
            rot_roll: 0.0,
        }
    }
}

impl TempTeleport {
    pub fn to_pb(&self) -> TemporaryTeleportInfo {
        let pos = Vector {
            X: self.pos_x,
            Y: self.pos_y,
            Z: self.pos_z,
        };
        TemporaryTeleportInfo {
            MarkId: self.mark_id,
            TemporaryTeleportId: self.id,
            MapId: self.map_id,
            InstId: self.inst_id,
            Pos: Some(pos.clone()),
            Rot: Some(Rotator {
                Pitch: self.rot_pitch,
                Yaw: self.rot_yaw,
                Roll: self.rot_roll,
            }),
            PlayerTeleportPos: Some(pos),
            Area: self.area_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct MapInfo {
    pub custom_marks: Vec<CustomMark>,
    pub tracked_mark_ids: Vec<i32>,
    pub unlocked_fogs: Vec<i32>,
    pub unlocked_teleporters: Vec<i32>,
    pub unlocked_areas: Vec<i32>,
    pub claimed_area_rewards: Vec<i32>,
    pub temp_teleports: Vec<TempTeleport>,
    pub next_custom_mark_id: i32,
    pub next_temp_teleport_id: i64,
}

impl Default for MapInfo {
    fn default() -> Self {
        Self {
            custom_marks: Vec::new(),
            tracked_mark_ids: Vec::new(),
            unlocked_fogs: Vec::new(),
            unlocked_teleporters: Vec::new(),
            unlocked_areas: Vec::new(),
            claimed_area_rewards: Vec::new(),
            temp_teleports: Vec::new(),
            next_custom_mark_id: 1_000_000,
            next_temp_teleport_id: 1,
        }
    }
}

impl MapInfo {
    pub const DATA_PATH: &'static str = "map_info";

    pub fn add_custom_mark(&mut self, request: &MarkPointRequestInfo) -> CustomMark {
        let mark_id = self.next_custom_mark_id;
        self.next_custom_mark_id += 1;
        let mark = CustomMark {
            id: mark_id,
            x: request.PosX,
            y: request.PosY,
            z: request.PosZ,
            is_trace: i32::from(request.IsTrace),
            mark_type: request.MarkType,
            map_id: request.MapId,
            config_id: request.ConfigId,
            entity_config_id: request.EntityConfigId,
            locked: false,
            state: 0,
        };
        self.custom_marks.push(mark.clone());
        mark
    }

    pub fn delete_custom_marks(&mut self, mark_ids: &[i32]) {
        if mark_ids.is_empty() {
            return;
        }
        self.custom_marks.retain(|m| !mark_ids.contains(&m.id));
        self.tracked_mark_ids.retain(|id| !mark_ids.contains(id));
    }

    pub fn track_mark(&mut self, mark_id: i32) {
        if mark_id != 0 && !self.tracked_mark_ids.contains(&mark_id) {
            self.tracked_mark_ids.push(mark_id);
        }
    }

    pub fn untrack_mark(&mut self, mark_id: i32) {
        self.tracked_mark_ids.retain(|&id| id != mark_id);
    }

    pub fn unlock_fog(&mut self, fog_id: i32) -> bool {
        append_unique_id(&mut self.unlocked_fogs, fog_id)
    }

    pub fn unlock_teleporter(&mut self, tp_id: i32) -> bool {
        append_unique_id(&mut self.unlocked_teleporters, tp_id)
    }

    pub fn unlock_area(&mut self, area_id: i32) -> bool {
        append_unique_id(&mut self.unlocked_areas, area_id)
    }

    pub fn claim_area_reward(&mut self, reward_id: i32) -> bool {
        append_unique_id(&mut self.claimed_area_rewards, reward_id)
    }

    pub fn add_temp_teleport(
        &mut self,
        map_id: i32,
        area_id: i32,
        pos: &Vector,
        rot: &Rotator,
    ) -> TempTeleport {
        let id = self.next_temp_teleport_id;
        self.next_temp_teleport_id += 1;
        let mark_id = self.next_custom_mark_id;
        self.next_custom_mark_id += 1;
        let tp = TempTeleport {
            id,
            mark_id,
            map_id,
            inst_id: map_id,
            area_id,
            pos_x: pos.X,
            pos_y: pos.Y,
            pos_z: pos.Z,
            rot_pitch: rot.Pitch,
            rot_yaw: rot.Yaw,
            rot_roll: rot.Roll,
        };
        self.temp_teleports.push(tp.clone());
        tp
    }

    pub fn find_temp_teleport(&self, id: i64) -> Option<TempTeleport> {
        self.temp_teleports.iter().find(|t| t.id == id).cloned()
    }

    pub fn remove_temp_teleport(&mut self, id: i64) -> bool {
        let before = self.temp_teleports.len();
        self.temp_teleports.retain(|t| t.id != id);
        self.temp_teleports.len() != before
    }
}

fn append_unique_id(slice: &mut Vec<i32>, id: i32) -> bool {
    if id <= 0 || slice.contains(&id) {
        return false;
    }
    slice.push(id);
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_info_custom_mark_and_temp_teleport_lifecycle() {
        let mut info = MapInfo::default();
        let mark = info.add_custom_mark(&MarkPointRequestInfo {
            PosX: 12.5,
            PosY: 34.5,
            PosZ: 56.5,
            ConfigId: 42,
            MarkType: 1,
            IsTrace: true,
            MapId: 8,
            EntityConfigId: 900123,
            ..Default::default()
        });
        let pb_mark = mark.to_pb();
        assert_eq!(pb_mark.EntityConfigId, 900123);
        assert_eq!(pb_mark.MarkId, 1_000_000);

        let tp = info.add_temp_teleport(
            8,
            12,
            &Vector {
                X: 100.0,
                Y: 200.0,
                Z: 300.0,
            },
            &Rotator {
                Yaw: 45.0,
                ..Default::default()
            },
        );
        assert_eq!(tp.id, 1);
        assert_eq!(tp.area_id, 12);
        assert!(info.find_temp_teleport(tp.id).is_some());
        assert_eq!(tp.to_pb().TemporaryTeleportId, 1);
        assert!(info.remove_temp_teleport(tp.id));
        assert!(info.find_temp_teleport(tp.id).is_none());
    }
}
