use std::convert::Infallible;
use std::env;
use std::fs;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::Router;
use axum::extract::{DefaultBodyLimit, RawQuery, State as AxumState};
use axum::http::{HeaderMap, HeaderValue, Method, StatusCode, header};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use rusqlite::Connection as SqliteConnection;
use serde::{Deserialize, Serialize};
use sonoro_gamesv_data::Assets;
use sonoro_gamesv_logic::commands;
use sonoro_gamesv_logic::commands::{attribute, cheat, js_patches, progression};
use sonoro_gamesv_logic::event_queue::EventQueue;
use tokio::sync::oneshot;

use super::aki_logic_bridge;
use super::gm_bridge::{GmAction, GmJob};
use super::gm_i18n;
use super::mod_state::SessionManager;
use super::session_supervisor::drain_and_reset;
use super::state::State;

const PANEL_HTML: &str = include_str!("gm_panel.html");

#[derive(Clone)]
struct AppState {
    assets: Arc<Assets>,
    session_manager: Arc<SessionManager>,
}

pub async fn serve(
    bind_address: SocketAddr,
    assets: Arc<Assets>,
    session_manager: Arc<SessionManager>,
    mut shutdown: tokio::sync::watch::Receiver<bool>,
) {
    let app_state = AppState {
        assets,
        session_manager,
    };

    let router = Router::new()
        .route("/", get(handle_panel))
        .route("/gm", get(handle_panel))
        .route("/gm/", get(handle_panel))
        .route("/announcement", get(handle_panel))
        .route("/health", get(handle_health))
        .route("/aki-logic/status", get(handle_aki_status))
        .route("/gm/catalog", get(handle_catalog))
        .route("/gm/auth", post(handle_auth))
        .route("/gm/state", get(handle_state))
        .route("/gm/exec", post(handle_exec))
        .fallback(handle_not_found)
        .layer(DefaultBodyLimit::max(65536))
        .layer(axum::middleware::from_fn(cors_layer))
        .with_state(app_state);

    let listener = match tokio::net::TcpListener::bind(bind_address).await {
        Ok(l) => l,
        Err(err) => {
            tracing::error!("failed to listen GM HTTP at {}: {}", bind_address, err);
            return;
        }
    };
    tracing::info!("GM HTTP & Web Panel listening at http://{}/gm", bind_address);

    let _ = axum::serve(listener, router)
        .with_graceful_shutdown(async move {
            let _ = shutdown.wait_for(|v| *v).await;
        })
        .await;
}

async fn cors_layer(
    req: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Result<Response, Infallible> {
    if req.method() == Method::OPTIONS {
        let mut resp = (StatusCode::NO_CONTENT, "").into_response();
        add_cors_headers(resp.headers_mut());
        return Ok(resp);
    }
    let mut resp = next.run(req).await;
    add_cors_headers(resp.headers_mut());
    Ok(resp)
}

fn add_cors_headers(headers: &mut HeaderMap) {
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_static("*"),
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("GET, POST, OPTIONS"),
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static("Content-Type, X-GM-Sign, X-GM-Admin"),
    );
    headers.insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("no-store"),
    );
}

fn json_resp(status: StatusCode, body: String) -> Response {
    (
        status,
        [(header::CONTENT_TYPE, "application/json; charset=utf-8")],
        body,
    )
        .into_response()
}

async fn handle_panel() -> impl IntoResponse {
    Html(PANEL_HTML)
}

async fn handle_health() -> Response {
    let aki_ok = tokio::task::spawn_blocking(aki_logic_bridge::ping)
        .await
        .unwrap_or(false);
    let body = if aki_ok {
        r#"{"ok":true,"aki_logic":true}"#
    } else {
        r#"{"ok":true,"aki_logic":false}"#
    };
    json_resp(StatusCode::OK, body.to_string())
}

async fn handle_aki_status() -> Response {
    let aki_ok = tokio::task::spawn_blocking(aki_logic_bridge::ping)
        .await
        .unwrap_or(false);
    let body = if aki_ok {
        r#"{"service":"AkiLogicService","status":"connected","runtime":"Rust","mode":"IPC UnixSocket"}"#
    } else {
        r#"{"service":"AkiLogicService","status":"offline","fallback":"Rust Native Engine"}"#
    };
    json_resp(StatusCode::OK, body.to_string())
}

fn query_param(raw: Option<&str>, key: &str) -> Option<String> {
    let raw = raw?;
    for pair in raw.split('&') {
        if pair.is_empty() {
            continue;
        }
        let (k, v) = pair.split_once('=').unwrap_or((pair, ""));
        if k == key {
            return Some(url_decode(v));
        }
    }
    None
}

