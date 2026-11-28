use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::helpers::pb_kv;
use sonoro_gamesv_logic::scene::now_millis;
use sonoro_proto::enums::ErrorCode;
use sonoro_proto::pb::{
    BlackboardParamValueOneof, CombatCommon, CombatNotifyData, CombatNotifyDataMessageOneof,
    CombatPushDataMessageOneof, CombatReceiveData, CombatReceiveDataMessageOneof,
    CombatReceivePackNotify, CombatRequestDataMessageOneof, CombatResponseData,
    CombatResponseDataMessageOneof, CombatSendDataMessageOneof, CombatSendPackRequest,
    CombatSendPackResponse, EntityBlackboardNotify, EntityRemoveNotify,
};

use super::{State, attach, buff, combat_envelope_handlers, damage, fsm, skill};

pub fn push_combat_notify(
    receive_data_pack: &mut Vec<CombatReceiveData>,
    common: Option<CombatCommon>,
    msg: CombatNotifyDataMessageOneof,
) {
    let mut resolved = common.unwrap_or_default();
    if resolved.MessageId == 0 {
        resolved.MessageId = -1;
    }
    if resolved.PreMessageId == 0 {
        resolved.PreMessageId = -1;
    }
    receive_data_pack.push(CombatReceiveData {
        Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
            CombatNotifyData {
                CombatCommon: Some(resolved),
                Message: Some(msg),
            },
        )),
    });
}

fn push_combat_response(
    receive_data_pack: &mut Vec<CombatReceiveData>,
    request_id: i32,
    common: Option<CombatCommon>,
    msg: CombatResponseDataMessageOneof,
) {
    receive_data_pack.push(CombatReceiveData {
        Message: Some(CombatReceiveDataMessageOneof::CombatResponseData(
            CombatResponseData {
                Message: Some(msg),
                CombatCommon: common,
                RequestId: request_id,
            },
        )),
    });
}

pub(super) fn dispatch_combat_push(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<CombatCommon>,
    push: CombatPushDataMessageOneof,
    pack: &mut Vec<CombatReceiveData>,
) {
    handle_combat_push(state, events, common, push, pack);
}

pub(super) fn dispatch_combat_request(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<CombatCommon>,
    req: CombatRequestDataMessageOneof,
    pack: &mut Vec<CombatReceiveData>,
) -> Option<CombatResponseDataMessageOneof> {
    handle_combat_request_inner(state, events, common, req, pack)
}

