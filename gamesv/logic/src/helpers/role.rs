use std::collections::BTreeMap;
use std::io;

use sonoro_common::fs::FileSystem;
use sonoro_gamesv_data::Assets;
use sonoro_gamesv_data::tables::skill_tree::SkillTree;
use sonoro_gamesv_fs::formation_info::Formation;
use sonoro_gamesv_fs::role_info::{RoleInfo, SkillNode};
use sonoro_gamesv_fs::special_item_incr;
use sonoro_gamesv_fs::weapon_item::WeaponItem;
use sonoro_proto::enums::{EAttributeType, ExitVehicleType, FavorItemStatus, LivingStatus};
use sonoro_proto::pb::{
    CombatCommon, CombatNotifyData, CombatNotifyDataMessageOneof, CombatReceiveData,
    CombatReceiveDataMessageOneof, CombatReceivePackNotify, EntityAddNotify,
    EntityLivingStatusNotify, EntityRemoveInfo, EntityRemoveNotify, FightRoleInfo, FightRoleInfos,
    GroupFormation, PlayerDeadNotify, RoleChangeUnlockNotify, RoleFavor, RoleFavorListNotify,
    RoleMotion, RoleMotionListNotify, UpdateGroupFormationNotify, VehicleUpdateNotify,
};
use sonoro_proto::wire::PbEnum;

use crate::component::comp_util;
use crate::component::entity::small::{ConfigType, EntityType};
use crate::component::player::{
    PlayerBasicComponent, PlayerEchoComponent, PlayerProgressComponent, PlayerRoleComponent,
    PlayerWeaponComponent,
};
use crate::event_queue::EventQueue;
use crate::events::Event;
use crate::fsm::types::wake_reason;
use crate::helpers::{attributes, entity_proto};
use crate::scene::Scene;
use crate::schedulers::BuffTimerScheduler;
use crate::sink::PushSink;
use crate::templates::role_entity_templates;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MainRoleTransferResult {
    pub formation_changed: bool,
    pub basic_changed: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RoleRepairSummary {
    pub removed_leaks: usize,
    pub fixed_roles: usize,
    pub fixed_weapons: usize,
    pub fixed_main_role: bool,
    pub fixed_formation: bool,
}

pub fn main_role_id_for_element(assets: &Assets, gender: i32, element_id: i32) -> Option<i32> {
    for config in &assets.tables.main_role_config.items {
        if config.Gender != gender {
            continue;
        }
        let Some(role_config) = assets.tables.role_info.get_data_by_id(&config.Id) else {
            continue;
        };
        if role_config.ElementId == element_id {
            return Some(config.Id);
        }
    }
    None
}

pub fn main_role_unlock_list(assets: &Assets, gender: i32) -> Vec<i32> {
    assets
        .tables
        .main_role_config
        .items
        .iter()
        .filter(|c| c.Gender == gender)
        .map(|c| c.Id)
        .collect()
}

pub fn is_main_role(assets: &Assets, role_id: i32) -> bool {
    assets
        .tables
        .main_role_config
        .get_data_by_id(&role_id)
        .is_some()
}

fn current_main_role_id(
    assets: &Assets,
    role_comp: &PlayerRoleComponent,
    gender: i32,
) -> Option<i32> {
    for config in &assets.tables.main_role_config.items {
        if config.Gender == gender && role_comp.role_map.contains_key(&config.Id) {
            return Some(config.Id);
        }
    }
    for config in &assets.tables.main_role_config.items {
        if role_comp.role_map.contains_key(&config.Id) {
            return Some(config.Id);
        }
    }
    None
}

pub fn selected_main_role_id(
    assets: &Assets,
    role_comp: &PlayerRoleComponent,
    gender: i32,
    selected_role_id: i32,
) -> Option<i32> {
    if selected_role_id != 0
        && is_main_role_for_gender(assets, gender, selected_role_id)
        && role_comp.role_map.contains_key(&selected_role_id)
    {
        return Some(selected_role_id);
    }
    current_main_role_id(assets, role_comp, gender)
}

pub fn is_main_role_for_gender(assets: &Assets, gender: i32, role_id: i32) -> bool {
    assets
        .tables
        .main_role_config
        .get_data_by_id(&role_id)
        .is_some_and(|c| c.Gender == gender)
}

pub fn current_formation_main_role_id(
    assets: &Assets,
    scene: &Scene,
    gender: i32,
) -> Option<i32> {
    let idx = usize::try_from(scene.formation_info.cur_formation).ok()?;
    let formation = scene.formation_info.formations.get(idx)?;
    if is_main_role_for_gender(assets, gender, formation.cur_role) {
        return Some(formation.cur_role);
    }
    for maybe_role in &formation.roles {
        let Some(role) = maybe_role else {
            continue;
        };
        if is_main_role_for_gender(assets, gender, role.role_id) {
            return Some(role.role_id);
        }
    }
    None
}

fn replace_formation_role(scene: &mut Scene, source_role_id: i32, target_role_id: i32) -> bool {
    if source_role_id == target_role_id {
        return false;
    }
    let mut changed = false;
    for formation in &mut scene.formation_info.formations {
        if formation.cur_role == source_role_id {
            formation.cur_role = target_role_id;
            changed = true;
        }
        for maybe_role in &mut formation.roles {
            if let Some(role) = maybe_role.as_mut()
                && role.role_id == source_role_id
            {
                role.role_id = target_role_id;
                changed = true;
            }
        }
    }
    changed
}

pub fn save_basic_info(fs: &FileSystem, basic_comp: &PlayerBasicComponent) -> io::Result<()> {
    comp_util::save_struct(
        fs,
        &basic_comp.info,
        &format!("player/{}/basic_info", basic_comp.player_id),
    )
}

pub fn save_role_info(
    fs: &FileSystem,
    role_comp: &PlayerRoleComponent,
    role_id: i32,
) -> io::Result<()> {
    if role_comp.is_trial_role(role_id) {
        return Ok(());
    }
    let Some(role_info) = role_comp.role_map.get(&role_id) else {
        return Ok(());
    };
    comp_util::save_struct(
        fs,
        role_info,
        &format!("player/{}/{}/{role_id}", role_comp.player_id, RoleInfo::DATA_DIR),
    )
}

pub fn set_selected_main_role(basic_comp: &mut PlayerBasicComponent, role_id: i32) -> bool {
    if basic_comp.info.selected_main_role_id == role_id {
        return false;
    }
    basic_comp.info.selected_main_role_id = role_id;
    true
}

fn replace_basic_show_role(
    basic_comp: &mut PlayerBasicComponent,
    source_role_id: i32,
    target_role_id: i32,
) -> bool {
    if source_role_id == target_role_id {
        return false;
    }
    let mut changed = false;
    for role_id in &mut basic_comp.info.role_show_list {
        if *role_id == source_role_id {
            *role_id = target_role_id;
            changed = true;
        }
    }
    changed
}

fn clear_weapon_owners_except(
    weapon_comp: &mut PlayerWeaponComponent,
    role_id: i32,
    keep_incr_id: i32,
) -> bool {
    let mut changed = false;
    for (&incr_id, weapon) in &mut weapon_comp.weapon_map {
        if incr_id == keep_incr_id {
            continue;
        }
        if weapon.role_id == Some(role_id) {
            weapon.role_id = None;
            changed = true;
        }
    }
    changed
}

fn first_owned_weapon_id(weapon_comp: &PlayerWeaponComponent, role_id: i32) -> Option<i32> {
    weapon_comp
        .weapon_map
        .iter()
        .find_map(|(&k, w)| (w.role_id == Some(role_id)).then_some(k))
}

fn replace_echo_role_id(
    echo_comp: &mut PlayerEchoComponent,
    source_role_id: i32,
    target_role_id: i32,
) -> bool {
    if source_role_id == target_role_id {
        return false;
    }
    let mut changed = false;
    for echo in echo_comp.echo_map.values_mut() {
        if echo.role_id == Some(target_role_id) {
            echo.role_id = None;
            changed = true;
        }
    }
    for echo in echo_comp.echo_map.values_mut() {
        if echo.role_id == Some(source_role_id) {
            echo.role_id = Some(target_role_id);
            changed = true;
        }
    }
    changed
}

fn normalize_target_role_info(
    assets: &Assets,
    role_info: &mut RoleInfo,
    target_role_id: i32,
) -> Result<(), &'static str> {
    let target_config = assets
        .tables
        .role_info
        .get_data_by_id(&target_role_id)
        .ok_or("RoleNotFound")?;
    role_info.role_skin_id = target_config.SkinId;
    role_info.cur_model = 0;
    role_info.models.clear();
    role_info.reset_properties(assets, target_role_id);
    Ok(())
}

