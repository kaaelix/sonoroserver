use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::helpers::quest_engine;
use sonoro_proto::enums::{BtType, ErrorCode};
use sonoro_proto::pb::{
    FlowActionRequest, FlowActionResponse, FlowEndRequest, FlowEndResponse, FlowServerSkipNotify,
    GameCtxPb, GameCtxPbCtxInfoOneof, JSPatchNotify,
};

use super::State;

pub fn handle_flow_end_request(
    state: &mut State,
    events: &mut EventQueue,
    req: FlowEndRequest,
) -> Result<FlowEndResponse, String> {
    let (had_active_flow, prev_flow_list, prev_flow_id, prev_state_id) = state
        .scene
        .as_ref()
        .map_or((false, String::new(), 0, 0), |scene| {
            let had = scene.active_flow_inc_id != 0;
            (
                had,
                if had {
                    scene.active_flow_list_name.clone()
                } else {
                    String::new()
                },
                scene.active_flow_id,
                scene.active_flow_state_id,
            )
        });
    let prev_instance_id = state.player_components.scene.last_scene_info.instance_id;

    {
        let mut qctx = state.quest_context(events);
        quest_engine::on_flow_end_with_options(
            &mut qctx,
            req.FlowIncId,
            &req.OptionInfos,
            req.IsSkip,
        )
        .map_err(|e| e.to_string())?;
    }

    let same_instance = state.player_components.scene.last_scene_info.instance_id == prev_instance_id;
    let active_flow_inc_id = state
        .scene
        .as_ref()
        .map_or(0, |scene| scene.active_flow_inc_id);

    if active_flow_inc_id == 0
        && had_active_flow
        && let Ok(js) = std::fs::read("assets/scripts/misc_patches/close_flow_loading_and_subtitle.js")
    {
        let _ = state.conn.push_msg(
            JSPatchNotify::MSG_ID,
            &JSPatchNotify { Content: js },
        );
    }

    let client_black_screen_open = state
        .scene
        .as_ref()
        .is_some_and(|s| s.client_black_screen_open);
    if active_flow_inc_id == 0
        && same_instance
        && (client_black_screen_open || (req.IsSkip && had_active_flow))
    {
        if let Some(scene) = state.scene.as_mut() {
            scene.client_black_screen_open = false;
        }
        let _ = state.conn.push_msg(
            FlowServerSkipNotify::MSG_ID,
            &FlowServerSkipNotify {
                FlowListName: prev_flow_list.into_bytes(),
                FlowId: prev_flow_id,
                StateId: prev_state_id,
                FadeOutScreen: true,
            },
        );
    }

    Ok(FlowEndResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_flow_action_request(
    state: &mut State,
    events: &mut EventQueue,
    req: FlowActionRequest,
) -> Result<FlowActionResponse, String> {
    let mut qctx = state.quest_context(events);
    quest_engine::on_flow_action(&mut qctx, req.FlowIncId, req.ActionId)
        .map_err(|e| e.to_string())?;
    Ok(FlowActionResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

#[allow(dead_code)]
pub fn build_flow_bt_ctx(bt_type: BtType, bt_id: i32, node_id: i32) -> GameCtxPb {
    GameCtxPb {
        CtxType: Some(sonoro_proto::enums::GameCtxType::BehaviorTree),
        CtxInfo: Some(GameCtxPbCtxInfoOneof::BehaviorTree(
            sonoro_proto::pb::BehaviorTreeCtxPb {
                IncId: i64::from(bt_id),
                BtType: Some(bt_type),
                BtId: bt_id,
                NodeId: node_id,
            },
        )),
    }
}
