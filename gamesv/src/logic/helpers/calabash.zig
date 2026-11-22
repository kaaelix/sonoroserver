const std = @import("std");
const pb = @import("proto").pb;
const Assets = @import("../../data/Assets.zig");
const EchoInfo = @import("../../fs/EchoInfo.zig");
const PlayerBasicComponent = @import("../component/player/PlayerBasicComponent.zig");
const PlayerEchoComponent = @import("../component/player/PlayerEchoComponent.zig");

pub const DiscoveryOutcome = struct {
    changed: bool = false,
    union_exp: i32 = 0,
};

pub const Summary = struct {
    level: i32,
    exp: i32,
    next_level_up_exp: i32,
    level_condition: i32,
    cost: i32,
};

pub fn discoveredMaxQuality(
    assets: *const Assets,
    echo_comp: *const PlayerEchoComponent,
    monster_id: i32,
) i32 {
    var best = echo_comp.calabash_info.highestQualityForMonster(monster_id);
    var echo_it = echo_comp.echo_map.iterator();
    while (echo_it.next()) |entry| {
        const item = assets.tables.phantom_item.getDataById(entry.value_ptr.id) orelse continue;
        if (item.MonsterId == monster_id) {
            best = @max(best, item.QualityId);
        }
    }
    return best;
}

fn checkLevelCondition(
    assets: *const Assets,
    basic_comp: *const PlayerBasicComponent,
    condition_id: i32,
) bool {
    if (condition_id <= 0) return true;
    if (assets.tables.condition.getDataById(condition_id)) |cond| {
        if (std.mem.eql(u8, cond.Type, "CheckCurWorldLevel")) {
            const req = paramInt(cond, "Level") orelse 0;
            return basic_comp.info.attributes.cur_world_level >= req;
        }
        if (std.mem.eql(u8, cond.Type, "CheckOriginWorldLevel")) {
            const req = paramInt(cond, "Level") orelse 0;
            return basic_comp.info.attributes.origin_world_level >= req;
        }
        if (std.mem.eql(u8, cond.Type, "CheckLevel")) {
            const req = paramInt(cond, "Level") orelse 1;
            return basic_comp.info.attributes.level >= req;
        }
        return true;
    }
    if (assets.tables.condition_group.getDataById(condition_id)) |group| {
        if (group.GroupId.len == 0) return true;
        if (group.Relation == 1) {
            for (group.GroupId) |sub_id| {
                if (checkLevelCondition(assets, basic_comp, sub_id)) return true;
            }
            return false;
        }
        for (group.GroupId) |sub_id| {
            if (!checkLevelCondition(assets, basic_comp, sub_id)) return false;
        }
        return true;
    }
    return true;
}

fn paramInt(cond: Assets.DataTables.Condition, key: []const u8) ?i32 {
    const lp = cond.LimitParams orelse return null;
    if (lp != .object) return null;
    const val = lp.object.get(key) orelse return null;
    return switch (val) {
        .integer => |i| @intCast(i),
        .float => |f| @intFromFloat(f),
        .string => |s| std.fmt.parseInt(i32, s, 10) catch null,
        else => null,
    };
}

fn calabashExpForCondition(assets: *const Assets, condition_id: i32) i32 {
    const cond = assets.tables.calabash_develop_condition.getDataById(condition_id) orelse return 10;
    return cond.RewardExp.map.get(10) orelse 0;
}

pub fn requiredQualityForCondition(assets: *const Assets, condition_id: i32, idx: usize) i32 {
    if (assets.tables.calabash_develop_condition.getDataById(condition_id)) |cond| {
        if (cond.ConditionParam.len > 0) return cond.ConditionParam[0];
    }
    return @intCast(idx + 2);
}

pub fn totalDevelopExp(assets: *const Assets, echo_comp: *const PlayerEchoComponent) i32 {
    var total_exp: i32 = 0;
    for (assets.tables.calabash_develop_reward.items) |reward| {
        if (!reward.IsShow) continue;
        const max_quality = discoveredMaxQuality(assets, echo_comp, reward.MonsterId);
        if (max_quality <= 0) continue;

        for (reward.DevelopCondition, 0..) |condition_id, idx| {
            const req_quality = requiredQualityForCondition(assets, condition_id, idx);
            if (max_quality >= req_quality) {
                total_exp += calabashExpForCondition(assets, condition_id);
            }
        }
    }
    return total_exp;
}

