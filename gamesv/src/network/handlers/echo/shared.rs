use sonoro_common::fs::FileSystem;
use sonoro_gamesv_data::Assets;
use sonoro_gamesv_data::tables::instance_dungeon::InstanceDungeon;
use sonoro_gamesv_fs::echo_info::{EchoInfo, RoleEquip};
use sonoro_gamesv_fs::role_info::RoleInfo;
use sonoro_gamesv_fs::special_item_incr;
use sonoro_gamesv_logic::component::entity::{ConcomitantComponent, VisionSkillComponent};
use sonoro_gamesv_logic::component::player::{
    PlayerBasicComponent, PlayerEchoComponent, PlayerRoleComponent, PlayerWeaponComponent,
};
use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::events::{BuffAdditionEntry, Event};
use sonoro_gamesv_logic::fsm::types::wake_reason;
use sonoro_gamesv_logic::handlers::{buff as buff_handlers, fsm as fsm_handlers};
use sonoro_gamesv_logic::helpers::{
    attributes, buff as buff_helper, calabash, entity_proto, quest_engine, role as role_helper,
    role_attribute_sync, role_stats,
};
use sonoro_gamesv_logic::scene::Scene;
use sonoro_gamesv_logic::schedulers::BuffTimerScheduler;
use sonoro_gamesv_logic::sink::PushSink;
use sonoro_gamesv_logic::templates::role_entity_templates;
use sonoro_proto::PbEnum;
use sonoro_proto::enums::{EAttributeType, ErrorCode};
use sonoro_proto::pb::{
    ArrayIntInt, AttributeChangedNotify, CombatCommon, CombatNotifyData,
    CombatNotifyDataMessageOneof, CombatReceiveData, CombatReceiveDataMessageOneof,
    CombatReceivePackNotify, EntityAddNotify, EntityRemoveInfo, EntityRemoveNotify, PhantomItem,
    PhantomStackSplitInfo, RecoverPropChangedNotify, RecoverPropFromServer,
    RefreshVisionEquipGroupData, RolePhantomEquipInfo, RolePhantomPropInfo,
    RolePhantomPropUpdateNotify, VisionSkillChangeNotify, VisionSkillInformation,
};

use super::super::State;
use crate::network::connection::Connection;

pub fn phantom_item_list(echo_comp: &PlayerEchoComponent) -> Vec<PhantomItem> {
    echo_comp
        .echo_map
        .iter()
        .map(|(&inc_id, item)| item.to_proto(inc_id))
        .collect()
}

pub fn vision_equip_group_list(
    echo_comp: &PlayerEchoComponent,
) -> Vec<RefreshVisionEquipGroupData> {
    echo_comp
        .preset_info
        .groups
        .iter()
        .map(|group| group.to_proto())
        .collect()
}

pub fn equip_info_list(
    role_comp: &PlayerRoleComponent,
    echo_comp: &PlayerEchoComponent,
) -> Vec<RolePhantomEquipInfo> {
    role_comp
        .role_map
        .keys()
        .map(|&role_id| echo_comp.role_equip(role_id).to_proto())
        .collect()
}

pub fn role_prop_info_list(
    assets: &Assets,
    role_comp: &mut PlayerRoleComponent,
    echo_comp: &PlayerEchoComponent,
    weapon_comp: &PlayerWeaponComponent,
) -> Result<Vec<RolePhantomPropInfo>, String> {
    let role_ids: Vec<i32> = role_comp.role_map.keys().copied().collect();
    let mut list = Vec::with_capacity(role_ids.len());
    for role_id in role_ids {
        let equip = echo_comp.role_equip(role_id);
        list.push(role_prop_info(
            assets,
            role_comp,
            echo_comp,
            weapon_comp,
            equip,
        )?);
    }
    Ok(list)
}

pub fn changed_role_prop_info_list(
    assets: &Assets,
    role_comp: &mut PlayerRoleComponent,
    echo_comp: &PlayerEchoComponent,
    weapon_comp: &PlayerWeaponComponent,
    changed_roles: &[i32],
) -> Result<Vec<RolePhantomPropInfo>, String> {
    let mut list = Vec::with_capacity(changed_roles.len());
    for &role_id in changed_roles {
        let equip = echo_comp.role_equip(role_id);
        list.push(role_prop_info(
            assets,
            role_comp,
            echo_comp,
            weapon_comp,
            equip,
        )?);
    }
    Ok(list)
}

