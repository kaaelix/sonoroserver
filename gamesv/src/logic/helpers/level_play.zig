const std = @import("std");
const pb = @import("proto").pb;
const Assets = @import("../../data/Assets.zig");
const Components = @import("../../data/tables/entity_components/Components.zig");
const LazyNodeStore = @import("../../data/LazyNodeStore.zig");
const FileSystem = @import("common").FileSystem;
const Connection = @import("../../network/Connection.zig");
const Scene = @import("../Scene.zig");
const EventQueue = @import("../EventQueue.zig");
const mem = @import("../../mem.zig");
const PlayerBasicComponent = @import("../component/player/PlayerBasicComponent.zig");
const PlayerInventoryComponent = @import("../component/player/PlayerInventoryComponent.zig");
const PlayerWeaponComponent = @import("../component/player/PlayerWeaponComponent.zig");
const PlayerEchoComponent = @import("../component/player/PlayerEchoComponent.zig");
const PlayerMapComponent = @import("../component/player/PlayerMapComponent.zig");
const PlayerQuestComponent = @import("../component/player/PlayerQuestComponent.zig");
const PlayerRoleComponent = @import("../component/player/PlayerRoleComponent.zig");
const PlayerGuideComponent = @import("../component/player/PlayerGuideComponent.zig");
const PlayerMailComponent = @import("../component/player/PlayerMailComponent.zig");
const PlayerProgressComponent = @import("../component/player/PlayerProgressComponent.zig");
const PlayerSceneComponent = @import("../component/player/PlayerSceneComponent.zig");
const RewardService = @import("reward.zig");
const QuestEngine = @import("quest_engine.zig");
const ConditionEvaluator = @import("condition.zig");
const QuestInfo = @import("../../fs/QuestInfo.zig");
const prologue_quest = @import("prologue_quest.zig");

const log = std.log.scoped(.level_play);

pub const LevelPlayContext = struct {
    assets: *const Assets,
    fs: *FileSystem,
    conn: *Connection,
    scene: *Scene,
    events: *EventQueue,
    alloc: mem.Alloc,
    io: std.Io,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    map_comp: *PlayerMapComponent,
    quest_comp: *PlayerQuestComponent,
    role_comp: *PlayerRoleComponent,
    scene_comp: ?*PlayerSceneComponent = null,
    guide_comp: ?*PlayerGuideComponent = null,
    mail_comp: ?*PlayerMailComponent = null,
    progress_comp: ?*PlayerProgressComponent = null,
};

pub const ResolvedEntityConfig = struct {
    blueprint_type: []const u8,
    components: Components,
};

fn toRewardCtx(ctx: LevelPlayContext) RewardService.RewardContext {
    return .{
        .alloc = ctx.alloc,
        .io = ctx.io,
        .fs = ctx.fs,
        .assets = ctx.assets,
        .conn = ctx.conn,
        .basic_comp = ctx.basic_comp,
        .inventory_comp = ctx.inventory_comp,
        .weapon_comp = ctx.weapon_comp,
        .echo_comp = ctx.echo_comp,
    };
}

fn toQuestCtx(ctx: LevelPlayContext) QuestEngine.QuestContext {
    return .{
        .assets = ctx.assets,
        .fs = ctx.fs,
        .conn = ctx.conn,
        .scene = ctx.scene,
        .scene_comp = ctx.scene_comp,
        .events = ctx.events,
        .alloc = ctx.alloc,
        .io = ctx.io,
        .basic_comp = ctx.basic_comp,
        .inventory_comp = ctx.inventory_comp,
        .weapon_comp = ctx.weapon_comp,
        .echo_comp = ctx.echo_comp,
        .map_comp = ctx.map_comp,
        .quest_comp = ctx.quest_comp,
        .role_comp = ctx.role_comp,
        .guide_comp = ctx.guide_comp,
        .mail_comp = ctx.mail_comp,
        .progress_comp = ctx.progress_comp,
    };
}

pub fn resolveEntityConfig(assets: *const Assets, scene_map_id: i32, config_id: i64) ?ResolvedEntityConfig {
    if (config_id <= 0) return null;

    var matched: ?*const Assets.DataTables.LevelEntityConfig = assets.findLevelEntity(scene_map_id, config_id);
    if (matched == null) {
        const indices = assets.indexes.levelEntityIndicesById(config_id);
        if (indices.len > 0) {
            matched = &assets.tables.level_entity_config.items[indices[0]];
        }
    }

    if (matched) |cfg| {
        var comps = cfg.ComponentsData;
        if (assets.findTemplateConfig(cfg.BlueprintType)) |tpl| {
            tpl.ComponentsData.mergeInto(&comps);
        }
        return .{
            .blueprint_type = cfg.BlueprintType,
            .components = comps,
        };
    }

    if (std.math.cast(i32, config_id)) |tid| {
        if (assets.tables.template_config.getDataById(tid)) |tpl| {
            return .{
                .blueprint_type = tpl.BlueprintType,
                .components = tpl.ComponentsData,
            };
        }
    }

    return null;
}

