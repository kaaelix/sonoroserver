const std = @import("std");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const FileSystem = @import("common").FileSystem;
const sliceToArrayList = @import("../component/entity/EntityComponentStorage.zig").sliceToArrayList;
const Assets = @import("../../data/Assets.zig");
const RoleEntityTemplates = @import("../templates/RoleEntityTemplates.zig");
const PlayerEntityTemplates = @import("../templates/PlayerEntityTemplates.zig");
const EventQueue = @import("../EventQueue.zig");
const PlayerID = @import("../PlayerID.zig");
const Scene = @import("../Scene.zig");
const SceneInstance = @import("../../fs/SceneInstance.zig");
const Connection = @import("../../network/Connection.zig");
const BuffTimerScheduler = @import("../schedulers/BuffTimerScheduler.zig");
const DirtySaveQueue = @import("../schedulers/DirtySaveQueue.zig");
const PlayerBasicComponent = @import("../component/player/PlayerBasicComponent.zig");
const PlayerSceneComponent = @import("../component/player/PlayerSceneComponent.zig");
const PlayerRoleComponent = @import("../component/player/PlayerRoleComponent.zig");
const PlayerWeaponComponent = @import("../component/player/PlayerWeaponComponent.zig");
const PlayerMotorComponent = @import("../component/player/PlayerMotorComponent.zig");
const PlayerEchoComponent = @import("../component/player/PlayerEchoComponent.zig");
const PlayerMapComponent = @import("../component/player/PlayerMapComponent.zig");
const PlayerInventoryComponent = @import("../component/player/PlayerInventoryComponent.zig");
const PlayerQuestComponent = @import("../component/player/PlayerQuestComponent.zig");
const PlayerProgressComponent = @import("../component/player/PlayerProgressComponent.zig");
const QuestInfo = @import("../../fs/QuestInfo.zig");
const ProgressInfo = @import("../../fs/ProgressInfo.zig");
const InteractConfig = @import("../../data/tables/entity_components/Interact.zig");
const file_util = @import("../../fs/file_util.zig");
const FormationInfo = @import("../../fs/FormationInfo.zig");
const RoleInfo = @import("../../fs/RoleInfo.zig");
const TrialRoles = @import("../helpers/trial_roles.zig");
const RoleHelper = @import("../helpers/role.zig");
const autopilot = @import("../helpers/autopilot.zig");
const QuestEngine = @import("../helpers/quest_engine.zig");
const LevelPlayHelper = @import("../helpers/level_play.zig");
const spline_state = @import("../helpers/spline_state.zig");
const spawn_cmd = @import("../commands/entity.zig").spawn;
const Entity = Scene.Entity;
const Io = std.Io;
const lahai_roi_dungeon_id = 906;
const roulette_slot_count = 8;
const default_formation_roles = [_]i32{ 1502, 1402, 1202 };

fn ensureValidFormation(formation_info: *FormationInfo, basic_comp: *PlayerBasicComponent) void {
    const first = &formation_info.formations[0];
    const is_empty = for (first.roles) |maybe_role| {
        if (maybe_role != null) break false;
    } else true;

    if (is_empty) {
        const rover_id: i32 = if (basic_comp.info.attributes.sex == 1) 1501 else 1502;
        const starter_roles = [_]i32{ rover_id, 1402, 1202 };
        first.cur_role = starter_roles[0];
        for (starter_roles, 0..) |role_id, i| {
            first.roles[i] = .{
                .role_id = role_id,
                .entity_id = -1,
                .on_stage_without_control = false,
            };
        }
        formation_info.cur_formation = 0;
    } else if (formation_info.cur_formation < 0 or
        formation_info.cur_formation >= 20)
    {
        formation_info.cur_formation = 0;
    }
}

fn rouletteSkillIds(arena: std.mem.Allocator, ids: []const i32) !std.ArrayList(i32) {
    const items = try arena.alloc(i32, roulette_slot_count);
    @memset(items, 0);
    const count = @min(ids.len, roulette_slot_count);
    @memcpy(items[0..count], ids[0..count]);
    return sliceToArrayList(i32, items);
}

fn notifyTransportRoadways(fs: *FileSystem, alloc: mem.Alloc, scene: *Scene, conn: *Connection, assets: *const Assets, io: std.Io) !void {
    const instance_dungeon = assets.tables.instance_dungeon.getDataById(scene.instance_id) orelse return;
    const roads = try autopilot.roadIdsForMap(fs, alloc, instance_dungeon.MapConfigId);
    if (roads.items.len == 0) return;

    try conn.push(alloc.gpa, io, pb.SceneRoadSyncNotify{
        .InstanceId = scene.instance_id,
        .EnabledRoads = roads,
    });
}

fn notifyInfrastructureRoadData(alloc: mem.Alloc, conn: *Connection, assets: *const Assets, io: std.Io) !void {
    var roads: std.ArrayList(pb.InfrOneRoad) = .empty;
    for (assets.tables.infr_road_build.items) |cfg| {
        if (cfg.DungeonId != lahai_roi_dungeon_id) continue;

        try roads.append(alloc.arena, .{
            .RoadId = cfg.Id,
            .status = .InfrStatusComplete,
            .CompleteTime = 0,
            .TotalGiftCount = 0,
            .LastGiftTime = 0,
        });
    }

    try conn.push(alloc.gpa, io, pb.InfrRoadUpdateNotify{
        .RoadInfo = .{ .Roads = roads },
    });
}

fn containsI32(items: []const i32, value: i32) bool {
    for (items) |item| {
        if (item == value) return true;
    }
    return false;
}

fn progressInt(info: ProgressInfo, key: []const u8) ?i64 {
    for (info.dynamic.ints) |entry| {
        if (std.mem.eql(u8, entry.key, key)) return entry.val;
    }
    return null;
}

fn storedWorldFloat(info: ProgressInfo, config_id: i32, field: []const u8) ?f32 {
    var key_buf: [64]u8 = undefined;
    const key = std.fmt.bufPrint(&key_buf, "world_pos:{d}:{s}", .{ config_id, field }) catch return null;
    const raw = progressInt(info, key) orelse return null;
    const bits = std.math.cast(u32, raw) orelse return null;
    const value: f32 = @bitCast(bits);
    return if (std.math.isFinite(value)) value else null;
}

fn storedWorldLocation(info: ProgressInfo, config_id: i32) ?[3]f32 {
    const x = storedWorldFloat(info, config_id, "x") orelse return null;
    const y = storedWorldFloat(info, config_id, "y") orelse return null;
    const z = storedWorldFloat(info, config_id, "z") orelse return null;
    return .{ x, y, z };
}

fn pushStoredWorldTransform(
    scene: *Scene,
    conn: *Connection,
    alloc: mem.Alloc,
    io: std.Io,
    progress_info: ProgressInfo,
    config_id: i32,
    net_id: i64,
) void {
    if (storedWorldFloat(progress_info, config_id, "roll") == null and
        storedWorldFloat(progress_info, config_id, "pitch") == null and
        storedWorldFloat(progress_info, config_id, "yaw") == null)
    {
        return;
    }
    const index = scene.net_id_map.get(net_id) orelse return;
    const position = scene.entities.items(.position)[index] orelse return;
    var move_infos: std.ArrayList(pb.MoveReplaySample) = .empty;
    move_infos.append(alloc.arena, .{
        .Location = .{ .X = position.location[0], .Y = position.location[1], .Z = position.location[2] },
        .Rotation = .{ .Roll = position.rotation[0], .Pitch = position.rotation[1], .Yaw = position.rotation[2] },
    }) catch return;
    var entities: std.ArrayList(pb.MovingEntityData) = .empty;
    entities.append(alloc.arena, .{
        .EntityId = net_id,
        .Originator = net_id,
        .MoveInfos = move_infos,
        .ForcePush = true,
    }) catch return;
    conn.pushBestEffort(alloc.gpa, io, pb.MovePackageNotify{ .MovingEntities = entities });
}

fn pushStoredElevatorState(
    conn: *Connection,
    alloc: mem.Alloc,
    io: std.Io,
    progress_info: ProgressInfo,
    config_id: i32,
    net_id: i64,
) void {
    if (config_id <= 0) return;
    var key_buf: [48]u8 = undefined;
    const key = std.fmt.bufPrint(&key_buf, "elevator:{d}", .{config_id}) catch return;
    const raw_floor = progressInt(progress_info, key) orelse return;
    const floor = std.math.cast(i32, raw_floor) orelse return;
    conn.pushBestEffort(alloc.gpa, io, pb.ElevatorMoveNotify{
        .EntityId = net_id,
        .Location = floor,
    });
}

fn storedProgressFloat(info: ProgressInfo, key: []const u8) ?f32 {
    const raw = progressInt(info, key) orelse return null;
    const bits = std.math.cast(u32, raw) orelse return null;
    const value: f32 = @bitCast(bits);
    return if (std.math.isFinite(value)) value else null;
}

