const std = @import("std");
const pb = @import("proto").pb;
const Assets = @import("../../data/Assets.zig");
const mem = @import("../../mem.zig");
const Connection = @import("../../network/Connection.zig");
const InventoryInfo = @import("../../fs/InventoryInfo.zig");
const LastSceneInfo = @import("../../fs/LastSceneInfo.zig");
const QuestInfo = @import("../../fs/QuestInfo.zig");
const MailInfo = @import("../../fs/MailInfo.zig");
const comp_util = @import("../component/comp_util.zig");
const Scene = @import("../Scene.zig");
const Events = @import("../events.zig");
const ConditionEvaluator = @import("condition.zig");
const gameplay_tags = @import("gameplay_tags.zig");
const js_patches = @import("../commands/js_patches.zig");
const prologue_quest = @import("prologue_quest.zig");
const RewardService = @import("reward.zig");
const quest_types = @import("quest_types.zig");
const TrialRoles = @import("trial_roles.zig");
const activity_handler = @import("../../network/handlers/activity.zig");
const map_handler = @import("../../network/handlers/map.zig");
const spline_state = @import("spline_state.zig");
const MoraleHelper = @import("morale.zig");
const GravityHelper = @import("gravity.zig");
const tags_handler = @import("../handlers/tags.zig");

const log = std.log.scoped(.quest_actions);

pub const QuestContext = quest_types.QuestContext;
pub const ActionCallbacks = quest_types.ActionCallbacks;

pub fn executeActionList(
    ctx: QuestContext,
    callbacks: ActionCallbacks,
    quest_id: i32,
    node_id: i32,
    actions: []const std.json.Value,
) anyerror!void {
    for (actions) |act| {
        try executeAction(ctx, callbacks, quest_id, node_id, act);
    }
}

pub fn executeAction(
    ctx: QuestContext,
    callbacks: ActionCallbacks,
    quest_id: i32,
    node_id: i32,
    act: std.json.Value,
) anyerror!void {
    if (act != .object) return;
    const name = quest_types.jsonStr(act.object.get("Name")) orelse return;
    const params_val = act.object.get("Params");
    const params: std.json.ObjectMap = if (params_val) |pv| switch (pv) {
        .object => |obj| obj,
        else => .empty,
    } else .empty;

    if (std.mem.eql(u8, name, "AwakeEntity")) {
        try applyAwakeEntity(ctx, params);
    } else if (std.mem.eql(u8, name, "DestroyEntity")) {
        try applyDestroyEntity(ctx, params);
    } else if (std.mem.eql(u8, name, "ChangeEntityState")) {
        try applyChangeEntityState(ctx, callbacks, params);
    } else if (std.mem.eql(u8, name, "ChangeSelfEntityState")) {
        try applyChangeSelfEntityState(ctx, callbacks, params);
    } else if (std.mem.eql(u8, name, "SetEntityVisible")) {
        try applySetEntityVisible(ctx, params);
    } else if (std.mem.eql(u8, name, "HideSpecificEntities")) {
        try applyEntityVisibilityList(ctx, params.get("EntityIds"), false);
    } else if (std.mem.eql(u8, name, "ShowSpecificEntities")) {
        try applyEntityVisibilityList(ctx, params.get("EntityIds"), true);
    } else if (std.mem.eql(u8, name, "HideGroup")) {
        try applyHideGroup(ctx, params);
    } else if (std.mem.eql(u8, name, "ShowHidedGroup")) {
        try applyShowHidedGroup(ctx, params);
    } else if (std.mem.eql(u8, name, "HideTargetRange")) {
        try applyHideTargetRange(ctx, params);
    } else if (std.mem.eql(u8, name, "ShowTargetRange")) {
        try applyShowTargetRange(ctx, params);
    } else if (std.mem.eql(u8, name, "HideByRangeInFlow")) {
        try applyHideByRangeInFlow(ctx, params);
    } else if (std.mem.eql(u8, name, "ShowAllHidedGroupInFlow")) {
        try restoreAllHiddenGroups(ctx);
    } else if (std.mem.eql(u8, name, "LockEntity")) {
        try applyLockEntities(ctx, params.get("EntityIds"), true);
    } else if (std.mem.eql(u8, name, "UnlockEntity")) {
        try applyLockEntities(ctx, params.get("EntityIds"), false);
    } else if (std.mem.eql(u8, name, "SwitchDataLayers")) {
        const load_layers = try quest_types.parseIntList(ctx.alloc.arena, params.get("LoadDataLayers"));
        const unload_layers = try quest_types.parseIntList(ctx.alloc.arena, params.get("UnloadDataLayers"));
        try applyDataLayerChange(ctx, load_layers, unload_layers);
    } else if (std.mem.eql(u8, name, "SwitchSubLevels")) {
        const switch_ty = quest_types.jsonStr(params.get("Type")) orelse "Directly";
        if (!std.mem.eql(u8, switch_ty, "Preload")) {
            const loads = try quest_types.parseStrList(ctx.alloc.arena, params.get("LoadLevels"));
            const unloads = try quest_types.parseStrList(ctx.alloc.arena, params.get("UnloadLevels"));
            const hides = try quest_types.parseStrList(ctx.alloc.arena, params.get("HideLevels"));
            const tp_eid = quest_types.jsonInt(params.get("TeleportEntityId")) orelse 0;
            try applySubLevelChange(ctx, loads, unloads, hides, tp_eid);
        }
    } else if (std.mem.eql(u8, name, "AddTrialCharacter")) {
        try applyAddTrialCharacter(ctx, params);
    } else if (std.mem.eql(u8, name, "RemoveTrialCharacter")) {
        try applyRemoveTrialCharacter(ctx, params);
    } else if (std.mem.eql(u8, name, "AddBuffToPlayer")) {
        try applyQuestBuffAction(ctx, params, true, false);
    } else if (std.mem.eql(u8, name, "RemoveBuffFromPlayer")) {
        try applyQuestBuffAction(ctx, params, true, true);
    } else if (std.mem.eql(u8, name, "AddBuffToEntity")) {
        try applyQuestBuffAction(ctx, params, false, false);
    } else if (std.mem.eql(u8, name, "RemoveBuffFromEntity")) {
        try applyQuestBuffAction(ctx, params, false, true);
    } else if (std.mem.eql(u8, name, "PlayFlow")) {
        if (callbacks.start_flow) |cb| {
            try cb(ctx, quest_id, node_id, params, true);
        }
    } else if (std.mem.eql(u8, name, "ChangeState")) {
        if (ctx.scene) |scene| {
            if (quest_types.jsonInt(params.get("StateId"))) |sid| {
                if (callbacks.defer_flow_transition) {
                    scene.flow_next_state_id = sid;
                } else if (callbacks.start_flow_state) |cb| {
                    if (scene.active_flow_list_name.len != 0) {
                        const qid = scene.active_flow_quest_id;
                        const nid = scene.active_flow_node_id;
                        const fln = try ctx.alloc.arena.dupe(u8, scene.active_flow_list_name);
                        const fid = scene.active_flow_id;
                        scene.clearActiveFlow(ctx.alloc.gpa);
                        try cb(ctx, qid, nid, fln, fid, sid, false);
                    }
                }
            }
        }
    } else if (std.mem.eql(u8, name, "FinishState")) {
        if (callbacks.on_finish_flow_state) |cb| try cb(ctx);
    } else if (std.mem.eql(u8, name, "AccpetCurrentQuest")) {
        if (callbacks.on_accept_focus_wait) |cb| try cb(ctx);
    } else if (std.mem.eql(u8, name, "TraceCurrentQuest")) {
        if (callbacks.on_trace_current_quest) |cb| try cb(ctx, quest_id);
    } else if (std.mem.eql(u8, name, "SetPlayerOperationRestriction")) {
        // The decompiled LevelEventSetPlayerOperation inherits the empty
        // LevelEventBase.ExecuteNew implementation. Operation restrictions
        // are applied by a separate LevelFlow action, not this quest event.
    } else if (std.mem.eql(u8, name, "DestroyQuest")) {
        if (quest_types.jsonInt(params.get("QuestId"))) |target_qid| {
            if (callbacks.on_destroy_quest) |cb| try cb(ctx, target_qid);
        }
    } else if (std.mem.eql(u8, name, "GetItem")) {
        try grantActionItems(ctx, callbacks, params.get("Items"));
    } else if (std.mem.eql(u8, name, "DestroyQuestItem")) {
        try destroyQuestItems(ctx, params);
    } else if (std.mem.eql(u8, name, "UnlockTeleportTrigger")) {
        if (quest_types.jsonInt(params.get("TeleportId"))) |tp_id| {
            try unlockTeleporter(ctx, tp_id);
        }
    } else if (std.mem.eql(u8, name, "OpenSystemFunction")) {
        if (quest_types.jsonInt(params.get("FunctionId"))) |func_id| {
            if (try ctx.quest_comp.info.unlockSystem(ctx.alloc.gpa, func_id)) {
                try pushFuncOpen(ctx);
            }
        }
    } else if (std.mem.eql(u8, name, "EnableSystem")) {
        try applyEnableSystemAction(ctx, params);
    } else if (std.mem.eql(u8, name, "EnableFunction")) {
        try applyEnableFunctionAction(ctx, params);
    } else if (std.mem.eql(u8, name, "UnlockSystemItem")) {
        if (params.get("SystemOption")) |opt_val| {
            if (opt_val == .object) {
                const sys_type = quest_types.jsonStr(opt_val.object.get("Type")) orelse "";
                const uopt_obj: ?std.json.ObjectMap = if (opt_val.object.get("UnlockOption")) |uopt|
                    (if (uopt == .object) uopt.object else null)
                else
                    null;
                const item_id = if (uopt_obj) |u| quest_types.jsonInt(u.get("Id")) else quest_types.jsonInt(opt_val.object.get("Id"));
                if (item_id) |id| {
                    if (id > 0) {
                        _ = try ctx.quest_comp.info.unlockSystem(ctx.alloc.gpa, id);
                        try pushFuncOpen(ctx);
                        if (std.mem.eql(u8, sys_type, "AtlasSystem")) {
                            const utype = if (uopt_obj) |u| (quest_types.jsonStr(u.get("Type")) orelse "") else "";
                            const ill_type: ?pb.IllustratedType = if (std.mem.eql(u8, utype, "PlotPhoto"))
                                .Photograph
                            else if (std.mem.eql(u8, utype, "Noun"))
                                .Noun
                            else
                                null;
                            if (ill_type) |it| {
                                ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.IllustratedUnlockNotify{
                                    .IsNew = true,
                                    .Type = it,
                                    .Entry = .{ .Id = id, .Num = 1, .IsRead = false },
                                }) catch {};
                            }
                        }
                    }
                }
            }
        }
    } else if (std.mem.eql(u8, name, "EnableLevelPlay")) {
        try applyEnableLevelPlay(ctx, callbacks, params);
    } else if (std.mem.eql(u8, name, "SetWeather")) {
        if (quest_types.jsonInt(params.get("WeatherId"))) |weather_id| {
            try applyWeatherChange(ctx, callbacks, weather_id);
        }
    } else if (std.mem.eql(u8, name, "SetWeatherLockState")) {
        const lock_str = quest_types.jsonStr(params.get("LockState")) orelse "";
        ctx.quest_comp.info.env.weather_locked = std.mem.eql(u8, lock_str, "Lock");
    } else if (std.mem.eql(u8, name, "SetAreaState")) {
        const area_id = quest_types.jsonInt(params.get("AreaId")) orelse return;
        const state_val = quest_types.jsonInt(params.get("State")) orelse if (quest_types.jsonBool(params.get("State")) orelse false) @as(i32, 1) else 0;
        try applyAreaStateChange(ctx, area_id, state_val != 0);
    } else if (std.mem.eql(u8, name, "SetTime")) {
        const hour = quest_types.jsonInt(params.get("Hour")) orelse 8;
        const min = quest_types.jsonInt(params.get("Min")) orelse 0;
        const show_ui = quest_types.jsonBool(params.get("ShowUi")) orelse false;
        try applySetTime(ctx, callbacks, hour, min, show_ui);
    } else if (std.mem.eql(u8, name, "SetTimeLockState")) {
        const lock_str = quest_types.jsonStr(params.get("LockState")) orelse "Unlock";
        const locked = std.mem.eql(u8, lock_str, "Lock");
        try applyTimeLockState(ctx, locked, quest_types.jsonInt(params.get("Hour")), quest_types.jsonInt(params.get("Min")));
    } else if (std.mem.eql(u8, name, "SetAreaTimeState")) {
        try applyAreaTimeState(ctx, params);
    } else if (std.mem.eql(u8, name, "ToggleMapMarkState")) {
        if (quest_types.jsonInt(params.get("MarkId"))) |mark_id| {
            const action_type = quest_types.jsonStr(params.get("Type")) orelse "Show";
            try toggleMapMark(ctx, mark_id, action_type);
        }
    } else if (std.mem.eql(u8, name, "PhoneSystemInteract")) {
        try applyPhoneSystemInteract(ctx, callbacks, params);
    } else if (std.mem.eql(u8, name, "SendNpcMail")) {
        if (quest_types.jsonInt(params.get("MailId"))) |mail_id| {
            try applySendNpcMail(ctx, callbacks, mail_id);
        }
    } else if (std.mem.eql(u8, name, "CompleteGuide")) {
        if (quest_types.jsonInt(params.get("GuideId"))) |guide_id| {
            if (guide_id > 0) {
                if (callbacks.complete_guide) |cb| try cb(ctx, guide_id);
            }
        }
    } else if (std.mem.eql(u8, name, "SetPlayerPos") or std.mem.eql(u8, name, "ServerSetPlayerPos")) {
        const action_id = quest_types.jsonInt(act.object.get("ActionId")) orelse 0;
        teleportPlayerFromAction(ctx, params, action_id, callbacks.is_flow_action);
    } else if (std.mem.eql(u8, name, "SetVar")) {
        try applySetVar(ctx, callbacks, quest_id, params);
    } else if (std.mem.eql(u8, name, "CalculateVar")) {
        try applyCalculateVar(ctx, callbacks, quest_id, params);
    } else if (std.mem.eql(u8, name, "TeleportDungeon")) {
        if (quest_types.jsonInt(params.get("DungeonId"))) |did| {
            const loc_eid = quest_types.jsonI64(params.get("LocationEntityId"));
            try resolveTeleportDungeonAction(ctx, did, loc_eid);
        }
    } else if (std.mem.eql(u8, name, "TeleportDungeonPos")) {
        const did = quest_types.jsonInt(params.get("DungeonId")) orelse return;
        const pos = parseVectorPos(params.get("TeleportPos"));
        try transitionToScene(ctx, did, pos, null);
    } else if (std.mem.eql(u8, name, "ExitDungeon")) {
        try leaveCurrentDungeon(ctx, 0);
    } else if (std.mem.eql(u8, name, "SetAudioState")) {
        try applySetAudioState(ctx, quest_id, params);
    } else if (std.mem.eql(u8, name, "AddPlayBubble")) {
        const guid = quest_types.jsonStr(act.object.get("ActionGuid")) orelse "";
        try applyAddPlayBubble(ctx, params, guid);
    } else if (std.mem.eql(u8, name, "ClearPlayBubble")) {
        try applyClearPlayBubble(ctx, params);
    } else if (std.mem.eql(u8, name, "AddFlowInteractOption")) {
        try applyAddFlowInteractOption(ctx, quest_id, node_id, params);
    } else if (std.mem.eql(u8, name, "RemoveFlowInteractOption")) {
        try applyRemoveFlowInteractOption(ctx, params);
    } else if (std.mem.eql(u8, name, "SetEntityPos")) {
        try applySetEntityPos(ctx, params);
    } else if (std.mem.eql(u8, name, "ResetEntityPos")) {
        try applyResetEntityPos(ctx, params);
    } else if (std.mem.eql(u8, name, "ChangeEntityCamp")) {
        try applyChangeEntityCamp(ctx, params);
    } else if (std.mem.eql(u8, name, "AddGuestCharacter")) {
        try applyGuestCharacter(ctx, params, true);
    } else if (std.mem.eql(u8, name, "RemoveGuestCharacter")) {
        try applyGuestCharacter(ctx, params, false);
    } else if (std.mem.eql(u8, name, "UpdateTrialCharacter")) {
        try applyUpdateTrialCharacter(ctx, params);
    } else if (std.mem.eql(u8, name, "RestoreTrialCharacter")) {
        try removeTrialRoles(ctx, &.{});
    } else if (std.mem.eql(u8, name, "FinishCondition")) {
        if (callbacks.on_finish_condition) |cb| try cb(ctx, quest_id, node_id);
    } else if (std.mem.eql(u8, name, "RunActions")) {
        if (params.get("ActionList")) |al| {
            if (al == .array) try executeActionList(ctx, callbacks, quest_id, node_id, al.array.items);
        }
    } else if (std.mem.eql(u8, name, "RandomVar")) {
        try applyRandomVar(ctx, callbacks, quest_id, params);
    } else if (std.mem.eql(u8, name, "EnableTemporaryTeleport")) {
        if (quest_types.jsonInt(params.get("RangeEntity"))) |eid| {
            if (ctx.progress_comp) |pc| {
                const k = try std.fmt.allocPrint(ctx.alloc.arena, "temp_tp_area:{d}", .{eid});
                const en = quest_types.jsonBool(params.get("Enable")) orelse true;
                try pc.info.setKeyInt(ctx.alloc.gpa, k, if (en) 1 else 0);
                try pc.save(ctx.fs, ctx.alloc.arena);
            }
        }
    } else if (std.mem.eql(u8, name, "AcceptFishingEntrust")) {
        if (quest_types.jsonInt(params.get("EntrustId"))) |eid| {
            try recordProgressInt(ctx, "fish:cur_entrust", eid, true);
        }
    } else if (std.mem.eql(u8, name, "DestroyFishingBoat")) {
        try recordProgressInt(ctx, "fish:boat_active", 0, false);
    } else if (std.mem.eql(u8, name, "SheriffUnlockClue")) {
        if (quest_types.jsonInt(params.get("ClueId"))) |cid| {
            const k = try std.fmt.allocPrint(ctx.alloc.arena, "sheriff_clue:{d}", .{cid});
            try recordProgressClaim(ctx, k);
        }
    } else if (std.mem.eql(u8, name, "SetWuYinQuState")) {
        const wname = quest_types.jsonStr(params.get("WuYinQuName")) orelse "";
        const st = quest_types.jsonInt(params.get("State")) orelse 1;
        if (wname.len > 0) {
            const k = try std.fmt.allocPrint(ctx.alloc.arena, "wuyinqu:{s}", .{wname});
            try recordProgressInt(ctx, k, st, false);
        }
    } else if (std.mem.eql(u8, name, "BvbSkipChallenge")) {
        if (quest_types.jsonInt(params.get("ChallengeId"))) |cid| {
            _ = try QuestInfo.appendUniqueInt(&ctx.quest_comp.info.progress.completed_bvb, ctx.alloc.gpa, cid);
        }
    } else if (std.mem.eql(u8, name, "InfrActivityUpgradeInfrastructure")) {
        if (ctx.progress_comp) |pc| {
            const lv = quest_types.jsonInt(params.get("Level")) orelse 1;
            try pc.info.setKeyInt(ctx.alloc.gpa, "infr:lv", @max(pc.info.getKeyInt("infr:lv"), lv));
            try pc.save(ctx.fs, ctx.alloc.arena);
        }
    } else if (std.mem.eql(u8, name, "HonamiStorySetFormation")) {
        if (ctx.progress_comp) |pc| {
            const aid = quest_types.jsonInt(params.get("ActivityId")) orelse 0;
            try pc.info.setKeyInt(ctx.alloc.gpa, "honami:act_id", aid);
            try pc.save(ctx.fs, ctx.alloc.arena);
        }
    } else if (std.mem.eql(u8, name, "SetupMoraleSystem")) {
        const mpid = quest_types.jsonInt(params.get("MoralePlayId")) orelse 1;
        const is_on = quest_types.jsonBool(params.get("IsOn")) orelse true;
        _ = try MoraleHelper.syncFromInventory(
            ctx.alloc,
            ctx.fs,
            ctx.io,
            ctx.conn,
            ctx.assets,
            ctx.basic_comp,
            ctx.inventory_comp,
            ctx.progress_comp,
            mpid,
            is_on,
            true,
        );
    } else if (std.mem.eql(u8, name, "AddTrialFollowShooter")) {
        const sid = quest_types.jsonInt(params.get("Id")) orelse 0;
        try recordProgressInt(ctx, "trial:follow_shooter", sid, false);
    } else if (std.mem.eql(u8, name, "RemoveTrialFollowShooter")) {
        try recordProgressInt(ctx, "trial:follow_shooter", 0, false);
    } else if (std.mem.eql(u8, name, "ChangeTeamPosition")) {
        if (quest_types.jsonInt(params.get("PositionId"))) |pid| {
            try recordProgressInt(ctx, "team:pos_id", pid, false);
        }
    } else if (std.mem.eql(u8, name, "ResetEntity")) {
        try applyResetEntityAction(ctx, params);
    } else if (std.mem.eql(u8, name, "ChangeOtherState")) {
        try applyChangeOtherState(ctx, callbacks, quest_id, node_id, params);
    } else if (std.mem.eql(u8, name, "SetRegionConfig")) {
        if (quest_types.jsonInt(params.get("RegionMpcId"))) |rid| {
            const k = try std.fmt.allocPrint(ctx.alloc.arena, "region_cfg:{d}", .{rid});
            try recordProgressInt(ctx, k, 1, false);
        }
    } else if (std.mem.eql(u8, name, "GuideTrigger")) {
        try applyGuideTrigger(ctx, callbacks, params);
    } else if (std.mem.eql(u8, name, "FinishDungeon")) {
        try applyFinishDungeon(ctx, params);
    } else if (std.mem.eql(u8, name, "SettlementDungeon")) {
        try applySettlementDungeon(ctx);
    } else if (std.mem.eql(u8, name, "RogueActivatePortal")) {
        try recordProgressInt(ctx, "rogue:portal_active", 1, false);
    } else if (std.mem.eql(u8, name, "DangoAbyssActivatePortal")) {
        try recordProgressInt(ctx, "abyss:portal_active", 1, false);
    } else if (std.mem.eql(u8, name, "DangoAbyssCreateRewardTreasureBox")) {
        _ = try recordProgressAdd(ctx, "abyss:reward_box", 1);
    } else if (std.mem.eql(u8, name, "ChangeTimer") or std.mem.eql(u8, name, "ToggleTimerPauseState")) {
        try applyTimerAction(ctx, name, params);
    } else if (std.mem.eql(u8, name, "RecordDungeonEvent")) {
        const eid = quest_types.jsonInt(params.get("EventConfig")) orelse 0;
        const k = try std.fmt.allocPrint(ctx.alloc.arena, "inst:event:{d}", .{eid});
        try recordProgressClaim(ctx, k);
    } else if (std.mem.eql(u8, name, "BossPilingRandomBuff")) {
        const pool_id = quest_types.jsonInt(params.get("BuffPoolId")) orelse 0;
        try recordProgressInt(ctx, "bp:last_buff_pool", pool_id, false);
    } else if (std.mem.eql(u8, name, "RoverRogueRoomFinish")) {
        const layer = try recordProgressAdd(ctx, "rover_rogue:layer", 1);
        _ = try recordProgressAdd(ctx, "rover_rogue:room_finish", 1);
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.RoverRogueRoomInfoNotify{
            .CurLayer = @intCast(layer),
            .RoomTypeId = 1,
            .MaxLayer = 5,
        }) catch {};
    } else if (std.mem.eql(u8, name, "RoverRogueCreateRoomReward")) {
        const rw_ty = quest_types.jsonInt(params.get("RewardType")) orelse 1;
        try recordProgressInt(ctx, "rover_rogue:room_rw_type", rw_ty, false);
    } else if (std.mem.eql(u8, name, "RoleTowerGotoNextFloor")) {
        const floor = try recordProgressAdd(ctx, "role_tower:stage", 1);
        _ = try recordProgressAdd(ctx, "role_tower:floor", 1);
        try recordProgressInt(ctx, "role_tower:stage", floor, false);
    } else if (std.mem.eql(u8, name, "SheriffUnlockProgress")) {
        if (quest_types.jsonInt(params.get("ProgressId"))) |pid| {
            const k = try std.fmt.allocPrint(ctx.alloc.arena, "sheriff_prog:{d}", .{pid});
            try recordProgressClaim(ctx, k);
            try recordProgressInt(ctx, "sheriff:progress_id", pid, false);
        }
    } else if (std.mem.eql(u8, name, "SheriffChangeCriminalState")) {
        const cid = quest_types.jsonInt(params.get("CriminalId")) orelse 0;
        const st = quest_types.jsonInt(params.get("CriminalState")) orelse 0;
        if (cid > 0) {
            const k = try std.fmt.allocPrint(ctx.alloc.arena, "sheriff_crim_st:{d}", .{cid});
            try recordProgressInt(ctx, k, st, false);
        }
    } else if (std.mem.eql(u8, name, "SheriffChangeCriminalIdentity")) {
        const cid = quest_types.jsonInt(params.get("CriminalId")) orelse 0;
        const iid = quest_types.jsonInt(params.get("IdentityId")) orelse 0;
        if (cid > 0) {
            const k = try std.fmt.allocPrint(ctx.alloc.arena, "sheriff_crim_id:{d}", .{cid});
            try recordProgressInt(ctx, k, iid, false);
        }
    } else if (std.mem.eql(u8, name, "SheriffUnlockEnding")) {
        if (quest_types.jsonInt(params.get("EndingId"))) |eid| {
            const k = try std.fmt.allocPrint(ctx.alloc.arena, "sheriff_end:{d}", .{eid});
            try recordProgressClaim(ctx, k);
        }
    } else if (std.mem.eql(u8, name, "ExecRiskHarvestEffect")) {
        if (quest_types.jsonInt(params.get("Id"))) |eid| {
            const k = try std.fmt.allocPrint(ctx.alloc.arena, "rh:effect:{d}", .{eid});
            try recordProgressClaim(ctx, k);
        }
    } else if (std.mem.eql(u8, name, "HonamiStoryGetItem")) {
        const iid = quest_types.jsonInt(params.get("ItemId")) orelse 0;
        _ = try recordProgressAdd(ctx, "honami:pickup", 1);
        if (iid > 0) {
            const k = try std.fmt.allocPrint(ctx.alloc.arena, "honami:item:{d}", .{iid});
            _ = try recordProgressAdd(ctx, k, 1);
        }
    } else if (std.mem.eql(u8, name, "DrinksRollRoleRequirement")) {
        const rid = quest_types.jsonInt(params.get("RoleId")) orelse 0;
        const req_id = quest_types.jsonInt(params.get("RequirementId")) orelse 1;
        if (rid > 0) {
            const k = try std.fmt.allocPrint(ctx.alloc.arena, "drinks:req:{d}", .{rid});
            try recordProgressInt(ctx, k, req_id, false);
        }
    } else if (std.mem.eql(u8, name, "ChangeFightTeam")) {
        _ = try recordProgressAdd(ctx, "team:fight_change", 1);
    } else if (std.mem.eql(u8, name, "ExecResurrection")) {
        try applyExecResurrection(ctx, params);
    } else if (std.mem.eql(u8, name, "ChangePhantomFormation")) {
        const fid = quest_types.jsonInt(params.get("FormationId")) orelse 0;
        try recordProgressInt(ctx, "phantom:formation_id", fid, false);
    } else if (std.mem.eql(u8, name, "RestorePhantomFormation")) {
        try recordProgressInt(ctx, "phantom:formation_id", 0, false);
    } else if (std.mem.eql(u8, name, "SetRegionConfig")) {
        if (quest_types.jsonInt(params.get("RegionMpcId"))) |rid| {
            try recordProgressInt(ctx, "region:mpc_id", rid, false);
        }
    } else if (std.mem.eql(u8, name, "SetReviveRegion")) {
        if (quest_types.jsonInt(params.get("ReviveId"))) |rid| {
            try recordProgressInt(ctx, "region:revive_id", rid, false);
        }
    } else if (std.mem.eql(u8, name, "SetBattleState")) {
        try applySetBattleStateAction(ctx, params);
    } else if (std.mem.eql(u8, name, "ExecBattleAction")) {
        try applyExecBattleAction(ctx, params);
    } else if (std.mem.eql(u8, name, "ExecAlertSystemAction")) {
        try applyExecAlertSystemAction(ctx, params);
    } else if (std.mem.eql(u8, name, "ModifySceneItemAttributeTag")) {
        try applyModifySceneTagAction(ctx, params);
    } else if (std.mem.eql(u8, name, "SetElementalBoxToTarget") or std.mem.eql(u8, name, "ChangeElementalBoxState")) {
        try applyElementalBoxAction(ctx, name, params);
    } else if (std.mem.eql(u8, name, "SetEntityClientVisibleSave") or std.mem.eql(u8, name, "SetEntityClientVisible")) {
        const vis = quest_types.jsonBool(params.get("Visible")) orelse true;
        try applyEntityVisibilityList(ctx, params.get("EntityIds"), vis);
    } else if (std.mem.eql(u8, name, "SetForceLock")) {
        if (quest_types.jsonInt(params.get("EntityId"))) |eid_raw| {
            if (std.math.cast(i32, eid_raw)) |eid| {
                const locked = quest_types.jsonBool(params.get("IsLocked")) orelse true;
                try setEntityLocked(ctx, eid, locked);
            }
        }
    } else if (std.mem.eql(u8, name, "ChangeEntityPrefabPerformance")) {
        if (quest_types.jsonInt(params.get("EntityId"))) |eid| {
            if (quest_types.jsonStr(params.get("PerformanceTag"))) |perf_tag| {
                try applyEntityStateChange(ctx, callbacks, eid, perf_tag);
            }
        }
    } else if (std.mem.eql(u8, name, "StopSceneItemMove")) {
        try applyStopSceneItemMove(ctx, params);
    } else if (std.mem.eql(u8, name, "StopMotorCruise")) {
        try recordProgressInt(ctx, "motor:cruise", 0, false);
    } else if (std.mem.eql(u8, name, "SetTeleControl")) {
        if (params.get("Config")) |cfg| {
            if (cfg == .object) {
                const ty = quest_types.jsonStr(cfg.object.get("Type")) orelse "";
                if (std.mem.eql(u8, ty, "SetResetPosition")) {
                    if (quest_types.jsonInt(cfg.object.get("EntityId"))) |eid| {
                        if (quest_types.jsonInt(cfg.object.get("PositionEntityId"))) |pos_eid| {
                            const k = try std.fmt.allocPrint(ctx.alloc.arena, "tele_reset:{d}", .{eid});
                            try recordProgressInt(ctx, k, pos_eid, false);
                        }
                    }
                } else if (quest_types.jsonInt(cfg.object.get("EntityId"))) |eid| {
                    const k = try std.fmt.allocPrint(ctx.alloc.arena, "tele_control:{d}", .{eid});
                    try recordProgressInt(ctx, k, 1, false);
                }
            }
        }
    } else if (std.mem.eql(u8, name, "ResetPlayerPosWithPresentation")) {
        try applyResetPlayerPosPresentation(ctx, params);
    } else if (std.mem.eql(u8, name, "GiantToggleModel")) {
        const en = quest_types.jsonBool(params.get("IsEnabled")) orelse true;
        try recordProgressInt(ctx, "giant:model_enabled", if (en) 1 else 0, false);
    } else if (std.mem.eql(u8, name, "PerformSunSpiritGearRoundTrip")) {
        if (quest_types.jsonInt(params.get("SunSpiritGearId"))) |gid| {
            const k = try std.fmt.allocPrint(ctx.alloc.arena, "sun_spirit_gear:{d}", .{gid});
            try recordProgressClaim(ctx, k);
        }
    } else if (std.mem.eql(u8, name, "TriggerSystemRandomPlot")) {
        if (quest_types.jsonInt(params.get("Id"))) |pid| {
            const k = try std.fmt.allocPrint(ctx.alloc.arena, "random_plot:{d}", .{pid});
            try recordProgressClaim(ctx, k);
        }
    } else if (std.mem.eql(u8, name, "CreatePrefab") or std.mem.eql(u8, name, "DestroyPrefab")) {
        if (params.get("VarName")) |vn| {
            if (vn == .object) {
                if (quest_types.jsonStr(vn.object.get("Name"))) |pname| {
                    const k = try std.fmt.allocPrint(ctx.alloc.arena, "prefab:{s}", .{pname});
                    try recordProgressInt(ctx, k, if (std.mem.eql(u8, name, "CreatePrefab")) 1 else 0, false);
                }
            }
        }
    } else if (isTrackedOrClientAction(name)) {
        // Explicit client-side presentation, camera, input, mini-game dialog/constraint, or preload action.
        return;
    } else if (std.mem.eql(u8, name, "ClearEntityVisibleTag")) {
        try applyEntityVisibilityList(ctx, params.get("EntityIds"), true);
    } else if (std.mem.eql(u8, name, "MoveSceneItem") or std.mem.eql(u8, name, "MoveWithSpline") or std.mem.eql(u8, name, "NewMoveWithSpline") or std.mem.eql(u8, name, "ChangeLiftTarget") or std.mem.eql(u8, name, "NpcMoveToPoint") or std.mem.eql(u8, name, "PlayerMoveToPoint") or std.mem.eql(u8, name, "CharacterMoveToPoint")) {
        try applyServerMoveAction(ctx, name, params);
    } else if (std.mem.eql(u8, name, "VehicleEnter") or std.mem.eql(u8, name, "VehicleExitPlayer") or std.mem.eql(u8, name, "VehicleExitNpc") or std.mem.eql(u8, name, "VehicleTeleport") or std.mem.eql(u8, name, "TeleportToAndEnterVehicle") or std.mem.eql(u8, name, "VehicleMoveWithPathLine") or std.mem.eql(u8, name, "StopNewMoveWithSpline")) {
        try applyVehicleAction(ctx, name, params);
    } else if (std.mem.eql(u8, name, "EnableHostility") or std.mem.eql(u8, name, "EnableAI") or std.mem.eql(u8, name, "OverrideAiBehaviorTree")) {
        try applyAiControlAction(ctx, name, params);
    } else if (std.mem.eql(u8, name, "ManualOccupations") or std.mem.eql(u8, name, "ForceOccupations")) {
        try applyOccupationAction(ctx, params);
    } else if (std.mem.eql(u8, name, "SetNpcGroupPerform")) {
        try applySetNpcGroupPerform(ctx, params);
    } else if (std.mem.eql(u8, name, "RemoveNpcGroupPerform")) {
        if (quest_types.jsonStr(params.get("Key"))) |key| {
            const k = try std.fmt.allocPrint(ctx.alloc.arena, "npc:group_perform:{s}", .{key});
            try recordProgressInt(ctx, k, 0, false);
        }
        try recordProgressInt(ctx, "npc:supported_walking", 0, false);
    } else if (std.mem.eql(u8, name, "PlayBubble")) {
        const guid = quest_types.jsonStr(act.object.get("ActionGuid")) orelse "";
        try applyAddPlayBubble(ctx, params, guid);
    } else {
        log.debug("unhandled quest action: {s}", .{name});
    }
}

