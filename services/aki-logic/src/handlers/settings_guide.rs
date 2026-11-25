//! Settings, AccessPath, PlayerTitle, Music, and MonsterDetection handlers.

use sonoro_proto::PbMessage;
use sonoro_proto::enums::*;
use sonoro_proto::pb::*;
use sonoro_proto::registry::msg_id;

use crate::bin_data;
use crate::ipc::AkiResult;
use crate::sqlite_store;

pub fn handle_language_setting_update(player_id: i32, payload: &[u8]) -> AkiResult {
    let req = LanguageSettingUpdateRequest::decode(payload).unwrap_or_default();
    if player_id > 0 {
        sqlite_store::set_kv(player_id, "settings", "language", &req.Language.to_string());
    }
    AkiResult::ok(
        msg_id::LANGUAGE_SETTING_UPDATE_RESPONSE,
        &LanguageSettingUpdateResponse {
            ErrorCode: Some(ErrorCode::Success),
        },
    )
}

pub fn handle_input_setting_request(player_id: i32, _payload: &[u8]) -> AkiResult {
    let mut resp = InputSettingResponse::default();
    if player_id > 0
        && let Some(raw_bytes) = sqlite_store::get_kv(player_id, "settings", "input_pb_b64")
            .and_then(|s| decode_b64(&s).ok())
            .or_else(|| {
                sqlite_store::get_kv(player_id, "settings", "input_pb_hex")
                    .and_then(|s| decode_hex(&s).ok())
            })
        && let Ok(data) = InputSettingData::decode(&raw_bytes)
    {
        resp.InputSettingData = Some(data);
    }
    AkiResult::ok(msg_id::INPUT_SETTING_RESPONSE, &resp)
}

pub fn handle_input_setting_update(player_id: i32, payload: &[u8]) -> AkiResult {
    let req = InputSettingUpdateRequest::decode(payload).unwrap_or_default();
    if player_id > 0
        && let Some(data) = &req.InputSettingData
    {
        let raw = data.encode_to_vec();
        sqlite_store::set_kv(player_id, "settings", "input_pb_b64", &encode_b64(&raw));
    }
    AkiResult::ok(
        msg_id::INPUT_SETTING_UPDATE_RESPONSE,
        &InputSettingUpdateResponse {
            ErrorCode: Some(ErrorCode::Success),
        },
    )
}

pub fn handle_access_path_config(_player_id: i32, _payload: &[u8]) -> AkiResult {
    let configs = bin_data::get()
        .access_path_ids
        .iter()
        .map(|&id| AccessPathConfig {
            Id: id,
            BeginTime: 1,
            EndTime: 2_000_000_000,
        })
        .collect();
    AkiResult::ok(
        msg_id::ACCESS_PATH_CONFIG_RESPONSE,
        &AccessPathConfigResponse { Configs: configs },
    )
}

pub fn handle_player_title_request(_player_id: i32, _payload: &[u8]) -> AkiResult {
    let infos = bin_data::get()
        .player_title_ids
        .iter()
        .map(|&tid| PlayerTitleInfo {
            PlayerTitleId: tid,
            IsUnlock: true,
            UnlockTime: 1,
            ..Default::default()
        })
        .collect();
    AkiResult::ok(
        msg_id::PLAYER_TITLE_RESPONSE,
        &PlayerTitleResponse {
            ErrorCode: Some(ErrorCode::Success),
            PlayerTitleInfo: infos,
            ..Default::default()
        },
    )
}

pub fn handle_phantom_music_info(player_id: i32, _payload: &[u8]) -> AkiResult {
    let cache = bin_data::get();
    let mut resp = PhantomMusicInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        ..Default::default()
    };

    if player_id > 0 {
        if let Some(cur_str) = sqlite_store::get_kv(player_id, "music", "cur_id")
            && let Ok(cur_id) = cur_str.parse::<i32>()
        {
            resp.CurMusicId = cur_id;
        }
        if let Some(fav_json) = sqlite_store::get_kv(player_id, "music", "collect_ids")
            && let Ok(fav_ids) = serde_json::from_str::<Vec<i32>>(&fav_json)
        {
            for mid in fav_ids {
                if cache.all_music_ids.contains(&mid) {
                    resp.CollectMusicIds.push(mid);
                }
            }
        }
    }

    for (&album_id, music_ids) in &cache.phonograph_albums {
        resp.AlbumTimeInfos.push(PhonographAlbumTimeInfo {
            AlbumId: album_id,
            BeginTime: 0,
            EndTime: 4_070_908_800,
            MusicIds: music_ids.clone(),
        });
    }

    AkiResult::ok(msg_id::PHANTOM_MUSIC_INFO_RESPONSE, &resp)
}

