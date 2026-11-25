use sonoro_gamesv_data::Assets;
use sonoro_gamesv_data::tables::buff::{DurationPolicy, StackingType};
use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::events::{BuffAdditionEntry, Event};
use sonoro_gamesv_logic::fsm::types::wake_reason;
use sonoro_gamesv_logic::helpers::{
    attributes::{self, AttributeChangeType, AttributeValueType},
    buff as buff_helper, gameplay_tags,
};
use sonoro_gamesv_logic::scene::{Scene, now_millis};
use sonoro_gamesv_logic::schedulers::BuffTimerScheduler;
use sonoro_proto::enums::ErrorCode;
use sonoro_proto::pb::{
    ActivateBuffNotify, ActivateBuffRequest, ActivateBuffResponse, ActiveBuffPush,
    ApplyGameplayEffectNotify, ApplyGameplayEffectPush, ApplyGameplayEffectPushTimeOneof,
    ApplyGameplayEffectRequest, ApplyGameplayEffectRequestTimeOneof,
    ApplyGameplayEffectResponse, AttributeChangedRequest,
    AttributeChangedResponse, BuffDurationNotify, BuffDurationNotifyGFsOneof,
    BuffDurationNotifyTimeOneof, BuffStackCountNotify, BuffStackCountNotifyGFsOneof,
    BuffStackCountNotifyTimeOneof, BuffStackCountPush, BuffStackCountRequest,
    BuffStackCountResponse, CombatCommon, CombatNotifyData, CombatNotifyDataMessageOneof,
    CombatReceiveData, CombatReceiveDataMessageOneof, CombatReceivePackNotify,
    OrderApplyBuffRequest, OrderApplyBuffRequestTimeOneof, OrderApplyBuffResponse,
    OrderRemoveBuffByTagsRequest, OrderRemoveBuffByTagsResponse, OrderRemoveBuffRequest,
    OrderRemoveBuffResponse, PassiveSkillAddNotify, PassiveSkillAddPush, PassiveSkillAddRequest,
    PassiveSkillAddResponse, PassiveSkillItemPb, PassiveSkillRemoveNotify, PassiveSkillRemovePush,
    PassiveSkillRemoveRequest, PassiveSkillRemoveResponse, RefreshBuffDurationPush,
    RemoveGameplayEffectNotify, RemoveGameplayEffectPush, RemoveGameplayEffectRequest,
    RemoveGameplayEffectResponse,
};

use super::State;

const MAX_DURATION_SECONDS: f64 = (i64::MAX as f64) / 1000.0;

