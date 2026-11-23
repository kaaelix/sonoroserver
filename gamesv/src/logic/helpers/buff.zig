const std = @import("std");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const Scene = @import("../../logic/Scene.zig");
const Entity = @import("../../logic/Scene.zig").Entity;
const Assets = @import("../../data/Assets.zig");
const FileSystem = @import("common").FileSystem;
const attributes_helper = @import("../../logic/helpers/attributes.zig");

const ATTR_COUNT: usize = @intCast(@intFromEnum(pb.EAttributeType.MAX));

pub fn configured_duration_seconds(buff_data: *const Assets.DataTables.Buff) f32 {
    return if (buff_data.DurationMagnitude.len > 0) buff_data.DurationMagnitude[0] else 0;
}

pub const CalcutionPolicy = enum(i5) {
    AdvancedMultiplyMagnitude1 = -1,
    AddValue = 0,
    AddRate = 1,
    AddFromAttr = 2,
    OverrideValue = 3,
    OverrideFromAttr = 4,
    AddTimeScaledValue = 5,
    AddTimeScaledRate = 6,
    AddFromAttrRate = 9,
};

fn isBaseScaledAttr(attr: pb.EAttributeType) bool {
    return switch (attr) {
        .LifeMax, .Atk, .Def => true,
        else => false,
    };
}

fn isPooledAttr(attr: pb.EAttributeType) bool {
    return switch (attr) {
        .Life,
        .Energy,
        .ElementEnergy,
        .Strength,
        .Tough,
        .Hardness,
        .Rage,
        .SpecialEnergy1,
        .SpecialEnergy2,
        .SpecialEnergy3,
        .SpecialEnergy4,
        .SpecialEnergy5,
        => true,
        else => false,
    };
}

fn resolvePolicy(buff_data: *const Assets.DataTables.Buff) ?CalcutionPolicy {
    const raw = if (buff_data.CalculationPolicy.len > 0) buff_data.CalculationPolicy[0] else return null;
    inline for (std.meta.fields(CalcutionPolicy)) |f| {
        if (f.value == raw) return @enumFromInt(raw);
    }
    return .AddValue;
}

fn magnitudeAtLevel(buff_data: *const Assets.DataTables.Buff, level: i32) i32 {
    if (buff_data.ModifierMagnitude.len == 0) return 0;
    const idx: usize = @intCast(@max(0, level - 1));
    return buff_data.ModifierMagnitude[@min(idx, buff_data.ModifierMagnitude.len - 1)];
}

fn findEntityAttribute(scene: *const Scene, entity_id: i64) ?*const Entity.AttributeComponent {
    const idx = scene.net_id_map.get(entity_id) orelse return null;
    if (scene.entities.items(.attribute)[idx]) |*attr| return attr;
    return null;
}

fn computeFromAttrDelta(
    scene: ?*const Scene,
    target_id: i64,
    instigator_id: i64,
    target_attr: *const Entity.AttributeComponent,
    buff_data: *const Assets.DataTables.Buff,
    level: i32,
    gpa: std.mem.Allocator,
) ?i32 {
    _ = gpa;
    if (buff_data.CalculationPolicy.len < 2) return null;
    const modifier = @as(f64, @floatFromInt(magnitudeAtLevel(buff_data, level)));
    const based_attr_id = buff_data.CalculationPolicy[1];
    const use_instigator = buff_data.CalculationPolicy.len > 2 and buff_data.CalculationPolicy[2] == 1;
    const source_id = if (use_instigator) instigator_id else target_id;

    const source_attr: *const Entity.AttributeComponent = if (source_id == target_id)
        target_attr
    else if (scene) |sc|
        findEntityAttribute(sc, source_id) orelse return null
    else
        return null;

    const raw_source = attributes_helper.get_attr_by_id(source_attr, based_attr_id);
    const source_value: f64 = @floatFromInt(raw_source);
    const min_required: f64 = if (buff_data.CalculationPolicy.len > 5)
        @floatFromInt(buff_data.CalculationPolicy[5])
    else
        0.0;
    if (source_value < min_required) return null;

    const conversion_factor: f64 = if (buff_data.CalculationPolicy.len > 6)
        @max(1.0, @as(f64, @floatFromInt(buff_data.CalculationPolicy[6])))
    else
        1.0;
    const max_value: f64 = if (buff_data.CalculationPolicy.len > 7)
        @floatFromInt(buff_data.CalculationPolicy[7])
    else
        @floatFromInt(std.math.maxInt(i32));

    const calculated = ((source_value - min_required) / conversion_factor) * (modifier / 1e4);
    const capped = @min(calculated, max_value);
    return @as(i32, @intFromFloat(@trunc(capped)));
}

