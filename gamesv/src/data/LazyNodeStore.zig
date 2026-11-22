const LazyNodeStore = @This();
const std = @import("std");
const Io = std.Io;
const Allocator = std.mem.Allocator;
const ArenaAllocator = std.heap.ArenaAllocator;

pub const ByteSpan = struct {
    offset: u64,
    len: u32,
};

pub const NodeEntry = struct {
    id: i32,
    parent_id: i32,
    node_type: []const u8,
    count: i32,
    reward_id: i32,
    data: std.json.Value,
};

const CacheSlot = struct {
    arena: ArenaAllocator,
    nodes: []const NodeEntry = &.{},
};

const CacheState = struct {
    mutex: Io.Mutex = .init,
    slots: std.AutoHashMapUnmanaged(i32, CacheSlot) = .empty,
};

gpa: Allocator,
index_arena: ArenaAllocator,
file_path: []const u8,
owner_spans: std.AutoHashMapUnmanaged(i32, []const ByteSpan),
cache: *CacheState,

pub fn init(gpa: Allocator, io: Io, file_path: []const u8) !LazyNodeStore {
    var index_arena = ArenaAllocator.init(gpa);
    errdefer index_arena.deinit();
    const arena = index_arena.allocator();

    const cache = try gpa.create(CacheState);
    errdefer gpa.destroy(cache);
    cache.* = .{};

    var temp_lists: std.AutoHashMapUnmanaged(i32, std.ArrayListUnmanaged(ByteSpan)) = .empty;
    defer {
        var it = temp_lists.valueIterator();
        while (it.next()) |list| list.deinit(gpa);
        temp_lists.deinit(gpa);
    }

    var file = try Io.Dir.cwd().openFile(io, file_path, .{});
    defer file.close(io);

    var read_buf: [64 * 1024]u8 = undefined;
    var header_buf: [4096]u8 = undefined;
    var header_len: usize = 0;

    var file_offset: u64 = 0;
    var depth: usize = 0;
    var obj_start: u64 = 0;
    var in_object = false;
    var in_string = false;
    var escape = false;

    while (true) {
        const n = try file.readPositionalAll(io, &read_buf, file_offset);
        if (n == 0) break;

        for (read_buf[0..n], 0..) |ch, i| {
            const abs_pos = file_offset + i;
            if (in_object and header_len < header_buf.len) {
                header_buf[header_len] = ch;
                header_len += 1;
            }

            if (in_string) {
                if (escape) {
                    escape = false;
                } else if (ch == '\\') {
                    escape = true;
                } else if (ch == '"') {
                    in_string = false;
                }
                continue;
            }

            if (ch == '"') {
                in_string = true;
                continue;
            }

            if (ch == '{') {
                if (depth == 0) {
                    in_object = true;
                    obj_start = abs_pos;
                    header_len = 1;
                    header_buf[0] = '{';
                }
                depth += 1;
            } else if (ch == '}') {
                if (depth > 0) depth -= 1;
                if (depth == 0 and in_object) {
                    in_object = false;
                    const obj_end = abs_pos + 1;
                    const obj_len_u64 = obj_end - obj_start;
                    const owner_opt = parseOwnerId(header_buf[0..header_len]) orelse blk: {
                        if (obj_len_u64 <= header_buf.len or obj_len_u64 > 1024 * 1024) break :blk null;
                        const full_buf = try gpa.alloc(u8, @intCast(obj_len_u64));
                        defer gpa.free(full_buf);
                        const rn = try file.readPositionalAll(io, full_buf, obj_start);
                        break :blk parseOwnerId(full_buf[0..rn]);
                    };
                    if (owner_opt) |owner_id| {
                        if (std.math.cast(u32, obj_len_u64)) |obj_len| {
                            const gop = try temp_lists.getOrPut(gpa, owner_id);
                            if (!gop.found_existing) gop.value_ptr.* = .empty;
                            try gop.value_ptr.append(gpa, .{
                                .offset = obj_start,
                                .len = obj_len,
                            });
                        }
                    }
                    header_len = 0;
                }
            }
        }
        file_offset += n;
    }

    var owner_spans: std.AutoHashMapUnmanaged(i32, []const ByteSpan) = .empty;
    try owner_spans.ensureTotalCapacity(arena, @intCast(temp_lists.count()));
    var it = temp_lists.iterator();
    while (it.next()) |kv| {
        const duped = try arena.dupe(ByteSpan, kv.value_ptr.items);
        owner_spans.putAssumeCapacity(kv.key_ptr.*, duped);
    }

    return .{
        .gpa = gpa,
        .index_arena = index_arena,
        .file_path = try arena.dupe(u8, file_path),
        .owner_spans = owner_spans,
        .cache = cache,
    };
}

