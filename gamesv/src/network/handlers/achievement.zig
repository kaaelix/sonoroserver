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
const CraftHelper = @import("../../logic/helpers/craft.zig");
const EnergyHelper = @import("../../logic/helpers/energy.zig");

const FileSystem = common.FileSystem;

fn computeAchievementTotals(
    assets: *const Assets,
    progress: ProgressInfo,
) struct { finish_count: i32, star_count: i32 } {
    var finish_count: i32 = 0;
    var star_count: i32 = 0;
    for (progress.achievement.entries) |rec| {
        if (rec.finish_time == 0) continue;
        finish_count += 1;
        if (assets.tables.achievement.getDataById(rec.id)) |cfg| {
            star_count += @max(1, cfg.Level);
        }
    }
    return .{ .finish_count = finish_count, .star_count = star_count };
}

pub fn entryToPb(rec: ProgressInfo.AchievementRecord) pb.AchievementEntry {
    return .{
        .Id = rec.id,
        .FinishTime = rec.finish_time,
        .IsReceive = rec.received,
        .Progress = .{
            .CurProgress = rec.cur_progress,
            .TotalProgress = rec.total_progress,
        },
    };
}

pub fn onAchievementInfoRequest(
    txn: *Transaction(pb.AchievementInfoRequest),
    assets: *const Assets,
    progress_comp: *const PlayerProgressComponent,
    alloc: mem.Alloc,
) !void {
    var groups: std.ArrayList(pb.AchievementGroupInfo) = .empty;
    for (assets.tables.achievement_group.items) |grp| {
        if (!grp.Enable) continue;
        var entries: std.ArrayList(pb.AchievementEntry) = .empty;
        var all_finished = false;
        for (assets.tables.achievement.items) |ach| {
            if (ach.GroupId != grp.Id) continue;
            for (progress_comp.info.achievement.entries) |rec| {
                if (rec.id == ach.Id) {
                    try entries.append(alloc.arena, entryToPb(rec));
                    if (rec.finish_time > 0) all_finished = true;
                    break;
                }
            }
        }
        const group_received = ProgressInfo.containsId(progress_comp.info.achievement.claimed_groups, grp.Id);
        try groups.append(alloc.arena, .{
            .AchievementGroupEntry = .{
                .Id = grp.Id,
                .FinishTime = if (all_finished) 1 else 0,
                .IsReceive = group_received,
            },
            .AchievementEntryList = entries,
        });
    }

    const totals = computeAchievementTotals(assets, progress_comp.info);
    txn.respond(.{
        .AchievementGroupInfoList = groups,
        .AchievementFinishedStar = totals.star_count,
        .FinishedAchievementNum = totals.finish_count,
    });
}

pub fn onUpdateAchievementInfoRequest(
    txn: *Transaction(pb.UpdateAchievementInfoRequest),
    progress_comp: *const PlayerProgressComponent,
    alloc: mem.Alloc,
) !void {
    var list: std.ArrayList(pb.AchievementEntry) = .empty;
    for (progress_comp.info.achievement.entries) |rec| {
        try list.append(alloc.arena, entryToPb(rec));
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .AchievementEntryList = list,
    });
}

pub fn onAchievementFinishRequest(
    txn: *Transaction(pb.AchievementFinishRequest),
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    const id = txn.message.Id;
    if (assets.tables.achievement.getDataById(id) == null) {
        txn.respond(.{ .ErrorCode = .RequestParamError });
        return;
    }
    const now_u32: u32 = @intCast(std.math.clamp(EnergyHelper.nowSeconds(io), 1, std.math.maxInt(u32)));
    const rec = try progress_comp.info.finishAchievement(alloc.gpa, id, now_u32);
    try progress_comp.save(fs, alloc.arena);

    try txn.conn.push(alloc.gpa, io, pb.AchievementProgressNotify{
        .AchievementEntry = entryToPb(rec.*),
    });
    const totals = computeAchievementTotals(assets, progress_comp.info);
    try txn.conn.push(alloc.gpa, io, pb.AchievementCountChangeNotify{
        .StarCount = totals.star_count,
        .FinishCount = totals.finish_count,
    });

    txn.respond(.{ .ErrorCode = .Success });
}

