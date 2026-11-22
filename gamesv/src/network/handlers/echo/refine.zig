const std = @import("std");
const pb = @import("proto").pb;
const Transaction = @import("../../handlers.zig").Transaction;
const mem = @import("../../../mem.zig");
const FileSystem = @import("common").FileSystem;
const Assets = @import("../../../data/Assets.zig");
const comp_util = @import("../../../logic/component/comp_util.zig");
const PlayerBasicComponent = @import("../../../logic/component/player/PlayerBasicComponent.zig");
const PlayerEchoComponent = @import("../../../logic/component/player/PlayerEchoComponent.zig");
const PlayerInventoryComponent = @import("../../../logic/component/player/PlayerInventoryComponent.zig");
const PlayerWeaponComponent = @import("../../../logic/component/player/PlayerWeaponComponent.zig");
const EchoInfo = @import("../../../fs/EchoInfo.zig");
const special_item_incr = @import("../../../fs/special_item_incr.zig");
const CalabashHelper = @import("../../../logic/helpers/calabash.zig");
const RewardService = @import("../../../logic/helpers/reward.zig");

const RefineResult = struct {
    items: std.ArrayList(pb.AddCountItemInfo) = .empty,
    extra_items: std.ArrayList(pb.AddCountItemInfo) = .empty,
    cost_phantoms: std.ArrayList(pb.PhantomItem) = .empty,
};

