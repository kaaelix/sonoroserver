const std = @import("std");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const Transaction = @import("../handlers.zig").Transaction;
const FileSystem = @import("common").FileSystem;
const Assets = @import("../../data/Assets.zig");
const QuestInfo = @import("../../fs/QuestInfo.zig");
const special_item_incr = @import("../../fs/special_item_incr.zig");
const CalabashHelper = @import("../../logic/helpers/calabash.zig");
const RewardService = @import("../../logic/helpers/reward.zig");
const PlayerBasicComponent = @import("../../logic/component/player/PlayerBasicComponent.zig");
const PlayerRoleComponent = @import("../../logic/component/player/PlayerRoleComponent.zig");
const PlayerWeaponComponent = @import("../../logic/component/player/PlayerWeaponComponent.zig");
const PlayerEchoComponent = @import("../../logic/component/player/PlayerEchoComponent.zig");
const PlayerMapComponent = @import("../../logic/component/player/PlayerMapComponent.zig");
const PlayerQuestComponent = @import("../../logic/component/player/PlayerQuestComponent.zig");
const PlayerProgressComponent = @import("../../logic/component/player/PlayerProgressComponent.zig");

const AdventureSnapshot = struct {
    assets: *const Assets,
    basic_comp: *const PlayerBasicComponent,
    role_comp: *const PlayerRoleComponent,
    weapon_comp: *const PlayerWeaponComponent,
    echo_comp: *const PlayerEchoComponent,
    map_comp: *const PlayerMapComponent,
    quest_comp: *const PlayerQuestComponent,
    progress_comp: *const PlayerProgressComponent,
};

fn countRolesAtLeastLevel(snap: AdventureSnapshot, min_level: i32) i32 {
    var count: i32 = 0;
    var it = snap.role_comp.role_map.iterator();
    while (it.next()) |kv| {
        const role_id = kv.key_ptr.*;
        if (snap.role_comp.isTrialRole(role_id)) continue;
        if (role_id < 1000 or role_id >= 2000) continue;
        if (kv.value_ptr.level >= min_level) count += 1;
    }
    return count;
}

fn countWeaponsAtLeastLevel(snap: AdventureSnapshot, min_level: i32) i32 {
    var count: i32 = 0;
    var it = snap.weapon_comp.weapon_map.iterator();
    while (it.next()) |kv| {
        if (kv.key_ptr.* >= special_item_incr.trial_incr_min) continue;
        if (kv.value_ptr.level >= min_level) count += 1;
    }
    return count;
}

fn countWeaponsAtLeastQuality(snap: AdventureSnapshot, min_quality: i32) i32 {
    var count: i32 = 0;
    var it = snap.weapon_comp.weapon_map.iterator();
    while (it.next()) |kv| {
        if (kv.key_ptr.* >= special_item_incr.trial_incr_min) continue;
        const cfg = snap.assets.tables.weapon_conf.getDataById(kv.value_ptr.id) orelse continue;
        if (cfg.QualityId >= min_quality) count += 1;
    }
    return count;
}

fn countRolesWithSkillLevel(snap: AdventureSnapshot, min_skills: i32, min_level: i32) i32 {
    var qualified_roles: i32 = 0;
    var it = snap.role_comp.role_map.iterator();
    while (it.next()) |kv| {
        const role_id = kv.key_ptr.*;
        if (snap.role_comp.isTrialRole(role_id)) continue;
        if (role_id < 1000 or role_id >= 2000) continue;
        var skills_met: i32 = 0;
        for (kv.value_ptr.skills) |pair| {
            if (pair[1] >= min_level) skills_met += 1;
        }
        if (skills_met >= min_skills) qualified_roles += 1;
    }
    return qualified_roles;
}

