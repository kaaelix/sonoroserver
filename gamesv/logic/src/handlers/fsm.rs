use sonoro_common::fs::FileSystem;
use sonoro_gamesv_data::Assets;
use sonoro_proto::enums::{EAttributeType, LivingStatus};
use sonoro_proto::pb::{
    CombatCommon, CombatNotifyData, CombatNotifyDataMessageOneof, CombatReceiveData,
    CombatReceiveDataMessageOneof, CombatReceivePackNotify, EntityLivingStatusNotify,
    EntityRemoveInfo, EntityRemoveNotify, PartUpdateNotify,
};
use sonoro_proto::wire::PbEnum;

use crate::component::entity::small::EntityType;
use crate::component::entity::{
    AttributeComponent, FightBuffComponent, FsmComponent, PartComponent, TagComponent,
};
use crate::component::player::PlayerRoleComponent;
use crate::event_queue::EventQueue;
use crate::events::{EntityHandle, Event, FsmServerActionKind};
use crate::fsm::types::{EvalContext, wake_reason};
use crate::fsm_lifecycle;
use crate::handlers::buff as buff_handlers;
use crate::helpers::attributes::{AttributeChangeType, AttributeValueType};
use crate::helpers::level_play::{self, LevelPlayContext};
use crate::helpers::quest_engine;
use crate::helpers::quest_types::QuestContext;
use crate::helpers::{attributes, buff as buff_helper, role as role_helper};
use crate::scene::Scene;
use crate::schedulers::BuffTimerScheduler;
use crate::schedulers::fsm_wake_scheduler::WakeKind;
use crate::sink::PushSink;

#[allow(clippy::too_many_arguments)]
pub fn handle_fsm_wake<S: PushSink + ?Sized>(
    now_ms: i64,
    qctx: &mut QuestContext<'_, S>,
    _assets: &Assets,
    role_comp: &mut PlayerRoleComponent,
    _fs: &FileSystem,
    buff_timers: &mut BuffTimerScheduler,
) {
    let mut sub_qctx = QuestContext {
        assets: qctx.assets,
        fs: qctx.fs,
        sink: qctx.sink,
        scene: qctx.scene.as_deref_mut(),
        scene_comp: qctx.scene_comp.as_deref_mut(),
        events: qctx.events,
        basic_comp: qctx.basic_comp,
        inventory_comp: qctx.inventory_comp,
        weapon_comp: qctx.weapon_comp,
        echo_comp: qctx.echo_comp,
        map_comp: qctx.map_comp,
        quest_comp: qctx.quest_comp,
        role_comp: Some(qctx.role_comp.as_deref_mut().unwrap_or(role_comp)),
        guide_comp: qctx.guide_comp.as_deref_mut(),
        mail_comp: qctx.mail_comp.as_deref_mut(),
        progress_comp: qctx.progress_comp.as_deref_mut(),
        activity_notify: qctx.activity_notify,
    };
    handle_fsm_wake_qctx(now_ms, &mut sub_qctx, buff_timers);
}

