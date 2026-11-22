// Handlers for 3.8.11 client RPCs and pushes missing from the 3.8.0 generated proto.
const std = @import("std");
const pb = @import("proto").pb;
const FileSystem = @import("common").FileSystem;
const mem = @import("../../mem.zig");
const Transaction = @import("../handlers.zig").Transaction;
const PlayerQuestComponent = @import("../../logic/component/player/PlayerQuestComponent.zig");
const PlayerCosmeticComponent = @import("../../logic/component/player/PlayerCosmeticComponent.zig");
const PlayerProgressComponent = @import("../../logic/component/player/PlayerProgressComponent.zig");
const Assets = @import("../../data/Assets.zig");
const EnergyHelper = @import("../../logic/helpers/energy.zig");
const PlayerID = @import("../../logic/PlayerID.zig");
const QuestInfo = @import("../../fs/QuestInfo.zig");
const QuestEngine = @import("../../logic/helpers/quest_engine.zig");

const res_log = std.log.scoped(.quest_resource);

pub fn onFlowSeqSetPlayerPosRequest(
    txn: *Transaction(pb.FlowSeqSetPlayerPosRequest),
    qctx: QuestEngine.QuestContext,
) !void {
    try QuestEngine.updatePlayerSceneState(qctx, .{ txn.message.X, txn.message.Y, txn.message.Z }, .{ 0, 0, txn.message.A });
    txn.respond(.{ .Code = .Success });
}

pub fn onFinishDownloadQuestResourceRequest(
    txn: *Transaction(pb.FinishDownloadQuestResourceRequest),
    qctx: QuestEngine.QuestContext,
) !void {
    try QuestEngine.syncQuestResourceAvailability(qctx, txn.message.QuestIds.items, &.{}, true);
    txn.respond(.{ .ErrorId = .Success });
}

pub fn onSetQuestResourceStateRequest(
    txn: *Transaction(pb.SetQuestResourceStateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    sender: PlayerID,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "quest:res_state", txn.message.QuestResourceState);
    try progress_comp.save(fs, alloc.arena);
    res_log.info("player({d}) reported QuestResourceState={d}", .{ sender.id, txn.message.QuestResourceState });
    txn.respond(.{ .ErrorId = .Success });
}

pub fn onGetDetectionTargetRequest(
    txn: *Transaction(pb.GetDetectionTargetRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
) !void {
    var datas: std.ArrayList(pb.DetectionTargetData) = .empty;
    var tids: std.ArrayList(i32) = .empty;
    const req_type = txn.message.Type orelse .NormalMonster;
    const det_id = txn.message.DetectionId;

    switch (req_type) {
        .NormalMonster, .Material => {
            if (det_id > 0) {
                if (assets.tables.monster_detection.getDataById(det_id)) |md| {
                    if (md.EntityConfigId != 0) {
                        try tids.append(alloc.arena, @intCast(md.EntityConfigId));
                    } else if (md.BlueprintType.len > 0) {
                        if (assets.getDb()) |db| {
                            const found = db.queryEntityIdsByMapBlueprint(8, md.BlueprintType, alloc.arena) catch &.{};
                            for (found) |eid| {
                                try tids.append(alloc.arena, eid);
                                if (tids.items.len >= 32) break;
                            }
                        } else {
                            for (assets.tables.level_entity_config.items) |lcfg| {
                                if (lcfg.MapId != 8) continue;
                                if (std.mem.eql(u8, lcfg.BlueprintType, md.BlueprintType)) {
                                    try tids.append(alloc.arena, @intCast(lcfg.EntityId));
                                    if (tids.items.len >= 32) break;
                                }
                            }
                        }
                    }
                }
            } else {
                for (assets.tables.monster_detection.items) |md| {
                    if (md.EntityConfigId != 0) {
                        try tids.append(alloc.arena, @intCast(md.EntityConfigId));
                        if (tids.items.len >= 16) break;
                    }
                }
            }
        },
        .Dungeon => {
            if (det_id > 0) {
                if (assets.tables.dungeon_detection.getDataById(det_id)) |dd| {
                    const entrance_id = if (dd.InstanceDungeonEntranceId != 0) dd.InstanceDungeonEntranceId else dd.DungeonId;
                    if (entrance_id != 0) {
                        try tids.append(alloc.arena, entrance_id);
                    }
                }
            } else {
                for (assets.tables.dungeon_detection.items) |dd| {
                    const entrance_id = if (dd.InstanceDungeonEntranceId != 0) dd.InstanceDungeonEntranceId else dd.DungeonId;
                    if (entrance_id != 0) {
                        try tids.append(alloc.arena, entrance_id);
                        if (tids.items.len >= 16) break;
                    }
                }
            }
        },
        .SilentArea => {
            if (det_id > 0) {
                if (assets.tables.silent_area_detection.getDataById(det_id)) |sd| {
                    for (sd.LevelPlayList) |lp_id| {
                        try tids.append(alloc.arena, lp_id);
                    }
                    if (tids.items.len == 0 and sd.DungeonId != 0) {
                        try tids.append(alloc.arena, sd.DungeonId);
                    }
                }
            } else {
                for (assets.tables.silent_area_detection.items) |sd| {
                    for (sd.LevelPlayList) |lp_id| {
                        try tids.append(alloc.arena, lp_id);
                        break;
                    }
                    if (tids.items.len >= 16) break;
                }
            }
        },
    }

    if (tids.items.len == 0 and det_id > 0) {
        if (assets.tables.monster_detection.getDataById(det_id)) |md| {
            if (md.EntityConfigId != 0) {
                try tids.append(alloc.arena, @intCast(md.EntityConfigId));
            }
        } else if (assets.tables.silent_area_detection.getDataById(det_id)) |sd| {
            for (sd.LevelPlayList) |lp_id| {
                try tids.append(alloc.arena, lp_id);
            }
        } else if (assets.tables.dungeon_detection.getDataById(det_id)) |dd| {
            const entrance_id = if (dd.InstanceDungeonEntranceId != 0) dd.InstanceDungeonEntranceId else dd.DungeonId;
            if (entrance_id != 0) {
                try tids.append(alloc.arena, entrance_id);
            }
        }
    }

    try datas.append(alloc.arena, .{ .MapId = 8, .TargetIds = tids });
    txn.respond(.{
        .Type = txn.message.Type,
        .Code = .Success,
        .DetectionId = txn.message.DetectionId,
        .DetectionTargetDatas = datas,
    });
}

pub fn onOutputDebugInfoRequest(txn: *Transaction(pb.OutputDebugInfoRequest)) !void {
    txn.respond(.{});
}

pub fn onInvalidItemRequest(txn: *Transaction(pb.InvalidItemRequest)) !void {
    txn.respond(.{
        .HasInvalidItem = false,
        .HasExpireConvertItem = false,
    });
}

pub fn onShowInvalidItemRequest(txn: *Transaction(pb.ShowInvalidItemRequest)) !void {
    txn.respond(.{
        .InvalidItemList = .empty,
        .ExpireConvertItemInfoList = .empty,
    });
}

pub fn onGameStatsShowRequest(txn: *Transaction(pb.GameStatsShowRequest)) !void {
    txn.respond(.{ .GoogleGameStatsShow = .default });
}

pub fn onThirdPartyShareRequest(txn: *Transaction(pb.ThirdPartyShareRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success });
}

