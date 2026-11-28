use sonoro_gamesv_logic::component::entity::tag::TagComponent;
use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::events::EntityHandle;
use sonoro_gamesv_logic::fsm::types::wake_reason;
use sonoro_gamesv_logic::handlers::tags as tag_handlers;
use sonoro_gamesv_logic::helpers::quest_actions;
use sonoro_proto::enums::{ErrorCode, TeleportReason};
use sonoro_proto::pb::{
    ANStartRequest, ANStartResponse, AiBlackboardCdPush, AiBlackboardCdRequest,
    AiBlackboardCdResponse, AiBlackboardsRequest, AiBlackboardsResponse,
    AnimationStateChangedRequest, AnimationStateChangedResponse, AnimationStateInitRequest,
    AnimationStateInitResponse, BlackboardParamValueOneof, BoneVisibleChangeRequest,
    BoneVisibleChangeResponse, BuffEffectRequest, BuffEffectResponse, CaughtRequest, CaughtResponse,
    ChangeSummonEntityStateRequest, ChangeSummonEntityStateResponse,
    ClientCurrentRoleReportRequest, ClientCurrentRoleReportResponse, CombatCommon,
    CombatNotifyDataMessageOneof, CombatReceiveData, CommonTagData, CreateBulletRequest,
    CreateBulletResponse, DErrorResult, DestroyBulletRequest, DestroyBulletResponse,
    DrownEndTeleportPush, DrownEndTeleportPushYcuOneof, DrownEndTeleportRequest,
    DrownEndTeleportResponse, EnterViewDirectionRequest, EnterViewDirectionResponse,
    EntityBlackboardNotify, EntityCommonTagNotify, EntityStaticHookMoveRequest,
    EntityStaticHookMoveResponse, ExecuteQteNotify, ExecuteQtePush, ExecuteQteRequest,
    ExecuteQteResponse, ExitViewDirectionRequest, ExitViewDirectionResponse, GameplayCueNotify,
    GameplayCueRequest, GameplayCueResponse, GaSwitchCommonEnemyProCampRequest,
    GaSwitchCommonEnemyProCampResponse, InterruptSkillInDelayRequest, InterruptSkillInDelayResponse,
    MaterialRequest, MaterialResponse, ModifyBulletParamsRequest, ModifyBulletParamsResponse,
    MotorSummonAndRideNotify, MotorSummonAndRidePush, RTimeStopInstRequest,
    RTimeStopInstResponse, RTimeStopRequest, RTimeStopResponse, RoleTagChangePush,
    RoleTagChangeRequest, RoleTagChangeResponse, TeleportNotify,
    ToughCalcExtraRatioChangeRequest, ToughCalcExtraRatioChangeResponse, TriggerExitSkillRequest,
    TriggerExitSkillResponse, TsAnimNotifyStateAbsoluteTimeStopRequest,
    TsAnimNotifyStateAbsoluteTimeStopResponse, UseSkillFailRequest, UseSkillFailResponse, Vector,
};

use super::combat::push_combat_notify;
use super::{State, fsm};

fn resolve_qte_common(common: Option<CombatCommon>, up_entity_id: i64) -> Option<CombatCommon> {
    common.or_else(|| {
        (up_entity_id != 0).then(|| CombatCommon {
            EntityId: up_entity_id,
            ..Default::default()
        })
    })
}

pub fn handle_execute_qte_push(
    push: ExecuteQtePush,
    common: Option<CombatCommon>,
    receive_data_pack: &mut Vec<CombatReceiveData>,
) {
    push_combat_notify(
        receive_data_pack,
        resolve_qte_common(common, push.UpEntityId),
        CombatNotifyDataMessageOneof::ExecuteQteNotify(ExecuteQteNotify {
            DownEntityId: push.DownEntityId,
            UpEntityId: push.UpEntityId,
            FnvHash: push.FnvHash,
        }),
    );
}

