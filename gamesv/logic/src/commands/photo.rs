use std::collections::BTreeMap;

use sonoro_proto::enums::{BtType, ChildQuestNodeStatus, GameCtxType, NodeStatus};
use sonoro_proto::pb::{
    BehaviorTreeCtxPb, BehaviorTreeDeleteNotify, BehaviorTreeInfoNotify,
    ChildQuestNodeEnterActionCtxPb, CreateInstanceDungeonNotify, FightPhotoLevelDataUpdateNotify,
    GameCtxPb, GameCtxPbCtxInfoOneof, LevelData, LevelEventNotify, TreeInfo,
    UpdateChildQuestNodeStatusNotify, UpdateNodeStatusNotify,
};

use crate::commands::respond;
use crate::event_queue::EventQueue;
use crate::scene::Scene;
use crate::sink::PushSink;

pub const ALIAS: &str = "cam";
pub const DESCRIPTION: &str = "puts your client into 2.7 photo event mode.\nusage: photo [enabled] \nenabled by default is 1 (true)";

pub fn call<S: PushSink + ?Sized>(
    events: &mut EventQueue,
    scene: &Scene,
    sink: &mut S,
    mode: Option<bool>,
) {
    let player_id = scene.player_id;
    if mode.unwrap_or(true) {
        let mut roles = Vec::new();
        if let Ok(idx) = usize::try_from(scene.formation_info.cur_formation)
            && let Some(formation) = scene.formation_info.formations.get(idx)
        {
            for role in formation.roles.iter().flatten() {
                roles.push(role.role_id);
            }
        }

        let fight_photo_notify = FightPhotoLevelDataUpdateNotify {
            levels: vec![LevelData {
                LevelId: 100016,
                InstId: 8,
                Roles: roles,
                GroupId: 1,
                IsUnlocked: true,
            }],
        };

        let instance_dungeon_notify = CreateInstanceDungeonNotify {
            LevelPlayId: 314800019,
        };

        let behavior_tree_info_notify = BehaviorTreeInfoNotify {
            TreeInfos: vec![TreeInfo {
                BlackboardId: 314800019,
                BtType: Some(BtType::BtTypeInst),
                TreeIncId: 69,
                TreeOwnerId: player_id,
                Nodes: BTreeMap::new(),
                Vars: BTreeMap::new(),
                TimerInfos: Vec::new(),
                SuspendType: 0,
                ..Default::default()
            }],
        };

        let update_node_status_notify = UpdateNodeStatusNotify {
            TreeOwnerId: player_id,
            TreeIncId: 69,
            NodeId: 13,
            Status: Some(NodeStatus::Activate),
        };

        let child_quest_notify_1 = UpdateChildQuestNodeStatusNotify {
            TreeOwnerId: player_id,
            TreeIncId: 69,
            NodeId: 13,
            Status: Some(ChildQuestNodeStatus::Enter),
        };

        let child_quest_notify_2 = UpdateChildQuestNodeStatusNotify {
            TreeOwnerId: player_id,
            TreeIncId: 69,
            NodeId: 13,
            Status: Some(ChildQuestNodeStatus::EnterAction),
        };

        let level_event_notify = LevelEventNotify {
            PlayerId: player_id,
            IncId: 69,
            GameCtx: Some(GameCtxPb {
                CtxType: Some(GameCtxType::ChildQuestNodeEnterAction),
                CtxInfo: Some(GameCtxPbCtxInfoOneof::ChildQuestNodeEnterAction(
                    ChildQuestNodeEnterActionCtxPb {
                        BehaviorTreeCtx: Some(BehaviorTreeCtxPb {
                            IncId: 69,
                            BtType: Some(BtType::BtTypeInst),
                            BtId: 314800019,
                            NodeId: 13,
                        }),
                    },
                )),
            }),
            TotalCount: 1,
            ..Default::default()
        };

        let child_quest_notify_3 = UpdateChildQuestNodeStatusNotify {
            TreeOwnerId: player_id,
            TreeIncId: 69,
            NodeId: 13,
            Status: Some(ChildQuestNodeStatus::Progress),
        };

        sink.push(FightPhotoLevelDataUpdateNotify::MSG_ID, &fight_photo_notify);
        sink.push(CreateInstanceDungeonNotify::MSG_ID, &instance_dungeon_notify);
        sink.push(BehaviorTreeInfoNotify::MSG_ID, &behavior_tree_info_notify);
        sink.push(UpdateNodeStatusNotify::MSG_ID, &update_node_status_notify);
        sink.push(
            UpdateChildQuestNodeStatusNotify::MSG_ID,
            &child_quest_notify_1,
        );
        sink.push(
            UpdateChildQuestNodeStatusNotify::MSG_ID,
            &child_quest_notify_2,
        );
        sink.push(LevelEventNotify::MSG_ID, &level_event_notify);
        sink.push(
            UpdateChildQuestNodeStatusNotify::MSG_ID,
            &child_quest_notify_3,
        );

        respond(events, player_id, "photo event enabled.");
    } else {
        sink.push(
            BehaviorTreeDeleteNotify::MSG_ID,
            &BehaviorTreeDeleteNotify {
                TreeIncIds: vec![69],
            },
        );
        respond(events, player_id, "photo event disabled.");
    }
}
