use std::time::{SystemTime, UNIX_EPOCH};

use sonoro_gamesv_data::Assets;
use sonoro_gamesv_data::tables::fly_skin_config::FlySkinConfig;
use sonoro_gamesv_data::tables::role_skin::RoleSkin;
use sonoro_gamesv_fs::cosmetic_info::CosmeticInfo;
use sonoro_gamesv_fs::role_info::RoleInfo;
use sonoro_gamesv_logic::component::comp_util;
use sonoro_gamesv_logic::component::entity::small::{
    BaseSkinComponent, ConfigType, EntityType, OrnamentComponent, WeaponSkinComponent,
};
use sonoro_gamesv_logic::component::player::PlayerRoleComponent;
use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::events::{BuffAdditionEntry, EntityHandle, Event};
use sonoro_gamesv_logic::handlers::buff;
use sonoro_gamesv_logic::helpers::{cosmetics, quest_engine, reward, role as role_helper};
use sonoro_proto::enums::{EquipPos, ErrorCode};
use sonoro_proto::pb::{
    CalabashSkinComponentPb,
    CalabashSkinTakeOnRequest, CalabashSkinTakeOnResponse, ChangeOrnamentRequest,
    ChangeOrnamentResponse, EntityCalabashSkinChangeNotify, EntityDressOrnamentChangeNotify,
    EntityEquipSkinChangeNotify, EntityFlySkinChangeData, EquipFlySkinData,
    EquipSkinTakeOnRequest, EquipSkinTakeOnResponse, EquipSkinUnLoadRequest,
    EquipSkinUnLoadResponse, FlySkinAllUnLoadRequest,
    FlySkinAllUnLoadResponse, FlySkinConfigData, FlySkinUnLoadRequest, FlySkinUnLoadResponse,
    FlySkinWearAllRoleRequest, FlySkinWearAllRoleResponse, FlySkinWearRequest,
    FlySkinWearResponse, LoadCalabashSkinInfoRequest, LoadCalabashSkinInfoResponse,
    LoadEquipInfoRequest, LoadEquipInfoResponse, OrnamentDressInfoUpdateNotify,
    PhantomMusicSaveRequest, PhantomMusicSaveResponse, PhantomMusicSwitchRequest,
    PhantomMusicSwitchResponse, PhantomMusicUnlockRequest, PhantomMusicUnlockResponse,
    RoleLoadEquipData, RoleSkinChangeRequest, RoleSkinChangeResponse, RoleSkinEquipData,
    RoleSkinRequest, RoleSkinResponse,
    SkinRewardRequest, SkinRewardResponse, SoarWingOrParaglidingSkinChangeNotify,
    WeaponSkinComponentPb, WeaponSkinUnLoadNotify,
};

use super::State;
use sonoro_gamesv_logic::sink::PushSink;

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_millis() as i64)
}

fn build_fly_equip_data(role_comp: &PlayerRoleComponent, skin_id: i32) -> Vec<EquipFlySkinData> {
    role_comp
        .role_map
        .keys()
        .map(|&role_id| EquipFlySkinData {
            RoleId: role_id,
            SkinId: skin_id,
        })
        .collect()
}

fn find_role_skin(assets: &Assets, role_id: i32, skin_id: i32) -> Option<RoleSkin> {
    let skin = assets.tables.role_skin.get_data_by_id(&skin_id)?;
    if skin.RoleId == role_id {
        Some(skin.into_owned())
    } else {
        None
    }
}

fn role_default_skin(assets: &Assets, role_id: i32) -> Option<i32> {
    assets
        .tables
        .role_info
        .get_data_by_id(&role_id)
        .map(|r| r.SkinId)
}

fn role_id_for_skin(assets: &Assets, role_skin_id: i32) -> Option<i32> {
    if let Some(skin) = assets.tables.role_skin.get_data_by_id(&role_skin_id) {
        return Some(skin.RoleId);
    }
    assets
        .tables
        .role_info
        .items
        .iter()
        .find(|r| r.SkinId == role_skin_id)
        .map(|r| r.Id)
}