fn handle_combat_push(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<CombatCommon>,
    push: CombatPushDataMessageOneof,
    pack: &mut Vec<CombatReceiveData>,
) {
    match push {
        CombatPushDataMessageOneof::EndSkillPush(p) => {
            skill::handle_end_skill_push(state, common, p);
        }
        CombatPushDataMessageOneof::ExecuteQtePush(p) => {
            combat_envelope_handlers::handle_execute_qte_push(p, common, pack);
        }
        CombatPushDataMessageOneof::MotorSummonAndRidePush(p) => {
            combat_envelope_handlers::handle_motor_summon_and_ride_push(state, p, common, pack);
        }
        CombatPushDataMessageOneof::AiBlackboardCdPush(p) => {
            combat_envelope_handlers::handle_ai_blackboard_cd_push(p, common, pack);
        }
        CombatPushDataMessageOneof::DrownEndTeleportPush(p) => {
            combat_envelope_handlers::handle_drown_end_teleport_push(state, events, p);
        }
        CombatPushDataMessageOneof::RTimeStopPush(p) => {
            if let Some(scene) = state.scene.as_mut() {
                scene.scene_time.set_dilation(now_millis(), f64::from(p.Dilation));
            }
        }
        CombatPushDataMessageOneof::ClientCurrentRoleReportPush(p) => {
            super::role::handle_client_current_role_report_push(state, p);
        }
        CombatPushDataMessageOneof::HitEndPush(p) => {
            damage::handle_hit_end_push(state, events, p);
        }
        CombatPushDataMessageOneof::DrownPush(p) => {
            damage::handle_drown_push(state, events, common.as_ref(), pack, p);
        }
        CombatPushDataMessageOneof::MonsterDrownPush(p) => {
            damage::handle_monster_drown_push(state, events, common.as_ref(), pack, p);
        }
        CombatPushDataMessageOneof::MonsterBoomPush(p) => {
            damage::handle_monster_boom_push(state, events, common.as_ref(), pack, p);
        }
        CombatPushDataMessageOneof::RoleGoDownPush(p) => {
            damage::handle_role_go_down_push(state, events, common.as_ref(), p);
        }
        CombatPushDataMessageOneof::ApplyGameplayEffectPush(p) => {
            buff::handle_apply_gameplay_effect_push(state, events, common.as_ref(), pack, p);
        }
        CombatPushDataMessageOneof::RemoveGameplayEffectPush(p) => {
            buff::handle_remove_gameplay_effect_push(state, events, common.as_ref(), pack, p);
        }
        CombatPushDataMessageOneof::ActiveBuffPush(p) => {
            buff::handle_active_buff_push(state, events, common.as_ref(), pack, p);
        }
        CombatPushDataMessageOneof::BuffStackCountPush(p) => {
            buff::handle_buff_stack_count_push(state, events, common.as_ref(), pack, p);
        }
        CombatPushDataMessageOneof::RefreshBuffDurationPush(p) => {
            buff::handle_refresh_buff_duration_push(state, events, common.as_ref(), pack, p);
        }
        CombatPushDataMessageOneof::PassiveSkillAddPush(p) => {
            buff::handle_passive_skill_add_push(state, common.as_ref(), pack, p);
        }
        CombatPushDataMessageOneof::PassiveSkillRemovePush(p) => {
            buff::handle_passive_skill_remove_push(state, common.as_ref(), pack, p);
        }
        CombatPushDataMessageOneof::SwitchCharacterStatePush(p) => {
            fsm::handle_switch_character_state_push(state, events, common.as_ref(), pack, p);
        }
        CombatPushDataMessageOneof::EntityIsVisiblePush(p) => {
            fsm::handle_entity_is_visible_push(state, common.as_ref(), pack, p);
        }
        CombatPushDataMessageOneof::ActorVisiblePush(p) => {
            fsm::handle_actor_visible_push(state, common.as_ref(), pack, p);
        }
        CombatPushDataMessageOneof::PartUpdatePush(p) => {
            fsm::handle_part_update_push(state, events, common.as_ref(), pack, p);
        }
        CombatPushDataMessageOneof::AnimationGameplayTagPush(p) => {
            fsm::handle_animation_gameplay_tag_push(state, events, common.as_ref(), pack, p);
        }
        CombatPushDataMessageOneof::RoleTagChangePush(p) => {
            fsm::handle_role_tag_change_push(state, events, common.as_ref(), pack, p);
        }
        CombatPushDataMessageOneof::ANStartPush(p) => {
            fsm::handle_an_start_push(common.as_ref(), p);
        }
        CombatPushDataMessageOneof::MontagePlayPush(p) => {
            if let Some(eid) = common.as_ref().map(|c| c.EntityId).filter(|&id| id != 0) {
                let mname = if p.Name.is_empty() {
                    format!("hash:{}", p.Path)
                } else {
                    String::from_utf8_lossy(&p.Name).into_owned()
                };
                fsm::apply_montage_to_entity_fsm(state, events, eid, &mname, pack);
            }
            fsm::handle_montage_play_push(common.as_ref(), p);
        }
        CombatPushDataMessageOneof::FsmPlayMontagePush(p) => {
            if let Some(eid) = common.as_ref().map(|c| c.EntityId).filter(|&id| id != 0) {
                let mname = if p.MontageName.is_empty() {
                    format!("hash:{}", p.MontagePathHash)
                } else {
                    String::from_utf8_lossy(&p.MontageName).into_owned()
                };
                fsm::apply_montage_to_entity_fsm(state, events, eid, &mname, pack);
            }
            fsm::handle_fsm_play_montage_push(common.as_ref(), p);
        }
        CombatPushDataMessageOneof::FsmConditionPassPush(p) => {
            fsm::handle_fsm_condition_pass_push(state, events, common.as_ref(), pack, p);
        }
        CombatPushDataMessageOneof::AiHatePush(p) => {
            fsm::handle_ai_hate_push(state, events, common.as_ref(), pack, p);
        }
        CombatPushDataMessageOneof::AiInformationPush(p) => {
            fsm::handle_ai_information_push(state, events, common.as_ref(), pack, p);
        }
        CombatPushDataMessageOneof::BattleStateChangePush(p) => {
            fsm::handle_battle_state_change_push(state, events, common.as_ref(), pack, p);
        }
        CombatPushDataMessageOneof::ChangeStateConfirmPush(p) => {
            fsm::handle_change_state_confirm_push(state, events, common.as_ref(), pack, p);
        }
        CombatPushDataMessageOneof::LogicStateInitPush(p) => {
            fsm::handle_logic_state_init_push(state, events, common.as_ref(), pack, p);
        }
        CombatPushDataMessageOneof::SwitchLogicStatePush(p) => {
            fsm::handle_switch_logic_state_push(state, events, common.as_ref(), pack, p);
        }
        CombatPushDataMessageOneof::AiBlackboardsPush(p) => {
            let eid = common.as_ref().map_or(0, |c| c.EntityId);
            let cfg_id = if eid != 0 {
                state
                    .scene
                    .as_ref()
                    .and_then(|sc| sc.find_config_id_by_net_id(eid))
                    .map_or(eid, i64::from)
            } else {
                0
            };
            let info = &mut state.player_components.progress.info;
            let mut dirty = false;
            for param in &p.AiBlackboards {
                if param.Key.is_empty() {
                    continue;
                }
                let pkey = String::from_utf8_lossy(&param.Key);
                let full_k = format!("ent_bb:{cfg_id}:{pkey}");
                match &param.Value {
                    Some(BlackboardParamValueOneof::IntValue(v)) => {
                        info.set_key_int(&full_k, i64::from(*v));
                        dirty = true;
                    }
                    Some(BlackboardParamValueOneof::LongValue(v)) => {
                        info.set_key_int(&full_k, *v);
                        dirty = true;
                    }
                    Some(BlackboardParamValueOneof::BooleanValue(v)) => {
                        info.set_key_int(&full_k, i64::from(*v));
                        dirty = true;
                    }
                    Some(BlackboardParamValueOneof::StringValue(s)) => {
                        info.set_key_str(&full_k, &String::from_utf8_lossy(s));
                        dirty = true;
                    }
                    Some(BlackboardParamValueOneof::FloatValue(f)) if f.is_finite() => {
                        info.set_key_int(&full_k, (*f * 1000.0) as i64);
                        dirty = true;
                    }
                    _ => {}
                }
            }
            if dirty {
                let _ = state.player_components.progress.save(&state.fs);
            }
            if let Some(coop) = state.coop_service.as_ref()
                && !p.AiBlackboards.is_empty()
            {
                coop.broadcast_room_except(
                    state.player_id,
                    EntityBlackboardNotify::MSG_ID,
                    &EntityBlackboardNotify {
                        EntityId: eid,
                        Params: p.AiBlackboards,
                    },
                );
            }
        }
        CombatPushDataMessageOneof::TsAnimNotifyStateAbsoluteTimeStopPush(p) => {
            if let Some(scene) = state.scene.as_mut() {
                let dilation = if p.Duration <= 0 || p.Dilation >= 100 {
                    1.0
                } else {
                    (f64::from(p.Dilation) / 100.0).clamp(0.0, 10.0)
                };
                scene.scene_time.set_dilation(now_millis(), dilation);
            }
        }
        CombatPushDataMessageOneof::RTimeStopInstPush(p) => {
            if let Some(scene) = state.scene.as_mut() {
                let dilation = if p.Flag { 0.0 } else { 1.0 };
                scene.scene_time.set_dilation(now_millis(), dilation);
            }
        }
        CombatPushDataMessageOneof::TriggerExitSkillPush(p) => {
            let key = format!("skill:exit_trig:{}:{}", p.EnterEntityId, p.LeaveEntityId);
            let _ = pb_kv::persist_int(&state.fs, &mut state.player_components.progress, &key, 1);
        }
        CombatPushDataMessageOneof::RoleTrialSceneRollbackPush(_) => {
            let mut qctx = state.quest_context(events);
            let _ = sonoro_gamesv_logic::helpers::quest_engine::events::on_condition_state_changed(
                &mut qctx,
            );
        }
        other => {
            tracing::trace!(target: "combat", "combat push passthrough: {:?}", std::mem::discriminant(&other));
        }
    }
}

