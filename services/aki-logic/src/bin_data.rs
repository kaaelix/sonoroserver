//! Cached JSON loader for `assets/BinData/*.json` matching `services/aki-logic/BinDataCache.cs`.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use serde_json::Value;

#[derive(Debug, Clone)]
pub struct BpRewardEntry {
    pub free_reward: HashMap<i32, i32>,
    pub pay_reward: HashMap<i32, i32>,
}

#[derive(Debug, Clone)]
pub struct BpTaskCfg {
    pub task_id: i32,
    pub update_type: i32,
    pub task_reward: HashMap<i32, i32>,
}

#[derive(Debug, Clone)]
pub struct BpMeta {
    pub latest_bp_id: i32,
    pub latest_reward_id: i32,
}

#[derive(Debug, Clone)]
pub struct LivenessBox {
    pub id: i32,
    pub goal: i32,
    pub drop_id: i32,
}

#[derive(Debug, Clone)]
pub struct LivenessTaskEntry {
    pub task_id: i32,
    pub task_reward: HashMap<i32, i32>,
}

#[derive(Debug, Clone)]
pub struct AchievementCfg {
    pub id: i32,
    pub group_id: i32,
    pub level: i32,
    pub override_drop_id: i32,
}

#[derive(Debug, Clone)]
pub struct AchievementGroupCfg {
    pub id: i32,
    pub enable: bool,
    pub drop_id: i32,
}

#[derive(Debug, Clone)]
pub struct PayItemDef {
    pub id: i32,
    pub pay_id: i32,
    pub item_id: i32,
    pub item_count: i32,
    pub bonus_item_count: i32,
    pub special_bonus_item_count: i32,
    pub stage_image: String,
    pub product_id: String,
    pub amount: String,
}

#[derive(Debug, Clone)]
pub struct TrapDefenseRewardCfg {
    pub id: i32,
    pub activity_id: i32,
}

#[derive(Debug, Clone)]
pub struct ActivityConfigItem {
    pub id: i32,
    pub activity_type: i32,
    pub tab_texture: Vec<String>,
}

pub struct BinDataCache {
    pub drop_packages: HashMap<i32, Vec<(i32, i32)>>,
    pub bp_rewards: HashMap<(i32, i32), BpRewardEntry>,
    pub bp_meta: BpMeta,
    pub bp_tasks: BTreeMap<i32, BpTaskCfg>,
    pub adventure_task_drops: HashMap<i32, i32>,
    pub pay_gift_items: BTreeMap<i32, i32>,
    pub pay_items: Vec<PayItemDef>,
    pub access_path_ids: Vec<i32>,
    pub liveness_boxes: Vec<LivenessBox>,
    pub liveness_tasks: Vec<LivenessTaskEntry>,
    pub achievements: Vec<AchievementCfg>,
    pub achievement_groups: Vec<AchievementGroupCfg>,
    pub player_title_ids: Vec<i32>,
    pub monster_detection_ids: Vec<i32>,
    pub dungeon_detection_ids: Vec<i32>,
    pub silent_area_detection_ids: Vec<i32>,
    pub phonograph_albums: BTreeMap<i32, Vec<i32>>,
    pub all_music_ids: HashSet<i32>,
    pub activities: Vec<ActivityConfigItem>,
    pub trap_defense_rewards: Vec<TrapDefenseRewardCfg>,
    pub trap_defense_special_rewards: Vec<TrapDefenseRewardCfg>,
}

static REPO_ROOT: OnceLock<PathBuf> = OnceLock::new();
static CACHE: OnceLock<BinDataCache> = OnceLock::new();

