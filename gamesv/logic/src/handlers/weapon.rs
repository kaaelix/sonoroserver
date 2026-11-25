use std::io;

use sonoro_common::fs::FileSystem;
use sonoro_gamesv_data::Assets;
use sonoro_gamesv_fs::special_item_incr::next as next_special_item_id;
use sonoro_gamesv_fs::weapon_item::WeaponItem;

use crate::component::comp_util;
use crate::component::player::{PlayerRoleComponent, PlayerWeaponComponent};
use crate::event_queue::EventQueue;
use crate::events::Event;

pub fn ensure_role_weapons(
    events: &mut EventQueue,
    fs: &FileSystem,
    assets: &Assets,
    role_comp: &mut PlayerRoleComponent,
    weapon_comp: &mut PlayerWeaponComponent,
) -> io::Result<()> {
    let player_id = role_comp.player_id;
    let role_ids: Vec<i32> = role_comp.role_map.keys().copied().collect();

    for role_id in role_ids {
        let already_equipped = weapon_comp
            .weapon_map
            .values()
            .any(|weapon| weapon.role_id == Some(role_id));
        if already_equipped {
            continue;
        }

        let Some(role_info) = assets.tables.role_info.get_data_by_id(&role_id) else {
            continue;
        };
        let weapon = WeaponItem {
            id: role_info.InitWeaponItemId,
            func_value: WeaponItem::default_func_value(assets, role_info.InitWeaponItemId),
            role_id: Some(role_id),
            ..Default::default()
        };

        let incr_id = next_special_item_id(fs, player_id)?;
        let path = format!("player/{player_id}/weapon/{incr_id}");
        comp_util::save_struct(fs, &weapon, &path)?;

        if let Some(role) = role_comp.role_map.get_mut(&role_id) {
            role.weapon = incr_id;
        }
        events.enqueue(Event::RoleInfoModified { player_id, role_id });
        weapon_comp.weapon_map.insert(incr_id, weapon);
    }
    Ok(())
}