fn role_prop_info(
    assets: &Assets,
    role_comp: &mut PlayerRoleComponent,
    echo_comp: &PlayerEchoComponent,
    weapon_comp: &PlayerWeaponComponent,
    equip: RoleEquip,
) -> Result<RolePhantomPropInfo, String> {
    let display = role_stats::build_phantom_display_props(
        assets,
        role_comp,
        weapon_comp,
        echo_comp,
        equip.role_id,
    )
    .map_err(|e| e.to_string())?;

    Ok(RolePhantomPropInfo {
        RoleId: equip.role_id,
        BaseProp: display
            .base_props
            .into_iter()
            .map(|(k, v)| ArrayIntInt { Key: k, Value: v })
            .collect(),
        AddProp: display
            .add_props
            .into_iter()
            .map(|(k, v)| ArrayIntInt { Key: k, Value: v })
            .collect(),
    })
}

pub fn changed_equip_info_list(
    echo_comp: &PlayerEchoComponent,
    changed_roles: &[i32],
) -> Vec<RolePhantomEquipInfo> {
    changed_roles
        .iter()
        .map(|&role_id| echo_comp.role_equip(role_id).to_proto())
        .collect()
}

pub fn split_single_from_stack(
    fs: &FileSystem,
    echo_comp: &mut PlayerEchoComponent,
    incr_id: i32,
) -> Result<Option<PhantomStackSplitInfo>, String> {
    let orig_count = match echo_comp.echo_map.get(&incr_id) {
        Some(e) => e.stack_count,
        None => return Ok(None),
    };
    if orig_count <= 1 {
        return Ok(None);
    }

    let mut rem = echo_comp.echo_map.get(&incr_id).unwrap().clone();
    rem.stack_count = orig_count - 1;
    rem.role_id = None;
    rem.equipped_pos = -1;

    let rem_incr_id = special_item_incr::next(fs, echo_comp.player_id).map_err(|e| e.to_string())?;
    let rem_proto = rem.to_proto(rem_incr_id);
    echo_comp.echo_map.insert(rem_incr_id, rem);

    let operated = echo_comp.echo_map.get_mut(&incr_id).unwrap();
    operated.stack_count = 1;
    let operated_proto = operated.to_proto(incr_id);

    Ok(Some(PhantomStackSplitInfo {
        OperateItem: Some(operated_proto),
        OperateBeforeCount: orig_count,
        RemainderItem: Some(rem_proto),
    }))
}

pub fn total_invested_phantom_exp(assets: &Assets, group_id: i32, item: &EchoInfo) -> i32 {
    let mut total = item.exp;
    for lvl in 1..=item.level {
        total += phantom_level_exp(assets, group_id, lvl).unwrap_or(0);
    }
    total
}

pub fn role_exists_and_editable(role_comp: &PlayerRoleComponent, role_id: i32) -> bool {
    if role_comp.is_trial_role(role_id) {
        return false;
    }
    role_comp.role_map.contains_key(&role_id)
}

pub fn validate_equip_slots(
    assets: &Assets,
    basic_comp: &PlayerBasicComponent,
    echo_comp: &PlayerEchoComponent,
    slots: [i32; 5],
    cost_err: ErrorCode,
) -> Option<ErrorCode> {
    let max_cost = calabash::current_cost(assets, basic_comp, echo_comp);
    let mut total_cost = 0i32;
    let mut seen_monsters = Vec::with_capacity(5);

    for inc_id in slots {
        if inc_id == 0 {
            continue;
        }
        let Some(item) = echo_comp.echo_map.get(&inc_id) else {
            return Some(ErrorCode::ErrPhantomItemNotExist);
        };
        let Some(cfg) = assets.tables.phantom_item.get_data_by_id(&item.id) else {
            return Some(ErrorCode::ErrPhantomConfigNotFound);
        };
        if seen_monsters.contains(&cfg.MonsterId) {
            return Some(ErrorCode::ErrPhantomEquipDuplicate);
        }
        seen_monsters.push(cfg.MonsterId);

        let Some(rarity) = assets.tables.phantom_rarity.get_data_by_id(&cfg.Rarity) else {
            return Some(ErrorCode::ErrPhantomQaulityNotExist);
        };
        total_cost += rarity.Cost;
    }

    if total_cost > max_cost {
        return Some(cost_err);
    }
    None
}

