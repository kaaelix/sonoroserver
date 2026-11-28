use std::fmt::Write as _;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use sonoro_common::fs::FileSystem;
use sonoro_gamesv_data::Assets;
use sonoro_gamesv_logic::coop_service::{CoopSender, CoopService, OnlinePlayer};
use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::events::Event;
use sonoro_gamesv_logic::handlers::{self as logic_handlers};
use sonoro_gamesv_logic::scene::now_millis;
use sonoro_gamesv_logic::sink::PushSink;
use sonoro_proto::enums::{ErrorCode, WorldEnterPermission};
use sonoro_proto::pb::{
    HeartbeatResponse, JoinSceneNotify, LeaveSceneNotify, ReconnectResponse,
};
use sonoro_proto::registry::{msg_name, response_id_for};
use sonoro_proto::{PbEnum, PbMessage};
use tokio::sync::{Notify, mpsc};

use super::CloseReason;
use super::gm_bridge::GmJob;
use super::gm_http;
use super::handlers;
use super::handlers::auth::{AuthSessionState, handle_auth_group_request};
use super::kcp_transport::ConnectionHandle;
use super::message::{
    Header, Message, PushHeader, ResponseHeader, encode_alloc, encode_raw_alloc,
};
use super::mod_state::SessionManager;
use super::now_unix_ms;
use super::state::{PlayerComponentStorage, State};

const GAMEPLAY_EVENT_LIMIT: usize = 512;
const GAMEPLAY_TIME_LIMIT: Duration = Duration::from_millis(16);
const INBOUND_BATCH_LIMIT: usize = 256;

#[derive(Clone)]
pub struct Connection {
    outbound: mpsc::Sender<Vec<u8>>,
    transport_work: Arc<Notify>,
    send_mutex: Arc<Mutex<()>>,
    session_key: Arc<Mutex<Option<[u8; 32]>>>,
    seq: Arc<AtomicU32>,
    last_client_seq_no: Arc<AtomicU32>,
    last_scene_transition_seq: Arc<AtomicU32>,
}

impl Connection {
    pub fn new(outbound: mpsc::Sender<Vec<u8>>, transport_work: Arc<Notify>) -> Self {
        Self {
            outbound,
            transport_work,
            send_mutex: Arc::new(Mutex::new(())),
            session_key: Arc::new(Mutex::new(None)),
            seq: Arc::new(AtomicU32::new(0)),
            last_client_seq_no: Arc::new(AtomicU32::new(0)),
            last_scene_transition_seq: Arc::new(AtomicU32::new(0)),
        }
    }

    pub fn seq(&self) -> u32 {
        self.seq.load(Ordering::Acquire)
    }

    pub fn set_seq(&self, v: u32) {
        self.seq.store(v, Ordering::Release);
    }

    pub fn last_client_seq_no(&self) -> u32 {
        self.last_client_seq_no.load(Ordering::Acquire)
    }

    pub fn set_last_client_seq_no(&self, v: u32) {
        self.last_client_seq_no.store(v, Ordering::Release);
    }

    pub fn last_scene_transition_seq(&self) -> u32 {
        self.last_scene_transition_seq.load(Ordering::Acquire)
    }

    pub fn set_last_scene_transition_seq(&self, v: u32) {
        self.last_scene_transition_seq.store(v, Ordering::Release);
    }

    pub fn set_session_key(&mut self, key: [u8; 32]) {
        *self.session_key.lock().unwrap_or_else(|e| e.into_inner()) = Some(key);
    }

    pub fn session_key(&self) -> Option<[u8; 32]> {
        *self.session_key.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn next_seq_no(&self) -> u32 {
        self.seq.fetch_add(1, Ordering::AcqRel) + 1
    }

    pub fn record_client_seq_no(&self, header: &Header) {
        let seq_no = match *header {
            Header::Request(h) => h.seq_no,
            Header::Push(h) => h.seq_no,
            Header::Response(_) => return,
        };
        self.last_client_seq_no.fetch_max(seq_no, Ordering::AcqRel);
    }

    pub fn resolve_reconnect_last_recv_seq_no(
        restored_client_seq_no: u32,
        fallback_seq_no: i32,
    ) -> i32 {
        if restored_client_seq_no > 0 {
            restored_client_seq_no.min(i32::MAX as u32) as i32
        } else {
            fallback_seq_no
        }
    }

    fn send_frame(&self, bytes: Vec<u8>) -> Result<(), &'static str> {
        self.outbound
            .try_send(bytes)
            .map_err(|_| "outbound queue full or closed")?;
        self.transport_work.notify_one();
        Ok(())
    }

