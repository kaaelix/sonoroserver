use sonoro_gamesv_data::Assets;
use sonoro_gamesv_fs::cosmetic_info::CosmeticInfo;
use sonoro_proto::enums::{EAttributeType, EClientStorageSystemIdType, FavorItemStatus};
use sonoro_proto::pb::{
    ClientStorageInfo, ClientStorageInfoDataOneof, ClientStorageSetData, FavorItem,
    FlyEquipAddNotify, FlySkinEquipData, NormalItemUpdateNotify, OrnamentInfo, OrnamentInfoNotify,
    PbGetRoleListNotify, RoleChangeUnlockNotify, RoleFavor, RoleFavorListNotify,
    RoleFlyEquipNotify, RoleMotion, RoleMotionListNotify, StorageInfoNotify, WeaponSkinAddNotify,
};
use sonoro_proto::wire::PbEnum;

use crate::component::player::{
    PlayerBasicComponent, PlayerCosmeticComponent, PlayerEchoComponent, PlayerInventoryComponent,
    PlayerProgressComponent, PlayerQuestComponent, PlayerRoleComponent, PlayerWeaponComponent,
};
use crate::event_queue::EventQueue;
use crate::events::Event;
use crate::helpers::{cosmetics, pb_kv, role as role_helper, role_stats, trial_roles};
use crate::sink::PushSink;

fn build_empty_storage_record(red_dot_type: EClientStorageSystemIdType) -> ClientStorageInfo {
    ClientStorageInfo {
        SystemId: red_dot_type.to_i32(),
        Data: Some(ClientStorageInfoDataOneof::SetData(ClientStorageSetData {
            Data: Vec::new(),
        })),
    }
}

fn build_int_storage_record(
    red_dot_type: EClientStorageSystemIdType,
    values: &[i32],
) -> ClientStorageInfo {
    ClientStorageInfo {
        SystemId: red_dot_type.to_i32(),
        Data: Some(ClientStorageInfoDataOneof::SetData(ClientStorageSetData {
            Data: values.to_vec(),
        })),
    }
}

fn append_fly_equip_role(list: &mut Vec<FlySkinEquipData>, skin_id: i32, role_id: i32) {
    if skin_id == 0 {
        return;
    }
    if let Some(item) = list.iter_mut().find(|item| item.SkinId == skin_id) {
        item.RoleIds.push(role_id);
        return;
    }
    list.push(FlySkinEquipData {
        SkinId: skin_id,
        RoleIds: vec![role_id],
    });
}

fn build_fly_equip_data(
    role_comp: &PlayerRoleComponent,
    cosmetic_info: &CosmeticInfo,
) -> Vec<FlySkinEquipData> {
    let mut list = Vec::new();
    for &skin_id in &cosmetic_info.fly_skins {
        if skin_id != 0 {
            list.push(FlySkinEquipData {
                SkinId: skin_id,
                RoleIds: Vec::new(),
            });
        }
    }
    for (&role_id, role_info) in &role_comp.role_map {
        append_fly_equip_role(&mut list, role_info.soar_skin_id, role_id);
        append_fly_equip_role(&mut list, role_info.paragliding_skin_id, role_id);
    }
    list
}

pub fn ensure_role_attributes(
    events: &mut EventQueue,
    assets: &Assets,
    role_comp: &mut PlayerRoleComponent,
) {
    let player_id = role_comp.player_id;
    let attr_max_len = EAttributeType::MAX.to_i32() as usize;

    for (&id, role) in &mut role_comp.role_map {
        if role.base_prop.len() != attr_max_len {
            // A new attribute type was introduced. Rebuild property table.
            role.reset_properties(assets, id);
            events.enqueue(Event::RoleInfoModified {
                player_id,
                role_id: id,
            });
        } else if (role.level > 1 || role.breakthrough > 0) && role.refresh_scaled_properties(assets, id) {
            events.enqueue(Event::RoleInfoModified {
                player_id,
                role_id: id,
            });
        }
    }
}

