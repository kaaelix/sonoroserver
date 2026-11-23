const std = @import("std");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const common = @import("common");
const FileSystem = common.FileSystem;
const Assets = @import("../../data/Assets.zig");
const State = @import("../State.zig");
const Scene = @import("../../logic/Scene.zig");
const EventQueue = @import("../../logic/EventQueue.zig");
const Transaction = @import("../handlers.zig").Transaction;
const combat = @import("combat.zig");
const PlayerBasicComponent = @import("../../logic/component/player/PlayerBasicComponent.zig");
const PlayerInventoryComponent = @import("../../logic/component/player/PlayerInventoryComponent.zig");
const PlayerProgressComponent = @import("../../logic/component/player/PlayerProgressComponent.zig");
const PlayerWeaponComponent = @import("../../logic/component/player/PlayerWeaponComponent.zig");
const PlayerEchoComponent = @import("../../logic/component/player/PlayerEchoComponent.zig");
const PlayerMapComponent = @import("../../logic/component/player/PlayerMapComponent.zig");
const PlayerQuestComponent = @import("../../logic/component/player/PlayerQuestComponent.zig");
const PlayerSceneComponent = @import("../../logic/component/player/PlayerSceneComponent.zig");
const PlayerRoleComponent = @import("../../logic/component/player/PlayerRoleComponent.zig");
const ConsumeHelper = @import("../../logic/helpers/consume.zig");
const EnergyHelper = @import("../../logic/helpers/energy.zig");
const QuestEngine = @import("../../logic/helpers/quest_engine.zig");
const LevelPlayHelper = @import("../../logic/helpers/level_play.zig");

const log = std.log.scoped(.combat_sync);

fn commonFromMsg(msg: anytype) ?pb.CombatCommon {
    const T = @TypeOf(msg);
    if (@hasField(T, "CombatCommon")) return msg.CombatCommon;
    if (@hasField(T, "Id")) {
        const id_val: i64 = @intCast(msg.Id);
        if (id_val != 0) return .{ .EntityId = id_val };
    }
    if (@hasField(T, "EntityId")) {
        const id_val: i64 = @intCast(msg.EntityId);
        if (id_val != 0) return .{ .EntityId = id_val };
    }
    return null;
}

fn forwardCombatRequest(
    comptime tag: anytype,
    txn: anytype,
    state: *State,
    alloc: mem.Alloc,
    io: std.Io,
    events: *EventQueue,
) !void {
    const ReqUnion = @typeInfo(@FieldType(pb.CombatRequestData, "Message")).optional.child;
    const RespUnion = @typeInfo(@FieldType(pb.CombatResponseData, "Message")).optional.child;
    const req_name = @tagName(tag);
    const resp_name = req_name[0 .. req_name.len - "Request".len] ++ "Response";

    var receive_pack: std.ArrayList(pb.CombatReceiveData) = .empty;
    const data: pb.CombatRequestData = .{
        .CombatCommon = commonFromMsg(txn.message),
        .RequestId = 0,
        .Message = @unionInit(ReqUnion, req_name, txn.message),
    };

    const maybe_resp = try combat.dispatch(pb.CombatRequestData, data, state, alloc.gpa, io, events, &receive_pack);
    if (receive_pack.items.len != 0) {
        try txn.conn.push(alloc.gpa, io, pb.CombatReceivePackNotify{ .Data = receive_pack });
    }

    if (maybe_resp) |recv| {
        if (recv.Message) |m| switch (m) {
            .CombatResponseData => |cr_opt| {
                if (cr_opt) |cr| {
                    if (cr.Message) |rm| {
                        switch (rm) {
                            inline else => |payload_opt, rtag| {
                                if (comptime std.mem.eql(u8, @tagName(rtag), resp_name)) {
                                    if (payload_opt) |resp_val| {
                                        txn.respond(resp_val);
                                        return;
                                    }
                                }
                            },
                        }
                    }
                }
            },
            else => {},
        };
    }

    const RespType = @typeInfo(@FieldType(RespUnion, resp_name)).optional.child;
    var fallback: RespType = .default;
    if (@hasField(RespType, "ErrorCode")) fallback.ErrorCode = .Success;
    if (@hasField(RespType, "ErrCode")) fallback.ErrCode = .Success;
    if (@hasField(RespType, "ErrorId")) fallback.ErrorId = .Success;
    if (@hasField(RespType, "Code")) fallback.Code = .Success;
    txn.respond(fallback);
}

fn forwardCombatPush(
    comptime tag: anytype,
    msg: anytype,
    state: *State,
    alloc: mem.Alloc,
    io: std.Io,
    events: *EventQueue,
) !void {
    const PushUnion = @typeInfo(@FieldType(pb.CombatPushData, "Message")).optional.child;
    var receive_pack: std.ArrayList(pb.CombatReceiveData) = .empty;
    const data: pb.CombatPushData = .{
        .CombatCommon = commonFromMsg(msg),
        .Message = @unionInit(PushUnion, @tagName(tag), msg),
    };
    _ = try combat.dispatch(pb.CombatPushData, data, state, alloc.gpa, io, events, &receive_pack);
    if (receive_pack.items.len != 0) {
        try state.conn.push(alloc.gpa, io, pb.CombatReceivePackNotify{ .Data = receive_pack });
    }
}

