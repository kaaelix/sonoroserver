const std = @import("std");
const EventQueue = @import("../EventQueue.zig");
const State = @import("../../network/State.zig");
const mem = @import("../../mem.zig");

pub fn chatCommandResponse(
    event: EventQueue.Dequeue(.chat_command_response),
    state: *State,
    alloc: mem.Alloc,
) !void {
    if (state.gm_capture) {
        if (state.gm_output.items.len > 0) {
            try state.gm_output.append(alloc.arena, '\n');
        }
        try state.gm_output.appendSlice(alloc.arena, event.data.content);
        return;
    }
    std.log.scoped(.gm).info("{s}", .{event.data.content});
}
