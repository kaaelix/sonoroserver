use std::sync::Arc;

use sonoro_gamesv_data::Assets;
use sonoro_gamesv_data::fsm_graph_registry::{Graph, MAX_STATE_DEPTH};
use sonoro_gamesv_data::tables::ai_state_machine_config::AiStateMachineConfig;
use sonoro_proto::pb::{
    ChangeStateNotify, CombatCommon, CombatNotifyData, CombatNotifyDataMessageOneof,
    CombatReceiveData, CombatReceiveDataMessageOneof, DFsm, EntityFsmComponentPb,
};

use crate::fsm::types::{
    BehaviorValidation, ClientPassResult, ConditionKey, ConfirmResult, EvalContext, FsmNode,
    LifecycleEffect, TagCount, TagMirror, Transition, WakeMask, wake_reason,
};
use crate::fsm::{blackboard, config_loader, state_hierarchy, transition_engine};

pub const ENCOUNTER_TARGET_BLACKBOARD_KEY: usize = blackboard::BLACKBOARD_TARGET_KEY;

#[derive(Debug, Clone)]
pub struct FsmComponent {
    pub graph: Arc<Graph>,
    pub runtime_nodes: Vec<FsmNode>,
    pub blackboard: [Option<i32>; 16],
    pub dirty_blackboard_mask: u16,
    pub custom_blackboards: Vec<(String, i32)>,
    pub pass_pool: Vec<ConditionKey>,
    pub tags: Vec<TagCount>,
    pub event: Option<String>,
    pub dissolve_combine_signal: bool,
    pub in_hate: bool,
    pub paralysis_active: bool,
    pub paralysis_last_ms: i64,
    pub paralysis_next_ms: Option<i64>,
    pub instance_state_tag: Option<i32>,
    pub last_tick_ms: i64,
    pub lifecycle_effects: Vec<LifecycleEffect>,
    pub lifecycle_effects_pending: bool,
    pub lifecycle_recheck_requested: bool,
    pub pending_tag_mirrors: Vec<TagMirror>,
}

