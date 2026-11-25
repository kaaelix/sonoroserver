use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::helpers::quest_engine;
use sonoro_gamesv_logic::scene::now_millis;
use sonoro_proto::enums::ErrorCode;
use sonoro_proto::pb::{
    CombatCommon, EndSkillPush, EndSkillRequest, EndSkillResponse, SkillRequest, SkillResponse,
    UseSkillInformation, UseSkillRequest, UseSkillResponse,
};

use super::State;

fn skill_id(info: Option<&UseSkillInformation>) -> Option<i32> {
    let id = info?.SkillId;
    (id > 0).then(|| i32::try_from(id).ok()).flatten()
}

pub fn handle_skill_request(_req: SkillRequest) -> SkillResponse {
    SkillResponse {
        ErrorCode: Some(ErrorCode::Success),
    }
}

pub fn handle_use_skill_request(
    state: &mut State,
    events: &mut EventQueue,
    common: Option<CombatCommon>,
    req: UseSkillRequest,
) -> UseSkillResponse {
    let info = req.UseSkillInfo;
    if let Some(sid) = skill_id(info.as_ref()) {
        if let Some(entity_id) = common.as_ref().map(|c| c.EntityId).filter(|&id| id != 0)
            && let Some(scene) = state.scene.as_mut()
        {
            scene.note_fsm_skill_start(entity_id, sid, now_millis());
        }
        let mut qctx = state.quest_context(events);
        if let Err(err) = quest_engine::on_skill_used(&mut qctx, sid) {
            tracing::warn!("onUseSkillRequest hook error: {err}");
        }
    }
    UseSkillResponse {
        UseSkillInfo: info,
        SkillSingleId: req.SkillSingleId,
        ErrorCode: Some(ErrorCode::Success),
    }
}

pub fn handle_end_skill_request(
    state: &mut State,
    common: Option<CombatCommon>,
    req: EndSkillRequest,
) -> EndSkillResponse {
    let info = req.UseSkillInfo;
    if let Some(entity_id) = common.as_ref().map(|c| c.EntityId).filter(|&id| id != 0)
        && let Some(sid) = skill_id(info.as_ref())
        && let Some(scene) = state.scene.as_mut()
    {
        scene.signal_fsm_skill_end(entity_id, sid, now_millis());
    }
    EndSkillResponse {
        UseSkillInfo: info,
        SkillSingleId: req.SkillSingleId,
        ErrorCode: Some(ErrorCode::Success),
    }
}

pub fn handle_end_skill_push(
    state: &mut State,
    common: Option<CombatCommon>,
    push: EndSkillPush,
) {
    if let Some(entity_id) = common.as_ref().map(|c| c.EntityId).filter(|&id| id != 0)
        && let Some(sid) = skill_id(push.UseSkillInfo.as_ref())
        && let Some(scene) = state.scene.as_mut()
    {
        scene.signal_fsm_skill_end(entity_id, sid, now_millis());
    }
}
