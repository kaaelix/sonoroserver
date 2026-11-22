const std = @import("std");
const pb = @import("proto").pb;
const common = @import("common");
const mem = @import("../../mem.zig");
const FileSystem = common.FileSystem;
const Connection = @import("../../network/Connection.zig");
const comp_util = @import("../component/comp_util.zig");
const InventoryInfo = @import("../../fs/InventoryInfo.zig");
const BasicInfo = @import("../../fs/BasicInfo.zig");
const IntMap = @import("../../data/tables/int_map.zig").IntMap;
const PlayerBasicComponent = @import("../component/player/PlayerBasicComponent.zig");
const PlayerInventoryComponent = @import("../component/player/PlayerInventoryComponent.zig");

pub const CostItem = struct {
    id: i32,
    count: i32,
};

pub fn appendIntMap(list: *std.ArrayList(CostItem), arena: std.mem.Allocator, consume_map: IntMap(i32, i32)) !void {
    var it = consume_map.map.iterator();
    while (it.next()) |kv| {
        if (kv.key_ptr.* <= 0 or kv.value_ptr.* <= 0) continue;
        try list.append(arena, .{ .id = kv.key_ptr.*, .count = kv.value_ptr.* });
    }
}

pub fn hasCosts(
    arena: std.mem.Allocator,
    basic_comp: *const PlayerBasicComponent,
    inventory_comp: *const PlayerInventoryComponent,
    costs: []const CostItem,
) !bool {
    var totals: std.AutoHashMapUnmanaged(i32, i64) = .empty;
    defer totals.deinit(arena);

    for (costs) |entry| {
        if (entry.id <= 0 or entry.count <= 0) continue;
        const gop = try totals.getOrPut(arena, entry.id);
        if (!gop.found_existing) gop.value_ptr.* = 0;
        gop.value_ptr.* += entry.count;
    }

    var it = totals.iterator();
    while (it.next()) |kv| {
        const id = kv.key_ptr.*;
        const needed = kv.value_ptr.*;
        const owned: i64 = switch (id) {
            2 => basic_comp.info.attributes.coin,
            3 => @max(basic_comp.info.attributes.rare_coin, inventory_comp.info.normalItemCount(3)),
            4 => basic_comp.info.attributes.cash_coin,
            else => inventory_comp.info.normalItemCount(id),
        };
        if (owned < needed) return false;
    }
    return true;
}

pub fn consumeCosts(
    conn: *Connection,
    alloc: mem.Alloc,
    fs: *FileSystem,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    costs: []const CostItem,
    io: std.Io,
) !bool {
    if (!(try hasCosts(alloc.arena, basic_comp, inventory_comp, costs))) return false;

    var totals: std.AutoHashMapUnmanaged(i32, i32) = .empty;
    defer totals.deinit(alloc.arena);
    var ordered_ids: std.ArrayList(i32) = .empty;

    for (costs) |entry| {
        if (entry.id <= 0 or entry.count <= 0) continue;
        const gop = try totals.getOrPut(alloc.arena, entry.id);
        if (!gop.found_existing) {
            gop.value_ptr.* = 0;
            try ordered_ids.append(alloc.arena, entry.id);
        }
        gop.value_ptr.* += entry.count;
    }

    if (ordered_ids.items.len == 0) return true;

    var basic_dirty = false;
    var inv_dirty = false;
    var updated_items: std.ArrayList(pb.NormalItem) = .empty;
    var attr_updates: std.ArrayList(pb.PlayerAttr) = .empty;

    for (ordered_ids.items) |id| {
        const count = totals.get(id) orelse continue;
        if (count <= 0) continue;

        if (id == 2) {
            basic_comp.info.attributes.coin -= count;
            basic_dirty = true;
            try updated_items.append(alloc.arena, .{
                .Id = 2,
                .Count = basic_comp.info.attributes.coin,
                .ExpireTime = 0,
            });
            try attr_updates.append(alloc.arena, .{
                .Key = .Coin,
                .ValueType = .Int32,
                .Value = .{ .Int32Value = basic_comp.info.attributes.coin },
            });
        } else if (id == 3) {
            const next_val = @max(basic_comp.info.attributes.rare_coin, inventory_comp.info.normalItemCount(3)) - count;
            basic_comp.info.attributes.rare_coin = next_val;
            basic_dirty = true;
            _ = try InventoryInfo.consumeNormalItem(&inventory_comp.info, alloc.gpa, 3, count);
            inv_dirty = true;
            try updated_items.append(alloc.arena, .{
                .Id = 3,
                .Count = next_val,
                .ExpireTime = 0,
            });
            try attr_updates.append(alloc.arena, .{
                .Key = .RareCoin,
                .ValueType = .Int32,
                .Value = .{ .Int32Value = next_val },
            });
        } else if (id == 4) {
            basic_comp.info.attributes.cash_coin -= count;
            basic_dirty = true;
            try updated_items.append(alloc.arena, .{
                .Id = 4,
                .Count = basic_comp.info.attributes.cash_coin,
                .ExpireTime = 0,
            });
            try attr_updates.append(alloc.arena, .{
                .Key = .CashCoin,
                .ValueType = .Int32,
                .Value = .{ .Int32Value = basic_comp.info.attributes.cash_coin },
            });
        } else {
            _ = try InventoryInfo.consumeNormalItem(&inventory_comp.info, alloc.gpa, id, count);
            inv_dirty = true;
            try updated_items.append(alloc.arena, .{
                .Id = id,
                .Count = inventory_comp.info.normalItemCount(id),
                .ExpireTime = 0,
            });
        }
    }

    if (inv_dirty) {
        const inv_path = try std.fmt.allocPrint(alloc.arena, "player/{}/{s}", .{ inventory_comp.player_id, InventoryInfo.data_path });
        try comp_util.saveStruct(fs, inventory_comp.info, inv_path, alloc.arena);
    }
    if (basic_dirty) {
        const basic_path = try std.fmt.allocPrint(alloc.arena, "player/{}/basic_info", .{basic_comp.player_id});
        try comp_util.saveStruct(fs, basic_comp.info, basic_path, alloc.arena);
    }

    if (updated_items.items.len > 0) {
        try conn.push(alloc.gpa, io, pb.NormalItemUpdateNotify{
            .NormalItemList = updated_items,
            .NoTips = true,
        });
    }
    if (attr_updates.items.len > 0) {
        try conn.push(alloc.gpa, io, pb.PlayerAttrNotify{
            .Attributes = attr_updates,
        });
    }

    return true;
}
