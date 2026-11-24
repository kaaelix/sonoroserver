const std = @import("std");
const common = @import("common");
const Io = std.Io;
const Allocator = std.mem.Allocator;
const Assets = @import("../data/Assets.zig");
const Connection = @import("Connection.zig");
const State = @import("State.zig");
const SessionManager = @import("../network.zig").SessionManager;
const EventQueue = @import("../logic/EventQueue.zig");
const logic_handlers = @import("../logic/handlers.zig");
const commands = @import("../logic/commands.zig");
const progression = @import("../logic/commands/progression.zig");
const cheat_cmd = @import("../logic/commands/cheat.zig");
const attr_cmd = @import("../logic/commands/attribute.zig");
const js_cmd = @import("../logic/commands/js_patches.zig");
const mem = @import("../mem.zig");
const gm_bridge = @import("gm_bridge.zig");
const aki_logic_bridge = @import("aki_logic_bridge.zig");
const gm_i18n = @import("gm_i18n.zig");

const log = std.log.scoped(.gm_http);
const panel_html = @embedFile("gm_panel.html");

pub fn serve(
    io: Io,
    gpa: Allocator,
    assets: *const Assets,
    session_manager: *SessionManager,
    bind_address: Io.net.IpAddress,
) void {
    var server = bind_address.listen(io, .{ .reuse_address = true }) catch |err| {
        log.err("failed to listen GM HTTP at {f}: {t}", .{ bind_address, err });
        return;
    };
    defer server.deinit(io);
    log.info("GM HTTP & Web Panel listening at http://{f}/gm", .{bind_address});

    var group: Io.Group = .init;
    defer group.cancel(io);

    while (true) {
        io.checkCancel() catch break;
        const stream = server.accept(io) catch |err| switch (err) {
            error.Canceled => break,
            else => {
                log.warn("GM accept error: {t}", .{err});
                continue;
            },
        };
        const args = .{ io, gpa, assets, session_manager, stream };
        group.concurrent(io, processConnection, args) catch {
            processConnection(io, gpa, assets, session_manager, stream);
        };
    }
}

fn processConnection(
    io: Io,
    gpa: Allocator,
    assets: *const Assets,
    session_manager: *SessionManager,
    stream: Io.net.Stream,
) void {
    defer stream.close(io);

    var read_buf: [4096]u8 = undefined;
    var reader = stream.reader(io, &read_buf);
    var write_buf: [8192]u8 = undefined;
    var writer = stream.writer(io, &write_buf);

    var arena_state = std.heap.ArenaAllocator.init(gpa);
    defer arena_state.deinit();

    handleHttp(io, gpa, arena_state.allocator(), assets, session_manager, &reader.interface, &writer.interface) catch |err| {
        if (err != error.Canceled and err != error.EndOfStream) {
            log.debug("GM HTTP request closed: {t}", .{err});
        }
    };
}

const HttpReq = struct {
    method: []const u8,
    path: []const u8,
    query: []const u8,
    body: []const u8,
    gm_sign: []const u8 = "",
    gm_admin: []const u8 = "",
};

fn readHttpRequest(arena: Allocator, reader: *Io.Reader) !HttpReq {
    const req_line_raw = try reader.takeDelimiterInclusive('\n');
    const req_line = std.mem.trimEnd(u8, req_line_raw, "\r\n");
    var tok = std.mem.tokenizeScalar(u8, req_line, ' ');
    const method = try arena.dupe(u8, tok.next() orelse return error.InvalidRequest);
    const full_target = tok.next() orelse return error.InvalidRequest;

    const path_raw, const query_raw = if (std.mem.findScalar(u8, full_target, '?')) |q_idx|
        .{ full_target[0..q_idx], full_target[q_idx + 1 ..] }
    else
        .{ full_target, "" };
    const path = try arena.dupe(u8, path_raw);
    const query = try arena.dupe(u8, query_raw);

    var content_length: usize = 0;
    var gm_sign: []const u8 = "";
    var gm_admin: []const u8 = "";
    while (true) {
        const line_raw = try reader.takeDelimiterInclusive('\n');
        const line = std.mem.trimEnd(u8, line_raw, "\r\n");
        if (line.len == 0) break;
        if (std.mem.indexOfScalar(u8, line, ':')) |colon| {
            const hname = std.mem.trim(u8, line[0..colon], " \t");
            const hval = std.mem.trim(u8, line[colon + 1 ..], " \t");
            if (std.ascii.eqlIgnoreCase(hname, "Content-Length")) {
                content_length = std.fmt.parseInt(usize, hval, 10) catch 0;
            } else if (std.ascii.eqlIgnoreCase(hname, "X-GM-Sign")) {
                gm_sign = try arena.dupe(u8, hval);
            } else if (std.ascii.eqlIgnoreCase(hname, "X-GM-Admin")) {
                gm_admin = try arena.dupe(u8, hval);
            }
        }
    }

    if (content_length > 65536) return error.PayloadTooLarge;
    var body: []const u8 = "";
    if (content_length > 0) {
        const buf = try arena.alloc(u8, content_length);
        try reader.readSliceAll(buf);
        body = buf;
    }

    return .{
        .method = method,
        .path = path,
        .query = query,
        .body = body,
        .gm_sign = gm_sign,
        .gm_admin = gm_admin,
    };
}