fn url_decode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            b'%' if i + 2 < bytes.len() => {
                let hi = (bytes[i + 1] as char).to_digit(16);
                let lo = (bytes[i + 2] as char).to_digit(16);
                if let (Some(h), Some(l)) = (hi, lo) {
                    out.push(((h << 4) | l) as u8);
                    i += 3;
                    continue;
                }
                out.push(b'%');
                i += 1;
            }
            b => {
                out.push(b);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

async fn handle_catalog(
    AxumState(app): AxumState<AppState>,
    RawQuery(raw): RawQuery,
) -> Response {
    let kind = query_param(raw.as_deref(), "kind").unwrap_or_else(|| "quests".to_string());
    let q = query_param(raw.as_deref(), "q").unwrap_or_default();
    let type_filter = query_param(raw.as_deref(), "type").and_then(|s| s.parse::<i32>().ok());
    let json = build_catalog_json(&app.assets, &kind, &q, type_filter);
    json_resp(StatusCode::OK, json)
}

#[derive(Deserialize, Default)]
struct AuthBody {
    #[serde(default)]
    admin_key: String,
}

async fn handle_auth(headers: HeaderMap, body: String) -> Response {
    let Ok(parsed) = serde_json::from_str::<AuthBody>(&body) else {
        return json_resp(
            StatusCode::BAD_REQUEST,
            r#"{"ok":false,"admin":false,"message":"Invalid JSON payload"}"#.to_string(),
        );
    };
    let hdr_admin = header_str(&headers, "x-gm-admin");
    let supplied = if !hdr_admin.is_empty() {
        hdr_admin
    } else {
        &parsed.admin_key
    };
    if is_valid_admin(supplied) {
        json_resp(
            StatusCode::OK,
            r#"{"ok":true,"admin":true}"#.to_string(),
        )
    } else {
        json_resp(
            StatusCode::UNAUTHORIZED,
            r#"{"ok":false,"admin":false,"message":"Wrong password."}"#.to_string(),
        )
    }
}

#[derive(Serialize)]
struct OfflinePayload<'a> {
    online: bool,
    online_players: Vec<i32>,
    message: &'a str,
}

async fn handle_state(
    AxumState(app): AxumState<AppState>,
    headers: HeaderMap,
    RawQuery(raw): RawQuery,
) -> Response {
    let role_id = query_param(raw.as_deref(), "role_id")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0);
    let q_admin = query_param(raw.as_deref(), "admin_key").unwrap_or_default();
    let q_sign = query_param(raw.as_deref(), "sign").unwrap_or_default();
    let hdr_admin = header_str(&headers, "x-gm-admin");
    let req_admin = if !hdr_admin.is_empty() {
        hdr_admin
    } else {
        &q_admin
    };
    let hdr_sign = header_str(&headers, "x-gm-sign");
    let req_sign = if !hdr_sign.is_empty() {
        hdr_sign
    } else {
        &q_sign
    };
    let is_admin = is_valid_admin(req_admin);

    if !is_admin && !is_authorized_for_player(role_id, req_sign, req_admin) {
        return json_resp(
            StatusCode::UNAUTHORIZED,
            r#"{"ok":false,"online":false,"auth_required":true,"online_players":[],"message":"Access denied. Open the GM panel from in-game Announcements or enter your UID and Access Key."}"#.to_string(),
        );
    }

    if role_id > 0 {
        let (tx, rx) = oneshot::channel();
        let job = GmJob {
            action: GmAction::GetState,
            target_str: String::new(),
            int_a: 0,
            int_b: 0,
            int_c: 0,
            flag: is_admin,
            raw_cmd: String::new(),
            reply_tx: tx,
        };
        if app.session_manager.submit_gm_job(role_id, job).is_ok()
            && let Ok(res) = rx.await
        {
            return json_resp(StatusCode::OK, res.response_json);
        }
    }

    let online_players = if is_admin {
        app.session_manager.list_online_ids()
    } else {
        Vec::new()
    };
    let json = serde_json::to_string(&OfflinePayload {
        online: false,
        online_players,
        message: "No active online player session",
    })
    .unwrap_or_else(|_| r#"{"online":false,"online_players":[]}"#.to_string());
    json_resp(StatusCode::OK, json)
}

#[derive(Deserialize, Default)]
struct ExecBody {
    #[serde(default)]
    action: String,
    #[serde(default)]
    role_id: i32,
    #[serde(default)]
    target_str: String,
    #[serde(default)]
    int_a: i32,
    #[serde(default)]
    int_b: i32,
    #[serde(default)]
    int_c: i32,
    #[serde(default)]
    flag: bool,
    #[serde(default)]
    raw_cmd: String,
    #[serde(default)]
    sign: String,
    #[serde(default)]
    admin_key: String,
}

#[derive(Serialize)]
struct RotateResp<'a> {
    ok: bool,
    gm_key: String,
    message: &'a str,
}