pub fn executeFlowStateActionById(
    ctx: QuestContext,
    callbacks: ActionCallbacks,
    flow_name: []const u8,
    flow_id: i32,
    state_id: i32,
    quest_id: i32,
    node_id: i32,
    action_id: i32,
) anyerror!void {
    const state_val = (try ctx.assets.flow_states.getFlowState(
        ctx.io,
        ctx.alloc.arena,
        flow_name,
        flow_id,
        state_id,
    )) orelse return;

    const action_val = Assets.LazyFlowStore.findActionById(state_val, action_id) orelse return;
    try executeAction(ctx, callbacks, quest_id, node_id, action_val);
}

pub fn pushFuncOpen(ctx: QuestContext) !void {
    try ConditionEvaluator.pushFuncOpenNotify(
        ctx.conn,
        ctx.alloc,
        ctx.assets,
        ctx.basic_comp,
        ctx.echo_comp,
        ctx.inventory_comp,
        ctx.map_comp,
        ctx.quest_comp,
        ctx.role_comp,
        ctx.io,
    );
    if (ctx.progress_comp) |prog| {
        activity_handler.pushActivityUpdateNotify(ctx.conn, ctx.alloc, ctx.assets, prog, .{
            .assets = ctx.assets,
            .basic_comp = ctx.basic_comp,
            .echo_comp = ctx.echo_comp,
            .inventory_comp = ctx.inventory_comp,
            .map_comp = ctx.map_comp,
            .quest_comp = ctx.quest_comp,
            .role_comp = ctx.role_comp,
            .progress_comp = prog,
        }, ctx.basic_comp.player_id, ctx.io);
    }
}

pub fn unlockTeleporter(ctx: QuestContext, tp_id: i32) !void {
    if (try ctx.map_comp.info.unlockTeleporter(ctx.alloc.gpa, tp_id)) {
        if (ctx.assets.tables.teleporter.getDataById(tp_id)) |tp| {
            if (tp.Type == 1 and tp.FogId > 0) {
                _ = try map_handler.unlockFogAndSiblings(ctx.conn, ctx.assets, ctx.map_comp, ctx.alloc.gpa, ctx.io, tp.FogId);
            }
            if (std.math.cast(i32, tp.TeleportEntityConfigId)) |cfg_id32| {
                if (cfg_id32 > 0) {
                    try ctx.quest_comp.info.setEntityState(ctx.alloc.gpa, tp.TeleportEntityConfigId, "关卡.Common.状态.激活");
                    try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);
                    if (ctx.scene) |scene| {
                        if (scene.findEntityNetIdByConfigId(cfg_id32)) |nid| {
                            if (try scene.updateEntityStateTag(ctx.alloc.gpa, &ctx.assets.tables.gameplay_tag_remap, nid, "关卡.Common.状态.激活")) |upd| {
                                try emitEntityStateTags(ctx, nid, upd);
                            }
                        }
                    }
                }
            }
        }
        try ctx.map_comp.save(ctx.fs, ctx.alloc.arena);

        var tp_ids: std.ArrayList(i32) = .empty;
        try tp_ids.append(ctx.alloc.arena, tp_id);
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.TeleportUpdateNotify{ .Ids = tp_ids }) catch {};
    }
}

pub fn applyDataLayerChange(ctx: QuestContext, load_layers: []const i32, unload_layers: []const i32) !void {
    if (load_layers.len == 0 and unload_layers.len == 0) return;

    var notify: pb.SceneChangeDataLayerNotify = .default;
    for (load_layers) |dl| {
        if (dl <= 0) continue;
        _ = try QuestInfo.appendUniqueInt(&ctx.quest_comp.info.world.loaded_datalayers, ctx.alloc.gpa, dl);
        _ = try QuestInfo.removeIntValue(&ctx.quest_comp.info.world.unloaded_datalayers, ctx.alloc.gpa, dl);
        try notify.LoadDataLayers.append(ctx.alloc.arena, dl);
    }
    for (unload_layers) |dl| {
        if (dl <= 0) continue;
        _ = try QuestInfo.removeIntValue(&ctx.quest_comp.info.world.loaded_datalayers, ctx.alloc.gpa, dl);
        _ = try QuestInfo.appendUniqueInt(&ctx.quest_comp.info.world.unloaded_datalayers, ctx.alloc.gpa, dl);
        try notify.UnloadDataLayers.append(ctx.alloc.arena, dl);
    }
    ctx.conn.push(ctx.alloc.gpa, ctx.io, notify) catch {};
    try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);

    if (ctx.scene) |scene| {
        var remove_ids: std.ArrayList(i64) = .empty;
        const slice = scene.entities.slice();
        for (slice.items(.entity_id), slice.items(.config)) |entity_id, config| {
            if (config.entity_type == .player or config.entity_type == .player_entity or config.config_id <= 0) continue;
            const entity_cfg = ctx.assets.findLevelEntity(scene.instance_id, config.config_id) orelse continue;
            if (!entityDataLayersActive(ctx.assets, ctx.quest_comp.info, scene.instance_id, entity_cfg)) {
                try remove_ids.append(ctx.alloc.arena, entity_id.net_id);
            }
        }
        if (remove_ids.items.len > 0) {
            var remove_infos: std.ArrayList(pb.EntityRemoveInfo) = .empty;
            for (remove_ids.items) |net_id| {
                try remove_infos.append(ctx.alloc.arena, .{ .EntityId = net_id, .Type = .RemoveTypeNormal });
                try scene.remove(ctx.alloc.gpa, ctx.fs, net_id);
            }
            ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.EntityRemoveNotify{
                .RemoveInfos = remove_infos,
                .IsRemove = true,
            }) catch {};
        }
        if (scene.instance.players.len > 0) {
            try ctx.events.enqueue(.spatial_spawns_check, .{ .location = scene.instance.players[0].location });
        }
    }
}

pub fn entityDataLayersActive(
    assets: *const Assets,
    quest_info: QuestInfo,
    instance_id: i32,
    entity_cfg: *const Assets.DataTables.LevelEntityConfig,
) bool {
    const base = entity_cfg.ComponentsData.BaseInfoComponent orelse return true;
    const layers = base.DataLayers orelse return true;
    if (layers.len == 0) return true;

    const map_id = assets.resolveMapConfigId(instance_id);
    for (layers) |layer_id| {
        if (layer_id <= 0 or QuestInfo.containsInt(quest_info.world.unloaded_datalayers, layer_id)) return false;
        const layer_cfg = assets.tables.data_layer_config.getDataById(layer_id) orelse return false;
        if (layer_cfg.DataLayer.len == 0) return false;
        if (layer_cfg.LevelId != 0 and layer_cfg.LevelId != instance_id and layer_cfg.LevelId != map_id) return false;
        if (!layer_cfg.InitLoad and !QuestInfo.containsInt(quest_info.world.loaded_datalayers, layer_id)) return false;
    }
    return true;
}

pub fn syncPersistedVehicleNpcLinks(
    scene: *Scene,
    progress_comp: *@import("../component/player/PlayerProgressComponent.zig"),
    conn: *Connection,
    alloc: mem.Alloc,
    io: std.Io,
    only_config_id: ?i32,
) !void {
    for (progress_comp.info.dynamic.ints) |entry| {
        if (!std.mem.startsWith(u8, entry.key, "vehicle:npc:")) continue;
        const npc_config_id = std.fmt.parseInt(i32, entry.key["vehicle:npc:".len..], 10) catch continue;
        const vehicle_config_id = std.math.cast(i32, entry.val) orelse continue;
        if (npc_config_id <= 0 or vehicle_config_id <= 0) continue;
        if (only_config_id) |filter| {
            if (filter != npc_config_id and filter != vehicle_config_id) continue;
        }
        const npc_net_id = scene.findEntityNetIdByConfigId(npc_config_id) orelse continue;
        const vehicle_net_id = scene.findEntityNetIdByConfigId(vehicle_config_id) orelse continue;
        const seat_key = try std.fmt.allocPrint(alloc.arena, "vehicle:npc_seat:{d}", .{npc_config_id});
        const seat = std.math.cast(i32, progress_comp.info.getKeyInt(seat_key)) orelse 0;
        const vehicle_index = scene.net_id_map.get(vehicle_net_id) orelse continue;
        if (scene.entities.items(.vehicle)[vehicle_index]) |*vehicle| {
            vehicle.occupied = true;
        } else continue;
        conn.push(alloc.gpa, io, pb.VehicleUpdateEntityNotify{
            .EntityId = npc_net_id,
            .VehicleCreatureId = vehicle_net_id,
            .Seat = seat,
            .IsEntering = true,
        }) catch {};
    }
}

pub fn updatePlayerSceneState(ctx: QuestContext, location: [3]f32, rotation_opt: ?[3]f32) !void {
    const scene = ctx.scene orelse return;
    if (!std.math.isFinite(location[0]) or !std.math.isFinite(location[1]) or !std.math.isFinite(location[2])) return;
    if (rotation_opt) |rotation| {
        if (!std.math.isFinite(rotation[0]) or !std.math.isFinite(rotation[1]) or !std.math.isFinite(rotation[2])) return;
    }
    var rotation: [3]f32 = .{ 0, 0, 0 };
    var found_player = false;
    for (scene.instance.players) |*player| {
        if (player.id != scene.player_id) continue;
        found_player = true;
        rotation = player.rotation;
        player.location = location;
        if (rotation_opt) |new_rotation| {
            rotation = new_rotation;
            player.rotation = new_rotation;
        }
        break;
    }
    if (!found_player) return;
    scene.last_spatial_check_loc = location;

    const slice = scene.entities.slice();
    for (slice.items(.entity_id), slice.items(.direct_control), 0..) |entity, direct, index| {
        if (direct == null) continue;
        _ = scene.setEntityPosition(entity.net_id, location, rotation[2]);
        if (scene.entities.items(.position)[index]) |*position| position.rotation = rotation;
        try scene.saveComponents(ctx.fs, ctx.alloc.gpa, .{ .index = index, .net_id = entity.net_id }, &.{Scene.Entity.PositionComponent});
    }
    try scene.saveInstance(ctx.fs, ctx.alloc.gpa);
    try ctx.events.enqueue(.spatial_spawns_check, .{ .location = location });
}

pub fn applySubLevelChange(
    ctx: QuestContext,
    load_levels: []const []const u8,
    unload_levels: []const []const u8,
    hide_levels: []const []const u8,
    teleport_entity_id: i32,
) !void {
    if (load_levels.len == 0 and unload_levels.len == 0 and hide_levels.len == 0 and teleport_entity_id == 0) return;

    var notify: pb.SceneSubLevelsChangedNotify = .default;
    notify.TeleportEntityId = teleport_entity_id;
    for (load_levels) |lvl| {
        _ = try QuestInfo.appendUniqueStr(&ctx.quest_comp.info.world.loaded_sublevels, ctx.alloc.gpa, lvl);
        _ = try QuestInfo.removeStrValue(&ctx.quest_comp.info.world.hidden_sublevels, ctx.alloc.gpa, lvl);
        try notify.NewLoaded.append(ctx.alloc.arena, lvl);
    }
    for (unload_levels) |lvl| {
        _ = try QuestInfo.removeStrValue(&ctx.quest_comp.info.world.loaded_sublevels, ctx.alloc.gpa, lvl);
        try notify.NewUnloaded.append(ctx.alloc.arena, lvl);
    }
    for (hide_levels) |lvl| {
        _ = try QuestInfo.appendUniqueStr(&ctx.quest_comp.info.world.hidden_sublevels, ctx.alloc.gpa, lvl);
        try notify.HideSubLevels.append(ctx.alloc.arena, lvl);
    }

    if (teleport_entity_id > 0) {
        if (ctx.scene) |scene| {
            if (ctx.assets.findLevelEntity(scene.instance_id, teleport_entity_id)) |cfg| {
                if (Assets.AssetIndexes.entityWorldPos(cfg)) |pos| {
                    const rot = Assets.AssetIndexes.entityWorldRot(cfg);
                    try updatePlayerSceneState(ctx, pos, rot);
                }
            }
        }
    }
    ctx.conn.push(ctx.alloc.gpa, ctx.io, notify) catch {};
    try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);
}

pub fn applyWeatherChange(ctx: QuestContext, callbacks: ActionCallbacks, weather_id: i32) !void {
    ctx.quest_comp.info.env.weather_id = weather_id;
    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.WeatherNotify{
        .WeatherId = weather_id,
        .IsClient = false,
    }) catch {};
    if (weather_id > 0) {
        js_patches.weather.push(ctx.alloc, ctx.conn, ctx.io, weather_id) catch {};
    }
    if (callbacks.on_weather_changed) |cb| try cb(ctx, weather_id);
}

pub fn applyAreaStateChange(ctx: QuestContext, area_id: i32, state: bool) !void {
    if (area_id <= 0) return;
    try ctx.quest_comp.info.setAreaState(ctx.alloc.gpa, area_id, state);
    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.SceneAreaStateNotify{
        .AreaState = .{ .AreaId = area_id, .State = state },
    }) catch {};
    try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);
}

pub fn applySetTime(ctx: QuestContext, callbacks: ActionCallbacks, hour: i32, minute: i32, show_ui: bool) !void {
    const norm_hour = @mod(hour, 24);
    const norm_min = @mod(minute, 60);
    const already_same = (ctx.quest_comp.info.env.time_lock_hour == norm_hour and ctx.quest_comp.info.env.time_lock_minute == norm_min);
    if (already_same and !show_ui) return;

    ctx.quest_comp.info.env.time_lock_hour = norm_hour;
    ctx.quest_comp.info.env.time_lock_minute = norm_min;

    const now_ms = std.Io.Clock.real.now(ctx.io).toMilliseconds();
    if (ctx.scene) |scene| {
        scene.instance.map_time.hour = norm_hour;
        scene.instance.map_time.minute = norm_min;
        scene.instance.map_time.owner_clock_time_span = now_ms;
    }
    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.SyncSceneTimeNotify{
        .TimeInfo = .{
            .Hour = norm_hour,
            .Minute = norm_min,
            .OwnerTimeClockTimeSpan = now_ms,
        },
        .ShowUi = show_ui,
    }) catch {};
    if (callbacks.on_time_changed) |cb| try cb(ctx, norm_hour, norm_min);
}

pub fn applyTimeLockState(ctx: QuestContext, locked: bool, hour_opt: ?i32, min_opt: ?i32) !void {
    ctx.quest_comp.info.env.time_locked = locked;
    if (hour_opt) |h| ctx.quest_comp.info.env.time_lock_hour = @mod(h, 24);
    if (min_opt) |m| ctx.quest_comp.info.env.time_lock_minute = @mod(m, 60);
    if (ctx.scene) |scene| {
        scene.instance.map_time.hour = ctx.quest_comp.info.env.time_lock_hour;
        scene.instance.map_time.minute = ctx.quest_comp.info.env.time_lock_minute;
        scene.instance.map_time.owner_clock_time_span = std.Io.Clock.real.now(ctx.io).toMilliseconds();
        try scene.saveInstance(ctx.fs, ctx.alloc.gpa);
    }
    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.TimeLockInfoNotify{
        .IsLock = locked,
        .Hour = ctx.quest_comp.info.env.time_lock_hour,
        .Minute = ctx.quest_comp.info.env.time_lock_minute,
    }) catch {};
}

pub fn applyTrialRoles(ctx: QuestContext, role_ids: []const i32, enable_map_and_tp: bool) !void {
    try applyTrialRolesWithOptions(ctx, role_ids, enable_map_and_tp, false, false, &.{});
}

pub fn applyInstanceTrialRoles(ctx: QuestContext, inst_id: i32) !void {
    if (inst_id <= 0 or inst_id == 8) return;
    if (ctx.quest_comp.info.env.trial_roles.len > 0) return;

    if (ctx.assets.tables.instance_trial_role_config.getDataById(inst_id)) |trial_cfg| {
        const is_male = ctx.basic_comp.info.attributes.sex == 1;
        const form = if (is_male) trial_cfg.MaleFormation else trial_cfg.FemaleFormation;
        if (form.len > 0) {
            try applyTrialRolesWithOptions(ctx, form, true, true, trial_cfg.AutoChange, &.{inst_id});
            return;
        }
    }

    for (ctx.assets.tables.role_trial_info.items) |rt| {
        if (rt.InstanceId == inst_id and rt.TrialRoleId > 0) {
            try applyTrialRolesWithOptions(ctx, &.{rt.TrialRoleId}, true, true, true, &.{inst_id});
            return;
        }
    }

    if (ctx.assets.tables.instance_dungeon.getDataById(inst_id)) |inst_cfg| {
        var ids: [3]i32 = undefined;
        var count: usize = 0;
        for (inst_cfg.TrialRoleInfo) |tid| {
            if (tid > 0) {
                ids[count] = tid;
                count += 1;
            }
        }
        if (count > 0) {
            try applyTrialRolesWithOptions(ctx, ids[0..count], true, true, true, &.{inst_id});
        }
    }
}

