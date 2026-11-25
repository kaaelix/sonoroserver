use sonoro_gamesv_data::asset_indexes::{entity_world_pos, entity_world_rot};
use sonoro_gamesv_data::tables::level_entity_config::LevelEntityConfig;
use sonoro_gamesv_fs::map_info::MapInfo;
use sonoro_gamesv_logic::component::comp_util;
use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::events::Event;
use sonoro_gamesv_logic::helpers::{gravity as gravity_helper, quest_actions, quest_engine, role as role_helper};
use sonoro_proto::enums::{ErrorCode, LivingStatus, TeleportReason};
use sonoro_proto::pb::{
    AddTemporaryTeleportInfoNotify, AddTemporaryTeleportRequest, AddTemporaryTeleportResponse,
    CombatReceivePackNotify, GameCtxPb, GameCtxPbCtxInfoOneof, LeaveSceneNotify,
    RemoveTemporaryTeleportNotify, RemoveTemporaryTeleportRequest,
    RemoveTemporaryTeleportResponse, TeleportDataRequest, TeleportDataResponse,
    TeleportFinishRequest, TeleportFinishResponse, TeleportNotify, TeleportToTargetRequest,
    TeleportToTargetResponse, TeleportTransferRequest, TeleportTransferResponse, TransferCtxPb,
    Vector,
};

use super::State;

#[derive(Debug, Clone, Copy)]
struct SpawnTransform {
    x: f32,
    y: f32,
    z: f32,
    yaw: f32,
    gravity: [f32; 3],
    has_gravity_config: bool,
}

fn resolve_teleporter_spawn(
    is_nexus: bool,
    entity_cfg: &LevelEntityConfig,
) -> Option<SpawnTransform> {
    let base_pos = entity_world_pos(entity_cfg)?;
    let base_rot = entity_world_rot(entity_cfg);
    let grav = gravity_helper::resolve_entity_gravity(entity_cfg, base_rot);

    let is_pillar_tp = is_nexus
        || (entity_cfg.BlueprintType.contains("Teleport")
            && !entity_cfg.BlueprintType.contains("TeleportPoint"));

    let mut ox = 0.0f32;
    let mut oy = if is_pillar_tp { 300.0 } else { 0.0 };
    let mut oz = 0.0f32;
    let mut yaw = base_rot[2];

    let tc_opt = entity_cfg
        .ComponentsData
        .TeleportComponent
        .as_ref()
        .or(entity_cfg.ComponentsData.DynamicTeleportComponent.as_ref());
    if let Some(tc) = tc_opt
        && let Some(p) = tc.TeleportPos.as_ref()
    {
        let px = p.X.unwrap_or(0.0);
        let py = p.Y.unwrap_or(0.0);
        if !is_pillar_tp || (px * px + py * py >= 1.0) {
            ox = px;
            oy = py;
        }
        oz = p.Z.unwrap_or(0.0);
        yaw = base_rot[2] + p.A.unwrap_or(0.0);
    }

    let rotated = gravity_helper::rotate_local_offset_ue([ox, oy, oz], base_rot);
    let cap_up = if grav.configured {
        [-grav.dir[0] * 85.0, -grav.dir[1] * 85.0, -grav.dir[2] * 85.0]
    } else {
        gravity_helper::rotate_local_offset_ue([0.0, 0.0, 85.0], base_rot)
    };

    Some(SpawnTransform {
        x: base_pos[0] + rotated[0] + cap_up[0],
        y: base_pos[1] + rotated[1] + cap_up[1],
        z: base_pos[2] + rotated[2] + cap_up[2],
        yaw,
        gravity: grav.dir,
        has_gravity_config: grav.configured,
    })
}