fn set_fly_skin(role: &mut RoleInfo, skin: &FlySkinConfig) {
    if skin.SkinType == 0 {
        role.soar_skin_id = skin.Id;
    } else {
        role.paragliding_skin_id = skin.Id;
    }
}

fn push_entity_fly_skin_change(
    state: &mut State,
    role_id: i32,
    skin: &FlySkinConfig,
) -> Result<(), String> {
    let Some(scene) = state.scene.as_mut() else {
        return Ok(());
    };
    let mut net_id_opt = None;
    for storage in &mut scene.entities {
        if storage.config.config_id != role_id {
            continue;
        }
        let base_skin = storage.base_skin.get_or_insert_with(BaseSkinComponent::default);
        if skin.SkinType == 0 {
            base_skin.soar_skin_id = skin.Id;
        } else {
            base_skin.paragliding_skin_id = skin.Id;
        }
        net_id_opt = Some(storage.net_id);
        break;
    }
    if let Some(net_id) = net_id_opt {
        scene
            .save_entity_by_net_id(&state.fs, net_id)
            .map_err(|e| e.to_string())?;
        state.conn.push(
            SoarWingOrParaglidingSkinChangeNotify::MSG_ID,
            &SoarWingOrParaglidingSkinChangeNotify {
                FlySkinData: vec![EntityFlySkinChangeData {
                    EntityId: net_id,
                    FlySkinConfigData: vec![FlySkinConfigData {
                        SkinId: skin.Id,
                        FlySkinId: skin.SkinType,
                    }],
                }],
            },
        );
    }
    Ok(())
}

fn push_entity_weapon_skin_change(
    state: &mut State,
    role_id: i32,
    skin_id: i32,
) -> Result<(), String> {
    let Some(scene) = state.scene.as_mut() else {
        return Ok(());
    };
    let mut net_id_opt = None;
    for storage in &mut scene.entities {
        if storage.config.config_id != role_id {
            continue;
        }
        if let Some(ws) = storage.weapon_skin.as_mut() {
            ws.skin_id = skin_id;
        } else {
            storage.weapon_skin = Some(WeaponSkinComponent { skin_id });
        }
        net_id_opt = Some(storage.net_id);
        break;
    }
    if let Some(net_id) = net_id_opt {
        scene
            .save_entity_by_net_id(&state.fs, net_id)
            .map_err(|e| e.to_string())?;
        state.conn.push(
            EntityEquipSkinChangeNotify::MSG_ID,
            &EntityEquipSkinChangeNotify {
                EntityId: net_id,
                WeaponSkinComponentPb: Some(WeaponSkinComponentPb {
                    WeaponSkinId: skin_id,
                }),
            },
        );
    }
    Ok(())
}

fn push_weapon_skin_equip_take_on_notify(state: &mut State, role_id: i32, skin_id: i32) {
    state.conn.push(
        sonoro_proto::pb::EquipTakeOnNotify::MSG_ID,
        &sonoro_proto::pb::EquipTakeOnNotify {
            DataList: vec![RoleLoadEquipData {
                RoleId: role_id,
                Pos: Some(EquipPos::WeaponSkin),
                EquipIncId: skin_id,
            }],
        },
    );
}

