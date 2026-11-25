use std::time::{SystemTime, UNIX_EPOCH};

use sonoro_gamesv_logic::component::entity::small::{ConfigType, EntityType};
use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::events::Event;
use sonoro_gamesv_logic::fsm::types::wake_reason;
use sonoro_gamesv_logic::helpers::{
    attributes::{self, AttributeChangeType, AttributeValueType},
    buff as buff_helper, damage as damage_helper, level_play::{self, LevelPlayContext},
    quest_engine, role as role_helper,
};
use sonoro_gamesv_logic::scene::Scene;
use sonoro_proto::enums::{EAttributeType, ErrorCode, LivingStatus};
use sonoro_proto::pb::{
    CombatCommon, CombatNotifyData, CombatNotifyDataMessageOneof, CombatReceiveData,
    CombatReceiveDataMessageOneof, DamageExecuteRequest, DamageExecuteResponse, DrownNotify,
    DrownPush, DrownRequest, DrownResponse, EntityLivingStatusNotify, FragileChangeRequest,
    FragileChangeResponse, HitEndPush, MonsterBoomPush, MonsterBoomRequest, MonsterBoomResponse,
    MonsterDrownPush, MonsterDrownRequest, MonsterDrownResponse, RoleGoDownNotify, RoleGoDownPush,
};

use super::State;

const KILLED_MONSTER_REMOVAL_DELAY_MS: i64 = 1800;

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| i64::try_from(d.as_millis()).unwrap_or(i64::MAX))
}

fn active_player_role_entity_id(scene: &Scene, common: Option<&CombatCommon>) -> Option<i64> {
    if let Some(c) = common
        && c.EntityId != 0
    {
        return Some(c.EntityId);
    }
    let f_idx = usize::try_from(scene.formation_info.cur_formation).ok()?;
    let form = scene.formation_info.formations.get(f_idx)?;
    form.roles
        .iter()
        .flatten()
        .find(|slot| slot.role_id == form.cur_role && slot.entity_id != 0)
        .map(|slot| slot.entity_id)
}

fn persist_role_hp(
    state: &mut State,
    events: &mut EventQueue,
    entity_id: i64,
    immediate_save: bool,
) -> bool {
    let Some((config_id, cur_hp)) = state.scene.as_ref().and_then(|scene| {
        let storage = scene.entity(entity_id)?;
        if storage.config.entity_type != EntityType::Player
            || storage.config.config_type != ConfigType::Character
        {
            return None;
        }
        let attr = storage.attribute.as_ref()?;
        let life = attributes::get_attr_value(attr, EAttributeType::Life).max(0);
        Some((storage.config.config_id, life))
    }) else {
        return false;
    };

    if let Some(role_info) = state.player_components.role.role_map.get_mut(&config_id)
        && role_info.current_hp != Some(cur_hp)
    {
        role_info.current_hp = Some(cur_hp);
        events.enqueue(Event::RoleInfoModified {
            player_id: state.player_id,
            role_id: config_id,
        });
        if immediate_save
            && !state.player_components.role.is_trial_role(config_id)
            && let Err(err) =
                role_helper::save_role_info(&state.fs, &state.player_components.role, config_id)
        {
            tracing::warn!(target: "damage", "save_role_info({config_id}) failed: {err}");
        }
        return true;
    }
    false
}

