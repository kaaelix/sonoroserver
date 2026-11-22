const std = @import("std");
const pb = @import("proto").pb;
const Transaction = @import("../../handlers.zig").Transaction;
const mem = @import("../../../mem.zig");
const FileSystem = @import("common").FileSystem;
const Assets = @import("../../../data/Assets.zig");
const PlayerBasicComponent = @import("../../../logic/component/player/PlayerBasicComponent.zig");
const PlayerEchoComponent = @import("../../../logic/component/player/PlayerEchoComponent.zig");
const PlayerInventoryComponent = @import("../../../logic/component/player/PlayerInventoryComponent.zig");
const PlayerWeaponComponent = @import("../../../logic/component/player/PlayerWeaponComponent.zig");
const RewardService = @import("../../../logic/helpers/reward.zig");
const CalabashHelper = @import("../../../logic/helpers/calabash.zig");
const PlayerProgressComponent = @import("../../../logic/component/player/PlayerProgressComponent.zig");

pub fn onCalabashLevelRewardRequest(
    txn: *Transaction(pb.CalabashLevelRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    io: std.Io,
) !void {
    const level = txn.message.Level;
    if (level <= 0) {
        txn.respond(.{ .ErrorCode = .ErrCalabashLevelRequest });
        return;
    }
    const calabash_cfg = assets.tables.calabash_level.getDataById(level) orelse {
        txn.respond(.{ .ErrorCode = .ErrCalabashLevelConfig });
        return;
    };
    const current_level = CalabashHelper.currentLevel(assets, basic_comp, echo_comp);
    if (level > current_level) {
        txn.respond(.{ .ErrorCode = .ErrCalabashLevelUp });
        return;
    }
    if (containsInt(echo_comp.calabash_info.rewarded_levels, level)) {
        txn.respond(.{ .ErrorCode = .ErrCalabashLevelRewardDone });
        return;
    }

    try appendRewardedLevel(alloc.gpa, echo_comp, level);
    try PlayerEchoComponent.saveCalabash(alloc.gpa, fs, echo_comp.player_id, echo_comp.calabash_info);
    if (calabash_cfg.RewardId > 0) {
        _ = try RewardService.grantDropPackage(.{
            .alloc = alloc,
            .io = io,
            .fs = fs,
            .assets = assets,
            .conn = txn.conn,
            .basic_comp = basic_comp,
            .inventory_comp = inventory_comp,
            .weapon_comp = weapon_comp,
            .echo_comp = echo_comp,
        }, calabash_cfg.RewardId, 0);
    }

    const rewarded_levels = try intList(echo_comp.calabash_info.rewarded_levels, alloc.arena);
    try txn.conn.push(alloc.gpa, io, pb.CalabashLevelsRewardNotify{ .RewardedLevels = rewarded_levels });
    txn.respond(.{ .ErrorCode = .Success });
}

fn appendRewardedLevel(
    gpa: std.mem.Allocator,
    echo_comp: *PlayerEchoComponent,
    level: i32,
) !void {
    const levels = try gpa.alloc(i32, echo_comp.calabash_info.rewarded_levels.len + 1);
    @memcpy(levels[0..echo_comp.calabash_info.rewarded_levels.len], echo_comp.calabash_info.rewarded_levels);
    levels[echo_comp.calabash_info.rewarded_levels.len] = level;

    if (echo_comp.calabash_info.rewarded_levels.len != 0) {
        gpa.free(echo_comp.calabash_info.rewarded_levels);
    }
    echo_comp.calabash_info.rewarded_levels = levels;
}

fn containsInt(values: []const i32, needle: i32) bool {
    for (values) |value| {
        if (value == needle) return true;
    }
    return false;
}

fn intList(values: []const i32, arena: std.mem.Allocator) !std.ArrayList(i32) {
    var list: std.ArrayList(i32) = .empty;
    try list.ensureTotalCapacity(arena, values.len);
    for (values) |value| list.appendAssumeCapacity(value);
    return list;
}

pub fn onMulCalabashLevelRewardRequest(
    txn: *Transaction(pb.MulCalabashLevelRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    basic_comp: *PlayerBasicComponent,
    echo_comp: *PlayerEchoComponent,
    reward_ctx: RewardService.RewardContext,
    io: std.Io,
) !void {
    const current_level = CalabashHelper.currentLevel(assets, basic_comp, echo_comp);
    var claimed_levels: std.ArrayList(i32) = .empty;
    var dirty = false;

    for (txn.message.Level.items) |level| {
        if (level <= 0 or level > current_level) continue;
        if (containsInt(echo_comp.calabash_info.rewarded_levels, level)) continue;
        const cfg = assets.tables.calabash_level.getDataById(level) orelse continue;

        try appendRewardedLevel(alloc.gpa, echo_comp, level);
        dirty = true;
        try claimed_levels.append(alloc.arena, level);
        if (cfg.RewardId > 0) {
            _ = try RewardService.grantDropPackagePopup(reward_ctx, cfg.RewardId, 19500);
        }
    }

    if (dirty) {
        try PlayerEchoComponent.saveCalabash(alloc.gpa, fs, echo_comp.player_id, echo_comp.calabash_info);
        const rewarded_levels = try intList(echo_comp.calabash_info.rewarded_levels, alloc.arena);
        try txn.conn.push(alloc.gpa, io, pb.CalabashLevelsRewardNotify{ .RewardedLevels = rewarded_levels });
    }

    txn.respond(.{
        .ErrorCode = .Success,
        .Level = claimed_levels,
    });
}

pub fn onPhantomCollectReceiveRequest(
    txn: *Transaction(pb.PhantomCollectReceiveRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const reward_type: i32 = if (txn.message.Type) |t| @intFromEnum(t) else 0;
    var key_buf: [64]u8 = undefined;
    const key = try std.fmt.bufPrint(&key_buf, "phantom_collect:{d}:{d}", .{ txn.message.ActivityId, reward_type });

    var item_map: std.ArrayList(pb.MapEntry(i32, i32)) = .empty;
    if (try progress_comp.info.markClaim(alloc.gpa, key)) {
        try progress_comp.save(fs, alloc.arena);
        var granted: std.ArrayList(RewardService.RewardItem) = .empty;
        _ = try RewardService.grantTableReward(
            reward_ctx,
            "PhantomCollect",
            reward_type + 1,
            false,
            true,
            19500,
            &.{ .{ .item_id = 3, .count = 60 }, .{ .item_id = 2, .count = 20000 } },
            &granted,
        );
        for (granted.items) |it| {
            try item_map.append(alloc.arena, .{ .key = it.item_id, .value = it.count });
        }
    }

    txn.respond(.{
        .ErrorCode = .Success,
        .ItemMap = item_map,
    });
}

pub fn onCalabashMsgRequest(
    txn: *Transaction(pb.CalabashMsgRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
    basic_comp: *const PlayerBasicComponent,
    echo_comp: *const PlayerEchoComponent,
) !void {
    const notify = try CalabashHelper.buildMsgNotify(alloc.arena, assets, basic_comp, echo_comp);
    txn.respond(.{
        .ErrCode = .Success,
        .CalabashMsg = notify.CalabashMsg,
        .CalabashCfg = notify.CalabashCfg,
    });
}
