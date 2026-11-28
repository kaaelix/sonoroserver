use std::env;
use std::io::{self, Read, Write};
use std::net::TcpStream;
#[cfg(unix)]
use std::os::unix::net::UnixStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

use sonoro_gamesv_logic::helpers::reward::RewardItem;

#[derive(Debug, Clone)]
pub struct PushFrame {
    pub msg_id: u16,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct LogicResponse {
    pub msg_id: u16,
    pub payload: Vec<u8>,
    pub pushes: Vec<PushFrame>,
    pub grants: Vec<RewardItem>,
}

enum BridgeStream {
    #[cfg(unix)]
    Unix(UnixStream),
    Tcp(TcpStream),
}

impl Read for BridgeStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            #[cfg(unix)]
            Self::Unix(s) => s.read(buf),
            Self::Tcp(s) => s.read(buf),
        }
    }
}

impl Write for BridgeStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            #[cfg(unix)]
            Self::Unix(s) => s.write(buf),
            Self::Tcp(s) => s.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            #[cfg(unix)]
            Self::Unix(s) => s.flush(),
            Self::Tcp(s) => s.flush(),
        }
    }
}

#[derive(Debug, Clone)]
struct BridgeConfig {
    unix_socket_path: String,
    tcp_fallback_addr: String,
}

impl Default for BridgeConfig {
    fn default() -> Self {
        Self {
            unix_socket_path: "/tmp/aki_logic.sock".to_string(),
            tcp_fallback_addr: "127.0.0.1:19999".to_string(),
        }
    }
}

const MAX_POOLED_STREAMS: usize = 8;

static BRIDGE_CONFIG: Mutex<Option<BridgeConfig>> = Mutex::new(None);
static STREAM_POOL: Mutex<Vec<BridgeStream>> = Mutex::new(Vec::new());
static UNSUPPORTED_LUT: OnceLock<Box<[AtomicBool]>> = OnceLock::new();

fn unsupported_lut() -> &'static [AtomicBool] {
    UNSUPPORTED_LUT
        .get_or_init(|| {
            (0..65536)
                .map(|_| AtomicBool::new(false))
                .collect::<Vec<_>>()
                .into_boxed_slice()
        })
}

pub fn configure_from_environment() {
    let mut cfg = BridgeConfig::default();
    if let Ok(path) = env::var("AKI_LOGIC_SOCKET_PATH")
        && !path.is_empty()
    {
        cfg.unix_socket_path = path;
    }
    if let Ok(addr) = env::var("AKI_LOGIC_TCP_ADDR")
        && !addr.is_empty()
    {
        cfg.tcp_fallback_addr = addr;
    }
    *BRIDGE_CONFIG.lock().unwrap_or_else(|e| e.into_inner()) = Some(cfg);
}

fn current_config() -> BridgeConfig {
    BRIDGE_CONFIG
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .clone()
        .unwrap_or_default()
}

fn connect_stream() -> io::Result<BridgeStream> {
    let cfg = current_config();
    let timeout = Some(Duration::from_secs(2));

    #[cfg(unix)]
    if let Ok(stream) = UnixStream::connect(&cfg.unix_socket_path) {
        let _ = stream.set_read_timeout(timeout);
        let _ = stream.set_write_timeout(timeout);
        return Ok(BridgeStream::Unix(stream));
    }

    let stream = TcpStream::connect(&cfg.tcp_fallback_addr)?;
    let _ = stream.set_nodelay(true);
    let _ = stream.set_read_timeout(timeout);
    let _ = stream.set_write_timeout(timeout);
    Ok(BridgeStream::Tcp(stream))
}

fn read_u16_le(r: &mut impl Read) -> io::Result<u16> {
    let mut buf = [0u8; 2];
    r.read_exact(&mut buf)?;
    Ok(u16::from_le_bytes(buf))
}

