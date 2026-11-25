use sonoro_gamesv_data::fsm_graph_registry::{MAX_STATE_DEPTH, condition_payload_matches};
use sonoro_gamesv_data::tables::ai_state_machine_config::{
    BindDelaySuicide, StateMachineAction, StateMachineBindState, StateMachineCondition,
    StateMachineNode, StateMachineTransition,
};

use crate::component::entity::fsm::FsmComponent;
use crate::fsm::types::{
    BindBuffAddition, ClientConditionLookup, ClientConditionRequirement, ClientPassResult,
    ConditionKey, ConfirmResult, EvalContext, FsmBindBuffSource, FsmNode, LifecycleEffect,
    PartActivation, PartReset, TagCount, TagMirror, Transition, WakeMask, wake_reason,
};
use crate::fsm::{blackboard, condition_evaluator, state_hierarchy};

const PENDING_TRANSITION_TIMEOUT_MS: i64 = 3000;

pub fn refresh_wake_requirements(comp: &mut FsmComponent, now_ms: i64) {
    for idx in 0..comp.runtime_nodes.len() {
        refresh_root_wake_requirements(comp, idx, now_ms, true);
    }
}

pub fn refresh_root_timer(comp: &mut FsmComponent, fsm_id: i32, now_ms: i64) {
    let canonical = state_hierarchy::canonical_state(comp, fsm_id);
    if let Some(idx) = comp.runtime_nodes.iter().position(|n| n.fsm_id == canonical) {
        refresh_root_wake_requirements(comp, idx, now_ms, false);
    }
}

pub fn mark_dirty(comp: &mut FsmComponent, reason: WakeMask) -> bool {
    let mut marked = false;
    for idx in 0..comp.runtime_nodes.len() {
        let fsm_id = comp.runtime_nodes[idx].fsm_id;
        if !server_evaluates_root(comp, fsm_id) {
            continue;
        }
        let runtime = &mut comp.runtime_nodes[idx];
        let relevant = if (reason & wake_reason::INITIAL) != 0 {
            reason
        } else {
            reason & runtime.wake_dependencies
        };
        if relevant == 0 {
            continue;
        }
        runtime.dirty_reasons |= relevant;
        marked = true;
    }
    marked
}

pub fn mark_root_dirty(comp: &mut FsmComponent, fsm_id: i32, reason: WakeMask) -> bool {
    let canonical = state_hierarchy::canonical_state(comp, fsm_id);
    let Some(idx) = comp.runtime_nodes.iter().position(|n| n.fsm_id == canonical) else {
        return false;
    };
    if !server_evaluates_root(comp, canonical) {
        return false;
    }
    let runtime = &mut comp.runtime_nodes[idx];
    let relevant = if (reason & wake_reason::INITIAL) != 0 {
        reason
    } else {
        reason & runtime.wake_dependencies
    };
    if relevant == 0 {
        return false;
    }
    runtime.dirty_reasons |= relevant;
    true
}

pub fn mark_other_roots_dirty(
    comp: &mut FsmComponent,
    exclude_fsm_id: i32,
    reason: WakeMask,
) -> bool {
    let excluded = state_hierarchy::canonical_state(comp, exclude_fsm_id);
    let mut marked = false;
    for idx in 0..comp.runtime_nodes.len() {
        let fsm_id = comp.runtime_nodes[idx].fsm_id;
        if fsm_id == excluded || !server_evaluates_root(comp, fsm_id) {
            continue;
        }
        let runtime = &mut comp.runtime_nodes[idx];
        let relevant = if (reason & wake_reason::INITIAL) != 0 {
            reason
        } else {
            reason & runtime.wake_dependencies
        };
        if relevant == 0 {
            continue;
        }
        runtime.dirty_reasons |= relevant;
        marked = true;
    }
    marked
}

pub fn pending_deadline(comp: &FsmComponent) -> Option<i64> {
    let mut deadline: Option<i64> = None;
    for runtime in &comp.runtime_nodes {
        if runtime.pending_to.is_none() || runtime.pending_started_ms <= 0 {
            continue;
        }
        let candidate = runtime
            .pending_started_ms
            .saturating_add(PENDING_TRANSITION_TIMEOUT_MS);
        deadline = Some(deadline.map_or(candidate, |d| d.min(candidate)));
    }
    deadline
}

fn refresh_root_wake_requirements(
    comp: &mut FsmComponent,
    runtime_idx: usize,
    now_ms: i64,
    refresh_dependencies: bool,
) {
    let fsm_id = comp.runtime_nodes[runtime_idx].fsm_id;
    if refresh_dependencies {
        comp.runtime_nodes[runtime_idx].wake_dependencies = 0;
    }
    comp.runtime_nodes[runtime_idx].next_timer_due_ms = None;
    if !server_evaluates_root(comp, fsm_id) {
        comp.runtime_nodes[runtime_idx].dirty_reasons = 0;
        return;
    }

    let active_len = comp.runtime_nodes[runtime_idx].active_len;
    if active_len < 2 {
        return;
    }
    let active_path = comp.runtime_nodes[runtime_idx].active_path;

    for index in 1..active_len {
        let active_state = active_path[index];
        let parent_state = active_path[index - 1];
        if let Some(any_state) = any_state_child_of(comp, parent_state) {
            refresh_source_wake_requirements(
                comp,
                runtime_idx,
                parent_state,
                any_state,
                parent_state,
                now_ms,
                refresh_dependencies,
            );
        }
        refresh_source_wake_requirements(
            comp,
            runtime_idx,
            parent_state,
            active_state,
            active_state,
            now_ms,
            refresh_dependencies,
        );
    }
}

