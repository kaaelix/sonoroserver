use std::sync::OnceLock;

use sonoro_gamesv_data::Assets;
use sonoro_gamesv_fs::motor_info::{MotorInfo, Preset};
use sonoro_gamesv_logic::component::player::PlayerMotorComponent;
use sonoro_gamesv_logic::coop_service::CoopService;
use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::events::EntityHandle;
use sonoro_gamesv_logic::helpers::consume::CostItem;
use sonoro_gamesv_logic::helpers::reward::RewardItem;
use sonoro_gamesv_logic::helpers::{consume, entity_proto, pb_kv, quest_engine, reward};
use sonoro_gamesv_logic::scene::{Scene, now_millis};
use sonoro_gamesv_logic::templates::player_entity_templates;
use sonoro_proto::enums::{
    ErrorCode, ExitVehicleType, FollowerType, MotorTaskTypePb, RideMode, VehicleShareReason,
};
use sonoro_proto::pb::{
    ChangeVehicleRideSharingRequest, ChangeVehicleRideSharingResponse, EntityAddNotify,
    EntityMotorOutlookChangeNotify, EntityRemoveInfo, EntityRemoveNotify,
    MotorAnimReplaySampleUdpNotify, MotorAnimReplaySampleUdpPush, MotorChangeOutlookRequest,
    MotorChangeOutlookResponse, MotorCreateRequest, MotorCreateResponse, MotorCycleCruiseStatePush,
    MotorDecalMotorIpRequest, MotorDecalMotorIpResponse, MotorDecalTaskRequest,
    MotorDecalTaskResponse, MotorDevelopTaskRewardRequest, MotorDevelopTaskRewardResponse,
    MotorDiyEquippedPb, MotorDiyInfoRequest, MotorDiyInfoResponse, MotorDiyOnwedPb, MotorDiyPb,
    MotorFightCollectionDrop, MotorFightEnterSubLevelRequest, MotorFightEnterSubLevelResponse,
    MotorFightKillRequest, MotorFightKillResponse, MotorFightLastSaveData,
    MotorFightLastSaveDataRequest, MotorFightLastSaveDataResponse, MotorFightOneKeyRewardRequest,
    MotorFightOneKeyRewardResponse, MotorFightPlayerDeadRequest, MotorFightPlayerDeadResponse,
    MotorFightPlayerRankingInfo, MotorFightRankListRequest, MotorFightRankListResponse,
    MotorFightRankListSelfDataRequest, MotorFightRankListSelfDataResponse,
    MotorFightRefreshBossPush, MotorFightRefreshCollectionRequest,
    MotorFightRefreshCollectionResponse, MotorFightRefreshNextWaveGroupRequest,
    MotorFightRefreshNextWaveGroupResponse, MotorFightResult, MotorFightResultBase,
    MotorFightSelectBuffGateRequest, MotorFightSelectBuffGateResponse,
    MotorFightSelectCollectionRequest, MotorFightSelectCollectionResponse, MotorFightSettleRequest,
    MotorFightSettleResponse, MotorFightTalentUpRequest, MotorFightTalentUpResponse,
    MotorInfoRequest, MotorInfoResponse, MotorIsEnablePush, MotorLevelOneKeyRewardRequest,
    MotorLevelOneKeyRewardResponse, MotorLinkTaskRequest, MotorLinkTaskResponse,
    MotorLockedTechUpdateNotify, MotorOnlineProgressReportPush, MotorOutlookCreatePresetRequest,
    MotorOutlookCreatePresetResponse, MotorOutlookDeletePresetRequest,
    MotorOutlookDeletePresetResponse, MotorOutlookEditPresetRequest,
    MotorOutlookEditPresetResponse, MotorOutlookEquippedChangeNotify, MotorOutlookEquippedPb,
    MotorOutlookIdTimePairPb, MotorOutlookInfoRequest, MotorOutlookInfoResponse,
    MotorOutlookOwnedPb, MotorOutlookPb, MotorOutlookPlayerPresetPb, MotorOutlookPresetPlanPb,
    MotorOutlookRegionInfoNotify, MotorOutlookRegionPb, MotorParkourLevelRewardRequest,
    MotorParkourLevelRewardResponse, MotorPb, MotorPositionStatePush, MotorSliderRequest,
    MotorSliderResponse, MotorSummonAndRidePush, MotorTaskOneKeyRewardRequest,
    MotorTaskOneKeyRewardResponse, MotorTaskPb, MotorTaskProcessPb, MotorTaskRewardPb,
    MotorTaskTreePb, MotorTaskUpdateNotify, MotorTechLevelUpRequest, MotorTechLevelUpResponse,
    MotorTechOneTreePb, MotorTechPb, MotorTechTreeSwitchRequest, MotorTechTreeSwitchResponse,
    MotorUseSceneRequest, MotorUseSceneResponse, MotorUseSkinRequest, MotorUseSkinResponse,
    RemoveRideSharingPassengerRequest, RemoveRideSharingPassengerResponse,
    SendMovieModeRideSharingRequest, SendMovieModeRideSharingResponse,
    UpdateVehicleRideSharingNotify, VehicleShareNotify, VehicleUpdateEntityNotify,
};
use sonoro_proto::wire::PbEnum;

use super::State;

fn timed_id_list(items: &[i32]) -> Vec<MotorOutlookIdTimePairPb> {
    items
        .iter()
        .map(|&id| MotorOutlookIdTimePairPb { Id: id, OpenTime: 0 })
        .collect()
}

fn motor_task_type(value: i32) -> MotorTaskTypePb {
    match value {
        1 => MotorTaskTypePb::Single,
        2 => MotorTaskTypePb::Limited,
        3 => MotorTaskTypePb::Cycle,
        _ => MotorTaskTypePb::Unknown,
    }
}

fn build_tech_trees(motor_comp: &PlayerMotorComponent) -> Vec<MotorTechOneTreePb> {
    let mut tree_ids = Vec::new();
    for node in &motor_comp.info.tech_nodes {
        if !tree_ids.contains(&node.tree_type) {
            tree_ids.push(node.tree_type);
        }
    }
    tree_ids
        .into_iter()
        .map(|tree_id| build_motor_tech_tree_data(motor_comp, tree_id))
        .collect()
}