fn countEquippedEchoes(snap: AdventureSnapshot, min_quality: i32, min_fetter_match: i32) i32 {
    var best_role_equipped: i32 = 0;
    var it = snap.role_comp.role_map.iterator();
    while (it.next()) |kv| {
        const role_id = kv.key_ptr.*;
        if (snap.role_comp.isTrialRole(role_id)) continue;
        if (role_id < 1000 or role_id >= 2000) continue;

        var equipped_count: i32 = 0;
        var fetter_ids: [5]i32 = .{ 0, 0, 0, 0, 0 };
        var fetter_counts: [5]i32 = .{ 0, 0, 0, 0, 0 };
        var fetter_len: usize = 0;
        var has_2pc = false;

        for (kv.value_ptr.phantom) |slot| {
            const incr_id = slot[1];
            if (incr_id <= 0) continue;
            const echo = snap.echo_comp.echo_map.get(incr_id) orelse continue;
            const cfg = snap.assets.tables.phantom_item.getDataById(echo.id) orelse continue;
            if (cfg.QualityId < min_quality) continue;
            equipped_count += 1;

            if (echo.fetter_group_id > 0) {
                var matched = false;
                for (0..fetter_len) |i| {
                    if (fetter_ids[i] == echo.fetter_group_id) {
                        fetter_counts[i] += 1;
                        if (fetter_counts[i] >= 2) has_2pc = true;
                        matched = true;
                        break;
                    }
                }
                if (!matched and fetter_len < fetter_ids.len) {
                    fetter_ids[fetter_len] = echo.fetter_group_id;
                    fetter_counts[fetter_len] = 1;
                    fetter_len += 1;
                }
            }
        }

        if (min_fetter_match > 0) {
            if (has_2pc and equipped_count > best_role_equipped) {
                best_role_equipped = equipped_count;
            }
        } else if (equipped_count > best_role_equipped) {
            best_role_equipped = equipped_count;
        }
    }
    return best_role_equipped;
}

fn countEchoes(snap: AdventureSnapshot, min_level: i32, min_quality: i32) i32 {
    var count: i32 = 0;
    var it = snap.echo_comp.echo_map.iterator();
    while (it.next()) |kv| {
        if (kv.key_ptr.* >= special_item_incr.trial_incr_min) continue;
        const echo = kv.value_ptr;
        if (echo.level < min_level) continue;
        const cfg = snap.assets.tables.phantom_item.getDataById(echo.id) orelse continue;
        if (cfg.QualityId >= min_quality) count += 1;
    }
    return count;
}

fn countTunedEchoes(snap: AdventureSnapshot) i32 {
    var count: i32 = 0;
    var it = snap.echo_comp.echo_map.iterator();
    while (it.next()) |kv| {
        if (kv.key_ptr.* >= special_item_incr.trial_incr_min) continue;
        if (kv.value_ptr.sub_prop.len > 0) count += 1;
    }
    return count;
}

fn countDiscoveredEchoes(snap: AdventureSnapshot, min_quality: i32) i32 {
    var count: i32 = 0;
    for (snap.assets.tables.calabash_develop_reward.items) |reward| {
        if (!reward.IsShow) continue;
        if (CalabashHelper.discoveredMaxQuality(snap.assets, snap.echo_comp, reward.MonsterId) >= min_quality) {
            count += 1;
        }
    }
    return count;
}

fn hasTeleporter(snap: AdventureSnapshot, mark_id: i32) bool {
    return QuestInfo.containsInt(snap.map_comp.info.unlocked_teleporters, mark_id);
}

fn countTeleportersByObjectType(snap: AdventureSnapshot, target_type: i32) i32 {
    var count: i32 = 0;
    for (snap.map_comp.info.unlocked_teleporters) |tp_id| {
        const mark = snap.assets.tables.map_mark.getDataById(tp_id) orelse continue;
        if (mark.ObjectType == target_type) count += 1;
    }
    return count;
}

fn hasCompletedMarkRelative(snap: AdventureSnapshot, mark_id: i32) bool {
    const mark = snap.assets.tables.map_mark.getDataById(mark_id) orelse return false;
    if (mark.RelativeId <= 0) return false;
    return QuestInfo.containsInt(snap.quest_comp.info.completed_level_plays, mark.RelativeId);
}

