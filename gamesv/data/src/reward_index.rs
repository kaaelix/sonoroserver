use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

use serde_json::{Map, Value};

use crate::assets_db::AssetsDb;

const MAX_CACHED_TABLES: usize = 48;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemPair {
    pub item_id: i32,
    pub count: i32,
}

#[derive(Debug, Clone)]
pub struct RowEntry {
    pub key: i64,
    pub drop_id: i32,
    pub alt_drop_id: i32,
    pub cost_id: i32,
    pub cost_count: i32,
    pub extra_int: i32,
    pub items: Vec<ItemPair>,
    pub pay_items: Vec<ItemPair>,
}

pub const fn composite_key(high: i32, low: i32) -> i64 {
    ((high as i64) << 32) | ((low as u32) as i64)
}

pub fn resolve_table_name(table_name: &str) -> &str {
    match table_name {
        "ActivityReward" => "CiacconaActivityReward",
        "Avignon" => "AvignonTask",
        "BlackCoastTheme" => "BlackCoastThemeTaskRe",
        "CertificateReward" => "BattlePassReward",
        "CorniceMeeting" => "CorniceChallenge",
        "DailyAdventure" => "DailyAdventureTask",
        "DropCatch" => "DropCatchReward",
        "ExploreScoreReward" => "ExploreReward",
        "FarmGold" => "FarmGoldScore",
        "FishingHandBook" => "FishingIllustratedReward",
        "FlagChallenge" => "FlagChallengeTask",
        "FunPlay" => "FunPlayActivityChallenge",
        "GiantActivity" => "GiantDeliveryReward",
        "Giveback" => "GivebackScoreReward",
        "HarvestActivity" => "RiskHarvestScoreReward",
        "HonamiStoryReward" => "HonamiStoryScoreReward",
        "HonamiStoryTask" => "HonamiStoryLimitTask",
        "JokerGuessLevel" => "GuessJokerLevel",
        "KurotatoTask" => "KurotatoLimitAward",
        "ExploreActivity" => "ExploreActivityTask",
        "TeamParkour" => "TeamParKOurReward",
        "WeeklyFramework" => "WeeklyFrameAward",
        "SpringSignReward" => "SpringReward",
        "MotorFightMainLevel" => "MotorFightLevel",
        "LinkageCheckIn" => "LinkageReward",
        "LongShanReward" => "LongShanScoreReward",
        "MapTravelLevel" => "MapTravelConfig",
        "MapTravelTask" => "TravelTask",
        "MixDrink" => "DrinksRoleInvite",
        "MoonPhase" => "MoonPhaseReward",
        "MowTower" => "MowTowerRewardRe",
        "NewBieCourse" => "NewbieCourse",
        "NewJourneyTask" => "WorldNewJourney",
        "NewbieCarnival" => "NewbieCarnivalTask",
        "NewbieScore" => "NewbieMainActReward",
        "OccupyReward" => "OccupyScore",
        "OnlineMotorActivity" => "OnlineMotorGlobalTask",
        "Parkour" => "ParkourChallenge",
        "PhantomBattleReward" => "PhantomBattleCardReward",
        "PhantomCollect" => "PhantomCollectActivity",
        "PhotoFight" => "PhotoFightReward",
        "PhotoMemory" => "PhotoMemoryCollect",
        "PreheatSign" => "PreheatSignRe",
        "QingXiao" => "GuQinActivityTask",
        "RacingBetsTask" => "RacingBetsReward",
        "RegressBonus" => "RegressBonusReward",
        "RegressReward" => "RegressDisposableReward",
        "RegressTask" => "RegressQuest",
        "RiskHarvest" => "RiskHarvestScoreReward",
        "RogueResEnding" => "RogueResEndAward",
        "RogueResIllustration" => "RogueResCollection",
        "RoguelikeReward" => "RogueSeasonReward",
        "RoleSkinTrial" => "RoleSkinTrialInfo",
        "RoleTowerRankReward" => "RoleTowerReward",
        "RoleTowerTask" => "RoleTowerReward",
        "RoleTrial" => "RoleTrialInfo",
        "ScratchCard" => "ScratchCardRewardRe",
        "ThemeCelebration" => "BlackCoastThemeRewardRe",
        "TimePointReward" => "TimePointRewardActivity",
        "TrackMoonPhase" => "TrackMoonPhaseActivity",
        "WuWuKuji" => "WuWuTaskPackage",
        other => other,
    }
}

