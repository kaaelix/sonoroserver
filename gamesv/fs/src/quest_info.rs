use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct ActiveQuest {
    pub quest_id: i32,
    pub active_nodes: Vec<i32>,
    pub finished_nodes: Vec<i32>,
    pub killed_entities: Vec<i32>,
}

impl ActiveQuest {
    pub fn is_node_active(&self, node_id: i32) -> bool {
        self.active_nodes.contains(&node_id)
    }

    pub fn is_node_finished(&self, node_id: i32) -> bool {
        self.finished_nodes.contains(&node_id)
    }

    pub fn mark_node_active(&mut self, node_id: i32) -> bool {
        if self.active_nodes.contains(&node_id) {
            return false;
        }
        self.active_nodes.push(node_id);
        true
    }

    pub fn mark_node_inactive(&mut self, node_id: i32) -> bool {
        let before = self.active_nodes.len();
        self.active_nodes.retain(|&id| id != node_id);
        self.active_nodes.len() != before
    }

    pub fn record_killed_entity(&mut self, entity_id: i32) -> bool {
        if self.killed_entities.contains(&entity_id) {
            return false;
        }
        self.killed_entities.push(entity_id);
        true
    }

    pub fn has_killed_entity(&self, entity_id: i32) -> bool {
        self.killed_entities.contains(&entity_id)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct QuestVar {
    pub key: String,
    pub int_val: i32,
    pub bool_val: bool,
    pub str_val: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct ChildQuestRecord {
    pub quest_id: i32,
    pub node_id: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct EntityStateRecord {
    pub entity_id: i64,
    pub state_name: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct AreaStateRecord {
    pub area_id: i32,
    pub state: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct AreaTimeRecord {
    pub area_id: i32,
    pub hour: i32,
    pub minute: i32,
    pub locked: bool,
}

impl Default for AreaTimeRecord {
    fn default() -> Self {
        Self {
            area_id: 0,
            hour: 12,
            minute: 0,
            locked: false,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct AudioStateRecord {
    pub group: String,
    pub state: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct QuestBranchRecord {
    pub page_id: i32,
    pub cur_branch: i32,
    pub complete_branches: Vec<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct HiddenGroupRecord {
    pub group_key: String,
    pub entity_ids: Vec<i32>,
    pub data_layers: Vec<i32>,
    pub level_plays: Vec<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct WorldState {
    pub loaded_datalayers: Vec<i32>,
    pub unloaded_datalayers: Vec<i32>,
    pub loaded_sublevels: Vec<String>,
    pub hidden_sublevels: Vec<String>,
    pub hidden_groups: Vec<HiddenGroupRecord>,
    pub hidden_entities: Vec<i32>,
    pub shown_entities: Vec<i32>,
    pub locked_entities: Vec<i32>,
    pub unlocked_map_marks: Vec<i32>,
    pub hidden_map_marks: Vec<i32>,
    pub entity_states: Vec<EntityStateRecord>,
    pub active_guests: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct EnvState {
    pub area_states: Vec<AreaStateRecord>,
    pub area_times: Vec<AreaTimeRecord>,
    pub audio_states: Vec<AudioStateRecord>,
    pub weather_id: i32,
    pub weather_locked: bool,
    pub unlocked_weather_switches: Vec<i32>,
    pub time_locked: bool,
    pub time_lock_hour: i32,
    pub time_lock_minute: i32,
    pub trial_roles: Vec<i32>,
    pub pre_trial_roles: Vec<i32>,
    pub pre_trial_cur_role: i32,
}

impl Default for EnvState {
    fn default() -> Self {
        Self {
            area_states: Vec::new(),
            area_times: Vec::new(),
            audio_states: Vec::new(),
            weather_id: 0,
            weather_locked: false,
            unlocked_weather_switches: Vec::new(),
            time_locked: false,
            time_lock_hour: 8,
            time_lock_minute: 0,
            trial_roles: Vec::new(),
            pre_trial_roles: Vec::new(),
            pre_trial_cur_role: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct DailyState {
    pub daily_quests: Vec<i32>,
    pub claimed_daily_rewards: Vec<i32>,
    pub preferred_daily_area: i32,
    pub preferred_daily_influence: i32,
}

impl Default for DailyState {
    fn default() -> Self {
        Self {
            daily_quests: Vec::new(),
            claimed_daily_rewards: Vec::new(),
            preferred_daily_area: 1,
            preferred_daily_influence: 0,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct ProgressState {
    pub focus_deter_cond: bool,
    pub quest_branches: Vec<QuestBranchRecord>,
    pub completed_bvb: Vec<i32>,
    pub read_mails: Vec<i32>,
    pub read_phone_messages: Vec<i32>,
    pub received_telecoms: Vec<i32>,
    pub opened_activities: Vec<i32>,
    pub completed_photo_nodes: Vec<ChildQuestRecord>,
    pub entered_level_plays: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct QuestInfo {
    pub finished_quests: Vec<i32>,
    pub finished_child_quests: Vec<ChildQuestRecord>,
    pub active_quests: Vec<ActiveQuest>,
    pub tracked_quest_id: i32,
    pub focus_quest_id: i32,
    pub vars: Vec<QuestVar>,
    pub unlocked_systems: Vec<i32>,
    pub completed_dungeons: Vec<i32>,
    pub completed_level_plays: Vec<i32>,
    pub opened_chests: Vec<i32>,
    pub next_flow_inc_id: i64,
    pub world: WorldState,
    pub env: EnvState,
    pub daily: DailyState,
    pub progress: ProgressState,
}

impl Default for QuestInfo {
    fn default() -> Self {
        Self {
            finished_quests: Vec::new(),
            finished_child_quests: Vec::new(),
            active_quests: Vec::new(),
            tracked_quest_id: 139000025,
            focus_quest_id: 0,
            vars: Vec::new(),
            unlocked_systems: Vec::new(),
            completed_dungeons: Vec::new(),
            completed_level_plays: Vec::new(),
            opened_chests: Vec::new(),
            next_flow_inc_id: 100,
            world: WorldState::default(),
            env: EnvState::default(),
            daily: DailyState::default(),
            progress: ProgressState::default(),
        }
    }
}

impl QuestInfo {
    pub const DATA_PATH: &'static str = "quest_info";

    pub fn has_finished_quest(&self, quest_id: i32) -> bool {
        self.finished_quests.contains(&quest_id)
    }

    pub fn has_finished_child_quest(&self, quest_id: i32, node_id: i32) -> bool {
        if self
            .finished_child_quests
            .iter()
            .any(|r| r.quest_id == quest_id && r.node_id == node_id)
        {
            return true;
        }
        if let Some(aq) = self.find_active_quest(quest_id) {
            return aq.finished_nodes.contains(&node_id);
        }
        self.has_finished_quest(quest_id)
    }

    pub fn has_active_quest(&self, quest_id: i32) -> bool {
        self.find_active_quest(quest_id).is_some()
    }

    pub fn find_active_quest(&self, quest_id: i32) -> Option<&ActiveQuest> {
        self.active_quests.iter().find(|aq| aq.quest_id == quest_id)
    }

    pub fn find_active_quest_mut(&mut self, quest_id: i32) -> Option<&mut ActiveQuest> {
        self.active_quests
            .iter_mut()
            .find(|aq| aq.quest_id == quest_id)
    }

    pub fn get_or_add_active_quest(&mut self, quest_id: i32) -> &mut ActiveQuest {
        if let Some(pos) = self
            .active_quests
            .iter()
            .position(|aq| aq.quest_id == quest_id)
        {
            return &mut self.active_quests[pos];
        }
        self.active_quests.push(ActiveQuest {
            quest_id,
            ..Default::default()
        });
        self.active_quests.last_mut().expect("just pushed")
    }

    pub fn finish_quest(&mut self, quest_id: i32) {
        append_unique_int(&mut self.finished_quests, quest_id);
        if let Some(pos) = self
            .active_quests
            .iter()
            .position(|aq| aq.quest_id == quest_id)
        {
            let aq = self.active_quests.remove(pos);
            for nid in aq.finished_nodes {
                self.record_child_quest_finished(quest_id, nid);
            }
        }
    }

    pub fn record_child_quest_finished(&mut self, quest_id: i32, node_id: i32) {
        if !self
            .finished_child_quests
            .iter()
            .any(|r| r.quest_id == quest_id && r.node_id == node_id)
        {
            self.finished_child_quests
                .push(ChildQuestRecord { quest_id, node_id });
        }
    }

    pub fn remove_active_quest(&mut self, quest_id: i32) {
        self.active_quests.retain(|aq| aq.quest_id != quest_id);
    }

    pub fn mark_node_finished(&mut self, quest_id: i32, node_id: i32) -> bool {
        let Some(aq) = self.find_active_quest_mut(quest_id) else {
            return false;
        };
        aq.mark_node_inactive(node_id);
        if aq.finished_nodes.contains(&node_id) {
            return false;
        }
        aq.finished_nodes.push(node_id);
        self.record_child_quest_finished(quest_id, node_id);
        true
    }

    pub fn has_var(&self, key: &str) -> bool {
        self.vars.iter().any(|v| v.key == key)
    }

    pub fn get_var_int(&self, key: &str) -> i32 {
        self.vars
            .iter()
            .find(|v| v.key == key)
            .map_or(0, |v| v.int_val)
    }

    pub fn set_var_int(&mut self, key: &str, val: i32) {
        if let Some(v) = self.vars.iter_mut().find(|v| v.key == key) {
            v.int_val = val;
            return;
        }
        self.vars.push(QuestVar {
            key: key.to_string(),
            int_val: val,
            ..Default::default()
        });
    }

    pub fn get_var_bool(&self, key: &str) -> bool {
        self.vars
            .iter()
            .find(|v| v.key == key)
            .is_some_and(|v| v.bool_val)
    }

    pub fn set_var_bool(&mut self, key: &str, val: bool) {
        if let Some(v) = self.vars.iter_mut().find(|v| v.key == key) {
            v.bool_val = val;
            return;
        }
        self.vars.push(QuestVar {
            key: key.to_string(),
            bool_val: val,
            ..Default::default()
        });
    }

    pub fn get_var_str(&self, key: &str) -> &str {
        self.vars
            .iter()
            .find(|v| v.key == key)
            .map_or("", |v| v.str_val.as_str())
    }

    pub fn set_var_str(&mut self, key: &str, val: &str) {
        if let Some(v) = self.vars.iter_mut().find(|v| v.key == key) {
            v.str_val = val.to_string();
            return;
        }
        self.vars.push(QuestVar {
            key: key.to_string(),
            str_val: val.to_string(),
            ..Default::default()
        });
    }

    pub fn set_entity_state(&mut self, entity_id: i64, state_name: &str) {
        if let Some(rec) = self
            .world
            .entity_states
            .iter_mut()
            .find(|r| r.entity_id == entity_id)
        {
            rec.state_name = state_name.to_string();
            return;
        }
        self.world.entity_states.push(EntityStateRecord {
            entity_id,
            state_name: state_name.to_string(),
        });
    }

    pub fn get_entity_state(&self, entity_id: i64) -> Option<&str> {
        self.world
            .entity_states
            .iter()
            .find(|r| r.entity_id == entity_id)
            .map(|r| r.state_name.as_str())
    }

    pub fn set_area_state(&mut self, area_id: i32, state: bool) {
        if let Some(rec) = self
            .env
            .area_states
            .iter_mut()
            .find(|r| r.area_id == area_id)
        {
            rec.state = state;
            return;
        }
        self.env
            .area_states
            .push(AreaStateRecord { area_id, state });
    }

    pub fn set_area_time(&mut self, area_id: i32, hour: i32, minute: i32, locked: bool) {
        if let Some(rec) = self
            .env
            .area_times
            .iter_mut()
            .find(|r| r.area_id == area_id)
        {
            rec.hour = hour;
            rec.minute = minute;
            rec.locked = locked;
            return;
        }
        self.env.area_times.push(AreaTimeRecord {
            area_id,
            hour,
            minute,
            locked,
        });
    }

    pub fn select_quest_branch(&mut self, page_id: i32, branch_id: i32) -> &mut QuestBranchRecord {
        if let Some(pos) = self
            .progress
            .quest_branches
            .iter()
            .position(|r| r.page_id == page_id)
        {
            let rec = &mut self.progress.quest_branches[pos];
            rec.cur_branch = branch_id;
            append_unique_int(&mut rec.complete_branches, branch_id);
            return rec;
        }
        self.progress.quest_branches.push(QuestBranchRecord {
            page_id,
            cur_branch: branch_id,
            complete_branches: vec![branch_id],
        });
        self.progress.quest_branches.last_mut().expect("just pushed")
    }

    pub fn mark_photo_node_completed(&mut self, quest_id: i32, node_id: i32) -> bool {
        if self.has_photo_node_completed(quest_id, node_id) {
            return false;
        }
        self.progress
            .completed_photo_nodes
            .push(ChildQuestRecord { quest_id, node_id });
        true
    }

    pub fn has_photo_node_completed(&self, quest_id: i32, node_id: i32) -> bool {
        self.progress
            .completed_photo_nodes
            .iter()
            .any(|r| r.quest_id == quest_id && r.node_id == node_id)
    }

    pub fn set_hidden_group(
        &mut self,
        group_key: &str,
        entity_ids: &[i32],
        data_layers: &[i32],
        level_plays: &[i32],
    ) {
        if group_key.is_empty() {
            return;
        }
        let _ = self.pop_hidden_group(group_key);
        self.world.hidden_groups.push(HiddenGroupRecord {
            group_key: group_key.to_string(),
            entity_ids: entity_ids.to_vec(),
            data_layers: data_layers.to_vec(),
            level_plays: level_plays.to_vec(),
        });
    }

    pub fn pop_hidden_group(&mut self, group_key: &str) -> Option<HiddenGroupRecord> {
        let pos = self
            .world
            .hidden_groups
            .iter()
            .position(|r| r.group_key == group_key)?;
        Some(self.world.hidden_groups.remove(pos))
    }

    pub fn mark_dungeon_completed(&mut self, dungeon_id: i32) {
        if dungeon_id > 0 {
            append_unique_int(&mut self.completed_dungeons, dungeon_id);
        }
    }

    pub fn mark_level_play_completed(&mut self, level_play_id: i32) {
        if level_play_id > 0 {
            append_unique_int(&mut self.completed_level_plays, level_play_id);
        }
    }

    pub fn set_audio_state(&mut self, group: &str, state: &str) {
        if group.is_empty() {
            return;
        }
        if let Some(rec) = self
            .env
            .audio_states
            .iter_mut()
            .find(|r| r.group == group)
        {
            rec.state = state.to_string();
            return;
        }
        self.env.audio_states.push(AudioStateRecord {
            group: group.to_string(),
            state: state.to_string(),
        });
    }

    pub fn has_unlocked_system(&self, system_id: i32) -> bool {
        self.unlocked_systems.contains(&system_id)
    }

    pub fn unlock_system(&mut self, system_id: i32) -> bool {
        system_id > 0 && append_unique_int(&mut self.unlocked_systems, system_id)
    }

    pub fn has_opened_chest(&self, entity_config_id: i32) -> bool {
        self.opened_chests.contains(&entity_config_id)
    }

    pub fn mark_chest_opened(&mut self, entity_config_id: i32) -> bool {
        entity_config_id > 0 && append_unique_int(&mut self.opened_chests, entity_config_id)
    }

    pub fn alloc_flow_inc_id(&mut self) -> i64 {
        let id = self.next_flow_inc_id;
        self.next_flow_inc_id += 1;
        id
    }
}

pub fn append_unique_int(slice: &mut Vec<i32>, val: i32) -> bool {
    if slice.contains(&val) {
        return false;
    }
    slice.push(val);
    true
}

pub fn remove_int_value(slice: &mut Vec<i32>, val: i32) -> bool {
    let before = slice.len();
    slice.retain(|&v| v != val);
    slice.len() != before
}

pub fn append_unique_str(slice: &mut Vec<String>, val: &str) -> bool {
    if val.is_empty() || slice.iter().any(|s| s == val) {
        return false;
    }
    slice.push(val.to_string());
    true
}

pub fn remove_str_value(slice: &mut Vec<String>, val: &str) -> bool {
    let before = slice.len();
    slice.retain(|s| s != val);
    slice.len() != before
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zon_codec;

    #[test]
    fn quest_info_parses_legacy_zon_and_handles_mutations() {
        let legacy_zon = r#".{
            .finished_quests = .{ 139000025 },
            .active_quests = .{},
            .tracked_quest_id = 139000025,
        }"#;
        let mut info: QuestInfo = zon_codec::from_zon_str(legacy_zon).unwrap();
        assert!(info.has_finished_quest(139000025));
        assert_eq!(info.env.weather_id, 0);
        assert!(info.world.loaded_datalayers.is_empty());

        info.set_var_str("dialog_choice", "A");
        info.set_var_str("dialog_choice", "B");
        assert_eq!(info.get_var_str("dialog_choice"), "B");

        info.set_entity_state(122006411, "关卡.Common.状态.常态");
        assert_eq!(
            info.get_entity_state(122006411),
            Some("关卡.Common.状态.常态")
        );

        info.set_hidden_group("q_125_a", &[101, 102], &[3], &[]);
        assert_eq!(info.world.hidden_groups.len(), 1);
        assert!(info.pop_hidden_group("q_125_a").is_some());
        assert!(info.world.hidden_groups.is_empty());

        info.select_quest_branch(1000, 1);
        assert_eq!(info.progress.quest_branches[0].cur_branch, 1);
        info.set_area_state(25, true);
        info.set_area_time(25, 17, 40, true);
        assert!(info.mark_photo_node_completed(1, 2));
    }
}