fn matchesSceneInstance(assets: *const Assets, lp: Assets.DataTables.LevelPlayData, scene_instance_id: i32) bool {
    const expected_inst: i32 = if (lp.Data.InstanceId != 0) lp.Data.InstanceId else lp.Data.LevelId;
    if (expected_inst == 0 or expected_inst == scene_instance_id) return true;
    return assets.resolveMapConfigId(scene_instance_id) == expected_inst;
}

fn jsonFloat(v_opt: ?std.json.Value) ?f32 {
    const v = v_opt orelse return null;
    return switch (v) {
        .integer => |n| @floatFromInt(n),
        .float => |f| if (std.math.isFinite(f)) @floatCast(f) else null,
        else => null,
    };
}

fn isPlayerWithinReachArea(ctx: LevelPlayContext, cond_obj: std.json.ObjectMap, x: f32, y: f32) bool {
    const range = jsonFloat(cond_obj.get("Range")) orelse return true;
    if (range <= 0) return true;
    const range_sq = range * range;

    if (cond_obj.get("RangeEntities")) |re_val| {
        if (re_val == .array and re_val.array.items.len > 0) {
            var resolved_any = false;
            for (re_val.array.items) |item| {
                const eid = jsonInt(item) orelse continue;
                var target_pos: ?[3]f32 = null;
                const slice = ctx.scene.entities.slice();
                for (slice.items(.config), slice.items(.position)) |cfg, pos_opt| {
                    if (cfg.config_id == eid) {
                        if (pos_opt) |p| {
                            target_pos = p.location;
                            break;
                        }
                    }
                }
                if (target_pos == null) {
                    if (ctx.assets.findLevelEntity(ctx.scene.instance_id, eid)) |lcfg| {
                        target_pos = Assets.AssetIndexes.entityWorldPos(lcfg);
                    }
                }
                if (target_pos) |tp| {
                    resolved_any = true;
                    const dx = tp[0] - x;
                    const dy = tp[1] - y;
                    if (dx * dx + dy * dy <= range_sq) return true;
                }
            }
            return !resolved_any;
        }
    }

    if (cond_obj.get("Pos")) |pos_val| {
        if (pos_val == .object) {
            const px = jsonFloat(pos_val.object.get("X")) orelse 0;
            const py = jsonFloat(pos_val.object.get("Y")) orelse 0;
            const pz = jsonFloat(pos_val.object.get("Z")) orelse 0;
            if (px != 0 or py != 0 or pz != 0) {
                const dx = px - x;
                const dy = py - y;
                return dx * dx + dy * dy <= range_sq;
            }
        }
    }

    return true;
}

fn jsonInt(v_opt: ?std.json.Value) ?i32 {
    const v = v_opt orelse return null;
    return switch (v) {
        .integer => |n| std.math.cast(i32, n),
        .float => |f| {
            if (!std.math.isFinite(f)) return null;
            const min_f: f64 = @floatFromInt(std.math.minInt(i32));
            const max_f: f64 = @floatFromInt(std.math.maxInt(i32));
            if (f < min_f or f > max_f) return null;
            return @intFromFloat(f);
        },
        else => null,
    };
}

fn jsonStr(v_opt: ?std.json.Value) ?[]const u8 {
    const v = v_opt orelse return null;
    return switch (v) {
        .string => |s| s,
        else => null,
    };
}

fn jsonArrayContainsInt(arr_opt: ?std.json.Value, needle: i32) bool {
    const v = arr_opt orelse return false;
    if (v != .array) return false;
    for (v.array.items) |item| {
        if (jsonInt(item)) |id| {
            if (id == needle) return true;
        }
    }
    return false;
}

fn isTargetStillAliveInScene(scene: *Scene, cfg_id: i32, exclude_net_id: i64) bool {
    const slice = scene.entities.slice();
    const configs = slice.items(.config);
    const net_ids = slice.items(.entity_id);
    for (configs, net_ids) |cfg, nid| {
        if (nid.net_id == exclude_net_id) continue;
        if (scene.rewarded_kills.contains(nid.net_id)) continue;
        if (cfg.config_id == cfg_id) return true;
    }
    return false;
}

fn ensureEntitySpawned(ctx: LevelPlayContext, config_id: i32) void {
    if (config_id <= 0) return;
    const slice = ctx.scene.entities.slice();
    for (slice.items(.config)) |cfg| {
        if (cfg.config_id == config_id) return;
    }
    if (ctx.quest_comp.info.hasOpenedChest(config_id)) return;
    prologue_quest.spawnQuestEntity(
        ctx.events,
        ctx.scene,
        ctx.fs,
        ctx.assets,
        ctx.conn,
        ctx.basic_comp,
        ctx.alloc,
        ctx.io,
        config_id,
    );
}

