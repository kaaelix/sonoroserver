//! Achievement info, finish, and reward handlers matching `AchievementHandler.cs`.

use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

use sonoro_proto::PbMessage;
use sonoro_proto::enums::*;
use sonoro_proto::pb::*;
use sonoro_proto::registry::msg_id;

use crate::bin_data::{self, AchievementCfg};
use crate::ipc::AkiResult;
use crate::sqlite_store;

fn now_u32() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| (d.as_secs() as u32).max(1))
        .unwrap_or(1)
}

fn load_claimed_groups(player_id: i32) -> HashSet<i32> {
    let Some(raw) = sqlite_store::get_kv(player_id, "achievement", "claimed_groups") else {
        return HashSet::new();
    };
    serde_json::from_str::<Vec<i32>>(&raw)
        .map(|v| v.into_iter().collect())
        .unwrap_or_default()
}

fn save_claimed_groups(player_id: i32, groups: &HashSet<i32>) {
    let vec: Vec<i32> = groups.iter().copied().collect();
    if let Ok(json) = serde_json::to_string(&vec) {
        sqlite_store::set_kv(player_id, "achievement", "claimed_groups", &json);
    }
}

pub fn handle_achievement_info_request(player_id: i32, _payload: &[u8]) -> AkiResult {
    let entries = sqlite_store::get_achievements(player_id);
    let claimed_groups = load_claimed_groups(player_id);
    let cache = bin_data::get();

    let mut ach_by_group: HashMap<i32, Vec<&AchievementCfg>> = HashMap::new();
    let mut level_by_id: HashMap<i32, i32> = HashMap::new();
    for ach in &cache.achievements {
        level_by_id.insert(ach.id, ach.level.max(1));
        ach_by_group.entry(ach.group_id).or_default().push(ach);
    }

    let finish_count = entries.len() as i32;
    let star_count: i32 = entries
        .keys()
        .map(|id| *level_by_id.get(id).unwrap_or(&1))
        .sum();

    let mut resp = AchievementInfoResponse {
        AchievementFinishedStar: star_count,
        FinishedAchievementNum: finish_count,
        ..Default::default()
    };

    let now = now_u32();
    for grp in &cache.achievement_groups {
        if !grp.enable {
            continue;
        }
        let mut grp_entry = AchievementGroupEntry {
            Id: grp.id,
            FinishTime: 0,
            IsReceive: claimed_groups.contains(&grp.id),
        };
        let mut entry_list = Vec::new();
        if let Some(list) = ach_by_group.get(&grp.id) {
            for cfg in list {
                if let Some(&(progress, is_taken)) = entries.get(&cfg.id) {
                    grp_entry.FinishTime = 1;
                    entry_list.push(AchievementEntry {
                        Id: cfg.id,
                        FinishTime: now,
                        IsReceive: is_taken,
                        Progress: Some(AchievementProgress {
                            CurProgress: progress,
                            TotalProgress: 1,
                        }),
                    });
                }
            }
        }
        resp.AchievementGroupInfoList.push(AchievementGroupInfo {
            AchievementGroupEntry: Some(grp_entry),
            AchievementEntryList: entry_list,
        });
    }

    AkiResult::ok(msg_id::ACHIEVEMENT_INFO_RESPONSE, &resp)
}

pub fn handle_achievement_update_request(player_id: i32, _payload: &[u8]) -> AkiResult {
    let entries = sqlite_store::get_achievements(player_id);
    let now = now_u32();
    let mut resp = AchievementUpdateResponse {
        ErrorCode: Some(ErrorCode::Success),
        ..Default::default()
    };

    for (id, (progress, is_taken)) in entries {
        resp.AchievementEntryList.push(AchievementEntry {
            Id: id,
            FinishTime: now,
            IsReceive: is_taken,
            Progress: Some(AchievementProgress {
                CurProgress: progress,
                TotalProgress: 1,
            }),
        });
    }

    AkiResult::ok(msg_id::ACHIEVEMENT_UPDATE_RESPONSE, &resp)
}

pub fn handle_achievement_finish_request(player_id: i32, payload: &[u8]) -> AkiResult {
    let req = AchievementFinishRequest::decode(payload).unwrap_or_default();
    if req.Id <= 0 {
        return AkiResult::ok(
            msg_id::ACHIEVEMENT_FINISH_RESPONSE,
            &AchievementFinishResponse {
                ErrorCode: Some(ErrorCode::RequestParamError),
            },
        );
    }

    sqlite_store::finish_achievement(player_id, req.Id, 1);
    let now = now_u32();
    let entries = sqlite_store::get_achievements(player_id);

    let level_by_id: HashMap<i32, i32> = bin_data::get()
        .achievements
        .iter()
        .map(|a| (a.id, a.level.max(1)))
        .collect();
    let finish_count = entries.len() as i32;
    let star_count: i32 = entries
        .keys()
        .map(|id| *level_by_id.get(id).unwrap_or(&1))
        .sum();

    let notify = AchievementProgressNotify {
        AchievementEntry: Some(AchievementEntry {
            Id: req.Id,
            FinishTime: now,
            IsReceive: false,
            Progress: Some(AchievementProgress {
                CurProgress: 1,
                TotalProgress: 1,
            }),
        }),
    };
    let count_notify = AchievementCountChangeNotify {
        StarCount: star_count,
        FinishCount: finish_count,
    };

    AkiResult::ok(
        msg_id::ACHIEVEMENT_FINISH_RESPONSE,
        &AchievementFinishResponse {
            ErrorCode: Some(ErrorCode::Success),
        },
    )
    .with_push(msg_id::ACHIEVEMENT_PROGRESS_NOTIFY, &notify)
    .with_push(msg_id::ACHIEVEMENT_COUNT_CHANGE_NOTIFY, &count_notify)
}

