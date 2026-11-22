const std = @import("std");
const pb = @import("proto").pb;
const common = @import("common");
const mem = @import("../../mem.zig");
const Connection = @import("../../network/Connection.zig");
const Assets = @import("../../data/Assets.zig");
const CostEntry = @import("../../data/tables/common_types.zig").CostEntry;
const ConsumeHelper = @import("consume.zig");
const RewardService = @import("reward.zig");
const PlayerBasicComponent = @import("../component/player/PlayerBasicComponent.zig");
const PlayerInventoryComponent = @import("../component/player/PlayerInventoryComponent.zig");
const PlayerWeaponComponent = @import("../component/player/PlayerWeaponComponent.zig");
const PlayerEchoComponent = @import("../component/player/PlayerEchoComponent.zig");

const FileSystem = common.FileSystem;

pub const CraftContext = struct {
    conn: *Connection,
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    assets: *const Assets,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
};

pub const CraftOutcome = struct {
    ok: bool,
    err: pb.ErrorCode = .Success,
    items: std.ArrayList(pb.SingleItemInfo) = .empty,
};

pub fn buildScaledCosts(
    arena: std.mem.Allocator,
    entries: []const CostEntry,
    count: i32,
) ![]ConsumeHelper.CostItem {
    var costs: std.ArrayList(ConsumeHelper.CostItem) = .empty;
    for (entries) |entry| {
        if (entry.ItemId <= 0 or entry.Count <= 0) continue;
        const total = std.math.mul(i32, entry.Count, count) catch return error.Overflow;
        try costs.append(arena, .{ .id = entry.ItemId, .count = total });
    }
    return costs.items;
}

/// Validates and deducts `costs`, then grants `reward_item_id * reward_count`.
pub fn consumeAndGrant(
    ctx: CraftContext,
    costs: []const ConsumeHelper.CostItem,
    reward_item_id: i32,
    reward_count: i32,
    reason: i32,
) !CraftOutcome {
    if (reward_item_id <= 0 or reward_count <= 0) {
        return .{ .ok = false, .err = .RequestParamError };
    }
    const paid = try ConsumeHelper.consumeCosts(
        ctx.conn,
        ctx.alloc,
        ctx.fs,
        ctx.basic_comp,
        ctx.inventory_comp,
        costs,
        ctx.io,
    );
    if (!paid) {
        return .{ .ok = false, .err = .ErrItemNotEnough };
    }
    _ = try RewardService.grantItems(.{
        .alloc = ctx.alloc,
        .io = ctx.io,
        .fs = ctx.fs,
        .assets = ctx.assets,
        .conn = ctx.conn,
        .basic_comp = ctx.basic_comp,
        .inventory_comp = ctx.inventory_comp,
        .weapon_comp = ctx.weapon_comp,
        .echo_comp = ctx.echo_comp,
    }, 0, false, &.{.{ .item_id = reward_item_id, .count = reward_count }}, reason);

    var items: std.ArrayList(pb.SingleItemInfo) = .empty;
    try items.append(ctx.alloc.arena, .{ .ItemId = reward_item_id, .ItemNum = reward_count });
    return .{ .ok = true, .items = items };
}

pub fn addProficiencyAndUpdateLevel(
    level: *i32,
    total_proficiency: *i32,
    gain: i32,
    level_rows: anytype,
) bool {
    if (gain <= 0) return false;
    total_proficiency.* += gain;
    var best_lvl: i32 = @max(1, level.*);
    for (level_rows) |row| {
        if (total_proficiency.* >= row.Completeness and row.Id > best_lvl) {
            best_lvl = row.Id;
        }
    }
    if (best_lvl != level.*) {
        level.* = best_lvl;
        return true;
    }
    return false;
}

pub fn grantDropAndCollectItems(
    ctx: CraftContext,
    drop_id: i32,
    fallback_item_id: i32,
    fallback_count: i32,
    reason: i32,
    out_map: *std.AutoHashMapUnmanaged(i32, i32),
) !void {
    const rctx: RewardService.RewardContext = .{
        .alloc = ctx.alloc,
        .io = ctx.io,
        .fs = ctx.fs,
        .assets = ctx.assets,
        .conn = ctx.conn,
        .basic_comp = ctx.basic_comp,
        .inventory_comp = ctx.inventory_comp,
        .weapon_comp = ctx.weapon_comp,
        .echo_comp = ctx.echo_comp,
    };
    if (drop_id > 0) {
        if (ctx.assets.tables.drop_package.getDataById(drop_id)) |pkg| {
            _ = try RewardService.grantDropPackage(rctx, drop_id, reason);
            var it = pkg.DropPreview.map.iterator();
            while (it.next()) |kv| {
                if (kv.key_ptr.* <= 0) continue;
                const cnt = if (kv.value_ptr.* > 0) kv.value_ptr.* else 1;
                const gop = try out_map.getOrPut(ctx.alloc.arena, kv.key_ptr.*);
                if (!gop.found_existing) gop.value_ptr.* = 0;
                gop.value_ptr.* += cnt;
            }
            return;
        }
    }
    if (fallback_item_id > 0 and fallback_count > 0) {
        _ = try RewardService.grantItems(rctx, 0, true, &.{.{ .item_id = fallback_item_id, .count = fallback_count }}, reason);
        const gop = try out_map.getOrPut(ctx.alloc.arena, fallback_item_id);
        if (!gop.found_existing) gop.value_ptr.* = 0;
        gop.value_ptr.* += fallback_count;
    }
}

test "CraftHelper addProficiencyAndUpdateLevel advances level by threshold" {
    const LevelRow = struct { Id: i32, Completeness: i32 };
    const rows = [_]LevelRow{
        .{ .Id = 1, .Completeness = 0 },
        .{ .Id = 2, .Completeness = 500 },
        .{ .Id = 3, .Completeness = 1500 },
    };
    var lvl: i32 = 1;
    var prof: i32 = 0;
    try std.testing.expect(!addProficiencyAndUpdateLevel(&lvl, &prof, 200, &rows));
    try std.testing.expectEqual(@as(i32, 1), lvl);
    try std.testing.expect(addProficiencyAndUpdateLevel(&lvl, &prof, 400, &rows));
    try std.testing.expectEqual(@as(i32, 2), lvl);
    try std.testing.expect(addProficiencyAndUpdateLevel(&lvl, &prof, 1000, &rows));
    try std.testing.expectEqual(@as(i32, 3), lvl);
}
