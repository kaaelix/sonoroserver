use std::io;

use serde::Serialize;
use serde::de::DeserializeOwned;
use sonoro_common::fs::FileSystem;

use crate::zon_codec::{self, ZonCodecError};

pub fn parse_zon<T: DeserializeOwned>(content: &[u8]) -> Result<T, ZonCodecError> {
    let text =
        std::str::from_utf8(content).map_err(|e| ZonCodecError::Custom(e.to_string()))?;
    zon_codec::from_zon_str(text)
}

pub fn serialize_zon<T: Serialize>(data: &T) -> Result<Vec<u8>, ZonCodecError> {
    zon_codec::to_zon_string(data).map(String::into_bytes)
}

pub fn load_zon<T: DeserializeOwned>(fs: &FileSystem, path: &str) -> io::Result<Option<T>> {
    let Some(bytes) = fs.read_file(path)? else {
        return Ok(None);
    };
    let parsed = parse_zon(&bytes)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("parse {path}: {e}")))?;
    Ok(Some(parsed))
}

pub fn load_or_create_zon<T>(fs: &FileSystem, path: &str) -> io::Result<T>
where
    T: DeserializeOwned + Serialize + Default,
{
    if let Some(bytes) = fs.read_file(path)?
        && let Ok(parsed) = parse_zon::<T>(&bytes)
    {
        return Ok(parsed);
    }
    let default_val = T::default();
    let serialized = serialize_zon(&default_val)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
    fs.write_file(path, &serialized)?;
    Ok(default_val)
}
