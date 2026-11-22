const RewardIndex = @This();
const std = @import("std");
const Io = std.Io;
const Allocator = std.mem.Allocator;
const ArenaAllocator = std.heap.ArenaAllocator;

pub const ItemPair = struct {
    item_id: i32,
    count: i32,
};

pub const RowEntry = struct {
    key: i64,
    drop_id: i32 = 0,
    alt_drop_id: i32 = 0,
    cost_id: i32 = 0,
    cost_count: i32 = 0,
    extra_int: i32 = 0,
    items: []const ItemPair = &.{},
    pay_items: []const ItemPair = &.{},
};

const IndexState = struct {
    mutex: Io.Mutex = .init,
    arena: ArenaAllocator,
    tables: std.StringHashMapUnmanaged([]const RowEntry) = .empty,
};

gpa: Allocator,
state: *IndexState,

pub fn init(gpa: Allocator) !RewardIndex {
    const state = try gpa.create(IndexState);
    errdefer gpa.destroy(state);
    state.* = .{
        .arena = ArenaAllocator.init(gpa),
    };
    return .{
        .gpa = gpa,
        .state = state,
    };
}

pub fn deinit(self: *RewardIndex) void {
    self.state.arena.deinit();
    self.gpa.destroy(self.state);
}

pub fn compositeKey(high: i32, low: i32) i64 {
    return (@as(i64, high) << 32) | @as(u32, @bitCast(low));
}

pub fn getRows(self: *const RewardIndex, io: Io, table_name: []const u8) []const RowEntry {
    self.state.mutex.lockUncancelable(io);
    defer self.state.mutex.unlock(io);

    if (self.state.tables.get(table_name)) |rows| return rows;
    return self.loadTableLocked(io, table_name) catch &.{};
}

pub fn getRow(self: *const RewardIndex, io: Io, table_name: []const u8, key: i64) ?RowEntry {
    const rows = self.getRows(io, table_name);
    for (rows) |row| {
        if (row.key == key) return row;
    }
    return null;
}

pub fn getRowAtOrFirst(self: *const RewardIndex, io: Io, table_name: []const u8, key: i64) ?RowEntry {
    const rows = self.getRows(io, table_name);
    for (rows) |row| {
        if (row.key == key) return row;
    }
    return if (rows.len > 0) rows[0] else null;
}

