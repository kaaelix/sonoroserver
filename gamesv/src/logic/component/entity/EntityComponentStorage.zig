const EntityComponentStorage = @This();
const std = @import("std");
const common = @import("common");
const file_util = @import("../../../fs/file_util.zig");
const comp_util = @import("../comp_util.zig");
const mem = @import("../../../mem.zig");
const pb = @import("proto").pb;
const BaseSkinComponent = @import("BaseSkinComponent.zig");
const Assets = @import("../../../data/Assets.zig");

const Allocator = std.mem.Allocator;
const FileSystem = common.FileSystem;

pub const WorldItemState = struct {
    gravity: ?@import("GravityComponent.zig") = null,
    lift: ?@import("LiftComponent.zig") = null,
    move_spline: ?@import("MoveSplineComponent.zig") = null,
    move_to_point: ?@import("MoveToPointComponent.zig") = null,
    scene_item: ?@import("SceneItemComponent.zig") = null,
    interact: ?@import("InteractStateComponent.zig") = null,
    monster_capture: ?@import("MonsterCaptureComponent.zig") = null,
    fan_turns: ?i32 = null,
    gravity_flip: ?i32 = null,
    hook_disabled: ?bool = null,
    connector_initiated: ?bool = null,
    elemental_holder_enable: ?bool = null,
    nearby_tracking_enable: ?bool = null,
    flower_pollution: bool = false,
    range_present: bool = false,
    trigger_present: bool = false,
    patrol_dir: ?bool = null,
    monster_weapon_id: ?i32 = null,
    buff_producer_present: bool = false,
    buff_consumer_present: bool = false,
    listener_present: bool = false,
    batch_bullet_cast_present: bool = false,
    be_controlled_present: bool = false,
    drop_present: bool = false,
    hacking_present: bool = false,
    hack_target_present: bool = false,
    pulling_foundation_present: bool = false,
    spirit_gear_present: bool = false,
    foundation_source_entity_id: i32 = 0,
    sub_entity_type: i32 = 0,
    drop_item_id: i32 = 0,
    drop_item_count: i32 = 1,
    in_range_entity_ids: [8]i64 = @splat(0),
    in_range_entity_len: u8 = 0,
    be_controlled_player_id: ?i64 = null,
    be_controlled_show: bool = false,
    npc_spline_id: i32 = 0,
    npc_spawn_id: i32 = 0,
    hold_hand_target_id: ?i64 = null,
    hold_hand_type: i32 = 0,
    in_range_player_id: i32 = 0,
    trigger_count: i32 = 0,
    exit_trigger_count: i32 = 0,
};

entity_id: struct { net_id: i64 },
config: @import("ConfigComponent.zig"),
position: ?@import("PositionComponent.zig") = null,
world_item: WorldItemState = .{},
player_id: ?@import("PlayerIDComponent.zig") = null,
visible: ?@import("VisibleMarker.zig") = null,
actor_visible: ?@import("ActorVisibleMarker.zig") = null,
direct_control: ?@import("DirectControlMarker.zig") = null,
attribute: ?@import("AttributeComponent.zig") = null,
tag: ?@import("TagComponent.zig") = null,
part: ?@import("PartComponent.zig") = null,
buffs: ?@import("FightBuffComponent.zig") = null,
character_attach: ?@import("CharacterAttachComponent.zig") = null,
concomitant: ?@import("ConcomitantComponent.zig") = null,
equip: ?@import("EquipComponent.zig") = null,
follower: ?@import("FollowerComponent.zig") = null,
vehicle: ?@import("VehicleComponent.zig") = null,
npc_drive_vehicle: ?@import("NpcDriveVehicleComponent.zig") = null,
motor_outlook: ?@import("MotorOutlookComponent.zig") = null,
motor_da_ctx: ?@import("MotorDaCtxComponent.zig") = null,
logic_state: ?@import("LogicStateComponent.zig") = null,
monster_ai: ?@import("MonsterAiComponent.zig") = null,
passive_ga_skill: ?@import("PassiveGaSkillComponent.zig") = null,
player_battle_binder: ?@import("PlayerBattleBinder.zig") = null,
summoner: ?@import("SummonerComponent.zig") = null,
follow_shooter: ?@import("FollowShooterComponent.zig") = null,
follow_entity: ?@import("FollowEntityComponent.zig") = null,
fsm: ?@import("FsmComponent.zig") = null,
vision_skills: ?@import("VisionSkillComponent.zig") = null,
base_skin: ?BaseSkinComponent = null,
calabash_skin: ?@import("CalabashSkinComponent.zig") = null,
weapon_skin: ?@import("WeaponSkinComponent.zig") = null,
ornament: ?@import("OrnamentComponent.zig") = null,
state_tag: ?@import("StateTagComponent.zig") = null,

