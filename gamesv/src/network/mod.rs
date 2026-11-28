use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use sonoro_common::args::{FlagSpec, parse_flags};
use sonoro_common::fs::FileSystem;
use sonoro_gamesv_data::Assets;
use sonoro_gamesv_logic::coop_service::CoopService;
use tokio::net::UdpSocket;
use tokio::sync::{Notify, mpsc};

use self::gm_bridge::GmJob;
use self::kcp_transport::{ConnectionHandle, KCP_OVERHEAD, Registry};
use self::state::State;

pub mod aki_logic_bridge;
pub mod connection;
pub mod encryption;
pub mod gm_bridge;
pub mod gm_http;
pub mod gm_i18n;
pub mod handlers;
pub mod kcp;
pub mod kcp_transport;
pub mod message;
pub mod mod_state {
    pub use super::SessionManager;
}
pub mod session_supervisor;
pub mod state;

const KRKCP_SYN: u8 = 0xED;
const KRKCP_ACK: u8 = 0xEE;
const UDP_RECEIVE_CAPACITY: usize = 1500;
const REPLACEMENT_TIMEOUT: Duration = Duration::from_secs(15);
const REPLACEMENT_POLL: Duration = Duration::from_millis(10);
const HANDOFF_TTL: Duration = Duration::from_secs(30);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CloseReason {
    Active = 0,
    Replaced = 1,
    GameplayExit = 2,
    InvalidKcpPacket = 3,
    TransportError = 4,
    KcpDeadLink = 5,
    SendFailed = 6,
    PreAuthTimeout = 7,
    IdleTimeout = 8,
    Shutdown = 9,
}

impl CloseReason {
    pub fn from_u8(v: u8) -> Self {
        match v {
            0 => Self::Active,
            1 => Self::Replaced,
            2 => Self::GameplayExit,
            3 => Self::InvalidKcpPacket,
            4 => Self::TransportError,
            5 => Self::KcpDeadLink,
            6 => Self::SendFailed,
            7 => Self::PreAuthTimeout,
            8 => Self::IdleTimeout,
            9 => Self::Shutdown,
            _ => Self::Active,
        }
    }
}

pub struct Handoff {
    pub state: State,
    pub seq: u32,
    pub last_client_seq_no: u32,
    pub last_scene_transition_seq: u32,
    created_at: Instant,
}

#[derive(Default)]
struct SessionRegistry {
    sessions: HashMap<i32, Arc<ConnectionHandle>>,
    handoffs: HashMap<i32, Handoff>,
    pending_replacements: HashMap<i32, HashMap<u32, Arc<ConnectionHandle>>>,
}

pub struct SessionManager {
    registry: Mutex<SessionRegistry>,
    fs: Mutex<Option<Arc<FileSystem>>>,
    pub coop: Arc<CoopService>,
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

fn drain_expired_handoffs(reg: &mut SessionRegistry) -> Vec<Handoff> {
    let expired_ids: Vec<i32> = reg
        .handoffs
        .iter()
        .filter_map(|(&pid, h)| (h.created_at.elapsed() >= HANDOFF_TTL).then_some(pid))
        .collect();
    let mut expired = Vec::with_capacity(expired_ids.len());
    for pid in expired_ids {
        if let Some(h) = reg.handoffs.remove(&pid) {
            expired.push(h);
        }
    }
    expired
}

fn cleanup_handoffs(mut list: Vec<Handoff>) {
    for h in &mut list {
        h.state.cleanup();
    }
}

fn fallback_state_dir_from_env() -> PathBuf {
    const SPECS: &[FlagSpec] = &[
        FlagSpec {
            flag: 's',
            name: "state_dir",
        },
        FlagSpec {
            flag: 'b',
            name: "bind_address",
        },
        FlagSpec {
            flag: 'g',
            name: "gm_bind_address",
        },
    ];
    let raw_args: Vec<String> = std::env::args().collect();
    if raw_args.len() > 1 {
        let mut parsed_dir: Option<String> = None;
        if parse_flags(&raw_args[1..], SPECS, |flag, val| {
            if flag == 's' && !val.is_empty() {
                parsed_dir = Some(val.to_string());
            }
            true
        }) && let Some(dir) = parsed_dir
        {
            return PathBuf::from(dir);
        }
    }
    if let Ok(dir) = std::env::var("GAMESV_STATE_DIR")
        && !dir.is_empty()
    {
        return PathBuf::from(dir);
    }
    PathBuf::from("state")
}

struct ReplacementGuard<'a> {
    manager: &'a SessionManager,
    player_id: i32,
    conv_id: u32,
    armed: bool,
}

