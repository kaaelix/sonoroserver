const std = @import("std");
const pb = @import("proto").pb;
const EventQueue = @import("../../logic/EventQueue.zig");
const Scene = @import("../../logic/Scene.zig");
const Assets = @import("../../data/Assets.zig");
const Entity = @import("../../logic/Scene.zig").Entity;
const FileSystem = @import("common").FileSystem;
const Connection = @import("../../network/Connection.zig");
const mem = @import("../../mem.zig");
const ConfigComponent = @import("../../logic/component/entity/ConfigComponent.zig");
const PlayerRoleComponent = @import("../../logic/component/player/PlayerRoleComponent.zig");
const PlayerWeaponComponent = @import("../../logic/component/player/PlayerWeaponComponent.zig");
const PlayerEchoComponent = @import("../../logic/component/player/PlayerEchoComponent.zig");
const PlayerBasicComponent = @import("../../logic/component/player/PlayerBasicComponent.zig");
const ProgressInfo = @import("../../fs/ProgressInfo.zig");
const destructible = @import("../helpers/destructible.zig");
const RoleHelper = @import("../../logic/helpers/role.zig");
const RoleEntityTemplates = @import("../../logic/templates/RoleEntityTemplates.zig");
const entity_attributes = @import("../../logic/helpers/entity_attributes.zig");
const gameplay_tags = @import("../../logic/helpers/gameplay_tags.zig");
const BuffTimerScheduler = @import("../../logic/schedulers/BuffTimerScheduler.zig");
const respond = @import("../commands.zig").respond;

const EntitySpawnBase = struct {
    config_type: ConfigComponent.ConfigType,
    entity_type: ConfigComponent.EntityType,
    entity_state: ConfigComponent.EntityState,
};

const FROZEN_BUFFS: *const [1]i64 = &.{10010001};
const TUNE_BROKEN_BUFFS: *const [5]i64 = &.{ 3161, 3162, 3163, 3164, 9000000000 };
const buffListFromIds = @import("../../data/tables/Buff.zig").buffListFromIds;

const LevelEntityConfig = Assets.DataTables.LevelEntityConfig;
const BlueprintConfig = Assets.DataTables.BlueprintConfig;
const TemplateConfig = Assets.DataTables.TemplateConfig;
const Components = @import("../../data/tables/entity_components/Components.zig");
const BaseInfo = @import("../../data/tables/entity_components/BaseInfo.zig");
const EntityLogic = @FieldType(BlueprintConfig, "EntityLogic");
const repeat_boss_tag_id = 1639442014;
const story_boss_tag_id = -1276461747;

const SpawnConfig = struct {
    entity_config: LevelEntityConfig,
    blueprint_config: BlueprintConfig,
    template_config: TemplateConfig,
    score: i32,
};

fn spawnBaseFromLogic(entity_logic: EntityLogic) ?EntitySpawnBase {
    return switch (entity_logic) {
        .Item => .{ .config_type = .level, .entity_type = .scene_item, .entity_state = .default },
        .Animal => .{ .config_type = .level, .entity_type = .animal, .entity_state = .default },
        .Monster => .{ .config_type = .level, .entity_type = .monster, .entity_state = .born },
        .Vehicle => .{ .config_type = .level, .entity_type = .vehicle, .entity_state = .default },
        .Npc => .{ .config_type = .level, .entity_type = .npc, .entity_state = .default },
        .Vision => .{ .config_type = .level, .entity_type = .vision, .entity_state = .default },
        .Custom => .{ .config_type = .level, .entity_type = .custom, .entity_state = .default },
        .SimpleCombat => .{ .config_type = .level, .entity_type = .monster, .entity_state = .default },
        .ClientOnly, .ServerOnly => null,
    };
}

fn buildMonsterPassiveGaSkill(
    assets: *const Assets,
    model_id: i32,
    gpa: std.mem.Allocator,
) !?Entity.PassiveGaSkillComponent {
    if (model_id <= 0) return null;
    const model = assets.tables.model_config_preload.getDataById(model_id) orelse return null;
    if (model.ActorClassPath.len == 0) return null;

    var comp: Entity.PassiveGaSkillComponent = .{};
    errdefer comp.deinit(gpa);
    if (assets.getDb()) |db| {
        var scratch = std.heap.ArenaAllocator.init(gpa);
        defer scratch.deinit();
        const skill_ids = try db.querySkillIdsByBlueprint(model.ActorClassPath, scratch.allocator());
        for (skill_ids) |sid| {
            _ = try comp.addSkill(gpa, sid);
        }
    } else {
        for (assets.tables.entity_skill_preload.items) |entry| {
            if (entry.ActorBlueprint.len == 0) continue;
            if (!std.mem.eql(u8, entry.ActorBlueprint, model.ActorClassPath)) continue;
            _ = try comp.addSkill(gpa, entry.SkillId);
        }
    }
    if (comp.skill_component_pb.len == 0) return null;
    return comp;
}

