use sonoro_gamesv_data::Assets;
use sonoro_gamesv_data::tables::buff::{Buff, DurationPolicy};
use sonoro_proto::enums::{EAttributeType, EShieldUpdateType, LivingStatus};
use sonoro_proto::pb::{
    CombatCommon, CombatNotifyData, CombatNotifyDataMessageOneof, CombatReceiveData,
    CombatReceiveDataMessageOneof, EntityLivingStatusNotify, FightBuffInformation,
    RemoveGameplayEffectNotify, ShieldUpdateInfo, ShieldUpdateNotify,
};
use sonoro_proto::wire::PbEnum;

use crate::component::entity::EntityComponentStorage;
use crate::component::entity::attribute::AttributeComponent;
use crate::component::entity::fight_buff::{AppliedBuffMod, FightBuffComponent, Shield};
use crate::fsm::types::wake_reason;
use crate::helpers::attributes::{
    self, AttributeChangeType, AttributeValueType, RelatedAttrKind,
};
use crate::scene::{Scene, now_millis};

const ATTR_COUNT: usize = 143;

pub fn configured_duration_seconds(buff_data: &Buff) -> f32 {
    buff_data.DurationMagnitude.first().copied().unwrap_or(0.0)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalculationPolicy {
    AdvancedMultiplyMagnitude1 = -1,
    AddValue = 0,
    AddRate = 1,
    AddFromAttr = 2,
    OverrideValue = 3,
    OverrideFromAttr = 4,
    AddTimeScaledValue = 5,
    AddTimeScaledRate = 6,
    AddFromAttrRate = 9,
}

fn is_base_scaled_attr(attr: EAttributeType) -> bool {
    matches!(
        attr,
        EAttributeType::LifeMax | EAttributeType::Atk | EAttributeType::Def
    )
}

fn is_pooled_attr(attr: EAttributeType) -> bool {
    matches!(
        attr,
        EAttributeType::Life
            | EAttributeType::Energy
            | EAttributeType::ElementEnergy
            | EAttributeType::Strength
            | EAttributeType::Tough
            | EAttributeType::Hardness
            | EAttributeType::Rage
            | EAttributeType::SpecialEnergy1
            | EAttributeType::SpecialEnergy2
            | EAttributeType::SpecialEnergy3
            | EAttributeType::SpecialEnergy4
            | EAttributeType::SpecialEnergy5
    )
}

fn resolve_policy(buff_data: &Buff) -> Option<CalculationPolicy> {
    let raw = *buff_data.CalculationPolicy.first()?;
    Some(match raw {
        -1 => CalculationPolicy::AdvancedMultiplyMagnitude1,
        0 => CalculationPolicy::AddValue,
        1 => CalculationPolicy::AddRate,
        2 => CalculationPolicy::AddFromAttr,
        3 => CalculationPolicy::OverrideValue,
        4 => CalculationPolicy::OverrideFromAttr,
        5 => CalculationPolicy::AddTimeScaledValue,
        6 => CalculationPolicy::AddTimeScaledRate,
        9 => CalculationPolicy::AddFromAttrRate,
        _ => CalculationPolicy::AddValue,
    })
}

fn magnitude_at_level(buff_data: &Buff, level: i32) -> i32 {
    if buff_data.ModifierMagnitude.is_empty() {
        return 0;
    }
    let idx = (level - 1).max(0) as usize;
    buff_data.ModifierMagnitude[idx.min(buff_data.ModifierMagnitude.len() - 1)]
}

fn resolve_from_attr_source<'a>(
    scene: Option<&'a Scene>,
    target_id: i64,
    instigator_id: i64,
    instigator_attr: Option<&'a AttributeComponent>,
    target_attr: &'a AttributeComponent,
    buff_data: &Buff,
) -> Option<&'a AttributeComponent> {
    if buff_data.CalculationPolicy.len() < 2 {
        return None;
    }
    let use_instigator = buff_data.CalculationPolicy.get(2).copied() == Some(1);
    if use_instigator && instigator_id != target_id {
        instigator_attr.or_else(|| scene?.entity(instigator_id)?.attribute.as_ref())
    } else {
        Some(target_attr)
    }
}

fn compute_from_attr_raw(
    source_attr: &AttributeComponent,
    buff_data: &Buff,
    level: i32,
) -> Option<i32> {
    let &based_attr_id = buff_data.CalculationPolicy.get(1)?;
    let modifier = f64::from(magnitude_at_level(buff_data, level));
    let raw_source = attributes::get_attr_by_id(source_attr, based_attr_id);
    let source_value = f64::from(raw_source);
    let min_required = buff_data
        .CalculationPolicy
        .get(5)
        .copied()
        .map_or(0.0, f64::from);
    if source_value < min_required {
        return None;
    }

    let conversion_factor = buff_data
        .CalculationPolicy
        .get(6)
        .copied()
        .map_or(1.0, |v| f64::from(v).max(1.0));
    let max_value = buff_data
        .CalculationPolicy
        .get(7)
        .copied()
        .map_or(f64::from(i32::MAX), f64::from);

    let calculated = ((source_value - min_required) / conversion_factor) * (modifier / 1e4);
    let scaled = if buff_data.Id == 1102 && buff_data.Period > 0.0 {
        // HardnessRecover is specified per second while Buff 1102 ticks on a 0.1s period.
        calculated * f64::from(buff_data.Period)
    } else {
        calculated
    };
    Some(scaled.min(max_value).trunc() as i32)
}

