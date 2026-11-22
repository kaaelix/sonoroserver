const std = @import("std");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const Assets = @import("../../data/Assets.zig");
const Transaction = @import("../handlers.zig").Transaction;

pub fn onAccessPathTimeServerConfigRequest(
    txn: *Transaction(pb.AccessPathTimeServerConfigRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
    io: std.Io,
) !void {
    var cfgs: std.ArrayList(pb.AccessPathTimeServerConfig) = .empty;
    for (assets.reward_index.getRows(io, "AccessPath")) |row| {
        const id: i32 = @intCast(row.key);
        if (id <= 0) continue;
        try cfgs.append(alloc.arena, .{ .Id = id, .BeginTime = 1, .EndTime = 2_000_000_000 });
        if (cfgs.items.len >= 128) break;
    }
    txn.respond(.{ .AccessPathTimeServerConfig = cfgs });
}