/// Discovers the repository/runtime root containing `assets/BinData` or `Cargo.toml`.
pub fn repo_root() -> &'static Path {
    REPO_ROOT.get_or_init(|| {
        let mut starts = Vec::new();
        if let Ok(exe) = std::env::current_exe()
            && let Some(parent) = exe.parent()
        {
            starts.push(parent.to_path_buf());
        }
        if let Ok(cwd) = std::env::current_dir() {
            starts.push(cwd);
        }

        for start in starts {
            let mut dir_opt = Some(start.as_path());
            while let Some(dir) = dir_opt {
                if dir.join("assets").join("BinData").is_dir()
                    || dir.join("assets").join("assets.db").is_file()
                    || dir.join("Cargo.toml").is_file()
                {
                    return dir.to_path_buf();
                }
                dir_opt = dir.parent();
            }
        }
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    })
}

pub fn bindata_dir() -> PathBuf {
    std::env::var_os("AKI_BINDATA_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo_root().join("assets").join("BinData"))
}

pub fn get() -> &'static BinDataCache {
    CACHE.get_or_init(BinDataCache::load)
}

impl BinDataCache {
    pub fn resolve_drop(&self, drop_id: i32) -> Vec<(i32, i32)> {
        if drop_id > 0
            && let Some(items) = self.drop_packages.get(&drop_id)
            && !items.is_empty()
        {
            return items.clone();
        }
        Vec::new()
    }

    fn load() -> Self {
        let dir = bindata_dir();
        let (phonograph_albums, all_music_ids) = load_music_data(&dir);
        let activities = load_activities(&dir);
        println!(
            "[ActivityHandler] Loaded {} activity definitions from BinData.",
            activities.len()
        );

        Self {
            drop_packages: load_drop_packages(&dir),
            bp_rewards: load_bp_rewards(&dir),
            bp_meta: load_bp_meta(&dir),
            bp_tasks: load_bp_tasks(&dir),
            adventure_task_drops: load_adventure_task_drops(&dir),
            pay_gift_items: load_pay_gift_items(&dir),
            pay_items: load_pay_items(&dir),
            access_path_ids: load_access_path_ids(&dir),
            liveness_boxes: load_liveness_boxes(&dir),
            liveness_tasks: load_liveness_tasks(&dir),
            achievements: load_achievements(&dir),
            achievement_groups: load_achievement_groups(&dir),
            player_title_ids: load_id_list(&dir, "PlayerTitle.json"),
            monster_detection_ids: load_id_list(&dir, "MonsterDetection.json"),
            dungeon_detection_ids: load_id_list(&dir, "DungeonDetection.json"),
            silent_area_detection_ids: load_id_list(&dir, "SilentAreaDetection.json"),
            phonograph_albums,
            all_music_ids,
            activities,
            trap_defense_rewards: load_trap_defense_rewards(&dir, "TrapDefenseReward.json"),
            trap_defense_special_rewards: load_trap_defense_rewards(
                &dir,
                "TrapDefenseSpecialReward.json",
            ),
        }
    }
}

fn load_trap_defense_rewards(dir: &Path, file_name: &str) -> Vec<TrapDefenseRewardCfg> {
    let mut res = Vec::new();
    for el in open_json_array(dir, file_name) {
        let Some(id) = as_i32(&el, "Id") else {
            continue;
        };
        if id <= 0 {
            continue;
        }
        res.push(TrapDefenseRewardCfg {
            id,
            activity_id: as_i32(&el, "ActivityId").unwrap_or(0),
        });
    }
    res
}

fn open_assets_db() -> Option<rusqlite::Connection> {
    let candidates = [
        repo_root().join("assets").join("assets.db"),
        PathBuf::from("assets/assets.db"),
        PathBuf::from("../../assets/assets.db"),
    ];
    for path in candidates {
        if path.is_file()
            && let Ok(conn) = rusqlite::Connection::open_with_flags(
                &path,
                rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
            )
        {
            return Some(conn);
        }
    }
    None
}

static ASSETS_DB: OnceLock<Option<std::sync::Mutex<rusqlite::Connection>>> = OnceLock::new();

