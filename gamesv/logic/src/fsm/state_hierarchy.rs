use sonoro_gamesv_data::fsm_graph_registry::{
    MAX_STATE_DEPTH, TaskKind, action_payload_matches, bind_payload_matches,
};
use sonoro_gamesv_data::tables::ai_state_machine_config::{
    AiStateMachineConfig, StateMachineNode,
};

use crate::component::entity::fsm::FsmComponent;
use crate::fsm::types::{
    BehaviorValidation, ClientConditionRequirement, FsmNode, ResolvedStates,
};

pub fn canonical_state(comp: &FsmComponent, state_id: i32) -> i32 {
    comp.graph.canonical_state(state_id)
}

pub fn client_state(comp: &FsmComponent, state_id: i32) -> i32 {
    comp.graph.client_state(state_id)
}

pub fn states_equivalent(comp: &FsmComponent, a: i32, b: i32) -> bool {
    canonical_state(comp, a) == canonical_state(comp, b)
}

pub fn runtime_node(comp: &FsmComponent, fsm_id: i32) -> Option<&FsmNode> {
    let canonical = canonical_state(comp, fsm_id);
    comp.runtime_nodes.iter().find(|n| n.fsm_id == canonical)
}

pub fn runtime_node_mut(comp: &mut FsmComponent, fsm_id: i32) -> Option<&mut FsmNode> {
    let canonical = canonical_state(comp, fsm_id);
    comp.runtime_nodes
        .iter_mut()
        .find(|n| n.fsm_id == canonical)
}

pub fn find_node_exact(comp: &FsmComponent, state_id: i32) -> Option<&StateMachineNode> {
    comp.graph.find_node_exact(state_id)
}

pub fn find_node(comp: &FsmComponent, state_id: i32) -> Option<&StateMachineNode> {
    comp.graph.find_node(state_id)
}

pub fn path_contains(comp: &FsmComponent, path: &[i32], target: i32) -> bool {
    path.iter().any(|&s| states_equivalent(comp, s, target))
}

pub fn common_path_prefix_len(a: &[i32], b: &[i32]) -> usize {
    let limit = a.len().min(b.len());
    let mut len = 0;
    while len < limit && a[len] == b[len] {
        len += 1;
    }
    len
}

pub fn state_contains(
    comp: &FsmComponent,
    root_id: i32,
    target_id: i32,
    depth: usize,
) -> bool {
    if depth >= MAX_STATE_DEPTH {
        return false;
    }
    if states_equivalent(comp, root_id, target_id) {
        return true;
    }
    let Some(node) = find_node(comp, root_id) else {
        return false;
    };
    let Some(children) = node.Children.as_deref() else {
        return false;
    };
    children
        .iter()
        .any(|&child| state_contains(comp, child, target_id, depth + 1))
}

pub fn state_belongs_to_fsm(comp: &FsmComponent, fsm_id: i32, state_id: i32) -> bool {
    let Some(runtime) = runtime_node(comp, fsm_id) else {
        return false;
    };
    state_contains(comp, runtime.fsm_id, state_id, 0)
}

pub fn is_conduit_state(comp: &FsmComponent, state_id: i32) -> bool {
    find_node(comp, state_id)
        .map(|n| n.IsConduitNode)
        .unwrap_or(false)
}

pub fn resolve_override_states(
    comp: &FsmComponent,
    from: i32,
    to: i32,
    reverse: bool,
) -> ResolvedStates {
    if reverse {
        return ResolvedStates {
            from: client_state(comp, from),
            to: client_state(comp, to),
        };
    }

    ResolvedStates {
        from: canonical_state(comp, from),
        to: canonical_state(comp, to),
    }
}

pub fn build_initial_path(
    comp: &FsmComponent,
    root_id: i32,
    out: &mut [i32; MAX_STATE_DEPTH],
) -> Option<usize> {
    let mut len = 0usize;
    let mut current = canonical_state(comp, root_id);
    while len < MAX_STATE_DEPTH {
        if out[..len].contains(&current) {
            return None;
        }
        out[len] = current;
        len += 1;
        let node = find_node(comp, current)?;
        let Some(children) = node.Children.as_deref() else {
            return Some(len);
        };
        if children.is_empty() {
            return Some(len);
        }
        current = canonical_state(comp, children[0]);
    }
    None
}

pub fn build_active_path(
    comp: &FsmComponent,
    root_id: i32,
    target_id: i32,
    out: &mut [i32; MAX_STATE_DEPTH],
) -> Option<usize> {
    let mut path_len = build_path_to_node(comp, root_id, target_id, 0, out)?;
    let mut current = out[path_len - 1];
    while path_len < MAX_STATE_DEPTH {
        let node = find_node(comp, current)?;
        let Some(children) = node.Children.as_deref() else {
            return Some(path_len);
        };
        if children.is_empty() {
            return Some(path_len);
        }
        current = canonical_state(comp, children[0]);
        if out[..path_len].contains(&current) {
            return None;
        }
        out[path_len] = current;
        path_len += 1;
    }
    None
}