fn countTowerStars(snap: AdventureSnapshot, min_difficulty: i32) i32 {
    var total: i32 = 0;
    for (snap.progress_comp.info.challenge.tower_floors) |fl| {
        if (fl.difficulty >= min_difficulty) total += fl.star;
    }
    return total;
}

fn evaluateTaskProgress(snap: AdventureSnapshot, task_id: i32) i32 {
    const quest_info = &snap.quest_comp.info;
    return switch (task_id) {
        // Chapter 1
        10010001 => countRolesAtLeastLevel(snap, 20),
        10010002 => countWeaponsAtLeastQuality(snap, 3),
        10010003 => countWeaponsAtLeastLevel(snap, 20),
        10010004 => @intFromBool(hasTeleporter(snap, 300404)),
        10010005 => @intFromBool(quest_info.hasFinishedQuest(139000029)),
        10010007 => @min(@as(i32, 1), @divFloor(countEquippedEchoes(snap, 0, 0), 3)),

        // Chapter 2
        10020001 => countRolesAtLeastLevel(snap, 30),
        10020002 => countWeaponsAtLeastLevel(snap, 30),
        10020003 => @intFromBool(hasTeleporter(snap, 300601)),
        10020004 => @intFromBool(quest_info.hasFinishedQuest(139000030)),
        10020005 => @intFromBool(quest_info.opened_chests.len > 0 or countEchoes(snap, 1, 1) > 0),
        10020006 => countTeleportersByObjectType(snap, 5),
        10020007 => countDiscoveredEchoes(snap, 2),
        10020009 => @intFromBool(hasCompletedMarkRelative(snap, 5021) or quest_info.completed_level_plays.len > 0),

        // Chapter 3
        10030001 => countRolesAtLeastLevel(snap, 30),
        10030002 => @intFromBool(hasTeleporter(snap, 3005)),
        10030003 => countEchoes(snap, 10, 1),
        10030004 => @intFromBool(hasTeleporter(snap, 300901)),
        10030005 => @intFromBool(quest_info.hasFinishedQuest(139000031)),
        10030006 => countTeleportersByObjectType(snap, 8),
        10030007 => @intCast(quest_info.completed_level_plays.len),
        10030008 => @intFromBool(quest_info.completed_dungeons.len > 0),
        10030009 => @intFromBool(countEquippedEchoes(snap, 1, 1) >= 2),

        // Chapter 4
        10040001 => countRolesAtLeastLevel(snap, 50),
        10040002 => countWeaponsAtLeastLevel(snap, 50),
        10040003 => countEchoes(snap, 10, 1),
        10040004 => @intFromBool(hasTeleporter(snap, 300701)),
        10040005 => countEchoes(snap, 0, 1),
        10040006 => @intFromBool(hasCompletedMarkRelative(snap, 5001) or quest_info.completed_level_plays.len > 0),
        10040007 => @intCast(quest_info.completed_dungeons.len),
        10040008 => @intFromBool(hasTeleporter(snap, 3003)),
        10040009 => countDiscoveredEchoes(snap, 3),
        10040010 => countRolesWithSkillLevel(snap, 1, 3),

        // Chapter 5
        10050001 => countRolesAtLeastLevel(snap, 50),
        10050002 => countWeaponsAtLeastLevel(snap, 50),
        10050003 => countEchoes(snap, 15, 1),
        10050004 => countRolesWithSkillLevel(snap, 2, 3),
        10050005 => @intFromBool(snap.basic_comp.info.attributes.cur_world_level >= 2 or snap.map_comp.info.unlocked_fogs.len >= 3),
        10050006 => countDiscoveredEchoes(snap, 4),
        10050007 => @intFromBool(hasCompletedMarkRelative(snap, 1101) or quest_info.completed_level_plays.len >= 2),
        10050008 => @intFromBool(quest_info.completed_dungeons.len >= 2),
        10050009 => countEchoes(snap, 0, 4),

        // Chapter 6
        10060001 => countRolesAtLeastLevel(snap, 60),
        10060002 => countWeaponsAtLeastLevel(snap, 60),
        10060003 => countEchoes(snap, 20, 1),
        10060004 => countEchoes(snap, 0, 4),
        10060005 => countDiscoveredEchoes(snap, 4),
        10060006 => @intFromBool(snap.basic_comp.info.attributes.level >= 30),
        10060007 => @intCast(quest_info.completed_level_plays.len),
        10060008 => countRolesWithSkillLevel(snap, 2, 3),
        10060009 => @intFromBool(hasCompletedMarkRelative(snap, 1102) or quest_info.completed_level_plays.len >= 3),

        // Chapter 7
        10070001 => countRolesAtLeastLevel(snap, 70),
        10070002 => countEchoes(snap, 25, 1),
        10070003 => countEchoes(snap, 0, 5),
        10070004 => @intFromBool(quest_info.hasFinishedQuest(125000124)),
        10070005 => CalabashHelper.currentLevel(snap.assets, snap.basic_comp, snap.echo_comp),
        10070006 => @intFromBool(quest_info.opened_chests.len >= 10 or snap.map_comp.info.unlocked_fogs.len >= 6),
        10070008 => @intFromBool(hasCompletedMarkRelative(snap, 1403) or quest_info.completed_level_plays.len >= 4),

        // Chapter 8
        10080001 => countRolesAtLeastLevel(snap, 70),
        10080002 => countRolesWithSkillLevel(snap, 3, 4),
        10080003 => countEchoes(snap, 25, 1),
        10080004 => @intFromBool(quest_info.hasFinishedQuest(145000004)),
        10080005 => CalabashHelper.currentLevel(snap.assets, snap.basic_comp, snap.echo_comp),
        10080006 => @intFromBool(quest_info.opened_chests.len >= 20 or snap.map_comp.info.unlocked_fogs.len >= 9),
        10080008 => @intFromBool(hasCompletedMarkRelative(snap, 1503) or quest_info.completed_level_plays.len >= 5),

        // Chapter 9
        10090001 => countRolesAtLeastLevel(snap, 80),
        10090002 => countRolesWithSkillLevel(snap, 3, 6),
        10090003 => countTunedEchoes(snap) * 10,
        10090004 => @intFromBool(quest_info.hasFinishedQuest(114000027)),
        10090005 => CalabashHelper.currentLevel(snap.assets, snap.basic_comp, snap.echo_comp),
        10090006 => @intFromBool(quest_info.opened_chests.len >= 30 or snap.map_comp.info.unlocked_fogs.len >= 12),
        10090007 => @intFromBool(hasCompletedMarkRelative(snap, 1603) or quest_info.completed_level_plays.len >= 6),
        10090008 => @intFromBool(hasCompletedMarkRelative(snap, 1703) or quest_info.completed_level_plays.len >= 7),

        // Tower of Adversity (10030008 handled in Chapter 3; Chapters 5-8 below)
        10050010 => @intFromBool(snap.progress_comp.info.challenge.tower_floors.len >= 1 or quest_info.completed_dungeons.len >= 3),
        10060010 => @intFromBool(snap.progress_comp.info.challenge.tower_floors.len >= 3 or quest_info.completed_dungeons.len >= 4),
        10070007 => @intFromBool(countTowerStars(snap, 0) >= 6 or snap.progress_comp.info.challenge.tower_floors.len >= 6),
        10080007 => @intFromBool(countTowerStars(snap, 2) >= 6 or countTowerStars(snap, 0) >= 12),
        else => 0,
    };
}

