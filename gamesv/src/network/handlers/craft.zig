const std = @import("std");
const common = @import("common");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const Transaction = @import("../handlers.zig").Transaction;
const Assets = @import("../../data/Assets.zig");
const ProgressInfo = @import("../../fs/ProgressInfo.zig");
const PlayerProgressComponent = @import("../../logic/component/player/PlayerProgressComponent.zig");
const PlayerBasicComponent = @import("../../logic/component/player/PlayerBasicComponent.zig");
const PlayerInventoryComponent = @import("../../logic/component/player/PlayerInventoryComponent.zig");
const PlayerWeaponComponent = @import("../../logic/component/player/PlayerWeaponComponent.zig");
const PlayerEchoComponent = @import("../../logic/component/player/PlayerEchoComponent.zig");
const ConsumeHelper = @import("../../logic/helpers/consume.zig");
const CraftHelper = @import("../../logic/helpers/craft.zig");
const QuestEngine = @import("../../logic/helpers/quest_engine.zig");

const FileSystem = common.FileSystem;

pub fn onForgeInfoRequest(
    txn: *Transaction(pb.ForgeInfoRequest),
    assets: *const Assets,
    progress_comp: *const PlayerProgressComponent,
    alloc: mem.Alloc,
) !void {
    var list: std.ArrayList(pb.OneForgeInfo) = .empty;
    var configs: std.ArrayList(pb.OneForgeConfig) = .empty;
    for (assets.tables.forge_formula.items) |row| {
        try configs.append(alloc.arena, .{ .Id = row.Id });
        const rec = ProgressInfo.findIdRoleCount(progress_comp.info.craft.forge_counts, row.Id);
        const unlocked = row.Unlock or ProgressInfo.containsId(progress_comp.info.craft.unlocked_forge, row.Id);
        if (unlocked or rec != null) {
            try list.append(alloc.arena, .{
                .Id = row.Id,
                .LastRoleId = if (rec) |r| r.last_role_id else 0,
            });
        }
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .ForgeInfoList = list,
        .ForgeConfigs = configs,
    });
}

pub fn onForgeFormulaUnlockRequest(
    txn: *Transaction(pb.ForgeFormulaUnlockRequest),
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    const id = txn.message.Id;
    const row = assets.tables.forge_formula.getDataById(id) orelse {
        txn.respond(.{ .Code = .ErrForgeConfigNotFound, .Id = id });
        return;
    };
    if (row.Unlock or ProgressInfo.containsId(progress_comp.info.craft.unlocked_forge, id)) {
        txn.respond(.{ .Code = .ErrForgeUnlocked, .Id = id });
        return;
    }
    if (row.FormulaItemId > 0) {
        const costs = [_]ConsumeHelper.CostItem{.{ .id = row.FormulaItemId, .count = 1 }};
        if (!(try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, &costs, io))) {
            txn.respond(.{ .Code = .ErrItemNotEnough, .Id = id });
            return;
        }
    }
    _ = try ProgressInfo.appendUniqueId(&progress_comp.info.craft.unlocked_forge, alloc.gpa, id);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .Code = .Success, .Id = id });
}