impl FsmComponent {
    pub fn from_graph(graph: Arc<Graph>, now_ms: i64) -> Result<Self, &'static str> {
        let mut comp = Self {
            graph,
            runtime_nodes: Vec::new(),
            blackboard: [None; 16],
            dirty_blackboard_mask: 0,
            custom_blackboards: Vec::new(),
            pass_pool: Vec::new(),
            tags: Vec::new(),
            event: None,
            dissolve_combine_signal: false,
            in_hate: false,
            paralysis_active: false,
            paralysis_last_ms: 0,
            paralysis_next_ms: None,
            instance_state_tag: None,
            last_tick_ms: now_ms,
            lifecycle_effects: Vec::new(),
            lifecycle_effects_pending: false,
            lifecycle_recheck_requested: false,
            pending_tag_mirrors: Vec::new(),
        };
        comp.init_runtime(now_ms)?;
        Ok(comp)
    }

    pub fn init_runtime(&mut self, now_ms: i64) -> Result<(), &'static str> {
        if !self.runtime_nodes.is_empty() {
            return Ok(());
        }
        let state_list = self.graph.state_list.clone();
        self.runtime_nodes.reserve(state_list.len());

        for root_id in state_list {
            let mut active_path = [0i32; MAX_STATE_DEPTH];
            let Some(active_len) =
                state_hierarchy::build_initial_path(self, root_id, &mut active_path)
            else {
                continue;
            };
            let canonical_root = active_path[0];
            if self
                .runtime_nodes
                .iter()
                .any(|n| n.fsm_id == canonical_root)
            {
                continue;
            }
            let mut active_since_ms = [0i64; MAX_STATE_DEPTH];
            active_since_ms[..active_len].fill(now_ms);
            self.runtime_nodes.push(FsmNode {
                fsm_id: canonical_root,
                active_path,
                active_since_ms,
                active_len,
                dirty_reasons: wake_reason::INITIAL,
                ..Default::default()
            });
        }

        self.event = None;
        for idx in 0..self.runtime_nodes.len() {
            let fsm_id = self.runtime_nodes[idx].fsm_id;
            let len = self.runtime_nodes[idx].active_len;
            let path = self.runtime_nodes[idx].active_path;
            let since = self.runtime_nodes[idx].active_since_ms;
            blackboard::prepare_path(self, &path[..len], &since[..len], false);
            transition_engine::enter_path(self, fsm_id, &path[..len], &since[..len])?;
        }
        transition_engine::refresh_wake_requirements(self, now_ms);
        transition_engine::mark_dirty(self, wake_reason::INITIAL | wake_reason::STATE);
        self.dirty_blackboard_mask = 0;
        self.finish_tick(now_ms);
        Ok(())
    }

    pub fn finish_tick(&mut self, now_ms: i64) {
        self.finish_tick_internal(now_ms, true);
    }

    pub fn finish_tick_preserving_event(&mut self, now_ms: i64) {
        self.finish_tick_internal(now_ms, false);
    }

    fn finish_tick_internal(&mut self, now_ms: i64, clear_event: bool) {
        if self.lifecycle_effects_pending {
            return;
        }
        for runtime in &mut self.runtime_nodes {
            runtime.previous_path[..runtime.active_len]
                .copy_from_slice(&runtime.active_path[..runtime.active_len]);
            runtime.previous_len = runtime.active_len;
        }
        if clear_event {
            self.event = None;
        }
        self.last_tick_ms = now_ms;
    }

    pub fn mark_root_dirty(&mut self, fsm_id: i32, reason: WakeMask) -> bool {
        transition_engine::mark_root_dirty(self, fsm_id, reason)
    }

    pub fn any_root_dirty(&self) -> bool {
        self.runtime_nodes.iter().any(|r| r.dirty_reasons != 0)
    }

    pub fn clear_dirty_reason(&mut self, reason: WakeMask) {
        for runtime in &mut self.runtime_nodes {
            runtime.dirty_reasons &= !reason;
        }
    }

    pub fn activate_paralysis(&mut self, now_ms: i64) {
        self.paralysis_active = true;
        self.paralysis_last_ms = now_ms;
        self.paralysis_next_ms = Some(now_ms + 50);
    }

    pub fn consume_delayed_suicide(&mut self, fsm_id: i32, due_ms: i64) -> bool {
        let Some(rt) = state_hierarchy::runtime_node(self, fsm_id) else {
            return false;
        };
        if rt.delayed_suicide_due_ms != Some(due_ms) || rt.delayed_suicide_fired {
            return false;
        }
        let Some(bound_state) = rt.delayed_suicide_state else {
            return false;
        };
        let active_copy = rt.active_path;
        let active_len = rt.active_len;
        if !state_hierarchy::path_contains(self, &active_copy[..active_len], bound_state) {
            return false;
        }
        if let Some(rt_mut) = state_hierarchy::runtime_node_mut(self, fsm_id) {
            rt_mut.delayed_suicide_due_ms = None;
            rt_mut.delayed_suicide_fired = true;
        }
        true
    }

    pub fn delayed_destroy_is_due(&self, fsm_id: i32, due_ms: i64) -> bool {
        let Some(rt) = state_hierarchy::runtime_node(self, fsm_id) else {
            return false;
        };
        if rt.delayed_destroy_due_ms != Some(due_ms) || !rt.delayed_suicide_fired {
            return false;
        }
        let Some(bound_state) = rt.delayed_suicide_state else {
            return false;
        };
        state_hierarchy::state_belongs_to_fsm(self, rt.fsm_id, bound_state)
    }

    pub fn from_ai_base_id(
        assets: &Assets,
        ai_base_id: i32,
        now_ms: i64,
    ) -> Result<Option<Self>, &'static str> {
        let Some(graph) = config_loader::from_ai_base_id(assets, ai_base_id) else {
            return Ok(None);
        };
        Ok(Some(Self::from_graph(graph, now_ms)?))
    }

    pub fn from_state_machine_id(
        assets: &Assets,
        id: &str,
        now_ms: i64,
    ) -> Result<Option<Self>, &'static str> {
        let Some(graph) = config_loader::from_state_machine_id(assets, id) else {
            return Ok(None);
        };
        Ok(Some(Self::from_graph(graph, now_ms)?))
    }

    pub fn lifecycle_effects(&self) -> &[LifecycleEffect] {
        &self.lifecycle_effects
    }

    pub fn lifecycle_effects_pending(&self) -> bool {
        self.lifecycle_effects_pending
    }

    pub fn mark_lifecycle_effects_enqueued(&mut self) {
        if self.lifecycle_effects.is_empty() {
            return;
        }
        self.lifecycle_effects.clear();
        self.lifecycle_effects_pending = true;
    }

    pub fn clear_lifecycle_effects(&mut self) {
        self.lifecycle_effects.clear();
    }

    pub fn drain_pending_tag_mirrors(&mut self) -> Vec<TagMirror> {
        std::mem::take(&mut self.pending_tag_mirrors)
    }

    pub fn complete_lifecycle_effects(&mut self) {
        self.lifecycle_effects_pending = false;
    }

    pub fn request_lifecycle_recheck(&mut self, requested: bool) {
        self.lifecycle_recheck_requested |= requested;
    }

    pub fn take_lifecycle_recheck_request(&mut self) -> bool {
        std::mem::take(&mut self.lifecycle_recheck_requested)
    }

    pub fn set_encounter_target(&mut self, target_id: Option<i32>) -> bool {
        blackboard::set_value(self, ENCOUNTER_TARGET_BLACKBOARD_KEY, target_id, true)
    }

    pub fn clear_encounter_target_reference(&mut self, target_id: i32) -> bool {
        if self.blackboard[ENCOUNTER_TARGET_BLACKBOARD_KEY] != Some(target_id) {
            return false;
        }
        self.set_encounter_target(None)
    }

    pub fn mark_dirty(&mut self, reason: WakeMask) -> bool {
        transition_engine::mark_dirty(self, reason)
    }

    pub fn pending_deadline(&self) -> Option<i64> {
        transition_engine::pending_deadline(self)
    }

    pub fn recover_expired_pending(&mut self, now_ms: i64) -> Result<bool, &'static str> {
        let recovered = transition_engine::recover_expired_pending(self, now_ms)?;
        if recovered {
            transition_engine::refresh_wake_requirements(self, now_ms);
            transition_engine::mark_dirty(self, wake_reason::INITIAL | wake_reason::STATE);
        }
        Ok(recovered)
    }

    pub fn set_in_hate(&mut self, in_hate: bool) -> bool {
        let old = self.in_hate;
        self.in_hate = in_hate;
        old != self.in_hate
    }

    pub fn signal_dissolve_combine(&mut self) {
        self.dissolve_combine_signal = true;
    }

    pub fn note_skill_start(&mut self, skill_id: i32) {
        if skill_id <= 0 {
            return;
        }
        let graph = self.graph.clone();
        for runtime in &mut self.runtime_nodes {
            if !server_evaluates_root_graph(&graph, runtime.fsm_id) {
                continue;
            }
            let use_pending = runtime.pending_len != 0;
            let path = if use_pending {
                runtime.pending()
            } else {
                runtime.active()
            };
            if !task_accepts_skill_start(&graph, path, skill_id) {
                continue;
            }
            let observed = if use_pending {
                &mut runtime.pending_skill_id
            } else {
                &mut runtime.active_skill_id
            };
            if observed.is_none() {
                *observed = Some(skill_id);
            }
        }
    }

    pub fn signal_skill_end(&mut self, skill_id: i32) -> bool {
        if skill_id <= 0 {
            return false;
        }
        let graph = self.graph.clone();
        let mut wake_roots = Vec::new();
        for runtime in &mut self.runtime_nodes {
            if !server_evaluates_root_graph(&graph, runtime.fsm_id) {
                continue;
            }
            let use_pending = runtime.pending_len != 0;
            let path = if use_pending {
                runtime.pending()
            } else {
                runtime.active()
            };
            let observed_id = if use_pending {
                runtime.pending_skill_id
            } else {
                runtime.active_skill_id
            };
            if !task_skill_matches(&graph, path, observed_id, skill_id) {
                continue;
            }
            if use_pending {
                runtime.pending_skill_ended = true;
            } else {
                runtime.active_skill_ended = true;
            }
            wake_roots.push(runtime.fsm_id);
        }
        let mut wake = false;
        for fsm_id in wake_roots {
            wake |= transition_engine::mark_root_dirty(self, fsm_id, wake_reason::SKILL_END);
        }
        wake
    }

    pub fn record_client_pass(&mut self, key: ConditionKey, value: bool) -> ClientPassResult {
        transition_engine::record_client_pass(self, key, value)
    }

    pub fn confirm_pending(
        &mut self,
        fsm_id: i32,
        state: i32,
        now_ms: i64,
    ) -> Result<ConfirmResult, &'static str> {
        let result = transition_engine::confirm_pending(self, fsm_id, state)?;
        if matches!(result, ConfirmResult::Confirmed | ConfirmResult::Accepted) {
            transition_engine::refresh_wake_requirements(self, now_ms);
            transition_engine::mark_root_dirty(self, fsm_id, wake_reason::INITIAL);
            transition_engine::mark_other_roots_dirty(self, fsm_id, wake_reason::STATE);
        }
        Ok(result)
    }

    pub fn confirm_state_request(
        &mut self,
        fsm_id: i32,
        from: i32,
        to: i32,
        ctx: &EvalContext<'_>,
    ) -> Result<ConfirmResult, &'static str> {
        let result = transition_engine::confirm_state_request(self, fsm_id, from, to, ctx)?;
        if matches!(result, ConfirmResult::Confirmed | ConfirmResult::Accepted) {
            transition_engine::refresh_wake_requirements(self, ctx.now_ms);
            transition_engine::mark_root_dirty(self, fsm_id, wake_reason::INITIAL);
            transition_engine::mark_other_roots_dirty(self, fsm_id, wake_reason::STATE);
        }
        Ok(result)
    }

    pub fn current_state(&self, fsm_id: i32) -> Option<i32> {
        transition_engine::current_state(self, fsm_id)
    }

    pub fn validate_behavior(
        &self,
        fsm_id: i32,
        state: i32,
        index: i32,
        behavior_type: i32,
    ) -> BehaviorValidation {
        state_hierarchy::validate_behavior(self, fsm_id, state, index, behavior_type)
    }

    pub fn append_ready_state_transitions(
        &mut self,
        entity_id: i64,
        output: &mut Vec<CombatReceiveData>,
        ctx: &EvalContext<'_>,
    ) -> Result<(), &'static str> {
        self.append_blackboard_notify(entity_id, output);
        for idx in 0..self.runtime_nodes.len() {
            if self.runtime_nodes[idx].pending_to.is_some() {
                continue;
            }
            let fsm_id = self.runtime_nodes[idx].fsm_id;
            if let Some(transition) = self.find_ready_transition(fsm_id, ctx)? {
                self.append_blackboard_notify(entity_id, output);
                output.push(Self::transition_notify(entity_id, transition));
            }
            self.runtime_nodes[idx].dirty_reasons = 0;
        }
        transition_engine::refresh_wake_requirements(self, ctx.now_ms);
        if self.can_discard_dissolve_combine_signal() {
            self.dissolve_combine_signal = false;
        }
        Ok(())
    }

    pub fn append_dirty_state_transitions(
        &mut self,
        entity_id: i64,
        output: &mut Vec<CombatReceiveData>,
        ctx: &EvalContext<'_>,
    ) -> Result<(), &'static str> {
        self.append_blackboard_notify(entity_id, output);
        for idx in 0..self.runtime_nodes.len() {
            let dirty_reasons = self.runtime_nodes[idx].dirty_reasons;
            if dirty_reasons == 0 || self.runtime_nodes[idx].pending_to.is_some() {
                continue;
            }
            let fsm_id = self.runtime_nodes[idx].fsm_id;
            if let Some(transition) = self.find_ready_transition(fsm_id, ctx)? {
                self.append_blackboard_notify(entity_id, output);
                output.push(Self::transition_notify(entity_id, transition));
            }
            self.runtime_nodes[idx].dirty_reasons = 0;
            if (dirty_reasons & wake_reason::TIMER) != 0 {
                transition_engine::refresh_root_timer(self, fsm_id, ctx.now_ms);
            }
        }
        if self.can_discard_dissolve_combine_signal() {
            self.dissolve_combine_signal = false;
        }
        Ok(())
    }

    pub fn append_blackboard_notify(
        &mut self,
        entity_id: i64,
        output: &mut Vec<CombatReceiveData>,
    ) {
        blackboard::append_blackboard_notify(self, entity_id, output);
    }

    pub fn append_reset_notify(
        &mut self,
        entity_id: i64,
        now_ms: i64,
        output: &mut Vec<CombatReceiveData>,
    ) {
        blackboard::append_reset_notify(self, entity_id, now_ms, output);
    }

    pub fn check_transitions(
        &mut self,
        entity_id: i64,
        fsm_id: i32,
        ctx: &EvalContext<'_>,
    ) -> Result<Option<CombatReceiveData>, &'static str> {
        if state_hierarchy::runtime_node(self, fsm_id).is_some_and(|n| n.pending_to.is_some()) {
            return Ok(None);
        }
        let found = self.find_ready_transition(fsm_id, ctx)?;
        if let Some(node) = state_hierarchy::runtime_node_mut(self, fsm_id) {
            node.dirty_reasons = 0;
        }
        Ok(found.map(|t| Self::transition_notify(entity_id, t)))
    }

    pub fn check_and_confirm(
        &mut self,
        entity_id: i64,
        fsm_id: i32,
        ctx: &EvalContext<'_>,
    ) -> Result<Option<CombatReceiveData>, &'static str> {
        let Some(transition) = self.find_ready_transition(fsm_id, ctx)? else {
            return Ok(None);
        };
        let _ = self.confirm_pending(fsm_id, transition.to, ctx.now_ms)?;
        Ok(Some(Self::transition_notify(entity_id, transition)))
    }

    pub fn transition_notify(entity_id: i64, transition: Transition) -> CombatReceiveData {
        CombatReceiveData {
            Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
                CombatNotifyData {
                    CombatCommon: Some(CombatCommon {
                        EntityId: entity_id,
                        ..Default::default()
                    }),
                    Message: Some(CombatNotifyDataMessageOneof::ChangeStateNotify(
                        ChangeStateNotify {
                            FsmId: transition.fsm_id,
                            FromState: transition.from,
                            ToState: transition.to,
                        },
                    )),
                },
            )),
        }
    }

    pub fn get_initial_fsm(&self, now_ms: i64) -> Vec<DFsm> {
        blackboard::get_initial_fsm(self, now_ms)
    }

    pub fn get_common_fsm(assets: &Assets) -> Option<AiStateMachineConfig> {
        config_loader::get_common_fsm(assets)
    }

    pub fn to_proto(&self, now_ms: i64) -> EntityFsmComponentPb {
        blackboard::to_proto(self, now_ms)
    }

    fn find_ready_transition(
        &mut self,
        fsm_id: i32,
        ctx: &EvalContext<'_>,
    ) -> Result<Option<Transition>, &'static str> {
        transition_engine::find_ready_transition(self, fsm_id, ctx)
    }

    fn can_discard_dissolve_combine_signal(&self) -> bool {
        if !self.dissolve_combine_signal
            || self.lifecycle_effects_pending
            || !self.lifecycle_effects.is_empty()
        {
            return false;
        }
        self.runtime_nodes.iter().all(|r| r.pending_to.is_none())
    }
}

