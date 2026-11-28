use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex};

use rusqlite::{Connection, OpenFlags, OptionalExtension, params};
use serde::de::DeserializeOwned;
use sonoro_common::sqlite::SqliteError;

use crate::tables::ai_state_machine_config::AiStateMachineConfig;
use crate::tables::base_property::BaseProperty;
use crate::tables::buff::Buff;
use crate::tables::condition::Condition;
use crate::tables::condition_group::ConditionGroup;
use crate::tables::drop_group::DropGroup;
use crate::tables::drop_package::DropPackage;
use crate::tables::flow::Flow;
use crate::tables::level_entity_config::LevelEntityConfig;
use crate::tables::model_config_preload::ModelConfigPreload;
use crate::tables::passive_skill::PassiveSkill;
use crate::tables::template_config::TemplateConfig;
use crate::tables::trail_phantom_prop::TrailPhantomProp;

pub const DEFAULT_ASSETS_DB_PATH: &str = "assets/assets.db";

#[derive(Debug, Clone, Copy)]
pub struct SunSpiritRow {
    pub entity_id: i64,
    pub map_id: i32,
    pub area_id: i32,
}

#[derive(Debug, Clone)]
pub struct LevelEntityIndexRow {
    pub idx: u32,
    pub entity_id: i64,
    pub map_id: i32,
    pub cell_x: i32,
    pub cell_y: i32,
    pub spawnable: bool,
    pub area_id: i32,
    pub in_sleep: bool,
    pub is_hidden: bool,
    pub blueprint_type: String,
    pub is_client_flag: bool,
}

/// 4-bucket rotating cache storing `Arc<V>` values so evicted handles stay valid for existing callers.
pub struct RingCache<K: Eq + Hash + Clone, V> {
    bucket_cap: usize,
    buckets: [Vec<K>; 4],
    cur_bucket: usize,
    map: HashMap<K, Arc<V>>,
}

impl<K: Eq + Hash + Clone, V> RingCache<K, V> {
    pub fn new(bucket_cap: usize) -> Self {
        Self {
            bucket_cap: bucket_cap.max(1),
            buckets: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
            cur_bucket: 0,
            map: HashMap::new(),
        }
    }

    pub fn get(&self, key: &K) -> Option<Arc<V>> {
        self.map.get(key).cloned()
    }

    pub fn get_borrowed<Q>(&self, key: &Q) -> Option<Arc<V>>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.map.get(key).cloned()
    }

    pub fn insert(&mut self, key: K, val: V) -> Arc<V> {
        let arc = Arc::new(val);
        if self.buckets[self.cur_bucket].len() >= self.bucket_cap {
            self.cur_bucket = (self.cur_bucket + 1) & 3;
            let evicted = std::mem::take(&mut self.buckets[self.cur_bucket]);
            for old_key in evicted {
                self.map.remove(&old_key);
            }
        }
        self.buckets[self.cur_bucket].push(key.clone());
        self.map.insert(key, Arc::clone(&arc));
        arc
    }
}

