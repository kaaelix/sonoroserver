use std::collections::BTreeMap;
use std::io;

use sonoro_common::fs::FileSystem;
use sonoro_gamesv_data::Assets;
use sonoro_gamesv_fs::special_item_incr::{self, TRIAL_INCR_MIN};
use sonoro_gamesv_fs::weapon_item::WeaponItem;

use crate::component::comp_util::{self, FsStoreItem, StoreType};

impl FsStoreItem for WeaponItem {
    const DATA_DIR: &'static str = WeaponItem::DATA_DIR;
    const SHARED_INCR_ID: bool = WeaponItem::SHARED_INCR_ID;

    fn add_defaults(assets: &Assets, map: &mut BTreeMap<i32, Self>) {
        WeaponItem::add_defaults(assets, map);
    }
}

#[derive(Debug, Default)]
pub struct PlayerWeaponComponent {
    pub player_id: i32,
    pub weapon_map: BTreeMap<i32, WeaponItem>,
}

impl PlayerWeaponComponent {
    pub fn init(fs: &FileSystem, assets: &Assets, player_id: i32) -> io::Result<Self> {
        let weapon_map =
            comp_util::load_items::<WeaponItem>(fs, assets, player_id, StoreType::ByIncrId)?;
        special_item_incr::ensure_at_least(
            fs,
            player_id,
            special_item_incr::next_after_map(&weapon_map),
        )?;
        Ok(Self {
            player_id,
            weapon_map,
        })
    }

    pub fn save_all(
        fs: &FileSystem,
        player_id: i32,
        weapon_map: &BTreeMap<i32, WeaponItem>,
    ) -> io::Result<()> {
        let dir_path = format!("player/{player_id}/{}", WeaponItem::DATA_DIR);
        if let Some(entries) = fs.read_dir(&dir_path)? {
            for entry in entries {
                let Some(name) = comp_util::entry_file_name(&entry) else {
                    continue;
                };
                let Ok(incr_id) = name.parse::<i32>() else {
                    continue;
                };
                if incr_id >= TRIAL_INCR_MIN || weapon_map.contains_key(&incr_id) {
                    continue;
                }
                fs.delete_file(&format!(
                    "player/{player_id}/{}/{incr_id}",
                    WeaponItem::DATA_DIR
                ))?;
            }
        }

        for (&incr_id, weapon) in weapon_map {
            if incr_id >= TRIAL_INCR_MIN {
                continue;
            }
            let path = format!("player/{player_id}/{}/{incr_id}", WeaponItem::DATA_DIR);
            comp_util::save_struct(fs, weapon, &path)?;
        }
        Ok(())
    }

    pub fn delete_weapon_file(fs: &FileSystem, player_id: i32, incr_id: i32) -> io::Result<()> {
        if incr_id >= TRIAL_INCR_MIN {
            return Ok(());
        }
        fs.delete_file(&format!(
            "player/{player_id}/{}/{incr_id}",
            WeaponItem::DATA_DIR
        ))
    }
}
