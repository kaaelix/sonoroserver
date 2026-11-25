use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct TutorialRecord {
    pub id: i32,
    pub create_time: u32,
    pub get_award: bool,
}

impl Default for TutorialRecord {
    fn default() -> Self {
        Self {
            id: 0,
            create_time: 1,
            get_award: false,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct GuideInfo {
    pub finished_groups: Vec<i32>,
    pub unlocked_tutorials: Vec<TutorialRecord>,
}

impl GuideInfo {
    pub const DATA_PATH: &'static str = "guide_info";

    pub fn has_finished(&self, group_id: i32) -> bool {
        group_id == 0 || self.finished_groups.contains(&group_id)
    }

    pub fn add_finished(&mut self, group_id: i32) {
        if !self.has_finished(group_id) {
            self.finished_groups.push(group_id);
        }
    }

    pub fn find_tutorial_mut(&mut self, tutorial_id: i32) -> Option<&mut TutorialRecord> {
        self.unlocked_tutorials
            .iter_mut()
            .find(|r| r.id == tutorial_id)
    }

    pub fn unlock_tutorial(&mut self, tutorial_id: i32, create_time: u32) -> TutorialRecord {
        if let Some(rec) = self.unlocked_tutorials.iter().find(|r| r.id == tutorial_id) {
            return rec.clone();
        }
        let rec = TutorialRecord {
            id: tutorial_id,
            create_time: if create_time > 0 { create_time } else { 1 },
            get_award: false,
        };
        self.unlocked_tutorials.push(rec.clone());
        rec
    }
}