impl Drop for ReplacementGuard<'_> {
    fn drop(&mut self) {
        if !self.armed {
            return;
        }
        let orphaned = {
            let mut reg = self.manager.registry.lock().unwrap_or_else(|e| e.into_inner());
            if let Some(waiters) = reg.pending_replacements.get_mut(&self.player_id) {
                waiters.remove(&self.conv_id);
                if waiters.is_empty() {
                    reg.pending_replacements.remove(&self.player_id);
                }
            }
            let no_waiters = !reg.pending_replacements.contains_key(&self.player_id);
            let no_session = !reg.sessions.contains_key(&self.player_id);
            if no_waiters && no_session {
                reg.handoffs.remove(&self.player_id)
            } else {
                None
            }
        };
        if let Some(mut handoff) = orphaned {
            handoff.state.cleanup();
        }
    }
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            registry: Mutex::new(SessionRegistry::default()),
            fs: Mutex::new(None),
            coop: Arc::new(CoopService::new()),
        }
    }

    pub fn attach_fs(&self, fs: Arc<FileSystem>) {
        *self.fs.lock().unwrap_or_else(|e| e.into_inner()) = Some(fs);
    }

    pub fn fs(&self) -> Option<Arc<FileSystem>> {
        self.fs.lock().unwrap_or_else(|e| e.into_inner()).clone()
    }

    pub fn state_dir(&self) -> PathBuf {
        if let Some(fs) = self.fs() {
            return fs.root_dir().to_path_buf();
        }
        fallback_state_dir_from_env()
    }

    pub fn has_session(&self, player_id: i32) -> bool {
        let reg = self.registry.lock().unwrap_or_else(|e| e.into_inner());
        reg.sessions.contains_key(&player_id)
    }

    pub fn has_pending_replacement(&self, player_id: i32) -> bool {
        let reg = self.registry.lock().unwrap_or_else(|e| e.into_inner());
        reg.pending_replacements
            .get(&player_id)
            .is_some_and(|waiters| {
                waiters
                    .values()
                    .any(|w| w.close_reason() == CloseReason::Active)
            })
    }

    pub fn clear_handoffs(&self) {
        let drained: Vec<Handoff> = {
            let mut reg = self.registry.lock().unwrap_or_else(|e| e.into_inner());
            reg.handoffs.drain().map(|(_, h)| h).collect()
        };
        cleanup_handoffs(drained);
    }

    pub async fn acquire(
        &self,
        handle: &Arc<ConnectionHandle>,
        player_id: i32,
    ) -> Result<(), &'static str> {
        self.acquire_with_timeout(handle, player_id, REPLACEMENT_TIMEOUT)
            .await
    }

    async fn acquire_with_timeout(
        &self,
        handle: &Arc<ConnectionHandle>,
        player_id: i32,
        timeout: Duration,
    ) -> Result<(), &'static str> {
        let deadline = Instant::now() + timeout;
        let mut replacement_requested: Option<u32> = None;
        let mut guard = ReplacementGuard {
            manager: self,
            player_id,
            conv_id: handle.conv_id,
            armed: false,
        };

        loop {
            if handle.close_reason() != CloseReason::Active {
                return Err("replacement session closed");
            }
            let (done, expired) = {
                let mut reg = self.registry.lock().unwrap_or_else(|e| e.into_inner());
                let expired = drain_expired_handoffs(&mut reg);
                if let Some(existing) = reg.sessions.get(&player_id).cloned() {
                    if Arc::ptr_eq(&existing, handle) {
                        guard.armed = false;
                        (true, expired)
                    } else {
                        guard.armed = true;
                        reg.pending_replacements
                            .entry(player_id)
                            .or_default()
                            .insert(handle.conv_id, Arc::clone(handle));
                        if replacement_requested != Some(existing.conv_id) {
                            existing.close(CloseReason::Replaced);
                            replacement_requested = Some(existing.conv_id);
                        }
                        (false, expired)
                    }
                } else {
                    reg.sessions.insert(player_id, Arc::clone(handle));
                    if let Some(waiters) = reg.pending_replacements.get_mut(&player_id) {
                        waiters.remove(&handle.conv_id);
                        if waiters.is_empty() {
                            reg.pending_replacements.remove(&player_id);
                        }
                    }
                    guard.armed = false;
                    (true, expired)
                }
            };
            cleanup_handoffs(expired);
            if done {
                return Ok(());
            }

            if Instant::now() >= deadline {
                return Err("session replacement timed out");
            }
            tokio::time::sleep(REPLACEMENT_POLL).await;
        }
    }

    pub fn release(&self, handle: &Arc<ConnectionHandle>, player_id: i32) {
        let expired = {
            let mut reg = self.registry.lock().unwrap_or_else(|e| e.into_inner());
            if let Some(existing) = reg.sessions.get(&player_id)
                && Arc::ptr_eq(existing, handle)
            {
                reg.sessions.remove(&player_id);
            }
            drain_expired_handoffs(&mut reg)
        };
        cleanup_handoffs(expired);
    }

    pub fn release_with_handoff(
        &self,
        handle: &Arc<ConnectionHandle>,
        player_id: i32,
        state: State,
        seq: u32,
        last_client_seq_no: u32,
        last_scene_transition_seq: u32,
    ) -> bool {
        let (accepted, unclaimed_state, expired) = {
            let mut reg = self.registry.lock().unwrap_or_else(|e| e.into_inner());
            let mut expired = drain_expired_handoffs(&mut reg);
            let is_owner = reg
                .sessions
                .get(&player_id)
                .is_some_and(|existing| Arc::ptr_eq(existing, handle));
            let has_live_waiter = reg
                .pending_replacements
                .get(&player_id)
                .is_some_and(|waiters| {
                    waiters
                        .values()
                        .any(|w| w.close_reason() == CloseReason::Active)
                });
            if is_owner && has_live_waiter {
                if let Some(prev) = reg.handoffs.insert(
                    player_id,
                    Handoff {
                        state,
                        seq,
                        last_client_seq_no,
                        last_scene_transition_seq,
                        created_at: Instant::now(),
                    },
                ) {
                    expired.push(prev);
                }
                reg.sessions.remove(&player_id);
                (true, None, expired)
            } else {
                (false, Some(state), expired)
            }
        };
        cleanup_handoffs(expired);
        if let Some(mut s) = unclaimed_state {
            s.cleanup();
        }
        accepted
    }

    pub fn take_handoff(&self, player_id: i32) -> Option<Handoff> {
        let (handoff, expired) = {
            let mut reg = self.registry.lock().unwrap_or_else(|e| e.into_inner());
            let expired = drain_expired_handoffs(&mut reg);
            (reg.handoffs.remove(&player_id), expired)
        };
        cleanup_handoffs(expired);
        handoff
    }

    pub fn list_online_ids(&self) -> Vec<i32> {
        let reg = self.registry.lock().unwrap_or_else(|e| e.into_inner());
        reg.sessions
            .iter()
            .filter_map(|(&pid, h)| (h.close_reason() == CloseReason::Active).then_some(pid))
            .collect()
    }

    pub fn submit_gm_job(&self, target_player_id: i32, job: GmJob) -> Result<(), GmJob> {
        if target_player_id <= 0 {
            return Err(job);
        }
        let handle = {
            let reg = self.registry.lock().unwrap_or_else(|e| e.into_inner());
            match reg.sessions.get(&target_player_id) {
                Some(h) if h.close_reason() == CloseReason::Active => Arc::clone(h),
                _ => return Err(job),
            }
        };
        handle.gm_inbox_tx.try_send(job).map_err(|e| e.into_inner())
    }

    pub fn kick_player(&self, target_player_id: i32) -> bool {
        let reg = self.registry.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(h) = reg.sessions.get(&target_player_id)
            && h.close_reason() == CloseReason::Active
        {
            h.close(CloseReason::GameplayExit);
            return true;
        }
        false
    }
}

