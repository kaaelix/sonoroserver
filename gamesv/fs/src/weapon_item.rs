use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use sonoro_gamesv_data::Assets;
use sonoro_proto::pb::WeaponItem as PbWeaponItem;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct WeaponItem {
    pub id: i32,
    pub func_value: i32,
    pub level: i32,
    pub exp: i32,
    pub breach: i32,
    pub reson_level: i32,
    pub role_id: Option<i32>,
}

impl Default for WeaponItem {
    fn default() -> Self {
        Self {
            id: 0,
            func_value: 0,
            level: 1,
            exp: 0,
            breach: 0,
            reson_level: 1,
            role_id: None,
        }
    }
}

impl WeaponItem {
    pub const DATA_DIR: &'static str = "weapon";
    pub const SHARED_INCR_ID: bool = true;

    pub fn to_proto(&self, incr_id: i32) -> PbWeaponItem {
        PbWeaponItem {
            Id: self.id,
            IncrId: incr_id,
            FuncValue: self.func_value,
            WeaponLevel: self.level,
            WeaponExp: self.exp,
            WeaponBreach: self.breach,
            WeaponResonLevel: self.reson_level,
            RoleId: self.role_id.unwrap_or(0),
        }
    }

    pub fn default_func_value(assets: &Assets, item_id: i32) -> i32 {
        if is_gold(assets, item_id) { 1 } else { 0 }
    }

    pub fn add_defaults(assets: &Assets, map: &mut BTreeMap<i32, WeaponItem>) {
        const STARTER_WEAPONS: [i32; 3] = [21020011, 21010011, 21050011];
        for weapon_id in STARTER_WEAPONS {
            let next_id = (map.len() + 1) as i32;
            map.insert(
                next_id,
                WeaponItem {
                    id: weapon_id,
                    func_value: Self::default_func_value(assets, weapon_id),
                    ..Default::default()
                },
            );
        }
    }
}

fn is_gold(assets: &Assets, item_id: i32) -> bool {
    assets
        .tables
        .weapon_conf
        .get_data_by_id(&item_id)
        .is_some_and(|c| c.QualityId >= 5)
}