fn compute_from_attr_delta(
    scene: Option<&Scene>,
    target_id: i64,
    instigator_id: i64,
    target_attr: &AttributeComponent,
    buff_data: &Buff,
    level: i32,
) -> Option<i32> {
    let source_attr =
        resolve_from_attr_source(scene, target_id, instigator_id, None, target_attr, buff_data)?;
    compute_from_attr_raw(source_attr, buff_data, level)
}

fn compute_override_from_attr(
    source_attr: &AttributeComponent,
    buff_data: &Buff,
    level: i32,
) -> Option<i32> {
    let &based_attr_id = buff_data.CalculationPolicy.get(1)?;
    let src_val = attributes::get_attr_by_id(source_attr, based_attr_id);
    let modifier = magnitude_at_level(buff_data, level);
    Some((f64::from(src_val) * f64::from(modifier) / 1e4).round() as i32)
}

fn scale_rate_bp(attr_type: EAttributeType, target_attr: &AttributeComponent, bp: i32) -> i32 {
    if bp == 0 {
        return 0;
    }
    let reference = compute_rate_reference(attr_type, target_attr);
    (f64::from(reference) * f64::from(bp) / 1e4).round() as i32
}

fn compute_rate_reference(attr_type: EAttributeType, target_attr: &AttributeComponent) -> i32 {
    if is_base_scaled_attr(attr_type) {
        let idx = attr_type.to_i32() as usize;
        return target_attr.attributes.get(idx).map_or(0, |a| a.base);
    }
    if is_pooled_attr(attr_type) {
        if let Some(max_attr) = attributes::get_related_attr(attr_type, RelatedAttrKind::Max) {
            let max_val = attributes::get_attr_value(target_attr, max_attr);
            if max_val > 0 {
                return max_val;
            }
        }
        return attributes::get_attr_value(target_attr, attr_type);
    }
    10_000
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExecutionMode {
    Instant,
    Periodic,
}

pub fn execute_buff_effects(
    combat_receive_pack: &mut Vec<CombatReceiveData>,
    target_id: i64,
    instigator_id: i64,
    buff_data: &Buff,
    scene: &mut Scene,
) {
    execute_buff_effects_inner(
        ExecutionMode::Instant,
        combat_receive_pack,
        target_id,
        instigator_id,
        buff_data,
        scene,
        None,
        None,
    );
}

pub fn execute_buff_effects_with_assets_and_level(
    combat_receive_pack: &mut Vec<CombatReceiveData>,
    target_id: i64,
    instigator_id: i64,
    buff_data: &Buff,
    scene: &mut Scene,
    assets: &Assets,
    level: i32,
) {
    execute_buff_effects_inner(
        ExecutionMode::Instant,
        combat_receive_pack,
        target_id,
        instigator_id,
        buff_data,
        scene,
        Some(assets),
        Some(level),
    );
}

pub fn execute_periodic_buff_effects(
    combat_receive_pack: &mut Vec<CombatReceiveData>,
    target_id: i64,
    instigator_id: i64,
    buff_data: &Buff,
    scene: &mut Scene,
) {
    execute_buff_effects_inner(
        ExecutionMode::Periodic,
        combat_receive_pack,
        target_id,
        instigator_id,
        buff_data,
        scene,
        None,
        None,
    );
}

pub fn execute_periodic_with_assets(
    combat_receive_pack: &mut Vec<CombatReceiveData>,
    target_id: i64,
    instigator_id: i64,
    buff_data: &Buff,
    scene: &mut Scene,
    assets: &Assets,
) {
    execute_buff_effects_inner(
        ExecutionMode::Periodic,
        combat_receive_pack,
        target_id,
        instigator_id,
        buff_data,
        scene,
        Some(assets),
        None,
    );
}

fn execute_buff_effects_inner(
    mode: ExecutionMode,
    combat_receive_pack: &mut Vec<CombatReceiveData>,
    target_id: i64,
    instigator_id: i64,
    buff_data: &Buff,
    scene: &mut Scene,
    maybe_assets: Option<&Assets>,
    explicit_level: Option<i32>,
) {
    let Some(&idx) = scene.net_id_map.get(&target_id) else {
        tracing::debug!("couldnt find target entity for buff effect: {target_id}");
        return;
    };

    match mode {
        ExecutionMode::Instant => {
            if buff_data.DurationPolicy != DurationPolicy::Instant {
                return;
            }
        }
        ExecutionMode::Periodic => {
            if buff_data.Period <= 0.0 {
                return;
            }
        }
    }

    if scene.entities[idx].attribute.is_none() {
        return;
    }
    let Some(calc_policy) = resolve_policy(buff_data) else {
        return;
    };

    if mode == ExecutionMode::Periodic {
        let storage = &scene.entities[idx];
        let assets_ref = maybe_assets;
        if let Some(assets) = assets_ref {
            if !buff_data.OngoingTagIgnores.is_empty()
                && buff_data.OngoingTagIgnores.iter().any(|name| {
                    let tid = crate::helpers::gameplay_tags::id_from_name(
                        &assets.tables.gameplay_tag_remap,
                        name,
                    );
                    crate::helpers::damage::entity_has_gameplay_tag(storage, assets, tid)
                })
            {
                return;
            }
            if buff_data.Id == 1102 {
                let rage_tid = crate::helpers::gameplay_tags::id_from_name(
                    &assets.tables.gameplay_tag_remap,
                    "功能.功能制作.狂暴生效",
                );
                let palsy_tid = crate::helpers::gameplay_tags::id_from_name(
                    &assets.tables.gameplay_tag_remap,
                    "怪物.common.状态标识.瘫痪",
                );
                if crate::helpers::damage::entity_has_gameplay_tag(storage, assets, rage_tid)
                    || crate::helpers::damage::entity_has_gameplay_tag(storage, assets, palsy_tid)
                {
                    return;
                }
                if let Some(attr) = storage.attribute.as_ref() {
                    let cur_h = attributes::get_attr_value(attr, EAttributeType::Hardness);
                    let max_h = attributes::get_attr_value(attr, EAttributeType::HardnessMax);
                    if max_h <= 0 || cur_h >= max_h {
                        return;
                    }
                }
            }
        }
    }

    let (eff_level, eff_stacks) = {
        let active_buff = scene.entities[idx]
            .buffs
            .as_ref()
            .and_then(|b| b.get_by_buff_id(buff_data.Id));
        let eff_level = explicit_level
            .filter(|&lvl| lvl > 0)
            .or_else(|| active_buff.map(|b| b.Level.max(1)))
            .unwrap_or(1);
        let eff_stacks = if mode == ExecutionMode::Periodic {
            active_buff.map_or(1, |b| b.StackCount.max(1))
        } else {
            1
        };
        (eff_level, eff_stacks)
    };

    // Clone scene attribute snapshot for cross-entity source lookups if instigator differs.
    let source_attr_snapshot = if instigator_id != target_id {
        scene
            .entity(instigator_id)
            .and_then(|e| e.attribute.clone())
    } else {
        None
    };

    let lock_floor = maybe_assets.and_then(|assets| lock_hp_floor(&scene.entities[idx], assets));

    let (before_hp, after_hp, change) = {
        let target_attr = scene.entities[idx]
            .attribute
            .as_mut()
            .expect("checked above");
        let before_hp = attributes::get_attr_value(target_attr, EAttributeType::Life);

        let mut change = Vec::new();
        for _ in 0..eff_stacks {
            let step_change = calculate_buff_effects(
                target_id,
                instigator_id,
                source_attr_snapshot.as_ref(),
                target_attr,
                buff_data,
                eff_level,
                calc_policy,
            );
            for attr_t in step_change {
                if !change.contains(&attr_t) {
                    change.push(attr_t);
                }
            }
        }
        if let Some(floor) = lock_floor
            && before_hp > 0
        {
            let cur_after = attributes::get_attr_value(target_attr, EAttributeType::Life);
            let min_allowed = before_hp.min(floor);
            if cur_after < min_allowed {
                let _ = attributes::set_life_unconditional(target_attr, min_allowed);
                if min_allowed == before_hp {
                    change.retain(|a| *a != EAttributeType::Life);
                }
            }
        }
        let after_hp = attributes::get_attr_value(target_attr, EAttributeType::Life);
        if !change.is_empty() {
            attributes::generate_attr_messages(combat_receive_pack, target_id, target_attr, &change);
        }
        (before_hp, after_hp, change)
    };

    if !change.is_empty() && after_hp > 0 {
        scene.mark_fsm_dirty(target_id, wake_reason::ATTRIBUTE);
    }

    if before_hp > 0 && after_hp <= 0 {
        if let Some(assets) = maybe_assets {
            let _ = handle_entity_death(combat_receive_pack, scene, assets, target_id);
        } else if !scene.entities[idx].dead_notified {
            scene.entities[idx].dead_notified = true;
            scene.set_battle_entity_active(target_id, false);
            combat_receive_pack.push(CombatReceiveData {
                Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
                    CombatNotifyData {
                        CombatCommon: Some(CombatCommon {
                            EntityId: target_id,
                            ..Default::default()
                        }),
                        Message: Some(CombatNotifyDataMessageOneof::EntityLivingStatusNotify(
                            EntityLivingStatusNotify {
                                Id: target_id,
                                LivingStatus: Some(LivingStatus::Dead),
                                ..Default::default()
                            },
                        )),
                    },
                )),
            });
        }
        let due_ms = now_millis().saturating_add(300);
        scene.queue_killed_monster_removal_at(target_id, due_ms);
        let _ = scene.append_battle_state_notify(combat_receive_pack);
    }
}

