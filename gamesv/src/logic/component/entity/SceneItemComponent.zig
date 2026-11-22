const std = @import("std");
const pb = @import("proto").pb;
const ProgressInfo = @import("../../../fs/ProgressInfo.zig");

pub const transient = true;

pos_sender: i32 = 0,
manipulatable_state: ?i32 = null,

pub fn fromProgress(info: ProgressInfo, player_id: i32, config_id: i32) @This() {
    var result: @This() = .{ .pos_sender = player_id };
    var key_buf: [64]u8 = undefined;
    const key = std.fmt.bufPrint(&key_buf, "item_bb:{d}:0", .{config_id}) catch return result;
    for (info.dynamic.ints) |entry| {
        if (std.mem.eql(u8, entry.key, key)) {
            result.manipulatable_state = std.math.cast(i32, entry.val);
            break;
        }
    }
    return result;
}

pub fn toProto(component: @This(), arena: std.mem.Allocator) !pb.SceneItemComponentPb {
    var blackboards: std.ArrayList(pb.SceneItemBlackboardParam) = .empty;
    if (component.manipulatable_state) |state| {
        try blackboards.append(arena, .{
            .Key = .ManipulatableState,
            .Value = .{ .IntValue = state },
        });
    }
    return .{
        .PosSender = component.pos_sender,
        .BlackBoards = blackboards,
    };
}