fn writeHttpResponse(
    writer: *Io.Writer,
    status: []const u8,
    content_type: []const u8,
    body: []const u8,
) !void {
    try writer.print(
        "HTTP/1.1 {s}\r\n" ++
            "Content-Type: {s}\r\n" ++
            "Content-Length: {d}\r\n" ++
            "Access-Control-Allow-Origin: *\r\n" ++
            "Access-Control-Allow-Headers: Content-Type, X-GM-Sign, X-GM-Admin\r\n" ++
            "Access-Control-Allow-Methods: GET, POST, OPTIONS\r\n" ++
            "Cache-Control: no-store\r\n" ++
            "Connection: close\r\n\r\n",
        .{ status, content_type, body.len },
    );
    try writer.writeAll(body);
    try writer.flush();
}

fn queryParam(query: []const u8, key: []const u8) ?[]const u8 {
    var it = std.mem.tokenizeScalar(u8, query, '&');
    while (it.next()) |pair| {
        const eq = std.mem.indexOfScalar(u8, pair, '=') orelse continue;
        if (std.mem.eql(u8, pair[0..eq], key)) return pair[eq + 1 ..];
    }
    return null;
}

fn urlDecode(arena: Allocator, raw: []const u8) ![]const u8 {
    var out: std.ArrayList(u8) = .empty;
    var i: usize = 0;
    while (i < raw.len) : (i += 1) {
        if (raw[i] == '+') {
            try out.append(arena, ' ');
        } else if (raw[i] == '%' and i + 2 < raw.len) {
            const b = std.fmt.parseInt(u8, raw[i + 1 .. i + 3], 16) catch {
                try out.append(arena, raw[i]);
                continue;
            };
            try out.append(arena, b);
            i += 2;
        } else {
            try out.append(arena, raw[i]);
        }
    }
    return out.items;
}

fn adminSecret() []const u8 {
    if (std.c.getenv("GM_ADMIN_PASSWORD")) |ptr| {
        const val = std.mem.span(ptr);
        if (val.len > 0) return val;
    }
    return "admin";
}

fn timingSafeEqual(a: []const u8, b: []const u8) bool {
    if (a.len == 0 or a.len != b.len) return false;
    var diff: u8 = 0;
    for (a, b) |x, y| {
        diff |= x ^ y;
    }
    return diff == 0;
}

fn isValidAdmin(key: []const u8) bool {
    return timingSafeEqual(key, adminSecret());
}

fn lookupPlayerSignToken(io: Io, arena: Allocator, target_id: i32) ?[]const u8 {
    if (target_id <= 0) return null;
    if (common.sqlite.Db.open("state/gamesv.db")) |db_val| {
        var db = db_val;
        defer db.close();
        if (db.prepare("SELECT val FROM player_kv WHERE player_id = ?1 AND ns = 'gm' AND k = 'sign_token' LIMIT 1;")) |stmt_val| {
            var stmt = stmt_val;
            defer stmt.deinit();
            if (stmt.bindInt(1, target_id)) {
                if (stmt.step() catch false) {
                    if (stmt.colText(0)) |txt| {
                        if (txt.len >= 16) return arena.dupe(u8, txt) catch null;
                    }
                }
            } else |_| {}
        } else |_| {}
    } else |_| {}

    const prog_path = std.fmt.allocPrint(arena, "state/player/{d}/progress_info", .{target_id}) catch return null;
    const content = Io.Dir.cwd().readFileAlloc(io, prog_path, arena, .limited(1024 * 1024)) catch return null;
    const marker = ".key = \"gm:sign_token\", .val = \"";
    if (std.mem.indexOf(u8, content, marker)) |idx| {
        const rest = content[idx + marker.len ..];
        if (std.mem.indexOfScalar(u8, rest, '"')) |end| {
            const val = rest[0..end];
            if (val.len >= 16) return arena.dupe(u8, val) catch null;
        }
    }
    return null;
}

fn rotatePlayerSignToken(io: Io, arena: Allocator, target_id: i32) ![]const u8 {
    var rand_bytes: [12]u8 = undefined;
    Io.random(io, &rand_bytes);
    const hex = std.fmt.bytesToHex(rand_bytes, .lower);
    const token = try arena.dupe(u8, &hex);

    var db = try common.sqlite.Db.open("state/gamesv.db");
    defer db.close();
    var stmt = try db.prepare(
        "INSERT INTO player_kv(player_id, ns, k, val, updated_at) VALUES(?1, 'gm', 'sign_token', ?2, ?3) " ++
            "ON CONFLICT(player_id, ns, k) DO UPDATE SET val = excluded.val, updated_at = excluded.updated_at;",
    );
    defer stmt.deinit();
    try stmt.bindInt(1, target_id);
    try stmt.bindText(2, token);
    const ts = std.Io.Clock.real.now(io).toSeconds();
    try stmt.bindInt(3, ts);
    _ = try stmt.step();
    return token;
}

