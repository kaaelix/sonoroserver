use sonoro_gamesv_data::tables::ai_state_machine_config::{
    ConditionBuffStack, ConditionPartLife, ConditionTimer, StateMachineCondition,
    StateMachineTransition,
};
use sonoro_proto::enums::EAttributeType;
use sonoro_proto::wire::PbEnum;

use crate::component::entity::fsm::FsmComponent;
use crate::fsm::state_hierarchy;
use crate::fsm::types::{ConditionKey, EvalContext, WakeMask, wake_reason};
use crate::helpers::gameplay_tags;

pub fn transition_uses_dissolve_combine(conditions: &[StateMachineCondition]) -> bool {
    let Some(root) = find_condition(conditions, 0) else {
        return false;
    };
    condition_uses_dissolve_combine(conditions, root, 0)
}

fn condition_uses_dissolve_combine(
    conditions: &[StateMachineCondition],
    condition: &StateMachineCondition,
    depth: usize,
) -> bool {
    if depth > 12 {
        return false;
    }
    if condition.CondCheckDissolveCombine.is_some() {
        return true;
    }
    if let Some(and) = &condition.CondAnd {
        return and.Conditions.iter().any(|&idx| {
            find_condition(conditions, idx)
                .is_some_and(|c| condition_uses_dissolve_combine(conditions, c, depth + 1))
        });
    }
    if let Some(or) = &condition.CondOr {
        return or.Conditions.iter().any(|&idx| {
            find_condition(conditions, idx)
                .is_some_and(|c| condition_uses_dissolve_combine(conditions, c, depth + 1))
        });
    }
    false
}

pub fn dependency_mask(
    conditions: &[StateMachineCondition],
    condition: &StateMachineCondition,
    depth: usize,
) -> WakeMask {
    if depth > 12 {
        return 0;
    }
    if condition.IsClient.unwrap_or(false) {
        return wake_reason::CLIENT_PASS;
    }

    let mut mask: WakeMask = 0;
    if let Some(and) = &condition.CondAnd {
        for &child_idx in &and.Conditions {
            if let Some(child) = find_condition(conditions, child_idx) {
                mask |= dependency_mask(conditions, child, depth + 1);
            }
        }
        return mask;
    }
    if let Some(or) = &condition.CondOr {
        for &child_idx in &or.Conditions {
            if let Some(child) = find_condition(conditions, child_idx) {
                mask |= dependency_mask(conditions, child, depth + 1);
            }
        }
        return mask;
    }

    if condition.CondHpLessThan.is_some()
        || condition.CondAttrCompare.is_some()
        || condition.CondAttribute.is_some()
        || condition.CondAttributeRate.is_some()
    {
        mask |= wake_reason::ATTRIBUTE;
    }
    if condition.CondSkillEnd.is_some() {
        mask |= wake_reason::SKILL_END;
    }
    if condition.CondBBValueCompare.is_some() {
        mask |= wake_reason::BLACKBOARD;
    }
    if condition.CondWaitClient.is_some() || condition.Type == Some(4) {
        mask |= wake_reason::CLIENT_PASS;
    }
    if condition.CondListenBeHit.is_some() {
        mask |= wake_reason::EVENT | wake_reason::BLACKBOARD | wake_reason::TAG;
    }
    if condition.CondTimer.is_some() {
        mask |= wake_reason::TIMER;
    }
    if condition.CondCheckState.is_some()
        || condition.CondCheckStateByName.is_some()
        || condition.CondCheckLastState.is_some()
    {
        mask |= wake_reason::STATE;
    }
    if condition.CondListenEvent.is_some() {
        mask |= wake_reason::EVENT;
    }
    if condition.CondCheckPositionState.is_some() {
        mask |= wake_reason::POSITION;
    }
    if condition.CondHate.is_some() || condition.Type == Some(20) {
        mask |= wake_reason::HATE;
    }
    if condition.CondTag.is_some() || condition.CondInstStateChange.is_some() {
        mask |= wake_reason::TAG;
    }
    if condition.CondBuffStack.is_some() {
        mask |= wake_reason::BUFF;
    }
    if condition.CondPartLife.is_some() || condition.CondCheckPartActivated.is_some() {
        mask |= wake_reason::PART;
    }
    if condition.CondCheckDissolveCombine.is_some() {
        mask |= wake_reason::DISSOLVE_COMBINE;
    }
    mask
}

