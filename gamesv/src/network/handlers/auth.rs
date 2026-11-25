use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

use sonoro_common::fs::FileSystem;
use sonoro_gamesv_fs::account::{self, Account};
use sonoro_gamesv_fs::basic_info::BasicInfo;
use sonoro_gamesv_fs::file_util::serialize_zon;
use sonoro_proto::PbMessage;
use sonoro_proto::enums::ErrorCode;
use sonoro_proto::pb::{
    CreateCharacterRequest, CreateCharacterResponse, EnterGameRequest, EnterGameResponse,
    HeartbeatRequest, HeartbeatResponse, LoginRequest, LoginResponse, ProtoKeyRequest,
    ProtoKeyResponse, ReconnectRequest, ReconnectResponse,
};
use sonoro_proto::registry::msg_id::{
    CREATE_CHARACTER_REQUEST, ENTER_GAME_REQUEST, HEARTBEAT_REQUEST, LOGIN_REQUEST,
    PROTO_KEY_REQUEST, RECONNECT_REQUEST,
};

use super::super::connection::Connection;
use super::super::encryption;
use super::super::message::{Header, Message};

const PROTO_KEY_TYPE: i32 = 2;
const RECONNECT_TOKEN_BYTES: usize = 32;

#[derive(Debug, Clone, Copy)]
pub struct PendingReconnect {
    pub rpc_id: u16,
    pub last_server_seq_no: i32,
}

pub struct AuthSessionState {
    pub player_id: Option<i32>,
    pub enter: bool,
    pub pending_reconnect: Option<PendingReconnect>,
}

fn now_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_millis() as i64)
}

fn now_secs_i32() -> i32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs() as i32)
}

pub fn handle_auth_group_request(
    message: &Message,
    conn: &mut Connection,
    fs: &FileSystem,
    auth: &mut AuthSessionState,
) -> Result<(), String> {
    let Header::Request(req_hdr) = message.header else {
        return Err("invalid message type in auth phase".to_string());
    };
    let rpc_id = req_hdr.rpc_id;
    let msg_id = req_hdr.msg_id;

    match msg_id {
        PROTO_KEY_REQUEST => {
            let req = ProtoKeyRequest::decode(&message.payload)
                .map_err(|e| format!("decode ProtoKeyRequest: {e}"))?;
            handle_proto_key(rpc_id, req, conn, fs)?;
        }
        LOGIN_REQUEST => {
            let req = LoginRequest::decode(&message.payload)
                .map_err(|e| format!("decode LoginRequest: {e}"))?;
            handle_login(rpc_id, req, conn, fs, auth)?;
        }
        CREATE_CHARACTER_REQUEST => {
            let req = CreateCharacterRequest::decode(&message.payload)
                .map_err(|e| format!("decode CreateCharacterRequest: {e}"))?;
            handle_create_character(rpc_id, req, conn, fs, auth)?;
        }
        RECONNECT_REQUEST => {
            let req = ReconnectRequest::decode(&message.payload)
                .map_err(|e| format!("decode ReconnectRequest: {e}"))?;
            handle_reconnect(rpc_id, req, conn, fs, auth)?;
        }
        HEARTBEAT_REQUEST => {
            let _ = HeartbeatRequest::decode(&message.payload)
                .map_err(|e| format!("decode HeartbeatRequest: {e}"))?;
            conn.respond(rpc_id, &HeartbeatResponse {})
                .map_err(|e| e.to_string())?;
        }
        ENTER_GAME_REQUEST => {
            let _ = EnterGameRequest::decode(&message.payload)
                .map_err(|e| format!("decode EnterGameRequest: {e}"))?;
            handle_enter_game(rpc_id, conn, fs, auth)?;
        }
        _ => {
            tracing::warn!(
                target: "auth",
                "unhandled auth message id: {}, ignoring to keep session alive",
                msg_id
            );
        }
    }
    Ok(())
}

fn handle_proto_key(
    rpc_id: u16,
    req: ProtoKeyRequest,
    conn: &mut Connection,
    fs: &FileSystem,
) -> Result<(), String> {
    let mut session_key = [0u8; 32];
    getrandom::fill(&mut session_key).map_err(|e| format!("rng failed: {e}"))?;

    let public_key_der = fs
        .read_file("gateway/client_public_key.der")
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "missing gateway/client_public_key.der".to_string())?;

    let encrypted_key =
        encryption::rsa_encrypt(&public_key_der, &session_key).map_err(|e| e.to_string())?;

    let resp = ProtoKeyResponse {
        Type: PROTO_KEY_TYPE,
        Key: encrypted_key,
        ..Default::default()
    };
    if req.IsLogin {
        conn.respond(rpc_id, &resp).map_err(|e| e.to_string())?;
    } else {
        conn.respond_seq0(rpc_id, &resp)
            .map_err(|e| e.to_string())?;
    }
    conn.set_session_key(session_key);
    Ok(())
}

