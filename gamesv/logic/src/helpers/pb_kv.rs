use std::collections::HashSet;
use std::io;

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use sonoro_common::fs::FileSystem;
use sonoro_proto::pb::{
    ClientStorageInfo, ClientStorageInfoDataOneof, ClientStorageLongData, ClientStorageStringData,
};
use sonoro_proto::wire::PbMessage;

use crate::component::player::PlayerProgressComponent;

/// Encodes a protobuf sub-message as base64 and persists it in `PlayerProgressComponent`.
pub fn save_msg<M: PbMessage>(
    fs: &FileSystem,
    pc: &mut PlayerProgressComponent,
    key: &str,
    msg: &M,
) -> io::Result<()> {
    let b64 = STANDARD.encode(msg.encode_to_vec());
    pc.info.set_key_str(key, &b64);
    pc.save(fs)
}

/// Decodes a persisted protobuf sub-message from `PlayerProgressComponent`, or `None` if unset/corrupt.
pub fn load_msg<M: PbMessage>(pc: &PlayerProgressComponent, key: &str) -> Option<M> {
    let b64 = pc.info.get_key_str(key);
    if b64.is_empty() {
        return None;
    }
    let bytes = STANDARD.decode(b64).ok()?;
    M::decode(&bytes).ok()
}

/// Persists `msg_opt` under `key` when present.
pub fn persist_opt_msg<M: PbMessage>(
    fs: &FileSystem,
    pc: &mut PlayerProgressComponent,
    key: &str,
    msg_opt: Option<&M>,
) -> io::Result<()> {
    if let Some(msg) = msg_opt {
        save_msg(fs, pc, key, msg)?;
    }
    Ok(())
}

/// Persists an integer value under a formatted `prefix:id` key and saves `PlayerProgressComponent`.
pub fn persist_int(
    fs: &FileSystem,
    pc: &mut PlayerProgressComponent,
    key: &str,
    val: i64,
) -> io::Result<()> {
    pc.info.set_key_int(key, val);
    pc.save(fs)
}

/// Persists a string value under a formatted key and saves `PlayerProgressComponent`.
pub fn persist_str(
    fs: &FileSystem,
    pc: &mut PlayerProgressComponent,
    key: &str,
    val: &str,
) -> io::Result<()> {
    pc.info.set_key_str(key, val);
    pc.save(fs)
}

/// Persists a one-time claim/flag key and saves `PlayerProgressComponent`.
pub fn persist_claim(fs: &FileSystem, pc: &mut PlayerProgressComponent, key: &str) -> io::Result<()> {
    let _ = pc.info.mark_claim(key);
    pc.save(fs)
}

/// Appends persisted `ClientStorageInfo` entries (`cstorage:msg:*`, `cstorage:i:*`, `cstorage:s:*`)
/// without duplicating `SystemId`s.
pub fn append_stored_client_storage_infos(
    progress_comp: &PlayerProgressComponent,
    infos: &mut Vec<ClientStorageInfo>,
) {
    let mut seen_sys: HashSet<i32> = infos.iter().map(|existing| existing.SystemId).collect();

    for entry in &progress_comp.info.dynamic.strs {
        let Some(rest) = entry.key.strip_prefix("cstorage:msg:") else {
            continue;
        };
        let Ok(sys_id) = rest.parse::<i32>() else {
            continue;
        };
        if seen_sys.contains(&sys_id) {
            continue;
        }
        if let Some(msg) = load_msg::<ClientStorageInfo>(progress_comp, &entry.key) {
            seen_sys.insert(sys_id);
            infos.push(msg);
        }
    }

    for entry in &progress_comp.info.dynamic.ints {
        let Some(rest) = entry.key.strip_prefix("cstorage:i:") else {
            continue;
        };
        let Ok(sys_id) = rest.parse::<i32>() else {
            continue;
        };
        if !seen_sys.insert(sys_id) {
            continue;
        }
        infos.push(ClientStorageInfo {
            SystemId: sys_id,
            Data: Some(ClientStorageInfoDataOneof::LongData(ClientStorageLongData {
                Data: entry.val,
            })),
        });
    }

    for entry in &progress_comp.info.dynamic.strs {
        let Some(rest) = entry.key.strip_prefix("cstorage:s:") else {
            continue;
        };
        let Ok(sys_id) = rest.parse::<i32>() else {
            continue;
        };
        if !seen_sys.insert(sys_id) {
            continue;
        }
        infos.push(ClientStorageInfo {
            SystemId: sys_id,
            Data: Some(ClientStorageInfoDataOneof::StringData(
                ClientStorageStringData {
                    Data: entry.val.as_bytes().to_vec(),
                },
            )),
        });
    }
}