fn runLevelPlayActions(ctx: LevelPlayContext, owner_id: i32, node_id: i32, actions_opt: ?std.json.Value) void {
    const actions_val = actions_opt orelse return;
    if (actions_val != .array) return;

    QuestEngine.executeActionList(
        toQuestCtx(ctx),
        QuestEngine.actionCallbacks(),
        owner_id,
        node_id,
        actions_val.array.items,
    ) catch {};

    for (actions_val.array.items) |act_val| {
        if (act_val != .object) continue;
        const name = jsonStr(act_val.object.get("Name")) orelse continue;
        const params_val = act_val.object.get("Params") orelse continue;
        if (params_val != .object) continue;
        const p = params_val.object;

        if (std.mem.eql(u8, name, "AwakeEntity") or std.mem.eql(u8, name, "UnlockEntity")) {
            if (jsonInt(p.get("EntityId"))) |eid| ensureEntitySpawned(ctx, eid);
            if (p.get("EntityIds")) |eids| {
                if (eids == .array) {
                    for (eids.array.items) |v| {
                        if (jsonInt(v)) |eid| ensureEntitySpawned(ctx, eid);
                    }
                }
            }
        }
    }
}

pub fn completeLevelPlay(ctx: LevelPlayContext, level_play_id: i32) !void {
    if (level_play_id <= 0) return;
    if (QuestInfo.containsInt(ctx.quest_comp.info.completed_level_plays, level_play_id)) return;

    const was_entered = try QuestInfo.removeIntValue(&ctx.quest_comp.info.progress.entered_level_plays, ctx.alloc.gpa, level_play_id);
    try ctx.quest_comp.info.markLevelPlayCompleted(ctx.alloc.gpa, level_play_id);
    try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);

    if (ctx.progress_comp) |pc| {
        var kbuf: [48]u8 = undefined;
        const k = std.fmt.bufPrint(&kbuf, "lp_state:{d}", .{level_play_id}) catch "";
        if (k.len > 0) {
            pc.info.setKeyInt(ctx.alloc.gpa, k, 3) catch {};
            pc.save(ctx.fs, ctx.alloc.arena) catch {};
        }
    }

    if (was_entered) {
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.LevelPlayFirstNotify{
            .Id = level_play_id,
        });
    }
    try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.LevelPlayStateNotify{
        .Id = level_play_id,
        .State = 3,
        .CompleteNumber = 1,
    });
    if (was_entered) {
        try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.LeaveLevelPlayNotify{
            .Id = level_play_id,
        });
    }

    var infos: std.ArrayList(pb.LevelPlayInfo) = .empty;
    try infos.append(ctx.alloc.arena, .{
        .Id = level_play_id,
        .IsFirst = true,
        .State = 3,
    });
    try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.LevelPlayInfoNotify{
        .LevelPlayInfo = infos,
    });

    const qctx = toQuestCtx(ctx);
    try QuestEngine.onLevelPlayCompleted(qctx, level_play_id);
    if (ctx.scene.instance_id != 8) {
        try QuestEngine.onDungeonEvent(qctx, ctx.scene.instance_id, true);
    }
    if (ctx.assets.level_play_nodes.getOwnerNodes(ctx.io, level_play_id)) |lp_nodes| {
        for (lp_nodes) |node| {
            if (!std.mem.eql(u8, node.node_type, "QuestSucceed")) continue;
            if (node.data != .object) continue;
            if (node.data.object.get("FinishActions")) |acts_val| {
                if (acts_val == .array) {
                    QuestEngine.executeActionList(qctx, QuestEngine.actionCallbacks(), level_play_id, node.id, acts_val.array.items) catch {};
                }
            }
        }
    } else |_| {}
    log.info("Completed LevelPlay {d}", .{level_play_id});
}

fn removeEntityFromSceneAndNotify(ctx: LevelPlayContext, net_id: i64) void {
    var remove_infos: std.ArrayList(pb.EntityRemoveInfo) = .empty;
    remove_infos.append(ctx.alloc.arena, .{ .EntityId = net_id }) catch {};
    ctx.scene.remove(ctx.alloc.gpa, ctx.fs, net_id) catch {};
    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.EntityRemoveNotify{
        .IsRemove = true,
        .RemoveInfos = remove_infos,
    }) catch {};
}