fn read_array_from_assets_db(table_name: &str) -> Option<Vec<Value>> {
    let lock = ASSETS_DB.get_or_init(|| open_assets_db().map(std::sync::Mutex::new));
    let conn = lock.as_ref()?.lock().ok()?;
    if let Ok(raw) = conn.query_row(
        "SELECT data FROM raw_files WHERE name = ?1",
        rusqlite::params![table_name],
        |row| row.get::<_, String>(0),
    ) {
        if let Ok(arr) = serde_json::from_str::<Vec<Value>>(&raw) {
            return Some(arr);
        }
    }
    let mut stmt = conn
        .prepare_cached("SELECT data FROM kv_int WHERE tbl = ?1")
        .ok()?;
    let rows = stmt
        .query_map(rusqlite::params![table_name], |row| row.get::<_, String>(0))
        .ok()?;
    let mut out = Vec::new();
    for raw in rows.flatten() {
        if let Ok(val) = serde_json::from_str::<Value>(&raw) {
            out.push(val);
        }
    }
    if out.is_empty() { None } else { Some(out) }
}

fn open_json_array(dir: &Path, file_name: &str) -> Vec<Value> {
    let table_name = file_name.strip_suffix(".json").unwrap_or(file_name);
    if let Some(arr) = read_array_from_assets_db(table_name) {
        return arr;
    }
    let path = dir.join(file_name);
    let Ok(bytes) = fs::read(path) else {
        return Vec::new();
    };
    serde_json::from_slice::<Vec<Value>>(&bytes).unwrap_or_default()
}

fn as_i32(val: &Value, key: &str) -> Option<i32> {
    val.get(key)
        .and_then(Value::as_i64)
        .and_then(|n| i32::try_from(n).ok())
}

fn read_int_map(obj: &Value, prop: &str) -> HashMap<i32, i32> {
    let mut map = HashMap::new();
    let Some(Value::Object(inner)) = obj.get(prop) else {
        return map;
    };
    for (k_str, v_val) in inner {
        if let (Ok(k), Some(v)) = (
            k_str.parse::<i32>(),
            v_val.as_i64().and_then(|n| i32::try_from(n).ok()),
        ) && k > 0
            && v > 0
        {
            map.insert(k, v);
        }
    }
    map
}

fn load_drop_packages(dir: &Path) -> HashMap<i32, Vec<(i32, i32)>> {
    let mut res = HashMap::new();
    for el in open_json_array(dir, "DropPackage.json") {
        let Some(id) = as_i32(&el, "Id") else {
            continue;
        };
        let list: Vec<(i32, i32)> = read_int_map(&el, "DropPreview").into_iter().collect();
        res.insert(id, list);
    }
    res
}

fn load_bp_rewards(dir: &Path) -> HashMap<(i32, i32), BpRewardEntry> {
    let mut res = HashMap::new();
    for el in open_json_array(dir, "BattlePassReward.json") {
        let bp_id = as_i32(&el, "BattlePassId").unwrap_or(0);
        let level = as_i32(&el, "Level").unwrap_or(0);
        if level <= 0 {
            continue;
        }
        res.entry((bp_id, level)).or_insert_with(|| BpRewardEntry {
            free_reward: read_int_map(&el, "FreeReward"),
            pay_reward: read_int_map(&el, "PayReward"),
        });
    }
    res
}

fn load_bp_meta(dir: &Path) -> BpMeta {
    let mut latest_id = 2;
    let mut latest_reward_id = 13;
    for el in open_json_array(dir, "BattlePass.json") {
        if let Some(id) = as_i32(&el, "Id")
            && id > 0
        {
            latest_id = id;
            if let Some(rwid) = as_i32(&el, "BattlePassRewardId") {
                latest_reward_id = rwid;
            }
        }
    }
    BpMeta {
        latest_bp_id: latest_id,
        latest_reward_id,
    }
}