pub fn fieldIndexForType(comptime Component: type) usize {
    return comptime fieldByType(Component).@"1";
}

pub fn fieldByType(comptime Component: type) struct { std.builtin.Type.StructField, usize } {
    inline for (comptime std.meta.fields(EntityComponentStorage), 0..) |field, i| {
        if (*field.type == Component) return .{ field, i };

        const is_optional = comptime std.meta.activeTag(@typeInfo(field.type)) == .optional;
        if (is_optional and comptime ?*std.meta.Child(field.type) == Component) return .{ field, i };
        if (is_optional and comptime *std.meta.Child(field.type) == Component) return .{ field, i };
    }

    @compileError("requested type is not a valid component: " ++ @typeName(Component));
}

pub fn isComponentOptional(comptime Component: type) bool {
    const T = if (comptime std.meta.activeTag(@typeInfo(Component)) == .optional) std.meta.Child(Component) else Component;

    inline for (comptime std.meta.fields(EntityComponentStorage)) |field| {
        const is_optional = comptime std.meta.activeTag(@typeInfo(field.type)) == .optional;
        const C = if (is_optional) std.meta.Child(field.type) else field.type;

        if (*C == T) return is_optional;
    }

    @compileError("requested type is not a valid component: " ++ @typeName(Component));
}

pub fn load(gpa: Allocator, fs: *FileSystem, player_id: i32, scene_id: i32, entity_id: i64) !EntityComponentStorage {
    const log = std.log.scoped(.load_entity_components);
    var storage: EntityComponentStorage = undefined;

    var arena = std.heap.ArenaAllocator.init(gpa);
    defer arena.deinit();

    inline for (comptime std.meta.fields(EntityComponentStorage)) |field| {
        if (comptime std.mem.eql(u8, field.name, "entity_id")) continue;
        const is_optional = comptime std.meta.activeTag(@typeInfo(field.type)) == .optional;
        const ComponentType = comptime if (is_optional) std.meta.Child(field.type) else field.type;

        if (comptime isTransientComponent(ComponentType)) {
            if (!is_optional) @compileError("transient entity components must be optional");
            @field(storage, field.name) = null;
            continue;
        }

        const component = try file_util.loadZon(
            ComponentType,
            gpa,
            arena.allocator(),
            fs,
            "player/{d}/scene/{d}/entity/{d}/" ++ field.name,
            .{ player_id, scene_id, entity_id },
        );

        if (is_optional) {
            @field(storage, field.name) = component;
        } else {
            @field(storage, field.name) = component orelse {
                log.err(
                    "entity [{d}:{d}:{d}] is missing a required component '" ++ field.name ++ "'",
                    .{ player_id, scene_id, entity_id },
                );

                return error.MissingComponent;
            };
        }
    }

    storage.entity_id = .{ .net_id = entity_id };

    return storage;
}

