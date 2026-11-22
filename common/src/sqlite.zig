const std = @import("std");

pub const sqlite3 = opaque {};
pub const sqlite3_stmt = opaque {};

pub const SQLITE_OK: c_int = 0;
pub const SQLITE_ROW: c_int = 100;
pub const SQLITE_DONE: c_int = 101;
pub const SQLITE_OPEN_READONLY: c_int = 0x00000001;
pub const SQLITE_TRANSIENT: ?*const anyopaque = @ptrFromInt(@as(usize, @bitCast(@as(isize, -1))));

extern "c" fn sqlite3_open(filename: [*:0]const u8, ppDb: *?*sqlite3) c_int;
extern "c" fn sqlite3_open_v2(filename: [*:0]const u8, ppDb: *?*sqlite3, flags: c_int, zVfs: ?[*:0]const u8) c_int;
extern "c" fn sqlite3_close(db: *sqlite3) c_int;
extern "c" fn sqlite3_exec(
    db: *sqlite3,
    sql: [*:0]const u8,
    callback: ?*const anyopaque,
    arg: ?*anyopaque,
    errmsg: ?*[*:0]u8,
) c_int;
extern "c" fn sqlite3_free(ptr: ?*anyopaque) void;
extern "c" fn sqlite3_prepare_v2(
    db: *sqlite3,
    zSql: [*]const u8,
    nByte: c_int,
    ppStmt: *?*sqlite3_stmt,
    pzTail: ?*[*]const u8,
) c_int;
extern "c" fn sqlite3_step(pStmt: *sqlite3_stmt) c_int;
extern "c" fn sqlite3_reset(pStmt: *sqlite3_stmt) c_int;
extern "c" fn sqlite3_clear_bindings(pStmt: *sqlite3_stmt) c_int;
extern "c" fn sqlite3_finalize(pStmt: *sqlite3_stmt) c_int;
extern "c" fn sqlite3_bind_int64(pStmt: *sqlite3_stmt, idx: c_int, val: i64) c_int;
extern "c" fn sqlite3_bind_text(
    pStmt: *sqlite3_stmt,
    idx: c_int,
    val: [*]const u8,
    n: c_int,
    dtor: ?*const anyopaque,
) c_int;
extern "c" fn sqlite3_column_int64(pStmt: *sqlite3_stmt, iCol: c_int) i64;
extern "c" fn sqlite3_column_double(pStmt: *sqlite3_stmt, iCol: c_int) f64;
extern "c" fn sqlite3_column_text(pStmt: *sqlite3_stmt, iCol: c_int) ?[*:0]const u8;
extern "c" fn sqlite3_column_bytes(pStmt: *sqlite3_stmt, iCol: c_int) c_int;
extern "c" fn sqlite3_errmsg(db: *sqlite3) [*:0]const u8;

pub const Db = struct {
    handle: *sqlite3,

    pub fn open(path_z: [*:0]const u8) !Db {
        var ptr: ?*sqlite3 = null;
        if (sqlite3_open(path_z, &ptr) != SQLITE_OK or ptr == null) {
            if (ptr) |p| _ = sqlite3_close(p);
            return error.SqliteOpenFailed;
        }
        var db: Db = .{ .handle = ptr.? };
        errdefer db.close();
        try db.exec("PRAGMA journal_mode=WAL; PRAGMA synchronous=NORMAL; PRAGMA busy_timeout=5000;");
        return db;
    }

    pub fn openReadOnly(path_z: [*:0]const u8) !Db {
        var ptr: ?*sqlite3 = null;
        if (sqlite3_open_v2(path_z, &ptr, SQLITE_OPEN_READONLY, null) != SQLITE_OK or ptr == null) {
            if (ptr) |p| _ = sqlite3_close(p);
            return error.SqliteOpenFailed;
        }
        var db: Db = .{ .handle = ptr.? };
        errdefer db.close();
        try db.exec("PRAGMA cache_size=-16384; PRAGMA temp_store=MEMORY; PRAGMA mmap_size=67108864;");
        return db;
    }

    pub fn close(db: *Db) void {
        _ = sqlite3_close(db.handle);
    }

    pub fn exec(db: *Db, sql_z: [*:0]const u8) !void {
        var errmsg: ?[*:0]u8 = null;
        const rc = sqlite3_exec(db.handle, sql_z, null, null, @ptrCast(&errmsg));
        if (rc != SQLITE_OK) {
            if (errmsg) |msg| {
                std.log.scoped(.sqlite).err("sqlite exec failed ({d}): {s}", .{ rc, std.mem.span(msg) });
                sqlite3_free(msg);
            }
            return error.SqliteExecFailed;
        }
    }

    pub fn prepare(db: *Db, sql: []const u8) !Stmt {
        var stmt_ptr: ?*sqlite3_stmt = null;
        const rc = sqlite3_prepare_v2(db.handle, sql.ptr, @intCast(sql.len), &stmt_ptr, null);
        if (rc != SQLITE_OK or stmt_ptr == null) {
            std.log.scoped(.sqlite).err("sqlite prepare failed ({d}): {s}", .{ rc, std.mem.span(sqlite3_errmsg(db.handle)) });
            return error.SqlitePrepareFailed;
        }
        return .{ .handle = stmt_ptr.? };
    }
};

pub const Stmt = struct {
    handle: *sqlite3_stmt,

    pub fn deinit(s: *Stmt) void {
        _ = sqlite3_finalize(s.handle);
    }

    pub fn reset(s: *Stmt) void {
        _ = sqlite3_reset(s.handle);
        _ = sqlite3_clear_bindings(s.handle);
    }

    pub fn bindInt(s: *Stmt, idx: c_int, val: i64) !void {
        if (sqlite3_bind_int64(s.handle, idx, val) != SQLITE_OK) return error.SqliteBindFailed;
    }

    pub fn bindText(s: *Stmt, idx: c_int, text: []const u8) !void {
        const ptr: [*]const u8 = if (text.len > 0) text.ptr else "".ptr;
        if (sqlite3_bind_text(s.handle, idx, ptr, @intCast(text.len), SQLITE_TRANSIENT) != SQLITE_OK) {
            return error.SqliteBindFailed;
        }
    }

    pub fn step(s: *Stmt) !bool {
        const rc = sqlite3_step(s.handle);
        if (rc == SQLITE_ROW) return true;
        if (rc == SQLITE_DONE) return false;
        return error.SqliteStepFailed;
    }

    pub fn colInt(s: *Stmt, col: c_int) i64 {
        return sqlite3_column_int64(s.handle, col);
    }

    pub fn colDouble(s: *Stmt, col: c_int) f64 {
        return sqlite3_column_double(s.handle, col);
    }

    pub fn colText(s: *Stmt, col: c_int) ?[]const u8 {
        const ptr = sqlite3_column_text(s.handle, col) orelse return null;
        const len = sqlite3_column_bytes(s.handle, col);
        return ptr[0..@intCast(len)];
    }
};
