const std = @import("std");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const FileSystem = @import("common").FileSystem;
const Transaction = @import("../handlers.zig").Transaction;
const Connection = @import("../Connection.zig");
const MotorInfo = @import("../../fs/MotorInfo.zig");
const PlayerMotorComponent = @import("../../logic/component/player/PlayerMotorComponent.zig");
const PlayerInventoryComponent = @import("../../logic/component/player/PlayerInventoryComponent.zig");
const PlayerBasicComponent = @import("../../logic/component/player/PlayerBasicComponent.zig");
const PlayerRoleComponent = @import("../../logic/component/player/PlayerRoleComponent.zig");
const Scene = @import("../../logic/Scene.zig");
const Assets = @import("../../data/Assets.zig");
const EventQueue = @import("../../logic/EventQueue.zig");
const PlayerID = @import("../../logic/PlayerID.zig");
const PlayerEntityTemplates = @import("../../logic/templates/PlayerEntityTemplates.zig");
const BuffTimerScheduler = @import("../../logic/schedulers/BuffTimerScheduler.zig");
const entity_proto = @import("../../logic/helpers/entity_proto.zig");
const PlayerProgressComponent = @import("../../logic/component/player/PlayerProgressComponent.zig");
const ConsumeHelper = @import("../../logic/helpers/consume.zig");
const RewardService = @import("../../logic/helpers/reward.zig");

fn intList(arena: std.mem.Allocator, items: []const i32) !std.ArrayList(i32) {
    var list: std.ArrayList(i32) = .empty;
    try list.ensureTotalCapacity(arena, items.len);
    for (items) |id| {
        list.appendAssumeCapacity(id);
    }
    return list;
}

fn timedIdList(arena: std.mem.Allocator, items: []const i32) !std.ArrayList(pb.MotorOutlookIdTimePairPb) {
    var list: std.ArrayList(pb.MotorOutlookIdTimePairPb) = .empty;
    try list.ensureTotalCapacity(arena, items.len);
    for (items) |id| {
        list.appendAssumeCapacity(.{ .Id = id, .OpenTime = 0 });
    }
    return list;
}

fn motorTaskType(value: i32) pb.MotorTaskTypePb {
    return switch (value) {
        1 => .Single,
        2 => .Limited,
        3 => .Cycle,
        else => .Unknown,
    };
}

fn buildTechTrees(motor_comp: *PlayerMotorComponent, alloc: mem.Alloc) !std.ArrayList(pb.MotorTechOneTreePb) {
    var tree_ids: std.ArrayList(i32) = .empty;
    for (motor_comp.info.tech_nodes) |node| {
        var found = false;
        for (tree_ids.items) |tree_id| {
            if (tree_id == node.tree_type) {
                found = true;
                break;
            }
        }
        if (!found) try tree_ids.append(alloc.arena, node.tree_type);
    }

    var trees: std.ArrayList(pb.MotorTechOneTreePb) = .empty;
    try trees.ensureTotalCapacity(alloc.arena, tree_ids.items.len);
    for (tree_ids.items) |tree_id| {
        var tech_list: std.ArrayList(pb.MotorTechPb) = .empty;
        for (motor_comp.info.tech_nodes) |node| {
            if (node.tree_type != tree_id) continue;
            try tech_list.append(alloc.arena, .{
                .Id = node.id,
                .Level = node.level,
                .Unlock = node.unlocked,
                .Current = node.current,
                .Target = node.target,
            });
        }
        trees.appendAssumeCapacity(.{
            .TreeId = tree_id,
            .Tech = tech_list,
        });
    }
    return trees;
}

fn buildTaskTrees(motor_comp: *PlayerMotorComponent, alloc: mem.Alloc) !std.ArrayList(pb.MotorTaskTreePb) {
    var tree_ids: std.ArrayList(i32) = .empty;
    for (motor_comp.info.task_nodes) |task| {
        var found = false;
        for (tree_ids.items) |tree_id| {
            if (tree_id == task.tree_type) {
                found = true;
                break;
            }
        }
        if (!found) try tree_ids.append(alloc.arena, task.tree_type);
    }

    var trees: std.ArrayList(pb.MotorTaskTreePb) = .empty;
    try trees.ensureTotalCapacity(alloc.arena, tree_ids.items.len);
    for (tree_ids.items) |tree_id| {
        var task_list: std.ArrayList(pb.MotorTaskPb) = .empty;
        var tp_rewarded: i32 = 0;
        for (motor_comp.info.task_nodes) |task| {
            if (task.tree_type != tree_id) continue;
            tp_rewarded += task.rewarded;
            try task_list.append(alloc.arena, .{
                .Id = task.id,
                .Type = motorTaskType(task.type),
                .Process = .{
                    .Current = task.current,
                    .Target = task.target,
                },
                .Reward = .{
                    .Rewarded = task.rewarded,
                    .WaitReward = task.wait_reward,
                    .MaxReward = task.max_reward,
                },
            });
        }
        trees.appendAssumeCapacity(.{
            .TreeId = tree_id,
            .Tasks = task_list,
            .TpRewarded = @max(tp_rewarded, 100),
        });
    }
    return trees;
}

fn buildMotorTechTreeData(motor_comp: *PlayerMotorComponent, alloc: mem.Alloc, tree_id: i32) !pb.MotorTechOneTreePb {
    var tech_list: std.ArrayList(pb.MotorTechPb) = .empty;
    for (motor_comp.info.tech_nodes) |node| {
        if (node.tree_type != tree_id) continue;
        try tech_list.append(alloc.arena, .{
            .Id = node.id,
            .Level = node.level,
            .Unlock = node.unlocked,
            .Current = node.current,
            .Target = node.target,
        });
    }
    return .{
        .TreeId = tree_id,
        .Tech = tech_list,
    };
}

fn buildMotorTaskListData(motor_comp: *PlayerMotorComponent, alloc: mem.Alloc, tree_id: i32) !std.ArrayList(pb.MotorTaskPb) {
    var task_list: std.ArrayList(pb.MotorTaskPb) = .empty;
    for (motor_comp.info.task_nodes) |task| {
        if (task.tree_type != tree_id) continue;
        try task_list.append(alloc.arena, .{
            .Id = task.id,
            .Type = motorTaskType(task.type),
            .Process = .{
                .Current = task.current,
                .Target = task.target,
            },
            .Reward = .{
                .Rewarded = task.rewarded,
                .WaitReward = task.wait_reward,
                .MaxReward = task.max_reward,
            },
        });
    }
    return task_list;
}

fn replaceList(gpa: std.mem.Allocator, target: *[]i32, source: []const i32) !void {
    if (target.len != 0) gpa.free(target.*);
    target.* = try gpa.dupe(i32, source);
}

fn motorOutlookErrorCode(err: anyerror) ?pb.ErrorCode {
    return switch (err) {
        error.MotorOutlookNotOwned => .MotorOutlookNotOwned,
        error.MotorFrameCfgNotFound => .MotorFrameCfgNotFound,
        error.MotorStickerCfgNotFound => .MotorStickerCfgNotFound,
        error.MotorStickerPartIdError => .MotorStickerPartIdError,
        error.MotorDecorationsCfgNotFound => .MotorDecorationsCfgNotFound,
        error.MotorDecorationsPartIdError => .MotorDecorationsPartIdError,
        else => null,
    };
}

fn partIndex(part_id: i32, count: usize) ?usize {
    if (part_id <= 0) return null;
    const index: usize = @intCast(part_id - 1);
    return if (index < count) index else null;
}