#[allow(clippy::too_many_arguments)]
fn refresh_role_ornament_entity(
    state: &mut State,
    events: &mut EventQueue,
    role_id: i32,
    role: &RoleInfo,
    stale_ornament_ids: &[i32],
    now_ms_val: i64,
) -> Result<(), String> {
    let State {
        conn,
        fs,
        assets,
        scene: scene_opt,
        buff_timers,
        ..
    } = state;
    let Some(scene) = scene_opt.as_mut() else {
        return Ok(());
    };

    let Some(idx) = scene
        .entities
        .iter()
        .position(|s| s.config.config_id == role_id)
    else {
        return Ok(());
    };

    let net_id = scene.entities[idx].net_id;
    let entity = EntityHandle { index: idx, net_id };
    let ornament_ids = cosmetics::build_ornament_ids_for_role_skin(role, role.role_skin_id);
    scene.entities[idx].ornament = Some(OrnamentComponent {
        ornament_ids: ornament_ids.clone(),
    });
    let ornament_pb = scene.entities[idx]
        .ornament
        .as_ref()
        .expect("just set")
        .to_proto();

    let stale_buffs = cosmetics::build_ornament_buffs_for_ids(assets, stale_ornament_ids);
    for entry in &stale_buffs {
        buff::remove_buff_from_entity_by_id(
            conn,
            events,
            scene,
            assets,
            buff_timers,
            entity,
            entry.id,
        );
    }

    let ornament_buffs = cosmetics::build_ornament_buffs_for_ids(assets, &ornament_ids);
    let add_entries: Vec<BuffAdditionEntry> = ornament_buffs
        .into_iter()
        .filter(|entry| {
            scene.entities[idx]
                .buffs
                .as_ref()
                .is_none_or(|b| b.get_by_buff_id(entry.id).is_none())
        })
        .collect();
    if !add_entries.is_empty() {
        buff::add_buff_to_entity(
            conn,
            events,
            scene,
            buff_timers,
            assets,
            entity,
            entity,
            &add_entries,
        );
    }

    let born_ids = cosmetics::build_ornament_born_buff_ids_for_ids(assets, &ornament_ids);
    if let Some(buffs) = scene.entities[idx].buffs.as_mut() {
        buffs.client_born_message_id = if born_ids.is_empty() { 0 } else { net_id };
        buffs.client_born_buff_ids = born_ids;
    }

    buff_timers.sync_entity_left_durations(scene, net_id, now_ms_val);
    scene
        .save_entity_by_net_id(fs, net_id)
        .map_err(|e| e.to_string())?;
    conn.push(
        EntityDressOrnamentChangeNotify::MSG_ID,
        &EntityDressOrnamentChangeNotify {
            EntityId: net_id,
            OrnamentComponentPb: Some(ornament_pb),
        },
    );
    Ok(())
}

fn push_ornament_equip_map(state: &mut State) {
    let infos = cosmetics::build_ornament_equip_map(&state.player_components.role);
    state.conn.push(
        OrnamentDressInfoUpdateNotify::MSG_ID,
        &OrnamentDressInfoUpdateNotify {
            OrnamentDressInfos: infos,
        },
    );
}

fn equipped_calabash_skin_id(assets: &Assets, role_comp: &PlayerRoleComponent) -> i32 {
    let mut fallback = 0i32;
    for (&role_id, role) in &role_comp.role_map {
        if role.calabash_skin_id == 0 {
            continue;
        }
        if role_helper::is_main_role(assets, role_id) {
            return role.calabash_skin_id;
        }
        if fallback == 0 {
            fallback = role.calabash_skin_id;
        }
    }
    fallback
}

fn calabash_skin_id_list(assets: &Assets) -> Vec<i32> {
    assets
        .tables
        .calabash_skin
        .items
        .iter()
        .map(|s| s.Id)
        .collect()
}

fn push_entity_calabash_skin_change(state: &mut State, skin_id: i32) -> Result<(), String> {
    let Some(scene) = state.scene.as_mut() else {
        return Ok(());
    };
    let mut changed_ids = Vec::new();
    for storage in &mut scene.entities {
        // Player character entities have non-zero base_skin / calabash_skin
        if storage.config.entity_type != EntityType::Player
            || storage.config.config_type != ConfigType::Character
        {
            continue;
        }
        storage.calabash_skin = Some(
            sonoro_gamesv_logic::component::entity::small::CalabashSkinComponent { skin_id },
        );
        changed_ids.push(storage.net_id);
    }
    for net_id in changed_ids {
        scene
            .save_entity_by_net_id(&state.fs, net_id)
            .map_err(|e| e.to_string())?;
        state.conn.push(
            EntityCalabashSkinChangeNotify::MSG_ID,
            &EntityCalabashSkinChangeNotify {
                EntityId: net_id,
                CalabashSkinCoponent: Some(CalabashSkinComponentPb {
                    CalabashSkinId: skin_id,
                }),
            },
        );
    }
    Ok(())
}