pub fn handle_execute_qte_request(
    req: ExecuteQteRequest,
    common: Option<CombatCommon>,
    receive_data_pack: &mut Vec<CombatReceiveData>,
) -> ExecuteQteResponse {
    push_combat_notify(
        receive_data_pack,
        resolve_qte_common(common, req.UpEntityId),
        CombatNotifyDataMessageOneof::ExecuteQteNotify(ExecuteQteNotify {
            DownEntityId: req.DownEntityId,
            UpEntityId: req.UpEntityId,
            FnvHash: req.FnvHash,
        }),
    );
    ExecuteQteResponse {
        ErrorCode: Some(ErrorCode::Success),
    }
}

pub fn handle_change_summon_entity_state_request(
    state: &mut State,
    req: ChangeSummonEntityStateRequest,
) -> ChangeSummonEntityStateResponse {
    if req.SummonEntityId != 0 {
        state.player_components.progress.info.set_key_int(
            &format!("summon_state:{}", req.SummonEntityId),
            i64::from(req.TagId),
        );
        if let Err(err) = state.player_components.progress.save(&state.fs) {
            tracing::warn!(
                "change_summon_entity_state progress save failed for {}: {err}",
                req.SummonEntityId
            );
        }
        let State {
            conn,
            fs,
            assets,
            scene: scene_opt,
            coop_service,
            player_id,
            ..
        } = state;
        if let Some(scene) = scene_opt.as_mut()
            && let Some(&idx) = scene.net_id_map.get(&req.SummonEntityId)
        {
            if req.TagId != 0 {
                let tags = scene.entities[idx]
                    .tag
                    .get_or_insert_with(TagComponent::default);
                tags.set_tag_count(req.TagId, 1);
                if !tags.entity_common_tags.contains(&req.TagId) {
                    tags.entity_common_tags.push(req.TagId);
                }
            }
            scene.mark_fsm_dirty(req.SummonEntityId, wake_reason::TAG);
            let _ = scene.save_entity_by_net_id(fs, req.SummonEntityId);
            if req.TagId != 0 {
                let handle = EntityHandle {
                    index: idx,
                    net_id: req.SummonEntityId,
                };
                tag_handlers::update_gameplay_tags(
                    conn,
                    scene,
                    assets,
                    handle,
                    &[req.TagId],
                    &[],
                    false,
                );
                if let Some(coop) = coop_service.as_ref()
                    && tag_handlers::has_client_level_tag_component(
                        scene,
                        Some(assets),
                        req.SummonEntityId,
                    )
                {
                    coop.broadcast_room_except(
                        *player_id,
                        EntityCommonTagNotify::MSG_ID,
                        &EntityCommonTagNotify {
                            Id: req.SummonEntityId,
                            Tags: vec![CommonTagData {
                                TagId: req.TagId,
                                RemoveTagIds: true,
                            }],
                        },
                    );
                }
            }
        }
    }
    ChangeSummonEntityStateResponse {
        ErrorCode: Some(ErrorCode::Success),
    }
}

pub fn handle_motor_summon_and_ride_push(
    state: &State,
    push: MotorSummonAndRidePush,
    common: Option<CombatCommon>,
    receive_data_pack: &mut Vec<CombatReceiveData>,
) {
    push_combat_notify(
        receive_data_pack,
        common,
        CombatNotifyDataMessageOneof::MotorSummonAndRideNotify(MotorSummonAndRideNotify {
            PlayerId: state.player_id,
            EntityId: push.EntityId,
            VehicleIncId: push.VehicleIncId,
            Transform: push.Transform,
        }),
    );
}