    #[allow(dead_code)]
    pub fn push_msg<M: PbMessage>(&self, msg_id: u16, message: &M) -> Result<(), &'static str> {
        let _guard = self.send_mutex.lock().unwrap_or_else(|e| e.into_inner());
        let seq_no = self.next_seq_no();
        if msg_id == LeaveSceneNotify::MSG_ID || msg_id == JoinSceneNotify::MSG_ID {
            self.last_scene_transition_seq
                .store(seq_no, Ordering::Release);
        }
        let key = *self.session_key.lock().unwrap_or_else(|e| e.into_inner());
        let bytes = encode_alloc(
            Header::Push(PushHeader { seq_no, msg_id }),
            message,
            key.as_ref(),
        );
        self.send_frame(bytes)
    }

    pub fn respond_with_msg_id<M: PbMessage>(
        &self,
        rpc_id: u16,
        message_id: u16,
        response: &M,
    ) -> Result<(), &'static str> {
        let _guard = self.send_mutex.lock().unwrap_or_else(|e| e.into_inner());
        let seq_no = self.next_seq_no();
        let key = *self.session_key.lock().unwrap_or_else(|e| e.into_inner());
        let bytes = encode_alloc(
            Header::Response(ResponseHeader {
                seq_no,
                rpc_id,
                msg_id: message_id,
            }),
            response,
            key.as_ref(),
        );
        self.send_frame(bytes)
    }

    pub fn respond_seq0_with_msg_id<M: PbMessage>(
        &self,
        rpc_id: u16,
        message_id: u16,
        response: &M,
    ) -> Result<(), &'static str> {
        let _guard = self.send_mutex.lock().unwrap_or_else(|e| e.into_inner());
        let key = *self.session_key.lock().unwrap_or_else(|e| e.into_inner());
        let bytes = encode_alloc(
            Header::Response(ResponseHeader {
                seq_no: 0,
                rpc_id,
                msg_id: message_id,
            }),
            response,
            key.as_ref(),
        );
        self.send_frame(bytes)
    }

    pub fn push_raw_with_message_id(
        &self,
        message_id: u16,
        raw_payload: &[u8],
    ) -> Result<(), &'static str> {
        let _guard = self.send_mutex.lock().unwrap_or_else(|e| e.into_inner());
        let seq_no = self.next_seq_no();
        if message_id == LeaveSceneNotify::MSG_ID || message_id == JoinSceneNotify::MSG_ID {
            self.last_scene_transition_seq
                .store(seq_no, Ordering::Release);
        }
        let key = *self.session_key.lock().unwrap_or_else(|e| e.into_inner());
        let bytes = encode_raw_alloc(
            Header::Push(PushHeader {
                seq_no,
                msg_id: message_id,
            }),
            raw_payload,
            key.as_ref(),
        );
        self.send_frame(bytes)
    }

    pub fn respond_raw_with_message_id(
        &self,
        rpc_id: u16,
        message_id: u16,
        raw_payload: &[u8],
    ) -> Result<(), &'static str> {
        let _guard = self.send_mutex.lock().unwrap_or_else(|e| e.into_inner());
        let seq_no = self.next_seq_no();
        let key = *self.session_key.lock().unwrap_or_else(|e| e.into_inner());
        let bytes = encode_raw_alloc(
            Header::Response(ResponseHeader {
                seq_no,
                rpc_id,
                msg_id: message_id,
            }),
            raw_payload,
            key.as_ref(),
        );
        self.send_frame(bytes)
    }
}

pub trait PbResponseId: PbMessage {
    const RESP_MSG_ID: u16;
}

impl PbResponseId for sonoro_proto::pb::ProtoKeyResponse {
    const RESP_MSG_ID: u16 = sonoro_proto::registry::msg_id::PROTO_KEY_RESPONSE;
}
impl PbResponseId for sonoro_proto::pb::LoginResponse {
    const RESP_MSG_ID: u16 = sonoro_proto::registry::msg_id::LOGIN_RESPONSE;
}
impl PbResponseId for sonoro_proto::pb::CreateCharacterResponse {
    const RESP_MSG_ID: u16 = sonoro_proto::registry::msg_id::CREATE_CHARACTER_RESPONSE;
}
impl PbResponseId for ReconnectResponse {
    const RESP_MSG_ID: u16 = sonoro_proto::registry::msg_id::RECONNECT_RESPONSE;
}
impl PbResponseId for HeartbeatResponse {
    const RESP_MSG_ID: u16 = sonoro_proto::registry::msg_id::HEARTBEAT_RESPONSE;
}
impl PbResponseId for sonoro_proto::pb::EnterGameResponse {
    const RESP_MSG_ID: u16 = sonoro_proto::registry::msg_id::ENTER_GAME_RESPONSE;
}

impl Connection {
    pub fn respond<M: PbResponseId>(&self, rpc_id: u16, response: &M) -> Result<(), &'static str> {
        self.respond_with_msg_id(rpc_id, M::RESP_MSG_ID, response)
    }

    pub fn respond_seq0<M: PbResponseId>(
        &self,
        rpc_id: u16,
        response: &M,
    ) -> Result<(), &'static str> {
        self.respond_seq0_with_msg_id(rpc_id, M::RESP_MSG_ID, response)
    }
}

impl PushSink for Connection {
    fn push_raw(&mut self, msg_id: u16, payload: Vec<u8>) {
        if let Err(err) = self.push_raw_with_message_id(msg_id, &payload) {
            tracing::warn!("push_raw msg_id={} failed: {}", msg_id, err);
        }
    }
}

impl CoopSender for Connection {
    fn send_raw(&self, msg_id: u16, payload: Vec<u8>) -> bool {
        self.push_raw_with_message_id(msg_id, &payload).is_ok()
    }
}

