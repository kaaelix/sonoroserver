use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::helpers::quest_actions;
use sonoro_proto::enums::{ErrorCode, TeleportReason};
use sonoro_proto::pb::{
    ANStartRequest, ANStartResponse, AiBlackboardCdPush, AiBlackboardCdRequest,
    AiBlackboardCdResponse, AiBlackboardsRequest, AiBlackboardsResponse,
    AnimationStateChangedRequest, AnimationStateChangedResponse, AnimationStateInitRequest,
    AnimationStateInitResponse, BoneVisibleChangeRequest, BoneVisibleChangeResponse,
    BuffEffectRequest, BuffEffectResponse, CaughtRequest, CaughtResponse,
    ChangeSummonEntityStateRequest, ChangeSummonEntityStateResponse,
    ClientCurrentRoleReportRequest, ClientCurrentRoleReportResponse, CombatCommon,
    CombatNotifyDataMessageOneof, CombatReceiveData, CreateBulletRequest, CreateBulletResponse,
    DErrorResult, DestroyBulletRequest, DestroyBulletResponse, DrownEndTeleportPush,
    DrownEndTeleportPushYcuOneof, DrownEndTeleportRequest, DrownEndTeleportResponse,
    EnterViewDirectionRequest, EnterViewDirectionResponse, EntityStaticHookMoveRequest,
    EntityStaticHookMoveResponse, ExecuteQteNotify, ExecuteQtePush, ExecuteQteRequest,
    ExecuteQteResponse, ExitViewDirectionRequest, ExitViewDirectionResponse, GameplayCueRequest,
    GameplayCueResponse, GaSwitchCommonEnemyProCampRequest, GaSwitchCommonEnemyProCampResponse,
    InterruptSkillInDelayRequest, InterruptSkillInDelayResponse, MaterialRequest, MaterialResponse,
    ModifyBulletParamsRequest, ModifyBulletParamsResponse, MotorSummonAndRideNotify,
    MotorSummonAndRidePush, RTimeStopInstRequest, RTimeStopInstResponse, RTimeStopRequest,
    RTimeStopResponse, RoleTagChangeRequest, RoleTagChangeResponse, TeleportNotify,
    ToughCalcExtraRatioChangeRequest, ToughCalcExtraRatioChangeResponse, TriggerExitSkillRequest,
    TriggerExitSkillResponse, TsAnimNotifyStateAbsoluteTimeStopRequest,
    TsAnimNotifyStateAbsoluteTimeStopResponse, UseSkillFailRequest, UseSkillFailResponse, Vector,
};

use super::combat::push_combat_notify;
use super::State;

pub fn handle_execute_qte_push(
    push: ExecuteQtePush,
    common: Option<CombatCommon>,
    receive_data_pack: &mut Vec<CombatReceiveData>,
) {
    push_combat_notify(
        receive_data_pack,
        common,
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
        common,
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
    if let Some(scene) = state.scene.as_mut()
        && let Some(&idx) = scene.net_id_map.get(&req.SummonEntityId)
        && let Some(tags) = scene.entities[idx].tag.as_mut()
    {
        tags.set_tag_count(req.TagId, 1);
        if !tags.entity_common_tags.contains(&req.TagId) {
            tags.entity_common_tags.push(req.TagId);
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

pub fn handle_client_current_role_report_request(
    req: ClientCurrentRoleReportRequest,
) -> ClientCurrentRoleReportResponse {
    let _ = req;
    ClientCurrentRoleReportResponse {
        ErrorCode: Some(ErrorCode::Success),
        ..Default::default()
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

simple_success_fn!(handle_gameplay_cue_request, GameplayCueRequest, GameplayCueResponse);
simple_success_fn!(handle_caught_request, CaughtRequest, CaughtResponse);
simple_success_fn!(handle_material_request, MaterialRequest, MaterialResponse);
simple_success_fn!(handle_bone_visible_change_request, BoneVisibleChangeRequest, BoneVisibleChangeResponse);
simple_success_fn!(handle_modify_bullet_params_request, ModifyBulletParamsRequest, ModifyBulletParamsResponse);
simple_success_fn!(handle_animation_state_changed_request, AnimationStateChangedRequest, AnimationStateChangedResponse);
simple_success_fn!(handle_animation_state_init_request, AnimationStateInitRequest, AnimationStateInitResponse);
simple_success_fn!(handle_entity_static_hook_move_request, EntityStaticHookMoveRequest, EntityStaticHookMoveResponse);
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
simple_success_fn!(handle_role_tag_change_request, RoleTagChangeRequest, RoleTagChangeResponse);
simple_success_fn!(handle_create_bullet_request, CreateBulletRequest, CreateBulletResponse);
simple_success_fn!(handle_destroy_bullet_request, DestroyBulletRequest, DestroyBulletResponse);
simple_success_fn!(handle_ga_switch_enemy_camp_request, GaSwitchCommonEnemyProCampRequest, GaSwitchCommonEnemyProCampResponse);