pub fn handle_fsm_wake_qctx<S: PushSink + ?Sized>(
    now_ms: i64,
    qctx: &mut QuestContext<'_, S>,
    buff_timers: &mut BuffTimerScheduler,
) {
    let assets = qctx.assets;
    let fs = qctx.fs;
    let mut data: Vec<CombatReceiveData> = Vec::new();

    loop {
        let due = {
            let Some(scene) = qctx.scene.as_deref_mut() else {
                return;
            };
            scene.fsm_wakes.pop_due(now_ms)
        };
        let Some(due) = due else {
            break;
        };

        match due.kind {
            WakeKind::RootTimer => {
                if let Some(scene) = qctx.scene.as_deref_mut() {
                    scene.mark_fsm_root_dirty(due.entity_id, due.fsm_id, wake_reason::TIMER);
                }
            }
            WakeKind::PendingTimeout | WakeKind::Paralysis => {
                if let Some(scene) = qctx.scene.as_deref_mut() {
                    scene.queue_fsm_sync(due.entity_id);
                }
            }
            WakeKind::DelayedSuicide => {
                let consumed = {
                    let Some(scene) = qctx.scene.as_deref_mut() else {
                        continue;
                    };
                    let Some(handle) = scene.entity_handle(due.entity_id) else {
                        continue;
                    };
                    let Some(fsm) = scene.entities[handle.index].fsm.as_mut() else {
                        continue;
                    };
                    if fsm.init_runtime(now_ms).is_err() {
                        continue;
                    }
                    fsm.consume_delayed_suicide(due.fsm_id, due.due_ms)
                };
                if !consumed {
                    continue;
                }

                let mut death_data = Vec::new();
                let cfg = {
                    let scene = qctx.scene.as_deref_mut().expect("checked above");
                    let handle = scene.entity_handle(due.entity_id).expect("checked above");
                    let cfg = scene.entities[handle.index].config;
                    if let Some(attr) = scene.entities[handle.index].attribute.as_mut() {
                        let _ = attributes::set_life_unconditional(attr, 0);
                    }
                    let has_buffs = scene.entities[handle.index].buffs.is_some();
                    if has_buffs {
                        let removed_handles = buff_helper::handle_entity_death(
                            &mut death_data,
                            scene,
                            assets,
                            due.entity_id,
                        );
                        for hid in removed_handles {
                            buff_timers.cancel_handle(due.entity_id, hid);
                        }
                    } else if !scene.entities[handle.index].dead_notified {
                        scene.entities[handle.index].dead_notified = true;
                        death_data.push(CombatReceiveData {
                            Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
                                CombatNotifyData {
                                    CombatCommon: Some(CombatCommon {
                                        EntityId: due.entity_id,
                                        ..Default::default()
                                    }),
                                    Message: Some(
                                        CombatNotifyDataMessageOneof::EntityLivingStatusNotify(
                                            EntityLivingStatusNotify {
                                                Id: due.entity_id,
                                                LivingStatus: Some(LivingStatus::Dead),
                                                ..Default::default()
                                            },
                                        ),
                                    ),
                                },
                            )),
                        });
                        scene.set_battle_entity_active(due.entity_id, false);
                    }
                    if cfg.entity_type == EntityType::Player
                        && cfg.config_type == crate::component::entity::small::ConfigType::Character
                        && let Some(role_comp) = qctx.role_comp.as_deref_mut()
                        && let Some(ri) = role_comp.role_map.get_mut(&cfg.config_id)
                    {
                        ri.current_hp = Some(0);
                        if let Err(err) = role_helper::save_role_info(fs, role_comp, cfg.config_id) {
                            tracing::warn!(target: "fsm", "save_role_info({}) failed: {err}", cfg.config_id);
                        }
                    }
                    let _ = scene.append_battle_state_notify(&mut death_data);
                    scene.sync_fsm_deadlines_for_entity(due.entity_id);
                    cfg
                };

                if !death_data.is_empty() {
                    qctx.sink.push(
                        CombatReceivePackNotify::MSG_ID,
                        &CombatReceivePackNotify { Data: death_data },
                    );
                }

                if let Err(err) = quest_engine::on_entity_killed(qctx, due.entity_id, None) {
                    tracing::warn!(target: "quest_engine", "on_entity_killed failed: {err}");
                }
                if let Some(mut lp_ctx) = LevelPlayContext::from_quest_ctx(qctx)
                    && let Err(err) = level_play::on_entity_killed_with_conditions(
                        &mut lp_ctx,
                        due.entity_id,
                        None,
                    )
                {
                    tracing::warn!(target: "level_play", "on_entity_killed_with_conditions failed: {err}");
                }
                if cfg.entity_type == EntityType::Monster
                    && let Some(scene) = qctx.scene.as_deref_mut()
                {
                    scene.queue_killed_monster_removal_at(due.entity_id, now_ms + 1800);
                }
            }
            WakeKind::DelayedDestroy => {
                let removed_is_monster = {
                    let Some(scene) = qctx.scene.as_deref_mut() else {
                        continue;
                    };
                    let Some(handle) = scene.entity_handle(due.entity_id) else {
                        continue;
                    };
                    let is_monster =
                        scene.entities[handle.index].config.entity_type == EntityType::Monster;
                    let Some(fsm) = scene.entities[handle.index].fsm.as_mut() else {
                        continue;
                    };
                    if fsm.init_runtime(now_ms).is_err()
                        || !fsm.delayed_destroy_is_due(due.fsm_id, due.due_ms)
                    {
                        continue;
                    }
                    if scene.remove(fs, due.entity_id).is_ok() {
                        Some(is_monster)
                    } else {
                        None
                    }
                };
                if let Some(is_monster) = removed_is_monster {
                    qctx.sink.push(
                        EntityRemoveNotify::MSG_ID,
                        &EntityRemoveNotify {
                            IsRemove: true,
                            RemoveInfos: vec![EntityRemoveInfo {
                                EntityId: due.entity_id,
                                Type: delayed_destroy_remove_type(is_monster),
                            }],
                        },
                    );
                }
            }
        }
    }

    loop {
        let entity_id = {
            let Some(scene) = qctx.scene.as_deref_mut() else {
                break;
            };
            scene.fsm_wakes.pop_dirty()
        };
        let Some(entity_id) = entity_id else {
            break;
        };

        let Some(scene) = qctx.scene.as_deref_mut() else {
            break;
        };
        let Some(handle) = scene.entity_handle(entity_id) else {
            continue;
        };
        let Some(fsm) = scene.entities[handle.index].fsm.as_mut() else {
            continue;
        };
        if fsm.init_runtime(now_ms).is_err() {
            continue;
        }

        let is_dead_monster = scene.entities[handle.index]
            .config
            .entity_type
            == EntityType::Monster
            && scene.entities[handle.index]
                .attribute
                .as_ref()
                .is_some_and(|attr| attributes::get_attr_value(attr, EAttributeType::Life) <= 0);
        if is_dead_monster {
            if let Some(fsm) = scene.entities[handle.index].fsm.as_mut() {
                fsm.finish_tick(now_ms);
            }
            continue;
        }

        let need_paralysis = scene.entities[handle.index]
            .fsm
            .as_ref()
            .and_then(|f| f.paralysis_active.then_some(f.paralysis_next_ms).flatten())
            .is_some_and(|next_ms| next_ms <= now_ms);
        if need_paralysis {
            update_paralysis(&mut scene.entities[handle.index], now_ms, qctx.sink);
        }

        let lifecycle_deferred = {
            let storage = &mut scene.entities[handle.index];
            let Some(fsm) = storage.fsm.as_mut() else {
                continue;
            };
            if fsm.recover_expired_pending(now_ms).unwrap_or(false) {
                fsm.append_reset_notify(entity_id, now_ms, &mut data);
            }
            let deferred = fsm_lifecycle::enqueue_effects(handle, fsm, qctx.events, now_ms);
            if !deferred {
                let eval_buffers = FsmEvalBuffers::from_components(
                    storage.attribute.as_ref(),
                    storage.buffs.as_ref(),
                    storage.tag.as_ref(),
                    storage.part.as_ref(),
                    storage.logic_state.as_ref(),
                );
                let eval_ctx = eval_buffers.as_context(
                    Some(&assets.tables.gameplay_tag_parent),
                    fsm.instance_state_tag,
                    now_ms,
                );
                let _ = fsm.append_dirty_state_transitions(entity_id, &mut data, &eval_ctx);
                fsm.finish_tick(now_ms);
                if !fsm.lifecycle_effects().is_empty() {
                    let _ = fsm_lifecycle::enqueue_effects_without_recheck(
                        handle,
                        fsm,
                        qctx.events,
                        now_ms,
                    );
                }
            }
            deferred
        };
        let _ = lifecycle_deferred;
        scene.sync_fsm_deadlines_for_entity(entity_id);
    }

    if !data.is_empty() {
        qctx.sink.push(
            CombatReceivePackNotify::MSG_ID,
            &CombatReceivePackNotify { Data: data },
        );
    }
}

