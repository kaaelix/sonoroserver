use sonoro_gamesv_logic::component::entity::TagComponent;
use sonoro_gamesv_logic::component::entity::fsm::FsmComponent;
use sonoro_gamesv_logic::component::entity::small::EntityState as LogicEntityState;
use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::fsm::types::{
    BehaviorValidation, ClientPassResult, ConditionKey, ConfirmResult, EVAL_FLAG_WAIT_CLIENT,
    EvalContext, PartState, wake_reason,
};
use sonoro_gamesv_logic::fsm_lifecycle;
use sonoro_gamesv_logic::helpers::quest_engine;
use sonoro_gamesv_logic::scene::now_millis;
use sonoro_proto::enums::{EntityState, ErrorCode};
use sonoro_proto::pb::{
    ANStartPush, ActorVisibleNotify, ActorVisiblePush, ActorVisibleRequest, ActorVisibleResponse,
    AiHateEntity, AiHatePush, AiHateRequest, AiHateResponse, AiInformation, AiInformationPush,
    AiInformationRequest, AiInformationResponse, AnimationGameplayTagNotify,
    AnimationGameplayTagPush, AnimationGameplayTagRequest, AnimationGameplayTagResponse,
    BattleStateChangeNotify, BattleStateChangePush, BattleStateChangeRequest,
    BattleStateChangeResponse, BlackboardParamValueOneof, ChangeStateConfirmPush,
    ChangeStateConfirmRequest, ChangeStateConfirmResponse, ChangeStateNotify, ChangeStateRequest,
    ChangeStateResponse, CombatCommon, CombatNotifyDataMessageOneof, CombatReceiveData,
    CombatReceiveDataMessageOneof, CombatReceivePackNotify, DErrorResult, EntityIsVisibleNotify,
    EntityIsVisiblePush, EntityIsVisibleRequest, EntityIsVisibleResponse, FsmConditionPassPush,
    FsmConditionPassRequest, FsmConditionPassResponse, FsmPlayMontagePush, FsmPlayMontageRequest,
    FsmPlayMontageResponse, FsmStateBehaviorRequest, FsmStateBehaviorResponse, HitEndRequest,
    HitEndResponse, HitInformation, HitNotify, HitRequest, HitResponse, LogicStateComponentPb, LogicStateInitPush, LogicStateInitRequest,
    LogicStateInitResponse, MontagePlayPush, PartInformation, PartUpdateInfo, PartUpdateNotify,
    PartUpdatePush, PartUpdateRequest, PartUpdateResponse, RoleTagChangePush,
    SwitchCharacterStateNotify, SwitchCharacterStatePush, SwitchCharacterStateRequest,
    SwitchCharacterStateResponse, SwitchLogicStatePush, SwitchLogicStateRequest,
    SwitchLogicStateResponse,
};
use sonoro_proto::wire::PbEnum;

use super::State;
use super::combat::push_combat_notify;

struct EvalBuffers {
    attributes: Vec<i32>,
    tags: Vec<i32>,
    buff_stacks: Vec<(i64, i32)>,
    parts: Vec<PartState>,
    position_state: Option<i32>,
    instance_state: Option<i32>,
}

fn collect_eval_buffers(state: &State, entity_id: i64) -> Option<EvalBuffers> {
    let scene = state.scene.as_ref()?;
    let handle = scene.entity_handle(entity_id)?;
    let storage = &scene.entities[handle.index];
    let attributes = storage
        .attribute
        .as_ref()
        .map(|a| a.attributes.iter().map(|entry| entry.current).collect())
        .unwrap_or_default();
    let tags = storage
        .tag
        .as_ref()
        .map(|t| t.active_tag_ids())
        .unwrap_or_default();
    let buff_stacks = storage
        .buffs
        .as_ref()
        .map(|buffs| buffs.buffs.iter().map(|b| (b.BuffId, b.StackCount)).collect())
        .unwrap_or_default();
    let parts = storage
        .part
        .as_ref()
        .map(|p| p.to_fsm_states())
        .unwrap_or_default();
    let position_state = storage.logic_state.as_ref().map(|ls| ls.position_state);
    let instance_state = storage.fsm.as_ref().and_then(|f| f.instance_state_tag);
    Some(EvalBuffers {
        attributes,
        tags,
        buff_stacks,
        parts,
        position_state,
        instance_state,
    })
}

fn common_entity_id(common: Option<&CombatCommon>) -> Option<i64> {
    common.map(|c| c.EntityId).filter(|&id| id != 0)
}

fn resolve_explicit_or_common(explicit_id: i64, common: Option<&CombatCommon>) -> i64 {
    if explicit_id != 0 {
        explicit_id
    } else {
        common_entity_id(common).unwrap_or(0)
    }
}

fn derror_ok() -> Option<DErrorResult> {
    Some(DErrorResult {
        ErrorCode: Some(ErrorCode::Success),
        ErrorParams: Vec::new(),
    })
}

fn derror_code(code: ErrorCode, params: &[i32]) -> Option<DErrorResult> {
    Some(DErrorResult {
        ErrorCode: Some(code),
        ErrorParams: params.iter().map(|p| p.to_string().into_bytes()).collect(),
    })
}

fn notify_quest_fsm_state_changed(
    state: &mut State,
    events: &mut EventQueue,
    config_id: i32,
    state_name: &str,
) {
    let mut qctx = state.quest_context(events);
    if let Err(err) = quest_engine::on_entity_state_changed(&mut qctx, config_id, state_name) {
        tracing::warn!(target: "fsm", "onEntityStateChanged failed: {err}");
    }
}

fn extract_change_state_notify(notify: &CombatReceiveData) -> Option<&ChangeStateNotify> {
    let CombatReceiveDataMessageOneof::CombatNotifyData(combat) = notify.Message.as_ref()? else {
        return None;
    };
    match combat.Message.as_ref()? {
        CombatNotifyDataMessageOneof::ChangeStateNotify(ch) => Some(ch),
        _ => None,
    }
}

