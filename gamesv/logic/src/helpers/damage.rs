use std::time::{SystemTime, UNIX_EPOCH};

use sonoro_gamesv_data::Assets;
use sonoro_gamesv_data::tables::damage::Damage;
use sonoro_proto::enums::{EAttributeType, EDamageImmune};
use sonoro_proto::pb::{CombatReceiveData, DamageExecuteNotify, DamageExecuteRequest};
use sonoro_proto::wire::PbEnum;

use crate::component::entity::EntityComponentStorage;
use crate::component::entity::attribute::{Attribute, AttributeComponent};
use crate::fsm::types::wake_reason;
use crate::helpers::attributes::{self, AttributeChangeType, AttributeValueType};
use crate::helpers::{buff as buff_helper, gameplay_tags, level_play};
use crate::scene::Scene;
use crate::schedulers::BuffTimerScheduler;

pub const TAG_WEAKNESS_FULL: &str = "功能.逻辑状态标识.6067066e.39f6ce36";
pub const TAG_WEAKNESS_BREAK: &str = "功能.逻辑状态标识.6067066e.39f6ce36.44cd27ac";
pub const TAG_WEAKNESS_CD: &str = "功能.逻辑状态标识.6067066e.0cbeb185.14d851f7";
pub const TAG_WEAKNESS_LOCK: &str = "功能.逻辑状态标识.6067066e.f2d5373f";
pub const BUFF_AUTO_WEAKNESS: i64 = 3158;

fn level_slice_value(slice: &[i32], skill_level: i32, default_val: i32) -> i32 {
    if slice.is_empty() {
        return default_val;
    }
    let idx = (skill_level - 1).max(0) as usize;
    slice[idx.min(slice.len() - 1)]
}

fn formula_param(slice: &[i32], skill_level: i32) -> f64 {
    f64::from(level_slice_value(slice, skill_level, 0))
}

fn attr_from_owner(
    attacker_attr: &AttributeComponent,
    target_attr: &AttributeComponent,
    owner_flag: f64,
    attr_id_f: f64,
) -> f64 {
    let attr_id = attr_id_f as i32;
    if attr_id <= 0 || attr_id >= EAttributeType::MAX.to_i32() {
        return 0.0;
    }
    let owner_idx = owner_flag as i32;
    let src = if owner_idx != 0 {
        target_attr
    } else {
        attacker_attr
    };
    f64::from(attributes::get_attr_by_id(src, attr_id))
}

fn resistance_multiplier(res_bp: i32) -> f64 {
    let res = f64::from(res_bp) / 10_000.0;
    if res < 0.0 {
        1.0 - (res / 2.0)
    } else if res < 0.8 {
        1.0 - res
    } else {
        1.0 / (1.0 + 5.0 * res)
    }
}

fn calculate_hurt_core(
    base_or_formula: f64,
    attacker_attr: &AttributeComponent,
    target_attr: &AttributeComponent,
    damage: &Damage,
    is_crit: bool,
    weakness_mult: f64,
) -> f64 {
    let raw_crit_dmg = attributes::get_attr_value(attacker_attr, EAttributeType::CritDamage);
    let crit_dmg_bp = if raw_crit_dmg > 0 {
        raw_crit_dmg
    } else {
        15_000
    };
    let crit_mult = if is_crit {
        f64::from(crit_dmg_bp) / 10_000.0
    } else {
        1.0
    };

    let elem_bonus = f64::from(attributes::get_attr_value(
        attacker_attr,
        attributes::get_element_bonus(damage.Element),
    )) / 10_000.0;
    let ability_bonus = f64::from(attributes::get_attr_value(
        attacker_attr,
        attributes::get_ability_bonus(damage.Type),
    )) / 10_000.0;
    let dmg_change =
        f64::from(attributes::get_attr_value(attacker_attr, EAttributeType::DamageChange))
            / 10_000.0;
    let total_bonus = ((1.0 + dmg_change) + elem_bonus + ability_bonus).max(0.0);

    let dmg_reduce =
        (f64::from(attributes::get_attr_value(target_attr, EAttributeType::DamageReduce))
            / 10_000.0)
            .min(1.0);
    let elem_reduce = (f64::from(attributes::get_attr_value(
        target_attr,
        attributes::get_element_reduce(damage.Element),
    )) / 10_000.0)
        .min(1.0);
    let special_mult = (1.0
        + f64::from(attributes::get_attr_value(
            attacker_attr,
            EAttributeType::SpecialDamageChange,
        )) / 10_000.0)
        .max(0.0);

    let attacker_lv = attributes::get_attr_value(attacker_attr, EAttributeType::Lv).max(1);
    let target_def = attributes::get_attr_value(target_attr, EAttributeType::Def).max(0);
    let ignore_def =
        (f64::from(attributes::get_attr_value(attacker_attr, EAttributeType::IgnoreDefRate))
            / 10_000.0)
            .clamp(0.0, 1.0);
    let lv_term = 800.0 + 8.0 * f64::from(attacker_lv);
    let def_mult = (1.0 / (f64::from(target_def) * (1.0 - ignore_def) / lv_term + 1.0)).min(2.0);

    let raw_res =
        attributes::get_attr_value(target_attr, attributes::get_element_res(damage.Element));
    let pen =
        attributes::get_attr_value(attacker_attr, attributes::get_element_pen(damage.Element));
    let res_mult = resistance_multiplier(raw_res - pen);

    (base_or_formula
        * crit_mult
        * def_mult
        * total_bonus
        * res_mult
        * (1.0 - dmg_reduce)
        * (1.0 - elem_reduce)
        * special_mult
        * weakness_mult.max(0.0))
        .max(0.0)
}