fn applyTrialRolesWithOptions(
    ctx: QuestContext,
    raw_role_ids: []const i32,
    enable_map_and_tp: bool,
    create_temp_team: bool,
    auto_change: bool,
    dungeon_ids: []const i32,
) !void {
    const world_level = @max(1, ctx.basic_comp.info.attributes.cur_world_level);
    const gender = ctx.basic_comp.info.attributes.sex;

    var resolved_ids: std.ArrayList(i32) = .empty;
    for (raw_role_ids) |raw_id| {
        if (raw_id <= 0) continue;
        const resolved_id: i32 = if (QuestInfo.containsInt(ctx.quest_comp.info.env.trial_roles, raw_id) and
            ctx.assets.tables.trial_role_info.getDataById(raw_id) != null)
            raw_id
        else
            ctx.assets.tables.resolveTrialRoleId(raw_id, world_level, gender) orelse raw_id;

        if (!QuestInfo.containsInt(resolved_ids.items, resolved_id)) {
            try resolved_ids.append(ctx.alloc.arena, resolved_id);
        }
    }
    if (resolved_ids.items.len == 0) return;

    var env_dirty = false;
    var newly_materialized: std.ArrayList(i32) = .empty;
    for (resolved_ids.items) |tid| {
        if (try QuestInfo.appendUniqueInt(&ctx.quest_comp.info.env.trial_roles, ctx.alloc.gpa, tid)) {
            env_dirty = true;
        }
        if (ctx.role_comp) |role_comp| {
            if (try TrialRoles.materializeTrialRole(
                ctx.alloc.gpa,
                ctx.assets,
                role_comp,
                ctx.weapon_comp,
                ctx.echo_comp,
                tid,
            )) {
                try newly_materialized.append(ctx.alloc.arena, tid);
            }
        }
    }

    if (ctx.role_comp) |role_comp| {
        if (newly_materialized.items.len > 0) {
            try TrialRoles.pushTrialRoleList(
                ctx.conn,
                ctx.alloc,
                ctx.assets,
                role_comp,
                ctx.weapon_comp,
                ctx.echo_comp,
                newly_materialized.items,
                ctx.io,
            );
        }
    }

    if (ctx.scene) |scene| {
        try scene.setTrialRoles(ctx.alloc.gpa, ctx.quest_comp.info.env.trial_roles);
    }

    var notify: pb.RoleTrialOpenNotify = .default;
    notify.ShowTips = newly_materialized.items.len > 0;
    notify.EnableMapAndTeleport = enable_map_and_tp;
    for (resolved_ids.items) |tid| {
        try notify.RoleIds.append(ctx.alloc.arena, tid);
        try notify.TrialRoleItems.append(ctx.alloc.arena, .{ .RoleId = tid, .Show = true });
    }
    for (dungeon_ids) |did| {
        if (did > 0) try notify.CanEnterDungeonList.append(ctx.alloc.arena, did);
    }
    if (notify.RoleIds.items.len > 0) {
        ctx.conn.push(ctx.alloc.gpa, ctx.io, notify) catch {};
    }

    if (ctx.scene) |scene| {
        if (ctx.role_comp) |role_comp| {
            const had_saved = ctx.quest_comp.info.env.pre_trial_roles.len != 0;
            try TrialRoles.savePreTrialFormation(ctx.quest_comp, ctx.alloc.gpa, scene);
            if (!had_saved and ctx.quest_comp.info.env.pre_trial_roles.len != 0) env_dirty = true;
            try TrialRoles.applyTrialRolesToSceneFormation(
                scene,
                ctx.fs,
                ctx.assets,
                ctx.conn,
                ctx.events,
                ctx.alloc,
                role_comp,
                ctx.weapon_comp,
                ctx.echo_comp,
                resolved_ids.items,
                create_temp_team,
                auto_change,
                ctx.io,
            );
        }
    }

    if (env_dirty) {
        try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);
    }
}

pub fn removeTrialRoles(ctx: QuestContext, role_ids: []const i32) !void {
    var removed_ids: std.ArrayList(i32) = .empty;
    var notify: pb.RoleTrialCloseNotify = .default;
    notify.EnableMapAndTeleport = true;

    if (role_ids.len == 0) {
        for (ctx.quest_comp.info.env.trial_roles) |rid| {
            if (!QuestInfo.containsInt(removed_ids.items, rid)) {
                try removed_ids.append(ctx.alloc.arena, rid);
                try notify.RoleIds.append(ctx.alloc.arena, rid);
            }
        }
        if (ctx.quest_comp.info.env.trial_roles.len != 0) {
            ctx.alloc.gpa.free(ctx.quest_comp.info.env.trial_roles);
            ctx.quest_comp.info.env.trial_roles = &.{};
        }
    } else {
        const world_level = @max(1, ctx.basic_comp.info.attributes.cur_world_level);
        const gender = ctx.basic_comp.info.attributes.sex;
        for (role_ids) |raw_id| {
            if (raw_id <= 0) continue;
            var matched_any = false;

            if (try QuestInfo.removeIntValue(&ctx.quest_comp.info.env.trial_roles, ctx.alloc.gpa, raw_id)) {
                matched_any = true;
                if (!QuestInfo.containsInt(removed_ids.items, raw_id)) {
                    try removed_ids.append(ctx.alloc.arena, raw_id);
                    try notify.RoleIds.append(ctx.alloc.arena, raw_id);
                }
            }

            if (ctx.assets.tables.resolveTrialRoleId(raw_id, world_level, gender)) |resolved_id| {
                if (try QuestInfo.removeIntValue(&ctx.quest_comp.info.env.trial_roles, ctx.alloc.gpa, resolved_id)) {
                    matched_any = true;
                    if (!QuestInfo.containsInt(removed_ids.items, resolved_id)) {
                        try removed_ids.append(ctx.alloc.arena, resolved_id);
                        try notify.RoleIds.append(ctx.alloc.arena, resolved_id);
                    }
                }
            }

            for (ctx.assets.tables.trial_role_info.items) |entry| {
                if (entry.GroupId != raw_id) continue;
                if (try QuestInfo.removeIntValue(&ctx.quest_comp.info.env.trial_roles, ctx.alloc.gpa, entry.Id)) {
                    matched_any = true;
                    if (!QuestInfo.containsInt(removed_ids.items, entry.Id)) {
                        try removed_ids.append(ctx.alloc.arena, entry.Id);
                        try notify.RoleIds.append(ctx.alloc.arena, entry.Id);
                    }
                }
            }

            if (!matched_any and !QuestInfo.containsInt(notify.RoleIds.items, raw_id)) {
                try notify.RoleIds.append(ctx.alloc.arena, raw_id);
            }
        }
    }

    if (ctx.scene) |scene| {
        try scene.setTrialRoles(ctx.alloc.gpa, ctx.quest_comp.info.env.trial_roles);
    }
    if (notify.RoleIds.items.len > 0) {
        ctx.conn.push(ctx.alloc.gpa, ctx.io, notify) catch {};
    }

    if (removed_ids.items.len > 0) {
        if (ctx.scene) |scene| {
            if (ctx.role_comp) |role_comp| {
                try TrialRoles.restoreFormationAfterTrialRemoval(
                    scene,
                    ctx.fs,
                    ctx.assets,
                    ctx.conn,
                    ctx.events,
                    ctx.alloc,
                    ctx.basic_comp,
                    role_comp,
                    ctx.weapon_comp,
                    ctx.echo_comp,
                    ctx.quest_comp,
                    removed_ids.items,
                    ctx.quest_comp.info.env.trial_roles,
                    ctx.io,
                );
            }
        } else if (ctx.quest_comp.info.env.trial_roles.len == 0) {
            TrialRoles.clearPreTrialFormation(ctx.quest_comp, ctx.alloc.gpa);
        }

        if (ctx.role_comp) |role_comp| {
            TrialRoles.cleanupRemovedTrialRoles(
                ctx.alloc.gpa,
                role_comp,
                ctx.weapon_comp,
                ctx.echo_comp,
                removed_ids.items,
            );
        }
        try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);
    }
}

pub fn toggleMapMark(ctx: QuestContext, mark_id: i32, action_type: []const u8) !void {
    if (mark_id <= 0) return;
    const show = std.mem.eql(u8, action_type, "Show") or std.mem.eql(u8, action_type, "Enable");
    if (show) {
        _ = try QuestInfo.appendUniqueInt(&ctx.quest_comp.info.world.unlocked_map_marks, ctx.alloc.gpa, mark_id);
        _ = try QuestInfo.removeIntValue(&ctx.quest_comp.info.world.hidden_map_marks, ctx.alloc.gpa, mark_id);
    } else {
        _ = try QuestInfo.appendUniqueInt(&ctx.quest_comp.info.world.hidden_map_marks, ctx.alloc.gpa, mark_id);
        _ = try QuestInfo.removeIntValue(&ctx.quest_comp.info.world.unlocked_map_marks, ctx.alloc.gpa, mark_id);
    }
    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.MapMarkShowIdInfoUpdateNotify{
        .ShowFlag = if (show) 1 else 0,
        .IsShow = show,
        .MarkId = mark_id,
        .NeedFocus = false,
    }) catch {};
}

pub fn resolveTeleportDungeonAction(ctx: QuestContext, did: i32, opt_loc_eid: ?i64) !void {
    if (did <= 0) return;
    var opt_pos: ?[3]f32 = null;
    var opt_rot: ?[3]f32 = null;
    if (opt_loc_eid) |loc_eid| {
        if (loc_eid > 0) {
            if (ctx.assets.findLevelEntity(did, loc_eid)) |cfg| {
                opt_pos = Assets.AssetIndexes.entityWorldPos(cfg);
                opt_rot = Assets.AssetIndexes.entityWorldRot(cfg);
            }
        }
    }
    try transitionToScene(ctx, did, opt_pos, opt_rot);
}

pub fn transitionToScene(
    ctx: QuestContext,
    target_dungeon_id: i32,
    opt_pos: ?[3]f32,
    opt_rot: ?[3]f32,
) !void {
    if (target_dungeon_id <= 0) return;
    const cur_inst_id = if (ctx.scene) |s| s.instance_id else 8;
    const target_inst = ctx.assets.tables.instance_dungeon.getDataById(target_dungeon_id) orelse return;

    if (target_dungeon_id == cur_inst_id) {
        if (opt_pos) |pos| {
            const rot = opt_rot orelse .{ 0, 0, 0 };
            try updatePlayerSceneState(ctx, pos, rot);
            ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.TeleportNotify{
                .MapId = target_dungeon_id,
                .Pos = .{ .X = pos[0], .Y = pos[1], .Z = pos[2] },
                .Rot = .{ .X = rot[0], .Y = rot[1], .Z = rot[2] },
                .Reason = .Action,
            }) catch {};
        }
        return;
    }

    if (ctx.scene) |scene| {
        if (scene.pending_transition_instance_id) |pending_id| {
            if (pending_id == target_dungeon_id) return;
        }
        scene.pending_transition_instance_id = target_dungeon_id;
        if (cur_inst_id == 8 and scene.instance.players.len > 0) {
            scene.instance.prev_world_instance_id = 8;
            scene.instance.prev_world_location = scene.instance.players[0].location;
            scene.instance.prev_world_rotation = scene.instance.players[0].rotation;
        }
        scene.instance.active_dungeon_id = if (target_dungeon_id == 8) 0 else target_dungeon_id;
        scene.savePersistent(ctx.fs, ctx.alloc.gpa) catch {};
    }

    if (ctx.scene_comp) |sc| {
        sc.last_scene_info.instance_id = target_dungeon_id;
    }
    const last_path = try std.fmt.allocPrint(ctx.alloc.arena, "player/{d}/last_scene", .{ctx.basic_comp.player_id});
    try comp_util.saveStruct(ctx.fs, LastSceneInfo{ .instance_id = target_dungeon_id }, last_path, ctx.alloc.arena);

    try ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.LeaveSceneNotify{
        .PlayerId = ctx.basic_comp.player_id,
        .SceneId = "",
        .TransitionOption = .{},
    });

    const spawn_pos: ?[3]f32 = opt_pos orelse if (target_dungeon_id != 8 and
        (target_inst.BornPosition[0] != 0 or target_inst.BornPosition[1] != 0 or target_inst.BornPosition[2] != 0))
        .{
            @floatFromInt(target_inst.BornPosition[0]),
            @floatFromInt(target_inst.BornPosition[1]),
            @floatFromInt(target_inst.BornPosition[2]),
        }
    else
        null;

    const spawn_rot: ?[3]f32 = opt_rot orelse if (target_dungeon_id != 8 and
        (target_inst.BornRotation[0] != 0 or target_inst.BornRotation[1] != 0 or target_inst.BornRotation[2] != 0))
        .{
            @floatFromInt(target_inst.BornRotation[0]),
            @floatFromInt(target_inst.BornRotation[1]),
            @floatFromInt(target_inst.BornRotation[2]),
        }
    else
        null;

    try ctx.events.enqueue(.initial_scene_join, .{
        .target_instance_id = target_dungeon_id,
        .location = spawn_pos,
        .rotation = spawn_rot,
    });
}

pub fn leaveCurrentDungeon(ctx: QuestContext, pos_entity_id: i32) !void {
    const cur_id = if (ctx.scene) |s| s.instance_id else 8;
    if (cur_id == 8) return;

    var target_map_id: i32 = 8;
    var opt_pos: ?[3]f32 = null;

    if (ctx.assets.tables.instance_dungeon.getDataById(cur_id)) |cur_inst| {
        if (cur_inst.RecoverWorldLocation.len >= 4 and cur_inst.RecoverWorldLocation[0] > 0) {
            target_map_id = cur_inst.RecoverWorldLocation[0];
            const rx = cur_inst.RecoverWorldLocation[1];
            const ry = cur_inst.RecoverWorldLocation[2];
            const rz = cur_inst.RecoverWorldLocation[3];
            if (rx != 0 or ry != 0 or rz != 0) {
                opt_pos = .{
                    @floatFromInt(rx),
                    @floatFromInt(ry),
                    @floatFromInt(rz),
                };
            }
        }
    }

    if (pos_entity_id > 0) {
        if (ctx.assets.findLevelEntity(target_map_id, pos_entity_id)) |cfg| {
            opt_pos = Assets.AssetIndexes.entityWorldPos(cfg);
        }
    }

    try removeTrialRoles(ctx, &.{});
    try transitionToScene(ctx, target_map_id, opt_pos, null);
}

pub fn sendQuestEntityVisibility(ctx: QuestContext, entity_ids: []const i32, visible: bool) !void {
    const scene = ctx.scene orelse return;
    if (entity_ids.len == 0) return;

    if (visible) {
        for (entity_ids) |eid| {
            if (eid <= 0) continue;
            ensureQuestEntitySpawned(ctx, eid);
            _ = try QuestInfo.appendUniqueInt(&ctx.quest_comp.info.world.shown_entities, ctx.alloc.gpa, eid);
            _ = try QuestInfo.removeIntValue(&ctx.quest_comp.info.world.hidden_entities, ctx.alloc.gpa, eid);
        }
    } else {
        for (entity_ids) |eid| {
            if (eid <= 0) continue;
            _ = try QuestInfo.appendUniqueInt(&ctx.quest_comp.info.world.hidden_entities, ctx.alloc.gpa, eid);
            _ = try QuestInfo.removeIntValue(&ctx.quest_comp.info.world.shown_entities, ctx.alloc.gpa, eid);
        }
    }

    var combat_pack: std.ArrayList(pb.CombatReceiveData) = .empty;
    var flow_ids: std.ArrayList(i32) = .empty;
    const slice = scene.entities.slice();
    for (entity_ids) |target_cfg| {
        for (slice.items(.config), slice.items(.entity_id)) |cfg, nid| {
            if (cfg.config_id != target_cfg) continue;
            _ = try scene.setEntityVisibility(nid.net_id, visible, &combat_pack, ctx.alloc.arena);
        }
        try flow_ids.append(ctx.alloc.arena, target_cfg);
    }

    if (combat_pack.items.len > 0) {
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.CombatReceivePackNotify{ .Data = combat_pack }) catch {};
    }
    if (flow_ids.items.len > 0) {
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.FlowShowEntityNotify{
            .EntityIds = flow_ids,
            .IsHide = !visible,
        }) catch {};
    }
}

pub fn applyEntityStateChange(
    ctx: QuestContext,
    callbacks: ActionCallbacks,
    entity_cfg_id: i32,
    state_str: []const u8,
) !void {
    if (entity_cfg_id <= 0 or state_str.len == 0) return;
    try ctx.quest_comp.info.setEntityState(ctx.alloc.gpa, entity_cfg_id, state_str);

    const is_sleep = std.mem.indexOf(u8, state_str, "休眠") != null or
        std.mem.indexOf(u8, state_str, "未激活") != null;
    const is_visible = !is_sleep and std.mem.indexOf(u8, state_str, "隐藏") == null;

    if (!is_sleep) {
        ensureQuestEntitySpawned(ctx, entity_cfg_id);
    }

    if (ctx.scene) |scene| {
        var combat_pack: std.ArrayList(pb.CombatReceiveData) = .empty;
        const slice = scene.entities.slice();
        const configs = slice.items(.config);
        const net_ids = slice.items(.entity_id);

        for (configs, net_ids) |*cfg, nid| {
            if (cfg.config_id != entity_cfg_id) continue;
            if (std.mem.indexOf(u8, state_str, "常态") != null) {
                cfg.state = .default;
            } else if (is_sleep) {
                cfg.state = .sleep;
            } else {
                cfg.state = .other;
            }

            if (try scene.updateEntityStateTag(ctx.alloc.gpa, &ctx.assets.tables.gameplay_tag_remap, nid.net_id, state_str)) |upd| {
                try emitEntityStateTags(ctx, nid.net_id, upd);
            }
            _ = try scene.setEntityVisibility(nid.net_id, is_visible, &combat_pack, ctx.alloc.arena);
        }
        if (combat_pack.items.len > 0) {
            ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.CombatReceivePackNotify{ .Data = combat_pack }) catch {};
        }
    }

    if (callbacks.on_entity_state_changed) |cb| {
        try cb(ctx, entity_cfg_id, state_str);
    }
}

pub fn syncWorldStateOnJoin(ctx: QuestContext) !void {
    if (ctx.quest_comp.info.env.weather_id > 0) {
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.WeatherNotify{
            .WeatherId = ctx.quest_comp.info.env.weather_id,
            .IsClient = false,
        }) catch {};
    }
    if (ctx.quest_comp.info.env.time_locked) {
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.TimeLockInfoNotify{
            .IsLock = true,
            .Hour = ctx.quest_comp.info.env.time_lock_hour,
            .Minute = ctx.quest_comp.info.env.time_lock_minute,
        }) catch {};
    }
    if (ctx.quest_comp.info.env.trial_roles.len > 0) {
        const is_temp_team = ctx.quest_comp.info.env.pre_trial_roles.len > 0;
        try applyTrialRolesWithOptions(ctx, ctx.quest_comp.info.env.trial_roles, true, is_temp_team, is_temp_team, &.{});
    }
    for (ctx.quest_comp.info.world.unlocked_map_marks) |mark_id| {
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.MapMarkShowIdInfoUpdateNotify{
            .ShowFlag = 1,
            .IsShow = true,
            .MarkId = mark_id,
            .NeedFocus = false,
        }) catch {};
    }
    for (ctx.quest_comp.info.world.hidden_map_marks) |mark_id| {
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.MapMarkShowIdInfoUpdateNotify{
            .ShowFlag = 0,
            .IsShow = false,
            .MarkId = mark_id,
            .NeedFocus = false,
        }) catch {};
    }
    // Re-apply persisted entity states & hidden entities onto newly loaded scene entities
    if (ctx.scene) |scene| {
        var combat_pack: std.ArrayList(pb.CombatReceiveData) = .empty;
        for (ctx.quest_comp.info.world.hidden_entities) |hid_cfg| {
            if (scene.findEntityNetIdByConfigId(hid_cfg)) |nid| {
                _ = try scene.setEntityVisibility(nid, false, &combat_pack, ctx.alloc.arena);
            }
        }
        for (ctx.quest_comp.info.world.hidden_groups) |grp| {
            for (grp.entity_ids) |eid| {
                if (scene.findEntityNetIdByConfigId(eid)) |nid| {
                    _ = try scene.setEntityVisibility(nid, false, &combat_pack, ctx.alloc.arena);
                }
            }
        }
        for (ctx.quest_comp.info.world.entity_states) |rec| {
            const cfg_id = std.math.cast(i32, rec.entity_id) orelse continue;
            if (scene.findEntityNetIdByConfigId(cfg_id)) |nid| {
                if (try scene.updateEntityStateTag(ctx.alloc.gpa, &ctx.assets.tables.gameplay_tag_remap, nid, rec.state_name)) |upd| {
                    try emitEntityStateTags(ctx, nid, upd);
                }
            }
        }
        if (ctx.progress_comp) |pc| {
            for (pc.info.dynamic.ints) |entry| {
                if (!std.mem.startsWith(u8, entry.key, "gravity:")) continue;
                const config_id = std.fmt.parseInt(i32, entry.key["gravity:".len..], 10) catch continue;
                if (config_id <= 0 or entry.val < 0 or entry.val > 3) continue;
                if (scene.findEntityNetIdByConfigId(config_id) != null) {
                    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.GravityFlipUpdateTypeNotify{
                        .NewGravityFlipType = @enumFromInt(entry.val),
                        .EntityId = config_id,
                    }) catch {};
                }
            }
            for (pc.info.dynamic.ints) |entry| {
                if (!std.mem.startsWith(u8, entry.key, "lift_target:")) continue;
                const config_id = std.fmt.parseInt(i32, entry.key["lift_target:".len..], 10) catch continue;
                if (config_id <= 0) continue;
                const net_id = scene.findEntityNetIdByConfigId(config_id) orelse continue;
                const idx = scene.net_id_map.get(net_id) orelse continue;
                const location = std.math.cast(i32, entry.val) orelse continue;
                scene.entities.items(.world_item)[idx].lift = .{ .location = location };
                ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.ElevatorMoveNotify{
                    .EntityId = net_id,
                    .Location = location,
                }) catch {};
            }
            const vehicle_id = std.math.cast(i32, pc.info.getKeyInt("vehicle:cur")) orelse 0;
            if (vehicle_id > 0) {
                ensureQuestEntitySpawned(ctx, vehicle_id);
                if (scene.findEntityNetIdByConfigId(vehicle_id)) |net_id| {
                    if (scene.net_id_map.get(net_id)) |index| {
                        if (scene.entities.items(.vehicle)[index]) |*vehicle| {
                            vehicle.occupied = true;
                            const seat = std.math.cast(i32, pc.info.getKeyInt("vehicle:seat")) orelse 0;
                            ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.VehicleUpdateNotify{
                                .EntityId = net_id,
                                .PlayerId = ctx.basic_comp.player_id,
                                .SeatId = seat,
                            }) catch {};
                        }
                    }
                }
            }
            try syncPersistedVehicleNpcLinks(scene, pc, ctx.conn, ctx.alloc, ctx.io, null);
        }
        if (combat_pack.items.len > 0) {
            ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.CombatReceivePackNotify{ .Data = combat_pack }) catch {};
        }
    }
    if (ctx.quest_comp.info.env.audio_states.len > 0) {
        var states: std.ArrayList(pb.AudioStatePb) = .empty;
        for (ctx.quest_comp.info.env.audio_states) |rec| {
            try states.append(ctx.alloc.arena, .{
                .Group = rec.group,
                .State = rec.state,
            });
        }
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.AudioStateUpdateNotify{ .AudioStates = states }) catch {};
    }
    for (ctx.quest_comp.info.world.active_guests) |gid| {
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.AddGuestNotify{ .GuestId = gid }) catch {};
    }
    if (ctx.map_comp.info.temp_teleports.len > 0) {
        var tp_list: std.ArrayList(pb.TemporaryTeleportInfo) = .empty;
        for (ctx.map_comp.info.temp_teleports) |tp| {
            try tp_list.append(ctx.alloc.arena, tp.toPb());
        }
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.TemporaryTeleportAllInfoNotify{
            .AllTemporaryTeleportInfo = tp_list,
        }) catch {};
    }
}

pub fn emitEntityStateTagUpdateChecked(
    scene: *const Scene,
    assets: ?*const Assets,
    conn: *Connection,
    gpa: std.mem.Allocator,
    arena: std.mem.Allocator,
    io: std.Io,
    net_id: i64,
    upd: Scene.StateTagUpdate,
) !void {
    if (tags_handler.hasClientLevelTagComponent(scene, assets, net_id)) {
        try emitEntityStateTagUpdate(conn, gpa, arena, io, net_id, upd);
        return;
    }
    if (upd.new_tag_id != 0) {
        conn.push(gpa, io, pb.EntityStateReadyNotify{
            .EntityId = net_id,
            .TagId = upd.new_tag_id,
            .Ready = true,
        }) catch {};
    }
}

pub fn emitEntityStateTagUpdate(
    conn: *Connection,
    gpa: std.mem.Allocator,
    arena: std.mem.Allocator,
    io: std.Io,
    net_id: i64,
    upd: Scene.StateTagUpdate,
) !void {
    var tag_notify: pb.EntityCommonTagNotify = .{ .Id = net_id };
    // CommonTagData field 2 is IsAdd (stale generated name RemoveTagIds): false removes, true adds.
    if (upd.old_tag_id != 0 and upd.old_tag_id != upd.new_tag_id) {
        try tag_notify.Tags.append(arena, .{ .TagId = upd.old_tag_id, .RemoveTagIds = false });
    }
    if (upd.new_tag_id != 0) {
        try tag_notify.Tags.append(arena, .{ .TagId = upd.new_tag_id, .RemoveTagIds = true });
    }
    if (tag_notify.Tags.items.len > 0) {
        conn.push(gpa, io, tag_notify) catch {};
    }
    if (upd.new_tag_id != 0) {
        conn.push(gpa, io, pb.EntityStateReadyNotify{
            .EntityId = net_id,
            .TagId = upd.new_tag_id,
            .Ready = true,
        }) catch {};
    }
}

fn emitEntityStateTags(ctx: QuestContext, net_id: i64, upd: Scene.StateTagUpdate) !void {
    if (ctx.scene) |scene| {
        try emitEntityStateTagUpdateChecked(scene, ctx.assets, ctx.conn, ctx.alloc.gpa, ctx.alloc.arena, ctx.io, net_id, upd);
    } else {
        try emitEntityStateTagUpdate(ctx.conn, ctx.alloc.gpa, ctx.alloc.arena, ctx.io, net_id, upd);
    }
}

fn applyChangeEntityState(ctx: QuestContext, callbacks: ActionCallbacks, params: std.json.ObjectMap) !void {
    const action_type = quest_types.jsonStr(params.get("Type")) orelse "Directly";
    if (std.mem.eql(u8, action_type, "BatchDirectly")) {
        const state_str = quest_types.jsonStr(params.get("State")) orelse return;
        const ids = try quest_types.parseIntList(ctx.alloc.arena, params.get("EntityIds"));
        for (ids) |eid| try applyEntityStateChange(ctx, callbacks, eid, state_str);
        return;
    }
    const entity_id = quest_types.jsonInt(params.get("EntityId")) orelse return;
    if (std.mem.eql(u8, action_type, "Loop")) {
        const cur = ctx.quest_comp.info.getEntityState(entity_id) orelse "";
        const next_state: []const u8 = if (std.mem.indexOf(u8, cur, "激活") != null)
            "关卡.Common.状态.常态"
        else
            "关卡.Common.状态.激活";
        try applyEntityStateChange(ctx, callbacks, entity_id, next_state);
        return;
    }
    const state_str = quest_types.jsonStr(params.get("State")) orelse return;
    try applyEntityStateChange(ctx, callbacks, entity_id, state_str);
}

fn applyChangeSelfEntityState(ctx: QuestContext, callbacks: ActionCallbacks, params: std.json.ObjectMap) !void {
    const scene = ctx.scene orelse return;
    const owner_nid = scene.active_flow_owner_entity_id;
    if (owner_nid == 0) return;
    const state_str = quest_types.jsonStr(params.get("EntityState")) orelse quest_types.jsonStr(params.get("State")) orelse return;
    const idx = scene.net_id_map.get(owner_nid) orelse return;
    const cfg_id = scene.entities.items(.config)[idx].config_id;
    try applyEntityStateChange(ctx, callbacks, cfg_id, state_str);
}

