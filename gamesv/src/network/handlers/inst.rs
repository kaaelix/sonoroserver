use std::collections::BTreeMap;

use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::events::Event;
use sonoro_gamesv_logic::helpers::{quest_actions, quest_engine, reward, role as role_helper};
use sonoro_proto::enums::{ErrorCode, TeleportReason};
use sonoro_proto::pb::{
    ApplyRechallengeRequest, ApplyRechallengeResponse, ChangeDataLayerFinishRequest,
    ChangeDataLayerFinishResponse, EnterBigWorldInstRequest, EnterBigWorldInstResponse,
    EnterInstRequest, EnterInstResponse, ExchangeRewardInfoNotify, ExchangeRewardInfoRequest,
    ExchangeRewardInfoResponse, FlowRestartRequest, FlowRestartResponse,
    GetLevelPlayRewardsRequest, GetLevelPlayRewardsResponse, InstFightEndPush, LeaveInstRequest,
    LeaveInstResponse, LeaveSceneNotify, LevelPlayFastReturnPosRequest,
    LevelPlayFastReturnPosResponse, LevelPlayInfo, LevelPlayInfoNotify, LevelPlayOpenRequest,
    LevelPlayOpenResponse, LevelPlayRewardRequest, LevelPlayRewardResponse,
    LevelPlaySwitchSportModeRequest, LevelPlaySwitchSportModeResponse, SceneSubLevelsRequest,
    SceneSubLevelsResponse, SingleInstRechallengeRequest, SingleInstRechallengeResponse,
    TeleportDungeonActionRequest, TeleportDungeonActionResponse, TeleportFlowEndRequest,
    TeleportFlowEndResponse, TeleportNotify, TransitionOptionPb, Vector,
};

use super::State;