pub struct GameplayState {
    pub connection: Connection,
    pub state: Option<State>,
    pub claimed_player_id: Option<i32>,
}

impl GameplayState {
    pub fn new(outbound: mpsc::Sender<Vec<u8>>, transport_work: Arc<Notify>) -> Self {
        Self {
            connection: Connection::new(outbound, transport_work),
            state: None,
            claimed_player_id: None,
        }
    }
}

pub struct GameplayBudget {
    started: Instant,
    completed_events: usize,
}

impl GameplayBudget {
    pub fn start() -> Self {
        Self {
            started: Instant::now(),
            completed_events: 0,
        }
    }

    pub fn can_start(&self) -> bool {
        if self.completed_events == 0 {
            return true;
        }
        self.completed_events < GAMEPLAY_EVENT_LIMIT && self.started.elapsed() < GAMEPLAY_TIME_LIMIT
    }

    pub fn complete(&mut self) {
        self.completed_events += 1;
    }
}

pub fn drain_event_queue(events: &mut EventQueue, state: &mut State) -> std::io::Result<()> {
    while let Some(event) = events.dequeue() {
        let mut rt = state.as_runtime(
            events,
            Some(super::handlers::activity::push_activity_update_notify),
        );
        logic_handlers::dispatch_event(&mut rt, event)?;
    }
    Ok(())
}

pub fn drain_event_queue_best_effort(events: &mut EventQueue, state: &mut State) {
    while let Some(event) = events.dequeue() {
        let mut rt = state.as_runtime(
            events,
            Some(super::handlers::activity::push_activity_update_notify),
        );
        if let Err(err) = logic_handlers::dispatch_event(&mut rt, event) {
            tracing::error!(target: "connection", "best-effort event dispatch error: {}", err);
        }
    }
}

fn drain_gameplay_deadlines(state: &mut State, now_ms: i64) -> bool {
    let mut budget = GameplayBudget::start();
    let mut scheduled_changed = false;

    if state.dirty_saves.is_due(now_ms) && budget.can_start() {
        let mut events = EventQueue::new();
        if state.dirty_saves.enqueue_due(&mut events, now_ms) {
            drain_event_queue_best_effort(&mut events, state);
            budget.complete();
            scheduled_changed = true;
        }
    }

    if state.scene.is_some() {
        if !budget.can_start() {
            return scheduled_changed;
        }
        {
            let State {
                scene,
                buff_timers,
                assets,
                ..
            } = state;
            if let Some(sc) = scene.as_ref() {
                buff_timers.ensure_initialized(sc, assets, now_ms);
            }
        }

        let buff_due = state
            .buff_timers
            .peek_due_ms()
            .is_some_and(|due| due <= now_ms);
        if buff_due && budget.can_start() {
            let mut events = EventQueue::new();
            events.enqueue(Event::BuffTimersDue { now_ms });
            drain_event_queue_best_effort(&mut events, state);
            budget.complete();
            scheduled_changed = true;
        }

        let fsm_due = state
            .scene
            .as_ref()
            .and_then(|s| s.fsm_wakes.next_wake_delay_ms(now_ms))
            .is_some_and(|d| d <= 0);
        if fsm_due && budget.can_start() {
            let mut events = EventQueue::new();
            events.enqueue(Event::FsmWake {
                entity_id: 0,
                now_ms,
            });
            drain_event_queue_best_effort(&mut events, state);
            budget.complete();
            scheduled_changed = true;
        }

        let quest_due = state
            .scene
            .as_ref()
            .and_then(|s| s.next_quest_wake_delay_ms(now_ms))
            .is_some_and(|d| d <= 0);
        if quest_due && budget.can_start() {
            let mut events = EventQueue::new();
            events.enqueue(Event::QuestTimersDue { now_ms });
            drain_event_queue_best_effort(&mut events, state);
            budget.complete();
            scheduled_changed = true;
        }

        let pending_due = state
            .scene
            .as_ref()
            .and_then(|s| s.pending_update_wake_delay_ms(now_ms))
            .is_some_and(|d| d <= 0);
        if pending_due && budget.can_start() {
            let mut events = EventQueue::new();
            events.enqueue(Event::ScenePendingUpdates);
            drain_event_queue_best_effort(&mut events, state);
            budget.complete();
            scheduled_changed = true;
        }

        let spatial_due = state
            .scene
            .as_mut()
            .is_some_and(|s| s.take_due_deferred_spatial_spawns(now_ms));
        if spatial_due && budget.can_start() {
            let mut events = EventQueue::new();
            events.enqueue(Event::SpatialSpawnsCheck {
                player_id: state.player_id,
            });
            drain_event_queue_best_effort(&mut events, state);
            budget.complete();
            scheduled_changed = true;
        }

        while state
            .buff_timers
            .peek_due_ms()
            .is_some_and(|due| due <= now_ms)
            && budget.can_start()
        {
            let mut events = EventQueue::new();
            events.enqueue(Event::BuffTimersDue { now_ms });
            drain_event_queue_best_effort(&mut events, state);
            budget.complete();
            scheduled_changed = true;
        }
    }

    scheduled_changed
}

