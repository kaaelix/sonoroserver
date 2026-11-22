const std = @import("std");
const pb = @import("proto").pb;
const FileSystem = @import("common").FileSystem;
const Assets = @import("../../data/Assets.zig");
const EventQueue = @import("../EventQueue.zig");
const Connection = @import("../../network/Connection.zig");
const Scene = @import("../Scene.zig");
const PlayerBasicComponent = @import("../component/player/PlayerBasicComponent.zig");
const PlayerInventoryComponent = @import("../component/player/PlayerInventoryComponent.zig");
const PlayerWeaponComponent = @import("../component/player/PlayerWeaponComponent.zig");
const PlayerEchoComponent = @import("../component/player/PlayerEchoComponent.zig");
const PlayerMapComponent = @import("../component/player/PlayerMapComponent.zig");
const PlayerQuestComponent = @import("../component/player/PlayerQuestComponent.zig");
const QuestEngine = @import("../helpers/quest_engine.zig");
const mem = @import("../../mem.zig");

pub fn runEnterGamePhase(
    _: EventQueue.Dequeue(.enter_game),
    events: *EventQueue,
) !void {
    // events will be consumed sequentially, extend as needed
    try events.enqueue(.push_data, .{});
    try events.enqueue(.push_data_complete, .{});
    try events.enqueue(.initial_scene_join, .{});
}

pub fn notifyPushDataComplete(
    _: EventQueue.Dequeue(.push_data_complete),
    conn: *Connection,
    qctx: QuestEngine.QuestContext,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    // Push quests after all push_data handlers (including FuncOpenNotify) so quest UI systems are unlocked.
    try QuestEngine.syncOnLogin(qctx);
    try conn.push(alloc.gpa, io, pb.PushContextIdNotify{ .Id = 0 });
    try conn.push(alloc.gpa, io, pb.PushDataCompleteNotify{});
}

pub fn onQuestTimersDue(
    event: EventQueue.Dequeue(.quest_timers_due),
    qctx: QuestEngine.QuestContext,
) !void {
    try QuestEngine.onTimersDue(qctx, event.data.now_ms);
}
