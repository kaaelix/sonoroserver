use sonoro_proto::PbMessage;
use sonoro_proto::enums::ActivityType;
use sonoro_proto::pb::{
    ActivityData, ActivityRequest, ActivityResponse, DirectTrainGetPlayerIdRequest,
    DirectTrainGetPlayerIdResponse,
};

use super::super::aki_logic_bridge;
use super::{State, activity};

pub(crate) fn filter_direct_train_activities(source: Vec<ActivityData>) -> Vec<ActivityData> {
    source
        .into_iter()
        .filter(|act| act.Type == Some(ActivityType::ThroughTrain))
        .collect()
}

pub fn handle_direct_train_get_player_id_request(
    state: &mut State,
    _req: DirectTrainGetPlayerIdRequest,
) -> Result<DirectTrainGetPlayerIdResponse, String> {
    if let Ok(resp) =
        aki_logic_bridge::call_player(state.player_id, 0, ActivityRequest::MSG_ID, &[])
        && resp.msg_id == ActivityResponse::MSG_ID
        && !resp.payload.is_empty()
        && let Ok(act_resp) = ActivityResponse::decode(&resp.payload)
    {
        return Ok(DirectTrainGetPlayerIdResponse {
            Activitys: filter_direct_train_activities(act_resp.Activities),
            MU1: None,
        });
    }

    let all = activity::build_activity_list(&state.assets, &state.player_components.progress, None);
    Ok(DirectTrainGetPlayerIdResponse {
        Activitys: filter_direct_train_activities(all),
        MU1: None,
    })
}
