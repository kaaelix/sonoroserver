//! Explicit gameplay, combat, minigame, and system handlers matching `ExtraHandlers.cs`.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use serde_json::Value;
use sonoro_proto::PbMessage;
use sonoro_proto::enums::*;
use sonoro_proto::pb::*;
use sonoro_proto::registry::msg_id;

use crate::bin_data;
use crate::handlers::HandlerFn;
use crate::ipc::AkiResult;
use crate::sqlite_store;

fn ok_resp<M: PbMessage>(resp_id: u16, resp: &M) -> AkiResult {
    AkiResult::ok(resp_id, resp)
}

#[derive(Default)]
struct ExtraTables {
    soar_challenges: HashMap<i32, (i32, i32, i32)>,        // Id -> (UnlockTravelLevel, NeedScore, RewardDrop)
    travel_tasks: HashMap<i32, i32>,                       // TaskId -> TaskRewardDrop
    rogue_weekly_rewards: HashMap<i32, (i32, i32, i32)>,   // Id -> (CycleId, Score, TargetRewardDrop)
    risk_buff_groups: HashMap<i32, (i32, i32, i32)>,       // Id -> (BuffType, BuffProgress, RewardDrop)
    risk_score_rewards: HashMap<i32, i32>,                 // Id -> RewardDrop
    cornice_challenges: HashMap<i32, Vec<(i32, i32)>>,     // Id -> [(ScoreNeed, DropId)]
    circum_tasks: HashMap<i32, i32>,                       // Id -> TargetRewardDrop
    pb_master_levels: HashMap<i32, (i32, i32, i32)>,       // Id -> (ExpNeed, NormalDropId, UnlockDropId)
    rogue_cat_rewards: HashMap<i32, (i32, i32)>,           // Id -> (NeedEnergy, DropId)
    phantom_recommends: HashMap<i32, Vec<i32>>,            // RoleId -> RecommendFetterGroups
    rogue_seasons: Vec<i32>,
    rogue_season_rewards: HashMap<i32, Vec<i32>>,
    rover_rogue_talents: Vec<(i32, i32)>,                  // (Id, Consule)
    floro_cards: Vec<i32>,
    floro_techs: Vec<i32>,
    floro_races: Vec<i32>,
    racing_leg_matches: HashMap<i32, (i32, Vec<i32>)>,     // Id -> (MatchId, Organ/Dango defaults)
    pb_cards: HashSet<i32>,
    pb_skills: HashSet<i32>,
}

static EXTRA_TABLES: OnceLock<ExtraTables> = OnceLock::new();

fn read_table_array(table_name: &str) -> Vec<Value> {
    let db_candidates = [
        bin_data::repo_root().join("assets").join("assets.db"),
        PathBuf::from("assets/assets.db"),
        PathBuf::from("../../assets/assets.db"),
    ];
    for db_path in db_candidates {
        if db_path.is_file()
            && let Ok(conn) = rusqlite::Connection::open_with_flags(
                &db_path,
                rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
            )
        {
            if let Ok(raw) = conn.query_row(
                "SELECT data FROM raw_files WHERE name = ?1",
                rusqlite::params![table_name],
                |row| row.get::<_, String>(0),
            ) && let Ok(arr) = serde_json::from_str::<Vec<Value>>(&raw)
            {
                return arr;
            }
            if let Ok(mut stmt) = conn.prepare("SELECT data FROM kv_int WHERE tbl = ?1")
                && let Ok(rows) = stmt.query_map(rusqlite::params![table_name], |r| r.get::<_, String>(0))
            {
                let mut out = Vec::new();
                for raw in rows.flatten() {
                    if let Ok(v) = serde_json::from_str::<Value>(&raw) {
                        out.push(v);
                    }
                }
                if !out.is_empty() {
                    return out;
                }
            }
        }
    }
    let json_path = bin_data::bindata_dir().join(format!("{table_name}.json"));
    fs::read(json_path)
        .ok()
        .and_then(|bytes| serde_json::from_slice::<Vec<Value>>(&bytes).ok())
        .unwrap_or_default()
}

fn j_i32(v: &Value, key: &str) -> i32 {
    v.get(key)
        .and_then(Value::as_i64)
        .and_then(|n| i32::try_from(n).ok())
        .unwrap_or(0)
}

fn j_i32_vec(v: &Value, key: &str) -> Vec<i32> {
    v.get(key)
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_i64().and_then(|n| i32::try_from(n).ok()))
                .filter(|&id| id > 0)
                .collect()
        })
        .unwrap_or_default()
}

fn extra_tables() -> &'static ExtraTables {
    EXTRA_TABLES.get_or_init(|| {
        let mut t = ExtraTables::default();
        for r in read_table_array("SoarChallenge") {
            let id = j_i32(&r, "Id");
            if id > 0 {
                t.soar_challenges
                    .insert(id, (j_i32(&r, "UnlockTravelLevel"), j_i32(&r, "NeedScore"), j_i32(&r, "Reward")));
            }
        }
        for r in read_table_array("TravelTask") {
            let tid = j_i32(&r, "TaskId");
            if tid > 0 {
                t.travel_tasks.insert(tid, j_i32(&r, "TaskReward"));
            }
        }
        for r in read_table_array("RogueWeeklyReward") {
            let id = j_i32(&r, "Id");
            if id > 0 {
                t.rogue_weekly_rewards.insert(
                    id,
                    (j_i32(&r, "CycleId"), j_i32(&r, "Score"), j_i32(&r, "TargetReward")),
                );
            }
        }
        for r in read_table_array("RiskHarvestBuffGroup") {
            let id = j_i32(&r, "Id");
            if id > 0 {
                t.risk_buff_groups.insert(
                    id,
                    (j_i32(&r, "BuffType"), j_i32(&r, "BuffProgress"), j_i32(&r, "Reward")),
                );
            }
        }
        for r in read_table_array("RiskHarvestScoreReward") {
            let id = j_i32(&r, "Id");
            if id > 0 {
                t.risk_score_rewards.insert(id, j_i32(&r, "Reward"));
            }
        }
        for r in read_table_array("CorniceChallenge") {
            let id = j_i32(&r, "Id");
            if id > 0 {
                let mut pairs = Vec::new();
                if let Some(arr) = r.get("RewardList").and_then(Value::as_array) {
                    for el in arr {
                        let s = j_i32(el, "Item1");
                        let d = j_i32(el, "Item2");
                        pairs.push((s, d));
                    }
                }
                t.cornice_challenges.insert(id, pairs);
            }
        }
        for r in read_table_array("CircumFluenceTask") {
            let id = j_i32(&r, "Id");
            if id > 0 {
                t.circum_tasks.insert(id, j_i32(&r, "TargetReward"));
            }
        }
        for r in read_table_array("PhantomBattleMasterLevel") {
            let id = j_i32(&r, "Id");
            if id > 0 {
                t.pb_master_levels.insert(
                    id,
                    (j_i32(&r, "ExpNeed"), j_i32(&r, "NormalDropId"), j_i32(&r, "UnlockDropId")),
                );
            }
        }
        for r in read_table_array("RogueWhiteCatReward") {
            let id = j_i32(&r, "Id");
            if id > 0 {
                t.rogue_cat_rewards
                    .insert(id, (j_i32(&r, "NeedEnergy"), j_i32(&r, "DropId")));
            }
        }
        for r in read_table_array("PhantomRecommend") {
            let role_id = j_i32(&r, "RoleId");
            if role_id > 0 {
                t.phantom_recommends
                    .insert(role_id, j_i32_vec(&r, "RecommendFetterGroups"));
            }
        }
        for r in read_table_array("RogueSeason") {
            let id = j_i32(&r, "Id");
            if id > 0 {
                t.rogue_seasons.push(id);
            }
        }
        for r in read_table_array("RogueSeasonReward") {
            let id = j_i32(&r, "Id");
            let sid = j_i32(&r, "SeasonId");
            if id > 0 && sid > 0 {
                t.rogue_season_rewards.entry(sid).or_default().push(id);
            }
        }
        for r in read_table_array("RoverRogueTalentTree") {
            let id = j_i32(&r, "Id");
            if id > 0 {
                t.rover_rogue_talents.push((id, j_i32(&r, "Consule")));
            }
        }
        for r in read_table_array("FloroRanchCard") {
            let id = j_i32(&r, "Id");
            if id > 0 {
                t.floro_cards.push(id);
            }
        }
        for r in read_table_array("FloroRanchTech") {
            let id = j_i32(&r, "Id");
            if id > 0 {
                t.floro_techs.push(id);
            }
        }
        for r in read_table_array("FloroRanchRace") {
            let id = j_i32(&r, "Id");
            if id > 0 {
                t.floro_races.push(id);
            }
        }
        for r in read_table_array("RacingBetsLegMatches") {
            let id = j_i32(&r, "Id");
            if id > 0 {
                t.racing_leg_matches
                    .insert(id, (j_i32(&r, "MatchId"), vec![1, 2, 3, 4]));
            }
        }
        for r in read_table_array("PhantomBattleCard") {
            let id = j_i32(&r, "Id");
            if id > 0 {
                t.pb_cards.insert(id);
            }
        }
        for r in read_table_array("PhantomBattleSkill") {
            let id = j_i32(&r, "Id");
            if id > 0 {
                t.pb_skills.insert(id);
            }
        }
        t
    })
}