async fn handle_exec(
    AxumState(app): AxumState<AppState>,
    headers: HeaderMap,
    body: String,
) -> Response {
    let Ok(parsed) = serde_json::from_str::<ExecBody>(&body) else {
        return json_resp(
            StatusCode::BAD_REQUEST,
            r#"{"ok":false,"message":"Invalid JSON payload"}"#.to_string(),
        );
    };
    let Some(action) = parse_gm_action(&parsed.action) else {
        return json_resp(
            StatusCode::BAD_REQUEST,
            r#"{"ok":false,"message":"Unknown action"}"#.to_string(),
        );
    };

    let hdr_admin = header_str(&headers, "x-gm-admin");
    let req_admin = if !hdr_admin.is_empty() {
        hdr_admin
    } else {
        &parsed.admin_key
    };
    let hdr_sign = header_str(&headers, "x-gm-sign");
    let req_sign = if !hdr_sign.is_empty() {
        hdr_sign
    } else {
        &parsed.sign
    };
    let is_admin = is_valid_admin(req_admin);

    if parsed.role_id <= 0 {
        return json_resp(
            StatusCode::OK,
            r#"{"ok":false,"message":"Invalid target player ID"}"#.to_string(),
        );
    }
    if !is_authorized_for_player(parsed.role_id, req_sign, req_admin) {
        return json_resp(
            StatusCode::UNAUTHORIZED,
            r#"{"ok":false,"auth_required":true,"message":"Access denied. Invalid UID or Access Key."}"#.to_string(),
        );
    }

    if action == GmAction::RotateGmKey {
        match rotate_player_sign_token(parsed.role_id) {
            Ok(new_key) => {
                let json = serde_json::to_string(&RotateResp {
                    ok: true,
                    gm_key: new_key,
                    message: "New personal Access Key generated. Previous keys are now invalid.",
                })
                .unwrap_or_default();
                return json_resp(StatusCode::OK, json);
            }
            Err(err) => {
                return json_resp(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    serde_json::json!({"ok": false, "message": format!("Key rotation failed: {err}")}).to_string(),
                );
            }
        }
    }

    if action == GmAction::ResetPlayer {
        let target_id = parsed.role_id;
        if app.session_manager.kick_player(target_id) {
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        }
        if let Some(mut st) = app.session_manager.take_handoff(target_id) {
            st.state.cleanup();
        }
        let player_dir = format!("state/player/{target_id}");
        if let Err(err) = fs::remove_dir_all(&player_dir)
            && err.kind() != std::io::ErrorKind::NotFound
        {
            tracing::warn!("failed to delete {}: {}", player_dir, err);
        }
        purge_player_sqlite(target_id);
        return json_resp(
            StatusCode::OK,
            format!(
                r#"{{"ok":true,"message":"Account UID {target_id} has been completely reset. Please log in again to start fresh."}}"#
            ),
        );
    }

    let (tx, rx) = oneshot::channel();
    let job = GmJob {
        action,
        target_str: parsed.target_str,
        int_a: parsed.int_a,
        int_b: parsed.int_b,
        int_c: parsed.int_c,
        flag: is_admin || parsed.flag,
        raw_cmd: parsed.raw_cmd,
        reply_tx: tx,
    };
    if app.session_manager.submit_gm_job(parsed.role_id, job).is_err() {
        return json_resp(
            StatusCode::OK,
            r#"{"ok":false,"message":"Player is offline or not logged in. Please log in to the game first."}"#.to_string(),
        );
    }
    match rx.await {
        Ok(res) => json_resp(StatusCode::OK, res.response_json),
        Err(_) => json_resp(
            StatusCode::OK,
            r#"{"ok":false,"message":"Session closed before responding"}"#.to_string(),
        ),
    }
}

async fn handle_not_found() -> Response {
    json_resp(
        StatusCode::NOT_FOUND,
        r#"{"ok":false,"message":"Not found"}"#.to_string(),
    )
}

fn header_str<'a>(headers: &'a HeaderMap, key: &str) -> &'a str {
    headers
        .get(key)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
}

