const std = @import("std");
const pb = @import("proto").pb;
const Transaction = @import("../handlers.zig").Transaction;
const Scene = @import("../../logic/Scene.zig");
const mem = @import("../../mem.zig");
const EntityComponentStorage = @import("../../logic/component/entity/EntityComponentStorage.zig");
const sliceToArrayList = EntityComponentStorage.sliceToArrayList;
const Entity = Scene.Entity;
const FileSystem = @import("common").FileSystem;
const Assets = @import("../../data/Assets.zig");
const PlayerEchoComponent = @import("../../logic/component/player/PlayerEchoComponent.zig");
const RoleEntityTemplates = @import("../../logic/templates/RoleEntityTemplates.zig");
const phantom_projector = @import("../../logic/helpers/phantom_projector.zig");
const entity_proto = @import("../../logic/helpers/entity_proto.zig");
const BuffTimerScheduler = @import("../../logic/schedulers/BuffTimerScheduler.zig");
const PlayerMapComponent = @import("../../logic/component/player/PlayerMapComponent.zig");
const QuestEngine = @import("../../logic/helpers/quest_engine.zig");
const LevelPlayHelper = @import("../../logic/helpers/level_play.zig");
const PlayerProgressComponent = @import("../../logic/component/player/PlayerProgressComponent.zig");
const buffListFromIds = @import("../../data/tables/Buff.zig").buffListFromIds;
const roulette_slot_count = 8;

fn ownedRouletteSkillIds(gpa: std.mem.Allocator, ids: []const i32) ![]i32 {
    const items = try gpa.alloc(i32, roulette_slot_count);
    @memset(items, 0);
    const count = @min(ids.len, roulette_slot_count);
    @memcpy(items[0..count], ids[0..count]);
    return items;
}

fn rouletteSkillIds(arena: std.mem.Allocator, ids: []const i32) !std.ArrayList(i32) {
    return sliceToArrayList(i32, try ownedRouletteSkillIds(arena, ids));
}

pub fn onEnterAreaRequest(
    txn: *Transaction(pb.EnterAreaRequest),
    map_comp: *PlayerMapComponent,
    fs: *FileSystem,
    alloc: mem.Alloc,
    qctx: QuestEngine.QuestContext,
) !void {
    if (txn.message.Id > 0) {
        if (try map_comp.info.unlockArea(alloc.gpa, txn.message.Id)) {
            try map_comp.save(fs, alloc.arena);
        }
        if (qctx.scene) |scene| {
            if (scene.instance.players.len > 0) {
                const loc = scene.instance.players[0].location;
                QuestEngine.onMove(qctx, loc) catch |err| QuestEngine.logHookErr("onMove", err);
                if (qctx.role_comp) |role_comp| {
                    LevelPlayHelper.onPlayerMove(.{
                        .assets = qctx.assets,
                        .fs = qctx.fs,
                        .conn = qctx.conn,
                        .scene = scene,
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
                    }, loc[0], loc[1]) catch |err| {
                        std.log.scoped(.level_play).warn("onPlayerMove failed: {t}", .{err});
                    };
                }
            }
        }
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .Id = txn.message.Id,
    });
}

