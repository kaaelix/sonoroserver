const std = @import("std");
const common = @import("common");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const Assets = @import("../../data/Assets.zig");
const PlayerMusicComponent = @import("../../logic/component/player/PlayerMusicComponent.zig");
const Transaction = @import("../handlers.zig").Transaction;

const FileSystem = common.FileSystem;

fn appendUnlockedMusicIds(assets: *const Assets, arena: std.mem.Allocator) !std.ArrayList(i32) {
    var ids: std.ArrayList(i32) = .empty;
    for (assets.tables.phonograph_music.items) |music| {
        try ids.append(arena, music.Id);
    }
    return ids;
}

fn appendAlbumTimeInfos(assets: *const Assets, arena: std.mem.Allocator) !std.ArrayList(pb.PhonographAlbumTimeInfo) {
    var albums: std.ArrayList(pb.PhonographAlbumTimeInfo) = .empty;
    for (assets.tables.phonograph_music.items) |music| {
        for (music.Album) |album_id| {
            var index: ?usize = null;
            for (albums.items, 0..) |album, i| {
                if (album.AlbumId == album_id) {
                    index = i;
                    break;
                }
            }

            const album_index = index orelse blk: {
                try albums.append(arena, .{ .AlbumId = album_id });
                break :blk albums.items.len - 1;
            };
            try albums.items[album_index].MusicIds.append(arena, music.Id);
        }
    }

    for (assets.tables.phonograph_album.items) |album| {
        if (!std.mem.eql(u8, album.Title, "MotorMusicFavourite")) continue;

        for (albums.items) |existing| {
            if (existing.AlbumId == album.Id) break;
        } else {
            try albums.append(arena, .{ .AlbumId = album.Id });
        }
        break;
    }

    return albums;
}

fn containsId(ids: []const i32, id: i32) bool {
    for (ids) |existing| {
        if (existing == id) return true;
    }
    return false;
}

fn appendFavoriteMusicIds(arena: std.mem.Allocator, source: []const i32, valid_ids: []const i32) !std.ArrayList(i32) {
    var ids: std.ArrayList(i32) = .empty;
    for (source) |id| {
        if (!containsId(valid_ids, id) or containsId(ids.items, id)) continue;
        try ids.append(arena, id);
    }
    return ids;
}

pub fn onGetMusicInfoRequest(
    txn: *Transaction(pb.GetMusicInfoRequest),
    music_comp: *PlayerMusicComponent,
    assets: *const Assets,
    alloc: mem.Alloc,
) !void {
    const music_ids = try appendUnlockedMusicIds(assets, alloc.arena);
    const album_time_infos = try appendAlbumTimeInfos(assets, alloc.arena);
    const favorite_music_ids = try appendFavoriteMusicIds(
        alloc.arena,
        music_comp.info.favorite_music_ids,
        music_ids.items,
    );

    txn.respond(.{
        .ErrorCode = .Success,
        .AlbumTimeInfos = album_time_infos,
        .CollectMusicIds = favorite_music_ids,
    });
}

pub fn onFavoriteMusicMotorCycleUpdateRequest(
    txn: *Transaction(pb.FavoriteMusicMotorCycleUpdateRequest),
    music_comp: *PlayerMusicComponent,
    assets: *const Assets,
    fs: *FileSystem,
    alloc: mem.Alloc,
) !void {
    const music_ids = try appendUnlockedMusicIds(assets, alloc.arena);
    var favorites: std.ArrayList(i32) = .empty;
    for (txn.message.FavoriteMusicList.items) |id| {
        if (!containsId(music_ids.items, id) or containsId(favorites.items, id)) continue;
        try favorites.append(alloc.gpa, id);
    }

    music_comp.info.replaceFavoriteMusicIds(alloc.gpa, try favorites.toOwnedSlice(alloc.gpa));
    try music_comp.save(fs, alloc.arena);

    txn.respond(.{ .ErrorCode = .Success });
}