pub fn handle_damage_execute_request(
    state: &mut State,
    events: &mut EventQueue,
    receive_data_pack: &mut Vec<CombatReceiveData>,
    req: DamageExecuteRequest,
) -> DamageExecuteResponse {
    let Some(damage) = state
        .assets
        .tables
        .damage
        .get_data_by_id(&req.DamageId)
        .map(|d| d.into_owned())
    else {
        return DamageExecuteResponse {
            ErrorCode: Some(ErrorCode::ErrConfDamageNotFound),
            ..Default::default()
        };
    };

    let (execute_data, target_info) = {
        let Some(scene) = state.scene.as_mut() else {
            return DamageExecuteResponse {
                ErrorCode: Some(ErrorCode::InternalError),
                ..Default::default()
            };
        };
        let execute_data = damage_helper::damage_entity(
            receive_data_pack,
            &req,
            &damage,
            &state.assets,
            scene,
            Some(&mut state.buff_timers),
        );
        let target_info = scene.entity(execute_data.TargetEntityId).map(|storage| {
            let hp_ratio = storage.attribute.as_ref().map(|attr| {
                let cur_hp = attributes::get_attr_value(attr, EAttributeType::Life).max(0) as f32;
                let cur_max = attributes::get_attr_value(attr, EAttributeType::LifeMax);
                let base_max = attr
                    .attributes
                    .get(sonoro_proto::wire::PbEnum::to_i32(EAttributeType::LifeMax) as usize)
                    .map_or(1, |a| a.base);
                let max_hp = cur_max.max(base_max).max(1) as f32;
                cur_hp / max_hp
            });
            (
                storage.config.entity_type,
                storage.config.config_type,
                storage.config.config_id,
                hp_ratio,
            )
        });
        (execute_data, target_info)
    };

    if let Some((entity_type, config_type, _cfg_id, hp_ratio)) = target_info {
        let target_is_player = entity_type == EntityType::Player;
        if target_is_player
            && config_type == ConfigType::Character
            && persist_role_hp(
                state,
                events,
                execute_data.TargetEntityId,
                execute_data.KilledTarget,
            )
            && execute_data.KilledTarget
        {
            let formation_id = state
                .scene
                .as_ref()
                .map_or(0, |s| s.formation_info.cur_formation);
            events.enqueue(Event::UpdateFormations {
                player_id: state.player_id,
                formation_id,
                role_ids: Vec::new(),
            });
            if let Some(scene) = state.scene.as_ref() {
                role_helper::maybe_push_party_wipe_notify(&mut state.conn, scene);
            }
        }

        if execute_data.KilledTarget && !target_is_player {
            let mut qctx = state.quest_context(events);
            if let Err(err) = quest_engine::events::on_entity_killed(
                &mut qctx,
                execute_data.TargetEntityId,
                None,
            ) {
                tracing::warn!(target: "damage", "on_entity_killed failed: {err}");
            }
            if let Some(mut lp_ctx) = LevelPlayContext::from_quest_ctx(&mut qctx)
                && let Err(err) = level_play::on_entity_killed_with_conditions(
                    &mut lp_ctx,
                    execute_data.TargetEntityId,
                    None,
                )
            {
                tracing::warn!(target: "level_play", "on_entity_killed failed: {err}");
            }
            if let Some(scene) = state.scene.as_mut() {
                scene.queue_killed_monster_removal_at(
            execute_data.TargetEntityId,
            now_ms() + KILLED_MONSTER_REMOVAL_DELAY_MS,
        );
            }
        } else if !execute_data.KilledTarget
            && !target_is_player
            && let Some(ratio) = hp_ratio
        {
            let mut qctx = state.quest_context(events);
            if let Err(err) =
                quest_engine::events::on_entity_hit(&mut qctx, execute_data.TargetEntityId, ratio)
            {
                tracing::warn!(target: "damage", "on_entity_hit failed: {err}");
            }
        }
    }

    DamageExecuteResponse {
        ErrorCode: Some(ErrorCode::Success),
        AttackerEntityId: execute_data.AttackerEntityId,
        TargetEntityId: execute_data.TargetEntityId,
        Damage: execute_data.Damage,
        PartIndex: execute_data.PartIndex,
        IsCrit: execute_data.IsCrit,
        KilledTarget: execute_data.KilledTarget,
        ShieldCoverDamage: execute_data.ShieldCoverDamage,
        ImmuneType: execute_data.ImmuneType,
        ElementType: execute_data.ElementType,
        ChangeLife: execute_data.ChangeLife,
        ..Default::default()
    }
}

pub fn handle_hit_end_push(_state: &mut State, _events: &mut EventQueue, _push: HitEndPush) {}

