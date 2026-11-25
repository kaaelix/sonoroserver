use std::time::{SystemTime, UNIX_EPOCH};

use sonoro_gamesv_fs::formation_info::{Formation, FormationRole};
use sonoro_gamesv_logic::component::player::PlayerRoleComponent;
use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::events::Event;
use sonoro_gamesv_logic::helpers::{entity_proto, quest_engine, role as role_helper};
use sonoro_gamesv_logic::scene::Scene;
use sonoro_gamesv_logic::templates::role_entity_templates;
use sonoro_proto::enums::ErrorCode;
use sonoro_proto::pb::{
    EntityAddNotify, EntityPb, EntityRemoveInfo, EntityRemoveNotify, FightFormation, FightRoleInfo,
    FightRoleInfos, GetFormationDataRequest, GetFormationDataResponse, GroupFormation,
    RenameFormationRequest, RenameFormationResponse, UpdateFormationRequest,
    UpdateFormationResponse, UpdateGroupFormationNotify,
};

use super::State;
use sonoro_gamesv_logic::sink::PushSink;

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_millis() as i64)
}

fn build_fight_formations(scene: &Scene) -> Vec<FightFormation> {
    scene
        .formation_info
        .formations
        .iter()
        .enumerate()
        .map(|(i, f)| FightFormation {
            FormationId: (i + 1) as i32,
            Name: f.name.as_deref().unwrap_or("").as_bytes().to_vec(),
            CurRole: f.cur_role,
            RoleIds: f.roles.iter().flatten().map(|r| r.role_id).collect(),
            IsCurrent: i as i32 == scene.formation_info.cur_formation,
        })
        .collect()
}

fn build_validated_formation_roles(
    role_comp: &PlayerRoleComponent,
    role_ids: &[i32],
    old_formation: Option<&Formation>,
) -> Option<[Option<FormationRole>; 3]> {
    let mut roles: [Option<FormationRole>; 3] = [None, None, None];
    let mut count = 0usize;

    for &role_id in role_ids {
        if role_id <= 0 || !role_comp.role_map.contains_key(&role_id) {
            return None;
        }
        if roles[..count]
            .iter()
            .flatten()
            .any(|existing| existing.role_id == role_id)
        {
            continue;
        }
        if count >= 3 {
            break;
        }

        let existing_entity_id = old_formation
            .and_then(|old| {
                old.roles
                    .iter()
                    .flatten()
                    .find(|r| r.role_id == role_id)
                    .map(|r| r.entity_id)
            })
            .unwrap_or(-1);

        roles[count] = Some(FormationRole {
            role_id,
            entity_id: existing_entity_id,
            on_stage_without_control: false,
        });
        count += 1;
    }

    Some(roles)
}

fn has_any_formation_role(roles: &[Option<FormationRole>; 3]) -> bool {
    roles.iter().any(Option::is_some)
}

fn contains_formation_role(roles: &[Option<FormationRole>; 3], role_id: i32) -> bool {
    roles.iter().flatten().any(|r| r.role_id == role_id)
}

fn first_formation_role_id(roles: &[Option<FormationRole>; 3]) -> Option<i32> {
    roles.iter().flatten().next().map(|r| r.role_id)
}