pub fn handle_attribute_changed_request(
    state: &mut State,
    _events: &mut EventQueue,
    req: AttributeChangedRequest,
) -> Result<AttributeChangedResponse, String> {
    let Some(scene) = state.scene.as_mut() else {
        return Ok(AttributeChangedResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    };
    let Some(&idx) = scene.net_id_map.get(&req.Id) else {
        return Ok(AttributeChangedResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    };
    let Some(attr) = scene.entities[idx].attribute.as_mut() else {
        return Ok(AttributeChangedResponse {
            ErrorCode: Some(ErrorCode::Success),
        });
    };

    let mut changed = Vec::new();
    for attr_pb in &req.Attributes {
        let Some(attr_type) = attr_pb.AttributeType else {
            continue;
        };
        let ch = attributes::change_attr(
            attr,
            attr_type,
            AttributeChangeType::Override,
            AttributeValueType::Current,
            attr_pb.CurrentValue,
        );
        changed.extend(ch);
    }

    if !changed.is_empty() {
        let mut pack = Vec::new();
        attributes::generate_attr_messages(&mut pack, req.Id, attr, &changed);
        if !pack.is_empty() {
            let _ = state
                .conn
                .push_msg(CombatReceivePackNotify::MSG_ID, &CombatReceivePackNotify { Data: pack });
        }
        scene.mark_fsm_dirty(req.Id, wake_reason::ATTRIBUTE);
        let _ = scene.save_entity_by_net_id(&state.fs, req.Id);
    }
    Ok(AttributeChangedResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

fn common_entity_id(common: Option<&CombatCommon>) -> Option<i64> {
    let value = common?;
    (value.EntityId != 0).then_some(value.EntityId)
}

fn validate_duration(duration: f32) -> Result<f32, ()> {
    if !duration.is_finite() || (duration > 0.0 && f64::from(duration) > MAX_DURATION_SECONDS) {
        return Err(());
    }
    Ok(duration)
}

fn request_duration(time: Option<&OrderApplyBuffRequestTimeOneof>) -> Result<Option<f32>, ()> {
    let Some(OrderApplyBuffRequestTimeOneof::Duration(d)) = time else {
        return Ok(None);
    };
    validate_duration(*d).map(Some)
}

#[allow(clippy::too_many_arguments)]
fn apply_buff_request_impl(
    state: &mut State,
    events: &mut EventQueue,
    entity_id: i64,
    buff_id: i64,
    instigator_id: i64,
    stack_count: i32,
    is_active: bool,
    duration_seconds: Option<f32>,
    echo_instant_notify: bool,
    receive_data_pack: &mut Vec<CombatReceiveData>,
) -> bool {
    let Some(buff_data) = state
        .assets
        .tables
        .buff
        .get_data_by_id(&buff_id)
        .map(|b| b.into_owned())
    else {
        return false;
    };
    let Some(scene) = state.scene.as_mut() else {
        return false;
    };

    if buff_data.DurationPolicy == DurationPolicy::Instant {
        // Client ApplyGameplayEffectPush originates after local AddBuffLocal execution.
        // Echoing ApplyGameplayEffectNotify { Handle: -2 } back to the same client re-triggers
        // AddBuffRemote(isIterable=true) -> ShareApplyBuffInner -> AddBuffLocal on teammates for
        // SharedByFormation buffs (e.g. 1512310011), causing an exponential feedback loop.
        if echo_instant_notify {
            receive_data_pack.push(CombatReceiveData {
                Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
                    CombatNotifyData {
                        CombatCommon: Some(CombatCommon {
                            EntityId: entity_id,
                            MessageId: -1,
                            PreMessageId: -1,
                            ..Default::default()
                        }),
                        Message: Some(CombatNotifyDataMessageOneof::ApplyGameplayEffectNotify(
                            ApplyGameplayEffectNotify {
                                Handle: -2,
                                Id: buff_data.Id,
                                EntityId: entity_id,
                                InstigatorId: instigator_id,
                                IsActive: true,
                                ..Default::default()
                            },
                        )),
                    },
                )),
            });
        }
        buff_helper::execute_buff_effects(
            receive_data_pack,
            entity_id,
            instigator_id,
            &buff_data,
            scene,
        );
        return true;
    }

    let Some(target) = scene.entity_handle(entity_id) else {
        return false;
    };
    if scene.entities[target.index].buffs.is_none() {
        return false;
    }
    let instigator = if instigator_id != 0 {
        scene.entity_handle(instigator_id).unwrap_or(target)
    } else {
        target
    };
    events.enqueue(Event::BuffAddition {
        target,
        instigator,
        buffs: vec![BuffAdditionEntry {
            id: buff_id,
            stack_count,
            is_active,
            duration_seconds,
            ..Default::default()
        }],
    });
    true
}

pub fn handle_order_apply_buff_request(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    receive_data_pack: &mut Vec<CombatReceiveData>,
    req: OrderApplyBuffRequest,
) -> OrderApplyBuffResponse {
    let Some(entity_id) = common_entity_id(common) else {
        return OrderApplyBuffResponse {
            ErrorCode: Some(ErrorCode::ErrOrderApplyBuffFailed),
        };
    };
    let Ok(duration_seconds) = request_duration(req.Time.as_ref()) else {
        return OrderApplyBuffResponse {
            ErrorCode: Some(ErrorCode::ErrOrderApplyBuffFailed),
        };
    };
    let mut pack = Vec::new();
    let ok = apply_buff_request_impl(
        state,
        events,
        entity_id,
        req.Id,
        req.InstigatorId,
        req.StackCount,
        true,
        duration_seconds,
        true,
        &mut pack,
    );
    if !ok {
        return OrderApplyBuffResponse {
            ErrorCode: Some(ErrorCode::ErrOrderApplyBuffFailed),
        };
    }
    receive_data_pack.extend(pack);
    OrderApplyBuffResponse {
        ErrorCode: Some(ErrorCode::Success),
    }
}

pub fn handle_apply_gameplay_effect_request(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    receive_data_pack: &mut Vec<CombatReceiveData>,
    req: ApplyGameplayEffectRequest,
) -> ApplyGameplayEffectResponse {
    let Some(entity_id) = common_entity_id(common) else {
        return ApplyGameplayEffectResponse {
            ErrorCode: Some(ErrorCode::ErrOrderApplyBuffFailed),
        };
    };
    let duration_seconds = match req.Time {
        Some(ApplyGameplayEffectRequestTimeOneof::Duration(d)) => match validate_duration(d) {
            Ok(v) => Some(v),
            Err(()) => {
                return ApplyGameplayEffectResponse {
                    ErrorCode: Some(ErrorCode::ErrOrderApplyBuffFailed),
                };
            }
        },
        None => None,
    };
    let mut pack = Vec::new();
    let ok = apply_buff_request_impl(
        state,
        events,
        entity_id,
        req.Id,
        req.InstigatorId,
        req.StackCount,
        req.IsActive,
        duration_seconds,
        true,
        &mut pack,
    );
    if !ok {
        return ApplyGameplayEffectResponse {
            ErrorCode: Some(ErrorCode::ErrOrderApplyBuffFailed),
        };
    }
    receive_data_pack.extend(pack);
    ApplyGameplayEffectResponse {
        ErrorCode: Some(ErrorCode::Success),
    }
}

pub fn handle_apply_gameplay_effect_push(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    receive_data_pack: &mut Vec<CombatReceiveData>,
    push: ApplyGameplayEffectPush,
) {
    let Some(entity_id) = common_entity_id(common) else {
        return;
    };
    let duration_seconds = match push.Time {
        Some(ApplyGameplayEffectPushTimeOneof::Duration(d)) => match validate_duration(d) {
            Ok(v) => Some(v),
            Err(()) => return,
        },
        None => None,
    };
    let stack_count = if push.StackCount > 0 {
        push.StackCount
    } else {
        1
    };
    let _ = apply_buff_request_impl(
        state,
        events,
        entity_id,
        push.Id,
        push.InstigatorId,
        stack_count,
        push.IsActive,
        duration_seconds,
        false,
        receive_data_pack,
    );
}

fn buff_has_any_tag(assets: &Assets, granted_tags: &[String], target_tag_ids: &[i32]) -> bool {
    for tag_name in granted_tags {
        let Some(granted_id) =
            gameplay_tags::id_from_name(&assets.tables.gameplay_tag_remap, tag_name)
        else {
            continue;
        };
        if target_tag_ids
            .iter()
            .any(|&target_id| gameplay_tags::contains(&assets.tables.gameplay_tag_parent, granted_id, target_id))
        {
            return true;
        }
    }
    false
}

fn append_stack_count_notify_in_pack(
    receive_data_pack: &mut Vec<CombatReceiveData>,
    entity_id: i64,
    handle_id: i32,
    new_stack_count: i32,
    instigator_id: i64,
    duration: f32,
) {
    receive_data_pack.push(CombatReceiveData {
        Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
            CombatNotifyData {
                CombatCommon: Some(CombatCommon {
                    EntityId: entity_id,
                    ..Default::default()
                }),
                Message: Some(CombatNotifyDataMessageOneof::BuffStackCountNotify(
                    BuffStackCountNotify {
                        HandleId: handle_id,
                        NewStackCount: new_stack_count,
                        InstigatorId: instigator_id,
                        NotRefreshDuration: duration <= 0.0,
                        NotRefreshPeriod: duration <= 0.0,
                        Time: (duration > 0.0).then_some(BuffStackCountNotifyTimeOneof::Duration(duration)),
                        gFs: (duration > 0.0).then_some(BuffStackCountNotifyGFsOneof::LeftDuration(duration)),
                    },
                )),
            },
        )),
    });
}