pub fn handle_fragile_change_request(
    state: &mut State,
    common: Option<&CombatCommon>,
    receive_data_pack: &mut Vec<CombatReceiveData>,
    req: FragileChangeRequest,
) -> FragileChangeResponse {
    let eid = if req.EntityId != 0 {
        req.EntityId
    } else {
        common.map_or(0, |c| c.EntityId)
    };
    if eid != 0
        && let Some(scene) = state.scene.as_mut()
    {
        let changed = if let Some(&idx) = scene.net_id_map.get(&eid)
            && let Some(attr) = scene.entities[idx].attribute.as_mut()
        {
            let target_val = if req.Flag {
                0
            } else {
                attributes::get_attr_value(attr, EAttributeType::ToughMax).max(0)
            };
            let cur_tough = attributes::get_attr_value(attr, EAttributeType::Tough);
            if (req.Flag && cur_tough > 0) || (!req.Flag && cur_tough <= 0 && target_val > 0) {
                let ch = attributes::change_attr(
                    attr,
                    EAttributeType::Tough,
                    AttributeChangeType::Override,
                    AttributeValueType::Current,
                    target_val,
                );
                if !ch.is_empty() {
                    attributes::generate_attr_messages(receive_data_pack, eid, attr, &ch);
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };
        if changed {
            scene.mark_fsm_dirty(eid, wake_reason::ATTRIBUTE);
            let _ = scene.save_entity_by_net_id(&state.fs, eid);
        }
    }
    FragileChangeResponse {
        ErrorCode: Some(ErrorCode::Success),
    }
}

fn handle_drown_impl(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    receive_data_pack: &mut Vec<CombatReceiveData>,
) {
    let (eid, killed) = {
        let Some(scene) = state.scene.as_mut() else {
            return;
        };
        let Some(eid) = active_player_role_entity_id(scene, common) else {
            return;
        };
        let Some(&idx) = scene.net_id_map.get(&eid) else {
            return;
        };
        let Some(after_hp) = scene.entities[idx].attribute.as_mut().and_then(|attr| {
            let max_hp = attributes::get_attr_value(attr, EAttributeType::LifeMax).max(1);
            let cur_hp = attributes::get_attr_value(attr, EAttributeType::Life);
            if cur_hp <= 0 {
                return None;
            }
            let drown_dmg = (max_hp / 10).max(1);
            let change = attributes::change_attr(
                attr,
                EAttributeType::Life,
                AttributeChangeType::Delta,
                AttributeValueType::Current,
                -drown_dmg,
            );
            attributes::generate_attr_messages(receive_data_pack, eid, attr, &change);
            Some(attributes::get_attr_value(attr, EAttributeType::Life))
        }) else {
            return;
        };

        scene.mark_fsm_dirty(eid, wake_reason::ATTRIBUTE);
        let killed = after_hp <= 0;
        if killed {
            if scene.entities[idx].buffs.is_some() {
                let removed = buff_helper::handle_entity_death(
                    receive_data_pack,
                    scene,
                    &state.assets,
                    eid,
                );
                for hid in removed {
                    state.buff_timers.cancel_handle(eid, hid);
                }
            } else {
                scene.set_battle_entity_active(eid, false);
                receive_data_pack.push(CombatReceiveData {
                    Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
                        CombatNotifyData {
                            CombatCommon: Some(CombatCommon {
                                EntityId: eid,
                                ..Default::default()
                            }),
                            Message: Some(CombatNotifyDataMessageOneof::EntityLivingStatusNotify(
                                EntityLivingStatusNotify {
                                    Id: eid,
                                    LivingStatus: Some(LivingStatus::Dead),
                                    ..Default::default()
                                },
                            )),
                        },
                    )),
                });
            }
            let _ = scene.append_battle_state_notify(receive_data_pack);
        }
        receive_data_pack.push(CombatReceiveData {
            Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
                CombatNotifyData {
                    CombatCommon: Some(CombatCommon {
                        EntityId: eid,
                        ..Default::default()
                    }),
                    Message: Some(CombatNotifyDataMessageOneof::DrownNotify(DrownNotify {})),
                },
            )),
        });
        (eid, killed)
    };

    persist_role_hp(state, events, eid, killed);
    if killed {
        let formation_id = state
            .scene
            .as_ref()
            .map_or(0, |s| s.formation_info.cur_formation);
        events.enqueue(Event::UpdateFormations {
            player_id: state.player_id,
            formation_id,
            role_ids: Vec::new(),
        });
        if let Some(scene) = state.scene.as_ref() {
            role_helper::maybe_push_party_wipe_notify(&mut state.conn, scene);
        }
    }
}