pub fn apply_hit_to_entity_fsm(
    state: &mut State,
    events: &mut EventQueue,
    entity_id: i64,
    pack: &mut Vec<CombatReceiveData>,
    hit: Option<&HitInformation>,
) {
    if entity_id == 0 {
        return;
    }
    let now_ms = now_millis();
    let Some(buffers) = collect_eval_buffers(state, entity_id) else {
        return;
    };
    let parent_tags = &state.assets.tables.gameplay_tag_parent;
    let Some(scene) = state.scene.as_mut() else {
        return;
    };
    let Some(handle) = scene.entity_handle(entity_id) else {
        return;
    };
    let need_sync = {
        let Some(fsm) = scene.entities[handle.index].fsm.as_mut() else {
            return;
        };
        let _ = fsm.init_runtime(now_ms);
        fsm.set_custom_blackboard_int("be_hit:active", 1);
        if let Some(h) = hit {
            fsm.set_custom_blackboard_int("be_hit:anim", h.BeHitAnim);
            fsm.set_custom_blackboard_int("be_hit:parry", i32::from(h.IsTriggerCounterattack));
            fsm.set_custom_blackboard_int("be_hit:break_weakness", i32::from(h.IsHitWeakness));
            fsm.set_custom_blackboard_int(
                "be_hit:vision_counter",
                if h.IsTriggerVisionCounterAttack { 1 } else { 0 },
            );
        }
        fsm.event = Some("BeHit".to_string());
        fsm.mark_root_dirty_all(wake_reason::EVENT | wake_reason::BLACKBOARD);
        let ctx = EvalContext {
            now_ms,
            flags: 0,
            position_state: buffers.position_state,
            attributes: &buffers.attributes,
            tags: &buffers.tags,
            parent_tags: Some(parent_tags),
            buff_stacks: &buffers.buff_stacks,
            parts: &buffers.parts,
            instance_state: buffers.instance_state,
        };
        let sync = if !fsm_lifecycle::enqueue_effects(handle, fsm, events, now_ms) {
            let _ = fsm.append_dirty_state_transitions(entity_id, pack, &ctx);
            false
        } else {
            true
        };
        fsm.finish_tick(now_ms);
        sync
    };
    if need_sync {
        scene.queue_fsm_sync(entity_id);
    }
    scene.sync_fsm_deadlines_for_entity(entity_id);
}

pub fn apply_montage_to_entity_fsm(
    state: &mut State,
    events: &mut EventQueue,
    entity_id: i64,
    montage_name: &str,
    pack: &mut Vec<CombatReceiveData>,
) {
    if entity_id == 0 || montage_name.is_empty() {
        return;
    }
    let now_ms = now_millis();
    let Some(buffers) = collect_eval_buffers(state, entity_id) else {
        return;
    };
    let parent_tags = &state.assets.tables.gameplay_tag_parent;
    let Some(scene) = state.scene.as_mut() else {
        return;
    };
    let Some(handle) = scene.entity_handle(entity_id) else {
        return;
    };
    let need_sync = {
        let Some(fsm) = scene.entities[handle.index].fsm.as_mut() else {
            return;
        };
        let _ = fsm.init_runtime(now_ms);
        fsm.set_custom_blackboard_int("montage:playing", 1);
        fsm.event = Some(montage_name.to_string());
        fsm.mark_root_dirty_all(wake_reason::EVENT | wake_reason::BLACKBOARD);
        let ctx = EvalContext {
            now_ms,
            flags: 0,
            position_state: buffers.position_state,
            attributes: &buffers.attributes,
            tags: &buffers.tags,
            parent_tags: Some(parent_tags),
            buff_stacks: &buffers.buff_stacks,
            parts: &buffers.parts,
            instance_state: buffers.instance_state,
        };
        let sync = if !fsm_lifecycle::enqueue_effects(handle, fsm, events, now_ms) {
            let _ = fsm.append_dirty_state_transitions(entity_id, pack, &ctx);
            false
        } else {
            true
        };
        fsm.finish_tick(now_ms);
        sync
    };
    if need_sync {
        scene.queue_fsm_sync(entity_id);
    }
    scene.sync_fsm_deadlines_for_entity(entity_id);
}

pub fn handle_hit_request(
    common: Option<&CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    req: HitRequest,
) -> HitResponse {
    if common.is_some() || req.HitInfo.is_some() {
        push_combat_notify(
            pack,
            common.cloned(),
            CombatNotifyDataMessageOneof::HitNotify(HitNotify {
                CombatCommon: common.cloned(),
                HitInfo: req.HitInfo.clone(),
            }),
        );
    }
    HitResponse {
        HitInfo: req.HitInfo,
        ErrorCode: Some(ErrorCode::Success),
    }
}

pub fn handle_hit_end_request(
    state: &mut State,
    events: &mut EventQueue,
    req: HitEndRequest,
) -> HitEndResponse {
    let target_id = resolve_explicit_or_common(req.TargetId, req.CombatCommon.as_ref());
    if target_id != 0 {
        let now_ms = now_millis();
        if let Some(buffers) = collect_eval_buffers(state, target_id) {
            let parent_tags = &state.assets.tables.gameplay_tag_parent;
            let mut pack = Vec::new();
            if let Some(scene) = state.scene.as_mut()
                && let Some(handle) = scene.entity_handle(target_id)
            {
                let mut need_sync = false;
                if let Some(fsm) = scene.entities[handle.index].fsm.as_mut() {
                    let _ = fsm.init_runtime(now_ms);
                    fsm.set_custom_blackboard_int("be_hit:active", 0);
                    fsm.set_custom_blackboard_int("be_hit:parry", 0);
                    fsm.set_custom_blackboard_int("be_hit:break_weakness", 0);
                    fsm.set_custom_blackboard_int("be_hit:vision_counter", 0);
                    fsm.set_custom_blackboard_int("be_hit:ended", 1);
                    fsm.event = Some("HitEnd".to_string());
                    fsm.mark_root_dirty_all(wake_reason::EVENT | wake_reason::BLACKBOARD);
                    let ctx = EvalContext {
                        now_ms,
                        flags: 0,
                        position_state: buffers.position_state,
                        attributes: &buffers.attributes,
                        tags: &buffers.tags,
                        parent_tags: Some(parent_tags),
                        buff_stacks: &buffers.buff_stacks,
                        parts: &buffers.parts,
                        instance_state: buffers.instance_state,
                    };
                    if !fsm_lifecycle::enqueue_effects(handle, fsm, events, now_ms) {
                        let _ = fsm.append_dirty_state_transitions(target_id, &mut pack, &ctx);
                    } else {
                        need_sync = true;
                    }
                    fsm.finish_tick(now_ms);
                }
                if need_sync {
                    scene.queue_fsm_sync(target_id);
                }
                scene.sync_fsm_deadlines_for_entity(target_id);
            }
            if !pack.is_empty() {
                let _ = state
                    .conn
                    .push_msg(CombatReceivePackNotify::MSG_ID, &CombatReceivePackNotify { Data: pack });
            }
        }
    }
    HitEndResponse {
        ErrorCode: Some(ErrorCode::Success),
    }
}

