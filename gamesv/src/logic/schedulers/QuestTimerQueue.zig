const QuestTimerQueue = @This();
const std = @import("std");

const Allocator = std.mem.Allocator;

/// Scheduled completion deadline for a `Timer` or `WaitTime` child quest node.
pub const Entry = struct {
    quest_id: i32,
    node_id: i32,
    due_ms: i64,
};

heap: std.ArrayListUnmanaged(Entry) = .empty,
positions: std.AutoHashMapUnmanaged(u64, usize) = .empty,

pub fn deinit(queue: *QuestTimerQueue, gpa: Allocator) void {
    queue.heap.deinit(gpa);
    queue.positions.deinit(gpa);
}

pub fn reset(queue: *QuestTimerQueue, gpa: Allocator) void {
    queue.deinit(gpa);
    queue.* = .{};
}

/// Inserts or updates a quest node timer deadline.
pub fn upsert(queue: *QuestTimerQueue, gpa: Allocator, entry: Entry) !void {
    const entry_key = key(entry.quest_id, entry.node_id);
    if (queue.positions.get(entry_key)) |index| {
        const previous_due = queue.heap.items[index].due_ms;
        queue.heap.items[index] = entry;
        if (entry.due_ms < previous_due) {
            queue.siftUp(index);
        } else if (entry.due_ms > previous_due) {
            queue.siftDown(index);
        }
        return;
    }

    const index = queue.heap.items.len;
    try queue.heap.append(gpa, entry);
    errdefer _ = queue.heap.pop();
    try queue.positions.put(gpa, entry_key, index);
    queue.siftUp(index);
}

pub fn contains(queue: *const QuestTimerQueue, quest_id: i32, node_id: i32) bool {
    return queue.positions.contains(key(quest_id, node_id));
}

pub fn getDueMs(queue: *const QuestTimerQueue, quest_id: i32, node_id: i32) ?i64 {
    const index = queue.positions.get(key(quest_id, node_id)) orelse return null;
    return queue.heap.items[index].due_ms;
}

pub fn cancel(queue: *QuestTimerQueue, quest_id: i32, node_id: i32) void {
    const index = queue.positions.get(key(quest_id, node_id)) orelse return;
    _ = queue.removeAt(index);
}

pub fn cancelQuest(queue: *QuestTimerQueue, quest_id: i32) void {
    var index: usize = 0;
    while (index < queue.heap.items.len) {
        if (queue.heap.items[index].quest_id == quest_id) {
            _ = queue.removeAt(index);
        } else {
            index += 1;
        }
    }
}

pub fn popDue(queue: *QuestTimerQueue, now_ms: i64) ?Entry {
    if (queue.heap.items.len == 0 or queue.heap.items[0].due_ms > now_ms) return null;
    return queue.removeAt(0);
}

pub fn nextWakeDelayMs(queue: *const QuestTimerQueue, now_ms: i64) ?i64 {
    if (queue.heap.items.len == 0) return null;
    const due_ms = queue.heap.items[0].due_ms;
    if (due_ms <= now_ms) return 0;
    const delay: i128 = @as(i128, due_ms) - now_ms;
    return @intCast(@min(delay, std.math.maxInt(i64)));
}

fn removeAt(queue: *QuestTimerQueue, index: usize) Entry {
    const removed = queue.heap.items[index];
    _ = queue.positions.remove(key(removed.quest_id, removed.node_id));

    const last_index = queue.heap.items.len - 1;
    if (index == last_index) {
        _ = queue.heap.pop();
        return removed;
    }

    queue.heap.items[index] = queue.heap.items[last_index];
    _ = queue.heap.pop();
    queue.positions.getPtr(key(
        queue.heap.items[index].quest_id,
        queue.heap.items[index].node_id,
    )).?.* = index;

    if (index > 0 and lessThan(queue.heap.items[index], queue.heap.items[(index - 1) / 2])) {
        queue.siftUp(index);
    } else {
        queue.siftDown(index);
    }
    return removed;
}

fn siftUp(queue: *QuestTimerQueue, start_index: usize) void {
    var index = start_index;
    while (index > 0) {
        const parent = (index - 1) / 2;
        if (!lessThan(queue.heap.items[index], queue.heap.items[parent])) break;
        queue.swapEntries(index, parent);
        index = parent;
    }
}

fn siftDown(queue: *QuestTimerQueue, start_index: usize) void {
    var index = start_index;
    while (true) {
        const left = index * 2 + 1;
        if (left >= queue.heap.items.len) return;
        const right = left + 1;
        const child = if (right < queue.heap.items.len and
            lessThan(queue.heap.items[right], queue.heap.items[left])) right else left;
        if (!lessThan(queue.heap.items[child], queue.heap.items[index])) return;
        queue.swapEntries(index, child);
        index = child;
    }
}

fn swapEntries(queue: *QuestTimerQueue, a: usize, b: usize) void {
    std.mem.swap(Entry, &queue.heap.items[a], &queue.heap.items[b]);
    queue.positions.getPtr(key(
        queue.heap.items[a].quest_id,
        queue.heap.items[a].node_id,
    )).?.* = a;
    queue.positions.getPtr(key(
        queue.heap.items[b].quest_id,
        queue.heap.items[b].node_id,
    )).?.* = b;
}

fn lessThan(a: Entry, b: Entry) bool {
    if (a.due_ms != b.due_ms) return a.due_ms < b.due_ms;
    if (a.quest_id != b.quest_id) return a.quest_id < b.quest_id;
    return a.node_id < b.node_id;
}

fn key(quest_id: i32, node_id: i32) u64 {
    return (@as(u64, @as(u32, @bitCast(quest_id))) << 32) |
        @as(u64, @as(u32, @bitCast(node_id)));
}

test "QuestTimerQueue orders deadlines, updates in place, and cancels" {
    var queue: QuestTimerQueue = .{};
    defer queue.deinit(std.testing.allocator);

    try queue.upsert(std.testing.allocator, .{ .quest_id = 139000025, .node_id = 107, .due_ms = 2_000 });
    try queue.upsert(std.testing.allocator, .{ .quest_id = 139000025, .node_id = 116, .due_ms = 1_000 });
    try std.testing.expectEqual(@as(?i64, 500), queue.nextWakeDelayMs(500));
    try std.testing.expect(queue.popDue(999) == null);

    // Reschedule node 116 later than node 107.
    try queue.upsert(std.testing.allocator, .{ .quest_id = 139000025, .node_id = 116, .due_ms = 3_000 });
    try std.testing.expectEqual(@as(i64, 2_000), queue.getDueMs(139000025, 107).?);

    const first = queue.popDue(2_000).?;
    try std.testing.expectEqual(@as(i32, 107), first.node_id);
    try std.testing.expect(queue.popDue(2_500) == null);

    queue.cancelQuest(139000025);
    try std.testing.expect(queue.nextWakeDelayMs(5_000) == null);
}
