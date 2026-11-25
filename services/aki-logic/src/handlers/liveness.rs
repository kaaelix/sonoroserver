//! Liveness / Daily Activity handlers matching `LivenessHandler.cs`.

use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

use sonoro_proto::PbMessage;
use sonoro_proto::enums::*;
use sonoro_proto::pb::*;
use sonoro_proto::registry::msg_id;

use crate::bin_data;
use crate::ipc::AkiResult;
use crate::sqlite_store;

fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

pub fn handle_liveness_request(player_id: i32, _payload: &[u8]) -> AkiResult {
    let row = sqlite_store::get_or_create_liveness(player_id);
    let now_sec = unix_now();
    let taken_set: HashSet<i32> = row.claimed_tasks.iter().copied().collect();

    let mut info = LivenessInfo {
        LivenessCount: row.liveness_count,
        DayEnd: now_sec + 86_400,
        AreaId: 1,
        RewardedLiveness: row.claimed_boxes,
        ..Default::default()
    };

    for task in &bin_data::get().liveness_tasks {
        info.Tasks.push(LivenessTask {
            Id: task.task_id,
            Current: 1,
            Target: 1,
            IsFinished: true,
            IsTaken: taken_set.contains(&task.task_id),
            ConditionFinishState: true,
        });
    }

    AkiResult::ok(
        msg_id::LIVENESS_RESPONSE,
        &LivenessResponse {
            LivenessInfo: Some(info),
        },
    )
}

pub fn handle_liveness_task_take_request(player_id: i32, payload: &[u8]) -> AkiResult {
    let req = LivenessTaskTakeRequest::decode(payload).unwrap_or_default();
    let mut row = sqlite_store::get_or_create_liveness(player_id);
    let mut taken: HashSet<i32> = row.claimed_tasks.iter().copied().collect();
    let task_lookup: HashMap<i32, &_> = bin_data::get()
        .liveness_tasks
        .iter()
        .map(|t| (t.task_id, t))
        .collect();

    let mut resp = LivenessTaskTakeResponse {
        ErrorCode: Some(ErrorCode::Success),
        ..Default::default()
    };
    let mut grants: HashMap<i32, i32> = HashMap::new();
    let mut new_count = row.liveness_count;

    for tid in req.TaskIds {
        if !taken.insert(tid) {
            continue;
        }
        resp.TaskIds.push(tid);
        if let Some(cfg) = task_lookup.get(&tid) {
            for (&k, &v) in &cfg.task_reward {
                if k == 9 {
                    new_count += v;
                } else {
                    *grants.entry(k).or_insert(0) += v;
                }
            }
        } else {
            new_count += 20;
        }
    }

    row.liveness_count = new_count;
    row.claimed_tasks = taken.into_iter().collect();
    sqlite_store::save_liveness(&row);

    AkiResult::ok(msg_id::LIVENESS_TASK_TAKE_RESPONSE, &resp)
        .with_push(
            msg_id::LIVENESS_COUNT_UPDATE_NOTIFY,
            &LivenessCountUpdateNotify {
                LivenessCount: new_count,
            },
        )
        .with_grants(grants)
}

pub fn handle_liveness_take_request(player_id: i32, payload: &[u8]) -> AkiResult {
    let req = LivenessTakeRequest::decode(payload).unwrap_or_default();
    let mut row = sqlite_store::get_or_create_liveness(player_id);
    let mut claimed_boxes: HashSet<i32> = row.claimed_boxes.iter().copied().collect();
    let box_lookup: HashMap<i32, &_> = bin_data::get()
        .liveness_boxes
        .iter()
        .map(|b| (b.id, b))
        .collect();

    let mut resp = LivenessTakeResponse {
        ErrorCode: Some(ErrorCode::Success),
        ..Default::default()
    };
    let mut grants: HashMap<i32, i32> = HashMap::new();
    let cache = bin_data::get();

    for box_id in req.Ids {
        let Some(cfg) = box_lookup.get(&box_id) else {
            continue;
        };
        if row.liveness_count < cfg.goal || claimed_boxes.contains(&box_id) {
            continue;
        }
        claimed_boxes.insert(box_id);
        resp.Ids.push(box_id);

        let mut drop_items = cache.resolve_drop(cfg.drop_id);
        if drop_items.is_empty() {
            drop_items.push((3, 20));
            drop_items.push((2, 3000));
        }
        for (item_id, cnt) in drop_items {
            *grants.entry(item_id).or_insert(0) += cnt;
        }
    }

    if !resp.Ids.is_empty() {
        row.claimed_boxes = claimed_boxes.into_iter().collect();
        sqlite_store::save_liveness(&row);
    }

    AkiResult::ok(msg_id::LIVENESS_TAKE_RESPONSE, &resp).with_grants(grants)
}