fn load_bp_tasks(dir: &Path) -> BTreeMap<i32, BpTaskCfg> {
    let mut res = BTreeMap::new();
    for el in open_json_array(dir, "BattlePassTask.json") {
        let Some(tid) = as_i32(&el, "TaskId") else {
            continue;
        };
        if tid <= 0 {
            continue;
        }
        let update_type = as_i32(&el, "UpdateType").unwrap_or(0);
        res.insert(
            tid,
            BpTaskCfg {
                task_id: tid,
                update_type,
                task_reward: read_int_map(&el, "TaskReward"),
            },
        );
    }
    res
}

fn load_adventure_task_drops(dir: &Path) -> HashMap<i32, i32> {
    let mut res = HashMap::new();
    for el in open_json_array(dir, "AdventureTask.json") {
        if let Some(id) = as_i32(&el, "Id")
            && id > 0
        {
            res.insert(id, as_i32(&el, "DropIds").unwrap_or(0));
        }
    }
    res
}

fn load_pay_gift_items(dir: &Path) -> BTreeMap<i32, i32> {
    let mut res = BTreeMap::new();
    for el in open_json_array(dir, "PayGift.json") {
        if let Some(id) = as_i32(&el, "Id")
            && id > 0
            && let Some(item_id) = as_i32(&el, "ItemId")
            && item_id > 0
        {
            res.insert(id, item_id);
        }
    }
    res
}

fn format_usd(amount: f64) -> String {
    let rounded = (amount * 100.0).round() / 100.0;
    let s = format!("{rounded:.2}");
    let trimmed = s.trim_end_matches('0').trim_end_matches('.');
    format!("${trimmed}")
}

fn load_pay_items(dir: &Path) -> Vec<PayItemDef> {
    let mut pay_meta: HashMap<i32, (String, String)> = HashMap::new();
    for el in open_json_array(dir, "Pay.json") {
        let Some(pay_id) = as_i32(&el, "PayId") else {
            continue;
        };
        let region = el.get("Region").and_then(Value::as_str).unwrap_or("");
        if region != "Default" && pay_meta.contains_key(&pay_id) {
            continue;
        }
        let prod = el
            .get("ProductId")
            .and_then(Value::as_str)
            .map(str::to_string)
            .unwrap_or_else(|| format!("com.kurogame.aki.pay{pay_id}"));
        let amt = el.get("Amount").and_then(Value::as_f64).unwrap_or(0.99);
        pay_meta.insert(pay_id, (prod, format_usd(amt)));
    }

    let mut res = Vec::new();
    for el in open_json_array(dir, "PayItem.json") {
        let Some(id) = as_i32(&el, "Id") else {
            continue;
        };
        if id <= 0 || id > 6 {
            continue;
        }
        let pay_id = as_i32(&el, "PayId").unwrap_or(id);
        let item_id = as_i32(&el, "ItemId").unwrap_or(4);
        let item_count = as_i32(&el, "ItemCount").unwrap_or(60);
        let bonus = as_i32(&el, "BonusItemCount").unwrap_or(0);
        let special = as_i32(&el, "SpecialBonusItemCount").unwrap_or(0);
        let stage_image = el
            .get("StageImage")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        let (product_id, amount) = pay_meta
            .get(&pay_id)
            .cloned()
            .unwrap_or_else(|| (format!("com.kurogame.aki.pay{pay_id}"), "$0.99".into()));

        res.push(PayItemDef {
            id,
            pay_id,
            item_id,
            item_count,
            bonus_item_count: bonus,
            special_bonus_item_count: special,
            stage_image,
            product_id,
            amount,
        });
    }
    res
}

fn load_access_path_ids(dir: &Path) -> Vec<i32> {
    let mut res = Vec::new();
    for el in open_json_array(dir, "AccessPath.json") {
        if let Some(id) = as_i32(&el, "Id")
            && id > 0
        {
            res.push(id);
            if res.len() >= 128 {
                break;
            }
        }
    }
    res
}

