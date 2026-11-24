using System;
using System.IO;
using System.Text.Json;
using System.Collections.Generic;
using Aki.Protocol;
using Google.Protobuf;

namespace AkiLogicService;

public static class ActivityHandler
{
    private static readonly List<ActivityConfigItem> CachedConfig = new();
    private static readonly string BinDataPath = Path.Combine(BinDataCache.BinDataDir, "Activity.json");

    public class ActivityConfigItem
    {
        public int Id { get; set; }
        public int Type { get; set; }
        public int OpenType { get; set; }
        public List<string>? TabTexture { get; set; }
    }

    private static bool HasMissingBuildAsset(ActivityConfigItem def)
    {
        if (def.TabTexture == null) return false;
        foreach (var tex in def.TabTexture)
        {
            if (tex.Contains("SignIn32", StringComparison.Ordinal) ||
                tex.Contains("SignIn33", StringComparison.Ordinal) ||
                tex.Contains("SignIn34", StringComparison.Ordinal) ||
                tex.Contains("Activity32/", StringComparison.Ordinal) ||
                tex.Contains("Activity33/", StringComparison.Ordinal) ||
                tex.Contains("Activity34/", StringComparison.Ordinal))
            {
                return true;
            }
        }
        return false;
    }

    // Legacy/seasonal minigames whose PhraseEx or UI expects complex stage matrices, collab assets,
    // or server-session room state (matches decompiled/index/activities.json and gamesv/src/network/handlers/activity.zig).
    private static readonly HashSet<int> ExcludedTypes = new()
    {
        13, 18, 22, 30, 41, 42, 45, 50, 55, 57, 58, 61, 62, 69, 72, 76, 80, 82, 83, 91, 97, 103, 104, 107, 108, 111, 113, 117
    };

    private static int RogueSeasonIdForActivity(int activityId) => activityId switch
    {
        100600001 => 1,
        100600002 => 2,
        100600003 => 3,
        100600004 => 4,
        102600001 => 5,
        100600005 => 6,
        100600006 => 7,
        _ => 1,
    };