fn next_gameplay_wake_delay_ms(state: Option<&State>, now_ms: i64) -> Option<i64> {
    let s = state?;
    let scene = s.scene.as_ref();
    let base = gameplay_wake_delay_ms(
        scene.and_then(|_| s.buff_timers.next_wake_delay_ms(now_ms)),
        scene.and_then(|sc| sc.fsm_wakes.next_wake_delay_ms(now_ms)),
        scene.and_then(|sc| sc.next_quest_wake_delay_ms(now_ms)),
        scene.and_then(|sc| sc.pending_update_wake_delay_ms(now_ms)),
        s.dirty_saves.next_wake_delay_ms(now_ms),
    );
    let spatial = scene.and_then(|sc| sc.next_deferred_spatial_spawns_delay_ms(now_ms));
    [base, spatial].into_iter().flatten().min()
}

pub fn gameplay_wake_delay_ms(
    buff_delay_ms: Option<i64>,
    fsm_delay_ms: Option<i64>,
    quest_delay_ms: Option<i64>,
    pending_update_delay_ms: Option<i64>,
    dirty_delay_ms: Option<i64>,
) -> Option<i64> {
    [
        buff_delay_ms,
        fsm_delay_ms,
        quest_delay_ms,
        pending_update_delay_ms,
        dirty_delay_ms,
    ]
    .into_iter()
    .flatten()
    .min()
}

pub fn normalize_wait_delay_ms(delay_ms: Option<i64>) -> Option<i64> {
    delay_ms.map(|d| if d <= 0 { 1 } else { d })
}

static UNKNOWN_MSG_LOGGED: [std::sync::atomic::AtomicU64; 1024] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ZERO: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    [ZERO; 1024]
};

fn mark_unknown_msg_logged(msg_id: u16) -> bool {
    let word_idx = (msg_id >> 6) as usize;
    let bit = 1u64 << (msg_id & 63);
    (UNKNOWN_MSG_LOGGED[word_idx].fetch_or(bit, Ordering::Relaxed) & bit) != 0
}

fn read_varint(bytes: &[u8], pos: &mut usize) -> Option<u64> {
    let mut result = 0u64;
    let mut shift = 0u32;
    let mut count = 0usize;
    while *pos < bytes.len() && count < 10 {
        let b = bytes[*pos];
        *pos += 1;
        result |= u64::from(b & 0x7f) << shift;
        if (b & 0x80) == 0 {
            return Some(result);
        }
        if shift >= 57 {
            return None;
        }
        shift += 7;
        count += 1;
    }
    None
}

pub fn format_proto_wire_summary(payload: &[u8]) -> String {
    let mut pos = 0usize;
    let mut out = String::new();
    let mut fields = 0usize;
    while pos < payload.len() && fields < 16 {
        let Some(tag) = read_varint(payload, &mut pos) else {
            break;
        };
        let field_no = tag >> 3;
        let wire_type = (tag & 7) as u8;
        if field_no == 0 {
            break;
        }
        if fields > 0 {
            out.push_str(", ");
        }
        let _ = write!(out, "{field_no}:{wire_type}");
        match wire_type {
            0 => {
                if read_varint(payload, &mut pos).is_none() {
                    break;
                }
            }
            1 => {
                if pos + 8 > payload.len() {
                    break;
                }
                pos += 8;
            }
            2 => {
                let Some(len) = read_varint(payload, &mut pos) else {
                    break;
                };
                if len as usize > payload.len() - pos {
                    break;
                }
                pos += len as usize;
            }
            5 => {
                if pos + 4 > payload.len() {
                    break;
                }
                pos += 4;
            }
            _ => break,
        }
        fields += 1;
    }
    out
}

fn log_unknown_message_once(msg_id: u16, rpc_id: u16, is_request: bool, payload: &[u8]) {
    if msg_id == 31999 || payload.windows(6).any(|w| w == b"ZPROBE") {
        tracing::warn!(
            target: "connection",
            "[ZPROBE] unmapped msg_id={}, rpc_id={}, len={}, utf8={}",
            msg_id,
            rpc_id,
            payload.len(),
            String::from_utf8_lossy(payload),
        );
        return;
    }
    let kind = if is_request { "request" } else { "push" };
    if mark_unknown_msg_logged(msg_id) {
        tracing::debug!(
            target: "connection",
            "repeated unmapped {} (msg_id: {}, rpc_id: {}, len: {})",
            kind,
            msg_id,
            rpc_id,
            payload.len()
        );
        return;
    }
    let mut hex = String::with_capacity((payload.len() * 2).min(2048));
    for &b in payload.iter().take(1024) {
        let _ = write!(hex, "{b:02x}");
    }
    let wire = format_proto_wire_summary(payload);
    tracing::warn!(
        target: "connection",
        "unhandled unmapped {} (msg_id: {}, rpc_id: {}, len: {}, hex: {}, fields: [{}])",
        kind,
        msg_id,
        rpc_id,
        payload.len(),
        hex,
        wire
    );
}