pub fn handle_load_calabash_skin_info_request(
    state: &mut State,
    _req: LoadCalabashSkinInfoRequest,
) -> Result<LoadCalabashSkinInfoResponse, String> {
    Ok(LoadCalabashSkinInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        EquipSkinId: equipped_calabash_skin_id(&state.assets, &state.player_components.role),
        SkinItemIds: calabash_skin_id_list(&state.assets),
    })
}

pub fn handle_calabash_skin_take_on_request(
    state: &mut State,
    events: &mut EventQueue,
    req: CalabashSkinTakeOnRequest,
) -> Result<CalabashSkinTakeOnResponse, String> {
    let skin_id = req.SkinId;
    if skin_id != 0
        && state
            .assets
            .tables
            .calabash_skin
            .get_data_by_id(&skin_id)
            .is_none()
    {
        return Ok(CalabashSkinTakeOnResponse {
            ErrorCode: Some(ErrorCode::CalabashSkinUnLockErr),
            SkinId: 0,
        });
    }

    let mut changed_roles = Vec::new();
    let all_role_ids: Vec<i32> = state.player_components.role.role_map.keys().copied().collect();
    for role_id in all_role_ids {
        if state.player_components.role.is_trial_role(role_id) {
            continue;
        }
        if let Some(role) = state.player_components.role.role_map.get_mut(&role_id)
            && role.calabash_skin_id != skin_id
        {
            role.calabash_skin_id = skin_id;
            changed_roles.push(role_id);
        }
    }

    for role_id in changed_roles {
        role_helper::save_role_info(&state.fs, &state.player_components.role, role_id)
            .map_err(|e| e.to_string())?;
        events.enqueue(Event::RoleInfoModified {
            player_id: state.player_id,
            role_id,
        });
    }

    push_entity_calabash_skin_change(state, skin_id)?;
    Ok(CalabashSkinTakeOnResponse {
        ErrorCode: Some(ErrorCode::Success),
        SkinId: skin_id,
    })
}

pub fn handle_role_skin_request(
    state: &mut State,
    _req: RoleSkinRequest,
) -> Result<RoleSkinResponse, String> {
    Ok(RoleSkinResponse {
        SkinIds: CosmeticInfo::int_list(&state.player_components.cosmetic.info.role_skins),
    })
}

pub fn handle_load_equip_info_request(
    state: &mut State,
    _req: LoadEquipInfoRequest,
) -> Result<LoadEquipInfoResponse, String> {
    let equip_list = cosmetics::build_weapon_skin_current_equip_list(
        &state.assets,
        &state.player_components.role,
    )
    .into_iter()
    .map(|d| RoleSkinEquipData {
        RoleID: d.RoleId,
        SkinItemId: d.SkinId,
    })
    .collect();
    Ok(LoadEquipInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        EquipList: equip_list,
    })
}

