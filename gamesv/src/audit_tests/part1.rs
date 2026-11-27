use std::collections::BTreeMap;
use std::fs;
use std::sync::Arc;

use serde_json::Value;
use sonoro_gamesv_data::tables::entity_components::components::Components;
use sonoro_gamesv_data::tables::entity_components::range::{ShapeConfig, Vec3};
use sonoro_gamesv_fs::echo_info;
use sonoro_gamesv_fs::quest_info::append_unique_int;
use sonoro_gamesv_fs::scene_instance::ScenePlayer;
use sonoro_gamesv_logic::component::entity::EntityComponentStorage;
use sonoro_gamesv_logic::component::entity::small::{
    ConfigComponent, ConfigType, EntityState, EntityType, PositionComponent,
};
use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::handlers::misc as misc_logic;
use sonoro_gamesv_logic::helpers::condition::{
    eval_single_inline_condition, is_pos_inside_range_shape,
};
use sonoro_gamesv_logic::helpers::quest_actions::execute_action_list;
use sonoro_gamesv_logic::helpers::quest_types::{ActionCallbacks, resolve_var_int};
use sonoro_gamesv_logic::helpers::reward::{RewardItem, grant_items};
use sonoro_gamesv_logic::helpers::{gravity, level_play, morale, quest_engine, role};
use sonoro_gamesv_logic::scene::Scene;
use sonoro_gamesv_logic::schedulers::quest_timer_queue::QuestTimerEntry;
use sonoro_proto::enums::{ActivityType, BtType, ErrorCode, FailReason, JokerGuessSkillOp};
use sonoro_proto::pb::{
    AcceptEntrustRequest, BossRushChooseBuffRequest, BtGiveUpRequest, BtRollbackRequest,
    ConsumptiveRewardRequest, DollDeliveryRequest, DollSmallMapInfoRequest,
    DrownEndTeleportPush, DrownEndTeleportPushYcuOneof, EntityDynamicInteractRequest,
    EntityInteractRequest, FloroRanchEventChoiceRequest, FloroRanchExecuteSkillPlayRequest,
    FloroRanchPlayNextDayRequest, FloroRanchPlayRemoveUnitRequest, FloroRanchPlayTributeRequest,
    FloroRanchPlayUnlimitedModeRequest, FlowEndRequest, FlowOptionInfo, FlowOptionInfoList,
    FoundationCoreOperateRequest, GetLevelPlayRewardsRequest,
    GravityHookLockPointChangeGravityRequest, HandInMingSuRequest, HonamiStoryActivateTalentRequest,
    HonamiStoryActivityInfoRequest, HonamiStoryWeaponDressRequest, InstEntranceDetailRequest,
    InvestRequest, JokerGuessDrawCardRequest, JokerGuessPlayCardRequest,
    JokerGuessStartNewRequest, JokerGuessUseSkillRequest, LoopTeleportVolumeActionRequest,
    MotorDecalTaskRequest, MotorLinkTaskRequest, NewPlayerSupportSetCurUseTrialRoleRequest,
    NewPlayerSupportSetCurUseTrialRoleV2Request, PhantomBattleCardRoleTargetInfoRequest,
    PhantomBattleCardTargetInfoRequest, PhantomConsumeItem, PhantomLevelUpRequest,
    RoguelikeChooseDataResultRequest, SimpleTrackReportRequest, SolarSchoolBatchSyncRequest,
    SolarSchoolSaveRequest, SolarSchoolSetFlowRequest, StartRecallRequest, Vector,
};

use super::TestHarness;
use crate::network::handlers::{
    activity, combat, combat_envelope_handlers, combat_sync, direct_train, echo, endgame,
    events_major, events_misc, flow, inst, inventory, minimal, motor, phantom_battle, quest,
    region_prog, roguelike, scene as scene_handler, world_puzzle,
};