fn checkLevelPlayKillProgress(ctx: LevelPlayContext, killed_cfg_id: i32, killed_net_id: i64) !void {
    if (killed_cfg_id <= 0) return;

    var candidate_lp_indices: std.ArrayList(usize) = .empty;
    for (ctx.assets.indexes.levelPlayIndicesForRefEntity(killed_cfg_id)) |lp_idx| {
        try candidate_lp_indices.append(ctx.alloc.arena, lp_idx);
    }
    if (ctx.scene.instance_id != 8) {
        for (ctx.assets.tables.level_play_data.items, 0..) |lp, lp_idx| {
            if ((lp.Data.InstanceId != 0 and lp.Data.InstanceId == ctx.scene.instance_id) or
                (lp.Data.LevelId != 0 and lp.Data.LevelId == ctx.scene.instance_id))
            {
                if (std.mem.indexOfScalar(usize, candidate_lp_indices.items, lp_idx) == null) {
                    try candidate_lp_indices.append(ctx.alloc.arena, lp_idx);
                }
            }
        }
    }

    for (candidate_lp_indices.items) |lp_idx| {
        const lp = ctx.assets.tables.level_play_data.items[lp_idx];
        if (!matchesSceneInstance(ctx.assets, lp, ctx.scene.instance_id)) continue;
        if (QuestInfo.containsInt(ctx.quest_comp.info.completed_level_plays, lp.LevelPlayId)) continue;

        const nodes = ctx.assets.level_play_nodes.getOwnerNodes(ctx.io, lp.LevelPlayId) catch continue;
        for (nodes) |node| {
            if (node.data != .object) continue;
            const ntype = jsonStr(node.data.object.get("Type")) orelse continue;
            if (!std.mem.eql(u8, ntype, "ChildQuest")) continue;
            const cond_val = node.data.object.get("Condition") orelse continue;
            if (cond_val != .object) continue;
            const ctype = jsonStr(cond_val.object.get("Type")) orelse continue;
            if (!std.mem.eql(u8, ctype, "Kill")) continue;

            const in_exist = jsonArrayContainsInt(cond_val.object.get("ExistTargets"), killed_cfg_id);
            const in_awake = jsonArrayContainsInt(cond_val.object.get("TargetsToAwake"), killed_cfg_id);
            if (!in_exist and !in_awake) continue;

            // Check whether all targets in this Kill node are now defeated.
            var any_alive = false;
            if (cond_val.object.get("ExistTargets")) |et| {
                if (et == .array) {
                    for (et.array.items) |item| {
                        const tid = jsonInt(item) orelse continue;
                        if (tid != killed_cfg_id and isTargetStillAliveInScene(ctx.scene, tid, killed_net_id)) {
                            any_alive = true;
                            break;
                        }
                    }
                }
            }
            if (!any_alive) {
                if (cond_val.object.get("TargetsToAwake")) |ta| {
                    if (ta == .array) {
                        for (ta.array.items) |item| {
                            const tid = jsonInt(item) orelse continue;
                            if (tid != killed_cfg_id and isTargetStillAliveInScene(ctx.scene, tid, killed_net_id)) {
                                any_alive = true;
                                break;
                            }
                        }
                    }
                }
            }
            if (any_alive) break;

            // Kill objective cleared! Run node FinishActions (often spawns/unlocks chest).
            runLevelPlayActions(ctx, lp.LevelPlayId, node.id, node.data.object.get("FinishActions"));

            const has_reward_chest = if (lp.Data.LevelPlayRewardConfig) |rc|
                rc.RewardEntityId > 0 and !ctx.quest_comp.info.hasOpenedChest(rc.RewardEntityId)
            else
                false;

            if (lp.Data.LevelPlayRewardConfig) |rc| {
                if (rc.RewardEntityId > 0) {
                    ensureEntitySpawned(ctx, rc.RewardEntityId);
                }
            }

            if (!has_reward_chest) {
                if (lp.Data.LevelPlayRewardConfig) |rc| {
                    if (rc.RewardId > 0) {
                        _ = try RewardService.grantTreasureBoxReward(toRewardCtx(ctx), 2, rc.RewardId, 7000);
                    }
                }
                try completeLevelPlay(ctx, lp.LevelPlayId);
            }
            break;
        }
    }
}

const normal_monster_respawn_ms: i64 = 60_000;
const boss_monster_respawn_ms: i64 = 180_000;
const collectible_respawn_ms: i64 = 300_000;
const animal_respawn_ms: i64 = 60_000;

fn recordWorldRespawnCooldown(ctx: LevelPlayContext, cfg_id: i32, duration_ms: i64) !void {
    if (cfg_id <= 0 or duration_ms <= 0) return;
    const now_ms = std.Io.Clock.awake.now(ctx.io).toMilliseconds();
    try ctx.scene.instance.setRespawnCooldown(ctx.alloc.gpa, cfg_id, now_ms, now_ms + duration_ms);
    try ctx.scene.savePersistent(ctx.fs, ctx.alloc.gpa);
}