fn calculate_buff_effects(
    target_id: i64,
    instigator_id: i64,
    instigator_attr: Option<&AttributeComponent>,
    target_attr: &mut AttributeComponent,
    buff_data: &Buff,
    level: i32,
    calc_policy: CalculationPolicy,
) -> Vec<EAttributeType> {
    if matches!(
        buff_data.GameAttributeID,
        EAttributeType::None | EAttributeType::MAX
    ) {
        return Vec::new();
    }
    match calc_policy {
        CalculationPolicy::AddValue => {
            let value = magnitude_at_level(buff_data, level);
            if value == 0 {
                return Vec::new();
            }
            attributes::change_attr(
                target_attr,
                buff_data.GameAttributeID,
                AttributeChangeType::Delta,
                AttributeValueType::Current,
                value,
            )
        }
        CalculationPolicy::AddRate => {
            let modifier = magnitude_at_level(buff_data, level);
            let delta = scale_rate_bp(buff_data.GameAttributeID, target_attr, modifier);
            if delta == 0 {
                return Vec::new();
            }
            attributes::change_attr(
                target_attr,
                buff_data.GameAttributeID,
                AttributeChangeType::Delta,
                AttributeValueType::Current,
                delta,
            )
        }
        CalculationPolicy::AddFromAttr => {
            let Some(delta) = compute_from_attr_delta_with_source(
                target_id,
                instigator_id,
                instigator_attr,
                target_attr,
                buff_data,
                level,
            ) else {
                return Vec::new();
            };
            if delta == 0 {
                return Vec::new();
            }
            attributes::change_attr(
                target_attr,
                buff_data.GameAttributeID,
                AttributeChangeType::Delta,
                AttributeValueType::Current,
                delta,
            )
        }
        CalculationPolicy::AddFromAttrRate => {
            let Some(bp) = compute_from_attr_delta_with_source(
                target_id,
                instigator_id,
                instigator_attr,
                target_attr,
                buff_data,
                level,
            ) else {
                return Vec::new();
            };
            let delta = scale_rate_bp(buff_data.GameAttributeID, target_attr, bp);
            if delta == 0 {
                return Vec::new();
            }
            attributes::change_attr(
                target_attr,
                buff_data.GameAttributeID,
                AttributeChangeType::Delta,
                AttributeValueType::Current,
                delta,
            )
        }
        CalculationPolicy::OverrideValue => {
            let value = magnitude_at_level(buff_data, level);
            attributes::change_attr(
                target_attr,
                buff_data.GameAttributeID,
                AttributeChangeType::Override,
                AttributeValueType::Current,
                value,
            )
        }
        CalculationPolicy::OverrideFromAttr => {
            let Some(source_attr) = resolve_from_attr_source(
                None,
                target_id,
                instigator_id,
                instigator_attr,
                target_attr,
                buff_data,
            ) else {
                return Vec::new();
            };
            let Some(value) = compute_override_from_attr(source_attr, buff_data, level) else {
                return Vec::new();
            };
            attributes::change_attr(
                target_attr,
                buff_data.GameAttributeID,
                AttributeChangeType::Override,
                AttributeValueType::Current,
                value,
            )
        }
        _ => Vec::new(),
    }
}

