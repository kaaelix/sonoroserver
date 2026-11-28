use sonoro_gamesv_data::Assets;
use sonoro_gamesv_data::data_tables::create_buff_information;
use sonoro_gamesv_data::tables::buff::{Buff, DurationPolicy, StackingType};
use sonoro_proto::enums::ApplyGEType;
use sonoro_proto::pb::{
    ActivateBuffNotify, ApplyGameplayEffectNotify, ApplyGameplayEffectNotifyCRoundActionOneof,
    ApplyGameplayEffectNotifyTimeOneof, BuffStackCountNotify, BuffStackCountNotifyGFsOneof,
    BuffStackCountNotifyTimeOneof, CombatCommon, CombatNotifyData, CombatNotifyDataMessageOneof,
    CombatReceiveData, CombatReceiveDataMessageOneof, CombatReceivePackNotify, FightBuffInformation,
    RemoveGameplayEffectNotify,
};

use crate::component::entity::fight_buff::FsmBindBuffRelease;
use crate::component::entity::{AttributeComponent, FightBuffComponent};
use crate::event_queue::EventQueue;
use crate::events::{BuffAdditionEntry, EntityHandle, Event, FsmBindBuffSource, NaturalExpiration};
use crate::helpers::{buff as buff_helper, gameplay_tags};
use crate::scene::{Scene, now_millis};
use crate::schedulers::BuffTimerScheduler;
use crate::schedulers::buff_timer_scheduler::ApplicationDisposition;
use crate::sink::PushSink;

#[allow(clippy::too_many_arguments)]
pub fn remove_buff_from_entity<S: PushSink + ?Sized>(
    sink: &mut S,
    events: &mut EventQueue,
    scene: &mut Scene,
    assets: &Assets,
    buff_timers: &mut BuffTimerScheduler,
    entity: EntityHandle,
    handle_ids: &[i32],
    natural_expiration: Option<&NaturalExpiration>,
) {
    let Some(handle) = scene.entity_handle(entity.net_id) else {
        for &handle_id in handle_ids {
            buff_timers.cancel_handle(entity.net_id, handle_id);
        }
        return;
    };
    if scene.entities[handle.index].buffs.is_none() {
        for &handle_id in handle_ids {
            buff_timers.cancel_handle(entity.net_id, handle_id);
        }
        return;
    }

    let combat_common = CombatCommon {
        EntityId: entity.net_id,
        ..Default::default()
    };
    let mut notify_data = Vec::new();
    {
        let storage = &mut scene.entities[handle.index];
        let (Some(buff_comp), mut maybe_attr) = (storage.buffs.as_mut(), storage.attribute.as_mut()) else {
            return;
        };
        for &handle_id in handle_ids {
            buff_helper::append_shield_del_for_handle(
                &mut notify_data,
                entity.net_id,
                buff_comp,
                maybe_attr.as_deref_mut(),
                handle_id,
            );
            notify_data.push(CombatReceiveData {
                Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
                    CombatNotifyData {
                        CombatCommon: Some(combat_common.clone()),
                        Message: Some(CombatNotifyDataMessageOneof::RemoveGameplayEffectNotify(
                            RemoveGameplayEffectNotify {
                                EntityId: entity.net_id,
                                Handle: handle_id,
                                ..Default::default()
                            },
                        )),
                    },
                )),
            });
        }
        for &handle_id in handle_ids {
            buff_timers.cancel_handle(entity.net_id, handle_id);
            buff_comp.remove_by_handle(handle_id);
        }
    }

    events.enqueue(Event::BuffChange {
        player_id: scene.player_id,
        entity,
        buff_id: 0,
        stack_count: 0,
        is_add: false,
        instigator: entity,
    });
    if let Some(expiration) = natural_expiration
        && !expiration.follow_up_buffs.is_empty()
    {
        events.enqueue(Event::BuffAddition {
            target: entity,
            instigator: expiration.instigator,
            buffs: expiration.follow_up_buffs.clone(),
        });
    }

    if scene.entities[handle.index].attribute.is_some() {
        buff_helper::sync_continuous_buff_modifiers(&mut notify_data, scene, entity.net_id, assets);
    }

    if !notify_data.is_empty() {
        sink.push(
            CombatReceivePackNotify::MSG_ID,
            &CombatReceivePackNotify { Data: notify_data },
        );
    }
    tracing::debug!(target: "buff_removal", "removed buffs from {}: {:?}", entity.net_id, handle_ids);
}