fn useAiRuntime(entity_type: ConfigComponent.EntityType) bool {
    return switch (entity_type) {
        .animal, .monster, .npc, .vision => true,
        else => false,
    };
}

fn hasUsableFsm(assets: *const Assets, components: *const Components) bool {
    if (components.AiComponent) |ai_comp| return Entity.FsmComponent.hasUsableAiBaseId(ai_comp.AiId, assets);
    return false;
}

fn useRepeatBossStartup(ai_id: ?i32, assets: *const Assets) bool {
    const id = ai_id orelse return false;
    const ai_base = assets.tables.ai_base.getDataById(id) orelse return false;
    const config = assets.tables.ai_state_machine_config.getDataById(ai_base.StateMachine) orelse return false;

    for (config.StateMachineJson.Nodes) |node| {
        for (node.Transitions) |repeat_transition| {
            if (!transitionHasTag(repeat_transition, repeat_boss_tag_id)) continue;

            for (node.Transitions) |story_transition| {
                if (story_transition.From == repeat_transition.From and
                    transitionHasTag(story_transition, story_boss_tag_id)) return true;
            }
        }
    }

    return false;
}

fn initialTagComponent(
    components: *const Components,
    tag_remaps: *const Assets.DataTables.GameplayTagRemapTable,
    repeat_boss_startup: bool,
    gpa: std.mem.Allocator,
) !Entity.TagComponent {
    var result: Entity.TagComponent = .{};
    errdefer result.deinit(gpa);

    const monster = components.MonsterComponent orelse return result;
    const has_combat_attrs = hasCombatAttributes(components);
    if (monster.InitGasTag) |startup_tags| {
        for (startup_tags) |tag_name| {
            if (has_combat_attrs and std.mem.eql(u8, tag_name, "功能.逻辑状态标识.隐藏状态条")) continue;
            var tag_id = try gameplay_tags.idFromName(tag_remaps, tag_name);
            if (tag_id == 0) continue;
            if (repeat_boss_startup and tag_id == story_boss_tag_id) tag_id = repeat_boss_tag_id;
            _ = try result.adjustGameplayTagCount(gpa, tag_id, 1);
        }
    }

    if (has_combat_attrs) {
        const fight_tag_id = gameplay_tags.idFromName(tag_remaps, "行为状态.逻辑状态.进入战斗") catch 0;
        if (fight_tag_id != 0) {
            _ = try result.adjustGameplayTagCount(gpa, fight_tag_id, 1);
            try result.addCommonTag(gpa, fight_tag_id);
        }
        if (components.AttributeComponent) |attr| {
            if ((attr.RageModeId orelse 0) > 0) {
                const rage_tag_id = gameplay_tags.idFromName(tag_remaps, "功能.功能制作.狂暴生效") catch 0;
                if (rage_tag_id != 0) {
                    _ = try result.adjustGameplayTagCount(gpa, rage_tag_id, 1);
                    try result.addCommonTag(gpa, rage_tag_id);
                }
            }
        }
    }

    return result;
}

fn transitionHasTag(transition: Assets.DataTables.AiStateMachineConfig.StateMachineTransition, tag_id: i32) bool {
    for (transition.Conditions) |condition| {
        if (condition.CondTag) |tag| {
            if (tag.TagId == tag_id) return true;
        }
    }

    return false;
}

fn hasCombatAttributes(components: *const Components) bool {
    if (components.AttributeComponent) |attr_comp| return !(attr_comp.Disabled orelse false);
    return false;
}

fn spawnCandidateScore(assets: *const Assets, scene_map_id: i32, entity_config: *const LevelEntityConfig, blueprint_config: *const BlueprintConfig) i32 {
    var score: i32 = 0;
    const spawn_base = spawnBaseFromLogic(blueprint_config.EntityLogic);

    if (entity_config.MapId == scene_map_id) score += 10000;

    if (spawn_base) |base| {
        if (useAiRuntime(base.entity_type)) score += 1000;
        if (base.entity_type == .monster) score += 3000;
    }

    if (hasUsableFsm(assets, &entity_config.ComponentsData)) score += 2500;
    if (hasCombatAttributes(&entity_config.ComponentsData)) score += 1500;
    if (!entity_config.IsHidden) score += 100;
    if (!entity_config.InSleep) score += 50;

    return score;
}