pub fn onVisionExploreSkillSetRequest(
    txn: *Transaction(pb.VisionExploreSkillSetRequest),
    scene: *Scene,
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    buff_timers: *BuffTimerScheduler,
    qctx: QuestEngine.QuestContext,
    io: std.Io,
    query: Scene.Query(&.{
        Entity,
        *Entity.VisionSkillComponent,
    }),
) !void {
    const now_ms = (std.Io.Clock.awake).now(io).toMilliseconds();
    const old_skill = scene.explore_tools_info.active_explore_skill;
    scene.explore_tools_info.active_explore_skill = txn.message.SkillId;

    var it = query.iterator;
    while (it.next()) |comps| {
        const entity = comps[0];
        const vision_skill_comp = comps[1];

        for (vision_skill_comp.vision_skills) |*skill| {
            if (skill.SkillId == old_skill) {
                skill.SkillId = txn.message.SkillId;
            }
        }

        try buff_timers.ensureEntityRegistered(
            alloc.gpa,
            scene,
            assets,
            entity.net_id,
            now_ms,
        );
        buff_timers.syncEntityLeftDurations(scene, entity.net_id, now_ms);
        try scene.saveEntity(fs, alloc.gpa, entity);
        try txn.conn.push(alloc.gpa, io, pb.VisionSkillChangeNotify{
            .EntityId = entity.net_id,
            .VisionSkillInfos = sliceToArrayList(pb.VisionSkillInformation, vision_skill_comp.vision_skills),
        });
    }

    try scene.savePersistent(fs, alloc.gpa);
    if (txn.message.SkillId != 0) {
        QuestEngine.onVisionSystemEvent(qctx, "VisionSetActivated") catch |err| QuestEngine.logHookErr("onVisionSystemEvent", err);
    }
    txn.respond(.{ .SkillId = txn.message.SkillId });
}

pub fn onExploreSkillRouletteSetRequest(
    txn: *Transaction(pb.ExploreSkillRouletteSetRequest),
    scene: *Scene,
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    buff_timers: *BuffTimerScheduler,
    io: std.Io,
) !void {
    const now_ms = (std.Io.Clock.awake).now(io).toMilliseconds();
    const roulette_type = txn.message.RouletteType orelse .Explore;
    const selected_roulette: ?pb.ExploreSkillRoulette = txn.message.SkillRoulette orelse if (txn.message.SkillRoulettes.items.len != 0)
        txn.message.SkillRoulettes.items[0]
    else
        null;

    if (selected_roulette) |roulette| {
        switch (roulette_type) {
            .Explore => {
                if (scene.explore_tools_info.roulette.len != 0) alloc.gpa.free(scene.explore_tools_info.roulette);
                scene.explore_tools_info.roulette = try ownedRouletteSkillIds(alloc.gpa, roulette.SkillIds.items);
                scene.explore_tools_info.explore_extra_item_id = roulette.ExtraItemId;
                scene.explore_tools_info.active_explore_skill = roulette.ExploreSkill;
            },
            .Function => {
                if (scene.explore_tools_info.function_roulette.len != 0) alloc.gpa.free(scene.explore_tools_info.function_roulette);
                scene.explore_tools_info.function_roulette = try ownedRouletteSkillIds(alloc.gpa, roulette.SkillIds.items);
                scene.explore_tools_info.function_extra_item_id = roulette.ExtraItemId;
                scene.explore_tools_info.active_function_skill = roulette.ExploreSkill;
            },
            .Motorcycle => {
                if (scene.explore_tools_info.motorcycle_roulette.len != 0) alloc.gpa.free(scene.explore_tools_info.motorcycle_roulette);
                scene.explore_tools_info.motorcycle_roulette = try ownedRouletteSkillIds(alloc.gpa, roulette.SkillIds.items);
                scene.explore_tools_info.motorcycle_extra_item_id = roulette.ExtraItemId;
                scene.explore_tools_info.active_motorcycle_skill = roulette.ExploreSkill;
            },
            .TrapDefense => {
                if (scene.explore_tools_info.trap_defense_roulette.len != 0) alloc.gpa.free(scene.explore_tools_info.trap_defense_roulette);
                scene.explore_tools_info.trap_defense_roulette = try ownedRouletteSkillIds(alloc.gpa, roulette.SkillIds.items);
                scene.explore_tools_info.trap_defense_extra_item_id = roulette.ExtraItemId;
                scene.explore_tools_info.active_trap_defense_skill = roulette.ExploreSkill;
            },
        }
        try buff_timers.ensureAllRegistered(alloc.gpa, scene, assets, now_ms);
        buff_timers.syncAllLeftDurations(scene, now_ms);
        try scene.savePersistent(fs, alloc.gpa);
    }

    var roulette_info: std.ArrayList(pb.ExploreSkillRoulette) = .empty;
    try roulette_info.append(alloc.arena, .{
        .SkillIds = try rouletteSkillIds(alloc.arena, scene.explore_tools_info.roulette),
        .ExtraItemId = scene.explore_tools_info.explore_extra_item_id,
        .ExploreSkill = scene.explore_tools_info.active_explore_skill,
    });
    try roulette_info.append(alloc.arena, .{
        .SkillIds = try rouletteSkillIds(alloc.arena, scene.explore_tools_info.function_roulette),
        .ExtraItemId = scene.explore_tools_info.function_extra_item_id,
        .ExploreSkill = scene.explore_tools_info.active_function_skill,
    });
    try roulette_info.append(alloc.arena, .{
        .SkillIds = try rouletteSkillIds(alloc.arena, scene.explore_tools_info.trap_defense_roulette),
        .ExtraItemId = scene.explore_tools_info.trap_defense_extra_item_id,
        .ExploreSkill = scene.explore_tools_info.active_trap_defense_skill,
    });
    try roulette_info.append(alloc.arena, .{
        .SkillIds = try rouletteSkillIds(alloc.arena, scene.explore_tools_info.motorcycle_roulette),
        .ExtraItemId = scene.explore_tools_info.motorcycle_extra_item_id,
        .ExploreSkill = scene.explore_tools_info.active_motorcycle_skill,
    });

    try txn.conn.push(alloc.gpa, io, pb.ExploreSkillRouletteUpdateNotify{
        .RouletteInfo = roulette_info,
    });

    txn.respond(.{
        .ErrorCode = .Success,
        .SkillRoulette = selected_roulette,
        .RouletteType = roulette_type,
        .SkillRoulettes = roulette_info,
    });
}