#[allow(clippy::too_many_arguments)]
pub fn next_timer_deadline(
    comp: &FsmComponent,
    fsm_id: i32,
    transition: &StateMachineTransition,
    conditions: &[StateMachineCondition],
    condition: &StateMachineCondition,
    now_ms: i64,
    activation_state: i32,
    depth: usize,
) -> Option<i64> {
    if depth > 12 || condition.IsClient.unwrap_or(false) {
        return None;
    }
    if let Some(and) = &condition.CondAnd {
        let mut earliest: Option<i64> = None;
        for &child_idx in &and.Conditions {
            let Some(child) = find_condition(conditions, child_idx) else {
                continue;
            };
            if let Some(cand) = next_timer_deadline(
                comp,
                fsm_id,
                transition,
                conditions,
                child,
                now_ms,
                activation_state,
                depth + 1,
            ) {
                earliest = Some(earliest.map_or(cand, |e| e.min(cand)));
            }
        }
        return earliest;
    }
    if let Some(or) = &condition.CondOr {
        let mut earliest: Option<i64> = None;
        for &child_idx in &or.Conditions {
            let Some(child) = find_condition(conditions, child_idx) else {
                continue;
            };
            if let Some(cand) = next_timer_deadline(
                comp,
                fsm_id,
                transition,
                conditions,
                child,
                now_ms,
                activation_state,
                depth + 1,
            ) {
                earliest = Some(earliest.map_or(cand, |e| e.min(cand)));
            }
        }
        return earliest;
    }

    let timer = condition.CondTimer.as_ref()?;
    let activated_at = state_active_since_ms(comp, fsm_id, activation_state)?;
    let min_ms = duration_ms(timer.MinTime);
    let configured_max = duration_ms(timer.MaxTime.unwrap_or(timer.MinTime));
    let max_ms = configured_max.max(min_ms);
    let delay_ms = timer_delay_ms(
        state_hierarchy::canonical_state(comp, fsm_id),
        state_hierarchy::canonical_state(comp, transition.From),
        state_hierarchy::canonical_state(comp, transition.To),
        condition.Index,
        activated_at,
        min_ms,
        max_ms,
    );
    let deadline = activated_at.saturating_add(delay_ms);
    (deadline > now_ms).then_some(deadline)
}

pub fn find_condition(
    conditions: &[StateMachineCondition],
    index: i32,
) -> Option<&StateMachineCondition> {
    if index < 0 {
        return None;
    }
    conditions.get(index as usize)
}