fn refresh_source_wake_requirements(
    comp: &mut FsmComponent,
    runtime_idx: usize,
    parent_state: i32,
    configured_source: i32,
    activation_state: i32,
    now_ms: i64,
    refresh_dependencies: bool,
) {
    let graph = comp.graph.clone();
    let Some(parent_node) = graph.find_node(parent_state) else {
        return;
    };
    let Some(children) = parent_node.Children.as_deref() else {
        return;
    };
    let canonical_source = state_hierarchy::canonical_state(comp, configured_source);

    let fsm_id = comp.runtime_nodes[runtime_idx].fsm_id;
    let has_pending = comp.runtime_nodes[runtime_idx].pending_to.is_some();

    let mut extra_deps: WakeMask = 0;
    let mut earliest_timer: Option<i64> = None;

    for transition in &parent_node.Transitions {
        let resolved =
            state_hierarchy::resolve_override_states(comp, transition.From, transition.To, false);
        if resolved.from != canonical_source || resolved.from == resolved.to {
            continue;
        }
        if !target_is_child(comp, children, resolved.to) {
            continue;
        }
        let Some(top_condition) = condition_evaluator::find_condition(&transition.Conditions, 0)
        else {
            continue;
        };
        if refresh_dependencies {
            extra_deps |=
                condition_evaluator::dependency_mask(&transition.Conditions, top_condition, 0);
        }
        if has_pending {
            continue;
        }
        if let Some(candidate) = condition_evaluator::next_timer_deadline(
            comp,
            fsm_id,
            transition,
            &transition.Conditions,
            top_condition,
            now_ms,
            activation_state,
            0,
        ) {
            earliest_timer = Some(earliest_timer.map_or(candidate, |c| c.min(candidate)));
        }
    }

    let runtime = &mut comp.runtime_nodes[runtime_idx];
    if refresh_dependencies {
        runtime.wake_dependencies |= extra_deps;
    }
    if let Some(candidate) = earliest_timer {
        runtime.next_timer_due_ms = Some(
            runtime
                .next_timer_due_ms
                .map_or(candidate, |c| c.min(candidate)),
        );
    }
}

pub fn recover_expired_pending(comp: &mut FsmComponent, now_ms: i64) -> Result<bool, &'static str> {
    if comp.lifecycle_effects_pending {
        return Ok(false);
    }
    let mut expired_indices = Vec::new();
    for (idx, runtime) in comp.runtime_nodes.iter().enumerate() {
        if runtime.pending_to.is_none() || runtime.pending_started_ms <= 0 {
            continue;
        }
        if now_ms < runtime.pending_started_ms {
            continue;
        }
        if now_ms - runtime.pending_started_ms < PENDING_TRANSITION_TIMEOUT_MS {
            continue;
        }
        expired_indices.push(idx);
    }
    if expired_indices.is_empty() {
        return Ok(false);
    }
    for idx in expired_indices {
        commit_pending(comp, idx)?;
    }
    Ok(true)
}

pub fn record_client_pass(
    comp: &mut FsmComponent,
    key: ConditionKey,
    value: bool,
) -> ClientPassResult {
    let fsm_id = state_hierarchy::canonical_state(comp, key.fsm_id);
    let Some(runtime) = state_hierarchy::runtime_node(comp, fsm_id) else {
        return ClientPassResult::MachineNotFound;
    };
    if !state_hierarchy::state_belongs_to_fsm(comp, fsm_id, key.from) {
        return ClientPassResult::InvalidSource;
    }
    let active_path = runtime.active().to_vec();
    let pending_path = runtime.pending().to_vec();
    let has_pending = runtime.pending_from.is_some() && runtime.pending_to.is_some();

    let source_is_active = state_hierarchy::path_contains(comp, &active_path, key.from)
        || path_contains_eligible_any_state(comp, &active_path, key.from);
    let source_is_pending = has_pending
        && (state_hierarchy::path_contains(comp, &pending_path, key.from)
            || path_contains_eligible_any_state(comp, &pending_path, key.from));
    if !source_is_active && !source_is_pending {
        return ClientPassResult::InactiveSource;
    }
    if !state_hierarchy::state_belongs_to_fsm(comp, fsm_id, key.to) {
        return ClientPassResult::InvalidTarget;
    }

    let resolved = state_hierarchy::resolve_override_states(comp, key.from, key.to, false);
    match client_condition_lookup(comp, fsm_id, resolved.from, resolved.to, key.index) {
        ClientConditionLookup::Valid(requirement) => {
            let path = if source_is_pending {
                &pending_path
            } else {
                &active_path
            };
            if !state_hierarchy::client_task_requirement_satisfied(
                comp,
                path,
                resolved.from,
                resolved.to,
                requirement,
            ) {
                return ClientPassResult::ConditionContextInvalid;
            }
        }
        ClientConditionLookup::TransitionNotFound => {
            return ClientPassResult::TransitionNotFound;
        }
        ClientConditionLookup::ConditionNotFound => {
            return ClientPassResult::ConditionNotFound;
        }
        ClientConditionLookup::ConditionNotClient => {
            return ClientPassResult::ConditionNotClient;
        }
        ClientConditionLookup::ConditionInvalid => {
            return ClientPassResult::ConditionContextInvalid;
        }
    }

    let resolved_key = ConditionKey {
        fsm_id,
        from: resolved.from,
        to: resolved.to,
        index: key.index,
    };

    if !value {
        remove_pass(comp, resolved_key);
    } else {
        append_pass(comp, resolved_key);
    }
    ClientPassResult::Updated
}

pub fn confirm_pending(
    comp: &mut FsmComponent,
    fsm_id: i32,
    state: i32,
) -> Result<ConfirmResult, &'static str> {
    let canonical_fsm = state_hierarchy::canonical_state(comp, fsm_id);
    let Some(runtime_idx) = comp
        .runtime_nodes
        .iter()
        .position(|n| n.fsm_id == canonical_fsm)
    else {
        return Ok(ConfirmResult::MachineNotFound);
    };
    if !state_hierarchy::state_belongs_to_fsm(comp, fsm_id, state) {
        return Ok(ConfirmResult::InvalidTarget);
    }
    let Some(pending_state) = comp.runtime_nodes[runtime_idx].pending_to else {
        return Ok(ConfirmResult::NoPending);
    };
    if state_hierarchy::states_equivalent(comp, state, pending_state) {
        commit_pending(comp, runtime_idx)?;
        return Ok(ConfirmResult::Confirmed);
    }
    Ok(ConfirmResult::Mismatch(state_hierarchy::client_state(
        comp,
        pending_state,
    )))
}