#[allow(clippy::too_many_arguments)]
pub fn handle_fsm_server_action<S: PushSink + ?Sized>(
    entity: EntityHandle,
    kind: &FsmServerActionKind,
    scene: &mut Scene,
    assets: &Assets,
    sink: &mut S,
    events: &mut EventQueue,
    buff_timers: &mut BuffTimerScheduler,
) {
    let Some(handle) = scene.entity_handle(entity.net_id) else {
        return;
    };
    if scene.entities[handle.index].fsm.is_none() {
        return;
    }

    match kind {
        FsmServerActionKind::EnterFight => {
            scene.set_battle_entity_active(entity.net_id, true);
            let mut data = Vec::new();
            if scene.append_battle_state_notify(&mut data) {
                sink.push(
                    CombatReceivePackNotify::MSG_ID,
                    &CombatReceivePackNotify { Data: data },
                );
            }
        }
        FsmServerActionKind::CueParalysis => {
            let now_ms = crate::scene::now_millis();
            if let Some(fsm) = scene.entities[handle.index].fsm.as_mut() {
                fsm.activate_paralysis(now_ms);
            }
            scene.queue_fsm_sync(entity.net_id);
        }
        FsmServerActionKind::ResetStatus => {
            if let Some(attr) = scene.entities[handle.index].attribute.as_mut() {
                reset_status_attributes(entity.net_id, attr, sink);
            }
            let handles: Vec<i32> = scene.entities[handle.index]
                .buffs
                .as_ref()
                .map(|b| b.buffs.iter().map(|x| x.HandleId).collect())
                .unwrap_or_default();
            if !handles.is_empty() {
                buff_handlers::remove_buff_handles_with_sync(
                    entity,
                    &handles,
                    Some(scene),
                    Some(assets),
                    sink,
                    events,
                    buff_timers,
                );
            }
            scene.set_battle_entity_active(entity.net_id, false);
            let mut battle_data = Vec::new();
            if scene.append_battle_state_notify(&mut battle_data) {
                sink.push(
                    CombatReceivePackNotify::MSG_ID,
                    &CombatReceivePackNotify { Data: battle_data },
                );
            }
            scene.mark_fsm_dirty(entity.net_id, wake_reason::ATTRIBUTE | wake_reason::BUFF);
        }
        FsmServerActionKind::SetRageFull => {
            if let Some(attr) = scene.entities[handle.index].attribute.as_mut() {
                let changed = set_rage_full_attributes(attr);
                push_attribute_changes(entity.net_id, attr, &changed, sink);
                if !changed.is_empty() {
                    scene.mark_fsm_dirty(entity.net_id, wake_reason::ATTRIBUTE);
                }
            }
        }
        FsmServerActionKind::InstanceState(tag_id) => {
            let storage = &mut scene.entities[handle.index];
            if let Some(fsm) = storage.fsm.as_mut() {
                apply_instance_state(entity, fsm, storage.tag.as_mut(), *tag_id, events);
            }
            scene.mark_fsm_dirty(entity.net_id, wake_reason::TAG);
        }
        FsmServerActionKind::ActivatePart { name, activate } => {
            let mut changed = Vec::new();
            {
                let storage = &mut scene.entities[handle.index];
                let tag_opt = storage.tag.as_mut();
                if let Some(part) = storage.part.as_mut() {
                    if let Some(idx) = part.activate(name, *activate, tag_opt) {
                        changed.push(idx);
                    }
                    push_part_changes(entity.net_id, part, &changed, sink);
                }
            }
            if !changed.is_empty() {
                scene.mark_fsm_dirty(entity.net_id, wake_reason::PART | wake_reason::TAG);
            }
        }
        FsmServerActionKind::ResetPart {
            name,
            reset_activate,
            reset_life,
        } => {
            let mut changed = Vec::new();
            {
                let storage = &mut scene.entities[handle.index];
                let tag_opt = storage.tag.as_mut();
                if let Some(part) = storage.part.as_mut() {
                    changed = part.reset(name, *reset_activate, *reset_life, tag_opt);
                    push_part_changes(entity.net_id, part, &changed, sink);
                }
            }
            if !changed.is_empty() {
                scene.mark_fsm_dirty(entity.net_id, wake_reason::PART | wake_reason::TAG);
            }
        }
    }
}

