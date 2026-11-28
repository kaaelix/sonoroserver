//! HTTP request routing and login JSON handling for `sonoro-loginsv`.

use std::sync::Arc;

use axum::Router;
use axum::extract::{RawQuery, State};
use axum::http::{HeaderValue, Method, StatusCode, Uri, header};
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use sonoro_common::FileSystem;

use crate::account::Account;

/// Gateway endpoint returned to game clients after login.
#[derive(Debug, Clone, Serialize)]
pub struct ClientGatewayConfig {
    pub host: String,
    pub port: u16,
}

/// Shared application state for `loginsv`.
#[derive(Clone)]
pub struct AppState {
    pub fs: Arc<FileSystem>,
    pub gateway: ClientGatewayConfig,
}

#[derive(Debug, PartialEq, Eq)]
struct LoginParameters<'a> {
    login_type: u8,
    user_id: &'a str,
    user_name: &'a str,
    token: &'a str,
    user_data: i32,
    device_id: Option<&'a str>,
    login_trace_id: Option<&'a str>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct LoginResult {
    code: i32,
    has_rpc: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    err_message: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    udp_hosts: Option<[ClientGatewayConfig; 1]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_data: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ban_time_stamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sex: Option<i32>,
}

impl LoginResult {
    fn error(code: i32, message: &'static str, user_data: i32) -> Self {
        Self {
            code,
            has_rpc: true,
            err_message: Some(message),
            token: None,
            udp_hosts: None,
            user_data: Some(user_data),
            ban_time_stamp: None,
            sex: None,
        }
    }
}

/// Builds the Axum router serving `/health` and `/api/login`.
pub fn router(state: AppState) -> Router {
    Router::new().fallback(handle_request).with_state(state)
}