#[allow(clippy::too_many_arguments)]
fn remove_buff_handle_in_pack(
    entity_id: i64,
    handle_id: i32,
    instigator_id: i64,
    is_premature: bool,
    scene: &mut Scene,
    assets: &Assets,
    buff_timers: &mut BuffTimerScheduler,
    events: &mut EventQueue,
    receive_data_pack: &mut Vec<CombatReceiveData>,
) {
    let Some(entity) = scene.entity_handle(entity_id) else {
        return;
    };
    let (buff_id, orig_instigator) = {
        let storage = &mut scene.entities[entity.index];
        let (Some(buffs), maybe_attr) = (storage.buffs.as_mut(), storage.attribute.as_mut()) else {
            return;
        };
        let Some((buff_id, orig_instigator)) = buffs
            .find_by_handle(handle_id)
            .map(|b| (b.BuffId, b.InstigatorId))
        else {
            return;
        };
        buff_helper::append_shield_del_for_handle(
            receive_data_pack,
            entity.net_id,
            buffs,
            maybe_attr,
            handle_id,
        );
        buff_timers.cancel_handle(entity.net_id, handle_id);
        buffs.remove_by_handle(handle_id);
        (buff_id, orig_instigator)
    };

    receive_data_pack.push(CombatReceiveData {
        Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
            CombatNotifyData {
                CombatCommon: Some(CombatCommon {
                    EntityId: entity.net_id,
                    ..Default::default()
                }),
                Message: Some(CombatNotifyDataMessageOneof::RemoveGameplayEffectNotify(
                    RemoveGameplayEffectNotify {
                        EntityId: entity.net_id,
                        Handle: handle_id,
                        InstigatorId: instigator_id,
                    },
                )),
            },
        )),
    });

    if is_premature && let Some(buff_data) = assets.tables.buff.get_data_by_id(&buff_id) {
        let follow_ups: Vec<BuffAdditionEntry> = buff_data
            .PrematureExpirationEffects
            .iter()
            .copied()
            .filter(|&fid| fid != buff_id)
            .map(|fid| BuffAdditionEntry {
                id: fid,
                is_active: true,
                ..Default::default()
            })
            .collect();
        if !follow_ups.is_empty() {
            let inst_entity = if orig_instigator != 0 {
                scene.entity_handle(orig_instigator).unwrap_or(entity)
            } else {
                entity
            };
            events.enqueue(Event::BuffAddition {
                target: entity,
                instigator: inst_entity,
                buffs: follow_ups,
            });
        }
    }

    if scene.entities[entity.index].attribute.is_some() {
        buff_helper::sync_continuous_buff_modifiers(receive_data_pack, scene, entity.net_id, assets);
    }
    events.enqueue(Event::BuffChange {
        player_id: scene.player_id,
        entity,
        buff_id: 0,
        stack_count: 0,
        is_add: false,
        instigator: entity,
    });
}