fn matching_skill_tree_node(
    assets: &Assets,
    node_group: i32,
    node_index: i32,
    node_type: i32,
) -> Option<&SkillTree> {
    assets.tables.skill_tree.items.iter().find(|n| {
        n.NodeGroup == node_group && n.NodeIndex == node_index && n.NodeType == node_type
    })
}

fn max_resonant_chain_index(assets: &Assets, group_id: i32) -> i32 {
    assets
        .tables
        .resonant_chain
        .items
        .iter()
        .filter(|c| c.GroupId == group_id)
        .map(|c| c.GroupIndex)
        .max()
        .unwrap_or(0)
}

fn copy_main_role_progression(
    assets: &Assets,
    source_role_id: i32,
    source_role: &RoleInfo,
    target_role_id: i32,
    target_role: &mut RoleInfo,
) -> Result<(), &'static str> {
    let source_config = assets
        .tables
        .role_info
        .get_data_by_id(&source_role_id)
        .ok_or("RoleNotFound")?;
    let target_config = assets
        .tables
        .role_info
        .get_data_by_id(&target_role_id)
        .ok_or("RoleNotFound")?;

    let mut skills = Vec::new();
    let mut nodes = Vec::new();

    for target_node in &assets.tables.skill_tree.items {
        if target_node.NodeGroup != target_config.SkillTreeGroupId {
            continue;
        }
        let source_node = matching_skill_tree_node(
            assets,
            source_config.SkillTreeGroupId,
            target_node.NodeIndex,
            target_node.NodeType,
        );
        let default_active =
            target_node.Condition.is_empty() && target_node.UnLockCondition == 0;
        let source_state = source_node.and_then(|node| {
            source_role
                .skill_node_state
                .iter()
                .find(|s| s.node_id == node.Id)
        });
        let active = source_state.map_or(default_active, |s| s.active);
        let source_level = source_node.and_then(|node| {
            (node.SkillId != 0)
                .then(|| {
                    source_role
                        .skills
                        .iter()
                        .find(|&&(id, _)| id == node.SkillId)
                        .map(|&(_, lv)| lv)
                })
                .flatten()
        });
        let default_level = if active && target_node.SkillId != 0 {
            1
        } else {
            0
        };
        let level = source_level.unwrap_or(default_level);

        nodes.push(SkillNode {
            node_id: target_node.Id,
            active,
            skill_id: target_node.SkillId,
        });
        if active && target_node.SkillId != 0 {
            skills.push((target_node.SkillId, level));
        }
    }

    target_role.level = source_role.level;
    target_role.exp = source_role.exp;
    target_role.breakthrough = source_role.breakthrough;
    target_role.star = source_role.star;
    target_role.favor = source_role.favor;
    target_role.calabash_skin_id = source_role.calabash_skin_id;
    target_role.resonant_chain_group_index = source_role
        .resonant_chain_group_index
        .min(max_resonant_chain_index(
            assets,
            target_config.ResonantChainGroupId,
        ));
    target_role.skills = skills;
    target_role.skill_node_state = nodes;
    Ok(())
}

fn create_default_weapon(
    fs: &FileSystem,
    assets: &Assets,
    role_comp: &PlayerRoleComponent,
    weapon_comp: &mut PlayerWeaponComponent,
    role_id: i32,
) -> Result<i32, String> {
    let role_config = assets
        .tables
        .role_info
        .get_data_by_id(&role_id)
        .ok_or_else(|| "RoleNotFound".to_string())?;
    let weapon = WeaponItem {
        id: role_config.InitWeaponItemId,
        func_value: WeaponItem::default_func_value(assets, role_config.InitWeaponItemId),
        role_id: Some(role_id),
        ..Default::default()
    };
    let incr_id = special_item_incr::next(fs, role_comp.player_id).map_err(|e| e.to_string())?;
    weapon_comp.weapon_map.insert(incr_id, weapon);
    Ok(incr_id)
}

