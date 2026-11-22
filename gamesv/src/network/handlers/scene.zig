const std = @import("std");
const pb = @import("proto").pb;
const Transaction = @import("../handlers.zig").Transaction;
const Connection = @import("../Connection.zig");
const Scene = @import("../../logic/Scene.zig");
const PlayerID = @import("../../logic/PlayerID.zig");
const mem = @import("../../mem.zig");
const EventQueue = @import("../../logic/EventQueue.zig");
const PositionComponent = @import("../../logic/component/entity/PositionComponent.zig");
const FileSystem = @import("common").FileSystem;
const Assets = @import("../../data/Assets.zig");
const BuffTimerScheduler = @import("../../logic/schedulers/BuffTimerScheduler.zig");
const entity_proto = @import("../../logic/helpers/entity_proto.zig");
const PlayerMapComponent = @import("../../logic/component/player/PlayerMapComponent.zig");
const PlayerBasicComponent = @import("../../logic/component/player/PlayerBasicComponent.zig");
const PlayerInventoryComponent = @import("../../logic/component/player/PlayerInventoryComponent.zig");
const PlayerWeaponComponent = @import("../../logic/component/player/PlayerWeaponComponent.zig");
const PlayerEchoComponent = @import("../../logic/component/player/PlayerEchoComponent.zig");
const PlayerQuestComponent = @import("../../logic/component/player/PlayerQuestComponent.zig");
const PlayerRoleComponent = @import("../../logic/component/player/PlayerRoleComponent.zig");
const PlayerProgressComponent = @import("../../logic/component/player/PlayerProgressComponent.zig");
const QuestEngine = @import("../../logic/helpers/quest_engine.zig");
const LevelPlayHelper = @import("../../logic/helpers/level_play.zig");
const RoleHelper = @import("../../logic/helpers/role.zig");
const map_handler = @import("map.zig");

pub fn onSceneTraceRequest(
    txn: *Transaction(pb.SceneTraceRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
    qctx: QuestEngine.QuestContext,
) !void {
    try QuestEngine.onSceneTrace(qctx, txn.message.SceneTraceId);
    if (qctx.scene != null) {
        QuestEngine.onSceneLoadingFinish(qctx) catch |err| QuestEngine.logHookErr("onSceneLoadingFinish", err);
    }
    var kbuf: [48]u8 = undefined;
    const key = try std.fmt.bufPrint(&kbuf, "scene:join_confirm:{d}", .{txn.message.SceneTraceId});
    _ = try pc.info.markClaim(alloc.gpa, key);
    try pc.save(fs, alloc.arena);

    txn.respond(.{});
}

pub fn onUpdateSceneDateRequest(
    txn: *Transaction(pb.UpdateSceneDateRequest),
    fs: *FileSystem,
    alloc: mem.Alloc,
    qctx: QuestEngine.QuestContext,
) !void {
    var hour = txn.message.Hour;
    var minute = txn.message.Minute;
    if (txn.message.AddDays != 0 and hour == 0 and minute == 0) {
        if (qctx.scene) |s| {
            hour = s.instance.map_time.hour;
            minute = s.instance.map_time.minute;
        }
    }
    try QuestEngine.setSceneTime(qctx, hour, minute, false);
    var day: u32 = 1;
    if (qctx.scene) |s| {
        s.instance.map_time.day +|= txn.message.AddDays;
        day = s.instance.map_time.day;
        try s.saveInstance(fs, alloc.gpa);
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .CurrDate = day,
    });
}

pub fn onChangeWeatherRequest(
    txn: *Transaction(pb.ChangeWeatherRequest),
    qctx: QuestEngine.QuestContext,
) !void {
    try QuestEngine.changeWeather(qctx, txn.message.WeatherId);
    txn.respond(.{
        .WeatherId = txn.message.WeatherId,
        .ErrorCode = .Success,
    });
}

