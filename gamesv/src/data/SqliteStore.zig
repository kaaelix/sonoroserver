const std = @import("std");
const common = @import("common");
const sqlite = common.sqlite;
const FileSystem = common.FileSystem;

const Io = std.Io;
const Allocator = std.mem.Allocator;
const log = std.log.scoped(.sqlite_store);

const SqliteStore = @This();

db: sqlite.Db,
io: Io,
gpa: Allocator,
mutex: Io.Mutex = .init,
stmt_upsert_state: sqlite.Stmt,
stmt_delete_file: sqlite.Stmt,
stmt_upsert_player: sqlite.Stmt,

pub fn init(gpa: Allocator, io: Io, state_dir: []const u8) !*SqliteStore {
    const db_path = try std.fmt.allocPrintSentinel(gpa, "{s}/gamesv.db", .{state_dir}, 0);
    defer gpa.free(db_path);

    var db = try sqlite.Db.open(db_path);
    errdefer db.close();

    try ensureSchema(&db);

    var stmt_upsert_state = try db.prepare(
        \\INSERT INTO state_files(path, content, updated_at) VALUES(?1, ?2, ?3)
        \\ON CONFLICT(path) DO UPDATE SET content = excluded.content, updated_at = excluded.updated_at;
    );
    errdefer stmt_upsert_state.deinit();

    var stmt_delete_file = try db.prepare("DELETE FROM state_files WHERE path = ?1;");
    errdefer stmt_delete_file.deinit();

    var stmt_upsert_player = try db.prepare(
        \\INSERT INTO players(player_id, name, level, exp, coin, rare_coin, cash_coin, sex, world_level, updated_at)
        \\VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
        \\ON CONFLICT(player_id) DO UPDATE SET
        \\  name = excluded.name,
        \\  level = excluded.level,
        \\  exp = excluded.exp,
        \\  coin = excluded.coin,
        \\  rare_coin = excluded.rare_coin,
        \\  cash_coin = excluded.cash_coin,
        \\  sex = excluded.sex,
        \\  world_level = excluded.world_level,
        \\  updated_at = excluded.updated_at;
    );
    errdefer stmt_upsert_player.deinit();

    const store = try gpa.create(SqliteStore);
    store.* = .{
        .db = db,
        .io = io,
        .gpa = gpa,
        .stmt_upsert_state = stmt_upsert_state,
        .stmt_delete_file = stmt_delete_file,
        .stmt_upsert_player = stmt_upsert_player,
    };
    log.info("Shared SQLite store initialized at {s} (WAL mode)", .{db_path});
    return store;
}

pub fn deinit(store: *SqliteStore) void {
    store.mutex.lockUncancelable(store.io);
    defer store.mutex.unlock(store.io);
    store.stmt_upsert_player.deinit();
    store.stmt_delete_file.deinit();
    store.stmt_upsert_state.deinit();
    store.db.close();
    store.gpa.destroy(store);
}