pub struct TunerCost {
    pub item_id: i32,
    pub count: i32,
}

pub fn tuner_cost(assets: &Assets, quality: i32) -> Option<TunerCost> {
    let config = assets.tables.phantom_quality.get_data_by_id(&quality)?;
    let (&item_id, &count) = config.IdentifyCost.iter().next()?;
    Some(TunerCost { item_id, count })
}

pub fn highest_phantom_level(assets: &Assets, group_id: i32) -> Option<i32> {
    assets
        .tables
        .phantom_level
        .items
        .iter()
        .filter(|e| e.GroupId == group_id)
        .map(|e| e.Level)
        .max()
}

pub fn phantom_level_exp(assets: &Assets, group_id: i32, level: i32) -> Option<i32> {
    assets
        .tables
        .phantom_level
        .items
        .iter()
        .find(|e| e.GroupId == group_id && e.Level == level)
        .map(|e| e.Exp)
}

pub fn push_unique_role(list: &mut Vec<i32>, role_id: i32) {
    if !list.contains(&role_id) {
        list.push(role_id);
    }
}

pub fn roles_equipping_echo(echo_comp: &PlayerEchoComponent, inc_id: i32) -> Vec<i32> {
    let mut out = Vec::new();
    append_roles_equipping_echo(echo_comp, inc_id, &mut out);
    out
}

pub fn append_roles_equipping_echo(
    echo_comp: &PlayerEchoComponent,
    inc_id: i32,
    changed_roles: &mut Vec<i32>,
) {
    if let Some(equip) = echo_comp.equipped_echo_by_incr_id(inc_id) {
        push_unique_role(changed_roles, equip.role_id);
    }
}

pub fn append_roles_with_main_echo(
    echo_comp: &PlayerEchoComponent,
    inc_id: i32,
    changed_roles: &mut Vec<i32>,
) {
    if let Some(equip) = echo_comp.equipped_echo_by_incr_id(inc_id)
        && equip.pos == 0
    {
        push_unique_role(changed_roles, equip.role_id);
    }
}

pub fn push_role_prop_update(state: &mut State, changed_roles: &[i32]) -> Result<(), String> {
    let prop_info = changed_role_prop_info_list(
        &state.assets,
        &mut state.player_components.role,
        &state.player_components.echo,
        &state.player_components.weapon,
        changed_roles,
    )?;
    state.conn.push(
        RolePhantomPropUpdateNotify::MSG_ID,
        &RolePhantomPropUpdateNotify { PropInfo: prop_info },
    );
    Ok(())
}

