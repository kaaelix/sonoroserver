use std::collections::BTreeMap;

use serde_json::Value;
use sonoro_gamesv_data::Assets;
use sonoro_gamesv_logic::component::player::PlayerProgressComponent;
use sonoro_gamesv_logic::helpers::condition::{self, EvalContext};
use sonoro_proto::enums::{
    ActivityTaskState, ActivityType, BossRushRewardClaimStatus, ChallengeState,
    ConditionTaskState, DailyAdventureTaskState, DangoBrawlChallengeState,
    DangoMonopolyTaskState, GatherActivityTaskState, MotorParkourRewardState,
    MowTowerRewardStatus, PhantomCollectRewardType, RhythmTaskTypePb, RoverRogueGrade, SignState,
};
use sonoro_proto::pb::*;

fn j_i32(obj: &serde_json::Map<String, Value>, key: &str) -> i32 {
    obj.get(key)
        .and_then(Value::as_i64)
        .and_then(|v| i32::try_from(v).ok())
        .unwrap_or(0)
}

fn j_i32_vec(obj: &serde_json::Map<String, Value>, key: &str) -> Vec<i32> {
    obj.get(key)
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_i64().and_then(|n| i32::try_from(n).ok()))
                .collect()
        })
        .unwrap_or_default()
}

fn rows_for_activity(assets: &Assets, table: &str, act_id: i32) -> Vec<serde_json::Map<String, Value>> {
    let raw = assets.raw_json_cached(table);
    let mut matched = Vec::new();
    let mut all = Vec::new();
    for v in raw.iter() {
        let Some(obj) = v.as_object() else {
            continue;
        };
        let row_act = obj
            .get("ActivityId")
            .or_else(|| obj.get("Activity"))
            .and_then(Value::as_i64)
            .and_then(|n| i32::try_from(n).ok())
            .unwrap_or(0);
        if row_act == act_id {
            matched.push(obj.clone());
        } else if row_act == 0 {
            all.push(obj.clone());
        }
    }
    if matched.is_empty() { all } else { matched }
}

fn make_cond_task(
    id: i32,
    taken: bool,
    done: bool,
    cur: i32,
    target: i32,
) -> ConditionTask {
    let tgt = target.max(1);
    let current = if taken || done { tgt } else { cur.min(tgt) };
    let status = if taken {
        ConditionTaskState::ConditionTaskTaken
    } else if done || current >= tgt {
        ConditionTaskState::ConditionTaskFinish
    } else {
        ConditionTaskState::ConditionTaskRunning
    };
    ConditionTask {
        Id: id,
        Current: current,
        Target: tgt,
        Status: Some(status),
    }
}

fn make_act_task(id: i32, taken: bool, done: bool, cur: i32, target: i32) -> ActivityTask {
    let tgt = target.max(1);
    let current = if taken || done { tgt } else { cur.min(tgt) };
    let status = if taken {
        ActivityTaskState::ActivityTaskTaken
    } else if done || current >= tgt {
        ActivityTaskState::ActivityTaskFinish
    } else {
        ActivityTaskState::ActivityTaskRunning
    };
    ActivityTask {
        Id: id,
        Current: current,
        Target: tgt,
        Status: Some(status),
        ..Default::default()
    }
}

fn claim_matches_id(claims: &[String], prefix: &str, id: i32) -> bool {
    let prefixes: &[&str] = match prefix {
        "gather:" => &["gather:", "gather_rw:"],
        "role_trial:" => &["role_trial:", "trial_role_rw:"],
        "skin_trial:" => &["skin_trial:", "trial_skin_rw:"],
        "harvest_pt:" => &["harvest_pt:", "harv_pt:"],
        "harvest_lv:" => &["harvest_lv:", "harv_lv:"],
        "daily_adv:task:" => &["daily_adv:task:", "dadv_task:"],
        "daily_adv:pt:" => &["daily_adv:pt:", "dadv_pt:"],
        "longshan:task:" => &["longshan:task:", "longshan_task:"],
        "longshan:score:" => &["longshan:score:", "longshan_sc:"],
        "td:inst:" => &["td:inst:", "td_inst:"],
        "td:score:" => &["td:score:", "td_score:"],
        "time_pt:" => &["time_pt:", "tpa_rw:", "tpa_brw:"],
        "risk:inst:" => &["risk:inst:", "rh_inst:"],
        "risk:score:" => &["risk:score:", "rh_score:"],
        "blackcoast:task:" => &["blackcoast:task:", "bc_task_rw:"],
        "blackcoast:rw:" => &["blackcoast:rw:", "bc_act_rw:"],
        "mow:clm:" => &["mow:clm:", "mow_rw:"],
        "through_train:" => &["through_train:", "throughtrain:", "direct_train:fin:"],
        "spring_sign:" => &["spring_sign:", "ssign_rw:"],
        "spring_sign:skin:" => &["spring_sign:skin:", "ssign_skin:"],
        "farmgold:pt:" => &["farmgold:pt:", "fg_pt:"],
        "farmgold:lv:" => &["farmgold:lv:", "fg_lp:"],
        "dango_mono:task:" => &["dango_mono:task:", "dmono_task_rw:"],
        "avignon:task:" => &["avignon:task:", "avignon_rw:"],
        "floro:task:" => &["floro:task:", "fr_task:"],
        "trapdef:rw:" => &["trapdef:rw:", "trap_rw:", "trap_sp_rw:"],
        "moon_phase:" => &["moon_phase:", "moon_phase_rw:"],
        "honami:col:" => &["honami:col:", "honami_col:"],
        "honami:perm:" => &["honami:perm:", "honami_rt:"],
        "honami:lim:" => &["honami:lim:", "honami_lt:"],
        "honami:score:" => &["honami:score:", "honami_score:"],
        "surv:task:" => &["surv:task:", "surv_norm:"],
        "photofight:" => &["photofight:", "pf_task:"],
        "wuwu:pkg:" => &["wuwu:pkg:", "wuwu_pkg:"],
        "wuwu:task:" => &["wuwu:task:", "wuwu_task:"],
        "infr:task:" => &["infr:task:", "infr:theme:"],
        "rhythm:task:" => &["rhythm:task:", "rhythm_task:"],
        "kurotato:lim:" => &["kurotato:lim:", "kurotato_lt:"],
        "kurotato:res:" => &["kurotato:res:", "kurotato_rt:"],
        "pinball:task:" => &["pinball:task:", "pinball_task:"],
        "edge:rw:" => &["edge:rw:", "er_task:"],
        "linkage:norm:" => &["linkage:norm:", "link_rw:", "link_ci:"],
        "role_gift:" => &["role_gift:", "role_gift_clm:"],
        "realm:task:" => &["realm:task:", "realmbetween:task:"],
        "db:score:" => &["db:score:", "db_score:"],
        "qingxiao:" => &["qingxiao:", "qingxiao_rw:"],
        "giant:rw:" => &["giant:rw:", "giant_sv:"],
        "regress:task:" => &["regress:task:", "reg_task:"],
        "trackmoon:task:" => &["trackmoon:task:", "trackmoon:target:", "trackmoon:act:"],
        _ => {
            let exact = format!("{prefix}{id}");
            return claims.iter().any(|c| c == &exact);
        }
    };
    prefixes.iter().any(|p| {
        let exact = format!("{p}{id}");
        claims.iter().any(|c| c == &exact)
    })
}

fn cond_done(cond_ctx: Option<&EvalContext<'_>>, cond_group_id: i32) -> bool {
    cond_group_id <= 0
        || cond_ctx.is_some_and(|c| condition::eval_condition_group(c, cond_group_id))
}

fn cond_task_list_from_rows(
    rows: &[serde_json::Map<String, Value>],
    progress: &PlayerProgressComponent,
    cond_ctx: Option<&EvalContext<'_>>,
    claim_prefix: &str,
) -> Vec<ConditionTask> {
    let claims = &progress.info.dynamic.claims;
    rows.iter()
        .filter_map(|obj| {
            let id = j_i32(obj, "Id").max(j_i32(obj, "TaskId"));
            if id <= 0 {
                return None;
            }
            let cg = j_i32(obj, "ConditionGroup").max(j_i32(obj, "ConditionGroupId"));
            let taken = claim_matches_id(claims, claim_prefix, id);
            let done = taken || (cg > 0 && cond_done(cond_ctx, cg));
            Some(make_cond_task(id, taken, done, i32::from(done), 1))
        })
        .collect()
}

fn act_task_list_from_rows(
    rows: &[serde_json::Map<String, Value>],
    progress: &PlayerProgressComponent,
    cond_ctx: Option<&EvalContext<'_>>,
    claim_prefix: &str,
) -> Vec<ActivityTask> {
    let claims = &progress.info.dynamic.claims;
    rows.iter()
        .filter_map(|obj| {
            let id = j_i32(obj, "Id").max(j_i32(obj, "TaskId"));
            if id <= 0 {
                return None;
            }
            let cg = j_i32(obj, "ConditionGroup").max(j_i32(obj, "ConditionGroupId"));
            let taken = claim_matches_id(claims, claim_prefix, id);
            let done = taken || (cg > 0 && cond_done(cond_ctx, cg));
            Some(make_act_task(id, taken, done, i32::from(done), 1))
        })
        .collect()
}

pub fn rogue_season_id_for_activity(assets: &Assets, activity_id: i32) -> i32 {
    for v in assets.raw_json_cached("RogueSeason").iter() {
        if let Some(obj) = v.as_object()
            && j_i32(obj, "ActivityId") == activity_id
        {
            let id = j_i32(obj, "Id");
            if id > 0 {
                return id;
            }
        }
    }
    match activity_id {
        100600001 => 1,
        100600002 => 2,
        100600003 => 3,
        100600004 => 4,
        102600001 => 5,
        100600005 => 6,
        100600006 => 7,
        _ => 1,
    }
}