fn entityInteractConfig(assets: *const Assets, scene: *Scene, config_id: i32) ?InteractConfig {
    const config = assets.findLevelEntity(scene.instance_id, config_id) orelse return null;
    var components = config.ComponentsData;
    if (assets.findTemplateConfig(config.BlueprintType)) |template| {
        template.ComponentsData.mergeInto(&components);
    }
    return components.InteractComponent;
}

fn applyStoredWorldEntityOverrides(
    scene: *Scene,
    assets: *const Assets,
    fs: *FileSystem,
    alloc: mem.Alloc,
    progress_comp: *PlayerProgressComponent,
    config_id: i32,
    net_id: i64,
) !?pb.GravityFlipType {
    if (config_id <= 0) return null;
    const index = scene.net_id_map.get(net_id) orelse return null;
    const info = progress_comp.info;
    var changed = false;

    const x = storedWorldFloat(info, config_id, "x");
    const y = storedWorldFloat(info, config_id, "y");
    const z = storedWorldFloat(info, config_id, "z");
    if (x != null and y != null and z != null) {
        var position = scene.entities.items(.position)[index] orelse Scene.Entity.PositionComponent{};
        position.location = .{ x.?, y.?, z.? };
        if (storedWorldFloat(info, config_id, "roll")) |roll| position.rotation[0] = roll;
        if (storedWorldFloat(info, config_id, "pitch")) |pitch| position.rotation[1] = pitch;
        if (storedWorldFloat(info, config_id, "yaw")) |yaw| position.rotation[2] = yaw;
        scene.entities.items(.position)[index] = position;
        changed = true;
    }

    if (progressInt(info, try std.fmt.allocPrint(alloc.arena, "world_camp:{d}", .{config_id}))) |camp_raw| {
        if (std.math.cast(u8, camp_raw)) |camp8| {
            scene.entities.items(.config)[index].camp = camp8;
            changed = true;
        }
    }

    const lift_key = try std.fmt.allocPrint(alloc.arena, "lift_target:{d}", .{config_id});
    if (progressInt(info, lift_key)) |location| {
        if (std.math.cast(i32, location)) |lift_location| {
            scene.entities.items(.world_item)[index].lift = .{ .location = lift_location };
            changed = true;
        }
    }

    var gravity_type: ?pb.GravityFlipType = null;
    const gravity_key = try std.fmt.allocPrint(alloc.arena, "gravity:{d}", .{config_id});
    if (progressInt(info, gravity_key)) |raw| {
        if (raw >= 0 and raw <= 3) {
            gravity_type = @enumFromInt(raw);
            scene.entities.items(.world_item)[index].gravity_flip = @intCast(raw);
            changed = true;
        }
    } else if (assets.findLevelEntity(scene.instance_id, config_id)) |lcfg| {
        if (lcfg.ComponentsData.GravityFlipComponent) |gfc| {
            if (gfc.DefaultGravity) |dg| {
                if (dg >= 0 and dg <= 3) {
                    scene.entities.items(.world_item)[index].gravity_flip = dg;
                }
            }
        }
    }

    if (assets.findLevelEntity(scene.instance_id, config_id)) |lcfg| {
        var merged = lcfg.ComponentsData;
        if (assets.findTemplateConfig(lcfg.BlueprintType)) |tpl| {
            tpl.ComponentsData.mergeInto(&merged);
        }
        for (merged.extra) |entry| {
            switch (entry) {
                .FanComponent => |fc| {
                    const fkey_cfg = try std.fmt.allocPrint(alloc.arena, "fan_turns:{d}", .{config_id});
                    const fkey_net = try std.fmt.allocPrint(alloc.arena, "fan_turns:{d}", .{net_id});
                    const stored = progressInt(info, fkey_net) orelse progressInt(info, fkey_cfg);
                    const turns: i32 = if (stored) |s|
                        std.math.cast(i32, s) orelse 0
                    else if (fc.InitCircle) |ic|
                        std.math.cast(i32, ic) orelse 0
                    else
                        0;
                    scene.entities.items(.world_item)[index].fan_turns = turns;
                },
                .HookLockPoint => |hlp| {
                    const hkey_cfg = try std.fmt.allocPrint(alloc.arena, "hook_point:active:{d}", .{config_id});
                    const hkey_net = try std.fmt.allocPrint(alloc.arena, "hook_point:active:{d}", .{net_id});
                    const is_active = (progressInt(info, hkey_net) orelse progressInt(info, hkey_cfg) orelse 0) == 1;
                    const default_disabled = (hlp.DisableAtCreateStage orelse false) or (hlp.Disabled orelse false);
                    scene.entities.items(.world_item)[index].hook_disabled = if (is_active) false else default_disabled;
                },
                else => {},
            }
        }
    }

    const pdir_cfg = try std.fmt.allocPrint(alloc.arena, "patrol:dir:{d}", .{config_id});
    const pdir_net = try std.fmt.allocPrint(alloc.arena, "patrol:dir:{d}", .{net_id});
    if (progressInt(info, pdir_net) orelse progressInt(info, pdir_cfg)) |pdir| {
        scene.entities.items(.world_item)[index].patrol_dir = pdir != 0;
    }

    const bc_act_cfg = try std.fmt.allocPrint(alloc.arena, "be_ctrl:active:{d}", .{config_id});
    const bc_act_net = try std.fmt.allocPrint(alloc.arena, "be_ctrl:active:{d}", .{net_id});
    const bc_show_cfg = try std.fmt.allocPrint(alloc.arena, "be_ctrl:show:{d}", .{config_id});
    const bc_show_net = try std.fmt.allocPrint(alloc.arena, "be_ctrl:show:{d}", .{net_id});
    const bc_active = (progressInt(info, bc_act_net) orelse progressInt(info, bc_act_cfg) orelse 0) != 0;
    const bc_show = (progressInt(info, bc_show_net) orelse progressInt(info, bc_show_cfg) orelse 0) != 0;
    if (bc_active or bc_show) {
        scene.entities.items(.world_item)[index].be_controlled_player_id = if (bc_active) scene.currentRoleEntityId() else null;
        scene.entities.items(.world_item)[index].be_controlled_show = bc_show;
    }

    var spline_entity_id: i32 = 0;
    var spline_active = false;
    const item_status_key = try std.fmt.allocPrint(alloc.arena, "scene_item_spline_status:{d}", .{config_id});
    const item_active_key = try std.fmt.allocPrint(alloc.arena, "scene_item_spline_active:{d}", .{config_id});
    const item_spline_key = try std.fmt.allocPrint(alloc.arena, "scene_item_spline_cfg:{d}", .{config_id});
    const stored_status = progressInt(info, item_status_key) orelse 0;
    if ((progressInt(info, item_active_key) orelse 0) == 1 and
        (stored_status == @intFromEnum(pb.MoveStatus.Moving) or stored_status == @intFromEnum(pb.MoveStatus.Switch)))
    {
        spline_entity_id = std.math.cast(i32, progressInt(info, item_spline_key) orelse 0) orelse 0;
        spline_active = spline_entity_id > 0;
    }
    if (!spline_active) {
        const quest_active_key = try std.fmt.allocPrint(alloc.arena, "quest_spline_active:{d}", .{config_id});
        const quest_spline_key = try std.fmt.allocPrint(alloc.arena, "quest_spline:{d}", .{config_id});
        if ((progressInt(info, quest_active_key) orelse 0) == 1) {
            spline_entity_id = std.math.cast(i32, progressInt(info, quest_spline_key) orelse 0) orelse 0;
            spline_active = spline_entity_id > 0;
        }
    }
    if (!spline_active) {
        const vehicle_active_key = try std.fmt.allocPrint(alloc.arena, "vehicle:spline_active:{d}", .{config_id});
        const vehicle_spline_key = try std.fmt.allocPrint(alloc.arena, "vehicle:spline:{d}", .{config_id});
        if ((progressInt(info, vehicle_active_key) orelse 0) == 1) {
            spline_entity_id = std.math.cast(i32, progressInt(info, vehicle_spline_key) orelse 0) orelse 0;
            spline_active = spline_entity_id > 0;
        }
    }
    if (spline_active) {
        const position = scene.entities.items(.position)[index] orelse Scene.Entity.PositionComponent{};
        scene.entities.items(.world_item)[index].move_spline = .{
            .move_status = .Moving,
            .spline_entity_id = spline_entity_id,
            .runtime_data = .{ .CurPos = .{
                .X = position.location[0],
                .Y = position.location[1],
                .Z = position.location[2],
            } },
            .move_spline_config = try spline_state.restoreMoveSplineConfig(alloc.arena, info, config_id),
        };
    } else {
        scene.entities.items(.world_item)[index].move_spline = null;
    }

    const target_fields = [_][]const u8{ "x", "y", "z", "move_time", "stay_time", "max_speed", "acceleration" };
    var target_values: [7]?f32 = undefined;
    for (target_fields, 0..) |field, i| {
        const key = try std.fmt.allocPrint(alloc.arena, "scene_item_move_target:{d}:{s}", .{ config_id, field });
        target_values[i] = storedProgressFloat(info, key);
    }
    const target_active_key = try std.fmt.allocPrint(alloc.arena, "scene_item_move_target_active:{d}", .{config_id});
    if ((progressInt(info, target_active_key) orelse 0) == 1 and target_values[0] != null and target_values[1] != null and target_values[2] != null) {
        scene.entities.items(.world_item)[index].move_to_point = .{
            .target_pos = .{ target_values[0].?, target_values[1].?, target_values[2].? },
            .move_time = target_values[3] orelse 0,
            .stay_time = target_values[4] orelse 0,
            .max_speed = target_values[5] orelse 0,
            .acceleration = target_values[6] orelse 0,
        };
    } else {
        scene.entities.items(.world_item)[index].move_to_point = null;
    }

    if (scene.entities.items(.config)[index].entity_type == .scene_item) {
        scene.entities.items(.world_item)[index].scene_item = Scene.Entity.SceneItemComponent.fromProgress(info, scene.player_id, config_id);
    } else {
        scene.entities.items(.world_item)[index].scene_item = null;
    }

    if (scene.entities.items(.world_item)[index].interact) |*interact| interact.deinit(alloc.gpa);
    scene.entities.items(.world_item)[index].interact = null;
    if (entityInteractConfig(assets, scene, config_id)) |interact_config| {
        if (!(interact_config.Disabled orelse false)) {
            scene.entities.items(.world_item)[index].interact = try Scene.Entity.InteractStateComponent.fromConfig(
                alloc.gpa,
                interact_config,
                scene.player_id,
                scene.instance_id,
                config_id,
            );
        }
    }

    if (changed) {
        try scene.saveComponents(fs, alloc.gpa, .{ .index = index, .net_id = net_id }, &.{
            Scene.Entity.PositionComponent,
        });
    }
    return gravity_type;
}

