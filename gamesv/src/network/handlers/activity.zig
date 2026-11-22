const std = @import("std");
const proto = @import("proto");
const pb = proto.pb;
const pb_desc = proto.pb_desc;
const mem = @import("../../mem.zig");
const Transaction = @import("../handlers.zig").Transaction;
const Connection = @import("../Connection.zig");
const Assets = @import("../../data/Assets.zig");
const EnergyHelper = @import("../../logic/helpers/energy.zig");
const PlayerID = @import("../../logic/PlayerID.zig");
const PlayerBasicComponent = @import("../../logic/component/player/PlayerBasicComponent.zig");
const PlayerEchoComponent = @import("../../logic/component/player/PlayerEchoComponent.zig");
const PlayerInventoryComponent = @import("../../logic/component/player/PlayerInventoryComponent.zig");
const PlayerMapComponent = @import("../../logic/component/player/PlayerMapComponent.zig");
const PlayerProgressComponent = @import("../../logic/component/player/PlayerProgressComponent.zig");
const PlayerQuestComponent = @import("../../logic/component/player/PlayerQuestComponent.zig");
const PlayerRoleComponent = @import("../../logic/component/player/PlayerRoleComponent.zig");
const ConditionEvaluator = @import("../../logic/helpers/condition.zig");
const aki_logic_bridge = @import("../aki_logic_bridge.zig");

const ActivityUnion = @typeInfo(@FieldType(pb.ActivityData, "Data")).optional.child;

pub fn isActivityUnlocked(
    cond_ctx_opt: ?ConditionEvaluator.EvalContext,
    assets: *const Assets,
    act_id: i32,
) bool {
    const cond_ctx = cond_ctx_opt orelse return true;
    if (!ConditionEvaluator.isFunctionOpen(cond_ctx, 10053)) return false;
    const def = assets.tables.activity.getDataById(act_id) orelse return false;
    if (def.PreConditionGroupId <= 0) return true;
    return ConditionEvaluator.evalConditionGroup(cond_ctx, def.PreConditionGroupId);
}

pub fn buildActivityEntry(
    id: i32,
    raw_type: i32,
    raw_open_type: i32,
    now_s: i64,
    is_unlock: bool,
    data: @FieldType(pb.ActivityData, "Data"),
) ?pb.ActivityData {
    const act_type = std.enums.fromInt(pb.ActivityType, raw_type) orelse return null;
    _ = raw_open_type;
    const open_type: pb.ActivityOpenType = .Permanent;
    const begin_s = @max(@as(i64, 1), now_s - 86_400);
    const end_s = begin_s + 30 * 86_400;

    return .{
        .Id = id,
        .Type = act_type,
        .BeginShowTime = begin_s,
        .EndShowTime = end_s,
        .BeginOpenTime = begin_s,
        .EndOpenTime = end_s,
        .IsUnlock = is_unlock,
        .IsFirstOpen = false,
        .ActivityOpenType = open_type,
        .IsPreOpen = false,
        .StartTime = begin_s,
        .EndTime = end_s,
        .BeginRewardTimeInternal = begin_s,
        .EndRewardTimeInternal = end_s,
        .Data = data,
    };
}