pub fn build_activity_data(
    act_type: ActivityType,
    id: i32,
    now_s: i64,
    assets: &Assets,
    progress_comp: &PlayerProgressComponent,
    cond_ctx: Option<&EvalContext<'_>>,
) -> Option<ActivityDataDataOneof> {
    let info = &progress_comp.info;
    let claims = &info.dynamic.claims;
    let begin_s = (now_s - 86_400).max(1);
    let end_s = now_s + 30 * 86_400;

    Some(match act_type {
        ActivityType::Parkour => {
            let challenges = assets
                .raw_json_cached("ParkourChallenge")
                .iter()
                .filter_map(|v| v.as_object())
                .filter_map(|o| {
                    let cid = j_i32(o, "Id");
                    (cid > 0).then_some(ParkourActivityChallenge {
                        ChallengeId: cid,
                        BeginTime: begin_s,
                        EndTime: end_s,
                    })
                })
                .collect();
            ActivityDataDataOneof::ParkourActivity(ParkourActivity {
                Challenges: challenges,
            })
        }
        ActivityType::GatherActivity => {
            let tasks = assets
                .raw_json_cached("GatherActivity")
                .iter()
                .filter_map(|v| v.as_object())
                .filter_map(|o| {
                    let tid = j_i32(o, "Id");
                    if tid <= 0 {
                        return None;
                    }
                    let play_task = j_i32(o, "PlayTask");
                    let taken = claim_matches_id(claims, "gather:", tid);
                    let done = taken
                        || (play_task > 0
                            && cond_ctx.is_some_and(|c| {
                                c.quest_comp.info.finished_quests.contains(&play_task)
                            }));
                    Some(GatherTaskDoneInfo {
                        TaskId: tid,
                        State: Some(if taken {
                            GatherActivityTaskState::GatherTakeReward
                        } else if done {
                            GatherActivityTaskState::GatherDone
                        } else {
                            GatherActivityTaskState::GatherRunning
                        }),
                    })
                })
                .collect();
            ActivityDataDataOneof::GatherActivityInfo(GatherActivityInfo {
                GatherTaskDoneInfo: tasks,
            })
        }
        ActivityType::DoubleInstanceRewardActivity => {
            let used = info.get_key_int(&format!("double_rwd:{id}")) as i32;
            ActivityDataDataOneof::DoubleInstActivityReward(DoubleInstActivityReward {
                GetDoubleInstRwdCount: used,
            })
        }
        ActivityType::RoleTrialActivity => {
            let mut tasks = Vec::new();
            let role_ids = assets
                .raw_json_cached("RoleTrialActivity")
                .iter()
                .filter_map(|v| v.as_object())
                .find(|o| j_i32(o, "Id") == id)
                .map(|o| j_i32_vec(o, "RoleIdList"))
                .unwrap_or_default();
            for rid in role_ids {
                let claimed = claim_matches_id(claims, "role_trial:", rid);
                let cleared = claimed || info.get_key_int(&format!("trial:pass:{rid}")) > 0;
                tasks.push(RoleTrialTask {
                    RoleId: rid,
                    ChallengeState: Some(if claimed {
                        ChallengeState::Finish
                    } else if cleared {
                        ChallengeState::WaitTakeReward
                    } else {
                        ChallengeState::Running
                    }),
                    BeginOpenTime: begin_s,
                    EndOpenTime: end_s,
                });
            }
            ActivityDataDataOneof::RoleTrialInfoActivity(RoleTrialInfoActivity {
                RoleTrialTask: tasks,
            })
        }
        ActivityType::RoleSkinTrialActivity => {
            let mut items = Vec::new();
            let ids = assets
                .raw_json_cached("RoleSkinTrialActivity")
                .iter()
                .filter_map(|v| v.as_object())
                .find(|o| j_i32(o, "Id") == id)
                .map(|o| j_i32_vec(o, "RoleIdList"))
                .unwrap_or_default();
            for sid in ids {
                let claimed = claim_matches_id(claims, "skin_trial:", sid);
                let cleared = claimed || info.get_key_int(&format!("skin_trial:pass:{sid}")) > 0;
                items.push(RoleSkinTrialContentData {
                    Id: sid,
                    ChallengeState: Some(if claimed {
                        ChallengeState::Finish
                    } else if cleared {
                        ChallengeState::WaitTakeReward
                    } else {
                        ChallengeState::Running
                    }),
                });
            }
            ActivityDataDataOneof::RoleSkinTrialActivity(RoleSkinTrialActivity {
                RoleSkinTrialContentData: items,
            })
        }
        ActivityType::Harvest => {
            let pts = rows_for_activity(assets, "RiskHarvestScoreReward", id)
                .iter()
                .filter_map(|o| {
                    let rid = j_i32(o, "Id");
                    (rid > 0).then_some(HarvestPointReward {
                        Id: rid,
                        State: Some(if claim_matches_id(claims, "harvest_pt:", rid) {
                            SignState::IsReceive
                        } else {
                            SignState::Unlock
                        }),
                    })
                })
                .collect();
            let lvs = rows_for_activity(assets, "RiskHarvestInst", id)
                .iter()
                .filter_map(|o| {
                    let lid = j_i32(o, "Id");
                    (lid > 0).then_some(HarvestLevelReward {
                        Id: lid,
                        StartTime: begin_s as i32,
                        IsOpen: true,
                        Points: info.get_key_int(&format!("harvest:score:{lid}")) as i32,
                        Diff: j_i32(o, "Difficulty"),
                        State: Some(if claim_matches_id(claims, "harvest_lv:", lid) {
                            SignState::IsReceive
                        } else {
                            SignState::Unlock
                        }),
                    })
                })
                .collect();
            ActivityDataDataOneof::HarvestActivity(HarvestActivity {
                HarvestPointRewards: pts,
                HarvestLevelRewards: lvs,
            })
        }
        ActivityType::PhantomCollect => {
            let mut rewards = Vec::new();
            for o in rows_for_activity(assets, "PhantomCollectActivity", id) {
                let phantoms = j_i32_vec(&o, "Phantoms");
                let claimed = claim_matches_id(claims, "phantom_col:", id);
                rewards.push(PhantomCollectReward {
                    Type: Some(PhantomCollectRewardType::PhantomsCollect),
                    Data: Some(PhantomCollectRewardDataOneof::Progress(
                        PhantomCollectProgress { Phantoms: phantoms },
                    )),
                    State: Some(if claimed {
                        SignState::IsReceive
                    } else {
                        SignState::Unlock
                    }),
                });
            }
            ActivityDataDataOneof::PhantomCollectActivity(PhantomCollectActivity {
                PhantomCollectRewards: rewards,
            })
        }
        ActivityType::DailyAdventureActivity => {
            let task_ids = assets
                .raw_json_cached("DailyAdventureActivity")
                .iter()
                .filter_map(|v| v.as_object())
                .find(|o| j_i32(o, "ActivityId") == id)
                .map(|o| j_i32_vec(o, "TaskList"))
                .unwrap_or_default();
            let tasks = assets
                .raw_json_cached("DailyAdventureTask")
                .iter()
                .filter_map(|v| v.as_object())
                .filter(|o| task_ids.is_empty() || task_ids.contains(&j_i32(o, "TaskId")))
                .filter_map(|o| {
                    let tid = j_i32(o, "TaskId");
                    if tid <= 0 {
                        return None;
                    }
                    let taken = claim_matches_id(claims, "daily_adv:task:", tid);
                    let done = taken || info.get_key_int(&format!("daily_adv:prog:{tid}")) > 0;
                    Some(DailyAdventureActivityTask {
                        Id: tid,
                        Current: i32::from(done),
                        Target: 1,
                        Status: Some(if taken {
                            DailyAdventureTaskState::DailyAdventureTaskTaken
                        } else if done {
                            DailyAdventureTaskState::DailyAdventureTaskFinish
                        } else {
                            DailyAdventureTaskState::DailyAdventureTaskRunning
                        }),
                    })
                })
                .collect();
            let mut taken_pts = Vec::new();
            for o in assets
                .raw_json_cached("DailyAdventurePoint")
                .iter()
                .filter_map(|v| v.as_object())
            {
                let pid = j_i32(o, "Id");
                if pid > 0 && claim_matches_id(claims, "daily_adv:pt:", pid) {
                    taken_pts.push(pid);
                }
            }
            ActivityDataDataOneof::DailyAdventureActivityData(DailyAdventureActivityData {
                DailyAdventureActivityTasks: tasks,
                PtRewardTaken: taken_pts,
            })
        }
        ActivityType::LongShanMainActivity => {
            let all_tasks = assets.raw_json_cached("LongShanTask");
            let stages = rows_for_activity(assets, "LongShanStage", id)
                .iter()
                .filter_map(|st| {
                    let sid = j_i32(st, "Id");
                    if sid <= 0 {
                        return None;
                    }
                    let unlock_cg = j_i32(st, "OpenConditionId");
                    let can_unlock = cond_done(cond_ctx, unlock_cg);
                    let stage_tasks = all_tasks
                        .iter()
                        .filter_map(|v| v.as_object())
                        .filter(|t| j_i32(t, "StageId") == sid)
                        .filter_map(|t| {
                            let tid = j_i32(t, "TaskId");
                            if tid <= 0 {
                                return None;
                            }
                            let taken = claim_matches_id(claims, "longshan:task:", tid);
                            let done = taken || can_unlock;
                            Some(LongShanMainTaskData {
                                Id: tid,
                                Current: i32::from(done),
                                Target: 1,
                                IsFinished: done,
                                IsTaken: taken,
                                Unlock: can_unlock,
                                FinishConditions: Vec::new(),
                                ConditionId: 0,
                                ConditionGroupId: 0,
                                UnlockConditionFinish: can_unlock,
                            })
                        })
                        .collect();
                    Some(LongShanMainData {
                        Id: sid,
                        Tasks: stage_tasks,
                        CanUnlock: can_unlock,
                        BeginOpenTime: begin_s,
                        EndOpenTime: end_s,
                    })
                })
                .collect();
            let scored = rows_for_activity(assets, "LongShanScoreReward", id)
                .iter()
                .filter_map(|o| {
                    let rid = j_i32(o, "Id");
                    (rid > 0 && claim_matches_id(claims, "longshan:score:", rid)).then_some(rid)
                })
                .collect();
            ActivityDataDataOneof::ActivityLongShanMain(ActivityLongShanMain {
                StageData: stages,
                ScoreRewardedId: scored,
            })
        }
        ActivityType::BossRushActivity => {
            let score = info.get_key_int("bossrush:score").max(0) as i32;
            let level_details = rows_for_activity(assets, "BossRushActivity", id)
                .iter()
                .filter_map(|o| {
                    let inst_id = j_i32(o, "InstId");
                    if inst_id <= 0 {
                        return None;
                    }
                    let rid = j_i32(o, "Id");
                    let lv_score = info.get_key_int(&format!("bossrush:score:{inst_id}")).max(0) as i32;
                    let req_score = j_i32(o, "RewardScore");
                    let claimed = claim_matches_id(claims, "bossrush:clm:", rid);
                    Some(LevelInfo {
                        InstId: inst_id,
                        StartTime: begin_s as i32,
                        IsOpen: true,
                        Score: lv_score.max(score),
                        LevelRewardClaimStatus: Some(if claimed {
                            BossRushRewardClaimStatus::Claimed
                        } else if lv_score >= req_score && req_score > 0 {
                            BossRushRewardClaimStatus::Claimable
                        } else {
                            BossRushRewardClaimStatus::Incomplete
                        }),
                        ..Default::default()
                    })
                })
                .collect();
            let reward_info = rows_for_activity(assets, "BossRushScore", id)
                .iter()
                .filter_map(|o| {
                    let rid = j_i32(o, "Id");
                    if rid <= 0 {
                        return None;
                    }
                    let need = j_i32(o, "Score");
                    let claimed = claim_matches_id(claims, "bossrush:lscore:", rid);
                    Some(BossRushScoreRewardData {
                        RewardDataId: rid,
                        State: Some(if claimed {
                            BossRushRewardClaimStatus::Claimed
                        } else if score >= need && need > 0 {
                            BossRushRewardClaimStatus::Claimable
                        } else {
                            BossRushRewardClaimStatus::Incomplete
                        }),
                    })
                })
                .collect();
            let tasks = act_task_list_from_rows(
                &rows_for_activity(assets, "BossRushTaskConfig", id),
                progress_comp,
                cond_ctx,
                "bossrush:task:",
            );
            let unlocked_buffs = assets
                .raw_json_cached("BossRushBuff")
                .iter()
                .filter_map(|v| v.as_object().map(|o| j_i32(o, "Id")))
                .filter(|&bid| bid > 0)
                .collect();
            ActivityDataDataOneof::BossRushActivityData(BossRushActivityData {
                LevelDetailInfo: level_details,
                RewardInfo: reward_info,
                UnlockedBuffIndices: unlocked_buffs,
                TaskProgressReward: tasks,
            })
        }
        ActivityType::TurnTableActivity => {
            let awards = rows_for_activity(assets, "TurntableAwards", id);
            let act_round_pfx = format!("turntable_rw:{id}:");
            let canon_round_pfx = format!("turntable:rw:{id}:");
            let claimed_rewards: Vec<i32> = awards
                .iter()
                .filter_map(|o| {
                    let aid = j_i32(o, "Id");
                    let suffix = format!(":{aid}");
                    let claimed = aid > 0
                        && (claim_matches_id(claims, "turntable:rw:", aid)
                            || claims.iter().any(|c| {
                                (c.starts_with(&act_round_pfx) || c.starts_with(&canon_round_pfx))
                                    && c.ends_with(&suffix)
                            }));
                    claimed.then_some(aid)
                })
                .collect();
            let tasks = act_task_list_from_rows(
                &rows_for_activity(assets, "TurntableTask", id),
                progress_comp,
                cond_ctx,
                "turntable:task:",
            );
            ActivityDataDataOneof::ActivityTurnTableData(ActivityTurnTableData {
                IsAllFinish: !awards.is_empty() && claimed_rewards.len() >= awards.len(),
                GroupId: id,
                Rewards: claimed_rewards,
                TurntableTasks: tasks,
            })
        }
        ActivityType::TrackMoonActivity => {
            let tasks = act_task_list_from_rows(
                &rows_for_activity(assets, "TrackMoonTarget", id),
                progress_comp,
                cond_ctx,
                "trackmoon:task:",
            );
            ActivityDataDataOneof::TrackMoonActivityTaskData(ActivityTaskData {
                ActivityTasks: tasks,
            })
        }
        ActivityType::TowerDefenceActivity => {
            let mut total_score = 0i32;
            let insts = rows_for_activity(assets, "TowerDefenceInstance", id)
                .iter()
                .filter_map(|o| {
                    let iid = j_i32(o, "Id");
                    if iid <= 0 {
                        return None;
                    }
                    let sc = info.get_key_int(&format!("td:score:{iid}")).max(0) as i32;
                    total_score = total_score.saturating_add(sc);
                    let rewarded = claim_matches_id(claims, "td:inst:", iid);
                    Some(TowerDefenceInstanceInfo {
                        Id: iid,
                        Score: sc,
                        Rewarded: rewarded,
                        IsPassed: sc > 0 || rewarded,
                        UnlockTime: begin_s,
                        MaxScore: sc,
                        PassTime: info.get_key_int(&format!("td:time:{iid}")).max(0) as i32,
                    })
                })
                .collect();
            let rewarded_scores = rows_for_activity(assets, "TowerDefenceReward", id)
                .iter()
                .filter_map(|o| {
                    let rid = j_i32(o, "Id");
                    (rid > 0 && claim_matches_id(claims, "td:score:", rid)).then_some(rid)
                })
                .collect();
            ActivityDataDataOneof::TowerDefenseActivityInfo(TowerDefenseActivityInfo {
                InstanceInfos: insts,
                RewardedScoreIds: rewarded_scores,
                TotalScore: total_score,
                ShowName: true,
            })
        }
        ActivityType::TimePointRewardActivity => {
            let rewards = rows_for_activity(assets, "TimePointRewardActivity", id)
                .iter()
                .filter_map(|o| {
                    let rid = j_i32(o, "Id");
                    if rid <= 0 {
                        return None;
                    }
                    let rewarded = claim_matches_id(claims, "time_pt:", rid);
                    Some(TimePointRewardData {
                        Id: rid,
                        RewardTime: begin_s,
                        Rewarded: rewarded,
                        CanGetReward: !rewarded,
                        Task: Some(make_cond_task(rid, rewarded, true, 1, 1)),
                    })
                })
                .collect();
            ActivityDataDataOneof::ActivityTimePointRewarData(ActivityTimePointRewarData {
                Rewards: rewards,
            })
        }
        ActivityType::RiskHarvest => {
            let insts = rows_for_activity(assets, "RiskHarvestInst", id)
                .iter()
                .filter_map(|o| {
                    let iid = j_i32(o, "Id");
                    if iid <= 0 {
                        return None;
                    }
                    let sc = info.get_key_int(&format!("risk:score:{iid}")).max(0) as i32;
                    let rewarded = claim_matches_id(claims, "risk:inst:", iid);
                    Some(RiskHarvestInstInfo {
                        Id: iid,
                        UnlockTime: begin_s,
                        IsUnlock: true,
                        Score: sc,
                        Rewarded: rewarded,
                        IsFinished: sc > 0 || rewarded,
                        StarRewardInfos: Vec::new(),
                    })
                })
                .collect();
            let rewarded_scores = rows_for_activity(assets, "RiskHarvestScoreReward", id)
                .iter()
                .filter_map(|o| {
                    let rid = j_i32(o, "Id");
                    (rid > 0 && claim_matches_id(claims, "risk:score:", rid)).then_some(rid)
                })
                .collect();
            let unlock_buffs = rows_for_activity(assets, "RiskHarvestBuffGroup", id)
                .iter()
                .filter_map(|o| {
                    let bid = j_i32(o, "Id");
                    (bid > 0).then_some(bid)
                })
                .collect();
            ActivityDataDataOneof::RiskHarvestActivityData(RiskHarvestActivityData {
                InstInfos: insts,
                RewardedScores: rewarded_scores,
                UnlockBuffGroups: unlock_buffs,
                ..Default::default()
            })
        }
        ActivityType::CorniceMeeting => {
            let mut entries = BTreeMap::new();
            for o in rows_for_activity(assets, "CorniceChallenge", id) {
                let cid = j_i32(&o, "Id");
                if cid <= 0 {
                    continue;
                }
                let sc = info.get_key_int(&format!("cornice:score:{cid}")).max(0) as i32;
                let cid_pfx = format!("cornice:{cid}:");
                let rw_pfx = format!("cornice_rw:{cid}:");
                let rw = claims
                    .iter()
                    .filter_map(|c| {
                        c.strip_prefix(&cid_pfx)
                            .or_else(|| c.strip_prefix(&rw_pfx))
                            .or_else(|| c.strip_prefix("cornice:"))
                    })
                    .filter_map(|s| s.parse::<i32>().ok())
                    .collect();
                entries.insert(
                    cid,
                    ActivityCorniceMeetingLevelEntryData {
                        MaxScore: sc,
                        RemainTime: 0,
                        UnlockTime: begin_s,
                        RewardedMap: rw,
                    },
                );
            }
            ActivityDataDataOneof::ActivityCorniceMeetingData(ActivityCorniceMeetingData {
                UnlockTime: begin_s,
                LevelEntryData: entries,
            })
        }
        ActivityType::BlackCoastTheme => {
            let all_tasks = assets.raw_json_cached("BlackCoastThemeTaskRe");
            let stages = rows_for_activity(assets, "BlackCoastThemeStageRe", id)
                .iter()
                .filter_map(|st| {
                    let sid = j_i32(st, "Id");
                    if sid <= 0 {
                        return None;
                    }
                    let task_rows: Vec<_> = all_tasks
                        .iter()
                        .filter_map(|v| v.as_object().cloned())
                        .filter(|t| j_i32(t, "StageId") == sid)
                        .collect();
                    Some(BlackCoastThemeStageInfo {
                        Id: sid,
                        Tasks: act_task_list_from_rows(
                            &task_rows,
                            progress_comp,
                            cond_ctx,
                            "blackcoast:task:",
                        ),
                    })
                })
                .collect();
            let rw_ids = rows_for_activity(assets, "BlackCoastThemeRewardRe", id)
                .iter()
                .filter_map(|o| {
                    let rid = j_i32(o, "Id");
                    (rid > 0 && claim_matches_id(claims, "blackcoast:rw:", rid)).then_some(rid)
                })
                .collect();
            ActivityDataDataOneof::ActivityBlackCoastData(ActivityBlackCoastData {
                StageData: stages,
                RewardIds: rw_ids,
            })
        }
        ActivityType::RogueWhiteCat => {
            let role_insts = assets
                .raw_json_cached("DreamLinkRoleDungeon")
                .iter()
                .filter_map(|v| v.as_object())
                .filter_map(|o| {
                    let iid = j_i32(o, "Id");
                    (iid > 0).then_some(RoleInstanceList {
                        InstId: iid,
                        IsUnlock: true,
                        CanUnlock: true,
                    })
                })
                .collect();
            let world_runs = rows_for_activity(assets, "DreamLinkWorldRun", id)
                .iter()
                .filter_map(|o| {
                    let lid = j_i32(o, "Id");
                    if lid <= 0 {
                        return None;
                    }
                    let claimed = claim_matches_id(claims, "rogue_cat:lp:", lid);
                    Some(LevelPlayList {
                        Index: lid,
                        LevelPlayId: lid,
                        State: Some(if claimed {
                            SignState::IsReceive
                        } else {
                            SignState::Unlock
                        }),
                        IsUnlock: true,
                        UnlockTime: begin_s,
                        PlayTime: info.get_key_int(&format!("rogue_cat:rec:{id}")).max(0) as i32,
                    })
                })
                .collect();
            let lim_rewards = rows_for_activity(assets, "RogueLimitTimeReward", id)
                .iter()
                .filter_map(|o| {
                    let cid = j_i32(o, "Id");
                    if cid <= 0 {
                        return None;
                    }
                    let claimed = claim_matches_id(claims, "rogue_cat:lim:", cid);
                    Some(AllLimitTimeReward {
                        ConfigId: cid,
                        CurProgress: i32::from(claimed),
                        Target: 1,
                        SignState: Some(if claimed {
                            SignState::IsReceive
                        } else {
                            SignState::Unlock
                        }),
                    })
                })
                .collect();
            ActivityDataDataOneof::DreamLinkActivityData(DreamLinkActivityData {
                MaxEnergy: 2000,
                RoleInstanceList: role_insts,
                LevelPlayList: world_runs,
                AllLimitTimeReward: lim_rewards,
                LimitTimeReward: begin_s,
                LimitTimeEnd: end_s,
                ..Default::default()
            })
        }
        ActivityType::ScratchCard => {
            let rounds = rows_for_activity(assets, "ScratchCardRoundRe", id)
                .iter()
                .filter_map(|o| {
                    let rid = j_i32(o, "RoundId");
                    (rid > 0).then_some(ScratchTicketRoundData {
                        RoundId: rid,
                        UnlockTime: begin_s,
                        ..Default::default()
                    })
                })
                .collect();
            ActivityDataDataOneof::ActivityScratchTicketData(ActivityScratchTicketData {
                RoundData: rounds,
                ConditionData: Vec::new(),
            })
        }
        ActivityType::PreheatSign => {
            let nodes = rows_for_activity(assets, "PreheatSignRe", id)
                .iter()
                .filter_map(|o| {
                    let nid = j_i32(o, "Id");
                    (nid > 0).then_some(PreheatSignNodeInfo {
                        PreheatNodeId: nid,
                        UnlockTime: begin_s,
                        Rewarded: claim_matches_id(claims, "preheat:", nid)
                            || claims.iter().any(|c| c == &format!("preheat:{nid}:{nid}")),
                    })
                })
                .collect();
            ActivityDataDataOneof::PreheatSignActivityData(PreheatSignActivityData {
                PreheatSignNodeInfos: nodes,
            })
        }
        ActivityType::MowTower => {
            let all_rewards = assets.raw_json_cached("MowTowerRewardRe");
            let levels = rows_for_activity(assets, "MowTowerLevelsRe", id)
                .iter()
                .filter_map(|o| {
                    let lid = j_i32(o, "Id");
                    if lid <= 0 {
                        return None;
                    }
                    let s1 = info.get_key_int(&format!("mow:s1:{lid}")).max(0) as i32;
                    let s2 = info.get_key_int(&format!("mow:s2:{lid}")).max(0) as i32;
                    let total = s1 + s2;
                    let mut rstatus = BTreeMap::new();
                    for rv in all_rewards.iter().filter_map(|v| v.as_object()) {
                        if j_i32(rv, "MowTowerLevelsId") != lid {
                            continue;
                        }
                        let rid = j_i32(rv, "Id");
                        let need = j_i32(rv, "Score");
                        let st = if claim_matches_id(claims, "mow:clm:", rid) {
                            MowTowerRewardStatus::Rewarded
                        } else if total >= need && need > 0 {
                            MowTowerRewardStatus::CanReward
                        } else {
                            MowTowerRewardStatus::CanNoReward
                        };
                        rstatus.insert(rid, st);
                    }
                    Some(MowTowerLevelsInfo {
                        BabelTowerLevelId: lid,
                        UnlockTime: begin_s,
                        IsUnlock: true,
                        FirstScore: s1,
                        SecondScore: s2,
                        LevelRewardStatus: rstatus,
                        HardLevelBuffs: j_i32_vec(o, "OptionalBuff"),
                        ..Default::default()
                    })
                })
                .collect();
            ActivityDataDataOneof::MowTowerActivityData(MowTowerActivityData {
                MowTowerLevelsInfo: levels,
            })
        }
        ActivityType::ThroughTrain => {
            ActivityDataDataOneof::ThroughTrainActivityData(ThroughTrainActivityData {
                IsFinish: claim_matches_id(claims, "through_train:", id),
            })
        }
        ActivityType::SprintSign => {
            let tasks = act_task_list_from_rows(
                &rows_for_activity(assets, "SpringReward", id),
                progress_comp,
                cond_ctx,
                "spring_sign:",
            );
            ActivityDataDataOneof::SpringSignData(SpringSignData {
                SpringSignActivityTasks: tasks,
                CanInvite: true,
                DrawRoles: Vec::new(),
                SkinReward: claim_matches_id(claims, "spring_sign:skin:", id),
            })
        }
        ActivityType::MapTravelActivity => {
            let tasks = act_task_list_from_rows(
                &rows_for_activity(assets, "TravelTask", id),
                progress_comp,
                cond_ctx,
                "map_travel:task:",
            );
            let soar = rows_for_activity(assets, "MapTravelConfig", id)
                .iter()
                .filter_map(|o| {
                    let lid = j_i32(o, "Id");
                    (lid > 0).then_some(SoarLevelPlayInfo {
                        SoarLevelPlatId: lid,
                        HistorySoarScore: info.get_key_int(&format!("soar:score:{lid}")).max(0) as i32,
                        ReceiveIds: Vec::new(),
                    })
                })
                .collect();
            ActivityDataDataOneof::MapTravelActivityData(MapTravelActivityData {
                ActivityTasks: tasks,
                MapTravelLevel: info.get_key_int("map_travel:lv").max(1) as i32,
                SoarLevels: soar,
                ..Default::default()
            })
        }
        ActivityType::FarmGold => {
            let got = rows_for_activity(assets, "FarmGoldScore", id)
                .iter()
                .filter_map(|o| {
                    let rid = j_i32(o, "Id");
                    (rid > 0 && claim_matches_id(claims, "farmgold:pt:", rid)).then_some(rid)
                })
                .collect();
            let levels = rows_for_activity(assets, "FarmGoldActivity", id)
                .iter()
                .filter_map(|o| {
                    let iid = j_i32(o, "InstId");
                    if iid <= 0 {
                        return None;
                    }
                    let pts = info.get_key_int(&format!("farmgold:score:{iid}")).max(0) as i32;
                    Some(FarmGoldLevelPlayInfo {
                        InstId: iid,
                        StartTime: begin_s as i32,
                        Challenges: pts > 0,
                        Points: pts,
                        LevelRewardGet: claim_matches_id(claims, "farmgold:lv:", iid),
                        Difficulty: j_i32_vec(o, "DifficultyOptions").first().copied().unwrap_or(1),
                    })
                })
                .collect();
            ActivityDataDataOneof::FarmGoldData(FarmGoldData {
                PointRewardGet: got,
                LevelPlayTasks: levels,
            })
        }
        ActivityType::RougeActivity => {
            let season_id = rogue_season_id_for_activity(assets, id);
            let tokens = assets
                .raw_json_cached("RogueToken")
                .iter()
                .filter_map(|v| v.as_object())
                .filter(|o| j_i32(o, "SeasonId") == season_id)
                .filter_map(|o| {
                    let tid = j_i32(o, "Id");
                    (tid > 0).then_some(RoguelikeTokenList {
                        Id: tid,
                        IsReceive: claims
                            .iter()
                            .any(|c| c == &format!("rogue:token:{season_id}:{tid}")),
                    })
                })
                .collect();
            let rewards = assets
                .rogue_season_rewards()
                .iter()
                .filter(|r| r.SeasonId == season_id || r.SeasonId == 0)
                .map(|r| RogueSeasonReward {
                    Id: r.Id,
                    IsReceive: claims
                        .iter()
                        .any(|c| c == &format!("rogue:season_rw:{season_id}:{}", r.Index)),
                })
                .collect();
            ActivityDataDataOneof::ActivityRogueData(ActivityRogueData {
                BeginOpenTime: begin_s,
                EndOpenTime: end_s,
                RoguelikeSeason: Some(RoguelikeSeason {
                    SeasonId: season_id,
                    StartTime: begin_s,
                    EndTime: end_s,
                    RoguelikeTokenList: tokens,
                    SeasonRewardList: rewards,
                    TokenItemCount: info.get_key_int("rogue:tokens").max(0) as i32,
                    BlackFlowerUseCount: 0,
                    BlackFlowerMaxCount: 3,
                    TowerTrialBestClearCount: info.get_key_int("rogue_trial:clear").max(0) as i32,
                }),
            })
        }
        ActivityType::RogueWeekly => {
            let cycle_id = info.get_key_int("rogue_weekly:cycle").max(
                assets
                    .raw_json_cached("RogueWeeklyCycle")
                    .iter()
                    .filter_map(|v| v.as_object())
                    .find(|o| j_i32(o, "ActivityId") == id)
                    .map(|o| i64::from(j_i32(o, "Id")))
                    .unwrap_or(1),
            ) as i32;
            let score = info.get_key_int("rogue_weekly:score").max(0) as i32;
            let max_score = assets
                .raw_json_cached("RogueWeeklyCycle")
                .iter()
                .filter_map(|v| v.as_object())
                .find(|o| j_i32(o, "Id") == cycle_id)
                .map(|o| j_i32(o, "MaxScore").max(4000))
                .unwrap_or(6000);
            let awards = assets
                .raw_json_cached("RogueWeeklyReward")
                .iter()
                .filter_map(|v| v.as_object())
                .filter(|o| j_i32(o, "CycleId") == cycle_id)
                .filter_map(|o| {
                    let cid = j_i32(o, "Id");
                    if cid <= 0 {
                        return None;
                    }
                    let need = j_i32(o, "Score").max(1);
                    let claimed = claims
                        .iter()
                        .any(|c| c == &format!("rogue_weekly:rw:{id}:{cid}"));
                    Some(RogueWeeklyAward {
                        ConfigId: cid,
                        CurProgress: score.min(need),
                        MaxProgress: need,
                        SignState: Some(if claimed {
                            SignState::IsReceive
                        } else if score >= need {
                            SignState::Unlock
                        } else {
                            SignState::Lock
                        }),
                    })
                })
                .collect();
            ActivityDataDataOneof::ActivityWeeklyRogueData(ActivityWeeklyRogueData {
                CycleId: cycle_id,
                Score: score,
                RogueWeeklyAward: awards,
                MaxScore: max_score,
                CurWorldLevel: 1,
                MaxFreeCount: 3,
                UseFreeCount: info.get_key_int("rogue_weekly:bf_used") as i32,
                ..Default::default()
            })
        }
        ActivityType::FishingActivity => {
            let tasks = act_task_list_from_rows(
                &rows_for_activity(assets, "FishingActivityLimitTask", id),
                progress_comp,
                cond_ctx,
                "fish:lim:",
            );
            let mut milestones = BTreeMap::new();
            for o in rows_for_activity(assets, "FishingActivityMilestone", id) {
                let mid = j_i32(&o, "Id");
                if mid <= 0 {
                    continue;
                }
                let st = if claim_matches_id(claims, "fish:ms:", mid) {
                    SignState::IsReceive
                } else {
                    SignState::Unlock
                };
                milestones.insert(mid, st);
            }
            ActivityDataDataOneof::ActivityFishingData(ActivityFishingData {
                ActivityTaskData: tasks,
                MilestoneReward: milestones,
                LimitTimeReward: begin_s,
                LimitTimeEnd: end_s,
                MilestoneRewardItemAccumulate: info.fishing.total_sold,
            })
        }
        ActivityType::BabelTower => {
            let levels = rows_for_activity(assets, "BabelTowerLevel", id)
                .iter()
                .filter_map(|o| {
                    let lid = j_i32(o, "Id");
                    (lid > 0).then_some(BabelTowerData {
                        BabelTowerLevelId: lid,
                        UnlockTime: begin_s,
                        NormalLevelBuffs: j_i32_vec(o, "OptionalBabelBuff"),
                        HasPassed: info.get_key_int("babel:finish") > 0,
                        ..Default::default()
                    })
                })
                .collect();
            let buffs = assets
                .raw_json_cached("BabelTowerBuff")
                .iter()
                .filter_map(|v| v.as_object())
                .filter_map(|o| {
                    let bid = j_i32(o, "Id");
                    (bid > 0).then_some(BabelBuff {
                        BuffId: bid,
                        Unlocked: true,
                    })
                })
                .collect();
            let debuffs = assets
                .raw_json_cached("BabelTowerDeTerm")
                .iter()
                .filter_map(|v| v.as_object())
                .filter_map(|o| {
                    let did = j_i32(o, "Id");
                    (did > 0).then_some(BabelDebuff {
                        BuffId: did,
                        Unlocked: true,
                    })
                })
                .collect();
            let normal_q = act_task_list_from_rows(
                &rows_for_activity(assets, "BabelTowerTask", id),
                progress_comp,
                cond_ctx,
                "babel:task:",
            );
            let daily_q = act_task_list_from_rows(
                &rows_for_activity(assets, "BabelTowerDailyTask", id),
                progress_comp,
                cond_ctx,
                "babel:daily:",
            );
            ActivityDataDataOneof::BabelTowerActivity(BabelTowerActivity {
                BabelTowerDataList: levels,
                BabelDebuffUnlocks: debuffs,
                BabelBuffUnlocks: buffs,
                NormalQuest: normal_q,
                DailyQuest: daily_q,
                CurrentItemCount: info.get_key_int("babel:star").max(0) as i32,
                ShowName: info.get_key_int("babel:showname") != 0,
            })
        }
        ActivityType::Explore => {
            let mut tasks = BTreeMap::new();
            for o in rows_for_activity(assets, "ExploreActivityTask", id) {
                let tid = j_i32(&o, "TaskId");
                if tid <= 0 {
                    continue;
                }
                let st = if claim_matches_id(claims, "explore_act:", tid) {
                    ActivityTaskState::ActivityTaskTaken
                } else {
                    ActivityTaskState::ActivityTaskFinish
                };
                tasks.insert(tid, st);
            }
            ActivityDataDataOneof::ActivityMapExploreData(ActivityMapExploreData {
                ActivityTasks: tasks,
            })
        }
        ActivityType::Abyss => {
            let roles = rows_for_activity(assets, "AbyssLittleRole", id)
                .iter()
                .filter_map(|o| {
                    let rid = j_i32(o, "Id");
                    (rid > 0).then_some(AbyssDangoRoleData {
                        Id: rid,
                        Level: info.get_key_int(&format!("abyss:lrole:{rid}")).max(1) as i32,
                        EquipItems: vec![0; 9],
                    })
                })
                .collect();
            let rw = rows_for_activity(assets, "AbyssReward", id)
                .iter()
                .filter_map(|o| {
                    let rid = j_i32(o, "Id");
                    if rid <= 0 {
                        return None;
                    }
                    let cg = j_i32(o, "ConditionId");
                    let done = cond_done(cond_ctx, cg);
                    let claimed = claim_matches_id(claims, "abyss:clm:", rid);
                    Some(AbyssRewardInfo {
                        Id: rid,
                        CanGetReward: !claimed && done,
                        CurrentProgress: i32::from(done || claimed),
                        TargetProgress: 1,
                        CanUnlock: true,
                    })
                })
                .collect();
            let mut unlock_ch = Vec::new();
            let mut ch_data = Vec::new();
            for o in rows_for_activity(assets, "AbyssInst", id) {
                let cid = j_i32(&o, "Id");
                if cid <= 0 {
                    continue;
                }
                let passed = info.get_key_int("abyss:room") > 0;
                unlock_ch.push(cid);
                ch_data.push(AbyssChallengeData {
                    ChallengeId: cid,
                    CanUnlock: true,
                    CanChallenge: true,
                    UnlockTime: begin_s,
                    ConditionFinishState: passed,
                    MaxProgress: j_i32(&o, "Difficulty"),
                    MinPassTime: 0,
                    IsPassed: passed,
                });
            }
            ActivityDataDataOneof::DangoAbyssActivityData(DangoAbyssActivityData {
                RoleList: roles,
                AbyssRewardInfo: rw,
                UnlockChallengeIdList: unlock_ch,
                AbyssChallengeData: ch_data,
                StartTime: begin_s * 1000,
                EndTime: end_s * 1000,
                ..Default::default()
            })
        }
        ActivityType::RogueRes => {
            let ends: Vec<ActivityTask> = assets
                .raw_json_cached("RogueResEndAward")
                .iter()
                .filter_map(|v| v.as_object())
                .filter_map(|o| {
                    let eid = j_i32(o, "Id");
                    (eid > 0).then(|| {
                        let claimed = claims.iter().any(|c| c.starts_with(&format!("rogue_res:end:")) && c.ends_with(&format!(":{eid}")));
                        make_act_task(eid, claimed, true, 1, 1)
                    })
                })
                .collect();
            let seasons = rows_for_activity(assets, "RogueResTheme", id)
                .iter()
                .filter_map(|o| {
                    let sid = j_i32(o, "Id");
                    (sid > 0).then_some(PermanentSeasonData {
                        PermanentSeasonDataId: sid,
                        RogueResEndAward: ends.clone(),
                        EndTime: end_s,
                        ..Default::default()
                    })
                })
                .collect();
            ActivityDataDataOneof::ActivityPermanentRogueData(ActivityPermanentRogueData {
                PermanentSeasonData: seasons,
                RogueResTaskData: None,
            })
        }
        ActivityType::DangoMonopoly => {
            let tasks = assets
                .raw_json_cached("DangoMonopolyTask")
                .iter()
                .filter_map(|v| v.as_object())
                .filter_map(|o| {
                    let tid = j_i32(o, "TaskId");
                    if tid <= 0 {
                        return None;
                    }
                    let taken = claim_matches_id(claims, "dango_mono:task:", tid);
                    let done = taken || cond_done(cond_ctx, j_i32(o, "ConditionGroupId"));
                    Some(DangoMonopolyConfig {
                        TaskId: tid,
                        ActivityTaskState: Some(if taken {
                            DangoMonopolyTaskState::HasGet
                        } else if done {
                            DangoMonopolyTaskState::Completed
                        } else {
                            DangoMonopolyTaskState::NotCompleted
                        }),
                        Progress: i32::from(done),
                        TargetProgress: 1,
                    })
                })
                .collect();
            let mut boards = BTreeMap::new();
            for o in assets
                .raw_json_cached("DangoMonopolyBoard")
                .iter()
                .filter_map(|v| v.as_object())
            {
                let bid = j_i32(o, "BoardId");
                if bid > 0 {
                    boards.insert(bid, DangoMonopolyBoardData::default());
                }
            }
            ActivityDataDataOneof::ActivityDangoMonopolyData(ActivityDangoMonopolyData {
                CurrentBoardId: info
                    .get_key_int("dm:board")
                    .max(info.get_key_int(&format!("dmono_board:{id}")))
                    .max(1) as i32,
                CurrentGridId: info
                    .get_key_int("dm:grid")
                    .max(info.get_key_int(&format!("dmono_grid:{id}")))
                    .max(1) as i32,
                DangoTaskConfig: tasks,
                UnlockTime: begin_s,
                BoardMap: boards,
                ..Default::default()
            })
        }
        ActivityType::CiacconaActivity => {
            let chaps = assets
                .raw_json_cached("CiacconaGalChapter")
                .iter()
                .filter_map(|v| v.as_object())
                .filter_map(|o| {
                    let cid = j_i32(o, "Id");
                    (cid > 0).then_some(CiacconaGalChapterData {
                        ChapterDataId: cid,
                        CanUnlock: true,
                        ..Default::default()
                    })
                })
                .collect();
            let rws = rows_for_activity(assets, "CiacconaActivityReward", id)
                .iter()
                .filter_map(|o| {
                    let rid = j_i32(o, "Id");
                    if rid <= 0 {
                        return None;
                    }
                    let done = cond_done(cond_ctx, j_i32(o, "CondGroupId"));
                    let got = claim_matches_id(claims, "ciaccona:rw:", rid);
                    Some(CiacconaGalRewardData {
                        RewardDataId: rid,
                        CanReceive: done && !got,
                        IsRewarded: got,
                    })
                })
                .collect();
            ActivityDataDataOneof::ActivityCiacconaGalData(ActivityCiacconaGalData {
                ChapterData: chaps,
                ProgressRewardData: rws,
                State2Unlock: true,
                State3Unlock: true,
                RewardStartTime: begin_s,
                RewardEndTime: end_s,
                ..Default::default()
            })
        }
        ActivityType::Avignon => {
            let stage_ids = assets
                .raw_json_cached("AvignonStage")
                .iter()
                .filter_map(|v| v.as_object().map(|o| j_i32(o, "Id")))
                .filter(|&sid| sid > 0)
                .collect();
            let tasks = act_task_list_from_rows(
                &assets
                    .raw_json_cached("AvignonTask")
                    .iter()
                    .filter_map(|v| v.as_object().cloned())
                    .collect::<Vec<_>>(),
                progress_comp,
                cond_ctx,
                "avignon:task:",
            );
            ActivityDataDataOneof::ActivityAvignon(ActivityAvignon {
                RewardData: Some(ActivityTaskData {
                    ActivityTasks: tasks,
                }),
                StageId: stage_ids,
            })
        }
        ActivityType::PhantomBattle | ActivityType::PhantomBattleRecord => {
            let tasks = act_task_list_from_rows(
                &rows_for_activity(assets, "PhantomBattleTask", id),
                progress_comp,
                cond_ctx,
                "pb_task:",
            );
            let badges = rows_for_activity(assets, "PhantomBattleBadge", id)
                .iter()
                .filter_map(|o| {
                    let bid = j_i32(o, "Id");
                    (bid > 0).then_some(PhantomArenaBadge {
                        BadgeId: bid,
                        IsUnlock: true,
                    })
                })
                .collect();
            let badge_rws = rows_for_activity(assets, "PhantomBattleBadgeReward", id)
                .iter()
                .filter_map(|o| {
                    let rid = j_i32(o, "Id");
                    (rid > 0).then_some(PhantomArenaBadgeReward {
                        BadgeRewardId: rid,
                        NeedCount: 1,
                        IsTaken: claim_matches_id(claims, "pb_badge:", rid),
                    })
                })
                .collect();
            let cards = rows_for_activity(assets, "PhantomBattleCard", id)
                .iter()
                .filter_map(|o| {
                    let cid = j_i32(o, "Id");
                    (cid > 0).then_some(PhantomArenaCardInfo {
                        CardId: cid,
                        IsUnlock: true,
                        IsCardOutlookUnlock: true,
                    })
                })
                .collect();
            let card_rws = rows_for_activity(assets, "PhantomBattleCardReward", id)
                .iter()
                .filter_map(|o| {
                    let rid = j_i32(o, "Id");
                    (rid > 0).then_some(PhantomArenaCardReward {
                        CardId: rid,
                        NeedCount: 1,
                        IsTaken: claim_matches_id(claims, "pb_card_rw:", rid),
                    })
                })
                .collect();
            let roles = rows_for_activity(assets, "PhantomBattleCardRole", id)
                .iter()
                .filter_map(|o| {
                    let rid = j_i32(o, "Id");
                    (rid > 0).then_some(PhantomArenaRoleInfo {
                        RoleInfoId: rid,
                        IsUnlock: true,
                        IsTaken: claim_matches_id(claims, "pb_role_rw:", rid),
                    })
                })
                .collect();
            let data = PhantomArenaActivityData {
                ActivityTasks: tasks,
                BadgeInfo: badges,
                BadgeReward: badge_rws,
                CardList: cards,
                CardReward: card_rws,
                RoleInfo: roles,
                TimeLimitShopEndTime: end_s,
                ..Default::default()
            };
            if act_type == ActivityType::PhantomBattleRecord {
                ActivityDataDataOneof::PhantomBattleRecordActivityInfo(data)
            } else {
                ActivityDataDataOneof::PhantomArenaActivityData(data)
            }
        }
        ActivityType::MoraleActivity => {
            let areas = assets
                .raw_json_cached("MoraleArea")
                .iter()
                .filter_map(|v| v.as_object())
                .filter_map(|o| {
                    let aid = j_i32(o, "Id");
                    (aid > 0).then_some(MoraleAreaData {
                        AreaDataId: aid,
                        ExploreBoxReceivedCount: 0,
                    })
                })
                .collect();
            ActivityDataDataOneof::ActivityMoraleData(ActivityMoraleData {
                AreaData: areas,
                ..Default::default()
            })
        }
        ActivityType::FloroRanchActivity => {
            let tasks = cond_task_list_from_rows(
                &rows_for_activity(assets, "FloroRanchReward", id),
                progress_comp,
                cond_ctx,
                "floro:task:",
            );
            ActivityDataDataOneof::FloroRanchActivityData(FloroRanchActivityData {
                FloroRangeData: Some(FloroRangeData {
                    ConditionTask: tasks,
                    FloroRangeUnlockTime: begin_s,
                    FloroRangeEndTime: end_s,
                    ..Default::default()
                }),
                ..Default::default()
            })
        }
        ActivityType::TrapDefense => {
            let rws = rows_for_activity(assets, "TrapDefenseReward", id)
                .iter()
                .filter_map(|o| {
                    let rid = j_i32(o, "Id");
                    if rid <= 0 {
                        return None;
                    }
                    let taken = claim_matches_id(claims, "trapdef:rw:", rid);
                    Some(TrapDefenseRewardData {
                        ActivityServerRewardItemData: Some(make_cond_task(
                            rid, taken, taken, i32::from(taken), 1,
                        )),
                        StartTime: begin_s,
                        EndTime: end_s,
                    })
                })
                .collect();
            let aux = rows_for_activity(assets, "TrapDefenseAuxiliaryType", id)
                .iter()
                .filter_map(|o| {
                    let aid = j_i32(o, "Id");
                    (aid > 0).then_some(TrapDefenseAuxiliaryData {
                        Id: aid,
                        Level: 1,
                        Branch: 0,
                        MaxLevel: j_i32(o, "MaxLevel").max(1),
                    })
                })
                .collect();
            let blds = rows_for_activity(assets, "TrapDefenseBuildingType", id)
                .iter()
                .filter_map(|o| {
                    let bid = j_i32(o, "Id");
                    (bid > 0).then_some(TrapDefenseBuildingData {
                        Id: bid,
                        Level: 1,
                        Branch: 0,
                        MaxLevel: j_i32(o, "MaxLevel").max(1),
                        ..Default::default()
                    })
                })
                .collect();
            let chs = rows_for_activity(assets, "TrapDefenseActivity", id)
                .iter()
                .filter_map(|o| {
                    let cid = j_i32(o, "Id");
                    (cid > 0).then_some(TrapDefenseLevelData {
                        ChallengeId: cid,
                        CanUnlock: true,
                        UnlockTime: begin_s,
                        ..Default::default()
                    })
                })
                .collect();
            ActivityDataDataOneof::ActivityTrapDefenseData(ActivityTrapDefenseData {
                Rewards: rws,
                Auxiliaries: aux,
                Buildings: blds,
                Challenges: chs,
                StartTime: begin_s,
                EndTime: end_s,
                ..Default::default()
            })
        }
        ActivityType::FunPlay => {
            let chs = rows_for_activity(assets, "FunPlayActivityChallenge", id)
                .iter()
                .filter_map(|o| {
                    let cid = j_i32(o, "Id");
                    (cid > 0).then_some(ActivityFunPlayChallengeData {
                        ChallengeId: cid,
                        UnlockTime: begin_s,
                        ..Default::default()
                    })
                })
                .collect();
            ActivityDataDataOneof::ActivityFunPlayData(ActivityFunPlayData {
                ActivityFunPlayChallengeData: chs,
            })
        }
        ActivityType::MoonPhase => {
            let sel = rows_for_activity(assets, "MoonPhaseReward", id)
                .iter()
                .filter_map(|o| {
                    let mid = j_i32(o, "Id");
                    (mid > 0).then_some(MoonSignInConfigData {
                        MoonId: mid,
                        ..Default::default()
                    })
                })
                .collect();
            ActivityDataDataOneof::ActivityMoonSignInData(ActivityMoonSignInData {
                MoonPhaseSelectList: sel,
                IsGrandReward: claim_matches_id(claims, "moon_phase:", id),
                CurrentMoonId: info.get_key_int("moon_phase:cur").max(1) as i32,
            })
        }
        ActivityType::LineCross => {
            let chs = rows_for_activity(assets, "LineCrossChallenge", id)
                .iter()
                .filter_map(|o| {
                    let cid = j_i32(o, "Id");
                    (cid > 0).then_some(LineCrossChallengeData {
                        ChallengeId: cid,
                        CanGetReward: !claim_matches_id(claims, "linecross:", cid),
                        OpenTime: begin_s,
                        RewardDataId: j_i32(o, "RewardId"),
                        EntityConfigId: j_i32(o, "EntityConfigId"),
                        IsPreChallengeState: false,
                    })
                })
                .collect();
            ActivityDataDataOneof::ActivityLineCrossData(ActivityLineCrossData {
                Challenges: chs,
            })
        }
        ActivityType::HonamiStory => {
            let areas = rows_for_activity(assets, "HonamiStoryArea", id)
                .iter()
                .filter_map(|o| {
                    let aid = j_i32(o, "Id");
                    (aid > 0).then_some(HonamiStoryAreaConfig {
                        AreaId: aid,
                        Status: 1,
                        SecreteStatus: 0,
                    })
                })
                .collect();
            let items = rows_for_activity(assets, "HonamiStoryItemCollection", id)
                .iter()
                .filter_map(|o| {
                    let iid = j_i32(o, "ItemId");
                    (iid > 0).then_some(HonamiStoryItemCollectionConfig {
                        Id: iid,
                        Status: i32::from(claim_matches_id(claims, "honami:col:", iid)),
                    })
                })
                .collect();
            let perm_tasks = cond_task_list_from_rows(
                &rows_for_activity(assets, "HonamiStoryResidentTask", id),
                progress_comp,
                cond_ctx,
                "honami:perm:",
            );
            let lim_tasks = cond_task_list_from_rows(
                &rows_for_activity(assets, "HonamiStoryLimitTask", id),
                progress_comp,
                cond_ctx,
                "honami:lim:",
            );
            let scores = rows_for_activity(assets, "HonamiStoryScoreReward", id)
                .iter()
                .filter_map(|o| {
                    let rid = j_i32(o, "Id");
                    (rid > 0).then_some(HonamiStoryScoreRewardInfo {
                        ScoreRewardId: rid,
                        Status: if claim_matches_id(claims, "honami:score:", rid) { 2 } else { 0 },
                    })
                })
                .collect();
            ActivityDataDataOneof::HonamiStoryActivityData(HonamiStoryActivityData {
                AreaConfigList: areas,
                ItemCollectionList: items,
                PermanentTaskData: perm_tasks,
                LimitTaskData: lim_tasks,
                ScoreRewardInfo: scores,
                LifeSupportLevel: info
                    .get_key_int("honami:life")
                    .max(info.get_key_int("honami:ls_lv"))
                    .max(1) as i32,
                TotalRevenue: info.get_key_int("honami:revenue").max(0) as i32,
                ..Default::default()
            })
        }
        ActivityType::Survivors => {
            let tasks = act_task_list_from_rows(
                &rows_for_activity(assets, "SurvivorsTask", id),
                progress_comp,
                cond_ctx,
                "surv:task:",
            );
            let chs = rows_for_activity(assets, "SurvivorsLevel", id)
                .iter()
                .filter_map(|o| {
                    let lid = j_i32(o, "Id");
                    (lid > 0).then_some(SurvivorsLevelData {
                        LevelId: lid,
                        OpenTime: begin_s,
                        NormalInfo: Some(SurvivorsLevelInfo {
                            IsUnlocked: true,
                            ..Default::default()
                        }),
                        ..Default::default()
                    })
                })
                .collect();
            ActivityDataDataOneof::SurvivorsActivityData(SurvivorsActivityData {
                NormalTaskData: Some(ActivityTaskData {
                    ActivityTasks: tasks,
                }),
                SurvivorsChallengeInfos: chs,
                ..Default::default()
            })
        }
        ActivityType::PhotoFight => {
            let tasks = rows_for_activity(assets, "PhotoFightReward", id)
                .iter()
                .filter_map(|o| {
                    let tid = j_i32(o, "Id");
                    if tid <= 0 {
                        return None;
                    }
                    let taken = claim_matches_id(claims, "photofight:", tid);
                    Some(TaskData {
                        Id: tid,
                        Status: Some(if taken {
                            ActivityTaskState::ActivityTaskTaken
                        } else {
                            ActivityTaskState::ActivityTaskFinish
                        }),
                        Progress: BTreeMap::new(),
                    })
                })
                .collect();
            ActivityDataDataOneof::FightPhotoActivityData(FightPhotoActivityData {
                ActivityId: id,
                Tasks: tasks,
                ..Default::default()
            })
        }
        ActivityType::WuWuKuji => {
            let groups = rows_for_activity(assets, "WuWuTaskPackage", id)
                .iter()
                .filter_map(|o| {
                    let pid = j_i32(o, "Id");
                    (pid > 0).then_some(AwardGroupData {
                        GroupId: pid,
                        ..Default::default()
                    })
                })
                .collect();
            ActivityDataDataOneof::ActivityPrizeDrawingData(ActivityPrizeDrawingData {
                ActivityId: id,
                AwardGroups: groups,
                ..Default::default()
            })
        }
        ActivityType::WuWuWeekSign => {
            let pkgs = rows_for_activity(assets, "WuWuTaskPackage", id)
                .iter()
                .filter_map(|o| {
                    let pid = j_i32(o, "Id");
                    (pid > 0).then_some(WuWuTaskPack {
                        WuWuPackageId: pid,
                        UnLockTime: begin_s,
                        HadReward: claim_matches_id(claims, "wuwu:pkg:", pid),
                    })
                })
                .collect();
            let tasks = cond_task_list_from_rows(
                &rows_for_activity(assets, "WuWuWeekTask", id),
                progress_comp,
                cond_ctx,
                "wuwu:task:",
            );
            ActivityDataDataOneof::WuWuWeekActivity(WuWuWeekActivity {
                ConditionTasks: tasks,
                TaskPack: pkgs,
            })
        }
        ActivityType::InfrTheme => {
            let tasks = act_task_list_from_rows(
                &rows_for_activity(assets, "InfrActivityTask", id),
                progress_comp,
                cond_ctx,
                "infr:task:",
            );
            ActivityDataDataOneof::InfrThemeActivityPb(InfrThemeActivityPb {
                ActivityTaskData: Some(ActivityTaskData {
                    ActivityTasks: tasks,
                }),
            })
        }
        ActivityType::MotorcycleIpLink => {
            let tasks = cond_task_list_from_rows(
                &assets
                    .raw_json_cached("MotorLinkageQuest")
                    .iter()
                    .filter_map(|v| v.as_object().cloned())
                    .collect::<Vec<_>>(),
                progress_comp,
                cond_ctx,
                "motor:link_task:",
            );
            ActivityDataDataOneof::MotorCycleIpActivityData(MotorCycleIpActivityData {
                TaskDataList: tasks,
            })
        }
        ActivityType::NewTowerClimbing => {
            let records = assets
                .raw_json_cached("NewTowerLevel")
                .iter()
                .filter_map(|v| v.as_object())
                .filter_map(|o| {
                    let lid = j_i32(o, "Id");
                    if lid <= 0 {
                        return None;
                    }
                    let sc = info.get_key_int(&format!("newtower:score:{lid}")).max(0) as i32;
                    Some(NewTowerClimbingLevelRecord {
                        LevelId: lid,
                        Score: sc,
                        HistoryScore: sc,
                        IsUnlock: true,
                        ..Default::default()
                    })
                })
                .collect();
            let cycle_id = info.get_key_int("newtower:cycle").max(1) as i32;
            let season_tasks = act_task_list_from_rows(
                &rows_for_activity(assets, "NewTowerSeasonAward", id),
                progress_comp,
                cond_ctx,
                "newtower:sclm:",
            );
            let act_tasks = act_task_list_from_rows(
                &rows_for_activity(assets, "NewTowerScoreReward", id),
                progress_comp,
                cond_ctx,
                "newtower:clm:",
            );
            let score_tasks = claims
                .iter()
                .filter_map(|c| c.strip_prefix("newtower:ssclm:")?.parse::<i32>().ok())
                .collect();
            ActivityDataDataOneof::NewTowerClimbingActivityData(NewTowerClimbingActivityData {
                CycleId: cycle_id,
                SeasonId: 1,
                Records: records,
                ScoreTasks: score_tasks,
                ActivityTasks: act_tasks,
                SeasonTasks: season_tasks,
                CycleBeginTime: begin_s,
                CycleCloseTime: end_s,
                SeasonBeginTime: begin_s,
                SeasonCloseTime: end_s,
            })
        }
        ActivityType::MotorFight => {
            let levels = rows_for_activity(assets, "MotorFightLevel", id)
                .iter()
                .filter_map(|o| {
                    let lid = j_i32(o, "Id");
                    if lid <= 0 {
                        return None;
                    }
                    let sc = info.get_key_int(&format!("mf:best:{lid}")).max(0) as i32;
                    Some(MotorFightLevelPb {
                        LevelId: lid,
                        OpenTime: begin_s,
                        Cleared: sc > 0,
                        BestScore: sc,
                        LastRoleId: 0,
                    })
                })
                .collect();
            let tasks = cond_task_list_from_rows(
                &rows_for_activity(assets, "MotorFightTask", id),
                progress_comp,
                cond_ctx,
                "motor:fight_task:",
            );
            let talents = rows_for_activity(assets, "MotorFightTalent", id)
                .iter()
                .filter_map(|o| {
                    let tid = j_i32(o, "Id");
                    (tid > 0).then_some(MotorFightTalentPb {
                        Id: tid,
                        Unlock: info.get_key_int(&format!("mf:talent:{tid}")) > 0,
                        InUse: info.get_key_int(&format!("mf:talent_use:{tid}")) > 0,
                    })
                })
                .collect();
            ActivityDataDataOneof::MotorFightActivityPb(MotorFightActivityPb {
                MotorFightLevelPb: levels,
                Task: tasks,
                TalentTree: Some(MotorFightTalentTreePb { Talent: talents }),
                ..Default::default()
            })
        }
        ActivityType::MotorParkourActivity => {
            let levels = rows_for_activity(assets, "MotorParkour", id)
                .iter()
                .filter_map(|o| {
                    let pid = j_i32(o, "Id");
                    if pid <= 0 {
                        return None;
                    }
                    let rw_ids = j_i32_vec(o, "RewardIds");
                    let states = rw_ids
                        .iter()
                        .map(|&ridx| {
                            if claim_matches_id(claims, &format!("motor:pk:{pid}:"), ridx) {
                                MotorParkourRewardState::MotorParkourRewardRewarded
                            } else {
                                MotorParkourRewardState::MotorParkourRewardAvailable
                            }
                        })
                        .collect();
                    Some(MotorParkourLevelInfo {
                        MotorParkourId: pid,
                        RewardStates: states,
                        UnlockTime: begin_s,
                        BestPassTime: info.get_key_int(&format!("motor:pk_time:{pid}")).max(0) as i32,
                    })
                })
                .collect();
            ActivityDataDataOneof::MotorParkourActivityInfo(MotorParkourActivityInfo {
                MotorParkourLevelInfos: levels,
            })
        }
        ActivityType::MotorDevelop => {
            let tasks = cond_task_list_from_rows(
                &rows_for_activity(assets, "MotorDevelopTask", id),
                progress_comp,
                cond_ctx,
                "motor:dev:",
            );
            ActivityDataDataOneof::MotorDevelopActivityData(MotorDevelopActivityData {
                Task: tasks,
            })
        }
        ActivityType::FlagChallenge => {
            let tasks = cond_task_list_from_rows(
                &rows_for_activity(assets, "FlagChallengeTask", id),
                progress_comp,
                cond_ctx,
                "flag_ch:",
            );
            ActivityDataDataOneof::FlagChallengeActivityInfo(FlagChallengeActivityInfo {
                ConditionTasks: tasks,
                ..Default::default()
            })
        }
        ActivityType::Rhythm => {
            let sub_all = assets.raw_json_cached("RhythmSubLevel");
            let mut planets_map: BTreeMap<i32, Vec<RhythmShipLevelPb>> = BTreeMap::new();
            for o in assets
                .raw_json_cached("RhythmShipLevel")
                .iter()
                .filter_map(|v| v.as_object())
            {
                let lid = j_i32(o, "Id");
                let planet = j_i32(o, "Planet").max(1);
                if lid <= 0 {
                    continue;
                }
                let subs = sub_all
                    .iter()
                    .filter_map(|v| v.as_object())
                    .filter(|s| j_i32(s, "LevelId") == lid)
                    .filter_map(|s| {
                        let sid = j_i32(s, "Id");
                        let sc = info
                            .get_key_int(&format!("rhythm:score:{sid}"))
                            .max(info.get_key_int(&format!("rhythm:best:{sid}")))
                            .max(0) as i32;
                        let best_rank = info.get_key_int(&format!("rhythm:rank:{sid}"));
                        (sid > 0).then_some(RhythmSubLevelPb {
                            SubLevelId: sid,
                            Cleared: sc > 0,
                            BestScore: sc,
                            BestAccuracy: if sc > 0 { 100 } else { 0 },
                            BestRank: match best_rank {
                                1 => Some(sonoro_proto::enums::RhythmShipRank::SSS),
                                2 => Some(sonoro_proto::enums::RhythmShipRank::SS),
                                3 => Some(sonoro_proto::enums::RhythmShipRank::S),
                                4 => Some(sonoro_proto::enums::RhythmShipRank::A),
                                5 => Some(sonoro_proto::enums::RhythmShipRank::B),
                                _ => None,
                            },
                        })
                    })
                    .collect();
                planets_map.entry(planet).or_default().push(RhythmShipLevelPb {
                    LevelId: lid,
                    RhythmSubLevelPb: subs,
                });
            }
            let planets = planets_map
                .into_iter()
                .map(|(pid, lvs)| RhythmShipPlanetPb {
                    PlanetId: pid,
                    OpenTime: begin_s,
                    RhythmShipLevelPb: lvs,
                })
                .collect();
            let task_list = cond_task_list_from_rows(
                &rows_for_activity(assets, "RhythmTask", id),
                progress_comp,
                cond_ctx,
                "rhythm:task:",
            );
            ActivityDataDataOneof::RhythmActivityPb(RhythmActivityPb {
                RhythmShipPlanetPb: planets,
                RhythmTask: vec![RhythmTaskPb {
                    TaskType: Some(RhythmTaskTypePb::Resident),
                    Task: task_list,
                }],
                ..Default::default()
            })
        }
        ActivityType::FeiXue => {
            let infos = rows_for_activity(assets, "FeiXuePreheat", id)
                .iter()
                .filter_map(|o| {
                    let fid = j_i32(o, "Id");
                    (fid > 0).then_some(FeiXuePreheatInfo {
                        Id: fid,
                        State: if claim_matches_id(claims, "feixue:", fid) { 2 } else { 1 },
                        QuestUnlockTime: begin_s,
                    })
                })
                .collect();
            ActivityDataDataOneof::FeiXuePreheatActivityInfo(FeiXuePreheatActivityInfo {
                FeiXuePreheatInfos: infos,
            })
        }
        ActivityType::DropCatchActivity => {
            let lvs = rows_for_activity(assets, "DropCatchGameplay", id)
                .iter()
                .filter_map(|o| {
                    let did = j_i32(o, "Id");
                    (did > 0).then_some(DropCatchLevelInfo {
                        DropCatchId: did,
                        RewardStates: Vec::new(),
                        UnlockTime: begin_s,
                        Score: info
                            .get_key_int(&format!("dropcatch:score:{did}"))
                            .max(info.get_key_int(&format!("dc:score:{did}")))
                            .max(0) as i32,
                    })
                })
                .collect();
            ActivityDataDataOneof::DropCatchActivityInfo(DropCatchActivityInfo {
                DropCatchLevelInfos: lvs,
            })
        }
        ActivityType::TetrisActivity => {
            let lvs = rows_for_activity(assets, "Tetris", id)
                .iter()
                .filter_map(|o| {
                    let tid = j_i32(o, "Id");
                    (tid > 0).then_some(TetrisLevelInfo {
                        Id: tid,
                        ..Default::default()
                    })
                })
                .collect();
            ActivityDataDataOneof::TetrisActivityInfo(TetrisActivityInfo {
                TetrisLevelInfos: lvs,
            })
        }
        ActivityType::Kurotato => {
            let lvs = rows_for_activity(assets, "KurotatoLevel", id)
                .iter()
                .filter_map(|o| {
                    let lid = j_i32(o, "Id");
                    (lid > 0).then_some(KurotatoLevelInfo {
                        LevelId: lid,
                        IsUnlock: true,
                        UnlockTime: begin_s,
                        IsFinished: info.get_key_int(&format!("kurotato:lv:{lid}")) > 0
                            || info.get_key_int(&format!("kurotato:lvl:{lid}")) > 0,
                        ..Default::default()
                    })
                })
                .collect();
            let lim = cond_task_list_from_rows(
                &rows_for_activity(assets, "KurotatoLimitAward", id),
                progress_comp,
                cond_ctx,
                "kurotato:lim:",
            );
            let res = cond_task_list_from_rows(
                &rows_for_activity(assets, "KurotatoResAward", id),
                progress_comp,
                cond_ctx,
                "kurotato:res:",
            );
            ActivityDataDataOneof::KurotatoActivityData(KurotatoActivityData {
                KurotatoLevelInfos: lvs,
                ResTasks: res,
                LimitTasks: lim,
                ..Default::default()
            })
        }
        ActivityType::PinballActivity => {
            let chaps = rows_for_activity(assets, "PinballChapterConfig", id)
                .iter()
                .filter_map(|o| {
                    let cid = j_i32(o, "Id");
                    (cid > 0).then_some(PinballChapterData {
                        ChapterId: cid,
                        UnLockTime: begin_s,
                    })
                })
                .collect();
            let lvs = assets
                .raw_json_cached("PinballLevelConfig")
                .iter()
                .filter_map(|v| v.as_object())
                .filter_map(|o| {
                    let cid = j_i32(o, "Id");
                    (cid > 0).then_some(PinballLevelData {
                        ConfigId: cid,
                        data: None,
                    })
                })
                .collect();
            let tasks = cond_task_list_from_rows(
                &rows_for_activity(assets, "PinballTask", id),
                progress_comp,
                cond_ctx,
                "pinball:task:",
            );
            ActivityDataDataOneof::PinballActivityData(PinballActivityData {
                Chapters: chaps,
                Levels: lvs,
                ConditionTasks: tasks,
                ..Default::default()
            })
        }
        ActivityType::BossPiling => {
            let tasks = cond_task_list_from_rows(
                &rows_for_activity(assets, "BossPilingTask", id),
                progress_comp,
                cond_ctx,
                "bosspiling:task:",
            );
            let lvs = rows_for_activity(assets, "BossPilingLevels", id)
                .iter()
                .filter_map(|o| {
                    let lid = j_i32(o, "Id");
                    (lid > 0).then_some(BossPilingLevelInfo {
                        id: lid,
                        UnlockTime: begin_s,
                        BossHpNum: info.get_key_int(&format!("bpiling:hp:{lid}")).max(0) as i32,
                        ..Default::default()
                    })
                })
                .collect();
            ActivityDataDataOneof::BossPilingActivityInfo(BossPilingActivityInfo {
                ConditionTasks: tasks,
                BossPilingLevelInfos: lvs,
            })
        }
        ActivityType::EdgeRunnerActivity => {
            let tasks = cond_task_list_from_rows(
                &rows_for_activity(assets, "EdgeRunnerReward", id),
                progress_comp,
                cond_ctx,
                "edge:rw:",
            );
            ActivityDataDataOneof::EdgeRunnerActivityInfo(EdgeRunnerActivityInfo {
                ConditionTasks: tasks,
                ..Default::default()
            })
        }
        ActivityType::MotorDecalActivity => {
            let tasks = cond_task_list_from_rows(
                &assets
                    .raw_json_cached("MotorDecalQuest")
                    .iter()
                    .filter_map(|v| v.as_object().cloned())
                    .collect::<Vec<_>>(),
                progress_comp,
                cond_ctx,
                "motor:decal_task:",
            );
            ActivityDataDataOneof::MotorDecalActivityData(MotorDecalActivityData {
                ConditionTasks: tasks,
            })
        }
        ActivityType::MotorParkour => {
            let lvs = rows_for_activity(assets, "OnlineMotorLevel", id)
                .iter()
                .filter_map(|o| {
                    let lid = j_i32(o, "Id");
                    (lid > 0).then_some(OnlineMotorLevelInfo {
                        LevelId: lid,
                        Ranking: info.get_key_int(&format!("om:rank:{lid}")).max(0) as i32,
                        TimeCost: info.get_key_int(&format!("om:time:{lid}")).max(0) as i32,
                    })
                })
                .collect();
            let unlocks = rows_for_activity(assets, "OnlineMotorLevel", id)
                .iter()
                .filter_map(|o| {
                    let lid = j_i32(o, "Id");
                    (lid > 0).then_some(OnlineMotorLevelUnLockTime {
                        LevelId: lid,
                        UnLockTime: begin_s,
                    })
                })
                .collect();
            ActivityDataDataOneof::OnlineMotorActivityData(OnlineMotorActivityData {
                OnlineMotorLevelInfos: lvs,
                UnLocks: unlocks,
                ..Default::default()
            })
        }
        ActivityType::LinkageCheckIn => {
            let normal = rows_for_activity(assets, "LinkageReward", id)
                .iter()
                .filter_map(|o| {
                    let rid = j_i32(o, "Id");
                    (rid > 0 && claim_matches_id(claims, "linkage:norm:", rid)).then_some(rid)
                })
                .collect();
            ActivityDataDataOneof::LinkageCheckInActivityData(LinkageCheckInActivityData {
                CheckInDay: info.get_key_int(&format!("linkage:day:{id}")).max(1) as i32,
                NormalReward: normal,
                KeepReward: Vec::new(),
            })
        }
        ActivityType::RoleGiftActivity => {
            ActivityDataDataOneof::RoleGiftActivityData(RoleGiftActivityData {
                RewardHadGet: claim_matches_id(claims, "role_gift:", id),
            })
        }
        ActivityType::RealmBetween => {
            let tasks = cond_task_list_from_rows(
                &rows_for_activity(assets, "TravelTask", id),
                progress_comp,
                cond_ctx,
                "realm:task:",
            );
            ActivityDataDataOneof::RealmBetweenActivityInfo(RealmBetweenActivityInfo {
                ActivityTasks: tasks,
                RealmBetweenLevel: info.get_key_int("realmbetween:lvl").max(1) as i32,
                ..Default::default()
            })
        }
        ActivityType::RoverRogue => {
            let ins_entries = assets
                .raw_json_cached("RoverRogueIns")
                .iter()
                .filter_map(|v| v.as_object())
                .filter(|o| j_i32(o, "ActivityId") == id || j_i32(o, "ActivityId") == 0)
                .filter_map(|o| {
                    let iid = j_i32(o, "InstId");
                    (iid > 0).then_some(RoverRogueInsEntry {
                        instId: iid,
                        DifficultyName: o
                            .get("Name")
                            .and_then(Value::as_str)
                            .unwrap_or("")
                            .as_bytes()
                            .to_vec(),
                        Level: j_i32(o, "Difficulty"),
                        ExpectedTimeMinSec: 60,
                        ExpectedTimeMaxSec: 300,
                        Unlocked: true,
                        BestGrade: Some(RoverRogueGrade::RoverRogueGradeS),
                    })
                })
                .collect();
            let loots: Vec<i32> = assets.rover_rogue_loots().iter().map(|l| l.Id).collect();
            let blesses: Vec<i32> = assets
                .raw_json_cached("RoverRogueBless")
                .iter()
                .filter_map(|v| v.as_object().map(|o| j_i32(o, "Id")))
                .filter(|&bid| bid > 0)
                .collect();
            ActivityDataDataOneof::RoverRogueActivityData(RoverRogueActivityData {
                activityId: id,
                HistoryInsInfo: Some(RoverRogueHistoryInsInfo {
                    CurInsId: info.get_key_int("rover_rogue:inst") as i32,
                    CurLayer: info.get_key_int("rover_rogue:layer") as i32,
                    TotalLayer: assets
                        .raw_json_cached("RoverRogueIns")
                        .iter()
                        .filter_map(|v| v.as_object())
                        .find(|o| j_i32(o, "ActivityId") == id || j_i32(o, "ActivityId") == 0)
                        .and_then(|o| j_i32_vec(o, "LevelRoomNumParam").first().copied())
                        .unwrap_or(8),
                    LootId: info.get_key_int("rover_rogue:loot") as i32,
                }),
                UnlockBlessIdList: blesses,
                UnlockLootIdList: loots,
                InsInfoList: ins_entries,
                ..Default::default()
            })
        }
        ActivityType::DangoBrawl => {
            let chs = rows_for_activity(assets, "DangoBrawlChallenge", id)
                .iter()
                .filter_map(|o| {
                    let cid = j_i32(o, "Id");
                    (cid > 0).then_some(DangoBrawlChallengeInfo {
                        ChallengeId: cid,
                        State: Some(if info.get_key_int(&format!("db:clear:{cid}")) > 0 {
                            DangoBrawlChallengeState::Passed
                        } else {
                            DangoBrawlChallengeState::Unlocked
                        }),
                        RetryCount: 0,
                    })
                })
                .collect();
            let rws = rows_for_activity(assets, "DangoBrawlScoreReward", id)
                .iter()
                .filter_map(|o| {
                    let rid = j_i32(o, "Id");
                    (rid > 0 && claim_matches_id(claims, "db:score:", rid)).then_some(rid)
                })
                .collect();
            ActivityDataDataOneof::DangoBrawlActivityInfo(DangoBrawlActivityInfo {
                ChallengeInfos: chs,
                RewardScoreIds: rws,
                ..Default::default()
            })
        }
        ActivityType::QingXiaoPlayThePiano => {
            let tasks = cond_task_list_from_rows(
                &rows_for_activity(assets, "GuQinActivityTask", id),
                progress_comp,
                cond_ctx,
                "qingxiao:",
            );
            ActivityDataDataOneof::QingXiaoActivityInfo(QingXiaoActivityInfo {
                ConditionTasks: tasks,
            })
        }
        ActivityType::MatchThreeActivity => {
            let lvs = assets
                .raw_json_cached("MatchThreeGameplay")
                .iter()
                .filter_map(|v| v.as_object())
                .filter_map(|o| {
                    let lid = j_i32(o, "Id");
                    (lid > 0).then_some(MatchThreeLevelInfo {
                        Id: lid,
                        UnlockTime: begin_s,
                        FirstPass: info.get_key_int(&format!("match3:pass:{lid}")) > 0,
                        ..Default::default()
                    })
                })
                .collect();
            ActivityDataDataOneof::MatchThreeActivityInfo(MatchThreeActivityInfo {
                MatchThreeLevelInfos: lvs,
            })
        }
        ActivityType::GiantSystemActivity => {
            let mut total = 0i64;
            for eco_type in 0..3 {
                total += info.get_key_int(&format!("giant:stable:{eco_type}")).max(0);
            }
            let got = assets
                .raw_json_cached("GiantDeliveryReward")
                .iter()
                .filter_map(|v| v.as_object())
                .filter_map(|o| {
                    let gid = j_i32(o, "Id");
                    (gid > 0 && claim_matches_id(claims, "giant:rw:", gid)).then_some(gid)
                })
                .collect();
            ActivityDataDataOneof::GiantActivityInfo(GiantActivityInfo {
                TotalStableValue: total.min(i64::from(i32::MAX)) as i32,
                GotRewardIds: got,
                ..Default::default()
            })
        }
        ActivityType::RoleTower => {
            let mut chs = Vec::new();
            for o in assets
                .raw_json_cached("RoleTowerLevel")
                .iter()
                .filter_map(|v| v.as_object())
            {
                let cid = j_i32(o, "Id");
                if cid <= 0 {
                    continue;
                }
                chs.push(RoleTowerChallengeInfo {
                    ChallengeId: cid,
                    IsUnlock: true,
                    UnlockTime: begin_s,
                    IsAllPassed: info.get_key_int("role_tower:stage") > 0,
                    ..Default::default()
                });
            }
            let score_tasks = rows_for_activity(assets, "RoleTowerScoreReward", id)
                .iter()
                .filter_map(|o| {
                    let sid = j_i32(o, "Id");
                    (sid > 0 && claim_matches_id(claims, "roletower:score:", sid)).then_some(sid)
                })
                .collect();
            let lim_tasks = cond_task_list_from_rows(
                &rows_for_activity(assets, "RoleTowerReward", id),
                progress_comp,
                cond_ctx,
                "roletower:task:",
            );
            ActivityDataDataOneof::RoleTowerActivityData(RoleTowerActivityData {
                RankRewardSettleTime: end_s,
                ChallengeInfos: chs,
                ScoreTasks: score_tasks,
                LimitTasks: lim_tasks,
                Score: info.get_key_int("role_tower:score") as i32,
                ..Default::default()
            })
        }
        ActivityType::RegressActivity => {
            let tasks = act_task_list_from_rows(
                &assets
                    .raw_json_cached("RegressQuest")
                    .iter()
                    .filter_map(|v| v.as_object().cloned())
                    .collect::<Vec<_>>(),
                progress_comp,
                cond_ctx,
                "regress:task:",
            );
            ActivityDataDataOneof::ActivityRegressData(ActivityRegressData {
                TaskProgressReward: tasks,
                EndTime: end_s,
                RefreshTime: end_s,
                Grade: 1,
                ..Default::default()
            })
        }
        _ => return None,
    })
}
