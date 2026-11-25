//! Battle Pass handlers matching `BattlePassHandler.cs`.

use std::collections::{HashMap, HashSet};

use sonoro_proto::PbMessage;
use sonoro_proto::enums::*;
use sonoro_proto::pb::*;
use sonoro_proto::registry::msg_id;

use crate::bin_data::{self, BpRewardEntry};
use crate::ipc::AkiResult;
use crate::sqlite_store;

const FAR_FUTURE: i64 = 4_070_908_800;

fn pay_status_from_i32(v: i32) -> BattlePassPayStatus {
    match v {
        0 => BattlePassPayStatus::NoPaid,
        1 => BattlePassPayStatus::Paid,
        _ => BattlePassPayStatus::Advanced,
    }
}

fn bp_type_from_i32(v: i32) -> BattlePassType {
    if v == 0 {
        BattlePassType::Free
    } else {
        BattlePassType::Pay
    }
}

fn resolve_reward_entry(bp_id: i32, level: i32) -> Option<&'static BpRewardEntry> {
    let cache = bin_data::get();
    for cand in [cache.bp_meta.latest_reward_id, bp_id, 0, 1] {
        if let Some(cfg) = cache.bp_rewards.get(&(cand, level)) {
            return Some(cfg);
        }
    }
    None
}

fn resolve_reward_item_ids(bp_id: i32, level: i32, type_int: i32) -> Vec<i32> {
    let Some(cfg) = resolve_reward_entry(bp_id, level) else {
        return Vec::new();
    };
    let map = if type_int == 0 {
        &cfg.free_reward
    } else {
        &cfg.pay_reward
    };
    map.keys().copied().filter(|&id| id > 0).collect()
}

fn collect_level_grants(bp_id: i32, level: i32, type_int: i32, acc: &mut HashMap<i32, i32>) {
    let Some(cfg) = resolve_reward_entry(bp_id, level) else {
        return;
    };
    let map = if type_int == 0 {
        &cfg.free_reward
    } else {
        &cfg.pay_reward
    };
    for (&k, &v) in map {
        *acc.entry(k).or_insert(0) += v;
    }
}

pub fn handle_battle_pass_request(player_id: i32, _payload: &[u8]) -> AkiResult {
    let mut row = sqlite_store::get_or_create_battle_pass(player_id);
    let bp_id = row.bp_id.max(bin_data::get().bp_meta.latest_bp_id);

    let mut pb_bp = PbBattlePass {
        InTimeRange: true,
        Id: bp_id,
        Level: row.level,
        Exp: row.exp,
        WeeklyTotalExp: row.weekly_exp,
        PayStatus: Some(pay_status_from_i32(row.pay_status)),
        BeginTime: 0,
        EndTime: FAR_FUTURE,
        HadEnter: row.had_enter,
        ..Default::default()
    };

    let mut repaired = false;
    let mut healed_entries: Vec<Vec<i32>> = Vec::with_capacity(row.taken_rewards.len());
    for entry in &row.taken_rewards {
        if entry.len() < 2 {
            healed_entries.push(entry.clone());
            continue;
        }
        let level = entry[0];
        let type_int = entry[1];
        let item_id = if entry.len() >= 3 { entry[2] } else { 0 };
        if item_id > 0 {
            pb_bp.TakenRewards.push(PbBattlePassReward {
                Level: level,
                Type: Some(bp_type_from_i32(type_int)),
                ItemId: item_id,
            });
            healed_entries.push(entry.clone());
            continue;
        }

        let resolved = resolve_reward_item_ids(bp_id, level, type_int);
        if resolved.is_empty() {
            healed_entries.push(entry.clone());
            continue;
        }
        repaired = true;
        for resolved_id in resolved {
            healed_entries.push(vec![level, type_int, resolved_id]);
            pb_bp.TakenRewards.push(PbBattlePassReward {
                Level: level,
                Type: Some(bp_type_from_i32(type_int)),
                ItemId: resolved_id,
            });
        }
    }

    if repaired {
        row.taken_rewards = healed_entries;
        sqlite_store::save_battle_pass(&row);
    }

    let resp = BattlePassResponse {
        ErrorCode: Some(ErrorCode::Success),
        BattlePass: Some(pb_bp),
    };
    AkiResult::ok(msg_id::BATTLE_PASS_RESPONSE, &resp)
}

pub fn handle_battle_pass_task_request(player_id: i32, _payload: &[u8]) -> AkiResult {
    let row = sqlite_store::get_or_create_battle_pass(player_id);
    let taken_set: HashSet<i32> = row.taken_tasks.into_iter().collect();
    let finished_set = sqlite_store::get_finished_bp_tasks(player_id);

    let mut resp = BattlePassTaskResponse {
        ErrorCode: Some(ErrorCode::Success),
        DayEnd: FAR_FUTURE,
        WeekEnd: FAR_FUTURE,
        ..Default::default()
    };

    for task in bin_data::get().bp_tasks.values() {
        let is_taken = taken_set.contains(&task.task_id);
        let is_finished = is_taken || finished_set.contains(&task.task_id) || task.update_type == 1;
        resp.Tasks.push(PbBattlePassTask {
            Id: task.task_id,
            Current: i32::from(is_finished),
            Target: 1,
            IsFinished: is_finished,
            IsTaken: is_taken,
        });
    }

    AkiResult::ok(msg_id::BATTLE_PASS_TASK_RESPONSE, &resp)
}