async fn handle_request(
    State(state): State<AppState>,
    method: Method,
    uri: Uri,
    RawQuery(raw_query): RawQuery,
) -> Response {
    if method != Method::GET && method != Method::POST {
        return StatusCode::METHOD_NOT_ALLOWED.into_response();
    }

    match uri.path() {
        "/health" => StatusCode::OK.into_response(),
        "/api/login" => {
            let query = raw_query.as_deref().unwrap_or("");
            let params = match parse_query(query) {
                Ok(p) => p,
                Err(reason) => {
                    tracing::warn!(%reason, query, "bad login request");
                    return StatusCode::BAD_REQUEST.into_response();
                }
            };

            let result = process_login(&state.fs, &state.gateway, params);
            match serde_json::to_string(&result) {
                Ok(body) => (
                    StatusCode::OK,
                    [(
                        header::CONTENT_TYPE,
                        HeaderValue::from_static("application/json"),
                    )],
                    body,
                )
                    .into_response(),
                Err(err) => {
                    tracing::error!(error = %err, "failed to serialize login response");
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        }
        other => {
            tracing::warn!(path = other, "unhandled request path");
            StatusCode::NOT_FOUND.into_response()
        }
    }
}

fn process_login(
    fs: &FileSystem,
    gateway: &ClientGatewayConfig,
    params: LoginParameters<'_>,
) -> LoginResult {
    tracing::debug!(
        login_type = params.login_type,
        user_id = params.user_id,
        user_name = params.user_name,
        token = params.token,
        user_data = params.user_data,
        device_id = ?params.device_id,
        login_trace_id = ?params.login_trace_id,
        "login request"
    );

    let account = match Account::load_or_create(fs, params.user_id, params.user_name, params.token)
    {
        Ok(acc) => acc,
        Err(err) => {
            tracing::error!(error = %err, "login failed due to an internal error");
            return LoginResult::error(-1, "Internal Server Error", params.user_data);
        }
    };

    // Ensure account token is persisted on disk before gamesv verifies it.
    fs.flush();

    LoginResult {
        code: 0,
        has_rpc: true,
        err_message: None,
        token: Some(account.token),
        udp_hosts: Some([gateway.clone()]),
        user_data: Some(params.user_data),
        ban_time_stamp: None,
        sex: Some(account.sex),
    }
}

fn parse_query(query: &str) -> Result<LoginParameters<'_>, &'static str> {
    let mut login_type: Option<u8> = None;
    let mut user_id: Option<&str> = None;
    let mut user_name: Option<&str> = None;
    let mut token: Option<&str> = None;
    let mut user_data: Option<i32> = None;
    let mut device_id: Option<&str> = None;
    let mut login_trace_id: Option<&str> = None;

    for param in query.split('&').filter(|s| !s.is_empty()) {
        let mut parts = param.split('=');
        let name = parts.next().ok_or("malformed query")?;
        let value = parts.next().ok_or("malformed query")?;
        if parts.next().is_some() {
            return Err("malformed query");
        }

        match name {
            "loginType" => {
                let parsed: u8 = value.parse().map_err(|_| "invalid parameter")?;
                if parsed > 1 {
                    return Err("invalid parameter");
                }
                login_type = Some(parsed);
            }
            "userId" => user_id = Some(value),
            "userName" => user_name = Some(value),
            "token" => token = Some(value),
            "userData" => {
                user_data = Some(value.parse().map_err(|_| "invalid parameter")?);
            }
            "deviceId" => device_id = Some(value),
            "loginTraceId" => login_trace_id = Some(value),
            _ => {}
        }
    }

    Ok(LoginParameters {
        login_type: login_type.ok_or("missing parameters")?,
        user_id: user_id.ok_or("missing parameters")?,
        user_name: user_name.ok_or("missing parameters")?,
        token: token.ok_or("missing parameters")?,
        user_data: user_data.ok_or("missing parameters")?,
        device_id,
        login_trace_id,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_login_query_and_ignores_unknown_keys() {
        let q =
            "loginType=1&userId=597938510&userName=U607938510A&token=abc&userData=42&extra=ignored";
        let parsed = parse_query(q).expect("parse query");
        assert_eq!(parsed.login_type, 1);
        assert_eq!(parsed.user_id, "597938510");
        assert_eq!(parsed.user_data, 42);
        assert_eq!(parsed.device_id, None);
    }

    #[test]
    fn rejects_invalid_or_missing_query_fields() {
        assert!(parse_query("loginType=2&userId=1&userName=a&token=t&userData=0").is_err());
        assert!(parse_query("loginType=0&userId=1").is_err());
        assert!(parse_query("loginType=0&bad=a=b").is_err());
    }

    #[tokio::test]
    async fn serves_health_and_login_endpoints() {
        use std::time::{SystemTime, UNIX_EPOCH};
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("sonoro_loginsv_http_{nanos}"));
        std::fs::create_dir_all(&dir).expect("mkdir");

        let app = router(AppState {
            fs: Arc::new(FileSystem::open(&dir).expect("open fs")),
            gateway: ClientGatewayConfig {
                host: "127.0.0.1".into(),
                port: 13100,
            },
        });
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind");
        let addr = listener.local_addr().expect("addr");
        let server = tokio::spawn(async move {
            let _ = axum::serve(listener, app).await;
        });

        let mut stream = tokio::net::TcpStream::connect(addr).await.expect("connect");
        stream
            .write_all(
                b"GET /api/login?loginType=1&userId=12345&userName=Tester&token=tok123&userData=7 HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
            )
            .await
            .expect("write");
        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).await.expect("read");
        server.abort();

        let text = String::from_utf8_lossy(&buf);
        assert!(text.starts_with("HTTP/1.1 200 OK"));
        assert!(text.contains(r#""code":0"#));
        assert!(text.contains(r#""token":"tok123""#));
        assert!(text.contains(r#""port":13100"#));
        assert!(
            dir.join("account/12345").exists(),
            "account file must be flushed to disk before HTTP login response returns"
        );

        let _ = std::fs::remove_dir_all(&dir);
    }
}