fn appendUniqueI32(items: *std.ArrayList(i32), arena: std.mem.Allocator, value: i32) !void {
    if (containsI32(items.items, value)) return;
    try items.append(arena, value);
}

fn appendCompletedRoadDataLayers(scene_info: *pb.SceneInformation, assets: *const Assets, arena: std.mem.Allocator) !void {
    var repaired_layers: std.ArrayList(i32) = .empty;
    var unrepaired_layers: std.ArrayList(i32) = .empty;

    for (assets.tables.infr_road_build.items) |cfg| {
        if (cfg.DungeonId != lahai_roi_dungeon_id) continue;

        for (cfg.LoadDataLayers) |layer| {
            try appendUniqueI32(&repaired_layers, arena, layer);
        }
        for (cfg.UnloadDataLayers) |layer| {
            try appendUniqueI32(&unrepaired_layers, arena, layer);
        }
    }

    var data_layers: std.ArrayList(i32) = .empty;
    for (scene_info.DataLayers.items) |layer| {
        if (!containsI32(unrepaired_layers.items, layer)) {
            try appendUniqueI32(&data_layers, arena, layer);
        }
    }
    for (repaired_layers.items) |layer| {
        try appendUniqueI32(&data_layers, arena, layer);
    }

    scene_info.DataLayers = data_layers;
}

pub fn handleScenePendingUpdates(
    _: EventQueue.Dequeue(.scene_pending_updates),
    scene: *Scene,
    conn: *Connection,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    var data: std.ArrayList(pb.CombatReceiveData) = .empty;
    _ = try scene.appendBattleStateNotify(alloc.arena, &data);
    const combine_detaches = scene.pendingCombineDetaches();
    for (combine_detaches) |detach| try data.append(alloc.arena, Scene.removeCombineNotify(detach));
    if (data.items.len == 0) return;

    try conn.push(alloc.gpa, io, pb.CombatReceivePackNotify{ .Data = data });
    for (combine_detaches) |detach| try scene.signalFsmDissolveCombine(alloc.gpa, detach.combine_entity_id);
    scene.clearPendingCombineDetaches();
}

pub fn exploreSkillNotify(alloc: mem.Alloc, scene: *Scene, conn: *Connection, io: std.Io) !void {
    var roulette_info: std.ArrayList(pb.ExploreSkillRoulette) = .empty;
    defer roulette_info.deinit(alloc.gpa);
    try roulette_info.append(alloc.gpa, .{
        .SkillIds = try rouletteSkillIds(alloc.arena, scene.explore_tools_info.roulette),
        .ExtraItemId = scene.explore_tools_info.explore_extra_item_id,
        .ExploreSkill = scene.explore_tools_info.active_explore_skill,
    });
    try roulette_info.append(alloc.gpa, .{
        .SkillIds = try rouletteSkillIds(alloc.arena, scene.explore_tools_info.function_roulette),
        .ExtraItemId = scene.explore_tools_info.function_extra_item_id,
        .ExploreSkill = scene.explore_tools_info.active_function_skill,
    });
    try roulette_info.append(alloc.gpa, .{});
    try roulette_info.append(alloc.gpa, .{
        .SkillIds = try rouletteSkillIds(alloc.arena, scene.explore_tools_info.motorcycle_roulette),
        .ExtraItemId = scene.explore_tools_info.motorcycle_extra_item_id,
        .ExploreSkill = scene.explore_tools_info.active_motorcycle_skill,
    });

    const vision_explore_notify: pb.VisionExploreSkillNotify = .{
        .ExploreSkill = 1001,
    };
    try conn.push(alloc.gpa, io, pb.ExploreToolAllNotify{
        .ExploreSkill = scene.explore_tools_info.active_explore_skill,
        .SkillList = sliceToArrayList(i32, scene.explore_tools_info.unlocked_explore_skills),
    });
    try conn.push(alloc.gpa, io, pb.ExploreSkillRouletteUpdateNotify{
        .RouletteInfo = roulette_info,
    });
    try conn.push(alloc.gpa, io, vision_explore_notify);
}

fn has_scene_data(fs: *FileSystem, arena: std.mem.Allocator, player_id: i32) bool {
    const path = std.fmt.allocPrint(arena, "player/{d}/scene", .{player_id}) catch return false;
    const dir = fs.root_dir.openDir(fs.io, path, .{ .iterate = true }) catch return false;
    defer dir.close(fs.io);
    var it = dir.iterate();
    while (it.next(fs.io) catch return false) |entry| {
        if (entry.kind == .directory) return true;
    }
    return false;
}