fn compute_formula_raw(
    assets: &Assets,
    damage: &Damage,
    attacker_attr: &AttributeComponent,
    target_attr: &AttributeComponent,
    skill_level: i32,
    is_crit: bool,
    weakness_mult: f64,
) -> f64 {
    let rate_bp = level_slice_value(&damage.RateLv, skill_level, 0);
    let prop_id = if damage.RelatedProperty > 0 {
        damage.RelatedProperty
    } else {
        EAttributeType::Atk.to_i32()
    };
    let base_stat = attributes::get_attr_by_id(attacker_attr, prop_id).max(0);
    let rate_scaled = f64::from(base_stat) * (f64::from(rate_bp) / 10_000.0);

    let p0 = formula_param(&damage.FormulaParam1, skill_level);
    let p1 = formula_param(&damage.FormulaParam2, skill_level);
    let p2 = formula_param(&damage.FormulaParam3, skill_level);
    let p3 = formula_param(&damage.FormulaParam4, skill_level);
    let p4 = formula_param(&damage.FormulaParam5, skill_level);
    let p5 = formula_param(&damage.FormulaParam6, skill_level);
    let p6 = formula_param(&damage.FormulaParam7, skill_level);
    let p7 = formula_param(&damage.FormulaParam8, skill_level);
    let p8 = formula_param(&damage.FormulaParam9, skill_level);
    let p9 = formula_param(&damage.FormulaParam10, skill_level);

    match damage.FormulaType {
        0 => {
            if damage.CalculateType == 1 {
                let cure_base = level_slice_value(&damage.CureBaseValue, skill_level, 0);
                return (rate_scaled + f64::from(cure_base.max(0))).max(0.0);
            }
            calculate_hurt_core(
                rate_scaled,
                attacker_attr,
                target_attr,
                damage,
                is_crit,
                weakness_mult,
            )
        }
        1 => {
            let num11 = attr_from_owner(attacker_attr, target_attr, p3, p2) * (p4 / 10_000.0) + p5;
            let num12 = attr_from_owner(attacker_attr, target_attr, p8, p7) * (p6 / 10_000.0)
                + p9
                + attr_from_owner(attacker_attr, target_attr, p1, p0) * num11;
            if damage.CalculateType == 1 {
                return num12.max(0.0);
            }
            calculate_hurt_core(
                (rate_scaled + num12).max(0.0),
                attacker_attr,
                target_attr,
                damage,
                is_crit,
                weakness_mult,
            )
        }
        2 => {
            let num10 = attr_from_owner(attacker_attr, target_attr, p1, p0) * (p2 / 10_000.0) + p3;
            let t_life = f64::from(attributes::get_attr_value(target_attr, EAttributeType::Life));
            if damage.CalculateType == 1 {
                (-(t_life - num10).min(0.0)).max(0.0)
            } else {
                (t_life - num10).max(0.0)
            }
        }
        3 | 4 => {
            (attr_from_owner(attacker_attr, target_attr, p1, p0) * (p2 / 10_000.0) + p3).max(0.0)
        }
        5 => {
            let t_max =
                f64::from(attributes::get_attr_value(target_attr, EAttributeType::LifeMax));
            (t_max * (p0 / 10_000.0) + p1).max(0.0)
        }
        6 => {
            let a = attr_from_owner(attacker_attr, target_attr, p1, p0) * (p2 / 10_000.0);
            let b = attr_from_owner(attacker_attr, target_attr, p4, p3) * (p5 / 10_000.0);
            (a + b).max(0.0)
        }
        7 => {
            let val = attr_from_owner(attacker_attr, target_attr, p1, p0) * (p2 / 10_000.0) + p3;
            let t_life = f64::from(attributes::get_attr_value(target_attr, EAttributeType::Life));
            let t_max =
                f64::from(attributes::get_attr_value(target_attr, EAttributeType::LifeMax));
            let val2 = t_life - (t_max * (p4 / 10_000.0) + p5);
            val.min(val2).max(0.0)
        }
        9 => {
            if rate_bp > 0 {
                return calculate_hurt_core(
                    rate_scaled,
                    attacker_attr,
                    target_attr,
                    damage,
                    is_crit,
                    weakness_mult,
                );
            }
            let lv = attributes::get_attr_value(attacker_attr, EAttributeType::Lv).max(1);
            f64::from(lv * 25 + 100)
        }
        1001 => {
            let attacker_lv =
                attributes::get_attr_value(attacker_attr, EAttributeType::Lv).max(1);
            let ab_base = assets
                .tables
                .abnormal_damage_config
                .get_data_by_id(&attacker_lv)
                .map_or(0.0, |ab| match damage.Element {
                    1 => f64::from(ab.Abnormal1003),
                    2 => f64::from(ab.Abnormal1004),
                    3 => f64::from(ab.Abnormal1002),
                    4 => f64::from(ab.Abnormal1001),
                    5 => f64::from(ab.Abnormal1005),
                    _ => f64::from(ab.Abnormal1006),
                });
            let rate = p0 / 10_000.0;
            let dmg_reduce =
                (f64::from(attributes::get_attr_value(target_attr, EAttributeType::DamageReduce))
                    / 10_000.0)
                    .min(1.0);
            let elem_reduce = (f64::from(attributes::get_attr_value(
                target_attr,
                attributes::get_element_reduce(damage.Element),
            )) / 10_000.0)
                .min(1.0);
            let special_change = f64::from(attributes::get_attr_value(
                attacker_attr,
                EAttributeType::SpecialDamageChange,
            )) / 10_000.0;
            let target_def = attributes::get_attr_value(target_attr, EAttributeType::Def).max(0);
            let ignore_def = (f64::from(attributes::get_attr_value(
                attacker_attr,
                EAttributeType::IgnoreDefRate,
            )) / 10_000.0)
                .clamp(0.0, 1.0);
            let lv_term = 800.0 + 8.0 * f64::from(attacker_lv);
            let def_mult =
                (1.0 / (f64::from(target_def) * (1.0 - ignore_def) / lv_term + 1.0)).min(2.0);
            let raw_res = attributes::get_attr_value(
                target_attr,
                attributes::get_element_res(damage.Element),
            );
            let pen = attributes::get_attr_value(
                attacker_attr,
                attributes::get_element_pen(damage.Element),
            );
            let res_mult = resistance_multiplier(raw_res - pen);
            (ab_base
                * rate
                * def_mult
                * res_mult
                * (1.0 - dmg_reduce)
                * (1.0 - elem_reduce)
                * (1.0 + special_change.max(-1.0)))
            .max(0.0)
        }
        _ => calculate_hurt_core(
            rate_scaled,
            attacker_attr,
            target_attr,
            damage,
            is_crit,
            weakness_mult,
        ),
    }
}

