const std = @import("std");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const Scene = @import("../../logic/Scene.zig");
const common_pkg = @import("common");
const FileSystem = common_pkg.FileSystem;
const Assets = @import("../../data/Assets.zig");
const Connection = @import("../Connection.zig");
const EventQueue = @import("../../logic/EventQueue.zig");
const PlayerBasicComponent = @import("../../logic/component/player/PlayerBasicComponent.zig");
const PlayerInventoryComponent = @import("../../logic/component/player/PlayerInventoryComponent.zig");
const PlayerWeaponComponent = @import("../../logic/component/player/PlayerWeaponComponent.zig");
const PlayerEchoComponent = @import("../../logic/component/player/PlayerEchoComponent.zig");
const PlayerMapComponent = @import("../../logic/component/player/PlayerMapComponent.zig");
const PlayerQuestComponent = @import("../../logic/component/player/PlayerQuestComponent.zig");
const QuestEngine = @import("../../logic/helpers/quest_engine.zig");
const dispatch = @import("combat.zig");

pub fn SkillRequest(txn: *dispatch.CombatRequestTxn(.SkillRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn UseSkillRequest(
    txn: *dispatch.CombatRequestTxn(.UseSkillRequest),
    scene: *Scene,
    qctx: QuestEngine.QuestContext,
    io: std.Io,
    alloc: mem.Alloc,
) !void {
    const request: pb.UseSkillRequest = txn.payload;
    if (commonEntityId(txn.common)) |entity_id| {
        if (skillId(request.UseSkillInfo)) |skill_id| {
            try scene.noteFsmSkillStart(alloc.gpa, entity_id, skill_id, queryNow(io));
            QuestEngine.onSkillUse(qctx, skill_id) catch |err| QuestEngine.logHookErr("onSkillUse", err);
        }
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .UseSkillInfo = request.UseSkillInfo,
        .SkillSingleId = request.SkillSingleId,
    });
}

pub fn EndSkillRequest(
    txn: *dispatch.CombatRequestTxn(.EndSkillRequest),
    scene: *Scene,
    io: std.Io,
    alloc: mem.Alloc,
) !void {
    const request: pb.EndSkillRequest = txn.payload;
    if (commonEntityId(txn.common)) |entity_id| {
        if (skillId(request.UseSkillInfo)) |skill_id| {
            try scene.signalFsmSkillEnd(alloc.gpa, entity_id, skill_id, queryNow(io));
        }
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .UseSkillInfo = request.UseSkillInfo,
        .SkillSingleId = request.SkillSingleId,
    });
}

pub fn EndSkillPush(
    push: pb.EndSkillPush,
    common: ?pb.CombatCommon,
    scene: *Scene,
    io: std.Io,
    alloc: mem.Alloc,
) !void {
    const entity_id = commonEntityId(common) orelse return;
    const skill_id = skillId(push.UseSkillInfo) orelse return;
    try scene.signalFsmSkillEnd(alloc.gpa, entity_id, skill_id, queryNow(io));
}

pub fn CreateBulletResponsePush(_: pb.CreateBulletResponsePush) !void {}

pub fn DestroyBulletResponsePush(_: pb.DestroyBulletResponsePush) !void {}

fn skillId(info: ?pb.UseSkillInformation) ?i32 {
    const value = info orelse return null;
    if (value.SkillId <= 0) return null;
    return std.math.cast(i32, value.SkillId);
}

fn commonEntityId(common: ?pb.CombatCommon) ?i64 {
    const value = common orelse return null;
    if (value.EntityId == 0) return null;
    return value.EntityId;
}

fn queryNow(io: std.Io) i64 {
    const rtc: std.Io.Clock = .awake;
    return rtc.now(io).toMilliseconds();
}