fn isLevelPlayHidden(info: QuestInfo, lpid: i32) bool {
    for (info.world.hidden_groups) |grp| {
        if (QuestInfo.containsInt(grp.level_plays, lpid)) return true;
    }
    return false;
}

pub fn onLevelPlayStateListRequest(
    txn: *Transaction(pb.LevelPlayStateListRequest),
    quest_comp: *const PlayerQuestComponent,
    alloc: mem.Alloc,
) !void {
    var rsps: std.ArrayList(pb.InstLevelPlayStateRsp) = .empty;
    for (txn.message.InstLevelPlayStateReqs.items) |req| {
        var rsp: pb.InstLevelPlayStateRsp = .{ .InstId = req.InstId };
        for (req.LevelPlayIds.items) |lpid| {
            const done = QuestInfo.containsInt(quest_comp.info.completed_level_plays, lpid);
            const hidden = isLevelPlayHidden(quest_comp.info, lpid);
            const state_val: i32 = if (done) 1 else 0;
            try rsp.LevelPlayStateDict.append(alloc.arena, .{ .key = lpid, .value = state_val });
            try rsp.CompleteNumberDict.append(alloc.arena, .{ .key = lpid, .value = state_val });
            try rsp.HideDict.append(alloc.arena, .{ .key = lpid, .value = hidden });
            try rsp.IsUnlockDict.append(alloc.arena, .{ .key = lpid, .value = !hidden });
        }
        try rsps.append(alloc.arena, rsp);
    }
    txn.respond(.{
        .InstLevelPlayStateRsps = rsps,
        .ErrorCode = .Success,
    });
}

pub fn onIllustratedUnlockRequest(
    txn: *Transaction(pb.IllustratedUnlockRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    cosmetic_comp: *PlayerCosmeticComponent,
    io: std.Io,
) !void {
    const entry_type = txn.message.Type orelse .Monster;
    const now_sec = EnergyHelper.nowSeconds(io);
    const create_time: u32 = @intCast(std.math.clamp(now_sec, 1, std.math.maxInt(u32)));
    const rec = try cosmetic_comp.info.unlockIllustrated(
        alloc.gpa,
        @intFromEnum(entry_type),
        txn.message.Id,
        create_time,
    );
    try PlayerCosmeticComponent.save(alloc.gpa, fs, cosmetic_comp.player_id, cosmetic_comp.info);
    txn.respond(.{
        .Entry = .{
            .Id = rec.id,
            .CreateTime = rec.create_time,
            .Num = rec.num,
            .IsRead = rec.is_read,
        },
        .ErrorCode = .Success,
    });
}

pub fn onEntityPatrolStartRequest(
    txn: *Transaction(pb.EntityPatrolStartRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "patrol:dir:{d}", .{txn.message.EntityId});
    try progress_comp.info.setKeyInt(alloc.gpa, key, if (txn.message.Dir) 1 else 0);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .Code = .Success });
}

pub fn onLandingDamageRequest(
    txn: *Transaction(pb.LandingDamageRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "combat:fall_dmg_cnt", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSceneBlockChangePush(_: *Transaction(pb.SceneBlockChangePush)) !void {}

pub fn onMapOpenPush(_: *Transaction(pb.MapOpenPush)) !void {}

pub fn onAceAntiDataPush(_: *Transaction(pb.AceAntiDataPush)) !void {}

pub fn onSceneBlockSplitClientLoginPush(_: *Transaction(pb.SceneBlockSplitClientLoginPush)) !void {}

pub fn onSceneBlockSplitSwitchStatePush(
    txn: *Transaction(pb.SceneBlockSplitSwitchStatePush),
    alloc: mem.Alloc,
    fs: *FileSystem,
    sender: PlayerID,
    progress_comp: *PlayerProgressComponent,
) !void {
    if (txn.message.SwitchState) |st| {
        const val: i64 = @intFromEnum(st);
        try progress_comp.info.setKeyInt(alloc.gpa, "quest:block_state", val);
        try progress_comp.save(fs, alloc.arena);
        res_log.info("player({d}) reported SceneBlockSwitchState={s}", .{ sender.id, @tagName(st) });
    }
}