fn summonEntityTypeFromBlueprint(assets: *const Assets, blueprint_type: []const u8) Entity.ConfigComponent.EntityType {
    const bp = assets.findBlueprintConfig(blueprint_type) orelse return .monster;
    return switch (bp.EntityLogic) {
        .Vision => .vision,
        .Monster => .monster,
        .Npc => .npc,
        .Vehicle => .vehicle,
        .Item => .scene_item,
        .ClientOnly => .client_only,
        .Custom => .custom,
        else => .monster,
    };
}

fn handleSkillSummonImpl(
    txn: *Transaction(pb.SummonRequest),
    scene: *Scene,
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    buff_timers: *BuffTimerScheduler,
    io: std.Io,
) !void {
    const now_ms = (std.Io.Clock.awake).now(io).toMilliseconds();
    const info = txn.message.SummonInfo orelse {
        txn.respond(.{ .ErrorCode = .RequestParamError });
        return;
    };
    if (info.SummonEntityId <= 0) {
        txn.respond(.{ .ErrorCode = .ErrSummonAddEntityFail });
        return;
    }
    if (scene.net_id_map.contains(info.SummonEntityId)) {
        txn.respond(.{ .ErrorCode = .ErrSummonEntityIdAlreadyExist });
        return;
    }

    const summon_cfg = assets.tables.summon_cfg.getDataById(info.SummonConfigId) orelse {
        txn.respond(.{ .ErrorCode = .ErrSummonCfgNotFound });
        return;
    };
    const template_config = if (summon_cfg.BlueprintType.len > 0)
        assets.findTemplateConfig(summon_cfg.BlueprintType)
    else
        null;
    const cfg_id = if (template_config) |tc| tc.Id else summon_cfg.Id;
    const ent_type = if (summon_cfg.BlueprintType.len > 0)
        summonEntityTypeFromBlueprint(assets, summon_cfg.BlueprintType)
    else
        Entity.ConfigComponent.EntityType.monster;

    var attr_comp: Entity.AttributeComponent = .{};
    if (scene.net_id_map.get(txn.message.SummonerEntityId)) |sidx| {
        if (scene.entities.items(.attribute)[sidx]) |sattr| {
            attr_comp = sattr;
        }
    }

    const pos = info.Pos orelse pb.Vector{};
    const rot = info.Rot orelse pb.Rotator{};
    const entity = scene.spawnWithNetId(alloc.gpa, fs, info.SummonEntityId, .{
        Entity.ConfigComponent{
            .camp = 0,
            .state = .default,
            .entity_type = ent_type,
            .config_type = if (template_config != null) .template else .old_entity,
            .config_id = cfg_id,
        },
        Entity.PlayerIDComponent{ .id = scene.player_id },
        Entity.PositionComponent{
            .location = .{ pos.X, pos.Y, pos.Z },
            .rotation = .{ rot.Roll, rot.Pitch, rot.Yaw },
        },
        if (info.IsVisible) Entity.ActorVisibleMarker{} else null,
        Entity.FightBuffComponent{},
        attr_comp,
        Entity.SummonerComponent{
            .player_id = scene.player_id,
            .summon_cfg_id = summon_cfg.Id,
            .summon_skill_id = info.SkillId,
            .summon_type = .ConcomitantCustom,
            .summoner_id = txn.message.SummonerEntityId,
        },
        Entity.LogicStateComponent{
            .direction_state = 0,
            .move_state = 0,
            .position_state = 0,
            .position_sub_state = 0,
        },
        Entity.MonsterAiComponent{
            .ai_team_init_id = 0,
            .combat_message_id = 1,
            .hatred_group_id = 0,
            .weapon_id = 0,
        },
        Entity.EquipComponent{},
        Entity.CharacterAttachComponent{
            .pb_combine_part_info_list = &.{},
            .pb_combine_target_server_id = 0,
        },
    }) catch |err| switch (err) {
        error.EntityIdAlreadyExists => {
            txn.respond(.{ .ErrorCode = .ErrSummonEntityIdAlreadyExist });
            return;
        },
        else => return err,
    };

    var born_buffs = try buffListFromIds(alloc.gpa, summon_cfg.BornBuffId);
    defer born_buffs.deinit(alloc.gpa);
    if (born_buffs.items.len > 0) {
        if (scene.entities.items(.buffs)[entity.index]) |*buffs| {
            buffs.fight_buff_infos = try scene.buildFightBuffInfos(born_buffs, entity.net_id, alloc.gpa);
        }
    }

    var entity_pbs: std.ArrayList(pb.EntityPb) = .empty;
    try entity_pbs.append(
        alloc.arena,
        try entity_proto.build(alloc, assets, scene, buff_timers, entity.net_id, now_ms),
    );
    try txn.conn.push(alloc.gpa, io, pb.EntityAddNotify{
        .EntityPbs = entity_pbs,
    });
    txn.respond(.{ .ErrorCode = .Success });
}

