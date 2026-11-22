const FileSystem = @This();

const std = @import("std");
const Io = std.Io;
const Allocator = std.mem.Allocator;
const ArenaAllocator = std.heap.ArenaAllocator;

pub const SyncOp = enum { write, delete_file, delete_tree };
pub const SyncFn = *const fn (ctx: *anyopaque, op: SyncOp, path: []const u8, content: []const u8) void;

io: Io,
gpa: Allocator,
root_dir: Io.Dir,
map_lock: Io.Mutex = .init,
map: std.StringArrayHashMapUnmanaged(*FileEntry) = .empty,
watchers_lock: Io.Mutex = .init,
watchers: std.ArrayList(*WatchSubscriber) = .empty,
sync_ctx: ?*anyopaque = null,
sync_fn: ?SyncFn = null,

pub fn setSyncHook(fs: *FileSystem, ctx: *anyopaque, func: SyncFn) void {
    fs.sync_ctx = ctx;
    fs.sync_fn = func;
}

const FileEntry = struct {
    file_lock: Io.Mutex = .init,
    content: ?[]const u8 = null,
    mtime: i64 = 0,
};

const WatchSubscriber = struct {
    basepath: []const u8,
    awaiter: *Io.Queue(Changes),
    dead: bool = false,
};

pub const ReadDir = struct {
    pub const Entry = struct {
        path: []const u8,
        kind: Io.File.Kind,

        pub fn basename(entry: Entry) []const u8 {
            return if (std.mem.findScalarLast(u8, entry.path, '/')) |last_segment_begin|
                entry.path[last_segment_begin + 1 ..]
            else
                entry.path;
        }
    };

    entries: []const Entry,
    arena: ArenaAllocator,

    pub fn deinit(readdir: ReadDir) void {
        readdir.arena.deinit();
    }
};

pub fn init(gpa: Allocator, io: Io, root_path: []const u8) !FileSystem {
    const root_dir = try Io.Dir.cwd().openDir(io, root_path, .{ .iterate = true });
    return .{ .root_dir = root_dir, .gpa = gpa, .io = io };
}

pub fn deinit(fs: *FileSystem) void {
    var iterator = fs.map.iterator();
    while (iterator.next()) |entry| {
        fs.gpa.free(entry.key_ptr.*);

        const file_entry = entry.value_ptr.*;
        if (file_entry.content) |content|
            fs.gpa.free(content);
        fs.gpa.destroy(file_entry);
    }

    for (fs.watchers.items) |watcher| {
        fs.gpa.free(watcher.basepath);
        fs.gpa.destroy(watcher);
    }

    fs.watchers.deinit(fs.gpa);
    fs.map.deinit(fs.gpa);
    fs.root_dir.close(fs.io);
}

fn getEntry(fs: *FileSystem, path: []const u8) !?*FileEntry {
    try fs.map_lock.lock(fs.io);
    defer fs.map_lock.unlock(fs.io);
    return fs.map.get(path);
}

fn getOrCreateEntry(fs: *FileSystem, path: []const u8) !*FileEntry {
    try fs.map_lock.lock(fs.io);
    defer fs.map_lock.unlock(fs.io);

    if (fs.map.get(path)) |existing| return existing;

    const entry = try fs.gpa.create(FileEntry);
    errdefer fs.gpa.destroy(entry);
    entry.* = .{};

    const owned_path = try fs.gpa.dupe(u8, path);
    errdefer fs.gpa.free(owned_path);

    try fs.map.put(fs.gpa, owned_path, entry);
    return entry;
}

pub fn readFile(fs: *FileSystem, caller_gpa: Allocator, path: []const u8) !?[:0]u8 {
    const entry = (try fs.getEntry(path)) orelse blk: {
        _ = fs.root_dir.statFile(fs.io, path, .{}) catch |err| switch (err) {
            error.FileNotFound => return null,
            else => return err,
        };
        break :blk try fs.getOrCreateEntry(path);
    };

    try entry.file_lock.lock(fs.io);
    defer entry.file_lock.unlock(fs.io);

    if (fs.root_dir.statFile(fs.io, path, .{})) |stat| {
        if (entry.content == null or stat.mtime.toSeconds() > entry.mtime) {
            const cached_content = (try readEntireFileAlloc(fs.io, fs.root_dir, path, fs.gpa)) orelse {
                if (entry.content) |old_content| {
                    fs.gpa.free(old_content);
                    entry.content = null;
                }
                return null;
            };
            errdefer fs.gpa.free(cached_content);
            const content = try caller_gpa.dupeZ(u8, cached_content);

            if (entry.content) |old_content|
                fs.gpa.free(old_content);

            entry.mtime = stat.mtime.toSeconds();
            entry.content = cached_content;
            return content;
        } else return try caller_gpa.dupeZ(u8, entry.content.?);
    } else |_| {
        if (entry.content) |content| {
            fs.gpa.free(content);
            entry.content = null;
        }
        entry.mtime = 0;
        return null;
    }
}