fn apply_switch_character_state(
    state: &mut State,
    common: Option<&CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    id: i64,
    old_state: Option<EntityState>,
    new_state: Option<EntityState>,
) {
    let target_id = resolve_explicit_or_common(id, common);
    if target_id == 0 {
        return;
    }
    let Some(next_state) = new_state else {
        return;
    };
    let Some(scene) = state.scene.as_mut() else {
        return;
    };
    let Some(handle) = scene.entity_handle(target_id) else {
        return;
    };
    let prev_state = match scene.entities[handle.index].config.state {
        LogicEntityState::Default => EntityState::Default,
        LogicEntityState::Sleep => EntityState::Sleep,
        LogicEntityState::Born => EntityState::Born,
        LogicEntityState::Other => EntityState::Other,
    };
    scene.entities[handle.index].config.state = match next_state {
        EntityState::Default => LogicEntityState::Default,
        EntityState::Sleep => LogicEntityState::Sleep,
        EntityState::Born => LogicEntityState::Born,
        EntityState::Other => LogicEntityState::Other,
    };
    push_combat_notify(
        pack,
        Some(CombatCommon {
            EntityId: target_id,
            ..Default::default()
        }),
        CombatNotifyDataMessageOneof::SwitchCharacterStateNotify(SwitchCharacterStateNotify {
            CombatCommon: Some(CombatCommon {
                EntityId: target_id,
                ..Default::default()
            }),
            Id: target_id,
            OldState: old_state.or(Some(prev_state)),
            NewState: Some(next_state),
        }),
    );
}

pub fn handle_switch_character_state_push(
    state: &mut State,
    _events: &mut EventQueue,
    common: Option<&CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    push: SwitchCharacterStatePush,
) {
    apply_switch_character_state(
        state,
        push.CombatCommon.as_ref().or(common),
        pack,
        push.Id,
        push.OldState,
        push.NewState,
    );
}

pub fn handle_switch_character_state_request(
    state: &mut State,
    _events: &mut EventQueue,
    common: Option<&CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    req: SwitchCharacterStateRequest,
) -> SwitchCharacterStateResponse {
    apply_switch_character_state(
        state,
        req.CombatCommon.as_ref().or(common),
        pack,
        req.Id,
        req.OldState,
        req.NewState,
    );
    SwitchCharacterStateResponse {
        ErrorCode: Some(ErrorCode::Success),
    }
}

fn apply_entity_visible(
    state: &mut State,
    common: Option<&CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    id: i64,
    is_visible: bool,
) {
    let target_id = resolve_explicit_or_common(id, common);
    if target_id == 0 {
        return;
    }
    let Some(scene) = state.scene.as_mut() else {
        return;
    };
    let Some(handle) = scene.entity_handle(target_id) else {
        return;
    };
    scene.entities[handle.index].visible = is_visible;
    let notify_common = CombatCommon {
        EntityId: target_id,
        ..Default::default()
    };
    push_combat_notify(
        pack,
        Some(notify_common.clone()),
        CombatNotifyDataMessageOneof::EntityIsVisibleNotify(EntityIsVisibleNotify {
            Id: target_id,
            IsVisible: is_visible,
            CombatCommon: Some(notify_common),
        }),
    );
}

pub fn handle_entity_is_visible_push(
    state: &mut State,
    common: Option<&CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    push: EntityIsVisiblePush,
) {
    apply_entity_visible(
        state,
        push.CombatCommon.as_ref().or(common),
        pack,
        push.Id,
        push.IsVisible,
    );
}

pub fn handle_entity_is_visible_request(
    state: &mut State,
    common: Option<&CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    req: EntityIsVisibleRequest,
) -> EntityIsVisibleResponse {
    apply_entity_visible(
        state,
        req.CombatCommon.as_ref().or(common),
        pack,
        req.Id,
        req.IsVisible,
    );
    EntityIsVisibleResponse {
        ErrorCode: Some(ErrorCode::Success),
    }
}

fn apply_actor_visible(
    state: &mut State,
    common: Option<&CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    id: i64,
    is_actor_visible: bool,
) {
    let target_id = resolve_explicit_or_common(id, common);
    if target_id == 0 {
        return;
    }
    let Some(scene) = state.scene.as_mut() else {
        return;
    };
    let Some(handle) = scene.entity_handle(target_id) else {
        return;
    };
    scene.entities[handle.index].actor_visible = is_actor_visible;
    push_combat_notify(
        pack,
        Some(CombatCommon {
            EntityId: target_id,
            ..Default::default()
        }),
        CombatNotifyDataMessageOneof::ActorVisibleNotify(ActorVisibleNotify {
            Id: target_id,
            IsActorVisible: is_actor_visible,
        }),
    );
}

pub fn handle_actor_visible_push(
    state: &mut State,
    common: Option<&CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    push: ActorVisiblePush,
) {
    apply_actor_visible(state, common, pack, push.Id, push.IsActorVisible);
}

pub fn handle_actor_visible_request(
    state: &mut State,
    common: Option<&CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    req: ActorVisibleRequest,
) -> ActorVisibleResponse {
    apply_actor_visible(state, common, pack, req.Id, req.IsActorVisible);
    ActorVisibleResponse {
        ErrorCode: Some(ErrorCode::Success),
    }
}

fn apply_part_update(
    state: &mut State,
    common: Option<&CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    entity_id: i64,
    updates: &[PartUpdateInfo],
) {
    let target_id = resolve_explicit_or_common(entity_id, common);
    if target_id == 0 || updates.is_empty() {
        return;
    }
    let Some(scene) = state.scene.as_mut() else {
        return;
    };
    let Some(handle) = scene.entity_handle(target_id) else {
        return;
    };

    let mut part_infos: Vec<PartInformation> = Vec::new();
    let mut any_activation = false;
    let mut any_changed = false;
    let has_fsm = {
        let storage = &mut scene.entities[handle.index];
        if let Some(part_comp) = storage.part.as_mut() {
            for upd in updates {
                let mut found_idx = None;
                for (idx, part) in part_comp.parts.iter_mut().enumerate() {
                    if part.index != upd.PartIndex {
                        continue;
                    }
                    if upd.Reset {
                        part.life = part.life_max;
                    }
                    if part.activated != upd.Activated {
                        part.activated = upd.Activated;
                        any_activation = true;
                    }
                    any_changed = true;
                    found_idx = Some(idx);
                    break;
                }
                if let Some(idx) = found_idx {
                    part_comp.append_part_info(idx, &mut part_infos);
                }
            }
            if any_activation {
                part_comp.sync_active_tags(storage.tag.as_mut());
            }
        }
        storage.fsm.is_some()
    };

    if has_fsm {
        if any_activation {
            scene.mark_fsm_dirty(target_id, wake_reason::PART | wake_reason::TAG);
        } else if any_changed {
            scene.mark_fsm_dirty(target_id, wake_reason::PART);
        }
    }

    if !part_infos.is_empty() {
        push_combat_notify(
            pack,
            Some(CombatCommon {
                EntityId: target_id,
                ..Default::default()
            }),
            CombatNotifyDataMessageOneof::PartUpdateNotify(PartUpdateNotify {
                EntityId: target_id,
                PartInfos: part_infos,
            }),
        );
    }
}

pub fn handle_part_update_push(
    state: &mut State,
    _events: &mut EventQueue,
    common: Option<&CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    push: PartUpdatePush,
) {
    apply_part_update(state, common, pack, push.EntityId, &push.PartUpdateInfos);
}