fn compute_from_attr_delta_with_source(
    target_id: i64,
    instigator_id: i64,
    instigator_attr: Option<&AttributeComponent>,
    target_attr: &AttributeComponent,
    buff_data: &Buff,
    level: i32,
) -> Option<i32> {
    let source_attr = resolve_from_attr_source(
        None,
        target_id,
        instigator_id,
        instigator_attr,
        target_attr,
        buff_data,
    )?;
    compute_from_attr_raw(source_attr, buff_data, level)
}

pub fn sync_continuous_buff_modifiers(
    combat_receive_pack: &mut Vec<CombatReceiveData>,
    scene: &mut Scene,
    entity_id: i64,
    assets: &Assets,
) {
    let Some(&idx) = scene.net_id_map.get(&entity_id) else {
        return;
    };
    let (Some(buff_infos), Some(applied_mods), Some(attr_snapshot)) = (
        scene.entities[idx].buffs.as_ref().map(|b| b.buffs.clone()),
        scene.entities[idx]
            .buffs
            .as_ref()
            .map(|b| b.applied_buff_mods.clone()),
        scene.entities[idx].attribute.clone(),
    ) else {
        return;
    };
    let prebaked_handles = scene.entities[idx]
        .buffs
        .as_ref()
        .map(|b| b.prebaked_handles.clone())
        .unwrap_or_default();

    let mut prev = vec![0i64; ATTR_COUNT];
    for m in &applied_mods {
        if m.attr_idx >= 0 && (m.attr_idx as usize) < ATTR_COUNT {
            prev[m.attr_idx as usize] = i64::from(m.delta);
        }
    }

    // Build an unbuffed view of the entity's own attributes so self-referencing
    // attr-based policies (e.g. LifeMax = LifeMax * 200%) remain idempotent across re-syncs.
    let mut unbuffed_self_attr = attr_snapshot.clone();
    for i in 0..ATTR_COUNT {
        if prev[i] != 0 {
            let inc = (i64::from(unbuffed_self_attr.attributes[i].increment) - prev[i])
                .clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32;
            unbuffed_self_attr.attributes[i].increment = inc;
            unbuffed_self_attr.attributes[i].current =
                unbuffed_self_attr.attributes[i].base.saturating_add(inc);
        }
    }

    let mut desired = vec![0i64; ATTR_COUNT];
    let mut override_targets: Vec<Option<i64>> = vec![None; ATTR_COUNT];
    for info in &buff_infos {
        let is_prebaked = prebaked_handles.contains(&info.HandleId);
        if !info.IsActive && !is_prebaked {
            continue;
        }
        let Some(buff_data) = assets.tables.buff.get_data_by_id(&info.BuffId) else {
            continue;
        };
        if buff_data.DurationPolicy == DurationPolicy::Instant || buff_data.Period > 0.0 {
            continue;
        }
        if matches!(
            buff_data.GameAttributeID,
            EAttributeType::None | EAttributeType::MAX
        ) {
            continue;
        }

        let attr_idx = buff_data.GameAttributeID.to_i32() as usize;
        if attr_idx >= ATTR_COUNT {
            continue;
        }

        let stacks = if is_prebaked {
            if info.IsActive {
                (info.StackCount - 1).max(0)
            } else {
                -1
            }
        } else {
            info.StackCount.max(1)
        };
        if stacks == 0 {
            continue;
        }

        let Some(policy) = resolve_policy(&buff_data) else {
            continue;
        };
        let level = if info.Level > 0 { info.Level } else { 1 };
        let use_unbuffed_self = buff_data.CalculationPolicy.get(1).copied()
            == Some(buff_data.GameAttributeID.to_i32())
            && !(buff_data.CalculationPolicy.get(2).copied() == Some(1)
                && info.InstigatorId != entity_id);
        let target_src_attr = if use_unbuffed_self {
            &unbuffed_self_attr
        } else {
            &attr_snapshot
        };
        let per_stack: i64 = match policy {
            CalculationPolicy::AddValue => i64::from(magnitude_at_level(&buff_data, level)),
            CalculationPolicy::AddRate => {
                let modifier = magnitude_at_level(&buff_data, level);
                i64::from(scale_rate_bp(
                    buff_data.GameAttributeID,
                    &attr_snapshot,
                    modifier,
                ))
            }
            CalculationPolicy::AddFromAttr => i64::from(
                compute_from_attr_delta(
                    Some(scene),
                    entity_id,
                    info.InstigatorId,
                    target_src_attr,
                    &buff_data,
                    level,
                )
                .unwrap_or(0),
            ),
            CalculationPolicy::AddFromAttrRate => {
                let bp = compute_from_attr_delta(
                    Some(scene),
                    entity_id,
                    info.InstigatorId,
                    target_src_attr,
                    &buff_data,
                    level,
                )
                .unwrap_or(0);
                i64::from(scale_rate_bp(buff_data.GameAttributeID, &attr_snapshot, bp))
            }
            CalculationPolicy::OverrideValue => {
                if !is_prebaked {
                    override_targets[attr_idx] =
                        Some(i64::from(magnitude_at_level(&buff_data, level)));
                }
                continue;
            }
            CalculationPolicy::OverrideFromAttr => {
                if !is_prebaked
                    && let Some(source_attr) = resolve_from_attr_source(
                        Some(scene),
                        entity_id,
                        info.InstigatorId,
                        None,
                        target_src_attr,
                        &buff_data,
                    )
                    && let Some(override_val) =
                        compute_override_from_attr(source_attr, &buff_data, level)
                {
                    override_targets[attr_idx] = Some(i64::from(override_val));
                }
                continue;
            }
            _ => 0,
        };
        desired[attr_idx] += per_stack * i64::from(stacks);
    }

    for i in 0..ATTR_COUNT {
        if let Some(override_val) = override_targets[i] {
            let non_buff_total = i64::from(attr_snapshot.attributes[i].base)
                + (i64::from(attr_snapshot.attributes[i].increment) - prev[i]);
            desired[i] += override_val - non_buff_total;
        }
    }

    let mut next_mods = Vec::new();
    let mut changed_attrs = Vec::new();
    let attr_comp = scene.entities[idx]
        .attribute
        .as_mut()
        .expect("checked above");

    for i in 0..ATTR_COUNT {
        let d_val = desired[i];
        let p_val = prev[i];
        if d_val != 0 {
            next_mods.push(AppliedBuffMod {
                attr_idx: i as i32,
                delta: d_val.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32,
            });
        }
        let diff = d_val - p_val;
        if diff != 0 {
            let clamped_diff = diff.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32;
            let attr_type = EAttributeType::from_i32_or_default(i as i32);
            let ch = attributes::change_attr(
                attr_comp,
                attr_type,
                AttributeChangeType::Delta,
                AttributeValueType::Increment,
                clamped_diff,
            );
            changed_attrs.extend(ch);
        }
    }

    if !changed_attrs.is_empty() {
        attributes::generate_attr_messages(
            combat_receive_pack,
            entity_id,
            attr_comp,
            &changed_attrs,
        );
    }
    if let Some(buffs) = scene.entities[idx].buffs.as_mut() {
        buffs.set_applied_buff_mods(next_mods);
    }
    if !changed_attrs.is_empty() {
        scene.mark_fsm_dirty(entity_id, wake_reason::ATTRIBUTE);
    }
}

