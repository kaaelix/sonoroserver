const std = @import("std");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const Scene = @import("../../logic/Scene.zig");
const Entity = @import("../../logic/Scene.zig").Entity;
const Assets = @import("../../data/Assets.zig");
const FileSystem = @import("common").FileSystem;
const attributes_helper = @import("attributes.zig");
const buff_helper = @import("buff.zig");
const gameplay_tags = @import("gameplay_tags.zig");
const BuffTimerScheduler = @import("../schedulers/BuffTimerScheduler.zig");

fn levelSliceValue(slice: []const i32, skill_level: i32, default_val: i32) i32 {
    if (slice.len == 0) return default_val;
    const idx: usize = @intCast(@max(0, skill_level - 1));
    return slice[@min(idx, slice.len - 1)];
}

fn firstParam(slice: []const i32, default_val: i32) i32 {
    return if (slice.len > 0) slice[0] else default_val;
}

fn formulaParam(slice: []const i32, skill_level: i32) f64 {
    return @floatFromInt(levelSliceValue(slice, skill_level, 0));
}

fn attrFromOwner(
    attacker_attr: *const Entity.AttributeComponent,
    target_attr: *const Entity.AttributeComponent,
    owner_flag: f64,
    attr_id_f: f64,
) f64 {
    const attr_id: i32 = @intFromFloat(attr_id_f);
    if (attr_id <= 0 or attr_id >= @intFromEnum(pb.EAttributeType.MAX)) return 0.0;
    const owner_idx: i32 = @intFromFloat(owner_flag);
    const src = if (owner_idx != 0) target_attr else attacker_attr;
    return @floatFromInt(attributes_helper.get_attr_by_id(src, attr_id));
}

fn calculateHurtCore(
    base_or_formula: f64,
    attacker_attr: *const Entity.AttributeComponent,
    target_attr: *const Entity.AttributeComponent,
    damage: *const Assets.DataTables.Damage,
    is_crit: bool,
) f64 {
    const raw_crit_dmg = attributes_helper.get_attr_value(attacker_attr, .CritDamage);
    const crit_dmg_bp = if (raw_crit_dmg > 0) raw_crit_dmg else 15000;
    const crit_mult: f64 = if (is_crit) @as(f64, @floatFromInt(crit_dmg_bp)) / 10000.0 else 1.0;

    const elem_bonus = @as(f64, @floatFromInt(attributes_helper.get_attr_value(attacker_attr, attributes_helper.get_element_bonus(damage.Element)))) / 10000.0;
    const ability_bonus = @as(f64, @floatFromInt(attributes_helper.get_attr_value(attacker_attr, attributes_helper.get_ability_bonus(damage.Type)))) / 10000.0;
    const dmg_change = @as(f64, @floatFromInt(attributes_helper.get_attr_value(attacker_attr, .DamageChange))) / 10000.0;
    const total_bonus = @max(0.0, (1.0 + dmg_change) + elem_bonus + ability_bonus);

    const dmg_reduce = @min(@as(f64, @floatFromInt(attributes_helper.get_attr_value(target_attr, .DamageReduce))) / 10000.0, 1.0);
    const elem_reduce = @min(@as(f64, @floatFromInt(attributes_helper.get_attr_value(target_attr, attributes_helper.get_element_reduce(damage.Element)))) / 10000.0, 1.0);
    const special_mult = @max(0.0, 1.0 + @as(f64, @floatFromInt(attributes_helper.get_attr_value(attacker_attr, .SpecialDamageChange))) / 10000.0);

    const attacker_lv = @max(1, attributes_helper.get_attr_value(attacker_attr, .Lv));
    const target_def = @max(0, attributes_helper.get_attr_value(target_attr, .Def));
    const ignore_def = std.math.clamp(@as(f64, @floatFromInt(attributes_helper.get_attr_value(attacker_attr, .IgnoreDefRate))) / 10000.0, 0.0, 1.0);
    const lv_term = 800.0 + 8.0 * @as(f64, @floatFromInt(attacker_lv));
    const def_mult = @min(2.0, 1.0 / (@as(f64, @floatFromInt(target_def)) * (1.0 - ignore_def) / lv_term + 1.0));

    const raw_res = attributes_helper.get_attr_value(target_attr, attributes_helper.get_element_res(damage.Element));
    const pen = attributes_helper.get_attr_value(attacker_attr, attributes_helper.get_element_pen(damage.Element));
    const res_mult = resistanceMultiplier(raw_res - pen);

    return @max(0.0, base_or_formula * crit_mult * def_mult * total_bonus * res_mult * (1.0 - dmg_reduce) * (1.0 - elem_reduce) * special_mult);
}