pub fn handle_fsm_buff_change(scene: &mut Scene, entity: EntityHandle) {
    scene.mark_fsm_dirty(entity.net_id, wake_reason::BUFF | wake_reason::ATTRIBUTE);
}

pub fn handle_fsm_gameplay_tag_change<S: PushSink + ?Sized>(
    qctx: &mut QuestContext<'_, S>,
    entity: EntityHandle,
    add_tag_ids: &[i32],
    remove_tag_ids: &[i32],
) {
    let mut tag_changed = !add_tag_ids.is_empty() || !remove_tag_ids.is_empty();
    let cfg_id_opt = {
        let Some(scene) = qctx.scene.as_deref_mut() else {
            return;
        };
        if let Some(handle) = scene.entity_handle(entity.net_id) {
            let storage = &mut scene.entities[handle.index];
            let cfg_id = storage.config.config_id;
            if let Some(fsm) = storage.fsm.as_mut() {
                let mirrors = fsm.drain_pending_tag_mirrors();
                if let Some(tag_comp) = storage.tag.as_mut() {
                    for mirror in mirrors {
                        if mirror.clear {
                            if tag_comp.tag_count(mirror.tag_id) > 0 {
                                tag_comp.remove_tag(mirror.tag_id);
                                tag_changed = true;
                            }
                        } else if mirror.delta != 0 {
                            let prev = tag_comp.tag_count(mirror.tag_id);
                            tag_comp.add_tag(mirror.tag_id, mirror.delta);
                            if tag_comp.tag_count(mirror.tag_id) != prev {
                                tag_changed = true;
                            }
                        }
                    }
                }
            }
            if tag_changed {
                scene.mark_fsm_dirty(entity.net_id, wake_reason::TAG);
            }
            Some(cfg_id)
        } else {
            if tag_changed {
                scene.mark_fsm_dirty(entity.net_id, wake_reason::TAG);
            }
            None
        }
    };

    if tag_changed
        && let Some(cfg_id) = cfg_id_opt
        && let Err(err) = quest_engine::on_entity_state_changed(qctx, cfg_id, "")
    {
        tracing::warn!(target: "quest_engine", "on_entity_state_changed failed: {err}");
    }
}

