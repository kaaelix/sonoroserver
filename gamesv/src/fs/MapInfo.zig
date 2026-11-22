const MapInfo = @This();
const std = @import("std");
const pb = @import("proto").pb;

const Allocator = std.mem.Allocator;

pub const default: MapInfo = .{};
pub const data_path = "map_info";

fn customMarkState(state: i32) pb.MarkPointState {
    return switch (state) {
        1 => .MarkDisable,
        2 => .MarkComplete,
        else => .MarkNormal,
    };
}

pub const CustomMark = struct {
    id: i32,
    x: f32,
    y: f32,
    z: f32,
    is_trace: i32,
    mark_type: i32,
    map_id: i32,
    config_id: i32,
    entity_config_id: i32 = 0,
    locked: bool,
    state: i32,

    pub fn toPb(mark: CustomMark) pb.MarkPointInfo {
        return .{
            .PosX = mark.x,
            .PosY = mark.y,
            .PosZ = mark.z,
            .ConfigId = mark.config_id,
            .MarkId = mark.id,
            .MapId = mark.map_id,
            .MarkType = mark.mark_type,
            .EntityConfigId = mark.entity_config_id,
            .IsServerDisable = mark.locked,
            .MarkPointState = customMarkState(mark.state),
        };
    }
};

pub const TempTeleport = struct {
    id: i64,
    mark_id: i32 = 0,
    map_id: i32 = 8,
    inst_id: i32 = 8,
    area_id: i32 = 0,
    pos_x: f32 = 0,
    pos_y: f32 = 0,
    pos_z: f32 = 0,
    rot_pitch: f32 = 0,
    rot_yaw: f32 = 0,
    rot_roll: f32 = 0,

    pub fn toPb(tp: TempTeleport) pb.TemporaryTeleportInfo {
        return .{
            .MarkId = tp.mark_id,
            .TemporaryTeleportId = tp.id,
            .MapId = tp.map_id,
            .InstId = tp.inst_id,
            .Pos = .{ .X = tp.pos_x, .Y = tp.pos_y, .Z = tp.pos_z },
            .Rot = .{ .Pitch = tp.rot_pitch, .Yaw = tp.rot_yaw, .Roll = tp.rot_roll },
            .PlayerTeleportPos = .{ .X = tp.pos_x, .Y = tp.pos_y, .Z = tp.pos_z },
            .Area = tp.area_id,
        };
    }
};

custom_marks: []CustomMark = &.{},
tracked_mark_ids: []i32 = &.{},
unlocked_fogs: []i32 = &.{},
unlocked_teleporters: []i32 = &.{},
unlocked_areas: []i32 = &.{},
claimed_area_rewards: []i32 = &.{},
temp_teleports: []TempTeleport = &.{},
next_custom_mark_id: i32 = 1_000_000,
next_temp_teleport_id: i64 = 1,

fn containsId(ids: []const i32, id: i32) bool {
    for (ids) |existing| {
        if (existing == id) return true;
    }

    return false;
}

fn containsAnyId(ids: []const i32, candidates: []const i32) bool {
    for (candidates) |id| {
        if (containsId(ids, id)) return true;
    }

    return false;
}

fn emptyCustomMarks() []CustomMark {
    return @constCast(&.{});
}

fn emptyTrackedIds() []i32 {
    return @constCast(&.{});
}

fn replaceIds(info: *MapInfo, gpa: Allocator, ids: []i32) void {
    if (info.tracked_mark_ids.len != 0) gpa.free(info.tracked_mark_ids);
    info.tracked_mark_ids = ids;
}

pub fn addCustomMark(info: *MapInfo, gpa: Allocator, request: pb.MarkPointRequestInfo) !CustomMark {
    const mark_id = info.next_custom_mark_id;
    info.next_custom_mark_id += 1;

    const mark: CustomMark = .{
        .id = mark_id,
        .x = request.PosX,
        .y = request.PosY,
        .z = request.PosZ,
        .is_trace = if (request.IsTrace) 1 else 0,
        .mark_type = request.MarkType,
        .map_id = request.MapId,
        .config_id = request.ConfigId,
        .entity_config_id = request.EntityConfigId,
        .locked = false,
        .state = 0,
    };

    const marks = try gpa.alloc(CustomMark, info.custom_marks.len + 1);
    @memcpy(marks[0..info.custom_marks.len], info.custom_marks);
    marks[info.custom_marks.len] = mark;

    if (info.custom_marks.len != 0) gpa.free(info.custom_marks);
    info.custom_marks = marks;

    return mark;
}