pub fn handle_part_update_request(
    state: &mut State,
    _events: &mut EventQueue,
    common: Option<&CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    req: PartUpdateRequest,
) -> PartUpdateResponse {
    apply_part_update(state, common, pack, req.EntityId, &req.PartUpdateInfos);
    PartUpdateResponse {
        ErrorCode: Some(ErrorCode::Success),
    }
}

fn append_animation_gameplay_tag_notify(
    pack: &mut Vec<CombatReceiveData>,
    entity_id: i64,
    tag_id: i32,
    add: bool,
) {
    push_combat_notify(
        pack,
        Some(CombatCommon {
            EntityId: entity_id,
            ..Default::default()
        }),
        CombatNotifyDataMessageOneof::AnimationGameplayTagNotify(AnimationGameplayTagNotify {
            AddTagIds: tag_id,
            RemoveTagIds: add,
        }),
    );
}

fn notify_fsm_after_tag_change(
    state: &mut State,
    events: &mut EventQueue,
    entity_id: i64,
    pack: &mut Vec<CombatReceiveData>,
    now_ms: i64,
) {
    let Some(buffers) = collect_eval_buffers(state, entity_id) else {
        return;
    };
    let parent_tags = &state.assets.tables.gameplay_tag_parent;
    let Some(scene) = state.scene.as_mut() else {
        return;
    };
    let Some(handle) = scene.entity_handle(entity_id) else {
        return;
    };
    if let Some(fsm) = scene.entities[handle.index].fsm.as_mut() {
        let _ = fsm.init_runtime(now_ms);
        fsm.mark_dirty(wake_reason::TAG);
        if !fsm_lifecycle::enqueue_effects(handle, fsm, events, now_ms) {
            let ctx = EvalContext {
                now_ms,
                flags: 0,
                position_state: buffers.position_state,
                attributes: &buffers.attributes,
                tags: &buffers.tags,
                parent_tags: Some(parent_tags),
                buff_stacks: &buffers.buff_stacks,
                parts: &buffers.parts,
                instance_state: buffers.instance_state,
            };
            let _ = fsm.append_dirty_state_transitions(entity_id, pack, &ctx);
        }
        fsm.finish_tick(now_ms);
    }
    scene.sync_fsm_deadlines_for_entity(entity_id);
}

fn apply_gameplay_tag_change(
    state: &mut State,
    events: &mut EventQueue,
    pack: &mut Vec<CombatReceiveData>,
    entity_id: i64,
    tag_id: i32,
    add: bool,
) {
    if tag_id == 0 {
        return;
    }
    let changed = {
        let Some(scene) = state.scene.as_mut() else {
            return;
        };
        let Some(handle) = scene.entity_handle(entity_id) else {
            return;
        };
        let tags = scene.entities[handle.index]
            .tag
            .get_or_insert_with(TagComponent::default);
        let delta = if add { 1 } else { -1 };
        let old_present = tags.tag_count(tag_id) > 0;
        let _ = tags.add_tag(tag_id, delta);
        let new_present = tags.tag_count(tag_id) > 0;
        old_present != new_present
    };
    if !changed {
        return;
    }
    append_animation_gameplay_tag_notify(pack, entity_id, tag_id, add);
    notify_fsm_after_tag_change(state, events, entity_id, pack, now_millis());
}

pub fn handle_animation_gameplay_tag_push(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    push: AnimationGameplayTagPush,
) {
    let Some(entity_id) = common_entity_id(common) else {
        return;
    };
    apply_gameplay_tag_change(state, events, pack, entity_id, push.AddTagIds, push.RemoveTagIds);
}

pub fn handle_animation_gameplay_tag_request(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    req: AnimationGameplayTagRequest,
) -> AnimationGameplayTagResponse {
    let Some(entity_id) = common_entity_id(common) else {
        return AnimationGameplayTagResponse {
            ErrorCode: Some(ErrorCode::ErrEntityNotFound),
        };
    };
    if state
        .scene
        .as_ref()
        .and_then(|s| s.entity_handle(entity_id))
        .is_none()
    {
        return AnimationGameplayTagResponse {
            ErrorCode: Some(ErrorCode::ErrEntityNotFound),
        };
    }
    apply_gameplay_tag_change(state, events, pack, entity_id, req.AddTagIds, req.RemoveTagIds);
    AnimationGameplayTagResponse {
        ErrorCode: Some(ErrorCode::Success),
    }
}

pub fn handle_role_tag_change_push(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    push: RoleTagChangePush,
) {
    let Some(entity_id) = common_entity_id(common) else {
        return;
    };
    if push.TagId == 0 {
        return;
    }
    let notify_change = {
        let Some(scene) = state.scene.as_mut() else {
            return;
        };
        let Some(handle) = scene.entity_handle(entity_id) else {
            return;
        };
        let tags = scene.entities[handle.index]
            .tag
            .get_or_insert_with(TagComponent::default);
        let prev_present = tags.tag_count(push.TagId) > 0;
        if push.TagCount <= 0 {
            tags.remove_tag(push.TagId);
        } else {
            let current = tags.tag_count(push.TagId);
            let delta = push.TagCount - current;
            if delta != 0 {
                tags.add_tag(push.TagId, delta);
            }
        }
        let next_present = push.TagCount > 0;
        (prev_present != next_present).then_some(next_present)
    };
    if let Some(next_present) = notify_change {
        append_animation_gameplay_tag_notify(pack, entity_id, push.TagId, next_present);
    }
    notify_fsm_after_tag_change(state, events, entity_id, pack, now_millis());
}

pub fn handle_an_start_push(common: Option<&CombatCommon>, push: ANStartPush) {
    tracing::debug!(
        target: "fsm",
        "ANStartPush entity={:?} (combat caller does not forward State)",
        common_entity_id(common),
    );
    let _ = push;
}

pub fn handle_montage_play_push(common: Option<&CombatCommon>, push: MontagePlayPush) {
    tracing::debug!(
        target: "fsm",
        "MontagePlayPush entity={:?} (combat caller does not forward State)",
        common_entity_id(common),
    );
    let _ = push;
}

pub fn handle_fsm_play_montage_push(common: Option<&CombatCommon>, push: FsmPlayMontagePush) {
    tracing::debug!(
        target: "fsm",
        "FsmPlayMontagePush entity={:?} path_hash={}",
        common_entity_id(common),
        push.MontagePathHash,
    );
}

pub fn handle_fsm_play_montage_request(
    common: Option<&CombatCommon>,
    req: FsmPlayMontageRequest,
) -> FsmPlayMontageResponse {
    if common_entity_id(common).is_none()
        || (req.MontageName.is_empty() && req.MontagePathHash == 0)
    {
        return FsmPlayMontageResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
        };
    }
    FsmPlayMontageResponse {
        ErrorCode: Some(ErrorCode::Success),
    }
}