fn computeRateReference(
    attr_type: pb.EAttributeType,
    target_attr: *const Entity.AttributeComponent,
    gpa: std.mem.Allocator,
) i32 {
    if (isBaseScaledAttr(attr_type)) {
        const idx = @intFromEnum(attr_type);
        return if (target_attr.attributes.len > idx) target_attr.attributes[@intCast(idx)].base else 0;
    }
    if (isPooledAttr(attr_type)) {
        const maybe_max = attributes_helper.get_related_attr(attr_type, .Max, gpa) catch null;
        if (maybe_max) |max_attr| {
            const max_val = attributes_helper.get_attr_value(target_attr, max_attr);
            if (max_val > 0) return max_val;
        }
        return attributes_helper.get_attr_value(target_attr, attr_type);
    }
    return 10000;
}

pub fn execute_buff_effects(
    combat_receive_pack: *std.ArrayList(pb.CombatReceiveData),
    target_id: i64,
    instigator_id: i64,
    buff_data: *const Assets.DataTables.Buff,
    scene: *Scene,
    fs: *FileSystem,
    io: std.Io,
    query: Scene.Query(&.{
        Entity,
        *Entity.FightBuffComponent,
        ?*Entity.AttributeComponent,
    }),
    alloc: mem.Alloc,
) !void {
    try executeBuffEffects(
        .instant,
        combat_receive_pack,
        target_id,
        instigator_id,
        buff_data,
        scene,
        null,
        fs,
        io,
        query,
        alloc,
    );
}

pub fn execute_periodic_buff_effects(
    combat_receive_pack: *std.ArrayList(pb.CombatReceiveData),
    target_id: i64,
    instigator_id: i64,
    buff_data: *const Assets.DataTables.Buff,
    scene: *Scene,
    fs: *FileSystem,
    io: std.Io,
    query: Scene.Query(&.{
        Entity,
        *Entity.FightBuffComponent,
        ?*Entity.AttributeComponent,
    }),
    alloc: mem.Alloc,
) !void {
    try executeBuffEffects(
        .periodic,
        combat_receive_pack,
        target_id,
        instigator_id,
        buff_data,
        scene,
        null,
        fs,
        io,
        query,
        alloc,
    );
}

pub fn executePeriodicWithAssets(
    combat_receive_pack: *std.ArrayList(pb.CombatReceiveData),
    target_id: i64,
    instigator_id: i64,
    buff_data: *const Assets.DataTables.Buff,
    scene: *Scene,
    assets: *const Assets,
    fs: *FileSystem,
    io: std.Io,
    query: Scene.Query(&.{
        Entity,
        *Entity.FightBuffComponent,
        ?*Entity.AttributeComponent,
    }),
    alloc: mem.Alloc,
) !void {
    try executeBuffEffects(
        .periodic,
        combat_receive_pack,
        target_id,
        instigator_id,
        buff_data,
        scene,
        assets,
        fs,
        io,
        query,
        alloc,
    );
}

const ExecutionMode = enum {
    instant,
    periodic,
};