fn register_online_coop(coop: &CoopService, s: &State, player_id: i32) {
    let attrs = &s.player_components.basic.info.attributes;
    let perm = <WorldEnterPermission as PbEnum>::from_i32_or_default(attrs.world_permission);
    coop.register_online(OnlinePlayer {
        player_id,
        sender: Arc::new(s.conn.clone()),
        name: attrs.name.clone(),
        level: attrs.level,
        origin_world_level: attrs.origin_world_level,
        cur_world_level: attrs.cur_world_level,
        head_id: attrs.head_photo,
        head_frame_id: attrs.head_frame,
        signature: attrs.sign.clone(),
        sex: attrs.sex,
        cur_role_id: 0,
        enter_permission: perm,
        is_can_lobby_online: perm != WorldEnterPermission::ForbidJoin,
        room_host_id: 0,
        match_inst_id: 0,
        match_host_id: 0,
        match_ready: false,
        match_confirm: false,
        match_ui_state: sonoro_proto::enums::EMatchPlayerUiState::Wait,
        match_roles: [0; 4],
        area_id: attrs.area_id,
    });
}

fn dispatch_single_state_message(
    st: &mut State,
    conn: &Connection,
    ready_message: &Message,
) -> bool {
    match handlers::dispatch_message(st, ready_message) {
        Ok(()) => true,
        Err(handlers::DispatchError::HandlerNotFound) => {
            let msg_id = ready_message.header.message_id();
            let rpc_id = match ready_message.header {
                Header::Request(h) => h.rpc_id,
                _ => 0,
            };
            let is_request = matches!(ready_message.header, Header::Request(_));
            if is_request {
                if let Some(response_id) = response_id_for(msg_id) {
                    if conn
                        .respond_with_msg_id(rpc_id, response_id, &HeartbeatResponse {})
                        .is_err()
                    {
                        return false;
                    }
                    if let Some(name) = msg_name(msg_id) {
                        tracing::warn!(
                            "sent mapped empty response (msg_id: {}) for unhandled {} (msg_id: {}, rpc_id: {})",
                            response_id,
                            name,
                            msg_id,
                            rpc_id
                        );
                    } else {
                        tracing::warn!(
                            "sent mapped empty response (msg_id: {}) for unmapped-name request (msg_id: {}, rpc_id: {})",
                            response_id,
                            msg_id,
                            rpc_id
                        );
                    }
                } else {
                    log_unknown_message_once(msg_id, rpc_id, true, &ready_message.payload);
                }
            } else if let Some(name) = msg_name(msg_id) {
                tracing::warn!(
                    "no handler for {} (msg_id: {}, len: {})",
                    name,
                    msg_id,
                    ready_message.payload.len()
                );
            } else {
                log_unknown_message_once(msg_id, 0, false, &ready_message.payload);
            }
            true
        }
        Err(handlers::DispatchError::Closed) => false,
        Err(handlers::DispatchError::Message(err)) => {
            tracing::error!(
                "failed to dispatch application message: {}, disconnecting",
                err
            );
            false
        }
    }
}

fn dispatch_state_batch(
    mut st: State,
    handle: &ConnectionHandle,
    conn: &Connection,
    batch: Vec<Vec<u8>>,
) -> (State, bool) {
    for owned_bytes in batch {
        if handle.close_reason() != CloseReason::Active {
            return (st, false);
        }
        let mut ready_message = match Message::decode(&owned_bytes) {
            Ok(m) => m,
            Err(err) => {
                tracing::error!("failed to decode application message: {}, disconnecting", err);
                return (st, false);
            }
        };
        if let Some(key) = conn.session_key()
            && let Err(err) = ready_message.decrypt(&key)
        {
            tracing::error!("failed to decrypt application message: {}, disconnecting", err);
            return (st, false);
        }
        if !dispatch_single_state_message(&mut st, conn, &ready_message) {
            return (st, false);
        }
        conn.record_client_seq_no(&ready_message.header);
    }
    (st, true)
}