#[derive(Default)]
struct IndexState {
    slots: HashMap<String, Arc<Vec<RowEntry>>>,
    order: VecDeque<String>,
}

#[derive(Default)]
pub struct RewardIndex {
    db: Option<Arc<AssetsDb>>,
    state: Mutex<IndexState>,
}

impl RewardIndex {
    pub fn new(db: Option<Arc<AssetsDb>>) -> Self {
        Self {
            db,
            state: Mutex::new(IndexState::default()),
        }
    }

    pub fn get_rows(&self, table_name: &str) -> Arc<Vec<RowEntry>> {
        let mut guard = self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(hit) = guard.slots.get(table_name) {
            return Arc::clone(hit);
        }
        let rows = self.load_table_rows(table_name);
        let arc = Arc::new(rows);
        if guard.order.len() >= MAX_CACHED_TABLES
            && let Some(oldest) = guard.order.pop_front()
        {
            guard.slots.remove(&oldest);
        }
        let key = table_name.to_string();
        guard.order.push_back(key.clone());
        guard.slots.insert(key, Arc::clone(&arc));
        arc
    }

    pub fn get_row(&self, table_name: &str, key: i64) -> Option<RowEntry> {
        let rows = self.get_rows(table_name);
        rows.iter().find(|r| r.key == key).cloned()
    }

    pub fn get_row_at_or_first(&self, table_name: &str, key: i64) -> Option<RowEntry> {
        let rows = self.get_rows(table_name);
        rows.iter()
            .find(|r| r.key == key)
            .or_else(|| rows.first())
            .cloned()
    }

    fn load_table_rows(&self, table_name: &str) -> Vec<RowEntry> {
        let resolved = resolve_table_name(table_name);
        let json_str = self
            .db
            .as_ref()
            .and_then(|db| db.read_raw_file(resolved))
            .or_else(|| {
                for dir in ["assets/BinData", "../../assets/BinData"] {
                    let path = format!("{dir}/{resolved}.json");
                    if let Ok(raw) = std::fs::read_to_string(path) {
                        return Some(raw);
                    }
                }
                None
            });
        let Some(json_str) = json_str else {
            return Vec::new();
        };
        let Ok(parsed) = serde_json::from_str::<Value>(&json_str) else {
            return Vec::new();
        };
        let mut rows = Vec::new();
        match parsed {
            Value::Array(arr) => {
                for elem in arr {
                    if let Value::Object(obj) = elem {
                        rows.push(parse_row_object(&obj));
                    }
                }
            }
            Value::Object(obj) => {
                for v in obj.values() {
                    if let Value::Object(inner) = v {
                        rows.push(parse_row_object(inner));
                    }
                }
            }
            _ => {}
        }
        rows
    }
}

fn as_int(val: &Value) -> Option<i32> {
    match val {
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                i32::try_from(i).ok()
            } else if let Some(f) = n.as_f64() {
                if f.is_finite() { Some(f as i32) } else { None }
            } else {
                None
            }
        }
        Value::String(s) => s.parse::<i32>().ok(),
        _ => None,
    }
}

fn first_int(obj: &Map<String, Value>, keys: &[&str]) -> i32 {
    for &k in keys {
        if let Some(v) = obj.get(k) {
            match v {
                Value::Number(_) | Value::String(_) => {
                    if let Some(iv) = as_int(v)
                        && iv != 0
                    {
                        return iv;
                    }
                }
                Value::Array(arr) => {
                    if let Some(first) = arr.first()
                        && let Some(iv) = as_int(first)
                        && iv != 0
                    {
                        return iv;
                    }
                }
                _ => {}
            }
        }
    }
    0
}