fn isAuthorizedForPlayer(
    io: Io,
    arena: Allocator,
    target_id: i32,
    req_sign: []const u8,
    req_admin: []const u8,
) bool {
    if (isValidAdmin(req_admin)) return true;
    if (target_id <= 0 or req_sign.len < 16) return false;
    const expected = lookupPlayerSignToken(io, arena, target_id) orelse return false;
    return timingSafeEqual(req_sign, expected);
}

fn handleHttp(
    io: Io,
    gpa: Allocator,
    arena: Allocator,
    assets: *const Assets,
    session_manager: *SessionManager,
    reader: *Io.Reader,
    writer: *Io.Writer,
) !void {
    const req = try readHttpRequest(arena, reader);

    if (std.mem.eql(u8, req.method, "OPTIONS")) {
        try writeHttpResponse(writer, "204 No Content", "text/plain", "");
        return;
    }

    if (std.mem.eql(u8, req.path, "/") or
        std.mem.eql(u8, req.path, "/gm") or
        std.mem.eql(u8, req.path, "/gm/") or
        std.mem.eql(u8, req.path, "/announcement"))
    {
        try writeHttpResponse(writer, "200 OK", "text/html; charset=utf-8", panel_html);
        return;
    }

    if (std.mem.eql(u8, req.path, "/health")) {
        const aki_ok = aki_logic_bridge.isHealthy(io);
        const body = if (aki_ok)
            "{\"ok\":true,\"aki_logic\":true}"
        else
            "{\"ok\":true,\"aki_logic\":false}";
        try writeHttpResponse(writer, "200 OK", "application/json", body);
        return;
    }

    if (std.mem.eql(u8, req.path, "/aki-logic/status")) {
        const aki_ok = aki_logic_bridge.isHealthy(io);
        const body = if (aki_ok)
            "{\"service\":\"AkiLogicService\",\"status\":\"connected\",\"runtime\":\".NET 9.0\",\"mode\":\"IPC UnixSocket\"}"
        else
            "{\"service\":\"AkiLogicService\",\"status\":\"offline\",\"fallback\":\"Zig Native Engine\"}";
        try writeHttpResponse(writer, "200 OK", "application/json", body);
        return;
    }

    if (std.mem.eql(u8, req.path, "/gm/catalog")) {
        const body = try buildCatalogJson(arena, assets, req.query);
        try writeHttpResponse(writer, "200 OK", "application/json; charset=utf-8", body);
        return;
    }

    if (std.mem.eql(u8, req.path, "/gm/auth") and std.mem.eql(u8, req.method, "POST")) {
        const AuthBody = struct {
            admin_key: []const u8 = "",
        };
        const parsed = std.json.parseFromSliceLeaky(AuthBody, arena, req.body, .{ .ignore_unknown_fields = true }) catch {
            try writeHttpResponse(writer, "400 Bad Request", "application/json", "{\"ok\":false,\"admin\":false,\"message\":\"Invalid JSON payload\"}");
            return;
        };
        const supplied = if (req.gm_admin.len > 0) req.gm_admin else parsed.admin_key;
        if (isValidAdmin(supplied)) {
            try writeHttpResponse(writer, "200 OK", "application/json", "{\"ok\":true,\"admin\":true}");
        } else {
            try writeHttpResponse(writer, "401 Unauthorized", "application/json", "{\"ok\":false,\"admin\":false,\"message\":\"Wrong password.\"}");
        }
        return;
    }

    if (std.mem.eql(u8, req.path, "/gm/state")) {
        const role_id = if (queryParam(req.query, "role_id")) |v| std.fmt.parseInt(i32, v, 10) catch 0 else 0;
        const req_admin = if (req.gm_admin.len > 0)
            req.gm_admin
        else
            try urlDecode(arena, queryParam(req.query, "admin_key") orelse "");
        const req_sign = if (req.gm_sign.len > 0)
            req.gm_sign
        else
            try urlDecode(arena, queryParam(req.query, "sign") orelse "");
        const is_admin = isValidAdmin(req_admin);

        if (!is_admin and !isAuthorizedForPlayer(io, arena, role_id, req_sign, req_admin)) {
            try writeHttpResponse(
                writer,
                "401 Unauthorized",
                "application/json; charset=utf-8",
                "{\"ok\":false,\"online\":false,\"auth_required\":true,\"online_players\":[],\"message\":\"Access denied. Open the GM panel from in-game Announcements or enter your UID and Access Key.\"}",
            );
            return;
        }

        if (role_id > 0) {
            var job: gm_bridge.GmJob = .{
                .action = .get_state,
                .flag = is_admin,
                .response_gpa = gpa,
            };
            if (try session_manager.submitGmJob(io, gpa, role_id, &job)) {
                defer gpa.free(job.response_json);
                job.done.waitUncancelable(io);
                try writeHttpResponse(writer, "200 OK", "application/json; charset=utf-8", job.response_json);
                return;
            }
        }
        const online_ids: []const i32 = if (is_admin)
            try session_manager.listOnlineIds(io, arena)
        else
            &.{};
        const OfflinePayload = struct {
            online: bool = false,
            online_players: []const i32,
            message: []const u8 = "No active online player session",
        };
        var out: Io.Writer.Allocating = .init(arena);
        try out.writer.print("{f}", .{std.json.fmt(OfflinePayload{ .online_players = online_ids }, .{})});
        try writeHttpResponse(writer, "200 OK", "application/json; charset=utf-8", out.written());
        return;
    }

    if (std.mem.eql(u8, req.path, "/gm/exec") and std.mem.eql(u8, req.method, "POST")) {
        const ExecBody = struct {
            action: []const u8 = "",
            role_id: i32 = 0,
            target_str: []const u8 = "",
            int_a: i32 = 0,
            int_b: i32 = 0,
            int_c: i32 = 0,
            flag: bool = false,
            raw_cmd: []const u8 = "",
            sign: []const u8 = "",
            admin_key: []const u8 = "",
        };
        const parsed = std.json.parseFromSliceLeaky(ExecBody, arena, req.body, .{ .ignore_unknown_fields = true }) catch {
            try writeHttpResponse(writer, "400 Bad Request", "application/json", "{\"ok\":false,\"message\":\"Invalid JSON payload\"}");
            return;
        };
        const action = std.meta.stringToEnum(gm_bridge.GmAction, parsed.action) orelse {
            try writeHttpResponse(writer, "400 Bad Request", "application/json", "{\"ok\":false,\"message\":\"Unknown action\"}");
            return;
        };

        const req_admin = if (req.gm_admin.len > 0) req.gm_admin else parsed.admin_key;
        const req_sign = if (req.gm_sign.len > 0) req.gm_sign else parsed.sign;
        const is_admin = isValidAdmin(req_admin);

        if (parsed.role_id <= 0) {
            try writeHttpResponse(writer, "200 OK", "application/json", "{\"ok\":false,\"message\":\"Invalid target player ID\"}");
            return;
        }
        if (!isAuthorizedForPlayer(io, arena, parsed.role_id, req_sign, req_admin)) {
            try writeHttpResponse(
                writer,
                "401 Unauthorized",
                "application/json",
                "{\"ok\":false,\"auth_required\":true,\"message\":\"Access denied. Invalid UID or Access Key.\"}",
            );
            return;
        }

        if (action == .rotate_gm_key) {
            const new_key = try rotatePlayerSignToken(io, arena, parsed.role_id);
            const RotateResp = struct {
                ok: bool = true,
                gm_key: []const u8,
                message: []const u8 = "New personal Access Key generated. Previous keys are now invalid.",
            };
            var out: Io.Writer.Allocating = .init(arena);
            try out.writer.print("{f}", .{std.json.fmt(RotateResp{ .gm_key = new_key }, .{})});
            try writeHttpResponse(writer, "200 OK", "application/json; charset=utf-8", out.written());
            return;
        }

        if (action == .reset_player) {
            const target_id = parsed.role_id;
            if (session_manager.kickPlayer(io, target_id)) {
                io.sleep(.fromMilliseconds(200), .awake) catch {};
            }
            if (session_manager.takeHandoff(io, target_id)) |handoff| {
                var st = handoff.state;
                st.deinit(gpa, io, st.fs);
            }
            const player_dir = try std.fmt.allocPrint(arena, "state/player/{d}", .{target_id});
            std.Io.Dir.cwd().deleteTree(io, player_dir) catch |err| {
                log.warn("failed to delete {s}: {t}", .{ player_dir, err });
            };
            purgePlayerSqlite(arena, target_id);
            const res_msg = try std.fmt.allocPrint(arena, "{{\"ok\":true,\"message\":\"Account UID {d} has been completely reset. Please log in again to start fresh.\"}}", .{target_id});
            try writeHttpResponse(writer, "200 OK", "application/json; charset=utf-8", res_msg);
            return;
        }

        var job: gm_bridge.GmJob = .{
            .action = action,
            .target_str = parsed.target_str,
            .int_a = parsed.int_a,
            .int_b = parsed.int_b,
            .int_c = parsed.int_c,
            .flag = is_admin or parsed.flag,
            .raw_cmd = parsed.raw_cmd,
            .response_gpa = gpa,
        };
        if (!(try session_manager.submitGmJob(io, gpa, parsed.role_id, &job))) {
            try writeHttpResponse(writer, "200 OK", "application/json", "{\"ok\":false,\"message\":\"Player is offline or not logged in. Please log in to the game first.\"}");
            return;
        }
        defer gpa.free(job.response_json);
        job.done.waitUncancelable(io);
        try writeHttpResponse(writer, "200 OK", "application/json; charset=utf-8", job.response_json);
        return;
    }

    try writeHttpResponse(writer, "404 Not Found", "application/json", "{\"ok\":false,\"message\":\"Not found\"}");
}

