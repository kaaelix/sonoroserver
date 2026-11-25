use std::collections::BTreeMap;

use sonoro_gamesv_data::asset_indexes::entity_world_rot;
use sonoro_gamesv_data::tables::entity_components::components::gravity_flip_from_default;
use sonoro_gamesv_data::tables::entity_components::extra::ExtraEntry;
use sonoro_gamesv_fs::map_info::MapInfo;
use sonoro_gamesv_logic::component::comp_util;
use sonoro_gamesv_logic::component::entity::small::GravityComponent;
use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::helpers::{gravity as gravity_helper, quest_engine, reward::{self, RewardItem}};
use sonoro_proto::enums::{AccessRangeType, ErrorCode, InfrStatusPb, InfrV2StatusPb};
use sonoro_proto::pb::{
    AccessRangeResultMessage, AreaExploreInfo, CountryExploreLevel, CountryExploreScoreInfoRequest,
    CountryExploreScoreInfoResponse, CountryExploreScoreReceived, EntityAccessInfo,
    EntityAccessRangeRequest, EntityAccessRangeResponse, ExploreLevelNotify,
    ExploreProgressRequest, ExploreProgressResponse, ExploreProgressRewardIdsNotify,
    ExploreProgressRewardRequest, ExploreProgressRewardResponse, ExploreScoreRewardRequest,
    ExploreScoreRewardResponse, InfrInfoRequest, InfrInfoResponse, InfrOneRoad, InfrPb,
    InfrRoadPb, InfrV2FirePb, InfrV2InfoRequest, InfrV2InfoResponse, InfrV2OneTree, InfrV2Pb,
    InfrV2TreePb, InitRangeRequest, InitRangeResponse, MapCancelTraceRequest,
    MapCancelTraceResponse, MapMarkAddNotify, MapMarkInfoNotify, MapMarkInfoRequest,
    MapMarkInfoResponse, MapMarkInfoUpdateRequest, MapMarkInfoUpdateResponse, MapMarkRequest,
    MapMarkResponse, MapMarkShowInfo, MapRemoveAllMarkRequest, MapRemoveAllMarkResponse,
    MapRemoveMarkRequest, MapRemoveMarkResponse, MapReplaceMarkRequest, MapReplaceMarkResponse,
    MapTraceInfoRequest, MapTraceInfoResponse, MapTraceRequest, MapTraceResponse,
    MapTravelFullRewardRequest, MapTravelFullRewardResponse, MapTravelLevelUpRequest,
    MapTravelLevelUpResponse, MapUnlockFieldInfoRequest, MapUnlockFieldInfoResponse,
    MulExploreScoreRewardRequest, MulExploreScoreRewardResponse, MulMapTravelSoarRewardRequest,
    MulMapTravelSoarRewardResponse, MulMapTravelTaskRewardRequest, MulMapTravelTaskRewardResponse,
    MultiMapInfoNotify, PlayerAccessRangeRequest, PlayerAccessRangeResponse,
};

use super::State;

const LAHAI_ROI_DUNGEON_ID: i32 = 906;

fn save_map_info(state: &State) -> Result<(), String> {
    comp_util::save_struct(
        &state.fs,
        &state.player_components.map.info,
        &format!("player/{}/{}", state.player_id, MapInfo::DATA_PATH),
    )
    .map_err(|e| e.to_string())
}

fn append_custom_marks(state: &State) -> Vec<sonoro_proto::pb::MarkPointInfo> {
    state
        .player_components
        .map
        .info
        .custom_marks
        .iter()
        .map(|m| m.to_pb())
        .collect()
}

fn compute_area_explore_percent(state: &State, area_id: i32) -> i32 {
    let mut pct = 0i32;

    for &rid in &state.player_components.map.info.claimed_area_rewards {
        if let Some(cfg) = state
            .assets
            .tables
            .explore_progress_reward
            .get_data_by_id(&rid)
            && cfg.Area == area_id
            && cfg.NeedExploreProgress > pct
        {
            pct = cfg.NeedExploreProgress;
        }
    }

    if let Some(es) = state.assets.tables.explore_score.get_data_by_id(&area_id) {
        for &step in es.Score.keys() {
            let key = format!("explore_score:{area_id}:{step}");
            if state.player_components.progress.info.has_claim(&key) && step > pct {
                pct = step;
            }
        }
    }

    let mut total_tp = 0i32;
    let mut unlocked_tp = 0i32;
    for tp in &state.assets.tables.teleporter.items {
        let Some(lcfg) = state
            .assets
            .find_level_entity(tp.MapId, tp.TeleportEntityConfigId)
        else {
            continue;
        };
        if lcfg.AreaId != area_id {
            continue;
        }
        total_tp += 1;
        if state
            .player_components
            .map
            .info
            .unlocked_teleporters
            .contains(&tp.Id)
        {
            unlocked_tp += 1;
        }
    }
    if total_tp > 0 {
        let tp_pct = (unlocked_tp * 100) / total_tp;
        if tp_pct > pct {
            pct = tp_pct;
        }
    }

    if pct == 0
        && state
            .player_components
            .map
            .info
            .unlocked_areas
            .contains(&area_id)
    {
        pct = 10;
    }
    pct.clamp(0, 100)
}