pub fn handle_enter_inst_request(
    state: &mut State,
    events: &mut EventQueue,
    req: EnterInstRequest,
) -> Result<EnterInstResponse, String> {
    if state
        .assets
        .tables
        .instance_dungeon
        .get_data_by_id(&req.InstanceId)
        .is_none()
    {
        return Ok(EnterInstResponse {
            ErrorCode: Some(ErrorCode::ErrInstanceConfigNotFound),
        });
    }
    let mut qctx = state.quest_context(events);
    quest_engine::enter_inst(&mut qctx, req.InstanceId, req.PosEntityId)
        .map_err(|e| e.to_string())?;
    Ok(EnterInstResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_enter_big_world_inst_request(
    state: &mut State,
    events: &mut EventQueue,
    _req: EnterBigWorldInstRequest,
) -> Result<EnterBigWorldInstResponse, String> {
    if let Some(instance_id) = state.scene.as_ref().map(|s| s.instance_id)
        && let Some(inst) = state.assets.tables.instance_dungeon.get_data_by_id(&instance_id)
        && inst.InstType != 1
    {
        let mut qctx = state.quest_context(events);
        if let Err(err) = quest_engine::on_player_transfer_or_leave_inst(&mut qctx, true) {
            tracing::warn!("onEnterBigWorldInst hook error: {err}");
        }
        quest_engine::leave_inst(&mut qctx, 0).map_err(|e| e.to_string())?;
    }
    Ok(EnterBigWorldInstResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_leave_inst_request(
    state: &mut State,
    events: &mut EventQueue,
    req: LeaveInstRequest,
) -> Result<LeaveInstResponse, String> {
    let mut qctx = state.quest_context(events);
    if let Err(err) = quest_engine::on_player_transfer_or_leave_inst(&mut qctx, true) {
        tracing::warn!("onLeaveInst hook error: {err}");
    }
    quest_engine::leave_inst(&mut qctx, req.PosEntityId).map_err(|e| e.to_string())?;
    Ok(LeaveInstResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_teleport_dungeon_action_request(
    state: &mut State,
    events: &mut EventQueue,
    req: TeleportDungeonActionRequest,
) -> Result<TeleportDungeonActionResponse, String> {
    let key = format!("inst:tp_action:{}", req.IncId);
    let pc = &mut state.player_components.progress;
    pc.info.set_key_int(&key, i64::from(req.IsConfirm));
    pc.save(&state.fs).map_err(|e| e.to_string())?;
    if req.IsConfirm && state.scene.is_some() {
        let mut qctx = state.quest_context(events);
        if let Err(err) = quest_engine::on_scene_loading_finish(&mut qctx) {
            tracing::warn!("onSceneLoadingFinish hook error: {err}");
        }
    }
    Ok(TeleportDungeonActionResponse {
        Code: Some(ErrorCode::Success),
    })
}

pub fn handle_teleport_flow_end_request(
    state: &mut State,
    events: &mut EventQueue,
    _req: TeleportFlowEndRequest,
) -> Result<TeleportFlowEndResponse, String> {
    if state.scene.is_some() {
        let mut qctx = state.quest_context(events);
        if let Err(err) = quest_engine::on_scene_loading_finish(&mut qctx) {
            tracing::warn!("onSceneLoadingFinish hook error: {err}");
        }
    }
    Ok(TeleportFlowEndResponse {
        Code: Some(ErrorCode::Success),
    })
}

pub fn handle_scene_sub_levels_request(
    state: &mut State,
    events: &mut EventQueue,
    req: SceneSubLevelsRequest,
) -> Result<SceneSubLevelsResponse, String> {
    let load: Vec<String> = req
        .ShowSubLevels
        .into_iter()
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .collect();
    let hide: Vec<String> = req
        .HideSubLevels
        .into_iter()
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .collect();
    let mut qctx = state.quest_context(events);
    quest_engine::change_sub_levels(&mut qctx, &load, &[], &hide, 0).map_err(|e| e.to_string())?;
    Ok(SceneSubLevelsResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_change_data_layer_finish_request(
    state: &mut State,
    events: &mut EventQueue,
    req: ChangeDataLayerFinishRequest,
) -> Result<ChangeDataLayerFinishResponse, String> {
    let key = format!("inst:datalayer:{}", req.InstId);
    let pc = &mut state.player_components.progress;
    pc.info.set_key_int(&key, 1);
    pc.save(&state.fs).map_err(|e| e.to_string())?;
    if state.scene.is_some() {
        let mut qctx = state.quest_context(events);
        if let Err(err) = quest_engine::on_condition_state_changed(&mut qctx) {
            tracing::warn!("onChangeDataLayerFinish hook error: {err}");
        }
    }
    Ok(ChangeDataLayerFinishResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_flow_restart_request(
    state: &mut State,
    events: &mut EventQueue,
    req: FlowRestartRequest,
) -> Result<FlowRestartResponse, String> {
    let mut qctx = state.quest_context(events);
    quest_engine::restart_flow(&mut qctx, req.FlowIncId);
    Ok(FlowRestartResponse {
        Code: Some(ErrorCode::Success),
    })
}

pub fn handle_level_play_open_request(
    state: &mut State,
    req: LevelPlayOpenRequest,
) -> Result<LevelPlayOpenResponse, String> {
    if let Some(lp) = state
        .assets
        .tables
        .level_play_data
        .get_data_by_id(&req.LevelPlayId)
    {
        let eval_ctx = state.eval_context();
        if !sonoro_gamesv_logic::handlers::misc::is_level_play_open_globally(&eval_ctx, &lp) {
            return Ok(LevelPlayOpenResponse {
                ErrorCode: Some(ErrorCode::ErrPreCondition),
                LevelPlayId: req.LevelPlayId,
            });
        }
    }
    let key = format!("lp:open:{}", req.LevelPlayId);
    let pc = &mut state.player_components.progress;
    let _ = pc.info.mark_claim(&key);
    pc.save(&state.fs).map_err(|e| e.to_string())?;
    Ok(LevelPlayOpenResponse {
        ErrorCode: Some(ErrorCode::Success),
        LevelPlayId: req.LevelPlayId,
    })
}

pub fn handle_level_play_reward_request(
    state: &mut State,
    req: LevelPlayRewardRequest,
) -> Result<LevelPlayRewardResponse, String> {
    let cnt = req.ExchangeCount.max(1);
    let mut resolved_cfg_id = req.EntityId;
    if let Some(scene) = state.scene.as_ref()
        && let Some(cid) = scene.find_config_id_by_net_id(req.EntityId)
    {
        resolved_cfg_id = i64::from(cid);
    }

    let mut matched_lp_id = 0i32;
    let mut reward_id = 0i32;
    let mut first_reward_id = 0i32;
    if let Ok(entity_id32) = i32::try_from(resolved_cfg_id) {
        let reward_indices = state
            .assets
            .indexes
            .level_play_indices_for_reward_entity(entity_id32);
        let lp_indices = if reward_indices.is_empty() {
            state
                .assets
                .indexes
                .level_play_indices_for_entity(entity_id32)
        } else {
            reward_indices
        };
        if let Some(&idx) = lp_indices.first() {
            let lp = &state.assets.tables.level_play_data.items[idx as usize];
            matched_lp_id = lp.LevelPlayId;
            if let Some(rc) = lp.Data.LevelPlayRewardConfig.as_ref() {
                reward_id = rc.RewardId;
                first_reward_id = rc.FirstRewardId;
            }
        }
    }

    let key = format!("lp:reward_cnt:{resolved_cfg_id}");
    let total_reward_cnt = state
        .player_components
        .progress
        .info
        .add_key_int(&key, i64::from(cnt))
        .clamp(0, i64::from(i32::MAX)) as i32;
    let claim_first = matched_lp_id > 0
        && state
            .player_components
            .progress
            .info
            .get_key_int(&format!("lp_reward_claimed:{matched_lp_id}"))
            == 0;
    if matched_lp_id > 0 {
        state
            .player_components
            .progress
            .info
            .set_key_int(&format!("lp_reward_claimed:{matched_lp_id}"), 1);
    }
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;

    let mut granted_any = false;
    if claim_first && first_reward_id > 0 {
        let mut rctx = state.reward_context();
        let res = reward::grant_treasure_box_reward(&mut rctx, 0, first_reward_id, 15_008)
            .map_err(|e| e.to_string())?;
        if res.granted_count > 0 {
            granted_any = true;
        }
    }
    if reward_id > 0 {
        for _ in 0..cnt {
            let mut rctx = state.reward_context();
            let res = reward::grant_treasure_box_reward(&mut rctx, 0, reward_id, 15_008)
                .map_err(|e| e.to_string())?;
            if res.granted_count > 0 {
                granted_any = true;
            }
        }
    }
    if !granted_any {
        let mut rctx = state.reward_context();
        let _ = reward::grant_items(
            &mut rctx,
            0,
            true,
            &[
                reward::RewardItem {
                    item_id: 3,
                    count: 20 * cnt,
                },
                reward::RewardItem {
                    item_id: 2,
                    count: 5000 * cnt,
                },
                reward::RewardItem {
                    item_id: 1,
                    count: 100 * cnt,
                },
            ],
            18500,
        )
        .map_err(|e| e.to_string())?;
    }

    if let Ok(cfg_id32) = i32::try_from(resolved_cfg_id)
        && cfg_id32 > 0
    {
        state.player_components.quest.info.mark_chest_opened(cfg_id32);
        state
            .player_components
            .quest
            .info
            .set_entity_state(resolved_cfg_id, "关卡.Common.状态.激活");
        state
            .player_components
            .quest
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        if let Some(scene) = state.scene.as_mut()
            && let Some(net_id) = scene.find_entity_net_id_by_config_id(cfg_id32)
            && let Some(upd) = scene.update_entity_state_tag(
                &state.assets.tables.gameplay_tag_remap,
                net_id,
                "关卡.Common.状态.激活",
            )
        {
            quest_actions::emit_entity_state_tag_update_checked(
                scene,
                Some(&state.assets),
                &mut state.conn,
                net_id,
                upd,
            );
        }
    }

    if matched_lp_id > 0 {
        let _ = state.conn.push_msg(
            LevelPlayInfoNotify::MSG_ID,
            &LevelPlayInfoNotify {
                LevelPlayInfo: vec![LevelPlayInfo {
                    Id: matched_lp_id,
                    IsFirst: false,
                    State: 3,
                    UpdateTime: 0,
                    GetRewardCount: total_reward_cnt,
                }],
            },
        );
    }
    if reward_id > 0 {
        let _ = state.conn.push_msg(
            ExchangeRewardInfoNotify::MSG_ID,
            &ExchangeRewardInfoNotify {
                ExchangeRewardId: reward_id,
                Count: total_reward_cnt,
            },
        );
    }

    Ok(LevelPlayRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_exchange_reward_info_request(
    state: &mut State,
    _req: ExchangeRewardInfoRequest,
) -> Result<ExchangeRewardInfoResponse, String> {
    let mut exchange_dict = BTreeMap::new();
    let mut share_counts: BTreeMap<i32, i32> = BTreeMap::new();
    for c in &state.player_components.progress.info.dynamic.claims {
        if let Some(rest) = c.strip_prefix("exchange:") {
            if let Ok(cid) = rest.parse::<i32>() {
                exchange_dict.insert(cid, 1);
                if let Some(cfg) = state.assets.tables.exchange_reward.get_data_by_id(&cid)
                    && cfg.SharedId > 0
                {
                    *share_counts.entry(cfg.SharedId).or_insert(0) += 1;
                }
            }
        } else if let Some(rest) = c.strip_prefix("exchange_share:")
            && let Ok(sid) = rest.parse::<i32>()
        {
            *share_counts.entry(sid).or_insert(0) += 1;
        }
    }
    for entry in &state.player_components.progress.info.dynamic.ints {
        if let Some(id_str) = entry.key.strip_prefix("lp:reward_cnt:")
            && let Ok(eid) = id_str.parse::<i32>()
            && entry.val > 0
        {
            exchange_dict.insert(eid, entry.val.clamp(0, i64::from(i32::MAX)) as i32);
        }
    }
    Ok(ExchangeRewardInfoResponse {
        ExchangeRewardDict: exchange_dict,
        SharedDict: share_counts,
    })
}

pub fn handle_get_level_play_rewards_request(
    state: &mut State,
    req: GetLevelPlayRewardsRequest,
) -> Result<GetLevelPlayRewardsResponse, String> {
    let mut reward_nodes = Vec::new();
    let mut treasure_boxes = Vec::new();
    let qi = &state.player_components.quest.info;
    let completed = qi.completed_level_plays.contains(&req.LevelPlayId);
    if let Some(lp) = state
        .assets
        .tables
        .level_play_data
        .get_data_by_id(&req.LevelPlayId)
    {
        if let Some(rc) = lp.Data.LevelPlayRewardConfig.as_ref()
            && rc.RewardEntityId > 0
            && qi.has_opened_chest(rc.RewardEntityId)
        {
            treasure_boxes.push(rc.RewardEntityId);
        }
        for child_str in &lp.Data.Children {
            if let Ok(child_id) = child_str.parse::<i32>()
                && qi.has_opened_chest(child_id)
                && !treasure_boxes.contains(&child_id)
            {
                treasure_boxes.push(child_id);
            }
        }
    }
    if completed || !treasure_boxes.is_empty() {
        reward_nodes.push(req.LevelPlayId);
    }

    Ok(GetLevelPlayRewardsResponse {
        ErrorCode: Some(ErrorCode::Success),
        TreasureBoxIds: treasure_boxes,
        RewardNodeIds: reward_nodes,
    })
}

pub fn handle_level_play_fast_return_pos_request(
    state: &mut State,
    events: &mut EventQueue,
    req: LevelPlayFastReturnPosRequest,
) -> Result<LevelPlayFastReturnPosResponse, String> {
    if req.NodeId > 0 {
        let owner_id = req.TreeOwnerId.max(0);
        let mut qctx = state.quest_context(events);
        let _ = quest_engine::on_submit_node(&mut qctx, owner_id, req.TreeIncId, req.NodeId)
            .map_err(|e| e.to_string())?;
    }
    Ok(LevelPlayFastReturnPosResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_level_play_switch_sport_mode_request(
    state: &mut State,
    req: LevelPlaySwitchSportModeRequest,
) -> Result<LevelPlaySwitchSportModeResponse, String> {
    let mode = req.NewSportMode.map_or(0, |m| m as i32);
    let pc = &mut state.player_components.progress;
    pc.info.set_key_int("lp:sport_mode", i64::from(mode));
    pc.save(&state.fs).map_err(|e| e.to_string())?;
    Ok(LevelPlaySwitchSportModeResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_single_inst_rechallenge_request(
    state: &mut State,
    events: &mut EventQueue,
    req: SingleInstRechallengeRequest,
) -> Result<SingleInstRechallengeResponse, String> {
    let Some(mut scene) = state.scene.take() else {
        return Err("missing active scene".to_string());
    };
    role_helper::revive_and_heal_formation(
        &mut state.conn,
        &mut scene,
        &mut state.player_components.role,
        events,
        &state.fs,
    );
    let inst_id = scene.instance_id;
    let player_id = scene.player_id;
    state.scene = Some(scene);

    if req.RechallengeInInst {
        if let Some(cfg) = state
            .assets
            .tables
            .instance_dungeon
            .get_data_by_id(&inst_id)
            .map(|c| c.into_owned())
        {
            let pos = [
                cfg.BornPosition[0] as f32,
                cfg.BornPosition[1] as f32,
                cfg.BornPosition[2] as f32,
            ];
            let rot = [
                cfg.BornRotation[0] as f32,
                cfg.BornRotation[1] as f32,
                cfg.BornRotation[2] as f32,
            ];
            let mut qctx = state.quest_context(events);
            quest_actions::update_player_scene_state(&mut qctx, pos, Some(rot))
                .map_err(|e| e.to_string())?;
            let _ = state.conn.push_msg(
                TeleportNotify::MSG_ID,
                &TeleportNotify {
                    MapId: cfg.MapConfigId,
                    Pos: Some(Vector {
                        X: pos[0],
                        Y: pos[1],
                        Z: pos[2],
                    }),
                    Rot: Some(Vector {
                        X: rot[0],
                        Y: rot[1],
                        Z: rot[2],
                    }),
                    ..Default::default()
                },
            );
        }
    } else {
        let _ = state.conn.push_msg(
            LeaveSceneNotify::MSG_ID,
            &LeaveSceneNotify {
                PlayerId: player_id,
                SceneId: Vec::new(),
                TransitionOption: Some(TransitionOptionPb::default()),
            },
        );
        events.enqueue(Event::InitialSceneJoin {
            player_id: state.player_id,
            map_config_id: 0,
            instance_id: 0,
            pending_flow: None,
            location: None,
            rotation: None,
        });
    }

    Ok(SingleInstRechallengeResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_apply_rechallenge_request(
    state: &mut State,
    events: &mut EventQueue,
    _req: ApplyRechallengeRequest,
) -> Result<ApplyRechallengeResponse, String> {
    let Some(mut scene) = state.scene.take() else {
        return Err("missing active scene".to_string());
    };
    role_helper::revive_and_heal_formation(
        &mut state.conn,
        &mut scene,
        &mut state.player_components.role,
        events,
        &state.fs,
    );
    state.scene = Some(scene);
    Ok(ApplyRechallengeResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_inst_fight_end_push(
    state: &mut State,
    events: &mut EventQueue,
    req: InstFightEndPush,
) -> Result<(), String> {
    if req.InstId > 0 {
        let key = format!("inst:fight_end:{}", req.InstId);
        let pc = &mut state.player_components.progress;
        let _ = pc.info.mark_claim(&key);
        pc.save(&state.fs).map_err(|e| e.to_string())?;
    }
    let mut qctx = state.quest_context(events);
    if let Err(err) = quest_engine::on_condition_state_changed(&mut qctx) {
        tracing::warn!("onConditionStateChanged hook error: {err}");
    }
    Ok(())
}

pub fn handle_inst_teleport_to_reset_point_request(
    state: &mut State,
    events: &mut EventQueue,
    _req: sonoro_proto::pb::InstTeleportToResetPointRequest,
) -> Result<sonoro_proto::pb::InstTeleportToResetPointResponse, String> {
    if let Some(inst_id) = state.scene.as_ref().map(|s| s.instance_id)
        && let Some(inst) = state
            .assets
            .tables
            .instance_dungeon
            .get_data_by_id(&inst_id)
            .map(|c| c.into_owned())
    {
        let pos = [
            inst.BornPosition[0] as f32,
            inst.BornPosition[1] as f32,
            inst.BornPosition[2] as f32,
        ];
        let rot = [
            inst.BornRotation[0] as f32,
            inst.BornRotation[1] as f32,
            inst.BornRotation[2] as f32,
        ];
        let mut qctx = state.quest_context(events);
        quest_actions::update_player_scene_state(&mut qctx, pos, Some(rot))
            .map_err(|e| e.to_string())?;
        let _ = state.conn.push_msg(
            TeleportNotify::MSG_ID,
            &TeleportNotify {
                MapId: inst_id,
                Pos: Some(Vector {
                    X: pos[0],
                    Y: pos[1],
                    Z: pos[2],
                }),
                Rot: Some(Vector {
                    X: rot[0],
                    Y: rot[1],
                    Z: rot[2],
                }),
                Reason: Some(TeleportReason::Action),
                ..Default::default()
            },
        );
    }
    let mut qctx = state.quest_context(events);
    if let Err(err) = quest_engine::on_condition_state_changed(&mut qctx) {
        tracing::warn!("onConditionStateChanged hook error: {err}");
    }
    Ok(sonoro_proto::pb::InstTeleportToResetPointResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}