pub(crate) fn enqueue_scene_transfer(
    state: &mut State,
    events: &mut EventQueue,
    target_inst_id: i32,
    location: [f32; 3],
    yaw: f32,
) -> Result<(), String> {
    state
        .player_components
        .scene
        .last_scene_info
        .instance_id = target_inst_id;
    let _ = state.conn.push_msg(
        LeaveSceneNotify::MSG_ID,
        &LeaveSceneNotify {
            PlayerId: state.player_id,
            SceneId: Vec::new(),
            TransitionOption: Some(Default::default()),
        },
    );
    if let Some(scene) = state.scene.as_ref() {
        scene
            .save_last_scene(&state.player_components.scene, &state.fs)
            .map_err(|e| e.to_string())?;
    } else {
        comp_util::save_struct(
            &state.fs,
            &state.player_components.scene.last_scene_info,
            &format!("player/{}/last_scene", state.player_id),
        )
        .map_err(|e| e.to_string())?;
    }
    events.enqueue(Event::InitialSceneJoin {
        player_id: state.player_id,
        map_config_id: state.assets.resolve_map_config_id(target_inst_id),
        instance_id: target_inst_id,
        location: Some(location),
        rotation: Some([0.0, 0.0, yaw]),
        pending_flow: None,
    });
    Ok(())
}

pub fn handle_teleport_data_request(
    state: &mut State,
    _req: TeleportDataRequest,
) -> Result<TeleportDataResponse, String> {
    let mut ids = state.player_components.map.info.unlocked_teleporters.clone();
    for tp in &state.assets.tables.teleporter.items {
        if tp.Type == 3 && !ids.contains(&tp.Id) {
            ids.push(tp.Id);
        }
    }
    Ok(TeleportDataResponse {
        ErrorCode: Some(ErrorCode::Success),
        Ids: ids,
    })
}

