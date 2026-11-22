const std = @import("std");
const proto = @import("proto");
const pb = proto.pb;
const mem = @import("../../mem.zig");
const Assets = @import("../../data/Assets.zig");
const PlayerID = @import("../../logic/PlayerID.zig");
const PlayerProgressComponent = @import("../../logic/component/player/PlayerProgressComponent.zig");
const activity = @import("activity.zig");
const aki_logic_bridge = @import("../aki_logic_bridge.zig");
const Transaction = @import("../handlers.zig").Transaction;

pub fn filterDirectTrainActivities(
    arena: std.mem.Allocator,
    source: []const pb.ActivityData,
) !std.ArrayList(pb.ActivityData) {
    var filtered: std.ArrayList(pb.ActivityData) = .empty;
    for (source) |act| {
        if (act.Type == .ThroughTrain) {
            try filtered.append(arena, act);
        }
    }
    return filtered;
}

pub fn onDirectTrainGetPlayerIdRequest(
    txn: *Transaction(pb.DirectTrainGetPlayerIdRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
    progress_comp: *const PlayerProgressComponent,
    player_id: PlayerID,
    io: std.Io,
) !void {
    if (aki_logic_bridge.callPlayer(io, alloc.arena, player_id.id, 0, proto.pb_desc.ActivityRequest.msg_id, &.{})) |resp| {
        if (resp.msg_id == proto.pb_desc.ActivityResponse.msg_id and resp.payload.len > 0) {
            var reader = std.Io.Reader.fixed(resp.payload);
            if (proto.decodeMessage(&reader, alloc.arena, pb.ActivityResponse, proto.pb_desc)) |act_resp| {
                txn.respond(.{
                    .Activitys = try filterDirectTrainActivities(alloc.arena, act_resp.Activities.items),
                });
                return;
            } else |_| {}
        }
    } else |_| {}

    const all = try activity.buildActivityList(alloc.arena, alloc.gpa, assets, progress_comp, null, io);
    txn.respond(.{
        .Activitys = try filterDirectTrainActivities(alloc.arena, all.items),
    });
}
