const std = @import("std");
const pb = @import("proto").pb;
const mem = @import("../../mem.zig");
const dispatch = @import("combat.zig");
const EventQueue = @import("../../logic/EventQueue.zig");
const FsmLifecycle = @import("../../logic/FsmLifecycle.zig");
const Scene = @import("../../logic/Scene.zig");
const QuestEngine = @import("../../logic/helpers/quest_engine.zig");
const Entity = Scene.Entity;

const EntityConfigQuery = Scene.Query(&.{
    Entity,
    *Entity.ConfigComponent,
});

const FsmQuery = Scene.Query(&.{
    Entity,
    *Entity.FsmComponent,
    ?*Entity.AttributeComponent,
    ?*Entity.FightBuffComponent,
    ?*Entity.LogicStateComponent,
    ?*Entity.TagComponent,
    ?*Entity.PartComponent,
});

const FsmBehaviorQuery = Scene.Query(&.{
    Entity,
    ?*Entity.FsmComponent,
});

const AiHateQuery = Scene.Query(&.{
    Entity,
    ?*Entity.FsmComponent,
    ?*Entity.AttributeComponent,
    ?*Entity.FightBuffComponent,
    ?*Entity.LogicStateComponent,
    ?*Entity.TagComponent,
    ?*Entity.PartComponent,
});

const LogicStateQuery = Scene.Query(&.{
    Entity,
    *Entity.LogicStateComponent,
});

const GameplayTagQuery = Scene.Query(&.{
    Entity,
    ?*Entity.TagComponent,
    ?*Entity.FsmComponent,
    ?*Entity.AttributeComponent,
    ?*Entity.FightBuffComponent,
    ?*Entity.LogicStateComponent,
    ?*Entity.PartComponent,
});

const PartComponentQuery = Scene.Query(&.{
    Entity,
    *Entity.PartComponent,
    ?*Entity.TagComponent,
    ?*Entity.FsmComponent,
});

pub fn HitRequest(txn: *dispatch.CombatRequestTxn(.HitRequest)) !void {
    txn.respond(.{
        .HitInfo = txn.payload.HitInfo,
        .ErrorCode = .Success,
    });
}