pub fn refresh_role_entities(state: &mut State, changed_roles: &[i32]) -> Result<(), String> {
    let State {
        conn,
        fs,
        assets,
        player_components,
        scene: scene_opt,
        buff_timers,
        ..
    } = state;
    let Some(scene) = scene_opt.as_mut() else {
        return Ok(());
    };
    let Some(instance_dungeon) = assets
        .tables
        .instance_dungeon
        .get_data_by_id(&scene.instance_id)
        .map(|r| r.into_owned())
    else {
        return Ok(());
    };

    let now_ms = quest_engine::tree::now_ms_pub();
    let role_comp = &mut player_components.role;
    let weapon_comp = &player_components.weapon;
    let echo_comp = &player_components.echo;

    let mut reset_roles = Vec::new();
    let entity_targets: Vec<(usize, i64, i32)> = scene
        .entities
        .iter()
        .enumerate()
        .filter_map(|(idx, s)| {
            if s.vision_skills.is_some()
                && s.concomitant.is_some()
                && s.attribute.is_some()
                && s.buffs.is_some()
                && changed_roles.contains(&s.config.config_id)
            {
                Some((idx, s.net_id, s.config.config_id))
            } else {
                None
            }
        })
        .collect();

    for (idx, net_id, config_id) in entity_targets {
        let Some(role_info) = role_comp.role_map.get(&config_id).cloned() else {
            continue;
        };
        let desired = desired_echo_vision_state(assets, echo_comp, config_id);
        let topology_ok = {
            let concom = scene.entities[idx].concomitant.as_ref().unwrap();
            vision_topology_matches(scene, concom, desired)
        };
        if !topology_ok {
            push_unique_role(&mut reset_roles, config_id);
            continue;
        }

        let vision_state = sync_vision_entities(
            conn,
            fs,
            scene,
            assets,
            &role_info,
            &instance_dungeon,
            net_id,
            desired,
            idx,
            buff_timers,
            now_ms,
        )?;
        let active_explore_skill = scene.explore_tools_info.active_explore_skill;
        let vision_skill_changed = {
            let vs = scene.entities[idx].vision_skills.as_mut().unwrap();
            sync_vision_skills(
                conn,
                echo_comp,
                assets,
                config_id,
                net_id,
                vision_state.main_entity_id,
                vision_state.combo_entity_id,
                active_explore_skill,
                vs,
            )
        };
        let attribute_changed = sync_echo_attributes(
            conn,
            assets,
            role_comp,
            config_id,
            weapon_comp,
            echo_comp,
            net_id,
            scene,
            idx,
        )?;
        let mut events = EventQueue::new();
        let buff_changed = sync_echo_buff_effects(
            conn,
            fs,
            &mut events,
            scene,
            assets,
            net_id,
            config_id,
            echo_comp,
            idx,
            buff_timers,
            now_ms,
        )?;
        let mut follow_up_steps = 0usize;
        while let Some(ev) = events.dequeue() {
            match ev {
                Event::BuffAddition {
                    target,
                    instigator,
                    buffs,
                } if follow_up_steps < 8 => {
                    follow_up_steps += 1;
                    buff_handlers::add_buff_to_entity(
                        conn,
                        &mut events,
                        scene,
                        buff_timers,
                        assets,
                        target,
                        instigator,
                        &buffs,
                    );
                }
                Event::BuffChange { entity, .. } => {
                    fsm_handlers::handle_fsm_buff_change(scene, entity);
                }
                _ => {}
            }
        }

        if !vision_state.changed && !vision_skill_changed && !attribute_changed && !buff_changed {
            continue;
        }
        let mut wake_mask: u16 = 0;
        if attribute_changed {
            wake_mask |= wake_reason::ATTRIBUTE;
        }
        if buff_changed {
            wake_mask |= wake_reason::BUFF;
        }
        scene.mark_fsm_dirty(net_id, wake_mask);
        buff_timers.ensure_entity_registered(scene, assets, net_id, now_ms);
        buff_timers.sync_entity_left_durations(scene, net_id, now_ms);
        scene
            .save_entity_by_net_id(fs, net_id)
            .map_err(|e| e.to_string())?;
    }

    if !reset_roles.is_empty() {
        role_helper::reset_roles(
            scene,
            fs,
            assets,
            role_comp,
            weapon_comp,
            echo_comp,
            conn,
            Some(&reset_roles),
            buff_timers,
            now_ms,
        )
        .map_err(|e| e.to_string())?;
    }

    role_attribute_sync::push_role_info_notify_for_roles(
        conn,
        assets,
        role_comp,
        weapon_comp,
        echo_comp,
        changed_roles,
    );
    Ok(())
}

#[derive(Clone, Copy)]
struct DesiredVisionState {
    main_summon_id: i32,
    combo_summon_id: i32,
}

struct SyncedVisionState {
    main_entity_id: i64,
    combo_entity_id: i64,
    changed: bool,
}

fn desired_echo_vision_state(
    assets: &Assets,
    echo_comp: &PlayerEchoComponent,
    role_id: i32,
) -> DesiredVisionState {
    DesiredVisionState {
        main_summon_id: echo_comp.main_echo_summon_id(assets, role_id).unwrap_or(0),
        combo_summon_id: echo_comp
            .combo_echo_summon_id(assets, role_id)
            .unwrap_or(0),
    }
}

