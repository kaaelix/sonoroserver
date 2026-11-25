use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

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
}

pub struct SessionManager {
    sessions: Mutex<HashMap<i32, Arc<ConnectionHandle>>>,
    handoffs: Mutex<HashMap<i32, Handoff>>,
    pub coop: Arc<CoopService>,
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
            handoffs: Mutex::new(HashMap::new()),
            coop: Arc::new(CoopService::new()),
        }
    }

    pub fn clear_handoffs(&self) {
        let mut handoffs = self.handoffs.lock().unwrap_or_else(|e| e.into_inner());
        for (_, mut h) in handoffs.drain() {
            h.state.cleanup();
        }
    }

    pub async fn acquire(
        &self,
        handle: &Arc<ConnectionHandle>,
        player_id: i32,
    ) -> Result<(), &'static str> {
        let deadline = Instant::now() + REPLACEMENT_TIMEOUT;
        let mut replacement_requested: Option<u32> = None;

        loop {
            {
                let mut sessions = self.sessions.lock().unwrap_or_else(|e| e.into_inner());
                if let Some(existing) = sessions.get(&player_id) {
                    if Arc::ptr_eq(existing, handle) {
                        return Ok(());
                    }
                    if replacement_requested != Some(existing.conv_id) {
                        existing.close(CloseReason::Replaced);
                        replacement_requested = Some(existing.conv_id);
                    }
                } else {
                    sessions.insert(player_id, Arc::clone(handle));
                    return Ok(());
                }
            }

            if Instant::now() >= deadline {
                return Err("session replacement timed out");
            }
            tokio::time::sleep(REPLACEMENT_POLL).await;
        }
    }

    pub fn release(&self, handle: &Arc<ConnectionHandle>, player_id: i32) {
        let mut sessions = self.sessions.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(existing) = sessions.get(&player_id)
            && Arc::ptr_eq(existing, handle)
        {
            sessions.remove(&player_id);
        }
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
        let mut sessions = self.sessions.lock().unwrap_or_else(|e| e.into_inner());
        let Some(existing) = sessions.get(&player_id) else {
            return false;
        };
        if !Arc::ptr_eq(existing, handle) {
            return false;
        }
        let mut handoffs = self.handoffs.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(mut prev) = handoffs.insert(
            player_id,
            Handoff {
                state,
                seq,
                last_client_seq_no,
                last_scene_transition_seq,
            },
        ) {
            prev.state.cleanup();
        }
        sessions.remove(&player_id);
        drop(handoffs);
        drop(sessions);
        true
    }

    pub fn take_handoff(&self, player_id: i32) -> Option<Handoff> {
        self.handoffs
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .remove(&player_id)
    }

    pub fn list_online_ids(&self) -> Vec<i32> {
        let sessions = self.sessions.lock().unwrap_or_else(|e| e.into_inner());
        sessions
            .iter()
            .filter_map(|(&pid, h)| (h.close_reason() == CloseReason::Active).then_some(pid))
            .collect()
    }

    pub fn submit_gm_job(&self, target_player_id: i32, job: GmJob) -> Result<(), GmJob> {
        if target_player_id <= 0 {
            return Err(job);
        }
        let handle = {
            let sessions = self.sessions.lock().unwrap_or_else(|e| e.into_inner());
            match sessions.get(&target_player_id) {
                Some(h) if h.close_reason() == CloseReason::Active => Arc::clone(h),
                _ => return Err(job),
            }
        };
        handle.gm_inbox_tx.try_send(job).map_err(|e| e.into_inner())
    }

    pub fn kick_player(&self, target_player_id: i32) -> bool {
        let sessions = self.sessions.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(h) = sessions.get(&target_player_id)
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