pub fn remove_buff_from_entity_by_id<S: PushSink + ?Sized>(
    sink: &mut S,
    events: &mut EventQueue,
    scene: &mut Scene,
    assets: &Assets,
    buff_timers: &mut BuffTimerScheduler,
    entity: EntityHandle,
    buff_id: i64,
) {
    let Some(handle) = scene.entity_handle(entity.net_id) else {
        return;
    };
    let Some(buffs) = scene.entities[handle.index].buffs.as_ref() else {
        return;
    };
    let handles: Vec<i32> = buffs
        .buffs
        .iter()
        .filter(|b| b.BuffId == buff_id)
        .map(|b| b.HandleId)
        .collect();
    if handles.is_empty() {
        return;
    }
    remove_buff_handles_with_sync(
        entity,
        &handles,
        Some(scene),
        Some(assets),
        sink,
        events,
        buff_timers,
    );
}

pub fn remove_buff_from_entity_by_fsm_bind<S: PushSink + ?Sized>(
    sink: &mut S,
    events: &mut EventQueue,
    scene: &mut Scene,
    assets: &Assets,
    buff_timers: &mut BuffTimerScheduler,
    entity: EntityHandle,
    source: FsmBindBuffSource,
) {
    let Some(handle) = scene.entity_handle(entity.net_id) else {
        return;
    };
    let Some(buffs) = scene.entities[handle.index].buffs.as_mut() else {
        return;
    };
    let Some(release) = buffs.release_fsm_bind_lease(source) else {
        return;
    };
    match release {
        FsmBindBuffRelease::Preserve => {
            events.enqueue(Event::BuffChange {
                player_id: scene.player_id,
                entity,
                buff_id: 0,
                stack_count: 0,
                is_add: false,
                instigator: entity,
            });
        }
        FsmBindBuffRelease::Remove(handle_id) => {
            if buffs.find_by_handle(handle_id).is_some() {
                remove_buff_handles_with_sync(
                    entity,
                    &[handle_id],
                    Some(scene),
                    Some(assets),
                    sink,
                    events,
                    buff_timers,
                );
            } else {
                events.enqueue(Event::BuffChange {
                    player_id: scene.player_id,
                    entity,
                    buff_id: 0,
                    stack_count: 0,
                    is_add: false,
                    instigator: entity,
                });
            }
        }
    }
}

pub fn remove_buff_handles<S: PushSink + ?Sized>(
    entity: EntityHandle,
    handle_ids: &[i32],
    buffs: &mut FightBuffComponent,
    sink: &mut S,
    events: &mut EventQueue,
    buff_timers: &mut BuffTimerScheduler,
    player_id: i32,
) {
    let mut notify_data = Vec::new();
    let combat_common = CombatCommon {
        EntityId: entity.net_id,
        ..Default::default()
    };
    for &handle_id in handle_ids {
        if buffs.find_by_handle(handle_id).is_none() {
            continue;
        }
        buff_helper::append_shield_del_for_handle(
            &mut notify_data,
            entity.net_id,
            buffs,
            None::<&mut AttributeComponent>,
            handle_id,
        );
        buff_timers.cancel_handle(entity.net_id, handle_id);
        buffs.remove_by_handle(handle_id);
        notify_data.push(CombatReceiveData {
            Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
                CombatNotifyData {
                    CombatCommon: Some(combat_common.clone()),
                    Message: Some(CombatNotifyDataMessageOneof::RemoveGameplayEffectNotify(
                        RemoveGameplayEffectNotify {
                            EntityId: entity.net_id,
                            Handle: handle_id,
                            ..Default::default()
                        },
                    )),
                },
            )),
        });
    }
    if !notify_data.is_empty() {
        sink.push(
            CombatReceivePackNotify::MSG_ID,
            &CombatReceivePackNotify { Data: notify_data },
        );
        events.enqueue(Event::BuffChange {
            player_id,
            entity,
            buff_id: 0,
            stack_count: 0,
            is_add: false,
            instigator: entity,
        });
    }
}