fn containsCaseInsensitive(haystack: []const u8, needle: []const u8) bool {
    if (needle.len == 0) return true;
    if (haystack.len < needle.len) return false;
    var i: usize = 0;
    while (i + needle.len <= haystack.len) : (i += 1) {
        if (std.ascii.eqlIgnoreCase(haystack[i .. i + needle.len], needle)) return true;
    }
    return false;
}

fn idMatchesQuery(arena: Allocator, id: i32, text: []const u8, q: []const u8) bool {
    if (q.len == 0) return true;
    if (containsCaseInsensitive(text, q)) return true;
    const id_str = std.fmt.allocPrint(arena, "{d}", .{id}) catch return false;
    return std.mem.indexOf(u8, id_str, q) != null;
}

fn buildCatalogJson(arena: Allocator, assets: *const Assets, query: []const u8) ![]const u8 {
    const kind = queryParam(query, "kind") orelse "quests";
    const q_raw = queryParam(query, "q") orelse "";
    const q = try urlDecode(arena, q_raw);
    const type_filter: ?i32 = if (queryParam(query, "type")) |t|
        if (t.len > 0) std.fmt.parseInt(i32, t, 10) catch null else null
    else
        null;

    var out: Io.Writer.Allocating = .init(arena);
    const max_rows: usize = 200;

    if (std.mem.eql(u8, kind, "quests")) {
        const QuestItem = struct {
            id: i32,
            type: i32,
            type_name: []const u8,
            ch: i32,
            chapter: []const u8,
            reg: i32,
            region: []const u8,
            title: []const u8,
        };
        var rows: std.ArrayList(QuestItem) = .empty;
        // Pass 1: priority story/companion/exploration/collab quests first when listing/searching
        const priority_types = [_]i32{ 1, 3, 16, 9, 2, 7, 10, 14, 15, 17, 4, 11 };
        for (priority_types) |pt| {
            if (type_filter) |tf| {
                if (tf != pt) continue;
            }
            for (assets.tables.quest_data.items) |entry| {
                const d = entry.Data;
                if (d.Id <= 0 or d.Type == 100 or d.Type != pt) continue;
                const title = gm_i18n.resolveQuestTitle(arena, d.Id, d.Type, d.ChapterId, d.RegionId, d.Key);
                const ch_name = gm_i18n.chapterName(d.ChapterId);
                const reg_name = gm_i18n.regionName(d.RegionId);
                const type_name = gm_i18n.questTypeName(d.Type);
                if (!idMatchesQuery(arena, d.Id, title, q) and
                    !containsCaseInsensitive(ch_name, q) and
                    !containsCaseInsensitive(reg_name, q) and
                    !containsCaseInsensitive(type_name, q) and
                    !containsCaseInsensitive(d.Key, q))
                {
                    continue;
                }
                try rows.append(arena, .{
                    .id = d.Id,
                    .type = d.Type,
                    .type_name = type_name,
                    .ch = d.ChapterId,
                    .chapter = ch_name,
                    .reg = d.RegionId,
                    .region = reg_name,
                    .title = title,
                });
                if (rows.items.len >= max_rows) break;
            }
            if (rows.items.len >= max_rows) break;
        }
        try out.writer.print("{f}", .{std.json.fmt(.{ .items = rows.items }, .{})});
        return out.written();
    }

    if (std.mem.eql(u8, kind, "roles")) {
        const RoleCat = struct { id: i32, star: i32, elem: i32, weapon: i32, name: []const u8, abp: []const u8 };
        var rows: std.ArrayList(RoleCat) = .empty;
        for (assets.tables.role_info.items) |r| {
            if (r.Id < 1000 or r.Id > 1999 or r.IsTrial or r.RoleType != 1) continue;
            if (!idMatchesQuery(arena, r.Id, r.UiScenePerformanceABP, q) and !idMatchesQuery(arena, r.Id, r.Name, q)) continue;
            try rows.append(arena, .{
                .id = r.Id,
                .star = r.QualityId,
                .elem = r.ElementId,
                .weapon = r.WeaponType,
                .name = r.Name,
                .abp = r.UiScenePerformanceABP,
            });
            if (rows.items.len >= 200) break;
        }
        try out.writer.print("{f}", .{std.json.fmt(.{ .items = rows.items }, .{})});
        return out.written();
    }

    if (std.mem.eql(u8, kind, "weapons")) {
        const WepCat = struct { id: i32, star: i32, wtype: i32, kind: []const u8 };
        var rows: std.ArrayList(WepCat) = .empty;
        for (assets.tables.weapon_conf.items) |w| {
            const wlabel = gm_i18n.weaponTypeName(w.WeaponType);
            const full_tag = std.fmt.allocPrint(arena, "{s} ({d}★)", .{ wlabel, w.QualityId }) catch wlabel;
            if (!idMatchesQuery(arena, w.ItemId, full_tag, q)) continue;
            try rows.append(arena, .{ .id = w.ItemId, .star = w.QualityId, .wtype = w.WeaponType, .kind = wlabel });
            if (rows.items.len >= max_rows) break;
        }
        try out.writer.print("{f}", .{std.json.fmt(.{ .items = rows.items }, .{})});
        return out.written();
    }

    if (std.mem.eql(u8, kind, "echoes")) {
        const EchoCat = struct { id: i32, mon: i32, star: i32, cost_hint: []const u8, fetters: []const i32 };
        var rows: std.ArrayList(EchoCat) = .empty;
        for (assets.tables.phantom_item.items) |p| {
            const cost_lbl: []const u8 = switch (p.Rarity) {
                0 => "Cost 1 (Common)",
                1 => "Cost 3 (Elite)",
                2 => "Cost 4 (Overlord)",
                3 => "Cost 4 (Calamity)",
                else => "Echo",
            };
            if (!idMatchesQuery(arena, p.ItemId, cost_lbl, q) and !idMatchesQuery(arena, p.MonsterId, cost_lbl, q)) continue;
            try rows.append(arena, .{ .id = p.ItemId, .mon = p.MonsterId, .star = p.QualityId, .cost_hint = cost_lbl, .fetters = p.FetterGroup });
            if (rows.items.len >= max_rows) break;
        }
        try out.writer.print("{f}", .{std.json.fmt(.{ .items = rows.items }, .{})});
        return out.written();
    }

    const ItemCat = struct { id: i32, star: i32, kind: []const u8 };
    var rows: std.ArrayList(ItemCat) = .empty;
    for (assets.tables.item_info.items) |it| {
        const tag = gm_i18n.itemKindName(it.MainTypeId, it.ItemType);
        if (!idMatchesQuery(arena, it.Id, tag, q)) continue;
        try rows.append(arena, .{ .id = it.Id, .star = it.QualityId, .kind = tag });
        if (rows.items.len >= max_rows) break;
    }
    try out.writer.print("{f}", .{std.json.fmt(.{ .items = rows.items }, .{})});
    return out.written();
}

