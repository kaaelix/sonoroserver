const std = @import("std");
const pb = @import("proto").pb;
const Transaction = @import("../handlers.zig").Transaction;
const mem = @import("../../mem.zig");
const Assets = @import("../../data/Assets.zig");
const PlayerProgressComponent = @import("../../logic/component/player/PlayerProgressComponent.zig");
const qol = @import("qol.zig");

pub fn onHeartbeatRequest(txn: *Transaction(pb.HeartbeatRequest)) !void {
    txn.respond(.{});
}

pub fn onAllMsgRequest(
    txn: *Transaction(pb.AllMsgRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
    progress_comp: *const PlayerProgressComponent,
    io: std.Io,
) !void {
    const infos = try qol.buildShortMessageInfos(alloc.arena, assets, progress_comp, io);
    const charm_saved = progress_comp.info.getKeyInt("sm:charm");
    const theme_saved = progress_comp.info.getKeyInt("sm:theme");
    const bg_saved = progress_comp.info.getKeyInt("sm:bg");
    const bubble_saved = progress_comp.info.getKeyInt("sm:bubble");

    const charm: i32 = if (charm_saved > 0) @intCast(charm_saved) else 80892001;
    const theme: i32 = if (theme_saved > 0) @intCast(theme_saved) else 80891001;
    const bg: i32 = if (bg_saved > 0) @intCast(bg_saved) else 80820001;
    const bubble: i32 = if (bubble_saved > 0) @intCast(bubble_saved) else 80810001;

    var bg_ids: std.ArrayList(i32) = .empty;
    try bg_ids.appendSlice(alloc.arena, &.{ 80820001, 80820002, 80820004 });

    var theme_ids: std.ArrayList(i32) = .empty;
    try theme_ids.appendSlice(alloc.arena, &.{ 80891001, 80891002 });

    var bubble_ids: std.ArrayList(i32) = .empty;
    try bubble_ids.appendSlice(alloc.arena, &.{ 80810001, 80810002, 80810003 });

    var charm_ids: std.ArrayList(i32) = .empty;
    try charm_ids.appendSlice(alloc.arena, &.{ 80892001, 80892002 });

    txn.respond(.{
        .ShortMessageInfos = infos,
        .ChatCharmId = charm,
        .ChatCharmIds = charm_ids,
        .ChatThemeId = theme,
        .ChatThemeIds = theme_ids,
        .ChatBgId = bg,
        .ChatBgIds = bg_ids,
        .BubbleId = bubble,
        .BubbleIds = bubble_ids,
        .ErrCode = .Success,
    });
}