pub fn save(
    storage: *const EntityComponentStorage,
    arena: Allocator,
    fs: *FileSystem,
    player_id: i32,
    scene_id: i32,
) !void {
    var path_buf: [512]u8 = undefined;

    inline for (comptime std.meta.fields(EntityComponentStorage)) |field| {
        if (comptime std.mem.eql(u8, field.name, "entity_id")) continue;
        const ComponentType = comptime if (std.meta.activeTag(@typeInfo(field.type)) == .optional)
            std.meta.Child(field.type)
        else
            field.type;
        if (comptime isTransientComponent(ComponentType)) continue;

        const path = try std.fmt.bufPrint(
            path_buf[0..],
            "player/{d}/scene/{d}/entity/{d}/" ++ field.name,
            .{ player_id, scene_id, storage.entity_id.net_id },
        );

        try saveComponent(@field(storage, field.name), arena, fs, path);
    }
}

pub fn savePartial(
    storage: *const EntityComponentStorage,
    arena: Allocator,
    fs: *FileSystem,
    player_id: i32,
    scene_id: i32,
    comptime components: []const type,
) !void {
    var path_buf: [512]u8 = undefined;

    inline for (components) |Component| {
        if (comptime isTransientComponent(Component)) continue;
        const field, _ = comptime fieldByType(*Component);

        const path = try std.fmt.bufPrint(
            path_buf[0..],
            "player/{d}/scene/{d}/entity/{d}/" ++ field.name,
            .{ player_id, scene_id, storage.entity_id.net_id },
        );

        try saveComponent(@field(storage, field.name), arena, fs, path);
    }
}

pub fn saveComponent(
    component: anytype,
    arena: Allocator,
    fs: *FileSystem,
    path: []const u8,
) !void {
    const is_optional = comptime std.meta.activeTag(@typeInfo(@TypeOf(component))) == .optional;
    const ComponentType = comptime if (is_optional) std.meta.Child(@TypeOf(component)) else @TypeOf(component);
    if (comptime isTransientComponent(ComponentType)) return;

    if (is_optional) {
        if (component) |comp| {
            try comp_util.saveStruct(fs, comp, path, arena);
        }
    } else {
        try comp_util.saveStruct(fs, component, path, arena);
    }
}

fn isTransientComponent(comptime Component: type) bool {
    if (!@hasDecl(Component, "transient")) return false;
    return Component.transient;
}

pub fn deinit(storage: *EntityComponentStorage, gpa: Allocator) void {
    inline for (comptime std.meta.fields(EntityComponentStorage)) |field| {
        const FieldType = comptime if (std.meta.activeTag(@typeInfo(field.type)) == .optional)
            std.meta.Child(field.type)
        else
            field.type;

        const has_deinit = comptime @hasDecl(FieldType, "deinit");

        if (comptime std.meta.activeTag(@typeInfo(field.type)) == .optional) {
            if (@field(storage, field.name)) |*comp| {
                if (has_deinit) comp.deinit(gpa) else std.zon.parse.free(gpa, comp.*);
            }
        } else {
            std.zon.parse.free(gpa, @field(storage, field.name));
        }
    }
}

pub fn sliceToArrayList(comptime T: type, slice: []T) std.ArrayList(T) {
    return .{
        .items = slice,
        .capacity = slice.len,
    };
}