#[test]
fn audit_91_gsv25_to_45_parity_and_regression_suite() {
    let mut h = TestHarness::new("audit91", Some(8));
    h.state.scene.as_mut().unwrap().instance.players = vec![ScenePlayer {
        id: 1,
        location: [0.0, 0.0, 0.0],
        rotation: [0.0, 0.0, 0.0],
        group_type: 0,
    }];

    // GSV-45: Echo stack_count + split + partial fodder consume
    let first_echo_cfg = h
        .state
        .assets
        .tables
        .phantom_item
        .items
        .iter()
        .find(|it| {
            echo_info::valid_default_owned_item(&h.state.assets, it)
                && it.QualityId >= 5
                && !it.FetterGroup.is_empty()
        })
        .expect("find echo item cfg")
        .clone();
    let mut stacked = echo_info::build_default_echo(
        &h.state.assets,
        &first_echo_cfg,
        first_echo_cfg.FetterGroup[0],
    );
    stacked.stack_count = 5;
    h.state.player_components.echo.echo_map.insert(1001, stacked.clone());
    assert_eq!(stacked.to_proto(1001).StackCount, 5);

    let split_info = echo::shared::split_single_from_stack(
        &h.state.fs,
        &mut h.state.player_components.echo,
        1001,
    )
    .expect("split ok")
    .expect("split some");
    assert_eq!(split_info.OperateBeforeCount, 5);
    assert_eq!(split_info.OperateItem.unwrap().StackCount, 1);
    let rem_item = split_info.RemainderItem.unwrap();
    assert_eq!(rem_item.StackCount, 4);
    assert_ne!(rem_item.IncrId, 1001);

    let mut fodder = echo_info::build_default_echo(
        &h.state.assets,
        &first_echo_cfg,
        first_echo_cfg.FetterGroup[0],
    );
    fodder.level = 5;
    fodder.stack_count = 4;
    h.state.player_components.echo.echo_map.insert(1002, fodder);

    let mut events = EventQueue::new();
    let lvl_resp = echo::upgrade::handle_phantom_level_up_request(
        &mut h.state,
        &mut events,
        PhantomLevelUpRequest {
            IncId: 1001,
            ConsumeList: vec![PhantomConsumeItem {
                IncId: 1002,
                Count: 2,
                ..Default::default()
            }],
            ..Default::default()
        },
    )
    .expect("phantom lvl up");
    assert_eq!(lvl_resp.ErrorCode, Some(ErrorCode::Success));
    assert_eq!(h.state.player_components.echo.echo_map.get(&1002).unwrap().stack_count, 2);
    assert!(h.drain_count() > 0);

    // GSV-44: DrownEndTeleportPush pushes TeleportNotify
    combat_envelope_handlers::handle_drown_end_teleport_push(
        &mut h.state,
        &mut events,
        DrownEndTeleportPush {
            ycu: Some(DrownEndTeleportPushYcuOneof::TeleportPos(Vector {
                X: 111.0,
                Y: 222.0,
                Z: 333.0,
            })),
        },
    );
    assert_eq!(h.drain_count(), 1);
    assert!((h.state.scene.as_ref().unwrap().instance.players[0].location[0] - 111.0).abs() < 0.01);

    // GSV-25: LoopTeleportVolumeActionRequest
    let ltp_resp = world_puzzle::handle_loop_teleport_volume_action_request(
        &mut h.state,
        LoopTeleportVolumeActionRequest { VolumeId: 10001 },
    )
    .expect("ltp");
    assert_eq!(ltp_resp.ErrorCode, Some(ErrorCode::Success));
    assert!(!ltp_resp.ConditionMetIds.is_empty());

    // GSV-26 & GSV-27: SolarSchool
    let sb_resp = region_prog::handle_solar_school_batch_sync_request(
        &mut h.state,
        SolarSchoolBatchSyncRequest {
            ActivityId: 1,
            GameDayNow: 3,
            ..Default::default()
        },
    )
    .expect("solar batch");
    assert_eq!(sb_resp.FailedIndex, -1);
    let _ = region_prog::handle_solar_school_set_flow_request(
        &mut h.state,
        SolarSchoolSetFlowRequest {
            SolarSchoolFlowId: 77,
            ReadPlot: vec![5],
        },
    )
    .expect("solar set flow");
    let ss_resp = region_prog::handle_solar_school_save_request(
        &mut h.state,
        SolarSchoolSaveRequest {
            ActivityId: 1,
            LoadSourceSlotId: 0,
            ..Default::default()
        },
    )
    .expect("solar save");
    assert!(!ss_resp.Flow.is_empty());

    // GSV-28: FoundationCoreOperateRequest
    let fc_take = events_misc::handle_foundation_core_operate_request(
        &mut h.state,
        FoundationCoreOperateRequest {
            FoundationEntityIncId: 999_999,
            IsTakeOut: true,
        },
    )
    .expect("fc take");
    assert_ne!(fc_take.FromFoundationEntityId, 0);
    let core_info = fc_take.FoundationCoreInfo.expect("core info");
    assert_ne!(core_info.SourceEntityId, 0);
    assert_ne!(core_info.ElementalBoxId, 0);

    let fc_put = events_misc::handle_foundation_core_operate_request(
        &mut h.state,
        FoundationCoreOperateRequest {
            FoundationEntityIncId: 999_999,
            IsTakeOut: false,
        },
    )
    .expect("fc put");
    assert!(fc_put.FoundationCoreInfo.is_none());

    // GSV-29: StartRecallRequest
    let rec_resp = region_prog::handle_start_recall_request(
        &mut h.state,
        StartRecallRequest {
            RecallId: 110_080,
            Continue: false,
        },
    )
    .expect("recall");
    assert_eq!(rec_resp.EnterInstId, 20_130_008);

    // GSV-30: RoguelikeChooseDataResultRequest
    let rg_resp = roguelike::handle_roguelike_choose_data_result_request(
        &mut h.state,
        RoguelikeChooseDataResultRequest {
            Layer: 1,
            BindId: -2,
            Index: 0,
        },
    )
    .expect("roguelike");
    assert!(rg_resp.RoguelikeChooseDataNotify.is_some());

    // GSV-31: FloroRanch 6 handlers
    assert!(!events_major::handle_floro_ranch_play_next_day_request(&mut h.state, FloroRanchPlayNextDayRequest { SubInsId: 1, ..Default::default() }).unwrap().Task.is_empty());
    assert!(!events_major::handle_floro_ranch_play_tribute_request(&mut h.state, FloroRanchPlayTributeRequest { SubInsId: 1, ..Default::default() }).unwrap().Task.is_empty());
    assert!(!events_major::handle_floro_ranch_play_unlimited_mode_request(&mut h.state, FloroRanchPlayUnlimitedModeRequest { SubInsId: 1, ..Default::default() }).unwrap().Task.is_empty());
    assert!(!events_major::handle_floro_ranch_play_remove_unit_request(&mut h.state, FloroRanchPlayRemoveUnitRequest { SubInsId: 1, UnitIncId: 2, ..Default::default() }).unwrap().Task.is_empty());
    assert!(!events_major::handle_floro_ranch_event_choice_request(&mut h.state, FloroRanchEventChoiceRequest { SubInsId: 1, IncId: 2, Choice: 1, ..Default::default() }).unwrap().Task.is_empty());
    assert!(!events_major::handle_floro_ranch_execute_skill_play_request(&mut h.state, FloroRanchExecuteSkillPlayRequest { SubInsId: 1, IncId: 2, ..Default::default() }).unwrap().Task.is_empty());

    // GSV-32: GuessJoker 4 handlers
    assert!(!events_major::handle_joker_guess_start_new_request(&mut h.state, JokerGuessStartNewRequest { LevelId: 1, ..Default::default() }).unwrap().Cmd.is_empty());
    assert!(!events_major::handle_joker_guess_draw_card_request(&mut h.state, JokerGuessDrawCardRequest { TargetCardId: 11 }).unwrap().Cmd.is_empty());
    assert!(!events_major::handle_joker_guess_play_card_request(&mut h.state, JokerGuessPlayCardRequest::default()).unwrap().Cmd.is_empty());
    assert!(!events_major::handle_joker_guess_use_skill_request(&mut h.state, JokerGuessUseSkillRequest { SkillId: 1, Op: Some(JokerGuessSkillOp::Use) }).unwrap().Cmd.is_empty());

    // GSV-33, GSV-35, GSV-36
    let (lp_id, chest_id) = h
        .state
        .assets
        .tables
        .level_play_data
        .items
        .iter()
        .find_map(|lp| {
            lp.Data
                .LevelPlayRewardConfig
                .as_ref()
                .filter(|rc| rc.RewardEntityId > 0)
                .map(|rc| (lp.LevelPlayId, rc.RewardEntityId))
        })
        .expect("find level_play chest");
    let _ = h.state.player_components.quest.info.mark_chest_opened(chest_id);
    h.state.player_components.quest.info.mark_level_play_completed(lp_id);

    let lp_rw = inst::handle_get_level_play_rewards_request(
        &mut h.state,
        GetLevelPlayRewardsRequest {
            LevelPlayId: lp_id,
            ..Default::default()
        },
    )
    .expect("lp rewards");
    assert!(!lp_rw.TreasureBoxIds.is_empty());

    let st_rep = combat_sync::handle_simple_track_report_request(
        &mut h.state,
        SimpleTrackReportRequest::default(),
    )
    .expect("simple track");
    assert!(!st_rep.SimpleTrackReportMsgs.is_empty());

    let ie_resp = world_puzzle::handle_inst_entrance_detail_request(
        &mut h.state,
        InstEntranceDetailRequest { EntranceId: 14 },
    )
    .expect("inst entrance detail");
    assert!(!ie_resp.InstDetailEntryList.is_empty());

    // GSV-34: BossRushChooseBuffRequest
    let br_resp = endgame::handle_boss_rush_choose_buff_request(
        &mut h.state,
        BossRushChooseBuffRequest::default(),
    )
    .expect("boss rush choose");
    assert!(!br_resp.BuffIds.is_empty());

    // GSV-37: Per-showcase DollDelivery + DollSmallMapInfo
    let _ = events_major::handle_doll_delivery_request(
        &mut h.state,
        DollDeliveryRequest { EntityCfgId: 90001 },
    )
    .expect("doll delivery");
    let dm_resp = minimal::handle_doll_small_map_info_request(
        &mut h.state,
        DollSmallMapInfoRequest::default(),
    )
    .expect("doll map");
    assert!(!dm_resp.SceneDollDeliveryInfos.is_empty());
    assert_eq!(
        dm_resp.SceneDollDeliveryInfos[0].DollDeliveryInfos[0]
            .DeliveredDolls
            .len(),
        1
    );

    // GSV-17: HonamiStoryActivityInfo
    let _ = events_major::handle_honami_story_activate_talent_request(
        &mut h.state,
        HonamiStoryActivateTalentRequest { TalentId: 501 },
    )
    .expect("honami talent");
    let _ = events_major::handle_honami_story_weapon_dress_request(
        &mut h.state,
        HonamiStoryWeaponDressRequest {
            ActivityId: 1,
            RackIndex: 0,
            WeaponItemId: 9901,
        },
    )
    .expect("honami weapon");
    let hi_resp = events_major::handle_honami_story_activity_info_request(
        &mut h.state,
        HonamiStoryActivityInfoRequest { ActivityId: 1 },
    )
    .expect("honami info");
    let info = hi_resp.HonamiStoryActivityInfo.expect("honami activity info");
    assert_eq!(info.ActivatedTalentId.len(), 1);
    assert_eq!(info.HonamiStoryPlayerBagInfo.unwrap().UnlockedWeaponIds.len(), 1);

    // GSV-38: PhantomBattle card EffectType >= 1
    let pb1 = phantom_battle::handle_phantom_battle_card_target_info_request(
        &mut h.state,
        PhantomBattleCardTargetInfoRequest {
            CardUId: 1,
            SkillId: 10,
        },
    )
    .expect("pb1");
    assert!(pb1.EffectType >= 1);
    let pb2 = phantom_battle::handle_phantom_battle_card_role_target_info_request(
        &mut h.state,
        PhantomBattleCardRoleTargetInfoRequest {
            SkillId: 10,
        },
    )
    .expect("pb2");
    assert!(pb2.EffectType >= 1);

    // GSV-39: ConsumptiveRewardRequest
    let cons = events_misc::handle_consumptive_reward_request(
        &mut h.state,
        ConsumptiveRewardRequest {
            ActivityId: 1,
            TaskIds: vec![1],
        },
    )
    .expect("consumptive");
    assert_eq!(cons.TaskInfos.len(), 1);
    h.drain_count();

    // GSV-40: MotorDecalTaskRequest & MotorLinkTaskRequest
    let md = motor::handle_motor_decal_task_request(
        &mut h.state,
        MotorDecalTaskRequest {
            MotorIp: 1,
        },
    )
    .expect("motor decal");
    assert!(!md.Ids.is_empty());
    let ml = motor::handle_motor_link_task_request(
        &mut h.state,
        MotorLinkTaskRequest {
            MotorIp: 101,
        },
    )
    .expect("motor link");
    assert!(!ml.Ids.is_empty());
    h.drain_count();

    // GSV-41 & GSV-42: AcceptEntrust + Invest coin deduction
    let ae = world_puzzle::handle_accept_entrust_request(
        &mut h.state,
        AcceptEntrustRequest {
            EntrustId: 1,
            RoleIds: vec![1],
        },
    )
    .expect("accept entrust");
    assert_eq!(ae.RoleProperSettle.len(), 1);
    assert_eq!(ae.RoleGoods.len(), 1);
    assert!(ae.Gold > 0);

    let inv_fail = events_misc::handle_invest_request(
        &mut h.state,
        InvestRequest {
            EntrustId: 1,
            Amount: 50,
        },
    )
    .expect("invest fail");
    assert_eq!(inv_fail.ErrorCode, Some(ErrorCode::ErrItemNotEnough));

    h.state.player_components.inventory.info.add_normal_item(15, 100);
    let inv_ok = events_misc::handle_invest_request(
        &mut h.state,
        InvestRequest {
            EntrustId: 1,
            Amount: 50,
        },
    )
    .expect("invest ok");
    assert_eq!(inv_ok.ErrorCode, Some(ErrorCode::Success));
    assert_eq!(h.state.player_components.inventory.info.normal_item_count(15), 50);
    assert!(!inv_ok.RoleProperSettle.is_empty());
    h.drain_count();

    // GSV-43: NewPlayerSupportSetCurUseTrialRole & V2
    let trial_id = h
        .state
        .assets
        .tables
        .trial_role_info
        .items
        .first()
        .map_or(0, |r| r.Id);
    let tr1 = region_prog::handle_new_player_support_set_cur_use_trial_role_request(
        &mut h.state,
        NewPlayerSupportSetCurUseTrialRoleRequest {
            TrialRoleId: trial_id,
        },
    )
    .expect("trial role v1");
    assert!(tr1.CurUseRoleInfo.is_some());
    let tr2 = region_prog::handle_new_player_support_set_cur_use_trial_role_v2_request(
        &mut h.state,
        NewPlayerSupportSetCurUseTrialRoleV2Request {
            TrialRoleId: trial_id,
        },
    )
    .expect("trial role v2");
    assert!(tr2.CurUseRoleInfo.is_some());
    h.drain_count();
}