fn computeFormulaRaw(
    assets: *const Assets,
    damage: *const Assets.DataTables.Damage,
    attacker_attr: *const Entity.AttributeComponent,
    target_attr: *const Entity.AttributeComponent,
    skill_level: i32,
    is_crit: bool,
) f64 {
    const rate_bp = levelSliceValue(damage.RateLv, skill_level, 0);
    const prop_id = if (damage.RelatedProperty > 0) damage.RelatedProperty else @intFromEnum(pb.EAttributeType.Atk);
    const base_stat = @max(0, attributes_helper.get_attr_by_id(attacker_attr, prop_id));
    const rate_scaled = @as(f64, @floatFromInt(base_stat)) * (@as(f64, @floatFromInt(rate_bp)) / 10000.0);

    const p0 = formulaParam(damage.FormulaParam1, skill_level);
    const p1 = formulaParam(damage.FormulaParam2, skill_level);
    const p2 = formulaParam(damage.FormulaParam3, skill_level);
    const p3 = formulaParam(damage.FormulaParam4, skill_level);
    const p4 = formulaParam(damage.FormulaParam5, skill_level);
    const p5 = formulaParam(damage.FormulaParam6, skill_level);
    const p6 = formulaParam(damage.FormulaParam7, skill_level);
    const p7 = formulaParam(damage.FormulaParam8, skill_level);
    const p8 = formulaParam(damage.FormulaParam9, skill_level);
    const p9 = formulaParam(damage.FormulaParam10, skill_level);

    switch (damage.FormulaType) {
        0 => {
            if (damage.CalculateType == 1) {
                const cure_base = levelSliceValue(damage.CureBaseValue, skill_level, 0);
                return @max(0.0, rate_scaled + @as(f64, @floatFromInt(@max(0, cure_base))));
            }
            return calculateHurtCore(rate_scaled, attacker_attr, target_attr, damage, is_crit);
        },
        1 => {
            const num11 = attrFromOwner(attacker_attr, target_attr, p3, p2) * (p4 / 10000.0) + p5;
            const num12 = attrFromOwner(attacker_attr, target_attr, p8, p7) * (p6 / 10000.0) + p9 +
                attrFromOwner(attacker_attr, target_attr, p1, p0) * num11;
            if (damage.CalculateType == 1) {
                return @max(0.0, num12);
            }
            return calculateHurtCore(@max(0.0, rate_scaled + num12), attacker_attr, target_attr, damage, is_crit);
        },
        2 => {
            const num10 = attrFromOwner(attacker_attr, target_attr, p1, p0) * (p2 / 10000.0) + p3;
            const t_life: f64 = @floatFromInt(attributes_helper.get_attr_value(target_attr, .Life));
            if (damage.CalculateType == 1) {
                return @max(0.0, -@min(t_life - num10, 0.0));
            }
            return @max(0.0, t_life - num10);
        },
        3 => {
            return @max(0.0, attrFromOwner(attacker_attr, target_attr, p1, p0) * (p2 / 10000.0) + p3);
        },
        4 => {
            const raw = attrFromOwner(attacker_attr, target_attr, p1, p0) * (p2 / 10000.0) + p3;
            return @max(0.0, raw);
        },
        5 => {
            const t_max: f64 = @floatFromInt(attributes_helper.get_attr_value(target_attr, .LifeMax));
            return @max(0.0, t_max * (p0 / 10000.0) + p1);
        },
        6 => {
            const a = attrFromOwner(attacker_attr, target_attr, p1, p0) * (p2 / 10000.0);
            const b = attrFromOwner(attacker_attr, target_attr, p4, p3) * (p5 / 10000.0);
            return @max(0.0, a + b);
        },
        7 => {
            const val = attrFromOwner(attacker_attr, target_attr, p1, p0) * (p2 / 10000.0) + p3;
            const t_life: f64 = @floatFromInt(attributes_helper.get_attr_value(target_attr, .Life));
            const t_max: f64 = @floatFromInt(attributes_helper.get_attr_value(target_attr, .LifeMax));
            const val2 = t_life - (t_max * (p4 / 10000.0) + p5);
            return @max(0.0, @min(val, val2));
        },
        9 => {
            if (rate_bp > 0) return calculateHurtCore(rate_scaled, attacker_attr, target_attr, damage, is_crit);
            const lv = @max(1, attributes_helper.get_attr_value(attacker_attr, .Lv));
            return @floatFromInt(lv * 25 + 100);
        },
        1001 => {
            const attacker_lv = @max(1, attributes_helper.get_attr_value(attacker_attr, .Lv));
            const ab_base: f64 = if (assets.tables.abnormal_damage_config.getDataById(attacker_lv)) |ab| switch (damage.Element) {
                1 => @floatFromInt(ab.Abnormal1003),
                2 => @floatFromInt(ab.Abnormal1004),
                3 => @floatFromInt(ab.Abnormal1002),
                4 => @floatFromInt(ab.Abnormal1001),
                5 => @floatFromInt(ab.Abnormal1005),
                else => @floatFromInt(ab.Abnormal1006),
            } else 0.0;
            const rate = p0 / 10000.0;
            const dmg_reduce = @min(@as(f64, @floatFromInt(attributes_helper.get_attr_value(target_attr, .DamageReduce))) / 10000.0, 1.0);
            const elem_reduce = @min(@as(f64, @floatFromInt(attributes_helper.get_attr_value(target_attr, attributes_helper.get_element_reduce(damage.Element)))) / 10000.0, 1.0);
            const special_change = @as(f64, @floatFromInt(attributes_helper.get_attr_value(attacker_attr, .SpecialDamageChange))) / 10000.0;
            const target_def = @max(0, attributes_helper.get_attr_value(target_attr, .Def));
            const ignore_def = std.math.clamp(@as(f64, @floatFromInt(attributes_helper.get_attr_value(attacker_attr, .IgnoreDefRate))) / 10000.0, 0.0, 1.0);
            const lv_term = 800.0 + 8.0 * @as(f64, @floatFromInt(attacker_lv));
            const def_mult = @min(2.0, 1.0 / (@as(f64, @floatFromInt(target_def)) * (1.0 - ignore_def) / lv_term + 1.0));
            const raw_res = attributes_helper.get_attr_value(target_attr, attributes_helper.get_element_res(damage.Element));
            const pen = attributes_helper.get_attr_value(attacker_attr, attributes_helper.get_element_pen(damage.Element));
            const res_mult = resistanceMultiplier(raw_res - pen);
            return @max(0.0, ab_base * rate * def_mult * res_mult * (1.0 - dmg_reduce) * (1.0 - elem_reduce) * (1.0 + @max(-1.0, special_change)));
        },
        else => return calculateHurtCore(rate_scaled, attacker_attr, target_attr, damage, is_crit),
    }
}