pub fn handle_order_remove_buff_request(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    receive_data_pack: &mut Vec<CombatReceiveData>,
    req: OrderRemoveBuffRequest,
) -> OrderRemoveBuffResponse {
    let Some(entity_id) = common_entity_id(common) else {
        return OrderRemoveBuffResponse {
            ErrorCode: Some(ErrorCode::ErrOrderRemoveBuffEntityNotExists),
        };
    };
    let Some(scene) = state.scene.as_mut() else {
        return OrderRemoveBuffResponse {
            ErrorCode: Some(ErrorCode::ErrOrderRemoveBuffEntityNotExists),
        };
    };
    let Some(entity) = scene.entity_handle(entity_id) else {
        return OrderRemoveBuffResponse {
            ErrorCode: Some(ErrorCode::ErrOrderRemoveBuffEntityNotExists),
        };
    };
    if scene.entities[entity.index].buffs.is_none() {
        return OrderRemoveBuffResponse {
            ErrorCode: Some(ErrorCode::ErrOrderRemoveBuffEntityNotExists),
        };
    }

    let is_none_stacking = state
        .assets
        .tables
        .buff
        .get_data_by_id(&req.Id)
        .is_some_and(|cfg| cfg.StackingType == StackingType::None);
    let remove_count = req.StackCount;

    if is_none_stacking && remove_count <= 0 {
        let handles: Vec<i32> = scene.entities[entity.index]
            .buffs
            .as_ref()
            .map(|b| {
                b.buffs
                    .iter()
                    .filter(|info| info.BuffId == req.Id)
                    .map(|info| info.HandleId)
                    .collect()
            })
            .unwrap_or_default();
        for hid in handles {
            remove_buff_handle_in_pack(
                entity_id,
                hid,
                req.InstigatorId,
                true,
                scene,
                &state.assets,
                &mut state.buff_timers,
                events,
                receive_data_pack,
            );
        }
    } else if let Some((handle_id, cur_stack)) = scene.entities[entity.index]
        .buffs
        .as_ref()
        .and_then(|b| b.get_by_buff_id(req.Id))
        .map(|b| (b.HandleId, b.StackCount))
    {
        if remove_count <= 0 || remove_count >= cur_stack {
            remove_buff_handle_in_pack(
                entity_id,
                handle_id,
                req.InstigatorId,
                true,
                scene,
                &state.assets,
                &mut state.buff_timers,
                events,
                receive_data_pack,
            );
        } else {
            let new_stack = cur_stack - remove_count;
            let buff_snap = if let Some(buffs) = scene.entities[entity.index].buffs.as_mut()
                && let Some(buff) = buffs.find_by_handle_mut(handle_id)
            {
                buff.StackCount = new_stack;
                Some(buff.clone())
            } else {
                None
            };
            append_stack_count_notify_in_pack(
                receive_data_pack,
                entity.net_id,
                handle_id,
                new_stack,
                req.InstigatorId,
                -1.0,
            );
            if let Some(buff_snap) = buff_snap
                && let Some(buff_data) = state.assets.tables.buff.get_data_by_id(&buff_snap.BuffId)
            {
                buff_helper::sync_buff_shield(
                    receive_data_pack,
                    scene,
                    &state.assets,
                    entity.net_id,
                    &buff_snap,
                    &buff_data,
                );
            }
            if scene.entities[entity.index].attribute.is_some() {
                buff_helper::sync_continuous_buff_modifiers(
                    receive_data_pack,
                    scene,
                    entity.net_id,
                    &state.assets,
                );
            }
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

    OrderRemoveBuffResponse {
        ErrorCode: Some(ErrorCode::Success),
    }
}

pub fn handle_order_remove_buff_by_tags_request(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    receive_data_pack: &mut Vec<CombatReceiveData>,
    req: OrderRemoveBuffByTagsRequest,
) -> OrderRemoveBuffByTagsResponse {
    let Some(entity_id) = common_entity_id(common) else {
        return OrderRemoveBuffByTagsResponse {
            ErrorCode: Some(ErrorCode::ErrOrderRemoveBuffEntityNotExists),
        };
    };
    let Some(scene) = state.scene.as_mut() else {
        return OrderRemoveBuffByTagsResponse {
            ErrorCode: Some(ErrorCode::ErrOrderRemoveBuffEntityNotExists),
        };
    };
    let Some(entity) = scene.entity_handle(entity_id) else {
        return OrderRemoveBuffByTagsResponse {
            ErrorCode: Some(ErrorCode::ErrOrderRemoveBuffEntityNotExists),
        };
    };
    let Some(buffs) = scene.entities[entity.index].buffs.as_ref() else {
        return OrderRemoveBuffByTagsResponse {
            ErrorCode: Some(ErrorCode::ErrOrderRemoveBuffEntityNotExists),
        };
    };

    if !req.TagIds.is_empty() && !buffs.buffs.is_empty() {
        let handles_to_remove: Vec<i32> = buffs
            .buffs
            .iter()
            .filter_map(|buff_info| {
                let buff_data = state.assets.tables.buff.get_data_by_id(&buff_info.BuffId)?;
                buff_has_any_tag(&state.assets, &buff_data.GrantedTags, &req.TagIds)
                    .then_some(buff_info.HandleId)
            })
            .collect();
        for handle_id in handles_to_remove {
            remove_buff_handle_in_pack(
                entity_id,
                handle_id,
                req.InstigatorId,
                true,
                scene,
                &state.assets,
                &mut state.buff_timers,
                events,
                receive_data_pack,
            );
        }
    }

    OrderRemoveBuffByTagsResponse {
        ErrorCode: Some(ErrorCode::Success),
    }
}

pub fn handle_remove_gameplay_effect_request(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    receive_data_pack: &mut Vec<CombatReceiveData>,
    req: RemoveGameplayEffectRequest,
) -> RemoveGameplayEffectResponse {
    let entity_id = if req.EntityId != 0 {
        req.EntityId
    } else {
        let Some(eid) = common_entity_id(common) else {
            return RemoveGameplayEffectResponse {
                ErrorCode: Some(ErrorCode::ErrOrderRemoveBuffEntityNotExists),
                Handle: req.Handle,
            };
        };
        eid
    };
    let Some(scene) = state.scene.as_mut() else {
        return RemoveGameplayEffectResponse {
            ErrorCode: Some(ErrorCode::ErrOrderRemoveBuffEntityNotExists),
            Handle: req.Handle,
        };
    };
    let Some(entity) = scene.entity_handle(entity_id) else {
        return RemoveGameplayEffectResponse {
            ErrorCode: Some(ErrorCode::ErrOrderRemoveBuffEntityNotExists),
            Handle: req.Handle,
        };
    };
    if scene.entities[entity.index].buffs.is_none() {
        return RemoveGameplayEffectResponse {
            ErrorCode: Some(ErrorCode::ErrOrderRemoveBuffEntityNotExists),
            Handle: req.Handle,
        };
    }

    remove_buff_handle_in_pack(
        entity_id,
        req.Handle,
        0,
        req.IsPrematureRemoval,
        scene,
        &state.assets,
        &mut state.buff_timers,
        events,
        receive_data_pack,
    );
    RemoveGameplayEffectResponse {
        ErrorCode: Some(ErrorCode::Success),
        Handle: req.Handle,
    }
}

pub fn handle_remove_gameplay_effect_push(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    receive_data_pack: &mut Vec<CombatReceiveData>,
    push: RemoveGameplayEffectPush,
) {
    let entity_id = if push.EntityId != 0 {
        push.EntityId
    } else {
        let Some(eid) = common_entity_id(common) else {
            return;
        };
        eid
    };
    let Some(scene) = state.scene.as_mut() else {
        return;
    };
    remove_buff_handle_in_pack(
        entity_id,
        push.Handle,
        push.InstigatorId,
        push.IsPrematureRemoval,
        scene,
        &state.assets,
        &mut state.buff_timers,
        events,
        receive_data_pack,
    );
}

fn set_buff_active_impl(
    state: &mut State,
    events: &mut EventQueue,
    entity_id: i64,
    handle: i32,
    on: bool,
    receive_data_pack: &mut Vec<CombatReceiveData>,
) {
    let Some(scene) = state.scene.as_mut() else {
        return;
    };
    let Some(entity) = scene.entity_handle(entity_id) else {
        return;
    };
    let Some(buff_snap) = scene.entities[entity.index].buffs.as_mut().and_then(|buffs| {
        let buff = buffs.find_by_handle_mut(handle)?;
        if buff.IsActive == on {
            return None;
        }
        buff.IsActive = on;
        Some(buff.clone())
    }) else {
        return;
    };

    receive_data_pack.push(CombatReceiveData {
        Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
            CombatNotifyData {
                CombatCommon: Some(CombatCommon {
                    EntityId: entity.net_id,
                    ..Default::default()
                }),
                Message: Some(CombatNotifyDataMessageOneof::ActivateBuffNotify(
                    ActivateBuffNotify {
                        Handle: handle,
                        On: on,
                    },
                )),
            },
        )),
    });
    if let Some(buff_data) = state.assets.tables.buff.get_data_by_id(&buff_snap.BuffId) {
        buff_helper::sync_buff_shield(
            receive_data_pack,
            scene,
            &state.assets,
            entity.net_id,
            &buff_snap,
            &buff_data,
        );
    }
    if scene.entities[entity.index].attribute.is_some() {
        buff_helper::sync_continuous_buff_modifiers(
            receive_data_pack,
            scene,
            entity.net_id,
            &state.assets,
        );
    }
    events.enqueue(Event::BuffChange {
        player_id: scene.player_id,
        entity,
        buff_id: 0,
        stack_count: 0,
        is_add: false,
        instigator: entity,
    });
}

