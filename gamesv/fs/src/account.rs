use std::collections::HashSet;
use std::io;

use serde::{Deserialize, Serialize};
use sonoro_common::fs::FileSystem;
use sonoro_gamesv_data::GameConfig;

use crate::file_util;
use crate::incr;

const MAX_UID_ALLOC_ATTEMPTS: usize = 100_000;

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
        return Ok(false);
    };
    let Ok(login_account) = file_util::parse_zon::<LoginAccount>(&bytes) else {
        return Ok(false);
    };
    Ok(login_account.token == token)
}

fn parse_account_link(name: &str, bytes: &[u8]) -> io::Result<Account> {
    file_util::parse_zon::<Account>(bytes).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("account '{name}' corrupted: {e}"),
        )
    })
}

fn collect_claimed_player_ids(fs: &FileSystem) -> io::Result<HashSet<i32>> {
    let mut claimed = HashSet::new();
    if let Some(entries) = fs.read_dir("account/link")? {
        for entry in entries {
            if entry.is_dir {
                continue;
            }
            if let Ok(Some(link)) = file_util::load_zon::<Account>(fs, &entry.path) {
                claimed.insert(link.player_id);
            }
        }
    }
    Ok(claimed)
}

fn allocate_unique_player_id(
    fs: &FileSystem,
    claimed_ids: &HashSet<i32>,
) -> io::Result<i32> {
    let default_uid = GameConfig::embedded().default_uid;
    for _ in 0..MAX_UID_ALLOC_ATTEMPTS {
        let candidate = incr::next_starting_at(fs, "player/next", default_uid)?;
        if candidate <= 0 || claimed_ids.contains(&candidate) {
            continue;
        }
        if fs.root_dir().join(format!("player/{candidate}")).exists() {
            continue;
        }
        if fs
            .read_file(&format!("player/{candidate}/basic_info"))?
            .is_some()
        {
            continue;
        }
        return Ok(candidate);
    }
    Err(io::Error::other(
        "exhausted attempts to allocate unique player id",
    ))
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
            return parse_account_link(name, &bytes);
        }

        // Snapshot existing links (and flush pending writes) before acquiring the entry lock.
        let claimed_ids = collect_claimed_player_ids(fs)?;
        fs.modify_file(&path, |existing| {
            if let Some(bytes) = existing {
                let account = parse_account_link(name, bytes)?;
                return Ok((None, account));
            }
            let player_id = allocate_unique_player_id(fs, &claimed_ids)?;
            let account = Self { player_id };
            let data = file_util::serialize_zon(&account)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
            Ok((Some(data), account))
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("sonoro_gamesv_acct_{label}_{nanos}"));
        std::fs::create_dir_all(&dir).expect("mkdir");
        dir
    }

    #[test]
    fn verify_login_token_rejects_missing_corrupt_and_mismatched_accounts() {
        let dir = temp_dir("verify_token");
        let fs_state = FileSystem::open(&dir).expect("open fs");

        assert!(!verify_login_token(&fs_state, "missing_user", "any").expect("missing"));
        assert!(!verify_login_token(&fs_state, "../escape", "any").expect("unsafe"));

        fs_state
            .write_file(
                "account/alice",
                br#".{ .user_id = "alice", .user_name = "Alice", .token = "secret", .sex = -1 }"#,
            )
            .expect("write account");
        assert!(verify_login_token(&fs_state, "alice", "secret").expect("valid"));
        assert!(!verify_login_token(&fs_state, "alice", "wrong").expect("wrong token"));

        fs_state
            .write_file("account/corrupt", b"not valid zon")
            .expect("write corrupt");
        assert!(!verify_login_token(&fs_state, "corrupt", "any").expect("corrupt"));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn load_or_create_prevents_numeric_hijack_and_preserves_existing_links() {
        let dir = temp_dir("link_collision");
        let fs_state = FileSystem::open(&dir).expect("open fs");

        let alice = Account::load_or_create(&fs_state, "alice").expect("create alice");
        let again = Account::load_or_create(&fs_state, "alice").expect("reload alice");
        assert_eq!(alice.player_id, again.player_id);

        // Numeric account matching alice's player_id must allocate a fresh unique ID.
        let spoof = Account::load_or_create(&fs_state, &alice.player_id.to_string())
            .expect("create numeric");
        assert_ne!(spoof.player_id, alice.player_id);

        // Pre-existing unlinked player directory must not be hijacked by numeric account name.
        fs_state
            .write_file("player/597938510/basic_info", b".{}")
            .expect("seed existing player");
        let numeric = Account::load_or_create(&fs_state, "597938510").expect("create 597938510");
        assert_ne!(numeric.player_id, 597_938_510);

        // Existing link file keeps working cleanly.
        fs_state
            .write_file("account/link/legacy", b".{ .player_id = 597938510 }")
            .expect("seed legacy link");
        let legacy = Account::load_or_create(&fs_state, "legacy").expect("load legacy");
        assert_eq!(legacy.player_id, 597_938_510);

        let _ = std::fs::remove_dir_all(&dir);
    }
}
