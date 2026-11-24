const std = @import("std");
const common = @import("common");
const proto = @import("proto");
const mem = @import("../../mem.zig");
const PlayerProgressComponent = @import("../../logic/component/player/PlayerProgressComponent.zig");

const FileSystem = common.FileSystem;

/// Encodes a protobuf sub-message as base64 and persists it in `PlayerProgressComponent`.
pub fn saveMsg(
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
    key: []const u8,
    msg: anytype,
) !void {
    var aw = std.Io.Writer.Allocating.init(alloc.arena);
    try proto.encodeMessage(&aw.writer, msg, proto.pb_desc);
    const bytes = aw.written();
    const enc_len = std.base64.standard.Encoder.calcSize(bytes.len);
    const b64 = try alloc.arena.alloc(u8, enc_len);
    _ = std.base64.standard.Encoder.encode(b64, bytes);
    try pc.info.setKeyStr(alloc.gpa, key, b64);
    try pc.save(fs, alloc.arena);
}

/// Decodes a persisted protobuf sub-message from `PlayerProgressComponent`, or null if unset/corrupt.
pub fn loadMsg(
    arena: std.mem.Allocator,
    pc: *const PlayerProgressComponent,
    key: []const u8,
    comptime T: type,
) ?T {
    const b64 = pc.info.getKeyStr(key);
    if (b64.len == 0) return null;
    const dec_len = std.base64.standard.Decoder.calcSizeForSlice(b64) catch return null;
    const buf = arena.alloc(u8, dec_len) catch return null;
    std.base64.standard.Decoder.decode(buf, b64) catch return null;
    var r = std.Io.Reader.fixed(buf);
    return proto.decodeMessage(&r, arena, T, proto.pb_desc) catch null;
}

/// Decodes `msg_opt`, persists it under `key`, and reports `.Success`.
pub fn persistOptMsg(
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
    key: []const u8,
    msg_opt: anytype,
) !void {
    if (msg_opt) |msg| {
        try saveMsg(alloc, fs, pc, key, msg);
    }
}

/// Persists an integer value under a formatted `prefix:id` key and saves `PlayerProgressComponent`.
pub fn persistInt(
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
    key: []const u8,
    val: i64,
) !void {
    try pc.info.setKeyInt(alloc.gpa, key, val);
    try pc.save(fs, alloc.arena);
}

/// Persists a one-time claim/flag key and saves `PlayerProgressComponent`.
pub fn persistClaim(
    alloc: mem.Alloc,
    fs: *FileSystem,
    pc: *PlayerProgressComponent,
    key: []const u8,
) !void {
    _ = try pc.info.markClaim(alloc.gpa, key);
    try pc.save(fs, alloc.arena);
}

/// Appends persisted `ClientStorageInfo` entries (`cstorage:msg:*`, `cstorage:i:*`, `cstorage:s:*`) without duplicating SystemIds.
pub fn appendStoredClientStorageInfos(
    arena: std.mem.Allocator,
    progress_comp: *const PlayerProgressComponent,
    infos: *std.ArrayList(proto.pb.ClientStorageInfo),
) !void {
    var seen_sys: std.AutoHashMapUnmanaged(i32, void) = .empty;
    for (infos.items) |existing| {
        try seen_sys.put(arena, existing.SystemId, {});
    }
    for (progress_comp.info.dynamic.strs) |entry| {
        if (!std.mem.startsWith(u8, entry.key, "cstorage:msg:")) continue;
        const sys_id = std.fmt.parseInt(i32, entry.key["cstorage:msg:".len..], 10) catch continue;
        if (seen_sys.contains(sys_id)) continue;
        if (loadMsg(arena, progress_comp, entry.key, proto.pb.ClientStorageInfo)) |msg| {
            try seen_sys.put(arena, sys_id, {});
            try infos.append(arena, msg);
        }
    }
    for (progress_comp.info.dynamic.ints) |entry| {
        if (!std.mem.startsWith(u8, entry.key, "cstorage:i:")) continue;
        const sys_id = std.fmt.parseInt(i32, entry.key["cstorage:i:".len..], 10) catch continue;
        if (seen_sys.contains(sys_id)) continue;
        try seen_sys.put(arena, sys_id, {});
        try infos.append(arena, .{
            .SystemId = sys_id,
            .Data = .{ .LongData = .{ .Data = entry.val } },
        });
    }
    for (progress_comp.info.dynamic.strs) |entry| {
        if (!std.mem.startsWith(u8, entry.key, "cstorage:s:")) continue;
        const sys_id = std.fmt.parseInt(i32, entry.key["cstorage:s:".len..], 10) catch continue;
        if (seen_sys.contains(sys_id)) continue;
        try seen_sys.put(arena, sys_id, {});
        try infos.append(arena, .{
            .SystemId = sys_id,
            .Data = .{ .StringData = .{ .Data = try arena.dupe(u8, entry.val) } },
        });
    }
}
