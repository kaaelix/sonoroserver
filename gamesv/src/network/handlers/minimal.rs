use std::collections::{BTreeMap, HashMap};
use std::sync::OnceLock;

use serde_json::Value;
use sonoro_gamesv_logic::component::entity::small::{ConfigType, EntityType};
use sonoro_gamesv_logic::component::player::PlayerEchoComponent;
use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::events::Event;
use sonoro_gamesv_logic::fsm::types::wake_reason;
use sonoro_gamesv_logic::helpers::reward::RewardItem;
use sonoro_gamesv_logic::helpers::{
    attributes as attributes_helper, condition, pb_kv, quest_actions, quest_engine, reward,
    role as role_helper,
};
use sonoro_proto::enums::{
    DetectionType, EAttributeType, ErrorCode, LivingStatus, PhantomFuncValue, PhantomSettingType,
    TeleportReason,
};
use sonoro_proto::wire::PbEnum;
use sonoro_proto::pb::{
    AdviceSetRequest, AdviceSetResponse, BlackScreenStatePush, BtnStateRequest, BtnStateResponse,
    ButtonEnableResult, CdKeyVerifyRequest, CdKeyVerifyResponse, CharacterDeathPush,
    ClientCurrentRoleReportRequest, ClientCurrentRoleReportResponse,
    ClientPullResourcePackageRequest, ClientPullResourcePackageResponse,
    CombatMessagePostInfoRequest, CombatMessagePostInfoResponse, CombatMessagePrefixInfoRequest,
    CombatMessagePrefixInfoResponse, CombatReceiveData, CombatReceiveDataMessageOneof,
    CombatReceivePackNotify, DErrorResult, DetectionRequest, DetectionResponse, DetectionTarget,
    DollDeliveryInfo, DollSmallMapInfoRequest, DollSmallMapInfoResponse, DrownEndTeleportRequest,
    DrownEndTeleportResponse, DrownRequest, DrownResponse, EnterViewDirectionRequest,
    EnterViewDirectionResponse, EntityFollowTrackRequest, EntityFollowTrackResponse,
    EntityLivingStatusNotify, EntityOnLandedRequest, EntityOnLandedResponse,
    EntityPatrolStopRequest, EntityPatrolStopResponse,
    ExitViewDirectionRequest, ExitViewDirectionResponse,
    FlowServerSkipNotify, FormationAttrNotify, FormationAttrRequest, FormationAttrResponse,
    FragmentMemoryItem, GetDetectionLabelInfoRequest, GetDetectionLabelInfoResponse,
    GivebackInfoRequest, GivebackInfoResponse, ItemExchangeInfo, ItemExchangeInfoRequest,
    ItemExchangeInfoResponse, ItemFuncValueUpdateNotify, LevelPlayStateMsg, LevelPlayStateRequest,
    LevelPlayStateResponse, LevelPlayVarAsyncRequest, LevelPlayVarAsyncResponse,
    MainPhantomRecommendInfo, MaxMessageIdPush, MonsterDetectionInfoRequest,
    MonsterDetectionInfoResponse, NetTestDataPush, PayInfoRequest, PayInfoResponse, PayItemInfo,
    PhBaOneAllSuitPlan, PhBaPlanBatchOperRequest, PhBaPlanBatchOperResponse,
    PhBaPlanFindPlanRequest, PhBaPlanFindPlanResponse, PhBaPlanSaveUsePlanRequest,
    PhBaPlanSaveUsePlanResponse, PhBaPlanSetFiveStarSwitchRequest,
    PhBaPlanSetFiveStarSwitchResponse, PhBaPlanSetPlanStatusRequest,
    PhBaPlanSetPlanStatusResponse, PhBaPlanUpdatePlanRequest, PhBaPlanUpdatePlanResponse,
    PhBaPlanUsePlanRequest, PhBaPlanUsePlanResponse, PhantomManageConfig,
    PhantomManageConfigRequest, PhantomManageConfigResponse, PhantomManageConfigUpdateRequest,
    PhantomManageConfigUpdateResponse, PhantomSettingBatchUpdateRequest,
    PhantomSettingBatchUpdateResponse, PhotoMemoryRequest, PhotoMemoryResponse,
    PlayerGameTokenRequest, PlayerGameTokenResponse, PlayerNetStatePush,
    PlayerNetStateS2cNotify, PlayerXboxBlockListRequest, PlayerXboxBlockListResponse,
    TeamRecommendInfo, TeamRecommendRole,
    TeamRecommendRoleInfo, TeamRecommendTeammate,
    PrivateChatOperateRequest, PrivateChatOperateResponse, RobotDestinationPush,
    RoleDevelopConfigRequest, RoleDevelopConfigResponse, RoleOperateSelfBgmRequest,
    RoleOperateSelfBgmResponse, RoleVisionMainPhantomRequest, RoleVisionMainPhantomResponse,
    SceneBlockSplitPlayerNeedBlockPush, SceneDollDeliveryInfo, SceneUnlimitedScoreInfo,
    SecGetReportData2FlowRequest, SecGetReportData2FlowResponse, SelectDetectionTargetRequest,
    SelectDetectionTargetResponse, ServerPlayStationPlayOnlyStateRequest,
    ServerPlayStationPlayOnlyStateResponse, SheriffZoneInfo, SheriffZoneInfoRequest,
    SheriffZoneInfoResponse, TeamRecommendRequest, TeamRecommendResponse, TeleportNotify,
    UnlockDetectionLabelInfo, UnlimitedScoreInfo,
    UpdatePlayStationBlockAccountRequest, UpdatePlayStationBlockAccountResponse,
    ValidTimeItemRequest, ValidTimeItemResponse, Vector, WebSignRequest, WebSignResponse,
    XboxSettingRequest, XboxSettingResponse,
};

use super::super::connection::drain_event_queue_best_effort;
use super::{world_puzzle, State};

const PH_BA_SUCCESS_CODE: ErrorCode = ErrorCode::Success;

