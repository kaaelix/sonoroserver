const std = @import("std");
const common = @import("common");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const Assets = @import("../../data/Assets.zig");
const ProgressInfo = @import("../../fs/ProgressInfo.zig");
const PlayerProgressComponent = @import("../../logic/component/player/PlayerProgressComponent.zig");
const PlayerBasicComponent = @import("../../logic/component/player/PlayerBasicComponent.zig");
const PlayerInventoryComponent = @import("../../logic/component/player/PlayerInventoryComponent.zig");
const PlayerWeaponComponent = @import("../../logic/component/player/PlayerWeaponComponent.zig");
const PlayerEchoComponent = @import("../../logic/component/player/PlayerEchoComponent.zig");
const ConsumeHelper = @import("../../logic/helpers/consume.zig");
const RewardService = @import("../../logic/helpers/reward.zig");
const QuestEngine = @import("../../logic/helpers/quest_engine.zig");
const Transaction = @import("../handlers.zig").Transaction;

const FileSystem = common.FileSystem;

fn cabinItemToPb(rec: ProgressInfo.FishingCabinItem) pb.FishingItemInfo {
    return .{
        .ItemId = rec.item_id,
        .IncrId = rec.incr_id,
        .Rotate = std.enums.fromInt(pb.FishingItemRotate, rec.rotate),
        .Pos = .{ .X = rec.x, .Y = rec.y },
        .Size = rec.size,
        .Cup = std.enums.fromInt(pb.FishCup, rec.cup),
        .Quality = rec.quality,
        .Price = rec.price,
    };
}

fn loadJsonArray(arena: std.mem.Allocator, io: std.Io, path: []const u8) ?[]const std.json.Value {
    const raw = std.Io.Dir.readFileAlloc(std.Io.Dir.cwd(), io, path, arena, std.Io.Limit.unlimited) catch return null;
    const parsed = std.json.parseFromSlice(std.json.Value, arena, raw, .{}) catch return null;
    return switch (parsed.value) {
        .array => |arr| arr.items,
        else => null,
    };
}

fn techEffectConsume(arena: std.mem.Allocator, io: std.Io, effect_id: i32) ![]ConsumeHelper.CostItem {
    const items = loadJsonArray(arena, io, "assets/BinData/FishingTechEffect.json") orelse return &.{};
    for (items) |entry| {
        const obj = switch (entry) {
            .object => |o| o,
            else => continue,
        };
        const id_val = obj.get("Id") orelse continue;
        const id: i64 = switch (id_val) {
            .integer => |v| v,
            else => continue,
        };
        if (id != effect_id) continue;
        const consume_val = obj.get("Consume") orelse return &.{};
        const consume_obj = switch (consume_val) {
            .object => |o| o,
            else => return &.{},
        };
        var costs: std.ArrayList(ConsumeHelper.CostItem) = .empty;
        var it = consume_obj.iterator();
        while (it.next()) |kv| {
            const item_id = std.fmt.parseInt(i32, kv.key_ptr.*, 10) catch continue;
            const cnt: i32 = switch (kv.value_ptr.*) {
                .integer => |v| @intCast(std.math.clamp(v, 0, std.math.maxInt(i32))),
                else => continue,
            };
            if (item_id > 0 and cnt > 0) {
                try costs.append(arena, .{ .id = item_id, .count = cnt });
            }
        }
        return costs.items;
    }
    return &.{};
}