fn handle_combat_request(
    state: &mut State,
    events: &mut EventQueue,
    request_id: i32,
    common: Option<CombatCommon>,
    req: CombatRequestDataMessageOneof,
    pack: &mut Vec<CombatReceiveData>,
) {
    if let Some(resp) = handle_combat_request_inner(state, events, common.clone(), req, pack) {
        push_combat_response(pack, request_id, common, resp);
    }
}

fn handle_combat_request_inner(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<CombatCommon>,
    req: CombatRequestDataMessageOneof,
    pack: &mut Vec<CombatReceiveData>,
) -> Option<CombatResponseDataMessageOneof> {
    use CombatRequestDataMessageOneof as Req;
    use CombatResponseDataMessageOneof as Resp;

    let resp: CombatResponseDataMessageOneof = match req {
        Req::SkillRequest(r) => Resp::SkillResponse(skill::handle_skill_request(r)),
        Req::UseSkillRequest(r) => Resp::UseSkillResponse(skill::handle_use_skill_request(
            state,
            events,
            common.clone(),
            r,
        )),
        Req::EndSkillRequest(r) => {
            Resp::EndSkillResponse(skill::handle_end_skill_request(state, common.clone(), r))
        }
        Req::CharacterAttachRequest(r) => Resp::CharacterAttachResponse(
            attach::handle_character_attach_request(state, common.clone(), r, pack),
        ),
        Req::CharacterDetachRequest(r) => Resp::CharacterDetachResponse(
            attach::handle_character_detach_request(state, common.clone(), r, pack),
        ),
        Req::ExecuteQteRequest(r) => Resp::ExecuteQteResponse(
            combat_envelope_handlers::handle_execute_qte_request(r, common.clone(), pack),
        ),
        Req::ChangeSummonEntityStateRequest(r) => Resp::ChangeSummonEntityStateResponse(
            combat_envelope_handlers::handle_change_summon_entity_state_request(state, r),
        ),
        Req::GameplayCueRequest(r) => Resp::GameplayCueResponse(
            combat_envelope_handlers::handle_gameplay_cue_request_with_state(
                state,
                common.clone(),
                pack,
                r,
            ),
        ),
        Req::CaughtRequest(r) => {
            Resp::CaughtResponse(combat_envelope_handlers::handle_caught_request(r))
        }
        Req::MaterialRequest(r) => {
            Resp::MaterialResponse(combat_envelope_handlers::handle_material_request(r))
        }
        Req::BoneVisibleChangeRequest(r) => Resp::BoneVisibleChangeResponse(
            combat_envelope_handlers::handle_bone_visible_change_request(r),
        ),
        Req::ModifyBulletParamsRequest(r) => Resp::ModifyBulletParamsResponse(
            combat_envelope_handlers::handle_modify_bullet_params_request(r),
        ),
        Req::AnimationStateChangedRequest(r) => Resp::AnimationStateChangedResponse(
            combat_envelope_handlers::handle_animation_state_changed_request(r),
        ),
        Req::AnimationStateInitRequest(r) => Resp::AnimationStateInitResponse(
            combat_envelope_handlers::handle_animation_state_init_request(r),
        ),
        Req::EntityStaticHookMoveRequest(r) => Resp::EntityStaticHookMoveResponse(
            combat_envelope_handlers::handle_entity_static_hook_move_request(r),
        ),
        Req::AiBlackboardsRequest(r) => Resp::AiBlackboardsResponse(
            combat_envelope_handlers::handle_ai_blackboards_request_with_state(
                state,
                common.as_ref(),
                r,
            ),
        ),
        Req::AiBlackboardCdRequest(r) => Resp::AiBlackboardCdResponse(
            combat_envelope_handlers::handle_ai_blackboard_cd_request(r),
        ),
        Req::ToughCalcExtraRatioChangeRequest(r) => Resp::ToughCalcExtraRatioChangeResponse(
            combat_envelope_handlers::handle_tough_calc_extra_ratio_change_request(r),
        ),
        Req::UseSkillFailRequest(r) => {
            Resp::UseSkillFailResponse(combat_envelope_handlers::handle_use_skill_fail_request(r))
        }
        Req::EnterViewDirectionRequest(r) => Resp::EnterViewDirectionResponse(
            combat_envelope_handlers::handle_enter_view_direction_request(r),
        ),
        Req::ExitViewDirectionRequest(r) => Resp::ExitViewDirectionResponse(
            combat_envelope_handlers::handle_exit_view_direction_request(r),
        ),
        Req::InterruptSkillInDelayRequest(r) => Resp::InterruptSkillInDelayResponse(
            combat_envelope_handlers::handle_interrupt_skill_in_delay_request(r),
        ),
        Req::TriggerExitSkillRequest(r) => Resp::TriggerExitSkillResponse(
            combat_envelope_handlers::handle_trigger_exit_skill_request(r),
        ),
        Req::BuffEffectRequest(r) => {
            Resp::BuffEffectResponse(combat_envelope_handlers::handle_buff_effect_request(r))
        }
        Req::DrownEndTeleportRequest(r) => Resp::DrownEndTeleportResponse(
            combat_envelope_handlers::handle_drown_end_teleport_request(r),
        ),
        Req::RTimeStopRequest(r) => {
            Resp::RTimeStopResponse(combat_envelope_handlers::handle_r_time_stop_request(r))
        }
        Req::RTimeStopInstRequest(r) => Resp::RTimeStopInstResponse(
            combat_envelope_handlers::handle_r_time_stop_inst_request(r),
        ),
        Req::TsAnimNotifyStateAbsoluteTimeStopRequest(r) => {
            Resp::TsAnimNotifyStateAbsoluteTimeStopResponse(
                combat_envelope_handlers::handle_ts_anim_stop_request(r),
            )
        }
        Req::RoleTagChangeRequest(r) => Resp::RoleTagChangeResponse(
            combat_envelope_handlers::handle_role_tag_change_request_with_state(
                state,
                events,
                common.as_ref(),
                pack,
                r,
            ),
        ),
        Req::CreateBulletRequest(r) => {
            Resp::CreateBulletResponse(combat_envelope_handlers::handle_create_bullet_request(r))
        }
        Req::DestroyBulletRequest(r) => {
            Resp::DestroyBulletResponse(combat_envelope_handlers::handle_destroy_bullet_request(r))
        }
        Req::ANStartRequest(r) => {
            Resp::ANStartResponse(combat_envelope_handlers::handle_an_start_request(r))
        }
        Req::ClientCurrentRoleReportRequest(r) => Resp::ClientCurrentRoleReportResponse(
            combat_envelope_handlers::handle_client_current_role_report_request_with_state(
                state, r,
            ),
        ),
        Req::GaSwitchCommonEnemyProCampRequest(r) => Resp::GaSwitchCommonEnemyProCampResponse(
            combat_envelope_handlers::handle_ga_switch_enemy_camp_request(r),
        ),
        Req::SwitchRoleRequest(r) => {
            Resp::SwitchRoleResponse(super::role::handle_switch_role_request(state, r))
        }
        Req::DamageExecuteRequest(r) => Resp::DamageExecuteResponse(
            damage::handle_damage_execute_request(state, events, pack, r),
        ),
        Req::FragileChangeRequest(r) => Resp::FragileChangeResponse(
            damage::handle_fragile_change_request(state, common.as_ref(), pack, r),
        ),
        Req::DrownRequest(r) => {
            Resp::DrownResponse(damage::handle_drown_request(state, events, common.as_ref(), pack, r))
        }
        Req::MonsterDrownRequest(r) => Resp::MonsterDrownResponse(
            damage::handle_monster_drown_request(state, events, common.as_ref(), pack, r),
        ),
        Req::MonsterBoomRequest(r) => Resp::MonsterBoomResponse(
            damage::handle_monster_boom_request(state, events, common.as_ref(), pack, r),
        ),
        Req::OrderApplyBuffRequest(r) => Resp::OrderApplyBuffResponse(
            buff::handle_order_apply_buff_request(state, events, common.as_ref(), pack, r),
        ),
        Req::ApplyGameplayEffectRequest(r) => Resp::ApplyGameplayEffectResponse(
            buff::handle_apply_gameplay_effect_request(state, events, common.as_ref(), pack, r),
        ),
        Req::OrderRemoveBuffRequest(r) => Resp::OrderRemoveBuffResponse(
            buff::handle_order_remove_buff_request(state, events, common.as_ref(), pack, r),
        ),
        Req::OrderRemoveBuffByTagsRequest(r) => Resp::OrderRemoveBuffByTagsResponse(
            buff::handle_order_remove_buff_by_tags_request(state, events, common.as_ref(), pack, r),
        ),
        Req::RemoveGameplayEffectRequest(r) => Resp::RemoveGameplayEffectResponse(
            buff::handle_remove_gameplay_effect_request(state, events, common.as_ref(), pack, r),
        ),
        Req::ActivateBuffRequest(r) => Resp::ActivateBuffResponse(
            buff::handle_activate_buff_request(state, events, common.as_ref(), pack, r),
        ),
        Req::BuffStackCountRequest(r) => Resp::BuffStackCountResponse(
            buff::handle_buff_stack_count_request(state, events, common.as_ref(), pack, r),
        ),
        Req::PassiveSkillAddRequest(r) => Resp::PassiveSkillAddResponse(
            buff::handle_passive_skill_add_request(state, common.as_ref(), pack, r),
        ),
        Req::PassiveSkillRemoveRequest(r) => Resp::PassiveSkillRemoveResponse(
            buff::handle_passive_skill_remove_request(state, common.as_ref(), pack, r),
        ),
        Req::HitRequest(r) => {
            if let Some(eid) = common.as_ref().map(|c| c.EntityId).filter(|&id| id != 0) {
                fsm::apply_hit_to_entity_fsm(state, events, eid, pack, r.HitInfo.as_ref());
            }
            Resp::HitResponse(fsm::handle_hit_request(common.as_ref(), pack, r))
        }
        Req::HitEndRequest(r) => {
            Resp::HitEndResponse(fsm::handle_hit_end_request(state, events, r))
        }
        Req::SwitchCharacterStateRequest(r) => Resp::SwitchCharacterStateResponse(
            fsm::handle_switch_character_state_request(state, events, common.as_ref(), pack, r),
        ),
        Req::EntityIsVisibleRequest(r) => Resp::EntityIsVisibleResponse(
            fsm::handle_entity_is_visible_request(state, common.as_ref(), pack, r),
        ),
        Req::ActorVisibleRequest(r) => Resp::ActorVisibleResponse(
            fsm::handle_actor_visible_request(state, common.as_ref(), pack, r),
        ),
        Req::PartUpdateRequest(r) => Resp::PartUpdateResponse(fsm::handle_part_update_request(
            state,
            events,
            common.as_ref(),
            pack,
            r,
        )),
        Req::AnimationGameplayTagRequest(r) => Resp::AnimationGameplayTagResponse(
            fsm::handle_animation_gameplay_tag_request(state, events, common.as_ref(), pack, r),
        ),
        Req::ChangeStateRequest(r) => Resp::ChangeStateResponse(fsm::handle_change_state_request(
            state,
            events,
            common.as_ref(),
            pack,
            r,
        )),
        Req::ChangeStateConfirmRequest(r) => Resp::ChangeStateConfirmResponse(
            fsm::handle_change_state_confirm_request(state, events, common.as_ref(), pack, r),
        ),
        Req::FsmConditionPassRequest(r) => Resp::FsmConditionPassResponse(
            fsm::handle_fsm_condition_pass_request(state, events, common.as_ref(), pack, r),
        ),
        Req::FsmStateBehaviorRequest(r) => {
            Resp::FsmStateBehaviorResponse(fsm::handle_fsm_state_behavior_request(
                state,
                common.as_ref(),
                r,
            ))
        }
        Req::FsmPlayMontageRequest(r) => {
            if let Some(eid) = common.as_ref().map(|c| c.EntityId).filter(|&id| id != 0)
                && (!r.MontageName.is_empty() || r.MontagePathHash != 0)
            {
                let mname = if r.MontageName.is_empty() {
                    format!("hash:{}", r.MontagePathHash)
                } else {
                    String::from_utf8_lossy(&r.MontageName).into_owned()
                };
                fsm::apply_montage_to_entity_fsm(state, events, eid, &mname, pack);
            }
            Resp::FsmPlayMontageResponse(fsm::handle_fsm_play_montage_request(common.as_ref(), r))
        }
        Req::AiHateRequest(r) => Resp::AiHateResponse(fsm::handle_ai_hate_request(
            state,
            events,
            common.as_ref(),
            pack,
            r,
        )),
        Req::AiInformationRequest(r) => Resp::AiInformationResponse(
            fsm::handle_ai_information_request(state, events, common.as_ref(), pack, r),
        ),
        Req::LogicStateInitRequest(r) => Resp::LogicStateInitResponse(
            fsm::handle_logic_state_init_request(state, events, common.as_ref(), pack, r),
        ),
        Req::SwitchLogicStateRequest(r) => Resp::SwitchLogicStateResponse(
            fsm::handle_switch_logic_state_request(state, events, common.as_ref(), pack, r),
        ),
        Req::BattleStateChangeRequest(r) => Resp::BattleStateChangeResponse(
            fsm::handle_battle_state_change_request(state, events, common.as_ref(), pack, r),
        ),
        // Protobuf sentinel variant; has no response payload (matches combat.zig:196).
        // Exhaustive match (no `_ =>`) ensures any future CombatRequest variant fails to compile
        // until explicitly handled instead of being dropped silently.
        Req::CombatMaxCaseMessageRequest(_) => return None,
    };

    Some(resp)
}