pub fn sync_buff_shield(
    combat_receive_pack: &mut Vec<CombatReceiveData>,
    scene: &mut Scene,
    assets: &Assets,
    entity_id: i64,
    buff_info: &FightBuffInformation,
    buff_data: &Buff,
) {
    if buff_data.ExtraEffectID != 11 || buff_data.ExtraEffectParameters.is_empty() {
        return;
    }
    let Ok(shield_id) = buff_data.ExtraEffectParameters[0].trim().parse::<i64>() else {
        return;
    };
    let Some(shield_cfg) = assets.tables.shield.get_data_by_id(&shield_id) else {
        return;
    };
    let Some(&idx) = scene.net_id_map.get(&entity_id) else {
        return;
    };
    if scene.entities[idx].attribute.is_none() || scene.entities[idx].buffs.is_none() {
        return;
    }

    let had_existing = scene.entities[idx]
        .buffs
        .as_ref()
        .is_some_and(|b| b.shields.iter().any(|s| s.handle_id == buff_info.HandleId));

    let mut updates = Vec::new();
    if !buff_info.IsActive {
        if !had_existing {
            return;
        }
        if let Some(buffs) = scene.entities[idx].buffs.as_mut() {
            buffs.remove_shield_by_handle(buff_info.HandleId);
        }
        updates.push(ShieldUpdateInfo {
            Handle: buff_info.HandleId,
            ConfigId: shield_id as i32,
            ShieldValue: 0,
            UpdateType: Some(EShieldUpdateType::EShieldUpdateTypeDel),
        });
    } else {
        let ref_id = if shield_cfg.RefrenceTarget == 0 && buff_info.InstigatorId != 0 {
            buff_info.InstigatorId
        } else {
            entity_id
        };
        let ref_val = scene
            .entity(ref_id)
            .and_then(|e| e.attribute.as_ref())
            .or(scene.entities[idx].attribute.as_ref())
            .map_or(0, |attr| {
                attributes::get_attr_by_id(attr, shield_cfg.RefrenceAttribute).max(0)
            });

        let lv_idx = (buff_info.Level - 1).max(0) as usize;
        let rate = if shield_cfg.AttributeRate.is_empty() {
            0.0
        } else {
            shield_cfg.AttributeRate[lv_idx.min(shield_cfg.AttributeRate.len() - 1)]
        };
        let flat = if shield_cfg.Value.is_empty() {
            0
        } else {
            shield_cfg.Value[lv_idx.min(shield_cfg.Value.len() - 1)]
        };
        let stacks = buff_info.StackCount.max(1);
        let raw_hp =
            ((f64::from(ref_val) * f64::from(rate) + f64::from(flat)).round() as i32).max(1)
                * stacks;

        if let Some(buffs) = scene.entities[idx].buffs.as_mut() {
            buffs.upsert_shield(Shield {
                handle_id: buff_info.HandleId,
                shield_id,
                buff_handle_id: buff_info.HandleId,
                current: raw_hp,
                max: raw_hp,
                priority: shield_cfg.Priority,
            });
        }
        updates.push(ShieldUpdateInfo {
            Handle: buff_info.HandleId,
            ConfigId: shield_id as i32,
            ShieldValue: raw_hp,
            UpdateType: Some(if had_existing {
                EShieldUpdateType::EShieldUpdateTypeModify
            } else {
                EShieldUpdateType::EShieldUpdateTypeAdd
            }),
        });
    }

    combat_receive_pack.push(CombatReceiveData {
        Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
            CombatNotifyData {
                CombatCommon: Some(CombatCommon {
                    EntityId: entity_id,
                    ..Default::default()
                }),
                Message: Some(CombatNotifyDataMessageOneof::ShieldUpdateNotify(
                    ShieldUpdateNotify { Shields: updates },
                )),
            },
        )),
    });
    let ent = &mut scene.entities[idx];
    if let (Some(buffs), Some(attr)) = (ent.buffs.as_ref(), ent.attribute.as_mut()) {
        sync_shield_attribute(combat_receive_pack, entity_id, buffs, attr);
    }
}