pub fn confirm_state_request(
    comp: &mut FsmComponent,
    fsm_id: i32,
    from: i32,
    to: i32,
    ctx: &EvalContext<'_>,
) -> Result<ConfirmResult, &'static str> {
    let canonical_fsm = state_hierarchy::canonical_state(comp, fsm_id);
    let Some(runtime_idx) = comp
        .runtime_nodes
        .iter()
        .position(|n| n.fsm_id == canonical_fsm)
    else {
        return Ok(ConfirmResult::MachineNotFound);
    };
    if !state_hierarchy::state_belongs_to_fsm(comp, fsm_id, from) {
        return Ok(ConfirmResult::InvalidSource);
    }
    if !state_hierarchy::state_belongs_to_fsm(comp, fsm_id, to) {
        return Ok(ConfirmResult::InvalidTarget);
    }

    let Some(pending_state) = comp.runtime_nodes[runtime_idx].pending_to else {
        if accept_predicted_transition(comp, fsm_id, from, to, ctx)? {
            return Ok(ConfirmResult::Accepted);
        }
        return Ok(ConfirmResult::NoPending);
    };

    if state_hierarchy::states_equivalent(comp, to, pending_state) {
        let Some(pending_from) = comp.runtime_nodes[runtime_idx].pending_from else {
            return Ok(ConfirmResult::Mismatch(state_hierarchy::client_state(
                comp,
                pending_state,
            )));
        };
        if !state_hierarchy::states_equivalent(comp, pending_from, from) {
            return Ok(ConfirmResult::Mismatch(state_hierarchy::client_state(
                comp,
                pending_state,
            )));
        }
        commit_pending(comp, runtime_idx)?;
        return Ok(ConfirmResult::Confirmed);
    }

    if can_apply_nested_transition(comp, runtime_idx, from, to) {
        commit_pending(comp, runtime_idx)?;
        change_current_state(comp, fsm_id, to, ctx.now_ms)?;
        return Ok(ConfirmResult::Confirmed);
    }

    Ok(ConfirmResult::Mismatch(state_hierarchy::client_state(
        comp,
        pending_state,
    )))
}

pub fn current_state(comp: &FsmComponent, fsm_id: i32) -> Option<i32> {
    let canonical = state_hierarchy::canonical_state(comp, fsm_id);
    comp.runtime_nodes
        .iter()
        .find(|n| n.fsm_id == canonical)
        .and_then(|r| r.leaf())
        .map(|leaf| state_hierarchy::client_state(comp, leaf))
}

pub fn find_ready_transition(
    comp: &mut FsmComponent,
    fsm_id: i32,
    ctx: &EvalContext<'_>,
) -> Result<Option<Transition>, &'static str> {
    if comp.lifecycle_effects_pending {
        return Ok(None);
    }
    let canonical_fsm = state_hierarchy::canonical_state(comp, fsm_id);
    let Some(runtime_idx) = comp
        .runtime_nodes
        .iter()
        .position(|n| n.fsm_id == canonical_fsm)
    else {
        return Ok(None);
    };
    let root_fsm_id = comp.runtime_nodes[runtime_idx].fsm_id;
    if !server_evaluates_root(comp, root_fsm_id) {
        return Ok(None);
    }
    if comp.runtime_nodes[runtime_idx].pending_to.is_some() {
        return Ok(None);
    }

    let active_len = comp.runtime_nodes[runtime_idx].active_len;
    if active_len < 2 {
        return Ok(None);
    }
    let active_path = comp.runtime_nodes[runtime_idx].active_path;

    for index in 1..active_len {
        let active_state = active_path[index];
        let parent_state = active_path[index - 1];
        if let Some(any_state) = any_state_child_of(comp, parent_state)
            && let Some(t) = check_transitions_for_state(
                comp,
                root_fsm_id,
                parent_state,
                any_state,
                active_state,
                parent_state,
                None,
                false,
                ctx,
            )?
        {
            return Ok(Some(t));
        }
        if let Some(t) = check_transitions_for_state(
            comp,
            root_fsm_id,
            parent_state,
            active_state,
            active_state,
            active_state,
            None,
            false,
            ctx,
        )? {
            return Ok(Some(t));
        }
    }

    Ok(None)
}

pub fn enter_path(
    comp: &mut FsmComponent,
    fsm_id: i32,
    path: &[i32],
    active_since_ms: &[i64],
) -> Result<(), &'static str> {
    for (index, &state) in path.iter().enumerate() {
        enter_node(comp, fsm_id, state, active_since_ms[index])?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn check_transitions_for_state(
    comp: &mut FsmComponent,
    fsm_id: i32,
    parent_state: i32,
    configured_source: i32,
    runtime_source: i32,
    activation_state: i32,
    requested_target: Option<i32>,
    client_request: bool,
    ctx: &EvalContext<'_>,
) -> Result<Option<Transition>, &'static str> {
    let canonical_fsm = state_hierarchy::canonical_state(comp, fsm_id);
    let Some(runtime_idx) = comp
        .runtime_nodes
        .iter()
        .position(|n| n.fsm_id == canonical_fsm)
    else {
        return Ok(None);
    };
    if comp.runtime_nodes[runtime_idx].pending_to.is_some() {
        return Ok(None);
    }

    let graph = comp.graph.clone();
    let Some(parent_node) = graph.find_node(parent_state) else {
        return Ok(None);
    };
    let Some(children) = parent_node.Children.as_deref() else {
        return Ok(None);
    };
    let canonical_source = state_hierarchy::canonical_state(comp, configured_source);
    let canonical_runtime_source = state_hierarchy::canonical_state(comp, runtime_source);
    let runtime_fsm_id = comp.runtime_nodes[runtime_idx].fsm_id;

    for transition in &parent_node.Transitions {
        let resolved_transition =
            state_hierarchy::resolve_override_states(comp, transition.From, transition.To, false);
        if resolved_transition.from != canonical_source {
            continue;
        }
        let target_state = resolved_transition.to;
        if canonical_source == target_state {
            continue;
        }
        if let Some(target) = requested_target
            && !state_hierarchy::states_equivalent(comp, target_state, target)
        {
            continue;
        }
        if !target_is_child(comp, children, target_state) {
            continue;
        }
        if client_request
            && !client_may_take_transition(comp, runtime_fsm_id, canonical_source, transition)
        {
            continue;
        }

        let Some(top_condition) = condition_evaluator::find_condition(&transition.Conditions, 0)
        else {
            continue;
        };
        if !condition_evaluator::evaluate(
            comp,
            fsm_id,
            transition,
            &transition.Conditions,
            top_condition,
            ctx,
            activation_state,
            0,
        ) {
            continue;
        }

        if comp.dissolve_combine_signal
            && condition_evaluator::transition_uses_dissolve_combine(&transition.Conditions)
        {
            comp.dissolve_combine_signal = false;
        }
        set_pending_transition(
            comp,
            runtime_idx,
            canonical_runtime_source,
            target_state,
            ctx.now_ms,
        )?;
        return Ok(Some(Transition {
            fsm_id: state_hierarchy::client_state(comp, runtime_fsm_id),
            from: state_hierarchy::client_state(comp, canonical_runtime_source),
            to: state_hierarchy::client_state(comp, target_state),
        }));
    }

    Ok(None)
}