fn try_claim_single(
    id: i32,
    is_group_id: bool,
    entries: &HashMap<i32, (i32, bool)>,
    claimed_groups: &mut HashSet<i32>,
    out_grants: &mut HashMap<i32, i32>,
    newly_taken_ids: &mut Vec<i32>,
) -> bool {
    let cache = bin_data::get();
    if is_group_id {
        let Some(grp) = cache.achievement_groups.iter().find(|g| g.id == id) else {
            return false;
        };
        if !claimed_groups.insert(id) {
            return false;
        }
        let mut drops = cache.resolve_drop(grp.drop_id);
        if drops.is_empty() {
            drops.push((3, 20));
        }
        for (iid, cnt) in drops {
            *out_grants.entry(iid).or_insert(0) += cnt;
        }
        true
    } else {
        let Some(cfg) = cache.achievements.iter().find(|a| a.id == id) else {
            return false;
        };
        let Some(&(_, is_taken)) = entries.get(&id) else {
            return false;
        };
        if is_taken {
            return false;
        }
        newly_taken_ids.push(id);
        let mut drops = cache.resolve_drop(cfg.override_drop_id);
        if drops.is_empty() {
            let astrite = match cfg.level.clamp(1, 3) {
                1 => 5,
                2 => 10,
                _ => 20,
            };
            drops.push((3, astrite));
        }
        for (iid, cnt) in drops {
            *out_grants.entry(iid).or_insert(0) += cnt;
        }
        true
    }
}

pub fn handle_achievement_receive_request(player_id: i32, payload: &[u8]) -> AkiResult {
    let req = AchievementReceiveRequest::decode(payload).unwrap_or_default();
    let entries = sqlite_store::get_achievements(player_id);
    let mut claimed_groups = load_claimed_groups(player_id);

    let mut grants = HashMap::new();
    let mut taken_ids = Vec::new();
    if !try_claim_single(
        req.Id,
        req.IsGroupId,
        &entries,
        &mut claimed_groups,
        &mut grants,
        &mut taken_ids,
    ) {
        return AkiResult::ok(
            msg_id::ACHIEVEMENT_RECEIVE_RESPONSE,
            &AchievementReceiveResponse {
                ErrorCode: Some(ErrorCode::RequestParamError),
                ..Default::default()
            },
        );
    }

    if !taken_ids.is_empty() {
        sqlite_store::mark_achievements_taken(player_id, &taken_ids);
    }
    if req.IsGroupId {
        save_claimed_groups(player_id, &claimed_groups);
    }

    let mut resp = AchievementReceiveResponse {
        ErrorCode: Some(ErrorCode::Success),
        ..Default::default()
    };
    for (&k, &v) in &grants {
        resp.ItemMap.insert(k, v);
    }

    AkiResult::ok(msg_id::ACHIEVEMENT_RECEIVE_RESPONSE, &resp).with_grants(grants)
}

pub fn handle_mul_achievement_receive_request(player_id: i32, payload: &[u8]) -> AkiResult {
    let req = MulAchievementReceiveRequest::decode(payload).unwrap_or_default();
    let entries = sqlite_store::get_achievements(player_id);
    let mut claimed_groups = load_claimed_groups(player_id);

    let mut grants = HashMap::new();
    let mut taken_ids = Vec::new();
    let mut resp = MulAchievementReceiveResponse {
        ErrorCode: Some(ErrorCode::Success),
        ..Default::default()
    };
    let mut groups_changed = false;

    for aid in req.AchievementIds {
        if try_claim_single(
            aid,
            false,
            &entries,
            &mut claimed_groups,
            &mut grants,
            &mut taken_ids,
        ) {
            resp.AchievementIds.push(aid);
        }
    }
    for gid in req.AchievementGroupIds {
        if try_claim_single(
            gid,
            true,
            &entries,
            &mut claimed_groups,
            &mut grants,
            &mut taken_ids,
        ) {
            resp.AchievementGroupIds.push(gid);
            groups_changed = true;
        }
    }

    if !taken_ids.is_empty() {
        sqlite_store::mark_achievements_taken(player_id, &taken_ids);
    }
    if groups_changed {
        save_claimed_groups(player_id, &claimed_groups);
    }

    AkiResult::ok(msg_id::MUL_ACHIEVEMENT_RECEIVE_RESPONSE, &resp).with_grants(grants)
}
