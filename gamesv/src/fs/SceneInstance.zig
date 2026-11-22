const SceneInstance = @This();
const std = @import("std");

const Allocator = std.mem.Allocator;

pub const default: SceneInstance = .{};

owner_id: i32 = 0,
buff_handle: i32 = 0,
map_time: MapTime = .{},
players: []Player = &.{},
active_dungeon_id: i32 = 0,
prev_world_instance_id: i32 = 8,
prev_world_location: [3]f32 = .{ 0, 0, 0 },
prev_world_rotation: [3]f32 = .{ 0, 0, 0 },
respawn_cooldowns: []RespawnCooldown = &.{},

pub const RespawnCooldown = struct {
    config_id: i32,
    ready_at_ms: i64,
};

pub const MapTime = struct {
    hour: i32 = 8,
    minute: i32 = 0,
    owner_clock_time_span: i64 = 0,
    day: u32 = 1,
};

pub const Player = struct {
    id: i32,
    group_type: i32 = 1,
    location: [3]f32,
    rotation: [3]f32,
};

pub fn isOnRespawnCooldown(inst: *const SceneInstance, config_id: i32, now_ms: i64) bool {
    if (config_id <= 0) return false;
    for (inst.respawn_cooldowns) |entry| {
        if (entry.config_id == config_id) return now_ms < entry.ready_at_ms;
    }
    return false;
}

pub fn setRespawnCooldown(inst: *SceneInstance, gpa: Allocator, config_id: i32, now_ms: i64, ready_at_ms: i64) !void {
    if (config_id <= 0 or ready_at_ms <= now_ms) return;

    var active_count: usize = 0;
    var found = false;
    for (inst.respawn_cooldowns) |*entry| {
        if (entry.config_id == config_id) {
            entry.ready_at_ms = ready_at_ms;
            found = true;
        }
        if (entry.ready_at_ms > now_ms) active_count += 1;
    }
    if (found and active_count == inst.respawn_cooldowns.len) return;

    const new_len = active_count + @intFromBool(!found);
    if (new_len == 0) {
        if (inst.respawn_cooldowns.len != 0) gpa.free(inst.respawn_cooldowns);
        inst.respawn_cooldowns = &.{};
        return;
    }

    const next = try gpa.alloc(RespawnCooldown, new_len);
    var write_idx: usize = 0;
    for (inst.respawn_cooldowns) |entry| {
        if (entry.ready_at_ms <= now_ms) continue;
        next[write_idx] = entry;
        write_idx += 1;
    }
    if (!found) {
        next[write_idx] = .{ .config_id = config_id, .ready_at_ms = ready_at_ms };
    }
    if (inst.respawn_cooldowns.len != 0) gpa.free(inst.respawn_cooldowns);
    inst.respawn_cooldowns = next;
}

pub fn deinit(inst: SceneInstance, gpa: Allocator) void {
    std.zon.parse.free(gpa, inst);
}
