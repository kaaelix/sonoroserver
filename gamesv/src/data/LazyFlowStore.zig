const LazyFlowStore = @This();
const std = @import("std");
const Io = std.Io;
const Allocator = std.mem.Allocator;
const ArenaAllocator = std.heap.ArenaAllocator;

pub const ByteSpan = struct {
    offset: u64,
    len: u32,
};

const CacheSlot = struct {
    state_key: []const u8 = "",
    arena: ArenaAllocator,
    state: std.json.Value = .null,
};

const CacheState = struct {
    mutex: Io.Mutex = .init,
    slots: std.StringHashMapUnmanaged(CacheSlot) = .empty,
};

gpa: Allocator,
index_arena: ArenaAllocator,
file_path: []const u8,
state_spans: std.StringHashMapUnmanaged(ByteSpan),
cache: *CacheState,

const JsonObjectScanner = struct {
    depth: usize = 0,
    obj_start: u64 = 0,
    in_object: bool = false,
    in_string: bool = false,
    escape: bool = false,
    header_buf: [4096]u8 = undefined,
    header_len: usize = 0,

    fn feed(
        self: *JsonObjectScanner,
        gpa: Allocator,
        arena: Allocator,
        io: ?Io,
        file: ?Io.File,
        chunk: []const u8,
        file_offset: u64,
        spans: *std.StringHashMapUnmanaged(ByteSpan),
    ) !void {
        for (chunk, 0..) |ch, i| {
            const abs_pos = file_offset + i;
            if (self.in_object and self.header_len < self.header_buf.len) {
                self.header_buf[self.header_len] = ch;
                self.header_len += 1;
            }

            if (self.in_string) {
                if (self.escape) {
                    self.escape = false;
                } else if (ch == '\\') {
                    self.escape = true;
                } else if (ch == '"') {
                    self.in_string = false;
                }
                continue;
            }

            if (ch == '"') {
                self.in_string = true;
                continue;
            }

            if (ch == '{') {
                if (self.depth == 0) {
                    self.in_object = true;
                    self.obj_start = abs_pos;
                    self.header_len = 1;
                    self.header_buf[0] = '{';
                }
                self.depth += 1;
            } else if (ch == '}') {
                if (self.depth > 0) self.depth -= 1;
                if (self.depth == 0 and self.in_object) {
                    self.in_object = false;
                    const obj_end = abs_pos + 1;
                    const obj_len_u64 = obj_end - self.obj_start;
                    var fallback_buf: ?[]u8 = null;
                    defer if (fallback_buf) |b| gpa.free(b);
                    const raw_key_opt = extractStateKey(self.header_buf[0..self.header_len]) orelse blk: {
                        if (io == null or file == null) break :blk null;
                        if (obj_len_u64 <= self.header_buf.len or obj_len_u64 > 1024 * 1024) break :blk null;
                        const buf = try gpa.alloc(u8, @intCast(obj_len_u64));
                        fallback_buf = buf;
                        const rn = try file.?.readPositionalAll(io.?, buf, self.obj_start);
                        break :blk extractStateKey(buf[0..rn]);
                    };
                    if (raw_key_opt) |raw_key| {
                        if (std.math.cast(u32, obj_len_u64)) |obj_len| {
                            const key = try arena.dupe(u8, raw_key);
                            try spans.put(gpa, key, .{
                                .offset = self.obj_start,
                                .len = obj_len,
                            });
                        }
                    }
                    self.header_len = 0;
                }
            }
        }
    }
};