fn now_monotonic_ms() -> i64 {
    static START: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();
    let start = START.get_or_init(Instant::now);
    start.elapsed().as_millis() as i64
}

pub fn now_unix_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_millis() as i64)
}

pub async fn bind_and_serve(
    bind_address: SocketAddr,
    fs: Arc<FileSystem>,
    assets: Arc<Assets>,
    session_manager: Arc<SessionManager>,
    shutdown: tokio::sync::watch::Receiver<bool>,
) -> io::Result<()> {
    session_manager.attach_fs(Arc::clone(&fs));
    let tokio_socket = UdpSocket::bind(bind_address).await.map_err(|err| {
        tracing::error!("failed to bind at {}: {}", bind_address, err);
        if err.kind() == io::ErrorKind::AddrInUse {
            tracing::error!("another instance of this server might be already running");
        }
        err
    })?;

    let std_socket = tokio_socket.into_std()?;
    std_socket.set_nonblocking(true)?;
    let sock_ref = socket2::SockRef::from(&std_socket);
    let _ = sock_ref.set_recv_buffer_size(4 * 1024 * 1024);
    let _ = sock_ref.set_send_buffer_size(4 * 1024 * 1024);
    let std_socket = Arc::new(std_socket);
    let async_socket = UdpSocket::from_std(std_socket.try_clone()?)?;

    receive_loop(
        async_socket,
        std_socket,
        fs,
        assets,
        session_manager,
        shutdown,
    )
    .await;
    Ok(())
}

