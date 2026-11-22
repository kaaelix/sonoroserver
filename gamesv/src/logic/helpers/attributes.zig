const std = @import("std");
const mem = @import("../../mem.zig");
const AttributeComponent = @import("../component/entity/AttributeComponent.zig");
const pb = @import("proto").pb;

pub const AttributeValueType = enum {
    Base,
    Current,
    Increment,
};

pub const AttributeChangeType = enum {
    Delta,
    Override,
};

pub const RelatedAttrKind = enum {
    Max,
    Recover,
};

pub fn gameplayAttributeData(attr_type: pb.EAttributeType, attr: AttributeComponent.Attribute) pb.GameplayAttributeData {
    const ratio_attr = switch (attr_type) {
        .Atk, .LifeMax, .Def => true,
        else => false,
    };
    return .{
        .AttributeType = attr_type,
        .BaseValue = if (ratio_attr) attr.base else attr.current,
        .CurrentValue = attr.current,
    };
}

pub fn get_element_bonus(element_type: i32) pb.EAttributeType {
    return switch (element_type) {
        0 => .DamageChangePhys,
        1 => .DamageChangeElement1,
        2 => .DamageChangeElement2,
        3 => .DamageChangeElement3,
        4 => .DamageChangeElement4,
        5 => .DamageChangeElement5,
        6 => .DamageChangeElement6,
        else => .DamageChangePhys,
    };
}

pub fn get_element_res(element_type: i32) pb.EAttributeType {
    return switch (element_type) {
        0 => .DamageResistancePhys,
        1 => .DamageResistanceElement1,
        2 => .DamageResistanceElement2,
        3 => .DamageResistanceElement3,
        4 => .DamageResistanceElement4,
        5 => .DamageResistanceElement5,
        6 => .DamageResistanceElement6,
        else => .DamageResistancePhys,
    };
}

pub fn get_element_pen(element_type: i32) pb.EAttributeType {
    return switch (element_type) {
        0 => .IgnoreDamageResistancePhys,
        1 => .IgnoreDamageResistanceElement1,
        2 => .IgnoreDamageResistanceElement2,
        3 => .IgnoreDamageResistanceElement3,
        4 => .IgnoreDamageResistanceElement4,
        5 => .IgnoreDamageResistanceElement5,
        6 => .IgnoreDamageResistanceElement6,
        else => .IgnoreDamageResistancePhys,
    };
}

pub fn get_element_reduce(element_type: i32) pb.EAttributeType {
    return switch (element_type) {
        0 => .DamageReducePhys,
        1 => .DamageReduceElement1,
        2 => .DamageReduceElement2,
        3 => .DamageReduceElement3,
        4 => .DamageReduceElement4,
        5 => .DamageReduceElement5,
        6 => .DamageReduceElement6,
        else => .DamageReducePhys,
    };
}

pub fn get_ability_bonus(ability_type: i32) pb.EAttributeType {
    return switch (ability_type) {
        0 => .DamageChangeAuto, // Based on Basic Attack
        1 => .DamageChangeCast, // Based on Heavy Attack
        2 => .DamageChangeUltra, // Based on Liberation Skill
        3 => .DamageChangeQte, // Based on QTE / Intro Skill
        4 => .DamageChangeNormalSkill, // Based on Resonance Skill
        5 => .DamageChangePhantom, // Based on Phantom Skill
        6 => .None, // Based on Field/Utility
        7 => .None, // Based on Outro Skill
        8 => .None, // Based on Coordinated Attacks
        9 => .None, // Based on Heal/Shield
        10 => .None, // Based on Negative Status
        11 => .None, // Based on Tune Break
        12 => .None, // Based on Tune Rupture
        else => .None,
    };
}

pub fn attrFromId(attr_id: i32) pb.EAttributeType {
    if (attr_id <= 0 or attr_id >= @intFromEnum(pb.EAttributeType.MAX)) return .None;
    return std.enums.fromInt(pb.EAttributeType, attr_id) orelse .None;
}

pub fn get_attr_value(attr_comp: *const AttributeComponent, attr_type: pb.EAttributeType) i32 {
    if (attr_type == .None or attr_type == .MAX) return 0;
    const idx = @intFromEnum(attr_type);
    if (idx < 0) return 0;
    const u_idx: usize = @intCast(idx);
    if (u_idx >= attr_comp.attributes.len) return 0;
    return attr_comp.attributes[u_idx].current;
}

pub fn get_attr_by_id(attr_comp: *const AttributeComponent, attr_id: i32) i32 {
    return get_attr_value(attr_comp, attrFromId(attr_id));
}

pub fn set_life_unconditional(attr_comp: *AttributeComponent, hp: i32) i32 {
    const life_idx = @intFromEnum(pb.EAttributeType.Life);
    const max_idx = @intFromEnum(pb.EAttributeType.LifeMax);
    if (attr_comp.attributes.len <= life_idx or attr_comp.attributes.len <= max_idx) return 0;
    const max_hp = @max(0, attr_comp.attributes[max_idx].current);
    const clamped = std.math.clamp(hp, 0, max_hp);
    attr_comp.attributes[life_idx].base = clamped;
    attr_comp.attributes[life_idx].increment = 0;
    attr_comp.attributes[life_idx].current = clamped;
    return clamped;
}

