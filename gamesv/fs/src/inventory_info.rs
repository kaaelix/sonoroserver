use serde::{Deserialize, Serialize};
use sonoro_proto::pb::NormalItem as PbNormalItem;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct NormalItem {
    pub id: i32,
    pub count: i32,
}

impl Default for NormalItem {
    fn default() -> Self {
        Self { id: 0, count: 1 }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct InventoryInfo {
    pub normal_items: Vec<NormalItem>,
}

impl InventoryInfo {
    pub const DATA_PATH: &'static str = "inventory_info";

    pub fn normal_item_list(&self) -> Vec<PbNormalItem> {
        self.normal_items
            .iter()
            .filter(|i| i.id != 0 && i.count > 0)
            .map(|i| PbNormalItem {
                Id: i.id,
                Count: i.count,
                ExpireTime: 0,
            })
            .collect()
    }

    pub fn normal_item_count(&self, id: i32) -> i32 {
        self.normal_items
            .iter()
            .find(|i| i.id == id)
            .map_or(0, |i| i.count)
    }

    pub fn add_normal_item(&mut self, id: i32, count: i32) {
        if count <= 0 {
            return;
        }
        if let Some(existing) = self.normal_items.iter_mut().find(|i| i.id == id) {
            existing.count += count;
        } else {
            self.normal_items.push(NormalItem { id, count });
        }
    }

    pub fn remove_normal_item(&mut self, id: i32) -> bool {
        if let Some(pos) = self.normal_items.iter().position(|i| i.id == id) {
            self.normal_items.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn consume_normal_item(&mut self, id: i32, count: i32) -> bool {
        if count <= 0 {
            return true;
        }
        let Some(pos) = self.normal_items.iter().position(|i| i.id == id) else {
            return false;
        };
        if self.normal_items[pos].count < count {
            return false;
        }
        if self.normal_items[pos].count == count {
            self.normal_items.remove(pos);
        } else {
            self.normal_items[pos].count -= count;
        }
        true
    }

    pub fn is_empty(&self) -> bool {
        self.normal_items.is_empty()
    }
}