pub fn onForgeItemRequest(
    txn: *Transaction(pb.ForgeItemRequest),
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    qctx: QuestEngine.QuestContext,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    const req = txn.message;
    const count = req.Count;
    if (count <= 0) {
        txn.respond(.{ .Code = .ErrItemCount, .Id = req.Id, .RoleId = req.RoleId });
        return;
    }
    const row = assets.tables.forge_formula.getDataById(req.Id) orelse {
        txn.respond(.{ .Code = .ErrForgeConfigNotFound, .Id = req.Id, .RoleId = req.RoleId });
        return;
    };
    const unlocked = row.Unlock or ProgressInfo.containsId(progress_comp.info.craft.unlocked_forge, row.Id);
    if (!unlocked) {
        txn.respond(.{ .Code = .ErrForgeLocked, .Id = req.Id, .RoleId = req.RoleId });
        return;
    }

    const costs = CraftHelper.buildScaledCosts(alloc.arena, row.ConsumeItems, count) catch {
        txn.respond(.{ .Code = .ErrItemCount, .Id = req.Id, .RoleId = req.RoleId });
        return;
    };
    const outcome = try CraftHelper.consumeAndGrant(.{
        .conn = txn.conn,
        .alloc = alloc,
        .io = io,
        .fs = fs,
        .assets = assets,
        .basic_comp = basic_comp,
        .inventory_comp = inventory_comp,
        .weapon_comp = weapon_comp,
        .echo_comp = echo_comp,
    }, costs, row.ItemId, count, 2);

    if (!outcome.ok) {
        txn.respond(.{ .Code = outcome.err, .Id = req.Id, .RoleId = req.RoleId });
        return;
    }

    try ProgressInfo.addIdRoleCount(&progress_comp.info.craft.forge_counts, alloc.gpa, row.Id, count, req.RoleId);
    try progress_comp.save(fs, alloc.arena);
    QuestEngine.onUiAction(qctx, "Forging") catch |err| QuestEngine.logHookErr("onUiAction(Forging)", err);

    var upd_list: std.ArrayList(pb.OneForgeInfo) = .empty;
    try upd_list.append(alloc.arena, .{ .Id = row.Id, .LastRoleId = req.RoleId });
    try txn.conn.push(alloc.gpa, io, pb.ForgeItemInfoUpdateNotify{ .ForgeInfoList = upd_list });

    txn.respond(.{
        .Code = .Success,
        .Id = row.Id,
        .RoleId = req.RoleId,
        .ItemInfos = outcome.items,
    });
}

fn buildSynthInfoPb(row: Assets.DataTables.SynthesisFormula, rec_opt: ?ProgressInfo.IdRoleCount) pb.OneSynthesisInfo {
    const cnt = if (rec_opt) |r| r.count else 0;
    return .{
        .Id = row.Id,
        .Count = cnt,
        .LastRoleId = if (rec_opt) |r| r.last_role_id else 0,
        .LimitCount = row.LimitCount,
        .LimitSynthesisCount = cnt,
    };
}

pub fn onSynthesisInfoRequest(
    txn: *Transaction(pb.SynthesisInfoRequest),
    assets: *const Assets,
    progress_comp: *const PlayerProgressComponent,
    alloc: mem.Alloc,
) !void {
    var infos: std.ArrayList(pb.OneSynthesisInfo) = .empty;
    var configs: std.ArrayList(pb.OneSynthesisConfig) = .empty;
    for (assets.tables.synthesis_formula.items) |row| {
        try configs.append(alloc.arena, .{ .Id = row.Id });
        const rec = ProgressInfo.findIdRoleCount(progress_comp.info.craft.synth_counts, row.Id);
        const unlocked = row.Unlock or ProgressInfo.containsId(progress_comp.info.craft.unlocked_synth, row.Id);
        if (unlocked or rec != null) {
            try infos.append(alloc.arena, buildSynthInfoPb(row, rec));
        }
    }
    txn.respond(.{
        .LevelInfo = .{
            .Level = progress_comp.info.craft.synth_level,
            .TotalProficiency = progress_comp.info.craft.synth_proficiency,
        },
        .SynthesisConfigs = configs,
        .Code = .Success,
        .SynthesisInfoList = infos,
    });
}