// =========================================================================
// Top-level EMessageId mirrors of CombatRequestData / CombatPushData
// =========================================================================

pub fn onAnimationStateChangedRequest(txn: *Transaction(pb.AnimationStateChangedRequest), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatRequest(.AnimationStateChangedRequest, txn, state, alloc, io, events);
}
pub fn onCreateBulletRequest(txn: *Transaction(pb.CreateBulletRequest), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatRequest(.CreateBulletRequest, txn, state, alloc, io, events);
}
pub fn onDamageExecuteRequest(txn: *Transaction(pb.DamageExecuteRequest), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatRequest(.DamageExecuteRequest, txn, state, alloc, io, events);
}
pub fn onEntityIsVisibleRequest(txn: *Transaction(pb.EntityIsVisibleRequest), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatRequest(.EntityIsVisibleRequest, txn, state, alloc, io, events);
}
pub fn onEntityStaticHookMoveRequest(txn: *Transaction(pb.EntityStaticHookMoveRequest), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatRequest(.EntityStaticHookMoveRequest, txn, state, alloc, io, events);
}
pub fn onFsmStateBehaviorRequest(txn: *Transaction(pb.FsmStateBehaviorRequest), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatRequest(.FsmStateBehaviorRequest, txn, state, alloc, io, events);
}
pub fn onHitRequest(txn: *Transaction(pb.HitRequest), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatRequest(.HitRequest, txn, state, alloc, io, events);
}
pub fn onOrderApplyBuffRequest(txn: *Transaction(pb.OrderApplyBuffRequest), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatRequest(.OrderApplyBuffRequest, txn, state, alloc, io, events);
}
pub fn onOrderRemoveBuffRequest(txn: *Transaction(pb.OrderRemoveBuffRequest), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatRequest(.OrderRemoveBuffRequest, txn, state, alloc, io, events);
}
pub fn onOrderRemoveBuffByTagsRequest(txn: *Transaction(pb.OrderRemoveBuffByTagsRequest), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatRequest(.OrderRemoveBuffByTagsRequest, txn, state, alloc, io, events);
}
pub fn onSkillRequest(txn: *Transaction(pb.SkillRequest), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatRequest(.SkillRequest, txn, state, alloc, io, events);
}
pub fn onUseSkillRequest(txn: *Transaction(pb.UseSkillRequest), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatRequest(.UseSkillRequest, txn, state, alloc, io, events);
}
pub fn onChangeStateRequest(txn: *Transaction(pb.ChangeStateRequest), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatRequest(.ChangeStateRequest, txn, state, alloc, io, events);
}
pub fn onSwitchRoleRequest(txn: *Transaction(pb.SwitchRoleRequest), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatRequest(.SwitchRoleRequest, txn, state, alloc, io, events);
}

