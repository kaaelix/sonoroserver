use std::collections::{BTreeMap, HashMap, HashSet};
use std::io;

use sonoro_common::fs::FileSystem;
use sonoro_gamesv_data::Assets;
use sonoro_gamesv_data::tables::buff::Buff;
use sonoro_gamesv_data::tables::phantom_summon_group::PhantomSummonGroup;
use sonoro_gamesv_data::zon_config::GameConfig;
use sonoro_gamesv_fs::echo_info::{
    self, CalabashInfo, EchoInfo, PresetInfo, RoleEquip,
};
use sonoro_gamesv_fs::file_util::load_zon;
use sonoro_gamesv_fs::special_item_incr::{self, TRIAL_INCR_MIN};
use sonoro_proto::pb::VisionSkillInformation;

use crate::component::comp_util;

#[derive(Debug, Clone, Copy)]
struct MainEchoRuntime {
    skill_id: i32,
    summon_id: i32,
    _monster_id: i32,
    base_monster_id: i32,
    level: i32,
}

#[derive(Debug, Clone, Copy)]
struct ComboEchoRuntime {
    inc_id: i32,
    runtime: MainEchoRuntime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EquippedEcho {
    pub inc_id: i32,
    pub role_id: i32,
    pub pos: i32,
}

#[derive(Debug, Default)]
pub struct PlayerEchoComponent {
    pub player_id: i32,
    pub echo_map: BTreeMap<i32, EchoInfo>,
    pub preset_info: PresetInfo,
    pub calabash_info: CalabashInfo,
}

impl PlayerEchoComponent {
    pub fn init(fs: &FileSystem, assets: &Assets, player_id: i32) -> io::Result<Self> {
        let mut echo_map = BTreeMap::new();
        let preset_info = load_zon::<PresetInfo>(
            fs,
            &format!("player/{player_id}/{}", EchoInfo::PRESET_DATA_PATH),
        )?
        .unwrap_or_default();

        let calabash_path = format!("player/{player_id}/{}", EchoInfo::CALABASH_DATA_PATH);
        let (mut calabash_info, mut calabash_dirty) =
            match load_zon::<CalabashInfo>(fs, &calabash_path)? {
                Some(info) => (info, false),
                None => (CalabashInfo::default(), true),
            };

        let dir_path = format!("player/{player_id}/{}", EchoInfo::DATA_DIR);
        if let Some(entries) = fs.read_dir(&dir_path)? {
            for entry in entries {
                let Some(name) = comp_util::entry_file_name(&entry) else {
                    continue;
                };
                let Ok(unique_id) = name.parse::<i32>() else {
                    continue;
                };
                let item_path = format!("player/{player_id}/{}/{unique_id}", EchoInfo::DATA_DIR);
                match load_zon::<EchoInfo>(fs, &item_path) {
                    Ok(Some(item)) => {
                        echo_map.insert(unique_id, item);
                    }
                    Ok(None) => {}
                    Err(err) => {
                        tracing::error!(unique_id, %err, "failed to load EchoInfo");
                    }
                }
            }
        }

        special_item_incr::ensure_at_least(
            fs,
            player_id,
            echo_info::next_after_preset_groups(&preset_info.groups),
        )?;

        if GameConfig::embedded().seed_default_echoes && echo_map.is_empty() {
            let start_incr = special_item_incr::current(fs, player_id)?;
            EchoInfo::add_defaults(assets, &mut echo_map, start_incr);
            Self::save_all(fs, player_id, &echo_map)?;
        }

        special_item_incr::ensure_at_least(
            fs,
            player_id,
            special_item_incr::next_after_map(&echo_map)
                .max(echo_info::next_after_preset_groups(&preset_info.groups)),
        )?;

        for echo in echo_map.values() {
            let Some(cfg) = assets.tables.phantom_item.get_data_by_id(&echo.id) else {
                continue;
            };
            if calabash_info.upsert_discovery(cfg.MonsterId, cfg.QualityId, true) {
                calabash_dirty = true;
            }
        }
        if calabash_dirty {
            Self::save_calabash(fs, player_id, &calabash_info)?;
        }

        Ok(Self {
            player_id,
            echo_map,
            preset_info,
            calabash_info,
        })
    }