struct IncomingPacket {
    data: Vec<u8>,
    from: SocketAddr,
}

fn recycle_packet(pkt: IncomingPacket, recycle_tx: &std::sync::mpsc::SyncSender<Vec<u8>>) {
    let mut data = pkt.data;
    if data.capacity() <= 8192 {
        data.clear();
        let _ = recycle_tx.try_send(data);
    }
}

async fn receive_loop(
    async_socket: UdpSocket,
    sync_socket: Arc<std::net::UdpSocket>,
    fs: Arc<FileSystem>,
    assets: Arc<Assets>,
    session_manager: Arc<SessionManager>,
    mut shutdown: tokio::sync::watch::Receiver<bool>,
) {
    let mut registry = Registry::new();
    let transport_work = Arc::new(Notify::new());
    let (done_tx, mut done_rx) = mpsc::channel::<Arc<ConnectionHandle>>(128);
    let (pkt_tx, mut pkt_rx) = mpsc::channel::<IncomingPacket>(1024);
    let (recycle_tx, recycle_rx) = std::sync::mpsc::sync_channel::<Vec<u8>>(256);

    let recv_handle = tokio::spawn(async move {
        let mut buf = [0u8; UDP_RECEIVE_CAPACITY];
        loop {
            match async_socket.recv_from(&mut buf).await {
                Ok((len, from)) => {
                    let mut data = recycle_rx
                        .try_recv()
                        .unwrap_or_else(|_| Vec::with_capacity(len.max(256)));
                    data.clear();
                    data.extend_from_slice(&buf[..len]);
                    if pkt_tx
                        .send(IncomingPacket { data, from })
                        .await
                        .is_err()
                    {
                        break;
                    }
                }
                Err(err) => {
                    if err.kind() == io::ErrorKind::ConnectionRefused
                        || err.kind() == io::ErrorKind::ConnectionReset
                    {
                        tracing::debug!("discarding UDP ICMP error: {}", err);
                        continue;
                    }
                    tracing::error!("UDP receive failed: {}", err);
                    break;
                }
            }
        }
    });

    loop {
        if *shutdown.borrow() {
            break;
        }
        let now_ms = now_monotonic_ms();
        while let Ok(pkt) = pkt_rx.try_recv() {
            handle_incoming_packet(
                &sync_socket,
                &transport_work,
                &done_tx,
                &session_manager,
                &fs,
                &assets,
                &mut registry,
                &pkt,
                now_ms,
            );
            recycle_packet(pkt, &recycle_tx);
        }
        while let Ok(handle) = done_rx.try_recv() {
            registry.finalize(&handle, now_ms);
        }
        if registry.drain_ready(&sync_socket, now_ms) {
            continue;
        }

        let wake_delay = registry.next_wake_delay_ms(now_monotonic_ms());
        let timer = async {
            match wake_delay {
                Some(ms) => tokio::time::sleep(Duration::from_millis(ms.max(0) as u64)).await,
                None => std::future::pending::<()>().await,
            }
        };

        tokio::select! {
            _ = shutdown.changed() => {
                if *shutdown.borrow() {
                    break;
                }
            }
            maybe_pkt = pkt_rx.recv() => {
                match maybe_pkt {
                    Some(pkt) => {
                        let now_ms = now_monotonic_ms();
                        handle_incoming_packet(
                            &sync_socket,
                            &transport_work,
                            &done_tx,
                            &session_manager,
                            &fs,
                            &assets,
                            &mut registry,
                            &pkt,
                            now_ms,
                        );
                        recycle_packet(pkt, &recycle_tx);
                    }
                    None => break,
                }
            }
            maybe_done = done_rx.recv() => {
                if let Some(handle) = maybe_done {
                    registry.finalize(&handle, now_monotonic_ms());
                }
            }
            _ = transport_work.notified() => {}
            _ = timer => {}
        }

        let now_ms = now_monotonic_ms();
        while let Ok(pkt) = pkt_rx.try_recv() {
            handle_incoming_packet(
                &sync_socket,
                &transport_work,
                &done_tx,
                &session_manager,
                &fs,
                &assets,
                &mut registry,
                &pkt,
                now_ms,
            );
            recycle_packet(pkt, &recycle_tx);
        }
        while let Ok(handle) = done_rx.try_recv() {
            registry.finalize(&handle, now_ms);
        }
        if registry.drain_ready(&sync_socket, now_ms) {
            transport_work.notify_one();
        }
    }

    recv_handle.abort();
    registry.close_all(CloseReason::Shutdown);
    while registry.session_count() > 0 {
        match tokio::time::timeout(Duration::from_secs(2), done_rx.recv()).await {
            Ok(Some(handle)) => registry.finalize(&handle, now_monotonic_ms()),
            _ => break,
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_incoming_packet(
    sync_socket: &std::net::UdpSocket,
    transport_work: &Arc<Notify>,
    done_tx: &mpsc::Sender<Arc<ConnectionHandle>>,
    session_manager: &Arc<SessionManager>,
    fs: &Arc<FileSystem>,
    assets: &Arc<Assets>,
    registry: &mut Registry,
    pkt: &IncomingPacket,
    now_ms: i64,
) {
    let data = &pkt.data;
    if data.len() == 35 && data[0] == KRKCP_SYN {
        if !registry.can_accept_pre_auth(pkt.from.ip()) {
            tracing::warn!(
                target: "network",
                "dropping KRKCP_SYN from {}: pre-auth session cap reached",
                pkt.from
            );
            return;
        }
        let Ok(conv_id) = registry.allocate_conv_id() else {
            tracing::error!("failed to allocate conv_id");
            return;
        };
        let channels = ConnectionHandle::open(conv_id, pkt.from, Arc::clone(transport_work));
        let handle = Arc::new(channels.handle);
        registry.create(Arc::clone(&handle), channels.outbound_rx, now_ms);

        let mut ack = [0u8; 5];
        ack[0] = KRKCP_ACK;
        ack[1..5].copy_from_slice(&conv_id.to_le_bytes());
        let _ = sync_socket.send_to(&ack, pkt.from);

        let h_clone = Arc::clone(&handle);
        let sm_clone = Arc::clone(session_manager);
        let fs_clone = Arc::clone(fs);
        let assets_clone = Arc::clone(assets);
        let done_clone = done_tx.clone();

        tokio::spawn(async move {
            session_supervisor::run(
                h_clone,
                channels.inbound_rx,
                channels.outbound_tx,
                channels.gm_inbox_rx,
                sm_clone,
                fs_clone,
                assets_clone,
            )
            .await;
            let _ = done_clone.send(handle).await;
        });
    } else if data.len() >= KCP_OVERHEAD {
        let conv_id = u32::from_le_bytes(data[0..4].try_into().unwrap());
        let _ = registry.input(sync_socket, conv_id, pkt.from, data, now_ms);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn acquire_timeout_clears_pending_replacement_waiter() {
        let sm = SessionManager::new();
        let notify = Arc::new(Notify::new());
        let addr: SocketAddr = "127.0.0.1:9001".parse().unwrap();

        let ch1 = ConnectionHandle::open(1, addr, Arc::clone(&notify));
        let h1 = Arc::new(ch1.handle);
        sm.acquire(&h1, 1001).await.unwrap();
        assert!(sm.has_session(1001));

        let ch2 = ConnectionHandle::open(2, addr, Arc::clone(&notify));
        let h2 = Arc::new(ch2.handle);
        let res = sm
            .acquire_with_timeout(&h2, 1001, Duration::from_millis(40))
            .await;
        assert!(res.is_err());
        assert_eq!(h1.close_reason(), CloseReason::Replaced);
        assert!(!sm.has_pending_replacement(1001));
    }
}