pub fn onChangeSummonEntityStateRequest(
    txn: *Transaction(pb.ChangeSummonEntityStateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    if (txn.message.SummonEntityId != 0) {
        const key = try std.fmt.allocPrint(alloc.arena, "summon_state:{d}", .{txn.message.SummonEntityId});
        try progress_comp.info.setKeyInt(alloc.gpa, key, txn.message.TagId);
        try progress_comp.save(fs, alloc.arena);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onANStartPush(txn: *Transaction(pb.ANStartPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.ANStartPush, txn.message, state, alloc, io, events);
}
pub fn onActivateBuffPush(txn: *Transaction(pb.ActivateBuffPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.ActiveBuffPush, pb.ActiveBuffPush{
        .Handle = txn.message.Handle,
        .On = txn.message.On,
    }, state, alloc, io, events);
}
pub fn onAnimationGameplayTagPush(txn: *Transaction(pb.AnimationGameplayTagPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.AnimationGameplayTagPush, txn.message, state, alloc, io, events);
}
pub fn onAnimationStateChangedPush(txn: *Transaction(pb.AnimationStateChangedPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.AnimationStateChangedPush, txn.message, state, alloc, io, events);
}
pub fn onAnimationStateInitPush(txn: *Transaction(pb.AnimationStateInitPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.AnimationStateInitPush, txn.message, state, alloc, io, events);
}
pub fn onApplyBuffS2cResponsePush(txn: *Transaction(pb.ApplyBuffS2cResponsePush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.ApplyBuffS2cResponsePush, txn.message, state, alloc, io, events);
}
pub fn onApplyGameplayEffectPush(txn: *Transaction(pb.ApplyGameplayEffectPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.ApplyGameplayEffectPush, txn.message, state, alloc, io, events);
}
pub fn onBuffEffectExecutePush(txn: *Transaction(pb.BuffEffectExecutePush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.BuffEffectExecutePush, txn.message, state, alloc, io, events);
}
pub fn onBuffEffectPush(txn: *Transaction(pb.BuffEffectPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.BuffEffectPush, txn.message, state, alloc, io, events);
}
pub fn onBuffStackCountPush(txn: *Transaction(pb.BuffStackCountPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.BuffStackCountPush, txn.message, state, alloc, io, events);
}
pub fn onBulletPatternPush(txn: *Transaction(pb.BulletPatternPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.BulletPatternPush, txn.message, state, alloc, io, events);
}
pub fn onClientCurrentRoleReportPush(txn: *Transaction(pb.ClientCurrentRoleReportPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.ClientCurrentRoleReportPush, txn.message, state, alloc, io, events);
}
pub fn onDodgeInfoPush(txn: *Transaction(pb.DodgeInfoPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.DodgeInfoPush, txn.message, state, alloc, io, events);
}
pub fn onEndSkillPush(txn: *Transaction(pb.EndSkillPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.EndSkillPush, txn.message, state, alloc, io, events);
}
pub fn onFsmConditionPassPush(txn: *Transaction(pb.FsmConditionPassPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.FsmConditionPassPush, txn.message, state, alloc, io, events);
}
pub fn onGameplayCuePush(txn: *Transaction(pb.GameplayCuePush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.GameplayCuePush, txn.message, state, alloc, io, events);
}
pub fn onHitEndPush(txn: *Transaction(pb.HitEndPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.HitEndPush, txn.message, state, alloc, io, events);
}
pub fn onInterruptSkillInDelayPush(txn: *Transaction(pb.InterruptSkillInDelayPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.InterruptSkillInDelayPush, txn.message, state, alloc, io, events);
}
pub fn onLogicStateInitPush(txn: *Transaction(pb.LogicStateInitPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.LogicStateInitPush, txn.message, state, alloc, io, events);
}
pub fn onModifyBulletParamsPush(txn: *Transaction(pb.ModifyBulletParamsPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.ModifyBulletParamsPush, txn.message, state, alloc, io, events);
}
pub fn onPassiveSkillAddPush(txn: *Transaction(pb.PassiveSkillAddPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.PassiveSkillAddPush, txn.message, state, alloc, io, events);
}
pub fn onPassiveSkillRemovePush(txn: *Transaction(pb.PassiveSkillRemovePush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.PassiveSkillRemovePush, txn.message, state, alloc, io, events);
}
pub fn onQuickHackOpenPush(txn: *Transaction(pb.QuickHackOpenPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.QuickHackOpenPush, txn.message, state, alloc, io, events);
}
pub fn onQuickHackRamVerifyPush(txn: *Transaction(pb.QuickHackRamVerifyPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.QuickHackRamVerifyPush, txn.message, state, alloc, io, events);
}
pub fn onRTimeStopPush(txn: *Transaction(pb.RTimeStopPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.RTimeStopPush, txn.message, state, alloc, io, events);
}
pub fn onRefreshBuffDurationPush(txn: *Transaction(pb.RefreshBuffDurationPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.RefreshBuffDurationPush, txn.message, state, alloc, io, events);
}
pub fn onRemoveBuffByIdS2cResponsePush(txn: *Transaction(pb.RemoveBuffByIdS2cResponsePush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.RemoveBuffByIdS2cResponsePush, txn.message, state, alloc, io, events);
}
pub fn onRemoveBuffS2cResponsePush(txn: *Transaction(pb.RemoveBuffS2cResponsePush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.RemoveBuffS2cResponsePush, txn.message, state, alloc, io, events);
}
pub fn onRemoveGameplayEffectPush(txn: *Transaction(pb.RemoveGameplayEffectPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.RemoveGameplayEffectPush, txn.message, state, alloc, io, events);
}
pub fn onRoleGoDownPush(txn: *Transaction(pb.RoleGoDownPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.RoleGoDownPush, txn.message, state, alloc, io, events);
}
pub fn onSwitchLogicStatePush(txn: *Transaction(pb.SwitchLogicStatePush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.SwitchLogicStatePush, txn.message, state, alloc, io, events);
}
pub fn onToughCalcExtraRatioChangePush(txn: *Transaction(pb.ToughCalcExtraRatioChangePush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.ToughCalcExtraRatioChangePush, txn.message, state, alloc, io, events);
}
pub fn onUseSkillFailPush(txn: *Transaction(pb.UseSkillFailPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.UseSkillFailPush, txn.message, state, alloc, io, events);
}
pub fn onChangeStateConfirmPush(txn: *Transaction(pb.ChangeStateConfirmPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.ChangeStateConfirmPush, txn.message, state, alloc, io, events);
}

// =========================================================================
// Standalone Combat / Buff / FSM / SoundBox / Security / Client RPCs
// =========================================================================

pub fn onBeforeHitPush(txn: *Transaction(pb.BeforeHitPush), state: *State, alloc: mem.Alloc) !void {
    _ = txn.message;
    if (state.scene) |*scene| {
        const nid = scene.currentRoleEntityId();
        if (nid > 0) scene.markFsmDirty(alloc.gpa, nid, Scene.Entity.FsmComponent.WakeReason.event) catch {};
    }
}
pub fn onCreateBulletPush(txn: *Transaction(pb.CreateBulletPush), state: *State, alloc: mem.Alloc) !void {
    if (state.scene) |*scene| {
        if (txn.message.TargetId > 0) scene.markFsmDirty(alloc.gpa, txn.message.TargetId, Scene.Entity.FsmComponent.WakeReason.event) catch {};
        if (txn.message.CombatCommon) |cc| {
            if (cc.EntityId > 0) scene.markFsmDirty(alloc.gpa, cc.EntityId, Scene.Entity.FsmComponent.WakeReason.event) catch {};
        }
    }
}
pub fn onDestroyBulletPush(txn: *Transaction(pb.DestroyBulletPush), state: *State, alloc: mem.Alloc) !void {
    if (state.scene) |*scene| {
        if (txn.message.CombatCommon) |cc| {
            if (cc.EntityId > 0) scene.markFsmDirty(alloc.gpa, cc.EntityId, Scene.Entity.FsmComponent.WakeReason.skill_end) catch {};
        }
    }
}
pub fn onGameplayTagPush(txn: *Transaction(pb.GameplayTagPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.AnimationGameplayTagPush, pb.AnimationGameplayTagPush{
        .AddTagIds = txn.message.TagId,
        .RemoveTagIds = txn.message.TagCount <= 0,
    }, state, alloc, io, events);
}
pub fn onRTimeStopAnimPush(_: *Transaction(pb.RTimeStopAnimPush)) !void {}
pub fn onSlowTimePush(_: *Transaction(pb.SlowTimePush)) !void {}
pub fn onRoleTrialUiEndPush(_: *Transaction(pb.RoleTrialUiEndPush), qctx: QuestEngine.QuestContext) !void {
    QuestEngine.onConditionStateChanged(qctx) catch {};
}
pub fn onRoleSkinTrialUiEndPush(_: *Transaction(pb.RoleSkinTrialUiEndPush), qctx: QuestEngine.QuestContext) !void {
    QuestEngine.onConditionStateChanged(qctx) catch {};
}
pub fn onCheckClientPush(_: *Transaction(pb.CheckClientPush)) !void {}
fn pushExecuteQteNotify(
    conn: anytype,
    alloc: mem.Alloc,
    io: std.Io,
    down_entity_id: i64,
    up_entity_id: i64,
    fnv_hash: i32,
) !void {
    var notify_pack: std.ArrayList(pb.CombatReceiveData) = .empty;
    try notify_pack.append(alloc.arena, .{ .Message = .{
        .CombatNotifyData = .{
            .CombatCommon = .{ .EntityId = up_entity_id },
            .Message = .{
                .ExecuteQteNotify = .{
                    .DownEntityId = down_entity_id,
                    .UpEntityId = up_entity_id,
                    .FnvHash = fnv_hash,
                },
            },
        },
    } });
    try conn.push(alloc.gpa, io, pb.CombatReceivePackNotify{ .Data = notify_pack });
}

pub fn onExecuteQtePush(txn: *Transaction(pb.ExecuteQtePush), alloc: mem.Alloc, io: std.Io) !void {
    try pushExecuteQteNotify(txn.conn, alloc, io, txn.message.DownEntityId, txn.message.UpEntityId, txn.message.FnvHash);
}

pub fn onExecuteQteRequest(txn: *Transaction(pb.ExecuteQteRequest), alloc: mem.Alloc, io: std.Io) !void {
    try pushExecuteQteNotify(txn.conn, alloc, io, txn.message.DownEntityId, txn.message.UpEntityId, txn.message.FnvHash);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onBuffItemRequest(
    txn: *Transaction(pb.BuffItemRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    scene_opt: *?Scene,
    events: *EventQueue,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    progress_comp: *PlayerProgressComponent,
    io: std.Io,
) !void {
    const count = if (txn.message.Num > 0) txn.message.Num else 1;
    if (txn.message.ItemId > 0) {
        const costs = [_]ConsumeHelper.CostItem{.{ .id = txn.message.ItemId, .count = count }};
        if (!(try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, &costs, io))) {
            txn.respond(.{ .ErrorCode = .ErrItemNotEnough, .RoleId = txn.message.RoleId });
            return;
        }
    }

    const buff_cfg_opt = assets.tables.buff_item.getDataById(txn.message.ItemId);
    const cd_sec: i64 = if (buff_cfg_opt) |bi| @max(@as(i64, 1), @as(i64, bi.Cd)) else 10;
    const now_ms = EnergyHelper.nowSeconds(io) * 1000;
    const cd_until = now_ms + cd_sec * 1000;
    const key = try std.fmt.allocPrint(alloc.arena, "buff_cd:{d}", .{txn.message.ItemId});
    try progress_comp.info.setKeyInt(alloc.gpa, key, cd_until);
    try progress_comp.save(fs, alloc.arena);

    if (buff_cfg_opt) |bi| {
        if (bi.Buffs.len > 0) {
            if (scene_opt.*) |*scene| {
                const target_role = if (txn.message.RoleId > 0) txn.message.RoleId else blk: {
                    if (scene.formation_info.cur_formation >= 0 and
                        @as(usize, @intCast(scene.formation_info.cur_formation)) < scene.formation_info.formations.len)
                    {
                        break :blk scene.formation_info.formations[@intCast(scene.formation_info.cur_formation)].cur_role;
                    }
                    break :blk 0;
                };
                if (target_role > 0) {
                    const cfgs = scene.entities.items(.config);
                    for (cfgs, 0..) |c, idx| {
                        if (c.entity_type == .player and c.config_type == .character and c.config_id == target_role) {
                            const ent: Scene.Entity = .{ .net_id = scene.entities.items(.entity_id)[idx].net_id, .index = idx };
                            const entries = try alloc.arena.alloc(@import("../../logic/events.zig").BuffAdditionEntry, bi.Buffs.len);
                            for (bi.Buffs, 0..) |bid, bi_idx| {
                                entries[bi_idx] = .{ .id = bid, .stack_count = 1, .is_active = true };
                            }
                            try events.enqueue(.buff_addition, .{ .target = ent, .instigator = ent, .buffs = entries });
                            break;
                        }
                    }
                }
            }
        }
    }

    txn.respond(.{
        .BuffItemInfo = .{ .ItemId = txn.message.ItemId, .CdTime = cd_until },
        .RoleId = txn.message.RoleId,
        .ErrorCode = .Success,
    });
}

pub fn onAtomicProcessAddBuffRequest(
    txn: *Transaction(pb.AtomicProcessAddBuffRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    for (txn.message.BuffIds.items) |buff_id| {
        const key = try std.fmt.allocPrint(alloc.arena, "bt_buff:{d}:{d}:{d}", .{
            txn.message.TreeIncId,
            txn.message.EntityId,
            buff_id,
        });
        try progress_comp.info.setKeyInt(alloc.gpa, key, 1);
    }
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorId = .Success });
}

pub fn onAtomicProcessRemoveBuffRequest(
    txn: *Transaction(pb.AtomicProcessRemoveBuffRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    for (txn.message.BuffIds.items) |buff_id| {
        const key = try std.fmt.allocPrint(alloc.arena, "bt_buff:{d}:{d}:{d}", .{
            txn.message.TreeIncId,
            txn.message.EntityId,
            buff_id,
        });
        try progress_comp.info.setKeyInt(alloc.gpa, key, 0);
    }
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorId = .Success });
}

pub fn onAtomicProcessTeleportRequest(
    txn: *Transaction(pb.AtomicProcessTeleportRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "combat:atomic_tp", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorId = .Success });
}

pub fn onAvoidBlockClientInitDoneRequest(
    txn: *Transaction(pb.AvoidBlockClientInitDoneRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "avoid_block:init:{d}", .{txn.message.NodeId});
    try progress_comp.info.setKeyInt(alloc.gpa, key, txn.message.TreeIncId);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorId = .Success });
}