fn initPartList(gpa: std.mem.Allocator, count: usize, previous: ?[]const i32) ![]i32 {
    const result = try gpa.alloc(i32, count);
    for (result, 0..) |*slot, i| {
        slot.* = if (previous) |items|
            if (i < items.len) items[i] else 0
        else
            0;
    }
    return result;
}

fn normalizeStickers(gpa: std.mem.Allocator, assets: *const Assets, source: []const i32, previous: ?[]const i32) ![]i32 {
    const result = try initPartList(gpa, assets.tables.motor_sticker_part.items.len, previous);
    errdefer gpa.free(result);

    for (source, 0..) |id, i| {
        if (id == 0) {
            if (i < result.len) result[i] = 0;
            continue;
        }

        const sticker = assets.tables.motor_sticker.getDataById(id) orelse return error.MotorStickerCfgNotFound;
        const index = partIndex(sticker.PartId, result.len) orelse return error.MotorStickerPartIdError;
        result[index] = id;
    }

    return result;
}

fn normalizeDecorations(gpa: std.mem.Allocator, assets: *const Assets, source: []const i32, previous: ?[]const i32) ![]i32 {
    const result = try initPartList(gpa, assets.tables.motor_decorations_part.items.len, previous);
    errdefer gpa.free(result);

    for (source, 0..) |id, i| {
        if (id == 0) {
            if (i < result.len) result[i] = 0;
            continue;
        }

        const decoration = assets.tables.motor_decorations.getDataById(id) orelse return error.MotorDecorationsCfgNotFound;
        const index = partIndex(decoration.PartId, result.len) orelse return error.MotorDecorationsPartIdError;
        result[index] = id;
    }

    return result;
}

fn resolveFrame(assets: *const Assets, requested: i32, previous: i32) !i32 {
    const frame = if (requested == 0) previous else requested;
    if (frame != 0 and assets.tables.motor_frame.getDataById(frame) == null) {
        return error.MotorFrameCfgNotFound;
    }
    return frame;
}

fn buildPresetData(alloc: mem.Alloc, preset: MotorInfo.Preset) !pb.MotorOutlookPresetPlanPb {
    return .{
        .Id = preset.id,
        .Mame = preset.name,
        .Preset = .{
            .SkinEquipped = preset.skin,
            .StickerEquipped = try intList(alloc.arena, preset.stickers),
            .DecorationsEquipped = try intList(alloc.arena, preset.decorations),
            .FrameEquipped = preset.frame,
        },
    };
}

fn buildPresetList(motor_comp: *PlayerMotorComponent, alloc: mem.Alloc) !pb.MotorOutlookPlayerPresetPb {
    var presets: std.ArrayList(pb.MotorOutlookPresetPlanPb) = .empty;
    try presets.ensureTotalCapacity(alloc.arena, motor_comp.info.presets.len);
    for (motor_comp.info.presets) |preset| {
        presets.appendAssumeCapacity(try buildPresetData(alloc, preset));
    }
    return .{ .Plan = presets };
}

fn buildCleanPresetList(motor_comp: *PlayerMotorComponent, alloc: mem.Alloc) !pb.MotorOutlookPlayerPresetPb {
    var presets: std.ArrayList(pb.MotorOutlookPresetPlanPb) = .empty;
    try presets.ensureTotalCapacity(alloc.arena, motor_comp.info.presets.len);
    for (motor_comp.info.presets) |preset| {
        presets.appendAssumeCapacity(.{
            .Id = preset.id,
            .Mame = preset.name,
            .Preset = .{
                .SkinEquipped = preset.skin,
                .StickerEquipped = try intList(alloc.arena, preset.stickers),
                .DecorationsEquipped = try intList(alloc.arena, preset.decorations),
                .FrameEquipped = preset.frame,
            },
        });
    }
    return .{ .Plan = presets };
}

fn buildEquippedOutlook(motor_comp: *PlayerMotorComponent, alloc: mem.Alloc) !pb.MotorDiyEquippedPb {
    return .{
        .SkinEquipped = motor_comp.info.equipped_skin,
        .StickerEquipped = try intList(alloc.arena, motor_comp.info.equipped_stickers),
        .DecorationsEquipped = try intList(alloc.arena, motor_comp.info.equipped_decorations),
        .FrameEquipped = motor_comp.info.equipped_frame,
    };
}

fn findMotorEntity(scene: *Scene, assets: *const Assets) ?Scene.Entity {
    const motorcycle_config_id = assets.getMotorcycleConfigId() orelse return null;
    const slice = scene.entities.slice();
    for (slice.items(.follower)) |maybe_follower| {
        const follower = maybe_follower orelse continue;
        for (follower.list) |entry| {
            if (entry.Type != @intFromEnum(pb.FollowerType.EPlayerFollowerMotor) or entry.EntityId == 0) continue;

            const index = scene.net_id_map.get(entry.EntityId) orelse continue;
            const config = slice.items(.config)[index];
            if (config.config_id == motorcycle_config_id) {
                return .{
                    .index = index,
                    .net_id = entry.EntityId,
                };
            }
        }
    }

    for (slice.items(.config), 0..) |config, i| {
        if (config.config_id == motorcycle_config_id) {
            return .{
                .index = i,
                .net_id = slice.items(.entity_id)[i].net_id,
            };
        }
    }

    return null;
}

fn isFormationEntity(scene: *Scene, entity_id: i64) bool {
    for (scene.formation_info.formations) |formation| {
        for (formation.roles) |maybe_role| {
            const role = maybe_role orelse continue;
            if (role.entity_id == entity_id) return true;
        }
    }
    return false;
}

fn pushEntityAddNotify(
    conn: *Connection,
    alloc: mem.Alloc,
    scene: *Scene,
    entity: Scene.Entity,
    assets: *const Assets,
    buff_timers: *BuffTimerScheduler,
    now_ms: i64,
    io: std.Io,
) !void {
    var role_entity_pbs: std.ArrayList(pb.EntityPb) = .empty;
    defer role_entity_pbs.deinit(alloc.gpa);
    var concom_entity_pbs: std.ArrayList(pb.EntityPb) = .empty;
    defer concom_entity_pbs.deinit(alloc.gpa);

    const storage = scene.entities.get(entity.index);
    try role_entity_pbs.append(
        alloc.gpa,
        try entity_proto.build(alloc, assets, scene, buff_timers, entity.net_id, now_ms),
    );

    if (storage.concomitant) |concomitant| {
        for (concomitant.custom_entity_ids) |concom_id| {
            if (!scene.net_id_map.contains(concom_id)) continue;
            const concom_pb = try entity_proto.build(
                alloc,
                assets,
                scene,
                buff_timers,
                concom_id,
                now_ms,
            );
            try concom_entity_pbs.append(alloc.gpa, concom_pb);
        }
    }

    var role_entity_add_notify: pb.EntityAddNotify = .{ .RemoveTagIds = false };
    role_entity_add_notify.EntityPbs = role_entity_pbs;
    try conn.push(alloc.gpa, io, role_entity_add_notify);

    if (concom_entity_pbs.items.len != 0) {
        var concom_entity_add_notify: pb.EntityAddNotify = .{ .RemoveTagIds = false };
        concom_entity_add_notify.EntityPbs = concom_entity_pbs;
        try conn.push(alloc.gpa, io, concom_entity_add_notify);
    }
}