fn spawnVisionCapturePhantom(
    ctx: LevelPlayContext,
    killed_net_id: i64,
    killed_cfg_id: i32,
    blueprint_type: []const u8,
    vision_id: i32,
) void {
    if (vision_id <= 0) return;
    const tpl = ctx.assets.findTemplateConfig(blueprint_type) orelse return;
    if (tpl.Id <= 0) return;

    var pos: [3]f32 = if (ctx.scene.instance.players.len > 0) ctx.scene.instance.players[0].location else .{ 0, 0, 0 };
    var rot: [3]f32 = .{ 0, 0, 0 };
    if (ctx.scene.net_id_map.get(killed_net_id)) |idx| {
        if (ctx.scene.entities.items(.position)[idx]) |p| {
            pos = p.location;
            rot = p.rotation;
        }
    } else if (ctx.assets.findLevelEntity(ctx.scene.instance_id, killed_cfg_id)) |lcfg| {
        if (Assets.AssetIndexes.entityWorldPos(lcfg)) |wpos| pos = wpos;
        rot = Assets.AssetIndexes.entityWorldRot(lcfg);
    }

    const entity = ctx.scene.spawn(ctx.alloc.gpa, ctx.fs, .{
        Scene.Entity.ConfigComponent{
            .camp = 2,
            .config_id = 14000056,
            .config_type = .template,
            .entity_type = .scene_item,
            .state = .default,
        },
        Scene.Entity.ActorVisibleMarker{},
        Scene.Entity.VisibleMarker{},
        Scene.Entity.PositionComponent{
            .location = pos,
            .rotation = rot,
        },
    }) catch return;

    ctx.scene.entities.items(.world_item)[entity.index].monster_capture = .{
        .template_id = tpl.Id,
        .entity_id = killed_cfg_id,
        .monster_id = vision_id,
    };
    ctx.scene.saveEntity(ctx.fs, ctx.alloc.gpa, entity) catch {};

    const storage = ctx.scene.entities.get(entity.index);
    const proto_pb = storage.entityToProto(entity.net_id, ctx.alloc, ctx.assets) catch return;
    var entity_pbs: std.ArrayList(pb.EntityPb) = .empty;
    entity_pbs.append(ctx.alloc.arena, proto_pb) catch return;
    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.EntityAddNotify{ .EntityPbs = entity_pbs }) catch {};
}

pub fn onEntityKilled(ctx: LevelPlayContext, target_net_id: i64, cfg_id_opt: ?i32) !void {
    if (ctx.scene.rewarded_kills.contains(target_net_id)) return;
    try ctx.scene.rewarded_kills.put(ctx.alloc.gpa, target_net_id, {});
    errdefer _ = ctx.scene.rewarded_kills.remove(target_net_id);

    var cfg_id: i32 = cfg_id_opt orelse 0;
    if (cfg_id <= 0) {
        if (ctx.scene.net_id_map.get(target_net_id)) |idx| {
            cfg_id = ctx.scene.entities.items(.config)[idx].config_id;
        }
    }

    if (cfg_id > 0) {
        if (resolveEntityConfig(ctx.assets, ctx.scene.instance_id, cfg_id)) |resolved| {
            const reward_id = if (resolved.components.RewardComponent) |rc| (rc.RewardId orelse 0) else 0;
            const vision_id = if (resolved.components.VisionCaptureComponent) |vc| (vc.VisionCaptureId orelse 0) else 0;
            const is_monster = std.mem.indexOf(u8, resolved.blueprint_type, "Monster") != null;
            const is_boss = std.mem.indexOf(u8, resolved.blueprint_type, "Boss") != null or
                std.mem.indexOf(u8, resolved.blueprint_type, "Elite") != null;

            if (is_monster or reward_id > 0 or vision_id > 0) {
                const calabash_lvl = ConditionEvaluator.currentCalabashLevel(ctx.assets, ctx.basic_comp, ctx.echo_comp);
                _ = try RewardService.grantMonsterDropWithEchoOption(toRewardCtx(ctx), reward_id, vision_id, is_boss, calabash_lvl, false);
                if (vision_id > 0) {
                    spawnVisionCapturePhantom(ctx, target_net_id, cfg_id, resolved.blueprint_type, vision_id);
                }
            }
            try recordWorldRespawnCooldown(ctx, cfg_id, if (is_boss) boss_monster_respawn_ms else normal_monster_respawn_ms);
            try checkLevelPlayKillProgress(ctx, cfg_id, target_net_id);
            if ((is_monster or reward_id > 0 or vision_id > 0) and is_boss and ctx.scene.instance_id != 8) {
                try QuestEngine.onDungeonEvent(toQuestCtx(ctx), ctx.scene.instance_id, true);
            }
        } else {
            try checkLevelPlayKillProgress(ctx, cfg_id, target_net_id);
        }
    }
}

