use sonoro_gamesv_data::tables::character_part_config::CharacterPartConfig;
use sonoro_proto::pb::{PartComponentPb, PartInformation};

use crate::component::entity::tag::TagComponent;
use crate::fsm::types::{MAX_PART_STATE_LEN, PartState};

#[derive(Debug, Clone, Default)]
pub struct Part {
    pub index: i32,
    pub name: String,
    pub life: f32,
    pub life_max: f32,
    pub activated: bool,
    pub birth_activated: bool,
    pub part_tag: i32,
    pub active_tag_id: i32,
    pub combine_socket: String,
}

#[derive(Debug, Clone, Default)]
pub struct PartComponent {
    pub parts: Vec<Part>,
}

impl PartComponent {
    pub fn from_config(config: &CharacterPartConfig, entity_life_max: i32) -> Self {
        let parts = config
            .Parts
            .iter()
            .map(|part| {
                let max_life = if part.LifeRatio > 0.0 {
                    (entity_life_max as f32) * part.LifeRatio
                } else {
                    -1.0
                };
                Part {
                    index: part.Index,
                    name: part.Name.clone(),
                    life: max_life,
                    life_max: max_life,
                    activated: part.BirthActivated,
                    birth_activated: part.BirthActivated,
                    part_tag: part.PartTagId,
                    active_tag_id: part.ActiveTagId,
                    combine_socket: part.CombineSocket.clone(),
                }
            })
            .collect();
        Self { parts }
    }

    pub fn supports_combine_part(&self, part_index: i32) -> bool {
        self.parts
            .iter()
            .any(|p| p.index == part_index && !p.combine_socket.is_empty())
    }

    pub fn find_mut(&mut self, name: &str) -> Option<&mut Part> {
        self.parts.iter_mut().find(|p| p.name == name)
    }

    pub fn find(&self, name: &str) -> Option<&Part> {
        self.parts.iter().find(|p| p.name == name)
    }

    pub fn sync_active_tags(&self, tags: Option<&mut TagComponent>) {
        let Some(tag_comp) = tags else {
            return;
        };
        for (index, part) in self.parts.iter().enumerate() {
            if self.parts[..index]
                .iter()
                .any(|prev| prev.active_tag_id == part.active_tag_id)
            {
                continue;
            }
            let active_count = self
                .parts
                .iter()
                .filter(|c| c.active_tag_id == part.active_tag_id && c.activated)
                .count() as i32;
            let cur_count = tag_comp.tag_count(part.active_tag_id);
            tag_comp.add_tag(part.active_tag_id, active_count - cur_count);
        }
    }

    pub fn activate(&mut self, part_name: &str, activated: bool, tags: Option<&mut TagComponent>) -> Option<usize> {
        if part_name.is_empty() {
            return None;
        }
        let idx = self.parts.iter().position(|p| p.name == part_name)?;
        if self.parts[idx].activated == activated {
            return None;
        }
        self.parts[idx].activated = activated;
        self.sync_active_tags(tags);
        Some(idx)
    }

    pub fn reset(
        &mut self,
        part_name: &str,
        reset_activate: bool,
        reset_life: bool,
        tags: Option<&mut TagComponent>,
    ) -> Vec<usize> {
        let mut changed = Vec::new();
        let mut activation_changed = false;
        for (index, part) in self.parts.iter_mut().enumerate() {
            if !part_name.is_empty() && part.name != part_name {
                continue;
            }
            let mut part_changed = false;
            if reset_life && (part.life - part.life_max).abs() > f32::EPSILON {
                part.life = part.life_max;
                part_changed = true;
            }
            if reset_activate && part.activated != part.birth_activated {
                part.activated = part.birth_activated;
                activation_changed = true;
                part_changed = true;
            }
            if part_changed {
                changed.push(index);
            }
        }
        if activation_changed {
            self.sync_active_tags(tags);
        }
        changed
    }

    pub fn to_fsm_states(&self) -> Vec<PartState> {
        self.parts
            .iter()
            .map(|p| {
                let bytes = p.name.as_bytes();
                let copy_len = bytes.len().min(MAX_PART_STATE_LEN);
                let mut name = [0u8; MAX_PART_STATE_LEN];
                name[..copy_len].copy_from_slice(&bytes[..copy_len]);
                PartState {
                    name,
                    name_len: copy_len as u8,
                    life: p.life,
                    life_max: p.life_max,
                    activated: p.activated,
                }
            })
            .collect()
    }

    pub fn append_part_info(&self, index: usize, output: &mut Vec<PartInformation>) {
        if let Some(p) = self.parts.get(index) {
            output.push(PartInformation {
                PartIndex: p.index,
                LifeValue: p.life,
                LifeMax: p.life_max,
                Activated: p.activated,
                PartTag: p.part_tag,
            });
        }
    }

    pub fn to_proto(&self) -> PartComponentPb {
        let mut part_life_infos = Vec::with_capacity(self.parts.len());
        for i in 0..self.parts.len() {
            self.append_part_info(i, &mut part_life_infos);
        }
        PartComponentPb {
            PartLifeInfos: part_life_infos,
        }
    }
}