fn pushEntityRefreshNotify(
    conn: *Connection,
    alloc: mem.Alloc,
    scene: *Scene,
    entity: Scene.Entity,
    assets: *const Assets,
    buff_timers: *BuffTimerScheduler,
    now_ms: i64,
    io: std.Io,
) !void {
    const storage = scene.entities.get(entity.index);
    var remove_infos: std.ArrayList(pb.EntityRemoveInfo) = .empty;

    if (storage.concomitant) |concomitant| {
        for (concomitant.custom_entity_ids) |concom_id| {
            try remove_infos.append(alloc.arena, .{ .EntityId = concom_id });
        }
    }

    try remove_infos.append(alloc.arena, .{ .EntityId = entity.net_id });
    try conn.push(alloc.gpa, io, pb.EntityRemoveNotify{
        .IsRemove = true,
        .RemoveInfos = remove_infos,
    });
    try pushEntityAddNotify(conn, alloc, scene, entity, assets, buff_timers, now_ms, io);
}

fn pushEntityRemoveNotify(
    conn: *Connection,
    alloc: mem.Alloc,
    fs: *FileSystem,
    scene: *Scene,
    entity: Scene.Entity,
    io: std.Io,
) !void {
    if (isFormationEntity(scene, entity.net_id)) return;

    const storage = scene.entities.get(entity.index);
    var remove_infos: std.ArrayList(pb.EntityRemoveInfo) = .empty;
    defer remove_infos.deinit(alloc.gpa);

    if (storage.concomitant) |concomitant| {
        for (concomitant.custom_entity_ids) |concom_id| {
            try scene.remove(alloc.gpa, fs, concom_id);
            try remove_infos.append(alloc.gpa, .{ .EntityId = concom_id });
        }
    }

    try scene.remove(alloc.gpa, fs, entity.net_id);
    try remove_infos.append(alloc.gpa, .{ .EntityId = entity.net_id });

    var remove_notify: pb.EntityRemoveNotify = .{ .IsRemove = true };
    remove_notify.RemoveInfos = remove_infos;
    try conn.push(alloc.gpa, io, remove_notify);
}

fn pushRideSharingSeatNotify(
    conn: *Connection,
    alloc: mem.Alloc,
    role_entity_id: i64,
    motor_entity_id: i64,
    seat: i32,
    occupied: bool,
    io: std.Io,
) !void {
    try conn.push(alloc.gpa, io, pb.VehicleUpdateEntityNotify{
        .EntityId = role_entity_id,
        .VehicleCreatureId = motor_entity_id,
        .Seat = seat,
        .IsEntering = occupied,
        .ExitType = .ExitVehicleTypeSeatStandUp,
    });
}

fn updateMotorOutlookEntity(
    alloc: mem.Alloc,
    fs: *FileSystem,
    scene: *Scene,
    assets: *const Assets,
    motor_comp: *PlayerMotorComponent,
) !?Scene.Entity {
    const slice = scene.entities.slice();
    const entity = findMotorEntity(scene, assets) orelse return null;
    const i = entity.index;

    if (slice.items(.motor_outlook)[i]) |*outlook| {
        outlook.skin = motor_comp.info.equipped_skin;
        try replaceList(alloc.gpa, &outlook.stickers, motor_comp.info.equipped_stickers);
        try replaceList(alloc.gpa, &outlook.decorations, motor_comp.info.equipped_decorations);
        outlook.frame = motor_comp.info.equipped_frame;
    } else {
        slice.items(.motor_outlook)[i] = .{
            .skin = motor_comp.info.equipped_skin,
            .stickers = try alloc.gpa.dupe(i32, motor_comp.info.equipped_stickers),
            .decorations = try alloc.gpa.dupe(i32, motor_comp.info.equipped_decorations),
            .frame = motor_comp.info.equipped_frame,
        };
    }

    try scene.saveComponents(fs, alloc.gpa, entity, &.{Scene.Entity.MotorOutlookComponent});
    return entity;
}

fn freePreset(gpa: std.mem.Allocator, preset: MotorInfo.Preset) void {
    if (preset.name.len != 0) gpa.free(preset.name);
    if (preset.stickers.len != 0) gpa.free(preset.stickers);
    if (preset.decorations.len != 0) gpa.free(preset.decorations);
}

fn makePreset(gpa: std.mem.Allocator, id: i32, name: []const u8, equipped: pb.MotorDiyEquippedPb, fallback_skin: i32) !MotorInfo.Preset {
    return .{
        .id = id,
        .name = try gpa.dupe(u8, name),
        .skin = if (equipped.SkinEquipped == 0) fallback_skin else equipped.SkinEquipped,
        .stickers = try gpa.dupe(i32, equipped.StickerEquipped.items),
        .decorations = try gpa.dupe(i32, equipped.DecorationsEquipped.items),
        .frame = equipped.FrameEquipped,
    };
}

pub fn buildMotorOutlookPb(motor_comp: *PlayerMotorComponent, alloc: mem.Alloc) !pb.MotorOutlookPb {
    return .{
        .MotorOutlookOwned = .{
            .SkinOwned = try intList(alloc.arena, motor_comp.info.owned_skins),
            .StickerOwned = try intList(alloc.arena, motor_comp.info.owned_stickers),
            .DecorationsOwned = try intList(alloc.arena, motor_comp.info.owned_decorations),
            .FrameOwned = try intList(alloc.arena, motor_comp.info.owned_frames),
        },
        .MotorOutlookEquipped = .{
            .SkinEquipped = motor_comp.info.equipped_skin,
            .StickerEquipped = try intList(alloc.arena, motor_comp.info.equipped_stickers),
            .DecorationsEquipped = try intList(alloc.arena, motor_comp.info.equipped_decorations),
            .FrameEquipped = motor_comp.info.equipped_frame,
        },
        .MotorOutlookPreset = try buildCleanPresetList(motor_comp, alloc),
        .SceneInUse = motor_comp.info.scene_in_use,
    };
}

fn buildMotorDiy(motor_comp: *PlayerMotorComponent, alloc: mem.Alloc) !pb.MotorDiyPb {
    return .{
        .MotorDiyOnwer = .{
            .SkinOwned = try intList(alloc.arena, motor_comp.info.owned_skins),
            .StickerOnwed = try intList(alloc.arena, motor_comp.info.owned_stickers),
            .DecorationsOwned = try intList(alloc.arena, motor_comp.info.owned_decorations),
            .FrameOwned = try intList(alloc.arena, motor_comp.info.owned_frames),
        },
        .MotorDiyEquipped = .{
            .SkinEquipped = motor_comp.info.equipped_skin,
            .StickerEquipped = try intList(alloc.arena, motor_comp.info.equipped_stickers),
            .DecorationsEquipped = try intList(alloc.arena, motor_comp.info.equipped_decorations),
            .FrameEquipped = motor_comp.info.equipped_frame,
        },
        .MotorOutlookPreset = try buildCleanPresetList(motor_comp, alloc),
        .SceneInUse = motor_comp.info.scene_in_use,
    };
}

pub fn onMotorInfoRequest(
    txn: *Transaction(pb.MotorInfoRequest),
    alloc: mem.Alloc,
    motor_comp: *PlayerMotorComponent,
) !void {
    txn.respond(.{
        .ErrorCode = .Success,
        .Motor = .{
            .MotorLevel = motor_comp.info.level,
            .MotorExp = motor_comp.info.exp,
            .MotorRewardedLvMax = motor_comp.info.rewarded_max_level,
            .UnlockedTree = try buildTechTrees(motor_comp, alloc),
            .TreeInUse = motor_comp.info.tree_in_use,
            .TaskTrees = try buildTaskTrees(motor_comp, alloc),
            .MotorExpLimitGainDaily = motor_comp.info.daily_exp_gain,
            .MotorExpMonsterDropDailyLimit = motor_comp.info.daily_exp_limit,
        },
    });
}