fn server_evaluates_root_graph(graph: &Graph, fsm_id: i32) -> bool {
    let Some(root) = graph.find_node(fsm_id) else {
        return false;
    };
    !root.IsAnimStateMachine.unwrap_or(false)
}

fn task_accepts_skill_start(graph: &Graph, path: &[i32], skill_id: i32) -> bool {
    let Some(&leaf) = path.last() else {
        return false;
    };
    let Some(node) = graph.find_node(leaf) else {
        return false;
    };
    let Some(task) = &node.Task else {
        return false;
    };
    if task.TaskSkillByName.is_some() {
        return true;
    }
    let Some(skill) = &task.TaskSkill else {
        return false;
    };
    if skill.ConfigReplaceTagId != 0 || !skill.ConfigReplaceTagName.is_empty() {
        return true;
    }
    skill.SkillId == skill_id
}

fn task_skill_matches(
    graph: &Graph,
    path: &[i32],
    observed_id: Option<i32>,
    skill_id: i32,
) -> bool {
    let Some(&leaf) = path.last() else {
        return false;
    };
    let Some(node) = graph.find_node(leaf) else {
        return false;
    };
    if node.Task.is_none() {
        return false;
    }
    if let Some(id) = observed_id {
        return id == skill_id;
    }
    let Some(skill) = node.Task.as_ref().and_then(|t| t.TaskSkill.as_ref()) else {
        return false;
    };
    if skill.ConfigReplaceTagId != 0 || !skill.ConfigReplaceTagName.is_empty() {
        return false;
    }
    skill.SkillId == skill_id
}