pub fn handle_change_state_request(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    req: ChangeStateRequest,
) -> ChangeStateResponse {
    let Some(target_id) = common_entity_id(common) else {
        return ChangeStateResponse {
            FsmId: req.FsmId,
            Error: derror_code(ErrorCode::ErrEntityNotFound, &[]),
            CurrentState: 0,
        };
    };
    let now_ms = now_millis();
    let Some(buffers) = collect_eval_buffers(state, target_id) else {
        return ChangeStateResponse {
            FsmId: req.FsmId,
            Error: derror_code(ErrorCode::ErrEntityNotFound, &[]),
            CurrentState: 0,
        };
    };

    let parent_tags = &state.assets.tables.gameplay_tag_parent;
    enum Outcome {
        Done {
            current_state: i32,
            error: Option<DErrorResult>,
            quest_state: Option<(i32, String)>,
        },
        MissingEntity,
    }

    let scene_opt = &mut state.scene;
    let outcome = (|| -> Outcome {
        let Some(scene) = scene_opt.as_mut() else {
            return Outcome::MissingEntity;
        };
        let Some(handle) = scene.entity_handle(target_id) else {
            return Outcome::MissingEntity;
        };
        let config_id = scene.entities[handle.index].config.config_id;
        let ctx = EvalContext {
            now_ms,
            flags: EVAL_FLAG_WAIT_CLIENT,
            position_state: buffers.position_state,
            attributes: &buffers.attributes,
            tags: &buffers.tags,
            parent_tags: Some(parent_tags),
            buff_stacks: &buffers.buff_stacks,
            parts: &buffers.parts,
            instance_state: buffers.instance_state,
        };
        let mut preserve_event_for_followup = false;
        let mut queue_followup_sync = false;
        let mut quest_state: Option<(i32, String)> = None;
        let mut enqueue_with_recheck = true;
        let current_state: i32;
        let response_error: Option<DErrorResult>;

        {
            let Some(fsm) = scene.entities[handle.index].fsm.as_mut() else {
                return Outcome::MissingEntity;
            };
            let _ = fsm.init_runtime(now_ms);
            let mut cur = fsm.current_state(req.FsmId).unwrap_or(0);
            match fsm.confirm_state_request(req.FsmId, req.FromState, req.ToState, &ctx) {
                Ok(ConfirmResult::Confirmed | ConfirmResult::Accepted) => {
                    cur = fsm.current_state(req.FsmId).unwrap_or(cur);
                    let sname = fsm
                        .graph
                        .find_node(cur)
                        .and_then(|n| n.Name.clone())
                        .unwrap_or_default();
                    quest_state = Some((config_id, sname));
                    let deferred = fsm_lifecycle::enqueue_effects(handle, fsm, events, now_ms);
                    fsm.append_blackboard_notify(target_id, pack);
                    if !deferred && fsm.any_root_dirty() {
                        queue_followup_sync = true;
                        preserve_event_for_followup = true;
                    }
                    response_error = derror_ok();
                }
                Ok(ConfirmResult::Mismatch(pending_state)) => {
                    cur = pending_state;
                    response_error = derror_code(
                        ErrorCode::ErrIentityFsmActionNotMatchState,
                        &[req.FsmId, req.ToState, pending_state],
                    );
                }
                Ok(ConfirmResult::MachineNotFound) => {
                    response_error =
                        derror_code(ErrorCode::ErrEntityFsmMachineNotExist, &[req.FsmId]);
                }
                Ok(ConfirmResult::InvalidSource) => {
                    response_error = derror_code(
                        ErrorCode::ErrEntityFsmStateIncorrect,
                        &[req.FsmId, req.FromState, cur],
                    );
                }
                Ok(ConfirmResult::InvalidTarget) => {
                    response_error = derror_code(
                        ErrorCode::ErrIentityFsmTransitToState,
                        &[req.FsmId, req.ToState],
                    );
                }
                Ok(ConfirmResult::NoPending) => {
                    if let Ok(Some(notify)) = fsm.check_and_confirm(target_id, req.FsmId, &ctx) {
                        fsm.append_blackboard_notify(target_id, pack);
                        let transition_copy = extract_change_state_notify(&notify).cloned();
                        pack.push(notify);
                        enqueue_with_recheck = false;
                        if !fsm_lifecycle::enqueue_effects_without_recheck(
                            handle, fsm, events, now_ms,
                        ) {
                            fsm.clear_dirty_reason(wake_reason::INITIAL);
                            if fsm.any_root_dirty() {
                                queue_followup_sync = true;
                                preserve_event_for_followup = true;
                            }
                        }
                        if let Some(t) = transition_copy {
                            cur = t.ToState;
                            let sname = fsm
                                .graph
                                .find_node(cur)
                                .and_then(|n| n.Name.clone())
                                .unwrap_or_default();
                            quest_state = Some((config_id, sname));
                            response_error = derror_code(
                                ErrorCode::ErrIentityFsmConfirmNotWait,
                                &[t.FsmId, t.FromState, t.ToState],
                            );
                        } else {
                            cur = fsm.current_state(req.FsmId).unwrap_or(cur);
                            let sname = fsm
                                .graph
                                .find_node(cur)
                                .and_then(|n| n.Name.clone())
                                .unwrap_or_default();
                            quest_state = Some((config_id, sname));
                            response_error = derror_code(
                                ErrorCode::ErrIentityFsmConfirmNotWait,
                                &[req.FsmId, req.FromState, cur],
                            );
                        }
                    } else {
                        cur = fsm.current_state(req.FsmId).unwrap_or(cur);
                        response_error = derror_code(
                            ErrorCode::ErrIentityFsmConfirmNotExist,
                            &[req.FsmId, cur],
                        );
                    }
                }
                Err(_) => {
                    response_error = derror_code(ErrorCode::UnKnownError, &[]);
                }
            }
            if enqueue_with_recheck {
                let _ = fsm_lifecycle::enqueue_effects(handle, fsm, events, now_ms);
            }
            if preserve_event_for_followup {
                fsm.finish_tick_preserving_event(now_ms);
            } else {
                fsm.finish_tick(now_ms);
            }
            current_state = cur;
        }

        if queue_followup_sync {
            scene.queue_fsm_sync(target_id);
        }
        scene.sync_fsm_deadlines_for_entity(target_id);
        Outcome::Done {
            current_state,
            error: response_error,
            quest_state,
        }
    })();

    match outcome {
        Outcome::MissingEntity => ChangeStateResponse {
            FsmId: req.FsmId,
            Error: derror_code(ErrorCode::ErrEntityNotFound, &[]),
            CurrentState: 0,
        },
        Outcome::Done {
            current_state,
            error,
            quest_state,
        } => {
            if let Some((config_id, state_name)) = quest_state {
                notify_quest_fsm_state_changed(state, events, config_id, &state_name);
            }
            ChangeStateResponse {
                FsmId: req.FsmId,
                Error: error,
                CurrentState: current_state,
            }
        }
    }
}

