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

#[derive(Default)]
pub struct Assets {
    pub tables: DataTables,
    pub indexes: AssetIndexes,
    pub fsm_graphs: FsmGraphRegistry,
    pub quest_nodes: LazyNodeStore,
    pub level_play_nodes: LazyNodeStore,
    pub flow_states: LazyFlowStore,
    pub reward_index: RewardIndex,
}

impl Assets {
    pub fn load(base_path: &str) -> Result<Self, String> {
        let db_candidate = Path::new(base_path)
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join("assets.db");
        let db = if db_candidate.exists() {
            Some(crate::assets_db::AssetsDb::open(&db_candidate).map_err(|e| e.to_string())?)
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

        Ok(Self {
            tables,
            indexes,
            fsm_graphs,
            quest_nodes,
            level_play_nodes,
            flow_states,
            reward_index,
        })
    }

    pub fn load_default() -> Result<Self, String> {
        Self::load(if Path::new("assets/BinData").exists() {
            "assets/BinData"
        } else {
            "../../assets/BinData"
        })
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
}