    private static readonly Dictionary<ActivityType, Action<ActivityData, int, long, HashSet<string>>> SubPayloadPopulators = new()
    {
        [ActivityType.Parkour] = (a, _, _, _) => a.ParkourActivity = new ParkourActivity(),
        [ActivityType.GatherActivity] = (a, _, _, _) => a.GatherActivityInfo = new GatherActivityInfo(),
        [ActivityType.Sign] = (a, id, _, claims) =>
        {
            var sign = new SignActivity();
            for (int d = 0; d < 7; d++)
            {
                sign.SignStateList.Add(claims.Contains($"sign_act:{id}:{d}") ? SignState.IsReceive : SignState.Unlock);
            }
            a.SignActivity = sign;
        },
        [ActivityType.NewBieCourse] = (a, _, _, claims) =>
        {
            var nbc = new NewBieCourseActivity();
            PopulateIntClaimSuffixes(claims, "region_prog:NewBieCourseRewardRequest:", nbc.HadTakeReward);
            PopulateIntClaimSuffixes(claims, "newbie:course:", nbc.HadTakeReward);
            a.NewBieCourseActivity = nbc;
        },
        [ActivityType.RougeActivity] = (a, id, farFuture, _) => a.RoguelikeActivityData = new RoguelikeActivityData
        {
            BeginOpenTime = 1,
            EndOpenTime = farFuture,
            RogueSeasonData = new RogueSeasonData { SeasonId = RogueSeasonIdForActivity(id), EndTime = farFuture }
        },
        [ActivityType.DoubleInstanceRewardActivity] = (a, _, _, _) => a.DoubleInstActivityReward = new DoubleInstActivityReward(),
        [ActivityType.RoleTrialActivity] = (a, _, _, _) => a.RoleTrialInfoActivity = new RoleTrialInfoActivity(),
        [ActivityType.Harvest] = (a, _, _, _) => a.HarvestActivity = new HarvestActivity(),
        [ActivityType.PhantomCollect] = (a, _, _, _) => a.PhantomCollectActivity = new PhantomCollectActivity(),
        [ActivityType.DailyAdventureActivity] = (a, _, _, _) => a.DailyAdventureActivityData = new DailyAdventureActivityData(),
        [ActivityType.LongShanMainActivity] = (a, _, _, _) => a.LongShanActivityData = new LongShanActivity(),
        [ActivityType.BossRushActivity] = (a, _, _, _) => a.BossRushActivityData = new BossRushActivityData(),
        [ActivityType.TurnTableActivity] = (a, _, _, _) => a.TurntableActivityData = new TurntableActivityData(),
        [ActivityType.TrackMoonActivity] = (a, _, _, _) => a.TrackMoonActivityTaskData = new ActivityTaskData(),
        [ActivityType.CircumFluence] = (a, _, _, _) => a.CircumFluenceTaskData = new CircumFluenceData(),
        [ActivityType.TowerDefenceActivity] = (a, _, _, _) => a.TowerDefenceActivityData = new TowerDefenceActivityInfo(),
        [ActivityType.TimePointRewardActivity] = (a, _, _, _) => a.TimePointRewardActivityData = new TimePointRewardActivityData(),
        [ActivityType.TrackMoonPhase] = (a, _, _, _) => a.TraceMoonPhaseData = new TraceMoonPhaseData(),
        [ActivityType.RiskHarvest] = (a, _, _, _) => a.RiskHarvestActivityInfo = new RiskHarvestActivityInfo(),
        [ActivityType.CorniceMeeting] = (a, _, _, _) => a.CorniceMeetingData = new CorniceMeetingData(),
        [ActivityType.BlackCoastTheme] = (a, _, _, _) => a.BlackCoastThemeActivityData = new BlackCoastThemeActivity(),
        [ActivityType.RogueWhiteCat] = (a, _, _, _) => a.RogueWhiteCatData = new RogueWhiteCatData(),
        [ActivityType.ScratchCard] = (a, _, _, _) => a.ScratchCardActivityInfo = new ScratchCardActivityInfo(),
        [ActivityType.PreheatSign] = (a, _, _, _) => a.PreheatSignActivityInfo = new PreheatSignActivityInfo(),
        [ActivityType.MowTower] = (a, _, _, _) => a.MowTowerActivityInfo = new MowTowerActivityInfo(),
        [ActivityType.ThroughTrain] = (a, _, _, _) => a.ThroughTrainActivityData = new ThroughTrainActivityData(),
        [ActivityType.SprintSign] = (a, _, _, _) => a.SpringSignInfo = new SpringSignInfo(),
        [ActivityType.MapTravelActivity] = (a, _, _, _) => a.MapTravelActivityInfo = new MapTravelActivityInfo(),
        [ActivityType.FarmGold] = (a, _, _, _) => a.FarmGoldInfo = new FarmGoldInfo(),
        [ActivityType.RoleSkinTrialActivity] = (a, _, _, _) => a.RoleSkinTrialInfoActivity = new RoleSkinTrialInfoActivity(),
        [ActivityType.RogueWeekly] = (a, _, _, _) => a.RogueWeeklyData = new RogueWeeklyData
        {
            CycleId = 1,
            MaxFreeCount = 3,
            CurWorldLevel = 1,
            RogueWeeklyLastInfo = new RogueWeeklyLastInfo()
        },
        [ActivityType.FishingActivity] = (a, _, _, _) => a.FishingActivityInfo = new FishingActivityInfo(),
        [ActivityType.TeamParkOurActivity] = (a, _, _, _) => a.TeamParkourLevelActivityInfo = new TeamParkourLevelActivityInfo
        {
            ActivityTaskDatas = new ActivityTaskData()
        },
        [ActivityType.BabelTower] = (a, _, _, _) => a.BabelActivityInfo = new BabelActivityInfo(),
        [ActivityType.Avignon] = (a, _, _, _) => a.AvignonActivityInfo = new AvignonActivityInfo
        {
            ActivityTaskDatas = new ActivityTaskData()
        },
        [ActivityType.BetHorses] = (a, id, _, _) => a.RacingBetsData = new RacingBetsActivityData { ActivityId = id },
        [ActivityType.Explore] = (a, _, _, _) => a.ExploreActivityInfo = new ExploreActivityInfo(),
        [ActivityType.Abyss] = (a, _, farFuture, _) => a.AbyssInfo = new AbyssInfo
        {
            LimitBeginTime = 1,
            LimitEndTime = farFuture * 1000L
        },
        [ActivityType.RogueRes] = (a, _, farFuture, _) =>
        {
            var r = new RogueResActivityData();
            r.RogueResSeasonData.Add(new RogueResSeasonData { SeasonId = 1, EndTime = farFuture });
            a.RogueResActivityData = r;
        },
        [ActivityType.H5CircumFluence] = (a, _, _, _) => a.H5CircumFluenceData = new H5CircumFluenceActivityData(),
        [ActivityType.DangoMonopoly] = (a, _, _, _) => a.DangoMonopolyActivityData = new DangoMonopolyActivityPbData { BoardId = 1 },
        [ActivityType.CiacconaActivity] = (a, _, farFuture, _) => a.CiacconaActivityPbData = new CiacconaActivityPbData
        {
            InspirationData = new CiacconaInspirationPbData(),
            LimitedStartTime = 0,
            LimitedEndTime = farFuture
        },
        [ActivityType.ActivityLinkage] = (a, id, _, _) => a.ActivityLinkageData = new ActivityLinkageData { ActivityId = id },
        [ActivityType.RegressActivity] = (a, _, farFuture, _) => a.RegressData = new RegressData { EndTime = farFuture, GradeId = 1 },
        [ActivityType.ConsumptiveActivity] = (a, id, _, _) => a.ConsumptiveActivityInfo = new ConsumptiveActivityInfo { ActivityId = id },
        [ActivityType.PhantomBattle] = (a, _, farFuture, _) => a.PhantomBattleActivityInfo = new PhantomBattleActivityInfo
        {
            PhantomBattleMasterInfo = new PhantomBattleMasterInfo(),
            TimeLimitShopEndTime = farFuture
        },
        [ActivityType.NewbieCarnival] = (a, _, _, _) => a.NewbieCarnivalData = new NewbieCarnivalData
        {
            ActivityTaskData = new ActivityTaskData()
        },
        [ActivityType.MoraleActivity] = (a, _, _, _) => a.MoraleActivityData = new MoraleActivityData(),
        [ActivityType.FloroRanchActivity] = (a, _, _, _) => a.FloroRanchActivityData = new FloroRanchActivityData
        {
            PlayerInfo = new FloroRanchPlayerInfo()
        },
        [ActivityType.LifePointChallenge] = (a, _, _, _) => a.LifePointChallenge = new LifePointChallengeActivityData(),
        [ActivityType.TrapDefense] = (a, _, farFuture, _) =>
        {
            var td = new TrapDefenseActivityInfo
            {
                LeftLevelUpPoint = 100,
                LeftTechPoint = 100,
                LimitBeginTime = 0,
                LimitEndTime = farFuture
            };
            var dummyReward = new TrapDefenseRewardInfo
            {
                TaskInfo = new ConditionTask { Id = 1001, Status = ConditionTaskState.ConditionTaskFinish },
                LimitBeginTime = 0,
                LimitEndTime = farFuture
            };
            td.SpecialRewards.Add(dummyReward);
            td.Rewards.Add(dummyReward);
            a.TrapDefenseActivityInfo = td;
        },
        [ActivityType.JinzhouFlyActivity] = (a, _, _, _) => a.JinzhouFlyData = new JinzhouFlyData(),
        [ActivityType.FunPlay] = (a, _, _, _) => a.FunPlayActivityInfo = new FunPlayActivityInfo(),
        [ActivityType.MoonPhase] = (a, _, _, _) => a.MoonPhaseActivityData = new MoonPhaseActivityData(),
        [ActivityType.LineCross] = (a, _, _, _) => a.LineCrossActivityData = new LineCrossActivityData(),
        [ActivityType.HonamiStory] = (a, _, _, _) => a.HonamiStoryActivityInfo = new HonamiStoryActivityInfo
        {
            HonamiStoryPlayerBagInfo = new HonamiStoryPlayerBagInfo()
        },
        [ActivityType.Survivors] = (a, _, _, _) => a.SurvivorsActivityData = new SurvivorsActivityData
        {
            NormalTaskData = new ActivityTaskData()
        },
        [ActivityType.PhotoFight] = (a, _, _, _) => a.PhotoFightActivityData = new PhotoFightActivityPb(),
        [ActivityType.WuWuKuji] = (a, _, _, _) => a.WuWuKujiActivityInfo = new WuWuKujiActivityPb(),
        [ActivityType.InfrTheme] = (a, _, _, _) => a.InfrThemeActivityPb = new InfrThemeActivityPb
        {
            ActivityTaskData = new ActivityTaskData()
        },
        [ActivityType.AdvanceNoticeActivity] = (a, _, _, _) => a.AdvertisingPageInfo = new AdvertisingPageData(),
        [ActivityType.PhantomBattleRecord] = (a, _, farFuture, _) => a.PhantomBattleRecordActivityInfo = new PhantomBattleActivityInfo
        {
            PhantomBattleMasterInfo = new PhantomBattleMasterInfo(),
            TimeLimitShopEndTime = farFuture
        },
        [ActivityType.CoopActivity] = (a, _, _, _) => a.RoleCoopActivityData = new RoleCoopActivityData(),
        [ActivityType.PhantomBattleRecordGuide] = (a, _, _, _) => a.PhantomBattleGuideActivity = new PhantomBattleGuideActivity(),
        [ActivityType.ArtemisActivity] = (a, _, _, _) => a.ArtemisActivityInfo = new ArtemisActivityInfo(),
        [ActivityType.MotorcycleIpLink] = (a, _, _, _) => a.MotorCycleIpActivityData = new MotorCycleIpActivityData(),
        [ActivityType.SkinRewardActivity] = (a, _, _, _) => a.SkinRewardActivityData = new SkinRewardActivityData(),
        [ActivityType.RoadBookActivity] = (a, _, _, _) => a.RoadBookActivityInfo = new RoadBookActivityInfo(),
        [ActivityType.NewTowerClimbing] = (a, _, farFuture, _) => a.NewTowerClimbingActivityData = new NewTowerClimbingActivityData
        {
            CycleId = 1,
            CycleBeginTime = 1,
            CycleCloseTime = farFuture,
            SeasonId = 1,
            SeasonBeginTime = 1,
            SeasonCloseTime = farFuture
        },
        [ActivityType.MotorFight] = (a, _, _, _) => a.MotorFightActivityPb = new MotorFightActivityPb
        {
            TalentTree = new MotorFightTalentTreePb()
        },
        [ActivityType.Encircle] = (a, _, _, _) => a.EncircleActivityPb = new EncircleActivityPb(),
        [ActivityType.NewPlayerSupportActivity] = (a, _, _, _) => a.NewPlayerSupportActivityData = new NewPlayerSupportActivityData(),
        [ActivityType.SpringFestivalActivity] = (a, _, _, _) => a.SpringFestivalActivityInfo = new SpringFestivalActivityInfo
        {
            DrinkMixData = new DrinkMixData()
        },
        [ActivityType.MotorParkourActivity] = (a, _, _, _) => a.MotorParkourActivityInfo = new MotorParkourActivityInfo(),
        [ActivityType.TotalTopUp] = (a, _, _, _) => a.TotalTopUpActivityInfo = new TotalTopUpActivityInfo(),
        [ActivityType.H5View] = (a, _, _, _) => a.H5ViewActivityData = new H5ViewActivityData(),
        [ActivityType.MotorDevelop] = (a, _, _, _) => a.MotorDevelopActivityData = new MotorDevelopActivityData(),
        [ActivityType.FlagChallenge] = (a, _, _, _) => a.FlagChallengeActivityInfo = new FlagChallengeActivityInfo
        {
            FlagChallengeRoleLevelInfo = new FlagChallengeRoleLevelInfo()
        },
        [ActivityType.Rhythm] = (a, _, _, _) => a.RhythmActivityPb = new RhythmActivityPb
        {
            RedDot = new RhythmRedDotPb()
        },
        [ActivityType.FeiXue] = (a, _, _, _) => a.FeiXuePreheatActivityInfo = new FeiXuePreheatActivityInfo(),
        [ActivityType.DropCatchActivity] = (a, _, _, _) => a.DropCatchActivityInfo = new DropCatchActivityInfo(),
        [ActivityType.TetrisActivity] = (a, _, _, _) => a.TetrisActivityInfo = new TetrisActivityInfo(),
        [ActivityType.Kurotato] = (a, _, _, _) => a.KurotatoActivityData = new KurotatoActivityData(),
        [ActivityType.PinballActivity] = (a, _, _, _) => a.PinballActivityData = new PinballActivityData
        {
            Weapons = new PinballWeapons(),
            Roles = new PinballRoles()
        },
        [ActivityType.WuWuWeekSign] = (a, _, _, _) => a.WuWuWeekActivity = new WuWuWeekActivity(),
        [ActivityType.AnniversaryTheme] = (a, _, _, _) => a.ThemeCelebration = new ThemeCelebration(),
        [ActivityType.BossPiling] = (a, _, _, _) => a.BossPilingActivityInfo = new BossPilingActivityInfo(),
        [ActivityType.GolemCrackActivity] = (a, _, _, _) => a.GolemCrackActivityInfo = new GolemCrackActivityInfo(),
        [ActivityType.EdgeRunnerActivity] = (a, _, _, _) => a.EdgeRunnerActivityInfo = new EdgeRunnerActivityInfo(),
        [ActivityType.MotorDecalActivity] = (a, _, _, _) => a.MotorDecalActivityData = new MotorDecalActivityData(),
        [ActivityType.MotorParkour] = (a, _, _, _) => a.OnlineMotorActivityData = new OnlineMotorActivityData(),
        [ActivityType.LinkageCheckIn] = (a, _, _, _) => a.LinkageCheckInActivityData = new LinkageCheckInActivityData(),
        [ActivityType.RoleGiftActivity] = (a, _, _, _) => a.RoleGiftActivityData = new RoleGiftActivityData(),
        [ActivityType.RealmBetween] = (a, _, _, _) => a.RealmBetweenActivityInfo = new RealmBetweenActivityInfo(),
        [ActivityType.ThroughTrainSummary] = (a, _, _, _) => a.ThroughTrainSummaryActivityData = new ThroughTrainSummaryActivityData(),
        [ActivityType.NewPlayerSupportActivityV2] = (a, _, _, _) => a.NewPlayerSupportActivityV2Pb = new NewPlayerSupportActivityV2Pb(),
        [ActivityType.NewbieAdventureV2] = (a, _, _, _) => a.NewbieAdventureV2Pb = new NewbieAdventureV2Pb(),
        [ActivityType.NewbieCourseV2] = (a, _, _, _) => a.NewbieCourseV2ActivityPb = new NewbieCourseV2ActivityPb(),
        [ActivityType.NewbieMain] = (a, _, _, _) => a.NewbieMainActivityPb = new NewbieMainActivityPb(),
        [ActivityType.RoverRogue] = (a, id, _, _) => a.RoverRogueActivityData = new RoverRogueActivityData
        {
            ActivityId = id,
            HistoryInsInfo = new RoverRogueHistoryInsInfo()
        },
        [ActivityType.DangoBrawl] = (a, _, _, _) => a.DangoBrawlActivityInfo = new DangoBrawlActivityInfo
        {
            SaveMatchBrief = new DangoBrawlSaveMatchBrief()
        },
        [ActivityType.QingXiaoPlayThePiano] = (a, _, _, _) => a.QingXiaoActivityInfo = new QingXiaoActivityInfo(),
        [ActivityType.MatchThreeActivity] = (a, _, _, _) => a.MatchThreeActivityInfo = new MatchThreeActivityInfo(),
        [ActivityType.SolarSchool] = (a, _, _, _) => a.SolarSchoolActivityPb = new SolarSchoolActivityPb(),
        [ActivityType.GiantSystemActivity] = (a, _, _, _) => a.GiantActivityInfo = new GiantActivityInfo { TotalStableValue = 100 },
        [ActivityType.RoleTower] = (a, _, farFuture, _) => a.RoleTowerActivityData = new RoleTowerActivityData { RankRewardSettleTime = farFuture },
    };