pub fn ensure_role_weapon(
    fs: &FileSystem,
    assets: &Assets,
    role_comp: &mut PlayerRoleComponent,
    weapon_comp: &mut PlayerWeaponComponent,
    role_id: i32,
) -> Result<bool, String> {
    let mut weapon_changed = false;

    let role_weapon = role_comp
        .role_map
        .get(&role_id)
        .ok_or_else(|| "RoleNotFound".to_string())?
        .weapon;

    if role_weapon != 0
        && let Some(weapon_info) = weapon_comp.weapon_map.get_mut(&role_weapon)
    {
        if weapon_info.role_id != Some(role_id) {
            weapon_info.role_id = Some(role_id);
            weapon_changed = true;
        }
        if clear_weapon_owners_except(weapon_comp, role_id, role_weapon) {
            weapon_changed = true;
        }
        if weapon_changed {
            PlayerWeaponComponent::save_all(fs, weapon_comp.player_id, &weapon_comp.weapon_map)
                .map_err(|e| e.to_string())?;
            role_comp.remove_equipment_cache(role_id);
        }
        return Ok(weapon_changed);
    }

    let role_changed = true;

    let next_weapon = if let Some(owned) = first_owned_weapon_id(weapon_comp, role_id) {
        if clear_weapon_owners_except(weapon_comp, role_id, owned) {
            weapon_changed = true;
        }
        owned
    } else {
        weapon_changed = true;
        create_default_weapon(fs, assets, role_comp, weapon_comp, role_id)?
    };
    if let Some(ri) = role_comp.role_map.get_mut(&role_id) {
        ri.weapon = next_weapon;
    }

    if role_changed {
        save_role_info(fs, role_comp, role_id).map_err(|e| e.to_string())?;
    }
    if weapon_changed {
        PlayerWeaponComponent::save_all(fs, weapon_comp.player_id, &weapon_comp.weapon_map)
            .map_err(|e| e.to_string())?;
        role_comp.remove_equipment_cache(role_id);
    }
    Ok(role_changed || weapon_changed)
}

fn move_selected_weapon(
    role_comp: &mut PlayerRoleComponent,
    weapon_comp: &mut PlayerWeaponComponent,
    source_role_id: i32,
    target_role_id: i32,
) -> bool {
    if source_role_id == target_role_id {
        return false;
    }
    let Some(source_weapon) = role_comp.role_map.get(&source_role_id).map(|r| r.weapon) else {
        return false;
    };
    if !role_comp.role_map.contains_key(&target_role_id) {
        return false;
    }

    let selected_weapon_id = if source_weapon != 0 && weapon_comp.weapon_map.contains_key(&source_weapon)
    {
        source_weapon
    } else {
        first_owned_weapon_id(weapon_comp, source_role_id).unwrap_or(0)
    };
    if selected_weapon_id == 0 {
        return false;
    }

    let mut changed = false;
    for (&k, w) in &mut weapon_comp.weapon_map {
        if k == selected_weapon_id {
            continue;
        }
        if w.role_id == Some(source_role_id) || w.role_id == Some(target_role_id) {
            w.role_id = None;
            changed = true;
        }
    }
    if let Some(w) = weapon_comp.weapon_map.get_mut(&selected_weapon_id)
        && w.role_id != Some(target_role_id)
    {
        w.role_id = Some(target_role_id);
        changed = true;
    }
    if let Some(target) = role_comp.role_map.get_mut(&target_role_id)
        && target.weapon != selected_weapon_id
    {
        target.weapon = selected_weapon_id;
        changed = true;
    }
    if let Some(source) = role_comp.role_map.get_mut(&source_role_id)
        && source.weapon == selected_weapon_id
    {
        source.weapon = 0;
        changed = true;
    }
    changed
}

#[allow(clippy::too_many_arguments)]
pub fn transfer_main_role_state(
    fs: &FileSystem,
    assets: &Assets,
    scene: Option<&mut Scene>,
    basic_comp: &mut PlayerBasicComponent,
    role_comp: &mut PlayerRoleComponent,
    weapon_comp: &mut PlayerWeaponComponent,
    echo_comp: &mut PlayerEchoComponent,
    source_role_id: i32,
    target_role_id: i32,
) -> Result<MainRoleTransferResult, String> {
    let mut result = MainRoleTransferResult::default();
    let mut weapon_changed = false;

    if !is_main_role(assets, target_role_id) {
        return Err("RoleNotFound".to_string());
    }
    if role_comp.role_map.contains_key(&source_role_id) {
        let _ = ensure_role_weapon(fs, assets, role_comp, weapon_comp, source_role_id)?;
    }
    let mut inserted_target = false;
    if let std::collections::btree_map::Entry::Vacant(entry) = role_comp.role_map.entry(target_role_id) {
        let new_role = RoleInfo::create_default(assets, target_role_id)?;
        entry.insert(new_role);
        inserted_target = true;
    }

    if source_role_id == target_role_id {
        result.basic_changed = set_selected_main_role(basic_comp, target_role_id);
        if inserted_target {
            if let Some(target_role) = role_comp.role_map.get_mut(&target_role_id) {
                normalize_target_role_info(assets, target_role, target_role_id)?;
            }
            let _ = ensure_role_weapon(fs, assets, role_comp, weapon_comp, target_role_id)?;
            let _ = role_comp.rebuild_equipment(assets, target_role_id, weapon_comp, echo_comp)?;
            save_role_info(fs, role_comp, target_role_id).map_err(|e| e.to_string())?;
        }
        return Ok(result);
    }

    result.basic_changed = set_selected_main_role(basic_comp, target_role_id)
        || replace_basic_show_role(basic_comp, source_role_id, target_role_id);
    let echo_changed = replace_echo_role_id(echo_comp, source_role_id, target_role_id);
    if let Some(sc) = scene {
        result.formation_changed = replace_formation_role(sc, source_role_id, target_role_id);
    }

    let source_snapshot = role_comp.role_map.get(&source_role_id).cloned();
    let target_role = role_comp
        .role_map
        .get_mut(&target_role_id)
        .ok_or_else(|| "RoleNotFound".to_string())?;
    if let Some(src) = source_snapshot.as_ref() {
        copy_main_role_progression(assets, source_role_id, src, target_role_id, target_role)?;
    }
    normalize_target_role_info(assets, target_role, target_role_id)?;
    let role_changed = true;

    if move_selected_weapon(role_comp, weapon_comp, source_role_id, target_role_id) {
        weapon_changed = true;
    }
    let _ = ensure_role_weapon(fs, assets, role_comp, weapon_comp, target_role_id)?;

    role_comp.remove_equipment_cache(source_role_id);
    role_comp.remove_equipment_cache(target_role_id);
    let _ = role_comp.rebuild_equipment(assets, target_role_id, weapon_comp, echo_comp)?;

    if role_changed {
        save_role_info(fs, role_comp, target_role_id).map_err(|e| e.to_string())?;
    }
    if role_comp.role_map.contains_key(&source_role_id) {
        save_role_info(fs, role_comp, source_role_id).map_err(|e| e.to_string())?;
    }
    if weapon_changed {
        PlayerWeaponComponent::save_all(fs, weapon_comp.player_id, &weapon_comp.weapon_map)
            .map_err(|e| e.to_string())?;
    }
    if echo_changed {
        PlayerEchoComponent::save_all(fs, echo_comp.player_id, &echo_comp.echo_map)
            .map_err(|e| e.to_string())?;
    }

    Ok(result)
}