fn invokeCommand(
    state: *State,
    gpa: Allocator,
    io: Io,
    events: *EventQueue,
    comptime Func: anytype,
    extra_args: anytype,
) !void {
    const Fn = @TypeOf(Func);
    const params = @typeInfo(Fn).@"fn".params;
    var args: std.meta.ArgsTuple(Fn) = undefined;
    comptime var extra_idx: usize = 0;

    inline for (params, 0..) |param, i| {
        const PType = param.type.?;
        if (comptime PType == *EventQueue) {
            args[i] = events;
        } else if (comptime canExtractType(PType)) {
            args[i] = try state.extract(gpa, io, PType);
        } else {
            args[i] = extra_args[extra_idx];
            extra_idx += 1;
        }
    }
    try @call(.auto, Func, args);
}

fn canExtractType(comptime T: type) bool {
    if (T == Io or
        T == *@import("common").FileSystem or
        T == *Connection or
        T == *const Assets or
        T == @import("../logic/PlayerID.zig") or
        T == mem.Alloc or
        T == *?@import("../logic/Scene.zig") or
        T == *@import("../logic/Scene.zig") or
        T == *@import("../logic/schedulers/BuffTimerScheduler.zig") or
        T == *@import("../logic/schedulers/DirtySaveQueue.zig") or
        T == *@import("../logic/CoopService.zig") or
        T == ?*@import("../logic/CoopService.zig") or
        T == *State)
    {
        return true;
    }
    if (@typeInfo(T) == .@"struct" and @hasDecl(T, "scene_query_types")) return true;
    return @import("../logic/component/player/PlayerComponentStorage.zig").hasComponent(T);
}

