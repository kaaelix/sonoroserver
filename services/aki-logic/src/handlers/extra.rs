//! Explicit gameplay, combat, minigame, and system handlers matching `ExtraHandlers.cs`.

use std::collections::HashMap;

use sonoro_proto::PbMessage;
use sonoro_proto::enums::*;
use sonoro_proto::pb::*;
use sonoro_proto::registry::msg_id;

use crate::handlers::HandlerFn;
use crate::ipc::AkiResult;
use crate::sqlite_store;

fn ok_resp<M: PbMessage>(resp_id: u16, resp: &M) -> AkiResult {
    AkiResult::ok(resp_id, resp)
}

fn grant_claim_reward<M: PbMessage>(
    player_id: i32,
    claim_key: &str,
    resp_id: u16,
    resp: &M,
    grants: &[(i32, i32)],
) -> AkiResult {
    let mut result = AkiResult::ok(resp_id, resp);
    if player_id > 0 {
        let claims = sqlite_store::get_activity_claims(player_id);
        if !claims.contains(claim_key) {
            sqlite_store::set_activity_claim(player_id, claim_key, "1");
            result = result.with_grants(grants.iter().copied());
        }
    }
    result
}

pub fn register_all(map: &mut HashMap<u16, HandlerFn>) {
    // Formation & Role / Phantom Recommend
    map.insert(msg_id::FORMATION_REQUEST, |_, _| {
        ok_resp(
            msg_id::FORMATION_RESPONSE,
            &FormationResponse {
                ErrorCode: Some(ErrorCode::Success),
                ..Default::default()
            },
        )
    });
    map.insert(msg_id::FORMATION_MODIFY_REQUEST, |pid, payload| {
        let req = FormationModifyRequest::decode(payload).unwrap_or_default();
        if pid > 0 {
            sqlite_store::set_kv(pid, "formation", "modify_id", &req.Id.to_string());
        }
        ok_resp(
            msg_id::FORMATION_MODIFY_RESPONSE,
            &FormationModifyResponse {
                ErrorCode: Some(ErrorCode::Success),
                Formation: Some(Formation {
                    Id: req.Id,
                    ..Default::default()
                }),
            },
        )
    });
    map.insert(msg_id::FORMATION_CHANGE_REQUEST, |pid, payload| {
        let req = FormationChangeRequest::decode(payload).unwrap_or_default();
        if pid > 0 {
            sqlite_store::set_kv(pid, "formation", "current_id", &req.FormationId.to_string());
        }
        ok_resp(
            msg_id::FORMATION_CHANGE_RESPONSE,
            &FormationChangeResponse {
                ErrorCode: Some(ErrorCode::Success),
                FormationId: req.FormationId,
                ..Default::default()
            },
        )
    });
    map.insert(msg_id::FORMATION_SWITCH_CURRENT_REQUEST, |pid, payload| {
        let req = FormationSwitchCurrentRequest::decode(payload).unwrap_or_default();
        if pid > 0 {
            sqlite_store::set_kv(pid, "formation", "current_id", &req.Id.to_string());
        }
        ok_resp(
            msg_id::FORMATION_SWITCH_CURRENT_RESPONSE,
            &FormationSwitchCurrentResponse {
                ErrorCode: Some(ErrorCode::Success),
                CurrentId: req.Id,
            },
        )
    });
    map.insert(msg_id::FORMATION_SWITCH_ROLE_REQUEST, |_, payload| {
        let req = FormationSwitchRoleRequest::decode(payload).unwrap_or_default();
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
        ok_resp(
            msg_id::PHANTOM_RECOMMEND_RESPONSE,
            &PhantomRecommendResponse {
                ErrorCode: Some(ErrorCode::Success),
                RoleId: req.RoleId,
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
    map.insert(msg_id::SPECIAL_ITEM_EQUIP_REQUEST, |pid, _| {
        if pid > 0 {
            sqlite_store::set_kv(pid, "special_item", "equipped", "1");
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
    map.insert(msg_id::SET_PLAYER_YU_XUE_MODE_REQUEST, |pid, _| {
        if pid > 0 {
            sqlite_store::set_kv(pid, "yuxue", "player_mode", "1");
        }
        ok_resp(
            msg_id::SET_PLAYER_YU_XUE_MODE_RESPONSE,
            &SetPlayerYuXueModeResponse {
                ErrorCode: Some(ErrorCode::Success),
            },
        )
    });
    map.insert(msg_id::SET_BT_OBJ_YU_XUE_MODE_REQUEST, |_, _| {
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
        let active =
            pid > 0 && sqlite_store::get_kv(pid, "yuxue", "player_mode").as_deref() == Some("1");
        ok_resp(
            msg_id::GET_TRACE_QUEST_YU_XUE_MODE_RESPONSE,
            &GetTraceQuestYuXueModeResponse {
                ErrorCode: Some(ErrorCode::Success),
                InYuXueMode: active,
            },
        )
    });

    // Activity & Minigame Handlers + SQLite Reward Claims
    map.insert(msg_id::MAP_TRAVEL_SOAR_REWARD_REQUEST, |pid, payload| {
        let req = MapTravelSoarRewardRequest::decode(payload).unwrap_or_default();
        grant_claim_reward(
            pid,
            &format!("map_travel_soar:{}", req.TaskId),
            msg_id::MAP_TRAVEL_SOAR_REWARD_RESPONSE,
            &MapTravelSoarRewardResponse {
                ErrorCode: Some(ErrorCode::Success),
            },
            &[(3, 40), (2, 10_000)],
        )
    });
    map.insert(msg_id::MAP_TRAVEL_TASK_REWARD_REQUEST, |pid, payload| {
        let req = MapTravelTaskRewardRequest::decode(payload).unwrap_or_default();
        grant_claim_reward(
            pid,
            &format!("map_travel_task:{}", req.TaskId),
            msg_id::MAP_TRAVEL_TASK_REWARD_RESPONSE,
            &MapTravelTaskRewardResponse {
                ErrorCode: Some(ErrorCode::Success),
            },
            &[(3, 40), (2, 10_000)],
        )
    });
    map.insert(msg_id::ROGUE_WEEKLY_REWARD_REQUEST, |pid, payload| {
        let req = RogueWeeklyRewardRequest::decode(payload).unwrap_or_default();
        grant_claim_reward(
            pid,
            &format!("rogue_weekly_rew:{}:{}", req.ActivityId, req.ConfigId),
            msg_id::ROGUE_WEEKLY_REWARD_RESPONSE,
            &RogueWeeklyRewardResponse {
                ErrorCode: Some(ErrorCode::Success),
            },
            &[(3, 80), (2, 20_000)],
        )
    });
    map.insert(
        msg_id::RISK_HARVEST_BUFF_COUNT_REWARD_REQUEST,
        |pid, payload| {
            let req = RiskHarvestBuffCountRewardRequest::decode(payload).unwrap_or_default();
            grant_claim_reward(
                pid,
                &format!("risk_buff_cnt:{}", req.Id),
                msg_id::RISK_HARVEST_BUFF_COUNT_REWARD_RESPONSE,
                &RiskHarvestBuffCountRewardResponse {
                    ErrorCode: Some(ErrorCode::Success),
                },
                &[(3, 40), (2, 10_000)],
            )
        },
    );
    map.insert(
        msg_id::RISK_HARVEST_BUFF_UNLOCK_REWARD_REQUEST,
        |pid, payload| {
            let req = RiskHarvestBuffUnlockRewardRequest::decode(payload).unwrap_or_default();
            grant_claim_reward(
                pid,
                &format!("risk_buff_unlock:{}", req.Id),
                msg_id::RISK_HARVEST_BUFF_UNLOCK_REWARD_RESPONSE,
                &RiskHarvestBuffUnlockRewardResponse {
                    ErrorCode: Some(ErrorCode::Success),
                },
                &[(3, 40), (2, 10_000)],
            )
        },
    );
    map.insert(msg_id::CORNICE_MEETING_REWARD_REQUEST, |pid, payload| {
        let req = CorniceMeetingRewardRequest::decode(payload).unwrap_or_default();
        grant_claim_reward(
            pid,
            &format!("cornice:{}:{}", req.LevelPlayId, req.ScoreIndex),
            msg_id::CORNICE_MEETING_REWARD_RESPONSE,
            &CorniceMeetingRewardResponse {
                ErrorCode: Some(ErrorCode::Success),
            },
            &[(3, 40), (2, 10_000)],
        )
    });
    map.insert(
        msg_id::CIRCUM_FLUENCE_TASK_REWARD_REQUEST,
        |pid, payload| {
            let req = CircumFluenceTaskRewardRequest::decode(payload).unwrap_or_default();
            let rtype = req.RewardType.map(|r| r as i32).unwrap_or(0);
            grant_claim_reward(
                pid,
                &format!("circum:{}:{rtype}", req.ActivityId),
                msg_id::CIRCUM_FLUENCE_TASK_REWARD_RESPONSE,
                &CircumFluenceTaskRewardResponse {
                    ErrorCode: Some(ErrorCode::Success),
                },
                &[(3, 40), (2, 10_000)],
            )
        },
    );
    map.insert(
        msg_id::PHANTOM_BATTLE_MASTER_LEVEL_REWARD_REQUEST,
        |pid, payload| {
            let req = PhantomBattleMasterLevelRewardRequest::decode(payload).unwrap_or_default();
            grant_claim_reward(
                pid,
                &format!("pb_master:{}", req.PhantomBattleMasterLevelId),
                msg_id::PHANTOM_BATTLE_MASTER_LEVEL_REWARD_RESPONSE,
                &PhantomBattleMasterLevelRewardResponse {
                    ErrorCode: Some(ErrorCode::Success),
                },
                &[(3, 40), (2, 10_000)],
            )
        },
    );
    map.insert(
        msg_id::ROGUELIKE_WRITE_CAT_REWARD_REQUEST,
        |pid, payload| {
            let req = RoguelikeWriteCatRewardRequest::decode(payload).unwrap_or_default();
            grant_claim_reward(
                pid,
                &format!("rogue_cat:{}:{}", req.ActivityId, req.Index),
                msg_id::ROGUELIKE_WRITE_CAT_REWARD_RESPONSE,
                &RoguelikeWriteCatRewardResponse {
                    ErrorCode: Some(ErrorCode::Success),
                },
                &[(3, 40), (2, 10_000)],
            )
        },
    );
    map.insert(msg_id::ROGUELIKE_SEASON_DATA_REQUEST, |_, _| {
        ok_resp(
            msg_id::ROGUELIKE_SEASON_DATA_RESPONSE,
            &RoguelikeSeasonDataResponse {
                ErrorCode: Some(ErrorCode::Success),
                SeasonData: Some(SeasonData::default()),
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
    map.insert(msg_id::ROVER_ROGUE_TALENT_INFO_REQUEST, |_, _| {
        ok_resp(
            msg_id::ROVER_ROGUE_TALENT_INFO_RESPONSE,
            &RoverRogueTalentInfoResponse {
                ErrorCode: Some(ErrorCode::Success),
                ..Default::default()
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
    map.insert(msg_id::FLORO_RANCH_WEEKLY_REQUEST, |_, _| {
        ok_resp(
            msg_id::FLORO_RANCH_WEEKLY_RESPONSE,
            &FloroRanchWeeklyResponse {
                ErrorCode: Some(ErrorCode::Success),
                PlayerInfo: Some(FloroRanchPlayerInfo::default()),
                ..Default::default()
            },
        )
    });
    map.insert(msg_id::FLORO_RANCH_PLAY_EVENT_CHOICE_REQUEST, |_, _| {
        ok_resp(
            msg_id::FLORO_RANCH_PLAY_EVENT_CHOICE_RESPONSE,
            &FloroRanchPlayEventChoiceResponse {
                ErrorCode: Some(ErrorCode::Success),
                Update: Some(FloroRanchPlayInfo::default()),
            },
        )
    });
    map.insert(msg_id::FLORO_RANCH_PLAY_REFRESH_GACHA_REQUEST, |_, _| {
        ok_resp(
            msg_id::FLORO_RANCH_PLAY_REFRESH_GACHA_RESPONSE,
            &FloroRanchPlayRefreshGachaResponse {
                ErrorCode: Some(ErrorCode::Success),
                Gacha: Some(FloroRanchGacha::default()),
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
    map.insert(msg_id::RACING_BETS_PLAYER_INFO_REQUEST, |_, _| {
        ok_resp(
            msg_id::RACING_BETS_PLAYER_INFO_RESPONSE,
            &RacingBetsPlayerInfoResponse {
                ErrorCode: Some(ErrorCode::Success),
                PlayerInfo: Some(RacingBetPlayerInfo::default()),
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
    map.insert(msg_id::RACING_BETS_LEG_MATCH_INFO_REQUEST, |_, _| {
        ok_resp(
            msg_id::RACING_BETS_LEG_MATCH_INFO_RESPONSE,
            &RacingBetsLegMatchInfoResponse {
                ErrorCode: Some(ErrorCode::Success),
                LegInfo: Some(RacingBetsLegMatchesInfo::default()),
            },
        )
    });
    map.insert(msg_id::RACING_BET_MATCH_REQUEST, |_, payload| {
        let req = RacingBetMatchRequest::decode(payload).unwrap_or_default();
        ok_resp(
            msg_id::RACING_BET_MATCH_RESPONSE,
            &RacingBetMatchResponse {
                ErrorCode: Some(ErrorCode::Success),
                ActivityId: req.ActivityId,
                LegMatchId: req.LegMatchId,
                ..Default::default()
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
        ok_resp(
            msg_id::PHANTOM_BATTLE_AREA_SKILL_RESPONSE,
            &PhantomBattleAreaSkillResponse {
                ErrorCode: Some(ErrorCode::Success),
                PhantomBattleSkillTriggerInfos: Some(PhantomBattleSkillTriggerInfo::default()),
                ..Default::default()
            },
        )
    });
    map.insert(
        msg_id::PHANTOM_BATTLE_PHANTOM_CARD_SKILL_REQUEST,
        |_, payload| {
            let req = PhantomBattlePhantomCardSkillRequest::decode(payload).unwrap_or_default();
            ok_resp(
                msg_id::PHANTOM_BATTLE_PHANTOM_CARD_SKILL_RESPONSE,
                &PhantomBattlePhantomCardSkillResponse {
                    ErrorCode: Some(ErrorCode::Success),
                    CardUId: req.CardUid,
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