fn defaultDataForType(
    act_type: pb.ActivityType,
    id: i32,
    now_s: i64,
    progress_comp: *const PlayerProgressComponent,
) ?ActivityUnion {
    const end_s = now_s + 30 * 86_400;
    return switch (act_type) {
        .Parkour => .{ .ParkourActivity = .{} },
        .GatherActivity => .{ .GatherActivityInfo = .{} },
        .Sign => .{ .SignActivity = .{} },
        .TowerGuide, .TowerGuideNew => null,
        .NewBieCourse => .{ .NewBieCourseActivity = .{} },
        .RougeActivity => .{ .ActivityRogueData = .{
            .BeginOpenTime = 1,
            .EndOpenTime = end_s,
            .RoguelikeSeason = .{
                .SeasonId = rogueSeasonIdForActivity(id),
                .EndTime = end_s,
            },
        } },
        .DoubleInstanceRewardActivity => .{ .DoubleInstActivityReward = .{} },
        .RoleTrialActivity => .{ .RoleTrialInfoActivity = .{} },
        .Harvest => .{ .HarvestActivity = .{} },
        .PhantomCollect => .{ .PhantomCollectActivity = .{} },
        .DailyAdventureActivity => .{ .DailyAdventureActivityData = .{} },
        .LongShanMainActivity => .{ .ActivityLongShanMain = .{} },
        .BossRushActivity => .{ .BossRushActivityData = .{} },
        .TurnTableActivity => .{ .ActivityTurnTableData = .{} },
        .TrackMoonActivity => .{ .TrackMoonActivityTaskData = .{} },
        .CircumFluence => null,
        .TowerDefenceActivity => .{ .TowerDefenseActivityInfo = .{} },
        .TimePointRewardActivity => .{ .ActivityTimePointRewarData = .{} },
        .TrackMoonPhase => null,
        .RiskHarvest => .{ .RiskHarvestActivityData = .{} },
        .CorniceMeeting => .{ .ActivityCorniceMeetingData = .{} },
        .BlackCoastTheme => .{ .ActivityBlackCoastData = .{} },
        .RogueWhiteCat => .{ .DreamLinkActivityData = .{} },
        .ScratchCard => .{ .ActivityScratchTicketData = .{} },
        .PreheatSign => .{ .PreheatSignActivityData = .{} },
        .MowTower => .{ .MowTowerActivityData = .{} },
        .ThroughTrain => .{ .ThroughTrainActivityData = .{} },
        .SprintSign => .{ .SpringSignData = .{} },
        .MapTravelActivity => .{ .MapTravelActivityData = .{} },
        .FarmGold => .{ .FarmGoldData = .{} },
        .RoleSkinTrialActivity => .{ .RoleSkinTrialActivity = .{} },
        .RogueWeekly => .{ .ActivityWeeklyRogueData = .{
            .CycleId = @intCast(@max(1, progress_comp.info.getKeyInt("rogue_weekly:cycle"))),
            .MaxFreeCount = 3,
            .UseFreeCount = @intCast(progress_comp.info.getKeyInt("rogue_weekly:bf_used")),
        } },
        .FishingActivity => .{ .ActivityFishingData = .{} },
        .TeamParkOurActivity => .{ .SolarisSpeedActivity = .{} },
        .BabelTower => .{ .BabelTowerActivity = .{} },
        .BetHorses => .{ .ActivityBetHorsesData = .{ .ActivityId = id } },
        .Explore => .{ .ActivityMapExploreData = .{} },
        .Abyss => .{ .DangoAbyssActivityData = .{ .StartTime = 1, .EndTime = end_s * 1000 } },
        .RogueRes => .{ .ActivityPermanentRogueData = .{} },
        .DangoMonopoly => .{ .ActivityDangoMonopolyData = .{ .CurrentBoardId = 1 } },
        .CiacconaActivity => .{ .ActivityCiacconaGalData = .{} },
        .Avignon => .{ .ActivityAvignon = .{} },
        .H5CircumFluence => .{ .ActivityInviteNewbie = .{} },
        .H5View => .{ .H5ViewActivityData = .{} },
        .ActivityLinkage => .{ .ActivityLinkageData = .{ .ActivityId = id } },
        .RegressActivity => .{ .ActivityRegressData = .{ .EndTime = end_s, .Grade = 1 } },
        .ConsumptiveActivity => .{ .CumulativeShopData = .{
            .ActivityId = id,
            .TotalScore = @intCast(progress_comp.info.getKeyInt("cons:score")),
        } },
        .PhantomBattle => .{ .PhantomArenaActivityData = .{} },
        .PhantomBattleRecord => .{ .PhantomBattleRecordActivityInfo = .{} },
        .PhantomBattleRecordGuide => .{ .PhantomBattleGuideActivity = .{} },
        .AdvanceNoticeActivity => .{ .AdvertisingPageInfo = .{} },
        .ArtemisActivity => .{ .ArtemisActivityInfo = .{} },
        .NewbieCarnival => .{ .BeginnerCarnivalData = .{} },
        .MoraleActivity => .{ .ActivityMoraleData = .{} },
        .FloroRanchActivity => .{ .FloroRanchActivityData = .{} },
        .LifePointChallenge => .{ .LifePointDrawActivityData = .{} },
        .TrapDefense => .{ .ActivityTrapDefenseData = .{} },
        .FunPlay => .{ .ActivityFunPlayData = .{} },
        .MoonPhase => .{ .ActivityMoonSignInData = .{} },
        .LineCross => .{ .ActivityLineCrossData = .{} },
        .HonamiStory => .{ .HonamiStoryActivityData = .{} },
        .Survivors => .{ .SurvivorsActivityData = .{} },
        .PhotoFight => .{ .FightPhotoActivityData = .{} },
        .WuWuKuji => .{ .ActivityPrizeDrawingData = .{} },
        .InfrTheme => .{ .InfrThemeActivityPb = .{} },
        .CoopActivity => .{ .RoleCoopActivityData = .{} },
        .MotorcycleIpLink => .{ .MotorCycleIpActivityData = .{} },
        .SkinRewardActivity => .{ .SkinRewardActivityData = .{} },
        .RoadBookActivity => .{ .RoadBookActivityInfo = .{} },
        .NewTowerClimbing => .{ .NewTowerClimbingActivityData = .{
            .CycleId = 1,
            .SeasonId = 1,
            .CycleBeginTime = 1,
            .CycleCloseTime = end_s,
            .SeasonBeginTime = 1,
        } },
        .MotorFight => .{ .MotorFightActivityPb = .{ .TalentTree = .{} } },
        .Encircle => .{ .EncircleActivityPb = .{} },
        .NewPlayerSupportActivity => .{ .NewPlayerSupportActivityData = .{} },
        .SpringFestivalActivity => .{ .SpringFestivalActivityInfo = .{} },
        .MotorParkourActivity => .{ .MotorParkourActivityInfo = .{} },
        .TotalTopUp => .{ .TotalTopUpActivityInfo = .{
            .Score = @intCast(progress_comp.info.getKeyInt("topup:score")),
        } },
        .MotorDevelop => .{ .MotorDevelopActivityData = .{} },
        .FlagChallenge => .{ .FlagChallengeActivityInfo = .{} },
        .Rhythm => .{ .RhythmActivityPb = .{} },
        .FeiXue => .{ .FeiXuePreheatActivityInfo = .{} },
        .DropCatchActivity => .{ .DropCatchActivityInfo = .{} },
        .TetrisActivity => .{ .TetrisActivityInfo = .{} },
        .Kurotato => .{ .KurotatoActivityData = .{} },
        .PinballActivity => .{ .PinballActivityData = .{} },
        .WuWuWeekSign => .{ .WuWuWeekActivity = .{} },
        .AnniversaryTheme => .{ .ThemeCelebration = .{} },
        .BossPiling => .{ .BossPilingActivityInfo = .{} },
        .GolemCrackActivity => .{ .GolemCrackActivityInfo = .{} },
        .EdgeRunnerActivity => .{ .EdgeRunnerActivityInfo = .{} },
        .MotorDecalActivity => .{ .MotorDecalActivityData = .{} },
        .MotorParkour => .{ .OnlineMotorActivityData = .{} },
        .LinkageCheckIn => .{ .LinkageCheckInActivityData = .{} },
        .RoleGiftActivity => .{ .RoleGiftActivityData = .{} },
        .RealmBetween => .{ .RealmBetweenActivityInfo = .{} },
        .ThroughTrainSummary => .{ .ThroughTrainSummaryActivityData = .{} },
        .NewPlayerSupportActivityV2 => .{ .NewPlayerSupportActivityV2Pb = .{} },
        .NewbieAdventureV2 => .{ .NewbieAdventureV2Pb = .{} },
        .NewbieCourseV2 => .{ .NewbieCourseV2ActivityPb = .{} },
        .NewbieMain => .{ .NewbieMainActivityPb = .{} },
        .RoverRogue => .{ .RoverRogueActivityData = .{
            .activityId = id,
            .HistoryInsInfo = .{},
        } },
        .DangoBrawl => .{ .DangoBrawlActivityInfo = .{} },
        .QingXiaoPlayThePiano => .{ .QingXiaoActivityInfo = .{} },
        .MatchThreeActivity => .{ .MatchThreeActivityInfo = .{} },
        .SolarSchool => .{ .SolarSchoolActivityPb = .{} },
        .GiantSystemActivity => .{ .GiantActivityInfo = .{ .TotalStableValue = giantTotalStableValue(progress_comp) } },
        .RoleTower => .{ .RoleTowerActivityData = .{
            .RankRewardSettleTime = end_s,
            .Score = @intCast(progress_comp.info.getKeyInt("role_tower:score")),
        } },
        else => null,
    };
}