#[allow(clippy::too_many_arguments)]
fn sync_vision_entities(
    conn: &mut Connection,
    fs: &FileSystem,
    scene: &mut Scene,
    assets: &Assets,
    role_info: &RoleInfo,
    instance_dungeon: &InstanceDungeon,
    role_entity_id: i64,
    desired: DesiredVisionState,
    role_idx: usize,
    buff_timers: &mut BuffTimerScheduler,
    now_ms: i64,
) -> Result<SyncedVisionState, String> {
    let old_vision_ids = scene.entities[role_idx]
        .concomitant
        .as_ref()
        .unwrap()
        .vision_entity_id
        .clone();
    let desired_summons = [desired.main_summon_id, desired.combo_summon_id];
    let mut desired_entities = [0i64; 2];
    let mut used_old = [false; 2];
    let mut remove_infos = Vec::new();
    let mut add_pbs = Vec::new();

    for (desired_index, &summon_id) in desired_summons.iter().enumerate() {
        if summon_id == 0 {
            continue;
        }
        for (old_index, &entity_id) in old_vision_ids.iter().enumerate() {
            if old_index >= used_old.len() || used_old[old_index] {
                continue;
            }
            if vision_entity_summon_id(scene, entity_id) != Some(summon_id) {
                continue;
            }
            desired_entities[desired_index] = entity_id;
            used_old[old_index] = true;
            break;
        }
    }

    for (old_index, &entity_id) in old_vision_ids.iter().enumerate() {
        if old_index < used_old.len() && used_old[old_index] {
            continue;
        }
        scene.remove(fs, entity_id).map_err(|e| e.to_string())?;
        remove_infos.push(EntityRemoveInfo {
            EntityId: entity_id,
            Type: None,
        });
    }

    for (desired_index, &summon_id) in desired_summons.iter().enumerate() {
        if summon_id == 0 || desired_entities[desired_index] != 0 {
            continue;
        }
        if let Some(vision_entity) = role_entity_templates::create_vision_entity(
            scene,
            scene.player_id,
            assets,
            role_info,
            instance_dungeon,
            summon_id,
            role_entity_id,
        ) {
            desired_entities[desired_index] = vision_entity.net_id;
            let pb = entity_proto::build(
                assets,
                scene,
                buff_timers,
                vision_entity.net_id,
                now_ms,
            )
            .map_err(|e| e.to_string())?;
            add_pbs.push(pb);
        }
    }

    let final_ids: Vec<i64> = desired_entities.into_iter().filter(|&id| id != 0).collect();
    let same_ids = old_vision_ids == final_ids;
    if !same_ids {
        scene.entities[role_idx]
            .concomitant
            .as_mut()
            .unwrap()
            .vision_entity_id = final_ids;
    }

    let changed = !same_ids || !remove_infos.is_empty() || !add_pbs.is_empty();
    if !remove_infos.is_empty() {
        conn.push(
            EntityRemoveNotify::MSG_ID,
            &EntityRemoveNotify {
                IsRemove: true,
                RemoveInfos: remove_infos,
            },
        );
    }
    if !add_pbs.is_empty() {
        conn.push(
            EntityAddNotify::MSG_ID,
            &EntityAddNotify {
                RemoveTagIds: false,
                EntityPbs: add_pbs,
            },
        );
    }

    Ok(SyncedVisionState {
        main_entity_id: desired_entities[0],
        combo_entity_id: desired_entities[1],
        changed,
    })
}

fn vision_entity_summon_id(scene: &Scene, entity_id: i64) -> Option<i32> {
    scene
        .entity(entity_id)
        .and_then(|s| s.summoner)
        .map(|s| s.summon_skill_id)
}

fn vision_topology_matches(
    scene: &Scene,
    concomitant_comp: &ConcomitantComponent,
    desired: DesiredVisionState,
) -> bool {
    let expected = [desired.main_summon_id, desired.combo_summon_id];
    let mut index = 0usize;
    for summon_id in expected {
        if summon_id == 0 {
            continue;
        }
        let Some(&eid) = concomitant_comp.vision_entity_id.get(index) else {
            return false;
        };
        if vision_entity_summon_id(scene, eid) != Some(summon_id) {
            return false;
        }
        index += 1;
    }
    index == concomitant_comp.vision_entity_id.len()
}

#[allow(clippy::too_many_arguments)]
fn sync_vision_skills(
    conn: &mut Connection,
    echo_comp: &PlayerEchoComponent,
    assets: &Assets,
    role_id: i32,
    role_entity_id: i64,
    vision_entity_id: i64,
    combo_vision_entity_id: i64,
    explore_skill_id: i32,
    vision_skill_comp: &mut VisionSkillComponent,
) -> bool {
    let desired = echo_comp.build_vision_skills(
        assets,
        role_id,
        vision_entity_id,
        combo_vision_entity_id,
        explore_skill_id,
    );
    if vision_skills_equal(&vision_skill_comp.vision_skills, &desired) {
        return false;
    }
    vision_skill_comp.vision_skills = desired;
    conn.push(
        VisionSkillChangeNotify::MSG_ID,
        &VisionSkillChangeNotify {
            EntityId: role_entity_id,
            VisionSkillInfos: vision_skill_comp.vision_skills.clone(),
            PhantomSkillInfo: None,
        },
    );
    true
}