fn build_path_to_node(
    comp: &FsmComponent,
    current_id: i32,
    target_id: i32,
    depth: usize,
    out: &mut [i32; MAX_STATE_DEPTH],
) -> Option<usize> {
    if depth >= MAX_STATE_DEPTH {
        return None;
    }
    let canonical = canonical_state(comp, current_id);
    if out[..depth].contains(&canonical) {
        return None;
    }
    out[depth] = canonical;
    if states_equivalent(comp, canonical, target_id) {
        return Some(depth + 1);
    }
    let node = find_node(comp, canonical)?;
    let children = node.Children.as_deref()?;
    for &child in children {
        if let Some(len) = build_path_to_node(comp, child, target_id, depth + 1, out) {
            return Some(len);
        }
    }
    None
}

pub fn validate_behavior(
    comp: &FsmComponent,
    fsm_id: i32,
    state: i32,
    index: i32,
    behavior_type: i32,
) -> BehaviorValidation {
    let Some(runtime) = runtime_node(comp, fsm_id) else {
        return BehaviorValidation::MachineNotFound;
    };
    if !state_belongs_to_fsm(comp, runtime.fsm_id, state) {
        return BehaviorValidation::InvalidState;
    }
    let path_matches = match behavior_type {
        0 | 2 | 4 => {
            path_contains(comp, runtime.active(), state)
                || path_contains(comp, runtime.pending(), state)
        }
        1 | 3 => path_contains(comp, runtime.previous(), state),
        _ => return BehaviorValidation::Mismatch,
    };
    if !path_matches {
        return BehaviorValidation::InactiveState;
    }
    if index < 0 {
        return BehaviorValidation::Mismatch;
    }
    let Some(node) = find_node(comp, state) else {
        return BehaviorValidation::NodeNotFound;
    };
    let Some(meta) = comp.graph.find_metadata(state) else {
        return BehaviorValidation::MetadataNotFound;
    };

    let behavior_index = index as usize;
    let matched = match behavior_type {
        0 => {
            behavior_index < meta.enter_action_count
                && node
                    .OnEnterActions
                    .get(behavior_index)
                    .is_some_and(action_payload_matches)
        }
        1 => {
            behavior_index < meta.exit_action_count
                && node
                    .OnExitActions
                    .get(behavior_index)
                    .is_some_and(action_payload_matches)
        }
        2 | 3 => {
            behavior_index < meta.bind_count
                && node
                    .BindStates
                    .get(behavior_index)
                    .is_some_and(bind_payload_matches)
        }
        4 => behavior_index == 0 && task_kind_is_usable(comp, state),
        _ => false,
    };
    if matched {
        BehaviorValidation::Ok
    } else {
        BehaviorValidation::Mismatch
    }
}

fn task_kind_is_usable(comp: &FsmComponent, state: i32) -> bool {
    comp.graph
        .find_metadata(state)
        .and_then(|meta| meta.task_kind)
        .is_some_and(|kind| !matches!(kind, TaskKind::Unknown | TaskKind::Invalid))
}

pub fn client_task_requirement_satisfied(
    comp: &FsmComponent,
    path: &[i32],
    from: i32,
    to: i32,
    requirement: ClientConditionRequirement,
) -> bool {
    if requirement == ClientConditionRequirement::None {
        return true;
    }
    if path.is_empty() {
        return false;
    }
    match requirement {
        ClientConditionRequirement::None => true,
        ClientConditionRequirement::Task => task_kind_is_usable(comp, path[path.len() - 1]),
        ClientConditionRequirement::Montage => comp
            .graph
            .find_metadata(from)
            .and_then(|m| m.task_kind)
            .is_some_and(|k| {
                matches!(
                    k,
                    TaskKind::Montage | TaskKind::RandomMontage | TaskKind::BeHitMontage
                )
            }),
        ClientConditionRequirement::GroupPatrol => comp
            .graph
            .find_metadata(to)
            .and_then(|m| m.task_kind)
            .is_some_and(|k| k == TaskKind::GroupPatrol),
        ClientConditionRequirement::GroupPerform => comp
            .graph
            .find_metadata(to)
            .and_then(|m| m.task_kind)
            .is_some_and(|k| k == TaskKind::GroupPerform),
    }
}

pub fn get_common_state_machine_id(cfg: &AiStateMachineConfig) -> i32 {
    cfg.StateMachineJson.StateMachines.first().copied().unwrap_or(0)
}
