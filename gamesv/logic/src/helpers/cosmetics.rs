use sonoro_gamesv_data::Assets;
use sonoro_gamesv_data::tables::ornament::Ornament;
use sonoro_gamesv_fs::role_info::RoleInfo;
use sonoro_proto::pb::{LoadEquipData, OrnamentDressInfo};

use crate::component::player::PlayerRoleComponent;
use crate::events::BuffAdditionEntry;

fn append_buff_entry(list: &mut Vec<BuffAdditionEntry>, id: i64) {
    if id == 0 || list.iter().any(|item| item.id == id) {
        return;
    }
    list.push(BuffAdditionEntry {
        id,
        is_active: true,
        ..Default::default()
    });
}

pub fn is_weapon_skin_compatible(assets: &Assets, role_id: i32, skin_id: i32) -> bool {
    if skin_id == 0 {
        return true;
    }
    let Some(role) = assets.tables.resolve_base_role_info(role_id) else {
        return false;
    };
    let Some(skin) = assets.tables.weapon_skin.get_data_by_id(&skin_id) else {
        return false;
    };
    skin.IsShow && !skin.HideInSkinView && skin.WeaponSkinType == role.WeaponType
}

pub fn build_weapon_skin_current_equip_list(
    assets: &Assets,
    role_comp: &PlayerRoleComponent,
) -> Vec<LoadEquipData> {
    let mut list: Vec<LoadEquipData> = Vec::new();
    for (&role_id, role_info) in &role_comp.role_map {
        if role_info.weapon_skin_id == 0 {
            continue;
        }
        let skin_id = if is_weapon_skin_compatible(assets, role_id, role_info.weapon_skin_id) {
            role_info.weapon_skin_id
        } else {
            tracing::warn!("invalid weapon skin, {}, for {}", 0, role_id);
            continue;
        };
        if !list
            .iter()
            .any(|item| item.RoleId == role_id && item.SkinId == skin_id)
        {
            list.push(LoadEquipData {
                RoleId: role_id,
                SkinId: skin_id,
            });
        }
    }
    list
}

pub fn ornament_for_role_skin(
    assets: &Assets,
    role_skin_id: i32,
    ornament_id: i32,
) -> Option<Ornament> {
    if ornament_id == 0 {
        return None;
    }
    let ornament = assets.tables.ornament.get_data_by_id(&ornament_id)?.into_owned();
    if !ornament.RoleSkinIds.contains(&role_skin_id) {
        return None;
    }
    Some(ornament)
}

pub fn build_ornament_equip_map(role_comp: &PlayerRoleComponent) -> Vec<OrnamentDressInfo> {
    let mut list: Vec<OrnamentDressInfo> = Vec::new();
    for role_info in role_comp.role_map.values() {
        for entry in &role_info.ornaments {
            if entry.ornament_id == 0 {
                continue;
            }
            if let Some(item) = list
                .iter_mut()
                .find(|item| item.RoleSkinId == entry.role_skin_id)
            {
                item.DressOrnamentIds.push(entry.ornament_id);
            } else {
                list.push(OrnamentDressInfo {
                    RoleSkinId: entry.role_skin_id,
                    DressOrnamentIds: vec![entry.ornament_id],
                });
            }
        }
    }
    list
}

pub fn build_ornament_ids_for_role_skin(role: &RoleInfo, role_skin_id: i32) -> Vec<i32> {
    role.ornaments
        .iter()
        .filter(|e| e.role_skin_id == role_skin_id && e.ornament_id != 0)
        .map(|e| e.ornament_id)
        .collect()
}

pub fn build_ornament_buffs_for_ids(
    assets: &Assets,
    ornament_ids: &[i32],
) -> Vec<BuffAdditionEntry> {
    let mut list = Vec::new();
    for &ornament_id in ornament_ids {
        let Some(ornament) = assets.tables.ornament.get_data_by_id(&ornament_id) else {
            continue;
        };
        for &buff_id in &ornament.OrnamentBuff {
            append_buff_entry(&mut list, buff_id);
        }
        for &buff_id in &ornament.OrnamentUiBuff {
            append_buff_entry(&mut list, buff_id);
        }
    }
    list
}

pub fn build_ornament_born_buff_ids_for_ids(assets: &Assets, ornament_ids: &[i32]) -> Vec<i64> {
    let mut ids = Vec::new();
    for &ornament_id in ornament_ids {
        let Some(ornament) = assets.tables.ornament.get_data_by_id(&ornament_id) else {
            continue;
        };
        for &buff_id in &ornament.OrnamentBuff {
            if buff_id != 0 {
                ids.push(buff_id);
            }
        }
        for &buff_id in &ornament.OrnamentUiBuff {
            if buff_id != 0 {
                ids.push(buff_id);
            }
        }
    }
    ids
}