pub fn onMotorDiyInfoRequest(
    txn: *Transaction(pb.MotorDiyInfoRequest),
    alloc: mem.Alloc,
    motor_comp: *PlayerMotorComponent,
    io: std.Io,
) !void {
    try txn.conn.push(alloc.gpa, io, pb.MotorOutlookRegionInfoNotify{
        .MotorOutlookRegion = .{
            .MotorSticker = try timedIdList(alloc.arena, motor_comp.info.owned_stickers),
            .MotorDecoration = try timedIdList(alloc.arena, motor_comp.info.owned_decorations),
            .MotorFrame = try timedIdList(alloc.arena, motor_comp.info.owned_frames),
        },
    });

    txn.respond(.{
        .ErrorCode = .Success,
        .MotorDiy = try buildMotorDiy(motor_comp, alloc),
    });
}

pub fn onMotorOutlookInfoRequest(
    txn: *Transaction(pb.MotorOutlookInfoRequest),
    alloc: mem.Alloc,
    motor_comp: *PlayerMotorComponent,
) !void {
    txn.respond(.{
        .ErrorCode = .Success,
        .MotorOutlook = .{
            .MotorOutlookOwned = .{
                .SkinOwned = try intList(alloc.arena, motor_comp.info.owned_skins),
                .StickerOwned = try intList(alloc.arena, motor_comp.info.owned_stickers),
                .DecorationsOwned = try intList(alloc.arena, motor_comp.info.owned_decorations),
                .FrameOwned = try intList(alloc.arena, motor_comp.info.owned_frames),
            },
            .MotorOutlookEquipped = .{
                .SkinEquipped = motor_comp.info.equipped_skin,
                .StickerEquipped = try intList(alloc.arena, motor_comp.info.equipped_stickers),
                .DecorationsEquipped = try intList(alloc.arena, motor_comp.info.equipped_decorations),
                .FrameEquipped = motor_comp.info.equipped_frame,
            },
            .MotorOutlookPreset = try buildCleanPresetList(motor_comp, alloc),
            .SceneInUse = motor_comp.info.scene_in_use,
        },
    });
}