pub fn onPhantomRefiningRequest(
    txn: *Transaction(pb.PhantomRefiningRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    io: std.Io,
) !void {
    const result = try executeRefine(
        txn.conn,
        alloc,
        fs,
        assets,
        basic_comp,
        inventory_comp,
        weapon_comp,
        echo_comp,
        txn.message.StackEntries.items,
        null,
        io,
    );
    switch (result) {
        .err => |code| txn.respond(.{ .ErrorCode = code }),
        .ok => |res| txn.respond(.{
            .ErrorCode = .Success,
            .Items = res.items,
            .ExtraItems = res.extra_items,
            .CostPhantoms = res.cost_phantoms,
        }),
    }
}

pub fn onPhantomBatchRefiningRequest(
    txn: *Transaction(pb.PhantomBatchRefiningRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    io: std.Io,
) !void {
    const result = try executeRefine(
        txn.conn,
        alloc,
        fs,
        assets,
        basic_comp,
        inventory_comp,
        weapon_comp,
        echo_comp,
        txn.message.StackEntries.items,
        null,
        io,
    );
    switch (result) {
        .err => |code| txn.respond(.{ .ErrorCode = code }),
        .ok => |res| txn.respond(.{
            .ErrorCode = .Success,
            .Items = res.items,
            .ExtraItems = res.extra_items,
            .CostPhantoms = res.cost_phantoms,
        }),
    }
}

pub fn onCalabashBatchRefiningRequest(
    txn: *Transaction(pb.CalabashBatchRefiningRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    io: std.Io,
) !void {
    const result = try executeRefine(
        txn.conn,
        alloc,
        fs,
        assets,
        basic_comp,
        inventory_comp,
        weapon_comp,
        echo_comp,
        txn.message.StackEntries.items,
        null,
        io,
    );
    switch (result) {
        .err => |code| txn.respond(.{ .ErrorCode = code }),
        .ok => |res| txn.respond(.{
            .ErrorCode = .Success,
            .Items = res.items,
            .ExtraItems = res.extra_items,
            .CostPhantoms = res.cost_phantoms,
        }),
    }
}

pub fn onPhantomBatchDirectRefiningRequest(
    txn: *Transaction(pb.PhantomBatchDirectRefiningRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    io: std.Io,
) !void {
    if (txn.message.TargetFetterGroupId <= 0 or !hasFetterCandidate(assets, txn.message.TargetFetterGroupId)) {
        txn.respond(.{
            .ErrorCode = .ErrPhantomConfigNotFound,
            .DirectRefineWeekTimes = echo_comp.calabash_info.direct_refine_week_times,
        });
        return;
    }
    const requested_batches: i32 = @intCast(txn.message.StackEntries.items.len / 5);
    if (requested_batches > 0 and echo_comp.calabash_info.direct_refine_week_times + requested_batches > 10) {
        txn.respond(.{
            .ErrorCode = .ErrPhantomRefiningMaxCount,
            .DirectRefineWeekTimes = echo_comp.calabash_info.direct_refine_week_times,
        });
        return;
    }

    const result = try executeRefine(
        txn.conn,
        alloc,
        fs,
        assets,
        basic_comp,
        inventory_comp,
        weapon_comp,
        echo_comp,
        txn.message.StackEntries.items,
        txn.message.TargetFetterGroupId,
        io,
    );
    switch (result) {
        .err => |code| txn.respond(.{
            .ErrorCode = code,
            .DirectRefineWeekTimes = echo_comp.calabash_info.direct_refine_week_times,
        }),
        .ok => |res| txn.respond(.{
            .ErrorCode = .Success,
            .Items = res.items,
            .CostPhantoms = res.cost_phantoms,
            .DirectRefineWeekTimes = echo_comp.calabash_info.direct_refine_week_times,
        }),
    }
}

const RefineOutcome = union(enum) {
    err: pb.ErrorCode,
    ok: RefineResult,
};

fn executeRefine(
    conn: anytype,
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    entries: []const pb.PhantomStackConsumeEntry,
    target_fetter: ?i32,
    io: std.Io,
) !RefineOutcome {
    if (entries.len < 5 or entries.len % 5 != 0) {
        return .{ .err = .ErrPhantomRefiningCount };
    }
    if (entries.len > 100) {
        return .{ .err = .ErrPhantomRefiningMaxCount };
    }

    var seen: std.AutoHashMapUnmanaged(i32, void) = .empty;
    defer seen.deinit(alloc.gpa);

    for (entries) |entry| {
        if (entry.Count != 1) return .{ .err = .ErrPhantomConsumeItemCount };
        if (seen.contains(entry.IncrId)) return .{ .err = .ErrPhantomRefiningDulplicate };
        try seen.put(alloc.gpa, entry.IncrId, {});

        const item = echo_comp.echo_map.get(entry.IncrId) orelse return .{ .err = .ErrPhantomItemNotExist };
        if (item.func_value == 1) return .{ .err = .ErrPhantomLevelUpMaterialLock };
        if (item.role_id != null) return .{ .err = .ErrPhantomDecomposeEquiped };
        if (item.level != 0 or item.exp != 0) return .{ .err = .ErrPhantomRefiningDeveloped };
    }

    var seed_bytes: [8]u8 = undefined;
    std.Io.random(fs.io, &seed_bytes);
    var prng = std.Random.DefaultPrng.init(std.mem.readInt(u64, &seed_bytes, .little));
    const rng = prng.random();

    const calabash_lvl = CalabashHelper.currentLevel(assets, basic_comp, echo_comp);
    const calabash_cfg = assets.tables.calabash_level.getDataById(calabash_lvl);

    var res: RefineResult = .{};
    var removed_ids: std.ArrayList(i32) = .empty;
    for (entries) |entry| {
        const inc_id = entry.IncrId;
        if (echo_comp.echo_map.fetchSwapRemove(inc_id)) |kv| {
            var old = kv.value;
            try res.cost_phantoms.append(alloc.arena, try old.toProto(inc_id, alloc.arena));
            old.deinit(alloc.gpa);
        }
        try PlayerEchoComponent.deleteEchoFile(alloc.gpa, fs, echo_comp.player_id, inc_id);
        try removed_ids.append(alloc.arena, inc_id);
    }

    var added_protos: std.ArrayList(pb.PhantomItem) = .empty;
    var bonus_union_exp: i32 = 0;
    var calabash_dirty = false;

    const refine_batches = entries.len / 5;
    var batch: usize = 0;
    while (batch < refine_batches) : (batch += 1) {
        const main_quality = rollQuality(calabash_cfg, rng);
        const produced = pickEchoCandidate(assets, echo_comp, main_quality, target_fetter, rng) orelse break;
        const main_entry = try grantRefinedEcho(alloc, fs, assets, echo_comp, produced, target_fetter, rng);
        try added_protos.append(alloc.arena, main_entry.proto);
        try res.items.append(alloc.arena, .{
            .Id = produced.ItemId,
            .Count = 1,
            .IncrId = main_entry.incr_id,
        });
        if (main_entry.discovery.changed) calabash_dirty = true;
        bonus_union_exp += main_entry.discovery.union_exp;

        if (target_fetter == null and calabash_cfg != null and calabash_cfg.?.TempCatchGain > 0) {
            if (rng.uintLessThan(u32, 1000) < @as(u32, @intCast(calabash_cfg.?.TempCatchGain))) {
                const extra_quality = rollQuality(calabash_cfg, rng);
                if (pickEchoCandidate(assets, echo_comp, extra_quality, null, rng)) |extra_cfg| {
                    const extra_entry = try grantRefinedEcho(alloc, fs, assets, echo_comp, extra_cfg, null, rng);
                    try added_protos.append(alloc.arena, extra_entry.proto);
                    try res.extra_items.append(alloc.arena, .{
                        .Id = extra_cfg.ItemId,
                        .Count = 1,
                        .IncrId = extra_entry.incr_id,
                    });
                    if (extra_entry.discovery.changed) calabash_dirty = true;
                    bonus_union_exp += extra_entry.discovery.union_exp;
                }
            }
        }
    }

    if (target_fetter != null and refine_batches > 0) {
        echo_comp.calabash_info.direct_refine_week_times += @intCast(refine_batches);
        calabash_dirty = true;
    }

    try conn.push(alloc.gpa, io, pb.PhantomItemRemoveNotify{
        .PhantomItemIncrIdList = removed_ids,
    });
    if (added_protos.items.len > 0) {
        try conn.push(alloc.gpa, io, pb.PhantomItemAddNotify{
            .PhantomItemList = added_protos,
        });
    }
    if (bonus_union_exp > 0) {
        const union_rewards = [_]RewardService.RewardItem{.{ .item_id = 1, .count = bonus_union_exp }};
        _ = try RewardService.grantItems(.{
            .alloc = alloc,
            .io = io,
            .fs = fs,
            .assets = assets,
            .conn = conn,
            .basic_comp = basic_comp,
            .inventory_comp = inventory_comp,
            .weapon_comp = weapon_comp,
            .echo_comp = echo_comp,
        }, 0, false, &union_rewards, 0);
    }
    if (calabash_dirty) {
        try PlayerEchoComponent.saveCalabash(alloc.gpa, fs, echo_comp.player_id, echo_comp.calabash_info);
        try conn.push(alloc.gpa, io, try CalabashHelper.buildMsgNotify(alloc.arena, assets, basic_comp, echo_comp));
    }

    return .{ .ok = res };
}

fn grantRefinedEcho(
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    echo_comp: *PlayerEchoComponent,
    cfg: Assets.DataTables.PhantomItem,
    forced_fetter: ?i32,
    rng: std.Random,
) !struct { incr_id: i32, proto: pb.PhantomItem, discovery: CalabashHelper.DiscoveryOutcome } {
    const fetter = forced_fetter orelse cfg.FetterGroup[rng.uintLessThan(usize, cfg.FetterGroup.len)];
    const incr_id = try special_item_incr.next(alloc.gpa, fs, echo_comp.player_id);
    const echo_item = try EchoInfo.buildDefaultEcho(alloc.gpa, assets, cfg, fetter);
    try echo_comp.echo_map.put(alloc.gpa, incr_id, echo_item);
    const e_path = try std.fmt.allocPrint(alloc.arena, "player/{}/{s}/{}", .{ echo_comp.player_id, EchoInfo.data_dir, incr_id });
    try comp_util.saveStruct(fs, echo_item, e_path, alloc.arena);
    const discovery = try CalabashHelper.recordEchoDiscovery(alloc.gpa, assets, echo_comp, cfg.MonsterId, cfg.QualityId);
    return .{
        .incr_id = incr_id,
        .proto = try echo_item.toProto(incr_id, alloc.arena),
        .discovery = discovery,
    };
}

fn rollQuality(calabash_cfg: ?Assets.DataTables.CalabashLevel, rng: std.Random) i32 {
    const cfg = calabash_cfg orelse return 2;
    var total: i32 = 0;
    var it = cfg.QualityDropWeight.map.iterator();
    while (it.next()) |kv| {
        if (kv.value_ptr.* > 0) total += kv.value_ptr.*;
    }
    if (total <= 0) return 2;
    var roll: i32 = @intCast(rng.uintLessThan(u32, @intCast(total)));
    it = cfg.QualityDropWeight.map.iterator();
    while (it.next()) |kv| {
        const w = kv.value_ptr.*;
        if (w <= 0) continue;
        if (roll < w) return kv.key_ptr.*;
        roll -= w;
    }
    return 2;
}

fn hasFetterCandidate(assets: *const Assets, fetter_id: i32) bool {
    for (assets.tables.phantom_item.items) |item| {
        if (!EchoInfo.validDefaultOwnedItem(assets, item)) continue;
        if (containsFetter(item.FetterGroup, fetter_id)) return true;
    }
    return false;
}

fn containsFetter(fetter_group: []const i32, fetter_id: i32) bool {
    for (fetter_group) |f| {
        if (f == fetter_id) return true;
    }
    return false;
}

fn pickEchoCandidate(
    assets: *const Assets,
    echo_comp: *const PlayerEchoComponent,
    quality: i32,
    target_fetter: ?i32,
    rng: std.Random,
) ?Assets.DataTables.PhantomItem {
    // Prefer echoes matching requested quality and already discovered in Calabash if available
    var count: usize = 0;
    for (assets.tables.phantom_item.items) |item| {
        if (!EchoInfo.validDefaultOwnedItem(assets, item)) continue;
        if (item.QualityId != quality) continue;
        if (target_fetter) |fetter| {
            if (!containsFetter(item.FetterGroup, fetter)) continue;
        }
        if (CalabashHelper.discoveredMaxQuality(assets, echo_comp, item.MonsterId) > 0) {
            count += 1;
        }
    }
    if (count > 0) {
        var idx = rng.uintLessThan(usize, count);
        for (assets.tables.phantom_item.items) |item| {
            if (!EchoInfo.validDefaultOwnedItem(assets, item)) continue;
            if (item.QualityId != quality) continue;
            if (target_fetter) |fetter| {
                if (!containsFetter(item.FetterGroup, fetter)) continue;
            }
            if (CalabashHelper.discoveredMaxQuality(assets, echo_comp, item.MonsterId) > 0) {
                if (idx == 0) return item;
                idx -= 1;
            }
        }
    }

    var fallback_count: usize = 0;
    for (assets.tables.phantom_item.items) |item| {
        if (!EchoInfo.validDefaultOwnedItem(assets, item)) continue;
        if (item.QualityId != quality) continue;
        if (target_fetter) |fetter| {
            if (!containsFetter(item.FetterGroup, fetter)) continue;
        }
        fallback_count += 1;
    }
    if (fallback_count == 0) return null;
    var idx = rng.uintLessThan(usize, fallback_count);
    for (assets.tables.phantom_item.items) |item| {
        if (!EchoInfo.validDefaultOwnedItem(assets, item)) continue;
        if (item.QualityId != quality) continue;
        if (target_fetter) |fetter| {
            if (!containsFetter(item.FetterGroup, fetter)) continue;
        }
        if (idx == 0) return item;
        idx -= 1;
    }
    return null;
}