const ActiveQuestSnapshot = struct {
    id: i32,
    type: i32,
    type_name: []const u8,
    chapter: []const u8,
    region: []const u8,
    title: []const u8,
    tracked: bool,
    active_nodes: []const i32,
};

const PlayerSnapshot = struct {
    online: bool = true,
    online_players: []const i32,
    uid: i32,
    name: []const u8,
    union_lv: i32,
    world_lv: i32,
    astrite: i32,
    coin: i32,
    cash_coin: i32,
    energy: i32,
    store_energy: i32,
    tp_unlocked: usize,
    tp_total: usize,
    roles_owned: usize,
    quest_res_state: i64,
    quest_block_state: i64,
    missing_resource_quests: []const i32,
    active_quests: []const ActiveQuestSnapshot,
};

fn buildSnapshot(
    state: *State,
    io: Io,
    session_manager: *SessionManager,
    include_online_players: bool,
    arena: Allocator,
) !PlayerSnapshot {
    const basic = &state.player_components.basic.info;
    const quest = &state.player_components.quest.info;
    const map_info = &state.player_components.map.info;
    const progress = &state.player_components.progress.info;

    var aq_list: std.ArrayList(ActiveQuestSnapshot) = .empty;
    for (quest.active_quests) |aq| {
        const meta = if (state.assets.tables.quest_data.getDataById(aq.quest_id)) |qd| qd.Data else null;
        const qtype = if (meta) |m| m.Type else 0;
        const ch_id = if (meta) |m| m.ChapterId else 0;
        const reg_id = if (meta) |m| m.RegionId else 0;
        const raw_key = if (meta) |m| m.Key else "";
        try aq_list.append(arena, .{
            .id = aq.quest_id,
            .type = qtype,
            .type_name = gm_i18n.questTypeName(qtype),
            .chapter = gm_i18n.chapterName(ch_id),
            .region = gm_i18n.regionName(reg_id),
            .title = gm_i18n.resolveQuestTitle(arena, aq.quest_id, qtype, ch_id, reg_id, raw_key),
            .tracked = quest.tracked_quest_id == aq.quest_id,
            .active_nodes = aq.active_nodes,
        });
    }

    var missing_res: std.ArrayList(i32) = .empty;
    var tok = std.mem.tokenizeScalar(u8, progress.getKeyStr("quest:missing_res"), ',');
    while (tok.next()) |s| {
        const qid = std.fmt.parseInt(i32, s, 10) catch continue;
        if (qid > 0) try missing_res.append(arena, qid);
    }

    const online_ids: []const i32 = if (include_online_players)
        try session_manager.listOnlineIds(io, arena)
    else
        &.{};
    return .{
        .online_players = online_ids,
        .uid = state.player_id.id,
        .name = basic.attributes.name,
        .union_lv = basic.attributes.level,
        .world_lv = basic.attributes.cur_world_level,
        .astrite = basic.attributes.rare_coin,
        .coin = basic.attributes.coin,
        .cash_coin = basic.attributes.cash_coin,
        .energy = basic.energy_count,
        .store_energy = basic.store_energy_count,
        .tp_unlocked = map_info.unlocked_teleporters.len,
        .tp_total = state.assets.tables.teleporter.items.len,
        .roles_owned = state.player_components.role.role_map.count(),
        .quest_res_state = progress.getKeyInt("quest:res_state"),
        .quest_block_state = progress.getKeyInt("quest:block_state"),
        .missing_resource_quests = missing_res.items,
        .active_quests = aq_list.items,
    };
}