pub fn remove_buff_handles_with_sync<S: PushSink + ?Sized>(
    entity: EntityHandle,
    handle_ids: &[i32],
    maybe_scene: Option<&mut Scene>,
    maybe_assets: Option<&Assets>,
    sink: &mut S,
    events: &mut EventQueue,
    buff_timers: &mut BuffTimerScheduler,
) {
    let Some(scene) = maybe_scene else {
        return;
    };
    let Some(handle) = scene.entity_handle(entity.net_id) else {
        return;
    };
    let combat_common = CombatCommon {
        EntityId: entity.net_id,
        ..Default::default()
    };
    let mut notify_data = Vec::new();

    {
        let storage = &mut scene.entities[handle.index];
        let (Some(buffs), mut maybe_attr) = (storage.buffs.as_mut(), storage.attribute.as_mut()) else {
            return;
        };
        for &handle_id in handle_ids {
            if buffs.find_by_handle(handle_id).is_none() {
                continue;
            }
            buff_helper::append_shield_del_for_handle(
                &mut notify_data,
                entity.net_id,
                buffs,
                maybe_attr.as_deref_mut(),
                handle_id,
            );
            buff_timers.cancel_handle(entity.net_id, handle_id);
            buffs.remove_by_handle(handle_id);
            notify_data.push(CombatReceiveData {
                Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
                    CombatNotifyData {
                        CombatCommon: Some(combat_common.clone()),
                        Message: Some(CombatNotifyDataMessageOneof::RemoveGameplayEffectNotify(
                            RemoveGameplayEffectNotify {
                                EntityId: entity.net_id,
                                Handle: handle_id,
                                ..Default::default()
                            },
                        )),
                    },
                )),
            });
        }
    }

    if notify_data.is_empty() {
        return;
    }
    if scene.entities[handle.index].attribute.is_some()
        && let Some(assets) = maybe_assets
    {
        buff_helper::sync_continuous_buff_modifiers(&mut notify_data, scene, entity.net_id, assets);
    }
    sink.push(
        CombatReceivePackNotify::MSG_ID,
        &CombatReceivePackNotify { Data: notify_data },
    );
    events.enqueue(Event::BuffChange {
        player_id: scene.player_id,
        entity,
        buff_id: 0,
        stack_count: 0,
        is_add: false,
        instigator: entity,
    });
}