fn appendRewardDataForDrop(
    arena: std.mem.Allocator,
    assets: *const Assets,
    rewards: *std.ArrayList(pb.AdventureRewardData),
    drop_id: i32,
) !void {
    if (drop_id <= 0) return;
    for (rewards.items) |existing| {
        if (existing.DropId == drop_id) return;
    }
    const pkg = assets.tables.drop_package.getDataById(drop_id) orelse return;
    var items: std.ArrayList(pb.AdventureItemData) = .empty;
    var it = pkg.DropPreview.map.iterator();
    while (it.next()) |kv| {
        const item_id = kv.key_ptr.*;
        const count = if (kv.value_ptr.* > 0) kv.value_ptr.* else 1;
        if (item_id <= 0) continue;
        try items.append(arena, .{
            .ItemId = item_id,
            .ItemNum = count,
        });
    }
    try rewards.append(arena, .{
        .DropId = drop_id,
        .Items = items,
    });
}

fn buildManualData(arena: std.mem.Allocator, snap: AdventureSnapshot) !pb.AdventureManualData {
    var tasks: std.ArrayList(pb.AdventreTask) = .empty;
    var finished_per_chapter: [16]i32 = .{0} ** 16;

    for (snap.assets.tables.adventure_task.items) |task| {
        const raw_progress = evaluateTaskProgress(snap, task.Id);
        const progress = std.math.clamp(raw_progress, 0, task.NeedProgress);
        const is_done = progress >= task.NeedProgress;
        if (is_done and task.ChapterId >= 1 and task.ChapterId < @as(i32, @intCast(finished_per_chapter.len))) {
            finished_per_chapter[@intCast(task.ChapterId)] += 1;
        }
        var cbuf: [48]u8 = undefined;
        const ckey = try std.fmt.bufPrint(&cbuf, "adv_task:{d}", .{task.Id});
        const is_claimed = snap.progress_comp.info.hasClaim(ckey);
        try tasks.append(arena, .{
            .Id = task.Id,
            .State = if (is_claimed) .Received else if (is_done) .Finish else .UnFinish,
            .AdventreProgress = progress,
        });
    }

    var unlock_chapters: std.ArrayList(i32) = .empty;
    try unlock_chapters.append(arena, 1);

    var now_chapter: i32 = 1;
    var check_chap: i32 = 2;
    while (snap.assets.tables.adventure_task_chapter.getDataById(check_chap)) |chap| : (check_chap += 1) {
        const prev_idx: usize = @intCast(check_chap - 1);
        if (prev_idx >= finished_per_chapter.len) break;
        if (finished_per_chapter[prev_idx] < chap.LevelUnlockCount) break;
        try unlock_chapters.append(arena, chap.Id);
        now_chapter = chap.Id;
    }

    var visible_tasks: std.ArrayList(pb.AdventreTask) = .empty;
    for (tasks.items) |t| {
        const cfg = snap.assets.tables.adventure_task.getDataById(t.Id) orelse continue;
        if (std.mem.indexOfScalar(i32, unlock_chapters.items, cfg.ChapterId) != null) {
            try visible_tasks.append(arena, t);
        }
    }

    var reward_chapters: std.ArrayList(i32) = .empty;
    var received_chapter: i32 = 0;
    for (snap.assets.tables.adventure_task_chapter.items) |chap| {
        var rbuf: [48]u8 = undefined;
        const rkey = try std.fmt.bufPrint(&rbuf, "adv_chap:{d}", .{chap.Id});
        if (snap.progress_comp.info.hasClaim(rkey)) {
            try reward_chapters.append(arena, chap.Id);
            if (chap.Id > received_chapter) received_chapter = chap.Id;
        }
    }

    return .{
        .AdventreTask = visible_tasks,
        .NowChapter = now_chapter,
        .ReceivedChapter = received_chapter,
        .UnlockChapters = unlock_chapters,
        .RewardChapters = reward_chapters,
    };
}