pub fn append_shield_del_for_handle(
    combat_receive_pack: &mut Vec<CombatReceiveData>,
    entity_id: i64,
    buff_comp: &mut FightBuffComponent,
    maybe_attr: Option<&mut AttributeComponent>,
    handle_id: i32,
) {
    let updates: Vec<ShieldUpdateInfo> = buff_comp
        .shields
        .iter()
        .filter(|entry| entry.handle_id == handle_id || entry.buff_handle_id == handle_id)
        .map(|entry| ShieldUpdateInfo {
            Handle: entry.handle_id,
            ConfigId: entry.shield_id as i32,
            ShieldValue: 0,
            UpdateType: Some(EShieldUpdateType::EShieldUpdateTypeDel),
        })
        .collect();
    if updates.is_empty() {
        return;
    }

    buff_comp.remove_shield_by_handle(handle_id);
    combat_receive_pack.push(CombatReceiveData {
        Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
            CombatNotifyData {
                CombatCommon: Some(CombatCommon {
                    EntityId: entity_id,
                    ..Default::default()
                }),
                Message: Some(CombatNotifyDataMessageOneof::ShieldUpdateNotify(
                    ShieldUpdateNotify { Shields: updates },
                )),
            },
        )),
    });
    if let Some(attr) = maybe_attr {
        sync_shield_attribute(combat_receive_pack, entity_id, buff_comp, attr);
    }
}