pub fn handle_fsm_lifecycle_complete<S: PushSink + ?Sized>(
    entity: EntityHandle,
    now_ms: i64,
    recheck: bool,
    scene: &mut Scene,
    assets: &Assets,
    sink: &mut S,
    events: &mut EventQueue,
) {
    let Some(handle) = scene.entity_handle(entity.net_id) else {
        return;
    };
    let mut data = Vec::new();
    {
        let storage = &mut scene.entities[handle.index];
        let Some(fsm) = storage.fsm.as_mut() else {
            return;
        };
        fsm.complete_lifecycle_effects();
        let do_recheck = recheck || fsm.take_lifecycle_recheck_request();
        let deferred = if do_recheck {
            fsm_lifecycle::enqueue_effects(handle, fsm, events, now_ms)
        } else {
            fsm_lifecycle::enqueue_effects_without_recheck(handle, fsm, events, now_ms)
        };
        if !deferred {
            let eval_buffers = FsmEvalBuffers::from_components(
                storage.attribute.as_ref(),
                storage.buffs.as_ref(),
                storage.tag.as_ref(),
                storage.part.as_ref(),
                storage.logic_state.as_ref(),
            );
            let eval_ctx = eval_buffers.as_context(
                Some(&assets.tables.gameplay_tag_parent),
                fsm.instance_state_tag,
                now_ms,
            );
            if do_recheck {
                let _ = fsm.append_ready_state_transitions(entity.net_id, &mut data, &eval_ctx);
            } else {
                fsm.clear_dirty_reason(wake_reason::INITIAL);
                let _ = fsm.append_dirty_state_transitions(entity.net_id, &mut data, &eval_ctx);
            }
        }
        fsm.finish_tick(now_ms);
        if !deferred && !fsm.lifecycle_effects().is_empty() {
            let _ = fsm_lifecycle::enqueue_effects_without_recheck(handle, fsm, events, now_ms);
        }
    }
    if !data.is_empty() {
        sink.push(
            CombatReceivePackNotify::MSG_ID,
            &CombatReceivePackNotify { Data: data },
        );
    }
    scene.sync_fsm_deadlines_for_entity(entity.net_id);
}

