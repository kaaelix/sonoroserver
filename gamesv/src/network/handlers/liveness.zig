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
const RewardService = @import("../../logic/helpers/reward.zig");
const EnergyHelper = @import("../../logic/helpers/energy.zig");

const FileSystem = common.FileSystem;

pub fn buildLivenessInfoPb(
    arena: std.mem.Allocator,
    assets: *const Assets,
    basic_comp: *const PlayerBasicComponent,
    progress: ProgressInfo,
    now_sec: i64,
) !pb.LivenessInfo {
    var rewarded: std.ArrayList(i32) = .empty;
    try rewarded.appendSlice(arena, progress.liveness.claimed_boxes);

    var tasks: std.ArrayList(pb.LivenessTask) = .empty;
    for (assets.tables.liveness_task.items) |row| {
        const taken = ProgressInfo.containsId(progress.liveness.taken_tasks, row.TaskId);
        const cur = @max(1, ProgressInfo.getIdCount(progress.liveness.task_progress, row.TaskId));
        try tasks.append(arena, .{
            .Id = row.TaskId,
            .Current = cur,
            .Target = 1,
            .IsFinished = true,
            .IsTaken = taken,
            .ConditionFinishState = true,
        });
    }

    return .{
        .LivenessCount = progress.liveness.liveness_count,
        .RewardedLiveness = rewarded,
        .Tasks = tasks,
        .DayEnd = now_sec + 86_400,
        .AreaId = basic_comp.info.attributes.area_id,
    };
}

pub fn onLivenessRequest(
    txn: *Transaction(pb.LivenessRequest),
    assets: *const Assets,
    basic_comp: *const PlayerBasicComponent,
    progress_comp: *const PlayerProgressComponent,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    txn.respond(.{
        .LivenessInfo = try buildLivenessInfoPb(
            alloc.arena,
            assets,
            basic_comp,
            progress_comp.info,
            EnergyHelper.nowSeconds(io),
        ),
    });
}

pub fn onLivenessTakeRequest(
    txn: *Transaction(pb.LivenessTakeRequest),
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
    var taken_ids: std.ArrayList(i32) = .empty;
    var item_map: std.AutoHashMapUnmanaged(i32, i32) = .empty;
    var changed = false;

    for (txn.message.Ids.items) |box_id| {
        const box = assets.tables.liveness.getDataById(box_id) orelse continue;
        if (progress_comp.info.liveness.liveness_count < box.Goal) continue;
        if (ProgressInfo.containsId(progress_comp.info.liveness.claimed_boxes, box_id)) continue;

        _ = try ProgressInfo.appendUniqueId(&progress_comp.info.liveness.claimed_boxes, alloc.gpa, box_id);
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
        }, box.DropId, 3, 10, 19600, &item_map);
        try taken_ids.append(alloc.arena, box_id);
        changed = true;
    }

    if (changed) {
        try progress_comp.save(fs, alloc.arena);
    }

    txn.respond(.{
        .Ids = taken_ids,
        .ErrorCode = .Success,
    });
}

pub fn onLivenessTaskTakeRequest(
    txn: *Transaction(pb.LivenessTaskTakeRequest),
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
    var claimed: std.ArrayList(i32) = .empty;
    var extra_items: std.ArrayList(RewardService.RewardItem) = .empty;
    var changed = false;

    for (txn.message.TaskIds.items) |tid| {
        const task = assets.tables.liveness_task.getDataById(tid) orelse continue;
        if (ProgressInfo.containsId(progress_comp.info.liveness.taken_tasks, tid)) continue;
        _ = try ProgressInfo.appendUniqueId(&progress_comp.info.liveness.taken_tasks, alloc.gpa, tid);

        var it = task.TaskReward.map.iterator();
        while (it.next()) |kv| {
            const rid = kv.key_ptr.*;
            const cnt = kv.value_ptr.*;
            if (rid == 9) {
                progress_comp.info.liveness.liveness_count += cnt;
            } else if (rid > 0 and cnt > 0) {
                try extra_items.append(alloc.arena, .{ .item_id = rid, .count = cnt });
            }
        }
        try claimed.append(alloc.arena, tid);
        changed = true;
    }

    if (extra_items.items.len > 0) {
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
        }, 0, true, extra_items.items, 19600);
    }

    if (changed) {
        try progress_comp.save(fs, alloc.arena);
        try txn.conn.push(alloc.gpa, io, pb.LivenessCountUpdateNotify{
            .LivenessCount = progress_comp.info.liveness.liveness_count,
        });
    }

    txn.respond(.{
        .ErrorCode = .Success,
        .TaskIds = claimed,
    });
}