const table_aliases = std.StaticStringMap([]const u8).initComptime(.{
    .{ "ActivityReward", "CiacconaActivityReward" },
    .{ "Avignon", "AvignonTask" },
    .{ "BlackCoastTheme", "BlackCoastThemeTaskRe" },
    .{ "CertificateReward", "BattlePassReward" },
    .{ "CorniceMeeting", "CorniceChallenge" },
    .{ "DailyAdventure", "DailyAdventureTask" },
    .{ "DropCatch", "DropCatchReward" },
    .{ "ExploreScoreReward", "ExploreReward" },
    .{ "FarmGold", "FarmGoldScore" },
    .{ "FishingHandBook", "FishingIllustratedReward" },
    .{ "FlagChallenge", "FlagChallengeTask" },
    .{ "FunPlay", "FunPlayActivityChallenge" },
    .{ "GiantActivity", "GiantDeliveryReward" },
    .{ "Giveback", "GivebackScoreReward" },
    .{ "HarvestActivity", "RiskHarvestScoreReward" },
    .{ "HonamiStoryReward", "HonamiStoryScoreReward" },
    .{ "HonamiStoryTask", "HonamiStoryLimitTask" },
    .{ "JokerGuessLevel", "GuessJokerLevel" },
    .{ "KurotatoTask", "KurotatoLimitAward" },
    .{ "LinkageCheckIn", "LinkageReward" },
    .{ "LongShanReward", "LongShanScoreReward" },
    .{ "MapTravelLevel", "MapTravelConfig" },
    .{ "MapTravelTask", "TravelTask" },
    .{ "MixDrink", "DrinksRoleInvite" },
    .{ "MoonPhase", "MoonPhaseReward" },
    .{ "MowTower", "MowTowerRewardRe" },
    .{ "NewBieCourse", "NewbieCourse" },
    .{ "NewJourneyTask", "WorldNewJourney" },
    .{ "NewbieCarnival", "NewbieCarnivalTask" },
    .{ "NewbieScore", "NewbieMainActReward" },
    .{ "OccupyReward", "OccupyScore" },
    .{ "OnlineMotorActivity", "OnlineMotorGlobalTask" },
    .{ "Parkour", "ParkourChallenge" },
    .{ "PhantomBattleReward", "PhantomBattleCardReward" },
    .{ "PhantomCollect", "PhantomCollectActivity" },
    .{ "PhotoFight", "PhotoFightReward" },
    .{ "PhotoMemory", "PhotoMemoryCollect" },
    .{ "PreheatSign", "PreheatSignRe" },
    .{ "QingXiao", "GuQinActivityTask" },
    .{ "RacingBetsTask", "RacingBetsReward" },
    .{ "RegressBonus", "RegressBonusReward" },
    .{ "RegressReward", "RegressDisposableReward" },
    .{ "RegressTask", "RegressQuest" },
    .{ "RiskHarvest", "RiskHarvestScoreReward" },
    .{ "RogueResEnding", "RogueResEndAward" },
    .{ "RogueResIllustration", "RogueResCollection" },
    .{ "RoguelikeReward", "RogueSeasonReward" },
    .{ "RoleSkinTrial", "RoleSkinTrialInfo" },
    .{ "RoleTowerRankReward", "RoleTowerReward" },
    .{ "RoleTowerTask", "RoleTowerReward" },
    .{ "RoleTrial", "RoleTrialInfo" },
    .{ "ScratchCard", "ScratchCardRewardRe" },
    .{ "ThemeCelebration", "BlackCoastThemeRewardRe" },
    .{ "TimePointReward", "TimePointRewardActivity" },
    .{ "TrackMoonPhase", "TrackMoonPhaseActivity" },
    .{ "WuWuKuji", "WuWuTaskPackage" },
});

pub fn resolveTableName(table_name: []const u8) []const u8 {
    return table_aliases.get(table_name) orelse table_name;
}

fn loadTableLocked(self: *const RewardIndex, io: Io, table_name: []const u8) ![]const RowEntry {
    const resolved_name = resolveTableName(table_name);
    var path_buf: [256]u8 = undefined;
    const path = try std.fmt.bufPrint(&path_buf, "assets/BinData/{s}.json", .{resolved_name});

    var scratch = ArenaAllocator.init(self.gpa);
    defer scratch.deinit();
    const salloc = scratch.allocator();

    const json_bytes = Io.Dir.cwd().readFileAlloc(io, path, salloc, .limited(8 * 1024 * 1024)) catch {
        const stored_name = try self.state.arena.allocator().dupe(u8, table_name);
        try self.state.tables.put(self.state.arena.allocator(), stored_name, &.{});
        return &.{};
    };

    const parsed = try std.json.parseFromSliceLeaky(std.json.Value, salloc, json_bytes, .{});
    const aalloc = self.state.arena.allocator();

    var list: std.ArrayList(RowEntry) = .empty;
    switch (parsed) {
        .array => |arr| {
            for (arr.items) |elem| {
                if (elem != .object) continue;
                try appendParsedRow(&list, aalloc, elem.object);
            }
        },
        .object => |obj| {
            var it = obj.iterator();
            while (it.next()) |kv| {
                if (kv.value_ptr.* == .object) {
                    try appendParsedRow(&list, aalloc, kv.value_ptr.object);
                }
            }
        },
        else => {},
    }

    const slice = try list.toOwnedSlice(aalloc);
    const stored_name = try aalloc.dupe(u8, table_name);
    try self.state.tables.put(aalloc, stored_name, slice);
    return slice;
}

fn asInt(val: std.json.Value) ?i32 {
    return switch (val) {
        .integer => |i| std.math.cast(i32, i),
        .float => |f| std.math.cast(i32, @as(i64, @intFromFloat(f))),
        .string => |s| std.fmt.parseInt(i32, s, 10) catch null,
        else => null,
    };
}

