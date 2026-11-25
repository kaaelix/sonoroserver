use sonoro_common::fs::FileSystem;
use sonoro_gamesv_data::Assets;
use sonoro_gamesv_data::asset_indexes;
use sonoro_proto::enums::{ChildQuestNodeStatus, NodeStatus, QuestFocusReason, QuestState};
use sonoro_proto::pb::{
    FlowStartNotify, FocusQuestSetNotify, FuncOpenNotify, Function, QuestFinishListNotify,
    QuestInfo, QuestListNotify, QuestShowListNotify, TeleportNotify, TraceQuestNotify,
    UpdateChildQuestNodeStatusNotify, UpdateNodeStatusNotify, Vector,
};

use crate::commands::entity as entity_cmd;
use crate::component::player::PlayerBasicComponent;
use crate::event_queue::EventQueue;
use crate::scene::Scene;
use crate::sink::PushSink;

pub const PROLOGUE_QUEST_ID: i32 = 139000025;
pub const ACT2_QUEST_ID: i32 = 139000026;
pub const PROLOGUE_FLOW_NAME: &str = "剧情_新主线_CE版_序章";

pub fn transition_node<S: PushSink + ?Sized>(
    sink: &mut S,
    scene: &mut Scene,
    from_node: i32,
    to_node: i32,
) {
    scene.current_quest_node = to_node;
    sink.push(
        UpdateNodeStatusNotify::MSG_ID,
        &UpdateNodeStatusNotify {
            TreeOwnerId: PROLOGUE_QUEST_ID,
            TreeIncId: 1,
            NodeId: from_node,
            Status: Some(NodeStatus::CompletedSuccess),
        },
    );
    sink.push(
        UpdateChildQuestNodeStatusNotify::MSG_ID,
        &UpdateChildQuestNodeStatusNotify {
            TreeOwnerId: PROLOGUE_QUEST_ID,
            TreeIncId: 1,
            NodeId: from_node,
            Status: Some(ChildQuestNodeStatus::Finished),
        },
    );

    if to_node != 0 {
        sink.push(
            UpdateNodeStatusNotify::MSG_ID,
            &UpdateNodeStatusNotify {
                TreeOwnerId: PROLOGUE_QUEST_ID,
                TreeIncId: 1,
                NodeId: to_node,
                Status: Some(NodeStatus::Activate),
            },
        );
        sink.push(
            UpdateChildQuestNodeStatusNotify::MSG_ID,
            &UpdateChildQuestNodeStatusNotify {
                TreeOwnerId: PROLOGUE_QUEST_ID,
                TreeIncId: 1,
                NodeId: to_node,
                Status: Some(ChildQuestNodeStatus::Progress),
            },
        );
    }
}

pub fn start_flow<S: PushSink + ?Sized>(sink: &mut S, state_id: i32) {
    sink.push(
        FlowStartNotify::MSG_ID,
        &FlowStartNotify {
            FlowIncId: i64::from(state_id),
            FlowListName: PROLOGUE_FLOW_NAME.as_bytes().to_vec(),
            FlowId: 1,
            StateId: state_id,
            ..Default::default()
        },
    );
}

#[allow(clippy::too_many_arguments)]
pub fn spawn_quest_entity<S: PushSink + ?Sized>(
    events: &mut EventQueue,
    scene: &mut Scene,
    fs: &FileSystem,
    assets: &Assets,
    sink: &mut S,
    basic_comp: &PlayerBasicComponent,
    entity_id: i64,
) {
    if scene.debug_spawn_route(entity_id).is_some() {
        return;
    }

    let Some(cfg) = assets.find_level_entity(scene.instance_id, entity_id) else {
        return;
    };
    let Some(pos) = asset_indexes::entity_world_pos(&cfg) else {
        return;
    };

    let _ = entity_cmd::spawn_with_location(
        events,
        scene,
        fs,
        assets,
        sink,
        basic_comp,
        entity_id,
        None,
        None,
        Some(cfg.MapId),
        Some(pos),
        true,
    );
}

pub fn finish_prologue_quest<S: PushSink + ?Sized>(sink: &mut S, scene: &mut Scene) {
    transition_node(sink, scene, 159, 0);

    sink.push(
        QuestFinishListNotify::MSG_ID,
        &QuestFinishListNotify {
            QuestId: vec![PROLOGUE_QUEST_ID],
        },
    );

    sink.push(
        QuestListNotify::MSG_ID,
        &QuestListNotify {
            Quests: vec![QuestInfo {
                QuestId: ACT2_QUEST_ID,
                Status: Some(QuestState::InProgress),
            }],
        },
    );

    sink.push(
        QuestShowListNotify::MSG_ID,
        &QuestShowListNotify::default(),
    );

    sink.push(
        TraceQuestNotify::MSG_ID,
        &TraceQuestNotify {
            QuestId: ACT2_QUEST_ID,
        },
    );
    sink.push(
        FocusQuestSetNotify::MSG_ID,
        &FocusQuestSetNotify {
            FocusQuestId: ACT2_QUEST_ID,
            Reason: Some(QuestFocusReason::Inherit),
        },
    );

    sink.push(
        FuncOpenNotify::MSG_ID,
        &FuncOpenNotify {
            Func: vec![
                Function {
                    Id: 10003,
                    Flag: 2,
                },
                Function {
                    Id: 10023,
                    Flag: 2,
                },
            ],
        },
    );

    sink.push(
        TeleportNotify::MSG_ID,
        &TeleportNotify {
            MapId: scene.instance_id,
            Pos: Some(Vector {
                X: -105396.0,
                Y: 75049.0,
                Z: 6715.0,
            }),
            ..Default::default()
        },
    );
}