pub fn onSynthesisFormulaUnlockRequest(
    txn: *Transaction(pb.SynthesisFormulaUnlockRequest),
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    const id = txn.message.Id;
    const row = assets.tables.synthesis_formula.getDataById(id) orelse {
        txn.respond(.{ .Code = .ErrSynthesisConfigNotFound, .Id = id });
        return;
    };
    if (row.Unlock or ProgressInfo.containsId(progress_comp.info.craft.unlocked_synth, id)) {
        txn.respond(.{ .Code = .ErrSynthesisUnlocked, .Id = id });
        return;
    }
    if (row.FormulaItemId > 0) {
        const costs = [_]ConsumeHelper.CostItem{.{ .id = row.FormulaItemId, .count = 1 }};
        if (!(try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, &costs, io))) {
            txn.respond(.{ .Code = .ErrItemNotEnough, .Id = id });
            return;
        }
    }
    _ = try ProgressInfo.appendUniqueId(&progress_comp.info.craft.unlocked_synth, alloc.gpa, id);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .Code = .Success, .Id = id });
}

pub fn onSynthesisItemRequest(
    txn: *Transaction(pb.SynthesisItemRequest),
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    qctx: QuestEngine.QuestContext,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    const req = txn.message;
    const count = req.Count;
    if (count <= 0) {
        txn.respond(.{ .Code = .ErrItemCount, .Id = req.Id, .RoleId = req.RoleId });
        return;
    }
    const row = assets.tables.synthesis_formula.getDataById(req.Id) orelse {
        txn.respond(.{ .Code = .ErrSynthesisConfigNotFound, .Id = req.Id, .RoleId = req.RoleId });
        return;
    };
    const unlocked = row.Unlock or ProgressInfo.containsId(progress_comp.info.craft.unlocked_synth, row.Id);
    if (!unlocked) {
        txn.respond(.{ .Code = .ErrSynthesisLocked, .Id = req.Id, .RoleId = req.RoleId });
        return;
    }
    const prev_rec = ProgressInfo.findIdRoleCount(progress_comp.info.craft.synth_counts, row.Id);
    const prev_cnt: i32 = if (prev_rec) |r| r.count else 0;
    if (row.LimitCount > 0 and (std.math.add(i32, prev_cnt, count) catch std.math.maxInt(i32)) > row.LimitCount) {
        txn.respond(.{ .Code = .ErrSynthesisCountLimit, .Id = req.Id, .RoleId = req.RoleId });
        return;
    }

    const costs = CraftHelper.buildScaledCosts(alloc.arena, row.ConsumeItems, count) catch {
        txn.respond(.{ .Code = .ErrItemCount, .Id = req.Id, .RoleId = req.RoleId });
        return;
    };
    const outcome = try CraftHelper.consumeAndGrant(.{
        .conn = txn.conn,
        .alloc = alloc,
        .io = io,
        .fs = fs,
        .assets = assets,
        .basic_comp = basic_comp,
        .inventory_comp = inventory_comp,
        .weapon_comp = weapon_comp,
        .echo_comp = echo_comp,
    }, costs, row.ItemId, count, 2);

    if (!outcome.ok) {
        txn.respond(.{ .Code = outcome.err, .Id = req.Id, .RoleId = req.RoleId });
        return;
    }

    try ProgressInfo.addIdRoleCount(&progress_comp.info.craft.synth_counts, alloc.gpa, row.Id, count, req.RoleId);
    const prof_left = if (row.MaxProficiencyCount > 0) @max(0, row.MaxProficiencyCount - prev_cnt) else count;
    const prof_cnt = @min(count, prof_left);
    const prof_gain = if (row.Proficiency > 0 and prof_cnt > 0) row.Proficiency * prof_cnt else 0;
    const leveled = CraftHelper.addProficiencyAndUpdateLevel(
        &progress_comp.info.craft.synth_level,
        &progress_comp.info.craft.synth_proficiency,
        prof_gain,
        assets.tables.synthesis_level.items,
    );
    try progress_comp.save(fs, alloc.arena);
    QuestEngine.onUiAction(qctx, "Synthetic") catch |err| QuestEngine.logHookErr("onUiAction(Synthetic)", err);

    if (leveled) {
        try txn.conn.push(alloc.gpa, io, pb.SynthesisLevelUpdateNotify{
            .LevelInfo = .{
                .Level = progress_comp.info.craft.synth_level,
                .TotalProficiency = progress_comp.info.craft.synth_proficiency,
            },
        });
    }

    txn.respond(.{
        .Code = .Success,
        .Id = row.Id,
        .RoleId = req.RoleId,
        .ItemInfos = outcome.items,
    });
}