pub fn onEntityActiveRequest(
    txn: *Transaction(pb.EntityActiveRequest),
    scene_opt: *?Scene,
    player_id: PlayerID,
    alloc: mem.Alloc,
    assets: *const Assets,
    buff_timers: *BuffTimerScheduler,
    io: std.Io,
) !void {
    const log = std.log.scoped(.entity_active);
    log.debug("request id: {d}", .{txn.message.EntityId});

    if (scene_opt.*) |*scene| {
        const ai_ctrl_id: i32 = if (scene.player_id != 0) scene.player_id else player_id.id;
        if (scene.net_id_map.get(txn.message.EntityId)) |_| {
            const now_ms = (std.Io.Clock.awake).now(io).toMilliseconds();
            if (entity_proto.build(
                alloc,
                assets,
                scene,
                buff_timers,
                txn.message.EntityId,
                now_ms,
            )) |entity_pb| {
                txn.respond(.{
                    .IsVisible = entity_pb.IsVisible,
                    .Rot = entity_pb.Rot,
                    .Pos = entity_pb.Pos,
                    .ComponentPbs = entity_pb.ComponentPbs,
                    .AiControlPlayerId = ai_ctrl_id,
                });
                return;
            } else |_| {}
        }
        txn.respond(.{
            .ErrorCode = .ErrSceneEntityNotExist,
            .IsVisible = false,
            .Pos = .{ .X = 0, .Y = 0, .Z = 0 },
            .Rot = .{ .Pitch = 0, .Roll = 0, .Yaw = 0 },
            .AiControlPlayerId = ai_ctrl_id,
        });
        return;
    }

    txn.respond(.{
        .ErrorCode = .ErrSceneEntityNotExist,
        .IsVisible = false,
        .Pos = .{ .X = 0, .Y = 0, .Z = 0 },
        .Rot = .{ .Pitch = 0, .Roll = 0, .Yaw = 0 },
        .AiControlPlayerId = player_id.id,
    });
}

pub fn onPlayerMotionRequest(txn: *Transaction(pb.PlayerMotionRequest)) !void {
    txn.respond(.{ .ErrorId = .Success });
}

pub fn onEntityPositionRequest(
    txn: *Transaction(pb.EntityPositionRequest),
    scene_opt: *?Scene,
    assets: *const Assets,
) !void {
    const req_cfg_id = txn.message.ConfigId;
    const target_map_id = if (txn.message.DungeonInstanceId > 0)
        txn.message.DungeonInstanceId
    else if (scene_opt.*) |*scene|
        scene.instance_id
    else
        8;

    if (req_cfg_id > 0) {
        if (scene_opt.*) |*scene| {
            if (scene.instance_id == target_map_id) {
                if (scene.findEntityNetIdByConfigId(req_cfg_id)) |net_id| {
                    if (scene.net_id_map.get(net_id)) |idx| {
                        if (scene.entities.items(.position)[idx]) |pos| {
                            txn.respond(.{
                                .ErrorCode = .Success,
                                .Pos = .{ .X = pos.location[0], .Y = pos.location[1], .Z = pos.location[2] },
                            });
                            return;
                        }
                    }
                }
            }
        }

        if (assets.findLevelEntity(target_map_id, req_cfg_id)) |cfg| {
            if (Assets.AssetIndexes.entityWorldPos(cfg)) |pos| {
                txn.respond(.{
                    .ErrorCode = .Success,
                    .Pos = .{ .X = pos[0], .Y = pos[1], .Z = pos[2] },
                });
                return;
            }
        }
    }

    txn.respond(.{
        .ErrorCode = .ErrSceneEntityNotExist,
        .Pos = .{},
    });
}

pub fn onSceneLoadingFinishRequest(
    txn: *Transaction(pb.SceneLoadingFinishRequest),
    events: *EventQueue,
    scene_opt: *?Scene,
    qctx: QuestEngine.QuestContext,
) !void {
    txn.respond(.{ .ErrorCode = .Success });
    if (scene_opt.*) |*scene| {
        if (scene.instance.players.len > 0) {
            scene.last_spatial_check_loc = scene.instance.players[0].location;
            events.enqueueBestEffort(.spatial_spawns_check, .{
                .location = scene.instance.players[0].location,
            });
        }
        try QuestEngine.onSceneLoadingFinish(qctx);
        if (scene.instance_id != 8) {
            try QuestEngine.onDungeonEvent(qctx, scene.instance_id, false);
        }
    }
}

fn currentRolePosition(
    scene: *Scene,
    controlled_roles: Scene.Query(&.{ Scene.Entity, *Scene.Entity.ConfigComponent, *Scene.Entity.PositionComponent, *Scene.Entity.DirectControlMarker }),
) ?*Scene.Entity.PositionComponent {
    const formation_idx = std.math.cast(usize, scene.formation_info.cur_formation) orelse return null;
    if (formation_idx >= scene.formation_info.formations.len) return null;
    const cur_role = scene.formation_info.formations[formation_idx].cur_role;
    var it = controlled_roles.iterator;
    while (it.next()) |comps| {
        const config = comps[1];
        if (config.entity_type == .player and config.config_type == .character and config.config_id == cur_role) {
            return comps[2];
        }
    }
    return null;
}

