use sonoro_gamesv_data::Assets;
use sonoro_gamesv_logic::component::player::PlayerProgressComponent;
use sonoro_gamesv_logic::helpers::condition::{self, EvalContext};
use sonoro_gamesv_logic::helpers::{energy, pb_kv};
use sonoro_gamesv_logic::sink::PushSink;
use sonoro_proto::PbMessage;
use sonoro_proto::enums::{ActivityOpenType, ActivityType, ConditionTaskState, ErrorCode, SignState};
use sonoro_proto::pb::{
    ActivityAvignon, ActivityBetHorsesData, ActivityBlackCoastData, ActivityCiacconaGalData,
    ActivityCorniceMeetingData, ActivityDangoMonopolyData, ActivityData, ActivityDataDataOneof,
    ActivityFishingData, ActivityFunPlayData, ActivityInviteNewbie, ActivityLineCrossData,
    ActivityLinkageData, ActivityLongShanMain, ActivityMapExploreData, ActivityMoonSignInData,
    ActivityMoraleData, ActivityPermanentRogueData, ActivityPrizeDrawingData, ActivityRegressData,
    ActivityRequest, ActivityResponse, ActivityRogueData, ActivityScratchTicketData,
    ActivityTaskData, ActivityTimePointRewarData, ActivityTrapDefenseData, ActivityTurnTableData,
    ActivityUpdateNotify, ActivityWeeklyRogueData, AdvertisingPageInfo, AreaInfo, ArtemisActivityInfo,
    BabelTowerActivity, BeginnerCarnivalData, BossPilingActivityInfo, BossRushActivityData,
    ConditionTask, CumulativeShopData, DailyAdventureActivityData, DangoAbyssActivityData,
    DangoBrawlActivityInfo, DoubleInstActivityReward, DreamLinkActivityData, DropCatchActivityInfo,
    EdgeRunnerActivityInfo, EncircleActivityPb, FarmGoldData, FeiXuePreheatActivityInfo,
    FightPhotoActivityData, FlagChallengeActivityInfo, FloroRanchActivityData,
    FurnitureDiySlotInfo, GatherActivityInfo, GiantActivityInfo, GolemCrackActivityInfo,
    H5ViewActivityData, HarvestActivity, HonamiStoryActivityData, InfrThemeActivityPb,
    KurotatoActivityData, LifePointDrawActivityData, LinkageCheckInActivityData,
    MapTravelActivityData, MatchThreeActivityInfo, MotorCycleIpActivityData,
    MotorDecalActivityData, MotorDevelopActivityData, MotorFightActivityPb,
    MotorFightTalentTreePb, MotorParkourActivityInfo, MowTowerActivityData, NewBieCourseActivity,
    NewPlayerSupportActivityData, NewPlayerSupportActivityV2Pb, NewTowerClimbingActivityData,
    NewbieAdventureV2Pb, NewbieCourseV2ActivityPb, NewbieMainActivityPb, OnlineMotorActivityData,
    ParkourActivity, PhantomArenaActivityData, PhantomBattleGuideActivity, PhantomCollectActivity,
    PinballActivityData, PreheatSignActivityData, QingXiaoActivityInfo, RealmBetweenActivityInfo,
    RhythmActivityPb, RiskHarvestActivityData, RoadBookActivityInfo, RoleCoopActivityData,
    RoleGiftActivityData, RoleSkinTrialActivity, RoleTowerActivityData, RoleTrialInfoActivity,
    RoguelikeSeason, RoverRogueActivityData, RoverRogueHistoryInsInfo, SignActivity,
    SkinRewardActivityData, SolarSchoolActivityPb, SolarisSpeedActivity,
    SpringFestivalActivityInfo, SpringSignData, SpringSkipEntry, SurvivorsActivityData,
    TetrisActivityInfo, ThemeCelebration, ThroughTrainActivityData,
    ThroughTrainSummaryActivityData, TotalTopUpActivityInfo, TowerDefenseActivityInfo,
    WuWuWeekActivity,
};
use sonoro_proto::wire::PbEnum;

use super::super::aki_logic_bridge;
use super::{DispatchError, State, deliver_aki_response};

fn has_missing_build_asset(act_id: i32) -> bool {
    matches!(
        act_id,
        100100019
            | 100100020
            | 100100021
            | 100200001
            | 107100001
            | 107500003
            | 108000001
            | 108200002
            | 108400002
            | 109000001
            | 109100001
            | 109100002
            | 109400001
            | 109500001
            | 109600001
            | 109900001
            | 110000001
            | 110100001
            | 110100002
            | 110300001
            | 110400001
            | 110800001
            | 110900001
            | 111000001
            | 111100001
    )
}

