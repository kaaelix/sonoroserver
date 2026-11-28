//! Account persistence stored in ZON format under `state/account/<user_id>`.

use sonoro_common::FileSystem;
use sonoro_common::zon::{self, ZonValue};
use thiserror::Error;

/// Persisted account record matching `loginsv/src/fs/Account.zig`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    pub user_id: String,
    pub user_name: String,
    pub token: String,
    pub sex: i32,
}

/// Errors encountered while validating, loading, or persisting an account.
#[derive(Debug, Error)]
pub enum AccountError {
    #[error("invalid user id")]
    InvalidUserId,
    #[error("filesystem error: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to parse account ZON: {0}")]
    ParseFailed(String),
}

/// Validates that `id` is a safe single-segment account identifier.
pub fn is_safe_identifier(id: &str) -> bool {
    if id.is_empty() || id.len() > 128 {
        return false;
    }
    if id == "." || id == ".." {
        return false;
    }
    !id.bytes()
        .any(|b| b == b'/' || b == b'\\' || b == 0 || b.is_ascii_control())
}

impl Account {
    fn from_zon(val: &ZonValue) -> Result<Self, AccountError> {
        let user_id = val
            .get("user_id")
            .and_then(ZonValue::as_str)
            .ok_or_else(|| AccountError::ParseFailed("missing user_id".into()))?
            .to_string();
        let user_name = val
            .get("user_name")
            .and_then(ZonValue::as_str)
            .ok_or_else(|| AccountError::ParseFailed("missing user_name".into()))?
            .to_string();
        let token = val
            .get("token")
            .and_then(ZonValue::as_str)
            .ok_or_else(|| AccountError::ParseFailed("missing token".into()))?
            .to_string();
        let sex = match val.get("sex") {
            Some(v) => {
                let raw = v
                    .as_i64()
                    .ok_or_else(|| AccountError::ParseFailed("invalid sex".into()))?;
                i32::try_from(raw)
                    .map_err(|_| AccountError::ParseFailed("sex out of range".into()))?
            }
            None => -1,
        };

        Ok(Self {
            user_id,
            user_name,
            token,
            sex,
        })
    }

    fn to_zon(&self) -> ZonValue {
        ZonValue::Struct(vec![
            ("user_id".into(), ZonValue::Str(self.user_id.clone())),
            ("user_name".into(), ZonValue::Str(self.user_name.clone())),
            ("token".into(), ZonValue::Str(self.token.clone())),
            ("sex".into(), ZonValue::Int(i64::from(self.sex))),
        ])
    }

    /// Loads an existing account or creates one if absent, updating token/name when changed.
    pub fn load_or_create(
        fs: &FileSystem,
        user_id: &str,
        user_name: &str,
        token: &str,
    ) -> Result<Self, AccountError> {
        if !is_safe_identifier(user_id) {
            return Err(AccountError::InvalidUserId);
        }

        let path = format!("account/{user_id}");
        fs.modify_file(&path, |existing| {
            if let Some(raw) = existing {
                let text = match std::str::from_utf8(raw) {
                    Ok(t) => t,
                    Err(e) => return Ok((None, Err(AccountError::ParseFailed(e.to_string())))),
                };
                let parsed = match zon::parse(text) {
                    Ok(v) => v,
                    Err(e) => return Ok((None, Err(AccountError::ParseFailed(e.to_string())))),
                };
                let mut account = match Self::from_zon(&parsed) {
                    Ok(acc) => acc,
                    Err(e) => return Ok((None, Err(e))),
                };

                if account.token != token && !token.is_empty() {
                    account.token = token.to_string();
                    if !user_name.is_empty() {
                        account.user_name = user_name.to_string();
                    }
                    let serialized = zon::stringify(&account.to_zon());
                    return Ok((Some(serialized.into_bytes()), Ok(account)));
                }
                Ok((None, Ok(account)))
            } else {
                let account = Self {
                    user_id: user_id.to_string(),
                    user_name: user_name.to_string(),
                    token: token.to_string(),
                    sex: -1,
                };
                let serialized = zon::stringify(&account.to_zon());
                Ok((Some(serialized.into_bytes()), Ok(account)))
            }
        })?
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_state() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("sonoro_account_test_{nanos}"));
        fs::create_dir_all(&dir).expect("mkdir");
        dir
    }

    #[test]
    fn validates_identifiers() {
        assert!(is_safe_identifier("597938510"));
        assert!(!is_safe_identifier(""));
        assert!(!is_safe_identifier("."));
        assert!(!is_safe_identifier(".."));
        assert!(!is_safe_identifier("a/b"));
        assert!(!is_safe_identifier("a\nb"));
    }

    #[test]
    fn creates_loads_and_updates_account() {
        let dir = temp_state();
        let fs_state = FileSystem::open(&dir).expect("open fs");

        let acc = Account::load_or_create(&fs_state, "1001", "PlayerA", "tok1").expect("create");
        assert_eq!(acc.sex, -1);
        assert_eq!(acc.token, "tok1");

        let updated =
            Account::load_or_create(&fs_state, "1001", "PlayerB", "tok2").expect("update");
        assert_eq!(updated.token, "tok2");
        assert_eq!(updated.user_name, "PlayerB");

        let _ = fs::remove_dir_all(&dir);
    }
}