fn fluctuationFactor(damage: *const Assets.DataTables.Damage, request: pb.DamageExecuteRequest) f64 {
    const upper = levelSliceValue(damage.FluctuationUpper, request.SkillLevel, 10000);
    const lower = levelSliceValue(damage.FluctuationLower, request.SkillLevel, 10000);
    if (upper == lower) return @as(f64, @floatFromInt(lower)) * 0.0001;
    const seed_mod: i64 = @rem(@max(@as(i64, 0), @as(i64, request.RandomSeed)), 10000);
    const interp_bp = @as(f64, @floatFromInt(lower)) + (@as(f64, @floatFromInt(upper - lower)) * @as(f64, @floatFromInt(seed_mod)) / 10000.0);
    return interp_bp * 0.0001;
}

fn rollIsCrit(request: pb.DamageExecuteRequest, crit_rate: i32, io: std.Io) bool {
    if (request.ForceCritical) return true;
    if (crit_rate <= 0) return false;
    if (crit_rate >= 10000) return true;

    const roll: u64 = if (request.RandomSeed != 0) blk: {
        var h: u64 = @bitCast(@as(i64, request.RandomSeed));
        h ^= @as(u64, @bitCast(request.DamageId)) *% 0x9e3779b97f4a7c15;
        h = (h ^ (h >> 30)) *% 0xbf58476d1ce4e5b9;
        h ^= h >> 27;
        break :blk h % 10000;
    } else blk: {
        const now_ns: i96 = (std.Io.Clock.real).now(io).nanoseconds;
        var h: u64 = @truncate(@as(u128, @bitCast(@as(i128, now_ns))));
        h ^= @as(u64, @bitCast(request.DamageId)) *% 0x9e3779b97f4a7c15;
        h ^= @bitCast(request.TargetEntityId);
        h = (h ^ (h >> 30)) *% 0xbf58476d1ce4e5b9;
        h ^= h >> 27;
        break :blk h % 10000;
    };
    return roll < @as(u64, @intCast(crit_rate));
}