#[test]
fn audit_91b_gsv18_and_gsv49_failure_retention_and_retry_safety() {
    let mut h = TestHarness::new("audit91b", Some(8));
    let mut events = EventQueue::new();

    // GSV-18: failed Scene.remove preserves net_id in pending_killed_removals until retry succeeds.
    let ent = h
        .state
        .scene
        .as_mut()
        .unwrap()
        .spawn_with_net_id(
            777_001,
            EntityComponentStorage::new(
                777_001,
                ConfigComponent {
                    camp: 0,
                    config_id: 1,
                    config_type: ConfigType::Level,
                    entity_type: EntityType::Monster,
                    state: EntityState::Default,
                },
            ),
        )
        .expect("spawn");
    h.state
        .scene
        .as_mut()
        .unwrap()
        .queue_killed_monster_removal_at(ent.net_id, 0);

    let entity_block_path = h.tmp.path.join("player/1/scene/8/entity");
    let _ = fs::remove_dir_all(&entity_block_path);
    fs::create_dir_all(entity_block_path.parent().unwrap()).unwrap();
    fs::write(&entity_block_path, b"block").unwrap();

    let _ = combat::handle_combat_send_pack_request(&mut h.state, &mut events, Default::default())
        .expect("combat send pack");
    assert_eq!(h.state.scene.as_ref().unwrap().pending_killed_removals.len(), 1);
    assert!(h.state.scene.as_ref().unwrap().net_id_map.contains_key(&ent.net_id));

    fs::remove_file(&entity_block_path).unwrap();
    let _ = combat::handle_combat_send_pack_request(&mut h.state, &mut events, Default::default())
        .expect("combat send pack retry");
    assert_eq!(h.state.scene.as_ref().unwrap().pending_killed_removals.len(), 0);
    assert!(!h.state.scene.as_ref().unwrap().net_id_map.contains_key(&ent.net_id));
    assert!(h.drain_count() > 0);

    // GSV-49: failed chest grant must NOT mark chest opened, allowing subsequent claim retry.
    let chest_cfg_id = h
        .state
        .assets
        .indexes
        .entity_by_map_pair
        .iter()
        .find_map(|(k, &idx)| {
            if k.map_id != 8 {
                return None;
            }
            let cfg = h.state.assets.get_level_entity_by_idx(idx)?;
            let resolved = level_play::resolve_entity_config(&h.state.assets, 8, cfg.EntityId)?;
            resolved.components.TreasureBoxComponent.as_ref()?;
            let rc = resolved.components.RewardComponent.as_ref()?;
            let rid = rc.RewardId?;
            if rid <= 0 {
                return None;
            }
            let pkg = h.state.assets.tables.drop_package.get_data_by_id(&rid)?;
            if pkg.DropPreview.is_empty() {
                return None;
            }
            let ok_item = pkg.DropPreview.keys().any(|iid| {
                h.state.assets.tables.weapon_conf.get_data_by_id(iid).is_none()
                    && h.state.assets.tables.phantom_item.get_data_by_id(iid).is_none()
            });
            ok_item.then(|| i32::try_from(cfg.EntityId).ok()).flatten()
        })
        .expect("find chest cfg id");

    let chest_ent = h
        .state
        .scene
        .as_mut()
        .unwrap()
        .spawn_with_net_id(
            777_002,
            EntityComponentStorage::new(
                777_002,
                ConfigComponent {
                    camp: 0,
                    config_id: chest_cfg_id,
                    config_type: ConfigType::Level,
                    entity_type: EntityType::SceneItem,
                    state: EntityState::Default,
                },
            ),
        )
        .expect("spawn chest");

    let inv_dir = h.tmp.path.join("player/1/inventory_info");
    let basic_dir = h.tmp.path.join("player/1/basic_info");
    let _ = fs::remove_file(&inv_dir);
    let _ = fs::remove_file(&basic_dir);
    fs::create_dir_all(&inv_dir).unwrap();
    fs::create_dir_all(&basic_dir).unwrap();

    let mut qctx = h.state.quest_context(&mut events);
    let mut lctx = level_play::LevelPlayContext::from_quest_ctx(&mut qctx).unwrap();
    let err = level_play::on_get_reward_treasure_box(&mut lctx, chest_ent.net_id);
    assert!(err.is_err());
    assert!(!h.state.player_components.quest.info.has_opened_chest(chest_cfg_id));

    fs::remove_dir_all(&inv_dir).unwrap();
    fs::remove_dir_all(&basic_dir).unwrap();

    let mut qctx2 = h.state.quest_context(&mut events);
    let mut lctx2 = level_play::LevelPlayContext::from_quest_ctx(&mut qctx2).unwrap();
    level_play::on_get_reward_treasure_box(&mut lctx2, chest_ent.net_id).expect("retry chest");
    assert!(h.state.player_components.quest.info.has_opened_chest(chest_cfg_id));
    assert!(h.drain_count() > 0);
}