fn load_formation_state(pid: i32) -> (i32, BTreeMap<i32, (Vec<i32>, i32)>) {
    let mut current_id = 1i32;
    let mut teams: BTreeMap<i32, (Vec<i32>, i32)> = BTreeMap::new();
    if pid > 0 {
        if let Some(raw) = sqlite_store::get_kv(pid, "formation", "state_json")
            && let Ok(Value::Object(obj)) = serde_json::from_str::<Value>(&raw)
        {
            if let Some(cur) = obj.get("current_id").and_then(Value::as_i64) {
                current_id = cur.max(1) as i32;
            }
            if let Some(Value::Object(map)) = obj.get("teams") {
                for (k, v) in map {
                    if let Ok(fid) = k.parse::<i32>()
                        && fid > 0
                    {
                        let roles = j_i32_vec(v, "roles");
                        let role_idx = j_i32(v, "role_index");
                        teams.insert(fid, (roles, role_idx));
                    }
                }
            }
        } else if let Some(cur_str) = sqlite_store::get_kv(pid, "formation", "current_id")
            && let Ok(cid) = cur_str.parse::<i32>()
            && cid > 0
        {
            current_id = cid;
        }
    }
    if teams.is_empty() {
        teams.insert(current_id, (Vec::new(), 0));
    }
    (current_id, teams)
}

fn save_formation_state(pid: i32, current_id: i32, teams: &BTreeMap<i32, (Vec<i32>, i32)>) {
    if pid <= 0 {
        return;
    }
    let mut map = serde_json::Map::new();
    for (&fid, (roles, role_idx)) in teams {
        map.insert(
            fid.to_string(),
            serde_json::json!({ "roles": roles, "role_index": role_idx }),
        );
    }
    let payload = serde_json::json!({
        "current_id": current_id,
        "teams": map,
    });
    sqlite_store::set_kv(pid, "formation", "state_json", &payload.to_string());
    sqlite_store::set_kv(pid, "formation", "current_id", &current_id.to_string());
}

fn build_formation_pb(id: i32, current_id: i32, roles: &[i32], role_idx: i32) -> Formation {
    Formation {
        Id: id,
        RoleIndex: role_idx,
        Role: roles.to_vec(),
        IsCurrent: id == current_id,
    }
}

fn resolve_drop_or_fallback(drop_id: i32, fallback: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let drops = bin_data::get().resolve_drop(drop_id);
    if drops.is_empty() {
        fallback.to_vec()
    } else {
        drops
    }
}

fn grant_validated_claim<M: PbMessage>(
    player_id: i32,
    claim_key: &str,
    aliases: &[String],
    resp_id: u16,
    ok_body: &M,
    grants: Vec<(i32, i32)>,
) -> AkiResult {
    let mut result = AkiResult::ok(resp_id, ok_body);
    if player_id <= 0 {
        return result;
    }
    let sql_claims = sqlite_store::get_activity_claims(player_id);
    let (_, zon_claims) = sqlite_store::get_progress_state(player_id);
    let already = sql_claims.contains(claim_key)
        || zon_claims.contains(claim_key)
        || aliases
            .iter()
            .any(|k| sql_claims.contains(k) || zon_claims.contains(k));
    if !already {
        sqlite_store::set_activity_claim(player_id, claim_key, "1");
        result = result.with_grants(grants);
    }
    result
}

