use std::collections::{BTreeMap, HashSet};

use sonoro_gamesv_data::Assets;
use sonoro_gamesv_data::tables::PropValue;
use sonoro_gamesv_data::tables::buff::Buff;
use sonoro_gamesv_data::tables::weapon_property_growth::WeaponPropertyGrowth;
use sonoro_gamesv_fs::role_info::RoleInfo;
use sonoro_proto::enums::EAttributeType;
use sonoro_proto::wire::PbEnum;

use crate::component::player::{PlayerEchoComponent, PlayerWeaponComponent};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SourceType {
    Weapon,
    Echo,
    Skill,
    Chain,
    Fetter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SourceKey {
    pub source_type: SourceType,
    pub id: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DisplayMode {
    Hidden,
    Base,
    Add,
}

#[derive(Debug, Clone, Copy)]
struct Contribution {
    attr_index: usize,
    is_ratio: bool,
    value: i32,
    ratio: f64,
    display_mode: DisplayMode,
}

#[derive(Debug, Clone, Default)]
pub struct Snapshot {
    pub base_props: Vec<i32>,
    pub add_props: Vec<i32>,
}

#[derive(Debug, Clone, Default)]
pub struct DisplayProps {
    pub base_props: BTreeMap<i32, i32>,
    pub add_props: BTreeMap<i32, i32>,
}

#[derive(Debug, Clone, Default)]
pub struct RoleEquipment {
    original_base_values: Vec<i32>,
    pub base_props: Vec<i32>,
    pub add_props: Vec<i32>,
    equipment_contributions: BTreeMap<SourceKey, Vec<Contribution>>,
}

impl RoleEquipment {
    pub fn build(
        assets: &Assets,
        role_id: i32,
        role_info: &RoleInfo,
        weapon_comp: &PlayerWeaponComponent,
        echo_comp: &PlayerEchoComponent,
    ) -> Self {
        let attr_count = EAttributeType::MAX.to_i32() as usize;
        let mut original_base_values = vec![0i32; attr_count];
        let copy_len = role_info.base_prop.len().min(attr_count);
        original_base_values[..copy_len].copy_from_slice(&role_info.base_prop[..copy_len]);

        let mut equipment = Self {
            original_base_values,
            base_props: vec![0i32; attr_count],
            add_props: vec![0i32; attr_count],
            equipment_contributions: BTreeMap::new(),
        };

        equipment.add_weapon(assets, role_info, weapon_comp);
        equipment.add_skills(assets, role_info);
        equipment.add_chains(assets, role_id, role_info);
        equipment.add_echoes(assets, role_id, role_info, weapon_comp, echo_comp);
        equipment.recalculate();
        equipment
    }

    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            base_props: self.base_props.clone(),
            add_props: self.add_props.clone(),
        }
    }

    pub fn phantom_display_props(&self) -> DisplayProps {
        let mut display = DisplayProps::default();
        for contributions in self.equipment_contributions.values() {
            for contribution in contributions {
                if contribution.display_mode == DisplayMode::Hidden {
                    continue;
                }
                let attr_id = contribution.attr_index as i32;
                let value = value_for_base(*contribution, &self.base_props);
                match contribution.display_mode {
                    DisplayMode::Hidden => {}
                    DisplayMode::Base => {
                        if contribution.is_ratio {
                            add_display_prop(&mut display.add_props, attr_id, value);
                        } else {
                            add_display_prop(&mut display.base_props, attr_id, value);
                        }
                    }
                    DisplayMode::Add => {
                        add_display_prop(&mut display.add_props, attr_id, value);
                    }
                }
            }
        }
        display
    }

    pub fn build_role_tags(
        assets: &Assets,
        role_id: i32,
        role_info: &RoleInfo,
        weapon_comp: &PlayerWeaponComponent,
    ) -> Vec<String> {
        let mut tags = Vec::new();
        let Some(weapon) = weapon_comp.weapon_map.get(&role_info.weapon) else {
            return tags;
        };
        let weapon_info = sonoro_gamesv_data::data_tables::WeaponEquipInfo {
            id: weapon.id,
            reson_level: weapon.reson_level,
        };
        let Ok(buffs) = assets
            .tables
            .get_role_auto_buffs(role_id, weapon_info, role_info.skill_branch)
        else {
            return tags;
        };
        for entry in buffs {
            if !entry.is_active {
                continue;
            }
            let Some(buff) = assets.tables.buff.get_data_by_id(&entry.id) else {
                continue;
            };
            for tag in &buff.GrantedTags {
                if !tags.iter().any(|existing| existing == tag) {
                    tags.push(tag.clone());
                }
            }
        }
        tags
    }

    fn add_weapon(
        &mut self,
        assets: &Assets,
        role_info: &RoleInfo,
        weapon_comp: &PlayerWeaponComponent,
    ) {
        let Some(weapon) = weapon_comp.weapon_map.get(&role_info.weapon) else {
            return;
        };
        let Some(config) = assets.tables.weapon_conf.get_data_by_id(&weapon.id) else {
            return;
        };
        self.add_weapon_prop(
            assets,
            role_info.weapon,
            &config.FirstPropId,
            config.FirstCurve,
            weapon.level,
            weapon.breach,
        );
        self.add_weapon_prop(
            assets,
            role_info.weapon,
            &config.SecondPropId,
            config.SecondCurve,
            weapon.level,
            weapon.breach,
        );
        self.add_weapon_base_attributes(assets, role_info.weapon, weapon.id, weapon.reson_level);
    }

    fn add_weapon_prop(
        &mut self,
        assets: &Assets,
        weapon_incr_id: i32,
        prop: &PropValue,
        curve_id: i32,
        level: i32,
        breach: i32,
    ) {
        if prop.Id == 0 {
            return;
        }
        let Some(growth) = weapon_growth(assets, curve_id, level, breach) else {
            return;
        };
        let scaled = prop.Value * f64::from(growth.CurveValue) / 10000.0;
        if let Some(c) = contribution_from_prop_value(
            prop.Id,
            scaled,
            prop.IsRatio,
            DisplayMode::Hidden,
        ) {
            self.add_contribution(
                SourceKey {
                    source_type: SourceType::Weapon,
                    id: i64::from(weapon_incr_id),
                },
                c,
            );
        }
    }

    fn add_weapon_base_attributes(
        &mut self,
        assets: &Assets,
        weapon_incr_id: i32,
        weapon_id: i32,
        reson_level: i32,
    ) {
        let Some(config) = assets
            .tables
            .weapon_base_attributes
            .get_data_by_id(&weapon_id)
        else {
            return;
        };
        let Some(&raw_value) = config.Values.get(&reson_level) else {
            return;
        };
        let value = if config.IsRatio {
            f64::from(raw_value) / 10000.0
        } else {
            f64::from(raw_value)
        };
        for &attribute in &config.Attributes {
            if let Some(c) = contribution_from_prop_value(
                attribute,
                value,
                config.IsRatio,
                DisplayMode::Hidden,
            ) {
                self.add_contribution(
                    SourceKey {
                        source_type: SourceType::Weapon,
                        id: i64::from(weapon_incr_id),
                    },
                    c,
                );
            }
        }
    }

    fn add_skills(&mut self, assets: &Assets, role_info: &RoleInfo) {
        for state in &role_info.skill_node_state {
            if !state.active {
                continue;
            }
            let Some(node) = assets.tables.skill_tree.get_data_by_id(&state.node_id) else {
                continue;
            };
            for prop in &node.Property {
                if let Some(c) = contribution_from_prop_value(
                    prop.Id,
                    prop.Value,
                    prop.IsRatio,
                    DisplayMode::Hidden,
                ) {
                    self.add_contribution(
                        SourceKey {
                            source_type: SourceType::Skill,
                            id: i64::from(state.node_id),
                        },
                        c,
                    );
                }
            }
        }
    }

    fn add_chains(&mut self, assets: &Assets, role_id: i32, role_info: &RoleInfo) {
        let Some(role_config) = assets.tables.resolve_base_role_info(role_id) else {
            return;
        };
        for chain in &assets.tables.resonant_chain.items {
            if chain.GroupId != role_config.ResonantChainGroupId
                || chain.GroupIndex > role_info.resonant_chain_group_index
            {
                continue;
            }
            for prop in &chain.AddProp {
                if let Some(c) = contribution_from_prop_value(
                    prop.Id,
                    prop.Value,
                    prop.IsRatio,
                    DisplayMode::Hidden,
                ) {
                    self.add_contribution(
                        SourceKey {
                            source_type: SourceType::Chain,
                            id: i64::from(chain.Id),
                        },
                        c,
                    );
                }
            }
        }
    }

    fn add_echoes(
        &mut self,
        assets: &Assets,
        role_id: i32,
        role_info: &RoleInfo,
        weapon_comp: &PlayerWeaponComponent,
        echo_comp: &PlayerEchoComponent,
    ) {
        let equip = echo_comp.role_equip(role_id);
        for &inc_id in &equip.slots {
            if inc_id == 0 {
                continue;
            }
            let Some(echo) = echo_comp.echo_map.get(&inc_id) else {
                continue;
            };
            for prop in &echo.main_prop {
                let c = if let Some(config) = assets
                    .tables
                    .phantom_main_prop_item
                    .get_data_by_id(&prop.id)
                {
                    contribution_from_echo_prop(
                        config.PropId,
                        config.AddType,
                        prop.value,
                        DisplayMode::Base,
                    )
                } else if let Some(tppi) = assets
                    .tables
                    .trial_phantom_prop_item
                    .get_data_by_id(&prop.id)
                {
                    contribution_from_echo_prop(
                        tppi.Prop.Id,
                        if tppi.Prop.IsRatio { 2 } else { 1 },
                        prop.value,
                        DisplayMode::Base,
                    )
                } else {
                    contribution_from_echo_prop(prop.id, 1, prop.value, DisplayMode::Base)
                };
                if let Some(c) = c {
                    self.add_contribution(
                        SourceKey {
                            source_type: SourceType::Echo,
                            id: i64::from(inc_id),
                        },
                        c,
                    );
                }
            }
            for prop in &echo.sub_prop {
                let c = if let Some(config) =
                    assets.tables.phantom_sub_property.get_data_by_id(&prop.id)
                {
                    contribution_from_echo_prop(
                        config.PropId,
                        config.AddType,
                        prop.value,
                        DisplayMode::Base,
                    )
                } else if let Some(tppi) = assets
                    .tables
                    .trial_phantom_prop_item
                    .get_data_by_id(&prop.id)
                {
                    contribution_from_echo_prop(
                        tppi.Prop.Id,
                        if tppi.Prop.IsRatio { 2 } else { 1 },
                        prop.value,
                        DisplayMode::Base,
                    )
                } else {
                    None
                };
                if let Some(c) = c {
                    self.add_contribution(
                        SourceKey {
                            source_type: SourceType::Echo,
                            id: i64::from(inc_id),
                        },
                        c,
                    );
                }
            }
        }

        let role_tags = Self::build_role_tags(assets, role_id, role_info, weapon_comp);
        let active_tags = echo_comp.active_echo_tags(assets, role_id, &role_tags);
        let mut seen_buffs = HashSet::new();

        for (index, &inc_id) in equip.slots.iter().enumerate() {
            if inc_id == 0 {
                continue;
            }
            if let Some(buff_id) = echo_marker_buff_effect(assets, echo_comp, inc_id) {
                self.add_buff_contribution(
                    assets,
                    SourceKey {
                        source_type: SourceType::Echo,
                        id: i64::from(inc_id),
                    },
                    buff_id,
                    &active_tags,
                    &mut seen_buffs,
                );
            }
            if index == 0 {
                for buff_id in echo_comp.main_echo_buff_effects(assets, role_id) {
                    self.add_buff_contribution(
                        assets,
                        SourceKey {
                            source_type: SourceType::Echo,
                            id: i64::from(inc_id),
                        },
                        buff_id,
                        &active_tags,
                        &mut seen_buffs,
                    );
                }
            }
        }

        for fetter_id in echo_comp.active_fetter_ids(assets, role_id) {
            let Some(fetter) = assets.tables.phantom_fetter.get_data_by_id(&fetter_id) else {
                continue;
            };
            for prop in &fetter.AddProp {
                if let Some(c) = contribution_from_prop_value(
                    prop.Id,
                    prop.Value,
                    prop.IsRatio,
                    DisplayMode::Base,
                ) {
                    self.add_contribution(
                        SourceKey {
                            source_type: SourceType::Fetter,
                            id: i64::from(fetter_id),
                        },
                        c,
                    );
                }
            }
            for &buff_id in &fetter.BuffIds {
                self.add_buff_contribution(
                    assets,
                    SourceKey {
                        source_type: SourceType::Fetter,
                        id: i64::from(fetter_id),
                    },
                    buff_id,
                    &active_tags,
                    &mut seen_buffs,
                );
            }
        }
    }

    fn add_buff_contribution(
        &mut self,
        assets: &Assets,
        key: SourceKey,
        buff_id: i64,
        active_tags: &[String],
        seen_buffs: &mut HashSet<i64>,
    ) {
        if seen_buffs.contains(&buff_id) {
            return;
        }
        let Some(buff) = assets.tables.buff.get_data_by_id(&buff_id) else {
            return;
        };
        if !PlayerEchoComponent::buff_tag_requirements_met(&buff, active_tags) {
            return;
        }
        let Some(contribution) = contribution_from_buff(&buff, DisplayMode::Add) else {
            return;
        };
        seen_buffs.insert(buff_id);
        self.add_contribution(key, contribution);
    }

    fn add_contribution(&mut self, key: SourceKey, contribution: Contribution) {
        self.equipment_contributions
            .entry(key)
            .or_default()
            .push(contribution);
    }

    fn recalculate(&mut self) {
        self.base_props
            .copy_from_slice(&self.original_base_values);
        self.add_props.fill(0);

        let weapon_contributions: Vec<(usize, i32)> = self
            .equipment_contributions
            .iter()
            .filter(|(k, _)| k.source_type == SourceType::Weapon)
            .flat_map(|(_, list)| {
                list.iter()
                    .filter(|c| !c.is_ratio)
                    .map(|c| (c.attr_index, c.value))
            })
            .collect();
        for (attr_index, value) in weapon_contributions {
            self.add_base_value(attr_index, value);
        }

        let remaining: Vec<(usize, i32)> = self
            .equipment_contributions
            .iter()
            .flat_map(|(k, list)| {
                list.iter().filter_map(|c| {
                    if k.source_type == SourceType::Weapon && !c.is_ratio {
                        None
                    } else {
                        Some((c.attr_index, value_for_base(*c, &self.base_props)))
                    }
                })
            })
            .collect();
        for (attr_index, value) in remaining {
            self.add_increment_value(attr_index, value);
        }
    }

    fn add_base_value(&mut self, attr_index: usize, value: i32) {
        if let Some(slot) = self.base_props.get_mut(attr_index) {
            *slot += value;
        }
        if EAttributeType::from_i32_or_default(attr_index as i32) == EAttributeType::LifeMax
            && let Some(life) = self
                .base_props
                .get_mut(EAttributeType::Life.to_i32() as usize)
        {
            *life += value;
        }
    }

    fn add_increment_value(&mut self, attr_index: usize, value: i32) {
        if let Some(slot) = self.add_props.get_mut(attr_index) {
            *slot += value;
        }
        if EAttributeType::from_i32_or_default(attr_index as i32) == EAttributeType::LifeMax
            && let Some(life) = self
                .add_props
                .get_mut(EAttributeType::Life.to_i32() as usize)
        {
            *life += value;
        }
    }
}

