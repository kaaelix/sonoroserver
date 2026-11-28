use std::sync::Arc;

use sonoro_common::fs::FileSystem;
use sonoro_gamesv_data::Assets;
use tokio::sync::mpsc;

use super::CloseReason;
use super::connection::{GameplayState, drain_event_queue_best_effort, run_gameplay};
use super::gm_bridge::GmJob;
use super::kcp_transport::ConnectionHandle;
use super::mod_state::SessionManager;

pub async fn run(
    handle: Arc<ConnectionHandle>,
    mut inbound_rx: mpsc::Receiver<Vec<u8>>,
    outbound_tx: mpsc::Sender<Vec<u8>>,
    mut gm_inbox_rx: mpsc::Receiver<GmJob>,
    session_manager: Arc<SessionManager>,
    fs: Arc<FileSystem>,
    assets: Arc<Assets>,
) {
    let mut gameplay = GameplayState::new(outbound_tx, Arc::clone(&handle.transport_work));

    run_gameplay(
        &handle,
        &mut inbound_rx,
        &mut gm_inbox_rx,
        &session_manager,
        &fs,
        &assets,
        &mut gameplay,
    )
    .await;

    handle.close(CloseReason::GameplayExit);
    cleanup_gameplay(&mut gameplay, &session_manager, &handle, &mut gm_inbox_rx);
}

pub fn cleanup_gameplay(
    gameplay: &mut GameplayState,
    session_manager: &SessionManager,
    handle: &Arc<ConnectionHandle>,
    gm_inbox_rx: &mut mpsc::Receiver<GmJob>,
) {
    while let Ok(job) = gm_inbox_rx.try_recv() {
        job.fail_and_signal("session closed");
    }

    if handle.close_reason() == CloseReason::Replaced
        && let Some(player_id) = gameplay.claimed_player_id
        && session_manager.has_pending_replacement(player_id)
        && let Some(live_state) = gameplay.state.take()
        && session_manager.release_with_handoff(
            handle,
            player_id,
            live_state,
            gameplay.connection.seq(),
            gameplay.connection.last_client_seq_no(),
            gameplay.connection.last_scene_transition_seq(),
        )
    {
        gameplay.claimed_player_id = None;
        return;
    }

    if let Some(mut state) = gameplay.state.take() {
        state.cleanup();
    }

    if let Some(player_id) = gameplay.claimed_player_id.take() {
        session_manager.release(handle, player_id);
    }
}

pub fn drain_and_reset(gameplay_state: &mut super::state::State, events: &mut sonoro_gamesv_logic::event_queue::EventQueue) {
    drain_event_queue_best_effort(events, gameplay_state);
}