pub fn readDir(fs: *FileSystem, path: []const u8) !?ReadDir {
    var dir = fs.root_dir.openDir(fs.io, path, .{ .iterate = true }) catch |err| switch (err) {
        error.FileNotFound => return null,
        else => return err,
    };

    defer dir.close(fs.io);

    var arena = ArenaAllocator.init(fs.gpa);
    errdefer arena.deinit();

    var entries: std.ArrayList(ReadDir.Entry) = .empty;

    var walker = try Io.Dir.walk(dir, fs.gpa);
    defer walker.deinit();

    while (try walker.next(fs.io)) |entry| {
        try entries.append(arena.allocator(), .{
            .path = try std.fmt.allocPrint(arena.allocator(), "{s}/{s}", .{ path, entry.path }),
            .kind = entry.kind,
        });
    }

    return .{
        .entries = entries.items,
        .arena = arena,
    };
}

fn readEntireFileAlloc(io: Io, dir: Io.Dir, sub_path: []const u8, a: Allocator) !?[]u8 {
    var file = dir.openFile(io, sub_path, .{}) catch |err| switch (err) {
        error.FileNotFound => return null,
        else => return err,
    };

    defer file.close(io);

    var reader = file.reader(io, "");
    return try reader.interface.allocRemaining(a, .unlimited);
}

fn writeEntryLocked(fs: *FileSystem, entry: *FileEntry, path: []const u8, content: []const u8) !void {
    const new_copy = try fs.gpa.dupe(u8, content);
    errdefer fs.gpa.free(new_copy);

    try makeDirAndWriteFile(fs.io, fs.root_dir, path, content);

    if (entry.content) |old_content| fs.gpa.free(old_content);
    entry.content = new_copy;
    const realtime_clock: Io.Clock = .real;
    entry.mtime = @intCast(realtime_clock.now(fs.io).toSeconds());
    if (fs.sync_fn) |func| {
        if (fs.sync_ctx) |ctx| func(ctx, .write, path, content);
    }
}

pub fn writeFile(fs: *FileSystem, path: []const u8, content: []const u8) !void {
    const entry = try fs.getOrCreateEntry(path);
    try entry.file_lock.lock(fs.io);
    defer entry.file_lock.unlock(fs.io);

    try fs.writeEntryLocked(entry, path, content);
}

pub fn deleteFile(fs: *FileSystem, path: []const u8) !void {
    if (try fs.getEntry(path)) |entry| {
        try entry.file_lock.lock(fs.io);
        defer entry.file_lock.unlock(fs.io);

        if (entry.content) |content| {
            fs.gpa.free(content);
            entry.content = null;
        }
        entry.mtime = 0;

        fs.root_dir.deleteFile(fs.io, path) catch |err| switch (err) {
            error.FileNotFound => {},
            else => return err,
        };
        if (fs.sync_fn) |func| {
            if (fs.sync_ctx) |ctx| func(ctx, .delete_file, path, "");
        }
        return;
    }

    fs.root_dir.deleteFile(fs.io, path) catch |err| switch (err) {
        error.FileNotFound => {},
        else => return err,
    };
    if (fs.sync_fn) |func| {
        if (fs.sync_ctx) |ctx| func(ctx, .delete_file, path, "");
    }
}

pub fn deleteTree(fs: *FileSystem, path: []const u8) !void {
    var matching: std.ArrayList(*FileEntry) = .empty;
    defer matching.deinit(fs.gpa);

    {
        try fs.map_lock.lock(fs.io);
        defer fs.map_lock.unlock(fs.io);

        var it = fs.map.iterator();
        while (it.next()) |kv| {
            const key = kv.key_ptr.*;
            if (std.mem.eql(u8, key, path) or
                (std.mem.startsWith(u8, key, path) and key.len > path.len and key[path.len] == '/'))
            {
                try matching.append(fs.gpa, kv.value_ptr.*);
            }
        }
    }

    for (matching.items) |entry| {
        try entry.file_lock.lock(fs.io);
        defer entry.file_lock.unlock(fs.io);
        if (entry.content) |content| {
            fs.gpa.free(content);
            entry.content = null;
        }
        entry.mtime = 0;
    }

    try fs.root_dir.deleteTree(fs.io, path);
    if (fs.sync_fn) |func| {
        if (fs.sync_ctx) |ctx| func(ctx, .delete_tree, path, "");
    }
}

pub const Changes = struct {
    files: []const File,
    arena: ArenaAllocator,

    pub const File = struct {
        path: []const u8,

        pub fn basename(file: File) []const u8 {
            return if (std.mem.findScalarLast(u8, file.path, '/')) |last_segment_begin|
                file.path[last_segment_begin + 1 ..]
            else
                file.path;
        }
    };

    pub fn deinit(changes: Changes) void {
        changes.arena.deinit();
    }
};