fn rogueSeasonIdForActivity(activity_id: i32) i32 {
    return switch (activity_id) {
        100600001 => 1,
        100600002 => 2,
        100600003 => 3,
        100600004 => 4,
        102600001 => 5,
        100600005 => 6,
        100600006 => 7,
        else => 1,
    };
}

fn giantTotalStableValue(progress_comp: *const PlayerProgressComponent) i32 {
    var total: i64 = 0;
    for (0..3) |eco_type| {
        var key_buf: [48]u8 = undefined;
        const key = std.fmt.bufPrint(&key_buf, "giant:stable:{d}", .{eco_type}) catch continue;
        total += @max(0, progress_comp.info.getKeyInt(key));
    }
    return @intCast(@min(total, std.math.maxInt(i32)));
}

fn supportsBaseOnly(act_type: pb.ActivityType) bool {
    return switch (act_type) {
        .WorldNewJourney,
        .NewRoleGuideActivity,
        .PhotoMemoryActivity,
        .NewLordGym,
        .SlashAndTowerLevelPlay,
        .PreHeatTaskActivity,
        .InfrThemeV2,
        .SheriffActivity,
        .PurePreviewActivity,
        .TowerGuide,
        .TowerGuideNew,
        .FirstPersonParkour,
        .PureUiactivity,
        => true,
        else => false,
    };
}