fn applySetEntityVisible(ctx: QuestContext, params: std.json.ObjectMap) !void {
    const visible = quest_types.jsonBool(params.get("Visible")) orelse true;
    const ids = try quest_types.parseIntList(ctx.alloc.arena, params.get("EntityIds"));
    try sendQuestEntityVisibility(ctx, ids, visible);
}

fn applyEntityVisibilityList(ctx: QuestContext, ids_opt: ?std.json.Value, visible: bool) !void {
    const ids = try quest_types.parseIntList(ctx.alloc.arena, ids_opt);
    try sendQuestEntityVisibility(ctx, ids, visible);
}

fn applyHideGroup(ctx: QuestContext, params: std.json.ObjectMap) !void {
    const group_key = quest_types.jsonStr(params.get("GroupKey")) orelse "";
    const cfg_val = params.get("HideConfig") orelse return;
    if (cfg_val != .object) return;
    const hide_ty = quest_types.jsonStr(cfg_val.object.get("Type")) orelse "";

    var entities_to_hide: std.ArrayList(i32) = .empty;
    var layers_to_unload: std.ArrayList(i32) = .empty;
    var lps_to_hide: std.ArrayList(i32) = .empty;

    if (std.mem.eql(u8, hide_ty, "SpecifyEntity")) {
        const ids = try quest_types.parseIntList(ctx.alloc.arena, cfg_val.object.get("EntityIds"));
        try entities_to_hide.appendSlice(ctx.alloc.arena, ids);
    } else if (std.mem.eql(u8, hide_ty, "SpecifyDataLayers")) {
        const layers = try quest_types.parseIntList(ctx.alloc.arena, cfg_val.object.get("DataLayers"));
        try layers_to_unload.appendSlice(ctx.alloc.arena, layers);
    } else if (std.mem.eql(u8, hide_ty, "SpecifyLevelPlay")) {
        const lps = try quest_types.parseIntList(ctx.alloc.arena, cfg_val.object.get("LevelPlays"));
        try lps_to_hide.appendSlice(ctx.alloc.arena, lps);
        for (lps) |lpid| {
            if (ctx.assets.tables.level_play_data.getDataById(lpid)) |lp| {
                if (lp.Data.LevelPlayEntityId > 0) try entities_to_hide.append(ctx.alloc.arena, lp.Data.LevelPlayEntityId);
            }
        }
    } else if (std.mem.eql(u8, hide_ty, "WorldEntityAndLevelPlay") or std.mem.eql(u8, hide_ty, "WorldMonsterAndMonsterTreasure")) {
        const only_m_and_c = std.mem.eql(u8, hide_ty, "WorldMonsterAndMonsterTreasure");
        const range_ids = try quest_types.parseIntList(ctx.alloc.arena, cfg_val.object.get("HideRangeEntities"));
        for (range_ids) |rid| {
            try collectConfigIdsInRangeEntity(ctx, rid, only_m_and_c, &entities_to_hide);
        }
    }

    if (group_key.len != 0) {
        try ctx.quest_comp.info.setHiddenGroup(ctx.alloc.gpa, group_key, entities_to_hide.items, layers_to_unload.items, lps_to_hide.items);
    }
    if (entities_to_hide.items.len > 0) try sendQuestEntityVisibility(ctx, entities_to_hide.items, false);
    if (layers_to_unload.items.len > 0) try applyDataLayerChange(ctx, &.{}, layers_to_unload.items);
}

fn applyShowHidedGroup(ctx: QuestContext, params: std.json.ObjectMap) !void {
    const group_key = quest_types.jsonStr(params.get("GroupKey")) orelse return;
    const rec = ctx.quest_comp.info.popHiddenGroup(ctx.alloc.gpa, group_key) orelse return;
    defer QuestInfo.freeHiddenGroupRecord(rec, ctx.alloc.gpa);

    if (rec.entity_ids.len > 0) try sendQuestEntityVisibility(ctx, rec.entity_ids, true);
    if (rec.data_layers.len > 0) try applyDataLayerChange(ctx, rec.data_layers, &.{});
}

fn applyHideTargetRange(ctx: QuestContext, params: std.json.ObjectMap) !void {
    const rid = quest_types.jsonInt(params.get("RangeEntity")) orelse return;
    const only_m_and_c = blk: {
        if (params.get("HideConfig")) |hc| {
            if (hc == .object) {
                if (quest_types.jsonStr(hc.object.get("Type"))) |t| {
                    break :blk std.mem.eql(u8, t, "WorldMonsterAndMonsterTreasure");
                }
            }
        }
        break :blk false;
    };
    var ids: std.ArrayList(i32) = .empty;
    try collectConfigIdsInRangeEntity(ctx, rid, only_m_and_c, &ids);
    const key = try std.fmt.allocPrint(ctx.alloc.arena, "__range_{d}", .{rid});
    try ctx.quest_comp.info.setHiddenGroup(ctx.alloc.gpa, key, ids.items, &.{}, &.{});
    if (ids.items.len > 0) try sendQuestEntityVisibility(ctx, ids.items, false);
}

fn applyShowTargetRange(ctx: QuestContext, params: std.json.ObjectMap) !void {
    const rids = try quest_types.parseIntList(ctx.alloc.arena, params.get("RangeEntities"));
    for (rids) |rid| {
        const key = try std.fmt.allocPrint(ctx.alloc.arena, "__range_{d}", .{rid});
        if (ctx.quest_comp.info.popHiddenGroup(ctx.alloc.gpa, key)) |rec| {
            defer QuestInfo.freeHiddenGroupRecord(rec, ctx.alloc.gpa);
            if (rec.entity_ids.len > 0) try sendQuestEntityVisibility(ctx, rec.entity_ids, true);
        }
    }
}

fn applyHideByRangeInFlow(ctx: QuestContext, params: std.json.ObjectMap) !void {
    var ids: std.ArrayList(i32) = .empty;
    const explicit = try quest_types.parseIntList(ctx.alloc.arena, params.get("EntityIds"));
    try ids.appendSlice(ctx.alloc.arena, explicit);

    if (ctx.scene) |scene| {
        if (parseVectorPos(params.get("Center"))) |center| {
            const radius = quest_types.jsonFloat(params.get("Radius")) orelse 2000;
            var net_ids: std.ArrayListUnmanaged(i64) = .empty;
            defer net_ids.deinit(ctx.alloc.gpa);
            try scene.collectEntitiesInRadius(ctx.alloc.gpa, center, radius, false, &net_ids);
            for (net_ids.items) |nid| {
                const idx = scene.net_id_map.get(nid) orelse continue;
                const cfg_id = scene.entities.items(.config)[idx].config_id;
                if (cfg_id > 0 and !QuestInfo.containsInt(ids.items, cfg_id)) try ids.append(ctx.alloc.arena, cfg_id);
            }
        }
    }
    try ctx.quest_comp.info.setHiddenGroup(ctx.alloc.gpa, "__flow_range", ids.items, &.{}, &.{});
    if (ids.items.len > 0) try sendQuestEntityVisibility(ctx, ids.items, false);
}

fn restoreAllHiddenGroups(ctx: QuestContext) !void {
    while (ctx.quest_comp.info.world.hidden_groups.len > 0) {
        const key_copy = try ctx.alloc.arena.dupe(u8, ctx.quest_comp.info.world.hidden_groups[0].group_key);
        const rec = ctx.quest_comp.info.popHiddenGroup(ctx.alloc.gpa, key_copy) orelse break;
        defer QuestInfo.freeHiddenGroupRecord(rec, ctx.alloc.gpa);
        if (rec.entity_ids.len > 0) try sendQuestEntityVisibility(ctx, rec.entity_ids, true);
        if (rec.data_layers.len > 0) try applyDataLayerChange(ctx, rec.data_layers, &.{});
    }
}

fn collectConfigIdsInRangeEntity(
    ctx: QuestContext,
    range_cfg_id: i32,
    only_monsters_and_chests: bool,
    out: *std.ArrayList(i32),
) !void {
    const scene = ctx.scene orelse return;
    const cfg = ctx.assets.findLevelEntity(scene.instance_id, range_cfg_id) orelse return;
    const center = Assets.AssetIndexes.entityWorldPos(cfg) orelse return;
    const rot = Assets.AssetIndexes.entityWorldRot(cfg);

    var shape: @import("../../data/tables/entity_components/Range.zig").ShapeConfig = .{ .Radius = 3000 };
    var comps = cfg.ComponentsData;
    if (ctx.assets.findTemplateConfig(cfg.BlueprintType)) |tpl| {
        tpl.ComponentsData.mergeInto(&comps);
    }
    if (comps.RangeComponent) |rc| {
        if (rc.Shape) |sh| shape = sh;
    }

    const slice = scene.entities.slice();
    for (slice.items(.config), slice.items(.position)) |ent_cfg, pos_opt| {
        if (only_monsters_and_chests and ent_cfg.entity_type != .monster and ent_cfg.entity_type != .scene_item) continue;
        const cid = ent_cfg.config_id;
        if (cid <= 0) continue;
        const pos = if (pos_opt) |p| p.location else continue;
        if (!ConditionEvaluator.isPosInsideRangeShape(pos, center, rot, shape, 3000.0, 0.0)) continue;
        if (!QuestInfo.containsInt(out.items, cid)) try out.append(ctx.alloc.arena, cid);
    }
}

fn setEntityLocked(ctx: QuestContext, eid: i32, locked: bool) !void {
    if (eid <= 0) return;
    if (locked) {
        _ = try QuestInfo.appendUniqueInt(&ctx.quest_comp.info.world.locked_entities, ctx.alloc.gpa, eid);
    } else {
        _ = try QuestInfo.removeIntValue(&ctx.quest_comp.info.world.locked_entities, ctx.alloc.gpa, eid);
    }
}

fn applyLockEntities(ctx: QuestContext, ids_opt: ?std.json.Value, locked: bool) !void {
    const ids = try quest_types.parseIntList(ctx.alloc.arena, ids_opt);
    for (ids) |eid| try setEntityLocked(ctx, eid, locked);
}

fn applyStopSceneItemMove(ctx: QuestContext, params: std.json.ObjectMap) !void {
    const ids = try quest_types.parseIntList(ctx.alloc.arena, params.get("EntityIds"));
    for (ids) |eid| {
        if (eid <= 0) continue;
        const quest_active_key = try std.fmt.allocPrint(ctx.alloc.arena, "quest_spline_active:{d}", .{eid});
        const item_active_key = try std.fmt.allocPrint(ctx.alloc.arena, "scene_item_spline_active:{d}", .{eid});
        try recordProgressInt(ctx, quest_active_key, 0, false);
        try recordProgressInt(ctx, item_active_key, 0, false);
        if (ctx.scene) |scene| {
            if (scene.findEntityNetIdByConfigId(eid)) |net_id| {
                if (scene.net_id_map.get(net_id)) |index| scene.entities.items(.world_item)[index].move_spline = null;
                pushSplineStatus(ctx, net_id, .Stop, 0, null, null);
            }
        }
    }
}

fn applyResetPlayerPosPresentation(ctx: QuestContext, params: std.json.ObjectMap) !void {
    const scene = ctx.scene orelse return;
    if (params.get("Presentation")) |pres| {
        if (pres == .object) {
            const cues = try quest_types.parseIntList(ctx.alloc.arena, pres.object.get("GamePlayCues"));
            for (cues) |cue_id| {
                ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.GameplayCueNotify{
                    .GameplayCueId = cue_id,
                }) catch {};
            }
        }
    }
    const rpc = params.get("ResetPosConfig") orelse return;
    if (rpc != .object) return;
    if (!std.mem.eql(u8, quest_types.jsonStr(rpc.object.get("Type")) orelse "", "PosEntity")) return;
    const eid = quest_types.jsonInt(rpc.object.get("Entity")) orelse return;
    const cfg = ctx.assets.findLevelEntity(scene.instance_id, eid) orelse return;
    const pos = Assets.AssetIndexes.entityWorldPos(cfg) orelse return;
    const rot = Assets.AssetIndexes.entityWorldRot(cfg);
    try updatePlayerSceneState(ctx, pos, rot);
    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.TeleportNotify{
        .MapId = scene.instance_id,
        .Pos = .{ .X = pos[0], .Y = pos[1], .Z = pos[2] },
        .Rot = .{ .X = rot[0], .Y = rot[1], .Z = rot[2] },
        .Reason = .Action,
    }) catch {};
}

fn isTrackedOrClientAction(name: []const u8) bool {
    const names = [_][]const u8{
        "BvbPlayDialog",
        "BvbPlayerOperationConstraint",
        "BvbSendSystemEvent",
        "ClientPreEnableSubLevels",
        "ClientSetSeasonState",
        "DisableEntityLookAt",
        "EnableAoiNotify",
        "EnableKey4Func",
        "EndCommonTip",
        "EnterOrbitalCamera",
        "ExitOrbitalCamera",
        "OpenQte",
        "OpenQuickHackSystem",
        "PlayerInput",
        "RemovePreloadResource",
        "ReportTrackingEvent",
        "ResumeSystemRewardPopup",
        "RunActorCustomEvent",
        "SetActorVar",
        "SetSportsState",
        "TrapDefenseChangeMiniMap",
        "TrapDefensePlayerOperationConstraint",
        "TriggerSpecificScanEffect",
        "UsePhantomSkill",
    };
    for (names) |candidate| {
        if (std.mem.eql(u8, name, candidate)) return true;
    }
    return false;
}

fn collectTrialRoleIds(arena: std.mem.Allocator, params: std.json.ObjectMap) !std.ArrayList(i32) {
    var ids: std.ArrayList(i32) = .empty;
    if (params.get("CharacterGroupNew")) |arr_val| {
        if (arr_val == .array) {
            for (arr_val.array.items) |entry| {
                if (entry != .object) continue;
                if (quest_types.jsonInt(entry.object.get("CharacterId"))) |cid| {
                    if (cid > 0) try ids.append(arena, cid);
                }
            }
        }
    }
    if (ids.items.len == 0) {
        try ids.appendSlice(arena, try quest_types.parseIntList(arena, params.get("CharacterGroup")));
    }
    return ids;
}

fn applyAddTrialCharacter(ctx: QuestContext, params: std.json.ObjectMap) !void {
    var ids = try collectTrialRoleIds(ctx.alloc.arena, params);
    if (ids.items.len == 0) {
        if (quest_types.jsonInt(params.get("CharacterId"))) |cid| {
            if (cid > 0) try ids.append(ctx.alloc.arena, cid);
        }
    }
    const auto_change = quest_types.jsonBool(params.get("AutoChange")) orelse false;
    const create_temp_team = quest_types.jsonBool(params.get("CreateTempTeam")) orelse false;
    const dungeon_ids = try quest_types.parseIntList(ctx.alloc.arena, params.get("DungeonList"));
    if (ids.items.len > 0) {
        try applyTrialRolesWithOptions(ctx, ids.items, true, create_temp_team, auto_change, dungeon_ids);
    }
}

fn applyRemoveTrialCharacter(ctx: QuestContext, params: std.json.ObjectMap) !void {
    var ids: std.ArrayList(i32) = .empty;
    const grp = try quest_types.parseIntList(ctx.alloc.arena, params.get("CharacterGroup"));
    try ids.appendSlice(ctx.alloc.arena, grp);
    if (ids.items.len == 0) {
        if (quest_types.jsonInt(params.get("CharacterId"))) |cid| {
            if (cid > 0) try ids.append(ctx.alloc.arena, cid);
        }
    }
    try removeTrialRoles(ctx, ids.items);
}

fn applyAreaTimeState(ctx: QuestContext, params: std.json.ObjectMap) !void {
    const cfg_val = params.get("SetAreaTimeConfig") orelse return;
    if (cfg_val != .object) return;
    const op_type = quest_types.jsonInt(cfg_val.object.get("Type")) orelse 0;
    const locked = (op_type == 0);
    var hour: i32 = ctx.quest_comp.info.env.time_lock_hour;
    var min: i32 = ctx.quest_comp.info.env.time_lock_minute;
    if (cfg_val.object.get("LockTime")) |lt| {
        if (lt == .object) {
            hour = quest_types.jsonInt(lt.object.get("Hour")) orelse hour;
            min = quest_types.jsonInt(lt.object.get("Minutes")) orelse (quest_types.jsonInt(lt.object.get("Minute")) orelse min);
        }
    }
    const areas = try quest_types.parseIntList(ctx.alloc.arena, cfg_val.object.get("AreaIds"));
    for (areas) |area_id| {
        try ctx.quest_comp.info.setAreaTime(ctx.alloc.gpa, area_id, hour, min, locked);
    }
    try applyTimeLockState(ctx, locked, hour, min);
}

fn applyPhoneSystemInteract(ctx: QuestContext, callbacks: ActionCallbacks, params: std.json.ObjectMap) !void {
    const itype = params.get("InteractType") orelse return;
    if (itype != .object) return;
    const kind = quest_types.jsonStr(itype.object.get("Type")) orelse "";

    if (std.mem.eql(u8, kind, "SendPhoneMessage")) {
        if (itype.object.get("PhoneRecordActionConfig")) |cfg| {
            if (cfg == .object) {
                if (quest_types.jsonInt(cfg.object.get("PhoneMessageId"))) |pmid| {
                    _ = try QuestInfo.appendUniqueInt(&ctx.quest_comp.info.progress.read_phone_messages, ctx.alloc.gpa, pmid);
                    if (callbacks.on_phone_message) |cb| try cb(ctx, pmid, 0);
                }
            }
        }
    } else if (std.mem.eql(u8, kind, "DeletePhoneMessage")) {
        if (quest_types.jsonInt(itype.object.get("PhoneMessageId"))) |pmid| {
            _ = try QuestInfo.removeIntValue(&ctx.quest_comp.info.progress.read_phone_messages, ctx.alloc.gpa, pmid);
        }
    } else if (quest_types.jsonInt(itype.object.get("TelecomId"))) |tid| {
        _ = try QuestInfo.appendUniqueInt(&ctx.quest_comp.info.progress.received_telecoms, ctx.alloc.gpa, tid);
        if (callbacks.on_phone_message) |cb| try cb(ctx, 0, tid);
    }
}

fn applySendNpcMail(ctx: QuestContext, callbacks: ActionCallbacks, mail_id: i32) !void {
    if (mail_id <= 0) return;
    const now_sec = @divFloor(std.Io.Clock.real.now(ctx.io).toMilliseconds(), std.time.ms_per_s);

    var notify: pb.MailInfosNotify = .default;
    if (ctx.mail_comp) |mail_comp| {
        const entry = try MailInfo.upsertNpcMail(&mail_comp.info, ctx.alloc.gpa, mail_id, now_sec);
        try mail_comp.save(ctx.alloc.arena, ctx.fs);
        try notify.MailInfos.append(ctx.alloc.arena, try MailInfo.entryToProto(entry.*, ctx.alloc.arena));
    } else {
        const id_str = try std.fmt.allocPrint(ctx.alloc.arena, "{d}", .{mail_id});
        try notify.MailInfos.append(ctx.alloc.arena, .{
            .Id = id_str,
            .ConfigId = mail_id,
            .ReceivedTime = now_sec,
            .State = 0,
            .ValidTime = 2_592_000,
            .ReadValidTime = 2_592_000,
            .ExpiryTime = now_sec + 2_592_000,
        });
    }
    ctx.conn.push(ctx.alloc.gpa, ctx.io, notify) catch {};

    if (callbacks.on_npc_mail) |cb| try cb(ctx, mail_id);
}

fn applyEnableLevelPlay(ctx: QuestContext, callbacks: ActionCallbacks, params: std.json.ObjectMap) !void {
    const cfgs = params.get("Configs") orelse return;
    if (cfgs != .array) return;
    for (cfgs.array.items) |entry| {
        if (entry != .object) continue;
        const lpid = quest_types.jsonInt(entry.object.get("LevelPlayId")) orelse continue;
        const enable = quest_types.jsonBool(entry.object.get("Enable")) orelse true;
        if (enable) {
            if (ctx.assets.tables.level_play_data.getDataById(lpid)) |lp| {
                if (lp.Data.LevelPlayEntityId > 0) ensureQuestEntitySpawned(ctx, lp.Data.LevelPlayEntityId);
            }
        }
        var kbuf: [48]u8 = undefined;
        if (std.fmt.bufPrint(&kbuf, "lp_state:{d}", .{lpid})) |key| {
            try recordProgressInt(ctx, key, if (enable) 2 else 0, false);
        } else |_| {}
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.LevelPlayStateNotify{
            .Id = lpid,
            .State = if (enable) 2 else 0,
            .CompleteNumber = 0,
        }) catch {};
        if (callbacks.on_level_play_enabled) |cb| try cb(ctx, lpid, enable);
    }
}

fn grantActionItems(ctx: QuestContext, callbacks: ActionCallbacks, items_opt: ?std.json.Value) !void {
    const arr = items_opt orelse return;
    if (arr != .array) return;

    var list: std.ArrayList(RewardService.RewardItem) = .empty;
    for (arr.array.items) |it| {
        if (it != .object) continue;
        const iid = quest_types.jsonInt(it.object.get("ItemId")) orelse continue;
        const cnt = quest_types.jsonInt(it.object.get("Count")) orelse 1;
        if (iid > 0 and cnt > 0) try list.append(ctx.alloc.arena, .{ .item_id = iid, .count = cnt });
    }
    if (list.items.len > 0) {
        _ = try RewardService.grantItems(quest_types.makeRewardContext(ctx), 0, false, list.items, 2);
        if (callbacks.on_item_gained) |cb| {
            for (list.items) |it| try cb(ctx, it.item_id);
        }
    }
}

fn destroyQuestItems(ctx: QuestContext, params: std.json.ObjectMap) !void {
    const iid = quest_types.jsonInt(params.get("ItemId")) orelse return;
    if (iid <= 0) return;
    const is_all = quest_types.jsonBool(params.get("IsAll")) orelse false;
    const owned = ctx.inventory_comp.info.normalItemCount(iid);
    if (owned <= 0) return;
    const to_take = if (is_all) owned else @min(owned, quest_types.jsonInt(params.get("Count")) orelse 1);
    if (to_take <= 0) return;

    if (try ctx.inventory_comp.info.consumeNormalItem(ctx.alloc.gpa, iid, to_take)) {
        const i_path = try std.fmt.allocPrint(ctx.alloc.arena, "player/{d}/{s}", .{ ctx.inventory_comp.player_id, InventoryInfo.data_path });
        try comp_util.saveStruct(ctx.fs, ctx.inventory_comp.info, i_path, ctx.alloc.arena);

        var updated: std.ArrayList(pb.NormalItem) = .empty;
        try updated.append(ctx.alloc.arena, .{
            .Id = iid,
            .Count = ctx.inventory_comp.info.normalItemCount(iid),
        });
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.NormalItemUpdateNotify{
            .NormalItemList = updated,
            .NoTips = false,
        }) catch {};
    }
}

fn applyQuestBuffAction(
    ctx: QuestContext,
    params: std.json.ObjectMap,
    is_player: bool,
    is_remove: bool,
) !void {
    const scene = ctx.scene orelse return;
    const buff_ids_val = params.get("BuffIds") orelse return;
    if (buff_ids_val != .array or buff_ids_val.array.items.len == 0) return;

    var targets: std.ArrayList(Scene.Entity) = .empty;
    if (is_player) {
        const slice = scene.entities.slice();
        for (slice.items(.config), slice.items(.entity_id), 0..) |cfg, nid, idx| {
            if (cfg.entity_type == .player) {
                try targets.append(ctx.alloc.arena, .{ .index = idx, .net_id = nid.net_id });
            }
        }
        if (targets.items.len == 0) {
            const role_nid = scene.currentRoleEntityId();
            const idx = scene.net_id_map.get(role_nid) orelse return;
            try targets.append(ctx.alloc.arena, .{ .index = idx, .net_id = role_nid });
        }
    } else {
        var cfg_ids: std.ArrayList(i32) = .empty;
        if (quest_types.jsonInt(params.get("EntityId"))) |eid| {
            if (eid > 0) try cfg_ids.append(ctx.alloc.arena, eid);
        }
        const multi = try quest_types.parseIntList(ctx.alloc.arena, params.get("EntityIds"));
        try cfg_ids.appendSlice(ctx.alloc.arena, multi);

        for (cfg_ids.items) |cfg_id| {
            if (!is_remove) ensureQuestEntitySpawned(ctx, cfg_id);
            const slice = scene.entities.slice();
            for (slice.items(.config), slice.items(.entity_id), 0..) |cfg, nid, idx| {
                if (cfg.config_id == cfg_id) {
                    try targets.append(ctx.alloc.arena, .{ .index = idx, .net_id = nid.net_id });
                    break;
                }
            }
        }
    }

    for (targets.items) |target_entity| {
        if (is_remove) {
            for (buff_ids_val.array.items) |v| {
                const raw_id = quest_types.jsonInt(v) orelse continue;
                if (raw_id > 0) {
                    try ctx.events.enqueue(.buff_removal_by_id, .{
                        .entity = target_entity,
                        .buff_id = raw_id,
                    });
                }
            }
        } else {
            var additions: std.ArrayList(Events.BuffAdditionEntry) = .empty;
            for (buff_ids_val.array.items) |v| {
                const raw_id = quest_types.jsonInt(v) orelse continue;
                if (raw_id > 0) {
                    try additions.append(ctx.alloc.arena, .{
                        .id = raw_id,
                        .stack_count = 1,
                        .is_active = true,
                    });
                }
            }
            if (additions.items.len > 0) {
                try ctx.events.enqueue(.buff_addition, .{
                    .target = target_entity,
                    .instigator = target_entity,
                    .buffs = additions.items,
                });
            }
        }
    }
}

fn applyEnableSystemAction(ctx: QuestContext, params: std.json.ObjectMap) !void {
    const sys_id = quest_types.jsonInt(params.get("SystemType")) orelse return;
    const enabled = quest_types.jsonBool(params.get("IsEnable")) orelse true;
    if (enabled) {
        _ = try ctx.quest_comp.info.unlockSystem(ctx.alloc.gpa, sys_id);
        try pushFuncOpen(ctx);
        return;
    }
    var notify: pb.FuncOpenNotify = .default;
    try notify.Func.append(ctx.alloc.arena, .{ .Id = sys_id, .Flag = 0 });
    ctx.conn.push(ctx.alloc.gpa, ctx.io, notify) catch {};
}

