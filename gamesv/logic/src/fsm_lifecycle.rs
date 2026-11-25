use crate::component::entity::fsm::FsmComponent;
use crate::event_queue::EventQueue;
use crate::events::{BuffAdditionEntry, EntityHandle, Event};
use crate::fsm::types::LifecycleEffect;

#[derive(Debug, Clone)]
struct AggregatedTagChange {
    tag_id: i32,
    first_was_present: bool,
    last_is_present: bool,
    net_delta_after_clear: i64,
    had_clear: bool,
}

pub fn enqueue_effects(
    entity: EntityHandle,
    fsm: &mut FsmComponent,
    event_queue: &mut EventQueue,
    now_ms: i64,
) -> bool {
    enqueue_effects_with_recheck(entity, fsm, event_queue, now_ms, true)
}

pub fn enqueue_effects_without_recheck(
    entity: EntityHandle,
    fsm: &mut FsmComponent,
    event_queue: &mut EventQueue,
    now_ms: i64,
) -> bool {
    enqueue_effects_with_recheck(entity, fsm, event_queue, now_ms, false)
}

fn enqueue_effects_with_recheck(
    entity: EntityHandle,
    fsm: &mut FsmComponent,
    event_queue: &mut EventQueue,
    now_ms: i64,
    recheck: bool,
) -> bool {
    if fsm.lifecycle_effects_pending() {
        fsm.request_lifecycle_recheck(recheck);
        return true;
    }
    if fsm.lifecycle_effects().is_empty() {
        return false;
    }

    let effects: Vec<LifecycleEffect> = fsm.lifecycle_effects().to_vec();
    let mut tag_summaries: Vec<AggregatedTagChange> = Vec::new();
    let mut enqueued_any = false;

    for effect in effects {
        match effect {
            LifecycleEffect::EnterFight => {
                event_queue.enqueue(Event::FsmServerAction {
                    entity,
                    kind: crate::events::FsmServerActionKind::EnterFight,
                });
                enqueued_any = true;
            }
            LifecycleEffect::AddBuff(buff_id) => {
                event_queue.enqueue(Event::BuffAddition {
                    target: entity,
                    instigator: entity,
                    buffs: vec![BuffAdditionEntry {
                        id: buff_id,
                        is_active: true,
                        ..Default::default()
                    }],
                });
                enqueued_any = true;
            }
            LifecycleEffect::RemoveBuff(buff_id) => {
                event_queue.enqueue(Event::BuffRemovalById { entity, buff_id });
                enqueued_any = true;
            }
            LifecycleEffect::BindBuffAdd(addition) => {
                event_queue.enqueue(Event::BuffAddition {
                    target: entity,
                    instigator: entity,
                    buffs: vec![BuffAdditionEntry {
                        id: addition.buff_id,
                        is_active: true,
                        fsm_bind_source: Some(addition.source),
                        ..Default::default()
                    }],
                });
                enqueued_any = true;
            }
            LifecycleEffect::BindBuffRemove(source) => {
                event_queue.enqueue(Event::BuffRemovalByFsmBind { entity, source });
                enqueued_any = true;
            }
            LifecycleEffect::CueParalysis => {
                event_queue.enqueue(Event::FsmServerAction {
                    entity,
                    kind: crate::events::FsmServerActionKind::CueParalysis,
                });
                enqueued_any = true;
            }
            LifecycleEffect::ResetStatus => {
                event_queue.enqueue(Event::FsmServerAction {
                    entity,
                    kind: crate::events::FsmServerActionKind::ResetStatus,
                });
                enqueued_any = true;
            }
            LifecycleEffect::SetRageFull => {
                event_queue.enqueue(Event::FsmServerAction {
                    entity,
                    kind: crate::events::FsmServerActionKind::SetRageFull,
                });
                enqueued_any = true;
            }
            LifecycleEffect::SetInstanceState(tag_id) => {
                event_queue.enqueue(Event::FsmServerAction {
                    entity,
                    kind: crate::events::FsmServerActionKind::InstanceState(tag_id),
                });
                enqueued_any = true;
            }
            LifecycleEffect::ActivatePart(part) => {
                event_queue.enqueue(Event::FsmServerAction {
                    entity,
                    kind: crate::events::FsmServerActionKind::ActivatePart {
                        name: part.name,
                        activate: part.activate,
                    },
                });
                enqueued_any = true;
            }
            LifecycleEffect::ResetPart(part) => {
                event_queue.enqueue(Event::FsmServerAction {
                    entity,
                    kind: crate::events::FsmServerActionKind::ResetPart {
                        name: part.name,
                        reset_activate: part.reset_activate,
                        reset_life: part.reset_life,
                    },
                });
                enqueued_any = true;
            }
            LifecycleEffect::TagMirror(mirror) => {
                if let Some(entry) = tag_summaries.iter_mut().find(|e| e.tag_id == mirror.tag_id) {
                    if mirror.clear {
                        entry.had_clear = true;
                        entry.net_delta_after_clear = 0;
                        entry.last_is_present = false;
                    } else {
                        entry.net_delta_after_clear += i64::from(mirror.delta);
                        entry.last_is_present = mirror.is_present;
                    }
                } else {
                    tag_summaries.push(AggregatedTagChange {
                        tag_id: mirror.tag_id,
                        first_was_present: mirror.was_present,
                        last_is_present: mirror.is_present,
                        net_delta_after_clear: if mirror.clear { 0 } else { i64::from(mirror.delta) },
                        had_clear: mirror.clear,
                    });
                }
            }
        }
    }

    if !tag_summaries.is_empty() {
        let mut add_tags: Vec<i32> = Vec::new();
        let mut remove_tags: Vec<i32> = Vec::new();
        let mut mirrors_pushed: usize = 0;

        for entry in tag_summaries {
            if entry.had_clear {
                fsm.pending_tag_mirrors.push(crate::fsm::types::TagMirror {
                    tag_id: entry.tag_id,
                    clear: true,
                    ..Default::default()
                });
                mirrors_pushed += 1;
                if entry.net_delta_after_clear > 0 {
                    let clamped = entry
                        .net_delta_after_clear
                        .clamp(i64::from(i32::MIN), i64::from(i32::MAX))
                        as i32;
                    fsm.pending_tag_mirrors.push(crate::fsm::types::TagMirror {
                        tag_id: entry.tag_id,
                        delta: clamped,
                        ..Default::default()
                    });
                    mirrors_pushed += 1;
                }
                if !entry.last_is_present || entry.first_was_present {
                    remove_tags.push(entry.tag_id);
                }
                if entry.last_is_present {
                    add_tags.push(entry.tag_id);
                }
            } else {
                if entry.net_delta_after_clear != 0 {
                    let clamped = entry
                        .net_delta_after_clear
                        .clamp(i64::from(i32::MIN), i64::from(i32::MAX))
                        as i32;
                    fsm.pending_tag_mirrors.push(crate::fsm::types::TagMirror {
                        tag_id: entry.tag_id,
                        delta: clamped,
                        ..Default::default()
                    });
                    mirrors_pushed += 1;
                }
                if !entry.first_was_present && entry.last_is_present {
                    add_tags.push(entry.tag_id);
                } else if entry.first_was_present && !entry.last_is_present {
                    remove_tags.push(entry.tag_id);
                }
            }
        }

        if mirrors_pushed != 0 || !add_tags.is_empty() || !remove_tags.is_empty() {
            event_queue.enqueue(Event::GameplayTagChange {
                entity,
                add_tag_ids: add_tags,
                remove_tag_ids: remove_tags,
                remove_before_add: true,
            });
            enqueued_any = true;
        }
    }

    if !enqueued_any {
        fsm.clear_lifecycle_effects();
        return false;
    }

    event_queue.enqueue(Event::FsmLifecycleComplete {
        entity,
        now_ms,
        recheck,
    });
    fsm.mark_lifecycle_effects_enqueued();
    true
}