pub fn executeSessionJob(
    state: *State,
    gpa: Allocator,
    io: Io,
    session_manager: *SessionManager,
    job: *gm_bridge.GmJob,
) void {
    const arena = state.arena.allocator();
    defer _ = state.arena.reset(.free_all);

    state.gm_capture = true;
    state.gm_output = .empty;
    defer {
        state.gm_capture = false;
        state.gm_output = .empty;
    }

    var events: EventQueue = .{ .arena = arena };
    var ok = true;
    var err_msg: []const u8 = "";

    const action_res: anyerror!void = switch (job.action) {
        .get_state => {},
        .exec_cmd => commands.dispatch(state, gpa, io, &events, job.raw_cmd),
        .give_item => invokeCommand(state, gpa, io, &events, progression.give.call, .{
            job.int_a,
            @as(?i32, if (job.int_b > 0) job.int_b else 1),
        }),
        .set_union_level => invokeCommand(state, gpa, io, &events, progression.level.call, .{
            @as([]const u8, "union"),
            job.int_a,
        }),
        .set_world_level => invokeCommand(state, gpa, io, &events, cheat_cmd.world_level.call, .{job.int_a}),
        .set_role_level => invokeCommand(state, gpa, io, &events, progression.level.call, .{
            if (job.target_str.len > 0) job.target_str else @as([]const u8, "team"),
            job.int_a,
        }),
        .unlock_roles => invokeCommand(state, gpa, io, &events, cheat_cmd.unlock_roles.call, .{
            @as(?[]const u8, if (job.target_str.len > 0) job.target_str else "all"),
        }),
        .role_max => invokeCommand(state, gpa, io, &events, cheat_cmd.role_max.call, .{
            @as(?[]const u8, if (job.target_str.len > 0) job.target_str else "team"),
            @as(?i32, if (job.int_a > 0) job.int_a else 90),
        }),
        .role_chain => invokeCommand(state, gpa, io, &events, cheat_cmd.role_chain.call, .{
            if (job.target_str.len > 0) job.target_str else @as([]const u8, "team"),
            job.int_a,
        }),
        .weapon_max => invokeCommand(state, gpa, io, &events, cheat_cmd.weapon_max.call, .{
            @as(?[]const u8, if (job.target_str.len > 0) job.target_str else "equipped"),
            @as(?i32, if (job.int_a > 0) job.int_a else 90),
            @as(?i32, if (job.int_b > 0) job.int_b else 5),
        }),
        .quest_pick => invokeCommand(state, gpa, io, &events, cheat_cmd.quest_pick.call, .{
            job.int_a,
            @as(?bool, job.flag),
        }),
        .quest_step => invokeCommand(state, gpa, io, &events, cheat_cmd.quest_step.call, .{
            @as(?i32, if (job.int_a > 0) job.int_a else null),
        }),
        .quest_skip => invokeCommand(state, gpa, io, &events, cheat_cmd.quest_skip.call, .{
            @as(?i32, if (job.int_a > 0) job.int_a else null),
        }),
        .unlock_tp => invokeCommand(state, gpa, io, &events, cheat_cmd.unlock_tp.call, .{}),
        .unlock_func => invokeCommand(state, gpa, io, &events, cheat_cmd.unlock_func.call, .{}),
        .time_set => invokeCommand(state, gpa, io, &events, cheat_cmd.time_set.call, .{
            job.int_a,
            @as(?i32, job.int_b),
        }),
        .heal_team => invokeCommand(state, gpa, io, &events, attr_cmd.heal.call, .{}),
        .reset_cd => invokeCommand(state, gpa, io, &events, js_cmd.reset_cd.call, .{}),
        .no_cd_mode => invokeCommand(state, gpa, io, &events, js_cmd.team_cd_controller.call, .{job.int_a}),
        .spawn_echo => commands.dispatch(state, gpa, io, &events, job.raw_cmd),
        .repair_account => invokeCommand(state, gpa, io, &events, cheat_cmd.repair_account.call, .{}),
        .switch_rover => invokeCommand(state, gpa, io, &events, cheat_cmd.switch_rover.call, .{
            if (job.target_str.len > 0) job.target_str else @as([]const u8, "havoc"),
        }),
        .reset_player, .rotate_gm_key => {},
    };

    action_res catch |err| {
        ok = false;
        err_msg = @errorName(err);
    };

    logic_handlers.drainEventQueueBestEffort(&events, state, gpa, io);

    const msg: []const u8 = if (!ok)
        std.fmt.allocPrint(arena, "Error: {s}{s}{s}", .{
            err_msg,
            if (state.gm_output.items.len > 0) "\n" else "",
            state.gm_output.items,
        }) catch err_msg
    else if (state.gm_output.items.len > 0)
        state.gm_output.items
    else
        "OK";

    const include_online = (job.action == .get_state) and job.flag;
    const snap = buildSnapshot(state, io, session_manager, include_online, arena) catch {
        job.failAndSignal(io, msg);
        return;
    };

    var out: Io.Writer.Allocating = .init(job.response_gpa);
    if (job.action == .get_state) {
        out.writer.print("{f}", .{std.json.fmt(snap, .{})}) catch {
            out.deinit();
            job.failAndSignal(io, "Serialization failed");
            return;
        };
    } else {
        const RespPayload = struct {
            ok: bool,
            message: []const u8,
            state: PlayerSnapshot,
        };
        out.writer.print("{f}", .{std.json.fmt(RespPayload{
            .ok = ok,
            .message = msg,
            .state = snap,
        }, .{})}) catch {
            out.deinit();
            job.failAndSignal(io, "Serialization failed");
            return;
        };
    }

    job.response_json = out.toOwnedSlice() catch &.{};
    job.ok = ok;
    job.done.set(io);
}

