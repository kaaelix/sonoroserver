//! Save blocks of a behaviour tree.
//!
//! A node that carries `SaveConfig` in the node tables is the unit of persistence of its tree:
//! progress made inside it does not survive a login. On login the tree resumes at the block and
//! the block is entered again from its start, so its own `SaveConfig` actions and the enter
//! actions of its children run exactly as on first entry. Nodes outside any block resume as saved.
//!
//! Everything here is pure and works on any node type exposing `id`, `parent_id` and `data`.

const std = @import("std");

const max_depth = 64;

fn findNode(nodes: anytype, node_id: i32) ?@TypeOf(&nodes[0]) {
    for (nodes) |*n| {
        if (n.id == node_id) return n;
    }
    return null;
}

fn saveConfigOf(node: anytype) ?std.json.ObjectMap {
    if (node.data != .object) return null;
    const cfg = node.data.object.get("SaveConfig") orelse return null;
    return if (cfg == .object) cfg.object else null;
}

/// Innermost node carrying `SaveConfig` among `node_id` and its ancestors.
pub fn findEnclosingSaveNode(nodes: anytype, node_id: i32) ?i32 {
    var cur = node_id;
    var depth: usize = 0;
    while (cur > 0 and depth < max_depth) : (depth += 1) {
        const node = findNode(nodes, cur) orelse return null;
        if (saveConfigOf(node.*) != null) return cur;
        cur = node.parent_id;
    }
    return null;
}

/// Whether `node_id` is `block_id` itself or one of its descendants.
pub fn isInBlock(nodes: anytype, node_id: i32, block_id: i32) bool {
    var cur = node_id;
    var depth: usize = 0;
    while (cur > 0 and depth < max_depth) : (depth += 1) {
        if (cur == block_id) return true;
        const node = findNode(nodes, cur) orelse return false;
        cur = node.parent_id;
    }
    return false;
}

pub const PosRollback = struct {
    range_entity_id: i32,
    position_entity_id: i32,
};

/// `SaveConfig.PosRollbackCfg` of a save node, when it names both entities.
pub fn posRollbackOf(nodes: anytype, block_id: i32) ?PosRollback {
    const node = findNode(nodes, block_id) orelse return null;
    const cfg = saveConfigOf(node.*) orelse return null;
    const pos_cfg = cfg.get("PosRollbackCfg") orelse return null;
    if (pos_cfg != .object) return null;
    const range_id = intOf(pos_cfg.object.get("RangeEntityId")) orelse return null;
    const position_id = intOf(pos_cfg.object.get("PositionEntityId")) orelse return null;
    if (position_id <= 0) return null;
    return .{ .range_entity_id = range_id, .position_entity_id = position_id };
}

fn intOf(value: ?std.json.Value) ?i32 {
    const v = value orelse return null;
    return switch (v) {
        .integer => |i| std.math.cast(i32, i),
        else => null,
    };
}

/// What a login discards from one tree.
pub const Discard = struct {
    /// Save nodes the tree resumes at, innermost per active node, without duplicates.
    blocks: []const i32,
    /// Active nodes that must be re-entered instead of restored.
    reenter: []const i32,
    /// Finished nodes inside the blocks whose completion is forgotten.
    unfinish: []const i32,

    pub const none: Discard = .{ .blocks = &.{}, .reenter = &.{}, .unfinish = &.{} };

    pub fn isEmpty(discard: Discard) bool {
        return discard.blocks.len == 0;
    }
};

/// Computes what a login discards for a tree whose saved state is `active_nodes` and
/// `finished_nodes`. Result slices are allocated from `arena`.
pub fn planLoginDiscard(
    arena: std.mem.Allocator,
    nodes: anytype,
    active_nodes: []const i32,
    finished_nodes: []const i32,
) !Discard {
    var blocks: std.ArrayList(i32) = .empty;
    var reenter: std.ArrayList(i32) = .empty;
    for (active_nodes) |node_id| {
        const block_id = findEnclosingSaveNode(nodes, node_id) orelse continue;
        try reenter.append(arena, node_id);
        if (std.mem.indexOfScalar(i32, blocks.items, block_id) == null) {
            try blocks.append(arena, block_id);
        }
    }
    if (blocks.items.len == 0) return .none;

    var unfinish: std.ArrayList(i32) = .empty;
    for (finished_nodes) |node_id| {
        for (blocks.items) |block_id| {
            if (node_id != block_id and isInBlock(nodes, node_id, block_id)) {
                try unfinish.append(arena, node_id);
                break;
            }
        }
    }
    return .{ .blocks = blocks.items, .reenter = reenter.items, .unfinish = unfinish.items };
}