fn pickFishForPoint(
    assets: *const Assets,
    arena: std.mem.Allocator,
    io: std.Io,
    entity_id: i64,
    seed: i64,
) struct { item_id: i32, quality: i32, price: i32, size: i32 } {
    if (loadJsonArray(arena, io, "assets/BinData/FishingPoint.json")) |points| {
        for (points) |entry| {
            const obj = switch (entry) {
                .object => |o| o,
                else => continue,
            };
            const eid: i64 = switch (obj.get("EntityConfigId") orelse obj.get("Id") orelse continue) {
                .integer => |v| v,
                else => continue,
            };
            if (eid != entity_id) continue;
            if (obj.get("RewardItem")) |rw| {
                if (rw == .object and rw.object.count() > 0) {
                    const keys_count = rw.object.count();
                    const pick_idx: usize = @intCast(@mod(seed, @as(i64, @intCast(keys_count))));
                    var it = rw.object.iterator();
                    var idx: usize = 0;
                    while (it.next()) |kv| : (idx += 1) {
                        if (idx == pick_idx) {
                            const item_id = std.fmt.parseInt(i32, kv.key_ptr.*, 10) catch break;
                            if (assets.tables.fishing_item.getDataById(item_id)) |fi| {
                                const price: i32 = if (fi.Price.len > 0) fi.Price[0] else 100;
                                return .{ .item_id = fi.Id, .quality = fi.Quality, .price = price, .size = 100 };
                            }
                            return .{ .item_id = item_id, .quality = 2, .price = 100, .size = 100 };
                        }
                    }
                }
            }
            if (obj.get("ShowItem")) |si| {
                if (si == .integer and si.integer > 0) {
                    const item_id: i32 = @intCast(std.math.clamp(si.integer, 1, std.math.maxInt(i32)));
                    if (assets.tables.fishing_item.getDataById(item_id)) |fi| {
                        const price: i32 = if (fi.Price.len > 0) fi.Price[0] else 100;
                        return .{ .item_id = fi.Id, .quality = fi.Quality, .price = price, .size = 100 };
                    }
                }
            }
            break;
        }
    }
    if (assets.tables.fishing_item.items.len > 0) {
        const idx: usize = @intCast(@mod(seed, @as(i64, @intCast(assets.tables.fishing_item.items.len))));
        const fi = assets.tables.fishing_item.items[idx];
        const price: i32 = if (fi.Price.len > 0) fi.Price[0] else 100;
        return .{ .item_id = fi.Id, .quality = fi.Quality, .price = price, .size = 100 };
    }
    return .{ .item_id = 80121101, .quality = 2, .price = 12, .size = 100 };
}

pub fn onFishingTechLevelUpRequest(
    txn: *Transaction(pb.FishingTechLevelUpRequest),
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    const node_id = txn.message.NodeId;
    const row = assets.tables.fishing_tech.getDataById(node_id) orelse {
        txn.respond(.{ .ErrorCode = .ErrFishingTechConfig });
        return;
    };
    if (row.PreNode != 0 and ProgressInfo.getIdCount(progress_comp.info.fishing.tech_levels, row.PreNode) <= 0) {
        txn.respond(.{ .ErrorCode = .ErrFishingTechConfig });
        return;
    }
    const cur_lvl = ProgressInfo.getIdCount(progress_comp.info.fishing.tech_levels, node_id);
    if (cur_lvl < 0 or @as(usize, @intCast(cur_lvl)) >= row.Effect.len) {
        txn.respond(.{
            .FishingTech = .{
                .NodeId = node_id,
                .Level = cur_lvl,
                .CanUnlock = true,
            },
            .ErrorCode = .Success,
        });
        return;
    }
    const effect_id = row.Effect[@intCast(cur_lvl)];
    const costs = try techEffectConsume(alloc.arena, io, effect_id);
    if (costs.len > 0) {
        if (!(try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, costs, io))) {
            txn.respond(.{ .ErrorCode = .ErrItemNotEnough });
            return;
        }
    }
    const new_lvl = try ProgressInfo.addIdCount(&progress_comp.info.fishing.tech_levels, alloc.gpa, node_id, 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .FishingTech = .{
            .NodeId = node_id,
            .Level = new_lvl,
            .CanUnlock = true,
        },
        .ErrorCode = .Success,
    });
}

fn upsertCabinItems(
    gpa: std.mem.Allocator,
    cabin_type: i32,
    cabin_id: i32,
    items: []const pb.FishingItemInfo,
    progress: *ProgressInfo,
) !void {
    for (items) |it| {
        if (it.IncrId > 0) {
            for (progress.fishing.cabin_items) |*existing| {
                if (existing.incr_id == it.IncrId) {
                    existing.cabin_type = cabin_type;
                    existing.id = cabin_id;
                    existing.item_id = it.ItemId;
                    existing.x = if (it.Pos) |p| p.X else 0;
                    existing.y = if (it.Pos) |p| p.Y else 0;
                    existing.rotate = if (it.Rotate) |r| @intFromEnum(r) else 0;
                    existing.size = it.Size;
                    existing.cup = if (it.Cup) |c| @intFromEnum(c) else 0;
                    existing.quality = it.Quality;
                    existing.price = it.Price;
                    break;
                }
            } else {
                _ = try ProgressInfo.appendItem(ProgressInfo.FishingCabinItem, &progress.fishing.cabin_items, gpa, .{
                    .cabin_type = cabin_type,
                    .id = cabin_id,
                    .incr_id = it.IncrId,
                    .item_id = it.ItemId,
                    .x = if (it.Pos) |p| p.X else 0,
                    .y = if (it.Pos) |p| p.Y else 0,
                    .rotate = if (it.Rotate) |r| @intFromEnum(r) else 0,
                    .size = it.Size,
                    .cup = if (it.Cup) |c| @intFromEnum(c) else 0,
                    .quality = it.Quality,
                    .price = it.Price,
                });
            }
        }
    }
}