#[allow(clippy::too_many_arguments)]
pub fn add_buff_to_entity<S: PushSink + ?Sized>(
    sink: &mut S,
    events: &mut EventQueue,
    scene: &mut Scene,
    buff_timers: &mut BuffTimerScheduler,
    assets: &Assets,
    target: EntityHandle,
    instigator: EntityHandle,
    entries: &[BuffAdditionEntry],
) {
    let Some(handle) = scene.entity_handle(target.net_id) else {
        return;
    };
    if scene.entities[handle.index].buffs.is_none() {
        return;
    }

    let combat_common = CombatCommon {
        EntityId: target.net_id,
        MessageId: -1,
        PreMessageId: -1,
        ..Default::default()
    };
    let mut notify_data = Vec::new();
    let mut buff_change_enqueued = false;
    let now_ms = now_millis();
    buff_timers.ensure_entity_registered(scene, assets, target.net_id, now_ms);
    let player_id = scene.player_id;

    // Attach passive HardnessRecover regen (Buff 1102) once for FSM-driven monsters
    // whose BaseProperty has recoverable white bar (e.g. elites Cycling into C白条常态).
    let needs_hardness_regen_buff = {
        let storage = &scene.entities[handle.index];
        storage.config.entity_type == crate::component::entity::small::EntityType::Monster
            && entries.iter().any(|e| e.fsm_bind_source.is_some())
            && !entries.iter().any(|e| e.id == 1102)
            && storage
                .buffs
                .as_ref()
                .is_some_and(|b| b.get_by_buff_id(1102).is_none())
            && storage.attribute.as_ref().is_some_and(|a| {
                crate::helpers::attributes::get_attr_value(a, sonoro_proto::enums::EAttributeType::HardnessMax) > 0
                    && crate::helpers::attributes::get_attr_value(a, sonoro_proto::enums::EAttributeType::HardnessRecover) > 0
            })
    };
    if needs_hardness_regen_buff {
        events.enqueue(Event::BuffAddition {
            target,
            instigator: target,
            buffs: vec![BuffAdditionEntry {
                id: 1102,
                is_active: true,
                stack_count: 1,
                duration_seconds: None,
                fsm_bind_source: None,
                message_id: -1,
                skill_id: 0,
                level: 1,
                notify_client: true,
            }],
        });
    }

    for entry in entries {
        let Some(buff_data) = assets.tables.buff.get_data_by_id(&entry.id) else {
            continue;
        };
        if !buff_data.RemoveBuffWithTags.is_empty() {
            remove_buffs_with_tags_inline(
                &mut notify_data,
                scene,
                handle,
                &buff_data.RemoveBuffWithTags,
                assets,
                events,
                buff_timers,
                &mut buff_change_enqueued,
                player_id,
            );
        }

        if buff_data.DurationPolicy == DurationPolicy::Instant {
            let entry_level = entry.level.max(1);
            notify_data.push(CombatReceiveData {
                Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
                    CombatNotifyData {
                        CombatCommon: Some(combat_common.clone()),
                        Message: Some(CombatNotifyDataMessageOneof::ApplyGameplayEffectNotify(
                            ApplyGameplayEffectNotify {
                                Handle: -2,
                                Id: buff_data.Id,
                                Level: entry_level,
                                EntityId: target.net_id,
                                InstigatorId: instigator.net_id,
                                IsActive: true,
                                ..Default::default()
                            },
                        )),
                    },
                )),
            });
            buff_helper::execute_buff_effects_with_assets_and_level(
                &mut notify_data,
                target.net_id,
                instigator.net_id,
                &buff_data,
                scene,
                assets,
                entry_level,
            );
            continue;
        }

        if let Some(source) = entry.fsm_bind_source
            && let Some(buffs) = scene.entities[handle.index].buffs.as_mut()
            && !buffs.prepare_fsm_bind_acquire(source)
        {
            continue;
        }

        let existing_handle_id: Option<i32> = scene.entities[handle.index].buffs.as_ref().and_then(|b| {
            let found = match buff_data.StackingType {
                StackingType::None => None,
                StackingType::ByInstigator => b.get_by_buff_id_and_instigator(entry.id, instigator.net_id),
                StackingType::ByTarget => b.get_by_buff_id(entry.id),
            };
            found.map(|info| info.HandleId)
        });

        let stack_limit = if buff_data.StackLimitCount > 0 {
            buff_data.StackLimitCount.min(i32::MAX as u32) as i32
        } else {
            i32::MAX
        };

        if buff_data.StackingType == StackingType::None && buff_data.StackLimitCount > 0 {
            let count = scene.entities[handle.index]
                .buffs
                .as_ref()
                .map_or(0, |b| b.buffs.iter().filter(|x| x.BuffId == entry.id).count() as i32);
            if count >= stack_limit {
                enqueue_overflow_effects(events, target, instigator, &buff_data);
                if buff_data.bClearStackOnOverflow {
                    let clear_handles: Vec<i32> = scene.entities[handle.index]
                        .buffs
                        .as_ref()
                        .map(|b| {
                            b.buffs
                                .iter()
                                .filter(|x| x.BuffId == entry.id)
                                .map(|x| x.HandleId)
                                .collect()
                        })
                        .unwrap_or_default();
                    for hid in clear_handles {
                        remove_single_buff_inline(
                            &mut notify_data,
                            scene,
                            handle,
                            hid,
                            events,
                            buff_timers,
                            &mut buff_change_enqueued,
                            player_id,
                        );
                    }
                    continue;
                }
                if buff_data.bDenyOverflowApplication {
                    continue;
                }
            }
        }

        if let Some(existing_hid) = existing_handle_id {
            let existing_snap = scene.entities[handle.index]
                .buffs
                .as_ref()
                .and_then(|b| b.find_by_handle(existing_hid))
                .cloned();
            let Some(buff) = existing_snap else {
                continue;
            };
            let append_stacks = if entry.stack_count > 0 {
                entry.stack_count
            } else if buff_data.StackAppendCount > 0 {
                buff_data.StackAppendCount as i32
            } else {
                0
            };
            let desired_total = i64::from(buff.StackCount) + i64::from(append_stacks);
            let overflowed = buff.StackCount >= stack_limit || desired_total > i64::from(stack_limit);
            if overflowed {
                enqueue_overflow_effects(events, target, instigator, &buff_data);
                if buff_data.bClearStackOnOverflow {
                    remove_single_buff_inline(
                        &mut notify_data,
                        scene,
                        handle,
                        buff.HandleId,
                        events,
                        buff_timers,
                        &mut buff_change_enqueued,
                        player_id,
                    );
                    continue;
                }
                if buff_data.bDenyOverflowApplication && buff.StackCount >= stack_limit {
                    continue;
                }
            }

            let next_stack_count = (desired_total.min(i64::from(stack_limit)) as i32).max(1);
            if let Some(buffs) = scene.entities[handle.index].buffs.as_mut() {
                if let Some(source) = entry.fsm_bind_source {
                    buffs.add_fsm_bind_lease(source, buff.HandleId, false);
                } else {
                    buffs.mark_fsm_bind_handle_external(buff.HandleId);
                }
            }
            buff_timers.sync_handle_left_duration(scene, target.net_id, buff.HandleId, now_ms);
            let was_active = buff.IsActive;
            let disposition =
                BuffTimerScheduler::application_disposition(&buff_data, false, was_active, entry.is_active);
            let mut next_buff = buff.clone();
            next_buff.Level = if entry.level > 0 {
                entry.level
            } else if next_buff.Level > 0 {
                next_buff.Level
            } else {
                1
            };
            next_buff.StackCount = next_stack_count;
            next_buff.InstigatorId = instigator.net_id;
            next_buff.EntityId = target.net_id;
            if disposition.refresh_duration {
                apply_buff_duration(&mut next_buff, &buff_data, entry.duration_seconds);
            }
            next_buff.ApplyType = Some(ApplyGEType::Common);
            next_buff.IsActive = entry.is_active;
            next_buff.MessageId = -1;

            if entry.notify_client {
                append_stack_count_notify(&mut notify_data, &combat_common, &next_buff, disposition);
                if was_active != next_buff.IsActive {
                    append_activate_buff_notify(
                        &mut notify_data,
                        &combat_common,
                        next_buff.HandleId,
                        next_buff.IsActive,
                    );
                }
            }
            enqueue_buff_change_once(events, target, &mut buff_change_enqueued, player_id);
            buff_timers.refresh_buff(&next_buff, &buff_data, target.net_id, now_ms, disposition);
            if let Some(slot) = scene.entities[handle.index]
                .buffs
                .as_mut()
                .and_then(|b| b.find_by_handle_mut(existing_hid))
            {
                *slot = next_buff.clone();
            }
            buff_helper::sync_buff_shield(
                &mut notify_data,
                scene,
                assets,
                target.net_id,
                &next_buff,
                &buff_data,
            );
            if disposition.execute_periodic_now {
                buff_helper::execute_periodic_with_assets(
                    &mut notify_data,
                    target.net_id,
                    next_buff.InstigatorId,
                    &buff_data,
                    scene,
                    assets,
                );
            }
        } else {
            let raw_init_stacks = if entry.stack_count > 0 {
                entry.stack_count
            } else if buff_data.DefaultStackCount > 0 {
                buff_data.DefaultStackCount as i32
            } else {
                1
            };
            let init_stacks = raw_init_stacks.min(stack_limit).max(1);
            scene.instance.buff_handle += 1;
            let handle_id = scene.instance.buff_handle;
            let mut new_buff = create_buff_information(
                handle_id,
                entry.id,
                instigator.net_id,
                target.net_id,
                entry.is_active,
            );
            new_buff.Level = entry.level.max(1);
            new_buff.StackCount = init_stacks;
            apply_buff_duration(&mut new_buff, &buff_data, entry.duration_seconds);
            let disposition =
                BuffTimerScheduler::application_disposition(&buff_data, true, false, entry.is_active);
            if entry.notify_client {
                notify_data.push(CombatReceiveData {
                    Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
                        CombatNotifyData {
                            CombatCommon: Some(combat_common.clone()),
                            Message: Some(CombatNotifyDataMessageOneof::ApplyGameplayEffectNotify(
                                ApplyGameplayEffectNotify {
                                    Handle: new_buff.HandleId,
                                    Id: new_buff.BuffId,
                                    Level: new_buff.Level,
                                    EntityId: new_buff.EntityId,
                                    InstigatorId: new_buff.InstigatorId,
                                    ApplyType: Some(new_buff.ApplyType.unwrap_or(ApplyGEType::Common)),
                                    IsActive: new_buff.IsActive,
                                    ServerId: new_buff.ServerId,
                                    StackCount: new_buff.StackCount,
                                    CRoundAction: Some(
                                        ApplyGameplayEffectNotifyCRoundActionOneof::Duration(
                                            new_buff.Duration,
                                        ),
                                    ),
                                    Time: Some(ApplyGameplayEffectNotifyTimeOneof::LeftDuration(
                                        new_buff.LeftDuration,
                                    )),
                                    ConfBuffId: new_buff.ConfBuffId,
                                },
                            )),
                        },
                    )),
                });
            }
            enqueue_buff_change_once(events, target, &mut buff_change_enqueued, player_id);
            buff_timers.schedule_new_buff(&new_buff, &buff_data, target.net_id, now_ms);
            if let Some(buffs) = scene.entities[handle.index].buffs.as_mut() {
                buffs.buffs.push(new_buff.clone());
                if let Some(source) = entry.fsm_bind_source {
                    buffs.add_fsm_bind_lease(source, new_buff.HandleId, true);
                }
            }
            buff_helper::sync_buff_shield(
                &mut notify_data,
                scene,
                assets,
                target.net_id,
                &new_buff,
                &buff_data,
            );
            if disposition.execute_periodic_now {
                buff_helper::execute_periodic_with_assets(
                    &mut notify_data,
                    target.net_id,
                    new_buff.InstigatorId,
                    &buff_data,
                    scene,
                    assets,
                );
            }
        }
    }

    if scene.entities[handle.index].attribute.is_some() {
        buff_helper::sync_continuous_buff_modifiers(&mut notify_data, scene, target.net_id, assets);
    }

    if !notify_data.is_empty() {
        sink.push(
            CombatReceivePackNotify::MSG_ID,
            &CombatReceivePackNotify { Data: notify_data },
        );
    }
    tracing::debug!(
        target: "buff_addition",
        "eid {}: added {} buffs to {}",
        instigator.net_id,
        entries.len(),
        target.net_id
    );
}