pub fn onAdventureManualRequest(
    txn: *Transaction(pb.AdventureManualRequest),
    assets: *const Assets,
    basic_comp: *const PlayerBasicComponent,
    role_comp: *const PlayerRoleComponent,
    weapon_comp: *const PlayerWeaponComponent,
    echo_comp: *const PlayerEchoComponent,
    map_comp: *const PlayerMapComponent,
    quest_comp: *const PlayerQuestComponent,
    progress_comp: *const PlayerProgressComponent,
    alloc: mem.Alloc,
) !void {
    const snap: AdventureSnapshot = .{
        .assets = assets,
        .basic_comp = basic_comp,
        .role_comp = role_comp,
        .weapon_comp = weapon_comp,
        .echo_comp = echo_comp,
        .map_comp = map_comp,
        .quest_comp = quest_comp,
        .progress_comp = progress_comp,
    };

    var targets: std.ArrayList(pb.DetectionTarget) = .empty;
    var rewards: std.ArrayList(pb.AdventureRewardData) = .empty;
    var monster_unlocks: std.ArrayList(i32) = .empty;
    var dungeon_unlocks: std.ArrayList(i32) = .empty;
    var silent_unlocks: std.ArrayList(i32) = .empty;
    var dungeon_configs: std.ArrayList(pb.AdventureDetectionConfig) = .empty;
    var silent_configs: std.ArrayList(pb.AdventureDetectionConfig) = .empty;
    var silent_first_award: std.ArrayList(pb.MapEntry(i32, pb.SlientFirstAwardState)) = .empty;

    for (assets.tables.adventure_task.items) |task| {
        try appendRewardDataForDrop(alloc.arena, assets, &rewards, task.DropIds);
    }
    for (assets.tables.adventure_task_chapter.items) |chap| {
        try appendRewardDataForDrop(alloc.arena, assets, &rewards, chap.DropIds);
    }

    for (assets.tables.monster_detection.items) |md| {
        try monster_unlocks.append(alloc.arena, md.Id);
        try targets.append(alloc.arena, .{
            .Id = md.Id,
            .Type = .NormalMonster,
            .UnlockState = true,
            .DetectionId = md.Id,
        });
    }
    for (assets.tables.dungeon_detection.items) |dd| {
        try dungeon_unlocks.append(alloc.arena, dd.Id);
        try targets.append(alloc.arena, .{
            .Id = dd.DungeonId,
            .Type = .Dungeon,
            .UnlockState = true,
            .DetectionId = dd.Id,
        });
        try dungeon_configs.append(alloc.arena, .{
            .Id = dd.Id,
            .EffectBeginTime = dd.BeginTimeStamp,
        });
    }
    for (assets.tables.silent_area_detection.items) |sd| {
        try silent_unlocks.append(alloc.arena, sd.Id);
        try targets.append(alloc.arena, .{
            .Id = sd.Id,
            .Type = .SilentArea,
            .UnlockState = true,
            .DetectionId = sd.Id,
        });
        try silent_configs.append(alloc.arena, .{
            .Id = sd.Id,
            .EffectBeginTime = sd.BeginTimeStamp,
        });

        const done = hasCompletedMarkRelative(snap, sd.MarkId);
        try silent_first_award.append(alloc.arena, .{
            .key = sd.Id,
            .value = if (done) .IsFinish else .NotFinish,
        });
    }

    txn.respond(.{
        .ErrorCode = .Success,
        .AdventureManualData = try buildManualData(alloc.arena, snap),
        .DetectionTarget = targets,
        .AdventureRewardData = rewards,
        .DetectionUnlocks = .{
            .MonsterDetectionIds = monster_unlocks,
            .DungeonDetectionIds = dungeon_unlocks,
            .SilentAreaDetectionIds = silent_unlocks,
        },
        .SlientFirstAwardMap = silent_first_award,
        .SilenceAreaConfigs = silent_configs,
        .DungeonDetections = dungeon_configs,
    });
}

