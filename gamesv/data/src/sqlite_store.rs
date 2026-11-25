use std::path::Path;
use std::sync::{Arc, Mutex, mpsc};
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::params;
use sonoro_common::fs::{FileSystem, SyncOp};
use sonoro_common::sqlite::{Db, SqliteError};
use sonoro_common::zon;

enum SyncCommand {
    Write { rel_path: String, content: Vec<u8> },
    DeleteFile { rel_path: String },
    DeleteTree { rel_prefix: String },
    Flush(mpsc::Sender<()>),
}

pub struct SqliteStore {
    db: Arc<Mutex<Db>>,
    tx: mpsc::Sender<SyncCommand>,
}

impl SqliteStore {
    pub fn open(db_path: impl AsRef<Path>) -> Result<Arc<Self>, SqliteError> {
        let db = Db::open(db_path)?;
        db.exec(
            "
            CREATE TABLE IF NOT EXISTS state_files (
                path TEXT PRIMARY KEY,
                content BLOB NOT NULL,
                updated_at INTEGER NOT NULL
            );
            CREATE TABLE IF NOT EXISTS players (
                player_id INTEGER PRIMARY KEY,
                name TEXT NOT NULL DEFAULT '',
                level INTEGER NOT NULL DEFAULT 1,
                exp INTEGER NOT NULL DEFAULT 0,
                coin INTEGER NOT NULL DEFAULT 0,
                rare_coin INTEGER NOT NULL DEFAULT 0,
                cash_coin INTEGER NOT NULL DEFAULT 0,
                sex INTEGER NOT NULL DEFAULT 1,
                world_level INTEGER NOT NULL DEFAULT 1,
                updated_at INTEGER NOT NULL DEFAULT 0
            );
            CREATE TABLE IF NOT EXISTS player_battlepass (
                player_id INTEGER PRIMARY KEY,
                bp_id INTEGER NOT NULL DEFAULT 1,
                level INTEGER NOT NULL DEFAULT 70,
                exp INTEGER NOT NULL DEFAULT 0,
                weekly_exp INTEGER NOT NULL DEFAULT 0,
                pay_status INTEGER NOT NULL DEFAULT 2,
                had_enter INTEGER NOT NULL DEFAULT 1,
                taken_rewards_json TEXT NOT NULL DEFAULT '[]',
                taken_tasks_json TEXT NOT NULL DEFAULT '[]',
                updated_at INTEGER NOT NULL DEFAULT 0
            );
            CREATE TABLE IF NOT EXISTS player_activity_claims (
                player_id INTEGER NOT NULL,
                claim_key TEXT NOT NULL,
                value_json TEXT NOT NULL DEFAULT '1',
                updated_at INTEGER NOT NULL DEFAULT 0,
                PRIMARY KEY (player_id, claim_key)
            );
            CREATE TABLE IF NOT EXISTS player_liveness (
                player_id INTEGER PRIMARY KEY,
                liveness_count INTEGER NOT NULL DEFAULT 100,
                claimed_boxes_json TEXT NOT NULL DEFAULT '[]',
                claimed_tasks_json TEXT NOT NULL DEFAULT '[]',
                updated_at INTEGER NOT NULL DEFAULT 0
            );
            CREATE TABLE IF NOT EXISTS player_achievements (
                player_id INTEGER NOT NULL,
                achievement_id INTEGER NOT NULL,
                status INTEGER NOT NULL DEFAULT 1,
                progress INTEGER NOT NULL DEFAULT 1,
                is_taken INTEGER NOT NULL DEFAULT 0,
                updated_at INTEGER NOT NULL DEFAULT 0,
                PRIMARY KEY (player_id, achievement_id)
            );
            CREATE TABLE IF NOT EXISTS player_shop_bought (
                player_id INTEGER NOT NULL,
                shop_id INTEGER NOT NULL,
                goods_id INTEGER NOT NULL,
                bought_count INTEGER NOT NULL DEFAULT 0,
                updated_at INTEGER NOT NULL DEFAULT 0,
                PRIMARY KEY (player_id, shop_id, goods_id)
            );
            CREATE TABLE IF NOT EXISTS player_kv (
                player_id INTEGER NOT NULL,
                ns TEXT NOT NULL,
                k TEXT NOT NULL,
                val TEXT NOT NULL,
                updated_at INTEGER NOT NULL DEFAULT 0,
                PRIMARY KEY (player_id, ns, k)
            );
            ",
        )?;
        let db = Arc::new(Mutex::new(db));
        let (tx, rx) = mpsc::channel();
        let worker_db = Arc::clone(&db);
        std::thread::Builder::new()
            .name("sqlite-mirror".to_owned())
            .spawn(move || writer_loop(worker_db, rx))
            .expect("failed to spawn sqlite mirror writer");
        Ok(Arc::new(Self { db, tx }))
    }