pub fn handle_battle_pass_take_reward_request(player_id: i32, payload: &[u8]) -> AkiResult {
    let req = BattlePassTakeRewardRequest::decode(payload).unwrap_or_default();
    let mut row = sqlite_store::get_or_create_battle_pass(player_id);
    let bp_id = row.bp_id.max(bin_data::get().bp_meta.latest_bp_id);
    let type_int = req.Type.map(|t| t as i32).unwrap_or(0);
    let mut grants = HashMap::new();

    let already_has_level_type = row
        .taken_rewards
        .iter()
        .any(|x| x.len() >= 2 && x[0] == req.Level && x[1] == type_int);
    let already_has_item = if req.ItemId > 0 {
        row.taken_rewards.iter().any(|x| {
            x.len() >= 3
                && x[0] == req.Level
                && x[1] == type_int
                && (x[2] == req.ItemId || x[2] == 0)
        })
    } else {
        already_has_level_type
    };

    if req.Level > 0 && req.Level <= row.level && !already_has_item {
        if req.ItemId > 0 {
            row.taken_rewards
                .push(vec![req.Level, type_int, req.ItemId]);
        } else {
            for resolved_id in resolve_reward_item_ids(bp_id, req.Level, type_int) {
                row.taken_rewards
                    .push(vec![req.Level, type_int, resolved_id]);
            }
        }
        sqlite_store::save_battle_pass(&row);
        if !already_has_level_type {
            collect_level_grants(row.bp_id, req.Level, type_int, &mut grants);
        }
    }

    AkiResult::ok(
        msg_id::BATTLE_PASS_TAKE_REWARD_RESPONSE,
        &BattlePassTakeRewardResponse {
            ErrorCode: Some(ErrorCode::Success),
        },
    )
    .with_grants(grants)
}

pub fn handle_battle_pass_take_all_reward_request(player_id: i32, _payload: &[u8]) -> AkiResult {
    let mut row = sqlite_store::get_or_create_battle_pass(player_id);
    let bp_id = row.bp_id.max(bin_data::get().bp_meta.latest_bp_id);
    let mut resp = BattlePassTakeAllRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
        ..Default::default()
    };
    let mut grants = HashMap::new();
    let max_type = if row.pay_status > 0 { 1 } else { 0 };

    for l in 1..=row.level {
        for t in 0..=max_type {
            if !row
                .taken_rewards
                .iter()
                .any(|x| x.len() >= 2 && x[0] == l && x[1] == t)
            {
                for item_id in resolve_reward_item_ids(bp_id, l, t) {
                    row.taken_rewards.push(vec![l, t, item_id]);
                    resp.TakenRewards.push(PbBattlePassReward {
                        Level: l,
                        Type: Some(bp_type_from_i32(t)),
                        ItemId: item_id,
                    });
                }
                collect_level_grants(row.bp_id, l, t, &mut grants);
            }
        }
    }

    sqlite_store::save_battle_pass(&row);
    AkiResult::ok(msg_id::BATTLE_PASS_TAKE_ALL_REWARD_RESPONSE, &resp).with_grants(grants)
}

pub fn handle_battle_pass_level_up_request(player_id: i32, payload: &[u8]) -> AkiResult {
    let req = BattlePassLevelUpRequest::decode(payload).unwrap_or_default();
    let mut row = sqlite_store::get_or_create_battle_pass(player_id);
    row.level = (row.level + req.Level.max(1)).clamp(1, 70);
    sqlite_store::save_battle_pass(&row);

    let notify = BattlePassExpUpdateNotify {
        Level: row.level,
        Exp: row.exp,
        WeeklyTotalExp: row.weekly_exp,
        ..Default::default()
    };

    AkiResult::ok(
        msg_id::BATTLE_PASS_LEVEL_UP_RESPONSE,
        &BattlePassLevelUpResponse {
            ErrorCode: Some(ErrorCode::Success),
        },
    )
    .with_push(msg_id::BATTLE_PASS_EXP_UPDATE_NOTIFY, &notify)
}

pub fn handle_battle_pass_recurring_take_request(_player_id: i32, _payload: &[u8]) -> AkiResult {
    AkiResult::ok(
        msg_id::BATTLE_PASS_RECURRING_TAKE_RESPONSE,
        &BattlePassRecurringTakeResponse {
            ErrorCode: Some(ErrorCode::Success),
        },
    )
}
