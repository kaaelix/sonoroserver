use std::collections::BTreeMap;

use sonoro_gamesv_data::tables::liveness_task::LivenessTask as LivenessTaskCfg;
use sonoro_gamesv_fs::progress_info::ProgressInfo;
use sonoro_gamesv_logic::helpers::{condition, craft, energy, reward};
use sonoro_proto::enums::ErrorCode;
use sonoro_proto::pb::{
    LivenessCountUpdateNotify, LivenessInfo, LivenessRequest, LivenessResponse, LivenessTakeRequest,
    LivenessTakeResponse, LivenessTask, LivenessTaskTakeRequest, LivenessTaskTakeResponse,
};

use super::State;

fn evaluate_liveness_task(
    state: &State,
    eval_ctx: &condition::EvalContext<'_>,
    row: &LivenessTaskCfg,
) -> (i32, i32, bool, bool) {
    let progress = &state.player_components.progress.info;
    let condition_ok = row.UnlockCondition <= 0
        || condition::eval_condition_or_group(eval_ctx, row.UnlockCondition);
    let taken = progress.liveness.taken_tasks.contains(&row.TaskId);
    let recorded = ProgressInfo::get_id_count(&progress.liveness.task_progress, row.TaskId);
    let target = 1;
    // TaskId 101 is the daily login task; an active session completes it automatically once unlocked.
    let current = if taken {
        recorded.max(target)
    } else if row.TaskId == 101 && condition_ok {
        recorded.max(target)
    } else {
        recorded.max(0)
    };
    let is_finished = condition_ok && current >= target;
    (current, target, is_finished, condition_ok)
}

pub fn build_liveness_info_pb(state: &State, now_sec: i64) -> LivenessInfo {
    let progress = &state.player_components.progress.info;
    let eval_ctx = state.eval_context();
    let tasks = state
        .assets
        .tables
        .liveness_task
        .items
        .iter()
        .map(|row| {
            let taken = progress.liveness.taken_tasks.contains(&row.TaskId);
            let (cur, target, is_finished, cond_ok) = evaluate_liveness_task(state, &eval_ctx, row);
            LivenessTask {
                Id: row.TaskId,
                Current: cur,
                Target: target,
                IsFinished: is_finished,
                IsTaken: taken,
                ConditionFinishState: cond_ok,
            }
        })
        .collect();

    LivenessInfo {
        LivenessCount: progress.liveness.liveness_count,
        RewardedLiveness: progress.liveness.claimed_boxes.clone(),
        Tasks: tasks,
        DayEnd: now_sec + 86_400,
        AreaId: state.player_components.basic.info.attributes.area_id,
    }
}

pub fn handle_liveness_request(
    state: &mut State,
    _req: LivenessRequest,
) -> Result<LivenessResponse, String> {
    Ok(LivenessResponse {
        LivenessInfo: Some(build_liveness_info_pb(state, energy::now_seconds())),
    })
}

pub fn handle_liveness_take_request(
    state: &mut State,
    req: LivenessTakeRequest,
) -> Result<LivenessTakeResponse, String> {
    let mut taken_ids = Vec::new();
    let mut item_map = BTreeMap::new();
    let mut changed = false;

    for box_id in req.Ids {
        let Some(box_cfg) = state
            .assets
            .tables
            .liveness
            .get_data_by_id(&box_id)
            .map(|c| c.into_owned())
        else {
            continue;
        };
        if state.player_components.progress.info.liveness.liveness_count < box_cfg.Goal {
            continue;
        }
        if state
            .player_components
            .progress
            .info
            .liveness
            .claimed_boxes
            .contains(&box_id)
        {
            continue;
        }
        ProgressInfo::append_unique_id(
            &mut state.player_components.progress.info.liveness.claimed_boxes,
            box_id,
        );
        let mut cctx = state.craft_context();
        craft::grant_drop_and_collect_items(&mut cctx, box_cfg.DropId, 3, 10, 19600, &mut item_map)
            .map_err(|e| e.to_string())?;
        taken_ids.push(box_id);
        changed = true;
    }

    if changed {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }

    Ok(LivenessTakeResponse {
        Ids: taken_ids,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_liveness_task_take_request(
    state: &mut State,
    req: LivenessTaskTakeRequest,
) -> Result<LivenessTaskTakeResponse, String> {
    let mut claimed = Vec::new();
    let mut extra_items = Vec::new();
    let mut changed = false;

    for tid in req.TaskIds {
        let Some(task) = state
            .assets
            .tables
            .liveness_task
            .get_data_by_id(&tid)
            .map(|c| c.into_owned())
        else {
            continue;
        };
        if state
            .player_components
            .progress
            .info
            .liveness
            .taken_tasks
            .contains(&tid)
        {
            continue;
        }
        let is_finished = {
            let eval_ctx = state.eval_context();
            let (_, _, finished, _) = evaluate_liveness_task(state, &eval_ctx, &task);
            finished
        };
        if !is_finished {
            continue;
        }
        ProgressInfo::append_unique_id(
            &mut state.player_components.progress.info.liveness.taken_tasks,
            tid,
        );
        for (&rid, &cnt) in task.TaskReward.iter() {
            if rid == 9 {
                state.player_components.progress.info.liveness.liveness_count += cnt;
            } else if rid > 0 && cnt > 0 {
                extra_items.push(reward::RewardItem {
                    item_id: rid,
                    count: cnt,
                });
            }
        }
        claimed.push(tid);
        changed = true;
    }

    if !extra_items.is_empty() {
        let mut rctx = state.reward_context();
        let _ = reward::grant_items(&mut rctx, 0, true, &extra_items, 19600)
            .map_err(|e| e.to_string())?;
    }

    if changed {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        let count = state.player_components.progress.info.liveness.liveness_count;
        let _ = state.conn.push_msg(
            LivenessCountUpdateNotify::MSG_ID,
            &LivenessCountUpdateNotify {
                LivenessCount: count,
            },
        );
    }

    Ok(LivenessTaskTakeResponse {
        ErrorCode: Some(ErrorCode::Success),
        TaskIds: claimed,
    })
}
