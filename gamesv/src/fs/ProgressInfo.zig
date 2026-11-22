const ProgressInfo = @This();
const std = @import("std");

const Allocator = std.mem.Allocator;

pub const default: ProgressInfo = .{};
pub const data_path = "progress_info";

pub const IdCount = struct {
    id: i32,
    count: i32 = 0,
};

pub const IdRoleCount = struct {
    id: i32,
    count: i32 = 0,
    last_role_id: i32 = 0,
};

pub const ShopGoodCount = struct {
    shop_id: i32,
    goods_id: i32,
    count: i32 = 0,
};

pub const AchievementRecord = struct {
    id: i32,
    finish_time: u32 = 0,
    received: bool = false,
    cur_progress: i32 = 0,
    total_progress: i32 = 1,
};

pub const LordGymPass = struct {
    gym_id: i32,
    pass_time: i32 = 1,
};

pub const TowerFloor = struct {
    config_id: i32,
    difficulty: i32 = 1,
    area_num: i32 = 1,
    star: i32 = 3,
    star_mask: i32 = 7,
};

pub const DifficultyReward = struct {
    difficulty: i32,
    reward_index: i32,
};

pub const DifficultyScore = struct {
    season: i32 = 1,
    difficulty: i32 = 0,
    score: i32 = 0,
};

pub const FishingCabinItem = struct {
    cabin_type: i32 = 0,
    id: i32 = 0,
    incr_id: i32 = 0,
    item_id: i32 = 0,
    x: i32 = 0,
    y: i32 = 0,
    rotate: i32 = 0,
    size: i32 = 0,
    cup: i32 = 0,
    quality: i32 = 0,
    price: i32 = 0,
};

pub const DailyCounter = struct {
    counter_type: i32,
    extra_param: i32 = 0,
    count: i32 = 0,
};

pub const CraftState = struct {
    unlocked_forge: []i32 = &.{},
    forge_counts: []IdRoleCount = &.{},
    synth_level: i32 = 1,
    synth_proficiency: i32 = 0,
    synth_claimed_level: i32 = 1,
    unlocked_synth: []i32 = &.{},
    synth_counts: []IdRoleCount = &.{},
    cook_level: i32 = 1,
    cook_proficiency: i32 = 0,
    cook_claimed_level: i32 = 1,
    unlocked_cook: []i32 = &.{},
    unlocked_processed: []i32 = &.{},
    cook_counts: []IdRoleCount = &.{},
};

pub const ShopState = struct {
    payshop_bought: []IdCount = &.{},
    fixed_shop_bought: []ShopGoodCount = &.{},
    item_exchange_counts: []IdCount = &.{},
};

pub const AchievementState = struct {
    entries: []AchievementRecord = &.{},
    claimed_groups: []i32 = &.{},
};

pub const LivenessState = struct {
    liveness_count: i32 = 0,
    claimed_boxes: []i32 = &.{},
    taken_tasks: []i32 = &.{},
    task_progress: []IdCount = &.{},
};

pub const ChallengeState = struct {
    read_lord_gyms: []i32 = &.{},
    passed_lord_gyms: []LordGymPass = &.{},
    tower_season: i32 = 1,
    tower_floors: []TowerFloor = &.{},
    tower_claimed_rewards: []DifficultyReward = &.{},
    finished_rogue_insts: []i32 = &.{},
    new_tower_scores: []DifficultyScore = &.{},
};

pub const FishingState = struct {
    tech_levels: []IdCount = &.{},
    cabin_items: []FishingCabinItem = &.{},
    next_fish_incr_id: i32 = 1,
    total_sold: i32 = 0,
};

pub const CounterState = struct {
    daily_counters: []DailyCounter = &.{},
    daily_quest_group_counts: []IdCount = &.{},
    records: []i32 = &.{},
    claimed_cdkeys: [][]const u8 = &.{},
};

pub const StrInt = struct {
    key: []const u8,
    val: i64 = 0,
};

pub const StrPair = struct {
    key: []const u8,
    val: []const u8 = "",
};

pub const DynamicBucket = struct {
    ints: []StrInt = &.{},
    strs: []StrPair = &.{},
    claims: [][]const u8 = &.{},
};

craft: CraftState = .{},
shop: ShopState = .{},
achievement: AchievementState = .{},
liveness: LivenessState = .{},
challenge: ChallengeState = .{},
fishing: FishingState = .{},
counters: CounterState = .{},
dynamic: DynamicBucket = .{},

