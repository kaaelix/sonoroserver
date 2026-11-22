const std = @import("std");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const FileSystem = @import("common").FileSystem;
const Assets = @import("../../data/Assets.zig");
const Connection = @import("../../network/Connection.zig");
const EventQueue = @import("../EventQueue.zig");
const Scene = @import("../Scene.zig");
const PlayerBasicComponent = @import("../component/player/PlayerBasicComponent.zig");
const spawn_cmd = @import("../commands/entity.zig").spawn;

pub const prologue_quest_id: i32 = 139000025;
pub const act2_quest_id: i32 = 139000026;
pub const prologue_flow_name: []const u8 = "剧情_新主线_CE版_序章";

pub fn transitionNode(
    conn: *Connection,
    scene: *Scene,
    alloc: mem.Alloc,
    io: std.Io,
    from_node: i32,
    to_node: i32,
) void {
    scene.current_quest_node = to_node;
    conn.push(alloc.gpa, io, pb.UpdateNodeStatusNotify{
        .TreeOwnerId = prologue_quest_id,
        .TreeIncId = 1,
        .NodeId = from_node,
        .Status = .CompletedSuccess,
    }) catch {};
    conn.push(alloc.gpa, io, pb.UpdateChildQuestNodeStatusNotify{
        .TreeOwnerId = prologue_quest_id,
        .TreeIncId = 1,
        .NodeId = from_node,
        .Status = .Finished,
    }) catch {};

    if (to_node != 0) {
        conn.push(alloc.gpa, io, pb.UpdateNodeStatusNotify{
            .TreeOwnerId = prologue_quest_id,
            .TreeIncId = 1,
            .NodeId = to_node,
            .Status = .Activate,
        }) catch {};
        conn.push(alloc.gpa, io, pb.UpdateChildQuestNodeStatusNotify{
            .TreeOwnerId = prologue_quest_id,
            .TreeIncId = 1,
            .NodeId = to_node,
            .Status = .Progress,
        }) catch {};
    }
}

pub fn startFlow(
    conn: *Connection,
    alloc: mem.Alloc,
    io: std.Io,
    state_id: i32,
) void {
    conn.push(alloc.gpa, io, pb.FlowStartNotify{
        .FlowIncId = state_id,
        .FlowListName = prologue_flow_name,
        .FlowId = 1,
        .StateId = state_id,
    }) catch {};
}

pub fn spawnQuestEntity(
    events: *EventQueue,
    scene: *Scene,
    fs: *FileSystem,
    assets: *const Assets,
    conn: *Connection,
    basic_comp: *const PlayerBasicComponent,
    alloc: mem.Alloc,
    io: std.Io,
    entity_id: i64,
) void {
    if (scene.debugSpawnRoute(entity_id) != null) return;

    const cfg = assets.findLevelEntity(scene.instance_id, entity_id) orelse return;
    const pos = Assets.AssetIndexes.entityWorldPos(cfg) orelse return;

    spawn_cmd.spawnWithLocation(
        events,
        scene,
        fs,
        assets,
        conn,
        basic_comp,
        alloc,
        io,
        entity_id,
        false,
        false,
        cfg.MapId,
        pos,
        true,
    ) catch {};
}

pub fn finishPrologueQuest(
    conn: *Connection,
    scene: *Scene,
    alloc: mem.Alloc,
    io: std.Io,
) void {
    transitionNode(conn, scene, alloc, io, 159, 0);

    var finish_list: std.ArrayList(i32) = .empty;
    finish_list.append(alloc.arena, prologue_quest_id) catch {};
    conn.push(alloc.gpa, io, pb.QuestFinishListNotify{ .QuestId = finish_list }) catch {};

    var act2_list: std.ArrayList(pb.QuestInfo) = .empty;
    act2_list.append(alloc.arena, .{ .QuestId = act2_quest_id, .Status = .InProgress }) catch {};
    conn.push(alloc.gpa, io, pb.QuestListNotify{ .Quests = act2_list }) catch {};

    conn.push(alloc.gpa, io, pb.QuestShowListNotify{}) catch {};

    conn.push(alloc.gpa, io, pb.TraceQuestNotify{ .QuestId = act2_quest_id }) catch {};
    conn.push(alloc.gpa, io, pb.FocusQuestSetNotify{ .FocusQuestId = act2_quest_id, .Reason = .Inherit }) catch {};

    var open_funcs: std.ArrayList(pb.Function) = .empty;
    open_funcs.append(alloc.arena, .{ .Id = 10003, .Flag = 2 }) catch {};
    open_funcs.append(alloc.arena, .{ .Id = 10023, .Flag = 2 }) catch {};
    conn.push(alloc.gpa, io, pb.FuncOpenNotify{ .Func = open_funcs }) catch {};

    conn.push(alloc.gpa, io, pb.TeleportNotify{
        .MapId = scene.instance_id,
        .Pos = .{ .X = -105396.0, .Y = 75049.0, .Z = 6715.0 },
    }) catch {};
}
