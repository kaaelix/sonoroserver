const common = @import("common");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const PlayerProgressComponent = @import("../../logic/component/player/PlayerProgressComponent.zig");
const Transaction = @import("../handlers.zig").Transaction;

const FileSystem = common.FileSystem;

pub fn onPlayerVoxelEnvRequest(
    txn: *Transaction(pb.PlayerVoxelEnvRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "player:voxel_env", txn.message.VoxelEnv);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .ErrorCode = .Success,
        .VoxelEnv = txn.message.VoxelEnv,
    });
}
