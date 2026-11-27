use sonoro_gamesv_logic::helpers::energy;
use sonoro_proto::enums::ActivityType;
use sonoro_proto::pb::{
    ActivityData, ActivityDataDataOneof, DirectTrainGetPlayerIdRequest,
    DirectTrainGetPlayerIdResponse, DirectTrainGetPlayerIdResponseMU1Oneof,
    ThroughTrainActivityData, ThroughTrainSummaryActivityData,
};

#[cfg_attr(not(test), allow(dead_code))]
pub fn filter_direct_train_activities(activities: Vec<ActivityData>) -> Vec<ActivityData> {
    activities
        .into_iter()
        .filter(|act| act.Type == Some(ActivityType::ThroughTrain))
        .collect()
}

use super::{State, activity};

pub fn handle_direct_train_get_player_id_request(
    state: &mut State,
    _req: DirectTrainGetPlayerIdRequest,
) -> Result<DirectTrainGetPlayerIdResponse, String> {
    let now_s = energy::now_seconds();
    let completed_quests = &state.player_components.quest.info.finished_quests;
    let mut activities = Vec::new();
    let mut completed_ids = Vec::new();

    for entry in state.assets.direct_train_activities() {
        if entry.Id <= 0 {
            continue;
        }
        let is_finish = (entry.SkipQuestId > 0 && completed_quests.contains(&entry.SkipQuestId))
            || state
                .player_components
                .progress
                .info
                .has_claim(&format!("direct_train:fin:{}", entry.Id));
        if is_finish {
            completed_ids.push(entry.Id);
        }
        if let Some(act) = activity::build_activity_entry(
            entry.Id,
            ActivityType::ThroughTrain as i32,
            now_s,
            true,
            Some(ActivityDataDataOneof::ThroughTrainActivityData(
                ThroughTrainActivityData { IsFinish: is_finish },
            )),
        ) {
            activities.push(act);
        }
    }

    let mu1 = state.assets.direct_train_main().first().and_then(|main| {
        let mut sub_ids = main.SubActivityIds.clone();
        if sub_ids.is_empty() {
            sub_ids = activities.iter().map(|a| a.Id).collect();
        }
        activity::build_activity_entry(
            main.ActivityId,
            ActivityType::ThroughTrainSummary as i32,
            now_s,
            true,
            Some(ActivityDataDataOneof::ThroughTrainSummaryActivityData(
                ThroughTrainSummaryActivityData {
                    ActivityIds: sub_ids,
                    CompletedActivityIds: completed_ids,
                },
            )),
        )
        .map(DirectTrainGetPlayerIdResponseMU1Oneof::Activities)
    });

    Ok(DirectTrainGetPlayerIdResponse {
        Activitys: activities,
        MU1: mu1,
    })
}