fn executeBuffEffects(
    mode: ExecutionMode,
    combat_receive_pack: *std.ArrayList(pb.CombatReceiveData),
    target_id: i64,
    instigator_id: i64,
    buff_data: *const Assets.DataTables.Buff,
    scene: *Scene,
    maybe_assets: ?*const Assets,
    fs: *FileSystem,
    io: std.Io,
    query: Scene.Query(&.{
        Entity,
        *Entity.FightBuffComponent,
        ?*Entity.AttributeComponent,
    }),
    alloc: mem.Alloc,
) !void {
    _ = fs;
    const log = std.log.scoped(.buff_helper);
    const target_entity, const target_buffs, const maybe_target_attr = query.byNetId(target_id) orelse {
        log.debug("couldnt find target entity for buff effect: {d}", .{target_id});
        return;
    };

    switch (mode) {
        .instant => if (buff_data.DurationPolicy != .Instant) return,
        .periodic => if (buff_data.Period <= 0) return,
    }

    if (maybe_target_attr) |target_attr| {
        const calc_policy = resolvePolicy(buff_data) orelse return;
        const before_hp = attributes_helper.get_attr_value(target_attr, .Life);
        const active_buff = target_buffs.getByBuffId(buff_data.Id);
        const eff_level: i32 = if (active_buff) |b| @max(1, b.Level) else 1;
        const eff_stacks: i32 = if (mode == .periodic and active_buff != null) @max(1, active_buff.?.StackCount) else 1;

        var change: std.ArrayList(pb.EAttributeType) = .empty;
        var s: i32 = 0;
        while (s < eff_stacks) : (s += 1) {
            const step_change = try calculate_buff_effects(
                scene,
                target_entity,
                target_attr,
                instigator_id,
                buff_data,
                eff_level,
                calc_policy,
                alloc,
            );
            for (step_change.items) |attr_t| {
                if (std.mem.indexOfScalar(pb.EAttributeType, change.items, attr_t) == null) {
                    try change.append(alloc.arena, attr_t);
                }
            }
        }
        const after_hp = attributes_helper.get_attr_value(target_attr, .Life);
        if (change.items.len != 0) {
            if (after_hp > 0) {
                try scene.markFsmDirty(alloc.gpa, target_entity.net_id, Entity.FsmComponent.WakeReason.attribute);
            }
            try attributes_helper.generate_attr_messages(
                combat_receive_pack,
                target_entity.net_id,
                target_attr,
                &change,
                alloc,
                io,
            );
        }

        if (before_hp > 0 and after_hp <= 0) {
            if (maybe_assets) |assets| {
                _ = try handleEntityDeath(
                    combat_receive_pack,
                    scene,
                    assets,
                    target_entity.net_id,
                    target_buffs,
                    target_attr,
                    alloc,
                    io,
                );
            } else {
                try scene.setBattleEntityActive(alloc.gpa, target_entity.net_id, false);
                try combat_receive_pack.append(alloc.arena, .{ .Message = .{ .CombatNotifyData = .{
                    .CombatCommon = .{ .EntityId = target_entity.net_id },
                    .Message = .{ .EntityLivingStatusNotify = .{
                        .Id = target_entity.net_id,
                        .LivingStatus = .Dead,
                    } },
                } } });
            }
            const due_ms = std.Io.Clock.awake.now(io).toMilliseconds() + 300;
            scene.queueKilledMonsterRemovalAt(alloc.gpa, target_entity.net_id, due_ms) catch {};
            _ = try scene.appendBattleStateNotify(alloc.arena, combat_receive_pack);
        }
    }
}

fn calculate_buff_effects(
    scene: *const Scene,
    target_entity: Entity,
    target_attr: *Entity.AttributeComponent,
    instigator_id: i64,
    buff_data: *const Assets.DataTables.Buff,
    level: i32,
    calc_policy: CalcutionPolicy,
    alloc: mem.Alloc,
) !std.ArrayList(pb.EAttributeType) {
    if (buff_data.GameAttributeID == .None or buff_data.GameAttributeID == .MAX) return .empty;
    switch (calc_policy) {
        .AddValue => {
            const value = magnitudeAtLevel(buff_data, level);
            if (value == 0) return .empty;
            return try attributes_helper.change_attr(
                target_attr,
                buff_data.GameAttributeID,
                .Delta,
                .Current,
                value,
                alloc,
            );
        },
        .AddRate => {
            const modifier = magnitudeAtLevel(buff_data, level);
            if (modifier == 0) return .empty;
            const ref = computeRateReference(buff_data.GameAttributeID, target_attr, alloc.gpa);
            const delta: i32 = @intFromFloat(@round(@as(f64, @floatFromInt(ref)) * @as(f64, @floatFromInt(modifier)) / 1e4));
            if (delta == 0) return .empty;
            return try attributes_helper.change_attr(
                target_attr,
                buff_data.GameAttributeID,
                .Delta,
                .Current,
                delta,
                alloc,
            );
        },
        .AddFromAttr, .AddFromAttrRate => {
            const delta = computeFromAttrDelta(
                scene,
                target_entity.net_id,
                instigator_id,
                target_attr,
                buff_data,
                level,
                alloc.gpa,
            ) orelse return .empty;
            if (delta == 0) return .empty;
            return try attributes_helper.change_attr(
                target_attr,
                buff_data.GameAttributeID,
                .Delta,
                .Current,
                delta,
                alloc,
            );
        },
        .OverrideValue => {
            const value = magnitudeAtLevel(buff_data, level);
            return try attributes_helper.change_attr(
                target_attr,
                buff_data.GameAttributeID,
                .Override,
                .Current,
                value,
                alloc,
            );
        },
        .OverrideFromAttr => {
            if (buff_data.CalculationPolicy.len < 2) return .empty;
            const based_attr_id = buff_data.CalculationPolicy[1];
            const use_instigator = buff_data.CalculationPolicy.len > 2 and buff_data.CalculationPolicy[2] == 1;
            const source_id = if (use_instigator) instigator_id else target_entity.net_id;
            const source_attr = if (source_id == target_entity.net_id)
                target_attr
            else
                findEntityAttribute(scene, source_id) orelse return .empty;
            const src_val = attributes_helper.get_attr_by_id(source_attr, based_attr_id);
            const modifier = magnitudeAtLevel(buff_data, level);
            const value: i32 = @intFromFloat(@round(@as(f64, @floatFromInt(src_val)) * @as(f64, @floatFromInt(modifier)) / 1e4));
            return try attributes_helper.change_attr(
                target_attr,
                buff_data.GameAttributeID,
                .Override,
                .Current,
                value,
                alloc,
            );
        },
        else => {},
    }

    return .empty;
}

