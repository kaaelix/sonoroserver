const std = @import("std");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const FileSystem = @import("common").FileSystem;
const Transaction = @import("../handlers.zig").Transaction;
const PlayerBasicComponent = @import("../../logic/component/player/PlayerBasicComponent.zig");
const EnergyHelper = @import("../../logic/helpers/energy.zig");

pub fn onEnergySyncRequest(
    txn: *Transaction(pb.EnergySyncRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    basic_comp: *PlayerBasicComponent,
    io: std.Io,
) !void {
    try EnergyHelper.syncAndSave(alloc.arena, fs, basic_comp, io);
    const list = try EnergyHelper.buildEnergyList(
        alloc.arena,
        basic_comp,
        txn.message.EnergyTypes.items,
    );
    txn.respond(.{
        .ErrorCode = .Success,
        .SyncInfo = list,
    });
}