pub fn waitForChanges(fs: *FileSystem, base_path: []const u8) !Changes {
    // wait through a oneshot queue, similar to a channel

    var queue_buffer: [1]Changes = undefined;
    var awaiter = Io.Queue(Changes).init(queue_buffer[0..]);

    const watcher_ptr = blk: {
        try fs.watchers_lock.lock(fs.io);
        defer fs.watchers_lock.unlock(fs.io);

        const watcher = try fs.gpa.create(WatchSubscriber);
        errdefer fs.gpa.destroy(watcher);
        watcher.* = .{
            .basepath = try fs.gpa.dupe(u8, base_path),
            .awaiter = &awaiter,
        };
        errdefer fs.gpa.free(watcher.basepath);
        try fs.watchers.append(fs.gpa, watcher);
        break :blk watcher;
    };

    return awaiter.getOne(fs.io) catch |err| {
        @atomicStore(bool, &watcher_ptr.dead, true, .seq_cst);
        return err;
    };
}

// Sets up the filesystem watcher. Meant to be ran concurrently.
pub fn watch(fs: *FileSystem) !void {
    const io = fs.io;
    const realtime_clock: Io.Clock = .real;
    const loop_interval = Io.Duration.fromMilliseconds(1000);
    const deprecated_dir = std.fs.Dir.adaptFromNewApi(fs.root_dir);

    while (!io.cancelRequested()) {
        try io.sleep(loop_interval, realtime_clock);

        try fs.watchers_lock.lock(fs.io);
        defer fs.watchers_lock.unlock(fs.io);

        for (fs.watchers.items) |watcher| {
            if (watcher.dead) continue;

            var arena = ArenaAllocator.init(fs.gpa);
            errdefer arena.deinit();

            var changes: std.ArrayList(Changes.File) = .empty;

            var walker = try deprecated_dir.walk(fs.gpa);
            defer walker.deinit();

            while (try walker.next()) |entry| {
                if (entry.kind != .file) continue;
                if (std.mem.startsWith(u8, entry.path, watcher.basepath)) {
                    const file_stat = fs.root_dir.statPath(fs.io, entry.path, .{}) catch continue;
                    if (!(try fs.isChangeAcknowledged(entry.path, file_stat.mtime.toSeconds()))) {
                        try changes.append(arena.allocator(), .{
                            .path = try arena.allocator().dupe(u8, entry.path),
                        });
                    }
                }
            }

            if (changes.items.len > 0) {
                watcher.dead = true;
                try watcher.awaiter.putOne(fs.io, .{
                    .files = changes.items,
                    .arena = arena,
                });
            }
        }

        var i: usize = 0;
        while (i < fs.watchers.items.len) {
            if (fs.watchers.items[i].dead) {
                fs.gpa.free(fs.watchers.items[i].basepath);
                fs.gpa.destroy(fs.watchers.items[i]);
                _ = fs.watchers.swapRemove(i);
            } else i += 1;
        }
    }
}

fn isChangeAcknowledged(fs: *FileSystem, path: []const u8, mtime: i64) !bool {
    const entry = (try fs.getEntry(path)) orelse return false;
    try entry.file_lock.lock(fs.io);
    defer entry.file_lock.unlock(fs.io);
    return entry.mtime >= mtime;
}

fn makeDirAndWriteFile(io: Io, dir: Io.Dir, sub_path: []const u8, content: []const u8) !void {
    var atomic = try dir.createFileAtomic(io, sub_path, .{
        .make_path = true,
        .replace = true,
    });
    defer atomic.deinit(io);

    try atomic.file.writeStreamingAll(io, content);
    try atomic.file.sync(io);
    try atomic.replace(io);
}

pub const FileLock = struct {
    fs: *FileSystem,
    path: []const u8,
    content: []const u8,
    mutex: *Io.Mutex,
    entry: *FileEntry,

    pub fn unlock(lock: *FileLock, replace_with: ?[]const u8) !void {
        defer lock.mutex.unlock(lock.fs.io);

        if (replace_with) |new_content| {
            try lock.fs.writeEntryLocked(lock.entry, lock.path, new_content);
        }
    }
};

pub fn lockFile(fs: *FileSystem, path: []const u8) !?FileLock {
    const entry = (try fs.getEntry(path)) orelse blk: {
        _ = fs.root_dir.statFile(fs.io, path, .{}) catch |err| switch (err) {
            error.FileNotFound => return null,
            else => return err,
        };
        break :blk try fs.getOrCreateEntry(path);
    };

    try entry.file_lock.lock(fs.io);
    errdefer entry.file_lock.unlock(fs.io);

    if (fs.root_dir.statFile(fs.io, path, .{})) |stat| {
        if (entry.content == null or stat.mtime.toSeconds() > entry.mtime) {
            const content = (try readEntireFileAlloc(fs.io, fs.root_dir, path, fs.gpa)) orelse {
                if (entry.content) |old_content| {
                    fs.gpa.free(old_content);
                    entry.content = null;
                }
                entry.mtime = 0;
                entry.file_lock.unlock(fs.io);
                return null;
            };
            if (entry.content) |old_content| fs.gpa.free(old_content);
            entry.mtime = stat.mtime.toSeconds();
            entry.content = content;
        }
        return .{
            .fs = fs,
            .path = path,
            .content = entry.content.?,
            .mutex = &entry.file_lock,
            .entry = entry,
        };
    } else |_| {
        if (entry.content) |content| {
            fs.gpa.free(content);
            entry.content = null;
        }
        entry.mtime = 0;
        entry.file_lock.unlock(fs.io);
        return null;
    }
}
