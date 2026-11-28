use std::collections::BTreeMap;
use std::sync::Arc;

use serde_json::Value;
use sonoro_common::fs::{FileSystem, SyncOp};
use sonoro_gamesv_data::asset_indexes::{entity_world_scale, is_spawnable_item_config};
use sonoro_gamesv_data::fsm_graph_registry::Graph;
use sonoro_gamesv_data::tables::blueprint_config::EntityType as BpEntityType;
use sonoro_gamesv_data::tables::damage::Damage;
use sonoro_gamesv_data::tables::entity_components::base_info::BaseInfoComponent;
use sonoro_gamesv_data::tables::entity_components::components::{
    Components, gravity_flip_from_default,
};
use sonoro_gamesv_data::tables::entity_components::extra::{
    ClientTriggerComponent, EntityVisibleComponent, ExtraEntry, ExtraKind,
};
use sonoro_gamesv_data::tables::entity_components::range::{ShapeConfig, Vec3};
use sonoro_gamesv_data::tables::level_entity_config::{LevelEntityConfig, Vector as LVector};
use sonoro_gamesv_fs::echo_info::EchoInfo;
use sonoro_gamesv_fs::formation_info::{Formation, FormationRole};
use sonoro_gamesv_fs::progress_info::ProgressInfo;
use sonoro_gamesv_fs::quest_info::append_unique_int;
use sonoro_gamesv_fs::role_info::RoleInfo;
use sonoro_gamesv_fs::scene_instance::ScenePlayer;
use sonoro_gamesv_fs::weapon_item::WeaponItem;
use sonoro_gamesv_logic::commands::{cheat, entity as entity_cmd};
use sonoro_gamesv_logic::component::entity::attribute::AttributeComponent;
use sonoro_gamesv_logic::component::entity::fight_buff::{FightBuffComponent, Shield};
use sonoro_gamesv_logic::component::entity::fsm::FsmComponent;
use sonoro_gamesv_logic::component::entity::logic_state::LogicStateComponent;
use sonoro_gamesv_logic::component::entity::part::PartComponent;
use sonoro_gamesv_logic::component::entity::small::{
    ConfigComponent, ConfigType, EntityState, EntityType, MonsterAiComponent, PositionComponent,
    SummonerComponent,
};
use sonoro_gamesv_logic::component::entity::{EntityComponentStorage, TagComponent};
use sonoro_gamesv_logic::coop_service::{CoopService, OnlinePlayer, VecCoopSender};
use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::events::{Event, PendingFlow};
use sonoro_gamesv_logic::fsm::types::wake_reason;
use sonoro_gamesv_logic::handlers::{
    fsm as fsm_handler, misc as misc_logic, role as role_logic, scene as scene_logic,
};
use sonoro_gamesv_logic::helpers::condition::{
    eval_entity_visible_component, eval_single_inline_condition, is_pos_inside_range_shape,
};
use sonoro_gamesv_logic::helpers::quest_actions::{
    emit_entity_state_tag_update_checked, execute_action, execute_action_list,
    update_player_scene_state,
};
use sonoro_gamesv_logic::helpers::quest_types::{ActionCallbacks, QuestContext};
use sonoro_gamesv_logic::helpers::reward::{self, RewardContext, RewardItem, grant_items};
use sonoro_gamesv_logic::helpers::{
    attributes, buff as buff_helper, damage as damage_helper, gameplay_tags, gravity, level_play,
    quest_engine, role as role_helper,
};
use sonoro_gamesv_logic::schedulers::quest_timer_queue::QuestTimerEntry;
use sonoro_gamesv_logic::templates::role_entity_templates;
use sonoro_proto::PbEnum;
use sonoro_proto::enums::{
    AccessRangeType, BtType, EAttributeType, EClientStorageSystemIdType, EDamageImmune,
    ESummonType, ErrorCode, GravityFlipType, RangeType,
};
use sonoro_proto::pb::{
    AbyssRankListRequest, AchievementInfoRequest, AfterJoinSceneNotify, BabelTowerRankListRequest,
    BeControlledRequest, BeControlledShowRequest, BehaviorTreeInfoNotify,
    BossRushSettlementRequest, BuffItemRequest, ChangeEntityStateRequest, ChannelChatRequest,
    CharacterDeathPush, ClientStorageInfo, ClientStorageInfoDataOneof, ClientStorageMapData,
    ClientStorageMapMapData, CombatNotifyDataMessageOneof, CombatReceiveData,
    CombatReceiveDataMessageOneof, DamageExecuteRequest, EntityAccessRangeRequest,
    EntityAddDynamicInteractNotify, EntityAddNotify, EntityComponentPbComponentPbOneof,
    EntityPatrolChangeDirRequest, EntityPositionRequest, EntityRemoveDynamicInteractNotify,
    FlowStartNotify, GachaInfoRequest, GachaPoolDetailRequest,
    GravityFlipInteractRequest, GravityFlipUpdateTypeNotify, HoldHandRequest, HookLockPointRequest,
    ItemDisuseRequest, ItemExchangeInfoRequest, ItemRewardNotify, JoinSceneNotify,
    KurotatoSyncEntityDealDamagePush, LandingDamageRequest, LowMemoryChangeSceneFinishRequest,
    LowMemoryChangeSceneStartNotify, MonsterDetectionInfoRequest, MonthCardRequest,
    MotorCycleCruiseStatePush, MotorFightRankListRequest, MotorSummonAndRidePush,
    AddPlacementNotify, NewTowerRecommendRequest, OperationRestrictionNotify, PayGiftInfoRequest,
    PayInfoRequest, PayShopInfoRequest, PhantomBattleCallCardRequest, PhantomBattleStartRequest,
    PhantomFetterRecommendRequest, PhantomMusicSwitchRequest, PinballFriendRankRequest,
    PlaceItemOnBoardRequest, PlayerAccessRangeRequest, PlayerDeadNotify,
    PrivateChatHistoryRequest, PrivateChatRequest, QuestReadyListNotify, RacingBetsRankRequest,
    ReadDisplayInfoRequest, RemoveSummonEntityRequest, RhythmRankListRequest,
    RoleTowerRankRequest, RoverRogueBlessGroupSelectRequest, SceneLoadingFinishRequest,
    SlashAndTowerRecommendRequest, StorageInfoNotify, StorageInfoRequest,
    StorageInfoUpdateRequest, SummonEntityNotify, TeleportDataRequest, TeleportTransferRequest,
    TimelineTraceControlRequest, TraceQuestNotify, TraceQuestRequest, UseSkillRequest,
    WeatherControlRequest, WeekCardInfoRequest, ZitherPlayNotePush,
};

use super::{TempDirGuard, TestHarness};
use crate::network::gm_i18n;
use crate::network::handlers::{
    achievement, activity, chat, combat_sync, compat_3811, echo, endgame, events_major,
    events_misc, explore, gacha, inventory, map, minimal, motor, phantom_battle, qol, quest,
    roguelike, scene, shop, skin, teleport, world_puzzle,
};

#[test]
fn audit_98_3d_scale_shapes_rotation_remove_claim_and_level_play_control() {
    let vecs = vec![
        LVector::default(),
        LVector::default(),
        LVector {
            X: 150.0,
            Y: 200.0,
            Z: 300.0,
        },
    ];
    let cfg_no_scale = LevelEntityConfig {
        Id: 1,
        EntityId: 1,
        BlueprintType: "Test".to_string(),
        IsScaleEnabled: false,
        MapId: 8,
        AreaId: 1,
        Transform: vecs.clone(),
        ..Default::default()
    };
    let cfg_scaled = LevelEntityConfig {
        Id: 2,
        EntityId: 2,
        BlueprintType: "Test".to_string(),
        IsScaleEnabled: true,
        MapId: 8,
        AreaId: 1,
        Transform: vecs,
        ..Default::default()
    };
    assert_eq!(entity_world_scale(&cfg_no_scale), [1.0, 1.0, 1.0]);
    assert_eq!(entity_world_scale(&cfg_scaled), [1.5, 2.0, 3.0]);

    let rot_v = gravity::rotate_local_offset_ue([100.0, 0.0, 0.0], [0.0, 0.0, 90.0]);
    assert!(rot_v[0].abs() < 0.01);
    assert!((rot_v[1] - 100.0).abs() < 0.01);
    assert!(rot_v[2].abs() < 0.01);

    let vol_shape = ShapeConfig {
        Type: Some("Volume".to_string()),
        Size: Some(Vec3 {
            X: Some(200.0),
            Y: Some(200.0),
            Z: Some(100.0),
        }),
        ..Default::default()
    };
    assert!(is_pos_inside_range_shape([150.0, 0.0, 50.0], [0.0; 3], [0.0; 3], &vol_shape, 0.0, 0.0));
    assert!(!is_pos_inside_range_shape([150.0, 0.0, 150.0], [0.0; 3], [0.0; 3], &vol_shape, 0.0, 0.0));

    let tbox_shape = ShapeConfig {
        Type: Some("TransportBox".to_string()),
        Size: Some(Vec3 {
            X: Some(50.0),
            Y: Some(50.0),
            Z: Some(50.0),
        }),
        ..Default::default()
    };
    assert!(is_pos_inside_range_shape([40.0, 40.0, 40.0], [0.0; 3], [0.0; 3], &tbox_shape, 0.0, 0.0));
    assert!(!is_pos_inside_range_shape([80.0, 0.0, 0.0], [0.0; 3], [0.0; 3], &tbox_shape, 0.0, 0.0));

    let cone_shape = ShapeConfig {
        Type: Some("Cone".to_string()),
        Radius: Some(500.0),
        Height: Some(300.0),
        Angle: Some(60.0),
        ..Default::default()
    };
    assert!(is_pos_inside_range_shape([300.0, 0.0, 0.0], [0.0; 3], [0.0; 3], &cone_shape, 0.0, 0.0));
    assert!(!is_pos_inside_range_shape([0.0, 300.0, 0.0], [0.0; 3], [0.0; 3], &cone_shape, 0.0, 0.0));
    assert!(!is_pos_inside_range_shape([300.0, 0.0, 400.0], [0.0; 3], [0.0; 3], &cone_shape, 0.0, 0.0));

    let small_box = ShapeConfig {
        Type: Some("Box".to_string()),
        Size: Some(Vec3 {
            X: Some(10.0),
            Y: Some(10.0),
            Z: Some(10.0),
        }),
        ..Default::default()
    };
    assert!(is_pos_inside_range_shape([5.0, 5.0, 5.0], [0.0; 3], [0.0; 3], &small_box, 0.0, 0.0));
    assert!(!is_pos_inside_range_shape([50.0, 0.0, 0.0], [0.0; 3], [0.0; 3], &small_box, 0.0, 0.0));

    let mut pinfo = ProgressInfo::default();
    assert!(pinfo.mark_claim("lp_node:10:1"));
    assert!(pinfo.has_claim("lp_node:10:1"));
    assert!(pinfo.remove_claim("lp_node:10:1"));
    assert!(!pinfo.has_claim("lp_node:10:1"));
    assert!(!pinfo.remove_claim("lp_node:10:1"));

    let mut h = TestHarness::new("audit98", Some(8));
    let mut events = EventQueue::new();
    h.state.player_components.basic.info.attributes.sex = 1;
    h.state.player_components.quest.info.get_or_add_active_quest(859_000_001);
    let _ = quest_engine::tree::recheck_pending_conditions(&mut h.state.quest_context(&mut events))
        .expect("recheck active trees");
    let _ = quest_engine::step_pending_control_nodes(&mut h.state.quest_context(&mut events))
        .expect("step pending");

    h.state.scene.as_mut().unwrap().lp_fail_timers.upsert(QuestTimerEntry {
        quest_id: 2_000_033,
        node_id: 9,
        due_ms: 5000,
    });
    quest_engine::on_entity_killed(
        &mut h.state.quest_context(&mut events),
        999_999,
        Some(999_999),
    )
    .expect("on entity killed");
    assert!(h.state.scene.as_ref().unwrap().lp_fail_timers.contains(2_000_033, 9));

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
        let _ = level_play::on_bt_timer_end(&mut lctx, 2_000_033).expect("lp bt timer end");
    }
    assert_eq!(h.state.player_components.progress.info.get_key_int("lp_node:2000033:5"), 1);
    quest_engine::on_forced_occupation(&mut h.state.quest_context(&mut events), 2_000_033)
        .expect("forced occ");
}

#[test]
fn audit_99_condition_action_sub_options_and_world_combat_rpc_sync() {
    let mut h = TestHarness::new("audit99", Some(8));
    let mut events = EventQueue::new();
    h.state.scene.as_mut().unwrap().instance.players = vec![ScenePlayer {
        id: 1,
        location: [0.0, 0.0, 0.0],
        rotation: [0.0, 0.0, 15.0],
        group_type: 0,
    }];

    let (role_net_id, npc_net_id) = {
        let sc = h.state.scene.as_mut().unwrap();
        let mut role_attr = AttributeComponent::default();
        role_attr.set_prop(EAttributeType::LifeMax, 10000);
        role_attr.set_prop(EAttributeType::Life, 10000);
        let mut role_buffs = FightBuffComponent::default();
        role_buffs.upsert_shield(Shield {
            handle_id: 1,
            shield_id: 9001,
            buff_handle_id: 0,
            current: 5000,
            max: 5000,
            priority: 1,
        });
        let mut role_storage = EntityComponentStorage::new(
            110_201,
            ConfigComponent {
                camp: 1,
                config_id: 1102,
                config_type: ConfigType::Character,
                entity_type: EntityType::Player,
                state: EntityState::Default,
            },
        );
        role_storage.position = Some(PositionComponent {
            location: [0.0, 0.0, 0.0],
            rotation: [-90.0, 0.0, 0.0],
        });
        role_storage.direct_control = true;
        role_storage.attribute = Some(role_attr);
        role_storage.buffs = Some(role_buffs);
        role_storage.world_item.gravity = Some(sonoro_gamesv_logic::component::entity::small::GravityComponent {
            location: [0.0, 1.0, 0.0],
        });
        let role_ent = sc.spawn_with_net_id(110_201, role_storage).unwrap();
        sc.formation_info.cur_formation = 0;
        sc.formation_info.formations[0].cur_role = 1102;
        sc.formation_info.formations[0].roles[0] = Some(FormationRole {
            role_id: 1102,
            entity_id: role_ent.net_id,
            ..Default::default()
        });

        let mut npc_storage = EntityComponentStorage::new(
            550_001,
            ConfigComponent {
                camp: 2,
                config_id: 550_001,
                config_type: ConfigType::Level,
                entity_type: EntityType::Npc,
                state: EntityState::Default,
            },
        );
        npc_storage.position = Some(PositionComponent {
            location: [100.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
        });
        let npc_ent = sc.spawn_with_net_id(550_001, npc_storage).unwrap();
        (role_ent.net_id, npc_ent.net_id)
    };

    h.state.player_components.basic.info.attributes.sex = 1;
    let pass_cases = [
        r#"{"Type":"CheckPlayerGender","Gender":"Male"}"#,
        r#"{"Type":"CheckGameplayTag","GameplayTag":"怪物.common.调试","Compare":"Ne","Target":{"Type":"Npc","EntityId":550001}}"#,
        r#"{"Type":"CheckDirection","Direction":{},"AngleInterval":30}"#,
        r#"{"Type":"CheckTargetAttribute","Option":{"Type":"Player","Option":"CurRole","AttributeTypes":[{"Type":"Shield","Compare":"Eq","Value":50}]}}"#,
        r#"{"Type":"CheckBvbChallengeState","ChallengeId":2007,"ChallengeState":0}"#,
        r#"{"Type":"CheckEntityGravityDirection","Target":{"Type":"OnlinePlayer","OnlinePlayerConditionTargetOption":{"Type":"Host"}},"GravityDirection":{"Type":"SelfRotation"}}"#,
    ];
    for raw in pass_cases {
        let v: Value = serde_json::from_str(raw).unwrap();
        assert!(eval_single_inline_condition(&h.state.eval_context(), &v), "expected true: {raw}");
    }
    let fail_cases = [
        r#"{"Type":"CheckDirection","AngleInterval":60}"#,
        r#"{"Type":"CheckDirection","Direction":{"Z":0},"AngleInterval":0}"#,
        r#"{"Type":"CheckBvbChallengeState","ChallengeId":2007,"ChallengeState":2}"#,
    ];
    for raw in fail_cases {
        let v: Value = serde_json::from_str(raw).unwrap();
        assert!(!eval_single_inline_condition(&h.state.eval_context(), &v), "expected false: {raw}");
    }

    let _ = h.state.player_components.quest.info.unlock_system(10012);
    let acts_val: Value = serde_json::from_str(
        r#"[
            {"Name":"SetBattleState","Params":{"StateOption":{"Type":"SetBattleTag","SetTags":[{"EntityId":550001,"SetType":"Add","GameplayTag":"怪物.common.调试"}]}}},
            {"Name":"ModifySceneItemAttributeTag","Params":{"Type":"Target","IsAddTag":true,"EntityIds":[550001],"PerformanceTag":[1937741205]}},
            {"Name":"ExecAlertSystemAction","Params":{"Option":{"Type":"EnableAlertArea","AreaId":900001,"IsEnable":true}}},
            {"Name":"EnableFunction","Params":{"Type":"Other","FunctionId":10012,"Enable":false}},
            {"Name":"EnableHostility","Params":{"EntityIds":[550001],"IsEnable":false}},
            {"Name":"EnableAI","Params":{"EntityIds":[550001],"IsEnable":true}},
            {"Name":"OverrideAiBehaviorTree","Params":{"Key":"B1","EntityIds":[550001],"BehaviorTree":800043}},
            {"Name":"ManualOccupations","Params":{"OccupationType":"Request","Occupations":["ganxue"]}},
            {"Name":"SetNpcGroupPerform","Params":{"Key":"q1","PerformType":{"Type":0}}}
        ]"#,
    )
    .unwrap();
    {
        let mut qctx = h.state.quest_context(&mut events);
        let mut cbs = ActionCallbacks::default();
        execute_action_list(&mut qctx, &mut cbs, 100, 1, acts_val.as_array().unwrap())
            .expect("exec acts");
    }

    assert!(!h.state.scene.as_ref().unwrap().player_in_battle);
    assert!(!h.state.player_components.quest.info.has_unlocked_system(10012));
    assert_eq!(h.state.player_components.progress.info.get_key_int("alert_enable:900001"), 1);
    assert_eq!(h.state.player_components.progress.info.get_key_int("hostility:550001"), 0);
    assert_eq!(h.state.player_components.progress.info.get_key_int("ai_enabled:550001"), 1);
    assert_eq!(h.state.player_components.progress.info.get_key_int("ai_bt_cur:550001"), 800_043);
    assert_eq!(h.state.player_components.progress.info.get_key_int("occ:ganxue"), 1);
    assert_eq!(h.state.player_components.progress.info.get_key_int("hold_hand:110201"), 1);

    h.drain_count();
    let _ = compat_3811::handle_landing_damage_request(
        &mut h.state,
        &mut events,
        LandingDamageRequest {
            EntityId: role_net_id,
            SpeedZ: -2600.0,
            ..Default::default()
        },
    )
    .expect("landing dmg");
    let updated_life = h
        .state
        .scene
        .as_ref()
        .unwrap()
        .entity(role_net_id)
        .unwrap()
        .attribute
        .as_ref()
        .unwrap()
        .get_current(EAttributeType::Life);
    assert!((1..10000).contains(&updated_life));
    assert!(h.drain_count() > 0);

    let _ = world_puzzle::handle_be_controlled_request(
        &mut h.state,
        BeControlledRequest {
            EntityId: npc_net_id,
            IsControl: true,
        },
    )
    .expect("be ctrl");
    let _ = world_puzzle::handle_be_controlled_show_request(
        &mut h.state,
        BeControlledShowRequest {
            EntityId: npc_net_id,
            IsShow: true,
        },
    )
    .expect("be ctrl show");
    let _ = world_puzzle::handle_entity_patrol_change_dir_request(
        &mut h.state,
        EntityPatrolChangeDirRequest {
            EntityId: npc_net_id,
            Dir: true,
        },
    )
    .expect("patrol dir");
    let _ = world_puzzle::handle_hold_hand_request(
        &mut h.state,
        HoldHandRequest {
            EntityId: role_net_id,
            TargetEntityId: npc_net_id,
            HandType: 1,
            IsHold: true,
            ..Default::default()
        },
    )
    .expect("hold hand");

    let sc = h.state.scene.as_ref().unwrap();
    let role_pb = sc.entity(role_net_id).unwrap().to_proto(&h.state.assets, 0);
    let saw_shield = role_pb.ComponentPbs.iter().any(|c| {
        matches!(
            &c.ComponentPb,
            Some(EntityComponentPbComponentPbOneof::ShieldComponentPb(s))
                if s.ShieldValueTotal == 5000 && s.ShieldInfoPbList.len() == 1
        )
    });
    let saw_hold_hand = role_pb.ComponentPbs.iter().any(|c| {
        matches!(
            &c.ComponentPb,
            Some(EntityComponentPbComponentPbOneof::HoldHandComponentPb(hh))
                if hh.TargetEntityId == npc_net_id && hh.HandType == 1
        )
    });
    assert!(saw_shield);
    assert!(saw_hold_hand);

    let npc_pb = sc.entity(npc_net_id).unwrap().to_proto(&h.state.assets, 0);
    let saw_be_ctrl = npc_pb.ComponentPbs.iter().any(|c| {
        matches!(
            &c.ComponentPb,
            Some(EntityComponentPbComponentPbOneof::BeControlledComponentPb(b))
                if b.IsShow && b.PlayerEntityId == role_net_id
        )
    });
    let saw_patrol = npc_pb.ComponentPbs.iter().any(|c| {
        matches!(
            &c.ComponentPb,
            Some(EntityComponentPbComponentPbOneof::PatrolComponent(p)) if p.Dir
        )
    });
    assert!(saw_be_ctrl);
    assert!(saw_patrol);
}

#[test]
fn audit_100_gravity_volume_access_range_monster_ai_and_push_handlers() {
    let mut h = TestHarness::new("audit100", Some(8));
    let mut events = EventQueue::new();

    let (role_net_id, gv_net_id) = {
        let sc = h.state.scene.as_mut().unwrap();
        let mut role_attr = AttributeComponent::default();
        role_attr.set_prop(EAttributeType::LifeMax, 10000);
        role_attr.set_prop(EAttributeType::Life, 10000);
        let mut role_storage = EntityComponentStorage::new(
            110_201,
            ConfigComponent {
                camp: 1,
                config_id: 1102,
                entity_type: EntityType::Player,
                config_type: ConfigType::Character,
                state: EntityState::Default,
            },
        );
        role_storage.attribute = Some(role_attr);
        let role_ent = sc.spawn_with_net_id(110_201, role_storage).unwrap();
        sc.formation_info.cur_formation = 0;
        sc.formation_info.formations[0].cur_role = 1102;
        sc.formation_info.formations[0].roles[0] = Some(FormationRole {
            role_id: 1102,
            entity_id: role_ent.net_id,
            ..Default::default()
        });

        let mut gv_storage = EntityComponentStorage::new(
            880_101,
            ConfigComponent {
                camp: 2,
                config_id: 187_000_148,
                entity_type: EntityType::SceneItem,
                config_type: ConfigType::Level,
                state: EntityState::Default,
            },
        );
        gv_storage.world_item.range_present = true;
        gv_storage.world_item.trigger_present = true;
        let gv_ent = sc.spawn_with_net_id(880_101, gv_storage).unwrap();
        (role_ent.net_id, gv_ent.net_id)
    };

    let mai = MonsterAiComponent {
        weapon_id: 1001,
        hatred_id: 21001,
        perception_ids: [1, 170001, 0, 0, 0, 0, 0, 0],
        perception_len: 2,
        ..Default::default()
    };
    let mai_pb = mai.to_proto();
    assert_eq!(mai_pb.HatredId, 21001);
    assert_eq!(mai_pb.BasicPerceptionIds.len(), 2);
    assert_eq!(mai_pb.BasicPerceptionIds[1], 170001);
    assert!(h.state.assets.tables.ai_sense_group.get_data_by_id(&1).is_some());

    let _ = map::handle_player_access_range_request(
        &mut h.state,
        &mut events,
        PlayerAccessRangeRequest {
            RangeEntityId: gv_net_id,
            AccessRangeType: Some(AccessRangeType::RangeEnter),
        },
    )
    .expect("range enter");
    assert_eq!(h.state.scene.as_ref().unwrap().entity(gv_net_id).unwrap().world_item.in_range_player_id, 1);
    assert!(h.state.scene.as_ref().unwrap().entity(role_net_id).unwrap().world_item.gravity.is_some());

    let gv_pb = h
        .state
        .scene
        .as_ref()
        .unwrap()
        .entity(gv_net_id)
        .unwrap()
        .to_proto(&h.state.assets, 0);
    assert!(gv_pb.ComponentPbs.iter().any(|c| {
        matches!(
            &c.ComponentPb,
            Some(EntityComponentPbComponentPbOneof::RangeComponent(r))
                if r.InRangePlayers == vec![1]
        )
    }));

    let _ = map::handle_player_access_range_request(
        &mut h.state,
        &mut events,
        PlayerAccessRangeRequest {
            RangeEntityId: gv_net_id,
            AccessRangeType: Some(AccessRangeType::RangeLeave),
        },
    )
    .expect("range leave");
    assert_eq!(h.state.scene.as_ref().unwrap().entity(gv_net_id).unwrap().world_item.in_range_player_id, 0);
    assert!(h.state.scene.as_ref().unwrap().entity(role_net_id).unwrap().world_item.gravity.is_none());

    let _ = crate::network::handlers::motor::handle_motor_summon_and_ride_push(
        &mut h.state,
        &mut events,
        MotorSummonAndRidePush {
            EntityId: role_net_id,
            VehicleIncId: 500,
            ..Default::default()
        },
    );
    assert_eq!(h.state.player_components.progress.info.get_key_int("motor:riding"), 1);

    let _ = crate::network::handlers::motor::handle_motor_cycle_cruise_state_push(
        &mut h.state,
        &mut events,
        MotorCycleCruiseStatePush { IsStart: true },
    );
    assert_eq!(h.state.player_components.progress.info.get_key_int("motor:cruise"), 1);

    let _ = events_misc::handle_zither_play_note_push(
        &mut h.state,
        ZitherPlayNotePush {
            Note: b"C4".to_vec(),
        },
    );
    assert_eq!(h.state.player_components.progress.info.get_key_int("misc:zither_notes"), 1);

    let mut kd_map = BTreeMap::new();
    kd_map.insert(10, 250);
    let _ = events_major::handle_kurotato_sync_entity_deal_damage_push(
        &mut h.state,
        KurotatoSyncEntityDealDamagePush {
            EntityDealDamages: kd_map,
        },
    );
    assert_eq!(h.state.player_components.progress.info.get_key_int("kurotato:dmg_total"), 250);

    h.drain_count();
    let _ = minimal::handle_character_death_push(
        &mut h.state,
        CharacterDeathPush {
            RoleId: 1102,
            ..Default::default()
        },
    );
    assert_eq!(
        h.state
            .scene
            .as_ref()
            .unwrap()
            .entity(role_net_id)
            .unwrap()
            .attribute
            .as_ref()
            .unwrap()
            .get_current(EAttributeType::Life),
        0
    );
    assert!(h.drain_count() > 0);
}

#[test]
fn audit_101_deep_audit_talk_flow_interact_always_false_and_world_3d_sync() {
    let mut h = TestHarness::new("audit101", Some(8));
    let mut events = EventQueue::new();
    let lp_anchor = h
        .state
        .assets
        .find_level_entity(8, 135_901_340)
        .and_then(|cfg| sonoro_gamesv_data::asset_indexes::entity_world_pos(&cfg))
        .unwrap_or([0.0, 0.0, 0.0]);
    h.state.scene.as_mut().unwrap().instance.players = vec![ScenePlayer {
        id: 1,
        location: lp_anchor,
        rotation: [0.0, 0.0, 0.0],
        group_type: 0,
    }];

    h.state
        .player_components
        .quest
        .info
        .finish_quest(quest_engine::PROLOGUE_QUEST_ID);
    h.state.scene.as_mut().unwrap().join_scene_pushed = false;
    quest_engine::sync_on_login(&mut h.state.quest_context(&mut events)).expect("sync on login");
    assert!(!h.drain_push_msg_ids().contains(&QuestReadyListNotify::MSG_ID));

    h.state.scene.as_mut().unwrap().join_scene_pushed = true;
    quest_engine::triggers::push_ready_quest_list(&mut h.state.quest_context(&mut events));
    assert!(h.drain_push_msg_ids().contains(&QuestReadyListNotify::MSG_ID));

    {
        let mut qctx = h.state.quest_context(&mut events);
        let mut lctx = level_play::LevelPlayContext::from_quest_ctx(&mut qctx).unwrap();
        level_play::on_level_play_enabled(&mut lctx, 135_900_013, true).expect("enable lp");
    }
    let lp_ids = h.drain_push_msg_ids();
    let bt_info_pos = lp_ids
        .iter()
        .position(|&m| m == BehaviorTreeInfoNotify::MSG_ID)
        .expect("bt info notify");
    let var_or_leave_before = lp_ids[..bt_info_pos].iter().any(|&m| {
        m == sonoro_proto::pb::BtVarUpdateNotify::MSG_ID
            || m == sonoro_proto::pb::UpdateNodeStatusNotify::MSG_ID
            || m == sonoro_proto::pb::LeaveLevelPlayNotify::MSG_ID
    });
    assert!(!var_or_leave_before);
    assert_eq!(h.state.player_components.progress.info.get_key_int("lp_act:135900013:14"), 1);

    let ent_a_net = h
        .state
        .scene
        .as_mut()
        .unwrap()
        .spawn_with_net_id(
            70001,
            EntityComponentStorage::new(
                70001,
                ConfigComponent {
                    camp: 2,
                    config_id: 145_750_006,
                    config_type: ConfigType::Level,
                    entity_type: EntityType::SceneItem,
                    state: EntityState::Default,
                },
            ),
        )
        .unwrap()
        .net_id;
    let st_id = gameplay_tags::id_from_name(
        &h.state.assets.tables.gameplay_tag_remap,
        "关卡.Common.状态.激活",
    )
    .unwrap_or(0);
    let ces = world_puzzle::handle_change_entity_state_request(
        &mut h.state,
        ChangeEntityStateRequest {
            EntityId: ent_a_net as i32,
            StateTagId: st_id,
        },
    )
    .expect("ces");
    assert_eq!(ces.ErrorCode, Some(ErrorCode::Success));
    assert_eq!(
        h.state.player_components.quest.info.get_entity_state(145_750_006),
        Some("关卡.Common.状态.激活")
    );
    h.drain_count();

    let mut act_map = BTreeMap::new();
    act_map.insert(1001, 1);
    let _ = qol::handle_storage_info_update_request(
        &mut h.state,
        StorageInfoUpdateRequest {
            Infos: vec![ClientStorageInfo {
                SystemId: EClientStorageSystemIdType::Activity.to_i32(),
                Data: Some(ClientStorageInfoDataOneof::MapData(ClientStorageMapData {
                    Data: act_map,
                })),
            }],
        },
    )
    .expect("storage update");
    let sg = qol::handle_storage_info_request(&mut h.state, StorageInfoRequest::default())
        .expect("storage get");
    assert!(sg.Infos.iter().any(|inf| {
        inf.SystemId == EClientStorageSystemIdType::Activity.to_i32()
            && matches!(&inf.Data, Some(ClientStorageInfoDataOneof::MapData(m)) if m.Data.len() == 1)
    }));

    let hook_net = {
        let mut s = EntityComponentStorage::new(
            70002,
            ConfigComponent {
                camp: 2,
                config_id: 187_000_148,
                config_type: ConfigType::Level,
                entity_type: EntityType::SceneItem,
                state: EntityState::Default,
            },
        );
        s.world_item.range_present = true;
        h.state.scene.as_mut().unwrap().spawn_with_net_id(70002, s).unwrap().net_id
    };
    let _ = world_puzzle::handle_hook_lock_point_request(
        &mut h.state,
        HookLockPointRequest { EntityId: hook_net },
    )
    .expect("hook lock");
    assert_eq!(h.state.player_components.progress.info.get_key_int("hook_point:active:187000148"), 1);

    let _ = map::handle_entity_access_range_request(
        &mut h.state,
        &mut events,
        EntityAccessRangeRequest {
            EntityId: hook_net,
            RangeType: Some(RangeType::RangeEnter),
            EntitiesToCheck: vec![999_001],
        },
    )
    .expect("entity range");
    assert_eq!(h.state.scene.as_ref().unwrap().entity(hook_net).unwrap().world_item.in_range_player_id, 0);

    let saved_scene = h.state.scene.take();
    quest_engine::sync_on_login(&mut h.state.quest_context(&mut events)).expect("sync no scene");
    assert!(!h.drain_push_msg_ids().contains(&QuestReadyListNotify::MSG_ID));
    h.state.scene = saved_scene;

    h.state.scene.as_mut().unwrap().remove(&h.state.fs, ent_a_net).expect("remove ent_a");
    assert!(!h.state.scene.as_ref().unwrap().net_id_map.contains_key(&ent_a_net));

    let mut scene_lp = sonoro_gamesv_logic::scene::Scene::init(&h.state.fs, 1, 8).unwrap();
    scene_lp.join_scene_pushed = true;
    sonoro_gamesv_fs::quest_info::remove_int_value(
        &mut h.state.player_components.quest.info.progress.entered_level_plays,
        135_900_013,
    );
    append_unique_int(
        &mut h.state.player_components.quest.info.progress.entered_level_plays,
        316_700_001,
    );
    h.state.scene = Some(scene_lp);
    {
        let mut qctx = h.state.quest_context(&mut events);
        let mut lctx = level_play::LevelPlayContext::from_quest_ctx(&mut qctx).unwrap();
        level_play::sync_on_login(&mut lctx).expect("lp316 sync");
    }
    let lp316_ids = h.drain_push_msg_ids();
    let enter_pos = lp316_ids
        .iter()
        .position(|&m| m == sonoro_proto::pb::EnterLevelPlayNotify::MSG_ID)
        .unwrap();
    let bt_pos = lp316_ids
        .iter()
        .position(|&m| m == BehaviorTreeInfoNotify::MSG_ID)
        .unwrap();
    assert!(enter_pos < bt_pos);
    let update_before = lp316_ids[..bt_pos].iter().any(|&m| {
        m == sonoro_proto::pb::BtVarUpdateNotify::MSG_ID
            || m == sonoro_proto::pb::UpdateNodeStatusNotify::MSG_ID
            || m == sonoro_proto::pb::UpdateChildQuestNodeStatusNotify::MSG_ID
            || m == sonoro_proto::pb::ActionNotify::MSG_ID
    });
    assert!(!update_before);
    assert!(lp316_ids.len() < 20);
}

#[test]
fn audit_102_fs_noop_skip_join_seq_batch_bullet_ready_filter_and_level_tag_guard() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let tmp = TempDirGuard::new("audit102_fs");
    let fs = FileSystem::open(&tmp.path).unwrap();
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_c = Arc::clone(&counter);
    fs.set_sync_hook(Arc::new(move |_op: SyncOp, _k: &str, _b: &[u8]| {
        counter_c.fetch_add(1, Ordering::Relaxed);
    }));
    fs.write_file("test/noop_check", b"payload_v1").unwrap();
    assert_eq!(counter.load(Ordering::Relaxed), 1);
    fs.write_file("test/noop_check", b"payload_v1").unwrap();
    assert_eq!(counter.load(Ordering::Relaxed), 1);
    fs.write_file("test/noop_check", b"payload_v2").unwrap();
    assert_eq!(counter.load(Ordering::Relaxed), 2);

    let mut h = TestHarness::new("audit102", Some(8));
    assert_eq!(h.state.conn.last_scene_transition_seq(), 0);
    h.state
        .conn
        .push_msg(QuestReadyListNotify::MSG_ID, &QuestReadyListNotify::default())
        .unwrap();
    assert_eq!(h.state.conn.last_scene_transition_seq(), 0);
    h.state
        .conn
        .push_msg(JoinSceneNotify::MSG_ID, &JoinSceneNotify::default())
        .unwrap();
    assert_eq!(h.state.conn.last_scene_transition_seq(), h.state.conn.seq());
    h.drain_count();

    let mut bbc = EntityComponentStorage::new(
        81001,
        ConfigComponent {
            camp: 2,
            config_id: 161_900_039,
            config_type: ConfigType::Level,
            entity_type: EntityType::SceneItem,
            state: EntityState::Default,
        },
    );
    bbc.world_item.batch_bullet_cast_present = true;
    h.state.scene.as_mut().unwrap().spawn_with_net_id(81001, bbc).unwrap();
    let bbc_pb = h
        .state
        .scene
        .as_ref()
        .unwrap()
        .entity(81001)
        .unwrap()
        .to_proto(&h.state.assets, 0);
    assert!(bbc_pb.ComponentPbs.iter().any(|c| {
        matches!(&c.ComponentPb, Some(EntityComponentPbComponentPbOneof::BatchBulletCastComponentPb(_)))
    }));

    let mut events = EventQueue::new();
    h.state
        .player_components
        .quest
        .info
        .finish_quest(quest_engine::PROLOGUE_QUEST_ID);
    h.state.scene.as_mut().unwrap().join_scene_pushed = true;
    quest_engine::triggers::push_ready_quest_list(&mut h.state.quest_context(&mut events));

    let ready_list = h.drain_decoded::<QuestReadyListNotify>(QuestReadyListNotify::MSG_ID);
    assert!(!ready_list.is_empty());
    for ready in ready_list {
        for qid in ready.QuestId {
            assert_ne!(qid, 133_000_010);
            if let Some(qe) = h.state.assets.tables.quest_data.get_data_by_id(&qid) {
                assert!(qe.Data.DungeonId == 0 || qe.Data.DungeonId == 8);
                if let Some(Value::Object(aio)) = qe.Data.AddInteractOption.as_ref()
                    && let Some(ev) = aio.get("EntityId").and_then(Value::as_i64)
                    && ev > 0
                {
                    assert!(h.state.assets.find_level_entity(8, ev).is_some());
                }
            }
        }
    }

    let item_net = h
        .state
        .scene
        .as_mut()
        .unwrap()
        .spawn_with_net_id(
            81002,
            EntityComponentStorage::new(
                81002,
                ConfigComponent {
                    camp: 2,
                    config_id: 145_750_006,
                    config_type: ConfigType::Level,
                    entity_type: EntityType::SceneItem,
                    state: EntityState::Default,
                },
            ),
        )
        .unwrap()
        .net_id;
    let npc_net = h
        .state
        .scene
        .as_mut()
        .unwrap()
        .spawn_with_net_id(
            81003,
            EntityComponentStorage::new(
                81003,
                ConfigComponent {
                    camp: 0,
                    config_id: 9_000_001,
                    config_type: ConfigType::Level,
                    entity_type: EntityType::Npc,
                    state: EntityState::Default,
                },
            ),
        )
        .unwrap()
        .net_id;
    assert!(sonoro_gamesv_logic::handlers::tags::has_client_level_tag_component(
        h.state.scene.as_ref().unwrap(),
        Some(&h.state.assets),
        item_net
    ));
    assert!(!sonoro_gamesv_logic::handlers::tags::has_client_level_tag_component(
        h.state.scene.as_ref().unwrap(),
        Some(&h.state.assets),
        npc_net
    ));

    h.drain_count();
    let sc = h.state.scene.as_ref().unwrap();
    emit_entity_state_tag_update_checked(
        sc,
        Some(&h.state.assets),
        &mut h.state.conn,
        npc_net,
        sonoro_gamesv_logic::scene::StateTagUpdate {
            old_tag_id: 101,
            new_tag_id: 102,
        },
    );
    let ids = h.drain_push_msg_ids();
    assert!(!ids.contains(&sonoro_proto::pb::EntityCommonTagNotify::MSG_ID));
    assert!(ids.contains(&sonoro_proto::pb::EntityStateReadyNotify::MSG_ID));
}

#[test]
fn audit_103_full_solo_regression_suite() {
    let tpl_comp = Components {
        extra: vec![ExtraEntry::VehicleComponent(Box::default())],
        ..Default::default()
    };
    let mut merged_comp = Components {
        extra: vec![ExtraEntry::BatchBulletCasterComponent(Box::default())],
        ..Default::default()
    };
    tpl_comp.merge_into(&mut merged_comp);
    assert!(merged_comp.has_extra(ExtraKind::BatchBulletCasterComponent));
    assert!(merged_comp.has_extra(ExtraKind::VehicleComponent));
    assert_eq!(gravity_flip_from_default(1), Some(0));
    assert_eq!(gravity_flip_from_default(4), Some(3));
    assert_eq!(gravity_flip_from_default(0), None);

    let mut h = TestHarness::new("audit103", Some(8));
    let mut events = EventQueue::new();

    {
        let mut full = EntityComponentStorage::new(
            82001,
            ConfigComponent {
                camp: 2,
                config_id: 161_900_039,
                config_type: ConfigType::Level,
                entity_type: EntityType::SceneItem,
                state: EntityState::Default,
            },
        );
        full.world_item.sub_entity_type = 5;
        full.world_item.be_controlled_present = true;
        full.world_item.drop_present = true;
        full.world_item.drop_item_id = 2;
        full.world_item.drop_item_count = 10;
        full.world_item.foundation_source_entity_id = 82002;
        full.world_item.range_present = true;
        full.world_item.in_range_entity_ids[0] = 9001;
        full.world_item.in_range_entity_len = 1;
        h.state.scene.as_mut().unwrap().spawn_with_net_id(82001, full).unwrap();
    }
    let full_pb = h
        .state
        .scene
        .as_ref()
        .unwrap()
        .entity(82001)
        .unwrap()
        .to_proto(&h.state.assets, 0);
    assert_eq!(full_pb.SubEntityType, 5);
    assert!(full_pb.ComponentPbs.iter().any(|c| matches!(&c.ComponentPb, Some(EntityComponentPbComponentPbOneof::BeControlledComponentPb(_)))));
    assert!(full_pb.ComponentPbs.iter().any(|c| matches!(&c.ComponentPb, Some(EntityComponentPbComponentPbOneof::DropComponentPb(d)) if d.ItemId == 2 && d.ItemCount == 10)));
    assert!(full_pb.ComponentPbs.iter().any(|c| matches!(&c.ComponentPb, Some(EntityComponentPbComponentPbOneof::EntityFoundationComponentPb(_)))));
    assert!(full_pb.ComponentPbs.iter().any(|c| matches!(&c.ComponentPb, Some(EntityComponentPbComponentPbOneof::RangeComponent(r)) if r.InRangeEntities == vec![9001])));

    let grav_net = h
        .state
        .scene
        .as_mut()
        .unwrap()
        .spawn_with_net_id(
            82010,
            EntityComponentStorage::new(
                82010,
                ConfigComponent {
                    camp: 2,
                    config_id: 5_551_234,
                    config_type: ConfigType::Level,
                    entity_type: EntityType::SceneItem,
                    state: EntityState::Default,
                },
            ),
        )
        .unwrap()
        .net_id;
    h.drain_count();
    let _ = world_puzzle::handle_gravity_flip_interact_request(
        &mut h.state,
        GravityFlipInteractRequest {
            GravityFlipEntityId: grav_net,
            TargetGravityFlipType: Some(GravityFlipType::GravityLeft),
        },
    )
    .expect("gravity flip");
    assert_eq!(
        h.state
            .scene
            .as_ref()
            .unwrap()
            .entity(grav_net)
            .unwrap()
            .world_item
            .gravity_flip,
        Some(2)
    );
    let gfn = h.drain_decoded::<GravityFlipUpdateTypeNotify>(GravityFlipUpdateTypeNotify::MSG_ID);
    assert!(gfn.iter().any(|n| n.EntityId == grav_net && n.NewGravityFlipType == Some(GravityFlipType::GravityLeft)));

    let flow_act: Value = serde_json::from_str(
        r#"{"Name":"PlayFlow","Async":true,"Params":{"FlowListName":"TestFlow","FlowId":1,"StateId":1}}"#,
    )
    .unwrap();
    assert!(quest_engine::has_async_play_flow_action(&[flow_act]));

    {
        let sc = h.state.scene.as_mut().unwrap();
        sc.set_active_flow(1, 139_000_025, 10, "FlowA", 1, 1);
        sc.enqueue_pending_flow(139_000_025, 10, "FlowB", 1, 1);
        assert!(sc.has_pending_flow_for_node(139_000_025, 10));
        sc.clear_active_flow();
        sc.clear_pending_flows();
    }
    quest_engine::on_entity_killed(&mut h.state.quest_context(&mut events), 0, Some(999_999)).unwrap();
    if let Some(aq) = h
        .state
        .player_components
        .quest
        .info
        .find_active_quest(quest_engine::PROLOGUE_QUEST_ID)
    {
        assert!(!aq.killed_entities.contains(&999_999));
    }

    h.state.player_components.inventory.info.add_normal_item(42_100_030, 3);
    let role_net = {
        let sc = h.state.scene.as_mut().unwrap();
        let mut attr = AttributeComponent::default();
        attr.set_prop(EAttributeType::LifeMax, 10000);
        attr.set_prop(EAttributeType::Life, 0);
        let mut s = EntityComponentStorage::new(
            110_201,
            ConfigComponent {
                camp: 1,
                config_id: 1102,
                config_type: ConfigType::Character,
                entity_type: EntityType::Player,
                state: EntityState::Default,
            },
        );
        s.attribute = Some(attr);
        let ent = sc.spawn_with_net_id(110_201, s).unwrap();
        sc.formation_info.cur_formation = 0;
        sc.formation_info.formations[0] = Formation {
            cur_role: 1102,
            roles: [
                Some(FormationRole {
                    role_id: 1102,
                    entity_id: ent.net_id,
                    ..Default::default()
                }),
                None,
                None,
            ],
            ..Default::default()
        };
        ent.net_id
    };
    h.drain_count();
    let buff1 = combat_sync::handle_buff_item_request(
        &mut h.state,
        &mut events,
        BuffItemRequest {
            ItemId: 42_100_030,
            Num: 1,
            RoleId: 1102,
        },
    )
    .expect("buff item 1");
    assert_eq!(buff1.ErrorCode, Some(ErrorCode::Success));
    assert!(
        attributes::get_attr_value(
            h.state
                .scene
                .as_ref()
                .unwrap()
                .entity(role_net)
                .unwrap()
                .attribute
                .as_ref()
                .unwrap(),
            EAttributeType::Life
        ) > 0
    );
    let buff2 = combat_sync::handle_buff_item_request(
        &mut h.state,
        &mut events,
        BuffItemRequest {
            ItemId: 42_100_030,
            Num: 1,
            RoleId: 1102,
        },
    )
    .expect("buff item 2");
    assert_eq!(buff2.ErrorCode, Some(ErrorCode::ErrBuffItemCdLimit));

    attributes::set_life_unconditional(
        h.state
            .scene
            .as_mut()
            .unwrap()
            .entity_mut(role_net)
            .unwrap()
            .attribute
            .as_mut()
            .unwrap(),
        0,
    );
    h.drain_count();
    let _ = minimal::handle_character_death_push(
        &mut h.state,
        CharacterDeathPush {
            RoleId: 1102,
            ..Default::default()
        },
    );
    assert!(h.drain_push_msg_ids().contains(&PlayerDeadNotify::MSG_ID));

    let wc = world_puzzle::handle_weather_control_request(
        &mut h.state,
        WeatherControlRequest { WeatherSwitchId: 2 },
    )
    .expect("weather ctrl");
    assert_eq!(wc.WeatherId, 30002);

    h.drain_count();
    let _ = events_misc::handle_place_item_on_board_request(
        &mut h.state,
        PlaceItemOnBoardRequest {
            TargetBoardEntityId: 82050,
            PlacementEntityId: 82051,
            ToPlace: 1,
            ..Default::default()
        },
    )
    .expect("place on board");
    assert!(!h.state.player_components.progress.info.get_key_str("misc:board_item:82050:82051").is_empty());
    assert!(h.drain_push_msg_ids().contains(&AddPlacementNotify::MSG_ID));

    h.state.player_components.echo.echo_map.insert(
        501,
        EchoInfo {
            id: 600_001,
            func_value: 0,
            ..Default::default()
        },
    );
    let dis1 = inventory::handle_item_disuse_request(
        &mut h.state,
        ItemDisuseRequest { IncrId: 501, Oper: 1 },
    )
    .unwrap();
    assert_eq!(dis1.ErrCode, Some(ErrorCode::Success));
    assert_eq!(h.state.player_components.echo.echo_map.get(&501).unwrap().func_value, 2);
    let dis2 = inventory::handle_item_disuse_request(
        &mut h.state,
        ItemDisuseRequest { IncrId: 501, Oper: 2 },
    )
    .unwrap();
    assert_eq!(dis2.ErrCode, Some(ErrorCode::Success));
    assert_eq!(h.state.player_components.echo.echo_map.get(&501).unwrap().func_value, 0);
}

#[test]
fn audit_104_damage_formula_solo_sync_delayed_boss_removal_and_shop() {
    let mut h = TestHarness::new("audit104", Some(8));
    let mut events = EventQueue::new();

    let (atk_net, boss_net) = {
        let sc = h.state.scene.as_mut().unwrap();
        let mut atk_attr = AttributeComponent::default();
        atk_attr.set_prop(EAttributeType::Lv, 90);
        atk_attr.set_prop(EAttributeType::Atk, 1000);
        atk_attr.set_prop(EAttributeType::CritDamage, 15000);
        atk_attr.set_prop(EAttributeType::Crit, 0);

        let mut atk_s = EntityComponentStorage::new(
            900_101,
            ConfigComponent {
                camp: 1,
                config_id: 1506,
                config_type: ConfigType::Character,
                entity_type: EntityType::Player,
                state: EntityState::Default,
            },
        );
        atk_s.attribute = Some(atk_attr);
        atk_s.buffs = Some(FightBuffComponent::default());
        atk_s.tag = Some(TagComponent::default());
        sc.spawn_with_net_id(900_101, atk_s).unwrap();

        let mut def_attr = AttributeComponent::default();
        def_attr.set_prop(EAttributeType::Lv, 90);
        def_attr.set_prop(EAttributeType::Life, 5000);
        def_attr.set_prop(EAttributeType::LifeMax, 5000);
        def_attr.set_prop(EAttributeType::Def, 1520);

        let mut boss_s = EntityComponentStorage::new(
            900_102,
            ConfigComponent {
                camp: 2,
                config_id: 147_902_300,
                config_type: ConfigType::Level,
                entity_type: EntityType::Monster,
                state: EntityState::Default,
            },
        );
        boss_s.attribute = Some(def_attr);
        boss_s.buffs = Some(FightBuffComponent::default());
        boss_s.tag = Some(TagComponent::default());
        boss_s.fsm = Some(FsmComponent::from_graph(Arc::new(Graph::default()), 0).unwrap());
        sc.spawn_with_net_id(900_102, boss_s).unwrap();
        (900_101, 900_102)
    };

    let dmg_cfg = Damage {
        Id: 900_001,
        FormulaType: 0,
        CalculateType: 0,
        Element: 0,
        Type: 0,
        ImmuneType: 0,
        RelatedProperty: EAttributeType::Atk.to_i32(),
        RateLv: vec![10000],
        FluctuationLower: vec![9000],
        FluctuationUpper: vec![11000],
        ..Default::default()
    };
    let mut pack: Vec<CombatReceiveData> = Vec::new();
    let dmg_res = damage_helper::damage_entity(
        &mut pack,
        &DamageExecuteRequest {
            DamageId: dmg_cfg.Id,
            SkillLevel: 1,
            AttackerEntityId: atk_net,
            TargetEntityId: boss_net,
            RandomSeed: 5000,
            ..Default::default()
        },
        &dmg_cfg,
        &h.state.assets,
        h.state.scene.as_mut().unwrap(),
        None,
    );
    assert_eq!(dmg_res.Damage, 500);
    for entry in &pack {
        if let Some(CombatReceiveDataMessageOneof::CombatNotifyData(cn)) = &entry.Message {
            assert!(!matches!(
                &cn.Message,
                Some(CombatNotifyDataMessageOneof::DamageExecuteNotify(_))
            ));
        }
    }

    let inv_tag = gameplay_tags::id_from_name(
        &h.state.assets.tables.gameplay_tag_remap,
        "功能.逻辑状态标识.无敌",
    )
    .unwrap_or(1_918_148_596);
    h.state
        .scene
        .as_mut()
        .unwrap()
        .entity_mut(boss_net)
        .unwrap()
        .tag
        .as_mut()
        .unwrap()
        .add_tag(inv_tag, 1);
    let inv_res = damage_helper::damage_entity(
        &mut pack,
        &DamageExecuteRequest {
            DamageId: dmg_cfg.Id,
            SkillLevel: 1,
            AttackerEntityId: atk_net,
            TargetEntityId: boss_net,
            RandomSeed: 5000,
            ..Default::default()
        },
        &dmg_cfg,
        &h.state.assets,
        h.state.scene.as_mut().unwrap(),
        None,
    );
    assert_eq!(inv_res.ImmuneType, Some(EDamageImmune::Invincible));
    assert_eq!(inv_res.Damage, 0);
    h.state
        .scene
        .as_mut()
        .unwrap()
        .entity_mut(boss_net)
        .unwrap()
        .tag
        .as_mut()
        .unwrap()
        .remove_tag(inv_tag);

    let ab_dmg = Damage {
        Id: 900_002,
        FormulaType: 1001,
        CalculateType: 0,
        Element: 4,
        RelatedProperty: EAttributeType::Atk.to_i32(),
        FormulaParam1: vec![10000],
        ..Default::default()
    };
    h.state
        .scene
        .as_mut()
        .unwrap()
        .entity_mut(atk_net)
        .unwrap()
        .attribute
        .as_mut()
        .unwrap()
        .set_prop(EAttributeType::SpecialDamageChange, 0);
    let ab_full = damage_helper::damage_entity(
        &mut pack,
        &DamageExecuteRequest {
            DamageId: ab_dmg.Id,
            SkillLevel: 1,
            AttackerEntityId: atk_net,
            TargetEntityId: boss_net,
            ..Default::default()
        },
        &ab_dmg,
        &h.state.assets,
        h.state.scene.as_mut().unwrap(),
        None,
    );
    h.state
        .scene
        .as_mut()
        .unwrap()
        .entity_mut(atk_net)
        .unwrap()
        .attribute
        .as_mut()
        .unwrap()
        .set_prop(EAttributeType::SpecialDamageChange, -5000);
    let ab_half = damage_helper::damage_entity(
        &mut pack,
        &DamageExecuteRequest {
            DamageId: ab_dmg.Id,
            SkillLevel: 1,
            AttackerEntityId: atk_net,
            TargetEntityId: boss_net,
            ..Default::default()
        },
        &ab_dmg,
        &h.state.assets,
        h.state.scene.as_mut().unwrap(),
        None,
    );
    assert!(ab_full.Damage > 0);
    assert!(ab_half.Damage < ab_full.Damage);

    let now_ms: i64 = 1_000_000;
    h.state
        .scene
        .as_mut()
        .unwrap()
        .queue_killed_monster_removal_at(boss_net, now_ms + 300);
    assert_eq!(
        h.state.scene.as_ref().unwrap().pending_update_wake_delay_ms(now_ms),
        Some(300)
    );
    scene_logic::handle_scene_pending_updates(
        now_ms,
        h.state.scene.as_mut().unwrap(),
        &h.state.fs,
        &mut h.state.conn,
    );
    assert!(h.state.scene.as_ref().unwrap().net_id_map.contains_key(&boss_net));
    h.drain_count();
    scene_logic::handle_scene_pending_updates(
        now_ms + 300,
        h.state.scene.as_mut().unwrap(),
        &h.state.fs,
        &mut h.state.conn,
    );
    assert!(!h.state.scene.as_ref().unwrap().net_id_map.contains_key(&boss_net));
    assert!(h.drain_count() > 0);

    {
        let mut dead = EntityComponentStorage::new(
            900_103,
            ConfigComponent {
                camp: 2,
                config_id: 147_902_300,
                config_type: ConfigType::Level,
                entity_type: EntityType::Monster,
                state: EntityState::Default,
            },
        );
        dead.attribute = Some(AttributeComponent::default());
        dead.buffs = Some(FightBuffComponent::default());
        dead.logic_state = Some(LogicStateComponent::default());
        dead.tag = Some(TagComponent::default());
        dead.part = Some(PartComponent::default());
        dead.fsm = Some(FsmComponent::from_graph(Arc::new(Graph::default()), 0).unwrap());
        let sc = h.state.scene.as_mut().unwrap();
        sc.spawn_with_net_id(900_103, dead).unwrap();
        sc.mark_fsm_dirty(900_103, wake_reason::ATTRIBUTE);
    }
    h.drain_count();
    {
        let st = &mut h.state;
        let mut qctx = QuestContext {
            assets: &st.assets,
            fs: &st.fs,
            sink: &mut st.conn,
            scene: st.scene.as_mut(),
            scene_comp: Some(&mut st.player_components.scene),
            events: &mut events,
            basic_comp: &mut st.player_components.basic,
            inventory_comp: &mut st.player_components.inventory,
            weapon_comp: &mut st.player_components.weapon,
            echo_comp: &mut st.player_components.echo,
            map_comp: &mut st.player_components.map,
            quest_comp: &mut st.player_components.quest,
            role_comp: None,
            guide_comp: Some(&mut st.player_components.guide),
            mail_comp: Some(&mut st.player_components.mail),
            progress_comp: Some(&mut st.player_components.progress),
            activity_notify: None,
        };
        fsm_handler::handle_fsm_wake(
            now_ms,
            &mut qctx,
            &st.assets,
            &mut st.player_components.role,
            &st.fs,
            &mut st.buff_timers,
        );
    }
    assert_eq!(h.drain_count(), 0);

    {
        let mut qctx = h.state.quest_context(&mut events);
        let mut lctx = level_play::LevelPlayContext::from_quest_ctx(&mut qctx).unwrap();
        level_play::spawn_vision_capture_phantom(
            &mut lctx,
            atk_net,
            147_902_300,
            "VisionItem001",
            600_001,
        );
    }
    assert!(h.state.scene.as_ref().unwrap().entities.iter().any(|s| {
        s.config.config_id == 14_000_056 && s.state_tag.is_some_and(|st| st.state_tag_id != 0)
    }));

    {
        let mut concom = EntityComponentStorage::new(
            900_106,
            ConfigComponent {
                camp: 1,
                config_id: 100_000_031,
                config_type: ConfigType::Template,
                entity_type: EntityType::Player,
                state: EntityState::Default,
            },
        );
        concom.summoner = Some(SummonerComponent {
            summoner_id: atk_net,
            summon_type: ESummonType::ConcomitantCustom,
            ..Default::default()
        });
        h.state.scene.as_mut().unwrap().spawn_with_net_id(900_106, concom).unwrap();
    }
    let _ = explore::handle_remove_summon_entity_request(
        &mut h.state,
        RemoveSummonEntityRequest {
            SummonerId: atk_net,
            RemoveENtityIds: vec![900_106],
            ..Default::default()
        },
    )
    .expect("remove summon");
    assert!(h.state.scene.as_ref().unwrap().net_id_map.contains_key(&900_106));

    let shop_resp = shop::handle_pay_shop_info_request(&mut h.state, PayShopInfoRequest::default())
        .expect("pay shop");
    for rec in &shop_resp.PayShopRecommendData {
        assert!(rec.Show);
        assert!(!rec.PrefabPath.is_empty());
        assert!(!rec.TabImage.is_empty());
    }
    assert!(shop_resp.Infos.iter().any(|i| i.Items.iter().any(|it| it.Id == 104_300_001)));
    assert!(shop_resp.Infos.iter().any(|i| i.Items.iter().any(|it| it.Id == 106_100_008)));

    h.drain_count();
    {
        let mut rctx = h.state.reward_context();
        let _ = grant_items(
            &mut rctx,
            999_999_991,
            true,
            &[RewardItem { item_id: 2, count: 10 }],
            19500,
        )
        .expect("grant items");
    }
    let rewards = h.drain_decoded::<ItemRewardNotify>(ItemRewardNotify::MSG_ID);
    assert!(!rewards.is_empty());
    for r in rewards {
        assert!(h.state.assets.tables.drop_package.get_data_by_id(&r.DropId).is_some());
        assert_ne!(r.DropId, 1);
    }
}

#[test]
fn audit_105_tower_heal_weekcard_shop_activity_and_auto_buffs() {
    let mut h = TestHarness::new("audit105", Some(8));
    let mut events = EventQueue::new();

    let role_net = {
        let sc = h.state.scene.as_mut().unwrap();
        let mut attr = AttributeComponent::default();
        attr.set_prop(EAttributeType::Lv, 60);
        attr.set_prop(EAttributeType::LifeMax, 10000);
        attr.set_prop(EAttributeType::Life, 4000);
        let mut s = EntityComponentStorage::new(
            900_201,
            ConfigComponent {
                camp: 0,
                config_id: 1506,
                config_type: ConfigType::Character,
                entity_type: EntityType::Player,
                state: EntityState::Default,
            },
        );
        s.attribute = Some(attr);
        s.buffs = Some(FightBuffComponent::default());
        let ent = sc.spawn_with_net_id(900_201, s).unwrap();
        sc.formation_info.cur_formation = 0;
        sc.formation_info.formations[0].cur_role = 1506;
        sc.formation_info.formations[0].roles[0] = Some(FormationRole {
            role_id: 1506,
            entity_id: ent.net_id,
            ..Default::default()
        });
        ent.net_id
    };

    role_helper::revive_and_heal_formation(
        &mut h.state.conn,
        h.state.scene.as_mut().unwrap(),
        &mut h.state.player_components.role,
        &mut events,
        &h.state.fs,
    );
    assert_eq!(
        h.state
            .scene
            .as_ref()
            .unwrap()
            .entity(role_net)
            .unwrap()
            .attribute
            .as_ref()
            .unwrap()
            .get_current(EAttributeType::Life),
        10000
    );
    while let Some(ev) = events.dequeue() {
        assert!(!matches!(ev, Event::BuffAddition { .. }));
    }

    let wc = qol::handle_week_card_info_request(
        &mut h.state,
        WeekCardInfoRequest {
            WeekCardIds: vec![1, 2],
            ..Default::default()
        },
    )
    .expect("week card");
    assert_eq!(wc.WeekCardInfos.len(), 2);
    assert_eq!(wc.WeekCardInfos[0].WeekCardContentInfos.len(), 3);
    assert_eq!(wc.WeekCardInfos[0].WeekCardContentInfos[0].WeekCardContentId, 101);
    assert_eq!(wc.WeekCardInfos[1].WeekCardContentInfos[0].WeekCardContentId, 201);

    let shop_resp = shop::handle_pay_shop_info_request(&mut h.state, PayShopInfoRequest::default())
        .expect("pay shop");
    let mut saw_gift = false;
    let mut saw_53009 = false;
    let mut saw_53010 = false;
    for info in &shop_resp.Infos {
        if info.Id == 3 && !info.Items.is_empty() {
            saw_gift = true;
        }
        for it in &info.Items {
            assert_eq!(it.Quality, 0);
            if it.Id == 106_100_007 && it.ItemId == 53009 {
                saw_53009 = true;
            }
            if it.Id == 106_100_008 && it.ItemId == 53010 {
                saw_53010 = true;
            }
        }
    }
    assert!(saw_gift && saw_53009 && saw_53010);

    assert!(!activity::is_activity_unlocked(None, &h.state.assets, 100_100_019));
    assert!(activity::is_activity_unlocked(None, &h.state.assets, 100_300_001));

    h.state.player_components.weapon.weapon_map.insert(
        9001,
        WeaponItem {
            id: 21_050_046,
            level: 60,
            breach: 3,
            reson_level: 1,
            role_id: Some(1506),
            ..Default::default()
        },
    );
    let mut phoebe_info = RoleInfo::create_default(&h.state.assets, 1506).expect("phoebe");
    phoebe_info.level = 60;
    phoebe_info.breakthrough = 3;
    phoebe_info.weapon = 9001;
    let _ = phoebe_info.refresh_scaled_properties(&h.state.assets, 1506);
    h.state.player_components.role.role_map.insert(1506, phoebe_info);

    let inst_dungeon = h
        .state
        .assets
        .tables
        .instance_dungeon
        .get_data_by_id(&8)
        .unwrap()
        .into_owned();
    let phoebe = role_entity_templates::create_role_entity(
        &h.state.fs,
        h.state.scene.as_mut().unwrap(),
        1,
        &h.state.assets,
        &mut h.state.player_components.role,
        &h.state.player_components.weapon,
        &h.state.player_components.echo,
        &inst_dungeon,
        1506,
    )
    .expect("create phoebe entity");

    let mut pack = Vec::new();
    buff_helper::sync_continuous_buff_modifiers(
        &mut pack,
        h.state.scene.as_mut().unwrap(),
        phoebe.net_id,
        &h.state.assets,
    );
    let spectro_dmg = attributes::get_attr_value(
        h.state
            .scene
            .as_ref()
            .unwrap()
            .entity(phoebe.net_id)
            .unwrap()
            .attribute
            .as_ref()
            .unwrap(),
        EAttributeType::DamageChangeElement5,
    );
    assert!(spectro_dmg >= 0);

    for bi in &mut h
        .state
        .scene
        .as_mut()
        .unwrap()
        .entity_mut(phoebe.net_id)
        .unwrap()
        .buffs
        .as_mut()
        .unwrap()
        .buffs
    {
        if bi.BuffId == 1_506_000_020 {
            bi.IsActive = true;
        }
    }
    buff_helper::sync_continuous_buff_modifiers(
        &mut pack,
        h.state.scene.as_mut().unwrap(),
        phoebe.net_id,
        &h.state.assets,
    );
    let spectro_dmg_after = attributes::get_attr_value(
        h.state
            .scene
            .as_ref()
            .unwrap()
            .entity(phoebe.net_id)
            .unwrap()
            .attribute
            .as_ref()
            .unwrap(),
        EAttributeType::DamageChangeElement5,
    );
    assert_eq!(spectro_dmg_after, spectro_dmg + 1200);
}

#[test]
fn audit_106_storage_info_notify_activity_and_month_card_silent_grant() {
    let mut h = TestHarness::new("audit106", None);

    let mut inner_map = BTreeMap::new();
    inner_map.insert(100_000, 1);
    let mut outer_map = BTreeMap::new();
    outer_map.insert(100_200_001, ClientStorageMapData { Data: inner_map });
    let _ = qol::handle_storage_info_update_request(
        &mut h.state,
        StorageInfoUpdateRequest {
            Infos: vec![ClientStorageInfo {
                SystemId: EClientStorageSystemIdType::Activity.to_i32(),
                Data: Some(ClientStorageInfoDataOneof::MapMapData(
                    ClientStorageMapMapData { Data: outer_map },
                )),
            }],
        },
    )
    .expect("storage update");

    role_logic::push_data(
        &mut h.state.conn,
        &h.state.assets,
        &h.state.player_components.basic,
        &mut h.state.player_components.role,
        &mut h.state.player_components.weapon,
        &mut h.state.player_components.echo,
        &h.state.player_components.cosmetic,
        &h.state.player_components.inventory,
        &h.state.player_components.quest,
        &h.state.player_components.progress,
    );
    let notifies = h.drain_decoded::<StorageInfoNotify>(StorageInfoNotify::MSG_ID);
    let saw_splash = notifies.iter().any(|n| {
        n.Infos.iter().any(|inf| {
            inf.SystemId == EClientStorageSystemIdType::Activity.to_i32()
                && matches!(
                    &inf.Data,
                    Some(ClientStorageInfoDataOneof::MapMapData(m))
                        if m.Data.get(&100_200_001).and_then(|im| im.Data.get(&100_000)) == Some(&1)
                )
        })
    });
    assert!(saw_splash);

    let before_astrite = h.state.player_components.inventory.info.normal_item_count(3);
    let mc = qol::handle_month_card_request(&mut h.state, MonthCardRequest::default())
        .expect("month card");
    assert_eq!(mc.ErrorCode, Some(ErrorCode::Success));
    assert_eq!(h.state.player_components.inventory.info.normal_item_count(3), before_astrite + 90);
    assert!(!h.drain_push_msg_ids().contains(&ItemRewardNotify::MSG_ID));
}

#[test]
fn audit_107_trace_quest_manual_untrack_and_resync() {
    let mut h = TestHarness::new("audit107", Some(8));
    let mut events = EventQueue::new();

    h.state
        .player_components
        .quest
        .info
        .get_or_add_active_quest(139_000_025);
    h.state.player_components.quest.info.tracked_quest_id = 139_000_025;

    let cancel = quest::handle_trace_quest_request(
        &mut h.state,
        &mut events,
        TraceQuestRequest {
            QuestId: 139_000_025,
            TraceType: 2,
            Operate: 1,
        },
    )
    .expect("trace cancel");
    assert_eq!(cancel.ErrorId, Some(ErrorCode::Success));
    assert_eq!(h.state.player_components.quest.info.tracked_quest_id, 0);
    assert_eq!(h.state.scene.as_ref().unwrap().current_quest_id, 0);

    let traces = h.drain_decoded::<TraceQuestNotify>(TraceQuestNotify::MSG_ID);
    assert!(traces.iter().any(|t| t.QuestId == 0));

    quest_engine::resync_on_reconnect(&mut h.state.quest_context(&mut events)).expect("resync");
    assert_eq!(h.state.player_components.quest.info.tracked_quest_id, 0);
    h.drain_count();

    quest_engine::sync_on_login(&mut h.state.quest_context(&mut events)).expect("sync login");
    assert_eq!(h.state.player_components.quest.info.tracked_quest_id, 0);
    h.drain_count();

    let track = quest::handle_trace_quest_request(
        &mut h.state,
        &mut events,
        TraceQuestRequest {
            QuestId: 139_000_025,
            TraceType: 1,
            Operate: 1,
        },
    )
    .expect("trace set");
    assert_eq!(track.ErrorId, Some(ErrorCode::Success));
    assert_eq!(h.state.player_components.quest.info.tracked_quest_id, 139_000_025);
    h.drain_count();
}

#[test]
fn audit_108_gm_skip_quest_repair_account_and_gm_i18n() {
    let mut h = TestHarness::new("audit108", Some(8));
    let mut events = EventQueue::new();

    h.state.player_components.quest.info.finished_quests =
        vec![139_000_025, 139_000_026, 140_800_000];
    h.state.player_components.quest.info.env.trial_roles = vec![5_001_001];
    h.state.player_components.quest.info.env.time_locked = true;
    h.state.player_components.quest.info.env.weather_locked = true;
    h.state.player_components.quest.info.world.active_guests = vec![1404];
    h.state.player_components.basic.info.attributes.level = 25;
    h.state.player_components.basic.info.attributes.sex = 1;
    h.state.player_components.basic.info.selected_main_role_id = 0;

    let leaked_trial = RoleInfo::create_default(&h.state.assets, 5_001_001).expect("trial");
    h.state.player_components.role.role_map.insert(5_001_001, leaked_trial);
    if let Some(rover) = h.state.player_components.role.role_map.get_mut(&1501) {
        rover.weapon = 0;
    }

    let rep = quest_engine::reconcile_account_state(&mut h.state.quest_context(&mut events), true)
        .expect("reconcile");
    assert_eq!(rep.repaired_quests, 3);
    assert_eq!(rep.removed_trial_roles, 1);
    assert!(rep.unlocked_locks >= 2);
    assert_eq!(rep.role_summary.removed_leaks, 1);
    assert!(!h.state.player_components.role.role_map.contains_key(&5_001_001));
    assert!(h.state.player_components.quest.info.env.trial_roles.is_empty());
    assert!(!h.state.player_components.quest.info.env.time_locked);
    assert!(!h.state.player_components.quest.info.env.weather_locked);
    assert!(h.state.player_components.quest.info.world.active_guests.is_empty());
    assert_eq!(h.state.player_components.basic.info.selected_main_role_id, 1501);
    assert_ne!(h.state.player_components.role.role_map.get(&1501).unwrap().weapon, 0);

    assert!(h.state.player_components.quest.info.has_finished_child_quest(140_800_000, 330));
    assert!(h.state.player_components.quest.info.has_unlocked_system(10036));
    assert!(h.state.player_components.map.info.unlocked_teleporters.contains(&300_201));
    assert!(
        h.state.player_components.quest.info.has_active_quest(880_000_013)
            || h.state.player_components.quest.info.has_finished_quest(880_000_013)
    );
    h.drain_count();

    let skipped_id = quest_engine::gm_skip_quest(&mut h.state.quest_context(&mut events), Some(880_000_013))
        .expect("gm skip quest");
    assert_eq!(skipped_id, 880_000_013);
    assert!(h.state.player_components.quest.info.has_finished_quest(880_000_013));
    assert!(!h.state.player_components.quest.info.has_active_quest(880_000_013));
    h.drain_count();

    let t_pro = gm_i18n::resolve_quest_title(139_000_025, 1, 1, 1, "万象新声");
    assert!(gm_i18n::is_ascii_printable(&t_pro));
    assert!(t_pro.contains("Utterance of Marvels"));

    let t_abby = gm_i18n::resolve_quest_title(880_000_013, 1, 8, 6, "阿布出现");
    assert!(gm_i18n::is_ascii_printable(&t_abby));
    assert!(t_abby.contains("Abby Appears"));

    let t_fallback = gm_i18n::resolve_quest_title(999_999_999, 9, 2004, 9, "怨鸟泽");
    assert!(gm_i18n::is_ascii_printable(&t_fallback));

    cheat::switch_rover::call(
        &mut events,
        &h.state.fs,
        &h.state.assets,
        &mut h.state.conn,
        h.state.scene.as_mut(),
        &mut h.state.player_components.basic,
        &mut h.state.player_components.role,
        &mut h.state.player_components.weapon,
        &mut h.state.player_components.echo,
        "havoc",
    )
    .expect("switch rover");
    h.drain_count();
    assert_eq!(h.state.player_components.basic.info.selected_main_role_id, 1605);
    assert!(h.state.player_components.role.role_map.contains_key(&1605));
    cheat::repair_account::call(&mut h.state.quest_context(&mut events)).expect("repair account");
    h.drain_count();
}

#[test]
fn audit_109_scene_join_defers_world_sync_until_loading_finish() {
    let mut h = TestHarness::new("audit109", Some(8));
    let mut events = EventQueue::new();

    {
        let sc = h.state.scene.as_mut().unwrap();
        sc.join_scene_pushed = true;
        sc.scene_loading_finished = false;
        sc.deferred_join_sync_pending = true;
    }
    append_unique_int(
        &mut h.state.player_components.quest.info.progress.entered_level_plays,
        135_900_013,
    );
    h.state
        .player_components
        .quest
        .info
        .get_or_add_active_quest(139_000_025)
        .mark_node_active(19);

    let pf = PendingFlow {
        flow_list_name: "TestFlow".to_string(),
        flow_id: 1,
        state_id: 1,
        start_action_id: 0,
        source_quest_id: 0,
        source_quest_child_id: 0,
        pos_sender: 1,
    };
    {
        let fs_c = Arc::clone(&h.state.fs);
        let mut qctx = h.state.quest_context(&mut events);
        let mut dummy_ev = EventQueue::new();
        scene_logic::after_scene_join(Some(&pf), &mut dummy_ev, &fs_c, &mut qctx, 1)
            .expect("after scene join");
    }
    {
        let sc = h.state.scene.as_ref().unwrap();
        assert!(!sc.scene_loading_finished);
        assert!(sc.deferred_join_sync_pending);
        assert!(sc.deferred_join_flow.is_some());
    }
    let after_ids = h.drain_push_msg_ids();
    assert!(after_ids.contains(&AfterJoinSceneNotify::MSG_ID));
    assert!(!after_ids.contains(&FlowStartNotify::MSG_ID));
    assert!(!after_ids.contains(&BehaviorTreeInfoNotify::MSG_ID));
    assert!(!after_ids.contains(&EntityAddNotify::MSG_ID));

    quest_engine::resync_on_reconnect(&mut h.state.quest_context(&mut events)).expect("reconnect");
    let reconn_ids = h.drain_push_msg_ids();
    assert!(reconn_ids.contains(&TraceQuestNotify::MSG_ID));
    assert!(!reconn_ids.contains(&BehaviorTreeInfoNotify::MSG_ID));
    assert!(!reconn_ids.contains(&EntityAddNotify::MSG_ID));

    let fin = crate::network::handlers::scene::handle_scene_loading_finish_request(
        &mut h.state,
        &mut events,
        SceneLoadingFinishRequest::default(),
    )
    .expect("scene loading finish");
    assert_eq!(fin.ErrorCode, Some(ErrorCode::Success));
    {
        let sc = h.state.scene.as_ref().unwrap();
        assert!(sc.scene_loading_finished);
        assert!(!sc.deferred_join_sync_pending);
        assert!(sc.deferred_join_flow.is_none());
    }
    let fin_ids = h.drain_push_msg_ids();
    assert!(fin_ids.contains(&BehaviorTreeInfoNotify::MSG_ID));
    assert!(fin_ids.contains(&FlowStartNotify::MSG_ID));
}

#[test]
fn audit_110_speed_ring_lift_component_and_missing_actions() {
    let mut h = TestHarness::new("audit110", Some(8));
    let mut events = EventQueue::new();

    {
        let mut p = EntityComponentStorage::new(
            1,
            ConfigComponent {
                camp: 0,
                config_id: 0,
                config_type: ConfigType::Character,
                entity_type: EntityType::Player,
                state: EntityState::Default,
            },
        );
        p.buffs = Some(FightBuffComponent::default());
        p.position = Some(PositionComponent::default());
        h.state.scene.as_mut().unwrap().spawn(p);
    }

    let lift_net = {
        let rt = h.state.as_runtime(&mut events, None);
        let sc = rt.scene.as_mut().unwrap();
        entity_cmd::spawn_with_location_and_progress(
            rt.events,
            sc,
            rt.fs,
            rt.assets,
            rt.sink,
            rt.basic_comp,
            122_005_926,
            Some(false),
            Some(false),
            Some(8),
            Some([-47_440.81, 4_137.25, 13_109.63]),
            true,
            &rt.progress_comp.info,
            true,
            true,
        )
        .unwrap()
        .unwrap()
        .net_id
    };
    assert_eq!(
        h.state.scene.as_ref().unwrap().entity(lift_net).unwrap().world_item.lift.unwrap().location,
        1
    );
    let lift_pb = h
        .state
        .scene
        .as_ref()
        .unwrap()
        .entity(lift_net)
        .unwrap()
        .to_proto(&h.state.assets, 0);
    assert!(lift_pb.ComponentPbs.iter().any(|c| matches!(&c.ComponentPb, Some(EntityComponentPbComponentPbOneof::LiftComponentPb(_)))));

    let cl_json: Value = serde_json::from_str(
        r#"{"Name":"ChangeLiftTarget","Params":{"EntityId":122005926,"Location":2}}"#,
    )
    .unwrap();
    {
        let mut qctx = h.state.quest_context(&mut events);
        let mut cbs = ActionCallbacks::default();
        execute_action(&mut qctx, &mut cbs, 0, 0, &cl_json).expect("change lift");
    }
    assert_eq!(
        h.state.scene.as_ref().unwrap().entity(lift_net).unwrap().world_item.lift.unwrap().location,
        2
    );
    h.drain_count();

    let _ = map::handle_player_access_range_request(
        &mut h.state,
        &mut events,
        PlayerAccessRangeRequest {
            RangeEntityId: 131_900_002,
            AccessRangeType: Some(AccessRangeType::RangeEnter),
        },
    )
    .expect("speed ring");
    let mut saw_speed_buff = false;
    while let Some(ev) = events.dequeue() {
        if let Event::BuffAddition { buffs, .. } = ev
            && buffs.iter().any(|b| b.id == 640_007_012)
        {
            saw_speed_buff = true;
        }
    }
    assert!(saw_speed_buff);
    h.drain_count();

    let editor_cfg = LevelEntityConfig {
        Id: 900_001,
        EntityId: 900_001,
        BlueprintType: "Editor001".to_string(),
        MapId: 8,
        AreaId: 1,
        ..Default::default()
    };
    assert!(scene_logic::is_blocked_world_entity_config(
        &h.state.assets,
        &h.state.player_components.quest.info,
        8,
        &editor_cfg,
        false,
        false
    ));
    assert!(!scene_logic::is_blocked_world_entity_config(
        &h.state.assets,
        &h.state.player_components.quest.info,
        8,
        &editor_cfg,
        true,
        false
    ));

    let disabled_cfg = LevelEntityConfig {
        Id: 900_002,
        EntityId: 900_002,
        BlueprintType: "Monster001".to_string(),
        MapId: 8,
        AreaId: 1,
        ComponentsData: Components {
            BaseInfoComponent: Some(BaseInfoComponent {
                Disabled: Some(true),
                ..Default::default()
            }),
            ..Default::default()
        },
        ..Default::default()
    };
    assert!(scene_logic::is_blocked_world_entity_config(
        &h.state.assets,
        &h.state.player_components.quest.info,
        8,
        &disabled_cfg,
        false,
        false
    ));
    assert!(!scene_logic::is_blocked_world_entity_config(
        &h.state.assets,
        &h.state.player_components.quest.info,
        8,
        &disabled_cfg,
        false,
        true
    ));

    let passerby_spawner_cfg = LevelEntityConfig {
        Id: 900_003,
        EntityId: 900_003,
        BlueprintType: "Gameplay581".to_string(),
        MapId: 8,
        AreaId: 1,
        ..Default::default()
    };
    assert!(!scene_logic::is_blocked_world_entity_config(
        &h.state.assets,
        &h.state.player_components.quest.info,
        8,
        &passerby_spawner_cfg,
        false,
        false
    ));

    let blocked_spawner_cfg = LevelEntityConfig {
        Id: 900_004,
        EntityId: 900_004,
        BlueprintType: "Gameplay101".to_string(),
        MapId: 8,
        AreaId: 1,
        ..Default::default()
    };
    assert!(scene_logic::is_blocked_world_entity_config(
        &h.state.assets,
        &h.state.player_components.quest.info,
        8,
        &blocked_spawner_cfg,
        false,
        false
    ));

    h.state.scene.as_mut().unwrap().active_flow_owner_entity_id = lift_net;
    for raw in [
        r#"{"Name":"ToggleEntityElementalAbility","Params":{"EntityId":0,"Enable":false}}"#,
        r#"{"Name":"RogueGotoNextFloor","Params":{}}"#,
        r#"{"Name":"DangoAbyssGotoNextFloor","Params":{}}"#,
        r#"{"Name":"HonamiStoryReceiveCorruptedChestReward","Params":{"Cost":0}}"#,
    ] {
        let act: Value = serde_json::from_str(raw).unwrap();
        let mut qctx = h.state.quest_context(&mut events);
        let mut cbs = ActionCallbacks::default();
        execute_action(&mut qctx, &mut cbs, 0, 0, &act).unwrap();
    }
    assert_eq!(
        h.state.scene.as_ref().unwrap().entity(lift_net).unwrap().world_item.elemental_holder_enable,
        Some(false)
    );
    assert_eq!(h.state.player_components.progress.info.get_key_int("rogue:floor"), 1);
    assert_eq!(h.state.player_components.progress.info.get_key_int("abyss:floor"), 1);
    assert_eq!(h.state.player_components.progress.info.get_key_int("honami:corrupted_chest_cnt"), 1);
    assert!(h.state.player_components.quest.info.has_opened_chest(122_005_926));

    let dself: Value = serde_json::from_str(r#"{"Name":"DestroySelf","Params":{}}"#).unwrap();
    {
        let mut qctx = h.state.quest_context(&mut events);
        let mut cbs = ActionCallbacks::default();
        execute_action(&mut qctx, &mut cbs, 0, 0, &dself).unwrap();
    }
    assert!(h.state.scene.as_ref().unwrap().find_entity_net_id_by_config_id(122_005_926).is_none());
    h.drain_count();
}

#[test]
fn audit_111_clock_weather_collect_lock_reset_point_and_filter_parity() {
    let mut h = TestHarness::new("audit111", Some(8));
    let mut events = EventQueue::new();

    {
        let mut p = EntityComponentStorage::new(
            1,
            ConfigComponent {
                camp: 0,
                config_id: 0,
                config_type: ConfigType::Character,
                entity_type: EntityType::Player,
                state: EntityState::Default,
            },
        );
        p.buffs = Some(FightBuffComponent::default());
        p.position = Some(PositionComponent::default());
        h.state.scene.as_mut().unwrap().spawn(p);
    }

    let var_mgr = LevelEntityConfig {
        Id: 900_101,
        EntityId: 900_101,
        BlueprintType: "Gameplay097".to_string(),
        MapId: 8,
        AreaId: 1,
        ..Default::default()
    };
    assert!(!scene_logic::is_blocked_world_entity_config(
        &h.state.assets,
        &h.state.player_components.quest.info,
        8,
        &var_mgr,
        false,
        false
    ));
    assert!(is_spawnable_item_config(&var_mgr, BpEntityType::Monitor));
    assert!(!is_spawnable_item_config(&var_mgr, BpEntityType::VarManager));

    h.state.player_components.quest.info.env.time_locked = true;
    h.state.player_components.quest.info.env.time_lock_hour = 20;
    h.state.player_components.quest.info.env.time_lock_minute = 0;
    h.state.player_components.quest.info.env.weather_id = 3;

    let clock_wrap: Value = serde_json::from_str(r#"{"Type":"Clock","Compare":"Eq","Start":18,"End":2}"#).unwrap();
    let clock_day: Value = serde_json::from_str(r#"{"Type":"Clock","Compare":"Eq","Start":6,"End":16}"#).unwrap();
    let weather: Value = serde_json::from_str(r#"{"Type":"Weather","Compare":"Ne","WeatherId":2}"#).unwrap();
    assert!(eval_single_inline_condition(&h.state.eval_context(), &clock_wrap));
    assert!(!eval_single_inline_condition(&h.state.eval_context(), &clock_day));
    assert!(eval_single_inline_condition(&h.state.eval_context(), &weather));

    h.state.player_components.quest.info.set_entity_state(14_000_359, "关卡.时间回放.状态5");
    h.state.player_components.quest.info.set_entity_state(14_000_360, "关卡.时间回放.状态7");
    let grp_cond: Value = serde_json::from_str(
        r#"{"Type":"CompareEntityGroupState","GroupCondition":{"Conditions":[{"EntityId":14000359,"State":"关卡.时间回放.状态5"},{"EntityId":14000360,"State":"关卡.时间回放.状态7"}],"Count":2,"Compare":"Eq"}}"#,
    )
    .unwrap();
    assert!(eval_single_inline_condition(&h.state.eval_context(), &grp_cond));

    let sun_avail: Value = serde_json::from_str(
        r#"{"Type":"CheckAvailableSunSpiritCount","AreaId":36106,"Compare":"Ge","Count":1}"#,
    )
    .unwrap();
    assert!(eval_single_inline_condition(&h.state.eval_context(), &sun_avail));

    let spawn_one = |state: &mut crate::network::state::State, ev: &mut EventQueue, cfg_id: i64, map_id: Option<i32>, pos: [f32; 3]| -> i64 {
        let rt = state.as_runtime(ev, None);
        let sc = rt.scene.as_mut().unwrap();
        entity_cmd::spawn_with_location_and_progress(
            rt.events,
            sc,
            rt.fs,
            rt.assets,
            rt.sink,
            rt.basic_comp,
            cfg_id,
            Some(false),
            Some(false),
            map_id,
            Some(pos),
            true,
            &rt.progress_comp.info,
            true,
            true,
        )
        .unwrap()
        .unwrap()
        .net_id
    };

    let sun_net = spawn_one(&mut h.state, &mut events, 939_000_138, None, [0.0; 3]);
    h.state.scene.as_mut().unwrap().active_flow_owner_entity_id = sun_net;
    for raw in [
        r#"{"Name":"SetInteractionLockState","Params":{"IsLock":true}}"#,
        r#"{"Name":"CollectSunSpirit","Params":{}}"#,
    ] {
        let act: Value = serde_json::from_str(raw).unwrap();
        let mut qctx = h.state.quest_context(&mut events);
        let mut cbs = ActionCallbacks::default();
        execute_action(&mut qctx, &mut cbs, 0, 0, &act).unwrap();
    }
    assert!(h.state.player_components.quest.info.world.locked_entities.contains(&939_000_138));
    assert!(h.state.player_components.progress.info.has_claim("sun_spirit:939000138"));
    assert!(h.state.scene.as_ref().unwrap().find_entity_net_id_by_config_id(939_000_138).is_none());

    let col_net = spawn_one(&mut h.state, &mut events, 11_000_249, Some(8), [-49_500.0, 11_000.0, 1600.0]);
    h.state.scene.as_mut().unwrap().active_flow_owner_entity_id = col_net;
    for raw in [
        r#"{"Name":"Collect","Params":{}}"#,
        r#"{"Name":"RemoveBuffToTriggeredEntity","Params":{"BuffIds":[640012025]}}"#,
    ] {
        let act: Value = serde_json::from_str(raw).unwrap();
        let mut qctx = h.state.quest_context(&mut events);
        let mut cbs = ActionCallbacks::default();
        execute_action(&mut qctx, &mut cbs, 0, 0, &act).unwrap();
    }
    assert!(h.state.scene.as_ref().unwrap().find_entity_net_id_by_config_id(11_000_249).is_none());

    let rp_net = spawn_one(&mut h.state, &mut events, 122_005_926, Some(8), [-47_440.0, 4_137.0, 13_109.0]);
    h.state.scene.as_mut().unwrap().active_flow_owner_entity_id = rp_net;
    {
        let act: Value = serde_json::from_str(r#"{"Name":"ActivateResetPoint","Params":{}}"#).unwrap();
        let mut qctx = h.state.quest_context(&mut events);
        let mut cbs = ActionCallbacks::default();
        execute_action(&mut qctx, &mut cbs, 0, 0, &act).unwrap();
    }
    assert_eq!(h.state.player_components.progress.info.get_key_int("reset_point:cfg"), 122_005_926);
    let _ = update_player_scene_state(&mut h.state.quest_context(&mut events), [1234.0, 5678.0, 90.0], None);
    {
        let act: Value = serde_json::from_str(
            r#"{"Name":"TeleportToLatestResetPoint","Params":{"Option":{"Type":"Directly"}}}"#,
        )
        .unwrap();
        let mut qctx = h.state.quest_context(&mut events);
        let mut cbs = ActionCallbacks::default();
        execute_action(&mut qctx, &mut cbs, 0, 0, &act).unwrap();
    }
    assert_ne!(h.state.player_components.progress.info.get_key_int("reset_point:x"), 0);
    h.drain_count();
}

#[test]
fn audit_112_visible_conditions_condition_listener_auto_interact_and_client_trigger() {
    let mut h = TestHarness::new("audit112", Some(8));
    let mut events = EventQueue::new();

    {
        let mut p = EntityComponentStorage::new(
            1,
            ConfigComponent {
                camp: 0,
                config_id: 0,
                config_type: ConfigType::Character,
                entity_type: EntityType::Player,
                state: EntityState::Default,
            },
        );
        p.buffs = Some(FightBuffComponent::default());
        p.position = Some(PositionComponent::default());
        h.state.scene.as_mut().unwrap().spawn(p);
    }

    h.state.player_components.quest.info.env.time_locked = true;
    h.state.player_components.quest.info.env.time_lock_hour = 21;
    h.state.player_components.quest.info.env.time_lock_minute = 0;

    let vis_night: Value = serde_json::from_str(
        r#"[{"Conditions":[{"Type":"TimePeriod","TimePeriod":"Night","Compare":"Eq"},{"Type":"PreQuest","PreQuest":140000100,"Compare":"Ne"}]}]"#,
    )
    .unwrap();
    let comps_vis = Components {
        extra: vec![ExtraEntry::EntityVisibleComponent(Box::new(
            EntityVisibleComponent {
                VisibleConditions: Some(vis_night),
                ..Default::default()
            },
        ))],
        ..Default::default()
    };
    assert!(eval_entity_visible_component(&h.state.eval_context(), &comps_vis));
    h.state.player_components.quest.info.finish_quest(140_000_100);
    assert!(!eval_entity_visible_component(&h.state.eval_context(), &comps_vis));

    let lcfg_night = h
        .state
        .assets
        .find_level_entity(8, 152_700_181)
        .expect("missing night audio entity")
        .into_owned();
    assert!(eval_entity_visible_component(&h.state.eval_context(), &lcfg_night.ComponentsData));
    h.state.player_components.quest.info.env.time_lock_hour = 12;
    assert!(!eval_entity_visible_component(&h.state.eval_context(), &lcfg_night.ComponentsData));
    h.state.player_components.quest.info.env.time_lock_hour = 21;

    h.state.player_components.progress.info.set_key_int("tag_ent:55112:3001", 1);
    h.state.player_components.progress.info.set_key_str("ai_state:55112", "Alert");
    h.state.player_components.progress.info.set_key_int("alert_enable:7001", 1);

    let tag_cond: Value = serde_json::from_str(
        r#"{"Type":"CheckEntityHasSceneItemAttributeTag","CheckType":0,"Tags":[3001]}"#,
    )
    .unwrap();
    let restr_cond: Value = serde_json::from_str(
        r#"{"Type":"CheckPlayerStateRestriction","RestrictionId":750001}"#,
    )
    .unwrap();
    let alert_cond: Value = serde_json::from_str(
        r#"{"Type":"CheckAlertAreaEnabled","AreaId":7001}"#,
    )
    .unwrap();
    let ai_cond: Value = serde_json::from_str(
        r#"{"Type":"CheckAiState","StateType":"Alert","Compare":"Eq"}"#,
    )
    .unwrap();

    {
        let mut ent_ctx = h.state.eval_context();
        ent_ctx.self_entity_config_id = 55112;
        assert!(eval_single_inline_condition(&ent_ctx, &tag_cond));
        assert!(eval_single_inline_condition(&ent_ctx, &alert_cond));
        assert!(eval_single_inline_condition(&ent_ctx, &ai_cond));
    }
    assert!(eval_single_inline_condition(&h.state.eval_context(), &restr_cond));
    h.state.scene.as_mut().unwrap().player_in_battle = true;
    assert!(!eval_single_inline_condition(&h.state.eval_context(), &restr_cond));
    h.state.scene.as_mut().unwrap().player_in_battle = false;

    let spawn_one = |state: &mut crate::network::state::State, ev: &mut EventQueue, cfg_id: i64, pos: [f32; 3]| -> i64 {
        let rt = state.as_runtime(ev, None);
        let sc = rt.scene.as_mut().unwrap();
        entity_cmd::spawn_with_location_and_progress(
            rt.events,
            sc,
            rt.fs,
            rt.assets,
            rt.sink,
            rt.basic_comp,
            cfg_id,
            Some(false),
            Some(false),
            Some(8),
            Some(pos),
            true,
            &rt.progress_comp.info,
            true,
            true,
        )
        .unwrap()
        .unwrap()
        .net_id
    };

    let _ = spawn_one(&mut h.state, &mut events, 2_900_002, [-43_990.0, 299_000.0, 5550.0]);
    quest_engine::on_condition_state_changed(&mut h.state.quest_context(&mut events)).unwrap();
    assert!(h.state.player_components.quest.info.get_entity_state(2_900_002).is_none());
    h.state
        .player_components
        .quest
        .info
        .record_child_quest_finished(106_000_002, 59);
    quest_engine::on_condition_state_changed(&mut h.state.quest_context(&mut events)).unwrap();
    assert_eq!(
        h.state.player_components.quest.info.get_entity_state(2_900_002),
        Some("关卡.Common.状态.完成")
    );

    h.state
        .player_components
        .quest
        .info
        .set_entity_state(109_005_550, "关卡.Common.状态.常态");
    let _ = spawn_one(&mut h.state, &mut events, 109_005_550, [-22_386.98, 267_128.44, 1251.0]);
    quest_engine::on_player_move(
        &mut h.state.quest_context(&mut events),
        -22_386.98,
        267_128.44,
        1251.0,
    )
    .unwrap();
    assert_eq!(
        h.state.player_components.quest.info.get_entity_state(109_005_550),
        Some("关卡.Common.状态.激活")
    );

    let cat_tpl = h
        .state
        .assets
        .find_template_config("Gameplay042")
        .expect("missing Gameplay042");
    let cat_interact = cat_tpl
        .ComponentsData
        .InteractComponent
        .as_ref()
        .expect("cat interact");
    assert_eq!(cat_interact.Range, Some(50));

    let ct_net = spawn_one(&mut h.state, &mut events, 29_750_002, [1_184_055.0, 1_286_034.8, 23_756.94]);
    let _ = map::handle_player_access_range_request(
        &mut h.state,
        &mut events,
        PlayerAccessRangeRequest {
            RangeEntityId: ct_net,
            AccessRangeType: Some(AccessRangeType::RangeEnter),
        },
    )
    .unwrap();

    let ct_enter: Value = serde_json::from_str(
        r#"{"Actions":[{"Name":"ChangeEntityState","Params":{"Type":"Directly","EntityId":991123,"State":"TriggeredEnter"}}]}"#,
    )
    .unwrap();
    quest_engine::triggers::handle_client_trigger_actions(
        &mut h.state.quest_context(&mut events),
        ct_net,
        991_123,
        &ClientTriggerComponent {
            OnTriggerEnter: Some(ct_enter),
            ..Default::default()
        },
        true,
    )
    .unwrap();
    assert_eq!(
        h.state.player_components.quest.info.get_entity_state(991_123),
        Some("TriggeredEnter")
    );
    h.drain_count();
}

#[test]
fn audit_113_save_block_destructible_do_interact_combat_state_and_spawn_state() {
    use sonoro_gamesv_logic::commands::entity as entity_cmd;
    use sonoro_gamesv_logic::handlers::scene as scene_logic;
    use sonoro_gamesv_logic::helpers::destructible;
    use sonoro_gamesv_logic::helpers::save_block;

    let mut h = TestHarness::new("audit113", Some(8));
    let mut events = EventQueue::new();

    // 1) Destructible item: DurabilityValue in EntityPb + WeaponDamage in ThrowDamageRequest
    let (ore_cfg_id, ore_dur) = {
        let mut found = None;
        for &idx in h.state.assets.indexes.spawnable_indices_for_map(8) {
            let Some(cfg) = h.state.assets.get_level_entity_by_idx(idx) else {
                continue;
            };
            if cfg.BlueprintType == "Collect504" {
                let tpl = h.state.assets.find_template_config(&cfg.BlueprintType);
                let dur = cfg
                    .ComponentsData
                    .DestructibleItem
                    .as_ref()
                    .and_then(|d| d.Durability)
                    .or_else(|| {
                        tpl.and_then(|t| t.ComponentsData.DestructibleItem.as_ref()?.Durability)
                    })
                    .unwrap_or(destructible::DEFAULT_DURABILITY);
                found = Some((cfg.EntityId, dur));
                break;
            }
        }
        found.expect("expected Collect504 ore entity on map 8")
    };

    entity_cmd::spawn_with_location(
        &mut events,
        h.state.scene.as_mut().unwrap(),
        &h.state.fs,
        &h.state.assets,
        &mut h.state.conn,
        &h.state.player_components.basic,
        ore_cfg_id,
        Some(false),
        Some(false),
        Some(8),
        Some([0.0, 0.0, 0.0]),
        true,
    )
    .expect("spawn ore");
    let ore_net = h
        .state
        .scene
        .as_ref()
        .unwrap()
        .find_entity_net_id_by_config_id(i32::try_from(ore_cfg_id).unwrap())
        .expect("ore net id");
    let pb_ore = h
        .state
        .scene
        .as_ref()
        .unwrap()
        .entity(ore_net)
        .unwrap()
        .to_proto(&h.state.assets, 0);
    assert_eq!(pb_ore.DurabilityValue, ore_dur);
    assert!(pb_ore.DurabilityValue > 0);

    // Equip GreatSword role (WeaponType 1) if present in RoleInfo and verify ThrowDamageRequest updates durability.
    if let Some(gs_role) = h
        .state
        .assets
        .tables
        .role_info
        .items
        .iter()
        .find(|r| r.WeaponType == 1)
    {
        let sc = h.state.scene.as_mut().unwrap();
        sc.formation_info.cur_formation = 0;
        sc.formation_info.formations[0].cur_role = gs_role.Id;
    }
    let resp = combat_sync::handle_throw_damage_request(
        &mut h.state,
        &mut events,
        sonoro_proto::pb::ThrowDamageRequest {
            EntityId: ore_net,
            ..Default::default()
        },
    )
    .expect("throw damage");
    assert_eq!(resp.ErrorCode, Some(ErrorCode::Success));
    let saved_after_hit = h
        .state
        .player_components
        .progress
        .info
        .get_key_int(&format!("durability:{ore_cfg_id}"));
    assert!(saved_after_hit < i64::from(ore_dur));

    // 2) SaveConfig login discard + PosRollbackCfg in on_initial_scene_join + re-entry in on_scene_loading_finish
    let nodes = h
        .state
        .assets
        .quest_nodes
        .get_owner_nodes(139_000_025)
        .expect("prologue quest nodes");
    let save_node_with_child = nodes.iter().find_map(|n| {
        let block_id = save_block::find_enclosing_save_node(&nodes, n.id)?;
        (n.id != block_id && n.node_type == "ChildQuest").then_some((block_id, n.id))
    });
    if let Some((block_id, child_id)) = save_node_with_child {
        let aq = h.state.player_components.quest.info.get_or_add_active_quest(139_000_025);
        aq.active_nodes = vec![child_id];
        aq.finished_nodes.push(block_id + 1_000_000); // outside sanity
        quest_engine::sync_on_login(&mut h.state.quest_context(&mut events)).expect("sync_on_login");
        assert!(
            h.state
                .player_components
                .quest
                .login_restore
                .reenter_quests
                .contains(&139_000_025)
        );
        // advance_quest_tree must stay deferred until scene loading finishes
        let active_before = h
            .state
            .player_components
            .quest
            .info
            .find_active_quest(139_000_025)
            .unwrap()
            .active_nodes
            .clone();
        assert!(!active_before.contains(&child_id));
        quest_engine::on_scene_loading_finish(&mut h.state.quest_context(&mut events))
            .expect("scene loading finish");
        assert!(
            !h.state
                .player_components
                .quest
                .login_restore
                .reenter_quests
                .contains(&139_000_025)
        );
    }

    // 3) notify_join_scene applies saved quest entity state onto persisted scene entities before AoiData proto
    h.state
        .scene
        .as_mut()
        .unwrap()
        .instance
        .players
        .push(sonoro_gamesv_fs::scene_instance::ScenePlayer {
            id: 1,
            group_type: 1,
            location: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
        });
    h.state
        .player_components
        .quest
        .info
        .set_entity_state(ore_cfg_id, "关卡.Common.状态.激活");
    scene_logic::notify_join_scene(
        None,
        &mut events,
        &h.state.fs,
        &h.state.assets,
        &mut h.state.conn,
        1,
        &mut h.state.player_components.scene,
        &h.state.player_components.basic,
        &mut h.state.player_components.role,
        &mut h.state.player_components.weapon,
        &h.state.player_components.motor,
        &mut h.state.player_components.echo,
        &h.state.player_components.inventory,
        &h.state.player_components.map,
        &h.state.player_components.quest,
        &h.state.player_components.progress,
        h.state.scene.as_mut().unwrap(),
        &mut h.state.buff_timers,
    )
    .expect("notify_join_scene");
    let activated_tag = sonoro_gamesv_logic::helpers::gameplay_tags::id_from_name(
        &h.state.assets.tables.gameplay_tag_remap,
        "关卡.Common.状态.激活",
    )
    .expect("activated tag id");
    let ore_state_tag = h
        .state
        .scene
        .as_ref()
        .unwrap()
        .entity(ore_net)
        .unwrap()
        .state_tag
        .as_ref()
        .map(|st| st.state_tag_id);
    assert_eq!(ore_state_tag, Some(activated_tag));
    h.drain_count();
}

#[test]
fn audit_114_boss_teleport_battle_lock_chest_owner_trace_gacha_trophy_and_quest_fixes() {
    let mut h = TestHarness::new("audit96", Some(8));
    let mut events = EventQueue::new();

    // 1. Scene::init sets non-zero owner_id so treasure chests allow client interaction.
    assert_eq!(h.state.scene.as_ref().unwrap().instance.owner_id, 1);

    // 2. Boss/challenge MapMarks do not hijack EntityMapMarkInfo, and quick-transfer teleporters (Type == 3)
    // are included in TeleportDataResponse.Ids.
    let prev_level = h.state.player_components.basic.info.attributes.level;
    h.state.player_components.basic.info.attributes.level = 10;
    h.state
        .player_components
        .quest
        .info
        .finish_quest(quest_engine::PROLOGUE_QUEST_ID);
    let mark_notify = misc_logic::build_map_mark_info_notify(
        &h.state.assets,
        &h.state.player_components.map,
        &h.state.player_components.basic,
        &h.state.player_components.quest,
    );
    h.state.player_components.basic.info.attributes.level = prev_level;
    h.state
        .player_components
        .quest
        .info
        .finished_quests
        .retain(|&qid| qid != quest_engine::PROLOGUE_QUEST_ID);
    assert!(mark_notify.EntityMapMarkInfo.is_empty());
    assert!(!mark_notify.UnlockMarkIds.is_empty());

    let tp_data = teleport::handle_teleport_data_request(&mut h.state, TeleportDataRequest {})
        .expect("teleport_data");
    let boss_quick_tp = h
        .state
        .assets
        .tables
        .teleporter
        .items
        .iter()
        .find(|t| t.Type == 3)
        .expect("at least one Type == 3 teleporter");
    assert!(tp_data.Ids.contains(&boss_quick_tp.Id));

    // 3. Non-lethal damage on a player role never marks that player entity as active_battle_entities;
    // revive_and_heal_formation and same-map teleports also clear battle state.
    let player_net = {
        let sc = h.state.scene.as_mut().unwrap();
        let mut attr = AttributeComponent::default();
        attr.set_prop(EAttributeType::LifeMax, 10_000);
        attr.set_prop(EAttributeType::Life, 10_000);
        let mut s = EntityComponentStorage::new(
            0,
            ConfigComponent {
                camp: 1,
                config_id: 1102,
                entity_type: EntityType::Player,
                config_type: ConfigType::Character,
                state: EntityState::Default,
            },
        );
        s.attribute = Some(attr);
        sc.spawn(s).net_id
    };
    let attacker_net = {
        let sc = h.state.scene.as_mut().unwrap();
        sc.spawn(EntityComponentStorage::new(
            0,
            ConfigComponent {
                camp: 2,
                config_id: 9001,
                entity_type: EntityType::Monster,
                config_type: ConfigType::Level,
                state: EntityState::Default,
            },
        ))
        .net_id
    };
    let dmg_cfg = Damage {
        Id: 99_961,
        FormulaType: 0,
        RateLv: vec![0; 10],
        CalculateType: 0,
        Element: 0,
        ..Default::default()
    };
    let mut pack = Vec::new();
    let _ = damage_helper::damage_entity(
        &mut pack,
        &DamageExecuteRequest {
            DamageId: dmg_cfg.Id,
            SkillLevel: 1,
            AttackerEntityId: attacker_net,
            TargetEntityId: player_net,
            RandomSeed: 1,
            ..Default::default()
        },
        &dmg_cfg,
        &h.state.assets,
        h.state.scene.as_mut().unwrap(),
        None,
    );
    {
        let sc = h.state.scene.as_ref().unwrap();
        assert!(!sc.active_battle_entities.contains(&player_net));
        assert!(!sc.player_in_battle);
    }

    h.state.scene.as_mut().unwrap().force_battle_state(true);
    assert!(h.state.scene.as_ref().unwrap().player_in_battle);
    {
        let sc = h.state.scene.as_mut().unwrap();
        let mut sink = sonoro_gamesv_logic::sink::VecPushSink::default();
        role_helper::revive_and_heal_formation(
            &mut sink,
            sc,
            &mut h.state.player_components.role,
            &mut events,
            &h.state.fs,
        );
        assert!(!sc.player_in_battle);
    }

    if let Some(map8_tp) = h
        .state
        .assets
        .tables
        .teleporter
        .items
        .iter()
        .find(|t| t.MapId == 8 && t.TeleportEntityConfigId > 0)
        .map(|t| t.Id)
    {
        h.state.scene.as_mut().unwrap().force_battle_state(true);
        let tp_resp = teleport::handle_teleport_transfer_request(
            &mut h.state,
            &mut events,
            TeleportTransferRequest { Id: map8_tp },
        )
        .expect("same-map teleport");
        assert_eq!(tp_resp.ErrorCode, Some(ErrorCode::Success));
        assert!(!h.state.scene.as_ref().unwrap().player_in_battle);
    }

    // 4. Achievement/Trophy list emits unfinished achievements with TotalProgress = 1 so UI groups populate.
    let ach_resp =
        achievement::handle_achievement_info_request(&mut h.state, AchievementInfoRequest {})
            .expect("achievement_info");
    assert!(!ach_resp.AchievementGroupInfoList.is_empty());
    let first_nonempty = ach_resp
        .AchievementGroupInfoList
        .iter()
        .find(|g| !g.AchievementEntryList.is_empty())
        .expect("at least one trophy group has entries");
    assert!(
        first_nonempty
            .AchievementEntryList
            .iter()
            .any(|e| e.FinishTime == 0 && e.Progress.as_ref().is_some_and(|p| p.TotalProgress == 1))
    );

    // 5. Gacha info & detail fixes: WeaponCommon (5) & NewPlayerCustom (6) resources_id + 5-star characters in FiveStarRoles.
    let gacha_info = gacha::handle_gacha_info_request(
        &mut h.state,
        GachaInfoRequest { Language: 0 },
    )
    .expect("gacha_info");
    assert!(
        gacha_info
            .GachaInfos
            .iter()
            .any(|g| g.Id == 3 && !g.ResourcesId.is_empty())
    );
    assert!(
        gacha_info
            .GachaInfos
            .iter()
            .any(|g| (g.Id == 4 || g.Id == 5) && !g.ResourcesId.is_empty())
    );
    let detail_resp = gacha::handle_gacha_pool_detail_request(
        &mut h.state,
        GachaPoolDetailRequest { PoolId: 1000011 },
    );
    if let Ok(detail) = detail_resp
        && detail.ErrorCode == Some(ErrorCode::Success)
        && let Some(pool) = detail.GachaPoolDetail
    {
        assert!(!pool.FiveStarRoles.is_empty());
    }

    // 6. EntityPositionRequest (wire 21153 = TraceEntityPosRequest) returns real LevelEntity world position instead of (0,0,0).
    let map8_tp_cfg_id = h
        .state
        .assets
        .tables
        .teleporter
        .items
        .iter()
        .find(|t| t.MapId == 8 && t.TeleportEntityConfigId > 0)
        .expect("map 8 teleporter entity")
        .TeleportEntityConfigId as i32;
    let trace_resp = scene::handle_entity_position_request(
        &mut h.state,
        EntityPositionRequest {
            DungeonInstanceId: 8,
            ConfigId: map8_tp_cfg_id,
        },
    )
    .expect("entity_position");
    assert_eq!(trace_resp.ErrorCode, Some(ErrorCode::Success));
    let pos = trace_resp.Pos.expect("EntityPositionResponse.Pos");
    assert!(pos.X != 0.0 || pos.Y != 0.0 || pos.Z != 0.0);
    h.drain_count();
}

#[test]
fn audit_115_comprehensive_server_fixes() {
    let mut h = TestHarness::new("audit_115", Some(8));
    h.drain_count();

    // 1. PayInfo & MonsterDetection return real Asset table rows.
    let pay_info = minimal::handle_pay_info_request(&mut h.state, PayInfoRequest { Version: Vec::new() })
        .expect("pay_info");
    assert_eq!(pay_info.ErrorCode, Some(ErrorCode::Success));
    assert!(!pay_info.Infos.is_empty(), "PayItem table should populate PayInfoResponse");

    let det_info = minimal::handle_monster_detection_info_request(
        &mut h.state,
        MonsterDetectionInfoRequest { Params: Vec::new() },
    )
    .expect("monster_detection");
    assert!(!det_info.DetectionTarget.is_empty());

    // 2. Chest Drops & DropShowPlan: chest drop 1001 grants real rewards with ShowPlanId > 0 and non-modal chest reason.
    {
        let pcs = &mut h.state.player_components;
        let mut rctx = RewardContext {
            fs: &h.state.fs,
            assets: &h.state.assets,
            sink: &mut h.state.conn,
            basic_comp: &mut pcs.basic,
            inventory_comp: &mut pcs.inventory,
            weapon_comp: &mut pcs.weapon,
            echo_comp: &mut pcs.echo,
            progress_comp: Some(&mut pcs.progress),
        };
        let res = reward::grant_treasure_box_reward(&mut rctx, 1, 1001, 0).expect("chest drop");
        assert!(res.granted_count > 0, "tier override should yield non-empty chest rewards");
    }
    let notifies = h.drain_decoded::<ItemRewardNotify>(ItemRewardNotify::MSG_ID);
    assert!(!notifies.is_empty(), "chest should emit ItemRewardNotify");
    assert_eq!(notifies[0].Reason, reward::CHEST_REWARD_REASON);
    assert!(
        notifies[0]
            .RewardItems
            .values()
            .flat_map(|l| &l.ItemList)
            .any(|ri| ri.ShowPlanId > 0 && ri.Count > 0),
        "non-currency / chest drop items must carry valid DropShowPlan ShowPlanId"
    );

    // 3. EntityGroupComponent index populated from BinData/db.
    assert!(
        !h.state.assets.indexes.entity_groups_by_member.is_empty(),
        "entity_groups_by_member index should contain EntityGroupComponent members"
    );

    // 4. Quest actions: SetPlayerOperationRestriction (15930), CreatePrefab/DestroyPrefab, SummonEntity/DestroySummonedEntities.
    let tpl_id = h
        .state
        .assets
        .tables
        .blueprint_config
        .items
        .iter()
        .find(|b| {
            matches!(
                b.EntityLogic,
                sonoro_gamesv_data::tables::blueprint_config::EntityLogic::Monster
                    | sonoro_gamesv_data::tables::blueprint_config::EntityLogic::Npc
            ) && h.state.assets.find_template_config(&b.BlueprintType).is_some()
        })
        .map(|b| b.BlueprintType.clone())
        .expect("spawnable blueprint with template_config");
    let prefab_id = h
        .state
        .assets
        .tables
        .prefab_config
        .items
        .first()
        .map_or(610_000_000, |p| p.Id);
    let mut events = EventQueue::new();
    {
        let mut qctx = h.state.quest_context(&mut events);
        let mut cbs = ActionCallbacks::default();
        let restriction_action = serde_json::json!({
            "Name": "SetPlayerOperationRestriction",
            "Params": { "RestrictionType": 1, "IsDisable": true }
        });
        execute_action(&mut qctx, &mut cbs, 1, 1, &restriction_action)
            .expect("SetPlayerOperationRestriction");

        let prefab_action = serde_json::json!({
            "Name": "CreatePrefab",
            "Params": { "Config": { "PrefabId": prefab_id }, "VarName": { "Name": "TestPrefab" } }
        });
        execute_action(&mut qctx, &mut cbs, 1, 1, &prefab_action)
            .expect("CreatePrefab");

        let summon_action = serde_json::json!({
            "Name": "SummonEntity",
            "Params": { "SummonEntityConfig": { "TemplateId": tpl_id } }
        });
        execute_action(&mut qctx, &mut cbs, 1, 1, &summon_action)
            .expect("SummonEntity");
    }
    let pushed_ids = h.drain_push_msg_ids();
    assert!(
        pushed_ids.contains(&OperationRestrictionNotify::MSG_ID),
        "SetPlayerOperationRestriction should push OperationRestrictionNotify (15930)"
    );
    assert!(
        pushed_ids.contains(&SummonEntityNotify::MSG_ID),
        "SummonEntity action should spawn into Scene & push SummonEntityNotify"
    );

    // 5. ProgramSpecialProcess, ReadDisplayInfoRequest & LowMemoryChangeSceneFinishRequest compile & execute cleanly.
    let _ = world_puzzle::handle_read_display_info_request(
        &mut h.state,
        &mut events,
        ReadDisplayInfoRequest { DisplayId: 1001 },
    )
    .expect("read_display_info");
    let _ = world_puzzle::handle_low_memory_change_scene_finish_request(
        &mut h.state,
        &mut events,
        LowMemoryChangeSceneFinishRequest {},
    )
    .expect("low_memory_finish");
    assert!(
        !h.drain_push_msg_ids().contains(&LowMemoryChangeSceneStartNotify::MSG_ID),
        "finish should not emit start notify"
    );

    // 6. Phase 5: PhantomFetterRecommend uses PhantomRecommend table for RoleId 1402.
    let fetter_rec = echo::presets::handle_phantom_fetter_recommend_request(
        &mut h.state,
        PhantomFetterRecommendRequest { RoleId: 1402 },
    )
    .expect("phantom_fetter_recommend");
    assert!(!fetter_rec.PhantomFetterRecommendInfos.is_empty());
    assert!(
        fetter_rec.PhantomFetterRecommendInfos[0]
            .PhantomFetterGroupInfos
            .iter()
            .any(|g| g.PhantomFetterGroupId == 4)
    );
    assert!(
        !fetter_rec.PhantomFetterRecommendInfos[0]
            .MainPhantomRecommendInfos
            .is_empty()
    );

    // 7. Phase 5: PrivateChat persistence + TimelineTraceControl multi-step + BossRush & Tower recommends.
    chat::handle_private_chat_request(
        &mut h.state,
        PrivateChatRequest {
            TargetUid: 999999,
            ChatContentType: None,
            Content: b"hello wuwa".to_vec(),
            XboxBlockedPlayerIds: Vec::new(),
        },
    )
    .expect("private_chat");
    let hist = chat::handle_private_chat_history_request(
        &mut h.state,
        PrivateChatHistoryRequest {
            TargetUid: 999999,
            StartIndex: 0,
        },
    )
    .expect("private_chat_history");
    let data = hist.Data.expect("history data");
    assert_eq!(data.TotalNums, 1);
    assert_eq!(data.Chats[0].Content, b"hello wuwa");

    let tc1 = world_puzzle::handle_timeline_trace_control_request(
        &mut h.state,
        TimelineTraceControlRequest {
            EntityId: 4242,
            Forward: true,
        },
    )
    .expect("timeline step 1");
    assert_eq!(tc1.Code, Some(ErrorCode::Success));
    let tc2 = world_puzzle::handle_timeline_trace_control_request(
        &mut h.state,
        TimelineTraceControlRequest {
            EntityId: 4242,
            Forward: true,
        },
    )
    .expect("timeline step 2");
    assert_eq!(tc2.Code, Some(ErrorCode::Success));
    assert_eq!(tc2.ControlPoint, 2);

    let boss_rush = endgame::handle_boss_rush_settlement_request(
        &mut h.state,
        BossRushSettlementRequest {},
    )
    .expect("boss_rush_settlement");
    assert_eq!(boss_rush.ErrorCode, Some(ErrorCode::Success));
    assert!(h.state.player_components.progress.info.get_key_int("bossrush:score") >= 1500);

    let slash_rec = endgame::handle_slash_and_tower_recommend_request(
        &mut h.state,
        SlashAndTowerRecommendRequest { Id: 1 },
    )
    .expect("slash_rec");
    assert!(
        slash_rec.RecommendInfos[0]
            .FormationFirst
            .as_ref()
            .is_some_and(|f| !f.SelectRoles.is_empty())
    );
    let nt_rec = endgame::handle_new_tower_recommend_request(
        &mut h.state,
        NewTowerRecommendRequest {},
    )
    .expect("nt_rec");
    assert!(
        nt_rec
            .NewTowerRecommendData
            .is_some_and(|d| !d.RecommendLevels.is_empty())
    );
    h.drain_count();
}

#[test]
fn audit_116_remaining_conditions_actions_and_item_exchange() {
    let mut h = TestHarness::new("audit_116", Some(8));
    h.drain_count();
    let mut events = EventQueue::new();

    // 1. Trigger skill, music, BvB card & RoverRogue handlers.
    let _ = combat_sync::handle_use_skill_request(
        &mut h.state,
        &mut events,
        UseSkillRequest {
            SkillSingleId: 1502003,
            ..Default::default()
        },
    )
    .expect("use_skill");
    let _ = skin::handle_phantom_music_switch_request(
        &mut h.state,
        &mut events,
        PhantomMusicSwitchRequest { MusicId: 10000001 },
    )
    .expect("music_switch");
    let _ = phantom_battle::handle_phantom_battle_start_request(
        &mut h.state,
        &mut events,
        PhantomBattleStartRequest::default(),
    )
    .expect("pb_start");
    let _ = phantom_battle::handle_phantom_battle_call_card_request(
        &mut h.state,
        &mut events,
        PhantomBattleCallCardRequest {
            EffectUid: 1,
            SelectedCardUid: vec![11, 12, 13],
        },
    )
    .expect("pb_call_card");
    let _ = roguelike::handle_rover_rogue_bless_group_select_request(
        &mut h.state,
        &mut events,
        RoverRogueBlessGroupSelectRequest {
            BlessGroupId: 7,
            ..Default::default()
        },
    )
    .expect("rover_rogue_select");

    // 2. Execute new server-stateful and client-FX quest actions cleanly.
    {
        let mut qctx = h.state.quest_context(&mut events);
        let mut cbs = ActionCallbacks::default();
        let drag_on = serde_json::json!({
            "Name": "ToggleDragActorPlay",
            "Params": { "Config": { "Type": "Enable" } }
        });
        execute_action(&mut qctx, &mut cbs, 1, 1, &drag_on).expect("ToggleDragActorPlay");

        let take_photo = serde_json::json!({
            "Name": "TakePlotPhoto",
            "Params": {}
        });
        execute_action(&mut qctx, &mut cbs, 1, 1, &take_photo).expect("TakePlotPhoto");

        let client_fx = serde_json::json!({
            "Name": "Wait",
            "Params": { "Time": 1 }
        });
        execute_action(&mut qctx, &mut cbs, 1, 1, &client_fx).expect("Wait client action");
    }

    let _ = crate::network::handlers::quest::handle_submit_photo_node_request(
        &mut h.state,
        &mut events,
        sonoro_proto::pb::SubmitPhotoNodeRequest {
            TreeIncId: 0,
            NodeId: 0,
            TreeOwnerId: 0,
            EntityIds: Vec::new(),
        },
    )
    .expect("submit_photo_node");

    // 3. Evaluate remaining inline Condition variants directly.
    {
        let qctx = h.state.quest_context(&mut events);
        let ec = qctx.make_eval_context();
        assert!(eval_single_inline_condition(
            &ec,
            &serde_json::json!({"Type": "CheckFinishLoading", "CheckTarget": {"Type": "AllPlayer"}})
        ));
        assert!(eval_single_inline_condition(
            &ec,
            &serde_json::json!({"Type": "CheckPlayerUseSkill", "CheckSkills": [{"SkillId": 1502003}]})
        ));
        assert!(eval_single_inline_condition(
            &ec,
            &serde_json::json!({"Type": "CheckIsGramophonePlayingMusic", "TargetGramophone": 36000157, "CheckCondition": {"PlayList": [10000001]}})
        ));
        assert!(eval_single_inline_condition(
            &ec,
            &serde_json::json!({"Type": "CheckBVBEvent", "EventName": "WaitForGameBegin"})
        ));
        assert!(eval_single_inline_condition(
            &ec,
            &serde_json::json!({"Type": "CompareCost", "CompareType": "Eq", "Cost": 3})
        ));
        assert!(eval_single_inline_condition(
            &ec,
            &serde_json::json!({"Type": "IsCapableToEvolve", "Cost": 3})
        ));
        assert!(eval_single_inline_condition(
            &ec,
            &serde_json::json!({"Type": "CheckRoverRogueAbilitySelect", "SelectType": "BlessGroup", "IsReceived": true})
        ));
        assert!(eval_single_inline_condition(
            &ec,
            &serde_json::json!({"Type": "CheckIsInDragActorPlay", "IsIn": true})
        ));
        assert!(eval_single_inline_condition(
            &ec,
            &serde_json::json!({"Type": "CheckPhotographCameraParam", "CameraParam": [{"TemplateId": 2}]})
        ));
        assert!(eval_single_inline_condition(
            &ec,
            &serde_json::json!({"Type": "CompareTeammateDie", "Compare": "Eq", "DieCount": 0})
        ));
    }

    // 4. ItemExchangeInfo reflects real table entries + player's exchange count.
    let ex_info = minimal::handle_item_exchange_info_request(
        &mut h.state,
        ItemExchangeInfoRequest {},
    )
    .expect("item_exchange_info");
    assert!(!ex_info.ItemExchangeInfos.is_empty());
    h.drain_count();
}

#[test]
fn audit_117_private_server_multiplayer_chat_ranks_and_cfgsv() {
    let mut h = TestHarness::new("audit_117", Some(8));
    h.drain_count();
    let mut events = EventQueue::new();

    // Create a second registered player account on the same server filesystem & write scores.
    let mut pcs2 =
        crate::network::state::PlayerComponentStorage::init(&h.state.fs, &h.state.assets, 2)
            .expect("init player 2");
    pcs2.basic.info.attributes.name = "PeerTwo".to_string();
    let _ = h.state.fs.write_file(
        "player/2/basic_info",
        &sonoro_gamesv_fs::file_util::serialize_zon(&pcs2.basic.info).unwrap(),
    );
    pcs2.progress.info.set_key_int("role_tower:stage", 12);
    pcs2.progress.info.set_key_int("abyss:room", 6);
    pcs2.progress.info.set_key_int("babel:finish", 1);
    pcs2.progress.info.set_key_int("motor_fight:best_score", 9500);
    pcs2.progress.info.set_key_int("rb:cash", 4200);
    pcs2.progress.info.set_key_int("pinball:kicks", 3);
    pcs2.progress.info.set_key_int("pinball:tower", 4);
    pcs2.progress.save(&h.state.fs).expect("save player 2 progress");

    // Attach CoopService with online PeerTwo (pid=2) & PlayerOne (pid=1).
    let coop = Arc::new(CoopService::new());
    let sender1 = Arc::new(VecCoopSender::default());
    let sender2 = Arc::new(VecCoopSender::default());
    coop.register_online(OnlinePlayer::new(1, sender1, "PlayerOne"));
    coop.register_online(OnlinePlayer::new(2, Arc::clone(&sender2) as Arc<dyn sonoro_gamesv_logic::coop_service::CoopSender>, "PeerTwo"));
    h.state.coop_service = Some(Arc::clone(&coop));

    // 1. World ChannelChat broadcasts to all online players, and PrivateChat delivers live + disk inbox.
    chat::handle_channel_chat_request(
        &mut h.state,
        &mut events,
        ChannelChatRequest {
            Content: b"world hello!".to_vec(),
            ..Default::default()
        },
    )
    .expect("channel chat");
    chat::handle_private_chat_request(
        &mut h.state,
        PrivateChatRequest {
            TargetUid: 2,
            Content: b"pm peer two".to_vec(),
            ..Default::default()
        },
    )
    .expect("private chat");
    let p2_packets = sender2.packets.lock().unwrap();
    assert_eq!(p2_packets.len(), 2, "PeerTwo should receive both world & private ChatNotify");
    drop(p2_packets);

    let pcs2_reloaded =
        sonoro_gamesv_logic::component::player::PlayerProgressComponent::init(&h.state.fs, 2)
            .expect("reload p2 progress");
    assert!(
        !pcs2_reloaded.info.get_key_str("chat:priv:1").is_empty(),
        "Private chat should also persist in TargetUid inbox on disk"
    );

    // 2. Server leaderboards read real multi-player scores from disk + CoopService.
    let rt_rank = endgame::handle_role_tower_rank_request(&mut h.state, RoleTowerRankRequest::default())
        .expect("role tower rank");
    assert_eq!(rt_rank.RankInfo[0].FriendEntries.len(), 1);
    assert_eq!(rt_rank.RankInfo[0].SelfServerEntry.as_ref().unwrap().Rank, 2);

    let abyss_rank = endgame::handle_abyss_rank_list_request(&mut h.state, AbyssRankListRequest::default())
        .expect("abyss rank");
    assert_eq!(abyss_rank.FriendPassData.len(), 1);

    let babel_rank = endgame::handle_babel_tower_rank_list_request(&mut h.state, BabelTowerRankListRequest::default())
        .expect("babel rank");
    assert_eq!(babel_rank.FriendPassData.len(), 1);

    let motor_rank = motor::handle_motor_fight_rank_list_request(&mut h.state, MotorFightRankListRequest::default())
        .expect("motor fight rank");
    assert_eq!(motor_rank.FriendData.len(), 1);

    let rhythm_rank = events_major::handle_rhythm_rank_list_request(&mut h.state, RhythmRankListRequest::default())
        .expect("rhythm rank");
    assert_eq!(rhythm_rank.FriendData.len(), 1);

    let pinball_rank = events_major::handle_pinball_friend_rank_request(&mut h.state, PinballFriendRankRequest::default())
        .expect("pinball rank");
    assert_eq!(pinball_rank.RankData.len(), 1);

    let rb_rank = events_major::handle_racing_bets_rank_request(&mut h.state, RacingBetsRankRequest::default())
        .expect("racing bets rank");
    assert_eq!(rb_rank.PlayerInfo.len(), 1);
    assert_eq!(rb_rank.PlayerInfo[0].Cash, 4200);
    h.drain_count();
}

#[test]
fn audit_114_cross_instance_skip_clears_black_screen_on_scene_loading_finish() {
    use crate::network::handlers::{flow, minimal, scene};
    use sonoro_gamesv_logic::events::Event;
    use sonoro_proto::pb::{
        BlackScreenStatePush, FlowEndRequest, FlowServerSkipNotify, SceneLoadingFinishRequest,
    };

    let mut h = TestHarness::new("audit114", Some(8));
    let mut events = EventQueue::new();

    h.state
        .scene
        .as_mut()
        .unwrap()
        .set_active_flow(9901, 0, 0, "TestCrossInstanceSkip", 1, 1);
    minimal::handle_black_screen_state_push(
        &mut h.state,
        BlackScreenStatePush {
            IsOpen: true,
            BlackScreenReason: b"SkipPlot_TestCrossInstanceSkip_1_1".to_vec(),
            ..Default::default()
        },
    )
    .expect("black screen push");
    assert!(h.state.scene.as_ref().unwrap().client_black_screen_open);

    // Simulate cross-instance transition while skipping active flow.
    {
        let s = h.state.scene.as_mut().unwrap();
        s.clear_active_flow();
        s.pending_transition_instance_id = Some(1529);
    }
    h.state.player_components.scene.last_scene_info.instance_id = 1529;
    let _ = flow::handle_flow_end_request(
        &mut h.state,
        &mut events,
        FlowEndRequest {
            FlowIncId: 9901,
            IsSkip: true,
            ..Default::default()
        },
    )
    .expect("flow end");
    let before_finish = h.drain_push_msg_ids();
    assert!(!before_finish.contains(&FlowServerSkipNotify::MSG_ID));
    assert!(h.state.scene.as_ref().unwrap().client_black_screen_open);

    // Entering the new instance must preserve client_black_screen_open across Scene replacement.
    let mut join_events = EventQueue::new();
    join_events.enqueue(Event::InitialSceneJoin {
        player_id: h.state.player_id,
        map_config_id: 1529,
        instance_id: 1529,
        location: None,
        rotation: None,
        pending_flow: None,
    });
    while let Some(ev) = join_events.dequeue() {
        let mut rt = h.state.as_runtime(&mut events, None);
        sonoro_gamesv_logic::handlers::dispatch_event(&mut rt, ev).expect("dispatch join");
    }
    assert_eq!(h.state.scene.as_ref().unwrap().instance_id, 1529);
    assert!(h.state.scene.as_ref().unwrap().client_black_screen_open);

    scene::handle_scene_loading_finish_request(
        &mut h.state,
        &mut events,
        SceneLoadingFinishRequest::default(),
    )
    .expect("scene loading finish");
    assert!(!h.state.scene.as_ref().unwrap().client_black_screen_open);

    let after_finish = h.drain_push_msg_ids();
    assert!(
        after_finish.contains(&FlowServerSkipNotify::MSG_ID),
        "SceneLoadingFinishRequest must clear pre-transition SkipPlot black screen"
    );
}

#[test]
fn audit_115_lift_initial_pos_dungeon_level_play_npc_priority_and_batch_spawn() {
    use sonoro_proto::enums::{ElevatorState, LiftMoveType};
    use sonoro_proto::pb::{
        ChangeDataLayerFinishRequest, ElevatorMoveNotify, ElevatorStateChangeRequest,
        EntityAddNotify, EnterLevelPlayNotify, JSPatchNotify, MapMarkInfoNotify,
        RoleDevelopConfigRequest, UpdateNodeStatusNotify,
    };

    let mut h = TestHarness::new("audit115", None);
    let mut events = EventQueue::new();

    // Seed many active quests + off-map Jinzhou LevelPlay 200700001 + active tracked quest.
    h.state
        .player_components
        .quest
        .info
        .get_or_add_active_quest(880_000_044)
        .active_nodes
        .push(4);
    h.state
        .player_components
        .quest
        .info
        .get_or_add_active_quest(311_000_000)
        .active_nodes
        .push(5);
    h.state
        .player_components
        .quest
        .info
        .get_or_add_active_quest(151_000_000)
        .active_nodes
        .push(2);
    h.state.player_components.quest.info.tracked_quest_id = 880_000_044;
    h.state
        .player_components
        .quest
        .info
        .progress
        .entered_level_plays
        .push(200_700_001);

    // Enter Instance 1529 cleanly and run SceneLoadingFinishRequest + SpatialSpawnsCheck.
    {
        let mut sc = sonoro_gamesv_logic::scene::Scene::init(&h.state.fs, h.state.player_id, 1529)
            .expect("init scene 1529");
        sc.join_scene_pushed = true;
        sc.deferred_join_sync_pending = true;
        if let Some(p) = sc.instance.players.first_mut() {
            p.location = [-14_610.0, 72_450.0, 5_250.0];
        } else {
            sc.instance.players.push(ScenePlayer {
                id: h.state.player_id,
                group_type: 1,
                location: [-14_610.0, 72_450.0, 5_250.0],
                rotation: [0.0; 3],
            });
        }
        h.state.scene = Some(sc);
    }
    h.drain_count();

    scene::handle_scene_loading_finish_request(
        &mut h.state,
        &mut events,
        SceneLoadingFinishRequest::default(),
    )
    .expect("scene loading finish 1529");
    while let Some(ev) = events.dequeue() {
        let mut rt = h.state.as_runtime(&mut events, None);
        sonoro_gamesv_logic::handlers::dispatch_event(&mut rt, ev)
            .expect("drain spatial spawns in 1529");
    }

    // 1. Dungeon LevelPlay 880000003 (Map 900 PackId 1529) must enter and load DataLayer 850039,
    // while unrelated Map 900 Dungeon LevelPlay 241750000 (PackId 1101008) must NOT enter.
    let qinfo = &h.state.player_components.quest.info;
    assert!(qinfo.progress.entered_level_plays.contains(&880000003));
    assert!(!qinfo.progress.entered_level_plays.contains(&241750000));
    assert!(qinfo.world.loaded_datalayers.contains(&850039));

    // 2. Elevator 880850006 MUST spawn at base Transform (-335.0) so client HoverPoints stay
    // [-335.0, +4365.0], with Floor 2 stored server-side and snapped via CustomSpeed notify.
    let scene_ref = h.state.scene.as_ref().expect("scene 1529");
    let lift_ent = scene_ref
        .find_entity_net_id_by_config_id(880_850_006)
        .and_then(|nid| scene_ref.entity(nid))
        .expect("elevator 880850006 spawned in 1529");
    let lift_net_id = lift_ent.net_id;
    assert_eq!(lift_ent.world_item.lift.map(|l| l.location), Some(2));
    let lift_z = lift_ent.position.expect("lift position").location[2];
    assert!(
        (lift_z - (-335.0)).abs() < 1.0,
        "expected elevator 880850006 base pos Z ~-335.0, got {lift_z}"
    );
    assert!(
        scene_ref.find_entity_net_id_by_config_id(880_850_034).is_some(),
        "NPC 880850034 near spawn must spawn in 1529"
    );
    let cond_eq2: Value = serde_json::from_str(
        r#"{"Type":"CompareLift","EntityId":880850006,"Location":2,"Compare":"Eq"}"#,
    )
    .unwrap();
    let cond_eq1: Value = serde_json::from_str(
        r#"{"Type":"CompareLift","EntityId":880850006,"Location":1,"Compare":"Eq"}"#,
    )
    .unwrap();
    assert!(eval_single_inline_condition(
        &h.state.eval_context(),
        &cond_eq2,
    ));
    assert!(!eval_single_inline_condition(
        &h.state.eval_context(),
        &cond_eq1,
    ));

    // 3. Spatial world spawns must batch EntityAddNotify natively (without JSPatchNotify),
    // spawn lift via CustomSpeed ElevatorMoveNotify, and SceneLoadingFinishRequest must NOT flood active quest nodes.
    let mut adds: Vec<EntityAddNotify> = Vec::new();
    let mut lift_moves: Vec<ElevatorMoveNotify> = Vec::new();
    let mut node_updates: Vec<UpdateNodeStatusNotify> = Vec::new();
    while let Ok(frame) = h.rx.try_recv() {
        if frame.len() < 3 {
            continue;
        }
        if let Ok(msg) = super::Message::decode(&frame[3..]) {
            let id = msg.header.message_id();
            if id == EntityAddNotify::MSG_ID
                && let Ok(v) = <EntityAddNotify as sonoro_proto::wire::PbMessage>::decode(&msg.payload)
            {
                adds.push(v);
            } else if id == ElevatorMoveNotify::MSG_ID
                && let Ok(v) = <ElevatorMoveNotify as sonoro_proto::wire::PbMessage>::decode(&msg.payload)
            {
                lift_moves.push(v);
            } else if id == UpdateNodeStatusNotify::MSG_ID
                && let Ok(v) = <UpdateNodeStatusNotify as sonoro_proto::wire::PbMessage>::decode(&msg.payload)
            {
                node_updates.push(v);
            }
        }
    }
    let total_added: usize = adds.iter().map(|n| n.EntityPbs.len()).sum();
    assert!(
        total_added > 1,
        "expected multiple entities spawned in 1529, got {total_added}"
    );
    assert!(
        adds.iter().any(|n| n.EntityPbs.len() > 1),
        "expected batched EntityAddNotify with multiple EntityPbs per notify, got {} notifies for {total_added} entities",
        adds.len()
    );
    assert!(
        lift_moves.iter().any(|m| {
            m.EntityId == lift_net_id
                && m.Location == 2
                && m.MoveType
                    .as_ref()
                    .and_then(|mt| mt.Type)
                    == Some(LiftMoveType::CustomSpeed)
        }),
        "expected CustomSpeed ElevatorMoveNotify to snap lift 880850006 to floor 2"
    );
    assert!(
        !node_updates
            .iter()
            .any(|u| u.TreeOwnerId == 311_000_000 || u.TreeOwnerId == 151_000_000),
        "SceneLoadingFinishRequest must not flood UpdateNodeStatusNotify across unrelated active quests: {node_updates:?}"
    );

    // 4. ElevatorStateChangeRequest End on same floor must NOT echo ElevatorMoveNotify;
    // Forward to Floor 1 MUST emit ElevatorMoveNotify { Location: 1 }, and End on Floor 1 must not.
    world_puzzle::handle_elevator_state_change_request(
        &mut h.state,
        ElevatorStateChangeRequest {
            EntityId: lift_net_id,
            State: Some(ElevatorState::End),
            TagState: 2,
        },
    )
    .expect("elevator end at floor 2");
    assert!(
        h.drain_decoded::<ElevatorMoveNotify>(ElevatorMoveNotify::MSG_ID)
            .is_empty(),
        "ElevatorState::End must not echo ElevatorMoveNotify back to client"
    );
    world_puzzle::handle_elevator_state_change_request(
        &mut h.state,
        ElevatorStateChangeRequest {
            EntityId: lift_net_id,
            State: Some(ElevatorState::Forward),
            TagState: 1,
        },
    )
    .expect("elevator forward to floor 1");
    let fwd_moves: Vec<ElevatorMoveNotify> = h.drain_decoded(ElevatorMoveNotify::MSG_ID);
    assert_eq!(fwd_moves.len(), 1);
    assert_eq!(fwd_moves[0].Location, 1);

    // 5. Call-the-elevator 880850007 StateChangeBehaviors moves 880850006 back to Floor 2.
    {
        let mut qctx = h.state.quest_context(&mut events);
        let mut cbs = sonoro_gamesv_logic::helpers::quest_types::ActionCallbacks::default();
        sonoro_gamesv_logic::helpers::quest_actions::apply_entity_state_change(
            &mut qctx,
            &mut cbs,
            880_850_007,
            "关卡.Common.状态.激活",
        )
        .expect("apply 880850007 active state");
        sonoro_gamesv_logic::helpers::quest_actions::apply_entity_state_change(
            &mut qctx,
            &mut cbs,
            880_850_007,
            "关卡.Common.状态.常态",
        )
        .expect("apply 880850007 normal state");
    }
    let call_moves: Vec<ElevatorMoveNotify> = h.drain_decoded(ElevatorMoveNotify::MSG_ID);
    assert_eq!(
        call_moves.len(),
        1,
        "calling elevator 880850007 should emit exactly one ElevatorMoveNotify without duplicate NaN move"
    );
    assert_eq!(call_moves[0].Location, 2);

    // 6. ChangeDataLayerFinishRequest in 1529 must NOT trigger off-map LevelPlay 200700001
    // (Wait -> LevelEventWait key release) or flood active quest notifications.
    crate::network::handlers::inst::handle_change_data_layer_finish_request(
        &mut h.state,
        &mut events,
        ChangeDataLayerFinishRequest { InstId: 1529 },
    )
    .expect("change data layer finish");
    let dl_updates: Vec<UpdateNodeStatusNotify> = h.drain_decoded(UpdateNodeStatusNotify::MSG_ID);
    assert!(
        !dl_updates.iter().any(|u| u.TreeOwnerId == 200_700_001),
        "off-map LevelPlay 200700001 must not tick inside Instance 1529"
    );
    assert!(
        !dl_updates.iter().any(|u| u.TreeOwnerId == 311_000_000),
        "ChangeDataLayerFinishRequest must not flood unrelated quests"
    );

    // 7. TraceQuestRequest Operate=Auto(2) must not steal tracking from active tracked quest,
    // build_map_mark_info_notify deduplicates MarkIds, PhantomFetterRecommend gives 2..=3 entries,
    // and RoleDevelopConfigResponse includes Configs.
    quest::handle_trace_quest_request(
        &mut h.state,
        &mut events,
        TraceQuestRequest {
            QuestId: 311_000_000,
            TraceType: 1,
            Operate: 2,
        },
    )
    .expect("auto trace blocked");
    quest::handle_trace_quest_request(
        &mut h.state,
        &mut events,
        TraceQuestRequest {
            QuestId: 311_000_000,
            TraceType: 1,
            Operate: 0,
        },
    )
    .expect("none-operate auto trace blocked");
    assert_eq!(h.state.player_components.quest.info.tracked_quest_id, 880_000_044);
    for herald_id in [
        100_200_001,
        108_000_001,
        110_800_001,
        110_900_001,
        111_000_001,
        111_100_001,
    ] {
        assert!(
            !activity::is_activity_unlocked(None, &h.state.assets, herald_id),
            "Herald/Newcomer activity {herald_id} must remain locked/disabled"
        );
    }
    let acts = activity::build_activity_list(
        &h.state.assets,
        &h.state.player_components.progress,
        None,
    );
    for herald_id in [
        100_200_001,
        108_000_001,
        110_800_001,
        110_900_001,
        111_000_001,
        111_100_001,
    ] {
        assert!(
            !acts.iter().any(|a| a.Id == herald_id),
            "Herald/Newcomer activity {herald_id} must not appear in build_activity_list"
        );
    }
    let mut show_seen = std::collections::HashSet::new();
    let mm_notify: MapMarkInfoNotify = sonoro_gamesv_logic::handlers::misc::build_map_mark_info_notify(
        &h.state.assets,
        &h.state.player_components.map,
        &h.state.player_components.basic,
        &h.state.player_components.quest,
    );
    assert!(mm_notify.ShowMarkIds.iter().all(|m| show_seen.insert(m.MarkId)));
    let fetter_rec = echo::presets::handle_phantom_fetter_recommend_request(
        &mut h.state,
        PhantomFetterRecommendRequest { RoleId: 1402 },
    )
    .expect("phantom fetter rec");
    let fgroups = &fetter_rec.PhantomFetterRecommendInfos[0].PhantomFetterGroupInfos;
    assert!((2..=3).contains(&fgroups.len()));
    let role_dev = minimal::handle_role_develop_config_request(
        &mut h.state,
        RoleDevelopConfigRequest::default(),
    )
    .expect("role develop config");
    assert!(role_dev.Configs.is_some());

    // Also verify EnterLevelPlayNotify for 880000003 + batched spawn preserves JSPatch source map call.
    h.state.scene.as_mut().unwrap().entities.clear();
    let rt = h.state.as_runtime(&mut events, None);
    let sc = rt.scene.as_mut().unwrap();
    let handles = vec![
        entity_cmd::spawn_with_location_and_progress(
            rt.events,
            sc,
            rt.fs,
            rt.assets,
            rt.sink,
            rt.basic_comp,
            880_850_006,
            Some(false),
            Some(false),
            Some(900),
            None,
            true,
            &rt.progress_comp.info,
            false,
            true,
        )
        .unwrap()
        .unwrap(),
        entity_cmd::spawn_with_location_and_progress(
            rt.events,
            sc,
            rt.fs,
            rt.assets,
            rt.sink,
            rt.basic_comp,
            880_000_004,
            Some(false),
            Some(false),
            Some(900),
            None,
            true,
            &rt.progress_comp.info,
            false,
            true,
        )
        .unwrap()
        .unwrap(),
    ];
    let batch: Vec<_> = handles
        .iter()
        .zip([880_850_006i64, 880_000_004i64])
        .map(|(&hnd, eid)| (hnd, eid, 900))
        .collect();
    let published = entity_cmd::publish_world_spawn_batch(sc, rt.fs, rt.assets, rt.sink, &batch);
    assert_eq!(published.len(), 2);
    let batch_msg_ids = h.drain_push_msg_ids();
    assert_eq!(
        batch_msg_ids
            .iter()
            .filter(|&&id| id == EntityAddNotify::MSG_ID)
            .count(),
        1
    );
    assert_eq!(
        batch_msg_ids
            .iter()
            .filter(|&&id| id == JSPatchNotify::MSG_ID)
            .count(),
        0
    );
    // Also verify pre-spawn DoInteract registration embeds DynamicInteractInfos into EntityAddNotify
    // without emitting EntityRemoveDynamicInteractNotify flicker on re-sync.
    h.state.scene.as_mut().unwrap().entities.clear();
    h.state
        .scene
        .as_mut()
        .unwrap()
        .dynamic_interact_options
        .clear();
    let rt = h.state.as_runtime(&mut events, None);
    let sc = rt.scene.as_mut().unwrap();
    let dyn_handle = entity_cmd::spawn_with_location_and_progress(
        rt.events,
        sc,
        rt.fs,
        rt.assets,
        rt.sink,
        rt.basic_comp,
        880_000_004,
        Some(false),
        Some(false),
        Some(900),
        None,
        true,
        &rt.progress_comp.info,
        false,
        true,
    )
    .unwrap()
    .unwrap();
    let do_interact_node = sonoro_gamesv_data::lazy_node_store::NodeEntry {
        id: 77,
        parent_id: 1,
        node_type: "ChildQuest".to_string(),
        count: 1,
        reward_id: 0,
        data: serde_json::json!({
            "Condition": {
                "Type": "DoInteract",
                "AddOptions": [{
                    "EntityId": 880_000_004i64,
                    "Option": {
                        "Guid": "audit115-dyn-guid",
                        "TidContent": "TestInteract",
                        "Type": { "Type": "Actions", "Actions": [] }
                    }
                }]
            }
        }),
    };
    quest_engine::push_do_interact_options_for_bt_node_mode(
        sc,
        rt.sink,
        BtType::BtTypeQuest,
        880_000_044,
        &do_interact_node,
        Some(880_000_004),
        false,
    )
    .expect("pre-spawn register dynamic interact");
    let published_dyn =
        entity_cmd::publish_world_spawn_batch(sc, rt.fs, rt.assets, rt.sink, &[(dyn_handle, 880_000_004, 900)]);
    assert_eq!(published_dyn.len(), 1);
    let dyn_adds: Vec<EntityAddNotify> = h.drain_decoded(EntityAddNotify::MSG_ID);
    let dyn_pb = dyn_adds
        .iter()
        .flat_map(|n| n.EntityPbs.iter())
        .find(|pb| pb.Id == dyn_handle.net_id)
        .expect("EntityPb in EntityAddNotify");
    let embedded_dyn = dyn_pb.ComponentPbs.iter().any(|c| {
        matches!(
            &c.ComponentPb,
            Some(EntityComponentPbComponentPbOneof::InteractComponent(ic))
                if ic
                    .DynamicInteractInfos
                    .iter()
                    .any(|di| di.OptionGuid == b"audit115-dyn-guid")
        )
    });
    assert!(
        embedded_dyn,
        "EntityAddNotify must embed DynamicInteractInfo synchronously in InteractComponentPb"
    );

    let rt = h.state.as_runtime(&mut events, None);
    let sc = rt.scene.as_mut().unwrap();
    quest_engine::push_do_interact_options_for_bt_node_mode(
        sc,
        rt.sink,
        BtType::BtTypeQuest,
        880_000_044,
        &do_interact_node,
        Some(880_000_004),
        true,
    )
    .expect("idempotent dynamic interact resync");
    let resync_msg_ids = h.drain_push_msg_ids();
    assert!(
        !resync_msg_ids.contains(&EntityRemoveDynamicInteractNotify::MSG_ID),
        "idempotent DoInteract sync must not emit EntityRemoveDynamicInteractNotify"
    );
    assert!(
        !resync_msg_ids.contains(&EntityAddDynamicInteractNotify::MSG_ID),
        "idempotent DoInteract sync must not duplicate EntityAddDynamicInteractNotify"
    );
    let _ = EnterLevelPlayNotify::MSG_ID;
}

#[test]
fn audit_116_join_scene_and_quest_entity_spawn_attach_dynamic_interact_infos() {
    let mut h = TestHarness::new("audit116", Some(8));
    let mut events = EventQueue::new();

    {
        let info = &mut h.state.player_components.quest.info;
        info.active_quests.push(sonoro_gamesv_fs::quest_info::ActiveQuest {
            quest_id: 152_750_013,
            active_nodes: vec![2],
            finished_nodes: Vec::new(),
            killed_entities: Vec::new(),
        });
        info.active_quests.push(sonoro_gamesv_fs::quest_info::ActiveQuest {
            quest_id: 880_000_044,
            active_nodes: vec![4],
            finished_nodes: Vec::new(),
            killed_entities: Vec::new(),
        });
    }

    // Simulate EnterGame sync_on_login spawning quest target entities prior to JoinSceneNotify.
    {
        let mut qctx = h.state.quest_context(&mut events);
        sonoro_gamesv_logic::helpers::quest_actions::ensure_quest_entity_spawned(
            &mut qctx,
            152_750_020,
        );
        if let Some(sc) = qctx.scene.as_deref_mut() {
            sc.instance.players.push(ScenePlayer {
                id: 1,
                group_type: 1,
                location: [305_050.0, 127_199.0, 4_895.0],
                rotation: [0.0; 3],
            });
        }
    }
    let _ = h.drain_push_msg_ids();

    scene_logic::notify_join_scene(
        None,
        &mut events,
        &h.state.fs,
        &h.state.assets,
        &mut h.state.conn,
        1,
        &mut h.state.player_components.scene,
        &h.state.player_components.basic,
        &mut h.state.player_components.role,
        &mut h.state.player_components.weapon,
        &mut h.state.player_components.motor,
        &mut h.state.player_components.echo,
        &mut h.state.player_components.inventory,
        &mut h.state.player_components.map,
        &h.state.player_components.quest,
        &h.state.player_components.progress,
        h.state.scene.as_mut().unwrap(),
        &mut h.state.buff_timers,
    )
    .expect("notify_join_scene");

    let join_notifies: Vec<JoinSceneNotify> = h.drain_decoded(JoinSceneNotify::MSG_ID);
    assert_eq!(join_notifies.len(), 1);
    let aoi_ents = &join_notifies[0]
        .SceneInfo
        .as_ref()
        .expect("SceneInfo")
        .AoiData
        .as_ref()
        .expect("AoiData")
        .Entities;
    let npc_pb = aoi_ents
        .iter()
        .find(|e| e.ConfigId == 152_750_020)
        .expect("NPC 152750020 in JoinSceneNotify");
    let has_join_dyn = npc_pb.ComponentPbs.iter().any(|c| {
        matches!(
            &c.ComponentPb,
            Some(EntityComponentPbComponentPbOneof::InteractComponent(ic))
                if ic
                    .DynamicInteractInfos
                    .iter()
                    .any(|di| di.OptionGuid == b"60011bf2c6c944dfb768850e90548d0b")
        )
    });
    assert!(
        has_join_dyn,
        "JoinSceneNotify must embed quest NPC DynamicInteractInfo inside InteractComponentPb"
    );

    // After JoinSceneNotify, ensure_quest_entity_spawned for 880000345 must also register & sync its DoInteract option.
    {
        let mut qctx = h.state.quest_context(&mut events);
        sonoro_gamesv_logic::helpers::quest_actions::ensure_quest_entity_spawned(
            &mut qctx,
            880_000_345,
        );
    }
    let dyn_notifies: Vec<EntityAddDynamicInteractNotify> =
        h.drain_decoded(EntityAddDynamicInteractNotify::MSG_ID);
    assert!(
        dyn_notifies
            .iter()
            .any(|n| n.OptionGuid == b"089ae92a86984afcbb6f230705750e25"),
        "post-join quest entity spawn must emit EntityAddDynamicInteractNotify for 880000345"
    );

    // Subsequent on_scene_loading_finish must preserve dynamic interact options idempotently.
    {
        let mut qctx = h.state.quest_context(&mut events);
        quest_engine::on_scene_loading_finish(&mut qctx).expect("on_scene_loading_finish");
    }
    let scene_ref = h.state.scene.as_ref().unwrap();
    assert!(
        scene_ref
            .find_dynamic_interact_option("60011bf2c6c944dfb768850e90548d0b")
            .is_some()
            && scene_ref
                .find_dynamic_interact_option("089ae92a86984afcbb6f230705750e25")
                .is_some(),
        "dynamic interact options must remain registered after scene loading finish"
    );
}

#[test]
fn audit_117_scene_loading_finish_defers_spatial_spawns_and_skips_duplicate_login_sync() {
    let mut h = TestHarness::new("audit117", Some(8));
    h.state
        .player_components
        .quest
        .info
        .progress
        .entered_level_plays
        .push(200_700_001);
    {
        let sc = h.state.scene.as_mut().unwrap();
        sc.join_scene_pushed = true;
        sc.deferred_join_sync_pending = true;
        sc.login_world_synced = true;
        sc.instance.players.push(ScenePlayer {
            id: 1,
            group_type: 1,
            location: [305_050.0, 127_199.0, 4_895.0],
            rotation: [0.0; 3],
        });
    }
    let _ = h.drain_push_msg_ids();

    let req_msg = crate::network::message::Message {
        header: crate::network::message::Header::Request(crate::network::message::RequestHeader {
            seq_no: 1,
            rpc_id: 77,
            msg_id: SceneLoadingFinishRequest::MSG_ID,
        }),
        payload: sonoro_proto::PbMessage::encode_to_vec(&SceneLoadingFinishRequest::default()),
    };
    crate::network::handlers::dispatch_message(&mut h.state, &req_msg)
        .expect("dispatch SceneLoadingFinishRequest");

    let push_ids = h.drain_push_msg_ids();
    assert!(
        !push_ids.contains(&BehaviorTreeInfoNotify::MSG_ID),
        "initial login SceneLoadingFinishRequest must not duplicate LevelPlay BehaviorTreeInfoNotify"
    );
    assert!(
        !push_ids.contains(&EntityAddNotify::MSG_ID),
        "live SceneLoadingFinishRequest must defer SpatialSpawnsCheck instead of flooding EntityAddNotify ahead of mask4 RPCs"
    );

    let now_ms = sonoro_gamesv_logic::scene::now_millis();
    let sc = h.state.scene.as_mut().unwrap();
    assert!(!sc.login_world_synced);
    assert!(sc.deferred_spatial_spawns_at_ms.is_some());
    assert!(!sc.take_due_deferred_spatial_spawns(now_ms));
    assert!(sc.take_due_deferred_spatial_spawns(now_ms + 500));
    assert!(sc.deferred_spatial_spawns_at_ms.is_none());
}

#[test]
fn audit_118_issue3_prologue_open_system_reach_area_flow_skip_echo_and_collectibles() {
    use sonoro_proto::pb::{
        EntityInteractRequest, EntityRemoveNotify, FlowEndRequest, FlowServerSkipNotify,
        FlowShowEntityNotify,
    };

    let mut h = TestHarness::new("audit118", Some(8));
    let mut events = EventQueue::new();

    // 1. OpenSystemFunction systems remain locked on a fresh account even when OpenConditionId == 0.
    {
        let eval_ctx = h.state.eval_context();
        assert!(condition_helper_is_open(&eval_ctx, 10001));
        for gated_id in [10007, 10031, 10036, 10001003] {
            assert!(
                !condition_helper_is_open(&eval_ctx, gated_id),
                "system {gated_id} should stay locked before OpenSystemFunction action"
            );
        }
    }
    assert!(h.state.player_components.quest.info.unlock_system(10036));
    assert!(condition_helper_is_open(&h.state.eval_context(), 10036));

    // 2. Quest 139000025 node 19 completion advances to ParallelSelect 23 (child node 4), not node 21.
    quest_engine::tree::activate_quest(&mut h.state.quest_context(&mut events), 139_000_025, false)
        .expect("activate 139000025");
    {
        let sc = h.state.scene.as_mut().unwrap();
        sc.set_active_flow(1, 139_000_025, 19, "剧情_新主线_CE版_序章", 1, 1);
    }
    quest_engine::on_flow_end_with_options(
        &mut h.state.quest_context(&mut events),
        1,
        &BTreeMap::new(),
        true,
    )
    .expect("finish node 19 flow");
    let aq_prologue = h
        .state
        .player_components
        .quest
        .info
        .find_active_quest(139_000_025)
        .expect("prologue remains active");
    assert!(aq_prologue.is_node_finished(19));
    assert!(
        aq_prologue.is_node_active(4) || aq_prologue.is_node_active(23),
        "expected node 19 to advance into ParallelSelect 23 / child 4, got active={:?}",
        aq_prologue.active_nodes
    );
    assert!(
        !aq_prologue.is_node_active(21),
        "node 19 must not jump straight to node 21"
    );

    // 3. Flow skip transitioning directly into another flow still emits FlowServerSkipNotify,
    //    and HideByRangeInFlow's __flow_range hidden group is restored when the flow ends.
    h.drain_count();
    {
        let sc = h.state.scene.as_mut().unwrap();
        sc.set_active_flow(10, 139_000_025, 4, "FlowA", 1, 1);
        sc.client_black_screen_open = true;
        sc.enqueue_pending_flow(139_000_025, 4, "FlowB", 2, 1);
    }
    let hide_range_act: Value = serde_json::from_str(
        r#"{"Name":"HideByRangeInFlow","Params":{"Center":{"X":0,"Y":0,"Z":0},"Radius":500,"EntityIds":[139800002]}}"#,
    )
    .unwrap();
    execute_action(
        &mut h.state.quest_context(&mut events),
        &mut ActionCallbacks::default(),
        139_000_025,
        4,
        &hide_range_act,
    )
    .expect("HideByRangeInFlow");
    assert!(
        h.state
            .player_components
            .quest
            .info
            .world
            .hidden_groups
            .iter()
            .any(|g| g.group_key == "__flow_range")
    );
    let _ = h.drain_count();

    crate::network::handlers::flow::handle_flow_end_request(
        &mut h.state,
        &mut events,
        FlowEndRequest {
            FlowIncId: 10,
            OptionInfos: BTreeMap::new(),
            IsSkip: true,
            ..Default::default()
        },
    )
    .expect("chained skip flow end");
    let push_ids = h.drain_push_msg_ids();
    assert!(
        push_ids.contains(&FlowServerSkipNotify::MSG_ID),
        "chained SkipPlot transition must emit FlowServerSkipNotify to clear FadeOutScreen"
    );
    assert!(
        !h.state
            .player_components
            .quest
            .info
            .world
            .hidden_groups
            .iter()
            .any(|g| g.group_key == "__flow_range"),
        "__flow_range must be cleared on flow end/skip"
    );
    assert!(
        push_ids.contains(&FlowShowEntityNotify::MSG_ID),
        "restoring __flow_range should push FlowShowEntityNotify"
    );
    assert!(
        !h.state
            .player_components
            .quest
            .info
            .world
            .hidden_entities
            .contains(&139_800_002),
        "restoring __flow_range must remove hidden entities mark"
    );

    // 4. ReachArea uses true 3D sphere even when Range > 3000 (node 13 vs node 140).
    let qnodes_139 = h
        .state
        .assets
        .quest_nodes
        .get_owner_nodes(139_000_025)
        .expect("139000025 nodes");
    let node13 = qnodes_139.iter().find(|n| n.id == 13).expect("node 13");
    let cond13 = node13
        .data
        .get("Condition")
        .and_then(|v| v.as_object())
        .expect("node 13 Condition");
    assert!(
        !quest_engine::triggers::matches_reach_area_pub(
            &mut h.state.quest_context(&mut events),
            139_000_025,
            &node13.data,
            cond13,
            [-114_191.45, 71_894.34, 13_479.53],
        ),
        "node 13 must not trigger 84m vertically above Z=5077 outside 3D sphere"
    );
    assert!(quest_engine::triggers::matches_reach_area_pub(
        &mut h.state.quest_context(&mut events),
        139_000_025,
        &node13.data,
        cond13,
        [-110_030.0, 73_280.0, 5_200.0],
    ));

    // 5. VisionCapture Disabled=true suppresses drop/phantom, while invalid level VisionCaptureId
    //    (139800002 -> 391090005) falls back to template ID present in CalabashDevelopReward (390077021).
    let (disabled_mob_nid, fallback_boss_nid) = {
        let mut s1 = EntityComponentStorage::new(
            0,
            ConfigComponent {
                camp: 1,
                config_id: 310_000_578,
                config_type: ConfigType::Level,
                entity_type: EntityType::Monster,
                state: EntityState::Default,
            },
        );
        s1.position = Some(PositionComponent {
            location: [0.0; 3],
            rotation: [0.0; 3],
        });
        let mut s2 = EntityComponentStorage::new(
            0,
            ConfigComponent {
                camp: 1,
                config_id: 139_800_002,
                config_type: ConfigType::Level,
                entity_type: EntityType::Monster,
                state: EntityState::Default,
            },
        );
        s2.position = Some(PositionComponent {
            location: [100.0, 200.0, 300.0],
            rotation: [0.0; 3],
        });
        let sc = h.state.scene.as_mut().unwrap();
        let e1 = sc.spawn(s1).net_id;
        let e2 = sc.spawn(s2).net_id;
        (e1, e2)
    };
    {
        let mut qctx = h.state.quest_context(&mut events);
        let mut lp_ctx = level_play::LevelPlayContext::from_quest_ctx(&mut qctx).expect("lp ctx");
        level_play::on_entity_killed(&mut lp_ctx, disabled_mob_nid, Some(310_000_578))
            .expect("kill disabled mob");
    }
    assert!(
        !h.state
            .scene
            .as_ref()
            .unwrap()
            .entities
            .iter()
            .any(|e| e.world_item.monster_capture.is_some()),
        "Disabled VisionCaptureComponent must not spawn an Echo phantom"
    );

    {
        let mut qctx = h.state.quest_context(&mut events);
        let mut lp_ctx = level_play::LevelPlayContext::from_quest_ctx(&mut qctx).expect("lp ctx");
        level_play::on_entity_killed(&mut lp_ctx, fallback_boss_nid, Some(139_800_002))
            .expect("kill 139800002");
    }
    let phantom = h
        .state
        .scene
        .as_ref()
        .unwrap()
        .entities
        .iter()
        .find_map(|e| e.world_item.monster_capture.as_ref())
        .expect("139800002 should spawn Echo phantom using valid template VisionCaptureId");
    assert_eq!(phantom.monster_id, 39_0077_021);

    // 6. Collectible interaction (Collect106 102001084 with RefreshComponent) removes entity from scene,
    //    sets cooldown, and does not stick permanently in hidden_entities.
    let collect_nid = {
        let mut s = EntityComponentStorage::new(
            0,
            ConfigComponent {
                camp: 2,
                config_id: 102_001_084,
                config_type: ConfigType::Level,
                entity_type: EntityType::SceneItem,
                state: EntityState::Default,
            },
        );
        s.position = Some(PositionComponent {
            location: [0.0; 3],
            rotation: [0.0; 3],
        });
        h.state.scene.as_mut().unwrap().spawn(s).net_id
    };
    let _ = h.drain_count();
    let resp = scene::handle_entity_interact_request(
        &mut h.state,
        &mut events,
        EntityInteractRequest {
            EntityId: collect_nid,
            OptionIndex: 0,
            ..Default::default()
        },
    )
    .expect("interact collectible");
    assert_eq!(resp.ErrorCode, Some(ErrorCode::Success));
    assert!(h.state.scene.as_ref().unwrap().entity(collect_nid).is_none());
    let removes: Vec<EntityRemoveNotify> = h.drain_decoded(EntityRemoveNotify::MSG_ID);
    assert!(
        removes
            .iter()
            .any(|n| n.RemoveInfos.iter().any(|ri| ri.EntityId == collect_nid))
    );
    assert!(
        !h.state
            .player_components
            .quest
            .info
            .world
            .hidden_entities
            .contains(&102_001_084),
        "repeatable RefreshComponent collectible should use cooldown, not permanent hidden_entities"
    );
}

fn condition_helper_is_open(
    ctx: &sonoro_gamesv_logic::helpers::condition::EvalContext<'_>,
    function_id: i32,
) -> bool {
    sonoro_gamesv_logic::helpers::condition::is_function_open(ctx, function_id)
}

#[test]
fn audit_119_rage_hardness_switch_guard_and_wavesline_gating() {
    use sonoro_proto::pb::{
        HardnessModeChangedRequest, ShortMessageInfoRequest, ShortMessageReadRequest,
        ShortMessageUpdateNotify,
    };

    let mut h = TestHarness::new("audit_119", Some(8));
    let mut events = EventQueue::new();

    // 1. Item 8: spawn + FSM BindBuff drives Rage / 狂暴生效 without hardcoded non-FSM bypass.
    let rage_tag_id = gameplay_tags::id_from_name(
        &h.state.assets.tables.gameplay_tag_remap,
        "功能.功能制作.狂暴生效",
    )
    .expect("rage tag id");

    // 2. Item 8 Step B: Damage with HardnessLv > 0 depletes Rage in Rage state (and triggers FSM
    //    transition 10130 -> 10131 paralysis when Rage hits 0, clearing 70000045 / 狂暴生效), and
    //    builds Hardness in non-rage HardnessMode when Percent > 0.
    let mut boss_props = vec![0i32; 144];
    boss_props[EAttributeType::LifeMax.to_i32() as usize] = 100_000;
    boss_props[EAttributeType::Life.to_i32() as usize] = 100_000;
    boss_props[EAttributeType::RageMax.to_i32() as usize] = 200;
    boss_props[EAttributeType::Rage.to_i32() as usize] = 200;
    boss_props[EAttributeType::HardnessMax.to_i32() as usize] = 500;
    boss_props[EAttributeType::Hardness.to_i32() as usize] = 0;

    let mut boss_attr = AttributeComponent::create_with_modes(&boss_props, 101, 1);
    boss_attr.set_prop(EAttributeType::RageMax, 200);
    boss_attr.set_prop(EAttributeType::Rage, 200);

    let graph = h
        .state
        .assets
        .fsm_graphs
        .get("SM_MB1DaniyaMd00101")
        .expect("SM_MB1DaniyaMd00101 graph");

    let mut boss_storage = EntityComponentStorage::new(
        0,
        ConfigComponent {
            camp: 2,
            config_id: 1,
            config_type: ConfigType::Level,
            entity_type: EntityType::Monster,
            state: EntityState::Default,
        },
    );
    boss_storage.attribute = Some(boss_attr);
    boss_storage.buffs = Some(FightBuffComponent::default());
    boss_storage.logic_state = Some(LogicStateComponent::default());
    boss_storage.tag = Some(TagComponent::default());
    boss_storage.part = Some(PartComponent::default());
    boss_storage.fsm = Some(FsmComponent::from_graph(graph, 0).expect("fsm graph"));

    let boss_nid = h.state.scene.as_mut().unwrap().spawn(boss_storage).net_id;
    let attacker_nid = {
        let mut s = EntityComponentStorage::new(
            0,
            ConfigComponent {
                camp: 1,
                config_id: 1501,
                config_type: ConfigType::Character,
                entity_type: EntityType::Player,
                state: EntityState::Default,
            },
        );
        let mut props = vec![0i32; 144];
        props[EAttributeType::Atk.to_i32() as usize] = 1000;
        props[EAttributeType::LifeMax.to_i32() as usize] = 10_000;
        props[EAttributeType::Life.to_i32() as usize] = 10_000;
        s.attribute = Some(AttributeComponent::create(&props));
        s.buffs = Some(FightBuffComponent::default());
        h.state.scene.as_mut().unwrap().spawn(s).net_id
    };

    // Give the boss an active hate target so SM_Common 10131 does not immediately de-aggro back to 10130.
    {
        let sc = h.state.scene.as_mut().unwrap();
        sc.update_fsm_hate_target(boss_nid, true);
        sc.set_fsm_encounter_target(boss_nid, Some(attacker_nid))
            .expect("set encounter target");
    }

    // Verify entity spawn did NOT hardcode non-FSM Buff 70000045 before FSM wake runs.
    assert!(
        !h.state
            .scene
            .as_ref()
            .unwrap()
            .entity(boss_nid)
            .unwrap()
            .tag
            .as_ref()
            .unwrap()
            .has_tag(&h.state.assets.tables.gameplay_tag_parent, rage_tag_id),
        "Spawn must let FSM apply 狂暴生效 instead of hardcoding it pre-FSM"
    );

    // Flush initial FSM BindBuff (10130 -> 70000045 -> grants 狂暴生效) via Event::FsmWake.
    let now_ms = 1_000_000i64;
    events.enqueue(Event::FsmWake {
        entity_id: 0,
        now_ms,
    });
    crate::network::connection::drain_event_queue_best_effort(&mut events, &mut h.state);

    let boss_has_rage_buff = {
        let sc = h.state.scene.as_ref().unwrap();
        let st = sc.entity(boss_nid).unwrap();
        st.buffs
            .as_ref()
            .unwrap()
            .get_by_buff_id(70_000_045)
            .is_some()
    };
    assert!(
        boss_has_rage_buff,
        "FSM initial state 10130 must apply BindBuff 70000045 -> 狂暴生效"
    );

    let dmg = Damage {
        Id: 999_001,
        CalculateType: 0,
        Type: 1,
        FormulaType: 0,
        RateLv: vec![10_000],
        HardnessLv: vec![120],
        ..Default::default()
    };
    let req = DamageExecuteRequest {
        DamageId: 999_001,
        SkillLevel: 1,
        AttackerEntityId: attacker_nid,
        TargetEntityId: boss_nid,
        ..Default::default()
    };
    let mut pack = Vec::new();
    damage_helper::damage_entity(
        &mut pack,
        &req,
        &dmg,
        &h.state.assets,
        h.state.scene.as_mut().unwrap(),
        Some(&mut h.state.buff_timers),
    );
    let rage_after_one = attributes::get_attr_value(
        h.state
            .scene
            .as_ref()
            .unwrap()
            .entity(boss_nid)
            .unwrap()
            .attribute
            .as_ref()
            .unwrap(),
        EAttributeType::Rage,
    );
    assert_eq!(rage_after_one, 80, "120 HardnessLv hit should reduce 200 Rage to 80");

    damage_helper::damage_entity(
        &mut pack,
        &req,
        &dmg,
        &h.state.assets,
        h.state.scene.as_mut().unwrap(),
        Some(&mut h.state.buff_timers),
    );
    let rage_after_two = attributes::get_attr_value(
        h.state
            .scene
            .as_ref()
            .unwrap()
            .entity(boss_nid)
            .unwrap()
            .attribute
            .as_ref()
            .unwrap(),
        EAttributeType::Rage,
    );
    assert_eq!(rage_after_two, 0, "Second hit clamps Rage at 0");

    // Wake FSM -> sets pending transition 10130 -> 10131; client confirm commits paralysis and drops 70000045.
    events.enqueue(Event::FsmWake {
        entity_id: 0,
        now_ms: now_ms + 100,
    });
    crate::network::connection::drain_event_queue_best_effort(&mut events, &mut h.state);
    {
        let pending = h
            .state
            .scene
            .as_ref()
            .unwrap()
            .entity(boss_nid)
            .unwrap()
            .fsm
            .as_ref()
            .unwrap()
            .runtime_nodes
            .iter()
            .find(|r| r.fsm_id == 10129)
            .and_then(|r| r.pending_to);
        assert_eq!(pending, Some(10131), "Rage=0 must initiate transition 10130 -> 10131");
    }
    let confirm_resp = crate::network::handlers::fsm::handle_change_state_confirm_request(
        &mut h.state,
        &mut events,
        Some(&sonoro_proto::pb::CombatCommon {
            EntityId: boss_nid,
            ..Default::default()
        }),
        &mut pack,
        sonoro_proto::pb::ChangeStateConfirmRequest {
            FsmId: 10129,
            State: 10131,
            ..Default::default()
        },
    );
    assert_eq!(confirm_resp.State, 10132);
    crate::network::connection::drain_event_queue_best_effort(&mut events, &mut h.state);

    let sc = h.state.scene.as_ref().unwrap();
    let st = sc.entity(boss_nid).unwrap();
    assert!(
        st.buffs
            .as_ref()
            .unwrap()
            .get_by_buff_id(70_000_045)
            .is_none()
            && !st
                .tag
                .as_ref()
                .unwrap()
                .has_tag(&h.state.assets.tables.gameplay_tag_parent, rage_tag_id),
        "Paralyzed state 10131 must drop BindBuff 70000045 and 狂暴生效 tag"
    );

    // Non-rage Hardness buildup when HardnessModeChangedRequest sets an active HardnessModeId (1 = Percent 10000).
    let hp_resp = crate::network::handlers::world_puzzle::handle_hardness_mode_changed_request(
        &mut h.state,
        HardnessModeChangedRequest {
            EntityId: boss_nid,
            HardnessModeId: 1,
        },
    )
    .expect("HardnessModeChangedRequest");
    assert_eq!(hp_resp.ErrorCode, Some(ErrorCode::Success));
    damage_helper::damage_entity(
        &mut pack,
        &req,
        &dmg,
        &h.state.assets,
        h.state.scene.as_mut().unwrap(),
        Some(&mut h.state.buff_timers),
    );
    let hardness_after = attributes::get_attr_value(
        h.state
            .scene
            .as_ref()
            .unwrap()
            .entity(boss_nid)
            .unwrap()
            .attribute
            .as_ref()
            .unwrap(),
        EAttributeType::Hardness,
    );
    assert_eq!(hardness_after, 120, "Active HardnessModeId=1 builds 120 Hardness");

    // 3. Item 3 (Opsi 1): 139000025 Guide 10043/10044 waits when >= 2 roles exist, auto-completes if < 2 roles.
    let owner_nodes = h
        .state
        .assets
        .quest_nodes
        .get_owner_nodes(quest_engine::PROLOGUE_QUEST_ID)
        .expect("139000025 nodes");
    let single_node = owner_nodes
        .iter()
        .find(|n| {
            n.node_type == "ChildQuest"
                && n.data
                    .get("Condition")
                    .and_then(|c| c.get("GuideGroupId"))
                    .and_then(|v| v.as_i64())
                    == Some(10043)
        })
        .cloned()
        .expect("139000025 Guide 10043 child node");
    {
        let sc = h.state.scene.as_mut().unwrap();
        sc.formation_info.cur_formation = 0;
        sc.formation_info.formations[0] = Formation {
            cur_role: 1501,
            name: None,
            roles: [
                Some(FormationRole {
                    role_id: 1501,
                    entity_id: attacker_nid,
                    on_stage_without_control: false,
                }),
                None,
                None,
            ],
        };
    }
    let auto_single = quest_engine::triggers::eval_child_condition_auto(
        &h.state.quest_context(&mut events),
        quest_engine::PROLOGUE_QUEST_ID,
        &single_node,
    );
    assert!(
        auto_single,
        "139000025 switch-role tutorial must auto-complete when active party has < 2 roles"
    );
    {
        let sc = h.state.scene.as_mut().unwrap();
        sc.formation_info.formations[0].roles[1] = Some(FormationRole {
            role_id: 1402,
            entity_id: attacker_nid + 1,
            on_stage_without_control: false,
        });
    }
    let auto_multi = quest_engine::triggers::eval_child_condition_auto(
        &h.state.quest_context(&mut events),
        quest_engine::PROLOGUE_QUEST_ID,
        &single_node,
    );
    assert!(
        !auto_multi,
        "139000025 switch-role tutorial should remain active when party has >= 2 roles"
    );

    // 4. WavesLine ShortMessage gating: empty before triggers, unlocks only triggered message.
    let initial_sm = crate::network::handlers::qol::handle_short_message_info_request(
        &mut h.state,
        ShortMessageInfoRequest::default(),
    )
    .expect("short message info");
    assert!(
        initial_sm.ShortMessageInfos.is_empty(),
        "Fresh account must start with 0 WavesLine messages unlocked"
    );

    let send_msg_act: Value = serde_json::from_str(
        r#"{"Name":"PhoneSystemInteract","Params":{"InteractType":{"Type":"SendPhoneMessage","PhoneRecordActionConfig":{"PhoneMessageId":30015}}}}"#,
    )
    .unwrap();
    let _ = h.drain_count();
    execute_action(
        &mut h.state.quest_context(&mut events),
        &mut ActionCallbacks::default(),
        196_700_003,
        2,
        &send_msg_act,
    )
    .expect("SendPhoneMessage");

    let updates: Vec<ShortMessageUpdateNotify> = h.drain_decoded(ShortMessageUpdateNotify::MSG_ID);
    assert_eq!(updates.len(), 1);
    assert_eq!(updates[0].Adds.len(), 1);
    assert_eq!(updates[0].Adds[0].ConfigId, 30015);
    assert!(
        !h.state
            .player_components
            .quest
            .info
            .progress
            .read_phone_messages
            .contains(&30015),
        "SendPhoneMessage must unlock without prematurely marking the message read"
    );

    let after_unlock = crate::network::handlers::qol::handle_short_message_info_request(
        &mut h.state,
        ShortMessageInfoRequest::default(),
    )
    .expect("short message info after unlock");
    assert_eq!(after_unlock.ShortMessageInfos.len(), 1);
    assert_eq!(after_unlock.ShortMessageInfos[0].ConfigId, 30015);
    assert!(!after_unlock.ShortMessageInfos[0].IsRead);

    let read_resp = crate::network::handlers::qol::handle_short_message_read_request(
        &mut h.state,
        &mut events,
        ShortMessageReadRequest { ConfigId: 30015 },
    )
    .expect("read short message");
    assert_eq!(read_resp.ErrCode, Some(ErrorCode::Success));
    assert!(
        h.state
            .player_components
            .quest
            .info
            .progress
            .read_phone_messages
            .contains(&30015)
    );

    let del_msg_act: Value = serde_json::from_str(
        r#"{"Name":"PhoneSystemInteract","Params":{"InteractType":{"Type":"DeletePhoneMessage","PhoneMessageId":30015}}}"#,
    )
    .unwrap();
    let _ = h.drain_count();
    execute_action(
        &mut h.state.quest_context(&mut events),
        &mut ActionCallbacks::default(),
        196_700_003,
        2,
        &del_msg_act,
    )
    .expect("DeletePhoneMessage");
    let del_updates: Vec<ShortMessageUpdateNotify> = h.drain_decoded(ShortMessageUpdateNotify::MSG_ID);
    assert_eq!(del_updates.len(), 1);
    assert_eq!(del_updates[0].Removes, vec![30015]);
    let after_del = crate::network::handlers::qol::handle_short_message_info_request(
        &mut h.state,
        ShortMessageInfoRequest::default(),
    )
    .expect("short message info after delete");
    assert!(after_del.ShortMessageInfos.is_empty());
}

#[test]
fn audit_120_tune_break_elite_regen_and_active_flow_collectibles() {
    use crate::network::handlers::damage as net_damage;
    use crate::network::handlers::scene as net_scene;
    use sonoro_gamesv_data::tables::damage::Damage;
    use sonoro_gamesv_logic::component::entity::{
        AttributeComponent, ConfigComponent, FightBuffComponent, FsmComponent, LogicStateComponent,
        PartComponent, PositionComponent, TagComponent,
        small::{ConfigType, EntityState, EntityType},
    };
    use sonoro_gamesv_logic::events::Event;
    use sonoro_gamesv_logic::helpers::damage as damage_helper;
    use sonoro_gamesv_logic::helpers::gameplay_tags;
    use sonoro_gamesv_logic::helpers::level_play;
    use sonoro_proto::enums::{EAttributeType, ErrorCode};
    use sonoro_proto::pb::{
        AnimalDropRequest, DamageExecuteRequest, EntityInteractRequest, EntityRemoveNotify,
    };

    let mut h = TestHarness::new("audit_120", Some(8));
    let mut events = EventQueue::new();

    // 1. Item #11 (Tune Break): WeaknessBuildUp fill -> Buff 3158 (FullTag) -> IsBreakWeakness
    //    scaling via WeaknessMastery -> ChangeWeakness=1 -> resets buildup & applies Buff 3197 (Elite CD)
    //    which strips 3158 and blocks buildup while CdTag is active.
    let full_tid = gameplay_tags::id_from_name(
        &h.state.assets.tables.gameplay_tag_remap,
        damage_helper::TAG_WEAKNESS_FULL,
    );
    let cd_tid = gameplay_tags::id_from_name(
        &h.state.assets.tables.gameplay_tag_remap,
        damage_helper::TAG_WEAKNESS_CD,
    );

    let attacker_nid = {
        let mut s = EntityComponentStorage::new(
            0,
            ConfigComponent {
                camp: 1,
                config_id: 1402,
                config_type: ConfigType::Character,
                entity_type: EntityType::Player,
                state: EntityState::Default,
            },
        );
        let mut props = vec![0i32; 144];
        props[EAttributeType::Lv.to_i32() as usize] = 10;
        props[EAttributeType::Atk.to_i32() as usize] = 1000;
        props[EAttributeType::LifeMax.to_i32() as usize] = 10_000;
        props[EAttributeType::Life.to_i32() as usize] = 10_000;
        props[EAttributeType::BreakWeaknessRatio.to_i32() as usize] = 10_000;
        props[EAttributeType::WeaknessMastery.to_i32() as usize] = 100;
        s.attribute = Some(AttributeComponent::create(&props));
        s.buffs = Some(FightBuffComponent::default());
        h.state.scene.as_mut().unwrap().spawn(s).net_id
    };

    let mut elite_props = vec![0i32; 144];
    elite_props[EAttributeType::Lv.to_i32() as usize] = 5;
    elite_props[EAttributeType::LifeMax.to_i32() as usize] = 100_000;
    elite_props[EAttributeType::Life.to_i32() as usize] = 100_000;
    elite_props[EAttributeType::HardnessMax.to_i32() as usize] = 27_950;
    elite_props[EAttributeType::Hardness.to_i32() as usize] = 0;
    elite_props[EAttributeType::HardnessRecover.to_i32() as usize] = 1_863;
    elite_props[EAttributeType::RageMax.to_i32() as usize] = 26_503;
    elite_props[EAttributeType::Rage.to_i32() as usize] = 26_503;
    elite_props[EAttributeType::WeaknessBuildUpMax.to_i32() as usize] = 10_000;
    elite_props[EAttributeType::WeaknessBuildUp.to_i32() as usize] = 0;

    let elite_graph = h
        .state
        .assets
        .fsm_graphs
        .get("SM_ME1QunJingMd00001_New")
        .expect("Antanke graph");
    let mut elite_storage = EntityComponentStorage::new(
        0,
        ConfigComponent {
            camp: 2,
            config_id: 139_800_002,
            config_type: ConfigType::Level,
            entity_type: EntityType::Monster,
            state: EntityState::Default,
        },
    );
    elite_storage.attribute = Some(AttributeComponent::create_with_modes(&elite_props, 101, 1));
    elite_storage.buffs = Some(FightBuffComponent::default());
    elite_storage.logic_state = Some(LogicStateComponent::default());
    elite_storage.tag = Some(TagComponent::default());
    elite_storage.part = Some(PartComponent::default());
    elite_storage.fsm = Some(FsmComponent::from_graph(elite_graph, 0).expect("fsm graph"));
    let elite_nid = h.state.scene.as_mut().unwrap().spawn(elite_storage).net_id;

    {
        let sc = h.state.scene.as_mut().unwrap();
        sc.update_fsm_hate_target(elite_nid, true);
        sc.set_fsm_encounter_target(elite_nid, Some(attacker_nid))
            .expect("set encounter");
    }
    events.enqueue(Event::FsmWake {
        entity_id: 0,
        now_ms: 1_000_000,
    });
    crate::network::connection::drain_event_queue_best_effort(&mut events, &mut h.state);

    // Item #8 check on Antanke 139800002: initiates conduit 10026 -> 10029, confirms into C白条狂暴 (70000045) + attaches Buff 1102.
    {
        let pending = h
            .state
            .scene
            .as_ref()
            .unwrap()
            .entity(elite_nid)
            .unwrap()
            .fsm
            .as_ref()
            .unwrap()
            .runtime_nodes
            .iter()
            .find(|r| r.fsm_id == 10023)
            .and_then(|r| r.pending_to);
        assert_eq!(pending, Some(10029), "Antanke must initiate 10026 -> 10029 when Rage > 0");
    }
    let mut pack = Vec::new();
    let confirm_elite = crate::network::handlers::fsm::handle_change_state_confirm_request(
        &mut h.state,
        &mut events,
        Some(&sonoro_proto::pb::CombatCommon {
            EntityId: elite_nid,
            ..Default::default()
        }),
        &mut pack,
        sonoro_proto::pb::ChangeStateConfirmRequest {
            FsmId: 10_023,
            State: 10_029,
            ..Default::default()
        },
    );
    assert!(
        confirm_elite.Error.is_none() || confirm_elite.Error.as_ref().is_some_and(|e| e.ErrorCode == Some(ErrorCode::Success)),
        "confirm_elite failed: {confirm_elite:?}"
    );
    crate::network::connection::drain_event_queue_best_effort(&mut events, &mut h.state);
    {
        let sc = h.state.scene.as_ref().unwrap();
        let st = sc.entity(elite_nid).unwrap();
        let buffs = st.buffs.as_ref().unwrap();
        assert!(
            buffs.get_by_buff_id(70_000_045).is_some(),
            "Antanke should enter C白条狂暴 (70000045); confirm={confirm_elite:?}, runtime={:?}, buffs={:?}",
            st.fsm.as_ref().unwrap().runtime_nodes,
            buffs.buffs.iter().map(|b| (b.BuffId, b.IsActive)).collect::<Vec<_>>()
        );
        assert!(buffs.get_by_buff_id(1102).is_some(), "Antanke should receive passive HardnessRecover Buff 1102");
    }

    // While 狂暴生效 (70000045) is active, Buff 1102 periodic tick must NOT recover Hardness.
    let buff_1102 = h.state.assets.tables.buff.get_data_by_id(&1102).unwrap().into_owned();
    sonoro_gamesv_logic::helpers::buff::execute_periodic_with_assets(
        &mut pack,
        elite_nid,
        elite_nid,
        &buff_1102,
        h.state.scene.as_mut().unwrap(),
        &h.state.assets,
    );
    assert_eq!(
        h.state.scene.as_ref().unwrap().entity(elite_nid).unwrap().attribute.as_ref().unwrap().get_current(EAttributeType::Hardness),
        0,
        "Hardness must stay 0 during 狂暴生效"
    );

    // Remove 70000045 (simulating entry into C白条常态) and tick Buff 1102 once: Hardness recovers by 1863 * 0.1 = 186.
    let (elite_handle, rage_handle) = {
        let sc = h.state.scene.as_ref().unwrap();
        let eh = sc.entity_handle(elite_nid).unwrap();
        let rh = sc.entity(elite_nid).unwrap().buffs.as_ref().unwrap().get_by_buff_id(70_000_045).unwrap().HandleId;
        (eh, rh)
    };
    sonoro_gamesv_logic::handlers::buff::remove_buff_from_entity(
        &mut h.state.conn,
        &mut events,
        h.state.scene.as_mut().unwrap(),
        &h.state.assets,
        &mut h.state.buff_timers,
        elite_handle,
        &[rage_handle],
        None,
    );
    sonoro_gamesv_logic::helpers::buff::execute_periodic_with_assets(
        &mut pack,
        elite_nid,
        elite_nid,
        &buff_1102,
        h.state.scene.as_mut().unwrap(),
        &h.state.assets,
    );
    assert_eq!(
        h.state.scene.as_ref().unwrap().entity(elite_nid).unwrap().attribute.as_ref().unwrap().get_current(EAttributeType::Hardness),
        186,
        "Buff 1102 must recover HardnessRecover * Period (1863 * 0.1 = 186) in C白条常态"
    );

    // Pick a real Damage ID with WeaknessLvl from assets (1202004001 -> WeakLv [10400]) + break hit (1000009900)
    let _ = h.drain_count();
    let req_build = DamageExecuteRequest {
        DamageId: 1_202_004_001,
        SkillLevel: 1,
        AttackerEntityId: attacker_nid,
        TargetEntityId: elite_nid,
        ..Default::default()
    };
    let build_resp = net_damage::handle_damage_execute_request(&mut h.state, &mut events, &mut pack, req_build);
    assert_eq!(build_resp.ErrorCode, Some(ErrorCode::Success));
    assert_eq!(build_resp.ChangeWeakness, 0);
    {
        let st = h.state.scene.as_ref().unwrap().entity(elite_nid).unwrap();
        assert_eq!(st.attribute.as_ref().unwrap().get_current(EAttributeType::WeaknessBuildUp), 10_000);
        assert!(st.buffs.as_ref().unwrap().get_by_buff_id(3158).is_some(), "Full WeaknessBuildUp must grant Buff 3158");
        assert!(damage_helper::entity_has_gameplay_tag(st, &h.state.assets, full_tid));
    }

    // Compare normal 1000009900 damage vs IsBreakWeakness=true with WeaknessMastery=100 (2.0x)
    let req_break = DamageExecuteRequest {
        DamageId: 1_000_009_900,
        SkillLevel: 1,
        AttackerEntityId: attacker_nid,
        TargetEntityId: elite_nid,
        IsBreakWeakness: true,
        ..Default::default()
    };
    let break_resp = net_damage::handle_damage_execute_request(&mut h.state, &mut events, &mut pack, req_break);
    assert_eq!(break_resp.ErrorCode, Some(ErrorCode::Success));
    assert_eq!(break_resp.ChangeWeakness, 1, "IsBreakWeakness on FullTag target must return ChangeWeakness=1");
    assert!(break_resp.Damage > 0);
    {
        let st = h.state.scene.as_ref().unwrap().entity(elite_nid).unwrap();
        assert_eq!(st.attribute.as_ref().unwrap().get_current(EAttributeType::WeaknessBuildUp), 0, "Tune Break must reset WeaknessBuildUp to 0");
        assert!(st.buffs.as_ref().unwrap().get_by_buff_id(3158).is_none(), "Cooldown buff 3197 must remove FullTag buff 3158");
        assert!(st.buffs.as_ref().unwrap().get_by_buff_id(3197).is_some(), "Elite monster must enter 6s Tune Break cooldown Buff 3197");
        assert!(damage_helper::entity_has_gameplay_tag(st, &h.state.assets, cd_tid));
    }

    // While Buff 3197 CdTag is active, WeaknessLvl hit must NOT add WeaknessBuildUp
    let cd_hit = Damage {
        Id: 999_120,
        CalculateType: 0,
        Type: 0,
        FormulaType: 0,
        RateLv: vec![1_000],
        WeaknessLvl: vec![5_000],
        ..Default::default()
    };
    let _ = damage_helper::damage_entity(
        &mut pack,
        &DamageExecuteRequest {
            DamageId: 999_120,
            SkillLevel: 1,
            AttackerEntityId: attacker_nid,
            TargetEntityId: elite_nid,
            ..Default::default()
        },
        &cd_hit,
        &h.state.assets,
        h.state.scene.as_mut().unwrap(),
        Some(&mut h.state.buff_timers),
    );
    assert_eq!(
        h.state.scene.as_ref().unwrap().entity(elite_nid).unwrap().attribute.as_ref().unwrap().get_current(EAttributeType::WeaknessBuildUp),
        0,
        "CdTag must prevent WeaknessBuildUp accumulation during cooldown"
    );

    // 2. Item #10: Collect106 (136001964) and Animal028 (136001965) collected cleanly even when active_flow_inc_id != 0
    h.state.scene.as_mut().unwrap().active_flow_inc_id = 42;
    let (plant_nid, lizard_interact_nid, lizard_drop_nid) = {
        let mut s1 = EntityComponentStorage::new(
            0,
            ConfigComponent {
                camp: 2,
                config_id: 136_001_964,
                config_type: ConfigType::Level,
                entity_type: EntityType::SceneItem,
                state: EntityState::Default,
            },
        );
        s1.position = Some(PositionComponent { location: [0.0; 3], rotation: [0.0; 3] });
        let mut s2 = EntityComponentStorage::new(
            0,
            ConfigComponent {
                camp: 6,
                config_id: 136_001_965,
                config_type: ConfigType::Level,
                entity_type: EntityType::Animal,
                state: EntityState::Default,
            },
        );
        s2.position = Some(PositionComponent { location: [10.0; 3], rotation: [0.0; 3] });
        let s3 = s2.clone();
        let sc = h.state.scene.as_mut().unwrap();
        (sc.spawn(s1).net_id, sc.spawn(s2).net_id, sc.spawn(s3).net_id)
    };
    let _ = h.drain_count();
    let r1 = net_scene::handle_entity_interact_request(
        &mut h.state,
        &mut events,
        EntityInteractRequest { EntityId: plant_nid, OptionIndex: 0, ..Default::default() },
    ).expect("Collect106 interact during flow");
    let r2 = net_scene::handle_entity_interact_request(
        &mut h.state,
        &mut events,
        EntityInteractRequest { EntityId: lizard_interact_nid, OptionIndex: 0, ..Default::default() },
    ).expect("Animal028 interact during flow");
    let r3 = net_scene::handle_animal_drop_request(
        &mut h.state,
        &mut events,
        AnimalDropRequest { EntityId: lizard_drop_nid },
    ).expect("Animal028 drop during flow");
    assert_eq!(r1.ErrorCode, Some(ErrorCode::Success));
    assert_eq!(r2.ErrorCode, Some(ErrorCode::Success));
    assert_eq!(r3.ErrorCode, Some(ErrorCode::Success));
    let sc = h.state.scene.as_ref().unwrap();
    assert!(sc.entity(plant_nid).is_none(), "136001964 must be removed from scene during active flow");
    assert!(sc.entity(lizard_interact_nid).is_none(), "136001965 must be removed on interact during active flow");
    assert!(sc.entity(lizard_drop_nid).is_none(), "136001965 must be removed on AnimalDropRequest during active flow");
    let removes: Vec<EntityRemoveNotify> = h.drain_decoded(EntityRemoveNotify::MSG_ID);
    assert!(removes.iter().flat_map(|n| n.RemoveInfos.iter()).any(|ri| ri.EntityId == plant_nid));
    assert!(removes.iter().flat_map(|n| n.RemoveInfos.iter()).any(|ri| ri.EntityId == lizard_interact_nid));
    assert!(removes.iter().flat_map(|n| n.RemoveInfos.iter()).any(|ri| ri.EntityId == lizard_drop_nid));
    assert!(level_play::resolve_entity_config(&h.state.assets, 8, 136_001_965).is_some());
}

#[test]
fn audit_client_log_fixes_lock_hp_guide_chest_and_multiwave() {
    let mut h = TestHarness::new("audit_client_log_fixes", Some(8));
    let mut events = EventQueue::default();
    let _ = h.drain_count();

    // 1. Invulnerability parity + Lock-HP (ExtraEffectID == 16) + duplicate death guard.
    let (atk_net, boss_net) = {
        let mut atk_attr = AttributeComponent::default();
        atk_attr.set_prop(EAttributeType::Atk, 1_000);
        atk_attr.set_prop(EAttributeType::LifeMax, 10_000);
        atk_attr.set_prop(EAttributeType::Life, 10_000);
        let mut atk_s = EntityComponentStorage::new(
            0,
            ConfigComponent {
                camp: 1,
                config_id: 1104,
                config_type: ConfigType::Character,
                entity_type: EntityType::Player,
                state: EntityState::Default,
            },
        );
        atk_s.attribute = Some(atk_attr);

        let mut def_attr = AttributeComponent::default();
        def_attr.set_prop(EAttributeType::LifeMax, 1_000);
        def_attr.set_prop(EAttributeType::Life, 1_000);
        let mut boss_s = EntityComponentStorage::new(
            0,
            ConfigComponent {
                camp: 2,
                config_id: 990_001,
                config_type: ConfigType::Level,
                entity_type: EntityType::Monster,
                state: EntityState::Default,
            },
        );
        boss_s.attribute = Some(def_attr);
        boss_s.buffs = Some(FightBuffComponent::default());
        boss_s.tag = Some(TagComponent::default());
        boss_s.fsm = Some(FsmComponent::from_graph(Arc::new(Graph::default()), 0).unwrap());

        let sc = h.state.scene.as_mut().unwrap();
        sc.spawn_with_net_id(910_101, atk_s).unwrap();
        sc.spawn_with_net_id(910_102, boss_s).unwrap();
        (910_101, 910_102)
    };

    let inv_tag = gameplay_tags::id_from_name(
        &h.state.assets.tables.gameplay_tag_remap,
        "功能.逻辑状态标识.无敌",
    )
    .unwrap_or(1_918_148_596);
    h.state
        .scene
        .as_mut()
        .unwrap()
        .entity_mut(boss_net)
        .unwrap()
        .tag
        .as_mut()
        .unwrap()
        .add_tag(inv_tag, 1);

    let mut dmg_cfg = Damage {
        Id: 910_001,
        FormulaType: 0,
        CalculateType: 0,
        ImmuneType: 0,
        RelatedProperty: EAttributeType::Atk.to_i32(),
        RateLv: vec![50_000],
        FluctuationLower: vec![10_000],
        FluctuationUpper: vec![10_000],
        ..Default::default()
    };
    let mut pack: Vec<CombatReceiveData> = Vec::new();
    let blocked = damage_helper::damage_entity(
        &mut pack,
        &DamageExecuteRequest {
            DamageId: dmg_cfg.Id,
            SkillLevel: 1,
            AttackerEntityId: atk_net,
            TargetEntityId: boss_net,
            RandomSeed: 5_000,
            ..Default::default()
        },
        &dmg_cfg,
        &h.state.assets,
        h.state.scene.as_mut().unwrap(),
        None,
    );
    assert_eq!(blocked.ImmuneType, Some(EDamageImmune::Invincible));
    assert_eq!(blocked.Damage, 0);

    // Non-zero ImmuneType bypasses the invulnerable tag per client BaseDamageComponent.
    dmg_cfg.ImmuneType = 1;
    let lock_buff_id = h
        .state
        .assets
        .tables
        .buff
        .get_data_by_id(&41_601)
        .filter(|b| b.ExtraEffectID == 16)
        .map(|b| b.Id)
        .expect("asset table must contain ExtraEffectID=16 Life Lock-HP buff 41601");
    {
        let sc = h.state.scene.as_mut().unwrap();
        let st = sc.entity_mut(boss_net).unwrap();
        st.tag.as_mut().unwrap().remove_tag(inv_tag);
        st.buffs.as_mut().unwrap().buffs.push(sonoro_proto::pb::FightBuffInformation {
            BuffId: lock_buff_id,
            HandleId: 77,
            Level: 1,
            IsActive: true,
            ..Default::default()
        });
    }
    let expected_floor = {
        let sc = h.state.scene.as_ref().unwrap();
        let st = sc.entity(boss_net).unwrap();
        buff_helper::lock_hp_floor(st, &h.state.assets).expect("active lock_hp_floor")
    };
    assert!(expected_floor >= 1 && expected_floor <= 1_000);

    pack.clear();
    let _ = damage_helper::damage_entity(
        &mut pack,
        &DamageExecuteRequest {
            DamageId: dmg_cfg.Id,
            SkillLevel: 1,
            AttackerEntityId: atk_net,
            TargetEntityId: boss_net,
            RandomSeed: 5_000,
            ..Default::default()
        },
        &dmg_cfg,
        &h.state.assets,
        h.state.scene.as_mut().unwrap(),
        None,
    );
    {
        let sc = h.state.scene.as_ref().unwrap();
        let st = sc.entity(boss_net).unwrap();
        let cur_hp = attributes::get_attr_value(st.attribute.as_ref().unwrap(), EAttributeType::Life);
        assert!(
            cur_hp >= expected_floor,
            "Lock-HP buff must prevent HP ({cur_hp}) from dropping below floor ({expected_floor})"
        );
        assert!(!st.dead_notified, "locked boss must not emit dead_notified");
    }

    // Remove Lock-HP buff and deal lethal damage twice -> EntityLivingStatusNotify(Dead) emitted once.
    h.state
        .scene
        .as_mut()
        .unwrap()
        .entity_mut(boss_net)
        .unwrap()
        .buffs
        .as_mut()
        .unwrap()
        .buffs
        .clear();
    pack.clear();
    for _ in 0..2 {
        let _ = damage_helper::damage_entity(
            &mut pack,
            &DamageExecuteRequest {
                DamageId: dmg_cfg.Id,
                SkillLevel: 1,
                AttackerEntityId: atk_net,
                TargetEntityId: boss_net,
                RandomSeed: 5_000,
                ..Default::default()
            },
            &dmg_cfg,
            &h.state.assets,
            h.state.scene.as_mut().unwrap(),
            None,
        );
    }
    let dead_notifies = pack
        .iter()
        .filter(|entry| {
            matches!(
                &entry.Message,
                Some(CombatReceiveDataMessageOneof::CombatNotifyData(cn))
                    if matches!(cn.Message, Some(CombatNotifyDataMessageOneof::EntityLivingStatusNotify(_)))
            )
        })
        .count();
    assert_eq!(dead_notifies, 1, "EntityLivingStatusNotify(Dead) must only fire once per death");
    assert!(h.state.scene.as_ref().unwrap().entity(boss_net).unwrap().dead_notified);

    // 2. Tutorials: GuideTrigger pushes GuideCondDoneNotify without pre-completing, and GuideTriggerRequest does not finish.
    let guide_act: Value =
        serde_json::from_str(r#"{"Name":"GuideTrigger","Params":{"GuideId":10001}}"#).unwrap();
    {
        let mut qctx = h.state.quest_context(&mut events);
        let mut cbs = ActionCallbacks::default();
        execute_action(&mut qctx, &mut cbs, 0, 0, &guide_act).expect("GuideTrigger");
    }
    let guide_notifies: Vec<sonoro_proto::pb::GuideCondDoneNotify> =
        h.drain_decoded(sonoro_proto::pb::GuideCondDoneNotify::MSG_ID);
    assert_eq!(guide_notifies.len(), 1);
    assert_eq!(guide_notifies[0].GroupId, 10_001);
    assert!(
        !h.state.player_components.guide.info.has_finished(10_001),
        "GuideTrigger must not pre-complete tutorial before client finishes it"
    );
    let guide_resp = crate::network::handlers::guide::handle_guide_trigger_request(
        &mut h.state,
        &mut events,
        sonoro_proto::pb::GuideTriggerRequest { GroupId: 10_001 },
    )
    .expect("GuideTriggerRequest");
    assert_eq!(guide_resp.ErrorCode, Some(ErrorCode::Success));
    assert!(
        !h.state.player_components.guide.info.has_finished(10_001),
        "GuideTriggerRequest (on opening) must not mark tutorial finished"
    );

    // 3. Multi-wave MonsterCreator target expansion.
    assert!(h.state.assets.has_spawn_monster_component(8, 102_700_037));
    assert!(
        h.state.assets.spawn_monster_targets(8, 102_700_037).len() > 1,
        "multi-wave SpawnMonsterComponent 102700037 must expand to all configured monster targets"
    );
}

#[test]
fn audit_issue4_jinzhou_early_game_fixes() {
    let mut h = TestHarness::new("audit_issue4_fixes", Some(8));
    let mut events = EventQueue::default();
    let _ = h.drain_count();

    // 1. Elevator 109002461 updates quest entity state tag ("关卡.电梯.上升") and emits ElevatorMoveNotify.
    let lift_nid = {
        let mut s = EntityComponentStorage::new(
            0,
            ConfigComponent {
                camp: 0,
                config_id: 109_002_461,
                config_type: ConfigType::Level,
                entity_type: EntityType::SceneItem,
                state: EntityState::Default,
            },
        );
        s.world_item.lift = Some(sonoro_gamesv_logic::component::entity::LiftComponent { location: 1 });
        h.state.scene.as_mut().unwrap().spawn(s).net_id
    };
    let elev_resp = world_puzzle::handle_elevator_state_change_request(
        &mut h.state,
        sonoro_proto::pb::ElevatorStateChangeRequest {
            EntityId: lift_nid,
            State: Some(sonoro_proto::enums::ElevatorState::Forward),
            TagState: 2,
        },
    )
    .expect("elevator state change");
    assert_eq!(elev_resp.ErrorCode, Some(ErrorCode::Success));
    assert_eq!(
        h.state.player_components.quest.info.get_entity_state(109_002_461),
        Some("关卡.电梯.上升")
    );
    let elev_moves: Vec<sonoro_proto::pb::ElevatorMoveNotify> =
        h.drain_decoded(sonoro_proto::pb::ElevatorMoveNotify::MSG_ID);
    assert!(
        elev_moves.iter().any(|m| m.EntityId == lift_nid && m.Location == 2),
        "Forward elevator change must emit ElevatorMoveNotify for floor 2"
    );
    // Proto3 omits default End (0) => State: None must map to 关卡.电梯.停靠 without ElevatorMoveNotify.
    let end_none_resp = world_puzzle::handle_elevator_state_change_request(
        &mut h.state,
        sonoro_proto::pb::ElevatorStateChangeRequest {
            EntityId: lift_nid,
            State: None,
            TagState: 1,
        },
    )
    .expect("elevator none end");
    assert_eq!(end_none_resp.ErrorCode, Some(ErrorCode::Success));
    assert_eq!(
        h.state.player_components.quest.info.get_entity_state(109_002_461),
        Some("关卡.电梯.停靠")
    );
    assert!(
        h.drain_decoded::<sonoro_proto::pb::ElevatorMoveNotify>(
            sonoro_proto::pb::ElevatorMoveNotify::MSG_ID
        )
        .is_empty(),
        "State: None (proto3 End) must not emit ElevatorMoveNotify"
    );

    // 2. ChangePhantomFormation switches SceneGroup to Phantom (Group 2), spawns Echo role, allows SwitchRoleRequest, and RestorePhantomFormation reverts to Battle (Group 1).
    {
        let act: Value = serde_json::from_str(
            r#"{"Name":"ChangePhantomFormation","Params":{"FormationId":1}}"#,
        )
        .unwrap();
        let mut qctx = h.state.quest_context(&mut events);
        let mut cbs = ActionCallbacks::default();
        execute_action(&mut qctx, &mut cbs, 0, 0, &act).expect("ChangePhantomFormation");
    }
    assert!(
        h.state
            .scene
            .as_ref()
            .unwrap()
            .phantom_role_entity_id(5_001_001)
            .is_some(),
        "ChangePhantomFormation must spawn Echo role 5001001"
    );
    let mut grp_updates: Vec<sonoro_proto::pb::UpdateGroupFormationNotify> = Vec::new();
    let mut sw_groups_enter: Vec<sonoro_proto::pb::SwitchSceneGroupNotify> = Vec::new();
    while let Ok(frame) = h.rx.try_recv() {
        if frame.len() >= 3
            && let Ok(msg) = super::Message::decode(&frame[3..])
        {
            match msg.header.message_id() {
                sonoro_proto::pb::UpdateGroupFormationNotify::MSG_ID => {
                    if let Ok(v) = <sonoro_proto::pb::UpdateGroupFormationNotify as sonoro_proto::wire::PbMessage>::decode(&msg.payload) {
                        grp_updates.push(v);
                    }
                }
                sonoro_proto::pb::SwitchSceneGroupNotify::MSG_ID => {
                    if let Ok(v) = <sonoro_proto::pb::SwitchSceneGroupNotify as sonoro_proto::wire::PbMessage>::decode(&msg.payload) {
                        sw_groups_enter.push(v);
                    }
                }
                _ => {}
            }
        }
    }
    assert!(
        grp_updates
            .iter()
            .flat_map(|n| &n.GroupFormation)
            .any(|gf| gf.CurrentGroupType == sonoro_proto::enums::ESceneRolesGroup::Phantom.to_i32()),
        "ChangePhantomFormation must push UpdateGroupFormationNotify for Phantom group"
    );
    assert!(
        sw_groups_enter
            .iter()
            .any(|n| n.Group == sonoro_proto::enums::ESceneRolesGroup::Phantom.to_i32()),
        "ChangePhantomFormation must emit SwitchSceneGroupNotify to Phantom"
    );
    let sw_role_resp = crate::network::handlers::role::handle_switch_role_request(
        &mut h.state,
        sonoro_proto::pb::SwitchRoleRequest {
            RoleId: 5_001_001,
            ..Default::default()
        },
    );
    assert_eq!(
        sw_role_resp.ErrorCode,
        Some(ErrorCode::Success),
        "SwitchRoleRequest for spawned Echo role 5001001 must succeed"
    );
    {
        let act: Value =
            serde_json::from_str(r#"{"Name":"RestorePhantomFormation","Params":{}}"#).unwrap();
        let mut qctx = h.state.quest_context(&mut events);
        let mut cbs = ActionCallbacks::default();
        execute_action(&mut qctx, &mut cbs, 0, 0, &act).expect("RestorePhantomFormation");
    }
    assert!(
        h.state
            .scene
            .as_ref()
            .unwrap()
            .phantom_role_entity_ids
            .is_empty(),
        "RestorePhantomFormation must remove phantom role entities"
    );
    let sw_groups_exit: Vec<sonoro_proto::pb::SwitchSceneGroupNotify> =
        h.drain_decoded(sonoro_proto::pb::SwitchSceneGroupNotify::MSG_ID);
    assert!(
        sw_groups_exit
            .iter()
            .any(|n| n.Group == sonoro_proto::enums::ESceneRolesGroup::Battle.to_i32()),
        "RestorePhantomFormation must emit SwitchSceneGroupNotify back to Battle"
    );

    // 2b. EnableNearbyTracking pushes EnableNearbyTrackingNotify for self entity.
    {
        let act: Value =
            serde_json::from_str(r#"{"Name":"EnableNearbyTracking","Params":{"IsEnable":true}}"#)
                .unwrap();
        let mut qctx = h.state.quest_context(&mut events);
        let mut cbs = ActionCallbacks::default();
        execute_action(&mut qctx, &mut cbs, 109_002_461, 0, &act)
            .expect("EnableNearbyTracking");
    }
    let track_notifies: Vec<sonoro_proto::pb::EnableNearbyTrackingNotify> =
        h.drain_decoded(sonoro_proto::pb::EnableNearbyTrackingNotify::MSG_ID);
    assert_eq!(track_notifies.len(), 1);
    assert_eq!(track_notifies[0].EntityIds, vec![lift_nid]);
    assert!(track_notifies[0].IsEnable);

    // 3. Interrupting active Flow closes black screen cleanly via FlowServerSkipNotify { FadeOutScreen: true }.
    {
        let sc = h.state.scene.as_mut().unwrap();
        sc.active_flow_inc_id = 42;
        sc.active_flow_quest_id = 139_000_027;
        sc.active_flow_list_name = "Flow_Jinzhou_Test".to_owned();
        sc.active_flow_id = 3;
        sc.active_flow_state_id = 9;
        sc.client_black_screen_open = true;
    }
    {
        let mut qctx = h.state.quest_context(&mut events);
        let ended = quest_engine::interrupt_active_flow_for_quest(&mut qctx, 139_000_027);
        assert_eq!(ended, Some(42));
    }
    let flow_skips: Vec<sonoro_proto::pb::FlowServerSkipNotify> =
        h.drain_decoded(sonoro_proto::pb::FlowServerSkipNotify::MSG_ID);
    assert!(
        flow_skips.iter().any(|n| n.FadeOutScreen),
        "interrupt_active_flow_for_quest must emit FlowServerSkipNotify with FadeOutScreen=true"
    );

    // 4. First Echo GetSkill (200001) condition satisfied once echo 60000182 is in echo_map.
    let get_skill_cond: Value =
        serde_json::from_str(r#"{"Type":"GetSkill","SkillId":200001}"#).unwrap();
    {
        let qctx = h.state.quest_context(&mut events);
        let eval_ctx = qctx.make_eval_context();
        assert!(!eval_single_inline_condition(&eval_ctx, &get_skill_cond));
    }
    h.state.player_components.echo.echo_map.insert(
        1,
        EchoInfo {
            id: 60_000_182,
            level: 1,
            ..Default::default()
        },
    );
    {
        let qctx = h.state.quest_context(&mut events);
        let eval_ctx = qctx.make_eval_context();
        assert!(
            eval_single_inline_condition(&eval_ctx, &get_skill_cond),
            "echo 60000182 (SkillId 200001) must satisfy GetSkill 200001"
        );
    }
}

#[test]
fn audit_prologue_spawns_towers_chests_rover_and_boss_level1() {
    use sonoro_gamesv_logic::commands::entity as entity_cmd;
    use sonoro_gamesv_logic::component::entity::small::PositionComponent;
    use sonoro_gamesv_logic::handlers::scene as scene_logic;

    // 1. Fresh player join emits Battle group with Rover (1501) in FightRoleInfos & AoiData, no UI hide restriction.
    let mut h = TestHarness::new("audit_prologue_join_and_spawns", None);
    let mut events = EventQueue::default();
    h.state.player_components.basic.info.attributes.sex = 1;
    h.state.player_components.basic.info.attributes.cur_world_level = 1;
    sonoro_gamesv_logic::handlers::phase::notify_push_data_complete(
        &mut h.state.quest_context(&mut events),
    )
    .expect("notify_push_data_complete");
    scene_logic::on_initial_scene_join(
        Some(8),
        None,
        None,
        None,
        &mut events,
        &h.state.fs,
        h.state.player_id,
        &h.state.assets,
        &mut h.state.player_components.scene,
        &h.state.player_components.basic,
        &mut h.state.player_components.role,
        &mut h.state.player_components.weapon,
        &mut h.state.player_components.echo,
        &mut h.state.player_components.quest,
        &mut h.state.scene,
        &mut h.state.buff_timers,
        &mut h.state.dirty_saves,
    )
    .expect("on_initial_scene_join");
    {
        let mut sc = h.state.scene.take().expect("scene");
        let _ = h.drain_count();
        scene_logic::notify_join_scene(
            None,
            &mut events,
            &h.state.fs,
            &h.state.assets,
            &mut h.state.conn,
            h.state.player_id,
            &mut h.state.player_components.scene,
            &h.state.player_components.basic,
            &mut h.state.player_components.role,
            &mut h.state.player_components.weapon,
            &mut h.state.player_components.motor,
            &mut h.state.player_components.echo,
            &mut h.state.player_components.inventory,
            &mut h.state.player_components.map,
            &mut h.state.player_components.quest,
            &mut h.state.player_components.progress,
            &mut sc,
            &mut h.state.buff_timers,
        )
        .expect("notify_join_scene");
        h.state.scene = Some(sc);
    }
    let join_notifies: Vec<sonoro_proto::pb::JoinSceneNotify> =
        h.drain_decoded(sonoro_proto::pb::JoinSceneNotify::MSG_ID);
    assert_eq!(join_notifies.len(), 1);
    let scene_info = join_notifies[0].SceneInfo.as_ref().expect("SceneInfo");
    let pinfo = &scene_info.PlayerInfos[0];
    assert_eq!(pinfo.GroupType, 1);
    assert_eq!(pinfo.CurRole, 1501);
    let grp = &pinfo.FightRoleInfos[0];
    assert_eq!(grp.CurRole, 1501);
    assert!(
        grp.FightRoleInfos.iter().any(|r| r.RoleId == 1501),
        "Rover (1501) must be present in initial FightRoleInfos"
    );
    let rover_eid = grp
        .FightRoleInfos
        .iter()
        .find(|r| r.RoleId == 1501)
        .unwrap()
        .EntityId;
    assert!(
        scene_info
            .AoiData
            .as_ref()
            .unwrap()
            .Entities
            .iter()
            .any(|e| e.Id == rover_eid),
        "Rover entity must be present in JoinScene AoiData.Entities"
    );

    // 2. Boss Crownless (310000014) and prologue mob (310000587) keep explicit Level 1;
    //    unleveled world monster (11000229) still scales with Area/WorldLevel.
    entity_cmd::spawn_with_location(
        &mut events,
        h.state.scene.as_mut().unwrap(),
        &h.state.fs,
        &h.state.assets,
        &mut h.state.conn,
        &h.state.player_components.basic,
        310_000_014,
        Some(false),
        Some(false),
        Some(8),
        None,
        true,
    )
    .expect("spawn Crownless");
    let crownless_nid = h
        .state
        .scene
        .as_ref()
        .unwrap()
        .find_entity_net_id_by_config_id(310_000_014)
        .expect("Crownless spawned");
    entity_cmd::spawn_with_location(
        &mut events,
        h.state.scene.as_mut().unwrap(),
        &h.state.fs,
        &h.state.assets,
        &mut h.state.conn,
        &h.state.player_components.basic,
        310_000_587,
        Some(false),
        Some(false),
        Some(8),
        None,
        true,
    )
    .expect("spawn prologue mob");
    let prologue_mob_nid = h
        .state
        .scene
        .as_ref()
        .unwrap()
        .find_entity_net_id_by_config_id(310_000_587)
        .expect("prologue mob spawned");
    entity_cmd::spawn_with_location(
        &mut events,
        h.state.scene.as_mut().unwrap(),
        &h.state.fs,
        &h.state.assets,
        &mut h.state.conn,
        &h.state.player_components.basic,
        11_000_229,
        Some(false),
        Some(false),
        Some(8),
        None,
        true,
    )
    .expect("spawn overworld mob");
    let overworld_mob_nid = h
        .state
        .scene
        .as_ref()
        .unwrap()
        .find_entity_net_id_by_config_id(11_000_229)
        .expect("overworld mob spawned");

    {
        let sc = h.state.scene.as_ref().unwrap();
        let boss_lv = attributes::get_attr_value(
            sc.entity(crownless_nid).unwrap().attribute.as_ref().unwrap(),
            EAttributeType::Lv,
        );
        let pmob_lv = attributes::get_attr_value(
            sc.entity(prologue_mob_nid).unwrap().attribute.as_ref().unwrap(),
            EAttributeType::Lv,
        );
        let ow_lv = attributes::get_attr_value(
            sc.entity(overworld_mob_nid).unwrap().attribute.as_ref().unwrap(),
            EAttributeType::Lv,
        );
        assert_eq!(boss_lv, 1, "Prologue boss Crownless (310000014) must be Level 1");
        assert_eq!(pmob_lv, 1, "Prologue mob (310000587) must be Level 1");
        assert!(ow_lv >= 5, "Normal unleveled overworld monster (11000229) must still scale (got {ow_lv})");
    }

    // 3. Towers (Teleport007 122750076) and Chests (Treasure001 122001000 / 122000241)
    //    spawn and stay protected past >60m / >30m Z even when 240 closer ambient entities exist.
    let prologue_pos = [-139333.3f32, 66243.8, 7550.0];
    entity_cmd::spawn_with_location(
        &mut events,
        h.state.scene.as_mut().unwrap(),
        &h.state.fs,
        &h.state.assets,
        &mut h.state.conn,
        &h.state.player_components.basic,
        122_000_241,
        Some(false),
        Some(false),
        Some(8),
        None,
        true,
    )
    .expect("spawn chest 122000241");
    let existing_chest_nid = h
        .state
        .scene
        .as_ref()
        .unwrap()
        .find_entity_net_id_by_config_id(122_000_241)
        .expect("chest 122000241 spawned");
    {
        let sc = h.state.scene.as_mut().unwrap();
        let base_count = sc.entities.len();
        for i in base_count..240usize {
            let mut s = EntityComponentStorage::new(
                0,
                ConfigComponent {
                    camp: 2,
                    config_id: 11_000_229,
                    config_type: ConfigType::Level,
                    entity_type: EntityType::Monster,
                    state: EntityState::Default,
                },
            );
            s.position = Some(PositionComponent {
                location: [prologue_pos[0] + (i as f32) * 5.0, prologue_pos[1], prologue_pos[2]],
                rotation: [0.0; 3],
            });
            sc.spawn(s);
        }
        assert!(sc.entities.len() >= 240);
    }

    let mut sc = h.state.scene.take().unwrap();
    scene_logic::handle_spatial_spawns_check(
        prologue_pos,
        &mut events,
        &mut sc,
        Some(&mut h.state.player_components.scene),
        &h.state.fs,
        &h.state.assets,
        &mut h.state.conn,
        &mut h.state.player_components.basic,
        &mut h.state.player_components.inventory,
        &mut h.state.player_components.weapon,
        &mut h.state.player_components.echo,
        &mut h.state.player_components.map,
        &mut h.state.player_components.quest,
        Some(&mut h.state.player_components.guide),
        Some(&mut h.state.player_components.mail),
        &mut h.state.player_components.progress,
        &mut h.state.player_components.role,
        None,
    )
    .expect("handle_spatial_spawns_check");
    assert!(
        sc.entity(existing_chest_nid).is_some(),
        "Existing treasure chest (122000241) > 60m must not be evicted at 240-entity cap"
    );
    assert!(
        sc.find_entity_net_id_by_config_id(122_750_076).is_some(),
        "Nearby Resonance Beacon Teleport007 (122750076, ~99m, AoiLayer=0) must still spawn"
    );
    assert!(
        sc.find_entity_net_id_by_config_id(122_001_000).is_some(),
        "Nearby Treasure Chest (122001000, ~109m, +35.7m Z) must still spawn"
    );
    h.state.scene = Some(sc);
}

#[test]
fn audit_99_prologue_and_level_play_regressions() {
    use crate::network::handlers::{inst, profile};
    use sonoro_gamesv_data::asset_indexes::entity_world_pos;
    use sonoro_gamesv_fs::quest_info::ActiveQuest;
    use sonoro_gamesv_logic::helpers::condition::EvalContext;
    use sonoro_gamesv_logic::helpers::quest_engine;
    use sonoro_gamesv_logic::helpers::trial_roles;
    use sonoro_proto::enums::ErrorCode;
    use sonoro_proto::pb::{
        ChildQuestNodeProgressProgressOneof, EntityInteractRequest, ExchangeRewardInfoNotify,
        ExchangeRewardInfoRequest, LevelPlayInfoNotify, LevelPlayRewardRequest,
        PlayerTitleRequest, UpdateGroupFormationNotify,
    };

    let mut h = TestHarness::new("audit99", Some(8));
    h.drain_count();

    // 1) KillProgress.MonId starts empty (0/1 and 0/2) and only contains killed IDs.
    let q_nodes = h
        .state
        .assets
        .quest_nodes
        .get_owner_nodes(quest_engine::PROLOGUE_QUEST_ID)
        .expect("139000025 quest nodes")
        .to_vec();
    let node_kill1 = q_nodes.iter().find(|n| n.id == 6).expect("139000025 node 6");
    let node_kill2 = q_nodes.iter().find(|n| n.id == 99).expect("139000025 node 99");
    let node_reach8 = q_nodes.iter().find(|n| n.id == 8).expect("139000025 node 8");

    h.state
        .player_components
        .quest
        .info
        .active_quests
        .push(ActiveQuest {
            quest_id: quest_engine::PROLOGUE_QUEST_ID,
            active_nodes: vec![6, 99, 8],
            finished_nodes: vec![],
            killed_entities: vec![],
        });

    let mut events = EventQueue::new();
    {
        let qctx = h.state.quest_context(&mut events);
        let aq = &qctx.quest_comp.info.active_quests[0];
        let p1 = quest_engine::triggers::build_child_node_progress(&qctx, aq, node_kill1)
            .expect("node 6 progress");
        let Some(ChildQuestNodeProgressProgressOneof::Kill(kp1)) = p1.Progress else {
            panic!("expected KillProgress on node 6");
        };
        assert_eq!(kp1.CurrNum, 0);
        assert_eq!(kp1.TotalNum, 1);
        assert!(
            kp1.MonId.is_empty(),
            "MonId must be empty before enemy is killed (was {:?})",
            kp1.MonId
        );

        let p2 = quest_engine::triggers::build_child_node_progress(&qctx, aq, node_kill2)
            .expect("node 99 progress");
        let Some(ChildQuestNodeProgressProgressOneof::Kill(kp2)) = p2.Progress else {
            panic!("expected KillProgress on node 99");
        };
        assert_eq!(kp2.CurrNum, 0);
        assert_eq!(kp2.TotalNum, 2);
        assert!(kp2.MonId.is_empty());
    }

    h.state.player_components.quest.info.active_quests[0]
        .record_killed_entity(310_000_000);
    {
        let qctx = h.state.quest_context(&mut events);
        let aq = &qctx.quest_comp.info.active_quests[0];
        let p2_partial = quest_engine::triggers::build_child_node_progress(&qctx, aq, node_kill2)
            .expect("node 99 partial progress");
        let Some(ChildQuestNodeProgressProgressOneof::Kill(kp2_partial)) = p2_partial.Progress else {
            panic!("expected KillProgress");
        };
        assert_eq!(kp2_partial.CurrNum, 1);
        assert_eq!(kp2_partial.TotalNum, 2);
        assert_eq!(kp2_partial.MonId, vec![310_000_000]);
    }

    // 2) ReachArea node 8 must use Condition.Pos (-128737.73, 62616.8, 10176.38, r=4000)
    // and NOT fall back to TrackTarget.Locations (-131224, 62578, 10171).
    {
        let cond_8 = node_reach8
            .data
            .get("Condition")
            .and_then(|v| v.as_object())
            .expect("node 8 Condition");
        let mut qctx = h.state.quest_context(&mut events);
        assert!(
            !quest_engine::matches_reach_area_pub(
                &mut qctx,
                quest_engine::PROLOGUE_QUEST_ID,
                &node_reach8.data,
                cond_8,
                [-133_620.0, 62_578.0, 10_171.0],
            ),
            "ReachArea must not fall back to TrackTarget.Locations when Condition.Pos is present"
        );
        assert!(
            quest_engine::matches_reach_area_pub(
                &mut qctx,
                quest_engine::PROLOGUE_QUEST_ID,
                &node_reach8.data,
                cond_8,
                [-128_737.73, 62_616.8, 10_176.38],
            ),
            "ReachArea must trigger inside Condition.Pos range"
        );
    }

    // 3) Yangyang (1402) joins Formation Slot 2 after Node 9 (on Node 155 / 10) even without
    // pre-existing RoleInfo or equipped weapon.
    assert_eq!(trial_roles::PROLOGUE_YANGYANG_ROLE_ID, 1402);
    h.state
        .player_components
        .role
        .role_map
        .remove(&trial_roles::PROLOGUE_YANGYANG_ROLE_ID);
    assert!(!trial_roles::should_prologue_have_yangyang(
        &h.state.player_components.quest
    ));
    h.state.player_components.quest.info.active_quests[0]
        .active_nodes
        .push(155);
    assert!(trial_roles::should_prologue_have_yangyang(
        &h.state.player_components.quest
    ));
    h.drain_count();
    {
        let mut sc = h.state.scene.take().unwrap();
        let rc = &mut h.state.player_components.role;
        let wc = &mut h.state.player_components.weapon;
        let ec = &h.state.player_components.echo;
        let qc = &h.state.player_components.quest;
        trial_roles::sync_prologue_yangyang_live(
            &mut sc,
            &h.state.fs,
            &h.state.assets,
            &mut h.state.conn,
            &mut events,
            rc,
            wc,
            ec,
            qc,
        )
        .expect("sync_prologue_yangyang_live");
        h.state.scene = Some(sc);
    }
    assert!(
        h.state
            .player_components
            .role
            .role_map
            .get(&1402)
            .is_some_and(|r| r.weapon > 0),
        "sync_prologue_yangyang_live must auto-provision Yangyang 1402 + her weapon"
    );
    let formation_roles: Vec<i32> = h
        .state
        .scene
        .as_ref()
        .unwrap()
        .formation_info
        .formations
        .first()
        .map(|f| f.roles.iter().flatten().map(|r| r.role_id).collect())
        .unwrap_or_default();
    assert!(
        formation_roles.contains(&trial_roles::PROLOGUE_YANGYANG_ROLE_ID),
        "Yangyang 1402 must join active formation: {:?}",
        formation_roles
    );
    let form_notifs: Vec<UpdateGroupFormationNotify> =
        h.drain_decoded(UpdateGroupFormationNotify::MSG_ID);
    assert!(
        form_notifs.iter().any(|n| n.GroupFormation.iter().any(|gf| {
            gf.FightRoleInfos.iter().any(|group| {
                group
                    .FightRoleInfos
                    .iter()
                    .any(|fr| fr.RoleId == trial_roles::PROLOGUE_YANGYANG_ROLE_ID)
            })
        })),
        "UpdateGroupFormationNotify must include Yangyang 1402"
    );

    // Login repair must also re-add Yangyang if missing from formation while on 155/10..131.
    if let Some(f0) = h
        .state
        .scene
        .as_mut()
        .unwrap()
        .formation_info
        .formations
        .first_mut()
    {
        for slot in &mut f0.roles {
            if slot
                .as_ref()
                .is_some_and(|r| r.role_id == trial_roles::PROLOGUE_YANGYANG_ROLE_ID)
            {
                *slot = None;
            }
        }
    }
    h.state
        .player_components
        .role
        .role_map
        .remove(&trial_roles::PROLOGUE_YANGYANG_ROLE_ID);
    {
        let mut sc = h.state.scene.take().unwrap();
        trial_roles::ensure_prologue_yangyang_in_formation_login(
            &h.state.fs,
            &h.state.assets,
            &mut sc.formation_info,
            &mut h.state.player_components.role,
            &mut h.state.player_components.weapon,
            &h.state.player_components.quest,
        );
        let repaired_roles: Vec<i32> = sc
            .formation_info
            .formations
            .first()
            .map(|f| f.roles.iter().flatten().map(|r| r.role_id).collect())
            .unwrap_or_default();
        assert!(repaired_roles.contains(&1402));
        h.state.scene = Some(sc);
    }

    // 3b) Unset world weather (env.weather_id == 0) must NOT be forced on join before a quest trigger fires;
    // explicit weather already set on quest state (e.g. WeatherId=3) must still sync on join.
    h.state.player_components.quest.info.env.weather_id = 0;
    h.drain_count();
    {
        let mut qctx = h.state.quest_context(&mut events);
        sonoro_gamesv_logic::helpers::quest_actions::sync_world_state_on_join(&mut qctx)
            .expect("sync_world_state_on_join");
    }
    assert_eq!(h.state.player_components.quest.info.env.weather_id, 0);
    let weather_notifs_unset: Vec<sonoro_proto::pb::WeatherNotify> =
        h.drain_decoded(sonoro_proto::pb::WeatherNotify::MSG_ID);
    assert!(
        weather_notifs_unset.is_empty(),
        "sync_world_state_on_join must not prematurely push WeatherNotify when env.weather_id is unset"
    );

    h.state.player_components.quest.info.env.weather_id = 3;
    {
        let mut qctx = h.state.quest_context(&mut events);
        sonoro_gamesv_logic::helpers::quest_actions::sync_world_state_on_join(&mut qctx)
            .expect("sync_world_state_on_join explicit");
    }
    assert_eq!(h.state.player_components.quest.info.env.weather_id, 3);
    let weather_notifs: Vec<sonoro_proto::pb::WeatherNotify> =
        h.drain_decoded(sonoro_proto::pb::WeatherNotify::MSG_ID);
    assert!(
        weather_notifs.iter().any(|w| w.WeatherId == 3),
        "sync_world_state_on_join must sync explicit WeatherId=3 when already set"
    );
    h.state.player_components.quest.info.env.weather_id = 0;

    // 4) LevelPlay 110000000 (RebornBoss), its map marker 5021, and arena entities (110000595, 110001071)
    // must stay hidden/blocked during unfinished 139000025 or ExploreLevel < 10.
    h.state.player_components.basic.info.attributes.level = 1;
    {
        let lp_init = misc_logic::build_level_play_info_notify(
            &h.state.assets,
            &h.state.player_components.basic,
            &h.state.player_components.quest,
        );
        assert!(
            !lp_init.LevelPlayInfo.iter().any(|lp| lp.Id == 110_000_000),
            "110000000 must not be pushed at login during prologue"
        );
        let mm_init = misc_logic::build_map_mark_info_notify(
            &h.state.assets,
            &h.state.player_components.map,
            &h.state.player_components.basic,
            &h.state.player_components.quest,
        );
        assert!(
            !mm_init.ShowMarkIds.iter().any(|m| m.MarkId == 5021)
                && !mm_init.UnlockMarkIds.contains(&5021),
            "Boss MarkId 5021 must be hidden and locked at game start"
        );
        let eval_ctx = h.state.eval_context();
        assert!(
            misc_logic::is_entity_blocked_by_locked_level_play(&eval_ctx, 8, 110_000_595)
                && misc_logic::is_entity_blocked_by_locked_level_play(&eval_ctx, 8, 110_001_071),
            "Challenge chord 110000595 and Tacet Field atmosphere 110001071 must be hidden while 110000000 is locked"
        );
    }
    let open_blocked = inst::handle_level_play_open_request(
        &mut h.state,
        sonoro_proto::pb::LevelPlayOpenRequest {
            LevelPlayId: 110_000_000,
            ..Default::default()
        },
    )
    .expect("lp open blocked");
    assert_ne!(open_blocked.ErrorCode, Some(ErrorCode::Success));
    let interact_blocked = scene::handle_entity_interact_request(
        &mut h.state,
        &mut events,
        EntityInteractRequest {
            EntityId: 110_000_595,
            OptionIndex: 0,
            ..Default::default()
        },
    )
    .expect("interact blocked");
    assert_ne!(interact_blocked.ErrorCode, Some(ErrorCode::Success));

    let lp_anchor_pos = entity_world_pos(
        &h.state
            .assets
            .find_level_entity(8, 129_800_000)
            .expect("entity 129800000"),
    )
    .expect("129800000 world pos");
    h.state.player_components.basic.info.attributes.level = 10;
    {
        let mut qctx = h.state.quest_context(&mut events);
        let mut lp_ctx = level_play::LevelPlayContext::from_quest_ctx(&mut qctx).unwrap();
        level_play::on_player_move(
            &mut lp_ctx,
            lp_anchor_pos[0],
            lp_anchor_pos[1],
            lp_anchor_pos[2],
        )
        .expect("lp move 1");
    }
    assert!(
        !h.state
            .player_components
            .quest
            .info
            .progress
            .entered_level_plays
            .contains(&110_000_000),
        "RebornBoss LevelPlay 110000000 must stay blocked while prologue 139000025 is active"
    );

    // Stale entered 110000000 during unfinished prologue must be pruned on login sync.
    append_unique_int(
        &mut h.state.player_components.quest.info.progress.entered_level_plays,
        110_000_000,
    );
    {
        let mut qctx = h.state.quest_context(&mut events);
        let mut lp_ctx = level_play::LevelPlayContext::from_quest_ctx(&mut qctx).unwrap();
        level_play::sync_on_login(&mut lp_ctx).expect("lp sync_on_login");
    }
    assert!(
        !h.state
            .player_components
            .quest
            .info
            .progress
            .entered_level_plays
            .contains(&110_000_000),
        "sync_on_login must prune stale RebornBoss entry during prologue"
    );

    // Finish prologue 139000025, but keep ExploreLevel = 1 (< 10): still blocked by OpenCondition.
    h.state.player_components.quest.info.active_quests.clear();
    append_unique_int(
        &mut h.state.player_components.quest.info.finished_quests,
        quest_engine::PROLOGUE_QUEST_ID,
    );
    h.state.player_components.basic.info.attributes.level = 1;
    {
        let mut qctx = h.state.quest_context(&mut events);
        let mut lp_ctx = level_play::LevelPlayContext::from_quest_ctx(&mut qctx).unwrap();
        level_play::on_player_move(
            &mut lp_ctx,
            lp_anchor_pos[0],
            lp_anchor_pos[1],
            lp_anchor_pos[2],
        )
        .expect("lp move lv1");
    }
    assert!(
        !h.state
            .player_components
            .quest
            .info
            .progress
            .entered_level_plays
            .contains(&110_000_000),
        "LevelPlay 110000000 must stay blocked when ExploreLevel < 10"
    );

    // With prologue finished AND ExploreLevel >= 10, 5021/110000000/110000595 unlock and entering 129800000 activates 110000000.
    h.state.player_components.basic.info.attributes.level = 10;
    {
        let lp_unlocked = misc_logic::build_level_play_info_notify(
            &h.state.assets,
            &h.state.player_components.basic,
            &h.state.player_components.quest,
        );
        assert!(lp_unlocked.LevelPlayInfo.iter().any(|lp| lp.Id == 110_000_000));
        let mm_unlocked = misc_logic::build_map_mark_info_notify(
            &h.state.assets,
            &h.state.player_components.map,
            &h.state.player_components.basic,
            &h.state.player_components.quest,
        );
        assert!(
            mm_unlocked.ShowMarkIds.iter().any(|m| m.MarkId == 5021)
                && mm_unlocked.UnlockMarkIds.contains(&5021),
            "Boss MarkId 5021 must appear once prologue finishes and ExploreLevel >= 10"
        );
        let eval_ctx = h.state.eval_context();
        assert!(!misc_logic::is_entity_blocked_by_locked_level_play(&eval_ctx, 8, 110_000_595));
    }
    {
        let mut qctx = h.state.quest_context(&mut events);
        let mut lp_ctx = level_play::LevelPlayContext::from_quest_ctx(&mut qctx).unwrap();
        level_play::on_player_move(
            &mut lp_ctx,
            lp_anchor_pos[0],
            lp_anchor_pos[1],
            lp_anchor_pos[2],
        )
        .expect("lp move lv10");
    }
    assert!(
        h.state
            .player_components
            .quest
            .info
            .progress
            .entered_level_plays
            .contains(&110_000_000),
        "LevelPlay 110000000 should activate once prologue is done and ExploreLevel >= 10"
    );

    // 5) CompareLevelPlayRewardState (no State field), LevelPlayRewardRequest, ExchangeRewardInfoRequest, ResetLevelPlay.
    append_unique_int(
        &mut h.state.player_components.quest.info.completed_level_plays,
        110_000_000,
    );
    h.state
        .player_components
        .progress
        .info
        .set_key_int("lp_state:110000000", 3);
    let cond_reward_ready = serde_json::json!({
        "Type": "CompareLevelPlayRewardState",
        "EntityId": 110_000_089,
        "Compare": "Ne"
    });
    let cond_reward_not_ready = serde_json::json!({
        "Type": "CompareLevelPlayRewardState",
        "EntityId": 110_000_089
    });
    {
        let mut ectx = EvalContext::new(
            &h.state.player_components.basic,
            &h.state.player_components.echo,
            &h.state.player_components.inventory,
            &h.state.player_components.map,
            &h.state.player_components.quest,
        );
        ectx.assets = Some(&h.state.assets);
        ectx.progress_comp = Some(&h.state.player_components.progress);
        assert!(
            eval_single_inline_condition(&ectx, &cond_reward_ready),
            "CompareLevelPlayRewardState (Ne, no State) must be true when completed and unclaimed"
        );
        assert!(
            !eval_single_inline_condition(&ectx, &cond_reward_not_ready),
            "CompareLevelPlayRewardState (Eq default, no State) must be false while CanGetReward is true"
        );
    }

    h.drain_count();
    let reward_resp = inst::handle_level_play_reward_request(
        &mut h.state,
        LevelPlayRewardRequest {
            EntityId: 110_000_089,
            ExchangeCount: 1,
        },
    )
    .expect("handle_level_play_reward_request");
    assert_eq!(reward_resp.ErrorCode, Some(ErrorCode::Success));
    assert!(
        h.state
            .player_components
            .quest
            .info
            .opened_chests
            .contains(&110_000_089)
    );
    use crate::network::message::Message;
    use sonoro_proto::PbMessage;
    let mut lp_notifs = Vec::new();
    let mut ex_notifs = Vec::new();
    while let Ok(frame) = h.rx.try_recv() {
        if frame.len() < 3 {
            continue;
        }
        if let Ok(msg) = Message::decode(&frame[3..]) {
            if msg.header.message_id() == LevelPlayInfoNotify::MSG_ID
                && let Ok(n) = LevelPlayInfoNotify::decode(&msg.payload)
            {
                lp_notifs.push(n);
            } else if msg.header.message_id() == ExchangeRewardInfoNotify::MSG_ID
                && let Ok(n) = ExchangeRewardInfoNotify::decode(&msg.payload)
            {
                ex_notifs.push(n);
            }
        }
    }
    assert!(
        lp_notifs
            .iter()
            .any(|n| n.LevelPlayInfo.iter().any(|i| i.Id == 110_000_000 && i.GetRewardCount == 1))
    );
    assert!(
        ex_notifs
            .iter()
            .any(|n| n.ExchangeRewardId > 0 && n.Count >= 1)
    );

    {
        let mut ectx = EvalContext::new(
            &h.state.player_components.basic,
            &h.state.player_components.echo,
            &h.state.player_components.inventory,
            &h.state.player_components.map,
            &h.state.player_components.quest,
        );
        ectx.assets = Some(&h.state.assets);
        ectx.progress_comp = Some(&h.state.player_components.progress);
        assert!(
            !eval_single_inline_condition(&ectx, &cond_reward_ready),
            "CompareLevelPlayRewardState (Ne) must become false after claiming"
        );
        assert!(
            eval_single_inline_condition(&ectx, &cond_reward_not_ready),
            "CompareLevelPlayRewardState (Eq default) must become true after claiming"
        );
    }

    let ex_info = inst::handle_exchange_reward_info_request(
        &mut h.state,
        ExchangeRewardInfoRequest::default(),
    )
    .expect("handle_exchange_reward_info_request");
    assert_eq!(ex_info.ExchangeRewardDict.get(&110_000_089).copied(), Some(1));

    // ResetLevelPlay clears claim, opened chest, and pushes State: 1.
    h.drain_count();
    {
        let mut qctx = h.state.quest_context(&mut events);
        let reset_act = serde_json::json!({
            "Name": "ResetLevelPlay",
            "Params": { "ResetLevelPlayList": [110_000_000] }
        });
        execute_action(&mut qctx, &mut ActionCallbacks::default(), 0, 0, &reset_act)
            .expect("ResetLevelPlay");
    }
    assert!(
        !h.state
            .player_components
            .quest
            .info
            .opened_chests
            .contains(&110_000_089)
    );
    assert_eq!(
        h.state
            .player_components
            .progress
            .info
            .get_key_int("lp_reward_claimed:110000000"),
        0
    );
    let reset_notifs: Vec<LevelPlayInfoNotify> = h.drain_decoded(LevelPlayInfoNotify::MSG_ID);
    assert!(
        reset_notifs
            .iter()
            .any(|n| n.LevelPlayInfo.iter().any(|i| i.Id == 110_000_000 && i.State == 1))
    );

    // 6) PlayerTitleRequest returns unlocked title list.
    let title_resp = profile::handle_player_title_request(
        &mut h.state,
        PlayerTitleRequest::default(),
    )
    .expect("handle_player_title_request");
    assert_eq!(title_resp.ErrorCode, Some(ErrorCode::Success));
    assert!(!title_resp.PlayerTitleInfo.is_empty());
}

#[test]
fn audit_142_boss_hp_does_not_refill_on_lifemax_buff_or_fsm_reset() {
    use sonoro_gamesv_logic::events::{BuffAdditionEntry, FsmServerActionKind};
    use sonoro_gamesv_logic::helpers::attributes::{
        self, AttributeChangeType, AttributeValueType,
    };

    // 1) Direct LifeMax delta must NOT scale current HP upward, and only clamps down when LifeMax shrinks below Life.
    let mut direct_props = vec![0i32; 144];
    direct_props[EAttributeType::LifeMax.to_i32() as usize] = 100_000;
    direct_props[EAttributeType::Life.to_i32() as usize] = 40_000;
    let mut direct_attr = AttributeComponent::create(&direct_props);

    let changed_up = attributes::change_attr(
        &mut direct_attr,
        EAttributeType::LifeMax,
        AttributeChangeType::Delta,
        AttributeValueType::Current,
        100_000,
    );
    assert!(changed_up.contains(&EAttributeType::LifeMax));
    assert!(!changed_up.contains(&EAttributeType::Life));
    assert_eq!(
        attributes::get_attr_value(&direct_attr, EAttributeType::LifeMax),
        200_000
    );
    assert_eq!(
        attributes::get_attr_value(&direct_attr, EAttributeType::Life),
        40_000,
        "increasing LifeMax must not scale current Life upward"
    );

    // Raise Life to 120_000 (healed while buffed), then shrink LifeMax back to 100_000 -> Life clamps down to 100_000 and is reported.
    attributes::change_attr(
        &mut direct_attr,
        EAttributeType::Life,
        AttributeChangeType::Override,
        AttributeValueType::Current,
        120_000,
    );
    let changed_down = attributes::change_attr(
        &mut direct_attr,
        EAttributeType::LifeMax,
        AttributeChangeType::Delta,
        AttributeValueType::Current,
        -100_000,
    );
    assert!(changed_down.contains(&EAttributeType::LifeMax));
    assert!(changed_down.contains(&EAttributeType::Life));
    assert_eq!(
        attributes::get_attr_value(&direct_attr, EAttributeType::Life),
        100_000
    );

    // 2) Live scene boss entity: Crownless Phase-3 +100% LifeMax buff (80004034) and FSM ResetStatus must NOT refill HP.
    let mut h = TestHarness::new("audit_142_boss_hp", Some(8));
    let mut events = EventQueue::default();

    let boss_nid = {
        let graph = h
            .state
            .assets
            .fsm_graphs
            .get("SM_MB1DaniyaMd00101")
            .expect("SM_MB1DaniyaMd00101 graph");
        let mut s = EntityComponentStorage::new(
            0,
            ConfigComponent {
                camp: 2,
                config_id: 900_142,
                config_type: ConfigType::Level,
                entity_type: EntityType::Monster,
                state: EntityState::Default,
            },
        );
        let mut props = vec![0i32; 144];
        props[EAttributeType::LifeMax.to_i32() as usize] = 100_000;
        props[EAttributeType::Life.to_i32() as usize] = 35_000;
        props[EAttributeType::ToughMax.to_i32() as usize] = 1_000;
        props[EAttributeType::Tough.to_i32() as usize] = 1_000;
        let mut attr = AttributeComponent::create(&props);
        attributes::change_attr(
            &mut attr,
            EAttributeType::Tough,
            AttributeChangeType::Override,
            AttributeValueType::Current,
            200,
        );
        s.attribute = Some(attr);
        s.buffs = Some(FightBuffComponent::default());
        s.tag = Some(TagComponent::default());
        s.logic_state = Some(LogicStateComponent::default());
        s.part = Some(PartComponent::default());
        s.fsm = Some(FsmComponent::from_graph(graph, 0).expect("fsm graph"));
        h.state.scene.as_mut().unwrap().spawn(s).net_id
    };
    let boss_handle = h
        .state
        .scene
        .as_ref()
        .unwrap()
        .entity_handle(boss_nid)
        .expect("boss handle");

    // Apply Buff 80004034 (+100% LifeMax) via event queue.
    events.enqueue(Event::BuffAddition {
        target: boss_handle,
        instigator: boss_handle,
        buffs: vec![BuffAdditionEntry {
            id: 80_004_034,
            is_active: true,
            duration_seconds: Some(10.0),
            ..Default::default()
        }],
    });
    crate::network::connection::drain_event_queue_best_effort(&mut events, &mut h.state);
    h.drain_count();

    {
        let sc = h.state.scene.as_ref().unwrap();
        let attr = sc.entity(boss_nid).unwrap().attribute.as_ref().unwrap();
        assert_eq!(attributes::get_attr_value(attr, EAttributeType::LifeMax), 200_000);
        assert_eq!(
            attributes::get_attr_value(attr, EAttributeType::Life),
            35_000,
            "Buff 80004034 (+100% LifeMax) must leave boss current Life unchanged"
        );
    }

    // Trigger FSM ResetStatus: must clear Tough back to 1000 and remove buffs without refilling Life to 100_000.
    events.enqueue(Event::FsmServerAction {
        entity: boss_handle,
        kind: FsmServerActionKind::ResetStatus,
    });
    crate::network::connection::drain_event_queue_best_effort(&mut events, &mut h.state);
    h.drain_count();

    {
        let sc = h.state.scene.as_ref().unwrap();
        let attr = sc.entity(boss_nid).unwrap().attribute.as_ref().unwrap();
        assert_eq!(
            attributes::get_attr_value(attr, EAttributeType::Life),
            35_000,
            "FsmServerActionKind::ResetStatus must not refill boss Life"
        );
        assert_eq!(
            attributes::get_attr_value(attr, EAttributeType::Tough),
            1_000
        );
    }
}

#[test]
fn audit_143_weather_not_forced_prematurely_and_deferred_past_flow_or_wait() {
    let mut h = TestHarness::new("audit_143_weather", Some(8));
    let mut events = EventQueue::default();

    // 1) World join + entering 139000025 Node 19 must NOT force WeatherId=1 when env.weather_id is unset.
    h.state.player_components.quest.info.env.weather_id = 0;
    {
        let mut qctx = h.state.quest_context(&mut events);
        sonoro_gamesv_logic::helpers::quest_actions::sync_world_state_on_join(&mut qctx)
            .expect("sync_world_state_on_join");
    }
    assert_eq!(h.state.player_components.quest.info.env.weather_id, 0);
    assert!(
        h.drain_decoded::<sonoro_proto::pb::WeatherNotify>(sonoro_proto::pb::WeatherNotify::MSG_ID)
            .is_empty()
    );

    quest_engine::tree::activate_quest(&mut h.state.quest_context(&mut events), 139_000_025, true)
        .expect("activate prologue");
    assert_eq!(
        h.state.player_components.quest.info.env.weather_id, 0,
        "entering prologue node 19 must not force weather before quest SetWeather trigger"
    );
    assert!(
        h.drain_decoded::<sonoro_proto::pb::WeatherNotify>(sonoro_proto::pb::WeatherNotify::MSG_ID)
            .is_empty()
    );
    let _ = events.drain_all().count();

    // 2) Normal quest SetWeather emits WeatherNotify cleanly WITHOUT injecting tweentime=0 JSPatchNotify snap.
    let mut cbs = quest_engine::action_callbacks();
    let set_w2 = serde_json::json!({
        "Name": "SetWeather",
        "Params": { "WeatherId": 2 }
    });
    {
        let mut qctx = h.state.quest_context(&mut events);
        sonoro_gamesv_logic::helpers::quest_actions::execute_action(
            &mut qctx,
            &mut cbs,
            139_000_025,
            35,
            &set_w2,
        )
        .expect("apply weather 2");
    }
    assert_eq!(h.state.player_components.quest.info.env.weather_id, 2);
    let w2_notifs: Vec<sonoro_proto::pb::WeatherNotify> =
        h.drain_decoded(sonoro_proto::pb::WeatherNotify::MSG_ID);
    assert_eq!(w2_notifs.len(), 1);
    assert_eq!(w2_notifs[0].WeatherId, 2);
    let js_notifs: Vec<sonoro_proto::pb::JSPatchNotify> =
        h.drain_decoded(sonoro_proto::pb::JSPatchNotify::MSG_ID);
    assert!(
        !js_notifs.iter().any(|p| String::from_utf8_lossy(&p.Content).contains("ChangeCurrentWeather")),
        "quest weather change must not inject instant-snap ChangeCurrentWeather JS patch"
    );
    let _ = events.drain_all().count();

    // 3) In an action list with sync PlayFlow followed by SetWeather, SetWeather must NOT fire until that Flow ends
    // (including across multi-state next_state_id transitions).
    if let Some(sc) = h.state.scene.as_mut() {
        sc.clear_active_flow();
        sc.clear_pending_flows();
    }
    let flow_then_weather = vec![
        serde_json::json!({
            "Name": "PlayFlow",
            "Params": { "FlowListName": "剧情_新主线_CE版_序章", "FlowId": 1, "StateId": 18 }
        }),
        serde_json::json!({
            "Name": "SetWeather",
            "Params": { "WeatherId": 3 }
        }),
    ];
    {
        let mut qctx = h.state.quest_context(&mut events);
        sonoro_gamesv_logic::helpers::quest_actions::execute_action_list(
            &mut qctx,
            &mut cbs,
            189_000_005,
            18,
            &flow_then_weather,
        )
        .expect("execute PlayFlow + SetWeather");
    }
    let started_inc = h.state.scene.as_ref().unwrap().active_flow_inc_id;
    assert_ne!(started_inc, 0);
    assert_eq!(
        h.state.player_components.quest.info.env.weather_id, 2,
        "SetWeather after sync PlayFlow must NOT apply before the flow ends"
    );
    h.drain_count();

    // Ending flow transitions through any intermediate FlowStates (or completes immediately if terminal).
    let mut safety = 0usize;
    loop {
        let cur_inc = h.state.scene.as_ref().unwrap().active_flow_inc_id;
        if cur_inc == 0 {
            break;
        }
        assert!(safety < 16, "flow chain exceeded expected length");
        safety += 1;
        let _ = crate::network::handlers::flow::handle_flow_end_request(
            &mut h.state,
            &mut events,
            sonoro_proto::pb::FlowEndRequest {
                FlowIncId: cur_inc,
                OptionInfos: BTreeMap::new(),
                IsSkip: true,
                ..Default::default()
            },
        )
        .expect("end flow");
        let _ = events.drain_all().count();
    }
    assert_eq!(
        h.state.player_components.quest.info.env.weather_id, 3,
        "deferred SetWeather must apply once the preceding PlayFlow finishes"
    );
    let w3_notifs: Vec<sonoro_proto::pb::WeatherNotify> =
        h.drain_decoded(sonoro_proto::pb::WeatherNotify::MSG_ID);
    assert!(w3_notifs.iter().any(|w| w.WeatherId == 3));

    // 4) Action list with Wait(0.25s) followed by SetWeather(4) must defer until timer fires.
    let wait_then_weather = vec![
        serde_json::json!({
            "Name": "Wait",
            "Params": { "Time": 0.25 }
        }),
        serde_json::json!({
            "Name": "SetWeather",
            "Params": { "WeatherId": 4 }
        }),
    ];
    {
        let mut qctx = h.state.quest_context(&mut events);
        sonoro_gamesv_logic::helpers::quest_actions::execute_action_list(
            &mut qctx,
            &mut cbs,
            151_000_024,
            82,
            &wait_then_weather,
        )
        .expect("execute Wait + SetWeather");
    }
    assert_eq!(
        h.state.player_components.quest.info.env.weather_id, 3,
        "SetWeather after Wait must not apply immediately"
    );
    {
        let future_ms = sonoro_gamesv_logic::scene::now_millis() + 1_000;
        let mut qctx = h.state.quest_context(&mut events);
        quest_engine::tree::on_timers_due(&mut qctx, future_ms).expect("on_timers_due");
    }
    assert_eq!(h.state.player_components.quest.info.env.weather_id, 4);
}

#[test]
fn audit_144_explore_teleporter_index_and_udp_kcp_burst_reconnect() {
    use sonoro_proto::PbMessage;
    use std::time::Instant;

    let mut h = TestHarness::new("audit_144_net", Some(8));
    let mut events = EventQueue::default();

    // 1) teleporters_by_area matches full teleporter scan across all 524 areas and runs fast (< 50ms even unoptimized debug).
    let all_areas: Vec<i32> = h.state.assets.tables.area.items.iter().map(|a| a.AreaId).collect();
    let mut area_tp_counts_expected = std::collections::HashMap::<i32, usize>::new();
    for tp in &h.state.assets.tables.teleporter.items {
        if let Some(lcfg) = h.state.assets.find_level_entity(tp.MapId, tp.TeleportEntityConfigId)
            && lcfg.AreaId > 0
        {
            *area_tp_counts_expected.entry(lcfg.AreaId).or_insert(0) += 1;
        }
    }
    for &aid in &all_areas {
        let exp = area_tp_counts_expected.get(&aid).copied().unwrap_or(0);
        assert_eq!(
            h.state.assets.indexes.teleporter_indices_for_area(aid).len(),
            exp,
            "teleporters_by_area count mismatch for area {aid}"
        );
    }
    let t_exp = Instant::now();
    let exp_resp = map::handle_explore_progress_request(
        &mut h.state,
        sonoro_proto::pb::ExploreProgressRequest { AreaIds: all_areas },
    )
    .expect("explore progress");
    assert!(!exp_resp.AreaProgress.is_empty());
    assert!(
        t_exp.elapsed() < std::time::Duration::from_millis(50),
        "ExploreProgressRequest took {:?}, expected < 50ms",
        t_exp.elapsed()
    );

    // 2) LazyNodeStore cached owner lookup parity.
    assert!(h.state.assets.quest_nodes.has_owner(139_000_025));
    assert!(!h.state.assets.quest_nodes.has_owner(-999_999));

    // 3) Real UDP bind_and_serve handles concurrent/unacked client traffic + immediate KRKCP_SYN reconnect.
    let _ = events.drain_all().count();
    let probe = std::net::UdpSocket::bind("127.0.0.1:0").expect("bind probe");
    let bind_addr = probe.local_addr().expect("probe addr");
    drop(probe);

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(2)
        .build()
        .expect("tokio rt");
    let assets = h.state.assets.clone();
    let fs = h.state.fs.clone();
    let sm = std::sync::Arc::new(crate::network::SessionManager::new());
    let (tx_shut, rx_shut) = tokio::sync::watch::channel(false);
    let srv_handle = rt.spawn(crate::network::bind_and_serve(
        bind_addr, fs, assets, sm, rx_shut,
    ));

    let kcp_req_frame = |seq: u32, rpc: u16, msg_id: u16, payload: &[u8]| -> Vec<u8> {
        let mut hasher = crc32fast::Hasher::new();
        hasher.update(payload);
        let crc = hasher.finalize();
        let mut app = Vec::with_capacity(13 + payload.len());
        app.push(1);
        app.extend_from_slice(&seq.to_le_bytes());
        app.extend_from_slice(&rpc.to_le_bytes());
        app.extend_from_slice(&msg_id.to_le_bytes());
        app.extend_from_slice(&crc.to_le_bytes());
        app.extend_from_slice(payload);
        let len_b = (app.len() as u32).to_le_bytes();
        let mut frame = vec![len_b[0], len_b[1], len_b[2]];
        frame.extend_from_slice(&app);
        frame
    };

    rt.block_on(async {
        tokio::time::sleep(std::time::Duration::from_millis(40)).await;
        let mut syn = [0u8; 35];
        syn[0] = 0xED;
        let mut buf = [0u8; 4096];

        let cli1 = tokio::net::UdpSocket::bind("127.0.0.1:0").await.unwrap();
        cli1.connect(bind_addr).await.unwrap();
        cli1.send(&syn).await.unwrap();
        let n1 = tokio::time::timeout(std::time::Duration::from_secs(2), cli1.recv(&mut buf))
            .await
            .expect("cli1 syn ack timeout")
            .unwrap();
        assert_eq!(n1, 5);
        assert_eq!(buf[0], 0xEE);
        let conv1 = u32::from_le_bytes(buf[1..5].try_into().unwrap());

        let mut kcp1 = crate::network::kcp::Kcp::new(conv1);
        kcp1.set_mtu(1400).unwrap();
        kcp1.set_nodelay(1, 10, 2, true);
        kcp1.set_wnd_size(512, 512);
        let hb_bytes = sonoro_proto::pb::HeartbeatRequest::default().encode_to_vec();
        // Send a burst of un-ACKed requests and drop cli1 without draining responses.
        for i in 1..=32u32 {
            kcp1.send(&kcp_req_frame(
                i,
                i as u16,
                sonoro_proto::pb::HeartbeatRequest::MSG_ID,
                &hb_bytes,
            ))
            .unwrap();
        }
        let mut pkts = Vec::new();
        kcp1.update(10, &mut |p| pkts.push(p.to_vec()));
        for p in pkts.drain(..) {
            let _ = cli1.send(&p).await;
        }
        drop(cli1);

        // Immediately reconnect with cli2 -> KRKCP_ACK and HeartbeatResponse must arrive promptly.
        let cli2 = tokio::net::UdpSocket::bind("127.0.0.1:0").await.unwrap();
        cli2.connect(bind_addr).await.unwrap();
        let tsyn2 = Instant::now();
        cli2.send(&syn).await.unwrap();
        let n2 = tokio::time::timeout(std::time::Duration::from_secs(2), cli2.recv(&mut buf))
            .await
            .expect("cli2 syn ack timeout")
            .unwrap();
        assert_eq!(n2, 5);
        assert_eq!(buf[0], 0xEE);
        assert!(tsyn2.elapsed() < std::time::Duration::from_secs(1));
        let conv2 = u32::from_le_bytes(buf[1..5].try_into().unwrap());

        let mut kcp2 = crate::network::kcp::Kcp::new(conv2);
        kcp2.set_mtu(1400).unwrap();
        kcp2.set_nodelay(1, 10, 2, true);
        kcp2.set_wnd_size(512, 512);
        kcp2.send(&kcp_req_frame(
            1,
            1,
            sonoro_proto::pb::HeartbeatRequest::MSG_ID,
            &hb_bytes,
        ))
        .unwrap();
        kcp2.update(10, &mut |p| pkts.push(p.to_vec()));
        for p in pkts.drain(..) {
            cli2.send(&p).await.unwrap();
        }
        let mut got_hb2 = false;
        for tick in 1..100u32 {
            if let Ok(Ok(rlen)) =
                tokio::time::timeout(std::time::Duration::from_millis(10), cli2.recv(&mut buf))
                    .await
            {
                let _ = kcp2.input(&buf[..rlen]);
                while let Ok(pkt) = kcp2.recv_vec() {
                    if pkt.len() > 3 {
                        got_hb2 = true;
                    }
                }
                kcp2.update(10 + tick * 10, &mut |p| pkts.push(p.to_vec()));
                for p in pkts.drain(..) {
                    let _ = cli2.send(&p).await;
                }
                if got_hb2 {
                    break;
                }
            }
        }
        assert!(got_hb2, "cli2 heartbeat after unacked burst must succeed");
    });

    let _ = tx_shut.send(true);
    srv_handle.abort();
}

#[test]
fn audit_145_trample_leisure_parkour_stop_and_challenge_unlocks() {
    use crate::network::handlers::{events_misc, map, quest};
    use sonoro_gamesv_logic::helpers::condition;
    use sonoro_proto::enums::{ElevatorState, RangeType, TrampleGearTransition};
    use sonoro_proto::pb::{
        BtGiveUpRequest, EntityAccessRangeRequest, ParkourChallengeRequest,
        ParkourChallengeTransRequest, TrampleGearTransitionNotify,
    };

    let _ = ElevatorState::End;
    let mut h = TestHarness::new("audit_145_trample_parkour", Some(8));
    let mut events = EventQueue::default();
    {
        let sc = h.state.scene.as_mut().expect("scene");
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
            .expect("spawn active role 1102");
        sc.formation_info.cur_formation = 0;
        sc.formation_info.formations[0] = Formation {
            cur_role: 1102,
            name: None,
            roles: [
                Some(FormationRole {
                    role_id: 1102,
                    entity_id: role_ent.net_id,
                    ..Default::default()
                }),
                None,
                None,
            ],
        };
    }
    let _ = events.drain_all().count();
    h.state
        .player_components
        .quest
        .info
        .finished_quests
        .push(quest_engine::PROLOGUE_QUEST_ID);

    // 1. TrampleComponent pressure plate (Gameplay541, 122006969) emits Down/Up transitions cleanly.
    entity_cmd::spawn_with_location(
        &mut events,
        h.state.scene.as_mut().unwrap(),
        &h.state.fs,
        &h.state.assets,
        &mut h.state.conn,
        &h.state.player_components.basic,
        122_006_969,
        Some(false),
        Some(false),
        Some(8),
        None,
        true,
    )
    .expect("spawn trample 122006969");
    let trample_nid = h
        .state
        .scene
        .as_ref()
        .unwrap()
        .find_entity_net_id_by_config_id(122_006_969)
        .expect("trample spawned");
    let role_nid = h
        .state
        .scene
        .as_ref()
        .unwrap()
        .current_role_entity_id();
    assert!(role_nid > 0, "notify_join_scene must spawn active role entity");
    h.drain_count();
    let _ = events.drain_all().count();

    let _ = map::handle_entity_access_range_request(
        &mut h.state,
        &mut events,
        EntityAccessRangeRequest {
            EntityId: trample_nid,
            RangeType: Some(RangeType::RangeEnter),
            EntitiesToCheck: vec![role_nid],
        },
    )
    .expect("trample enter");
    let _ = events.drain_all().count();
    assert!(
        h.state
            .scene
            .as_ref()
            .unwrap()
            .entity(trample_nid)
            .unwrap()
            .world_item
            .trample_down,
        "TrampleComponent must enter pressed state on RangeEnter"
    );
    let trample_down_notifs: Vec<TrampleGearTransitionNotify> =
        h.drain_decoded(TrampleGearTransitionNotify::MSG_ID);
    assert!(trample_down_notifs.iter().any(|n| {
        n.TrampleGearEntiyId == trample_nid
            && n.TrampleGearTransition == Some(TrampleGearTransition::Down)
    }));

    let _ = map::handle_entity_access_range_request(
        &mut h.state,
        &mut events,
        EntityAccessRangeRequest {
            EntityId: trample_nid,
            RangeType: Some(RangeType::RangeLeave),
            EntitiesToCheck: vec![role_nid],
        },
    )
    .expect("trample leave non-reset");
    let _ = events.drain_all().count();
    assert!(
        h.state
            .scene
            .as_ref()
            .unwrap()
            .entity(trample_nid)
            .unwrap()
            .world_item
            .trample_down,
        "TrampleComponent 122006969 overrides IsResetGear=false and stays pressed on RangeLeave"
    );

    // 1b. Default Gameplay541 (102002394, IsResetGear=true from template) emits both Down and Up transitions.
    entity_cmd::spawn_with_location(
        &mut events,
        h.state.scene.as_mut().unwrap(),
        &h.state.fs,
        &h.state.assets,
        &mut h.state.conn,
        &h.state.player_components.basic,
        102_002_394,
        Some(false),
        Some(false),
        Some(8),
        None,
        true,
    )
    .expect("spawn trample 102002394");
    let reset_trample_nid = h
        .state
        .scene
        .as_ref()
        .unwrap()
        .find_entity_net_id_by_config_id(102_002_394)
        .expect("reset trample spawned");
    h.drain_count();
    let _ = events.drain_all().count();

    let _ = map::handle_entity_access_range_request(
        &mut h.state,
        &mut events,
        EntityAccessRangeRequest {
            EntityId: reset_trample_nid,
            RangeType: Some(RangeType::RangeEnter),
            EntitiesToCheck: vec![role_nid],
        },
    )
    .expect("reset trample enter");
    let _ = events.drain_all().count();
    assert!(
        h.state
            .scene
            .as_ref()
            .unwrap()
            .entity(reset_trample_nid)
            .unwrap()
            .world_item
            .trample_down,
        "TrampleComponent 102002394 must enter pressed state on RangeEnter"
    );
    let _ = h.drain_decoded::<TrampleGearTransitionNotify>(TrampleGearTransitionNotify::MSG_ID);

    let _ = map::handle_entity_access_range_request(
        &mut h.state,
        &mut events,
        EntityAccessRangeRequest {
            EntityId: reset_trample_nid,
            RangeType: Some(RangeType::RangeLeave),
            EntitiesToCheck: vec![role_nid],
        },
    )
    .expect("reset trample leave");
    let _ = events.drain_all().count();
    assert!(
        !h.state
            .scene
            .as_ref()
            .unwrap()
            .entity(reset_trample_nid)
            .unwrap()
            .world_item
            .trample_down,
        "TrampleComponent 102002394 must release on RangeLeave"
    );
    let trample_up_notifs: Vec<TrampleGearTransitionNotify> =
        h.drain_decoded(TrampleGearTransitionNotify::MSG_ID);
    assert!(trample_up_notifs.iter().any(|n| {
        n.TrampleGearEntiyId == reset_trample_nid
            && n.TrampleGearTransition == Some(TrampleGearTransition::Up)
    }));

    // 2. 3D LeisureInteract safety covers Bounce/Swing/Slide while keeping Catapult entity-only.
    for opt_type in ["Bounce", "Slide", "Soar2", "PainCoveringHead", "WindProtagonistParkour1"] {
        let act = serde_json::json!({
            "Name": "LeisureInteract",
            "Params": { "Option": { "Type": opt_type } }
        });
        assert!(
            quest_engine::triggers::is_safe_bt_leisure_interact(&act),
            "LeisureInteract {opt_type} must be BT-safe"
        );
    }
    for opt_type in ["Swing", "MotorEnterSlide", "FoxPushBox"] {
        let with_id = serde_json::json!({
            "Name": "LeisureInteract",
            "Params": { "Option": { "Type": opt_type, "EntityId": 12345 } }
        });
        assert!(
            quest_engine::triggers::is_safe_bt_leisure_interact(&with_id),
            "LeisureInteract {opt_type} with EntityId must be BT-safe"
        );
        let without_id = serde_json::json!({
            "Name": "LeisureInteract",
            "Params": { "Option": { "Type": opt_type } }
        });
        assert!(
            !quest_engine::triggers::is_safe_bt_leisure_interact(&without_id),
            "LeisureInteract {opt_type} without EntityId must not be BT-safe"
        );
    }
    let catapult = serde_json::json!({
        "Name": "LeisureInteract",
        "Params": { "Option": { "Type": "Catapult" } }
    });
    assert!(!quest_engine::triggers::is_safe_bt_leisure_interact(&catapult));

    // 3. Parkour challenges & gameplay samples stay locked early game (before Function 10053 + Condition 13000010).
    assert!(!condition::is_parkour_unlocked(&h.state.eval_context()));
    let locked_pk = events_misc::handle_parkour_challenge_request(&mut h.state, ParkourChallengeRequest {})
        .expect("parkour req while locked");
    assert!(locked_pk.Challenges.is_empty());
    let lp_parkour = h
        .state
        .assets
        .tables
        .level_play_data
        .get_data_by_id(&124_000_079)
        .expect("lp 124000079");
    assert!(!misc_logic::is_level_play_open_globally(
        &h.state.eval_context(),
        &lp_parkour,
    ));
    let lp_sample = h
        .state
        .assets
        .tables
        .level_play_data
        .get_data_by_id(&610_000_000)
        .expect("lp 610000000");
    assert!(!misc_logic::is_level_play_open_globally(
        &h.state.eval_context(),
        &lp_sample,
    ));
    assert!(misc_logic::is_entity_blocked_by_locked_level_play(
        &h.state.eval_context(),
        8,
        124_003_592,
    ));
    let mm_locked = misc_logic::build_map_mark_info_notify(
        &h.state.assets,
        &h.state.player_components.map,
        &h.state.player_components.basic,
        &h.state.player_components.quest,
    );
    assert!(
        !mm_locked.ShowMarkIds.iter().any(|m| m.MarkId == 10001),
        "Parkour MapMark 10001 must stay hidden before Parkour unlocks"
    );

    // 4. Unlock Parkour (Func 10053 + Quest 139000029 finished + Lv 14), enter 124000079, and verify GiveUp stops it cleanly.
    h.state
        .player_components
        .basic
        .info
        .attributes
        .level = 14;
    h.state
        .player_components
        .quest
        .info
        .unlocked_systems
        .push(10053);
    h.state
        .player_components
        .quest
        .info
        .finished_quests
        .push(139_000_029);
    assert!(condition::is_parkour_unlocked(&h.state.eval_context()));
    assert!(misc_logic::is_level_play_open_globally(
        &h.state.eval_context(),
        &lp_parkour,
    ));
    let mm_unlocked = misc_logic::build_map_mark_info_notify(
        &h.state.assets,
        &h.state.player_components.map,
        &h.state.player_components.basic,
        &h.state.player_components.quest,
    );
    assert!(mm_unlocked.ShowMarkIds.iter().any(|m| m.MarkId == 10001));

    let trans_resp = events_misc::handle_parkour_challenge_trans_request(
        &mut h.state,
        ParkourChallengeTransRequest { ChallengeId: 2 },
    )
    .expect("parkour trans 2");
    assert_eq!(trans_resp.ErrorCode, Some(ErrorCode::Success));

    {
        let mut sc = h.state.scene.take().unwrap();
        let mut lp_ctx = level_play::LevelPlayContext {
            assets: &h.state.assets,
            fs: &h.state.fs,
            sink: &mut h.state.conn,
            scene: &mut sc,
            scene_comp: Some(&mut h.state.player_components.scene),
            events: &mut events,
            basic_comp: &mut h.state.player_components.basic,
            inventory_comp: &mut h.state.player_components.inventory,
            weapon_comp: &mut h.state.player_components.weapon,
            echo_comp: &mut h.state.player_components.echo,
            map_comp: &mut h.state.player_components.map,
            quest_comp: &mut h.state.player_components.quest,
            role_comp: Some(&mut h.state.player_components.role),
            guide_comp: Some(&mut h.state.player_components.guide),
            mail_comp: Some(&mut h.state.player_components.mail),
            progress_comp: Some(&mut h.state.player_components.progress),
            activity_notify: None,
        };
        level_play::on_player_move(&mut lp_ctx, -37326.62, 114371.23, 4541.35)
            .expect("enter parkour 124000079");
        h.state.scene = Some(sc);
    }
    let _ = events.drain_all().count();
    assert!(
        h.state
            .player_components
            .quest
            .info
            .progress
            .entered_level_plays
            .contains(&124_000_079),
        "Parkour 124000079 must enter once unlocked and triggered"
    );

    quest::handle_bt_give_up_request(
        &mut h.state,
        &mut events,
        BtGiveUpRequest {
            TreeOwnerId: 124_000_079,
            TreeIncId: 0,
            NodeId: 0,
        },
    )
    .expect("give up parkour 124000079");
    let _ = events.drain_all().count();
    assert!(
        !h.state
            .player_components
            .quest
            .info
            .progress
            .entered_level_plays
            .contains(&124_000_079),
        "BtGiveUpRequest must exit Parkour challenge even when SaveConfig checkpoint exists"
    );
    assert!(
        h.state
            .scene
            .as_ref()
            .unwrap()
            .suppressed_auto_enter_lps
            .contains(&124_000_079)
    );
}

#[test]
fn audit_146_elevator_airwall_jinzhou_npcs_tower_and_tod_lock() {
    use crate::network::handlers::{scene as scene_handler, tower, world_puzzle};
    use sonoro_proto::enums::{ElevatorState, SceneDateUpdateReason};
    use sonoro_proto::pb::{
        AccessPasserbyNpcSpawnerRequest, ElevatorStateChangeRequest, TowerRequest,
        TowerSeasonUpdateRequest, UpdateSceneDateRequest, VfxNpcPatrolFinishRequest,
    };

    let mut h = TestHarness::new("audit_146_world_fixes", Some(8));
    let mut events = EventQueue::default();
    h.state
        .player_components
        .quest
        .info
        .finished_quests
        .push(quest_engine::PROLOGUE_QUEST_ID);

    // 1. Spawning near Elevator 109002461 fires EntityGroup 109900001 and closes AirWall 109900000.
    {
        let mut sc = h.state.scene.take().unwrap();
        scene_logic::handle_spatial_spawns_check(
            [-31895.0, 120726.0, 4876.0],
            &mut events,
            &mut sc,
            Some(&mut h.state.player_components.scene),
            &h.state.fs,
            &h.state.assets,
            &mut h.state.conn,
            &mut h.state.player_components.basic,
            &mut h.state.player_components.inventory,
            &mut h.state.player_components.weapon,
            &mut h.state.player_components.echo,
            &mut h.state.player_components.map,
            &mut h.state.player_components.quest,
            Some(&mut h.state.player_components.guide),
            Some(&mut h.state.player_components.mail),
            &mut h.state.player_components.progress,
            &mut h.state.player_components.role,
            None,
        )
        .expect("spawn near elevator 109002461");
        h.state.scene = Some(sc);
    }
    let _ = events.drain_all().count();
    let lift_nid = h
        .state
        .scene
        .as_ref()
        .unwrap()
        .find_entity_net_id_by_config_id(109_002_461)
        .expect("lift 109002461 spawned");
    assert!(
        h.state
            .scene
            .as_ref()
            .unwrap()
            .find_entity_net_id_by_config_id(109_900_000)
            .is_some(),
        "AirWall 109900000 must spawn near lift"
    );
    assert_eq!(
        h.state
            .player_components
            .quest
            .info
            .get_entity_state(109_900_000),
        Some("关卡.Common.状态.常态"),
        "Elevator Floor 1 must close AirWall 109900000 on spawn"
    );

    world_puzzle::handle_elevator_state_change_request(
        &mut h.state,
        ElevatorStateChangeRequest {
            EntityId: lift_nid,
            TagState: 2,
            State: Some(ElevatorState::End),
        },
    )
    .expect("move elevator to floor 2");
    assert_eq!(
        h.state
            .player_components
            .quest
            .info
            .get_entity_state(109_900_000),
        Some("关卡.Common.状态.激活"),
        "Moving elevator to Floor 2 must activate AirWall 109900000"
    );
    world_puzzle::handle_elevator_state_change_request(
        &mut h.state,
        ElevatorStateChangeRequest {
            EntityId: lift_nid,
            TagState: 1,
            State: Some(ElevatorState::End),
        },
    )
    .expect("move elevator back to floor 1");
    assert_eq!(
        h.state
            .player_components
            .quest
            .info
            .get_entity_state(109_900_000),
        Some("关卡.Common.状态.常态"),
        "Returning elevator to Floor 1 must re-close AirWall 109900000"
    );
    h.drain_count();

    // 2. Jinzhou NPCs (SimpleNPC216 SubEntityType == 1) and Gameplay581 PasserbyNpc spawner (SubEntityType == 2).
    {
        let mut sc = h.state.scene.take().unwrap();
        scene_logic::handle_spatial_spawns_check(
            [-43200.0, 96000.0, 1000.0],
            &mut events,
            &mut sc,
            Some(&mut h.state.player_components.scene),
            &h.state.fs,
            &h.state.assets,
            &mut h.state.conn,
            &mut h.state.player_components.basic,
            &mut h.state.player_components.inventory,
            &mut h.state.player_components.weapon,
            &mut h.state.player_components.echo,
            &mut h.state.player_components.map,
            &mut h.state.player_components.quest,
            Some(&mut h.state.player_components.guide),
            Some(&mut h.state.player_components.mail),
            &mut h.state.player_components.progress,
            &mut h.state.player_components.role,
            None,
        )
        .expect("spawn in Jinzhou");
        h.state.scene = Some(sc);
    }
    let _ = events.drain_all().count();
    let sc_ref = h.state.scene.as_ref().unwrap();
    let simple_nid = sc_ref
        .find_entity_net_id_by_config_id(11_000_339)
        .expect("SimpleNPC216 11000339 must spawn in Jinzhou");
    let simple_pb = sc_ref
        .entity(simple_nid)
        .unwrap()
        .to_proto(&h.state.assets, 0);
    assert_eq!(
        simple_pb.SubEntityType, 1,
        "SimpleNPC216 must emit SubEntityType=1"
    );
    assert!(
        sc_ref
            .entities
            .iter()
            .any(|e| e.world_item.npc_spawn_id == 11_000_704),
        "Jinzhou Gameplay581 spawner 11000704 must auto-spawn PasserbyNpc pedestrians"
    );
    world_puzzle::handle_access_passerby_npc_spawner_request(
        &mut h.state,
        &mut events,
        AccessPasserbyNpcSpawnerRequest {
            EntityId: 11_000_704,
            SplineConfigId: 140_800_452,
            IsSplneNotValid: false,
        },
    )
    .expect("spawn passerby on spline 140800452");
    let walker_nid = h
        .state
        .scene
        .as_ref()
        .unwrap()
        .entities
        .iter()
        .find(|e| e.world_item.npc_spawn_id == 11_000_704 && e.world_item.npc_spline_id == 140_800_452)
        .map(|e| e.net_id)
        .expect("PasserbyNpc spawned for spline 140800452");
    world_puzzle::handle_vfx_npc_patrol_finish_request(
        &mut h.state,
        &mut events,
        VfxNpcPatrolFinishRequest {
            EntityId: walker_nid,
        },
    )
    .expect("finish passerby patrol");
    assert!(h.state.scene.as_ref().unwrap().entity(walker_nid).is_none());
    h.drain_count();

    // 3. Tower MaxUnlockDifficulty stays 0 while Function 10055 is locked, then announces 1 once.
    let t_locked = tower::handle_tower_request(&mut h.state, TowerRequest {}).expect("tower locked");
    assert_eq!(t_locked.TowerInfo.unwrap().MaxUnlockDifficulty, 0);
    let ts_locked = tower::handle_tower_season_update_request(&mut h.state, TowerSeasonUpdateRequest {})
        .expect("season update locked");
    assert_eq!(ts_locked.MaxUnlockDifficulty, 0);

    h.state.player_components.basic.info.attributes.level = 15;
    let ts_first = tower::handle_tower_season_update_request(&mut h.state, TowerSeasonUpdateRequest {})
        .expect("season update first unlock");
    assert_eq!(ts_first.MaxUnlockDifficulty, 1);
    let ts_second = tower::handle_tower_season_update_request(&mut h.state, TowerSeasonUpdateRequest {})
        .expect("season update repeat must not re-open banner");
    assert_eq!(ts_second.MaxUnlockDifficulty, 0);

    // 4. TOD TimeFlowAuto must not darken early-game sky before Function 10018 unlocks, and SetTime defers past PlayFlow.
    let before_hr = h.state.scene.as_ref().unwrap().instance.map_time.hour;
    scene_handler::handle_update_scene_date_request(
        &mut h.state,
        &mut events,
        UpdateSceneDateRequest {
            AddDays: 0,
            Hour: 22,
            Minute: 30,
            Reason: Some(SceneDateUpdateReason::TimeFlowAuto),
        },
    )
    .expect("time flow auto before 10018");
    assert_eq!(
        h.state.scene.as_ref().unwrap().instance.map_time.hour,
        before_hr,
        "TimeFlowAuto drift must stay locked until Function 10018 (TimeOfDay) unlocks"
    );

    if let Some(sc) = h.state.scene.as_mut() {
        sc.clear_active_flow();
        sc.clear_pending_flows();
    }
    let mut cbs = quest_engine::action_callbacks();
    let flow_then_time = vec![
        serde_json::json!({
            "Name": "PlayFlow",
            "Params": { "FlowListName": "剧情_新主线_CE版_序章", "FlowId": 1, "StateId": 18 }
        }),
        serde_json::json!({
            "Name": "SetTime",
            "Params": { "Hour": 19, "Min": 30, "ShowUi": false }
        }),
    ];
    execute_action_list(
        &mut h.state.quest_context(&mut events),
        &mut cbs,
        139_000_027,
        223,
        &flow_then_time,
    )
    .expect("defer SetTime past PlayFlow");
    assert_ne!(
        h.state.scene.as_ref().unwrap().instance.map_time.hour,
        19,
        "SetTime after sync PlayFlow must NOT apply before the flow ends"
    );
    while let Some(sc) = h.state.scene.as_ref()
        && sc.active_flow_inc_id != 0
    {
        let inc = sc.active_flow_inc_id;
        let _ = crate::network::handlers::flow::handle_flow_end_request(
            &mut h.state,
            &mut events,
            sonoro_proto::pb::FlowEndRequest {
                FlowIncId: inc,
                OptionInfos: BTreeMap::new(),
                IsSkip: true,
                ..Default::default()
            },
        )
        .expect("end flow");
        let _ = events.drain_all().count();
    }
    assert_eq!(h.state.scene.as_ref().unwrap().instance.map_time.hour, 19);
}

#[test]
fn audit_147_ch2_cctv_alert_area_sneak_and_delay_state() {
    use crate::network::handlers::{combat_sync, world_puzzle};
    use crate::network::state::State;
    use sonoro_gamesv_logic::helpers::{alert, condition};
    use sonoro_proto::enums::SneakFinishType;
    use sonoro_proto::pb::{
        ChangeEntityStateRequest, EntityCameraAlertRequest, QuickHackSkillRequest, SneakFinishRequest,
        SneakNotify, SneakRequest, SneakTimeRequest,
    };

    let mut h = TestHarness::new("audit_147_ch2_cctv", Some(100));
    let mut events = EventQueue::default();

    // 1) All world gameplay entity types unblocked in spawn table.
    let mut cfg_stub = LevelEntityConfig::default();
    cfg_stub.Id = 1;
    cfg_stub.EntityId = 999_999;
    cfg_stub.MapId = 100;
    for bpt in [
        BpEntityType::Monitor,
        BpEntityType::DaemonHack,
        BpEntityType::DollGrabShowcase,
        BpEntityType::ExhibitBase,
        BpEntityType::Furniture,
        BpEntityType::Chair,
    ] {
        assert!(
            is_spawnable_item_config(&cfg_stub, bpt),
            "{bpt:?} must be spawnable"
        );
    }

    // Spawn real Monitor (36000369) and QuickHack target (39700001) on Map 100.
    let (mon_nid, qh_nid) = {
        let State {
            scene,
            fs,
            assets,
            conn,
            player_components,
            ..
        } = &mut h.state;
        let sc = scene.as_mut().expect("scene");
        let m = entity_cmd::spawn_with_location_and_progress(
            &mut events,
            sc,
            fs,
            assets,
            conn,
            &player_components.basic,
            36_000_369,
            Some(false),
            Some(false),
            Some(100),
            None,
            true,
            &player_components.progress.info,
            false,
            true,
        )
        .expect("spawn Monitor")
        .expect("Monitor handle");
        let q = entity_cmd::spawn_with_location_and_progress(
            &mut events,
            sc,
            fs,
            assets,
            conn,
            &player_components.basic,
            39_700_001,
            Some(false),
            Some(false),
            Some(100),
            None,
            true,
            &player_components.progress.info,
            false,
            true,
        )
        .expect("spawn QuickHack")
        .expect("QuickHack handle");
        (m.net_id, q.net_id)
    };
    let _ = events.drain_all().count();

    // 2) AlertArea + EntityCameraAlertRequest + CompareAlertValue Custom + CheckAlertAreaEnabled.
    let area_id = h
        .state
        .assets
        .alert_area_configs
        .keys()
        .copied()
        .next()
        .unwrap_or(1);
    {
        let mut qctx = h.state.quest_context(&mut events);
        let mut cbs = quest_engine::action_callbacks();
        let enable_params = serde_json::json!({
            "Option": { "Type": "EnableAlertArea", "AreaId": area_id, "IsEnable": true }
        });
        alert::apply_exec_alert_system_action(&mut qctx, &mut cbs, enable_params.as_object())
            .expect("enable AlertArea");
        let modify_params = serde_json::json!({
            "Option": {
                "Type": "ModifyAlertValue",
                "AreaId": area_id,
                "SetType": { "Type": 3, "CustomValue": 10.0 }
            }
        });
        alert::apply_exec_alert_system_action(&mut qctx, &mut cbs, modify_params.as_object())
            .expect("set AlertArea value");
        assert!(alert::is_alert_area_enabled(
            qctx.progress_comp.as_deref(),
            qctx.scene.as_deref(),
            area_id
        ));
    }
    let alert_resp = world_puzzle::handle_entity_camera_alert_request(
        &mut h.state,
        EntityCameraAlertRequest {
            EntityId: mon_nid,
            IsAlert: true,
        },
    )
    .expect("camera alert");
    assert_eq!(alert_resp.ErrorCode, Some(ErrorCode::Success));
    {
        let sc = h.state.scene.as_ref().unwrap();
        let rt = sc.alert_areas.get(&area_id).expect("AlertArea runtime");
        assert!(rt.alert_value >= 10.0);
        let qctx = h.state.quest_context(&mut events);
        let eval_ctx = qctx.make_eval_context();
        assert!(eval_single_inline_condition(
            &eval_ctx,
            &serde_json::json!({
                "Type": "CompareAlertValue",
                "AreaId": area_id,
                "CompareType": "Ge",
                "CompareValue": { "Type": 2, "CompareValue": 5.0 }
            })
        ));
        assert!(eval_single_inline_condition(
            &eval_ctx,
            &serde_json::json!({
                "Type": "CheckAlertAreaEnabled",
                "AreaId": area_id
            })
        ));
    }

    // 3) QuickHackSkillRequest & Sneak lifecycle.
    let qh_resp = combat_sync::handle_quick_hack_skill_request(
        &mut h.state,
        &mut events,
        QuickHackSkillRequest {
            EntityId: qh_nid,
            SkillId: 1,
        },
    )
    .expect("QuickHack skill");
    assert_eq!(qh_resp.ErrorCode, Some(ErrorCode::Success));

    let sneak_resp = world_puzzle::handle_sneak_request(
        &mut h.state,
        SneakRequest {
            TreeOwnerId: 139_000_025,
            NodeId: 1,
            IsStart: true,
            TreeIncId: 139_000_025,
        },
    )
    .expect("SneakRequest start");
    assert_eq!(sneak_resp.ErrorId, Some(ErrorCode::Success));
    assert!(!h.drain_decoded::<SneakNotify>(SneakNotify::MSG_ID).is_empty());

    let sneak_t = world_puzzle::handle_sneak_time_request(
        &mut h.state,
        SneakTimeRequest {
            TreeIncId: 139_000_025,
            TreeOwnerId: 139_000_025,
        },
    )
    .expect("SneakTimeRequest");
    assert_eq!(sneak_t.ErrorId, Some(ErrorCode::Success));

    let sneak_fin = world_puzzle::handle_sneak_finish_request(
        &mut h.state,
        SneakFinishRequest {
            SneakId: 1,
            FinishType: Some(SneakFinishType::SneakSuccess),
        },
    )
    .expect("SneakFinish");
    assert_eq!(sneak_fin.ErrorCode, Some(ErrorCode::Success));

    // 4) DelayChangeState schedules & transitions state when due.
    let lock_id = gameplay_tags::id_from_name(
        &h.state.assets.tables.gameplay_tag_remap,
        "关卡.Common.状态.锁定",
    )
    .unwrap_or(0);
    if let Some(sc) = h.state.scene.as_mut() {
        sc.upsert_pending_state_change(
            36_000_369,
            String::new(),
            "关卡.Common.状态.锁定".into(),
            0,
        );
        let due = sc.drain_due_state_changes(i64::MAX);
        assert_eq!(due.len(), 1);
    }
    let ch_resp = world_puzzle::handle_change_entity_state_request(
        &mut h.state,
        ChangeEntityStateRequest {
            EntityId: mon_nid as i32,
            StateTagId: lock_id,
        },
    )
    .expect("change state");
    assert_eq!(ch_resp.ErrorCode, Some(ErrorCode::Success));
    let _ = condition::eval_entity_visible_component;
}

#[test]
fn audit_148_step3_beam_fan_jigsaw_foundation_hook_and_timeline() {
    use crate::network::handlers::{combat_sync, events_misc, minimal, world_puzzle};
    use crate::network::state::State;
    use sonoro_proto::enums::EntityBeamReceiveType;
    use sonoro_proto::pb::{
        ActionNotify, AddPlacementNotify, BeamCastHitPlayerRequest, BeControlledRequest,
        BoardGridPositionInfo, EntityBeamReceiveRequest, EntityFollowTrackRequest,
        EntitySendEventRequest, FoundationRequest, HookLockPointFinishRequest,
        ItemRelateFoundationNotify, MovePlacementNotify, MovePlacementRequest,
        PlaceItemOnBoardRequest, PositionAndRotate, PullingFoundationRequest,
        SetBaoziStateRequest, SetFanStateRequest, TimelineTraceControlRequest,
        TimelineTraceStartRequest, TurntableCompleteRequest, VacuumCleanerRequest, Vector,
    };

    let mut h = TestHarness::new("audit_148_puzzles", Some(8));
    let mut events = EventQueue::default();

    let (cast_nid, recv_nid, pull_nid, fnd2_nid, board_nid, track_nid, fan_nid) = {
        let State {
            scene,
            fs,
            assets,
            conn,
            player_components,
            ..
        } = &mut h.state;
        let sc = scene.as_mut().expect("scene");
        let mut spawn_one = |cfg_id: i64| -> i64 {
            entity_cmd::spawn_with_location_and_progress(
                &mut events,
                sc,
                fs,
                assets,
                conn,
                &player_components.basic,
                cfg_id,
                Some(false),
                Some(false),
                Some(8),
                None,
                true,
                &player_components.progress.info,
                false,
                true,
            )
            .expect("spawn ok")
            .expect("handle")
            .net_id
        };
        (
            spawn_one(133_003_485),
            spawn_one(133_003_491),
            spawn_one(106_901_929),
            spawn_one(114_900_011),
            spawn_one(102_002_159),
            spawn_one(102_002_091),
            spawn_one(106_002_504),
        )
    };
    h.drain_count();

    // 1) BeamCast & BeamReceive
    let bc = combat_sync::handle_beam_cast_hit_player_request(
        &mut h.state,
        BeamCastHitPlayerRequest { EntityId: cast_nid },
    )
    .expect("beam cast");
    assert_eq!(bc.Code, Some(ErrorCode::Success));
    let br = world_puzzle::handle_entity_beam_receive_request(
        &mut h.state,
        EntityBeamReceiveRequest {
            EntityId: recv_nid,
            ReceiveType: Some(EntityBeamReceiveType::CompleteAction),
        },
    )
    .expect("beam receive");
    assert_eq!(br.ErrorCode, Some(ErrorCode::Success));

    // 2) Projector SendEvent, Baozi, Fan
    let se = world_puzzle::handle_entity_send_event_request(
        &mut h.state,
        EntitySendEventRequest {
            EntityId: cast_nid,
            EventKey: b"beam_done".to_vec(),
        },
    )
    .expect("send event");
    assert_eq!(se.ErrorCode, Some(ErrorCode::Success));
    assert!(
        h.state
            .player_components
            .progress
            .info
            .has_claim("ent_self_ev:133003485:beam_done"),
        "expected ent_self_ev claim recorded"
    );
    let _ = world_puzzle::handle_set_baozi_state_request(
        &mut h.state,
        SetBaoziStateRequest {
            FanEntityId: fan_nid,
            BaoziEntityId: recv_nid,
            IsActive: 1,
        },
    )
    .expect("baozi");
    let _ = world_puzzle::handle_set_fan_state_request(
        &mut h.state,
        SetFanStateRequest {
            FanEntityId: fan_nid,
            IsActive: 1,
        },
    )
    .expect("fan");

    // 3) Jigsaw PlaceItemOnBoard & MovePlacement (with config_id + PosOnBoard notifies)
    let place = events_misc::handle_place_item_on_board_request(
        &mut h.state,
        PlaceItemOnBoardRequest {
            TargetBoardEntityId: board_nid,
            PlacementEntityId: pull_nid,
            ToPlace: 1,
            PosOnBoard: Some(BoardGridPositionInfo {
                Row: 1,
                Column: 2,
                RotAngle: 0,
            }),
            PosRot: None,
        },
    )
    .expect("place on board");
    assert_eq!(place.ErrCode, Some(ErrorCode::Success));
    let add_n = h.drain_decoded::<AddPlacementNotify>(AddPlacementNotify::MSG_ID);
    assert!(add_n.iter().any(|n| {
        n.BoardEntityConfigId == 102_002_159
            && n.GridInfo
                .as_ref()
                .is_some_and(|g| g.OccupyingEntityConfigId == 106_901_929)
    }));

    let mv = world_puzzle::handle_move_placement_request(
        &mut h.state,
        MovePlacementRequest {
            TargetBoardEntityId: board_nid,
            PlacementEntityId: pull_nid,
            PosOnBoard: Some(BoardGridPositionInfo {
                Row: 0,
                Column: 1,
                RotAngle: 90,
            }),
            PosRot: Some(PositionAndRotate::default()),
        },
    )
    .expect("move placement");
    assert_eq!(mv.ErrCode, Some(ErrorCode::Success));
    let mv_n = h.drain_decoded::<MovePlacementNotify>(MovePlacementNotify::MSG_ID);
    assert!(mv_n.iter().any(|n| n.PosOnBoard.is_some()));

    // 4) PullingObjectFoundation, Foundation, BeControlled, Turntable, Vacuum, FollowTrack, Hook, Timeline
    let _ = events_misc::handle_pulling_foundation_request(
        &mut h.state,
        PullingFoundationRequest {
            EntityId: pull_nid,
            AccusedEntityId: fnd2_nid,
            Pos: Some(Vector::default()),
            Rot: None,
        },
    )
    .expect("pulling foundation");
    let relate_n = h.drain_decoded::<ItemRelateFoundationNotify>(ItemRelateFoundationNotify::MSG_ID);
    assert!(
        relate_n
            .iter()
            .any(|n| n.FoundationEntityId == 106_901_929 && n.ItemEntityId == 114_900_011)
    );

    let _ = events_misc::handle_foundation_request(
        &mut h.state,
        FoundationRequest {
            EntityId: fnd2_nid,
            AccusedEntityId: recv_nid,
            ActiveOperate: 1,
            Pos: Some(Vector::default()),
            Rot: None,
        },
    )
    .expect("foundation");
    let _ = world_puzzle::handle_be_controlled_request(
        &mut h.state,
        BeControlledRequest {
            EntityId: recv_nid,
            IsControl: false,
        },
    )
    .expect("be_controlled detach");
    let _ = events_misc::handle_turntable_complete_request(
        &mut h.state,
        TurntableCompleteRequest { EntityId: fan_nid },
    )
    .expect("turntable");
    let _ = events_misc::handle_vacuum_cleaner_request(
        &mut h.state,
        VacuumCleanerRequest {
            SkillId: 1,
            SceneOwnerId: 0,
            AdsorbEntityIncId: fan_nid,
            EntityIncIds: vec![recv_nid],
            Type: None,
        },
    )
    .expect("vacuum");
    let ft = minimal::handle_entity_follow_track_request(
        &mut h.state,
        EntityFollowTrackRequest {
            EntityId: track_nid,
        },
    )
    .expect("follow track");
    assert_eq!(ft.ErrorCode, Some(ErrorCode::Success));
    let _ = world_puzzle::handle_hook_lock_point_finish_request(
        &mut h.state,
        HookLockPointFinishRequest { EntityId: fan_nid },
    )
    .expect("hook finish");
    let _ = world_puzzle::handle_timeline_trace_start_request(
        &mut h.state,
        TimelineTraceStartRequest {
            EntityId: fan_nid,
            Index: 0,
        },
    )
    .expect("timeline start");
    let _ = world_puzzle::handle_timeline_trace_control_request(
        &mut h.state,
        TimelineTraceControlRequest {
            Forward: true,
            EntityId: fan_nid,
        },
    )
    .expect("timeline control");
    let _ = ActionNotify::MSG_ID;
}

#[test]
fn audit_149_roll_block_grid_physics_and_progression() {
    use crate::network::connection::drain_event_queue_best_effort;
    use crate::network::handlers::events_major;
    use sonoro_gamesv_logic::helpers::roll_block;
    use sonoro_proto::enums::RbGridDirection;
    use sonoro_proto::pb::{
        RbBlockPbStateStateOneof, RbBlockStateChangeNotify, RbInput, RbRollInput,
        RollBlockCloseGamePlayNotify, RollBlockExitGamePlayRequest,
        RollBlockGamePlayReadyRequest, RollBlockInputRequest, RollBlockMovementFinishRequest,
        RollBlockOpenGamePlayNotify, RollBlockResetGamePlayNotify,
        RollBlockResetGamePlayRequest, RollBlockToggleHintRequest,
        RollBlockUpdateAvailableInputsNotify, RollBlockUpdateGamePlayNotify,
    };

    let mut h = TestHarness::new("audit_149_roll_block", Some(100));
    let mut events = EventQueue::default();

    // 1) RollBlock & RollBlockTemplate assets loaded cleanly.
    assert!(h.state.assets.roll_blocks.contains_key(&700_004));
    assert!(h.state.assets.roll_blocks.contains_key(&700_001));
    assert!(h.state.assets.roll_block_templates.contains_key(&100_001));

    // Activate LevelPlay 41000000 node 5 (FinishRollBlock for CenterEntityId 41000018, Group 700004).
    h.state
        .player_components
        .quest
        .info
        .progress
        .entered_level_plays
        .push(41_000_000);
    h.state
        .player_components
        .progress
        .info
        .set_key_int("lp_act:41000000:5", 1);

    let launched = {
        let mut qctx = h.state.quest_context(&mut events);
        roll_block::try_launch_for_trigger(&mut qctx, "Interact", 41_700_031)
            .expect("launch roll block")
    };
    assert!(launched, "FinishRollBlock must launch on Interact trigger");
    drain_event_queue_best_effort(&mut events, &mut h.state);

    let opens = h.drain_decoded::<RollBlockOpenGamePlayNotify>(RollBlockOpenGamePlayNotify::MSG_ID);
    assert_eq!(opens.len(), 1);
    let open_info = opens[0].Info.as_ref().expect("open info");
    let inc_id = open_info.IncId;
    assert_eq!(open_info.GroupId, 700_004);
    assert_eq!(open_info.Difficulty, 1);
    assert!(!open_info.InitInvisibleEntityIds.is_empty());
    let sc = h.state.scene.as_ref().expect("scene");
    assert!(
        open_info
            .InitInvisibleEntityIds
            .iter()
            .all(|nid| sc.entity(*nid).is_some()),
        "RollBlock must spawn board entities into scene"
    );
    assert!(
        sc.entities.iter().any(|e| e.world_item.rb_block.is_some()),
        "RollBlock must spawn at least one RbBlock"
    );
    assert!(
        sc.entities.iter().any(|e| e.world_item.rb_floor.is_some()),
        "RollBlock must spawn at least one RbFloor"
    );

    // 2) Ready -> pushes AvailableInputs.
    events_major::handle_roll_block_game_play_ready_request(
        &mut h.state,
        RollBlockGamePlayReadyRequest { IncId: inc_id },
    )
    .expect("ready");
    let avail = h.drain_decoded::<RollBlockUpdateAvailableInputsNotify>(
        RollBlockUpdateAvailableInputsNotify::MSG_ID,
    );
    assert_eq!(avail.len(), 1);
    assert!(!avail[0].AvailableInputs.is_empty());

    // 3) Replay HintSteps for 700004 Diff 1 -> should solve Diff 1 and push Diff 2 update!
    let blk_nid = h
        .state
        .scene
        .as_ref()
        .unwrap()
        .roll_block_sessions
        .get(&inc_id)
        .unwrap()
        .blocks[0]
        .net_id;
    let hints: Vec<RbGridDirection> = h
        .state
        .assets
        .roll_blocks
        .get(&700_004)
        .unwrap()
        .Levels[0]
        .HintSteps
        .iter()
        .map(|s| match s.Direction.as_str() {
            "Backward" => RbGridDirection::RbBackward,
            "Right" => RbGridDirection::RbRight,
            "Left" => RbGridDirection::RbLeft,
            _ => RbGridDirection::RbForward,
        })
        .collect();
    assert!(!hints.is_empty());

    for (idx, dir) in hints.iter().enumerate() {
        events_major::handle_roll_block_input_request(
            &mut h.state,
            RollBlockInputRequest {
                IncId: inc_id,
                Input: Some(RbInput {
                    Roll: Some(RbRollInput {
                        Direction: Some(*dir),
                    }),
                }),
            },
        )
        .expect("input");
        let st_n = h.drain_decoded::<RbBlockStateChangeNotify>(RbBlockStateChangeNotify::MSG_ID);
        assert!(
            matches!(
                st_n.last().and_then(|n| n.State.as_ref()).and_then(|s| s.State.as_ref()),
                Some(RbBlockPbStateStateOneof::MovingState(_))
            ),
            "step {idx} must emit MovingState"
        );
        events_major::handle_roll_block_movement_finish_request(
            &mut h.state,
            RollBlockMovementFinishRequest {
                IncId: inc_id,
                EntityId: blk_nid,
            },
        )
        .expect("movement finish");
    }

    let updates =
        h.drain_decoded::<RollBlockUpdateGamePlayNotify>(RollBlockUpdateGamePlayNotify::MSG_ID);
    assert_eq!(
        updates.len(),
        1,
        "Completing Difficulty 1 of 700004 must advance to Difficulty 2"
    );
    assert_eq!(updates[0].Info.as_ref().unwrap().Difficulty, 2);

    // 4) Reset, ToggleHint, Exit cleanly.
    events_major::handle_roll_block_toggle_hint_request(
        &mut h.state,
        RollBlockToggleHintRequest {
            IncId: inc_id,
            IsOpen: true,
        },
    )
    .expect("toggle hint");
    events_major::handle_roll_block_reset_game_play_request(
        &mut h.state,
        RollBlockResetGamePlayRequest { IncId: inc_id },
    )
    .expect("reset");
    assert!(
        !h.drain_decoded::<RollBlockResetGamePlayNotify>(RollBlockResetGamePlayNotify::MSG_ID)
            .is_empty()
    );
    events_major::handle_roll_block_exit_game_play_request(
        &mut h.state,
        RollBlockExitGamePlayRequest { IncId: inc_id },
    )
    .expect("exit");
    assert!(
        !h.drain_decoded::<RollBlockCloseGamePlayNotify>(RollBlockCloseGamePlayNotify::MSG_ID)
            .is_empty()
    );
}

#[test]
fn audit_150_doll_grab_exhibit_furniture_and_spring_festival() {
    use crate::network::handlers::{activity, events_major, events_misc, world_puzzle};
    use crate::network::state::State;
    use sonoro_proto::pb::{
        ActivityDataDataOneof, DollDeliveryNotify, DollDeliveryRequest, DollGarbInsertCoinRequest,
        DollGarbStartRequest, DollGrabRequest, DollGrabStopRequest, DollGrabTimeBallRequest,
        EntityItemBundleInfo, ExhibitionItemSaveRequest, FurnitureDiySaveRequest,
        FurnitureDiySlotInfo, SpringFestivalAtmosphereUpdateNotify,
    };

    let mut h = TestHarness::new("audit_150_doll_furn", Some(8));
    let mut events = EventQueue::default();

    // 1) Spawn DollGrabMachine (150750068) on Map 8.
    let doll_machine_nid = {
        let State {
            scene,
            fs,
            assets,
            conn,
            player_components,
            ..
        } = &mut h.state;
        let sc = scene.as_mut().expect("scene");
        let dm = entity_cmd::spawn_with_location_and_progress(
            &mut events,
            sc,
            fs,
            assets,
            conn,
            &player_components.basic,
            150_750_068,
            Some(false),
            Some(false),
            Some(8),
            None,
            true,
            &player_components.progress.info,
            false,
            true,
        )
        .expect("spawn doll machine")
        .expect("dm handle");
        let pb = sc.entity(dm.net_id).unwrap().to_proto(assets, 0);
        assert!(
            pb.ComponentPbs.iter().any(|c| matches!(
                &c.ComponentPb,
                Some(EntityComponentPbComponentPbOneof::DollGrabMachineComponentPb(d))
                    if !d.CanCapturedItems.is_empty()
            )),
            "DollGrabMachine must populate CanCapturedItems from ItemUniId list"
        );
        dm.net_id
    };
    h.drain_count();

    // Grab the 3 parts of combo 350001 (MainItemId=350004, PartItemIds=[350005, 350006]) + timeball!
    let _ = events_major::handle_doll_garb_insert_coin_request(
        &mut h.state,
        DollGarbInsertCoinRequest {
            EntityIncId: doll_machine_nid,
        },
    )
    .expect("coin");
    let _ = events_major::handle_doll_garb_start_request(
        &mut h.state,
        DollGarbStartRequest {
            EntityIncId: doll_machine_nid,
        },
    )
    .expect("start");
    for item_id in [350_004, 350_005, 350_006] {
        let gr = events_major::handle_doll_grab_request(
            &mut h.state,
            DollGrabRequest {
                EntityIncId: doll_machine_nid,
                ItemUniId: item_id,
            },
        )
        .expect("doll grab");
        assert_eq!(gr.ErrorCode, Some(ErrorCode::Success));
        assert!(!gr.GrabItemInfos.is_empty());
    }
    let _ = events_major::handle_doll_grab_time_ball_request(
        &mut h.state,
        DollGrabTimeBallRequest {
            EntityIncId: doll_machine_nid,
            ItemUniId: 750_014,
        },
    )
    .expect("time ball");
    let stop_r = events_major::handle_doll_grab_stop_request(
        &mut h.state,
        DollGrabStopRequest {
            EntityIncId: doll_machine_nid,
        },
    )
    .expect("stop");
    assert!(stop_r.SessionScore >= 45);

    let deliv = events_major::handle_doll_delivery_request(
        &mut h.state,
        DollDeliveryRequest {
            EntityCfgId: 13_750_056,
        },
    )
    .expect("delivery");
    assert!(deliv.ItemIds.contains(&350_004));
    assert!(
        !h.drain_decoded::<DollDeliveryNotify>(DollDeliveryNotify::MSG_ID)
            .is_empty(),
        "DollDelivery must push DollDeliveryNotify"
    );
    assert!(
        h.state
            .player_components
            .progress
            .info
            .get_key_int("doll:combo:13750056")
            >= 1,
        "Combo 350001 must increment doll:combo:13750056 for CheckDollGrabShowcaseItemComboAchieved"
    );

    // 2) ExhibitionItemSaveRequest persists and hydrates ExhibitBase component.
    let ex_resp = events_misc::handle_exhibition_item_save_request(
        &mut h.state,
        ExhibitionItemSaveRequest {
            EntityItemBundleInfos: vec![EntityItemBundleInfo {
                EntityId: 42_750_039,
                ItemId: 60_000_123,
            }],
        },
    )
    .expect("exhibit save");
    assert_eq!(ex_resp.ErrCode, Some(ErrorCode::Success));
    assert_eq!(
        h.state
            .player_components
            .progress
            .info
            .get_key_int("exhibit:42750039"),
        60_000_123
    );

    // 3) FurnitureDiySaveRequest computes real Atmosphere & notifies SpringFestivalActivity.
    let furn_id = *h
        .state
        .assets
        .furniture_configs
        .keys()
        .next()
        .expect("Furniture config");
    let diy_resp = world_puzzle::handle_furniture_diy_save_request(
        &mut h.state,
        FurnitureDiySaveRequest {
            IsOneClickPreset: false,
            ActivityId: 107_900_001,
            AreaId: 101,
            FurnitureDiySlotInfos: vec![FurnitureDiySlotInfo {
                SlotEntityCfgId: 691_750_152,
                RootFurnitureId: furn_id,
                SubFurnitureIds: Vec::new(),
            }],
        },
    )
    .expect("furniture diy save");
    assert_eq!(diy_resp.ErrCode, Some(ErrorCode::Success));
    assert!(diy_resp.Atmosphere > 0);
    assert!(
        !h.drain_decoded::<SpringFestivalAtmosphereUpdateNotify>(
            SpringFestivalAtmosphereUpdateNotify::MSG_ID
        )
        .is_empty()
    );

    let acts = {
        let qctx = h.state.quest_context(&mut events);
        let eval_ctx = qctx.make_eval_context();
        activity::build_activity_list(
            qctx.assets,
            qctx.progress_comp.as_deref().unwrap(),
            Some(&eval_ctx),
        )
    };
    let sf = acts.iter().find_map(|a| match &a.Data {
        Some(ActivityDataDataOneof::SpringFestivalActivityInfo(s)) => Some(s),
        _ => None,
    });
    if let Some(info) = sf {
        assert!(!info.UnlockFurnitures.is_empty());
        assert!(!info.AreaInfos.is_empty());
        assert!(info.Atmosphere > 0);
    }
}

#[test]
fn audit_151_native_protobuf_replaces_jspatch_and_reset_pos_failure_flow() {
    use sonoro_gamesv_logic::commands::js_patches;
    use sonoro_gamesv_logic::helpers::quest_actions;
    use sonoro_gamesv_logic::helpers::quest_engine;
    use sonoro_proto::pb::{
        ActionResetPosPrePerformanceEndRequest, ActionResetPosPrePerformanceNotify,
        JSPatchNotify, TeleportNotify, WeatherNotify,
    };

    let mut h = TestHarness::new("audit_151_native_proto", Some(8));
    h.drain_count();

    // 1) Weather push emits native WeatherNotify and zero JSPatchNotify.
    js_patches::weather::push(&mut h.state.conn, 2).expect("weather push");
    let ids = h.drain_push_msg_ids();
    assert!(ids.contains(&WeatherNotify::MSG_ID));
    assert!(!ids.contains(&JSPatchNotify::MSG_ID));

    // 2) ResetPlayerPosWithPresentation (Type=Failure) emits ActionResetPosPrePerformanceNotify,
    //    then ActionResetPosPrePerformanceEndRequest completes TeleportNotify and clears state.
    let mut events = EventQueue::default();
    let act_val = serde_json::json!({
        "Name": "ResetPlayerPosWithPresentation",
        "Params": {
            "Presentation": { "Type": "Failure", "Duration": 1.0, "GamePlayCues": [643901001] },
            "ResetPosConfig": { "Type": "PosEntity", "Entity": 6700003 }
        }
    });
    {
        let mut qctx = h.state.quest_context(&mut events);
        let mut cbs = quest_engine::action_callbacks();
        quest_actions::execute_action_list(&mut qctx, &mut cbs, 1, 1, &[act_val])
            .expect("reset pos failure action");
    }
    let pre_notifies = h.drain_decoded::<ActionResetPosPrePerformanceNotify>(
        ActionResetPosPrePerformanceNotify::MSG_ID,
    );
    assert_eq!(pre_notifies.len(), 1);
    let inc_id = pre_notifies[0].IncId;
    assert!(inc_id >= 1);
    let pid = h.state.player_id;
    let end_resp = crate::network::handlers::world_puzzle::handle_action_reset_pos_pre_performance_end_request(
        &mut h.state,
        ActionResetPosPrePerformanceEndRequest {
            PlayerId: pid,
            Index: 0,
            ContinueExecute: true,
            IncId: inc_id,
        },
    )
    .expect("reset pos end");
    assert_eq!(end_resp.Code, Some(ErrorCode::Success));
    let tp_notifies = h.drain_decoded::<TeleportNotify>(TeleportNotify::MSG_ID);
    assert!(!tp_notifies.is_empty(), "ContinueExecute must teleport player");
    assert_eq!(
        h.state
            .player_components
            .progress
            .info
            .get_key_int(&format!("act:reset_pos_target:{inc_id}:0")),
        0,
        "pending reset pos key must be cleared after end"
    );

    // 3) TakePlotPhoto action must NOT falsely set photo:submitted=1.
    let photo_act = serde_json::json!({
        "Name": "TakePlotPhoto",
        "Params": { "PhotoId": 101 }
    });
    {
        let mut qctx = h.state.quest_context(&mut events);
        let mut cbs = quest_engine::action_callbacks();
        quest_actions::execute_action_list(&mut qctx, &mut cbs, 1, 2, &[photo_act])
            .expect("take plot photo");
    }
    assert_eq!(
        h.state
            .player_components
            .progress
            .info
            .get_key_int("photo:submitted"),
        0,
        "TakePlotPhoto must wait for real submission rather than pre-marking submitted"
    );
}

#[test]
fn audit_152_quest_mode_actions_and_condition_operators() {
    use sonoro_gamesv_logic::helpers::condition;
    use sonoro_gamesv_logic::helpers::quest_actions;
    use sonoro_gamesv_logic::helpers::quest_engine;

    let mut h = TestHarness::new("audit_152_actions_conds", Some(8));
    let mut events = EventQueue::default();

    // Test RogueGotoNextFloor, DangoAbyssGotoNextFloor, HonamiStoryInteractPickUp, RoverRogueActivate
    let actions = vec![
        serde_json::json!({ "Name": "RogueGotoNextFloor", "Params": { "Floor": 2 } }),
        serde_json::json!({ "Name": "DangoAbyssGotoNextFloor", "Params": {} }),
        serde_json::json!({ "Name": "HonamiStoryInteractPickUp", "Params": { "ItemId": 9001 } }),
        serde_json::json!({ "Name": "RoverRogueActivate", "Params": {} }),
    ];
    {
        let mut qctx = h.state.quest_context(&mut events);
        let mut cbs = quest_engine::action_callbacks();
        quest_actions::execute_action_list(&mut qctx, &mut cbs, 10, 1, &actions)
            .expect("quest mode actions");
    }
    assert!(
        h.state
            .player_components
            .progress
            .info
            .get_key_int("rogue:layer")
            >= 1
    );
    assert!(
        h.state
            .player_components
            .progress
            .info
            .get_key_int("abyss:room")
            >= 1
    );

    // Test inline & table condition evaluation (CheckRoleOwned & LimitParams operators)
    let eval_ctx = {
        use sonoro_gamesv_data::tables::condition::Condition as TableCondition;
        let qctx = h.state.quest_context(&mut events);
        let role_id = *qctx
            .role_comp
            .as_ref()
            .and_then(|r| r.role_map.keys().next())
            .expect("PlayerRoleComponent has starter role");
        let owned_cond = serde_json::json!({
            "Type": 0,
            "Conditions": [{
                "Type": "CheckRoleOwned",
                "RoleId": role_id
            }]
        });
        let ec = qctx.make_eval_context();
        assert!(condition::eval_inline_condition_block(&ec, &owned_cond));
        let owned_tbl = TableCondition {
            Id: 9_999_001,
            Type: "CheckRoleOwned".to_string(),
            NeedNum: 1,
            LimitParams: Some(serde_json::json!({ "RoleId": role_id })),
            LimitParamsOpe: None,
            IsClient: false,
        };
        assert!(condition::eval_condition_record(&ec, &owned_tbl));
        let shop_cond = TableCondition {
            Id: 9_999_002,
            Type: "FishingTechLevel".to_string(),
            NeedNum: 1,
            LimitParams: Some(serde_json::json!({ "Level": 99 })),
            LimitParamsOpe: Some(serde_json::json!({ "Level": "Ge" })),
            IsClient: false,
        };
        !condition::eval_condition_record(&ec, &shop_cond)
    };
    assert!(eval_ctx, "FishingTechLevel >= 99 must be false at level 0");
}

#[test]
fn audit_153_world_controllers_flower_pollution_seq_frame_and_patrol() {
    use crate::network::handlers::{events_misc, world_puzzle};
    use sonoro_gamesv_data::tables::entity_components::extra::ExtraEntry;
    use sonoro_gamesv_logic::helpers::world_controllers;
    use sonoro_proto::pb::{
        FlowerPollutionSplinePush, MonsterPatrolPointReachedRequest,
        SceneItemSequenceFrameEndRequest, SceneItemSequenceFrameStartRequest,
    };

    let mut h = TestHarness::new("audit_153_world_ctrl", Some(8));
    h.drain_count();

    // 1) AssetIndexes must expose world controllers on Map 8 / Map 100.
    let ctrls_m8 = h.state.assets.world_controller_entities(8);
    assert!(
        !ctrls_m8.is_empty(),
        "Map 8 must load cached RefreshGroup/EntityBatchRefresh/TimeSchedule controllers"
    );

    // 2) FlowerPollutionSplinePush persists splines & hydrates FlowerPollutionComponentPb.
    events_misc::handle_flower_pollution_spline_push(
        &mut h.state,
        FlowerPollutionSplinePush {
            ManagerEntityId: 1,
            IsRemovePollution: true,
            SplineConfigId: 101,
        },
    )
    .expect("flower spline push");
    let has_merged_well = h
        .state
        .assets
        .find_level_entity(100, 138_501_78)
        .is_some_and(|cfg| {
            let mut m = cfg.ComponentsData.clone();
            if let Some(tpl) = h.state.assets.find_template_config(&cfg.BlueprintType) {
                tpl.ComponentsData.merge_into(&mut m);
            }
            m.extra
                .iter()
                .chain(m.template_extra.iter())
                .any(|e| matches!(e, ExtraEntry::ElementalWellComponent(_)))
        });
    assert!(has_merged_well, "ElementalWell entity 13850178 must merge template well config");

    // 3) MonsterPatrolPointReachedRequest resolves real SplineComponent point index on 16000135.
    let patrol_resp = world_puzzle::handle_monster_patrol_point_reached_request(
        &mut h.state,
        MonsterPatrolPointReachedRequest {
            EntityId: 16_000_135,
            SplineEntityConfigId: 16_000_135,
            Index: 1,
        },
    )
    .expect("patrol point");
    assert_eq!(patrol_resp.ErrorCode, Some(ErrorCode::Success));
    assert_eq!(
        h.state
            .player_components
            .progress
            .info
            .get_key_int("patrol:pt:16000135"),
        1
    );

    // 4) SequenceFrame start/end executes without error and records frame state.
    let pid = h.state.player_id;
    let seq_start = world_puzzle::handle_scene_item_sequence_frame_start_request(
        &mut h.state,
        SceneItemSequenceFrameStartRequest {
            SeqGuid: b"939cd1ee14294e14bd68c2bfb112ca2e".to_vec(),
            EventName: b"\xe6\x8c\x81\xe6\x9c\x89Tag".to_vec(),
            HostPlayerId: pid,
            EntityId: 13_750_003,
        },
    )
    .expect("seq frame start");
    assert_eq!(seq_start.Code, Some(ErrorCode::Success));
    let seq_end = world_puzzle::handle_scene_item_sequence_frame_end_request(
        &mut h.state,
        SceneItemSequenceFrameEndRequest {
            SeqGuid: b"939cd1ee14294e14bd68c2bfb112ca2e".to_vec(),
            EventName: b"\xe6\x8c\x81\xe6\x9c\x89Tag".to_vec(),
            HostPlayerId: pid,
            EntityId: 13_750_003,
        },
    )
    .expect("seq frame end");
    assert_eq!(seq_end.Code, Some(ErrorCode::Success));

    // 5) World controllers tick cleanly on Map 8.
    let mut events = EventQueue::default();
    {
        let mut qctx = h.state.quest_context(&mut events);
        world_controllers::tick_world_controllers(&mut qctx).expect("tick world controllers");
    }
}

#[test]
fn audit_154_activities_hydrated_from_assets_and_no_fake_fallbacks() {
    use crate::network::handlers::{activity, endgame, roguelike};
    use sonoro_proto::pb::{
        ActivityDataDataOneof, BossRushClaimRewardRequest, RogueWeeklyBlackFlowerRequest,
    };

    let mut h = TestHarness::new("audit_154_act_real", Some(8));

    // 1) build_activity_list hydrates real table rows across activity types.
    let acts = activity::build_activity_list(
        &h.state.assets,
        &h.state.player_components.progress,
        None,
    );
    assert!(!acts.is_empty());
    let has_boss_rush = acts.iter().any(|a| matches!(
        &a.Data,
        Some(ActivityDataDataOneof::BossRushActivityData(br)) if !br.LevelDetailInfo.is_empty()
    ));
    let has_new_tower = acts.iter().any(|a| matches!(
        &a.Data,
        Some(ActivityDataDataOneof::NewTowerClimbingActivityData(nt)) if !nt.Records.is_empty()
    ));
    assert!(has_boss_rush, "BossRushActivityData must be hydrated from BossRushActivity table");
    assert!(has_new_tower, "NewTowerClimbingActivityData must be hydrated from NewTowerLevel table");

    // 2) Claiming an invalid endgame reward ID must NOT inject fake Astrite (item_id=3) / Credit.
    let astrite_before = h
        .state
        .player_components
        .inventory
        .info
        .normal_item_count(3);
    let bad_claim = endgame::handle_boss_rush_claim_reward_request(
        &mut h.state,
        BossRushClaimRewardRequest {
            ActivityId: 100_805_001,
            RewardType: Some(sonoro_proto::enums::BossRushRewardType::Level),
            RewardId: 999_999_999,
        },
    )
    .expect("bad claim succeeds without fallback");
    assert_eq!(bad_claim.ErrorCode, Some(ErrorCode::Success));
    let astrite_after = h
        .state
        .player_components
        .inventory
        .info
        .normal_item_count(3);
    assert_eq!(
        astrite_after, astrite_before,
        "Invalid reward ID must NOT grant hardcoded Astrite fallback"
    );

    // 3) RogueWeekly black flower handles real RogueWeeklyBF SilentAreaDetectionId without fake fallback.
    let bf = roguelike::handle_rogue_weekly_black_flower_request(
        &mut h.state,
        RogueWeeklyBlackFlowerRequest {
            EntityConfigId: 1,
            IsDouble: false,
            Ret: true,
            SilentAreaId: 30_010_001,
        },
    )
    .expect("black flower");
    assert_eq!(bf.ErrorCode, Some(ErrorCode::Success));
    assert_eq!(bf.UseFreeCount, 1);
}

#[test]
fn audit_155_throwable_pilot_throw_attributes_and_timeline_pb() {
    use crate::network::handlers::world_puzzle;
    use sonoro_gamesv_logic::commands::entity as entity_cmd;
    use sonoro_proto::enums::{EControlState, PilotThrowOperationType};
    use sonoro_proto::pb::{
        BeControlledRequest, BeControlledThrowRequest, CombatReceivePackNotify,
        EntityComponentPbComponentPbOneof, MoveSceneItemResetPositionRequest,
        PilotThrowOperationRequest, TimelineTraceControlRequest, TimelineTraceStartRequest,
    };

    let mut h = TestHarness::new("audit_155_throw_attr_tl", Some(8));
    let mut events = EventQueue::default();

    // 1) Spawn real TeleControl2 throwable (122006287, DestroyCfg=[CreateBullet, LetGo]),
    //    hold -> throw (LetGo triggers DestroyCfg hide) -> reset position restores visibility & clears BB.
    {
        let mut sc = h.state.scene.take().unwrap();
        entity_cmd::call(
            &mut events,
            &mut sc,
            &h.state.fs,
            &h.state.assets,
            &mut h.state.conn,
            &h.state.player_components.basic,
            122_006_287,
            None,
            None,
        )
        .expect("spawn throwable 122006287");
        h.state.scene = Some(sc);
    }
    h.drain_count();
    let throw_nid = h
        .state
        .scene
        .as_ref()
        .unwrap()
        .find_entity_net_id_by_config_id(122_006_287)
        .expect("122006287 net id");
    let hold_resp = world_puzzle::handle_be_controlled_request(
        &mut h.state,
        BeControlledRequest {
            EntityId: throw_nid,
            IsControl: true,
        },
    )
    .expect("be controlled hold");
    assert_eq!(hold_resp.ErrorCode, Some(ErrorCode::Success));

    let letgo_resp = world_puzzle::handle_be_controlled_throw_request(
        &mut h.state,
        BeControlledThrowRequest {
            EntityId: throw_nid,
            ControlState: Some(EControlState::LetGo),
            ThrowType: 0,
        },
    )
    .expect("be controlled let go");
    assert_eq!(letgo_resp.ErrorCode, Some(ErrorCode::Success));
    assert_eq!(
        h.state
            .player_components
            .progress
            .info
            .get_key_int("item_bb:122006287:0"),
        11
    );
    assert!(
        h.state
            .player_components
            .progress
            .info
            .has_claim("be_ctrl:destroy:122006287"),
        "LetGo must honor TeleControl2 DestroyCfg"
    );
    assert!(
        !h.drain_decoded::<CombatReceivePackNotify>(CombatReceivePackNotify::MSG_ID)
            .is_empty(),
        "DestroyCfg hide must push CombatReceivePackNotify visibility update"
    );
    let throw_pb = h
        .state
        .scene
        .as_ref()
        .unwrap()
        .entity(throw_nid)
        .unwrap()
        .to_proto(&h.state.assets, 0);
    assert!(throw_pb.ComponentPbs.iter().any(|c| matches!(
        &c.ComponentPb,
        Some(EntityComponentPbComponentPbOneof::BlackboardParamComponent(bb)) if !bb.BlackboardParams.is_empty()
    )));

    let reset_resp = world_puzzle::handle_move_scene_item_reset_position_request(
        &mut h.state,
        MoveSceneItemResetPositionRequest {
            EntityId: throw_nid,
            ResetEntityId: throw_nid,
            SceneOwnerId: 0,
        },
    )
    .expect("reset throwable");
    assert_eq!(reset_resp.ErrorCode, Some(ErrorCode::Success));
    assert_eq!(
        h.state
            .player_components
            .progress
            .info
            .get_key_int("item_bb:122006287:0"),
        0
    );
    assert!(
        !h.state
            .player_components
            .progress
            .info
            .has_claim("be_ctrl:destroy:122006287")
    );

    // 2) PilotThrowOperationRequest clears hook active state and finishes active PilotThrow target.
    {
        let info = &mut h.state.player_components.progress.info;
        info.set_key_int("hook_point:active:122006287", 1);
        info.set_key_int("pilot_throw:hook_net", throw_nid);
        info.set_key_int("pilot_throw:hook_cfg", 122_006_287);
        info.set_key_int("pilot_throw:titan_cfg", 147_750_026);
    }
    let pilot_resp = world_puzzle::handle_pilot_throw_operation_request(
        &mut h.state,
        PilotThrowOperationRequest {
            OpType: Some(PilotThrowOperationType::Ptothrow),
        },
    )
    .expect("pilot throw");
    assert_eq!(pilot_resp.ErrorCode, Some(ErrorCode::Success));
    assert_eq!(
        h.state
            .player_components
            .progress
            .info
            .get_key_int("pilot_throw:hook_cfg"),
        0
    );
    assert_eq!(
        h.state
            .player_components
            .quest
            .info
            .get_entity_state(122_006_287),
        Some("关卡.Common.状态.完成")
    );

    // 3) TimelineTrackComponentPb and SceneItemAttributeComponentPb emit cleanly on spawn & trace control.
    {
        let mut sc = h.state.scene.take().unwrap();
        entity_cmd::call(
            &mut events,
            &mut sc,
            &h.state.fs,
            &h.state.assets,
            &mut h.state.conn,
            &h.state.player_components.basic,
            133_003_532,
            None,
            None,
        )
        .expect("spawn timeline 133003532");
        entity_cmd::call(
            &mut events,
            &mut sc,
            &h.state.fs,
            &h.state.assets,
            &mut h.state.conn,
            &h.state.player_components.basic,
            147_750_026,
            None,
            None,
        )
        .expect("spawn attr entity 147750026");
        h.state.scene = Some(sc);
    }
    let tl_nid = h
        .state
        .scene
        .as_ref()
        .unwrap()
        .find_entity_net_id_by_config_id(133_003_532)
        .expect("133003532 net id");
    let tl_pb = h
        .state
        .scene
        .as_ref()
        .unwrap()
        .entity(tl_nid)
        .unwrap()
        .to_proto(&h.state.assets, 0);
    assert!(tl_pb.ComponentPbs.iter().any(|c| matches!(
        &c.ComponentPb,
        Some(EntityComponentPbComponentPbOneof::TimelineTrackComponentPb(t)) if !t.ControlDatas.is_empty()
    )));
    let attr_nid = h
        .state
        .scene
        .as_ref()
        .unwrap()
        .find_entity_net_id_by_config_id(147_750_026)
        .expect("147750026 net id");
    let attr_pb = h
        .state
        .scene
        .as_ref()
        .unwrap()
        .entity(attr_nid)
        .unwrap()
        .to_proto(&h.state.assets, 0);
    assert!(attr_pb.ComponentPbs.iter().any(|c| matches!(
        &c.ComponentPb,
        Some(EntityComponentPbComponentPbOneof::AttributesIdsComponentPb(a)) if !a.PbSceneItemAttributeIds.is_empty()
    )));

    let tl_start = world_puzzle::handle_timeline_trace_start_request(
        &mut h.state,
        TimelineTraceStartRequest {
            EntityId: tl_nid,
            Index: 0,
        },
    )
    .expect("timeline start");
    assert_eq!(tl_start.Code, Some(ErrorCode::Success));
    let tl_ctrl = world_puzzle::handle_timeline_trace_control_request(
        &mut h.state,
        TimelineTraceControlRequest {
            EntityId: tl_nid,
            Forward: true,
        },
    )
    .expect("timeline forward");
    assert_eq!(tl_ctrl.Code, Some(ErrorCode::Success));
    assert_eq!(tl_ctrl.ControlPoint, 1);

    // 4) EntityRewardItemPb and BubbleComponent emit cleanly for TreasureBox and active play bubbles.
    {
        let mut sc = h.state.scene.take().unwrap();
        sc.add_play_bubble(102_002_186, "test-bubble-guid");
        entity_cmd::call(
            &mut events,
            &mut sc,
            &h.state.fs,
            &h.state.assets,
            &mut h.state.conn,
            &h.state.player_components.basic,
            102_002_186,
            None,
            None,
        )
        .expect("spawn chest 102002186");
        h.state.scene = Some(sc);
    }
    let box_nid = h
        .state
        .scene
        .as_ref()
        .unwrap()
        .find_entity_net_id_by_config_id(102_002_186)
        .expect("102002186 net id");
    let box_pb = h
        .state
        .scene
        .as_ref()
        .unwrap()
        .entity(box_nid)
        .unwrap()
        .to_proto(&h.state.assets, 0);
    assert!(box_pb.ComponentPbs.iter().any(|c| matches!(
        &c.ComponentPb,
        Some(EntityComponentPbComponentPbOneof::EntityRewardItemPb(r)) if r.HasCount == 1
    )));
    assert!(box_pb.ComponentPbs.iter().any(|c| matches!(
        &c.ComponentPb,
        Some(EntityComponentPbComponentPbOneof::BubbleComponent(b)) if !b.BubbleInfos.is_empty()
    )));

    // 5) RoadNetworkComponentPb emits on RoadNetworkNavigationComponent entity and updates on push.
    let mut h_road = TestHarness::new("audit_155_road", Some(100));
    {
        let mut sc = h_road.state.scene.take().unwrap();
        entity_cmd::call(
            &mut events,
            &mut sc,
            &h_road.state.fs,
            &h_road.state.assets,
            &mut h_road.state.conn,
            &h_road.state.player_components.basic,
            36_000_411,
            None,
            None,
        )
        .expect("spawn road nav entity 36000411");
        h_road.state.scene = Some(sc);
    }
    let road_nid = h_road
        .state
        .scene
        .as_ref()
        .unwrap()
        .find_entity_net_id_by_config_id(36_000_411)
        .expect("36000411 net id");
    world_puzzle::handle_road_network_entity_update_cur_road_push(
        &mut h_road.state,
        sonoro_proto::pb::RoadNetworkEntityUpdateCurRoadPush {
            EntityId: road_nid,
            CurRoadId: 12,
            CurRoadIndex: 3,
        },
    )
    .expect("road nav push");
    let road_pb = h_road
        .state
        .scene
        .as_ref()
        .unwrap()
        .entity(road_nid)
        .unwrap()
        .to_proto(&h_road.state.assets, 0);
    assert!(road_pb.ComponentPbs.iter().any(|c| matches!(
        &c.ComponentPb,
        Some(EntityComponentPbComponentPbOneof::RoadNetworkComponentPb(rn)) if rn.DestRoadId == 12 && rn.DestIndex == 3
    )));
}

#[test]
fn audit_156_live_store_pay_shop_gifts_and_topup() {
    let mut h = TestHarness::new("audit_156_store", Some(8));

    let shop_resp =
        shop::handle_pay_shop_info_request(&mut h.state, PayShopInfoRequest::default())
            .expect("pay_shop_info");
    assert_eq!(shop_resp.ErrorCode, Some(ErrorCode::Success));

    // 1. Recommended includes MonthCard (Id 1, RecommendType 1) & Feixue (Id 12, RecommendId 106100009),
    // with localized TabNames instead of raw PrefabTextItem_* keys.
    assert!(shop_resp
        .PayShopRecommendData
        .iter()
        .any(|r| r.Id == 1 && r.RecommendType == 1 && !r.PrefabPath.is_empty()));
    let feixue_rec = shop_resp
        .PayShopRecommendData
        .iter()
        .find(|r| r.Id == 12 && r.RecommendId == 106_100_009)
        .expect("Feixue skin recommend entry");
    assert_eq!(feixue_rec.TabImage, b"T_ShopTabRoleFeixue1");
    for rec in &shop_resp.PayShopRecommendData {
        let name = String::from_utf8_lossy(&rec.TabName);
        assert!(
            !name.starts_with("PrefabTextItem_"),
            "raw recommend key leaked: {name}"
        );
    }

    // 2. Subtab labels for core Store tabs are localized instead of raw PayShopTab_* keys.
    for tab in &shop_resp.PayShopTabData {
        if matches!(tab.ShopId, 1 | 3 | 4 | 5 | 6 | 7 | 100) {
            let name = String::from_utf8_lossy(&tab.name);
            assert!(
                !name.starts_with("PayShopTab_"),
                "raw PayShopTab key leaked for shop={} tab={}: {name}",
                tab.ShopId,
                tab.TabId
            );
        }
    }

    // 3. Shop 4 (Item Exchange), Shop 5 (Points Shop), and Shop 6 (Outfits tab 1) have live items.
    let shop4 = shop_resp
        .Infos
        .iter()
        .find(|s| s.Id == 4)
        .expect("Shop 4");
    assert!(shop4.Items.iter().any(|it| it.TabId == 1 && it.ItemId == 10_001_503));
    assert!(shop4.Items.iter().any(|it| it.TabId == 2 && it.ItemId == 43_010_003));
    assert!(shop4.Items.iter().any(|it| it.TabId == 3 && it.Id == 104_300_001));

    let shop5 = shop_resp
        .Infos
        .iter()
        .find(|s| s.Id == 5)
        .expect("Shop 5");
    assert!(shop5.Items.iter().any(|it| it.TabId == 1));
    assert!(shop5.Items.iter().any(|it| it.TabId == 2));
    assert!(shop5.Items.iter().any(|it| it.TabId == 3));

    let shop6 = shop_resp
        .Infos
        .iter()
        .find(|s| s.Id == 6)
        .expect("Shop 6");
    assert!(shop6
        .Items
        .iter()
        .any(|it| it.Id == 106_100_009 && it.ItemId == 53011 && it.TabId == 1));
    assert!(shop6
        .Items
        .iter()
        .all(|it| it.TabId == 1));

    // 4. Embedded PayGiftShopInfo and standalone PayGiftInfoRequest return valid ItemId / ShopId / numeric Amount.
    let gift_shop = shop_resp
        .PayGiftShopInfo
        .as_ref()
        .expect("PayGiftShopInfo must be embedded in PayShopInfoResponse");
    assert!(!gift_shop.Gifts.is_empty());

    let gift_resp =
        qol::handle_pay_gift_info_request(&mut h.state, PayGiftInfoRequest::default())
            .expect("pay_gift_info");
    assert_eq!(gift_resp.ErrorCode, Some(ErrorCode::Success));
    assert_eq!(gift_resp.Infos.len(), gift_shop.Gifts.len());
    for g in &gift_resp.Infos {
        assert!(g.ItemId > 0, "PayGift {} must have ItemId > 0", g.Id);
        assert!(
            matches!(g.ShopId, 0 | 3 | 7),
            "PayGift {} has invalid ShopId {}",
            g.Id,
            g.ShopId
        );
        let amt = String::from_utf8_lossy(&g.Amount);
        assert!(!amt.starts_with('$'), "PayGift {} amount must be numeric: {amt}", g.Id);
        assert!(!g.ProductId.is_empty());
    }
    assert!(gift_resp.Infos.iter().any(|g| g.Id == 42 && g.ItemId == 50011 && g.Type == 1));
    assert!(gift_resp
        .Infos
        .iter()
        .any(|g| g.Id == 105_100_101 && g.ShopId == 0 && g.TabId == 2));

    // 5. Purchase (PayInfoRequest) returns the 6 standard packs with numeric USD amounts.
    let pay_info =
        minimal::handle_pay_info_request(&mut h.state, PayInfoRequest::default())
            .expect("pay_info");
    assert_eq!(pay_info.ErrorCode, Some(ErrorCode::Success));
    assert_eq!(pay_info.Infos.len(), 6);
    let expected_amounts = ["0.99", "4.99", "14.99", "29.99", "49.99", "99.99"];
    for (idx, (info, expected_amt)) in pay_info
        .Infos
        .iter()
        .zip(expected_amounts.iter())
        .enumerate()
    {
        assert_eq!(info.Id, (idx + 1) as i32);
        assert_eq!(String::from_utf8_lossy(&info.Amount), *expected_amt);
    }
}

#[test]
fn audit_157_gm_panel_webview_close_bridge() {
    let html = include_str!("../network/gm_panel.html");
    assert!(html.contains("id=\"closeBtn\" type=\"button\" onclick=\"closeWebView()\""));
    assert!(html.contains("window[handlerName]()"));
    assert!(html.contains("window.WebViewJavascriptBridge"));
    assert!(html.contains("WebViewJavascriptBridgeReady"));
    assert!(html.contains("https://__bridge_loaded__"));
    assert!(html.contains("invokeKuroBridge('close_webview')"));
    assert!(html.contains("invokeKuroBridge('loading_success')"));
    assert!(!html.contains("window.location.href = 'kuro://close'"));
}

#[test]
fn audit_158_role_trial_and_consumptive_asset_exclusions() {
    let h = TestHarness::new("audit_158_act_excl", Some(8));
    // Type 51 ConsumptiveActivity (105100001 / 105100002) uses stripped UiItem_CumulativeShop prefab.
    assert!(!activity::is_activity_unlocked(
        None,
        &h.state.assets,
        105_100_001
    ));
    assert!(!activity::is_activity_unlocked(
        None,
        &h.state.assets,
        105_100_002
    ));
    // Type 21 TowerGuideNew (100300002) remains unlocked.
    assert!(activity::is_activity_unlocked(
        None,
        &h.state.assets,
        100_300_002
    ));
    // Legacy RoleTrial activities 100900001..=100900006 reference RoleTrialInfo Id 1 (missing).
    for legacy_id in 100_900_001..=100_900_006 {
        assert!(
            !activity::is_activity_unlocked(None, &h.state.assets, legacy_id),
            "legacy RoleTrial {legacy_id} must be excluded"
        );
    }
    // Live RoleTrial activities (e.g. 100900007) have valid RoleTrialInfo entries and stay unlocked.
    assert!(activity::is_activity_unlocked(
        None,
        &h.state.assets,
        100_900_007
    ));

    let list = activity::build_activity_list(
        &h.state.assets,
        &h.state.player_components.progress,
        None,
    );
    assert!(!list.iter().any(|a| matches!(a.Id, 105_100_001 | 105_100_002)));
    assert!(list.iter().any(|a| a.Id == 100_300_002));
    assert!(
        !list
            .iter()
            .any(|a| (100_900_001..=100_900_006).contains(&a.Id))
    );
    assert!(list.iter().any(|a| a.Id == 100_900_007));
}

#[test]
fn audit_159_week_card_unbought_and_millisecond_purchase_lifecycle() {
    use sonoro_proto::pb::{EffectiveWeekCardInfoNotify, PayGiftRequest};

    let mut h = TestHarness::new("audit_159_week_card", Some(8));

    // Unbought WeekCards must report BuyTimeStamp/EndTimeStamp == 0 so NewPlayerShop stays open via config.
    let unbought = qol::handle_week_card_info_request(
        &mut h.state,
        WeekCardInfoRequest {
            WeekCardIds: vec![1, 2],
            ..Default::default()
        },
    )
    .expect("unbought week card info");
    assert_eq!(unbought.WeekCardInfos.len(), 2);
    for wc in &unbought.WeekCardInfos {
        assert_eq!(wc.Days, 15);
        assert_eq!(wc.BuyTimeStamp, 0);
        assert_eq!(wc.EndTimeStamp, 0);
        assert_eq!(wc.WeekCardContentInfos.len(), 3);
        for c in &wc.WeekCardContentInfos {
            assert_eq!(c.Status, 0);
            assert_eq!(c.UnlockTimeStamp, 0);
            assert!(!c.RewardContent.is_empty());
        }
    }

    // Purchasing GiftId 48 activates WeekCard 2 with Unix millisecond timestamps and notifies the client.
    let buy_resp = qol::handle_pay_gift_request(
        &mut h.state,
        PayGiftRequest {
            Id: 48,
            ..Default::default()
        },
    )
    .expect("buy week card gift 48");
    assert_eq!(buy_resp.ErrorCode, Some(ErrorCode::Success));

    let notifs: Vec<EffectiveWeekCardInfoNotify> =
        h.drain_decoded(EffectiveWeekCardInfoNotify::MSG_ID);
    assert_eq!(notifs.len(), 1);
    let bought_card = notifs[0]
        .WeekCardInfos
        .iter()
        .find(|w| w.WeekCardId == 2)
        .expect("notified week card 2");
    assert_eq!(bought_card.Days, 15);
    assert!(bought_card.BuyTimeStamp > 1_000_000_000_000);
    assert_eq!(
        bought_card.EndTimeStamp - bought_card.BuyTimeStamp,
        15 * 86_400_000
    );
    assert_eq!(bought_card.WeekCardContentInfos[0].Status, 1);
    assert_eq!(bought_card.WeekCardContentInfos[1].Status, 0);

    let after = qol::handle_week_card_info_request(
        &mut h.state,
        WeekCardInfoRequest {
            WeekCardIds: vec![1, 2],
            ..Default::default()
        },
    )
    .expect("week card info after buy");
    assert_eq!(after.WeekCardInfos[0].EndTimeStamp, 0);
    assert_eq!(after.WeekCardInfos[1].EndTimeStamp, bought_card.EndTimeStamp);
}

#[test]
fn audit_160_level_play_cross_map_cleanup_and_kill_progress_guard() {
    use sonoro_proto::pb::{LeaveLevelPlayNotify, UpdateNodeProgressNotify};

    let mut h = TestHarness::new("audit_160_lp_cleanup", Some(68));
    // Simulate Map 8 Challenge 109000005 (entity 109005370.bytes) still marked entered when entering Map 68.
    h.state
        .player_components
        .quest
        .info
        .progress
        .entered_level_plays
        .push(109_000_005);
    h.drain_count();

    level_play::leave_off_map_level_plays(
        &h.state.assets,
        &h.state.fs,
        &mut h.state.conn,
        h.state.scene.as_mut(),
        &mut h.state.player_components.quest,
        68,
    )
    .expect("leave off-map level plays");

    let left: Vec<LeaveLevelPlayNotify> = h.drain_decoded(LeaveLevelPlayNotify::MSG_ID);
    assert!(
        left.iter().any(|n| n.Id == 109_000_005),
        "cross-map transition to map 68 must leave map 8 LevelPlay 109000005: {left:?}"
    );
    assert!(
        !h.state
            .player_components
            .quest
            .info
            .progress
            .entered_level_plays
            .contains(&109_000_005)
    );

    // On Map 1505, killing a monster referenced by LevelPlay 610800000 must NOT emit UpdateNodeProgressNotify
    // when 610800000's behavior tree has never been sent to the client.
    let mut h2 = TestHarness::new("audit_160_bt_guard", Some(1505));
    let mut events = EventQueue::new();
    entity_cmd::spawn_with_location(
        &mut events,
        h2.state.scene.as_mut().expect("scene"),
        &h2.state.fs,
        &h2.state.assets,
        &mut h2.state.conn,
        &h2.state.player_components.basic,
        139_800_000,
        Some(false),
        Some(false),
        Some(1505),
        None,
        true,
    )
    .expect("spawn ref monster");
    let monster_net_id = h2
        .state
        .scene
        .as_ref()
        .expect("scene")
        .find_entity_net_id_by_config_id(139_800_000)
        .expect("monster net_id");
    h2.drain_count();

    {
        let mut qctx = h2.state.quest_context(&mut events);
        let mut lp_ctx =
            level_play::LevelPlayContext::from_quest_ctx(&mut qctx).expect("lp ctx");
        level_play::on_entity_killed(&mut lp_ctx, monster_net_id, Some(139_800_000))
            .expect("on_entity_killed");
    }

    let updates: Vec<UpdateNodeProgressNotify> =
        h2.drain_decoded(UpdateNodeProgressNotify::MSG_ID);
    assert!(
        !updates.iter().any(|u| u.TreeOwnerId == 610_800_000),
        "must not push UpdateNodeProgressNotify before BehaviorTree 610800000 exists on client: {updates:?}"
    );
}

#[test]
fn audit_161_elevator_after_join_skips_same_floor_move_notify() {
    use sonoro_gamesv_logic::component::entity::small::LiftComponent;
    use sonoro_proto::pb::ElevatorMoveNotify;

    let mut h = TestHarness::new("audit_161_elevator_nan", Some(8));
    let mut events = EventQueue::new();
    entity_cmd::spawn_with_location(
        &mut events,
        h.state.scene.as_mut().expect("scene"),
        &h.state.fs,
        &h.state.assets,
        &mut h.state.conn,
        &h.state.player_components.basic,
        109_002_461,
        Some(false),
        Some(false),
        Some(8),
        None,
        true,
    )
    .expect("spawn elevator");
    let lift_net_id = h
        .state
        .scene
        .as_ref()
        .expect("scene")
        .find_entity_net_id_by_config_id(109_002_461)
        .expect("elevator net_id");

    h.state
        .player_components
        .progress
        .info
        .set_key_int("elevator:109002461", 2);
    if let Some(storage) = h
        .state
        .scene
        .as_mut()
        .expect("scene")
        .entity_mut(lift_net_id)
    {
        storage.world_item.lift = Some(LiftComponent { location: 2 });
    }
    h.drain_count();

    let fs_c = Arc::clone(&h.state.fs);
    {
        let mut qctx = h.state.quest_context(&mut events);
        let mut dummy_ev = EventQueue::new();
        scene_logic::after_scene_join(None, &mut dummy_ev, &fs_c, &mut qctx, 1)
            .expect("after_scene_join at floor 2");
    }
    let same_floor: Vec<ElevatorMoveNotify> = h.drain_decoded(ElevatorMoveNotify::MSG_ID);
    assert!(
        !same_floor.iter().any(|m| m.EntityId == lift_net_id),
        "must skip redundant same-floor ElevatorMoveNotify on floor 2: {same_floor:?}"
    );

    if let Some(storage) = h
        .state
        .scene
        .as_mut()
        .expect("scene")
        .entity_mut(lift_net_id)
    {
        storage.world_item.lift = Some(LiftComponent { location: 1 });
    }
    h.drain_count();

    {
        let mut qctx = h.state.quest_context(&mut events);
        let mut dummy_ev = EventQueue::new();
        scene_logic::after_scene_join(None, &mut dummy_ev, &fs_c, &mut qctx, 1)
            .expect("after_scene_join from floor 1 to 2");
    }
    let moved: Vec<ElevatorMoveNotify> = h.drain_decoded(ElevatorMoveNotify::MSG_ID);
    assert!(
        moved
            .iter()
            .any(|m| m.EntityId == lift_net_id && m.Location == 2),
        "must sync ElevatorMoveNotify when elevator is still at floor 1: {moved:?}"
    );
}

#[test]
fn audit_quest_tree_order_phone_message_and_empty_save_block_recovery() {
    let mut h = TestHarness::new("audit_quest_tree_phone_save", Some(8));
    let mut events = EventQueue::new();

    // 1) Duplicate QuestTreeNode wrapper (110070 vs 112000) still allows 125000120 after 880000013.
    {
        let qinfo = &mut h.state.player_components.quest.info;
        qinfo.finish_quest(quest_engine::PROLOGUE_QUEST_ID);
        qinfo.finish_quest(880_000_013);
    }
    quest_engine::tree::refresh_available_quests(&mut h.state.quest_context(&mut events), false)
        .expect("refresh after 880000013");
    assert!(
        h.state
            .player_components
            .quest
            .info
            .has_active_quest(125_000_120),
        "125000120 must still unlock via TN 110070 despite TN 112000 duplicate wrapper"
    );

    // 2) Finishing 114000026 must NOT early-unlock Ch.2 Epilogue 880000044 (PreNode 212000),
    // while finishing 114000029 unlocks Ch.2 Act IV 145000005.
    for q in [114_000_026, 114_000_027, 114_000_028, 114_000_030, 140_000_011] {
        h.state.player_components.quest.info.finish_quest(q);
    }
    quest_engine::tree::refresh_available_quests(&mut h.state.quest_context(&mut events), false)
        .expect("refresh before 114000029");
    assert!(
        !h.state
            .player_components
            .quest
            .info
            .has_active_quest(880_000_044),
        "880000044 must not auto-unlock before PreNode 212000 is finished"
    );

    h.state
        .player_components
        .quest
        .info
        .finish_quest(114_000_029);
    quest_engine::tree::refresh_available_quests(&mut h.state.quest_context(&mut events), false)
        .expect("refresh after 114000029");
    assert!(
        h.state
            .player_components
            .quest
            .info
            .has_active_quest(145_000_005),
        "145000005 (Ch.2 Act IV) must auto-unlock after 114000029"
    );
    assert!(
        !h.state
            .player_components
            .quest
            .info
            .has_active_quest(880_000_044),
        "880000044 must remain locked until Ch.2 Act XII (175000000) finishes"
    );

    // 3) Reading unrelated phone message 37021 must NOT activate Ch.3 PhoneMessage quests
    // (196700000..196700002, 146700000); reading 30015 unlocks only 196700000.
    quest_engine::events::on_phone_message_read(&mut h.state.quest_context(&mut events), 37021)
        .expect("unrelated phone read");
    for q in [196_700_000, 196_700_001, 196_700_002, 146_700_000] {
        assert!(
            !h.state.player_components.quest.info.has_active_quest(q),
            "unrelated SMS 37021 must not activate PhoneMessage quest {q}"
        );
    }
    quest_engine::events::on_phone_message_read(&mut h.state.quest_context(&mut events), 30015)
        .expect("linked phone read 30015");
    assert!(
        h.state
            .player_components
            .quest
            .info
            .has_active_quest(196_700_000),
        "linked SMS 30015 should activate 196700000"
    );
    assert!(
        !h.state
            .player_components
            .quest
            .info
            .has_active_quest(196_700_001)
    );

    // 4) Empty active_nodes on an active SaveConfig quest (145000005 after node 24) recovers on login + SceneLoadingFinish.
    {
        let aq = h
            .state
            .player_components
            .quest
            .info
            .get_or_add_active_quest(145_000_005);
        aq.active_nodes.clear();
        aq.finished_nodes = vec![12, 20, 22, 319, 614, 13, 23, 405, 191, 190, 36, 24];
    }
    quest_engine::sync_on_login(&mut h.state.quest_context(&mut events)).expect("sync_on_login");
    assert!(
        h.state
            .player_components
            .quest
            .login_restore
            .reenter_quests
            .contains(&145_000_005),
        "empty active_nodes quest 145000005 must queue tree re-entry on login"
    );
    quest_engine::on_scene_loading_finish(&mut h.state.quest_context(&mut events))
        .expect("on_scene_loading_finish");
    let aq_after = h
        .state
        .player_components
        .quest
        .info
        .find_active_quest(145_000_005)
        .expect("145000005 active after recovery");
    assert!(
        !aq_after.active_nodes.is_empty(),
        "145000005 active_nodes must recover non-empty after SceneLoadingFinish: {aq_after:?}"
    );
}

#[test]
fn audit_225_event_slot_dreamlink_self_var_fade_and_cross_map_node() {
    use sonoro_proto::pb::{ActivityDataDataOneof, LevelEventNotify};

    let mut h = TestHarness::new("audit_225_event_fade", Some(8));
    let mut events = EventQueue::new();

    // 1) DreamLinkWorldRun Index must match table Id (1..=N, never 0), and Abyss roles have 9 plugin slots.
    let acts = activity::build_activity_list(
        &h.state.assets,
        &h.state.player_components.progress,
        None,
    );
    let dl = acts
        .iter()
        .find_map(|a| match &a.Data {
            Some(ActivityDataDataOneof::DreamLinkActivityData(d)) => Some(d),
            _ => None,
        })
        .expect("DreamLinkActivityData present");
    assert!(!dl.LevelPlayList.is_empty());
    assert!(
        dl.LevelPlayList
            .iter()
            .all(|lp| lp.Index > 0 && lp.Index == lp.LevelPlayId),
        "DreamLink LevelPlayList.Index must match DreamLinkWorldRun.Id: {:?}",
        dl.LevelPlayList
    );

    let abyss = acts
        .iter()
        .find_map(|a| match &a.Data {
            Some(ActivityDataDataOneof::DangoAbyssActivityData(a)) => Some(a),
            _ => None,
        })
        .expect("DangoAbyssActivityData present");
    assert!(!abyss.RoleList.is_empty());
    assert!(
        abyss.RoleList.iter().all(|r| r.EquipItems.len() == 9),
        "Abyss role EquipItems must always contain 9 slots: {:?}",
        abyss.RoleList
    );

    // 2) LevelPlay 145750001 real ride: node 2 -> node 408 -> Wait(0.2) -> SetVar(Self."白屏"=1) -> node 409 CompareVar completes.
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
        2_838.75,
    )
    .expect("approach gondola LP");
    let train_net_id = h
        .state
        .scene
        .as_ref()
        .unwrap()
        .find_entity_net_id_by_config_id(145_750_138)
        .expect("train net id");
    let _ = crate::network::handlers::scene::handle_entity_dynamic_interact_request(
        &mut h.state,
        &mut events,
        sonoro_proto::pb::EntityDynamicInteractRequest {
            EntityId: train_net_id,
            OptionGuid: b"4cae376712984b8095a277b04d5d0a26".to_vec(),
        },
    )
    .expect("board gondola");
    let t0 = quest_engine::tree::now_ms_pub();
    quest_engine::tree::on_timers_due(&mut h.state.quest_context(&mut events), t0 + 600)
        .expect("flush node 2 Wait(0.5)");
    let _ = h.drain_count();

    quest_engine::events::on_player_move(
        &mut h.state.quest_context(&mut events),
        874_590.94,
        708_230.56,
        2_779.5,
    )
    .expect("reach node 408");
    assert_eq!(
        h.state
            .player_components
            .quest
            .info
            .get_var_int("Self:145750001:白屏"),
        1
    );
    assert!(
        !level_play::is_lp_node_active_pub(
            Some(&h.state.player_components.progress),
            145_750_001,
            408,
        ) && !level_play::is_lp_node_active_pub(
            Some(&h.state.player_components.progress),
            145_750_001,
            409,
        ) && h.state.player_components.progress.info.get_key_int("lp_node:145750001:409") == 1,
        "node 408 (SetVar Self.白屏=1) must allow node 409 (FadeOutScreen) to finish: dynamic={:?}",
        h.state.player_components.progress.info.dynamic.ints
    );
    let fade_out_events: Vec<LevelEventNotify> = h.drain_decoded(LevelEventNotify::MSG_ID);
    assert!(
        fade_out_events.iter().any(|ev| ev.IncId == 145_750_001),
        "node 408/409 completion must push FadeIn/FadeOut LevelEventNotify for 145750001"
    );

    // Also verify CompareVar Self-scope gating directly (0 -> false, 1 -> true).
    h.state
        .player_components
        .quest
        .info
        .set_var_int("Self:145750001:白屏", 0);
    let n409 = h
        .state
        .assets
        .level_play_nodes
        .get_owner_nodes(145_750_001)
        .expect("145750001 nodes")
        .iter()
        .find(|n| n.id == 409)
        .cloned()
        .expect("node 409");
    let c409 = quest_engine::get_child_condition_pub(&n409.data)
        .expect("node 409 cond")
        .clone();
    assert!(
        !quest_engine::triggers::eval_compare_var_list(
            &h.state.quest_context(&mut events),
            145_750_001,
            &c409,
        ),
        "CompareVar(Self.白屏 == 1) must be false when Self:145750001:白屏 == 0"
    );
    h.state
        .player_components
        .quest
        .info
        .set_var_int("Self:145750001:白屏", 1);
    assert!(
        quest_engine::triggers::eval_compare_var_list(
            &h.state.quest_context(&mut events),
            145_750_001,
            &c409,
        ),
        "CompareVar(Self.白屏 == 1) must evaluate true with Self:145750001:白屏 == 1"
    );

    // 3) Finished ChildQuest node 42 (entity 145000008, DungeonId 904) must be omitted on Map 8.
    let aq_snapshot = {
        let aq = h
            .state
            .player_components
            .quest
            .info
            .get_or_add_active_quest(145_000_005);
        aq.active_nodes = vec![454];
        aq.finished_nodes = vec![12, 24, 361, 42];
        aq.clone()
    };
    let tree_map8 = quest_engine::triggers::build_tree_info(
        &h.state.quest_context(&mut events),
        &aq_snapshot,
    );
    assert!(
        !tree_map8.Nodes.contains_key(&42),
        "finished cross-dungeon DoInteract node 42 must be omitted on Map 8"
    );
    h.state.scene = Some(
        sonoro_gamesv_logic::scene::Scene::init(&h.state.fs, 1, 904).expect("scene 904"),
    );
    let tree_map904 = quest_engine::triggers::build_tree_info(
        &h.state.quest_context(&mut events),
        &aq_snapshot,
    );
    assert!(
        tree_map904.Nodes.contains_key(&42),
        "node 42 should still appear inside its matching Dungeon 904"
    );
}

#[test]
fn audit_226_keyword_entself_system_string_copy_and_lp_reach_pre_context() {
    use serde_json::json;

    let mut h = TestHarness::new("audit_226_var_fixes", Some(8));
    let mut events = EventQueue::new();

    // 1) "Keyword" + persisted progress_comp Global vars (e.g. Ch.2 Act V 165000003 node 97).
    let nodes_165 = h
        .state
        .assets
        .quest_nodes
        .get_owner_nodes(165_000_003)
        .expect("165000003 nodes");
    let n97 = nodes_165
        .iter()
        .find(|n| n.id == 97)
        .cloned()
        .expect("165000003 node 97");
    assert!(
        !quest_engine::eval_child_condition_auto(&h.state.quest_context(&mut events), 165_000_003, &n97),
        "165000003 node 97 must be false before 夏空线海边变量1 is set"
    );
    h.state
        .player_components
        .progress
        .info
        .set_key_int("var:夏空线海边变量1", 1);
    assert!(
        quest_engine::eval_child_condition_auto(&h.state.quest_context(&mut events), 165_000_003, &n97),
        "165000003 node 97 must resolve Keyword + Global progress_comp var:夏空线海边变量1"
    );
    h.state
        .player_components
        .progress
        .info
        .set_key_str("var:kw_str_key", "ok");
    let kw_str_cond = json!({
        "Type": "CompareVar",
        "Var1": { "Type": "String", "Source": "Global", "Keyword": "kw_str_key" },
        "Compare": "Eq",
        "Var2": { "Type": "String", "Source": "Constant", "Value": "ok" }
    });
    assert!(
        eval_single_inline_condition(&h.state.quest_context(&mut events).make_eval_context(), &kw_str_cond),
        "String CompareVar must resolve Keyword + Global progress_comp var"
    );

    // 2) EntSelf / self_entity_config_id scoped Self variable lookup.
    let cfg_id = 145_750_138i32;
    h.state
        .player_components
        .quest
        .info
        .set_var_int(&format!("Entity:{cfg_id}:ent_flag"), 7);
    let ent_cond = json!({
        "Type": "CompareVar",
        "Var1": { "Type": "Int", "Source": "Self", "Name": "ent_flag" },
        "Compare": "Eq",
        "Var2": { "Type": "Int", "Source": "Constant", "Value": 7 }
    });
    let qctx = h.state.quest_context(&mut events);
    let mut ent_eval = qctx.make_eval_context();
    ent_eval.self_entity_config_id = cfg_id;
    assert!(
        eval_single_inline_condition(&ent_eval, &ent_cond),
        "Entity trigger CompareVar with Source=Self must resolve self_entity_config_id"
    );

    // 3) apply_set_var copying a Self String variable preserves owner_id scope.
    let mut cbs = ActionCallbacks::default();
    execute_action_list(
        &mut h.state.quest_context(&mut events),
        &mut cbs,
        145_750_001,
        1,
        &[
            json!({
                "Name": "SetVar",
                "Params": {
                    "VarLeft": { "Type": "String", "Source": "Self", "Name": "src_text" },
                    "VarRight": { "Type": "String", "Source": "Constant", "Value": "avinoleum" }
                }
            }),
            json!({
                "Name": "SetVar",
                "Params": {
                    "VarLeft": { "Type": "String", "Source": "Self", "Name": "dst_text" },
                    "VarRight": { "Type": "String", "Source": "Self", "Name": "src_text" }
                }
            }),
        ],
    )
    .expect("execute Self String SetVar copy");
    assert_eq!(
        h.state
            .player_components
            .quest
            .info
            .get_var_str("Self:145750001:dst_text"),
        "avinoleum"
    );

    // 4) System variables: QuestBranchPageId + CommonHostToTargetDistance (SetVar & CompareVar).
    h.state
        .player_components
        .quest
        .info
        .select_quest_branch(1000, 2);
    let nodes_ep = h
        .state
        .assets
        .quest_nodes
        .get_owner_nodes(880_000_045)
        .expect("880000045 nodes");
    let n29 = nodes_ep
        .iter()
        .find(|n| n.id == 29)
        .cloned()
        .expect("880000045 node 29");
    assert!(
        quest_engine::eval_child_condition_auto(&h.state.quest_context(&mut events), 880_000_045, &n29),
        "880000045 node 29 QuestBranchPageId==2 must evaluate true"
    );
    h.state
        .scene
        .as_mut()
        .unwrap()
        .instance
        .players
        .push(ScenePlayer {
            id: 1,
            group_type: 1,
            location: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
        });
    execute_action_list(
        &mut h.state.quest_context(&mut events),
        &mut cbs,
        938_750_003,
        74,
        &[json!({
            "Name": "SetVar",
            "Params": {
                "VarLeft": { "Type": "Int", "Source": "Self", "Name": "dist" },
                "VarRight": {
                    "Type": "Int",
                    "Source": "System",
                    "Var": {
                        "Type": "CommonHostToTargetDistance",
                        "Pos": { "X": 300.0, "Y": 400.0, "Z": 9999.0 },
                        "DistanceMeasureMode": "Horizontal"
                    }
                }
            }
        })],
    )
    .expect("execute System CommonHostToTargetDistance SetVar");
    assert_eq!(
        h.state
            .player_components
            .quest
            .info
            .get_var_int("Self:938750003:dist"),
        500,
        "System CommonHostToTargetDistance Horizontal from (0,0) to (300,400) must equal 500"
    );

    // 5) LevelPlayContext::make_eval_context_for_lp evaluates Self vars with owner_quest_id = lp_id.
    h.state
        .player_components
        .quest
        .info
        .set_var_int("Self:170700000:lp_pre_ok", 1);
    let lp_pre_cond = json!({
        "Type": "CompareVar",
        "Var1": { "Type": "Int", "Source": "Self", "Name": "lp_pre_ok" },
        "Compare": "Eq",
        "Var2": { "Type": "Int", "Source": "Constant", "Value": 1 }
    });
    let mut qctx = h.state.quest_context(&mut events);
    let lp_ctx = level_play::LevelPlayContext::from_quest_ctx(&mut qctx).unwrap();
    assert!(!eval_single_inline_condition(&lp_ctx.make_eval_context(), &lp_pre_cond));
    assert!(eval_single_inline_condition(&lp_ctx.make_eval_context_for_lp(170_700_000), &lp_pre_cond));
}