fn contribution_from_prop_value(
    prop_id: i32,
    value: f64,
    is_ratio: bool,
    display_mode: DisplayMode,
) -> Option<Contribution> {
    let attr_index = attr_index(prop_id)?;
    Some(Contribution {
        attr_index,
        is_ratio,
        value: if is_ratio { 0 } else { value.round() as i32 },
        ratio: if is_ratio { value } else { 0.0 },
        display_mode: if is_ratio {
            DisplayMode::Add
        } else {
            display_mode
        },
    })
}

fn contribution_from_echo_prop(
    prop_id: i32,
    add_type: i32,
    value: i32,
    display_mode: DisplayMode,
) -> Option<Contribution> {
    let attr_index = attr_index(prop_id)?;
    let is_ratio = add_type == 2;
    Some(Contribution {
        attr_index,
        is_ratio,
        value: if is_ratio { 0 } else { value },
        ratio: if is_ratio {
            f64::from(value) / 10000.0
        } else {
            0.0
        },
        display_mode: if is_ratio {
            DisplayMode::Add
        } else {
            display_mode
        },
    })
}

fn contribution_from_buff(buff: &Buff, display_mode: DisplayMode) -> Option<Contribution> {
    if matches!(
        buff.GameAttributeID,
        EAttributeType::None | EAttributeType::MAX
    ) || buff.ModifierMagnitude.is_empty()
    {
        return None;
    }
    Some(Contribution {
        attr_index: buff.GameAttributeID.to_i32() as usize,
        is_ratio: false,
        value: buff.ModifierMagnitude[0],
        ratio: 0.0,
        display_mode,
    })
}