fn accept_predicted_transition(
    comp: &mut FsmComponent,
    fsm_id: i32,
    from: i32,
    to: i32,
    ctx: &EvalContext<'_>,
) -> Result<bool, &'static str> {
    let canonical_fsm = state_hierarchy::canonical_state(comp, fsm_id);
    let Some(runtime_idx) = comp
        .runtime_nodes
        .iter()
        .position(|n| n.fsm_id == canonical_fsm)
    else {
        return Ok(false);
    };
    let runtime_fsm_id = comp.runtime_nodes[runtime_idx].fsm_id;
    let active_len = comp.runtime_nodes[runtime_idx].active_len;
    let active_path = comp.runtime_nodes[runtime_idx].active_path;

    if !state_hierarchy::path_contains(comp, &active_path[..active_len], from) {
        return Ok(false);
    }
    if !server_evaluates_root(comp, runtime_fsm_id) {
        if !can_client_transition(comp, runtime_fsm_id, from, to)
            && !can_client_any_state_transition(
                comp,
                runtime_fsm_id,
                &active_path[..active_len],
                from,
                to,
            )
        {
            return Ok(false);
        }
        change_current_state(comp, fsm_id, to, ctx.now_ms)?;
        return Ok(true);
    }

    if active_len < 2 {
        return Ok(false);
    }
    for index in 1..active_len {
        let active_state = active_path[index];
        if !state_hierarchy::states_equivalent(comp, active_state, from) {
            continue;
        }
        let parent_state = active_path[index - 1];
        if state_hierarchy::find_node(comp, parent_state).is_none() {
            return Ok(false);
        }
        let accepted = if let Some(any_state) = any_state_child_of(comp, parent_state) {
            check_transitions_for_state(
                comp,
                runtime_fsm_id,
                parent_state,
                any_state,
                active_state,
                parent_state,
                Some(to),
                true,
                ctx,
            )?
        } else {
            None
        };
        let accepted = if accepted.is_some() {
            accepted
        } else {
            check_transitions_for_state(
                comp,
                runtime_fsm_id,
                parent_state,
                active_state,
                active_state,
                active_state,
                Some(to),
                true,
                ctx,
            )?
        };
        if accepted.is_none() {
            return Ok(false);
        }
        commit_pending(comp, runtime_idx)?;
        return Ok(true);
    }
    Ok(false)
}

fn can_apply_nested_transition(
    comp: &FsmComponent,
    runtime_idx: usize,
    from: i32,
    to: i32,
) -> bool {
    let runtime = &comp.runtime_nodes[runtime_idx];
    let pending = runtime.pending();
    if !state_hierarchy::path_contains(comp, pending, from) {
        return false;
    }
    if !state_hierarchy::state_belongs_to_fsm(comp, runtime.fsm_id, to) {
        return false;
    }
    if !allows_nested_transition(comp, runtime.fsm_id, from) {
        let Some(any_state) = any_state_source_for_active_state(comp, pending, from) else {
            return false;
        };
        if !allows_nested_transition(comp, runtime.fsm_id, any_state) {
            return false;
        }
    }
    can_client_transition(comp, runtime.fsm_id, from, to)
        || can_client_any_state_transition(comp, runtime.fsm_id, pending, from, to)
}

fn any_state_context_for_active_state<'a>(
    comp: &'a FsmComponent,
    path: &[i32],
    active_source: i32,
) -> Option<(i32, &'a StateMachineNode)> {
    if path.len() < 2 {
        return None;
    }
    for index in 1..path.len() {
        let active_state = path[index];
        if !state_hierarchy::states_equivalent(comp, active_state, active_source) {
            continue;
        }
        let parent_state = path[index - 1];
        let parent_node = state_hierarchy::find_node(comp, parent_state)?;
        let source = any_state_child_of_node(comp, parent_node)?;
        return Some((source, parent_node));
    }
    None
}

fn any_state_source_for_active_state(
    comp: &FsmComponent,
    path: &[i32],
    active_source: i32,
) -> Option<i32> {
    any_state_context_for_active_state(comp, path, active_source).map(|(s, _)| s)
}

fn path_contains_eligible_any_state(comp: &FsmComponent, path: &[i32], source: i32) -> bool {
    if path.len() < 2 {
        return false;
    }
    for index in 1..path.len() {
        let Some(parent_node) = state_hierarchy::find_node(comp, path[index - 1]) else {
            continue;
        };
        let Some(any_state) = any_state_child_of_node(comp, parent_node) else {
            continue;
        };
        if state_hierarchy::states_equivalent(comp, any_state, source) {
            return true;
        }
    }
    false
}

fn any_state_child_of(comp: &FsmComponent, parent_state: i32) -> Option<i32> {
    let parent_node = state_hierarchy::find_node(comp, parent_state)?;
    any_state_child_of_node(comp, parent_node)
}