struct FsmEvalBuffers {
    attributes: Vec<i32>,
    buff_stacks: Vec<(i64, i32)>,
    tags: Vec<i32>,
    parts: Vec<crate::fsm::types::PartState>,
    position_state: Option<i32>,
}

impl FsmEvalBuffers {
    fn from_components(
        attr: Option<&AttributeComponent>,
        buffs: Option<&FightBuffComponent>,
        tags: Option<&TagComponent>,
        parts: Option<&PartComponent>,
        logic_state: Option<&crate::component::entity::LogicStateComponent>,
    ) -> Self {
        Self {
            attributes: attr
                .map(|a| a.attributes.iter().map(|entry| entry.current).collect())
                .unwrap_or_default(),
            buff_stacks: buffs
                .map(|b| b.buffs.iter().map(|info| (info.BuffId, info.StackCount)).collect())
                .unwrap_or_default(),
            tags: tags.map(|t| t.active_tag_ids()).unwrap_or_default(),
            parts: parts.map(|p| p.to_fsm_states()).unwrap_or_default(),
            position_state: logic_state.map(|ls| ls.position_state),
        }
    }

    fn as_context<'a>(
        &'a self,
        parent_tags: Option<&'a sonoro_gamesv_data::data_tables::Table<sonoro_gamesv_data::tables::gameplay_tag_parent::GameplayTagParent, i32>>,
        instance_state: Option<i32>,
        now_ms: i64,
    ) -> EvalContext<'a> {
        EvalContext {
            now_ms,
            flags: 0,
            position_state: self.position_state,
            attributes: &self.attributes,
            tags: &self.tags,
            parent_tags,
            buff_stacks: &self.buff_stacks,
            parts: &self.parts,
            instance_state,
        }
    }
}

fn push_part_changes<S: PushSink + ?Sized>(
    entity_id: i64,
    parts: &PartComponent,
    changed: &[usize],
    sink: &mut S,
) {
    if changed.is_empty() {
        return;
    }
    let mut part_infos = Vec::with_capacity(changed.len());
    for &idx in changed {
        parts.append_part_info(idx, &mut part_infos);
    }
    sink.push(
        CombatReceivePackNotify::MSG_ID,
        &CombatReceivePackNotify {
            Data: vec![CombatReceiveData {
                Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
                    CombatNotifyData {
                        CombatCommon: Some(CombatCommon {
                            EntityId: entity_id,
                            ..Default::default()
                        }),
                        Message: Some(CombatNotifyDataMessageOneof::PartUpdateNotify(
                            PartUpdateNotify {
                                EntityId: entity_id,
                                PartInfos: part_infos,
                            },
                        )),
                    },
                )),
            }],
        },
    );
}

fn apply_instance_state(
    entity: EntityHandle,
    fsm: &mut FsmComponent,
    tags: Option<&mut TagComponent>,
    tag_id: i32,
    events: &mut EventQueue,
) {
    let previous = fsm.instance_state_tag;
    if previous == Some(tag_id) {
        return;
    }
    fsm.instance_state_tag = Some(tag_id);
    if let Some(tag_comp) = tags {
        if let Some(old_tag) = previous {
            tag_comp.remove_tag(old_tag);
        }
        tag_comp.set_tag_count(tag_id, 1);
    }
    let remove_tag_ids = previous.into_iter().collect();
    events.enqueue(Event::GameplayTagChange {
        entity,
        add_tag_ids: vec![tag_id],
        remove_tag_ids,
        remove_before_add: previous.is_some(),
    });
}