pub fn SwitchCharacterStateRequest(
    txn: *dispatch.CombatRequestTxn(.SwitchCharacterStateRequest),
    scene: *Scene,
    alloc: mem.Alloc,
) !void {
    const req = txn.payload;
    const eid = if (req.Id != 0) req.Id else commonEntityId(req.CombatCommon orelse txn.common) orelse 0;
    try applyCharacterStateSwitch(scene, eid, req.OldState, req.NewState, alloc, txn.receive_data_pack);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn SwitchCharacterStatePush(
    push: pb.SwitchCharacterStatePush,
    common: ?pb.CombatCommon,
    scene: *Scene,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    const eid = if (push.Id != 0) push.Id else commonEntityId(push.CombatCommon orelse common) orelse 0;
    try applyCharacterStateSwitch(scene, eid, push.OldState, push.NewState, alloc, receive_data_pack);
}

fn applyCharacterStateSwitch(
    scene: *Scene,
    entity_id: i64,
    old_state: ?pb.EntityState,
    new_state: ?pb.EntityState,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    if (entity_id == 0) return;
    const idx = scene.net_id_map.get(entity_id) orelse return;
    const next_state = new_state orelse return;
    const mapped: Entity.ConfigComponent.EntityState = switch (next_state) {
        .Default => .default,
        .Sleep => .sleep,
        .Born => .born,
        .Other => .other,
    };
    const prev_state: pb.EntityState = switch (scene.entities.items(.config)[idx].state) {
        .default => .Default,
        .sleep => .Sleep,
        .born => .Born,
        .other => .Other,
    };
    scene.entities.items(.config)[idx].state = mapped;
    try receive_data_pack.append(alloc.arena, .{ .Message = .{
        .CombatNotifyData = .{
            .CombatCommon = .{ .EntityId = entity_id },
            .Message = .{ .SwitchCharacterStateNotify = .{
                .CombatCommon = .{ .EntityId = entity_id },
                .Id = entity_id,
                .OldState = old_state orelse prev_state,
                .NewState = next_state,
            } },
        },
    } });
}

pub fn EntityIsVisibleRequest(
    txn: *dispatch.CombatRequestTxn(.EntityIsVisibleRequest),
    scene: *Scene,
    alloc: mem.Alloc,
) !void {
    const req = txn.payload;
    const eid = if (req.Id != 0) req.Id else commonEntityId(req.CombatCommon orelse txn.common) orelse 0;
    try applyEntityVisible(scene, eid, req.IsVisible, alloc, txn.receive_data_pack);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn EntityIsVisiblePush(
    push: pb.EntityIsVisiblePush,
    common: ?pb.CombatCommon,
    scene: *Scene,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    const eid = if (push.Id != 0) push.Id else commonEntityId(push.CombatCommon orelse common) orelse 0;
    try applyEntityVisible(scene, eid, push.IsVisible, alloc, receive_data_pack);
}

fn applyEntityVisible(
    scene: *Scene,
    entity_id: i64,
    is_visible: bool,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    if (entity_id == 0) return;
    const idx = scene.net_id_map.get(entity_id) orelse return;
    scene.entities.items(.visible)[idx] = if (is_visible) .{} else null;
    try receive_data_pack.append(alloc.arena, .{ .Message = .{
        .CombatNotifyData = .{
            .CombatCommon = .{ .EntityId = entity_id },
            .Message = .{ .EntityIsVisibleNotify = .{
                .Id = entity_id,
                .IsVisible = is_visible,
                .CombatCommon = .{ .EntityId = entity_id },
            } },
        },
    } });
}

pub fn ActorVisibleRequest(
    txn: *dispatch.CombatRequestTxn(.ActorVisibleRequest),
    scene: *Scene,
    alloc: mem.Alloc,
) !void {
    const req = txn.payload;
    const eid = if (req.Id != 0) req.Id else commonEntityId(txn.common) orelse 0;
    try applyActorVisible(scene, eid, req.IsActorVisible, alloc, txn.receive_data_pack);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn ActorVisiblePush(
    push: pb.ActorVisiblePush,
    common: ?pb.CombatCommon,
    scene: *Scene,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    const eid = if (push.Id != 0) push.Id else commonEntityId(common) orelse 0;
    try applyActorVisible(scene, eid, push.IsActorVisible, alloc, receive_data_pack);
}

fn applyActorVisible(
    scene: *Scene,
    entity_id: i64,
    is_actor_visible: bool,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    if (entity_id == 0) return;
    const idx = scene.net_id_map.get(entity_id) orelse return;
    scene.entities.items(.actor_visible)[idx] = if (is_actor_visible) .{} else null;
    try receive_data_pack.append(alloc.arena, .{ .Message = .{
        .CombatNotifyData = .{
            .CombatCommon = .{ .EntityId = entity_id },
            .Message = .{ .ActorVisibleNotify = .{
                .Id = entity_id,
                .IsActorVisible = is_actor_visible,
            } },
        },
    } });
}

pub fn PartUpdateRequest(
    txn: *dispatch.CombatRequestTxn(.PartUpdateRequest),
    scene: *Scene,
    query: PartComponentQuery,
    alloc: mem.Alloc,
) !void {
    const eid = if (txn.payload.EntityId != 0) txn.payload.EntityId else commonEntityId(txn.common) orelse 0;
    try applyPartUpdates(scene, query, eid, txn.payload.PartUpdateInfos.items, alloc, txn.receive_data_pack);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn PartUpdatePush(
    push: pb.PartUpdatePush,
    common: ?pb.CombatCommon,
    scene: *Scene,
    query: PartComponentQuery,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    const eid = if (push.EntityId != 0) push.EntityId else commonEntityId(common) orelse 0;
    try applyPartUpdates(scene, query, eid, push.PartUpdateInfos.items, alloc, receive_data_pack);
}

fn applyPartUpdates(
    scene: *Scene,
    query: PartComponentQuery,
    entity_id: i64,
    updates: []const pb.PartUpdateInfo,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    if (entity_id == 0 or updates.len == 0) return;
    const item = query.byNetId(entity_id) orelse return;
    const entity, const part_comp, const maybe_tags, const maybe_fsm = item;

    var changed_indices: std.ArrayList(usize) = .empty;
    var any_activation = false;
    var part_infos: std.ArrayList(pb.PartInformation) = .empty;

    for (updates) |upd| {
        for (part_comp.parts, 0..) |*part, idx| {
            if (part.index != upd.PartIndex) continue;
            if (upd.Reset) {
                part.life = part.max_life;
                if (part.activated != upd.Activated) {
                    part.activated = upd.Activated;
                    any_activation = true;
                }
            } else if (part.activated != upd.Activated) {
                part.activated = upd.Activated;
                any_activation = true;
            }
            try changed_indices.append(alloc.arena, idx);
            try part_comp.appendPartInfo(idx, &part_infos, alloc.arena);
        }
    }

    if (any_activation) {
        try part_comp.syncActiveTags(maybe_tags, alloc.gpa);
        if (maybe_fsm != null) {
            try scene.markFsmDirty(alloc.gpa, entity.net_id, Entity.FsmComponent.WakeReason.part | Entity.FsmComponent.WakeReason.tag);
        }
    } else if (changed_indices.items.len != 0 and maybe_fsm != null) {
        try scene.markFsmDirty(alloc.gpa, entity.net_id, Entity.FsmComponent.WakeReason.part);
    }

    if (part_infos.items.len != 0) {
        try receive_data_pack.append(alloc.arena, .{ .Message = .{
            .CombatNotifyData = .{
                .CombatCommon = .{ .EntityId = entity.net_id },
                .Message = .{ .PartUpdateNotify = .{
                    .EntityId = entity.net_id,
                    .PartInfos = part_infos,
                } },
            },
        } });
    }
}

pub fn HitEndRequest(txn: *dispatch.CombatRequestTxn(.HitEndRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn AnimationGameplayTagRequest(
    txn: *dispatch.CombatRequestTxn(.AnimationGameplayTagRequest),
    scene: *Scene,
    query: GameplayTagQuery,
    events: *EventQueue,
    io: std.Io,
    alloc: mem.Alloc,
) !void {
    const entity_id = commonEntityId(txn.common) orelse {
        txn.respond(.{ .ErrorCode = .ErrEntityNotFound });
        return;
    };
    const item = query.byNetId(entity_id) orelse {
        txn.respond(.{ .ErrorCode = .ErrEntityNotFound });
        return;
    };

    const entity, const optional_tags, const fsm, const attribute, const buffs, const logic_state, const parts = item;
    const tags = optional_tags orelse ensureTagComponent(scene, entity) orelse {
        txn.respond(.{ .ErrorCode = .ErrMonsterNotGameplayTagComp });
        return;
    };
    try applyGameplayTagChange(
        scene,
        entity,
        tags,
        fsm,
        attribute,
        buffs,
        logic_state,
        parts,
        txn.payload.AddTagIds,
        txn.payload.RemoveTagIds,
        events,
        io,
        alloc,
        txn.receive_data_pack,
    );
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn AnimationGameplayTagPush(
    push: pb.AnimationGameplayTagPush,
    common: ?pb.CombatCommon,
    scene: *Scene,
    query: GameplayTagQuery,
    events: *EventQueue,
    io: std.Io,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    const entity_id = commonEntityId(common) orelse return;
    const item = query.byNetId(entity_id) orelse return;
    const entity, const optional_tags, const fsm, const attribute, const buffs, const logic_state, const parts = item;
    const tags = optional_tags orelse ensureTagComponent(scene, entity) orelse return;

    try applyGameplayTagChange(
        scene,
        entity,
        tags,
        fsm,
        attribute,
        buffs,
        logic_state,
        parts,
        push.AddTagIds,
        push.RemoveTagIds,
        events,
        io,
        alloc,
        receive_data_pack,
    );
}

pub fn RoleTagChangePush(
    push: pb.RoleTagChangePush,
    common: ?pb.CombatCommon,
    scene: *Scene,
    query: GameplayTagQuery,
    events: *EventQueue,
    io: std.Io,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    const entity_id = commonEntityId(common) orelse return;
    if (push.TagId == 0) return;
    const item = query.byNetId(entity_id) orelse return;
    const entity, const optional_tags, const fsm, const attribute, const buffs, const logic_state, const parts = item;
    const tags = optional_tags orelse ensureTagComponent(scene, entity) orelse return;
    const prev_present = tags.gameplayTagCount(push.TagId) > 0;
    try tags.setGameplayTagCount(alloc.gpa, push.TagId, push.TagCount);
    const next_present = push.TagCount > 0;
    if (prev_present != next_present) {
        try appendAnimationGameplayTagNotify(receive_data_pack, alloc.arena, entity.net_id, push.TagId, next_present);
    }
    try notifyFsmAfterTagChange(scene, entity, tags, fsm, attribute, buffs, logic_state, parts, events, io, alloc, receive_data_pack);
}

pub fn ANStartPush(_: pb.ANStartPush) !void {}

pub fn MontagePlayPush(_: pb.MontagePlayPush) !void {}

pub fn ChangeStateRequest(
    txn: *dispatch.CombatRequestTxn(.ChangeStateRequest),
    scene: *Scene,
    query: FsmQuery,
    config_query: EntityConfigQuery,
    qctx: QuestEngine.QuestContext,
    events: *EventQueue,
    io: std.Io,
    alloc: mem.Alloc,
) !void {
    const entity_id = commonEntityId(txn.common) orelse {
        txn.respond(.{
            .FsmId = txn.payload.FsmId,
            .Error = try errorResult(alloc.arena, .ErrEntityNotFound, &.{}),
            .CurrentState = 0,
        });
        return;
    };
    const item = query.byNetId(entity_id) orelse {
        txn.respond(.{
            .FsmId = txn.payload.FsmId,
            .Error = try errorResult(alloc.arena, .ErrEntityNotFound, &.{}),
            .CurrentState = 0,
        });
        return;
    };

    const entity, const fsm, const attribute, const buffs, const logic_state, const tags, const parts = item;
    const now_ms = queryNow(io);
    try fsm.initRuntime(alloc.gpa, now_ms);
    var preserve_event_for_followup = false;
    defer if (preserve_event_for_followup)
        fsm.finishTickPreservingEvent(now_ms)
    else
        fsm.finishTick(now_ms);

    var current_state = fsm.currentState(txn.payload.FsmId) orelse 0;
    var response_error: pb.DErrorResult = successResult();
    const ctx: Entity.FsmComponent.EvalContext = .{
        .attribute = attribute,
        .buffs = buffs,
        .logic_state = logic_state,
        .tags = tags,
        .parts = if (parts) |part| part.states() else null,
        .now_ms = now_ms,
    };

    var enqueue_with_recheck = true;
    switch (try fsm.confirmStateRequest(
        txn.payload.FsmId,
        txn.payload.FromState,
        txn.payload.ToState,
        alloc.gpa,
        ctx,
    )) {
        .confirmed, .accepted => {
            current_state = fsm.currentState(txn.payload.FsmId) orelse current_state;
            notifyQuestFsmStateChanged(qctx, config_query, entity.net_id, fsm, current_state);
            const lifecycle_deferred = try FsmLifecycle.enqueueEffects(entity, fsm, events, alloc, now_ms);
            try fsm.appendBlackboardNotify(entity.net_id, alloc.arena, txn.receive_data_pack);
            if (!lifecycle_deferred and fsm.anyRootDirty()) {
                try scene.queueFsmSync(alloc.gpa, entity.net_id);
                preserve_event_for_followup = true;
            }
        },
        .mismatch => |pending_state| {
            current_state = pending_state;
            response_error = try errorResult(alloc.arena, .ErrIentityFsmActionNotMatchState, &.{
                txn.payload.FsmId,
                txn.payload.ToState,
                pending_state,
            });
        },
        .machine_not_found => response_error = try errorResult(alloc.arena, .ErrEntityFsmMachineNotExist, &.{txn.payload.FsmId}),
        .invalid_source => response_error = try errorResult(alloc.arena, .ErrEntityFsmStateIncorrect, &.{
            txn.payload.FsmId,
            txn.payload.FromState,
            current_state,
        }),
        .invalid_target => response_error = try errorResult(alloc.arena, .ErrIentityFsmTransitToState, &.{
            txn.payload.FsmId,
            txn.payload.ToState,
        }),
        .no_pending => {
            if (try fsm.checkAndConfirm(entity.net_id, txn.payload.FsmId, alloc.gpa, ctx)) |notify| {
                try fsm.appendBlackboardNotify(entity.net_id, alloc.arena, txn.receive_data_pack);
                try txn.receive_data_pack.append(alloc.arena, notify);
                enqueue_with_recheck = false;
                if (!try FsmLifecycle.enqueueEffectsWithoutRecheck(entity, fsm, events, alloc, now_ms)) {
                    fsm.clearDirtyReason(Entity.FsmComponent.WakeReason.initial);
                    if (fsm.anyRootDirty()) {
                        try scene.queueFsmSync(alloc.gpa, entity.net_id);
                        preserve_event_for_followup = true;
                    }
                }
                if (extractChangeStateNotify(notify)) |transition| {
                    current_state = transition.ToState;
                    notifyQuestFsmStateChanged(qctx, config_query, entity.net_id, fsm, current_state);
                    response_error = try errorResult(alloc.arena, .ErrIentityFsmConfirmNotWait, &.{
                        transition.FsmId,
                        transition.FromState,
                        transition.ToState,
                    });
                } else {
                    current_state = fsm.currentState(txn.payload.FsmId) orelse current_state;
                    notifyQuestFsmStateChanged(qctx, config_query, entity.net_id, fsm, current_state);
                    response_error = try errorResult(alloc.arena, .ErrIentityFsmConfirmNotWait, &.{
                        txn.payload.FsmId,
                        txn.payload.FromState,
                        current_state,
                    });
                }
            } else {
                current_state = fsm.currentState(txn.payload.FsmId) orelse current_state;
                response_error = try errorResult(alloc.arena, .ErrIentityFsmConfirmNotExist, &.{
                    txn.payload.FsmId,
                    current_state,
                });
            }
        },
    }
    if (enqueue_with_recheck) {
        _ = try FsmLifecycle.enqueueEffects(entity, fsm, events, alloc, now_ms);
    }
    try scene.syncFsmDeadlines(alloc.gpa, entity.net_id, fsm);

    txn.respond(.{
        .FsmId = txn.payload.FsmId,
        .Error = response_error,
        .CurrentState = current_state,
    });
}

pub fn ChangeStateConfirmRequest(
    txn: *dispatch.CombatRequestTxn(.ChangeStateConfirmRequest),
    scene: *Scene,
    query: FsmQuery,
    config_query: EntityConfigQuery,
    qctx: QuestEngine.QuestContext,
    events: *EventQueue,
    io: std.Io,
    alloc: mem.Alloc,
) !void {
    const entity_id = commonEntityId(txn.common) orelse {
        txn.respond(.{
            .FsmId = txn.payload.FsmId,
            .State = 0,
            .Error = try errorResult(alloc.arena, .ErrEntityNotFound, &.{}),
        });
        return;
    };
    const item = query.byNetId(entity_id) orelse {
        txn.respond(.{
            .FsmId = txn.payload.FsmId,
            .State = 0,
            .Error = try errorResult(alloc.arena, .ErrEntityNotFound, &.{}),
        });
        return;
    };

    const entity, const fsm, const attribute, const buffs, const logic_state, const tags, const parts = item;
    const now_ms = queryNow(io);
    try fsm.initRuntime(alloc.gpa, now_ms);
    defer fsm.finishTick(now_ms);

    var response_state = fsm.currentState(txn.payload.FsmId) orelse 0;
    var response_error: pb.DErrorResult = successResult();

    switch (try fsm.confirmPending(txn.payload.FsmId, txn.payload.State, alloc.gpa, now_ms)) {
        .confirmed => {
            response_state = fsm.currentState(txn.payload.FsmId) orelse response_state;
            notifyQuestFsmStateChanged(qctx, config_query, entity.net_id, fsm, response_state);
            if (!try FsmLifecycle.enqueueEffects(entity, fsm, events, alloc, now_ms)) {
                try fsm.appendBlackboardNotify(entity.net_id, alloc.arena, txn.receive_data_pack);
                try appendFollowupTransition(entity, fsm, txn.payload.FsmId, attribute, buffs, logic_state, tags, parts, now_ms, alloc, txn.receive_data_pack);
            }
        },
        .mismatch => |pending_state| {
            response_state = pending_state;
            response_error = try errorResult(alloc.arena, .ErrIentityFsmActionNotMatchState, &.{
                txn.payload.FsmId,
                txn.payload.State,
                pending_state,
            });
        },
        .machine_not_found => response_error = try errorResult(alloc.arena, .ErrEntityFsmMachineNotExist, &.{txn.payload.FsmId}),
        .invalid_target => response_error = try errorResult(alloc.arena, .ErrIentityFsmTransitToState, &.{
            txn.payload.FsmId,
            txn.payload.State,
        }),
        .no_pending => response_error = try errorResult(alloc.arena, .ErrIentityFsmConfirmNotExist, &.{
            txn.payload.FsmId,
            response_state,
        }),
        .accepted => response_state = fsm.currentState(txn.payload.FsmId) orelse response_state,
        .invalid_source => response_error = try errorResult(alloc.arena, .ErrEntityFsmStateIncorrect, &.{
            txn.payload.FsmId,
            response_state,
        }),
    }
    _ = try FsmLifecycle.enqueueEffects(entity, fsm, events, alloc, now_ms);
    try scene.syncFsmDeadlines(alloc.gpa, entity.net_id, fsm);

    txn.respond(.{
        .FsmId = txn.payload.FsmId,
        .State = response_state,
        .Error = response_error,
    });
}

pub fn FsmConditionPassRequest(
    txn: *dispatch.CombatRequestTxn(.FsmConditionPassRequest),
    scene: *Scene,
    query: FsmQuery,
    events: *EventQueue,
    io: std.Io,
    alloc: mem.Alloc,
) !void {
    const entity_id = commonEntityId(txn.common) orelse {
        txn.respond(.{
            .FsmId = txn.payload.FsmId,
            .Error = try errorResult(alloc.arena, .ErrEntityNotFound, &.{}),
        });
        return;
    };
    const item = query.byNetId(entity_id) orelse {
        txn.respond(.{
            .FsmId = txn.payload.FsmId,
            .Error = try errorResult(alloc.arena, .ErrEntityNotFound, &.{}),
        });
        return;
    };

    const entity, const fsm, const attribute, const buffs, const logic_state, const tags, const parts = item;
    const now_ms = queryNow(io);
    try fsm.initRuntime(alloc.gpa, now_ms);
    defer fsm.finishTick(now_ms);
    const pass_result = try fsm.recordClientPass(alloc.gpa, .{
        .fsm_id = txn.payload.FsmId,
        .from = txn.payload.FromState,
        .to = txn.payload.ToState,
        .index = txn.payload.ConditionIndex,
    }, txn.payload.Value);
    const response_error = try clientPassError(alloc.arena, pass_result, txn.payload);
    if (pass_result == .updated) {
        _ = fsm.markRootDirty(txn.payload.FsmId, Entity.FsmComponent.WakeReason.client_pass);
    }
    const lifecycle_deferred = try FsmLifecycle.enqueueEffects(entity, fsm, events, alloc, now_ms);

    if (pass_result == .updated and !lifecycle_deferred) {
        if (try fsm.checkTransitions(entity.net_id, txn.payload.FsmId, .{
            .attribute = attribute,
            .buffs = buffs,
            .logic_state = logic_state,
            .tags = tags,
            .parts = if (parts) |part| part.states() else null,
            .now_ms = now_ms,
        })) |notify| {
            try fsm.appendBlackboardNotify(entity.net_id, alloc.arena, txn.receive_data_pack);
            try txn.receive_data_pack.append(alloc.arena, notify);
        }
    }
    try scene.syncFsmDeadlines(alloc.gpa, entity.net_id, fsm);

    txn.respond(.{
        .FsmId = txn.payload.FsmId,
        .Error = response_error,
    });
}

pub fn FsmStateBehaviorRequest(
    txn: *dispatch.CombatRequestTxn(.FsmStateBehaviorRequest),
    query: FsmBehaviorQuery,
    io: std.Io,
    alloc: mem.Alloc,
) !void {
    const entity_id = commonEntityId(txn.common) orelse {
        txn.respond(.{
            .FsmId = txn.payload.FsmId,
            .State = 0,
            .ErrorCode = .ErrEntityNotFound,
        });
        return;
    };
    const item = query.byNetId(entity_id) orelse {
        txn.respond(.{
            .FsmId = txn.payload.FsmId,
            .State = 0,
            .ErrorCode = .ErrEntityNotFound,
        });
        return;
    };
    const optional_fsm = item[1];
    const fsm = optional_fsm orelse {
        txn.respond(.{
            .FsmId = txn.payload.FsmId,
            .State = 0,
            .ErrorCode = .ErrEntityFsmMachineNotExist,
        });
        return;
    };

    const now_ms = queryNow(io);
    try fsm.initRuntime(alloc.gpa, now_ms);
    const current_state = fsm.currentState(txn.payload.FsmId) orelse 0;
    const behavior_type = txn.payload.Type orelse {
        txn.respond(.{
            .FsmId = txn.payload.FsmId,
            .State = current_state,
            .ErrorCode = .ErrNotStateMachineBehavior,
        });
        return;
    };
    const validation = fsm.validateBehavior(
        txn.payload.FsmId,
        txn.payload.State,
        txn.payload.Index,
        @intFromEnum(behavior_type),
    );
    const error_code: pb.ErrorCode = switch (validation) {
        .valid => .Success,
        .machine_not_found => .ErrEntityFsmMachineNotExist,
        .invalid_state, .inactive_state => .ErrEntityFsmStateIncorrect,
        .invalid_behavior => .ErrNotStateMachineBehavior,
    };
    txn.respond(.{
        .FsmId = txn.payload.FsmId,
        .State = if (validation == .valid) txn.payload.State else current_state,
        .ErrorCode = error_code,
    });
}

pub fn FsmPlayMontageRequest(txn: *dispatch.CombatRequestTxn(.FsmPlayMontageRequest)) !void {
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn FsmPlayMontagePush(_: pb.FsmPlayMontagePush) !void {}

pub fn AiHateRequest(
    txn: *dispatch.CombatRequestTxn(.AiHateRequest),
    scene: *Scene,
    query: AiHateQuery,
    events: *EventQueue,
    io: std.Io,
    alloc: mem.Alloc,
) !void {
    if (commonEntityId(txn.common)) |entity_id| {
        if (query.byNetId(entity_id)) |item| {
            try applyAiHate(scene, item, txn.payload.HateList.items, events, queryNow(io), alloc, txn.receive_data_pack);
        }
    }

    txn.respond(.{ .ErrorCode = .Success });
}

pub fn LogicStateInitRequest(
    txn: *dispatch.CombatRequestTxn(.LogicStateInitRequest),
    scene: *Scene,
    query: LogicStateQuery,
    alloc: mem.Alloc,
) !void {
    _ = try updateLogicState(scene, query, txn.payload.EntityId, txn.payload.CombatCommon, txn.payload.InitData, alloc);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn SwitchLogicStateRequest(
    txn: *dispatch.CombatRequestTxn(.SwitchLogicStateRequest),
    scene: *Scene,
    query: LogicStateQuery,
    alloc: mem.Alloc,
    qctx: QuestEngine.QuestContext,
) !void {
    const target_id = commonEntityId(txn.common) orelse 0;
    const changed = try updateLogicState(scene, query, 0, txn.common, txn.payload.States, alloc);
    if (changed and target_id == scene.currentRoleEntityId()) {
        scene.has_player_motion = true;
        QuestEngine.onConditionStateChanged(qctx) catch |err| QuestEngine.logHookErr("SwitchLogicStateReq", err);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn FsmConditionPassPush(
    push: pb.FsmConditionPassPush,
    common: ?pb.CombatCommon,
    scene: *Scene,
    query: FsmQuery,
    events: *EventQueue,
    io: std.Io,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    const entity_id = commonEntityId(common) orelse return;
    if (query.byNetId(entity_id)) |item| {
        const entity, const fsm, const attribute, const buffs, const logic_state, const tags, const parts = item;
        const now_ms = queryNow(io);
        try fsm.initRuntime(alloc.gpa, now_ms);
        defer fsm.finishTick(now_ms);
        const pass_result = try fsm.recordClientPass(alloc.gpa, .{
            .fsm_id = push.FsmId,
            .from = push.FromState,
            .to = push.ToState,
            .index = push.ConditionIndex,
        }, push.Value);
        if (pass_result == .updated) {
            _ = fsm.markRootDirty(push.FsmId, Entity.FsmComponent.WakeReason.client_pass);
        }
        const lifecycle_deferred = try FsmLifecycle.enqueueEffects(entity, fsm, events, alloc, now_ms);
        if (pass_result == .updated and !lifecycle_deferred) {
            if (try fsm.checkTransitions(entity.net_id, push.FsmId, .{
                .attribute = attribute,
                .buffs = buffs,
                .logic_state = logic_state,
                .tags = tags,
                .parts = if (parts) |part| part.states() else null,
                .now_ms = now_ms,
            })) |notify| {
                try fsm.appendBlackboardNotify(entity.net_id, alloc.arena, receive_data_pack);
                try receive_data_pack.append(alloc.arena, notify);
            }
        }
        try scene.syncFsmDeadlines(alloc.gpa, entity.net_id, fsm);
    }
}

pub fn AiHatePush(
    push: pb.AiHatePush,
    common: ?pb.CombatCommon,
    scene: *Scene,
    query: AiHateQuery,
    events: *EventQueue,
    io: std.Io,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    const entity_id = commonEntityId(common) orelse return;
    if (query.byNetId(entity_id)) |item| {
        try applyAiHate(scene, item, push.HateList.items, events, queryNow(io), alloc, receive_data_pack);
    }
}

pub fn AiInformationRequest(
    txn: *dispatch.CombatRequestTxn(.AiInformationRequest),
    scene: *Scene,
    query: AiHateQuery,
    events: *EventQueue,
    io: std.Io,
    alloc: mem.Alloc,
) !void {
    if (commonEntityId(txn.common)) |entity_id| {
        if (txn.payload.AiInfo) |ai_info| {
            if (query.byNetId(entity_id)) |item| {
                try applyAiHate(scene, item, ai_info.HateList.items, events, queryNow(io), alloc, txn.receive_data_pack);
            }
        }
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn AiInformationPush(
    push: pb.AiInformationPush,
    common: ?pb.CombatCommon,
    scene: *Scene,
    query: AiHateQuery,
    events: *EventQueue,
    io: std.Io,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    const entity_id = commonEntityId(common) orelse return;
    const ai_info = push.AiInfo orelse return;
    if (query.byNetId(entity_id)) |item| {
        try applyAiHate(scene, item, ai_info.HateList.items, events, queryNow(io), alloc, receive_data_pack);
    }
}

pub fn BattleStateChangeRequest(
    txn: *dispatch.CombatRequestTxn(.BattleStateChangeRequest),
    scene: *Scene,
    alloc: mem.Alloc,
) !void {
    const req = txn.payload;
    const eid = if (req.EntityId != 0) req.EntityId else commonEntityId(txn.common) orelse 0;
    try applyBattleStateChange(scene, eid, req.InBattle, alloc, txn.receive_data_pack);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn BattleStateChangePush(
    push: pb.BattleStateChangePush,
    common: ?pb.CombatCommon,
    scene: *Scene,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    const eid = if (push.EntityId != 0) push.EntityId else commonEntityId(common) orelse 0;
    try applyBattleStateChange(scene, eid, push.InBattle, alloc, receive_data_pack);
}

fn applyBattleStateChange(
    scene: *Scene,
    entity_id: i64,
    in_battle: bool,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    if (entity_id == 0) return;
    try scene.setBattleEntityActive(alloc.gpa, entity_id, in_battle);
    try receive_data_pack.append(alloc.arena, .{ .Message = .{
        .CombatNotifyData = .{
            .CombatCommon = .{ .EntityId = entity_id },
            .Message = .{ .BattleStateChangeNotify = .{
                .EntityId = entity_id,
                .InBattle = in_battle,
            } },
        },
    } });
    _ = try scene.appendBattleStateNotify(alloc.arena, receive_data_pack);
}

pub fn ChangeStateConfirmPush(
    push: pb.ChangeStateConfirmPush,
    common: ?pb.CombatCommon,
    scene: *Scene,
    query: FsmQuery,
    config_query: EntityConfigQuery,
    qctx: QuestEngine.QuestContext,
    events: *EventQueue,
    io: std.Io,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    const entity_id = commonEntityId(common) orelse return;
    if (query.byNetId(entity_id)) |item| {
        const entity, const fsm, const attribute, const buffs, const logic_state, const tags, const parts = item;
        const now_ms = queryNow(io);
        try fsm.initRuntime(alloc.gpa, now_ms);
        defer fsm.finishTick(now_ms);
        switch (try fsm.confirmPending(push.FsmId, push.State, alloc.gpa, now_ms)) {
            .confirmed => {
                const confirmed_state = fsm.currentState(push.FsmId) orelse push.State;
                notifyQuestFsmStateChanged(qctx, config_query, entity.net_id, fsm, confirmed_state);
                if (!try FsmLifecycle.enqueueEffects(entity, fsm, events, alloc, now_ms)) {
                    try fsm.appendBlackboardNotify(entity.net_id, alloc.arena, receive_data_pack);
                    try appendFollowupTransition(entity, fsm, push.FsmId, attribute, buffs, logic_state, tags, parts, now_ms, alloc, receive_data_pack);
                }
            },
            .accepted, .machine_not_found, .invalid_source, .invalid_target, .mismatch, .no_pending => {},
        }
        _ = try FsmLifecycle.enqueueEffects(entity, fsm, events, alloc, now_ms);
        try scene.syncFsmDeadlines(alloc.gpa, entity.net_id, fsm);
    }
}

pub fn LogicStateInitPush(
    push: pb.LogicStateInitPush,
    common: ?pb.CombatCommon,
    scene: *Scene,
    query: LogicStateQuery,
    alloc: mem.Alloc,
) !void {
    _ = try updateLogicState(scene, query, push.EntityId, push.CombatCommon orelse common, push.InitData, alloc);
}

pub fn SwitchLogicStatePush(
    push: pb.SwitchLogicStatePush,
    common: ?pb.CombatCommon,
    scene: *Scene,
    query: LogicStateQuery,
    alloc: mem.Alloc,
    qctx: QuestEngine.QuestContext,
) !void {
    const target_id = commonEntityId(common) orelse 0;
    const changed = try updateLogicState(scene, query, 0, common, push.States, alloc);
    if (changed and target_id == scene.currentRoleEntityId()) {
        scene.has_player_motion = true;
        QuestEngine.onConditionStateChanged(qctx) catch |err| QuestEngine.logHookErr("SwitchLogicState", err);
    }
}

fn updateLogicState(
    scene: *Scene,
    query: LogicStateQuery,
    entity_id: i64,
    common: ?pb.CombatCommon,
    state_data: ?pb.LogicStateComponentPb,
    alloc: mem.Alloc,
) !bool {
    const target_id = if (entity_id != 0) entity_id else commonEntityId(common) orelse return false;
    const data = state_data orelse return false;
    if (query.byNetId(target_id)) |item| {
        _, const logic_state = item;
        const changed = logic_state.position_state != data.PositionState or
            logic_state.move_state != data.MoveState or
            logic_state.direction_state != data.DirectionState or
            logic_state.position_sub_state != data.PositionSubState;
        logic_state.position_state = data.PositionState;
        logic_state.move_state = data.MoveState;
        logic_state.direction_state = data.DirectionState;
        logic_state.position_sub_state = data.PositionSubState;
        if (changed) try scene.markFsmDirty(alloc.gpa, target_id, Entity.FsmComponent.WakeReason.position);
        return changed;
    }
    return false;
}

fn appendFollowupTransition(
    entity: Entity,
    fsm: *Entity.FsmComponent,
    fsm_id: i32,
    attribute: ?*Entity.AttributeComponent,
    buffs: ?*Entity.FightBuffComponent,
    logic_state: ?*Entity.LogicStateComponent,
    tags: ?*Entity.TagComponent,
    parts: ?*Entity.PartComponent,
    now_ms: i64,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    const ctx: Entity.FsmComponent.EvalContext = .{
        .attribute = attribute,
        .buffs = buffs,
        .logic_state = logic_state,
        .tags = tags,
        .parts = if (parts) |part| part.states() else null,
        .now_ms = now_ms,
    };
    if (try fsm.checkTransitions(entity.net_id, fsm_id, ctx)) |notify| {
        try fsm.appendBlackboardNotify(entity.net_id, alloc.arena, receive_data_pack);
        try receive_data_pack.append(alloc.arena, notify);
    }
    try fsm.appendDirtyStateTransitions(entity.net_id, alloc.arena, receive_data_pack, ctx);
}

fn applyAiHate(
    scene: *Scene,
    item: AiHateQuery.Item,
    hate_list: []const pb.AiHateEntity,
    events: *EventQueue,
    now_ms: i64,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    const entity, const optional_fsm, const attribute, const buffs, const logic_state, const tags, const parts = item;

    try scene.updateBattleEntityFromHate(alloc.gpa, entity.net_id, hate_list);
    _ = try scene.appendBattleStateNotify(alloc.arena, receive_data_pack);

    var has_target = false;
    for (hate_list) |entry| {
        if (entry.HatredValue >= 1) {
            has_target = true;
            break;
        }
    }
    _ = try scene.updateFsmHateTarget(alloc.gpa, entity.net_id, has_target);

    const fsm = optional_fsm orelse return;
    try fsm.initRuntime(alloc.gpa, now_ms);
    defer fsm.finishTick(now_ms);
    if (!try FsmLifecycle.enqueueEffects(entity, fsm, events, alloc, now_ms)) {
        try fsm.appendDirtyStateTransitions(entity.net_id, alloc.arena, receive_data_pack, .{
            .attribute = attribute,
            .buffs = buffs,
            .logic_state = logic_state,
            .tags = tags,
            .parts = if (parts) |part| part.states() else null,
            .now_ms = now_ms,
        });
    }
    try scene.syncFsmDeadlines(alloc.gpa, entity.net_id, fsm);
}

fn applyGameplayTagChange(
    scene: *Scene,
    entity: Entity,
    tags: *Entity.TagComponent,
    optional_fsm: ?*Entity.FsmComponent,
    attribute: ?*Entity.AttributeComponent,
    buffs: ?*Entity.FightBuffComponent,
    logic_state: ?*Entity.LogicStateComponent,
    parts: ?*Entity.PartComponent,
    tag_id: i32,
    add: bool,
    events: *EventQueue,
    io: std.Io,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    if (tag_id == 0) return;
    const changed = try tags.adjustGameplayTagCount(alloc.gpa, tag_id, if (add) 1 else -1);
    if (!changed) return;

    try appendAnimationGameplayTagNotify(receive_data_pack, alloc.arena, entity.net_id, tag_id, add);
    try notifyFsmAfterTagChange(scene, entity, tags, optional_fsm, attribute, buffs, logic_state, parts, events, io, alloc, receive_data_pack);
}

fn appendAnimationGameplayTagNotify(
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
    arena: std.mem.Allocator,
    entity_id: i64,
    tag_id: i32,
    add: bool,
) !void {
    // Generated names are stale: AddTagIds is the tag id, RemoveTagIds is the add/remove flag.
    try receive_data_pack.append(arena, .{ .Message = .{
        .CombatNotifyData = .{
            .CombatCommon = .{ .EntityId = entity_id },
            .Message = .{
                .AnimationGameplayTagNotify = .{
                    .AddTagIds = tag_id,
                    .RemoveTagIds = add,
                },
            },
        },
    } });
}

fn ensureTagComponent(scene: *Scene, entity: Entity) ?*Entity.TagComponent {
    if (entity.index >= scene.entities.len) return null;
    if (scene.entities.items(.entity_id)[entity.index].net_id != entity.net_id) return null;
    const tag_slot = &scene.entities.items(.tag)[entity.index];
    if (tag_slot.* == null) {
        tag_slot.* = .{};
    }
    return if (tag_slot.*) |*comp| comp else null;
}

fn extractChangeStateNotify(notify: pb.CombatReceiveData) ?pb.ChangeStateNotify {
    const outer = notify.Message orelse return null;
    switch (outer) {
        .CombatNotifyData => |maybe_combat| {
            const combat = maybe_combat orelse return null;
            const inner = combat.Message orelse return null;
            switch (inner) {
                .ChangeStateNotify => |change| return change,
                else => return null,
            }
        },
        else => return null,
    }
}

fn notifyFsmAfterTagChange(
    scene: *Scene,
    entity: Entity,
    tags: *Entity.TagComponent,
    optional_fsm: ?*Entity.FsmComponent,
    attribute: ?*Entity.AttributeComponent,
    buffs: ?*Entity.FightBuffComponent,
    logic_state: ?*Entity.LogicStateComponent,
    parts: ?*Entity.PartComponent,
    events: *EventQueue,
    io: std.Io,
    alloc: mem.Alloc,
    receive_data_pack: *std.ArrayList(pb.CombatReceiveData),
) !void {
    const fsm = optional_fsm orelse return;
    const now_ms = queryNow(io);
    try fsm.initRuntime(alloc.gpa, now_ms);
    defer fsm.finishTick(now_ms);
    _ = fsm.markDirty(Entity.FsmComponent.WakeReason.tag);

    if (!try FsmLifecycle.enqueueEffects(entity, fsm, events, alloc, now_ms)) {
        try fsm.appendDirtyStateTransitions(entity.net_id, alloc.arena, receive_data_pack, .{
            .attribute = attribute,
            .buffs = buffs,
            .logic_state = logic_state,
            .tags = tags,
            .parts = if (parts) |part| part.states() else null,
            .now_ms = now_ms,
        });
    }
    try scene.syncFsmDeadlines(alloc.gpa, entity.net_id, fsm);
}

fn notifyQuestFsmStateChanged(
    qctx: QuestEngine.QuestContext,
    config_query: EntityConfigQuery,
    entity_net_id: i64,
    fsm: *const Entity.FsmComponent,
    state_id: i32,
) void {
    if (config_query.byNetId(entity_net_id)) |item| {
        _, const cfg = item;
        const state_name = if (fsm.graph.findNode(state_id)) |node| (node.Name orelse "") else "";
        QuestEngine.onEntityStateChanged(qctx, cfg.config_id, state_name) catch |err| QuestEngine.logHookErr("onEntityStateChanged", err);
    }
}

fn queryNow(io: std.Io) i64 {
    const rtc: std.Io.Clock = .awake;
    return rtc.now(io).toMilliseconds();
}

fn commonEntityId(common: ?pb.CombatCommon) ?i64 {
    const value = common orelse return null;
    if (value.EntityId == 0) return null;
    return value.EntityId;
}

fn successResult() pb.DErrorResult {
    return .{ .ErrorCode = .Success };
}

fn clientPassError(
    arena: std.mem.Allocator,
    result: Entity.FsmComponent.ClientPassResult,
    payload: pb.FsmConditionPassRequest,
) !pb.DErrorResult {
    return switch (result) {
        .updated => successResult(),
        .machine_not_found => try errorResult(arena, .ErrEntityFsmMachineNotExist, &.{payload.FsmId}),
        .invalid_source => try errorResult(arena, .ErrEntityFsmStateIncorrect, &.{
            payload.FsmId,
            payload.FromState,
        }),
        .invalid_target => try errorResult(arena, .ErrIentityFsmTransitToState, &.{
            payload.FsmId,
            payload.ToState,
        }),
        .inactive_source,
        .transition_not_found,
        .condition_not_found,
        .condition_not_client,
        .condition_context_invalid,
        => try errorResult(
            arena,
            .ErrIentityFsmActionNotMatchState,
            &.{ payload.FsmId, payload.FromState, payload.ToState, payload.ConditionIndex },
        ),
    };
}

fn errorResult(arena: std.mem.Allocator, code: pb.ErrorCode, params: []const i32) !pb.DErrorResult {
    var error_params: std.ArrayList([]const u8) = .empty;
    for (params) |param| {
        try error_params.append(arena, try std.fmt.allocPrint(arena, "{d}", .{param}));
    }

    return .{
        .ErrorCode = code,
        .ErrorParams = error_params,
    };
}
