use serde::{Deserialize, Serialize};
use sonoro_gamesv_data::Assets;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct IllustratedRecord {
    pub entry_type: i32,
    pub id: i32,
    pub create_time: u32,
    pub num: i32,
    pub is_read: bool,
}

impl Default for IllustratedRecord {
    fn default() -> Self {
        Self {
            entry_type: 0,
            id: 0,
            create_time: 1,
            num: 1,
            is_read: false,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct CosmeticInfo {
    pub role_skins: Vec<i32>,
    pub phantom_skins: Vec<i32>,
    pub fly_skins: Vec<i32>,
    pub weapon_skins: Vec<i32>,
    pub ornaments: Vec<i32>,
    pub viewed_ornaments: Vec<i32>,
    pub illustrated_entries: Vec<IllustratedRecord>,
}

impl CosmeticInfo {
    pub const DATA_PATH: &'static str = "cosmetic_info";

    pub fn int_list(items: &[i32]) -> Vec<i32> {
        items.iter().copied().filter(|&id| id != 0).collect()
    }

    pub fn has(items: &[i32], id: i32) -> bool {
        id == 0 || items.contains(&id)
    }

    pub fn is_empty(&self) -> bool {
        self.role_skins.is_empty()
            && self.phantom_skins.is_empty()
            && self.fly_skins.is_empty()
            && self.weapon_skins.is_empty()
            && self.ornaments.is_empty()
            && self.viewed_ornaments.is_empty()
    }

    pub fn add_defaults(&mut self, assets: &Assets) {
        self.role_skins.clear();
        self.phantom_skins.clear();
        self.fly_skins.clear();
        self.weapon_skins.clear();
        self.ornaments.clear();
        self.viewed_ornaments.clear();

        for info in &assets.tables.fly_skin_config.items {
            self.fly_skins.push(info.Id);
            self.phantom_skins.push(info.Id);
        }
        for info in &assets.tables.role_skin.items {
            self.role_skins.push(info.Id);
            self.phantom_skins.push(info.Id);
        }
        for info in &assets.tables.weapon_skin.items {
            if info.HideInSkinView {
                continue;
            }
            self.weapon_skins.push(info.Id);
            self.phantom_skins.push(info.Id);
        }
        for info in &assets.tables.ornament.items {
            if info.HideInUi {
                continue;
            }
            self.ornaments.push(info.Id);
            self.phantom_skins.push(info.Id);
            self.viewed_ornaments.push(info.Id);
            for &id in &info.ItemAccess {
                self.phantom_skins.push(id);
                self.viewed_ornaments.push(id);
            }
        }
        for info in &assets.tables.phantom_item.items {
            if info.ParentMonsterId != 0 {
                self.phantom_skins.push(info.ItemId);
            }
        }
    }

    pub fn ensure_default_phantom_skins(&mut self, assets: &Assets) -> bool {
        let mut changed = false;
        for item in &assets.tables.phantom_item.items {
            if item.ParentMonsterId == 0 || Self::has(&self.phantom_skins, item.ItemId) {
                continue;
            }
            self.phantom_skins.push(item.ItemId);
            changed = true;
        }
        changed
    }

    pub fn find_illustrated(&self, entry_type: i32, id: i32) -> Option<IllustratedRecord> {
        self.illustrated_entries
            .iter()
            .find(|r| r.entry_type == entry_type && r.id == id)
            .cloned()
    }

    pub fn mark_illustrated_read(&mut self, entry_type: i32, id: i32) -> bool {
        let mut changed = false;
        for rec in &mut self.illustrated_entries {
            if rec.entry_type == entry_type && (id == 0 || rec.id == id) && !rec.is_read {
                rec.is_read = true;
                changed = true;
            }
        }
        changed
    }

    pub fn unlock_illustrated(
        &mut self,
        entry_type: i32,
        id: i32,
        create_time: u32,
    ) -> IllustratedRecord {
        if let Some(rec) = self
            .illustrated_entries
            .iter_mut()
            .find(|r| r.entry_type == entry_type && r.id == id)
        {
            rec.num += 1;
            return rec.clone();
        }
        let rec = IllustratedRecord {
            entry_type,
            id,
            create_time: if create_time > 0 { create_time } else { 1 },
            num: 1,
            is_read: false,
        };
        self.illustrated_entries.push(rec.clone());
        rec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mark_illustrated_read_marks_single_or_category_entries_without_losing_unread() {
        let mut info = CosmeticInfo::default();
        info.unlock_illustrated(0, 1001, 10);
        info.unlock_illustrated(0, 1002, 20);
        info.unlock_illustrated(3, 2001, 30);

        assert!(!info.find_illustrated(0, 1001).unwrap().is_read);
        assert!(info.mark_illustrated_read(0, 1001));
        assert!(info.find_illustrated(0, 1001).unwrap().is_read);
        assert!(!info.find_illustrated(0, 1002).unwrap().is_read);
        assert!(!info.mark_illustrated_read(0, 1001));

        assert!(info.mark_illustrated_read(0, 0));
        assert!(info.find_illustrated(0, 1002).unwrap().is_read);
        assert!(!info.find_illustrated(3, 2001).unwrap().is_read);
    }
}