fn applyEnableFunctionAction(ctx: QuestContext, params: std.json.ObjectMap) !void {
    const ty = quest_types.jsonStr(params.get("Type")) orelse return;
    if (std.mem.eql(u8, ty, "InteractiveItems")) {
        const enable = quest_types.jsonBool(params.get("Enable")) orelse true;
        try recordProgressInt(ctx, "func:interactive_items", @intFromBool(enable), false);
        return;
    }
    if (std.mem.eql(u8, ty, "MotorRadioUI")) {
        const enable = if (params.get("DisplayMode")) |dm|
            (dm == .object and std.mem.eql(u8, quest_types.jsonStr(dm.object.get("Type")) orelse "", "EnableAll"))
        else
            (quest_types.jsonBool(params.get("Enable")) orelse true);
        try recordProgressInt(ctx, "func:motor_radio_ui", @intFromBool(enable), false);
        return;
    }
    if (std.mem.eql(u8, ty, "TeleportDungeon")) {
        const enable = quest_types.jsonBool(params.get("Enable")) orelse true;
        const dlist = try quest_types.parseIntList(ctx.alloc.arena, params.get("DungeonList"));
        if (dlist.len > 0) {
            for (dlist) |did| {
                const key = try std.fmt.allocPrint(ctx.alloc.arena, "func:tp_dungeon:{d}", .{did});
                try recordProgressInt(ctx, key, @intFromBool(enable), false);
            }
        } else {
            try recordProgressInt(ctx, "func:tp_dungeon_all", @intFromBool(enable), false);
        }
        return;
    }
    if (quest_types.jsonInt(params.get("FunctionId"))) |fid| {
        const enable = quest_types.jsonBool(params.get("Enable")) orelse true;
        const changed = if (enable)
            try ctx.quest_comp.info.unlockSystem(ctx.alloc.gpa, fid)
        else
            try QuestInfo.removeIntValue(&ctx.quest_comp.info.unlocked_systems, ctx.alloc.gpa, fid);
        if (changed) {
            try pushFuncOpen(ctx);
        }
    }
}

fn teleportPlayerFromAction(ctx: QuestContext, params: std.json.ObjectMap, action_id: i32, is_flow_action: bool) void {
    const scene = ctx.scene orelse return;
    const tp_cfg = params.get("TelePortConfig") orelse return;
    if (tp_cfg != .object) return;
    const tp_type = quest_types.jsonInt(tp_cfg.object.get("Type")) orelse 0;

    var pos_xyz: ?[3]f32 = null;
    var rot_xyz: [3]f32 = .{ 0, 0, 0 };

    if (tp_type == 0) {
        if (tp_cfg.object.get("TargetPos")) |tp| {
            if (tp == .object) {
                if (parseVectorPos(tp)) |vec| {
                    pos_xyz = vec;
                    rot_xyz[2] = quest_types.jsonFloat(tp.object.get("A")) orelse 0;
                }
            }
        }
    } else if (tp_type == 1) {
        if (tp_cfg.object.get("EntityIds")) |arr| {
            if (arr == .array and arr.array.items.len > 0) {
                if (quest_types.jsonInt(arr.array.items[0])) |eid| {
                    if (ctx.assets.findLevelEntity(scene.instance_id, eid)) |cfg| {
                        pos_xyz = Assets.AssetIndexes.entityWorldPos(cfg);
                        rot_xyz = Assets.AssetIndexes.entityWorldRot(cfg);
                    }
                }
            }
        }
    } else if (tp_type == 2) {
        if (quest_types.jsonInt(tp_cfg.object.get("GravityAndPosEntityId"))) |eid| {
            if (ctx.assets.findLevelEntity(scene.instance_id, eid)) |cfg| {
                pos_xyz = Assets.AssetIndexes.entityWorldPos(cfg);
                rot_xyz = Assets.AssetIndexes.entityWorldRot(cfg);
                const gres = GravityHelper.resolveEntityGravity(cfg, rot_xyz);
                if (gres.configured) {
                    const role_nid = scene.currentRoleEntityId();
                    if (scene.net_id_map.get(role_nid)) |ridx| {
                        scene.entities.items(.world_item)[ridx].gravity = .{ .location = gres.dir };
                    }
                }
            }
        }
    }

    const pos = pos_xyz orelse return;
    updatePlayerSceneState(ctx, pos, rot_xyz) catch return;

    var notify: pb.TeleportNotify = .{
        .MapId = scene.instance_id,
        .Pos = .{ .X = pos[0], .Y = pos[1], .Z = pos[2] },
        .Rot = .{ .X = rot_xyz[0], .Y = rot_xyz[1], .Z = rot_xyz[2] },
        .Reason = .Action,
    };
    if (is_flow_action and action_id > 0 and scene.active_flow_list_name.len != 0) {
        notify.Reason = .FlowAction;
        notify.GameCtx = .{
            .CtxType = .FlowActionCtx,
            .CtxInfo = .{
                .FlowAction = .{
                    .FlowListName = scene.active_flow_list_name,
                    .FlowId = scene.active_flow_id,
                    .StateId = scene.active_flow_state_id,
                    .ActionId = action_id,
                },
            },
        };
    }
    ctx.conn.push(ctx.alloc.gpa, ctx.io, notify) catch {};
}

fn emitVarChanged(
    ctx: QuestContext,
    callbacks: ActionCallbacks,
    owner_id: i32,
    var_name: []const u8,
    var_define: pb.VarDefinePb,
    is_global: bool,
) !void {
    if (is_global) {
        var entries: std.ArrayList(pb.MapEntry([]const u8, pb.VarDefinePb)) = .empty;
        try entries.append(ctx.alloc.arena, .{ .key = var_name, .value = var_define });
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.PlayerVarNotify{
            .VarInfos = entries,
        }) catch {};
    }
    if (owner_id > 0) {
        if (callbacks.deferred_bt_vars) |deferred| {
            const name_dup = ctx.alloc.arena.dupe(u8, var_name) catch var_name;
            var def_dup = var_define;
            if (def_dup.Value) |v| {
                if (v == .String) {
                    def_dup.Value = .{ .String = ctx.alloc.arena.dupe(u8, v.String) catch v.String };
                }
            }
            deferred.append(ctx.alloc.arena, .{
                .owner_id = owner_id,
                .var_name = name_dup,
                .var_define = def_dup,
            }) catch {};
        } else {
            ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.BtVarUpdateNotify{
                .TreeOwnerId = owner_id,
                .TreeIncId = owner_id,
                .VarName = var_name,
                .VarDefine = var_define,
            }) catch {};
        }
    }
    if (callbacks.on_var_changed) |cb| {
        try cb(ctx, owner_id, var_name);
    }
}

fn persistGlobalVarInt(ctx: QuestContext, name: []const u8, val: i64) void {
    const pc = ctx.progress_comp orelse return;
    const key = std.fmt.allocPrint(ctx.alloc.arena, "var:{s}", .{name}) catch return;
    pc.info.setKeyInt(ctx.alloc.gpa, key, val) catch {};
    pc.save(ctx.fs, ctx.alloc.arena) catch {};
}

fn persistGlobalVarStr(ctx: QuestContext, name: []const u8, val: []const u8) void {
    const pc = ctx.progress_comp orelse return;
    const key = std.fmt.allocPrint(ctx.alloc.arena, "var:{s}", .{name}) catch return;
    pc.info.setKeyStr(ctx.alloc.gpa, key, val) catch {};
    pc.save(ctx.fs, ctx.alloc.arena) catch {};
}

fn applySetVar(ctx: QuestContext, callbacks: ActionCallbacks, owner_id: i32, params: std.json.ObjectMap) !void {
    const left = params.get("VarLeft") orelse return;
    if (left != .object) return;
    const base_name = quest_types.resolveVarName(left.object) orelse return;
    const right = params.get("VarRight") orelse return;
    if (right != .object) return;

    const src = quest_types.jsonStr(left.object.get("Source")) orelse "Self";
    const is_global = std.mem.eql(u8, src, "Global");
    const store_key: []const u8 = if (!is_global)
        (quest_types.scopedVarKeyWithOwnerAlloc(ctx.alloc.arena, ctx.scene, owner_id, left.object, base_name) orelse base_name)
    else
        base_name;

    if (right.object.get("Value")) |val| {
        switch (val) {
            .bool => |b| {
                try ctx.quest_comp.info.setVarBool(ctx.alloc.gpa, store_key, b);
                try ctx.quest_comp.info.setVarInt(ctx.alloc.gpa, store_key, if (b) 1 else 0);
                if (is_global) persistGlobalVarInt(ctx, base_name, if (b) 1 else 0);
                try emitVarChanged(ctx, callbacks, owner_id, base_name, .{
                    .VarType = 0,
                    .Value = .{ .Boolean = b },
                }, is_global);
            },
            .integer => |i| {
                const iv = std.math.cast(i32, i) orelse @as(i32, @intCast(std.math.clamp(i, std.math.minInt(i32), std.math.maxInt(i32))));
                try ctx.quest_comp.info.setVarInt(ctx.alloc.gpa, store_key, iv);
                if (is_global) persistGlobalVarInt(ctx, base_name, i);
                try emitVarChanged(ctx, callbacks, owner_id, base_name, .{
                    .VarType = 1,
                    .Value = .{ .Int = i },
                }, is_global);
            },
            .float => |f| {
                const iv: i32 = @intFromFloat(std.math.clamp(f, @as(f64, @floatFromInt(std.math.minInt(i32))), @as(f64, @floatFromInt(std.math.maxInt(i32)))));
                try ctx.quest_comp.info.setVarInt(ctx.alloc.gpa, store_key, iv);
                if (is_global) persistGlobalVarInt(ctx, base_name, @intFromFloat(f * 1000.0));
                try emitVarChanged(ctx, callbacks, owner_id, base_name, .{
                    .VarType = 3,
                    .Value = .{ .Float = @floatCast(f) },
                }, is_global);
            },
            .string => |s| {
                try ctx.quest_comp.info.setVarStr(ctx.alloc.gpa, store_key, s);
                if (is_global) persistGlobalVarStr(ctx, base_name, s);
                try emitVarChanged(ctx, callbacks, owner_id, base_name, .{
                    .VarType = 2,
                    .Value = .{ .String = s },
                }, is_global);
            },
            else => {},
        }
    } else if (quest_types.resolveVarName(right.object)) |src_name| {
        const src_type = quest_types.jsonStr(right.object.get("Type")) orelse "";
        if (std.mem.eql(u8, src_type, "String")) {
            const sk = quest_types.scopedVarKeyAlloc(ctx.alloc.arena, right.object, src_name) orelse src_name;
            const sv = blk: {
                const s1 = ctx.quest_comp.info.getVarStr(sk);
                break :blk if (s1.len > 0) s1 else ctx.quest_comp.info.getVarStr(src_name);
            };
            try ctx.quest_comp.info.setVarStr(ctx.alloc.gpa, store_key, sv);
            if (is_global) persistGlobalVarStr(ctx, base_name, sv);
            try emitVarChanged(ctx, callbacks, owner_id, base_name, .{
                .VarType = 2,
                .Value = .{ .String = sv },
            }, is_global);
        } else {
            const iv = quest_types.resolveVarIntForOwner(ctx, owner_id, right);
            try ctx.quest_comp.info.setVarInt(ctx.alloc.gpa, store_key, iv);
            if (is_global) persistGlobalVarInt(ctx, base_name, iv);
            try emitVarChanged(ctx, callbacks, owner_id, base_name, .{
                .VarType = 1,
                .Value = .{ .Int = iv },
            }, is_global);
        }
    }
}

fn applyCalculateVar(ctx: QuestContext, callbacks: ActionCallbacks, owner_id: i32, params: std.json.ObjectMap) !void {
    const res_obj = params.get("Result") orelse return;
    if (res_obj != .object) return;
    const base_name = quest_types.resolveVarName(res_obj.object) orelse return;
    const src = quest_types.jsonStr(res_obj.object.get("Source")) orelse "Self";
    const is_global = std.mem.eql(u8, src, "Global");
    const store_key: []const u8 = if (!is_global)
        (quest_types.scopedVarKeyWithOwnerAlloc(ctx.alloc.arena, ctx.scene, owner_id, res_obj.object, base_name) orelse base_name)
    else
        base_name;

    const v1 = quest_types.resolveVarIntForOwner(ctx, owner_id, params.get("Var1"));
    const v2 = quest_types.resolveVarIntForOwner(ctx, owner_id, params.get("Var2"));
    const op = quest_types.jsonStr(params.get("Op")) orelse "Add";

    const out: i32 = if (std.mem.eql(u8, op, "Add"))
        v1 +| v2
    else if (std.mem.eql(u8, op, "Sub"))
        v1 -| v2
    else if (std.mem.eql(u8, op, "Mul") or std.mem.eql(u8, op, "Mut"))
        v1 *| v2
    else if (std.mem.eql(u8, op, "Div") and v2 != 0)
        (if (v1 == std.math.minInt(i32) and v2 == -1) std.math.maxInt(i32) else @divTrunc(v1, v2))
    else
        v1;

    try ctx.quest_comp.info.setVarInt(ctx.alloc.gpa, store_key, out);
    if (is_global) persistGlobalVarInt(ctx, base_name, out);
    try emitVarChanged(ctx, callbacks, owner_id, base_name, .{
        .VarType = 1,
        .Value = .{ .Int = out },
    }, is_global);
}

/// Clears any persisted hidden state and guarantees the quest/flow entity is spawned in the active scene.
pub fn ensureQuestEntityAwakened(ctx: QuestContext, entity_config_id: i32) void {
    if (entity_config_id <= 0) return;
    _ = QuestInfo.appendUniqueInt(&ctx.quest_comp.info.world.shown_entities, ctx.alloc.gpa, entity_config_id) catch {};
    _ = QuestInfo.removeIntValue(&ctx.quest_comp.info.world.hidden_entities, ctx.alloc.gpa, entity_config_id) catch {};
    ensureQuestEntitySpawned(ctx, entity_config_id);
}

fn applyAwakeEntity(ctx: QuestContext, params: std.json.ObjectMap) !void {
    var ids: std.ArrayList(i32) = .empty;
    if (quest_types.jsonInt(params.get("EntityId"))) |eid| {
        if (eid > 0) try ids.append(ctx.alloc.arena, eid);
    }
    const list_ids = try quest_types.parseIntList(ctx.alloc.arena, params.get("EntityIds"));
    for (list_ids) |eid| {
        if (eid > 0 and !QuestInfo.containsInt(ids.items, eid)) {
            try ids.append(ctx.alloc.arena, eid);
        }
    }
    for (ids.items) |eid| {
        ensureQuestEntityAwakened(ctx, eid);
    }
    if (ids.items.len > 0) {
        try sendQuestEntityVisibility(ctx, ids.items, true);
    }
}

fn spawnConfigIdList(ctx: QuestContext, ids_opt: ?std.json.Value) void {
    const ids_val = ids_opt orelse return;
    if (ids_val != .array) return;
    for (ids_val.array.items) |v| {
        const eid = quest_types.jsonInt(v) orelse continue;
        if (eid > 0) {
            _ = QuestInfo.appendUniqueInt(&ctx.quest_comp.info.world.shown_entities, ctx.alloc.gpa, eid) catch {};
            _ = QuestInfo.removeIntValue(&ctx.quest_comp.info.world.hidden_entities, ctx.alloc.gpa, eid) catch {};
        }
        ensureQuestEntitySpawned(ctx, eid);
    }
}

pub fn ensureQuestEntitySpawned(ctx: QuestContext, entity_config_id: i32) void {
    const scene = ctx.scene orelse return;
    if (entity_config_id <= 0) return;
    if (ctx.quest_comp.info.hasOpenedChest(entity_config_id)) return;
    if (QuestInfo.containsInt(ctx.quest_comp.info.world.hidden_entities, entity_config_id)) return;
    if (ctx.assets.findLevelEntity(scene.instance_id, entity_config_id)) |entity_cfg| {
        if (!entityDataLayersActive(ctx.assets, ctx.quest_comp.info, scene.instance_id, entity_cfg)) return;
    }
    const slice = scene.entities.slice();
    for (slice.items(.config)) |cfg| {
        if (cfg.config_id == entity_config_id) return;
    }
    prologue_quest.spawnQuestEntity(
        ctx.events,
        scene,
        ctx.fs,
        ctx.assets,
        ctx.conn,
        ctx.basic_comp,
        ctx.alloc,
        ctx.io,
        @intCast(entity_config_id),
    );
    if (scene.findEntityNetIdByConfigId(entity_config_id)) |nid| {
        if (ctx.progress_comp) |pc| {
            var kbuf: [48]u8 = undefined;
            const xk = std.fmt.bufPrint(&kbuf, "world_pos:{d}:x", .{entity_config_id}) catch "";
            const xr = if (xk.len > 0) pc.info.getKeyInt(xk) else 0;
            const yk = std.fmt.bufPrint(&kbuf, "world_pos:{d}:y", .{entity_config_id}) catch "";
            const yr = if (yk.len > 0) pc.info.getKeyInt(yk) else 0;
            const zk = std.fmt.bufPrint(&kbuf, "world_pos:{d}:z", .{entity_config_id}) catch "";
            const zr = if (zk.len > 0) pc.info.getKeyInt(zk) else 0;
            if (xr != 0 or yr != 0 or zr != 0) {
                const fx: f32 = @bitCast(@as(u32, @truncate(@as(u64, @bitCast(xr)))));
                const fy: f32 = @bitCast(@as(u32, @truncate(@as(u64, @bitCast(yr)))));
                const fz: f32 = @bitCast(@as(u32, @truncate(@as(u64, @bitCast(zr)))));
                _ = scene.setEntityPosition(nid, .{ fx, fy, fz }, null);
            }
            const ck = std.fmt.bufPrint(&kbuf, "world_camp:{d}", .{entity_config_id}) catch "";
            if (ck.len > 0) {
                const cval = pc.info.getKeyInt(ck);
                if (cval > 0) {
                    if (scene.net_id_map.get(nid)) |eidx| {
                        scene.entities.items(.config)[eidx].camp = std.math.cast(u8, cval) orelse scene.entities.items(.config)[eidx].camp;
                    }
                }
            }
        }
        if (ctx.quest_comp.info.getEntityState(entity_config_id)) |saved_state| {
            if (saved_state.len != 0) {
                if (scene.updateEntityStateTag(ctx.alloc.gpa, &ctx.assets.tables.gameplay_tag_remap, nid, saved_state) catch null) |upd| {
                    if (upd.old_tag_id != upd.new_tag_id) {
                        emitEntityStateTagUpdateChecked(scene, ctx.assets, ctx.conn, ctx.alloc.gpa, ctx.alloc.arena, ctx.io, nid, upd) catch {};
                    }
                }
            }
        }
    }
}

fn applyDestroyEntity(ctx: QuestContext, params: std.json.ObjectMap) !void {
    var ids: std.ArrayList(i32) = .empty;
    if (quest_types.jsonInt(params.get("EntityId"))) |eid| {
        if (eid > 0) try ids.append(ctx.alloc.arena, eid);
    }
    const list_ids = try quest_types.parseIntList(ctx.alloc.arena, params.get("EntityIds"));
    for (list_ids) |eid| {
        if (eid > 0 and !QuestInfo.containsInt(ids.items, eid)) {
            try ids.append(ctx.alloc.arena, eid);
        }
    }
    if (ids.items.len == 0) return;

    for (ids.items) |target_cfg| {
        _ = try QuestInfo.appendUniqueInt(&ctx.quest_comp.info.world.hidden_entities, ctx.alloc.gpa, target_cfg);
        _ = try QuestInfo.removeIntValue(&ctx.quest_comp.info.world.shown_entities, ctx.alloc.gpa, target_cfg);
    }

    const scene = ctx.scene orelse return;
    var to_remove: std.ArrayList(i64) = .empty;
    const slice = scene.entities.slice();
    for (ids.items) |target_cfg| {
        for (slice.items(.config), slice.items(.entity_id)) |cfg, nid| {
            if (cfg.config_id == target_cfg) {
                to_remove.append(ctx.alloc.arena, nid.net_id) catch {};
            }
        }
    }
    if (to_remove.items.len == 0) return;

    var remove_infos: std.ArrayList(pb.EntityRemoveInfo) = .empty;
    for (to_remove.items) |net_id| {
        try scene.remove(ctx.alloc.gpa, ctx.fs, net_id);
        try remove_infos.append(ctx.alloc.arena, .{
            .EntityId = net_id,
            .Type = .RemoveTypeNormal,
        });
    }
    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.EntityRemoveNotify{
        .RemoveInfos = remove_infos,
        .IsRemove = true,
    }) catch {};
}

fn destroyConfigIdList(ctx: QuestContext, ids_opt: ?std.json.Value) void {
    const ids_val = ids_opt orelse return;
    if (ids_val != .array) return;
    var map = std.json.ObjectMap.init(ctx.alloc.arena);
    map.put("EntityIds", ids_val) catch return;
    applyDestroyEntity(ctx, map) catch {};
}

fn parseVectorPos(val_opt: ?std.json.Value) ?[3]f32 {
    const v = val_opt orelse return null;
    if (v != .object) return null;
    const x = quest_types.jsonFloat(v.object.get("X")) orelse return null;
    const y = quest_types.jsonFloat(v.object.get("Y")) orelse return null;
    const z = quest_types.jsonFloat(v.object.get("Z")) orelse 0;
    return .{ x, y, z };
}

fn pushEntityMoved(ctx: QuestContext, net_id: i64, pos: [3]f32, yaw_opt: ?f32) !void {
    var samples: std.ArrayList(pb.MoveReplaySample) = .empty;
    try samples.append(ctx.alloc.arena, .{
        .Location = .{ .X = pos[0], .Y = pos[1], .Z = pos[2] },
        .Rotation = if (yaw_opt) |yaw| .{ .Yaw = yaw } else null,
    });
    var entities: std.ArrayList(pb.MovingEntityData) = .empty;
    try entities.append(ctx.alloc.arena, .{
        .EntityId = net_id,
        .Originator = net_id,
        .MoveInfos = samples,
        .ForcePush = true,
    });
    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.MovePackageNotify{ .MovingEntities = entities }) catch {};
}

fn pushSplineStatus(
    ctx: QuestContext,
    net_id: i64,
    status: pb.MoveStatus,
    spline_entity_id: i32,
    runtime_data: ?pb.SceneItemSplineRuntimeData,
    move_spline_config: ?pb.MoveSplineConfig,
) void {
    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.MoveSplineStatusNotify{
        .SceneItemRuntimeData = runtime_data,
        .MoveStatus = status,
        .EntityId = net_id,
        .SplineEntityId = spline_entity_id,
        .MoveSplineConfig = move_spline_config,
    }) catch {};
}

fn moveSplineConfigFromAction(params: std.json.ObjectMap) ?pb.MoveSplineConfig {
    var config: pb.MoveSplineConfig = .{};
    var has_config = false;
    if (quest_types.jsonInt(params.get("StartPointIndex"))) |value| {
        if (std.math.cast(i32, value)) |index| {
            config.StartPoint = .{ .StartPointIndex = index };
            has_config = true;
        }
    }
    if (quest_types.jsonInt(params.get("EndPointIndex"))) |value| {
        if (std.math.cast(i32, value)) |index| {
            config.EndPoint = .{ .EndPointIndex = index };
            has_config = true;
        }
    }
    var look_dir = quest_types.jsonBool(params.get("IsLookDir"));
    if (look_dir == null) {
        if (params.get("MoveTarget")) |target| {
            if (target == .object) look_dir = quest_types.jsonBool(target.object.get("IsLookDir"));
        }
    }
    if (look_dir) |enabled| {
        config.LookDir = .{ .IsLookDir = enabled };
        has_config = true;
    }
    return if (has_config) config else null;
}

fn applySetAudioState(ctx: QuestContext, quest_id: i32, params: std.json.ObjectMap) !void {
    const cfg = params.get("AudioConfig") orelse return;
    if (cfg != .object) return;
    const group = quest_types.jsonStr(cfg.object.get("Group")) orelse return;
    const state = quest_types.jsonStr(cfg.object.get("State")) orelse return;
    try ctx.quest_comp.info.setAudioState(ctx.alloc.gpa, group, state);
    var states: std.ArrayList(pb.AudioStatePb) = .empty;
    try states.append(ctx.alloc.arena, .{
        .TreeOwnerId = quest_id,
        .TreeIncId = quest_id,
        .Group = group,
        .State = state,
    });
    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.AudioStateUpdateNotify{ .AudioStates = states }) catch {};
}

fn applyAddPlayBubble(ctx: QuestContext, params: std.json.ObjectMap, guid: []const u8) !void {
    const scene = ctx.scene orelse return;
    if (guid.len == 0) return;
    const ids = try quest_types.parseIntList(ctx.alloc.arena, params.get("EntityIds"));
    for (ids) |cfg_id| {
        if (cfg_id <= 0) continue;
        try scene.addPlayBubble(ctx.alloc.gpa, cfg_id, guid);
        if (scene.findEntityNetIdByConfigId(cfg_id)) |nid| {
            ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.EntityAddBubbleNotify{
                .EntityId = nid,
                .ActionGuid = guid,
            }) catch {};
        }
    }
}

fn applyClearPlayBubble(ctx: QuestContext, params: std.json.ObjectMap) !void {
    const scene = ctx.scene orelse return;
    const ids = try quest_types.parseIntList(ctx.alloc.arena, params.get("EntityIds"));
    for (ids) |cfg_id| {
        if (cfg_id <= 0) continue;
        const removed = try scene.takePlayBubblesForEntity(ctx.alloc.arena, cfg_id);
        const net_id = scene.findEntityNetIdByConfigId(cfg_id);
        for (removed) |*rec| {
            defer rec.deinit(ctx.alloc.gpa);
            if (net_id) |nid| {
                ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.EntityRemoveBubbleNotify{
                    .EntityId = nid,
                    .ActionGuid = rec.action_guid,
                }) catch {};
            }
        }
    }
}

