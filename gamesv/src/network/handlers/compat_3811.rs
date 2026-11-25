use sonoro_gamesv_fs::quest_info::QuestInfo;
use sonoro_gamesv_logic::component::player::PlayerCosmeticComponent;
use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::fsm::types::wake_reason;
use sonoro_gamesv_logic::helpers::{attributes, energy, quest_actions, quest_engine};
use sonoro_gamesv_logic::sink::PushSink;
use sonoro_proto::PbEnum;
use sonoro_proto::enums::{DetectionType, EAttributeType, ErrorCode, IllustratedType};
use sonoro_proto::pb::{
    AceAntiDataPush, CombatReceivePackNotify, DetectionTargetData, EntityPatrolStartRequest,
    EntityPatrolStartResponse, FinishDownloadQuestResourceRequest,
    FinishDownloadQuestResourceResponse, FlowSeqSetPlayerPosRequest, FlowSeqSetPlayerPosResponse,
    GameStatsShowRequest, GameStatsShowResponse, GetDetectionTargetRequest,
    GetDetectionTargetResponse, GoogleGameStatsShow, IllustratedEntry, IllustratedUnlockRequest,
    IllustratedUnlockResponse, InstLevelPlayStateRsp, InvalidItemRequest, InvalidItemResponse,
    LandingDamageRequest, LandingDamageResponse, LevelPlayStateListRequest,
    LevelPlayStateListResponse, MapOpenPush, OutputDebugInfoRequest, OutputDebugInfoResponse,
    SceneBlockChangePush, SceneBlockSplitClientLoginPush, SceneBlockSplitSwitchStatePush,
    SetQuestResourceStateRequest, SetQuestResourceStateResponse, ShowInvalidItemRequest,
    ShowInvalidItemResponse, ThirdPartyShareRequest, ThirdPartyShareResponse,
};

use super::State;

pub fn handle_flow_seq_set_player_pos_request(
    state: &mut State,
    events: &mut EventQueue,
    req: FlowSeqSetPlayerPosRequest,
) -> Result<FlowSeqSetPlayerPosResponse, String> {
    let mut qctx = state.quest_context(events);
    quest_actions::update_player_scene_state(
        &mut qctx,
        [req.X, req.Y, req.Z],
        Some([0.0, 0.0, req.A]),
    )
    .map_err(|e| e.to_string())?;
    Ok(FlowSeqSetPlayerPosResponse {
        Code: Some(ErrorCode::Success),
    })
}

pub fn handle_finish_download_quest_resource_request(
    state: &mut State,
    events: &mut EventQueue,
    req: FinishDownloadQuestResourceRequest,
) -> Result<FinishDownloadQuestResourceResponse, String> {
    let mut qctx = state.quest_context(events);
    quest_engine::sync_quest_resource_availability(&mut qctx, &req.QuestIds, &[], true)
        .map_err(|e| e.to_string())?;
    Ok(FinishDownloadQuestResourceResponse {
        ErrorId: Some(ErrorCode::Success),
    })
}

pub fn handle_set_quest_resource_state_request(
    state: &mut State,
    req: SetQuestResourceStateRequest,
) -> Result<SetQuestResourceStateResponse, String> {
    state
        .player_components
        .progress
        .info
        .set_key_int("quest:res_state", i64::from(req.QuestResourceState));
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    tracing::info!(
        "player({}) reported QuestResourceState={}",
        state.player_id,
        req.QuestResourceState
    );
    Ok(SetQuestResourceStateResponse {
        ErrorId: Some(ErrorCode::Success),
    })
}