pub fn register_all(map: &mut HashMap<u16, HandlerFn>) {
    // Formation & Role / Phantom Recommend
    map.insert(msg_id::FORMATION_REQUEST, |pid, _| {
        let (current_id, teams) = load_formation_state(pid);
        let list = teams
            .iter()
            .map(|(&fid, (roles, role_idx))| build_formation_pb(fid, current_id, roles, *role_idx))
            .collect();
        ok_resp(
            msg_id::FORMATION_RESPONSE,
            &FormationResponse {
                ErrorCode: Some(ErrorCode::Success),
                Formation: list,
            },
        )
    });
    map.insert(msg_id::FORMATION_MODIFY_REQUEST, |pid, payload| {
        let req = FormationModifyRequest::decode(payload).unwrap_or_default();
        if req.Id <= 0 {
            return ok_resp(
                msg_id::FORMATION_MODIFY_RESPONSE,
                &FormationModifyResponse {
                    ErrorCode: Some(ErrorCode::RequestParamError),
                    Formation: None,
                },
            );
        }
        let (current_id, mut teams) = load_formation_state(pid);
        let entry = teams.entry(req.Id).or_insert_with(|| (Vec::new(), 0));
        entry.0 = req.Role.clone();
        let pb_form = build_formation_pb(req.Id, current_id, &entry.0, entry.1);
        save_formation_state(pid, current_id, &teams);
        ok_resp(
            msg_id::FORMATION_MODIFY_RESPONSE,
            &FormationModifyResponse {
                ErrorCode: Some(ErrorCode::Success),
                Formation: Some(pb_form),
            },
        )
    });
    map.insert(msg_id::FORMATION_CHANGE_REQUEST, |pid, payload| {
        let req = FormationChangeRequest::decode(payload).unwrap_or_default();
        let (mut current_id, mut teams) = load_formation_state(pid);
        for item in &req.ChangeList {
            if item.Id > 0 {
                let entry = teams.entry(item.Id).or_insert_with(|| (Vec::new(), 0));
                entry.0 = item.Role.clone();
            }
        }
        if req.FormationId > 0 {
            current_id = req.FormationId;
            teams.entry(current_id).or_insert_with(|| (Vec::new(), 0));
        }
        save_formation_state(pid, current_id, &teams);
        ok_resp(
            msg_id::FORMATION_CHANGE_RESPONSE,
            &FormationChangeResponse {
                ErrorCode: Some(ErrorCode::Success),
                FormationId: current_id,
                ..Default::default()
            },
        )
    });
    map.insert(msg_id::FORMATION_SWITCH_CURRENT_REQUEST, |pid, payload| {
        let req = FormationSwitchCurrentRequest::decode(payload).unwrap_or_default();
        if req.Id <= 0 {
            return ok_resp(
                msg_id::FORMATION_SWITCH_CURRENT_RESPONSE,
                &FormationSwitchCurrentResponse {
                    ErrorCode: Some(ErrorCode::RequestParamError),
                    CurrentId: 0,
                },
            );
        }
        let (_, mut teams) = load_formation_state(pid);
        teams.entry(req.Id).or_insert_with(|| (Vec::new(), 0));
        save_formation_state(pid, req.Id, &teams);
        ok_resp(
            msg_id::FORMATION_SWITCH_CURRENT_RESPONSE,
            &FormationSwitchCurrentResponse {
                ErrorCode: Some(ErrorCode::Success),
                CurrentId: req.Id,
            },
        )
    });
    map.insert(msg_id::FORMATION_SWITCH_ROLE_REQUEST, |pid, payload| {
        let req = FormationSwitchRoleRequest::decode(payload).unwrap_or_default();
        let (current_id, mut teams) = load_formation_state(pid);
        if let Some(entry) = teams.get_mut(&current_id) {
            entry.1 = req.RoleIndex;
            save_formation_state(pid, current_id, &teams);
        }
        ok_resp(
            msg_id::FORMATION_SWITCH_ROLE_RESPONSE,
            &FormationSwitchRoleResponse {
                ErrorCode: Some(ErrorCode::Success),
                RoleIndex: req.RoleIndex,
            },
        )
    });
    map.insert(msg_id::PHANTOM_RECOMMEND_REQUEST, |_, payload| {
        let req = PhantomRecommendRequest::decode(payload).unwrap_or_default();
        let fetter = extra_tables()
            .phantom_recommends
            .get(&req.RoleId)
            .and_then(|groups| groups.first().copied())
            .unwrap_or(0);
        ok_resp(
            msg_id::PHANTOM_RECOMMEND_RESPONSE,
            &PhantomRecommendResponse {
                ErrorCode: Some(ErrorCode::Success),
                RoleId: req.RoleId,
                FetterGroupId: fetter,
                ..Default::default()
            },
        )
    });
    map.insert(msg_id::PHANTOM_RETURN_REQUEST, |_, _| {
        ok_resp(
            msg_id::PHANTOM_RETURN_RESPONSE,
            &PhantomReturnResponse {
                ErrorCode: Some(ErrorCode::Success),
                ..Default::default()
            },
        )
    });
    map.insert(msg_id::PHANTOM_RETURN_PREVIEW_REQUEST, |_, _| {
        ok_resp(
            msg_id::PHANTOM_RETURN_PREVIEW_RESPONSE,
            &PhantomReturnPreviewResponse {
                ErrorCode: Some(ErrorCode::Success),
                ..Default::default()
            },
        )
    });
    map.insert(msg_id::UPDATE_PHANTOM_EQUIP_GROUP_REQUEST, |_, _| {
        ok_resp(
            msg_id::UPDATE_PHANTOM_EQUIP_GROUP_RESPONSE,
            &UpdatePhantomEquipGroupResponse {
                ErrorCode: Some(ErrorCode::Success),
                ..Default::default()
            },
        )
    });
    map.insert(msg_id::UPDATE_PLAYER_PASSIVE_SKILL_REQUEST, |_, _| {
        ok_resp(
            msg_id::UPDATE_PLAYER_PASSIVE_SKILL_RESPONSE,
            &UpdatePlayerPassiveSkillResponse {
                ErrCode: Some(ErrorCode::Success),
            },
        )
    });
    map.insert(msg_id::BUFF_EFFECT_EXECUTE_REQUEST, |_, _| {
        ok_resp(
            msg_id::BUFF_EFFECT_EXECUTE_RESPONSE,
            &BuffEffectExecuteResponse {
                Code: Some(ErrorCode::Success),
            },
        )
    });
    map.insert(msg_id::MONTAGE_PLAY_REQUEST, |_, payload| {
        let req = MontagePlayRequest::decode(payload).unwrap_or_default();
        ok_resp(
            msg_id::MONTAGE_PLAY_RESPONSE,
            &MontagePlayResponse {
                MontageIndex: req.MontageIndex,
                Error: Some(DErrorResult::default()),
                ..Default::default()
            },
        )
    });
    map.insert(msg_id::ROLE_FAVOR_TALK_SCORE_REQUEST, |_, _| {
        ok_resp(
            msg_id::ROLE_FAVOR_TALK_SCORE_RESPONSE,
            &RoleFavorTalkScoreResponse {
                ErrCode: Some(ErrorCode::Success),
                IsComplete: true,
            },
        )
    });
    map.insert(msg_id::ROLE_QUEST_POINT_STATE_REQUEST, |_, _| {
        ok_resp(
            msg_id::ROLE_QUEST_POINT_STATE_RESPONSE,
            &RoleQuestPointStateResponse {
                CurrPoint: 0,
                MaxPoint: 100,
                ..Default::default()
            },
        )
    });
    map.insert(msg_id::CREAT_STORY_CHARACTER_REQUEST, |_, _| {
        ok_resp(
            msg_id::CREAT_STORY_CHARACTER_RESPONSE,
            &CreatStoryCharacterResponse {
                ErrorCode: Some(ErrorCode::Success),
                ..Default::default()
            },
        )
    });
    map.insert(msg_id::DELETE_STORY_CHARACTER_REQUEST, |_, _| {
        ok_resp(
            msg_id::DELETE_STORY_CHARACTER_RESPONSE,
            &DeleteStoryCharacterResponse {
                ErrorCode: Some(ErrorCode::Success),
                ..Default::default()
            },
        )
    });

    // World, Combat Init, Explore Skills, DragonPool & Items
    map.insert(msg_id::ACTIVE_DRAGON_POOL_REQUEST, |pid, payload| {
        let req = ActiveDragonPoolRequest::decode(payload).unwrap_or_default();
        if pid > 0 && req.DragonPoolId > 0 {
            sqlite_store::set_activity_claim(
                pid,
                &format!("dragon_pool:{}", req.DragonPoolId),
                "1",
            );
        }
        ok_resp(
            msg_id::ACTIVE_DRAGON_POOL_RESPONSE,
            &ActiveDragonPoolResponse {
                ErrorCode: Some(ErrorCode::Success),
                DragonPoolId: req.DragonPoolId,
                ActiveStatus: 1,
            },
        )
    });
    map.insert(msg_id::SPECIAL_ITEM_EQUIP_REQUEST, |pid, payload| {
        let req = SpecialItemEquipRequest::decode(payload).unwrap_or_default();
        if pid > 0 {
            sqlite_store::set_kv(pid, "special_item", "equipped_id", &req.ItemId.to_string());
            sqlite_store::set_kv(
                pid,
                "special_item",
                "equipped",
                if req.ItemId > 0 { "1" } else { "0" },
            );
        }
        ok_resp(
            msg_id::SPECIAL_ITEM_EQUIP_RESPONSE,
            &SpecialItemEquipResponse {
                ErrorCode: Some(ErrorCode::Success),
            },
        )
    });
    map.insert(msg_id::SPECIAL_ITEM_UN_EQUIP_REQUEST, |pid, _| {
        if pid > 0 {
            sqlite_store::set_kv(pid, "special_item", "equipped_id", "0");
            sqlite_store::set_kv(pid, "special_item", "equipped", "0");
        }
        ok_resp(
            msg_id::SPECIAL_ITEM_UN_EQUIP_RESPONSE,
            &SpecialItemUnEquipResponse {
                ErrorCode: Some(ErrorCode::Success),
            },
        )
    });
    map.insert(msg_id::UNLOCK_INST_ENTRANCE_REQUEST, |_, _| {
        ok_resp(
            msg_id::UNLOCK_INST_ENTRANCE_RESPONSE,
            &UnlockInstEntranceResponse {
                ErrorCode: Some(ErrorCode::Success),
            },
        )
    });
    map.insert(msg_id::ADD_REVIVE_REGION_REQUEST, |_, _| {
        ok_resp(
            msg_id::ADD_REVIVE_REGION_RESPONSE,
            &AddReviveRegionResponse {
                ErrorCode: Some(ErrorCode::Success),
            },
        )
    });
    map.insert(msg_id::REMOVE_REVIVE_REGION_REQUEST, |_, _| {
        ok_resp(
            msg_id::REMOVE_REVIVE_REGION_RESPONSE,
            &RemoveReviveRegionResponse {
                ErrorCode: Some(ErrorCode::Success),
            },
        )
    });
    map.insert(msg_id::ATTRIBUTE_INIT_REQUEST, |_, payload| {
        let req = AttributeInitRequest::decode(payload).unwrap_or_default();
        ok_resp(
            msg_id::ATTRIBUTE_INIT_RESPONSE,
            &AttributeInitResponse {
                Success: true,
                Id: req.Id,
            },
        )
    });
    map.insert(msg_id::PART_COMPONENT_INIT_REQUEST, |_, _| {
        ok_resp(
            msg_id::PART_COMPONENT_INIT_RESPONSE,
            &PartComponentInitResponse::default(),
        )
    });
    map.insert(msg_id::GAMEPLAY_TAG_REQUEST, |_, _| {
        ok_resp(
            msg_id::GAMEPLAY_TAG_RESPONSE,
            &GameplayTagResponse {
                ErrorCode: Some(ErrorCode::Success),
            },
        )
    });
    map.insert(msg_id::CREATURE_DETAIL_REQUEST, |_, payload| {
        let req = CreatureDetailRequest::decode(payload).unwrap_or_default();
        ok_resp(
            msg_id::CREATURE_DETAIL_RESPONSE,
            &CreatureDetailResponse {
                ErrorCode: Some(ErrorCode::Success),
                Id: req.Id,
                ConditionSatisfy: true,
                ..Default::default()
            },
        )
    });
    map.insert(msg_id::SCHEDULE_NPC_UPDATE_POS_REQUEST, |_, _| {
        ok_resp(
            msg_id::SCHEDULE_NPC_UPDATE_POS_RESPONSE,
            &ScheduleNpcUpdatePosResponse {
                ErrCode: Some(ErrorCode::Success),
            },
        )
    });
    map.insert(msg_id::POLLUTE_GRID_CELL_REQUEST, |_, _| {
        ok_resp(
            msg_id::POLLUTE_GRID_CELL_RESPONSE,
            &PolluteGridCellResponse {
                ErrorCode: Some(ErrorCode::Success),
            },
        )
    });
    map.insert(msg_id::REMOVE_INSTANCE_TIMER_REQUEST, |_, payload| {
        let req = RemoveInstanceTimerRequest::decode(payload).unwrap_or_default();
        ok_resp(
            msg_id::REMOVE_INSTANCE_TIMER_RESPONSE,
            &RemoveInstanceTimerResponse {
                ErrorCode: Some(ErrorCode::Success),
                TimerId: req.TimerId,
            },
        )
    });
    map.insert(msg_id::SET_ALERT_UI_VISIBLE_REQUEST, |_, _| {
        ok_resp(
            msg_id::SET_ALERT_UI_VISIBLE_RESPONSE,
            &SetAlertUiVisibleResponse {
                ErrorCode: Some(ErrorCode::Success),
            },
        )
    });
    map.insert(msg_id::CHECK_GEAR_REQUEST, |_, _| {
        ok_resp(
            msg_id::CHECK_GEAR_RESPONSE,
            &CheckGearResponse {
                ErrorCode: Some(ErrorCode::Success),
            },
        )
    });
    map.insert(msg_id::GRAVITY_GEAR_REQUEST, |_, _| {
        ok_resp(
            msg_id::GRAVITY_GEAR_RESPONSE,
            &GravityGearResponse {
                ErrorCode: Some(ErrorCode::Success),
            },
        )
    });
    map.insert(msg_id::EXPLORE_SKILL_PULL_GIANT_REQUEST, |_, payload| {
        let req = ExploreSkillPullGiantRequest::decode(payload).unwrap_or_default();
        ok_resp(
            msg_id::EXPLORE_SKILL_PULL_GIANT_RESPONSE,
            &ExploreSkillPullGiantResponse {
                Code: Some(ErrorCode::Success),
                EntityId: req.EntityId,
            },
        )
    });
    map.insert(msg_id::EXPLORE_SKILL_CUSTOM_REQUEST, |_, payload| {
        let req = ExploreSkillCustomRequest::decode(payload).unwrap_or_default();
        ok_resp(
            msg_id::EXPLORE_SKILL_CUSTOM_RESPONSE,
            &ExploreSkillCustomResponse {
                Code: Some(ErrorCode::Success),
                EntityId: req.EntityId,
            },
        )
    });
    map.insert(msg_id::USE_DETECTION_SKILL_REQUEST, |_, _| {
        ok_resp(
            msg_id::USE_DETECTION_SKILL_RESPONSE,
            &UseDetectionSkillResponse {
                Code: Some(ErrorCode::Success),
                ..Default::default()
            },
        )
    });
    map.insert(msg_id::R_TIME_STOP_ANIM_REQUEST, |_, _| {
        ok_resp(
            msg_id::R_TIME_STOP_ANIM_RESPONSE,
            &RTimeStopAnimResponse {
                ErrorCode: Some(ErrorCode::Success),
            },
        )
    });

    // YuXue Mode
    map.insert(msg_id::SET_PLAYER_YU_XUE_MODE_REQUEST, |pid, payload| {
        let req = SetPlayerYuXueModeRequest::decode(payload).unwrap_or_default();
        if pid > 0 {
            sqlite_store::set_kv(pid, "yuxue", "player_mode", &req.GlobalSetting.to_string());
        }
        ok_resp(
            msg_id::SET_PLAYER_YU_XUE_MODE_RESPONSE,
            &SetPlayerYuXueModeResponse {
                ErrorCode: Some(ErrorCode::Success),
            },
        )
    });
    map.insert(msg_id::SET_BT_OBJ_YU_XUE_MODE_REQUEST, |pid, payload| {
        let req = SetBtObjYuXueModeRequest::decode(payload).unwrap_or_default();
        if pid > 0 && req.BtObjId > 0 {
            let btype = req.BtType.map(|t| t as i32).unwrap_or(0);
            sqlite_store::set_kv(
                pid,
                "yuxue",
                &format!("bt:{btype}:{}", req.BtObjId),
                &req.BtObjSetting.to_string(),
            );
        }
        ok_resp(
            msg_id::SET_BT_OBJ_YU_XUE_MODE_RESPONSE,
            &SetBtObjYuXueModeResponse {
                ErrorCode: Some(ErrorCode::Success),
            },
        )
    });
    map.insert(msg_id::GET_YU_XUE_MODE_INFO_REQUEST, |_, _| {
        ok_resp(
            msg_id::GET_YU_XUE_MODE_INFO_RESPONSE,
            &GetYuXueModeInfoResponse {
                ErrorCode: Some(ErrorCode::Success),
            },
        )
    });
    map.insert(msg_id::GET_TRACE_QUEST_YU_XUE_MODE_REQUEST, |pid, _| {
        let active = pid > 0
            && sqlite_store::get_kv(pid, "yuxue", "player_mode")
                .and_then(|s| s.parse::<i32>().ok())
                .is_some_and(|v| v != 0);
        ok_resp(
            msg_id::GET_TRACE_QUEST_YU_XUE_MODE_RESPONSE,
            &GetTraceQuestYuXueModeResponse {
                ErrorCode: Some(ErrorCode::Success),
                InYuXueMode: active,
            },
        )
    });

    // Activity & Minigame Reward Handlers (table-validated + ZON/SQLite claim deduplication)
    map.insert(msg_id::MAP_TRAVEL_SOAR_REWARD_REQUEST, |pid, payload| {
        let req = MapTravelSoarRewardRequest::decode(payload).unwrap_or_default();
        let Some(&(unlock_lv, need_score, drop_id)) = extra_tables().soar_challenges.get(&req.TaskId)
        else {
            return ok_resp(
                msg_id::MAP_TRAVEL_SOAR_REWARD_RESPONSE,
                &MapTravelSoarRewardResponse {
                    ErrorCode: Some(ErrorCode::RequestParamError),
                },
            );
        };
        let (zints, zclaims) = sqlite_store::get_progress_state(pid);
        let lv = zints
            .get("map_travel:lv")
            .copied()
            .max(zints.get("map_travel:lvl").copied())
            .unwrap_or(1) as i32;
        let soar_score = zints
            .get("map_travel:soar_score")
            .or_else(|| zints.get("soar:score"))
            .copied()
            .unwrap_or(0) as i32;
        let has_score_gate = zints.contains_key("map_travel:soar_score") || zints.contains_key("soar:score");
        if lv < unlock_lv
            || (has_score_gate
                && need_score > 0
                && soar_score < need_score
                && !zclaims.contains(&format!("map_travel_soar_done:{}", req.TaskId)))
        {
            return ok_resp(
                msg_id::MAP_TRAVEL_SOAR_REWARD_RESPONSE,
                &MapTravelSoarRewardResponse {
                    ErrorCode: Some(ErrorCode::RequestParamError),
                },
            );
        }
        let grants = resolve_drop_or_fallback(drop_id, &[(3, 30)]);
        grant_validated_claim(
            pid,
            &format!("map_travel_soar:{}", req.TaskId),
            &[],
            msg_id::MAP_TRAVEL_SOAR_REWARD_RESPONSE,
            &MapTravelSoarRewardResponse {
                ErrorCode: Some(ErrorCode::Success),
            },
            grants,
        )
    });
    map.insert(msg_id::MAP_TRAVEL_TASK_REWARD_REQUEST, |pid, payload| {
        let req = MapTravelTaskRewardRequest::decode(payload).unwrap_or_default();
        let Some(&drop_id) = extra_tables().travel_tasks.get(&req.TaskId) else {
            return ok_resp(
                msg_id::MAP_TRAVEL_TASK_REWARD_RESPONSE,
                &MapTravelTaskRewardResponse {
                    ErrorCode: Some(ErrorCode::RequestParamError),
                },
            );
        };
        let grants = resolve_drop_or_fallback(drop_id, &[(3, 30)]);
        grant_validated_claim(
            pid,
            &format!("map_travel_task:{}", req.TaskId),
            &[format!("map_travel:task:{}", req.TaskId)],
            msg_id::MAP_TRAVEL_TASK_REWARD_RESPONSE,
            &MapTravelTaskRewardResponse {
                ErrorCode: Some(ErrorCode::Success),
            },
            grants,
        )
    });
    map.insert(msg_id::ROGUE_WEEKLY_REWARD_REQUEST, |pid, payload| {
        let req = RogueWeeklyRewardRequest::decode(payload).unwrap_or_default();
        let Some(&(_cycle_id, need_score, drop_id)) =
            extra_tables().rogue_weekly_rewards.get(&req.ConfigId)
        else {
            return ok_resp(
                msg_id::ROGUE_WEEKLY_REWARD_RESPONSE,
                &RogueWeeklyRewardResponse {
                    ErrorCode: Some(ErrorCode::RequestParamError),
                },
            );
        };
        let (zints, _) = sqlite_store::get_progress_state(pid);
        let cur_score = zints
            .get("rogue_weekly:score")
            .copied()
            .max(zints.get("rogue_weekly:total_score").copied())
            .unwrap_or(0);
        if need_score > 0 && cur_score < i64::from(need_score) {
            return ok_resp(
                msg_id::ROGUE_WEEKLY_REWARD_RESPONSE,
                &RogueWeeklyRewardResponse {
                    ErrorCode: Some(ErrorCode::RequestParamError),
                },
            );
        }
        let grants = resolve_drop_or_fallback(drop_id, &[(3, 80), (2, 20_000)]);
        grant_validated_claim(
            pid,
            &format!("rogue_weekly:rw:{}:{}", req.ActivityId, req.ConfigId),
            &[format!("rogue_weekly_rew:{}:{}", req.ActivityId, req.ConfigId)],
            msg_id::ROGUE_WEEKLY_REWARD_RESPONSE,
            &RogueWeeklyRewardResponse {
                ErrorCode: Some(ErrorCode::Success),
            },
            grants,
        )
    });
    map.insert(
        msg_id::RISK_HARVEST_BUFF_COUNT_REWARD_REQUEST,
        |pid, payload| {
            let req = RiskHarvestBuffCountRewardRequest::decode(payload).unwrap_or_default();
            if req.Id <= 0 || req.Id > extra_tables().risk_buff_groups.len().max(16) as i32 {
                return ok_resp(
                    msg_id::RISK_HARVEST_BUFF_COUNT_REWARD_RESPONSE,
                    &RiskHarvestBuffCountRewardResponse {
                        ErrorCode: Some(ErrorCode::RequestParamError),
                    },
                );
            }
            let (zints, zclaims) = sqlite_store::get_progress_state(pid);
            let unlocked_cnt = zclaims
                .iter()
                .filter(|c| c.starts_with("risk:buff:") || c.starts_with("risk_buff:"))
                .count() as i64;
            let tracked = zints
                .get("risk:buff_cnt")
                .copied()
                .unwrap_or(unlocked_cnt)
                .max(unlocked_cnt);
            let has_score = zints.get("risk:score").copied().unwrap_or(0) > 0;
            if !has_score && tracked < i64::from(req.Id) {
                return ok_resp(
                    msg_id::RISK_HARVEST_BUFF_COUNT_REWARD_RESPONSE,
                    &RiskHarvestBuffCountRewardResponse {
                        ErrorCode: Some(ErrorCode::RequestParamError),
                    },
                );
            }
            let drop_id = extra_tables()
                .risk_score_rewards
                .get(&req.Id)
                .copied()
                .unwrap_or(0);
            let grants = resolve_drop_or_fallback(drop_id, &[(3, 40), (2, 10_000)]);
            grant_validated_claim(
                pid,
                &format!("risk_buff_cnt:{}", req.Id),
                &[format!("risk:buff_cnt_clm:{}", req.Id)],
                msg_id::RISK_HARVEST_BUFF_COUNT_REWARD_RESPONSE,
                &RiskHarvestBuffCountRewardResponse {
                    ErrorCode: Some(ErrorCode::Success),
                },
                grants,
            )
        },
    );
    map.insert(
        msg_id::RISK_HARVEST_BUFF_UNLOCK_REWARD_REQUEST,
        |pid, payload| {
            let req = RiskHarvestBuffUnlockRewardRequest::decode(payload).unwrap_or_default();
            let Some(&(_btype, _bprog, drop_id)) = extra_tables().risk_buff_groups.get(&req.Id)
            else {
                return ok_resp(
                    msg_id::RISK_HARVEST_BUFF_UNLOCK_REWARD_RESPONSE,
                    &RiskHarvestBuffUnlockRewardResponse {
                        ErrorCode: Some(ErrorCode::RequestParamError),
                    },
                );
            };
            let (zints, zclaims) = sqlite_store::get_progress_state(pid);
            let unlocked = zclaims.contains(&format!("risk:buff:{}", req.Id))
                || zclaims.contains(&format!("risk_buff:{}", req.Id))
                || zints.get("risk:score").copied().unwrap_or(0) > 0;
            if !unlocked {
                return ok_resp(
                    msg_id::RISK_HARVEST_BUFF_UNLOCK_REWARD_RESPONSE,
                    &RiskHarvestBuffUnlockRewardResponse {
                        ErrorCode: Some(ErrorCode::RequestParamError),
                    },
                );
            }
            let grants = resolve_drop_or_fallback(drop_id, &[(3, 40), (2, 10_000)]);
            grant_validated_claim(
                pid,
                &format!("risk_buff_unlock:{}", req.Id),
                &[],
                msg_id::RISK_HARVEST_BUFF_UNLOCK_REWARD_RESPONSE,
                &RiskHarvestBuffUnlockRewardResponse {
                    ErrorCode: Some(ErrorCode::Success),
                },
                grants,
            )
        },
    );
    map.insert(msg_id::CORNICE_MEETING_REWARD_REQUEST, |pid, payload| {
        let req = CorniceMeetingRewardRequest::decode(payload).unwrap_or_default();
        let Some(reward_list) = extra_tables().cornice_challenges.get(&req.LevelPlayId) else {
            return ok_resp(
                msg_id::CORNICE_MEETING_REWARD_RESPONSE,
                &CorniceMeetingRewardResponse {
                    ErrorCode: Some(ErrorCode::RequestParamError),
                },
            );
        };
        let tier = if req.ScoreIndex > 0 {
            reward_list
                .get((req.ScoreIndex - 1) as usize)
                .or_else(|| reward_list.iter().find(|(s, _)| *s == req.ScoreIndex))
        } else {
            reward_list.first()
        };
        let Some(&(need_score, drop_id)) = tier else {
            return ok_resp(
                msg_id::CORNICE_MEETING_REWARD_RESPONSE,
                &CorniceMeetingRewardResponse {
                    ErrorCode: Some(ErrorCode::RequestParamError),
                },
            );
        };
        let (zints, _) = sqlite_store::get_progress_state(pid);
        let cur_score = zints
            .get(&format!("cornice:score:{}", req.LevelPlayId))
            .copied()
            .unwrap_or(0);
        if need_score > 0 && cur_score < i64::from(need_score) {
            return ok_resp(
                msg_id::CORNICE_MEETING_REWARD_RESPONSE,
                &CorniceMeetingRewardResponse {
                    ErrorCode: Some(ErrorCode::RequestParamError),
                },
            );
        }
        let grants = resolve_drop_or_fallback(drop_id, &[(3, 40), (2, 10_000)]);
        grant_validated_claim(
            pid,
            &format!("cornice:{}:{}", req.LevelPlayId, req.ScoreIndex),
            &[format!("cornice_rw:{}:{}", req.LevelPlayId, req.ScoreIndex)],
            msg_id::CORNICE_MEETING_REWARD_RESPONSE,
            &CorniceMeetingRewardResponse {
                ErrorCode: Some(ErrorCode::Success),
            },
            grants,
        )
    });
    map.insert(
        msg_id::CIRCUM_FLUENCE_TASK_REWARD_REQUEST,
        |pid, payload| {
            let req = CircumFluenceTaskRewardRequest::decode(payload).unwrap_or_default();
            if req.ActivityId <= 0 {
                return ok_resp(
                    msg_id::CIRCUM_FLUENCE_TASK_REWARD_RESPONSE,
                    &CircumFluenceTaskRewardResponse {
                        ErrorCode: Some(ErrorCode::RequestParamError),
                    },
                );
            }
            let rtype = req.RewardType.map(|r| r as i32).unwrap_or(0);
            let target_ids: Vec<i32> = req
                .Ids
                .iter()
                .copied()
                .filter(|id| {
                    extra_tables().circum_tasks.contains_key(id)
                        || (rtype == 3 && (1..=4).contains(id))
                })
                .collect();
            if target_ids.is_empty() {
                return ok_resp(
                    msg_id::CIRCUM_FLUENCE_TASK_REWARD_RESPONSE,
                    &CircumFluenceTaskRewardResponse {
                        ErrorCode: Some(ErrorCode::RequestParamError),
                    },
                );
            }
            let sql_claims = sqlite_store::get_activity_claims(pid);
            let (zints, zclaims) = sqlite_store::get_progress_state(pid);
            let mut grants = Vec::new();
            for tid in &target_ids {
                let claim_k = format!("circum:{}:{rtype}:{tid}", req.ActivityId);
                let legacy_k = format!("circum:{}:{rtype}", req.ActivityId);
                if sql_claims.contains(&claim_k)
                    || sql_claims.contains(&legacy_k)
                    || zclaims.contains(&claim_k)
                {
                    continue;
                }
                let done = zclaims.contains(&format!("circum:done:{tid}"))
                    || zclaims.contains(&format!("circum:task:{tid}"))
                    || zints.get("circum:score").copied().unwrap_or(0) > 0
                    || sql_claims.contains(&format!("circum_read:{}", req.ActivityId));
                if !done {
                    continue;
                }
                sqlite_store::set_activity_claim(pid, &claim_k, "1");
                let drop_id = extra_tables().circum_tasks.get(tid).copied().unwrap_or(0);
                grants.extend(resolve_drop_or_fallback(drop_id, &[(3, 40), (2, 10_000)]));
            }
            let mut res = ok_resp(
                msg_id::CIRCUM_FLUENCE_TASK_REWARD_RESPONSE,
                &CircumFluenceTaskRewardResponse {
                    ErrorCode: Some(ErrorCode::Success),
                },
            );
            if !grants.is_empty() {
                res = res.with_grants(grants);
            }
            res
        },
    );
    map.insert(
        msg_id::PHANTOM_BATTLE_MASTER_LEVEL_REWARD_REQUEST,
        |pid, payload| {
            let req = PhantomBattleMasterLevelRewardRequest::decode(payload).unwrap_or_default();
            let Some(&(exp_need, normal_drop, unlock_drop)) = extra_tables()
                .pb_master_levels
                .get(&req.PhantomBattleMasterLevelId)
            else {
                return ok_resp(
                    msg_id::PHANTOM_BATTLE_MASTER_LEVEL_REWARD_RESPONSE,
                    &PhantomBattleMasterLevelRewardResponse {
                        ErrorCode: Some(ErrorCode::RequestParamError),
                    },
                );
            };
            let (zints, _) = sqlite_store::get_progress_state(pid);
            let pb_exp = zints.get("pb:exp").copied().unwrap_or(0);
            if exp_need > 0 && pb_exp < i64::from(exp_need) {
                return ok_resp(
                    msg_id::PHANTOM_BATTLE_MASTER_LEVEL_REWARD_RESPONSE,
                    &PhantomBattleMasterLevelRewardResponse {
                        ErrorCode: Some(ErrorCode::RequestParamError),
                    },
                );
            }
            let drop_id = if normal_drop > 0 { normal_drop } else { unlock_drop };
            let grants = resolve_drop_or_fallback(drop_id, &[(3, 40), (2, 10_000)]);
            grant_validated_claim(
                pid,
                &format!("pb_ml:{}", req.PhantomBattleMasterLevelId),
                &[format!("pb_master:{}", req.PhantomBattleMasterLevelId)],
                msg_id::PHANTOM_BATTLE_MASTER_LEVEL_REWARD_RESPONSE,
                &PhantomBattleMasterLevelRewardResponse {
                    ErrorCode: Some(ErrorCode::Success),
                },
                grants,
            )
        },
    );
    map.insert(
        msg_id::ROGUELIKE_WRITE_CAT_REWARD_REQUEST,
        |pid, payload| {
            let req = RoguelikeWriteCatRewardRequest::decode(payload).unwrap_or_default();
            let Some(&(need_energy, drop_id)) = extra_tables().rogue_cat_rewards.get(&req.Index)
            else {
                return ok_resp(
                    msg_id::ROGUELIKE_WRITE_CAT_REWARD_RESPONSE,
                    &RoguelikeWriteCatRewardResponse {
                        ErrorCode: Some(ErrorCode::RequestParamError),
                    },
                );
            };
            let (zints, _) = sqlite_store::get_progress_state(pid);
            let energy = zints
                .get(&format!("rogue_cat:rec:{}", req.ActivityId))
                .copied()
                .max(zints.get("rogue_cat:energy").copied())
                .unwrap_or(0);
            if need_energy > 0 && energy < i64::from(need_energy) {
                return ok_resp(
                    msg_id::ROGUELIKE_WRITE_CAT_REWARD_RESPONSE,
                    &RoguelikeWriteCatRewardResponse {
                        ErrorCode: Some(ErrorCode::RequestParamError),
                    },
                );
            }
            let grants = resolve_drop_or_fallback(drop_id, &[(3, 40), (2, 10_000)]);
            grant_validated_claim(
                pid,
                &format!("rogue_cat:rw:{}:{}", req.ActivityId, req.Index),
                &[format!("rogue_cat:{}:{}", req.ActivityId, req.Index)],
                msg_id::ROGUELIKE_WRITE_CAT_REWARD_RESPONSE,
                &RoguelikeWriteCatRewardResponse {
                    ErrorCode: Some(ErrorCode::Success),
                },
                grants,
            )
        },
    );
    map.insert(msg_id::ROGUELIKE_SEASON_DATA_REQUEST, |pid, payload| {
        let req = RoguelikeSeasonDataRequest::decode(payload).unwrap_or_default();
        let tables = extra_tables();
        let season_id = if req.SeasonId > 0 && tables.rogue_seasons.contains(&req.SeasonId) {
            req.SeasonId
        } else {
            tables.rogue_seasons.last().copied().unwrap_or(1)
        };
        let (zints, zclaims) = sqlite_store::get_progress_state(pid);
        let rewards = tables
            .rogue_season_rewards
            .get(&season_id)
            .map(|ids| {
                ids.iter()
                    .map(|&rid| SeasonReward {
                        Id: rid,
                        IsReceive: zclaims.contains(&format!("rogue:season_rw:{season_id}:{rid}")),
                    })
                    .collect()
            })
            .unwrap_or_default();
        let tokens = zints.get("rogue:season_pt").copied().unwrap_or(0).max(0) as i32;
        ok_resp(
            msg_id::ROGUELIKE_SEASON_DATA_RESPONSE,
            &RoguelikeSeasonDataResponse {
                ErrorCode: Some(ErrorCode::Success),
                SeasonData: Some(SeasonData {
                    SeasonId: season_id,
                    StartTime: 1,
                    EndTime: 2_000_000_000,
                    RoguelikeTokenList: Vec::new(),
                    SeasonRewardList: rewards,
                    TokenItemCount: tokens,
                }),
            },
        )
    });
    map.insert(msg_id::ROGUELIKE_BOSS_INST_START_REQUEST, |_, _| {
        ok_resp(
            msg_id::ROGUELIKE_BOSS_INST_START_RESPONSE,
            &RoguelikeBossInstStartResponse {
                ErrorCode: Some(ErrorCode::Success),
            },
        )
    });
    map.insert(msg_id::ROGUE_TOWER_TRIAL_RE_CHALLENGE_REQUEST, |_, _| {
        ok_resp(
            msg_id::ROGUE_TOWER_TRIAL_RE_CHALLENGE_RESPONSE,
            &RogueTowerTrialReChallengeResponse {
                ErrorCode: Some(ErrorCode::Success),
            },
        )
    });
    map.insert(msg_id::ROVER_ROGUE_TALENT_INFO_REQUEST, |pid, _| {
        let (zints, zclaims) = sqlite_store::get_progress_state(pid);
        let point = zints.get("rover_rogue:pt").copied().unwrap_or(0).max(0) as i32;
        let talent_ids: Vec<i32> = extra_tables()
            .rover_rogue_talents
            .iter()
            .filter_map(|&(tid, _)| {
                zclaims
                    .contains(&format!("rover_rogue:talent:{tid}"))
                    .then_some(tid)
            })
            .collect();
        ok_resp(
            msg_id::ROVER_ROGUE_TALENT_INFO_RESPONSE,
            &RoverRogueTalentInfoResponse {
                ErrorCode: Some(ErrorCode::Success),
                Point: point,
                TalentId: talent_ids,
            },
        )
    });
    map.insert(msg_id::TRAP_DEFENSE_QUICK_EQUIP_REQUEST, |_, _| {
        ok_resp(
            msg_id::TRAP_DEFENSE_QUICK_EQUIP_RESPONSE,
            &TrapDefenseQuickEquipResponse {
                ErrorCode: Some(ErrorCode::Success),
            },
        )
    });
    map.insert(msg_id::MIX_DRINK_TOPIC_SELECT_REQUEST, |_, _| {
        ok_resp(
            msg_id::MIX_DRINK_TOPIC_SELECT_RESPONSE,
            &MixDrinkTopicSelectResponse {
                ErrorCode: Some(ErrorCode::Success),
            },
        )
    });
    map.insert(msg_id::FARM_GOLD_LEVEL_PLAY_REQUEST, |_, _| {
        ok_resp(
            msg_id::FARM_GOLD_LEVEL_PLAY_RESPONSE,
            &FarmGoldLevelPlayResponse {
                ErrorCode: Some(ErrorCode::Success),
            },
        )
    });
    map.insert(msg_id::FARM_GOLD_POINT_REQUEST, |_, _| {
        ok_resp(
            msg_id::FARM_GOLD_POINT_RESPONSE,
            &FarmGoldPointResponse {
                ErrorCode: Some(ErrorCode::Success),
            },
        )
    });
    map.insert(msg_id::FLORO_RANCH_WEEKLY_REQUEST, |pid, _| {
        let (_, zclaims) = sqlite_store::get_progress_state(pid);
        let tables = extra_tables();
        let unlock_cards = tables
            .floro_cards
            .iter()
            .map(|&cid| FloroUnLockInfo {
                Key: cid,
                UnLockCondition: 0,
                UnLock: zclaims.contains(&format!("floro:card:{cid}")),
            })
            .collect();
        let unlock_tech = tables
            .floro_techs
            .iter()
            .copied()
            .filter(|tid| zclaims.contains(&format!("floro:tech:{tid}")))
            .collect();
        let reward_milestones = zclaims
            .iter()
            .filter_map(|c| c.strip_prefix("floro:milestone:")?.parse::<i32>().ok())
            .collect();
        let race_unlock = tables
            .floro_races
            .iter()
            .map(|&rid| FloroUnLockInfo {
                Key: rid,
                UnLockCondition: 0,
                UnLock: zclaims.contains(&format!("floro:race:{rid}")) || rid == 1,
            })
            .collect();
        ok_resp(
            msg_id::FLORO_RANCH_WEEKLY_RESPONSE,
            &FloroRanchWeeklyResponse {
                ErrorCode: Some(ErrorCode::Success),
                PlayerInfo: Some(FloroRanchPlayerInfo {
                    UnlockCards: unlock_cards,
                    UnlockTech: unlock_tech,
                    RewardMilestones: reward_milestones,
                    RaceUnLockInfo: race_unlock,
                    Animation: zclaims.contains("floro:anim"),
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
    });
    map.insert(msg_id::FLORO_RANCH_PLAY_EVENT_CHOICE_REQUEST, |pid, payload| {
        let req = FloroRanchPlayEventChoiceRequest::decode(payload).unwrap_or_default();
        if pid > 0 && req.EventId > 0 {
            sqlite_store::set_kv(
                pid,
                "floro",
                &format!("choice:{}:{}", req.ActivityId, req.EventId),
                &req.ChoiceId.to_string(),
            );
        }
        ok_resp(
            msg_id::FLORO_RANCH_PLAY_EVENT_CHOICE_RESPONSE,
            &FloroRanchPlayEventChoiceResponse {
                ErrorCode: Some(ErrorCode::Success),
                Update: Some(FloroRanchPlayInfo {
                    SubInsId: req.SubInsId,
                    ..Default::default()
                }),
            },
        )
    });
    map.insert(msg_id::FLORO_RANCH_PLAY_REFRESH_GACHA_REQUEST, |pid, payload| {
        let req = FloroRanchPlayRefreshGachaRequest::decode(payload).unwrap_or_default();
        let refresh_cnt = if pid > 0 {
            let prev = sqlite_store::get_kv(pid, "floro", "gacha_refresh")
                .and_then(|s| s.parse::<i32>().ok())
                .unwrap_or(0);
            let next = prev + 1;
            sqlite_store::set_kv(pid, "floro", "gacha_refresh", &next.to_string());
            next
        } else {
            1
        };
        let pool = &extra_tables().floro_cards;
        let card_ids = if pool.is_empty() {
            Vec::new()
        } else {
            (0..3)
                .map(|i| pool[((refresh_cnt as usize) + i) % pool.len()])
                .collect()
        };
        ok_resp(
            msg_id::FLORO_RANCH_PLAY_REFRESH_GACHA_RESPONSE,
            &FloroRanchPlayRefreshGachaResponse {
                ErrorCode: Some(ErrorCode::Success),
                Gacha: Some(FloroRanchGacha {
                    CardIds: card_ids,
                    AllowedRefresh: true,
                    IncId: req.IncId,
                    ..Default::default()
                }),
            },
        )
    });
    map.insert(msg_id::RACING_BET_MATCH_ROUND_REQUEST, |_, _| {
        ok_resp(
            msg_id::RACING_BET_MATCH_ROUND_RESPONSE,
            &RacingBetMatchRoundResponse {
                ErrorCode: Some(ErrorCode::Success),
                Round: Some(RacingBetsMatchRound::default()),
                ..Default::default()
            },
        )
    });
    map.insert(msg_id::RACING_BETS_PLAYER_INFO_REQUEST, |pid, _| {
        let (zints, zclaims) = sqlite_store::get_progress_state(pid);
        let cur_cash = if zclaims.contains("rb:cash_init") {
            zints.get("rb:cash").copied().unwrap_or(0).max(0) as i32
        } else {
            zints.get("rb:cash").copied().unwrap_or(1000).max(0) as i32
        };
        let total_cash = zints
            .get("rb:total_cash")
            .copied()
            .unwrap_or(i64::from(cur_cash))
            .max(0) as i32;
        let hit_num = zints.get("rb:hit").copied().unwrap_or(0).max(0) as i32;
        ok_resp(
            msg_id::RACING_BETS_PLAYER_INFO_RESPONSE,
            &RacingBetsPlayerInfoResponse {
                ErrorCode: Some(ErrorCode::Success),
                PlayerInfo: Some(RacingBetPlayerInfo {
                    CurCash: cur_cash,
                    TotalCash: total_cash,
                    GearInfo: Vec::new(),
                    HitNum: hit_num,
                }),
            },
        )
    });
    map.insert(msg_id::RACING_BETS_TASK_REQUEST, |_, _| {
        ok_resp(
            msg_id::RACING_BETS_TASK_RESPONSE,
            &RacingBetsTaskResponse {
                ErrorCode: Some(ErrorCode::Success),
            },
        )
    });
    map.insert(msg_id::RACING_BETS_LEG_MATCH_INFO_REQUEST, |_, payload| {
        let req = RacingBetsLegMatchInfoRequest::decode(payload).unwrap_or_default();
        let leg_id = if req.LegMatchId > 0
            && extra_tables().racing_leg_matches.contains_key(&req.LegMatchId)
        {
            req.LegMatchId
        } else {
            1
        };
        let dango_info = extra_tables()
            .racing_leg_matches
            .get(&leg_id)
            .map(|(_, dangos)| {
                dangos
                    .iter()
                    .map(|&did| RacingBetsDangoInfo {
                        Id: did,
                        CurPoint: 0,
                        Odds: 200,
                    })
                    .collect()
            })
            .unwrap_or_default();
        ok_resp(
            msg_id::RACING_BETS_LEG_MATCH_INFO_RESPONSE,
            &RacingBetsLegMatchInfoResponse {
                ErrorCode: Some(ErrorCode::Success),
                LegInfo: Some(RacingBetsLegMatchesInfo {
                    Id: leg_id,
                    DangoInfo: dango_info,
                    OddsVersion: b"1".to_vec(),
                    ..Default::default()
                }),
            },
        )
    });
    map.insert(msg_id::RACING_BET_MATCH_REQUEST, |_, payload| {
        let req = RacingBetMatchRequest::decode(payload).unwrap_or_default();
        let dango_info = extra_tables()
            .racing_leg_matches
            .get(&req.LegMatchId)
            .map(|(_, dangos)| {
                dangos
                    .iter()
                    .enumerate()
                    .map(|(idx, &did)| RacingBetsInstanceDangoInfo {
                        DangoId: did,
                        CurPoint: 0,
                        DiceId: 1001 + (idx as i32),
                        High: 0,
                        EntityId: 0,
                        BpId: 0,
                    })
                    .collect()
            })
            .unwrap_or_default();
        ok_resp(
            msg_id::RACING_BET_MATCH_RESPONSE,
            &RacingBetMatchResponse {
                ErrorCode: Some(ErrorCode::Success),
                ActivityId: req.ActivityId,
                LegMatchId: req.LegMatchId,
                DangoInfo: dango_info,
                Rounds: Vec::new(),
            },
        )
    });
    map.insert(msg_id::PHANTOM_BATTLE_DEBUG_INFO_REQUEST, |_, _| {
        ok_resp(
            msg_id::PHANTOM_BATTLE_DEBUG_INFO_RESPONSE,
            &PhantomBattleDebugInfoResponse {
                ErrorCode: Some(ErrorCode::Success),
                ..Default::default()
            },
        )
    });
    map.insert(msg_id::PHANTOM_BATTLE_AREA_SKILL_REQUEST, |_, _| {
        let skill_id = extra_tables().pb_skills.iter().copied().next().unwrap_or(0);
        ok_resp(
            msg_id::PHANTOM_BATTLE_AREA_SKILL_RESPONSE,
            &PhantomBattleAreaSkillResponse {
                ErrorCode: Some(ErrorCode::Success),
                PhantomBattleSkillTriggerInfos: Some(PhantomBattleSkillTriggerInfo {
                    SkillId: skill_id,
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
    });
    map.insert(
        msg_id::PHANTOM_BATTLE_PHANTOM_CARD_SKILL_REQUEST,
        |_, payload| {
            let req = PhantomBattlePhantomCardSkillRequest::decode(payload).unwrap_or_default();
            let valid_card = extra_tables().pb_cards.contains(&req.CardUid);
            ok_resp(
                msg_id::PHANTOM_BATTLE_PHANTOM_CARD_SKILL_RESPONSE,
                &PhantomBattlePhantomCardSkillResponse {
                    ErrorCode: Some(ErrorCode::Success),
                    CardUId: req.CardUid,
                    EffectType: i32::from(valid_card),
                    Num: i32::from(req.CardUid > 0),
                    ..Default::default()
                },
            )
        },
    );

    // Network, Platform, Debug, System
    map.insert(msg_id::CHECK_PUBLIC_RESOURCE_VERSION_REQUEST, |_, _| {
        ok_resp(
            msg_id::CHECK_PUBLIC_RESOURCE_VERSION_RESPONSE,
            &CheckPublicResourceVersionResponse {
                ErrorId: Some(ErrorCode::Success),
                ..Default::default()
            },
        )
    });
    map.insert(msg_id::PLAYER_CREATE_XBOX_SESSION_REQUEST, |_, _| {
        ok_resp(
            msg_id::PLAYER_CREATE_XBOX_SESSION_RESPONSE,
            &PlayerCreateXboxSessionResponse::default(),
        )
    });
    map.insert(msg_id::GATEWAY_ECHO_REQUEST, |_, payload| {
        let req = GatewayEchoRequest::decode(payload).unwrap_or_default();
        ok_resp(
            msg_id::GATEWAY_ECHO_RESPONSE,
            &GatewayEchoResponse {
                Content: req.Content,
            },
        )
    });
    map.insert(msg_id::NET_TEST_DATA_REQUEST, |_, _| {
        ok_resp(
            msg_id::NET_TEST_DATA_RESPONSE,
            &NetTestDataResponse::default(),
        )
    });
    map.insert(msg_id::PRESS_REQUEST, |_, _| {
        ok_resp(msg_id::PRESS_RESPONSE, &PressResponse::default())
    });
    map.insert(msg_id::PRESS_GAME_REQUEST, |_, _| {
        ok_resp(msg_id::PRESS_GAME_RESPONSE, &PressGameResponse::default())
    });
    map.insert(msg_id::ENABLE_DAMAGE_RECORD_REQUEST, |_, _| {
        AkiResult::ok_raw(msg_id::ENABLE_DAMAGE_RECORD_RESPONSE, Vec::new())
    });
    map.insert(msg_id::COMBAT_MAX_CASE_MESSAGE_REQUEST, |_, _| {
        AkiResult::ok_raw(msg_id::COMBAT_MAX_CASE_MESSAGE_RESPONSE, Vec::new())
    });
    map.insert(msg_id::CHEAT_INPUT_REQUEST, |_, _| {
        AkiResult::ok_raw(msg_id::CHEAT_INPUT_RESPONSE, Vec::new())
    });
    map.insert(msg_id::RUN_TEST_REQUEST, |_, _| {
        ok_resp(
            msg_id::RUN_TEST_RESPONSE,
            &RunTestResponse {
                Code: Some(ErrorCode::Success),
            },
        )
    });
    map.insert(msg_id::GM_LEVEL_ACTION_REQUEST, |_, _| {
        ok_resp(
            msg_id::GM_LEVEL_ACTION_RESPONSE,
            &GmLevelActionResponse {
                Code: Some(ErrorCode::Success),
            },
        )
    });
    map.insert(msg_id::KCP_CONV_REQUEST, |_, payload| {
        let req = KcpConvRequest::decode(payload).unwrap_or_default();
        ok_resp(
            msg_id::KCP_CONV_RESPONSE,
            &KcpConvResponse {
                Code: Some(ErrorCode::Success),
                Conv: 1,
                CrcCheck: if req.DisableCrc { 0 } else { 1 },
                KcpMtu: 1400,
            },
        )
    });
}