fn parse_item_pairs(val: &Value) -> Vec<ItemPair> {
    let mut items = Vec::new();
    match val {
        Value::Object(map) => {
            let pair_id = first_int(map, &["ItemId", "Id", "Item1", "itemId", "id"]);
            let pair_cnt = first_int(map, &["Count", "Num", "Amount", "Item2", "count"]);
            if pair_id > 0 && pair_cnt > 0 {
                items.push(ItemPair {
                    item_id: pair_id,
                    count: pair_cnt,
                });
                return items;
            }
            for (k, v) in map {
                let Ok(item_id) = k.parse::<i32>() else {
                    continue;
                };
                let Some(count) = as_int(v) else {
                    continue;
                };
                if item_id > 0 && count > 0 {
                    items.push(ItemPair { item_id, count });
                }
            }
        }
        Value::Array(arr) => {
            for elem in arr {
                let Some(obj) = elem.as_object() else {
                    continue;
                };
                let item_id = first_int(obj, &["ItemId", "Id", "Item1", "itemId", "id"]);
                let count = first_int(obj, &["Count", "Num", "Amount", "Item2", "count"]);
                if item_id > 0 && count > 0 {
                    items.push(ItemPair { item_id, count });
                }
            }
        }
        _ => {}
    }
    items
}

fn parse_first_cost(obj: &Map<String, Value>) -> (i32, i32) {
    for k in ["Consume", "Cost", "ConsumeItem"] {
        if let Some(Value::Object(m)) = obj.get(k) {
            for (ks, vs) in m {
                if let Ok(id) = ks.parse::<i32>()
                    && let Some(cnt) = as_int(vs)
                    && id > 0
                    && cnt > 0
                {
                    return (id, cnt);
                }
            }
        }
    }
    (
        first_int(obj, &["ConsumeId", "CostId"]),
        first_int(obj, &["ConsumeCount", "CostCount"]),
    )
}