pub fn handle_ai_blackboard_cd_push(
    push: AiBlackboardCdPush,
    common: Option<CombatCommon>,
    receive_data_pack: &mut Vec<CombatReceiveData>,
) {
    push_combat_notify(
        receive_data_pack,
        common,
        CombatNotifyDataMessageOneof::AiBlackboardCdNotify(
            sonoro_proto::pb::AiBlackboardCdNotify {
                AiBlackboardCdDel: Vec::new(),
                AiBlackboardCdModify: push.AiBlackboardCdModify,
                AiBlackboardCdComplete: push.AiBlackboardCdComplete,
            },
        ),
    );
}

pub fn handle_drown_end_teleport_push(
    state: &mut State,
    events: &mut EventQueue,
    push: DrownEndTeleportPush,
) {
    let Some(DrownEndTeleportPushYcuOneof::TeleportPos(v)) = push.ycu else {
        return;
    };
    if !v.X.is_finite() || !v.Y.is_finite() || !v.Z.is_finite() {
        return;
    }
    let inst_id = state
        .scene
        .as_ref()
        .map_or(state.player_components.scene.last_scene_info.instance_id, |s| {
            s.instance_id
        });
    let cur_map_id = state.assets.resolve_map_config_id(inst_id);
    if cur_map_id <= 0 {
        return;
    }
    let mut qctx = state.quest_context(events);
    let _ = quest_actions::update_player_scene_state(&mut qctx, [v.X, v.Y, v.Z], None);
    let _ = state.conn.push_msg(
        TeleportNotify::MSG_ID,
        &TeleportNotify {
            MapId: cur_map_id,
            Pos: Some(Vector {
                X: v.X,
                Y: v.Y,
                Z: v.Z,
            }),
            Reason: Some(TeleportReason::Drown),
            ..Default::default()
        },
    );
}

pub fn handle_use_skill_fail_request(req: UseSkillFailRequest) -> UseSkillFailResponse {
    UseSkillFailResponse {
        SkillId: req.SkillId,
        Error: Some(DErrorResult {
            ErrorCode: Some(ErrorCode::Success),
            ErrorParams: Vec::new(),
        }),
    }
}

pub fn handle_an_start_request(req: ANStartRequest) -> ANStartResponse {
    ANStartResponse {
        SkillId: req.SkillId,
        MontageIndex: req.MontageIndex,
        AnIndex: req.AnIndex,
        Error: Some(DErrorResult {
            ErrorCode: Some(ErrorCode::Success),
            ErrorParams: Vec::new(),
        }),
    }
}

#[allow(dead_code)]
pub fn handle_client_current_role_report_request(
    req: ClientCurrentRoleReportRequest,
) -> ClientCurrentRoleReportResponse {
    ClientCurrentRoleReportResponse {
        ErrorCode: Some(ErrorCode::Success),
        CurrentEntityId: req.CurrentEntityId,
        PlayerId: req.PlayerId,
    }
}

pub fn handle_client_current_role_report_request_with_state(
    state: &mut State,
    req: ClientCurrentRoleReportRequest,
) -> ClientCurrentRoleReportResponse {
    if req.CurrentEntityId != 0
        && let Some(scene) = state.scene.as_mut()
        && let Some(&idx) = scene.net_id_map.get(&req.CurrentEntityId)
    {
        let role_id = scene.entities[idx].config.config_id;
        if role_id > 0 {
            let f_idx = scene.formation_info.cur_formation as usize;
            if let Some(form) = scene.formation_info.formations.get_mut(f_idx)
                && form
                    .roles
                    .iter()
                    .flatten()
                    .any(|slot| slot.role_id == role_id)
            {
                let prev_role = form.cur_role;
                form.cur_role = role_id;
                if prev_role != role_id {
                    for ent in &mut scene.entities {
                        if ent.config.config_id == prev_role {
                            ent.visible = false;
                        } else if ent.config.config_id == role_id {
                            ent.visible = true;
                        }
                    }
                    let _ = scene.save_persistent(&state.fs);
                    if let Some(coop) = state.coop_service.as_ref() {
                        coop.update_active_role(state.player_id, role_id);
                    }
                }
            }
        }
    }
    ClientCurrentRoleReportResponse {
        ErrorCode: Some(ErrorCode::Success),
        CurrentEntityId: req.CurrentEntityId,
        PlayerId: req.PlayerId,
    }
}