pub fn handle_update_formation_request(
    state: &mut State,
    events: &mut EventQueue,
    req: UpdateFormationRequest,
) -> Result<UpdateFormationResponse, String> {
    let Some(scene_ref) = state.scene.as_ref() else {
        return Ok(UpdateFormationResponse {
            ErrorCode: Some(ErrorCode::ErrFormationUnknown),
            ..Default::default()
        });
    };
    let formation_count = scene_ref.formation_info.formations.len() as i32;

    for fight_formation in &req.Formations {
        if fight_formation.FormationId < 1 || fight_formation.FormationId > formation_count {
            return Ok(UpdateFormationResponse {
                ErrorCode: Some(ErrorCode::ErrFormationIdOutOfRange),
                ..Default::default()
            });
        }
        let Some(validated) = build_validated_formation_roles(
            &state.player_components.role,
            &fight_formation.RoleIds,
            None,
        ) else {
            return Ok(UpdateFormationResponse {
                ErrorCode: Some(ErrorCode::ErrFightFormationRoleNotExist),
                ..Default::default()
            });
        };
        if fight_formation.IsCurrent && !has_any_formation_role(&validated) {
            return Ok(UpdateFormationResponse {
                ErrorCode: Some(ErrorCode::ErrFormationEmpty),
                ..Default::default()
            });
        }
    }

    let now_ms = now_ms();
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
        let scene = scene_opt
            .as_mut()
            .ok_or_else(|| "SceneNotFound".to_string())?;
        let role_comp = &mut player_components.role;
        let weapon_comp = &player_components.weapon;
        let echo_comp = &player_components.echo;

        for fight_formation in &req.Formations {
            let idx = (fight_formation.FormationId - 1) as usize;
            let existing_name = scene.formation_info.formations[idx].name.clone();

            if fight_formation.IsCurrent {
                let cur_idx = scene.formation_info.cur_formation.max(0) as usize;
                let old_formation = scene.formation_info.formations[cur_idx].clone();
                scene.formation_info.cur_formation = idx as i32;
                let Some(roles) = build_validated_formation_roles(
                    role_comp,
                    &fight_formation.RoleIds,
                    Some(&old_formation),
                ) else {
                    return Ok(UpdateFormationResponse {
                        ErrorCode: Some(ErrorCode::ErrFightFormationRoleNotExist),
                        ..Default::default()
                    });
                };

                let mut current_formation = Formation {
                    cur_role: fight_formation.CurRole,
                    name: existing_name,
                    roles,
                };

                let added_roles: Vec<i32> = current_formation
                    .roles
                    .iter()
                    .flatten()
                    .filter(|role| !contains_formation_role(&old_formation.roles, role.role_id))
                    .map(|role| role.role_id)
                    .collect();

                let removed_entity_ids: Vec<i64> = old_formation
                    .roles
                    .iter()
                    .flatten()
                    .filter(|role| !contains_formation_role(&current_formation.roles, role.role_id))
                    .map(|role| role.entity_id)
                    .collect();

                tracing::debug!(
                    target: "formation_update",
                    "added: {:?}, removed: {:?}",
                    added_roles,
                    removed_entity_ids
                );

                let mut remove_infos = Vec::new();
                for entity_id in removed_entity_ids {
                    if let Some(&entity_index) = scene.net_id_map.get(&entity_id) {
                        let (vision_ids, custom_ids) =
                            if let Some(concom) = scene.entities[entity_index].concomitant.as_ref() {
                                (
                                    concom.vision_entity_id.clone(),
                                    concom.custom_entity_ids.clone(),
                                )
                            } else {
                                (Vec::new(), Vec::new())
                            };
                        for concom_id in vision_ids {
                            scene.remove(fs, concom_id).map_err(|e| e.to_string())?;
                            remove_infos.push(EntityRemoveInfo {
                                EntityId: concom_id,
                                ..Default::default()
                            });
                        }
                        for concom_id in custom_ids {
                            scene.remove(fs, concom_id).map_err(|e| e.to_string())?;
                            remove_infos.push(EntityRemoveInfo {
                                EntityId: concom_id,
                                ..Default::default()
                            });
                        }
                    }
                    scene.remove(fs, entity_id).map_err(|e| e.to_string())?;
                    remove_infos.push(EntityRemoveInfo {
                        EntityId: entity_id,
                        ..Default::default()
                    });
                }
                conn.push(
                    EntityRemoveNotify::MSG_ID,
                    &EntityRemoveNotify {
                        IsRemove: true,
                        RemoveInfos: remove_infos,
                    },
                );

                let instance_dungeon = assets
                    .tables
                    .instance_dungeon
                    .get_data_by_id(&scene.instance_id)
                    .map(|r| r.into_owned())
                    .ok_or_else(|| "InstanceDungeonNotFound".to_string())?;

                let mut role_entity_pbs: Vec<EntityPb> = Vec::new();
                let mut concom_entity_pbs: Vec<EntityPb> = Vec::new();

                for role_id in added_roles {
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
                    )
                    .map_err(|e| e.to_string())?;
                    for slot in current_formation.roles.iter_mut().flatten() {
                        if slot.role_id == role_id {
                            slot.entity_id = entity.net_id;
                            break;
                        }
                    }

                    let entity_pb =
                        entity_proto::build(assets, scene, buff_timers, entity.net_id, now_ms)
                            .map_err(|e| e.to_string())?;
                    role_entity_pbs.push(entity_pb);

                    let (vision_ids, custom_ids) =
                        if let Some(concom) = scene.entities[entity.index].concomitant.as_ref() {
                            (
                                concom.vision_entity_id.clone(),
                                concom.custom_entity_ids.clone(),
                            )
                        } else {
                            (Vec::new(), Vec::new())
                        };
                    for concom_id in vision_ids.into_iter().chain(custom_ids) {
                        if !scene.net_id_map.contains_key(&concom_id) {
                            continue;
                        }
                        let concom_pb =
                            entity_proto::build(assets, scene, buff_timers, concom_id, now_ms)
                                .map_err(|e| e.to_string())?;
                        concom_entity_pbs.push(concom_pb);
                    }
                }

                conn.push(
                    EntityAddNotify::MSG_ID,
                    &EntityAddNotify {
                        RemoveTagIds: false,
                        EntityPbs: role_entity_pbs,
                    },
                );
                conn.push(
                    EntityAddNotify::MSG_ID,
                    &EntityAddNotify {
                        RemoveTagIds: false,
                        EntityPbs: concom_entity_pbs,
                    },
                );

                current_formation.cur_role = if contains_formation_role(
                    &current_formation.roles,
                    current_formation.cur_role,
                ) {
                    current_formation.cur_role
                } else if contains_formation_role(&current_formation.roles, old_formation.cur_role) {
                    old_formation.cur_role
                } else {
                    let old_slot = old_formation
                        .roles
                        .iter()
                        .position(|r| r.as_ref().is_some_and(|x| x.role_id == old_formation.cur_role))
                        .unwrap_or(0);
                    let mut picked = None;
                    let mut i = old_slot;
                    loop {
                        if let Some(r) = &current_formation.roles[i] {
                            picked = Some(r.role_id);
                            break;
                        }
                        if i == 0 {
                            break;
                        }
                        i -= 1;
                    }
                    picked.unwrap_or_else(|| first_formation_role_id(&current_formation.roles).unwrap_or(0))
                };

                scene.formation_info.formations[idx] = current_formation;
                let cf = &scene.formation_info.formations[idx];

                let fight_role_infos: Vec<FightRoleInfo> = cf
                    .roles
                    .iter()
                    .flatten()
                    .map(|slot| FightRoleInfo {
                        RoleId: slot.role_id,
                        EntityId: slot.entity_id,
                        OnStageWithoutControl: false,
                    })
                    .collect();

                let living_status = role_helper::formation_living_status(scene, cf);
                conn.push(
                    UpdateGroupFormationNotify::MSG_ID,
                    &UpdateGroupFormationNotify {
                        GroupFormation: vec![GroupFormation {
                            PlayerId: scene.player_id,
                            FightRoleInfos: vec![FightRoleInfos {
                                GroupType: 1,
                                FightRoleInfos: fight_role_infos,
                                CurRole: cf.cur_role,
                                LivingStatus: Some(living_status),
                                IsFixedLocation: false,
                            }],
                            CurrentGroupType: 1,
                        }],
                    },
                );
            } else {
                let Some(roles) =
                    build_validated_formation_roles(role_comp, &fight_formation.RoleIds, None)
                else {
                    return Ok(UpdateFormationResponse {
                        ErrorCode: Some(ErrorCode::ErrFightFormationRoleNotExist),
                        ..Default::default()
                    });
                };
                let resolved_cur_role = if contains_formation_role(&roles, fight_formation.CurRole) {
                    fight_formation.CurRole
                } else {
                    first_formation_role_id(&roles).unwrap_or(0)
                };
                scene.formation_info.formations[idx] = Formation {
                    cur_role: resolved_cur_role,
                    name: existing_name,
                    roles,
                };
            }
        }

        buff_timers.ensure_all_registered(scene, assets, now_ms);
        buff_timers.sync_all_left_durations(scene, now_ms);
        scene.save_persistent(fs).map_err(|e| e.to_string())?;
    }

    let cur_form = state.scene.as_ref().map_or(0, |s| s.formation_info.cur_formation);
    events.enqueue(Event::UpdateFormations {
        player_id: state.player_id,
        formation_id: cur_form + 1,
        role_ids: Vec::new(),
    });
    let mut qctx = state.quest_context(events);
    quest_engine::on_condition_state_changed(&mut qctx).map_err(|e| e.to_string())?;

    Ok(UpdateFormationResponse {
        ErrorCode: Some(ErrorCode::Success),
        ..Default::default()
    })
}

