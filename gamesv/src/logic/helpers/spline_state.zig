const std = @import("std");
const pb = @import("proto").pb;
const ProgressInfo = @import("../../fs/ProgressInfo.zig");

const absent_value = std.math.minInt(i64);

pub fn persistMoveSplineConfig(
    gpa: std.mem.Allocator,
    arena: std.mem.Allocator,
    info: *ProgressInfo,
    config_id: i32,
    config_opt: ?pb.MoveSplineConfig,
) !void {
    if (config_id <= 0) return;
    const config = config_opt orelse pb.MoveSplineConfig{};

    var start: i64 = absent_value;
    var end: i64 = absent_value;
    var look_dir: i64 = absent_value;
    var cycle: i64 = absent_value;
    var circle: i64 = absent_value;
    if (config.StartPoint) |value| switch (value) {
        .StartPointIndex => |index| start = index,
    };
    if (config.EndPoint) |value| switch (value) {
        .EndPointIndex => |index| end = index,
    };
    if (config.LookDir) |value| switch (value) {
        .IsLookDir => |enabled| look_dir = @intFromBool(enabled),
    };
    if (config.Cycle) |value| switch (value) {
        .CycleCount => |count| cycle = count,
    };
    if (config.Circle) |value| switch (value) {
        .IsCircle => |enabled| circle = @intFromBool(enabled),
    };

    const fields = [_][]const u8{ "start", "end", "look_dir", "cycle", "circle" };
    const values = [_]i64{ start, end, look_dir, cycle, circle };
    for (fields, values) |field, value| {
        const key = try std.fmt.allocPrint(arena, "scene_item_spline_opt:{d}:{s}", .{ config_id, field });
        try info.setKeyInt(gpa, key, value);
    }
    const written_key = try std.fmt.allocPrint(arena, "scene_item_spline_opt:{d}:written", .{config_id});
    try info.setKeyInt(gpa, written_key, 1);
}

pub fn restoreMoveSplineConfig(
    arena: std.mem.Allocator,
    info: ProgressInfo,
    config_id: i32,
) !?pb.MoveSplineConfig {
    if (config_id <= 0) return null;
    const written_key = try std.fmt.allocPrint(arena, "scene_item_spline_opt:{d}:written", .{config_id});
    if (info.getKeyInt(written_key) != 1) return null;

    var config: pb.MoveSplineConfig = .{};
    var has_config = false;
    const start_key = try std.fmt.allocPrint(arena, "scene_item_spline_opt:{d}:start", .{config_id});
    const start = info.getKeyInt(start_key);
    if (start != absent_value) {
        config.StartPoint = .{ .StartPointIndex = std.math.cast(i32, start) orelse return null };
        has_config = true;
    }
    const end_key = try std.fmt.allocPrint(arena, "scene_item_spline_opt:{d}:end", .{config_id});
    const end = info.getKeyInt(end_key);
    if (end != absent_value) {
        config.EndPoint = .{ .EndPointIndex = std.math.cast(i32, end) orelse return null };
        has_config = true;
    }
    const look_dir_key = try std.fmt.allocPrint(arena, "scene_item_spline_opt:{d}:look_dir", .{config_id});
    const look_dir = info.getKeyInt(look_dir_key);
    if (look_dir != absent_value) {
        config.LookDir = .{ .IsLookDir = look_dir != 0 };
        has_config = true;
    }
    const cycle_key = try std.fmt.allocPrint(arena, "scene_item_spline_opt:{d}:cycle", .{config_id});
    const cycle = info.getKeyInt(cycle_key);
    if (cycle != absent_value) {
        config.Cycle = .{ .CycleCount = std.math.cast(i32, cycle) orelse return null };
        has_config = true;
    }
    const circle_key = try std.fmt.allocPrint(arena, "scene_item_spline_opt:{d}:circle", .{config_id});
    const circle = info.getKeyInt(circle_key);
    if (circle != absent_value) {
        config.Circle = .{ .IsCircle = circle != 0 };
        has_config = true;
    }

    return if (has_config) config else null;
}
