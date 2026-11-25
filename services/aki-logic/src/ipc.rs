//! Binary IPC v3 server matching `gamesv/src/network/aki_logic_bridge.zig`.

use std::collections::HashMap;
use std::future::Future;
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::sync::Arc;

use sonoro_proto::PbMessage;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::watch;

use crate::handlers::{self, HandlerFn};

/// Result returned by an IPC handler (primary response, optional server pushes, optional item grants).
#[derive(Debug, Default, Clone)]
pub struct AkiResult {
    pub resp_msg_id: u16,
    pub resp_bytes: Vec<u8>,
    pub pushes: Vec<(u16, Vec<u8>)>,
    pub grants: Vec<(i32, i32)>,
}

impl AkiResult {
    /// Creates an unhandled sentinel result (`resp_msg_id = 0`, empty payload).
    pub fn unhandled() -> Self {
        Self::default()
    }

    /// Encodes a protobuf response message with its wire `resp_id`.
    pub fn ok<M: PbMessage>(resp_id: u16, msg: &M) -> Self {
        Self {
            resp_msg_id: resp_id,
            resp_bytes: msg.encode_to_vec(),
            pushes: Vec::new(),
            grants: Vec::new(),
        }
    }

    /// Returns pre-encoded raw response bytes.
    pub fn ok_raw(resp_id: u16, bytes: Vec<u8>) -> Self {
        Self {
            resp_msg_id: resp_id,
            resp_bytes: bytes,
            pushes: Vec::new(),
            grants: Vec::new(),
        }
    }

    /// Appends an encoded push frame to deliver alongside the primary response.
    pub fn with_push<M: PbMessage>(mut self, msg_id: u16, msg: &M) -> Self {
        self.pushes.push((msg_id, msg.encode_to_vec()));
        self
    }

    /// Appends an item grant `(item_id, count)` when both are positive.
    pub fn with_grant(mut self, item_id: i32, count: i32) -> Self {
        if item_id > 0 && count > 0 {
            self.grants.push((item_id, count));
        }
        self
    }

    /// Appends multiple item grants.
    pub fn with_grants<I: IntoIterator<Item = (i32, i32)>>(mut self, items: I) -> Self {
        for (id, cnt) in items {
            self = self.with_grant(id, cnt);
        }
        self
    }
}

/// Dual-transport IPC v3 server (Unix domain socket on Unix + TCP loopback everywhere).
pub struct IpcServer {
    socket_path: PathBuf,
    tcp_port: u16,
    handlers: Arc<HashMap<u16, HandlerFn>>,
}

impl IpcServer {
    pub fn new(socket_path: impl Into<PathBuf>, tcp_port: u16) -> Self {
        Self {
            socket_path: socket_path.into(),
            tcp_port,
            handlers: Arc::new(handlers::build_handlers()),
        }
    }

    /// Starts listeners and serves until `shutdown` resolves.
    pub async fn run<F>(self, shutdown: F) -> io::Result<()>
    where
        F: Future<Output = ()> + Send,
    {
        let (stop_tx, stop_rx) = watch::channel(false);

        #[cfg(unix)]
        let unix_handle = {
            if self.socket_path.exists() {
                let _ = std::fs::remove_file(&self.socket_path);
            }
            match tokio::net::UnixListener::bind(&self.socket_path) {
                Ok(listener) => {
                    println!(
                        "[AkiLogicService] IPC Server v3 listening on Unix Domain Socket: {} ({} rich handlers)",
                        self.socket_path.display(),
                        handlers::rich_handler_count()
                    );
                    let handlers = Arc::clone(&self.handlers);
                    let mut rx = stop_rx.clone();
                    Some(tokio::spawn(async move {
                        loop {
                            tokio::select! {
                                _ = rx.changed() => break,
                                accepted = listener.accept() => {
                                    match accepted {
                                        Ok((stream, _)) => {
                                            let h = Arc::clone(&handlers);
                                            let c_rx = rx.clone();
                                            tokio::spawn(handle_client(stream, h, c_rx));
                                        }
                                        Err(err) => {
                                            tracing::warn!(error = %err, "Unix accept error");
                                        }
                                    }
                                }
                            }
                        }
                    }))
                }
                Err(err) => {
                    tracing::warn!(path = %self.socket_path.display(), error = %err, "Unix socket init warning");
                    None
                }
            }
        };

        let tcp_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), self.tcp_port);
        let tcp_listener = TcpListener::bind(tcp_addr).await?;
        println!(
            "[AkiLogicService] IPC Server v3 listening on TCP Loopback: {tcp_addr} ({} rich handlers)",
            handlers::rich_handler_count()
        );

        let tcp_handlers = Arc::clone(&self.handlers);
        let mut tcp_rx = stop_rx;
        let tcp_handle = tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = tcp_rx.changed() => break,
                    accepted = tcp_listener.accept() => {
                        match accepted {
                            Ok((stream, _)) => {
                                let _ = stream.set_nodelay(true);
                                let h = Arc::clone(&tcp_handlers);
                                let c_rx = tcp_rx.clone();
                                tokio::spawn(handle_client(stream, h, c_rx));
                            }
                            Err(err) => {
                                tracing::warn!(error = %err, "TCP accept error");
                            }
                        }
                    }
                }
            }
        });

        shutdown.await;
        let _ = stop_tx.send(true);
        let _ = tcp_handle.await;

        #[cfg(unix)]
        {
            if let Some(handle) = unix_handle {
                let _ = handle.await;
            }
            if self.socket_path.exists() {
                let _ = std::fs::remove_file(&self.socket_path);
            }
        }

        Ok(())
    }
}