pub fn onInitialSceneJoin(
    event: EventQueue.Dequeue(.initial_scene_join),
    events: *EventQueue,
    fs: *FileSystem,
    alloc: mem.Alloc,
    player_id: PlayerID,
    assets: *const Assets,
    scene_comp: *PlayerSceneComponent,
    basic_comp: *PlayerBasicComponent,
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    quest_comp: *const PlayerQuestComponent,
    cur_scene: *?Scene,
    buff_timers: *BuffTimerScheduler,
    dirty_saves: *DirtySaveQueue,
    io: Io,
) !void {
    const log = std.log.scoped(.initial_scene_join);
    const no_scene_data = !has_scene_data(fs, alloc.arena, scene_comp.player_id);
    const now_ms = (Io.Clock.awake).now(io).toMilliseconds();

    if (cur_scene.*) |*active_scene| {
        try DirtySaveQueue.saveAllBuffs(
            alloc.gpa,
            fs,
            active_scene,
            buff_timers,
            assets,
            now_ms,
        );
    }

    try dirty_saves.flush(
        alloc.gpa,
        fs,
        role_comp,
        weapon_comp,
        if (cur_scene.*) |*active_scene| active_scene else null,
        buff_timers,
        assets,
        now_ms,
    );

    if (cur_scene.*) |*scene| {
        try scene.savePersistent(fs, alloc.gpa);
        scene.deinit(alloc.gpa, fs);
        cur_scene.* = null;
    }
    buff_timers.reset(alloc.gpa);

    const requested_inst_id = event.data.target_instance_id orelse scene_comp.last_scene_info.instance_id;
    const instance_dungeon = assets.tables.instance_dungeon.getDataById(requested_inst_id) orelse blk: {
        log.err(
            "player({d}) last scene instance id {d} doesn't exist; falling back to default instance 8",
            .{ player_id.id, requested_inst_id },
        );
        scene_comp.last_scene_info.instance_id = 8;
        break :blk assets.tables.instance_dungeon.getDataById(8) orelse return;
    };
    const sync_last_scene = scene_comp.last_scene_info.instance_id != instance_dungeon.Id;
    if (sync_last_scene) {
        scene_comp.last_scene_info.instance_id = instance_dungeon.Id;
    }

    var scene = try Scene.init(alloc.gpa, fs, player_id.id, instance_dungeon.Id);
    errdefer scene.deinit(alloc.gpa, fs);
    if (sync_last_scene) {
        try scene.saveLastScene(scene_comp, fs, alloc.gpa);
    }

    if (scene.instance.players.len == 0) {
        // A new scene. Player should be spawned.
        // TODO: add a check that actually searches for a player with matching ID. However this is only useful for MP, I guess?

        // Official Gorges of Spirits Prologue Spawn for Map 8 (matches 139000025_23 ServerSetPlayerPos)
        const default_born_loc: [3]f32 = if (instance_dungeon.Id == 8)
            .{ -139333.3, 66243.8, 7550.0 }
        else
            .{
                @floatFromInt(instance_dungeon.BornPosition[0]),
                @floatFromInt(instance_dungeon.BornPosition[1]),
                @floatFromInt(instance_dungeon.BornPosition[2]),
            };
        const default_born_rot: [3]f32 = if (instance_dungeon.Id == 8)
            .{ 0.0, 0.0, 30.0 }
        else
            .{
                @floatFromInt(instance_dungeon.BornRotation[0]),
                @floatFromInt(instance_dungeon.BornRotation[1]),
                @floatFromInt(instance_dungeon.BornRotation[2]),
            };

        scene.instance.players = try alloc.gpa.alloc(SceneInstance.Player, 1);
        scene.instance.players[0] = .{
            .id = player_id.id,
            .location = event.data.location orelse default_born_loc,
            .rotation = event.data.rotation orelse default_born_rot,
        };
    } else if (event.data.location) |location| {
        scene.instance.players[0] = .{
            .id = player_id.id,
            .location = location,
            .rotation = event.data.rotation orelse .{
                @floatFromInt(instance_dungeon.BornRotation[0]),
                @floatFromInt(instance_dungeon.BornRotation[1]),
                @floatFromInt(instance_dungeon.BornRotation[2]),
            },
        };
    }

    if (no_scene_data) {
        // The first Scene. Basic data should be initialized.

        scene.explore_tools_info.unlocked_explore_skills = blk: {
            var list: std.ArrayList(i32) = .empty;
            for (assets.tables.explore_tools.items) |tool| {
                // Exclude motorcycle skills (6001..6020) until unlocked.
                if (tool.PhantomSkillId >= 6000 and tool.PhantomSkillId < 7000) continue;
                if (tool.Authorization.map.entries.len == 0 or tool.PhantomSkillId == 1015) {
                    try list.append(alloc.gpa, tool.PhantomSkillId);
                }
            }
            break :blk try list.toOwnedSlice(alloc.gpa);
        };

        scene.explore_tools_info.roulette = blk: {
            const default_roulette: []const i32 = &.{ 1001, 1003, 1004, 1007 };
            const additional_roulette: []const i32 = &.{1009};

            var list: std.ArrayList(i32) = .empty;
            for (default_roulette) |skill_id| {
                try list.append(alloc.gpa, skill_id);
            }
            for (assets.tables.explore_tools.items) |tool| {
                if (std.mem.indexOfScalar(i32, additional_roulette, tool.PhantomSkillId) != null) {
                    try list.append(alloc.gpa, tool.PhantomSkillId);
                }
            }
            while (list.items.len < roulette_slot_count) {
                try list.append(alloc.gpa, 0);
            }
            break :blk try list.toOwnedSlice(alloc.gpa);
        };
        scene.explore_tools_info.active_explore_skill = 1001;
        scene.explore_tools_info.active_function_skill = 0;
        scene.explore_tools_info.motorcycle_roulette = try alloc.gpa.dupe(i32, &[_]i32{ 0, 0, 0, 0, 0, 0, 0, 0 });
        scene.explore_tools_info.active_motorcycle_skill = 0;
    }

    if (std.mem.indexOfScalar(i32, scene.explore_tools_info.unlocked_explore_skills, 1015) == null) {
        const old = scene.explore_tools_info.unlocked_explore_skills;
        const next = try alloc.gpa.alloc(i32, old.len + 1);
        @memcpy(next[0..old.len], old);
        next[old.len] = 1015;
        alloc.gpa.free(old);
        scene.explore_tools_info.unlocked_explore_skills = next;
    }

    try TrialRoles.materializeActiveTrialRoles(
        alloc.gpa,
        assets,
        role_comp,
        weapon_comp,
        echo_comp,
        quest_comp.info.env.trial_roles,
    );
    TrialRoles.sanitizeFormationForLogin(&scene.formation_info, role_comp, basic_comp, quest_comp.info.env.trial_roles);

    ensureValidFormation(&scene.formation_info, basic_comp);

    const new_scene_now_ms = (Io.Clock.awake).now(io).toMilliseconds();
    try buff_timers.ensureAllRegistered(alloc.gpa, &scene, assets, new_scene_now_ms);
    buff_timers.syncAllLeftDurations(&scene, new_scene_now_ms);
    try scene.save(fs, alloc.gpa);
    try events.enqueue(.scene_switch, .{
        .pending_flow = event.data.pending_flow,
    });
    cur_scene.* = scene;
}

