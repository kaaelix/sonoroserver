use sonoro_gamesv_data::Assets;
use sonoro_gamesv_data::data_tables::{DataTables, Table};
use sonoro_gamesv_data::tables::area::Area;
use sonoro_gamesv_data::tables::blueprint_config::EntityLogic;
use sonoro_gamesv_data::tables::entity_components::components::Components;
use sonoro_gamesv_data::tables::monster_property_growth::MonsterPropertyGrowth;
use sonoro_proto::enums::EAttributeType;
use sonoro_proto::wire::PbEnum;

use crate::component::entity::AttributeComponent;

const SCALE: i64 = 10000;

fn attr_index(attr_type: EAttributeType) -> usize {
    attr_type.to_i32() as usize
}

fn scale_property(value: &mut i32, ratio: i32) {
    *value = ((i64::from(*value) * i64::from(ratio)) / SCALE) as i32;
}

fn scale_if_present(props: &mut [i32], attr_type: EAttributeType, ratio: i32) {
    let idx = attr_index(attr_type);
    if let Some(slot) = props.get_mut(idx) {
        scale_property(slot, ratio);
    }
}

fn set_if_present(props: &mut [i32], attr_type: EAttributeType, value: i32) {
    let idx = attr_index(attr_type);
    if let Some(slot) = props.get_mut(idx) {
        *slot = value;
    }
}

fn value_if_present(props: &[i32], attr_type: EAttributeType) -> i32 {
    props.get(attr_index(attr_type)).copied().unwrap_or(0)
}

fn clamp_level(level: i32) -> i32 {
    level.clamp(1, 120)
}

fn property_id(components: &Components, entity_logic: EntityLogic) -> Option<i32> {
    if let Some(attr) = &components.AttributeComponent
        && !attr.Disabled.unwrap_or(false)
        && let Some(id) = attr.PropertyId
        && id > 0
    {
        return Some(id);
    }
    if let Some(base_info) = &components.BaseInfoComponent
        && let Some(id) = base_info.EntityPropertyId
        && id > 0
    {
        return Some(id);
    }
    match entity_logic {
        EntityLogic::Animal | EntityLogic::Monster | EntityLogic::Npc | EntityLogic::Vision => {
            Some(2)
        }
        _ => None,
    }
}

fn configured_level(components: &Components) -> i32 {
    components
        .AttributeComponent
        .as_ref()
        .and_then(|a| a.Level)
        .map_or(1, clamp_level)
}

fn area_monster_level(
    area_table: &Table<Area, i32>,
    area_id: i32,
    world_level: i32,
) -> Option<i32> {
    let mut current_area_id = area_id;
    let mut remaining = area_table.items.len();
    while current_area_id > 0 && remaining > 0 {
        remaining -= 1;
        let area = area_table.get_data_by_id(&current_area_id)?;
        if let Some(&level) = area.WorldMonsterLevelMax.get(&world_level) {
            return Some(clamp_level(level));
        }
        if area.Father <= 0 || area.Father == current_area_id {
            return None;
        }
        current_area_id = area.Father;
    }
    None
}

fn fallback_monster_level(world_level: i32) -> Option<i32> {
    match world_level {
        1 => Some(10),
        2 => Some(21),
        3 => Some(32),
        4 => Some(43),
        5 => Some(54),
        6 => Some(65),
        7 => Some(76),
        8 => Some(85),
        _ => None,
    }
}

fn effective_level(
    area_table: &Table<Area, i32>,
    components: &Components,
    area_id: i32,
    world_level: i32,
) -> i32 {
    let level = configured_level(components);
    if level >= 10 || components.AttributeComponent.is_none() {
        return level;
    }
    area_monster_level(area_table, area_id, world_level)
        .or_else(|| fallback_monster_level(world_level))
        .unwrap_or(level)
}

fn configured_curve_id(components: &Components) -> i32 {
    components
        .AttributeComponent
        .as_ref()
        .and_then(|a| a.MonsterPropGrowthId)
        .filter(|&id| id > 0)
        .unwrap_or(0)
}

fn hardness_mode_id(components: &Components) -> i32 {
    components
        .AttributeComponent
        .as_ref()
        .and_then(|a| a.HardnessModeId)
        .unwrap_or(0)
}

fn rage_mode_id(components: &Components) -> i32 {
    components
        .AttributeComponent
        .as_ref()
        .and_then(|a| a.RageModeId)
        .unwrap_or(0)
}

fn monster_growth_at(
    tables: &DataTables,
    level: i32,
    curve_id: i32,
) -> Option<MonsterPropertyGrowth> {
    if curve_id != 0 {
        tables
            .get_monster_property_growth(level, curve_id)
            .or_else(|| tables.get_monster_property_growth(level, 0))
            .cloned()
    } else {
        tables.get_monster_property_growth(level, 0).cloned()
    }
}

fn apply_monster_growth_config(props: &mut [i32], config: &MonsterPropertyGrowth) {
    scale_if_present(props, EAttributeType::LifeMax, config.LifeMaxRatio);
    scale_if_present(props, EAttributeType::Life, config.LifeMaxRatio);
    scale_if_present(props, EAttributeType::Atk, config.AtkRatio);
    scale_if_present(props, EAttributeType::Def, config.DefRatio);
    scale_if_present(props, EAttributeType::HardnessMax, config.HardnessMaxRatio);
    scale_if_present(props, EAttributeType::Hardness, config.HardnessRatio);
    scale_if_present(
        props,
        EAttributeType::HardnessRecover,
        config.HardnessRecoverRatio,
    );
    scale_if_present(props, EAttributeType::RageMax, config.RageMaxRatio);
    scale_if_present(props, EAttributeType::Rage, config.RageRatio);
    scale_if_present(props, EAttributeType::RageRecover, config.RageRecoverRatio);
    scale_if_present(
        props,
        EAttributeType::WeaknessBuildUpMax,
        config.WeaknessBuildUpMaxRatio,
    );
}

fn apply_monster_growth(tables: &DataTables, props: &mut [i32], components: &Components, level: i32) {
    if let Some(config) = monster_growth_at(tables, level, configured_curve_id(components)) {
        apply_monster_growth_config(props, &config);
    }
}

fn normalize_life(props: &mut [i32]) {
    let life_max = value_if_present(props, EAttributeType::LifeMax);
    let life = value_if_present(props, EAttributeType::Life);
    if life_max <= 0 {
        return;
    }
    if life <= 0 || life > life_max {
        set_if_present(props, EAttributeType::Life, life_max);
    }
}

pub fn create_combat_attributes(
    assets: &Assets,
    components: &Components,
    entity_logic: EntityLogic,
    area_id: i32,
    world_level: i32,
) -> Option<AttributeComponent> {
    let prop_id = property_id(components, entity_logic)?;
    let level = effective_level(&assets.tables.area, components, area_id, world_level);
    let mut props = assets.tables.get_props(prop_id);
    if props.is_empty() {
        return None;
    }
    set_if_present(&mut props, EAttributeType::Lv, level);
    apply_monster_growth(&assets.tables, &mut props, components, level);
    normalize_life(&mut props);

    Some(AttributeComponent::create_with_modes(
        &props,
        hardness_mode_id(components),
        rage_mode_id(components),
    ))
}