pub fn init(gpa: Allocator, io: Io, file_path: []const u8) !LazyFlowStore {
    var index_arena = ArenaAllocator.init(gpa);
    errdefer index_arena.deinit();
    const arena = index_arena.allocator();

    const cache = try gpa.create(CacheState);
    errdefer gpa.destroy(cache);
    cache.* = .{};

    var temp_spans: std.StringHashMapUnmanaged(ByteSpan) = .empty;
    defer temp_spans.deinit(gpa);

    var file = try Io.Dir.cwd().openFile(io, file_path, .{});
    defer file.close(io);

    var read_buf: [64 * 1024]u8 = undefined;
    var scanner: JsonObjectScanner = .{};
    var file_offset: u64 = 0;

    while (true) {
        const n = try file.readPositionalAll(io, &read_buf, file_offset);
        if (n == 0) break;
        try scanner.feed(gpa, arena, io, file, read_buf[0..n], file_offset, &temp_spans);
        file_offset += n;
    }

    var state_spans: std.StringHashMapUnmanaged(ByteSpan) = .empty;
    try state_spans.ensureTotalCapacity(arena, @intCast(temp_spans.count()));
    var it = temp_spans.iterator();
    while (it.next()) |kv| {
        state_spans.putAssumeCapacity(kv.key_ptr.*, kv.value_ptr.*);
    }

    return .{
        .gpa = gpa,
        .index_arena = index_arena,
        .file_path = try arena.dupe(u8, file_path),
        .state_spans = state_spans,
        .cache = cache,
    };
}

pub fn deinit(self: *LazyFlowStore) void {
    var it = self.cache.slots.valueIterator();
    while (it.next()) |slot| {
        slot.arena.deinit();
    }
    self.cache.slots.deinit(self.gpa);
    self.gpa.destroy(self.cache);
    self.index_arena.deinit();
}

pub fn hasStateKey(self: *const LazyFlowStore, state_key: []const u8) bool {
    return self.state_spans.contains(state_key);
}

pub fn getFlowState(
    self: *const LazyFlowStore,
    io: Io,
    arena: Allocator,
    flow_list_name: []const u8,
    flow_id: i32,
    state_id: i32,
) !?std.json.Value {
    if (flow_list_name.len == 0 or flow_id <= 0 or state_id <= 0) return null;
    const key = try std.fmt.allocPrint(arena, "{s}_{d}_{d}", .{ flow_list_name, flow_id, state_id });
    return try self.getStateByKey(io, key);
}

pub fn getStateByKey(self: *const LazyFlowStore, io: Io, state_key: []const u8) !?std.json.Value {
    try self.cache.mutex.lock(io);
    defer self.cache.mutex.unlock(io);

    if (self.cache.slots.get(state_key)) |slot| {
        return slot.state;
    }

    const span = self.state_spans.get(state_key) orelse return null;

    var slot_arena = ArenaAllocator.init(self.gpa);
    errdefer slot_arena.deinit();
    const arena = slot_arena.allocator();

    var file = try Io.Dir.cwd().openFile(io, self.file_path, .{});
    defer file.close(io);

    const buf = try arena.alloc(u8, span.len);
    const read_n = try file.readPositionalAll(io, buf, span.offset);
    if (read_n != span.len) return error.UnexpectedEof;

    const parsed = try std.json.parseFromSliceLeaky(
        std.json.Value,
        arena,
        buf,
        .{ .allocate = .alloc_always },
    );

    const dup_key = try arena.dupe(u8, state_key);
    try self.cache.slots.put(self.gpa, dup_key, .{
        .state_key = dup_key,
        .arena = slot_arena,
        .state = parsed,
    });
    return parsed;
}

/// Recursively locates an action object by `ActionId` anywhere in a parsed FlowState tree.
pub fn findActionById(val: std.json.Value, target_action_id: i32) ?std.json.Value {
    switch (val) {
        .object => |obj| {
            if (obj.get("Name") != null) {
                if (obj.get("ActionId")) |id_val| {
                    if (id_val == .integer and id_val.integer == target_action_id) {
                        return val;
                    }
                }
            }
            var it = obj.iterator();
            while (it.next()) |entry| {
                if (findActionById(entry.value_ptr.*, target_action_id)) |found| {
                    return found;
                }
            }
        },
        .array => |arr| {
            for (arr.items) |item| {
                if (findActionById(item, target_action_id)) |found| {
                    return found;
                }
            }
        },
        else => {},
    }
    return null;
}