    pub fn save_all(
        fs: &FileSystem,
        player_id: i32,
        echo_map: &BTreeMap<i32, EchoInfo>,
    ) -> io::Result<()> {
        let dir_path = format!("player/{player_id}/{}", EchoInfo::DATA_DIR);
        if let Some(entries) = fs.read_dir(&dir_path)? {
            for entry in entries {
                let Some(name) = comp_util::entry_file_name(&entry) else {
                    continue;
                };
                let Ok(unique_id) = name.parse::<i32>() else {
                    continue;
                };
                if unique_id >= TRIAL_INCR_MIN || echo_map.contains_key(&unique_id) {
                    continue;
                }
                fs.delete_file(&format!(
                    "player/{player_id}/{}/{unique_id}",
                    EchoInfo::DATA_DIR
                ))?;
            }
        }

        for (&unique_id, echo) in echo_map {
            if unique_id >= TRIAL_INCR_MIN {
                continue;
            }
            comp_util::save_struct(
                fs,
                echo,
                &format!("player/{player_id}/{}/{unique_id}", EchoInfo::DATA_DIR),
            )?;
        }
        Ok(())
    }

    pub fn delete_echo_file(fs: &FileSystem, player_id: i32, unique_id: i32) -> io::Result<()> {
        if unique_id >= TRIAL_INCR_MIN {
            return Ok(());
        }
        fs.delete_file(&format!(
            "player/{player_id}/{}/{unique_id}",
            EchoInfo::DATA_DIR
        ))
    }

    pub fn save_presets(
        fs: &FileSystem,
        player_id: i32,
        preset_info: &PresetInfo,
    ) -> io::Result<()> {
        comp_util::save_struct(
            fs,
            preset_info,
            &format!("player/{player_id}/{}", EchoInfo::PRESET_DATA_PATH),
        )
    }

    pub fn save_calabash(
        fs: &FileSystem,
        player_id: i32,
        calabash_info: &CalabashInfo,
    ) -> io::Result<()> {
        comp_util::save_struct(
            fs,
            calabash_info,
            &format!("player/{player_id}/{}", EchoInfo::CALABASH_DATA_PATH),
        )
    }

    pub fn build_vision_skills(
        &self,
        assets: &Assets,
        role_id: i32,
        vision_entity_id: i64,
        combo_vision_entity_id: i64,
        explore_skill_id: i32,
    ) -> Vec<VisionSkillInformation> {
        let mut skills = Vec::new();
        let equip = self.role_equip(role_id);
        let combo = self.combo_echo_runtime(assets, role_id);
        let combo_inc_id = combo.map_or(0, |p| p.inc_id);
        for (index, &inc_id) in equip.slots.iter().enumerate() {
            let skill_vision_entity_id = if index == 0 {
                vision_entity_id
            } else if inc_id != 0 && inc_id == combo_inc_id {
                combo_vision_entity_id
            } else {
                0
            };
            if let Some(skill) =
                self.echo_vision_skill(assets, inc_id, index as i32, skill_vision_entity_id)
            {
                skills.push(skill);
            }
        }
        if explore_skill_id != 0 {
            skills.push(VisionSkillInformation {
                SkillId: explore_skill_id,
                Index: 5,
                ..Default::default()
            });
        }
        skills
    }

    pub fn main_echo_skill_id(&self, assets: &Assets, role_id: i32) -> Option<i32> {
        let runtime = self.main_echo_runtime(assets, role_id)?;
        (runtime.skill_id != 0).then_some(runtime.skill_id)
    }

    pub fn main_echo_buff_effects(&self, assets: &Assets, role_id: i32) -> Vec<i64> {
        let Some(skill_id) = self.main_echo_skill_id(assets, role_id) else {
            return Vec::new();
        };
        assets
            .tables
            .phantom_skill
            .get_data_by_id(&skill_id)
            .map_or_else(Vec::new, |s| s.BuffEffects.clone())
    }

    pub fn active_echo_buff_effects(&self, assets: &Assets, role_id: i32) -> Vec<i64> {
        let mut ids = Vec::new();
        let equip = self.role_equip(role_id);
        for inc_id in equip.slots {
            let Some(runtime) = self.echo_runtime(assets, inc_id) else {
                continue;
            };
            if let Some(buff_id) = echo_marker_buff_effect(assets, runtime.skill_id) {
                push_unique_i64(&mut ids, buff_id);
            }
        }
        for buff_id in self.main_echo_buff_effects(assets, role_id) {
            push_unique_i64(&mut ids, buff_id);
        }
        for fetter_id in self.active_fetter_ids(assets, role_id) {
            let Some(fetter) = assets.tables.phantom_fetter.get_data_by_id(&fetter_id) else {
                continue;
            };
            for &buff_id in &fetter.BuffIds {
                push_unique_i64(&mut ids, buff_id);
            }
        }
        ids
    }