fn applyAddFlowInteractOption(ctx: QuestContext, quest_id: i32, node_id: i32, params: std.json.ObjectMap) !void {
    const scene = ctx.scene orelse return;
    const cfg_id = quest_types.jsonInt(params.get("EntityId")) orelse return;
    if (cfg_id <= 0) return;
    const net_id = scene.findEntityNetIdByConfigId(cfg_id) orelse return;

    const opt_val = params.get("Option") orelse return;
    if (opt_val != .object) return;
    const guid = quest_types.jsonStr(opt_val.object.get("Guid")) orelse return;
    if (guid.len == 0) return;
    const text = quest_types.jsonStr(opt_val.object.get("TidContent")) orelse "";

    var kind: Scene.DynamicInteractOption.Kind = .none;
    var flow_name: []const u8 = "";
    var flow_id: i32 = 0;
    var state_id: i32 = 1;
    var actions_json: []const u8 = "";

    if (opt_val.object.get("Type")) |ty_val| {
        if (ty_val == .object) {
            const ty_name = quest_types.jsonStr(ty_val.object.get("Type")) orelse "";
            if (std.mem.eql(u8, ty_name, "Flow")) {
                if (ty_val.object.get("Flow")) |fv| {
                    if (fv == .object) {
                        flow_name = quest_types.jsonStr(fv.object.get("FlowListName")) orelse "";
                        flow_id = quest_types.jsonInt(fv.object.get("FlowId")) orelse 0;
                        state_id = quest_types.jsonInt(fv.object.get("StateId")) orelse 1;
                        if (flow_name.len > 0 and flow_id > 0) kind = .flow;
                    }
                }
            } else if (std.mem.eql(u8, ty_name, "Actions")) {
                kind = .actions;
                if (ty_val.object.get("Actions")) |av| {
                    if (av == .array and av.array.items.len > 0) {
                        actions_json = try std.json.Stringify.valueAlloc(ctx.alloc.arena, av, .{});
                    } else {
                        actions_json = "[]";
                    }
                } else {
                    actions_json = "[]";
                }
            }
        }
    }

    if (kind == .none) {
        kind = .actions;
        actions_json = "[]";
    }

    try scene.upsertDynamicInteractOption(
        ctx.alloc.gpa,
        guid,
        net_id,
        cfg_id,
        quest_id,
        node_id,
        kind,
        flow_name,
        flow_id,
        state_id,
        actions_json,
    );

    const game_ctx: pb.GameCtxPb = if (quest_id > 0) .{
        .CtxType = .BehaviorTree,
        .CtxInfo = .{ .BehaviorTree = .{
            .IncId = quest_id,
            .BtType = .BtTypeQuest,
            .BtId = quest_id,
            .NodeId = node_id,
        } },
    } else .{
        .CtxType = .Entity,
        .CtxInfo = .{ .Entity = .{ .ConfigId = cfg_id, .IncId = net_id } },
    };

    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.EntityAddDynamicInteractNotify{
        .GameCtx = game_ctx,
        .EntityId = net_id,
        .DelayRemove = false,
        .Text = text,
        .OptionGuid = guid,
    }) catch {};
}

fn applyRemoveFlowInteractOption(ctx: QuestContext, params: std.json.ObjectMap) !void {
    const scene = ctx.scene orelse return;
    const cfg_id = quest_types.jsonInt(params.get("EntityId")) orelse 0;
    var flow_name_opt: ?[]const u8 = null;
    var flow_id_opt: ?i32 = null;
    if (params.get("Flow")) |fv| {
        if (fv == .object) {
            flow_name_opt = quest_types.jsonStr(fv.object.get("FlowListName"));
            flow_id_opt = quest_types.jsonInt(fv.object.get("FlowId"));
        }
    }
    const removed = try scene.takeMatchingDynamicInteractOptions(
        ctx.alloc.arena,
        cfg_id,
        flow_name_opt,
        flow_id_opt,
    );
    for (removed) |*opt| {
        defer opt.deinit(ctx.alloc.gpa);
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.EntityRemoveDynamicInteractNotify{
            .OptionGuid = opt.option_guid,
            .EntityId = opt.entity_net_id,
        }) catch {};
    }
}

fn applySetEntityPos(ctx: QuestContext, params: std.json.ObjectMap) !void {
    const scene = ctx.scene orelse return;
    const cfg_id = quest_types.jsonInt(params.get("EntityId")) orelse return;
    const pos = parseVectorPos(params.get("Pos")) orelse return;
    const yaw_opt = if (params.get("Pos")) |pv| if (pv == .object) quest_types.jsonFloat(pv.object.get("A")) else null else null;
    const net_id = scene.findEntityNetIdByConfigId(cfg_id) orelse return;
    if (!scene.setEntityPosition(net_id, pos, yaw_opt)) return;
    try pushEntityMoved(ctx, net_id, pos, yaw_opt);
    persistQuestEntityPosition(ctx, cfg_id, net_id, pos) catch {};
}

fn applyResetEntityPos(ctx: QuestContext, params: std.json.ObjectMap) !void {
    const scene = ctx.scene orelse return;
    const cfg_id = quest_types.jsonInt(params.get("EntityId")) orelse return;
    const cfg = ctx.assets.findLevelEntity(scene.instance_id, cfg_id) orelse return;
    const pos = Assets.AssetIndexes.entityWorldPos(cfg) orelse return;
    if (scene.findEntityNetIdByConfigId(cfg_id)) |net_id| {
        try pushEntityMoved(ctx, net_id, pos, null);
        persistQuestEntityPosition(ctx, cfg_id, net_id, pos) catch {};
    }
    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.EntityPosResetNotify{ .EntityId = cfg_id }) catch {};
}

fn applyChangeEntityCamp(ctx: QuestContext, params: std.json.ObjectMap) !void {
    const scene = ctx.scene orelse return;
    const cfg_id = quest_types.jsonInt(params.get("TargetEntity")) orelse
        quest_types.jsonInt(params.get("EntityId")) orelse return;
    const camp = quest_types.jsonInt(params.get("Camp")) orelse return;
    if (ctx.progress_comp) |pc| {
        const key = try std.fmt.allocPrint(ctx.alloc.arena, "world_camp:{d}", .{cfg_id});
        try pc.info.setKeyInt(ctx.alloc.gpa, key, camp);
        try pc.save(ctx.fs, ctx.alloc.arena);
    }
    const net_id = scene.findEntityNetIdByConfigId(cfg_id) orelse return;
    if (scene.net_id_map.get(net_id)) |idx| {
        if (std.math.cast(u8, camp)) |c8| {
            scene.entities.items(.config)[idx].camp = c8;
        }
    }
    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.ModifyEntityCampNotify{
        .TargetEntityId = net_id,
        .Camp = camp,
    }) catch {};
}

fn applyGuestCharacter(ctx: QuestContext, params: std.json.ObjectMap, add: bool) !void {
    const gid = quest_types.jsonInt(params.get("GuestCharacterId")) orelse
        quest_types.jsonInt(params.get("GuestId")) orelse return;
    if (gid <= 0) return;
    if (add) {
        _ = try QuestInfo.appendUniqueInt(&ctx.quest_comp.info.world.active_guests, ctx.alloc.gpa, gid);
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.AddGuestNotify{ .GuestId = gid }) catch {};
    } else {
        _ = try QuestInfo.removeIntValue(&ctx.quest_comp.info.world.active_guests, ctx.alloc.gpa, gid);
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.RemoveGuestNotify{ .GuestId = gid }) catch {};
    }
}

fn applyUpdateTrialCharacter(ctx: QuestContext, params: std.json.ObjectMap) !void {
    const ids = try collectTrialRoleIds(ctx.alloc.arena, params);
    if (ids.items.len == 0 and ctx.quest_comp.info.env.trial_roles.len == 0) return;
    const target_ids: []const i32 = if (ids.items.len > 0) ids.items else ctx.quest_comp.info.env.trial_roles;
    const auto_change = quest_types.jsonBool(params.get("AutoChange")) orelse false;
    const enable_map = quest_types.jsonBool(params.get("EnableMapAndTeleport")) orelse true;
    const dungeon_ids = try quest_types.parseIntList(ctx.alloc.arena, params.get("DungeonList"));
    try applyTrialRolesWithOptions(ctx, target_ids, enable_map, false, auto_change, dungeon_ids);
}

fn applyExecResurrection(ctx: QuestContext, params: std.json.ObjectMap) !void {
    const revive_id = quest_types.jsonInt(params.get("ReviveId")) orelse 1;
    _ = try recordProgressAdd(ctx, "combat:revive_cnt", 1);
    if (ctx.scene) |scene| {
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.PlayerReviveNotify{
            .PlayerId = scene.player_id,
            .ReviveType = revive_id,
            .TeleportId = scene.instance_id,
        }) catch {};
    }
}

fn applyGuideTrigger(ctx: QuestContext, callbacks: ActionCallbacks, params: std.json.ObjectMap) !void {
    const gid = quest_types.jsonInt(params.get("GuideId")) orelse return;
    if (gid <= 0) return;
    if (callbacks.complete_guide) |cb| {
        try cb(ctx, gid);
        return;
    }
    if (ctx.guide_comp) |gc| {
        try gc.info.addFinished(ctx.alloc.gpa, gid);
    }
}

fn applyFinishDungeon(ctx: QuestContext, params: std.json.ObjectMap) !void {
    const is_success = quest_types.jsonBool(params.get("IsSuccess")) orelse true;
    const inst_id: i32 = if (ctx.scene) |s| s.instance_id else 0;
    if (is_success and inst_id > 0) {
        try ctx.quest_comp.info.markDungeonCompleted(ctx.alloc.gpa, inst_id);
        try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);
        _ = try recordProgressAdd(ctx, "inst:finish_cnt", 1);
    }
    if (inst_id > 0) {
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.InstResultNotify{
            .Id = inst_id,
            .Succ = is_success,
            .IsRecReward = is_success,
            .Reason = 1,
        }) catch {};
    }
}

fn applySettlementDungeon(ctx: QuestContext) !void {
    const inst_id: i32 = if (ctx.scene) |s| s.instance_id else 0;
    if (inst_id > 0) {
        try ctx.quest_comp.info.markDungeonCompleted(ctx.alloc.gpa, inst_id);
        try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);
    }
    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.InstSettleNotify{
        .IsSuccess = true,
        .Magnification = 1,
    }) catch {};
}

fn applyTimerAction(ctx: QuestContext, name: []const u8, params: std.json.ObjectMap) !void {
    const is_pause = if (std.mem.eql(u8, name, "ToggleTimerPauseState"))
        quest_types.jsonBool(params.get("IsPause")) orelse true
    else
        false;
    const timer_ty = quest_types.jsonStr(params.get("TimerType")) orelse "Default";
    const key = try std.fmt.allocPrint(ctx.alloc.arena, "inst:timer:{s}:paused", .{timer_ty});
    try recordProgressInt(ctx, key, if (is_pause) 1 else 0, false);
    const status: i32 = if (is_pause) 2 else 1;
    var timers: std.ArrayList(pb.MapEntry(i32, pb.InstanceTimer)) = .empty;
    try timers.append(ctx.alloc.arena, .{ .key = 1, .value = .{ .Status = status, .RemainTick = 60 } });
    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.InstanceTimerNotify{
        .InstanceTimers = timers,
    }) catch {};
}

fn recordProgressAdd(ctx: QuestContext, key: []const u8, delta: i64) !i64 {
    const pc = ctx.progress_comp orelse return 0;
    const next = try pc.info.addKeyInt(ctx.alloc.gpa, key, delta);
    try pc.save(ctx.fs, ctx.alloc.arena);
    return next;
}

fn recordProgressInt(ctx: QuestContext, key: []const u8, val: i64, also_claim: bool) !void {
    const pc = ctx.progress_comp orelse return;
    try pc.info.setKeyInt(ctx.alloc.gpa, key, val);
    if (also_claim) _ = try pc.info.markClaim(ctx.alloc.gpa, key);
    try pc.save(ctx.fs, ctx.alloc.arena);
}

fn recordProgressClaim(ctx: QuestContext, key: []const u8) !void {
    const pc = ctx.progress_comp orelse return;
    if (try pc.info.markClaim(ctx.alloc.gpa, key)) {
        try pc.save(ctx.fs, ctx.alloc.arena);
    }
}

fn applyRandomVar(ctx: QuestContext, callbacks: ActionCallbacks, owner_id: i32, params: std.json.ObjectMap) !void {
    const rv = params.get("Result") orelse params.get("Var") orelse return;
    if (rv != .object) return;
    const base_name = quest_types.resolveVarName(rv.object) orelse return;
    const src = quest_types.jsonStr(rv.object.get("Source")) orelse "Self";
    const is_global = std.mem.eql(u8, src, "Global");
    const store_key: []const u8 = if (!is_global)
        (quest_types.scopedVarKeyWithOwnerAlloc(ctx.alloc.arena, ctx.scene, owner_id, rv.object, base_name) orelse base_name)
    else
        base_name;
    const max_val: i32 = if (params.get("RightVar")) |rw| switch (rw) {
        .object => |obj| quest_types.jsonInt(obj.get("Value")) orelse 1,
        else => 1,
    } else 1;
    const now_ns: u64 = @intCast(@max(0, (std.Io.Clock.awake).now(ctx.io).nanoseconds));
    const chosen: i32 = @intCast(now_ns % @as(u64, @intCast(@max(max_val, 1))));
    try ctx.quest_comp.info.setVarInt(ctx.alloc.gpa, store_key, chosen);
    if (is_global) persistGlobalVarInt(ctx, base_name, chosen);
    try emitVarChanged(ctx, callbacks, owner_id, base_name, .{
        .VarType = 1,
        .Value = .{ .Int = chosen },
    }, is_global);
}

fn applyResetEntityAction(ctx: QuestContext, params: std.json.ObjectMap) !void {
    const scene = ctx.scene orelse return;
    const cfg_obj = if (params.get("ResetEntityConfig")) |rc| switch (rc) {
        .object => |o| o,
        else => params,
    } else params;
    const ids = try quest_types.parseIntList(ctx.alloc.arena, cfg_obj.get("EntityIds"));
    for (ids) |cfg_id| {
        if (scene.findEntityNetIdByConfigId(cfg_id)) |net_id| {
            scene.removeCombineRelationsForEntity(ctx.alloc.gpa, net_id) catch {};
            if (ctx.assets.findLevelEntity(scene.instance_id, cfg_id)) |ecfg| {
                if (Assets.AssetIndexes.entityWorldPos(ecfg)) |pos| {
                    _ = scene.setEntityPosition(net_id, pos, null);
                }
            }
        }
    }
}

fn applyChangeOtherState(
    ctx: QuestContext,
    callbacks: ActionCallbacks,
    quest_id: i32,
    node_id: i32,
    params: std.json.ObjectMap,
) !void {
    if (callbacks.start_flow_state) |cb| {
        if (params.get("State")) |sv| {
            if (sv == .object) {
                const fname = quest_types.jsonStr(sv.object.get("FlowListName")) orelse "";
                const fid = quest_types.jsonInt(sv.object.get("FlowId")) orelse 0;
                const sid = quest_types.jsonInt(sv.object.get("StateId")) orelse 1;
                if (fname.len > 0 and fid > 0) {
                    try cb(ctx, quest_id, node_id, fname, fid, sid, true);
                }
            }
        }
    }
}

fn wakeEntityFsm(scene: *Scene, gpa: std.mem.Allocator, cfg_id: i32, reason: Scene.Entity.FsmComponent.WakeMask) void {
    const net_id = scene.findEntityNetIdByConfigId(cfg_id) orelse return;
    scene.markFsmDirty(gpa, net_id, reason) catch {};
}

fn applyEntityGameplayTagChange(
    ctx: QuestContext,
    scene: *Scene,
    net_id: i64,
    tag_ids: []const i32,
    is_add: bool,
) !void {
    const idx = scene.net_id_map.get(net_id) orelse return;
    if (tag_ids.len == 0) return;

    var notify: pb.EntityCommonTagNotify = .{ .Id = net_id };
    const tag_slot = &scene.entities.items(.tag)[idx];
    for (tag_ids) |tid| {
        if (tid == 0) continue;
        if (is_add) {
            if (tag_slot.*) |*tags| {
                try tags.addCommonTag(ctx.alloc.gpa, tid);
                try tags.setGameplayTagCount(ctx.alloc.gpa, tid, 1);
            } else {
                var new_tags = Scene.Entity.TagComponent{};
                try new_tags.addCommonTag(ctx.alloc.gpa, tid);
                try new_tags.setGameplayTagCount(ctx.alloc.gpa, tid, 1);
                tag_slot.* = new_tags;
            }
        } else if (tag_slot.*) |*tags| {
            _ = tags.removeCommonTag(ctx.alloc.gpa, tid);
            try tags.removeGameplayTag(ctx.alloc.gpa, tid);
        }
        try notify.Tags.append(ctx.alloc.arena, .{ .TagId = tid, .RemoveTagIds = is_add });
    }
    if (notify.Tags.items.len > 0 and tags_handler.hasClientLevelTagComponent(scene, ctx.assets, net_id)) {
        ctx.conn.push(ctx.alloc.gpa, ctx.io, notify) catch {};
    }
    scene.markFsmDirty(ctx.alloc.gpa, net_id, Scene.Entity.FsmComponent.WakeReason.tag) catch {};
}

fn applySetBattleStateAction(ctx: QuestContext, params: std.json.ObjectMap) !void {
    const scene = ctx.scene orelse return;
    if (params.get("StateOption")) |so| {
        if (so == .object) {
            const st_type = quest_types.jsonStr(so.object.get("Type")) orelse "";
            if (std.mem.eql(u8, st_type, "SetBattleTag")) {
                if (so.object.get("SetTags")) |arr| {
                    if (arr == .array) {
                        for (arr.array.items) |item| {
                            if (item != .object) continue;
                            const cfg_id = std.math.cast(i32, quest_types.jsonInt(item.object.get("EntityId")) orelse 0) orelse continue;
                            if (cfg_id <= 0) continue;
                            const set_type = quest_types.jsonStr(item.object.get("SetType")) orelse "Add";
                            const is_add = !std.mem.eql(u8, set_type, "Remove");
                            const tag_name = quest_types.jsonStr(item.object.get("GameplayTag")) orelse continue;
                            const tid = gameplay_tags.idFromName(&ctx.assets.tables.gameplay_tag_remap, tag_name) catch continue;
                            if (is_add) ensureQuestEntitySpawned(ctx, cfg_id);
                            if (scene.findEntityNetIdByConfigId(cfg_id)) |net_id| {
                                try applyEntityGameplayTagChange(ctx, scene, net_id, &.{tid}, is_add);
                            }
                        }
                        return;
                    }
                }
            }
        }
    }
    if (quest_types.jsonBool(params.get("InBattle")) orelse quest_types.jsonBool(params.get("IsInBattle"))) |in_battle| {
        scene.forceBattleState(in_battle);
    } else {
        scene.forceBattleState(true);
    }
}

fn applyExecBattleAction(ctx: QuestContext, params: std.json.ObjectMap) !void {
    const scene = ctx.scene orelse return;
    const bo = params.get("BattleOption") orelse {
        scene.forceBattleState(true);
        return;
    };
    if (bo != .object) return;
    const btype = quest_types.jsonStr(bo.object.get("Type")) orelse return;
    if (std.mem.eql(u8, btype, "SetBattleTags")) {
        if (bo.object.get("Configs")) |cfgs| {
            if (cfgs == .array) {
                for (cfgs.array.items) |entry| {
                    if (entry != .object) continue;
                    const cfg_id = std.math.cast(i32, quest_types.jsonInt(entry.object.get("EntityId")) orelse 0) orelse continue;
                    const tid = std.math.cast(i32, quest_types.jsonInt(entry.object.get("TagConfigId")) orelse 0) orelse continue;
                    if (cfg_id <= 0 or tid == 0) continue;
                    ensureQuestEntitySpawned(ctx, cfg_id);
                    if (scene.findEntityNetIdByConfigId(cfg_id)) |net_id| {
                        try applyEntityGameplayTagChange(ctx, scene, net_id, &.{tid}, true);
                    }
                }
            }
        }
    } else if (std.mem.eql(u8, btype, "SetMonsterMoveTarget")) {
        const target_eid = std.math.cast(i32, quest_types.jsonInt(bo.object.get("TargetEntityId")) orelse 0) orelse 0;
        var target_pos: ?[3]f32 = null;
        if (target_eid > 0) {
            if (scene.findEntityNetIdByConfigId(target_eid)) |tnid| {
                if (scene.net_id_map.get(tnid)) |tidx| {
                    if (scene.entities.items(.position)[tidx]) |tp| target_pos = tp.location;
                }
            }
            if (target_pos == null) {
                if (ctx.assets.findLevelEntity(scene.instance_id, target_eid)) |lcfg| {
                    target_pos = Assets.AssetIndexes.entityWorldPos(lcfg);
                }
            }
        }
        const mids = try quest_types.parseIntList(ctx.alloc.arena, bo.object.get("MonsterEntityIds"));
        for (mids) |mid| {
            if (mid <= 0) continue;
            ensureQuestEntitySpawned(ctx, mid);
            if (target_pos) |pos| {
                if (scene.findEntityNetIdByConfigId(mid)) |mnid| {
                    try persistQuestEntityPosition(ctx, mid, mnid, pos);
                    try pushEntityMoved(ctx, mnid, pos, null);
                }
            }
            wakeEntityFsm(scene, ctx.alloc.gpa, mid, Scene.Entity.FsmComponent.WakeReason.event);
        }
    } else if (std.mem.eql(u8, btype, "SendBattleEvents")) {
        if (bo.object.get("Configs")) |cfgs| {
            if (cfgs == .array) {
                for (cfgs.array.items) |entry| {
                    if (entry != .object) continue;
                    const cfg_id = std.math.cast(i32, quest_types.jsonInt(entry.object.get("EntityId")) orelse 0) orelse continue;
                    if (cfg_id > 0) wakeEntityFsm(scene, ctx.alloc.gpa, cfg_id, Scene.Entity.FsmComponent.WakeReason.event);
                }
            }
        }
    }
}

fn applyExecAlertSystemAction(ctx: QuestContext, params: std.json.ObjectMap) !void {
    const opt = params.get("Option") orelse return;
    if (opt != .object) return;
    const atype = quest_types.jsonStr(opt.object.get("Type")) orelse return;
    const area_id = std.math.cast(i32, quest_types.jsonInt(opt.object.get("AreaId")) orelse 0) orelse return;
    if (area_id <= 0) return;

    const k_enable = try std.fmt.allocPrint(ctx.alloc.arena, "alert_enable:{d}", .{area_id});
    const k_ui = try std.fmt.allocPrint(ctx.alloc.arena, "alert_ui:{d}", .{area_id});
    const k_vis = try std.fmt.allocPrint(ctx.alloc.arena, "alert_vis:{d}", .{area_id});
    const k_val = try std.fmt.allocPrint(ctx.alloc.arena, "alert_val:{d}", .{area_id});

    var remove_ids: std.ArrayList(i32) = .empty;
    var areas: std.ArrayList(pb.AlertArea) = .empty;

    if (std.mem.eql(u8, atype, "EnableAlertArea")) {
        const enable = quest_types.jsonBool(opt.object.get("IsEnable")) orelse true;
        try recordProgressInt(ctx, k_enable, @intFromBool(enable), false);
        if (!enable) {
            try remove_ids.append(ctx.alloc.arena, area_id);
        } else {
            try areas.append(ctx.alloc.arena, .{
                .AreaId = area_id,
                .AlertValue = @floatFromInt(if (ctx.progress_comp) |pc| pc.info.getKeyInt(k_val) else 0),
                .EnableUi = if (ctx.progress_comp) |pc| pc.info.getKeyInt(k_ui) != 0 else false,
                .VisibleUi = if (ctx.progress_comp) |pc| pc.info.getKeyInt(k_vis) != 0 else false,
            });
        }
    } else if (std.mem.eql(u8, atype, "EnableAlertUi")) {
        const enable = quest_types.jsonBool(opt.object.get("IsEnable")) orelse true;
        try recordProgressInt(ctx, k_ui, @intFromBool(enable), false);
        try areas.append(ctx.alloc.arena, .{
            .AreaId = area_id,
            .AlertValue = @floatFromInt(if (ctx.progress_comp) |pc| pc.info.getKeyInt(k_val) else 0),
            .EnableUi = enable,
            .VisibleUi = if (ctx.progress_comp) |pc| pc.info.getKeyInt(k_vis) != 0 else enable,
        });
    } else if (std.mem.eql(u8, atype, "SetAlertUiVisible")) {
        const vis = quest_types.jsonBool(opt.object.get("IsVisible")) orelse true;
        try recordProgressInt(ctx, k_vis, @intFromBool(vis), false);
        try areas.append(ctx.alloc.arena, .{
            .AreaId = area_id,
            .AlertValue = @floatFromInt(if (ctx.progress_comp) |pc| pc.info.getKeyInt(k_val) else 0),
            .EnableUi = if (ctx.progress_comp) |pc| pc.info.getKeyInt(k_ui) != 0 else true,
            .VisibleUi = vis,
        });
    } else if (std.mem.eql(u8, atype, "ModifyAlertValue")) {
        const val = quest_types.jsonInt(opt.object.get("AlertValue")) orelse 0;
        try recordProgressInt(ctx, k_val, val, false);
        try areas.append(ctx.alloc.arena, .{
            .AreaId = area_id,
            .AlertValue = @floatFromInt(val),
            .EnableUi = if (ctx.progress_comp) |pc| pc.info.getKeyInt(k_ui) != 0 else true,
            .VisibleUi = if (ctx.progress_comp) |pc| pc.info.getKeyInt(k_vis) != 0 else true,
        });
    }
    if (areas.items.len > 0 or remove_ids.items.len > 0) {
        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.AlertAreaNotify{
            .AlertAreas = areas,
            .RemoveAreaIds = remove_ids,
        }) catch {};
    }
}

fn applyElementalBoxAction(ctx: QuestContext, name: []const u8, params: std.json.ObjectMap) !void {
    const target = params.get("Target") orelse return;
    if (target != .object) return;
    const ttype = quest_types.jsonStr(target.object.get("Type")) orelse "";

    if (std.mem.eql(u8, name, "SetElementalBoxToTarget")) {
        const box_id = quest_types.jsonInt(params.get("ElementalBoxId")) orelse return;
        const k = if (std.mem.eql(u8, ttype, "Target")) blk: {
            const eid = quest_types.jsonInt(target.object.get("EntityId")) orelse return;
            break :blk try std.fmt.allocPrint(ctx.alloc.arena, "elemental_box:ent:{d}", .{eid});
        } else try std.fmt.allocPrint(ctx.alloc.arena, "elemental_box:player", .{});
        try recordProgressInt(ctx, k, box_id, false);
    } else if (std.mem.eql(u8, name, "ChangeElementalBoxState")) {
        const state_str = quest_types.jsonStr(params.get("State")) orelse return;
        if (std.mem.eql(u8, ttype, "TargetBoxId")) {
            const box_id = std.math.cast(i32, quest_types.jsonInt(target.object.get("TargetBoxId")) orelse 0) orelse return;
            if (box_id <= 0) return;
            try ctx.quest_comp.info.setEntityState(ctx.alloc.gpa, box_id, state_str);
            try ctx.quest_comp.save(ctx.fs, ctx.alloc.arena);
            if (ctx.scene) |scene| {
                if (scene.findEntityNetIdByConfigId(box_id)) |nid| {
                    if (try scene.updateEntityStateTag(ctx.alloc.gpa, &ctx.assets.tables.gameplay_tag_remap, nid, state_str)) |upd| {
                        try emitEntityStateTags(ctx, nid, upd);
                    }
                }
            }
        } else {
            const box_id = if (ctx.progress_comp) |pc| pc.info.getKeyInt("elemental_box:player") else 0;
            const k = try std.fmt.allocPrint(ctx.alloc.arena, "elemental_box:state:{d}", .{box_id});
            try recordProgressInt(ctx, k, 1, false);
            if (ctx.quest_comp.info.setVarStr(ctx.alloc.gpa, "elemental_box_state", state_str)) |_| {} else |_| {}
        }
    }
}