struct DbInner {
    conn: Connection,
    buff_cache: RingCache<i64, Buff>,
    le_idx_cache: RingCache<u32, LevelEntityConfig>,
    tpl_id_cache: RingCache<i32, TemplateConfig>,
    tpl_bp_cache: RingCache<String, TemplateConfig>,
    damage_cache: RingCache<i64, crate::tables::damage::Damage>,
    base_prop_cache: RingCache<i32, BaseProperty>,
    trail_phantom_cache: RingCache<i32, TrailPhantomProp>,
    model_preload_cache: RingCache<i32, ModelConfigPreload>,
    cond_cache: RingCache<i32, Condition>,
    cond_grp_cache: RingCache<i32, ConditionGroup>,
    passive_cache: RingCache<i64, PassiveSkill>,
    drop_pkg_cache: RingCache<i32, DropPackage>,
    drop_grp_cache: RingCache<i32, DropGroup>,
    flow_cache: RingCache<String, Flow>,
    ai_fsm_cache: RingCache<String, AiStateMachineConfig>,
    miss_int: HashMap<(&'static str, i64), ()>,
    miss_str: HashMap<&'static str, HashSet<String>>,
}

pub struct AssetsDb {
    inner: Mutex<DbInner>,
}

impl AssetsDb {
    /// Returns `true` if `db_path` exists and matches a sibling `assets.version.json` fast stamp.
    fn is_prebuilt_version_fresh(db_path: &Path) -> bool {
        let Ok(meta) = std::fs::metadata(db_path) else {
            return false;
        };
        if !meta.is_file() || meta.len() < 4096 {
            return false;
        }
        let version_path = db_path.with_file_name("assets.version.json");
        let Ok(raw) = std::fs::read_to_string(version_path) else {
            return false;
        };
        let Ok(v) = serde_json::from_str::<serde_json::Value>(&raw) else {
            return false;
        };
        if v.get("size").and_then(|s| s.as_u64()) != Some(meta.len()) {
            return false;
        }
        let Some(expected_mtime_ns) = v.get("mtime_ns").and_then(|m| m.as_u64()) else {
            return false;
        };
        let Ok(modified) = meta.modified() else {
            return false;
        };
        let Ok(dur) = modified.duration_since(std::time::UNIX_EPOCH) else {
            return false;
        };
        dur.as_nanos() == u128::from(expected_mtime_ns)
    }

    pub fn ensure_built(db_path: &Path) -> Result<(), String> {
        if Self::is_prebuilt_version_fresh(db_path) {
            return Ok(());
        }
        let script = Path::new("scripts/build_assets_db.py");
        if script.exists() {
            let status = Command::new("python3")
                .args([script.to_string_lossy().as_ref(), "--if-stale"])
                .status();
            if let Ok(st) = status
                && !st.success()
                && !db_path.exists()
            {
                return Err(format!("build_assets_db.py exited with {st}"));
            }
        }
        if db_path.exists() {
            Ok(())
        } else {
            Err(format!("missing assets database at {}", db_path.display()))
        }
    }

    pub fn open_default() -> Result<Arc<Self>, SqliteError> {
        let db_path = Path::new(DEFAULT_ASSETS_DB_PATH);
        let _ = Self::ensure_built(db_path);
        Self::open(db_path)
    }