pub fn entityToProto(
    storage: *const EntityComponentStorage,
    net_id: i64,
    alloc: mem.Alloc,
    assets: *const Assets,
) !pb.EntityPb {
    const base_skin_comp = storage.base_skin orelse BaseSkinComponent{};
    const living_status: ?pb.LivingStatus = if (storage.attribute) |attr_comp| blk: {
        const life_idx = @intFromEnum(pb.EAttributeType.Life);
        const max_idx = @intFromEnum(pb.EAttributeType.LifeMax);
        const life_max = attr_comp.attributes[max_idx].current;
        if (life_max <= 0) break :blk null;
        break :blk if (attr_comp.attributes[life_idx].current <= 0) .Dead else .Alive;
    } else null;
    const resolved_sub_type: i32 = if (storage.world_item.sub_entity_type != 0)
        storage.world_item.sub_entity_type
    else if (storage.config.entity_type == .npc) blk: {
        if (assets.findLevelEntityAnyMap(storage.config.config_id)) |le| {
            if (le.ComponentsData.BaseInfoComponent) |bi| {
                if (bi.Category) |cat| {
                    if (cat.NpcType) |nt| break :blk nt;
                }
            }
            if (std.mem.eql(u8, le.BlueprintType, "SimpleNpc") or std.mem.eql(u8, le.BlueprintType, "SimpleNPc")) break :blk 1;
            if (std.mem.eql(u8, le.BlueprintType, "PasserbyNpc")) break :blk 2;
        }
        break :blk 0;
    } else 0;
    var entity: pb.EntityPb = .{
        .Id = net_id,
        .ConfigId = storage.config.config_id,
        .ConfigType = @enumFromInt(@intFromEnum(storage.config.config_type)),
        .EntityType = @enumFromInt(@intFromEnum(storage.config.entity_type)),
        .SubEntityType = resolved_sub_type,
        .EntityState = @enumFromInt(@intFromEnum(storage.config.state)),
        .d3s = .{ .Camp = storage.config.camp },
        .LivingStatus = living_status,
        .IsVisible = storage.visible != null,
        .IsActorVisible = storage.actor_visible != null,
        .PlayerId = if (storage.player_id) |comp| comp.id else 0,
        .InitLinearVelocity = .{},
        .Gravity = if (storage.world_item.gravity) |gravity| .{
            .X = gravity.location[0],
            .Y = gravity.location[1],
            .Z = gravity.location[2],
        } else .{ .X = 0.0, .Y = 0.0, .Z = -1.0 },
        .RoleSkinId = base_skin_comp.role_skin_id,
        .ParaglidingSkinId = base_skin_comp.paragliding_skin_id,
        .SoarWingSkinId = base_skin_comp.soar_skin_id,
    };

    if (storage.position) |pos_comp| {
        const proto_transform: pb.Transform = pos_comp.toProto();
        entity.Pos = proto_transform.Pos;
        entity.Rot = proto_transform.Rot;
        entity.InitPos = entity.Pos;
    }

    if (storage.world_item.lift) |comp| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .LiftComponentPb = comp.toProto() },
        });
    }

    if (storage.world_item.move_spline) |comp| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .EntityMoveSplineComponentPb = comp.toProto() },
        });
    }

    if (storage.world_item.move_to_point) |comp| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .MoveToPointComponentPb = comp.toProto() },
        });
    }

    if (storage.world_item.scene_item) |comp| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .SceneItemComponentPb = try comp.toProto(alloc.arena) },
        });
    }

    if (storage.world_item.interact) |comp| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .InteractComponent = comp.toProto() },
        });
    }

    if (storage.world_item.monster_capture) |comp| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .MonsterCaptureComponent = comp.toProto() },
        });
    }

    if (storage.world_item.fan_turns) |turns| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .FanComponentPb = .{ .NumOfTurns = turns } },
        });
    }

    if (storage.world_item.gravity_flip) |gt| {
        if (gt >= 0 and gt <= 3) {
            try entity.ComponentPbs.append(alloc.arena, .{
                .ComponentPb = .{ .GravityFlipComponent = .{ .Direction = @enumFromInt(gt) } },
            });
        }
    }

    if (storage.world_item.hook_disabled) |disabled| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .GrapplingHookPointComponentPb = .{ .HookLockPointDisabled = disabled } },
        });
    }

    if (storage.world_item.connector_initiated) |initiated| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .ConnectorComponentPb = .{ .HasInitiatedConnect = initiated } },
        });
    }

    if (storage.world_item.elemental_holder_enable) |enabled| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .ElementalHolderComponentPb = .{ .Enable = enabled } },
        });
    }

    if (storage.world_item.nearby_tracking_enable) |enabled| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .NearbyTrackingComponentPb = .{ .IsEnable = enabled } },
        });
    }

    if (storage.world_item.flower_pollution) {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .FlowerPollutionComponentPb = .{ .UnPollutionSpline = .empty } },
        });
    }

    if (storage.world_item.range_present) {
        var in_range_players: std.ArrayList(i32) = .empty;
        if (storage.world_item.in_range_player_id > 0) {
            try in_range_players.append(alloc.arena, storage.world_item.in_range_player_id);
        }
        var in_range_entities: std.ArrayList(i64) = .empty;
        const n_ents = @min(@as(usize, storage.world_item.in_range_entity_len), storage.world_item.in_range_entity_ids.len);
        for (storage.world_item.in_range_entity_ids[0..n_ents]) |eid| {
            if (eid > 0) try in_range_entities.append(alloc.arena, eid);
        }
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .RangeComponent = .{
                .InRangePlayers = in_range_players,
                .InRangeEntities = in_range_entities,
            } },
        });
    }

    if (storage.world_item.trigger_present) {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .TriggerComponent = .{
                .TriggerCount = storage.world_item.trigger_count,
                .ExitTriggerCount = storage.world_item.exit_trigger_count,
            } },
        });
    }

    if (storage.world_item.patrol_dir) |dir| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .PatrolComponent = .{ .Dir = dir } },
        });
    }

    if (storage.world_item.monster_weapon_id) |wid| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .MonsterWeaponComponentPb = .{ .WeaponId = wid } },
        });
    }

    if (storage.world_item.buff_producer_present) {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .BuffProducerComponentPb = .{} },
        });
    }

    if (storage.world_item.buff_consumer_present) {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .BuffConsumerComponentPb = .{} },
        });
    }

    if (storage.world_item.listener_present) {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .SceneItemEventListenerComponentPb = .{} },
        });
    }

    if (storage.world_item.batch_bullet_cast_present) {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .BatchBulletCastComponentPb = .{ .ConstateId = 0 } },
        });
    }

    if (storage.world_item.hacking_present) {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .HackingComponentPb = .{} },
        });
    }

    if (storage.world_item.hack_target_present) {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .HackTargetComponentPb = .{} },
        });
    }

    if (storage.world_item.pulling_foundation_present) {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .PullingFoundationComponentPb = .{} },
        });
    }

    if (storage.world_item.spirit_gear_present) {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .SpiritGearComponentPb = .{} },
        });
    }

    if (storage.world_item.be_controlled_present or storage.world_item.be_controlled_player_id != null or storage.world_item.be_controlled_show) {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .BeControlledComponentPb = .{
                .PlayerEntityId = storage.world_item.be_controlled_player_id orelse 0,
                .IsShow = storage.world_item.be_controlled_show,
            } },
        });
    }

    if (storage.world_item.drop_present) {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .DropComponentPb = .{
                .ItemId = storage.world_item.drop_item_id,
                .ItemCount = storage.world_item.drop_item_count,
                .EntityConfigId = storage.config.config_id,
            } },
        });
    }

    if (storage.world_item.foundation_source_entity_id > 0) {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .EntityFoundationComponentPb = .{
                .egM = .{ .FoundationCoreInfo = .{
                    .SourceEntityId = storage.world_item.foundation_source_entity_id,
                    .State = "关卡.Common.状态.激活",
                } },
            } },
        });
    }

    if (storage.config.entity_type == .npc and (storage.world_item.npc_spline_id != 0 or storage.world_item.npc_spawn_id != 0)) {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .NpcPb = .{
                .SplineEntityId = storage.world_item.npc_spline_id,
                .SpawnEntityId = storage.world_item.npc_spawn_id,
            } },
        });
    }

    if (storage.world_item.hold_hand_target_id) |target_eid| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .HoldHandComponentPb = .{
                .TargetEntityId = target_eid,
                .HandType = storage.world_item.hold_hand_type,
                .IsFollow = true,
            } },
        });
    }

    if (storage.attribute) |attr_comp| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .AttributeComponent = try attr_comp.toProto(alloc) },
        });
        var shield_list: std.ArrayList(pb.ShieldInfoPb) = .empty;
        var shield_total: i32 = 0;
        if (storage.buffs) |buffs| {
            for (buffs.shields) |sh| {
                if (sh.current <= 0) continue;
                shield_total +|= sh.current;
                try shield_list.append(alloc.arena, .{
                    .Handle = sh.handle_id,
                    .ConfigId = std.math.cast(i32, sh.shield_id) orelse 0,
                    .ShieldValue = sh.current,
                    .Priority = sh.priority,
                    .IsValid = true,
                });
            }
        }
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .ShieldComponentPb = .{
                .ShieldInfoPbList = shield_list,
                .ShieldValueTotal = shield_total,
            } },
        });
    }

    if (storage.tag) |tag_comp| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .TagComponent = tag_comp.toProto() },
        });
    }

    if (storage.part) |*part_comp| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .PartComponent = try part_comp.toProto(alloc) },
        });
    }

    if (storage.buffs) |buff_comp| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .FightBuffComponent = buff_comp.toProto() },
        });
    }

    if (storage.character_attach) |comp| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .CharacterAttachComponentPb = try comp.toProto() },
        });
    }

    if (storage.concomitant) |comp| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .ConcomitantsComponentPb = try comp.toProto() },
        });
    }

    if (storage.equip) |comp| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .EquipComponent = try comp.toProto() },
        });
    }

    if (storage.follower) |comp| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .FollowerComponentPb = try comp.toProto(alloc) },
        });
    }

    if (storage.vehicle) |comp| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .VehiclePb = comp.toProto() },
        });
    }

    if (storage.npc_drive_vehicle) |comp| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .NpcDriveVehicleComponentPb = try comp.toProto() },
        });
    }

    if (storage.motor_outlook) |comp| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .MotorOutlookComponentPb = comp.toProto() },
        });
    }

    if (storage.motor_da_ctx) |comp| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .MotorDaCtxComponentPb = comp.toProto() },
        });
    }

    if (storage.logic_state) |comp| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .LogicStateComponentPb = try comp.toProto() },
        });
    }

    if (storage.monster_ai) |comp| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .MonsterAiComponentPb = try comp.toProto(alloc.arena) },
        });
    }

    if (storage.passive_ga_skill) |comp| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .PassiveGaSkillComponentPb = try comp.toProto() },
        });
        const passive_pb = try comp.toPassiveSkillProto(alloc.arena, assets, net_id);
        if (passive_pb.PassiveSkillItemPbList.items.len > 0) {
            try entity.ComponentPbs.append(alloc.arena, .{
                .ComponentPb = .{ .PassiveSkillComponentPb = passive_pb },
            });
        }
    }

    if (storage.player_battle_binder) |comp| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .PlayerSceneComponentPb = try comp.toProto(alloc) },
        });
    }

    if (storage.summoner) |comp| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .SummonerComponent = try comp.toProto() },
        });
    }

    if (storage.follow_shooter) |comp| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .FollowShooterComponentPb = try comp.toProto() },
        });
    }

    if (storage.follow_entity) |comp| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .FollowEntityComponentPb = try comp.toProto() },
        });
    }

    if (storage.fsm) |comp| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .EntityFsmComponentPb = try comp.toProto(alloc.arena, assets) },
        });
    }

    if (storage.vision_skills) |comp| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .VisionSkillComponent = try comp.toProto() },
        });
    }

    if (storage.weapon_skin) |comp| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .WeaponSkinComponentPb = try comp.toProto() },
        });
    }

    if (storage.ornament) |comp| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .OrnamentComponentPb = try comp.toProto() },
        });
    }

    if (storage.calabash_skin) |comp| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .CalabashSkinComponentPb = try comp.toProto() },
        });
    }

    if (storage.state_tag) |comp| {
        try entity.ComponentPbs.append(alloc.arena, .{
            .ComponentPb = .{ .StateTagComponentPb = try comp.toProto() },
        });
    }

    return entity;
}
