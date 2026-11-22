const std = @import("std");
const pb = @import("proto").pb;
const Transaction = @import("../handlers.zig").Transaction;
const FileSystem = @import("common").FileSystem;
const Connection = @import("../../network/Connection.zig");
const Scene = @import("../../logic/Scene.zig");
const EventQueue = @import("../../logic/EventQueue.zig");
const Assets = @import("../../data/Assets.zig");
const mem = @import("../../mem.zig");
const PlayerBasicComponent = @import("../../logic/component/player/PlayerBasicComponent.zig");
const PlayerInventoryComponent = @import("../../logic/component/player/PlayerInventoryComponent.zig");
const PlayerWeaponComponent = @import("../../logic/component/player/PlayerWeaponComponent.zig");
const PlayerEchoComponent = @import("../../logic/component/player/PlayerEchoComponent.zig");
const PlayerMapComponent = @import("../../logic/component/player/PlayerMapComponent.zig");
const PlayerQuestComponent = @import("../../logic/component/player/PlayerQuestComponent.zig");
const PlayerRoleComponent = @import("../../logic/component/player/PlayerRoleComponent.zig");
const PlayerSceneComponent = @import("../../logic/component/player/PlayerSceneComponent.zig");
const PlayerGuideComponent = @import("../../logic/component/player/PlayerGuideComponent.zig");
const PlayerMailComponent = @import("../../logic/component/player/PlayerMailComponent.zig");
const PlayerProgressComponent = @import("../../logic/component/player/PlayerProgressComponent.zig");
const QuestEngine = @import("../../logic/helpers/quest_engine.zig");
const Io = std.Io;

fn buildQuestContext(
    assets: *const Assets,
    fs: *FileSystem,
    conn: *Connection,
    scene: *Scene,
    scene_comp: *PlayerSceneComponent,
    events: *EventQueue,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    map_comp: *PlayerMapComponent,
    quest_comp: *PlayerQuestComponent,
    guide_comp: *PlayerGuideComponent,
    mail_comp: *PlayerMailComponent,
    progress_comp: *PlayerProgressComponent,
    alloc: mem.Alloc,
    io: std.Io,
) QuestEngine.QuestContext {
    return .{
        .assets = assets,
        .fs = fs,
        .conn = conn,
        .scene = scene,
        .scene_comp = scene_comp,
        .events = events,
        .alloc = alloc,
        .io = io,
        .basic_comp = basic_comp,
        .inventory_comp = inventory_comp,
        .role_comp = role_comp,
        .weapon_comp = weapon_comp,
        .echo_comp = echo_comp,
        .map_comp = map_comp,
        .quest_comp = quest_comp,
        .guide_comp = guide_comp,
        .mail_comp = mail_comp,
        .progress_comp = progress_comp,
    };
}

pub fn onFlowEndRequest(
    txn: *Transaction(pb.FlowEndRequest),
    assets: *const Assets,
    fs: *FileSystem,
    conn: *Connection,
    scene: *Scene,
    scene_comp: *PlayerSceneComponent,
    events: *EventQueue,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    map_comp: *PlayerMapComponent,
    quest_comp: *PlayerQuestComponent,
    guide_comp: *PlayerGuideComponent,
    mail_comp: *PlayerMailComponent,
    progress_comp: *PlayerProgressComponent,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    const had_active_flow = scene.active_flow_inc_id != 0;
    const prev_flow_list = if (had_active_flow) try alloc.arena.dupe(u8, scene.active_flow_list_name) else "";
    const prev_flow_id = scene.active_flow_id;
    const prev_state_id = scene.active_flow_state_id;
    const prev_instance_id = scene_comp.last_scene_info.instance_id;

    try QuestEngine.onFlowEnd(buildQuestContext(
        assets,
        fs,
        conn,
        scene,
        scene_comp,
        events,
        basic_comp,
        inventory_comp,
        role_comp,
        weapon_comp,
        echo_comp,
        map_comp,
        quest_comp,
        guide_comp,
        mail_comp,
        progress_comp,
        alloc,
        io,
    ), txn.message.FlowIncId);

    const same_instance = scene_comp.last_scene_info.instance_id == prev_instance_id;

    // Only close the subtitle view if no follow-up/queued flow immediately started.
    if (scene.active_flow_inc_id == 0 and had_active_flow) {
        if (Io.Dir.readFileAlloc(Io.Dir.cwd(), fs.io, "assets/scripts/misc_patches/flow_ender.js", alloc.gpa, Io.Limit.unlimited)) |js| {
            defer alloc.gpa.free(js);
            conn.pushBestEffort(alloc.gpa, io, pb.JSPatchNotify{
                .Content = js,
            });
        } else |_| {}
    }

    if (shouldSendFlowSkipBlackScreen(
        had_active_flow,
        scene.active_flow_inc_id,
        same_instance,
        scene.client_black_screen_open,
        txn.message.IsSkip,
    )) {
        scene.client_black_screen_open = false;
        conn.pushBestEffort(alloc.gpa, io, pb.FlowServerSkipNotify{
            .FlowListName = prev_flow_list,
            .FlowId = prev_flow_id,
            .StateId = prev_state_id,
            .FadeOutScreen = true,
        });
    }

    txn.respond(.{ .ErrorCode = .Success });
}

fn shouldSendFlowSkipBlackScreen(
    had_active_flow: bool,
    active_flow_inc_id: i64,
    same_instance: bool,
    client_black_screen_open: bool,
    is_skip: bool,
) bool {
    return active_flow_inc_id == 0 and same_instance and (client_black_screen_open or (is_skip and had_active_flow));
}

pub fn onFlowActionRequest(
    txn: *Transaction(pb.FlowActionRequest),
    assets: *const Assets,
    fs: *FileSystem,
    conn: *Connection,
    scene: *Scene,
    scene_comp: *PlayerSceneComponent,
    events: *EventQueue,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    map_comp: *PlayerMapComponent,
    quest_comp: *PlayerQuestComponent,
    guide_comp: *PlayerGuideComponent,
    mail_comp: *PlayerMailComponent,
    progress_comp: *PlayerProgressComponent,
    alloc: mem.Alloc,
    io: std.Io,
) !void {
    try QuestEngine.onFlowAction(buildQuestContext(
        assets,
        fs,
        conn,
        scene,
        scene_comp,
        events,
        basic_comp,
        inventory_comp,
        role_comp,
        weapon_comp,
        echo_comp,
        map_comp,
        quest_comp,
        guide_comp,
        mail_comp,
        progress_comp,
        alloc,
        io,
    ), txn.message.FlowIncId, txn.message.ActionId);
    txn.respond(.{ .ErrorCode = .Success });
}

test "shouldSendFlowSkipBlackScreen notifies only when flow ends without follow-up flow or map switch" {
    const testing = std.testing;
    try testing.expect(shouldSendFlowSkipBlackScreen(true, 0, true, true, false));
    try testing.expect(shouldSendFlowSkipBlackScreen(true, 0, true, false, true));
    try testing.expect(shouldSendFlowSkipBlackScreen(false, 0, true, true, false));
    try testing.expect(!shouldSendFlowSkipBlackScreen(true, 42, true, true, true));
    try testing.expect(!shouldSendFlowSkipBlackScreen(true, 0, false, true, true));
    try testing.expect(!shouldSendFlowSkipBlackScreen(true, 0, true, false, false));
}