pub fn deleteCustomMarks(info: *MapInfo, gpa: Allocator, mark_ids: []const i32) !void {
    if (mark_ids.len == 0) return;

    var mark_count: usize = 0;
    for (info.custom_marks) |mark| {
        if (!containsId(mark_ids, mark.id)) mark_count += 1;
    }

    if (mark_count != info.custom_marks.len) {
        const marks = if (mark_count == 0) emptyCustomMarks() else marks: {
            const marks = try gpa.alloc(CustomMark, mark_count);
            var write_index: usize = 0;
            for (info.custom_marks) |mark| {
                if (containsId(mark_ids, mark.id)) continue;
                marks[write_index] = mark;
                write_index += 1;
            }
            break :marks marks;
        };

        if (info.custom_marks.len != 0) gpa.free(info.custom_marks);
        info.custom_marks = marks;
    }

    if (containsAnyId(info.tracked_mark_ids, mark_ids)) {
        var track_count: usize = 0;
        for (info.tracked_mark_ids) |mark_id| {
            if (!containsId(mark_ids, mark_id)) track_count += 1;
        }

        const tracked = if (track_count == 0) emptyTrackedIds() else tracked: {
            const tracked = try gpa.alloc(i32, track_count);
            var write_index: usize = 0;
            for (info.tracked_mark_ids) |mark_id| {
                if (containsId(mark_ids, mark_id)) continue;
                tracked[write_index] = mark_id;
                write_index += 1;
            }
            break :tracked tracked;
        };

        replaceIds(info, gpa, tracked);
    }
}

pub fn trackMark(info: *MapInfo, gpa: Allocator, mark_id: i32) !void {
    if (mark_id == 0 or containsId(info.tracked_mark_ids, mark_id)) return;

    const ids = try gpa.alloc(i32, info.tracked_mark_ids.len + 1);
    @memcpy(ids[0..info.tracked_mark_ids.len], info.tracked_mark_ids);
    ids[info.tracked_mark_ids.len] = mark_id;
    replaceIds(info, gpa, ids);
}

pub fn untrackMark(info: *MapInfo, gpa: Allocator, mark_id: i32) !void {
    if (!containsId(info.tracked_mark_ids, mark_id)) return;

    if (info.tracked_mark_ids.len == 1) {
        replaceIds(info, gpa, emptyTrackedIds());
        return;
    }

    var write_index: usize = 0;
    const ids = try gpa.alloc(i32, info.tracked_mark_ids.len - 1);
    for (info.tracked_mark_ids) |existing| {
        if (existing == mark_id) continue;
        ids[write_index] = existing;
        write_index += 1;
    }

    replaceIds(info, gpa, ids);
}

pub fn ensureDefaultUnlocks(info: *MapInfo, gpa: Allocator) !bool {
    _ = info;
    _ = gpa;
    // Fresh players start with undiscovered fog and teleporter towers; they unlock when interacted with or reached.
    return false;
}

fn appendUniqueId(slice: *[]i32, gpa: Allocator, id: i32) !bool {
    if (id <= 0 or containsId(slice.*, id)) return false;

    const ids = try gpa.alloc(i32, slice.*.len + 1);
    @memcpy(ids[0..slice.*.len], slice.*);
    ids[slice.*.len] = id;
    if (slice.*.len != 0) gpa.free(slice.*);
    slice.* = ids;
    return true;
}

pub fn unlockFog(info: *MapInfo, gpa: Allocator, fog_id: i32) !bool {
    return appendUniqueId(&info.unlocked_fogs, gpa, fog_id);
}

pub fn unlockTeleporter(info: *MapInfo, gpa: Allocator, tp_id: i32) !bool {
    return appendUniqueId(&info.unlocked_teleporters, gpa, tp_id);
}