#[allow(clippy::too_many_arguments)]
fn remove_single_buff_inline(
    data: &mut Vec<CombatReceiveData>,
    scene: &mut Scene,
    target: EntityHandle,
    handle_id: i32,
    events: &mut EventQueue,
    buff_timers: &mut BuffTimerScheduler,
    enqueued: &mut bool,
    player_id: i32,
) {
    let storage = &mut scene.entities[target.index];
    let (Some(buffs), maybe_attr) = (storage.buffs.as_mut(), storage.attribute.as_mut()) else {
        return;
    };
    if buffs.find_by_handle(handle_id).is_none() {
        return;
    }
    buff_helper::append_shield_del_for_handle(
        data,
        target.net_id,
        buffs,
        maybe_attr,
        handle_id,
    );
    buff_timers.cancel_handle(target.net_id, handle_id);
    buffs.remove_by_handle(handle_id);
    data.push(CombatReceiveData {
        Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
            CombatNotifyData {
                CombatCommon: Some(CombatCommon {
                    EntityId: target.net_id,
                    ..Default::default()
                }),
                Message: Some(CombatNotifyDataMessageOneof::RemoveGameplayEffectNotify(
                    RemoveGameplayEffectNotify {
                        EntityId: target.net_id,
                        Handle: handle_id,
                        ..Default::default()
                    },
                )),
            },
        )),
    });
    enqueue_buff_change_once(events, target, enqueued, player_id);
}