pub fn notifyJoinScene(
    event: EventQueue.Dequeue(.scene_switch),
    events: *EventQueue,
    fs: *FileSystem,
    assets: *const Assets,
    conn: *Connection,
    alloc: mem.Alloc,
    player_id: PlayerID,
    scene_comp: *PlayerSceneComponent,
    basic_comp: *PlayerBasicComponent, // we'll have to be able to query for component of each individual player if we're going to implement MP
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    motor_comp: *PlayerMotorComponent,
    echo_comp: *PlayerEchoComponent,
    quest_comp: *const PlayerQuestComponent,
    progress_comp: *PlayerProgressComponent,
    scene: *Scene,
    buff_timers: *BuffTimerScheduler,
    io: Io,
) !void {
    const log = std.log.scoped(.scene_join);
    try exploreSkillNotify(alloc, scene, conn, io);
    var scene_info: pb.SceneInformation = .{
        .InstanceId = scene.instance_id,
        .OwnerId = scene.instance.owner_id,
        .CurContextId = player_id.id,
        .Mode = .Single,
        .SceneBulletOwnerId = 0,
    };

    const env_info = quest_comp.info.env;
    scene_info.TimeInfo = .{
        .Hour = if (env_info.time_locked) env_info.time_lock_hour else scene.instance.map_time.hour,
        .Minute = if (env_info.time_locked) env_info.time_lock_minute else scene.instance.map_time.minute,
        .OwnerTimeClockTimeSpan = scene.instance.map_time.owner_clock_time_span,
    };

    const instance_dungeon = assets.tables.instance_dungeon.getDataById(scene_comp.last_scene_info.instance_id) orelse blk: {
        log.err(
            "player({d}) last scene instance id {d} doesn't exist; falling back to default instance 8",
            .{ player_id.id, scene_comp.last_scene_info.instance_id },
        );
        scene_comp.last_scene_info.instance_id = 8;
        break :blk assets.tables.instance_dungeon.getDataById(8) orelse return;
    };
    const join_map_id = assets.resolveMapConfigId(instance_dungeon.Id);
    for (assets.tables.data_layer_config.items) |dl_cfg| {
        if (dl_cfg.InitLoad and
            dl_cfg.DataLayer.len > 0 and
            (dl_cfg.LevelId == instance_dungeon.Id or dl_cfg.LevelId == join_map_id) and
            !containsI32(quest_comp.info.world.unloaded_datalayers, dl_cfg.Id))
        {
            try appendUniqueI32(&scene_info.DataLayers, alloc.arena, dl_cfg.Id);
        }
    }
    for (quest_comp.info.world.loaded_datalayers) |dl| {
        if (dl > 0 and !containsI32(quest_comp.info.world.unloaded_datalayers, dl)) {
            const dl_cfg = assets.tables.data_layer_config.getDataById(dl) orelse continue;
            if (dl_cfg.DataLayer.len == 0) continue;
            if (dl_cfg.LevelId != 0 and dl_cfg.LevelId != instance_dungeon.Id and dl_cfg.LevelId != join_map_id) continue;
            try appendUniqueI32(&scene_info.DataLayers, alloc.arena, dl);
        }
    }
    if (instance_dungeon.Id == lahai_roi_dungeon_id) {
        try appendCompletedRoadDataLayers(&scene_info, assets, alloc.arena);
    }
    for (quest_comp.info.world.loaded_sublevels) |sub| {
        try scene_info.LoadedSubLevels.append(alloc.arena, sub);
    }
    for (quest_comp.info.world.hidden_sublevels) |sub| {
        try scene_info.HideSubLevels.append(alloc.arena, sub);
    }
    for (quest_comp.info.env.area_states) |area| {
        try scene_info.AreaStates.append(alloc.arena, .{
            .AreaId = area.area_id,
            .State = area.state,
        });
    }

    for (scene.instance.players) |scene_player| {
        if (scene_player.id == player_id.id) {
            var fight_role_groups: std.ArrayList(pb.FightRoleInfos) = .empty;

            const formation = &scene.formation_info.formations[@intCast(scene.formation_info.cur_formation)];
            var infos: pb.FightRoleInfos = .{
                .GroupType = scene_player.group_type,
                .CurRole = formation.cur_role,
                .LivingStatus = .Alive,
            };

            const motorcycle_entity = try PlayerEntityTemplates.createMotorcycleEntity(
                fs,
                scene,
                alloc,
                player_id.id,
                assets,
                motor_comp,
            );
            const player_scene_entity = try PlayerEntityTemplates.createPlayerSceneEntity(fs, scene, alloc, player_id.id, assets, motorcycle_entity.net_id);
            _ = try PlayerEntityTemplates.ensureMotorcycleCompanionEntity(
                fs,
                scene,
                alloc,
                player_id.id,
                assets,
                motorcycle_entity,
                player_scene_entity,
            );
            const scene_battle_entity = try PlayerEntityTemplates.createSceneBattleEntity(fs, scene, alloc, player_id.id, assets);
            scene_info.SceneBulletOwnerId = scene_battle_entity.net_id;

            for (&formation.roles) |*maybe_role| if (maybe_role.*) |*role| {
                const entity = try RoleEntityTemplates.createRoleEntity(
                    fs,
                    scene,
                    alloc,
                    player_id.id,
                    assets,
                    role_comp,
                    weapon_comp,
                    echo_comp,
                    instance_dungeon,
                    role.role_id,
                );
                role.entity_id = entity.net_id;
                try infos.FightRoleInfos.append(alloc.arena, .{
                    .RoleId = role.role_id,
                    .EntityId = entity.net_id,
                    .OnStageWithoutControl = role.on_stage_without_control,
                });
            };
            _ = RoleHelper.ensureAliveCurRole(scene, formation);
            infos.CurRole = formation.cur_role;
            infos.LivingStatus = RoleHelper.formationLivingStatus(scene, formation.*);

            try fight_role_groups.append(alloc.arena, infos);

            try scene_info.PlayerInfos.append(alloc.arena, .{
                .PlayerId = player_id.id,
                .PlayerName = basic_comp.info.attributes.name,
                .Level = basic_comp.info.attributes.level,
                .GroupType = scene_player.group_type,
                .Location = .{ // maybe it should be extracted from the active entity instead of storing it? Not quite sure.
                    .X = scene_player.location[0],
                    .Y = scene_player.location[1],
                    .Z = scene_player.location[2],
                },
                .Rotation = .{
                    .Roll = scene_player.rotation[0],
                    .Pitch = scene_player.rotation[1],
                    .Yaw = scene_player.rotation[2],
                },
                .FightRoleInfos = fight_role_groups,
                .CurRole = formation.cur_role,
                .VehiclePlayerData = .{},
                .PlayerPrefix = 1,
                .PlayerGEIncHandle = 0,
                .Gravity = .{},
            });

            break;
        }
    } else {
        // Shouldn't happen unless scene instance file is corrupted. Maybe should log it as well?
        return error.PlayerNotFoundInScene;
    }
    const now_ms = (Io.Clock.awake).now(io).toMilliseconds();
    try buff_timers.ensureAllRegistered(alloc.gpa, scene, assets, now_ms);
    buff_timers.syncAllLeftDurations(scene, now_ms);
    const fsm_clock: Io.Clock = .awake;
    try scene.initFsmRuntimes(alloc.gpa, fsm_clock.now(io).toMilliseconds());
    try scene.save(fs, alloc.gpa);

    var active_quest_targets = try QuestEngine.collectActiveQuestTargetIds(quest_comp.info, io, assets, alloc.gpa);
    defer active_quest_targets.deinit(alloc.gpa);

    var aoi: pb.PlayerSceneAoiData = .{};
    var stale_net_ids: std.ArrayList(i64) = .empty;

    const entities = scene.entities.slice();
    for (0..entities.len) |index| {
        var storage = entities.get(index);
        const config_id = storage.config.config_id;
        if (storage.config.entity_type != .player and storage.config.entity_type != .player_entity) {
            _ = try applyStoredWorldEntityOverrides(
                scene,
                assets,
                fs,
                alloc,
                progress_comp,
                config_id,
                storage.entity_id.net_id,
            );
            storage = entities.get(index);
        }
        var force_hidden_in_aoi = false;
        if (storage.config.entity_type != .player and
            storage.config.entity_type != .player_entity)
        {
            const cfg_id = storage.config.config_id;
            const is_quest_target = QuestEngine.isTargetIdInSet(&active_quest_targets, cfg_id);
            const explicitly_shown = cfg_id > 0 and QuestInfo.containsInt(quest_comp.info.world.shown_entities, cfg_id);
            var should_remove = QuestEngine.shouldHideWorldEntity(quest_comp.info, &active_quest_targets, cfg_id);
            if (!should_remove) {
                if (assets.findLevelEntity(scene.instance_id, cfg_id)) |lcfg| {
                    if (!QuestEngine.entityDataLayersActive(assets, quest_comp.info, scene.instance_id, lcfg)) {
                        should_remove = true;
                    }
                }
            }
            if (!should_remove and cfg_id > 0 and !is_quest_target) {
                if (quest_comp.info.hasOpenedChest(cfg_id) or scene.instance.isOnRespawnCooldown(cfg_id, now_ms)) {
                    should_remove = true;
                } else if (!explicitly_shown) {
                    if (assets.findLevelEntity(scene.instance_id, cfg_id)) |lcfg| {
                        // Send InSleep/IsHidden entities in AOI disabled rather than dropping them from AOI,
                        // so later AwakeEntity/SetEntityVisible can enable the existing client entity.
                        if (lcfg.InSleep or lcfg.IsHidden) {
                            force_hidden_in_aoi = true;
                        }
                    }
                }
            }
            if (should_remove) {
                try stale_net_ids.append(alloc.arena, storage.entity_id.net_id);
                continue;
            }
        }
        var entity = try storage.entityToProto(storage.entity_id.net_id, alloc, assets);
        if (force_hidden_in_aoi) {
            entity.IsVisible = false;
            entity.IsActorVisible = false;
        }
        try aoi.Entities.append(alloc.arena, entity);
    }

    for (stale_net_ids.items) |nid| {
        scene.remove(alloc.gpa, fs, nid) catch |err| {
            std.log.scoped(.scene).warn("failed to remove stale net_id {d}: {t}", .{ nid, err });
        };
    }

    scene_info.AoiData = aoi;

    try conn.push(alloc.gpa, io, pb.JoinSceneNotify{
        .MaxEntityId = 0,
        .SceneInfo = scene_info,
        .TransitionOption = .{},
    });
    scene.join_scene_pushed = true;

    try events.enqueue(.after_scene_join, .{
        .pending_flow = event.data.pending_flow,
    });
}

