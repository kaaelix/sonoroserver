use sonoro_gamesv_data::Assets;
use sonoro_gamesv_data::tables::phantom_item::PhantomItem;
use sonoro_gamesv_fs::cosmetic_info::CosmeticInfo;
use sonoro_gamesv_fs::echo_info::{CalabashInfo, ProjectorSkin};
use sonoro_proto::pb::{PhantomInteractionUnlockNotify, UnlockIllustratedPhantom};

pub const SLOT_COUNT: usize = 8;

pub fn build_unlock_notify(
    assets: &Assets,
    calabash_info: &CalabashInfo,
    cosmetic_info: &CosmeticInfo,
) -> PhantomInteractionUnlockNotify {
    PhantomInteractionUnlockNotify {
        UnlockIllustratedPhantoms: unlock_list(assets, calabash_info, cosmetic_info),
        EquipedMonsterIds: equipped_monster_list(assets, calabash_info),
    }
}

pub fn equipped_monster_list(assets: &Assets, calabash_info: &CalabashInfo) -> Vec<i32> {
    (0..SLOT_COUNT)
        .map(|idx| {
            let monster_id = calabash_info
                .projector_monster_ids
                .get(idx)
                .copied()
                .unwrap_or(0);
            if monster_id == 0 || is_unlocked_monster(assets, monster_id) {
                monster_id
            } else {
                0
            }
        })
        .collect()
}

pub fn replace_equipped_monsters(calabash_info: &mut CalabashInfo, monster_ids: &[i32]) {
    calabash_info.projector_monster_ids = (0..SLOT_COUNT)
        .map(|idx| monster_ids.get(idx).copied().unwrap_or(0))
        .collect();
}

pub fn set_equipped_skin(calabash_info: &mut CalabashInfo, monster_id: i32, skin_id: i32) {
    calabash_info
        .projector_skins
        .retain(|e| e.monster_id != monster_id);
    if skin_id != 0 {
        calabash_info.projector_skins.push(ProjectorSkin {
            monster_id,
            skin_id,
        });
    }
}

pub fn is_unlocked_monster(assets: &Assets, monster_id: i32) -> bool {
    let Some(reward) = assets
        .tables
        .calabash_develop_reward
        .get_data_by_id(&monster_id)
    else {
        return false;
    };
    reward.IsShow
        && reward.AllowVision
        && phantom_item_by_monster_id(assets, monster_id).is_some()
}

pub fn is_valid_skin(
    assets: &Assets,
    cosmetic_info: &CosmeticInfo,
    monster_id: i32,
    skin_id: i32,
) -> bool {
    if skin_id == 0 {
        return true;
    }
    let Some(skin) = assets.tables.phantom_item.get_data_by_id(&skin_id) else {
        return false;
    };
    skin.ParentMonsterId == monster_id && CosmeticInfo::has(&cosmetic_info.phantom_skins, skin_id)
}

pub fn displayed_phantom_item(
    assets: &Assets,
    calabash_info: &CalabashInfo,
    monster_id: i32,
) -> Option<PhantomItem> {
    let base_item = phantom_item_by_monster_id(assets, monster_id)?;
    for entry in &calabash_info.projector_skins {
        if entry.monster_id != monster_id || entry.skin_id == 0 {
            continue;
        }
        let Some(skin_item) = assets.tables.phantom_item.get_data_by_id(&entry.skin_id) else {
            return Some(base_item);
        };
        if skin_item.ParentMonsterId == monster_id {
            return Some(skin_item.into_owned());
        }
    }
    Some(base_item)
}

fn unlock_list(
    assets: &Assets,
    calabash_info: &CalabashInfo,
    cosmetic_info: &CosmeticInfo,
) -> Vec<UnlockIllustratedPhantom> {
    let mut list = Vec::new();
    for reward in &assets.tables.calabash_develop_reward.items {
        if !reward.IsShow
            || !reward.AllowVision
            || phantom_item_by_monster_id(assets, reward.MonsterId).is_none()
        {
            continue;
        }
        list.push(UnlockIllustratedPhantom {
            MonsterId: reward.MonsterId,
            SkinIds: skin_id_list(assets, cosmetic_info, reward.MonsterId),
            EqupiedSkin: equipped_skin(calabash_info, cosmetic_info, assets, reward.MonsterId),
            IsSpecial: reward.IsWorldInteractable,
        });
    }
    list
}

fn skin_id_list(assets: &Assets, cosmetic_info: &CosmeticInfo, monster_id: i32) -> Vec<i32> {
    cosmetic_info
        .phantom_skins
        .iter()
        .copied()
        .filter(|skin_id| {
            assets
                .tables
                .phantom_item
                .get_data_by_id(skin_id)
                .is_some_and(|skin| skin.ParentMonsterId == monster_id)
        })
        .collect()
}

fn equipped_skin(
    calabash_info: &CalabashInfo,
    cosmetic_info: &CosmeticInfo,
    assets: &Assets,
    monster_id: i32,
) -> i32 {
    for entry in &calabash_info.projector_skins {
        if entry.monster_id == monster_id
            && is_valid_skin(assets, cosmetic_info, monster_id, entry.skin_id)
        {
            return entry.skin_id;
        }
    }
    0
}

fn phantom_item_by_monster_id(assets: &Assets, monster_id: i32) -> Option<PhantomItem> {
    assets
        .tables
        .phantom_item
        .items
        .iter()
        .find(|i| i.MonsterId == monster_id)
        .cloned()
}