pub fn revive_entity(attr_comp: *AttributeComponent, hp: ?i32) bool {
    const life_idx = @intFromEnum(pb.EAttributeType.Life);
    const max_idx = @intFromEnum(pb.EAttributeType.LifeMax);
    if (attr_comp.attributes.len <= life_idx or attr_comp.attributes.len <= max_idx) return false;
    const was_dead = attr_comp.attributes[life_idx].current <= 0;
    const max_hp = @max(1, attr_comp.attributes[max_idx].current);
    const target_hp = if (hp) |v| std.math.clamp(v, 1, max_hp) else max_hp;
    _ = set_life_unconditional(attr_comp, target_hp);
    return was_dead;
}

pub fn get_related_attr(
    attr_type: pb.EAttributeType,
    kind: RelatedAttrKind,
    allocator: std.mem.Allocator,
) !?pb.EAttributeType {
    const suffix = switch (kind) {
        .Max => "Max",
        .Recover => "Recover",
    };
    const attr_name = @tagName(attr_type);
    const attr_name_max = try std.fmt.allocPrint(allocator, "{s}{s}", .{ attr_name, suffix });
    defer allocator.free(attr_name_max);
    return std.meta.stringToEnum(pb.EAttributeType, attr_name_max);
}

const ATTR_ALLOW_NEGATIVES: []const pb.EAttributeType = &.{
    .Crit,
    .DamageReduce,
    .DamageReducePhys,
    .DamageReduceElement1,
    .DamageReduceElement2,
    .DamageReduceElement3,
    .DamageReduceElement4,
    .DamageReduceElement5,
    .DamageReduceElement6,
    .DamageResistancePhys,
    .DamageResistanceElement1,
    .DamageResistanceElement2,
    .DamageResistanceElement3,
    .DamageResistanceElement4,
    .DamageResistanceElement5,
    .DamageResistanceElement6,
    .ParalysisTimeRecover,
};

fn clamp_attr(
    attr_type: pb.EAttributeType,
    value: i32,
    max_value: ?i32,
) i32 {
    switch (attr_type) {
        .AutoAttackSpeed, .CastAttackSpeed => return std.math.clamp(value, 0, 20000),
        .DamageReduce => return std.math.clamp(value, -10000, 10000),
        else => {
            if (max_value) |max| {
                return std.math.clamp(value, 0, max);
            }
            const allows_negative = std.mem.indexOfScalar(
                pb.EAttributeType,
                ATTR_ALLOW_NEGATIVES,
                attr_type,
            ) != null;
            return if (allows_negative) value else @max(value, 0);
        },
    }
}

pub const AttributeChange = struct {
    changes: std.ArrayList(pb.GameplayAttributeData),
    recovers: std.ArrayList(pb.RecoverPropFromServer),
};

pub fn change_attr(
    attr_comp: *AttributeComponent,
    attr_type: pb.EAttributeType,
    change_type: AttributeChangeType,
    value_type: AttributeValueType,
    value: i32,
    alloc: mem.Alloc,
) !std.ArrayList(pb.EAttributeType) {
    const life_idx = @intFromEnum(pb.EAttributeType.Life);
    const life_max_idx = @intFromEnum(pb.EAttributeType.LifeMax);
    const is_alive = attr_comp.attributes.len > life_idx and
        attr_comp.attributes[life_idx].current > 0;

    if (attr_type == .Life and !is_alive) {
        return .empty;
    }

    const max_value: ?i32 = blk: {
        const maybe_max_attr = try get_related_attr(attr_type, .Max, alloc.gpa);
        if (maybe_max_attr) |max_attr| {
            const max_idx = @intFromEnum(max_attr);
            if (max_idx >= 0 and @as(usize, @intCast(max_idx)) < attr_comp.attributes.len) {
                break :blk attr_comp.attributes[@intCast(max_idx)].current;
            }
        }
        break :blk null;
    };

    const hp_ratio_before: ?f32 = if (attr_type == .LifeMax) ratio: {
        if (life_idx < attr_comp.attributes.len and life_max_idx < attr_comp.attributes.len) {
            const life = attr_comp.attributes[life_idx].current;
            const life_max = attr_comp.attributes[life_max_idx].current;
            break :ratio if (life_max > 0)
                @as(f32, @floatFromInt(life)) / @as(f32, @floatFromInt(life_max))
            else
                null;
        }
        break :ratio null;
    } else null;

    const result = apply_attr_change(
        attr_comp,
        attr_type,
        change_type,
        value_type,
        value,
        max_value,
        alloc,
    );

    if (hp_ratio_before) |ratio| {
        if (life_max_idx < attr_comp.attributes.len and life_idx < attr_comp.attributes.len) {
            const new_max = attr_comp.attributes[life_max_idx].current;
            const new_hp: i32 = @as(i32, @round(@as(f32, @floatFromInt(new_max)) * ratio));
            const clamped = std.math.clamp(new_hp, 0, new_max);
            attr_comp.attributes[life_idx].base = clamped;
            attr_comp.attributes[life_idx].current = clamped;
        }
    }

    return result;
}