pub fn onBatchSynthesisItemRequest(
    txn: *Transaction(pb.BatchSynthesisItemRequest),
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
    var out_items: std.ArrayList(pb.MapEntry(i32, i32)) = .empty;
    var any_changed = false;
    var leveled = false;

    for (txn.message.BatchItems.items) |batch_entry| {
        if (batch_entry.Count <= 0) continue;
        const row = assets.tables.synthesis_formula.getDataById(batch_entry.Id) orelse continue;
        const unlocked = row.Unlock or ProgressInfo.containsId(progress_comp.info.craft.unlocked_synth, row.Id);
        if (!unlocked) continue;
        const prev_rec = ProgressInfo.findIdRoleCount(progress_comp.info.craft.synth_counts, row.Id);
        const prev_cnt: i32 = if (prev_rec) |r| r.count else 0;
        if (row.LimitCount > 0 and (std.math.add(i32, prev_cnt, batch_entry.Count) catch std.math.maxInt(i32)) > row.LimitCount) continue;
        const costs = CraftHelper.buildScaledCosts(alloc.arena, row.ConsumeItems, batch_entry.Count) catch continue;
        const outcome = try CraftHelper.consumeAndGrant(.{
            .conn = txn.conn,
            .alloc = alloc,
            .io = io,
            .fs = fs,
            .assets = assets,
            .basic_comp = basic_comp,
            .inventory_comp = inventory_comp,
            .weapon_comp = weapon_comp,
            .echo_comp = echo_comp,
        }, costs, row.ItemId, batch_entry.Count, 2);
        if (!outcome.ok) continue;

        any_changed = true;
        try ProgressInfo.addIdRoleCount(&progress_comp.info.craft.synth_counts, alloc.gpa, row.Id, batch_entry.Count, 0);
        const prof_left = if (row.MaxProficiencyCount > 0) @max(0, row.MaxProficiencyCount - prev_cnt) else batch_entry.Count;
        const prof_cnt = @min(batch_entry.Count, prof_left);
        const prof_gain = if (row.Proficiency > 0 and prof_cnt > 0) row.Proficiency * prof_cnt else 0;
        if (CraftHelper.addProficiencyAndUpdateLevel(
            &progress_comp.info.craft.synth_level,
            &progress_comp.info.craft.synth_proficiency,
            prof_gain,
            assets.tables.synthesis_level.items,
        )) leveled = true;
        try out_items.append(alloc.arena, .{ .key = row.ItemId, .value = batch_entry.Count });
    }

    if (any_changed) {
        try progress_comp.save(fs, alloc.arena);
        if (leveled) {
            try txn.conn.push(alloc.gpa, io, pb.SynthesisLevelUpdateNotify{
                .LevelInfo = .{
                    .Level = progress_comp.info.craft.synth_level,
                    .TotalProficiency = progress_comp.info.craft.synth_proficiency,
                },
            });
        }
    }

    txn.respond(.{
        .Items = out_items,
        .Code = .Success,
    });
}

pub fn onSynthesisLevelRewardRequest(
    txn: *Transaction(pb.SynthesisLevelRewardRequest),
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
    const cur_lvl = progress_comp.info.craft.synth_level;
    var reward_map: std.AutoHashMapUnmanaged(i32, i32) = .empty;
    for (assets.tables.synthesis_level.items) |row| {
        if (row.Id <= cur_lvl and row.Id > progress_comp.info.craft.synth_claimed_level and row.DropIds > 0) {
            try CraftHelper.grantDropAndCollectItems(.{
                .conn = txn.conn,
                .alloc = alloc,
                .io = io,
                .fs = fs,
                .assets = assets,
                .basic_comp = basic_comp,
                .inventory_comp = inventory_comp,
                .weapon_comp = weapon_comp,
                .echo_comp = echo_comp,
            }, row.DropIds, 0, 0, 2, &reward_map);
        }
    }
    if (cur_lvl > progress_comp.info.craft.synth_claimed_level) {
        progress_comp.info.craft.synth_claimed_level = cur_lvl;
        try progress_comp.save(fs, alloc.arena);
    }
    txn.respond(.{
        .Code = .Success,
        .LevelInfo = .{
            .Level = progress_comp.info.craft.synth_level,
            .TotalProficiency = progress_comp.info.craft.synth_proficiency,
        },
    });
}