fn compute_country_explore_score(state: &State, country_id: i32) -> i32 {
    let mut total_score = 0i32;
    for area in &state.assets.tables.area.items {
        if area.CountryId != country_id {
            continue;
        }
        let Some(es) = state
            .assets
            .tables
            .explore_score
            .get_data_by_id(&area.AreaId)
        else {
            continue;
        };
        for (&step, &pts) in es.Score.iter() {
            let key = format!("explore_score:{}:{step}", area.AreaId);
            if state.player_components.progress.info.has_claim(&key) {
                total_score += pts;
            }
        }
    }
    total_score
}

fn append_max_explore_level_entries(
    state: &State,
    country_filter: Option<i32>,
) -> Vec<CountryExploreLevel> {
    let mut levels: Vec<CountryExploreLevel> = Vec::new();
    for rw in &state.assets.tables.explore_reward.items {
        if let Some(cid) = country_filter
            && rw.Country != cid
        {
            continue;
        }
        if levels.iter().any(|entry| entry.CountryId == rw.Country) {
            continue;
        }
        let score = compute_country_explore_score(state, rw.Country);
        let mut lvl = 0i32;
        let mut max_lvl = 0i32;
        for r in &state.assets.tables.explore_reward.items {
            if r.Country != rw.Country {
                continue;
            }
            if r.ExploreLevel > max_lvl {
                max_lvl = r.ExploreLevel;
            }
            if r.NeedScore > 0 && score >= r.NeedScore && r.ExploreLevel + 1 > lvl {
                lvl = r.ExploreLevel + 1;
            }
        }
        levels.push(CountryExploreLevel {
            CountryId: rw.Country,
            ExploreLevel: lvl.min(max_lvl),
        });
    }
    levels
}

pub fn handle_map_trace_request(
    state: &mut State,
    req: MapTraceRequest,
) -> Result<MapTraceResponse, String> {
    state.player_components.map.info.track_mark(req.MarkId);
    save_map_info(state)?;
    Ok(MapTraceResponse {
        ErrorCode: Some(ErrorCode::Success),
        MarkId: req.MarkId,
    })
}

pub fn handle_map_cancel_trace_request(
    state: &mut State,
    req: MapCancelTraceRequest,
) -> Result<MapCancelTraceResponse, String> {
    state.player_components.map.info.untrack_mark(req.MarkId);
    save_map_info(state)?;
    Ok(MapCancelTraceResponse {
        ErrorCode: Some(ErrorCode::Success),
        MarkId: req.MarkId,
    })
}

pub fn handle_map_trace_info_request(
    state: &mut State,
    _req: MapTraceInfoRequest,
) -> Result<MapTraceInfoResponse, String> {
    Ok(MapTraceInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        MarkIdList: state.player_components.map.info.tracked_mark_ids.clone(),
    })
}

pub fn handle_map_mark_request(
    state: &mut State,
    req: MapMarkRequest,
) -> Result<MapMarkResponse, String> {
    let Some(request_mark) = req.MarkPointRequestInfo else {
        return Ok(MapMarkResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
            Info: None,
        });
    };
    let saved_mark = state
        .player_components
        .map
        .info
        .add_custom_mark(&request_mark);
    if request_mark.IsTrace {
        state.player_components.map.info.track_mark(saved_mark.id);
    }
    save_map_info(state)?;
    let mark = saved_mark.to_pb();
    let _ = state.conn.push_msg(
        MapMarkAddNotify::MSG_ID,
        &MapMarkAddNotify {
            Info: Some(mark.clone()),
            ..Default::default()
        },
    );
    Ok(MapMarkResponse {
        ErrorCode: Some(ErrorCode::Success),
        Info: Some(mark),
    })
}