fn append_followup_transition(
    fsm: &mut FsmComponent,
    entity_id: i64,
    ctx: &EvalContext<'_>,
    fsm_id: i32,
    pack: &mut Vec<CombatReceiveData>,
) {
    if let Ok(Some(notify)) = fsm.check_transitions(entity_id, fsm_id, ctx) {
        fsm.append_blackboard_notify(entity_id, pack);
        pack.push(notify);
    }
    let _ = fsm.append_dirty_state_transitions(entity_id, pack, ctx);
}

fn apply_change_state_confirm(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    fsm_id: i32,
    confirm_state: i32,
) -> (i32, Option<DErrorResult>) {
    let Some(target_id) = common_entity_id(common) else {
        return (0, derror_code(ErrorCode::ErrEntityNotFound, &[]));
    };
    let now_ms = now_millis();
    let Some(buffers) = collect_eval_buffers(state, target_id) else {
        return (0, derror_code(ErrorCode::ErrEntityNotFound, &[]));
    };
    let parent_tags = &state.assets.tables.gameplay_tag_parent;

    let (response_state, response_error, quest_state) = {
        let Some(scene) = state.scene.as_mut() else {
            return (0, derror_code(ErrorCode::ErrEntityNotFound, &[]));
        };
        let Some(handle) = scene.entity_handle(target_id) else {
            return (0, derror_code(ErrorCode::ErrEntityNotFound, &[]));
        };
        let config_id = scene.entities[handle.index].config.config_id;
        let ctx = EvalContext {
            now_ms,
            flags: 0,
            position_state: buffers.position_state,
            attributes: &buffers.attributes,
            tags: &buffers.tags,
            parent_tags: Some(parent_tags),
            buff_stacks: &buffers.buff_stacks,
            parts: &buffers.parts,
            instance_state: buffers.instance_state,
        };
        let (resp_state, resp_err, qstate) = {
            let Some(fsm) = scene.entities[handle.index].fsm.as_mut() else {
                return (0, derror_code(ErrorCode::ErrEntityNotFound, &[]));
            };
            let _ = fsm.init_runtime(now_ms);
            let mut resp_state = fsm.current_state(fsm_id).unwrap_or(0);
            let mut qstate = None;
            let resp_err = match fsm.confirm_pending(fsm_id, confirm_state, now_ms) {
                Ok(ConfirmResult::Confirmed) => {
                    resp_state = fsm.current_state(fsm_id).unwrap_or(resp_state);
                    let sname = fsm
                        .graph
                        .find_node(resp_state)
                        .and_then(|n| n.Name.clone())
                        .unwrap_or_default();
                    qstate = Some((config_id, sname));
                    if !fsm_lifecycle::enqueue_effects(handle, fsm, events, now_ms) {
                        fsm.append_blackboard_notify(target_id, pack);
                        append_followup_transition(fsm, target_id, &ctx, fsm_id, pack);
                    }
                    derror_ok()
                }
                Ok(ConfirmResult::Mismatch(pending_state)) => {
                    resp_state = pending_state;
                    derror_code(
                        ErrorCode::ErrIentityFsmActionNotMatchState,
                        &[fsm_id, confirm_state, pending_state],
                    )
                }
                Ok(ConfirmResult::MachineNotFound) => {
                    derror_code(ErrorCode::ErrEntityFsmMachineNotExist, &[fsm_id])
                }
                Ok(ConfirmResult::InvalidTarget) => {
                    derror_code(ErrorCode::ErrIentityFsmTransitToState, &[fsm_id, confirm_state])
                }
                Ok(ConfirmResult::NoPending) => {
                    derror_code(ErrorCode::ErrIentityFsmConfirmNotExist, &[fsm_id, resp_state])
                }
                Ok(ConfirmResult::Accepted) => {
                    resp_state = fsm.current_state(fsm_id).unwrap_or(resp_state);
                    derror_ok()
                }
                Ok(ConfirmResult::InvalidSource) => {
                    derror_code(ErrorCode::ErrEntityFsmStateIncorrect, &[fsm_id, resp_state])
                }
                Err(_) => derror_code(ErrorCode::UnKnownError, &[]),
            };
            let _ = fsm_lifecycle::enqueue_effects(handle, fsm, events, now_ms);
            fsm.finish_tick(now_ms);
            (resp_state, resp_err, qstate)
        };
        scene.sync_fsm_deadlines_for_entity(target_id);
        (resp_state, resp_err, qstate)
    };

    if let Some((config_id, state_name)) = quest_state {
        notify_quest_fsm_state_changed(state, events, config_id, &state_name);
    }
    (response_state, response_error)
}

pub fn handle_change_state_confirm_push(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    push: ChangeStateConfirmPush,
) {
    let _ = apply_change_state_confirm(state, events, common, pack, push.FsmId, push.State);
}

pub fn handle_change_state_confirm_request(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    req: ChangeStateConfirmRequest,
) -> ChangeStateConfirmResponse {
    let (resp_state, err) =
        apply_change_state_confirm(state, events, common, pack, req.FsmId, req.State);
    ChangeStateConfirmResponse {
        FsmId: req.FsmId,
        State: resp_state,
        Error: err,
    }
}

fn client_pass_error(result: ClientPassResult, key: &ConditionKey) -> Option<DErrorResult> {
    match result {
        ClientPassResult::Updated => derror_ok(),
        ClientPassResult::MachineNotFound => {
            derror_code(ErrorCode::ErrEntityFsmMachineNotExist, &[key.fsm_id])
        }
        ClientPassResult::InvalidSource => derror_code(
            ErrorCode::ErrEntityFsmStateIncorrect,
            &[key.fsm_id, key.from],
        ),
        ClientPassResult::InvalidTarget => {
            derror_code(ErrorCode::ErrIentityFsmTransitToState, &[key.fsm_id, key.to])
        }
        ClientPassResult::InactiveSource
        | ClientPassResult::TransitionNotFound
        | ClientPassResult::ConditionNotFound
        | ClientPassResult::ConditionNotClient
        | ClientPassResult::ConditionContextInvalid => derror_code(
            ErrorCode::ErrIentityFsmActionNotMatchState,
            &[key.fsm_id, key.from, key.to, key.index],
        ),
    }
}