fn appendIntClaimSuffixes(
    arena: std.mem.Allocator,
    claims: []const []const u8,
    prefix: []const u8,
    target: *std.ArrayList(i32),
) !void {
    for (claims) |c| {
        if (!std.mem.startsWith(u8, c, prefix)) continue;
        const parsed = std.fmt.parseInt(i32, c[prefix.len..], 10) catch continue;
        if (parsed <= 0) continue;
        var dup = false;
        for (target.items) |existing| {
            if (existing == parsed) {
                dup = true;
                break;
            }
        }
        if (!dup) try target.append(arena, parsed);
    }
}

pub fn buildActivityList(
    arena: std.mem.Allocator,
    gpa: std.mem.Allocator,
    assets: *const Assets,
    progress_comp: *const PlayerProgressComponent,
    cond_ctx: ?ConditionEvaluator.EvalContext,
    io: std.Io,
) !std.ArrayList(pb.ActivityData) {
    _ = gpa;
    const now_s = EnergyHelper.nowSeconds(io);
    var activities: std.ArrayList(pb.ActivityData) = .empty;

    for (assets.tables.activity.items) |def| {
        // Exclude activity types that require complex unsupported server modules, lack client controllers,
        // or trigger recurring login splash banners / NREs (30,50,69,80,97,104,107,108,111; 18=CircumFluence).
        switch (def.Type) {
            13, 18, 22, 30, 41, 42, 45, 50, 55, 57, 58, 61, 62, 69, 72, 76, 80, 82, 83, 91, 97, 103, 104, 107, 108, 111, 113, 117 => continue,
            else => {},
        }

        const act_type = std.enums.fromInt(pb.ActivityType, def.Type) orelse continue;

        var act_data: ?ActivityUnion = null;
        if (act_type == .Sign) {
            var sign_states: std.ArrayList(pb.SignState) = .empty;
            var prev_claimed = true;
            for (0..7) |day_idx| {
                const key = try std.fmt.allocPrint(arena, "sign_act:{d}:{d}", .{ def.Id, day_idx });
                const is_claimed = progress_comp.info.hasClaim(key);
                if (is_claimed) {
                    try sign_states.append(arena, .IsReceive);
                } else if (prev_claimed) {
                    try sign_states.append(arena, .Unlock);
                    prev_claimed = false;
                } else {
                    try sign_states.append(arena, .Lock);
                }
            }
            const free_key = try std.fmt.allocPrint(arena, "sign_free:{d}", .{def.Id});
            act_data = .{
                .SignActivity = .{
                    .SignStateList = sign_states,
                    .RewardFree = progress_comp.info.hasClaim(free_key),
                },
            };
        } else if (act_type == .NewBieCourse) {
            var claimed: std.ArrayList(i32) = .empty;
            const claims = progress_comp.info.dynamic.claims;
            try appendIntClaimSuffixes(arena, claims, "region_prog:NewBieCourseRewardRequest:", &claimed);
            try appendIntClaimSuffixes(arena, claims, "newbie:course:", &claimed);
            act_data = .{
                .NewBieCourseActivity = .{
                    .HadTakeReward = claimed,
                },
            };
        } else if (act_type == .DoubleInstanceRewardActivity) {
            act_data = .{
                .DoubleInstActivityReward = .{},
            };
        } else if (act_type == .RoleTrialActivity) {
            act_data = .{
                .RoleTrialInfoActivity = .{
                    .RoleTrialTask = .empty,
                },
            };
        } else {
            act_data = defaultDataForType(act_type, def.Id, now_s, progress_comp);
        }

        if (act_data == null and !supportsBaseOnly(act_type)) {
            continue;
        }

        const unlocked = isActivityUnlocked(cond_ctx, assets, def.Id);
        if (!unlocked) continue;
        const entry = buildActivityEntry(
            def.Id,
            def.Type,
            def.OpenType,
            now_s,
            true,
            act_data,
        ) orelse continue;
        try activities.append(arena, entry);
    }
    return activities;
}

