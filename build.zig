const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const allow_heavy_llvm = b.option(bool, "allow-heavy-llvm", "Allow ReleaseSafe/ReleaseFast LLVM builds (can OOM on <=8 GiB RAM hosts)") orelse false;
    const strip = b.option(bool, "strip", "Strip debug symbols to reduce binary size and linker memory") orelse true;
    const max_rss_mb = b.option(usize, "max-rss-mb", "Per-step RSS upper limit in MiB (0 disables)") orelse 1400;
    const sequential = b.option(bool, "sequential", "Compile executables sequentially to avoid multi-process memory spikes") orelse true;

    if ((optimize == .ReleaseSafe or optimize == .ReleaseFast) and !allow_heavy_llvm) {
        std.log.err("Refusing -Doptimize={s} without -Dallow-heavy-llvm=true on low-RAM hosts to prevent OOM. Use Debug or -Doptimize=ReleaseSmall.", .{@tagName(optimize)});
        b.invalid_user_input = true;
        return;
    }

    const gamesv_max_rss: usize = max_rss_mb *| @as(usize, 1024 * 1024);
    const aux_max_rss: usize = @as(usize, @min(max_rss_mb, 768)) *| @as(usize, 1024 * 1024);

    const common = b.createModule(.{
        .root_source_file = b.path("common/src/root.zig"),
        .strip = strip,
    });

    const proto_gen = b.addExecutable(.{
        .name = "sonoro-protogen",
        .max_rss = aux_max_rss,
        .root_module = b.createModule(.{
            .root_source_file = b.path("proto/gen/src/main.zig"),
            .optimize = optimize,
            .target = target,
            .strip = strip,
        }),
    });

    var update_src = b.addUpdateSourceFiles();

    if (std.Io.Dir.cwd().access(b.graph.io, "proto/pb/WuWa.proto", .{})) {
        const run_proto_gen = b.addRunArtifact(proto_gen);
        run_proto_gen.expectExitCode(0);
        run_proto_gen.setStdIn(.{ .lazy_path = b.path("proto/pb/WuWa.proto") });
        const pb_gen = run_proto_gen.captureStdOut(.{ .basename = "aki_generated.zig" });
        update_src.addCopyFileToSource(pb_gen, "proto/src/aki_generated.zig");
    } else |_| {}

    const proto = b.createModule(.{
        .root_source_file = b.path("proto/src/root.zig"),
        .target = target,
        .optimize = optimize,
        .strip = strip,
    });

    const cfgsv = b.addExecutable(.{
        .name = "sonoro-cfgsv",
        .max_rss = aux_max_rss,
        .root_module = b.createModule(.{
            .root_source_file = b.path("cfgsv/src/main.zig"),
            .target = target,
            .optimize = optimize,
            .strip = strip,
            .imports = &.{.{ .name = "common", .module = common }},
        }),
    });

    const loginsv = b.addExecutable(.{
        .name = "sonoro-loginsv",
        .max_rss = aux_max_rss,
        .root_module = b.createModule(.{
            .root_source_file = b.path("loginsv/src/main.zig"),
            .target = target,
            .optimize = optimize,
            .strip = strip,
            .imports = &.{.{ .name = "common", .module = common }},
        }),
    });

    const gamesv = b.addExecutable(.{
        .name = "sonoro-gamesv",
        .max_rss = gamesv_max_rss,
        .root_module = b.createModule(.{
            .root_source_file = b.path("gamesv/src/main.zig"),
            .target = target,
            .optimize = optimize,
            .strip = strip,
            .link_libc = true,
            .imports = &.{
                .{ .name = "common", .module = common },
                .{ .name = "proto", .module = proto },
            },
        }),
    });
    gamesv.root_module.addObjectFile(.{ .cwd_relative = "/lib/x86_64-linux-gnu/libsqlite3.so.0" });

    b.step(
        "run-cfgsv",
        "compile and run the config server",
    ).dependOn(&b.addRunArtifact(cfgsv).step);

    b.step(
        "run-loginsv",
        "compile and run the login server",
    ).dependOn(&b.addRunArtifact(loginsv).step);

    const run_gamesv = b.addRunArtifact(gamesv);
    run_gamesv.step.dependOn(&update_src.step);

    b.step(
        "run-gamesv",
        "run the game server",
    ).dependOn(&run_gamesv.step);

    const serve_all_exe = b.addExecutable(.{
        .name = "serve-all",
        .max_rss = aux_max_rss,
        .root_module = b.createModule(.{
            .root_source_file = b.path("build/serve-all.zig"),
            .target = target,
            .optimize = optimize,
            .strip = strip,
        }),
    });

    const serve_all = b.addRunArtifact(serve_all_exe);
    serve_all.addFileArg(cfgsv.getEmittedBin());
    serve_all.addFileArg(loginsv.getEmittedBin());
    serve_all.addFileArg(gamesv.getEmittedBin());
    serve_all.step.dependOn(&update_src.step);

    b.step(
        "serve-all",
        "start cfgsv, loginsv, gamesv at once",
    ).dependOn(&serve_all.step);

    const clear_state_exe = b.addExecutable(.{
        .name = "clear-state",
        .max_rss = aux_max_rss,
        .root_module = b.createModule(.{
            .root_source_file = b.path("build/clear-state.zig"),
            .target = target,
            .optimize = optimize,
            .strip = strip,
        }),
    });

    const clear_state = b.addRunArtifact(clear_state_exe);
    clear_state.addArg("state/account");
    clear_state.addArg("state/player");

    const clear_all = b.addRunArtifact(serve_all_exe);
    clear_all.addFileArg(cfgsv.getEmittedBin());
    clear_all.addFileArg(loginsv.getEmittedBin());
    clear_all.addFileArg(gamesv.getEmittedBin());
    clear_all.step.dependOn(&update_src.step);
    clear_all.step.dependOn(&clear_state.step);

    b.step(
        "clear-all",
        "clear account and player state, then start all servers",
    ).dependOn(&clear_all.step);

    if (sequential) {
        loginsv.step.dependOn(&cfgsv.step);
        gamesv.step.dependOn(&loginsv.step);
    }

    const install_cfgsv = b.addInstallArtifact(cfgsv, .{});
    const install_loginsv = b.addInstallArtifact(loginsv, .{});
    const install_gamesv = b.addInstallArtifact(gamesv, .{});
    b.getInstallStep().dependOn(&install_cfgsv.step);
    b.getInstallStep().dependOn(&install_loginsv.step);
    b.getInstallStep().dependOn(&install_gamesv.step);

    b.step("cfgsv", "build sonoro-cfgsv").dependOn(&install_cfgsv.step);
    b.step("loginsv", "build sonoro-loginsv").dependOn(&install_loginsv.step);
    b.step("gamesv", "build sonoro-gamesv").dependOn(&install_gamesv.step);
}