fn handleProjectorSummonImpl(
    txn: anytype,
    scene: *Scene,
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    echo_comp: *PlayerEchoComponent,
    buff_timers: *BuffTimerScheduler,
    io: std.Io,
) !void {
    const now_ms = (std.Io.Clock.awake).now(io).toMilliseconds();
    const info = txn.message.SummonInfo orelse {
        txn.respond(.{ .ErrorCode = .RequestParamError });
        return;
    };
    if (info.SummonEntityId <= 0) {
        txn.respond(.{ .ErrorCode = .ErrSummonAddEntityFail });
        return;
    }
    if (scene.net_id_map.contains(info.SummonEntityId)) {
        txn.respond(.{ .ErrorCode = .ErrSummonEntityIdAlreadyExist });
        return;
    }
    if (!phantom_projector.isUnlockedMonster(assets, info.SummonConfigId)) {
        txn.respond(.{ .ErrorCode = .ErrPhantomInteractionNotUnlock });
        return;
    }

    const item = phantom_projector.displayedPhantomItem(assets, echo_comp.calabash_info, info.SummonConfigId) orelse {
        txn.respond(.{ .ErrorCode = .ErrPhantomInteractionConfigNotFind });
        return;
    };
    const summon_id = if (item.MeshId != 0) item.MeshId else item.SkillId;
    if (summon_id == 0) {
        txn.respond(.{ .ErrorCode = .ErrSummonCfgNotFound });
        return;
    }

    const entity = RoleEntityTemplates.createProjectorVisionEntity(
        fs,
        scene,
        alloc,
        scene.player_id,
        assets,
        summon_id,
        txn.message.SummonerEntityId,
        info.SummonEntityId,
        info.Pos orelse .{},
        info.Rot orelse .{},
    ) catch |err| switch (err) {
        error.EntityIdAlreadyExists => {
            txn.respond(.{ .ErrorCode = .ErrSummonEntityIdAlreadyExist });
            return;
        },
        else => return err,
    } orelse {
        txn.respond(.{ .ErrorCode = .ErrSummonCfgNotFound });
        return;
    };

    var entity_pbs: std.ArrayList(pb.EntityPb) = .empty;
    try entity_pbs.append(
        alloc.arena,
        try entity_proto.build(
            alloc,
            assets,
            scene,
            buff_timers,
            entity.net_id,
            now_ms,
        ),
    );
    try txn.conn.push(alloc.gpa, io, pb.EntityAddNotify{
        .EntityPbs = entity_pbs,
    });
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSummonRequest(
    txn: *Transaction(pb.SummonRequest),
    scene: *Scene,
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    buff_timers: *BuffTimerScheduler,
    io: std.Io,
) !void {
    try handleSkillSummonImpl(txn, scene, alloc, fs, assets, buff_timers, io);
}

pub fn onSummon3Request(
    txn: *Transaction(pb.Summon3Request),
    scene: *Scene,
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    echo_comp: *PlayerEchoComponent,
    buff_timers: *BuffTimerScheduler,
    io: std.Io,
) !void {
    try handleProjectorSummonImpl(txn, scene, alloc, fs, assets, echo_comp, buff_timers, io);
}

pub fn onRemoveSummonEntityRequest(
    txn: *Transaction(pb.RemoveSummonEntityRequest),
    scene: *Scene,
    alloc: mem.Alloc,
    fs: *FileSystem,
    io: std.Io,
) !void {
    var remove_infos: std.ArrayList(pb.EntityRemoveInfo) = .empty;

    for (txn.message.RemoveENtityIds.items) |entity_id| {
        const index = scene.net_id_map.get(entity_id) orelse continue;
        const storage = scene.entities.get(index);
        const summoner = storage.summoner orelse continue;
        if (summoner.summoner_id != txn.message.SummonerId) continue;

        try scene.remove(alloc.gpa, fs, entity_id);
        try remove_infos.append(alloc.arena, .{ .EntityId = entity_id });
    }

    if (remove_infos.items.len != 0) {
        try txn.conn.push(alloc.gpa, io, pb.EntityRemoveNotify{
            .IsRemove = true,
            .RemoveInfos = remove_infos,
        });
    }

    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onPlayerAccessCasketRequest(
    txn: *Transaction(pb.PlayerAccessCasketRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [48]u8 = undefined;
    const val: i64 = if (txn.message.IsEnter) 1 else 0;
    if (txn.message.AccessEntityIncIds.items.len > 0) {
        for (txn.message.AccessEntityIncIds.items) |inc_id| {
            const k = try std.fmt.bufPrint(&kbuf, "casket:player:{d}", .{inc_id});
            try progress_comp.info.setKeyInt(alloc.gpa, k, val);
        }
    } else if (txn.message.CasketEntityIncId != 0) {
        const k = try std.fmt.bufPrint(&kbuf, "casket:player:{d}", .{txn.message.CasketEntityIncId});
        try progress_comp.info.setKeyInt(alloc.gpa, k, val);
    }
    try progress_comp.save(fs, alloc.arena);
    const resp_inc: i64 = if (txn.message.AccessEntityIncIds.items.len > 0)
        txn.message.AccessEntityIncIds.items[0]
    else
        txn.message.CasketEntityIncId;
    txn.respond(.{
        .ErrorCode = .Success,
        .CasketEntityIncId = resp_inc,
    });
}

pub fn onEntityAccessCasketRequest(
    txn: *Transaction(pb.EntityAccessCasketRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    var kbuf: [64]u8 = undefined;
    const val: i64 = if (txn.message.IsEnter) 1 else 0;
    if (txn.message.AccessEntityIncIds.items.len > 0) {
        for (txn.message.AccessEntityIncIds.items) |inc_id| {
            const k = try std.fmt.bufPrint(&kbuf, "casket:ent:{d}:{d}", .{ txn.message.EntityIncId, inc_id });
            try progress_comp.info.setKeyInt(alloc.gpa, k, val);
        }
    } else {
        const k = try std.fmt.bufPrint(&kbuf, "casket:ent:{d}", .{txn.message.EntityIncId});
        try progress_comp.info.setKeyInt(alloc.gpa, k, val);
    }
    try progress_comp.save(fs, alloc.arena);
    const resp_inc: i64 = if (txn.message.AccessEntityIncIds.items.len > 0)
        txn.message.AccessEntityIncIds.items[0]
    else
        txn.message.CasketEntityIncId;
    txn.respond(.{
        .ErrorCode = .Success,
        .CasketEntityIncId = resp_inc,
    });
}

pub fn onExploreEntityTraceRequest(
    txn: *Transaction(pb.ExploreEntityTraceRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "explore:trace_area", txn.message.AreaId);
    try progress_comp.save(fs, alloc.arena);
    var eids: std.ArrayList(i32) = .empty;
    if (txn.message.ExploratoryDegree > 0) {
        if (assets.getDb()) |db| {
            const found = try db.queryEntityIdsByAreaExplore(txn.message.AreaId, txn.message.ExploratoryDegree, alloc.arena);
            const cap = @min(found.len, 32);
            try eids.appendSlice(alloc.arena, found[0..cap]);
        } else {
            for (assets.tables.level_entity_config.items) |lcfg| {
                if (txn.message.AreaId != 0 and lcfg.AreaId != txn.message.AreaId) continue;
                const bi = lcfg.ComponentsData.BaseInfoComponent orelse continue;
                const cat = bi.Category orelse continue;
                if ((cat.ExploratoryDegree orelse 0) != txn.message.ExploratoryDegree) continue;
                if (lcfg.EntityId > 0 and lcfg.EntityId <= std.math.maxInt(i32)) {
                    try eids.append(alloc.arena, @intCast(lcfg.EntityId));
                }
                if (eids.items.len >= 32) break;
            }
        }
    }
    if (eids.items.len == 0 and txn.message.ExploratoryDegree == 0) {
        for (assets.tables.map_mark.items) |m| {
            if (txn.message.AreaId == 0 or m.RelativeId == txn.message.AreaId) {
                if (m.EntityConfigId > 0 and m.EntityConfigId <= std.math.maxInt(i32)) {
                    try eids.append(alloc.arena, @intCast(m.EntityConfigId));
                }
                if (eids.items.len >= 32) break;
            }
        }
    }
    txn.respond(.{
        .EntityIds = eids,
        .ErrorCode = .Success,
    });
}

pub fn onMoralePosRequest(
    txn: *Transaction(pb.MoralePosRequest),
    progress_comp: *const PlayerProgressComponent,
) !void {
    const pos = @as(i32, @intCast(progress_comp.info.getKeyInt("morale:area_pos")));
    txn.respond(.{ .AreaPos = pos });
}

pub fn onMoraleTreasureBoxTraceRequest(
    txn: *Transaction(pb.MoraleTreasureBoxTraceRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "morale:area_pos", txn.message.AreaPos);
    try progress_comp.save(fs, alloc.arena);
    var eids: std.ArrayList(i32) = .empty;
    for (assets.tables.map_mark.items) |m| {
        if (m.EntityConfigId > 0 and m.EntityConfigId <= std.math.maxInt(i32) and (txn.message.AreaPos == 0 or m.RelativeId == txn.message.AreaPos)) {
            try eids.append(alloc.arena, @intCast(m.EntityConfigId));
            if (eids.items.len >= 16) break;
        }
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .AreaPos = txn.message.AreaPos,
        .EntityIds = eids,
    });
}