pub fn onAdventureManualDataRequest(
    txn: *Transaction(pb.AdventureManualDataRequest),
    assets: *const Assets,
    basic_comp: *const PlayerBasicComponent,
    role_comp: *const PlayerRoleComponent,
    weapon_comp: *const PlayerWeaponComponent,
    echo_comp: *const PlayerEchoComponent,
    map_comp: *const PlayerMapComponent,
    quest_comp: *const PlayerQuestComponent,
    progress_comp: *const PlayerProgressComponent,
    alloc: mem.Alloc,
) !void {
    const snap: AdventureSnapshot = .{
        .assets = assets,
        .basic_comp = basic_comp,
        .role_comp = role_comp,
        .weapon_comp = weapon_comp,
        .echo_comp = echo_comp,
        .map_comp = map_comp,
        .quest_comp = quest_comp,
        .progress_comp = progress_comp,
    };
    txn.respond(.{
        .ErrorCode = .Success,
        .AdventureManualData = try buildManualData(alloc.arena, snap),
    });
}

pub fn onWeeklyFrameworkInfoRequest(
    txn: *Transaction(pb.WeeklyFrameworkInfoRequest),
    io: std.Io,
    alloc: mem.Alloc,
    basic_comp: *const PlayerBasicComponent,
    progress_comp: *const PlayerProgressComponent,
) !void {
    const rtc: std.Io.Clock = .real;
    const now_ms = rtc.now(io).toMilliseconds();
    var score_tasks: std.ArrayList(i32) = .empty;
    for ([_]i32{ 1000, 2000, 3000, 4000, 5000, 6000 }) |tid| {
        const k = try std.fmt.allocPrint(alloc.arena, "weekly_fw:{d}", .{tid});
        if (!progress_comp.info.hasClaim(k)) {
            try score_tasks.append(alloc.arena, tid);
        }
    }
    if (score_tasks.items.len == 0) {
        try score_tasks.appendSlice(alloc.arena, &.{ 1000, 2000, 3000, 4000, 5000, 6000 });
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .FrameworkInfo = .{
            .ConfigId = 1,
            .BeginTime = now_ms - 86_400_000,
            .EndTime = now_ms + 604_800_000,
            .ScoreTasks = score_tasks,
            .WorldLevel = @max(1, basic_comp.info.attributes.cur_world_level),
        },
    });
}

