use sonoro_gamesv_data::data_tables::Table;
use sonoro_gamesv_data::tables::gameplay_tag_parent::GameplayTagParent;
use sonoro_proto::pb::{GameplayTagData, TagComponentPb};

use crate::helpers::gameplay_tags;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TagEntry {
    pub id: i32,
    pub count: i32,
}

#[derive(Debug, Clone, Default)]
pub struct TagComponent {
    pub gameplay_tags: Vec<TagEntry>,
    pub entity_common_tags: Vec<i32>,
    pub init_gameplay_tag: bool,
}

impl TagComponent {
    pub fn has_tag(&self, parents: &Table<GameplayTagParent, i32>, tag_id: i32) -> bool {
        if tag_id == 0 {
            return false;
        }
        self.gameplay_tags
            .iter()
            .any(|t| t.count > 0 && gameplay_tags::contains(parents, t.id, tag_id))
            || self.entity_common_tags.contains(&tag_id)
    }

    pub fn has_direct_tag(&self, tag_id: i32) -> bool {
        self.tag_count(tag_id) > 0 || self.entity_common_tags.contains(&tag_id)
    }

    pub fn tag_count(&self, tag_id: i32) -> i32 {
        self.gameplay_tags
            .iter()
            .find(|t| t.id == tag_id)
            .map_or(0, |t| t.count)
    }

    pub fn add_tag(&mut self, tag_id: i32, delta: i32) -> bool {
        if tag_id == 0 || delta == 0 {
            return false;
        }
        if let Some(entry) = self.gameplay_tags.iter_mut().find(|t| t.id == tag_id) {
            entry.count = entry.count.saturating_add(delta).max(0);
            if entry.count == 0 {
                self.gameplay_tags.retain(|t| t.count > 0);
            }
            return true;
        }
        if delta > 0 {
            self.gameplay_tags.push(TagEntry {
                id: tag_id,
                count: delta,
            });
            return true;
        }
        false
    }

    pub fn remove_tag(&mut self, tag_id: i32) -> bool {
        let prev = self.gameplay_tags.len();
        self.gameplay_tags.retain(|t| t.id != tag_id);
        self.gameplay_tags.len() != prev
    }

    pub fn set_tag_count(&mut self, tag_id: i32, count: i32) {
        if count <= 0 {
            let _ = self.remove_tag(tag_id);
            return;
        }
        if let Some(entry) = self.gameplay_tags.iter_mut().find(|t| t.id == tag_id) {
            entry.count = count;
            return;
        }
        self.gameplay_tags.push(TagEntry { id: tag_id, count });
    }

    pub fn active_tag_ids(&self) -> Vec<i32> {
        let mut out: Vec<i32> = self
            .gameplay_tags
            .iter()
            .filter(|t| t.count > 0)
            .map(|t| t.id)
            .collect();
        for &cid in &self.entity_common_tags {
            if !out.contains(&cid) {
                out.push(cid);
            }
        }
        out
    }

    pub fn to_proto(&self) -> TagComponentPb {
        TagComponentPb {
            GameplayTags: self
                .gameplay_tags
                .iter()
                .filter(|t| t.count > 0)
                .map(|t| GameplayTagData {
                    Id: t.id,
                    TagCount: t.count,
                })
                .collect(),
            EntityCommonTags: self.entity_common_tags.clone(),
            InitGameplayTag: self.init_gameplay_tag,
        }
    }
}