fn normalInteractRequestError(
    assets: *const Assets,
    scene: *Scene,
    entity_net_id: i64,
    option_index: i32,
) ?pb.ErrorCode {
    if (option_index < 0) return .ErrInteractOptionGuidInvalid;
    const config_id = scene.findConfigIdByNetId(entity_net_id) orelse return .ErrSceneEntityNotExist;
    const level_entity = assets.findLevelEntity(scene.instance_id, config_id) orelse return null;
    var components = level_entity.ComponentsData;
    if (assets.findTemplateConfig(level_entity.BlueprintType)) |template| {
        template.ComponentsData.mergeInto(&components);
    }
    const interact = components.InteractComponent orelse return null;
    if (interact.Disabled orelse false) return .ErrInteractComponentNotExist;
    const index = std.math.cast(usize, option_index) orelse return .ErrInteractOptionGuidInvalid;
    if (index >= interact.Options.len) return .ErrInteractOptionGuidInvalid;
    return null;
}

fn ensureVehicleEntity(scene: *Scene, net_id: i64) ?struct { entity: Scene.Entity, comp: *Scene.Entity.VehicleComponent } {
    const idx = scene.net_id_map.get(net_id) orelse return null;
    if (scene.entities.items(.vehicle)[idx] == null) {
        scene.entities.items(.vehicle)[idx] = .{ .source = .VehicleSourceNone };
    }
    const comp = if (scene.entities.items(.vehicle)[idx]) |*v| v else return null;
    return .{
        .entity = .{ .index = idx, .net_id = net_id },
        .comp = comp,
    };
}

fn applyVehicleTransition(
    qctx: QuestEngine.QuestContext,
    scene: *Scene,
    sender_id: i32,
    controlled_roles: Scene.Query(&.{ Scene.Entity, *Scene.Entity.ConfigComponent, *Scene.Entity.PositionComponent, *Scene.Entity.DirectControlMarker }),
    entity_id: i64,
    is_entering: bool,
    seat: i32,
    exit_type: ?pb.ExitVehicleType,
    client_predicted: bool,
) !pb.ErrorCode {
    if (ensureVehicleEntity(scene, entity_id)) |found| {
        found.comp.occupied = is_entering;
        if (is_entering) {
            if (currentRolePosition(scene, controlled_roles)) |role_position| {
                if (scene.entities.items(.position)[found.entity.index]) |*pos| {
                    pos.location = role_position.location;
                    pos.rotation = role_position.rotation;
                }
            }
            try scene.saveComponents(qctx.fs, qctx.alloc.gpa, found.entity, &.{ Scene.Entity.PositionComponent, Scene.Entity.VehicleComponent });
        } else {
            try scene.saveComponents(qctx.fs, qctx.alloc.gpa, found.entity, &.{Scene.Entity.VehicleComponent});
        }
    } else if (is_entering) {
        return .ErrVehicleEntityNotExist;
    }

    try qctx.conn.push(qctx.alloc.gpa, qctx.io, pb.VehicleUpdateNotify{
        .EntityId = if (is_entering) entity_id else 0,
        .PlayerId = sender_id,
        .SeatId = if (is_entering) seat else -1,
        .ExitType = if (is_entering) null else exit_type,
        .ClientPredicted = client_predicted,
    });

    const loc: [3]f32 = if (scene.instance.players.len > 0) scene.instance.players[0].location else .{ 0, 0, 0 };
    QuestEngine.onMove(qctx, loc) catch |err| {
        std.log.scoped(.level_play).warn("onPlayerMove failed: {t}", .{err});
    };

    return .Success;
}

pub fn onJoinSceneNotifyConfirmRequest(
    txn: *Transaction(pb.JoinSceneNotifyConfirmRequest),
    qctx: QuestEngine.QuestContext,
) !void {
    if (txn.message.SceneTraceId > 0) {
        QuestEngine.onSceneTrace(qctx, txn.message.SceneTraceId) catch |err| QuestEngine.logHookErr("onSceneTrace", err);
    }
    txn.respond(.{});
}

pub fn onVehicleManipulateRequest(
    txn: *Transaction(pb.VehicleManipulateRequest),
    qctx: QuestEngine.QuestContext,
    scene: *Scene,
    sender: PlayerID,
    controlled_roles: Scene.Query(&.{ Scene.Entity, *Scene.Entity.ConfigComponent, *Scene.Entity.PositionComponent, *Scene.Entity.DirectControlMarker }),
) !void {
    const req = txn.message;
    const exit_type: ?pb.ExitVehicleType = if (req.Exit) |ex| switch (ex) {
        .ExitType => |et| et,
    } else null;
    const code = try applyVehicleTransition(
        qctx,
        scene,
        sender.id,
        controlled_roles,
        req.EntityId,
        req.IsEntering,
        req.Seat,
        exit_type,
        req.ClientPredicted,
    );
    txn.respond(.{ .ErrorCode = code });
}