pub fn onGetRewardTreasureBox(ctx: LevelPlayContext, net_id: i64) !void {
    const idx = ctx.scene.net_id_map.get(net_id) orelse return;
    const cfg_id = ctx.scene.entities.items(.config)[idx].config_id;
    if (cfg_id > 0 and ctx.quest_comp.info.hasOpenedChest(cfg_id)) {
        return;
    }

    var chest_type_id: i32 = 1;
    var reward_id: i32 = 0;

    if (resolveEntityConfig(ctx.assets, ctx.scene.instance_id, cfg_id)) |resolved| {
        if (resolved.components.TreasureBoxComponent) |tb| {
            if (tb.TypeId) |tid| chest_type_id = tid;
        }
        if (resolved.components.RewardComponent) |rc| {
            if (rc.RewardId) |rid| reward_id = rid;
        }
    }

    var completed_lp_id: ?i32 = null;
    if (cfg_id > 0) {
        if (std.math.cast(i32, cfg_id)) |cfg_id32| {
            const reward_indices = ctx.assets.indexes.levelPlayIndicesForRewardEntity(cfg_id32);
            for (reward_indices) |lp_idx| {
                const lp = ctx.assets.tables.level_play_data.items[lp_idx];
                if (!matchesSceneInstance(ctx.assets, lp, ctx.scene.instance_id)) continue;
                if (lp.Data.LevelPlayRewardConfig) |rc| {
                    if (reward_id == 0 and rc.RewardId > 0) reward_id = rc.RewardId;
                }
                completed_lp_id = lp.LevelPlayId;
                break;
            }
        }
    }

    // Grant before marking chest opened so a failed grant can be retried cleanly.
    _ = try RewardService.grantTreasureBoxReward(toRewardCtx(ctx), chest_type_id, reward_id, 7000);

    if (cfg_id > 0) {
        _ = try ctx.quest_comp.info.markChestOpened(ctx.alloc.gpa, cfg_id);
        _ = try ctx.quest_comp.info.setEntityState(ctx.alloc.gpa, cfg_id, "关卡.Common.状态.激活");
        try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);
    }

    if (try ctx.scene.updateEntityStateTag(ctx.alloc.gpa, &ctx.assets.tables.gameplay_tag_remap, net_id, "关卡.Common.状态.激活")) |upd| {
        try QuestEngine.emitEntityStateTagUpdate(ctx.conn, ctx.alloc.gpa, ctx.alloc.arena, ctx.io, net_id, upd);
    }

    if (completed_lp_id) |lp_id| {
        try completeLevelPlay(ctx, lp_id);
    }
}

pub fn onEntityInteractFallback(ctx: LevelPlayContext, net_id: i64) !bool {
    const idx = ctx.scene.net_id_map.get(net_id) orelse return false;
    const cfg_id = ctx.scene.entities.items(.config)[idx].config_id;
    if (cfg_id <= 0) return false;

    const resolved = resolveEntityConfig(ctx.assets, ctx.scene.instance_id, cfg_id) orelse return false;

    if (resolved.components.TreasureBoxComponent != null or
        std.mem.indexOf(u8, resolved.blueprint_type, "TreasureBox") != null)
    {
        try onGetRewardTreasureBox(ctx, net_id);
        return true;
    }

    const is_collectible = resolved.components.RewardComponent != null and
        (std.mem.indexOf(u8, resolved.blueprint_type, "Collect") != null or
            std.mem.indexOf(u8, resolved.blueprint_type, "Pick") != null);

    if (is_collectible) {
        const now_ms = std.Io.Clock.awake.now(ctx.io).toMilliseconds();
        if (!ctx.quest_comp.info.hasOpenedChest(cfg_id) and !ctx.scene.instance.isOnRespawnCooldown(cfg_id, now_ms)) {
            const reward_id = if (resolved.components.RewardComponent) |rc| (rc.RewardId orelse 0) else 0;
            if (reward_id > 0) {
                const res = try RewardService.grantDropPackage(toRewardCtx(ctx), reward_id, 1);
                if (res.granted_count == 0) {
                    const items = [_]RewardService.RewardItem{.{ .item_id = 2, .count = 200 }};
                    _ = try RewardService.grantItems(toRewardCtx(ctx), 0, false, &items, 1);
                }
            }
            try recordWorldRespawnCooldown(ctx, cfg_id, collectible_respawn_ms);
        }
        removeEntityFromSceneAndNotify(ctx, net_id);
        return true;
    }

    // Also advance any nearby LevelPlay waiting on DoInteract for this entity.
    const cfg_id32 = std.math.cast(i32, cfg_id) orelse return false;
    for (ctx.assets.indexes.levelPlayIndicesForRefEntity(cfg_id32)) |lp_idx| {
        const lp = ctx.assets.tables.level_play_data.items[lp_idx];
        if (!matchesSceneInstance(ctx.assets, lp, ctx.scene.instance_id)) continue;
        if (QuestInfo.containsInt(ctx.quest_comp.info.completed_level_plays, lp.LevelPlayId)) continue;

        const nodes = ctx.assets.level_play_nodes.getOwnerNodes(ctx.io, lp.LevelPlayId) catch continue;
        var matched_interact = false;
        for (nodes) |node| {
            if (node.data != .object) continue;
            const ntype = jsonStr(node.data.object.get("Type")) orelse continue;
            if (!std.mem.eql(u8, ntype, "ChildQuest")) continue;
            const cond_val = node.data.object.get("Condition") orelse continue;
            if (cond_val != .object) continue;
            const ctype = jsonStr(cond_val.object.get("Type")) orelse continue;
            if (!std.mem.eql(u8, ctype, "DoInteract")) continue;
            runLevelPlayActions(ctx, lp.LevelPlayId, node.id, node.data.object.get("FinishActions"));
            matched_interact = true;
        }
        if (matched_interact) {
            try completeLevelPlay(ctx, lp.LevelPlayId);
            return true;
        }
    }

    return false;
}