pub fn handle_activate_buff_request(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    receive_data_pack: &mut Vec<CombatReceiveData>,
    req: ActivateBuffRequest,
) -> ActivateBuffResponse {
    if let Some(entity_id) = common_entity_id(common) {
        set_buff_active_impl(state, events, entity_id, req.Handle, req.On, receive_data_pack);
    }
    ActivateBuffResponse {
        ErrorCode: Some(ErrorCode::Success),
    }
}

pub fn handle_active_buff_push(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    receive_data_pack: &mut Vec<CombatReceiveData>,
    push: ActiveBuffPush,
) {
    if let Some(entity_id) = common_entity_id(common) {
        set_buff_active_impl(state, events, entity_id, push.Handle, push.On, receive_data_pack);
    }
}

#[allow(clippy::too_many_arguments)]
fn set_buff_stack_count_impl(
    state: &mut State,
    events: &mut EventQueue,
    entity_id: i64,
    handle_id: i32,
    new_stack_count: i32,
    instigator_id: i64,
    is_premature: bool,
    not_refresh_duration: bool,
    duration: f32,
    receive_data_pack: &mut Vec<CombatReceiveData>,
) {
    let Some(scene) = state.scene.as_mut() else {
        return;
    };
    let Some(entity) = scene.entity_handle(entity_id) else {
        return;
    };
    if scene.entities[entity.index].buffs.is_none() {
        return;
    }

    if new_stack_count <= 0 {
        remove_buff_handle_in_pack(
            entity_id,
            handle_id,
            instigator_id,
            is_premature,
            scene,
            &state.assets,
            &mut state.buff_timers,
            events,
            receive_data_pack,
        );
        return;
    }

    let Some((buff_id, cur_stack)) = scene.entities[entity.index]
        .buffs
        .as_ref()
        .and_then(|b| b.find_by_handle(handle_id))
        .map(|b| (b.BuffId, b.StackCount))
    else {
        return;
    };

    let maybe_cfg = state
        .assets
        .tables
        .buff
        .get_data_by_id(&buff_id)
        .map(|b| b.into_owned());
    let mut final_count = new_stack_count;

    if let Some(ref buff_data) = maybe_cfg
        && buff_data.StackLimitCount > 0
    {
        let limit = buff_data.StackLimitCount.min(i32::MAX as u32) as i32;
        if new_stack_count > limit {
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
                let inst = if instigator_id != 0 {
                    scene.entity_handle(instigator_id).unwrap_or(entity)
                } else {
                    entity
                };
                events.enqueue(Event::BuffAddition {
                    target: entity,
                    instigator: inst,
                    buffs: follow_ups,
                });
            }
            if buff_data.bClearStackOnOverflow {
                remove_buff_handle_in_pack(
                    entity_id,
                    handle_id,
                    instigator_id,
                    is_premature,
                    scene,
                    &state.assets,
                    &mut state.buff_timers,
                    events,
                    receive_data_pack,
                );
                return;
            }
            if buff_data.bDenyOverflowApplication && cur_stack >= limit {
                return;
            }
            final_count = limit;
        }
    }

    let buff_snap = if let Some(buffs) = scene.entities[entity.index].buffs.as_mut()
        && let Some(buff) = buffs.find_by_handle_mut(handle_id)
    {
        buff.StackCount = final_count;
        if !not_refresh_duration && duration > 0.0 {
            buff.Duration = duration;
            buff.LeftDuration = duration;
        }
        Some(buff.clone())
    } else {
        None
    };

    let Some(buff_snap) = buff_snap else {
        return;
    };

    if !not_refresh_duration
        && duration > 0.0
        && let Some(ref buff_data) = maybe_cfg
    {
        state.buff_timers.refresh_buff(
            &buff_snap,
            buff_data,
            entity.net_id,
            now_millis(),
            sonoro_gamesv_logic::schedulers::buff_timer_scheduler::ApplicationDisposition {
                refresh_duration: true,
                reset_period: false,
                execute_periodic_now: false,
            },
        );
    }

    append_stack_count_notify_in_pack(
        receive_data_pack,
        entity.net_id,
        handle_id,
        final_count,
        instigator_id,
        if !not_refresh_duration && duration > 0.0 {
            duration
        } else {
            -1.0
        },
    );

    if let Some(ref buff_data) = maybe_cfg {
        buff_helper::sync_buff_shield(
            receive_data_pack,
            scene,
            &state.assets,
            entity.net_id,
            &buff_snap,
            buff_data,
        );
    }
    if scene.entities[entity.index].attribute.is_some() {
        buff_helper::sync_continuous_buff_modifiers(
            receive_data_pack,
            scene,
            entity.net_id,
            &state.assets,
        );
    }
    events.enqueue(Event::BuffChange {
        player_id: scene.player_id,
        entity,
        buff_id: 0,
        stack_count: 0,
        is_add: false,
        instigator: entity,
    });
}