pub fn onCookingDataRequest(
    txn: *Transaction(pb.CookingDataRequest),
    assets: *const Assets,
    progress_comp: *const PlayerProgressComponent,
    alloc: mem.Alloc,
) !void {
    var formulas: std.ArrayList(pb.SingleFoodFormulaInfo) = .empty;
    var configs: std.ArrayList(pb.OneFormulaConfig) = .empty;
    for (assets.tables.cook_formula.items) |row| {
        try configs.append(alloc.arena, .{ .Id = row.Id });
        const rec = ProgressInfo.findIdRoleCount(progress_comp.info.craft.cook_counts, row.Id);
        const unlocked = row.Unlock or ProgressInfo.containsId(progress_comp.info.craft.unlocked_cook, row.Id);
        if (unlocked or rec != null) {
            try formulas.append(alloc.arena, .{
                .Id = row.Id,
                .CookCount = if (rec) |r| r.count else 0,
                .LastRoleId = if (rec) |r| r.last_role_id else 0,
            });
        }
    }

    var processed: std.ArrayList(pb.SingleProcessedFoodFormulaInfo) = .empty;
    for (assets.tables.cook_processed.items) |row| {
        const unlocked = row.Unlock or ProgressInfo.containsId(progress_comp.info.craft.unlocked_processed, row.Id);
        var interations: std.ArrayList(i32) = .empty;
        try interations.appendSlice(alloc.arena, row.InterationId);
        try processed.append(alloc.arena, .{
            .Id = row.Id,
            .LockState = unlocked,
            .Interations = interations,
        });
    }

    txn.respond(.{
        .ProcessedFoodFormulaInfos = processed,
        .FormulaConfigs = configs,
        .FoodFormulaInfos = formulas,
        .Code = .Success,
        .CookingInfo = .{
            .CookingLevel = progress_comp.info.craft.cook_level,
            .TotalProficiencys = progress_comp.info.craft.cook_proficiency,
        },
    });
}

pub fn onCookFormulaRequest(
    txn: *Transaction(pb.CookFormulaRequest),
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    const id = txn.message.CookFormulaId;
    if (assets.tables.cook_formula.getDataById(id)) |cf| {
        if (cf.FormulaItemId > 0 and !cf.Unlock and !ProgressInfo.containsId(progress_comp.info.craft.unlocked_cook, id)) {
            const costs = [_]ConsumeHelper.CostItem{.{ .id = cf.FormulaItemId, .count = 1 }};
            if (!(try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, &costs, io))) {
                txn.respond(.{ .Code = .ErrItemNotEnough, .CookFormulaId = id });
                return;
            }
        }
        _ = try ProgressInfo.appendUniqueId(&progress_comp.info.craft.unlocked_cook, alloc.gpa, id);
    } else if (assets.tables.cook_processed.getDataById(id) != null) {
        _ = try ProgressInfo.appendUniqueId(&progress_comp.info.craft.unlocked_processed, alloc.gpa, id);
    } else {
        txn.respond(.{ .Code = .ErrCookingFormulaNotFound, .CookFormulaId = id });
        return;
    }
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .Code = .Success, .CookFormulaId = id });
}

