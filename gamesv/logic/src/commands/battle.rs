use sonoro_proto::pb::CombatReceivePackNotify;

use crate::commands::respond;
use crate::event_queue::EventQueue;
use crate::scene::Scene;
use crate::sink::PushSink;

pub const ALIAS: &str = "bt";
pub const DESCRIPTION: &str = "change if the current player is in battle mode or not.\nusage: battle [mode]\nmode is a true or false value.";

pub fn call<S: PushSink + ?Sized>(
    events: &mut EventQueue,
    scene: &mut Scene,
    sink: &mut S,
    mode: bool,
) {
    scene.force_battle_state(mode);
    let mut data = Vec::new();
    if scene.append_battle_state_notify(&mut data) {
        sink.push(
            CombatReceivePackNotify::MSG_ID,
            &CombatReceivePackNotify { Data: data },
        );
    }
    respond(events, scene.player_id, "changed battle state");
}