#[allow(clippy::too_many_arguments)]
fn remove_buffs_with_tags_inline(
    data: &mut Vec<CombatReceiveData>,
    scene: &mut Scene,
    target: EntityHandle,
    tag_names: &[String],
    assets: &Assets,
    events: &mut EventQueue,
    buff_timers: &mut BuffTimerScheduler,
    enqueued: &mut bool,
    player_id: i32,
) {
    let Some(buffs) = scene.entities[target.index].buffs.as_ref() else {
        return;
    };
    if tag_names.is_empty() || buffs.buffs.is_empty() {
        return;
    }
    let removal_tag_ids: Vec<i32> = tag_names
        .iter()
        .filter_map(|name| gameplay_tags::id_from_name(&assets.tables.gameplay_tag_remap, name))
        .collect();
    if removal_tag_ids.is_empty() {
        return;
    }
    let mut to_remove = Vec::new();
    for info in &buffs.buffs {
        let Some(candidate) = assets.tables.buff.get_data_by_id(&info.BuffId) else {
            continue;
        };
        let matched = candidate.GrantedTags.iter().any(|granted_name| {
            gameplay_tags::id_from_name(&assets.tables.gameplay_tag_remap, granted_name)
                .is_some_and(|gid| {
                    removal_tag_ids
                        .iter()
                        .any(|&rem_id| gameplay_tags::contains(&assets.tables.gameplay_tag_parent, gid, rem_id))
                })
        });
        if matched {
            to_remove.push(info.HandleId);
        }
    }
    for hid in to_remove {
        remove_single_buff_inline(
            data,
            scene,
            target,
            hid,
            events,
            buff_timers,
            enqueued,
            player_id,
        );
    }
}