fn has_storage_system(items: &[ClientStorageInfo], sys_type: EClientStorageSystemIdType) -> bool {
    let target_id = sys_type.to_i32();
    items.iter().any(|item| item.SystemId == target_id)
}

#[allow(clippy::too_many_arguments)]
pub fn push_data<S: PushSink + ?Sized>(
    sink: &mut S,
    assets: &Assets,
    basic_comp: &PlayerBasicComponent,
    role_comp: &mut PlayerRoleComponent,
    weapon_comp: &mut PlayerWeaponComponent,
    echo_comp: &mut PlayerEchoComponent,
    cosmetic_comp: &PlayerCosmeticComponent,
    inventory_comp: &PlayerInventoryComponent,
    quest_comp: &PlayerQuestComponent,
    progress_comp: &PlayerProgressComponent,
) {
    sink.push(
        NormalItemUpdateNotify::MSG_ID,
        &NormalItemUpdateNotify {
            NormalItemList: inventory_comp.info.normal_item_list(),
            NoTips: true,
        },
    );

    sink.push(
        WeaponSkinAddNotify::MSG_ID,
        &WeaponSkinAddNotify {
            SkinIds: CosmeticInfo::int_list(&cosmetic_comp.info.weapon_skins),
            IsLogin: true,
        },
    );

    sink.push(
        FlyEquipAddNotify::MSG_ID,
        &FlyEquipAddNotify {
            UnlockFlySkinIds: CosmeticInfo::int_list(&cosmetic_comp.info.fly_skins),
        },
    );

    sink.push(
        RoleFlyEquipNotify::MSG_ID,
        &RoleFlyEquipNotify {
            FlySkinEquipData: build_fly_equip_data(role_comp, &cosmetic_comp.info),
        },
    );

    sink.push(
        OrnamentInfoNotify::MSG_ID,
        &OrnamentInfoNotify {
            OrnamentInfo: Some(OrnamentInfo {
                UnlockOrnamentIds: CosmeticInfo::int_list(&cosmetic_comp.info.ornaments),
                OrnamentDressInfos: cosmetics::build_ornament_equip_map(role_comp),
                RedPointOrnamentIds: CosmeticInfo::int_list(&cosmetic_comp.info.viewed_ornaments),
            }),
        },
    );

    let mut storage_info_notify = StorageInfoNotify::default();
    pb_kv::append_stored_client_storage_infos(progress_comp, &mut storage_info_notify.Infos);
    for sys_type in [
        EClientStorageSystemIdType::RoleSkinRedDot,
        EClientStorageSystemIdType::FlySkinRedDot,
        EClientStorageSystemIdType::WeaponSkinRedDot,
        EClientStorageSystemIdType::CalabashSkinRedDot,
        EClientStorageSystemIdType::GetOrnament,
    ] {
        if !has_storage_system(&storage_info_notify.Infos, sys_type) {
            storage_info_notify
                .Infos
                .push(build_empty_storage_record(sys_type));
        }
    }
    if !has_storage_system(&storage_info_notify.Infos, EClientStorageSystemIdType::Ornament) {
        storage_info_notify.Infos.push(build_int_storage_record(
            EClientStorageSystemIdType::Ornament,
            &cosmetic_comp.info.viewed_ornaments,
        ));
    }
    sink.push(StorageInfoNotify::MSG_ID, &storage_info_notify);

    let selected_main_role = role_helper::selected_main_role_id(
        assets,
        role_comp,
        basic_comp.info.attributes.sex,
        basic_comp.info.selected_main_role_id,
    );
    sink.push(
        RoleChangeUnlockNotify::MSG_ID,
        &RoleChangeUnlockNotify {
            UnlockRoleIds: role_helper::main_role_unlock_list(
                assets,
                basic_comp.info.attributes.sex,
            ),
            NextAllowChangeTime: 0,
        },
    );

    trial_roles::materialize_active_trial_roles(
        assets,
        role_comp,
        weapon_comp,
        echo_comp,
        &quest_comp.info.env.trial_roles,
    );

    let role_entries: Vec<(i32, sonoro_gamesv_fs::role_info::RoleInfo)> = role_comp
        .role_map
        .iter()
        .map(|(&id, r)| (id, r.clone()))
        .collect();
    let mut notify = PbGetRoleListNotify::default();
    for (role_id, role_info) in &role_entries {
        if role_comp.is_trial_role(*role_id) || *role_id > 1999 || *role_id < 1000 {
            continue;
        }
        if let Some(selected) = selected_main_role
            && role_helper::is_main_role(assets, *role_id)
            && *role_id != selected
        {
            continue;
        }
        if let Ok(pb_role) = role_stats::to_client_role_info(
            assets,
            role_comp,
            *role_id,
            role_info,
            weapon_comp,
            echo_comp,
        ) {
            notify.RoleList.push(pb_role);
        }
    }
    sink.push(PbGetRoleListNotify::MSG_ID, &notify);

    push_favor_list(sink, assets, role_comp);
    push_motion_list(sink, assets, role_comp);
}