fn any_state_child_of_node(comp: &FsmComponent, parent_node: &StateMachineNode) -> Option<i32> {
    let children = parent_node.Children.as_deref()?;
    let mut any_state = None;
    for &child in children {
        let Some(child_node) = state_hierarchy::find_node(comp, child) else {
            continue;
        };
        if child_node.IsAnyState {
            any_state = Some(state_hierarchy::canonical_state(comp, child));
        }
    }
    any_state
}

fn target_is_child(comp: &FsmComponent, children: &[i32], target: i32) -> bool {
    children
        .iter()
        .any(|&child| state_hierarchy::states_equivalent(comp, child, target))
}

fn can_client_any_state_transition(
    comp: &FsmComponent,
    fsm_id: i32,
    path: &[i32],
    from: i32,
    to: i32,
) -> bool {
    let Some((source, parent_node)) = any_state_context_for_active_state(comp, path, from) else {
        return false;
    };
    let Some(children) = parent_node.Children.as_deref() else {
        return false;
    };
    if !target_is_child(comp, children, to) {
        return false;
    }
    can_client_transition(comp, fsm_id, source, to)
}

fn can_client_transition(comp: &FsmComponent, fsm_id: i32, from: i32, to: i32) -> bool {
    let requested = state_hierarchy::resolve_override_states(comp, from, to, false);
    let Some(root) = state_hierarchy::find_node(comp, fsm_id) else {
        return false;
    };
    let client_owns_animation = root.IsAnimStateMachine.unwrap_or(false);
    if !comp.graph.has_transition(requested.from, requested.to) {
        return false;
    }
    if client_owns_animation || state_hierarchy::is_conduit_state(comp, requested.from) {
        return true;
    }
    comp.graph
        .is_predicted_transition(requested.from, requested.to)
}

fn client_may_take_transition(
    comp: &FsmComponent,
    fsm_id: i32,
    from: i32,
    transition: &StateMachineTransition,
) -> bool {
    let Some(root) = state_hierarchy::find_node(comp, fsm_id) else {
        return false;
    };
    if root.IsAnimStateMachine.unwrap_or(false) || state_hierarchy::is_conduit_state(comp, from) {
        return true;
    }
    matches!(transition.TransitionPredictionType, Some(1) | Some(2))
}

fn allows_nested_transition(comp: &FsmComponent, fsm_id: i32, from: i32) -> bool {
    let Some(root) = state_hierarchy::find_node(comp, fsm_id) else {
        return false;
    };
    root.IsAnimStateMachine.unwrap_or(false) || state_hierarchy::is_conduit_state(comp, from)
}

fn server_evaluates_root(comp: &FsmComponent, fsm_id: i32) -> bool {
    let Some(root) = state_hierarchy::find_node(comp, fsm_id) else {
        return false;
    };
    !root.IsAnimStateMachine.unwrap_or(false)
}

fn client_condition_lookup(
    comp: &FsmComponent,
    fsm_id: i32,
    from: i32,
    to: i32,
    index: i32,
) -> ClientConditionLookup {
    let mut found_transition = false;
    let mut found_condition = false;
    let mut invalid_condition = false;
    let mut requirement = ClientConditionRequirement::None;

    if find_client_condition_recursive(
        comp,
        state_hierarchy::canonical_state(comp, fsm_id),
        state_hierarchy::canonical_state(comp, from),
        state_hierarchy::canonical_state(comp, to),
        index,
        &mut found_transition,
        &mut found_condition,
        &mut invalid_condition,
        &mut requirement,
        0,
    ) {
        return ClientConditionLookup::Valid(requirement);
    }

    if !found_transition {
        ClientConditionLookup::TransitionNotFound
    } else if !found_condition {
        ClientConditionLookup::ConditionNotFound
    } else if invalid_condition {
        ClientConditionLookup::ConditionInvalid
    } else {
        ClientConditionLookup::ConditionNotClient
    }
}

#[allow(clippy::too_many_arguments)]
fn find_client_condition_recursive(
    comp: &FsmComponent,
    node_id: i32,
    from: i32,
    to: i32,
    index: i32,
    found_transition: &mut bool,
    found_condition: &mut bool,
    invalid_condition: &mut bool,
    requirement: &mut ClientConditionRequirement,
    depth: usize,
) -> bool {
    if depth >= MAX_STATE_DEPTH {
        return false;
    }
    let Some(node) = state_hierarchy::find_node(comp, node_id) else {
        return false;
    };

    for transition in &node.Transitions {
        let resolved =
            state_hierarchy::resolve_override_states(comp, transition.From, transition.To, false);
        if resolved.from != from || resolved.to != to {
            continue;
        }
        *found_transition = true;
        let Some(condition) = condition_evaluator::find_condition(&transition.Conditions, index)
        else {
            continue;
        };
        *found_condition = true;
        if !condition.IsClient.unwrap_or(false) {
            continue;
        }
        if !condition_payload_matches(condition) {
            *invalid_condition = true;
            continue;
        }
        *requirement = client_condition_requirement(condition);
        return true;
    }

    if let Some(children) = node.Children.as_deref() {
        for &child in children {
            if find_client_condition_recursive(
                comp,
                child,
                from,
                to,
                index,
                found_transition,
                found_condition,
                invalid_condition,
                requirement,
                depth + 1,
            ) {
                return true;
            }
        }
    }
    false
}

fn client_condition_requirement(condition: &StateMachineCondition) -> ClientConditionRequirement {
    if condition.CondTaskFinish.is_some() {
        ClientConditionRequirement::Task
    } else if condition.CondMontageTimeRemaining.is_some()
        || condition.CondMontageTimeElapsing.is_some()
    {
        ClientConditionRequirement::Montage
    } else if condition.CondCheckGroupPatrol.is_some() {
        ClientConditionRequirement::GroupPatrol
    } else if condition.CondCheckGroupPerform.is_some() {
        ClientConditionRequirement::GroupPerform
    } else {
        ClientConditionRequirement::None
    }
}