#[allow(clippy::too_many_arguments)]
pub fn reset_roles<S: PushSink + ?Sized>(
    scene: &mut Scene,
    fs: &FileSystem,
    assets: &Assets,
    role_comp: &mut PlayerRoleComponent,
    weapon_comp: &PlayerWeaponComponent,
    echo_comp: &PlayerEchoComponent,
    sink: &mut S,
    role_ids: Option<&[i32]>,
    buff_timers: &mut BuffTimerScheduler,
    now_ms: i64,
) -> Result<(), &'static str> {
    let Some(instance_dungeon) = assets.tables.instance_dungeon.get_data_by_id(&scene.instance_id)
    else {
        tracing::error!(
            "player({}) last scene instance id {} doesn't exist",
            scene.player_id,
            scene.instance_id
        );
        return Ok(());
    };
    let Ok(cur_form_idx) = usize::try_from(scene.formation_info.cur_formation) else {
        return Ok(());
    };
    if cur_form_idx >= scene.formation_info.formations.len() {
        return Ok(());
    }

    let removed_entity_ids: Vec<i64> = scene.formation_info.formations[cur_form_idx]
        .roles
        .iter()
        .flatten()
        .filter(|r| role_ids.is_none_or(|ids| ids.contains(&r.role_id)))
        .map(|r| r.entity_id)
        .collect();

    let mut remove_infos = Vec::new();
    for entity_id in removed_entity_ids {
        let Some(&idx) = scene.net_id_map.get(&entity_id) else {
            continue;
        };
        if let Some(concomitant) = scene.entities[idx].concomitant.clone() {
            for cid in concomitant.vision_entity_id {
                let _ = scene.remove(fs, cid);
                remove_infos.push(EntityRemoveInfo {
                    EntityId: cid,
                    ..Default::default()
                });
            }
            for cid in concomitant.custom_entity_ids {
                let _ = scene.remove(fs, cid);
                remove_infos.push(EntityRemoveInfo {
                    EntityId: cid,
                    ..Default::default()
                });
            }
        }
        let _ = scene.remove(fs, entity_id);
        remove_infos.push(EntityRemoveInfo {
            EntityId: entity_id,
            ..Default::default()
        });
    }
    sink.push(
        EntityRemoveNotify::MSG_ID,
        &EntityRemoveNotify {
            IsRemove: true,
            RemoveInfos: remove_infos,
        },
    );

    let mut role_entity_pbs = Vec::new();
    let mut concom_entity_pbs = Vec::new();

    for slot_idx in 0..scene.formation_info.formations[cur_form_idx].roles.len() {
        let Some(role_id) = scene.formation_info.formations[cur_form_idx].roles[slot_idx]
            .as_ref()
            .map(|r| r.role_id)
        else {
            continue;
        };
        if role_ids.is_some_and(|ids| !ids.contains(&role_id)) {
            continue;
        }
        let entity = role_entity_templates::create_role_entity(
            fs,
            scene,
            scene.player_id,
            assets,
            role_comp,
            weapon_comp,
            echo_comp,
            &instance_dungeon,
            role_id,
        )?;
        if let Some(slot) = scene.formation_info.formations[cur_form_idx].roles[slot_idx].as_mut() {
            slot.entity_id = entity.net_id;
        }
        role_entity_pbs.push(entity_proto::build(
            assets,
            scene,
            buff_timers,
            entity.net_id,
            now_ms,
        )?);
        let concomitant_opt = scene
            .entity(entity.net_id)
            .and_then(|s| s.concomitant.clone());
        if let Some(concomitant) = concomitant_opt {
            for cid in concomitant
                .vision_entity_id
                .into_iter()
                .chain(concomitant.custom_entity_ids)
            {
                if !scene.net_id_map.contains_key(&cid) {
                    continue;
                }
                concom_entity_pbs.push(entity_proto::build(
                    assets,
                    scene,
                    buff_timers,
                    cid,
                    now_ms,
                )?);
            }
        }
    }

    sink.push(
        EntityAddNotify::MSG_ID,
        &EntityAddNotify {
            EntityPbs: role_entity_pbs,
            RemoveTagIds: false,
        },
    );
    sink.push(
        EntityAddNotify::MSG_ID,
        &EntityAddNotify {
            EntityPbs: concom_entity_pbs,
            RemoveTagIds: false,
        },
    );

    let formation = &scene.formation_info.formations[cur_form_idx];
    let fight_role_infos: Vec<FightRoleInfo> = formation
        .roles
        .iter()
        .flatten()
        .map(|slot| FightRoleInfo {
            RoleId: slot.role_id,
            EntityId: slot.entity_id,
            OnStageWithoutControl: false,
        })
        .collect();
    let group_formations = vec![GroupFormation {
        PlayerId: scene.player_id,
        FightRoleInfos: vec![FightRoleInfos {
            GroupType: 1,
            FightRoleInfos: fight_role_infos,
            CurRole: formation.cur_role,
            LivingStatus: Some(formation_living_status(scene, formation)),
            IsFixedLocation: false,
        }],
        CurrentGroupType: 1,
    }];
    sink.push(
        UpdateGroupFormationNotify::MSG_ID,
        &UpdateGroupFormationNotify {
            GroupFormation: group_formations,
        },
    );
    Ok(())
}