fn load_liveness_boxes(dir: &Path) -> Vec<LivenessBox> {
    let mut res = Vec::new();
    for el in open_json_array(dir, "Liveness.json") {
        let id = as_i32(&el, "Id").unwrap_or(0);
        let goal = as_i32(&el, "Goal").unwrap_or(0);
        let drop_id = as_i32(&el, "DropId").unwrap_or(0);
        if id > 0 {
            res.push(LivenessBox { id, goal, drop_id });
        }
    }
    res
}

fn load_liveness_tasks(dir: &Path) -> Vec<LivenessTaskEntry> {
    let mut res = Vec::new();
    for el in open_json_array(dir, "LivenessTask.json") {
        if let Some(task_id) = as_i32(&el, "TaskId") {
            res.push(LivenessTaskEntry {
                task_id,
                task_reward: read_int_map(&el, "TaskReward"),
            });
        }
    }
    res
}

fn load_achievements(dir: &Path) -> Vec<AchievementCfg> {
    let mut res = Vec::new();
    for el in open_json_array(dir, "Achievement.json") {
        let id = as_i32(&el, "Id").unwrap_or(0);
        if id <= 0 {
            continue;
        }
        res.push(AchievementCfg {
            id,
            group_id: as_i32(&el, "GroupId").unwrap_or(0),
            level: as_i32(&el, "Level").unwrap_or(1),
            override_drop_id: as_i32(&el, "OverrideDropId").unwrap_or(0),
        });
    }
    res
}

fn load_achievement_groups(dir: &Path) -> Vec<AchievementGroupCfg> {
    let mut res = Vec::new();
    for el in open_json_array(dir, "AchievementGroup.json") {
        let id = as_i32(&el, "Id").unwrap_or(0);
        if id <= 0 {
            continue;
        }
        let enable = el.get("Enable").and_then(Value::as_bool).unwrap_or(false);
        let drop_id = as_i32(&el, "DropId").unwrap_or(0);
        res.push(AchievementGroupCfg {
            id,
            enable,
            drop_id,
        });
    }
    res
}

fn load_id_list(dir: &Path, file_name: &str) -> Vec<i32> {
    let mut res = Vec::new();
    for el in open_json_array(dir, file_name) {
        if let Some(id) = as_i32(&el, "Id")
            && id > 0
        {
            res.push(id);
        }
    }
    res
}

fn load_music_data(dir: &Path) -> (BTreeMap<i32, Vec<i32>>, HashSet<i32>) {
    let mut albums: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    let mut all_music = HashSet::new();

    for el in open_json_array(dir, "PhonographAlbum.json") {
        if let Some(aid) = as_i32(&el, "Id")
            && aid > 0
        {
            albums.entry(aid).or_default();
        }
    }

    for el in open_json_array(dir, "PhonographMusic.json") {
        let Some(mid) = as_i32(&el, "Id") else {
            continue;
        };
        if mid <= 0 {
            continue;
        }
        all_music.insert(mid);
        if let Some(Value::Array(alb_list)) = el.get("Album") {
            for a in alb_list {
                if let Some(aid) = a.as_i64().and_then(|n| i32::try_from(n).ok()) {
                    albums.entry(aid).or_default().push(mid);
                }
            }
        }
    }

    (albums, all_music)
}

fn load_activities(dir: &Path) -> Vec<ActivityConfigItem> {
    let mut res = Vec::new();
    for el in open_json_array(dir, "Activity.json") {
        let Some(id) = as_i32(&el, "Id") else {
            continue;
        };
        let Some(activity_type) = as_i32(&el, "Type") else {
            continue;
        };
        let tab_texture = el
            .get("TabTexture")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default();
        res.push(ActivityConfigItem {
            id,
            activity_type,
            tab_texture,
        });
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_kv_int_drop_packages_from_assets_db_without_bindata() {
        let drops = load_drop_packages(Path::new("/root/wuwa-server/assets/NonExistentBinData"));
        assert!(
            !drops.is_empty(),
            "DropPackage in kv_int must load from assets.db when BinData dir is absent"
        );
    }
}