fn build_task_trees(motor_comp: &PlayerMotorComponent) -> Vec<MotorTaskTreePb> {
    let mut tree_ids = Vec::new();
    for task in &motor_comp.info.task_nodes {
        if !tree_ids.contains(&task.tree_type) {
            tree_ids.push(task.tree_type);
        }
    }
    tree_ids
        .into_iter()
        .map(|tree_id| {
            let mut tp_rewarded = 0i32;
            let mut tasks = Vec::new();
            for task in &motor_comp.info.task_nodes {
                if task.tree_type != tree_id {
                    continue;
                }
                tp_rewarded += task.rewarded;
                tasks.push(MotorTaskPb {
                    Id: task.id,
                    Type: Some(motor_task_type(task.r#type)),
                    Process: Some(MotorTaskProcessPb {
                        Current: task.current,
                        Target: task.target,
                    }),
                    Reward: Some(MotorTaskRewardPb {
                        Rewarded: task.rewarded,
                        WaitReward: task.wait_reward,
                        MaxReward: task.max_reward,
                    }),
                    ..Default::default()
                });
            }
            MotorTaskTreePb {
                TreeId: tree_id,
                Tasks: tasks,
                TpRewarded: tp_rewarded.max(100),
            }
        })
        .collect()
}

fn build_motor_tech_tree_data(
    motor_comp: &PlayerMotorComponent,
    tree_id: i32,
) -> MotorTechOneTreePb {
    let tech = motor_comp
        .info
        .tech_nodes
        .iter()
        .filter(|n| n.tree_type == tree_id)
        .map(|node| MotorTechPb {
            Id: node.id,
            Level: node.level,
            Unlock: node.unlocked,
            Current: node.current,
            Target: node.target,
        })
        .collect();
    MotorTechOneTreePb {
        TreeId: tree_id,
        Tech: tech,
    }
}

fn build_motor_task_list_data(
    motor_comp: &PlayerMotorComponent,
    tree_id: i32,
) -> Vec<MotorTaskPb> {
    motor_comp
        .info
        .task_nodes
        .iter()
        .filter(|t| t.tree_type == tree_id)
        .map(|task| MotorTaskPb {
            Id: task.id,
            Type: Some(motor_task_type(task.r#type)),
            Process: Some(MotorTaskProcessPb {
                Current: task.current,
                Target: task.target,
            }),
            Reward: Some(MotorTaskRewardPb {
                Rewarded: task.rewarded,
                WaitReward: task.wait_reward,
                MaxReward: task.max_reward,
            }),
            ..Default::default()
        })
        .collect()
}

fn part_index(part_id: i32, count: usize) -> Option<usize> {
    if part_id <= 0 {
        return None;
    }
    let idx = (part_id - 1) as usize;
    (idx < count).then_some(idx)
}

fn init_part_list(count: usize, previous: Option<&[i32]>) -> Vec<i32> {
    (0..count)
        .map(|i| previous.and_then(|p| p.get(i).copied()).unwrap_or(0))
        .collect()
}

fn normalize_stickers(
    assets: &Assets,
    source: &[i32],
    previous: Option<&[i32]>,
) -> Result<Vec<i32>, ErrorCode> {
    let mut result = init_part_list(assets.tables.motor_sticker_part.items.len(), previous);
    for (i, &id) in source.iter().enumerate() {
        if id == 0 {
            if let Some(slot) = result.get_mut(i) {
                *slot = 0;
            }
            continue;
        }
        let sticker = assets
            .tables
            .motor_sticker
            .get_data_by_id(&id)
            .ok_or(ErrorCode::MotorStickerCfgNotFound)?;
        let idx = part_index(sticker.PartId, result.len())
            .ok_or(ErrorCode::MotorStickerPartIdError)?;
        result[idx] = id;
    }
    Ok(result)
}

fn normalize_decorations(
    assets: &Assets,
    source: &[i32],
    previous: Option<&[i32]>,
) -> Result<Vec<i32>, ErrorCode> {
    let mut result = init_part_list(assets.tables.motor_decorations_part.items.len(), previous);
    for (i, &id) in source.iter().enumerate() {
        if id == 0 {
            if let Some(slot) = result.get_mut(i) {
                *slot = 0;
            }
            continue;
        }
        let deco = assets
            .tables
            .motor_decorations
            .get_data_by_id(&id)
            .ok_or(ErrorCode::MotorDecorationsCfgNotFound)?;
        let idx = part_index(deco.PartId, result.len())
            .ok_or(ErrorCode::MotorDecorationsPartIdError)?;
        result[idx] = id;
    }
    Ok(result)
}

fn resolve_frame(assets: &Assets, requested: i32, previous: i32) -> Result<i32, ErrorCode> {
    let frame = if requested == 0 { previous } else { requested };
    if frame != 0 && assets.tables.motor_frame.get_data_by_id(&frame).is_none() {
        return Err(ErrorCode::MotorFrameCfgNotFound);
    }
    Ok(frame)
}

fn build_preset_data(preset: &Preset) -> MotorOutlookPresetPlanPb {
    MotorOutlookPresetPlanPb {
        Id: preset.id,
        Mame: preset.name.as_bytes().to_vec(),
        Preset: Some(MotorDiyEquippedPb {
            SkinEquipped: preset.skin,
            StickerEquipped: preset.stickers.clone(),
            DecorationsEquipped: preset.decorations.clone(),
            FrameEquipped: preset.frame,
        }),
    }
}

fn build_preset_list(motor_comp: &PlayerMotorComponent) -> MotorOutlookPlayerPresetPb {
    MotorOutlookPlayerPresetPb {
        Plan: motor_comp.info.presets.iter().map(build_preset_data).collect(),
    }
}

fn build_equipped_outlook(motor_comp: &PlayerMotorComponent) -> MotorDiyEquippedPb {
    MotorDiyEquippedPb {
        SkinEquipped: motor_comp.info.equipped_skin,
        StickerEquipped: motor_comp.info.equipped_stickers.clone(),
        DecorationsEquipped: motor_comp.info.equipped_decorations.clone(),
        FrameEquipped: motor_comp.info.equipped_frame,
    }
}

fn find_motor_entity(scene: &Scene, assets: &Assets) -> Option<EntityHandle> {
    let motor_cfg_id = assets.get_motorcycle_config_id()?;
    let motor_follower_type = FollowerType::EPlayerFollowerMotor.to_i32();
    for e in &scene.entities {
        let Some(follower) = &e.follower else {
            continue;
        };
        for entry in &follower.list {
            if entry.kind != motor_follower_type || entry.entity_id == 0 {
                continue;
            }
            if let Some(h) = scene.entity_handle(entry.entity_id)
                && scene.entities[h.index].config.config_id == motor_cfg_id
            {
                return Some(h);
            }
        }
    }
    for (idx, e) in scene.entities.iter().enumerate() {
        if e.config.config_id == motor_cfg_id {
            return Some(EntityHandle {
                index: idx,
                net_id: e.net_id,
            });
        }
    }
    None
}

fn is_formation_entity(scene: &Scene, entity_id: i64) -> bool {
    scene
        .formation_info
        .formations
        .iter()
        .any(|f| f.roles.iter().flatten().any(|r| r.entity_id == entity_id))
}

fn push_entity_add_notify(state: &mut State, entity: EntityHandle, now_ms: i64) {
    let mut role_entity_pbs = Vec::new();
    let mut concom_entity_pbs = Vec::new();
    let concom_ids: Vec<i64> = {
        let Some(scene) = state.scene.as_mut() else {
            return;
        };
        let concom = scene
            .entities
            .get(entity.index)
            .and_then(|s| s.concomitant.as_ref())
            .map(|c| c.custom_entity_ids.clone())
            .unwrap_or_default();
        if let Ok(pb) =
            entity_proto::build(&state.assets, scene, &mut state.buff_timers, entity.net_id, now_ms)
        {
            role_entity_pbs.push(pb);
        }
        concom
    };

    if let Some(scene) = state.scene.as_mut() {
        for cid in concom_ids {
            if scene.net_id_map.contains_key(&cid)
                && let Ok(pb) =
                    entity_proto::build(&state.assets, scene, &mut state.buff_timers, cid, now_ms)
            {
                concom_entity_pbs.push(pb);
            }
        }
    }

    let _ = state.conn.push_msg(
        EntityAddNotify::MSG_ID,
        &EntityAddNotify {
            RemoveTagIds: false,
            EntityPbs: role_entity_pbs,
        },
    );
    if !concom_entity_pbs.is_empty() {
        let _ = state.conn.push_msg(
            EntityAddNotify::MSG_ID,
            &EntityAddNotify {
                RemoveTagIds: false,
                EntityPbs: concom_entity_pbs,
            },
        );
    }
}

fn push_entity_refresh_notify(state: &mut State, entity: EntityHandle, now_ms: i64) {
    let mut remove_infos = Vec::new();
    if let Some(scene) = state.scene.as_ref()
        && let Some(storage) = scene.entities.get(entity.index)
        && let Some(concom) = &storage.concomitant
    {
        for &cid in &concom.custom_entity_ids {
            remove_infos.push(EntityRemoveInfo {
                EntityId: cid,
                ..Default::default()
            });
        }
    }
    remove_infos.push(EntityRemoveInfo {
        EntityId: entity.net_id,
        ..Default::default()
    });
    let _ = state.conn.push_msg(
        EntityRemoveNotify::MSG_ID,
        &EntityRemoveNotify {
            IsRemove: true,
            RemoveInfos: remove_infos,
        },
    );
    push_entity_add_notify(state, entity, now_ms);
}

fn push_entity_remove_notify(state: &mut State, entity: EntityHandle) -> Result<(), String> {
    let mut remove_infos = Vec::new();
    {
        let Some(scene) = state.scene.as_mut() else {
            return Ok(());
        };
        if is_formation_entity(scene, entity.net_id) {
            return Ok(());
        }
        let concom_ids = scene
            .entities
            .get(entity.index)
            .and_then(|s| s.concomitant.as_ref())
            .map(|c| c.custom_entity_ids.clone())
            .unwrap_or_default();
        for cid in concom_ids {
            let _ = scene.remove(&state.fs, cid);
            remove_infos.push(EntityRemoveInfo {
                EntityId: cid,
                ..Default::default()
            });
        }
        scene.remove(&state.fs, entity.net_id).map_err(|e| e.to_string())?;
        remove_infos.push(EntityRemoveInfo {
            EntityId: entity.net_id,
            ..Default::default()
        });
    }
    let _ = state.conn.push_msg(
        EntityRemoveNotify::MSG_ID,
        &EntityRemoveNotify {
            IsRemove: true,
            RemoveInfos: remove_infos,
        },
    );
    Ok(())
}

fn push_ride_sharing_seat_notify(
    state: &mut State,
    role_entity_id: i64,
    motor_entity_id: i64,
    seat: i32,
    occupied: bool,
) {
    let _ = state.conn.push_msg(
        VehicleUpdateEntityNotify::MSG_ID,
        &VehicleUpdateEntityNotify {
            EntityId: role_entity_id,
            VehicleCreatureId: motor_entity_id,
            Seat: seat,
            IsEntering: occupied,
            ExitType: Some(ExitVehicleType::ExitVehicleTypeSeatStandUp),
            ..Default::default()
        },
    );
}

fn update_motor_outlook_entity(state: &mut State) -> Option<EntityHandle> {
    let motor_comp = &state.player_components.motor;
    let scene = state.scene.as_mut()?;
    let entity = find_motor_entity(scene, &state.assets)?;
    let storage = &mut scene.entities[entity.index];
    if let Some(outlook) = storage.motor_outlook.as_mut() {
        outlook.skin = motor_comp.info.equipped_skin;
        outlook.stickers = motor_comp.info.equipped_stickers.clone();
        outlook.decorations = motor_comp.info.equipped_decorations.clone();
        outlook.frame = motor_comp.info.equipped_frame;
    } else {
        storage.motor_outlook = Some(
            sonoro_gamesv_logic::component::entity::small::MotorOutlookComponent {
                skin: motor_comp.info.equipped_skin,
                stickers: motor_comp.info.equipped_stickers.clone(),
                decorations: motor_comp.info.equipped_decorations.clone(),
                frame: motor_comp.info.equipped_frame,
            },
        );
    }
    Some(entity)
}

fn make_preset(
    id: i32,
    name_bytes: Vec<u8>,
    equipped: &MotorDiyEquippedPb,
    fallback_skin: i32,
) -> Preset {
    Preset {
        id,
        name: String::from_utf8_lossy(&name_bytes).into_owned(),
        skin: if equipped.SkinEquipped == 0 {
            fallback_skin
        } else {
            equipped.SkinEquipped
        },
        stickers: equipped.StickerEquipped.clone(),
        decorations: equipped.DecorationsEquipped.clone(),
        frame: equipped.FrameEquipped,
    }
}

#[allow(dead_code)]
pub fn build_motor_outlook_pb(motor_comp: &PlayerMotorComponent) -> MotorOutlookPb {
    MotorOutlookPb {
        MotorOutlookOwned: Some(MotorOutlookOwnedPb {
            SkinOwned: motor_comp.info.owned_skins.clone(),
            StickerOwned: motor_comp.info.owned_stickers.clone(),
            DecorationsOwned: motor_comp.info.owned_decorations.clone(),
            FrameOwned: motor_comp.info.owned_frames.clone(),
        }),
        MotorOutlookEquipped: Some(MotorOutlookEquippedPb {
            SkinEquipped: motor_comp.info.equipped_skin,
            StickerEquipped: motor_comp.info.equipped_stickers.clone(),
            DecorationsEquipped: motor_comp.info.equipped_decorations.clone(),
            FrameEquipped: motor_comp.info.equipped_frame,
        }),
        MotorOutlookPreset: Some(build_preset_list(motor_comp)),
        LatestMotorSkinSuit: Vec::new(),
        SceneInUse: motor_comp.info.scene_in_use,
    }
}

#[allow(dead_code)]
fn build_motor_diy(motor_comp: &PlayerMotorComponent) -> MotorDiyPb {
    MotorDiyPb {
        MotorDiyOnwer: Some(MotorDiyOnwedPb {
            SkinOwned: motor_comp.info.owned_skins.clone(),
            StickerOnwed: motor_comp.info.owned_stickers.clone(),
            DecorationsOwned: motor_comp.info.owned_decorations.clone(),
            FrameOwned: motor_comp.info.owned_frames.clone(),
        }),
        MotorDiyEquipped: Some(build_equipped_outlook(motor_comp)),
        MotorOutlookPreset: Some(build_preset_list(motor_comp)),
        LatestMotorSkinSuit: Vec::new(),
        SceneInUse: motor_comp.info.scene_in_use,
    }
}

pub fn handle_motor_info_request(
    state: &mut State,
    _req: MotorInfoRequest,
) -> Result<MotorInfoResponse, String> {
    let motor_comp = &state.player_components.motor;
    Ok(MotorInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        Motor: Some(MotorPb {
            MotorLevel: motor_comp.info.level,
            MotorExp: motor_comp.info.exp,
            MotorRewardedLvMax: motor_comp.info.rewarded_max_level,
            UnlockedTree: build_tech_trees(motor_comp),
            TreeInUse: motor_comp.info.tree_in_use,
            TaskTrees: build_task_trees(motor_comp),
            MotorExpLimitGainDaily: motor_comp.info.daily_exp_gain,
            MotorExpMonsterDropDailyLimit: motor_comp.info.daily_exp_limit,
        }),
    })
}

pub fn handle_motor_diy_info_request(
    state: &mut State,
    _req: MotorDiyInfoRequest,
) -> Result<MotorDiyInfoResponse, String> {
    let region = MotorOutlookRegionPb {
        MotorSticker: timed_id_list(&state.player_components.motor.info.owned_stickers),
        MotorDecoration: timed_id_list(&state.player_components.motor.info.owned_decorations),
        MotorFrame: timed_id_list(&state.player_components.motor.info.owned_frames),
        ..Default::default()
    };
    let _ = state.conn.push_msg(
        MotorOutlookRegionInfoNotify::MSG_ID,
        &MotorOutlookRegionInfoNotify {
            MotorOutlookRegion: Some(region),
        },
    );
    Ok(MotorDiyInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        MotorDiy: Some(build_motor_diy(&state.player_components.motor)),
    })
}

#[allow(dead_code)]
pub fn handle_motor_outlook_info_request(
    state: &mut State,
    _req: MotorOutlookInfoRequest,
) -> Result<MotorOutlookInfoResponse, String> {
    Ok(MotorOutlookInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        MotorOutlook: Some(build_motor_outlook_pb(&state.player_components.motor)),
    })
}

pub fn handle_motor_use_scene_request(
    state: &mut State,
    req: MotorUseSceneRequest,
) -> Result<MotorUseSceneResponse, String> {
    let Some(scene_cfg) = state
        .assets
        .tables
        .motor_scene
        .get_data_by_id(&req.SceneId)
    else {
        return Ok(MotorUseSceneResponse {
            ErrorCode: Some(ErrorCode::MotorSceneCfgNotFound),
            SceneInUse: 0,
        });
    };
    if scene_cfg.UnlockItem != 0
        && state
            .player_components
            .inventory
            .info
            .normal_item_count(scene_cfg.UnlockItem)
            <= 0
    {
        return Ok(MotorUseSceneResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
            SceneInUse: 0,
        });
    }
    state.player_components.motor.info.scene_in_use = scene_cfg.Id;
    state
        .player_components
        .motor
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(MotorUseSceneResponse {
        ErrorCode: Some(ErrorCode::Success),
        SceneInUse: scene_cfg.Id,
    })
}