pub fn handle_gameplay_cue_request_with_state(
    state: &State,
    common: Option<CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    req: GameplayCueRequest,
) -> GameplayCueResponse {
    let resolved_common = common.or_else(|| {
        state
            .scene
            .as_ref()
            .map(|s| s.current_role_entity_id())
            .filter(|&id| id != 0)
            .map(|id| CombatCommon {
                EntityId: id,
                ..Default::default()
            })
    });
    if req.GameplayCueId != 0 {
        push_combat_notify(
            pack,
            resolved_common,
            CombatNotifyDataMessageOneof::GameplayCueNotify(GameplayCueNotify {
                GameplayCueId: req.GameplayCueId,
            }),
        );
    }
    GameplayCueResponse {
        ErrorCode: Some(ErrorCode::Success),
    }
}

pub fn handle_role_tag_change_request_with_state(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<&CombatCommon>,
    pack: &mut Vec<CombatReceiveData>,
    req: RoleTagChangeRequest,
) -> RoleTagChangeResponse {
    fsm::handle_role_tag_change_push(
        state,
        events,
        common,
        pack,
        RoleTagChangePush {
            TagId: req.TagId,
            TagCount: req.TagCount,
        },
    );
    RoleTagChangeResponse {
        ErrorCode: Some(ErrorCode::Success),
    }
}

