use std::io;

use sonoro_common::fs::FileSystem;
use sonoro_gamesv_data::Assets;
use sonoro_proto::pb::{LeaveSceneNotify, TransitionOptionPb};

use crate::commands::respond;
use crate::component::player::PlayerSceneComponent;
use crate::event_queue::EventQueue;
use crate::events::Event;
use crate::scene::Scene;
use crate::sink::PushSink;

pub const ALIAS: &str = "m";
pub const DESCRIPTION: &str = "sends you to the map id in the first parameter.\nusage: map [map_id]\nmap_id is a number.";

pub fn call<S: PushSink + ?Sized>(
    events: &mut EventQueue,
    assets: &Assets,
    fs: &FileSystem,
    scene: &Scene,
    sink: &mut S,
    scene_comp: &mut PlayerSceneComponent,
    map_id: i32,
) -> io::Result<()> {
    let player_id = scene_comp.player_id;
    respond(events, player_id, "sending to map...");

    sink.push(
        LeaveSceneNotify::MSG_ID,
        &LeaveSceneNotify {
            PlayerId: player_id,
            SceneId: Vec::new(),
            TransitionOption: Some(TransitionOptionPb::default()),
        },
    );
    scene_comp.last_scene_info.instance_id = map_id;
    scene.save_last_scene(scene_comp, fs)?;
    events.enqueue(Event::InitialSceneJoin {
        player_id,
        map_config_id: assets.resolve_map_config_id(map_id),
        instance_id: map_id,
        location: None,
        rotation: None,
        pending_flow: None,
    });

    respond(events, player_id, "probably, sent you to map successfully!");
    Ok(())
}
