const std = @import("std");
const pb = @import("proto").pb;
const common = @import("common");
const mem = @import("../../mem.zig");
const FileSystem = common.FileSystem;
const Connection = @import("../../network/Connection.zig");
const comp_util = @import("../component/comp_util.zig");
const PlayerBasicComponent = @import("../component/player/PlayerBasicComponent.zig");

pub const type_waveplate: i32 = 5;
pub const type_crystalwave_plate: i32 = 6;

pub const waveplate_natural_cap: i32 = 240;
pub const waveplate_renew_seconds: i64 = 360;
pub const waveplate_hard_cap: i32 = 2400;

pub const crystalwave_cap: i32 = 480;
pub const crystalwave_renew_seconds: i64 = 720;

pub fn nowSeconds(io: std.Io) i64 {
    return @divFloor(std.Io.Clock.real.now(io).toMilliseconds(), std.time.ms_per_s);
}

fn toClientTime(ts: i64) i32 {
    return @intCast(std.math.clamp(ts, 0, std.math.maxInt(i32)));
}

pub fn syncEnergy(basic_comp: *PlayerBasicComponent, now_sec: i64) bool {
    const info = &basic_comp.info;
    var changed = false;

    if (info.last_renew_energy_time <= 0) {
        info.last_renew_energy_time = now_sec;
        changed = true;
    }
    if (info.last_store_energy_time <= 0) {
        info.last_store_energy_time = now_sec;
        changed = true;
    }

    if (info.energy_count < waveplate_natural_cap) {
        if (now_sec > info.last_renew_energy_time) {
            const elapsed = now_sec - info.last_renew_energy_time;
            const gained = @divFloor(elapsed, waveplate_renew_seconds);
            if (gained > 0) {
                const space: i64 = waveplate_natural_cap - info.energy_count;
                if (gained < space) {
                    info.energy_count += @intCast(gained);
                    info.last_renew_energy_time += gained * waveplate_renew_seconds;
                } else {
                    const reach_cap_time = info.last_renew_energy_time + space * waveplate_renew_seconds;
                    info.energy_count = waveplate_natural_cap;
                    info.last_renew_energy_time = now_sec;
                    info.last_store_energy_time = reach_cap_time;
                }
                changed = true;
            }
        }
    }

    if (info.energy_count >= waveplate_natural_cap) {
        if (info.last_renew_energy_time != now_sec) {
            info.last_renew_energy_time = now_sec;
            changed = true;
        }
        if (info.store_energy_count < crystalwave_cap and now_sec > info.last_store_energy_time) {
            const elapsed = now_sec - info.last_store_energy_time;
            const gained = @divFloor(elapsed, crystalwave_renew_seconds);
            if (gained > 0) {
                const space: i64 = crystalwave_cap - info.store_energy_count;
                const add = @min(space, gained);
                info.store_energy_count += @intCast(add);
                if (info.store_energy_count >= crystalwave_cap) {
                    info.last_store_energy_time = now_sec;
                } else {
                    info.last_store_energy_time += add * crystalwave_renew_seconds;
                }
                changed = true;
            }
        } else if (info.store_energy_count >= crystalwave_cap and info.last_store_energy_time != now_sec) {
            info.last_store_energy_time = now_sec;
            changed = true;
        }
    } else if (info.last_store_energy_time != now_sec) {
        info.last_store_energy_time = now_sec;
        changed = true;
    }

    return changed;
}

pub fn saveBasicInfo(arena: std.mem.Allocator, fs: *FileSystem, basic_comp: *const PlayerBasicComponent) !void {
    const path = try std.fmt.allocPrint(arena, "player/{}/basic_info", .{basic_comp.player_id});
    try comp_util.saveStruct(fs, basic_comp.info, path, arena);
}

pub fn syncAndSave(arena: std.mem.Allocator, fs: *FileSystem, basic_comp: *PlayerBasicComponent, io: std.Io) !void {
    if (syncEnergy(basic_comp, nowSeconds(io))) {
        try saveBasicInfo(arena, fs, basic_comp);
    }
}

