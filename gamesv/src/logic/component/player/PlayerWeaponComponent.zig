const PlayerWeaponComponent = @This();
const std = @import("std");
const common = @import("common");
const comp_util = @import("../comp_util.zig");
const file_util = @import("../../../fs/file_util.zig");

const Assets = @import("../../../data/Assets.zig");
const WeaponItem = @import("../../../fs/WeaponItem.zig");
const special_item_incr = @import("../../../fs/special_item_incr.zig");
const Allocator = std.mem.Allocator;
const FileSystem = common.FileSystem;

player_id: i32,
weapon_map: std.array_hash_map.Auto(i32, WeaponItem),

pub fn init(gpa: Allocator, fs: *FileSystem, assets: *const Assets, player_id: i32) !PlayerWeaponComponent {
    var weapon_map = try comp_util.loadItems(WeaponItem, gpa, fs, assets, player_id, .by_incr_id);
    errdefer comp_util.freeMap(gpa, &weapon_map);

    try special_item_incr.ensureAtLeast(gpa, fs, player_id, special_item_incr.nextAfterMap(weapon_map));

    return .{
        .player_id = player_id,
        .weapon_map = weapon_map,
    };
}

pub fn deinit(comp: *PlayerWeaponComponent, gpa: Allocator) void {
    comp_util.freeMap(gpa, &comp.weapon_map);
}

pub fn saveAll(
    gpa: Allocator,
    fs: *FileSystem,
    player_id: i32,
    weapon_map: std.array_hash_map.Auto(i32, WeaponItem),
) !void {
    var arena_state = std.heap.ArenaAllocator.init(gpa);
    defer arena_state.deinit();
    const arena = arena_state.allocator();

    const data_dir_path = try std.fmt.allocPrint(arena, "player/{}/{s}", .{ player_id, WeaponItem.data_dir });
    if (try fs.readDir(data_dir_path)) |dir| {
        defer dir.deinit();
        for (dir.entries) |entry| if (entry.kind == .file) {
            const incr_id = std.fmt.parseInt(i32, entry.basename(), 10) catch continue;
            if (incr_id >= special_item_incr.trial_incr_min) continue;
            if (!weapon_map.contains(incr_id)) {
                const rm_path = try std.fmt.allocPrint(arena, "player/{}/{s}/{}", .{ player_id, WeaponItem.data_dir, incr_id });
                try fs.deleteFile(rm_path);
            }
        };
    }

    var iterator = weapon_map.iterator();
    while (iterator.next()) |kv| {
        if (kv.key_ptr.* >= special_item_incr.trial_incr_min) continue;
        try comp_util.saveStruct(
            fs,
            kv.value_ptr.*,
            try std.fmt.allocPrint(arena, "player/{}/{s}/{}", .{ player_id, WeaponItem.data_dir, kv.key_ptr.* }),
            arena,
        );
    }
}

pub fn deleteWeaponFile(
    gpa: Allocator,
    fs: *FileSystem,
    player_id: i32,
    incr_id: i32,
) !void {
    if (incr_id >= special_item_incr.trial_incr_min) return;
    const path = try std.fmt.allocPrint(gpa, "player/{}/{s}/{}", .{ player_id, WeaponItem.data_dir, incr_id });
    defer gpa.free(path);
    try fs.deleteFile(path);
}