fn applyAiControlAction(ctx: QuestContext, name: []const u8, params: std.json.ObjectMap) !void {
    const ids = try quest_types.parseIntList(ctx.alloc.arena, params.get("EntityIds"));
    for (ids) |cfg_id| {
        if (cfg_id <= 0) continue;
        if (std.mem.eql(u8, name, "EnableHostility")) {
            const en = quest_types.jsonBool(params.get("IsEnable")) orelse true;
            const k = try std.fmt.allocPrint(ctx.alloc.arena, "hostility:{d}", .{cfg_id});
            try recordProgressInt(ctx, k, @intFromBool(en), false);
            if (ctx.scene) |scene| wakeEntityFsm(scene, ctx.alloc.gpa, cfg_id, Scene.Entity.FsmComponent.WakeReason.hate);
        } else if (std.mem.eql(u8, name, "EnableAI")) {
            const en = quest_types.jsonBool(params.get("IsEnable")) orelse true;
            const k = try std.fmt.allocPrint(ctx.alloc.arena, "ai_enabled:{d}", .{cfg_id});
            try recordProgressInt(ctx, k, @intFromBool(en), false);
            if (ctx.scene) |scene| {
                if (scene.findEntityNetIdByConfigId(cfg_id)) |nid| {
                    if (!en) {
                        scene.setBattleEntityActive(ctx.alloc.gpa, nid, false) catch {};
                    }
                    scene.markFsmDirty(ctx.alloc.gpa, nid, Scene.Entity.FsmComponent.WakeReason.initial) catch {};
                }
            }
        } else if (std.mem.eql(u8, name, "OverrideAiBehaviorTree")) {
            const bt_id = quest_types.jsonInt(params.get("BehaviorTree")) orelse 0;
            const bkey = quest_types.jsonStr(params.get("Key")) orelse "default";
            const k1 = try std.fmt.allocPrint(ctx.alloc.arena, "ai_bt:{d}:{s}", .{ cfg_id, bkey });
            const k2 = try std.fmt.allocPrint(ctx.alloc.arena, "ai_bt_cur:{d}", .{cfg_id});
            try recordProgressInt(ctx, k1, bt_id, false);
            try recordProgressInt(ctx, k2, bt_id, false);
            if (ctx.scene) |scene| wakeEntityFsm(scene, ctx.alloc.gpa, cfg_id, Scene.Entity.FsmComponent.WakeReason.blackboard);
        }
    }
}

fn applyOccupationAction(ctx: QuestContext, params: std.json.ObjectMap) !void {
    const otype = quest_types.jsonStr(params.get("OccupationType")) orelse "Request";
    const is_req = !std.mem.eql(u8, otype, "Release");
    if (params.get("Occupations")) |arr| {
        if (arr == .array) {
            for (arr.array.items) |item| {
                const occ_name = quest_types.jsonStr(item) orelse continue;
                const k = try std.fmt.allocPrint(ctx.alloc.arena, "occ:{s}", .{occ_name});
                try recordProgressInt(ctx, k, @intFromBool(is_req), false);
            }
        }
    }
    try recordProgressInt(ctx, "world:occupation", @intFromBool(is_req), false);
}

fn applySetNpcGroupPerform(ctx: QuestContext, params: std.json.ObjectMap) !void {
    const pkey = quest_types.jsonStr(params.get("Key")) orelse "";
    const pt = params.get("PerformType") orelse return;
    if (pt != .object) return;

    if (pkey.len > 0) {
        const k = try std.fmt.allocPrint(ctx.alloc.arena, "npc:group_perform:{s}", .{pkey});
        try recordProgressInt(ctx, k, 1, false);
    }

    if (pt.object.get("Type")) |tv| {
        switch (tv) {
            .string => |pt_type| {
                if (std.mem.eql(u8, pt_type, "SupportedWalking")) {
                    try recordProgressInt(ctx, "npc:supported_walking", 1, false);
                } else if (std.mem.eql(u8, pt_type, "CancelSupportedWalking")) {
                    try recordProgressInt(ctx, "npc:supported_walking", 0, false);
                }
            },
            .integer => |pt_id| {
                if (pt_id == 0) {
                    if (ctx.scene) |scene| {
                        const role_id = scene.currentRoleEntityId();
                        if (role_id > 0) {
                            const k = try std.fmt.allocPrint(ctx.alloc.arena, "hold_hand:{d}", .{role_id});
                            try recordProgressInt(ctx, k, 1, false);
                        }
                    }
                } else if (pt_id == 5 or pt_id == 6) {
                    try recordProgressInt(ctx, "npc:supported_walking", 1, false);
                }
            },
            else => {},
        }
    }
}

fn applyModifySceneTagAction(ctx: QuestContext, params: std.json.ObjectMap) !void {
    const scene = ctx.scene orelse return;
    const ids = try quest_types.parseIntList(ctx.alloc.arena, params.get("EntityIds"));
    const tags = try quest_types.parseIntList(ctx.alloc.arena, params.get("PerformanceTag"));
    if (tags.len == 0) return;
    const is_add = quest_types.jsonBool(params.get("IsAddTag")) orelse true;
    for (ids) |cfg_id| {
        if (is_add) ensureQuestEntitySpawned(ctx, cfg_id);
        if (scene.findEntityNetIdByConfigId(cfg_id)) |net_id| {
            try applyEntityGameplayTagChange(ctx, scene, net_id, tags, is_add);
            if (is_add) {
                if (try scene.updateEntityStateTagId(ctx.alloc.gpa, net_id, tags[0])) |upd| {
                    try emitEntityStateTags(ctx, net_id, upd);
                }
            } else if (scene.net_id_map.get(net_id)) |idx| {
                if (scene.entities.items(.state_tag)[idx]) |st| {
                    for (tags) |t| {
                        if (st.state_tag_id == t) {
                            _ = try scene.updateEntityStateTagId(ctx.alloc.gpa, net_id, 0);
                            break;
                        }
                    }
                }
            }
        }
    }
}

fn resolveQuestSplineEndPos(assets: *const Assets, instance_id: i32, spline_cfg_id: i64) ?[3]f32 {
    const lcfg = assets.findLevelEntity(instance_id, spline_cfg_id) orelse return null;
    const base = Assets.AssetIndexes.entityWorldPos(lcfg) orelse .{ 0, 0, 0 };
    const rot = Assets.AssetIndexes.entityWorldRot(lcfg);
    var components = lcfg.ComponentsData;
    if (assets.findTemplateConfig(lcfg.BlueprintType)) |tpl| {
        tpl.ComponentsData.mergeInto(&components);
    }
    const spline = components.SplineComponent orelse return null;
    const opt = spline.Option orelse return null;
    if (opt != .object) return null;
    const pts_val = opt.object.get("Points") orelse return null;
    if (pts_val != .array or pts_val.array.items.len == 0) return null;
    const last = pts_val.array.items[pts_val.array.items.len - 1];
    if (last != .object) return null;
    const pos_val = last.object.get("Position") orelse return null;
    const off = GravityHelper.rotateLocalOffsetUe(parseVectorPos(pos_val) orelse return null, rot);
    return .{ base[0] + off[0], base[1] + off[1], base[2] + off[2] };
}

fn applyServerMoveAction(ctx: QuestContext, name: []const u8, params: std.json.ObjectMap) !void {
    const scene = ctx.scene orelse return;
    if (std.mem.eql(u8, name, "MoveSceneItem")) {
        const cfg_id = quest_types.jsonInt(params.get("EntityId")) orelse return;
        const mc = params.get("MoveConfig") orelse return;
        if (mc != .object) return;
        const pos = parseVectorPos(mc.object.get("Point")) orelse return;
        const move_time = quest_types.jsonFloat(mc.object.get("Time")) orelse 0;
        ensureQuestEntitySpawned(ctx, cfg_id);
        if (scene.findEntityNetIdByConfigId(cfg_id)) |net_id| {
            try persistQuestEntityPosition(ctx, cfg_id, net_id, pos);
            if (move_time > 0) {
                ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.SceneItemMoveTargetNotify{
                    .MoveInfo = .{
                        .EntityId = net_id,
                        .Location = .{ .X = pos[0], .Y = pos[1], .Z = pos[2] },
                        .MoveTime = move_time,
                    },
                }) catch {};
            } else {
                try pushEntityMoved(ctx, net_id, pos, null);
            }
        }
    } else if (std.mem.eql(u8, name, "NpcMoveToPoint")) {
        const cfg_id = parseQuestEntityId(params.get("Target") orelse params.get("EntityId")) orelse return;
        var pos_opt = parseVectorPos(params.get("Pos")) orelse parseVectorPos(params.get("TargetPos"));
        if (pos_opt == null) {
            if (params.get("Points")) |pts| {
                if (pts == .array and pts.array.items.len > 0) {
                    const last = pts.array.items[pts.array.items.len - 1];
                    pos_opt = if (last == .object) parseVectorPos(last.object.get("Position") orelse last) else null;
                }
            }
        }
        const pos = pos_opt orelse return;
        ensureQuestEntitySpawned(ctx, cfg_id);
        if (scene.findEntityNetIdByConfigId(cfg_id)) |net_id| {
            try persistQuestEntityPosition(ctx, cfg_id, net_id, pos);
            try pushEntityMoved(ctx, net_id, pos, null);
        }
    } else if (std.mem.eql(u8, name, "CharacterMoveToPoint")) {
        const cfg_id = parseQuestEntityId(params.get("Target")) orelse return;
        const pos = parseVectorPos(params.get("Pos")) orelse return;
        ensureQuestEntitySpawned(ctx, cfg_id);
        if (scene.findEntityNetIdByConfigId(cfg_id)) |net_id| {
            try persistQuestEntityPosition(ctx, cfg_id, net_id, pos);
            try pushEntityMoved(ctx, net_id, pos, null);
        }
    } else if (std.mem.eql(u8, name, "MoveWithSpline") or std.mem.eql(u8, name, "NewMoveWithSpline")) {
        var target = parseQuestEntityId(params.get("MoveTarget")) orelse 0;
        if (target <= 0 and scene.active_flow_owner_entity_id != 0) {
            if (scene.findConfigIdByNetId(scene.active_flow_owner_entity_id)) |owner_cfg| {
                target = std.math.cast(i32, owner_cfg) orelse 0;
            }
        }
        const spline_id = quest_types.jsonInt(params.get("SplineEntityId")) orelse return;
        if (target <= 0 or spline_id <= 0) return;
        ensureQuestEntitySpawned(ctx, target);
        if (scene.findEntityNetIdByConfigId(target)) |net_id| {
            if (scene.net_id_map.get(net_id)) |index| {
                const spline_config_id = std.math.cast(i32, spline_id) orelse return;
                const move_spline_config = moveSplineConfigFromAction(params);
                scene.entities.items(.world_item)[index].move_spline = .{
                    .move_status = .Moving,
                    .spline_entity_id = spline_config_id,
                    .move_spline_config = move_spline_config,
                };
                const key = try std.fmt.allocPrint(ctx.alloc.arena, "quest_spline:{d}", .{target});
                const active_key = try std.fmt.allocPrint(ctx.alloc.arena, "quest_spline_active:{d}", .{target});
                const item_active_key = try std.fmt.allocPrint(ctx.alloc.arena, "scene_item_spline_active:{d}", .{target});
                const vehicle_active_key = try std.fmt.allocPrint(ctx.alloc.arena, "vehicle:spline_active:{d}", .{target});
                try recordProgressInt(ctx, key, spline_id, false);
                try recordProgressInt(ctx, active_key, 1, false);
                if (ctx.progress_comp) |progress| {
                    try progress.info.setKeyInt(ctx.alloc.gpa, item_active_key, 0);
                    try progress.info.setKeyInt(ctx.alloc.gpa, vehicle_active_key, 0);
                    try spline_state.persistMoveSplineConfig(
                        ctx.alloc.gpa,
                        ctx.alloc.arena,
                        &progress.info,
                        target,
                        move_spline_config,
                    );
                    try progress.save(ctx.fs, ctx.alloc.arena);
                }
                pushSplineStatus(ctx, net_id, .Moving, spline_config_id, null, move_spline_config);
            }
        }
    } else if (std.mem.eql(u8, name, "PlayerMoveToPoint")) {
        if (parseVectorPos(params.get("Pos")) orelse parseVectorPos(params.get("TargetPos"))) |pos| {
            try updatePlayerSceneState(ctx, pos, null);
            ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.TeleportNotify{
                .MapId = scene.instance_id,
                .Pos = .{ .X = pos[0], .Y = pos[1], .Z = pos[2] },
                .Reason = .Action,
            }) catch {};
        }
    } else if (std.mem.eql(u8, name, "ChangeLiftTarget")) {
        if (quest_types.jsonInt(params.get("EntityId"))) |cfg_id| {
            const loc = quest_types.jsonInt(params.get("Location")) orelse 0;
            const k = try std.fmt.allocPrint(ctx.alloc.arena, "lift_target:{d}", .{cfg_id});
            try recordProgressInt(ctx, k, loc, false);
            const ek = try std.fmt.allocPrint(ctx.alloc.arena, "elevator:{d}", .{cfg_id});
            try recordProgressInt(ctx, ek, loc, false);
            if (scene.findEntityNetIdByConfigId(cfg_id)) |net_id| {
                if (scene.net_id_map.get(net_id)) |index| {
                    scene.entities.items(.world_item)[index].lift = .{ .location = std.math.cast(i32, loc) orelse 0 };
                    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.ElevatorMoveNotify{
                        .EntityId = net_id,
                        .Location = std.math.cast(i32, loc) orelse 0,
                    }) catch {};
                }
            }
        }
    }
}

fn parseQuestEntityId(value_opt: ?std.json.Value) ?i32 {
    const value = value_opt orelse return null;
    if (quest_types.jsonInt(value)) |id| return std.math.cast(i32, id);
    if (value == .object) {
        if (quest_types.jsonInt(value.object.get("EntityId"))) |id| return std.math.cast(i32, id);
    }
    return null;
}

fn persistQuestEntityPosition(ctx: QuestContext, config_id: i32, net_id: i64, pos: [3]f32) !void {
    const scene = ctx.scene orelse return;
    if (!std.math.isFinite(pos[0]) or !std.math.isFinite(pos[1]) or !std.math.isFinite(pos[2])) return;
    if (!scene.setEntityPosition(net_id, pos, null)) return;
    const index = scene.net_id_map.get(net_id) orelse return;
    const entity_config = scene.entities.items(.config)[index];
    try scene.saveComponents(ctx.fs, ctx.alloc.gpa, .{ .index = index, .net_id = net_id }, &.{Scene.Entity.PositionComponent});

    if (entity_config.entity_type != .player and entity_config.entity_type != .player_entity and
        scene.entities.items(.player_id)[index] == null)
    {
        const pc = ctx.progress_comp orelse return;
        const position = scene.entities.items(.position)[index] orelse return;
        const fields = [_][]const u8{ "x", "y", "z", "roll", "pitch", "yaw" };
        const values = [_]f32{
            position.location[0], position.location[1], position.location[2],
            position.rotation[0], position.rotation[1], position.rotation[2],
        };
        for (fields, values) |field, value| {
            const key = try std.fmt.allocPrint(ctx.alloc.arena, "world_pos:{d}:{s}", .{ config_id, field });
            try pc.info.setKeyInt(ctx.alloc.gpa, key, @intCast(@as(u32, @bitCast(value))));
        }
        try pc.save(ctx.fs, ctx.alloc.arena);
    }
}

fn applyVehicleAction(ctx: QuestContext, name: []const u8, params: std.json.ObjectMap) !void {
    if (std.mem.eql(u8, name, "VehicleEnter") or std.mem.eql(u8, name, "TeleportToAndEnterVehicle")) {
        const seat: i32 = @intCast(quest_types.jsonInt(params.get("Seat")) orelse 0);
        const is_teleport_enter = std.mem.eql(u8, name, "TeleportToAndEnterVehicle");
        const vt_vid: ?i32 = if (params.get("VehicleTarget")) |vt| blk: {
            if (vt != .object) break :blk null;
            break :blk quest_types.jsonInt(vt.object.get("VehicleId"));
        } else null;

        if (params.get("EnteringTarget")) |et| {
            if (et == .object) {
                const target_ty = quest_types.jsonStr(et.object.get("Type")) orelse "Player";
                if (quest_types.jsonInt(et.object.get("TargetVehicle")) orelse vt_vid) |vid| {
                    ensureQuestEntitySpawned(ctx, vid);
                    if (std.mem.eql(u8, target_ty, "Npc")) {
                        if (quest_types.jsonInt(et.object.get("TargetNpc"))) |npc_cfg| {
                            ensureQuestEntitySpawned(ctx, npc_cfg);
                            try recordVehicleNpcLink(ctx, npc_cfg, vid, seat);
                            if (ctx.scene) |scene| {
                                const npc_cid = std.math.cast(i32, npc_cfg) orelse 0;
                                const veh_cid = std.math.cast(i32, vid) orelse 0;
                                const npc_nid = if (npc_cid > 0) scene.findEntityNetIdByConfigId(npc_cid) else null;
                                const veh_nid = if (veh_cid > 0) scene.findEntityNetIdByConfigId(veh_cid) else null;
                                if (npc_nid != null and veh_nid != null) {
                                    if (scene.net_id_map.get(veh_nid.?)) |vehicle_index| {
                                        if (scene.entities.items(.vehicle)[vehicle_index]) |*vehicle| vehicle.occupied = true;
                                    }
                                    if (scene.net_id_map.get(npc_nid.?)) |npc_idx| {
                                        scene.entities.items(.npc_drive_vehicle)[npc_idx] = .{
                                            .vehicle_entity_id = veh_nid.?,
                                            .seat = seat,
                                        };
                                    }
                                    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.VehicleUpdateEntityNotify{
                                        .EntityId = npc_nid.?,
                                        .VehicleCreatureId = veh_nid.?,
                                        .Seat = seat,
                                        .IsEntering = true,
                                    }) catch {};
                                }
                            }
                        }
                        return;
                    }
                    try recordProgressInt(ctx, "vehicle:cur", vid, false);
                    try recordProgressInt(ctx, "vehicle:seat", seat, false);
                    if (ctx.scene) |scene| {
                        const veh_cid = std.math.cast(i32, vid) orelse 0;
                        if (veh_cid > 0) {
                            if (scene.findEntityNetIdByConfigId(veh_cid)) |veh_nid| {
                                if (scene.net_id_map.get(veh_nid)) |vidx| {
                                    if (scene.entities.items(.vehicle)[vidx]) |*veh| veh.occupied = true;
                                    if (is_teleport_enter) {
                                        if (scene.entities.items(.position)[vidx]) |vp| {
                                            try updatePlayerSceneState(ctx, vp.location, vp.rotation);
                                            ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.TeleportNotify{
                                                .MapId = scene.instance_id,
                                                .Pos = .{ .X = vp.location[0], .Y = vp.location[1], .Z = vp.location[2] },
                                                .Rot = .{ .X = vp.rotation[0], .Y = vp.rotation[1], .Z = vp.rotation[2] },
                                                .Reason = .Action,
                                            }) catch {};
                                        }
                                    }
                                }
                                ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.VehicleUpdateNotify{
                                    .EntityId = veh_nid,
                                    .PlayerId = ctx.basic_comp.player_id,
                                    .SeatId = seat,
                                }) catch {};
                            }
                        }
                    }
                    return;
                }
            }
        }
        if (quest_types.jsonInt(params.get("VehicleEntityId")) orelse quest_types.jsonInt(params.get("TargetVehicle")) orelse vt_vid) |vid| {
            ensureQuestEntitySpawned(ctx, vid);
            try recordProgressInt(ctx, "vehicle:cur", vid, false);
            try recordProgressInt(ctx, "vehicle:seat", seat, false);
            if (ctx.scene) |scene| {
                const veh_cid = std.math.cast(i32, vid) orelse 0;
                if (veh_cid > 0) {
                    if (scene.findEntityNetIdByConfigId(veh_cid)) |veh_nid| {
                        if (scene.net_id_map.get(veh_nid)) |vidx| {
                            if (scene.entities.items(.vehicle)[vidx]) |*veh| veh.occupied = true;
                            if (is_teleport_enter) {
                                if (scene.entities.items(.position)[vidx]) |vp| {
                                    try updatePlayerSceneState(ctx, vp.location, vp.rotation);
                                    ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.TeleportNotify{
                                        .MapId = scene.instance_id,
                                        .Pos = .{ .X = vp.location[0], .Y = vp.location[1], .Z = vp.location[2] },
                                        .Rot = .{ .X = vp.rotation[0], .Y = vp.rotation[1], .Z = vp.rotation[2] },
                                        .Reason = .Action,
                                    }) catch {};
                                }
                            }
                        }
                        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.VehicleUpdateNotify{
                            .EntityId = veh_nid,
                            .PlayerId = ctx.basic_comp.player_id,
                            .SeatId = seat,
                        }) catch {};
                    }
                }
            }
        } else if (params.get("ToVehicle")) |tv| {
            if (tv == .object) {
                const tv_type = quest_types.jsonStr(tv.object.get("Type")) orelse "";
                if (quest_types.jsonInt(tv.object.get("EntityId"))) |vid| {
                    ensureQuestEntitySpawned(ctx, vid);
                    try recordProgressInt(ctx, "vehicle:cur", vid, false);
                    try recordProgressInt(ctx, "vehicle:seat", seat, false);
                    if (ctx.scene) |scene| {
                        if (scene.findEntityNetIdByConfigId(@intCast(vid))) |veh_nid| {
                            if (scene.net_id_map.get(veh_nid)) |vidx| {
                                if (scene.entities.items(.vehicle)[vidx]) |*veh| veh.occupied = true;
                                if (is_teleport_enter) {
                                    if (scene.entities.items(.position)[vidx]) |vp| {
                                        try updatePlayerSceneState(ctx, vp.location, vp.rotation);
                                        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.TeleportNotify{
                                            .MapId = scene.instance_id,
                                            .Pos = .{ .X = vp.location[0], .Y = vp.location[1], .Z = vp.location[2] },
                                            .Rot = .{ .X = vp.rotation[0], .Y = vp.rotation[1], .Z = vp.rotation[2] },
                                            .Reason = .Action,
                                        }) catch {};
                                    }
                                }
                            }
                            ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.VehicleUpdateNotify{
                                .EntityId = veh_nid,
                                .PlayerId = ctx.basic_comp.player_id,
                                .SeatId = seat,
                            }) catch {};
                        }
                    }
                } else if (std.mem.eql(u8, tv_type, "FishingBoat")) {
                    try recordProgressInt(ctx, "fish:boat_active", 1, false);
                    if (ctx.scene) |scene| {
                        const slice = scene.entities.slice();
                        for (slice.items(.entity_id), slice.items(.config), slice.items(.vehicle), slice.items(.position)) |nid, cfg, *veh_opt, pos_opt| {
                            if (veh_opt.*) |*veh| {
                                veh.occupied = true;
                                try recordProgressInt(ctx, "vehicle:cur", cfg.config_id, false);
                                try recordProgressInt(ctx, "vehicle:seat", seat, false);
                                if (is_teleport_enter) {
                                    if (pos_opt) |vp| {
                                        try updatePlayerSceneState(ctx, vp.location, vp.rotation);
                                        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.TeleportNotify{
                                            .MapId = scene.instance_id,
                                            .Pos = .{ .X = vp.location[0], .Y = vp.location[1], .Z = vp.location[2] },
                                            .Rot = .{ .X = vp.rotation[0], .Y = vp.rotation[1], .Z = vp.rotation[2] },
                                            .Reason = .Action,
                                        }) catch {};
                                    }
                                }
                                ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.VehicleUpdateNotify{
                                    .EntityId = nid.net_id,
                                    .PlayerId = ctx.basic_comp.player_id,
                                    .SeatId = seat,
                                }) catch {};
                                break;
                            }
                        }
                    }
                }
            }
        }
    } else if (std.mem.eql(u8, name, "VehicleExitPlayer")) {
        const vehicle_id = if (ctx.progress_comp) |pc| pc.info.getKeyInt("vehicle:cur") else 0;
        try recordProgressInt(ctx, "vehicle:cur", 0, false);
        try recordProgressInt(ctx, "vehicle:seat", -1, false);
        if (ctx.scene) |scene| {
            const vehicle_cid = std.math.cast(i32, vehicle_id) orelse 0;
            if (vehicle_cid > 0 and !vehicleHasOtherOccupants(ctx, vehicle_cid, null)) {
                if (scene.findEntityNetIdByConfigId(vehicle_cid)) |vehicle_net_id| {
                    if (scene.net_id_map.get(vehicle_net_id)) |index| {
                        if (scene.entities.items(.vehicle)[index]) |*vehicle| vehicle.occupied = false;
                    }
                }
            }
            ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.VehicleUpdateNotify{
                .EntityId = 0,
                .PlayerId = ctx.basic_comp.player_id,
                .SeatId = -1,
                .ExitType = .ExitVehicleTypeNormal,
            }) catch {};
        }
    } else if (std.mem.eql(u8, name, "VehicleExitNpc")) {
        if (quest_types.jsonInt(params.get("NpcEntityId")) orelse quest_types.jsonInt(params.get("TargetNpc"))) |npc_cfg| {
            const npc_id = std.math.cast(i32, npc_cfg) orelse 0;
            const link_key = try std.fmt.allocPrint(ctx.alloc.arena, "vehicle:npc:{d}", .{npc_cfg});
            const previous_vehicle_id = if (ctx.progress_comp) |pc| std.math.cast(i32, pc.info.getKeyInt(link_key)) orelse 0 else 0;
            try recordVehicleNpcLink(ctx, npc_cfg, 0, -1);
            if (ctx.scene) |scene| {
                const npc_cid = npc_id;
                if (npc_cid > 0) {
                    if (scene.findEntityNetIdByConfigId(npc_cid)) |npc_nid| {
                        if (scene.net_id_map.get(npc_nid)) |npc_idx| {
                            scene.entities.items(.npc_drive_vehicle)[npc_idx] = null;
                        }
                        ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.VehicleUpdateEntityNotify{
                            .EntityId = npc_nid,
                            .VehicleCreatureId = 0,
                            .Seat = -1,
                            .IsEntering = false,
                            .ExitType = .ExitVehicleTypeNormal,
                        }) catch {};
                    }
                }
                if (previous_vehicle_id > 0 and !vehicleHasOtherOccupants(ctx, previous_vehicle_id, npc_id)) {
                    if (scene.findEntityNetIdByConfigId(previous_vehicle_id)) |vehicle_net_id| {
                        if (scene.net_id_map.get(vehicle_net_id)) |index| {
                            if (scene.entities.items(.vehicle)[index]) |*vehicle| vehicle.occupied = false;
                        }
                    }
                }
            }
        }
    } else if (std.mem.eql(u8, name, "VehicleTeleport")) {
        if (parseVectorPos(params.get("Pos"))) |pos| {
            if (ctx.scene) |scene| {
                try updatePlayerSceneState(ctx, pos, null);
                const slice = scene.entities.slice();
                for (slice.items(.entity_id), slice.items(.vehicle), slice.items(.config), 0..) |eid, veh_opt, config, index| {
                    if (veh_opt) |veh| {
                        if (veh.occupied) {
                            try persistQuestEntityPosition(ctx, config.config_id, eid.net_id, pos);
                            try scene.saveComponents(ctx.fs, ctx.alloc.gpa, .{ .index = index, .net_id = eid.net_id }, &.{Scene.Entity.PositionComponent});
                            try pushEntityMoved(ctx, eid.net_id, pos, null);
                        }
                    }
                }
                ctx.conn.push(ctx.alloc.gpa, ctx.io, pb.TeleportNotify{
                    .MapId = scene.instance_id,
                    .Pos = .{ .X = pos[0], .Y = pos[1], .Z = pos[2] },
                    .Reason = .Action,
                }) catch {};
            }
        }
    } else if (std.mem.eql(u8, name, "VehicleMoveWithPathLine")) {
        const control = params.get("ControlType") orelse return;
        if (control != .object) return;
        const control_type = quest_types.jsonStr(control.object.get("Type")) orelse return;
        const target = params.get("TargetVehicle") orelse return;
        if (target != .object) return;
        const target_type = quest_types.jsonStr(target.object.get("Type")) orelse return;
        const vehicle_config_id: i64 = if (std.mem.eql(u8, target_type, "Appointed")) blk: {
            break :blk quest_types.jsonInt(target.object.get("VehicleId")) orelse 0;
        } else if (std.mem.eql(u8, target_type, "Current")) blk: {
            const progress = ctx.progress_comp orelse return;
            break :blk progress.info.getKeyInt("vehicle:cur");
        } else 0;
        if (vehicle_config_id <= 0) return;
        const spline_id = quest_types.jsonInt(params.get("SplineEntityId")) orelse 0;
        const vehicle_key = try std.fmt.allocPrint(ctx.alloc.arena, "vehicle:spline:{d}", .{vehicle_config_id});
        const active_key = try std.fmt.allocPrint(ctx.alloc.arena, "vehicle:spline_active:{d}", .{vehicle_config_id});
        const item_active_key = try std.fmt.allocPrint(ctx.alloc.arena, "scene_item_spline_active:{d}", .{vehicle_config_id});
        const quest_active_key = try std.fmt.allocPrint(ctx.alloc.arena, "quest_spline_active:{d}", .{vehicle_config_id});
        const is_entering = std.mem.eql(u8, control_type, "EnterPathMoving");
        try recordProgressInt(ctx, vehicle_key, if (is_entering) spline_id else 0, false);
        try recordProgressInt(ctx, active_key, @intFromBool(is_entering), false);
        try recordProgressInt(ctx, item_active_key, 0, false);
        try recordProgressInt(ctx, quest_active_key, 0, false);
        const vehicle_cfg_id = std.math.cast(i32, vehicle_config_id) orelse return;
        ensureQuestEntitySpawned(ctx, vehicle_cfg_id);
        if (ctx.scene) |scene| {
            if (scene.findEntityNetIdByConfigId(vehicle_cfg_id)) |net_id| {
                const spline_config_id = std.math.cast(i32, spline_id) orelse return;
                if (scene.net_id_map.get(net_id)) |index| {
                    if (is_entering) {
                        scene.entities.items(.world_item)[index].move_spline = .{
                            .move_status = .Moving,
                            .spline_entity_id = spline_config_id,
                        };
                    } else {
                        scene.entities.items(.world_item)[index].move_spline = null;
                    }
                }
                pushSplineStatus(ctx, net_id, if (is_entering) .Moving else .Stop, spline_config_id, null, null);
            }
        }
    } else if (std.mem.eql(u8, name, "StopNewMoveWithSpline")) {
        const stop_target = params.get("StopTarget") orelse return;
        if (stop_target != .object) return;
        const stop_type = quest_types.jsonStr(stop_target.object.get("Type")) orelse return;

        var target_cfg_id: i32 = 0;
        if (std.mem.eql(u8, stop_type, "Vehicle")) {
            const target = stop_target.object.get("TargetVehicle") orelse return;
            if (target != .object) return;
            const target_type = quest_types.jsonStr(target.object.get("Type")) orelse return;
            const vehicle_config_id: i64 = if (std.mem.eql(u8, target_type, "Appointed")) blk: {
                break :blk quest_types.jsonInt(target.object.get("VehicleId")) orelse 0;
            } else if (std.mem.eql(u8, target_type, "Current")) blk: {
                const progress = ctx.progress_comp orelse return;
                break :blk progress.info.getKeyInt("vehicle:cur");
            } else 0;
            target_cfg_id = std.math.cast(i32, vehicle_config_id) orelse return;
        } else if (std.mem.eql(u8, stop_type, "SceneItem") or std.mem.eql(u8, stop_type, "Entity")) {
            target_cfg_id = std.math.cast(i32, quest_types.jsonInt(stop_target.object.get("EntityId")) orelse 0) orelse return;
        }
        if (target_cfg_id <= 0) return;

        const vehicle_key = try std.fmt.allocPrint(ctx.alloc.arena, "vehicle:spline:{d}", .{target_cfg_id});
        const quest_spline_key = try std.fmt.allocPrint(ctx.alloc.arena, "quest_spline:{d}", .{target_cfg_id});
        const item_spline_key = try std.fmt.allocPrint(ctx.alloc.arena, "scene_item_spline:{d}", .{target_cfg_id});
        const active_key = try std.fmt.allocPrint(ctx.alloc.arena, "vehicle:spline_active:{d}", .{target_cfg_id});
        const quest_active_key = try std.fmt.allocPrint(ctx.alloc.arena, "quest_spline_active:{d}", .{target_cfg_id});
        const item_active_key = try std.fmt.allocPrint(ctx.alloc.arena, "scene_item_spline_active:{d}", .{target_cfg_id});
        var spline_id: i32 = 0;
        if (ctx.progress_comp) |progress| {
            const vsp = progress.info.getKeyInt(vehicle_key);
            const qsp = progress.info.getKeyInt(quest_spline_key);
            const isp = progress.info.getKeyInt(item_spline_key);
            spline_id = std.math.cast(i32, if (vsp != 0) vsp else if (qsp != 0) qsp else isp) orelse 0;
        }
        try recordProgressInt(ctx, active_key, 0, false);
        try recordProgressInt(ctx, quest_active_key, 0, false);
        try recordProgressInt(ctx, item_active_key, 0, false);
        const stop_at_end = if (params.get("StopType")) |st|
            (st == .object and std.mem.eql(u8, quest_types.jsonStr(st.object.get("Type")) orelse "", "StopAtEndPoint"))
        else
            false;
        if (ctx.scene) |scene| {
            if (scene.findEntityNetIdByConfigId(target_cfg_id)) |net_id| {
                if (scene.net_id_map.get(net_id)) |index| {
                    if (spline_id == 0) {
                        if (scene.entities.items(.world_item)[index].move_spline) |ms| {
                            spline_id = ms.spline_entity_id;
                        }
                    }
                    scene.entities.items(.world_item)[index].move_spline = null;
                }
                if (stop_at_end and spline_id > 0) {
                    if (resolveQuestSplineEndPos(ctx.assets, scene.instance_id, spline_id)) |end_pos| {
                        try persistQuestEntityPosition(ctx, target_cfg_id, net_id, end_pos);
                        try pushEntityMoved(ctx, net_id, end_pos, null);
                    }
                }
                pushSplineStatus(ctx, net_id, .Stop, spline_id, null, null);
            }
        }
    }
}