fn run_actions(
    comp: &mut FsmComponent,
    actions: &[StateMachineAction],
    state_entry: bool,
) -> Result<(), &'static str> {
    for action in actions {
        if let Some(ev) = &action.ActionDispatchEvent {
            comp.event = Some(ev.Event.clone());
        }
        if state_entry && action.ActionEnterFight.is_some() {
            comp.lifecycle_effects.push(LifecycleEffect::EnterFight);
        }
        if let Some(buff) = &action.ActionAddBuff {
            comp.lifecycle_effects
                .push(LifecycleEffect::AddBuff(buff.BuffId));
        }
        if let Some(buff) = &action.ActionRemoveBuff {
            comp.lifecycle_effects
                .push(LifecycleEffect::RemoveBuff(buff.BuffId));
        }
        if let Some(cue) = &action.ActionCue
            && cue.CueIds.contains(&1020)
        {
            comp.lifecycle_effects.push(LifecycleEffect::CueParalysis);
        }
        if action.ActionResetStatus.is_some() {
            comp.lifecycle_effects.push(LifecycleEffect::ResetStatus);
        }
        if action.ActionSetRageFullAttribute.is_some() {
            comp.lifecycle_effects.push(LifecycleEffect::SetRageFull);
        }
        if let Some(state) = &action.ActionInstChangeStateTag
            && let Ok(tag_id) = i32::try_from(state.TagId)
        {
            comp.lifecycle_effects
                .push(LifecycleEffect::SetInstanceState(tag_id));
        }
        if let Some(part) = &action.ActionActivatePart {
            comp.lifecycle_effects
                .push(LifecycleEffect::ActivatePart(PartActivation {
                    name: part.PartName.clone(),
                    activate: part.Activate,
                }));
        }
        if let Some(part) = &action.ActionResetPart {
            comp.lifecycle_effects
                .push(LifecycleEffect::ResetPart(PartReset {
                    name: part.PartName.clone(),
                    reset_activate: part.ResetActivate,
                    reset_life: part.ResetLife,
                }));
        }
        if let Some(tag) = &action.ActionAddTagCount
            && tag.Count > 0
        {
            update_tag_count(comp, tag.TagId, tag.Count)?;
        }
        if let Some(tag) = &action.ActionRemoveTagCount {
            if tag.Count == -1 {
                clear_tag(comp, tag.TagId);
            } else if tag.Count > 0 {
                update_tag_count(comp, tag.TagId, -tag.Count)?;
            }
        }
    }
    Ok(())
}

fn update_activation_bind_states(
    comp: &mut FsmComponent,
    binds: &[StateMachineBindState],
    delta: i32,
) -> Result<(), &'static str> {
    for bind in binds {
        if let Some(tag) = &bind.BindTag {
            update_tag_count(comp, tag.TagId, delta)?;
        }
    }
    Ok(())
}

fn enter_bind_states(
    comp: &mut FsmComponent,
    fsm_id: i32,
    state: i32,
    activation_ms: i64,
    binds: &[StateMachineBindState],
) -> Result<(), &'static str> {
    let mut delayed_suicide: Option<&BindDelaySuicide> = None;
    for (index, bind) in binds.iter().enumerate() {
        if let Some(buff) = &bind.BindBuff {
            let source = bind_buff_source(comp, fsm_id, state, index)?;
            comp.lifecycle_effects
                .push(LifecycleEffect::BindBuffAdd(BindBuffAddition {
                    source,
                    buff_id: buff.BuffId,
                }));
        }
        if let Some(delayed) = &bind.BindDelaySuicide {
            delayed_suicide = Some(delayed);
        }
    }
    if let Some(delayed) = delayed_suicide {
        schedule_delayed_suicide(comp, fsm_id, state, activation_ms, delayed);
    }
    Ok(())
}

fn exit_bind_states(
    comp: &mut FsmComponent,
    fsm_id: i32,
    state: i32,
    binds: &[StateMachineBindState],
) -> Result<(), &'static str> {
    for (index, bind) in binds.iter().enumerate() {
        if bind.BindBuff.is_none() {
            continue;
        }
        let source = bind_buff_source(comp, fsm_id, state, index)?;
        comp.lifecycle_effects
            .push(LifecycleEffect::BindBuffRemove(source));
    }
    Ok(())
}

fn bind_buff_source(
    comp: &FsmComponent,
    fsm_id: i32,
    state: i32,
    index: usize,
) -> Result<FsmBindBuffSource, &'static str> {
    Ok(FsmBindBuffSource {
        fsm_id: state_hierarchy::canonical_state(comp, fsm_id),
        state: state_hierarchy::canonical_state(comp, state),
        bind_index: i32::try_from(index).map_err(|_| "FsmBindIndexOverflow")?,
    })
}

fn schedule_delayed_suicide(
    comp: &mut FsmComponent,
    fsm_id: i32,
    state: i32,
    activation_ms: i64,
    delayed: &BindDelaySuicide,
) {
    if !delayed.SuicideDelay.is_finite()
        || !delayed.DestroyDelay.is_finite()
        || delayed.SuicideDelay < 0.0
        || delayed.DestroyDelay < delayed.SuicideDelay
    {
        return;
    }
    let Some(suicide_due_ms) = delayed_deadline(activation_ms, delayed.SuicideDelay) else {
        return;
    };
    let Some(destroy_due_ms) = delayed_deadline(activation_ms, delayed.DestroyDelay) else {
        return;
    };
    let canonical_state = state_hierarchy::canonical_state(comp, state);
    let Some(runtime) = state_hierarchy::runtime_node_mut(comp, fsm_id) else {
        return;
    };
    if runtime.delayed_suicide_fired && runtime.delayed_destroy_due_ms.is_some() {
        return;
    }
    runtime.delayed_suicide_state = Some(canonical_state);
    runtime.delayed_suicide_due_ms = Some(suicide_due_ms);
    runtime.delayed_destroy_due_ms = Some(destroy_due_ms);
    runtime.delayed_suicide_fired = false;
}

fn delayed_deadline(activation_ms: i64, delay_ms: f32) -> Option<i64> {
    let wide_delay = f64::from(delay_ms);
    let max_i64 = i64::MAX as f64;
    if !wide_delay.is_finite() || wide_delay < 0.0 || wide_delay >= max_i64 {
        return None;
    }
    let rounded_delay = wide_delay.ceil() as i64;
    activation_ms.checked_add(rounded_delay)
}