pub fn handle_get_formation_data_request(
    state: &mut State,
    _req: GetFormationDataRequest,
) -> Result<GetFormationDataResponse, String> {
    if let Some(scene) = state.scene.as_ref() {
        Ok(GetFormationDataResponse {
            ErrorCode: Some(ErrorCode::Success),
            Formations: build_fight_formations(scene),
        })
    } else {
        Ok(GetFormationDataResponse {
            ErrorCode: Some(ErrorCode::Success),
            ..Default::default()
        })
    }
}

pub fn handle_rename_formation_request(
    state: &mut State,
    events: &mut EventQueue,
    req: RenameFormationRequest,
) -> Result<RenameFormationResponse, String> {
    let Some(scene) = state.scene.as_mut() else {
        return Ok(RenameFormationResponse {
            ErrorCode: Some(ErrorCode::ErrFormationUnknown),
        });
    };
    if req.FormationId < 1 || req.FormationId > scene.formation_info.formations.len() as i32 {
        return Ok(RenameFormationResponse {
            ErrorCode: Some(ErrorCode::ErrFormationIdOutOfRange),
        });
    }
    let name_str = String::from_utf8_lossy(&req.Name);
    let trimmed = name_str.trim();
    if trimmed.is_empty() || trimmed.len() > 64 {
        return Ok(RenameFormationResponse {
            ErrorCode: Some(ErrorCode::ErrFightFormationNameLength),
        });
    }
    let idx = (req.FormationId - 1) as usize;
    scene.formation_info.formations[idx].name = Some(trimmed.to_string());
    scene.save_persistent(&state.fs).map_err(|e| e.to_string())?;
    events.enqueue(Event::UpdateFormations {
        player_id: state.player_id,
        formation_id: req.FormationId,
        role_ids: Vec::new(),
    });
    Ok(RenameFormationResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}