#[allow(clippy::too_many_arguments)]
pub fn evaluate(
    comp: &FsmComponent,
    fsm_id: i32,
    transition: &StateMachineTransition,
    conditions: &[StateMachineCondition],
    condition: &StateMachineCondition,
    ctx: &EvalContext<'_>,
    activation_state: i32,
    depth: usize,
) -> bool {
    if depth > 12 {
        return false;
    }

    let resolved = state_hierarchy::resolve_override_states(
        comp,
        transition.From,
        transition.To,
        false,
    );
    if condition.IsClient.unwrap_or(false) {
        let key = ConditionKey {
            fsm_id: state_hierarchy::canonical_state(comp, fsm_id),
            from: resolved.from,
            to: resolved.to,
            index: condition.Index,
        };
        // `FsmConditionPassPush.Value` is already `ResultSelf == !Reverse` on the client
        // (AiStateMachineCondition.Result), so do not apply `condition.Reverse` a second time.
        return comp.pass_pool.contains(&key);
    }
    let result = if condition.Type == Some(4) {
        (ctx.flags & crate::fsm::types::EVAL_FLAG_WAIT_CLIENT) != 0
    } else if condition.Type == Some(20) {
        comp.in_hate
    } else if let Some(and) = &condition.CondAnd {
        and.Conditions.iter().all(|&child_idx| {
            find_condition(conditions, child_idx).is_some_and(|child| {
                evaluate(
                    comp,
                    fsm_id,
                    transition,
                    conditions,
                    child,
                    ctx,
                    activation_state,
                    depth + 1,
                )
            })
        })
    } else if let Some(or) = &condition.CondOr {
        or.Conditions.iter().any(|&child_idx| {
            find_condition(conditions, child_idx).is_some_and(|child| {
                evaluate(
                    comp,
                    fsm_id,
                    transition,
                    conditions,
                    child,
                    ctx,
                    activation_state,
                    depth + 1,
                )
            })
        })
    } else if condition.CondTrue.is_some() {
        true
    } else if let Some(hp) = &condition.CondHpLessThan {
        hp_less_than(ctx.attributes, hp.HpRatio)
    } else if condition.CondSkillEnd.is_some() {
        skill_ended(comp, fsm_id, transition.From)
    } else if let Some(bb) = &condition.CondBBValueCompare {
        let v1 = usize::try_from(bb.Key1)
            .ok()
            .and_then(|k1| comp.blackboard.get(k1).copied().flatten());
        let v2 = usize::try_from(bb.Key2)
            .ok()
            .and_then(|k2| comp.blackboard.get(k2).copied().flatten());
        match (v1, v2) {
            (Some(v1), Some(v2)) => compare_values(v1, v2, bb.Compare),
            _ => false,
        }
    } else if let Some(ac) = &condition.CondAttrCompare {
        let v1 = usize::try_from(ac.Attr1)
            .ok()
            .and_then(|a1| ctx.attributes.get(a1).copied());
        let v2 = usize::try_from(ac.Attr2)
            .ok()
            .and_then(|a2| ctx.attributes.get(a2).copied());
        match (v1, v2) {
            (Some(v1), Some(v2)) => compare_values(v1, v2, ac.Compare),
            _ => false,
        }
    } else if condition.CondWaitClient.is_some() {
        (ctx.flags & crate::fsm::types::EVAL_FLAG_WAIT_CLIENT) != 0
    } else if let Some(be_hit) = &condition.CondListenBeHit {
        listen_be_hit_matches(comp, be_hit)
    } else if let Some(timer) = &condition.CondTimer {
        timer_passes(
            comp,
            fsm_id,
            transition,
            activation_state,
            condition.Index,
            timer,
            ctx.now_ms,
        )
    } else if let Some(cs) = &condition.CondCheckState {
        let target = cs.TargetState;
        comp.runtime_nodes
            .iter()
            .any(|r| state_hierarchy::path_contains(comp, r.active(), target))
    } else if let Some(csn) = &condition.CondCheckStateByName {
        comp.runtime_nodes.iter().any(|r| {
            r.active()
                .iter()
                .any(|&sid| state_name_matches(comp, sid, &csn.TargetStateName))
        })
    } else if let Some(cls) = &condition.CondCheckLastState {
        comp.runtime_nodes.iter().any(|r| {
            r.previous()
                .iter()
                .any(|&sid| state_name_matches(comp, sid, &cls.TargetStateName))
        })
    } else if let Some(ev) = &condition.CondListenEvent {
        comp.event.as_deref().is_some_and(|e| e == ev.Event)
    } else if let Some(ps) = &condition.CondCheckPositionState {
        ctx.position_state == Some(ps.PositionState)
    } else if condition.CondHate.is_some() {
        comp.in_hate
    } else if let Some(attr) = &condition.CondAttribute {
        usize::try_from(attr.AttributeId)
            .ok()
            .and_then(|id| ctx.attributes.get(id).copied())
            .is_some_and(|val| {
                let v = f64::from(val);
                v >= f64::from(attr.Min) && v <= f64::from(attr.Max)
            })
    } else if let Some(rate) = &condition.CondAttributeRate {
        attr_rate_in_range(ctx.attributes, rate.AttributeId, rate.Denominator, rate.Min, rate.Max)
    } else if let Some(tag) = &condition.CondTag {
        tag.TagId.is_some_and(|tag_id| has_tag(comp, ctx, tag_id))
    } else if let Some(inst) = &condition.CondInstStateChange {
        instance_state_matches(ctx, inst.TagId)
    } else if let Some(buff) = &condition.CondBuffStack {
        buff_stack_in_range(ctx.buff_stacks, buff)
    } else if let Some(part_life) = &condition.CondPartLife {
        part_life_in_range(ctx.parts, part_life)
    } else if let Some(part_act) = &condition.CondCheckPartActivated {
        ctx.parts
            .iter()
            .find(|p| p.slice() == part_act.PartName.as_bytes())
            .is_some_and(|p| p.activated)
    } else if condition.CondCheckDissolveCombine.is_some() {
        comp.dissolve_combine_signal
    } else {
        false
    };

    if condition.Reverse { !result } else { result }
}