async fn process_auth_inbound_message(
    handle: &Arc<ConnectionHandle>,
    session_manager: &Arc<SessionManager>,
    fs: &Arc<FileSystem>,
    assets: &Arc<Assets>,
    gameplay: &mut GameplayState,
    auth: &mut AuthSessionState,
    owned_bytes: Vec<u8>,
) -> bool {
    let mut conn = gameplay.connection.clone();
    let key = conn.session_key();
    let fs_auth = Arc::clone(fs);
    let mut moved_auth = std::mem::replace(
        auth,
        AuthSessionState {
            player_id: None,
            enter: false,
            pending_reconnect: None,
        },
    );
    let Ok((returned_conn, returned_auth, auth_ok)) =
        tokio::task::spawn_blocking(move || {
            let mut ready_message = match Message::decode(&owned_bytes) {
                Ok(m) => m,
                Err(err) => {
                    tracing::error!(
                        "failed to decode application message: {}, disconnecting",
                        err
                    );
                    return (conn, moved_auth, false);
                }
            };
            if let Some(key) = key
                && let Err(err) = ready_message.decrypt(&key)
            {
                tracing::error!(
                    "failed to decrypt application message: {}, disconnecting",
                    err
                );
                return (conn, moved_auth, false);
            }
            if let Err(err) =
                handle_auth_group_request(&ready_message, &mut conn, &fs_auth, &mut moved_auth)
            {
                tracing::error!("failed to handle auth request: {}, disconnecting", err);
                return (conn, moved_auth, false);
            }
            (conn, moved_auth, true)
        })
        .await
    else {
        return false;
    };
    gameplay.connection = returned_conn;
    *auth = returned_auth;
    if !auth_ok {
        return false;
    }

    if auth.enter
        && let Some(id) = auth.player_id
    {
        if let Err(err) = session_manager.acquire(handle, id).await {
            tracing::error!("failed to acquire player session: {}, disconnecting", err);
            return false;
        }
        gameplay.claimed_player_id = Some(id);
        handle.mark_authenticated();

        let is_reconnect = auth.pending_reconnect.is_some();
        let mut maybe_handoff = session_manager.take_handoff(id);
        if let Some(reconnect) = auth.pending_reconnect.take() {
            let last_seq = reconnect.last_server_seq_no.max(0) as u32;
            if let Some(handoff) = maybe_handoff.as_mut()
                && last_seq < handoff.last_scene_transition_seq
            {
                tracing::info!(
                    "player({}) reconnect seq={} missed scene transition seq={}; forcing fresh login",
                    id,
                    last_seq,
                    handoff.last_scene_transition_seq
                );
                handoff.state.cleanup();
                maybe_handoff = None;
            }
            let permit_silent_login = maybe_handoff.is_some();
            let restored_client_seq_no = maybe_handoff
                .as_ref()
                .map_or(0, |h| h.last_client_seq_no);
            let next_seq = maybe_handoff
                .as_ref()
                .map_or(last_seq, |h| h.seq.max(last_seq));
            gameplay.connection.set_seq(next_seq);
            gameplay
                .connection
                .set_last_client_seq_no(restored_client_seq_no);
            gameplay.connection.set_last_scene_transition_seq(
                maybe_handoff
                    .as_ref()
                    .map_or(0, |h| h.last_scene_transition_seq),
            );

            if gameplay
                .connection
                .respond(
                    reconnect.rpc_id,
                    &ReconnectResponse {
                        ErrorCode: Some(if permit_silent_login {
                            ErrorCode::Success
                        } else {
                            ErrorCode::ErrReconnectGwgetGatePlayerFailed
                        }),
                        LastRecvSeqNo: Connection::resolve_reconnect_last_recv_seq_no(
                            restored_client_seq_no,
                            reconnect.last_server_seq_no,
                        ),
                        Timestamp: now_unix_ms(),
                        IsPermittedSilentLogin: permit_silent_login,
                    },
                )
                .is_err()
            {
                if let Some(mut h) = maybe_handoff {
                    h.state.cleanup();
                }
                return false;
            }
            if !permit_silent_login {
                tracing::info!(
                    "player({}) reconnect requires fresh login (no compatible live session to hand off)",
                    id
                );
                return false;
            }
        }

        let conn_clone = gameplay.connection.clone();
        let sm_clone = Arc::clone(session_manager);
        let fs_clone = Arc::clone(fs);
        let assets_clone = Arc::clone(assets);
        let Ok(st_opt) = tokio::task::spawn_blocking(move || -> Option<State> {
            if is_reconnect {
                let mut handoff = maybe_handoff.expect("checked permit_silent_login above");
                handoff.state.conn = conn_clone;
                handoff.state.coop_service = Some(Arc::clone(&sm_clone.coop));
                register_online_coop(&sm_clone.coop, &handoff.state, id);
                tracing::info!(
                    "player({}) preserved live scene state on silent reconnect (seq={})",
                    id,
                    handoff.state.conn.seq()
                );

                let mut events = EventQueue::new();
                events.enqueue(Event::ReconnectResync {
                    player_id: id,
                    reason: "reconnect".to_string(),
                });
                if let Err(err) = drain_event_queue(&mut events, &mut handoff.state) {
                    tracing::error!(
                        "failed to execute reconnect resync event chain: {}, disconnecting",
                        err
                    );
                    handoff.state.cleanup();
                    return None;
                }
                Some(handoff.state)
            } else {
                if let Some(mut h) = maybe_handoff {
                    h.state.cleanup();
                }
                let pcs = match PlayerComponentStorage::init(&fs_clone, &assets_clone, id) {
                    Ok(pcs) => pcs,
                    Err(err) => {
                        tracing::error!(
                            "failed to init player component storage: {}, disconnecting",
                            err
                        );
                        return None;
                    }
                };
                let mut st = State::new(
                    fs_clone,
                    conn_clone,
                    assets_clone,
                    pcs,
                    id,
                );
                st.coop_service = Some(Arc::clone(&sm_clone.coop));
                register_online_coop(&sm_clone.coop, &st, id);

                let mut events = EventQueue::new();
                events.enqueue(Event::EnterGame { player_id: id });
                if let Err(err) = drain_event_queue(&mut events, &mut st) {
                    tracing::error!(
                        "failed to execute initial event chain: {}, disconnecting",
                        err
                    );
                    st.cleanup();
                    return None;
                }
                Some(st)
            }
        })
        .await
        else {
            return false;
        };

        let Some(st) = st_opt else {
            return false;
        };
        gameplay.state = Some(st);
    }

    true
}