pub fn summarize(
    assets: *const Assets,
    basic_comp: *const PlayerBasicComponent,
    echo_comp: *const PlayerEchoComponent,
) Summary {
    const total_exp = totalDevelopExp(assets, echo_comp);
    var current_level: i32 = 1;
    var remaining_exp: i32 = total_exp;
    var next_level_up_exp: i32 = 30;
    var level_condition: i32 = 502001;
    var cost: i32 = 8;

    if (assets.tables.calabash_level.getDataById(1)) |lv1| {
        next_level_up_exp = lv1.LevelUpExp;
        level_condition = lv1.LevelUpCondition;
        cost = lv1.Cost;
    }

    var check_lvl: i32 = 1;
    while (assets.tables.calabash_level.getDataById(check_lvl)) |cfg| : (check_lvl += 1) {
        current_level = cfg.Level;
        next_level_up_exp = cfg.LevelUpExp;
        level_condition = cfg.LevelUpCondition;
        cost = cfg.Cost;
        if (cfg.LevelUpExp <= 0 or remaining_exp < cfg.LevelUpExp) break;
        if (!checkLevelCondition(assets, basic_comp, cfg.LevelUpCondition)) break;
        remaining_exp -= cfg.LevelUpExp;
    }

    return .{
        .level = current_level,
        .exp = remaining_exp,
        .next_level_up_exp = next_level_up_exp,
        .level_condition = level_condition,
        .cost = cost,
    };
}

pub fn currentLevel(
    assets: *const Assets,
    basic_comp: *const PlayerBasicComponent,
    echo_comp: *const PlayerEchoComponent,
) i32 {
    return summarize(assets, basic_comp, echo_comp).level;
}

pub fn currentCost(
    assets: *const Assets,
    basic_comp: *const PlayerBasicComponent,
    echo_comp: *const PlayerEchoComponent,
) i32 {
    return summarize(assets, basic_comp, echo_comp).cost;
}

pub fn recordEchoDiscovery(
    gpa: std.mem.Allocator,
    assets: *const Assets,
    echo_comp: *PlayerEchoComponent,
    monster_id: i32,
    quality: i32,
) !DiscoveryOutcome {
    if (monster_id <= 0 or quality <= 0) return .{};

    const prev_rewarded = echo_comp.calabash_info.rewardedQualityForMonster(monster_id);
    const changed = try echo_comp.calabash_info.upsertDiscovery(gpa, monster_id, quality, true);
    if (!changed) return .{};

    var union_exp: i32 = 0;
    if (assets.tables.calabash_develop_reward.getDataById(monster_id)) |reward| {
        if (reward.IsShow) {
            for (reward.DevelopCondition, 0..) |condition_id, idx| {
                const req_quality = requiredQualityForCondition(assets, condition_id, idx);
                if (quality >= req_quality and prev_rewarded < req_quality) {
                    if (assets.tables.calabash_develop_condition.getDataById(condition_id)) |cond| {
                        union_exp += cond.RewardExp.map.get(1) orelse 0;
                    }
                }
            }
        }
    }

    return .{
        .changed = true,
        .union_exp = union_exp,
    };
}

pub fn buildMsgNotify(
    arena: std.mem.Allocator,
    assets: *const Assets,
    basic_comp: *const PlayerBasicComponent,
    echo_comp: *const PlayerEchoComponent,
) !pb.CalabashMsgNotify {
    var develop_infos: std.ArrayList(pb.CalabashDevelopInfo) = .empty;

    for (assets.tables.calabash_develop_reward.items) |reward| {
        if (!reward.IsShow) continue;

        const max_quality = discoveredMaxQuality(assets, echo_comp, reward.MonsterId);
        if (max_quality <= 0) continue;

        var conditions: std.ArrayList(pb.CalabashDevelopConditionState) = .empty;
        for (reward.DevelopCondition, 0..) |condition_id, idx| {
            const req_quality = requiredQualityForCondition(assets, condition_id, idx);
            if (max_quality >= req_quality) {
                try conditions.append(arena, .{
                    .ConditionId = condition_id,
                    .Rewarded = true,
                });
            }
        }
        if (conditions.items.len > 0) {
            try develop_infos.append(arena, .{
                .MonsterId = reward.MonsterId,
                .UnlockConditions = conditions,
            });
        }
    }

    const summary = summarize(assets, basic_comp, echo_comp);
    var levels: std.ArrayList(i32) = .empty;
    var lv: i32 = 1;
    while (lv <= summary.level) : (lv += 1) {
        try levels.append(arena, lv);
    }

    var catch_gain: std.ArrayList(pb.MapEntry(i32, i32)) = .empty;
    for (assets.tables.calabash_level.items) |level_cfg| {
        try catch_gain.append(arena, .{ .key = level_cfg.Level, .value = level_cfg.TempCatchGain });
    }

    return .{
        .CalabashMsg = .{
            .Level = summary.level,
            .Exp = summary.exp,
            .UnlockedLevels = levels,
            .UnlockedDevelopRewards = develop_infos,
        },
        .CalabashCfg = .{
            .LevelUpExp = summary.next_level_up_exp,
            .LevelUpCondition = summary.level_condition,
            .CatchGain = catch_gain,
        },
    };
}
