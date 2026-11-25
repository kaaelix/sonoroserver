const PlayerQuestComponent = @This();
const std = @import("std");
const common = @import("common");
const file_util = @import("../../../fs/file_util.zig");
const comp_util = @import("../comp_util.zig");
const QuestInfo = @import("../../../fs/QuestInfo.zig");
const save_block = @import("../../helpers/save_block.zig");

const Allocator = std.mem.Allocator;
const FileSystem = common.FileSystem;

/// Work left over from the login restore of save blocks (see `logic/helpers/save_block.zig`).
/// Lives only for the session and is never persisted.
pub const LoginRestore = struct {
    /// Quest trees whose save block was reset and must be entered again once a scene exists.
    reenter_quests: std.ArrayList(i32) = .empty,
    /// Position rollback asked by one of those blocks, applied when the player joins the scene.
    pos_rollback: ?save_block.PosRollback = null,
};

player_id: i32,
info: QuestInfo,
login_restore: LoginRestore = .{},

pub fn init(gpa: Allocator, fs: *FileSystem, player_id: i32) !PlayerQuestComponent {
    var arena = std.heap.ArenaAllocator.init(gpa);
    defer arena.deinit();

    return .{
        .player_id = player_id,
        .info = try file_util.loadOrCreateZon(
            QuestInfo,
            gpa,
            arena.allocator(),
            fs,
            "player/{}/{s}",
            .{ player_id, QuestInfo.data_path },
        ),
    };
}

pub fn save(comp: *const PlayerQuestComponent, fs: *FileSystem, arena: Allocator) !void {
    const path = try std.fmt.allocPrint(arena, "player/{}/{s}", .{ comp.player_id, QuestInfo.data_path });
    try comp_util.saveStruct(fs, comp.info, path, arena);
}

pub fn deinit(comp: *PlayerQuestComponent, gpa: Allocator) void {
    comp.login_restore.reenter_quests.deinit(gpa);
    comp.info.deinit(gpa);
}