pub fn syncContinuousBuffModifiers(
    combat_receive_pack: *std.ArrayList(pb.CombatReceiveData),
    scene: *Scene,
    entity_id: i64,
    buff_comp: *Entity.FightBuffComponent,
    attr_comp: *Entity.AttributeComponent,
    assets: *const Assets,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    var desired: [ATTR_COUNT]i64 = @splat(0);

    for (buff_comp.fight_buff_infos) |info| {
        const is_prebaked = buff_comp.isPrebakedHandle(info.HandleId);
        if (!info.IsActive and !is_prebaked) continue;
        const buff_data = assets.tables.buff.getDataById(info.BuffId) orelse continue;
        if (buff_data.DurationPolicy == .Instant or buff_data.Period > 0) continue;
        if (buff_data.GameAttributeID == .None or buff_data.GameAttributeID == .MAX) continue;

        const attr_idx: usize = @intCast(@intFromEnum(buff_data.GameAttributeID));
        if (attr_idx >= ATTR_COUNT) continue;

        const stacks: i32 = if (is_prebaked)
            (if (info.IsActive) @max(0, info.StackCount - 1) else -1)
        else
            @max(1, info.StackCount);
        if (stacks == 0) continue;

        const policy = resolvePolicy(&buff_data) orelse continue;
        const level = if (info.Level > 0) info.Level else 1;
        const per_stack: i64 = switch (policy) {
            .AddValue => magnitudeAtLevel(&buff_data, level),
            .AddRate => blk: {
                const mod = magnitudeAtLevel(&buff_data, level);
                if (mod == 0) break :blk 0;
                const ref = computeRateReference(buff_data.GameAttributeID, attr_comp, alloc.gpa);
                break :blk @as(i64, @intFromFloat(@round(@as(f64, @floatFromInt(ref)) * @as(f64, @floatFromInt(mod)) / 1e4)));
            },
            .AddFromAttr, .AddFromAttrRate => blk: {
                const d = computeFromAttrDelta(
                    scene,
                    entity_id,
                    info.InstigatorId,
                    attr_comp,
                    &buff_data,
                    level,
                    alloc.gpa,
                ) orelse 0;
                break :blk d;
            },
            .OverrideValue => blk: {
                if (buff_comp.isPrebakedHandle(info.HandleId)) break :blk 0;
                const override_val = magnitudeAtLevel(&buff_data, level);
                desired[attr_idx] = @as(i64, override_val) - attr_comp.attributes[attr_idx].base;
                continue;
            },
            else => 0,
        };
        desired[attr_idx] += per_stack * stacks;
    }

    var prev: [ATTR_COUNT]i64 = @splat(0);
    for (buff_comp.applied_buff_mods) |mod| {
        if (mod.attr_idx >= 0 and @as(usize, @intCast(mod.attr_idx)) < ATTR_COUNT) {
            prev[@intCast(mod.attr_idx)] = mod.delta;
        }
    }

    var changed_attrs: std.ArrayList(pb.EAttributeType) = .empty;
    var next_mods: std.ArrayList(Entity.FightBuffComponent.AppliedBuffMod) = .empty;
    defer next_mods.deinit(alloc.gpa);

    inline for (comptime std.meta.fields(pb.EAttributeType), 0..) |field, i| {
        if (i < ATTR_COUNT) {
            const d_val = desired[i];
            const p_val = prev[i];
            if (d_val != 0) {
                const clamped_d: i32 = @intCast(std.math.clamp(d_val, std.math.minInt(i32), std.math.maxInt(i32)));
                try next_mods.append(alloc.gpa, .{ .attr_idx = @intCast(i), .delta = clamped_d });
            }
            const diff = d_val - p_val;
            if (diff != 0) {
                const clamped_diff: i32 = @intCast(std.math.clamp(diff, std.math.minInt(i32), std.math.maxInt(i32)));
                const attr_type: pb.EAttributeType = @enumFromInt(field.value);
                const ch = try attributes_helper.change_attr(attr_comp, attr_type, .Delta, .Increment, clamped_diff, alloc);
                try changed_attrs.appendSlice(alloc.arena, ch.items);
            }
        }
    }

    try buff_comp.setAppliedBuffMods(alloc.gpa, next_mods.items);
    if (changed_attrs.items.len != 0) {
        try scene.markFsmDirty(alloc.gpa, entity_id, Entity.FsmComponent.WakeReason.attribute);
        try attributes_helper.generate_attr_messages(combat_receive_pack, entity_id, attr_comp, &changed_attrs, alloc, io);
    }
}

