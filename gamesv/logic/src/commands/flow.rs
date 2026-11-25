use std::io;

use sonoro_common::fs::FileSystem;
use sonoro_gamesv_data::Assets;
use sonoro_proto::pb::{LeaveSceneNotify, TransitionOptionPb};

use crate::commands::respond;
use crate::component::player::PlayerSceneComponent;
use crate::event_queue::EventQueue;
use crate::events::{Event, PendingFlow};
use crate::scene::Scene;
use crate::sink::PushSink;

pub const ALIAS: &str = "flow";
pub const DESCRIPTION: &str = "starts a flow, also sends you to the map containing the flow.\nusage: flow [flow_namespace] [flow_id] [state_id]\nflow_namespace is a string, flow_id is a number, state_id is a number.";

#[allow(clippy::too_many_arguments)]
pub fn call<S: PushSink + ?Sized>(
    events: &mut EventQueue,
    assets: &Assets,
    fs: &FileSystem,
    scene: &Scene,
    sink: &mut S,
    scene_comp: &mut PlayerSceneComponent,
    flow_namespace: &str,
    flow_id: i32,
    state_id: i32,
) -> io::Result<()> {
    let player_id = scene_comp.player_id;
    respond(events, player_id, "sending to flow...");

    let flow_key = format!("{flow_namespace}_{flow_id}");
    let Some(flow_inst) = assets.tables.flow.get_data_by_id(&flow_key) else {
        return Ok(());
    };
    sink.push(
        LeaveSceneNotify::MSG_ID,
        &LeaveSceneNotify {
            PlayerId: player_id,
            SceneId: Vec::new(),
            TransitionOption: Some(TransitionOptionPb::default()),
        },
    );
    scene_comp.last_scene_info.instance_id = flow_inst.DungeonId;
    scene.save_last_scene(scene_comp, fs)?;

    events.enqueue(Event::InitialSceneJoin {
        player_id,
        map_config_id: assets.resolve_map_config_id(flow_inst.DungeonId),
        instance_id: flow_inst.DungeonId,
        location: None,
        rotation: None,
        pending_flow: Some(PendingFlow {
            flow_list_name: flow_namespace.to_string(),
            flow_id,
            state_id,
            start_action_id: 1,
            source_quest_id: 0,
            source_quest_child_id: 0,
            pos_sender: 0,
        }),
    });

    respond(events, player_id, "probably, sent you to flow successfully!");
    Ok(())
}
