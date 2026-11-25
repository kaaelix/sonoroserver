use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use crate::assets_db::AssetsDb;
use crate::tables::ai_state_machine_config::{
    AiStateMachineConfig, StateMachineAction, StateMachineBindState, StateMachineCondition,
    StateMachineJsonData, StateMachineNode, StateMachineTask,
};

pub const TRANSITION_PRESENT: u8 = 1 << 0;
pub const TRANSITION_PREDICTED: u8 = 1 << 1;
pub const MAX_STATE_DEPTH: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskKind {
    Invalid,
    Skill,
    SkillByName,
    RandomMontage,
    LeaveFight,
    Montage,
    MoveToTarget,
    Patrol,
    BeHitMontage,
    GroupPatrol,
    GroupPerform,
    Unknown,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct NodeMetadata {
    pub enter_action_count: usize,
    pub exit_action_count: usize,
    pub bind_count: usize,
    pub take_control_type: i32,
    pub task_kind: Option<TaskKind>,
    pub blackboard_mask: u32,
}

#[derive(Debug, Clone)]
pub struct GraphNode {
    pub key: i32,
    pub value: StateMachineNode,
}

#[derive(Debug, Clone, Default)]
pub struct Graph {
    pub hash_code: i32,
    pub common_hash_code: i32,
    pub state_list: Vec<i32>,
    pub nodes: Vec<GraphNode>,
    pub exact_nodes: HashMap<i32, usize>,
    pub alias_to_resolved: HashMap<i32, i32>,
    pub resolved_to_alias: HashMap<i32, i32>,
    pub transition_flags: HashMap<u64, u8>,
    pub node_metadata: HashMap<i32, NodeMetadata>,
}

impl Graph {
    pub fn canonical_state(&self, state_id: i32) -> i32 {
        self.alias_to_resolved.get(&state_id).copied().unwrap_or(state_id)
    }

    pub fn effective_state(&self, state_id: i32) -> i32 {
        self.resolved_to_alias.get(&state_id).copied().unwrap_or(state_id)
    }

    pub fn client_state(&self, state_id: i32) -> i32 {
        let canonical = self.canonical_state(state_id);
        self.resolved_to_alias
            .get(&canonical)
            .copied()
            .unwrap_or(canonical)
    }

    pub fn find_node_exact(&self, state_id: i32) -> Option<&StateMachineNode> {
        self.exact_nodes.get(&state_id).map(|&idx| &self.nodes[idx].value)
    }

    pub fn find_metadata(&self, state_id: i32) -> Option<NodeMetadata> {
        let canonical = self.canonical_state(state_id);
        if let Some(alias) = self.resolved_to_alias.get(&canonical)
            && let Some(metadata) = self.node_metadata.get(alias)
        {
            return Some(*metadata);
        }
        self.node_metadata.get(&canonical).copied()
    }

    pub fn find_node(&self, state_id: i32) -> Option<&StateMachineNode> {
        let canonical = self.canonical_state(state_id);
        if let Some(alias) = self.resolved_to_alias.get(&canonical)
            && let Some(node) = self.find_node_exact(*alias)
        {
            return Some(node);
        }
        self.find_node_exact(canonical)
    }

    pub fn has_transition(&self, from: i32, to: i32) -> bool {
        let flags = self
            .transition_flags
            .get(&edge_key(self.canonical_state(from), self.canonical_state(to)))
            .copied()
            .unwrap_or(0);
        (flags & TRANSITION_PRESENT) != 0
    }

    pub fn is_predicted_transition(&self, from: i32, to: i32) -> bool {
        let flags = self
            .transition_flags
            .get(&edge_key(self.canonical_state(from), self.canonical_state(to)))
            .copied()
            .unwrap_or(0);
        (flags & TRANSITION_PREDICTED) != 0
    }
}

#[derive(Default)]
struct LazyState {
    graphs: HashMap<String, Arc<Graph>>,
    missing: HashSet<String>,
}

#[derive(Default)]
pub struct FsmGraphRegistry {
    graphs: HashMap<String, Arc<Graph>>,
    db: Option<Arc<AssetsDb>>,
    common_config: Option<Arc<AiStateMachineConfig>>,
    lazy_state: Option<Mutex<LazyState>>,
}

impl FsmGraphRegistry {
    pub fn from_configs(configs: &[AiStateMachineConfig]) -> Self {
        let mut reg = Self {
            graphs: HashMap::new(),
            db: None,
            common_config: None,
            lazy_state: None,
        };
        let Some(common) = configs.iter().find(|c| c.Id == "SM_Common") else {
            return reg;
        };
        for cfg in configs {
            if cfg.Id == "SM_Common" {
                continue;
            }
            if let Some(graph) = build_graph(&cfg.StateMachineJson, &common.StateMachineJson) {
                reg.graphs.insert(cfg.Id.clone(), Arc::new(graph));
            }
        }
        reg
    }

    pub fn with_db(db: Arc<AssetsDb>) -> Self {
        let common_config = db.get_ai_state_machine_config("SM_Common");
        Self {
            graphs: HashMap::new(),
            db: Some(db),
            common_config,
            lazy_state: Some(Mutex::new(LazyState::default())),
        }
    }

    pub fn get(&self, id: &str) -> Option<Arc<Graph>> {
        if let Some(lazy) = &self.lazy_state {
            let db = self.db.as_ref()?;
            let common = self.common_config.as_ref()?;
            let mut guard = lazy.lock().ok()?;
            if let Some(g) = guard.graphs.get(id) {
                return Some(Arc::clone(g));
            }
            if guard.missing.contains(id) {
                return None;
            }
            let Some(cfg) = db.get_ai_state_machine_config(id) else {
                guard.missing.insert(id.to_string());
                return None;
            };
            let Some(graph) = build_graph(&cfg.StateMachineJson, &common.StateMachineJson) else {
                guard.missing.insert(id.to_string());
                return None;
            };
            let arc = Arc::new(graph);
            guard.graphs.insert(id.to_string(), Arc::clone(&arc));
            return Some(arc);
        }
        self.graphs.get(id).cloned()
    }
}

pub fn build_graph(
    state_machine_config: &StateMachineJsonData,
    common_state_machine: &StateMachineJsonData,
) -> Option<Graph> {
    let mut unified_nodes: HashMap<i32, &StateMachineNode> = HashMap::new();
    for node in &common_state_machine.Nodes {
        unified_nodes.insert(node.Uuid, node);
    }
    for node in &state_machine_config.Nodes {
        unified_nodes.insert(node.Uuid, node);
    }

    let mut graph = Graph {
        hash_code: state_machine_config.Version,
        common_hash_code: common_state_machine.Version,
        ..Default::default()
    };

    let mut reference_targets: HashMap<i32, i32> = HashMap::new();
    let mut reference_order: Vec<i32> = Vec::new();
    for node in &common_state_machine.Nodes {
        if let Some(target_id) = node.ReferenceUuid {
            put_reference(&mut reference_targets, &mut reference_order, node.Uuid, target_id);
        }
    }
    for node in &state_machine_config.Nodes {
        if let Some(common_id) = node.OverrideCommonUuid {
            graph.alias_to_resolved.insert(node.Uuid, common_id);
            graph.resolved_to_alias.insert(common_id, node.Uuid);
        }
        if let Some(target_id) = node.ReferenceUuid {
            put_reference(&mut reference_targets, &mut reference_order, node.Uuid, target_id);
        }
    }

    let mut state_list = Vec::new();
    for &root_id in &state_machine_config.StateMachines {
        let effective_id = graph.effective_state(root_id);
        if !unified_nodes.contains_key(&effective_id) {
            continue;
        }
        if !reference_targets.contains_key(&effective_id) && !state_list.contains(&root_id) {
            state_list.push(root_id);
        }
    }
    for alias_id in reference_order {
        let target_id = reference_targets.get(&alias_id).copied()?;
        if !validate_reference_chain(target_id, &unified_nodes, &reference_targets, &graph) {
            return None;
        }
        if !state_list.contains(&target_id) {
            state_list.push(target_id);
        }
    }

    let mut nodes = Vec::new();
    for &root_id in &state_list {
        insert_with_descendants(
            root_id,
            &unified_nodes,
            &graph.resolved_to_alias,
            &reference_targets,
            &mut nodes,
        );
    }

    graph.state_list = state_list;
    for (idx, entry) in nodes.iter().enumerate() {
        graph.exact_nodes.entry(entry.key).or_insert(idx);
        graph.exact_nodes.entry(entry.value.Uuid).or_insert(idx);
        let meta = if reference_targets.contains_key(&entry.key) {
            NodeMetadata::default()
        } else {
            node_metadata(&entry.value)
        };
        graph.node_metadata.entry(entry.key).or_insert(meta);
    }
    for entry in &nodes {
        if reference_targets.contains_key(&entry.key) {
            continue;
        }
        for transition in &entry.value.Transitions {
            let key = edge_key(
                graph.canonical_state(transition.From),
                graph.canonical_state(transition.To),
            );
            let flags = graph.transition_flags.entry(key).or_insert(0);
            *flags |= TRANSITION_PRESENT;
            let prediction_type = transition.TransitionPredictionType.unwrap_or(0);
            if prediction_type == 1 || prediction_type == 2 {
                *flags |= TRANSITION_PREDICTED;
            }
        }
    }
    graph.nodes = nodes;

    if !has_valid_initial_root(&graph) {
        return None;
    }
    Some(graph)
}

pub fn task_kind(task: &StateMachineTask) -> TaskKind {
    let Some(t) = task.Type else {
        return TaskKind::Invalid;
    };
    match t {
        1 => if task.TaskSkill.is_some() { TaskKind::Skill } else { TaskKind::Invalid },
        2 => if task.TaskSkillByName.is_some() { TaskKind::SkillByName } else { TaskKind::Invalid },
        3 => if task.TaskRandomMontage.is_some() { TaskKind::RandomMontage } else { TaskKind::Invalid },
        101 => if task.TaskLeaveFight.is_some() { TaskKind::LeaveFight } else { TaskKind::Invalid },
        102 => if task.TaskMontage.is_some() { TaskKind::Montage } else { TaskKind::Invalid },
        103 => if task.TaskMoveToTarget.is_some() { TaskKind::MoveToTarget } else { TaskKind::Invalid },
        104 => if task.TaskPatrol.is_some() { TaskKind::Patrol } else { TaskKind::Invalid },
        105 => if task.TaskBeHitMontage.is_some() { TaskKind::BeHitMontage } else { TaskKind::Invalid },
        106 => if task.TaskGroupPatrol.is_some() { TaskKind::GroupPatrol } else { TaskKind::Invalid },
        107 => if task.TaskGroupPerform.is_some() { TaskKind::GroupPerform } else { TaskKind::Invalid },
        _ => TaskKind::Unknown,
    }
}

pub fn action_payload_matches(action: &StateMachineAction) -> bool {
    let Some(t) = action.Type else {
        return false;
    };
    match t {
        1 => action.ActionAddBuff.is_some(),
        2 => action.ActionRemoveBuff.is_some(),
        3 => action.ActionCastSkill.is_some(),
        4 => action.ActionCancelSkill.is_some(),
        7 => action.ActionResetStatus.is_some(),
        8 => action.ActionEnterFight.is_some(),
        9 => action.ActionCastSkillByName.is_some(),
        10 => action.ActionCancelSkillByName.is_some(),
        11 => action.ActionInstChangeStateTag.is_some(),
        12 => action.ActionResetPart.is_some(),
        13 => action.ActionActivatePart.is_some(),
        14 => action.ActionActivateSkillGroup.is_some(),
        15 => action.ActionDispatchEvent.is_some(),
        19 => action.ActionSetRageFullAttribute.is_some(),
        20 => action.ActionAddTagCount.is_some(),
        21 => action.ActionRemoveTagCount.is_some(),
        22 => action.ActionDispatchGameEvent.is_some(),
        101 => action.ActionCue.is_some(),
        102 => action.ActionStopMontage.is_some(),
        103 => action.ActionExitHit.is_some(),
        104 => action.ActionSendGameplayEvent.is_some(),
        105 => action.ActionCameraLockOn.is_some(),
        _ => false,
    }
}

pub fn bind_payload_matches(bind: &StateMachineBindState) -> bool {
    let Some(t) = bind.Type else {
        return false;
    };
    match t {
        1 => bind.BindBuff.is_some(),
        2 => bind.BindSkill.is_some(),
        3 => bind.BindTag.is_some(),
        4 => bind.BindSkillByName.is_some(),
        6 => bind.BindSkillCounter.is_some(),
        7 => bind.BindDelaySuicide.is_some(),
        102 => bind.BindAiHateConfig.is_some(),
        103 => bind.BindAiSenseEnable.is_some(),
        104 => bind.BindCue.is_some(),
        105 => bind.BindDisableActor.is_some(),
        108 => bind.BindBoneVisible.is_some(),
        109 => bind.BindMeshVisible.is_some(),
        110 => bind.BindBoneCollision.is_some(),
        111 => bind.BindPartPanelVisible.is_some(),
        112 => bind.BindDeathMontage.is_some(),
        113 => bind.BindPalsy.is_some(),
        114 => bind.BindCollisionChannel.is_some(),
        115 => bind.BindDisableCollision.is_some(),
        116 => bind.BindDeathMontageByTag.is_some(),
        _ => false,
    }
}

pub fn condition_payload_matches(condition: &StateMachineCondition) -> bool {
    let Some(t) = condition.Type else {
        return false;
    };
    match t {
        1 => condition.CondAnd.is_some(),
        2 => condition.CondOr.is_some(),
        4 | 13 | 20 | 23 => true,
        11 => condition.CondHpLessThan.is_some(),
        14 => condition.CondTag.is_some(),
        15 => condition.CondBBValueCompare.is_some(),
        16 => condition.CondAttrCompare.is_some(),
        17 => condition.CondAttribute.is_some(),
        18 => condition.CondAttributeRate.is_some(),
        19 => condition.CondCheckState.is_some(),
        22 => condition.CondTimer.is_some(),
        24 => condition.CondCheckStateByName.is_some(),
        25 => condition.CondInstStateChange.is_some(),
        26 => condition.CondBuffStack.is_some(),
        27 => condition.CondPartLife.is_some(),
        28 => condition.CondCheckPartActivated.is_some(),
        29 => condition.CondListenEvent.is_some(),
        31 => condition.CondCheckLastState.is_some(),
        32 => condition.CondCheckDissolveCombine.is_some(),
        101 => condition.CondTaskFinish.is_some(),
        102 => condition.CondMontageTimeRemaining.is_some(),
        103 => condition.CondListenBeHit.is_some(),
        104 => condition.CondHasMoveInput.is_some(),
        105 => condition.CondMontageTimeElapsing.is_some(),
        106 => condition.CondCheckGroupPatrol.is_some(),
        108 => condition.CondCheckPositionState.is_some(),
        109 => condition.CondCheckGroupPerform.is_some(),
        _ => false,
    }
}

fn node_metadata(node: &StateMachineNode) -> NodeMetadata {
    let mut meta = NodeMetadata {
        enter_action_count: node.OnEnterActions.len(),
        exit_action_count: node.OnExitActions.len(),
        bind_count: node.BindStates.len(),
        take_control_type: node.TakeControlType,
        task_kind: None,
        blackboard_mask: 0,
    };
    if let Some(task) = &node.Task {
        meta.task_kind = Some(task_kind(task));
        if let Some(montage) = &task.TaskRandomMontage
            && !montage.RandomByClient
            && !montage.MontageNames.is_empty()
        {
            meta.blackboard_mask |= 1 << 1;
        }
        if let Some(mv) = &task.TaskMoveToTarget
            && mv.TargetType == 0
        {
            meta.blackboard_mask |= 1 << 2;
        }
    }
    meta
}

fn insert_with_descendants(
    node_id: i32,
    source: &HashMap<i32, &StateMachineNode>,
    overrides: &HashMap<i32, i32>,
    references: &HashMap<i32, i32>,
    target: &mut Vec<GraphNode>,
) {
    let effective_id = overrides.get(&node_id).copied().unwrap_or(node_id);
    let Some(&node) = source.get(&effective_id) else {
        return;
    };
    if target.iter().any(|entry| entry.key == effective_id) {
        return;
    }

    if references.contains_key(&effective_id) {
        let ref_node = StateMachineNode {
            Uuid: node.Uuid,
            Name: node.Name.clone(),
            ..Default::default()
        };
        target.push(GraphNode {
            key: effective_id,
            value: ref_node,
        });
        return;
    }

    target.push(GraphNode {
        key: effective_id,
        value: node.clone(),
    });
    if let Some(children) = &node.Children {
        for &child_id in children {
            insert_with_descendants(child_id, source, overrides, references, target);
        }
    }
}

fn put_reference(
    targets: &mut HashMap<i32, i32>,
    order: &mut Vec<i32>,
    alias_id: i32,
    target_id: i32,
) {
    if let std::collections::hash_map::Entry::Vacant(e) = targets.entry(alias_id) {
        order.push(alias_id);
        e.insert(target_id);
    } else {
        targets.insert(alias_id, target_id);
    }
}

fn validate_reference_chain(
    start_id: i32,
    nodes: &HashMap<i32, &StateMachineNode>,
    references: &HashMap<i32, i32>,
    graph: &Graph,
) -> bool {
    let mut path = [0i32; MAX_STATE_DEPTH];
    let mut len = 0usize;
    let mut current = start_id;

    loop {
        let effective_id = graph.effective_state(current);
        if len >= path.len() || path[..len].contains(&effective_id) {
            return false;
        }
        path[len] = effective_id;
        len += 1;

        if !nodes.contains_key(&effective_id) {
            return false;
        }
        let Some(&target_id) = references.get(&effective_id) else {
            return true;
        };
        current = target_id;
    }
}

fn has_valid_initial_root(graph: &Graph) -> bool {
    for &raw_id in &graph.state_list {
        let mut path = [0i32; MAX_STATE_DEPTH];
        let mut len = 0usize;
        let mut current = graph.canonical_state(raw_id);

        loop {
            if len >= path.len() || path[..len].contains(&current) {
                break;
            }
            path[len] = current;
            len += 1;

            let Some(node) = graph.find_node(current) else {
                break;
            };
            let Some(children) = &node.Children else {
                return true;
            };
            if children.is_empty() {
                return true;
            }
            current = graph.canonical_state(children[0]);
        }
    }
    false
}

const fn edge_key(from: i32, to: i32) -> u64 {
    ((from as u32 as u64) << 32) | (to as u32 as u64)
}
