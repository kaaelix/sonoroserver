use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct ExploreToolsInfo {
    pub unlocked_explore_skills: Vec<i32>,
    pub active_explore_skill: i32,
    pub roulette: Vec<i32>,
    pub explore_extra_item_id: i32,
    pub active_function_skill: i32,
    pub function_roulette: Vec<i32>,
    pub function_extra_item_id: i32,
    pub active_motorcycle_skill: i32,
    pub motorcycle_roulette: Vec<i32>,
    pub motorcycle_extra_item_id: i32,
    pub active_trap_defense_skill: i32,
    pub trap_defense_roulette: Vec<i32>,
    pub trap_defense_extra_item_id: i32,
}

impl Default for ExploreToolsInfo {
    fn default() -> Self {
        Self {
            unlocked_explore_skills: Vec::new(),
            active_explore_skill: 1001,
            roulette: Vec::new(),
            explore_extra_item_id: 0,
            active_function_skill: 0,
            function_roulette: Vec::new(),
            function_extra_item_id: 0,
            active_motorcycle_skill: 0,
            motorcycle_roulette: Vec::new(),
            motorcycle_extra_item_id: 0,
            active_trap_defense_skill: 0,
            trap_defense_roulette: Vec::new(),
            trap_defense_extra_item_id: 0,
        }
    }
}
