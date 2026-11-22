const std = @import("std");
const Io = std.Io;
const Allocator = std.mem.Allocator;

pub const GmAction = enum {
    get_state,
    exec_cmd,
    give_item,
    set_union_level,
    set_world_level,
    set_role_level,
    unlock_roles,
    role_max,
    role_chain,
    weapon_max,
    quest_pick,
    quest_step,
    quest_skip,
    unlock_tp,
    unlock_func,
    time_set,
    heal_team,
    reset_cd,
    no_cd_mode,
    spawn_echo,
    reset_player,
    rotate_gm_key,
};

pub const GmJob = struct {
    action: GmAction,
    target_str: []const u8 = "",
    int_a: i32 = 0,
    int_b: i32 = 0,
    int_c: i32 = 0,
    flag: bool = false,
    raw_cmd: []const u8 = "",

    done: Io.Event = .unset,
    ok: bool = false,
    response_gpa: Allocator,
    response_json: []u8 = &.{},

    pub fn failAndSignal(job: *GmJob, io: Io, msg: []const u8) void {
        const ErrPayload = struct {
            ok: bool = false,
            message: []const u8,
        };
        var out: Io.Writer.Allocating = .init(job.response_gpa);
        out.writer.print("{f}", .{std.json.fmt(ErrPayload{ .message = msg }, .{})}) catch {
            out.deinit();
            job.ok = false;
            job.done.set(io);
            return;
        };
        job.response_json = out.toOwnedSlice() catch &.{};
        job.ok = false;
        job.done.set(io);
    }
};