fn enqueue_overflow_effects(
    events: &mut EventQueue,
    target: EntityHandle,
    instigator: EntityHandle,
    buff_data: &Buff,
) {
    if buff_data.OverflowEffects.is_empty() {
        return;
    }
    let follow_ups: Vec<BuffAdditionEntry> = buff_data
        .OverflowEffects
        .iter()
        .copied()
        .filter(|&ov_id| ov_id != buff_data.Id)
        .map(|ov_id| BuffAdditionEntry {
            id: ov_id,
            is_active: true,
            ..Default::default()
        })
        .collect();
    if !follow_ups.is_empty() {
        events.enqueue(Event::BuffAddition {
            target,
            instigator,
            buffs: follow_ups,
        });
    }
}

fn append_stack_count_notify(
    data: &mut Vec<CombatReceiveData>,
    combat_common: &CombatCommon,
    buff: &FightBuffInformation,
    disposition: ApplicationDisposition,
) {
    let mut stack_notify = BuffStackCountNotify {
        HandleId: buff.HandleId,
        NewStackCount: buff.StackCount,
        InstigatorId: buff.InstigatorId,
        NotRefreshDuration: !disposition.refresh_duration,
        NotRefreshPeriod: !disposition.reset_period,
        ..Default::default()
    };
    if disposition.refresh_duration {
        stack_notify.Time = Some(BuffStackCountNotifyTimeOneof::Duration(buff.Duration));
        if buff.LeftDuration > 0.0 {
            stack_notify.gFs = Some(BuffStackCountNotifyGFsOneof::LeftDuration(
                buff.LeftDuration,
            ));
        }
    }
    data.push(CombatReceiveData {
        Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
            CombatNotifyData {
                CombatCommon: Some(combat_common.clone()),
                Message: Some(CombatNotifyDataMessageOneof::BuffStackCountNotify(
                    stack_notify,
                )),
            },
        )),
    });
}

fn append_activate_buff_notify(
    data: &mut Vec<CombatReceiveData>,
    combat_common: &CombatCommon,
    handle_id: i32,
    is_active: bool,
) {
    data.push(CombatReceiveData {
        Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
            CombatNotifyData {
                CombatCommon: Some(combat_common.clone()),
                Message: Some(CombatNotifyDataMessageOneof::ActivateBuffNotify(
                    ActivateBuffNotify {
                        Handle: handle_id,
                        On: is_active,
                    },
                )),
            },
        )),
    });
}

fn enqueue_buff_change_once(
    events: &mut EventQueue,
    entity: EntityHandle,
    enqueued: &mut bool,
    player_id: i32,
) {
    if *enqueued {
        return;
    }
    events.enqueue(Event::BuffChange {
        player_id,
        entity,
        buff_id: 0,
        stack_count: 0,
        is_add: false,
        instigator: entity,
    });
    *enqueued = true;
}

