const PlayerProgressComponent = @This();
const std = @import("std");
const common = @import("common");
const file_util = @import("../../../fs/file_util.zig");
const comp_util = @import("../comp_util.zig");
const ProgressInfo = @import("../../../fs/ProgressInfo.zig");

const Allocator = std.mem.Allocator;
const FileSystem = common.FileSystem;

player_id: i32,
info: ProgressInfo,

pub fn init(gpa: Allocator, fs: *FileSystem, player_id: i32) !PlayerProgressComponent {
    var arena_state = std.heap.ArenaAllocator.init(gpa);
    defer arena_state.deinit();
    const arena = arena_state.allocator();

    return .{
        .player_id = player_id,
        .info = try file_util.loadOrCreateZon(
            ProgressInfo,
            gpa,
            arena,
            fs,
            "player/{}/{s}",
            .{ player_id, ProgressInfo.data_path },
        ),
    };
}

pub fn save(comp: *const PlayerProgressComponent, fs: *FileSystem, arena: Allocator) !void {
    const path = try std.fmt.allocPrint(arena, "player/{d}/{s}", .{ comp.player_id, ProgressInfo.data_path });
    try comp_util.saveStruct(fs, comp.info, path, arena);
}

pub fn deinit(comp: *PlayerProgressComponent, gpa: Allocator) void {
    comp.info.deinit(gpa);
}