fn recordVehicleNpcLink(ctx: QuestContext, npc_config_id: i64, vehicle_config_id: i64, seat: i32) !void {
    const progress = ctx.progress_comp orelse return;
    const npc_key = try std.fmt.allocPrint(ctx.alloc.arena, "vehicle:npc:{d}", .{npc_config_id});
    const seat_key = try std.fmt.allocPrint(ctx.alloc.arena, "vehicle:npc_seat:{d}", .{npc_config_id});
    try progress.info.setKeyInt(ctx.alloc.gpa, npc_key, vehicle_config_id);
    try progress.info.setKeyInt(ctx.alloc.gpa, seat_key, seat);
    try progress.save(ctx.fs, ctx.alloc.arena);
}

fn vehicleHasOtherOccupants(ctx: QuestContext, vehicle_config_id: i32, excluding_npc_id: ?i32) bool {
    const progress = ctx.progress_comp orelse return false;
    if (std.math.cast(i32, progress.info.getKeyInt("vehicle:cur"))) |current_vehicle_id| {
        if (current_vehicle_id == vehicle_config_id) return true;
    }
    for (progress.info.dynamic.ints) |entry| {
        if (!std.mem.startsWith(u8, entry.key, "vehicle:npc:")) continue;
        const npc_id = std.fmt.parseInt(i32, entry.key["vehicle:npc:".len..], 10) catch continue;
        if (excluding_npc_id) |exclude| if (npc_id == exclude) continue;
        if (std.math.cast(i32, entry.val)) |linked_vehicle_id| {
            if (linked_vehicle_id == vehicle_config_id) return true;
        }
    }
    return false;
}

test "QuestInfo DataLayer, SubLevel, AreaState, and Var state transitions remain consistent" {
    const gpa = std.testing.allocator;
    var info: QuestInfo = .{};
    defer info.deinit(gpa);

    try info.setVarInt(gpa, "调查", 2);
    try std.testing.expectEqual(@as(i32, 2), info.getVarInt("调查"));

    _ = try QuestInfo.appendUniqueInt(&info.world.loaded_datalayers, gpa, 3);
    _ = try QuestInfo.appendUniqueInt(&info.world.unloaded_datalayers, gpa, 2);
    _ = try QuestInfo.removeIntValue(&info.world.loaded_datalayers, gpa, 2);
    try std.testing.expect(QuestInfo.containsInt(info.world.loaded_datalayers, 3));
    try std.testing.expect(!QuestInfo.containsInt(info.world.loaded_datalayers, 2));

    _ = try QuestInfo.appendUniqueStr(&info.world.loaded_sublevels, gpa, "/Game/A");
    try std.testing.expect(QuestInfo.containsStr(info.world.loaded_sublevels, "/Game/A"));
    try std.testing.expect(try QuestInfo.removeStrValue(&info.world.loaded_sublevels, gpa, "/Game/A"));
    try std.testing.expect(!QuestInfo.containsStr(info.world.loaded_sublevels, "/Game/A"));
}

test "executeAction handles RunActions, RandomVar, SheriffUnlockClue, SetWuYinQuState, and Vehicle actions" {
    const common = @import("common");
    const FileSystem = common.FileSystem;
    const EventQueue = @import("../EventQueue.zig");
    const PlayerBasicComponent = @import("../component/player/PlayerBasicComponent.zig");
    const PlayerInventoryComponent = @import("../component/player/PlayerInventoryComponent.zig");
    const PlayerWeaponComponent = @import("../component/player/PlayerWeaponComponent.zig");
    const PlayerEchoComponent = @import("../component/player/PlayerEchoComponent.zig");
    const PlayerMapComponent = @import("../component/player/PlayerMapComponent.zig");
    const PlayerQuestComponent = @import("../component/player/PlayerQuestComponent.zig");
    const PlayerProgressComponent = @import("../component/player/PlayerProgressComponent.zig");

    const gpa = std.testing.allocator;
    const io = std.testing.io;
    var arena = std.heap.ArenaAllocator.init(gpa);
    defer arena.deinit();
    const aalloc = arena.allocator();

    var tmp = std.testing.tmpDir(.{});
    defer tmp.cleanup();
    const sub_dir = try tmp.dir.openDir(io, ".", .{ .iterate = true });
    var fs: FileSystem = .{ .root_dir = sub_dir, .gpa = gpa, .io = io };
    defer fs.deinit();

    var dummy_buf: [16]@import("../../network.zig").OwnedFrame = undefined;
    var outbound = std.Io.Queue(@import("../../network.zig").OwnedFrame).init(&dummy_buf);
    defer {
        outbound.close(io);
        while (outbound.getOne(io)) |f| {
            gpa.free(f.bytes);
        } else |_| {}
    }
    var ev: std.Io.Event = .unset;
    var conn: Connection = .{ .outbound = &outbound, .transport_work = &ev };

    var basic: PlayerBasicComponent = .{ .player_id = 1, .info = .default };
    var inv: PlayerInventoryComponent = .{ .player_id = 1, .info = .default };
    var wep: PlayerWeaponComponent = .{ .player_id = 1, .weapon_map = .empty };
    var echo: PlayerEchoComponent = .{ .player_id = 1, .echo_map = .empty, .preset_info = .default, .calabash_info = .default };
    var map: PlayerMapComponent = .{ .player_id = 1, .info = .default };
    var quest: PlayerQuestComponent = .{ .player_id = 1, .info = .default };
    defer quest.info.deinit(gpa);
    var prog: PlayerProgressComponent = .{ .player_id = 1, .info = .default };
    defer prog.info.deinit(gpa);
    var events: EventQueue = .{ .arena = aalloc };
    var dummy_assets: Assets = undefined;

    const ctx: QuestContext = .{
        .assets = &dummy_assets,
        .fs = &fs,
        .conn = &conn,
        .events = &events,
        .alloc = .{ .gpa = gpa, .arena = aalloc },
        .io = io,
        .basic_comp = &basic,
        .inventory_comp = &inv,
        .weapon_comp = &wep,
        .echo_comp = &echo,
        .map_comp = &map,
        .quest_comp = &quest,
        .progress_comp = &prog,
    };

    const trace_current_quest = struct {
        fn run(action_ctx: QuestContext, quest_id: i32) !void {
            const progress = action_ctx.progress_comp orelse return error.MissingProgress;
            try progress.info.setKeyInt(action_ctx.alloc.gpa, "test:trace_current_quest", quest_id);
        }
    }.run;
    const callbacks: ActionCallbacks = .{ .on_trace_current_quest = trace_current_quest };

    const run_json =
        \\{"Name":"RunActions","Params":{"ActionList":[
        \\  {"Name":"RandomVar","Params":{"Result":{"Name":"roll_res"},"RightVar":{"Value":5}}},
        \\  {"Name":"SheriffUnlockClue","Params":{"ClueId":42}},
        \\  {"Name":"SetWuYinQuState","Params":{"WuYinQuName":"zone_alpha","State":2}},
        \\  {"Name":"TraceCurrentQuest","Params":{}},
        \\  {"Name":"SetPlayerOperationRestriction","Params":{"Type":"DisableAll"}},
        \\  {"Name":"VehicleEnter","Params":{"VehicleEntityId":8001}}
        \\]}}
    ;
    const act = try std.json.parseFromSliceLeaky(std.json.Value, aalloc, run_json, .{});
    try executeAction(ctx, callbacks, 100, 1, act);

    const roll = quest.info.getVarInt("roll_res");
    try std.testing.expect(roll >= 0 and roll < 5);
    try std.testing.expect(prog.info.hasClaim("sheriff_clue:42"));
    try std.testing.expectEqual(@as(i64, 2), prog.info.getKeyInt("wuyinqu:zone_alpha"));
    try std.testing.expectEqual(@as(i64, 100), prog.info.getKeyInt("test:trace_current_quest"));
    try std.testing.expectEqual(@as(i64, 8001), prog.info.getKeyInt("vehicle:cur"));

    const exit_act = try std.json.parseFromSliceLeaky(std.json.Value, aalloc, "{\"Name\":\"VehicleExitPlayer\",\"Params\":{}}", .{});
    try executeAction(ctx, .{}, 100, 2, exit_act);
    try std.testing.expectEqual(@as(i64, 0), prog.info.getKeyInt("vehicle:cur"));
}

test "executeAction handles 3.8.11 RoleTower, RoverRogue, Sheriff, and PhantomFormation actions" {
    const common = @import("common");
    const FileSystem = common.FileSystem;
    const EventQueue = @import("../EventQueue.zig");
    const PlayerBasicComponent = @import("../component/player/PlayerBasicComponent.zig");
    const PlayerInventoryComponent = @import("../component/player/PlayerInventoryComponent.zig");
    const PlayerWeaponComponent = @import("../component/player/PlayerWeaponComponent.zig");
    const PlayerEchoComponent = @import("../component/player/PlayerEchoComponent.zig");
    const PlayerMapComponent = @import("../component/player/PlayerMapComponent.zig");
    const PlayerQuestComponent = @import("../component/player/PlayerQuestComponent.zig");
    const PlayerProgressComponent = @import("../component/player/PlayerProgressComponent.zig");

    const gpa = std.testing.allocator;
    const io = std.testing.io;
    var arena = std.heap.ArenaAllocator.init(gpa);
    defer arena.deinit();
    const aalloc = arena.allocator();

    var tmp = std.testing.tmpDir(.{});
    defer tmp.cleanup();
    const sub_dir = try tmp.dir.openDir(io, ".", .{ .iterate = true });
    var fs: FileSystem = .{ .root_dir = sub_dir, .gpa = gpa, .io = io };
    defer fs.deinit();

    var dummy_buf: [4]@import("../../network.zig").OwnedFrame = undefined;
    var outbound = std.Io.Queue(@import("../../network.zig").OwnedFrame).init(&dummy_buf);
    var ev: std.Io.Event = .unset;
    var conn: Connection = .{ .outbound = &outbound, .transport_work = &ev };

    var basic: PlayerBasicComponent = .{ .player_id = 1, .info = .default };
    var inv: PlayerInventoryComponent = .{ .player_id = 1, .info = .default };
    var wep: PlayerWeaponComponent = .{ .player_id = 1, .weapon_map = .empty };
    var echo: PlayerEchoComponent = .{ .player_id = 1, .echo_map = .empty, .preset_info = .default, .calabash_info = .default };
    var map: PlayerMapComponent = .{ .player_id = 1, .info = .default };
    var quest: PlayerQuestComponent = .{ .player_id = 1, .info = .default };
    defer quest.info.deinit(gpa);
    var prog: PlayerProgressComponent = .{ .player_id = 1, .info = .default };
    defer prog.info.deinit(gpa);
    var events: EventQueue = .{ .arena = aalloc };
    var dummy_assets: Assets = undefined;

    const ctx: QuestContext = .{
        .assets = &dummy_assets,
        .fs = &fs,
        .conn = &conn,
        .events = &events,
        .alloc = .{ .gpa = gpa, .arena = aalloc },
        .io = io,
        .basic_comp = &basic,
        .inventory_comp = &inv,
        .weapon_comp = &wep,
        .echo_comp = &echo,
        .map_comp = &map,
        .quest_comp = &quest,
        .progress_comp = &prog,
    };

    const actions_json =
        \\{"Name":"RunActions","Params":{"ActionList":[
        \\  {"Name":"RoleTowerGotoNextFloor","Params":{}},
        \\  {"Name":"RoverRogueRoomFinish","Params":{}},
        \\  {"Name":"SheriffUnlockProgress","Params":{"ProgressId":9}},
        \\  {"Name":"ChangePhantomFormation","Params":{"FormationId":3}}
        \\]}}
    ;
    const act = try std.json.parseFromSliceLeaky(std.json.Value, aalloc, actions_json, .{});
    try executeAction(ctx, .{}, 101, 1, act);
    outbound.close(io);
    while (outbound.getOne(io)) |frame| gpa.free(frame.bytes) else |_| {}

    try std.testing.expectEqual(@as(i64, 1), prog.info.getKeyInt("role_tower:stage"));
    try std.testing.expectEqual(@as(i64, 1), prog.info.getKeyInt("rover_rogue:layer"));
    try std.testing.expect(prog.info.hasClaim("sheriff_prog:9"));
    try std.testing.expectEqual(@as(i64, 3), prog.info.getKeyInt("phantom:formation_id"));

    const restore_act = try std.json.parseFromSliceLeaky(std.json.Value, aalloc, "{\"Name\":\"RestorePhantomFormation\",\"Params\":{}}", .{});
    try executeAction(ctx, .{}, 101, 2, restore_act);
    try std.testing.expectEqual(@as(i64, 0), prog.info.getKeyInt("phantom:formation_id"));
}

test "executeAction SetPlayerPos attaches FlowActionCtx when executed inside an active flow" {
    const common = @import("common");
    const proto = @import("proto");
    const net = @import("../../network.zig");
    const Message = @import("../../network/Message.zig");
    const FileSystem = common.FileSystem;
    const EventQueue = @import("../EventQueue.zig");
    const PlayerBasicComponent = @import("../component/player/PlayerBasicComponent.zig");
    const PlayerInventoryComponent = @import("../component/player/PlayerInventoryComponent.zig");
    const PlayerWeaponComponent = @import("../component/player/PlayerWeaponComponent.zig");
    const PlayerEchoComponent = @import("../component/player/PlayerEchoComponent.zig");
    const PlayerMapComponent = @import("../component/player/PlayerMapComponent.zig");
    const PlayerQuestComponent = @import("../component/player/PlayerQuestComponent.zig");
    const PlayerProgressComponent = @import("../component/player/PlayerProgressComponent.zig");

    const gpa = std.testing.allocator;
    const io = std.testing.io;
    var arena = std.heap.ArenaAllocator.init(gpa);
    defer arena.deinit();
    const aalloc = arena.allocator();

    var tmp = std.testing.tmpDir(.{});
    defer tmp.cleanup();
    const sub_dir = try tmp.dir.openDir(io, ".", .{ .iterate = true });
    var fs: FileSystem = .{ .root_dir = sub_dir, .gpa = gpa, .io = io };
    defer fs.deinit();

    var frame_buf: [32]net.OwnedFrame = undefined;
    var outbound = std.Io.Queue(net.OwnedFrame).init(&frame_buf);
    defer {
        outbound.close(io);
        while (outbound.getOne(io)) |f| {
            gpa.free(f.bytes);
        } else |_| {}
    }
    var ev: std.Io.Event = .unset;
    var conn: Connection = .{ .outbound = &outbound, .transport_work = &ev };

    var basic: PlayerBasicComponent = .{ .player_id = 1, .info = .default };
    var inv: PlayerInventoryComponent = .{ .player_id = 1, .info = .default };
    var wep: PlayerWeaponComponent = .{ .player_id = 1, .weapon_map = .empty };
    var echo: PlayerEchoComponent = .{ .player_id = 1, .echo_map = .empty, .preset_info = .default, .calabash_info = .default };
    var map: PlayerMapComponent = .{ .player_id = 1, .info = .default };
    var quest: PlayerQuestComponent = .{ .player_id = 1, .info = .default };
    defer quest.info.deinit(gpa);
    var prog: PlayerProgressComponent = .{ .player_id = 1, .info = .default };
    defer prog.info.deinit(gpa);
    var events: EventQueue = .{ .arena = aalloc };
    var dummy_assets: Assets = undefined;

    var scene: Scene = .{
        .player_id = 1,
        .instance_id = 8,
        .instance = .{ .owner_id = 1 },
        .formation_info = undefined,
        .explore_tools_info = undefined,
    };
    scene.active_flow_list_name = try gpa.dupe(u8, "剧情_新主线_CE版_序章");
    scene.active_flow_id = 1;
    scene.active_flow_state_id = 12;
    defer {
        gpa.free(scene.active_flow_list_name);
        scene.entities.deinit(gpa);
        scene.net_id_map.deinit(gpa);
    }

    const ctx: QuestContext = .{
        .assets = &dummy_assets,
        .fs = &fs,
        .conn = &conn,
        .scene = &scene,
        .events = &events,
        .alloc = .{ .gpa = gpa, .arena = aalloc },
        .io = io,
        .basic_comp = &basic,
        .inventory_comp = &inv,
        .weapon_comp = &wep,
        .echo_comp = &echo,
        .map_comp = &map,
        .quest_comp = &quest,
        .progress_comp = &prog,
    };

    const act_json =
        \\{"Name":"SetPlayerPos","ActionId":5000073,"Params":{"TelePortConfig":{"Type":0,"TargetPos":{"X":-114682,"Y":71774,"Z":13457,"A":30}}}}
    ;
    const act = try std.json.parseFromSliceLeaky(std.json.Value, aalloc, act_json, .{});
    // Non-flow SetPlayerPos while a flow happens to be open must not steal FlowActionCtx.
    try executeAction(ctx, .{ .is_flow_action = false }, 139000025, 21, act);
    const non_flow_frame = try outbound.getOne(io);
    defer gpa.free(non_flow_frame.bytes);
    var r_nf = std.Io.Reader.fixed(non_flow_frame.bytes[3..]);
    const msg_nf = try Message.decode(&r_nf);
    var pr_nf = std.Io.Reader.fixed(msg_nf.payload);
    const notify_nf = try proto.decodeMessage(&pr_nf, aalloc, pb.TeleportNotify, proto.pb_desc);
    try std.testing.expectEqual(pb.TeleportReason.Action, notify_nf.Reason.?);
    try std.testing.expect(notify_nf.GameCtx == null);

    try executeAction(ctx, .{ .is_flow_action = true }, 139000025, 21, act);
    outbound.close(io);

    const frame = try outbound.getOne(io);
    defer gpa.free(frame.bytes);
    var r = std.Io.Reader.fixed(frame.bytes[3..]);
    const msg = try Message.decode(&r);
    var pr = std.Io.Reader.fixed(msg.payload);
    const notify = try proto.decodeMessage(&pr, aalloc, pb.TeleportNotify, proto.pb_desc);

    try std.testing.expectEqual(pb.TeleportReason.FlowAction, notify.Reason.?);
    const gctx = notify.GameCtx.?;
    try std.testing.expectEqual(pb.GameCtxType.FlowActionCtx, gctx.CtxType.?);
    const flow_ctx = gctx.CtxInfo.?.FlowAction.?;
    try std.testing.expectEqualStrings("剧情_新主线_CE版_序章", flow_ctx.FlowListName);
    try std.testing.expectEqual(@as(i32, 1), flow_ctx.FlowId);
    try std.testing.expectEqual(@as(i32, 12), flow_ctx.StateId);
    try std.testing.expectEqual(@as(i32, 5000073), flow_ctx.ActionId);
}