pub fn onCookFoodRequest(
    txn: *Transaction(pb.CookFoodRequest),
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    qctx: QuestEngine.QuestContext,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    const req = txn.message;
    const count = req.CookCount;
    if (count <= 0) {
        txn.respond(.{ .Code = .ErrCookingCount, .Id = req.Id, .RoleId = req.RoleId });
        return;
    }

    var out_item_id: i32 = 0;
    var prof_per: i32 = 0;
    var max_prof_cnt: i32 = 0;

    var raw_costs: []const @import("../../data/tables/common_types.zig").CostEntry = &.{};
    if (assets.tables.cook_formula.getDataById(req.Id)) |cf| {
        if (!cf.Unlock and !ProgressInfo.containsId(progress_comp.info.craft.unlocked_cook, req.Id)) {
            txn.respond(.{ .Code = .ErrCookingFormulaNotFound, .Id = req.Id, .RoleId = req.RoleId });
            return;
        }
        out_item_id = cf.FoodItemId;
        prof_per = cf.Proficiency;
        max_prof_cnt = cf.MaxProficiencyCount;
        raw_costs = cf.ConsumeItems;
    } else if (assets.tables.cook_processed.getDataById(req.Id)) |cp| {
        if (!cp.Unlock and !ProgressInfo.containsId(progress_comp.info.craft.unlocked_processed, req.Id)) {
            txn.respond(.{ .Code = .ErrCookingProcessNotFound, .Id = req.Id, .RoleId = req.RoleId });
            return;
        }
        out_item_id = cp.FinalItemId;
        prof_per = 10;
        raw_costs = cp.ConsumeItemsId;
    } else {
        txn.respond(.{ .Code = .ErrCookingFormulaNotFound, .Id = req.Id, .RoleId = req.RoleId });
        return;
    }

    const costs = CraftHelper.buildScaledCosts(alloc.arena, raw_costs, count) catch {
        txn.respond(.{ .Code = .ErrCookingCount, .Id = req.Id, .RoleId = req.RoleId });
        return;
    };
    const outcome = try CraftHelper.consumeAndGrant(.{
        .conn = txn.conn,
        .alloc = alloc,
        .io = io,
        .fs = fs,
        .assets = assets,
        .basic_comp = basic_comp,
        .inventory_comp = inventory_comp,
        .weapon_comp = weapon_comp,
        .echo_comp = echo_comp,
    }, costs, out_item_id, count, 2);

    if (!outcome.ok) {
        txn.respond(.{ .Code = outcome.err, .Id = req.Id, .RoleId = req.RoleId });
        return;
    }

    const prev_rec = ProgressInfo.findIdRoleCount(progress_comp.info.craft.cook_counts, req.Id);
    const prev_cnt: i32 = if (prev_rec) |r| r.count else 0;
    const prof_left = if (max_prof_cnt > 0) @max(0, max_prof_cnt - prev_cnt) else count;
    const prof_cnt = @min(count, prof_left);

    try ProgressInfo.addIdRoleCount(&progress_comp.info.craft.cook_counts, alloc.gpa, req.Id, count, req.RoleId);
    const leveled = CraftHelper.addProficiencyAndUpdateLevel(
        &progress_comp.info.craft.cook_level,
        &progress_comp.info.craft.cook_proficiency,
        if (prof_per > 0 and prof_cnt > 0) prof_per * prof_cnt else 0,
        assets.tables.cook_level.items,
    );
    try progress_comp.save(fs, alloc.arena);
    QuestEngine.onUiAction(qctx, "Cook") catch |err| QuestEngine.logHookErr("onUiAction(Cook)", err);

    if (leveled) {
        try txn.conn.push(alloc.gpa, io, pb.CookingInfoUpdateNotify{
            .CookingInfo = .{
                .CookingLevel = progress_comp.info.craft.cook_level,
                .TotalProficiencys = progress_comp.info.craft.cook_proficiency,
            },
        });
    }

    txn.respond(.{
        .ItemInfos = outcome.items,
        .Code = .Success,
        .Id = req.Id,
        .RoleId = req.RoleId,
    });
}