pub fn handle_teleport_finish_request(
    _state: &mut State,
    _req: TeleportFinishRequest,
) -> Result<TeleportFinishResponse, String> {
    Ok(TeleportFinishResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_teleport_transfer_request(
    state: &mut State,
    events: &mut EventQueue,
    req: TeleportTransferRequest,
) -> Result<TeleportTransferResponse, String> {
    let Some(teleporter) = state
        .assets
        .tables
        .teleporter
        .get_data_by_id(&req.Id)
        .map(|t| t.into_owned())
    else {
        return Ok(TeleportTransferResponse {
            ErrorCode: Some(ErrorCode::ErrTeleportIdNotExist),
            ..Default::default()
        });
    };

    let Some(entity_cfg) = state
        .assets
        .find_level_entity(teleporter.MapId, teleporter.TeleportEntityConfigId)
        .map(|c| c.into_owned())
    else {
        return Ok(TeleportTransferResponse {
            ErrorCode: Some(ErrorCode::ErrTeleportCreatureIdNotExist),
            ..Default::default()
        });
    };

    let is_nexus = role_helper::is_resonance_nexus_config(
        &state.assets,
        teleporter.TeleportEntityConfigId,
    );
    let Some(spawn) = resolve_teleporter_spawn(is_nexus, &entity_cfg) else {
        return Ok(TeleportTransferResponse {
            ErrorCode: Some(ErrorCode::ErrTeleportCreatureIdNotExist),
            ..Default::default()
        });
    };

    let is_wiped = state.scene.as_ref().is_some_and(|scene| {
        usize::try_from(scene.formation_info.cur_formation)
            .ok()
            .and_then(|f_idx| scene.formation_info.formations.get(f_idx))
            .is_some_and(|form| role_helper::formation_living_status(scene, form) == LivingStatus::Dead)
    });
    if is_nexus || is_wiped {
        let mut scene = state.scene.take();
        if let Some(sc) = scene.as_mut() {
            role_helper::revive_and_heal_formation(
                &mut state.conn,
                sc,
                &mut state.player_components.role,
                events,
                &state.fs,
            );
        }
        state.scene = scene;
    }

    let cur_inst_id = state.player_components.scene.last_scene_info.instance_id;
    let cur_map_id = state.assets.resolve_map_config_id(cur_inst_id);
    let target_inst_id = state.assets.resolve_instance_id_for_map(teleporter.MapId);
    {
        let mut qctx = state.quest_context(events);
        if let Err(err) = quest_engine::events::on_player_transfer_or_leave_inst(
            &mut qctx,
            cur_map_id != 8 && teleporter.MapId == 8,
        ) {
            tracing::warn!(target: "teleport", "on_player_transfer_or_leave_inst failed: {err}");
        }
    }

    let target_pos = [spawn.x, spawn.y, spawn.z];
    role_helper::dismount_active_vehicle(
        &mut state.conn,
        state.scene.as_mut(),
        &mut state.player_components.progress,
        &state.assets,
        &state.fs,
        state.player_id,
    );
    if cur_inst_id == target_inst_id {
        if let Some(scene) = state.scene.as_mut() {
            if spawn.has_gravity_config {
                scene.set_current_role_gravity(spawn.gravity);
            }
            scene.force_battle_state(false);
            let mut battle_data = Vec::new();
            if scene.append_battle_state_notify(&mut battle_data) {
                let _ = state.conn.push_msg(
                    CombatReceivePackNotify::MSG_ID,
                    &CombatReceivePackNotify { Data: battle_data },
                );
            }
        }
        {
            let mut qctx = state.quest_context(events);
            quest_actions::update_player_scene_state(
                &mut qctx,
                target_pos,
                Some([0.0, 0.0, spawn.yaw]),
            )
            .map_err(|e| e.to_string())?;
        }
        let _ = state.conn.push_msg(
            TeleportNotify::MSG_ID,
            &TeleportNotify {
                MapId: teleporter.MapId,
                Pos: Some(Vector {
                    X: spawn.x,
                    Y: spawn.y,
                    Z: spawn.z,
                }),
                Rot: Some(Vector {
                    X: 0.0,
                    Y: 0.0,
                    Z: spawn.yaw,
                }),
                Gravity: spawn.has_gravity_config.then_some(Vector {
                    X: spawn.gravity[0],
                    Y: spawn.gravity[1],
                    Z: spawn.gravity[2],
                }),
                Reason: Some(TeleportReason::Transfer),
                GameCtx: Some(GameCtxPb {
                    CtxInfo: Some(GameCtxPbCtxInfoOneof::Transfer(TransferCtxPb {
                        TeleportId: teleporter.Id,
                    })),
                    ..Default::default()
                }),
                ..Default::default()
            },
        );
    } else {
        enqueue_scene_transfer(state, events, target_inst_id, target_pos, spawn.yaw)?;
    }

    Ok(TeleportTransferResponse {
        ErrorCode: Some(ErrorCode::Success),
        MapId: teleporter.MapId,
        PosX: spawn.x,
        PosY: spawn.y,
        PosZ: spawn.z,
        ..Default::default()
    })
}

pub fn handle_add_temporary_teleport_request(
    state: &mut State,
    req: AddTemporaryTeleportRequest,
) -> Result<AddTemporaryTeleportResponse, String> {
    let pos = req.Pos.unwrap_or_default();
    let rot = req.Rot.unwrap_or_default();
    let inst_id = state.scene.as_ref().map_or(8, |s| s.instance_id);
    let map_id = state.assets.resolve_map_config_id(inst_id);
    let mut tp = state
        .player_components
        .map
        .info
        .add_temp_teleport(map_id, req.Area, &pos, &rot);
    if let Some(last) = state.player_components.map.info.temp_teleports.last_mut() {
        last.inst_id = inst_id;
        tp.inst_id = inst_id;
    }
    comp_util::save_struct(
        &state.fs,
        &state.player_components.map.info,
        &format!("player/{}/{}", state.player_id, MapInfo::DATA_PATH),
    )
    .map_err(|e| e.to_string())?;

    let _ = state.conn.push_msg(
        AddTemporaryTeleportInfoNotify::MSG_ID,
        &AddTemporaryTeleportInfoNotify {
            TemporaryTeleportInfo: Some(tp.to_pb()),
        },
    );
    Ok(AddTemporaryTeleportResponse {
        TemporaryTeleportId: tp.id,
        Code: Some(ErrorCode::Success),
    })
}

pub fn handle_remove_temporary_teleport_request(
    state: &mut State,
    req: RemoveTemporaryTeleportRequest,
) -> Result<RemoveTemporaryTeleportResponse, String> {
    let removed = state
        .player_components
        .map
        .info
        .remove_temp_teleport(req.TemporaryTeleportId);
    if removed {
        comp_util::save_struct(
            &state.fs,
            &state.player_components.map.info,
            &format!("player/{}/{}", state.player_id, MapInfo::DATA_PATH),
        )
        .map_err(|e| e.to_string())?;
        let _ = state.conn.push_msg(
            RemoveTemporaryTeleportNotify::MSG_ID,
            &RemoveTemporaryTeleportNotify {
                TemporaryTeleportId: req.TemporaryTeleportId,
            },
        );
    }
    Ok(RemoveTemporaryTeleportResponse {
        ErrCode: Some(if removed {
            ErrorCode::Success
        } else {
            ErrorCode::ErrTeleportIdNotExist
        }),
    })
}

pub fn handle_teleport_to_target_request(
    state: &mut State,
    events: &mut EventQueue,
    req: TeleportToTargetRequest,
) -> Result<TeleportToTargetResponse, String> {
    let Some(tp) = state
        .player_components
        .map
        .info
        .find_temp_teleport(req.TemporaryTeleportId)
    else {
        return Ok(TeleportToTargetResponse {
            ErrCode: Some(ErrorCode::ErrTeleportIdNotExist),
        });
    };

    let yaw = req.Rot.map_or(tp.rot_yaw, |r| r.Yaw);
    let cur_inst_id = state.player_components.scene.last_scene_info.instance_id;
    let cur_map_id = state.assets.resolve_map_config_id(cur_inst_id);
    let tp_map_id = state.assets.resolve_map_config_id(tp.map_id);
    {
        let mut qctx = state.quest_context(events);
        if let Err(err) = quest_engine::events::on_player_transfer_or_leave_inst(
            &mut qctx,
            cur_map_id != 8 && tp_map_id == 8,
        ) {
            tracing::warn!(target: "teleport", "on_teleport_to_target failed: {err}");
        }
    }

    let target_pos = [tp.pos_x, tp.pos_y, tp.pos_z];
    role_helper::dismount_active_vehicle(
        &mut state.conn,
        state.scene.as_mut(),
        &mut state.player_components.progress,
        &state.assets,
        &state.fs,
        state.player_id,
    );
    if cur_map_id == tp_map_id {
        if let Some(scene) = state.scene.as_mut() {
            scene.force_battle_state(false);
            let mut battle_data = Vec::new();
            if scene.append_battle_state_notify(&mut battle_data) {
                let _ = state.conn.push_msg(
                    CombatReceivePackNotify::MSG_ID,
                    &CombatReceivePackNotify { Data: battle_data },
                );
            }
        }
        let mut qctx = state.quest_context(events);
        quest_actions::update_player_scene_state(&mut qctx, target_pos, Some([0.0, 0.0, yaw]))
            .map_err(|e| e.to_string())?;
        let _ = state.conn.push_msg(
            TeleportNotify::MSG_ID,
            &TeleportNotify {
                MapId: tp_map_id,
                Pos: Some(Vector {
                    X: target_pos[0],
                    Y: target_pos[1],
                    Z: target_pos[2],
                }),
                Rot: Some(Vector {
                    X: 0.0,
                    Y: 0.0,
                    Z: yaw,
                }),
                Reason: Some(TeleportReason::TemporaryTransfer),
                ..Default::default()
            },
        );
    } else {
        let target_inst_id = if tp.inst_id > 0
            && state
                .assets
                .tables
                .instance_dungeon
                .get_data_by_id(&tp.inst_id)
                .is_some()
        {
            tp.inst_id
        } else {
            state.assets.resolve_instance_id_for_map(tp_map_id)
        };
        enqueue_scene_transfer(state, events, target_inst_id, target_pos, yaw)?;
    }

    Ok(TeleportToTargetResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use sonoro_gamesv_data::tables::entity_components::base_info::{
        BaseInfoComponent, GravityConfigData, GravityDirection,
    };
    use sonoro_gamesv_data::tables::entity_components::components::Components;
    use sonoro_gamesv_data::tables::entity_components::teleport::{Pos, TeleportComponent};
    use sonoro_gamesv_data::tables::level_entity_config::Vector as CfgVector;

    #[test]
    fn resolve_teleporter_spawn_applies_offset_capsule_lift_gravity_and_pillar_fallback() {
        let transform_90 = vec![
            CfgVector {
                X: 100_000.0,
                Y: 200_000.0,
                Z: 30_000.0,
            },
            CfgVector {
                X: 0.0,
                Y: 0.0,
                Z: 9_000.0,
            },
            CfgVector {
                X: 100.0,
                Y: 100.0,
                Z: 100.0,
            },
        ];

        let cfg_with_pos = LevelEntityConfig {
            Id: 1,
            EntityId: 113_003_404,
            BlueprintType: "Teleport001".to_string(),
            InSleep: false,
            IsHidden: false,
            MapId: 8,
            AreaId: 1101,
            Transform: transform_90.clone(),
            ComponentsData: Components {
                TeleportComponent: Some(TeleportComponent {
                    Disabled: None,
                    TeleporterId: Some(300_011),
                    TeleportPos: Some(Pos {
                        X: Some(100.0),
                        Y: Some(200.0),
                        Z: Some(117.25),
                        A: Some(-45.0),
                    }),
                }),
                ..Default::default()
            },
            ..Default::default()
        };
        let s1 = resolve_teleporter_spawn(false, &cfg_with_pos).unwrap();
        assert!((s1.x - 800.0).abs() < 0.01);
        assert!((s1.y - 2100.0).abs() < 0.01);
        assert!((s1.z - 502.25).abs() < 0.01);
        assert!((s1.yaw - 45.0).abs() < 0.01);
        assert!(!s1.has_gravity_config);

        let cfg_null_z = LevelEntityConfig {
            Id: 2,
            EntityId: 113_003_404,
            BlueprintType: "branch2.0_TeleportPoint".to_string(),
            InSleep: false,
            IsHidden: false,
            MapId: 8,
            AreaId: 1101,
            Transform: transform_90.clone(),
            ComponentsData: Components {
                TeleportComponent: Some(TeleportComponent {
                    Disabled: None,
                    TeleporterId: Some(300_011),
                    TeleportPos: Some(Pos {
                        X: None,
                        Y: None,
                        Z: None,
                        A: Some(-10.0),
                    }),
                }),
                ..Default::default()
            },
            ..Default::default()
        };
        let s2 = resolve_teleporter_spawn(false, &cfg_null_z).unwrap();
        assert!((s2.x - 1000.0).abs() < 0.01);
        assert!((s2.y - 2000.0).abs() < 0.01);
        assert!((s2.z - 385.0).abs() < 0.01);
        assert!((s2.yaw - 80.0).abs() < 0.01);

        let cfg_pillar_fallback = LevelEntityConfig {
            Id: 3,
            EntityId: 113_003_404,
            BlueprintType: "Teleport3".to_string(),
            InSleep: false,
            IsHidden: false,
            MapId: 8,
            AreaId: 1101,
            Transform: transform_90,
            ComponentsData: Components::default(),
            ..Default::default()
        };
        let s3 = resolve_teleporter_spawn(false, &cfg_pillar_fallback).unwrap();
        assert!((s3.x - 700.0).abs() < 0.01);
        assert!((s3.y - 2000.0).abs() < 0.01);
        assert!((s3.z - 385.0).abs() < 0.01);

        let transform_inverted = vec![
            CfgVector {
                X: 100_000.0,
                Y: 200_000.0,
                Z: 30_000.0,
            },
            CfgVector {
                X: 18_000.0,
                Y: 0.0,
                Z: 0.0,
            },
            CfgVector {
                X: 100.0,
                Y: 100.0,
                Z: 100.0,
            },
        ];
        let cfg_inverted = LevelEntityConfig {
            Id: 4,
            EntityId: 169_750_416,
            BlueprintType: "Teleport2".to_string(),
            InSleep: false,
            IsHidden: false,
            MapId: 903,
            AreaId: 1,
            Transform: transform_inverted,
            ComponentsData: Components {
                BaseInfoComponent: Some(BaseInfoComponent {
                    GravityConfig: Some(GravityConfigData {
                        GravityDirection: Some(GravityDirection {
                            Type: Some("WorldAxis".to_string()),
                            WorldAxis: Some("PositiveZ".to_string()),
                            Direction: None,
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                TeleportComponent: Some(TeleportComponent {
                    Disabled: None,
                    TeleporterId: Some(390_302),
                    TeleportPos: Some(Pos {
                        X: Some(100.0),
                        Y: Some(200.0),
                        Z: None,
                        A: Some(90.0),
                    }),
                }),
                ..Default::default()
            },
            ..Default::default()
        };
        let s4 = resolve_teleporter_spawn(false, &cfg_inverted).unwrap();
        assert!((s4.x - 1100.0).abs() < 0.01);
        assert!((s4.y - 1800.0).abs() < 0.01);
        assert!((s4.z - 215.0).abs() < 0.01);
        assert!(s4.has_gravity_config);
        assert!((s4.gravity[2] - 1.0).abs() < 0.01);
    }
}