pub fn deinit(self: *LazyNodeStore) void {
    var it = self.cache.slots.valueIterator();
    while (it.next()) |slot| {
        slot.arena.deinit();
    }
    self.cache.slots.deinit(self.gpa);
    self.gpa.destroy(self.cache);
    self.index_arena.deinit();
}

pub fn hasOwner(self: *const LazyNodeStore, owner_id: i32) bool {
    return self.owner_spans.contains(owner_id);
}

pub fn getOwnerNodes(self: *const LazyNodeStore, io: Io, owner_id: i32) ![]const NodeEntry {
    try self.cache.mutex.lock(io);
    defer self.cache.mutex.unlock(io);

    if (self.cache.slots.get(owner_id)) |slot| {
        return slot.nodes;
    }

    const spans = self.owner_spans.get(owner_id) orelse return &.{};

    var slot_arena = ArenaAllocator.init(self.gpa);
    errdefer slot_arena.deinit();
    const arena = slot_arena.allocator();

    var file = try Io.Dir.cwd().openFile(io, self.file_path, .{});
    defer file.close(io);

    var nodes: std.ArrayListUnmanaged(NodeEntry) = .empty;
    for (spans) |span| {
        const buf = try arena.alloc(u8, span.len);
        const read_n = try file.readPositionalAll(io, buf, span.offset);
        if (read_n != span.len) return error.UnexpectedEof;

        const parsed = try std.json.parseFromSliceLeaky(
            std.json.Value,
            arena,
            buf,
            .{ .allocate = .alloc_always },
        );
        if (parsed != .object) continue;
        const data_val = parsed.object.get("Data") orelse continue;
        try appendNodeFlattened(arena, &nodes, data_val, null);
    }

    try normalizeSequenceChildOrder(arena, owner_id, nodes.items);
    const owned_nodes = try arena.dupe(NodeEntry, nodes.items);

    try self.cache.slots.put(self.gpa, owner_id, .{
        .arena = slot_arena,
        .nodes = owned_nodes,
    });
    return owned_nodes;
}

fn appendNodeFlattened(
    arena: Allocator,
    nodes: *std.ArrayListUnmanaged(NodeEntry),
    node_val: std.json.Value,
    override_parent: ?i32,
) !void {
    if (node_val != .object) return;
    const obj = node_val.object;
    const id_val = obj.get("Id") orelse return;
    const id: i32 = switch (id_val) {
        .integer => |v| std.math.cast(i32, v) orelse return,
        else => return,
    };
    const parent_id: i32 = override_parent orelse if (obj.get("ParentNodeId")) |pv| switch (pv) {
        .integer => |v| std.math.cast(i32, v) orelse 0,
        else => 0,
    } else 0;
    const node_type: []const u8 = if (obj.get("Type")) |tv| switch (tv) {
        .string => |s| s,
        else => "",
    } else "";
    const count: i32 = if (obj.get("Count")) |cv| switch (cv) {
        .integer => |v| std.math.cast(i32, v) orelse 1,
        else => 1,
    } else 1;
    const reward_id: i32 = if (obj.get("RewardId")) |rv| switch (rv) {
        .integer => |v| std.math.cast(i32, v) orelse 0,
        else => 0,
    } else 0;

    try nodes.append(arena, .{
        .id = id,
        .parent_id = parent_id,
        .node_type = node_type,
        .count = count,
        .reward_id = reward_id,
        .data = node_val,
    });

    if (obj.get("Children")) |children_val| {
        if (children_val == .array) {
            for (children_val.array.items) |child_val| {
                try appendNodeFlattened(arena, nodes, child_val, id);
            }
        }
    }
    if (obj.get("Slots")) |slots_val| {
        if (slots_val == .array) {
            for (slots_val.array.items) |slot_val| {
                if (slot_val != .object) continue;
                if (slot_val.object.get("Node")) |slot_node| {
                    try appendNodeFlattened(arena, nodes, slot_node, id);
                }
            }
        }
    }
}