pub fn formationUpdateNotify(
    _: EventQueue.Dequeue(.update_formations),
    conn: *Connection,
    scene: *Scene,
    query: Scene.Query(&.{
        Entity,
        *Entity.AttributeComponent,
    }),
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    const log = std.log.scoped(.update_formations);

    var update_formation_notify: pb.UpdateFormationNotify = .default;
    const main_scene_player = scene.instance.players[0];
    var formations: std.ArrayList(pb.FightFormationNotifyInfo) = .empty;

    for (scene.formation_info.formations, 1..) |formation, i| {
        const has_any_role = for (formation.roles) |maybe_role| {
            if (maybe_role != null) break true;
        } else false;
        if (!has_any_role) continue;
        const id: i32 = @intCast(i);
        var role_infos: std.ArrayList(pb.FormationRoleInfo) = .empty;

        for (formation.roles) |maybe_role| {
            const role = maybe_role orelse continue;

            const role_info = role_comp.role_map.get(role.role_id) orelse {
                log.warn("role {} not found in role_map, skipping", .{role.role_id});
                continue;
            };
            const weapon_info = weapon_comp.weapon_map.get(role_info.weapon) orelse {
                log.warn("weapon {} not found for formation role {}, skipping", .{ role_info.weapon, role.role_id });
                continue;
            };
            var dress_list: std.ArrayList(i32) = .empty;
            defer dress_list.deinit(alloc.arena);
            for (role_info.ornaments) |ornament| {
                if (ornament.role_skin_id != role_info.role_skin_id) continue;
                try dress_list.append(alloc.arena, ornament.ornament_id);
            }

            if (role.entity_id != -1) {
                if (query.byNetId(role.entity_id)) |comps| {
                    const attribute_comp = comps[1];
                    try role_infos.append(alloc.arena, pb.FormationRoleInfo{
                        .roleId = role.role_id,
                        .MaxHp = attribute_comp.attributes[@intFromEnum(pb.EAttributeType.LifeMax)].current,
                        .CurHp = attribute_comp.attributes[@intFromEnum(pb.EAttributeType.Life)].current,
                        .Level = attribute_comp.attributes[@intFromEnum(pb.EAttributeType.Lv)].current,
                        .SkillBranchId = role_info.skill_branch,
                        .RoleSkinId = role_info.role_skin_id,
                        .WeaponBreachLevel = weapon_info.breach,
                        .WeaponId = weapon_info.id,
                        .WeaponSkinId = role_info.weapon_skin_id,
                        .DressList = dress_list,
                    });
                    continue;
                }
            }

            try role_infos.append(alloc.arena, pb.FormationRoleInfo{
                .roleId = role.role_id,
                .MaxHp = role_info.base_prop[@intFromEnum(pb.EAttributeType.LifeMax)],
                .CurHp = role_info.base_prop[@intFromEnum(pb.EAttributeType.Life)],
                .Level = role_info.level,
                .SkillBranchId = role_info.skill_branch,
                .RoleSkinId = role_info.role_skin_id,
                .WeaponBreachLevel = weapon_info.breach,
                .WeaponId = weapon_info.id,
                .WeaponSkinId = role_info.weapon_skin_id,
                .DressList = dress_list,
            });
        }

        try formations.append(alloc.arena, pb.FightFormationNotifyInfo{
            .FormationId = id,
            .Name = formation.name orelse "",
            .CurRole = formation.cur_role,
            .RoleInfos = role_infos,
            .IsCurrent = scene.formation_info.cur_formation == i - 1,
        });
    }

    try update_formation_notify.PlayersFormations.append(alloc.arena, pb.PlayerFightFormations{
        .Formations = formations,
        .PlayerId = main_scene_player.id,
    });

    for (update_formation_notify.PlayersFormations.items) |pf| {
        log.debug("Player {} formations:", .{pf.PlayerId});
        for (pf.Formations.items) |f| {
            log.debug(
                "  Formation {} cur={} roles={any}, is_current {}",
                .{ f.FormationId, f.CurRole, f.RoleInfos.items, f.IsCurrent },
            );
        }
    }

    try conn.push(alloc.gpa, io, update_formation_notify);
}

pub fn afterSceneJoin(
    event: EventQueue.Dequeue(.after_scene_join),
    events: *EventQueue,
    scene: *Scene,
    conn: *Connection,
    fs: *FileSystem,
    assets: *const Assets,
    basic_comp: *const PlayerBasicComponent,
    qctx: QuestEngine.QuestContext,
    io: Io,
    player_id: PlayerID,
    alloc: mem.Alloc,
) !void {
    _ = basic_comp;
    try conn.push(alloc.gpa, io, pb.AfterJoinSceneNotify{});
    if (qctx.progress_comp) |progress| {
        const entities = scene.entities.slice();
        for (entities.items(.entity_id), entities.items(.config)) |entity, config| {
            if (config.entity_type != .scene_item) continue;
            pushStoredElevatorState(conn, alloc, io, progress.info, config.config_id, entity.net_id);
        }
    }
    try conn.push(alloc.gpa, io, pb.SwitchBattleModeNotify{});
    try notifyTransportRoadways(fs, alloc, scene, conn, assets, io);
    try notifyInfrastructureRoadData(alloc, conn, assets, io);
    try QuestEngine.syncWorldStateOnJoin(qctx);
    try QuestEngine.pushReadyQuestList(qctx);

    const rtc: Io.Clock = .real;
    const monotonic_clock: Io.Clock = .awake;
    const now_ms = rtc.now(io).toMilliseconds();
    const monotonic_now = monotonic_clock.now(io).toMilliseconds();
    scene.scene_time.reset(now_ms, monotonic_now);
    try scene.initFsmRuntimes(alloc.gpa, monotonic_now);
    const movement_entities = scene.entities.slice();
    for (
        movement_entities.items(.entity_id),
        movement_entities.items(.world_item),
    ) |entity, world_item| {
        if (world_item.move_to_point) |move| {
            conn.pushBestEffort(alloc.gpa, io, pb.SceneItemMoveTargetNotify{
                .MoveInfo = .{
                    .EntityId = entity.net_id,
                    .Location = .{
                        .X = move.target_pos[0],
                        .Y = move.target_pos[1],
                        .Z = move.target_pos[2],
                    },
                    .MoveTime = move.move_time,
                    .StayTime = move.stay_time,
                    .MaxSpeed = move.max_speed,
                    .Acceleration = move.acceleration,
                },
            });
        }
    }
    try conn.push(alloc.gpa, io, pb.TimeCheckNotify{
        .ClientTime = 0,
        .ServerTime = now_ms,
        .ServerCombatTime = now_ms,
        .ServerStopTime = now_ms,
        .ServerFlowTimestamp = scene.scene_time.currentFlowTimestamp(monotonic_now),
    });
    try events.enqueue(.update_formations, .{});

    var formation_attrs: std.ArrayList(pb.FormationAttr) = .empty;
    try formation_attrs.appendSlice(alloc.arena, &.{
        .{ .AttrId = 1, .Ratio = 2400, .BaseMaxValue = 24000, .MaxValue = 24000, .CurrentValue = 22000 },
        .{ .AttrId = 10, .Ratio = 2400, .BaseMaxValue = 7000, .MaxValue = 15000, .CurrentValue = 7000 },
        .{ .AttrId = 14, .Ratio = 0, .BaseMaxValue = 10000, .MaxValue = 10000, .CurrentValue = 10000 },
    });
    const formation_attr_notify: pb.FormationAttrNotify = .{
        .Duration = 1534854458,
        .FormationAttrs = formation_attrs,
    };
    try conn.push(alloc.gpa, io, formation_attr_notify);

    const no_uid_watermark = try Io.Dir.readFileAlloc(Io.Dir.cwd(), fs.io, "assets/scripts/join_scene_patches/uid_watermark.js", alloc.gpa, Io.Limit.unlimited);
    defer alloc.gpa.free(no_uid_watermark);

    const uid_str = try std.fmt.allocPrint(alloc.gpa, "{d}", .{player_id.id});
    defer alloc.gpa.free(uid_str);

    const watermark_js = try std.mem.replaceOwned(u8, alloc.gpa, no_uid_watermark, "{PLR_UID}", uid_str);
    defer alloc.gpa.free(watermark_js);
    try conn.push(alloc.gpa, io, pb.JSPatchNotify{ .Content = watermark_js });

    const patch_files = [_][]const u8{
        "assets/scripts/join_scene_patches/main_watermask_disable.js",
        "assets/scripts/join_scene_patches/flight_fix.js",
        "assets/scripts/join_scene_patches/motorcycle.js",
        "assets/scripts/join_scene_patches/global_spawn.js",
        "assets/scripts/join_scene_patches/lily.js",
    };

    for (patch_files) |path| {
        const content = try Io.Dir.readFileAlloc(Io.Dir.cwd(), fs.io, path, alloc.gpa, Io.Limit.unlimited);
        defer alloc.gpa.free(content);
        try conn.push(alloc.gpa, io, pb.JSPatchNotify{ .Content = content });
    }

    if (scene.instance.players.len > 0) {
        scene.last_spatial_check_loc = scene.instance.players[0].location;
    }

    // shitty solution for a shitty problem...
    if (event.data.pending_flow) |flow| {
        try conn.push(alloc.gpa, io, pb.FlowStartNotify{
            .FlowIncId = flow.inc_id,
            .FlowListName = flow.namespace,
            .FlowId = flow.id,
            .StateId = flow.state,
        });
    }
}

fn inRange2D(loc: [3]f32, x: f32, y: f32, radius: f32) bool {
    const dx = loc[0] - x;
    const dy = loc[1] - y;
    return dx * dx + dy * dy <= radius * radius;
}

pub fn handleSpatialSpawnsCheck(
    event: EventQueue.Dequeue(.spatial_spawns_check),
    events: *EventQueue,
    scene: *Scene,
    fs: *FileSystem,
    assets: *const Assets,
    conn: *Connection,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    map_comp: *PlayerMapComponent,
    quest_comp: *PlayerQuestComponent,
    progress_comp: *PlayerProgressComponent,
    role_comp: *PlayerRoleComponent,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    const loc = event.data.location;

    const unlocked_new_tp = try map_comp.info.ensureDefaultUnlocks(alloc.gpa);
    if (unlocked_new_tp) {
        try map_comp.save(fs, alloc.arena);
        var tp_ids: std.ArrayList(i32) = .empty;
        for (map_comp.info.unlocked_teleporters) |id| {
            try tp_ids.append(alloc.arena, id);
        }
        conn.pushBestEffort(alloc.gpa, io, pb.TeleportUpdateNotify{ .Ids = tp_ids });
    }

    try LevelPlayHelper.onPlayerMove(.{
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
        .progress_comp = progress_comp,
    }, loc[0], loc[1], loc[2]);

    try spawnNearbyWorldEntities(
        events,
        scene,
        fs,
        assets,
        conn,
        basic_comp,
        map_comp,
        quest_comp,
        progress_comp,
        alloc,
        io,
        loc,
        20000.0,
        35,
        (Io.Clock.awake).now(io).toMilliseconds(),
    );
}