pub fn handle_get_detection_target_request(
    state: &mut State,
    req: GetDetectionTargetRequest,
) -> Result<GetDetectionTargetResponse, String> {
    let mut tids = Vec::new();
    let req_type = req.Type.unwrap_or(DetectionType::NormalMonster);
    let det_id = req.DetectionId;

    match req_type {
        DetectionType::NormalMonster | DetectionType::Material => {
            if det_id > 0 {
                if let Some(md) = state.assets.tables.monster_detection.get_data_by_id(&det_id) {
                    if md.EntityConfigId != 0 {
                        tids.push(md.EntityConfigId as i32);
                    } else if !md.BlueprintType.is_empty() {
                        if let Some(db) = state.assets.tables.assets_db() {
                            let found = db
                                .query_entity_ids_by_map_blueprint(8, &md.BlueprintType)
                                .unwrap_or_default();
                            tids.extend(found.into_iter().take(32));
                        } else {
                            for lcfg in &state.assets.tables.level_entity_config.items {
                                if lcfg.MapId == 8 && lcfg.BlueprintType == md.BlueprintType {
                                    tids.push(lcfg.EntityId as i32);
                                    if tids.len() >= 32 {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                for md in &state.assets.tables.monster_detection.items {
                    if md.EntityConfigId != 0 {
                        tids.push(md.EntityConfigId as i32);
                        if tids.len() >= 16 {
                            break;
                        }
                    }
                }
            }
        }
        DetectionType::Dungeon => {
            if det_id > 0 {
                if let Some(dd) = state.assets.tables.dungeon_detection.get_data_by_id(&det_id) {
                    let entrance_id = if dd.InstanceDungeonEntranceId != 0 {
                        dd.InstanceDungeonEntranceId
                    } else {
                        dd.DungeonId
                    };
                    if entrance_id != 0 {
                        tids.push(entrance_id);
                    }
                }
            } else {
                for dd in &state.assets.tables.dungeon_detection.items {
                    let entrance_id = if dd.InstanceDungeonEntranceId != 0 {
                        dd.InstanceDungeonEntranceId
                    } else {
                        dd.DungeonId
                    };
                    if entrance_id != 0 {
                        tids.push(entrance_id);
                        if tids.len() >= 16 {
                            break;
                        }
                    }
                }
            }
        }
        DetectionType::SilentArea => {
            if det_id > 0 {
                if let Some(sd) = state
                    .assets
                    .tables
                    .silent_area_detection
                    .get_data_by_id(&det_id)
                {
                    tids.extend_from_slice(&sd.LevelPlayList);
                    if tids.is_empty() && sd.DungeonId != 0 {
                        tids.push(sd.DungeonId);
                    }
                }
            } else {
                for sd in &state.assets.tables.silent_area_detection.items {
                    if let Some(&lp_id) = sd.LevelPlayList.first() {
                        tids.push(lp_id);
                    }
                    if tids.len() >= 16 {
                        break;
                    }
                }
            }
        }
    }

    if tids.is_empty() && det_id > 0 {
        if let Some(md) = state.assets.tables.monster_detection.get_data_by_id(&det_id) {
            if md.EntityConfigId != 0 {
                tids.push(md.EntityConfigId as i32);
            }
        } else if let Some(sd) = state
            .assets
            .tables
            .silent_area_detection
            .get_data_by_id(&det_id)
        {
            tids.extend_from_slice(&sd.LevelPlayList);
        } else if let Some(dd) = state.assets.tables.dungeon_detection.get_data_by_id(&det_id) {
            let entrance_id = if dd.InstanceDungeonEntranceId != 0 {
                dd.InstanceDungeonEntranceId
            } else {
                dd.DungeonId
            };
            if entrance_id != 0 {
                tids.push(entrance_id);
            }
        }
    }

    Ok(GetDetectionTargetResponse {
        Type: req.Type,
        Code: Some(ErrorCode::Success),
        DetectionId: req.DetectionId,
        DetectionTargetDatas: vec![DetectionTargetData {
            MapId: 8,
            TargetIds: tids,
        }],
    })
}

pub fn handle_output_debug_info_request(
    _state: &mut State,
    _req: OutputDebugInfoRequest,
) -> Result<OutputDebugInfoResponse, String> {
    Ok(OutputDebugInfoResponse {})
}

pub fn handle_invalid_item_request(
    _state: &mut State,
    _req: InvalidItemRequest,
) -> Result<InvalidItemResponse, String> {
    Ok(InvalidItemResponse {
        HasInvalidItem: false,
        HasExpireConvertItem: false,
    })
}

pub fn handle_show_invalid_item_request(
    _state: &mut State,
    _req: ShowInvalidItemRequest,
) -> Result<ShowInvalidItemResponse, String> {
    Ok(ShowInvalidItemResponse {
        InvalidItemList: Vec::new(),
        ExpireConvertItemInfoList: Vec::new(),
    })
}

pub fn handle_game_stats_show_request(
    _state: &mut State,
    _req: GameStatsShowRequest,
) -> Result<GameStatsShowResponse, String> {
    Ok(GameStatsShowResponse {
        GoogleGameStatsShow: Some(GoogleGameStatsShow::default()),
    })
}

pub fn handle_third_party_share_request(
    _state: &mut State,
    _req: ThirdPartyShareRequest,
) -> Result<ThirdPartyShareResponse, String> {
    Ok(ThirdPartyShareResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

fn is_level_play_hidden(info: &QuestInfo, lpid: i32) -> bool {
    info.world
        .hidden_groups
        .iter()
        .any(|grp| grp.level_plays.contains(&lpid))
}

pub fn handle_level_play_state_list_request(
    state: &mut State,
    req: LevelPlayStateListRequest,
) -> Result<LevelPlayStateListResponse, String> {
    let quest_info = &state.player_components.quest.info;
    let rsps = req
        .InstLevelPlayStateReqs
        .iter()
        .map(|entry| {
            let mut rsp = InstLevelPlayStateRsp {
                InstId: entry.InstId,
                ..Default::default()
            };
            for &lpid in &entry.LevelPlayIds {
                let done = quest_info.completed_level_plays.contains(&lpid);
                let hidden = is_level_play_hidden(quest_info, lpid);
                let state_val = i32::from(done);
                rsp.LevelPlayStateDict.insert(lpid, state_val);
                rsp.CompleteNumberDict.insert(lpid, state_val);
                rsp.HideDict.insert(lpid, hidden);
                rsp.IsUnlockDict.insert(lpid, !hidden);
            }
            rsp
        })
        .collect();

    Ok(LevelPlayStateListResponse {
        InstLevelPlayStateRsps: rsps,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_illustrated_unlock_request(
    state: &mut State,
    req: IllustratedUnlockRequest,
) -> Result<IllustratedUnlockResponse, String> {
    let entry_type = req.Type.unwrap_or(IllustratedType::Monster);
    let now_sec = energy::now_seconds();
    let create_time = now_sec.clamp(1, i64::from(u32::MAX)) as u32;
    let rec = state.player_components.cosmetic.info.unlock_illustrated(
        entry_type.to_i32(),
        req.Id,
        create_time,
    );
    PlayerCosmeticComponent::save(
        &state.fs,
        state.player_id,
        &state.player_components.cosmetic.info,
    )
    .map_err(|e| e.to_string())?;

    Ok(IllustratedUnlockResponse {
        Entry: Some(IllustratedEntry {
            SubType: None,
            Id: rec.id,
            CreateTime: rec.create_time,
            Num: rec.num,
            IsRead: rec.is_read,
        }),
        ErrorCode: Some(ErrorCode::Success),
        ErrorParams: Vec::new(),
    })
}

pub fn handle_entity_patrol_start_request(
    state: &mut State,
    req: EntityPatrolStartRequest,
) -> Result<EntityPatrolStartResponse, String> {
    let dir_val = i64::from(req.Dir);
    state
        .player_components
        .progress
        .info
        .set_key_int(&format!("patrol:dir:{}", req.EntityId), dir_val);
    if let Some(scene) = state.scene.as_mut() {
        if let Some(cfg_id) = scene.find_config_id_by_net_id(req.EntityId) {
            state
                .player_components
                .progress
                .info
                .set_key_int(&format!("patrol:dir:{cfg_id}"), dir_val);
        }
        if let Some(storage) = scene.entity_mut(req.EntityId) {
            storage.world_item.patrol_dir = Some(req.Dir);
        }
    }
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(EntityPatrolStartResponse {
        Code: Some(ErrorCode::Success),
    })
}

pub fn handle_landing_damage_request(
    state: &mut State,
    events: &mut EventQueue,
    req: LandingDamageRequest,
) -> Result<LandingDamageResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int("combat:fall_dmg_cnt", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;

    let speed = req.SpeedZ.abs();
    if (speed > 1200.0 || req.TimeExceeding > 0)
        && let Some(scene) = state.scene.as_mut()
    {
        let target_nid = if req.EntityId > 0 {
            req.EntityId
        } else {
            scene.current_role_entity_id()
        };
        if let Some(storage) = scene.entity_mut(target_nid)
            && let Some(attr) = storage.attribute.as_mut()
        {
            let life_idx = EAttributeType::Life.to_i32() as usize;
            let max_idx = EAttributeType::LifeMax.to_i32() as usize;
            let cur_hp = attr.attributes[life_idx].current;
            let max_hp = attr.attributes[max_idx].current;
            if cur_hp > 0 && max_hp > 0 {
                let raw_ratio = if speed > 1200.0 {
                    (speed - 1200.0) / 2500.0
                } else {
                    (req.TimeExceeding.min(10) as f32) * 0.08
                };
                let clamped_ratio = raw_ratio.clamp(0.05, 0.90);
                let dmg = ((max_hp as f32) * clamped_ratio).trunc().max(1.0) as i32;
                let next_hp = (cur_hp - dmg).max(1);
                let _ = attributes::set_life_unconditional(attr, next_hp);
                let mut cpack = Vec::new();
                attributes::generate_attr_messages(
                    &mut cpack,
                    target_nid,
                    attr,
                    &[EAttributeType::Life],
                );
                if !cpack.is_empty() {
                    state.conn.push(
                        CombatReceivePackNotify::MSG_ID,
                        &CombatReceivePackNotify { Data: cpack },
                    );
                }
                scene.mark_fsm_dirty(target_nid, wake_reason::ATTRIBUTE);
            }
        }
    }

    {
        let mut qctx = state.quest_context(events);
        quest_engine::on_condition_state_changed(&mut qctx).map_err(|e| e.to_string())?;
    }

    Ok(LandingDamageResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_scene_block_change_push(
    state: &mut State,
    _events: &mut EventQueue,
    push: SceneBlockChangePush,
) -> Result<(), String> {
    let mut dirty = false;
    for bid in push.DownloadBlockId {
        state
            .player_components
            .progress
            .info
            .set_key_int(&format!("scene:block:{bid}"), 1);
        dirty = true;
    }
    for bid in push.DeleteBlockId {
        state
            .player_components
            .progress
            .info
            .set_key_int(&format!("scene:block:{bid}"), 0);
        dirty = true;
    }
    if dirty {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub fn handle_map_open_push(
    state: &mut State,
    events: &mut EventQueue,
    _push: MapOpenPush,
) -> Result<(), String> {
    state
        .player_components
        .progress
        .info
        .add_key_int("ui:map_open_cnt", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    let mut qctx = state.quest_context(events);
    let _ = quest_engine::on_condition_state_changed(&mut qctx);
    Ok(())
}

pub fn handle_ace_anti_data_push(
    _state: &mut State,
    _events: &mut EventQueue,
    _push: AceAntiDataPush,
) -> Result<(), String> {
    Ok(())
}

pub fn handle_scene_block_split_client_login_push(
    _state: &mut State,
    _events: &mut EventQueue,
    _push: SceneBlockSplitClientLoginPush,
) -> Result<(), String> {
    Ok(())
}

pub fn handle_scene_block_split_switch_state_push(
    state: &mut State,
    _events: &mut EventQueue,
    push: SceneBlockSplitSwitchStatePush,
) -> Result<(), String> {
    if let Some(st) = push.SwitchState {
        state
            .player_components
            .progress
            .info
            .set_key_int("quest:block_state", i64::from(st.to_i32()));
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        tracing::info!(
            "player({}) reported SceneBlockSwitchState={:?}",
            state.player_id,
            st
        );
    }
    Ok(())
}