fn update_tag_count(comp: &mut FsmComponent, tag_id: i64, delta: i32) -> Result<(), &'static str> {
    if delta == 0 {
        return Ok(());
    }
    let tag32 = i32::try_from(tag_id).ok();
    for tag in &mut comp.tags {
        if tag.id != tag_id {
            continue;
        }
        let old_count = tag.count;
        let count = i64::from(old_count) + i64::from(delta);
        if count > i64::from(i32::MAX) {
            return Err("FsmTagCountOverflow");
        }
        let new_count = if count > 0 { count as i32 } else { 0 };
        tag.count = new_count;
        if let Some(id32) = tag32
            && id32 != 0
        {
            let effective_delta = new_count - old_count;
            if effective_delta != 0 {
                comp.lifecycle_effects
                    .push(LifecycleEffect::TagMirror(TagMirror {
                        tag_id: id32,
                        delta: effective_delta,
                        clear: false,
                        was_present: old_count > 0,
                        is_present: new_count > 0,
                    }));
            }
        }
        return Ok(());
    }

    if delta < 0 {
        if let Some(id32) = tag32
            && id32 != 0
        {
            comp.lifecycle_effects
                .push(LifecycleEffect::TagMirror(TagMirror {
                    tag_id: id32,
                    delta,
                    clear: false,
                    was_present: false,
                    is_present: false,
                }));
        }
        return Ok(());
    }

    comp.tags.push(TagCount {
        id: tag_id,
        count: delta,
    });
    if let Some(id32) = tag32
        && id32 != 0
    {
        comp.lifecycle_effects
            .push(LifecycleEffect::TagMirror(TagMirror {
                tag_id: id32,
                delta,
                clear: false,
                was_present: false,
                is_present: true,
            }));
    }
    Ok(())
}

fn clear_tag(comp: &mut FsmComponent, tag_id: i64) {
    let mut was_present = false;
    for tag in &mut comp.tags {
        if tag.id == tag_id {
            was_present = tag.count > 0;
            tag.count = 0;
            break;
        }
    }
    if let Ok(id32) = i32::try_from(tag_id)
        && id32 != 0
    {
        comp.lifecycle_effects
            .push(LifecycleEffect::TagMirror(TagMirror {
                tag_id: id32,
                delta: 0,
                clear: true,
                was_present,
                is_present: false,
            }));
    }
}

fn apply_path_lifecycle(
    comp: &mut FsmComponent,
    fsm_id: i32,
    active_path: &[i32],
    target_path: &[i32],
    target_since_ms: &[i64],
) -> Result<(), &'static str> {
    let common_len = state_hierarchy::common_path_prefix_len(active_path, target_path);

    comp.event = None;
    remove_passes_for_exited_path(comp, fsm_id, active_path, target_path);

    for &state in active_path[common_len..].iter().rev() {
        exit_node(comp, fsm_id, state)?;
    }

    for (offset, &state) in target_path[common_len..].iter().enumerate() {
        let index = common_len + offset;
        enter_node(comp, fsm_id, state, target_since_ms[index])?;
    }
    Ok(())
}

fn enter_node(
    comp: &mut FsmComponent,
    fsm_id: i32,
    state: i32,
    activation_ms: i64,
) -> Result<(), &'static str> {
    let graph = comp.graph.clone();
    let Some(node) = graph.find_node(state) else {
        return Ok(());
    };
    update_activation_bind_states(comp, &node.BindStates, 1)?;
    run_actions(comp, &node.OnEnterActions, true)?;
    enter_bind_states(comp, fsm_id, state, activation_ms, &node.BindStates)?;
    Ok(())
}

fn exit_node(comp: &mut FsmComponent, fsm_id: i32, state: i32) -> Result<(), &'static str> {
    let graph = comp.graph.clone();
    let Some(node) = graph.find_node(state) else {
        return Ok(());
    };
    update_activation_bind_states(comp, &node.BindStates, -1)?;
    run_actions(comp, &node.OnExitActions, false)?;
    exit_bind_states(comp, fsm_id, state, &node.BindStates)?;
    exit_delayed_suicide_bind(comp, fsm_id, state);
    Ok(())
}

fn exit_delayed_suicide_bind(comp: &mut FsmComponent, fsm_id: i32, state: i32) {
    let Some(bound_state) = state_hierarchy::runtime_node(comp, fsm_id).and_then(|r| r.delayed_suicide_state) else {
        return;
    };
    if !state_hierarchy::states_equivalent(comp, bound_state, state) {
        return;
    }
    if let Some(runtime) = state_hierarchy::runtime_node_mut(comp, fsm_id) {
        runtime.delayed_suicide_due_ms = None;
        if runtime.delayed_suicide_fired {
            return;
        }
        runtime.delayed_suicide_state = None;
        runtime.delayed_destroy_due_ms = None;
    }
}

fn set_pending_transition(
    comp: &mut FsmComponent,
    runtime_idx: usize,
    from: i32,
    to: i32,
    now_ms: i64,
) -> Result<(), &'static str> {
    let runtime_fsm_id = comp.runtime_nodes[runtime_idx].fsm_id;
    let mut pending_path = [0i32; MAX_STATE_DEPTH];
    let pending_len = state_hierarchy::build_active_path(comp, runtime_fsm_id, to, &mut pending_path)
        .ok_or("InvalidFsmTransitionTarget")?;

    let active_len = comp.runtime_nodes[runtime_idx].active_len;
    let active_path = comp.runtime_nodes[runtime_idx].active_path;
    let active_since_ms = comp.runtime_nodes[runtime_idx].active_since_ms;
    let preserves_skill = active_len != 0
        && pending_len != 0
        && active_path[active_len - 1] == pending_path[pending_len - 1];

    let mut pending_since_ms = [0i64; MAX_STATE_DEPTH];
    for (index, &state) in pending_path[..pending_len].iter().enumerate() {
        pending_since_ms[index] = if index < active_len && active_path[index] == state {
            active_since_ms[index]
        } else {
            now_ms
        };
    }

    let canon_from = state_hierarchy::canonical_state(comp, from);
    let canon_to = state_hierarchy::canonical_state(comp, to);
    let runtime = &mut comp.runtime_nodes[runtime_idx];
    runtime.pending_path = pending_path;
    runtime.pending_since_ms = pending_since_ms;
    runtime.pending_len = pending_len;
    runtime.pending_from = Some(canon_from);
    runtime.pending_to = Some(canon_to);
    runtime.pending_started_ms = now_ms;
    runtime.pending_skill_id = if preserves_skill {
        runtime.active_skill_id
    } else {
        None
    };
    runtime.pending_skill_ended = preserves_skill && runtime.active_skill_ended;
    runtime.next_timer_due_ms = None;

    blackboard::prepare_path(
        comp,
        &pending_path[..pending_len],
        &pending_since_ms[..pending_len],
        true,
    );
    Ok(())
}