pub fn handle_combat_send_pack_request(
    state: &mut State,
    events: &mut EventQueue,
    req: CombatSendPackRequest,
) -> Result<CombatSendPackResponse, String> {
    let mut receive_data_pack = Vec::new();
    for send_data in req.Data {
        match send_data.Message {
            Some(CombatSendDataMessageOneof::Push(push)) => {
                if let Some(msg) = push.Message {
                    handle_combat_push(
                        state,
                        events,
                        push.CombatCommon,
                        msg,
                        &mut receive_data_pack,
                    );
                }
            }
            Some(CombatSendDataMessageOneof::Request(sub_req)) => {
                if let Some(msg) = sub_req.Message {
                    handle_combat_request(
                        state,
                        events,
                        sub_req.RequestId,
                        sub_req.CombatCommon,
                        msg,
                        &mut receive_data_pack,
                    );
                }
            }
            None => {}
        }
    }

    if !receive_data_pack.is_empty()
        && let Some(coop) = state.coop_service.as_ref()
    {
        coop.broadcast_room_except(
            state.player_id,
            CombatReceivePackNotify::MSG_ID,
            &CombatReceivePackNotify {
                Data: receive_data_pack.clone(),
            },
        );
    }
    let resp = Ok(CombatSendPackResponse {
        ErrorCode: Some(ErrorCode::Success),
        ReceivePackNotify: Some(CombatReceivePackNotify {
            Data: receive_data_pack,
        }),
    });
    if let Some(scene) = state.scene.as_mut()
        && !scene.pending_killed_removals.is_empty()
    {
        let mut remove_infos = Vec::new();
        scene.flush_due_killed_removals(&state.fs, now_millis(), &mut remove_infos);
        if !remove_infos.is_empty() {
            let _ = state.conn.push_msg(
                EntityRemoveNotify::MSG_ID,
                &EntityRemoveNotify {
                    IsRemove: true,
                    RemoveInfos: remove_infos,
                },
            );
        }
    }
    resp
}
