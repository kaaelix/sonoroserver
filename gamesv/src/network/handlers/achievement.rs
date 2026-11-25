use std::collections::BTreeMap;

use sonoro_gamesv_fs::progress_info::ProgressInfo;
use sonoro_gamesv_logic::helpers::{craft, energy};
use sonoro_proto::enums::ErrorCode;
use sonoro_proto::pb::{
    AchievementCountChangeNotify, AchievementEntry, AchievementFinishRequest,
    AchievementFinishResponse, AchievementGroupEntry, AchievementGroupInfo,
    AchievementGroupProgressNotify, AchievementInfoRequest, AchievementInfoResponse,
    AchievementProgress, AchievementProgressNotify, AchievementReceiveRequest,
    AchievementReceiveResponse, MulAchievementReceiveRequest, MulAchievementReceiveResponse,
    UpdateAchievementInfoRequest, UpdateAchievementInfoResponse,
};

use super::State;

fn compute_achievement_totals(state: &State) -> (i32, i32) {
    let mut finish_count = 0i32;
    let mut star_count = 0i32;
    for rec in &state.player_components.progress.info.achievement.entries {
        if rec.finish_time == 0 {
            continue;
        }
        finish_count += 1;
        if let Some(cfg) = state.assets.tables.achievement.get_data_by_id(&rec.id) {
            star_count += cfg.Level.max(1);
        }
    }
    (finish_count, star_count)
}

pub fn entry_to_pb(rec: &sonoro_gamesv_fs::progress_info::AchievementRecord) -> AchievementEntry {
    AchievementEntry {
        Id: rec.id,
        FinishTime: rec.finish_time,
        IsReceive: rec.received,
        Progress: Some(AchievementProgress {
            CurProgress: rec.cur_progress,
            TotalProgress: rec.total_progress,
        }),
    }
}

pub fn handle_achievement_info_request(
    state: &mut State,
    _req: AchievementInfoRequest,
) -> Result<AchievementInfoResponse, String> {
    let mut groups = Vec::new();
    let progress = &state.player_components.progress.info;
    for grp in &state.assets.tables.achievement_group.items {
        if !grp.Enable {
            continue;
        }
        let mut entries = Vec::new();
        let mut all_finished = false;
        for ach in &state.assets.tables.achievement.items {
            if ach.GroupId != grp.Id {
                continue;
            }
            if let Some(rec) = progress
                .achievement
                .entries
                .iter()
                .find(|r| r.id == ach.Id)
            {
                entries.push(entry_to_pb(rec));
                if rec.finish_time > 0 {
                    all_finished = true;
                }
            }
        }
        let group_received = progress.achievement.claimed_groups.contains(&grp.Id);
        groups.push(AchievementGroupInfo {
            AchievementGroupEntry: Some(AchievementGroupEntry {
                Id: grp.Id,
                FinishTime: u32::from(all_finished),
                IsReceive: group_received,
            }),
            AchievementEntryList: entries,
        });
    }

    let (finish_count, star_count) = compute_achievement_totals(state);
    Ok(AchievementInfoResponse {
        AchievementGroupInfoList: groups,
        AchievementFinishedStar: star_count,
        FinishedAchievementNum: finish_count,
    })
}

pub fn handle_update_achievement_info_request(
    state: &mut State,
    _req: UpdateAchievementInfoRequest,
) -> Result<UpdateAchievementInfoResponse, String> {
    let list = state
        .player_components
        .progress
        .info
        .achievement
        .entries
        .iter()
        .map(entry_to_pb)
        .collect();
    Ok(UpdateAchievementInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        AchievementEntryList: list,
    })
}

