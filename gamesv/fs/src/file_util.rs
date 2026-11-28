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
    if let Some(bytes) = fs.read_file(path)? {
        return parse_zon::<T>(&bytes).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("parse {path}: {e}"))
        });
    }
    fs.modify_file(path, |existing| {
        if let Some(bytes) = existing {
            let parsed = parse_zon::<T>(bytes).map_err(|e| {
                io::Error::new(io::ErrorKind::InvalidData, format!("parse {path}: {e}"))
            })?;
            return Ok((None, parsed));
        }
        let default_val = T::default();
        let serialized = serialize_zon(&default_val)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
        Ok((Some(serialized), default_val))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
    struct SampleSave {
        level: i32,
    }

    fn temp_dir(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("sonoro_file_util_{label}_{nanos}"));
        std::fs::create_dir_all(&dir).expect("mkdir");
        dir
    }

    #[test]
    fn load_or_create_zon_preserves_corrupt_files_instead_of_overwriting() {
        let dir = temp_dir("corrupt");
        let fs_state = FileSystem::open(&dir).expect("open fs");

        let created = load_or_create_zon::<SampleSave>(&fs_state, "player/1/sample")
            .expect("create default");
        assert_eq!(created, SampleSave { level: 0 });

        let corrupt_bytes = b".{ .level = not_an_int ";
        fs_state
            .write_file("player/1/corrupt", corrupt_bytes)
            .expect("write corrupt");
        fs_state.flush();

        let err = load_or_create_zon::<SampleSave>(&fs_state, "player/1/corrupt")
            .expect_err("corrupt file must return Err");
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert_eq!(
            fs_state.read_file("player/1/corrupt").expect("read back"),
            Some(corrupt_bytes.to_vec())
        );

        let _ = std::fs::remove_dir_all(&dir);
    }
}