pub fn canAddWaveplate(basic_comp: *PlayerBasicComponent, io: std.Io, amount: i32) bool {
    if (amount <= 0) return false;
    _ = syncEnergy(basic_comp, nowSeconds(io));
    return basic_comp.info.energy_count < waveplate_hard_cap and
        @as(i64, basic_comp.info.energy_count) + amount <= waveplate_hard_cap;
}

pub fn addWaveplate(
    conn: *Connection,
    alloc: mem.Alloc,
    fs: *FileSystem,
    basic_comp: *PlayerBasicComponent,
    amount: i32,
    io: std.Io,
) !bool {
    if (amount <= 0) return false;
    const now_sec = nowSeconds(io);
    _ = syncEnergy(basic_comp, now_sec);
    if (basic_comp.info.energy_count >= waveplate_hard_cap or
        @as(i64, basic_comp.info.energy_count) + amount > waveplate_hard_cap)
    {
        return false;
    }
    const was_below_cap = basic_comp.info.energy_count < waveplate_natural_cap;
    basic_comp.info.energy_count += amount;
    if (was_below_cap and basic_comp.info.energy_count >= waveplate_natural_cap) {
        basic_comp.info.last_renew_energy_time = now_sec;
        basic_comp.info.last_store_energy_time = now_sec;
    }
    try saveBasicInfo(alloc.arena, fs, basic_comp);
    try pushEnergyNotify(conn, alloc, basic_comp, io);
    return true;
}

pub fn consumeWaveplate(
    conn: *Connection,
    alloc: mem.Alloc,
    fs: *FileSystem,
    basic_comp: *PlayerBasicComponent,
    amount: i32,
    io: std.Io,
) !bool {
    if (amount <= 0) return true;
    const now_sec = nowSeconds(io);
    _ = syncEnergy(basic_comp, now_sec);
    if (basic_comp.info.energy_count < amount) return false;

    const was_at_or_above_cap = basic_comp.info.energy_count >= waveplate_natural_cap;
    basic_comp.info.energy_count -= amount;
    if (was_at_or_above_cap and basic_comp.info.energy_count < waveplate_natural_cap) {
        basic_comp.info.last_renew_energy_time = now_sec;
        basic_comp.info.last_store_energy_time = now_sec;
    }
    try saveBasicInfo(alloc.arena, fs, basic_comp);
    try pushEnergyNotify(conn, alloc, basic_comp, io);
    return true;
}

pub fn buildEnergyList(arena: std.mem.Allocator, basic_comp: *const PlayerBasicComponent, filter_types: []const i32) !std.ArrayList(pb.EnergyInfo) {
    var list: std.ArrayList(pb.EnergyInfo) = .empty;
    const want_waveplate = filter_types.len == 0 or std.mem.indexOfScalar(i32, filter_types, type_waveplate) != null;
    const want_store = filter_types.len == 0 or std.mem.indexOfScalar(i32, filter_types, type_crystalwave_plate) != null;

    if (want_waveplate) {
        try list.append(arena, .{
            .EnergyCount = basic_comp.info.energy_count,
            .LastRenewEnergyTime = toClientTime(basic_comp.info.last_renew_energy_time),
            .EnergyType = type_waveplate,
        });
    }
    if (want_store) {
        try list.append(arena, .{
            .EnergyCount = basic_comp.info.store_energy_count,
            .LastRenewEnergyTime = toClientTime(basic_comp.info.last_store_energy_time),
            .EnergyType = type_crystalwave_plate,
        });
    }
    return list;
}

pub fn pushEnergyNotify(
    conn: *Connection,
    alloc: mem.Alloc,
    basic_comp: *const PlayerBasicComponent,
    io: std.Io,
) !void {
    const info_list = try buildEnergyList(alloc.arena, basic_comp, &.{});
    try conn.push(alloc.gpa, io, pb.EnergyUpdateNotify{
        .UpdateInfo = info_list,
    });
}