fn selectSpawnConfig(assets: *const Assets, entity_id: i64, scene_map_id: i32, source_map_id: ?i32) ?SpawnConfig {
    var best: ?SpawnConfig = null;

    for (assets.indexes.levelEntityIndicesById(entity_id)) |idx| {
        const cfg = (assets.getLevelEntityByIdx(idx) orelse continue).*;
        if (source_map_id) |map_id| {
            if (cfg.MapId != map_id) continue;
        }

        const blueprint_config = (assets.findBlueprintConfig(cfg.BlueprintType) orelse continue).*;
        const template_config = (assets.findTemplateConfig(cfg.BlueprintType) orelse continue).*;
        var entity_config = cfg;
        template_config.ComponentsData.mergeInto(&entity_config.ComponentsData);

        const score = spawnCandidateScore(assets, source_map_id orelse scene_map_id, &entity_config, &blueprint_config);
        const is_better = if (best) |b| score > b.score else true;
        if (is_better) {
            best = .{
                .entity_config = entity_config,
                .blueprint_config = blueprint_config,
                .template_config = template_config,
                .score = score,
            };
        }
    }

    return best;
}

fn templateCamp(template_config: *const TemplateConfig) ?i32 {
    if (template_config.ComponentsData.BaseInfoComponent) |base_info| return base_info.Camp;
    return null;
}

fn spawnCamp(base_info: BaseInfo, template_config: *const TemplateConfig, spawn_base: EntitySpawnBase) i32 {
    const base_camp = base_info.Camp;
    const tpl_camp = templateCamp(template_config);

    if (spawn_base.entity_type == .monster) {
        if (tpl_camp) |camp| {
            const camp_val = base_camp orelse 0;
            if (camp_val == 0 or camp_val == 2) return camp;
        }
    }

    return base_camp orelse tpl_camp orelse 0;
}

fn gravityForSpawn(base_info: BaseInfo, rotation: [3]f32) [3]f32 {
    const gravity_config = base_info.GravityConfig orelse return .{ 0, 0, -1 };
    const direction = gravity_config.GravityDirection orelse return .{ 0, 0, -1 };

    if (direction.Direction) |vector| {
        return .{ vector.X, vector.Y, vector.Z };
    }

    if (direction.WorldAxis) |axis| {
        if (std.mem.eql(u8, axis, "PositiveX")) return .{ 1, 0, 0 };
        if (std.mem.eql(u8, axis, "NegativeX")) return .{ -1, 0, 0 };
        if (std.mem.eql(u8, axis, "PositiveY")) return .{ 0, 1, 0 };
        if (std.mem.eql(u8, axis, "NegativeY")) return .{ 0, -1, 0 };
        if (std.mem.eql(u8, axis, "PositiveZ")) return .{ 0, 0, 1 };
        if (std.mem.eql(u8, axis, "NegativeZ")) return .{ 0, 0, -1 };
    }

    if (direction.Type) |kind| {
        if (std.mem.eql(u8, kind, "SelfRotation")) {
            const to_radians = std.math.pi / 180.0;
            const roll = rotation[0] * to_radians;
            const pitch = rotation[1] * to_radians;
            const yaw = rotation[2] * to_radians;
            const cr = @cos(roll);
            const sr = @sin(roll);
            const cp = @cos(pitch);
            const sp = @sin(pitch);
            const cy = @cos(yaw);
            const sy = @sin(yaw);
            // Match GravityUtils.GetGravityDirectByEntityData: negative actor up vector.
            return .{
                cp * sr * cy + sp * sy,
                cp * sr * sy - sp * cy,
                -cp * cr,
            };
        }
    }

    return .{ 0, 0, -1 };
}

fn appendConfiguredBuffIds(buff_ids: *std.ArrayList(i64), components: *const Components, gpa: std.mem.Allocator) !void {
    const attr = components.AttributeComponent orelse return;
    if (attr.AppendBuffIds) |configured_ids| {
        try buff_ids.appendSlice(gpa, configured_ids);
    }
    if ((attr.RageModeId orelse 0) > 0) {
        const rage_buff_id: i64 = 70000045;
        if (std.mem.indexOfScalar(i64, buff_ids.items, rage_buff_id) == null) {
            try buff_ids.append(gpa, rage_buff_id);
        }
    }
}