pub fn is_role_entity_alive(scene: &Scene, role_id: i32) -> bool {
    for e in &scene.entities {
        if e.config.entity_type != EntityType::Player || e.config.config_type != ConfigType::Character
        {
            continue;
        }
        if e.config.config_id != role_id {
            continue;
        }
        let Some(attr) = e.attribute.as_ref() else {
            return true;
        };
        let max_hp = attributes::get_attr_value(attr, EAttributeType::LifeMax);
        if max_hp <= 0 {
            return true;
        }
        return attributes::get_attr_value(attr, EAttributeType::Life) > 0;
    }
    true
}

pub fn formation_living_status(scene: &Scene, formation: &Formation) -> LivingStatus {
    let mut has_checked_entity = false;
    for maybe_slot in &formation.roles {
        let Some(slot) = maybe_slot else {
            continue;
        };
        if slot.entity_id == 0 {
            continue;
        }
        let Some(entity) = scene.entity(slot.entity_id) else {
            continue;
        };
        let Some(attr) = entity.attribute.as_ref() else {
            continue;
        };
        let max_hp = attributes::get_attr_value(attr, EAttributeType::LifeMax);
        if max_hp <= 0 {
            continue;
        }
        has_checked_entity = true;
        if attributes::get_attr_value(attr, EAttributeType::Life) > 0 {
            return LivingStatus::Alive;
        }
    }
    if has_checked_entity {
        LivingStatus::Dead
    } else {
        LivingStatus::Alive
    }
}

pub fn ensure_alive_cur_role(scene: &mut Scene, form_idx: usize) -> bool {
    let Some(formation) = scene.formation_info.formations.get(form_idx) else {
        return false;
    };
    if formation.cur_role > 0 && is_role_entity_alive(scene, formation.cur_role) {
        return false;
    }
    let next_role = formation
        .roles
        .iter()
        .flatten()
        .find(|slot| is_role_entity_alive(scene, slot.role_id))
        .map(|slot| slot.role_id);
    let Some(next_id) = next_role else {
        return false;
    };
    let prev_role = scene.formation_info.formations[form_idx].cur_role;
    if prev_role == next_id {
        return false;
    }
    scene.formation_info.formations[form_idx].cur_role = next_id;
    for e in &mut scene.entities {
        if e.config.entity_type != EntityType::Player || e.config.config_type != ConfigType::Character
        {
            continue;
        }
        if e.config.config_id == prev_role {
            e.visible = false;
        } else if e.config.config_id == next_id {
            e.visible = true;
        }
    }
    true
}

pub fn is_resonance_nexus_config(assets: &Assets, entity_config_id: i64) -> bool {
    if entity_config_id == 0 {
        return false;
    }
    assets
        .tables
        .map_mark
        .items
        .iter()
        .any(|mark| mark.EntityConfigId == entity_config_id && mark.ObjectType == 8)
}

pub fn is_resonance_nexus_teleporter(assets: &Assets, teleporter_id: i32) -> bool {
    assets
        .tables
        .teleporter
        .get_data_by_id(&teleporter_id)
        .is_some_and(|tp| is_resonance_nexus_config(assets, tp.TeleportEntityConfigId))
}

pub fn revive_and_heal_formation<S: PushSink + ?Sized>(
    sink: &mut S,
    scene: &mut Scene,
    role_comp: &mut PlayerRoleComponent,
    events: &mut EventQueue,
    fs: &FileSystem,
) {
    let mut pack_data = Vec::new();
    let mut any_revived = false;
    let mut dirty_ids = Vec::new();

    for e in &mut scene.entities {
        if e.config.entity_type != EntityType::Player || e.config.config_type != ConfigType::Character
        {
            continue;
        }
        if let Some(attr) = e.attribute.as_mut() {
            let life_max = attributes::get_attr_value(attr, EAttributeType::LifeMax).max(1);
            let cur_life = attributes::get_attr_value(attr, EAttributeType::Life);
            let was_dead = cur_life <= 0;
            if was_dead || cur_life < life_max {
                let _ = attributes::revive_entity(attr, None);
                e.dead_notified = false;
                if was_dead {
                    any_revived = true;
                    pack_data.push(CombatReceiveData {
                        Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
                            CombatNotifyData {
                                CombatCommon: Some(CombatCommon {
                                    EntityId: e.net_id,
                                    ..Default::default()
                                }),
                                Message: Some(
                                    CombatNotifyDataMessageOneof::EntityLivingStatusNotify(
                                        EntityLivingStatusNotify {
                                            Id: e.net_id,
                                            LivingStatus: Some(LivingStatus::Alive),
                                            ..Default::default()
                                        },
                                    ),
                                ),
                            },
                        )),
                    });
                }
                attributes::generate_attr_messages(
                    &mut pack_data,
                    e.net_id,
                    attr,
                    &[EAttributeType::Life],
                );
                dirty_ids.push(e.net_id);
            }
        }
    }
    for eid in dirty_ids {
        scene.mark_fsm_dirty(eid, wake_reason::ATTRIBUTE);
    }

    let trial_roles = &role_comp.trial_roles;
    let role_ids_to_save: Vec<i32> = role_comp
        .role_map
        .iter_mut()
        .filter_map(|(&rid, ri)| {
            if ri.current_hp.take().is_some() && !trial_roles.contains(&rid) {
                Some(rid)
            } else {
                None
            }
        })
        .collect();
    for rid in role_ids_to_save {
        events.enqueue(Event::RoleInfoModified {
            player_id: scene.player_id,
            role_id: rid,
        });
        if let Err(err) = save_role_info(fs, role_comp, rid) {
            tracing::warn!("save_role_info({rid}) failed: {err}");
        }
    }

    if !pack_data.is_empty() {
        sink.push(
            CombatReceivePackNotify::MSG_ID,
            &CombatReceivePackNotify { Data: pack_data },
        );
    }
    scene.force_battle_state(false);
    let mut battle_data = Vec::new();
    if scene.append_battle_state_notify(&mut battle_data) {
        sink.push(
            CombatReceivePackNotify::MSG_ID,
            &CombatReceivePackNotify { Data: battle_data },
        );
    }
    if any_revived {
        events.enqueue(Event::UpdateFormations {
            player_id: scene.player_id,
            formation_id: scene.formation_info.cur_formation,
            role_ids: Vec::new(),
        });
    }
}