#[test]
fn audit_91c_gameplay129_disabled_null_template_merge_and_interact() {
    let tpl: Components = serde_json::from_str(
        r#"{"InteractComponent":{"Disabled":true,"Options":[{"Type":{"Type":"Actions","Actions":[{"Name":"ChangeSelfEntityState","Params":{"EntityState":"关卡.Common.状态.激活"}}]}}]}}"#,
    )
    .unwrap();
    let mut cleared: Components = serde_json::from_str(
        r#"{"InteractComponent":{"Disabled":null}}"#,
    )
    .unwrap();
    tpl.merge_into(&mut cleared);
    assert!(!cleared.InteractComponent.as_ref().unwrap().Disabled.unwrap_or(false));
    assert_eq!(cleared.InteractComponent.as_ref().unwrap().Options.len(), 1);

    let mut omitted: Components = serde_json::from_str(r#"{"InteractComponent":{}}"#).unwrap();
    tpl.merge_into(&mut omitted);
    assert!(omitted.InteractComponent.as_ref().unwrap().Disabled.unwrap_or(false));

    let mut h = TestHarness::new("audit91c", Some(1513));
    let ent = h
        .state
        .scene
        .as_mut()
        .unwrap()
        .spawn_with_net_id(
            11,
            EntityComponentStorage::new(
                11,
                ConfigComponent {
                    camp: 2,
                    config_id: 145_750_006,
                    config_type: ConfigType::Level,
                    entity_type: EntityType::SceneItem,
                    state: EntityState::Default,
                },
            ),
        )
        .expect("spawn");
    let mut events = EventQueue::new();
    let resp = scene_handler::handle_entity_interact_request(
        &mut h.state,
        &mut events,
        EntityInteractRequest {
            EntityId: ent.net_id,
            OptionIndex: 0,
            ..Default::default()
        },
    )
    .expect("interact");
    assert_eq!(resp.ErrorCode, Some(ErrorCode::Success));
    assert_eq!(
        h.state.player_components.quest.info.get_entity_state(145_750_006),
        Some("关卡.Common.状态.激活")
    );
    assert!(h.drain_count() > 0);
}

#[test]
fn audit_91d_inst_level_play_activity_direct_train_and_scene_loading_finish() {
    let mut h = TestHarness::new("audit91d", Some(8));
    h.state.player_components.quest.info.completed_dungeons = vec![903, 1513, 806, 807];

    let inst_notify =
        misc_logic::build_inst_data_notify(&h.state.assets, &h.state.player_components.quest);
    assert_eq!(inst_notify.EnterInfos.len(), 1);
    assert_eq!(inst_notify.EnterInfos[0].Id, 2021);

    let lp_notify = misc_logic::build_level_play_info_notify(
        &h.state.assets,
        &h.state.player_components.basic,
        &h.state.player_components.quest,
    );
    for lp in &lp_notify.LevelPlayInfo {
        assert_ne!(lp.Id, 169750008);
    }

    let acts = activity::build_activity_list(
        &h.state.assets,
        &h.state.player_components.progress,
        None,
    );
    let mut saw_rogue = false;
    let mut saw_through_train = false;
    for act in &acts {
        assert_ne!(act.Type, Some(ActivityType::CircumFluence));
        if act.Type == Some(ActivityType::RougeActivity) {
            saw_rogue = true;
            assert!(
                act.Data
                    .as_ref()
                    .and_then(|d| match d {
                        sonoro_proto::pb::ActivityDataDataOneof::ActivityRogueData(r) => {
                            r.RoguelikeSeason.as_ref().map(|s| s.SeasonId > 0)
                        }
                        _ => None,
                    })
                    .unwrap_or(false)
            );
        }
        if act.Type == Some(ActivityType::ThroughTrain) {
            saw_through_train = true;
        }
    }
    assert!(saw_rogue);

    let dt_filtered = direct_train::filter_direct_train_activities(acts);
    for act in &dt_filtered {
        assert_eq!(act.Type, Some(ActivityType::ThroughTrain));
    }
    if saw_through_train {
        assert!(!dt_filtered.is_empty());
    }

    let mut events = EventQueue::new();
    quest_engine::sync_on_login(&mut h.state.quest_context(&mut events)).expect("sync on login");
    let login_frames = h.drain_count();
    assert!(login_frames > 0);
    quest_engine::on_scene_loading_finish(&mut h.state.quest_context(&mut events))
        .expect("scene loading finish");
    let finish_frames = h.drain_count();
    assert!(finish_frames <= login_frames);

    let q145_nodes = h
        .state
        .assets
        .quest_nodes
        .get_owner_nodes(145_000_005)
        .expect("q145 nodes");
    let pos_of = |id: i32| q145_nodes.iter().position(|n| n.id == id).unwrap();
    assert!(pos_of(127) < pos_of(442));
    assert!(pos_of(442) < pos_of(661));
    assert!(pos_of(131) < pos_of(7));
    assert!(pos_of(133) < pos_of(670));
    assert!(pos_of(7) < pos_of(8));
}

#[test]
fn audit_92a_transfer_main_role_state_auto_initializes_rover() {
    let mut h = TestHarness::new("audit92a", Some(8));
    for rid in [1605, 1406, 1604, 1408] {
        h.state.player_components.role.role_map.remove(&rid);
    }
    assert!(h.state.player_components.role.role_map.contains_key(&1501));
    assert!(!h.state.player_components.role.role_map.contains_key(&1605));

    let _ = role::transfer_main_role_state(
        &h.state.fs,
        &h.state.assets,
        h.state.scene.as_mut(),
        &mut h.state.player_components.basic,
        &mut h.state.player_components.role,
        &mut h.state.player_components.weapon,
        &mut h.state.player_components.echo,
        1501,
        1605,
    )
    .expect("transfer 1501->1605");
    assert!(h.state.player_components.role.role_map.contains_key(&1605));
    assert_eq!(h.state.player_components.basic.info.selected_main_role_id, 1605);

    let _ = role::transfer_main_role_state(
        &h.state.fs,
        &h.state.assets,
        h.state.scene.as_mut(),
        &mut h.state.player_components.basic,
        &mut h.state.player_components.role,
        &mut h.state.player_components.weapon,
        &mut h.state.player_components.echo,
        1605,
        1406,
    )
    .expect("transfer 1605->1406");
    assert!(h.state.player_components.role.role_map.contains_key(&1406));
    assert_eq!(h.state.player_components.basic.info.selected_main_role_id, 1406);
}

#[test]
fn audit_92b_level_play_145750001_and_quest_145000005_chain() {
    let mut h = TestHarness::new("audit92b", Some(8));
    let mut events = EventQueue::new();

    h.state
        .player_components
        .quest
        .info
        .finish_quest(quest_engine::PROLOGUE_QUEST_ID);
    {
        h.state
            .player_components
            .quest
            .info
            .get_or_add_active_quest(145_000_005)
            .mark_node_active(27);
        for pre_nid in [13, 190, 24, 45, 61] {
            h.state
                .player_components
                .quest
                .info
                .mark_node_finished(145_000_005, pre_nid);
        }
    }

    quest_engine::events::on_player_move(
        &mut h.state.quest_context(&mut events),
        871_472.75,
        705_142.8,
        2838.75,
    )
    .expect("move 1");
    h.drain_count();

    {
        let aq = h
            .state
            .player_components
            .quest
            .info
            .find_active_quest(145_000_005)
            .unwrap();
        assert!(aq.is_node_finished(27));
        assert!(aq.is_node_active(441));
        assert!(aq.is_node_active(299));
    }
    assert!(
        h.state
            .player_components
            .quest
            .info
            .progress
            .entered_level_plays
            .contains(&145_750_001)
    );
    assert!(
        !h.state
            .player_components
            .quest
            .info
            .completed_level_plays
            .contains(&145_750_001)
    );

    let train_net_id = h
        .state
        .scene
        .as_ref()
        .unwrap()
        .find_entity_net_id_by_config_id(145_750_138)
        .expect("train net id");
    let dyn_opt = h
        .state
        .scene
        .as_ref()
        .unwrap()
        .find_dynamic_interact_option("4cae376712984b8095a277b04d5d0a26")
        .expect("dyn opt");
    assert_eq!(dyn_opt.bt_type, BtType::BtTypeLevelPlay);
    assert_eq!(dyn_opt.quest_id, 145_750_001);
    assert_eq!(dyn_opt.node_id, 2);

    let _ = scene_handler::handle_entity_dynamic_interact_request(
        &mut h.state,
        &mut events,
        EntityDynamicInteractRequest {
            EntityId: train_net_id,
            OptionGuid: "4cae376712984b8095a277b04d5d0a26".as_bytes().to_vec(),
        },
    )
    .expect("dyn interact");
    h.drain_count();

    assert!(
        !h.state
            .player_components
            .quest
            .info
            .completed_level_plays
            .contains(&145_750_001)
    );
    assert_eq!(
        h.state.player_components.quest.info.get_entity_state(145_750_138),
        Some("关卡.Common.状态.激活")
    );
    {
        let aq = h
            .state
            .player_components
            .quest
            .info
            .find_active_quest(145_000_005)
            .unwrap();
        assert!(aq.is_node_finished(299));
        assert!(aq.is_node_active(441));
    }

    quest_engine::events::on_player_move(
        &mut h.state.quest_context(&mut events),
        979_477.25,
        803_665.7,
        -12_077.33,
    )
    .expect("move 2");
    h.drain_count();

    assert!(
        h.state
            .player_components
            .quest
            .info
            .completed_level_plays
            .contains(&145_750_001)
    );
    assert_eq!(
        h.state
            .player_components
            .quest
            .info
            .get_var_int("Self:145750001:列车变量"),
        2
    );
    let aq_final = h
        .state
        .player_components
        .quest
        .info
        .find_active_quest(145_000_005)
        .unwrap();
    assert!(aq_final.is_node_finished(441));
}

#[test]
fn audit_93a_3d_range_shapes_and_lord_gym_sublevel() {
    let cyl = ShapeConfig {
        Type: Some("Cylinder".to_string()),
        Radius: Some(400.0),
        Height: Some(500.0),
        ..Default::default()
    };
    assert!(is_pos_inside_range_shape([200.0, 0.0, 200.0], [0.0; 3], [0.0; 3], &cyl, 400.0, 0.0));
    assert!(!is_pos_inside_range_shape([200.0, 0.0, 400.0], [0.0; 3], [0.0; 3], &cyl, 400.0, 0.0));

    let rbox = ShapeConfig {
        Type: Some("Box".to_string()),
        Size: Some(Vec3 {
            X: Some(200.0),
            Y: Some(600.0),
            Z: Some(100.0),
        }),
        Rotator: Some(Vec3 {
            X: None,
            Y: None,
            Z: Some(90.0),
        }),
        ..Default::default()
    };
    assert!(is_pos_inside_range_shape([450.0, 0.0, 0.0], [0.0; 3], [0.0; 3], &rbox, 0.0, 0.0));
    assert!(!is_pos_inside_range_shape([0.0, 450.0, 0.0], [0.0; 3], [0.0; 3], &rbox, 0.0, 0.0));

    let mut h = TestHarness::new("audit93a", Some(8));
    let _ = h.state.player_components.progress.info.record_lord_gym_pass(1901, 45);
    h.state
        .player_components
        .quest
        .info
        .world
        .loaded_sublevels
        .push("Sub_Enable_A".to_string());
    h.state.scene.as_mut().unwrap().instance.players = vec![ScenePlayer {
        id: 1,
        location: [1000.0, 2000.0, 900.0],
        rotation: [0.0, 0.0, 0.0],
        group_type: 0,
    }];

    let rs_val: Value = serde_json::from_str(
        r#"{"Type":"RangeSphere","Center":{"X":1000,"Y":2000,"Z":500},"Radius":300}"#,
    )
    .unwrap();
    assert!(!eval_single_inline_condition(&h.state.eval_context(), &rs_val));
    h.state.scene.as_mut().unwrap().instance.players[0].location = [1000.0, 2000.0, 600.0];
    assert!(eval_single_inline_condition(&h.state.eval_context(), &rs_val));

    let gym_val: Value = serde_json::from_str(
        r#"{"Type":"CheckLordGymFinish","LordGymId":1901}"#,
    )
    .unwrap();
    assert!(eval_single_inline_condition(&h.state.eval_context(), &gym_val));

    let sub_val: Value = serde_json::from_str(
        r#"{"Type":"CheckSubLevelState","ConditionCount":0,"CheckList":[{"SubLevelName":"Sub_Enable_A","SubLevelState":"Enable"},{"SubLevelName":"Sub_Disabled_B","SubLevelState":"Disable"}]}"#,
    )
    .unwrap();
    assert!(eval_single_inline_condition(&h.state.eval_context(), &sub_val));
}

#[test]
fn audit_93b_flow_option_infos_change_state_and_global_var_atlas() {
    let mut h = TestHarness::new("audit93b", Some(8));
    let mut events = EventQueue::new();

    let actions_val: Value = serde_json::from_str(
        r#"[
            {"Name":"SetVar","Params":{"VarLeft":{"Type":"Int","Source":"Global","Keyword":"audit_93_gvar"},"VarRight":{"Type":"Int","Source":"Constant","Value":7}}},
            {"Name":"UnlockSystemItem","Params":{"SystemOption":{"Type":"AtlasSystem","UnlockOption":{"Type":"PlotPhoto","Id":4201}}}}
        ]"#,
    )
    .unwrap();
    {
        let mut qctx = h.state.quest_context(&mut events);
        let mut cbs = quest_engine::action_callbacks();
        execute_action_list(&mut qctx, &mut cbs, 0, 0, actions_val.as_array().unwrap())
            .expect("exec action list");
    }
    assert_eq!(h.state.player_components.progress.info.get_key_int("var:audit_93_gvar"), 7);
    let var_ref: Value = serde_json::from_str(
        r#"{"Type":"Int","Source":"Global","Keyword":"audit_93_gvar"}"#,
    )
    .unwrap();
    assert_eq!(
        resolve_var_int(&h.state.quest_context(&mut events), Some(&var_ref)),
        7
    );

    h.state
        .scene
        .as_mut()
        .unwrap()
        .set_active_flow(9001, 0, 0, "剧情_POI_怨鸟泽", 51, 1);
    h.drain_count();
    let _ = flow::handle_flow_end_request(
        &mut h.state,
        &mut events,
        FlowEndRequest {
            FlowIncId: 9001,
            ..Default::default()
        },
    )
    .expect("flow end 1");
    assert_eq!(h.state.scene.as_ref().unwrap().active_flow_state_id, 2);
    let next_inc = h.state.scene.as_ref().unwrap().active_flow_inc_id;
    assert_ne!(next_inc, 0);
    let _ = flow::handle_flow_end_request(
        &mut h.state,
        &mut events,
        FlowEndRequest {
            FlowIncId: next_inc,
            ..Default::default()
        },
    )
    .expect("flow end 2");
    assert_eq!(h.state.scene.as_ref().unwrap().active_flow_inc_id, 0);

    h.state
        .scene
        .as_mut()
        .unwrap()
        .set_active_flow(9002, 0, 0, "剧情_InteractTest", 2, 1);
    let mut opt_map = BTreeMap::new();
    opt_map.insert(
        5_000_006,
        FlowOptionInfoList {
            OptionIndexList: vec![FlowOptionInfo {
                TalkId: 0,
                OptionIndex: 0,
            }],
        },
    );
    let _ = flow::handle_flow_end_request(
        &mut h.state,
        &mut events,
        FlowEndRequest {
            FlowIncId: 9002,
            OptionInfos: opt_map,
            ..Default::default()
        },
    )
    .expect("flow end opt");
    assert_eq!(h.state.scene.as_ref().unwrap().active_flow_state_id, 2);
    assert_ne!(h.state.scene.as_ref().unwrap().active_flow_inc_id, 0);
}