fn publishDebugSpawn(
    scene: *Scene,
    fs: *FileSystem,
    assets: *const Assets,
    conn: *Connection,
    alloc: mem.Alloc,
    io: std.Io,
    entity: Scene.Entity,
    config_id: i64,
    source_map_id: i32,
    repeat_boss: bool,
    components: *const Components,
    is_frozen: bool,
    is_tune_broken: bool,
    route_patch_sent: *bool,
) !void {
    try scene.registerDebugSpawnRoute(
        alloc.gpa,
        entity.net_id,
        config_id,
        source_map_id,
        repeat_boss,
    );

    var buff_ids: std.ArrayList(i64) = .empty;
    defer buff_ids.deinit(alloc.gpa);
    try appendConfiguredBuffIds(&buff_ids, components, alloc.gpa);
    if (is_tune_broken) try buff_ids.appendSlice(alloc.gpa, TUNE_BROKEN_BUFFS);
    if (is_frozen) try buff_ids.appendSlice(alloc.gpa, FROZEN_BUFFS);
    if (buff_ids.items.len > 0) {
        const slice = scene.entities.slice();
        if (slice.items(.buffs)[entity.index]) |*buffs| {
            var born_buffs = try buffListFromIds(alloc.gpa, buff_ids.items);
            defer born_buffs.deinit(alloc.gpa);
            buffs.fight_buff_infos = try scene.buildFightBuffInfos(born_buffs, entity.net_id, alloc.gpa);
        }
        try scene.saveEntity(fs, alloc.gpa, entity);
    }

    var entity_pbs: std.ArrayList(pb.EntityPb) = try .initCapacity(alloc.gpa, 1);
    defer entity_pbs.deinit(alloc.gpa);
    const storage = scene.entities.get(entity.index);
    entity_pbs.appendAssumeCapacity(try storage.entityToProto(entity.net_id, alloc, assets));

    try conn.push(alloc.gpa, io, pb.JSPatchNotify{
        .Content = try std.fmt.allocPrint(
            alloc.arena,
            "globalThis.__akiSetEntitySourceMap?.({d},{d},{s},{d});",
            .{ config_id, source_map_id, if (repeat_boss) "true" else "false", entity.net_id },
        ),
    });
    route_patch_sent.* = true;

    try conn.push(alloc.gpa, io, pb.EntityAddNotify{ .EntityPbs = entity_pbs });
}

fn removeClientDebugSpawnOwner(
    conn: *Connection,
    alloc: mem.Alloc,
    io: std.Io,
    config_id: i64,
    net_id: i64,
) void {
    const content = std.fmt.allocPrint(
        alloc.arena,
        "globalThis.__akiRemoveEntitySourceOwner?.({d},{d});",
        .{ config_id, net_id },
    ) catch return;
    conn.pushBestEffort(alloc.gpa, io, pb.JSPatchNotify{ .Content = content });
}