pub fn maybe_push_party_wipe_notify<S: PushSink + ?Sized>(sink: &mut S, scene: &Scene) {
    let Ok(f_idx) = usize::try_from(scene.formation_info.cur_formation) else {
        return;
    };
    let Some(form) = scene.formation_info.formations.get(f_idx) else {
        return;
    };
    if formation_living_status(scene, form) == LivingStatus::Dead {
        sink.push(
            PlayerDeadNotify::MSG_ID,
            &PlayerDeadNotify {
                PlayerId: scene.player_id,
                IsShowRevive: true,
                ..Default::default()
            },
        );
    }
}

pub fn heal_formation_if_near_nexus<S: PushSink + ?Sized>(
    sink: &mut S,
    scene: &mut Scene,
    assets: &Assets,
    role_comp: &mut PlayerRoleComponent,
    events: &mut EventQueue,
    fs: &FileSystem,
    player_loc: [f32; 3],
) {
    let needs_heal = scene.entities.iter().any(|e| {
        e.config.entity_type == EntityType::Player
            && e.config.config_type == ConfigType::Character
            && e.attribute.as_ref().is_some_and(|attr| {
                let life_max = attributes::get_attr_value(attr, EAttributeType::LifeMax).max(1);
                attributes::get_attr_value(attr, EAttributeType::Life) < life_max
            })
    });
    if !needs_heal {
        return;
    }

    let map_id = assets.resolve_map_config_id(scene.instance_id);
    for &tp_idx in assets.indexes.teleporter_indices_for_map(map_id) {
        let tp = &assets.tables.teleporter.items[tp_idx as usize];
        let Some(cfg) = assets.find_level_entity(tp.MapId, tp.TeleportEntityConfigId) else {
            continue;
        };
        let Some(tp_pos) = sonoro_gamesv_data::asset_indexes::entity_world_pos(&cfg) else {
            continue;
        };
        let dx = tp_pos[0] - player_loc[0];
        let dy = tp_pos[1] - player_loc[1];
        let dz = tp_pos[2] - player_loc[2];
        if dx * dx + dy * dy + dz * dz <= 1500.0 * 1500.0 {
            revive_and_heal_formation(sink, scene, role_comp, events, fs);
            return;
        }
    }
}

fn is_rover_gender_valid(assets: &Assets, gender: i32, role_id: i32) -> bool {
    is_main_role(assets, role_id) && is_main_role_for_gender(assets, gender, role_id)
}