fn parse_gm_action(s: &str) -> Option<GmAction> {
    match s {
        "get_state" => Some(GmAction::GetState),
        "exec_cmd" => Some(GmAction::ExecCmd),
        "give_item" => Some(GmAction::GiveItem),
        "set_union_level" => Some(GmAction::SetUnionLevel),
        "set_world_level" => Some(GmAction::SetWorldLevel),
        "set_role_level" => Some(GmAction::SetRoleLevel),
        "unlock_roles" => Some(GmAction::UnlockRoles),
        "role_max" => Some(GmAction::RoleMax),
        "role_chain" => Some(GmAction::RoleChain),
        "weapon_max" => Some(GmAction::WeaponMax),
        "quest_pick" => Some(GmAction::QuestPick),
        "quest_step" => Some(GmAction::QuestStep),
        "quest_skip" => Some(GmAction::QuestSkip),
        "unlock_tp" => Some(GmAction::UnlockTp),
        "unlock_func" => Some(GmAction::UnlockFunc),
        "time_set" => Some(GmAction::TimeSet),
        "heal_team" => Some(GmAction::HealTeam),
        "reset_cd" => Some(GmAction::ResetCd),
        "no_cd_mode" => Some(GmAction::NoCdMode),
        "spawn_echo" => Some(GmAction::SpawnEcho),
        "repair_account" => Some(GmAction::RepairAccount),
        "switch_rover" => Some(GmAction::SwitchRover),
        "reset_player" => Some(GmAction::ResetPlayer),
        "rotate_gm_key" => Some(GmAction::RotateGmKey),
        _ => None,
    }
}

fn admin_secret() -> String {
    env::var("GM_ADMIN_PASSWORD")
        .ok()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "admin".to_string())
}

fn timing_safe_equal(a: &[u8], b: &[u8]) -> bool {
    if a.is_empty() || a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (&x, &y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

fn is_valid_admin(key: &str) -> bool {
    timing_safe_equal(key.as_bytes(), admin_secret().as_bytes())
}

fn extract_sign_token_from_progress_info(content: &str) -> Option<String> {
    let marker = ".key = \"gm:sign_token\", .val = \"";
    let idx = content.find(marker)?;
    let rest = &content[idx + marker.len()..];
    let end = rest.find('"')?;
    let val = &rest[..end];
    (val.len() >= 16).then(|| val.to_string())
}

fn lookup_player_sign_token(target_id: i32) -> Option<String> {
    if target_id <= 0 {
        return None;
    }
    if let Ok(conn) = SqliteConnection::open("state/gamesv.db") {
        if let Ok(val) = conn.query_row(
            "SELECT val FROM player_kv WHERE player_id = ?1 AND ns = 'gm' AND k = 'sign_token' LIMIT 1",
            [target_id],
            |row| row.get::<_, String>(0),
        ) && val.len() >= 16
        {
            return Some(val);
        }
        let rel = format!("player/{target_id}/progress_info");
        if let Ok(blob) = conn.query_row(
            "SELECT content FROM state_files WHERE path = ?1 LIMIT 1",
            [&rel],
            |row| row.get::<_, Vec<u8>>(0),
        ) && let Ok(text) = std::str::from_utf8(&blob)
            && let Some(val) = extract_sign_token_from_progress_info(text)
        {
            return Some(val);
        }
    }

    let content = fs::read_to_string(format!("state/player/{target_id}/progress_info")).ok()?;
    extract_sign_token_from_progress_info(&content)
}

fn rotate_player_sign_token(target_id: i32) -> Result<String, String> {
    let mut rand_bytes = [0u8; 12];
    getrandom::fill(&mut rand_bytes).map_err(|e| e.to_string())?;
    let mut token = String::with_capacity(24);
    for b in rand_bytes {
        token.push_str(&format!("{b:02x}"));
    }

    let conn = SqliteConnection::open("state/gamesv.db").map_err(|e| e.to_string())?;
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0i64, |d| d.as_secs() as i64);
    conn.execute(
        "INSERT INTO player_kv(player_id, ns, k, val, updated_at) VALUES(?1, 'gm', 'sign_token', ?2, ?3)
         ON CONFLICT(player_id, ns, k) DO UPDATE SET val = excluded.val, updated_at = excluded.updated_at",
        rusqlite::params![target_id, token, ts],
    )
    .map_err(|e| e.to_string())?;
    Ok(token)
}

fn is_authorized_for_player(target_id: i32, req_sign: &str, req_admin: &str) -> bool {
    if is_valid_admin(req_admin) {
        return true;
    }
    if target_id <= 0 || req_sign.len() < 16 {
        return false;
    }
    let Some(expected) = lookup_player_sign_token(target_id) else {
        return false;
    };
    timing_safe_equal(req_sign.as_bytes(), expected.as_bytes())
}

fn contains_case_insensitive(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }
    haystack
        .to_ascii_lowercase()
        .contains(&needle.to_ascii_lowercase())
}

fn id_matches_query(id: i32, text: &str, q: &str) -> bool {
    if q.is_empty() {
        return true;
    }
    contains_case_insensitive(text, q) || id.to_string().contains(q)
}