pub fn handle_ai_blackboards_request_with_state(
    state: &mut State,
    common: Option<&CombatCommon>,
    req: AiBlackboardsRequest,
) -> AiBlackboardsResponse {
    let eid = common
        .map(|c| c.EntityId)
        .filter(|&id| id != 0)
        .or_else(|| {
            state
                .scene
                .as_ref()
                .map(|s| s.current_role_entity_id())
                .filter(|&id| id != 0)
        })
        .unwrap_or(0);
    let cfg_id = if eid != 0 {
        state
            .scene
            .as_ref()
            .and_then(|sc| sc.find_config_id_by_net_id(eid))
            .map_or(eid, i64::from)
    } else {
        0
    };
    let mut dirty = false;
    for param in &req.AiBlackboards {
        if param.Key.is_empty() {
            continue;
        }
        let pkey = String::from_utf8_lossy(&param.Key);
        let full_k = format!("ent_bb:{cfg_id}:{pkey}");
        let info = &mut state.player_components.progress.info;
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
        && !req.AiBlackboards.is_empty()
    {
        coop.broadcast_room_except(
            state.player_id,
            EntityBlackboardNotify::MSG_ID,
            &EntityBlackboardNotify {
                EntityId: eid,
                Params: req.AiBlackboards,
            },
        );
    }
    AiBlackboardsResponse {
        ErrorCode: Some(ErrorCode::Success),
    }
}

macro_rules! simple_success_fn {
    ($fn_name:ident, $req:ty, $resp:ty) => {
        pub fn $fn_name(_req: $req) -> $resp {
            <$resp>::default_with_success()
        }
    };
}

trait WithSuccess {
    fn default_with_success() -> Self;
}

macro_rules! impl_with_success {
    ($($t:ty),* $(,)?) => {
        $(
            impl WithSuccess for $t {
                #[allow(clippy::needless_update)]
                fn default_with_success() -> Self {
                    Self {
                        ErrorCode: Some(ErrorCode::Success),
                        ..Default::default()
                    }
                }
            }
        )*
    };
}

impl_with_success!(
    GameplayCueResponse,
    CaughtResponse,
    MaterialResponse,
    BoneVisibleChangeResponse,
    ModifyBulletParamsResponse,
    AnimationStateChangedResponse,
    AnimationStateInitResponse,
    EntityStaticHookMoveResponse,
    AiBlackboardsResponse,
    AiBlackboardCdResponse,
    ToughCalcExtraRatioChangeResponse,
    EnterViewDirectionResponse,
    ExitViewDirectionResponse,
    InterruptSkillInDelayResponse,
    TriggerExitSkillResponse,
    BuffEffectResponse,
    DrownEndTeleportResponse,
    RTimeStopResponse,
    RTimeStopInstResponse,
    TsAnimNotifyStateAbsoluteTimeStopResponse,
    RoleTagChangeResponse,
    CreateBulletResponse,
    DestroyBulletResponse,
    GaSwitchCommonEnemyProCampResponse,
);

#[allow(dead_code)]
fn _unused_placeholder_marker() {}
#[allow(dead_code)]
simple_success_fn!(handle_gameplay_cue_request, GameplayCueRequest, GameplayCueResponse);
simple_success_fn!(handle_caught_request, CaughtRequest, CaughtResponse);
simple_success_fn!(handle_material_request, MaterialRequest, MaterialResponse);
simple_success_fn!(handle_bone_visible_change_request, BoneVisibleChangeRequest, BoneVisibleChangeResponse);
simple_success_fn!(handle_modify_bullet_params_request, ModifyBulletParamsRequest, ModifyBulletParamsResponse);
simple_success_fn!(handle_animation_state_changed_request, AnimationStateChangedRequest, AnimationStateChangedResponse);
simple_success_fn!(handle_animation_state_init_request, AnimationStateInitRequest, AnimationStateInitResponse);
simple_success_fn!(handle_entity_static_hook_move_request, EntityStaticHookMoveRequest, EntityStaticHookMoveResponse);
#[allow(dead_code)]
simple_success_fn!(handle_ai_blackboards_request, AiBlackboardsRequest, AiBlackboardsResponse);
simple_success_fn!(handle_ai_blackboard_cd_request, AiBlackboardCdRequest, AiBlackboardCdResponse);
simple_success_fn!(handle_tough_calc_extra_ratio_change_request, ToughCalcExtraRatioChangeRequest, ToughCalcExtraRatioChangeResponse);
simple_success_fn!(handle_enter_view_direction_request, EnterViewDirectionRequest, EnterViewDirectionResponse);
simple_success_fn!(handle_exit_view_direction_request, ExitViewDirectionRequest, ExitViewDirectionResponse);
simple_success_fn!(handle_interrupt_skill_in_delay_request, InterruptSkillInDelayRequest, InterruptSkillInDelayResponse);
simple_success_fn!(handle_trigger_exit_skill_request, TriggerExitSkillRequest, TriggerExitSkillResponse);
simple_success_fn!(handle_buff_effect_request, BuffEffectRequest, BuffEffectResponse);
simple_success_fn!(handle_drown_end_teleport_request, DrownEndTeleportRequest, DrownEndTeleportResponse);
simple_success_fn!(handle_r_time_stop_request, RTimeStopRequest, RTimeStopResponse);
simple_success_fn!(handle_r_time_stop_inst_request, RTimeStopInstRequest, RTimeStopInstResponse);
simple_success_fn!(handle_ts_anim_stop_request, TsAnimNotifyStateAbsoluteTimeStopRequest, TsAnimNotifyStateAbsoluteTimeStopResponse);
#[allow(dead_code)]
simple_success_fn!(handle_role_tag_change_request, RoleTagChangeRequest, RoleTagChangeResponse);
simple_success_fn!(handle_create_bullet_request, CreateBulletRequest, CreateBulletResponse);
simple_success_fn!(handle_destroy_bullet_request, DestroyBulletRequest, DestroyBulletResponse);
simple_success_fn!(handle_ga_switch_enemy_camp_request, GaSwitchCommonEnemyProCampRequest, GaSwitchCommonEnemyProCampResponse);