/// Collects positive entity config IDs referenced by `SetEntityVisible`, `AwakeEntity`,
/// `ChangeEntityState`, `SetEntityPos`, `EntityLookAt`, and `BeginFlowTemplate.Actors`
/// anywhere in a FlowState tree so the server can ensure they exist before `FlowStartNotify`.
pub fn collectSetEntityVisibleIds(arena: Allocator, val: std.json.Value, out: *std.ArrayList(i32)) !void {
    switch (val) {
        .object => |obj| {
            if (obj.get("Name")) |name_val| {
                if (name_val == .string) {
                    const name = name_val.string;
                    if (obj.get("Params")) |params_val| {
                        if (params_val == .object) {
                            const params = params_val.object;
                            if (std.mem.eql(u8, name, "SetEntityVisible") or
                                std.mem.eql(u8, name, "AwakeEntity") or
                                std.mem.eql(u8, name, "ChangeEntityState"))
                            {
                                try appendEntityIdList(arena, params.get("EntityIds"), out);
                                try appendSingleEntityId(arena, params.get("EntityId"), out);
                            } else if (std.mem.eql(u8, name, "SetEntityPos")) {
                                try appendSingleEntityId(arena, params.get("EntityId"), out);
                            } else if (std.mem.eql(u8, name, "EntityLookAt")) {
                                try appendSingleEntityId(arena, params.get("EntityId"), out);
                                try appendSingleEntityId(arena, params.get("TargetEntityId"), out);
                            } else if (std.mem.eql(u8, name, "BeginFlowTemplate")) {
                                if (params.get("Actors")) |actors_val| {
                                    if (actors_val == .array) {
                                        for (actors_val.array.items) |actor_item| {
                                            if (actor_item == .object) {
                                                try appendSingleEntityId(arena, actor_item.object.get("EntityId"), out);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            var it = obj.iterator();
            while (it.next()) |entry| {
                try collectSetEntityVisibleIds(arena, entry.value_ptr.*, out);
            }
        },
        .array => |arr| {
            for (arr.items) |item| {
                try collectSetEntityVisibleIds(arena, item, out);
            }
        },
        else => {},
    }
}

/// Returns true if the FlowState switches to the player's main role (Rover) for cutscenes/dialogue.
pub fn doesFlowSwitchMainRole(val: std.json.Value) bool {
    switch (val) {
        .object => |obj| {
            if (obj.get("IsSwitchMainRole")) |v| {
                if (v == .bool and v.bool) return true;
            }
            if (obj.get("Name")) |name_val| {
                if (name_val == .string and std.mem.eql(u8, name_val.string, "BeginFlowTemplate")) {
                    if (obj.get("Params")) |pv| {
                        if (pv == .object) {
                            if (pv.object.get("Actors")) |actors_val| {
                                if (actors_val == .array) {
                                    for (actors_val.array.items) |actor_item| {
                                        if (actor_item != .object) continue;
                                        const is_player = if (actor_item.object.get("IsPlayer")) |ip| ip == .bool and ip.bool else false;
                                        const eid = parseJsonInt(actor_item.object.get("EntityId")) orelse 0;
                                        if (is_player or eid == -1) return true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            var it = obj.iterator();
            while (it.next()) |entry| {
                if (doesFlowSwitchMainRole(entry.value_ptr.*)) return true;
            }
        },
        .array => |arr| {
            for (arr.items) |item| {
                if (doesFlowSwitchMainRole(item)) return true;
            }
        },
        else => {},
    }
    return false;
}

/// Collects entity config IDs awakened or explicitly shown by `AwakeEntity`, `BeginFlowTemplate`, or `SetEntityVisible` (`Visible != false`).
pub fn collectAwakeEntityIds(arena: Allocator, val: std.json.Value, out: *std.ArrayList(i32)) !void {
    switch (val) {
        .object => |obj| {
            if (obj.get("Name")) |name_val| {
                if (name_val == .string) {
                    const name = name_val.string;
                    if (obj.get("Params")) |params_val| {
                        if (params_val == .object) {
                            const p = params_val.object;
                            if (std.mem.eql(u8, name, "AwakeEntity")) {
                                try appendSingleEntityId(arena, p.get("EntityId"), out);
                                try appendEntityIdList(arena, p.get("EntityIds"), out);
                            } else if (std.mem.eql(u8, name, "SetEntityVisible")) {
                                const visible = if (p.get("Visible")) |vv| switch (vv) {
                                    .bool => |b| b,
                                    else => true,
                                } else true;
                                if (visible) {
                                    try appendSingleEntityId(arena, p.get("EntityId"), out);
                                    try appendEntityIdList(arena, p.get("EntityIds"), out);
                                }
                            } else if (std.mem.eql(u8, name, "BeginFlowTemplate")) {
                                if (p.get("Actors")) |actors_val| {
                                    if (actors_val == .array) {
                                        for (actors_val.array.items) |actor_item| {
                                            if (actor_item == .object) {
                                                try appendSingleEntityId(arena, actor_item.object.get("EntityId"), out);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            var it = obj.iterator();
            while (it.next()) |entry| {
                try collectAwakeEntityIds(arena, entry.value_ptr.*, out);
            }
        },
        .array => |arr| {
            for (arr.items) |item| {
                try collectAwakeEntityIds(arena, item, out);
            }
        },
        else => {},
    }
}

fn parseJsonInt(v_opt: ?std.json.Value) ?i32 {
    const v = v_opt orelse return null;
    return switch (v) {
        .integer => |i| std.math.cast(i32, i),
        .float => |f| if (std.math.isFinite(f)) @intFromFloat(f) else null,
        .string => |s| std.fmt.parseInt(i32, s, 10) catch null,
        else => null,
    };
}

fn appendSingleEntityId(arena: Allocator, v_opt: ?std.json.Value, out: *std.ArrayList(i32)) !void {
    const eid = parseJsonInt(v_opt) orelse return;
    if (eid > 0 and !containsInt(out.items, eid)) {
        try out.append(arena, eid);
    }
}

fn appendEntityIdList(arena: Allocator, ids_val_opt: ?std.json.Value, out: *std.ArrayList(i32)) !void {
    const ids_val = ids_val_opt orelse return;
    if (ids_val != .array) return;
    for (ids_val.array.items) |id_item| {
        try appendSingleEntityId(arena, id_item, out);
    }
}

fn containsInt(slice: []const i32, target: i32) bool {
    for (slice) |item| {
        if (item == target) return true;
    }
    return false;
}

fn extractStateKey(header: []const u8) ?[]const u8 {
    const key_marker = "\"StateKey\"";
    const key_pos = std.mem.indexOf(u8, header, key_marker) orelse return null;
    const after_marker = header[key_pos + key_marker.len ..];
    const colon_rel = std.mem.indexOfScalar(u8, after_marker, ':') orelse return null;
    const after_colon = after_marker[colon_rel + 1 ..];
    const open_quote = std.mem.indexOfScalar(u8, after_colon, '"') orelse return null;
    const str_start = after_colon[open_quote + 1 ..];

    var i: usize = 0;
    while (i < str_start.len) : (i += 1) {
        if (str_start[i] == '\\') {
            i += 1;
            continue;
        }
        if (str_start[i] == '"') {
            return str_start[0..i];
        }
    }
    return null;
}

test "indexes StateKey spans across chunk boundaries and extracts nested actions" {
    const testing = std.testing;
    const sample =
        \\[
        \\  {
        \\    "StateKey": "剧情_新主线_CE版_序章_1_12",
        \\    "Id": 12,
        \\    "Actions": [
        \\      {
        \\        "Name": "ShowTalk",
        \\        "ActionId": 5000013,
        \\        "Params": {
        \\          "TalkItems": [
        \\            {
        \\              "Id": 7,
        \\              "Actions": [
        \\                {
        \\                  "Name": "SetEntityVisible",
        \\                  "ActionId": 5000157,
        \\                  "Params": { "Visible": true, "EntityIds": [139000395] }
        \\                }
        \\              ]
        \\            }
        \\          ]
        \\        }
        \\      },
        \\      {
        \\        "Name": "SetEntityVisible",
        \\        "ActionId": 5000007,
        \\        "Params": { "Visible": false, "EntityIds": [122002966] }
        \\      }
        \\    ]
        \\  }
        \\]
    ;

    var arena_state = ArenaAllocator.init(testing.allocator);
    defer arena_state.deinit();
    const arena = arena_state.allocator();

    var spans: std.StringHashMapUnmanaged(ByteSpan) = .empty;
    defer spans.deinit(testing.allocator);
    var scanner: JsonObjectScanner = .{};

    // Feed small chunks to verify chunk-boundary robustness.
    var offset: usize = 0;
    while (offset < sample.len) {
        const end = @min(sample.len, offset + 17);
        try scanner.feed(testing.allocator, arena, null, null, sample[offset..end], offset, &spans);
        offset = end;
    }

    const span = spans.get("剧情_新主线_CE版_序章_1_12").?;
    const slice = sample[span.offset .. span.offset + span.len];
    const parsed = try std.json.parseFromSliceLeaky(std.json.Value, arena, slice, .{ .allocate = .alloc_always });

    const nested_act = findActionById(parsed, 5000157).?;
    try testing.expectEqualStrings("SetEntityVisible", nested_act.object.get("Name").?.string);

    const top_act = findActionById(parsed, 5000007).?;
    try testing.expectEqualStrings("SetEntityVisible", top_act.object.get("Name").?.string);
    try testing.expect(findActionById(parsed, 999999) == null);

    var entity_ids: std.ArrayList(i32) = .empty;
    try collectSetEntityVisibleIds(arena, parsed, &entity_ids);
    try testing.expectEqualSlices(i32, &.{ 139000395, 122002966 }, entity_ids.items);
}

test "collects AwakeEntity, ChangeEntityState, and BeginFlowTemplate actors plus main role switch" {
    const testing = std.testing;
    const sample =
        \\{
        \\  "StateKey": "剧情_test_1_1",
        \\  "Actions": [
        \\    {
        \\      "Name": "BeginFlowTemplate",
        \\      "Params": {
        \\        "IsSwitchMainRole": true,
        \\        "Actors": [
        \\          { "EntityId": -1, "IsPlayer": true, "TalkerId": 750088 },
        \\          { "EntityId": 145750135 }
        \\        ]
        \\      }
        \\    },
        \\    {
        \\      "Name": "AwakeEntity",
        \\      "Params": { "EntityIds": [145000246, 145750147] }
        \\    },
        \\    {
        \\      "Name": "ChangeEntityState",
        \\      "Params": { "EntityId": 147900228 }
        \\    }
        \\  ]
        \\}
    ;

    var arena_state = ArenaAllocator.init(testing.allocator);
    defer arena_state.deinit();
    const arena = arena_state.allocator();
    const parsed = try std.json.parseFromSliceLeaky(std.json.Value, arena, sample, .{ .allocate = .alloc_always });

    var entity_ids: std.ArrayList(i32) = .empty;
    try collectSetEntityVisibleIds(arena, parsed, &entity_ids);
    try testing.expectEqualSlices(i32, &.{ 145750135, 145000246, 145750147, 147900228 }, entity_ids.items);
    try testing.expect(doesFlowSwitchMainRole(parsed));

    var awake_ids: std.ArrayList(i32) = .empty;
    try collectAwakeEntityIds(arena, parsed, &awake_ids);
    try testing.expectEqualSlices(i32, &.{ 145750135, 145000246, 145750147 }, awake_ids.items);
}

test "collectAwakeEntityIds skips SetEntityVisible hide and DestroyEntity" {
    const testing = std.testing;
    const sample =
        \\{
        \\  "StateKey": "剧情_test_1_2",
        \\  "Actions": [
        \\    { "Name": "SetEntityVisible", "Params": { "Visible": false, "EntityIds": [101, 102] } },
        \\    { "Name": "DestroyEntity", "Params": { "EntityIds": [103] } },
        \\    { "Name": "SetEntityVisible", "Params": { "Visible": true, "EntityIds": [104] } },
        \\    { "Name": "AwakeEntity", "Params": { "EntityId": 105 } }
        \\  ]
        \\}
    ;

    var arena_state = ArenaAllocator.init(testing.allocator);
    defer arena_state.deinit();
    const arena = arena_state.allocator();
    const parsed = try std.json.parseFromSliceLeaky(std.json.Value, arena, sample, .{ .allocate = .alloc_always });

    var awake_ids: std.ArrayList(i32) = .empty;
    try collectAwakeEntityIds(arena, parsed, &awake_ids);
    try testing.expectEqualSlices(i32, &.{ 104, 105 }, awake_ids.items);
}