fn evictDistantAmbientEntities(
    scene: *Scene,
    fs: *FileSystem,
    assets: *const Assets,
    conn: *Connection,
    quest_info: QuestInfo,
    active_quest_targets: *const std.AutoHashMapUnmanaged(i32, void),
    alloc: mem.Alloc,
    io: std.Io,
    player_loc: [3]f32,
    evict_radius: f32,
    skip_priority: bool,
) void {
    const evict_r2 = evict_radius * evict_radius;
    var victim_ids: std.ArrayList(i64) = .empty;
    var remove_infos: std.ArrayList(pb.EntityRemoveInfo) = .empty;

    const slice = scene.entities.slice();
    const net_ids = slice.items(.entity_id);
    const configs = slice.items(.config);
    const positions = slice.items(.position);
    const player_ids = slice.items(.player_id);
    const followers = slice.items(.follower);
    const vehicles = slice.items(.vehicle);
    const summoners = slice.items(.summoner);
    const concomitants = slice.items(.concomitant);

    for (net_ids, configs, positions, player_ids, followers, vehicles, summoners, concomitants) |nid, cfg, pos_opt, pid_opt, flw_opt, veh_opt, sum_opt, con_opt| {
        if (pid_opt != null or flw_opt != null or veh_opt != null or sum_opt != null or con_opt != null) continue;
        switch (cfg.entity_type) {
            .monster, .animal, .npc, .scene_item, .vision => {},
            else => continue,
        }
        if (scene.active_battle_entities.contains(nid.net_id)) continue;
        if (QuestEngine.isTargetIdInSet(active_quest_targets, cfg.config_id)) continue;
        if (cfg.config_id > 0) {
            if (QuestInfo.containsInt(quest_info.world.shown_entities, cfg.config_id)) continue;
            if (quest_info.getEntityState(cfg.config_id) != null) continue;
            var keep_lp = false;
            for (assets.indexes.levelPlayIndicesForRefEntity(cfg.config_id)) |lp_idx| {
                const lpid = assets.tables.level_play_data.items[lp_idx].LevelPlayId;
                if (QuestInfo.containsInt(quest_info.progress.entered_level_plays, lpid)) {
                    keep_lp = true;
                    break;
                }
            }
            if (keep_lp) continue;
        }
        if (skip_priority and cfg.config_id > 0) {
            if (assets.findLevelEntity(scene.instance_id, cfg.config_id)) |raw_cfg| {
                if (isPriorityWorldEntity(assets, raw_cfg)) continue;
            }
        }

        const pos = pos_opt orelse continue;
        const dx = pos.location[0] - player_loc[0];
        const dy = pos.location[1] - player_loc[1];
        const dz = pos.location[2] - player_loc[2];
        if (dx * dx + dy * dy + dz * dz <= evict_r2) continue;

        victim_ids.append(alloc.arena, nid.net_id) catch continue;
    }

    for (victim_ids.items) |net_id| {
        scene.remove(alloc.gpa, fs, net_id) catch continue;
        remove_infos.append(alloc.arena, .{
            .EntityId = net_id,
            .Type = .RemoveTypeNormal,
        }) catch |err| {
            std.log.scoped(.scene).warn("failed to append entity removal {d}: {t}", .{ net_id, err });
            break;
        };
    }

    if (remove_infos.items.len > 0) {
        conn.pushBestEffort(alloc.gpa, io, pb.EntityRemoveNotify{
            .RemoveInfos = remove_infos,
            .IsRemove = true,
        });
    }
}

const SpawnCandidate = struct {
    cfg_idx: usize,
    x: f32,
    y: f32,
    z: f32,
    dist2: f32,
};

fn isPriorityWorldEntity(assets: *const Assets, cfg: *const Assets.DataTables.LevelEntityConfig) bool {
    if (std.mem.indexOf(u8, cfg.BlueprintType, "Treasure") != null) return true;
    if (std.mem.indexOf(u8, cfg.BlueprintType, "Teleport") != null) return true;
    if (assets.indexes.findBlueprintConfig(&assets.tables, cfg.BlueprintType)) |bp| {
        if (bp.EntityLogic == .Vehicle) return true;
        switch (bp.EntityType) {
            .AirWallEffectTrigger,
            .AirPassage,
            .ChallengeInteract,
            .DungeonEntry,
            .DungeonEntryWithTrigger,
            .ExploreSkillGroupInteractor,
            .ExploreSkillInteractor,
            .GameplayInteractGear,
            .GravityFlip,
            .HookLockPoint,
            .HookTreasureBox,
            .HookWithRange,
            .InteractGear,
            .InteractGear2,
            .InteractGearCanHit,
            .InteractGearTrigger,
            .KiteHook,
            .CableWayHook,
            .Lift,
            .Lift2,
            .SequenceLift,
            .Portal,
            .NoRenderPortal,
            .NoRenderPortalWithInteract,
            .MoveNoRenderPortal,
            .SceneAura,
            .SimpleInteract,
            .StateSceneItem,
            .TargetGear,
            .TargetGear2,
            .TeleControl,
            .Teleporter,
            .TreasureBox,
            .VehicleSceneItem,
            .VisionTreasureBox,
            .WaterSpout,
            => return true,
            else => {},
        }
    }
    return false;
}

fn insertSpawnCandidate(buf: []SpawnCandidate, len: *usize, cap: usize, cand: SpawnCandidate) void {
    if (cap == 0) return;
    if (len.* < cap) {
        buf[len.*] = cand;
        len.* += 1;
        var j = len.* - 1;
        while (j > 0 and buf[j].dist2 < buf[j - 1].dist2) : (j -= 1) {
            std.mem.swap(SpawnCandidate, &buf[j], &buf[j - 1]);
        }
    } else if (cand.dist2 < buf[cap - 1].dist2) {
        buf[cap - 1] = cand;
        var j = cap - 1;
        while (j > 0 and buf[j].dist2 < buf[j - 1].dist2) : (j -= 1) {
            std.mem.swap(SpawnCandidate, &buf[j], &buf[j - 1]);
        }
    }
}

fn considerSpawnCandidates(
    assets: *const Assets,
    scene: *const Scene,
    quest_info: QuestInfo,
    active_quest_targets: *const std.AutoHashMapUnmanaged(i32, void),
    now_ms: i64,
    player_loc: [3]f32,
    r2: f32,
    indices: []const u32,
    priority_buf: *[48]SpawnCandidate,
    priority_len: *usize,
    best_buf: *[48]SpawnCandidate,
    best_len: *usize,
    cap: usize,
) void {
    for (indices) |cfg_idx| {
        const cfg = (assets.getLevelEntityByIdx(cfg_idx) orelse continue).*;
        if (!QuestEngine.entityDataLayersActive(assets, quest_info, scene.instance_id, &cfg)) continue;
        const pos = Assets.AssetIndexes.entityWorldPos(&cfg) orelse continue;

        const dx = pos[0] - player_loc[0];
        const dy = pos[1] - player_loc[1];
        const dz = pos[2] - player_loc[2];
        const dist2 = dx * dx + dy * dy + dz * dz;
        if (dist2 > r2) continue;
        if (cfg.ComponentsData.BaseInfoComponent) |bi| {
            if (bi.AoiZRadius) |az| {
                if (az > 0 and @abs(dz) > @as(f32, @floatFromInt(az))) continue;
            }
        }

        const is_quest_target = QuestEngine.isTargetIdInSet(active_quest_targets, cfg.EntityId);
        const is_priority = is_quest_target or isPriorityWorldEntity(assets, &cfg);
        if (is_priority) {
            if (priority_len.* >= priority_buf.len and dist2 >= priority_buf[priority_buf.len - 1].dist2) continue;
        } else {
            if (cap > 0 and best_len.* >= cap and dist2 >= best_buf[cap - 1].dist2) continue;
        }
        if (scene.debugSpawnRoute(cfg.EntityId) != null) continue;
        const cfg_id32 = std.math.cast(i32, cfg.EntityId) orelse 0;
        if (cfg_id32 > 0) {
            if (scene.findEntityNetIdByConfigId(cfg_id32) != null) continue;
            if (!is_quest_target and quest_info.hasOpenedChest(cfg_id32)) continue;
            if (!is_quest_target and scene.instance.isOnRespawnCooldown(cfg_id32, now_ms)) continue;
        }
        const explicitly_shown = cfg_id32 > 0 and QuestInfo.containsInt(quest_info.world.shown_entities, cfg_id32);
        if ((cfg.InSleep or cfg.IsHidden) and !explicitly_shown and !is_quest_target) continue;
        if (QuestEngine.shouldHideWorldEntity(quest_info, active_quest_targets, cfg.EntityId)) continue;

        const cand: SpawnCandidate = .{
            .cfg_idx = cfg_idx,
            .x = pos[0],
            .y = pos[1],
            .z = pos[2],
            .dist2 = dist2,
        };
        if (is_priority) {
            insertSpawnCandidate(priority_buf, priority_len, priority_buf.len, cand);
        } else {
            insertSpawnCandidate(best_buf, best_len, cap, cand);
        }
    }
}

