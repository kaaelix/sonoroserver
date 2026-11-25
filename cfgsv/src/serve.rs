//! HTTP request handling for `sonoro-cfgsv`.

use std::sync::Arc;

use axum::Router;
use axum::extract::State;
use axum::http::{HeaderValue, Method, StatusCode, Uri, header};
use axum::response::{IntoResponse, Response};
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use sonoro_common::FileSystem;

use crate::encryption::encrypt_aes256_cbc_base64;

const AES_KEY_SIZE: usize = 32;
const AES_IV_SIZE: usize = 16;

/// Shared application state for `cfgsv`.
#[derive(Clone)]
pub struct AppState {
    pub fs: Arc<FileSystem>,
}

/// Builds the Axum router serving config endpoints.
pub fn router(state: AppState) -> Router {
    Router::new().fallback(handle_request).with_state(state)
}

async fn handle_request(State(state): State<AppState>, method: Method, uri: Uri) -> Response {
    if method != Method::GET {
        return StatusCode::METHOD_NOT_ALLOWED.into_response();
    }

    let path = uri.path();
    tracing::info!(%path, "request");

    if !is_safe_request_path(path) {
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
        let app = router(AppState { fs: fs_state });

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
}
