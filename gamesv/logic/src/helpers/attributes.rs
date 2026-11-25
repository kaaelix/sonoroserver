use std::time::{SystemTime, UNIX_EPOCH};

use sonoro_proto::enums::EAttributeType;
use sonoro_proto::pb::{
    AttributeChangedNotify, CombatCommon, CombatNotifyData, CombatNotifyDataMessageOneof,
    CombatReceiveData, CombatReceiveDataMessageOneof, GameplayAttributeData,
    RecoverPropChangedNotify, RecoverPropFromServer,
};
use sonoro_proto::wire::PbEnum;

use crate::component::entity::attribute::{Attribute, AttributeComponent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttributeValueType {
    Base,
    Current,
    Increment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttributeChangeType {
    Delta,
    Override,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelatedAttrKind {
    Max,
    Recover,
}

pub fn gameplay_attribute_data(
    attr_type: EAttributeType,
    attr: Attribute,
) -> GameplayAttributeData {
    let ratio_attr = matches!(
        attr_type,
        EAttributeType::Atk | EAttributeType::LifeMax | EAttributeType::Def
    );
    GameplayAttributeData {
        AttributeType: Some(attr_type),
        BaseValue: if ratio_attr { attr.base } else { attr.current },
        CurrentValue: attr.current,
    }
}

pub fn get_element_bonus(element_type: i32) -> EAttributeType {
    match element_type {
        1 => EAttributeType::DamageChangeElement1,
        2 => EAttributeType::DamageChangeElement2,
        3 => EAttributeType::DamageChangeElement3,
        4 => EAttributeType::DamageChangeElement4,
        5 => EAttributeType::DamageChangeElement5,
        6 => EAttributeType::DamageChangeElement6,
        _ => EAttributeType::DamageChangePhys,
    }
}

pub fn get_element_res(element_type: i32) -> EAttributeType {
    match element_type {
        1 => EAttributeType::DamageResistanceElement1,
        2 => EAttributeType::DamageResistanceElement2,
        3 => EAttributeType::DamageResistanceElement3,
        4 => EAttributeType::DamageResistanceElement4,
        5 => EAttributeType::DamageResistanceElement5,
        6 => EAttributeType::DamageResistanceElement6,
        _ => EAttributeType::DamageResistancePhys,
    }
}

pub fn get_element_pen(element_type: i32) -> EAttributeType {
    match element_type {
        1 => EAttributeType::IgnoreDamageResistanceElement1,
        2 => EAttributeType::IgnoreDamageResistanceElement2,
        3 => EAttributeType::IgnoreDamageResistanceElement3,
        4 => EAttributeType::IgnoreDamageResistanceElement4,
        5 => EAttributeType::IgnoreDamageResistanceElement5,
        6 => EAttributeType::IgnoreDamageResistanceElement6,
        _ => EAttributeType::IgnoreDamageResistancePhys,
    }
}

pub fn get_element_reduce(element_type: i32) -> EAttributeType {
    match element_type {
        1 => EAttributeType::DamageReduceElement1,
        2 => EAttributeType::DamageReduceElement2,
        3 => EAttributeType::DamageReduceElement3,
        4 => EAttributeType::DamageReduceElement4,
        5 => EAttributeType::DamageReduceElement5,
        6 => EAttributeType::DamageReduceElement6,
        _ => EAttributeType::DamageReducePhys,
    }
}

pub fn get_ability_bonus(ability_type: i32) -> EAttributeType {
    match ability_type {
        0 => EAttributeType::DamageChangeAuto,
        1 => EAttributeType::DamageChangeCast,
        2 => EAttributeType::DamageChangeUltra,
        3 => EAttributeType::DamageChangeQte,
        4 => EAttributeType::DamageChangeNormalSkill,
        5 => EAttributeType::DamageChangePhantom,
        _ => EAttributeType::None,
    }
}

pub fn attr_from_id(attr_id: i32) -> EAttributeType {
    if attr_id <= 0 || attr_id >= EAttributeType::MAX.to_i32() {
        return EAttributeType::None;
    }
    EAttributeType::from_i32_or_default(attr_id)
}

pub fn get_attr_value(attr_comp: &AttributeComponent, attr_type: EAttributeType) -> i32 {
    if matches!(attr_type, EAttributeType::None | EAttributeType::MAX) {
        return 0;
    }
    let idx = attr_type.to_i32();
    if idx < 0 {
        return 0;
    }
    attr_comp
        .attributes
        .get(idx as usize)
        .map_or(0, |a| a.current)
}

pub fn get_attr_by_id(attr_comp: &AttributeComponent, attr_id: i32) -> i32 {
    get_attr_value(attr_comp, attr_from_id(attr_id))
}

pub fn set_life_unconditional(attr_comp: &mut AttributeComponent, hp: i32) -> i32 {
    let life_idx = EAttributeType::Life.to_i32() as usize;
    let max_idx = EAttributeType::LifeMax.to_i32() as usize;
    if attr_comp.attributes.len() <= life_idx || attr_comp.attributes.len() <= max_idx {
        return 0;
    }
    let max_hp = attr_comp.attributes[max_idx].current.max(0);
    let clamped = hp.clamp(0, max_hp);
    attr_comp.attributes[life_idx] = Attribute {
        base: clamped,
        increment: 0,
        current: clamped,
    };
    clamped
}

pub fn revive_entity(attr_comp: &mut AttributeComponent, hp: Option<i32>) -> bool {
    let life_idx = EAttributeType::Life.to_i32() as usize;
    let max_idx = EAttributeType::LifeMax.to_i32() as usize;
    if attr_comp.attributes.len() <= life_idx || attr_comp.attributes.len() <= max_idx {
        return false;
    }
    let was_dead = attr_comp.attributes[life_idx].current <= 0;
    let max_hp = attr_comp.attributes[max_idx].current.max(1);
    let target_hp = hp.map_or(max_hp, |v| v.clamp(1, max_hp));
    let _ = set_life_unconditional(attr_comp, target_hp);
    was_dead
}

pub fn get_related_attr(attr_type: EAttributeType, kind: RelatedAttrKind) -> Option<EAttributeType> {
    match kind {
        RelatedAttrKind::Max => match attr_type {
            EAttributeType::Life => Some(EAttributeType::LifeMax),
            EAttributeType::Strength => Some(EAttributeType::StrengthMax),
            EAttributeType::Energy => Some(EAttributeType::EnergyMax),
            EAttributeType::SpecialEnergy1 => Some(EAttributeType::SpecialEnergy1Max),
            EAttributeType::SpecialEnergy2 => Some(EAttributeType::SpecialEnergy2Max),
            EAttributeType::SpecialEnergy3 => Some(EAttributeType::SpecialEnergy3Max),
            EAttributeType::SpecialEnergy4 => Some(EAttributeType::SpecialEnergy4Max),
            EAttributeType::SpecialEnergy5 => Some(EAttributeType::SpecialEnergy5Max),
            EAttributeType::ElementEnergy => Some(EAttributeType::ElementEnergyMax),
            EAttributeType::Hardness => Some(EAttributeType::HardnessMax),
            EAttributeType::Tough => Some(EAttributeType::ToughMax),
            EAttributeType::Rage => Some(EAttributeType::RageMax),
            EAttributeType::ParalysisTime => Some(EAttributeType::ParalysisTimeMax),
            EAttributeType::WeaknessBuildUp => Some(EAttributeType::WeaknessBuildUpMax),
            EAttributeType::StatusBuildUp1 => Some(EAttributeType::StatusBuildUp1Max),
            EAttributeType::StatusBuildUp2 => Some(EAttributeType::StatusBuildUp2Max),
            EAttributeType::StatusBuildUp3 => Some(EAttributeType::StatusBuildUp3Max),
            EAttributeType::StatusBuildUp4 => Some(EAttributeType::StatusBuildUp4Max),
            EAttributeType::StatusBuildUp5 => Some(EAttributeType::StatusBuildUp5Max),
            _ => None,
        },
        RelatedAttrKind::Recover => match attr_type {
            EAttributeType::Hardness => Some(EAttributeType::HardnessRecover),
            EAttributeType::Tough => Some(EAttributeType::ToughRecover),
            EAttributeType::ParalysisTime => Some(EAttributeType::ParalysisTimeRecover),
            EAttributeType::Rage => Some(EAttributeType::RageRecover),
            EAttributeType::Strength => Some(EAttributeType::StrengthRecover),
            _ => None,
        },
    }
}

const ATTR_ALLOW_NEGATIVES: &[EAttributeType] = &[
    EAttributeType::Crit,
    EAttributeType::DamageReduce,
    EAttributeType::DamageReducePhys,
    EAttributeType::DamageReduceElement1,
    EAttributeType::DamageReduceElement2,
    EAttributeType::DamageReduceElement3,
    EAttributeType::DamageReduceElement4,
    EAttributeType::DamageReduceElement5,
    EAttributeType::DamageReduceElement6,
    EAttributeType::DamageResistancePhys,
    EAttributeType::DamageResistanceElement1,
    EAttributeType::DamageResistanceElement2,
    EAttributeType::DamageResistanceElement3,
    EAttributeType::DamageResistanceElement4,
    EAttributeType::DamageResistanceElement5,
    EAttributeType::DamageResistanceElement6,
    EAttributeType::ParalysisTimeRecover,
];

fn clamp_attr(attr_type: EAttributeType, value: i32, max_value: Option<i32>) -> i32 {
    match attr_type {
        EAttributeType::AutoAttackSpeed | EAttributeType::CastAttackSpeed => value.clamp(0, 20000),
        EAttributeType::DamageReduce => value.clamp(-10000, 10000),
        _ => {
            if let Some(max) = max_value {
                return value.clamp(0, max);
            }
            if ATTR_ALLOW_NEGATIVES.contains(&attr_type) {
                value
            } else {
                value.max(0)
            }
        }
    }
}

pub fn change_attr(
    attr_comp: &mut AttributeComponent,
    attr_type: EAttributeType,
    change_type: AttributeChangeType,
    value_type: AttributeValueType,
    value: i32,
) -> Vec<EAttributeType> {
    let life_idx = EAttributeType::Life.to_i32() as usize;
    let life_max_idx = EAttributeType::LifeMax.to_i32() as usize;
    let is_alive = attr_comp.attributes.len() > life_idx && attr_comp.attributes[life_idx].current > 0;
    if attr_type == EAttributeType::Life && !is_alive {
        return Vec::new();
    }

    let max_value = get_related_attr(attr_type, RelatedAttrKind::Max).and_then(|max_attr| {
        let max_idx = max_attr.to_i32();
        if max_idx >= 0 {
            attr_comp.attributes.get(max_idx as usize).map(|a| a.current)
        } else {
            None
        }
    });

    let hp_ratio_before = if attr_type == EAttributeType::LifeMax
        && life_idx < attr_comp.attributes.len()
        && life_max_idx < attr_comp.attributes.len()
    {
        let life = attr_comp.attributes[life_idx].current;
        let life_max = attr_comp.attributes[life_max_idx].current;
        if life_max > 0 {
            Some((life as f32) / (life_max as f32))
        } else {
            None
        }
    } else {
        None
    };

    let result = apply_attr_change(
        attr_comp,
        attr_type,
        change_type,
        value_type,
        value,
        max_value,
    );

    if let Some(ratio) = hp_ratio_before
        && life_max_idx < attr_comp.attributes.len()
        && life_idx < attr_comp.attributes.len()
    {
        let new_max = attr_comp.attributes[life_max_idx].current;
        let new_hp = ((new_max as f32) * ratio).round() as i32;
        let clamped = new_hp.clamp(0, new_max);
        attr_comp.attributes[life_idx].base = clamped;
        attr_comp.attributes[life_idx].current = clamped;
    }

    result
}

pub fn apply_attr_change(
    attr_comp: &mut AttributeComponent,
    attr_type: EAttributeType,
    change_type: AttributeChangeType,
    value_type: AttributeValueType,
    value: i32,
    max_value: Option<i32>,
) -> Vec<EAttributeType> {
    let idx = attr_type.to_i32();
    if idx < 0 || (idx as usize) >= attr_comp.attributes.len() {
        return Vec::new();
    }
    let attr = &mut attr_comp.attributes[idx as usize];
    match change_type {
        AttributeChangeType::Override => match value_type {
            AttributeValueType::Base => {
                attr.base = clamp_attr(attr_type, value, max_value);
                attr.current = clamp_attr(attr_type, attr.base + attr.increment, max_value);
            }
            AttributeValueType::Increment => {
                attr.increment = value;
                attr.current = clamp_attr(attr_type, attr.base + attr.increment, max_value);
            }
            AttributeValueType::Current => {
                attr.current = clamp_attr(attr_type, value, max_value);
            }
        },
        AttributeChangeType::Delta => match value_type {
            AttributeValueType::Base => {
                attr.base = clamp_attr(attr_type, attr.base + value, max_value);
                attr.current = clamp_attr(attr_type, attr.base + attr.increment, max_value);
            }
            AttributeValueType::Increment => {
                attr.increment += value;
                attr.current = clamp_attr(attr_type, attr.base + attr.increment, max_value);
            }
            AttributeValueType::Current => {
                attr.current = clamp_attr(attr_type, attr.current + value, max_value);
            }
        },
    }
    vec![attr_type]
}

pub fn generate_attr_messages(
    combat_receive_pack: &mut Vec<CombatReceiveData>,
    entity_id: i64,
    attr_comp: &AttributeComponent,
    changed: &[EAttributeType],
) {
    if changed.is_empty() {
        return;
    }
    let mut changes = Vec::new();
    let mut recovers = Vec::new();
    let recover_types = [
        EAttributeType::Hardness,
        EAttributeType::Tough,
        EAttributeType::ParalysisTime,
        EAttributeType::Rage,
    ];

    for &attr_type in changed {
        let idx = attr_type.to_i32();
        if idx < 0 || (idx as usize) >= attr_comp.attributes.len() {
            continue;
        }
        let attr = attr_comp.attributes[idx as usize];
        if recover_types.contains(&attr_type) {
            let max_val = get_related_attr(attr_type, RelatedAttrKind::Max)
                .and_then(|m| attr_comp.attributes.get(m.to_i32() as usize))
                .map_or(0, |a| a.current);
            let recover_val = get_related_attr(attr_type, RelatedAttrKind::Recover)
                .and_then(|r| attr_comp.attributes.get(r.to_i32() as usize))
                .map_or(0, |a| a.current);
            recovers.push(RecoverPropFromServer {
                AttrId: idx,
                Ratio: recover_val,
                MaxValue: max_val,
                ValueIncrement: attr.current,
            });
        }
        changes.push(gameplay_attribute_data(attr_type, attr));
    }

    if !changes.is_empty() {
        combat_receive_pack.push(CombatReceiveData {
            Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
                CombatNotifyData {
                    CombatCommon: Some(CombatCommon {
                        EntityId: entity_id,
                        ..Default::default()
                    }),
                    Message: Some(CombatNotifyDataMessageOneof::AttributeChangedNotify(
                        AttributeChangedNotify { Attributes: changes },
                    )),
                },
            )),
        });
    }
    if !recovers.is_empty() {
        let now_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_millis() as i64);
        combat_receive_pack.push(CombatReceiveData {
            Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
                CombatNotifyData {
                    CombatCommon: Some(CombatCommon {
                        EntityId: entity_id,
                        ..Default::default()
                    }),
                    Message: Some(CombatNotifyDataMessageOneof::RecoverPropChangedNotify(
                        RecoverPropChangedNotify {
                            Attributes: recovers,
                            Duration: now_ms,
                        },
                    )),
                },
            )),
        });
    }
}