fn isUnlockedTeleporterEntity(
    assets: *const Assets,
    map_comp: *const PlayerMapComponent,
    map_id: i32,
    entity_config_id: i64,
) bool {
    if (entity_config_id <= 0) return false;
    for (assets.indexes.teleporterIndicesForMap(map_id)) |tp_idx| {
        const tp = assets.tables.teleporter.items[tp_idx];
        if (tp.TeleportEntityConfigId == entity_config_id and
            QuestInfo.containsInt(map_comp.info.unlocked_teleporters, tp.Id))
        {
            return true;
        }
    }
    return false;
}

fn spawnNearbyWorldEntities(
    events: *EventQueue,
    scene: *Scene,
    fs: *FileSystem,
    assets: *const Assets,
    conn: *Connection,
    basic_comp: *const PlayerBasicComponent,
    map_comp: *const PlayerMapComponent,
    quest_comp: *const PlayerQuestComponent,
    progress_comp: *PlayerProgressComponent,
    alloc: mem.Alloc,
    io: std.Io,
    player_loc: [3]f32,
    radius: f32,
    max_count: usize,
    now_ms: i64,
) !void {
    const need_evict = scene.entities.slice().len >= 200;
    if (!need_evict and scene.entities.slice().len >= 240) return;

    var active_quest_targets = try QuestEngine.collectActiveQuestTargetIds(quest_comp.info, io, assets, alloc.gpa);
    defer active_quest_targets.deinit(alloc.gpa);

    if (need_evict) {
        evictDistantAmbientEntities(scene, fs, assets, conn, quest_comp.info, &active_quest_targets, alloc, io, player_loc, 24000.0, true);
        if (scene.entities.slice().len >= 215) {
            evictDistantAmbientEntities(scene, fs, assets, conn, quest_comp.info, &active_quest_targets, alloc, io, player_loc, 21000.0, true);
        }
        if (scene.entities.slice().len >= 230) {
            evictDistantAmbientEntities(scene, fs, assets, conn, quest_comp.info, &active_quest_targets, alloc, io, player_loc, 32000.0, false);
        }
    }
    if (scene.entities.slice().len >= 240) return;

    const cap: usize = @min(max_count, 40);
    var priority_buf: [48]SpawnCandidate = undefined;
    var priority_len: usize = 0;
    var best_buf: [48]SpawnCandidate = undefined;
    var best_len: usize = 0;
    const r2 = radius * radius;

    const map_id = assets.resolveMapConfigId(scene.instance_id);

    if (Assets.AssetIndexes.spatialCellRange(player_loc[0], player_loc[1], radius)) |range| {
        var cx = range.min_x;
        while (cx <= range.max_x) : (cx += 1) {
            var cy = range.min_y;
            while (cy <= range.max_y) : (cy += 1) {
                const cell_indices = assets.indexes.spawnableIndicesForMapCell(map_id, cx, cy);
                if (cell_indices.len == 0) continue;
                considerSpawnCandidates(
                    assets,
                    scene,
                    quest_comp.info,
                    &active_quest_targets,
                    now_ms,
                    player_loc,
                    r2,
                    cell_indices,
                    &priority_buf,
                    &priority_len,
                    &best_buf,
                    &best_len,
                    cap,
                );
            }
        }
    } else {
        considerSpawnCandidates(
            assets,
            scene,
            quest_comp.info,
            &active_quest_targets,
            now_ms,
            player_loc,
            r2,
            assets.indexes.spawnableIndicesForMap(map_id),
            &priority_buf,
            &priority_len,
            &best_buf,
            &best_len,
            cap,
        );
    }

    const pools = [_][]const SpawnCandidate{
        priority_buf[0..priority_len],
        best_buf[0..best_len],
    };
    for (pools) |pool| {
        for (pool) |cand| {
            if (scene.entities.slice().len >= 240) break;
            const cfg = (assets.getLevelEntityByIdx(@intCast(cand.cfg_idx)) orelse continue).*;
            const saved_location = if (std.math.cast(i32, cfg.EntityId)) |config_id|
                storedWorldLocation(progress_comp.info, config_id)
            else
                null;
            spawn_cmd.spawnWithLocationAndProgress(
                events,
                scene,
                fs,
                assets,
                conn,
                basic_comp,
                alloc,
                io,
                cfg.EntityId,
                false,
                false,
                cfg.MapId,
                saved_location orelse .{ cand.x, cand.y, cand.z },
                true,
                progress_comp.info,
            ) catch continue;

            if (std.math.cast(i32, cfg.EntityId)) |cfg_id| {
                if (scene.findEntityNetIdByConfigId(cfg_id)) |net_id| {
                    const gravity_type = try applyStoredWorldEntityOverrides(
                        scene,
                        assets,
                        fs,
                        alloc,
                        progress_comp,
                        cfg_id,
                        net_id,
                    );
                    pushStoredWorldTransform(
                        scene,
                        conn,
                        alloc,
                        io,
                        progress_comp.info,
                        cfg_id,
                        net_id,
                    );
                    var lift_key_buf: [48]u8 = undefined;
                    const lift_key = std.fmt.bufPrint(&lift_key_buf, "lift_target:{d}", .{cfg_id}) catch "";
                    if (lift_key.len > 0) {
                        if (progressInt(progress_comp.info, lift_key)) |raw_location| {
                            if (std.math.cast(i32, raw_location)) |location| {
                                conn.pushBestEffort(alloc.gpa, io, pb.ElevatorMoveNotify{
                                    .EntityId = net_id,
                                    .Location = location,
                                });
                            }
                        }
                    }
                    pushStoredElevatorState(conn, alloc, io, progress_comp.info, cfg_id, net_id);
                    if (gravity_type) |flip_type| {
                        conn.pushBestEffort(alloc.gpa, io, pb.GravityFlipUpdateTypeNotify{
                            .NewGravityFlipType = flip_type,
                            .EntityId = cfg_id,
                        });
                    }
                    if (scene.net_id_map.get(net_id)) |index| {
                        const world_item = scene.entities.items(.world_item)[index];
                        if (world_item.move_spline) |move| {
                            conn.pushBestEffort(alloc.gpa, io, pb.MoveSplineStatusNotify{
                                .SceneItemRuntimeData = move.runtime_data,
                                .MoveStatus = move.move_status,
                                .EntityId = net_id,
                                .SplineEntityId = move.spline_entity_id,
                                .MoveSplineConfig = move.move_spline_config,
                            });
                        }
                        if (world_item.move_to_point) |move| {
                            conn.pushBestEffort(alloc.gpa, io, pb.SceneItemMoveTargetNotify{
                                .MoveInfo = .{
                                    .EntityId = net_id,
                                    .Location = .{
                                        .X = move.target_pos[0],
                                        .Y = move.target_pos[1],
                                        .Z = move.target_pos[2],
                                    },
                                    .MoveTime = move.move_time,
                                    .StayTime = move.stay_time,
                                    .MaxSpeed = move.max_speed,
                                    .Acceleration = move.acceleration,
                                },
                            });
                        }
                    }
                    try QuestEngine.syncPersistedVehicleNpcLinks(scene, progress_comp, conn, alloc, io, cfg_id);
                }
            }

            if (std.math.cast(i32, cfg.EntityId)) |cfg_id32| {
                const desired_state: ?[]const u8 = if (quest_comp.info.getEntityState(cfg.EntityId)) |saved|
                    if (saved.len != 0) saved else null
                else if (isUnlockedTeleporterEntity(assets, map_comp, map_id, cfg.EntityId))
                    "关卡.Common.状态.激活"
                else
                    null;

                if (desired_state) |state_name| {
                    if (scene.findEntityNetIdByConfigId(cfg_id32)) |nid| {
                        if (try scene.updateEntityStateTag(alloc.gpa, &assets.tables.gameplay_tag_remap, nid, state_name)) |upd| {
                            if (upd.old_tag_id != upd.new_tag_id) {
                                try QuestEngine.emitEntityStateTagUpdate(conn, alloc.gpa, alloc.arena, io, nid, upd);
                            }
                        }
                    }
                }
                try QuestEngine.syncDynamicInteractOptionsForEntity(scene, assets, conn, quest_comp.info, alloc, io, cfg.EntityId);
            }
        }
    }
}