fn vision_skills_equal(a: &[VisionSkillInformation], b: &[VisionSkillInformation]) -> bool {
    a.len() == b.len()
        && a.iter().zip(b.iter()).all(|(l, r)| {
            l.SkillId == r.SkillId
                && l.Level == r.Level
                && l.Quality == r.Quality
                && l.VisionEntityId == r.VisionEntityId
                && l.Index == r.Index
        })
}

#[allow(clippy::too_many_arguments)]
fn sync_echo_attributes(
    conn: &mut Connection,
    assets: &Assets,
    role_comp: &mut PlayerRoleComponent,
    role_id: i32,
    weapon_comp: &PlayerWeaponComponent,
    echo_comp: &PlayerEchoComponent,
    entity_id: i64,
    scene: &mut Scene,
    role_idx: usize,
) -> Result<bool, String> {
    let before = scene.entities[role_idx]
        .attribute
        .as_ref()
        .expect("attribute checked")
        .attributes
        .clone();
    role_stats::refresh_attribute_component(
        assets,
        role_comp,
        role_id,
        weapon_comp,
        echo_comp,
        scene.entities[role_idx]
            .attribute
            .as_mut()
            .expect("attribute checked"),
    )
    .map_err(|e| e.to_string())?;

    // Clear stale buff mod tracking wiped by refresh_attribute_component, then re-sync.
    if let Some(buffs) = scene.entities[role_idx].buffs.as_mut() {
        buffs.set_applied_buff_mods(Vec::new());
    }
    let mut sync_pack = Vec::new();
    buff_helper::sync_continuous_buff_modifiers(&mut sync_pack, scene, entity_id, assets);
    {
        let storage = &mut scene.entities[role_idx];
        if let (Some(buffs), Some(attr_comp)) = (storage.buffs.as_ref(), storage.attribute.as_mut())
        {
            buff_helper::sync_shield_attribute(&mut sync_pack, entity_id, buffs, attr_comp);
        }
    }

    let attr_comp = scene.entities[role_idx]
        .attribute
        .as_ref()
        .expect("attribute checked");
    let mut changed = Vec::new();
    for (index, &new_attr) in attr_comp.attributes.iter().enumerate() {
        let attr_type = attributes::attr_from_id(index as i32);
        if attr_type.to_i32() != index as i32 {
            continue;
        }
        let old_attr = before[index];
        if old_attr.base != new_attr.base
            || old_attr.increment != new_attr.increment
            || old_attr.current != new_attr.current
        {
            changed.push(attr_type);
        }
    }

    if changed.is_empty() {
        return Ok(false);
    }
    push_changed_attributes(conn, entity_id, attr_comp, &changed);
    Ok(true)
}