fn firstInt(obj: std.json.ObjectMap, keys: []const []const u8) i32 {
    for (keys) |k| {
        if (obj.get(k)) |v| {
            switch (v) {
                .integer, .float, .string => if (asInt(v)) |iv| if (iv != 0) return iv,
                .array => |arr| if (arr.items.len > 0) if (asInt(arr.items[0])) |iv| if (iv != 0) return iv,
                else => {},
            }
        }
    }
    return 0;
}

fn parseItemPairs(val: std.json.Value, aalloc: Allocator) ![]const ItemPair {
    var items: std.ArrayList(ItemPair) = .empty;
    switch (val) {
        .object => |map| {
            var it = map.iterator();
            while (it.next()) |kv| {
                const item_id = std.fmt.parseInt(i32, kv.key_ptr.*, 10) catch continue;
                const count = asInt(kv.value_ptr.*) orelse continue;
                if (item_id > 0 and count > 0) {
                    try items.append(aalloc, .{ .item_id = item_id, .count = count });
                }
            }
        },
        .array => |arr| {
            for (arr.items) |elem| {
                if (elem != .object) continue;
                const item_id = firstInt(elem.object, &.{ "ItemId", "Id", "itemId", "id" });
                const count = firstInt(elem.object, &.{ "Count", "Num", "Amount", "count" });
                if (item_id > 0 and count > 0) {
                    try items.append(aalloc, .{ .item_id = item_id, .count = count });
                }
            }
        },
        else => {},
    }
    return try items.toOwnedSlice(aalloc);
}

fn parseFirstCost(obj: std.json.ObjectMap) struct { id: i32, count: i32 } {
    for ([_][]const u8{ "Consume", "Cost", "ConsumeItem" }) |k| {
        const v = obj.get(k) orelse continue;
        switch (v) {
            .object => |m| {
                var it = m.iterator();
                while (it.next()) |kv| {
                    const id = std.fmt.parseInt(i32, kv.key_ptr.*, 10) catch continue;
                    const cnt = asInt(kv.value_ptr.*) orelse continue;
                    if (id > 0 and cnt > 0) return .{ .id = id, .count = cnt };
                }
            },
            else => {},
        }
    }
    return .{
        .id = firstInt(obj, &.{ "ConsumeId", "CostId" }),
        .count = firstInt(obj, &.{ "ConsumeCount", "CostCount" }),
    };
}

fn appendParsedRow(list: *std.ArrayList(RowEntry), aalloc: Allocator, obj: std.json.ObjectMap) !void {
    const key: i64 = blk: {
        if (obj.get("BattlePassId")) |bp_v| {
            if (obj.get("Level")) |lvl_v| {
                if (asInt(bp_v)) |bp_id| if (asInt(lvl_v)) |lvl| {
                    break :blk compositeKey(bp_id, lvl);
                };
            }
        }
        const primary = firstInt(obj, &.{
            "Id", "TaskId", "RewardId", "BoardId", "RecallId", "Level", "ActivityId", "GoodsId", "BuffId", "RoleId", "MailId", "Score",
        });
        break :blk primary;
    };

    const drop_id = firstInt(obj, &.{
        "DropId", "DropIds", "Drop", "RewardId", "RewardIds", "FirstClearDropId", "FirstPassDropId", "PassDropId", "NormalDropId", "UnlockDropId", "FreeDropId", "PreviewDrop", "DropGroup", "DropReward", "RewardDropId", "RewardDrop", "EntrustReward", "PassReward", "SignReward", "DailyClearDropId", "FirstWinReward", "RewardGroup", "EndReward", "Award", "FirstAward", "ScoreReward", "PhantomReward", "FinalRewardId", "Reward", "TaskReward", "TargetReward",
    });
    const alt_drop_id = firstInt(obj, &.{
        "GrandRewardId", "PayDropId", "ExtraDrop", "PermanentPreviewDrop", "PayDrop", "HighDropId", "MaxLevelDropId", "KeepDropId", "LevelPassReward", "DataDockReward", "PhantomSideQuestReward",
    });
    const extra_int = firstInt(obj, &.{
        "NeedProgress", "NeedPt", "RewardScore", "TargetScore", "Score", "LevelUpExp", "QuestId", "RewardUnlockCount", "NeedScore", "Target",
    });

    var items: []const ItemPair = &.{};
    for ([_][]const u8{ "FreeReward", "TaskReward", "Reward", "TargetReward", "Content", "UnlockReward", "Items", "ItemMap", "SignRewards", "Award", "Rewards", "ActivityReward", "RewardItem", "RewardInfo" }) |k| {
        if (obj.get(k)) |v| {
            const parsed_items = try parseItemPairs(v, aalloc);
            if (parsed_items.len > 0) {
                items = parsed_items;
                break;
            }
        }
    }

    var pay_items: []const ItemPair = &.{};
    for ([_][]const u8{ "PayReward", "ExtraReward" }) |k| {
        if (obj.get(k)) |v| {
            const parsed_items = try parseItemPairs(v, aalloc);
            if (parsed_items.len > 0) {
                pay_items = parsed_items;
                break;
            }
        }
    }

    const cost = parseFirstCost(obj);
    try list.append(aalloc, .{
        .key = key,
        .drop_id = drop_id,
        .alt_drop_id = alt_drop_id,
        .cost_id = cost.id,
        .cost_count = cost.count,
        .extra_int = extra_int,
        .items = items,
        .pay_items = pay_items,
    });
}

