const std = @import("std");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const PlayerSceneComponent = @import("../../logic/component/player/PlayerSceneComponent.zig");
const PlayerMapComponent = @import("../../logic/component/player/PlayerMapComponent.zig");
const PlayerRoleComponent = @import("../../logic/component/player/PlayerRoleComponent.zig");
const RoleHelper = @import("../../logic/helpers/role.zig");
const LevelEntityConfig = @import("../../data/tables/LevelEntityConfig.zig");
const EventQueue = @import("../../logic/EventQueue.zig");
const Scene = @import("../../logic/Scene.zig");
const FileSystem = @import("common").FileSystem;
const Transaction = @import("../handlers.zig").Transaction;
const Assets = @import("../../data/Assets.zig");
const QuestEngine = @import("../../logic/helpers/quest_engine.zig");
const GravityHelper = @import("../../logic/helpers/gravity.zig");

const log = std.log.scoped(.teleport);

pub fn onTeleportDataRequest(
    txn: *Transaction(pb.TeleportDataRequest),
    map_comp: *const PlayerMapComponent,
    alloc: mem.Alloc,
) !void {
    var ids: std.ArrayList(i32) = .empty;
    try ids.appendSlice(alloc.arena, map_comp.info.unlocked_teleporters);
    txn.respond(.{ .ErrorCode = .Success, .Ids = ids });
}