pub fn syncBuffShield(
    combat_receive_pack: *std.ArrayList(pb.CombatReceiveData),
    scene: *const Scene,
    assets: *const Assets,
    entity_id: i64,
    buff_comp: *Entity.FightBuffComponent,
    maybe_attr: ?*Entity.AttributeComponent,
    buff_info: pb.FightBuffInformation,
    buff_data: *const Assets.DataTables.Buff,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    if (buff_data.ExtraEffectID != 11 or buff_data.ExtraEffectParameters.len == 0) return;

    const shield_id = std.fmt.parseInt(i64, std.mem.trim(u8, buff_data.ExtraEffectParameters[0], " "), 10) catch return;
    const shield_cfg = assets.tables.shield.getDataById(shield_id) orelse return;
    const target_attr = maybe_attr orelse return;

    var updates: std.ArrayList(pb.ShieldUpdateInfo) = .empty;
    var had_existing = false;
    for (buff_comp.shields) |entry| {
        if (entry.handle_id == buff_info.HandleId) {
            had_existing = true;
            break;
        }
    }

    if (!buff_info.IsActive) {
        if (!had_existing) return;
        buff_comp.removeShieldByHandle(alloc.gpa, buff_info.HandleId);
        try updates.append(alloc.arena, .{
            .Handle = buff_info.HandleId,
            .ConfigId = @intCast(shield_id),
            .ShieldValue = 0,
            .UpdateType = .EShieldUpdateTypeDel,
        });
    } else {
        const ref_id = if (shield_cfg.RefrenceTarget == 0 and buff_info.InstigatorId != 0)
            buff_info.InstigatorId
        else
            entity_id;
        const ref_attr: *const Entity.AttributeComponent = if (ref_id == entity_id)
            target_attr
        else
            findEntityAttribute(scene, ref_id) orelse target_attr;

        const ref_val = @max(0, attributes_helper.get_attr_by_id(ref_attr, shield_cfg.RefrenceAttribute));
        const lv_idx: usize = @intCast(@max(0, buff_info.Level - 1));
        const rate: f64 = if (shield_cfg.AttributeRate.len > 0)
            shield_cfg.AttributeRate[@min(lv_idx, shield_cfg.AttributeRate.len - 1)]
        else
            0.0;
        const flat: i32 = if (shield_cfg.Value.len > 0)
            shield_cfg.Value[@min(lv_idx, shield_cfg.Value.len - 1)]
        else
            0;
        const stacks = @max(1, buff_info.StackCount);
        const raw_hp: i32 = @max(1, @as(i32, @intFromFloat(@round(@as(f64, @floatFromInt(ref_val)) * rate + @as(f64, @floatFromInt(flat)))))) * stacks;

        try buff_comp.upsertShield(alloc.gpa, .{
            .handle_id = buff_info.HandleId,
            .shield_id = shield_id,
            .priority = shield_cfg.Priority,
            .current = raw_hp,
            .max = raw_hp,
        });
        try updates.append(alloc.arena, .{
            .Handle = buff_info.HandleId,
            .ConfigId = @intCast(shield_id),
            .ShieldValue = raw_hp,
            .UpdateType = if (had_existing) .EShieldUpdateTypeModify else .EShieldUpdateTypeAdd,
        });
    }

    try combat_receive_pack.append(alloc.arena, .{
        .Message = .{
            .CombatNotifyData = .{
                .CombatCommon = .{ .EntityId = entity_id },
                .Message = .{ .ShieldUpdateNotify = .{ .Shields = updates } },
            },
        },
    });
    try syncShieldAttribute(combat_receive_pack, entity_id, buff_comp, target_attr, alloc, io);
}