pub fn handle_map_remove_mark_request(
    state: &mut State,
    req: MapRemoveMarkRequest,
) -> Result<MapRemoveMarkResponse, String> {
    state
        .player_components
        .map
        .info
        .delete_custom_marks(&req.MarkIds);
    save_map_info(state)?;
    let _ = state.conn.push_msg(
        MapMarkInfoNotify::MSG_ID,
        &MapMarkInfoNotify {
            InfoList: append_custom_marks(state),
            ..Default::default()
        },
    );
    Ok(MapRemoveMarkResponse {
        ErrorCode: Some(ErrorCode::Success),
        MarkIds: req.MarkIds,
    })
}

pub fn handle_map_mark_info_request(
    state: &mut State,
    _req: MapMarkInfoRequest,
) -> Result<MapMarkInfoResponse, String> {
    let show_ids = state
        .player_components
        .map
        .info
        .tracked_mark_ids
        .iter()
        .map(|&mid| MapMarkShowInfo {
            MarkId: mid,
            ShowFlag: 1,
        })
        .collect();
    Ok(MapMarkInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        InfoList: append_custom_marks(state),
        ShowMarkIds: show_ids,
    })
}

pub fn handle_map_remove_all_mark_request(
    state: &mut State,
    req: MapRemoveAllMarkRequest,
) -> Result<MapRemoveAllMarkResponse, String> {
    let removed: Vec<i32> = state
        .player_components
        .map
        .info
        .custom_marks
        .iter()
        .filter(|m| req.MarkType == 0 || m.mark_type == req.MarkType)
        .map(|m| m.id)
        .collect();
    if !removed.is_empty() {
        state
            .player_components
            .map
            .info
            .delete_custom_marks(&removed);
        save_map_info(state)?;
        let _ = state.conn.push_msg(
            MapMarkInfoNotify::MSG_ID,
            &MapMarkInfoNotify {
                InfoList: append_custom_marks(state),
                ..Default::default()
            },
        );
    }
    Ok(MapRemoveAllMarkResponse {
        ErrorCode: Some(ErrorCode::Success),
        MarkIdList: removed,
    })
}

pub fn handle_map_unlock_field_info_request(
    state: &mut State,
    _req: MapUnlockFieldInfoRequest,
) -> Result<MapUnlockFieldInfoResponse, String> {
    let multi_maps: Vec<i32> = state
        .assets
        .tables
        .multi_map
        .items
        .iter()
        .map(|m| m.Id)
        .collect();
    let map_blocks: Vec<i32> = state
        .assets
        .tables
        .map_block_info
        .items
        .iter()
        .map(|b| b.BlockId)
        .collect();
    let _ = state.conn.push_msg(
        MultiMapInfoNotify::MSG_ID,
        &MultiMapInfoNotify {
            UnlockMultiMapId: multi_maps,
            UnlockBlockId: map_blocks,
        },
    );
    Ok(MapUnlockFieldInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        FieldId: state.player_components.map.info.unlocked_fogs.clone(),
    })
}

pub fn handle_explore_progress_request(
    state: &mut State,
    req: ExploreProgressRequest,
) -> Result<ExploreProgressResponse, String> {
    let _ = state.conn.push_msg(
        ExploreLevelNotify::MSG_ID,
        &ExploreLevelNotify {
            CountryExploreLevel: append_max_explore_level_entries(state, None),
        },
    );
    let _ = state.conn.push_msg(
        ExploreProgressRewardIdsNotify::MSG_ID,
        &ExploreProgressRewardIdsNotify {
            AreaStageRewardDataList: state
                .player_components
                .map
                .info
                .claimed_area_rewards
                .clone(),
        },
    );
    let area_progress = req
        .AreaIds
        .into_iter()
        .map(|area_id| AreaExploreInfo {
            AreaId: area_id,
            ExplorePercent: compute_area_explore_percent(state, area_id),
            ..Default::default()
        })
        .collect();
    Ok(ExploreProgressResponse {
        AreaProgress: area_progress,
    })
}

