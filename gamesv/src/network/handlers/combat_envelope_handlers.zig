const std = @import("std");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const dispatch = @import("combat.zig");
const Scene = @import("../../logic/Scene.zig");
const PlayerBasicComponent = @import("../../logic/component/player/PlayerBasicComponent.zig");
const QuestEngine = @import("../../logic/helpers/quest_engine.zig");
const Entity = Scene.Entity;

const TagQuery = Scene.Query(&.{
    Entity,
    ?*Entity.TagComponent,
});

fn appendNotify(
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
    arena: std.mem.Allocator,
    common: ?pb.CombatCommon,
    msg: @FieldType(pb.CombatNotifyData, "Message"),
) !void {
    var resolved_common = common orelse pb.CombatCommon{ .EntityId = 0 };
    if (resolved_common.MessageId == 0) resolved_common.MessageId = -1;
    if (resolved_common.PreMessageId == 0) resolved_common.PreMessageId = -1;
    try receive_data_pack.append(arena, .{
        .Message = .{
            .CombatNotifyData = .{
                .CombatCommon = resolved_common,
                .Message = msg,
            },
        },
    });
}

pub fn ExecuteQtePush(
    push: pb.ExecuteQtePush,
    common: ?pb.CombatCommon,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    try appendNotify(receive_data_pack, alloc.arena, common, .{
        .ExecuteQteNotify = .{
            .DownEntityId = push.DownEntityId,
            .UpEntityId = push.UpEntityId,
            .FnvHash = push.FnvHash,
        },
    });
}