fn build_catalog_json(
    assets: &Assets,
    kind: &str,
    q: &str,
    type_filter: Option<i32>,
) -> String {
    const MAX_ROWS: usize = 200;
    match kind {
        "quests" => {
            #[derive(Serialize)]
            struct QuestItem<'a> {
                id: i32,
                #[serde(rename = "type")]
                qtype: i32,
                type_name: &'a str,
                ch: i32,
                chapter: &'a str,
                reg: i32,
                region: &'a str,
                title: String,
            }
            let priority_types = [1, 3, 16, 9, 2, 7, 10, 14, 15, 17, 4, 11];
            let mut rows = Vec::new();
            for pt in priority_types {
                if let Some(tf) = type_filter
                    && tf != pt
                {
                    continue;
                }
                for entry in &assets.tables.quest_data.items {
                    let d = &entry.Data;
                    if d.Id <= 0 || d.Type == 100 || d.Type != pt {
                        continue;
                    }
                    let title = gm_i18n::resolve_quest_title(
                        d.Id,
                        d.Type,
                        d.ChapterId,
                        d.RegionId,
                        &d.Key,
                    );
                    let ch_name = gm_i18n::chapter_name(d.ChapterId);
                    let reg_name = gm_i18n::region_name(d.RegionId);
                    let type_name = gm_i18n::quest_type_name(d.Type);
                    if !id_matches_query(d.Id, &title, q)
                        && !contains_case_insensitive(ch_name, q)
                        && !contains_case_insensitive(reg_name, q)
                        && !contains_case_insensitive(type_name, q)
                        && !contains_case_insensitive(&d.Key, q)
                    {
                        continue;
                    }
                    rows.push(QuestItem {
                        id: d.Id,
                        qtype: d.Type,
                        type_name,
                        ch: d.ChapterId,
                        chapter: ch_name,
                        reg: d.RegionId,
                        region: reg_name,
                        title,
                    });
                    if rows.len() >= MAX_ROWS {
                        break;
                    }
                }
                if rows.len() >= MAX_ROWS {
                    break;
                }
            }
            serde_json::json!({ "items": rows }).to_string()
        }
        "roles" => {
            #[derive(Serialize)]
            struct RoleCat<'a> {
                id: i32,
                star: i32,
                elem: i32,
                weapon: i32,
                name: &'a str,
                abp: &'a str,
            }
            let mut rows = Vec::new();
            for r in &assets.tables.role_info.items {
                if !(1000..=1999).contains(&r.Id) || r.IsTrial || r.RoleType != 1 {
                    continue;
                }
                if !id_matches_query(r.Id, &r.UiScenePerformanceABP, q)
                    && !id_matches_query(r.Id, &r.Name, q)
                {
                    continue;
                }
                rows.push(RoleCat {
                    id: r.Id,
                    star: r.QualityId,
                    elem: r.ElementId,
                    weapon: r.WeaponType,
                    name: &r.Name,
                    abp: &r.UiScenePerformanceABP,
                });
                if rows.len() >= MAX_ROWS {
                    break;
                }
            }
            serde_json::json!({ "items": rows }).to_string()
        }
        "weapons" => {
            #[derive(Serialize)]
            struct WepCat<'a> {
                id: i32,
                star: i32,
                wtype: i32,
                kind: &'a str,
            }
            let mut rows = Vec::new();
            for w in &assets.tables.weapon_conf.items {
                let wlabel = gm_i18n::weapon_type_name(w.WeaponType);
                let full_tag = format!("{wlabel} ({}★)", w.QualityId);
                if !id_matches_query(w.ItemId, &full_tag, q) {
                    continue;
                }
                rows.push(WepCat {
                    id: w.ItemId,
                    star: w.QualityId,
                    wtype: w.WeaponType,
                    kind: wlabel,
                });
                if rows.len() >= MAX_ROWS {
                    break;
                }
            }
            serde_json::json!({ "items": rows }).to_string()
        }
        "echoes" => {
            #[derive(Serialize)]
            struct EchoCat<'a> {
                id: i32,
                mon: i32,
                star: i32,
                cost_hint: &'a str,
                fetters: &'a [i32],
            }
            let mut rows = Vec::new();
            for p in &assets.tables.phantom_item.items {
                let cost_lbl = match p.Rarity {
                    0 => "Cost 1 (Common)",
                    1 => "Cost 3 (Elite)",
                    2 => "Cost 4 (Overlord)",
                    3 => "Cost 4 (Calamity)",
                    _ => "Echo",
                };
                if !id_matches_query(p.ItemId, cost_lbl, q)
                    && !id_matches_query(p.MonsterId, cost_lbl, q)
                {
                    continue;
                }
                rows.push(EchoCat {
                    id: p.ItemId,
                    mon: p.MonsterId,
                    star: p.QualityId,
                    cost_hint: cost_lbl,
                    fetters: &p.FetterGroup,
                });
                if rows.len() >= MAX_ROWS {
                    break;
                }
            }
            serde_json::json!({ "items": rows }).to_string()
        }
        _ => {
            #[derive(Serialize)]
            struct ItemCat<'a> {
                id: i32,
                star: i32,
                kind: &'a str,
            }
            let mut rows = Vec::new();
            for it in &assets.tables.item_info.items {
                let tag = gm_i18n::item_kind_name(it.MainTypeId, it.ItemType);
                if !id_matches_query(it.Id, tag, q) {
                    continue;
                }
                rows.push(ItemCat {
                    id: it.Id,
                    star: it.QualityId,
                    kind: tag,
                });
                if rows.len() >= MAX_ROWS {
                    break;
                }
            }
            serde_json::json!({ "items": rows }).to_string()
        }
    }
}