pub fn handle_doll_small_map_info_request(
    state: &mut State,
    _req: DollSmallMapInfoRequest,
) -> Result<DollSmallMapInfoResponse, String> {
    let mut doll_scores: BTreeMap<i32, (i32, i32)> = BTreeMap::new();
    for entry in &state.player_components.progress.info.dynamic.ints {
        if let Some(rest) = entry.key.strip_prefix("doll:high:")
            && let Ok(eid) = rest.parse::<i32>()
            && eid > 0
            && entry.val > 0
        {
            doll_scores.entry(eid).or_insert((0, 0)).0 = entry.val.clamp(0, i64::from(i32::MAX)) as i32;
        } else if let Some(rest) = entry.key.strip_prefix("doll:acc:")
            && let Ok(eid) = rest.parse::<i32>()
            && eid > 0
            && entry.val > 0
        {
            doll_scores.entry(eid).or_insert((0, 0)).1 = entry.val.clamp(0, i64::from(i32::MAX)) as i32;
        }
    }
    let fallback_score = state
        .player_components
        .progress
        .info
        .get_key_int("doll:score")
        .clamp(0, i64::from(i32::MAX)) as i32;
    let inst_id = state.player_components.scene.last_scene_info.instance_id;
    let inst_id = if inst_id > 0 { inst_id } else { 8 };
    let mut unlimited_list: Vec<UnlimitedScoreInfo> = doll_scores
        .into_iter()
        .map(|(eid, (high, acc))| UnlimitedScoreInfo {
            entityId: eid,
            HighestScore: high,
            AccumulatedScore: acc.max(high),
        })
        .collect();
    if unlimited_list.is_empty() && fallback_score > 0 {
        unlimited_list.push(UnlimitedScoreInfo {
            entityId: 0,
            HighestScore: fallback_score,
            AccumulatedScore: fallback_score,
        });
    }
    let scores = if unlimited_list.is_empty() {
        Vec::new()
    } else {
        vec![SceneUnlimitedScoreInfo {
            instId: inst_id,
            UnlimitedScoreInfos: unlimited_list,
        }]
    };
    let mut by_showcase: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    for c in &state.player_components.progress.info.dynamic.claims {
        if let Some(rest) = c.strip_prefix("doll:item:") {
            let mut it = rest.split(':');
            let Some(sc_str) = it.next() else {
                continue;
            };
            let Some(d_str) = it.next() else {
                continue;
            };
            let Ok(showcase_id) = sc_str.parse::<i32>() else {
                continue;
            };
            let Ok(doll_id) = d_str.parse::<i32>() else {
                continue;
            };
            if showcase_id > 0 && doll_id > 0 {
                by_showcase.entry(showcase_id).or_default().push(doll_id);
            }
        } else if let Some(rest) = c.strip_prefix("doll:delivery:")
            && let Ok(showcase_id) = rest.parse::<i32>()
            && showcase_id > 0
        {
            by_showcase.entry(showcase_id).or_default();
        }
    }
    let delivery_infos: Vec<DollDeliveryInfo> = by_showcase
        .into_iter()
        .map(|(showcase_id, delivered_dolls)| DollDeliveryInfo {
            ShowcaseEntityId: showcase_id,
            DeliveredDolls: delivered_dolls,
        })
        .collect();
    let scene_deliveries = if delivery_infos.is_empty() {
        Vec::new()
    } else {
        let inst_id = state.player_components.scene.last_scene_info.instance_id;
        let inst_id = if inst_id > 0 { inst_id } else { 8 };
        vec![SceneDollDeliveryInfo {
            instId: inst_id,
            DollDeliveryInfos: delivery_infos,
        }]
    };
    Ok(DollSmallMapInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        SceneUnlimitedScoreInfos: scores,
        SceneDollDeliveryInfos: scene_deliveries,
    })
}

pub fn handle_role_vision_main_phantom_request(
    state: &mut State,
    _req: RoleVisionMainPhantomRequest,
) -> Result<RoleVisionMainPhantomResponse, String> {
    let mut list = Vec::new();
    for pi in &state.assets.tables.phantom_item.items {
        if pi.MonsterId > 0 && !pi.FetterGroup.is_empty() {
            list.push(MainPhantomRecommendInfo {
                Usage: 1,
                MonsterId: pi.MonsterId,
                FetterGroupId: pi.FetterGroup[0],
            });
            if list.len() >= 3 {
                break;
            }
        }
    }
    Ok(RoleVisionMainPhantomResponse {
        ErrorCode: Some(ErrorCode::Success),
        RecommendInfo: list,
    })
}

pub fn handle_ph_ba_plan_use_plan_request(
    state: &mut State,
    _req: PhBaPlanUsePlanRequest,
) -> Result<PhBaPlanUsePlanResponse, String> {
    let plan = pb_kv::load_msg::<PhBaOneAllSuitPlan>(
        &state.player_components.progress,
        "phba:use_plan",
    )
    .unwrap_or_default();
    let five_star = state
        .player_components
        .progress
        .info
        .get_key_int("phba_plan:5star")
        != 0;
    Ok(PhBaPlanUsePlanResponse {
        ErrorCode: Some(PH_BA_SUCCESS_CODE),
        SelfPlanCode: b"local-plan".to_vec(),
        UsePlan: Some(plan),
        FiveStarSwitch: five_star,
        TowPlanSame: true,
    })
}

pub fn handle_ph_ba_plan_save_use_plan_request(
    state: &mut State,
    req: PhBaPlanSaveUsePlanRequest,
) -> Result<PhBaPlanSaveUsePlanResponse, String> {
    pb_kv::persist_opt_msg(
        &state.fs,
        &mut state.player_components.progress,
        "phba:use_plan",
        req.UsePlan.as_ref(),
    )
    .map_err(|e| e.to_string())?;
    state
        .player_components
        .progress
        .info
        .mark_claim("phba_plan:saved");
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(PhBaPlanSaveUsePlanResponse {
        ErrorCode: Some(PH_BA_SUCCESS_CODE),
        SelfPlanCode: b"local-plan".to_vec(),
        TowPlanSame: true,
    })
}

pub fn handle_ph_ba_plan_find_plan_request(
    state: &mut State,
    _req: PhBaPlanFindPlanRequest,
) -> Result<PhBaPlanFindPlanResponse, String> {
    let plan = pb_kv::load_msg::<PhBaOneAllSuitPlan>(
        &state.player_components.progress,
        "phba:use_plan",
    )
    .unwrap_or_default();
    Ok(PhBaPlanFindPlanResponse {
        ErrorCode: Some(PH_BA_SUCCESS_CODE),
        Plan: Some(plan),
        TowPlanSame: true,
    })
}

pub fn handle_ph_ba_plan_update_plan_request(
    state: &mut State,
    _req: PhBaPlanUpdatePlanRequest,
) -> Result<PhBaPlanUpdatePlanResponse, String> {
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        "phba_plan:updated",
    )
    .map_err(|e| e.to_string())?;
    Ok(PhBaPlanUpdatePlanResponse {
        ErrorCode: Some(PH_BA_SUCCESS_CODE),
        SelfPlanCode: b"local-plan".to_vec(),
    })
}