pub fn onMotorUseSceneRequest(
    txn: *Transaction(pb.MotorUseSceneRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    motor_comp: *PlayerMotorComponent,
    inventory_comp: *PlayerInventoryComponent,
    assets: *const Assets,
) !void {
    const scene = assets.tables.motor_scene.getDataById(txn.message.SceneId) orelse {
        txn.respond(.{ .ErrorCode = .MotorSceneCfgNotFound });
        return;
    };

    if (scene.UnlockItem != 0 and inventory_comp.info.normalItemCount(scene.UnlockItem) <= 0) {
        txn.respond(.{ .ErrorCode = .RequestParamError });
        return;
    }

    motor_comp.info.scene_in_use = scene.Id;
    try motor_comp.save(fs, alloc.arena);

    txn.respond(.{
        .ErrorCode = .Success,
        .SceneInUse = scene.Id,
    });
}

pub fn onMotorCreateRequest(
    txn: *Transaction(pb.MotorCreateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "motor:created", if (txn.message.IsCreate) 1 else 0);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSendMovieModeRideSharingRequest(
    txn: *Transaction(pb.SendMovieModeRideSharingRequest),
    alloc: mem.Alloc,
    player_id: PlayerID,
    io: std.Io,
) !void {
    try txn.conn.push(alloc.gpa, io, pb.VehicleShareNotify{
        .PlayerId = player_id.id,
        .ShareRideMode = txn.message.ShareRideMode,
        .IsInMovieRideSharingMode = txn.message.IsInMovieRideSharingMode,
        .Reason = .ClientRequest,
    });

    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onChangeVehicleRideSharingRequest(
    txn: *Transaction(pb.ChangeVehicleRideSharingRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    player_id: PlayerID,
    scene: *Scene,
    assets: *const Assets,
    role_comp: *PlayerRoleComponent,
    buff_timers: *BuffTimerScheduler,
    io: std.Io,
) !void {
    const passenger_seat = 1;
    const now_ms = (std.Io.Clock.awake).now(io).toMilliseconds();

    if (txn.message.RoleId <= 0) {
        txn.respond(.{ .ErrorCode = .RequestParamError });
        return;
    }
    if (!role_comp.role_map.contains(txn.message.RoleId)) {
        txn.respond(.{ .ErrorCode = .RequestParamError });
        return;
    }

    const motor_entity = findMotorEntity(scene, assets) orelse {
        txn.respond(.{ .ErrorCode = .InternalError });
        return;
    };
    const player_scene_entity = PlayerEntityTemplates.findPlayerSceneEntity(scene, player_id.id) orelse {
        txn.respond(.{ .ErrorCode = .InternalError });
        return;
    };
    const companion = try PlayerEntityTemplates.ensureMotorcycleCompanionEntity(
        fs,
        scene,
        alloc,
        player_id.id,
        assets,
        motor_entity,
        player_scene_entity,
    );
    if (companion.created) {
        try pushEntityAddNotify(
            txn.conn,
            alloc,
            scene,
            companion.entity,
            assets,
            buff_timers,
            now_ms,
            io,
        );
    }

    if (PlayerEntityTemplates.findMotorcyclePassengerEntity(scene, assets)) |old_passenger| {
        try pushRideSharingSeatNotify(txn.conn, alloc, old_passenger.net_id, motor_entity.net_id, passenger_seat, false, io);
        try pushEntityRemoveNotify(txn.conn, alloc, fs, scene, old_passenger, io);
    }

    const passenger_entity = PlayerEntityTemplates.createMotorcyclePassengerEntity(
        fs,
        scene,
        alloc,
        player_id.id,
        assets,
        txn.message.RoleId,
        motor_entity,
        passenger_seat,
    ) catch |err| switch (err) {
        error.MotorcyclePassengerTemplateNotFound => {
            txn.respond(.{ .ErrorCode = .RequestParamError });
            return;
        },
        else => return err,
    };
    try pushEntityAddNotify(
        txn.conn,
        alloc,
        scene,
        passenger_entity,
        assets,
        buff_timers,
        now_ms,
        io,
    );

    try txn.conn.push(alloc.gpa, io, pb.UpdateVehicleRideSharingNotify{
        .PlayerId = player_id.id,
        .RoleId = txn.message.RoleId,
        .Seat = passenger_seat,
        .EntityId = passenger_entity.net_id,
    });
    try pushRideSharingSeatNotify(txn.conn, alloc, passenger_entity.net_id, motor_entity.net_id, passenger_seat, true, io);

    txn.respond(.{ .ErrorCode = .Success });
}

pub fn RemoveRideSharingPassengerRequest(
    txn: *Transaction(pb.RemoveRideSharingPassengerRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    player_id: PlayerID,
    scene: *Scene,
    assets: *const Assets,
    io: std.Io,
) !void {
    const passenger_seat = 1;

    if (PlayerEntityTemplates.findMotorcyclePassengerEntity(scene, assets)) |entity| {
        if (findMotorEntity(scene, assets)) |motor_entity| {
            try pushRideSharingSeatNotify(txn.conn, alloc, entity.net_id, motor_entity.net_id, passenger_seat, false, io);
        }

        try txn.conn.push(alloc.gpa, io, pb.UpdateVehicleRideSharingNotify{
            .PlayerId = player_id.id,
            .RoleId = 0,
            .Seat = passenger_seat,
            .EntityId = entity.net_id,
        });

        try pushEntityRemoveNotify(txn.conn, alloc, fs, scene, entity, io);
    }

    txn.respond(.{ .ErrorCode = .Success });
}

fn loadMotorLevelRewardId(arena: std.mem.Allocator, io: std.Io, target_level: i32) i32 {
    const raw = std.Io.Dir.readFileAlloc(std.Io.Dir.cwd(), io, "assets/BinData/MotorLvl.json", arena, std.Io.Limit.unlimited) catch return 0;
    const parsed = std.json.parseFromSlice(std.json.Value, arena, raw, .{}) catch return 0;
    const arr = switch (parsed.value) {
        .array => |a| a.items,
        else => return 0,
    };
    for (arr) |entry| {
        const obj = switch (entry) {
            .object => |o| o,
            else => continue,
        };
        const lvl: i64 = switch (obj.get("Level") orelse continue) {
            .integer => |v| v,
            else => continue,
        };
        if (lvl != target_level) continue;
        const rid: i64 = switch (obj.get("RewardId") orelse return 0) {
            .integer => |v| v,
            else => return 0,
        };
        return @intCast(std.math.clamp(rid, 0, std.math.maxInt(i32)));
    }
    return 0;
}

fn checkMotorOutlookOwnership(motor_comp: *const PlayerMotorComponent, assets: *const Assets, frame: i32, stickers: []const i32, decorations: []const i32) bool {
    if (frame != 0) {
        const fcfg = assets.tables.motor_frame.getDataById(frame) orelse return false;
        if (!fcfg.DefaultFlag and !MotorInfo.hasOwnedId(motor_comp.info.owned_frames, frame)) return false;
    }
    for (stickers) |sid| {
        if (sid == 0) continue;
        const scfg = assets.tables.motor_sticker.getDataById(sid) orelse return false;
        if (!scfg.FreeFlag and !MotorInfo.hasOwnedId(motor_comp.info.owned_stickers, sid)) return false;
    }
    for (decorations) |did| {
        if (did == 0) continue;
        const dcfg = assets.tables.motor_decorations.getDataById(did) orelse return false;
        if (!dcfg.FreeFlag and !MotorInfo.hasOwnedId(motor_comp.info.owned_decorations, did)) return false;
    }
    return true;
}

pub fn onMotorTechLevelUpRequest(
    txn: *Transaction(pb.MotorTechLevelUpRequest),
    events: *EventQueue,
    alloc: mem.Alloc,
    fs: *FileSystem,
    motor_comp: *PlayerMotorComponent,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    scene: *Scene,
    assets: *const Assets,
    io: std.Io,
) !void {
    const tech_cfg = assets.tables.motor_tech.getDataById(txn.message.TechId) orelse {
        txn.respond(.{ .ErrorCode = .MotorTechCfgNotFound });
        return;
    };

    for (tech_cfg.PreNode) |pre_id| {
        const pre_unlocked = for (motor_comp.info.tech_nodes) |n| {
            if (n.id == pre_id and n.level > 0) break true;
        } else false;
        if (!pre_unlocked) {
            txn.respond(.{ .ErrorCode = .MotorTechPreNodeLock });
            return;
        }
    }

    for (motor_comp.info.tech_nodes) |*node| {
        if (node.id != txn.message.TechId) continue;
        if (node.level < 0 or node.level >= node.target or @as(usize, @intCast(node.level)) >= tech_cfg.TechLv.len) {
            txn.respond(.{ .ErrorCode = .MotorTechLevelMax });
            return;
        }
        const tech_lv_id = tech_cfg.TechLv[@intCast(node.level)];
        if (assets.tables.motor_tech_lv.getDataById(tech_lv_id)) |lv_cfg| {
            if (lv_cfg.Consume > 0) {
                const tp_item_id = if (assets.tables.motor_tech_tree.getDataById(node.tree_type)) |tree_cfg|
                    tree_cfg.TpItemId
                else
                    0;
                if (tp_item_id > 0) {
                    const costs = [_]ConsumeHelper.CostItem{.{ .id = tp_item_id, .count = lv_cfg.Consume }};
                    if (!(try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, &costs, io))) {
                        txn.respond(.{ .ErrorCode = .ErrItemNotEnough });
                        return;
                    }
                }
            }
        }
        node.level += 1;
        node.current = node.level;
        node.unlocked = true;
        try motor_comp.save(fs, alloc.arena);
        if (findMotorEntity(scene, assets)) |entity| {
            try PlayerEntityTemplates.refreshMotorcycleBornBuffs(events, scene, alloc, assets, motor_comp, entity);
        }
        const tree = try buildMotorTechTreeData(motor_comp, alloc, node.tree_type);
        try txn.conn.push(alloc.gpa, io, pb.MotorLockedTechUpdateNotify{
            .TreeId = node.tree_type,
            .Tech = tree.Tech,
        });
        txn.respond(.{
            .ErrorCode = .Success,
            .Tree = tree,
        });
        return;
    }

    txn.respond(.{
        .ErrorCode = .MotorTechCfgNotFound,
    });
}

pub fn onMotorTechTreeSwitchRequest(
    txn: *Transaction(pb.MotorTechTreeSwitchRequest),
    events: *EventQueue,
    alloc: mem.Alloc,
    fs: *FileSystem,
    motor_comp: *PlayerMotorComponent,
    scene: *Scene,
    assets: *const Assets,
) !void {
    motor_comp.info.tree_in_use = txn.message.TreeId;
    try motor_comp.save(fs, alloc.arena);
    if (findMotorEntity(scene, assets)) |entity| {
        try PlayerEntityTemplates.refreshMotorcycleBornBuffs(events, scene, alloc, assets, motor_comp, entity);
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .TreeInUse = motor_comp.info.tree_in_use,
    });
}

pub fn onMotorLevelOneKeyRewardRequest(
    txn: *Transaction(pb.MotorLevelOneKeyRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    io: std.Io,
    motor_comp: *PlayerMotorComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    const start_lvl = motor_comp.info.rewarded_max_level + 1;
    if (start_lvl <= motor_comp.info.level) {
        var lvl = start_lvl;
        while (lvl <= motor_comp.info.level) : (lvl += 1) {
            const reward_id = loadMotorLevelRewardId(alloc.arena, io, lvl);
            if (reward_id > 0) {
                _ = try RewardService.grantDropPackagePopup(reward_ctx, reward_id, 8001);
            }
        }
        motor_comp.info.rewarded_max_level = motor_comp.info.level;
        try motor_comp.save(fs, alloc.arena);
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .MotorRewardedLvMax = motor_comp.info.rewarded_max_level,
    });
}

pub fn onMotorTaskOneKeyRewardRequest(
    txn: *Transaction(pb.MotorTaskOneKeyRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    motor_comp: *PlayerMotorComponent,
    reward_ctx: RewardService.RewardContext,
    io: std.Io,
) !void {
    var updated_tree: ?i32 = null;
    for (txn.message.TaskIds.items) |task_id| {
        for (motor_comp.info.task_nodes) |*task| {
            if (task.id != task_id) continue;
            if (task.wait_reward > 0) {
                const gain = task.wait_reward;
                task.rewarded += gain;
                task.wait_reward = 0;
                if (assets.tables.motor_tech_tree.getDataById(task.tree_type)) |tree_cfg| {
                    if (tree_cfg.TpItemId > 0) {
                        _ = try RewardService.grantItems(reward_ctx, 0, true, &.{.{ .item_id = tree_cfg.TpItemId, .count = gain }}, 8001);
                    }
                }
            }
            updated_tree = task.tree_type;
            break;
        }
    }

    if (updated_tree) |tree_id| {
        try motor_comp.save(fs, alloc.arena);
        try txn.conn.push(alloc.gpa, io, pb.MotorTaskUpdateNotify{
            .Task = try buildMotorTaskListData(motor_comp, alloc, tree_id),
        });
    }

    txn.respond(.{
        .ErrorCode = .Success,
    });
}

pub fn onMotorUseSkinRequest(
    txn: *Transaction(pb.MotorUseSkinRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    motor_comp: *PlayerMotorComponent,
    scene: *Scene,
    assets: *const Assets,
    buff_timers: *BuffTimerScheduler,
    io: std.Io,
) !void {
    const previous_frame = motor_comp.info.equipped_frame;
    const now_ms = (std.Io.Clock.awake).now(io).toMilliseconds();
    const skin = assets.tables.motor_skin.getDataById(txn.message.SkinId) orelse {
        txn.respond(.{
            .ErrorCode = .MotorOutlookNotOwned,
        });
        return;
    };
    if (!skin.DefaultFlag and !MotorInfo.hasOwnedId(motor_comp.info.owned_skins, txn.message.SkinId)) {
        txn.respond(.{
            .ErrorCode = .MotorOutlookNotOwned,
        });
        return;
    }
    const frame = resolveFrame(assets, skin.BindFrame, motor_comp.info.equipped_frame) catch |err| {
        if (motorOutlookErrorCode(err)) |code| {
            txn.respond(.{ .ErrorCode = code });
            return;
        }
        return err;
    };
    const stickers = normalizeStickers(alloc.gpa, assets, skin.BindSticker, null) catch |err| {
        if (motorOutlookErrorCode(err)) |code| {
            txn.respond(.{ .ErrorCode = code });
            return;
        }
        return err;
    };
    defer alloc.gpa.free(stickers);
    const decorations = normalizeDecorations(alloc.gpa, assets, skin.BindDecorations, null) catch |err| {
        if (motorOutlookErrorCode(err)) |code| {
            txn.respond(.{ .ErrorCode = code });
            return;
        }
        return err;
    };
    defer alloc.gpa.free(decorations);

    motor_comp.info.equipped_skin = txn.message.SkinId;
    motor_comp.info.equipped_frame = frame;
    try replaceList(alloc.gpa, &motor_comp.info.equipped_stickers, stickers);
    try replaceList(alloc.gpa, &motor_comp.info.equipped_decorations, decorations);
    try motor_comp.save(fs, alloc.arena);
    try txn.conn.push(alloc.gpa, io, pb.MotorOutlookEquippedChangeNotify{
        .MotorDiyEquipped = try buildEquippedOutlook(motor_comp, alloc),
    });
    if (try updateMotorOutlookEntity(alloc, fs, scene, assets, motor_comp)) |entity| {
        if (previous_frame != frame) {
            try pushEntityRefreshNotify(
                txn.conn,
                alloc,
                scene,
                entity,
                assets,
                buff_timers,
                now_ms,
                io,
            );
        } else {
            try txn.conn.push(alloc.gpa, io, pb.EntityMotorOutlookChangeNotify{
                .EntityId = entity.net_id,
                .MotorDiyEquipped = try buildEquippedOutlook(motor_comp, alloc),
            });
        }
    }
    txn.respond(.{
        .ErrorCode = .Success,
    });
}

pub fn onMotorChangeOutlookRequest(
    txn: *Transaction(pb.MotorChangeOutlookRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    motor_comp: *PlayerMotorComponent,
    scene: *Scene,
    assets: *const Assets,
    buff_timers: *BuffTimerScheduler,
    io: std.Io,
) !void {
    const previous_frame = motor_comp.info.equipped_frame;
    const now_ms = (std.Io.Clock.awake).now(io).toMilliseconds();
    const frame = resolveFrame(assets, txn.message.FrameEquipped, motor_comp.info.equipped_frame) catch |err| {
        if (motorOutlookErrorCode(err)) |code| {
            txn.respond(.{ .ErrorCode = code });
            return;
        }
        return err;
    };
    const stickers = normalizeStickers(alloc.gpa, assets, txn.message.StickerEquipped.items, motor_comp.info.equipped_stickers) catch |err| {
        if (motorOutlookErrorCode(err)) |code| {
            txn.respond(.{ .ErrorCode = code });
            return;
        }
        return err;
    };
    defer alloc.gpa.free(stickers);
    const decorations = normalizeDecorations(alloc.gpa, assets, txn.message.DecorationsEquipped.items, motor_comp.info.equipped_decorations) catch |err| {
        if (motorOutlookErrorCode(err)) |code| {
            txn.respond(.{ .ErrorCode = code });
            return;
        }
        return err;
    };
    defer alloc.gpa.free(decorations);

    if (!checkMotorOutlookOwnership(motor_comp, assets, frame, stickers, decorations)) {
        txn.respond(.{ .ErrorCode = .MotorOutlookNotOwned });
        return;
    }

    try replaceList(alloc.gpa, &motor_comp.info.equipped_stickers, stickers);
    try replaceList(alloc.gpa, &motor_comp.info.equipped_decorations, decorations);
    motor_comp.info.equipped_frame = frame;
    try motor_comp.save(fs, alloc.arena);
    try txn.conn.push(alloc.gpa, io, pb.MotorOutlookEquippedChangeNotify{
        .MotorDiyEquipped = try buildEquippedOutlook(motor_comp, alloc),
    });
    if (try updateMotorOutlookEntity(alloc, fs, scene, assets, motor_comp)) |entity| {
        if (previous_frame != frame) {
            try pushEntityRefreshNotify(
                txn.conn,
                alloc,
                scene,
                entity,
                assets,
                buff_timers,
                now_ms,
                io,
            );
        } else {
            try txn.conn.push(alloc.gpa, io, pb.EntityMotorOutlookChangeNotify{
                .EntityId = entity.net_id,
                .MotorDiyEquipped = try buildEquippedOutlook(motor_comp, alloc),
            });
        }
    }
    txn.respond(.{
        .ErrorCode = .Success,
    });
}

pub fn MotorOutlookCreatePresetRequest(
    txn: *Transaction(pb.MotorOutlookCreatePresetRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    motor_comp: *PlayerMotorComponent,
) !void {
    const equipped = txn.message.Preset orelse {
        txn.respond(.{ .ErrorCode = pb.ErrorCode.MotorOutlookPresetInputInvalid });
        return;
    };

    const preset = try makePreset(
        alloc.gpa,
        motor_comp.info.next_preset_id,
        txn.message.name,
        equipped,
        motor_comp.info.equipped_skin,
    );
    motor_comp.info.next_preset_id += 1;

    const old_presets = motor_comp.info.presets;
    const new_presets = try alloc.gpa.alloc(MotorInfo.Preset, old_presets.len + 1);
    @memcpy(new_presets[0..old_presets.len], old_presets);
    new_presets[old_presets.len] = preset;
    if (old_presets.len != 0) alloc.gpa.free(old_presets);
    motor_comp.info.presets = new_presets;

    try motor_comp.save(fs, alloc.arena);
    txn.respond(.{
        .ErrorCode = pb.ErrorCode.Success,
        .MotorOutlookPreset = try buildPresetList(motor_comp, alloc),
    });
}

pub fn onMotorOutlookDeletePresetRequest(
    txn: *Transaction(pb.MotorOutlookDeletePresetRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    motor_comp: *PlayerMotorComponent,
) !void {
    var index: ?usize = null;
    for (motor_comp.info.presets, 0..) |preset, i| {
        if (preset.id == txn.message.id) {
            index = i;
            break;
        }
    }

    const remove_index = index orelse {
        txn.respond(.{ .ErrorCode = pb.ErrorCode.MotorOutlookPresetNotExist });
        return;
    };

    const old_presets = motor_comp.info.presets;
    const new_presets = try alloc.gpa.alloc(MotorInfo.Preset, old_presets.len - 1);
    if (remove_index > 0) @memcpy(new_presets[0..remove_index], old_presets[0..remove_index]);
    if (remove_index + 1 < old_presets.len) {
        @memcpy(new_presets[remove_index..], old_presets[remove_index + 1 ..]);
    }
    freePreset(alloc.gpa, old_presets[remove_index]);
    alloc.gpa.free(old_presets);
    motor_comp.info.presets = new_presets;

    try motor_comp.save(fs, alloc.arena);
    txn.respond(.{
        .ErrorCode = pb.ErrorCode.Success,
        .MotorOutlookPreset = try buildPresetList(motor_comp, alloc),
    });
}

pub fn onMotorOutlookEditPresetRequest(
    txn: *Transaction(pb.MotorOutlookEditPresetRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    motor_comp: *PlayerMotorComponent,
) !void {
    const request = txn.message.PresetPlan orelse {
        txn.respond(.{ .ErrorCode = pb.ErrorCode.MotorOutlookPresetInputInvalid });
        return;
    };
    const equipped = request.Preset orelse {
        txn.respond(.{ .ErrorCode = pb.ErrorCode.MotorOutlookPresetInputInvalid });
        return;
    };

    for (motor_comp.info.presets) |*preset| {
        if (preset.id != request.Id) continue;
        const next_preset = try makePreset(alloc.gpa, request.Id, request.Mame, equipped, motor_comp.info.equipped_skin);
        freePreset(alloc.gpa, preset.*);
        preset.* = next_preset;
        try motor_comp.save(fs, alloc.arena);
        txn.respond(.{
            .ErrorCode = pb.ErrorCode.Success,
            .MotorOutlookPreset = try buildPresetList(motor_comp, alloc),
        });
        return;
    }

    txn.respond(.{
        .ErrorCode = pb.ErrorCode.MotorOutlookPresetNotExist,
    });
}

pub fn onMotorAnimReplaySampleUdpPush(_: *Transaction(pb.MotorAnimReplaySampleUdpPush)) !void {}
pub fn onMotorCycleCruiseStatePush(_: *Transaction(pb.MotorCycleCruiseStatePush)) !void {}
pub fn onMotorIsEnablePush(_: *Transaction(pb.MotorIsEnablePush)) !void {}
pub fn onMotorOnlineProgressReportPush(_: *Transaction(pb.MotorOnlineProgressReportPush)) !void {}
pub fn onMotorPositionStatePush(_: *Transaction(pb.MotorPositionStatePush)) !void {}
pub fn onMotorSummonAndRidePush(_: *Transaction(pb.MotorSummonAndRidePush)) !void {}
pub fn onMotorFightRefreshBossPush(_: *Transaction(pb.MotorFightRefreshBossPush)) !void {}

pub fn onMotorDecalMotorIpRequest(
    txn: *Transaction(pb.MotorDecalMotorIpRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "motor:decal_ip", txn.message.ActivityId);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success, .MotorIp = txn.message.ActivityId });
}

pub fn onMotorDecalTaskRequest(
    txn: *Transaction(pb.MotorDecalTaskRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var kbuf: [48]u8 = undefined;
    var ids: std.ArrayList(i32) = .empty;
    var dirty = false;
    for (assets.tables.motor_decal_quest.items) |q| {
        if (q.Ip != txn.message.MotorIp) continue;
        try ids.append(alloc.arena, q.TaskId);
        const tk = try std.fmt.bufPrint(&kbuf, "motor:decal_task:{d}", .{q.TaskId});
        if (try progress_comp.info.markClaim(alloc.gpa, tk)) {
            dirty = true;
            _ = try RewardService.grantTableReward(reward_ctx, "MotorDecalQuest", q.TaskId, false, true, 1, &.{.{ .item_id = 3, .count = 40 }}, null);
        }
    }
    const k = try std.fmt.bufPrint(&kbuf, "motor:decal:{d}", .{txn.message.MotorIp});
    if (try progress_comp.info.markClaim(alloc.gpa, k)) {
        dirty = true;
        _ = try RewardService.grantTableReward(reward_ctx, "MotorDecalIp", txn.message.MotorIp, false, true, 1, &.{.{ .item_id = 3, .count = 40 }}, null);
    }
    if (dirty) try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success, .Ids = ids });
}

pub fn onMotorDevelopTaskRewardRequest(
    txn: *Transaction(pb.MotorDevelopTaskRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var kbuf: [48]u8 = undefined;
    var dirty = false;
    for (txn.message.Ids.items) |tid| {
        const k = try std.fmt.bufPrint(&kbuf, "motor:dev:{d}", .{tid});
        if (try progress_comp.info.markClaim(alloc.gpa, k)) {
            dirty = true;
            _ = try RewardService.grantTableReward(reward_ctx, "MotorDevelopTask", tid, false, true, 8001, &.{.{ .item_id = 3, .count = 40 }}, null);
        }
    }
    if (dirty) try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onMotorLinkTaskRequest(
    txn: *Transaction(pb.MotorLinkTaskRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var kbuf: [48]u8 = undefined;
    var ids: std.ArrayList(i32) = .empty;
    var dirty = false;
    for (assets.tables.motor_linkage_quest.items) |q| {
        if (q.Ip != txn.message.MotorIp) continue;
        try ids.append(alloc.arena, q.TaskId);
        const tk = try std.fmt.bufPrint(&kbuf, "motor:link_task:{d}", .{q.TaskId});
        if (try progress_comp.info.markClaim(alloc.gpa, tk)) {
            dirty = true;
            _ = try RewardService.grantTableReward(reward_ctx, "MotorLinkageQuest", q.TaskId, false, true, 8001, &.{.{ .item_id = 3, .count = 40 }}, null);
        }
    }
    const k = try std.fmt.bufPrint(&kbuf, "motor:link:{d}", .{txn.message.MotorIp});
    if (try progress_comp.info.markClaim(alloc.gpa, k)) {
        dirty = true;
        _ = try RewardService.grantItems(reward_ctx, 0, true, &.{.{ .item_id = 3, .count = 40 }}, 8001);
    }
    if (dirty) try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success, .Ids = ids });
}

pub fn onMotorParkourLevelRewardRequest(
    txn: *Transaction(pb.MotorParkourLevelRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var kbuf: [48]u8 = undefined;
    var dirty = false;
    for (txn.message.RewardIdxs.items) |ridx| {
        const k = try std.fmt.bufPrint(&kbuf, "motor:pk:{d}:{d}", .{ txn.message.MotorParkourId, ridx });
        if (try progress_comp.info.markClaim(alloc.gpa, k)) {
            dirty = true;
            _ = try RewardService.grantItems(reward_ctx, 0, true, &.{.{ .item_id = 3, .count = 30 }}, 8001);
        }
    }
    if (dirty) try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .Error = .Success });
}

pub fn onMotorSliderRequest(
    txn: *Transaction(pb.MotorSliderRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const itype: i64 = if (txn.message.InteractType) |t| @intFromEnum(t) else 0;
    const key = try std.fmt.allocPrint(alloc.arena, "motor:slider:{d}", .{txn.message.EntityId});
    try progress_comp.info.setKeyInt(alloc.gpa, key, itype);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onMotorFightEnterSubLevelRequest(
    txn: *Transaction(pb.MotorFightEnterSubLevelRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "motor:fight_sub_lv", txn.message.SubLevelIndex);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onMotorFightKillRequest(
    txn: *Transaction(pb.MotorFightKillRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    if (txn.message.IsKilled) {
        _ = try progress_comp.info.addKeyInt(alloc.gpa, "motor_fight:kills", 1);
        _ = try progress_comp.info.addKeyInt(alloc.gpa, "motor_fight:best_score", if (txn.message.IsBoss) 500 else 100);
        try progress_comp.save(fs, alloc.arena);
    }
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onMotorFightLastSaveDataRequest(
    txn: *Transaction(pb.MotorFightLastSaveDataRequest),
    progress_comp: *const PlayerProgressComponent,
) !void {
    _ = txn.message;
    const sub_lv = progress_comp.info.getKeyInt("motor:fight_sub_lv");
    const score = progress_comp.info.getKeyInt("motor_fight:best_score");
    txn.respond(.{
        .ErrCode = .Success,
        .Exists = sub_lv > 0 or score > 0,
        .LastSaveData = .{
            .LevelId = @intCast(@max(0, sub_lv)),
            .Score = @intCast(@max(0, score)),
        },
    });
}
pub fn onMotorFightOneKeyRewardRequest(
    txn: *Transaction(pb.MotorFightOneKeyRewardRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    var kbuf: [48]u8 = undefined;
    var dirty = false;
    for (txn.message.TaskIds.items) |tid| {
        const k = try std.fmt.bufPrint(&kbuf, "motor:fight_task:{d}", .{tid});
        if (try progress_comp.info.markClaim(alloc.gpa, k)) {
            dirty = true;
            _ = try RewardService.grantTableReward(reward_ctx, "MotorFightTask", tid, false, true, 8001, &.{.{ .item_id = 3, .count = 40 }}, null);
        }
    }
    if (dirty) try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}
pub fn onMotorFightPlayerDeadRequest(
    txn: *Transaction(pb.MotorFightPlayerDeadRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "motor:fight_deaths", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrCode = .Success });
}

fn buildMotorFightSelfRank(
    player_id: PlayerID,
    basic_comp: *const PlayerBasicComponent,
    progress_comp: *const PlayerProgressComponent,
) pb.MotorFightPlayerRankingInfo {
    const kills: i32 = @intCast(@max(0, progress_comp.info.getKeyInt("motor_fight:kills")));
    const score: i32 = @intCast(@max(0, progress_comp.info.getKeyInt("motor_fight:best_score")));
    const sub_lv: i32 = @intCast(@max(1, progress_comp.info.getKeyInt("motor:fight_sub_lv")));
    return .{
        .PlayerId = player_id.id,
        .Name = basic_comp.info.attributes.name,
        .ResultCommon = .{
            .LevelId = sub_lv,
            .TotalScore = score,
            .KillMonsterCount = kills,
        },
    };
}

pub fn onMotorFightRankListRequest(
    txn: *Transaction(pb.MotorFightRankListRequest),
    player_id: PlayerID,
    basic_comp: *const PlayerBasicComponent,
    progress_comp: *const PlayerProgressComponent,
) !void {
    _ = txn.message;
    txn.respond(.{
        .ErrorCode = .Success,
        .SelfData = buildMotorFightSelfRank(player_id, basic_comp, progress_comp),
    });
}

pub fn onMotorFightRankListSelfDataRequest(
    txn: *Transaction(pb.MotorFightRankListSelfDataRequest),
    player_id: PlayerID,
    basic_comp: *const PlayerBasicComponent,
    progress_comp: *const PlayerProgressComponent,
) !void {
    _ = txn.message;
    txn.respond(.{
        .ErrorCode = .Success,
        .SelfData = buildMotorFightSelfRank(player_id, basic_comp, progress_comp),
    });
}

pub fn onMotorFightRefreshCollectionRequest(
    txn: *Transaction(pb.MotorFightRefreshCollectionRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    io: std.Io,
) !void {
    _ = txn.message;
    const cnt = try progress_comp.info.addKeyInt(alloc.gpa, "motor:fight_col_refresh", 1);
    try progress_comp.save(fs, alloc.arena);
    var drops: std.ArrayList(pb.MotorFightCollectionDrop) = .empty;
    for (assets.reward_index.getRows(io, "MotorFightCollection")) |row| {
        try drops.append(alloc.arena, .{ .CollectionId = @intCast(row.key) });
        if (drops.items.len >= 3) break;
    }
    txn.respond(.{
        .ErrCode = .Success,
        .RefreshCollectionTimes = @intCast(cnt),
        .Drops = drops,
    });
}

pub fn onMotorFightRefreshNextWaveGroupRequest(
    txn: *Transaction(pb.MotorFightRefreshNextWaveGroupRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = txn.message;
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "motor:fight_wave", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onMotorFightSelectBuffGateRequest(
    txn: *Transaction(pb.MotorFightSelectBuffGateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "motor:fight_buff_gate", txn.message.BuffGateId);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onMotorFightSelectCollectionRequest(
    txn: *Transaction(pb.MotorFightSelectCollectionRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "motor:fight_col_cnt", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onMotorFightSettleRequest(
    txn: *Transaction(pb.MotorFightSettleRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
    reward_ctx: RewardService.RewardContext,
) !void {
    _ = txn.message;
    const sub_lv: i32 = @intCast(@max(1, progress_comp.info.getKeyInt("motor:fight_sub_lv")));
    const kills: i32 = @intCast(@max(0, progress_comp.info.getKeyInt("motor_fight:kills")));
    const score = progress_comp.info.getKeyInt("motor_fight:best_score");
    var clear_kbuf: [48]u8 = undefined;
    const clear_key = try std.fmt.bufPrint(&clear_kbuf, "motor_fight:clear:{d}", .{sub_lv});
    const is_first_clear = try progress_comp.info.markClaim(alloc.gpa, clear_key);
    const best_key = try std.fmt.bufPrint(&clear_kbuf, "motor_fight:best:{d}", .{sub_lv});
    const prev_best = progress_comp.info.getKeyInt(best_key);
    const is_best = score > prev_best;
    if (is_best) {
        try progress_comp.info.setKeyInt(alloc.gpa, best_key, score);
    }
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "motor_fight:pass", 1);
    try progress_comp.save(fs, alloc.arena);

    if (is_first_clear) {
        _ = try RewardService.grantTableReward(reward_ctx, "MotorFightMainLevel", sub_lv, false, true, 1, &.{
            .{ .item_id = 3, .count = 40 },
            .{ .item_id = 2, .count = 10000 },
        }, null);
    }

    txn.respond(.{
        .ErrCode = .Success,
        .Win = true,
        .Result = .{
            .IsFirstClear = is_first_clear,
            .IsBestScore = is_best,
            .ResultCommon = .{
                .LevelId = sub_lv,
                .TotalScore = @intCast(@max(0, score)),
                .KillMonsterCount = kills,
            },
        },
    });
}
pub fn onMotorFightTalentUpRequest(
    txn: *Transaction(pb.MotorFightTalentUpRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const k = try std.fmt.bufPrint(&kbuf, "motor:fight_tal:{d}", .{txn.message.TalentId});
    _ = try progress_comp.info.addKeyInt(alloc.gpa, k, 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}