pub fn containsId(slice: []const i32, needle: i32) bool {
    for (slice) |v| {
        if (v == needle) return true;
    }
    return false;
}

pub fn appendUniqueId(slice: *[]i32, gpa: Allocator, val: i32) !bool {
    if (val <= 0 or containsId(slice.*, val)) return false;
    const grown = try gpa.alloc(i32, slice.len + 1);
    @memcpy(grown[0..slice.len], slice.*);
    grown[slice.len] = val;
    if (slice.len != 0) gpa.free(slice.*);
    slice.* = grown;
    return true;
}

pub fn appendItem(comptime T: type, slice: *[]T, gpa: Allocator, val: T) !*T {
    const grown = try gpa.alloc(T, slice.len + 1);
    @memcpy(grown[0..slice.len], slice.*);
    grown[slice.len] = val;
    if (slice.len != 0) gpa.free(slice.*);
    slice.* = grown;
    return &slice.*[slice.len - 1];
}

pub fn getIdCount(slice: []const IdCount, id: i32) i32 {
    for (slice) |entry| {
        if (entry.id == id) return entry.count;
    }
    return 0;
}

pub fn addIdCount(slice: *[]IdCount, gpa: Allocator, id: i32, delta: i32) !i32 {
    if (id <= 0 or delta == 0) return getIdCount(slice.*, id);
    for (slice.*) |*entry| {
        if (entry.id == id) {
            entry.count += delta;
            return entry.count;
        }
    }
    const created = try appendItem(IdCount, slice, gpa, .{ .id = id, .count = delta });
    return created.count;
}

pub fn findIdRoleCount(slice: []const IdRoleCount, id: i32) ?IdRoleCount {
    for (slice) |entry| {
        if (entry.id == id) return entry;
    }
    return null;
}

pub fn addIdRoleCount(slice: *[]IdRoleCount, gpa: Allocator, id: i32, delta: i32, role_id: i32) !void {
    if (id <= 0) return;
    for (slice.*) |*entry| {
        if (entry.id == id) {
            entry.count += delta;
            if (role_id > 0) entry.last_role_id = role_id;
            return;
        }
    }
    _ = try appendItem(IdRoleCount, slice, gpa, .{
        .id = id,
        .count = delta,
        .last_role_id = role_id,
    });
}

pub fn getShopGoodCount(info: ProgressInfo, shop_id: i32, goods_id: i32) i32 {
    for (info.shop.fixed_shop_bought) |rec| {
        if (rec.shop_id == shop_id and rec.goods_id == goods_id) return rec.count;
    }
    return 0;
}

pub fn addShopGoodCount(info: *ProgressInfo, gpa: Allocator, shop_id: i32, goods_id: i32, delta: i32) !i32 {
    for (info.shop.fixed_shop_bought) |*rec| {
        if (rec.shop_id == shop_id and rec.goods_id == goods_id) {
            rec.count += delta;
            return rec.count;
        }
    }
    const created = try appendItem(ShopGoodCount, &info.shop.fixed_shop_bought, gpa, .{
        .shop_id = shop_id,
        .goods_id = goods_id,
        .count = delta,
    });
    return created.count;
}

pub fn findAchievementPtr(info: *ProgressInfo, id: i32) ?*AchievementRecord {
    for (info.achievement.entries) |*rec| {
        if (rec.id == id) return rec;
    }
    return null;
}

pub fn finishAchievement(info: *ProgressInfo, gpa: Allocator, id: i32, finish_time: u32) !*AchievementRecord {
    if (info.findAchievementPtr(id)) |rec| {
        if (rec.finish_time == 0) rec.finish_time = if (finish_time > 0) finish_time else 1;
        rec.cur_progress = rec.total_progress;
        return rec;
    }
    return try appendItem(AchievementRecord, &info.achievement.entries, gpa, .{
        .id = id,
        .finish_time = if (finish_time > 0) finish_time else 1,
        .received = false,
        .cur_progress = 1,
        .total_progress = 1,
    });
}

pub fn hasPassedLordGym(info: ProgressInfo, gym_id: i32) bool {
    for (info.challenge.passed_lord_gyms) |rec| {
        if (rec.gym_id == gym_id) return true;
    }
    return false;
}