pub fn sync_shield_attribute(
    combat_receive_pack: &mut Vec<CombatReceiveData>,
    entity_id: i64,
    buff_comp: &FightBuffComponent,
    attr_comp: &mut AttributeComponent,
) {
    let new_shield = buff_comp.total_shield();
    let cur_shield = attributes::get_attr_value(attr_comp, EAttributeType::Sheild);
    if new_shield == cur_shield {
        return;
    }
    let changed = attributes::change_attr(
        attr_comp,
        EAttributeType::Sheild,
        AttributeChangeType::Override,
        AttributeValueType::Current,
        new_shield,
    );
    attributes::generate_attr_messages(combat_receive_pack, entity_id, attr_comp, &changed);
}

pub fn absorb_damage_with_shields(
    combat_receive_pack: &mut Vec<CombatReceiveData>,
    entity_id: i64,
    buff_comp: &mut FightBuffComponent,
    attr_comp: &mut AttributeComponent,
    raw_damage: i32,
) -> i32 {
    if raw_damage <= 0 || buff_comp.shields.is_empty() {
        return 0;
    }

    let mut remaining = raw_damage;
    let mut total_covered = 0;
    let mut updates = Vec::new();
    let mut depleted_handles = Vec::new();

    while remaining > 0 {
        let best_idx = buff_comp
            .shields
            .iter()
            .enumerate()
            .filter(|(_, s)| s.current > 0)
            .min_by_key(|(idx, s)| (std::cmp::Reverse(s.priority), *idx))
            .map(|(idx, _)| idx);
        let Some(idx) = best_idx else {
            break;
        };
        let entry = &mut buff_comp.shields[idx];
        let absorbed = remaining.min(entry.current);
        entry.current -= absorbed;
        remaining -= absorbed;
        total_covered += absorbed;

        let is_depleted = entry.current <= 0;
        updates.push(ShieldUpdateInfo {
            Handle: entry.handle_id,
            ConfigId: entry.shield_id as i32,
            ShieldValue: entry.current.max(0),
            UpdateType: Some(if is_depleted {
                EShieldUpdateType::EShieldUpdateTypeDel
            } else {
                EShieldUpdateType::EShieldUpdateTypeModify
            }),
        });
        if is_depleted {
            depleted_handles.push(entry.handle_id);
        }
    }

    for handle_id in depleted_handles {
        buff_comp.remove_shield_by_handle(handle_id);
    }

    if !updates.is_empty() {
        combat_receive_pack.push(CombatReceiveData {
            Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
                CombatNotifyData {
                    CombatCommon: Some(CombatCommon {
                        EntityId: entity_id,
                        ..Default::default()
                    }),
                    Message: Some(CombatNotifyDataMessageOneof::ShieldUpdateNotify(
                        ShieldUpdateNotify { Shields: updates },
                    )),
                },
            )),
        });
        sync_shield_attribute(combat_receive_pack, entity_id, buff_comp, attr_comp);
    }

    total_covered
}