pub fn handle_buff_stack_count_request(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    receive_data_pack: &mut Vec<CombatReceiveData>,
    req: BuffStackCountRequest,
) -> BuffStackCountResponse {
    if let Some(entity_id) = common_entity_id(common) {
        set_buff_stack_count_impl(
            state,
            events,
            entity_id,
            req.HandleId,
            req.NewStackCount,
            req.InstigatorId,
            req.IsPrematureRemoval,
            true,
            -1.0,
            receive_data_pack,
        );
    }
    BuffStackCountResponse {
        ErrorCode: Some(ErrorCode::Success),
    }
}

pub fn handle_buff_stack_count_push(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    receive_data_pack: &mut Vec<CombatReceiveData>,
    push: BuffStackCountPush,
) {
    let Some(entity_id) = common_entity_id(common) else {
        return;
    };
    set_buff_stack_count_impl(
        state,
        events,
        entity_id,
        push.HandleId,
        push.NewStackCount,
        push.InstigatorId,
        push.IsPrematureRemoval,
        push.NotRefreshDuration,
        push.Duration,
        receive_data_pack,
    );
}

pub fn handle_refresh_buff_duration_push(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    receive_data_pack: &mut Vec<CombatReceiveData>,
    push: RefreshBuffDurationPush,
) {
    let Some(entity_id) = common_entity_id(common) else {
        return;
    };
    let Some(scene) = state.scene.as_mut() else {
        return;
    };
    let Some(entity) = scene.entity_handle(entity_id) else {
        return;
    };
    let Some(buffs) = scene.entities[entity.index].buffs.as_mut() else {
        return;
    };
    if push.BuffIds.is_empty() || buffs.buffs.is_empty() {
        return;
    }

    let now_ms = now_millis();
    let mut any_changed = false;

    for buff in &mut buffs.buffs {
        if !push.BuffIds.contains(&buff.BuffId) {
            continue;
        }
        let Some(buff_data) = state.assets.tables.buff.get_data_by_id(&buff.BuffId) else {
            continue;
        };
        if buff_data.DurationPolicy != DurationPolicy::HasDuration {
            continue;
        }
        let dur = if buff.Duration > 0.0 {
            buff.Duration
        } else {
            buff_helper::configured_duration_seconds(&buff_data)
        };
        if dur <= 0.0 {
            continue;
        }
        buff.Duration = dur;
        buff.LeftDuration = dur;
        state.buff_timers.refresh_buff(
            buff,
            &buff_data,
            entity_id,
            now_ms,
            sonoro_gamesv_logic::schedulers::buff_timer_scheduler::ApplicationDisposition {
                refresh_duration: true,
                reset_period: false,
                execute_periodic_now: false,
            },
        );
        receive_data_pack.push(CombatReceiveData {
            Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
                CombatNotifyData {
                    CombatCommon: Some(CombatCommon {
                        EntityId: entity_id,
                        ..Default::default()
                    }),
                    Message: Some(CombatNotifyDataMessageOneof::BuffDurationNotify(
                        BuffDurationNotify {
                            HandleId: buff.HandleId,
                            Time: Some(BuffDurationNotifyTimeOneof::Duration(dur)),
                            gFs: Some(BuffDurationNotifyGFsOneof::LeftDuration(dur)),
                        },
                    )),
                },
            )),
        });
        any_changed = true;
    }

    if any_changed {
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

fn handle_passive_skill_add(
    state: &mut State,
    target_entity_id: i64,
    passive_skill_id: i64,
    receive_data_pack: &mut Vec<CombatReceiveData>,
) {
    let Ok(skill_id) = i32::try_from(passive_skill_id) else {
        return;
    };
    if skill_id == 0 || target_entity_id == 0 {
        return;
    }
    let Some(scene) = state.scene.as_mut() else {
        return;
    };
    let Some(&idx) = scene.net_id_map.get(&target_entity_id) else {
        return;
    };
    let Some(passive) = scene.entities[idx].passive_ga_skill.as_mut() else {
        return;
    };
    if !passive.add_skill(skill_id) {
        return;
    }

    receive_data_pack.push(CombatReceiveData {
        Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
            CombatNotifyData {
                CombatCommon: Some(CombatCommon {
                    EntityId: target_entity_id,
                    MessageId: -1,
                    PreMessageId: -1,
                    ..Default::default()
                }),
                Message: Some(CombatNotifyDataMessageOneof::PassiveSkillAddNotify(
                    PassiveSkillAddNotify {
                        EntityId: target_entity_id,
                        PassiveSkillItemPbList: vec![PassiveSkillItemPb {
                            CombatCommon: Some(CombatCommon {
                                EntityId: target_entity_id,
                                MessageId: -1,
                                PreMessageId: -1,
                                ..Default::default()
                            }),
                            SkillId: passive_skill_id,
                        }],
                    },
                )),
            },
        )),
    });
}