pub fn recordLordGymPass(info: *ProgressInfo, gpa: Allocator, gym_id: i32, pass_time: i32) !bool {
    if (gym_id <= 0) return false;
    for (info.challenge.passed_lord_gyms) |*rec| {
        if (rec.gym_id == gym_id) {
            if (pass_time > 0 and (rec.pass_time <= 0 or pass_time < rec.pass_time)) {
                rec.pass_time = pass_time;
            }
            return false;
        }
    }
    _ = try appendItem(LordGymPass, &info.challenge.passed_lord_gyms, gpa, .{
        .gym_id = gym_id,
        .pass_time = if (pass_time > 0) pass_time else 1,
    });
    return true;
}

pub fn hasClaimedTowerReward(info: ProgressInfo, difficulty: i32, reward_index: i32) bool {
    for (info.challenge.tower_claimed_rewards) |rec| {
        if (rec.difficulty == difficulty and rec.reward_index == reward_index) return true;
    }
    return false;
}

pub fn claimTowerReward(info: *ProgressInfo, gpa: Allocator, difficulty: i32, reward_index: i32) !bool {
    if (info.hasClaimedTowerReward(difficulty, reward_index)) return false;
    _ = try appendItem(DifficultyReward, &info.challenge.tower_claimed_rewards, gpa, .{
        .difficulty = difficulty,
        .reward_index = reward_index,
    });
    return true;
}

pub fn recordTowerFloor(info: *ProgressInfo, gpa: Allocator, config_id: i32, difficulty: i32, area_num: i32, star: i32, star_mask: i32) !void {
    for (info.challenge.tower_floors) |*rec| {
        if (rec.config_id == config_id) {
            if (star > rec.star) rec.star = star;
            rec.star_mask |= star_mask;
            return;
        }
    }
    _ = try appendItem(TowerFloor, &info.challenge.tower_floors, gpa, .{
        .config_id = config_id,
        .difficulty = difficulty,
        .area_num = area_num,
        .star = star,
        .star_mask = star_mask,
    });
}

pub fn getDailyCounter(info: ProgressInfo, counter_type: i32, extra_param: i32) i32 {
    for (info.counters.daily_counters) |c| {
        if (c.counter_type == counter_type and c.extra_param == extra_param) return c.count;
    }
    return 0;
}

pub fn incDailyCounter(info: *ProgressInfo, gpa: Allocator, counter_type: i32, extra_param: i32, delta: i32) !void {
    if (counter_type <= 0 or delta == 0) return;
    for (info.counters.daily_counters) |*c| {
        if (c.counter_type == counter_type and c.extra_param == extra_param) {
            c.count += delta;
            return;
        }
    }
    _ = try appendItem(DailyCounter, &info.counters.daily_counters, gpa, .{
        .counter_type = counter_type,
        .extra_param = extra_param,
        .count = delta,
    });
}

pub fn getNewTowerScore(info: ProgressInfo, season: i32, difficulty: i32) i32 {
    var best: i32 = 0;
    for (info.challenge.new_tower_scores) |s| {
        if ((season <= 0 or s.season == season) and (difficulty < 0 or s.difficulty == difficulty)) {
            if (s.score > best) best = s.score;
        }
    }
    return best;
}

pub fn hasClaimedCdKey(info: ProgressInfo, key: []const u8) bool {
    for (info.counters.claimed_cdkeys) |existing| {
        if (std.mem.eql(u8, existing, key)) return true;
    }
    return false;
}

pub fn claimCdKey(info: *ProgressInfo, gpa: Allocator, key: []const u8) !bool {
    if (key.len == 0 or info.hasClaimedCdKey(key)) return false;
    const grown = try gpa.alloc([]const u8, info.counters.claimed_cdkeys.len + 1);
    @memcpy(grown[0..info.counters.claimed_cdkeys.len], info.counters.claimed_cdkeys);
    grown[info.counters.claimed_cdkeys.len] = try gpa.dupe(u8, key);
    if (info.counters.claimed_cdkeys.len != 0) gpa.free(info.counters.claimed_cdkeys);
    info.counters.claimed_cdkeys = grown;
    return true;
}

pub fn getKeyInt(info: ProgressInfo, key: []const u8) i64 {
    for (info.dynamic.ints) |entry| {
        if (std.mem.eql(u8, entry.key, key)) return entry.val;
    }
    return 0;
}