pub fn onMulGetAdventureRewardRequest(
    txn: *Transaction(pb.MulGetAdventureRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var claimed: std.ArrayList(i32) = .empty;
    var dirty = false;
    var kbuf: [48]u8 = undefined;

    for (txn.message.Id.items) |task_id| {
        const k = try std.fmt.bufPrint(&kbuf, "adv_task:{d}", .{task_id});
        if (!(try progress_comp.info.markClaim(alloc.gpa, k))) continue;
        dirty = true;
        try claimed.append(alloc.arena, task_id);

        if (assets.tables.adventure_task.getDataById(task_id)) |cfg| {
            if (cfg.DropIds > 0) {
                _ = try RewardService.grantDropPackagePopup(reward_ctx, cfg.DropIds, 19500);
            }
        } else {
            _ = try RewardService.grantTableReward(
                reward_ctx,
                "AdventureTaskV2",
                task_id,
                false,
                true,
                19500,
                &.{ .{ .item_id = 3, .count = 40 }, .{ .item_id = 1, .count = 100 } },
                null,
            );
        }
    }
    if (dirty) try progress_comp.save(fs, alloc.arena);

    txn.respond(.{
        .Code = .Success,
        .Id = claimed,
    });
}

pub fn onGetChapterRewardRequest(
    txn: *Transaction(pb.GetChapterRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const chap_id = txn.message.Chapter;
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "adv_chap:{d}", .{chap_id});
    if (try progress_comp.info.markClaim(alloc.gpa, k)) {
        try progress_comp.save(fs, alloc.arena);
        if (assets.tables.adventure_task_chapter.getDataById(chap_id)) |cfg| {
            if (cfg.DropIds > 0) {
                _ = try RewardService.grantDropPackagePopup(reward_ctx, cfg.DropIds, 19500);
            }
        } else {
            _ = try RewardService.grantTableReward(
                reward_ctx,
                "AdventureTaskChapterV2",
                chap_id,
                false,
                true,
                19500,
                &.{.{ .item_id = 3, .count = 100 }},
                null,
            );
        }
    }
    txn.respond(.{ .Chapter = chap_id, .Code = .Success });
}

pub fn onAddChapterResultRequest(
    txn: *Transaction(pb.AddChapterResultRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [64]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "chap_res:{d}:{d}:{d}", .{ txn.message.ActivityId, txn.message.ChapterId, txn.message.ResultId });
    _ = try progress_comp.info.markClaim(alloc.gpa, k);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onChapterResultRewardRequest(
    txn: *Transaction(pb.ChapterResultRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var kbuf: [64]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "chap_res_clm:{d}:{d}:{d}", .{ txn.message.ActivityId, txn.message.ChapterId, txn.message.ResultId });
    if (try progress_comp.info.markClaim(alloc.gpa, k)) {
        try progress_comp.save(fs, alloc.arena);
        _ = try RewardService.grantItems(reward_ctx, 0, true, &.{
            .{ .item_id = 3, .count = 50 },
            .{ .item_id = 2, .count = 10000 },
        }, 19500);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onReceivedSilentFirstAwardRequest(
    txn: *Transaction(pb.ReceivedSilentFirstAwardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "silent_first:{d}", .{txn.message.Id});
    var item_map: std.ArrayList(pb.MapEntry(i32, i32)) = .empty;
    if (try progress_comp.info.markClaim(alloc.gpa, k)) {
        try progress_comp.save(fs, alloc.arena);
        const drop_id: i32 = if (assets.tables.silent_area_detection.getDataById(txn.message.Id)) |_| 0 else 0;
        _ = drop_id;
        var granted: std.ArrayList(RewardService.RewardItem) = .empty;
        _ = try RewardService.grantTableReward(
            reward_ctx,
            "SilentAreaDetection",
            txn.message.Id,
            false,
            true,
            1,
            &.{ .{ .item_id = 3, .count = 40 }, .{ .item_id = 2, .count = 10000 } },
            &granted,
        );
        for (granted.items) |it| {
            try item_map.append(alloc.arena, .{ .key = it.item_id, .value = it.count });
        }
    }
    txn.respond(.{ .ErrorCode = .Success, .ItemMap = item_map });
}

pub fn onNormalMonsterManualInfoRequest(
    txn: *Transaction(pb.NormalMonsterManualInfoRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
) !void {
    var targets: std.ArrayList(pb.DetectionTarget) = .empty;
    for (assets.tables.monster_detection.items) |md| {
        if (txn.message.DetectionId > 0 and md.Id != txn.message.DetectionId) continue;
        try targets.append(alloc.arena, .{
            .Id = md.Id,
            .Type = .NormalMonster,
            .UnlockState = true,
            .DetectionId = md.Id,
        });
    }
    txn.respond(.{ .Code = .Success, .DetectionTarget = targets });
}

pub fn onWeeklyFrameworkScoreRewardRequest(
    txn: *Transaction(pb.WeeklyFrameworkScoreRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var kbuf: [48]u8 = undefined;
    var dirty = false;
    for (txn.message.TaskIds.items) |tid| {
        const k = try std.fmt.bufPrint(&kbuf, "weekly_fw:{d}", .{tid});
        if (try progress_comp.info.markClaim(alloc.gpa, k)) {
            dirty = true;
            _ = try RewardService.grantTableReward(
                reward_ctx,
                "WeeklyFrameAward",
                tid,
                false,
                true,
                19500,
                &.{
                    .{ .item_id = 3, .count = 80 },
                    .{ .item_id = 2, .count = 20000 },
                },
                null,
            );
        }
    }
    if (dirty) try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}