pub fn handle_achievement_finish_request(
    state: &mut State,
    req: AchievementFinishRequest,
) -> Result<AchievementFinishResponse, String> {
    if state
        .assets
        .tables
        .achievement
        .get_data_by_id(&req.Id)
        .is_none()
    {
        return Ok(AchievementFinishResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    }
    let now_u32 = energy::now_seconds().clamp(1, i64::from(u32::MAX)) as u32;
    let entry_pb = {
        let pc = &mut state.player_components.progress;
        let rec = pc.info.finish_achievement(req.Id, now_u32);
        let pb = entry_to_pb(rec);
        pc.save(&state.fs).map_err(|e| e.to_string())?;
        pb
    };
    let _ = state.conn.push_msg(
        AchievementProgressNotify::MSG_ID,
        &AchievementProgressNotify {
            AchievementEntry: Some(entry_pb),
        },
    );
    let (finish_count, star_count) = compute_achievement_totals(state);
    let _ = state.conn.push_msg(
        AchievementCountChangeNotify::MSG_ID,
        &AchievementCountChangeNotify {
            StarCount: star_count,
            FinishCount: finish_count,
        },
    );
    Ok(AchievementFinishResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

fn claim_single_achievement(
    state: &mut State,
    id: i32,
    is_group: bool,
    out_items: &mut BTreeMap<i32, i32>,
) -> Result<bool, String> {
    if is_group {
        let Some(grp) = state
            .assets
            .tables
            .achievement_group
            .get_data_by_id(&id)
            .map(|c| c.into_owned())
        else {
            return Ok(false);
        };
        if state
            .player_components
            .progress
            .info
            .achievement
            .claimed_groups
            .contains(&id)
        {
            return Ok(false);
        }
        ProgressInfo::append_unique_id(
            &mut state.player_components.progress.info.achievement.claimed_groups,
            id,
        );
        let mut cctx = state.craft_context();
        craft::grant_drop_and_collect_items(&mut cctx, grp.DropId, 3, 20, 19101, out_items)
            .map_err(|e| e.to_string())?;
        let now_u32 = energy::now_seconds().clamp(1, i64::from(u32::MAX)) as u32;
        let _ = state.conn.push_msg(
            AchievementGroupProgressNotify::MSG_ID,
            &AchievementGroupProgressNotify {
                AchievementGroupEntry: Some(AchievementGroupEntry {
                    Id: id,
                    FinishTime: now_u32,
                    IsReceive: true,
                }),
            },
        );
        return Ok(true);
    }

    let Some(cfg) = state
        .assets
        .tables
        .achievement
        .get_data_by_id(&id)
        .map(|c| c.into_owned())
    else {
        return Ok(false);
    };
    let entry_pb = {
        let Some(rec) = state
            .player_components
            .progress
            .info
            .find_achievement_mut(id)
        else {
            return Ok(false);
        };
        if rec.finish_time == 0 || rec.received {
            return Ok(false);
        }
        rec.received = true;
        entry_to_pb(rec)
    };
    let fallback_astrite = cfg.Level.max(1) * 5;
    let mut cctx = state.craft_context();
    craft::grant_drop_and_collect_items(
        &mut cctx,
        cfg.OverrideDropId,
        3,
        fallback_astrite,
        19100,
        out_items,
    )
    .map_err(|e| e.to_string())?;
    let _ = state.conn.push_msg(
        AchievementProgressNotify::MSG_ID,
        &AchievementProgressNotify {
            AchievementEntry: Some(entry_pb),
        },
    );
    Ok(true)
}

pub fn handle_achievement_receive_request(
    state: &mut State,
    req: AchievementReceiveRequest,
) -> Result<AchievementReceiveResponse, String> {
    let mut out_items = BTreeMap::new();
    if !claim_single_achievement(state, req.Id, req.IsGroupId, &mut out_items)? {
        return Ok(AchievementReceiveResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
            ..Default::default()
        });
    }
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(AchievementReceiveResponse {
        ErrorCode: Some(ErrorCode::Success),
        ItemMap: out_items,
        ..Default::default()
    })
}

pub fn handle_mul_achievement_receive_request(
    state: &mut State,
    req: MulAchievementReceiveRequest,
) -> Result<MulAchievementReceiveResponse, String> {
    let mut out_items = BTreeMap::new();
    let mut claimed_ids = Vec::new();
    let mut claimed_groups = Vec::new();
    let mut changed = false;

    for aid in req.AchievementIds {
        if claim_single_achievement(state, aid, false, &mut out_items)? {
            claimed_ids.push(aid);
            changed = true;
        }
    }
    for gid in req.AchievementGroupIds {
        if claim_single_achievement(state, gid, true, &mut out_items)? {
            claimed_groups.push(gid);
            changed = true;
        }
    }
    if changed {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }

    Ok(MulAchievementReceiveResponse {
        AchievementIds: claimed_ids,
        ErrorCode: Some(ErrorCode::Success),
        AchievementGroupIds: claimed_groups,
    })
}