pub fn onAnimalDrop(ctx: LevelPlayContext, net_id: i64) !void {
    const idx = ctx.scene.net_id_map.get(net_id) orelse return;
    const cfg_id = ctx.scene.entities.items(.config)[idx].config_id;
    var reward_id: i32 = 0;
    if (resolveEntityConfig(ctx.assets, ctx.scene.instance_id, cfg_id)) |resolved| {
        if (resolved.components.RewardComponent) |rc| {
            reward_id = rc.RewardId orelse 0;
        }
    }
    var res: RewardService.GrantResult = .{};
    if (reward_id > 0) {
        res = try RewardService.grantDropPackage(toRewardCtx(ctx), reward_id, 1);
    }
    if (res.granted_count == 0) {
        const items = [_]RewardService.RewardItem{
            .{ .item_id = 42300010, .count = 1 },
            .{ .item_id = 2, .count = 100 },
        };
        _ = try RewardService.grantItems(toRewardCtx(ctx), 0, false, &items, 1);
    }
    try recordWorldRespawnCooldown(ctx, cfg_id, animal_respawn_ms);
    removeEntityFromSceneAndNotify(ctx, net_id);
}

/// Removes a hunted or destructible animal entity cleanly on client `AnimalDestroyRequest`.
pub fn onAnimalDestroy(ctx: LevelPlayContext, net_id: i64) !void {
    const idx = ctx.scene.net_id_map.get(net_id) orelse return;
    const cfg_id = ctx.scene.entities.items(.config)[idx].config_id;
    try recordWorldRespawnCooldown(ctx, cfg_id, animal_respawn_ms);
    removeEntityFromSceneAndNotify(ctx, net_id);
}

pub fn onPlayerMove(ctx: LevelPlayContext, x: f32, y: f32) !void {
    const slice = ctx.scene.entities.slice();
    const configs = slice.items(.config);
    const positions = slice.items(.position);
    var dirty_entered = false;

    // 1) Check currently entered LevelPlays for leaving LeaveRadius
    if (ctx.quest_comp.info.progress.entered_level_plays.len > 0) {
        var idx: usize = 0;
        while (idx < ctx.quest_comp.info.progress.entered_level_plays.len) {
            const lpid = ctx.quest_comp.info.progress.entered_level_plays[idx];
            const lp_opt = ctx.assets.tables.level_play_data.getDataById(lpid);
            var still_inside = false;
            if (lp_opt) |lp| {
                if (matchesSceneInstance(ctx.assets, lp, ctx.scene.instance_id)) {
                    const enter_r: f32 = if (lp.Data.EnterRadius > 0) @floatFromInt(lp.Data.EnterRadius) else 3000.0;
                    const leave_r: f32 = if (lp.Data.LeaveRadius > 0) @floatFromInt(lp.Data.LeaveRadius) else enter_r + 1500.0;
                    for (configs, positions) |cfg, pos_opt| {
                        if (cfg.config_id != lp.Data.LevelPlayEntityId) continue;
                        const pos = pos_opt orelse continue;
                        const dx = pos.location[0] - x;
                        const dy = pos.location[1] - y;
                        if (dx * dx + dy * dy <= leave_r * leave_r) {
                            still_inside = true;
                        }
                        break;
                    }
                }
            }
            if (!still_inside) {
                if (try QuestInfo.removeIntValue(&ctx.quest_comp.info.progress.entered_level_plays, ctx.alloc.gpa, lpid)) {
                    dirty_entered = true;
                    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.LeaveLevelPlayNotify{ .Id = lpid }) catch {};
                    continue;
                }
            }
            idx += 1;
        }
    }

    // 2) Snapshot nearby LevelPlay candidate entities before running actions that may spawn/remove scene entities
    const ProximityCandidate = struct { cfg_id: i32, dist2: f32 };
    var candidates: std.ArrayList(ProximityCandidate) = .empty;
    for (configs, positions) |cfg, pos_opt| {
        const cfg_id = cfg.config_id;
        if (cfg_id <= 0) continue;
        const pos = pos_opt orelse continue;
        const dx = pos.location[0] - x;
        const dy = pos.location[1] - y;
        const d2 = dx * dx + dy * dy;
        if (d2 > 15000.0 * 15000.0) continue;
        const cfg_id32 = std.math.cast(i32, cfg_id) orelse continue;
        if (ctx.assets.indexes.levelPlayIndicesForEntity(cfg_id32).len == 0) continue;
        try candidates.append(ctx.alloc.arena, .{ .cfg_id = cfg_id32, .dist2 = d2 });
    }

    for (candidates.items) |cand| {
        for (ctx.assets.indexes.levelPlayIndicesForEntity(cand.cfg_id)) |lp_idx| {
            const lp = ctx.assets.tables.level_play_data.items[lp_idx];
            if (!matchesSceneInstance(ctx.assets, lp, ctx.scene.instance_id)) continue;
            if (std.mem.eql(u8, lp.Data.Type, "Quest")) continue;
            if (QuestInfo.containsInt(ctx.quest_comp.info.completed_level_plays, lp.LevelPlayId)) continue;
            const enter_r: f32 = if (lp.Data.EnterRadius > 0) @floatFromInt(lp.Data.EnterRadius) else 3000.0;
            if (cand.dist2 > enter_r * enter_r) continue;
            if (QuestInfo.containsInt(ctx.quest_comp.info.progress.entered_level_plays, lp.LevelPlayId)) continue;

            if (try QuestInfo.appendUniqueInt(&ctx.quest_comp.info.progress.entered_level_plays, ctx.alloc.gpa, lp.LevelPlayId)) {
                dirty_entered = true;
            }
            ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.LevelPlayStateNotify{
                .Id = lp.LevelPlayId,
                .State = 2,
                .CompleteNumber = 0,
            }) catch {};
            ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.EnterLevelPlayNotify{
                .Id = lp.LevelPlayId,
                .State = 2,
                .CanGetReward = true,
            }) catch {};

            const nodes = ctx.assets.level_play_nodes.getOwnerNodes(ctx.io, lp.LevelPlayId) catch continue;
            var reach_area_done = false;
            for (nodes) |node| {
                if (node.data != .object) continue;
                const ntype = jsonStr(node.data.object.get("Type")) orelse continue;
                if (!std.mem.eql(u8, ntype, "ChildQuest")) continue;
                runLevelPlayActions(ctx, lp.LevelPlayId, node.id, node.data.object.get("EnterActions"));
                const cond_val = node.data.object.get("Condition") orelse continue;
                if (cond_val != .object) continue;
                const ctype = jsonStr(cond_val.object.get("Type")) orelse continue;
                if (std.mem.eql(u8, ctype, "ReachArea") and isPlayerWithinReachArea(ctx, cond_val.object, x, y)) {
                    runLevelPlayActions(ctx, lp.LevelPlayId, node.id, node.data.object.get("FinishActions"));
                    reach_area_done = true;
                }
            }
            if (reach_area_done) {
                try completeLevelPlay(ctx, lp.LevelPlayId);
                dirty_entered = false;
            }
        }
    }

    if (dirty_entered) {
        ctx.quest_comp.save(ctx.fs, ctx.alloc.arena) catch {};
    }
}