fn read_u32_le(r: &mut impl Read) -> io::Result<u32> {
    let mut buf = [0u8; 4];
    r.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

fn read_i32_le(r: &mut impl Read) -> io::Result<i32> {
    let mut buf = [0u8; 4];
    r.read_exact(&mut buf)?;
    Ok(i32::from_le_bytes(buf))
}

fn round_trip(
    stream: &mut BridgeStream,
    player_id: i32,
    flags: u8,
    msg_id: u16,
    payload: &[u8],
    request_sent: &mut bool,
    response_started: &mut bool,
) -> io::Result<LogicResponse> {
    let mut frame = Vec::with_capacity(11 + payload.len());
    frame.extend_from_slice(&player_id.to_le_bytes());
    frame.push(flags);
    frame.extend_from_slice(&msg_id.to_le_bytes());
    frame.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    frame.extend_from_slice(payload);
    stream.write_all(&frame)?;
    stream.flush()?;
    *request_sent = true;

    let mut resp_id_buf = [0u8; 2];
    stream.read_exact(&mut resp_id_buf[..1])?;
    *response_started = true;
    stream.read_exact(&mut resp_id_buf[1..])?;
    let resp_msg_id = u16::from_le_bytes(resp_id_buf);
    let resp_len = read_u32_le(stream)? as usize;
    let mut resp_bytes = vec![0u8; resp_len];
    if resp_len > 0 {
        stream.read_exact(&mut resp_bytes)?;
    }

    let push_count = read_u16_le(stream)? as usize;
    let mut pushes = Vec::with_capacity(push_count);
    for _ in 0..push_count {
        let p_id = read_u16_le(stream)?;
        let p_len = read_u32_le(stream)? as usize;
        let mut p_bytes = vec![0u8; p_len];
        if p_len > 0 {
            stream.read_exact(&mut p_bytes)?;
        }
        pushes.push(PushFrame {
            msg_id: p_id,
            payload: p_bytes,
        });
    }

    let grant_count = read_u16_le(stream)? as usize;
    let mut grants = Vec::with_capacity(grant_count);
    for _ in 0..grant_count {
        let item_id = read_i32_le(stream)?;
        let count = read_i32_le(stream)?;
        grants.push(RewardItem { item_id, count });
    }

    if resp_msg_id == 0 && msg_id != 0 {
        return Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "unhandled by logic service",
        ));
    }

    Ok(LogicResponse {
        msg_id: resp_msg_id,
        payload: resp_bytes,
        pushes,
        grants,
    })
}

fn pop_pooled_from(pool: &Mutex<Vec<BridgeStream>>) -> Option<BridgeStream> {
    pool.lock().unwrap_or_else(|e| e.into_inner()).pop()
}

fn push_pooled_to(pool: &Mutex<Vec<BridgeStream>>, stream: BridgeStream) {
    let mut guard = pool.lock().unwrap_or_else(|e| e.into_inner());
    if guard.len() < MAX_POOLED_STREAMS {
        guard.push(stream);
    }
}

fn in_process_enabled() -> bool {
    static IN_PROCESS_ENABLED: OnceLock<bool> = OnceLock::new();
    *IN_PROCESS_ENABLED.get_or_init(|| env::var_os("AKI_LOGIC_FORCE_IPC").is_none())
}