pub fn pushActivityUpdateNotify(
    conn: *Connection,
    alloc: mem.Alloc,
    assets: *const Assets,
    progress_comp: *const PlayerProgressComponent,
    cond_ctx: ConditionEvaluator.EvalContext,
    player_id: i32,
    io: std.Io,
) void {
    if (aki_logic_bridge.callPlayer(io, alloc.arena, player_id, 0, pb_desc.ActivityUpdateNotify.msg_id, &.{})) |resp| {
        if (resp.msg_id == pb_desc.ActivityUpdateNotify.msg_id and resp.payload.len > 0) {
            var reader = std.Io.Reader.fixed(resp.payload);
            if (proto.decodeMessage(&reader, alloc.arena, pb.ActivityUpdateNotify, pb_desc)) |decoded| {
                var filtered: std.ArrayList(pb.ActivityData) = .empty;
                for (decoded.Activities.items) |act| {
                    if (!isActivityUnlocked(cond_ctx, assets, act.Id)) continue;
                    var copy = act;
                    copy.IsUnlock = true;
                    filtered.append(alloc.arena, copy) catch |err| {
                        std.log.scoped(.activity).warn("failed to append filtered activity {d}: {t}", .{ act.Id, err });
                        break;
                    };
                }
                var mut_decoded = decoded;
                mut_decoded.Activities = filtered;
                conn.push(alloc.gpa, io, mut_decoded) catch |err| {
                    std.log.scoped(.activity).warn("failed to push patched ActivityUpdateNotify: {t}", .{err});
                };
                return;
            } else |_| {}
        }
    } else |_| {}

    if (buildActivityList(alloc.arena, alloc.gpa, assets, progress_comp, cond_ctx, io)) |acts| {
        conn.push(alloc.gpa, io, pb.ActivityUpdateNotify{ .Activities = acts }) catch |err| {
            std.log.scoped(.activity).warn("failed to push ActivityUpdateNotify: {t}", .{err});
        };
    } else |err| {
        std.log.scoped(.activity).warn("failed to build activities for ActivityUpdateNotify: {t}", .{err});
    }
}

pub fn onActivityRequest(
    txn: *Transaction(pb.ActivityRequest),
    assets: *const Assets,
    basic_comp: *const PlayerBasicComponent,
    echo_comp: *const PlayerEchoComponent,
    inventory_comp: *const PlayerInventoryComponent,
    map_comp: *const PlayerMapComponent,
    quest_comp: *const PlayerQuestComponent,
    role_comp: *const PlayerRoleComponent,
    progress_comp: *const PlayerProgressComponent,
    player_id: PlayerID,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    const cond_ctx: ConditionEvaluator.EvalContext = .{
        .assets = assets,
        .basic_comp = basic_comp,
        .echo_comp = echo_comp,
        .inventory_comp = inventory_comp,
        .map_comp = map_comp,
        .quest_comp = quest_comp,
        .role_comp = role_comp,
        .progress_comp = progress_comp,
    };

    // 1. Delegate to AkiLogicService (.NET 9 Logic Engine with Kuro's official models & BinData)
    if (aki_logic_bridge.callPlayer(io, alloc.arena, player_id.id, 0, 16076, txn.raw_payload)) |resp| {
        if (resp.msg_id == pb_desc.ActivityResponse.msg_id and resp.payload.len > 0) {
            var reader = std.Io.Reader.fixed(resp.payload);
            if (proto.decodeMessage(&reader, alloc.arena, pb.ActivityResponse, pb_desc)) |decoded| {
                var filtered: std.ArrayList(pb.ActivityData) = .empty;
                for (decoded.Activities.items) |act| {
                    if (!isActivityUnlocked(cond_ctx, assets, act.Id)) continue;
                    var copy = act;
                    copy.IsUnlock = true;
                    try filtered.append(alloc.arena, copy);
                }
                var mut_decoded = decoded;
                mut_decoded.Activities = filtered;
                txn.respond(mut_decoded);
                return;
            } else |_| {}
        }
        if (resp.msg_id != 0) {
            try txn.respondRaw(alloc.gpa, io, resp.msg_id, resp.payload);
            return;
        }
    } else |err| {
        std.log.warn("AkiLogicService bridge unreachable ({s}), falling back to native Zig activities", .{@errorName(err)});
    }

    // 2. Safe fallback: native Zig handler
    const activities = try buildActivityList(alloc.arena, alloc.gpa, assets, progress_comp, cond_ctx, io);
    txn.respond(.{
        .Activities = activities,
        .ErrorCode = .Success,
    });
}