#[derive(Serialize)]
struct ActiveQuestSnapshot<'a> {
    id: i32,
    #[serde(rename = "type")]
    qtype: i32,
    type_name: &'a str,
    chapter: &'a str,
    region: &'a str,
    title: String,
    tracked: bool,
    active_nodes: &'a [i32],
}

#[derive(Serialize)]
struct PlayerSnapshot<'a> {
    online: bool,
    online_players: Vec<i32>,
    uid: i32,
    name: &'a str,
    union_lv: i32,
    world_lv: i32,
    astrite: i32,
    coin: i32,
    cash_coin: i32,
    energy: i32,
    store_energy: i32,
    tp_unlocked: usize,
    tp_total: usize,
    roles_owned: usize,
    quest_res_state: i64,
    quest_block_state: i64,
    missing_resource_quests: Vec<i32>,
    active_quests: Vec<ActiveQuestSnapshot<'a>>,
}

fn build_snapshot<'a>(
    state: &'a State,
    session_manager: &SessionManager,
    include_online_players: bool,
) -> PlayerSnapshot<'a> {
    let basic = &state.player_components.basic.info;
    let quest = &state.player_components.quest.info;
    let map_info = &state.player_components.map.info;
    let progress = &state.player_components.progress.info;

    let mut aq_list = Vec::with_capacity(quest.active_quests.len());
    for aq in &quest.active_quests {
        let qdata = state.assets.tables.quest_data.get_data_by_id(&aq.quest_id);
        let (qtype, ch_id, reg_id, raw_key) = qdata.as_deref().map_or((0, 0, 0, ""), |qd| {
            (
                qd.Data.Type,
                qd.Data.ChapterId,
                qd.Data.RegionId,
                qd.Data.Key.as_str(),
            )
        });
        aq_list.push(ActiveQuestSnapshot {
            id: aq.quest_id,
            qtype,
            type_name: gm_i18n::quest_type_name(qtype),
            chapter: gm_i18n::chapter_name(ch_id),
            region: gm_i18n::region_name(reg_id),
            title: gm_i18n::resolve_quest_title(aq.quest_id, qtype, ch_id, reg_id, raw_key),
            tracked: quest.tracked_quest_id == aq.quest_id,
            active_nodes: &aq.active_nodes,
        });
    }

    let missing_resource_quests: Vec<i32> = progress
        .get_key_str("quest:missing_res")
        .split(',')
        .filter_map(|s| s.parse::<i32>().ok().filter(|&qid| qid > 0))
        .collect();

    let online_players = if include_online_players {
        session_manager.list_online_ids()
    } else {
        Vec::new()
    };

    PlayerSnapshot {
        online: true,
        online_players,
        uid: state.player_id,
        name: &basic.attributes.name,
        union_lv: basic.attributes.level,
        world_lv: basic.attributes.cur_world_level,
        astrite: basic.attributes.rare_coin,
        coin: basic.attributes.coin,
        cash_coin: basic.attributes.cash_coin,
        energy: basic.energy_count,
        store_energy: basic.store_energy_count,
        tp_unlocked: map_info.unlocked_teleporters.len(),
        tp_total: state.assets.tables.teleporter.items.len(),
        roles_owned: state.player_components.role.role_map.len(),
        quest_res_state: progress.get_key_int("quest:res_state"),
        quest_block_state: progress.get_key_int("quest:block_state"),
        missing_resource_quests,
        active_quests: aq_list,
    }
}