fn value_for_base(contribution: Contribution, base_props: &[i32]) -> i32 {
    if !contribution.is_ratio {
        return contribution.value;
    }
    let Some(&base) = base_props.get(contribution.attr_index) else {
        return 0;
    };
    (f64::from(base) * contribution.ratio).round() as i32
}

fn add_display_prop(map: &mut BTreeMap<i32, i32>, key: i32, value: i32) {
    *map.entry(key).or_default() += value;
}

fn attr_index(prop_id: i32) -> Option<usize> {
    if prop_id == 0 {
        return None;
    }
    let attr_type = match prop_id {
        10002 => EAttributeType::LifeMax,
        10007 => EAttributeType::Atk,
        10010 => EAttributeType::Def,
        other => {
            let attr_id = if other >= 10000 { other - 10000 } else { other };
            if attr_id <= 0 || attr_id >= EAttributeType::MAX.to_i32() {
                return None;
            }
            EAttributeType::from_i32_or_default(attr_id)
        }
    };
    if matches!(attr_type, EAttributeType::None | EAttributeType::MAX) {
        return None;
    }
    Some(attr_type.to_i32() as usize)
}

fn weapon_growth(
    assets: &Assets,
    curve_id: i32,
    level: i32,
    breach: i32,
) -> Option<&WeaponPropertyGrowth> {
    assets
        .tables
        .weapon_property_growth
        .items
        .iter()
        .find(|e| e.CurveId == curve_id && e.Level == level && e.BreachLevel == breach)
}

fn echo_marker_buff_effect(
    assets: &Assets,
    echo_comp: &PlayerEchoComponent,
    inc_id: i32,
) -> Option<i64> {
    let echo = echo_comp.echo_map.get(&inc_id)?;
    let base_item = assets.tables.phantom_item.get_data_by_id(&echo.id)?;
    let item = if echo.skin_id != 0
        && let Some(skin_item) = assets.tables.phantom_item.get_data_by_id(&echo.skin_id)
        && skin_item.ParentMonsterId == base_item.MonsterId
    {
        skin_item
    } else {
        base_item
    };
    if item.SkillId == 0 {
        return None;
    }
    let marker_id = i64::from(item.SkillId) * 1000 + 1;
    let marker = assets.tables.buff.get_data_by_id(&marker_id)?;
    if marker.GrantedTags.is_empty() || marker.ExtraEffectID != 0 {
        return None;
    }
    Some(marker_id)
}
