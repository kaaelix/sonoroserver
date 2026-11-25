use std::collections::BTreeMap;
use std::io;

use sonoro_common::fs::FileSystem;

use crate::incr;

pub const TRIAL_INCR_MIN: i32 = 2_000_000_000;

fn counter_path(player_id: i32) -> String {
    format!("player/{player_id}/special_item/next")
}

pub fn current(fs: &FileSystem, player_id: i32) -> io::Result<i32> {
    let path = counter_path(player_id);
    if let Some(bytes) = fs.read_file(&path)? {
        let text = std::str::from_utf8(&bytes)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let tok = text
            .split_whitespace()
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "empty counter file"))?;
        let id: i32 = tok
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid counter int"))?;
        return Ok(id);
    }
    Ok(1)
}

pub fn next(fs: &FileSystem, player_id: i32) -> io::Result<i32> {
    incr::next(fs, &counter_path(player_id))
}

pub fn ensure_at_least(fs: &FileSystem, player_id: i32, next_id: i32) -> io::Result<()> {
    if next_id <= 1 {
        return Ok(());
    }
    let path = counter_path(player_id);
    fs.modify_file(&path, |existing| {
        if let Some(bytes) = existing {
            let text = std::str::from_utf8(bytes)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            let tok = text
                .split_whitespace()
                .next()
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "empty counter"))?;
            let current_id: i32 = tok
                .parse()
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid counter int"))?;
            if current_id < next_id {
                Ok((Some(format!("{next_id}\n").into_bytes()), ()))
            } else {
                Ok((None, ()))
            }
        } else {
            Ok((Some(format!("{next_id}\n").into_bytes()), ()))
        }
    })
}

pub fn next_after_map<V>(map: &BTreeMap<i32, V>) -> i32 {
    let mut next_id = 1i32;
    for &k in map.keys() {
        if k >= TRIAL_INCR_MIN {
            continue;
        }
        next_id = next_id.max(k + 1);
    }
    next_id
}

pub fn next_after_maps<A, B>(a: &BTreeMap<i32, A>, b: &BTreeMap<i32, B>) -> i32 {
    next_after_map(a).max(next_after_map(b))
}
