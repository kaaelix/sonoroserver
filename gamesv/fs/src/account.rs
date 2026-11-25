use std::io;

use serde::{Deserialize, Serialize};
use sonoro_common::fs::FileSystem;
use sonoro_gamesv_data::GameConfig;

use crate::file_util;
use crate::incr;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Account {
    pub player_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct LoginAccount {
    pub user_id: String,
    pub user_name: String,
    pub token: String,
    #[serde(default = "default_sex")]
    pub sex: i32,
}

const fn default_sex() -> i32 {
    -1
}

pub fn is_safe_identifier(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 128
        && id != "."
        && id != ".."
        && !id
            .bytes()
            .any(|b| b == b'/' || b == b'\\' || b == 0 || b.is_ascii_control())
}

pub fn verify_login_token(fs: &FileSystem, name: &str, token: &str) -> io::Result<bool> {
    if !is_safe_identifier(name) {
        return Ok(false);
    }
    let path = format!("account/{name}");
    let Some(bytes) = fs.read_file(&path)? else {
        return Ok(true);
    };
    let Ok(login_account) = file_util::parse_zon::<LoginAccount>(&bytes) else {
        return Ok(false);
    };
    Ok(login_account.token == token)
}

impl Account {
    pub fn load_or_create(fs: &FileSystem, name: &str) -> io::Result<Self> {
        if !is_safe_identifier(name) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid account identifier",
            ));
        }
        let path = format!("account/link/{name}");
        if let Some(bytes) = fs.read_file(&path)? {
            return file_util::parse_zon::<Self>(&bytes).map_err(|e| {
                io::Error::new(io::ErrorKind::InvalidData, format!("account '{name}' corrupted: {e}"))
            });
        }

        let player_id = match name.parse::<i32>() {
            Ok(id) if (10_000_000..2_000_000_000).contains(&id) => id,
            _ => incr::next_starting_at(fs, "player/next", GameConfig::embedded().default_uid)?,
        };
        let account = Self { player_id };
        let data = file_util::serialize_zon(&account)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
        fs.write_file(&path, &data)?;
        Ok(account)
    }

    pub fn sync_login_sex_for_player(fs: &FileSystem, player_id: i32, sex: i32) -> io::Result<()> {
        let Some(entries) = fs.read_dir("account/link")? else {
            return Ok(());
        };
        for entry in entries {
            if entry.is_dir {
                continue;
            }
            let Ok(Some(link)) = file_util::load_zon::<Account>(fs, &entry.path) else {
                continue;
            };
            if link.player_id != player_id {
                continue;
            }
            let account_path = format!("account/{}", entry.basename());
            let Some(mut login) = file_util::load_zon::<LoginAccount>(fs, &account_path)? else {
                return Ok(());
            };
            login.sex = sex;
            let data = file_util::serialize_zon(&login)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
            fs.write_file(&account_path, &data)?;
            return Ok(());
        }
        Ok(())
    }
}