fn fluctuation_factor(damage: &Damage, request: &DamageExecuteRequest) -> f64 {
    let upper = level_slice_value(&damage.FluctuationUpper, request.SkillLevel, 10_000);
    let lower = level_slice_value(&damage.FluctuationLower, request.SkillLevel, 10_000);
    if upper == lower {
        return f64::from(lower) * 0.0001;
    }
    let seed_mod = i64::from(request.RandomSeed).max(0) % 10_000;
    let interp_bp =
        f64::from(lower) + (f64::from(upper - lower) * (seed_mod as f64) / 10_000.0);
    interp_bp * 0.0001
}

fn roll_is_crit(request: &DamageExecuteRequest, crit_rate: i32) -> bool {
    if request.ForceCritical {
        return true;
    }
    if crit_rate <= 0 {
        return false;
    }
    if crit_rate >= 10_000 {
        return true;
    }
    let roll = if request.RandomSeed != 0 {
        let mut h = i64::from(request.RandomSeed) as u64;
        h ^= (request.DamageId as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15);
        h = (h ^ (h >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        h ^= h >> 27;
        h % 10_000
    } else {
        let now_ns = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0u64, |d| d.as_nanos() as u64);
        let mut h = now_ns;
        h ^= (request.DamageId as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15);
        h ^= request.TargetEntityId as u64;
        h = (h ^ (h >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        h ^= h >> 27;
        h % 10_000
    };
    roll < (crit_rate as u64)
}

fn apply_attacker_energy(
    combat_receive_pack: &mut Vec<CombatReceiveData>,
    request: &DamageExecuteRequest,
    damage: &Damage,
    attacker_id: i64,
    scene: &mut Scene,
) {
    let Some(&idx) = scene.net_id_map.get(&attacker_id) else {
        return;
    };
    let Some(attacker_attr) = scene.entities[idx].attribute.as_mut() else {
        return;
    };
    let mut woke = false;

    let base_concerto = level_slice_value(&damage.ElementPower, request.SkillLevel, 0);
    if base_concerto > 0 {
        let raw_elem_eff =
            attributes::get_attr_value(attacker_attr, EAttributeType::ElementEfficiency);
        let elem_eff = if raw_elem_eff > 0 {
            raw_elem_eff
        } else {
            10_000
        };
        let concerto = (i64::from(base_concerto) * i64::from(elem_eff) / 10_000) as i32;
        if concerto > 0 {
            let change = attributes::change_attr(
                attacker_attr,
                EAttributeType::ElementEnergy,
                AttributeChangeType::Delta,
                AttributeValueType::Current,
                concerto,
            );
            if !change.is_empty() {
                woke = true;
                attributes::generate_attr_messages(
                    combat_receive_pack,
                    request.AttackerEntityId,
                    attacker_attr,
                    &change,
                );
            }
        }
    }

    if request.IsAddEnergy {
        let base_energy = level_slice_value(&damage.Energy, request.SkillLevel, 0);
        if base_energy > 0 {
            let raw_energy_eff =
                attributes::get_attr_value(attacker_attr, EAttributeType::EnergyEfficiency);
            let energy_eff = if raw_energy_eff > 0 {
                raw_energy_eff
            } else {
                10_000
            };
            let energy_gain = (i64::from(base_energy) * i64::from(energy_eff) / 10_000) as i32;
            if energy_gain > 0 {
                let change = attributes::change_attr(
                    attacker_attr,
                    EAttributeType::Energy,
                    AttributeChangeType::Delta,
                    AttributeValueType::Current,
                    energy_gain,
                );
                if !change.is_empty() {
                    woke = true;
                    attributes::generate_attr_messages(
                        combat_receive_pack,
                        request.AttackerEntityId,
                        attacker_attr,
                        &change,
                    );
                }
            }
        }
    }

    if woke {
        scene.mark_fsm_dirty(attacker_id, wake_reason::ATTRIBUTE);
    }
}

fn default_attacker_attributes() -> AttributeComponent {
    let mut comp = AttributeComponent::default();
    let set = |c: &mut AttributeComponent, attr: EAttributeType, val: i32| {
        c.attributes[attr.to_i32() as usize] = Attribute {
            base: val,
            increment: 0,
            current: val,
        };
    };
    set(&mut comp, EAttributeType::Lv, 1);
    set(&mut comp, EAttributeType::Atk, 100);
    set(&mut comp, EAttributeType::CritDamage, 15_000);
    set(&mut comp, EAttributeType::BreakWeaknessRatio, 10_000);
    comp
}

fn sanitize_part_index(scene: &Scene, target_net_id: i64, requested_part: i32) -> i32 {
    if requested_part < 0 {
        return -1;
    }
    let Some(entity) = scene.entity(target_net_id) else {
        return -1;
    };
    let Some(part_comp) = entity.part.as_ref() else {
        return -1;
    };
    if part_comp.parts.iter().any(|p| p.index == requested_part) {
        requested_part
    } else {
        -1
    }
}

fn hardness_mode_multiplier(
    assets: &Assets,
    mode_id: i32,
    damage_type: i32,
    default_without_mode: f64,
) -> f64 {
    let Some(mode) = (mode_id > 0)
        .then(|| assets.tables.hardness_mode.get_data_by_id(&mode_id))
        .flatten()
    else {
        return default_without_mode;
    };
    let pct = f64::from(mode.Percent) / 10_000.0;
    if pct <= 0.0 {
        return 0.0;
    }
    let skill_param = usize::try_from(damage_type)
        .ok()
        .and_then(|idx| mode.SkillTypeParam.get(idx).copied())
        .unwrap_or(1.0);
    (pct * skill_param).max(0.0)
}

pub fn entity_has_gameplay_tag(
    storage: &EntityComponentStorage,
    assets: &Assets,
    target_tag_id: Option<i32>,
) -> bool {
    let Some(target_tag_id) = target_tag_id.filter(|&id| id != 0) else {
        return false;
    };
    let parents = &assets.tables.gameplay_tag_parent;
    if storage
        .tag
        .as_ref()
        .is_some_and(|t| t.has_tag(parents, target_tag_id))
    {
        return true;
    }
    if storage.fsm.as_ref().is_some_and(|fsm| {
        fsm.tags.iter().any(|tag| {
            tag.count > 0
                && i32::try_from(tag.id)
                    .is_ok_and(|gid| gameplay_tags::contains(parents, gid, target_tag_id))
        })
    }) {
        return true;
    }
    storage.buffs.as_ref().is_some_and(|buffs| {
        buffs.buffs.iter().any(|info| {
            info.IsActive
                && assets
                    .tables
                    .buff
                    .get_data_by_id(&info.BuffId)
                    .is_some_and(|buff_cfg| {
                        buff_cfg.GrantedTags.iter().any(|name| {
                            gameplay_tags::id_from_name(&assets.tables.gameplay_tag_remap, name)
                                .is_some_and(|gid| gameplay_tags::contains(parents, gid, target_tag_id))
                        })
                    })
        })
    })
}

fn apply_hardness_and_rage_on_hit(
    combat_receive_pack: &mut Vec<CombatReceiveData>,
    assets: &Assets,
    damage: &Damage,
    request: &DamageExecuteRequest,
    attacker_attr: &AttributeComponent,
    scene: &mut Scene,
    target_idx: usize,
) -> bool {
    if scene.entities[target_idx].config.entity_type != crate::component::entity::EntityType::Monster
    {
        return false;
    }
    let hardness_lv = f64::from(level_slice_value(&damage.HardnessLv, request.SkillLevel, 0));
    if hardness_lv <= 0.0 {
        return false;
    }

    let rage_tag_id =
        gameplay_tags::id_from_name(&assets.tables.gameplay_tag_remap, "功能.功能制作.狂暴生效");
    let palsy_tag_id =
        gameplay_tags::id_from_name(&assets.tables.gameplay_tag_remap, "怪物.common.状态标识.瘫痪");
    let target_storage = &scene.entities[target_idx];
    let in_rage_mode = entity_has_gameplay_tag(target_storage, assets, rage_tag_id);
    let is_paralyzed = entity_has_gameplay_tag(target_storage, assets, palsy_tag_id);

    let Some(target_attr) = scene.entities[target_idx].attribute.as_mut() else {
        return false;
    };

    let (attr_type, signed_delta) = if in_rage_mode {
        let rage_max = attributes::get_attr_value(target_attr, EAttributeType::RageMax);
        let cur_rage = attributes::get_attr_value(target_attr, EAttributeType::Rage);
        if rage_max <= 0 || cur_rage <= 0 {
            return false;
        }
        let mode_mult =
            hardness_mode_multiplier(assets, target_attr.rage_mode_id, damage.Type, 1.0);
        let atk_chg = (1.0
            + f64::from(attributes::get_attr_value(attacker_attr, EAttributeType::RageChange))
                / 10_000.0)
            .max(0.0);
        let tgt_red = (1.0
            + f64::from(attributes::get_attr_value(target_attr, EAttributeType::RageReduce))
                / 10_000.0)
            .max(0.0);
        let amount = ((hardness_lv * mode_mult * atk_chg * tgt_red).round() as i32).max(0);
        let actual = -(amount.min(cur_rage));
        (EAttributeType::Rage, actual)
    } else {
        if is_paralyzed {
            return false;
        }
        let hardness_max = attributes::get_attr_value(target_attr, EAttributeType::HardnessMax);
        let cur_hardness = attributes::get_attr_value(target_attr, EAttributeType::Hardness);
        if hardness_max <= 0 || cur_hardness >= hardness_max {
            return false;
        }
        let mode_mult =
            hardness_mode_multiplier(assets, target_attr.hardness_mode_id, damage.Type, 0.0);
        let atk_chg = (1.0
            + f64::from(attributes::get_attr_value(
                attacker_attr,
                EAttributeType::HardnessChange,
            )) / 10_000.0)
            .max(0.0);
        let tgt_red = (1.0
            + f64::from(attributes::get_attr_value(
                target_attr,
                EAttributeType::HardnessReduce,
            )) / 10_000.0)
            .max(0.0);
        let amount = ((hardness_lv * mode_mult * atk_chg * tgt_red).round() as i32).max(0);
        let actual = amount.min(hardness_max - cur_hardness);
        (EAttributeType::Hardness, actual)
    };

    if signed_delta == 0 {
        return false;
    }
    let change = attributes::change_attr(
        target_attr,
        attr_type,
        AttributeChangeType::Delta,
        AttributeValueType::Current,
        signed_delta,
    );
    if change.is_empty() {
        return false;
    }
    attributes::generate_attr_messages(combat_receive_pack, request.TargetEntityId, target_attr, &change);
    true
}

pub fn tune_break_cooldown_buff_id(
    assets: &Assets,
    scene_map_id: i32,
    storage: &EntityComponentStorage,
) -> i64 {
    let match_type = level_play::resolve_entity_config(
        assets,
        scene_map_id,
        i64::from(storage.config.config_id),
    )
    .and_then(|resolved| resolved.components.BaseInfoComponent)
    .and_then(|bi| bi.Category)
    .and_then(|cat| cat.MonsterMatchType)
    .unwrap_or(0);
    match match_type {
        0 => 3196,
        1 => 3197,
        2 => 3198,
        3 => 3199,
        _ => 3196,
    }
}

fn weakness_damage_multiplier(
    assets: &Assets,
    damage: &Damage,
    request: &DamageExecuteRequest,
    attacker_attr: &AttributeComponent,
    target_attr: &AttributeComponent,
    target_storage: &EntityComponentStorage,
) -> f64 {
    if damage.CalculateType != 0 {
        return 1.0;
    }
    let break_tag_id = gameplay_tags::id_from_name(&assets.tables.gameplay_tag_remap, TAG_WEAKNESS_BREAK);
    let target_has_break = entity_has_gameplay_tag(target_storage, assets, break_tag_id);
    let is_break_hit = request.IsBreakWeakness || damage.Type == 11;
    if !is_break_hit && !target_has_break {
        return 1.0;
    }

    let total_bonus =
        attributes::get_attr_value(target_attr, EAttributeType::WeaknessTotalBonus).max(0);
    let total_bonus_mult = 1.0 + f64::from(total_bonus) / 10_000.0;
    let mastery_mult = if is_break_hit {
        let mastery =
            attributes::get_attr_value(attacker_attr, EAttributeType::WeaknessMastery).max(0);
        1.0 + f64::from(mastery) / 100.0
    } else {
        1.0
    };
    total_bonus_mult * mastery_mult
}

fn apply_weakness_on_hit(
    combat_receive_pack: &mut Vec<CombatReceiveData>,
    assets: &Assets,
    damage: &Damage,
    request: &DamageExecuteRequest,
    attacker_attr: &AttributeComponent,
    scene: &mut Scene,
    target_idx: usize,
) -> i32 {
    if scene.entities[target_idx].config.entity_type
        != crate::component::entity::EntityType::Monster
        || damage.CalculateType != 0
    {
        return 0;
    }

    let full_tid = gameplay_tags::id_from_name(&assets.tables.gameplay_tag_remap, TAG_WEAKNESS_FULL);
    let break_tid =
        gameplay_tags::id_from_name(&assets.tables.gameplay_tag_remap, TAG_WEAKNESS_BREAK);
    let cd_tid = gameplay_tags::id_from_name(&assets.tables.gameplay_tag_remap, TAG_WEAKNESS_CD);
    let lock_tid = gameplay_tags::id_from_name(&assets.tables.gameplay_tag_remap, TAG_WEAKNESS_LOCK);

    let target_storage = &scene.entities[target_idx];
    let has_full = entity_has_gameplay_tag(target_storage, assets, full_tid);
    let has_break = entity_has_gameplay_tag(target_storage, assets, break_tid);
    let has_cd = entity_has_gameplay_tag(target_storage, assets, cd_tid);
    let has_lock = entity_has_gameplay_tag(target_storage, assets, lock_tid);

    let Some(target_attr) = scene.entities[target_idx].attribute.as_mut() else {
        return 0;
    };

    if request.IsBreakWeakness {
        if !has_full {
            return 0;
        }
        let change = attributes::change_attr(
            target_attr,
            EAttributeType::WeaknessBuildUp,
            AttributeChangeType::Override,
            AttributeValueType::Current,
            0,
        );
        if !change.is_empty() {
            attributes::generate_attr_messages(
                combat_receive_pack,
                request.TargetEntityId,
                target_attr,
                &change,
            );
        }
        return 1;
    }

    if has_full || has_break || has_cd || has_lock {
        return 0;
    }

    let weakness_lv = f64::from(level_slice_value(&damage.WeaknessLvl, request.SkillLevel, 0));
    if weakness_lv <= 0.0 {
        return 0;
    }

    let weakness_max = attributes::get_attr_value(target_attr, EAttributeType::WeaknessBuildUpMax);
    let cur_weakness = attributes::get_attr_value(target_attr, EAttributeType::WeaknessBuildUp);
    if weakness_max <= 0 || cur_weakness >= weakness_max {
        return 0;
    }

    let ratio_bp = if damage.WeaknessRatio.is_empty() {
        10_000
    } else {
        damage
            .WeaknessRatio
            .get(&damage.Element)
            .or_else(|| damage.WeaknessRatio.get(&0))
            .copied()
            .unwrap_or(10_000)
    };
    let raw_brk_ratio = attributes::get_attr_value(attacker_attr, EAttributeType::BreakWeaknessRatio);
    let brk_ratio = if raw_brk_ratio > 0 {
        raw_brk_ratio
    } else {
        10_000
    };
    let amount =
        ((weakness_lv * (f64::from(ratio_bp) / 10_000.0) * (f64::from(brk_ratio) / 10_000.0))
            .round() as i32)
            .max(0);
    let actual = amount.min(weakness_max - cur_weakness);
    if actual <= 0 {
        return 0;
    }

    let change = attributes::change_attr(
        target_attr,
        EAttributeType::WeaknessBuildUp,
        AttributeChangeType::Delta,
        AttributeValueType::Current,
        actual,
    );
    if change.is_empty() {
        return 0;
    }
    attributes::generate_attr_messages(
        combat_receive_pack,
        request.TargetEntityId,
        target_attr,
        &change,
    );
    scene.mark_fsm_dirty(request.TargetEntityId, wake_reason::ATTRIBUTE);
    0
}

pub fn damage_entity(
    combat_receive_pack: &mut Vec<CombatReceiveData>,
    request: &DamageExecuteRequest,
    damage: &Damage,
    assets: &Assets,
    scene: &mut Scene,
    buff_timers: Option<&mut BuffTimerScheduler>,
) -> DamageExecuteNotify {
    let effective_part_index = sanitize_part_index(scene, request.TargetEntityId, request.PartIndex);

    let mut base_notify = DamageExecuteNotify {
        DamageId: request.DamageId,
        AttackerEntityId: request.AttackerEntityId,
        TargetEntityId: request.TargetEntityId,
        PartIndex: effective_part_index,
        SkillLevel: request.SkillLevel,
        DamageContext: request.DamageContext.clone(),
        ElementType: damage.Element,
        ..Default::default()
    };

    let Some(&target_idx) = scene.net_id_map.get(&request.TargetEntityId) else {
        return base_notify;
    };
    if scene.entities[target_idx].attribute.is_none()
        || scene.entities[target_idx].buffs.is_none()
    {
        return base_notify;
    }

    let before_hp = attributes::get_attr_value(
        scene.entities[target_idx]
            .attribute
            .as_ref()
            .expect("checked"),
        EAttributeType::Life,
    );
    if before_hp <= 0 {
        return base_notify;
    }

    if damage.CalculateType == 0 && damage.ImmuneType != 0 {
        let invincible_tag_id =
            gameplay_tags::id_from_name(&assets.tables.gameplay_tag_remap, "功能.逻辑状态标识.无敌")
                .unwrap_or(1_918_148_596);
        if let Some(target_tags) = scene.entities[target_idx].tag.as_ref()
            && target_tags.has_tag(&assets.tables.gameplay_tag_parent, invincible_tag_id)
        {
            base_notify.ImmuneType = Some(EDamageImmune::Invincible);
            return base_notify;
        }
    }

    if scene.net_id_map.contains_key(&request.AttackerEntityId) {
        apply_attacker_energy(
            combat_receive_pack,
            request,
            damage,
            request.AttackerEntityId,
            scene,
        );
    }

    let attacker_attr = scene
        .entity(request.AttackerEntityId)
        .and_then(|e| e.attribute.clone())
        .unwrap_or_else(default_attacker_attributes);

    let target_attr_snap = scene.entities[target_idx]
        .attribute
        .as_ref()
        .expect("checked")
        .clone();

    let crit_rate = attributes::get_attr_value(&attacker_attr, EAttributeType::Crit);
    let is_crit = if damage.CalculateType == 0 {
        roll_is_crit(request, crit_rate)
    } else {
        false
    };
    let weakness_mult = weakness_damage_multiplier(
        assets,
        damage,
        request,
        &attacker_attr,
        &target_attr_snap,
        &scene.entities[target_idx],
    );
    let base_val = compute_formula_raw(
        assets,
        damage,
        &attacker_attr,
        &target_attr_snap,
        request.SkillLevel,
        is_crit,
        weakness_mult,
    );
    let fluct = fluctuation_factor(damage, request);

    let mut reported_value = 0;
    let mut killed_target = false;
    let mut change_life = 0;
    let mut shield_cover_damage = 0;

    if damage.CalculateType == 1 {
        let heal_change = attributes::get_attr_value(&attacker_attr, EAttributeType::HealChange);
        let healed_change =
            attributes::get_attr_value(&target_attr_snap, EAttributeType::HealedChange);
        let heal_mult = (1.0 + f64::from(heal_change + healed_change) / 10_000.0).max(0.0);
        let heal_amount = ((base_val * heal_mult * fluct).ceil() as i32).max(0);
        if heal_amount > 0 {
            let target_attr = scene.entities[target_idx].attribute.as_mut().unwrap();
            let target_change = attributes::change_attr(
                target_attr,
                EAttributeType::Life,
                AttributeChangeType::Delta,
                AttributeValueType::Current,
                heal_amount,
            );
            let after_hp = attributes::get_attr_value(target_attr, EAttributeType::Life);
            change_life = after_hp - before_hp;
            reported_value = heal_amount;
            if !target_change.is_empty() {
                attributes::generate_attr_messages(
                    combat_receive_pack,
                    request.TargetEntityId,
                    target_attr,
                    &target_change,
                );
                scene.mark_fsm_dirty(request.TargetEntityId, wake_reason::ATTRIBUTE);
            }
        }
    } else if damage.CalculateType == 2 {
        let drain = ((base_val * fluct).ceil() as i32).max(0);
        if drain > 0 && before_hp > 1 {
            let actual_drain = drain.min(before_hp - 1);
            if actual_drain > 0 {
                let target_attr = scene.entities[target_idx].attribute.as_mut().unwrap();
                let target_change = attributes::change_attr(
                    target_attr,
                    EAttributeType::Life,
                    AttributeChangeType::Delta,
                    AttributeValueType::Current,
                    -actual_drain,
                );
                let after_hp = attributes::get_attr_value(target_attr, EAttributeType::Life);
                change_life = after_hp - before_hp;
                reported_value = actual_drain;
                if !target_change.is_empty() {
                    attributes::generate_attr_messages(
                        combat_receive_pack,
                        request.TargetEntityId,
                        target_attr,
                        &target_change,
                    );
                    scene.mark_fsm_dirty(request.TargetEntityId, wake_reason::ATTRIBUTE);
                }
            }
        }
    } else {
        let raw_final = base_val * fluct;
        let final_damage = if raw_final > 0.0 {
            (raw_final.ceil() as i32).max(1)
        } else {
            0
        };
        reported_value = final_damage;

        if final_damage > 0 {
            let mut hp_damage = final_damage;
            let has_shields = scene.entities[target_idx]
                .buffs
                .as_ref()
                .is_some_and(|b| !b.shields.is_empty());
            if has_shields {
                let s_chg = attributes::get_attr_value(
                    &attacker_attr,
                    EAttributeType::SheildDamageChange,
                );
                let s_red = attributes::get_attr_value(
                    &target_attr_snap,
                    EAttributeType::SheildDamageReduce,
                );
                let shield_mult = (f64::from(10_000 + s_chg - s_red) / 10_000.0).max(0.1);
                let shield_raw = ((f64::from(final_damage) * shield_mult).round() as i32).max(1);

                let storage = &mut scene.entities[target_idx];
                let absorbed = buff_helper::absorb_damage_with_shields(
                    combat_receive_pack,
                    request.TargetEntityId,
                    storage.buffs.as_mut().unwrap(),
                    storage.attribute.as_mut().unwrap(),
                    shield_raw,
                );
                let equiv_hp = final_damage.min((f64::from(absorbed) / shield_mult).round() as i32);
                shield_cover_damage = equiv_hp;
                hp_damage = (final_damage - equiv_hp).max(0);
            }

            if hp_damage > 0 {
                let (after_hp, changed) = {
                    let target_attr = scene.entities[target_idx].attribute.as_mut().unwrap();
                    let target_change = attributes::change_attr(
                        target_attr,
                        EAttributeType::Life,
                        AttributeChangeType::Delta,
                        AttributeValueType::Current,
                        -hp_damage,
                    );
                    let after_hp = attributes::get_attr_value(target_attr, EAttributeType::Life);
                    change_life = after_hp - before_hp;
                    if !target_change.is_empty() {
                        attributes::generate_attr_messages(
                            combat_receive_pack,
                            request.TargetEntityId,
                            target_attr,
                            &target_change,
                        );
                    }
                    (after_hp, !target_change.is_empty())
                };
                if changed && after_hp > 0 {
                    scene.mark_fsm_dirty(request.TargetEntityId, wake_reason::ATTRIBUTE);
                }
                let target_is_monster = scene.entity_handle(request.TargetEntityId).is_some_and(|h| {
                    scene.entities[h.index].config.entity_type
                        == crate::component::entity::EntityType::Monster
                });
                if after_hp <= 0 {
                    killed_target = true;
                    let removed_handles = buff_helper::handle_entity_death(
                        combat_receive_pack,
                        scene,
                        assets,
                        request.TargetEntityId,
                    );
                    if let Some(timers) = buff_timers {
                        for handle_id in removed_handles {
                            timers.cancel_handle(request.TargetEntityId, handle_id);
                        }
                    }
                } else if target_is_monster {
                    scene.set_battle_entity_active(request.TargetEntityId, true);
                }
            } else if scene.entity_handle(request.TargetEntityId).is_some_and(|h| {
                scene.entities[h.index].config.entity_type
                    == crate::component::entity::EntityType::Monster
            }) {
                scene.set_battle_entity_active(request.TargetEntityId, true);
            }
            let _ = scene.append_battle_state_notify(combat_receive_pack);
        }
    }

    let mut change_weakness = 0;
    if damage.CalculateType == 0 && !killed_target {
        if apply_hardness_and_rage_on_hit(
            combat_receive_pack,
            assets,
            damage,
            request,
            &attacker_attr,
            scene,
            target_idx,
        ) {
            scene.mark_fsm_dirty(request.TargetEntityId, wake_reason::ATTRIBUTE);
        }
        change_weakness = apply_weakness_on_hit(
            combat_receive_pack,
            assets,
            damage,
            request,
            &attacker_attr,
            scene,
            target_idx,
        );
    }

    DamageExecuteNotify {
        DamageId: request.DamageId,
        AttackerEntityId: request.AttackerEntityId,
        TargetEntityId: request.TargetEntityId,
        Damage: reported_value,
        PartIndex: effective_part_index,
        IsCrit: is_crit,
        KilledTarget: killed_target,
        ShieldCoverDamage: shield_cover_damage,
        SkillLevel: request.SkillLevel,
        DamageContext: request.DamageContext.clone(),
        ImmuneType: None,
        ElementType: damage.Element,
        ChangeLife: change_life,
        ChangeWeakness: change_weakness,
        Bop: 0,
    }
}