pub fn unlockArea(info: *MapInfo, gpa: Allocator, area_id: i32) !bool {
    return appendUniqueId(&info.unlocked_areas, gpa, area_id);
}

pub fn claimAreaReward(info: *MapInfo, gpa: Allocator, reward_id: i32) !bool {
    return appendUniqueId(&info.claimed_area_rewards, gpa, reward_id);
}

pub fn addTempTeleport(
    info: *MapInfo,
    gpa: Allocator,
    map_id: i32,
    area_id: i32,
    pos: pb.Vector,
    rot: pb.Rotator,
) !TempTeleport {
    const id = info.next_temp_teleport_id;
    info.next_temp_teleport_id += 1;
    const mark_id = info.next_custom_mark_id;
    info.next_custom_mark_id += 1;

    const tp: TempTeleport = .{
        .id = id,
        .mark_id = mark_id,
        .map_id = map_id,
        .inst_id = map_id,
        .area_id = area_id,
        .pos_x = pos.X,
        .pos_y = pos.Y,
        .pos_z = pos.Z,
        .rot_pitch = rot.Pitch,
        .rot_yaw = rot.Yaw,
        .rot_roll = rot.Roll,
    };
    const next = try gpa.alloc(TempTeleport, info.temp_teleports.len + 1);
    @memcpy(next[0..info.temp_teleports.len], info.temp_teleports);
    next[info.temp_teleports.len] = tp;
    if (info.temp_teleports.len != 0) gpa.free(info.temp_teleports);
    info.temp_teleports = next;
    return tp;
}

pub fn findTempTeleport(info: MapInfo, id: i64) ?TempTeleport {
    for (info.temp_teleports) |tp| {
        if (tp.id == id) return tp;
    }
    return null;
}

pub fn removeTempTeleport(info: *MapInfo, gpa: Allocator, id: i64) !bool {
    for (info.temp_teleports, 0..) |tp, idx| {
        if (tp.id != id) continue;
        if (info.temp_teleports.len == 1) {
            gpa.free(info.temp_teleports);
            info.temp_teleports = &.{};
            return true;
        }
        const shrunk = try gpa.alloc(TempTeleport, info.temp_teleports.len - 1);
        @memcpy(shrunk[0..idx], info.temp_teleports[0..idx]);
        @memcpy(shrunk[idx..], info.temp_teleports[idx + 1 ..]);
        gpa.free(info.temp_teleports);
        info.temp_teleports = shrunk;
        return true;
    }
    return false;
}

pub fn deinit(info: MapInfo, gpa: Allocator) void {
    std.zon.parse.free(gpa, info);
}

test "MapInfo CustomMark EntityConfigId and TempTeleport lifecycle" {
    const gpa = std.testing.allocator;
    var info: MapInfo = .{};
    defer info.deinit(gpa);

    const mark = try info.addCustomMark(gpa, .{
        .PosX = 12.5,
        .PosY = 34.5,
        .PosZ = 56.5,
        .ConfigId = 42,
        .MarkType = 1,
        .IsTrace = true,
        .MapId = 8,
        .EntityConfigId = 900123,
    });
    const pb_mark = mark.toPb();
    try std.testing.expectEqual(@as(i32, 900123), pb_mark.EntityConfigId);
    try std.testing.expectEqual(@as(i32, 1_000_000), pb_mark.MarkId);

    const tp = try info.addTempTeleport(gpa, 8, 12, .{ .X = 100, .Y = 200, .Z = 300 }, .{ .Yaw = 45 });
    try std.testing.expectEqual(@as(i64, 1), tp.id);
    try std.testing.expectEqual(@as(i32, 12), tp.area_id);
    try std.testing.expect(info.findTempTeleport(tp.id) != null);
    const pb_tp = tp.toPb();
    try std.testing.expectEqual(@as(i64, 1), pb_tp.TemporaryTeleportId);
    try std.testing.expect(try info.removeTempTeleport(gpa, tp.id));
    try std.testing.expect(info.findTempTeleport(tp.id) == null);
}