#[test]
fn audit_94a_3d_combo_shapes_occupation_and_timer_queues() {
    let combo_json = r#"{"Type":"Combination","CombinationShapes":[
        {"Type":"Box","Center":{"X":1000,"Y":0,"Z":0},"Size":{"X":100,"Y":100,"Z":100}},
        {"Type":"Box","Center":{"X":0,"Y":0,"Z":0},"Size":{"X":100,"Y":200,"Z":500},"Rotator":{"X":0,"Y":90,"Z":0}}
    ]}"#;
    let parsed: ShapeConfig = serde_json::from_str(combo_json).unwrap();
    assert!(is_pos_inside_range_shape([400.0, 0.0, 0.0], [0.0; 3], [0.0; 3], &parsed, 0.0, 0.0));
    assert!(!is_pos_inside_range_shape([0.0, 0.0, 400.0], [0.0; 3], [0.0; 3], &parsed, 0.0, 0.0));
    assert!(is_pos_inside_range_shape([1050.0, 0.0, 0.0], [0.0; 3], [0.0; 3], &parsed, 0.0, 0.0));

    let tmp = super::TempDirGuard::new("audit94a");
    let fs = sonoro_common::fs::FileSystem::open(&tmp.path).unwrap();
    let mut scene = Scene::init(&fs, 1, 8).unwrap();

    scene.upsert_occupation(BtType::BtTypeQuest, 1001, 1001, 5, "npc_shared");
    let conflict = scene
        .find_occupation_conflict("npc_shared", 2002, 2002)
        .expect("conflict");
    assert_eq!(conflict.owner_id, 1001);
    scene.release_occupations_for_node(1001, 5);
    assert!(scene.find_occupation_conflict("npc_shared", 2002, 2002).is_none());

    assert!(scene.mark_save_node_entered(1001, 8));
    assert!(!scene.mark_save_node_entered(1001, 8));

    scene.quest_timers.upsert(QuestTimerEntry {
        quest_id: 500,
        node_id: 1,
        due_ms: 5000,
    });
    scene.lp_timers.upsert(QuestTimerEntry {
        quest_id: 500,
        node_id: 1,
        due_ms: 2000,
    });
    scene.fail_timers.upsert(QuestTimerEntry {
        quest_id: 500,
        node_id: 1,
        due_ms: 3500,
    });
    assert_eq!(scene.next_quest_wake_delay_ms(1000), Some(1000));
}