fn makeTestInstanceDungeon(id: i32, map_config_id: i32) Assets.DataTables.InstanceDungeon {
    return .{
        .Id = id,
        .MapConfigId = map_config_id,
        .MapName = "",
        .InstType = 0,
        .InstSubType = 0,
        .WorldDungeonSubType = 0,
        .OnlineType = 0,
        .CustomTypes = &.{},
        .MiniMapId = 0,
        .FightFormationId = 0,
        .RoleTypeList = .{ 0, 0, 0 },
        .TrialRoleInfo = .{ 0, 0, 0 },
        .ReviveId = 0,
        .BornPosition = .{ 0, 0, 0 },
        .BornRotation = .{ 0, 0, 0 },
        .RecoverWorldLocation = &.{},
        .ExitDungeonConfirmId = &.{},
        .EntranceEntities = &.{},
        .ExitEntities = &.{},
        .FirstRewardId = 0,
        .RewardId = 0,
        .RepeatRewardId = 0,
        .ExchangeRewardId = 0,
        .DropVisionLimit = 0,
        .SharedTreasureGroup = &.{},
        .EnterControlId = 0,
        .EnterCondition = &.{},
        .EntityLevel = 0,
        .AutoLeaveTime = 0,
        .VerifyCreatureGen = false,
    };
}

test "matchesSceneInstance respects InstanceId override and rejects cross-map LevelId" {
    const testing = std.testing;
    const dungeons = [_]Assets.DataTables.InstanceDungeon{
        makeTestInstanceDungeon(8, 8),
        makeTestInstanceDungeon(904, 904),
        makeTestInstanceDungeon(909, 909),
        makeTestInstanceDungeon(1500, 8),
    };
    var inst_map: std.array_hash_map.Auto(i32, usize) = .empty;
    defer inst_map.deinit(testing.allocator);
    for (dungeons, 0..) |d, i| try inst_map.put(testing.allocator, d.Id, i);

    var assets: Assets = undefined;
    assets.tables.instance_dungeon = .{ .items = &dungeons, .index = inst_map };

    const lp_909: Assets.DataTables.LevelPlayData = .{
        .LevelPlayId = 109000078,
        .Data = .{ .LevelId = 909, .InstanceId = 0, .Type = "Quest" },
    };
    try testing.expect(!matchesSceneInstance(&assets, lp_909, 904));
    try testing.expect(!matchesSceneInstance(&assets, lp_909, 8));
    try testing.expect(matchesSceneInstance(&assets, lp_909, 909));

    // InstanceId overrides LevelId when both are set.
    const lp_inst_override: Assets.DataTables.LevelPlayData = .{
        .LevelPlayId = 16000004,
        .Data = .{ .LevelId = 100, .InstanceId = 8, .Type = "Challenge" },
    };
    try testing.expect(matchesSceneInstance(&assets, lp_inst_override, 8));
    try testing.expect(!matchesSceneInstance(&assets, lp_inst_override, 100));
    try testing.expect(matchesSceneInstance(&assets, lp_inst_override, 1500));
}