fn claimSingleAchievement(
    ctx: CraftHelper.CraftContext,
    progress_comp: *PlayerProgressComponent,
    id: i32,
    is_group: bool,
    out_items: *std.AutoHashMapUnmanaged(i32, i32),
) !bool {
    if (is_group) {
        const grp = ctx.assets.tables.achievement_group.getDataById(id) orelse return false;
        if (ProgressInfo.containsId(progress_comp.info.achievement.claimed_groups, id)) return false;
        _ = try ProgressInfo.appendUniqueId(&progress_comp.info.achievement.claimed_groups, ctx.alloc.gpa, id);
        try CraftHelper.grantDropAndCollectItems(ctx, grp.DropId, 3, 20, 19101, out_items);
        const now_u32: u32 = @intCast(std.math.clamp(EnergyHelper.nowSeconds(ctx.io), 1, std.math.maxInt(u32)));
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.AchievementGroupProgressNotify{
            .AchievementGroupEntry = .{ .Id = id, .FinishTime = now_u32, .IsReceive = true },
        });
        return true;
    }

    const cfg = ctx.assets.tables.achievement.getDataById(id) orelse return false;
    const rec = progress_comp.info.findAchievementPtr(id) orelse return false;
    if (rec.finish_time == 0 or rec.received) return false;
    rec.received = true;
    const fallback_astrite: i32 = @max(1, cfg.Level) * 5;
    try CraftHelper.grantDropAndCollectItems(ctx, cfg.OverrideDropId, 3, fallback_astrite, 19100, out_items);
    try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.AchievementProgressNotify{
        .AchievementEntry = entryToPb(rec.*),
    });
    return true;
}

pub fn onAchievementReceiveRequest(
    txn: *Transaction(pb.AchievementReceiveRequest),
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
    var out_items: std.AutoHashMapUnmanaged(i32, i32) = .empty;
    const ctx: CraftHelper.CraftContext = .{
        .conn = txn.conn,
        .alloc = alloc,
        .io = io,
        .fs = fs,
        .assets = assets,
        .basic_comp = basic_comp,
        .inventory_comp = inventory_comp,
        .weapon_comp = weapon_comp,
        .echo_comp = echo_comp,
    };
    const ok = try claimSingleAchievement(ctx, progress_comp, txn.message.Id, txn.message.IsGroupId, &out_items);
    if (!ok) {
        txn.respond(.{ .ErrorCode = .RequestParamError });
        return;
    }
    try progress_comp.save(fs, alloc.arena);

    var item_map: std.ArrayList(pb.MapEntry(i32, i32)) = .empty;
    var it = out_items.iterator();
    while (it.next()) |kv| {
        try item_map.append(alloc.arena, .{ .key = kv.key_ptr.*, .value = kv.value_ptr.* });
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .ItemMap = item_map,
    });
}

pub fn onMulAchievementReceiveRequest(
    txn: *Transaction(pb.MulAchievementReceiveRequest),
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
    var out_items: std.AutoHashMapUnmanaged(i32, i32) = .empty;
    const ctx: CraftHelper.CraftContext = .{
        .conn = txn.conn,
        .alloc = alloc,
        .io = io,
        .fs = fs,
        .assets = assets,
        .basic_comp = basic_comp,
        .inventory_comp = inventory_comp,
        .weapon_comp = weapon_comp,
        .echo_comp = echo_comp,
    };
    var claimed_ids: std.ArrayList(i32) = .empty;
    var claimed_groups: std.ArrayList(i32) = .empty;
    var changed = false;

    for (txn.message.AchievementIds.items) |aid| {
        if (try claimSingleAchievement(ctx, progress_comp, aid, false, &out_items)) {
            try claimed_ids.append(alloc.arena, aid);
            changed = true;
        }
    }
    for (txn.message.AchievementGroupIds.items) |gid| {
        if (try claimSingleAchievement(ctx, progress_comp, gid, true, &out_items)) {
            try claimed_groups.append(alloc.arena, gid);
            changed = true;
        }
    }
    if (changed) {
        try progress_comp.save(fs, alloc.arena);
    }

    txn.respond(.{
        .AchievementIds = claimed_ids,
        .ErrorCode = .Success,
        .AchievementGroupIds = claimed_groups,
    });
}