    private static void PopulateIntClaimSuffixes(HashSet<string> claims, string prefix, IList<int> target)
    {
        foreach (var c in claims)
        {
            if (c.StartsWith(prefix, StringComparison.Ordinal) &&
                int.TryParse(c.AsSpan(prefix.Length), out int val) &&
                val > 0 &&
                !target.Contains(val))
            {
                target.Add(val);
            }
        }
    }

    static ActivityHandler()
    {
        LoadConfigs();
    }

    private static void LoadConfigs()
    {
        try
        {
            if (File.Exists(BinDataPath))
            {
                using var fs = File.OpenRead(BinDataPath);
                using var doc = JsonDocument.Parse(fs);
                foreach (var el in doc.RootElement.EnumerateArray())
                {
                    int id = el.GetProperty("Id").GetInt32();
                    int type = el.GetProperty("Type").GetInt32();
                    int openType = el.TryGetProperty("OpenType", out var ot) ? ot.GetInt32() : 0;
                    CachedConfig.Add(new ActivityConfigItem { Id = id, Type = type, OpenType = openType });
                }
                Console.WriteLine($"[ActivityHandler] Loaded {CachedConfig.Count} activity definitions from BinData.");
            }
            else
            {
                Console.WriteLine($"[ActivityHandler] Warning: {BinDataPath} not found.");
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine($"[ActivityHandler] Error loading Activity.json: {ex.Message}");
        }
    }

    private static void BuildActivities(int playerId, IList<ActivityData> target)
    {
        var claims = playerId > 0 ? SqliteStore.GetActivityClaims(playerId) : new HashSet<string>();
        var (progInts, progClaims) = playerId > 0
            ? SqliteStore.GetProgressState(playerId)
            : (new Dictionary<string, long>(), new HashSet<string>());
        foreach (var pc in progClaims)
        {
            claims.Add(pc);
        }

        long farFuture = 4070908800; // Year 2099

        foreach (var def in CachedConfig)
        {
            if (ExcludedTypes.Contains(def.Type)) continue;
            if (HasMissingBuildAsset(def)) continue;

            var actType = (ActivityType)def.Type;
            var act = new ActivityData
            {
                Id = def.Id,
                Type = actType,
                BeginShowTime = 1,
                EndShowTime = farFuture,
                BeginOpenTime = 1,
                EndOpenTime = farFuture,
                LimitBeginTime = 1,
                LimitEndTime = farFuture,
                LimitRewardBeginTime = 1,
                LimitRewardEndTime = farFuture,
                ActivityOpenType = ActivityOpenType.Permanent,
                IsUnlock = true,
                IsFirstOpen = false,
                CanPreOpen = false
            };

            if (SubPayloadPopulators.TryGetValue(actType, out var populator))
            {
                populator(act, def.Id, farFuture, claims);
                ApplyProgressState(act, actType, progInts);
            }

            target.Add(act);
        }
    }

    private static void ApplyProgressState(ActivityData act, ActivityType actType, Dictionary<string, long> ints)
    {
        switch (actType)
        {
            case ActivityType.ConsumptiveActivity when act.ConsumptiveActivityInfo != null:
                act.ConsumptiveActivityInfo.TotalScore = (int)ints.GetValueOrDefault("cons:score");
                break;
            case ActivityType.TotalTopUp when act.TotalTopUpActivityInfo != null:
                act.TotalTopUpActivityInfo.Score = (int)ints.GetValueOrDefault("topup:score");
                break;
            case ActivityType.RoleTower when act.RoleTowerActivityData != null:
                act.RoleTowerActivityData.Score = (int)ints.GetValueOrDefault("role_tower:score");
                break;
            case ActivityType.RogueWeekly when act.RogueWeeklyData != null:
                act.RogueWeeklyData.CycleId = (int)Math.Max(1, ints.GetValueOrDefault("rogue_weekly:cycle"));
                act.RogueWeeklyData.UseFreeCount = (int)ints.GetValueOrDefault("rogue_weekly:bf_used");
                break;
        }
    }

    public static AkiResult HandleActivityRequest(int playerId, byte[] payload)
    {
        var req = new ActivityRequest();
        if (payload.Length > 0)
        {
            req.MergeFrom(payload);
        }

        var resp = new ActivityResponse { ErrorCode = 0 };
        BuildActivities(playerId, resp.Activities);

        Console.WriteLine($"[ActivityHandler] Prepared ActivityResponse with {resp.Activities.Count} activities for player {playerId}.");
        return AkiResult.Ok(EMessageId.ActivityResponse, resp);
    }

    public static AkiResult HandleActivityUpdateNotify(int playerId, byte[] payload)
    {
        var notify = new ActivityUpdateNotify();
        BuildActivities(playerId, notify.Activities);

        Console.WriteLine($"[ActivityHandler] Prepared ActivityUpdateNotify with {notify.Activities.Count} activities for player {playerId}.");
        return AkiResult.Ok(EMessageId.ActivityUpdateNotify, notify);
    }

    public static AkiResult HandleTowerGuideInfo(int playerId, byte[] payload)
    {
        return AkiResult.Ok(EMessageId.TowerGuideActivityInfoResponse, new TowerGuideActivityInfoResponse());
    }

    public static AkiResult HandleTowerGuideReward(int playerId, byte[] payload)
    {
        var result = AkiResult.Ok(EMessageId.TowerGuideActivityRewardResponse, new TowerGuideActivityRewardResponse
        {
            ErrorCode = ErrorCode.Success
        });
        if (playerId > 0)
        {
            var claims = SqliteStore.GetActivityClaims(playerId);
            if (!claims.Contains("tower_guide:claimed"))
            {
                SqliteStore.SetActivityClaim(playerId, "tower_guide:claimed", "1");
                result.WithGrant(3, 60).WithGrant(2, 10000);
            }
        }
        return result;
    }
}