    pub fn open(db_path: impl AsRef<Path>) -> Result<Arc<Self>, SqliteError> {
        let conn = Connection::open_with_flags(db_path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
        conn.execute_batch(
            "PRAGMA cache_size=-65536; PRAGMA temp_store=MEMORY; PRAGMA mmap_size=268435456;",
        )?;
        Ok(Arc::new(Self {
            inner: Mutex::new(DbInner {
                conn,
                buff_cache: RingCache::new(4096),
                le_idx_cache: RingCache::new(16384),
                tpl_id_cache: RingCache::new(2048),
                tpl_bp_cache: RingCache::new(2048),
                damage_cache: RingCache::new(4096),
                base_prop_cache: RingCache::new(1024),
                trail_phantom_cache: RingCache::new(512),
                model_preload_cache: RingCache::new(1024),
                cond_cache: RingCache::new(4096),
                cond_grp_cache: RingCache::new(2048),
                passive_cache: RingCache::new(2048),
                drop_pkg_cache: RingCache::new(1024),
                drop_grp_cache: RingCache::new(1024),
                flow_cache: RingCache::new(1024),
                ai_fsm_cache: RingCache::new(512),
                miss_int: HashMap::new(),
                miss_str: HashMap::new(),
            }),
        }))
    }

    pub fn get_level_entity_by_idx(&self, idx: u32) -> Option<Arc<LevelEntityConfig>> {
        let mut guard = self.inner.lock().ok()?;
        if let Some(hit) = guard.le_idx_cache.get(&idx) {
            return Some(hit);
        }
        if guard.miss_int.contains_key(&("le_idx", i64::from(idx))) {
            return None;
        }
        let json: Option<String> = guard
            .conn
            .prepare_cached("SELECT data FROM level_entities WHERE idx = ?1")
            .ok()?
            .query_row(params![idx], |row| row.get(0))
            .optional()
            .ok()?;
        let Some(json) = json else {
            guard.miss_int.insert(("le_idx", i64::from(idx)), ());
            return None;
        };
        let parsed: LevelEntityConfig = serde_json::from_str(&json).ok()?;
        Some(guard.le_idx_cache.insert(idx, parsed))
    }

    pub fn get_buff(&self, id: i64) -> Option<Arc<Buff>> {
        let mut guard = self.inner.lock().ok()?;
        if let Some(hit) = guard.buff_cache.get(&id) {
            return Some(hit);
        }
        if guard.miss_int.contains_key(&("Buff", id)) {
            return None;
        }
        let json: Option<String> = guard
            .conn
            .prepare_cached("SELECT data FROM buffs WHERE id = ?1")
            .ok()?
            .query_row(params![id], |row| row.get(0))
            .optional()
            .ok()?;
        let Some(json) = json else {
            guard.miss_int.insert(("Buff", id), ());
            return None;
        };
        let parsed: Buff = serde_json::from_str(&json).ok()?;
        Some(guard.buff_cache.insert(id, parsed))
    }

    pub fn get_auto_buff_candidates(&self, role_id: i32) -> Result<Vec<Buff>, SqliteError> {
        let prefix = role_id;
        let guard = self
            .inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut stmt = guard.conn.prepare_cached(
            "SELECT data FROM buffs WHERE id_prefix = ?1 AND is_infinite = 1 ORDER BY id",
        )?;
        let mut rows = stmt.query(params![prefix])?;
        let mut out = Vec::new();
        while let Some(row) = rows.next()? {
            let json: String = row.get(0)?;
            if let Ok(b) = serde_json::from_str::<Buff>(&json) {
                out.push(b);
            }
        }
        Ok(out)
    }

    pub fn get_template_by_id(&self, id: i32) -> Option<Arc<TemplateConfig>> {
        let mut guard = self.inner.lock().ok()?;
        if let Some(hit) = guard.tpl_id_cache.get(&id) {
            return Some(hit);
        }
        if guard.miss_int.contains_key(&("TemplateId", i64::from(id))) {
            return None;
        }
        let json: Option<String> = guard
            .conn
            .prepare_cached("SELECT data FROM template_configs WHERE id = ?1")
            .ok()?
            .query_row(params![id], |row| row.get(0))
            .optional()
            .ok()?;
        let Some(json) = json else {
            guard.miss_int.insert(("TemplateId", i64::from(id)), ());
            return None;
        };
        let parsed: TemplateConfig = serde_json::from_str(&json).ok()?;
        Some(guard.tpl_id_cache.insert(id, parsed))
    }

    pub fn find_template_by_blueprint(&self, bp: &str) -> Option<Arc<TemplateConfig>> {
        let mut guard = self.inner.lock().ok()?;
        if let Some(hit) = guard.tpl_bp_cache.get_borrowed(bp) {
            return Some(hit);
        }
        if guard
            .miss_str
            .get("TemplateBp")
            .is_some_and(|set| set.contains(bp))
        {
            return None;
        }
        let json: Option<String> = guard
            .conn
            .prepare_cached("SELECT data FROM template_configs WHERE blueprint_type = ?1 LIMIT 1")
            .ok()?
            .query_row(params![bp], |row| row.get(0))
            .optional()
            .ok()?;
        let Some(json) = json else {
            guard
                .miss_str
                .entry("TemplateBp")
                .or_default()
                .insert(bp.to_string());
            return None;
        };
        let parsed: TemplateConfig = serde_json::from_str(&json).ok()?;
        Some(guard.tpl_bp_cache.insert(bp.to_string(), parsed))
    }

    pub fn query_passenger_blueprint_types(&self) -> Result<Vec<String>, SqliteError> {
        let guard = self
            .inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut stmt = guard
            .conn
            .prepare_cached("SELECT blueprint_type FROM template_configs WHERE is_passenger = 1")?;
        let mut rows = stmt.query([])?;
        let mut out = Vec::new();
        while let Some(row) = rows.next()? {
            out.push(row.get(0)?);
        }
        Ok(out)
    }

    pub fn query_skill_ids_by_actor_blueprint(
        &self,
        bp: &str,
    ) -> Result<Vec<i32>, SqliteError> {
        let guard = self
            .inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut stmt = guard.conn.prepare_cached(
            "SELECT skill_id FROM entity_skill_preload WHERE actor_blueprint = ?1 ORDER BY idx",
        )?;
        let mut rows = stmt.query(params![bp])?;
        let mut out = Vec::new();
        while let Some(row) = rows.next()? {
            let sid: i32 = row.get(0)?;
            if sid > 0 {
                out.push(sid);
            }
        }
        Ok(out)
    }

    pub fn query_skill_ids_by_role_prefix(&self, role_id: i32) -> Result<Vec<i32>, SqliteError> {
        let guard = self
            .inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut stmt = guard.conn.prepare_cached(
            "SELECT skill_id FROM entity_skill_preload WHERE skill_prefix = ?1 ORDER BY idx",
        )?;
        let mut rows = stmt.query(params![role_id])?;
        let mut out = Vec::new();
        while let Some(row) = rows.next()? {
            let sid: i32 = row.get(0)?;
            if sid > 0 {
                out.push(sid);
            }
        }
        Ok(out)
    }

    pub fn query_entity_ids_by_area_explore(
        &self,
        area_id: i32,
        exp_degree: i32,
    ) -> Result<Vec<i32>, SqliteError> {
        let guard = self
            .inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut out = Vec::new();
        if area_id != 0 {
            let mut stmt = guard.conn.prepare_cached(
                "SELECT entity_id FROM level_entities WHERE area_id = ?1 AND exploratory_degree = ?2 LIMIT 32",
            )?;
            let mut rows = stmt.query(params![area_id, exp_degree])?;
            while let Some(row) = rows.next()? {
                let eid: i64 = row.get(0)?;
                if eid > 0 && eid <= i64::from(i32::MAX) {
                    out.push(eid as i32);
                }
            }
        } else {
            let mut stmt = guard.conn.prepare_cached(
                "SELECT entity_id FROM level_entities WHERE exploratory_degree = ?1 LIMIT 32",
            )?;
            let mut rows = stmt.query(params![exp_degree])?;
            while let Some(row) = rows.next()? {
                let eid: i64 = row.get(0)?;
                if eid > 0 && eid <= i64::from(i32::MAX) {
                    out.push(eid as i32);
                }
            }
        }
        Ok(out)
    }

    pub fn query_entity_ids_by_map_blueprint(
        &self,
        map_id: i32,
        bp: &str,
    ) -> Result<Vec<i32>, SqliteError> {
        let guard = self
            .inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut stmt = guard.conn.prepare_cached(
            "SELECT entity_id FROM level_entities WHERE map_id = ?1 AND blueprint_type = ?2 LIMIT 32",
        )?;
        let mut rows = stmt.query(params![map_id, bp])?;
        let mut out = Vec::new();
        while let Some(row) = rows.next()? {
            let eid: i64 = row.get(0)?;
            if eid > 0 && eid <= i64::from(i32::MAX) {
                out.push(eid as i32);
            }
        }
        Ok(out)
    }

    pub fn query_sun_spirit_entities(&self) -> Result<Vec<SunSpiritRow>, SqliteError> {
        let guard = self
            .inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut stmt = guard.conn.prepare_cached(
            "SELECT entity_id, map_id, area_id FROM level_entities WHERE is_sun_spirit = 1",
        )?;
        let mut rows = stmt.query([])?;
        let mut out = Vec::new();
        while let Some(row) = rows.next()? {
            out.push(SunSpiritRow {
                entity_id: row.get(0)?,
                map_id: row.get(1)?,
                area_id: row.get(2)?,
            });
        }
        Ok(out)
    }

    pub fn query_world_controller_entities(
        &self,
        map_id: i32,
    ) -> Result<Vec<Arc<LevelEntityConfig>>, SqliteError> {
        let guard = self
            .inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut tpl_stmt = guard.conn.prepare_cached(
            "SELECT blueprint_type, data FROM template_configs",
        )?;
        let mut tpl_rows = tpl_stmt.query([])?;
        let mut tpl_bps = std::collections::HashSet::new();
        while let Some(row) = tpl_rows.next()? {
            let bp: String = row.get(0)?;
            let data: String = row.get(1)?;
            if !bp.is_empty()
                && (data.contains("RefreshGroupComponent")
                    || data.contains("EntityBatchRefreshComponent")
                    || data.contains("DynamicSpawnMonsterComponent")
                    || data.contains("DynamicSpawnMonsterPointComponent")
                    || data.contains("TimeScheduleComponent"))
            {
                tpl_bps.insert(bp);
            }
        }
        drop(tpl_rows);

        let mut stmt = guard.conn.prepare_cached(
            "SELECT blueprint_type, data FROM level_entities WHERE map_id = ?1",
        )?;
        let mut rows = stmt.query(params![map_id])?;
        let mut out = Vec::new();
        while let Some(row) = rows.next()? {
            let bp: String = row.get(0)?;
            let raw: String = row.get(1)?;
            let matched = tpl_bps.contains(&bp)
                || raw.contains("\"RefreshGroupComponent\"")
                || raw.contains("\"EntityBatchRefreshComponent\"")
                || raw.contains("\"DynamicSpawnMonsterComponent\"")
                || raw.contains("\"DynamicSpawnMonsterPointComponent\"")
                || raw.contains("\"TimeScheduleComponent\"");
            if matched
                && let Ok(parsed) = serde_json::from_str::<LevelEntityConfig>(&raw)
            {
                out.push(Arc::new(parsed));
            }
        }
        Ok(out)
    }

    pub fn query_level_entity_index_rows(&self) -> Result<Vec<LevelEntityIndexRow>, SqliteError> {
        let guard = self
            .inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut stmt = guard.conn.prepare_cached(
            "SELECT idx, entity_id, map_id, COALESCE(cell_x, 0), COALESCE(cell_y, 0), spawnable, area_id, in_sleep, is_hidden, blueprint_type, COALESCE(json_extract(data, '$.ComponentsData.BaseInfoComponent.IsClientEntityFlag'), 0) FROM level_entities ORDER BY idx",
        )?;
        let mut rows = stmt.query([])?;
        let mut out = Vec::with_capacity(220_000);
        while let Some(row) = rows.next()? {
            out.push(LevelEntityIndexRow {
                idx: row.get::<_, u32>(0)?,
                entity_id: row.get(1)?,
                map_id: row.get(2)?,
                cell_x: row.get(3)?,
                cell_y: row.get(4)?,
                spawnable: row.get::<_, i32>(5)? != 0,
                area_id: row.get(6)?,
                in_sleep: row.get::<_, i32>(7)? != 0,
                is_hidden: row.get::<_, i32>(8)? != 0,
                blueprint_type: row.get::<_, String>(9)?,
                is_client_flag: row.get::<_, i32>(10)? != 0,
            });
        }
        Ok(out)
    }

    fn query_kv_int<T: DeserializeOwned>(
        cache: &mut RingCache<i32, T>,
        conn: &Connection,
        miss_int: &mut HashMap<(&'static str, i64), ()>,
        tbl: &'static str,
        key: i32,
    ) -> Option<Arc<T>> {
        if let Some(hit) = cache.get(&key) {
            return Some(hit);
        }
        if miss_int.contains_key(&(tbl, i64::from(key))) {
            return None;
        }
        let json: Option<String> = conn
            .prepare_cached("SELECT data FROM kv_int WHERE tbl = ?1 AND key_id = ?2")
            .ok()?
            .query_row(params![tbl, key], |row| row.get(0))
            .optional()
            .ok()?;
        let Some(json) = json else {
            miss_int.insert((tbl, i64::from(key)), ());
            return None;
        };
        let parsed: T = serde_json::from_str(&json).ok()?;
        Some(cache.insert(key, parsed))
    }

    fn query_kv_i64<T: DeserializeOwned>(
        cache: &mut RingCache<i64, T>,
        conn: &Connection,
        miss_int: &mut HashMap<(&'static str, i64), ()>,
        tbl: &'static str,
        key: i64,
    ) -> Option<Arc<T>> {
        if let Some(hit) = cache.get(&key) {
            return Some(hit);
        }
        if miss_int.contains_key(&(tbl, key)) {
            return None;
        }
        let json: Option<String> = conn
            .prepare_cached("SELECT data FROM kv_int WHERE tbl = ?1 AND key_id = ?2")
            .ok()?
            .query_row(params![tbl, key], |row| row.get(0))
            .optional()
            .ok()?;
        let Some(json) = json else {
            miss_int.insert((tbl, key), ());
            return None;
        };
        let parsed: T = serde_json::from_str(&json).ok()?;
        Some(cache.insert(key, parsed))
    }

    fn query_kv_str<T: DeserializeOwned>(
        cache: &mut RingCache<String, T>,
        conn: &Connection,
        miss_str: &mut HashMap<&'static str, HashSet<String>>,
        tbl: &'static str,
        key: &str,
    ) -> Option<Arc<T>> {
        if let Some(hit) = cache.get_borrowed(key) {
            return Some(hit);
        }
        if miss_str.get(tbl).is_some_and(|set| set.contains(key)) {
            return None;
        }
        let json: Option<String> = conn
            .prepare_cached("SELECT data FROM kv_str WHERE tbl = ?1 AND key_str = ?2")
            .ok()?
            .query_row(params![tbl, key], |row| row.get(0))
            .optional()
            .ok()?;
        let Some(json) = json else {
            miss_str
                .entry(tbl)
                .or_default()
                .insert(key.to_string());
            return None;
        };
        let parsed: T = serde_json::from_str(&json).ok()?;
        Some(cache.insert(key.to_string(), parsed))
    }

    pub fn get_damage(&self, id: i64) -> Option<Arc<crate::tables::damage::Damage>> {
        let mut g = self.inner.lock().ok()?;
        let g = &mut *g;
        Self::query_kv_i64(&mut g.damage_cache, &g.conn, &mut g.miss_int, "Damage", id)
    }

    pub fn get_base_property(&self, id: i32) -> Option<Arc<BaseProperty>> {
        let mut g = self.inner.lock().ok()?;
        let g = &mut *g;
        Self::query_kv_int(
            &mut g.base_prop_cache,
            &g.conn,
            &mut g.miss_int,
            "BaseProperty",
            id,
        )
    }

    pub fn get_trail_phantom_prop(&self, id: i32) -> Option<Arc<TrailPhantomProp>> {
        let mut g = self.inner.lock().ok()?;
        let g = &mut *g;
        Self::query_kv_int(
            &mut g.trail_phantom_cache,
            &g.conn,
            &mut g.miss_int,
            "TrailPhantomProp",
            id,
        )
    }

    pub fn get_model_config_preload(&self, id: i32) -> Option<Arc<ModelConfigPreload>> {
        let mut g = self.inner.lock().ok()?;
        let g = &mut *g;
        Self::query_kv_int(
            &mut g.model_preload_cache,
            &g.conn,
            &mut g.miss_int,
            "ModelConfigPreload",
            id,
        )
    }

    pub fn get_condition(&self, id: i32) -> Option<Arc<Condition>> {
        let mut g = self.inner.lock().ok()?;
        let g = &mut *g;
        Self::query_kv_int(
            &mut g.cond_cache,
            &g.conn,
            &mut g.miss_int,
            "Condition",
            id,
        )
    }

    pub fn get_condition_group(&self, id: i32) -> Option<Arc<ConditionGroup>> {
        let mut g = self.inner.lock().ok()?;
        let g = &mut *g;
        Self::query_kv_int(
            &mut g.cond_grp_cache,
            &g.conn,
            &mut g.miss_int,
            "ConditionGroup",
            id,
        )
    }

    pub fn get_passive_skill(&self, id: i64) -> Option<Arc<PassiveSkill>> {
        let mut g = self.inner.lock().ok()?;
        let g = &mut *g;
        Self::query_kv_i64(
            &mut g.passive_cache,
            &g.conn,
            &mut g.miss_int,
            "PassiveSkill",
            id,
        )
    }

    pub fn get_drop_package(&self, id: i32) -> Option<Arc<DropPackage>> {
        let mut g = self.inner.lock().ok()?;
        let g = &mut *g;
        Self::query_kv_int(
            &mut g.drop_pkg_cache,
            &g.conn,
            &mut g.miss_int,
            "DropPackage",
            id,
        )
    }

    pub fn get_drop_group(&self, id: i32) -> Option<Arc<DropGroup>> {
        let mut g = self.inner.lock().ok()?;
        let g = &mut *g;
        Self::query_kv_int(
            &mut g.drop_grp_cache,
            &g.conn,
            &mut g.miss_int,
            "DropGroup",
            id,
        )
    }

    pub fn get_flow(&self, id: &str) -> Option<Arc<Flow>> {
        let mut g = self.inner.lock().ok()?;
        let g = &mut *g;
        Self::query_kv_str(&mut g.flow_cache, &g.conn, &mut g.miss_str, "Flow", id)
    }

    pub fn get_ai_state_machine_config(&self, id: &str) -> Option<Arc<AiStateMachineConfig>> {
        let mut g = self.inner.lock().ok()?;
        let g = &mut *g;
        Self::query_kv_str(
            &mut g.ai_fsm_cache,
            &g.conn,
            &mut g.miss_str,
            "AiStateMachineConfig",
            id,
        )
    }

    pub fn read_raw_file(&self, name: &str) -> Option<String> {
        let guard = self.inner.lock().ok()?;
        guard
            .conn
            .prepare_cached("SELECT data FROM raw_files WHERE name = ?1")
            .ok()?
            .query_row(params![name], |row| row.get(0))
            .optional()
            .ok()?
    }

    pub fn query_node_tree_owners(&self, kind: i32) -> Result<Vec<i32>, SqliteError> {
        let guard = self
            .inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut stmt = guard
            .conn
            .prepare_cached("SELECT DISTINCT owner_id FROM node_trees WHERE kind = ?1")?;
        let mut rows = stmt.query(params![kind])?;
        let mut out = Vec::new();
        while let Some(row) = rows.next()? {
            out.push(row.get(0)?);
        }
        Ok(out)
    }

    pub fn has_node_tree_owner(&self, kind: i32, owner_id: i32) -> bool {
        let Ok(guard) = self.inner.lock() else {
            return false;
        };
        let Ok(mut stmt) = guard
            .conn
            .prepare_cached("SELECT 1 FROM node_trees WHERE kind = ?1 AND owner_id = ?2 LIMIT 1")
        else {
            return false;
        };
        stmt.query_row(params![kind, owner_id], |_| Ok(()))
            .optional()
            .ok()
            .flatten()
            .is_some()
    }

    pub fn query_node_tree_data_slices(
        &self,
        kind: i32,
        owner_id: i32,
    ) -> Result<Vec<String>, SqliteError> {
        let guard = self
            .inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut stmt = guard.conn.prepare_cached(
            "SELECT data FROM node_trees WHERE kind = ?1 AND owner_id = ?2 ORDER BY seq",
        )?;
        let mut rows = stmt.query(params![kind, owner_id])?;
        let mut out = Vec::new();
        while let Some(row) = rows.next()? {
            out.push(row.get(0)?);
        }
        Ok(out)
    }

    pub fn query_flow_state_json(&self, state_key: &str) -> Result<Option<String>, SqliteError> {
        let guard = self
            .inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut stmt = guard
            .conn
            .prepare_cached("SELECT data FROM kv_str WHERE tbl = 'FlowState' AND key_str = ?1")?;
        stmt.query_row(params![state_key], |row| row.get(0))
            .optional()
            .map_err(SqliteError::from)
    }

    pub fn query_entity_group_rows(&self) -> Result<Vec<(i32, i64, String)>, SqliteError> {
        let guard = self
            .inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut stmt = guard.conn.prepare_cached(
            "SELECT map_id, entity_id, data FROM level_entities WHERE data LIKE '%EntityGroupComponent%'",
        )?;
        let mut rows = stmt.query([])?;
        let mut out = Vec::new();
        while let Some(row) = rows.next()? {
            out.push((row.get(0)?, row.get(1)?, row.get(2)?));
        }
        Ok(out)
    }

    pub fn query_passerby_spawner_rows(&self) -> Result<Vec<(i32, u32)>, SqliteError> {
        let guard = self
            .inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut stmt = guard.conn.prepare_cached(
            "SELECT map_id, idx FROM level_entities WHERE data LIKE '%PasserbyNpcSpawnComponent%'",
        )?;
        let mut rows = stmt.query([])?;
        let mut out = Vec::new();
        while let Some(row) = rows.next()? {
            out.push((row.get(0)?, row.get(1)?));
        }
        Ok(out)
    }

    /// Collects all FunctionIds unlocked by `OpenSystemFunction` node actions.
    pub fn query_action_opened_function_ids(&self) -> Result<Vec<i32>, SqliteError> {
        fn collect(val: &serde_json::Value, out: &mut HashSet<i32>) {
            match val {
                serde_json::Value::Object(map) => {
                    if map.get("Name").and_then(|v| v.as_str()) == Some("OpenSystemFunction")
                        && let Some(fid) = map
                            .get("Params")
                            .and_then(|p| p.get("FunctionId"))
                            .and_then(|v| v.as_i64())
                            .and_then(|v| i32::try_from(v).ok())
                        && fid > 0
                    {
                        out.insert(fid);
                    }
                    for v in map.values() {
                        collect(v, out);
                    }
                }
                serde_json::Value::Array(arr) => {
                    for v in arr {
                        collect(v, out);
                    }
                }
                _ => {}
            }
        }

        let guard = self
            .inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut stmt = guard.conn.prepare_cached(
            "SELECT data FROM node_trees WHERE data LIKE '%OpenSystemFunction%'",
        )?;
        let mut rows = stmt.query([])?;
        let mut ids = HashSet::new();
        while let Some(row) = rows.next()? {
            let raw: String = row.get(0)?;
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&raw) {
                collect(&val, &mut ids);
            }
        }
        Ok(ids.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring_cache_rotates_4_buckets_while_preserving_recent_handles() {
        #[derive(Debug)]
        struct Val {
            a: i32,
        }
        let mut cache = RingCache::<i32, Val>::new(2);
        let p1 = cache.insert(1, Val { a: 10 });
        let p2 = cache.insert(2, Val { a: 20 });
        assert_eq!(p1.a, 10);
        assert_eq!(p2.a, 20);

        for i in 3..10 {
            cache.insert(i, Val { a: i * 10 });
            if i == 8 {
                assert_eq!(cache.get(&1).unwrap().a, 10);
            }
        }
        assert!(cache.get(&1).is_none());
        assert_eq!(cache.get(&9).unwrap().a, 90);
    }

    #[test]
    fn assets_db_queries_real_db_when_built() {
        let db_path = Path::new("/root/wuwa-server/assets/assets.db");
        if !db_path.exists() {
            return;
        }
        let db = AssetsDb::open(db_path).unwrap();
        let first = db.get_level_entity_by_idx(0).expect("first entity");
        let second = db.get_level_entity_by_idx(0).expect("cached entity");
        assert!(Arc::ptr_eq(&first, &second));

        let suns = db.query_sun_spirit_entities().unwrap();
        assert!(!suns.is_empty());
    }
}