    pub fn flush(&self) {
        let (ack_tx, ack_rx) = mpsc::channel();
        if self.tx.send(SyncCommand::Flush(ack_tx)).is_ok() {
            let _ = ack_rx.recv();
        }
    }

    pub fn attach_to_fs(self: &Arc<Self>, fs: &FileSystem) -> Result<(), SqliteError> {
        self.backfill_existing_state(fs)?;
        let store = Arc::clone(self);
        fs.set_sync_hook(Arc::new(move |op, rel_path, content| {
            store.handle_sync(op, rel_path, content);
        }));
        Ok(())
    }

    pub fn backfill_existing_state(&self, fs: &FileSystem) -> Result<(), SqliteError> {
        let guard = self
            .db
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        guard.connection().execute_batch("BEGIN IMMEDIATE;")?;
        let now = unix_now_secs();
        let res = backfill_state_locked(&guard, fs, now);
        if res.is_err() {
            let _ = guard.connection().execute_batch("ROLLBACK;");
            return res;
        }
        guard.connection().execute_batch("COMMIT;")?;
        Ok(())
    }

    fn handle_sync(&self, op: SyncOp, rel_path: &str, content: &[u8]) {
        let cmd = match op {
            SyncOp::Write => SyncCommand::Write {
                rel_path: rel_path.to_owned(),
                content: content.to_vec(),
            },
            SyncOp::DeleteFile => SyncCommand::DeleteFile {
                rel_path: rel_path.to_owned(),
            },
            SyncOp::DeleteTree => SyncCommand::DeleteTree {
                rel_prefix: rel_path.to_owned(),
            },
        };
        let _ = self.tx.send(cmd);
    }

    pub fn read_file_content(&self, rel_path: &str) -> Result<Option<Vec<u8>>, SqliteError> {
        self.flush();
        let guard = self
            .db
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut stmt = guard
            .connection()
            .prepare_cached("SELECT content FROM state_files WHERE path = ?1")?;
        let mut rows = stmt.query(params![rel_path])?;
        if let Some(row) = rows.next()? {
            let blob: Vec<u8> = row.get(0)?;
            Ok(Some(blob))
        } else {
            Ok(None)
        }
    }
}

fn writer_loop(db: Arc<Mutex<Db>>, rx: mpsc::Receiver<SyncCommand>) {
    let mut batch = Vec::with_capacity(64);
    while let Ok(first) = rx.recv() {
        batch.push(first);
        while let Ok(next) = rx.try_recv() {
            batch.push(next);
        }

        let has_writes = batch.iter().any(|cmd| !matches!(cmd, SyncCommand::Flush(_)));
        let mut flushes = Vec::new();
        if has_writes {
            let guard = db.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            let in_tx = guard
                .connection()
                .execute_batch("BEGIN IMMEDIATE;")
                .is_ok();
            let now = unix_now_secs();
            for cmd in batch.drain(..) {
                match cmd {
                    SyncCommand::Write { rel_path, content } => {
                        let _ = mirror_write_locked(&guard, &rel_path, &content, now);
                    }
                    SyncCommand::DeleteFile { rel_path } => {
                        delete_file_locked(&guard, &rel_path);
                    }
                    SyncCommand::DeleteTree { rel_prefix } => {
                        delete_tree_locked(&guard, &rel_prefix);
                    }
                    SyncCommand::Flush(ack) => flushes.push(ack),
                }
            }
            if in_tx {
                let _ = guard.connection().execute_batch("COMMIT;");
            }
        } else {
            for cmd in batch.drain(..) {
                if let SyncCommand::Flush(ack) = cmd {
                    flushes.push(ack);
                }
            }
        }

        for ack in flushes {
            let _ = ack.send(());
        }
    }
}

fn delete_file_locked(db: &Db, rel_path: &str) {
    if let Ok(mut stmt) = db
        .connection()
        .prepare_cached("DELETE FROM state_files WHERE path = ?1")
    {
        let _ = stmt.execute(params![rel_path]);
    }
}

fn delete_tree_locked(db: &Db, rel_prefix: &str) {
    let pattern = format!("{rel_prefix}/%");
    if let Ok(mut stmt) = db
        .connection()
        .prepare_cached("DELETE FROM state_files WHERE path = ?1 OR path LIKE ?2")
    {
        let _ = stmt.execute(params![rel_prefix, pattern]);
    }
    if let Some(uid_str) = rel_prefix.strip_prefix("player/")
        && let Ok(uid) = uid_str.parse::<i64>()
        && let Ok(mut stmt) = db
            .connection()
            .prepare_cached("DELETE FROM players WHERE player_id = ?1")
    {
        let _ = stmt.execute(params![uid]);
    }
}

