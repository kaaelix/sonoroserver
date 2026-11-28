//! HTTP request handling for `sonoro-cfgsv`.

use std::sync::Arc;
use std::time::Duration;

use axum::Router;
use axum::body::{Body, to_bytes};
use axum::extract::State;
use axum::http::{HeaderMap, HeaderName, HeaderValue, Method, StatusCode, Uri, header};
use axum::response::{IntoResponse, Response};
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use sonoro_common::FileSystem;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::encryption::encrypt_aes256_cbc_base64;

const AES_KEY_SIZE: usize = 32;
const AES_IV_SIZE: usize = 16;
const DEFAULT_GM_UPSTREAM: &str = "127.0.0.1:13101";
const GM_PROXY_MAX_BODY: usize = 128 * 1024;
const GM_PROXY_MAX_RESPONSE: usize = 16 * 1024 * 1024;
const GM_UPSTREAM_CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
const GM_UPSTREAM_READ_TIMEOUT: Duration = Duration::from_secs(10);

/// Shared application state for `cfgsv`.
#[derive(Clone)]
pub struct AppState {
    pub fs: Arc<FileSystem>,
    pub gm_upstream: String,
}

impl AppState {
    pub fn new(fs: Arc<FileSystem>) -> Self {
        let gm_upstream = std::env::var("GM_HTTP_UPSTREAM")
            .ok()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| String::from(DEFAULT_GM_UPSTREAM));
        Self { fs, gm_upstream }
    }
}

/// Builds the Axum router serving config endpoints and proxying `/gm` requests to `gamesv`.
pub fn router(state: AppState) -> Router {
    Router::new().fallback(handle_request).with_state(state)
}

fn is_gm_proxy_path(path: &str) -> bool {
    path == "/gm"
        || path == "/gm/"
        || path.starts_with("/gm/")
        || path == "/announcement"
        || path == "/aki-logic/status"
}

async fn proxy_to_gm_upstream(
    upstream: &str,
    method: Method,
    uri: &Uri,
    headers: &HeaderMap,
    body: Body,
) -> Response {
    proxy_to_gm_upstream_with_limits(
        upstream,
        method,
        uri,
        headers,
        body,
        GM_UPSTREAM_CONNECT_TIMEOUT,
        GM_UPSTREAM_READ_TIMEOUT,
        GM_PROXY_MAX_RESPONSE,
    )
    .await
}

#[allow(clippy::too_many_arguments)]
async fn proxy_to_gm_upstream_with_limits(
    upstream: &str,
    method: Method,
    uri: &Uri,
    headers: &HeaderMap,
    body: Body,
    connect_timeout: Duration,
    io_timeout: Duration,
    max_resp_bytes: usize,
) -> Response {
    if !matches!(method, Method::GET | Method::POST | Method::OPTIONS) {
        return StatusCode::METHOD_NOT_ALLOWED.into_response();
    }
    let body_bytes = match to_bytes(body, GM_PROXY_MAX_BODY).await {
        Ok(b) => b,
        Err(_) => return StatusCode::PAYLOAD_TOO_LARGE.into_response(),
    };
    let mut stream =
        match tokio::time::timeout(connect_timeout, tokio::net::TcpStream::connect(upstream)).await
        {
            Ok(Ok(s)) => s,
            Ok(Err(err)) => {
                tracing::warn!(%upstream, error = %err, "failed to connect to GM upstream");
                return (
                    StatusCode::BAD_GATEWAY,
                    r#"{"ok":false,"message":"GM service unavailable"}"#,
                )
                    .into_response();
            }
            Err(_) => {
                tracing::warn!(%upstream, "timed out connecting to GM upstream");
                return (
                    StatusCode::GATEWAY_TIMEOUT,
                    r#"{"ok":false,"message":"GM service timeout"}"#,
                )
                    .into_response();
            }
        };

    let target = uri
        .path_and_query()
        .map(|pq| pq.as_str())
        .unwrap_or(uri.path());
    let mut req_head = format!(
        "{method} {target} HTTP/1.1\r\nHost: {upstream}\r\nConnection: close\r\nContent-Length: {}\r\n",
        body_bytes.len()
    );
    for key in ["content-type", "x-gm-sign", "x-gm-admin", "accept"] {
        if let Some(val) = headers.get(key).and_then(|v| v.to_str().ok()) {
            req_head.push_str(&format!("{key}: {val}\r\n"));
        }
    }
    req_head.push_str("\r\n");

    let write_res = tokio::time::timeout(io_timeout, async {
        stream.write_all(req_head.as_bytes()).await?;
        if !body_bytes.is_empty() {
            stream.write_all(&body_bytes).await?;
        }
        Ok::<(), std::io::Error>(())
    })
    .await;
    match write_res {
        Ok(Ok(())) => {}
        Ok(Err(err)) => {
            tracing::warn!(%upstream, error = %err, "failed writing request to GM upstream");
            return StatusCode::BAD_GATEWAY.into_response();
        }
        Err(_) => {
            tracing::warn!(%upstream, "timed out writing request to GM upstream");
            return StatusCode::GATEWAY_TIMEOUT.into_response();
        }
    }

    let mut raw_resp = Vec::new();
    let limit = u64::try_from(max_resp_bytes.saturating_add(1)).unwrap_or(u64::MAX);
    let read_res =
        tokio::time::timeout(io_timeout, stream.take(limit).read_to_end(&mut raw_resp)).await;
    match read_res {
        Ok(Ok(_)) => {}
        Ok(Err(err)) => {
            tracing::warn!(%upstream, error = %err, "failed reading response from GM upstream");
            return StatusCode::BAD_GATEWAY.into_response();
        }
        Err(_) => {
            tracing::warn!(%upstream, "timed out reading response from GM upstream");
            return StatusCode::GATEWAY_TIMEOUT.into_response();
        }
    }
    if raw_resp.len() > max_resp_bytes {
        tracing::warn!(
            %upstream,
            bytes = raw_resp.len(),
            max_resp_bytes,
            "GM upstream response exceeded size limit"
        );
        return StatusCode::BAD_GATEWAY.into_response();
    }

    let Some(split_pos) = raw_resp.windows(4).position(|w| w == b"\r\n\r\n") else {
        return StatusCode::BAD_GATEWAY.into_response();
    };
    let head_str = String::from_utf8_lossy(&raw_resp[..split_pos]);
    let resp_body = raw_resp[split_pos + 4..].to_vec();

    let mut lines = head_str.lines();
    let status_code = lines
        .next()
        .and_then(|l| l.split_whitespace().nth(1))
        .and_then(|s| s.parse::<u16>().ok())
        .and_then(|c| StatusCode::from_u16(c).ok())
        .unwrap_or(StatusCode::BAD_GATEWAY);

    let mut resp = Response::builder().status(status_code);
    for line in lines {
        if let Some((k, v)) = line.split_once(':') {
            let k_lower = k.trim().to_ascii_lowercase();
            if matches!(
                k_lower.as_str(),
                "content-length" | "connection" | "transfer-encoding"
            ) {
                continue;
            }
            if let (Ok(hn), Ok(hv)) = (
                HeaderName::from_bytes(k.trim().as_bytes()),
                HeaderValue::from_str(v.trim()),
            ) {
                resp = resp.header(hn, hv);
            }
        }
    }
    resp.body(Body::from(resp_body))
        .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())
}