fn commit_pending(comp: &mut FsmComponent, runtime_idx: usize) -> Result<(), &'static str> {
    let runtime = &comp.runtime_nodes[runtime_idx];
    if runtime.pending_from.is_none() || runtime.pending_len == 0 {
        return Ok(());
    }
    let fsm_id = runtime.fsm_id;
    let active_len = runtime.active_len;
    let active_path = runtime.active_path;
    let pending_len = runtime.pending_len;
    let pending_path = runtime.pending_path;
    let pending_since_ms = runtime.pending_since_ms;
    let pending_skill_id = runtime.pending_skill_id;
    let pending_skill_ended = runtime.pending_skill_ended;

    apply_path_lifecycle(
        comp,
        fsm_id,
        &active_path[..active_len],
        &pending_path[..pending_len],
        &pending_since_ms[..pending_len],
    )?;

    let runtime = &mut comp.runtime_nodes[runtime_idx];
    runtime.active_path[..pending_len].copy_from_slice(&pending_path[..pending_len]);
    runtime.active_since_ms[..pending_len].copy_from_slice(&pending_since_ms[..pending_len]);
    runtime.active_len = pending_len;
    runtime.active_skill_id = pending_skill_id;
    runtime.active_skill_ended = pending_skill_ended;
    clear_pending(runtime);
    Ok(())
}

fn change_current_state(
    comp: &mut FsmComponent,
    fsm_id: i32,
    to: i32,
    now_ms: i64,
) -> Result<(), &'static str> {
    let canonical_fsm = state_hierarchy::canonical_state(comp, fsm_id);
    let Some(runtime_idx) = comp
        .runtime_nodes
        .iter()
        .position(|n| n.fsm_id == canonical_fsm)
    else {
        return Ok(());
    };
    let runtime_fsm_id = comp.runtime_nodes[runtime_idx].fsm_id;
    let active_len_prev = comp.runtime_nodes[runtime_idx].active_len;
    let active_path_prev = comp.runtime_nodes[runtime_idx].active_path;
    let active_since_prev = comp.runtime_nodes[runtime_idx].active_since_ms;

    let mut target_path = [0i32; MAX_STATE_DEPTH];
    let Some(active_len) =
        state_hierarchy::build_active_path(comp, runtime_fsm_id, to, &mut target_path)
    else {
        return Ok(());
    };
    let preserves_skill = active_len_prev != 0
        && active_len != 0
        && active_path_prev[active_len_prev - 1] == target_path[active_len - 1];

    let mut target_since_ms = [0i64; MAX_STATE_DEPTH];
    for (index, &state) in target_path[..active_len].iter().enumerate() {
        target_since_ms[index] = if index < active_len_prev && active_path_prev[index] == state {
            active_since_prev[index]
        } else {
            now_ms
        };
    }

    blackboard::prepare_path(
        comp,
        &target_path[..active_len],
        &target_since_ms[..active_len],
        true,
    );
    apply_path_lifecycle(
        comp,
        runtime_fsm_id,
        &active_path_prev[..active_len_prev],
        &target_path[..active_len],
        &target_since_ms[..active_len],
    )?;

    let runtime = &mut comp.runtime_nodes[runtime_idx];
    runtime.active_path[..active_len].copy_from_slice(&target_path[..active_len]);
    runtime.active_since_ms[..active_len].copy_from_slice(&target_since_ms[..active_len]);
    runtime.active_len = active_len;
    if !preserves_skill {
        runtime.active_skill_id = None;
        runtime.active_skill_ended = false;
    }
    clear_pending(runtime);
    Ok(())
}

fn clear_pending(runtime: &mut FsmNode) {
    runtime.pending_len = 0;
    runtime.pending_from = None;
    runtime.pending_to = None;
    runtime.pending_started_ms = 0;
    runtime.pending_skill_id = None;
    runtime.pending_skill_ended = false;
}

fn append_pass(comp: &mut FsmComponent, key: ConditionKey) {
    if !comp.pass_pool.contains(&key) {
        comp.pass_pool.push(key);
    }
}

fn remove_pass(comp: &mut FsmComponent, key: ConditionKey) {
    let graph = comp.graph.clone();
    comp.pass_pool.retain(|entry| {
        !(entry.fsm_id == key.fsm_id
            && entry.index == key.index
            && graph.canonical_state(entry.from) == graph.canonical_state(key.from)
            && graph.canonical_state(entry.to) == graph.canonical_state(key.to))
    });
}

fn remove_passes_for_exited_path(
    comp: &mut FsmComponent,
    fsm_id: i32,
    active_path: &[i32],
    target_path: &[i32],
) {
    let common_len = state_hierarchy::common_path_prefix_len(active_path, target_path);
    let exited_path = &active_path[common_len..];
    if exited_path.is_empty() || comp.pass_pool.is_empty() {
        return;
    }
    let canonical_fsm = state_hierarchy::canonical_state(comp, fsm_id);
    let graph = comp.graph.clone();
    comp.pass_pool.retain(|entry| {
        !(entry.fsm_id == canonical_fsm
            && exited_path
                .iter()
                .any(|&s| graph.canonical_state(s) == graph.canonical_state(entry.from)))
    });
}