fn state_name_matches(comp: &FsmComponent, state: i32, name: &str) -> bool {
    if state_hierarchy::find_node(comp, state)
        .and_then(|n| n.Name.as_deref())
        .is_some_and(|n| n == name)
    {
        return true;
    }
    state_hierarchy::find_node_exact(comp, state)
        .and_then(|n| n.Name.as_deref())
        .is_some_and(|n| n == name)
}

fn custom_bb_int(comp: &FsmComponent, key: &str) -> i32 {
    comp.custom_blackboards
        .iter()
        .find(|(k, _)| k == key)
        .map_or(0, |(_, v)| *v)
}

fn listen_be_hit_matches(
    comp: &FsmComponent,
    be_hit: &sonoro_gamesv_data::tables::ai_state_machine_config::ConditionListenBeHit,
) -> bool {
    let hit_active =
        custom_bb_int(comp, "be_hit:active") > 0 || comp.event.as_deref() == Some("BeHit");
    if !hit_active {
        return false;
    }
    if be_hit.Parry && custom_bb_int(comp, "be_hit:parry") <= 0 {
        return false;
    }
    if be_hit.BreakWeakness && custom_bb_int(comp, "be_hit:break_weakness") <= 0 {
        return false;
    }
    if be_hit.VisionCounterAttackId != 0 {
        let vc = custom_bb_int(comp, "be_hit:vision_counter");
        if vc <= 0 || (vc != 1 && vc != be_hit.VisionCounterAttackId) {
            return false;
        }
    }
    let any_anim_filter = be_hit.NoHitAnimation
        || be_hit.SoftKnock
        || be_hit.HeavyKnock
        || be_hit.KnockUp
        || be_hit.KnockDown;
    if any_anim_filter {
        let anim = custom_bb_int(comp, "be_hit:anim");
        if anim >= 0 && comp.custom_blackboards.iter().any(|(k, _)| k == "be_hit:anim") {
            let matches_anim = (be_hit.NoHitAnimation && anim == 0)
                || (be_hit.SoftKnock && anim == 1)
                || (be_hit.HeavyKnock && anim == 2)
                || (be_hit.KnockUp && anim == 3)
                || (be_hit.KnockDown && anim == 4);
            if !matches_anim {
                return false;
            }
        }
    }
    true
}

fn hp_less_than(attributes: &[i32], hp_ratio: f32) -> bool {
    if !hp_ratio.is_finite() {
        return false;
    }
    let Some(&life) = attributes.get(EAttributeType::Life.to_i32() as usize) else {
        return false;
    };
    let Some(&life_max) = attributes.get(EAttributeType::LifeMax.to_i32() as usize) else {
        return false;
    };
    if life_max <= 0 {
        return false;
    }
    f64::from(life) * 10000.0 < f64::from(life_max) * f64::from(hp_ratio)
}

fn skill_ended(comp: &FsmComponent, fsm_id: i32, transition_source: i32) -> bool {
    let Some(runtime) = state_hierarchy::runtime_node(comp, fsm_id) else {
        return false;
    };
    let Some(leaf) = runtime.leaf() else {
        return false;
    };
    runtime.active_skill_ended && state_hierarchy::states_equivalent(comp, leaf, transition_source)
}