pub fn handle_buff_timers_due<S: PushSink + ?Sized>(
    sink: &mut S,
    events: &mut EventQueue,
    buff_timers: &mut BuffTimerScheduler,
    scene: &mut Scene,
    assets: &Assets,
    now_ms: i64,
) -> Result<(), &'static str> {
    let mut combat_receive_pack = Vec::new();
    buff_timers.drain_one_due(now_ms, events, scene, assets, &mut combat_receive_pack)?;
    if !combat_receive_pack.is_empty() {
        sink.push(
            CombatReceivePackNotify::MSG_ID,
            &CombatReceivePackNotify {
                Data: combat_receive_pack,
            },
        );
    }
    Ok(())
}

fn apply_buff_duration(
    buff: &mut FightBuffInformation,
    buff_data: &Buff,
    requested_duration: Option<f32>,
) {
    match buff_data.DurationPolicy {
        DurationPolicy::Instant => {
            buff.Duration = 0.0;
            buff.LeftDuration = 0.0;
        }
        DurationPolicy::Infinite => {
            buff.Duration = -1.0;
            buff.LeftDuration = -1.0;
        }
        DurationPolicy::HasDuration => {
            let duration = requested_duration
                .unwrap_or_else(|| buff_helper::configured_duration_seconds(buff_data));
            if duration > 0.0 {
                buff.Duration = duration;
                buff.LeftDuration = duration;
            } else {
                buff.Duration = -1.0;
                buff.LeftDuration = -1.0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_refresh_notify_omits_preserved_duration() {
        let mut data = Vec::new();
        append_stack_count_notify(
            &mut data,
            &CombatCommon {
                EntityId: 10,
                ..Default::default()
            },
            &FightBuffInformation {
                HandleId: 20,
                StackCount: 3,
                InstigatorId: 30,
                Duration: 2.0,
                LeftDuration: 1.0,
                ..Default::default()
            },
            ApplicationDisposition {
                refresh_duration: false,
                reset_period: false,
                execute_periodic_now: false,
            },
        );

        let CombatReceiveData {
            Message:
                Some(CombatReceiveDataMessageOneof::CombatNotifyData(CombatNotifyData {
                    Message: Some(CombatNotifyDataMessageOneof::BuffStackCountNotify(notify)),
                    ..
                })),
        } = &data[0]
        else {
            panic!("expected BuffStackCountNotify");
        };
        assert!(notify.NotRefreshDuration);
        assert!(notify.NotRefreshPeriod);
        assert!(notify.Time.is_none());
        assert!(notify.gFs.is_none());
    }

    #[test]
    fn stack_refresh_notify_includes_refreshed_duration() {
        let mut data = Vec::new();
        append_stack_count_notify(
            &mut data,
            &CombatCommon {
                EntityId: 10,
                ..Default::default()
            },
            &FightBuffInformation {
                HandleId: 20,
                StackCount: 3,
                InstigatorId: 30,
                Duration: 2.0,
                LeftDuration: 2.0,
                ..Default::default()
            },
            ApplicationDisposition {
                refresh_duration: true,
                reset_period: true,
                execute_periodic_now: false,
            },
        );

        let CombatReceiveData {
            Message:
                Some(CombatReceiveDataMessageOneof::CombatNotifyData(CombatNotifyData {
                    Message: Some(CombatNotifyDataMessageOneof::BuffStackCountNotify(notify)),
                    ..
                })),
        } = &data[0]
        else {
            panic!("expected BuffStackCountNotify");
        };
        assert!(!notify.NotRefreshDuration);
        assert!(!notify.NotRefreshPeriod);
        assert_eq!(notify.Time, Some(BuffStackCountNotifyTimeOneof::Duration(2.0)));
        assert_eq!(notify.gFs, Some(BuffStackCountNotifyGFsOneof::LeftDuration(2.0)));
    }

    #[test]
    fn activation_notify_carries_the_changed_active_state() {
        let mut data = Vec::new();
        append_activate_buff_notify(
            &mut data,
            &CombatCommon {
                EntityId: 10,
                ..Default::default()
            },
            20,
            true,
        );

        let CombatReceiveData {
            Message:
                Some(CombatReceiveDataMessageOneof::CombatNotifyData(CombatNotifyData {
                    Message: Some(CombatNotifyDataMessageOneof::ActivateBuffNotify(notify)),
                    ..
                })),
        } = &data[0]
        else {
            panic!("expected ActivateBuffNotify");
        };
        assert_eq!(notify.Handle, 20);
        assert!(notify.On);
    }
}