    pub fn main_echo_summon_id(&self, assets: &Assets, role_id: i32) -> Option<i32> {
        let runtime = self.main_echo_runtime(assets, role_id)?;
        (runtime.summon_id != 0).then_some(runtime.summon_id)
    }

    pub fn combo_echo_summon_id(&self, assets: &Assets, role_id: i32) -> Option<i32> {
        let combo = self.combo_echo_runtime(assets, role_id)?;
        (combo.runtime.summon_id != 0).then_some(combo.runtime.summon_id)
    }

    fn echo_vision_skill(
        &self,
        assets: &Assets,
        inc_id: i32,
        index: i32,
        vision_entity_id: i64,
    ) -> Option<VisionSkillInformation> {
        if inc_id == 0 {
            return None;
        }
        let runtime = self.echo_runtime(assets, inc_id)?;
        if runtime.skill_id == 0 {
            return None;
        }
        Some(VisionSkillInformation {
            SkillId: runtime.skill_id,
            Level: runtime.level,
            Quality: 5,
            VisionEntityId: vision_entity_id,
            Index: index,
        })
    }

    fn main_echo_runtime(&self, assets: &Assets, role_id: i32) -> Option<MainEchoRuntime> {
        let equip = self.role_equip(role_id);
        self.echo_runtime(assets, equip.slots[0])
    }

    fn echo_runtime(&self, assets: &Assets, inc_id: i32) -> Option<MainEchoRuntime> {
        if inc_id == 0 {
            return None;
        }
        let echo = self.echo_map.get(&inc_id)?;
        let base_item = assets.tables.phantom_item.get_data_by_id(&echo.id)?;
        let item = if echo.skin_id != 0
            && let Some(skin_item) = assets.tables.phantom_item.get_data_by_id(&echo.skin_id)
            && skin_item.ParentMonsterId == base_item.MonsterId
        {
            skin_item
        } else {
            base_item
        };
        Some(MainEchoRuntime {
            skill_id: item.SkillId,
            summon_id: if item.MeshId != 0 {
                item.MeshId
            } else {
                item.SkillId
            },
            _monster_id: item.MonsterId,
            base_monster_id: if item.ParentMonsterId != 0 {
                item.ParentMonsterId
            } else {
                item.MonsterId
            },
            level: echo.level,
        })
    }

    fn combo_echo_runtime(&self, assets: &Assets, role_id: i32) -> Option<ComboEchoRuntime> {
        let equip = self.role_equip(role_id);
        let main_runtime = self.echo_runtime(assets, equip.slots[0])?;
        if main_runtime.base_monster_id == 0 {
            return None;
        }
        let main_group = phantom_summon_group(assets, main_runtime.base_monster_id)?;
        for &inc_id in &equip.slots[1..] {
            let Some(runtime) = self.echo_runtime(assets, inc_id) else {
                continue;
            };
            if runtime.summon_id == 0 || runtime.base_monster_id == main_runtime.base_monster_id {
                continue;
            }
            if main_group.GroupMonsterIds.contains(&runtime.base_monster_id) {
                return Some(ComboEchoRuntime { inc_id, runtime });
            }
        }
        None
    }

    pub fn role_equip(&self, role_id: i32) -> RoleEquip {
        let mut equip = RoleEquip {
            role_id,
            slots: [0; 5],
        };
        for (&inc_id, echo) in &self.echo_map {
            if echo.role_id != Some(role_id) {
                continue;
            }
            if echo.equipped_pos >= 0 && (echo.equipped_pos as usize) < equip.slots.len() {
                equip.slots[echo.equipped_pos as usize] = inc_id;
            }
        }
        equip
    }