pub fn onTeleportFinishRequest(txn: *Transaction(pb.TeleportFinishRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onTeleportTransferRequest(
    txn: *Transaction(pb.TeleportTransferRequest),
    fs: *FileSystem,
    scene: *Scene,
    assets: *const Assets,
    events: *EventQueue,
    alloc: mem.Alloc,
    scene_comp: *PlayerSceneComponent,
    role_comp: *PlayerRoleComponent,
    io: std.Io,
    qctx: QuestEngine.QuestContext,
) !void {
    const teleporter = assets.tables.teleporter.getDataById(txn.message.Id) orelse {
        txn.respond(.{ .ErrorCode = .ErrTeleportIdNotExist });
        return;
    };

    const entity_cfg = assets.findLevelEntity(teleporter.MapId, teleporter.TeleportEntityConfigId) orelse {
        txn.respond(.{ .ErrorCode = .ErrTeleportCreatureIdNotExist });
        return;
    };

    const is_nexus = RoleHelper.isResonanceNexusConfig(assets, teleporter.TeleportEntityConfigId);
    const spawn = resolveTeleporterSpawn(is_nexus, entity_cfg) orelse {
        txn.respond(.{ .ErrorCode = .ErrTeleportCreatureIdNotExist });
        return;
    };
    const x = spawn.x;
    const y = spawn.y;
    const z = spawn.z;

    const is_wiped = if (scene.formation_info.cur_formation >= 0 and
        @as(usize, @intCast(scene.formation_info.cur_formation)) < scene.formation_info.formations.len)
        RoleHelper.formationLivingStatus(scene, scene.formation_info.formations[@intCast(scene.formation_info.cur_formation)]) == .Dead
    else
        false;
    if (is_nexus or is_wiped) {
        try RoleHelper.reviveAndHealFormation(txn.conn, scene, role_comp, events, fs, alloc, io);
    }

    const cur_inst_id = scene_comp.last_scene_info.instance_id;
    const cur_map_id = assets.resolveMapConfigId(cur_inst_id);
    const target_inst_id = assets.resolveInstanceIdForMap(teleporter.MapId);
    QuestEngine.onPlayerTransferOrLeaveInst(qctx, cur_map_id != 8 and teleporter.MapId == 8) catch |err| QuestEngine.logHookErr("onTeleportTransfer", err);

    if (cur_inst_id == target_inst_id) {
        const target_pos = [3]f32{ x, y, z };
        if (spawn.has_gravity_config) {
            scene.setCurrentRoleGravity(spawn.gravity);
        }
        try QuestEngine.updatePlayerSceneState(qctx, target_pos, .{ 0, 0, spawn.yaw });
        try txn.conn.push(alloc.gpa, io, pb.TeleportNotify{
            .MapId = teleporter.MapId,
            .Pos = .{ .X = x, .Y = y, .Z = z },
            .Rot = .{ .X = 0, .Y = 0, .Z = spawn.yaw },
            .Gravity = if (spawn.has_gravity_config) .{ .X = spawn.gravity[0], .Y = spawn.gravity[1], .Z = spawn.gravity[2] } else null,
            .Reason = .Transfer,
            .GameCtx = .{ .CtxInfo = .{ .Transfer = .{ .TeleportId = teleporter.Id } } },
        });
    } else {
        try txn.conn.push(alloc.gpa, io, pb.LeaveSceneNotify{
            .PlayerId = scene_comp.player_id,
            .SceneId = "",
            .TransitionOption = .{},
        });
        scene_comp.last_scene_info.instance_id = target_inst_id;
        try scene.saveLastScene(scene_comp, fs, alloc.gpa);
        try events.enqueue(.initial_scene_join, .{
            .target_instance_id = target_inst_id,
            .location = .{ x, y, z },
            .rotation = .{ 0, 0, spawn.yaw },
        });
    }

    txn.respond(.{
        .ErrorCode = .Success,
        .MapId = teleporter.MapId,
        .PosX = x,
        .PosY = y,
        .PosZ = z,
    });
}

pub fn onAddTemporaryTeleportRequest(
    txn: *Transaction(pb.AddTemporaryTeleportRequest),
    fs: *FileSystem,
    scene: *Scene,
    assets: *const Assets,
    map_comp: *PlayerMapComponent,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    const pos = txn.message.Pos orelse pb.Vector{};
    const rot = txn.message.Rot orelse pb.Rotator{};
    const map_id = assets.resolveMapConfigId(scene.instance_id);
    var tp = try map_comp.info.addTempTeleport(alloc.gpa, map_id, txn.message.Area, pos, rot);
    if (map_comp.info.temp_teleports.len > 0) {
        map_comp.info.temp_teleports[map_comp.info.temp_teleports.len - 1].inst_id = scene.instance_id;
        tp.inst_id = scene.instance_id;
    }
    try map_comp.save(fs, alloc.arena);

    try txn.conn.push(alloc.gpa, io, pb.AddTemporaryTeleportInfoNotify{
        .TemporaryTeleportInfo = tp.toPb(),
    });
    txn.respond(.{
        .TemporaryTeleportId = tp.id,
        .Code = .Success,
    });
}

pub fn onRemoveTemporaryTeleportRequest(
    txn: *Transaction(pb.RemoveTemporaryTeleportRequest),
    fs: *FileSystem,
    map_comp: *PlayerMapComponent,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    const removed = try map_comp.info.removeTempTeleport(alloc.gpa, txn.message.TemporaryTeleportId);
    if (removed) {
        try map_comp.save(fs, alloc.arena);
        try txn.conn.push(alloc.gpa, io, pb.RemoveTemporaryTeleportNotify{
            .TemporaryTeleportId = txn.message.TemporaryTeleportId,
        });
    }
    txn.respond(.{
        .ErrCode = if (removed) .Success else .ErrTeleportIdNotExist,
    });
}

pub fn onTeleportToTargetRequest(
    txn: *Transaction(pb.TeleportToTargetRequest),
    fs: *FileSystem,
    scene: *Scene,
    assets: *const Assets,
    events: *EventQueue,
    scene_comp: *PlayerSceneComponent,
    map_comp: *const PlayerMapComponent,
    alloc: mem.Alloc,
    io: std.Io,
    qctx: QuestEngine.QuestContext,
) !void {
    const tp = map_comp.info.findTempTeleport(txn.message.TemporaryTeleportId) orelse {
        txn.respond(.{ .ErrCode = .ErrTeleportIdNotExist });
        return;
    };
    const yaw = if (txn.message.Rot) |r| r.Yaw else tp.rot_yaw;
    const cur_map_id = assets.resolveMapConfigId(scene_comp.last_scene_info.instance_id);
    const tp_map_id = assets.resolveMapConfigId(tp.map_id);
    QuestEngine.onPlayerTransferOrLeaveInst(qctx, cur_map_id != 8 and tp_map_id == 8) catch |err| QuestEngine.logHookErr("onTeleportToTarget", err);
    if (cur_map_id == tp_map_id) {
        const target_pos = [3]f32{ tp.pos_x, tp.pos_y, tp.pos_z };
        try QuestEngine.updatePlayerSceneState(qctx, target_pos, .{ 0, 0, yaw });
        try txn.conn.push(alloc.gpa, io, pb.TeleportNotify{
            .MapId = tp_map_id,
            .Pos = .{ .X = target_pos[0], .Y = target_pos[1], .Z = target_pos[2] },
            .Rot = .{ .X = 0, .Y = 0, .Z = yaw },
            .Reason = .TemporaryTransfer,
        });
    } else {
        try txn.conn.push(alloc.gpa, io, pb.LeaveSceneNotify{
            .PlayerId = scene_comp.player_id,
            .SceneId = "",
            .TransitionOption = .{},
        });
        const target_inst_id = if (tp.inst_id > 0 and assets.tables.instance_dungeon.getDataById(tp.inst_id) != null)
            tp.inst_id
        else
            assets.resolveInstanceIdForMap(tp_map_id);
        scene_comp.last_scene_info.instance_id = target_inst_id;
        try scene.saveLastScene(scene_comp, fs, alloc.gpa);
        try events.enqueue(.initial_scene_join, .{
            .target_instance_id = target_inst_id,
            .location = .{ tp.pos_x, tp.pos_y, tp.pos_z },
            .rotation = .{ 0, 0, yaw },
        });
    }
    txn.respond(.{ .ErrCode = .Success });
}

const SpawnTransform = struct {
    x: f32,
    y: f32,
    z: f32,
    yaw: f32,
    gravity: [3]f32 = .{ 0, 0, -1 },
    has_gravity_config: bool = false,
};

fn resolveTeleporterSpawn(
    is_nexus: bool,
    entity_cfg: *const LevelEntityConfig,
) ?SpawnTransform {
    const base_pos = Assets.AssetIndexes.entityWorldPos(entity_cfg) orelse return null;
    const base_rot = Assets.AssetIndexes.entityWorldRot(entity_cfg);
    const grav = GravityHelper.resolveEntityGravity(entity_cfg, base_rot);

    const is_pillar_tp = is_nexus or
        (std.mem.indexOf(u8, entity_cfg.BlueprintType, "Teleport") != null and
            std.mem.indexOf(u8, entity_cfg.BlueprintType, "TeleportPoint") == null);

    var ox: f32 = 0.0;
    var oy: f32 = if (is_pillar_tp) 300.0 else 0.0;
    var oz: f32 = 0.0;
    var yaw: f32 = base_rot[2];

    const tc_opt = entity_cfg.ComponentsData.TeleportComponent orelse entity_cfg.ComponentsData.DynamicTeleportComponent;
    if (tc_opt) |tc| {
        if (tc.TeleportPos) |p| {
            const px = p.X orelse 0.0;
            const py = p.Y orelse 0.0;
            // Some pillar teleporters (e.g. 3520003) carry null/near-zero X/Y; keep them beside the pillar.
            if (!is_pillar_tp or (px * px + py * py >= 1.0)) {
                ox = px;
                oy = py;
            }
            // TeleportPos.Z is ground surface height relative to entity origin.
            oz = p.Z orelse 0.0;
            yaw = base_rot[2] + (p.A orelse 0.0);
        }
    }

    const rotated = GravityHelper.rotateLocalOffsetUe(.{ ox, oy, oz }, base_rot);
    // Capsule half-height (85cm) must lift away from the gravity down vector.
    const cap_up = if (grav.configured)
        [3]f32{ -grav.dir[0] * 85.0, -grav.dir[1] * 85.0, -grav.dir[2] * 85.0 }
    else
        GravityHelper.rotateLocalOffsetUe(.{ 0.0, 0.0, 85.0 }, base_rot);
    return .{
        .x = base_pos[0] + rotated[0] + cap_up[0],
        .y = base_pos[1] + rotated[1] + cap_up[1],
        .z = base_pos[2] + rotated[2] + cap_up[2],
        .yaw = yaw,
        .gravity = grav.dir,
        .has_gravity_config = grav.configured,
    };
}

test "resolveTeleporterSpawn applies TeleportComponent offset, capsule Z lift, 3D gravity rotation, and pillar fallback" {
    var transform_90 = [_]LevelEntityConfig.Vector{
        .{ .X = 100000, .Y = 200000, .Z = 30000 },
        .{ .X = 0, .Y = 0, .Z = 9000 }, // 90 deg yaw
        .{ .X = 100, .Y = 100, .Z = 100 },
    };

    // 1. With TeleportPos rotated by 90 deg: (ox=100, oy=200, oz=117.25+85) -> (-200, 100, +202.25)
    const cfg_with_pos: LevelEntityConfig = .{
        .Id = 1,
        .EntityId = 113003404,
        .BlueprintType = "Teleport001",
        .InSleep = false,
        .IsHidden = false,
        .MapId = 8,
        .AreaId = 1101,
        .Transform = &transform_90,
        .ComponentsData = .{
            .TeleportComponent = .{
                .TeleporterId = 300011,
                .TeleportPos = .{ .X = 100.0, .Y = 200.0, .Z = 117.25, .A = -45.0 },
            },
        },
    };
    const s1 = resolveTeleporterSpawn(false, &cfg_with_pos).?;
    try std.testing.expectApproxEqAbs(@as(f32, 800.0), s1.x, 0.01);
    try std.testing.expectApproxEqAbs(@as(f32, 2100.0), s1.y, 0.01);
    try std.testing.expectApproxEqAbs(@as(f32, 502.25), s1.z, 0.01);
    try std.testing.expectApproxEqAbs(@as(f32, 45.0), s1.yaw, 0.01);
    try std.testing.expect(!s1.has_gravity_config);

    // 2. With TeleportPos where Z is null -> lifted by +85cm
    const cfg_null_z: LevelEntityConfig = .{
        .Id = 2,
        .EntityId = 113003404,
        .BlueprintType = "branch2.0_TeleportPoint",
        .InSleep = false,
        .IsHidden = false,
        .MapId = 8,
        .AreaId = 1101,
        .Transform = &transform_90,
        .ComponentsData = .{
            .TeleportComponent = .{
                .TeleporterId = 300011,
                .TeleportPos = .{ .X = null, .Y = null, .Z = null, .A = -10.0 },
            },
        },
    };
    const s2 = resolveTeleporterSpawn(false, &cfg_null_z).?;
    try std.testing.expectApproxEqAbs(@as(f32, 1000.0), s2.x, 0.01);
    try std.testing.expectApproxEqAbs(@as(f32, 2000.0), s2.y, 0.01);
    try std.testing.expectApproxEqAbs(@as(f32, 385.0), s2.z, 0.01);
    try std.testing.expectApproxEqAbs(@as(f32, 80.0), s2.yaw, 0.01);

    // 3. Missing TeleportComponent on Teleport3 pillar -> offset (0, 300, 85) rotated by 90 deg -> (-300, 0, +85)
    const cfg_pillar_fallback: LevelEntityConfig = .{
        .Id = 3,
        .EntityId = 113003404,
        .BlueprintType = "Teleport3",
        .InSleep = false,
        .IsHidden = false,
        .MapId = 8,
        .AreaId = 1101,
        .Transform = &transform_90,
        .ComponentsData = .{},
    };
    const s3 = resolveTeleporterSpawn(false, &cfg_pillar_fallback).?;
    try std.testing.expectApproxEqAbs(@as(f32, 700.0), s3.x, 0.01);
    try std.testing.expectApproxEqAbs(@as(f32, 2000.0), s3.y, 0.01);
    try std.testing.expectApproxEqAbs(@as(f32, 385.0), s3.z, 0.01);

    // 4. Upside-down Avinoleum beacon (Roll=180 deg, Yaw=0, Gravity=PositiveZ, TeleportPos.Z=null)
    // Local (100, 200, 85) with Roll=180 flips Y & Z -> (+100, -200, -85) with Gravity=(0,0,1).
    var transform_inverted = [_]LevelEntityConfig.Vector{
        .{ .X = 100000, .Y = 200000, .Z = 30000 },
        .{ .X = 18000, .Y = 0, .Z = 0 },
        .{ .X = 100, .Y = 100, .Z = 100 },
    };
    const cfg_inverted: LevelEntityConfig = .{
        .Id = 4,
        .EntityId = 169750416,
        .BlueprintType = "Teleport2",
        .InSleep = false,
        .IsHidden = false,
        .MapId = 903,
        .AreaId = 1,
        .Transform = &transform_inverted,
        .ComponentsData = .{
            .BaseInfoComponent = .{
                .GravityConfig = .{
                    .GravityDirection = .{ .Type = "WorldAxis", .WorldAxis = "PositiveZ" },
                },
            },
            .TeleportComponent = .{
                .TeleporterId = 390302,
                .TeleportPos = .{ .X = 100.0, .Y = 200.0, .Z = null, .A = 90.0 },
            },
        },
    };
    const s4 = resolveTeleporterSpawn(false, &cfg_inverted).?;
    try std.testing.expectApproxEqAbs(@as(f32, 1100.0), s4.x, 0.01);
    try std.testing.expectApproxEqAbs(@as(f32, 1800.0), s4.y, 0.01);
    try std.testing.expectApproxEqAbs(@as(f32, 215.0), s4.z, 0.01);
    try std.testing.expect(s4.has_gravity_config);
    try std.testing.expectApproxEqAbs(@as(f32, 1.0), s4.gravity[2], 0.01);
}