pub fn onFishingCabinPutRequest(
    txn: *Transaction(pb.FishingCabinPutRequest),
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    alloc: mem.Alloc,
) !void {
    const req = txn.message;
    const ctype: i32 = if (req.CabinType) |c| @intFromEnum(c) else 0;

    if (req.RemoveIncrId > 0) {
        var i: usize = 0;
        while (i < progress_comp.info.fishing.cabin_items.len) {
            if (progress_comp.info.fishing.cabin_items[i].incr_id == req.RemoveIncrId) {
                const len = progress_comp.info.fishing.cabin_items.len;
                const shrunk = try alloc.gpa.alloc(ProgressInfo.FishingCabinItem, len - 1);
                @memcpy(shrunk[0..i], progress_comp.info.fishing.cabin_items[0..i]);
                @memcpy(shrunk[i..], progress_comp.info.fishing.cabin_items[i + 1 ..]);
                alloc.gpa.free(progress_comp.info.fishing.cabin_items);
                progress_comp.info.fishing.cabin_items = shrunk;
                break;
            }
            i += 1;
        }
    }

    try upsertCabinItems(alloc.gpa, ctype, req.Id, req.LeftFishingItems.items, &progress_comp.info);
    try upsertCabinItems(alloc.gpa, ctype, req.Id, req.RightFishingItems.items, &progress_comp.info);
    try progress_comp.save(fs, alloc.arena);

    txn.respond(.{ .ErrorCode = .Success });
}

fn sellFishValue(assets: *const Assets, rec: ProgressInfo.FishingCabinItem) i32 {
    if (rec.price > 0) return rec.price;
    if (assets.tables.fishing_item.getDataById(rec.item_id)) |row| {
        if (row.Price.len > 0) return row.Price[0];
    }
    return 100;
}

