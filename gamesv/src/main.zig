const std = @import("std");
const common = @import("common");
const network = @import("network.zig");
const GmHttp = @import("network/GmHttp.zig");
const Assets = @import("data/Assets.zig");
const SqliteStore = @import("data/SqliteStore.zig");

const Io = std.Io;
const Allocator = std.mem.Allocator;

const FileSystem = common.FileSystem;
const zigaction = common.zigaction;

const Args = struct {
    state_dir: []const u8 = "state",
    bind_address: []const u8 = "127.0.0.1:13100",
    gm_bind_address: []const u8 = "127.0.0.1:13101",
};

fn init(gpa: Allocator, io: Io, environ: std.process.Environ, cmd_args: std.process.Args) u8 {
    const log = std.log.scoped(.init);

    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const sliced_args = cmd_args.toSlice(arena.allocator()) catch |err| {
        log.err("failed to slice args: {t}", .{err});
        return 1;
    };

    const args = common.args.parse(Args, sliced_args[1..]) orelse {
        common.args.printUsage(Args, sliced_args[0]);
        return 1;
    };

    var environ_map = std.process.Environ.createMap(environ, arena.allocator()) catch |err| {
        log.err("failed to read environment: {t}", .{err});
        return 1;
    };
    defer environ_map.deinit();
    @import("network/aki_logic_bridge.zig").configureFromEnvironment(&environ_map);

    var assets = Assets.init(gpa, io) catch |err| {
        log.err("failed to load assets: {t}", .{err});
        return 1;
    };

    defer assets.deinit();

    const bind_address = Io.net.IpAddress.parseLiteral(args.bind_address) catch |err| {
        log.err("invalid bind address specified: {t}", .{err});
        return 1;
    };

    const gm_bind_address = Io.net.IpAddress.parseLiteral(args.gm_bind_address) catch |err| {
        log.err("invalid gm bind address specified: {t}", .{err});
        return 1;
    };

    var fs = FileSystem.init(gpa, io, args.state_dir) catch |err| {
        log.err("failed to open filesystem at '{s}': {t}", .{ args.state_dir, err });
        return 1;
    };

    defer fs.deinit();

    const sqlite_store: ?*SqliteStore = SqliteStore.init(gpa, io, args.state_dir) catch |err| blk: {
        log.warn("failed to initialize shared SQLite store ({t}), continuing with file-only state", .{err});
        break :blk null;
    };
    defer if (sqlite_store) |st| st.deinit();
    if (sqlite_store) |st| {
        st.attachToFs(&fs);
        _ = st.backfillExistingState(&fs) catch |err| {
            log.warn("SQLite backfill warning: {t}", .{err});
        };
    }

    var session_manager: network.SessionManager = .{};
    defer {
        session_manager.clearHandoffs(io, gpa, &fs);
        session_manager.deinit(gpa);
    }

    var net_future = network.bind(io, gpa, &fs, &assets, &session_manager, bind_address) catch return 1; // TODO: retry with a timeout?
    defer net_future.cancel(io);

    var gm_future = io.concurrent(GmHttp.serve, .{ io, gpa, &assets, &session_manager, gm_bind_address }) catch
        io.async(GmHttp.serve, .{ io, gpa, &assets, &session_manager, gm_bind_address });
    defer gm_future.cancel(io);

    log.info("game server is listening at {f}", .{bind_address});

    var sigint = zigaction.Handler(.INT).wait(io);
    sigint.await(io);

    log.info("shutting down...", .{});
    return 0;
}

pub fn main(minimal: std.process.Init.Minimal) u8 {
    var debug_allocator: std.heap.DebugAllocator(.{}) = .init;
    defer std.debug.assert(debug_allocator.deinit() == .ok);

    var threaded: Io.Threaded = .init(debug_allocator.allocator(), .{
        .environ = minimal.environ,
    });
    defer threaded.deinit();

    return init(debug_allocator.allocator(), threaded.io(), minimal.environ, minimal.args);
}