fn parseTidTipKey(owner_id: i32, data: std.json.Value) ?u64 {
    if (data != .object) return null;
    const tid_val = data.object.get("TidTip") orelse return null;
    if (tid_val != .string) return null;
    const tid = tid_val.string;
    const last_us = std.mem.lastIndexOfScalar(u8, tid, '_') orelse return null;
    const lo = std.fmt.parseInt(u32, tid[last_us + 1 ..], 10) catch return null;
    var hi: u32 = 0;
    if (std.mem.lastIndexOfScalar(u8, tid[0..last_us], '_')) |prev_us| {
        hi = std.fmt.parseInt(u32, tid[prev_us + 1 .. last_us], 10) catch 0;
    }
    if (owner_id >= 1_000_000) {
        const owner_prefix: u32 = @intCast(@divTrunc(owner_id, 1_000_000));
        if (hi != 0 and hi != owner_prefix) return null;
    }
    return (@as(u64, hi) << 32) | @as(u64, lo);
}

fn findTeleportDungeonTarget(val: std.json.Value) ?i32 {
    switch (val) {
        .object => |obj| {
            if (obj.get("Name")) |nv| {
                if (nv == .string and std.mem.eql(u8, nv.string, "TeleportDungeon")) {
                    if (obj.get("Params")) |pv| {
                        if (pv == .object) {
                            if (pv.object.get("DungeonId")) |dv| {
                                if (dv == .integer) {
                                    const did = std.math.cast(i32, dv.integer) orelse 0;
                                    if (did > 0 and did != 8) return did;
                                }
                            }
                        }
                    }
                }
            }
            var it = obj.iterator();
            while (it.next()) |entry| {
                if (std.mem.eql(u8, entry.key_ptr.*, "Children") or std.mem.eql(u8, entry.key_ptr.*, "Slots")) continue;
                if (findTeleportDungeonTarget(entry.value_ptr.*)) |did| return did;
            }
        },
        .array => |arr| {
            for (arr.items) |item| {
                if (findTeleportDungeonTarget(item)) |did| return did;
            }
        },
        else => {},
    }
    return null;
}

fn parseDungeonId(data: std.json.Value) i32 {
    if (data != .object) return 0;
    const dv = data.object.get("DungeonId") orelse return 0;
    if (dv != .integer) return 0;
    const did = std.math.cast(i32, dv.integer) orelse return 0;
    return if (did > 0 and did != 8) did else 0;
}

fn hasTeleportDungeonFinishAction(data: std.json.Value) bool {
    if (data != .object) return false;
    const acts_val = data.object.get("FinishActions") orelse return false;
    if (acts_val != .array) return false;
    for (acts_val.array.items) |act_val| {
        if (act_val != .object) continue;
        if (act_val.object.get("Name")) |nv| {
            if (nv == .string and std.mem.eql(u8, nv.string, "TeleportDungeon")) return true;
        }
    }
    return false;
}

fn isTerminalQuestNode(node_type: []const u8) bool {
    return std.mem.eql(u8, node_type, "QuestSucceed") or std.mem.eql(u8, node_type, "QuestFailed");
}