#[test]
fn audit_94b_level_play_timer_give_up_rollback_forced_occupation() {
    let mut h = TestHarness::new("audit94b", Some(8));
    let mut events = EventQueue::new();

    append_unique_int(
        &mut h.state.player_components.quest.info.progress.entered_level_plays,
        2_000_033,
    );
    h.state
        .player_components
        .progress
        .info
        .set_key_int("lp_act:2000033:5", 1);
    {
        let mut qctx = h.state.quest_context(&mut events);
        let mut lctx = level_play::LevelPlayContext::from_quest_ctx(&mut qctx).unwrap();
        level_play::ensure_active_timers_scheduled(&mut lctx).expect("schedule lp timers");
    }
    let scene = h.state.scene.as_ref().unwrap();
    assert!(scene.lp_timers.contains(2_000_033, 5));
    let due_ms = scene.lp_timers.get_due_ms(2_000_033, 5).unwrap();

    quest_engine::tree::on_timers_due(&mut h.state.quest_context(&mut events), due_ms)
        .expect("on_timers_due");
    assert!(!h.state.scene.as_ref().unwrap().lp_timers.contains(2_000_033, 5));
    assert_eq!(h.state.player_components.progress.info.get_key_int("lp_node:2000033:5"), 1);

    h.state
        .player_components
        .quest
        .info
        .get_or_add_active_quest(139_000_025)
        .mark_node_active(19);
    h.state
        .scene
        .as_mut()
        .unwrap()
        .upsert_occupation(BtType::BtTypeQuest, 99999, 99999, 1, "shared_res");

    h.drain_count();
    let _ = quest::handle_bt_give_up_request(
        &mut h.state,
        &mut events,
        BtGiveUpRequest {
            TreeOwnerId: 139_000_025,
            TreeIncId: 139_000_025,
            NodeId: 19,
        },
    )
    .expect("bt give up");
    assert!(h.drain_count() > 0);
    let _ = quest::handle_bt_rollback_request(
        &mut h.state,
        &mut events,
        BtRollbackRequest {
            TreeOwnerId: 139_000_025,
            TreeIncId: 139_000_025,
            Select: 1,
        },
    )
    .expect("bt rollback");
    assert!(h.drain_count() > 0);

    quest_engine::on_forced_occupation(&mut h.state.quest_context(&mut events), 139_000_025)
        .expect("forced occ");
    assert!(
        h.state
            .scene
            .as_ref()
            .unwrap()
            .find_occupation_conflict("shared_res", 139_000_025, 139_000_025)
            .is_none()
    );
}