pub fn handle_equip_skin_take_on_request(
    state: &mut State,
    events: &mut EventQueue,
    req: EquipSkinTakeOnRequest,
) -> Result<EquipSkinTakeOnResponse, String> {
    let Some(data) = req.Data else {
        return Ok(EquipSkinTakeOnResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
            ..Default::default()
        });
    };
    let role_id = data.RoleID;
    let skin_id = data.SkinItemId;
    if state.player_components.role.is_trial_role(role_id)
        || !CosmeticInfo::has(&state.player_components.cosmetic.info.weapon_skins, skin_id)
        || !cosmetics::is_weapon_skin_compatible(&state.assets, role_id, skin_id)
    {
        return Ok(EquipSkinTakeOnResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
            ..Default::default()
        });
    }
    let Some(role) = state.player_components.role.role_map.get_mut(&role_id) else {
        return Ok(EquipSkinTakeOnResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
            ..Default::default()
        });
    };
    role.weapon_skin_id = skin_id;
    events.enqueue(Event::RoleInfoModified {
        player_id: state.player_id,
        role_id,
    });
    events.enqueue(Event::UpdateFormations {
        player_id: state.player_id,
        formation_id: 0,
        role_ids: Vec::new(),
    });
    push_entity_weapon_skin_change(state, role_id, skin_id)?;
    push_weapon_skin_equip_take_on_notify(state, role_id, skin_id);

    Ok(EquipSkinTakeOnResponse {
        ErrorCode: Some(ErrorCode::Success),
        DataList: vec![RoleSkinEquipData {
            RoleID: role_id,
            SkinItemId: skin_id,
        }],
    })
}