pub fn handle_phantom_music_unlock(_player_id: i32, _payload: &[u8]) -> AkiResult {
    let add_ids: Vec<i32> = bin_data::get().all_music_ids.iter().copied().collect();
    AkiResult::ok(
        msg_id::PHANTOM_MUSIC_UNLOCK_RESPONSE,
        &PhantomMusicUnlockResponse {
            ErrorCode: Some(ErrorCode::Success),
            AddMusicIds: add_ids,
        },
    )
}

pub fn handle_phantom_music_switch(player_id: i32, payload: &[u8]) -> AkiResult {
    let req = PhantomMusicSwitchRequest::decode(payload).unwrap_or_default();
    if player_id > 0 {
        sqlite_store::set_kv(player_id, "music", "cur_id", &req.MusicId.to_string());
    }
    AkiResult::ok(
        msg_id::PHANTOM_MUSIC_SWITCH_RESPONSE,
        &PhantomMusicSwitchResponse {
            ErrorCode: Some(ErrorCode::Success),
        },
    )
}

pub fn handle_phantom_music_collect_update(player_id: i32, payload: &[u8]) -> AkiResult {
    let req = PhantomMusicCollectUpdateRequest::decode(payload).unwrap_or_default();
    if player_id > 0 {
        let cache = bin_data::get();
        let mut valid = Vec::new();
        for mid in req.CollectMusicIds {
            if cache.all_music_ids.contains(&mid) && !valid.contains(&mid) {
                valid.push(mid);
            }
        }
        if let Ok(json) = serde_json::to_string(&valid) {
            sqlite_store::set_kv(player_id, "music", "collect_ids", &json);
        }
    }
    AkiResult::ok(
        msg_id::PHANTOM_MUSIC_COLLECT_UPDATE_RESPONSE,
        &PhantomMusicCollectUpdateResponse {
            ErrorCode: Some(ErrorCode::Success),
        },
    )
}

pub fn handle_phantom_music_save(_player_id: i32, _payload: &[u8]) -> AkiResult {
    AkiResult::ok(
        msg_id::PHANTOM_MUSIC_SAVE_RESPONSE,
        &PhantomMusicSaveResponse {
            ErrCode: Some(ErrorCode::Success),
        },
    )
}

pub fn handle_monster_detection_info_request(_player_id: i32, _payload: &[u8]) -> AkiResult {
    let cache = bin_data::get();
    let mut targets = Vec::new();

    for (ids, dtype) in [
        (&cache.monster_detection_ids, DetectionType::NormalMonster),
        (&cache.dungeon_detection_ids, DetectionType::Dungeon),
        (&cache.silent_area_detection_ids, DetectionType::SilentArea),
    ] {
        for &id in ids {
            targets.push(DetectionTarget {
                Id: id,
                Type: Some(dtype),
                UnlockState: true,
                RefresherTime: 0,
                DetectionId: id,
                IsTrace: 0,
            });
        }
    }

    AkiResult::ok(
        msg_id::MONSTER_DETECTION_INFO_RESPONSE,
        &MonsterDetectionInfoResponse {
            DetectionTarget: targets,
        },
    )
}

fn encode_b64(bytes: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        let n = (b0 << 16) | (b1 << 8) | b2;
        out.push(TABLE[((n >> 18) & 63) as usize] as char);
        out.push(TABLE[((n >> 12) & 63) as usize] as char);
        out.push(if chunk.len() > 1 {
            TABLE[((n >> 6) & 63) as usize] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            TABLE[(n & 63) as usize] as char
        } else {
            '='
        });
    }
    out
}

fn decode_b64(s: &str) -> Result<Vec<u8>, ()> {
    let trimmed = s.trim();
    let mut out = Vec::with_capacity(trimmed.len() * 3 / 4);
    let mut buf = 0u32;
    let mut bits = 0u8;
    for b in trimmed.bytes() {
        let val = match b {
            b'A'..=b'Z' => b - b'A',
            b'a'..=b'z' => b - b'a' + 26,
            b'0'..=b'9' => b - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            b'=' => break,
            b' ' | b'\r' | b'\n' | b'\t' => continue,
            _ => return Err(()),
        };
        buf = (buf << 6) | u32::from(val);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((buf >> bits) as u8);
        }
    }
    Ok(out)
}

fn decode_hex(s: &str) -> Result<Vec<u8>, ()> {
    let trimmed = s.trim();
    let nibbles: Vec<u8> = trimmed
        .bytes()
        .filter(|b| !matches!(b, b' ' | b'\r' | b'\n' | b'\t'))
        .map(|b| match b {
            b'0'..=b'9' => Ok(b - b'0'),
            b'a'..=b'f' => Ok(b - b'a' + 10),
            b'A'..=b'F' => Ok(b - b'A' + 10),
            _ => Err(()),
        })
        .collect::<Result<_, _>>()?;
    if !nibbles.len().is_multiple_of(2) {
        return Err(());
    }
    Ok(nibbles
        .as_chunks::<2>()
        .0
        .iter()
        .map(|[hi, lo]| (hi << 4) | lo)
        .collect())
}