fn handle_login(
    rpc_id: u16,
    req: LoginRequest,
    conn: &Connection,
    fs: &FileSystem,
    auth: &mut AuthSessionState,
) -> Result<(), String> {
    let account_str = String::from_utf8_lossy(&req.Account);
    let token_str = String::from_utf8_lossy(&req.Token);

    let verified = account::is_safe_identifier(&account_str)
        && account::verify_login_token(fs, &account_str, &token_str).unwrap_or(false);
    if !verified {
        tracing::warn!(target: "login", "login rejected for account '{}'", account_str);
        conn.respond(
            rpc_id,
            &LoginResponse {
                ErrorCode: Some(ErrorCode::InvalidToken),
                Timestamp: now_millis(),
                ..Default::default()
            },
        )
        .map_err(|e| e.to_string())?;
        return Ok(());
    }

    let acc = Account::load_or_create(fs, &account_str).map_err(|e| e.to_string())?;
    auth.player_id = Some(acc.player_id);

    let basic_info_path = format!("player/{}/basic_info", acc.player_id);
    let has_character = fs
        .read_file(&basic_info_path)
        .map_err(|e| e.to_string())?
        .is_some();

    let reconnect_token = issue_reconnect_token(fs, acc.player_id).map_err(|e| e.to_string())?;
    if !has_character {
        tracing::info!(
            target: "login",
            "player {} has no character yet, awaiting create character request",
            acc.player_id
        );
        conn.respond(
            rpc_id,
            &LoginResponse {
                ErrorCode: Some(ErrorCode::HaveNoCharacter),
                ReconnectToken: reconnect_token.into_bytes(),
                Timestamp: now_millis(),
                ..Default::default()
            },
        )
        .map_err(|e| e.to_string())?;
        return Ok(());
    }

    let _ = fs.delete_file(&fresh_reconnect_path(acc.player_id));
    conn.respond(
        rpc_id,
        &LoginResponse {
            ErrorCode: Some(ErrorCode::Success),
            ReconnectToken: reconnect_token.into_bytes(),
            Timestamp: now_millis(),
            ..Default::default()
        },
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

fn handle_create_character(
    rpc_id: u16,
    req: CreateCharacterRequest,
    conn: &Connection,
    fs: &FileSystem,
    auth: &mut AuthSessionState,
) -> Result<(), String> {
    let Some(player_id) = auth.player_id else {
        tracing::error!(
            target: "create_character",
            "create character request received without player_id in session"
        );
        conn.respond(
            rpc_id,
            &CreateCharacterResponse {
                ErrorCode: Some(ErrorCode::CreateCharacterFailed),
                ..Default::default()
            },
        )
        .map_err(|e| e.to_string())?;
        return Ok(());
    };

    let raw_name = String::from_utf8_lossy(&req.Name);
    let trimmed_name = raw_name.trim_matches([' ', '\t', '\r', '\n']);
    if trimmed_name.is_empty() || trimmed_name.len() > 64 || (req.Sex != 0 && req.Sex != 1) {
        conn.respond(
            rpc_id,
            &CreateCharacterResponse {
                ErrorCode: Some(ErrorCode::CreateCharacterFailed),
                ..Default::default()
            },
        )
        .map_err(|e| e.to_string())?;
        return Ok(());
    }

    let basic_info_path = format!("player/{player_id}/basic_info");
    if fs
        .read_file(&basic_info_path)
        .map_err(|e| e.to_string())?
        .is_some()
    {
        conn.respond(
            rpc_id,
            &CreateCharacterResponse {
                ErrorCode: Some(ErrorCode::CreateCharacterDuplicateKey),
                ..Default::default()
            },
        )
        .map_err(|e| e.to_string())?;
        return Ok(());
    }

    let mut basic_info = BasicInfo::default();
    basic_info.attributes.name = trimmed_name.to_string();
    basic_info.attributes.sex = req.Sex;
    basic_info.attributes.level = 1;
    basic_info.attributes.cur_world_level = 1;
    basic_info.attributes.origin_world_level = 1;
    let basic_data = serialize_zon(&basic_info).map_err(|e| e.to_string())?;
    fs.write_file(&basic_info_path, &basic_data)
        .map_err(|e| e.to_string())?;

    Account::sync_login_sex_for_player(fs, player_id, req.Sex).map_err(|e| e.to_string())?;
    fs.write_file(&fresh_reconnect_path(player_id), b"1")
        .map_err(|e| e.to_string())?;

    conn.respond(
        rpc_id,
        &CreateCharacterResponse {
            ErrorCode: Some(ErrorCode::Success),
            Name: trimmed_name.as_bytes().to_vec(),
            PlayerId: player_id,
            CreateTime: now_secs_i32(),
        },
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

fn handle_reconnect(
    rpc_id: u16,
    req: ReconnectRequest,
    conn: &Connection,
    fs: &FileSystem,
    auth: &mut AuthSessionState,
) -> Result<(), String> {
    let token_str = String::from_utf8_lossy(&req.ReconnectToken);
    let mut resolved_player_id = req.PlayerId;
    if resolved_player_id <= 0 && token_str.len() == RECONNECT_TOKEN_BYTES * 2 {
        if let Some(known_pid) = auth.player_id
            && validate_reconnect_token(fs, known_pid, &token_str).unwrap_or(false)
        {
            resolved_player_id = known_pid;
        }
        if resolved_player_id <= 0 {
            let fallback_pid = 597_938_510;
            if validate_reconnect_token(fs, fallback_pid, &token_str).unwrap_or(false) {
                resolved_player_id = fallback_pid;
            }
        }
    }

    if resolved_player_id <= 0 {
        conn.respond(
            rpc_id,
            &ReconnectResponse {
                ErrorCode: Some(ErrorCode::ErrReconnectGwgetGatePlayerFailed),
                Timestamp: now_millis(),
                IsPermittedSilentLogin: false,
                ..Default::default()
            },
        )
        .map_err(|e| e.to_string())?;
        return Ok(());
    }

    let basic_info_path = format!("player/{resolved_player_id}/basic_info");
    if fs
        .read_file(&basic_info_path)
        .map_err(|e| e.to_string())?
        .is_none()
    {
        conn.respond(
            rpc_id,
            &ReconnectResponse {
                ErrorCode: Some(ErrorCode::ErrReconnectGwgetGatePlayerFailed),
                Timestamp: now_millis(),
                IsPermittedSilentLogin: false,
                ..Default::default()
            },
        )
        .map_err(|e| e.to_string())?;
        return Ok(());
    }

    let fresh_path = fresh_reconnect_path(resolved_player_id);
    let has_fresh_marker = fs
        .read_file(&fresh_path)
        .map_err(|e| e.to_string())?
        .is_some();
    let token_valid = if token_str.is_empty() && has_fresh_marker {
        let _ = fs.delete_file(&fresh_path);
        true
    } else {
        let valid = validate_reconnect_token(fs, resolved_player_id, &token_str).unwrap_or(false);
        if valid && has_fresh_marker {
            let _ = fs.delete_file(&fresh_path);
        }
        valid
    };

    if !token_valid {
        conn.respond(
            rpc_id,
            &ReconnectResponse {
                ErrorCode: Some(ErrorCode::ErrReconnectGwgetGatePlayerFailed),
                Timestamp: now_millis(),
                IsPermittedSilentLogin: false,
                ..Default::default()
            },
        )
        .map_err(|e| e.to_string())?;
        return Ok(());
    }

    auth.player_id = Some(resolved_player_id);
    auth.enter = true;
    auth.pending_reconnect = Some(PendingReconnect {
        rpc_id,
        last_server_seq_no: req.LastSvrSeqNo,
    });
    Ok(())
}

fn handle_enter_game(
    rpc_id: u16,
    conn: &Connection,
    fs: &FileSystem,
    auth: &mut AuthSessionState,
) -> Result<(), String> {
    let Some(player_id) = auth.player_id else {
        conn.respond(
            rpc_id,
            &EnterGameResponse {
                ErrorCode: Some(ErrorCode::HaveNoCharacter),
                ..Default::default()
            },
        )
        .map_err(|e| e.to_string())?;
        return Ok(());
    };
    let basic_info_path = format!("player/{player_id}/basic_info");
    if fs
        .read_file(&basic_info_path)
        .map_err(|e| e.to_string())?
        .is_none()
    {
        conn.respond(
            rpc_id,
            &EnterGameResponse {
                ErrorCode: Some(ErrorCode::HaveNoCharacter),
                ..Default::default()
            },
        )
        .map_err(|e| e.to_string())?;
        return Ok(());
    }

    auth.enter = true;
    conn.respond(rpc_id, &EnterGameResponse::default())
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn issue_reconnect_token(fs: &FileSystem, player_id: i32) -> io::Result<String> {
    let mut random = [0u8; RECONNECT_TOKEN_BYTES];
    getrandom::fill(&mut random).map_err(|e| io::Error::other(e.to_string()))?;
    let mut hex = String::with_capacity(RECONNECT_TOKEN_BYTES * 2);
    for b in random {
        hex.push_str(&format!("{b:02x}"));
    }
    fs.write_file(&reconnect_token_path(player_id), hex.as_bytes())?;
    Ok(hex)
}

fn validate_reconnect_token(fs: &FileSystem, player_id: i32, token: &str) -> io::Result<bool> {
    if token.len() != RECONNECT_TOKEN_BYTES * 2 {
        return Ok(false);
    }
    let Some(saved) = fs.read_file(&reconnect_token_path(player_id))? else {
        return Ok(false);
    };
    Ok(saved == token.as_bytes())
}

fn reconnect_token_path(player_id: i32) -> String {
    format!("player/{player_id}/reconnect_token")
}

fn fresh_reconnect_path(player_id: i32) -> String {
    format!("player/{player_id}/fresh_reconnect")
}