fn push_changed_attributes(
    conn: &mut Connection,
    entity_id: i64,
    attr_comp: &sonoro_gamesv_logic::component::entity::AttributeComponent,
    changed: &[EAttributeType],
) {
    let mut attrs = Vec::new();
    let mut recovers = Vec::new();
    let recover_types = [
        EAttributeType::Hardness,
        EAttributeType::Tough,
        EAttributeType::ParalysisTime,
    ];

    for &attr_type in changed {
        let index = attr_type.to_i32();
        if index < 0 || (index as usize) >= attr_comp.attributes.len() {
            continue;
        }
        let attr = attr_comp.attributes[index as usize];
        attrs.push(attributes::gameplay_attribute_data(attr_type, attr));

        if recover_types.contains(&attr_type) {
            let max_value = attributes::get_related_attr(attr_type, attributes::RelatedAttrKind::Max)
                .and_then(|m| attr_comp.attributes.get(m.to_i32() as usize))
                .map_or(0, |a| a.current);
            let recover_value =
                attributes::get_related_attr(attr_type, attributes::RelatedAttrKind::Recover)
                    .and_then(|r| attr_comp.attributes.get(r.to_i32() as usize))
                    .map_or(0, |a| a.current);
            recovers.push(RecoverPropFromServer {
                AttrId: index,
                Ratio: recover_value,
                MaxValue: max_value,
                ValueIncrement: attr.current,
            });
        }
    }

    let mut data = vec![CombatReceiveData {
        Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
            CombatNotifyData {
                CombatCommon: Some(CombatCommon {
                    EntityId: entity_id,
                    ..Default::default()
                }),
                Message: Some(CombatNotifyDataMessageOneof::AttributeChangedNotify(
                    AttributeChangedNotify { Attributes: attrs },
                )),
            },
        )),
    }];
    if !recovers.is_empty() {
        data.push(CombatReceiveData {
            Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
                CombatNotifyData {
                    CombatCommon: Some(CombatCommon {
                        EntityId: entity_id,
                        ..Default::default()
                    }),
                    Message: Some(CombatNotifyDataMessageOneof::RecoverPropChangedNotify(
                        RecoverPropChangedNotify {
                            Attributes: recovers,
                            Duration: 0,
                        },
                    )),
                },
            )),
        });
    }

    conn.push(
        CombatReceivePackNotify::MSG_ID,
        &CombatReceivePackNotify { Data: data },
    );
}

#[allow(clippy::too_many_arguments)]
fn sync_echo_buff_effects(
    conn: &mut Connection,
    fs: &FileSystem,
    events: &mut EventQueue,
    scene: &mut Scene,
    assets: &Assets,
    entity_id: i64,
    role_id: i32,
    echo_comp: &PlayerEchoComponent,
    role_idx: usize,
    buff_timers: &mut BuffTimerScheduler,
    now_ms: i64,
) -> Result<bool, String> {
    let Some(handle) = scene.entity_handle(entity_id) else {
        return Ok(false);
    };
    buff_timers.ensure_entity_registered(scene, assets, entity_id, now_ms);
    let expected = echo_comp.active_echo_buff_effects(assets, role_id);

    let to_remove_handles: Vec<i32> = scene.entities[role_idx]
        .buffs
        .as_ref()
        .unwrap()
        .buffs
        .iter()
        .filter(|b| is_echo_buff_effect(assets, b.BuffId) && !expected.contains(&b.BuffId))
        .map(|b| b.HandleId)
        .collect();

    let mut changed = false;
    if !to_remove_handles.is_empty() {
        buff_handlers::remove_buff_handles_with_sync(
            handle,
            &to_remove_handles,
            Some(scene),
            Some(assets),
            conn,
            events,
            buff_timers,
        );
        changed = true;
    }

    let mut to_add_entries: Vec<BuffAdditionEntry> = Vec::new();
    for buff_id in expected {
        let already_present = scene.entities[role_idx]
            .buffs
            .as_ref()
            .unwrap()
            .get_by_buff_id(buff_id)
            .is_some();
        if already_present
            || to_add_entries.iter().any(|e| e.id == buff_id)
            || assets.tables.buff.get_data_by_id(&buff_id).is_none()
        {
            continue;
        }
        to_add_entries.push(BuffAdditionEntry {
            id: buff_id,
            is_active: true,
            notify_client: true,
            ..Default::default()
        });
    }

    if !to_add_entries.is_empty() {
        let _ = scene.save_entity_by_net_id(fs, entity_id);
        buff_handlers::add_buff_to_entity(
            conn,
            events,
            scene,
            buff_timers,
            assets,
            handle,
            handle,
            &to_add_entries,
        );
        changed = true;
    }

    Ok(changed)
}

fn is_echo_buff_effect(assets: &Assets, buff_id: i64) -> bool {
    if buff_id > 1000
        && buff_id.rem_euclid(1000) == 1
        && let Some(buff) = assets.tables.buff.get_data_by_id(&buff_id)
        && !buff.GrantedTags.is_empty()
        && buff.ExtraEffectID == 0
    {
        return true;
    }
    if assets
        .tables
        .phantom_skill
        .items
        .iter()
        .any(|s| s.BuffEffects.contains(&buff_id))
    {
        return true;
    }
    assets
        .tables
        .phantom_fetter
        .items
        .iter()
        .any(|f| f.BuffIds.contains(&buff_id))
}