pub fn handle_explore_progress_reward_request(
    state: &mut State,
    req: ExploreProgressRewardRequest,
) -> Result<ExploreProgressRewardResponse, String> {
    let mut claimed_ids = Vec::new();
    let mut dirty = false;
    for reward_id in req.RewardIds {
        let Some(drop_reward) = state
            .assets
            .tables
            .explore_progress_reward
            .get_data_by_id(&reward_id)
            .map(|c| c.DropReward)
        else {
            continue;
        };
        if !state.player_components.map.info.claim_area_reward(reward_id) {
            continue;
        }
        dirty = true;
        if drop_reward > 0 {
            let mut rctx = state.reward_context();
            let _ = reward::grant_drop_package_popup(&mut rctx, drop_reward, 19500)
                .map_err(|e| e.to_string())?;
        }
        claimed_ids.push(reward_id);
    }
    if dirty {
        save_map_info(state)?;
    }
    Ok(ExploreProgressRewardResponse {
        RewardIds: claimed_ids,
    })
}

pub fn handle_country_explore_score_info_request(
    state: &mut State,
    req: CountryExploreScoreInfoRequest,
) -> Result<CountryExploreScoreInfoResponse, String> {
    let mut received = Vec::new();
    let mut total_score = 0i32;
    for area in &state.assets.tables.area.items {
        if area.CountryId != req.CountryId {
            continue;
        }
        let Some(es) = state
            .assets
            .tables
            .explore_score
            .get_data_by_id(&area.AreaId)
        else {
            continue;
        };
        let mut steps = Vec::new();
        for (&step, &pts) in es.Score.iter() {
            let key = format!("explore_score:{}:{step}", area.AreaId);
            if state.player_components.progress.info.has_claim(&key) {
                total_score += pts;
                steps.push(step);
            }
        }
        if !steps.is_empty() {
            received.push(CountryExploreScoreReceived {
                AreaId: area.AreaId,
                ExploreProgress: steps,
            });
        }
    }

    let _ = state.conn.push_msg(
        ExploreLevelNotify::MSG_ID,
        &ExploreLevelNotify {
            CountryExploreLevel: append_max_explore_level_entries(state, Some(req.CountryId)),
        },
    );
    Ok(CountryExploreScoreInfoResponse {
        ExploreScore: total_score,
        CountryExploreScoreReceived: received,
    })
}

fn claim_explore_score_step(
    state: &mut State,
    area_id: i32,
    progress_step: i32,
) -> Result<bool, String> {
    let key = format!("explore_score:{area_id}:{progress_step}");
    if !state.player_components.progress.info.mark_claim(&key) {
        return Ok(false);
    }
    let table_id = if state
        .assets
        .tables
        .explore_score
        .get_data_by_id(&area_id)
        .is_some()
    {
        i64::from(area_id)
    } else {
        i64::from(progress_step)
    };
    let mut rctx = state.reward_context();
    let _ = reward::grant_table_reward(
        &mut rctx,
        "ExploreScoreReward",
        table_id,
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
                count: 15_000,
            },
        ],
        None,
    )
    .map_err(|e| e.to_string())?;
    Ok(true)
}

