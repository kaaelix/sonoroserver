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
const Scene = @import("../../logic/Scene.zig");
const QuestInfo = @import("../../fs/QuestInfo.zig");
const QuestEngine = @import("../../logic/helpers/quest_engine.zig");
const attributes_helper = @import("../../logic/helpers/attributes.zig");

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
    scene_opt: *?Scene,
    progress_comp: *PlayerProgressComponent,
) !void {
    const dir_val: i64 = if (txn.message.Dir) 1 else 0;
    const key = try std.fmt.allocPrint(alloc.arena, "patrol:dir:{d}", .{txn.message.EntityId});
    try progress_comp.info.setKeyInt(alloc.gpa, key, dir_val);
    if (scene_opt.*) |*scene| {
        if (scene.findConfigIdByNetId(txn.message.EntityId)) |cfg_id| {
            const cfg_key = try std.fmt.allocPrint(alloc.arena, "patrol:dir:{d}", .{cfg_id});
            try progress_comp.info.setKeyInt(alloc.gpa, cfg_key, dir_val);
        }
        if (scene.net_id_map.get(txn.message.EntityId)) |idx| {
            scene.entities.items(.world_item)[idx].patrol_dir = txn.message.Dir;
        }
    }
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .Code = .Success });
}

pub fn onLandingDamageRequest(
    txn: *Transaction(pb.LandingDamageRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    scene_opt: *?Scene,
    progress_comp: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "combat:fall_dmg_cnt", 1);
    try progress_comp.save(fs, alloc.arena);

    const speed = @abs(txn.message.SpeedZ);
    if (speed > 1200.0 or txn.message.TimeExceeding > 0) {
        if (scene_opt.*) |*scene| {
            const target_nid = if (txn.message.EntityId > 0) txn.message.EntityId else scene.currentRoleEntityId();
            if (scene.net_id_map.get(target_nid)) |idx| {
                if (scene.entities.items(.attribute)[idx]) |*attr| {
                    const life_idx = @intFromEnum(pb.EAttributeType.Life);
                    const max_idx = @intFromEnum(pb.EAttributeType.LifeMax);
                    const cur_hp = attr.attributes[life_idx].current;
                    const max_hp = attr.attributes[max_idx].current;
                    if (cur_hp > 0 and max_hp > 0) {
                        const raw_ratio = if (speed > 1200.0)
                            (speed - 1200.0) / 2500.0
                        else
                            @as(f32, @floatFromInt(@min(txn.message.TimeExceeding, 10))) * 0.08;
                        const clamped_ratio = std.math.clamp(raw_ratio, 0.05, 0.90);
                        const dmg: i32 = @max(1, @as(i32, @intFromFloat(@as(f32, @floatFromInt(max_hp)) * clamped_ratio)));
                        const next_hp = @max(1, cur_hp - dmg);
                        _ = attributes_helper.set_life_unconditional(attr, next_hp);
                        var changed: std.ArrayList(pb.EAttributeType) = .empty;
                        try changed.append(alloc.arena, .Life);
                        var cpack: std.ArrayList(pb.CombatReceiveData) = .empty;
                        try attributes_helper.generate_attr_messages(&cpack, target_nid, attr, &changed, alloc, io);
                        if (cpack.items.len > 0) {
                            try txn.conn.push(alloc.gpa, io, pb.CombatReceivePackNotify{ .Data = cpack });
                        }
                        scene.markFsmDirty(alloc.gpa, target_nid, Scene.Entity.FsmComponent.WakeReason.attribute) catch {};
                    }
                }
            }
        }
    }

    txn.respond(.{ .ErrorCode = .Success });
    try QuestEngine.onConditionStateChanged(qctx);
}

pub fn onSceneBlockChangePush(
    txn: *Transaction(pb.SceneBlockChangePush),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var dirty = false;
    for (txn.message.DownloadBlockId.items) |bid| {
        const key = try std.fmt.allocPrint(alloc.arena, "scene:block:{d}", .{bid});
        try progress_comp.info.setKeyInt(alloc.gpa, key, 1);
        dirty = true;
    }
    for (txn.message.DeleteBlockId.items) |bid| {
        const key = try std.fmt.allocPrint(alloc.arena, "scene:block:{d}", .{bid});
        try progress_comp.info.setKeyInt(alloc.gpa, key, 0);
        dirty = true;
    }
    if (dirty) try progress_comp.save(fs, alloc.arena);
}

pub fn onMapOpenPush(
    txn: *Transaction(pb.MapOpenPush),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    _ = txn.message;
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "ui:map_open_cnt", 1);
    try progress_comp.save(fs, alloc.arena);
    QuestEngine.onConditionStateChanged(qctx) catch {};
}

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