fn handle_passive_skill_remove(
    state: &mut State,
    target_entity_id: i64,
    passive_skill_id: i64,
    receive_data_pack: &mut Vec<CombatReceiveData>,
) {
    let Ok(skill_id) = i32::try_from(passive_skill_id) else {
        return;
    };
    if skill_id == 0 || target_entity_id == 0 {
        return;
    }
    let Some(scene) = state.scene.as_mut() else {
        return;
    };
    let Some(&idx) = scene.net_id_map.get(&target_entity_id) else {
        return;
    };
    let Some(passive) = scene.entities[idx].passive_ga_skill.as_mut() else {
        return;
    };
    if !passive.remove_skill(skill_id) {
        return;
    }

    receive_data_pack.push(CombatReceiveData {
        Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
            CombatNotifyData {
                CombatCommon: Some(CombatCommon {
                    EntityId: target_entity_id,
                    ..Default::default()
                }),
                Message: Some(CombatNotifyDataMessageOneof::PassiveSkillRemoveNotify(
                    PassiveSkillRemoveNotify {
                        EntityId: target_entity_id,
                        SkillIdList: vec![passive_skill_id],
                    },
                )),
            },
        )),
    });
}

pub fn handle_passive_skill_add_request(
    state: &mut State,
    common: Option<&CombatCommon>,
    receive_data_pack: &mut Vec<CombatReceiveData>,
    req: PassiveSkillAddRequest,
) -> PassiveSkillAddResponse {
    let tid = if req.TargetEntityId != 0 {
        req.TargetEntityId
    } else {
        common_entity_id(common).unwrap_or(0)
    };
    handle_passive_skill_add(state, tid, req.PassiveSkillId, receive_data_pack);
    PassiveSkillAddResponse {
        ErrorCode: Some(ErrorCode::Success),
    }
}

