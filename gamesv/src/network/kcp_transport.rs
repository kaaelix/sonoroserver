use std::collections::{BTreeMap, HashMap, HashSet};
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::time::Instant;

use tokio::sync::{Notify, mpsc};

use super::CloseReason;
use super::gm_bridge::GmJob;
use super::kcp::{IKCP_OVERHEAD, Kcp, KcpError};

pub const PRE_AUTH_IDLE_TIMEOUT_MS: i64 = 30_000;
pub const AUTHENTICATED_IDLE_TIMEOUT_MS: i64 = 5 * 60_000;
pub const KCP_OVERHEAD: usize = IKCP_OVERHEAD;
pub const MAX_FRAME_LEN: usize = 4 * 1024 * 1024;
pub const MAX_PRE_AUTH_PER_IP: usize = 16;
pub const MAX_PRE_AUTH_GLOBAL: usize = 256;
const INITIAL_FRAME_CAP: usize = 64 * 1024;
const SEND_FAILURE_LIMIT: u8 = 3;
const ACTION_LIMIT: usize = 4096;
const TIME_LIMIT_NS: u128 = 16_000_000;
const OUTBOUND_BATCH_LIMIT: usize = 512;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamError {
    FrameTooLarge(usize),
}

#[derive(Default)]
pub struct StreamAssembler {
    length_prefix: [u8; 3],
    length_prefix_len: usize,
    body: Option<Vec<u8>>,
    body_len: usize,
    expected_len: usize,
}

pub struct ConsumeResult {
    pub consumed: usize,
    pub message: Option<Vec<u8>>,
    pub error: Option<StreamError>,
}

impl StreamAssembler {
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn reset(&mut self) {
        self.body = None;
        self.body_len = 0;
        self.expected_len = 0;
        self.length_prefix_len = 0;
    }

    pub fn consume(&mut self, input: &[u8]) -> ConsumeResult {
        let mut consumed = 0usize;

        while consumed < input.len() {
            if self.body.is_none() {
                let prefix_needed = self.length_prefix.len() - self.length_prefix_len;
                let prefix_count = prefix_needed.min(input.len() - consumed);
                self.length_prefix[self.length_prefix_len..self.length_prefix_len + prefix_count]
                    .copy_from_slice(&input[consumed..consumed + prefix_count]);
                self.length_prefix_len += prefix_count;
                consumed += prefix_count;

                if self.length_prefix_len != self.length_prefix.len() {
                    break;
                }

                let body_len = u32::from_le_bytes([
                    self.length_prefix[0],
                    self.length_prefix[1],
                    self.length_prefix[2],
                    0,
                ]) as usize;
                self.length_prefix_len = 0;
                self.body_len = 0;
                self.expected_len = 0;

                if body_len > MAX_FRAME_LEN {
                    return ConsumeResult {
                        consumed,
                        message: None,
                        error: Some(StreamError::FrameTooLarge(body_len)),
                    };
                }

                if body_len == 0 {
                    return ConsumeResult {
                        consumed,
                        message: Some(Vec::new()),
                        error: None,
                    };
                }
                self.expected_len = body_len;
                self.body = Some(Vec::with_capacity(body_len.min(INITIAL_FRAME_CAP)));
            }

            let body = self.body.as_mut().expect("allocated above");
            let body_needed = self.expected_len - self.body_len;
            let body_count = body_needed.min(input.len() - consumed);
            body.extend_from_slice(&input[consumed..consumed + body_count]);
            self.body_len += body_count;
            consumed += body_count;

            if self.body_len == self.expected_len {
                let finished = self.body.take().unwrap();
                self.body_len = 0;
                self.expected_len = 0;
                return ConsumeResult {
                    consumed,
                    message: Some(finished),
                    error: None,
                };
            }
        }

        ConsumeResult {
            consumed,
            message: None,
            error: None,
        }
    }
}

#[derive(Default)]
struct ApplicationInput {
    bytes: Option<Vec<u8>>,
    offset: usize,
}

impl ApplicationInput {
    fn receive(&mut self, kcp: &mut Kcp) -> bool {
        debug_assert!(self.bytes.is_none());
        match kcp.recv_vec() {
            Ok(bytes) => {
                self.bytes = Some(bytes);
                self.offset = 0;
                true
            }
            Err(KcpError::EmptyQueue | KcpError::IncompletePacket) => false,
            Err(_) => false,
        }
    }