pub fn parse_row_object(obj: &Map<String, Value>) -> RowEntry {
    let key = if let (Some(bp_v), Some(lvl_v)) = (obj.get("BattlePassId"), obj.get("Level"))
        && let (Some(bp_id), Some(lvl)) = (as_int(bp_v), as_int(lvl_v))
    {
        composite_key(bp_id, lvl)
    } else {
        i64::from(first_int(
            obj,
            &[
                "Id",
                "TaskId",
                "RewardId",
                "InstId",
                "LevelId",
                "IdKey",
                "BoardId",
                "RecallId",
                "Level",
                "ActivityId",
                "GoodsId",
                "BuffId",
                "RoleId",
                "MailId",
                "Score",
            ],
        ))
    };

    let drop_id = first_int(
        obj,
        &[
            "DropId",
            "DropIds",
            "Drop",
            "RewardId",
            "RewardIds",
            "FirstClearDropId",
            "FirstPassDropId",
            "PassDropId",
            "NormalDropId",
            "UnlockDropId",
            "FreeDropId",
            "PreviewDrop",
            "DropGroup",
            "DropReward",
            "RewardDropId",
            "RewardDrop",
            "EntrustReward",
            "PassReward",
            "SignReward",
            "DailyClearDropId",
            "FirstWinReward",
            "RewardGroup",
            "EndReward",
            "Award",
            "FirstAward",
            "ScoreReward",
            "PhantomReward",
            "FinalRewardId",
            "Reward",
            "TaskReward",
            "TargetReward",
            "FirstDropId",
            "ShowReward",
            "ExchangeRewardId",
        ],
    );
    let alt_drop_id = first_int(
        obj,
        &[
            "GrandRewardId",
            "PayDropId",
            "ExtraDrop",
            "PermanentPreviewDrop",
            "PayDrop",
            "HighDropId",
            "MaxLevelDropId",
            "KeepDropId",
            "LevelPassReward",
            "DataDockReward",
            "PhantomSideQuestReward",
        ],
    );
    let extra_int = first_int(
        obj,
        &[
            "NeedProgress",
            "NeedPt",
            "RewardScore",
            "TargetScore",
            "Score",
            "LevelUpExp",
            "QuestId",
            "RewardUnlockCount",
            "NeedScore",
            "Target",
        ],
    );

    let mut items = Vec::new();
    for k in [
        "FreeReward",
        "TaskReward",
        "Reward",
        "TargetReward",
        "Content",
        "UnlockReward",
        "Items",
        "ItemMap",
        "SignRewards",
        "Award",
        "Rewards",
        "ActivityReward",
        "RewardItem",
        "RewardInfo",
    ] {
        if let Some(v) = obj.get(k) {
            let parsed = parse_item_pairs(v);
            if !parsed.is_empty() {
                items = parsed;
                break;
            }
        }
    }

    let mut pay_items = Vec::new();
    for k in ["PayReward", "ExtraReward"] {
        if let Some(v) = obj.get(k) {
            let parsed = parse_item_pairs(v);
            if !parsed.is_empty() {
                pay_items = parsed;
                break;
            }
        }
    }

    let (cost_id, cost_count) = parse_first_cost(obj);
    RowEntry {
        key,
        drop_id,
        alt_drop_id,
        cost_id,
        cost_count,
        extra_int,
        items,
        pay_items,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reward_index_parses_key_composite_drops_and_item_maps() {
        let parsed: Value = serde_json::from_str(
            r#"[{"BattlePassId": 2, "Level": 5, "FreeReward": {"2": 10000}, "PayReward": {"3": 680}, "DropId": 4101}]"#,
        )
        .unwrap();
        let row = parse_row_object(parsed[0].as_object().unwrap());
        assert_eq!(row.key, composite_key(2, 5));
        assert_eq!(row.drop_id, 4101);
        assert_eq!(row.items[0].item_id, 2);
        assert_eq!(row.pay_items[0].count, 680);
    }

    #[test]
    fn reward_index_resolve_table_name_and_real_bindata_loads() {
        assert_eq!(resolve_table_name("Avignon"), "AvignonTask");
        assert_eq!(resolve_table_name("ActivityReward"), "CiacconaActivityReward");
        assert_eq!(resolve_table_name("CertificateReward"), "BattlePassReward");
        assert_eq!(resolve_table_name("WuWuKuji"), "WuWuTaskPackage");
        assert_eq!(resolve_table_name("Giveback"), "GivebackScoreReward");
        assert_eq!(resolve_table_name("RogueResEnding"), "RogueResEndAward");
        assert_eq!(resolve_table_name("MixDrink"), "DrinksRoleInvite");

        let db_path = std::path::Path::new("/root/wuwa-server/assets/assets.db");
        if !db_path.exists() {
            return;
        }
        let db = AssetsDb::open(db_path).unwrap();
        let idx = RewardIndex::new(Some(db));
        for tbl in [
            "Avignon",
            "RogueSeasonReward",
            "RogueLimitTimeReward",
            "KurotatoResAward",
            "HonamiStoryScoreReward",
            "DailyAdventurePoint",
            "PhantomBattleBadgeReward",
            "ExploreActivity",
            "WeeklyFrameAward",
            "TeamParKOurReward",
            "SpringReward",
            "MotorFightLevel",
            "MotorParkourReward",
        ] {
            assert!(!idx.get_rows(tbl).is_empty(), "empty rows for {tbl}");
        }

        let silent = idx.get_row("SilentAreaDetection", 30000000).unwrap();
        assert!(silent.drop_id > 0);
        let bf = idx.get_row("RogueWeeklyBF", 1).unwrap();
        assert!(bf.drop_id > 0);
        let rr_ins = idx.get_row("RoverRogueIns", 9860).unwrap();
        assert_eq!(rr_ins.key, 9860);
        let scratch = idx.get_row("ScratchCard", 1).unwrap();
        assert!(!scratch.items.is_empty());
        let turn = idx.get_row("TurntableAwards", 1).unwrap();
        assert!(!turn.items.is_empty());
        let sign = idx.get_row("ActivitySign", 100100001).unwrap();
        assert_eq!(sign.items.len(), 7);
    }
}