pub fn appendShieldDelForHandle(
    combat_receive_pack: *std.ArrayList(pb.CombatReceiveData),
    entity_id: i64,
    buff_comp: *Entity.FightBuffComponent,
    maybe_attr: ?*Entity.AttributeComponent,
    handle_id: i32,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    var updates: std.ArrayList(pb.ShieldUpdateInfo) = .empty;
    for (buff_comp.shields) |entry| {
        if (entry.handle_id == handle_id) {
            try updates.append(alloc.arena, .{
                .Handle = entry.handle_id,
                .ConfigId = @intCast(entry.shield_id),
                .ShieldValue = 0,
                .UpdateType = .EShieldUpdateTypeDel,
            });
        }
    }
    if (updates.items.len == 0) return;

    buff_comp.removeShieldByHandle(alloc.gpa, handle_id);
    try combat_receive_pack.append(alloc.arena, .{
        .Message = .{
            .CombatNotifyData = .{
                .CombatCommon = .{ .EntityId = entity_id },
                .Message = .{ .ShieldUpdateNotify = .{ .Shields = updates } },
            },
        },
    });
    if (maybe_attr) |attr| {
        try syncShieldAttribute(combat_receive_pack, entity_id, buff_comp, attr, alloc, io);
    }
}

pub fn syncShieldAttribute(
    combat_receive_pack: *std.ArrayList(pb.CombatReceiveData),
    entity_id: i64,
    buff_comp: *const Entity.FightBuffComponent,
    attr_comp: *Entity.AttributeComponent,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    const new_shield = buff_comp.totalShield();
    const cur_shield = attributes_helper.get_attr_value(attr_comp, .Sheild);
    if (new_shield == cur_shield) return;

    const changed = try attributes_helper.change_attr(
        attr_comp,
        .Sheild,
        .Override,
        .Current,
        new_shield,
        alloc,
    );
    try attributes_helper.generate_attr_messages(
        combat_receive_pack,
        entity_id,
        attr_comp,
        &changed,
        alloc,
        io,
    );
}

pub fn absorbDamageWithShields(
    combat_receive_pack: *std.ArrayList(pb.CombatReceiveData),
    entity_id: i64,
    buff_comp: *Entity.FightBuffComponent,
    attr_comp: *Entity.AttributeComponent,
    raw_damage: i32,
    alloc: mem.Alloc,
    io: std.Io,
) !i32 {
    if (raw_damage <= 0 or buff_comp.shields.len == 0) return 0;

    var remaining = raw_damage;
    var total_covered: i32 = 0;
    var updates: std.ArrayList(pb.ShieldUpdateInfo) = .empty;
    var depleted_handles: std.ArrayList(i32) = .empty;

    while (remaining > 0) {
        var best_idx: ?usize = null;
        for (buff_comp.shields, 0..) |entry, i| {
            if (entry.current <= 0) continue;
            if (best_idx) |bi| {
                if (entry.priority > buff_comp.shields[bi].priority) best_idx = i;
            } else {
                best_idx = i;
            }
        }
        const idx = best_idx orelse break;
        const entry = &buff_comp.shields[idx];
        const absorbed = @min(remaining, entry.current);
        entry.current -= absorbed;
        remaining -= absorbed;
        total_covered += absorbed;

        const is_depleted = entry.current <= 0;
        try updates.append(alloc.arena, .{
            .Handle = entry.handle_id,
            .ConfigId = @intCast(entry.shield_id),
            .ShieldValue = @max(0, entry.current),
            .UpdateType = if (is_depleted) .EShieldUpdateTypeDel else .EShieldUpdateTypeModify,
        });
        if (is_depleted) {
            try depleted_handles.append(alloc.arena, entry.handle_id);
        }
    }

    for (depleted_handles.items) |handle_id| {
        buff_comp.removeShieldByHandle(alloc.gpa, handle_id);
    }

    if (updates.items.len != 0) {
        try combat_receive_pack.append(alloc.arena, .{
            .Message = .{
                .CombatNotifyData = .{
                    .CombatCommon = .{ .EntityId = entity_id },
                    .Message = .{ .ShieldUpdateNotify = .{ .Shields = updates } },
                },
            },
        });
        try syncShieldAttribute(combat_receive_pack, entity_id, buff_comp, attr_comp, alloc, io);
    }

    return total_covered;
}