pub fn onFishingSellRequest(
    txn: *Transaction(pb.FishingSellRequest),
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    var total_coins: i32 = 0;
    var keep: std.ArrayList(ProgressInfo.FishingCabinItem) = .empty;
    for (progress_comp.info.fishing.cabin_items) |rec| {
        if (std.mem.indexOfScalar(i32, txn.message.IncrIds.items, rec.incr_id) != null) {
            total_coins += sellFishValue(assets, rec);
        } else {
            try keep.append(alloc.arena, rec);
        }
    }

    if (total_coins > 0) {
        const next = try alloc.gpa.dupe(ProgressInfo.FishingCabinItem, keep.items);
        if (progress_comp.info.fishing.cabin_items.len != 0) alloc.gpa.free(progress_comp.info.fishing.cabin_items);
        progress_comp.info.fishing.cabin_items = next;
        progress_comp.info.fishing.total_sold += total_coins;
        try progress_comp.save(fs, alloc.arena);

        _ = try RewardService.grantItems(.{
            .alloc = alloc,
            .io = io,
            .fs = fs,
            .assets = assets,
            .conn = txn.conn,
            .basic_comp = basic_comp,
            .inventory_comp = inventory_comp,
            .weapon_comp = weapon_comp,
            .echo_comp = echo_comp,
        }, 0, false, &.{.{ .item_id = 80100001, .count = total_coins }}, 2);
    }

    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onFishingQuickSellRequest(
    txn: *Transaction(pb.FishingQuickSellRequest),
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    var total_coins: i32 = 0;
    for (progress_comp.info.fishing.cabin_items) |rec| {
        total_coins += sellFishValue(assets, rec);
    }
    if (progress_comp.info.fishing.cabin_items.len != 0) {
        alloc.gpa.free(progress_comp.info.fishing.cabin_items);
        progress_comp.info.fishing.cabin_items = &.{};
        progress_comp.info.fishing.total_sold += total_coins;
        try progress_comp.save(fs, alloc.arena);
    }
    if (total_coins > 0) {
        _ = try RewardService.grantItems(.{
            .alloc = alloc,
            .io = io,
            .fs = fs,
            .assets = assets,
            .conn = txn.conn,
            .basic_comp = basic_comp,
            .inventory_comp = inventory_comp,
            .weapon_comp = weapon_comp,
            .echo_comp = echo_comp,
        }, 0, false, &.{.{ .item_id = 80100001, .count = total_coins }}, 2);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onFishingInfoRequest(
    txn: *Transaction(pb.FishingInfoRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
    progress_comp: *const PlayerProgressComponent,
) !void {
    var ports: std.ArrayList(i32) = .empty;
    try ports.appendSlice(alloc.arena, &.{ 1, 2, 3, 4 });
    var skins: std.ArrayList(i32) = .empty;
    const eq_skin: i32 = @intCast(@max(1, progress_comp.info.getKeyInt("fish:ship_skin")));
    try skins.append(alloc.arena, 1);
    if (eq_skin != 1) try skins.append(alloc.arena, eq_skin);

    var techs: std.ArrayList(pb.FishingTechInfo) = .empty;
    for (assets.tables.fishing_tech.items) |row| {
        const lvl = ProgressInfo.getIdCount(progress_comp.info.fishing.tech_levels, row.Id);
        const pre_unlocked = row.PreNode == 0 or ProgressInfo.getIdCount(progress_comp.info.fishing.tech_levels, row.PreNode) > 0;
        try techs.append(alloc.arena, .{
            .NodeId = row.Id,
            .Level = lvl,
            .CanUnlock = pre_unlocked,
        });
    }

    var ship_items: std.ArrayList(pb.FishingItemInfo) = .empty;
    for (progress_comp.info.fishing.cabin_items) |rec| {
        if (rec.cabin_type == @intFromEnum(pb.CabinType.ShipCabin)) {
            try ship_items.append(alloc.arena, cabinItemToPb(rec));
        }
    }

    txn.respond(.{
        .FishingInfo = .{
            .TraceEntrusts = @intCast(@max(0, progress_comp.info.getKeyInt("fish:trace_entrust"))),
            .FishingTech = techs,
            .ShipInfo = .{
                .SkinId = eq_skin,
                .PortId = @intCast(@max(1, progress_comp.info.getKeyInt("fish:port"))),
                .IsInPort = progress_comp.info.getKeyInt("fish:sailing") == 0,
                .CabinInfo = .{
                    .CabinShape = 700001,
                    .QuickSellShape = 700005,
                    .FishingItem = ship_items,
                },
            },
            .IllustratedInfo = .default,
            .UnlockPortId = ports,
            .UnlockSkins = skins,
            .EntrustRefreshRatio = 100,
        },
    });
}

pub fn onFishingActivityLimitRewardRequest(
    txn: *Transaction(pb.FishingActivityLimitRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "fish:lim:{d}", .{txn.message.TaskId});
    if (try progress_comp.info.markClaim(alloc.gpa, k)) {
        try progress_comp.save(fs, alloc.arena);
        _ = try RewardService.grantTableReward(reward_ctx, "FishingActivityLimitTask", txn.message.TaskId, false, true, 1, &.{.{ .item_id = 3, .count = 40 }}, null);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onFishingActivityMilestoneRewardRequest(
    txn: *Transaction(pb.FishingActivityMilestoneRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var kbuf: [48]u8 = undefined;
    var dirty = false;
    for (txn.message.MilestoneId.items) |mid| {
        const k = try std.fmt.bufPrint(&kbuf, "fish:ms:{d}", .{mid});
        if (try progress_comp.info.markClaim(alloc.gpa, k)) {
            dirty = true;
            _ = try RewardService.grantTableReward(reward_ctx, "FishingActivityMilestone", mid, false, true, 1, &.{.{ .item_id = 3, .count = 60 }}, null);
        }
    }
    if (dirty) try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onFishingIllustratedRewardRequest(
    txn: *Transaction(pb.FishingIllustratedRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "fish:ill:{d}", .{txn.message.RewardId});
    if (try progress_comp.info.markClaim(alloc.gpa, k)) {
        try progress_comp.save(fs, alloc.arena);
        _ = try RewardService.grantTableReward(reward_ctx, "FishingIllustratedReward", txn.message.RewardId, false, true, 1, &.{.{ .item_id = 3, .count = 30 }}, null);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onFishingCompleteRequest(
    txn: *Transaction(pb.FishingCompleteRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    io: std.Io,
) !void {
    const pt_key = try std.fmt.allocPrint(alloc.arena, "fish:point_cnt:{d}", .{txn.message.EntityId});
    const pt_cnt = try progress_comp.info.addKeyInt(alloc.gpa, pt_key, 1);
    const total_cnt = try progress_comp.info.addKeyInt(alloc.gpa, "fish:catch_cnt", 1);

    const fish = pickFishForPoint(assets, alloc.arena, io, txn.message.EntityId, pt_cnt + total_cnt);
    const incr = progress_comp.info.fishing.next_fish_incr_id;
    progress_comp.info.fishing.next_fish_incr_id += 1;

    const rec: ProgressInfo.FishingCabinItem = .{
        .cabin_type = @intFromEnum(pb.CabinType.ShipCabin),
        .id = 1,
        .incr_id = incr,
        .item_id = fish.item_id,
        .size = fish.size,
        .quality = fish.quality,
        .price = fish.price,
    };
    _ = try ProgressInfo.appendItem(ProgressInfo.FishingCabinItem, &progress_comp.info.fishing.cabin_items, alloc.gpa, rec);
    try progress_comp.save(fs, alloc.arena);

    var items: std.ArrayList(pb.FishingItemInfo) = .empty;
    try items.append(alloc.arena, cabinItemToPb(rec));
    txn.respond(.{ .ErrorCode = .Success, .Items = items });
}

pub fn onFishingEntrustAcceptRequest(
    txn: *Transaction(pb.FishingEntrustAcceptRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "fish:entrust_acc:{d}", .{txn.message.EntrustId});
    try progress_comp.info.setKeyInt(alloc.gpa, k, if (txn.message.Accept) 1 else 0);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onFishingEntrustHandInRequest(
    txn: *Transaction(pb.FishingEntrustHandInRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
    qctx: QuestEngine.QuestContext,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "fish:entrust_done:{d}", .{txn.message.EntrustId});
    if (try progress_comp.info.markClaim(alloc.gpa, k)) {
        try progress_comp.save(fs, alloc.arena);
        _ = try RewardService.grantTableReward(reward_ctx, "FishingEntrust", txn.message.EntrustId, false, true, 1, &.{.{ .item_id = 80100001, .count = 200 }}, null);
    }
    txn.respond(.{ .ErrorCode = .Success });
    try QuestEngine.onConditionStateChanged(qctx);
}

pub fn onFishingEntrustRefreshRequest(
    txn: *Transaction(pb.FishingEntrustRefreshRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const next_id = try progress_comp.info.addKeyInt(alloc.gpa, "fish:entrust_refresh", 1);
    try progress_comp.save(fs, alloc.arena);
    const out_id: i32 = if (txn.message.EntrustId > 0) txn.message.EntrustId else @intCast(@mod(next_id - 1, 5) + 1);
    txn.respond(.{ .ErrorCode = .Success, .EntrustId = out_id });
}
pub fn onFishingEntrustTraceRequest(
    txn: *Transaction(pb.FishingEntrustTraceRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "fish:trace_entrust", txn.message.EntrustId);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}
pub fn onFishingExitRequest(
    txn: *Transaction(pb.FishingExitRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "fish:sailing", 0);
    try progress_comp.save(fs, alloc.arena);
    _ = txn.message;
    txn.respond(.{});
}
pub fn onFishingHandInRequest(
    txn: *Transaction(pb.FishingHandInRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const k = try std.fmt.allocPrint(alloc.arena, "fish:handin:{d}", .{txn.message.Id});
    _ = try progress_comp.info.addKeyInt(alloc.gpa, k, 1);
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "fish:sell_cnt", 1);
    try progress_comp.save(fs, alloc.arena);
    const total_cnt: i32 = @intCast(@max(1, txn.message.LeftFishingItems.items.len + txn.message.RightFishingItems.items.len));
    _ = try RewardService.grantItems(reward_ctx, 0, true, &.{
        .{ .item_id = 3, .count = 20 },
        .{ .item_id = 2, .count = 2000 * total_cnt },
    }, 1);
    txn.respond(.{ .ErrorCode = .Success, .HandInComplete = true });
}
pub fn onFishingPointInfoRequest(
    txn: *Transaction(pb.FishingPointInfoRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    const k = try std.fmt.allocPrint(alloc.arena, "fish:point_cnt:{d}", .{txn.message.Id});
    const used = progress_comp.info.getKeyInt(k);
    const remain: i32 = @intCast(@max(0, 5 - @min(used, 5)));
    txn.respond(.{ .ErrorCode = .Success, .FishPointInfo = .{ .Id = txn.message.Id, .EntityConfigId = txn.message.Id, .CurCount = remain, .MaxCount = 5, .RefreshTime = 60 } });
}
pub fn onFishingCageInfoRequest(
    txn: *Transaction(pb.FishingCageInfoRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    io: std.Io,
) !void {
    var entity_cfg_id: i32 = txn.message.Id;
    var max_count: i32 = 5;
    var refresh_time: i32 = 120;
    if (loadJsonArray(alloc.arena, io, "assets/BinData/FishingCage.json")) |cages| {
        for (cages) |entry| {
            const obj = switch (entry) {
                .object => |o| o,
                else => continue,
            };
            const cid: i64 = switch (obj.get("Id") orelse continue) {
                .integer => |v| v,
                else => continue,
            };
            const ecid: i64 = switch (obj.get("EntityConfigId") orelse continue) {
                .integer => |v| v,
                else => continue,
            };
            if (cid == txn.message.Id or ecid == txn.message.Id) {
                entity_cfg_id = @intCast(std.math.clamp(ecid, 0, std.math.maxInt(i32)));
                if (obj.get("CountLimit")) |cl| {
                    if (cl == .integer and cl.integer > 0) {
                        max_count = @intCast(std.math.clamp(cl.integer, 1, std.math.maxInt(i32)));
                    }
                }
                if (obj.get("RefreshTime")) |rt| {
                    if (rt == .integer and rt.integer > 0) {
                        refresh_time = @intCast(std.math.clamp(rt.integer, 1, std.math.maxInt(i32)));
                    }
                }
                break;
            }
        }
    }

    const cage_type = @intFromEnum(pb.CabinType.Cage);
    var items: std.ArrayList(pb.FishingItemInfo) = .empty;
    for (progress_comp.info.fishing.cabin_items) |rec| {
        if (rec.cabin_type == cage_type and rec.id == txn.message.Id) {
            try items.append(alloc.arena, cabinItemToPb(rec));
        }
    }
    if (items.items.len == 0 and assets.tables.fishing_item.items.len > 0) {
        const seed = progress_comp.info.getKeyInt("fish:catch_cnt");
        const count = @min(@as(usize, 2), @as(usize, @intCast(max_count)));
        for (0..count) |i| {
            const fish = pickFishForPoint(assets, alloc.arena, io, entity_cfg_id, seed + @as(i64, @intCast(i)));
            const incr = progress_comp.info.fishing.next_fish_incr_id;
            progress_comp.info.fishing.next_fish_incr_id += 1;
            const rec: ProgressInfo.FishingCabinItem = .{
                .cabin_type = cage_type,
                .id = txn.message.Id,
                .incr_id = incr,
                .item_id = fish.item_id,
                .size = fish.size,
                .quality = fish.quality,
                .price = fish.price,
            };
            _ = try ProgressInfo.appendItem(ProgressInfo.FishingCabinItem, &progress_comp.info.fishing.cabin_items, alloc.gpa, rec);
            try items.append(alloc.arena, cabinItemToPb(rec));
        }
        try progress_comp.save(fs, alloc.arena);
    }

    txn.respond(.{
        .ErrorCode = .Success,
        .FishCageInfo = .{
            .Id = txn.message.Id,
            .EntityConfigId = entity_cfg_id,
            .MaxCount = max_count,
            .RefreshTime = refresh_time,
            .Items = items,
        },
    });
}
pub fn onFishingSailingRequest(
    txn: *Transaction(pb.FishingSailingRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "fish:port", @max(1, txn.message.PortId));
    try progress_comp.info.setKeyInt(alloc.gpa, "fish:sailing", 1);
    if (txn.message.Fix) try progress_comp.info.setKeyInt(alloc.gpa, "fish:ship_dmg", 0);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}
pub fn onFishingShipFixRequest(
    txn: *Transaction(pb.FishingShipFixRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "fish:ship_dmg", 0);
    try progress_comp.save(fs, alloc.arena);
    _ = txn.message;
    txn.respond(.{ .ErrorCode = .Success });
}
pub fn onFishingShipSkinChangeRequest(
    txn: *Transaction(pb.FishingShipSkinChangeRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "fish:ship_skin", txn.message.SkinId);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}
pub fn onFishingSkillBaitRequest(
    txn: *Transaction(pb.FishingSkillBaitRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "fish:active_bait", txn.message.ItemId);
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "fish:skill_use", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}
pub fn onFishingSkillBombRequest(
    txn: *Transaction(pb.FishingSkillBombRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    progress_comp: *PlayerProgressComponent,
    io: std.Io,
) !void {
    if (txn.message.ItemId > 0) {
        const costs = [_]ConsumeHelper.CostItem{.{ .id = txn.message.ItemId, .count = 1 }};
        if (!(try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, &costs, io))) {
            txn.respond(.{ .ErrorCode = .ErrItemNotEnough });
            return;
        }
    }
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "fish:skill_use", 1);
    const base_cnt = try progress_comp.info.addKeyInt(alloc.gpa, "fish:catch_cnt", 2);
    var items: std.ArrayList(pb.FishingItemInfo) = .empty;
    for (0..2) |i| {
        const fish = pickFishForPoint(assets, alloc.arena, io, 0, base_cnt + @as(i64, @intCast(i)));
        const incr = progress_comp.info.fishing.next_fish_incr_id;
        progress_comp.info.fishing.next_fish_incr_id += 1;
        const rec: ProgressInfo.FishingCabinItem = .{
            .cabin_type = @intFromEnum(pb.CabinType.ShipCabin),
            .id = 1,
            .incr_id = incr,
            .item_id = fish.item_id,
            .size = 12,
            .quality = fish.quality,
            .price = fish.price,
        };
        _ = try ProgressInfo.appendItem(ProgressInfo.FishingCabinItem, &progress_comp.info.fishing.cabin_items, alloc.gpa, rec);
        try items.append(alloc.arena, cabinItemToPb(rec));
    }
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success, .Items = items });
}
pub fn onFishingSkillTeleportRequest(
    txn: *Transaction(pb.FishingSkillTeleportRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "fish:skill_use", 1);
    try progress_comp.save(fs, alloc.arena);
    _ = txn.message;
    txn.respond(.{ .ErrorCode = .Success });
}
pub fn onFishingTeleportToBoatRequest(
    txn: *Transaction(pb.FishingTeleportToBoatRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "fish:boat_tp", 1);
    try progress_comp.save(fs, alloc.arena);
    _ = txn.message;
    txn.respond(.{ .ErrorCode = .Success });
}
pub fn onFishingQuickSellPreviewRequest(
    txn: *Transaction(pb.FishingQuickSellPreviewRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
    progress_comp: *const PlayerProgressComponent,
) !void {
    var total: i32 = 0;
    for (progress_comp.info.fishing.cabin_items) |rec| total += sellFishValue(assets, rec);
    var rwds: std.ArrayList(pb.MapEntry(i32, i32)) = .empty;
    if (total > 0) try rwds.append(alloc.arena, .{ .key = 80100001, .value = total });
    txn.respond(.{ .ErrorCode = .Success, .RewardItems = rwds });
}
pub fn onFishingSellPreviewRequest(
    txn: *Transaction(pb.FishingSellPreviewRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
    progress_comp: *const PlayerProgressComponent,
) !void {
    var total: i32 = 0;
    for (progress_comp.info.fishing.cabin_items) |rec| {
        if (std.mem.indexOfScalar(i32, txn.message.IncrIds.items, rec.incr_id) != null) {
            total += sellFishValue(assets, rec);
        }
    }
    var rwds: std.ArrayList(pb.MapEntry(i32, i32)) = .empty;
    if (total > 0) try rwds.append(alloc.arena, .{ .key = 80100001, .value = total });
    txn.respond(.{ .ErrorCode = .Success, .RewardItems = rwds });
}