async fn handle_client<S>(
    mut stream: S,
    handlers: Arc<HashMap<u16, HandlerFn>>,
    mut stop_rx: watch::Receiver<bool>,
) where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    let mut header = [0u8; 11];
    loop {
        let read_res = tokio::select! {
            _ = stop_rx.changed() => break,
            res = stream.read_exact(&mut header) => res,
        };

        match read_res {
            Ok(_) => {}
            Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => break,
            Err(err) => {
                tracing::debug!(error = %err, "IPC client read error");
                break;
            }
        }

        let player_id = i32::from_le_bytes([header[0], header[1], header[2], header[3]]);
        let _flags = header[4];
        let msg_id = u16::from_le_bytes([header[5], header[6]]);
        let payload_len =
            u32::from_le_bytes([header[7], header[8], header[9], header[10]]) as usize;

        let mut payload = vec![0u8; payload_len];
        if payload_len > 0
            && let Err(err) = stream.read_exact(&mut payload).await
        {
            tracing::debug!(error = %err, "IPC client payload read error");
            break;
        }

        let result = if let Some(&handler) = handlers.get(&msg_id) {
            tokio::task::block_in_place(|| handler(player_id, &payload))
        } else {
            AkiResult::unhandled()
        };

        if let Err(err) = write_ipc_v3_response(&mut stream, &result).await {
            tracing::debug!(error = %err, "IPC client write error");
            break;
        }
    }
}

async fn write_ipc_v3_response<W: AsyncWrite + Unpin>(
    stream: &mut W,
    res: &AkiResult,
) -> io::Result<()> {
    let mut frame = Vec::with_capacity(
        10 + res.resp_bytes.len()
            + res.pushes.iter().map(|(_, b)| 6 + b.len()).sum::<usize>()
            + res.grants.len() * 8,
    );

    frame.extend_from_slice(&res.resp_msg_id.to_le_bytes());
    frame.extend_from_slice(&(res.resp_bytes.len() as u32).to_le_bytes());
    frame.extend_from_slice(&res.resp_bytes);

    frame.extend_from_slice(&(res.pushes.len() as u16).to_le_bytes());
    for (p_id, p_bytes) in &res.pushes {
        frame.extend_from_slice(&p_id.to_le_bytes());
        frame.extend_from_slice(&(p_bytes.len() as u32).to_le_bytes());
        frame.extend_from_slice(p_bytes);
    }

    frame.extend_from_slice(&(res.grants.len() as u16).to_le_bytes());
    for &(item_id, count) in &res.grants {
        frame.extend_from_slice(&item_id.to_le_bytes());
        frame.extend_from_slice(&count.to_le_bytes());
    }

    stream.write_all(&frame).await?;
    stream.flush().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn handles_ping_over_ipc_v3_frame() {
        let (mut client, server_side) = tokio::io::duplex(256);
        let handlers = Arc::new(handlers::build_handlers());
        let (_tx, rx) = watch::channel(false);
        let task = tokio::spawn(handle_client(server_side, handlers, rx));

        // Ping request: player_id=0, flags=0, msg_id=0, payload_len=0
        client.write_all(&[0u8; 11]).await.expect("write ping");
        let mut resp_hdr = [0u8; 6];
        client.read_exact(&mut resp_hdr).await.expect("read hdr");
        let resp_id = u16::from_le_bytes([resp_hdr[0], resp_hdr[1]]);
        let resp_len = u32::from_le_bytes([resp_hdr[2], resp_hdr[3], resp_hdr[4], resp_hdr[5]]);
        assert_eq!(resp_id, 0);
        assert_eq!(resp_len, 1);

        let mut body = [0u8; 1];
        client.read_exact(&mut body).await.expect("read body");
        assert_eq!(body[0], 1);

        let mut counts = [0u8; 4];
        client.read_exact(&mut counts).await.expect("read counts");
        assert_eq!(counts, [0, 0, 0, 0]);

        drop(client);
        let _ = task.await;
    }
}