pub fn onAvoidBlockSubmitOverNodeRequest(
    txn: *Transaction(pb.AvoidBlockSubmitOverNodeRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "avoid_block:score:{d}", .{txn.message.NodeId});
    if (txn.message.Score > progress_comp.info.getKeyInt(key)) {
        try progress_comp.info.setKeyInt(alloc.gpa, key, txn.message.Score);
    }
    if (txn.message.Win) {
        const wkey = try std.fmt.allocPrint(alloc.arena, "avoid_block:win:{d}", .{txn.message.NodeId});
        _ = try progress_comp.info.markClaim(alloc.gpa, wkey);
    }
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorId = .Success });
}

pub fn onBeamCastHitPlayerRequest(
    txn: *Transaction(pb.BeamCastHitPlayerRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "combat:beam_hit", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .Code = .Success });
}

pub fn onDeadeyeShootHitRequest(
    txn: *Transaction(pb.DeadeyeShootHitRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "deadeye:hits", @intCast(txn.message.EntityIds.items.len));
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onTargetGearHitRequest(
    txn: *Transaction(pb.TargetGearHitRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "gear_hit:{d}", .{txn.message.EntityId});
    _ = try progress_comp.info.addKeyInt(alloc.gpa, key, 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onThrowDamageRequest(
    txn: *Transaction(pb.ThrowDamageRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "combat:throw_dmg", 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onExploreSkillActionRequest(
    txn: *Transaction(pb.ExploreSkillActionRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "explore:skill_act:{d}", .{txn.message.EntityId});
    _ = try progress_comp.info.addKeyInt(alloc.gpa, key, 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .EntityId = txn.message.EntityId,
        .Code = .Success,
    });
}

pub fn onEntityBuffProducerRequest(
    txn: *Transaction(pb.EntityBuffProducerRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "buff_prod:req:{d}", .{txn.message.BuffConsumerId});
    _ = try progress_comp.info.addKeyInt(alloc.gpa, key, 1);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onEntityBuffProducerOperateRequest(
    txn: *Transaction(pb.EntityBuffProducerOperateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "buff_prod:op:{d}", .{txn.message.OpEntityId});
    const op_val: i64 = if (txn.message.OpType) |ot| @intFromEnum(ot) else 0;
    try progress_comp.info.setKeyInt(alloc.gpa, key, op_val);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{
        .ErrorCode = .Success,
        .OpEntityId = txn.message.OpEntityId,
        .OpType = txn.message.OpType,
    });
}

pub fn onQuickHackSkillRequest(
    txn: *Transaction(pb.QuickHackSkillRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    const key = try std.fmt.allocPrint(alloc.arena, "quickhack:{d}", .{txn.message.EntityId});
    try progress_comp.info.setKeyInt(alloc.gpa, key, txn.message.SkillId);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

fn findNearestDetectableEntity(scene: *Scene) ?pb.DetectionEntityPosInfo {
    var role_pos: [3]f32 = .{ 0, 0, 0 };
    var has_role_pos = false;
    const role_eid = scene.currentRoleEntityId();
    if (role_eid != 0) {
        if (scene.net_id_map.get(role_eid)) |ridx| {
            if (scene.entities.items(.position)[ridx]) |p| {
                role_pos = p.location;
                has_role_pos = true;
            }
        }
    }

    var best_dist_sq: f64 = std.math.inf(f64);
    var best: ?pb.DetectionEntityPosInfo = null;
    const cfgs = scene.entities.items(.config);
    const positions = scene.entities.items(.position);
    const visibles = scene.entities.items(.visible);
    for (cfgs, positions, visibles) |cfg, maybe_pos, maybe_vis| {
        if (cfg.entity_type == .player or maybe_vis == null) continue;
        const pos = maybe_pos orelse continue;
        if (!has_role_pos) {
            return .{
                .Pos = .{ .X = pos.location[0], .Y = pos.location[1], .Z = pos.location[2] },
                .ConfigId = cfg.config_id,
            };
        }
        const dx = @as(f64, pos.location[0]) - @as(f64, role_pos[0]);
        const dy = @as(f64, pos.location[1]) - @as(f64, role_pos[1]);
        const dz = @as(f64, pos.location[2]) - @as(f64, role_pos[2]);
        const dist_sq = dx * dx + dy * dy + dz * dz;
        if (dist_sq < best_dist_sq) {
            best_dist_sq = dist_sq;
            best = .{
                .Pos = .{ .X = pos.location[0], .Y = pos.location[1], .Z = pos.location[2] },
                .ConfigId = cfg.config_id,
            };
        }
    }
    return best;
}

pub fn onCheckUseSoundBoxSkillRequest(
    txn: *Transaction(pb.CheckUseSoundBoxSkillRequest),
    scene_opt: *?Scene,
) !void {
    const has_target = if (scene_opt.*) |*scene| findNearestDetectableEntity(scene) != null else true;
    txn.respond(.{ .Code = if (has_target) .Success else .ErrNoSoundBox });
}

pub fn onUseSoundBoxSkillRequest(
    txn: *Transaction(pb.UseSoundBoxSkillRequest),
    scene_opt: *?Scene,
) !void {
    var detected: ?pb.DetectionEntityPosInfo = null;
    if (scene_opt.*) |*scene| {
        detected = findNearestDetectableEntity(scene);
    }
    txn.respond(.{
        .Code = .Success,
        .DetectionEntityPosInfo = detected,
    });
}

pub fn onGmEntityFsmGroupInfoRequest(
    txn: *Transaction(pb.GmEntityFsmGroupInfoRequest),
    scene_opt: *?Scene,
    alloc: mem.Alloc,
) !void {
    var machines: std.ArrayList(pb.GmEntityFsmMachineInfo) = .empty;
    if (scene_opt.*) |*scene| {
        if (scene.net_id_map.get(txn.message.EntityId)) |idx| {
            if (scene.entities.items(.fsm)[idx]) |fsm_comp| {
                for (fsm_comp.runtime_nodes, 0..) |node, i| {
                    var states: std.ArrayList(pb.GmEntityFsmStateInfo) = .empty;
                    try states.append(alloc.arena, .{ .StateIndex = node.leaf() orelse 0 });
                    try machines.append(alloc.arena, .{
                        .StateIndex = @intCast(i),
                        .GmEntityFsmStateInfos = states,
                    });
                }
            }
        }
    }
    txn.respond(.{
        .EntityId = txn.message.EntityId,
        .GmEntityFsmMachineInfos = machines,
    });
}

pub fn onSimpleCombatEntityDieRequest(
    txn: *Transaction(pb.SimpleCombatEntityDieRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    scene_opt: *?Scene,
    qctx: QuestEngine.QuestContext,
    progress_comp: *PlayerProgressComponent,
) !void {
    for (txn.message.EntityDieCtxMap.items) |entry| {
        _ = try progress_comp.info.addKeyInt(alloc.gpa, "simple_combat:kills", 1);
        const ekey = try std.fmt.allocPrint(alloc.arena, "simple_kill:{d}", .{entry.key});
        try progress_comp.info.setKeyInt(alloc.gpa, ekey, 1);

        if (entry.value.TrapDefenseGoldenCoinAbosorbedCtx != null or entry.value.SurvivorsGoldenCoinAbosorbedCtx != null) {
            _ = try progress_comp.info.addKeyInt(alloc.gpa, "simple_combat:coins", 1);
        }
        if (scene_opt.* != null) {
            const cfg_opt = std.math.cast(i32, entry.key);
            QuestEngine.onEntityKilled(qctx, entry.key, cfg_opt) catch |err| QuestEngine.logHookErr("onEntityKilled", err);
            if (LevelPlayHelper.fromQuestContext(qctx)) |lp_ctx| {
                LevelPlayHelper.onEntityKilledWithConditions(lp_ctx, progress_comp, entry.key, cfg_opt) catch |err| {
                    std.log.scoped(.level_play).warn("onEntityKilled failed: {t}", .{err});
                };
            }
        }
    }
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onSimpleCombatEntitySyncAttributePush(
    txn: *Transaction(pb.SimpleCombatEntitySyncAttributePush),
    scene_opt: *?Scene,
) !void {
    if (scene_opt.*) |*scene| {
        for (txn.message.EntityAttributeMap.items) |entry| {
            if (scene.net_id_map.get(entry.key)) |idx| {
                if (scene.entities.items(.attribute)[idx]) |*attr| {
                    for (entry.value.AttributeMap.items) |kv| {
                        if (kv.key >= 0 and kv.key < @as(i32, @intCast(attr.attributes.len))) {
                            attr.attributes[@intCast(kv.key)].current = kv.value;
                        }
                    }
                }
            }
        }
    }
}

pub fn onSetClientGlobalVarRequest(
    txn: *Transaction(pb.SetClientGlobalVarRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    if (txn.message.VarName.len > 0) {
        const key = try std.fmt.allocPrint(alloc.arena, "var:{s}", .{txn.message.VarName});
        if (txn.message.VarDefinePb) |v| {
            if (v.Value) |val| switch (val) {
                .Boolean => |b| try progress_comp.info.setKeyInt(alloc.gpa, key, if (b) 1 else 0),
                .Int => |n| try progress_comp.info.setKeyInt(alloc.gpa, key, n),
                .Float => |f| try progress_comp.info.setKeyInt(alloc.gpa, key, @intFromFloat(f * 1000.0)),
                .Entity => |e| try progress_comp.info.setKeyInt(alloc.gpa, key, e),
                .Quest => |q| try progress_comp.info.setKeyInt(alloc.gpa, key, q),
                .String => |s| try progress_comp.info.setKeyStr(alloc.gpa, key, s),
                .QuestState => |qs| try progress_comp.info.setKeyInt(alloc.gpa, key, if (qs) |qsv| @intFromEnum(qsv) else 0),
                else => try progress_comp.info.setKeyInt(alloc.gpa, key, 1),
            };
        }
        try progress_comp.save(fs, alloc.arena);
    }
    txn.respond(.{ .ErrorId = .Success });
}

pub fn onClientBasicInfoRequest(
    txn: *Transaction(pb.ClientBasicInfoRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    if (txn.message.ClientBasicInfo) |info| {
        if (info.Platform.len > 0) try progress_comp.info.setKeyStr(alloc.gpa, "client:platform", info.Platform);
        if (info.OS.len > 0) try progress_comp.info.setKeyStr(alloc.gpa, "client:os", info.OS);
        if (info.Model.len > 0) try progress_comp.info.setKeyStr(alloc.gpa, "client:model", info.Model);
        try progress_comp.info.setKeyInt(alloc.gpa, "client:lang", info.Language);
        try progress_comp.save(fs, alloc.arena);
        log.debug("client basic info platform={s} os={s}", .{ info.Platform, info.OS });
    }
    txn.respond(.{});
}

pub fn onSimpleTrackReportRequest(
    txn: *Transaction(pb.SimpleTrackReportRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    progress_comp: *PlayerProgressComponent,
    quest_comp: *const PlayerQuestComponent,
    scene_comp: *const PlayerSceneComponent,
) !void {
    _ = try progress_comp.info.addKeyInt(alloc.gpa, "track:report_cnt", 1);
    try progress_comp.save(fs, alloc.arena);
    var msgs: std.ArrayList(pb.SimpleTrackReportMsg) = .empty;
    for (quest_comp.info.completed_level_plays) |lp_id| {
        const lp_opt = assets.tables.level_play_data.getDataById(lp_id);
        const inst_id: i32 = if (lp_opt) |lp|
            (if (lp.Data.InstanceId != 0) lp.Data.InstanceId else scene_comp.last_scene_info.instance_id)
        else
            scene_comp.last_scene_info.instance_id;
        const chest_cnt: i32 = if (lp_opt) |lp|
            (if (lp.Data.LevelPlayRewardConfig) |rc|
                (if (rc.RewardEntityId > 0 and quest_comp.info.hasOpenedChest(rc.RewardEntityId)) @as(i32, 1) else 0)
            else
                0)
        else
            0;
        try msgs.append(alloc.arena, .{
            .InstId = inst_id,
            .LevelPlayId = lp_id,
            .GainTreasureCount = chest_cnt,
        });
    }
    if (msgs.items.len == 0 and quest_comp.info.opened_chests.len > 0) {
        try msgs.append(alloc.arena, .{
            .InstId = scene_comp.last_scene_info.instance_id,
            .LevelPlayId = 0,
            .GainTreasureCount = @intCast(quest_comp.info.opened_chests.len),
        });
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .SimpleTrackReportMsgs = msgs,
    });
}

pub fn onSecFBRoundStartFlowRequest(txn: *Transaction(pb.SecFBRoundStartFlowRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success, .LogId = txn.message.LogId });
}

pub fn onSecFBRoundEndFlowReportRequest(txn: *Transaction(pb.SecFBRoundEndFlowReportRequest)) !void {
    txn.respond(.{ .Error = .{ .ErrorCode = .Success }, .LogId = txn.message.LogId });
}

pub fn onSecRoleFightFlowRequest(txn: *Transaction(pb.SecRoleFightFlowRequest)) !void {
    txn.respond(.{ .Error = .{ .ErrorCode = .Success }, .LogId = txn.message.LogId });
}

pub fn onSecWorldStartFlowRequest(txn: *Transaction(pb.SecWorldStartFlowRequest)) !void {
    txn.respond(.{ .Error = .{ .ErrorCode = .Success }, .LogId = txn.message.LogId });
}

pub fn onSecWorldInfoFlowRequest(txn: *Transaction(pb.SecWorldInfoFlowRequest)) !void {
    txn.respond(.{ .Error = .{ .ErrorCode = .Success }, .LogId = txn.message.LogId });
}

pub fn onSecWorldSEndFlowRequest(txn: *Transaction(pb.SecWorldSEndFlowRequest)) !void {
    txn.respond(.{ .Error = .{ .ErrorCode = .Success }, .LogId = txn.message.LogId });
}

pub fn onFragileChangeRequest(txn: *Transaction(pb.FragileChangeRequest), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatRequest(.FragileChangeRequest, txn, state, alloc, io, events);
}

pub fn onActorVisiblePush(txn: *Transaction(pb.ActorVisiblePush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.ActorVisiblePush, txn.message, state, alloc, io, events);
}
pub fn onEntityIsVisiblePush(txn: *Transaction(pb.EntityIsVisiblePush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.EntityIsVisiblePush, txn.message, state, alloc, io, events);
}
pub fn onBoneVisibleChangePush(txn: *Transaction(pb.BoneVisibleChangePush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.BoneVisibleChangePush, txn.message, state, alloc, io, events);
}
pub fn onCaughtPush(txn: *Transaction(pb.CaughtPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.CaughtPush, txn.message, state, alloc, io, events);
}
pub fn onDrownEndTeleportPush(txn: *Transaction(pb.DrownEndTeleportPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.DrownEndTeleportPush, txn.message, state, alloc, io, events);
}
pub fn onDrownPush(txn: *Transaction(pb.DrownPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.DrownPush, txn.message, state, alloc, io, events);
}
pub fn onEnterViewDirectionPush(txn: *Transaction(pb.EnterViewDirectionPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.EnterViewDirectionPush, txn.message, state, alloc, io, events);
}
pub fn onExitViewDirectionPush(txn: *Transaction(pb.ExitViewDirectionPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.ExitViewDirectionPush, txn.message, state, alloc, io, events);
}
pub fn onEntityStaticHookMovePush(txn: *Transaction(pb.EntityStaticHookMovePush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.EntityStaticHookMovePush, txn.message, state, alloc, io, events);
}
pub fn onMaterialPush(txn: *Transaction(pb.MaterialPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.MaterialPush, txn.message, state, alloc, io, events);
}
pub fn onMonsterBoomPush(txn: *Transaction(pb.MonsterBoomPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.MonsterBoomPush, txn.message, state, alloc, io, events);
}
pub fn onMonsterDrownPush(txn: *Transaction(pb.MonsterDrownPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.MonsterDrownPush, txn.message, state, alloc, io, events);
}
pub fn onNewLinkBurstPush(txn: *Transaction(pb.NewLinkBurstPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.NewLinkBurstPush, txn.message, state, alloc, io, events);
}
pub fn onVisionTriggerPush(txn: *Transaction(pb.VisionTriggerPush), state: *State, alloc: mem.Alloc, io: std.Io, events: *EventQueue) !void {
    try forwardCombatPush(.VisionTriggerPush, txn.message, state, alloc, io, events);
}