pub const spawn = struct {
    pub const alias = "s";
    pub const description = "spawns an entity.\nusage: spawn [entity_id] [freeze?] [tune_break?]";
    pub fn call(
        events: *EventQueue,
        scene: *Scene,
        fs: *FileSystem,
        assets: *const Assets,
        conn: *Connection,
        basic_comp: *const PlayerBasicComponent,
        alloc: mem.Alloc,
        io: std.Io,
        entity_id: i64,
        is_frozen: ?bool,
        is_tune_broken: ?bool,
    ) !void {
        try spawnWithLocation(events, scene, fs, assets, conn, basic_comp, alloc, io, entity_id, is_frozen, is_tune_broken, null, null, false);
    }

    pub fn spawnWithLocation(
        events: *EventQueue,
        scene: *Scene,
        fs: *FileSystem,
        assets: *const Assets,
        conn: *Connection,
        basic_comp: *const PlayerBasicComponent,
        alloc: mem.Alloc,
        io: std.Io,
        entity_id: i64,
        is_frozen: ?bool,
        is_tune_broken: ?bool,
        source_map_id: ?i32,
        location_override: ?[3]f32,
        silent: bool,
    ) !void {
        _ = try spawnWithLocationInternal(
            events,
            scene,
            fs,
            assets,
            conn,
            basic_comp,
            alloc,
            io,
            entity_id,
            is_frozen,
            is_tune_broken,
            source_map_id,
            location_override,
            silent,
            null,
            false,
            false,
        );
    }

    pub fn spawnWithLocationAndProgress(
        events: *EventQueue,
        scene: *Scene,
        fs: *FileSystem,
        assets: *const Assets,
        conn: *Connection,
        basic_comp: *const PlayerBasicComponent,
        alloc: mem.Alloc,
        io: std.Io,
        entity_id: i64,
        is_frozen: ?bool,
        is_tune_broken: ?bool,
        source_map_id: ?i32,
        location_override: ?[3]f32,
        silent: bool,
        progress_info: ProgressInfo,
        respect_sleep_hidden: bool,
        defer_publish: bool,
    ) !?Entity {
        return try spawnWithLocationInternal(
            events,
            scene,
            fs,
            assets,
            conn,
            basic_comp,
            alloc,
            io,
            entity_id,
            is_frozen,
            is_tune_broken,
            source_map_id,
            location_override,
            silent,
            progress_info,
            respect_sleep_hidden,
            defer_publish,
        );
    }

    pub fn publishWorldSpawn(
        scene: *Scene,
        fs: *FileSystem,
        assets: *const Assets,
        conn: *Connection,
        alloc: mem.Alloc,
        io: std.Io,
        entity: Entity,
        entity_id: i64,
        source_map_id: i32,
    ) !void {
        const selected = selectSpawnConfig(assets, entity_id, scene.instance_id, source_map_id) orelse return;
        const ai_id = if (selected.entity_config.ComponentsData.AiComponent) |comp| comp.AiId else null;
        const repeat_boss = useRepeatBossStartup(ai_id, assets);
        var route_patch_sent = false;
        publishDebugSpawn(
            scene,
            fs,
            assets,
            conn,
            alloc,
            io,
            entity,
            entity_id,
            selected.entity_config.MapId,
            repeat_boss,
            &selected.entity_config.ComponentsData,
            false,
            false,
            &route_patch_sent,
        ) catch |publish_err| {
            if (route_patch_sent) removeClientDebugSpawnOwner(conn, alloc, io, entity_id, entity.net_id);
            scene.rollbackSpawn(alloc.gpa, fs, entity.net_id) catch |rollback_err| return rollback_err;
            return publish_err;
        };
    }

    fn spawnWithLocationInternal(
        events: *EventQueue,
        scene: *Scene,
        fs: *FileSystem,
        assets: *const Assets,
        conn: *Connection,
        basic_comp: *const PlayerBasicComponent,
        alloc: mem.Alloc,
        io: std.Io,
        entity_id: i64,
        is_frozen: ?bool,
        is_tune_broken: ?bool,
        source_map_id: ?i32,
        location_override: ?[3]f32,
        silent: bool,
        progress_info: ?ProgressInfo,
        respect_sleep_hidden: bool,
        defer_publish: bool,
    ) !?Entity {
        const selected = selectSpawnConfig(assets, entity_id, scene.instance_id, source_map_id) orelse {
            if (!silent) {
                if (source_map_id) |map_id| {
                    try respond(events, alloc.arena, "{d} couldn't be spawned, no LevelEntityConfig row for map {d}", .{ entity_id, map_id });
                } else {
                    try respond(events, alloc.arena, "{d} couldn't be spawned, couldn't find it in LevelEntityConfig", .{entity_id});
                }
            }
            return null;
        };
        const entity_config = selected.entity_config;
        const blueprint_config = selected.blueprint_config;
        const template_config = selected.template_config;

        const spawn_base = spawnBaseFromLogic(blueprint_config.EntityLogic) orelse {
            if (!silent) {
                try respond(events, alloc.arena, "{d} couldn't be spawned, unhandled entity logic: {s}", .{ entity_id, @tagName(blueprint_config.EntityLogic) });
            }
            return null;
        };
        const components_data = entity_config.ComponentsData;

        const base_info = components_data.BaseInfoComponent orelse @import("../../data/tables/entity_components/BaseInfo.zig"){};

        const ai_comp = components_data.AiComponent;
        const ai_id = if (ai_comp) |comp| comp.AiId else null;
        const repeat_boss_startup = useRepeatBossStartup(ai_id, assets);
        if (scene.debugSpawnRoute(entity_id)) |route| {
            if (!route.matches(entity_config.MapId, repeat_boss_startup)) {
                try respond(
                    events,
                    alloc.arena,
                    "{d} couldn't be spawned, active instances use map {d} with repeat_boss={s}",
                    .{ entity_id, route.source_map_id, if (route.repeat_boss) "true" else "false" },
                );
                return null;
            }
        }
        const weapon_id = if (ai_comp) |comp| parseOptionalInt(comp.WeaponId) else 0;
        const final_camp = spawnCamp(base_info, &template_config, spawn_base);
        const gravity = gravityForSpawn(base_info, Assets.AssetIndexes.entityWorldRot(&entity_config));
        const model_id = if (components_data.ModelComponent) |model| model.ModelType.ModelId else 0;

        const use_ai_runtime = useAiRuntime(spawn_base.entity_type);
        var fsm_component: ?Entity.FsmComponent = null;
        var attribute_component: ?Entity.AttributeComponent = null;
        var tag_component: ?Entity.TagComponent = null;
        var part_component: ?Entity.PartComponent = null;
        var state_tag_component: ?Entity.StateTagComponent = null;
        var passive_ga_skill_component: ?Entity.PassiveGaSkillComponent = null;
        if (use_ai_runtime) {
            fsm_component = Entity.FsmComponent.fromAiBaseId(ai_id, assets);
            attribute_component = (try entity_attributes.createCombatAttributes(
                assets,
                &components_data,
                blueprint_config.EntityLogic,
                entity_config.AreaId,
                basic_comp.info.attributes.cur_world_level,
                alloc,
            ) orelse Entity.AttributeComponent{});
            tag_component = try initialTagComponent(
                &components_data,
                &assets.tables.gameplay_tag_remap,
                repeat_boss_startup,
                alloc.gpa,
            );
            const life_max_index = @intFromEnum(pb.EAttributeType.LifeMax);
            const life_max = if (attribute_component) |attribute|
                if (life_max_index < attribute.attributes.len) attribute.attributes[life_max_index].current else 0
            else
                0;
            part_component = if (assets.tables.character_part_config.getDataById(model_id)) |config|
                try Entity.PartComponent.init(config, life_max, alloc.gpa)
            else
                null;
        }
        if (spawn_base.entity_type == .scene_item or components_data.EntityStateComponent != null) {
            const default_state: ?[]const u8 = if (components_data.EntityStateComponent) |es|
                es.State orelse if (spawn_base.entity_type == .scene_item) "关卡.Common.状态.常态" else null
            else if (spawn_base.entity_type == .scene_item)
                "关卡.Common.状态.常态"
            else
                null;
            const state_tag_id: i32 = if (default_state) |state_name|
                gameplay_tags.idFromName(&assets.tables.gameplay_tag_remap, state_name) catch 0
            else
                0;
            state_tag_component = Entity.StateTagComponent{ .state_tag_id = state_tag_id };
            if (tag_component == null) {
                tag_component = try initialTagComponent(
                    &components_data,
                    &assets.tables.gameplay_tag_remap,
                    repeat_boss_startup,
                    alloc.gpa,
                );
            }
            if (state_tag_id != 0) {
                if (tag_component) |*tags| {
                    _ = try tags.adjustGameplayTagCount(alloc.gpa, state_tag_id, 1);
                    try tags.addCommonTag(alloc.gpa, state_tag_id);
                }
            }
        }
        errdefer if (fsm_component) |*fsm| fsm.deinit(alloc.gpa);
        errdefer if (tag_component) |*tags| tags.deinit(alloc.gpa);
        errdefer if (part_component) |*part| part.deinit(alloc.gpa);
        var interact_state_component: ?Entity.InteractStateComponent = null;
        if (components_data.InteractComponent) |interact_config| {
            if (!(interact_config.Disabled orelse false)) {
                interact_state_component = try Entity.InteractStateComponent.fromConfig(
                    alloc.gpa,
                    interact_config,
                    scene.player_id,
                    scene.instance_id,
                    @intCast(entity_id),
                );
            }
        }
        errdefer if (interact_state_component) |*interact| interact.deinit(alloc.gpa);
        if (part_component) |*part| {
            const tags = if (tag_component) |*component| component else null;
            try part.syncActiveTags(tags, alloc.gpa);
        }

        const initially_hidden = respect_sleep_hidden and (entity_config.IsHidden or entity_config.InSleep);
        if (spawn_base.entity_type == .monster) {
            passive_ga_skill_component = try buildMonsterPassiveGaSkill(assets, model_id, alloc.gpa);
        }
        errdefer if (passive_ga_skill_component) |*p| p.deinit(alloc.gpa);

        var monster_ai_component: ?Entity.MonsterAiComponent = null;
        if (use_ai_runtime) {
            var hatred_id: i64 = 0;
            var perception_ids: [8]i32 = @splat(0);
            var perception_len: u8 = 0;
            if (ai_id) |aid| {
                if (assets.tables.ai_base.getDataById(aid)) |ab| {
                    if (ab.SubBehaviorConfigs) |sbc| {
                        if (sbc.AiHate) |hate_str| {
                            hatred_id = std.fmt.parseInt(i64, hate_str, 10) catch 0;
                        }
                        if (sbc.AiSense) |sense_str| {
                            const sg_id = std.fmt.parseInt(i32, sense_str, 10) catch 0;
                            if (sg_id > 0) {
                                if (assets.tables.ai_sense_group.getDataById(sg_id)) |sg| {
                                    for (sg.AiSenseIds) |sid| {
                                        if (perception_len >= perception_ids.len) break;
                                        if (sid != 0) {
                                            perception_ids[perception_len] = sid;
                                            perception_len += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            monster_ai_component = .{
                .weapon_id = weapon_id,
                .hatred_group_id = 0,
                .ai_team_init_id = 100,
                .combat_message_id = 1,
                .hatred_id = hatred_id,
                .perception_ids = perception_ids,
                .perception_len = perception_len,
            };
        }

        const entity = try scene.spawn(alloc.gpa, fs, .{
            Entity.ConfigComponent{
                .camp = final_camp,
                .config_id = @intCast(entity_id),
                .config_type = spawn_base.config_type,
                .entity_type = spawn_base.entity_type,
                .state = spawn_base.entity_state,
            },
            if (!initially_hidden) Entity.ActorVisibleMarker{} else null,
            if (!initially_hidden) Entity.VisibleMarker{} else null,
            Entity.PositionComponent{
                .location = if (location_override) |loc| loc else scene.instance.players[0].location,
                .rotation = if (location_override != null) Assets.AssetIndexes.entityWorldRot(&entity_config) else scene.instance.players[0].rotation,
            },
            Entity.FightBuffComponent{},
            attribute_component,
            if (use_ai_runtime) Entity.LogicStateComponent{} else null,
            monster_ai_component,
            if (use_ai_runtime) Entity.CharacterAttachComponent{} else null,
            if (spawn_base.entity_type == .vehicle or components_data.hasExtra(.VehicleComponent)) Entity.VehicleComponent{ .source = .VehicleSourceNone } else null,
            tag_component,
            part_component,
            fsm_component,
            state_tag_component,
            passive_ga_skill_component,
        });
        passive_ga_skill_component = null;
        scene.entities.items(.world_item)[entity.index].gravity =
            if (base_info.GravityConfig != null) Entity.GravityComponent{ .location = gravity } else null;
        if (components_data.GravityFlipComponent) |gfc| {
            if (gfc.DefaultGravity) |dg| {
                scene.entities.items(.world_item)[entity.index].gravity_flip = Components.gravityFlipFromDefault(dg);
            }
        }
        if (components_data.TeleControl2 != null) {
            scene.entities.items(.world_item)[entity.index].be_controlled_present = true;
        }
        if (blueprint_config.EntityType == .Drop or blueprint_config.EntityType == .DropItem) {
            scene.entities.items(.world_item)[entity.index].drop_present = true;
        }
        if (spawn_base.entity_type == .npc) {
            if (base_info.Category) |cat| {
                if (cat.NpcType) |nt| scene.entities.items(.world_item)[entity.index].sub_entity_type = nt;
            } else if (blueprint_config.EntityType == .PasserbyNpc) {
                scene.entities.items(.world_item)[entity.index].sub_entity_type = 2;
            } else if (std.mem.eql(u8, entity_config.BlueprintType, "SimpleNpc") or std.mem.eql(u8, entity_config.BlueprintType, "SimpleNPc")) {
                scene.entities.items(.world_item)[entity.index].sub_entity_type = 1;
            }
        }
        if (components_data.RangeComponent != null) {
            scene.entities.items(.world_item)[entity.index].range_present = true;
        }
        if (components_data.TriggerComponent != null) {
            scene.entities.items(.world_item)[entity.index].trigger_present = true;
        }
        if (components_data.DestructibleItem) |destructible_config| {
            if (!(destructible_config.Disabled orelse false)) {
                var key_buf: [48]u8 = undefined;
                const saved_durability: i64 = if (progress_info) |info| blk: {
                    const key = std.fmt.bufPrint(&key_buf, "durability:{d}", .{entity_id}) catch break :blk 0;
                    break :blk info.getKeyInt(key);
                } else 0;
                scene.entities.items(.world_item)[entity.index].durability =
                    destructible.initialDurability(destructible_config.Durability, saved_durability);
            }
        }
        if (components_data.LiftComponent) |lc| {
            if (!(lc.Disabled orelse false)) {
                scene.entities.items(.world_item)[entity.index].lift = .{
                    .location = lc.InitialFloor orelse 1,
                };
            }
        }
        if (components_data.AiComponent) |aic| {
            scene.entities.items(.world_item)[entity.index].patrol_dir = false;
            if (aic.WeaponId) |wid_val| {
                if (wid_val == .integer and wid_val.integer > 0) {
                    scene.entities.items(.world_item)[entity.index].monster_weapon_id = std.math.cast(i32, wid_val.integer);
                }
            }
        }
        for ([2][]const @import("../../data/tables/entity_components/Extra.zig").Entry{
            components_data.extra,
            components_data.template_extra,
        }) |extra_slice| {
            for (extra_slice) |entry| {
                switch (entry) {
                    .FanComponent => |fc| {
                        scene.entities.items(.world_item)[entity.index].fan_turns = if (fc.InitCircle) |ic| std.math.cast(i32, ic) orelse 0 else 0;
                    },
                    .HookLockPoint => |hlp| {
                        scene.entities.items(.world_item)[entity.index].hook_disabled = (hlp.DisableAtCreateStage orelse false) or (hlp.Disabled orelse false);
                    },
                    .ConnectorComponent => {
                        scene.entities.items(.world_item)[entity.index].connector_initiated = false;
                    },
                    .ElementalHolderComponent => |eh| {
                        scene.entities.items(.world_item)[entity.index].elemental_holder_enable = !(eh.Disabled orelse false);
                    },
                    .NearbyTrackingComponent => |nt| {
                        scene.entities.items(.world_item)[entity.index].nearby_tracking_enable = nt.IsEnable orelse !(nt.Disabled orelse false);
                    },
                    .FlowerPollutionComponent => {
                        scene.entities.items(.world_item)[entity.index].flower_pollution = true;
                    },
                    .BuffProducerComponent => {
                        scene.entities.items(.world_item)[entity.index].buff_producer_present = true;
                    },
                    .BuffConsumerComponent => {
                        scene.entities.items(.world_item)[entity.index].buff_consumer_present = true;
                    },
                    .SceneItemEventListenerComponent => {
                        scene.entities.items(.world_item)[entity.index].listener_present = true;
                    },
                    .BatchBulletCasterComponent => {
                        scene.entities.items(.world_item)[entity.index].batch_bullet_cast_present = true;
                    },
                    .QuickHackComponent => {
                        scene.entities.items(.world_item)[entity.index].hacking_present = true;
                    },
                    .HackManagementComponent => {
                        scene.entities.items(.world_item)[entity.index].hack_target_present = true;
                    },
                    .PullingObjectFoundation => {
                        scene.entities.items(.world_item)[entity.index].pulling_foundation_present = true;
                    },
                    .SunSpiritGearComponent => {
                        scene.entities.items(.world_item)[entity.index].spirit_gear_present = true;
                    },
                    else => {},
                }
            }
        }
        scene.entities.items(.world_item)[entity.index].interact = interact_state_component;
        interact_state_component = null;
        if (spawn_base.entity_type == .npc) {
            if (std.math.cast(i32, entity_id)) |cid32| {
                scene.entities.items(.world_item)[entity.index].npc_spawn_id = cid32;
            }
        }
        if (spawn_base.entity_type == .scene_item) {
            scene.entities.items(.world_item)[entity.index].scene_item = Entity.SceneItemComponent.fromProgress(
                progress_info orelse .{},
                scene.player_id,
                scene.entities.items(.config)[entity.index].config_id,
            );
        }
        fsm_component = null;
        attribute_component = null;
        tag_component = null;
        part_component = null;
        if (!defer_publish) {
            var route_patch_sent = false;
            publishDebugSpawn(
                scene,
                fs,
                assets,
                conn,
                alloc,
                io,
                entity,
                entity_id,
                entity_config.MapId,
                repeat_boss_startup,
                &components_data,
                is_frozen orelse false,
                is_tune_broken orelse false,
                &route_patch_sent,
            ) catch |publish_err| {
                if (route_patch_sent) removeClientDebugSpawnOwner(conn, alloc, io, entity_id, entity.net_id);
                scene.rollbackSpawn(alloc.gpa, fs, entity.net_id) catch |rollback_err| return rollback_err;
                return publish_err;
            };
        }

        if (!silent) {
            try respond(events, alloc.arena, "spawned {d}, map: {d}, bp: {s}, camp: {d}, ai_id: {any}", .{ entity_id, entity_config.MapId, entity_config.BlueprintType, final_camp, ai_id });
        }
        return entity;
    }
};

pub const spawn_map = struct {
    pub const alias = "sm";
    pub const description = "spawns an entity using an exact source map row.\nusage: spawn_map [entity_id] [source_map_id] [freeze?] [tune_break?]";
    pub fn call(
        events: *EventQueue,
        scene: *Scene,
        fs: *FileSystem,
        assets: *const Assets,
        conn: *Connection,
        basic_comp: *const PlayerBasicComponent,
        alloc: mem.Alloc,
        io: std.Io,
        entity_id: i64,
        source_map_id: i32,
        is_frozen: ?bool,
        is_tune_broken: ?bool,
    ) !void {
        try spawn.spawnWithLocation(events, scene, fs, assets, conn, basic_comp, alloc, io, entity_id, is_frozen, is_tune_broken, source_map_id, null, false);
    }
};

fn parseOptionalInt(value: ?std.json.Value) i32 {
    const raw = value orelse return 0;
    return switch (raw) {
        .integer => |v| std.math.cast(i32, v) orelse 0,
        .float => |v| if (std.math.isFinite(v)) @intFromFloat(std.math.clamp(v, -2147483648.0, 2147483647.0)) else 0,
        .number_string => |v| std.fmt.parseInt(i32, v, 10) catch 0,
        .string => |v| std.fmt.parseInt(i32, v, 10) catch 0,
        else => 0,
    };
}

pub const reset_formation = struct {
    pub const alias = "rf";
    pub const description = "resets the current formation by respawning all role entities.\nusage: reset_formation";
    pub fn call(
        events: *EventQueue,
        scene: *Scene,
        fs: *FileSystem,
        assets: *const Assets,
        role_comp: *PlayerRoleComponent,
        weapon_comp: *PlayerWeaponComponent,
        echo_comp: *PlayerEchoComponent,
        conn: *Connection,
        alloc: mem.Alloc,
        buff_timers: *BuffTimerScheduler,
        io: std.Io,
    ) !void {
        const now_ms = (std.Io.Clock.awake).now(io).toMilliseconds();
        try RoleHelper.resetRoles(
            scene,
            fs,
            assets,
            role_comp,
            weapon_comp,
            echo_comp,
            conn,
            alloc,
            null,
            buff_timers,
            now_ms,
            io,
        );
        try respond(events, alloc.arena, "reset formation successfully", .{});
    }
};