pub fn clear_dead_remove_buffs(
    combat_receive_pack: &mut Vec<CombatReceiveData>,
    scene: &mut Scene,
    entity_id: i64,
    assets: &Assets,
) -> Vec<i32> {
    let Some(&idx) = scene.net_id_map.get(&entity_id) else {
        return Vec::new();
    };
    let removed_handles: Vec<i32> = {
        let Some(buffs) = scene.entities[idx].buffs.as_ref() else {
            return Vec::new();
        };
        buffs
            .buffs
            .iter()
            .filter(|info| {
                assets
                    .tables
                    .buff
                    .get_data_by_id(&info.BuffId)
                    .is_some_and(|b| b.DeadRemove)
            })
            .map(|info| info.HandleId)
            .collect()
    };
    if removed_handles.is_empty() {
        return Vec::new();
    }

    let mut shield_updates = Vec::new();
    if let Some(buffs) = scene.entities[idx].buffs.as_mut() {
        for &handle_id in &removed_handles {
            for shield in &buffs.shields {
                if shield.handle_id == handle_id || shield.buff_handle_id == handle_id {
                    shield_updates.push(ShieldUpdateInfo {
                        Handle: shield.handle_id,
                        ConfigId: shield.shield_id as i32,
                        ShieldValue: 0,
                        UpdateType: Some(EShieldUpdateType::EShieldUpdateTypeDel),
                    });
                }
            }
            buffs.remove_by_handle(handle_id);
            combat_receive_pack.push(CombatReceiveData {
                Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
                    CombatNotifyData {
                        CombatCommon: Some(CombatCommon {
                            EntityId: entity_id,
                            ..Default::default()
                        }),
                        Message: Some(CombatNotifyDataMessageOneof::RemoveGameplayEffectNotify(
                            RemoveGameplayEffectNotify {
                                EntityId: entity_id,
                                Handle: handle_id,
                                InstigatorId: 0,
                            },
                        )),
                    },
                )),
            });
        }
    }

    if !shield_updates.is_empty() {
        combat_receive_pack.push(CombatReceiveData {
            Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
                CombatNotifyData {
                    CombatCommon: Some(CombatCommon {
                        EntityId: entity_id,
                        ..Default::default()
                    }),
                    Message: Some(CombatNotifyDataMessageOneof::ShieldUpdateNotify(
                        ShieldUpdateNotify {
                            Shields: shield_updates,
                        },
                    )),
                },
            )),
        });
        let storage = &mut scene.entities[idx];
        if let (Some(buffs), Some(attr)) = (storage.buffs.as_ref(), storage.attribute.as_mut()) {
            sync_shield_attribute(combat_receive_pack, entity_id, buffs, attr);
        }
    }

    sync_continuous_buff_modifiers(combat_receive_pack, scene, entity_id, assets);
    removed_handles
}

/// Compute the minimum HP floor enforced by any active Lock-HP (`ExtraEffectID == 16`)
/// buff on `entity`, matching the client's `((LifeMax * Grow1[lvl]) / 10000 + Grow2[lvl]).clamp(1, LifeMax)`.
pub fn lock_hp_floor(entity: &EntityComponentStorage, assets: &Assets) -> Option<i32> {
    let attr = entity.attribute.as_ref()?;
    let buffs = entity.buffs.as_ref()?;
    let life_max = attributes::get_attr_value(attr, EAttributeType::LifeMax);
    if life_max <= 0 {
        return None;
    }

    let life_attr_id = EAttributeType::Life.to_i32();
    let mut max_floor: Option<i32> = None;
    for info in &buffs.buffs {
        let is_prebaked = buffs.prebaked_handles.contains(&info.HandleId);
        if !info.IsActive && !is_prebaked {
            continue;
        }
        let Some(buff_data) = assets.tables.buff.get_data_by_id(&info.BuffId) else {
            continue;
        };
        if buff_data.ExtraEffectID != 16 {
            continue;
        }
        let targets_life = buff_data.ExtraEffectParameters.is_empty()
            || buff_data.ExtraEffectParameters[0].trim().parse::<i32>() == Ok(life_attr_id);
        if !targets_life {
            continue;
        }

        let level = if info.Level > 0 { info.Level } else { 1 };
        let idx = (level - 1).max(0) as usize;
        let pct = i64::from(
            buff_data
                .ExtraEffectParametersGrow1
                .get(idx)
                .or_else(|| buff_data.ExtraEffectParametersGrow1.last())
                .copied()
                .unwrap_or(0),
        );
        let flat = i64::from(
            buff_data
                .ExtraEffectParametersGrow2
                .get(idx)
                .or_else(|| buff_data.ExtraEffectParametersGrow2.last())
                .copied()
                .unwrap_or(0),
        );
        let floor = ((i64::from(life_max) * pct) / 10_000 + flat)
            .clamp(1, i64::from(life_max)) as i32;
        max_floor = Some(max_floor.map_or(floor, |cur| cur.max(floor)));
    }
    max_floor
}

pub fn handle_entity_death(
    combat_receive_pack: &mut Vec<CombatReceiveData>,
    scene: &mut Scene,
    assets: &Assets,
    entity_id: i64,
) -> Vec<i32> {
    if let Some(&idx) = scene.net_id_map.get(&entity_id) {
        if scene.entities[idx].dead_notified {
            return Vec::new();
        }
        scene.entities[idx].dead_notified = true;
    }
    scene.set_battle_entity_active(entity_id, false);
    combat_receive_pack.push(CombatReceiveData {
        Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
            CombatNotifyData {
                CombatCommon: Some(CombatCommon {
                    EntityId: entity_id,
                    ..Default::default()
                }),
                Message: Some(CombatNotifyDataMessageOneof::EntityLivingStatusNotify(
                    EntityLivingStatusNotify {
                        Id: entity_id,
                        LivingStatus: Some(LivingStatus::Dead),
                        ..Default::default()
                    },
                )),
            },
        )),
    });
    clear_dead_remove_buffs(combat_receive_pack, scene, entity_id, assets)
}