fn apply_fsm_condition_pass(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    key: ConditionKey,
    value: bool,
) -> Option<DErrorResult> {
    let Some(target_id) = common_entity_id(common) else {
        return derror_code(ErrorCode::ErrEntityNotFound, &[]);
    };
    let now_ms = now_millis();
    let Some(buffers) = collect_eval_buffers(state, target_id) else {
        return derror_code(ErrorCode::ErrEntityNotFound, &[]);
    };
    let parent_tags = &state.assets.tables.gameplay_tag_parent;

    let Some(scene) = state.scene.as_mut() else {
        return derror_code(ErrorCode::ErrEntityNotFound, &[]);
    };
    let Some(handle) = scene.entity_handle(target_id) else {
        return derror_code(ErrorCode::ErrEntityNotFound, &[]);
    };
    let resp_err = {
        let Some(fsm) = scene.entities[handle.index].fsm.as_mut() else {
            return derror_code(ErrorCode::ErrEntityNotFound, &[]);
        };
        let _ = fsm.init_runtime(now_ms);
        let pass_res = fsm.record_client_pass(key, value);
        let err = client_pass_error(pass_res, &key);
        if matches!(pass_res, ClientPassResult::Updated) {
            fsm.mark_root_dirty(key.fsm_id, wake_reason::CLIENT_PASS);
        }
        let deferred = fsm_lifecycle::enqueue_effects(handle, fsm, events, now_ms);
        if matches!(pass_res, ClientPassResult::Updated) && !deferred {
            let ctx = EvalContext {
                now_ms,
                flags: EVAL_FLAG_WAIT_CLIENT,
                position_state: buffers.position_state,
                attributes: &buffers.attributes,
                tags: &buffers.tags,
                parent_tags: Some(parent_tags),
                buff_stacks: &buffers.buff_stacks,
                parts: &buffers.parts,
                instance_state: buffers.instance_state,
            };
            if let Ok(Some(notify)) = fsm.check_transitions(target_id, key.fsm_id, &ctx) {
                fsm.append_blackboard_notify(target_id, pack);
                pack.push(notify);
            }
        }
        fsm.finish_tick(now_ms);
        err
    };
    scene.sync_fsm_deadlines_for_entity(target_id);
    resp_err
}

pub fn handle_fsm_condition_pass_push(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    push: FsmConditionPassPush,
) {
    let _ = apply_fsm_condition_pass(
        state,
        events,
        common,
        pack,
        ConditionKey {
            fsm_id: push.FsmId,
            from: push.FromState,
            to: push.ToState,
            index: push.ConditionIndex,
        },
        push.Value,
    );
}

pub fn handle_fsm_condition_pass_request(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    req: FsmConditionPassRequest,
) -> FsmConditionPassResponse {
    let err = apply_fsm_condition_pass(
        state,
        events,
        common,
        pack,
        ConditionKey {
            fsm_id: req.FsmId,
            from: req.FromState,
            to: req.ToState,
            index: req.ConditionIndex,
        },
        req.Value,
    );
    FsmConditionPassResponse {
        FsmId: req.FsmId,
        Error: err,
    }
}

pub fn handle_fsm_state_behavior_request(
    state: &mut State,
    common: Option<&CombatCommon>,
    req: FsmStateBehaviorRequest,
) -> FsmStateBehaviorResponse {
    let Some(target_id) = common_entity_id(common) else {
        return FsmStateBehaviorResponse {
            FsmId: req.FsmId,
            State: 0,
            ErrorCode: Some(ErrorCode::ErrEntityNotFound),
        };
    };
    let now_ms = now_millis();
    let Some(scene) = state.scene.as_mut() else {
        return FsmStateBehaviorResponse {
            FsmId: req.FsmId,
            State: 0,
            ErrorCode: Some(ErrorCode::ErrEntityNotFound),
        };
    };
    let Some(handle) = scene.entity_handle(target_id) else {
        return FsmStateBehaviorResponse {
            FsmId: req.FsmId,
            State: 0,
            ErrorCode: Some(ErrorCode::ErrEntityNotFound),
        };
    };
    let Some(fsm) = scene.entities[handle.index].fsm.as_mut() else {
        return FsmStateBehaviorResponse {
            FsmId: req.FsmId,
            State: 0,
            ErrorCode: Some(ErrorCode::ErrEntityFsmMachineNotExist),
        };
    };

    let _ = fsm.init_runtime(now_ms);
    let current_state = fsm.current_state(req.FsmId).unwrap_or(0);
    let Some(behavior_type) = req.Type else {
        return FsmStateBehaviorResponse {
            FsmId: req.FsmId,
            State: current_state,
            ErrorCode: Some(ErrorCode::ErrNotStateMachineBehavior),
        };
    };
    let validation = fsm.validate_behavior(
        req.FsmId,
        req.State,
        req.Index,
        behavior_type.to_i32(),
    );
    let err_code = match validation {
        BehaviorValidation::Ok => ErrorCode::Success,
        BehaviorValidation::MachineNotFound => ErrorCode::ErrEntityFsmMachineNotExist,
        BehaviorValidation::InvalidState
        | BehaviorValidation::InactiveState
        | BehaviorValidation::NodeNotFound
        | BehaviorValidation::MetadataNotFound => ErrorCode::ErrEntityFsmStateIncorrect,
        BehaviorValidation::Mismatch => ErrorCode::ErrNotStateMachineBehavior,
    };

    FsmStateBehaviorResponse {
        FsmId: req.FsmId,
        State: if matches!(validation, BehaviorValidation::Ok) {
            req.State
        } else {
            current_state
        },
        ErrorCode: Some(err_code),
    }
}

fn apply_ai_hate(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    hate_list: &[AiHateEntity],
    ai_blackboards: Option<&[sonoro_proto::pb::BlackboardParam]>,
) {
    let Some(target_id) = common_entity_id(common) else {
        return;
    };
    let now_ms = now_millis();
    let Some(buffers) = collect_eval_buffers(state, target_id) else {
        return;
    };
    let parent_tags = &state.assets.tables.gameplay_tag_parent;

    let (hate_changed, has_target) = {
        let Some(scene) = state.scene.as_mut() else {
            return;
        };
        if !scene.net_id_map.contains_key(&target_id) {
            return;
        }
        scene.update_battle_entity_from_hate(target_id, hate_list);
        let _ = scene.append_battle_state_notify(pack);
        let has_target = hate_list.iter().any(|entry| entry.HatredValue >= 1);
        let hate_changed = scene.update_fsm_hate_target(target_id, has_target);

        if let Some(handle) = scene.entity_handle(target_id)
            && let Some(fsm) = scene.entities[handle.index].fsm.as_mut()
        {
            if let Some(bb_params) = ai_blackboards {
                let mut bb_changed = false;
                for param in bb_params {
                    let Ok(key) = String::from_utf8(param.Key.clone()) else {
                        continue;
                    };
                    let val = match &param.Value {
                        Some(BlackboardParamValueOneof::IntValue(v)) => *v,
                        Some(BlackboardParamValueOneof::LongValue(v)) => *v as i32,
                        Some(BlackboardParamValueOneof::BooleanValue(v)) => i32::from(*v),
                        Some(BlackboardParamValueOneof::FloatValue(v)) => *v as i32,
                        _ => continue,
                    };
                    if let Some(existing) =
                        fsm.custom_blackboards.iter_mut().find(|(k, _)| k == &key)
                    {
                        if existing.1 != val {
                            existing.1 = val;
                            bb_changed = true;
                        }
                    } else {
                        fsm.custom_blackboards.push((key, val));
                        bb_changed = true;
                    }
                }
                if bb_changed {
                    fsm.mark_dirty(wake_reason::BLACKBOARD);
                }
            }
            let _ = fsm.init_runtime(now_ms);
            if !fsm_lifecycle::enqueue_effects(handle, fsm, events, now_ms) {
                let ctx = EvalContext {
                    now_ms,
                    flags: 0,
                    position_state: buffers.position_state,
                    attributes: &buffers.attributes,
                    tags: &buffers.tags,
                    parent_tags: Some(parent_tags),
                    buff_stacks: &buffers.buff_stacks,
                    parts: &buffers.parts,
                    instance_state: buffers.instance_state,
                };
                let _ = fsm.append_dirty_state_transitions(target_id, pack, &ctx);
            }
            fsm.finish_tick(now_ms);
            scene.sync_fsm_deadlines_for_entity(target_id);
        }
        (hate_changed, has_target)
    };

    if hate_changed && has_target {
        let mut qctx = state.quest_context(events);
        if let Err(err) = quest_engine::on_condition_state_changed(&mut qctx) {
            tracing::warn!(target: "fsm", "onConditionStateChanged failed: {err}");
        }
    }
}