async fn run_gm_job_offloaded(
    gameplay: &mut GameplayState,
    session_manager: &Arc<SessionManager>,
    job: GmJob,
) {
    let Some(mut st) = gameplay.state.take() else {
        job.fail_and_signal("Session is not logged in");
        return;
    };
    let sm = Arc::clone(session_manager);
    if let Ok(st_back) = tokio::task::spawn_blocking(move || {
        gm_http::execute_session_job(&mut st, &sm, job);
        st
    })
    .await
    {
        gameplay.state = Some(st_back);
    }
}

async fn run_state_batch_offloaded(
    handle: &Arc<ConnectionHandle>,
    gameplay: &mut GameplayState,
    batch: Vec<Vec<u8>>,
) -> bool {
    let Some(st) = gameplay.state.take() else {
        return false;
    };
    let conn = gameplay.connection.clone();
    let h_clone = Arc::clone(handle);
    match tokio::task::spawn_blocking(move || dispatch_state_batch(st, &h_clone, &conn, batch))
        .await
    {
        Ok((st_back, keep_going)) => {
            gameplay.state = Some(st_back);
            keep_going
        }
        Err(_) => false,
    }
}

async fn run_deadlines_offloaded(gameplay: &mut GameplayState, now_ms: i64) -> bool {
    let Some(mut st) = gameplay.state.take() else {
        return false;
    };
    match tokio::task::spawn_blocking(move || {
        let progressed = drain_gameplay_deadlines(&mut st, now_ms);
        (st, progressed)
    })
    .await
    {
        Ok((st_back, progressed)) => {
            gameplay.state = Some(st_back);
            progressed
        }
        Err(_) => false,
    }
}