pub fn is_activity_unlocked(
    cond_ctx_opt: Option<&EvalContext<'_>>,
    assets: &Assets,
    act_id: i32,
) -> bool {
    if has_missing_build_asset(act_id) {
        return false;
    }
    let Some(cond_ctx) = cond_ctx_opt else {
        return true;
    };
    if !condition::is_function_open(cond_ctx, 10053) {
        return false;
    }
    let Some(def) = assets.tables.activity.get_data_by_id(&act_id) else {
        return false;
    };
    if def.PreConditionGroupId <= 0 {
        return true;
    }
    condition::eval_condition_group(cond_ctx, def.PreConditionGroupId)
}

pub fn build_activity_entry(
    id: i32,
    raw_type: i32,
    now_s: i64,
    is_unlock: bool,
    data: Option<ActivityDataDataOneof>,
) -> Option<ActivityData> {
    let act_type = ActivityType::from_i32_or_default(raw_type);
    if (act_type as i32) != raw_type {
        return None;
    }
    let begin_s = (now_s - 86_400).max(1);
    let end_s = begin_s + 30 * 86_400;
    Some(ActivityData {
        Id: id,
        Type: Some(act_type),
        BeginShowTime: begin_s,
        EndShowTime: end_s,
        BeginOpenTime: begin_s,
        EndOpenTime: end_s,
        IsUnlock: is_unlock,
        IsFirstOpen: false,
        ActivityOpenType: Some(ActivityOpenType::Permanent),
        IsPreOpen: false,
        StartTime: begin_s,
        EndTime: end_s,
        BeginRewardTimeInternal: begin_s,
        EndRewardTimeInternal: end_s,
        Data: data,
        ..Default::default()
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

fn giant_total_stable_value(progress_comp: &PlayerProgressComponent) -> i32 {
    let mut total = 0i64;
    for eco_type in 0..3 {
        let key = format!("giant:stable:{eco_type}");
        total += progress_comp.info.get_key_int(&key).max(0);
    }
    total.min(i64::from(i32::MAX)) as i32
}

fn default_data_for_type(
    act_type: ActivityType,
    id: i32,
    now_s: i64,
    progress_comp: &PlayerProgressComponent,
) -> Option<ActivityDataDataOneof> {
    let end_s = now_s + 30 * 86_400;
    Some(match act_type {
        ActivityType::Parkour => ActivityDataDataOneof::ParkourActivity(ParkourActivity::default()),
        ActivityType::GatherActivity => {
            ActivityDataDataOneof::GatherActivityInfo(GatherActivityInfo::default())
        }
        ActivityType::Sign => ActivityDataDataOneof::SignActivity(SignActivity::default()),
        ActivityType::TowerGuide | ActivityType::TowerGuideNew | ActivityType::CircumFluence
        | ActivityType::TrackMoonPhase => return None,
        ActivityType::NewBieCourse => {
            ActivityDataDataOneof::NewBieCourseActivity(NewBieCourseActivity::default())
        }
        ActivityType::RougeActivity => ActivityDataDataOneof::ActivityRogueData(ActivityRogueData {
            BeginOpenTime: 1,
            EndOpenTime: end_s,
            RoguelikeSeason: Some(RoguelikeSeason {
                SeasonId: rogue_season_id_for_activity(id),
                EndTime: end_s,
                ..Default::default()
            }),
        }),
        ActivityType::DoubleInstanceRewardActivity => {
            ActivityDataDataOneof::DoubleInstActivityReward(DoubleInstActivityReward::default())
        }
        ActivityType::RoleTrialActivity => {
            ActivityDataDataOneof::RoleTrialInfoActivity(RoleTrialInfoActivity::default())
        }
        ActivityType::Harvest => ActivityDataDataOneof::HarvestActivity(HarvestActivity::default()),
        ActivityType::PhantomCollect => {
            ActivityDataDataOneof::PhantomCollectActivity(PhantomCollectActivity::default())
        }
        ActivityType::DailyAdventureActivity => {
            ActivityDataDataOneof::DailyAdventureActivityData(DailyAdventureActivityData::default())
        }
        ActivityType::LongShanMainActivity => {
            ActivityDataDataOneof::ActivityLongShanMain(ActivityLongShanMain::default())
        }
        ActivityType::BossRushActivity => {
            ActivityDataDataOneof::BossRushActivityData(BossRushActivityData::default())
        }
        ActivityType::TurnTableActivity => {
            ActivityDataDataOneof::ActivityTurnTableData(ActivityTurnTableData::default())
        }
        ActivityType::TrackMoonActivity => {
            ActivityDataDataOneof::TrackMoonActivityTaskData(ActivityTaskData::default())
        }
        ActivityType::TowerDefenceActivity => {
            ActivityDataDataOneof::TowerDefenseActivityInfo(TowerDefenseActivityInfo::default())
        }
        ActivityType::TimePointRewardActivity => {
            ActivityDataDataOneof::ActivityTimePointRewarData(ActivityTimePointRewarData::default())
        }
        ActivityType::RiskHarvest => {
            ActivityDataDataOneof::RiskHarvestActivityData(RiskHarvestActivityData::default())
        }
        ActivityType::CorniceMeeting => {
            ActivityDataDataOneof::ActivityCorniceMeetingData(ActivityCorniceMeetingData::default())
        }
        ActivityType::BlackCoastTheme => {
            ActivityDataDataOneof::ActivityBlackCoastData(ActivityBlackCoastData::default())
        }
        ActivityType::RogueWhiteCat => {
            ActivityDataDataOneof::DreamLinkActivityData(DreamLinkActivityData::default())
        }
        ActivityType::ScratchCard => {
            ActivityDataDataOneof::ActivityScratchTicketData(ActivityScratchTicketData::default())
        }
        ActivityType::PreheatSign => {
            ActivityDataDataOneof::PreheatSignActivityData(PreheatSignActivityData::default())
        }
        ActivityType::MowTower => {
            ActivityDataDataOneof::MowTowerActivityData(MowTowerActivityData::default())
        }
        ActivityType::ThroughTrain => {
            ActivityDataDataOneof::ThroughTrainActivityData(ThroughTrainActivityData::default())
        }
        ActivityType::SprintSign => ActivityDataDataOneof::SpringSignData(SpringSignData::default()),
        ActivityType::MapTravelActivity => {
            ActivityDataDataOneof::MapTravelActivityData(MapTravelActivityData::default())
        }
        ActivityType::FarmGold => ActivityDataDataOneof::FarmGoldData(FarmGoldData::default()),
        ActivityType::RoleSkinTrialActivity => {
            ActivityDataDataOneof::RoleSkinTrialActivity(RoleSkinTrialActivity::default())
        }
        ActivityType::RogueWeekly => {
            ActivityDataDataOneof::ActivityWeeklyRogueData(ActivityWeeklyRogueData {
                CycleId: progress_comp.info.get_key_int("rogue_weekly:cycle").max(1) as i32,
                MaxFreeCount: 3,
                UseFreeCount: progress_comp.info.get_key_int("rogue_weekly:bf_used") as i32,
                ..Default::default()
            })
        }
        ActivityType::FishingActivity => {
            ActivityDataDataOneof::ActivityFishingData(ActivityFishingData::default())
        }
        ActivityType::TeamParkOurActivity => {
            ActivityDataDataOneof::SolarisSpeedActivity(SolarisSpeedActivity::default())
        }
        ActivityType::BabelTower => {
            ActivityDataDataOneof::BabelTowerActivity(BabelTowerActivity::default())
        }
        ActivityType::BetHorses => {
            ActivityDataDataOneof::ActivityBetHorsesData(ActivityBetHorsesData {
                ActivityId: id,
                ..Default::default()
            })
        }
        ActivityType::Explore => {
            ActivityDataDataOneof::ActivityMapExploreData(ActivityMapExploreData::default())
        }
        ActivityType::Abyss => {
            ActivityDataDataOneof::DangoAbyssActivityData(DangoAbyssActivityData {
                StartTime: 1,
                EndTime: end_s * 1000,
                ..Default::default()
            })
        }
        ActivityType::RogueRes => {
            ActivityDataDataOneof::ActivityPermanentRogueData(ActivityPermanentRogueData::default())
        }
        ActivityType::DangoMonopoly => {
            ActivityDataDataOneof::ActivityDangoMonopolyData(ActivityDangoMonopolyData {
                CurrentBoardId: 1,
                ..Default::default()
            })
        }
        ActivityType::CiacconaActivity => {
            ActivityDataDataOneof::ActivityCiacconaGalData(ActivityCiacconaGalData::default())
        }
        ActivityType::Avignon => {
            ActivityDataDataOneof::ActivityAvignon(ActivityAvignon::default())
        }
        ActivityType::H5CircumFluence => {
            ActivityDataDataOneof::ActivityInviteNewbie(ActivityInviteNewbie::default())
        }
        ActivityType::H5View => {
            ActivityDataDataOneof::H5ViewActivityData(H5ViewActivityData::default())
        }
        ActivityType::ActivityLinkage => {
            ActivityDataDataOneof::ActivityLinkageData(ActivityLinkageData {
                ActivityId: id,
                ..Default::default()
            })
        }
        ActivityType::RegressActivity => {
            ActivityDataDataOneof::ActivityRegressData(ActivityRegressData {
                EndTime: end_s,
                Grade: 1,
                ..Default::default()
            })
        }
        ActivityType::ConsumptiveActivity => {
            ActivityDataDataOneof::CumulativeShopData(CumulativeShopData {
                ActivityId: id,
                TotalScore: progress_comp.info.get_key_int("cons:score") as i32,
                ..Default::default()
            })
        }
        ActivityType::PhantomBattle => {
            ActivityDataDataOneof::PhantomArenaActivityData(PhantomArenaActivityData::default())
        }
        ActivityType::PhantomBattleRecord => {
            ActivityDataDataOneof::PhantomBattleRecordActivityInfo(
                PhantomArenaActivityData::default(),
            )
        }
        ActivityType::PhantomBattleRecordGuide => {
            ActivityDataDataOneof::PhantomBattleGuideActivity(PhantomBattleGuideActivity::default())
        }
        ActivityType::AdvanceNoticeActivity => {
            ActivityDataDataOneof::AdvertisingPageInfo(AdvertisingPageInfo::default())
        }
        ActivityType::ArtemisActivity => {
            ActivityDataDataOneof::ArtemisActivityInfo(ArtemisActivityInfo::default())
        }
        ActivityType::NewbieCarnival => {
            ActivityDataDataOneof::BeginnerCarnivalData(BeginnerCarnivalData::default())
        }
        ActivityType::MoraleActivity => {
            ActivityDataDataOneof::ActivityMoraleData(ActivityMoraleData::default())
        }
        ActivityType::FloroRanchActivity => {
            ActivityDataDataOneof::FloroRanchActivityData(FloroRanchActivityData::default())
        }
        ActivityType::LifePointChallenge => {
            ActivityDataDataOneof::LifePointDrawActivityData(LifePointDrawActivityData::default())
        }
        ActivityType::TrapDefense => {
            ActivityDataDataOneof::ActivityTrapDefenseData(ActivityTrapDefenseData::default())
        }
        ActivityType::FunPlay => {
            ActivityDataDataOneof::ActivityFunPlayData(ActivityFunPlayData::default())
        }
        ActivityType::MoonPhase => {
            ActivityDataDataOneof::ActivityMoonSignInData(ActivityMoonSignInData::default())
        }
        ActivityType::LineCross => {
            ActivityDataDataOneof::ActivityLineCrossData(ActivityLineCrossData::default())
        }
        ActivityType::HonamiStory => {
            ActivityDataDataOneof::HonamiStoryActivityData(HonamiStoryActivityData::default())
        }
        ActivityType::Survivors => {
            ActivityDataDataOneof::SurvivorsActivityData(SurvivorsActivityData::default())
        }
        ActivityType::PhotoFight => {
            ActivityDataDataOneof::FightPhotoActivityData(FightPhotoActivityData::default())
        }
        ActivityType::WuWuKuji => {
            ActivityDataDataOneof::ActivityPrizeDrawingData(ActivityPrizeDrawingData::default())
        }
        ActivityType::InfrTheme => {
            ActivityDataDataOneof::InfrThemeActivityPb(InfrThemeActivityPb::default())
        }
        ActivityType::CoopActivity => {
            ActivityDataDataOneof::RoleCoopActivityData(RoleCoopActivityData::default())
        }
        ActivityType::MotorcycleIpLink => {
            ActivityDataDataOneof::MotorCycleIpActivityData(MotorCycleIpActivityData::default())
        }
        ActivityType::SkinRewardActivity => {
            ActivityDataDataOneof::SkinRewardActivityData(SkinRewardActivityData::default())
        }
        ActivityType::RoadBookActivity => {
            ActivityDataDataOneof::RoadBookActivityInfo(RoadBookActivityInfo::default())
        }
        ActivityType::NewTowerClimbing => {
            ActivityDataDataOneof::NewTowerClimbingActivityData(NewTowerClimbingActivityData {
                CycleId: 1,
                SeasonId: 1,
                CycleBeginTime: 1,
                CycleCloseTime: end_s,
                SeasonBeginTime: 1,
                ..Default::default()
            })
        }
        ActivityType::MotorFight => {
            ActivityDataDataOneof::MotorFightActivityPb(MotorFightActivityPb {
                TalentTree: Some(MotorFightTalentTreePb::default()),
                ..Default::default()
            })
        }
        ActivityType::Encircle => {
            ActivityDataDataOneof::EncircleActivityPb(EncircleActivityPb::default())
        }
        ActivityType::NewPlayerSupportActivity => {
            ActivityDataDataOneof::NewPlayerSupportActivityData(
                NewPlayerSupportActivityData::default(),
            )
        }
        ActivityType::SpringFestivalActivity => {
            ActivityDataDataOneof::SpringFestivalActivityInfo(SpringFestivalActivityInfo::default())
        }
        ActivityType::MotorParkourActivity => {
            ActivityDataDataOneof::MotorParkourActivityInfo(MotorParkourActivityInfo::default())
        }
        ActivityType::TotalTopUp => {
            ActivityDataDataOneof::TotalTopUpActivityInfo(TotalTopUpActivityInfo {
                Score: progress_comp.info.get_key_int("topup:score") as i32,
                ..Default::default()
            })
        }
        ActivityType::MotorDevelop => {
            ActivityDataDataOneof::MotorDevelopActivityData(MotorDevelopActivityData::default())
        }
        ActivityType::FlagChallenge => {
            ActivityDataDataOneof::FlagChallengeActivityInfo(FlagChallengeActivityInfo::default())
        }
        ActivityType::Rhythm => ActivityDataDataOneof::RhythmActivityPb(RhythmActivityPb::default()),
        ActivityType::FeiXue => {
            ActivityDataDataOneof::FeiXuePreheatActivityInfo(FeiXuePreheatActivityInfo::default())
        }
        ActivityType::DropCatchActivity => {
            ActivityDataDataOneof::DropCatchActivityInfo(DropCatchActivityInfo::default())
        }
        ActivityType::TetrisActivity => {
            ActivityDataDataOneof::TetrisActivityInfo(TetrisActivityInfo::default())
        }
        ActivityType::Kurotato => {
            ActivityDataDataOneof::KurotatoActivityData(KurotatoActivityData::default())
        }
        ActivityType::PinballActivity => {
            ActivityDataDataOneof::PinballActivityData(PinballActivityData::default())
        }
        ActivityType::WuWuWeekSign => {
            ActivityDataDataOneof::WuWuWeekActivity(WuWuWeekActivity::default())
        }
        ActivityType::AnniversaryTheme => {
            ActivityDataDataOneof::ThemeCelebration(ThemeCelebration::default())
        }
        ActivityType::BossPiling => {
            ActivityDataDataOneof::BossPilingActivityInfo(BossPilingActivityInfo::default())
        }
        ActivityType::GolemCrackActivity => {
            ActivityDataDataOneof::GolemCrackActivityInfo(GolemCrackActivityInfo::default())
        }
        ActivityType::EdgeRunnerActivity => {
            ActivityDataDataOneof::EdgeRunnerActivityInfo(EdgeRunnerActivityInfo::default())
        }
        ActivityType::MotorDecalActivity => {
            ActivityDataDataOneof::MotorDecalActivityData(MotorDecalActivityData::default())
        }
        ActivityType::MotorParkour => {
            ActivityDataDataOneof::OnlineMotorActivityData(OnlineMotorActivityData::default())
        }
        ActivityType::LinkageCheckIn => {
            ActivityDataDataOneof::LinkageCheckInActivityData(LinkageCheckInActivityData::default())
        }
        ActivityType::RoleGiftActivity => {
            ActivityDataDataOneof::RoleGiftActivityData(RoleGiftActivityData::default())
        }
        ActivityType::RealmBetween => {
            ActivityDataDataOneof::RealmBetweenActivityInfo(RealmBetweenActivityInfo::default())
        }
        ActivityType::ThroughTrainSummary => ActivityDataDataOneof::ThroughTrainSummaryActivityData(
            ThroughTrainSummaryActivityData::default(),
        ),
        ActivityType::NewPlayerSupportActivityV2 => {
            ActivityDataDataOneof::NewPlayerSupportActivityV2Pb(
                NewPlayerSupportActivityV2Pb::default(),
            )
        }
        ActivityType::NewbieAdventureV2 => {
            ActivityDataDataOneof::NewbieAdventureV2Pb(NewbieAdventureV2Pb::default())
        }
        ActivityType::NewbieCourseV2 => {
            ActivityDataDataOneof::NewbieCourseV2ActivityPb(NewbieCourseV2ActivityPb::default())
        }
        ActivityType::NewbieMain => {
            ActivityDataDataOneof::NewbieMainActivityPb(NewbieMainActivityPb::default())
        }
        ActivityType::RoverRogue => {
            ActivityDataDataOneof::RoverRogueActivityData(RoverRogueActivityData {
                activityId: id,
                HistoryInsInfo: Some(RoverRogueHistoryInsInfo::default()),
                ..Default::default()
            })
        }
        ActivityType::DangoBrawl => {
            ActivityDataDataOneof::DangoBrawlActivityInfo(DangoBrawlActivityInfo::default())
        }
        ActivityType::QingXiaoPlayThePiano => {
            ActivityDataDataOneof::QingXiaoActivityInfo(QingXiaoActivityInfo::default())
        }
        ActivityType::MatchThreeActivity => {
            ActivityDataDataOneof::MatchThreeActivityInfo(MatchThreeActivityInfo::default())
        }
        ActivityType::SolarSchool => {
            ActivityDataDataOneof::SolarSchoolActivityPb(SolarSchoolActivityPb::default())
        }
        ActivityType::GiantSystemActivity => {
            ActivityDataDataOneof::GiantActivityInfo(GiantActivityInfo {
                TotalStableValue: giant_total_stable_value(progress_comp),
                ..Default::default()
            })
        }
        ActivityType::RoleTower => ActivityDataDataOneof::RoleTowerActivityData(RoleTowerActivityData {
            RankRewardSettleTime: end_s,
            Score: progress_comp.info.get_key_int("role_tower:score") as i32,
            ..Default::default()
        }),
        _ => return None,
    })
}

fn supports_base_only(act_type: ActivityType) -> bool {
    matches!(
        act_type,
        ActivityType::WorldNewJourney
            | ActivityType::NewRoleGuideActivity
            | ActivityType::PhotoMemoryActivity
            | ActivityType::NewLordGym
            | ActivityType::SlashAndTowerLevelPlay
            | ActivityType::PreHeatTaskActivity
            | ActivityType::InfrThemeV2
            | ActivityType::SheriffActivity
            | ActivityType::PurePreviewActivity
            | ActivityType::TowerGuide
            | ActivityType::TowerGuideNew
            | ActivityType::FirstPersonParkour
            | ActivityType::PureUiactivity
    )
}

fn build_spring_festival_activity(
    assets: &Assets,
    progress_comp: &PlayerProgressComponent,
    cond_ctx: Option<&EvalContext<'_>>,
    act_id: i32,
) -> SpringFestivalActivityInfo {
    let mut area_infos = Vec::new();
    let mut computed_atm = 0i32;
    for area in &assets.spring_festival_areas {
        if area.ActivityId != act_id {
            continue;
        }
        let key = format!("diy_slots:{act_id}:{}", area.Id);
        let area_info = if let Some(saved) = pb_kv::load_msg::<AreaInfo>(progress_comp, &key)
        {
            saved
        } else {
            let mut slot_infos = Vec::new();
            let mut default_atm = 0i32;
            for &slot_id in &area.SlotEntityIds {
                let root_furn = progress_comp
                    .info
                    .get_key_int(&format!("furn_slot:{slot_id}"))
                    .max(0) as i32;
                let root_furn = if root_furn > 0 {
                    root_furn
                } else {
                    area.DefaultFurnitures
                        .get(&slot_id.to_string())
                        .and_then(|d| d.ArrayInt.first().copied())
                        .unwrap_or(0)
                };
                if root_furn > 0
                    && let Some(fc) = assets.furniture_configs.get(&root_furn)
                {
                    default_atm = default_atm.saturating_add(fc.Atmosphere);
                }
                slot_infos.push(FurnitureDiySlotInfo {
                    SlotEntityCfgId: slot_id,
                    RootFurnitureId: root_furn,
                    SubFurnitureIds: Vec::new(),
                });
            }
            AreaInfo {
                AreaId: area.Id,
                Atmosphere: default_atm,
                FurnitureDiySlotInfos: slot_infos,
            }
        };
        computed_atm = computed_atm.saturating_add(area_info.Atmosphere);
        area_infos.push(area_info);
    }

    let mut unlock_furnitures: Vec<i32> = assets
        .furniture_configs
        .values()
        .filter(|f| f.ActivityId == act_id || f.ActivityId == 0)
        .map(|f| f.Id)
        .collect();
    unlock_furnitures.sort_unstable();
    unlock_furnitures.dedup();

    let claims = &progress_comp.info.dynamic.claims;
    let mut reward_score_ids = Vec::new();
    append_int_claim_suffixes(claims, "sf_score:", &mut reward_score_ids);
    let mut reward_level_ids = Vec::new();
    append_int_claim_suffixes(claims, "sf_atm:", &mut reward_level_ids);

    let saved_atm_score = progress_comp.info.get_key_int("spring:atmosphere_score") as i32;
    let atmosphere = saved_atm_score.max(computed_atm);
    let atmosphere_level = (progress_comp.info.get_key_int("spring:atmosphere") as i32)
        .max(if atmosphere > 0 { (atmosphere / 1000).max(1) } else { 0 });

    let condition_tasks = assets
        .spring_festival_rewards
        .iter()
        .filter(|r| r.ActivityId == act_id)
        .map(|r| {
            let taken = progress_comp
                .info
                .has_claim(&format!("sf_task:{}", r.Id));
            let finished = taken
                || (r.ConditionGroup > 0
                    && cond_ctx.is_some_and(|c| condition::eval_condition_group(c, r.ConditionGroup)));
            ConditionTask {
                Id: r.Id,
                Current: i32::from(finished),
                Target: 1,
                Status: Some(if taken {
                    ConditionTaskState::ConditionTaskTaken
                } else if finished {
                    ConditionTaskState::ConditionTaskFinish
                } else {
                    ConditionTaskState::ConditionTaskRunning
                }),
            }
        })
        .collect();

    let spring_skip_entries = assets
        .spring_festival_skip_entries
        .iter()
        .filter(|e| e.Activity == act_id)
        .map(|e| SpringSkipEntry {
            Id: e.Id,
            UnLock: e.UnlockCondition <= 0
                || cond_ctx.is_some_and(|c| condition::eval_condition_group(c, e.UnlockCondition)),
            Finish: e.FinishCondition > 0
                && cond_ctx.is_some_and(|c| condition::eval_condition_group(c, e.FinishCondition)),
        })
        .collect();

    SpringFestivalActivityInfo {
        AreaInfos: area_infos,
        UnlockFurnitures: unlock_furnitures,
        ConditionTasks: condition_tasks,
        RewardScoreIds: reward_score_ids,
        RewardLevelIds: reward_level_ids,
        Atmosphere: atmosphere,
        AtmosphereLevel: atmosphere_level,
        SpringSkipEntries: spring_skip_entries,
        ..Default::default()
    }
}

fn append_int_claim_suffixes(claims: &[String], prefix: &str, target: &mut Vec<i32>) {
    for c in claims {
        if let Some(rest) = c.strip_prefix(prefix)
            && let Ok(parsed) = rest.parse::<i32>()
            && parsed > 0
            && !target.contains(&parsed)
        {
            target.push(parsed);
        }
    }
}

pub fn build_activity_list(
    assets: &Assets,
    progress_comp: &PlayerProgressComponent,
    cond_ctx: Option<&EvalContext<'_>>,
) -> Vec<ActivityData> {
    let now_s = energy::now_seconds();
    let mut activities = Vec::new();

    for def in &assets.tables.activity.items {
        if matches!(
            def.Type,
            13 | 18
                | 22
                | 30
                | 41
                | 42
                | 45
                | 50
                | 55
                | 57
                | 58
                | 61
                | 62
                | 69
                | 72
                | 76
                | 80
                | 82
                | 83
                | 91
                | 97
                | 103
                | 104
                | 107
                | 108
                | 109
                | 110
                | 111
                | 113
                | 117
        ) {
            continue;
        }
        let act_type = ActivityType::from_i32_or_default(def.Type);
        if (act_type as i32) != def.Type {
            continue;
        }

        let act_data = if act_type == ActivityType::Sign {
            let mut sign_states = Vec::with_capacity(7);
            let mut prev_claimed = true;
            for day_idx in 0..7 {
                let key = format!("sign_act:{}:{day_idx}", def.Id);
                if progress_comp.info.has_claim(&key) {
                    sign_states.push(SignState::IsReceive);
                } else if prev_claimed {
                    sign_states.push(SignState::Unlock);
                    prev_claimed = false;
                } else {
                    sign_states.push(SignState::Lock);
                }
            }
            let free_key = format!("sign_free:{}", def.Id);
            Some(ActivityDataDataOneof::SignActivity(SignActivity {
                SignStateList: sign_states,
                RewardFree: progress_comp.info.has_claim(&free_key),
            }))
        } else if act_type == ActivityType::NewBieCourse {
            let mut claimed = Vec::new();
            let claims = &progress_comp.info.dynamic.claims;
            append_int_claim_suffixes(claims, "region_prog:NewBieCourseRewardRequest:", &mut claimed);
            append_int_claim_suffixes(claims, "newbie:course:", &mut claimed);
            Some(ActivityDataDataOneof::NewBieCourseActivity(
                NewBieCourseActivity {
                    HadTakeReward: claimed,
                },
            ))
        } else if act_type == ActivityType::DoubleInstanceRewardActivity {
            Some(ActivityDataDataOneof::DoubleInstActivityReward(
                DoubleInstActivityReward::default(),
            ))
        } else if act_type == ActivityType::RoleTrialActivity {
            Some(ActivityDataDataOneof::RoleTrialInfoActivity(
                RoleTrialInfoActivity::default(),
            ))
        } else if act_type == ActivityType::SpringFestivalActivity {
            Some(ActivityDataDataOneof::SpringFestivalActivityInfo(
                build_spring_festival_activity(assets, progress_comp, cond_ctx, def.Id),
            ))
        } else {
            default_data_for_type(act_type, def.Id, now_s, progress_comp)
        };

        if act_data.is_none() && !supports_base_only(act_type) {
            continue;
        }
        if !is_activity_unlocked(cond_ctx, assets, def.Id) {
            continue;
        }
        if let Some(entry) = build_activity_entry(def.Id, def.Type, now_s, true, act_data) {
            activities.push(entry);
        }
    }

    activities
}

#[allow(clippy::too_many_arguments)]
pub fn push_activity_update_notify<S: PushSink + ?Sized>(
    sink: &mut S,
    assets: &Assets,
    progress_comp: &PlayerProgressComponent,
    basic_comp: &sonoro_gamesv_logic::component::player::PlayerBasicComponent,
    echo_comp: &sonoro_gamesv_logic::component::player::PlayerEchoComponent,
    inventory_comp: &sonoro_gamesv_logic::component::player::PlayerInventoryComponent,
    map_comp: &sonoro_gamesv_logic::component::player::PlayerMapComponent,
    quest_comp: &sonoro_gamesv_logic::component::player::PlayerQuestComponent,
    role_comp: Option<&sonoro_gamesv_logic::component::player::PlayerRoleComponent>,
    player_id: i32,
) {
    let cond_ctx = EvalContext {
        assets: Some(assets),
        basic_comp,
        echo_comp,
        inventory_comp,
        map_comp,
        progress_comp: Some(progress_comp),
        quest_comp,
        role_comp,
        weapon_comp: None,
        guide_comp: None,
        scene: None,
        owner_quest_id: 0,
        target_role_id: 0,
        self_entity_config_id: 0,
    };

    if let Ok(resp) = aki_logic_bridge::call_player(player_id, 0, ActivityUpdateNotify::MSG_ID, &[])
        && resp.msg_id == ActivityUpdateNotify::MSG_ID
        && !resp.payload.is_empty()
        && let Ok(mut decoded) = ActivityUpdateNotify::decode(&resp.payload)
    {
        decoded.Activities.retain(|act| is_activity_unlocked(Some(&cond_ctx), assets, act.Id));
        for act in &mut decoded.Activities {
            act.IsUnlock = true;
        }
        sink.push(ActivityUpdateNotify::MSG_ID, &decoded);
        return;
    }

    let acts = build_activity_list(assets, progress_comp, Some(&cond_ctx));
    sink.push(
        ActivityUpdateNotify::MSG_ID,
        &ActivityUpdateNotify { Activities: acts },
    );
}

pub fn on_activity_request(
    state: &mut State,
    rpc_id: u16,
    raw_payload: &[u8],
) -> Result<(), DispatchError> {
    if ActivityRequest::decode(raw_payload).is_err() {
        let _ = state.conn.respond_with_msg_id(
            rpc_id,
            ActivityResponse::MSG_ID,
            &ActivityResponse {
                ErrorCode: Some(ErrorCode::InternalError),
                ..Default::default()
            },
        );
        return Ok(());
    }

    if let Ok(resp) = aki_logic_bridge::call_player(state.player_id, 0, 16076, raw_payload) {
        if resp.msg_id == ActivityResponse::MSG_ID
            && !resp.payload.is_empty()
            && let Ok(mut decoded) = ActivityResponse::decode(&resp.payload)
        {
            let cond_ctx = state.eval_context();
            decoded
                .Activities
                .retain(|act| is_activity_unlocked(Some(&cond_ctx), &state.assets, act.Id));
            for act in &mut decoded.Activities {
                act.IsUnlock = true;
            }
            state
                .conn
                .respond_with_msg_id(rpc_id, ActivityResponse::MSG_ID, &decoded)
                .map_err(|_| DispatchError::Closed)?;
            return Ok(());
        }
        if resp.msg_id != 0 {
            return deliver_aki_response(state, rpc_id, resp);
        }
    }

    let cond_ctx = state.eval_context();
    let activities = build_activity_list(
        &state.assets,
        &state.player_components.progress,
        Some(&cond_ctx),
    );
    state
        .conn
        .respond_with_msg_id(
            rpc_id,
            ActivityResponse::MSG_ID,
            &ActivityResponse {
                Activities: activities,
                ErrorCode: Some(ErrorCode::Success),
            },
        )
        .map_err(|_| DispatchError::Closed)?;
    Ok(())
}
