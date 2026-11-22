const Component = @This();
const pb = @import("proto").pb;
const std = @import("std");
const Assets = @import("../../../data/Assets.zig");
const sliceToArrayList = @import("EntityComponentStorage.zig").sliceToArrayList;

skill_component_pb: []pb.SkillComponentPb = &.{},

pub fn deinit(comp: *Component, gpa: std.mem.Allocator) void {
    gpa.free(comp.skill_component_pb);
}

pub fn toProto(comp: Component) !pb.PassiveGaSkillComponentPb {
    return .{
        .SkillInfoList = .empty,
        .SkillComponentPb = sliceToArrayList(pb.SkillComponentPb, comp.skill_component_pb),
    };
}

pub fn toPassiveSkillProto(comp: Component, arena: std.mem.Allocator, assets: *const Assets, net_id: i64) !pb.PassiveSkillComponentPb {
    var list: std.ArrayList(pb.PassiveSkillItemPb) = .empty;
    for (comp.skill_component_pb) |entry| {
        if (assets.tables.passive_skill.getDataById(entry.SkillId) == null) continue;
        try list.append(arena, .{
            .CombatCommon = .{ .EntityId = net_id, .MessageId = -1, .PreMessageId = -1 },
            .SkillId = entry.SkillId,
        });
    }
    return .{
        .PassiveSkillItemPbList = list,
    };
}

pub fn addSkill(comp: *Component, gpa: std.mem.Allocator, skill_id: i32) !bool {
    if (skill_id == 0) return false;
    for (comp.skill_component_pb) |entry| {
        if (entry.SkillId == skill_id) return false;
    }
    const old = comp.skill_component_pb;
    const next = try gpa.alloc(pb.SkillComponentPb, old.len + 1);
    @memcpy(next[0..old.len], old);
    next[old.len] = .{ .SkillId = skill_id, .ConstateId = -1 };
    gpa.free(old);
    comp.skill_component_pb = next;
    return true;
}

pub fn removeSkill(comp: *Component, gpa: std.mem.Allocator, skill_id: i32) bool {
    const idx = for (comp.skill_component_pb, 0..) |entry, i| {
        if (entry.SkillId == skill_id) break i;
    } else return false;
    const items = comp.skill_component_pb;
    items[idx] = items[items.len - 1];
    comp.skill_component_pb = gpa.realloc(items, items.len - 1) catch items[0 .. items.len - 1];
    return true;
}