fn resistanceMultiplier(res_bp: i32) f64 {
    const res = @as(f64, @floatFromInt(res_bp)) / 10000.0;
    if (res < 0.0) return 1.0 - (res / 2.0);
    if (res < 0.8) return 1.0 - res;
    return 1.0 / (1.0 + 5.0 * res);
}

fn applyAttackerEnergy(
    combat_receive_pack: *std.ArrayList(pb.CombatReceiveData),
    request: pb.DamageExecuteRequest,
    damage: *const Assets.DataTables.Damage,
    attacker_entity: Entity,
    attacker_attr: *Entity.AttributeComponent,
    scene: *Scene,
    io: std.Io,
    alloc: mem.Alloc,
) !void {
    var woke = false;

    const base_concerto = levelSliceValue(damage.ElementPower, request.SkillLevel, 0);
    if (base_concerto > 0) {
        const raw_elem_eff = attributes_helper.get_attr_value(attacker_attr, .ElementEfficiency);
        const elem_eff = if (raw_elem_eff > 0) raw_elem_eff else 10000;
        const concerto = @as(i32, @intCast(@divTrunc(@as(i64, base_concerto) * elem_eff, 10000)));
        if (concerto > 0) {
            const change = try attributes_helper.change_attr(attacker_attr, .ElementEnergy, .Delta, .Current, concerto, alloc);
            if (change.items.len != 0) {
                woke = true;
                try attributes_helper.generate_attr_messages(combat_receive_pack, request.AttackerEntityId, attacker_attr, &change, alloc, io);
            }
        }
    }

    if (request.IsAddEnergy) {
        const base_energy = levelSliceValue(damage.Energy, request.SkillLevel, 0);
        if (base_energy > 0) {
            const raw_energy_eff = attributes_helper.get_attr_value(attacker_attr, .EnergyEfficiency);
            const energy_eff = if (raw_energy_eff > 0) raw_energy_eff else 10000;
            const energy_gain = @as(i32, @intCast(@divTrunc(@as(i64, base_energy) * energy_eff, 10000)));
            if (energy_gain > 0) {
                const e_change = try attributes_helper.change_attr(attacker_attr, .Energy, .Delta, .Current, energy_gain, alloc);
                if (e_change.items.len != 0) {
                    woke = true;
                    try attributes_helper.generate_attr_messages(combat_receive_pack, request.AttackerEntityId, attacker_attr, &e_change, alloc, io);
                }
            }
        }
    }

    if (woke) {
        try scene.markFsmDirty(alloc.gpa, attacker_entity.net_id, Entity.FsmComponent.WakeReason.attribute);
    }
}

fn defaultAttackerAttributes() Entity.AttributeComponent {
    var comp: Entity.AttributeComponent = .{};
    comp.attributes[@intCast(@intFromEnum(pb.EAttributeType.Lv))] = .{ .base = 1, .increment = 0, .current = 1 };
    comp.attributes[@intCast(@intFromEnum(pb.EAttributeType.Atk))] = .{ .base = 100, .increment = 0, .current = 100 };
    comp.attributes[@intCast(@intFromEnum(pb.EAttributeType.CritDamage))] = .{ .base = 15000, .increment = 0, .current = 15000 };
    return comp;
}

fn sanitizePartIndex(scene: *const Scene, target_net_id: i64, requested_part: i32) i32 {
    if (requested_part < 0) return -1;
    const idx = scene.net_id_map.get(target_net_id) orelse return -1;
    const part_comp = scene.entities.items(.part)[idx] orelse return -1;
    for (part_comp.parts) |p| {
        if (p.index == requested_part) return requested_part;
    }
    return -1;
}