/// Restores DFS execution order for `Sequence`/`Select` children when `QuestNodeData.json`
/// rows are out of order (e.g. `QuestSucceed` appearing before patched sibling steps):
/// 1) Terminal `QuestSucceed`/`QuestFailed` children move after non-terminal siblings in `Sequence`.
/// 2) Non-terminal siblings that carry a subtree `TidTip` index are sorted in-place by that index,
///    preserving relative file order for untagged `Action`/`Condition` siblings.
/// 3) Within `Sequence`, a sibling `DungeonId == D` subtree is paired immediately after a sibling
///    containing `TeleportDungeon(D)` (`D > 0, D != 8`) so patched quest branches enter their dungeon first.
fn normalizeSequenceChildOrder(arena: Allocator, owner_id: i32, nodes: []NodeEntry) !void {
    if (nodes.len <= 1) return;

    const min_tids = try arena.alloc(?u64, nodes.len);
    const teleport_dungeons = try arena.alloc(i32, nodes.len);
    const subtree_dungeons = try arena.alloc(i32, nodes.len);
    for (nodes, 0..) |node, i| {
        if (std.mem.eql(u8, node.node_type, "AlwaysFalse") or hasTeleportDungeonFinishAction(node.data)) {
            min_tids[i] = null;
        } else {
            min_tids[i] = parseTidTipKey(owner_id, node.data);
        }
        if (std.mem.eql(u8, node.node_type, "AlwaysFalse")) {
            teleport_dungeons[i] = 0;
            subtree_dungeons[i] = 0;
        } else {
            teleport_dungeons[i] = findTeleportDungeonTarget(node.data) orelse 0;
            subtree_dungeons[i] = parseDungeonId(node.data);
        }
    }

    // Bottom-up propagation of minimum TidTip DFS key and dungeon tags from children to parent.
    var pass: usize = 0;
    while (pass < nodes.len) : (pass += 1) {
        var changed = false;
        for (nodes, 0..) |node, i| {
            if (std.mem.eql(u8, node.node_type, "AlwaysFalse")) continue;
            if (node.parent_id <= 0) continue;
            const child_key = min_tids[i];
            const child_tp = teleport_dungeons[i];
            const child_dung = subtree_dungeons[i];
            for (nodes, 0..) |p, pi| {
                if (p.id != node.parent_id) continue;
                if (std.mem.eql(u8, p.node_type, "AlwaysFalse")) continue;
                if (child_key) |ck| {
                    if (min_tids[pi] == null or ck < min_tids[pi].?) {
                        min_tids[pi] = ck;
                        changed = true;
                    }
                }
                if (child_tp > 0 and teleport_dungeons[pi] == 0) {
                    teleport_dungeons[pi] = child_tp;
                    changed = true;
                }
                if (child_dung > 0 and subtree_dungeons[pi] == 0) {
                    subtree_dungeons[pi] = child_dung;
                    changed = true;
                }
            }
        }
        if (!changed) break;
    }

    // Collect Sequence / Select parent IDs first so in-place permutations don't disturb iteration.
    var seq_ids: std.ArrayListUnmanaged(i32) = .empty;
    var seq_is_sequence: std.ArrayListUnmanaged(bool) = .empty;
    for (nodes) |p| {
        const is_seq = std.mem.eql(u8, p.node_type, "Sequence");
        const is_sel = std.mem.eql(u8, p.node_type, "Select");
        if (!is_seq and !is_sel) continue;
        var seen = false;
        for (seq_ids.items) |sid| {
            if (sid == p.id) {
                seen = true;
                break;
            }
        }
        if (!seen) {
            try seq_ids.append(arena, p.id);
            try seq_is_sequence.append(arena, is_seq);
        }
    }

    const ChildSlot = struct {
        entry: NodeEntry,
        tid_key: ?u64,
        tp_dungeon: i32,
        sub_dungeon: i32,
        orig_pos: usize,
        paired: bool = false,
    };
    const TidSlot = struct {
        tid_key: u64,
        tp_dungeon: i32,
        sub_dungeon: i32,
        orig_pos: usize,
        entry: NodeEntry,
    };

    for (seq_ids.items, seq_is_sequence.items) |parent_id, is_sequence| {
        var positions: std.ArrayListUnmanaged(usize) = .empty;
        var non_term: std.ArrayListUnmanaged(ChildSlot) = .empty;
        var term: std.ArrayListUnmanaged(ChildSlot) = .empty;

        for (nodes, 0..) |node, i| {
            if (node.parent_id != parent_id) continue;
            try positions.append(arena, i);
            const slot: ChildSlot = .{
                .entry = node,
                .tid_key = min_tids[i],
                .tp_dungeon = teleport_dungeons[i],
                .sub_dungeon = subtree_dungeons[i],
                .orig_pos = i,
            };
            if (is_sequence and isTerminalQuestNode(node.node_type)) {
                try term.append(arena, slot);
            } else {
                try non_term.append(arena, slot);
            }
        }
        if (positions.items.len <= 1) continue;

        var tagged_indices: std.ArrayListUnmanaged(usize) = .empty;
        var tagged_items: std.ArrayListUnmanaged(TidSlot) = .empty;
        for (non_term.items, 0..) |c, idx| {
            if (c.tid_key) |tk| {
                try tagged_indices.append(arena, idx);
                try tagged_items.append(arena, .{
                    .tid_key = tk,
                    .tp_dungeon = c.tp_dungeon,
                    .sub_dungeon = c.sub_dungeon,
                    .orig_pos = c.orig_pos,
                    .entry = c.entry,
                });
            }
        }
        if (tagged_items.items.len > 1) {
            std.mem.sort(TidSlot, tagged_items.items, {}, struct {
                fn lessThan(_: void, a: TidSlot, b: TidSlot) bool {
                    if (a.tid_key != b.tid_key) return a.tid_key < b.tid_key;
                    return a.orig_pos < b.orig_pos;
                }
            }.lessThan);
            for (tagged_indices.items, tagged_items.items) |slot_idx, sorted_item| {
                non_term.items[slot_idx] = .{
                    .entry = sorted_item.entry,
                    .tid_key = sorted_item.tid_key,
                    .tp_dungeon = sorted_item.tp_dungeon,
                    .sub_dungeon = sorted_item.sub_dungeon,
                    .orig_pos = sorted_item.orig_pos,
                };
            }
        }

        if (is_sequence and non_term.items.len > 1) {
            var k: usize = 0;
            while (k < non_term.items.len) : (k += 1) {
                const tp_did = non_term.items[k].tp_dungeon;
                if (tp_did <= 0) continue;
                var found_idx: ?usize = null;
                for (non_term.items, 0..) |cand, m| {
                    if (m != k and !cand.paired and cand.sub_dungeon == tp_did) {
                        found_idx = m;
                        break;
                    }
                }
                if (found_idx) |m| {
                    var target = non_term.orderedRemove(m);
                    target.paired = true;
                    const insert_idx = if (m < k) k else k + 1;
                    try non_term.insert(arena, insert_idx, target);
                    k = insert_idx;
                }
            }
        }

        var write_i: usize = 0;
        for (non_term.items) |c| {
            const dst = positions.items[write_i];
            nodes[dst] = c.entry;
            min_tids[dst] = c.tid_key;
            teleport_dungeons[dst] = c.tp_dungeon;
            subtree_dungeons[dst] = c.sub_dungeon;
            write_i += 1;
        }
        for (term.items) |c| {
            const dst = positions.items[write_i];
            nodes[dst] = c.entry;
            min_tids[dst] = c.tid_key;
            teleport_dungeons[dst] = c.tp_dungeon;
            subtree_dungeons[dst] = c.sub_dungeon;
            write_i += 1;
        }
    }
}