const TestNode = struct {
    id: i32,
    parent_id: i32,
    data: std.json.Value,
};

/// Shape of quest 139000025 around its first combat block, as in the node table:
/// root Sequence 1 -> ParallelSelect 111 (SaveConfig) -> Kill 99, AlwaysFalse 147 -> ReachArea 148;
/// and ParallelSelect 136 (no SaveConfig) -> ReachArea 8, AlwaysFalse 188 -> Sequence 189 -> ReachArea 190.
fn testTree(arena: std.mem.Allocator) ![]const TestNode {
    const plain = try std.json.parseFromSliceLeaky(std.json.Value, arena, "{}", .{});
    const block = try std.json.parseFromSliceLeaky(std.json.Value, arena,
        \\{"SaveConfig":{"DisableRollbackWhileReconnect":true,"PosRollbackCfg":{"RangeEntityId":12000000,"PositionEntityId":12000001}}}
    , .{});
    return try arena.dupe(TestNode, &.{
        .{ .id = 1, .parent_id = 0, .data = plain },
        .{ .id = 111, .parent_id = 1, .data = block },
        .{ .id = 99, .parent_id = 111, .data = plain },
        .{ .id = 147, .parent_id = 111, .data = plain },
        .{ .id = 148, .parent_id = 147, .data = plain },
        .{ .id = 136, .parent_id = 1, .data = plain },
        .{ .id = 8, .parent_id = 136, .data = plain },
        .{ .id = 188, .parent_id = 136, .data = plain },
        .{ .id = 189, .parent_id = 188, .data = plain },
        .{ .id = 190, .parent_id = 189, .data = plain },
    });
}

test "a node resolves to the innermost save node above it, and to none outside a block" {
    var arena_state = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena_state.deinit();
    const nodes = try testTree(arena_state.allocator());

    try std.testing.expectEqual(@as(?i32, 111), findEnclosingSaveNode(nodes, 148));
    try std.testing.expectEqual(@as(?i32, 111), findEnclosingSaveNode(nodes, 111));
    try std.testing.expectEqual(@as(?i32, null), findEnclosingSaveNode(nodes, 190));
    try std.testing.expectEqual(@as(?i32, null), findEnclosingSaveNode(nodes, 1));
}

test "login forgets a node finished inside a save block and re-enters the active ones" {
    var arena_state = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena_state.deinit();
    const arena = arena_state.allocator();
    const nodes = try testTree(arena);

    // Saved state: 148 was completed inside block 111 while 99 was still in progress.
    const discard = try planLoginDiscard(arena, nodes, &.{99}, &.{ 136, 8, 148 });

    try std.testing.expectEqualSlices(i32, &.{111}, discard.blocks);
    try std.testing.expectEqualSlices(i32, &.{99}, discard.reenter);
    try std.testing.expectEqualSlices(i32, &.{148}, discard.unfinish);
}

test "login keeps progress of nodes that are outside any save block" {
    var arena_state = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena_state.deinit();
    const arena = arena_state.allocator();
    const nodes = try testTree(arena);

    const discard = try planLoginDiscard(arena, nodes, &.{ 8, 190 }, &.{ 111, 99, 148 });

    try std.testing.expect(discard.isEmpty());
    try std.testing.expectEqual(@as(usize, 0), discard.reenter.len);
    try std.testing.expectEqual(@as(usize, 0), discard.unfinish.len);
}

test "position rollback is read from the save node configuration" {
    var arena_state = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena_state.deinit();
    const nodes = try testTree(arena_state.allocator());

    const rollback = posRollbackOf(nodes, 111) orelse return error.TestExpectedRollback;
    try std.testing.expectEqual(@as(i32, 12000000), rollback.range_entity_id);
    try std.testing.expectEqual(@as(i32, 12000001), rollback.position_entity_id);
    try std.testing.expectEqual(@as(?PosRollback, null), posRollbackOf(nodes, 136));
}