pub fn ExecuteQteRequest(
    txn: *dispatch.CombatRequestTxn(.ExecuteQteRequest),
    alloc: mem.Alloc,
) !void {
    try appendNotify(txn.receive_data_pack, alloc.arena, txn.common, .{
        .ExecuteQteNotify = .{
            .DownEntityId = txn.payload.DownEntityId,
            .UpEntityId = txn.payload.UpEntityId,
            .FnvHash = txn.payload.FnvHash,
        },
    });
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn ChangeSummonEntityStateRequest(
    txn: *dispatch.CombatRequestTxn(.ChangeSummonEntityStateRequest),
    query: TagQuery,
    alloc: mem.Alloc,
) !void {
    if (query.byNetId(txn.payload.SummonEntityId)) |row| {
        if (row[1]) |tags| {
            try tags.setGameplayTagCount(alloc.gpa, txn.payload.TagId, 1);
            try tags.addCommonTag(alloc.gpa, txn.payload.TagId);
        }
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn MotorSummonAndRidePush(
    push: pb.MotorSummonAndRidePush,
    common: ?pb.CombatCommon,
    basic_comp: *const PlayerBasicComponent,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    try appendNotify(receive_data_pack, alloc.arena, common, .{
        .MotorSummonAndRideNotify = .{
            .PlayerId = basic_comp.player_id,
            .EntityId = push.EntityId,
            .VehicleIncId = push.VehicleIncId,
            .Transform = push.Transform,
        },
    });
}

pub fn MotorIsEnablePush(_: pb.MotorIsEnablePush) !void {}

pub fn VisionTriggerPush(_: pb.VisionTriggerPush) !void {}

pub fn GameplayCuePush(_: pb.GameplayCuePush) !void {}
pub fn GameplayCueRequest(txn: *dispatch.CombatRequestTxn(.GameplayCueRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn BeforeHitPush(_: pb.BeforeHitPush) !void {}

pub fn CaughtPush(_: pb.CaughtPush) !void {}
pub fn CaughtRequest(txn: *dispatch.CombatRequestTxn(.CaughtRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn MaterialPush(_: pb.MaterialPush) !void {}
pub fn MaterialRequest(txn: *dispatch.CombatRequestTxn(.MaterialRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn BoneVisibleChangePush(_: pb.BoneVisibleChangePush) !void {}
pub fn BoneVisibleChangeRequest(txn: *dispatch.CombatRequestTxn(.BoneVisibleChangeRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn ModifyBulletParamsPush(_: pb.ModifyBulletParamsPush) !void {}
pub fn ModifyBulletParamsRequest(txn: *dispatch.CombatRequestTxn(.ModifyBulletParamsRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn BulletPatternPush(_: pb.BulletPatternPush) !void {}

pub fn AnimationStateChangedPush(_: pb.AnimationStateChangedPush) !void {}
pub fn AnimationStateChangedRequest(txn: *dispatch.CombatRequestTxn(.AnimationStateChangedRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn AnimationStateInitPush(_: pb.AnimationStateInitPush) !void {}
pub fn AnimationStateInitRequest(txn: *dispatch.CombatRequestTxn(.AnimationStateInitRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn EntityStaticHookMovePush(_: pb.EntityStaticHookMovePush) !void {}
pub fn EntityStaticHookMoveRequest(txn: *dispatch.CombatRequestTxn(.EntityStaticHookMoveRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn AiBlackboardsPush(_: pb.AiBlackboardsPush) !void {}
pub fn AiBlackboardsRequest(txn: *dispatch.CombatRequestTxn(.AiBlackboardsRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn AiBlackboardCdPush(
    push: pb.AiBlackboardCdPush,
    common: ?pb.CombatCommon,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    try appendNotify(receive_data_pack, alloc.arena, common, .{
        .AiBlackboardCdNotify = .{
            .AiBlackboardCdModify = push.AiBlackboardCdModify,
            .AiBlackboardCdComplete = push.AiBlackboardCdComplete,
        },
    });
}

pub fn AiBlackboardCdRequest(txn: *dispatch.CombatRequestTxn(.AiBlackboardCdRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn ToughCalcExtraRatioChangePush(_: pb.ToughCalcExtraRatioChangePush) !void {}
pub fn ToughCalcExtraRatioChangeRequest(txn: *dispatch.CombatRequestTxn(.ToughCalcExtraRatioChangeRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn UseSkillFailPush(_: pb.UseSkillFailPush) !void {}
pub fn UseSkillFailRequest(txn: *dispatch.CombatRequestTxn(.UseSkillFailRequest)) !void {
    txn.respond(pb.UseSkillFailResponse{
        .SkillId = txn.payload.SkillId,
        .Error = .{ .ErrorCode = .Success },
    });
}

pub fn EnterViewDirectionPush(_: pb.EnterViewDirectionPush) !void {}
pub fn EnterViewDirectionRequest(txn: *dispatch.CombatRequestTxn(.EnterViewDirectionRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn ExitViewDirectionPush(_: pb.ExitViewDirectionPush) !void {}
pub fn ExitViewDirectionRequest(txn: *dispatch.CombatRequestTxn(.ExitViewDirectionRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn InterruptSkillInDelayPush(_: pb.InterruptSkillInDelayPush) !void {}
pub fn InterruptSkillInDelayRequest(txn: *dispatch.CombatRequestTxn(.InterruptSkillInDelayRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn TriggerExitSkillPush(_: pb.TriggerExitSkillPush) !void {}
pub fn TriggerExitSkillRequest(txn: *dispatch.CombatRequestTxn(.TriggerExitSkillRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn BuffEffectPush(_: pb.BuffEffectPush) !void {}
pub fn BuffEffectRequest(txn: *dispatch.CombatRequestTxn(.BuffEffectRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn BuffEffectExecutePush(_: pb.BuffEffectExecutePush) !void {}

fn handleDrownEndTeleport(teleport_pos: ?pb.Vector, qctx: QuestEngine.QuestContext) !void {
    const v = teleport_pos orelse return;
    if (!std.math.isFinite(v.X) or !std.math.isFinite(v.Y) or !std.math.isFinite(v.Z)) return;
    const inst_id: i32 = if (qctx.scene) |s|
        s.instance_id
    else if (qctx.scene_comp) |sc|
        sc.last_scene_info.instance_id
    else
        0;
    const cur_map_id = qctx.assets.resolveMapConfigId(inst_id);
    if (cur_map_id <= 0) return;
    try QuestEngine.updatePlayerSceneState(qctx, .{ v.X, v.Y, v.Z }, null);
    try qctx.conn.push(qctx.alloc.gpa, qctx.io, pb.TeleportNotify{
        .MapId = cur_map_id,
        .Pos = .{ .X = v.X, .Y = v.Y, .Z = v.Z },
        .Reason = .Drown,
    });
}

pub fn DrownEndTeleportPush(
    push: pb.DrownEndTeleportPush,
    qctx: QuestEngine.QuestContext,
) !void {
    const teleport_pos: ?pb.Vector = if (push.ycu) |u| switch (u) {
        .TeleportPos => |pos| pos,
    } else null;
    try handleDrownEndTeleport(teleport_pos, qctx);
}
pub fn DrownEndTeleportRequest(txn: *dispatch.CombatRequestTxn(.DrownEndTeleportRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn RTimeStopRequest(txn: *dispatch.CombatRequestTxn(.RTimeStopRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn RTimeStopInstPush(_: pb.RTimeStopInstPush) !void {}
pub fn RTimeStopInstRequest(txn: *dispatch.CombatRequestTxn(.RTimeStopInstRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn TsAnimNotifyStateAbsoluteTimeStopPush(_: pb.TsAnimNotifyStateAbsoluteTimeStopPush) !void {}
pub fn TsAnimNotifyStateAbsoluteTimeStopRequest(txn: *dispatch.CombatRequestTxn(.TsAnimNotifyStateAbsoluteTimeStopRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn RoleTagChangeRequest(txn: *dispatch.CombatRequestTxn(.RoleTagChangeRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn CreateBulletRequest(txn: *dispatch.CombatRequestTxn(.CreateBulletRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn DestroyBulletRequest(txn: *dispatch.CombatRequestTxn(.DestroyBulletRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn ANStartRequest(txn: *dispatch.CombatRequestTxn(.ANStartRequest)) !void {
    txn.respond(pb.ANStartResponse{
        .SkillId = txn.payload.SkillId,
        .MontageIndex = txn.payload.MontageIndex,
        .AnIndex = txn.payload.AnIndex,
        .Error = .{ .ErrorCode = .Success },
    });
}

pub fn ClientCurrentRoleReportRequest(txn: *dispatch.CombatRequestTxn(.ClientCurrentRoleReportRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn GaSwitchCommonEnemyProCampRequest(txn: *dispatch.CombatRequestTxn(.GaSwitchCommonEnemyProCampRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn CounterAttackPush(_: pb.CounterAttackPush) !void {}
pub fn NewLinkBurstPush(_: pb.NewLinkBurstPush) !void {}
pub fn QuickHackRamVerifyPush(_: pb.QuickHackRamVerifyPush) !void {}
pub fn QuickHackOpenPush(_: pb.QuickHackOpenPush) !void {}
pub fn DodgeInfoPush(_: pb.DodgeInfoPush) !void {}
pub fn RoleTrialSceneRollbackPush(_: pb.RoleTrialSceneRollbackPush) !void {}
pub fn ApplyBuffS2cResponsePush(_: pb.ApplyBuffS2cResponsePush) !void {}
pub fn RemoveBuffS2cResponsePush(_: pb.RemoveBuffS2cResponsePush) !void {}
pub fn RemoveBuffByIdS2cResponsePush(_: pb.RemoveBuffByIdS2cResponsePush) !void {}