fn attr_rate_in_range(
    attributes: &[i32],
    attribute_id: i32,
    denominator: f32,
    min: f32,
    max: f32,
) -> bool {
    let Some(denom_id) = attribute_id_from_number(denominator) else {
        return false;
    };
    let Some(&num) = usize::try_from(attribute_id)
        .ok()
        .and_then(|id| attributes.get(id))
    else {
        return false;
    };
    let Some(&denom_val) = usize::try_from(denom_id)
        .ok()
        .and_then(|id| attributes.get(id))
    else {
        return false;
    };
    if denom_val == 0 {
        return false;
    }
    let rate = f64::from(num) * 10000.0 / f64::from(denom_val);
    rate >= f64::from(min) && rate <= f64::from(max)
}

fn attribute_id_from_number(value: f32) -> Option<i32> {
    if !value.is_finite() || value.trunc() != value {
        return None;
    }
    if value < i32::MIN as f32 || value > i32::MAX as f32 {
        return None;
    }
    Some(value as i32)
}

fn buff_stack_in_range(buff_stacks: &[(i64, i32)], condition: &ConditionBuffStack) -> bool {
    let stack = buff_stacks
        .iter()
        .find(|(id, _)| *id == condition.BuffId)
        .map_or(0, |(_, s)| *s);
    stack >= condition.MinStack && stack <= condition.MaxStack
}

fn part_life_in_range(parts: &[crate::fsm::types::PartState], condition: &ConditionPartLife) -> bool {
    let Some(part) = parts
        .iter()
        .find(|p| p.slice() == condition.PartName.as_bytes())
    else {
        return false;
    };
    let value = if condition.CheckRate {
        if part.life_max <= 0.0 {
            return false;
        }
        f64::from(part.life) * 10000.0 / f64::from(part.life_max)
    } else {
        f64::from(part.life)
    };
    value >= f64::from(condition.Min) && value <= f64::from(condition.Max)
}

fn timer_passes(
    comp: &FsmComponent,
    fsm_id: i32,
    transition: &StateMachineTransition,
    activation_state: i32,
    condition_index: i32,
    timer: &ConditionTimer,
    now_ms: i64,
) -> bool {
    let Some(activated_at) = state_active_since_ms(comp, fsm_id, activation_state) else {
        return false;
    };
    if now_ms < activated_at {
        return false;
    }
    let min_ms = duration_ms(timer.MinTime);
    let configured_max = duration_ms(timer.MaxTime.unwrap_or(timer.MinTime));
    let max_ms = configured_max.max(min_ms);
    let delay_ms = timer_delay_ms(
        state_hierarchy::canonical_state(comp, fsm_id),
        state_hierarchy::canonical_state(comp, transition.From),
        state_hierarchy::canonical_state(comp, transition.To),
        condition_index,
        activated_at,
        min_ms,
        max_ms,
    );
    now_ms - activated_at >= delay_ms
}

fn duration_ms(value: f32) -> i64 {
    if !value.is_finite() || value <= 0.0 {
        return 0;
    }
    let wide = f64::from(value);
    if wide >= i64::MAX as f64 {
        return i64::MAX;
    }
    wide.ceil() as i64
}

fn timer_delay_ms(
    fsm_id: i32,
    from: i32,
    to: i32,
    condition_index: i32,
    activated_at: i64,
    min_ms: i64,
    max_ms: i64,
) -> i64 {
    if max_ms <= min_ms {
        return min_ms;
    }
    let mut seed: u64 = activated_at as u64;
    seed ^= u64::from(fsm_id as u32).wrapping_mul(0x9E37_79B1_85EB_CA87);
    seed ^= u64::from(from as u32).wrapping_mul(0xC2B2_AE3D_27D4_EB4F);
    seed ^= u64::from(to as u32).wrapping_mul(0x1656_67B1_9E37_79F9);
    seed ^= u64::from(condition_index as u32).wrapping_mul(0x85EB_CA77_C2B2_AE63);
    seed ^= seed >> 12;
    seed ^= seed << 25;
    seed ^= seed >> 27;
    seed = seed.wrapping_mul(0x2545_F491_4F6C_DD1D);

    let span = ((i128::from(max_ms) - i128::from(min_ms)) as u128) + 1;
    let offset = (u128::from(seed) % span) as i128;
    i64::try_from(i128::from(min_ms) + offset).unwrap_or(i64::MAX)
}

