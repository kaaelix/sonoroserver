use serde::Serialize;
use serde::de::DeserializeOwned;
use sonoro_common::fs::{DirEntry, FileSystem};
use sonoro_gamesv_data::Assets;
use sonoro_gamesv_fs::file_util::{load_zon, parse_zon, serialize_zon};
use std::collections::BTreeMap;
use std::io;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreType {
    ByDataId,
    ByIncrId,
}

pub trait FsStoreItem: Sized + DeserializeOwned + Serialize {
    const DATA_DIR: &'static str;
    const SHARED_INCR_ID: bool = false;
    fn add_defaults(assets: &Assets, map: &mut BTreeMap<i32, Self>);
}

pub fn entry_file_name(entry: &DirEntry) -> Option<&str> {
    if entry.is_dir {
        return None;
    }
    Path::new(&entry.path).file_name()?.to_str()
}

pub fn save_struct<T: Serialize>(fs: &FileSystem, data: &T, path: &str) -> io::Result<()> {
    let bytes =
        serialize_zon(data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
    fs.write_file(path, &bytes)
}

pub fn load_items<T: FsStoreItem>(
    fs: &FileSystem,
    assets: &Assets,
    player_id: i32,
    store_type: StoreType,
) -> io::Result<BTreeMap<i32, T>> {
    let mut map = BTreeMap::new();
    let dir_path = format!("player/{player_id}/{}", T::DATA_DIR);

    if let Some(entries) = fs.read_dir(&dir_path)? {
        for entry in entries {
            let Some(name) = entry_file_name(&entry) else {
                continue;
            };
            let Ok(unique_id) = name.parse::<i32>() else {
                continue;
            };
            let item_path = format!("player/{player_id}/{}/{unique_id}", T::DATA_DIR);
            match load_zon::<T>(fs, &item_path) {
                Ok(Some(item)) => {
                    map.insert(unique_id, item);
                }
                Ok(None) => {}
                Err(err) => {
                    tracing::error!(unique_id, %err, "failed to load item from {item_path}");
                }
            }
        }
    } else {
        T::add_defaults(assets, &mut map);
        let mut highest_id = 0i32;
        for (&id, item) in &map {
            highest_id = highest_id.max(id);
            let item_path = format!("player/{player_id}/{}/{id}", T::DATA_DIR);
            save_struct(fs, item, &item_path)?;
        }
        if store_type == StoreType::ByIncrId && !T::SHARED_INCR_ID {
            let counter_path = format!("player/{player_id}/{}/next", T::DATA_DIR);
            fs.write_file(&counter_path, format!("{}", highest_id + 1).as_bytes())?;
        }
    }

    Ok(map)
}

pub fn parse_item_bytes<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, String> {
    parse_zon::<T>(bytes).map_err(|e| e.to_string())
}