fn update_paralysis<S: PushSink + ?Sized>(
    storage: &mut crate::component::entity::EntityComponentStorage,
    now_ms: i64,
    sink: &mut S,
) {
    let Some(fsm) = storage.fsm.as_mut() else {
        return;
    };
    if !fsm.paralysis_active {
        return;
    }
    let time_idx = EAttributeType::ParalysisTime.to_i32() as usize;
    let recover_idx = EAttributeType::ParalysisTimeRecover.to_i32() as usize;
    let Some(attr) = storage.attribute.as_mut() else {
        fsm.paralysis_active = false;
        fsm.paralysis_next_ms = None;
        return;
    };
    if time_idx >= attr.attributes.len() || recover_idx >= attr.attributes.len() {
        fsm.paralysis_active = false;
        fsm.paralysis_next_ms = None;
        return;
    }

    let elapsed_ms = (now_ms - fsm.paralysis_last_ms).max(0);
    let mut changed = Vec::new();
    let recover = attr.attributes[recover_idx].current;
    let delta = scaled_paralysis_recovery(recover, elapsed_ms);
    if attr.attributes[time_idx].current > 0 && delta != 0 {
        changed.extend(attributes::change_attr(
            attr,
            EAttributeType::ParalysisTime,
            AttributeChangeType::Delta,
            AttributeValueType::Current,
            delta,
        ));
    }
    if attr.attributes[time_idx].current <= 0 {
        fsm.paralysis_active = false;
        if recover != 0 {
            changed.extend(attributes::change_attr(
                attr,
                EAttributeType::ParalysisTimeRecover,
                AttributeChangeType::Override,
                AttributeValueType::Current,
                0,
            ));
        }
    }
    push_attribute_changes(storage.net_id, attr, &changed, sink);
    if !changed.is_empty() {
        fsm.mark_dirty(wake_reason::ATTRIBUTE);
    }
    fsm.paralysis_last_ms = fsm.paralysis_last_ms.max(now_ms);
    fsm.paralysis_next_ms = fsm.paralysis_active.then_some(now_ms + 50);
}

fn scaled_paralysis_recovery(recover: i32, elapsed_ms: i64) -> i32 {
    if elapsed_ms <= 0 || recover == 0 {
        return 0;
    }
    let delta = (i128::from(recover) * i128::from(elapsed_ms)) / 1000;
    delta.clamp(i128::from(i32::MIN), i128::from(i32::MAX)) as i32
}

fn reset_status_attributes<S: PushSink + ?Sized>(
    entity_id: i64,
    attr: &mut AttributeComponent,
    sink: &mut S,
) {
    let mut changed = Vec::new();
    for index in 0..attr.attributes.len() {
        let attr_type = EAttributeType::from_i32_or_default(index as i32);
        // Never refill HP (`Life`) when `ActionResetStatus` fires on phase/leash FSM transitions.
        if attr_type == EAttributeType::Life {
            continue;
        }
        let cur = attr.attributes[index];
        let desired_i64 = i64::from(cur.base) + i64::from(cur.increment);
        let desired = desired_i64.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32;
        if cur.current == desired {
            continue;
        }
        changed.extend(attributes::change_attr(
            attr,
            attr_type,
            AttributeChangeType::Override,
            AttributeValueType::Current,
            desired,
        ));
    }
    push_attribute_changes(entity_id, attr, &changed, sink);
}

fn set_rage_full_attributes(attr: &mut AttributeComponent) -> Vec<EAttributeType> {
    let rage_max_idx = EAttributeType::RageMax.to_i32() as usize;
    let Some(max_attr) = attr.attributes.get(rage_max_idx) else {
        return Vec::new();
    };
    attributes::change_attr(
        attr,
        EAttributeType::Rage,
        AttributeChangeType::Override,
        AttributeValueType::Current,
        max_attr.current,
    )
}

fn delayed_destroy_remove_type(is_monster: bool) -> Option<sonoro_proto::enums::ERemoveEntityType> {
    is_monster.then_some(sonoro_proto::enums::ERemoveEntityType::HpIsZero)
}

fn push_attribute_changes<S: PushSink + ?Sized>(
    entity_id: i64,
    attr: &AttributeComponent,
    changed: &[EAttributeType],
    sink: &mut S,
) {
    if changed.is_empty() {
        return;
    }
    let mut data = Vec::new();
    attributes::generate_attr_messages(&mut data, entity_id, attr, changed);
    if !data.is_empty() {
        sink.push(
            CombatReceivePackNotify::MSG_ID,
            &CombatReceivePackNotify { Data: data },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sonoro_proto::enums::ERemoveEntityType;

    #[test]
    fn delayed_destroy_uses_hp_is_zero_only_for_monsters() {
        assert_eq!(
            delayed_destroy_remove_type(true),
            Some(ERemoveEntityType::HpIsZero)
        );
        assert_eq!(delayed_destroy_remove_type(false), None);
    }
}