pub fn execute_session_job(state: &mut State, session_manager: &SessionManager, job: GmJob) {
    state.gm_output.capture = true;
    state.gm_output.output.clear();

    let mut events = EventQueue::new();
    let now_ms = sonoro_gamesv_logic::scene::now_millis();

    let action_res: Result<(), String> = (|| {
        match job.action {
            GmAction::GetState => {}
            GmAction::ExecCmd | GmAction::SpawnEcho => {
                let mut rt = state.as_runtime(
                    &mut events,
                    Some(super::handlers::activity::push_activity_update_notify),
                );
                commands::dispatch(&mut rt, &job.raw_cmd, now_ms).map_err(|e| e.to_string())?;
            }
            GmAction::GiveItem => {
                let count = if job.int_b > 0 { job.int_b } else { 1 };
                progression::give::call(
                    &mut events,
                    &state.fs,
                    &state.assets,
                    &mut state.conn,
                    &mut state.player_components.basic,
                    &mut state.player_components.inventory,
                    &mut state.player_components.weapon,
                    &mut state.player_components.echo,
                    job.int_a,
                    Some(count),
                )
                .map_err(|e| e.to_string())?;
            }
            GmAction::SetUnionLevel => {
                progression::level::call(
                    &mut events,
                    state.scene.as_mut(),
                    &state.fs,
                    &state.assets,
                    &mut state.conn,
                    &mut state.player_components.basic,
                    &mut state.player_components.role,
                    &state.player_components.weapon,
                    &state.player_components.echo,
                    "union",
                    job.int_a,
                )
                .map_err(|e| e.to_string())?;
            }
            GmAction::SetWorldLevel => {
                cheat::world_level::call(
                    &mut events,
                    &state.fs,
                    &mut state.conn,
                    &mut state.player_components.basic,
                    job.int_a,
                )
                .map_err(|e| e.to_string())?;
            }
            GmAction::SetRoleLevel => {
                let subj = if job.target_str.is_empty() {
                    "team"
                } else {
                    &job.target_str
                };
                progression::level::call(
                    &mut events,
                    state.scene.as_mut(),
                    &state.fs,
                    &state.assets,
                    &mut state.conn,
                    &mut state.player_components.basic,
                    &mut state.player_components.role,
                    &state.player_components.weapon,
                    &state.player_components.echo,
                    subj,
                    job.int_a,
                )
                .map_err(|e| e.to_string())?;
            }
            GmAction::UnlockRoles => {
                let tgt = if job.target_str.is_empty() {
                    "all"
                } else {
                    &job.target_str
                };
                cheat::unlock_roles::call(
                    &mut events,
                    &state.fs,
                    &state.assets,
                    &mut state.conn,
                    &mut state.player_components.basic,
                    &mut state.player_components.role,
                    &mut state.player_components.weapon,
                    &mut state.player_components.echo,
                    Some(tgt),
                )
                .map_err(|e| e.to_string())?;
            }
            GmAction::RoleMax => {
                let subj = if job.target_str.is_empty() {
                    "team"
                } else {
                    &job.target_str
                };
                let lv = if job.int_a > 0 { job.int_a } else { 90 };
                cheat::role_max::call(
                    &mut events,
                    &state.fs,
                    &state.assets,
                    &mut state.conn,
                    state.scene.as_mut(),
                    &mut state.player_components.role,
                    &state.player_components.weapon,
                    &state.player_components.echo,
                    Some(subj),
                    Some(lv),
                )
                .map_err(|e| e.to_string())?;
            }
            GmAction::RoleChain => {
                let subj = if job.target_str.is_empty() {
                    "team"
                } else {
                    &job.target_str
                };
                cheat::role_chain::call(
                    &mut events,
                    &state.fs,
                    &state.assets,
                    &mut state.conn,
                    state.scene.as_mut(),
                    &mut state.player_components.role,
                    &state.player_components.weapon,
                    &state.player_components.echo,
                    subj,
                    job.int_a,
                )
                .map_err(|e| e.to_string())?;
            }
            GmAction::WeaponMax => {
                let scope = if job.target_str.is_empty() {
                    "equipped"
                } else {
                    &job.target_str
                };
                let lv = if job.int_a > 0 { job.int_a } else { 90 };
                let rank = if job.int_b > 0 { job.int_b } else { 5 };
                cheat::weapon_max::call(
                    &mut events,
                    &state.fs,
                    &state.assets,
                    &mut state.conn,
                    state.scene.as_mut(),
                    &mut state.player_components.role,
                    &mut state.player_components.weapon,
                    &state.player_components.echo,
                    Some(scope),
                    Some(lv),
                    Some(rank),
                )
                .map_err(|e| e.to_string())?;
            }
            GmAction::QuestPick => {
                let mut qctx = state.quest_context(&mut events);
                cheat::quest_pick::call(&mut qctx, job.int_a, Some(job.flag))
                    .map_err(|e| e.to_string())?;
            }
            GmAction::QuestStep => {
                let mut qctx = state.quest_context(&mut events);
                cheat::quest_step::call(&mut qctx, (job.int_a > 0).then_some(job.int_a))
                    .map_err(|e| e.to_string())?;
            }
            GmAction::QuestSkip => {
                let mut qctx = state.quest_context(&mut events);
                cheat::quest_skip::call(&mut qctx, (job.int_a > 0).then_some(job.int_a))
                    .map_err(|e| e.to_string())?;
            }
            GmAction::UnlockTp => {
                cheat::unlock_tp::call(
                    &mut events,
                    &state.fs,
                    &state.assets,
                    &mut state.conn,
                    &mut state.player_components.map,
                )
                .map_err(|e| e.to_string())?;
            }
            GmAction::UnlockFunc => {
                let mut qctx = state.quest_context(&mut events);
                cheat::unlock_func::call(&mut qctx).map_err(|e| e.to_string())?;
            }
            GmAction::TimeSet => {
                let mut qctx = state.quest_context(&mut events);
                cheat::time_set::call(&mut qctx, job.int_a, Some(job.int_b))
                    .map_err(|e| e.to_string())?;
            }
            GmAction::HealTeam => {
                let sc = state
                    .scene
                    .as_mut()
                    .ok_or_else(|| "player has no active scene".to_string())?;
                attribute::heal::call(
                    &mut events,
                    sc,
                    &state.fs,
                    &mut state.player_components.role,
                    &mut state.conn,
                );
            }
            GmAction::ResetCd => {
                js_patches::reset_cd::call(&mut events, &mut state.conn, state.player_id)
                    .map_err(|e| e.to_string())?;
            }
            GmAction::NoCdMode => {
                js_patches::team_cd_controller::call(
                    &mut events,
                    &mut state.conn,
                    state.player_id,
                    job.int_a,
                )
                .map_err(|e| e.to_string())?;
            }
            GmAction::RepairAccount => {
                let mut qctx = state.quest_context(&mut events);
                cheat::repair_account::call(&mut qctx).map_err(|e| e.to_string())?;
            }
            GmAction::SwitchRover => {
                let elem = if job.target_str.is_empty() {
                    "havoc"
                } else {
                    &job.target_str
                };
                cheat::switch_rover::call(
                    &mut events,
                    &state.fs,
                    &state.assets,
                    &mut state.conn,
                    state.scene.as_mut(),
                    &mut state.player_components.basic,
                    &mut state.player_components.role,
                    &mut state.player_components.weapon,
                    &mut state.player_components.echo,
                    elem,
                )
                .map_err(|e| e.to_string())?;
            }
            GmAction::ResetPlayer | GmAction::RotateGmKey => {}
        }
        Ok(())
    })();

    drain_and_reset(state, &mut events);

    let (ok, err_msg) = match action_res {
        Ok(()) => (true, String::new()),
        Err(e) => (false, e),
    };
    let captured = std::mem::take(&mut state.gm_output.output);
    state.gm_output.capture = false;

    let msg = if !ok {
        if captured.is_empty() {
            format!("Error: {err_msg}")
        } else {
            format!("Error: {err_msg}\n{captured}")
        }
    } else if !captured.is_empty() {
        captured
    } else {
        "OK".to_string()
    };

    let include_online = (job.action == GmAction::GetState) && job.flag;
    let snap = build_snapshot(state, session_manager, include_online);
    if job.action == GmAction::GetState {
        match serde_json::to_string(&snap) {
            Ok(json) => job.succeed_and_signal(json),
            Err(_) => job.fail_and_signal("Serialization failed"),
        }
    } else {
        #[derive(Serialize)]
        struct RespPayload<'a> {
            ok: bool,
            message: &'a str,
            state: PlayerSnapshot<'a>,
        }
        match serde_json::to_string(&RespPayload {
            ok,
            message: &msg,
            state: snap,
        }) {
            Ok(json) => job.succeed_and_signal(json),
            Err(_) => job.fail_and_signal("Serialization failed"),
        }
    }
}

fn purge_player_sqlite(target_id: i32) {
    let Ok(conn) = SqliteConnection::open("state/gamesv.db") else {
        tracing::warn!("failed to open state/gamesv.db for reset");
        return;
    };
    let pattern = format!("player/{target_id}/%");
    if let Err(err) = conn.execute("DELETE FROM state_files WHERE path LIKE ?1", [&pattern]) {
        tracing::warn!(
            target: "gm_http",
            "resetPlayer state_files delete failed for uid={}: {}",
            target_id,
            err
        );
    }
    let tables = [
        "DELETE FROM players WHERE player_id = ?1",
        "DELETE FROM player_battlepass WHERE player_id = ?1",
        "DELETE FROM player_activity_claims WHERE player_id = ?1",
        "DELETE FROM player_liveness WHERE player_id = ?1",
        "DELETE FROM player_achievements WHERE player_id = ?1",
        "DELETE FROM player_shop_bought WHERE player_id = ?1",
        "DELETE FROM player_kv WHERE player_id = ?1",
    ];
    for sql in tables {
        let _ = conn.execute(sql, [target_id]);
    }
}