    pub fn equipped_echo(&self, role_id: i32, pos: i32) -> Option<EquippedEcho> {
        self.echo_map.iter().find_map(|(&inc_id, echo)| {
            if echo.role_id == Some(role_id) && echo.equipped_pos == pos {
                Some(EquippedEcho {
                    inc_id,
                    role_id,
                    pos,
                })
            } else {
                None
            }
        })
    }

    pub fn equipped_echo_by_incr_id(&self, inc_id: i32) -> Option<EquippedEcho> {
        let echo = self.echo_map.get(&inc_id)?;
        let role_id = echo.role_id?;
        if !(0..5).contains(&echo.equipped_pos) {
            return None;
        }
        Some(EquippedEcho {
            inc_id,
            role_id,
            pos: echo.equipped_pos,
        })
    }

    pub fn active_fetter_ids(&self, assets: &Assets, role_id: i32) -> Vec<i32> {
        let mut ids = Vec::new();
        let equip = self.role_equip(role_id);
        let mut counts: HashMap<i32, i32> = HashMap::new();
        let mut seen: HashSet<i64> = HashSet::new();

        for inc_id in equip.slots {
            if inc_id == 0 {
                continue;
            }
            let Some(echo) = self.echo_map.get(&inc_id) else {
                continue;
            };
            if echo.fetter_group_id == 0 {
                continue;
            }
            let Some(item) = assets.tables.phantom_item.get_data_by_id(&echo.id) else {
                continue;
            };
            let seen_key = i64::from(echo.fetter_group_id) * 10_000_000 + i64::from(item.MonsterId);
            if !seen.insert(seen_key) {
                continue;
            }
            *counts.entry(echo.fetter_group_id).or_default() += 1;
        }

        for (group_id, count) in counts {
            let Some(group) = assets.tables.phantom_fetter_group.get_data_by_id(&group_id) else {
                continue;
            };
            for (&req, &fetter_id) in group.FetterMap.iter() {
                if count >= req {
                    ids.push(fetter_id);
                }
            }
        }
        ids
    }

    pub fn active_echo_tags(
        &self,
        assets: &Assets,
        role_id: i32,
        role_tags: &[String],
    ) -> Vec<String> {
        let mut tags: Vec<String> = Vec::new();
        for tag in role_tags {
            if !contains_tag(&tags, tag) {
                tags.push(tag.clone());
            }
        }
        let equip = self.role_equip(role_id);
        for inc_id in equip.slots {
            let Some(runtime) = self.echo_runtime(assets, inc_id) else {
                continue;
            };
            let Some(buff_id) = echo_marker_buff_effect(assets, runtime.skill_id) else {
                continue;
            };
            let Some(buff) = assets.tables.buff.get_data_by_id(&buff_id) else {
                continue;
            };
            for tag in &buff.GrantedTags {
                if !contains_tag(&tags, tag) {
                    tags.push(tag.clone());
                }
            }
        }
        tags
    }

    pub fn buff_tag_requirements_met(buff: &Buff, active_tags: &[String]) -> bool {
        buff.OngoingTagRequirements
            .iter()
            .all(|t| contains_tag(active_tags, t))
            && !buff
                .OngoingTagIgnores
                .iter()
                .any(|t| contains_tag(active_tags, t))
    }
}

fn phantom_summon_group(assets: &Assets, monster_id: i32) -> Option<PhantomSummonGroup> {
    assets
        .tables
        .phantom_summon_group
        .items
        .iter()
        .find(|g| g.GroupMonsterIds.contains(&monster_id))
        .cloned()
}

fn contains_tag(tags: &[String], needle: &str) -> bool {
    tags.iter().any(|tag| tag_matches(tag, needle))
}

fn tag_matches(tag: &str, requirement: &str) -> bool {
    tag == requirement
        || (tag.len() > requirement.len()
            && tag.as_bytes()[requirement.len()] == b'.'
            && tag.starts_with(requirement))
}

fn echo_marker_buff_effect(assets: &Assets, skill_id: i32) -> Option<i64> {
    if skill_id == 0 {
        return None;
    }
    let marker_id = i64::from(skill_id) * 1000 + 1;
    let marker = assets.tables.buff.get_data_by_id(&marker_id)?;
    if marker.GrantedTags.is_empty() || marker.ExtraEffectID != 0 {
        return None;
    }
    Some(marker_id)
}

fn push_unique_i64(list: &mut Vec<i64>, value: i64) {
    if !list.contains(&value) {
        list.push(value);
    }
}
