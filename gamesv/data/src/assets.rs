use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use crate::asset_indexes::AssetIndexes;
use crate::data_tables::{DataTables, TableRef};
use crate::fsm_graph_registry::FsmGraphRegistry;
use crate::lazy_flow_store::LazyFlowStore;
use crate::lazy_node_store::LazyNodeStore;
use crate::reward_index::RewardIndex;
use crate::tables::blueprint_config::BlueprintConfig;
use crate::tables::level_entity_config::LevelEntityConfig;
use crate::tables::template_config::TemplateConfig;

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct AlertAreaConfigEntry {
    #[serde(default)]
    pub Id: i32,
    #[serde(default)]
    pub Description: String,
    #[serde(default)]
    pub CreatorId: i32,
    #[serde(default)]
    pub MinValue: f64,
    #[serde(default)]
    pub MaxValue: f64,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct RollBlockGridSize {
    #[serde(default)]
    pub X: i32,
    #[serde(default)]
    pub Y: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct RollBlockBlockInit {
    #[serde(default)]
    pub Type: String,
    #[serde(default)]
    pub BlockId: i32,
    #[serde(default)]
    pub LayType: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct RollBlockCellItem {
    #[serde(default)]
    pub Type: String,
    #[serde(default)]
    pub Key: Option<i32>,
    #[serde(default)]
    pub InitActive: Option<bool>,
    #[serde(default)]
    pub Direction: Option<String>,
    #[serde(default)]
    pub Height: Option<f64>,
    #[serde(default)]
    pub Path: Option<RollBlockCellPath>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct RollBlockCellPath {
    #[serde(default)]
    pub Height: f64,
    #[serde(default)]
    pub Nodes: Vec<RollBlockGridSize>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct RollBlockCellConfig {
    #[serde(default)]
    pub Type: String,
    #[serde(default)]
    pub LinkDirection: Option<String>,
    #[serde(default)]
    pub JumpDirection: Option<String>,
    #[serde(default)]
    pub Key: Option<i32>,
    #[serde(default)]
    pub Block: Option<RollBlockBlockInit>,
    #[serde(default)]
    pub Items: Vec<RollBlockCellItem>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct RollBlockGridConfig {
    #[serde(default)]
    pub BoardSize: RollBlockGridSize,
    #[serde(default)]
    pub Grids: Vec<RollBlockCellConfig>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct RollBlockHintStep {
    #[serde(default)]
    pub Type: String,
    #[serde(default)]
    pub BlockId: i32,
    #[serde(default)]
    pub Direction: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct RollBlockLevelConfig {
    #[serde(default)]
    pub Difficulty: i32,
    #[serde(default)]
    pub GridConfig: RollBlockGridConfig,
    #[serde(default)]
    pub DefaultControlledBlockId: i32,
    #[serde(default)]
    pub HintSteps: Vec<RollBlockHintStep>,
    #[serde(default)]
    pub HintStepThreshold: Option<i32>,
    #[serde(default)]
    pub RewardId: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct RollBlockConfig {
    #[serde(default)]
    pub Id: i32,
    #[serde(default)]
    pub GroupName: String,
    #[serde(default)]
    pub Levels: Vec<RollBlockLevelConfig>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct RollBlockTemplateBlockEntry {
    #[serde(default)]
    pub BlockType: String,
    #[serde(default)]
    pub TemplateId: String,
    #[serde(default)]
    pub PhantomTemplateId: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct RollBlockTemplateGridEntry {
    #[serde(default)]
    pub GridType: String,
    #[serde(default)]
    pub TemplateId: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct RollBlockTemplateItemEntry {
    #[serde(default)]
    pub ItemType: String,
    #[serde(default)]
    pub TemplateId: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct RollBlockTemplateConfig {
    #[serde(default)]
    pub Id: i32,
    #[serde(default)]
    pub Name: String,
    #[serde(default)]
    pub Length: f32,
    #[serde(default)]
    pub DelayTime: f32,
    #[serde(default)]
    pub DeadDelayTime: f32,
    #[serde(default)]
    pub BlockTemplate: Vec<RollBlockTemplateBlockEntry>,
    #[serde(default)]
    pub GridTemplates: Vec<RollBlockTemplateGridEntry>,
    #[serde(default)]
    pub ItemTemplates: Vec<RollBlockTemplateItemEntry>,
    #[serde(default)]
    pub TreasureTemplate: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct DollGrabItemComboEntry {
    #[serde(default)]
    pub Id: i32,
    #[serde(default)]
    pub Name: String,
    #[serde(default)]
    pub MainItemId: i32,
    #[serde(default)]
    pub PartItemIds: Vec<i32>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct FurnitureConfigEntry {
    #[serde(default)]
    pub Id: i32,
    #[serde(default)]
    pub ActivityId: i32,
    #[serde(default)]
    pub TagId: i32,
    #[serde(default)]
    pub Atmosphere: i32,
    #[serde(default)]
    pub EntityId: i64,
    #[serde(default)]
    pub IsEntityGroup: bool,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct SpringFestivalDefaultFurnitures {
    #[serde(default)]
    pub ArrayInt: Vec<i32>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct SpringFestivalAreaEntry {
    #[serde(default)]
    pub Id: i32,
    #[serde(default)]
    pub Floor: i32,
    #[serde(default)]
    pub ActivityId: i32,
    #[serde(default)]
    pub MaxAtmosphere: i32,
    #[serde(default)]
    pub SlotEntityIds: Vec<i32>,
    #[serde(default)]
    pub AtmosphereLevelId: i32,
    #[serde(default)]
    pub DefaultFurnitures: HashMap<String, SpringFestivalDefaultFurnitures>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct SpringFestivalSkipEntryCfg {
    #[serde(default)]
    pub Id: i32,
    #[serde(default)]
    pub Activity: i32,
    #[serde(default)]
    pub FinishCondition: i32,
    #[serde(default)]
    pub UnlockCondition: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct SpringFestivalRewardEntry {
    #[serde(default)]
    pub Id: i32,
    #[serde(default)]
    pub ActivityId: i32,
    #[serde(default)]
    pub TabId: i32,
    #[serde(default)]
    pub ConditionGroup: i32,
    #[serde(default)]
    pub DropId: i32,
}

#[derive(Default)]
pub struct Assets {
    pub tables: DataTables,
    pub indexes: AssetIndexes,
    pub fsm_graphs: FsmGraphRegistry,
    pub quest_nodes: LazyNodeStore,
    pub level_play_nodes: LazyNodeStore,
    pub flow_states: LazyFlowStore,
    pub reward_index: RewardIndex,
    pub alert_area_configs: HashMap<i32, AlertAreaConfigEntry>,
    pub roll_blocks: HashMap<i32, RollBlockConfig>,
    pub roll_block_templates: HashMap<i32, RollBlockTemplateConfig>,
    pub doll_grab_combos: Vec<DollGrabItemComboEntry>,
    pub furniture_configs: HashMap<i32, FurnitureConfigEntry>,
    pub spring_festival_areas: Vec<SpringFestivalAreaEntry>,
    pub spring_festival_skip_entries: Vec<SpringFestivalSkipEntryCfg>,
    pub spring_festival_rewards: Vec<SpringFestivalRewardEntry>,
}

fn read_raw_table_str(
    base_path: &str,
    tables: &DataTables,
    name: &str,
) -> String {
    if let Some(db) = tables.assets_db()
        && let Some(raw) = db.read_raw_file(name)
    {
        raw
    } else {
        let path = format!("{base_path}/{name}.json");
        std::fs::read_to_string(&path).unwrap_or_default()
    }
}

fn parse_raw_vec<T: serde::de::DeserializeOwned>(
    base_path: &str,
    tables: &DataTables,
    name: &str,
) -> Vec<T> {
    let raw = read_raw_table_str(base_path, tables, name);
    if raw.is_empty() {
        return Vec::new();
    }
    serde_json::from_str::<Vec<T>>(&raw).unwrap_or_default()
}

impl Assets {
    pub fn load(base_path: &str) -> Result<Self, String> {
        let db_candidate = Path::new(base_path)
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join("assets.db");
        let db_path = if db_candidate.exists() {
            Some(db_candidate)
        } else if Path::new(crate::assets_db::DEFAULT_ASSETS_DB_PATH).exists() {
            Some(Path::new(crate::assets_db::DEFAULT_ASSETS_DB_PATH).to_path_buf())
        } else if Path::new("../../assets/assets.db").exists() {
            Some(Path::new("../../assets/assets.db").to_path_buf())
        } else {
            None
        };
        let db = if let Some(path) = db_path {
            Some(crate::assets_db::AssetsDb::open(&path).map_err(|e| e.to_string())?)
        } else {
            None
        };
        let tables = DataTables::init_with_db(base_path, db)?;
        let indexes = if let Some(db) = tables.assets_db() {
            AssetIndexes::from_db(db, &tables)?
        } else {
            AssetIndexes::from_tables(&tables)
        };
        let fsm_graphs = if let Some(db) = tables.assets_db() {
            FsmGraphRegistry::with_db(Arc::clone(db))
        } else {
            FsmGraphRegistry::from_configs(tables.ai_state_machine_config.as_slice())
        };

        let quest_path = format!("{base_path}/QuestNodeData.json");
        let lp_path = format!("{base_path}/LevelPlayNodeData.json");
        let flow_path = format!("{base_path}/FlowState.json");

        let (quest_nodes, level_play_nodes, flow_states) = if let Some(db) = tables.assets_db() {
            (
                LazyNodeStore::with_db(Arc::clone(db), 0, &quest_path),
                LazyNodeStore::with_db(Arc::clone(db), 1, &lp_path),
                LazyFlowStore::with_db(Arc::clone(db), &flow_path),
            )
        } else {
            (
                LazyNodeStore::from_file(0, &quest_path).map_err(|e| e.to_string())?,
                LazyNodeStore::from_file(1, &lp_path).map_err(|e| e.to_string())?,
                LazyFlowStore::from_file(&flow_path).map_err(|e| e.to_string())?,
            )
        };
        let reward_index = RewardIndex::new(tables.assets_db().cloned());
        let mut alert_area_configs = HashMap::new();
        for entry in parse_raw_vec::<AlertAreaConfigEntry>(base_path, &tables, "AlertAreaConfig") {
            if entry.Id > 0 {
                alert_area_configs.insert(entry.Id, entry);
            }
        }
        let mut roll_blocks = HashMap::new();
        for entry in parse_raw_vec::<RollBlockConfig>(base_path, &tables, "RollBlock") {
            if entry.Id > 0 {
                roll_blocks.insert(entry.Id, entry);
            }
        }
        let mut roll_block_templates = HashMap::new();
        for entry in parse_raw_vec::<RollBlockTemplateConfig>(base_path, &tables, "RollBlockTemplate") {
            if entry.Id > 0 {
                roll_block_templates.insert(entry.Id, entry);
            }
        }
        let doll_grab_combos =
            parse_raw_vec::<DollGrabItemComboEntry>(base_path, &tables, "DollGrabItemComboCsv");
        let mut furniture_configs = HashMap::new();
        for entry in parse_raw_vec::<FurnitureConfigEntry>(base_path, &tables, "Furniture") {
            if entry.Id > 0 {
                furniture_configs.insert(entry.Id, entry);
            }
        }
        let spring_festival_areas =
            parse_raw_vec::<SpringFestivalAreaEntry>(base_path, &tables, "SpringFestivalArea");
        let spring_festival_skip_entries = parse_raw_vec::<SpringFestivalSkipEntryCfg>(
            base_path,
            &tables,
            "SpringFestivalSkipEntry",
        );
        let spring_festival_rewards =
            parse_raw_vec::<SpringFestivalRewardEntry>(base_path, &tables, "SpringFestivalReward");

        Ok(Self {
            tables,
            indexes,
            fsm_graphs,
            quest_nodes,
            level_play_nodes,
            flow_states,
            reward_index,
            alert_area_configs,
            roll_blocks,
            roll_block_templates,
            doll_grab_combos,
            furniture_configs,
            spring_festival_areas,
            spring_festival_skip_entries,
            spring_festival_rewards,
        })
    }

    pub fn load_default() -> Result<Self, String> {
        let base_path = if Path::new("assets/BinData").exists()
            || Path::new(crate::assets_db::DEFAULT_ASSETS_DB_PATH).exists()
        {
            "assets/BinData"
        } else {
            "../../assets/BinData"
        };
        Self::load(base_path)
    }

    pub fn read_raw_file(&self, table_name: &str) -> Option<String> {
        if let Some(db) = self.tables.assets_db()
            && let Some(raw) = db.read_raw_file(table_name)
        {
            return Some(raw);
        }
        for dir in ["assets/BinData", "../../assets/BinData"] {
            let p = Path::new(dir).join(format!("{table_name}.json"));
            if let Ok(raw) = std::fs::read_to_string(p) {
                return Some(raw);
            }
        }
        None
    }

    pub fn read_raw_json_array(&self, table_name: &str) -> Vec<serde_json::Value> {
        self.read_raw_file(table_name)
            .and_then(|raw| serde_json::from_str::<serde_json::Value>(&raw).ok())
            .and_then(|v| v.as_array().cloned())
            .unwrap_or_default()
    }

    pub fn resolve_map_config_id(&self, instance_id: i32) -> i32 {
        self.tables
            .instance_dungeon
            .get_data_by_id(&instance_id)
            .map(|inst| inst.MapConfigId)
            .unwrap_or(instance_id)
    }

    pub fn resolve_instance_id_for_map(&self, map_config_id: i32) -> i32 {
        if let Some(inst) = self.tables.instance_dungeon.get_data_by_id(&map_config_id)
            && inst.MapConfigId == map_config_id
        {
            return inst.Id;
        }
        self.tables
            .instance_dungeon
            .as_slice()
            .iter()
            .find(|inst| inst.MapConfigId == map_config_id && inst.InstType == 1)
            .map_or(map_config_id, |inst| inst.Id)
    }

    pub fn find_level_entity(
        &self,
        map_id: i32,
        entity_id: i64,
    ) -> Option<TableRef<'_, LevelEntityConfig>> {
        if let Some(cfg) = self.indexes.find_level_entity(&self.tables, map_id, entity_id) {
            return Some(cfg);
        }
        let resolved_map_id = self.resolve_map_config_id(map_id);
        if resolved_map_id != map_id {
            return self
                .indexes
                .find_level_entity(&self.tables, resolved_map_id, entity_id);
        }
        None
    }

    pub fn find_level_entity_any_map(&self, entity_id: i64) -> Option<TableRef<'_, LevelEntityConfig>> {
        let &idx = self.indexes.level_entity_indices_by_id(entity_id).first()?;
        self.get_level_entity_by_idx(idx)
    }

    pub fn get_level_entity_by_idx(&self, idx: u32) -> Option<TableRef<'_, LevelEntityConfig>> {
        if let Some(db) = self.tables.assets_db() {
            return db.get_level_entity_by_idx(idx).map(TableRef::Owned);
        }
        self.tables
            .level_entity_config
            .as_slice()
            .get(idx as usize)
            .map(TableRef::Borrowed)
    }

    pub fn find_blueprint_config(&self, blueprint_type: &str) -> Option<&BlueprintConfig> {
        self.indexes.find_blueprint_config(&self.tables, blueprint_type)
    }

    pub fn find_template_config(&self, blueprint_type: &str) -> Option<TableRef<'_, TemplateConfig>> {
        self.indexes.find_template_config(&self.tables, blueprint_type)
    }

    pub fn spawn_monster_configs(
        &self,
        map_id: i32,
        entity_id: i64,
    ) -> Vec<crate::tables::entity_components::extra::SpawnMonsterConfig> {
        let cfg = self
            .find_level_entity(map_id, entity_id)
            .or_else(|| self.find_level_entity_any_map(entity_id));
        let Some(cfg) = cfg else {
            return Vec::new();
        };
        let mut merged = cfg.ComponentsData.clone();
        if let Some(tpl) = self.find_template_config(&cfg.BlueprintType) {
            tpl.ComponentsData.merge_into(&mut merged);
        }
        for entry in merged.extra.iter().chain(merged.template_extra.iter()) {
            if let crate::tables::entity_components::extra::ExtraEntry::SpawnMonsterComponent(sm) =
                entry
                && !sm.SpawnMonsterConfigs.is_empty()
            {
                return sm.SpawnMonsterConfigs.clone();
            }
        }
        Vec::new()
    }

    pub fn spawn_monster_targets(&self, map_id: i32, entity_id: i64) -> Vec<i32> {
        let mut out = Vec::new();
        for wave in self.spawn_monster_configs(map_id, entity_id) {
            for tid in wave.TargetsToAwake {
                if tid > 0 && !out.contains(&tid) {
                    out.push(tid);
                }
            }
        }
        out
    }

    pub fn has_spawn_monster_component(&self, map_id: i32, entity_id: i64) -> bool {
        let cfg = self
            .find_level_entity(map_id, entity_id)
            .or_else(|| self.find_level_entity_any_map(entity_id));
        let Some(cfg) = cfg else {
            return false;
        };
        let mut merged = cfg.ComponentsData.clone();
        if let Some(tpl) = self.find_template_config(&cfg.BlueprintType) {
            tpl.ComponentsData.merge_into(&mut merged);
        }
        merged.extra.iter().chain(merged.template_extra.iter()).any(|e| {
            matches!(
                e,
                crate::tables::entity_components::extra::ExtraEntry::SpawnMonsterComponent(_)
            )
        })
    }

    pub fn quest_requires_resource(&self, quest_id: i32) -> bool {
        if self.tables.quest_resource_white.get_data_by_id(&quest_id).is_some() {
            return false;
        }
        if self
            .tables
            .quest_ref_map_block_config
            .get_data_by_id(&quest_id)
            .is_some()
        {
            return true;
        }
        self.tables
            .quest_ref_video_config
            .as_slice()
            .iter()
            .any(|v| v.QuestId == quest_id)
    }

    pub fn get_motorcycle_config_id(&self) -> Option<i32> {
        self.tables.get_motorcycle_config_id()
    }

    pub fn alert_area_bounds(&self, area_id: i32) -> (f64, f64) {
        self.alert_area_configs
            .get(&area_id)
            .map(|c| {
                let lo = c.MinValue.min(c.MaxValue);
                let hi = c.MaxValue.max(lo);
                (lo, hi)
            })
            .unwrap_or((0.0, 100.0))
    }
}

#[cfg(test)]
mod tests {
    use super::Assets;
    use std::path::Path;

    #[test]
    fn load_resolves_assets_db_when_bindata_dir_missing() {
        let db_path = Path::new("/root/wuwa-server/assets/assets.db");
        if !db_path.is_file() {
            return;
        }
        let assets = Assets::load("/root/wuwa-server/assets/NonExistentBinData")
            .expect("Assets::load must open sibling assets.db without BinData dir");
        assert!(assets.tables.assets_db().is_some());
        assert!(!assets.tables.role_info.items.is_empty());
        for tbl in [
            "FishingTechEffect",
            "FishingPoint",
            "FishingCage",
            "MotorLvl",
            "MaterialReplace",
            "AutoPilotCircles",
        ] {
            assert!(
                !assets.read_raw_json_array(tbl).is_empty(),
                "assets.db raw_files must contain {tbl}"
            );
        }
    }
}
