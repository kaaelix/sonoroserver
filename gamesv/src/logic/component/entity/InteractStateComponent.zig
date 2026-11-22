const std = @import("std");
const pb = @import("proto").pb;
const InteractConfig = @import("../../../data/tables/entity_components/Interact.zig");

pub const transient = true;

random_option_indices: []i32 = &.{},

fn supportedFlowOption(entry: InteractConfig.RandomInteractOption) bool {
    const option = entry.Option orelse return false;
    const option_type = option.Type orelse return false;
    if (std.mem.eql(u8, option_type.Type, "Flow")) {
        const flow = option_type.Flow orelse return false;
        return flow.FlowListName.len > 0 and flow.FlowId > 0;
    }
    if (std.mem.eql(u8, option_type.Type, "Actions")) {
        for (option_type.Actions) |action| {
            if (!std.mem.eql(u8, action.Name, "PlayFlow")) continue;
            const params = action.Params orelse continue;
            if (params.FlowListName.len > 0 and params.FlowId > 0) return true;
        }
    }
    return false;
}

pub fn fromConfig(
    gpa: std.mem.Allocator,
    config: InteractConfig,
    player_id: i32,
    scene_id: i32,
    config_id: i32,
) !@This() {
    if (config.Disabled orelse false) return .{};
    const random = config.RandomInteract orelse return .{};
    if (random.RandomCount <= 0 or random.Options.len == 0) return .{};

    var scratch = std.heap.ArenaAllocator.init(gpa);
    defer scratch.deinit();
    const selected_flags = try scratch.allocator().alloc(bool, random.Options.len);
    @memset(selected_flags, false);

    var selected: std.ArrayList(i32) = .empty;
    errdefer selected.deinit(gpa);

    var random_state: u64 =
        (@as(u64, @bitCast(@as(i64, player_id))) *% 0x9e3779b97f4a7c15) ^
        (@as(u64, @bitCast(@as(i64, scene_id))) *% 0xd1b54a32d192ed03) ^
        (@as(u64, @bitCast(@as(i64, config_id))) *% 0x94d049bb133111eb);
    if (random_state == 0) random_state = 0xa0761d6478bd642f;

    const draw_count = @min(@as(usize, @intCast(random.RandomCount)), random.Options.len);
    for (0..draw_count) |_| {
        var total_weight: u64 = 0;
        for (random.Options, selected_flags) |option, already_selected| {
            if (already_selected or option.Weight <= 0 or !supportedFlowOption(option)) continue;
            total_weight +|= @intCast(option.Weight);
        }
        if (total_weight == 0) break;

        random_state ^= random_state >> 12;
        random_state ^= random_state << 25;
        random_state ^= random_state >> 27;
        const roll = (random_state *% 0x2545f4914f6cdd1d) % total_weight;
        var remaining_roll = roll;
        var selected_index: ?usize = null;
        for (random.Options, selected_flags, 0..) |option, already_selected, index| {
            if (already_selected or option.Weight <= 0 or !supportedFlowOption(option)) continue;
            const weight: u64 = @intCast(option.Weight);
            if (remaining_roll < weight) {
                selected_index = index;
                break;
            }
            remaining_roll -= weight;
        }
        const index = selected_index orelse break;
        selected_flags[index] = true;
        try selected.append(gpa, @intCast(index));
    }

    return .{ .random_option_indices = try selected.toOwnedSlice(gpa) };
}

pub fn deinit(component: *@This(), gpa: std.mem.Allocator) void {
    if (component.random_option_indices.len > 0) gpa.free(component.random_option_indices);
    component.random_option_indices = &.{};
}

pub fn toProto(component: @This()) pb.InteractComponentPb {
    return .{
        .RandomInteractIndex = .{
            .items = component.random_option_indices,
            .capacity = component.random_option_indices.len,
        },
        .Interacting = false,
    };
}