pub fn repair_account_roles_and_weapons(
    fs: &FileSystem,
    assets: &Assets,
    mut scene: Option<&mut Scene>,
    basic_comp: &mut PlayerBasicComponent,
    role_comp: &mut PlayerRoleComponent,
    weapon_comp: &mut PlayerWeaponComponent,
    echo_comp: &mut PlayerEchoComponent,
) -> Result<RoleRepairSummary, String> {
    let mut summary = RoleRepairSummary::default();
    let mut basic_dirty = false;
    let mut weapon_dirty = false;
    let mut echo_dirty = false;

    if basic_comp.info.attributes.sex != 0 && basic_comp.info.attributes.sex != 1 {
        basic_comp.info.attributes.sex = 1;
        basic_dirty = true;
        summary.fixed_main_role = true;
    }
    let gender = basic_comp.info.attributes.sex;

    let leak_ids: Vec<i32> = role_comp
        .role_map
        .keys()
        .copied()
        .filter(|&rid| {
            if role_comp.is_trial_role(rid) {
                return false;
            }
            let is_trial_entry = assets.tables.trial_role_info.get_data_by_id(&rid).is_some();
            let rc_opt = assets.tables.role_info.get_data_by_id(&rid);
            let invalid_rc = rc_opt
                .as_ref()
                .is_none_or(|rc| rc.IsTrial || rc.RoleType != 1)
                || !(1000..=1999).contains(&rid);
            is_trial_entry || invalid_rc
        })
        .collect();

    for rid in leak_ids {
        if role_comp.role_map.remove(&rid).is_some() {
            summary.removed_leaks += 1;
        }
        role_comp.remove_equipment_cache(rid);
        let _ = fs.delete_file(&format!(
            "player/{}/{}/{rid}",
            role_comp.player_id,
            RoleInfo::DATA_DIR
        ));
        for w in weapon_comp.weapon_map.values_mut() {
            if w.role_id == Some(rid) {
                w.role_id = None;
                weapon_dirty = true;
            }
        }
        for e in echo_comp.echo_map.values_mut() {
            if e.role_id == Some(rid) {
                e.role_id = None;
                e.equipped_pos = 0;
                echo_dirty = true;
            }
        }
    }

    let default_main = if gender == 0 { 1502 } else { 1501 };
    let mut valid_main = 0i32;
    if basic_comp.info.selected_main_role_id != 0
        && role_comp
            .role_map
            .contains_key(&basic_comp.info.selected_main_role_id)
        && is_rover_gender_valid(assets, gender, basic_comp.info.selected_main_role_id)
    {
        valid_main = basic_comp.info.selected_main_role_id;
    } else if let Some(cand) =
        selected_main_role_id(assets, role_comp, gender, basic_comp.info.selected_main_role_id)
        && role_comp.role_map.contains_key(&cand)
        && is_rover_gender_valid(assets, gender, cand)
    {
        valid_main = cand;
    }

    if valid_main == 0 {
        let opp_rover = role_comp
            .role_map
            .keys()
            .copied()
            .find(|&rid| !role_comp.is_trial_role(rid) && is_main_role(assets, rid));
        if let Some(src_id) = opp_rover {
            let elem = assets
                .tables
                .role_info
                .get_data_by_id(&src_id)
                .map_or(5, |rc| rc.ElementId);
            let target_id = main_role_id_for_element(assets, gender, elem).unwrap_or(default_main);
            let _ = transfer_main_role_state(
                fs,
                assets,
                scene.as_deref_mut(),
                basic_comp,
                role_comp,
                weapon_comp,
                echo_comp,
                src_id,
                target_id,
            )?;
            valid_main = target_id;
        } else {
            let new_main = RoleInfo::create_default(assets, default_main)?;
            role_comp.role_map.insert(default_main, new_main);
            save_role_info(fs, role_comp, default_main).map_err(|e| e.to_string())?;
            valid_main = default_main;
        }
        summary.fixed_main_role = true;
    }

    if basic_comp.info.selected_main_role_id != valid_main {
        basic_comp.info.selected_main_role_id = valid_main;
        basic_dirty = true;
        summary.fixed_main_role = true;
    }

    for sr in &mut basic_comp.info.role_show_list {
        if *sr == 0 {
            continue;
        }
        if !role_comp.role_map.contains_key(sr) || role_comp.is_trial_role(*sr) {
            *sr = 0;
            basic_dirty = true;
        } else if is_main_role(assets, *sr) && *sr != valid_main {
            *sr = valid_main;
            basic_dirty = true;
        }
    }

    let role_ids: Vec<i32> = role_comp
        .role_map
        .keys()
        .copied()
        .filter(|&rid| !role_comp.is_trial_role(rid))
        .collect();
    let max_attr = EAttributeType::MAX.to_i32() as usize;
    for rid in role_ids {
        let mut r_dirty = false;
        if let Some(rinfo) = role_comp.role_map.get_mut(&rid) {
            if rinfo.base_prop.len() != max_attr {
                rinfo.reset_properties(assets, rid);
                let _ = rinfo.refresh_scaled_properties(assets, rid);
                r_dirty = true;
            }
            if rinfo.skill_node_state.is_empty() {
                rinfo.add_default_skills(assets, rid)?;
                r_dirty = true;
            }
            if rinfo.role_skin_id == 0
                && let Some(rc) = assets.tables.role_info.get_data_by_id(&rid)
                && rc.SkinId != 0
            {
                rinfo.role_skin_id = rc.SkinId;
                r_dirty = true;
            }
        }
        if r_dirty {
            summary.fixed_roles += 1;
            save_role_info(fs, role_comp, rid).map_err(|e| e.to_string())?;
            role_comp.remove_equipment_cache(rid);
        }
        if ensure_role_weapon(fs, assets, role_comp, weapon_comp, rid)? {
            summary.fixed_weapons += 1;
        }
    }

    if let Some(sc) = scene {
        let mut form_dirty = false;
        for form in &mut sc.formation_info.formations {
            let mut kept: [Option<sonoro_gamesv_fs::formation_info::FormationRole>; 3] =
                [None, None, None];
            let mut klen = 0usize;
            for maybe_slot in &form.roles {
                let Some(slot) = maybe_slot else {
                    continue;
                };
                let keep_id = if is_main_role(assets, slot.role_id) && slot.role_id != valid_main {
                    valid_main
                } else {
                    slot.role_id
                };
                if !role_comp.role_map.contains_key(&keep_id) {
                    form_dirty = true;
                    continue;
                }
                if kept[..klen]
                    .iter()
                    .flatten()
                    .any(|ks| ks.role_id == keep_id)
                {
                    form_dirty = true;
                    continue;
                }
                let mut next_slot = slot.clone();
                if keep_id != slot.role_id {
                    next_slot.role_id = keep_id;
                    next_slot.entity_id = -1;
                    form_dirty = true;
                }
                kept[klen] = Some(next_slot);
                klen += 1;
            }
            if klen == 0 {
                kept[0] = Some(sonoro_gamesv_fs::formation_info::FormationRole {
                    role_id: valid_main,
                    entity_id: -1,
                    on_stage_without_control: false,
                });
                form.cur_role = valid_main;
                form_dirty = true;
            } else if !kept[..klen]
                .iter()
                .flatten()
                .any(|ks| ks.role_id == form.cur_role)
            {
                form.cur_role = kept[0].as_ref().unwrap().role_id;
                form_dirty = true;
            }
            form.roles = kept;
        }
        if form_dirty {
            summary.fixed_formation = true;
            let _ = sc.save_persistent(fs);
        }
    }

    if basic_dirty {
        save_basic_info(fs, basic_comp).map_err(|e| e.to_string())?;
    }
    if weapon_dirty {
        PlayerWeaponComponent::save_all(fs, weapon_comp.player_id, &weapon_comp.weapon_map)
            .map_err(|e| e.to_string())?;
    }
    if echo_dirty {
        PlayerEchoComponent::save_all(fs, echo_comp.player_id, &echo_comp.echo_map)
            .map_err(|e| e.to_string())?;
    }

    Ok(summary)
}

pub fn push_role_metadata_notifies<S: PushSink + ?Sized>(
    sink: &mut S,
    assets: &Assets,
    basic_comp: &PlayerBasicComponent,
    role_comp: &PlayerRoleComponent,
) {
    sink.push(
        RoleChangeUnlockNotify::MSG_ID,
        &RoleChangeUnlockNotify {
            UnlockRoleIds: main_role_unlock_list(assets, basic_comp.info.attributes.sex),
            NextAllowChangeTime: 0,
        },
    );

    let mut favor_list = Vec::new();
    let mut motion_list = Vec::new();
    for (&rid, role_info) in &role_comp.role_map {
        favor_list.push(RoleFavor {
            RoleId: rid,
            Level: role_info.favor,
            Exp: 0,
            ..Default::default()
        });
        let motion_ids: Vec<sonoro_proto::pb::FavorItem> = assets
            .tables
            .motion
            .items
            .iter()
            .filter(|m| m.RoleId == rid)
            .map(|m| sonoro_proto::pb::FavorItem {
                Id: m.Id,
                Status: Some(FavorItemStatus::ItemUnLocked),
            })
            .collect();
        if !motion_ids.is_empty() {
            motion_list.push(RoleMotion {
                RoleId: rid,
                MotionIds: motion_ids,
            });
        }
    }
    sink.push(
        RoleFavorListNotify::MSG_ID,
        &RoleFavorListNotify {
            FavorList: favor_list,
            RoleConditionInfoMap: BTreeMap::new(),
        },
    );
    sink.push(
        RoleMotionListNotify::MSG_ID,
        &RoleMotionListNotify {
            MotionList: motion_list,
            RoleConditionInfoMap: BTreeMap::new(),
        },
    );
}