fn kill_entity_direct(
    state: &mut State,
    events: &mut EventQueue,
    eid: i64,
    receive_data_pack: &mut Vec<CombatReceiveData>,
) {
    let cfg_opt = {
        let Some(scene) = state.scene.as_mut() else {
            return;
        };
        let Some(&idx) = scene.net_id_map.get(&eid) else {
            return;
        };
        let cfg = scene.entities[idx].config;
        if let Some(attr) = scene.entities[idx].attribute.as_mut() {
            let before_hp = attributes::get_attr_value(attr, EAttributeType::Life);
            if before_hp > 0 {
                let _ = attributes::set_life_unconditional(attr, 0);
                attributes::generate_attr_messages(
                    receive_data_pack,
                    eid,
                    attr,
                    &[EAttributeType::Life],
                );
            }
        }
        if scene.entities[idx].buffs.is_some() {
            let removed =
                buff_helper::handle_entity_death(receive_data_pack, scene, &state.assets, eid);
            for hid in removed {
                state.buff_timers.cancel_handle(eid, hid);
            }
        } else {
            scene.set_battle_entity_active(eid, false);
            receive_data_pack.push(CombatReceiveData {
                Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
                    CombatNotifyData {
                        CombatCommon: Some(CombatCommon {
                            EntityId: eid,
                            ..Default::default()
                        }),
                        Message: Some(CombatNotifyDataMessageOneof::EntityLivingStatusNotify(
                            EntityLivingStatusNotify {
                                Id: eid,
                                LivingStatus: Some(LivingStatus::Dead),
                                ..Default::default()
                            },
                        )),
                    },
                )),
            });
        }
        let _ = scene.append_battle_state_notify(receive_data_pack);
        Some(cfg)
    };

    let Some(cfg) = cfg_opt else {
        return;
    };
    if cfg.entity_type == EntityType::Player && cfg.config_type == ConfigType::Character {
        persist_role_hp(state, events, eid, true);
        let formation_id = state
            .scene
            .as_ref()
            .map_or(0, |s| s.formation_info.cur_formation);
        events.enqueue(Event::UpdateFormations {
            player_id: state.player_id,
            formation_id,
            role_ids: Vec::new(),
        });
        if let Some(scene) = state.scene.as_ref() {
            role_helper::maybe_push_party_wipe_notify(&mut state.conn, scene);
        }
        let mut qctx = state.quest_context(events);
        if let Err(err) = quest_engine::events::on_condition_state_changed(&mut qctx) {
            tracing::warn!(target: "damage", "kill_entity_direct player quest hook failed: {err}");
        }
    } else if cfg.entity_type != EntityType::Player {
        let mut qctx = state.quest_context(events);
        if let Err(err) = quest_engine::events::on_entity_killed(&mut qctx, eid, None) {
            tracing::warn!(target: "damage", "on_entity_killed failed: {err}");
        }
        if let Some(mut lp_ctx) = LevelPlayContext::from_quest_ctx(&mut qctx)
            && let Err(err) = level_play::on_entity_killed_with_conditions(&mut lp_ctx, eid, None)
        {
            tracing::warn!(target: "level_play", "on_entity_killed failed: {err}");
        }
        if let Some(scene) = state.scene.as_mut() {
            scene.queue_killed_monster_removal_at(eid, now_ms() + KILLED_MONSTER_REMOVAL_DELAY_MS);
        }
    }
}

pub fn handle_drown_request(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    receive_data_pack: &mut Vec<CombatReceiveData>,
    _req: DrownRequest,
) -> DrownResponse {
    handle_drown_impl(state, events, common, receive_data_pack);
    DrownResponse {
        ErrorCode: Some(ErrorCode::Success),
    }
}

pub fn handle_drown_push(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    receive_data_pack: &mut Vec<CombatReceiveData>,
    _push: DrownPush,
) {
    handle_drown_impl(state, events, common, receive_data_pack);
}

pub fn handle_monster_drown_request(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    receive_data_pack: &mut Vec<CombatReceiveData>,
    _req: MonsterDrownRequest,
) -> MonsterDrownResponse {
    if let Some(c) = common
        && c.EntityId != 0
    {
        kill_entity_direct(state, events, c.EntityId, receive_data_pack);
    }
    MonsterDrownResponse {
        ErrorCode: Some(ErrorCode::Success),
    }
}

pub fn handle_monster_drown_push(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    receive_data_pack: &mut Vec<CombatReceiveData>,
    _push: MonsterDrownPush,
) {
    if let Some(c) = common
        && c.EntityId != 0
    {
        kill_entity_direct(state, events, c.EntityId, receive_data_pack);
    }
}

pub fn handle_monster_boom_request(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    receive_data_pack: &mut Vec<CombatReceiveData>,
    _req: MonsterBoomRequest,
) -> MonsterBoomResponse {
    if let Some(c) = common
        && c.EntityId != 0
    {
        kill_entity_direct(state, events, c.EntityId, receive_data_pack);
    }
    MonsterBoomResponse {
        ErrorCode: Some(ErrorCode::Success),
    }
}

pub fn handle_monster_boom_push(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    receive_data_pack: &mut Vec<CombatReceiveData>,
    _push: MonsterBoomPush,
) {
    if let Some(c) = common
        && c.EntityId != 0
    {
        kill_entity_direct(state, events, c.EntityId, receive_data_pack);
    }
}

pub fn handle_role_go_down_push(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    _push: RoleGoDownPush,
) {
    let Some(eid) = state
        .scene
        .as_ref()
        .and_then(|scene| active_player_role_entity_id(scene, common))
    else {
        return;
    };
    let _ = state
        .conn
        .push_msg(RoleGoDownNotify::MSG_ID, &RoleGoDownNotify { EntityId: eid });
    let mut qctx = state.quest_context(events);
    if let Err(err) = quest_engine::events::on_condition_state_changed(&mut qctx) {
        tracing::warn!(target: "damage", "role_go_down quest hook failed: {err}");
    }
}
