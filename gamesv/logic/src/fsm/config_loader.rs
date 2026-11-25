use std::sync::Arc;

use sonoro_gamesv_data::Assets;
use sonoro_gamesv_data::fsm_graph_registry::Graph;
use sonoro_gamesv_data::tables::ai_state_machine_config::AiStateMachineConfig;

pub fn from_ai_base_id(assets: &Assets, ai_base_id: i32) -> Option<Arc<Graph>> {
    if ai_base_id <= 0 {
        return None;
    }
    let ai_base = assets.tables.ai_base.get_data_by_id(&ai_base_id)?;
    if ai_base.StateMachine.is_empty() {
        return None;
    }
    from_state_machine_id(assets, &ai_base.StateMachine)
}

pub fn has_usable_ai_base_id(assets: &Assets, ai_base_id: i32) -> bool {
    from_ai_base_id(assets, ai_base_id).is_some()
}

pub fn from_state_machine_id(assets: &Assets, id: &str) -> Option<Arc<Graph>> {
    if id.is_empty() {
        return None;
    }
    assets.fsm_graphs.get(id)
}

pub fn get_common_fsm(assets: &Assets) -> Option<AiStateMachineConfig> {
    if let Some(cfg) = assets
        .tables
        .ai_state_machine_config
        .get_data_by_id(&"SM_Common".to_string())
    {
        return Some(cfg.into_owned());
    }
    assets
        .tables
        .assets_db()
        .and_then(|db| db.get_ai_state_machine_config("SM_Common"))
        .map(|arc| (*arc).clone())
}