pub fn setKeyInt(info: *ProgressInfo, gpa: Allocator, key: []const u8, val: i64) !void {
    if (key.len == 0) return;
    for (info.dynamic.ints) |*entry| {
        if (std.mem.eql(u8, entry.key, key)) {
            entry.val = val;
            return;
        }
    }
    const dup_k = try gpa.dupe(u8, key);
    errdefer gpa.free(dup_k);
    _ = try appendItem(StrInt, &info.dynamic.ints, gpa, .{ .key = dup_k, .val = val });
}

pub fn addKeyInt(info: *ProgressInfo, gpa: Allocator, key: []const u8, delta: i64) !i64 {
    const next = info.getKeyInt(key) + delta;
    try info.setKeyInt(gpa, key, next);
    return next;
}

pub fn getKeyStr(info: ProgressInfo, key: []const u8) []const u8 {
    for (info.dynamic.strs) |entry| {
        if (std.mem.eql(u8, entry.key, key)) return entry.val;
    }
    return "";
}

pub fn setKeyStr(info: *ProgressInfo, gpa: Allocator, key: []const u8, val: []const u8) !void {
    if (key.len == 0) return;
    for (info.dynamic.strs) |*entry| {
        if (std.mem.eql(u8, entry.key, key)) {
            const dup_v = try gpa.dupe(u8, val);
            if (entry.val.len != 0) gpa.free(entry.val);
            entry.val = dup_v;
            return;
        }
    }
    const dup_k = try gpa.dupe(u8, key);
    errdefer gpa.free(dup_k);
    const dup_v = try gpa.dupe(u8, val);
    errdefer gpa.free(dup_v);
    _ = try appendItem(StrPair, &info.dynamic.strs, gpa, .{ .key = dup_k, .val = dup_v });
}

pub fn hasClaim(info: ProgressInfo, key: []const u8) bool {
    for (info.dynamic.claims) |existing| {
        if (std.mem.eql(u8, existing, key)) return true;
    }
    return false;
}

pub fn markClaim(info: *ProgressInfo, gpa: Allocator, key: []const u8) !bool {
    if (key.len == 0 or info.hasClaim(key)) return false;
    const dup_k = try gpa.dupe(u8, key);
    errdefer gpa.free(dup_k);
    _ = try appendItem([]const u8, &info.dynamic.claims, gpa, dup_k);
    return true;
}

pub fn countClaimsPrefix(info: ProgressInfo, prefix: []const u8) i32 {
    var count: i32 = 0;
    for (info.dynamic.claims) |existing| {
        if (std.mem.startsWith(u8, existing, prefix)) count += 1;
    }
    return count;
}

pub fn deinit(info: ProgressInfo, gpa: Allocator) void {
    std.zon.parse.free(gpa, info);
}

test "ProgressInfo tracks shop, craft, achievements, challenges, counters, and frees cleanly" {
    const gpa = std.testing.allocator;
    var info: ProgressInfo = .{};
    defer info.deinit(gpa);

    try std.testing.expectEqual(@as(i32, 3), try ProgressInfo.addIdCount(&info.shop.payshop_bought, gpa, 10001, 3));
    try ProgressInfo.addIdRoleCount(&info.craft.cook_counts, gpa, 10000, 2, 1104);
    try std.testing.expectEqual(@as(i32, 2), ProgressInfo.findIdRoleCount(info.craft.cook_counts, 10000).?.count);

    const ach = try info.finishAchievement(gpa, 300101, 12345);
    try std.testing.expectEqual(@as(u32, 12345), ach.finish_time);
    try std.testing.expect(!ach.received);

    try std.testing.expect(try info.recordLordGymPass(gpa, 1001, 45));
    try std.testing.expect(info.hasPassedLordGym(1001));
    try info.recordTowerFloor(gpa, 1, 1, 1, 3, 7);
    try std.testing.expect(try info.claimTowerReward(gpa, 1, 0));
    try std.testing.expect(!(try info.claimTowerReward(gpa, 1, 0)));

    try info.incDailyCounter(gpa, 15, 1, 2);
    try std.testing.expectEqual(@as(i32, 2), info.getDailyCounter(15, 1));
    try std.testing.expect(try info.claimCdKey(gpa, "WUTHERINGGIFT"));
    try std.testing.expect(!(try info.claimCdKey(gpa, "WUTHERINGGIFT")));
}