async fn handle_request(
    State(state): State<AppState>,
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    body: Body,
) -> Response {
    let path = uri.path().to_string();
    if is_gm_proxy_path(&path) {
        tracing::info!(%path, %method, "proxying GM request");
        return proxy_to_gm_upstream(&state.gm_upstream, method, &uri, &headers, body).await;
    }

    if method != Method::GET {
        return StatusCode::METHOD_NOT_ALLOWED.into_response();
    }

    tracing::info!(%path, "request");

    if !is_safe_request_path(&path) {
        tracing::warn!(%path, "rejected unsafe request path");
        return StatusCode::NOT_FOUND.into_response();
    }

    let rel_path = format!("cfg/index/{}", &path[1..]);
    let content = match state.fs.read_file(&rel_path) {
        Ok(Some(bytes)) => bytes,
        Ok(None) => {
            tracing::warn!(file = &path[1..], "requested file not found");
            return StatusCode::NOT_FOUND.into_response();
        }
        Err(err) => {
            tracing::error!(error = %err, "failed to read config file");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let (key, iv) = match read_aes_parameters(&state.fs) {
        Ok(params) => params,
        Err(err) => {
            tracing::error!(error = %err, "failed to load AES key/iv");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let body = encrypt_aes256_cbc_base64(&content, &key, &iv);
    (
        StatusCode::OK,
        [(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        )],
        body,
    )
        .into_response()
}

/// Validates that `path` is a clean, relative URL path with no traversal segments.
pub fn is_safe_request_path(path: &str) -> bool {
    if path.len() <= 1 || !path.starts_with('/') {
        return false;
    }
    if path.starts_with("//") || path.contains(['\\', '\0']) {
        return false;
    }
    path[1..]
        .split('/')
        .all(|seg| !seg.is_empty() && seg != "." && seg != "..")
}

fn read_aes_parameters(fs: &FileSystem) -> Result<([u8; AES_KEY_SIZE], [u8; AES_IV_SIZE]), String> {
    let key = decode_b64_fixed::<AES_KEY_SIZE>(fs, "cfg/key", "key")?;
    let iv = decode_b64_fixed::<AES_IV_SIZE>(fs, "cfg/iv", "iv")?;
    Ok((key, iv))
}

fn decode_b64_fixed<const N: usize>(
    fs: &FileSystem,
    rel_path: &str,
    label: &str,
) -> Result<[u8; N], String> {
    let raw = fs
        .read_file(rel_path)
        .map_err(|e| format!("failed reading {label} file: {e}"))?
        .ok_or_else(|| format!("{label} file does not exist"))?;
    let text = std::str::from_utf8(&raw)
        .map_err(|_| format!("invalid utf-8 in {label} file"))?
        .trim_matches(|c: char| matches!(c, ' ' | '\t' | '\r' | '\n'));
    let decoded = BASE64
        .decode(text)
        .map_err(|_| format!("invalid base64 in {label} file"))?;
    decoded
        .try_into()
        .map_err(|_| format!("invalid {label} length (expected {N} bytes)"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn make_temp_state() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("sonoro_cfgsv_test_{nanos}"));
        fs::create_dir_all(dir.join("cfg/index")).expect("mkdir");
        fs::write(
            dir.join("cfg/key"),
            "t+AEu5SGdpz06tomonajLMau9AJgmyTvVhz9VtGf1+0=\n",
        )
        .expect("write key");
        fs::write(dir.join("cfg/iv"), "fprc5lBWADQB7tim0R2JxQ==\n").expect("write iv");
        fs::write(dir.join("cfg/index/server.json"), br#"{"status":"ok"}"#).expect("write index");
        dir
    }

    #[test]
    fn validates_request_paths() {
        assert!(is_safe_request_path("/index.json"));
        assert!(is_safe_request_path("/a/b/c.json"));
        assert!(!is_safe_request_path("/"));
        assert!(!is_safe_request_path("//etc/passwd"));
        assert!(!is_safe_request_path("/../secret"));
        assert!(!is_safe_request_path("/a/./b"));
        assert!(!is_safe_request_path("/a\\b"));
    }

    #[tokio::test]
    async fn serves_encrypted_config_and_rejects_bad_methods() {
        let dir = make_temp_state();
        let fs_state = Arc::new(FileSystem::open(&dir).expect("open fs"));
        let app = router(AppState::new(fs_state));

        let req = Request::builder()
            .method(Method::GET)
            .uri("/server.json")
            .body(Body::empty())
            .expect("req");
        let resp = tower_call(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let post_req = Request::builder()
            .method(Method::POST)
            .uri("/server.json")
            .body(Body::empty())
            .expect("req");
        let post_resp = tower_call(&app, post_req).await;
        assert_eq!(post_resp.status(), StatusCode::METHOD_NOT_ALLOWED);

        let _ = fs::remove_dir_all(&dir);
    }

    async fn tower_call(app: &Router, req: Request<Body>) -> Response {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind");
        let addr = listener.local_addr().expect("local addr");
        let app_clone = app.clone();
        let server = tokio::spawn(async move {
            let _ = axum::serve(listener, app_clone).await;
        });

        let mut stream = tokio::net::TcpStream::connect(addr).await.expect("connect");
        let (method, uri): (Method, Uri) = (req.method().clone(), req.uri().clone());
        let raw =
            format!("{method} {uri} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n");
        stream.write_all(raw.as_bytes()).await.expect("write");
        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).await.expect("read");
        server.abort();

        let text = String::from_utf8_lossy(&buf);
        let status_code = text
            .lines()
            .next()
            .and_then(|l| l.split_whitespace().nth(1))
            .and_then(|s| s.parse::<u16>().ok())
            .expect("status code");
        StatusCode::from_u16(status_code)
            .expect("valid status")
            .into_response()
    }

    #[tokio::test]
    async fn gm_proxy_enforces_timeouts_and_response_size_cap() {
        let slow_listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind slow");
        let slow_addr = slow_listener.local_addr().expect("addr");
        let slow_srv = tokio::spawn(async move {
            if let Ok((_sock, _)) = slow_listener.accept().await {
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        });

        let timeout_resp = proxy_to_gm_upstream_with_limits(
            &slow_addr.to_string(),
            Method::GET,
            &"/gm".parse::<Uri>().expect("uri"),
            &HeaderMap::new(),
            Body::empty(),
            Duration::from_millis(200),
            Duration::from_millis(50),
            1024,
        )
        .await;
        assert_eq!(timeout_resp.status(), StatusCode::GATEWAY_TIMEOUT);
        slow_srv.abort();

        let big_listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind big");
        let big_addr = big_listener.local_addr().expect("addr");
        let big_srv = tokio::spawn(async move {
            if let Ok((mut sock, _)) = big_listener.accept().await {
                let mut req = [0u8; 512];
                let _ = sock.read(&mut req).await;
                let payload = vec![b'A'; 4096];
                let head = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    payload.len()
                );
                let _ = sock.write_all(head.as_bytes()).await;
                let _ = sock.write_all(&payload).await;
            }
        });

        let big_resp = proxy_to_gm_upstream_with_limits(
            &big_addr.to_string(),
            Method::GET,
            &"/gm".parse::<Uri>().expect("uri"),
            &HeaderMap::new(),
            Body::empty(),
            Duration::from_secs(1),
            Duration::from_secs(1),
            1024,
        )
        .await;
        assert_eq!(big_resp.status(), StatusCode::BAD_GATEWAY);
        big_srv.abort();
    }
}