fn upsert_player_from_zon_locked(db: &Db, uid: i64, zon_bytes: &[u8], now: i64) {
    let Ok(zon_str) = std::str::from_utf8(zon_bytes) else {
        return;
    };
    let Ok(parsed) = zon::parse(zon_str) else {
        return;
    };
    let attrs = parsed.get("attributes").unwrap_or(&parsed);
    let read_str = |key: &str| {
        attrs
            .get(key)
            .or_else(|| parsed.get(key))
            .and_then(zon::ZonValue::as_str)
    };
    let read_i64 = |key: &str| {
        attrs
            .get(key)
            .or_else(|| parsed.get(key))
            .and_then(zon::ZonValue::as_i64)
    };
    let name = read_str("name").unwrap_or("");
    let level = read_i64("level").unwrap_or(1);
    let exp = read_i64("exp").unwrap_or(0);
    let coin = read_i64("coin").unwrap_or(0);
    let rare_coin = read_i64("rare_coin").unwrap_or(0);
    let cash_coin = read_i64("cash_coin").unwrap_or(0);
    let sex = read_i64("sex").unwrap_or(1);
    let world_level = read_i64("cur_world_level").unwrap_or(1);
    if let Ok(mut stmt) = db.connection().prepare_cached(
        "INSERT INTO players(player_id, name, level, exp, coin, rare_coin, cash_coin, sex, world_level, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
         ON CONFLICT(player_id) DO UPDATE SET
             name = excluded.name,
             level = excluded.level,
             exp = excluded.exp,
             coin = excluded.coin,
             rare_coin = excluded.rare_coin,
             cash_coin = excluded.cash_coin,
             sex = excluded.sex,
             world_level = excluded.world_level,
             updated_at = excluded.updated_at",
    ) {
        let _ = stmt.execute(params![
            uid,
            name,
            level,
            exp,
            coin,
            rare_coin,
            cash_coin,
            sex,
            world_level,
            now
        ]);
    }
}

fn backfill_state_locked(db: &Db, fs: &FileSystem, now: i64) -> Result<(), SqliteError> {
    for sub in ["player", "account", "social"] {
        let Ok(Some(entries)) = fs.read_dir(sub) else {
            continue;
        };
        for entry in entries {
            if entry.is_dir {
                continue;
            }
            let Ok(Some(bytes)) = fs.read_file(&entry.path) else {
                continue;
            };
            mirror_write_locked(db, &entry.path, &bytes, now)?;
        }
    }
    Ok(())
}

fn mirror_write_locked(db: &Db, rel_path: &str, content: &[u8], now: i64) -> Result<(), SqliteError> {
    let mut stmt = db.connection().prepare_cached(
        "INSERT INTO state_files(path, content, updated_at) VALUES (?1, ?2, ?3)
         ON CONFLICT(path) DO UPDATE SET content = excluded.content, updated_at = excluded.updated_at",
    )?;
    stmt.execute(params![rel_path, content, now])?;

    if let Some(uid_str) = rel_path
        .strip_prefix("player/")
        .and_then(|s| s.strip_suffix("/basic_info"))
        && let Ok(uid) = uid_str.parse::<i64>()
    {
        upsert_player_from_zon_locked(db, uid, content, now);
    }
    Ok(())
}

fn unix_now_secs() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mirrors_writes_and_deletes_into_sqlite() {
        let dir = std::env::temp_dir().join(format!("sonoro_sq_test_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("player/100000001")).unwrap();

        let fs = FileSystem::open(&dir).unwrap();
        let db_path = dir.join("test.db");
        let store = SqliteStore::open(&db_path).unwrap();
        store.attach_to_fs(&fs).unwrap();

        fs.write_file(
            "player/100000001/basic_info",
            br#".{ .name = "Rover", .level = 10, .cur_world_level = 2 }"#,
        )
        .unwrap();

        let blob = store
            .read_file_content("player/100000001/basic_info")
            .unwrap()
            .unwrap();
        assert!(String::from_utf8_lossy(&blob).contains("Rover"));

        let (name, level, world_level): (String, i64, i64) = store
            .db
            .lock()
            .unwrap()
            .connection()
            .query_row(
                "SELECT name, level, world_level FROM players WHERE player_id = 100000001",
                [],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .unwrap();
        assert_eq!(name, "Rover");
        assert_eq!(level, 10);
        assert_eq!(world_level, 2);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn backfills_existing_state_in_transaction() {
        let dir = std::env::temp_dir().join(format!("sonoro_sq_backfill_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("player")).unwrap();
        std::fs::write(dir.join("player/a.zon"), b"alpha").unwrap();
        std::fs::write(dir.join("player/b.zon"), b"beta").unwrap();

        let fs = FileSystem::open(&dir).unwrap();
        let store = SqliteStore::open(dir.join("test.db")).unwrap();
        store.backfill_existing_state(&fs).unwrap();

        assert_eq!(
            store.read_file_content("player/a.zon").unwrap().as_deref(),
            Some(b"alpha".as_slice())
        );
        assert_eq!(
            store.read_file_content("player/b.zon").unwrap().as_deref(),
            Some(b"beta".as_slice())
        );

        let _ = std::fs::remove_dir_all(&dir);
    }
}