pub fn handle_equip_skin_unload_request(
    state: &mut State,
    events: &mut EventQueue,
    req: EquipSkinUnLoadRequest,
) -> Result<EquipSkinUnLoadResponse, String> {
    let role_id = req.RoleID;
    if state.player_components.role.is_trial_role(role_id) {
        return Ok(EquipSkinUnLoadResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    }
    let Some(role) = state.player_components.role.role_map.get_mut(&role_id) else {
        return Ok(EquipSkinUnLoadResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    };
    role.weapon_skin_id = 0;
    events.enqueue(Event::RoleInfoModified {
        player_id: state.player_id,
        role_id,
    });
    events.enqueue(Event::UpdateFormations {
        player_id: state.player_id,
        formation_id: 0,
        role_ids: Vec::new(),
    });
    push_entity_weapon_skin_change(state, role_id, 0)?;
    state.conn.push(
        WeaponSkinUnLoadNotify::MSG_ID,
        &WeaponSkinUnLoadNotify {
            RoleId: role_id,
            SkinId: 0,
        },
    );
    Ok(EquipSkinUnLoadResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_fly_skin_wear_request(
    state: &mut State,
    events: &mut EventQueue,
    req: FlySkinWearRequest,
) -> Result<FlySkinWearResponse, String> {
    if !CosmeticInfo::has(&state.player_components.cosmetic.info.fly_skins, req.SkinId) {
        return Ok(FlySkinWearResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    }
    let Some(skin) = state
        .assets
        .tables
        .fly_skin_config
        .get_data_by_id(&req.SkinId)
        .map(|r| r.into_owned())
    else {
        return Ok(FlySkinWearResponse {
            ErrorCode: Some(ErrorCode::FlySkinItemNoConfig),
        });
    };
    if state.player_components.role.is_trial_role(req.RoleId) {
        return Ok(FlySkinWearResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    }
    let Some(role) = state.player_components.role.role_map.get_mut(&req.RoleId) else {
        return Ok(FlySkinWearResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    };
    set_fly_skin(role, &skin);
    events.enqueue(Event::RoleInfoModified {
        player_id: state.player_id,
        role_id: req.RoleId,
    });
    push_entity_fly_skin_change(state, req.RoleId, &skin)?;
    Ok(FlySkinWearResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_fly_skin_wear_all_role_request(
    state: &mut State,
    events: &mut EventQueue,
    req: FlySkinWearAllRoleRequest,
) -> Result<FlySkinWearAllRoleResponse, String> {
    if !CosmeticInfo::has(&state.player_components.cosmetic.info.fly_skins, req.SkinId) {
        return Ok(FlySkinWearAllRoleResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
            ..Default::default()
        });
    }
    let Some(skin) = state
        .assets
        .tables
        .fly_skin_config
        .get_data_by_id(&req.SkinId)
        .map(|r| r.into_owned())
    else {
        return Ok(FlySkinWearAllRoleResponse {
            ErrorCode: Some(ErrorCode::FlySkinItemNoConfig),
            ..Default::default()
        });
    };

    let role_ids: Vec<i32> = state
        .player_components
        .role
        .role_map
        .keys()
        .copied()
        .filter(|&id| !state.player_components.role.is_trial_role(id))
        .collect();
    for role_id in role_ids {
        if let Some(role) = state.player_components.role.role_map.get_mut(&role_id) {
            set_fly_skin(role, &skin);
        }
        events.enqueue(Event::RoleInfoModified {
            player_id: state.player_id,
            role_id,
        });
        push_entity_fly_skin_change(state, role_id, &skin)?;
    }

    Ok(FlySkinWearAllRoleResponse {
        ErrorCode: Some(ErrorCode::Success),
        FlySkinData: build_fly_equip_data(&state.player_components.role, req.SkinId),
    })
}

pub fn handle_role_skin_change_request(
    state: &mut State,
    events: &mut EventQueue,
    req: RoleSkinChangeRequest,
) -> Result<RoleSkinChangeResponse, String> {
    let now_ms_val = now_ms();
    if state.player_components.role.is_trial_role(req.RoleId) {
        return Ok(RoleSkinChangeResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    }
    if !state.player_components.role.role_map.contains_key(&req.RoleId) {
        return Ok(RoleSkinChangeResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    }

    let skin_id = if req.SkinId == 0 {
        let Some(default_id) = role_default_skin(&state.assets, req.RoleId) else {
            return Ok(RoleSkinChangeResponse {
                ErrorCode: Some(ErrorCode::ErrRoleSkinConfig),
            });
        };
        default_id
    } else {
        if !CosmeticInfo::has(&state.player_components.cosmetic.info.role_skins, req.SkinId) {
            return Ok(RoleSkinChangeResponse {
                ErrorCode: Some(ErrorCode::ErrRoleSkinNotMatch),
            });
        }
        let Some(skin) = find_role_skin(&state.assets, req.RoleId, req.SkinId) else {
            return Ok(RoleSkinChangeResponse {
                ErrorCode: Some(ErrorCode::ErrRoleSkinNotMatch),
            });
        };
        skin.Id
    };

    let skin = find_role_skin(&state.assets, req.RoleId, skin_id);
    let mut weapon_skin_changed = false;
    let mut applied_weapon_skin = 0i32;
    if let Some(role) = state.player_components.role.role_map.get_mut(&req.RoleId) {
        role.role_skin_id = skin_id;
        if req.IsWearWeaponSkin
            && let Some(role_skin) = skin.as_ref()
            && CosmeticInfo::has(
                &state.player_components.cosmetic.info.weapon_skins,
                role_skin.SuitWeaponSkinId,
            )
            && cosmetics::is_weapon_skin_compatible(
                &state.assets,
                req.RoleId,
                role_skin.SuitWeaponSkinId,
            )
        {
            role.weapon_skin_id = role_skin.SuitWeaponSkinId;
            applied_weapon_skin = role.weapon_skin_id;
            weapon_skin_changed = true;
        }
    }

    role_helper::save_role_info(&state.fs, &state.player_components.role, req.RoleId)
        .map_err(|e| e.to_string())?;
    {
        let State {
            conn,
            fs,
            assets,
            player_components,
            scene: scene_opt,
            buff_timers,
            ..
        } = state;
        if let Some(scene) = scene_opt.as_mut() {
            role_helper::reset_roles(
                scene,
                fs,
                assets,
                &mut player_components.role,
                &player_components.weapon,
                &player_components.echo,
                conn,
                Some(&[req.RoleId]),
                buff_timers,
                now_ms_val,
            )
            .map_err(|e| e.to_string())?;
            buff_timers.ensure_all_registered(scene, assets, now_ms_val);
            buff_timers.sync_all_left_durations(scene, now_ms_val);
            scene.save(fs).map_err(|e| e.to_string())?;
        }
    }

    if weapon_skin_changed {
        push_weapon_skin_equip_take_on_notify(state, req.RoleId, applied_weapon_skin);
    }
    events.enqueue(Event::RoleInfoModified {
        player_id: state.player_id,
        role_id: req.RoleId,
    });
    events.enqueue(Event::UpdateFormations {
        player_id: state.player_id,
        formation_id: 0,
        role_ids: Vec::new(),
    });
    push_ornament_equip_map(state);

    Ok(RoleSkinChangeResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_change_ornament_request(
    state: &mut State,
    events: &mut EventQueue,
    req: ChangeOrnamentRequest,
) -> Result<ChangeOrnamentResponse, String> {
    let now_ms_val = now_ms();
    let role_skin_id = req.RoleSkinId;
    let ornament_id = req.OrnamentId;
    let Some(role_id) = role_id_for_skin(&state.assets, role_skin_id) else {
        return Ok(ChangeOrnamentResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    };

    if ornament_id == 0
        || !CosmeticInfo::has(&state.player_components.cosmetic.info.ornaments, ornament_id)
        || cosmetics::ornament_for_role_skin(&state.assets, role_skin_id, ornament_id).is_none()
    {
        return Ok(ChangeOrnamentResponse {
            ErrorCode: Some(ErrorCode::ErrOrnamentConfig),
        });
    }

    if state.player_components.role.is_trial_role(role_id) {
        return Ok(ChangeOrnamentResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    }
    let Some(role_snapshot) = state.player_components.role.role_map.get(&role_id).cloned() else {
        return Ok(ChangeOrnamentResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    };

    let stale_ornament_ids =
        cosmetics::build_ornament_ids_for_role_skin(&role_snapshot, role_skin_id);
    let assets_ref = state.assets.clone();
    let changed = {
        let role = state
            .player_components
            .role
            .role_map
            .get_mut(&role_id)
            .expect("checked above");
        if req.IsDress {
            role.dress_ornament(&assets_ref, role_skin_id, ornament_id)
                .map_err(|e| e.to_string())?
        } else {
            role.undress_ornament(role_skin_id, ornament_id)
        }
    };

    if changed {
        let updated_role = state
            .player_components
            .role
            .role_map
            .get(&role_id)
            .expect("checked above")
            .clone();
        events.enqueue(Event::RoleInfoModified {
            player_id: state.player_id,
            role_id,
        });
        events.enqueue(Event::UpdateFormations {
            player_id: state.player_id,
            formation_id: 0,
            role_ids: Vec::new(),
        });
        refresh_role_ornament_entity(
            state,
            events,
            role_id,
            &updated_role,
            &stale_ornament_ids,
            now_ms_val,
        )?;
    }

    push_ornament_equip_map(state);
    Ok(ChangeOrnamentResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_fly_skin_unload_request(
    state: &mut State,
    events: &mut EventQueue,
    req: FlySkinUnLoadRequest,
) -> Result<FlySkinUnLoadResponse, String> {
    if !state.player_components.role.is_trial_role(req.RoleId)
        && let Some(role) = state.player_components.role.role_map.get_mut(&req.RoleId)
    {
        if role.soar_skin_id == req.SkinId {
            role.soar_skin_id = 0;
        }
        if role.paragliding_skin_id == req.SkinId {
            role.paragliding_skin_id = 0;
        }
        let role_path = format!(
            "player/{}/role/{}",
            state.player_components.role.player_id, req.RoleId
        );
        comp_util::save_struct(&state.fs, role, &role_path).map_err(|e| e.to_string())?;
        events.enqueue(Event::RoleInfoModified {
            player_id: state.player_id,
            role_id: req.RoleId,
        });
        events.enqueue(Event::UpdateFormations {
            player_id: state.player_id,
            formation_id: 0,
            role_ids: Vec::new(),
        });
    }
    Ok(FlySkinUnLoadResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_fly_skin_all_unload_request(
    state: &mut State,
    events: &mut EventQueue,
    req: FlySkinAllUnLoadRequest,
) -> Result<FlySkinAllUnLoadResponse, String> {
    let mut unloaded = Vec::new();
    let role_ids: Vec<i32> = state.player_components.role.role_map.keys().copied().collect();
    for role_id in role_ids {
        if state.player_components.role.is_trial_role(role_id) {
            continue;
        }
        let Some(role) = state.player_components.role.role_map.get_mut(&role_id) else {
            continue;
        };
        let mut changed = false;
        if req.SkinType == 0 && role.soar_skin_id != 0 {
            role.soar_skin_id = 0;
            changed = true;
        } else if req.SkinType != 0 && role.paragliding_skin_id != 0 {
            role.paragliding_skin_id = 0;
            changed = true;
        }
        if changed {
            let role_path = format!(
                "player/{}/role/{role_id}",
                state.player_components.role.player_id
            );
            comp_util::save_struct(&state.fs, role, &role_path).map_err(|e| e.to_string())?;
            unloaded.push(role_id);
            events.enqueue(Event::RoleInfoModified {
                player_id: state.player_id,
                role_id,
            });
        }
    }
    if !unloaded.is_empty() {
        events.enqueue(Event::UpdateFormations {
            player_id: state.player_id,
            formation_id: 0,
            role_ids: Vec::new(),
        });
    }
    Ok(FlySkinAllUnLoadResponse {
        ErrorCode: Some(ErrorCode::Success),
        UnLoadSkinRoles: unloaded,
    })
}

pub fn handle_skin_reward_request(
    state: &mut State,
    req: SkinRewardRequest,
) -> Result<SkinRewardResponse, String> {
    if req.RewardId <= 0
        || state
            .assets
            .tables
            .role_skin
            .get_data_by_id(&req.RewardId)
            .is_none()
        || !CosmeticInfo::has(&state.player_components.cosmetic.info.role_skins, req.RewardId)
    {
        return Ok(SkinRewardResponse {
            ErrorCode: Some(ErrorCode::ErrSkinRewardNotFound),
        });
    }
    let k = format!("skin_reward:{}", req.RewardId);
    if state.player_components.progress.info.mark_claim(&k) {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        let mut rctx = state.reward_context();
        reward::grant_items(
            &mut rctx,
            0,
            true,
            &[reward::RewardItem {
                item_id: 3,
                count: 50,
            }],
            19500,
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(SkinRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_phantom_music_save_request(
    state: &mut State,
    req: PhantomMusicSaveRequest,
) -> Result<PhantomMusicSaveResponse, String> {
    state
        .player_components
        .progress
        .info
        .set_key_int("phantom_music:cur", i64::from(req.Music));
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(PhantomMusicSaveResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_phantom_music_switch_request(
    state: &mut State,
    events: &mut EventQueue,
    req: PhantomMusicSwitchRequest,
) -> Result<PhantomMusicSwitchResponse, String> {
    state
        .player_components
        .progress
        .info
        .set_key_int("phantom_music:cur", i64::from(req.MusicId));
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    if state.scene.is_some() {
        let mut qctx = state.quest_context(events);
        let _ = quest_engine::events::on_condition_state_changed(&mut qctx);
    }
    Ok(PhantomMusicSwitchResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_phantom_music_unlock_request(
    state: &mut State,
    req: PhantomMusicUnlockRequest,
) -> Result<PhantomMusicUnlockResponse, String> {
    let mut added = Vec::new();
    let mut dirty = false;
    for id in req.Items {
        let k = format!("phantom_music:unl:{id}");
        if state.player_components.progress.info.mark_claim(&k) {
            dirty = true;
            added.push(id);
        }
    }
    if dirty {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(PhantomMusicUnlockResponse {
        ErrorCode: Some(ErrorCode::Success),
        AddMusicIds: added,
    })
}