pub fn apply_attr_change(
    attr_comp: *AttributeComponent,
    attr_type: pb.EAttributeType,
    change_type: AttributeChangeType,
    value_type: AttributeValueType,
    value: i32,
    max_value: ?i32,
    alloc: mem.Alloc,
) !std.ArrayList(pb.EAttributeType) {
    var change: std.ArrayList(pb.EAttributeType) = .empty;

    const idx = @intFromEnum(attr_type);
    if (idx < 0 or @as(usize, @intCast(idx)) >= attr_comp.attributes.len) return change;

    const attr = &attr_comp.attributes[@intCast(idx)];

    switch (change_type) {
        .Override => switch (value_type) {
            .Base => {
                attr.base = clamp_attr(attr_type, value, max_value);
                attr.current = clamp_attr(attr_type, attr.base + attr.increment, max_value);
            },
            .Increment => {
                attr.increment = value;
                attr.current = clamp_attr(attr_type, attr.base + attr.increment, max_value);
            },
            .Current => {
                attr.current = clamp_attr(attr_type, value, max_value);
            },
        },
        .Delta => switch (value_type) {
            .Base => {
                attr.base = clamp_attr(attr_type, attr.base + value, max_value);
                attr.current = clamp_attr(attr_type, attr.base + attr.increment, max_value);
            },
            .Increment => {
                attr.increment += value;
                attr.current = clamp_attr(attr_type, attr.base + attr.increment, max_value);
            },
            .Current => {
                attr.current = clamp_attr(attr_type, attr.current + value, max_value);
            },
        },
    }

    try change.append(alloc.arena, attr_type);

    return change;
}

pub fn generate_attr_messages(
    combat_receive_pack: *std.ArrayList(pb.CombatReceiveData),
    entity_id: i64,
    attr_comp: *AttributeComponent,
    changed: *const std.ArrayList(pb.EAttributeType),
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    if (changed.items.len == 0) return;

    var changes: std.ArrayList(pb.GameplayAttributeData) = .empty;
    var recovers: std.ArrayList(pb.RecoverPropFromServer) = .empty;

    const recover_types = &[_]pb.EAttributeType{ .Hardness, .Tough, .ParalysisTime, .Rage };

    for (changed.items) |attr_type| {
        const idx = @intFromEnum(attr_type);
        if (idx < 0 or @as(usize, @intCast(idx)) >= attr_comp.attributes.len) continue;
        const attr = &attr_comp.attributes[@intCast(idx)];

        if (std.mem.indexOfScalar(pb.EAttributeType, recover_types, attr_type) != null) {
            const max_val = blk: {
                if (try get_related_attr(attr_type, .Max, alloc.gpa)) |max_attr| {
                    const midx = @intFromEnum(max_attr);
                    if (midx >= 0 and @as(usize, @intCast(midx)) < attr_comp.attributes.len) break :blk attr_comp.attributes[@intCast(midx)].current;
                }
                break :blk 0;
            };
            const recover_val = blk: {
                if (try get_related_attr(attr_type, .Recover, alloc.gpa)) |rec_attr| {
                    const ridx = @intFromEnum(rec_attr);
                    if (ridx >= 0 and @as(usize, @intCast(ridx)) < attr_comp.attributes.len) break :blk attr_comp.attributes[@intCast(ridx)].current;
                }
                break :blk 0;
            };
            try recovers.append(alloc.arena, .{
                .AttrId = @intCast(idx),
                .Ratio = recover_val,
                .MaxValue = max_val,
                .ValueIncrement = attr.current,
            });
        }

        try changes.append(alloc.arena, gameplayAttributeData(attr_type, attr.*));
    }

    const realtime_clock: std.Io.Clock = .real;
    if (changes.items.len != 0) {
        try combat_receive_pack.append(alloc.arena, .{
            .Message = .{
                .CombatNotifyData = .{
                    .CombatCommon = .{ .EntityId = entity_id },
                    .Message = .{ .AttributeChangedNotify = .{ .Attributes = changes } },
                },
            },
        });
    }
    if (recovers.items.len != 0) {
        try combat_receive_pack.append(alloc.arena, .{
            .Message = .{
                .CombatNotifyData = .{
                    .CombatCommon = .{ .EntityId = entity_id },
                    .Message = .{
                        .RecoverPropChangedNotify = .{
                            .Attributes = recovers,
                            .Duration = realtime_clock.now(io).toMilliseconds(),
                        },
                    },
                },
            },
        });
    }
}