pub fn handle_ph_ba_plan_set_five_star_switch_request(
    state: &mut State,
    req: PhBaPlanSetFiveStarSwitchRequest,
) -> Result<PhBaPlanSetFiveStarSwitchResponse, String> {
    let info = &mut state.player_components.progress.info;
    info.set_key_int("phba_plan:5star", i64::from(req.Open));
    info.add_key_int("phba_plan:5star_toggle", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(PhBaPlanSetFiveStarSwitchResponse {
        ErrorCode: Some(PH_BA_SUCCESS_CODE),
    })
}

pub fn handle_ph_ba_plan_set_plan_status_request(
    state: &mut State,
    req: PhBaPlanSetPlanStatusRequest,
) -> Result<PhBaPlanSetPlanStatusResponse, String> {
    let info = &mut state.player_components.progress.info;
    for si in &req.SetInfo {
        info.set_key_int(&format!("phba_plan:suit:{}", si.Suit), i64::from(si.Open));
    }
    info.add_key_int("phba_plan:status", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(PhBaPlanSetPlanStatusResponse {
        ErrorCode: Some(PH_BA_SUCCESS_CODE),
        TowPlanSame: true,
    })
}

pub fn handle_ph_ba_plan_batch_oper_request(
    state: &mut State,
    req: PhBaPlanBatchOperRequest,
) -> Result<PhBaPlanBatchOperResponse, String> {
    for op in req.BatchOper {
        let new_func = match op.Oper.unwrap_or(PhantomFuncValue::Reset) {
            PhantomFuncValue::Locked => 1,
            PhantomFuncValue::Disuse => 2,
            PhantomFuncValue::Reset => 0,
        };
        for incr_id in op.IncrId {
            if let Some(item) = state.player_components.echo.echo_map.get_mut(&incr_id) {
                item.func_value = new_func;
                let _ = state.conn.push_msg(
                    ItemFuncValueUpdateNotify::MSG_ID,
                    &ItemFuncValueUpdateNotify {
                        IncrId: incr_id,
                        FuncValue: new_func,
                    },
                );
            }
        }
    }
    PlayerEchoComponent::save_all(
        &state.fs,
        state.player_id,
        &state.player_components.echo.echo_map,
    )
    .map_err(|e| e.to_string())?;
    Ok(PhBaPlanBatchOperResponse {
        errCode: Some(ErrorCode::Success),
    })
}

pub fn handle_phantom_setting_batch_update_request(
    state: &mut State,
    req: PhantomSettingBatchUpdateRequest,
) -> Result<PhantomSettingBatchUpdateResponse, String> {
    for entry in &req.Settings {
        let Some(setting) = &entry.Setting else {
            continue;
        };
        let pfx = if entry.SettingType.unwrap_or(PhantomSettingType::AutoLock)
            == PhantomSettingType::AutoDisuse
        {
            "phantom_mgr:disuse"
        } else {
            "phantom_mgr:lock"
        };
        let key = format!("{pfx}:{}", setting.Index);
        pb_kv::save_msg(
            &state.fs,
            &mut state.player_components.progress,
            &key,
            setting,
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(PhantomSettingBatchUpdateResponse {
        ErrorCode: Some(ErrorCode::Success),
        Settings: req.Settings,
    })
}

pub fn handle_phantom_manage_config_request(
    state: &mut State,
    _req: PhantomManageConfigRequest,
) -> Result<PhantomManageConfigResponse, String> {
    let pc = &state.player_components.progress;
    let mut auto_lock = Vec::new();
    let mut auto_disuse = Vec::new();
    for i in 0..5 {
        if let Some(cfg) = pb_kv::load_msg::<PhantomManageConfig>(pc, &format!("phantom_mgr:lock:{i}")) {
            auto_lock.push(cfg);
        }
        if let Some(cfg) = pb_kv::load_msg::<PhantomManageConfig>(pc, &format!("phantom_mgr:disuse:{i}")) {
            auto_disuse.push(cfg);
        }
    }
    Ok(PhantomManageConfigResponse {
        AutoLock: auto_lock,
        AutoDisuse: auto_disuse,
    })
}

pub fn handle_phantom_manage_config_update_request(
    state: &mut State,
    req: PhantomManageConfigUpdateRequest,
) -> Result<PhantomManageConfigUpdateResponse, String> {
    if let Some(ref setting) = req.Setting {
        let pfx = if req.SettingType.unwrap_or(PhantomSettingType::AutoLock)
            == PhantomSettingType::AutoDisuse
        {
            "phantom_mgr:disuse"
        } else {
            "phantom_mgr:lock"
        };
        let key = format!("{pfx}:{}", setting.Index);
        pb_kv::save_msg(
            &state.fs,
            &mut state.player_components.progress,
            &key,
            setting,
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(PhantomManageConfigUpdateResponse {
        ErrorCode: Some(ErrorCode::Success),
        Setting: req.Setting,
    })
}

pub fn handle_get_detection_label_info_request(
    state: &mut State,
    _req: GetDetectionLabelInfoRequest,
) -> Result<GetDetectionLabelInfoResponse, String> {
    let mut guide_ids = Vec::new();
    for dd in &state.assets.tables.dungeon_detection.items {
        if dd.GuideId > 0 && !guide_ids.contains(&dd.GuideId) {
            guide_ids.push(dd.GuideId);
        }
    }
    for sd in &state.assets.tables.silent_area_detection.items {
        if sd.GuideId > 0 && !guide_ids.contains(&sd.GuideId) {
            guide_ids.push(sd.GuideId);
        }
    }
    Ok(GetDetectionLabelInfoResponse {
        UnlockLabelInfo: Some(UnlockDetectionLabelInfo {
            UnlockedGuideIds: guide_ids,
            ..Default::default()
        }),
    })
}

pub fn handle_detection_request(
    _state: &mut State,
    req: DetectionRequest,
) -> Result<DetectionResponse, String> {
    Ok(DetectionResponse {
        DetectionConfId: req.DetectionConfId,
        ..Default::default()
    })
}

pub fn handle_select_detection_target_request(
    _state: &mut State,
    req: SelectDetectionTargetRequest,
) -> Result<SelectDetectionTargetResponse, String> {
    Ok(SelectDetectionTargetResponse {
        SelectDetectionTarget: req.SelectDetectionTarget,
        ..Default::default()
    })
}

pub fn handle_monster_detection_info_request(
    state: &mut State,
    _req: MonsterDetectionInfoRequest,
) -> Result<MonsterDetectionInfoResponse, String> {
    let mut targets = Vec::new();
    for m in &state.assets.tables.monster_detection.items {
        targets.push(DetectionTarget {
            Id: m.Id,
            Type: Some(DetectionType::NormalMonster),
            UnlockState: true,
            RefresherTime: 0,
            DetectionId: m.Id,
            IsTrace: 0,
        });
    }
    for d in &state.assets.tables.dungeon_detection.items {
        targets.push(DetectionTarget {
            Id: d.Id,
            Type: Some(DetectionType::Dungeon),
            UnlockState: true,
            RefresherTime: 0,
            DetectionId: d.Id,
            IsTrace: 0,
        });
    }
    for s in &state.assets.tables.silent_area_detection.items {
        targets.push(DetectionTarget {
            Id: s.Id,
            Type: Some(DetectionType::SilentArea),
            UnlockState: true,
            RefresherTime: 0,
            DetectionId: s.Id,
            IsTrace: 0,
        });
    }
    Ok(MonsterDetectionInfoResponse {
        DetectionTarget: targets,
    })
}

pub fn handle_level_play_var_async_request(
    state: &mut State,
    req: LevelPlayVarAsyncRequest,
) -> Result<LevelPlayVarAsyncResponse, String> {
    Ok(LevelPlayVarAsyncResponse {
        ErrorCode: Some(ErrorCode::Success),
        Vars: world_puzzle::build_level_play_vars_for(
            &state.assets,
            &state.player_components,
            req.InstId,
            req.LevelPlayId,
        ),
    })
}

pub fn handle_role_develop_config_request(
    _state: &mut State,
    _req: RoleDevelopConfigRequest,
) -> Result<RoleDevelopConfigResponse, String> {
    Ok(RoleDevelopConfigResponse {
        ErrorCode: Some(ErrorCode::Success),
        Configs: Some(sonoro_proto::pb::RoleDevelopConfigs {
            Version: vec![1],
            ..Default::default()
        }),
    })
}

pub fn handle_server_play_station_play_only_state_request(
    state: &mut State,
    _req: ServerPlayStationPlayOnlyStateRequest,
) -> Result<ServerPlayStationPlayOnlyStateResponse, String> {
    Ok(ServerPlayStationPlayOnlyStateResponse {
        CrossPlayEnabled: state
            .player_components
            .progress
            .info
            .get_key_int("psn:only_play")
            != 0,
    })
}

pub fn handle_xbox_setting_request(
    state: &mut State,
    _req: XboxSettingRequest,
) -> Result<XboxSettingResponse, String> {
    let pc = &state.player_components.progress.info;
    Ok(XboxSettingResponse {
        MatchXboxUser: pc.get_key_int("setting:xbox_match") != 0,
        XboxBlockUserCreatedContentViewing: pc.get_key_int("setting:xbox_block_ugc") != 0,
    })
}

pub fn handle_photo_memory_request(
    state: &mut State,
    _req: PhotoMemoryRequest,
) -> Result<PhotoMemoryResponse, String> {
    let mut items = Vec::new();
    for row in state.assets.reward_index.get_rows("PhotoMemory").iter() {
        let id = row.key as i32;
        items.push(FragmentMemoryItem {
            Id: id,
            IsUnlock: state
                .player_components
                .progress
                .info
                .has_claim(&format!("photo_mem:{id}")),
            ..Default::default()
        });
        if items.len() >= 64 {
            break;
        }
    }
    Ok(PhotoMemoryResponse {
        Item: items,
    })
}

pub fn handle_sheriff_zone_info_request(
    state: &mut State,
    _req: SheriffZoneInfoRequest,
) -> Result<SheriffZoneInfoResponse, String> {
    static CLUE_ZONE_MAP: OnceLock<HashMap<i32, i32>> = OnceLock::new();
    let clue_to_zone = CLUE_ZONE_MAP.get_or_init(|| {
        let mut anomaly_zone = HashMap::new();
        for row in state.assets.read_raw_json_array("SheriffAnomaly") {
            if let (Some(aid), Some(zid)) = (
                row.get("Id").and_then(Value::as_i64),
                row.get("SheriffZoneId").and_then(Value::as_i64),
            ) {
                anomaly_zone.insert(aid as i32, zid.max(1) as i32);
            }
        }
        let mut map = HashMap::new();
        for row in state.assets.read_raw_json_array("SheriffClue") {
            if let Some(cid) = row.get("Id").and_then(Value::as_i64) {
                let aid = row.get("SheriffAnomalyId").and_then(Value::as_i64).unwrap_or(0) as i32;
                let zid = anomaly_zone.get(&aid).copied().unwrap_or(1);
                map.insert(cid as i32, zid);
            }
        }
        map
    });
    let mut per_zone: BTreeMap<i32, i64> = BTreeMap::new();
    for c in &state.player_components.progress.info.dynamic.claims {
        if let Some(rest) = c.strip_prefix("sheriff_clue:") {
            let zone_id = rest
                .parse::<i32>()
                .ok()
                .and_then(|cid| clue_to_zone.get(&cid).copied())
                .unwrap_or(1);
            *per_zone.entry(zone_id).or_insert(0) += 1;
        }
    }
    let zones: Vec<SheriffZoneInfo> = per_zone
        .into_iter()
        .map(|(zone_id, cnt)| SheriffZoneInfo {
            ZoneId: zone_id,
            ItemCount: cnt,
            ..Default::default()
        })
        .collect();
    Ok(SheriffZoneInfoResponse {
        errorCode: Some(ErrorCode::Success),
        ZoneInfos: zones,
    })
}

fn ensure_gm_sign_token(state: &mut State) -> Result<String, String> {
    let db_path = std::path::Path::new("state/gamesv.db");
    let db_token = if db_path.exists()
        && let Ok(conn) = rusqlite::Connection::open(db_path)
        && let Ok(val) = conn.query_row(
            "SELECT val FROM player_kv WHERE player_id = ?1 AND ns = 'gm' AND k = 'sign_token' LIMIT 1",
            [state.player_id],
            |row| row.get::<_, String>(0),
        )
        && val.len() >= 16
    {
        Some(val)
    } else {
        None
    };

    let mem_token = state
        .player_components
        .progress
        .info
        .get_key_str("gm:sign_token")
        .to_string();

    let token = if let Some(tok) = db_token {
        tok
    } else if mem_token.len() >= 16 {
        mem_token.clone()
    } else {
        let mut random_bytes = [0u8; 12];
        getrandom::fill(&mut random_bytes).map_err(|e| format!("rng failed: {e}"))?;
        let mut hex = String::with_capacity(24);
        for b in random_bytes {
            use std::fmt::Write as _;
            let _ = write!(&mut hex, "{b:02x}");
        }
        hex
    };

    if mem_token != token {
        state
            .player_components
            .progress
            .info
            .set_key_str("gm:sign_token", &token);
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }

    if db_path.exists()
        && let Ok(conn) = rusqlite::Connection::open(db_path)
    {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0i64, |d| i64::try_from(d.as_secs()).unwrap_or(i64::MAX));
        let _ = conn.execute(
            "INSERT INTO player_kv(player_id, ns, k, val, updated_at) VALUES(?1, 'gm', 'sign_token', ?2, ?3)
             ON CONFLICT(player_id, ns, k) DO UPDATE SET val = excluded.val, updated_at = excluded.updated_at",
            rusqlite::params![state.player_id, token, ts],
        );
    }

    Ok(token)
}

pub fn handle_web_sign_request(
    state: &mut State,
    _req: WebSignRequest,
) -> Result<WebSignResponse, String> {
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        "web_sign:claimed",
    )
    .map_err(|e| e.to_string())?;
    let token = ensure_gm_sign_token(state)?;
    Ok(WebSignResponse {
        NoticeSign: token.into_bytes(),
    })
}

pub fn handle_client_pull_resource_package_request(
    _state: &mut State,
    _req: ClientPullResourcePackageRequest,
) -> Result<ClientPullResourcePackageResponse, String> {
    Ok(ClientPullResourcePackageResponse {
        ErrorId: Some(ErrorCode::Success),
        ..Default::default()
    })
}

pub fn handle_formation_attr_request(
    state: &mut State,
    req: FormationAttrRequest,
) -> Result<FormationAttrResponse, String> {
    let _ = state.conn.push_msg(
        FormationAttrNotify::MSG_ID,
        &FormationAttrNotify {
            Duration: req.Duration,
            FormationAttrs: req.FormationAttrs,
        },
    );
    Ok(FormationAttrResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_combat_message_prefix_info_request(
    _state: &mut State,
    _req: CombatMessagePrefixInfoRequest,
) -> Result<CombatMessagePrefixInfoResponse, String> {
    Ok(CombatMessagePrefixInfoResponse::default())
}

pub fn handle_private_chat_operate_request(
    state: &mut State,
    req: PrivateChatOperateRequest,
) -> Result<PrivateChatOperateResponse, String> {
    let op = req.OperateType.map(|o| o as i64).unwrap_or(0);
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("chat:priv_op:{}", req.TargetPlayerId),
        op,
    )
    .map_err(|e| e.to_string())?;
    Ok(PrivateChatOperateResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_entity_patrol_stop_request(
    state: &mut State,
    req: EntityPatrolStopRequest,
) -> Result<EntityPatrolStopResponse, String> {
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        &format!("wp:patrol_stop:{}", req.EntityId),
    )
    .map_err(|e| e.to_string())?;
    Ok(EntityPatrolStopResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_giveback_info_request(
    state: &mut State,
    _req: GivebackInfoRequest,
) -> Result<GivebackInfoResponse, String> {
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        "giveback:viewed",
    )
    .map_err(|e| e.to_string())?;
    Ok(GivebackInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_valid_time_item_request(
    _state: &mut State,
    _req: ValidTimeItemRequest,
) -> Result<ValidTimeItemResponse, String> {
    Ok(ValidTimeItemResponse::default())
}

pub fn handle_update_play_station_block_account_request(
    _state: &mut State,
    _req: UpdatePlayStationBlockAccountRequest,
) -> Result<UpdatePlayStationBlockAccountResponse, String> {
    Ok(UpdatePlayStationBlockAccountResponse::default())
}

pub fn handle_player_xbox_block_list_request(
    _state: &mut State,
    _req: PlayerXboxBlockListRequest,
) -> Result<PlayerXboxBlockListResponse, String> {
    Ok(PlayerXboxBlockListResponse::default())
}

pub fn handle_team_recommend_request(
    state: &mut State,
    _req: TeamRecommendRequest,
) -> Result<TeamRecommendResponse, String> {
    let mut role_ids: Vec<i32> = state
        .player_components
        .role
        .role_map
        .keys()
        .copied()
        .collect();
    role_ids.sort_unstable();
    if role_ids.is_empty() {
        role_ids = vec![1402, 1102, 1202];
    }

    let list = role_ids
        .iter()
        .take(6)
        .map(|&rid| {
            let teammates = role_ids
                .iter()
                .copied()
                .filter(|&other| other != rid)
                .take(2)
                .map(|other| TeamRecommendTeammate {
                    Main: Some(TeamRecommendRoleInfo {
                        RoleId: other,
                        SkillBranchId: 0,
                    }),
                    Spares: Vec::new(),
                })
                .collect();
            TeamRecommendRole {
                RoleId: rid,
                Teams: vec![TeamRecommendInfo {
                    SkillBranchId: 0,
                    Teammates: teammates,
                }],
            }
        })
        .collect();

    Ok(TeamRecommendResponse {
        TeamRecommendList: list,
    })
}

pub fn handle_item_exchange_info_request(
    state: &mut State,
    _req: ItemExchangeInfoRequest,
) -> Result<ItemExchangeInfoResponse, String> {
    let counts = &state.player_components.progress.info.shop.item_exchange_counts;
    let infos: Vec<ItemExchangeInfo> = if !state.assets.tables.item_exchange_content.items.is_empty()
    {
        state
            .assets
            .tables
            .item_exchange_content
            .items
            .iter()
            .filter(|cfg| cfg.ItemId > 0)
            .map(|cfg| {
                let used = counts
                    .iter()
                    .find(|c| c.id == cfg.ItemId)
                    .map_or(0, |c| c.count);
                ItemExchangeInfo {
                    ItemId: cfg.ItemId,
                    TodayTimes: used,
                    TotalTimes: used,
                    DailyLimit: cfg.Times.max(100),
                    TotalLimit: 0,
                }
            })
            .collect()
    } else {
        [3, 50001, 50002, 50005, 50007, 50008]
            .into_iter()
            .map(|item_id| {
                let used = counts
                    .iter()
                    .find(|c| c.id == item_id)
                    .map_or(0, |c| c.count);
                ItemExchangeInfo {
                    ItemId: item_id,
                    TodayTimes: used,
                    TotalTimes: used,
                    DailyLimit: 100,
                    TotalLimit: 0,
                }
            })
            .collect()
    };
    Ok(ItemExchangeInfoResponse {
        ItemExchangeInfos: infos,
    })
}

pub fn handle_pay_info_request(
    state: &mut State,
    req: PayInfoRequest,
) -> Result<PayInfoResponse, String> {
    let mut standard_items: Vec<_> = state
        .assets
        .tables
        .pay_item
        .items
        .iter()
        .filter(|item| (1..=6).contains(&item.Id))
        .collect();
    standard_items.sort_by_key(|item| item.Id);

    let infos = standard_items
        .into_iter()
        .map(|item| {
            let (prod, amt) =
                super::qol::resolve_pay_pricing(&state.assets, item.PayId, "com.kurogame.aki.pay");
            let first_bought = state
                .player_components
                .progress
                .info
                .get_key_int(&format!("payitem:{}", item.Id))
                > 0;
            PayItemInfo {
                Id: item.Id,
                PayId: item.PayId,
                ItemId: item.ItemId,
                ItemCount: item.ItemCount,
                BonusItemCount: item.BonusItemCount,
                SpecialBonusItemCount: item.SpecialBonusItemCount,
                CanSpecialBonus: item.SpecialBonusItemCount > 0 && !first_bought,
                StageImage: item.StageImage.as_bytes().to_vec(),
                ProductId: prod.into_bytes(),
                Amount: amt.into_bytes(),
                ..Default::default()
            }
        })
        .collect();
    Ok(PayInfoResponse {
        Version: if req.Version.is_empty() {
            b"3.8.11".to_vec()
        } else {
            req.Version
        },
        ErrorCode: Some(ErrorCode::Success),
        Infos: infos,
    })
}

fn is_btn_enabled(
    state: &State,
    eval_ctx: &sonoro_gamesv_logic::helpers::condition::EvalContext<'_>,
    bt_raw: i32,
) -> bool {
    if bt_raw > 0
        && let Some(fc) = state
            .assets
            .tables
            .function_condition
            .get_data_by_id(&bt_raw)
    {
        return fc.IsOn
            && (fc.OpenConditionId <= 0
                || condition::eval_condition_or_group(eval_ctx, fc.OpenConditionId));
    }
    true
}

pub fn handle_btn_state_request(
    state: &mut State,
    req: BtnStateRequest,
) -> Result<BtnStateResponse, String> {
    let eval_ctx = state.eval_context();
    let single_enabled = req
        .Type
        .map_or(true, |bt| is_btn_enabled(state, &eval_ctx, bt.to_i32()));
    let results = req
        .Types
        .into_iter()
        .map(|bt| {
            let en = is_btn_enabled(state, &eval_ctx, bt.to_i32());
            ButtonEnableResult {
                Type: Some(bt),
                Enabled: en,
            }
        })
        .collect();
    Ok(BtnStateResponse {
        ErrorCode: Some(ErrorCode::Success),
        Type: req.Type,
        Enabled: single_enabled,
        Result: results,
    })
}

fn resolve_level_play_exploratory_type(
    state: &State,
    mark_subtypes: &BTreeMap<i32, i32>,
    level_play_id: i32,
) -> i32 {
    if let Some(lp) = state
        .assets
        .tables
        .level_play_data
        .get_data_by_id(&level_play_id)
        && lp.Data.ExploratoryDegree > 0
    {
        return lp.Data.ExploratoryDegree;
    }
    *mark_subtypes.get(&level_play_id).unwrap_or(&0)
}

pub fn handle_level_play_state_request(
    state: &mut State,
    req: LevelPlayStateRequest,
) -> Result<LevelPlayStateResponse, String> {
    let mut mark_subtypes = BTreeMap::new();
    for mark in &state.assets.tables.map_mark.items {
        if mark.RelativeType == 1 && mark.RelativeId > 0 && mark.RelativeSubType > 0 {
            mark_subtypes.insert(mark.RelativeId, mark.RelativeSubType);
        }
    }
    let filter_inst = req.ArenaId > 0 || req.InstId > 0;
    let mut dict = BTreeMap::new();

    for &lpid in &state.player_components.quest.info.completed_level_plays {
        if filter_inst
            && let Some(lp) = state.assets.tables.level_play_data.get_data_by_id(&lpid)
            && ((req.ArenaId > 0 && lp.Data.LevelId != req.ArenaId)
                || (req.InstId > 0 && lp.Data.InstanceId != req.InstId))
        {
            continue;
        }
        dict.entry(lpid).or_insert_with(|| LevelPlayStateMsg {
            LevelPlayEntityId: lpid,
            ExploratoryType: resolve_level_play_exploratory_type(state, &mark_subtypes, lpid),
            StateType: 2,
            CompleteNumber: 1,
            IsUnlocked: true,
            LevelPlayMarkUnlock: true,
            ..Default::default()
        });
    }
    for c in &state.player_components.progress.info.dynamic.claims {
        let Some(rest) = c.strip_prefix("lp:open:") else {
            continue;
        };
        let Ok(lpid) = rest.parse::<i32>() else {
            continue;
        };
        if filter_inst
            && let Some(lp) = state.assets.tables.level_play_data.get_data_by_id(&lpid)
            && ((req.ArenaId > 0 && lp.Data.LevelId != req.ArenaId)
                || (req.InstId > 0 && lp.Data.InstanceId != req.InstId))
        {
            continue;
        }
        dict.entry(lpid).or_insert_with(|| LevelPlayStateMsg {
            LevelPlayEntityId: lpid,
            ExploratoryType: resolve_level_play_exploratory_type(state, &mark_subtypes, lpid),
            StateType: 1,
            IsUnlocked: true,
            LevelPlayMarkUnlock: true,
            ..Default::default()
        });
    }
    Ok(LevelPlayStateResponse {
        ErrorCode: Some(ErrorCode::Success),
        LevelPlayStateDict: dict,
    })
}

pub fn handle_sec_get_report_data2_flow_request(
    _state: &mut State,
    req: SecGetReportData2FlowRequest,
) -> Result<SecGetReportData2FlowResponse, String> {
    if req.ReportData.windows(6).any(|w| w == b"ZPROBE") {
        tracing::warn!(target: "zprobe", "ReportData ZPROBE: {}", String::from_utf8_lossy(&req.ReportData));
    }
    Ok(SecGetReportData2FlowResponse {
        Error: Some(DErrorResult {
            ErrorCode: Some(ErrorCode::Success),
            ErrorParams: Vec::new(),
        }),
    })
}

#[allow(dead_code)]
pub fn handle_player_game_token_request(
    state: &mut State,
    _req: PlayerGameTokenRequest,
) -> Result<PlayerGameTokenResponse, String> {
    let token = ensure_gm_sign_token(state)?;
    Ok(PlayerGameTokenResponse {
        GameToken: token.into_bytes(),
    })
}

pub fn handle_scene_block_split_player_need_block_push(
    _state: &mut State,
    _push: SceneBlockSplitPlayerNeedBlockPush,
) -> Result<(), String> {
    Ok(())
}

pub fn handle_black_screen_state_push(
    state: &mut State,
    push: BlackScreenStatePush,
) -> Result<(), String> {
    if let Some(scene) = state.scene.as_mut() {
        scene.client_black_screen_open = push.IsOpen;
        if push.IsOpen
            && scene.active_flow_inc_id == 0
            && push.BlackScreenReason.starts_with(b"SkipPlot_")
        {
            scene.client_black_screen_open = false;
            let _ = state.conn.push_msg(
                FlowServerSkipNotify::MSG_ID,
                &FlowServerSkipNotify {
                    FadeOutScreen: true,
                    ..Default::default()
                },
            );
        }
    }
    Ok(())
}

pub fn handle_advice_set_request(
    state: &mut State,
    req: AdviceSetRequest,
) -> Result<AdviceSetResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "advice:show",
        i64::from(req.IsShow),
    )
    .map_err(|e| e.to_string())?;
    Ok(AdviceSetResponse {
        IsShow: req.IsShow,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_entity_follow_track_request(
    state: &mut State,
    req: EntityFollowTrackRequest,
) -> Result<EntityFollowTrackResponse, String> {
    pb_kv::persist_claim(
        &state.fs,
        &mut state.player_components.progress,
        &format!("wp:follow_track:{}", req.EntityId),
    )
    .map_err(|e| e.to_string())?;
    let map_id = state
        .scene
        .as_ref()
        .map(|s| state.assets.resolve_map_config_id(s.instance_id))
        .unwrap_or(8);
    let cfg_id = state
        .scene
        .as_ref()
        .and_then(|s| s.find_config_id_by_net_id(req.EntityId))
        .or_else(|| i32::try_from(req.EntityId).ok())
        .unwrap_or(0);
    let mut foundation_id = 0i32;
    if cfg_id > 0
        && let Some(resolved) =
            sonoro_gamesv_logic::helpers::level_play::resolve_entity_config(
                &state.assets,
                map_id,
                i64::from(cfg_id),
            )
    {
        for entry in &resolved.components.extra {
            if let sonoro_gamesv_data::tables::entity_components::extra::ExtraEntry::FollowTrackComponent(
                ft,
            ) = entry
            {
                foundation_id = ft
                    .EndType
                    .as_ref()
                    .and_then(|v| v.get("FoundationId"))
                    .and_then(|v| v.as_i64())
                    .and_then(|v| i32::try_from(v).ok())
                    .unwrap_or(0);
                break;
            }
        }
    }
    if cfg_id > 0 || foundation_id > 0 {
        let mut events = EventQueue::new();
        {
            let mut qctx = state.quest_context(&mut events);
            let cbs = quest_engine::action_callbacks();
            if cfg_id > 0 {
                let _ = sonoro_gamesv_logic::helpers::quest_actions::apply_entity_state_change(
                    &mut qctx,
                    &cbs,
                    cfg_id,
                    "关卡.Common.状态.完成",
                );
            }
            if foundation_id > 0 {
                let _ = sonoro_gamesv_logic::helpers::quest_actions::apply_entity_state_change(
                    &mut qctx,
                    &cbs,
                    foundation_id,
                    "关卡.Common.状态.完成",
                );
            }
            let _ = quest_engine::on_condition_state_changed(&mut qctx);
        }
        drain_event_queue_best_effort(&mut events, state);
    }
    let reward_target = if foundation_id > 0 { foundation_id } else { cfg_id };
    if reward_target > 0
        && !state
            .player_components
            .progress
            .info
            .has_claim(&format!("follow_track:rw:{reward_target}"))
        && let Some(resolved) =
            sonoro_gamesv_logic::helpers::level_play::resolve_entity_config(
                &state.assets,
                map_id,
                i64::from(reward_target),
            )
        && let Some(rc) = &resolved.components.RewardComponent
        && let Some(reward_id) = rc.RewardId
        && reward_id > 0
    {
        state
            .player_components
            .progress
            .info
            .mark_claim(&format!("follow_track:rw:{reward_target}"));
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        let mut rctx = state.reward_context();
        let _ = reward::grant_drop_package(&mut rctx, reward_id, 1);
    }
    Ok(EntityFollowTrackResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_entity_on_landed_request(
    _state: &mut State,
    _req: EntityOnLandedRequest,
) -> Result<EntityOnLandedResponse, String> {
    Ok(EntityOnLandedResponse::default())
}

pub fn handle_combat_message_post_info_request(
    _state: &mut State,
    _req: CombatMessagePostInfoRequest,
) -> Result<CombatMessagePostInfoResponse, String> {
    Ok(CombatMessagePostInfoResponse::default())
}

pub fn handle_role_operate_self_bgm_request(
    state: &mut State,
    req: RoleOperateSelfBgmRequest,
) -> Result<RoleOperateSelfBgmResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        &format!("role_bgm:{}", req.RoleId),
        i64::from(req.IsOpen),
    )
    .map_err(|e| e.to_string())?;
    Ok(RoleOperateSelfBgmResponse {
        ErrorCode: Some(ErrorCode::Success),
        RoleId: req.RoleId,
        IsOpen: req.IsOpen,
    })
}

pub fn handle_cd_key_verify_request(
    state: &mut State,
    req: CdKeyVerifyRequest,
) -> Result<CdKeyVerifyResponse, String> {
    let raw = String::from_utf8_lossy(&req.CdKey).trim().to_string();
    if raw.len() < 4 {
        return Ok(CdKeyVerifyResponse {
            ErrorCode: Some(ErrorCode::ErrCdKeyLength),
        });
    }
    let normalized = raw.to_ascii_uppercase();
    static CDKEY_DROPS: OnceLock<HashMap<String, i32>> = OnceLock::new();
    let table_keys = CDKEY_DROPS.get_or_init(|| {
        let mut out = HashMap::new();
        for name in ["CdKey", "GiftCode"] {
            for row in state.assets.read_raw_json_array(name) {
                if let Some(code) = row
                    .get("Code")
                    .or_else(|| row.get("CdKey"))
                    .or_else(|| row.get("Key"))
                    .and_then(Value::as_str)
                {
                    let drop_id = row
                        .get("DropId")
                        .or_else(|| row.get("RewardId"))
                        .and_then(Value::as_i64)
                        .unwrap_or(0) as i32;
                    out.insert(code.trim().to_ascii_uppercase(), drop_id);
                }
            }
        }
        out
    });
    let table_drop = table_keys.get(&normalized).copied();
    let builtin_valid = matches!(
        normalized.as_str(),
        "WUTHERINGGIFT" | "SOLARIS3811" | "WW3811"
    );
    if table_drop.is_none() && !builtin_valid {
        return Ok(CdKeyVerifyResponse {
            ErrorCode: Some(ErrorCode::ErrCdKeyNotFound),
        });
    }
    let prog = &state.player_components.progress.info;
    if prog.has_claimed_cdkey(&normalized) || prog.has_claimed_cdkey(&raw) {
        return Ok(CdKeyVerifyResponse {
            ErrorCode: Some(ErrorCode::ErrCdKeyEachPlayerMaxCount),
        });
    }
    state
        .player_components
        .progress
        .info
        .claim_cdkey(&normalized);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    let drop_id = table_drop.unwrap_or(0);
    let mut granted = false;
    if drop_id > 0 {
        let mut rctx = state.reward_context();
        let res = reward::grant_drop_package_popup(&mut rctx, drop_id, 2)
            .map_err(|e| e.to_string())?;
        granted = res.granted_count > 0;
    }
    if !granted {
        let mut rctx = state.reward_context();
        reward::grant_items(
            &mut rctx,
            0,
            true,
            &[
                RewardItem {
                    item_id: 3,
                    count: 300,
                },
                RewardItem {
                    item_id: 50002,
                    count: 2,
                },
                RewardItem {
                    item_id: 2,
                    count: 50000,
                },
            ],
            2,
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(CdKeyVerifyResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_drown_request(
    state: &mut State,
    _req: DrownRequest,
) -> Result<DrownResponse, String> {
    state
        .player_components
        .progress
        .info
        .add_key_int("player:drown_cnt", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(DrownResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_drown_end_teleport_request(
    state: &mut State,
    _req: DrownEndTeleportRequest,
) -> Result<DrownEndTeleportResponse, String> {
    let mut events = EventQueue::new();
    let mut teleport_target: Option<(i32, [f32; 3], [f32; 3])> = None;
    if let Some(scene) = state.scene.as_mut() {
        role_helper::revive_and_heal_formation(
            &mut state.conn,
            scene,
            &mut state.player_components.role,
            &mut events,
            &state.fs,
        );
        let inst_id = scene.instance_id;
        let map_id = state.assets.resolve_map_config_id(inst_id);
        let fallback_rot = scene
            .instance
            .players
            .first()
            .map_or([0.0, 0.0, 0.0], |p| p.rotation);
        let pos_opt = if scene.has_quest_move_loc
            && scene.last_quest_move_loc.iter().all(|v| v.is_finite())
        {
            Some(scene.last_quest_move_loc)
        } else if let Some(inst) = state.assets.tables.instance_dungeon.get_data_by_id(&inst_id)
            && inst.BornPosition.iter().any(|&v| v != 0)
        {
            Some([
                inst.BornPosition[0] as f32,
                inst.BornPosition[1] as f32,
                inst.BornPosition[2] as f32,
            ])
        } else {
            scene
                .instance
                .players
                .first()
                .map(|p| p.location)
                .filter(|loc| loc.iter().all(|v| v.is_finite()))
        };
        if let Some(pos) = pos_opt
            && map_id > 0
        {
            teleport_target = Some((map_id, pos, fallback_rot));
        }
    }
    if let Some((map_id, pos, _rot)) = teleport_target {
        {
            let mut qctx = state.quest_context(&mut events);
            let _ = quest_actions::update_player_scene_state(&mut qctx, pos, None);
        }
        let _ = state.conn.push_msg(
            TeleportNotify::MSG_ID,
            &TeleportNotify {
                MapId: map_id,
                Pos: Some(Vector {
                    X: pos[0],
                    Y: pos[1],
                    Z: pos[2],
                }),
                Reason: Some(TeleportReason::Drown),
                ..Default::default()
            },
        );
    }
    drain_event_queue_best_effort(&mut events, state);
    Ok(DrownEndTeleportResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_enter_view_direction_request(
    state: &mut State,
    _req: EnterViewDirectionRequest,
) -> Result<EnterViewDirectionResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "player:view_dir",
        1,
    )
    .map_err(|e| e.to_string())?;
    Ok(EnterViewDirectionResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_exit_view_direction_request(
    state: &mut State,
    _req: ExitViewDirectionRequest,
) -> Result<ExitViewDirectionResponse, String> {
    pb_kv::persist_int(
        &state.fs,
        &mut state.player_components.progress,
        "player:view_dir",
        0,
    )
    .map_err(|e| e.to_string())?;
    Ok(ExitViewDirectionResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_client_current_role_report_request(
    _state: &mut State,
    req: ClientCurrentRoleReportRequest,
) -> Result<ClientCurrentRoleReportResponse, String> {
    Ok(ClientCurrentRoleReportResponse {
        ErrorCode: Some(ErrorCode::Success),
        CurrentEntityId: req.CurrentEntityId,
        PlayerId: req.PlayerId,
    })
}

pub fn handle_character_death_push(
    state: &mut State,
    push: CharacterDeathPush,
) -> Result<(), String> {
    state
        .player_components
        .progress
        .info
        .add_key_int("combat:char_death_cnt", 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    let mut events = EventQueue::new();
    if let Some(scene) = state.scene.as_mut() {
        let mut target_nid = scene.current_role_entity_id();
        if push.RoleId > 0 {
            for ent in &scene.entities {
                if ent.config.entity_type == EntityType::Player
                    && ent.config.config_type == ConfigType::Character
                    && ent.config.config_id == push.RoleId
                {
                    target_nid = ent.net_id;
                    break;
                }
            }
        }
        if let Some(&idx) = scene.net_id_map.get(&target_nid) {
            if scene.entities[idx].dead_notified {
                return Ok(());
            }
            scene.entities[idx].dead_notified = true;
            let cfg_id = scene.entities[idx].config.config_id;
            let is_char =
                scene.entities[idx].config.config_type == ConfigType::Character;
            let mut cpack = Vec::new();
            if let Some(attr) = scene.entities[idx].attribute.as_mut() {
                let _ = attributes_helper::set_life_unconditional(attr, 0);
                attributes_helper::generate_attr_messages(
                    &mut cpack,
                    target_nid,
                    attr,
                    &[EAttributeType::Life],
                );
            }
            scene.set_battle_entity_active(target_nid, false);
            cpack.push(CombatReceiveData {
                Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
                    sonoro_proto::pb::CombatNotifyData {
                        CombatCommon: Some(sonoro_proto::pb::CombatCommon {
                            EntityId: target_nid,
                            ..Default::default()
                        }),
                        Message: Some(
                            sonoro_proto::pb::CombatNotifyDataMessageOneof::EntityLivingStatusNotify(
                                EntityLivingStatusNotify {
                                    Id: target_nid,
                                    LivingStatus: Some(LivingStatus::Dead),
                                    DropVisionItem: Vec::new(),
                                },
                            ),
                        ),
                    },
                )),
            });
            if !cpack.is_empty() {
                let _ = state.conn.push_msg(
                    CombatReceivePackNotify::MSG_ID,
                    &CombatReceivePackNotify { Data: cpack },
                );
            }
            if is_char && let Some(role_info) = state.player_components.role.role_map.get_mut(&cfg_id) {
                role_info.current_hp = Some(0);
                events.enqueue(Event::RoleInfoModified {
                    player_id: state.player_id,
                    role_id: cfg_id,
                });
                let _ = role_helper::save_role_info(&state.fs, &state.player_components.role, cfg_id);
            }
            events.enqueue(Event::UpdateFormations {
                player_id: state.player_id,
                formation_id: scene.formation_info.cur_formation + 1,
                role_ids: Vec::new(),
            });
            role_helper::maybe_push_party_wipe_notify(&mut state.conn, scene);
            scene.mark_fsm_dirty(target_nid, wake_reason::ATTRIBUTE);
        }
    }
    {
        let mut qctx = state.quest_context(&mut events);
        let _ = quest_engine::on_condition_state_changed(&mut qctx);
    }
    drain_event_queue_best_effort(&mut events, state);
    Ok(())
}

pub fn handle_player_net_state_push(
    state: &mut State,
    _push: PlayerNetStatePush,
) -> Result<(), String> {
    let uid = i64::from(state.player_id);
    let _ = state.conn.push_msg(
        PlayerNetStateS2cNotify::MSG_ID,
        &PlayerNetStateS2cNotify { Uid: uid },
    );
    Ok(())
}

pub fn handle_robot_destination_push(
    _state: &mut State,
    _push: RobotDestinationPush,
) -> Result<(), String> {
    Ok(())
}

pub fn handle_net_test_data_push(
    _state: &mut State,
    _push: NetTestDataPush,
) -> Result<(), String> {
    Ok(())
}

pub fn handle_max_message_id_push(
    _state: &mut State,
    _push: MaxMessageIdPush,
) -> Result<(), String> {
    Ok(())
}