pub fn clearDeadRemoveBuffs(
    combat_receive_pack: *std.ArrayList(pb.CombatReceiveData),
    scene: *Scene,
    entity_id: i64,
    buff_comp: *Entity.FightBuffComponent,
    maybe_attr: ?*Entity.AttributeComponent,
    assets: *const Assets,
    alloc: mem.Alloc,
    io: std.Io,
) ![]i32 {
    var removed_handles: std.ArrayList(i32) = .empty;
    for (buff_comp.fight_buff_infos) |info| {
        const buff_data = assets.tables.buff.getDataById(info.BuffId) orelse continue;
        if (buff_data.DeadRemove) {
            try removed_handles.append(alloc.arena, info.HandleId);
        }
    }
    if (removed_handles.items.len == 0) return &.{};

    var shield_updates: std.ArrayList(pb.ShieldUpdateInfo) = .empty;
    for (removed_handles.items) |handle_id| {
        for (buff_comp.shields) |shield| {
            if (shield.handle_id == handle_id) {
                try shield_updates.append(alloc.arena, .{
                    .Handle = shield.handle_id,
                    .ConfigId = @intCast(shield.shield_id),
                    .ShieldValue = 0,
                    .UpdateType = .EShieldUpdateTypeDel,
                });
            }
        }
        buff_comp.removeByHandleId(alloc.gpa, handle_id);
        try combat_receive_pack.append(alloc.arena, .{
            .Message = .{
                .CombatNotifyData = .{
                    .CombatCommon = .{ .EntityId = entity_id },
                    .Message = .{
                        .RemoveGameplayEffectNotify = .{
                            .EntityId = entity_id,
                            .Handle = handle_id,
                        },
                    },
                },
            },
        });
    }

    if (shield_updates.items.len != 0) {
        try combat_receive_pack.append(alloc.arena, .{
            .Message = .{
                .CombatNotifyData = .{
                    .CombatCommon = .{ .EntityId = entity_id },
                    .Message = .{ .ShieldUpdateNotify = .{ .Shields = shield_updates } },
                },
            },
        });
        if (maybe_attr) |attr_comp| {
            try syncShieldAttribute(combat_receive_pack, entity_id, buff_comp, attr_comp, alloc, io);
        }
    }
    if (maybe_attr) |attr_comp| {
        try syncContinuousBuffModifiers(combat_receive_pack, scene, entity_id, buff_comp, attr_comp, assets, alloc, io);
    }

    return removed_handles.items;
}

pub fn handleEntityDeath(
    combat_receive_pack: *std.ArrayList(pb.CombatReceiveData),
    scene: *Scene,
    assets: *const Assets,
    entity_id: i64,
    buff_comp: *Entity.FightBuffComponent,
    maybe_attr: ?*Entity.AttributeComponent,
    alloc: mem.Alloc,
    io: std.Io,
) ![]i32 {
    try scene.setBattleEntityActive(alloc.gpa, entity_id, false);
    try combat_receive_pack.append(alloc.arena, .{
        .Message = .{
            .CombatNotifyData = .{
                .CombatCommon = .{ .EntityId = entity_id },
                .Message = .{
                    .EntityLivingStatusNotify = .{
                        .Id = entity_id,
                        .LivingStatus = .Dead,
                    },
                },
            },
        },
    });
    return try clearDeadRemoveBuffs(combat_receive_pack, scene, entity_id, buff_comp, maybe_attr, assets, alloc, io);
}