pub fn handle_ai_hate_push(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    push: AiHatePush,
) {
    apply_ai_hate(state, events, common, pack, &push.HateList, None);
}

pub fn handle_ai_hate_request(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    req: AiHateRequest,
) -> AiHateResponse {
    apply_ai_hate(state, events, common, pack, &req.HateList, None);
    AiHateResponse {
        ErrorCode: Some(ErrorCode::Success),
    }
}

fn apply_ai_information(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    ai_info: Option<&AiInformation>,
) {
    let Some(info) = ai_info else {
        return;
    };
    apply_ai_hate(
        state,
        events,
        common,
        pack,
        &info.HateList,
        Some(&info.AiBlackboards),
    );
}

pub fn handle_ai_information_push(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    push: AiInformationPush,
) {
    apply_ai_information(state, events, common, pack, push.AiInfo.as_ref());
}

pub fn handle_ai_information_request(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    req: AiInformationRequest,
) -> AiInformationResponse {
    apply_ai_information(state, events, common, pack, req.AiInfo.as_ref());
    AiInformationResponse {
        ErrorCode: Some(ErrorCode::Success),
    }
}

fn apply_battle_state_change(
    state: &mut State,
    common: Option<&CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    entity_id: i64,
    in_battle: bool,
) {
    let target_id = resolve_explicit_or_common(entity_id, common);
    if target_id == 0 {
        return;
    }
    if let Some(scene) = state.scene.as_mut() {
        scene.set_battle_entity_active(target_id, in_battle);
        push_combat_notify(
            pack,
            Some(CombatCommon {
                EntityId: target_id,
                ..Default::default()
            }),
            CombatNotifyDataMessageOneof::BattleStateChangeNotify(BattleStateChangeNotify {
                EntityId: target_id,
                InBattle: in_battle,
            }),
        );
        let _ = scene.append_battle_state_notify(pack);
    }
}

pub fn handle_battle_state_change_push(
    state: &mut State,
    _events: &mut EventQueue,
    common: Option<&CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    push: BattleStateChangePush,
) {
    apply_battle_state_change(state, common, pack, push.EntityId, push.InBattle);
}

pub fn handle_battle_state_change_request(
    state: &mut State,
    _events: &mut EventQueue,
    common: Option<&CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    req: BattleStateChangeRequest,
) -> BattleStateChangeResponse {
    apply_battle_state_change(state, common, pack, req.EntityId, req.InBattle);
    BattleStateChangeResponse {
        ErrorCode: Some(ErrorCode::Success),
    }
}

fn update_logic_state(
    state: &mut State,
    entity_id: i64,
    common: Option<&CombatCommon>,
    init_data: Option<&LogicStateComponentPb>,
) -> (i64, bool) {
    let target_id = resolve_explicit_or_common(entity_id, common);
    if target_id == 0 {
        return (0, false);
    }
    let Some(data) = init_data else {
        return (target_id, false);
    };
    let Some(scene) = state.scene.as_mut() else {
        return (target_id, false);
    };
    let Some(handle) = scene.entity_handle(target_id) else {
        return (target_id, false);
    };
    let changed = {
        let storage = &mut scene.entities[handle.index];
        let Some(logic_state) = storage.logic_state.as_mut() else {
            return (target_id, false);
        };
        let changed = logic_state.position_state != data.PositionState
            || logic_state.move_state != data.MoveState
            || logic_state.direction_state != data.DirectionState
            || logic_state.position_sub_state != data.PositionSubState;
        logic_state.position_state = data.PositionState;
        logic_state.move_state = data.MoveState;
        logic_state.direction_state = data.DirectionState;
        logic_state.position_sub_state = data.PositionSubState;
        changed
    };
    if changed {
        scene.mark_fsm_dirty(target_id, wake_reason::POSITION);
    }
    (target_id, changed)
}

pub fn handle_logic_state_init_push(
    state: &mut State,
    _events: &mut EventQueue,
    common: Option<&CombatCommon>,
    _pack: &mut Vec<CombatReceiveData>,
    push: LogicStateInitPush,
) {
    let _ = update_logic_state(
        state,
        push.EntityId,
        push.CombatCommon.as_ref().or(common),
        push.InitData.as_ref(),
    );
}

pub fn handle_logic_state_init_request(
    state: &mut State,
    _events: &mut EventQueue,
    common: Option<&CombatCommon>,
    _pack: &mut Vec<CombatReceiveData>,
    req: LogicStateInitRequest,
) -> LogicStateInitResponse {
    let _ = update_logic_state(
        state,
        req.EntityId,
        req.CombatCommon.as_ref().or(common),
        req.InitData.as_ref(),
    );
    LogicStateInitResponse {
        ErrorCode: Some(ErrorCode::Success),
    }
}

fn handle_switch_logic_state_common(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    states: Option<&LogicStateComponentPb>,
) {
    let (target_id, changed) = update_logic_state(state, 0, common, states);
    let is_current_role = changed
        && state
            .scene
            .as_ref()
            .is_some_and(|s| target_id != 0 && target_id == s.current_role_entity_id());
    if is_current_role {
        if let Some(scene) = state.scene.as_mut() {
            scene.has_player_motion = true;
        }
        let mut qctx = state.quest_context(events);
        if let Err(err) = quest_engine::on_condition_state_changed(&mut qctx) {
            tracing::warn!(target: "fsm", "onConditionStateChanged failed: {err}");
        }
    }
}

pub fn handle_switch_logic_state_push(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    _pack: &mut Vec<CombatReceiveData>,
    push: SwitchLogicStatePush,
) {
    handle_switch_logic_state_common(state, events, common, push.States.as_ref());
}

pub fn handle_switch_logic_state_request(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    _pack: &mut Vec<CombatReceiveData>,
    req: SwitchLogicStateRequest,
) -> SwitchLogicStateResponse {
    handle_switch_logic_state_common(state, events, common, req.States.as_ref());
    SwitchLogicStateResponse {
        ErrorCode: Some(ErrorCode::Success),
    }
}