fn parseOwnerId(header: []const u8) ?i32 {
    const key_marker = "\"Key\"";
    const key_pos = std.mem.indexOf(u8, header, key_marker) orelse return null;
    const colon_rel = std.mem.indexOfScalar(u8, header[key_pos + key_marker.len ..], ':') orelse return null;
    const after_colon = header[key_pos + key_marker.len + colon_rel + 1 ..];
    const open_quote = std.mem.indexOfScalar(u8, after_colon, '"') orelse return null;
    const str_start = after_colon[open_quote + 1 ..];
    const underscore = std.mem.indexOfScalar(u8, str_start, '_') orelse return null;
    return std.fmt.parseInt(i32, str_start[0..underscore], 10) catch null;
}

test "normalizeSequenceChildOrder pairs TeleportDungeon with sibling DungeonId sequence" {
    var arena = ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const alloc = arena.allocator();

    const raw_nodes = [_][]const u8{
        "{\"Type\":\"Sequence\",\"Id\":2,\"ParentNodeId\":1}",
        "{\"Type\":\"Sequence\",\"Id\":13,\"ParentNodeId\":2}",
        "{\"Type\":\"ChildQuest\",\"Id\":20,\"ParentNodeId\":13,\"TidTip\":\"Quest_145000005_ChildQuestTip_145_10\"}",
        "{\"Type\":\"Sequence\",\"Id\":24,\"ParentNodeId\":2,\"DungeonId\":904}",
        "{\"Type\":\"ChildQuest\",\"Id\":36,\"ParentNodeId\":24,\"TidTip\":\"Quest_145000005_ChildQuestTip_145_32\"}",
        "{\"Type\":\"Sequence\",\"Id\":26,\"ParentNodeId\":2}",
        "{\"Type\":\"ChildQuest\",\"Id\":27,\"ParentNodeId\":26,\"TidTip\":\"Quest_145000005_ChildQuestTip_145_23\"}",
        "{\"Type\":\"ParallelSelect\",\"Id\":190,\"ParentNodeId\":2}",
        "{\"Type\":\"ChildQuest\",\"Id\":405,\"ParentNodeId\":190,\"TidTip\":\"Quest_145000005_ChildQuestTip_145_20\",\"Condition\":{\"Type\":\"DoInteract\",\"AddOptions\":[{\"EntityId\":145750144,\"Option\":{\"Type\":{\"Type\":\"Actions\",\"Actions\":[{\"Name\":\"TeleportDungeon\",\"Params\":{\"DungeonId\":904}}]}}}]}}",
    };

    var nodes: std.ArrayListUnmanaged(NodeEntry) = .empty;
    for (raw_nodes) |raw| {
        const val = try std.json.parseFromSliceLeaky(std.json.Value, alloc, raw, .{ .allocate = .alloc_always });
        try appendNodeFlattened(alloc, &nodes, val, null);
    }

    try normalizeSequenceChildOrder(alloc, 145000005, nodes.items);

    var child_order: [4]i32 = undefined;
    var ci: usize = 0;
    for (nodes.items) |n| {
        if (n.parent_id == 2) {
            child_order[ci] = n.id;
            ci += 1;
        }
    }
    try std.testing.expectEqual(@as(usize, 4), ci);
    try std.testing.expectEqualSlices(i32, &.{ 13, 190, 24, 26 }, &child_order);
}
