const Component = @This();
const std = @import("std");
const pb = @import("proto").pb;
const mem = @import("../../../mem.zig");

weapon_id: i32 = 0,
hatred_group_id: i64 = 0,
ai_team_init_id: i32 = 0,
combat_message_id: i64 = 0,
hatred_id: i64 = 0,
perception_ids: [8]i32 = @splat(0),
perception_len: u8 = 0,

pub fn toProto(comp: Component, arena: std.mem.Allocator) !pb.MonsterAiComponentPb {
    var perceptions: std.ArrayList(i32) = .empty;
    const len: usize = @min(@as(usize, comp.perception_len), comp.perception_ids.len);
    for (comp.perception_ids[0..len]) |pid| {
        if (pid != 0) try perceptions.append(arena, pid);
    }
    return .{
        .WeaponId = comp.weapon_id,
        .HatredGroupId = comp.hatred_group_id,
        .AiTeamInitId = comp.ai_team_init_id,
        .CombatMessageId = comp.combat_message_id,
        .BasicPerceptionIds = perceptions,
        .HatredId = comp.hatred_id,
    };
}