pub fn VehicleFinishRequest(txn: *Transaction(pb.VehicleFinishRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onVehicleRequest(
    txn: *Transaction(pb.VehicleRequest),
    qctx: QuestEngine.QuestContext,
    scene: *Scene,
    sender: PlayerID,
    controlled_roles: Scene.Query(&.{ Scene.Entity, *Scene.Entity.ConfigComponent, *Scene.Entity.PositionComponent, *Scene.Entity.DirectControlMarker }),
) !void {
    const req = txn.message;
    const code = try applyVehicleTransition(
        qctx,
        scene,
        sender.id,
        controlled_roles,
        req.EntityId,
        req.IsGetOn,
        req.SeatId,
        req.ExitType,
        req.ClientPredicted,
    );
    txn.respond(.{ .ErrorCode = code });
}

pub fn onVehicleChangePassengerRequest(
    txn: *Transaction(pb.VehicleChangePassengerRequest),
    alloc: mem.Alloc,
    io: std.Io,
    sender: PlayerID,
) !void {
    txn.respond(.{ .ErrorCode = .Success });
    try txn.conn.push(alloc.gpa, io, pb.VehiclePassengerUpdateNotify{
        .PlayerId = sender.id,
        .RoleId = txn.message.RoleId,
        .SeatId = txn.message.SeatId,
    });
}

pub fn onVehicleRemovePassengerRequest(
    txn: *Transaction(pb.VehicleRemovePassengerRequest),
    alloc: mem.Alloc,
    io: std.Io,
    sender: PlayerID,
) !void {
    txn.respond(.{ .ErrorCode = .Success });
    try txn.conn.push(alloc.gpa, io, pb.VehiclePassengerUpdateNotify{
        .PlayerId = sender.id,
        .RoleId = txn.message.RoleId,
        .SeatId = -1,
    });
}

pub fn onVehicleEnterSharingRequest(
    txn: *Transaction(pb.VehicleEnterSharingRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "vehicle:sharing", if (txn.message.IsEnter) 1 else 0);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onAwakeItemVehicleRequest(
    txn: *Transaction(pb.AwakeItemVehicleRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "vehicle:awake:{d}", .{txn.message.ItemID});
    try progress_comp.info.setKeyInt(alloc.gpa, key, txn.message.PosEntityId);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onEntityEnterVehicleRequest(
    txn: *Transaction(pb.EntityEnterVehicleRequest),
    alloc: mem.Alloc,
    io: std.Io,
    fs: *FileSystem,
    scene: *Scene,
    sender: PlayerID,
) !void {
    const req = txn.message;
    const found = ensureVehicleEntity(scene, req.VehicleCreatureId) orelse {
        txn.respond(.{ .ErrorCode = .ErrVehicleEntityNotExist });
        return;
    };

    found.comp.occupied = req.IsEntering;
    try scene.saveComponents(fs, alloc.gpa, found.entity, &.{Scene.Entity.VehicleComponent});
    txn.respond(.{ .ErrorCode = .Success });
    try txn.conn.push(alloc.gpa, io, pb.VehicleUpdateNotify{
        .EntityId = if (req.IsEntering) req.VehicleCreatureId else 0,
        .PlayerId = sender.id,
        .SeatId = if (req.IsEntering) req.Seat else -1,
        .ExitType = if (req.IsEntering) null else if (req.Exit) |ex| switch (ex) {
            .ExitType => |et| et,
        } else null,
        .ClientPredicted = req.ClientPredicted,
    });
}

pub fn onMovePackagePush(
    txn: *Transaction(pb.MovePackagePush),
    movable_entities: Scene.Query(&.{ Scene.Entity, *Scene.Entity.PositionComponent }),
    controlled_entities: Scene.Query(&.{ Scene.Entity, *Scene.Entity.DirectControlMarker }),
    scene: *Scene,
    sender: PlayerID,
    events: *EventQueue,
    assets: *const Assets,
    fs: *FileSystem,
    alloc: mem.Alloc,
    io: std.Io,
    quest_comp: *PlayerQuestComponent,
    role_comp: *PlayerRoleComponent,
    qctx: QuestEngine.QuestContext,
    maybe_coop: ?*@import("../../logic/CoopService.zig"),
) !void {
    const log = std.log.scoped(.client_movement);

    for (txn.message.MovingEntities.items) |data| {
        const entity, const position_comp = movable_entities.byNetId(data.EntityId) orelse {
            log.debug(
                "entity with id {d} is not movable (originator: {d}, sender: {d})",
                .{ data.EntityId, data.Originator, sender.id },
            );
            continue;
        };
        const is_occupied_player_vehicle = if (scene.net_id_map.get(data.EntityId)) |eidx| blk: {
            const slice = scene.entities.slice();
            const veh = slice.items(.vehicle)[eidx] orelse break :blk false;
            if (!veh.occupied) break :blk false;
            const pid = slice.items(.player_id)[eidx] orelse break :blk true;
            break :blk pid.id == sender.id;
        } else false;
        const controlled_by_player = controlled_entities.byNetId(data.EntityId) != null or is_occupied_player_vehicle;

        const last_move_replay = if (data.MoveInfos.items.len > 0) data.MoveInfos.items[data.MoveInfos.items.len - 1] else {
            log.warn(
                "received empty movement data for entity with id {d} (originator: {d}, sender: {d})",
                .{ data.EntityId, data.Originator, sender.id },
            );
            continue;
        };

        if (last_move_replay.Location) |location| {
            const next_location = [3]f32{ location.X, location.Y, location.Z };
            if (!std.math.isFinite(next_location[0]) or !std.math.isFinite(next_location[1]) or !std.math.isFinite(next_location[2])) continue;
            if (last_move_replay.Rotation) |rotation| {
                if (!std.math.isFinite(rotation.Roll) or !std.math.isFinite(rotation.Pitch) or !std.math.isFinite(rotation.Yaw)) continue;
            }
            var move_quest_pos: ?[3]f32 = null;
            if (controlled_by_player) {
                scene.player_movement_mode = last_move_replay.MovementMode;
                for (scene.instance.players) |*player| {
                    if (player.id == sender.id) {
                        player.location = next_location;
                        const dx = player.location[0] - scene.last_spatial_check_loc[0];
                        const dy = player.location[1] - scene.last_spatial_check_loc[1];
                        if (dx * dx + dy * dy >= 2500.0 * 2500.0) {
                            scene.last_spatial_check_loc = player.location;
                            events.enqueueBestEffort(.spatial_spawns_check, .{ .location = player.location });
                        }
                        if (quest_comp.info.active_quests.len > 0) {
                            const mdx = player.location[0] - scene.last_quest_move_loc[0];
                            const mdy = player.location[1] - scene.last_quest_move_loc[1];
                            const mdz = player.location[2] - scene.last_quest_move_loc[2];
                            if (!scene.has_quest_move_loc or mdx * mdx + mdy * mdy + mdz * mdz >= 50.0 * 50.0) {
                                move_quest_pos = player.location;
                            }
                        }
                    }
                }
            }
            position_comp.location = next_location;

            if (last_move_replay.Rotation) |rotation| {
                if (controlled_by_player) {
                    for (scene.instance.players) |*player| {
                        if (player.id == sender.id) {
                            player.rotation = .{ rotation.Roll, rotation.Pitch, rotation.Yaw };
                        }
                    }
                }
                position_comp.rotation = .{ rotation.Roll, rotation.Pitch, rotation.Yaw };
            }

            try events.enqueue(.entity_movement, .{ .entity = entity });
            if (controlled_by_player) {
                RoleHelper.healFormationIfNearNexus(txn.conn, scene, assets, role_comp, events, fs, alloc, io, next_location) catch |err| {
                    std.log.scoped(.role).warn("healFormationIfNearNexus failed: {t}", .{err});
                };
            }
            // Run QuestEngine.onMove after mutating position_comp since quest actions can spawn/remove entities and reallocate storage.
            if (move_quest_pos) |pos| {
                scene.last_quest_move_loc = pos;
                scene.has_quest_move_loc = true;
                try QuestEngine.onMove(qctx, pos);
            }
        }
    }

    if (maybe_coop) |coop| {
        coop.broadcastMoveExcept(alloc.gpa, io, sender.id, txn.message.MovingEntities);
    }
}

pub fn onApplySceneClientEntityIncIdRequest(
    txn: *Transaction(pb.ApplySceneClientEntityIncIdRequest),
    scene: *Scene,
    alloc: mem.Alloc,
    fs: *FileSystem,
    sender: @import("../../logic/PlayerID.zig"),
    config_entities: Scene.Query(&.{ Scene.Entity, *Scene.Entity.ConfigComponent }),
) !void {
    const log = std.log.scoped(.apply_scene_client_entity_inc_id);
    for (txn.message.EntityConfigId.items, 0..) |config_id, i| {
        log.debug("received entity config id [{d}]: {d}", .{ i, config_id });
    }

    var client_entity_info_list: std.ArrayList(pb.ClientEntityInfoPb) = .empty;

    for (txn.message.EntityConfigId.items) |config_id| {
        var found_net_id: ?i64 = null;

        var it = config_entities.iterator;
        while (it.next()) |comps| {
            const entity = comps[0];
            const config = comps[1];
            if (config.config_id == config_id) {
                found_net_id = entity.net_id;
                break;
            }
        }

        const net_id = found_net_id orelse blk: {
            const entity = try scene.spawn(alloc.gpa, fs, .{
                Scene.Entity.ConfigComponent{
                    .camp = 0,
                    .state = .default,
                    .entity_type = .scene_entity,
                    .config_type = .template,
                    .config_id = config_id,
                },
                Scene.Entity.PositionComponent{
                    .location = [_]f32{ -10000.0, -10000.0, -10000.0 },
                    .rotation = [_]f32{ 0.0, 0.0, 0.0 },
                },
                Scene.Entity.PlayerIDComponent{ .id = sender.id },
                Scene.Entity.ActorVisibleMarker{},
            });
            log.debug("spawned client entity config_id={d} net_id={d}", .{ config_id, entity.net_id });
            break :blk entity.net_id;
        };
        const info: pb.ClientEntityInfoPb = .{
            .EntityConfigId = config_id,
            .IsClientEntityConfig = true,
            .DOLLARFy = .{ .IncId = net_id },
            .WFy = .{ .ServerRealExist = found_net_id != null },
        };

        try client_entity_info_list.append(alloc.arena, info);
    }

    txn.respond(.{
        .ErrorCode = 0,
        .InstId = scene.instance_id,
        .ClientEntityInfo = client_entity_info_list,
    });
}

fn tryUnlockTeleporterForInteraction(
    conn: anytype,
    scene: *Scene,
    assets: *const Assets,
    map_comp: *PlayerMapComponent,
    quest_comp: *PlayerQuestComponent,
    fs: *FileSystem,
    alloc: mem.Alloc,
    io: std.Io,
    entity_id: i64,
    config_entities: Scene.Query(&.{ Scene.Entity, *Scene.Entity.ConfigComponent }),
) !bool {
    var changed = try map_comp.info.ensureDefaultUnlocks(alloc.gpa);
    var is_nexus_interaction = false;

    var target_config_id: ?i32 = null;
    var it = config_entities.iterator;
    while (it.next()) |comps| {
        if (comps[0].net_id == entity_id) {
            target_config_id = comps[1].config_id;
            if (RoleHelper.isResonanceNexusConfig(assets, comps[1].config_id)) {
                is_nexus_interaction = true;
            }
            break;
        }
    }

    const player_loc: ?[3]f32 = if (scene.instance.players.len > 0) scene.instance.players[0].location else null;

    var newly_unlocked_tps: std.ArrayList(i32) = .empty;
    const map_id = assets.resolveMapConfigId(scene.instance_id);
    for (assets.indexes.teleporterIndicesForMap(map_id)) |tp_idx| {
        const tp = assets.tables.teleporter.items[tp_idx];

        var matched = false;
        if (target_config_id) |cid| {
            if (cid == tp.TeleportEntityConfigId) matched = true;
        }
        if (!matched and entity_id == tp.TeleportEntityConfigId) {
            matched = true;
        }
        if (!matched and target_config_id == null) {
            if (player_loc) |loc| {
                if (assets.findLevelEntity(tp.MapId, tp.TeleportEntityConfigId)) |cfg| {
                    if (Assets.AssetIndexes.entityWorldPos(cfg)) |tp_pos| {
                        const tx = tp_pos[0] - loc[0];
                        const ty = tp_pos[1] - loc[1];
                        const tz = tp_pos[2] - loc[2];
                        if (tx * tx + ty * ty + tz * tz <= 800.0 * 800.0) {
                            matched = true;
                        }
                    }
                }
            }
        }

        if (matched) {
            if (RoleHelper.isResonanceNexusConfig(assets, tp.TeleportEntityConfigId) or tp.Type == 1) {
                is_nexus_interaction = true;
            }
            if (try map_comp.info.unlockTeleporter(alloc.gpa, tp.Id)) {
                changed = true;
                try newly_unlocked_tps.append(alloc.arena, tp.Id);
                if (std.math.cast(i32, tp.TeleportEntityConfigId)) |cfg_id32| {
                    if (cfg_id32 > 0) {
                        try quest_comp.info.setEntityState(alloc.gpa, tp.TeleportEntityConfigId, "关卡.Common.状态.激活");
                        try quest_comp.save(fs, alloc.arena);
                        if (scene.findEntityNetIdByConfigId(cfg_id32)) |nid| {
                            if (try scene.updateEntityStateTag(alloc.gpa, &assets.tables.gameplay_tag_remap, nid, "关卡.Common.状态.激活")) |upd| {
                                try QuestEngine.emitEntityStateTagUpdate(conn, alloc.gpa, alloc.arena, io, nid, upd);
                            }
                        }
                    }
                }
            }
            if (tp.Type == 1 and tp.FogId > 0 and try map_handler.unlockFogAndSiblings(conn, assets, map_comp, alloc.gpa, io, tp.FogId)) {
                changed = true;
            }
        }
    }

    if (changed) {
        try map_comp.save(fs, alloc.arena);
        if (newly_unlocked_tps.items.len > 0) {
            conn.pushBestEffort(alloc.gpa, io, pb.TeleportUpdateNotify{ .Ids = newly_unlocked_tps });
        }
    }
    return is_nexus_interaction;
}

pub fn onEntityInteractRequest(
    txn: *Transaction(pb.EntityInteractRequest),
    scene: *Scene,
    assets: *const Assets,
    events: *EventQueue,
    map_comp: *PlayerMapComponent,
    quest_comp: *PlayerQuestComponent,
    role_comp: *PlayerRoleComponent,
    qctx: QuestEngine.QuestContext,
    fs: *FileSystem,
    alloc: mem.Alloc,
    io: std.Io,
    config_entities: Scene.Query(&.{ Scene.Entity, *Scene.Entity.ConfigComponent }),
) !void {
    if (normalInteractRequestError(assets, scene, txn.message.EntityId, txn.message.OptionIndex)) |error_code| {
        txn.respond(.{ .ErrorCode = error_code, .Interacting = false });
        return;
    }
    if (try tryUnlockTeleporterForInteraction(
        txn.conn,
        scene,
        assets,
        map_comp,
        quest_comp,
        fs,
        alloc,
        io,
        txn.message.EntityId,
        config_entities,
    )) {
        try RoleHelper.reviveAndHealFormation(txn.conn, scene, role_comp, events, fs, alloc, io);
    }
    try QuestEngine.onInteract(qctx, txn.message.EntityId, txn.message.OptionIndex);
    _ = try LevelPlayHelper.onEntityInteractFallback(.{
        .assets = qctx.assets,
        .fs = qctx.fs,
        .conn = qctx.conn,
        .scene = scene,
        .scene_comp = qctx.scene_comp,
        .events = qctx.events,
        .alloc = qctx.alloc,
        .io = qctx.io,
        .basic_comp = qctx.basic_comp,
        .inventory_comp = qctx.inventory_comp,
        .weapon_comp = qctx.weapon_comp,
        .echo_comp = qctx.echo_comp,
        .map_comp = qctx.map_comp,
        .quest_comp = qctx.quest_comp,
        .role_comp = role_comp,
    }, txn.message.EntityId);
    txn.respond(.{
        .ErrorCode = .Success,
        .Interacting = false,
    });
}

fn makeLevelPlayCtx(
    conn: *Connection,
    scene: *Scene,
    assets: *const Assets,
    events: *EventQueue,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    map_comp: *PlayerMapComponent,
    quest_comp: *PlayerQuestComponent,
    role_comp: *PlayerRoleComponent,
    fs: *FileSystem,
    alloc: mem.Alloc,
    io: std.Io,
) LevelPlayHelper.LevelPlayContext {
    return .{
        .assets = assets,
        .fs = fs,
        .conn = conn,
        .scene = scene,
        .events = events,
        .alloc = alloc,
        .io = io,
        .basic_comp = basic_comp,
        .inventory_comp = inventory_comp,
        .weapon_comp = weapon_comp,
        .echo_comp = echo_comp,
        .map_comp = map_comp,
        .quest_comp = quest_comp,
        .role_comp = role_comp,
    };
}

pub fn onGetRewardTreasureBoxRequest(
    txn: *Transaction(pb.GetRewardTreasureBoxRequest),
    scene: *Scene,
    assets: *const Assets,
    events: *EventQueue,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    map_comp: *PlayerMapComponent,
    quest_comp: *PlayerQuestComponent,
    role_comp: *PlayerRoleComponent,
    fs: *FileSystem,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    try LevelPlayHelper.onGetRewardTreasureBox(
        makeLevelPlayCtx(txn.conn, scene, assets, events, basic_comp, inventory_comp, weapon_comp, echo_comp, map_comp, quest_comp, role_comp, fs, alloc, io),
        txn.message.EntityId,
    );
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onAnimalDropRequest(
    txn: *Transaction(pb.AnimalDropRequest),
    scene: *Scene,
    assets: *const Assets,
    events: *EventQueue,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    map_comp: *PlayerMapComponent,
    quest_comp: *PlayerQuestComponent,
    role_comp: *PlayerRoleComponent,
    fs: *FileSystem,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    try LevelPlayHelper.onAnimalDrop(
        makeLevelPlayCtx(txn.conn, scene, assets, events, basic_comp, inventory_comp, weapon_comp, echo_comp, map_comp, quest_comp, role_comp, fs, alloc, io),
        txn.message.EntityId,
    );
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onAnimalDieRequest(
    txn: *Transaction(pb.AnimalDieRequest),
    scene: *Scene,
    assets: *const Assets,
    events: *EventQueue,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    map_comp: *PlayerMapComponent,
    quest_comp: *PlayerQuestComponent,
    role_comp: *PlayerRoleComponent,
    fs: *FileSystem,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    try LevelPlayHelper.onAnimalDrop(
        makeLevelPlayCtx(txn.conn, scene, assets, events, basic_comp, inventory_comp, weapon_comp, echo_comp, map_comp, quest_comp, role_comp, fs, alloc, io),
        txn.message.EntityId,
    );
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onAnimalDestroyRequest(
    txn: *Transaction(pb.AnimalDestroyRequest),
    scene: *Scene,
    assets: *const Assets,
    events: *EventQueue,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    map_comp: *PlayerMapComponent,
    quest_comp: *PlayerQuestComponent,
    role_comp: *PlayerRoleComponent,
    fs: *FileSystem,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    try LevelPlayHelper.onAnimalDestroy(.{
        .assets = assets,
        .fs = fs,
        .conn = txn.conn,
        .scene = scene,
        .events = events,
        .alloc = alloc,
        .io = io,
        .basic_comp = basic_comp,
        .inventory_comp = inventory_comp,
        .weapon_comp = weapon_comp,
        .echo_comp = echo_comp,
        .map_comp = map_comp,
        .quest_comp = quest_comp,
        .role_comp = role_comp,
    }, txn.message.EntityId);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onEntityDynamicInteractRequest(
    txn: *Transaction(pb.EntityDynamicInteractRequest),
    scene: *Scene,
    assets: *const Assets,
    events: *EventQueue,
    map_comp: *PlayerMapComponent,
    quest_comp: *PlayerQuestComponent,
    role_comp: *PlayerRoleComponent,
    qctx: QuestEngine.QuestContext,
    fs: *FileSystem,
    alloc: mem.Alloc,
    io: std.Io,
    config_entities: Scene.Query(&.{ Scene.Entity, *Scene.Entity.ConfigComponent }),
) !void {
    const dynamic_option = scene.findDynamicInteractOption(txn.message.OptionGuid) orelse {
        txn.respond(.{ .ErrorCode = .ErrInteractOptionGuidInvalid, .Interacting = false });
        return;
    };
    if (dynamic_option.entity_net_id != txn.message.EntityId) {
        txn.respond(.{ .ErrorCode = .ErrInteractOptionGuidInvalid, .Interacting = false });
        return;
    }
    if (dynamic_option.kind == .none) {
        txn.respond(.{ .ErrorCode = .ErrInteractOptionGuidInvalid, .Interacting = false });
        return;
    }
    if (try tryUnlockTeleporterForInteraction(
        txn.conn,
        scene,
        assets,
        map_comp,
        quest_comp,
        fs,
        alloc,
        io,
        txn.message.EntityId,
        config_entities,
    )) {
        try RoleHelper.reviveAndHealFormation(txn.conn, scene, role_comp, events, fs, alloc, io);
    }
    try QuestEngine.executeDynamicInteractOption(qctx, txn.message.EntityId, txn.message.OptionGuid);
    try QuestEngine.onInteract(qctx, txn.message.EntityId, 0);
    txn.respond(.{
        .ErrorCode = .Success,
        .Interacting = false,
    });
}

pub fn onEntityPosAbnormalRequest(
    txn: *Transaction(pb.EntityPosAbnormalRequest),
    scene_opt: *?Scene,
    assets: *const Assets,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    if (txn.message.IsAbnormal) {
        if (scene_opt.*) |*scene| {
            if (scene.net_id_map.get(txn.message.EntityId)) |idx| {
                const cfg_id = scene.entities.items(.config)[idx].config_id;
                if (cfg_id > 0) {
                    if (assets.findLevelEntity(scene.instance_id, cfg_id)) |cfg| {
                        if (Assets.AssetIndexes.entityWorldPos(cfg)) |pos| {
                            _ = scene.setEntityPosition(txn.message.EntityId, pos, null);
                            try txn.conn.push(alloc.gpa, io, pb.EntityPosResetNotify{ .EntityId = cfg_id });
                        }
                    }
                }
            }
        }
    }
    txn.respond(.{ .Code = .Success });
}
