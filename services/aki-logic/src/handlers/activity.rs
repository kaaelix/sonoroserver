//! Activity list & update handlers matching `ActivityHandler.cs`.

use std::collections::{HashMap, HashSet};

use sonoro_proto::PbEnum;
use sonoro_proto::enums::*;
use sonoro_proto::pb::*;
use sonoro_proto::registry::msg_id;

use crate::bin_data::{self, ActivityConfigItem};
use crate::ipc::AkiResult;
use crate::sqlite_store;

const FAR_FUTURE: i64 = 4_070_908_800;

const EXCLUDED_TYPES: &[i32] = &[
    13, 18, 22, 30, 41, 42, 45, 50, 55, 57, 58, 61, 62, 69, 72, 76, 80, 82, 83, 91, 97, 103, 104,
    107, 108, 111, 113, 117,
];

fn has_missing_build_asset(def: &ActivityConfigItem) -> bool {
    def.tab_texture.iter().any(|tex| {
        tex.contains("SignIn32")
            || tex.contains("SignIn33")
            || tex.contains("SignIn34")
            || tex.contains("Activity32/")
            || tex.contains("Activity33/")
            || tex.contains("Activity34/")
    })
}

fn rogue_season_id_for_activity(activity_id: i32) -> i32 {
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

fn populate_int_claim_suffixes(claims: &HashSet<String>, prefix: &str, target: &mut Vec<i32>) {
    for c in claims {
        if let Some(rest) = c.strip_prefix(prefix)
            && let Ok(val) = rest.parse::<i32>()
            && val > 0
            && !target.contains(&val)
        {
            target.push(val);
        }
    }
}

fn populate_sub_payload(
    act: &mut ActivityData,
    act_type: ActivityType,
    id: i32,
    claims: &HashSet<String>,
    ints: &HashMap<String, i64>,
) {
    use ActivityDataDataOneof as D;
    let payload: Option<D> = match act_type {
        ActivityType::Parkour => Some(D::ParkourActivity(Default::default())),
        ActivityType::GatherActivity => Some(D::GatherActivityInfo(Default::default())),
        ActivityType::Sign => {
            let mut sign = SignActivity::default();
            for d in 0..7 {
                let state = if claims.contains(&format!("sign_act:{id}:{d}")) {
                    SignState::IsReceive
                } else {
                    SignState::Unlock
                };
                sign.SignStateList.push(state);
            }
            Some(D::SignActivity(sign))
        }
        ActivityType::NewBieCourse => {
            let mut nbc = NewBieCourseActivity::default();
            populate_int_claim_suffixes(
                claims,
                "region_prog:NewBieCourseRewardRequest:",
                &mut nbc.HadTakeReward,
            );
            populate_int_claim_suffixes(claims, "newbie:course:", &mut nbc.HadTakeReward);
            Some(D::NewBieCourseActivity(nbc))
        }
        ActivityType::RougeActivity => Some(D::ActivityRogueData(ActivityRogueData {
            BeginOpenTime: 1,
            EndOpenTime: FAR_FUTURE,
            RoguelikeSeason: Some(RoguelikeSeason {
                SeasonId: rogue_season_id_for_activity(id),
                EndTime: FAR_FUTURE,
                ..Default::default()
            }),
        })),
        ActivityType::DoubleInstanceRewardActivity => {
            Some(D::DoubleInstActivityReward(Default::default()))
        }
        ActivityType::RoleTrialActivity => Some(D::RoleTrialInfoActivity(Default::default())),
        ActivityType::Harvest => Some(D::HarvestActivity(Default::default())),
        ActivityType::PhantomCollect => Some(D::PhantomCollectActivity(Default::default())),
        ActivityType::DailyAdventureActivity => {
            Some(D::DailyAdventureActivityData(Default::default()))
        }
        ActivityType::LongShanMainActivity => Some(D::ActivityLongShanMain(Default::default())),
        ActivityType::BossRushActivity => Some(D::BossRushActivityData(Default::default())),
        ActivityType::TurnTableActivity => Some(D::ActivityTurnTableData(Default::default())),
        ActivityType::TrackMoonActivity => Some(D::TrackMoonActivityTaskData(Default::default())),
        ActivityType::CircumFluence => Some(D::CircumFluenceTaskData(Default::default())),
        ActivityType::TowerDefenceActivity => Some(D::TowerDefenseActivityInfo(Default::default())),
        ActivityType::TimePointRewardActivity => {
            Some(D::ActivityTimePointRewarData(Default::default()))
        }
        ActivityType::TrackMoonPhase => Some(D::ActivityRoleGiveData(Default::default())),
        ActivityType::RiskHarvest => Some(D::RiskHarvestActivityData(Default::default())),
        ActivityType::CorniceMeeting => Some(D::ActivityCorniceMeetingData(Default::default())),
        ActivityType::BlackCoastTheme => Some(D::ActivityBlackCoastData(Default::default())),
        ActivityType::RogueWhiteCat => Some(D::DreamLinkActivityData(Default::default())),
        ActivityType::ScratchCard => Some(D::ActivityScratchTicketData(Default::default())),
        ActivityType::PreheatSign => Some(D::PreheatSignActivityData(Default::default())),
        ActivityType::MowTower => Some(D::MowTowerActivityData(Default::default())),
        ActivityType::ThroughTrain => Some(D::ThroughTrainActivityData(Default::default())),
        ActivityType::SprintSign => Some(D::SpringSignData(Default::default())),
        ActivityType::MapTravelActivity => Some(D::MapTravelActivityData(Default::default())),
        ActivityType::FarmGold => Some(D::FarmGoldData(Default::default())),
        ActivityType::RoleSkinTrialActivity => Some(D::RoleSkinTrialActivity(Default::default())),
        ActivityType::RogueWeekly => Some(D::ActivityWeeklyRogueData(ActivityWeeklyRogueData {
            Data: Some(ActivityWeeklyRogueDataDataOneof::RogueWeeklyLastInfo(
                Default::default(),
            )),
            CycleId: (*ints.get("rogue_weekly:cycle").unwrap_or(&0)).max(1) as i32,
            MaxFreeCount: 3,
            CurWorldLevel: 1,
            UseFreeCount: *ints.get("rogue_weekly:bf_used").unwrap_or(&0) as i32,
            ..Default::default()
        })),
        ActivityType::FishingActivity => Some(D::ActivityFishingData(Default::default())),
        ActivityType::TeamParkOurActivity => Some(D::SolarisSpeedActivity(SolarisSpeedActivity {
            ActivityTaskDatas: Some(Default::default()),
            ..Default::default()
        })),
        ActivityType::BabelTower => Some(D::BabelTowerActivity(Default::default())),
        ActivityType::Avignon => Some(D::ActivityAvignon(ActivityAvignon {
            RewardData: Some(Default::default()),
            ..Default::default()
        })),
        ActivityType::BetHorses => Some(D::ActivityBetHorsesData(ActivityBetHorsesData {
            ActivityId: id,
            ..Default::default()
        })),
        ActivityType::Explore => Some(D::ActivityMapExploreData(Default::default())),
        ActivityType::Abyss => Some(D::DangoAbyssActivityData(DangoAbyssActivityData {
            StartTime: 1,
            EndTime: FAR_FUTURE * 1000,
            ..Default::default()
        })),
        ActivityType::RogueRes => Some(D::ActivityPermanentRogueData(ActivityPermanentRogueData {
            PermanentSeasonData: vec![PermanentSeasonData {
                PermanentSeasonDataId: 1,
                EndTime: FAR_FUTURE,
                ..Default::default()
            }],
            ..Default::default()
        })),
        ActivityType::H5CircumFluence => Some(D::ActivityInviteNewbie(Default::default())),
        ActivityType::DangoMonopoly => {
            Some(D::ActivityDangoMonopolyData(ActivityDangoMonopolyData {
                CurrentBoardId: 1,
                ..Default::default()
            }))
        }
        ActivityType::CiacconaActivity => {
            Some(D::ActivityCiacconaGalData(ActivityCiacconaGalData {
                CiacconaGalInspirationData: Some(Default::default()),
                RewardStartTime: 0,
                RewardEndTime: FAR_FUTURE,
                ..Default::default()
            }))
        }
        ActivityType::ActivityLinkage => Some(D::ActivityLinkageData(ActivityLinkageData {
            ActivityId: id,
            ..Default::default()
        })),
        ActivityType::RegressActivity => Some(D::ActivityRegressData(ActivityRegressData {
            EndTime: FAR_FUTURE,
            Grade: 1,
            ..Default::default()
        })),
        ActivityType::ConsumptiveActivity => Some(D::CumulativeShopData(CumulativeShopData {
            ActivityId: id,
            TotalScore: *ints.get("cons:score").unwrap_or(&0) as i32,
            ..Default::default()
        })),
        ActivityType::PhantomBattle => {
            Some(D::PhantomArenaActivityData(PhantomArenaActivityData {
                PhantomArenaMasterInfo: Some(Default::default()),
                TimeLimitShopEndTime: FAR_FUTURE,
                ..Default::default()
            }))
        }
        ActivityType::NewbieCarnival => Some(D::BeginnerCarnivalData(BeginnerCarnivalData {
            ActivityTaskData: Some(Default::default()),
            ..Default::default()
        })),
        ActivityType::MoraleActivity => Some(D::ActivityMoraleData(Default::default())),
        ActivityType::FloroRanchActivity => {
            Some(D::FloroRanchActivityData(FloroRanchActivityData {
                FloroRangeData: Some(Default::default()),
                ..Default::default()
            }))
        }
        ActivityType::LifePointChallenge => Some(D::LifePointDrawActivityData(Default::default())),
        ActivityType::TrapDefense => {
            let cache = bin_data::get();
            let build_rewards = |cfgs: &[bin_data::TrapDefenseRewardCfg], claim_prefix: &str| {
                cfgs.iter()
                    .filter(|c| c.activity_id == 0 || c.activity_id == id)
                    .map(|c| {
                        let claimed = claims.contains(&format!("{claim_prefix}{}", c.id));
                        TrapDefenseRewardData {
                            ActivityServerRewardItemData: Some(ConditionTask {
                                Id: c.id,
                                Status: Some(if claimed {
                                    ConditionTaskState::ConditionTaskTaken
                                } else {
                                    ConditionTaskState::ConditionTaskRunning
                                }),
                                ..Default::default()
                            }),
                            StartTime: 0,
                            EndTime: FAR_FUTURE,
                        }
                    })
                    .collect()
            };
            let talent_pts = *ints.get("trap_defense:talent_pts").unwrap_or(&0) as i32;
            let remain_pts = *ints
                .get("trap_defense:remain_pts")
                .unwrap_or(&i64::from(talent_pts)) as i32;
            Some(D::ActivityTrapDefenseData(ActivityTrapDefenseData {
                TrapDefenseTalentTreePoints: talent_pts,
                TrapDefenseRemainPoints: remain_pts,
                StartTime: 0,
                EndTime: FAR_FUTURE,
                SpecialReward: build_rewards(&cache.trap_defense_special_rewards, "trap_def:sclm:"),
                Rewards: build_rewards(&cache.trap_defense_rewards, "trap_def:clm:"),
                ..Default::default()
            }))
        }
        ActivityType::JinzhouFlyActivity => Some(D::ActivitySoarData(Default::default())),
        ActivityType::FunPlay => Some(D::ActivityFunPlayData(Default::default())),
        ActivityType::MoonPhase => Some(D::ActivityMoonSignInData(Default::default())),
        ActivityType::LineCross => Some(D::ActivityLineCrossData(Default::default())),
        ActivityType::HonamiStory => Some(D::HonamiStoryActivityData(HonamiStoryActivityData {
            PlayerBagInfo: Some(Default::default()),
            ..Default::default()
        })),
        ActivityType::Survivors => Some(D::SurvivorsActivityData(SurvivorsActivityData {
            NormalTaskData: Some(Default::default()),
            ..Default::default()
        })),
        ActivityType::PhotoFight => Some(D::FightPhotoActivityData(Default::default())),
        ActivityType::WuWuKuji => Some(D::ActivityPrizeDrawingData(Default::default())),
        ActivityType::InfrTheme => Some(D::InfrThemeActivityPb(InfrThemeActivityPb {
            ActivityTaskData: Some(Default::default()),
        })),
        ActivityType::AdvanceNoticeActivity => Some(D::AdvertisingPageInfo(Default::default())),
        ActivityType::PhantomBattleRecord => Some(D::PhantomBattleRecordActivityInfo(
            PhantomArenaActivityData {
                PhantomArenaMasterInfo: Some(Default::default()),
                TimeLimitShopEndTime: FAR_FUTURE,
                ..Default::default()
            },
        )),
        ActivityType::CoopActivity => Some(D::RoleCoopActivityData(Default::default())),
        ActivityType::PhantomBattleRecordGuide => {
            Some(D::PhantomBattleGuideActivity(Default::default()))
        }
        ActivityType::ArtemisActivity => Some(D::ArtemisActivityInfo(Default::default())),
        ActivityType::MotorcycleIpLink => Some(D::MotorCycleIpActivityData(Default::default())),
        ActivityType::SkinRewardActivity => Some(D::SkinRewardActivityData(Default::default())),
        ActivityType::RoadBookActivity => Some(D::RoadBookActivityInfo(Default::default())),
        ActivityType::NewTowerClimbing => Some(D::NewTowerClimbingActivityData(
            NewTowerClimbingActivityData {
                CycleId: 1,
                CycleBeginTime: 1,
                CycleCloseTime: FAR_FUTURE,
                SeasonId: 1,
                SeasonBeginTime: 1,
                SeasonCloseTime: FAR_FUTURE,
                ..Default::default()
            },
        )),
        ActivityType::MotorFight => Some(D::MotorFightActivityPb(MotorFightActivityPb {
            TalentTree: Some(Default::default()),
            ..Default::default()
        })),
        ActivityType::Encircle => Some(D::EncircleActivityPb(Default::default())),
        ActivityType::NewPlayerSupportActivity => {
            Some(D::NewPlayerSupportActivityData(Default::default()))
        }
        ActivityType::SpringFestivalActivity => {
            Some(D::SpringFestivalActivityInfo(SpringFestivalActivityInfo {
                DrinkMixData: Some(Default::default()),
                ..Default::default()
            }))
        }
        ActivityType::MotorParkourActivity => Some(D::MotorParkourActivityInfo(Default::default())),
        ActivityType::TotalTopUp => Some(D::TotalTopUpActivityInfo(TotalTopUpActivityInfo {
            Score: *ints.get("topup:score").unwrap_or(&0) as i32,
            ..Default::default()
        })),
        ActivityType::H5View => Some(D::H5ViewActivityData(Default::default())),
        ActivityType::MotorDevelop => Some(D::MotorDevelopActivityData(Default::default())),
        ActivityType::FlagChallenge => {
            Some(D::FlagChallengeActivityInfo(FlagChallengeActivityInfo {
                FlagChallengeRoleLevelInfo: Some(Default::default()),
                ..Default::default()
            }))
        }
        ActivityType::Rhythm => Some(D::RhythmActivityPb(RhythmActivityPb {
            RedDot: Some(Default::default()),
            ..Default::default()
        })),
        ActivityType::FeiXue => Some(D::FeiXuePreheatActivityInfo(Default::default())),
        ActivityType::DropCatchActivity => Some(D::DropCatchActivityInfo(Default::default())),
        ActivityType::TetrisActivity => Some(D::TetrisActivityInfo(Default::default())),
        ActivityType::Kurotato => Some(D::KurotatoActivityData(Default::default())),
        ActivityType::PinballActivity => Some(D::PinballActivityData(PinballActivityData {
            Weapons: Some(Default::default()),
            Roles: Some(Default::default()),
            ..Default::default()
        })),
        ActivityType::WuWuWeekSign => Some(D::WuWuWeekActivity(Default::default())),
        ActivityType::AnniversaryTheme => Some(D::ThemeCelebration(Default::default())),
        ActivityType::BossPiling => Some(D::BossPilingActivityInfo(Default::default())),
        ActivityType::GolemCrackActivity => Some(D::GolemCrackActivityInfo(Default::default())),
        ActivityType::EdgeRunnerActivity => Some(D::EdgeRunnerActivityInfo(Default::default())),
        ActivityType::MotorDecalActivity => Some(D::MotorDecalActivityData(Default::default())),
        ActivityType::MotorParkour => Some(D::OnlineMotorActivityData(Default::default())),
        ActivityType::LinkageCheckIn => Some(D::LinkageCheckInActivityData(Default::default())),
        ActivityType::RoleGiftActivity => Some(D::RoleGiftActivityData(Default::default())),
        ActivityType::RealmBetween => Some(D::RealmBetweenActivityInfo(Default::default())),
        ActivityType::ThroughTrainSummary => {
            Some(D::ThroughTrainSummaryActivityData(Default::default()))
        }
        ActivityType::NewPlayerSupportActivityV2 => {
            Some(D::NewPlayerSupportActivityV2Pb(Default::default()))
        }
        ActivityType::NewbieAdventureV2 => Some(D::NewbieAdventureV2Pb(Default::default())),
        ActivityType::NewbieCourseV2 => Some(D::NewbieCourseV2ActivityPb(Default::default())),
        ActivityType::NewbieMain => Some(D::NewbieMainActivityPb(Default::default())),
        ActivityType::RoverRogue => Some(D::RoverRogueActivityData(RoverRogueActivityData {
            activityId: id,
            HistoryInsInfo: Some(Default::default()),
            ..Default::default()
        })),
        ActivityType::DangoBrawl => Some(D::DangoBrawlActivityInfo(DangoBrawlActivityInfo {
            SaveMatchBrief: Some(Default::default()),
            ..Default::default()
        })),
        ActivityType::QingXiaoPlayThePiano => Some(D::QingXiaoActivityInfo(Default::default())),
        ActivityType::MatchThreeActivity => Some(D::MatchThreeActivityInfo(Default::default())),
        ActivityType::SolarSchool => Some(D::SolarSchoolActivityPb(Default::default())),
        ActivityType::GiantSystemActivity => Some(D::GiantActivityInfo(GiantActivityInfo {
            TotalStableValue: 100,
            ..Default::default()
        })),
        ActivityType::RoleTower => Some(D::RoleTowerActivityData(RoleTowerActivityData {
            RankRewardSettleTime: FAR_FUTURE,
            Score: *ints.get("role_tower:score").unwrap_or(&0) as i32,
            ..Default::default()
        })),
        _ => None,
    };
    act.Data = payload;
}

fn build_activities(player_id: i32) -> Vec<ActivityData> {
    let mut claims = if player_id > 0 {
        sqlite_store::get_activity_claims(player_id)
    } else {
        HashSet::new()
    };
    let (prog_ints, prog_claims) = if player_id > 0 {
        sqlite_store::get_progress_state(player_id)
    } else {
        (HashMap::new(), HashSet::new())
    };
    claims.extend(prog_claims);

    let cache = bin_data::get();
    let mut out = Vec::new();
    for def in &cache.activities {
        if EXCLUDED_TYPES.contains(&def.activity_type) {
            continue;
        }
        if has_missing_build_asset(def) {
            continue;
        }
        let act_type = ActivityType::from_i32_or_default(def.activity_type);
        let mut act = ActivityData {
            Id: def.id,
            Type: Some(act_type),
            BeginShowTime: 1,
            EndShowTime: FAR_FUTURE,
            BeginOpenTime: 1,
            EndOpenTime: FAR_FUTURE,
            StartTime: 1,
            EndTime: FAR_FUTURE,
            BeginRewardTimeInternal: 1,
            EndRewardTimeInternal: FAR_FUTURE,
            ActivityOpenType: Some(ActivityOpenType::Permanent),
            IsUnlock: true,
            IsFirstOpen: false,
            IsPreOpen: false,
            ..Default::default()
        };
        populate_sub_payload(&mut act, act_type, def.id, &claims, &prog_ints);
        out.push(act);
    }
    out
}

pub fn handle_activity_request(player_id: i32, _payload: &[u8]) -> AkiResult {
    let activities = build_activities(player_id);
    let resp = ActivityResponse {
        Activities: activities,
        ErrorCode: Some(ErrorCode::Success),
    };
    AkiResult::ok(msg_id::ACTIVITY_RESPONSE, &resp)
}

pub fn handle_activity_update_notify(player_id: i32, _payload: &[u8]) -> AkiResult {
    let activities = build_activities(player_id);
    let notify = ActivityUpdateNotify {
        Activities: activities,
    };
    AkiResult::ok(msg_id::ACTIVITY_UPDATE_NOTIFY, &notify)
}