pub fn handle_passive_skill_add_push(
    state: &mut State,
    common: Option<&CombatCommon>,
    receive_data_pack: &mut Vec<CombatReceiveData>,
    push: PassiveSkillAddPush,
) {
    let tid = if push.TargetEntityId != 0 {
        push.TargetEntityId
    } else {
        common_entity_id(common).unwrap_or(0)
    };
    handle_passive_skill_add(state, tid, push.PassiveSkillId, receive_data_pack);
}

pub fn handle_passive_skill_remove_request(
    state: &mut State,
    common: Option<&CombatCommon>,
    receive_data_pack: &mut Vec<CombatReceiveData>,
    req: PassiveSkillRemoveRequest,
) -> PassiveSkillRemoveResponse {
    let tid = if req.TargetEntityId != 0 {
        req.TargetEntityId
    } else {
        common_entity_id(common).unwrap_or(0)
    };
    handle_passive_skill_remove(state, tid, req.PassiveSkillId, receive_data_pack);
    PassiveSkillRemoveResponse {
        ErrorCode: Some(ErrorCode::Success),
    }
}

pub fn handle_passive_skill_remove_push(
    state: &mut State,
    common: Option<&CombatCommon>,
    receive_data_pack: &mut Vec<CombatReceiveData>,
    push: PassiveSkillRemovePush,
) {
    let tid = if push.TargetEntityId != 0 {
        push.TargetEntityId
    } else {
        common_entity_id(common).unwrap_or(0)
    };
    handle_passive_skill_remove(state, tid, push.PassiveSkillId, receive_data_pack);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requested_buff_duration_validates_explicit_client_values() {
        assert_eq!(request_duration(None), Ok(None));
        assert_eq!(
            request_duration(Some(&OrderApplyBuffRequestTimeOneof::Duration(1.5))),
            Ok(Some(1.5))
        );
        assert_eq!(
            request_duration(Some(&OrderApplyBuffRequestTimeOneof::Duration(0.0))),
            Ok(Some(0.0))
        );
        assert_eq!(
            request_duration(Some(&OrderApplyBuffRequestTimeOneof::Duration(-1.0))),
            Ok(Some(-1.0))
        );

        assert!(request_duration(Some(&OrderApplyBuffRequestTimeOneof::Duration(f32::NAN))).is_err());
        assert!(request_duration(Some(&OrderApplyBuffRequestTimeOneof::Duration(f32::INFINITY))).is_err());
        assert!(request_duration(Some(&OrderApplyBuffRequestTimeOneof::Duration(f32::NEG_INFINITY))).is_err());
        assert!(request_duration(Some(&OrderApplyBuffRequestTimeOneof::Duration(f32::MAX))).is_err());
    }

    #[test]
    fn instant_apply_gameplay_effect_push_does_not_echo_apply_notify() {
        let mut h = crate::audit_tests::TestHarness::new("instant_push_no_echo", Some(8));
        let mut events = EventQueue::new();
        let entity_id = h
            .state
            .scene
            .as_ref()
            .unwrap()
            .current_role_entity_id();
        let common = CombatCommon {
            EntityId: entity_id,
            ..Default::default()
        };

        // 1512310011 is Instant + SharedByFormation; push must execute effects without echoing
        // ApplyGameplayEffectNotify { Handle: -2 } back to the originating client.
        let mut push_pack = Vec::new();
        handle_apply_gameplay_effect_push(
            &mut h.state,
            &mut events,
            Some(&common),
            &mut push_pack,
            ApplyGameplayEffectPush {
                Id: 1512310011,
                InstigatorId: entity_id,
                IsActive: true,
                StackCount: 1,
                ..Default::default()
            },
        );
        assert!(
            !push_pack.iter().any(|entry| matches!(
                entry.Message.as_ref(),
                Some(CombatReceiveDataMessageOneof::CombatNotifyData(notify))
                    if matches!(
                        notify.Message.as_ref(),
                        Some(CombatNotifyDataMessageOneof::ApplyGameplayEffectNotify(_))
                    )
            )),
            "ApplyGameplayEffectPush for Instant buff must not echo ApplyGameplayEffectNotify"
        );

        let mut req_pack = Vec::new();
        let resp = handle_apply_gameplay_effect_request(
            &mut h.state,
            &mut events,
            Some(&common),
            &mut req_pack,
            ApplyGameplayEffectRequest {
                Id: 1512310011,
                InstigatorId: entity_id,
                IsActive: true,
                StackCount: 1,
                ..Default::default()
            },
        );
        assert_eq!(resp.ErrorCode, Some(ErrorCode::Success));
        assert!(req_pack.iter().any(|entry| matches!(
            entry.Message.as_ref(),
            Some(CombatReceiveDataMessageOneof::CombatNotifyData(notify))
                if matches!(
                    notify.Message.as_ref(),
                    Some(CombatNotifyDataMessageOneof::ApplyGameplayEffectNotify(_))
                )
        )));
    }
}