fn ensureSchema(db: *sqlite.Db) !void {
    try db.exec(
        \\CREATE TABLE IF NOT EXISTS state_files (
        \\    path TEXT PRIMARY KEY,
        \\    content TEXT NOT NULL,
        \\    updated_at INTEGER NOT NULL
        \\);
        \\CREATE TABLE IF NOT EXISTS players (
        \\    player_id INTEGER PRIMARY KEY,
        \\    name TEXT NOT NULL DEFAULT '',
        \\    level INTEGER NOT NULL DEFAULT 1,
        \\    exp INTEGER NOT NULL DEFAULT 0,
        \\    coin INTEGER NOT NULL DEFAULT 0,
        \\    rare_coin INTEGER NOT NULL DEFAULT 0,
        \\    cash_coin INTEGER NOT NULL DEFAULT 0,
        \\    sex INTEGER NOT NULL DEFAULT 1,
        \\    world_level INTEGER NOT NULL DEFAULT 1,
        \\    updated_at INTEGER NOT NULL DEFAULT 0
        \\);
        \\CREATE TABLE IF NOT EXISTS player_battlepass (
        \\    player_id INTEGER PRIMARY KEY,
        \\    bp_id INTEGER NOT NULL DEFAULT 1,
        \\    level INTEGER NOT NULL DEFAULT 70,
        \\    exp INTEGER NOT NULL DEFAULT 0,
        \\    weekly_exp INTEGER NOT NULL DEFAULT 0,
        \\    pay_status INTEGER NOT NULL DEFAULT 2,
        \\    had_enter INTEGER NOT NULL DEFAULT 1,
        \\    taken_rewards_json TEXT NOT NULL DEFAULT '[]',
        \\    taken_tasks_json TEXT NOT NULL DEFAULT '[]',
        \\    updated_at INTEGER NOT NULL DEFAULT 0
        \\);
        \\CREATE TABLE IF NOT EXISTS player_activity_claims (
        \\    player_id INTEGER NOT NULL,
        \\    claim_key TEXT NOT NULL,
        \\    value_json TEXT NOT NULL DEFAULT '1',
        \\    updated_at INTEGER NOT NULL DEFAULT 0,
        \\    PRIMARY KEY (player_id, claim_key)
        \\);
        \\CREATE TABLE IF NOT EXISTS player_liveness (
        \\    player_id INTEGER PRIMARY KEY,
        \\    liveness_count INTEGER NOT NULL DEFAULT 100,
        \\    claimed_boxes_json TEXT NOT NULL DEFAULT '[]',
        \\    claimed_tasks_json TEXT NOT NULL DEFAULT '[]',
        \\    updated_at INTEGER NOT NULL DEFAULT 0
        \\);
        \\CREATE TABLE IF NOT EXISTS player_achievements (
        \\    player_id INTEGER NOT NULL,
        \\    achievement_id INTEGER NOT NULL,
        \\    status INTEGER NOT NULL DEFAULT 1,
        \\    progress INTEGER NOT NULL DEFAULT 1,
        \\    is_taken INTEGER NOT NULL DEFAULT 0,
        \\    updated_at INTEGER NOT NULL DEFAULT 0,
        \\    PRIMARY KEY (player_id, achievement_id)
        \\);
        \\CREATE TABLE IF NOT EXISTS player_shop_bought (
        \\    player_id INTEGER NOT NULL,
        \\    shop_id INTEGER NOT NULL,
        \\    goods_id INTEGER NOT NULL,
        \\    bought_count INTEGER NOT NULL DEFAULT 0,
        \\    updated_at INTEGER NOT NULL DEFAULT 0,
        \\    PRIMARY KEY (player_id, shop_id, goods_id)
        \\);
        \\CREATE TABLE IF NOT EXISTS player_kv (
        \\    player_id INTEGER NOT NULL,
        \\    ns TEXT NOT NULL,
        \\    k TEXT NOT NULL,
        \\    val TEXT NOT NULL,
        \\    updated_at INTEGER NOT NULL DEFAULT 0,
        \\    PRIMARY KEY (player_id, ns, k)
        \\);
    );
}

pub fn attachToFs(store: *SqliteStore, fs: *FileSystem) void {
    fs.setSyncHook(store, syncCallback);
}

fn syncCallback(ctx: *anyopaque, op: FileSystem.SyncOp, path: []const u8, content: []const u8) void {
    const store: *SqliteStore = @ptrCast(@alignCast(ctx));
    store.handleSync(op, path, content) catch |err| {
        log.warn("SQLite sync error on '{s}': {t}", .{ path, err });
    };
}

fn nowSeconds(io: Io) i64 {
    const clock: Io.Clock = .real;
    return @intCast(clock.now(io).toSeconds());
}

fn handleSync(store: *SqliteStore, op: FileSystem.SyncOp, path: []const u8, content: []const u8) !void {
    store.mutex.lockUncancelable(store.io);
    defer store.mutex.unlock(store.io);

    const now_s = nowSeconds(store.io);
    switch (op) {
        .write => try store.upsertStateFileLocked(path, content, now_s),
        .delete_file => {
            store.stmt_delete_file.reset();
            try store.stmt_delete_file.bindText(1, path);
            _ = try store.stmt_delete_file.step();
        },
        .delete_tree => {
            const prefix = try std.fmt.allocPrint(store.gpa, "{s}/%", .{path});
            defer store.gpa.free(prefix);
            var stmt = try store.db.prepare("DELETE FROM state_files WHERE path = ?1 OR path LIKE ?2;");
            defer stmt.deinit();
            try stmt.bindText(1, path);
            try stmt.bindText(2, prefix);
            _ = try stmt.step();
        },
    }
}