test "RewardIndex parses key, compositeKey, drops and item maps" {
    const gpa = std.testing.allocator;
    var idx = try RewardIndex.init(gpa);
    defer idx.deinit();

    var scratch = ArenaAllocator.init(gpa);
    defer scratch.deinit();
    const salloc = scratch.allocator();

    const sample =
        \\[{"BattlePassId": 2, "Level": 5, "FreeReward": {"2": 10000}, "PayReward": {"3": 680}, "DropId": 4101}]
    ;
    const parsed = try std.json.parseFromSliceLeaky(std.json.Value, salloc, sample, .{});
    var list: std.ArrayList(RowEntry) = .empty;
    try appendParsedRow(&list, idx.state.arena.allocator(), parsed.array.items[0].object);
    try std.testing.expectEqual(@as(usize, 1), list.items.len);
    try std.testing.expectEqual(compositeKey(2, 5), list.items[0].key);
    try std.testing.expectEqual(@as(i32, 4101), list.items[0].drop_id);
    try std.testing.expectEqual(@as(i32, 2), list.items[0].items[0].item_id);
    try std.testing.expectEqual(@as(i32, 680), list.items[0].pay_items[0].count);
}

test "RewardIndex resolveTableName and real 3.8.11 BinData loads" {
    try std.testing.expectEqualStrings("AvignonTask", resolveTableName("Avignon"));
    try std.testing.expectEqualStrings("CiacconaActivityReward", resolveTableName("ActivityReward"));
    try std.testing.expectEqualStrings("BattlePassReward", resolveTableName("CertificateReward"));
    try std.testing.expectEqualStrings("WuWuTaskPackage", resolveTableName("WuWuKuji"));
    try std.testing.expectEqualStrings("GivebackScoreReward", resolveTableName("Giveback"));
    try std.testing.expectEqualStrings("RogueResEndAward", resolveTableName("RogueResEnding"));
    try std.testing.expectEqualStrings("DrinksRoleInvite", resolveTableName("MixDrink"));

    const gpa = std.testing.allocator;
    const io = std.testing.io;
    var idx = try RewardIndex.init(gpa);
    defer idx.deinit();

    for ([_][]const u8{
        "Avignon",
        "RogueSeasonReward",
        "RogueLimitTimeReward",
        "KurotatoResAward",
        "HonamiStoryScoreReward",
        "DailyAdventurePoint",
        "PhantomBattleBadgeReward",
    }) |tbl| {
        try std.testing.expect(idx.getRows(io, tbl).len > 0);
    }
}