#[test]
fn audit_95_extended_quest_failed_conditions_and_gravity_sync() {
    let mut h = TestHarness::new("audit95", Some(8));
    let mut events = EventQueue::new();

    {
        let sc = h.state.scene.as_mut().unwrap();
        sc.formation_info.cur_formation = 0;
        let role_ent = sc
            .spawn_with_net_id(
                110_201,
                EntityComponentStorage::new(
                    110_201,
                    ConfigComponent {
                        camp: 1,
                        config_id: 1102,
                        entity_type: EntityType::Player,
                        config_type: ConfigType::Character,
                        state: EntityState::Default,
                    },
                ),
            )
            .unwrap();
        sc.formation_info.formations[0].cur_role = 1102;
        sc.formation_info.formations[0].roles[0] = Some(sonoro_gamesv_fs::formation_info::FormationRole {
            role_id: 1102,
            entity_id: role_ent.net_id,
            ..Default::default()
        });
        sc.set_current_role_gravity([1.0, 0.0, 0.0]);
    }
    let grav_cond: Value = serde_json::from_str(
        r#"{"Type":"CheckEntityGravityDirection","Target":{"Type":"OnlinePlayer","OnlinePlayerConditionTargetOption":{"Type":"Host"}},"GravityDirection":{"Type":"WorldAxis","WorldAxis":"PositiveX"}}"#,
    )
    .unwrap();
    assert!(eval_single_inline_condition(&h.state.eval_context(), &grav_cond));

    append_unique_int(&mut h.state.player_components.quest.info.world.loaded_datalayers, 2);
    {
        let aq = h
            .state
            .player_components
            .quest
            .info
            .get_or_add_active_quest(34_900_020);
        aq.mark_node_active(2);
        aq.record_killed_entity(102_000_863);
    }

    let dl_node = sonoro_gamesv_data::lazy_node_store::NodeEntry {
        id: 6,
        parent_id: 4,
        node_type: "QuestFailed".to_string(),
        count: 0,
        reward_id: 0,
        data: serde_json::from_str(
            r#"{"Type":"QuestFailed","Id":6,"FailedCondition":{"CheckDataLayer":{"DataLayerId":2,"IsLoad":true}},"ParentNodeId":4}"#,
        )
        .unwrap(),
    };
    let mut dummy_queue = sonoro_gamesv_logic::schedulers::quest_timer_queue::QuestTimerQueue::new();
    let dl_reason = quest_engine::eval_quest_failed_condition_pub(
        &mut h.state.quest_context(&mut events),
        34_900_020,
        &dl_node,
        None,
        &mut dummy_queue,
    )
    .unwrap();
    assert_eq!(dl_reason, Some(FailReason::DataLayerFail));

    let death_node = sonoro_gamesv_data::lazy_node_store::NodeEntry {
        id: 50,
        parent_id: 1,
        node_type: "QuestFailed".to_string(),
        count: 0,
        reward_id: 0,
        data: serde_json::from_str(
            r#"{"Type":"QuestFailed","Id":50,"FailedCondition":{"EntityDeathCondition":{"EntityIds":[102000863]}},"ParentNodeId":1}"#,
        )
        .unwrap(),
    };
    let death_reason = quest_engine::eval_quest_failed_condition_pub(
        &mut h.state.quest_context(&mut events),
        34_900_020,
        &death_node,
        None,
        &mut dummy_queue,
    )
    .unwrap();
    assert_eq!(death_reason, Some(FailReason::EntityDieFail));

    h.state.player_components.quest.info.get_or_add_active_quest(30_000_000);
    quest_engine::on_player_transfer_or_leave_inst(&mut h.state.quest_context(&mut events), true)
        .expect("transfer or leave inst");
}

#[test]
fn audit_96_entity_alert_check_dungeon_failure_difficulty_and_sneak() {
    let mut h = TestHarness::new("audit96", Some(8));
    let mut events = EventQueue::new();
    let mut dummy_queue = sonoro_gamesv_logic::schedulers::quest_timer_queue::QuestTimerQueue::new();

    {
        let sc = h.state.scene.as_mut().unwrap();
        let npc_ent = sc
            .spawn_with_net_id(
                137_024,
                EntityComponentStorage::new(
                    137_024,
                    ConfigComponent {
                        camp: 2,
                        config_id: 137_000_024,
                        entity_type: EntityType::Monster,
                        config_type: ConfigType::Level,
                        state: EntityState::Default,
                    },
                ),
            )
            .unwrap();
        sc.fsm_hate_states.insert(npc_ent.net_id, true);
        sc.record_failed_dungeon(80);
        sc.active_sneak_id = 11;
    }

    let alert_node = sonoro_gamesv_data::lazy_node_store::NodeEntry {
        id: 6,
        parent_id: 1,
        node_type: "QuestFailed".to_string(),
        count: 0,
        reward_id: 0,
        data: serde_json::from_str(
            r#"{"Type":"QuestFailed","Id":6,"FailedCondition":{"EntityAlert":{"EntityIds":[137000024]}},"ParentNodeId":1}"#,
        )
        .unwrap(),
    };
    let alert_reason = quest_engine::eval_quest_failed_condition_pub(
        &mut h.state.quest_context(&mut events),
        137_000_001,
        &alert_node,
        None,
        &mut dummy_queue,
    )
    .unwrap();
    assert_eq!(alert_reason, Some(FailReason::DefaultFail));

    let cdf_node = sonoro_gamesv_data::lazy_node_store::NodeEntry {
        id: 23,
        parent_id: 22,
        node_type: "QuestFailed".to_string(),
        count: 0,
        reward_id: 0,
        data: serde_json::from_str(
            r#"{"Type":"QuestFailed","Id":23,"FailedCondition":{"CheckDungeonFailure":[80]},"ParentNodeId":22}"#,
        )
        .unwrap(),
    };
    let cdf_reason = quest_engine::eval_quest_failed_condition_pub(
        &mut h.state.quest_context(&mut events),
        30_000_002,
        &cdf_node,
        None,
        &mut dummy_queue,
    )
    .unwrap();
    assert_eq!(cdf_reason, Some(FailReason::InstFail));

    let sneak_node = sonoro_gamesv_data::lazy_node_store::NodeEntry {
        id: 11,
        parent_id: 6,
        node_type: "QuestFailed".to_string(),
        count: 0,
        reward_id: 0,
        data: serde_json::from_str(
            r#"{"Type":"QuestFailed","Id":11,"FailedCondition":{"SneakPlayCondition":{"Time":8}},"ParentNodeId":6}"#,
        )
        .unwrap(),
    };
    let _ = quest_engine::eval_quest_failed_condition_pub(
        &mut h.state.quest_context(&mut events),
        16_000_000,
        &sneak_node,
        None,
        &mut dummy_queue,
    )
    .unwrap();
    assert!(dummy_queue.contains(16_000_000, 11));

    h.state.player_components.quest.info.get_or_add_active_quest(16_000_000);
    quest_engine::on_sneak_failed(&mut h.state.quest_context(&mut events), 16_000_000, 16_000_000)
        .expect("sneak failed");
    assert!(h.drain_count() > 0);
    quest_engine::on_difficulty_changed(&mut h.state.quest_context(&mut events))
        .expect("difficulty changed");
}