pub fn handle_motor_create_request(
    state: &mut State,
    events: &mut EventQueue,
    req: MotorCreateRequest,
) -> Result<MotorCreateResponse, String> {
    let val = i64::from(req.IsCreate);
    state
        .player_components
        .progress
        .info
        .set_key_int("motor:created", val);
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "motor:summoned",
        val,
    )
    .map_err(|e| e.to_string())?;
    let mut qctx = state.quest_context(events);
    quest_engine::events::on_condition_state_changed(&mut qctx).map_err(|e| e.to_string())?;
    Ok(MotorCreateResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

#[allow(dead_code)]
pub fn handle_send_movie_mode_ride_sharing_request(
    state: &mut State,
    req: SendMovieModeRideSharingRequest,
) -> Result<SendMovieModeRideSharingResponse, String> {
    let _ = state.conn.push_msg(
        VehicleShareNotify::MSG_ID,
        &VehicleShareNotify {
            PlayerId: state.player_id,
            ShareRideMode: req.ShareRideMode.or(Some(RideMode::ShareRideMode_MovieMotor)),
            IsInMovieRideSharingMode: req.IsInMovieRideSharingMode,
            Reason: Some(VehicleShareReason::ClientRequest),
        },
    );
    Ok(SendMovieModeRideSharingResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

#[allow(dead_code)]
pub fn handle_change_vehicle_ride_sharing_request(
    state: &mut State,
    req: ChangeVehicleRideSharingRequest,
) -> Result<ChangeVehicleRideSharingResponse, String> {
    const PASSENGER_SEAT: i32 = 1;
    let now_ms = now_millis();

    if req.RoleId <= 0 || !state.player_components.role.role_map.contains_key(&req.RoleId) {
        return Ok(ChangeVehicleRideSharingResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    }

    let (motor_entity, player_scene_entity) = {
        let Some(scene) = state.scene.as_ref() else {
            return Ok(ChangeVehicleRideSharingResponse {
                ErrorCode: Some(ErrorCode::InternalError),
            });
        };
        let Some(m) = find_motor_entity(scene, &state.assets) else {
            return Ok(ChangeVehicleRideSharingResponse {
                ErrorCode: Some(ErrorCode::InternalError),
            });
        };
        let Some(p) = player_entity_templates::find_player_scene_entity(scene, state.player_id)
        else {
            return Ok(ChangeVehicleRideSharingResponse {
                ErrorCode: Some(ErrorCode::InternalError),
            });
        };
        (m, p)
    };

    let companion = {
        let Some(scene) = state.scene.as_mut() else {
            return Ok(ChangeVehicleRideSharingResponse {
                ErrorCode: Some(ErrorCode::InternalError),
            });
        };
        player_entity_templates::ensure_motorcycle_companion_entity(
            scene,
            state.player_id,
            &state.assets,
            motor_entity,
            player_scene_entity,
        )
        .map_err(|e| e.to_string())?
    };
    if companion.created {
        if let Some(scene) = state.scene.as_ref() {
            let _ = scene.save_entity(&state.fs, companion.entity);
            let _ = scene.save_entity(&state.fs, motor_entity);
        }
        push_entity_add_notify(state, companion.entity, now_ms);
    }

    let old_passenger = state
        .scene
        .as_ref()
        .and_then(|scene| player_entity_templates::find_motorcycle_passenger_entity(scene, &state.assets));
    if let Some(old_p) = old_passenger {
        push_ride_sharing_seat_notify(state, old_p.net_id, motor_entity.net_id, PASSENGER_SEAT, false);
        push_entity_remove_notify(state, old_p)?;
    }

    let passenger_entity = {
        let Some(scene) = state.scene.as_mut() else {
            return Ok(ChangeVehicleRideSharingResponse {
                ErrorCode: Some(ErrorCode::InternalError),
            });
        };
        match player_entity_templates::create_motorcycle_passenger_entity(
            scene,
            state.player_id,
            &state.assets,
            req.RoleId,
            motor_entity,
            PASSENGER_SEAT,
        ) {
            Ok(e) => e,
            Err("MotorcyclePassengerTemplateNotFound") => {
                return Ok(ChangeVehicleRideSharingResponse {
                    ErrorCode: Some(ErrorCode::RequestParamError),
                });
            }
            Err(e) => return Err(e.to_string()),
        }
    };
    push_entity_add_notify(state, passenger_entity, now_ms);
    let _ = state.conn.push_msg(
        UpdateVehicleRideSharingNotify::MSG_ID,
        &UpdateVehicleRideSharingNotify {
            PlayerId: state.player_id,
            RoleId: req.RoleId,
            Seat: PASSENGER_SEAT,
            EntityId: passenger_entity.net_id,
        },
    );
    push_ride_sharing_seat_notify(
        state,
        passenger_entity.net_id,
        motor_entity.net_id,
        PASSENGER_SEAT,
        true,
    );
    Ok(ChangeVehicleRideSharingResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

#[allow(dead_code)]
pub fn handle_remove_ride_sharing_passenger_request(
    state: &mut State,
    _req: RemoveRideSharingPassengerRequest,
) -> Result<RemoveRideSharingPassengerResponse, String> {
    const PASSENGER_SEAT: i32 = 1;
    let found = state.scene.as_ref().and_then(|scene| {
        let p = player_entity_templates::find_motorcycle_passenger_entity(scene, &state.assets)?;
        let m = find_motor_entity(scene, &state.assets);
        Some((p, m))
    });
    if let Some((entity, motor_opt)) = found {
        if let Some(motor_entity) = motor_opt {
            push_ride_sharing_seat_notify(
                state,
                entity.net_id,
                motor_entity.net_id,
                PASSENGER_SEAT,
                false,
            );
        }
        let _ = state.conn.push_msg(
            UpdateVehicleRideSharingNotify::MSG_ID,
            &UpdateVehicleRideSharingNotify {
                PlayerId: state.player_id,
                RoleId: 0,
                Seat: PASSENGER_SEAT,
                EntityId: entity.net_id,
            },
        );
        push_entity_remove_notify(state, entity)?;
    }
    Ok(RemoveRideSharingPassengerResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

fn load_motor_level_reward_id(assets: &Assets, target_level: i32) -> i32 {
    static REWARD_BY_LVL: OnceLock<Vec<(i32, i32)>> = OnceLock::new();
    let table = REWARD_BY_LVL.get_or_init(|| {
        assets
            .read_raw_json_array("MotorLvl")
            .into_iter()
            .filter_map(|entry| {
                let lvl = entry.get("Level")?.as_i64()? as i32;
                let rid = entry.get("RewardId").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                Some((lvl, rid))
            })
            .collect()
    });
    table
        .iter()
        .find(|&&(lvl, _)| lvl == target_level)
        .map_or(0, |&(_, rid)| rid)
}

fn check_motor_outlook_ownership(
    motor_comp: &PlayerMotorComponent,
    assets: &Assets,
    frame: i32,
    stickers: &[i32],
    decorations: &[i32],
) -> bool {
    if frame != 0 {
        let Some(fcfg) = assets.tables.motor_frame.get_data_by_id(&frame) else {
            return false;
        };
        if !fcfg.DefaultFlag && !MotorInfo::has_owned_id(&motor_comp.info.owned_frames, frame) {
            return false;
        }
    }
    for &sid in stickers {
        if sid == 0 {
            continue;
        }
        let Some(scfg) = assets.tables.motor_sticker.get_data_by_id(&sid) else {
            return false;
        };
        if !scfg.FreeFlag && !MotorInfo::has_owned_id(&motor_comp.info.owned_stickers, sid) {
            return false;
        }
    }
    for &did in decorations {
        if did == 0 {
            continue;
        }
        let Some(dcfg) = assets.tables.motor_decorations.get_data_by_id(&did) else {
            return false;
        };
        if !dcfg.FreeFlag
            && !MotorInfo::has_owned_id(&motor_comp.info.owned_decorations, did)
        {
            return false;
        }
    }
    true
}

pub fn handle_motor_tech_level_up_request(
    state: &mut State,
    events: &mut EventQueue,
    req: MotorTechLevelUpRequest,
) -> Result<MotorTechLevelUpResponse, String> {
    let Some(tech_cfg) = state.assets.tables.motor_tech.get_data_by_id(&req.TechId) else {
        return Ok(MotorTechLevelUpResponse {
            ErrorCode: Some(ErrorCode::MotorTechCfgNotFound),
            Tree: None,
        });
    };

    for &pre_id in &tech_cfg.PreNode {
        let pre_unlocked = state
            .player_components
            .motor
            .info
            .tech_nodes
            .iter()
            .any(|n| n.id == pre_id && n.level > 0);
        if !pre_unlocked {
            return Ok(MotorTechLevelUpResponse {
                ErrorCode: Some(ErrorCode::MotorTechPreNodeLock),
                Tree: None,
            });
        }
    }

    let Some(node_idx) = state
        .player_components
        .motor
        .info
        .tech_nodes
        .iter()
        .position(|n| n.id == req.TechId)
    else {
        return Ok(MotorTechLevelUpResponse {
            ErrorCode: Some(ErrorCode::MotorTechCfgNotFound),
            Tree: None,
        });
    };

    let (node_level, node_target, tree_type) = {
        let n = &state.player_components.motor.info.tech_nodes[node_idx];
        (n.level, n.target, n.tree_type)
    };
    if node_level < 0 || node_level >= node_target || (node_level as usize) >= tech_cfg.TechLv.len() {
        return Ok(MotorTechLevelUpResponse {
            ErrorCode: Some(ErrorCode::MotorTechLevelMax),
            Tree: None,
        });
    }

    let tech_lv_id = tech_cfg.TechLv[node_level as usize];
    if let Some(lv_cfg) = state.assets.tables.motor_tech_lv.get_data_by_id(&tech_lv_id)
        && lv_cfg.Consume > 0
    {
        let tp_item_id = state
            .assets
            .tables
            .motor_tech_tree
            .get_data_by_id(&tree_type)
            .map_or(0, |t| t.TpItemId);
        if tp_item_id > 0 {
            let costs = [CostItem {
                id: tp_item_id,
                count: lv_cfg.Consume,
            }];
            let ok = consume::consume_costs(
                &mut state.conn,
                &state.fs,
                &mut state.player_components.basic,
                &mut state.player_components.inventory,
                &costs,
            )
            .map_err(|e| e.to_string())?;
            if !ok {
                return Ok(MotorTechLevelUpResponse {
                    ErrorCode: Some(ErrorCode::ErrItemNotEnough),
                    Tree: None,
                });
            }
        }
    }

    let node = &mut state.player_components.motor.info.tech_nodes[node_idx];
    node.level += 1;
    node.current = node.level;
    node.unlocked = true;
    state
        .player_components
        .motor
        .save(&state.fs)
        .map_err(|e| e.to_string())?;

    if let Some(scene) = state.scene.as_mut()
        && let Some(entity) = find_motor_entity(scene, &state.assets)
    {
        player_entity_templates::refresh_motorcycle_born_buffs(events, scene, &state.assets, entity);
    }

    let tree = build_motor_tech_tree_data(&state.player_components.motor, tree_type);
    let _ = state.conn.push_msg(
        MotorLockedTechUpdateNotify::MSG_ID,
        &MotorLockedTechUpdateNotify {
            TreeId: tree_type,
            Tech: tree.Tech.clone(),
        },
    );
    Ok(MotorTechLevelUpResponse {
        ErrorCode: Some(ErrorCode::Success),
        Tree: Some(tree),
    })
}

pub fn handle_motor_tech_tree_switch_request(
    state: &mut State,
    events: &mut EventQueue,
    req: MotorTechTreeSwitchRequest,
) -> Result<MotorTechTreeSwitchResponse, String> {
    state.player_components.motor.info.tree_in_use = req.TreeId;
    state
        .player_components
        .motor
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    if let Some(scene) = state.scene.as_mut()
        && let Some(entity) = find_motor_entity(scene, &state.assets)
    {
        player_entity_templates::refresh_motorcycle_born_buffs(events, scene, &state.assets, entity);
    }
    Ok(MotorTechTreeSwitchResponse {
        ErrorCode: Some(ErrorCode::Success),
        TreeInUse: state.player_components.motor.info.tree_in_use,
    })
}

pub fn handle_motor_level_one_key_reward_request(
    state: &mut State,
    _req: MotorLevelOneKeyRewardRequest,
) -> Result<MotorLevelOneKeyRewardResponse, String> {
    let start_lvl = state.player_components.motor.info.rewarded_max_level + 1;
    let max_lvl = state.player_components.motor.info.level;
    if start_lvl <= max_lvl {
        for lvl in start_lvl..=max_lvl {
            let reward_id = load_motor_level_reward_id(&state.assets, lvl);
            if reward_id > 0 {
                let mut rctx = state.reward_context();
                reward::grant_drop_package_popup(&mut rctx, reward_id, 8001)
                    .map_err(|e| e.to_string())?;
            }
        }
        state.player_components.motor.info.rewarded_max_level = max_lvl;
        state
            .player_components
            .motor
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(MotorLevelOneKeyRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
        MotorRewardedLvMax: state.player_components.motor.info.rewarded_max_level,
    })
}

pub fn handle_motor_task_one_key_reward_request(
    state: &mut State,
    req: MotorTaskOneKeyRewardRequest,
) -> Result<MotorTaskOneKeyRewardResponse, String> {
    let mut updated_tree: Option<i32> = None;
    for task_id in req.TaskIds {
        let Some(pos) = state
            .player_components
            .motor
            .info
            .task_nodes
            .iter()
            .position(|t| t.id == task_id)
        else {
            continue;
        };
        let (wait_reward, tree_type) = {
            let t = &mut state.player_components.motor.info.task_nodes[pos];
            let gain = t.wait_reward;
            if gain > 0 {
                t.rewarded += gain;
                t.wait_reward = 0;
            }
            (gain, t.tree_type)
        };
        let tp_item_id = if wait_reward > 0 {
            state
                .assets
                .tables
                .motor_tech_tree
                .get_data_by_id(&tree_type)
                .map_or(0, |t| t.TpItemId)
        } else {
            0
        };
        if wait_reward > 0 && tp_item_id > 0 {
            let mut rctx = state.reward_context();
            reward::grant_items(
                &mut rctx,
                0,
                true,
                &[RewardItem {
                    item_id: tp_item_id,
                    count: wait_reward,
                }],
                8001,
            )
            .map_err(|e| e.to_string())?;
        }
        updated_tree = Some(tree_type);
    }

    if let Some(tree_id) = updated_tree {
        state
            .player_components
            .motor
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        let tasks = build_motor_task_list_data(&state.player_components.motor, tree_id);
        let _ = state
            .conn
            .push_msg(MotorTaskUpdateNotify::MSG_ID, &MotorTaskUpdateNotify { Task: tasks });
    }
    Ok(MotorTaskOneKeyRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

fn apply_motor_outlook_update(
    state: &mut State,
    previous_frame: i32,
    skin_id: Option<i32>,
    frame: i32,
    stickers: Vec<i32>,
    decorations: Vec<i32>,
) -> Result<(), String> {
    let info = &mut state.player_components.motor.info;
    if let Some(sid) = skin_id {
        info.equipped_skin = sid;
    }
    info.equipped_frame = frame;
    info.equipped_stickers = stickers;
    info.equipped_decorations = decorations;
    state
        .player_components
        .motor
        .save(&state.fs)
        .map_err(|e| e.to_string())?;

    let equipped = build_equipped_outlook(&state.player_components.motor);
    let _ = state.conn.push_msg(
        MotorOutlookEquippedChangeNotify::MSG_ID,
        &MotorOutlookEquippedChangeNotify {
            MotorDiyEquipped: Some(equipped.clone()),
            LatestMotorSkinSuit: Vec::new(),
        },
    );
    if let Some(entity) = update_motor_outlook_entity(state) {
        if previous_frame != frame {
            push_entity_refresh_notify(state, entity, now_millis());
        } else {
            let _ = state.conn.push_msg(
                EntityMotorOutlookChangeNotify::MSG_ID,
                &EntityMotorOutlookChangeNotify {
                    EntityId: entity.net_id,
                    MotorDiyEquipped: Some(equipped),
                },
            );
        }
    }
    Ok(())
}

pub fn handle_motor_use_skin_request(
    state: &mut State,
    req: MotorUseSkinRequest,
) -> Result<MotorUseSkinResponse, String> {
    let previous_frame = state.player_components.motor.info.equipped_frame;
    let Some(skin) = state.assets.tables.motor_skin.get_data_by_id(&req.SkinId) else {
        return Ok(MotorUseSkinResponse {
            ErrorCode: Some(ErrorCode::MotorOutlookNotOwned),
        });
    };
    if !skin.DefaultFlag
        && !MotorInfo::has_owned_id(&state.player_components.motor.info.owned_skins, req.SkinId)
    {
        return Ok(MotorUseSkinResponse {
            ErrorCode: Some(ErrorCode::MotorOutlookNotOwned),
        });
    }
    let frame = match resolve_frame(&state.assets, skin.BindFrame, previous_frame) {
        Ok(f) => f,
        Err(code) => return Ok(MotorUseSkinResponse { ErrorCode: Some(code) }),
    };
    let stickers = match normalize_stickers(&state.assets, &skin.BindSticker, None) {
        Ok(s) => s,
        Err(code) => return Ok(MotorUseSkinResponse { ErrorCode: Some(code) }),
    };
    let decorations = match normalize_decorations(&state.assets, &skin.BindDecorations, None) {
        Ok(d) => d,
        Err(code) => return Ok(MotorUseSkinResponse { ErrorCode: Some(code) }),
    };

    apply_motor_outlook_update(
        state,
        previous_frame,
        Some(req.SkinId),
        frame,
        stickers,
        decorations,
    )?;
    Ok(MotorUseSkinResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_motor_change_outlook_request(
    state: &mut State,
    req: MotorChangeOutlookRequest,
) -> Result<MotorChangeOutlookResponse, String> {
    let previous_frame = state.player_components.motor.info.equipped_frame;
    let prev_stickers = state
        .player_components
        .motor
        .info
        .equipped_stickers
        .clone();
    let prev_decorations = state
        .player_components
        .motor
        .info
        .equipped_decorations
        .clone();
    let frame = match resolve_frame(&state.assets, req.FrameEquipped, previous_frame) {
        Ok(f) => f,
        Err(code) => return Ok(MotorChangeOutlookResponse { ErrorCode: Some(code) }),
    };
    let stickers = match normalize_stickers(&state.assets, &req.StickerEquipped, Some(&prev_stickers))
    {
        Ok(s) => s,
        Err(code) => return Ok(MotorChangeOutlookResponse { ErrorCode: Some(code) }),
    };
    let decorations =
        match normalize_decorations(&state.assets, &req.DecorationsEquipped, Some(&prev_decorations))
        {
            Ok(d) => d,
            Err(code) => return Ok(MotorChangeOutlookResponse { ErrorCode: Some(code) }),
        };

    if !check_motor_outlook_ownership(
        &state.player_components.motor,
        &state.assets,
        frame,
        &stickers,
        &decorations,
    ) {
        return Ok(MotorChangeOutlookResponse {
            ErrorCode: Some(ErrorCode::MotorOutlookNotOwned),
        });
    }

    apply_motor_outlook_update(
        state,
        previous_frame,
        None,
        frame,
        stickers,
        decorations,
    )?;
    Ok(MotorChangeOutlookResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_motor_outlook_create_preset_request(
    state: &mut State,
    req: MotorOutlookCreatePresetRequest,
) -> Result<MotorOutlookCreatePresetResponse, String> {
    let Some(equipped) = req.Preset else {
        return Ok(MotorOutlookCreatePresetResponse {
            ErrorCode: Some(ErrorCode::MotorOutlookPresetInputInvalid),
            MotorOutlookPreset: None,
        });
    };
    let motor = &mut state.player_components.motor;
    let id = motor.info.next_preset_id;
    motor.info.next_preset_id += 1;
    let preset = make_preset(id, req.name, &equipped, motor.info.equipped_skin);
    motor.info.presets.push(preset);
    motor.save(&state.fs).map_err(|e| e.to_string())?;
    Ok(MotorOutlookCreatePresetResponse {
        ErrorCode: Some(ErrorCode::Success),
        MotorOutlookPreset: Some(build_preset_list(motor)),
    })
}

pub fn handle_motor_outlook_delete_preset_request(
    state: &mut State,
    req: MotorOutlookDeletePresetRequest,
) -> Result<MotorOutlookDeletePresetResponse, String> {
    let motor = &mut state.player_components.motor;
    let Some(pos) = motor.info.presets.iter().position(|p| p.id == req.id) else {
        return Ok(MotorOutlookDeletePresetResponse {
            ErrorCode: Some(ErrorCode::MotorOutlookPresetNotExist),
            MotorOutlookPreset: None,
        });
    };
    motor.info.presets.remove(pos);
    motor.save(&state.fs).map_err(|e| e.to_string())?;
    Ok(MotorOutlookDeletePresetResponse {
        ErrorCode: Some(ErrorCode::Success),
        MotorOutlookPreset: Some(build_preset_list(motor)),
    })
}

pub fn handle_motor_outlook_edit_preset_request(
    state: &mut State,
    req: MotorOutlookEditPresetRequest,
) -> Result<MotorOutlookEditPresetResponse, String> {
    let Some(plan) = req.PresetPlan else {
        return Ok(MotorOutlookEditPresetResponse {
            ErrorCode: Some(ErrorCode::MotorOutlookPresetInputInvalid),
            MotorOutlookPreset: None,
        });
    };
    let Some(equipped) = plan.Preset else {
        return Ok(MotorOutlookEditPresetResponse {
            ErrorCode: Some(ErrorCode::MotorOutlookPresetInputInvalid),
            MotorOutlookPreset: None,
        });
    };
    let motor = &mut state.player_components.motor;
    let Some(preset) = motor.info.presets.iter_mut().find(|p| p.id == plan.Id) else {
        return Ok(MotorOutlookEditPresetResponse {
            ErrorCode: Some(ErrorCode::MotorOutlookPresetNotExist),
            MotorOutlookPreset: None,
        });
    };
    *preset = make_preset(plan.Id, plan.Mame, &equipped, motor.info.equipped_skin);
    motor.save(&state.fs).map_err(|e| e.to_string())?;
    Ok(MotorOutlookEditPresetResponse {
        ErrorCode: Some(ErrorCode::Success),
        MotorOutlookPreset: Some(build_preset_list(motor)),
    })
}

pub fn handle_motor_anim_replay_sample_udp_push(
    state: &mut State,
    _events: &mut EventQueue,
    msg: MotorAnimReplaySampleUdpPush,
) -> Result<(), String> {
    if msg.MotorAnimEntities.is_empty() {
        return Ok(());
    }
    if let Some(coop) = state.coop_service.as_ref() {
        let host_id = if msg.SceneOwnerId != 0 {
            msg.SceneOwnerId
        } else {
            state.player_id
        };
        coop.broadcast_room(
            host_id,
            state.player_id,
            MotorAnimReplaySampleUdpNotify::MSG_ID,
            &MotorAnimReplaySampleUdpNotify {
                MotorAnimEntities: msg.MotorAnimEntities,
            },
        );
    }
    Ok(())
}

pub fn handle_motor_cycle_cruise_state_push(
    state: &mut State,
    events: &mut EventQueue,
    msg: MotorCycleCruiseStatePush,
) -> Result<(), String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "motor:cruise",
        i64::from(msg.IsStart),
    )
    .map_err(|e| e.to_string())?;
    let mut qctx = state.quest_context(events);
    let _ = quest_engine::events::on_condition_state_changed(&mut qctx);
    Ok(())
}

pub fn handle_motor_is_enable_push(
    state: &mut State,
    events: &mut EventQueue,
    msg: MotorIsEnablePush,
) -> Result<(), String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "motor:enabled",
        i64::from(msg.IsEnable),
    )
    .map_err(|e| e.to_string())?;
    let mut qctx = state.quest_context(events);
    let _ = quest_engine::events::on_condition_state_changed(&mut qctx);
    Ok(())
}

pub fn handle_motor_online_progress_report_push(
    state: &mut State,
    _events: &mut EventQueue,
    msg: MotorOnlineProgressReportPush,
) -> Result<(), String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("motor:progress:{}", msg.InstId),
        i64::from(msg.Progress),
    )
    .map_err(|e| e.to_string())
}

pub fn handle_motor_position_state_push(
    state: &mut State,
    events: &mut EventQueue,
    msg: MotorPositionStatePush,
) -> Result<(), String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("motor:pos_state:{}", msg.EntityId),
        i64::from(msg.PositionState),
    )
    .map_err(|e| e.to_string())?;
    let mut qctx = state.quest_context(events);
    let _ = quest_engine::events::on_condition_state_changed(&mut qctx);
    Ok(())
}

pub fn handle_motor_summon_and_ride_push(
    state: &mut State,
    events: &mut EventQueue,
    msg: MotorSummonAndRidePush,
) -> Result<(), String> {
    state
        .player_components
        .progress
        .info
        .set_key_int("motor:summoned", 1);
    state
        .player_components
        .progress
        .info
        .set_key_int("motor:riding", 1);
    if let Some(scene) = state.scene.as_mut()
        && let Some(tr) = msg.Transform
        && let Some(p) = tr.Pos
        && p.X.is_finite()
        && p.Y.is_finite()
        && p.Z.is_finite()
    {
        let _ = scene.set_entity_position(
            msg.VehicleIncId,
            [p.X, p.Y, p.Z],
            tr.Rot.map(|r| r.Yaw),
        );
    }
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    let mut qctx = state.quest_context(events);
    let _ = quest_engine::events::on_condition_state_changed(&mut qctx);
    Ok(())
}

pub fn handle_motor_fight_refresh_boss_push(
    state: &mut State,
    events: &mut EventQueue,
    msg: MotorFightRefreshBossPush,
) -> Result<(), String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("motor:boss_refresh:{}:{}", msg.SubLevelIndex, msg.WaveGroupIndex),
        i64::from(msg.RefreshId),
    )
    .map_err(|e| e.to_string())?;
    let mut qctx = state.quest_context(events);
    let _ = quest_engine::events::on_condition_state_changed(&mut qctx);
    Ok(())
}

pub fn handle_motor_decal_motor_ip_request(
    state: &mut State,
    req: MotorDecalMotorIpRequest,
) -> Result<MotorDecalMotorIpResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "motor:decal_ip",
        i64::from(req.ActivityId),
    )
    .map_err(|e| e.to_string())?;
    Ok(MotorDecalMotorIpResponse {
        ErrorCode: Some(ErrorCode::Success),
        MotorIp: req.ActivityId,
    })
}

pub fn handle_motor_decal_task_request(
    state: &mut State,
    req: MotorDecalTaskRequest,
) -> Result<MotorDecalTaskResponse, String> {
    let mut ids = Vec::new();
    let mut dirty = false;
    let task_ids: Vec<i32> = state
        .assets
        .tables
        .motor_decal_quest
        .items
        .iter()
        .filter(|q| q.Ip == req.MotorIp)
        .map(|q| q.TaskId)
        .collect();
    for tid in task_ids {
        ids.push(tid);
        if state
            .player_components
            .progress
            .info
            .mark_claim(&format!("motor:decal_task:{tid}"))
        {
            dirty = true;
            let mut rctx = state.reward_context();
            reward::grant_table_reward(
                &mut rctx,
                "MotorDecalQuest",
                i64::from(tid),
                false,
                true,
                1,
                &[RewardItem {
                    item_id: 3,
                    count: 40,
                }],
                None,
            )
            .map_err(|e| e.to_string())?;
        }
    }
    if state
        .player_components
        .progress
        .info
        .mark_claim(&format!("motor:decal:{}", req.MotorIp))
    {
        dirty = true;
        let mut rctx = state.reward_context();
        reward::grant_table_reward(
            &mut rctx,
            "MotorDecalIp",
            i64::from(req.MotorIp),
            false,
            true,
            1,
            &[RewardItem {
                item_id: 3,
                count: 40,
            }],
            None,
        )
        .map_err(|e| e.to_string())?;
    }
    if dirty {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(MotorDecalTaskResponse {
        ErrorCode: Some(ErrorCode::Success),
        Ids: ids,
    })
}

pub fn handle_motor_develop_task_reward_request(
    state: &mut State,
    req: MotorDevelopTaskRewardRequest,
) -> Result<MotorDevelopTaskRewardResponse, String> {
    let mut dirty = false;
    for tid in req.Ids {
        if state
            .player_components
            .progress
            .info
            .mark_claim(&format!("motor:dev:{tid}"))
        {
            dirty = true;
            let mut rctx = state.reward_context();
            reward::grant_table_reward(
                &mut rctx,
                "MotorDevelopTask",
                i64::from(tid),
                false,
                true,
                8001,
                &[RewardItem {
                    item_id: 3,
                    count: 40,
                }],
                None,
            )
            .map_err(|e| e.to_string())?;
        }
    }
    if dirty {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(MotorDevelopTaskRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_motor_link_task_request(
    state: &mut State,
    req: MotorLinkTaskRequest,
) -> Result<MotorLinkTaskResponse, String> {
    let mut ids = Vec::new();
    let mut dirty = false;
    let task_ids: Vec<i32> = state
        .assets
        .tables
        .motor_linkage_quest
        .items
        .iter()
        .filter(|q| q.Ip == req.MotorIp)
        .map(|q| q.TaskId)
        .collect();
    for tid in task_ids {
        ids.push(tid);
        if state
            .player_components
            .progress
            .info
            .mark_claim(&format!("motor:link_task:{tid}"))
        {
            dirty = true;
            let mut rctx = state.reward_context();
            reward::grant_table_reward(
                &mut rctx,
                "MotorLinkageQuest",
                i64::from(tid),
                false,
                true,
                8001,
                &[RewardItem {
                    item_id: 3,
                    count: 40,
                }],
                None,
            )
            .map_err(|e| e.to_string())?;
        }
    }
    if state
        .player_components
        .progress
        .info
        .mark_claim(&format!("motor:link:{}", req.MotorIp))
    {
        dirty = true;
        let mut rctx = state.reward_context();
        reward::grant_items(
            &mut rctx,
            0,
            true,
            &[RewardItem {
                item_id: 3,
                count: 40,
            }],
            8001,
        )
        .map_err(|e| e.to_string())?;
    }
    if dirty {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(MotorLinkTaskResponse {
        ErrorCode: Some(ErrorCode::Success),
        Ids: ids,
    })
}

pub fn handle_motor_parkour_level_reward_request(
    state: &mut State,
    req: MotorParkourLevelRewardRequest,
) -> Result<MotorParkourLevelRewardResponse, String> {
    let mut dirty = false;
    for ridx in req.RewardIdxs {
        let key = format!("motor:pk:{}:{ridx}", req.MotorParkourId);
        if state.player_components.progress.info.mark_claim(&key) {
            dirty = true;
            let idx = i64::from(req.MotorParkourId) * 100 + i64::from(ridx);
            let reward_id = if state
                .assets
                .reward_index
                .get_row("MotorParkourReward", idx)
                .is_some()
            {
                idx
            } else {
                i64::from(ridx)
            };
            let mut rctx = state.reward_context();
            reward::grant_table_reward(
                &mut rctx,
                "MotorParkourReward",
                reward_id,
                false,
                true,
                8001,
                &[RewardItem {
                    item_id: 3,
                    count: 30,
                }],
                None,
            )
            .map_err(|e| e.to_string())?;
        }
    }
    if dirty {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(MotorParkourLevelRewardResponse {
        Error: Some(ErrorCode::Success),
    })
}

pub fn handle_motor_slider_request(
    state: &mut State,
    req: MotorSliderRequest,
) -> Result<MotorSliderResponse, String> {
    let itype = req.InteractType.map_or(0, |t| i64::from(t.to_i32()));
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("motor:slider:{}", req.EntityId),
        itype,
    )
    .map_err(|e| e.to_string())?;
    Ok(MotorSliderResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_motor_fight_enter_sub_level_request(
    state: &mut State,
    req: MotorFightEnterSubLevelRequest,
) -> Result<MotorFightEnterSubLevelResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "motor:fight_sub_lv",
        i64::from(req.SubLevelIndex),
    )
    .map_err(|e| e.to_string())?;
    Ok(MotorFightEnterSubLevelResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_motor_fight_kill_request(
    state: &mut State,
    req: MotorFightKillRequest,
) -> Result<MotorFightKillResponse, String> {
    if req.IsKilled {
        let info = &mut state.player_components.progress.info;
        info.add_key_int("motor_fight:kills", 1);
        info.add_key_int(
            "motor_fight:best_score",
            if req.IsBoss { 500 } else { 100 },
        );
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(MotorFightKillResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_motor_fight_last_save_data_request(
    state: &mut State,
    _req: MotorFightLastSaveDataRequest,
) -> Result<MotorFightLastSaveDataResponse, String> {
    let info = &state.player_components.progress.info;
    let sub_lv = info.get_key_int("motor:fight_sub_lv");
    let score = info.get_key_int("motor_fight:best_score");
    Ok(MotorFightLastSaveDataResponse {
        ErrCode: Some(ErrorCode::Success),
        Exists: sub_lv > 0 || score > 0,
        LastSaveData: Some(MotorFightLastSaveData {
            LevelId: sub_lv.max(0) as i32,
            Score: score.max(0) as i32,
            ..Default::default()
        }),
    })
}

pub fn handle_motor_fight_one_key_reward_request(
    state: &mut State,
    req: MotorFightOneKeyRewardRequest,
) -> Result<MotorFightOneKeyRewardResponse, String> {
    let mut dirty = false;
    for tid in req.TaskIds {
        if state
            .player_components
            .progress
            .info
            .mark_claim(&format!("motor:fight_task:{tid}"))
        {
            dirty = true;
            let mut rctx = state.reward_context();
            reward::grant_table_reward(
                &mut rctx,
                "MotorFightTask",
                i64::from(tid),
                false,
                true,
                8001,
                &[RewardItem {
                    item_id: 3,
                    count: 40,
                }],
                None,
            )
            .map_err(|e| e.to_string())?;
        }
    }
    if dirty {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(MotorFightOneKeyRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_motor_fight_player_dead_request(
    state: &mut State,
    _req: MotorFightPlayerDeadRequest,
) -> Result<MotorFightPlayerDeadResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int("motor:fight_deaths", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(MotorFightPlayerDeadResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

fn build_motor_fight_self_rank(state: &State) -> MotorFightPlayerRankingInfo {
    let pc = &state.player_components.progress.info;
    let kills = pc.get_key_int("motor_fight:kills").max(0) as i32;
    let score = pc.get_key_int("motor_fight:best_score").max(0) as i32;
    let sub_lv = pc.get_key_int("motor:fight_sub_lv").max(1) as i32;
    MotorFightPlayerRankingInfo {
        PlayerId: state.player_id,
        Name: state
            .player_components
            .basic
            .info
            .attributes
            .name
            .as_bytes()
            .to_vec(),
        ResultCommon: Some(MotorFightResultBase {
            LevelId: sub_lv,
            TotalScore: score,
            KillMonsterCount: kills,
            ..Default::default()
        }),
    }
}

pub fn handle_motor_fight_rank_list_request(
    state: &mut State,
    _req: MotorFightRankListRequest,
) -> Result<MotorFightRankListResponse, String> {
    let mut friend_data: Vec<MotorFightPlayerRankingInfo> =
        CoopService::collect_server_player_snapshots(
            &state.fs,
            state.coop_service.as_deref(),
            state.player_id,
        )
        .into_iter()
        .filter_map(|p| {
            let score = p.progress.get_key_int("motor_fight:best_score").max(0) as i32;
            let kills = p.progress.get_key_int("motor_fight:kills").max(0) as i32;
            if score <= 0 && kills <= 0 {
                return None;
            }
            let sub_lv = p.progress.get_key_int("motor:fight_sub_lv").max(1) as i32;
            Some(MotorFightPlayerRankingInfo {
                PlayerId: p.player_id,
                Name: p.name.into_bytes(),
                ResultCommon: Some(MotorFightResultBase {
                    LevelId: sub_lv,
                    TotalScore: score,
                    KillMonsterCount: kills,
                    ..Default::default()
                }),
            })
        })
        .collect();
    friend_data.sort_by(|a, b| {
        let sa = a.ResultCommon.as_ref().map_or(0, |r| r.TotalScore);
        let sb = b.ResultCommon.as_ref().map_or(0, |r| r.TotalScore);
        sb.cmp(&sa)
    });
    Ok(MotorFightRankListResponse {
        ErrorCode: Some(ErrorCode::Success),
        SelfData: Some(build_motor_fight_self_rank(state)),
        FriendData: friend_data,
    })
}

pub fn handle_motor_fight_rank_list_self_data_request(
    state: &mut State,
    _req: MotorFightRankListSelfDataRequest,
) -> Result<MotorFightRankListSelfDataResponse, String> {
    Ok(MotorFightRankListSelfDataResponse {
        ErrorCode: Some(ErrorCode::Success),
        SelfData: Some(build_motor_fight_self_rank(state)),
    })
}

pub fn handle_motor_fight_refresh_collection_request(
    state: &mut State,
    _req: MotorFightRefreshCollectionRequest,
) -> Result<MotorFightRefreshCollectionResponse, String> {
    let cnt = state
        .player_components
        .progress
        .info
        .add_key_int("motor:fight_col_refresh", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    let drops = state
        .assets
        .reward_index
        .get_rows("MotorFightCollection")
        .iter()
        .take(3)
        .map(|row| MotorFightCollectionDrop {
            CollectionId: row.key as i32,
            ..Default::default()
        })
        .collect();
    Ok(MotorFightRefreshCollectionResponse {
        ErrCode: Some(ErrorCode::Success),
        RefreshCollectionTimes: cnt as i32,
        Drops: drops,
    })
}

pub fn handle_motor_fight_refresh_next_wave_group_request(
    state: &mut State,
    _req: MotorFightRefreshNextWaveGroupRequest,
) -> Result<MotorFightRefreshNextWaveGroupResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int("motor:fight_wave", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(MotorFightRefreshNextWaveGroupResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_motor_fight_select_buff_gate_request(
    state: &mut State,
    req: MotorFightSelectBuffGateRequest,
) -> Result<MotorFightSelectBuffGateResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "motor:fight_buff_gate",
        i64::from(req.BuffGateId),
    )
    .map_err(|e| e.to_string())?;
    Ok(MotorFightSelectBuffGateResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_motor_fight_select_collection_request(
    state: &mut State,
    _req: MotorFightSelectCollectionRequest,
) -> Result<MotorFightSelectCollectionResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int("motor:fight_col_cnt", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(MotorFightSelectCollectionResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_motor_fight_settle_request(
    state: &mut State,
    _req: MotorFightSettleRequest,
) -> Result<MotorFightSettleResponse, String> {
    let pc = &mut state.player_components.progress;
    let sub_lv = pc.info.get_key_int("motor:fight_sub_lv").max(1) as i32;
    let kills = pc.info.get_key_int("motor_fight:kills").max(0) as i32;
    let score = pc.info.get_key_int("motor_fight:best_score");
    let is_first_clear = pc.info.mark_claim(&format!("motor_fight:clear:{sub_lv}"));
    let best_key = format!("motor_fight:best:{sub_lv}");
    let prev_best = pc.info.get_key_int(&best_key);
    let is_best = score > prev_best;
    if is_best {
        pc.info.set_key_int(&best_key, score);
    }
    pc.info.add_key_int("motor_fight:pass", 1);
    pc.save(&state.fs).map_err(|e| e.to_string())?;

    if is_first_clear {
        let main_lv = if sub_lv >= 100_000 { sub_lv / 100 } else { sub_lv };
        let mut rctx = state.reward_context();
        reward::grant_table_reward(
            &mut rctx,
            "MotorFightLevel",
            i64::from(main_lv),
            false,
            true,
            1,
            &[
                RewardItem {
                    item_id: 3,
                    count: 40,
                },
                RewardItem {
                    item_id: 2,
                    count: 10000,
                },
            ],
            None,
        )
        .map_err(|e| e.to_string())?;
    }

    Ok(MotorFightSettleResponse {
        ErrCode: Some(ErrorCode::Success),
        Win: true,
        Result: Some(MotorFightResult {
            IsFirstClear: is_first_clear,
            IsBestScore: is_best,
            ResultCommon: Some(MotorFightResultBase {
                LevelId: sub_lv,
                TotalScore: score.max(0) as i32,
                KillMonsterCount: kills,
                ..Default::default()
            }),
            ..Default::default()
        }),
    })
}

pub fn handle_motor_fight_talent_up_request(
    state: &mut State,
    req: MotorFightTalentUpRequest,
) -> Result<MotorFightTalentUpResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int(&format!("motor:fight_tal:{}", req.TalentId), 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(MotorFightTalentUpResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}
