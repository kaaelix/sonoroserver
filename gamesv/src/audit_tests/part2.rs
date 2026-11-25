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
    AccessRangeType, EAttributeType, EClientStorageSystemIdType, EDamageImmune, ESummonType,
    ErrorCode, GravityFlipType, RangeType,
};
use sonoro_proto::pb::{
    AbyssRankListRequest, AchievementInfoRequest, AfterJoinSceneNotify, BabelTowerRankListRequest,
    BeControlledRequest, BeControlledShowRequest, BehaviorTreeInfoNotify,
    BossRushSettlementRequest, BuffItemRequest, ChangeEntityStateRequest, ChannelChatRequest,
    CharacterDeathPush, ClientStorageInfo, ClientStorageInfoDataOneof, ClientStorageMapData,
    ClientStorageMapMapData, CombatNotifyDataMessageOneof, CombatReceiveData,
    CombatReceiveDataMessageOneof, DamageExecuteRequest, EntityAccessRangeRequest,
    EntityAddNotify, EntityComponentPbComponentPbOneof, EntityPatrolChangeDirRequest,
    EntityPositionRequest, FlowStartNotify, GachaInfoRequest, GachaPoolDetailRequest,
    GravityFlipInteractRequest, GravityFlipUpdateTypeNotify, HoldHandRequest, HookLockPointRequest,
    ItemDisuseRequest, ItemExchangeInfoRequest, ItemRewardNotify, JoinSceneNotify,
    KurotatoSyncEntityDealDamagePush, LandingDamageRequest, LowMemoryChangeSceneFinishRequest,
    LowMemoryChangeSceneStartNotify, MonsterDetectionInfoRequest, MonthCardRequest,
    MotorCycleCruiseStatePush, MotorFightRankListRequest, MotorSummonAndRidePush,
    MovePlacementNotify, NewTowerRecommendRequest, OperationRestrictionNotify, PayInfoRequest,
    PayShopInfoRequest, PhantomBattleCallCardRequest, PhantomBattleStartRequest,
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
    assert!(h.drain_push_msg_ids().contains(&MovePlacementNotify::MSG_ID));

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
        ImmuneType: 1,
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

    let spawner_cfg = LevelEntityConfig {
        Id: 900_003,
        EntityId: 900_003,
        BlueprintType: "Gameplay581".to_string(),
        MapId: 8,
        AreaId: 1,
        ..Default::default()
    };
    assert!(scene_logic::is_blocked_world_entity_config(
        &h.state.assets,
        &h.state.player_components.quest.info,
        8,
        &spawner_cfg,
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
    assert!(!is_spawnable_item_config(&var_mgr, BpEntityType::Monitor));
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
    let mark_notify = misc_logic::build_map_mark_info_notify(
        &h.state.assets,
        &h.state.player_components.map,
    );
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

    // 2. Chest Drops & DropShowPlan: chest drop 1001 grants real rewards with ShowPlanId > 0 and Reason 7000.
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
    assert_eq!(notifies[0].Reason, 7000);
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

        // 3. Evaluate remaining inline Condition variants directly.
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