#[test]
fn audit_97_morale_exp_gravity_hook_and_dragon_pool() {
    let mut h = TestHarness::new("audit97", Some(8));
    let mut events = EventQueue::new();
    let assets = Arc::clone(&h.state.assets);

    assert_eq!(morale::compute_keep_level(Some(&assets), 0), 1);
    assert_eq!(morale::compute_keep_level(Some(&assets), 199), 1);
    assert_eq!(morale::compute_keep_level(Some(&assets), 200), 2);
    assert_eq!(morale::compute_keep_level(Some(&assets), 20000), 101);
    assert_eq!(morale::compute_keep_level(Some(&assets), 300000), 500);
    assert_eq!(morale::compute_temp_level(Some(&assets), 50001, 150), 1);

    let setup_val: Value =
        serde_json::from_str(r#"[{"Name":"SetupMoraleSystem","Params":{"MoralePlayId":1,"IsOn":true}}]"#)
            .unwrap();
    {
        let mut qctx = h.state.quest_context(&mut events);
        let mut cbs = ActionCallbacks::default();
        execute_action_list(&mut qctx, &mut cbs, 133_850_004, 1, setup_val.as_array().unwrap())
            .expect("setup morale");
    }
    assert_eq!(h.state.player_components.progress.info.get_key_int("morale:play:1"), 1);
    assert!(h.drain_count() > 0);

    {
        let mut rctx = h.state.reward_context();
        let grant = grant_items(
            &mut rctx,
            0,
            false,
            &[
                RewardItem { item_id: 47, count: 20000 },
                RewardItem { item_id: 48, count: 150 },
            ],
            2,
        )
        .expect("grant morale");
        assert_eq!(grant.granted_count, 2);
    }
    assert_eq!(h.state.player_components.progress.info.get_key_int("morale:level"), 101);
    assert_eq!(h.state.player_components.progress.info.get_key_int("morale:exp"), 20000);
    assert_eq!(h.state.player_components.progress.info.get_key_int("morale:temp_level"), 1);
    assert!(h.drain_count() > 0);

    let morale_match: Value = serde_json::from_str(
        r#"{"Type":"CheckPlayerMoraleLevelRange","MinMoraleLevel":101,"MaxMoraleLevel":150}"#,
    )
    .unwrap();
    let morale_fail: Value = serde_json::from_str(
        r#"{"Type":"CheckPlayerMoraleLevelRange","MinMoraleLevel":1,"MaxMoraleLevel":100}"#,
    )
    .unwrap();
    assert!(eval_single_inline_condition(&h.state.eval_context(), &morale_match));
    assert!(!eval_single_inline_condition(&h.state.eval_context(), &morale_fail));

    let hook_net_id = {
        let sc = h.state.scene.as_mut().unwrap();
        sc.formation_info.cur_formation = 0;
        let role_ent = sc
            .spawn_with_net_id(
                110_201,
                EntityComponentStorage::new(
                    110_201,
                    ConfigComponent {
                        camp: 1,
                        config_id: 1102,
                        entity_type: EntityType::Player,
                        config_type: ConfigType::Character,
                        state: EntityState::Default,
                    },
                ),
            )
            .unwrap();
        sc.formation_info.formations[0].cur_role = 1102;
        sc.formation_info.formations[0].roles[0] = Some(sonoro_gamesv_fs::formation_info::FormationRole {
            role_id: 1102,
            entity_id: role_ent.net_id,
            ..Default::default()
        });

        let mut hook_storage = EntityComponentStorage::new(
            1_870_148,
            ConfigComponent {
                camp: 2,
                config_id: 187_000_148,
                entity_type: EntityType::SceneItem,
                config_type: ConfigType::Level,
                state: EntityState::Default,
            },
        );
        hook_storage.position = Some(PositionComponent {
            location: [0.0, 0.0, 0.0],
            rotation: [-90.0, 0.0, 0.0],
        });
        sc.spawn_with_net_id(1_870_148, hook_storage).unwrap().net_id
    };
    let _ = world_puzzle::handle_gravity_hook_lock_point_change_gravity_request(
        &mut h.state,
        GravityHookLockPointChangeGravityRequest {
            EntityId: hook_net_id,
        },
    )
    .expect("gravity hook");
    assert!(h.state.player_components.progress.info.has_claim("gravity_hook:chg:1870148"));

    let grav_y: Value = serde_json::from_str(
        r#"{"Type":"CheckEntityGravityDirection","Target":{"Type":"OnlinePlayer","OnlinePlayerConditionTargetOption":{"Type":"Host"}},"GravityDirection":{"Type":"WorldAxis","WorldAxis":"PositiveY"}}"#,
    )
    .unwrap();
    assert!(eval_single_inline_condition(&h.state.eval_context(), &grav_y));
    let hook_direct = gravity::resolve_hook_gravity(&assets, 8, 187_750_086, [-90.0, 0.0, 45.0]);
    assert!(hook_direct.configured);

    h.state.player_components.inventory.info.add_normal_item(40_040_003, 3);
    let _ = inventory::handle_hand_in_ming_su_request(
        &mut h.state,
        &mut events,
        HandInMingSuRequest {
            DragonPoolId: 3,
            ..Default::default()
        },
    )
    .expect("mingsu 1");
    assert_eq!(h.state.player_components.progress.info.get_key_int("mingsu:lvl:3"), 1);
    let _ = inventory::handle_hand_in_ming_su_request(
        &mut h.state,
        &mut events,
        HandInMingSuRequest {
            DragonPoolId: 3,
            ..Default::default()
        },
    )
    .expect("mingsu 2");
    assert_eq!(h.state.player_components.progress.info.get_key_int("mingsu:lvl:3"), 4);

    let fc_ge4: Value = serde_json::from_str(
        r#"{"Type":"CheckFormationRoleInfo","Option":{"Type":"FeatureCollection","Id":3,"Compare":"Ge","Level":4}}"#,
    )
    .unwrap();
    let fc_ge5: Value = serde_json::from_str(
        r#"{"Type":"CheckFormationRoleInfo","Option":{"Type":"FeatureCollection","Id":3,"Compare":"Ge","Level":5}}"#,
    )
    .unwrap();
    assert!(eval_single_inline_condition(&h.state.eval_context(), &fc_ge4));
    assert!(!eval_single_inline_condition(&h.state.eval_context(), &fc_ge5));
}