pub fn handle_explore_score_reward_request(
    state: &mut State,
    req: ExploreScoreRewardRequest,
) -> Result<ExploreScoreRewardResponse, String> {
    if claim_explore_score_step(state, req.AreaId, req.ExploreProgress)? {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        let _ = state.conn.push_msg(
            ExploreLevelNotify::MSG_ID,
            &ExploreLevelNotify {
                CountryExploreLevel: append_max_explore_level_entries(state, None),
            },
        );
    }
    Ok(ExploreScoreRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_mul_explore_score_reward_request(
    state: &mut State,
    req: MulExploreScoreRewardRequest,
) -> Result<MulExploreScoreRewardResponse, String> {
    let mut dirty = false;
    for (area_id, step) in req.Rewards {
        if claim_explore_score_step(state, area_id, step)? {
            dirty = true;
        }
    }
    if dirty {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        let _ = state.conn.push_msg(
            ExploreLevelNotify::MSG_ID,
            &ExploreLevelNotify {
                CountryExploreLevel: append_max_explore_level_entries(state, None),
            },
        );
    }
    Ok(MulExploreScoreRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

#[allow(clippy::too_many_arguments)]
fn track_range_area(
    state: &mut State,
    events: &mut EventQueue,
    range_entity_id: i64,
    is_enter: bool,
    is_player_range: bool,
    trigger_events: bool,
    out_results: &mut BTreeMap<i32, ErrorCode>,
    progress_dirty: &mut bool,
) -> Result<(), String> {
    let mut area_id = 0i32;
    let mut resolved_in_world = false;
    let mut has_range_comp = false;
    let mut has_trample = false;
    let mut has_portal = false;
    let mut has_air_passage = false;
    let mut has_gravity_flip = false;
    let mut has_gravity_volume = false;
    let mut resolved_gravity_dir: Option<[f32; 3]> = None;
    let mut has_roll_block = false;
    let mut cfg_id = i32::try_from(range_entity_id).unwrap_or(0);
    let mut trigger_comp = None;
    let mut client_trigger_comp = None;

    if let Some(scene) = state.scene.as_mut() {
        if scene.net_id_map.contains_key(&range_entity_id) {
            resolved_in_world = true;
        }
        let resolved = scene
            .find_config_id_by_net_id(range_entity_id)
            .map_or(range_entity_id, i64::from);
        if let Ok(cid32) = i32::try_from(resolved) {
            cfg_id = cid32;
        }
        if let Some(lcfg) = state.assets.find_level_entity(scene.instance_id, resolved) {
            resolved_in_world = true;
            area_id = lcfg.AreaId;
            let mut components = lcfg.ComponentsData.clone();
            if let Some(tpl) = state.assets.find_template_config(&lcfg.BlueprintType) {
                tpl.ComponentsData.merge_into(&mut components);
            }
            has_range_comp = components.RangeComponent.is_some();
            has_portal = components.PortalComponent.is_some()
                || components.NoRenderPortalComponent.is_some();
            if let Some(gf) = components.GravityFlipComponent.as_ref() {
                has_gravity_flip = !gf.Disabled.unwrap_or(false);
                if has_gravity_flip {
                    let default_flip = gf
                        .DefaultGravity
                        .and_then(gravity_flip_from_default)
                        .unwrap_or(0);
                    let flip_val = scene
                        .entity(range_entity_id)
                        .and_then(|s| s.world_item.gravity_flip)
                        .unwrap_or(default_flip);
                    resolved_gravity_dir = match flip_val {
                        0 => Some([0.0, 0.0, 1.0]),
                        1 => Some([0.0, 0.0, -1.0]),
                        2 => Some([-1.0, 0.0, 0.0]),
                        3 => Some([1.0, 0.0, 0.0]),
                        _ => None,
                    };
                }
            }
            if resolved_gravity_dir.is_none() && components.BaseInfoComponent.is_some() {
                let gres = gravity_helper::resolve_components_gravity(
                    &components,
                    entity_world_rot(&lcfg),
                );
                if gres.configured {
                    has_gravity_volume = true;
                    resolved_gravity_dir = Some(gres.dir);
                }
            } else if has_gravity_flip {
                has_gravity_volume = true;
            }
            trigger_comp = components.TriggerComponent.clone();
            has_air_passage = lcfg.BlueprintType.contains("AirPassage");
            has_roll_block = lcfg.BlueprintType.contains("RollBlock");
            for entry in components.extra.iter().chain(components.template_extra.iter()) {
                match entry {
                    ExtraEntry::TrampleComponent(tc) => {
                        if !tc.Disabled.unwrap_or(false) {
                            has_trample = true;
                        }
                    }
                    ExtraEntry::AirPassageComponent(_) => has_air_passage = true,
                    ExtraEntry::RollBlockComponent(_)
                    | ExtraEntry::RollBlockFloorComponent(_)
                    | ExtraEntry::RollBlockItemComponent(_) => has_roll_block = true,
                    ExtraEntry::ClientTriggerComponent(ct) => {
                        client_trigger_comp = Some((**ct).clone());
                        if !ct.Disabled.unwrap_or(false) && components.SplineComponent.is_some() {
                            has_air_passage = true;
                        }
                    }
                    _ => {}
                }
            }
        }
        if resolved_in_world || is_player_range {
            out_results.insert(5, ErrorCode::Success);
        }
        if is_player_range {
            let pid = scene.player_id;
            if let Some(ent) = scene.entity_mut(range_entity_id) {
                ent.world_item.in_range_player_id = if is_enter { pid } else { 0 };
            }
            if has_gravity_volume {
                let role_nid = scene.current_role_entity_id();
                if let Some(role_ent) = scene.entity_mut(role_nid) {
                    role_ent.world_item.gravity = if is_enter {
                        resolved_gravity_dir.map(|location| GravityComponent { location })
                    } else {
                        None
                    };
                }
                let mut qctx = state.quest_context(events);
                quest_engine::events::on_condition_state_changed(&mut qctx)
                    .map_err(|e| e.to_string())?;
            }
        }
    } else {
        out_results.insert(5, ErrorCode::Success);
    }

    if has_trample {
        out_results.insert(1, ErrorCode::Success);
    }
    if has_portal {
        out_results.insert(2, ErrorCode::Success);
    }
    if has_range_comp {
        out_results.insert(3, ErrorCode::Success);
    }
    if has_air_passage {
        out_results.insert(4, ErrorCode::Success);
    }
    if has_gravity_flip {
        out_results.insert(6, ErrorCode::Success);
    }
    if has_roll_block {
        out_results.insert(7, ErrorCode::Success);
    }

    if let Some(ref trig) = trigger_comp
        && !trig.Disabled.unwrap_or(false)
    {
        if trigger_events
            && let Some(scene) = state.scene.as_mut()
            && let Some(ent) = scene.entity_mut(range_entity_id)
        {
            if is_enter {
                ent.world_item.trigger_count = ent.world_item.trigger_count.saturating_add(1);
            } else {
                ent.world_item.exit_trigger_count =
                    ent.world_item.exit_trigger_count.saturating_add(1);
            }
        }
        out_results.insert(0, ErrorCode::Success);
        if trigger_events {
            let mut qctx = state.quest_context(events);
            if quest_engine::triggers::handle_entity_range_trigger(
                &mut qctx,
                range_entity_id,
                cfg_id,
                trig,
                is_enter,
            )
            .map_err(|e| e.to_string())?
            {
                *progress_dirty = true;
            }
        }
    }
    if let Some(ref ct) = client_trigger_comp
        && trigger_events
    {
        let mut qctx = state.quest_context(events);
        quest_engine::triggers::handle_client_trigger_actions(
            &mut qctx,
            range_entity_id,
            cfg_id,
            ct,
            is_enter,
        )
        .map_err(|e| e.to_string())?;
        *progress_dirty = true;
    }

    if is_enter
        && area_id > 0
        && state.player_components.map.info.unlock_area(area_id)
    {
        save_map_info(state)?;
    }
    Ok(())
}

pub fn handle_player_access_range_request(
    state: &mut State,
    events: &mut EventQueue,
    req: PlayerAccessRangeRequest,
) -> Result<PlayerAccessRangeResponse, String> {
    let rtype = req.AccessRangeType.unwrap_or(AccessRangeType::RangeEnter);
    let is_enter = matches!(
        rtype,
        AccessRangeType::RangeEnter | AccessRangeType::RangeInit
    );
    let mut results = BTreeMap::new();
    let mut progress_dirty = false;
    track_range_area(
        state,
        events,
        req.RangeEntityId,
        is_enter,
        true,
        true,
        &mut results,
        &mut progress_dirty,
    )?;
    if progress_dirty {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(PlayerAccessRangeResponse {
        AccessRangeResult: Some(AccessRangeResultMessage {
            AccessEntityId: req.RangeEntityId,
            AccessRangeType: req.AccessRangeType,
            AccessRangeResults: results,
        }),
        RangeEntityId: req.RangeEntityId,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_entity_access_range_request(
    state: &mut State,
    events: &mut EventQueue,
    req: EntityAccessRangeRequest,
) -> Result<EntityAccessRangeResponse, String> {
    let rtype = req
        .RangeType
        .unwrap_or(sonoro_proto::enums::RangeType::RangeEnter);
    let is_enter = matches!(
        rtype,
        sonoro_proto::enums::RangeType::RangeEnter | sonoro_proto::enums::RangeType::RangeInit
    );
    let mut range_results = BTreeMap::new();
    let mut progress_dirty = false;
    track_range_area(
        state,
        events,
        req.EntityId,
        is_enter,
        false,
        false,
        &mut range_results,
        &mut progress_dirty,
    )?;

    let next_val = i64::from(is_enter);
    let mut infos = Vec::with_capacity(req.EntitiesToCheck.len());
    for eid in req.EntitiesToCheck {
        let ekey = format!("range:ent:{}:{eid}", req.EntityId);
        if state.player_components.progress.info.get_key_int(&ekey) != next_val {
            state
                .player_components
                .progress
                .info
                .set_key_int(&ekey, next_val);
            progress_dirty = true;
        }
        if let Some(scene) = state.scene.as_mut()
            && let Some(ent) = scene.entity_mut(req.EntityId)
        {
            let wi = &mut ent.world_item;
            if is_enter {
                let n = usize::from(wi.in_range_entity_len).min(wi.in_range_entity_ids.len());
                if !wi.in_range_entity_ids[..n].contains(&eid) && n < wi.in_range_entity_ids.len() {
                    wi.in_range_entity_ids[n] = eid;
                    wi.in_range_entity_len += 1;
                }
            } else {
                let mut i = 0usize;
                while i < usize::from(wi.in_range_entity_len) {
                    if wi.in_range_entity_ids[i] == eid {
                        wi.in_range_entity_len -= 1;
                        let last = usize::from(wi.in_range_entity_len);
                        wi.in_range_entity_ids[i] = wi.in_range_entity_ids[last];
                        wi.in_range_entity_ids[last] = 0;
                    } else {
                        i += 1;
                    }
                }
            }
        }
        infos.push(EntityAccessInfo {
            EntityId: eid,
            RangeType: req.RangeType,
            AcessRangeResults: range_results.clone(),
        });
    }

    if progress_dirty {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(EntityAccessRangeResponse {
        ErrorCode: Some(ErrorCode::Success),
        EntityId: req.EntityId,
        Info: infos,
    })
}

pub fn handle_init_range_request(
    state: &mut State,
    events: &mut EventQueue,
    req: InitRangeRequest,
) -> Result<InitRangeResponse, String> {
    let mut range_results = BTreeMap::new();
    let mut progress_dirty = false;
    track_range_area(
        state,
        events,
        req.EntityId,
        req.IsPlayerInRange,
        true,
        req.IsPlayerInRange,
        &mut range_results,
        &mut progress_dirty,
    )?;

    let mut infos = Vec::with_capacity(req.EntitiesToRequest.len());
    for eid in req.EntitiesToRequest {
        let ekey = format!("range:ent:{}:{eid}", req.EntityId);
        if state.player_components.progress.info.get_key_int(&ekey) != 1 {
            state.player_components.progress.info.set_key_int(&ekey, 1);
            progress_dirty = true;
        }
        infos.push(EntityAccessInfo {
            EntityId: eid,
            RangeType: Some(sonoro_proto::enums::RangeType::RangeInit),
            AcessRangeResults: range_results.clone(),
        });
    }
    if progress_dirty {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(InitRangeResponse {
        ErrorCode: Some(ErrorCode::Success),
        EntityId: req.EntityId,
        Info: infos,
        PlayerAccessRangeResult: Some(EntityAccessInfo {
            EntityId: req.EntityId,
            RangeType: Some(sonoro_proto::enums::RangeType::RangeInit),
            AcessRangeResults: range_results,
        }),
    })
}

pub fn handle_infr_v2_info_request(
    state: &mut State,
    _req: InfrV2InfoRequest,
) -> Result<InfrV2InfoResponse, String> {
    let tree_list = state
        .assets
        .tables
        .infr_v2_tree_build
        .items
        .iter()
        .map(|cfg| InfrV2OneTree {
            TreeId: cfg.Id,
            status: Some(InfrV2StatusPb::InfrV2StatusComplete),
            ..Default::default()
        })
        .collect();
    Ok(InfrV2InfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        InfrInfo: Some(InfrV2Pb {
            FireInfo: Some(InfrV2FirePb {
                FireLevel: 1,
                FireStatus: Some(InfrV2StatusPb::InfrV2StatusComplete),
                ..Default::default()
            }),
            TreeInfo: Some(InfrV2TreePb {
                Trees: tree_list,
                ManualTraceTree: 0,
            }),
            ..Default::default()
        }),
    })
}

pub fn handle_infr_info_request(
    state: &mut State,
    _req: InfrInfoRequest,
) -> Result<InfrInfoResponse, String> {
    let roads = state
        .assets
        .tables
        .infr_road_build
        .items
        .iter()
        .filter(|cfg| cfg.DungeonId == LAHAI_ROI_DUNGEON_ID)
        .map(|cfg| InfrOneRoad {
            RoadId: cfg.Id,
            status: Some(InfrStatusPb::InfrStatusComplete),
            ..Default::default()
        })
        .collect();
    Ok(InfrInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        InfrInfo: Some(InfrPb {
            RoadInfo: Some(InfrRoadPb {
                Roads: roads,
                ..Default::default()
            }),
            ..Default::default()
        }),
    })
}

pub fn handle_map_travel_level_up_request(
    state: &mut State,
    _req: MapTravelLevelUpRequest,
) -> Result<MapTravelLevelUpResponse, String> {
    let next_lvl = state
        .player_components
        .progress
        .info
        .add_key_int("map_travel:lvl", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    let mut rctx = state.reward_context();
    let _ = reward::grant_table_reward(
        &mut rctx,
        "MapTravelLevel",
        next_lvl,
        false,
        true,
        1,
        &[
            RewardItem {
                item_id: 3,
                count: 50,
            },
            RewardItem {
                item_id: 2,
                count: 10_000,
            },
        ],
        None,
    )
    .map_err(|e| e.to_string())?;
    Ok(MapTravelLevelUpResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_map_travel_full_reward_request(
    state: &mut State,
    _req: MapTravelFullRewardRequest,
) -> Result<MapTravelFullRewardResponse, String> {
    if state
        .player_components
        .progress
        .info
        .mark_claim("map_travel:full_reward")
    {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        let cfg_key = state
            .assets
            .reward_index
            .get_rows("MapTravelLevel")
            .first()
            .map_or(1, |r| r.key);
        let mut rctx = state.reward_context();
        let _ = reward::grant_table_reward(
            &mut rctx,
            "MapTravelLevel",
            cfg_key,
            false,
            true,
            1,
            &[
                RewardItem {
                    item_id: 3,
                    count: 160,
                },
                RewardItem {
                    item_id: 2,
                    count: 30_000,
                },
            ],
            None,
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(MapTravelFullRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_mul_map_travel_soar_reward_request(
    state: &mut State,
    req: MulMapTravelSoarRewardRequest,
) -> Result<MulMapTravelSoarRewardResponse, String> {
    let mut claimed = Vec::new();
    let mut dirty = false;
    for tid in req.TaskIds {
        if !state
            .player_components
            .progress
            .info
            .mark_claim(&format!("map_travel_soar:{tid}"))
        {
            continue;
        }
        dirty = true;
        claimed.push(tid);
        let mut rctx = state.reward_context();
        let _ = reward::grant_table_reward(
            &mut rctx,
            "MapTravelTask",
            i64::from(tid),
            false,
            true,
            1,
            &[RewardItem {
                item_id: 3,
                count: 30,
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
    Ok(MulMapTravelSoarRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
        TaskIds: claimed,
    })
}

pub fn handle_mul_map_travel_task_reward_request(
    state: &mut State,
    req: MulMapTravelTaskRewardRequest,
) -> Result<MulMapTravelTaskRewardResponse, String> {
    let mut claimed = Vec::new();
    let mut dirty = false;
    for tid in req.TaskId {
        if !state
            .player_components
            .progress
            .info
            .mark_claim(&format!("map_travel_task:{tid}"))
        {
            continue;
        }
        dirty = true;
        claimed.push(tid);
        let mut rctx = state.reward_context();
        let _ = reward::grant_table_reward(
            &mut rctx,
            "MapTravelTask",
            i64::from(tid),
            false,
            true,
            1,
            &[RewardItem {
                item_id: 3,
                count: 30,
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
    Ok(MulMapTravelTaskRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
        TaskId: claimed,
    })
}

pub fn handle_map_replace_mark_request(
    state: &mut State,
    req: MapReplaceMarkRequest,
) -> Result<MapReplaceMarkResponse, String> {
    let mut changed = false;
    for mark in &mut state.player_components.map.info.custom_marks {
        if mark.id == req.MarkId {
            mark.config_id = req.ConfigId;
            changed = true;
            break;
        }
    }
    if changed {
        save_map_info(state)?;
    }
    Ok(MapReplaceMarkResponse {
        ErrorCode: Some(ErrorCode::Success),
        MarkId: req.MarkId,
        ConfigId: req.ConfigId,
    })
}

pub fn handle_map_mark_info_update_request(
    state: &mut State,
    req: MapMarkInfoUpdateRequest,
) -> Result<MapMarkInfoUpdateResponse, String> {
    if let Some(pos) = req.Pos {
        let mut changed = false;
        for mark in &mut state.player_components.map.info.custom_marks {
            if mark.id == req.MarkId {
                mark.x = pos.X;
                mark.y = pos.Y;
                mark.z = pos.Z;
                changed = true;
                break;
            }
        }
        if changed {
            save_map_info(state)?;
        }
    }
    Ok(MapMarkInfoUpdateResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}