pub fn damageEntity(
    combat_receive_pack: *std.ArrayList(pb.CombatReceiveData),
    request: pb.DamageExecuteRequest,
    damage: *const Assets.DataTables.Damage,
    assets: *const Assets,
    scene: *Scene,
    buff_timers: ?*BuffTimerScheduler,
    fs: *FileSystem,
    io: std.Io,
    query: Scene.Query(&.{
        Entity,
        *Entity.FightBuffComponent,
        *Entity.AttributeComponent,
    }),
    alloc: mem.Alloc,
) !pb.DamageExecuteNotify {
    _ = fs;

    const effective_part_index = sanitizePartIndex(scene, request.TargetEntityId, request.PartIndex);

    const fallback_attacker = defaultAttackerAttributes();
    const attacker_lookup = query.byNetId(request.AttackerEntityId);
    const attacker_attr: *const Entity.AttributeComponent = if (attacker_lookup) |comps| comps[2] else &fallback_attacker;

    const target_comps = query.byNetId(request.TargetEntityId) orelse {
        return .{
            .DamageId = request.DamageId,
            .AttackerEntityId = request.AttackerEntityId,
            .TargetEntityId = request.TargetEntityId,
            .PartIndex = effective_part_index,
            .SkillLevel = request.SkillLevel,
            .DamageContext = request.DamageContext,
            .ElementType = damage.Element,
        };
    };

    const target_entity, const target_buffs, const target_attr = target_comps;
    const before_hp = attributes_helper.get_attr_value(target_attr, .Life);
    if (before_hp <= 0) {
        return .{
            .DamageId = request.DamageId,
            .AttackerEntityId = request.AttackerEntityId,
            .TargetEntityId = request.TargetEntityId,
            .PartIndex = effective_part_index,
            .SkillLevel = request.SkillLevel,
            .DamageContext = request.DamageContext,
            .ElementType = damage.Element,
        };
    }

    if (damage.CalculateType == 0 and damage.ImmuneType != 0) {
        const invincible_tag_id = gameplay_tags.idFromName(&assets.tables.gameplay_tag_remap, "功能.逻辑状态标识.无敌") catch 1918148596;
        if (scene.entities.items(.tag)[target_entity.index]) |*target_tags| {
            if (target_tags.hasTag(&assets.tables.gameplay_tag_parent, invincible_tag_id)) {
                return .{
                    .DamageId = request.DamageId,
                    .AttackerEntityId = request.AttackerEntityId,
                    .TargetEntityId = request.TargetEntityId,
                    .PartIndex = effective_part_index,
                    .SkillLevel = request.SkillLevel,
                    .DamageContext = request.DamageContext,
                    .ImmuneType = .Invincible,
                    .ElementType = damage.Element,
                };
            }
        }
    }

    if (attacker_lookup) |comps| {
        try applyAttackerEnergy(combat_receive_pack, request, damage, comps[0], comps[2], scene, io, alloc);
    }

    const crit_rate = attributes_helper.get_attr_value(attacker_attr, .Crit);
    const is_crit = if (damage.CalculateType == 0) rollIsCrit(request, crit_rate, io) else false;
    const base_val = computeFormulaRaw(assets, damage, attacker_attr, target_attr, request.SkillLevel, is_crit);
    const fluct = fluctuationFactor(damage, request);
    var reported_value: i32 = 0;
    var killed_target = false;
    var change_life: i32 = 0;
    var shield_cover_damage: i32 = 0;

    if (damage.CalculateType == 1) {
        // Heal (cannot revive dead entities: already returned above if before_hp <= 0)
        const heal_change = attributes_helper.get_attr_value(attacker_attr, .HealChange);
        const healed_change = attributes_helper.get_attr_value(target_attr, .HealedChange);
        const heal_mult = @max(0.0, 1.0 + @as(f64, @floatFromInt(heal_change + healed_change)) / 10000.0);
        const heal_amount: i32 = @max(0, @as(i32, @intFromFloat(@ceil(base_val * heal_mult * fluct))));
        if (heal_amount > 0) {
            const target_change = try attributes_helper.change_attr(target_attr, .Life, .Delta, .Current, heal_amount, alloc);
            const after_hp = attributes_helper.get_attr_value(target_attr, .Life);
            change_life = after_hp - before_hp;
            reported_value = heal_amount;
            if (target_change.items.len != 0) {
                try attributes_helper.generate_attr_messages(combat_receive_pack, request.TargetEntityId, target_attr, &target_change, alloc, io);
                try scene.markFsmDirty(alloc.gpa, target_entity.net_id, Entity.FsmComponent.WakeReason.attribute);
            }
        }
    } else if (damage.CalculateType == 2) {
        // Utility HP drain (non-lethal self cost / skill sacrifice)
        const drain: i32 = @max(0, @as(i32, @intFromFloat(@ceil(base_val * fluct))));
        if (drain > 0 and before_hp > 1) {
            const actual_drain = @min(drain, before_hp - 1);
            if (actual_drain > 0) {
                const target_change = try attributes_helper.change_attr(target_attr, .Life, .Delta, .Current, -actual_drain, alloc);
                const after_hp = attributes_helper.get_attr_value(target_attr, .Life);
                change_life = after_hp - before_hp;
                reported_value = actual_drain;
                if (target_change.items.len != 0) {
                    try attributes_helper.generate_attr_messages(combat_receive_pack, request.TargetEntityId, target_attr, &target_change, alloc, io);
                    try scene.markFsmDirty(alloc.gpa, target_entity.net_id, Entity.FsmComponent.WakeReason.attribute);
                }
            }
        }
    } else {
        // Standard damage (CalculateType == 0)
        const raw_final = base_val * fluct;
        const final_damage: i32 = if (raw_final > 0.0) @max(1, @as(i32, @intFromFloat(@ceil(raw_final)))) else 0;
        reported_value = final_damage;

        if (final_damage > 0) {
            var hp_damage = final_damage;
            if (target_buffs.shields.len != 0) {
                const s_chg = attributes_helper.get_attr_value(attacker_attr, .SheildDamageChange);
                const s_red = attributes_helper.get_attr_value(target_attr, .SheildDamageReduce);
                const shield_mult = @max(0.1, @as(f64, @floatFromInt(10000 + s_chg - s_red)) / 10000.0);
                const shield_raw: i32 = @max(1, @as(i32, @intFromFloat(@round(@as(f64, @floatFromInt(final_damage)) * shield_mult))));
                const absorbed = try buff_helper.absorbDamageWithShields(
                    combat_receive_pack,
                    target_entity.net_id,
                    target_buffs,
                    target_attr,
                    shield_raw,
                    alloc,
                    io,
                );
                const equiv_hp: i32 = @min(
                    final_damage,
                    @as(i32, @intFromFloat(@round(@as(f64, @floatFromInt(absorbed)) / shield_mult))),
                );
                shield_cover_damage = equiv_hp;
                hp_damage = @max(0, final_damage - equiv_hp);
            }

            if (hp_damage > 0) {
                const target_change = try attributes_helper.change_attr(
                    target_attr,
                    .Life,
                    .Delta,
                    .Current,
                    -hp_damage,
                    alloc,
                );
                const after_hp = attributes_helper.get_attr_value(target_attr, .Life);
                change_life = after_hp - before_hp;
                if (target_change.items.len != 0) {
                    try attributes_helper.generate_attr_messages(
                        combat_receive_pack,
                        request.TargetEntityId,
                        target_attr,
                        &target_change,
                        alloc,
                        io,
                    );
                    if (after_hp > 0) {
                        try scene.markFsmDirty(
                            alloc.gpa,
                            target_entity.net_id,
                            Entity.FsmComponent.WakeReason.attribute,
                        );
                    }
                }
                if (after_hp <= 0) {
                    killed_target = true;
                    const removed_handles = try buff_helper.handleEntityDeath(
                        combat_receive_pack,
                        scene,
                        assets,
                        target_entity.net_id,
                        target_buffs,
                        target_attr,
                        alloc,
                        io,
                    );
                    if (buff_timers) |timers| {
                        for (removed_handles) |handle_id| {
                            timers.cancelHandle(target_entity.net_id, handle_id);
                        }
                    }
                } else {
                    try scene.setBattleEntityActive(alloc.gpa, target_entity.net_id, true);
                }
            } else {
                try scene.setBattleEntityActive(alloc.gpa, target_entity.net_id, true);
            }
            _ = try scene.appendBattleStateNotify(alloc.arena, combat_receive_pack);
        }
    }

    return .{
        .DamageId = request.DamageId,
        .AttackerEntityId = request.AttackerEntityId,
        .TargetEntityId = request.TargetEntityId,
        .Damage = reported_value,
        .PartIndex = effective_part_index,
        .IsCrit = is_crit,
        .KilledTarget = killed_target,
        .ShieldCoverDamage = shield_cover_damage,
        .SkillLevel = request.SkillLevel,
        .DamageContext = request.DamageContext,
        .ElementType = damage.Element,
        .ChangeLife = change_life,
    };
}
