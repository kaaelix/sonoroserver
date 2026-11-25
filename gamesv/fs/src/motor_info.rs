use serde::{Deserialize, Serialize};
use sonoro_gamesv_data::Assets;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct TechNode {
    pub id: i32,
    pub tree_type: i32,
    pub level: i32,
    pub unlocked: bool,
    pub current: i32,
    pub target: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct TaskNode {
    pub id: i32,
    pub tree_type: i32,
    pub r#type: i32,
    pub current: i32,
    pub target: i32,
    pub rewarded: i32,
    pub wait_reward: i32,
    pub max_reward: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Preset {
    pub id: i32,
    pub name: String,
    pub skin: i32,
    pub stickers: Vec<i32>,
    pub decorations: Vec<i32>,
    pub frame: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct MotorInfo {
    pub level: i32,
    pub exp: i32,
    pub rewarded_max_level: i32,
    pub tree_in_use: i32,
    pub scene_in_use: i32,
    pub daily_exp_gain: i32,
    pub daily_exp_limit: i32,
    pub tech_nodes: Vec<TechNode>,
    pub task_nodes: Vec<TaskNode>,
    pub owned_skins: Vec<i32>,
    pub owned_stickers: Vec<i32>,
    pub owned_decorations: Vec<i32>,
    pub owned_frames: Vec<i32>,
    pub equipped_skin: i32,
    pub equipped_frame: i32,
    pub equipped_stickers: Vec<i32>,
    pub equipped_decorations: Vec<i32>,
    pub presets: Vec<Preset>,
    pub next_preset_id: i32,
}

impl Default for MotorInfo {
    fn default() -> Self {
        Self {
            level: 1,
            exp: 0,
            rewarded_max_level: 0,
            tree_in_use: 0,
            scene_in_use: 0,
            daily_exp_gain: 0,
            daily_exp_limit: 0,
            tech_nodes: Vec::new(),
            task_nodes: Vec::new(),
            owned_skins: Vec::new(),
            owned_stickers: Vec::new(),
            owned_decorations: Vec::new(),
            owned_frames: Vec::new(),
            equipped_skin: 0,
            equipped_frame: 0,
            equipped_stickers: Vec::new(),
            equipped_decorations: Vec::new(),
            presets: Vec::new(),
            next_preset_id: 1,
        }
    }
}

impl MotorInfo {
    pub const DATA_PATH: &'static str = "motor_info";

    pub fn default_scene_id(assets: &Assets) -> i32 {
        assets
            .tables
            .motor_scene
            .items
            .iter()
            .find(|s| s.DefaultFlag)
            .map_or(0, |s| s.Id)
    }

    pub fn is_empty(&self) -> bool {
        self.scene_in_use == 0
            && self.tech_nodes.is_empty()
            && self.task_nodes.is_empty()
            && self.owned_skins.is_empty()
            && self.owned_stickers.is_empty()
            && self.owned_decorations.is_empty()
            && self.owned_frames.is_empty()
    }

    pub fn add_defaults(&mut self, assets: &Assets) {
        self.level = 1;
        self.exp = 0;
        self.rewarded_max_level = 0;
        self.tree_in_use = assets
            .tables
            .motor_tech_tree
            .items
            .first()
            .map_or(0, |t| t.Id);
        self.scene_in_use = Self::default_scene_id(assets);

        self.tech_nodes.clear();
        for tech in &assets.tables.motor_tech.items {
            let level = (tech.InitLevel.max(0) as usize).min(tech.TechLv.len()) as i32;
            self.tech_nodes.push(TechNode {
                id: tech.Id,
                tree_type: tech.TreeType,
                level,
                unlocked: level > 0,
                current: level,
                target: tech.TechLv.len() as i32,
            });
        }

        self.task_nodes.clear();
        for task in &assets.tables.motor_task.items {
            self.task_nodes.push(TaskNode {
                id: task.Id,
                tree_type: task.TreeType,
                r#type: 1,
                current: 0,
                target: 1,
                rewarded: 0,
                wait_reward: 0,
                max_reward: task.RewardTpCount.max(1),
            });
        }

        self.owned_skins.clear();
        for skin in &assets.tables.motor_skin.items {
            push_unique_nz(&mut self.owned_skins, skin.Id);
            if skin.DefaultFlag {
                self.equipped_skin = skin.Id;
            }
        }

        self.owned_stickers.clear();
        for preset in &assets.tables.motor_load_project.items {
            for &id in &preset.Sticker {
                push_unique_nz(&mut self.owned_stickers, id);
            }
        }
        for preview in &assets.tables.motor_general_preview.items {
            for &id in &preview.Sticker {
                push_unique_nz(&mut self.owned_stickers, id);
            }
        }
        for ip in &assets.tables.motor_decal_ip.items {
            for &id in &ip.IpStickerList {
                push_unique_nz(&mut self.owned_stickers, id);
            }
        }
        for ip in &assets.tables.motor_linkage_ip.items {
            for &id in &ip.IpStickerList {
                push_unique_nz(&mut self.owned_stickers, id);
            }
        }

        self.owned_decorations.clear();
        for dec in &assets.tables.motor_decorations.items {
            push_unique_nz(&mut self.owned_decorations, dec.Id);
        }

        self.owned_frames.clear();
        for frame in &assets.tables.motor_frame.items {
            push_unique_nz(&mut self.owned_frames, frame.Id);
            if frame.DefaultFlag {
                self.equipped_frame = frame.Id;
            }
        }

        if self.equipped_skin == 0 && !self.owned_skins.is_empty() {
            self.equipped_skin = self.owned_skins[0];
        }
        if self.equipped_frame == 0 && !self.owned_frames.is_empty() {
            self.equipped_frame = self.owned_frames[0];
        }
        self.equipped_stickers = vec![0; assets.tables.motor_sticker_part.items.len()];
        self.equipped_decorations = vec![0; assets.tables.motor_decorations_part.items.len()];
    }

    pub fn has_owned_id(items: &[i32], id: i32) -> bool {
        items.contains(&id)
    }
}

fn push_unique_nz(list: &mut Vec<i32>, id: i32) {
    if id != 0 && !list.contains(&id) {
        list.push(id);
    }
}