    fn consume(&mut self, assembler: &mut StreamAssembler) -> Result<Option<Vec<u8>>, StreamError> {
        let Some(bytes) = self.bytes.as_ref() else {
            return Ok(None);
        };
        let res = assembler.consume(&bytes[self.offset..]);
        self.offset += res.consumed;
        if let Some(err) = res.error {
            self.bytes = None;
            self.offset = 0;
            assembler.reset();
            return Err(err);
        }
        if self.offset == bytes.len() {
            self.bytes = None;
            self.offset = 0;
        }
        Ok(res.message)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Phase {
    PreAuth,
    Authenticated,
}

pub struct ConnectionHandle {
    pub conv_id: u32,
    address: std::sync::Mutex<SocketAddr>,
    pub inbound_tx: mpsc::Sender<Vec<u8>>,
    pub gm_inbox_tx: mpsc::Sender<GmJob>,
    pub closed_notify: Arc<Notify>,
    pub transport_work: Arc<Notify>,
    close_reason: AtomicU8,
    authenticated: AtomicBool,
}

pub struct ConnectionChannels {
    pub handle: ConnectionHandle,
    pub inbound_rx: mpsc::Receiver<Vec<u8>>,
    pub outbound_tx: mpsc::Sender<Vec<u8>>,
    pub outbound_rx: mpsc::Receiver<Vec<u8>>,
    pub gm_inbox_rx: mpsc::Receiver<GmJob>,
}

impl ConnectionHandle {
    pub fn open(
        conv_id: u32,
        address: SocketAddr,
        transport_work: Arc<Notify>,
    ) -> ConnectionChannels {
        let (inbound_tx, inbound_rx) = mpsc::channel(256);
        let (outbound_tx, outbound_rx) = mpsc::channel(8192);
        let (gm_inbox_tx, gm_inbox_rx) = mpsc::channel(16);
        let handle = Self {
            conv_id,
            address: std::sync::Mutex::new(address),
            inbound_tx,
            gm_inbox_tx,
            closed_notify: Arc::new(Notify::new()),
            transport_work,
            close_reason: AtomicU8::new(CloseReason::Active as u8),
            authenticated: AtomicBool::new(false),
        };
        ConnectionChannels {
            handle,
            inbound_rx,
            outbound_tx,
            outbound_rx,
            gm_inbox_rx,
        }
    }

    pub fn close(&self, reason: CloseReason) {
        let _ = self.close_reason.compare_exchange(
            CloseReason::Active as u8,
            reason as u8,
            Ordering::AcqRel,
            Ordering::Acquire,
        );
        self.closed_notify.notify_waiters();
        self.transport_work.notify_one();
    }

    pub fn close_reason(&self) -> CloseReason {
        CloseReason::from_u8(self.close_reason.load(Ordering::Acquire))
    }

    pub fn mark_authenticated(&self) {
        self.authenticated.store(true, Ordering::Release);
        self.transport_work.notify_one();
    }

    pub fn address(&self) -> SocketAddr {
        *self.address.lock().unwrap_or_else(|e| e.into_inner())
    }

    pub fn set_address(&self, addr: SocketAddr) {
        *self.address.lock().unwrap_or_else(|e| e.into_inner()) = addr;
    }

    pub fn is_authenticated(&self) -> bool {
        self.authenticated.load(Ordering::Acquire)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DeadlineKind {
    KcpUpdate = 0,
    Idle = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeadlineEntry {
    pub conv_id: u32,
    pub kind: DeadlineKind,
    pub due_ms: i64,
}

#[derive(Default)]
pub struct DeadlineHeap {
    by_due: BTreeMap<(i64, u32, DeadlineKind), ()>,
    by_key: HashMap<u64, (i64, u32, DeadlineKind)>,
}

impl DeadlineHeap {
    fn key(conv_id: u32, kind: DeadlineKind) -> u64 {
        (u64::from(conv_id) << 1) | (kind as u64)
    }

    pub fn upsert(&mut self, entry: DeadlineEntry) {
        let k = Self::key(entry.conv_id, entry.kind);
        if let Some(prev) = self.by_key.insert(k, (entry.due_ms, entry.conv_id, entry.kind)) {
            self.by_due.remove(&prev);
        }
        self.by_due
            .insert((entry.due_ms, entry.conv_id, entry.kind), ());
    }

    pub fn cancel(&mut self, conv_id: u32, kind: DeadlineKind) {
        if let Some(prev) = self.by_key.remove(&Self::key(conv_id, kind)) {
            self.by_due.remove(&prev);
        }
    }

    pub fn cancel_session(&mut self, conv_id: u32) {
        self.cancel(conv_id, DeadlineKind::KcpUpdate);
        self.cancel(conv_id, DeadlineKind::Idle);
    }

    pub fn peek_due_ms(&self) -> Option<i64> {
        self.by_due.keys().next().map(|&(due, _, _)| due)
    }

    pub fn pop_due(&mut self, now_ms: i64) -> Option<DeadlineEntry> {
        let &(due_ms, conv_id, kind) = self.by_due.keys().next()?;
        if due_ms > now_ms {
            return None;
        }
        self.by_due.remove(&(due_ms, conv_id, kind));
        self.by_key.remove(&Self::key(conv_id, kind));
        Some(DeadlineEntry {
            conv_id,
            kind,
            due_ms,
        })
    }
}

struct Session {
    handle: Arc<ConnectionHandle>,
    outbound_rx: mpsc::Receiver<Vec<u8>>,
    kcp: Kcp,
    assembler: StreamAssembler,
    application: ApplicationInput,
    pending_inbound: Option<Vec<u8>>,
    init_time_ms: i64,
    last_receive_time_ms: i64,
    consecutive_send_failures: u8,
    phase: Phase,
}

impl Session {
    fn new(handle: Arc<ConnectionHandle>, outbound_rx: mpsc::Receiver<Vec<u8>>, now_ms: i64) -> Self {
        let mut kcp = Kcp::new(handle.conv_id);
        kcp.stream = true;
        let _ = kcp.set_mtu(1000);
        kcp.dead_link = 60;
        kcp.set_nodelay(1, 5, 1, true);
        kcp.rx_minrto = 25;
        kcp.set_wnd_size(512, 512);
        Self {
            handle,
            outbound_rx,
            kcp,
            assembler: StreamAssembler::new(),
            application: ApplicationInput::default(),
            pending_inbound: None,
            init_time_ms: now_ms,
            last_receive_time_ms: now_ms,
            consecutive_send_failures: 0,
            phase: Phase::PreAuth,
        }
    }

    fn elapsed_ms(&self, now_ms: i64) -> u32 {
        (now_ms - self.init_time_ms).max(0) as u32
    }

    fn idle_timeout_ms(&self) -> i64 {
        match self.phase {
            Phase::PreAuth => PRE_AUTH_IDLE_TIMEOUT_MS,
            Phase::Authenticated => AUTHENTICATED_IDLE_TIMEOUT_MS,
        }
    }

    fn idle_deadline_ms(&self) -> i64 {
        self.last_receive_time_ms
            .saturating_add(self.idle_timeout_ms())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ActionClass {
    Inbound,
    Outbound,
    Deadline,
}

pub struct WorkBudget {
    started: Instant,
    actions: usize,
}

impl WorkBudget {
    pub fn start() -> Self {
        Self {
            started: Instant::now(),
            actions: 0,
        }
    }

    pub fn can_start(&self) -> bool {
        if self.actions == 0 {
            return true;
        }
        self.actions < ACTION_LIMIT && self.started.elapsed().as_nanos() < TIME_LIMIT_NS
    }

    pub fn complete(&mut self) {
        self.actions += 1;
    }
}

pub struct Registry {
    active: HashMap<u32, Session>,
    closing: HashMap<u32, Session>,
    issued_ids: HashSet<u32>,
    order: Vec<u32>,
    deadlines: DeadlineHeap,
    next_conv_id: u32,
    inbound_cursor: usize,
    outbound_cursor: usize,
    next_action_class: ActionClass,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            active: HashMap::new(),
            closing: HashMap::new(),
            issued_ids: HashSet::new(),
            order: Vec::new(),
            deadlines: DeadlineHeap::default(),
            next_conv_id: 1,
            inbound_cursor: 0,
            outbound_cursor: 0,
            next_action_class: ActionClass::Inbound,
        }
    }

    pub fn allocate_conv_id(&mut self) -> Result<u32, &'static str> {
        let first = self.next_conv_id;
        loop {
            let candidate = self.next_conv_id;
            self.next_conv_id = self.next_conv_id.wrapping_add(1);
            if self.next_conv_id == 0 {
                self.next_conv_id = 1;
            }
            if self.issued_ids.insert(candidate) {
                return Ok(candidate);
            }
            if self.next_conv_id == first {
                return Err("conversation IDs exhausted");
            }
        }
    }

    pub fn create(
        &mut self,
        handle: Arc<ConnectionHandle>,
        outbound_rx: mpsc::Receiver<Vec<u8>>,
        now_ms: i64,
    ) {
        let conv_id = handle.conv_id;
        let session = Session::new(handle, outbound_rx, now_ms);
        let kcp_due = now_ms.saturating_add(i64::from(
            session.kcp.next_update_delay(session.elapsed_ms(now_ms)),
        ));
        let idle_due = session.idle_deadline_ms();
        self.active.insert(conv_id, session);
        self.order.push(conv_id);
        self.deadlines.upsert(DeadlineEntry {
            conv_id,
            kind: DeadlineKind::KcpUpdate,
            due_ms: kcp_due,
        });
        self.deadlines.upsert(DeadlineEntry {
            conv_id,
            kind: DeadlineKind::Idle,
            due_ms: idle_due,
        });
    }

    pub fn session_count(&self) -> usize {
        self.active.len() + self.closing.len()
    }

    pub fn can_accept_pre_auth(&self, ip: IpAddr) -> bool {
        let mut global_pre_auth = 0usize;
        let mut ip_pre_auth = 0usize;
        for session in self.active.values() {
            if session.phase == Phase::PreAuth
                && !session.handle.is_authenticated()
                && session.handle.close_reason() == CloseReason::Active
            {
                global_pre_auth += 1;
                if session.handle.address().ip() == ip {
                    ip_pre_auth += 1;
                }
                if global_pre_auth >= MAX_PRE_AUTH_GLOBAL || ip_pre_auth >= MAX_PRE_AUTH_PER_IP {
                    return false;
                }
            }
        }
        true
    }

    pub fn close_all(&mut self, reason: CloseReason) {
        let conv_ids: Vec<u32> = self.order.clone();
        for conv_id in conv_ids {
            self.mark_closing(conv_id, reason);
        }
    }

    pub fn input(
        &mut self,
        socket: &std::net::UdpSocket,
        conv_id: u32,
        from: SocketAddr,
        bytes: &[u8],
        now_ms: i64,
    ) -> bool {
        let Some(session) = self.active.get_mut(&conv_id) else {
            return false;
        };
        let reason = session.handle.close_reason();
        if reason != CloseReason::Active {
            self.mark_closing(conv_id, reason);
            return false;
        }
        let current_addr = session.handle.address();
        if !registered_endpoint_matches(&current_addr, &from) {
            if is_loopback_endpoint(&current_addr) && is_loopback_endpoint(&from) {
                session.handle.set_address(from);
            } else {
                return false;
            }
        }

        if session.kcp.input(bytes).is_err() {
            self.mark_closing(conv_id, CloseReason::InvalidKcpPacket);
            return false;
        }
        session.last_receive_time_ms = now_ms;
        flush_kcp_to_socket(session, socket, now_ms);
        if session.consecutive_send_failures >= SEND_FAILURE_LIMIT {
            self.mark_closing(conv_id, CloseReason::SendFailed);
            return false;
        }
        let next_kcp_due = now_ms.saturating_add(i64::from(
            session.kcp.next_update_delay(session.elapsed_ms(now_ms)),
        ));
        let idle_due = session.idle_deadline_ms();
        self.deadlines.upsert(DeadlineEntry {
            conv_id,
            kind: DeadlineKind::KcpUpdate,
            due_ms: next_kcp_due,
        });
        self.deadlines.upsert(DeadlineEntry {
            conv_id,
            kind: DeadlineKind::Idle,
            due_ms: idle_due,
        });
        true
    }

    pub fn next_wake_delay_ms(&self, now_ms: i64) -> Option<i64> {
        self.deadlines
            .peek_due_ms()
            .map(|due| (due - now_ms).max(0))
    }

    pub fn drain_ready(&mut self, socket: &std::net::UdpSocket, now_ms: i64) -> bool {
        let mut budget = WorkBudget::start();
        self.sync_shared_state();

        let mut empty_classes = 0u8;
        while budget.can_start() && empty_classes < 3 {
            let progressed = match self.next_action_class {
                ActionClass::Inbound => self.forward_next(),
                ActionClass::Outbound => self.send_next(socket, now_ms),
                ActionClass::Deadline => self.process_next_deadline(socket, now_ms),
            };
            self.next_action_class = match self.next_action_class {
                ActionClass::Inbound => ActionClass::Outbound,
                ActionClass::Outbound => ActionClass::Deadline,
                ActionClass::Deadline => ActionClass::Inbound,
            };
            if progressed {
                budget.complete();
                empty_classes = 0;
            } else {
                empty_classes += 1;
            }
        }
        !budget.can_start()
    }

    pub fn finalize(&mut self, handle: &ConnectionHandle, now_ms: i64) {
        self.mark_closing(handle.conv_id, handle.close_reason());
        if let Some(session) = self.closing.remove(&handle.conv_id) {
            tracing::info!(
                target: "kcp_transport",
                "session ended, conv: {}, endpoint: {}, phase: {:?}, reason: {:?}, last_input_age_ms: {}, send_failures: {}",
                handle.conv_id,
                handle.address(),
                session.phase,
                handle.close_reason(),
                (now_ms - session.last_receive_time_ms).max(0),
                session.consecutive_send_failures,
            );
        }
        if let Some(idx) = self.order.iter().position(|&id| id == handle.conv_id) {
            self.order.remove(idx);
            if self.order.is_empty() {
                self.inbound_cursor = 0;
                self.outbound_cursor = 0;
            } else {
                if self.inbound_cursor >= self.order.len() {
                    self.inbound_cursor = 0;
                }
                if self.outbound_cursor >= self.order.len() {
                    self.outbound_cursor = 0;
                }
            }
        }
        self.deadlines.cancel_session(handle.conv_id);
        self.issued_ids.remove(&handle.conv_id);
    }

    fn sync_shared_state(&mut self) {
        let convs: Vec<u32> = self.order.clone();
        for conv_id in convs {
            let Some(session) = self.active.get_mut(&conv_id) else {
                continue;
            };
            let close_reason = session.handle.close_reason();
            if close_reason != CloseReason::Active {
                self.mark_closing(conv_id, close_reason);
                continue;
            }
            if session.phase == Phase::PreAuth && session.handle.is_authenticated() {
                session.phase = Phase::Authenticated;
                let idle_due = session.idle_deadline_ms();
                self.deadlines.upsert(DeadlineEntry {
                    conv_id,
                    kind: DeadlineKind::Idle,
                    due_ms: idle_due,
                });
            }
        }
    }

    fn forward_next(&mut self) -> bool {
        if self.order.is_empty() {
            return false;
        }
        let len = self.order.len();
        for _ in 0..len {
            self.inbound_cursor %= self.order.len();
            let conv_id = self.order[self.inbound_cursor];
            self.inbound_cursor = (self.inbound_cursor + 1) % self.order.len();
            if self.forward_one(conv_id) {
                return true;
            }
        }
        false
    }

    fn send_next(&mut self, socket: &std::net::UdpSocket, now_ms: i64) -> bool {
        if self.order.is_empty() {
            return false;
        }
        let len = self.order.len();
        for _ in 0..len {
            self.outbound_cursor %= self.order.len();
            let conv_id = self.order[self.outbound_cursor];
            self.outbound_cursor = (self.outbound_cursor + 1) % self.order.len();
            if self.send_one(conv_id, socket, now_ms) {
                return true;
            }
        }
        false
    }

    fn process_next_deadline(&mut self, socket: &std::net::UdpSocket, now_ms: i64) -> bool {
        while let Some(entry) = self.deadlines.pop_due(now_ms) {
            if !self.active.contains_key(&entry.conv_id) {
                continue;
            }
            self.process_deadline(entry.conv_id, entry.kind, socket, now_ms);
            return true;
        }
        false
    }

    fn forward_one(&mut self, conv_id: u32) -> bool {
        let Some(session) = self.active.get_mut(&conv_id) else {
            return false;
        };
        if session.pending_inbound.is_none() {
            match pump_application_data(
                &mut session.kcp,
                &mut session.assembler,
                &mut session.application,
            ) {
                Ok(msg) => session.pending_inbound = msg,
                Err(StreamError::FrameTooLarge(len)) => {
                    tracing::warn!(
                        target: "kcp_transport",
                        "closing session conv={} due to oversized frame ({} > {} bytes)",
                        conv_id,
                        len,
                        MAX_FRAME_LEN
                    );
                    self.mark_closing(conv_id, CloseReason::TransportError);
                    return false;
                }
            }
        }
        let Some(msg) = session.pending_inbound.take() else {
            return false;
        };
        match session.handle.inbound_tx.try_send(msg) {
            Ok(()) => true,
            Err(mpsc::error::TrySendError::Full(returned)) => {
                session.pending_inbound = Some(returned);
                false
            }
            Err(mpsc::error::TrySendError::Closed(_)) => {
                self.mark_closing(conv_id, CloseReason::GameplayExit);
                false
            }
        }
    }

    fn send_one(&mut self, conv_id: u32, socket: &std::net::UdpSocket, now_ms: i64) -> bool {
        let Some(session) = self.active.get_mut(&conv_id) else {
            return false;
        };
        let wait_snd = session.kcp.wait_snd();
        let snd_wnd = session.kcp.snd_wnd as usize;
        let eff_wnd = snd_wnd.min(session.kcp.rmt_wnd.max(128) as usize);
        if wait_snd >= eff_wnd {
            return false;
        }
        // In stream mode multiple small notifies coalesce into a single MSS segment, so allow
        // draining up to 4x remaining segment slots (bounded by OUTBOUND_BATCH_LIMIT).
        let max_batch = OUTBOUND_BATCH_LIMIT.min((eff_wnd - wait_snd).saturating_mul(4));

        let mut sent_any = false;
        let mut send_err = false;
        for _ in 0..max_batch {
            match session.outbound_rx.try_recv() {
                Ok(frame) => {
                    sent_any = true;
                    if !send_err && session.kcp.send(&frame).is_err() {
                        send_err = true;
                    }
                }
                Err(mpsc::error::TryRecvError::Empty | mpsc::error::TryRecvError::Disconnected) => {
                    break;
                }
            }
        }
        if !sent_any {
            return false;
        }
        if send_err {
            self.mark_closing(conv_id, CloseReason::TransportError);
            return true;
        }
        flush_kcp_to_socket(session, socket, now_ms);
        if session.consecutive_send_failures >= SEND_FAILURE_LIMIT {
            self.mark_closing(conv_id, CloseReason::SendFailed);
            return true;
        }
        let due_ms = now_ms.saturating_add(i64::from(
            session.kcp.next_update_delay(session.elapsed_ms(now_ms)),
        ));
        self.deadlines.upsert(DeadlineEntry {
            conv_id,
            kind: DeadlineKind::KcpUpdate,
            due_ms,
        });
        true
    }

    fn process_deadline(
        &mut self,
        conv_id: u32,
        kind: DeadlineKind,
        socket: &std::net::UdpSocket,
        now_ms: i64,
    ) {
        let Some(session) = self.active.get_mut(&conv_id) else {
            return;
        };
        match kind {
            DeadlineKind::KcpUpdate => {
                let elapsed = session.elapsed_ms(now_ms);
                let addr = session.handle.address();
                let failures = &mut session.consecutive_send_failures;
                session.kcp.update(elapsed, &mut |pkt| {
                    send_udp(socket, addr, pkt, failures);
                });
                if session.kcp.is_dead() {
                    self.mark_closing(conv_id, CloseReason::KcpDeadLink);
                    return;
                }
                if session.consecutive_send_failures >= SEND_FAILURE_LIMIT {
                    self.mark_closing(conv_id, CloseReason::SendFailed);
                    return;
                }
                let due_ms = now_ms.saturating_add(
                    i64::from(session.kcp.next_update_delay(session.elapsed_ms(now_ms))).max(1),
                );
                self.deadlines.upsert(DeadlineEntry {
                    conv_id,
                    kind: DeadlineKind::KcpUpdate,
                    due_ms,
                });
            }
            DeadlineKind::Idle => {
                let due_ms = session.idle_deadline_ms();
                if now_ms >= due_ms {
                    let reason = if session.phase == Phase::PreAuth {
                        CloseReason::PreAuthTimeout
                    } else {
                        CloseReason::IdleTimeout
                    };
                    self.mark_closing(conv_id, reason);
                } else {
                    self.deadlines.upsert(DeadlineEntry {
                        conv_id,
                        kind: DeadlineKind::Idle,
                        due_ms,
                    });
                }
            }
        }
    }

    fn mark_closing(&mut self, conv_id: u32, reason: CloseReason) {
        self.deadlines.cancel_session(conv_id);
        if let Some(session) = self.active.remove(&conv_id) {
            session.handle.close(reason);
            self.closing.insert(conv_id, session);
        }
    }
}

fn send_udp(socket: &std::net::UdpSocket, addr: SocketAddr, buf: &[u8], failures: &mut u8) {
    match socket.send_to(buf, addr) {
        Ok(_) => *failures = 0,
        Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {}
        Err(err) => {
            *failures = failures.saturating_add(1);
            tracing::debug!(
                target: "kcp_transport",
                "send failed, endpoint: {}, data_len: {}, error: {}",
                addr,
                buf.len(),
                err
            );
        }
    }
}

fn flush_kcp_to_socket(session: &mut Session, socket: &std::net::UdpSocket, now_ms: i64) {
    let elapsed = session.elapsed_ms(now_ms);
    session.kcp.advance_clock(elapsed);
    let addr = session.handle.address();
    let failures = &mut session.consecutive_send_failures;
    session.kcp.flush(&mut |pkt| {
        send_udp(socket, addr, pkt, failures);
    });
}

fn pump_application_data(
    kcp: &mut Kcp,
    assembler: &mut StreamAssembler,
    application: &mut ApplicationInput,
) -> Result<Option<Vec<u8>>, StreamError> {
    loop {
        if application.bytes.is_none() && !application.receive(kcp) {
            return Ok(None);
        }
        if let Some(msg) = application.consume(assembler)? {
            return Ok(Some(msg));
        }
    }
}

pub fn registered_endpoint_matches(expected: &SocketAddr, actual: &SocketAddr) -> bool {
    expected == actual
}

pub fn is_loopback_endpoint(addr: &SocketAddr) -> bool {
    addr.ip().is_loopback()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn append_framed(buf: &mut Vec<u8>, body: &[u8]) {
        let len = (body.len() as u32).to_le_bytes();
        buf.extend_from_slice(&len[..3]);
        buf.extend_from_slice(body);
    }

    #[test]
    fn stream_assembler_accepts_split_prefixes_bodies_and_multiple_messages() {
        let mut assembler = StreamAssembler::new();
        let mut framed = Vec::new();
        append_framed(&mut framed, b"one");
        append_framed(&mut framed, b"split-body");

        assert!(assembler.consume(&framed[0..2]).message.is_none());
        let first = assembler.consume(&framed[2..6]);
        assert_eq!(first.message.as_deref(), Some(b"one".as_slice()));

        let second_start = 2 + first.consumed;
        assert!(assembler.consume(&framed[second_start..10]).message.is_none());
        let second = assembler.consume(&framed[10..]);
        assert_eq!(second.message.as_deref(), Some(b"split-body".as_slice()));
    }

    #[test]
    fn stream_assembler_frees_truncated_storage_on_reset() {
        let mut assembler = StreamAssembler::new();
        let framed = [5u8, 0, 0, 1, 2];
        assert!(assembler.consume(&framed).message.is_none());
        assembler.reset();
        assert!(assembler.body.is_none());
        assert_eq!(assembler.length_prefix_len, 0);
    }

    #[test]
    fn kcp_rejects_malformed_and_truncated_packet_input() {
        let mut kcp = Kcp::new(1);
        assert!(kcp.input(&[0u8; 23]).is_err());
        assert!(kcp.input(&[0u8; 24]).is_err());
    }

    #[test]
    fn indexed_transport_deadlines_update_and_order_deterministically() {
        let mut deadlines = DeadlineHeap::default();
        deadlines.upsert(DeadlineEntry {
            conv_id: 2,
            kind: DeadlineKind::Idle,
            due_ms: 100,
        });
        deadlines.upsert(DeadlineEntry {
            conv_id: 1,
            kind: DeadlineKind::Idle,
            due_ms: 100,
        });
        deadlines.upsert(DeadlineEntry {
            conv_id: 1,
            kind: DeadlineKind::KcpUpdate,
            due_ms: 100,
        });
        deadlines.upsert(DeadlineEntry {
            conv_id: 2,
            kind: DeadlineKind::Idle,
            due_ms: 50,
        });

        assert_eq!(deadlines.peek_due_ms(), Some(50));
        assert_eq!(deadlines.pop_due(50).unwrap().conv_id, 2);
        let first_tie = deadlines.pop_due(100).unwrap();
        assert_eq!(first_tie.conv_id, 1);
        assert_eq!(first_tie.kind, DeadlineKind::KcpUpdate);
        deadlines.cancel_session(1);
        assert!(deadlines.pop_due(100).is_none());
    }

    #[test]
    fn conversation_ids_are_monotonic_and_skip_active_values_after_wrap() {
        let mut registry = Registry::new();
        assert_eq!(registry.allocate_conv_id().unwrap(), 1);
        assert_eq!(registry.allocate_conv_id().unwrap(), 2);
        registry.next_conv_id = u32::MAX;
        assert_eq!(registry.allocate_conv_id().unwrap(), u32::MAX);
        assert_eq!(registry.allocate_conv_id().unwrap(), 3);
    }

    #[test]
    fn packet_routing_matches_the_complete_registered_endpoint() {
        let expected: SocketAddr = "127.0.0.1:7777".parse().unwrap();
        let same: SocketAddr = "127.0.0.1:7777".parse().unwrap();
        let other_port: SocketAddr = "127.0.0.1:7778".parse().unwrap();
        let other_address: SocketAddr = "127.0.0.2:7777".parse().unwrap();
        assert!(registered_endpoint_matches(&expected, &same));
        assert!(!registered_endpoint_matches(&expected, &other_port));
        assert!(!registered_endpoint_matches(&expected, &other_address));
    }

    #[test]
    fn stream_assembler_rejects_oversized_frames_without_allocating() {
        let mut assembler = StreamAssembler::new();
        let too_large = ((MAX_FRAME_LEN + 1) as u32).to_le_bytes();
        let res = assembler.consume(&too_large[..3]);
        assert_eq!(
            res.error,
            Some(StreamError::FrameTooLarge(MAX_FRAME_LEN + 1))
        );
        assert!(res.message.is_none());
        assert!(assembler.body.is_none());

        let max_u24 = [0xFFu8, 0xFF, 0xFF, 0x01, 0x02];
        let res2 = assembler.consume(&max_u24);
        assert_eq!(res2.error, Some(StreamError::FrameTooLarge(0x00FF_FFFF)));
        assert!(assembler.body.is_none());
    }

    #[test]
    fn pre_auth_caps_enforce_per_ip_and_release_on_auth() {
        let mut registry = Registry::new();
        let notify = Arc::new(Notify::new());
        let ip1: IpAddr = "198.51.100.10".parse().unwrap();
        let ip2: IpAddr = "198.51.100.11".parse().unwrap();

        let mut handles = Vec::new();
        for i in 0..MAX_PRE_AUTH_PER_IP {
            assert!(registry.can_accept_pre_auth(ip1));
            let conv_id = registry.allocate_conv_id().unwrap();
            let addr = SocketAddr::new(ip1, 10000 + i as u16);
            let ch = ConnectionHandle::open(conv_id, addr, Arc::clone(&notify));
            let h = Arc::new(ch.handle);
            registry.create(Arc::clone(&h), ch.outbound_rx, 100);
            handles.push(h);
        }
        assert!(!registry.can_accept_pre_auth(ip1));
        assert!(registry.can_accept_pre_auth(ip2));

        handles[0].mark_authenticated();
        assert!(registry.can_accept_pre_auth(ip1));
    }
}