pub async fn run_gameplay(
    handle: &Arc<ConnectionHandle>,
    inbound_rx: &mut mpsc::Receiver<Vec<u8>>,
    gm_inbox_rx: &mut mpsc::Receiver<GmJob>,
    session_manager: &Arc<SessionManager>,
    fs: &Arc<FileSystem>,
    assets: &Arc<Assets>,
    gameplay: &mut GameplayState,
) {
    let mut auth = AuthSessionState {
        player_id: None,
        enter: false,
        pending_reconnect: None,
    };

    loop {
        if handle.close_reason() != CloseReason::Active {
            return;
        }

        while let Ok(job) = gm_inbox_rx.try_recv() {
            if handle.close_reason() != CloseReason::Active {
                job.fail_and_signal("session closed");
                return;
            }
            run_gm_job_offloaded(gameplay, session_manager, job).await;
        }

        if gameplay.state.is_some() {
            let mut batch = Vec::new();
            while batch.len() < INBOUND_BATCH_LIMIT {
                match inbound_rx.try_recv() {
                    Ok(msg) => {
                        handle.transport_work.notify_one();
                        batch.push(msg);
                    }
                    Err(mpsc::error::TryRecvError::Empty) => break,
                    Err(mpsc::error::TryRecvError::Disconnected) => return,
                }
            }
            if !batch.is_empty() {
                if handle.close_reason() != CloseReason::Active {
                    return;
                }
                if !run_state_batch_offloaded(handle, gameplay, batch).await {
                    return;
                }
                continue;
            }
        } else if let Ok(msg) = inbound_rx.try_recv() {
            handle.transport_work.notify_one();
            if handle.close_reason() != CloseReason::Active {
                return;
            }
            if !process_auth_inbound_message(
                handle,
                session_manager,
                fs,
                assets,
                gameplay,
                &mut auth,
                msg,
            )
            .await
            {
                return;
            }
            continue;
        }

        if handle.close_reason() != CloseReason::Active {
            return;
        }

        let now_ms = now_millis();
        let mut delay_ms = next_gameplay_wake_delay_ms(gameplay.state.as_ref(), now_ms);
        if delay_ms.is_some_and(|d| d <= 0) && gameplay.state.is_some() {
            let progressed = run_deadlines_offloaded(gameplay, now_ms).await;
            if handle.close_reason() != CloseReason::Active {
                return;
            }
            if progressed {
                continue;
            }
            delay_ms = next_gameplay_wake_delay_ms(gameplay.state.as_ref(), now_millis());
        }

        let wait_ms = normalize_wait_delay_ms(delay_ms);
        let timer = async {
            match wait_ms {
                Some(ms) => tokio::time::sleep(Duration::from_millis(ms.max(1) as u64)).await,
                None => std::future::pending::<()>().await,
            }
        };

        tokio::select! {
            _ = handle.closed_notify.notified() => return,
            maybe_job = gm_inbox_rx.recv() => {
                match maybe_job {
                    Some(job) => run_gm_job_offloaded(gameplay, session_manager, job).await,
                    None => return,
                }
            }
            maybe_msg = inbound_rx.recv() => {
                match maybe_msg {
                    Some(msg) => {
                        handle.transport_work.notify_one();
                        if gameplay.state.is_some() {
                            let mut batch = vec![msg];
                            while batch.len() < INBOUND_BATCH_LIMIT {
                                match inbound_rx.try_recv() {
                                    Ok(next_msg) => {
                                        handle.transport_work.notify_one();
                                        batch.push(next_msg);
                                    }
                                    Err(mpsc::error::TryRecvError::Empty) => break,
                                    Err(mpsc::error::TryRecvError::Disconnected) => return,
                                }
                            }
                            if !run_state_batch_offloaded(handle, gameplay, batch).await {
                                return;
                            }
                        } else if !process_auth_inbound_message(
                            handle,
                            session_manager,
                            fs,
                            assets,
                            gameplay,
                            &mut auth,
                            msg,
                        )
                        .await
                        {
                            return;
                        }
                    }
                    None => return,
                }
            }
            _ = timer => {
                if handle.close_reason() != CloseReason::Active {
                    return;
                }
                if gameplay.state.is_some() {
                    let _ = run_deadlines_offloaded(gameplay, now_millis()).await;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::message::Message;
    use sonoro_proto::pb::{JSPatchNotify, ProtoKeyResponse};

    #[test]
    fn format_proto_wire_summary_inspects_fields_without_panicking() {
        assert_eq!(
            format_proto_wire_summary(&[0x08, 0x96, 0x01, 0x12, 0x02, b'a', b'b']),
            "1:0, 2:2"
        );
        assert_eq!(format_proto_wire_summary(&[0x0a, 0x10, 0x01]), "1:2");
    }

    #[test]
    fn connection_queues_push_and_response_frames_in_sequence_order() {
        let (tx, mut rx) = mpsc::channel(8);
        let work = Arc::new(Notify::new());
        let conn = Connection::new(tx, work);

        conn.push_msg(
            JSPatchNotify::MSG_ID,
            &JSPatchNotify {
                Content: b"owned".to_vec(),
            },
        )
        .unwrap();
        conn.respond(77, &HeartbeatResponse {}).unwrap();
        conn.respond_with_msg_id(88, 1234, &HeartbeatResponse {})
            .unwrap();

        let f1 = rx.try_recv().unwrap();
        let f2 = rx.try_recv().unwrap();
        let f3 = rx.try_recv().unwrap();

        let m1 = Message::decode(&f1[3..]).unwrap();
        let m2 = Message::decode(&f2[3..]).unwrap();
        let m3 = Message::decode(&f3[3..]).unwrap();

        assert_eq!(m1.header.seq_no(), 1);
        assert_eq!(m2.header.seq_no(), 2);
        assert_eq!(m3.header.seq_no(), 3);
        assert_eq!(m3.header.message_id(), 1234);
    }

    #[test]
    fn queued_proto_key_response_retains_old_key_boundary() {
        let (tx, mut rx) = mpsc::channel(8);
        let work = Arc::new(Notify::new());
        let mut conn = Connection::new(tx, work);

        conn.respond(
            9,
            &ProtoKeyResponse {
                Type: 2,
                Key: b"unencrypted".to_vec(),
                ..Default::default()
            },
        )
        .unwrap();
        let key = [0x5au8; 32];
        conn.set_session_key(key);
        conn.push_msg(
            JSPatchNotify::MSG_ID,
            &JSPatchNotify {
                Content: b"encrypted".to_vec(),
            },
        )
        .unwrap();

        let kframe = rx.try_recv().unwrap();
        let eframe = rx.try_recv().unwrap();
        let km = Message::decode(&kframe[3..]).unwrap();
        let mut em = Message::decode(&eframe[3..]).unwrap();
        em.decrypt(&key).unwrap();
        assert_eq!(
            ProtoKeyResponse::decode(&km.payload).unwrap().Key,
            b"unencrypted"
        );
        assert_eq!(
            JSPatchNotify::decode(&em.payload).unwrap().Content,
            b"encrypted"
        );
    }

    #[test]
    fn gameplay_wake_and_normalize_prevent_spinning() {
        assert_eq!(
            gameplay_wake_delay_ms(Some(50), Some(0), None, Some(1000), Some(30000)),
            Some(0)
        );
        assert_eq!(
            gameplay_wake_delay_ms(Some(50), Some(100), Some(15), Some(1000), None),
            Some(15)
        );
        assert_eq!(gameplay_wake_delay_ms(None, None, None, None, None), None);
        assert_eq!(normalize_wait_delay_ms(None), None);
        assert_eq!(normalize_wait_delay_ms(Some(-50)), Some(1));
        assert_eq!(normalize_wait_delay_ms(Some(0)), Some(1));
        assert_eq!(normalize_wait_delay_ms(Some(25)), Some(25));
    }

    #[test]
    fn tracks_highest_client_seq_and_resolves_reconnect_ack() {
        let (tx, _rx) = mpsc::channel(4);
        let conn = Connection::new(tx, Arc::new(Notify::new()));
        assert_eq!(conn.last_client_seq_no(), 0);
        assert_eq!(
            Connection::resolve_reconnect_last_recv_seq_no(conn.last_client_seq_no(), 1),
            1
        );
        conn.record_client_seq_no(&Header::Request(crate::network::message::RequestHeader {
            seq_no: 245,
            rpc_id: 9,
            msg_id: 100,
        }));
        conn.record_client_seq_no(&Header::Push(PushHeader {
            seq_no: 200,
            msg_id: 101,
        }));
        conn.record_client_seq_no(&Header::Response(ResponseHeader {
            seq_no: 999,
            rpc_id: 1,
            msg_id: 102,
        }));
        assert_eq!(conn.last_client_seq_no(), 245);
        assert_eq!(
            Connection::resolve_reconnect_last_recv_seq_no(u32::MAX, 1),
            i32::MAX
        );
    }
}