fn has_tag(comp: &FsmComponent, ctx: &EvalContext<'_>, tag_id: i32) -> bool {
    if let Some(parents) = ctx.parent_tags {
        for tag in &comp.tags {
            if tag.count > 0
                && let Ok(actual) = i32::try_from(tag.id)
                && gameplay_tags::contains(parents, actual, tag_id)
            {
                return true;
            }
        }
        return ctx
            .tags
            .iter()
            .any(|&actual| gameplay_tags::contains(parents, actual, tag_id));
    }
    has_fsm_tag(comp, i64::from(tag_id)) || ctx.tags.contains(&tag_id)
}

fn instance_state_matches(ctx: &EvalContext<'_>, expected: i32) -> bool {
    if let Some(state_tag) = ctx.instance_state {
        if let Some(parents) = ctx.parent_tags {
            return gameplay_tags::contains(parents, state_tag, expected);
        }
        return state_tag == expected;
    }
    if let Some(parents) = ctx.parent_tags {
        return ctx
            .tags
            .iter()
            .any(|&actual| gameplay_tags::contains(parents, actual, expected));
    }
    ctx.tags.contains(&expected)
}

fn state_active_since_ms(comp: &FsmComponent, fsm_id: i32, activation_state: i32) -> Option<i64> {
    let runtime = state_hierarchy::runtime_node(comp, fsm_id)?;
    let target = state_hierarchy::canonical_state(comp, activation_state);
    for (idx, &state) in runtime.active().iter().enumerate() {
        if state_hierarchy::states_equivalent(comp, state, target) {
            return Some(runtime.active_since_ms[idx]);
        }
    }
    None
}

fn has_fsm_tag(comp: &FsmComponent, tag_id: i64) -> bool {
    comp.tags.iter().any(|t| t.id == tag_id && t.count > 0)
}

fn compare_values(lhs: i32, rhs: i32, op: i32) -> bool {
    match op {
        0 => lhs == rhs,
        1 => lhs != rhs,
        2 => lhs < rhs,
        3 => lhs <= rhs,
        4 => lhs > rhs,
        5 => lhs >= rhs,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sonoro_gamesv_data::fsm_graph_registry::Graph;

    use super::*;

    #[test]
    fn client_condition_does_not_reapply_reverse_on_pass_pool() {
        let mut comp = FsmComponent::from_graph(Arc::new(Graph::default()), 0).unwrap();
        let transition = StateMachineTransition {
            From: 10013,
            To: 10015,
            ..Default::default()
        };
        let cond = StateMachineCondition {
            Index: 0,
            Reverse: true,
            IsClient: Some(true),
            ..Default::default()
        };
        let ctx = EvalContext::default();

        // When client has NOT sent a passing FsmConditionPassPush, pass_pool is empty:
        // must evaluate to false even when Reverse is true.
        assert!(!evaluate(&comp, 10007, &transition, &[cond.clone()], &cond, &ctx, 10013, 0));

        // When client HAS sent FsmConditionPassPush { Value = true } (which already includes Reverse),
        // pass_pool contains the key and must evaluate to true.
        comp.pass_pool.push(ConditionKey {
            fsm_id: 10007,
            from: 10013,
            to: 10015,
            index: 0,
        });
        assert!(evaluate(&comp, 10007, &transition, &[cond.clone()], &cond, &ctx, 10013, 0));

        // Server-evaluated conditions still apply Reverse normally.
        let server_cond = StateMachineCondition {
            Index: 0,
            Reverse: true,
            IsClient: Some(false),
            CondTrue: Some(Default::default()),
            ..Default::default()
        };
        assert!(!evaluate(
            &comp,
            10007,
            &transition,
            &[server_cond.clone()],
            &server_cond,
            &ctx,
            10013,
            0,
        ));
    }
}