fn purgePlayerSqlite(arena: Allocator, target_id: i32) void {
    var db = common.sqlite.Db.open("state/gamesv.db") catch |err| {
        log.warn("failed to open state/gamesv.db for reset: {t}", .{err});
        return;
    };
    defer db.close();

    const pattern = std.fmt.allocPrint(arena, "player/{d}/%", .{target_id}) catch return;
    if (db.prepare("DELETE FROM state_files WHERE path LIKE ?1;")) |stmt_val| {
        var stmt = stmt_val;
        defer stmt.deinit();
        if (stmt.bindText(1, pattern)) {
            _ = stmt.step() catch |err| {
                std.log.scoped(.gm_http).warn("resetPlayer state_files delete failed for uid={d}: {t}", .{ target_id, err });
            };
        } else |err| {
            std.log.scoped(.gm_http).warn("resetPlayer bindText failed for uid={d}: {t}", .{ target_id, err });
        }
    } else |_| {}

    const tables = [_][]const u8{
        "DELETE FROM players WHERE player_id = ?1;",
        "DELETE FROM player_battlepass WHERE player_id = ?1;",
        "DELETE FROM player_activity_claims WHERE player_id = ?1;",
        "DELETE FROM player_liveness WHERE player_id = ?1;",
        "DELETE FROM player_achievements WHERE player_id = ?1;",
        "DELETE FROM player_shop_bought WHERE player_id = ?1;",
        "DELETE FROM player_kv WHERE player_id = ?1;",
    };
    for (tables) |sql| {
        var stmt = db.prepare(sql) catch continue;
        defer stmt.deinit();
        stmt.bindInt(1, target_id) catch continue;
        _ = stmt.step() catch |err| {
            log.warn("reset sqlite step failed for {s}: {t}", .{ sql, err });
        };
    }
}
