//! Shared SQLite WAL persistence matching `services/aki-logic/SqliteStore.cs`.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{OptionalExtension, params};
use sonoro_common::sqlite::{Db, SqliteError};
use sonoro_common::zon::{self, ZonValue};

use crate::bin_data;

static INIT_DONE: OnceLock<PathBuf> = OnceLock::new();
static SHARED_DB: Mutex<Option<(PathBuf, Db)>> = Mutex::new(None);

/// Resolves the path to `gamesv.db` from `GAMESV_DB_PATH` or `<repo_root>/state/gamesv.db`.
pub fn db_path() -> &'static Path {
    INIT_DONE.get_or_init(|| {
        std::env::var_os("GAMESV_DB_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|| bin_data::repo_root().join("state").join("gamesv.db"))
    })
}

fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

fn ensure_schema_on(db: &Db) -> Result<(), SqliteError> {
    db.exec(
        "
CREATE TABLE IF NOT EXISTS state_files (
    path TEXT PRIMARY KEY,
    content TEXT NOT NULL,
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
    )
}

fn with_connection<T>(f: impl FnOnce(&mut Db) -> T) -> Result<T, SqliteError> {
    let target = db_path().to_path_buf();
    let mut guard = SHARED_DB
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let needs_open = guard.as_ref().is_none_or(|(p, _)| *p != target);
    if needs_open {
        if let Some(parent) = target.parent()
            && !parent.as_os_str().is_empty()
        {
            let _ = std::fs::create_dir_all(parent);
        }
        let db = Db::open(&target)?;
        ensure_schema_on(&db)?;
        *guard = Some((target, db));
    }
    let (_, db) = guard.as_mut().expect("shared db initialized above");
    Ok(f(db))
}

/// Ensures the shared SQLite schema exists on disk.
pub fn ensure_initialized() -> Result<(), SqliteError> {
    with_connection(|_| ())
}

#[derive(Debug, Clone)]
pub struct BattlePassRow {
    pub player_id: i32,
    pub bp_id: i32,
    pub level: i32,
    pub exp: i32,
    pub weekly_exp: i32,
    pub pay_status: i32,
    pub had_enter: bool,
    pub taken_rewards: Vec<Vec<i32>>,
    pub taken_tasks: Vec<i32>,
}

pub fn get_or_create_battle_pass(player_id: i32) -> BattlePassRow {
    if let Ok(Some(found)) = with_connection(|db| {
        let row: Option<BattlePassRow> = db
            .connection()
            .prepare_cached(
                "SELECT bp_id, level, exp, weekly_exp, pay_status, had_enter, taken_rewards_json, taken_tasks_json FROM player_battlepass WHERE player_id = ?1;",
            )
            .ok()
            .and_then(|mut stmt| {
                stmt.query_row([player_id], |r| {
                    let rew_json: String = r.get(6)?;
                    let tsk_json: String = r.get(7)?;
                    Ok(BattlePassRow {
                        player_id,
                        bp_id: r.get(0)?,
                        level: r.get(1)?,
                        exp: r.get(2)?,
                        weekly_exp: r.get(3)?,
                        pay_status: r.get(4)?,
                        had_enter: r.get::<_, i32>(5)? != 0,
                        taken_rewards: serde_json::from_str(&rew_json).unwrap_or_default(),
                        taken_tasks: serde_json::from_str(&tsk_json).unwrap_or_default(),
                    })
                })
                .optional()
                .ok()
                .flatten()
            });

        if row.is_none() {
            let _ = db.connection().execute(
                "INSERT OR IGNORE INTO player_battlepass(player_id, bp_id, level, exp, weekly_exp, pay_status, had_enter, taken_rewards_json, taken_tasks_json, updated_at) VALUES(?1, 1, 70, 0, 0, 2, 1, '[]', '[]', ?2);",
                params![player_id, unix_now()],
            );
        }
        row
    }) {
        return found;
    }

    BattlePassRow {
        player_id,
        bp_id: 1,
        level: 70,
        exp: 0,
        weekly_exp: 0,
        pay_status: 2,
        had_enter: true,
        taken_rewards: Vec::new(),
        taken_tasks: Vec::new(),
    }
}

pub fn save_battle_pass(row: &BattlePassRow) {
    let rew_json = serde_json::to_string(&row.taken_rewards).unwrap_or_else(|_| "[]".into());
    let tsk_json = serde_json::to_string(&row.taken_tasks).unwrap_or_else(|_| "[]".into());
    let _ = with_connection(|db| {
        if let Ok(mut stmt) = db.connection().prepare_cached(
            "INSERT INTO player_battlepass(player_id, bp_id, level, exp, weekly_exp, pay_status, had_enter, taken_rewards_json, taken_tasks_json, updated_at)
             VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
             ON CONFLICT(player_id) DO UPDATE SET
                 bp_id = excluded.bp_id,
                 level = excluded.level,
                 exp = excluded.exp,
                 weekly_exp = excluded.weekly_exp,
                 pay_status = excluded.pay_status,
                 had_enter = excluded.had_enter,
                 taken_rewards_json = excluded.taken_rewards_json,
                 taken_tasks_json = excluded.taken_tasks_json,
                 updated_at = excluded.updated_at;",
        ) {
            let _ = stmt.execute(params![
                row.player_id,
                row.bp_id,
                row.level,
                row.exp,
                row.weekly_exp,
                row.pay_status,
                i32::from(row.had_enter),
                rew_json,
                tsk_json,
                unix_now(),
            ]);
        }
    });
}

#[derive(Debug, Clone)]
pub struct LivenessRow {
    pub player_id: i32,
    pub liveness_count: i32,
    pub claimed_boxes: Vec<i32>,
    pub claimed_tasks: Vec<i32>,
}

pub fn get_or_create_liveness(player_id: i32) -> LivenessRow {
    if let Ok(Some(found)) = with_connection(|db| {
        let row: Option<LivenessRow> = db
            .connection()
            .prepare_cached(
                "SELECT liveness_count, claimed_boxes_json, claimed_tasks_json FROM player_liveness WHERE player_id = ?1;",
            )
            .ok()
            .and_then(|mut stmt| {
                stmt.query_row([player_id], |r| {
                    let boxes_json: String = r.get(1)?;
                    let tasks_json: String = r.get(2)?;
                    Ok(LivenessRow {
                        player_id,
                        liveness_count: r.get(0)?,
                        claimed_boxes: serde_json::from_str(&boxes_json).unwrap_or_default(),
                        claimed_tasks: serde_json::from_str(&tasks_json).unwrap_or_default(),
                    })
                })
                .optional()
                .ok()
                .flatten()
            });

        if row.is_none() {
            let _ = db.connection().execute(
                "INSERT OR IGNORE INTO player_liveness(player_id, liveness_count, claimed_boxes_json, claimed_tasks_json, updated_at) VALUES(?1, 100, '[]', '[]', ?2);",
                params![player_id, unix_now()],
            );
        }
        row
    }) {
        return found;
    }

    LivenessRow {
        player_id,
        liveness_count: 100,
        claimed_boxes: Vec::new(),
        claimed_tasks: Vec::new(),
    }
}

pub fn save_liveness(row: &LivenessRow) {
    let boxes = serde_json::to_string(&row.claimed_boxes).unwrap_or_else(|_| "[]".into());
    let tasks = serde_json::to_string(&row.claimed_tasks).unwrap_or_else(|_| "[]".into());
    let _ = with_connection(|db| {
        if let Ok(mut stmt) = db.connection().prepare_cached(
            "INSERT INTO player_liveness(player_id, liveness_count, claimed_boxes_json, claimed_tasks_json, updated_at)
             VALUES(?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(player_id) DO UPDATE SET
                 liveness_count = excluded.liveness_count,
                 claimed_boxes_json = excluded.claimed_boxes_json,
                 claimed_tasks_json = excluded.claimed_tasks_json,
                 updated_at = excluded.updated_at;",
        ) {
            let _ = stmt.execute(params![row.player_id, row.liveness_count, boxes, tasks, unix_now()]);
        }
    });
}

pub fn get_shop_bought_counts(player_id: i32, shop_id: i32) -> HashMap<i32, i32> {
    let mut map = HashMap::new();
    let _ = with_connection(|db| {
        if let Ok(mut stmt) = db.connection().prepare_cached(
            "SELECT goods_id, bought_count FROM player_shop_bought WHERE player_id = ?1 AND shop_id = ?2;",
        ) && let Ok(rows) = stmt.query_map(params![player_id, shop_id], |r| {
            Ok((r.get::<_, i32>(0)?, r.get::<_, i32>(1)?))
        }) {
            for (gid, cnt) in rows.flatten() {
                map.insert(gid, cnt);
            }
        }
    });
    map
}

pub fn increment_shop_bought(player_id: i32, shop_id: i32, goods_id: i32, delta: i32) {
    let _ = with_connection(|db| {
        if let Ok(mut stmt) = db.connection().prepare_cached(
            "INSERT INTO player_shop_bought(player_id, shop_id, goods_id, bought_count, updated_at)
             VALUES(?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(player_id, shop_id, goods_id) DO UPDATE SET
                 bought_count = player_shop_bought.bought_count + excluded.bought_count,
                 updated_at = excluded.updated_at;",
        ) {
            let _ = stmt.execute(params![player_id, shop_id, goods_id, delta, unix_now()]);
        }
    });
}

pub fn get_achievements(player_id: i32) -> HashMap<i32, (i32, bool)> {
    let mut map = HashMap::new();
    let _ = with_connection(|db| {
        if let Ok(mut stmt) = db.connection().prepare_cached(
            "SELECT achievement_id, progress, is_taken FROM player_achievements WHERE player_id = ?1;",
        ) && let Ok(rows) = stmt.query_map([player_id], |r| {
            Ok((
                r.get::<_, i32>(0)?,
                r.get::<_, i32>(1)?,
                r.get::<_, i32>(2)? != 0,
            ))
        }) {
            for (aid, prog, taken) in rows.flatten() {
                map.insert(aid, (prog, taken));
            }
        }
    });
    map
}

pub fn mark_achievements_taken(player_id: i32, achievement_ids: &[i32]) {
    if achievement_ids.is_empty() {
        return;
    }
    let _ = with_connection(|db| {
        if let Ok(tx) = db.connection_mut().transaction() {
            let now = unix_now();
            if let Ok(mut stmt) = tx.prepare_cached(
                "INSERT INTO player_achievements(player_id, achievement_id, status, progress, is_taken, updated_at)
                 VALUES(?1, ?2, 1, 1, 1, ?3)
                 ON CONFLICT(player_id, achievement_id) DO UPDATE SET
                     is_taken = 1,
                     updated_at = excluded.updated_at;",
            ) {
                for &id in achievement_ids {
                    let _ = stmt.execute(params![player_id, id, now]);
                }
            }
            let _ = tx.commit();
        }
    });
}

pub fn finish_achievement(player_id: i32, achievement_id: i32, progress: i32) {
    let _ = with_connection(|db| {
        if let Ok(mut stmt) = db.connection().prepare_cached(
            "INSERT INTO player_achievements(player_id, achievement_id, status, progress, is_taken, updated_at)
             VALUES(?1, ?2, 1, ?3, 0, ?4)
             ON CONFLICT(player_id, achievement_id) DO UPDATE SET
                 status = 1,
                 progress = MAX(player_achievements.progress, excluded.progress),
                 updated_at = excluded.updated_at;",
        ) {
            let _ = stmt.execute(params![player_id, achievement_id, progress, unix_now()]);
        }
    });
}

pub fn set_activity_claim(player_id: i32, claim_key: &str, value_json: &str) {
    let _ = with_connection(|db| {
        if let Ok(mut stmt) = db.connection().prepare_cached(
            "INSERT INTO player_activity_claims(player_id, claim_key, value_json, updated_at)
             VALUES(?1, ?2, ?3, ?4)
             ON CONFLICT(player_id, claim_key) DO UPDATE SET
                 value_json = excluded.value_json,
                 updated_at = excluded.updated_at;",
        ) {
            let _ = stmt.execute(params![player_id, claim_key, value_json, unix_now()]);
        }
    });
}

pub fn get_activity_claims(player_id: i32) -> HashSet<String> {
    let mut set = HashSet::new();
    let _ = with_connection(|db| {
        if let Ok(mut stmt) = db
            .connection()
            .prepare_cached("SELECT claim_key FROM player_activity_claims WHERE player_id = ?1;")
            && let Ok(rows) = stmt.query_map([player_id], |r| r.get::<_, String>(0))
        {
            for key in rows.flatten() {
                set.insert(key);
            }
        }
    });
    set
}

pub fn get_kv(player_id: i32, ns: &str, key: &str) -> Option<String> {
    with_connection(|db| {
        db.connection()
            .prepare_cached("SELECT val FROM player_kv WHERE player_id = ?1 AND ns = ?2 AND k = ?3;")
            .ok()
            .and_then(|mut stmt| {
                stmt.query_row(params![player_id, ns, key], |r| r.get(0))
                    .optional()
                    .ok()
                    .flatten()
            })
    })
    .ok()
    .flatten()
}

pub fn set_kv(player_id: i32, ns: &str, key: &str, value: &str) {
    let _ = with_connection(|db| {
        if let Ok(mut stmt) = db.connection().prepare_cached(
            "INSERT INTO player_kv(player_id, ns, k, val, updated_at)
             VALUES(?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(player_id, ns, k) DO UPDATE SET
                 val = excluded.val,
                 updated_at = excluded.updated_at;",
        ) {
            let _ = stmt.execute(params![player_id, ns, key, value, unix_now()]);
        }
    });
}

pub fn get_finished_bp_tasks(player_id: i32) -> HashSet<i32> {
    if player_id <= 0 {
        return HashSet::new();
    }
    let Some(raw) = get_kv(player_id, "bp", "finished_tasks") else {
        return HashSet::new();
    };
    serde_json::from_str::<Vec<i32>>(&raw)
        .map(|v| v.into_iter().collect())
        .unwrap_or_default()
}

pub fn get_progress_state(player_id: i32) -> (HashMap<String, i64>, HashSet<String>) {
    let mut ints = HashMap::new();
    let mut claims = HashSet::new();
    if player_id <= 0 {
        return (ints, claims);
    }

    let content: Option<String> = with_connection(|db| {
        db.connection()
            .prepare_cached("SELECT content FROM state_files WHERE path = ?1;")
            .ok()
            .and_then(|mut stmt| {
                stmt.query_row([format!("player/{player_id}/progress_info")], |r| r.get(0))
                    .optional()
                    .ok()
                    .flatten()
            })
    })
    .ok()
    .flatten();

    if let Some(text) = content
        && let Ok(val) = zon::parse(&text)
    {
        collect_progress_from_zon(&val, &mut ints, &mut claims);
    }

    (ints, claims)
}

fn collect_progress_from_zon(
    val: &ZonValue,
    ints: &mut HashMap<String, i64>,
    claims: &mut HashSet<String>,
) {
    match val {
        ZonValue::Struct(fields) => {
            if let (Some(k), Some(v)) = (
                val.get("key").and_then(ZonValue::as_str),
                val.get("val").and_then(ZonValue::as_i64),
            ) {
                ints.insert(k.to_string(), v);
            }
            for (name, child) in fields {
                if name == "claims"
                    && let ZonValue::Array(items) = child
                {
                    for item in items {
                        if let Some(s) = item.as_str() {
                            claims.insert(s.to_string());
                        }
                    }
                } else {
                    collect_progress_from_zon(child, ints, claims);
                }
            }
        }
        ZonValue::Array(items) => {
            for item in items {
                collect_progress_from_zon(item, ints, claims);
            }
        }
        _ => {}
    }
}

pub fn get_database_stats() -> Result<(i64, i64), SqliteError> {
    with_connection(|db| {
        let p_count: i64 = db
            .connection()
            .query_row("SELECT COUNT(*) FROM players;", [], |r| r.get(0))?;
        let f_count: i64 = db
            .connection()
            .query_row("SELECT COUNT(*) FROM state_files;", [], |r| r.get(0))?;
        Ok((p_count, f_count))
    })?
}