/// Forces any mounted vehicle or motorcycle to dismount via `VehicleUpdateNotify` before teleporting.
pub fn dismount_active_vehicle<S: PushSink + ?Sized>(
    sink: &mut S,
    scene: Option<&mut Scene>,
    progress: &mut PlayerProgressComponent,
    assets: &Assets,
    fs: &FileSystem,
    player_id: i32,
) {
    let mut active_vehicle_ids: Vec<i64> = Vec::new();
    let motor_cfg_id = assets.get_motorcycle_config_id();
    let motor_riding = progress.info.get_key_int("motor:riding") > 0;
    let vehicle_cur = progress.info.get_key_int("vehicle:cur");

    if let Some(sc) = scene {
        let mut motor_fallback_id: Option<i64> = None;
        for storage in &mut sc.entities {
            if let Some(veh) = storage.vehicle.as_mut()
                && veh.occupied
            {
                veh.occupied = false;
                active_vehicle_ids.push(storage.net_id);
            }
            if motor_fallback_id.is_none()
                && motor_riding
                && let Some(mid) = motor_cfg_id
                && storage.config.config_id == mid
            {
                motor_fallback_id = Some(storage.net_id);
            }
        }
        for &eid in &active_vehicle_ids {
            let _ = sc.save_entity_by_net_id(fs, eid);
        }
        if active_vehicle_ids.is_empty()
            && let Some(mid) = motor_fallback_id
        {
            active_vehicle_ids.push(mid);
        }
    }

    if active_vehicle_ids.is_empty() && vehicle_cur <= 0 && !motor_riding {
        return;
    }

    progress.info.set_key_int("vehicle:cur", 0);
    progress.info.set_key_int("vehicle:seat", -1);
    progress.info.set_key_int("motor:riding", 0);
    let _ = progress.save(fs);

    let vehicle_entity_id = active_vehicle_ids.first().copied().unwrap_or(0);
    let exit_type = if vehicle_entity_id != 0 {
        ExitVehicleType::ExitVehicleTypeAllRole
    } else {
        ExitVehicleType::ExitVehicleTypeNormal
    };
    sink.push(
        VehicleUpdateNotify::MSG_ID,
        &VehicleUpdateNotify {
            EntityId: vehicle_entity_id,
            PlayerId: player_id,
            SeatId: -1,
            ExitType: Some(exit_type),
            ClientPredicted: false,
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component::entity::EntityComponentStorage;
    use crate::component::entity::small::{ConfigComponent, EntityState, VehicleComponent};
    use crate::sink::VecPushSink;
    use sonoro_gamesv_fs::explore_tools_info::ExploreToolsInfo;
    use sonoro_gamesv_fs::formation_info::FormationInfo;
    use sonoro_gamesv_fs::progress_info::ProgressInfo;
    use sonoro_gamesv_fs::scene_instance::SceneInstance;
    use sonoro_proto::PbMessage;

    #[test]
    fn dismount_active_vehicle_clears_state_and_emits_notify() {
        let dir = std::env::temp_dir().join(format!("sonoro_vehicle_tp_test_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("player/100000001")).unwrap();
        let fs = FileSystem::open(&dir).unwrap();
        let assets = Assets::load_default().unwrap();

        let mut scene = Scene::from_parts(
            100000001,
            8,
            SceneInstance::default(),
            FormationInfo::default(),
            ExploreToolsInfo::default(),
        );
        let mut storage = EntityComponentStorage::new(
            0,
            ConfigComponent {
                camp: 0,
                config_id: 999999,
                config_type: ConfigType::Level,
                entity_type: EntityType::Vehicle,
                state: EntityState::Default,
            },
        );
        storage.vehicle = Some(VehicleComponent {
            occupied: true,
            ..Default::default()
        });
        let veh = scene.spawn(storage);

        let mut progress = PlayerProgressComponent {
            player_id: 100000001,
            info: ProgressInfo::default(),
        };
        progress.info.set_key_int("vehicle:cur", 999999);
        progress.info.set_key_int("vehicle:seat", 0);
        progress.info.set_key_int("motor:riding", 1);

        let mut sink = VecPushSink::default();
        dismount_active_vehicle(
            &mut sink,
            Some(&mut scene),
            &mut progress,
            &assets,
            &fs,
            100000001,
        );

        assert!(!scene.entities[veh.index].vehicle.as_ref().unwrap().occupied);
        assert_eq!(progress.info.get_key_int("vehicle:cur"), 0);
        assert_eq!(progress.info.get_key_int("vehicle:seat"), -1);
        assert_eq!(progress.info.get_key_int("motor:riding"), 0);
        assert_eq!(sink.packets.len(), 1);
        assert_eq!(sink.packets[0].msg_id, VehicleUpdateNotify::MSG_ID);
        let decoded = VehicleUpdateNotify::decode(&sink.packets[0].payload).unwrap();
        assert_eq!(decoded.EntityId, veh.net_id);
        assert_eq!(decoded.SeatId, -1);
        assert_eq!(decoded.ExitType, Some(ExitVehicleType::ExitVehicleTypeAllRole));

        let mut noop_sink = VecPushSink::default();
        dismount_active_vehicle(
            &mut noop_sink,
            Some(&mut scene),
            &mut progress,
            &assets,
            &fs,
            100000001,
        );
        assert!(noop_sink.packets.is_empty());

        let _ = std::fs::remove_dir_all(&dir);
    }
}