fn push_favor_list<S: PushSink + ?Sized>(
    sink: &mut S,
    assets: &Assets,
    role_comp: &PlayerRoleComponent,
) {
    let mut favor_list = Vec::with_capacity(role_comp.role_map.len());
    for &role_id in role_comp.role_map.keys() {
        if role_comp.is_trial_role(role_id) || !(1000..=1999).contains(&role_id) {
            continue;
        }
        let word_ids = assets
            .tables
            .favor_word
            .items
            .iter()
            .filter(|w| w.RoleId == role_id)
            .map(|w| FavorItem {
                Id: w.Id,
                Status: Some(FavorItemStatus::ItemUnLocked),
            })
            .collect();
        let story_ids = assets
            .tables
            .favor_story
            .items
            .iter()
            .filter(|s| s.RoleId == role_id)
            .map(|s| FavorItem {
                Id: s.Id,
                Status: Some(FavorItemStatus::ItemUnLocked),
            })
            .collect();
        let goods_ids = assets
            .tables
            .favor_goods
            .items
            .iter()
            .filter(|g| g.RoleId == role_id)
            .map(|g| FavorItem {
                Id: g.Id,
                Status: Some(FavorItemStatus::ItemUnLocked),
            })
            .collect();

        favor_list.push(RoleFavor {
            RoleId: role_id,
            Level: 5,
            Exp: 16800,
            WordIds: word_ids,
            StoryIds: story_ids,
            GoodsIds: goods_ids,
            FavorQuest: None,
        });
    }
    sink.push(
        RoleFavorListNotify::MSG_ID,
        &RoleFavorListNotify {
            FavorList: favor_list,
            RoleConditionInfoMap: Default::default(),
        },
    );
}

fn push_motion_list<S: PushSink + ?Sized>(
    sink: &mut S,
    assets: &Assets,
    role_comp: &PlayerRoleComponent,
) {
    let mut motion_list = Vec::new();
    for &role_id in role_comp.role_map.keys() {
        if role_comp.is_trial_role(role_id) || !(1000..=1999).contains(&role_id) {
            continue;
        }
        let motion_ids: Vec<FavorItem> = assets
            .tables
            .motion
            .items
            .iter()
            .filter(|m| m.RoleId == role_id)
            .map(|m| FavorItem {
                Id: m.Id,
                Status: Some(FavorItemStatus::ItemUnLocked),
            })
            .collect();
        if !motion_ids.is_empty() {
            motion_list.push(RoleMotion {
                RoleId: role_id,
                MotionIds: motion_ids,
            });
        }
    }
    sink.push(
        RoleMotionListNotify::MSG_ID,
        &RoleMotionListNotify {
            MotionList: motion_list,
            RoleConditionInfoMap: Default::default(),
        },
    );
}