fn upsertStateFileLocked(store: *SqliteStore, path: []const u8, content: []const u8, now_s: i64) !void {
    store.stmt_upsert_state.reset();
    try store.stmt_upsert_state.bindText(1, path);
    try store.stmt_upsert_state.bindText(2, content);
    try store.stmt_upsert_state.bindInt(3, now_s);
    _ = try store.stmt_upsert_state.step();

    // Also mirror structured player basic info if path is player/<uid>/basic_info
    if (std.mem.startsWith(u8, path, "player/") and std.mem.endsWith(u8, path, "/basic_info")) {
        const inner = path["player/".len .. path.len - "/basic_info".len];
        if (std.fmt.parseInt(i64, inner, 10)) |uid| {
            store.upsertPlayerFromZonLocked(uid, content, now_s) catch |err| {
                log.warn("SQLite player mirror upsert failed for uid={d}: {t}", .{ uid, err });
            };
        } else |err| {
            log.warn("SQLite player mirror invalid uid path '{s}': {t}", .{ path, err });
        }
    }
}

fn extractZonInt(content: []const u8, key: []const u8) ?i64 {
    const pos = std.mem.indexOf(u8, content, key) orelse return null;
    var rest = content[pos + key.len ..];
    const eq = std.mem.indexOfScalar(u8, rest, '=') orelse return null;
    rest = std.mem.trimStart(u8, rest[eq + 1 ..], " \t");
    var end: usize = 0;
    while (end < rest.len and (rest[end] == '-' or (rest[end] >= '0' and rest[end] <= '9'))) : (end += 1) {}
    if (end == 0) return null;
    return std.fmt.parseInt(i64, rest[0..end], 10) catch null;
}

fn extractZonString(content: []const u8, key: []const u8) ?[]const u8 {
    const pos = std.mem.indexOf(u8, content, key) orelse return null;
    const eq = std.mem.indexOfScalarPos(u8, content, pos + key.len, '=') orelse return null;
    const q1 = std.mem.indexOfScalarPos(u8, content, eq + 1, '"') orelse return null;
    const q2 = std.mem.indexOfScalarPos(u8, content, q1 + 1, '"') orelse return null;
    return content[q1 + 1 .. q2];
}

fn upsertPlayerFromZonLocked(store: *SqliteStore, uid: i64, content: []const u8, now_s: i64) !void {
    const name = extractZonString(content, ".name") orelse "";
    const level = extractZonInt(content, ".level") orelse 1;
    const exp = extractZonInt(content, ".exp") orelse 0;
    const coin = extractZonInt(content, ".coin") orelse 0;
    const rare_coin = extractZonInt(content, ".rare_coin") orelse 0;
    const cash_coin = extractZonInt(content, ".cash_coin") orelse 0;
    const sex = extractZonInt(content, ".sex") orelse 1;
    const world_level = extractZonInt(content, ".cur_world_level") orelse 1;

    store.stmt_upsert_player.reset();
    try store.stmt_upsert_player.bindInt(1, uid);
    try store.stmt_upsert_player.bindText(2, name);
    try store.stmt_upsert_player.bindInt(3, level);
    try store.stmt_upsert_player.bindInt(4, exp);
    try store.stmt_upsert_player.bindInt(5, coin);
    try store.stmt_upsert_player.bindInt(6, rare_coin);
    try store.stmt_upsert_player.bindInt(7, cash_coin);
    try store.stmt_upsert_player.bindInt(8, sex);
    try store.stmt_upsert_player.bindInt(9, world_level);
    try store.stmt_upsert_player.bindInt(10, now_s);
    _ = try store.stmt_upsert_player.step();
}

pub fn backfillExistingState(store: *SqliteStore, fs: *FileSystem) !usize {
    var count: usize = 0;
    const now_s = nowSeconds(store.io);

    store.mutex.lockUncancelable(store.io);
    defer store.mutex.unlock(store.io);

    try store.db.exec("BEGIN TRANSACTION;");
    errdefer store.db.exec("ROLLBACK;") catch {};

    const dirs = [_][]const u8{ "player", "account", "social" };
    for (dirs) |sub| {
        const rd_opt = fs.readDir(sub) catch continue;
        if (rd_opt) |rd| {
            defer rd.deinit();
            for (rd.entries) |entry| {
                if (entry.kind != .file) continue;
                const data = (fs.readFile(store.gpa, entry.path) catch continue) orelse continue;
                defer store.gpa.free(data);

                try store.upsertStateFileLocked(entry.path, data, now_s);
                count += 1;
            }
        }
    }

    try store.db.exec("COMMIT;");
    log.info("SQLite backfill synchronized {d} state files into gamesv.db", .{count});
    return count;
}