fn call_ipc_with(
    pool: &Mutex<Vec<BridgeStream>>,
    connect: impl Fn() -> io::Result<BridgeStream>,
    player_id: i32,
    flags: u8,
    msg_id: u16,
    payload: &[u8],
) -> io::Result<LogicResponse> {
    while let Some(mut stream) = pop_pooled_from(pool) {
        let mut request_sent = false;
        let mut response_started = false;
        match round_trip(
            &mut stream,
            player_id,
            flags,
            msg_id,
            payload,
            &mut request_sent,
            &mut response_started,
        ) {
            Ok(resp) => {
                push_pooled_to(pool, stream);
                return Ok(resp);
            }
            Err(err) => {
                if err.kind() == io::ErrorKind::Unsupported {
                    if msg_id != 0 {
                        unsupported_lut()[msg_id as usize].store(true, Ordering::Relaxed);
                    }
                    push_pooled_to(pool, stream);
                    return Err(err);
                }
                let retryable_idle_disconnect = !response_started
                    && (!request_sent
                        || matches!(
                            err.kind(),
                            io::ErrorKind::UnexpectedEof
                                | io::ErrorKind::ConnectionReset
                                | io::ErrorKind::BrokenPipe
                        ));
                if !retryable_idle_disconnect {
                    return Err(err);
                }
            }
        }
    }

    let mut fresh = connect()?;
    let mut request_sent = false;
    let mut response_started = false;
    match round_trip(
        &mut fresh,
        player_id,
        flags,
        msg_id,
        payload,
        &mut request_sent,
        &mut response_started,
    ) {
        Ok(resp) => {
            push_pooled_to(pool, fresh);
            Ok(resp)
        }
        Err(err) => {
            if err.kind() == io::ErrorKind::Unsupported {
                if msg_id != 0 {
                    unsupported_lut()[msg_id as usize].store(true, Ordering::Relaxed);
                }
                push_pooled_to(pool, fresh);
            }
            Err(err)
        }
    }
}

pub fn call_player(
    player_id: i32,
    flags: u8,
    msg_id: u16,
    payload: &[u8],
) -> io::Result<LogicResponse> {
    if msg_id != 0 && unsupported_lut()[msg_id as usize].load(Ordering::Relaxed) {
        return Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "unhandled by logic service",
        ));
    }

    if in_process_enabled() {
        return match sonoro_aki_logic::dispatch_in_process(player_id, msg_id, payload) {
            Some(res) => Ok(LogicResponse {
                msg_id: res.resp_msg_id,
                payload: res.resp_bytes,
                pushes: res
                    .pushes
                    .into_iter()
                    .map(|(p_id, p_bytes)| PushFrame {
                        msg_id: p_id,
                        payload: p_bytes,
                    })
                    .collect(),
                grants: res
                    .grants
                    .into_iter()
                    .map(|(item_id, count)| RewardItem { item_id, count })
                    .collect(),
            }),
            None => {
                if msg_id != 0 {
                    unsupported_lut()[msg_id as usize].store(true, Ordering::Relaxed);
                }
                Err(io::Error::new(
                    io::ErrorKind::Unsupported,
                    "unhandled by logic service",
                ))
            }
        };
    }

    call_ipc_with(&STREAM_POOL, connect_stream, player_id, flags, msg_id, payload)
}

pub fn warm_up_in_process() {
    if in_process_enabled() {
        let _ = sonoro_aki_logic::dispatch_in_process(0, 0, &[]);
    }
}

pub fn ping() -> bool {
    call_player(0, 0, 0, &[]).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;

    #[test]
    fn pooled_stream_eof_retries_with_fresh_connection() {
        let stale_listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let stale_addr = stale_listener.local_addr().unwrap();
        let stale_client = TcpStream::connect(stale_addr).unwrap();
        let (stale_srv, _) = stale_listener.accept().unwrap();
        drop(stale_srv);

        let live_listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let live_addr = live_listener.local_addr().unwrap();
        let srv_thread = std::thread::spawn(move || {
            let (mut conn, _) = live_listener.accept().unwrap();
            let mut hdr = [0u8; 11];
            conn.read_exact(&mut hdr).unwrap();
            // resp_msg_id=0, resp_len=1, body=[1], push_count=0, grant_count=0
            conn.write_all(&[0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0]).unwrap();
            conn.flush().unwrap();
        });

        let pool = Mutex::new(vec![BridgeStream::Tcp(stale_client)]);
        let resp = call_ipc_with(
            &pool,
            || {
                let s = TcpStream::connect(live_addr)?;
                s.set_read_timeout(Some(Duration::from_secs(2)))?;
                Ok(BridgeStream::Tcp(s))
            },
            0,
            0,
            0,
            &[],
        )
        .expect("should reconnect on stale pooled EOF");
        assert_eq!(resp.msg_id, 0);
        assert_eq!(resp.payload, vec![1]);
        srv_thread.join().unwrap();
    }
}
